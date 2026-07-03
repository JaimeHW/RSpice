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
    pub p328: f64, pub p329: f64, pub p330: f64, pub p331: f64, pub p332: f64, pub p333: f64, pub p334: f64, pub p335: f64, 
    pub p336: f64, pub p337: f64, pub p338: f64, pub p339: f64, pub p340: f64, pub p341: f64, pub p342: f64, pub p343: f64, 
    pub p344: f64, pub p345: f64, pub p346: f64, pub p347: f64, pub p348: f64, pub p349: f64, pub p350: f64, pub p351: f64, 
    pub p352: f64, pub p353: f64, pub p354: f64, pub p355: f64, pub p356: f64, pub p357: f64, pub p358: f64, pub p359: f64, 
    pub p360: f64, pub p361: f64, pub p362: f64, pub p363: f64, pub p364: f64, pub p365: f64, pub p366: f64, pub p367: f64, 
    pub p368: f64, pub p369: f64, pub p370: f64, pub p371: f64, pub p372: f64, pub p373: f64, pub p374: f64, pub p375: f64, 
    pub p376: f64, pub p377: f64, pub p378: f64, pub p379: f64, pub p380: f64, pub p381: f64, pub p382: f64, pub p383: f64, 
    pub p384: f64, pub p385: f64, pub p386: f64, pub p387: f64, pub p388: f64, pub p389: f64, pub p390: f64, pub p391: f64, 
    pub p392: f64, pub p393: f64, pub p394: f64, pub p395: f64, pub p396: f64, pub p397: f64, pub p398: f64, pub p399: f64, 
    pub p400: f64, pub p401: f64, pub p402: f64, pub p403: f64, pub p404: f64, pub p405: f64, pub p406: f64, pub p407: f64, 
    pub p408: f64, pub p409: f64, pub p410: f64, pub p411: f64, pub p412: f64, pub p413: f64, pub p414: f64, pub p415: f64, 
    pub p416: f64, pub p417: f64, pub p418: f64, pub p419: f64, pub p420: f64, pub p421: f64, pub p422: f64, pub p423: f64, 
    pub p424: f64, pub p425: f64, pub p426: f64, pub p427: f64, pub p428: f64, pub p429: f64, pub p430: f64, pub p431: f64, 
    pub p432: f64, pub p433: f64, pub p434: f64, pub p435: f64, pub p436: f64, pub p437: f64, pub p438: f64, pub p439: f64, 
    pub p440: f64, pub p441: f64, pub p442: f64, pub p443: f64, pub p444: f64, pub p445: f64, pub p446: f64, pub p447: f64, 
    pub p448: f64, pub p449: f64, pub p450: f64, pub p451: f64, pub p452: f64, pub p453: f64, pub p454: f64, pub p455: f64, 
    pub p456: f64, pub p457: f64, pub p458: f64, pub p459: f64, pub p460: f64, pub p461: f64, pub p462: f64, pub p463: f64, 
    pub p464: f64, pub p465: f64, pub p466: f64, pub p467: f64, pub p468: f64, pub p469: f64, pub p470: f64, pub p471: f64, 
    pub p472: f64, pub p473: f64, pub p474: f64, pub p475: f64, pub p476: f64, pub p477: f64, pub p478: f64, pub p479: f64, 
    pub p480: f64, pub p481: f64, pub p482: f64, pub p483: f64, pub p484: f64, pub p485: f64, pub p486: f64, pub p487: f64, 
    pub p488: f64, pub p489: f64, pub p490: f64, pub p491: f64, pub p492: f64, 
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 33] = [
                0.0, 102.8, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 21.0,
                150.0, 1.0, 0.0, 0.001, 1e-6, 1e-6, 1e-12, 1e-12,
                1e-6, 1e-6, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 33);
            {
                let params = &mut *ptr;
                params.p33 = params.p31;
                validate_parameter("MULT_FN", params.p33, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 459] = [
                0.0, 1.0, 0.0, 1.0, 1e-7, 0.0, 0.0, 2e-9,
                1e-8, 0.0, 1e-7, 0.0, 3e18, 0.0, 2e-9, 1e20,
                1e20, 0.0, 0.0, 0.0, 1e21, 1.0, 1.0, 0.0,
                1.0, 1e22, 0.0, 0.0, 0.0, 1.0, 0.0, 0.2,
                0.0, 0.0, 0.05, 1.0, 1.5, 0.0, 0.0, 0.0,
                0.0, 1.5, 0.0, 2.0, 1.0, 0.0, 0.0, 1.5,
                0.0, 0.0, 1.0, 0.0, 1.0, 30.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, -0.1, 0.0, 0.0, 8.0,
                0.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.375, 0.063, 0.375, 0.063,
                0.375, 0.063, 0.0, 1.0, 3.1, 0.0, 0.0, 0.0,
                0.2, 0.0, 0.0, 0.0, 41.0, 41.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.2, 0.05,
                1.5, 1.0, 10.0, 0.0, 1.0, 1e-12, 0.0, 1e22,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 8.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.04e-18, 0.0, 10000.0, 0.0, 1e-11, 1.0, 0.0, 8e22,
                30000000.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 2e-9, 1e-8, 0.0, 1e-7, 0.0,
                3e18, 0.0, 2e-9, 1e20, 1e20, 0.0, 0.0, 2.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1e21, 0.0, 1.0, 1.0, 0.0, 2.0,
                0.0, 1.0, 1e22, 0.0, 0.0, 0.0, 2.0, 0.0,
                1.0, 0.0, 0.2, 0.0, 0.0, 0.0, 0.05, 0.0,
                0.0, 1e-8, 0.0, 0.0, 1e-8, 0.0, 0.0, 1e-8,
                1.0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.5, 0.0, 2.0, 1.0, 0.0, 0.0, 1.5, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                30.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, -0.1, 0.0, 0.0, 0.0,
                0.0, 0.0, 8.0, 0.0, 1.0, 0.0, 1.5, 0.0,
                1.0, 0.0, 2.0, 0.0, 0.0, 0.5, 0.0, 1.5,
                0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.375, 0.063, 0.375, 0.063, 0.375,
                0.063, 0.0, 1.0, 3.1, 0.0, 0.0, 0.0, 0.2,
                0.0, 0.0, 0.0, 0.0, 0.0, 41.0, 41.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-8,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 2.0,
                0.0, 1.0, 0.0, 2.0, 0.0, 1.0, 0.2, 0.0,
                1e-8, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1e22, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0,
                2.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 8.0,
                0.0, 1.0, 0.0, 1.5, 0.0, 1.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 100000.0, 1.5, 3.0,
                4.5, 0.0, 1e-12, 1e-7, 0.0, 1.0, 0.0, 2.0,
                8e22, 0.0, 30000000.0, 0.0, 0.0, 0.0, 1.0, 1.0,
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 1e-7, 3.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(34), 459);
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 494] = [
    ("swscale", 0), ("version", 1), ("swsubdep", 2), ("swigate", 3), ("swgidl", 4), ("swshe", 5), ("swign", 6), ("swjunasym", 7), ("swimpact", 8), ("swpdep", 9), ("swcryo", 10), ("swqmod", 11), ("swedge", 12), ("qmc", 13), ("type", 14), ("tr", 15), 
    ("tref", 15), ("tmax", 16), ("tmin", 17), ("atmin", 18), ("btmin", 19), ("l", 20), ("w", 21), ("asource", 22), ("adrain", 23), ("psource", 24), ("pdrain", 25), ("sa", 26), ("sb", 27), ("sd", 28), ("nf", 29), ("mult", 30), 
    ("mult_i", 31), ("mult_q", 32), ("mult_fn", 33), ("delvto", 34), ("factuo", 35), ("dtemp", 36), ("ngcon", 37), ("xgw", 38), ("nrs", 39), ("nrd", 40), ("toxe", 41), ("tsi", 42), ("xge", 43), ("tbox", 44), ("nch", 45), ("nsub", 46), 
    ("ct", 47), ("toxp", 48), ("nov", 49), ("novd", 50), ("vfb", 51), ("vfbb", 52), ("stvfb", 53), ("np", 54), ("cicf", 55), ("cic", 56), ("psce", 57), ("psceb", 58), ("nsddc", 59), ("pscedlb", 60), ("pnce", 61), ("cf", 62), 
    ("cfb", 63), ("stcf", 64), ("cfd", 65), ("cfdl", 66), ("cfdlb", 67), ("betn", 68), ("betnb", 69), ("stbet", 70), ("cs", 71), ("csfi", 72), ("csbi", 73), ("stcs", 74), ("thecs", 75), ("stthecs", 76), ("csthr", 77), ("csthrb", 78), 
    ("mue", 79), ("stmue", 80), ("themu", 81), ("stthemu", 82), ("xcor", 83), ("xcorb", 84), ("stxcor", 85), ("feta", 86), ("rs", 87), ("rsig", 88), ("strs", 89), ("rsg", 90), ("thersg", 91), ("rsb", 92), ("thesat", 93), ("stthesat", 94), 
    ("thesatg", 95), ("thesatb", 96), ("ax", 97), ("alp", 98), ("alp1", 99), ("alpb", 100), ("vp", 101), ("vpg", 102), ("gco", 103), ("iginv", 104), ("igovinv", 105), ("igovinvd", 106), ("igovacc", 107), ("igovaccd", 108), ("stig", 109), ("gc2ch", 110), 
    ("gc3ch", 111), ("gc2ovinv", 112), ("gc3ovinv", 113), ("gc2ovacc", 114), ("gc3ovacc", 115), ("gcdov", 116), ("gcvdov", 117), ("chib", 118), ("niginv", 119), ("fnovinv", 120), ("fnovinvd", 121), ("gcovinvfn", 122), ("stigfn", 123), ("agidl", 124), ("agidld", 125), ("bgidl", 126), 
    ("bgidld", 127), ("stbgidl", 128), ("stbgidld", 129), ("cgidl", 130), ("cgidld", 131), ("dgidl", 132), ("dgidld", 133), ("ctedge", 134), ("vfbedge", 135), ("vfbbedge", 136), ("stvfbedge", 137), ("cicfedge", 138), ("cicedge", 139), ("psceedge", 140), ("pscebedge", 141), ("cfedge", 142), 
    ("cfbedge", 143), ("cfdedge", 144), ("betnedge", 145), ("stbetedge", 146), ("a1", 147), ("a2", 148), ("sta2", 149), ("a3", 150), ("areaq", 151), ("cgbov", 152), ("nsdac", 153), ("fif", 154), ("fsceac", 155), ("vfbac", 156), ("vfbbac", 157), ("psceac", 158), 
    ("cfac", 159), ("thesatac", 160), ("axac", 161), ("alpac", 162), ("cov", 163), ("covd", 164), ("covdl", 165), ("covdlb", 166), ("dvfbov", 167), ("cfr", 168), ("cfrd", 169), ("csd", 170), ("csdbp", 171), ("rth", 172), ("strth", 173), ("cth", 174), 
    ("fnt", 175), ("fntexc", 176), ("nfa", 177), ("nfb", 178), ("nfc", 179), ("nfe", 180), ("nfeb", 181), ("ef", 182), ("rg", 183), ("rse", 184), ("rde", 185), ("rwell", 186), ("lvaro", 187), ("lvarl", 188), ("lvarw", 189), ("lap", 190), 
    ("wvaro", 191), ("wvarl", 192), ("wvarw", 193), ("wot", 194), ("dlq", 195), ("dwq", 196), ("toxeo", 197), ("tsio", 198), ("xgeo", 199), ("tboxo", 200), ("ncho", 201), ("nsubo", 202), ("cto", 203), ("toxpo", 204), ("novo", 205), ("novdo", 206), 
    ("vfbo", 207), ("vfbl", 208), ("vfblexp", 209), ("vfbl2", 210), ("vfblexp2", 211), ("vfbw", 212), ("vfblw", 213), ("vfbbo", 214), ("vfblbo", 215), ("stvfbo", 216), ("stvfbl", 217), ("stvfbw", 218), ("stvfblw", 219), ("npo", 220), ("npl", 221), ("cicfo", 222), 
    ("cico", 223), ("pscel", 224), ("pscelexp", 225), ("pscew", 226), ("pscebo", 227), ("nsddco", 228), ("pscedlbo", 229), ("pncew", 230), ("cfl", 231), ("cflexp", 232), ("cfw", 233), ("cfbo", 234), ("stcfl", 235), ("cfdo", 236), ("cfdll", 237), ("cfdlw", 238), 
    ("cfdlbo", 239), ("uo", 240), ("fbet1", 241), ("fbet1w", 242), ("lp1", 243), ("lp1w", 244), ("fbet2", 245), ("lp2", 246), ("betw1", 247), ("betw2", 248), ("wbet", 249), ("betnbo", 250), ("stbeto", 251), ("stbetl", 252), ("stbetw", 253), ("stbetlw", 254), 
    ("cso", 255), ("csl", 256), ("cslexp", 257), ("csw", 258), ("cslw", 259), ("csfio", 260), ("csbio", 261), ("stcso", 262), ("stcsl", 263), ("stcsw", 264), ("stcslw", 265), ("thecso", 266), ("stthecso", 267), ("csthro", 268), ("csthrbo", 269), ("mueo", 270), 
    ("stmueo", 271), ("themuo", 272), ("stthemuo", 273), ("xcoro", 274), ("xcorl", 275), ("xcorlexp", 276), ("xcorw", 277), ("xcorlw", 278), ("xcorbo", 279), ("stxcoro", 280), ("fetao", 281), ("rsw1", 282), ("rsw2", 283), ("rsigo", 284), ("strso", 285), ("rsgo", 286), 
    ("thersgo", 287), ("rsbo", 288), ("thesato", 289), ("thesatl", 290), ("thesatlexp", 291), ("thesatw", 292), ("thesatlw", 293), ("stthesato", 294), ("stthesatl", 295), ("stthesatw", 296), ("stthesatlw", 297), ("thesatgo", 298), ("thesatbo", 299), ("axo", 300), ("axl", 301), ("axlexp", 302), 
    ("axl2", 303), ("axlexp2", 304), ("alpl1", 305), ("alplexp", 306), ("alpl2", 307), ("alplexp2", 308), ("alpw", 309), ("alp1l1", 310), ("alp1lexp", 311), ("alp1l2", 312), ("alp1lexp2", 313), ("alp1w", 314), ("alpbo", 315), ("vpo", 316), ("vpgo", 317), ("gcoo", 318), 
    ("iginvlw", 319), ("igovinvw", 320), ("igovinvdw", 321), ("igovaccw", 322), ("igovaccdw", 323), ("stigo", 324), ("gc2cho", 325), ("gc3cho", 326), ("gc2ovinvo", 327), ("gc3ovinvo", 328), ("gc2ovacco", 329), ("gc3ovacco", 330), ("gcdovl", 331), ("gcvdovo", 332), ("chibo", 333), ("niginvo", 334), 
    ("fnovinvw", 335), ("fnovinvdw", 336), ("gcovinvfno", 337), ("stigfno", 338), ("agidlo", 339), ("agidldo", 340), ("agidlw", 341), ("agidldw", 342), ("bgidlo", 343), ("bgidldo", 344), ("stbgidlo", 345), ("stbgidldo", 346), ("cgidlo", 347), ("cgidldo", 348), ("dgidlo", 349), ("dgidldo", 350), 
    ("dgidll", 351), ("dgidldl", 352), ("wedge", 353), ("wedgew", 354), ("ctedgeo", 355), ("vfbedgeo", 356), ("vfbedgel", 357), ("vfbedgelexp", 358), ("vfbedgew", 359), ("vfbedgelw", 360), ("vfbbedgeo", 361), ("stvfbedgeo", 362), ("stvfbedgel", 363), ("stvfbedgew", 364), ("stvfbedgelw", 365), ("cicfedgeo", 366), 
    ("cicedgeo", 367), ("psceedgel", 368), ("psceedgelexp", 369), ("psceedgew", 370), ("pscebedgeo", 371), ("cfedgel", 372), ("cfedgelexp", 373), ("cfedgew", 374), ("cfbedgeo", 375), ("cfdedgeo", 376), ("fbetedge", 377), ("lpedge", 378), ("betedgew", 379), ("stbetedgeo", 380), ("stbetedgel", 381), ("stbetedgew", 382), 
    ("stbetedgelw", 383), ("a1o", 384), ("a1l", 385), ("a1w", 386), ("a2o", 387), ("sta2o", 388), ("a3o", 389), ("a3l", 390), ("a3w", 391), ("cgbovo", 392), ("cgbovl", 393), ("nsdaco", 394), ("fifw", 395), ("fsceaco", 396), ("vfbaco", 397), ("vfbacl", 398), 
    ("vfbaclexp", 399), ("vfbacl2", 400), ("vfbaclexp2", 401), ("vfbacw", 402), ("vfbaclw", 403), ("vfbbaco", 404), ("vfblbaco", 405), ("psceacl", 406), ("psceaclexp", 407), ("psceacw", 408), ("cfacl", 409), ("cfaclexp", 410), ("cfacw", 411), ("thesataco", 412), ("thesatacl", 413), ("thesataclexp", 414), 
    ("thesatacw", 415), ("thesataclw", 416), ("axaco", 417), ("axacl", 418), ("axaclexp", 419), ("axacl2", 420), ("axaclexp2", 421), ("alpacl1", 422), ("alpaclexp", 423), ("alpacl2", 424), ("alpaclexp2", 425), ("alpacw", 426), ("lovo", 427), ("lovdo", 428), ("covdlo", 429), ("covdlw", 430), 
    ("covdlbo", 431), ("dvfbovo", 432), ("cfro", 433), ("cfrdo", 434), ("cfrw", 435), ("cfrdw", 436), ("csdo", 437), ("csdbpo", 438), ("rtho", 439), ("rthl", 440), ("rthw", 441), ("rthlw", 442), ("strtho", 443), ("ctho", 444), ("lambtho", 445), ("ftho", 446), 
    ("fnto", 447), ("fntexcl", 448), ("fntexclexp", 449), ("nfalw", 450), ("nfaw", 451), ("nfblw", 452), ("nfclw", 453), ("nfeo", 454), ("nfebo", 455), ("efo", 456), ("swstress", 457), ("saref", 458), ("sbref", 459), ("wlod", 460), ("kuo", 461), ("kvsat", 462), 
    ("tkuo", 463), ("lkuo", 464), ("wkuo", 465), ("pkuo", 466), ("llodkuo", 467), ("wlodkuo", 468), ("kvtho", 469), ("lkvtho", 470), ("wkvtho", 471), ("pkvtho", 472), ("llodvth", 473), ("wlodvth", 474), ("stetao", 475), ("lodetao", 476), ("strlambda", 477), ("stralpha", 478), 
    ("strdvfbo", 479), ("strwdvfbo", 480), ("strdcfl", 481), ("strruo", 482), ("strtruo", 483), ("strrvsat", 484), ("rgo", 485), ("rint", 486), ("rvpoly", 487), ("rshg", 488), ("dlsil", 489), ("rsh", 490), ("rshd", 491), ("rwello", 492), 
];

const PARAMETER_DISPLAY_NAMES: [&str; 493] = [
    "SWSCALE", "VERSION", "SWSUBDEP", "SWIGATE", "SWGIDL", "SWSHE", "SWIGN", "SWJUNASYM", "SWIMPACT", "SWPDEP", "SWCRYO", "SWQMOD", "SWEDGE", "QMC", "TYPE", "TR", 
    "TMAX", "TMIN", "ATMIN", "BTMIN", "L", "W", "ASOURCE", "ADRAIN", "PSOURCE", "PDRAIN", "SA", "SB", "SD", "NF", "MULT", "MULT_I", 
    "MULT_Q", "MULT_FN", "DELVTO", "FACTUO", "DTEMP", "NGCON", "XGW", "NRS", "NRD", "TOXE", "TSI", "XGE", "TBOX", "NCH", "NSUB", "CT", 
    "TOXP", "NOV", "NOVD", "VFB", "VFBB", "STVFB", "NP", "CICF", "CIC", "PSCE", "PSCEB", "NSDDC", "PSCEDLB", "PNCE", "CF", "CFB", 
    "STCF", "CFD", "CFDL", "CFDLB", "BETN", "BETNB", "STBET", "CS", "CSFI", "CSBI", "STCS", "THECS", "STTHECS", "CSTHR", "CSTHRB", "MUE", 
    "STMUE", "THEMU", "STTHEMU", "XCOR", "XCORB", "STXCOR", "FETA", "RS", "RSIG", "STRS", "RSG", "THERSG", "RSB", "THESAT", "STTHESAT", "THESATG", 
    "THESATB", "AX", "ALP", "ALP1", "ALPB", "VP", "VPG", "GCO", "IGINV", "IGOVINV", "IGOVINVD", "IGOVACC", "IGOVACCD", "STIG", "GC2CH", "GC3CH", 
    "GC2OVINV", "GC3OVINV", "GC2OVACC", "GC3OVACC", "GCDOV", "GCVDOV", "CHIB", "NIGINV", "FNOVINV", "FNOVINVD", "GCOVINVFN", "STIGFN", "AGIDL", "AGIDLD", "BGIDL", "BGIDLD", 
    "STBGIDL", "STBGIDLD", "CGIDL", "CGIDLD", "DGIDL", "DGIDLD", "CTEDGE", "VFBEDGE", "VFBBEDGE", "STVFBEDGE", "CICFEDGE", "CICEDGE", "PSCEEDGE", "PSCEBEDGE", "CFEDGE", "CFBEDGE", 
    "CFDEDGE", "BETNEDGE", "STBETEDGE", "A1", "A2", "STA2", "A3", "AREAQ", "CGBOV", "NSDAC", "FIF", "FSCEAC", "VFBAC", "VFBBAC", "PSCEAC", "CFAC", 
    "THESATAC", "AXAC", "ALPAC", "COV", "COVD", "COVDL", "COVDLB", "DVFBOV", "CFR", "CFRD", "CSD", "CSDBP", "RTH", "STRTH", "CTH", "FNT", 
    "FNTEXC", "NFA", "NFB", "NFC", "NFE", "NFEB", "EF", "RG", "RSE", "RDE", "RWELL", "LVARO", "LVARL", "LVARW", "LAP", "WVARO", 
    "WVARL", "WVARW", "WOT", "DLQ", "DWQ", "TOXEO", "TSIO", "XGEO", "TBOXO", "NCHO", "NSUBO", "CTO", "TOXPO", "NOVO", "NOVDO", "VFBO", 
    "VFBL", "VFBLEXP", "VFBL2", "VFBLEXP2", "VFBW", "VFBLW", "VFBBO", "VFBLBO", "STVFBO", "STVFBL", "STVFBW", "STVFBLW", "NPO", "NPL", "CICFO", "CICO", 
    "PSCEL", "PSCELEXP", "PSCEW", "PSCEBO", "NSDDCO", "PSCEDLBO", "PNCEW", "CFL", "CFLEXP", "CFW", "CFBO", "STCFL", "CFDO", "CFDLL", "CFDLW", "CFDLBO", 
    "UO", "FBET1", "FBET1W", "LP1", "LP1W", "FBET2", "LP2", "BETW1", "BETW2", "WBET", "BETNBO", "STBETO", "STBETL", "STBETW", "STBETLW", "CSO", 
    "CSL", "CSLEXP", "CSW", "CSLW", "CSFIO", "CSBIO", "STCSO", "STCSL", "STCSW", "STCSLW", "THECSO", "STTHECSO", "CSTHRO", "CSTHRBO", "MUEO", "STMUEO", 
    "THEMUO", "STTHEMUO", "XCORO", "XCORL", "XCORLEXP", "XCORW", "XCORLW", "XCORBO", "STXCORO", "FETAO", "RSW1", "RSW2", "RSIGO", "STRSO", "RSGO", "THERSGO", 
    "RSBO", "THESATO", "THESATL", "THESATLEXP", "THESATW", "THESATLW", "STTHESATO", "STTHESATL", "STTHESATW", "STTHESATLW", "THESATGO", "THESATBO", "AXO", "AXL", "AXLEXP", "AXL2", 
    "AXLEXP2", "ALPL1", "ALPLEXP", "ALPL2", "ALPLEXP2", "ALPW", "ALP1L1", "ALP1LEXP", "ALP1L2", "ALP1LEXP2", "ALP1W", "ALPBO", "VPO", "VPGO", "GCOO", "IGINVLW", 
    "IGOVINVW", "IGOVINVDW", "IGOVACCW", "IGOVACCDW", "STIGO", "GC2CHO", "GC3CHO", "GC2OVINVO", "GC3OVINVO", "GC2OVACCO", "GC3OVACCO", "GCDOVL", "GCVDOVO", "CHIBO", "NIGINVO", "FNOVINVW", 
    "FNOVINVDW", "GCOVINVFNO", "STIGFNO", "AGIDLO", "AGIDLDO", "AGIDLW", "AGIDLDW", "BGIDLO", "BGIDLDO", "STBGIDLO", "STBGIDLDO", "CGIDLO", "CGIDLDO", "DGIDLO", "DGIDLDO", "DGIDLL", 
    "DGIDLDL", "WEDGE", "WEDGEW", "CTEDGEO", "VFBEDGEO", "VFBEDGEL", "VFBEDGELEXP", "VFBEDGEW", "VFBEDGELW", "VFBBEDGEO", "STVFBEDGEO", "STVFBEDGEL", "STVFBEDGEW", "STVFBEDGELW", "CICFEDGEO", "CICEDGEO", 
    "PSCEEDGEL", "PSCEEDGELEXP", "PSCEEDGEW", "PSCEBEDGEO", "CFEDGEL", "CFEDGELEXP", "CFEDGEW", "CFBEDGEO", "CFDEDGEO", "FBETEDGE", "LPEDGE", "BETEDGEW", "STBETEDGEO", "STBETEDGEL", "STBETEDGEW", "STBETEDGELW", 
    "A1O", "A1L", "A1W", "A2O", "STA2O", "A3O", "A3L", "A3W", "CGBOVO", "CGBOVL", "NSDACO", "FIFW", "FSCEACO", "VFBACO", "VFBACL", "VFBACLEXP", 
    "VFBACL2", "VFBACLEXP2", "VFBACW", "VFBACLW", "VFBBACO", "VFBLBACO", "PSCEACL", "PSCEACLEXP", "PSCEACW", "CFACL", "CFACLEXP", "CFACW", "THESATACO", "THESATACL", "THESATACLEXP", "THESATACW", 
    "THESATACLW", "AXACO", "AXACL", "AXACLEXP", "AXACL2", "AXACLEXP2", "ALPACL1", "ALPACLEXP", "ALPACL2", "ALPACLEXP2", "ALPACW", "LOVO", "LOVDO", "COVDLO", "COVDLW", "COVDLBO", 
    "DVFBOVO", "CFRO", "CFRDO", "CFRW", "CFRDW", "CSDO", "CSDBPO", "RTHO", "RTHL", "RTHW", "RTHLW", "STRTHO", "CTHO", "LAMBTHO", "FTHO", "FNTO", 
    "FNTEXCL", "FNTEXCLEXP", "NFALW", "NFAW", "NFBLW", "NFCLW", "NFEO", "NFEBO", "EFO", "SWSTRESS", "SAREF", "SBREF", "WLOD", "KUO", "KVSAT", "TKUO", 
    "LKUO", "WKUO", "PKUO", "LLODKUO", "WLODKUO", "KVTHO", "LKVTHO", "WKVTHO", "PKVTHO", "LLODVTH", "WLODVTH", "STETAO", "LODETAO", "STRLAMBDA", "STRALPHA", "STRDVFBO", 
    "STRWDVFBO", "STRDCFL", "STRRUO", "STRTRUO", "STRRVSAT", "RGO", "RINT", "RVPOLY", "RSHG", "DLSIL", "RSH", "RSHD", "RWELLO", 
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 493] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, 
    None, Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 3e-9, label: "3e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 3e-10, label: "3e-10" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), None, None, None, Some(ParameterBound { value: 1e19, label: "1e19" }), Some(ParameterBound { value: 0.1, label: "0.1" }), 
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, Some(ParameterBound { value: 0.05, label: "0.05" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), 
    Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 3e-9, label: "3e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 3e-10, label: "3e-10" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 3e-10, label: "3e-10" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), Some(ParameterBound { value: 1000000000000000.0, label: "1000000000000000.0" }), None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, 
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None, 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.05, label: "0.05" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, None, Some(ParameterBound { value: 1e18, label: "1e18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), 
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.5, label: "0.5" }), None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 493] = [
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, 
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, 
    None, Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, None, None, 
    Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 1e21, label: "1e21" }), Some(ParameterBound { value: 1e21, label: "1e21" }), None, None, None, Some(ParameterBound { value: 1e22, label: "1e22" }), Some(ParameterBound { value: 10.0, label: "10.0" }), 
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 16.0, label: "16.0" }), None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), 
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), 
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, 
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, 
    None, Some(ParameterBound { value: 16.0, label: "16.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1e-6, label: "1e-6" }), None, None, None, Some(ParameterBound { value: 1e-6, label: "1e-6" }), Some(ParameterBound { value: 1e21, label: "1e21" }), Some(ParameterBound { value: 1e21, label: "1e21" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), 
    None, None, None, None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, 
    None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), 
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, 
    None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 1e22, label: "1e22" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, 
];

const PARAMETER_RANGE_FLAGS: [u8; 493] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 3, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 2, 2, 
    0, 2, 0, 2, 2, 0, 0, 2, 2, 2, 0, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 2, 0, 2, 
    2, 0, 2, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 
    0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 0, 0, 2, 
    2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 
    0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 2, 0, 2, 0, 0, 2, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 2, 0, 
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 0, 2, 
    2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 
    0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 2, 0, 2, 0, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 2, 2, 2, 
    2, 0, 0, 0, 2, 2, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 
    0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 2, 
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 493] = [
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], 
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
    &[], &[], &[], &[], &[], 
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
    pub nodes: [usize; 10],
    pub branches: [usize; 4],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 493]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 19]>,
    pub(crate) ddt_state_previous: Box<[f64; 19]>,
    pub(crate) ddt_state_older: Box<[f64; 19]>,
    pub(crate) ddt_state_initialized: Box<[bool; 19]>,
    pub(crate) ddt_derivative_current: Box<[f64; 19]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 19]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 5;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 5] = ["NSIG", "si", "di", "bp", "gp"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 493;
    pub const VARIABLE_COUNT: usize = 1901;
    pub const DDT_STATE_COUNT: usize = 19;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
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
        }
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'l_utsoi'", name));
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
}
