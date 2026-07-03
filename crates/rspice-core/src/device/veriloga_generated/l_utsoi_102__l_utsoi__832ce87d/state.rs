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
    pub(crate) params: Box<Parameters>,
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
    pub(crate) scalar_static_f64: Box<[f64; 2364]>,
    pub(crate) scalar_static_bool: Box<[bool; 301]>,
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
            scalar_static_f64: boxed_zero_f64_array::<2364>(),
            scalar_static_bool: boxed_zero_bool_array::<301>(),
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
        self.scalar_static_f64[0]=p.p15;
        self.scalar_static_f64[1]=(273.15+self.scalar_static_f64[0]);
        self.scalar_static_f64[2]=p.p36;
        self.scalar_static_f64[3]=p.p10;
        self.scalar_static_bool[0]=(self.scalar_static_f64[3]==1.0);
        self.scalar_static_f64[4]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[5]=p.p17;
        self.scalar_static_f64[6]=p.p18;
        self.scalar_static_f64[7]=p.p19;
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[8]=p.p0;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[8]);
        self.scalar_static_f64[9]=p.p172;
        self.scalar_static_bool[3]=(self.scalar_static_f64[9]>0.0);
        self.scalar_static_bool[4]=(self.scalar_static_bool[2]&&self.scalar_static_bool[3]);
        self.scalar_static_bool[5]=(self.scalar_static_f64[8]>0.0);
        self.scalar_static_f64[10]=p.p439;
        self.scalar_static_bool[6]=(self.scalar_static_f64[10]>0.0);
        self.scalar_static_bool[7]=(self.scalar_static_bool[5]&&self.scalar_static_bool[6]);
        self.scalar_static_bool[8]=(self.scalar_static_bool[4]||self.scalar_static_bool[7]);
        self.scalar_static_f64[11]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[12]=p.p5;
        self.scalar_static_f64[13]=(if (self.scalar_static_f64[11]!=0.0){self.scalar_static_f64[12]}else{0.0});
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[11]!=0.0));
        self.scalar_static_f64[14]=(if self.scalar_static_bool[9]{0.0}else{self.scalar_static_f64[13]});
        self.scalar_static_f64[15]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[16]=p.p23;
        self.scalar_static_f64[17]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[16]}else{0.0});
        self.scalar_static_f64[18]=p.p22;
        self.scalar_static_f64[19]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[18]}else{0.0});
        self.scalar_static_f64[20]=p.p25;
        self.scalar_static_f64[21]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[22]=p.p24;
        self.scalar_static_f64[23]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=p.p30;
        self.scalar_static_f64[25]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[24]}else{0.0});
        self.scalar_static_f64[26]=p.p41;
        self.scalar_static_f64[27]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[26]}else{0.0});
        self.scalar_static_f64[28]=p.p42;
        self.scalar_static_f64[29]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[28]}else{0.0});
        self.scalar_static_f64[30]=p.p43;
        self.scalar_static_f64[31]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[30]}else{0.0});
        self.scalar_static_f64[32]=p.p44;
        self.scalar_static_f64[33]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[32]}else{0.0});
        self.scalar_static_f64[34]=(if (self.scalar_static_f64[15]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[35]=p.p45;
        self.scalar_static_bool[10]=(self.scalar_static_f64[35]<0.0);
        self.scalar_static_f64[36]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_bool[11]=((self.scalar_static_f64[15]!=0.0)&&(self.scalar_static_f64[36]!=0.0));
        self.scalar_static_f64[37]=(if self.scalar_static_bool[11]{-1.0}else{self.scalar_static_f64[34]});
        self.scalar_static_f64[38]=(self.scalar_static_f64[35]).abs();
        self.scalar_static_bool[12]=(self.scalar_static_f64[38]<1e19);
        self.scalar_static_f64[39]=(if self.scalar_static_bool[12]{self.scalar_static_f64[38]}else{1e19});
        self.scalar_static_f64[40]=(self.scalar_static_f64[39]*1000000.0);
        self.scalar_static_f64[41]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[40]}else{0.0});
        self.scalar_static_f64[42]=p.p46;
        self.scalar_static_bool[13]=(self.scalar_static_f64[42]<0.0);
        self.scalar_static_f64[43]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_bool[14]=((self.scalar_static_f64[15]!=0.0)&&(self.scalar_static_f64[43]!=0.0));
        self.scalar_static_f64[44]=(if self.scalar_static_bool[14]{-1.0}else{self.scalar_static_f64[34]});
        self.scalar_static_f64[45]=(self.scalar_static_f64[42]).abs();
        self.scalar_static_bool[15]=(self.scalar_static_f64[45]>1e16);
        self.scalar_static_f64[46]=(if self.scalar_static_bool[15]{self.scalar_static_f64[45]}else{1e16});
        self.scalar_static_bool[16]=(self.scalar_static_f64[46]<1e21);
        self.scalar_static_f64[47]=(if self.scalar_static_bool[16]{self.scalar_static_f64[46]}else{1e21});
        self.scalar_static_f64[48]=(1000000.0*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[48]}else{0.0});
        self.scalar_static_f64[50]=p.p47;
        self.scalar_static_f64[51]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[50]}else{0.0});
        self.scalar_static_f64[52]=p.p48;
        self.scalar_static_f64[53]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[52]}else{0.0});
        self.scalar_static_f64[54]=p.p49;
        self.scalar_static_f64[55]=(1000000.0*self.scalar_static_f64[54]);
        self.scalar_static_f64[56]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[55]}else{0.0});
        self.scalar_static_f64[57]=p.p50;
        self.scalar_static_f64[58]=(1000000.0*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[58]}else{0.0});
        self.scalar_static_f64[60]=p.p51;
        self.scalar_static_f64[61]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[60]}else{0.0});
        self.scalar_static_f64[62]=p.p52;
        self.scalar_static_f64[63]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[62]}else{0.0});
        self.scalar_static_f64[64]=p.p53;
        self.scalar_static_f64[65]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[64]}else{0.0});
        self.scalar_static_f64[66]=p.p54;
        self.scalar_static_f64[67]=(1000000.0*self.scalar_static_f64[66]);
        self.scalar_static_f64[68]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[67]}else{0.0});
        self.scalar_static_f64[69]=p.p55;
        self.scalar_static_f64[70]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[69]}else{0.0});
        self.scalar_static_f64[71]=p.p56;
        self.scalar_static_f64[72]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[71]}else{0.0});
        self.scalar_static_f64[73]=p.p57;
        self.scalar_static_f64[74]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[73]}else{0.0});
        self.scalar_static_f64[75]=p.p58;
        self.scalar_static_f64[76]=(self.scalar_static_f64[74]*self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=(self.scalar_static_f64[33]*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(self.scalar_static_f64[77]/self.scalar_static_f64[27]);
        self.scalar_static_f64[79]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[78]}else{0.0});
        self.scalar_static_f64[80]=p.p59;
        self.scalar_static_f64[81]=(1000000.0*self.scalar_static_f64[80]);
        self.scalar_static_f64[82]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[81]}else{0.0});
        self.scalar_static_f64[83]=p.p60;
        self.scalar_static_f64[84]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[83]}else{0.0});
        self.scalar_static_f64[85]=p.p61;
        self.scalar_static_f64[86]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[85]}else{0.0});
        self.scalar_static_f64[87]=p.p62;
        self.scalar_static_f64[88]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[87]}else{0.0});
        self.scalar_static_f64[89]=p.p63;
        self.scalar_static_f64[90]=(self.scalar_static_f64[88]*self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=(self.scalar_static_f64[33]*self.scalar_static_f64[90]);
        self.scalar_static_f64[92]=(self.scalar_static_f64[91]/self.scalar_static_f64[27]);
        self.scalar_static_f64[93]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[92]}else{0.0});
        self.scalar_static_f64[94]=p.p64;
        self.scalar_static_f64[95]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[94]}else{0.0});
        self.scalar_static_f64[96]=p.p65;
        self.scalar_static_f64[97]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[96]}else{0.0});
        self.scalar_static_f64[98]=p.p66;
        self.scalar_static_f64[99]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[98]}else{0.0});
        self.scalar_static_f64[100]=p.p67;
        self.scalar_static_f64[101]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[100]}else{0.0});
        self.scalar_static_f64[102]=p.p68;
        self.scalar_static_f64[103]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[102]}else{0.0});
        self.scalar_static_f64[104]=p.p69;
        self.scalar_static_f64[105]=(self.scalar_static_f64[103]*self.scalar_static_f64[104]);
        self.scalar_static_f64[106]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[105]}else{0.0});
        self.scalar_static_f64[107]=p.p70;
        self.scalar_static_f64[108]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[107]}else{0.0});
        self.scalar_static_f64[109]=p.p71;
        self.scalar_static_f64[110]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[109]}else{0.0});
        self.scalar_static_f64[111]=p.p72;
        self.scalar_static_f64[112]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[111]}else{0.0});
        self.scalar_static_f64[113]=p.p73;
        self.scalar_static_f64[114]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[113]}else{0.0});
        self.scalar_static_f64[115]=p.p74;
        self.scalar_static_f64[116]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[115]}else{0.0});
        self.scalar_static_f64[117]=p.p75;
        self.scalar_static_f64[118]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[117]}else{0.0});
        self.scalar_static_f64[119]=p.p76;
        self.scalar_static_f64[120]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[119]}else{0.0});
        self.scalar_static_f64[121]=p.p77;
        self.scalar_static_f64[122]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[121]}else{0.0});
        self.scalar_static_f64[123]=p.p78;
        self.scalar_static_f64[124]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[123]}else{0.0});
        self.scalar_static_f64[125]=p.p79;
        self.scalar_static_f64[126]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[125]}else{0.0});
        self.scalar_static_f64[127]=p.p80;
        self.scalar_static_f64[128]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[127]}else{0.0});
        self.scalar_static_f64[129]=p.p81;
        self.scalar_static_f64[130]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[129]}else{0.0});
        self.scalar_static_f64[131]=p.p82;
        self.scalar_static_f64[132]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[131]}else{0.0});
        self.scalar_static_f64[133]=p.p83;
        self.scalar_static_f64[134]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[133]}else{0.0});
        self.scalar_static_f64[135]=p.p84;
        self.scalar_static_f64[136]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[135]}else{0.0});
        self.scalar_static_f64[137]=p.p85;
        self.scalar_static_f64[138]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[137]}else{0.0});
        self.scalar_static_f64[139]=p.p86;
        self.scalar_static_f64[140]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[139]}else{0.0});
        self.scalar_static_f64[141]=p.p87;
        self.scalar_static_f64[142]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[141]}else{0.0});
        self.scalar_static_f64[143]=p.p88;
        self.scalar_static_f64[144]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[143]}else{0.0});
        self.scalar_static_f64[145]=p.p89;
        self.scalar_static_f64[146]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[145]}else{0.0});
        self.scalar_static_f64[147]=p.p90;
        self.scalar_static_f64[148]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[147]}else{0.0});
        self.scalar_static_f64[149]=p.p91;
        self.scalar_static_f64[150]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[149]}else{0.0});
        self.scalar_static_f64[151]=p.p92;
        self.scalar_static_f64[152]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[151]}else{0.0});
        self.scalar_static_f64[153]=p.p93;
        self.scalar_static_f64[154]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[153]}else{0.0});
        self.scalar_static_f64[155]=p.p94;
        self.scalar_static_f64[156]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[155]}else{0.0});
        self.scalar_static_f64[157]=p.p95;
        self.scalar_static_f64[158]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[157]}else{0.0});
        self.scalar_static_f64[159]=p.p96;
        self.scalar_static_f64[160]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[159]}else{0.0});
        self.scalar_static_f64[161]=p.p97;
        self.scalar_static_f64[162]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[161]}else{0.0});
        self.scalar_static_f64[163]=p.p98;
        self.scalar_static_f64[164]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[163]}else{0.0});
        self.scalar_static_f64[165]=p.p99;
        self.scalar_static_f64[166]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[165]}else{0.0});
        self.scalar_static_f64[167]=p.p100;
        self.scalar_static_f64[168]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[167]}else{0.0});
        self.scalar_static_f64[169]=p.p101;
        self.scalar_static_f64[170]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[169]}else{0.0});
        self.scalar_static_f64[171]=p.p102;
        self.scalar_static_f64[172]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[171]}else{0.0});
        self.scalar_static_f64[173]=p.p103;
        self.scalar_static_f64[174]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[173]}else{0.0});
        self.scalar_static_f64[175]=p.p104;
        self.scalar_static_f64[176]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[175]}else{0.0});
        self.scalar_static_f64[177]=p.p105;
        self.scalar_static_f64[178]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[177]}else{0.0});
        self.scalar_static_f64[179]=p.p106;
        self.scalar_static_f64[180]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[179]}else{0.0});
        self.scalar_static_f64[181]=p.p120;
        self.scalar_static_f64[182]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[181]}else{0.0});
        self.scalar_static_f64[183]=p.p121;
        self.scalar_static_f64[184]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[183]}else{0.0});
        self.scalar_static_f64[185]=p.p107;
        self.scalar_static_f64[186]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[185]}else{0.0});
        self.scalar_static_f64[187]=p.p108;
        self.scalar_static_f64[188]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[187]}else{0.0});
        self.scalar_static_f64[189]=p.p109;
        self.scalar_static_f64[190]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[189]}else{0.0});
        self.scalar_static_f64[191]=p.p123;
        self.scalar_static_f64[192]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[193]=p.p110;
        self.scalar_static_f64[194]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[195]=p.p111;
        self.scalar_static_f64[196]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[197]=p.p112;
        self.scalar_static_f64[198]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[199]=p.p122;
        self.scalar_static_f64[200]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[199]}else{0.0});
        self.scalar_static_f64[201]=p.p113;
        self.scalar_static_f64[202]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[201]}else{0.0});
        self.scalar_static_f64[203]=p.p114;
        self.scalar_static_f64[204]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[203]}else{0.0});
        self.scalar_static_f64[205]=p.p115;
        self.scalar_static_f64[206]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[205]}else{0.0});
        self.scalar_static_f64[207]=p.p116;
        self.scalar_static_f64[208]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[207]}else{0.0});
        self.scalar_static_f64[209]=p.p117;
        self.scalar_static_f64[210]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[209]}else{0.0});
        self.scalar_static_f64[211]=p.p118;
        self.scalar_static_f64[212]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[211]}else{0.0});
        self.scalar_static_f64[213]=p.p119;
        self.scalar_static_f64[214]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[213]}else{0.0});
        self.scalar_static_f64[215]=p.p124;
        self.scalar_static_f64[216]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[215]}else{0.0});
        self.scalar_static_f64[217]=p.p125;
        self.scalar_static_f64[218]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[217]}else{0.0});
        self.scalar_static_f64[219]=p.p126;
        self.scalar_static_f64[220]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[219]}else{0.0});
        self.scalar_static_f64[221]=p.p127;
        self.scalar_static_f64[222]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[221]}else{0.0});
        self.scalar_static_f64[223]=p.p128;
        self.scalar_static_f64[224]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[223]}else{0.0});
        self.scalar_static_f64[225]=p.p129;
        self.scalar_static_f64[226]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[225]}else{0.0});
        self.scalar_static_f64[227]=p.p130;
        self.scalar_static_f64[228]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[227]}else{0.0});
        self.scalar_static_f64[229]=p.p131;
        self.scalar_static_f64[230]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[229]}else{0.0});
        self.scalar_static_f64[231]=p.p132;
        self.scalar_static_f64[232]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[231]}else{0.0});
        self.scalar_static_f64[233]=p.p133;
        self.scalar_static_f64[234]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[233]}else{0.0});
        self.scalar_static_f64[235]=p.p147;
        self.scalar_static_f64[236]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[235]}else{0.0});
        self.scalar_static_f64[237]=p.p148;
        self.scalar_static_f64[238]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[237]}else{0.0});
        self.scalar_static_f64[239]=p.p149;
        self.scalar_static_f64[240]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[239]}else{0.0});
        self.scalar_static_f64[241]=p.p150;
        self.scalar_static_f64[242]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[241]}else{0.0});
        self.scalar_static_f64[243]=p.p134;
        self.scalar_static_f64[244]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[243]}else{0.0});
        self.scalar_static_f64[245]=p.p135;
        self.scalar_static_f64[246]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[245]}else{0.0});
        self.scalar_static_f64[247]=p.p136;
        self.scalar_static_f64[248]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[247]}else{0.0});
        self.scalar_static_f64[249]=p.p137;
        self.scalar_static_f64[250]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[249]}else{0.0});
        self.scalar_static_f64[251]=p.p138;
        self.scalar_static_f64[252]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[251]}else{0.0});
        self.scalar_static_f64[253]=p.p139;
        self.scalar_static_f64[254]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[253]}else{0.0});
        self.scalar_static_f64[255]=p.p140;
        self.scalar_static_f64[256]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[255]}else{0.0});
        self.scalar_static_f64[257]=p.p141;
        self.scalar_static_f64[258]=(self.scalar_static_f64[256]*self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(self.scalar_static_f64[33]*self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[259]/self.scalar_static_f64[27]);
        self.scalar_static_f64[261]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[260]}else{0.0});
        self.scalar_static_f64[262]=p.p142;
        self.scalar_static_f64[263]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[262]}else{0.0});
        self.scalar_static_f64[264]=p.p143;
        self.scalar_static_f64[265]=(self.scalar_static_f64[263]*self.scalar_static_f64[264]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[33]*self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[266]/self.scalar_static_f64[27]);
        self.scalar_static_f64[268]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[267]}else{0.0});
        self.scalar_static_f64[269]=p.p144;
        self.scalar_static_f64[270]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[269]}else{0.0});
        self.scalar_static_f64[271]=p.p145;
        self.scalar_static_f64[272]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[271]}else{0.0});
        self.scalar_static_f64[273]=p.p146;
        self.scalar_static_f64[274]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[275]=p.p151;
        self.scalar_static_f64[276]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=p.p152;
        self.scalar_static_f64[278]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_f64[279]=p.p153;
        self.scalar_static_f64[280]=(1000000.0*self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[280]}else{0.0});
        self.scalar_static_f64[282]=p.p154;
        self.scalar_static_f64[283]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[282]}else{0.0});
        self.scalar_static_f64[284]=p.p155;
        self.scalar_static_f64[285]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[284]}else{0.0});
        self.scalar_static_f64[286]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[61]}else{0.0});
        self.scalar_static_f64[287]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[63]}else{0.0});
        self.scalar_static_f64[288]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[74]}else{0.0});
        self.scalar_static_f64[289]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[79]}else{0.0});
        self.scalar_static_f64[290]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[88]}else{0.0});
        self.scalar_static_f64[291]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[93]}else{0.0});
        self.scalar_static_f64[292]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[154]}else{0.0});
        self.scalar_static_f64[293]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[162]}else{0.0});
        self.scalar_static_f64[294]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[164]}else{0.0});
        self.scalar_static_f64[295]=p.p11;
        self.scalar_static_bool[17]=(self.scalar_static_f64[295]>0.0);
        self.scalar_static_f64[296]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=((self.scalar_static_f64[15]!=0.0)&&(self.scalar_static_f64[296]!=0.0));
        self.scalar_static_f64[297]=(if self.scalar_static_bool[18]{self.scalar_static_f64[60]}else{self.scalar_static_f64[286]});
        self.scalar_static_f64[298]=if param_given[156]{1.0}else{0.0};
        self.scalar_static_bool[19]=(1.0==self.scalar_static_f64[298]);
        self.scalar_static_f64[299]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_bool[20]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[299]!=0.0));
        self.scalar_static_f64[300]=p.p156;
        self.scalar_static_f64[301]=(if self.scalar_static_bool[20]{self.scalar_static_f64[300]}else{self.scalar_static_f64[297]});
        self.scalar_static_f64[302]=(if self.scalar_static_bool[18]{self.scalar_static_f64[62]}else{self.scalar_static_f64[287]});
        self.scalar_static_f64[303]=if param_given[157]{1.0}else{0.0};
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[303]);
        self.scalar_static_f64[304]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_bool[22]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[304]!=0.0));
        self.scalar_static_f64[305]=p.p157;
        self.scalar_static_f64[306]=(if self.scalar_static_bool[22]{self.scalar_static_f64[305]}else{self.scalar_static_f64[302]});
        self.scalar_static_f64[307]=(if self.scalar_static_bool[18]{self.scalar_static_f64[73]}else{self.scalar_static_f64[288]});
        self.scalar_static_f64[308]=if param_given[158]{1.0}else{0.0};
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[308]);
        self.scalar_static_f64[309]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[309]!=0.0));
        self.scalar_static_f64[310]=p.p158;
        self.scalar_static_f64[311]=(if self.scalar_static_bool[24]{self.scalar_static_f64[310]}else{self.scalar_static_f64[307]});
        self.scalar_static_f64[312]=(self.scalar_static_f64[75]*self.scalar_static_f64[311]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[33]*self.scalar_static_f64[312]);
        self.scalar_static_f64[314]=(self.scalar_static_f64[313]/self.scalar_static_f64[27]);
        self.scalar_static_f64[315]=(if self.scalar_static_bool[18]{self.scalar_static_f64[314]}else{self.scalar_static_f64[289]});
        self.scalar_static_f64[316]=(if self.scalar_static_bool[18]{self.scalar_static_f64[87]}else{self.scalar_static_f64[290]});
        self.scalar_static_f64[317]=if param_given[159]{1.0}else{0.0};
        self.scalar_static_bool[25]=(1.0==self.scalar_static_f64[317]);
        self.scalar_static_f64[318]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_bool[26]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[318]!=0.0));
        self.scalar_static_f64[319]=p.p159;
        self.scalar_static_f64[320]=(if self.scalar_static_bool[26]{self.scalar_static_f64[319]}else{self.scalar_static_f64[316]});
        self.scalar_static_f64[321]=(self.scalar_static_f64[89]*self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[33]*self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[322]/self.scalar_static_f64[27]);
        self.scalar_static_f64[324]=(if self.scalar_static_bool[18]{self.scalar_static_f64[323]}else{self.scalar_static_f64[291]});
        self.scalar_static_f64[325]=(if self.scalar_static_bool[18]{self.scalar_static_f64[153]}else{self.scalar_static_f64[292]});
        self.scalar_static_f64[326]=if param_given[160]{1.0}else{0.0};
        self.scalar_static_bool[27]=(1.0==self.scalar_static_f64[326]);
        self.scalar_static_f64[327]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_bool[28]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[327]!=0.0));
        self.scalar_static_f64[328]=p.p160;
        self.scalar_static_f64[329]=(if self.scalar_static_bool[28]{self.scalar_static_f64[328]}else{self.scalar_static_f64[325]});
        self.scalar_static_f64[330]=(if self.scalar_static_bool[18]{self.scalar_static_f64[161]}else{self.scalar_static_f64[293]});
        self.scalar_static_f64[331]=if param_given[161]{1.0}else{0.0};
        self.scalar_static_bool[29]=(1.0==self.scalar_static_f64[331]);
        self.scalar_static_f64[332]=(if self.scalar_static_bool[29]{1.0}else{0.0});
        self.scalar_static_bool[30]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[332]!=0.0));
        self.scalar_static_f64[333]=p.p161;
        self.scalar_static_f64[334]=(if self.scalar_static_bool[30]{self.scalar_static_f64[333]}else{self.scalar_static_f64[330]});
        self.scalar_static_f64[335]=(if self.scalar_static_bool[18]{self.scalar_static_f64[163]}else{self.scalar_static_f64[294]});
        self.scalar_static_f64[336]=if param_given[162]{1.0}else{0.0};
        self.scalar_static_bool[31]=(1.0==self.scalar_static_f64[336]);
        self.scalar_static_f64[337]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(self.scalar_static_bool[18]&&(self.scalar_static_f64[337]!=0.0));
        self.scalar_static_f64[338]=p.p162;
        self.scalar_static_f64[339]=(if self.scalar_static_bool[32]{self.scalar_static_f64[338]}else{self.scalar_static_f64[335]});
        self.scalar_static_f64[340]=p.p163;
        self.scalar_static_f64[341]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[340]}else{0.0});
        self.scalar_static_f64[342]=p.p164;
        self.scalar_static_f64[343]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[342]}else{0.0});
        self.scalar_static_f64[344]=p.p165;
        self.scalar_static_f64[345]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[344]}else{0.0});
        self.scalar_static_f64[346]=p.p166;
        self.scalar_static_f64[347]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[346]}else{0.0});
        self.scalar_static_f64[348]=p.p167;
        self.scalar_static_f64[349]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[348]}else{0.0});
        self.scalar_static_f64[350]=p.p168;
        self.scalar_static_f64[351]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[350]}else{0.0});
        self.scalar_static_f64[352]=p.p169;
        self.scalar_static_f64[353]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[354]=p.p170;
        self.scalar_static_f64[355]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_f64[356]=p.p171;
        self.scalar_static_f64[357]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[356]}else{0.0});
        self.scalar_static_f64[358]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[9]}else{0.0});
        self.scalar_static_f64[359]=p.p173;
        self.scalar_static_f64[360]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[359]}else{0.0});
        self.scalar_static_f64[361]=p.p174;
        self.scalar_static_f64[362]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[361]}else{0.0});
        self.scalar_static_f64[363]=p.p175;
        self.scalar_static_f64[364]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[363]}else{0.0});
        self.scalar_static_f64[365]=p.p176;
        self.scalar_static_f64[366]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[365]}else{0.0});
        self.scalar_static_f64[367]=p.p183;
        self.scalar_static_f64[368]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[367]}else{0.0});
        self.scalar_static_f64[369]=p.p184;
        self.scalar_static_f64[370]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[369]}else{0.0});
        self.scalar_static_f64[371]=p.p185;
        self.scalar_static_f64[372]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[371]}else{0.0});
        self.scalar_static_f64[373]=p.p186;
        self.scalar_static_f64[374]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[373]}else{0.0});
        self.scalar_static_bool[33]=(!(self.scalar_static_f64[15]!=0.0));
        self.scalar_static_f64[375]=p.p29;
        self.scalar_static_f64[376]=(1.0/self.scalar_static_f64[375]);
        self.scalar_static_f64[377]=(if self.scalar_static_bool[33]{self.scalar_static_f64[376]}else{0.0});
        self.scalar_static_f64[378]=p.p21;
        self.scalar_static_f64[379]=(self.scalar_static_f64[377]*self.scalar_static_f64[378]);
        self.scalar_static_bool[34]=(self.scalar_static_f64[379]>1e-9);
        self.scalar_static_f64[380]=(if self.scalar_static_bool[34]{self.scalar_static_f64[379]}else{1e-9});
        self.scalar_static_f64[381]=(if self.scalar_static_bool[33]{self.scalar_static_f64[380]}else{0.0});
        self.scalar_static_f64[382]=(self.scalar_static_f64[16]*self.scalar_static_f64[377]);
        self.scalar_static_f64[383]=(if self.scalar_static_bool[33]{self.scalar_static_f64[382]}else{self.scalar_static_f64[17]});
        self.scalar_static_f64[384]=(self.scalar_static_f64[18]*self.scalar_static_f64[377]);
        self.scalar_static_f64[385]=(if self.scalar_static_bool[33]{self.scalar_static_f64[384]}else{self.scalar_static_f64[19]});
        self.scalar_static_f64[386]=(self.scalar_static_f64[20]*self.scalar_static_f64[377]);
        self.scalar_static_f64[387]=(if self.scalar_static_bool[33]{self.scalar_static_f64[386]}else{self.scalar_static_f64[21]});
        self.scalar_static_f64[388]=(self.scalar_static_f64[22]*self.scalar_static_f64[377]);
        self.scalar_static_f64[389]=(if self.scalar_static_bool[33]{self.scalar_static_f64[388]}else{self.scalar_static_f64[23]});
        self.scalar_static_f64[390]=(self.scalar_static_f64[24]*self.scalar_static_f64[375]);
        self.scalar_static_f64[391]=(if self.scalar_static_bool[33]{self.scalar_static_f64[390]}else{self.scalar_static_f64[25]});
        self.scalar_static_f64[392]=(if self.scalar_static_bool[33]{1e-6}else{0.0});
        self.scalar_static_f64[393]=p.p20;
        self.scalar_static_f64[394]=(self.scalar_static_f64[392]/self.scalar_static_f64[393]);
        self.scalar_static_f64[395]=(if self.scalar_static_bool[33]{self.scalar_static_f64[394]}else{0.0});
        self.scalar_static_f64[396]=(self.scalar_static_f64[392]/self.scalar_static_f64[381]);
        self.scalar_static_f64[397]=(if self.scalar_static_bool[33]{self.scalar_static_f64[396]}else{0.0});
        self.scalar_static_f64[398]=p.p187;
        self.scalar_static_f64[399]=p.p188;
        self.scalar_static_f64[400]=(self.scalar_static_f64[395]*self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=(1.0+self.scalar_static_f64[400]);
        self.scalar_static_f64[402]=(self.scalar_static_f64[398]*self.scalar_static_f64[401]);
        self.scalar_static_f64[403]=p.p189;
        self.scalar_static_f64[404]=(self.scalar_static_f64[397]*self.scalar_static_f64[403]);
        self.scalar_static_f64[405]=(1.0+self.scalar_static_f64[404]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[402]*self.scalar_static_f64[405]);
        self.scalar_static_f64[407]=(if self.scalar_static_bool[33]{self.scalar_static_f64[406]}else{0.0});
        self.scalar_static_f64[408]=p.p191;
        self.scalar_static_f64[409]=p.p193;
        self.scalar_static_f64[410]=(self.scalar_static_f64[397]*self.scalar_static_f64[409]);
        self.scalar_static_f64[411]=(1.0+self.scalar_static_f64[410]);
        self.scalar_static_f64[412]=(self.scalar_static_f64[408]*self.scalar_static_f64[411]);
        self.scalar_static_f64[413]=p.p192;
        self.scalar_static_f64[414]=(self.scalar_static_f64[395]*self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(1.0+self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=(self.scalar_static_f64[412]*self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=(if self.scalar_static_bool[33]{self.scalar_static_f64[416]}else{0.0});
        self.scalar_static_f64[418]=(self.scalar_static_f64[393]+self.scalar_static_f64[407]);
        self.scalar_static_f64[419]=p.p190;
        self.scalar_static_f64[420]=(2.0*self.scalar_static_f64[419]);
        self.scalar_static_f64[421]=(self.scalar_static_f64[418]-self.scalar_static_f64[420]);
        self.scalar_static_bool[35]=(self.scalar_static_f64[421]>1e-9);
        self.scalar_static_f64[422]=(if self.scalar_static_bool[35]{self.scalar_static_f64[421]}else{1e-9});
        self.scalar_static_f64[423]=(if self.scalar_static_bool[33]{self.scalar_static_f64[422]}else{0.0});
        self.scalar_static_f64[424]=(self.scalar_static_f64[381]+self.scalar_static_f64[417]);
        self.scalar_static_f64[425]=p.p194;
        self.scalar_static_f64[426]=(2.0*self.scalar_static_f64[425]);
        self.scalar_static_f64[427]=(self.scalar_static_f64[424]-self.scalar_static_f64[426]);
        self.scalar_static_bool[36]=(self.scalar_static_f64[427]>1e-9);
        self.scalar_static_f64[428]=(if self.scalar_static_bool[36]{self.scalar_static_f64[427]}else{1e-9});
        self.scalar_static_f64[429]=(if self.scalar_static_bool[33]{self.scalar_static_f64[428]}else{0.0});
        self.scalar_static_f64[430]=p.p195;
        self.scalar_static_f64[431]=(self.scalar_static_f64[421]+self.scalar_static_f64[430]);
        self.scalar_static_bool[37]=(self.scalar_static_f64[431]>1e-9);
        self.scalar_static_f64[432]=(if self.scalar_static_bool[37]{self.scalar_static_f64[431]}else{1e-9});
        self.scalar_static_f64[433]=(if self.scalar_static_bool[33]{self.scalar_static_f64[432]}else{0.0});
        self.scalar_static_f64[434]=p.p196;
        self.scalar_static_f64[435]=(self.scalar_static_f64[427]+self.scalar_static_f64[434]);
        self.scalar_static_bool[38]=(self.scalar_static_f64[435]>1e-9);
        self.scalar_static_f64[436]=(if self.scalar_static_bool[38]{self.scalar_static_f64[435]}else{1e-9});
        self.scalar_static_f64[437]=(if self.scalar_static_bool[33]{self.scalar_static_f64[436]}else{0.0});
        self.scalar_static_f64[438]=(self.scalar_static_f64[392]/self.scalar_static_f64[423]);
        self.scalar_static_f64[439]=(if self.scalar_static_bool[33]{self.scalar_static_f64[438]}else{0.0});
        self.scalar_static_f64[440]=(self.scalar_static_f64[392]/self.scalar_static_f64[429]);
        self.scalar_static_f64[441]=(if self.scalar_static_bool[33]{self.scalar_static_f64[440]}else{0.0});
        self.scalar_static_f64[442]=(self.scalar_static_f64[439]*self.scalar_static_f64[441]);
        self.scalar_static_f64[443]=(if self.scalar_static_bool[33]{self.scalar_static_f64[442]}else{0.0});
        self.scalar_static_bool[39]=(self.scalar_static_f64[418]>1e-9);
        self.scalar_static_f64[444]=(if self.scalar_static_bool[39]{self.scalar_static_f64[418]}else{1e-9});
        self.scalar_static_bool[40]=(self.scalar_static_f64[424]>1e-9);
        self.scalar_static_f64[445]=(if self.scalar_static_bool[40]{self.scalar_static_f64[424]}else{1e-9});
        self.scalar_static_f64[446]=(if self.scalar_static_bool[33]{self.scalar_static_f64[444]}else{0.0});
        self.scalar_static_f64[447]=p.p489;
        self.scalar_static_f64[448]=(self.scalar_static_f64[446]+self.scalar_static_f64[447]);
        self.scalar_static_bool[41]=(self.scalar_static_f64[448]>1e-9);
        self.scalar_static_f64[449]=(if self.scalar_static_bool[41]{self.scalar_static_f64[448]}else{1e-9});
        self.scalar_static_f64[450]=(if self.scalar_static_bool[33]{self.scalar_static_f64[449]}else{0.0});
        self.scalar_static_f64[451]=(if self.scalar_static_bool[33]{self.scalar_static_f64[445]}else{0.0});
        self.scalar_static_f64[452]=p.p38;
        self.scalar_static_f64[453]=(0.5*self.scalar_static_f64[417]);
        self.scalar_static_f64[454]=(self.scalar_static_f64[452]-self.scalar_static_f64[453]);
        self.scalar_static_bool[42]=(self.scalar_static_f64[454]>1e-9);
        self.scalar_static_f64[455]=(if self.scalar_static_bool[42]{self.scalar_static_f64[454]}else{1e-9});
        self.scalar_static_f64[456]=(if self.scalar_static_bool[33]{self.scalar_static_f64[455]}else{0.0});
        self.scalar_static_f64[457]=p.p197;
        self.scalar_static_f64[458]=(if self.scalar_static_bool[33]{self.scalar_static_f64[457]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[459]=p.p198;
        self.scalar_static_f64[460]=(if self.scalar_static_bool[33]{self.scalar_static_f64[459]}else{self.scalar_static_f64[29]});
        self.scalar_static_f64[461]=p.p199;
        self.scalar_static_f64[462]=(if self.scalar_static_bool[33]{self.scalar_static_f64[461]}else{self.scalar_static_f64[31]});
        self.scalar_static_f64[463]=p.p200;
        self.scalar_static_f64[464]=(if self.scalar_static_bool[33]{self.scalar_static_f64[463]}else{self.scalar_static_f64[33]});
        self.scalar_static_f64[465]=(if self.scalar_static_bool[33]{1.0}else{self.scalar_static_f64[37]});
        self.scalar_static_f64[466]=p.p201;
        self.scalar_static_bool[43]=(self.scalar_static_f64[466]<0.0);
        self.scalar_static_f64[467]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_bool[44]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[467]!=0.0));
        self.scalar_static_f64[468]=(if self.scalar_static_bool[44]{-1.0}else{self.scalar_static_f64[465]});
        self.scalar_static_f64[469]=(self.scalar_static_f64[466]).abs();
        self.scalar_static_bool[45]=(self.scalar_static_f64[469]<1e19);
        self.scalar_static_f64[470]=(if self.scalar_static_bool[45]{self.scalar_static_f64[469]}else{1e19});
        self.scalar_static_f64[471]=(1000000.0*self.scalar_static_f64[470]);
        self.scalar_static_f64[472]=(if self.scalar_static_bool[33]{self.scalar_static_f64[471]}else{self.scalar_static_f64[41]});
        self.scalar_static_f64[473]=(if self.scalar_static_bool[33]{1.0}else{self.scalar_static_f64[44]});
        self.scalar_static_f64[474]=p.p202;
        self.scalar_static_bool[46]=(self.scalar_static_f64[474]<0.0);
        self.scalar_static_f64[475]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_bool[47]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[475]!=0.0));
        self.scalar_static_f64[476]=(if self.scalar_static_bool[47]{-1.0}else{self.scalar_static_f64[473]});
        self.scalar_static_f64[477]=(self.scalar_static_f64[474]).abs();
        self.scalar_static_bool[48]=(self.scalar_static_f64[477]>1e16);
        self.scalar_static_f64[478]=(if self.scalar_static_bool[48]{self.scalar_static_f64[477]}else{1e16});
        self.scalar_static_bool[49]=(self.scalar_static_f64[478]<1e21);
        self.scalar_static_f64[479]=(if self.scalar_static_bool[49]{self.scalar_static_f64[478]}else{1e21});
        self.scalar_static_f64[480]=(1000000.0*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(if self.scalar_static_bool[33]{self.scalar_static_f64[480]}else{self.scalar_static_f64[49]});
        self.scalar_static_f64[482]=p.p203;
        self.scalar_static_f64[483]=(if self.scalar_static_bool[33]{self.scalar_static_f64[482]}else{self.scalar_static_f64[51]});
        self.scalar_static_f64[484]=p.p204;
        self.scalar_static_f64[485]=(if self.scalar_static_bool[33]{self.scalar_static_f64[484]}else{self.scalar_static_f64[53]});
        self.scalar_static_f64[486]=p.p205;
        self.scalar_static_f64[487]=(1000000.0*self.scalar_static_f64[486]);
        self.scalar_static_f64[488]=(if self.scalar_static_bool[33]{self.scalar_static_f64[487]}else{self.scalar_static_f64[56]});
        self.scalar_static_f64[489]=p.p206;
        self.scalar_static_f64[490]=(1000000.0*self.scalar_static_f64[489]);
        self.scalar_static_f64[491]=(if self.scalar_static_bool[33]{self.scalar_static_f64[490]}else{self.scalar_static_f64[59]});
        self.scalar_static_f64[492]=p.p208;
        self.scalar_static_f64[493]=p.p209;
        self.scalar_static_f64[494]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[493]);
        self.scalar_static_f64[495]=(self.scalar_static_f64[492]*self.scalar_static_f64[494]);
        self.scalar_static_f64[496]=p.p210;
        self.scalar_static_f64[497]=p.p211;
        self.scalar_static_f64[498]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[497]);
        self.scalar_static_f64[499]=(self.scalar_static_f64[496]*self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=(1.0+self.scalar_static_f64[499]);
        self.scalar_static_f64[501]=(self.scalar_static_f64[495]/self.scalar_static_f64[500]);
        self.scalar_static_f64[502]=p.p207;
        self.scalar_static_f64[503]=p.p212;
        self.scalar_static_f64[504]=(self.scalar_static_f64[441]*self.scalar_static_f64[503]);
        self.scalar_static_f64[505]=p.p213;
        self.scalar_static_f64[506]=(self.scalar_static_f64[443]*self.scalar_static_f64[505]);
        self.scalar_static_f64[507]=p.p214;
        self.scalar_static_f64[508]=p.p215;
        self.scalar_static_f64[509]=(self.scalar_static_f64[464]*self.scalar_static_f64[508]);
        self.scalar_static_f64[510]=(self.scalar_static_f64[509]/self.scalar_static_f64[458]);
        self.scalar_static_f64[511]=p.p216;
        self.scalar_static_f64[512]=p.p217;
        self.scalar_static_f64[513]=(self.scalar_static_f64[439]*self.scalar_static_f64[512]);
        self.scalar_static_f64[514]=(1.0+self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(self.scalar_static_f64[511]*self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=p.p218;
        self.scalar_static_f64[517]=(self.scalar_static_f64[441]*self.scalar_static_f64[516]);
        self.scalar_static_f64[518]=(1.0+self.scalar_static_f64[517]);
        self.scalar_static_f64[519]=(self.scalar_static_f64[515]*self.scalar_static_f64[518]);
        self.scalar_static_f64[520]=p.p219;
        self.scalar_static_f64[521]=(self.scalar_static_f64[443]*self.scalar_static_f64[520]);
        self.scalar_static_f64[522]=(1.0+self.scalar_static_f64[521]);
        self.scalar_static_f64[523]=(self.scalar_static_f64[519]*self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(if self.scalar_static_bool[33]{self.scalar_static_f64[523]}else{self.scalar_static_f64[65]});
        self.scalar_static_f64[525]=p.p220;
        self.scalar_static_f64[526]=p.p221;
        self.scalar_static_f64[527]=(self.scalar_static_f64[439]*self.scalar_static_f64[526]);
        self.scalar_static_f64[528]=(1.0+self.scalar_static_f64[527]);
        self.scalar_static_f64[529]=(self.scalar_static_f64[525]*self.scalar_static_f64[528]);
        self.scalar_static_f64[530]=(1000000.0*self.scalar_static_f64[529]);
        self.scalar_static_f64[531]=(if self.scalar_static_bool[33]{self.scalar_static_f64[530]}else{0.0});
        self.scalar_static_bool[50]=(self.scalar_static_f64[531]>1e25);
        self.scalar_static_f64[532]=(if self.scalar_static_bool[50]{self.scalar_static_f64[531]}else{1e25});
        self.scalar_static_bool[51]=(self.scalar_static_f64[532]<1e28);
        self.scalar_static_f64[533]=(if self.scalar_static_bool[51]{self.scalar_static_f64[532]}else{1e28});
        self.scalar_static_f64[534]=(if self.scalar_static_bool[33]{self.scalar_static_f64[533]}else{self.scalar_static_f64[68]});
        self.scalar_static_f64[535]=p.p222;
        self.scalar_static_f64[536]=(if self.scalar_static_bool[33]{self.scalar_static_f64[535]}else{self.scalar_static_f64[70]});
        self.scalar_static_f64[537]=p.p223;
        self.scalar_static_f64[538]=(if self.scalar_static_bool[33]{self.scalar_static_f64[537]}else{self.scalar_static_f64[72]});
        self.scalar_static_f64[539]=(1.0-self.scalar_static_f64[462]);
        self.scalar_static_f64[540]=(if self.scalar_static_bool[33]{self.scalar_static_f64[539]}else{0.0});
        self.scalar_static_f64[541]=(self.scalar_static_f64[540]*1.04479e-10);
        self.scalar_static_f64[542]=(self.scalar_static_f64[462]*1.43438e-10);
        self.scalar_static_f64[543]=(self.scalar_static_f64[541]+self.scalar_static_f64[542]);
        self.scalar_static_f64[544]=(if self.scalar_static_bool[33]{self.scalar_static_f64[543]}else{0.0});
        self.scalar_static_f64[545]=(self.scalar_static_f64[544]/3.45313e-11);
        self.scalar_static_f64[546]=(self.scalar_static_f64[460]*self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=(self.scalar_static_f64[458]+4e-10);
        self.scalar_static_f64[548]=(self.scalar_static_f64[546]*self.scalar_static_f64[547]);
        self.scalar_static_f64[549]=(self.scalar_static_f64[548]).sqrt();
        self.scalar_static_f64[550]=(self.scalar_static_f64[549]/self.scalar_static_f64[423]);
        self.scalar_static_f64[551]=(if self.scalar_static_bool[33]{self.scalar_static_f64[550]}else{0.0});
        self.scalar_static_f64[552]=p.p224;
        self.scalar_static_f64[553]=(2.0*self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=p.p225;
        self.scalar_static_f64[555]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[554]);
        self.scalar_static_f64[556]=(self.scalar_static_f64[553]*self.scalar_static_f64[555]);
        self.scalar_static_f64[557]=p.p226;
        self.scalar_static_f64[558]=(self.scalar_static_f64[441]*self.scalar_static_f64[557]);
        self.scalar_static_f64[559]=(1.0+self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[556]*self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=(if self.scalar_static_bool[33]{self.scalar_static_f64[560]}else{0.0});
        self.scalar_static_bool[52]=(self.scalar_static_f64[561]>0.0);
        self.scalar_static_f64[562]=(if self.scalar_static_bool[52]{self.scalar_static_f64[561]}else{0.0});
        self.scalar_static_bool[53]=(self.scalar_static_f64[562]<5.0);
        self.scalar_static_f64[563]=(if self.scalar_static_bool[53]{self.scalar_static_f64[562]}else{5.0});
        self.scalar_static_f64[564]=(if self.scalar_static_bool[33]{self.scalar_static_f64[563]}else{self.scalar_static_f64[74]});
        self.scalar_static_f64[565]=p.p227;
        self.scalar_static_f64[566]=(self.scalar_static_f64[564]*self.scalar_static_f64[565]);
        self.scalar_static_f64[567]=(self.scalar_static_f64[464]*self.scalar_static_f64[566]);
        self.scalar_static_f64[568]=(self.scalar_static_f64[567]/self.scalar_static_f64[458]);
        self.scalar_static_f64[569]=(if self.scalar_static_bool[33]{self.scalar_static_f64[568]}else{self.scalar_static_f64[79]});
        self.scalar_static_f64[570]=p.p228;
        self.scalar_static_f64[571]=(1000000.0*self.scalar_static_f64[570]);
        self.scalar_static_f64[572]=(if self.scalar_static_bool[33]{self.scalar_static_f64[571]}else{self.scalar_static_f64[82]});
        self.scalar_static_f64[573]=p.p229;
        self.scalar_static_f64[574]=(if self.scalar_static_bool[33]{self.scalar_static_f64[573]}else{self.scalar_static_f64[84]});
        self.scalar_static_f64[575]=p.p230;
        self.scalar_static_f64[576]=(self.scalar_static_f64[441]*self.scalar_static_f64[575]);
        self.scalar_static_f64[577]=(if self.scalar_static_bool[33]{self.scalar_static_f64[576]}else{0.0});
        self.scalar_static_bool[54]=(self.scalar_static_f64[577]> -1.0);
        self.scalar_static_f64[578]=(if self.scalar_static_bool[54]{self.scalar_static_f64[577]}else{-1.0});
        self.scalar_static_bool[55]=(self.scalar_static_f64[578]<1.0);
        self.scalar_static_f64[579]=(if self.scalar_static_bool[55]{self.scalar_static_f64[578]}else{1.0});
        self.scalar_static_f64[580]=(if self.scalar_static_bool[33]{self.scalar_static_f64[579]}else{self.scalar_static_f64[86]});
        self.scalar_static_f64[581]=p.p232;
        self.scalar_static_f64[582]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[581]);
        self.scalar_static_f64[583]=p.p233;
        self.scalar_static_f64[584]=(self.scalar_static_f64[441]*self.scalar_static_f64[583]);
        self.scalar_static_f64[585]=(1.0+self.scalar_static_f64[584]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[582]*self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=p.p231;
        self.scalar_static_f64[588]=p.p234;
        self.scalar_static_f64[589]=p.p235;
        self.scalar_static_f64[590]=p.p236;
        self.scalar_static_f64[591]=(if self.scalar_static_bool[33]{self.scalar_static_f64[590]}else{self.scalar_static_f64[97]});
        self.scalar_static_f64[592]=p.p237;
        self.scalar_static_f64[593]=(self.scalar_static_f64[439]*self.scalar_static_f64[592]);
        self.scalar_static_f64[594]=p.p238;
        self.scalar_static_f64[595]=(self.scalar_static_f64[441]*self.scalar_static_f64[594]);
        self.scalar_static_f64[596]=(1.0+self.scalar_static_f64[595]);
        self.scalar_static_bool[56]=(self.scalar_static_f64[596]>0.001);
        self.scalar_static_f64[597]=(if self.scalar_static_bool[56]{self.scalar_static_f64[596]}else{0.001});
        self.scalar_static_f64[598]=(self.scalar_static_f64[593]/self.scalar_static_f64[597]);
        self.scalar_static_f64[599]=(if self.scalar_static_bool[33]{self.scalar_static_f64[598]}else{self.scalar_static_f64[99]});
        self.scalar_static_f64[600]=p.p239;
        self.scalar_static_f64[601]=(if self.scalar_static_bool[33]{self.scalar_static_f64[600]}else{self.scalar_static_f64[101]});
        self.scalar_static_f64[602]=(-self.scalar_static_f64[423]);
        self.scalar_static_f64[603]=p.p243;
        self.scalar_static_f64[604]=p.p244;
        self.scalar_static_f64[605]=(self.scalar_static_f64[441]*self.scalar_static_f64[604]);
        self.scalar_static_f64[606]=(1.0+self.scalar_static_f64[605]);
        self.scalar_static_bool[57]=(self.scalar_static_f64[606]>0.001);
        self.scalar_static_f64[607]=(if self.scalar_static_bool[57]{self.scalar_static_f64[606]}else{0.001});
        self.scalar_static_f64[608]=(self.scalar_static_f64[603]*self.scalar_static_f64[607]);
        self.scalar_static_f64[609]=(self.scalar_static_f64[602]/self.scalar_static_f64[608]);
        self.scalar_static_f64[610]=(if self.scalar_static_bool[33]{self.scalar_static_f64[609]}else{0.0});
        self.scalar_static_bool[58]=(self.scalar_static_f64[610]> -80.0);
        self.scalar_static_f64[611]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_bool[59]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[611]!=0.0));
        self.scalar_static_f64[612]=(self.scalar_static_f64[610]).exp();
        self.scalar_static_f64[613]=(if self.scalar_static_bool[59]{self.scalar_static_f64[612]}else{0.0});
        self.scalar_static_bool[60]=(!(self.scalar_static_f64[611]!=0.0));
        self.scalar_static_bool[61]=(self.scalar_static_bool[33]&&self.scalar_static_bool[60]);
        self.scalar_static_f64[614]=(-self.scalar_static_f64[610]);
        self.scalar_static_f64[615]=(self.scalar_static_f64[614]-80.0);
        self.scalar_static_f64[616]=(0.5*self.scalar_static_f64[615]);
        self.scalar_static_f64[617]=(self.scalar_static_f64[615]*0.3333333333333);
        self.scalar_static_f64[618]=(1.0+self.scalar_static_f64[617]);
        self.scalar_static_f64[619]=(self.scalar_static_f64[616]*self.scalar_static_f64[618]);
        self.scalar_static_f64[620]=(1.0+self.scalar_static_f64[619]);
        self.scalar_static_f64[621]=(self.scalar_static_f64[615]*self.scalar_static_f64[620]);
        self.scalar_static_f64[622]=(1.0+self.scalar_static_f64[621]);
        self.scalar_static_f64[623]=(1.80485e-35/self.scalar_static_f64[622]);
        self.scalar_static_f64[624]=(if self.scalar_static_bool[61]{self.scalar_static_f64[623]}else{self.scalar_static_f64[613]});
        self.scalar_static_f64[625]=p.p246;
        self.scalar_static_f64[626]=(self.scalar_static_f64[602]/self.scalar_static_f64[625]);
        self.scalar_static_f64[627]=(if self.scalar_static_bool[33]{self.scalar_static_f64[626]}else{0.0});
        self.scalar_static_bool[62]=(self.scalar_static_f64[627]> -80.0);
        self.scalar_static_f64[628]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_bool[63]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[628]!=0.0));
        self.scalar_static_f64[629]=(self.scalar_static_f64[627]).exp();
        self.scalar_static_f64[630]=(if self.scalar_static_bool[63]{self.scalar_static_f64[629]}else{0.0});
        self.scalar_static_bool[64]=(!(self.scalar_static_f64[628]!=0.0));
        self.scalar_static_bool[65]=(self.scalar_static_bool[33]&&self.scalar_static_bool[64]);
        self.scalar_static_f64[631]=(-self.scalar_static_f64[627]);
        self.scalar_static_f64[632]=(self.scalar_static_f64[631]-80.0);
        self.scalar_static_f64[633]=(0.5*self.scalar_static_f64[632]);
        self.scalar_static_f64[634]=(0.3333333333333*self.scalar_static_f64[632]);
        self.scalar_static_f64[635]=(1.0+self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(self.scalar_static_f64[633]*self.scalar_static_f64[635]);
        self.scalar_static_f64[637]=(1.0+self.scalar_static_f64[636]);
        self.scalar_static_f64[638]=(self.scalar_static_f64[632]*self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=(1.0+self.scalar_static_f64[638]);
        self.scalar_static_f64[640]=(1.80485e-35/self.scalar_static_f64[639]);
        self.scalar_static_f64[641]=(if self.scalar_static_bool[65]{self.scalar_static_f64[640]}else{self.scalar_static_f64[630]});
        self.scalar_static_f64[642]=p.p241;
        self.scalar_static_f64[643]=p.p242;
        self.scalar_static_f64[644]=(self.scalar_static_f64[441]*self.scalar_static_f64[643]);
        self.scalar_static_f64[645]=(1.0+self.scalar_static_f64[644]);
        self.scalar_static_f64[646]=(self.scalar_static_f64[642]*self.scalar_static_f64[645]);
        self.scalar_static_f64[647]=(self.scalar_static_f64[624]-1.0);
        self.scalar_static_f64[648]=(self.scalar_static_f64[646]*self.scalar_static_f64[647]);
        self.scalar_static_f64[649]=(self.scalar_static_f64[648]/self.scalar_static_f64[610]);
        self.scalar_static_f64[650]=(1.0+self.scalar_static_f64[649]);
        self.scalar_static_f64[651]=p.p245;
        self.scalar_static_f64[652]=(self.scalar_static_f64[641]-1.0);
        self.scalar_static_f64[653]=(self.scalar_static_f64[651]*self.scalar_static_f64[652]);
        self.scalar_static_f64[654]=(self.scalar_static_f64[653]/self.scalar_static_f64[627]);
        self.scalar_static_f64[655]=(self.scalar_static_f64[650]+self.scalar_static_f64[654]);
        self.scalar_static_bool[66]=(self.scalar_static_f64[655]>1e-6);
        self.scalar_static_f64[656]=(if self.scalar_static_bool[66]{self.scalar_static_f64[655]}else{1e-6});
        self.scalar_static_f64[657]=(if self.scalar_static_bool[33]{self.scalar_static_f64[656]}else{0.0});
        self.scalar_static_f64[658]=p.p247;
        self.scalar_static_f64[659]=(self.scalar_static_f64[441]*self.scalar_static_f64[658]);
        self.scalar_static_f64[660]=(1.0+self.scalar_static_f64[659]);
        self.scalar_static_f64[661]=p.p248;
        self.scalar_static_f64[662]=(self.scalar_static_f64[441]*self.scalar_static_f64[661]);
        self.scalar_static_f64[663]=p.p249;
        self.scalar_static_f64[664]=(self.scalar_static_f64[429]/self.scalar_static_f64[663]);
        self.scalar_static_f64[665]=(1.0+self.scalar_static_f64[664]);
        self.scalar_static_f64[666]=(self.scalar_static_f64[665]).ln();
        self.scalar_static_f64[667]=(self.scalar_static_f64[662]*self.scalar_static_f64[666]);
        self.scalar_static_f64[668]=(self.scalar_static_f64[660]+self.scalar_static_f64[667]);
        self.scalar_static_bool[67]=(self.scalar_static_f64[668]>1e-6);
        self.scalar_static_f64[669]=(if self.scalar_static_bool[67]{self.scalar_static_f64[668]}else{1e-6});
        self.scalar_static_f64[670]=(if self.scalar_static_bool[33]{self.scalar_static_f64[669]}else{0.0});
        self.scalar_static_f64[671]=p.p240;
        self.scalar_static_f64[672]=(self.scalar_static_f64[671]/self.scalar_static_f64[657]);
        self.scalar_static_f64[673]=(self.scalar_static_f64[670]*self.scalar_static_f64[672]);
        self.scalar_static_f64[674]=(if self.scalar_static_bool[33]{self.scalar_static_f64[673]}else{0.0});
        self.scalar_static_f64[675]=(self.scalar_static_f64[429]*self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=(self.scalar_static_f64[675]/self.scalar_static_f64[423]);
        self.scalar_static_f64[677]=(if self.scalar_static_bool[33]{self.scalar_static_f64[676]}else{0.0});
        self.scalar_static_bool[68]=(self.scalar_static_f64[677]>1e-10);
        self.scalar_static_f64[678]=(if self.scalar_static_bool[68]{self.scalar_static_f64[677]}else{1e-10});
        self.scalar_static_f64[679]=(if self.scalar_static_bool[33]{self.scalar_static_f64[678]}else{self.scalar_static_f64[103]});
        self.scalar_static_f64[680]=p.p250;
        self.scalar_static_f64[681]=(self.scalar_static_f64[679]*self.scalar_static_f64[680]);
        self.scalar_static_f64[682]=(if self.scalar_static_bool[33]{self.scalar_static_f64[681]}else{self.scalar_static_f64[106]});
        self.scalar_static_f64[683]=p.p251;
        self.scalar_static_f64[684]=p.p252;
        self.scalar_static_f64[685]=(self.scalar_static_f64[439]*self.scalar_static_f64[684]);
        self.scalar_static_f64[686]=(1.0+self.scalar_static_f64[685]);
        self.scalar_static_f64[687]=(self.scalar_static_f64[683]*self.scalar_static_f64[686]);
        self.scalar_static_f64[688]=p.p253;
        self.scalar_static_f64[689]=(self.scalar_static_f64[441]*self.scalar_static_f64[688]);
        self.scalar_static_f64[690]=(1.0+self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[687]*self.scalar_static_f64[690]);
        self.scalar_static_f64[692]=p.p254;
        self.scalar_static_f64[693]=(self.scalar_static_f64[443]*self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(1.0+self.scalar_static_f64[693]);
        self.scalar_static_f64[695]=(self.scalar_static_f64[691]*self.scalar_static_f64[694]);
        self.scalar_static_f64[696]=(if self.scalar_static_bool[33]{self.scalar_static_f64[695]}else{self.scalar_static_f64[108]});
        self.scalar_static_f64[697]=p.p255;
        self.scalar_static_f64[698]=p.p256;
        self.scalar_static_f64[699]=p.p257;
        self.scalar_static_f64[700]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[699]);
        self.scalar_static_f64[701]=(self.scalar_static_f64[698]*self.scalar_static_f64[700]);
        self.scalar_static_f64[702]=(self.scalar_static_f64[697]+self.scalar_static_f64[701]);
        self.scalar_static_f64[703]=p.p258;
        self.scalar_static_f64[704]=(self.scalar_static_f64[441]*self.scalar_static_f64[703]);
        self.scalar_static_f64[705]=(1.0+self.scalar_static_f64[704]);
        self.scalar_static_f64[706]=(self.scalar_static_f64[702]*self.scalar_static_f64[705]);
        self.scalar_static_f64[707]=p.p259;
        self.scalar_static_f64[708]=(self.scalar_static_f64[443]*self.scalar_static_f64[707]);
        self.scalar_static_f64[709]=(1.0+self.scalar_static_f64[708]);
        self.scalar_static_f64[710]=(self.scalar_static_f64[706]*self.scalar_static_f64[709]);
        self.scalar_static_f64[711]=(if self.scalar_static_bool[33]{self.scalar_static_f64[710]}else{0.0});
        self.scalar_static_bool[69]=(self.scalar_static_f64[711]>0.0);
        self.scalar_static_f64[712]=(if self.scalar_static_bool[69]{self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[713]=(if self.scalar_static_bool[33]{self.scalar_static_f64[712]}else{self.scalar_static_f64[110]});
        self.scalar_static_f64[714]=p.p260;
        self.scalar_static_f64[715]=(if self.scalar_static_bool[33]{self.scalar_static_f64[714]}else{self.scalar_static_f64[112]});
        self.scalar_static_f64[716]=p.p261;
        self.scalar_static_f64[717]=(if self.scalar_static_bool[33]{self.scalar_static_f64[716]}else{self.scalar_static_f64[114]});
        self.scalar_static_f64[718]=p.p262;
        self.scalar_static_f64[719]=p.p263;
        self.scalar_static_f64[720]=(self.scalar_static_f64[439]*self.scalar_static_f64[719]);
        self.scalar_static_f64[721]=(1.0+self.scalar_static_f64[720]);
        self.scalar_static_f64[722]=(self.scalar_static_f64[718]*self.scalar_static_f64[721]);
        self.scalar_static_f64[723]=p.p264;
        self.scalar_static_f64[724]=(self.scalar_static_f64[441]*self.scalar_static_f64[723]);
        self.scalar_static_f64[725]=(1.0+self.scalar_static_f64[724]);
        self.scalar_static_f64[726]=(self.scalar_static_f64[722]*self.scalar_static_f64[725]);
        self.scalar_static_f64[727]=p.p265;
        self.scalar_static_f64[728]=(self.scalar_static_f64[443]*self.scalar_static_f64[727]);
        self.scalar_static_f64[729]=(1.0+self.scalar_static_f64[728]);
        self.scalar_static_f64[730]=(self.scalar_static_f64[726]*self.scalar_static_f64[729]);
        self.scalar_static_f64[731]=(if self.scalar_static_bool[33]{self.scalar_static_f64[730]}else{self.scalar_static_f64[116]});
        self.scalar_static_f64[732]=p.p266;
        self.scalar_static_f64[733]=(if self.scalar_static_bool[33]{self.scalar_static_f64[732]}else{self.scalar_static_f64[118]});
        self.scalar_static_f64[734]=p.p267;
        self.scalar_static_f64[735]=(if self.scalar_static_bool[33]{self.scalar_static_f64[734]}else{self.scalar_static_f64[120]});
        self.scalar_static_f64[736]=p.p268;
        self.scalar_static_f64[737]=(if self.scalar_static_bool[33]{self.scalar_static_f64[736]}else{self.scalar_static_f64[122]});
        self.scalar_static_f64[738]=p.p269;
        self.scalar_static_f64[739]=(if self.scalar_static_bool[33]{self.scalar_static_f64[738]}else{self.scalar_static_f64[124]});
        self.scalar_static_f64[740]=p.p270;
        self.scalar_static_f64[741]=(if self.scalar_static_bool[33]{self.scalar_static_f64[740]}else{self.scalar_static_f64[126]});
        self.scalar_static_f64[742]=p.p271;
        self.scalar_static_f64[743]=(if self.scalar_static_bool[33]{self.scalar_static_f64[742]}else{self.scalar_static_f64[128]});
        self.scalar_static_f64[744]=p.p272;
        self.scalar_static_f64[745]=(if self.scalar_static_bool[33]{self.scalar_static_f64[744]}else{self.scalar_static_f64[130]});
        self.scalar_static_f64[746]=p.p273;
        self.scalar_static_f64[747]=(if self.scalar_static_bool[33]{self.scalar_static_f64[746]}else{self.scalar_static_f64[132]});
        self.scalar_static_f64[748]=p.p274;
        self.scalar_static_f64[749]=p.p275;
        self.scalar_static_f64[750]=p.p276;
        self.scalar_static_f64[751]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[750]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[749]*self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(self.scalar_static_f64[748]+self.scalar_static_f64[752]);
        self.scalar_static_f64[754]=p.p277;
        self.scalar_static_f64[755]=(self.scalar_static_f64[441]*self.scalar_static_f64[754]);
        self.scalar_static_f64[756]=(1.0+self.scalar_static_f64[755]);
        self.scalar_static_f64[757]=(self.scalar_static_f64[753]*self.scalar_static_f64[756]);
        self.scalar_static_f64[758]=p.p278;
        self.scalar_static_f64[759]=(self.scalar_static_f64[443]*self.scalar_static_f64[758]);
        self.scalar_static_f64[760]=(1.0+self.scalar_static_f64[759]);
        self.scalar_static_f64[761]=(self.scalar_static_f64[757]*self.scalar_static_f64[760]);
        self.scalar_static_f64[762]=(if self.scalar_static_bool[33]{self.scalar_static_f64[761]}else{self.scalar_static_f64[134]});
        self.scalar_static_f64[763]=p.p279;
        self.scalar_static_f64[764]=(if self.scalar_static_bool[33]{self.scalar_static_f64[763]}else{self.scalar_static_f64[136]});
        self.scalar_static_f64[765]=p.p280;
        self.scalar_static_f64[766]=(if self.scalar_static_bool[33]{self.scalar_static_f64[765]}else{self.scalar_static_f64[138]});
        self.scalar_static_f64[767]=p.p281;
        self.scalar_static_f64[768]=(if self.scalar_static_bool[33]{self.scalar_static_f64[767]}else{self.scalar_static_f64[140]});
        self.scalar_static_f64[769]=p.p282;
        self.scalar_static_f64[770]=(self.scalar_static_f64[441]*self.scalar_static_f64[769]);
        self.scalar_static_f64[771]=p.p283;
        self.scalar_static_f64[772]=(self.scalar_static_f64[441]*self.scalar_static_f64[771]);
        self.scalar_static_f64[773]=(1.0+self.scalar_static_f64[772]);
        self.scalar_static_f64[774]=(self.scalar_static_f64[770]*self.scalar_static_f64[773]);
        self.scalar_static_f64[775]=(if self.scalar_static_bool[33]{self.scalar_static_f64[774]}else{0.0});
        self.scalar_static_bool[70]=(self.scalar_static_f64[775]>0.0);
        self.scalar_static_f64[776]=(if self.scalar_static_bool[70]{self.scalar_static_f64[775]}else{0.0});
        self.scalar_static_f64[777]=(if self.scalar_static_bool[33]{self.scalar_static_f64[776]}else{self.scalar_static_f64[142]});
        self.scalar_static_f64[778]=p.p284;
        self.scalar_static_f64[779]=(if self.scalar_static_bool[33]{self.scalar_static_f64[778]}else{self.scalar_static_f64[144]});
        self.scalar_static_f64[780]=p.p285;
        self.scalar_static_f64[781]=(if self.scalar_static_bool[33]{self.scalar_static_f64[780]}else{self.scalar_static_f64[146]});
        self.scalar_static_f64[782]=p.p286;
        self.scalar_static_f64[783]=(if self.scalar_static_bool[33]{self.scalar_static_f64[782]}else{self.scalar_static_f64[148]});
        self.scalar_static_f64[784]=p.p287;
        self.scalar_static_f64[785]=(if self.scalar_static_bool[33]{self.scalar_static_f64[784]}else{self.scalar_static_f64[150]});
        self.scalar_static_f64[786]=p.p288;
        self.scalar_static_f64[787]=(if self.scalar_static_bool[33]{self.scalar_static_f64[786]}else{self.scalar_static_f64[152]});
        self.scalar_static_f64[788]=p.p289;
        self.scalar_static_f64[789]=p.p290;
        self.scalar_static_f64[790]=p.p291;
        self.scalar_static_f64[791]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[790]);
        self.scalar_static_f64[792]=(self.scalar_static_f64[789]*self.scalar_static_f64[791]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[788]+self.scalar_static_f64[792]);
        self.scalar_static_f64[794]=(self.scalar_static_f64[674]*self.scalar_static_f64[793]);
        self.scalar_static_f64[795]=p.p292;
        self.scalar_static_f64[796]=(self.scalar_static_f64[441]*self.scalar_static_f64[795]);
        self.scalar_static_f64[797]=(1.0+self.scalar_static_f64[796]);
        self.scalar_static_f64[798]=(self.scalar_static_f64[794]*self.scalar_static_f64[797]);
        self.scalar_static_f64[799]=p.p293;
        self.scalar_static_f64[800]=(self.scalar_static_f64[443]*self.scalar_static_f64[799]);
        self.scalar_static_f64[801]=(1.0+self.scalar_static_f64[800]);
        self.scalar_static_f64[802]=(self.scalar_static_f64[798]*self.scalar_static_f64[801]);
        self.scalar_static_f64[803]=(if self.scalar_static_bool[33]{self.scalar_static_f64[802]}else{0.0});
        self.scalar_static_bool[71]=(self.scalar_static_f64[803]>0.0);
        self.scalar_static_f64[804]=(if self.scalar_static_bool[71]{self.scalar_static_f64[803]}else{0.0});
        self.scalar_static_f64[805]=(if self.scalar_static_bool[33]{self.scalar_static_f64[804]}else{self.scalar_static_f64[154]});
        self.scalar_static_f64[806]=p.p294;
        self.scalar_static_f64[807]=p.p295;
        self.scalar_static_f64[808]=(self.scalar_static_f64[439]*self.scalar_static_f64[807]);
        self.scalar_static_f64[809]=(1.0+self.scalar_static_f64[808]);
        self.scalar_static_f64[810]=(self.scalar_static_f64[806]*self.scalar_static_f64[809]);
        self.scalar_static_f64[811]=p.p296;
        self.scalar_static_f64[812]=(self.scalar_static_f64[441]*self.scalar_static_f64[811]);
        self.scalar_static_f64[813]=(1.0+self.scalar_static_f64[812]);
        self.scalar_static_f64[814]=(self.scalar_static_f64[810]*self.scalar_static_f64[813]);
        self.scalar_static_f64[815]=p.p297;
        self.scalar_static_f64[816]=(self.scalar_static_f64[443]*self.scalar_static_f64[815]);
        self.scalar_static_f64[817]=(1.0+self.scalar_static_f64[816]);
        self.scalar_static_f64[818]=(self.scalar_static_f64[814]*self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(if self.scalar_static_bool[33]{self.scalar_static_f64[818]}else{self.scalar_static_f64[156]});
        self.scalar_static_f64[820]=p.p298;
        self.scalar_static_f64[821]=(if self.scalar_static_bool[33]{self.scalar_static_f64[820]}else{self.scalar_static_f64[158]});
        self.scalar_static_f64[822]=p.p299;
        self.scalar_static_f64[823]=(if self.scalar_static_bool[33]{self.scalar_static_f64[822]}else{self.scalar_static_f64[160]});
        self.scalar_static_f64[824]=p.p300;
        self.scalar_static_f64[825]=p.p301;
        self.scalar_static_f64[826]=p.p302;
        self.scalar_static_f64[827]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[826]);
        self.scalar_static_f64[828]=(self.scalar_static_f64[825]*self.scalar_static_f64[827]);
        self.scalar_static_f64[829]=p.p303;
        self.scalar_static_f64[830]=p.p304;
        self.scalar_static_f64[831]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[830]);
        self.scalar_static_f64[832]=(self.scalar_static_f64[829]*self.scalar_static_f64[831]);
        self.scalar_static_f64[833]=(1.0+self.scalar_static_f64[832]);
        self.scalar_static_f64[834]=(self.scalar_static_f64[828]/self.scalar_static_f64[833]);
        self.scalar_static_f64[835]=(1.0+self.scalar_static_f64[834]);
        self.scalar_static_f64[836]=(self.scalar_static_f64[824]/self.scalar_static_f64[835]);
        self.scalar_static_f64[837]=(if self.scalar_static_bool[33]{self.scalar_static_f64[836]}else{0.0});
        self.scalar_static_bool[72]=(self.scalar_static_f64[837]>1.0);
        self.scalar_static_f64[838]=(if self.scalar_static_bool[72]{self.scalar_static_f64[837]}else{1.0});
        self.scalar_static_bool[73]=(self.scalar_static_f64[838]<16.0);
        self.scalar_static_f64[839]=(if self.scalar_static_bool[73]{self.scalar_static_f64[838]}else{16.0});
        self.scalar_static_f64[840]=(if self.scalar_static_bool[33]{self.scalar_static_f64[839]}else{self.scalar_static_f64[162]});
        self.scalar_static_f64[841]=p.p305;
        self.scalar_static_f64[842]=p.p306;
        self.scalar_static_f64[843]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[842]);
        self.scalar_static_f64[844]=(self.scalar_static_f64[841]*self.scalar_static_f64[843]);
        self.scalar_static_f64[845]=p.p309;
        self.scalar_static_f64[846]=(self.scalar_static_f64[441]*self.scalar_static_f64[845]);
        self.scalar_static_f64[847]=(1.0+self.scalar_static_f64[846]);
        self.scalar_static_f64[848]=(self.scalar_static_f64[844]*self.scalar_static_f64[847]);
        self.scalar_static_f64[849]=p.p307;
        self.scalar_static_f64[850]=p.p308;
        self.scalar_static_f64[851]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[850]);
        self.scalar_static_f64[852]=(self.scalar_static_f64[849]*self.scalar_static_f64[851]);
        self.scalar_static_f64[853]=(1.0+self.scalar_static_f64[852]);
        self.scalar_static_f64[854]=(self.scalar_static_f64[848]/self.scalar_static_f64[853]);
        self.scalar_static_f64[855]=(if self.scalar_static_bool[33]{self.scalar_static_f64[854]}else{0.0});
        self.scalar_static_bool[74]=(self.scalar_static_f64[855]>0.0);
        self.scalar_static_f64[856]=(if self.scalar_static_bool[74]{self.scalar_static_f64[855]}else{0.0});
        self.scalar_static_f64[857]=(if self.scalar_static_bool[33]{self.scalar_static_f64[856]}else{self.scalar_static_f64[164]});
        self.scalar_static_f64[858]=p.p310;
        self.scalar_static_f64[859]=p.p311;
        self.scalar_static_f64[860]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[859]);
        self.scalar_static_f64[861]=(self.scalar_static_f64[858]*self.scalar_static_f64[860]);
        self.scalar_static_f64[862]=p.p314;
        self.scalar_static_f64[863]=(self.scalar_static_f64[441]*self.scalar_static_f64[862]);
        self.scalar_static_f64[864]=(1.0+self.scalar_static_f64[863]);
        self.scalar_static_f64[865]=(self.scalar_static_f64[861]*self.scalar_static_f64[864]);
        self.scalar_static_f64[866]=p.p312;
        self.scalar_static_f64[867]=p.p313;
        self.scalar_static_f64[868]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[867]);
        self.scalar_static_f64[869]=(self.scalar_static_f64[866]*self.scalar_static_f64[868]);
        self.scalar_static_f64[870]=(1.0+self.scalar_static_f64[869]);
        self.scalar_static_f64[871]=(self.scalar_static_f64[865]/self.scalar_static_f64[870]);
        self.scalar_static_f64[872]=(if self.scalar_static_bool[33]{self.scalar_static_f64[871]}else{0.0});
        self.scalar_static_bool[75]=(self.scalar_static_f64[872]>0.0);
        self.scalar_static_f64[873]=(if self.scalar_static_bool[75]{self.scalar_static_f64[872]}else{0.0});
        self.scalar_static_f64[874]=(if self.scalar_static_bool[33]{self.scalar_static_f64[873]}else{self.scalar_static_f64[166]});
        self.scalar_static_f64[875]=p.p315;
        self.scalar_static_f64[876]=(if self.scalar_static_bool[33]{self.scalar_static_f64[875]}else{self.scalar_static_f64[168]});
        self.scalar_static_f64[877]=p.p316;
        self.scalar_static_f64[878]=(if self.scalar_static_bool[33]{self.scalar_static_f64[877]}else{self.scalar_static_f64[170]});
        self.scalar_static_f64[879]=p.p317;
        self.scalar_static_f64[880]=(if self.scalar_static_bool[33]{self.scalar_static_f64[879]}else{self.scalar_static_f64[172]});
        self.scalar_static_f64[881]=p.p318;
        self.scalar_static_f64[882]=(if self.scalar_static_bool[33]{self.scalar_static_f64[881]}else{self.scalar_static_f64[174]});
        self.scalar_static_f64[883]=p.p319;
        self.scalar_static_f64[884]=(self.scalar_static_f64[883]/self.scalar_static_f64[443]);
        self.scalar_static_f64[885]=(if self.scalar_static_bool[33]{self.scalar_static_f64[884]}else{self.scalar_static_f64[176]});
        self.scalar_static_f64[886]=p.p320;
        self.scalar_static_f64[887]=(self.scalar_static_f64[886]/self.scalar_static_f64[441]);
        self.scalar_static_f64[888]=(if self.scalar_static_bool[33]{self.scalar_static_f64[887]}else{self.scalar_static_f64[178]});
        self.scalar_static_f64[889]=p.p321;
        self.scalar_static_f64[890]=(self.scalar_static_f64[889]/self.scalar_static_f64[441]);
        self.scalar_static_f64[891]=(if self.scalar_static_bool[33]{self.scalar_static_f64[890]}else{self.scalar_static_f64[180]});
        self.scalar_static_f64[892]=p.p335;
        self.scalar_static_f64[893]=(self.scalar_static_f64[892]/self.scalar_static_f64[441]);
        self.scalar_static_f64[894]=(if self.scalar_static_bool[33]{self.scalar_static_f64[893]}else{self.scalar_static_f64[182]});
        self.scalar_static_f64[895]=p.p336;
        self.scalar_static_f64[896]=(self.scalar_static_f64[895]/self.scalar_static_f64[441]);
        self.scalar_static_f64[897]=(if self.scalar_static_bool[33]{self.scalar_static_f64[896]}else{self.scalar_static_f64[184]});
        self.scalar_static_f64[898]=p.p322;
        self.scalar_static_f64[899]=(self.scalar_static_f64[898]/self.scalar_static_f64[441]);
        self.scalar_static_f64[900]=(if self.scalar_static_bool[33]{self.scalar_static_f64[899]}else{self.scalar_static_f64[186]});
        self.scalar_static_f64[901]=p.p323;
        self.scalar_static_f64[902]=(self.scalar_static_f64[901]/self.scalar_static_f64[441]);
        self.scalar_static_f64[903]=(if self.scalar_static_bool[33]{self.scalar_static_f64[902]}else{self.scalar_static_f64[188]});
        self.scalar_static_f64[904]=p.p324;
        self.scalar_static_f64[905]=(if self.scalar_static_bool[33]{self.scalar_static_f64[904]}else{self.scalar_static_f64[190]});
        self.scalar_static_f64[906]=p.p338;
        self.scalar_static_f64[907]=(if self.scalar_static_bool[33]{self.scalar_static_f64[906]}else{self.scalar_static_f64[192]});
        self.scalar_static_f64[908]=p.p325;
        self.scalar_static_f64[909]=(if self.scalar_static_bool[33]{self.scalar_static_f64[908]}else{self.scalar_static_f64[194]});
        self.scalar_static_f64[910]=p.p326;
        self.scalar_static_f64[911]=(if self.scalar_static_bool[33]{self.scalar_static_f64[910]}else{self.scalar_static_f64[196]});
        self.scalar_static_f64[912]=p.p327;
        self.scalar_static_f64[913]=(if self.scalar_static_bool[33]{self.scalar_static_f64[912]}else{self.scalar_static_f64[198]});
        self.scalar_static_f64[914]=p.p337;
        self.scalar_static_f64[915]=(if self.scalar_static_bool[33]{self.scalar_static_f64[914]}else{self.scalar_static_f64[200]});
        self.scalar_static_f64[916]=p.p328;
        self.scalar_static_f64[917]=(if self.scalar_static_bool[33]{self.scalar_static_f64[916]}else{self.scalar_static_f64[202]});
        self.scalar_static_f64[918]=p.p329;
        self.scalar_static_f64[919]=(if self.scalar_static_bool[33]{self.scalar_static_f64[918]}else{self.scalar_static_f64[204]});
        self.scalar_static_f64[920]=p.p330;
        self.scalar_static_f64[921]=(if self.scalar_static_bool[33]{self.scalar_static_f64[920]}else{self.scalar_static_f64[206]});
        self.scalar_static_f64[922]=p.p331;
        self.scalar_static_f64[923]=(self.scalar_static_f64[439]*self.scalar_static_f64[922]);
        self.scalar_static_f64[924]=(if self.scalar_static_bool[33]{self.scalar_static_f64[923]}else{self.scalar_static_f64[208]});
        self.scalar_static_f64[925]=p.p332;
        self.scalar_static_f64[926]=(if self.scalar_static_bool[33]{self.scalar_static_f64[925]}else{self.scalar_static_f64[210]});
        self.scalar_static_f64[927]=p.p333;
        self.scalar_static_f64[928]=(if self.scalar_static_bool[33]{self.scalar_static_f64[927]}else{self.scalar_static_f64[212]});
        self.scalar_static_f64[929]=p.p334;
        self.scalar_static_f64[930]=(if self.scalar_static_bool[33]{self.scalar_static_f64[929]}else{self.scalar_static_f64[214]});
        self.scalar_static_f64[931]=p.p339;
        self.scalar_static_f64[932]=p.p341;
        self.scalar_static_f64[933]=(self.scalar_static_f64[932]/self.scalar_static_f64[441]);
        self.scalar_static_f64[934]=(self.scalar_static_f64[931]+self.scalar_static_f64[933]);
        self.scalar_static_f64[935]=(if self.scalar_static_bool[33]{self.scalar_static_f64[934]}else{0.0});
        self.scalar_static_bool[76]=(self.scalar_static_f64[935]>0.0);
        self.scalar_static_f64[936]=(if self.scalar_static_bool[76]{self.scalar_static_f64[935]}else{0.0});
        self.scalar_static_f64[937]=(if self.scalar_static_bool[33]{self.scalar_static_f64[936]}else{self.scalar_static_f64[216]});
        self.scalar_static_f64[938]=p.p340;
        self.scalar_static_f64[939]=p.p342;
        self.scalar_static_f64[940]=(self.scalar_static_f64[939]/self.scalar_static_f64[441]);
        self.scalar_static_f64[941]=(self.scalar_static_f64[938]+self.scalar_static_f64[940]);
        self.scalar_static_f64[942]=(if self.scalar_static_bool[33]{self.scalar_static_f64[941]}else{0.0});
        self.scalar_static_bool[77]=(self.scalar_static_f64[942]>0.0);
        self.scalar_static_f64[943]=(if self.scalar_static_bool[77]{self.scalar_static_f64[942]}else{0.0});
        self.scalar_static_f64[944]=(if self.scalar_static_bool[33]{self.scalar_static_f64[943]}else{self.scalar_static_f64[218]});
        self.scalar_static_f64[945]=p.p343;
        self.scalar_static_f64[946]=(if self.scalar_static_bool[33]{self.scalar_static_f64[945]}else{self.scalar_static_f64[220]});
        self.scalar_static_f64[947]=p.p344;
        self.scalar_static_f64[948]=(if self.scalar_static_bool[33]{self.scalar_static_f64[947]}else{self.scalar_static_f64[222]});
        self.scalar_static_f64[949]=p.p345;
        self.scalar_static_f64[950]=(if self.scalar_static_bool[33]{self.scalar_static_f64[949]}else{self.scalar_static_f64[224]});
        self.scalar_static_f64[951]=p.p346;
        self.scalar_static_f64[952]=(if self.scalar_static_bool[33]{self.scalar_static_f64[951]}else{self.scalar_static_f64[226]});
        self.scalar_static_f64[953]=p.p347;
        self.scalar_static_f64[954]=(if self.scalar_static_bool[33]{self.scalar_static_f64[953]}else{self.scalar_static_f64[228]});
        self.scalar_static_f64[955]=p.p348;
        self.scalar_static_f64[956]=(if self.scalar_static_bool[33]{self.scalar_static_f64[955]}else{self.scalar_static_f64[230]});
        self.scalar_static_f64[957]=p.p349;
        self.scalar_static_f64[958]=p.p351;
        self.scalar_static_f64[959]=(self.scalar_static_f64[439]*self.scalar_static_f64[958]);
        self.scalar_static_f64[960]=(self.scalar_static_f64[957]+self.scalar_static_f64[959]);
        self.scalar_static_f64[961]=(if self.scalar_static_bool[33]{self.scalar_static_f64[960]}else{self.scalar_static_f64[232]});
        self.scalar_static_f64[962]=p.p350;
        self.scalar_static_f64[963]=p.p352;
        self.scalar_static_f64[964]=(self.scalar_static_f64[439]*self.scalar_static_f64[963]);
        self.scalar_static_f64[965]=(self.scalar_static_f64[962]+self.scalar_static_f64[964]);
        self.scalar_static_f64[966]=(if self.scalar_static_bool[33]{self.scalar_static_f64[965]}else{self.scalar_static_f64[234]});
        self.scalar_static_f64[967]=p.p384;
        self.scalar_static_f64[968]=p.p385;
        self.scalar_static_f64[969]=(self.scalar_static_f64[439]*self.scalar_static_f64[968]);
        self.scalar_static_f64[970]=(1.0+self.scalar_static_f64[969]);
        self.scalar_static_f64[971]=(self.scalar_static_f64[967]*self.scalar_static_f64[970]);
        self.scalar_static_f64[972]=p.p386;
        self.scalar_static_f64[973]=(self.scalar_static_f64[441]*self.scalar_static_f64[972]);
        self.scalar_static_f64[974]=(1.0+self.scalar_static_f64[973]);
        self.scalar_static_f64[975]=(self.scalar_static_f64[971]*self.scalar_static_f64[974]);
        self.scalar_static_f64[976]=(if self.scalar_static_bool[33]{self.scalar_static_f64[975]}else{0.0});
        self.scalar_static_bool[78]=(self.scalar_static_f64[976]>0.0);
        self.scalar_static_f64[977]=(if self.scalar_static_bool[78]{self.scalar_static_f64[976]}else{0.0});
        self.scalar_static_f64[978]=(if self.scalar_static_bool[33]{self.scalar_static_f64[977]}else{self.scalar_static_f64[236]});
        self.scalar_static_f64[979]=p.p387;
        self.scalar_static_f64[980]=(if self.scalar_static_bool[33]{self.scalar_static_f64[979]}else{self.scalar_static_f64[238]});
        self.scalar_static_f64[981]=p.p388;
        self.scalar_static_f64[982]=(if self.scalar_static_bool[33]{self.scalar_static_f64[981]}else{self.scalar_static_f64[240]});
        self.scalar_static_f64[983]=p.p389;
        self.scalar_static_f64[984]=p.p390;
        self.scalar_static_f64[985]=(self.scalar_static_f64[439]*self.scalar_static_f64[984]);
        self.scalar_static_f64[986]=(1.0+self.scalar_static_f64[985]);
        self.scalar_static_f64[987]=(self.scalar_static_f64[983]*self.scalar_static_f64[986]);
        self.scalar_static_f64[988]=p.p391;
        self.scalar_static_f64[989]=(self.scalar_static_f64[441]*self.scalar_static_f64[988]);
        self.scalar_static_f64[990]=(1.0+self.scalar_static_f64[989]);
        self.scalar_static_f64[991]=(self.scalar_static_f64[987]*self.scalar_static_f64[990]);
        self.scalar_static_f64[992]=(if self.scalar_static_bool[33]{self.scalar_static_f64[991]}else{0.0});
        self.scalar_static_bool[79]=(self.scalar_static_f64[992]>0.0);
        self.scalar_static_f64[993]=(if self.scalar_static_bool[79]{self.scalar_static_f64[992]}else{0.0});
        self.scalar_static_f64[994]=(if self.scalar_static_bool[33]{self.scalar_static_f64[993]}else{self.scalar_static_f64[242]});
        self.scalar_static_f64[995]=p.p353;
        self.scalar_static_f64[996]=(2.0*self.scalar_static_f64[995]);
        self.scalar_static_f64[997]=p.p354;
        self.scalar_static_f64[998]=(self.scalar_static_f64[429]*self.scalar_static_f64[997]);
        self.scalar_static_f64[999]=(self.scalar_static_f64[996]+self.scalar_static_f64[998]);
        self.scalar_static_f64[1000]=(if self.scalar_static_bool[33]{self.scalar_static_f64[999]}else{0.0});
        self.scalar_static_f64[1001]=p.p355;
        self.scalar_static_f64[1002]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1001]}else{self.scalar_static_f64[244]});
        self.scalar_static_f64[1003]=p.p357;
        self.scalar_static_f64[1004]=p.p358;
        self.scalar_static_f64[1005]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1004]);
        self.scalar_static_f64[1006]=(self.scalar_static_f64[1003]*self.scalar_static_f64[1005]);
        self.scalar_static_f64[1007]=p.p356;
        self.scalar_static_f64[1008]=p.p359;
        self.scalar_static_f64[1009]=(self.scalar_static_f64[441]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1010]=p.p360;
        self.scalar_static_f64[1011]=(self.scalar_static_f64[443]*self.scalar_static_f64[1010]);
        self.scalar_static_f64[1012]=p.p361;
        self.scalar_static_f64[1013]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1012]}else{self.scalar_static_f64[248]});
        self.scalar_static_f64[1014]=p.p362;
        self.scalar_static_f64[1015]=p.p363;
        self.scalar_static_f64[1016]=(self.scalar_static_f64[439]*self.scalar_static_f64[1015]);
        self.scalar_static_f64[1017]=(1.0+self.scalar_static_f64[1016]);
        self.scalar_static_f64[1018]=(self.scalar_static_f64[1014]*self.scalar_static_f64[1017]);
        self.scalar_static_f64[1019]=p.p364;
        self.scalar_static_f64[1020]=(self.scalar_static_f64[441]*self.scalar_static_f64[1019]);
        self.scalar_static_f64[1021]=(1.0+self.scalar_static_f64[1020]);
        self.scalar_static_f64[1022]=(self.scalar_static_f64[1018]*self.scalar_static_f64[1021]);
        self.scalar_static_f64[1023]=p.p365;
        self.scalar_static_f64[1024]=(self.scalar_static_f64[443]*self.scalar_static_f64[1023]);
        self.scalar_static_f64[1025]=(1.0+self.scalar_static_f64[1024]);
        self.scalar_static_f64[1026]=(self.scalar_static_f64[1022]*self.scalar_static_f64[1025]);
        self.scalar_static_f64[1027]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1026]}else{self.scalar_static_f64[250]});
        self.scalar_static_f64[1028]=p.p366;
        self.scalar_static_f64[1029]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1028]}else{self.scalar_static_f64[252]});
        self.scalar_static_f64[1030]=p.p367;
        self.scalar_static_f64[1031]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1030]}else{self.scalar_static_f64[254]});
        self.scalar_static_f64[1032]=p.p368;
        self.scalar_static_f64[1033]=(2.0*self.scalar_static_f64[1032]);
        self.scalar_static_f64[1034]=p.p369;
        self.scalar_static_f64[1035]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[1034]);
        self.scalar_static_f64[1036]=(self.scalar_static_f64[1033]*self.scalar_static_f64[1035]);
        self.scalar_static_f64[1037]=p.p370;
        self.scalar_static_f64[1038]=(self.scalar_static_f64[441]*self.scalar_static_f64[1037]);
        self.scalar_static_f64[1039]=(1.0+self.scalar_static_f64[1038]);
        self.scalar_static_f64[1040]=(self.scalar_static_f64[1036]*self.scalar_static_f64[1039]);
        self.scalar_static_f64[1041]=p.p371;
        self.scalar_static_f64[1042]=p.p373;
        self.scalar_static_f64[1043]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[1042]);
        self.scalar_static_f64[1044]=p.p374;
        self.scalar_static_f64[1045]=(self.scalar_static_f64[441]*self.scalar_static_f64[1044]);
        self.scalar_static_f64[1046]=(1.0+self.scalar_static_f64[1045]);
        self.scalar_static_f64[1047]=(self.scalar_static_f64[1043]*self.scalar_static_f64[1046]);
        self.scalar_static_f64[1048]=p.p372;
        self.scalar_static_f64[1049]=p.p375;
        self.scalar_static_f64[1050]=p.p376;
        self.scalar_static_f64[1051]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1050]}else{self.scalar_static_f64[270]});
        self.scalar_static_f64[1052]=p.p377;
        self.scalar_static_f64[1053]=p.p378;
        self.scalar_static_f64[1054]=(self.scalar_static_f64[1052]*self.scalar_static_f64[1053]);
        self.scalar_static_f64[1055]=(self.scalar_static_f64[1054]/self.scalar_static_f64[423]);
        self.scalar_static_f64[1056]=(self.scalar_static_f64[602]/self.scalar_static_f64[1053]);
        self.scalar_static_f64[1057]=(self.scalar_static_f64[1056]).exp();
        self.scalar_static_f64[1058]=(1.0-self.scalar_static_f64[1057]);
        self.scalar_static_f64[1059]=(self.scalar_static_f64[1055]*self.scalar_static_f64[1058]);
        self.scalar_static_f64[1060]=(1.0+self.scalar_static_f64[1059]);
        self.scalar_static_f64[1061]=(self.scalar_static_f64[671]*self.scalar_static_f64[1000]);
        self.scalar_static_f64[1062]=p.p379;
        self.scalar_static_f64[1063]=(self.scalar_static_f64[441]*self.scalar_static_f64[1062]);
        self.scalar_static_f64[1064]=(1.0+self.scalar_static_f64[1063]);
        self.scalar_static_f64[1065]=p.p380;
        self.scalar_static_f64[1066]=p.p381;
        self.scalar_static_f64[1067]=(self.scalar_static_f64[439]*self.scalar_static_f64[1066]);
        self.scalar_static_f64[1068]=(self.scalar_static_f64[1065]+self.scalar_static_f64[1067]);
        self.scalar_static_f64[1069]=p.p382;
        self.scalar_static_f64[1070]=(self.scalar_static_f64[441]*self.scalar_static_f64[1069]);
        self.scalar_static_f64[1071]=(self.scalar_static_f64[1068]+self.scalar_static_f64[1070]);
        self.scalar_static_f64[1072]=p.p383;
        self.scalar_static_f64[1073]=(self.scalar_static_f64[439]*self.scalar_static_f64[1072]);
        self.scalar_static_f64[1074]=(self.scalar_static_f64[441]*self.scalar_static_f64[1073]);
        self.scalar_static_f64[1075]=(self.scalar_static_f64[1071]+self.scalar_static_f64[1074]);
        self.scalar_static_f64[1076]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1075]}else{self.scalar_static_f64[274]});
        self.scalar_static_f64[1077]=(self.scalar_static_f64[433]*self.scalar_static_f64[437]);
        self.scalar_static_f64[1078]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1077]}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[1079]=p.p392;
        self.scalar_static_f64[1080]=p.p393;
        self.scalar_static_f64[1081]=p.p394;
        self.scalar_static_f64[1082]=(1000000.0*self.scalar_static_f64[1081]);
        self.scalar_static_f64[1083]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1082]}else{self.scalar_static_f64[281]});
        self.scalar_static_f64[1084]=p.p395;
        self.scalar_static_f64[1085]=(self.scalar_static_f64[437]*self.scalar_static_f64[1084]);
        self.scalar_static_f64[1086]=(self.scalar_static_f64[1085]/self.scalar_static_f64[392]);
        self.scalar_static_f64[1087]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1086]}else{self.scalar_static_f64[283]});
        self.scalar_static_f64[1088]=p.p396;
        self.scalar_static_f64[1089]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1088]}else{self.scalar_static_f64[285]});
        self.scalar_static_f64[1090]=(if self.scalar_static_bool[33]{self.scalar_static_f64[564]}else{self.scalar_static_f64[311]});
        self.scalar_static_f64[1091]=(if self.scalar_static_bool[33]{self.scalar_static_f64[569]}else{self.scalar_static_f64[315]});
        self.scalar_static_f64[1092]=(if self.scalar_static_bool[33]{self.scalar_static_f64[803]}else{0.0});
        self.scalar_static_f64[1093]=(if self.scalar_static_bool[33]{self.scalar_static_f64[805]}else{self.scalar_static_f64[329]});
        self.scalar_static_f64[1094]=(if self.scalar_static_bool[33]{self.scalar_static_f64[840]}else{self.scalar_static_f64[334]});
        self.scalar_static_f64[1095]=(if self.scalar_static_bool[33]{self.scalar_static_f64[857]}else{self.scalar_static_f64[339]});
        self.scalar_static_bool[80]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[33]);
        self.scalar_static_f64[1096]=(if self.scalar_static_bool[80]{self.scalar_static_f64[502]}else{0.0});
        self.scalar_static_f64[1097]=if param_given[397]{1.0}else{0.0};
        self.scalar_static_bool[81]=(1.0==self.scalar_static_f64[1097]);
        self.scalar_static_f64[1098]=(if self.scalar_static_bool[81]{1.0}else{0.0});
        self.scalar_static_bool[82]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1098]!=0.0));
        self.scalar_static_f64[1099]=p.p397;
        self.scalar_static_f64[1100]=(if self.scalar_static_bool[82]{self.scalar_static_f64[1099]}else{self.scalar_static_f64[1096]});
        self.scalar_static_f64[1101]=(if self.scalar_static_bool[80]{self.scalar_static_f64[492]}else{0.0});
        self.scalar_static_f64[1102]=if param_given[398]{1.0}else{0.0};
        self.scalar_static_bool[83]=(1.0==self.scalar_static_f64[1102]);
        self.scalar_static_f64[1103]=(if self.scalar_static_bool[83]{1.0}else{0.0});
        self.scalar_static_bool[84]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1103]!=0.0));
        self.scalar_static_f64[1104]=p.p398;
        self.scalar_static_f64[1105]=(if self.scalar_static_bool[84]{self.scalar_static_f64[1104]}else{self.scalar_static_f64[1101]});
        self.scalar_static_f64[1106]=(if self.scalar_static_bool[80]{self.scalar_static_f64[493]}else{0.0});
        self.scalar_static_f64[1107]=if param_given[399]{1.0}else{0.0};
        self.scalar_static_bool[85]=(1.0==self.scalar_static_f64[1107]);
        self.scalar_static_f64[1108]=(if self.scalar_static_bool[85]{1.0}else{0.0});
        self.scalar_static_bool[86]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1108]!=0.0));
        self.scalar_static_f64[1109]=p.p399;
        self.scalar_static_f64[1110]=(if self.scalar_static_bool[86]{self.scalar_static_f64[1109]}else{self.scalar_static_f64[1106]});
        self.scalar_static_f64[1111]=(if self.scalar_static_bool[80]{self.scalar_static_f64[503]}else{0.0});
        self.scalar_static_f64[1112]=if param_given[402]{1.0}else{0.0};
        self.scalar_static_bool[87]=(1.0==self.scalar_static_f64[1112]);
        self.scalar_static_f64[1113]=(if self.scalar_static_bool[87]{1.0}else{0.0});
        self.scalar_static_bool[88]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1113]!=0.0));
        self.scalar_static_f64[1114]=p.p402;
        self.scalar_static_f64[1115]=(if self.scalar_static_bool[88]{self.scalar_static_f64[1114]}else{self.scalar_static_f64[1111]});
        self.scalar_static_f64[1116]=(if self.scalar_static_bool[80]{self.scalar_static_f64[505]}else{0.0});
        self.scalar_static_f64[1117]=if param_given[403]{1.0}else{0.0};
        self.scalar_static_bool[89]=(1.0==self.scalar_static_f64[1117]);
        self.scalar_static_f64[1118]=(if self.scalar_static_bool[89]{1.0}else{0.0});
        self.scalar_static_bool[90]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1118]!=0.0));
        self.scalar_static_f64[1119]=p.p403;
        self.scalar_static_f64[1120]=(if self.scalar_static_bool[90]{self.scalar_static_f64[1119]}else{self.scalar_static_f64[1116]});
        self.scalar_static_f64[1121]=(if self.scalar_static_bool[80]{self.scalar_static_f64[496]}else{0.0});
        self.scalar_static_f64[1122]=if param_given[400]{1.0}else{0.0};
        self.scalar_static_bool[91]=(1.0==self.scalar_static_f64[1122]);
        self.scalar_static_f64[1123]=(if self.scalar_static_bool[91]{1.0}else{0.0});
        self.scalar_static_bool[92]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1123]!=0.0));
        self.scalar_static_f64[1124]=p.p400;
        self.scalar_static_f64[1125]=(if self.scalar_static_bool[92]{self.scalar_static_f64[1124]}else{self.scalar_static_f64[1121]});
        self.scalar_static_f64[1126]=(if self.scalar_static_bool[80]{self.scalar_static_f64[497]}else{0.0});
        self.scalar_static_f64[1127]=if param_given[401]{1.0}else{0.0};
        self.scalar_static_bool[93]=(1.0==self.scalar_static_f64[1127]);
        self.scalar_static_f64[1128]=(if self.scalar_static_bool[93]{1.0}else{0.0});
        self.scalar_static_bool[94]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1128]!=0.0));
        self.scalar_static_f64[1129]=p.p401;
        self.scalar_static_f64[1130]=(if self.scalar_static_bool[94]{self.scalar_static_f64[1129]}else{self.scalar_static_f64[1126]});
        self.scalar_static_f64[1131]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1110]);
        self.scalar_static_f64[1132]=(self.scalar_static_f64[1105]*self.scalar_static_f64[1131]);
        self.scalar_static_f64[1133]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1130]);
        self.scalar_static_f64[1134]=(self.scalar_static_f64[1125]*self.scalar_static_f64[1133]);
        self.scalar_static_f64[1135]=(1.0+self.scalar_static_f64[1134]);
        self.scalar_static_f64[1136]=(self.scalar_static_f64[1132]/self.scalar_static_f64[1135]);
        self.scalar_static_f64[1137]=(self.scalar_static_f64[441]*self.scalar_static_f64[1115]);
        self.scalar_static_f64[1138]=(self.scalar_static_f64[443]*self.scalar_static_f64[1120]);
        self.scalar_static_f64[1139]=(if self.scalar_static_bool[80]{self.scalar_static_f64[507]}else{0.0});
        self.scalar_static_f64[1140]=if param_given[404]{1.0}else{0.0};
        self.scalar_static_bool[95]=(1.0==self.scalar_static_f64[1140]);
        self.scalar_static_f64[1141]=(if self.scalar_static_bool[95]{1.0}else{0.0});
        self.scalar_static_bool[96]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1141]!=0.0));
        self.scalar_static_f64[1142]=p.p404;
        self.scalar_static_f64[1143]=(if self.scalar_static_bool[96]{self.scalar_static_f64[1142]}else{self.scalar_static_f64[1139]});
        self.scalar_static_f64[1144]=(if self.scalar_static_bool[80]{self.scalar_static_f64[508]}else{0.0});
        self.scalar_static_f64[1145]=if param_given[405]{1.0}else{0.0};
        self.scalar_static_bool[97]=(1.0==self.scalar_static_f64[1145]);
        self.scalar_static_f64[1146]=(if self.scalar_static_bool[97]{1.0}else{0.0});
        self.scalar_static_bool[98]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1146]!=0.0));
        self.scalar_static_f64[1147]=p.p405;
        self.scalar_static_f64[1148]=(if self.scalar_static_bool[98]{self.scalar_static_f64[1147]}else{self.scalar_static_f64[1144]});
        self.scalar_static_f64[1149]=(self.scalar_static_f64[464]*self.scalar_static_f64[1148]);
        self.scalar_static_f64[1150]=(self.scalar_static_f64[1149]/self.scalar_static_f64[458]);
        self.scalar_static_f64[1151]=(if self.scalar_static_bool[80]{self.scalar_static_f64[552]}else{0.0});
        self.scalar_static_f64[1152]=if param_given[406]{1.0}else{0.0};
        self.scalar_static_bool[99]=(1.0==self.scalar_static_f64[1152]);
        self.scalar_static_f64[1153]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_bool[100]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1153]!=0.0));
        self.scalar_static_f64[1154]=p.p406;
        self.scalar_static_f64[1155]=(if self.scalar_static_bool[100]{self.scalar_static_f64[1154]}else{self.scalar_static_f64[1151]});
        self.scalar_static_f64[1156]=(if self.scalar_static_bool[80]{self.scalar_static_f64[554]}else{0.0});
        self.scalar_static_f64[1157]=if param_given[407]{1.0}else{0.0};
        self.scalar_static_bool[101]=(1.0==self.scalar_static_f64[1157]);
        self.scalar_static_f64[1158]=(if self.scalar_static_bool[101]{1.0}else{0.0});
        self.scalar_static_bool[102]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1158]!=0.0));
        self.scalar_static_f64[1159]=p.p407;
        self.scalar_static_f64[1160]=(if self.scalar_static_bool[102]{self.scalar_static_f64[1159]}else{self.scalar_static_f64[1156]});
        self.scalar_static_f64[1161]=(if self.scalar_static_bool[80]{self.scalar_static_f64[557]}else{0.0});
        self.scalar_static_f64[1162]=if param_given[408]{1.0}else{0.0};
        self.scalar_static_bool[103]=(1.0==self.scalar_static_f64[1162]);
        self.scalar_static_f64[1163]=(if self.scalar_static_bool[103]{1.0}else{0.0});
        self.scalar_static_bool[104]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1163]!=0.0));
        self.scalar_static_f64[1164]=p.p408;
        self.scalar_static_f64[1165]=(if self.scalar_static_bool[104]{self.scalar_static_f64[1164]}else{self.scalar_static_f64[1161]});
        self.scalar_static_f64[1166]=(2.0*self.scalar_static_f64[1155]);
        self.scalar_static_f64[1167]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[1160]);
        self.scalar_static_f64[1168]=(self.scalar_static_f64[1166]*self.scalar_static_f64[1167]);
        self.scalar_static_f64[1169]=(self.scalar_static_f64[441]*self.scalar_static_f64[1165]);
        self.scalar_static_f64[1170]=(1.0+self.scalar_static_f64[1169]);
        self.scalar_static_f64[1171]=(self.scalar_static_f64[1168]*self.scalar_static_f64[1170]);
        self.scalar_static_f64[1172]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1171]}else{0.0});
        self.scalar_static_bool[105]=(self.scalar_static_f64[1172]>0.0);
        self.scalar_static_f64[1173]=(if self.scalar_static_bool[105]{self.scalar_static_f64[1172]}else{0.0});
        self.scalar_static_bool[106]=(self.scalar_static_f64[1173]<5.0);
        self.scalar_static_f64[1174]=(if self.scalar_static_bool[106]{self.scalar_static_f64[1173]}else{5.0});
        self.scalar_static_f64[1175]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1174]}else{self.scalar_static_f64[1090]});
        self.scalar_static_f64[1176]=(self.scalar_static_f64[565]*self.scalar_static_f64[1175]);
        self.scalar_static_f64[1177]=(self.scalar_static_f64[464]*self.scalar_static_f64[1176]);
        self.scalar_static_f64[1178]=(self.scalar_static_f64[1177]/self.scalar_static_f64[458]);
        self.scalar_static_f64[1179]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1178]}else{self.scalar_static_f64[1091]});
        self.scalar_static_f64[1180]=(if self.scalar_static_bool[80]{self.scalar_static_f64[587]}else{0.0});
        self.scalar_static_f64[1181]=if param_given[409]{1.0}else{0.0};
        self.scalar_static_bool[107]=(1.0==self.scalar_static_f64[1181]);
        self.scalar_static_f64[1182]=(if self.scalar_static_bool[107]{1.0}else{0.0});
        self.scalar_static_bool[108]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1182]!=0.0));
        self.scalar_static_f64[1183]=p.p409;
        self.scalar_static_f64[1184]=(if self.scalar_static_bool[108]{self.scalar_static_f64[1183]}else{self.scalar_static_f64[1180]});
        self.scalar_static_f64[1185]=(if self.scalar_static_bool[80]{self.scalar_static_f64[581]}else{0.0});
        self.scalar_static_f64[1186]=if param_given[410]{1.0}else{0.0};
        self.scalar_static_bool[109]=(1.0==self.scalar_static_f64[1186]);
        self.scalar_static_f64[1187]=(if self.scalar_static_bool[109]{1.0}else{0.0});
        self.scalar_static_bool[110]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1187]!=0.0));
        self.scalar_static_f64[1188]=p.p410;
        self.scalar_static_f64[1189]=(if self.scalar_static_bool[110]{self.scalar_static_f64[1188]}else{self.scalar_static_f64[1185]});
        self.scalar_static_f64[1190]=(if self.scalar_static_bool[80]{self.scalar_static_f64[583]}else{0.0});
        self.scalar_static_f64[1191]=if param_given[411]{1.0}else{0.0};
        self.scalar_static_bool[111]=(1.0==self.scalar_static_f64[1191]);
        self.scalar_static_f64[1192]=(if self.scalar_static_bool[111]{1.0}else{0.0});
        self.scalar_static_bool[112]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1192]!=0.0));
        self.scalar_static_f64[1193]=p.p411;
        self.scalar_static_f64[1194]=(if self.scalar_static_bool[112]{self.scalar_static_f64[1193]}else{self.scalar_static_f64[1190]});
        self.scalar_static_f64[1195]=f64::powf(self.scalar_static_f64[551],self.scalar_static_f64[1189]);
        self.scalar_static_f64[1196]=(self.scalar_static_f64[441]*self.scalar_static_f64[1194]);
        self.scalar_static_f64[1197]=(1.0+self.scalar_static_f64[1196]);
        self.scalar_static_f64[1198]=(self.scalar_static_f64[1195]*self.scalar_static_f64[1197]);
        self.scalar_static_f64[1199]=(if self.scalar_static_bool[80]{self.scalar_static_f64[788]}else{0.0});
        self.scalar_static_f64[1200]=if param_given[412]{1.0}else{0.0};
        self.scalar_static_bool[113]=(1.0==self.scalar_static_f64[1200]);
        self.scalar_static_f64[1201]=(if self.scalar_static_bool[113]{1.0}else{0.0});
        self.scalar_static_bool[114]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1201]!=0.0));
        self.scalar_static_f64[1202]=p.p412;
        self.scalar_static_f64[1203]=(if self.scalar_static_bool[114]{self.scalar_static_f64[1202]}else{self.scalar_static_f64[1199]});
        self.scalar_static_f64[1204]=(if self.scalar_static_bool[80]{self.scalar_static_f64[789]}else{0.0});
        self.scalar_static_f64[1205]=if param_given[413]{1.0}else{0.0};
        self.scalar_static_bool[115]=(1.0==self.scalar_static_f64[1205]);
        self.scalar_static_f64[1206]=(if self.scalar_static_bool[115]{1.0}else{0.0});
        self.scalar_static_bool[116]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1206]!=0.0));
        self.scalar_static_f64[1207]=p.p413;
        self.scalar_static_f64[1208]=(if self.scalar_static_bool[116]{self.scalar_static_f64[1207]}else{self.scalar_static_f64[1204]});
        self.scalar_static_f64[1209]=(if self.scalar_static_bool[80]{self.scalar_static_f64[790]}else{0.0});
        self.scalar_static_f64[1210]=if param_given[414]{1.0}else{0.0};
        self.scalar_static_bool[117]=(1.0==self.scalar_static_f64[1210]);
        self.scalar_static_f64[1211]=(if self.scalar_static_bool[117]{1.0}else{0.0});
        self.scalar_static_bool[118]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1211]!=0.0));
        self.scalar_static_f64[1212]=p.p414;
        self.scalar_static_f64[1213]=(if self.scalar_static_bool[118]{self.scalar_static_f64[1212]}else{self.scalar_static_f64[1209]});
        self.scalar_static_f64[1214]=(if self.scalar_static_bool[80]{self.scalar_static_f64[795]}else{0.0});
        self.scalar_static_f64[1215]=if param_given[415]{1.0}else{0.0};
        self.scalar_static_bool[119]=(1.0==self.scalar_static_f64[1215]);
        self.scalar_static_f64[1216]=(if self.scalar_static_bool[119]{1.0}else{0.0});
        self.scalar_static_bool[120]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1216]!=0.0));
        self.scalar_static_f64[1217]=p.p415;
        self.scalar_static_f64[1218]=(if self.scalar_static_bool[120]{self.scalar_static_f64[1217]}else{self.scalar_static_f64[1214]});
        self.scalar_static_f64[1219]=(if self.scalar_static_bool[80]{self.scalar_static_f64[799]}else{0.0});
        self.scalar_static_f64[1220]=if param_given[416]{1.0}else{0.0};
        self.scalar_static_bool[121]=(1.0==self.scalar_static_f64[1220]);
        self.scalar_static_f64[1221]=(if self.scalar_static_bool[121]{1.0}else{0.0});
        self.scalar_static_bool[122]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1221]!=0.0));
        self.scalar_static_f64[1222]=p.p416;
        self.scalar_static_f64[1223]=(if self.scalar_static_bool[122]{self.scalar_static_f64[1222]}else{self.scalar_static_f64[1219]});
        self.scalar_static_f64[1224]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1213]);
        self.scalar_static_f64[1225]=(self.scalar_static_f64[1208]*self.scalar_static_f64[1224]);
        self.scalar_static_f64[1226]=(self.scalar_static_f64[1203]+self.scalar_static_f64[1225]);
        self.scalar_static_f64[1227]=(self.scalar_static_f64[674]*self.scalar_static_f64[1226]);
        self.scalar_static_f64[1228]=(self.scalar_static_f64[441]*self.scalar_static_f64[1218]);
        self.scalar_static_f64[1229]=(1.0+self.scalar_static_f64[1228]);
        self.scalar_static_f64[1230]=(self.scalar_static_f64[1227]*self.scalar_static_f64[1229]);
        self.scalar_static_f64[1231]=(self.scalar_static_f64[443]*self.scalar_static_f64[1223]);
        self.scalar_static_f64[1232]=(1.0+self.scalar_static_f64[1231]);
        self.scalar_static_f64[1233]=(self.scalar_static_f64[1230]*self.scalar_static_f64[1232]);
        self.scalar_static_f64[1234]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1233]}else{self.scalar_static_f64[1092]});
        self.scalar_static_bool[123]=(self.scalar_static_f64[1234]>0.0);
        self.scalar_static_f64[1235]=(if self.scalar_static_bool[123]{self.scalar_static_f64[1234]}else{0.0});
        self.scalar_static_f64[1236]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1235]}else{self.scalar_static_f64[1093]});
        self.scalar_static_f64[1237]=(if self.scalar_static_bool[80]{self.scalar_static_f64[824]}else{0.0});
        self.scalar_static_f64[1238]=if param_given[417]{1.0}else{0.0};
        self.scalar_static_bool[124]=(1.0==self.scalar_static_f64[1238]);
        self.scalar_static_f64[1239]=(if self.scalar_static_bool[124]{1.0}else{0.0});
        self.scalar_static_bool[125]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1239]!=0.0));
        self.scalar_static_f64[1240]=p.p417;
        self.scalar_static_f64[1241]=(if self.scalar_static_bool[125]{self.scalar_static_f64[1240]}else{self.scalar_static_f64[1237]});
        self.scalar_static_f64[1242]=(if self.scalar_static_bool[80]{self.scalar_static_f64[825]}else{0.0});
        self.scalar_static_f64[1243]=if param_given[418]{1.0}else{0.0};
        self.scalar_static_bool[126]=(1.0==self.scalar_static_f64[1243]);
        self.scalar_static_f64[1244]=(if self.scalar_static_bool[126]{1.0}else{0.0});
        self.scalar_static_bool[127]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1244]!=0.0));
        self.scalar_static_f64[1245]=p.p418;
        self.scalar_static_f64[1246]=(if self.scalar_static_bool[127]{self.scalar_static_f64[1245]}else{self.scalar_static_f64[1242]});
        self.scalar_static_f64[1247]=(if self.scalar_static_bool[80]{self.scalar_static_f64[826]}else{0.0});
        self.scalar_static_f64[1248]=if param_given[419]{1.0}else{0.0};
        self.scalar_static_bool[128]=(1.0==self.scalar_static_f64[1248]);
        self.scalar_static_f64[1249]=(if self.scalar_static_bool[128]{1.0}else{0.0});
        self.scalar_static_bool[129]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1249]!=0.0));
        self.scalar_static_f64[1250]=p.p419;
        self.scalar_static_f64[1251]=(if self.scalar_static_bool[129]{self.scalar_static_f64[1250]}else{self.scalar_static_f64[1247]});
        self.scalar_static_f64[1252]=(if self.scalar_static_bool[80]{self.scalar_static_f64[829]}else{0.0});
        self.scalar_static_f64[1253]=if param_given[420]{1.0}else{0.0};
        self.scalar_static_bool[130]=(1.0==self.scalar_static_f64[1253]);
        self.scalar_static_f64[1254]=(if self.scalar_static_bool[130]{1.0}else{0.0});
        self.scalar_static_bool[131]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1254]!=0.0));
        self.scalar_static_f64[1255]=p.p420;
        self.scalar_static_f64[1256]=(if self.scalar_static_bool[131]{self.scalar_static_f64[1255]}else{self.scalar_static_f64[1252]});
        self.scalar_static_f64[1257]=(if self.scalar_static_bool[80]{self.scalar_static_f64[830]}else{0.0});
        self.scalar_static_f64[1258]=if param_given[421]{1.0}else{0.0};
        self.scalar_static_bool[132]=(1.0==self.scalar_static_f64[1258]);
        self.scalar_static_f64[1259]=(if self.scalar_static_bool[132]{1.0}else{0.0});
        self.scalar_static_bool[133]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1259]!=0.0));
        self.scalar_static_f64[1260]=p.p421;
        self.scalar_static_f64[1261]=(if self.scalar_static_bool[133]{self.scalar_static_f64[1260]}else{self.scalar_static_f64[1257]});
        self.scalar_static_f64[1262]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1251]);
        self.scalar_static_f64[1263]=(self.scalar_static_f64[1246]*self.scalar_static_f64[1262]);
        self.scalar_static_f64[1264]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1261]);
        self.scalar_static_f64[1265]=(self.scalar_static_f64[1256]*self.scalar_static_f64[1264]);
        self.scalar_static_f64[1266]=(1.0+self.scalar_static_f64[1265]);
        self.scalar_static_f64[1267]=(self.scalar_static_f64[1263]/self.scalar_static_f64[1266]);
        self.scalar_static_f64[1268]=(1.0+self.scalar_static_f64[1267]);
        self.scalar_static_f64[1269]=(self.scalar_static_f64[1241]/self.scalar_static_f64[1268]);
        self.scalar_static_f64[1270]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1269]}else{0.0});
        self.scalar_static_bool[134]=(self.scalar_static_f64[1270]>1.0);
        self.scalar_static_f64[1271]=(if self.scalar_static_bool[134]{self.scalar_static_f64[1270]}else{1.0});
        self.scalar_static_bool[135]=(self.scalar_static_f64[1271]<16.0);
        self.scalar_static_f64[1272]=(if self.scalar_static_bool[135]{self.scalar_static_f64[1271]}else{16.0});
        self.scalar_static_f64[1273]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1272]}else{self.scalar_static_f64[1094]});
        self.scalar_static_f64[1274]=(if self.scalar_static_bool[80]{self.scalar_static_f64[841]}else{0.0});
        self.scalar_static_f64[1275]=if param_given[422]{1.0}else{0.0};
        self.scalar_static_bool[136]=(1.0==self.scalar_static_f64[1275]);
        self.scalar_static_f64[1276]=(if self.scalar_static_bool[136]{1.0}else{0.0});
        self.scalar_static_bool[137]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1276]!=0.0));
        self.scalar_static_f64[1277]=p.p422;
        self.scalar_static_f64[1278]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1277]}else{self.scalar_static_f64[1274]});
        self.scalar_static_f64[1279]=(if self.scalar_static_bool[80]{self.scalar_static_f64[842]}else{0.0});
        self.scalar_static_f64[1280]=if param_given[423]{1.0}else{0.0};
        self.scalar_static_bool[138]=(1.0==self.scalar_static_f64[1280]);
        self.scalar_static_f64[1281]=(if self.scalar_static_bool[138]{1.0}else{0.0});
        self.scalar_static_bool[139]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1281]!=0.0));
        self.scalar_static_f64[1282]=p.p423;
        self.scalar_static_f64[1283]=(if self.scalar_static_bool[139]{self.scalar_static_f64[1282]}else{self.scalar_static_f64[1279]});
        self.scalar_static_f64[1284]=(if self.scalar_static_bool[80]{self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[1285]=if param_given[424]{1.0}else{0.0};
        self.scalar_static_bool[140]=(1.0==self.scalar_static_f64[1285]);
        self.scalar_static_f64[1286]=(if self.scalar_static_bool[140]{1.0}else{0.0});
        self.scalar_static_bool[141]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1286]!=0.0));
        self.scalar_static_f64[1287]=p.p424;
        self.scalar_static_f64[1288]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1287]}else{self.scalar_static_f64[1284]});
        self.scalar_static_f64[1289]=(if self.scalar_static_bool[80]{self.scalar_static_f64[850]}else{0.0});
        self.scalar_static_f64[1290]=if param_given[425]{1.0}else{0.0};
        self.scalar_static_bool[142]=(1.0==self.scalar_static_f64[1290]);
        self.scalar_static_f64[1291]=(if self.scalar_static_bool[142]{1.0}else{0.0});
        self.scalar_static_bool[143]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1291]!=0.0));
        self.scalar_static_f64[1292]=p.p425;
        self.scalar_static_f64[1293]=(if self.scalar_static_bool[143]{self.scalar_static_f64[1292]}else{self.scalar_static_f64[1289]});
        self.scalar_static_f64[1294]=(if self.scalar_static_bool[80]{self.scalar_static_f64[845]}else{0.0});
        self.scalar_static_f64[1295]=if param_given[426]{1.0}else{0.0};
        self.scalar_static_bool[144]=(1.0==self.scalar_static_f64[1295]);
        self.scalar_static_f64[1296]=(if self.scalar_static_bool[144]{1.0}else{0.0});
        self.scalar_static_bool[145]=(self.scalar_static_bool[80]&&(self.scalar_static_f64[1296]!=0.0));
        self.scalar_static_f64[1297]=p.p426;
        self.scalar_static_f64[1298]=(if self.scalar_static_bool[145]{self.scalar_static_f64[1297]}else{self.scalar_static_f64[1294]});
        self.scalar_static_f64[1299]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1283]);
        self.scalar_static_f64[1300]=(self.scalar_static_f64[1278]*self.scalar_static_f64[1299]);
        self.scalar_static_f64[1301]=(self.scalar_static_f64[441]*self.scalar_static_f64[1298]);
        self.scalar_static_f64[1302]=(1.0+self.scalar_static_f64[1301]);
        self.scalar_static_f64[1303]=(self.scalar_static_f64[1300]*self.scalar_static_f64[1302]);
        self.scalar_static_f64[1304]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1293]);
        self.scalar_static_f64[1305]=(self.scalar_static_f64[1288]*self.scalar_static_f64[1304]);
        self.scalar_static_f64[1306]=(1.0+self.scalar_static_f64[1305]);
        self.scalar_static_f64[1307]=(self.scalar_static_f64[1303]/self.scalar_static_f64[1306]);
        self.scalar_static_f64[1308]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1307]}else{0.0});
        self.scalar_static_bool[146]=(self.scalar_static_f64[1308]>0.0);
        self.scalar_static_f64[1309]=(if self.scalar_static_bool[146]{self.scalar_static_f64[1308]}else{0.0});
        self.scalar_static_f64[1310]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1309]}else{self.scalar_static_f64[1095]});
        self.scalar_static_f64[1311]=(3.45313e-11/self.scalar_static_f64[458]);
        self.scalar_static_f64[1312]=(self.scalar_static_f64[437]*self.scalar_static_f64[1311]);
        self.scalar_static_f64[1313]=p.p427;
        self.scalar_static_f64[1314]=p.p428;
        self.scalar_static_f64[1315]=p.p429;
        self.scalar_static_f64[1316]=p.p430;
        self.scalar_static_f64[1317]=(self.scalar_static_f64[392]*self.scalar_static_f64[1316]);
        self.scalar_static_f64[1318]=(self.scalar_static_f64[1317]/self.scalar_static_f64[437]);
        self.scalar_static_f64[1319]=(1.0+self.scalar_static_f64[1318]);
        self.scalar_static_bool[147]=(self.scalar_static_f64[1319]>0.001);
        self.scalar_static_f64[1320]=(if self.scalar_static_bool[147]{self.scalar_static_f64[1319]}else{0.001});
        self.scalar_static_f64[1321]=(self.scalar_static_f64[1315]/self.scalar_static_f64[1320]);
        self.scalar_static_f64[1322]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1321]}else{self.scalar_static_f64[345]});
        self.scalar_static_f64[1323]=p.p431;
        self.scalar_static_f64[1324]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1323]}else{self.scalar_static_f64[347]});
        self.scalar_static_f64[1325]=p.p432;
        self.scalar_static_f64[1326]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1325]}else{self.scalar_static_f64[349]});
        self.scalar_static_f64[1327]=p.p433;
        self.scalar_static_f64[1328]=p.p435;
        self.scalar_static_f64[1329]=p.p434;
        self.scalar_static_f64[1330]=p.p436;
        self.scalar_static_f64[1331]=p.p437;
        self.scalar_static_f64[1332]=(self.scalar_static_f64[544]*self.scalar_static_f64[1331]);
        self.scalar_static_f64[1333]=(self.scalar_static_f64[460]*self.scalar_static_f64[1332]);
        self.scalar_static_f64[1334]=(self.scalar_static_f64[429]*self.scalar_static_f64[1333]);
        self.scalar_static_f64[1335]=(self.scalar_static_f64[1334]/self.scalar_static_f64[423]);
        self.scalar_static_f64[1336]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1335]}else{self.scalar_static_f64[355]});
        self.scalar_static_f64[1337]=p.p438;
        self.scalar_static_f64[1338]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1337]}else{self.scalar_static_f64[357]});
        self.scalar_static_f64[1339]=p.p440;
        self.scalar_static_f64[1340]=p.p441;
        self.scalar_static_f64[1341]=p.p442;
        self.scalar_static_f64[1342]=(if self.scalar_static_bool[33]{0.0}else{self.scalar_static_f64[610]});
        self.scalar_static_bool[148]=(self.scalar_static_f64[375]>1.0);
        self.scalar_static_f64[1343]=p.p28;
        self.scalar_static_bool[149]=(self.scalar_static_f64[1343]>0.0);
        self.scalar_static_bool[150]=(self.scalar_static_bool[148]&&self.scalar_static_bool[149]);
        self.scalar_static_f64[1344]=(if self.scalar_static_bool[150]{1.0}else{0.0});
        self.scalar_static_bool[151]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[1344]!=0.0));
        self.scalar_static_f64[1345]=(self.scalar_static_f64[393]+self.scalar_static_f64[1343]);
        self.scalar_static_f64[1346]=(-self.scalar_static_f64[1345]);
        self.scalar_static_f64[1347]=p.p445;
        self.scalar_static_f64[1348]=(self.scalar_static_f64[1346]/self.scalar_static_f64[1347]);
        self.scalar_static_f64[1349]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1348]}else{self.scalar_static_f64[624]});
        self.scalar_static_f64[1350]=(self.scalar_static_f64[1349]).abs();
        self.scalar_static_bool[152]=(self.scalar_static_f64[1350]<80.0);
        self.scalar_static_f64[1351]=(if self.scalar_static_bool[152]{1.0}else{0.0});
        self.scalar_static_bool[153]=(self.scalar_static_bool[151]&&(self.scalar_static_f64[1351]!=0.0));
        self.scalar_static_f64[1352]=(self.scalar_static_f64[1349]).exp();
        self.scalar_static_f64[1353]=(if self.scalar_static_bool[153]{self.scalar_static_f64[1352]}else{self.scalar_static_f64[627]});
        self.scalar_static_bool[154]=(self.scalar_static_f64[1349]< -80.0);
        self.scalar_static_f64[1354]=(if self.scalar_static_bool[154]{1.0}else{0.0});
        self.scalar_static_bool[155]=(!(self.scalar_static_f64[1351]!=0.0));
        self.scalar_static_bool[156]=(self.scalar_static_bool[151]&&self.scalar_static_bool[155]);
        self.scalar_static_bool[157]=((self.scalar_static_f64[1354]!=0.0)&&self.scalar_static_bool[156]);
        self.scalar_static_f64[1355]=(-self.scalar_static_f64[1349]);
        self.scalar_static_f64[1356]=(self.scalar_static_f64[1355]-80.0);
        self.scalar_static_f64[1357]=(0.5*self.scalar_static_f64[1356]);
        self.scalar_static_f64[1358]=(0.3333333333333*self.scalar_static_f64[1356]);
        self.scalar_static_f64[1359]=(1.0+self.scalar_static_f64[1358]);
        self.scalar_static_f64[1360]=(self.scalar_static_f64[1357]*self.scalar_static_f64[1359]);
        self.scalar_static_f64[1361]=(1.0+self.scalar_static_f64[1360]);
        self.scalar_static_f64[1362]=(self.scalar_static_f64[1356]*self.scalar_static_f64[1361]);
        self.scalar_static_f64[1363]=(1.0+self.scalar_static_f64[1362]);
        self.scalar_static_f64[1364]=(1.80485e-35/self.scalar_static_f64[1363]);
        self.scalar_static_f64[1365]=(if self.scalar_static_bool[157]{self.scalar_static_f64[1364]}else{self.scalar_static_f64[1353]});
        self.scalar_static_bool[158]=(!(self.scalar_static_f64[1354]!=0.0));
        self.scalar_static_bool[159]=(self.scalar_static_bool[156]&&self.scalar_static_bool[158]);
        self.scalar_static_f64[1366]=(self.scalar_static_f64[1349]-80.0);
        self.scalar_static_f64[1367]=(0.5*self.scalar_static_f64[1366]);
        self.scalar_static_f64[1368]=(0.3333333333333*self.scalar_static_f64[1366]);
        self.scalar_static_f64[1369]=(1.0+self.scalar_static_f64[1368]);
        self.scalar_static_f64[1370]=(self.scalar_static_f64[1367]*self.scalar_static_f64[1369]);
        self.scalar_static_f64[1371]=(1.0+self.scalar_static_f64[1370]);
        self.scalar_static_f64[1372]=(self.scalar_static_f64[1366]*self.scalar_static_f64[1371]);
        self.scalar_static_f64[1373]=(1.0+self.scalar_static_f64[1372]);
        self.scalar_static_f64[1374]=(5.54062e34*self.scalar_static_f64[1373]);
        self.scalar_static_f64[1375]=(if self.scalar_static_bool[159]{self.scalar_static_f64[1374]}else{self.scalar_static_f64[1365]});
        self.scalar_static_f64[1376]=(1.0-self.scalar_static_f64[1375]);
        self.scalar_static_f64[1377]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1376]}else{self.scalar_static_f64[641]});
        self.scalar_static_f64[1378]=p.p446;
        self.scalar_static_f64[1379]=(2.0*self.scalar_static_f64[1378]);
        self.scalar_static_f64[1380]=(self.scalar_static_f64[1375]*self.scalar_static_f64[1379]);
        self.scalar_static_f64[1381]=f64::powf(self.scalar_static_f64[1375],self.scalar_static_f64[375]);
        self.scalar_static_f64[1382]=(1.0-self.scalar_static_f64[1381]);
        self.scalar_static_f64[1383]=(self.scalar_static_f64[1382]/self.scalar_static_f64[375]);
        self.scalar_static_f64[1384]=(self.scalar_static_f64[1377]-self.scalar_static_f64[1383]);
        self.scalar_static_f64[1385]=(self.scalar_static_f64[1380]*self.scalar_static_f64[1384]);
        self.scalar_static_f64[1386]=(self.scalar_static_f64[1377]*self.scalar_static_f64[1377]);
        self.scalar_static_f64[1387]=(self.scalar_static_f64[1385]/self.scalar_static_f64[1386]);
        self.scalar_static_f64[1388]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1387]}else{self.scalar_static_f64[1342]});
        self.scalar_static_f64[1389]=(1.0+self.scalar_static_f64[1388]);
        self.scalar_static_f64[1390]=p.p443;
        self.scalar_static_f64[1391]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1390]}else{self.scalar_static_f64[360]});
        self.scalar_static_f64[1392]=p.p444;
        self.scalar_static_f64[1393]=p.p447;
        self.scalar_static_f64[1394]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1393]}else{self.scalar_static_f64[364]});
        self.scalar_static_f64[1395]=p.p448;
        self.scalar_static_f64[1396]=(self.scalar_static_f64[677]*self.scalar_static_f64[1395]);
        self.scalar_static_f64[1397]=(self.scalar_static_f64[677]*self.scalar_static_f64[1396]);
        self.scalar_static_f64[1398]=(self.scalar_static_f64[441]*self.scalar_static_f64[1397]);
        self.scalar_static_f64[1399]=(self.scalar_static_f64[441]*self.scalar_static_f64[1398]);
        self.scalar_static_f64[1400]=p.p449;
        self.scalar_static_f64[1401]=(self.scalar_static_f64[1400]-2.0);
        self.scalar_static_f64[1402]=f64::powf(self.scalar_static_f64[439],self.scalar_static_f64[1401]);
        self.scalar_static_f64[1403]=(self.scalar_static_f64[1399]*self.scalar_static_f64[1402]);
        self.scalar_static_f64[1404]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1403]}else{self.scalar_static_f64[366]});
        self.scalar_static_f64[1405]=p.p488;
        self.scalar_static_f64[1406]=(self.scalar_static_f64[451]*0.3333333333333);
        self.scalar_static_f64[1407]=p.p37;
        self.scalar_static_f64[1408]=(self.scalar_static_f64[1406]/self.scalar_static_f64[1407]);
        self.scalar_static_f64[1409]=(self.scalar_static_f64[456]+self.scalar_static_f64[1408]);
        self.scalar_static_f64[1410]=(self.scalar_static_f64[1405]*self.scalar_static_f64[1409]);
        self.scalar_static_f64[1411]=(self.scalar_static_f64[450]*self.scalar_static_f64[1407]);
        self.scalar_static_f64[1412]=(self.scalar_static_f64[1410]/self.scalar_static_f64[1411]);
        self.scalar_static_f64[1413]=p.p486;
        self.scalar_static_f64[1414]=p.p487;
        self.scalar_static_f64[1415]=(self.scalar_static_f64[1413]+self.scalar_static_f64[1414]);
        self.scalar_static_f64[1416]=(self.scalar_static_f64[446]*self.scalar_static_f64[451]);
        self.scalar_static_f64[1417]=(self.scalar_static_f64[1415]/self.scalar_static_f64[1416]);
        self.scalar_static_f64[1418]=(self.scalar_static_f64[1412]+self.scalar_static_f64[1417]);
        self.scalar_static_f64[1419]=p.p485;
        self.scalar_static_f64[1420]=(self.scalar_static_f64[375]*self.scalar_static_f64[1419]);
        self.scalar_static_f64[1421]=(self.scalar_static_f64[1418]+self.scalar_static_f64[1420]);
        self.scalar_static_f64[1422]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1421]}else{0.0});
        self.scalar_static_bool[160]=(self.scalar_static_f64[1422]>0.0);
        self.scalar_static_f64[1423]=(if self.scalar_static_bool[160]{self.scalar_static_f64[1422]}else{0.0});
        self.scalar_static_f64[1424]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1423]}else{self.scalar_static_f64[368]});
        self.scalar_static_f64[1425]=p.p490;
        self.scalar_static_bool[161]=(self.scalar_static_f64[1425]>0.0);
        self.scalar_static_f64[1426]=(if self.scalar_static_bool[161]{self.scalar_static_f64[1425]}else{0.0});
        self.scalar_static_f64[1427]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1426]}else{0.0});
        self.scalar_static_f64[1428]=p.p491;
        self.scalar_static_bool[162]=(self.scalar_static_f64[1428]>0.0);
        self.scalar_static_f64[1429]=(if self.scalar_static_bool[162]{self.scalar_static_f64[1428]}else{0.0});
        self.scalar_static_f64[1430]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1429]}else{0.0});
        self.scalar_static_f64[1431]=p.p7;
        self.scalar_static_bool[163]=(0.0==self.scalar_static_f64[1431]);
        self.scalar_static_f64[1432]=(if self.scalar_static_bool[163]{1.0}else{0.0});
        self.scalar_static_bool[164]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[1432]!=0.0));
        self.scalar_static_f64[1433]=(if self.scalar_static_bool[164]{self.scalar_static_f64[1427]}else{self.scalar_static_f64[1430]});
        self.scalar_static_f64[1434]=p.p39;
        self.scalar_static_f64[1435]=(self.scalar_static_f64[375]*self.scalar_static_f64[1434]);
        self.scalar_static_f64[1436]=(self.scalar_static_f64[1427]*self.scalar_static_f64[1435]);
        self.scalar_static_f64[1437]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1436]}else{self.scalar_static_f64[370]});
        self.scalar_static_f64[1438]=p.p40;
        self.scalar_static_f64[1439]=(self.scalar_static_f64[375]*self.scalar_static_f64[1438]);
        self.scalar_static_f64[1440]=(self.scalar_static_f64[1433]*self.scalar_static_f64[1439]);
        self.scalar_static_f64[1441]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1440]}else{self.scalar_static_f64[372]});
        self.scalar_static_f64[1442]=p.p492;
        self.scalar_static_f64[1443]=(self.scalar_static_f64[375]*self.scalar_static_f64[1442]);
        self.scalar_static_f64[1444]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1443]}else{self.scalar_static_f64[374]});
        self.scalar_static_f64[1445]=p.p457;
        self.scalar_static_bool[165]=(self.scalar_static_f64[1445]>0.0);
        self.scalar_static_f64[1446]=p.p26;
        self.scalar_static_bool[166]=(self.scalar_static_f64[1446]>0.0);
        self.scalar_static_bool[167]=(self.scalar_static_bool[165]&&self.scalar_static_bool[166]);
        self.scalar_static_f64[1447]=p.p27;
        self.scalar_static_bool[168]=(self.scalar_static_f64[1447]>0.0);
        self.scalar_static_bool[169]=(self.scalar_static_bool[167]&&self.scalar_static_bool[168]);
        self.scalar_static_bool[170]=(1.0==self.scalar_static_f64[375]);
        self.scalar_static_bool[171]=(self.scalar_static_bool[150]||self.scalar_static_bool[170]);
        self.scalar_static_bool[172]=(self.scalar_static_bool[169]&&self.scalar_static_bool[171]);
        self.scalar_static_f64[1448]=(if self.scalar_static_bool[172]{1.0}else{0.0});
        self.scalar_static_bool[173]=(1.0==self.scalar_static_f64[1445]);
        self.scalar_static_f64[1449]=(if self.scalar_static_bool[173]{1.0}else{0.0});
        self.scalar_static_bool[174]=(self.scalar_static_bool[33]&&(self.scalar_static_f64[1448]!=0.0));
        self.scalar_static_bool[175]=((self.scalar_static_f64[1449]!=0.0)&&self.scalar_static_bool[174]);
        self.scalar_static_f64[1450]=(self.scalar_static_f64[375]-0.5);
        self.scalar_static_f64[1451]=(0.5*self.scalar_static_f64[393]);
        self.scalar_static_f64[1452]=(self.scalar_static_f64[1446]+self.scalar_static_f64[1451]);
        self.scalar_static_f64[1453]=(self.scalar_static_f64[1447]+self.scalar_static_f64[1451]);
        self.scalar_static_f64[1454]=p.p458;
        self.scalar_static_f64[1455]=(self.scalar_static_f64[1451]+self.scalar_static_f64[1454]);
        self.scalar_static_f64[1456]=(1.0/self.scalar_static_f64[1455]);
        self.scalar_static_f64[1457]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1456]}else{0.0});
        self.scalar_static_f64[1458]=p.p459;
        self.scalar_static_f64[1459]=(self.scalar_static_f64[1451]+self.scalar_static_f64[1458]);
        self.scalar_static_f64[1460]=(1.0/self.scalar_static_f64[1459]);
        self.scalar_static_f64[1461]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1460]}else{0.0});
        self.scalar_static_f64[1462]=(if self.scalar_static_bool[175]{self.scalar_static_f64[444]}else{0.0});
        self.scalar_static_f64[1463]=p.p460;
        self.scalar_static_f64[1464]=(self.scalar_static_f64[424]+self.scalar_static_f64[1463]);
        self.scalar_static_bool[176]=(self.scalar_static_f64[1464]>1e-9);
        self.scalar_static_f64[1465]=(if self.scalar_static_bool[176]{self.scalar_static_f64[1464]}else{1e-9});
        self.scalar_static_f64[1466]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1465]}else{0.0});
        self.scalar_static_f64[1467]=p.p467;
        self.scalar_static_f64[1468]=f64::powf(self.scalar_static_f64[1462],self.scalar_static_f64[1467]);
        self.scalar_static_f64[1469]=(1.0/self.scalar_static_f64[1468]);
        self.scalar_static_f64[1470]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1469]}else{0.0});
        self.scalar_static_f64[1471]=p.p468;
        self.scalar_static_f64[1472]=f64::powf(self.scalar_static_f64[1466],self.scalar_static_f64[1471]);
        self.scalar_static_f64[1473]=(1.0/self.scalar_static_f64[1472]);
        self.scalar_static_f64[1474]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1473]}else{0.0});
        self.scalar_static_f64[1475]=p.p464;
        self.scalar_static_f64[1476]=(self.scalar_static_f64[1470]*self.scalar_static_f64[1475]);
        self.scalar_static_f64[1477]=(1.0+self.scalar_static_f64[1476]);
        self.scalar_static_f64[1478]=p.p465;
        self.scalar_static_f64[1479]=(self.scalar_static_f64[1474]*self.scalar_static_f64[1478]);
        self.scalar_static_f64[1480]=(self.scalar_static_f64[1477]+self.scalar_static_f64[1479]);
        self.scalar_static_f64[1481]=p.p466;
        self.scalar_static_f64[1482]=(self.scalar_static_f64[1470]*self.scalar_static_f64[1481]);
        self.scalar_static_f64[1483]=(self.scalar_static_f64[1474]*self.scalar_static_f64[1482]);
        self.scalar_static_f64[1484]=(self.scalar_static_f64[1480]+self.scalar_static_f64[1483]);
        self.scalar_static_f64[1485]=p.p463;
        self.scalar_static_f64[1486]=p.p461;
        self.scalar_static_f64[1487]=(self.scalar_static_f64[1457]+self.scalar_static_f64[1461]);
        self.scalar_static_f64[1488]=(self.scalar_static_f64[1486]*self.scalar_static_f64[1487]);
        self.scalar_static_f64[1489]=p.p473;
        self.scalar_static_f64[1490]=f64::powf(self.scalar_static_f64[1462],self.scalar_static_f64[1489]);
        self.scalar_static_f64[1491]=(1.0/self.scalar_static_f64[1490]);
        self.scalar_static_f64[1492]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1491]}else{self.scalar_static_f64[1470]});
        self.scalar_static_f64[1493]=p.p474;
        self.scalar_static_f64[1494]=f64::powf(self.scalar_static_f64[1466],self.scalar_static_f64[1493]);
        self.scalar_static_f64[1495]=(1.0/self.scalar_static_f64[1494]);
        self.scalar_static_f64[1496]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1495]}else{self.scalar_static_f64[1474]});
        self.scalar_static_f64[1497]=p.p470;
        self.scalar_static_f64[1498]=(self.scalar_static_f64[1492]*self.scalar_static_f64[1497]);
        self.scalar_static_f64[1499]=(1.0+self.scalar_static_f64[1498]);
        self.scalar_static_f64[1500]=p.p471;
        self.scalar_static_f64[1501]=(self.scalar_static_f64[1496]*self.scalar_static_f64[1500]);
        self.scalar_static_f64[1502]=(self.scalar_static_f64[1499]+self.scalar_static_f64[1501]);
        self.scalar_static_f64[1503]=p.p472;
        self.scalar_static_f64[1504]=(self.scalar_static_f64[1492]*self.scalar_static_f64[1503]);
        self.scalar_static_f64[1505]=(self.scalar_static_f64[1496]*self.scalar_static_f64[1504]);
        self.scalar_static_f64[1506]=(self.scalar_static_f64[1502]+self.scalar_static_f64[1505]);
        self.scalar_static_bool[177]=(self.scalar_static_f64[1506]>1e-20);
        self.scalar_static_f64[1507]=(if self.scalar_static_bool[177]{self.scalar_static_f64[1506]}else{1e-20});
        self.scalar_static_f64[1508]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1507]}else{0.0});
        self.scalar_static_f64[1509]=p.p462;
        self.scalar_static_f64[1510]=p.p469;
        self.scalar_static_f64[1511]=p.p475;
        self.scalar_static_f64[1512]=p.p476;
        self.scalar_static_f64[1513]=f64::powf(self.scalar_static_f64[1508],self.scalar_static_f64[1512]);
        self.scalar_static_f64[1514]=(self.scalar_static_f64[464]*self.scalar_static_f64[588]);
        self.scalar_static_f64[1515]=(self.scalar_static_f64[1514]/self.scalar_static_f64[458]);
        self.scalar_static_bool[178]=(!(self.scalar_static_f64[1449]!=0.0));
        self.scalar_static_bool[179]=(self.scalar_static_bool[174]&&self.scalar_static_bool[178]);
        self.scalar_static_f64[1516]=p.p478;
        self.scalar_static_f64[1517]=(-1.0/self.scalar_static_f64[1516]);
        self.scalar_static_f64[1518]=p.p477;
        self.scalar_static_f64[1519]=(self.scalar_static_f64[375]-1.0);
        self.scalar_static_f64[1520]=(-self.scalar_static_f64[1516]);
        self.scalar_static_f64[1521]=(-self.scalar_static_f64[1455]);
        self.scalar_static_f64[1522]=(self.scalar_static_f64[1521]/self.scalar_static_f64[1518]);
        self.scalar_static_bool[180]=(self.scalar_static_f64[1522]> -80.0);
        self.scalar_static_f64[1523]=(if self.scalar_static_bool[180]{1.0}else{0.0});
        self.scalar_static_bool[181]=(self.scalar_static_bool[179]&&(self.scalar_static_f64[1523]!=0.0));
        self.scalar_static_f64[1524]=(self.scalar_static_f64[1522]).exp();
        self.scalar_static_bool[182]=(!(self.scalar_static_f64[1523]!=0.0));
        self.scalar_static_bool[183]=(self.scalar_static_bool[179]&&self.scalar_static_bool[182]);
        self.scalar_static_f64[1525]=(-self.scalar_static_f64[1522]);
        self.scalar_static_f64[1526]=(self.scalar_static_f64[1525]-80.0);
        self.scalar_static_f64[1527]=(0.5*self.scalar_static_f64[1526]);
        self.scalar_static_f64[1528]=(0.3333333333333*self.scalar_static_f64[1526]);
        self.scalar_static_f64[1529]=(1.0+self.scalar_static_f64[1528]);
        self.scalar_static_f64[1530]=(self.scalar_static_f64[1527]*self.scalar_static_f64[1529]);
        self.scalar_static_f64[1531]=(1.0+self.scalar_static_f64[1530]);
        self.scalar_static_f64[1532]=(self.scalar_static_f64[1526]*self.scalar_static_f64[1531]);
        self.scalar_static_f64[1533]=(1.0+self.scalar_static_f64[1532]);
        self.scalar_static_f64[1534]=(1.80485e-35/self.scalar_static_f64[1533]);
        self.scalar_static_f64[1535]=(-self.scalar_static_f64[1459]);
        self.scalar_static_f64[1536]=(self.scalar_static_f64[1535]/self.scalar_static_f64[1518]);
        self.scalar_static_bool[184]=(self.scalar_static_f64[1536]> -80.0);
        self.scalar_static_f64[1537]=(if self.scalar_static_bool[184]{1.0}else{0.0});
        self.scalar_static_bool[185]=(self.scalar_static_bool[179]&&(self.scalar_static_f64[1537]!=0.0));
        self.scalar_static_f64[1538]=(self.scalar_static_f64[1536]).exp();
        self.scalar_static_bool[186]=(!(self.scalar_static_f64[1537]!=0.0));
        self.scalar_static_bool[187]=(self.scalar_static_bool[179]&&self.scalar_static_bool[186]);
        self.scalar_static_f64[1539]=(-self.scalar_static_f64[1536]);
        self.scalar_static_f64[1540]=(self.scalar_static_f64[1539]-80.0);
        self.scalar_static_f64[1541]=(0.5*self.scalar_static_f64[1540]);
        self.scalar_static_f64[1542]=(0.3333333333333*self.scalar_static_f64[1540]);
        self.scalar_static_f64[1543]=(1.0+self.scalar_static_f64[1542]);
        self.scalar_static_f64[1544]=(self.scalar_static_f64[1541]*self.scalar_static_f64[1543]);
        self.scalar_static_f64[1545]=(1.0+self.scalar_static_f64[1544]);
        self.scalar_static_f64[1546]=(self.scalar_static_f64[1540]*self.scalar_static_f64[1545]);
        self.scalar_static_f64[1547]=(1.0+self.scalar_static_f64[1546]);
        self.scalar_static_f64[1548]=(1.80485e-35/self.scalar_static_f64[1547]);
        self.scalar_static_f64[1549]=(if self.scalar_static_bool[179]{self.scalar_static_f64[1465]}else{self.scalar_static_f64[1466]});
        self.scalar_static_f64[1550]=p.p482;
        self.scalar_static_f64[1551]=p.p483;
        self.scalar_static_f64[1552]=p.p480;
        self.scalar_static_f64[1553]=(self.scalar_static_f64[1549]*self.scalar_static_f64[1552]);
        self.scalar_static_f64[1554]=(self.scalar_static_f64[1553]/self.scalar_static_f64[392]);
        self.scalar_static_f64[1555]=(1.0+self.scalar_static_f64[1554]);
        self.scalar_static_bool[188]=(self.scalar_static_f64[1555]>1e-20);
        self.scalar_static_f64[1556]=(if self.scalar_static_bool[188]{self.scalar_static_f64[1555]}else{1e-20});
        self.scalar_static_f64[1557]=(if self.scalar_static_bool[179]{self.scalar_static_f64[1556]}else{self.scalar_static_f64[1508]});
        self.scalar_static_f64[1558]=p.p484;
        self.scalar_static_f64[1559]=p.p479;
        self.scalar_static_f64[1560]=p.p481;
        self.scalar_static_f64[1561]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[488]}else{self.scalar_static_f64[491]});
        self.scalar_static_f64[1562]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[888]}else{self.scalar_static_f64[891]});
        self.scalar_static_f64[1563]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[894]}else{self.scalar_static_f64[897]});
        self.scalar_static_f64[1564]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[900]}else{self.scalar_static_f64[903]});
        self.scalar_static_f64[1565]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[937]}else{self.scalar_static_f64[944]});
        self.scalar_static_f64[1566]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[946]}else{self.scalar_static_f64[948]});
        self.scalar_static_f64[1567]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[950]}else{self.scalar_static_f64[952]});
        self.scalar_static_f64[1568]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[954]}else{self.scalar_static_f64[956]});
        self.scalar_static_f64[1569]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[961]}else{self.scalar_static_f64[966]});
        self.scalar_static_f64[1570]=(self.scalar_static_f64[539]*1.04479e-10);
        self.scalar_static_f64[1571]=(self.scalar_static_f64[542]+self.scalar_static_f64[1570]);
        self.scalar_static_f64[1572]=(self.scalar_static_f64[539]* -0.4);
        self.scalar_static_f64[1573]=(10.0*self.scalar_static_f64[462]);
        self.scalar_static_f64[1574]=(self.scalar_static_f64[1573]).sqrt();
        self.scalar_static_f64[1575]=(1.0+self.scalar_static_f64[1574]);
        self.scalar_static_f64[1576]=(1.0/self.scalar_static_f64[1575]);
        self.scalar_static_f64[1577]=(self.scalar_static_f64[462]*0.05);
        self.scalar_static_f64[1578]=(self.scalar_static_f64[472]*1.602176565e-19);
        self.scalar_static_f64[1579]=(0.5*self.scalar_static_f64[1578]);
        self.scalar_static_f64[1580]=(self.scalar_static_f64[460]*self.scalar_static_f64[1579]);
        self.scalar_static_f64[1581]=(self.scalar_static_f64[1580]/3.45313e-11);
        self.scalar_static_bool[189]=(self.scalar_static_f64[468]>0.0);
        self.scalar_static_f64[1582]=(if self.scalar_static_bool[189]{1.0}else{0.0});
        self.scalar_static_f64[1583]=p.p13;
        self.scalar_static_f64[1584]=(4e-10*self.scalar_static_f64[1583]);
        self.scalar_static_f64[1585]=(self.scalar_static_f64[458]+self.scalar_static_f64[1584]);
        self.scalar_static_f64[1586]=(self.scalar_static_f64[1581]*self.scalar_static_f64[1585]);
        self.scalar_static_f64[1587]=(if (self.scalar_static_f64[1582]!=0.0){self.scalar_static_f64[1586]}else{0.0});
        self.scalar_static_f64[1588]=(self.scalar_static_f64[464]+self.scalar_static_f64[1584]);
        self.scalar_static_f64[1589]=(self.scalar_static_f64[1581]*self.scalar_static_f64[1588]);
        self.scalar_static_f64[1590]=(if (self.scalar_static_f64[1582]!=0.0){self.scalar_static_f64[1589]}else{0.0});
        self.scalar_static_bool[190]=(!(self.scalar_static_f64[1582]!=0.0));
        self.scalar_static_f64[1591]=(-self.scalar_static_f64[1581]);
        self.scalar_static_f64[1592]=(self.scalar_static_f64[1585]*self.scalar_static_f64[1591]);
        self.scalar_static_f64[1593]=(if self.scalar_static_bool[190]{self.scalar_static_f64[1592]}else{self.scalar_static_f64[1587]});
        self.scalar_static_f64[1594]=(self.scalar_static_f64[1588]*self.scalar_static_f64[1591]);
        self.scalar_static_f64[1595]=(if self.scalar_static_bool[190]{self.scalar_static_f64[1594]}else{self.scalar_static_f64[1590]});
        self.scalar_static_f64[1596]=(3.45313e-11/self.scalar_static_f64[464]);
        self.scalar_static_bool[191]=(self.scalar_static_f64[580]>0.0);
        self.scalar_static_f64[1597]=(if self.scalar_static_bool[191]{1.0}else{0.0});
        self.scalar_static_f64[1598]=(1.0+self.scalar_static_f64[580]);
        self.scalar_static_f64[1599]=(self.scalar_static_f64[1311]*self.scalar_static_f64[1598]);
        self.scalar_static_f64[1600]=(if (self.scalar_static_f64[1597]!=0.0){self.scalar_static_f64[1599]}else{0.0});
        self.scalar_static_f64[1601]=(if (self.scalar_static_f64[1597]!=0.0){self.scalar_static_f64[1596]}else{0.0});
        self.scalar_static_bool[192]=(!(self.scalar_static_f64[1597]!=0.0));
        self.scalar_static_f64[1602]=(if self.scalar_static_bool[192]{self.scalar_static_f64[1311]}else{self.scalar_static_f64[1600]});
        self.scalar_static_f64[1603]=(1.0-self.scalar_static_f64[580]);
        self.scalar_static_f64[1604]=(self.scalar_static_f64[1596]*self.scalar_static_f64[1603]);
        self.scalar_static_f64[1605]=(if self.scalar_static_bool[192]{self.scalar_static_f64[1604]}else{self.scalar_static_f64[1601]});
        self.scalar_static_f64[1606]=(self.scalar_static_f64[1571]/self.scalar_static_f64[460]);
        self.scalar_static_f64[1607]=(self.scalar_static_f64[1602]/self.scalar_static_f64[1606]);
        self.scalar_static_f64[1608]=(self.scalar_static_f64[1605]/self.scalar_static_f64[1606]);
        self.scalar_static_f64[1609]=(1.0/self.scalar_static_f64[1607]);
        self.scalar_static_f64[1610]=(1.0+self.scalar_static_f64[1609]);
        self.scalar_static_f64[1611]=(1.0/self.scalar_static_f64[1608]);
        self.scalar_static_f64[1612]=(self.scalar_static_f64[1610]+self.scalar_static_f64[1611]);
        self.scalar_static_f64[1613]=(1.0/self.scalar_static_f64[1612]);
        self.scalar_static_f64[1614]=(self.scalar_static_f64[1606]*self.scalar_static_f64[1606]);
        self.scalar_static_f64[1615]=(self.scalar_static_f64[572]*8.010882825e-20);
        self.scalar_static_f64[1616]=(self.scalar_static_f64[460]*self.scalar_static_f64[1615]);
        self.scalar_static_f64[1617]=(self.scalar_static_f64[1602]+self.scalar_static_f64[1605]);
        self.scalar_static_f64[1618]=(self.scalar_static_f64[1616]/self.scalar_static_f64[1617]);
        self.scalar_static_f64[1619]=(self.scalar_static_f64[481]*3.20435313e-19);
        self.scalar_static_f64[1620]=(1.04479e-10*self.scalar_static_f64[1619]);
        self.scalar_static_f64[1621]=p.p2;
        self.scalar_static_bool[193]=(self.scalar_static_f64[1621]>0.0);
        self.scalar_static_f64[1622]=(if self.scalar_static_bool[193]{1.0}else{0.0});
        self.scalar_static_f64[1623]=p.p9;
        self.scalar_static_bool[194]=(self.scalar_static_f64[1623]>0.0);
        self.scalar_static_f64[1624]=(if self.scalar_static_bool[194]{1.0}else{0.0});
        self.scalar_static_f64[1625]=(self.scalar_static_f64[1571]*3.20435313e-19);
        self.scalar_static_f64[1626]=(self.scalar_static_f64[534]*self.scalar_static_f64[1625]);
        self.scalar_static_f64[1627]=(self.scalar_static_f64[1626]).sqrt();
        self.scalar_static_f64[1628]=(self.scalar_static_f64[1627]/self.scalar_static_f64[1311]);
        self.scalar_static_f64[1629]=(self.scalar_static_f64[460]*1e18);
        self.scalar_static_f64[1630]=(self.scalar_static_f64[460]*self.scalar_static_f64[1629]);
        self.scalar_static_bool[195]=(self.scalar_static_f64[1583]>0.0);
        self.scalar_static_f64[1631]=(if self.scalar_static_bool[195]{1.0}else{0.0});
        self.scalar_static_f64[1632]=p.p14;
        self.scalar_static_bool[196]=(1.0==self.scalar_static_f64[1632]);
        self.scalar_static_f64[1633]=(if self.scalar_static_bool[196]{1.0}else{0.0});
        self.scalar_static_bool[197]=((self.scalar_static_f64[1631]!=0.0)&&(self.scalar_static_f64[1633]!=0.0));
        self.scalar_static_f64[1634]=(0.409618895/self.scalar_static_f64[1630]);
        self.scalar_static_f64[1635]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1634]}else{0.0});
        self.scalar_static_f64[1636]=(0.4*self.scalar_static_f64[1583]);
        self.scalar_static_f64[1637]=(self.scalar_static_f64[1636]*1.27520989);
        self.scalar_static_bool[198]=(!(self.scalar_static_f64[1633]!=0.0));
        self.scalar_static_bool[199]=((self.scalar_static_f64[1631]!=0.0)&&self.scalar_static_bool[198]);
        self.scalar_static_f64[1638]=(0.723134895/self.scalar_static_f64[1630]);
        self.scalar_static_f64[1639]=(if self.scalar_static_bool[199]{self.scalar_static_f64[1638]}else{self.scalar_static_f64[1635]});
        self.scalar_static_f64[1640]=(self.scalar_static_f64[1636]*1.5412087);
        self.scalar_static_f64[1641]=(self.scalar_static_f64[524]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1642]=p.p34;
        self.scalar_static_f64[1643]=p.p35;
        self.scalar_static_f64[1644]=(0.5*self.scalar_static_f64[737]);
        self.scalar_static_f64[1645]=(1.0/self.scalar_static_f64[1644]);
        self.scalar_static_f64[1646]=(self.scalar_static_f64[1645]/self.scalar_static_f64[739]);
        self.scalar_static_f64[1647]=(0.5*self.scalar_static_f64[768]);
        self.scalar_static_f64[1648]=(if (self.scalar_static_f64[1633]!=0.0){self.scalar_static_f64[1647]}else{0.0});
        self.scalar_static_f64[1649]=(0.3333333333333*self.scalar_static_f64[768]);
        self.scalar_static_f64[1650]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1649]}else{self.scalar_static_f64[1648]});
        self.scalar_static_f64[1651]=(1.0-self.scalar_static_f64[1650]);
        self.scalar_static_f64[1652]=(16.0/self.scalar_static_f64[840]);
        self.scalar_static_f64[1653]=(0.6931471805599*self.scalar_static_f64[1652]);
        self.scalar_static_f64[1654]=(self.scalar_static_f64[1653]).exp();
        self.scalar_static_f64[1655]=(self.scalar_static_f64[1654]-1.0);
        self.scalar_static_f64[1656]=(self.scalar_static_f64[1655]).ln();
        self.scalar_static_f64[1657]=(0.375*self.scalar_static_f64[1656]);
        self.scalar_static_f64[1658]=(self.scalar_static_f64[1657]).exp();
        self.scalar_static_f64[1659]=(self.scalar_static_f64[1658]-1.0);
        self.scalar_static_f64[1660]=(16.0/self.scalar_static_f64[1273]);
        self.scalar_static_f64[1661]=(0.6931471805599*self.scalar_static_f64[1660]);
        self.scalar_static_f64[1662]=(self.scalar_static_f64[1661]).exp();
        self.scalar_static_f64[1663]=(self.scalar_static_f64[1662]-1.0);
        self.scalar_static_f64[1664]=(self.scalar_static_f64[1663]).ln();
        self.scalar_static_f64[1665]=(0.375*self.scalar_static_f64[1664]);
        self.scalar_static_f64[1666]=(self.scalar_static_f64[1665]).exp();
        self.scalar_static_f64[1667]=(self.scalar_static_f64[1666]-1.0);
        self.scalar_static_f64[1668]=(-self.scalar_static_f64[905]);
        self.scalar_static_f64[1669]=(-self.scalar_static_f64[907]);
        self.scalar_static_f64[1670]=(1.0/self.scalar_static_f64[928]);
        self.scalar_static_f64[1671]=(self.scalar_static_f64[928]*2.9189679640027008e-49);
        self.scalar_static_f64[1672]=(self.scalar_static_f64[1671]).sqrt();
        self.scalar_static_f64[1673]=(1.3333333333332*self.scalar_static_f64[1672]);
        self.scalar_static_f64[1674]=(self.scalar_static_f64[1673]/1.054571726e-34);
        self.scalar_static_f64[1675]=(self.scalar_static_f64[485]*self.scalar_static_f64[1674]);
        self.scalar_static_bool[200]=(self.scalar_static_f64[911]<0.0);
        self.scalar_static_f64[1676]=(if self.scalar_static_bool[200]{1.0}else{0.0});
        self.scalar_static_f64[1677]=(self.scalar_static_f64[909]* -0.495);
        self.scalar_static_f64[1678]=(self.scalar_static_f64[1677]/self.scalar_static_f64[911]);
        self.scalar_static_f64[1679]=(if (self.scalar_static_f64[1676]!=0.0){self.scalar_static_f64[1678]}else{0.0});
        self.scalar_static_bool[201]=(self.scalar_static_f64[917]<0.0);
        self.scalar_static_f64[1680]=(if self.scalar_static_bool[201]{1.0}else{0.0});
        self.scalar_static_f64[1681]=(self.scalar_static_f64[913]* -0.495);
        self.scalar_static_f64[1682]=(self.scalar_static_f64[1681]/self.scalar_static_f64[917]);
        self.scalar_static_f64[1683]=(if (self.scalar_static_f64[1680]!=0.0){self.scalar_static_f64[1682]}else{0.0});
        self.scalar_static_bool[202]=(self.scalar_static_f64[921]<0.0);
        self.scalar_static_f64[1684]=(if self.scalar_static_bool[202]{1.0}else{0.0});
        self.scalar_static_f64[1685]=(self.scalar_static_f64[919]* -0.495);
        self.scalar_static_f64[1686]=(self.scalar_static_f64[1685]/self.scalar_static_f64[921]);
        self.scalar_static_f64[1687]=(if (self.scalar_static_f64[1684]!=0.0){self.scalar_static_f64[1686]}else{0.0});
        self.scalar_static_f64[1688]=(self.scalar_static_f64[485]*self.scalar_static_f64[485]);
        self.scalar_static_f64[1689]=(4e-18/self.scalar_static_f64[1688]);
        self.scalar_static_f64[1690]=(self.scalar_static_f64[937]*self.scalar_static_f64[1689]);
        self.scalar_static_f64[1691]=(self.scalar_static_f64[1565]*self.scalar_static_f64[1689]);
        self.scalar_static_f64[1692]=(self.scalar_static_f64[485]*500000000.0);
        self.scalar_static_f64[1693]=(-self.scalar_static_f64[982]);
        self.scalar_static_f64[1694]=(self.scalar_static_f64[1027]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1695]=(self.scalar_static_f64[1083]*4.0054414125e-20);
        self.scalar_static_f64[1696]=(self.scalar_static_f64[1087]*1.25e-6);
        self.scalar_static_f64[1697]=(self.scalar_static_f64[1571]/3.45313e-11);
        self.scalar_static_f64[1698]=(self.scalar_static_f64[460]*self.scalar_static_f64[1697]);
        self.scalar_static_f64[1699]=(self.scalar_static_f64[547]*self.scalar_static_f64[1698]);
        self.scalar_static_f64[1700]=(self.scalar_static_f64[1699]).sqrt();
        self.scalar_static_f64[1701]=(self.scalar_static_f64[1404]*9.10938291e-19);
        self.scalar_static_bool[203]=(self.scalar_static_f64[1424]>0.0);
        self.scalar_static_f64[1702]=(if self.scalar_static_bool[203]{1.0}else{0.0});
        self.scalar_static_f64[1703]=(1.0/self.scalar_static_f64[1424]);
        self.scalar_static_f64[1704]=(if (self.scalar_static_f64[1702]!=0.0){self.scalar_static_f64[1703]}else{0.0});
        self.scalar_static_bool[204]=(!(self.scalar_static_f64[1702]!=0.0));
        self.scalar_static_f64[1705]=(if self.scalar_static_bool[204]{0.0}else{self.scalar_static_f64[1704]});
        self.scalar_static_bool[205]=(self.scalar_static_f64[1437]>0.0);
        self.scalar_static_f64[1706]=(if self.scalar_static_bool[205]{1.0}else{0.0});
        self.scalar_static_f64[1707]=(1.0/self.scalar_static_f64[1437]);
        self.scalar_static_f64[1708]=(if (self.scalar_static_f64[1706]!=0.0){self.scalar_static_f64[1707]}else{0.0});
        self.scalar_static_bool[206]=(!(self.scalar_static_f64[1706]!=0.0));
        self.scalar_static_f64[1709]=(if self.scalar_static_bool[206]{0.0}else{self.scalar_static_f64[1708]});
        self.scalar_static_bool[207]=(self.scalar_static_f64[1441]>0.0);
        self.scalar_static_f64[1710]=(if self.scalar_static_bool[207]{1.0}else{0.0});
        self.scalar_static_f64[1711]=(1.0/self.scalar_static_f64[1441]);
        self.scalar_static_f64[1712]=(if (self.scalar_static_f64[1710]!=0.0){self.scalar_static_f64[1711]}else{0.0});
        self.scalar_static_bool[208]=(!(self.scalar_static_f64[1710]!=0.0));
        self.scalar_static_f64[1713]=(if self.scalar_static_bool[208]{0.0}else{self.scalar_static_f64[1712]});
        self.scalar_static_bool[209]=(self.scalar_static_f64[1444]>0.0);
        self.scalar_static_f64[1714]=(if self.scalar_static_bool[209]{1.0}else{0.0});
        self.scalar_static_f64[1715]=(1.0/self.scalar_static_f64[1444]);
        self.scalar_static_f64[1716]=(if (self.scalar_static_f64[1714]!=0.0){self.scalar_static_f64[1715]}else{0.0});
        self.scalar_static_bool[210]=(!(self.scalar_static_f64[1714]!=0.0));
        self.scalar_static_f64[1717]=(if self.scalar_static_bool[210]{0.0}else{self.scalar_static_f64[1716]});
        self.scalar_static_bool[211]=(self.scalar_static_f64[14]>0.0);
        self.scalar_static_f64[1718]=(if self.scalar_static_bool[211]{1.0}else{0.0});
        self.scalar_static_bool[212]=((self.scalar_static_f64[4]!=0.0)&&(self.scalar_static_f64[1718]!=0.0));
        self.scalar_static_bool[213]=(self.scalar_static_bool[1]&&(self.scalar_static_f64[1718]!=0.0));
        self.scalar_static_bool[214]=((self.scalar_static_f64[1624]!=0.0)&&(self.scalar_static_f64[1718]!=0.0));
        self.scalar_static_bool[215]=((self.scalar_static_f64[1631]!=0.0)&&(self.scalar_static_f64[1718]!=0.0));
        self.scalar_static_bool[216]=((self.scalar_static_f64[1633]!=0.0)&&self.scalar_static_bool[215]);
        self.scalar_static_bool[217]=(self.scalar_static_bool[198]&&self.scalar_static_bool[215]);
        self.scalar_static_f64[1719]=(self.scalar_static_f64[476]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1720]=(1.0+self.scalar_static_f64[1607]);
        self.scalar_static_f64[1721]=(1.0+self.scalar_static_f64[1608]);
        self.scalar_static_f64[1722]=(self.scalar_static_f64[1720]/self.scalar_static_f64[1721]);
        self.scalar_static_f64[1723]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1722]}else{0.0});
        self.scalar_static_f64[1724]=(self.scalar_static_f64[1723]).ln();
        self.scalar_static_f64[1725]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1724]}else{0.0});
        self.scalar_static_bool[218]=(self.scalar_static_f64[1725]>1e-8);
        self.scalar_static_f64[1726]=(if self.scalar_static_bool[218]{1.0}else{0.0});
        self.scalar_static_bool[219]=((self.scalar_static_f64[1622]!=0.0)&&(self.scalar_static_f64[1726]!=0.0));
        self.scalar_static_f64[1727]=(2.0*self.scalar_static_f64[1725]);
        self.scalar_static_f64[1728]=(1.0+self.scalar_static_f64[1723]);
        self.scalar_static_f64[1729]=(self.scalar_static_f64[1727]*self.scalar_static_f64[1728]);
        self.scalar_static_f64[1730]=(self.scalar_static_f64[1723]-1.0);
        self.scalar_static_f64[1731]=(self.scalar_static_f64[1729]/self.scalar_static_f64[1730]);
        self.scalar_static_f64[1732]=(if self.scalar_static_bool[219]{self.scalar_static_f64[1731]}else{0.0});
        self.scalar_static_bool[220]=(!(self.scalar_static_f64[1726]!=0.0));
        self.scalar_static_bool[221]=((self.scalar_static_f64[1622]!=0.0)&&self.scalar_static_bool[220]);
        self.scalar_static_f64[1733]=(2.0+self.scalar_static_f64[1725]);
        self.scalar_static_f64[1734]=(2.0*self.scalar_static_f64[1733]);
        self.scalar_static_f64[1735]=(if self.scalar_static_bool[221]{self.scalar_static_f64[1734]}else{self.scalar_static_f64[1732]});
        self.scalar_static_f64[1736]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1609]}else{0.0});
        self.scalar_static_f64[1737]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1611]}else{0.0});
        self.scalar_static_f64[1738]=(1.0+self.scalar_static_f64[1736]);
        self.scalar_static_f64[1739]=(self.scalar_static_f64[1737]+self.scalar_static_f64[1738]);
        self.scalar_static_f64[1740]=(1.0/self.scalar_static_f64[1739]);
        self.scalar_static_f64[1741]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1740]}else{0.0});
        self.scalar_static_f64[1742]=(1.0/self.scalar_static_f64[1720]);
        self.scalar_static_f64[1743]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1742]}else{0.0});
        self.scalar_static_f64[1744]=(1.0/self.scalar_static_f64[1721]);
        self.scalar_static_f64[1745]=(if (self.scalar_static_f64[1622]!=0.0){self.scalar_static_f64[1744]}else{0.0});
        self.scalar_static_f64[1746]=(self.scalar_static_f64[1608]*self.scalar_static_f64[1745]);
        self.scalar_static_f64[1747]=(self.scalar_static_f64[1607]+self.scalar_static_f64[1746]);
        self.scalar_static_f64[1748]=(self.scalar_static_f64[1735]*self.scalar_static_f64[1747]);
        self.scalar_static_f64[1749]=(self.scalar_static_f64[1607]*self.scalar_static_f64[1743]);
        self.scalar_static_f64[1750]=(self.scalar_static_f64[1608]+self.scalar_static_f64[1749]);
        self.scalar_static_f64[1751]=(self.scalar_static_f64[1735]*self.scalar_static_f64[1750]);
        self.scalar_static_bool[222]=(!(self.scalar_static_f64[1622]!=0.0));
        self.scalar_static_bool[223]=(!(self.scalar_static_f64[1631]!=0.0));
        self.scalar_static_bool[224]=(0.0==self.scalar_static_f64[783]);
        self.scalar_static_f64[1752]=(if self.scalar_static_bool[224]{1.0}else{0.0});
        self.scalar_static_bool[225]=(self.scalar_static_f64[783]<0.0);
        self.scalar_static_f64[1753]=(if self.scalar_static_bool[225]{1.0}else{0.0});
        self.scalar_static_bool[226]=(!(self.scalar_static_f64[1752]!=0.0));
        self.scalar_static_bool[227]=((self.scalar_static_f64[1753]!=0.0)&&self.scalar_static_bool[226]);
        self.scalar_static_bool[228]=(!(self.scalar_static_f64[1753]!=0.0));
        self.scalar_static_bool[229]=(self.scalar_static_bool[226]&&self.scalar_static_bool[228]);
        self.scalar_static_bool[230]=(self.scalar_static_f64[821]<0.0);
        self.scalar_static_f64[1754]=(if self.scalar_static_bool[230]{1.0}else{0.0});
        self.scalar_static_bool[231]=(!(self.scalar_static_f64[1754]!=0.0));
        self.scalar_static_bool[232]=(self.scalar_static_f64[823]<0.0);
        self.scalar_static_f64[1755]=(if self.scalar_static_bool[232]{1.0}else{0.0});
        self.scalar_static_bool[233]=(!(self.scalar_static_f64[1755]!=0.0));
        self.scalar_static_f64[1756]=(self.scalar_static_f64[1628]*0.25);
        self.scalar_static_f64[1757]=(self.scalar_static_f64[1628]*self.scalar_static_f64[1756]);
        self.scalar_static_f64[1758]=(0.5*self.scalar_static_f64[1628]);
        self.scalar_static_bool[234]=(self.scalar_static_f64[876]>0.0);
        self.scalar_static_f64[1759]=(if self.scalar_static_bool[234]{1.0}else{0.0});
        self.scalar_static_bool[235]=(!(self.scalar_static_f64[1759]!=0.0));
        self.scalar_static_f64[1760]=(self.scalar_static_f64[1326]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1761]=(self.scalar_static_f64[488]*3.20435313e-19);
        self.scalar_static_f64[1762]=(self.scalar_static_f64[1571]*self.scalar_static_f64[1761]);
        self.scalar_static_f64[1763]=p.p3;
        self.scalar_static_bool[236]=(self.scalar_static_f64[1763]>0.0);
        self.scalar_static_f64[1764]=p.p4;
        self.scalar_static_bool[237]=(self.scalar_static_f64[1764]>0.0);
        self.scalar_static_bool[238]=(self.scalar_static_f64[1690]>0.0);
        self.scalar_static_bool[239]=(self.scalar_static_bool[237]&&self.scalar_static_bool[238]);
        self.scalar_static_f64[1765]=(self.scalar_static_f64[1561]*3.20435313e-19);
        self.scalar_static_f64[1766]=(self.scalar_static_f64[1571]*self.scalar_static_f64[1765]);
        self.scalar_static_bool[240]=(self.scalar_static_f64[1691]>0.0);
        self.scalar_static_bool[241]=(self.scalar_static_bool[237]&&self.scalar_static_bool[240]);
        self.scalar_static_f64[1767]=(if self.scalar_static_bool[236]{1.0}else{0.0});
        self.scalar_static_f64[1768]=(-self.scalar_static_f64[915]);
        self.scalar_static_bool[242]=(0.0==self.scalar_static_f64[909]);
        self.scalar_static_bool[243]=(0.0==self.scalar_static_f64[911]);
        self.scalar_static_bool[244]=(self.scalar_static_bool[242]&&self.scalar_static_bool[243]);
        self.scalar_static_f64[1769]=(2.0*self.scalar_static_f64[911]);
        self.scalar_static_f64[1770]=(self.scalar_static_f64[954]*self.scalar_static_f64[954]);
        self.scalar_static_f64[1771]=(-self.scalar_static_f64[1690]);
        self.scalar_static_f64[1772]=(self.scalar_static_f64[1568]*self.scalar_static_f64[1568]);
        self.scalar_static_f64[1773]=(-self.scalar_static_f64[1691]);
        self.scalar_static_f64[1774]=p.p12;
        self.scalar_static_bool[245]=(self.scalar_static_f64[1774]>0.0);
        self.scalar_static_f64[1775]=(if self.scalar_static_bool[245]{1.0}else{0.0});
        self.scalar_static_f64[1776]=p.p8;
        self.scalar_static_bool[246]=(0.0!=self.scalar_static_f64[1776]);
        self.scalar_static_f64[1777]=(if self.scalar_static_bool[246]{1.0}else{0.0});
        self.scalar_static_f64[1778]=p.p16;
        self.scalar_static_f64[1779]=(100000000.0*self.scalar_static_f64[1778]);
        self.scalar_static_f64[1780]=(0.25/self.scalar_static_f64[1778]);
        self.scalar_static_f64[1781]=(self.scalar_static_f64[1778]+self.scalar_static_f64[1780]);
        self.scalar_static_f64[1782]=(-self.scalar_static_f64[1781]);
        self.scalar_static_bool[247]=(!(self.scalar_static_f64[1718]!=0.0));
        self.scalar_static_f64[1783]=(if (self.scalar_static_f64[296]!=0.0){self.scalar_static_f64[1175]}else{0.0});
        self.scalar_static_f64[1784]=(if (self.scalar_static_f64[296]!=0.0){self.scalar_static_f64[1179]}else{0.0});
        self.scalar_static_f64[1785]=(if (self.scalar_static_f64[296]!=0.0){self.scalar_static_f64[1667]}else{0.0});
        self.scalar_static_f64[1786]=(if (self.scalar_static_f64[296]!=0.0){self.scalar_static_f64[1310]}else{0.0});
        self.scalar_static_bool[248]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1622]!=0.0));
        self.scalar_static_f64[1787]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1722]}else{0.0});
        self.scalar_static_f64[1788]=(self.scalar_static_f64[1787]).ln();
        self.scalar_static_f64[1789]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1788]}else{0.0});
        self.scalar_static_bool[249]=(self.scalar_static_f64[1789]>1e-8);
        self.scalar_static_f64[1790]=(if self.scalar_static_bool[249]{1.0}else{0.0});
        self.scalar_static_bool[250]=(self.scalar_static_bool[248]&&(self.scalar_static_f64[1790]!=0.0));
        self.scalar_static_f64[1791]=(2.0*self.scalar_static_f64[1789]);
        self.scalar_static_f64[1792]=(1.0+self.scalar_static_f64[1787]);
        self.scalar_static_f64[1793]=(self.scalar_static_f64[1791]*self.scalar_static_f64[1792]);
        self.scalar_static_f64[1794]=(self.scalar_static_f64[1787]-1.0);
        self.scalar_static_f64[1795]=(self.scalar_static_f64[1793]/self.scalar_static_f64[1794]);
        self.scalar_static_f64[1796]=(if self.scalar_static_bool[250]{self.scalar_static_f64[1795]}else{0.0});
        self.scalar_static_bool[251]=(!(self.scalar_static_f64[1790]!=0.0));
        self.scalar_static_bool[252]=(self.scalar_static_bool[248]&&self.scalar_static_bool[251]);
        self.scalar_static_f64[1797]=(2.0+self.scalar_static_f64[1789]);
        self.scalar_static_f64[1798]=(2.0*self.scalar_static_f64[1797]);
        self.scalar_static_f64[1799]=(if self.scalar_static_bool[252]{self.scalar_static_f64[1798]}else{self.scalar_static_f64[1796]});
        self.scalar_static_f64[1800]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1609]}else{0.0});
        self.scalar_static_f64[1801]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1611]}else{0.0});
        self.scalar_static_f64[1802]=(1.0+self.scalar_static_f64[1800]);
        self.scalar_static_f64[1803]=(self.scalar_static_f64[1801]+self.scalar_static_f64[1802]);
        self.scalar_static_f64[1804]=(1.0/self.scalar_static_f64[1803]);
        self.scalar_static_f64[1805]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1804]}else{0.0});
        self.scalar_static_f64[1806]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1742]}else{0.0});
        self.scalar_static_f64[1807]=(if self.scalar_static_bool[248]{self.scalar_static_f64[1744]}else{0.0});
        self.scalar_static_f64[1808]=(self.scalar_static_f64[1608]*self.scalar_static_f64[1807]);
        self.scalar_static_f64[1809]=(self.scalar_static_f64[1607]+self.scalar_static_f64[1808]);
        self.scalar_static_f64[1810]=(self.scalar_static_f64[1799]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1811]=(self.scalar_static_f64[1607]*self.scalar_static_f64[1806]);
        self.scalar_static_f64[1812]=(self.scalar_static_f64[1608]+self.scalar_static_f64[1811]);
        self.scalar_static_f64[1813]=(self.scalar_static_f64[1799]*self.scalar_static_f64[1812]);
        self.scalar_static_bool[253]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[222]);
        self.scalar_static_bool[254]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1631]!=0.0));
        self.scalar_static_bool[255]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[223]);
        self.scalar_static_bool[256]=((self.scalar_static_f64[4]!=0.0)&&(self.scalar_static_f64[296]!=0.0));
        self.scalar_static_bool[257]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1752]!=0.0));
        self.scalar_static_bool[258]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[226]);
        self.scalar_static_bool[259]=((self.scalar_static_f64[1753]!=0.0)&&self.scalar_static_bool[258]);
        self.scalar_static_bool[260]=(self.scalar_static_bool[228]&&self.scalar_static_bool[258]);
        self.scalar_static_f64[1814]=(if (self.scalar_static_f64[296]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[261]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1624]!=0.0));
        self.scalar_static_bool[262]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1759]!=0.0));
        self.scalar_static_bool[263]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[235]);
        self.scalar_static_bool[264]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1754]!=0.0));
        self.scalar_static_bool[265]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[231]);
        self.scalar_static_bool[266]=((self.scalar_static_f64[296]!=0.0)&&(self.scalar_static_f64[1755]!=0.0));
        self.scalar_static_bool[267]=((self.scalar_static_f64[296]!=0.0)&&self.scalar_static_bool[233]);
        self.scalar_static_bool[268]=(!(self.scalar_static_f64[296]!=0.0));
        self.scalar_static_bool[269]=(self.scalar_static_f64[1087]>0.0);
        self.scalar_static_f64[1815]=(if self.scalar_static_bool[269]{1.0}else{0.0});
        self.scalar_static_bool[270]=(!(self.scalar_static_f64[1815]!=0.0));
        self.scalar_static_f64[1816]=(self.scalar_static_f64[385]*self.scalar_static_f64[1596]);
        self.scalar_static_f64[1817]=(self.scalar_static_f64[389]*self.scalar_static_f64[1338]);
        self.scalar_static_f64[1818]=(self.scalar_static_f64[1816]+self.scalar_static_f64[1817]);
        self.scalar_static_f64[1819]=(-self.scalar_static_f64[1818]);
        self.scalar_static_f64[1820]=(self.scalar_static_f64[383]*self.scalar_static_f64[1596]);
        self.scalar_static_f64[1821]=(self.scalar_static_f64[387]*self.scalar_static_f64[1338]);
        self.scalar_static_f64[1822]=(self.scalar_static_f64[1820]+self.scalar_static_f64[1821]);
        self.scalar_static_f64[1823]=(-self.scalar_static_f64[1822]);
        self.scalar_static_f64[1824]=p.p31;
        self.scalar_static_f64[1825]=(self.scalar_static_f64[391]*self.scalar_static_f64[1824]);
        self.scalar_static_f64[1826]=p.p32;
        self.scalar_static_f64[1827]=(self.scalar_static_f64[391]*self.scalar_static_f64[1826]);
        self.scalar_static_bool[271]=(self.scalar_static_f64[1404]>0.0);
        self.scalar_static_f64[1828]=(if self.scalar_static_bool[271]{1.0}else{0.0});
        self.scalar_static_f64[1829]=p.p6;
        self.scalar_static_bool[272]=(self.scalar_static_f64[1829]>0.0);
        self.scalar_static_f64[1830]=(if self.scalar_static_bool[272]{1.0}else{0.0});
        self.scalar_static_bool[273]=(!(self.scalar_static_f64[1830]!=0.0));
        self.scalar_static_f64[1831]=(0.0*self.scalar_static_f64[1824]);
        self.scalar_static_f64[1832]=(self.scalar_static_f64[1705]*self.scalar_static_f64[1825]);
        self.scalar_static_f64[1833]=(self.scalar_static_f64[1709]*self.scalar_static_f64[1825]);
        self.scalar_static_f64[1834]=(self.scalar_static_f64[1713]*self.scalar_static_f64[1825]);
        self.scalar_static_f64[1835]=(self.scalar_static_f64[1717]*self.scalar_static_f64[1825]);
        self.scalar_static_f64[1836]=(self.scalar_static_f64[1520]-1.0);
        self.scalar_static_f64[1837]=(if (self.scalar_static_f64[1718]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1838]=(if (self.scalar_static_f64[1718]!=0.0){self.scalar_static_f64[1837]}else{0.0});
        self.scalar_static_f64[1839]=(if (self.scalar_static_f64[1718]!=0.0){self.scalar_static_f64[1838]}else{0.0});
        self.scalar_static_f64[1840]=(self.scalar_static_f64[1]*self.scalar_static_f64[1838]);
        self.scalar_static_f64[1841]=(-self.scalar_static_f64[1840]);
        self.scalar_static_f64[1842]=(8.617332384961e-5*self.scalar_static_f64[1838]);
        self.scalar_static_f64[1843]=(if (self.scalar_static_f64[1718]!=0.0){self.scalar_static_f64[1842]}else{0.0});
        self.scalar_static_f64[1844]=(-self.scalar_static_f64[1843]);
        self.scalar_static_f64[1845]=(0.0033333333333*self.scalar_static_f64[1838]);
        self.scalar_static_f64[1846]=(self.scalar_static_f64[1641]*self.scalar_static_f64[1839]);
        self.scalar_static_f64[1847]=(self.scalar_static_f64[882]*self.scalar_static_f64[1843]);
        self.scalar_static_f64[1848]=(if (self.scalar_static_f64[1718]!=0.0){self.scalar_static_f64[1847]}else{0.0});
        self.scalar_static_f64[1849]=(self.scalar_static_f64[950]*self.scalar_static_f64[1839]);
        self.scalar_static_f64[1850]=(self.scalar_static_f64[1567]*self.scalar_static_f64[1839]);
        self.scalar_static_f64[1851]=(if (self.scalar_static_f64[1633]!=0.0){-1.0}else{0.0});
        self.scalar_static_f64[1852]=(if (self.scalar_static_f64[1633]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1853]=(if self.scalar_static_bool[198]{1.0}else{self.scalar_static_f64[1851]});
        self.scalar_static_f64[1854]=(if self.scalar_static_bool[198]{-1.0}else{self.scalar_static_f64[1852]});
        self.scalar_static_f64[1855]=(-self.scalar_static_f64[1853]);
        self.scalar_static_f64[1856]=(-self.scalar_static_f64[1854]);
        self.scalar_static_f64[1857]=(self.scalar_static_f64[1853]+self.scalar_static_f64[1855]);
        self.scalar_static_f64[1858]=(self.scalar_static_f64[1853]+self.scalar_static_f64[1854]);
        self.scalar_static_f64[1859]=(-self.scalar_static_f64[1857]);
        self.scalar_static_f64[1860]=(self.scalar_static_f64[924]*self.scalar_static_f64[1857]);
        self.scalar_static_f64[1861]=(self.scalar_static_f64[924]*self.scalar_static_f64[1856]);
        self.scalar_static_f64[1862]=(self.scalar_static_f64[924]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1863]=(self.scalar_static_f64[924]*self.scalar_static_f64[1855]);
        self.scalar_static_f64[1864]=(self.scalar_static_f64[924]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1865]=(self.scalar_static_f64[1770]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1866]=(self.scalar_static_f64[1770]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1867]=(self.scalar_static_f64[961]*self.scalar_static_f64[1855]);
        self.scalar_static_f64[1868]=(self.scalar_static_f64[961]*self.scalar_static_f64[1856]);
        self.scalar_static_f64[1869]=(self.scalar_static_f64[1771]*self.scalar_static_f64[1855]);
        self.scalar_static_f64[1870]=(self.scalar_static_f64[1771]*self.scalar_static_f64[1856]);
        self.scalar_static_f64[1871]=(self.scalar_static_f64[1772]*self.scalar_static_f64[1858]);
        self.scalar_static_f64[1872]=(self.scalar_static_f64[1772]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1873]=(self.scalar_static_f64[1772]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1874]=(self.scalar_static_f64[1569]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1875]=(self.scalar_static_f64[1569]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1876]=(self.scalar_static_f64[1773]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1877]=(self.scalar_static_f64[1773]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1878]=(self.scalar_static_f64[1819]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1879]=(self.scalar_static_f64[1819]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1880]=(self.scalar_static_f64[1823]*self.scalar_static_f64[1858]);
        self.scalar_static_f64[1881]=(self.scalar_static_f64[1823]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[1882]=(self.scalar_static_f64[1823]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[1883]=(self.scalar_static_f64[1827]*self.scalar_static_f64[1878]);
        self.scalar_static_f64[1884]=(self.scalar_static_f64[1827]*self.scalar_static_f64[1879]);
        self.scalar_static_f64[1885]=(self.scalar_static_f64[1827]*self.scalar_static_f64[1880]);
        self.scalar_static_f64[1886]=(self.scalar_static_f64[1827]*self.scalar_static_f64[1881]);
        self.scalar_static_f64[1887]=(self.scalar_static_f64[1827]*self.scalar_static_f64[1882]);
        self.scalar_static_f64[1888]=(-self.scalar_static_f64[1831]);
        self.scalar_static_f64[1889]=(-self.scalar_static_f64[1832]);
        self.scalar_static_f64[1890]=(if (self.scalar_static_f64[1702]!=0.0){self.scalar_static_f64[1832]}else{0.0});
        self.scalar_static_f64[1891]=(if (self.scalar_static_f64[1702]!=0.0){self.scalar_static_f64[1889]}else{0.0});
        self.scalar_static_f64[1892]=(-self.scalar_static_f64[1833]);
        self.scalar_static_f64[1893]=(if (self.scalar_static_f64[1706]!=0.0){self.scalar_static_f64[1833]}else{0.0});
        self.scalar_static_f64[1894]=(if (self.scalar_static_f64[1706]!=0.0){self.scalar_static_f64[1892]}else{0.0});
        self.scalar_static_f64[1895]=(-self.scalar_static_f64[1834]);
        self.scalar_static_f64[1896]=(if (self.scalar_static_f64[1710]!=0.0){self.scalar_static_f64[1834]}else{0.0});
        self.scalar_static_f64[1897]=(if (self.scalar_static_f64[1710]!=0.0){self.scalar_static_f64[1895]}else{0.0});
        self.scalar_static_f64[1898]=(-self.scalar_static_f64[1835]);
        self.scalar_static_f64[1899]=(if (self.scalar_static_f64[1714]!=0.0){self.scalar_static_f64[1835]}else{0.0});
        self.scalar_static_f64[1900]=(if (self.scalar_static_f64[1714]!=0.0){self.scalar_static_f64[1898]}else{0.0});
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
        self.scalar_static_f64[1901]=(temperature+self.scalar_static_f64[2]);
        self.scalar_static_bool[274]=(self.scalar_static_f64[1901]<1000.0);
        self.scalar_static_f64[1902]=(if self.scalar_static_bool[274]{self.scalar_static_f64[1901]}else{1000.0});
        self.scalar_static_f64[1903]=(self.scalar_static_f64[1902]*self.scalar_static_f64[6]);
        self.scalar_static_f64[1904]=(self.scalar_static_f64[5]+self.scalar_static_f64[1903]);
        self.scalar_static_f64[1905]=(self.scalar_static_f64[1902]+self.scalar_static_f64[1904]);
        self.scalar_static_f64[1906]=(self.scalar_static_f64[1902]-self.scalar_static_f64[1904]);
        self.scalar_static_f64[1907]=(self.scalar_static_f64[1906]*self.scalar_static_f64[1906]);
        self.scalar_static_f64[1908]=(self.scalar_static_f64[1907]+self.scalar_static_f64[7]);
        self.scalar_static_f64[1909]=(self.scalar_static_f64[1908]).sqrt();
        self.scalar_static_f64[1910]=(self.scalar_static_f64[1905]+self.scalar_static_f64[1909]);
        self.scalar_static_f64[1911]=(0.5*self.scalar_static_f64[1910]);
        self.scalar_static_f64[1912]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[1911]}else{0.0});
        self.scalar_static_f64[1913]=(self.scalar_static_f64[1912]*8.617332384961e-5);
        self.scalar_static_f64[1914]=(10.0/self.scalar_static_f64[1913]);
        self.scalar_static_f64[1915]=(self.scalar_static_f64[1914]+600.0);
        self.scalar_static_f64[1916]=(self.scalar_static_f64[1914]-600.0);
        self.scalar_static_f64[1917]=(self.scalar_static_f64[1916]*self.scalar_static_f64[1916]);
        self.scalar_static_f64[1918]=(self.scalar_static_f64[1917]+0.01);
        self.scalar_static_f64[1919]=(self.scalar_static_f64[1918]).sqrt();
        self.scalar_static_f64[1920]=(self.scalar_static_f64[1915]+self.scalar_static_f64[1919]);
        self.scalar_static_f64[1921]=(0.5*self.scalar_static_f64[1920]);
        self.scalar_static_f64[1922]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[1921]}else{0.0});
        self.scalar_static_f64[1923]=(self.scalar_static_f64[1902]+1.0);
        self.scalar_static_f64[1924]=(self.scalar_static_f64[1902]-1.0);
        self.scalar_static_f64[1925]=(self.scalar_static_f64[1924]*self.scalar_static_f64[1924]);
        self.scalar_static_f64[1926]=(self.scalar_static_f64[1925]+0.001);
        self.scalar_static_f64[1927]=(self.scalar_static_f64[1926]).sqrt();
        self.scalar_static_f64[1928]=(self.scalar_static_f64[1923]+self.scalar_static_f64[1927]);
        self.scalar_static_f64[1929]=(0.5*self.scalar_static_f64[1928]);
        self.scalar_static_f64[1930]=(if self.scalar_static_bool[1]{self.scalar_static_f64[1929]}else{self.scalar_static_f64[1912]});
        self.scalar_static_f64[1931]=(if self.scalar_static_bool[1]{600.0}else{self.scalar_static_f64[1922]});
        self.scalar_static_f64[1932]=(self.scalar_static_f64[1930]*self.scalar_static_f64[1930]);
        self.scalar_static_f64[1933]=(self.scalar_static_f64[1930]-self.scalar_static_f64[1]);
        self.scalar_static_f64[1934]=(self.scalar_static_f64[1930]/self.scalar_static_f64[1]);
        self.scalar_static_f64[1935]=(self.scalar_static_f64[1]/self.scalar_static_f64[1930]);
        self.scalar_static_f64[1936]=(8.617332384961e-5*self.scalar_static_f64[1930]);
        self.scalar_static_f64[1937]=(1.0/self.scalar_static_f64[1936]);
        self.scalar_static_f64[1938]=(if self.scalar_static_bool[33]{self.scalar_static_f64[444]}else{self.scalar_static_f64[1902]});
        self.scalar_static_f64[1939]=(self.scalar_static_f64[1938]/self.scalar_static_f64[392]);
        self.scalar_static_f64[1940]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1939]}else{0.0});
        self.scalar_static_f64[1941]=(if self.scalar_static_bool[33]{self.scalar_static_f64[445]}else{self.scalar_static_f64[1938]});
        self.scalar_static_f64[1942]=(self.scalar_static_f64[1941]/self.scalar_static_f64[392]);
        self.scalar_static_f64[1943]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1942]}else{0.0});
        self.scalar_static_f64[1944]=(if self.scalar_static_bool[33]{self.scalar_static_f64[501]}else{self.scalar_static_f64[1941]});
        self.scalar_static_f64[1945]=(self.scalar_static_f64[1944]+self.scalar_static_f64[502]);
        self.scalar_static_f64[1946]=(self.scalar_static_f64[1945]+self.scalar_static_f64[504]);
        self.scalar_static_f64[1947]=(self.scalar_static_f64[1946]+self.scalar_static_f64[506]);
        self.scalar_static_f64[1948]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1947]}else{self.scalar_static_f64[61]});
        self.scalar_static_f64[1949]=(self.scalar_static_f64[1944]*self.scalar_static_f64[510]);
        self.scalar_static_f64[1950]=(self.scalar_static_f64[507]+self.scalar_static_f64[1949]);
        self.scalar_static_f64[1951]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1950]}else{self.scalar_static_f64[63]});
        self.scalar_static_f64[1952]=(if self.scalar_static_bool[33]{self.scalar_static_f64[586]}else{self.scalar_static_f64[1944]});
        self.scalar_static_f64[1953]=(self.scalar_static_f64[1952]*self.scalar_static_f64[587]);
        self.scalar_static_f64[1954]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1953]}else{0.0});
        self.scalar_static_bool[275]=(self.scalar_static_f64[1954]>0.0);
        self.scalar_static_f64[1955]=(if self.scalar_static_bool[275]{self.scalar_static_f64[1954]}else{0.0});
        self.scalar_static_f64[1956]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1955]}else{self.scalar_static_f64[88]});
        self.scalar_static_f64[1957]=(self.scalar_static_f64[1956]*self.scalar_static_f64[588]);
        self.scalar_static_f64[1958]=(self.scalar_static_f64[464]*self.scalar_static_f64[1957]);
        self.scalar_static_f64[1959]=(self.scalar_static_f64[1958]/self.scalar_static_f64[458]);
        self.scalar_static_f64[1960]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1959]}else{self.scalar_static_f64[93]});
        self.scalar_static_f64[1961]=(self.scalar_static_f64[1952]*self.scalar_static_f64[589]);
        self.scalar_static_f64[1962]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1961]}else{self.scalar_static_f64[95]});
        self.scalar_static_f64[1963]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1006]}else{self.scalar_static_f64[1952]});
        self.scalar_static_f64[1964]=(self.scalar_static_f64[1963]+self.scalar_static_f64[1007]);
        self.scalar_static_f64[1965]=(self.scalar_static_f64[1964]+self.scalar_static_f64[1009]);
        self.scalar_static_f64[1966]=(self.scalar_static_f64[1965]+self.scalar_static_f64[1011]);
        self.scalar_static_f64[1967]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1966]}else{self.scalar_static_f64[246]});
        self.scalar_static_f64[1968]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1040]}else{self.scalar_static_f64[1963]});
        self.scalar_static_bool[276]=(self.scalar_static_f64[1968]>0.0);
        self.scalar_static_f64[1969]=(if self.scalar_static_bool[276]{self.scalar_static_f64[1968]}else{0.0});
        self.scalar_static_bool[277]=(self.scalar_static_f64[1969]<5.0);
        self.scalar_static_f64[1970]=(if self.scalar_static_bool[277]{self.scalar_static_f64[1969]}else{5.0});
        self.scalar_static_f64[1971]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1970]}else{self.scalar_static_f64[256]});
        self.scalar_static_f64[1972]=(self.scalar_static_f64[1971]*self.scalar_static_f64[1041]);
        self.scalar_static_f64[1973]=(self.scalar_static_f64[464]*self.scalar_static_f64[1972]);
        self.scalar_static_f64[1974]=(self.scalar_static_f64[1973]/self.scalar_static_f64[458]);
        self.scalar_static_f64[1975]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1974]}else{self.scalar_static_f64[261]});
        self.scalar_static_f64[1976]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1047]}else{self.scalar_static_f64[1968]});
        self.scalar_static_f64[1977]=(self.scalar_static_f64[1976]*self.scalar_static_f64[1048]);
        self.scalar_static_f64[1978]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1977]}else{self.scalar_static_f64[1976]});
        self.scalar_static_bool[278]=(self.scalar_static_f64[1978]>0.0);
        self.scalar_static_f64[1979]=(if self.scalar_static_bool[278]{self.scalar_static_f64[1978]}else{0.0});
        self.scalar_static_f64[1980]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1979]}else{self.scalar_static_f64[263]});
        self.scalar_static_f64[1981]=(self.scalar_static_f64[1980]*self.scalar_static_f64[1049]);
        self.scalar_static_f64[1982]=(self.scalar_static_f64[464]*self.scalar_static_f64[1981]);
        self.scalar_static_f64[1983]=(self.scalar_static_f64[1982]/self.scalar_static_f64[458]);
        self.scalar_static_f64[1984]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1983]}else{self.scalar_static_f64[268]});
        self.scalar_static_f64[1985]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1060]}else{self.scalar_static_f64[1978]});
        self.scalar_static_bool[279]=(self.scalar_static_f64[1985]>1e-15);
        self.scalar_static_f64[1986]=(if self.scalar_static_bool[279]{self.scalar_static_f64[1985]}else{1e-15});
        self.scalar_static_f64[1987]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1986]}else{self.scalar_static_f64[1985]});
        self.scalar_static_f64[1988]=(self.scalar_static_f64[423]*self.scalar_static_f64[1987]);
        self.scalar_static_f64[1989]=(self.scalar_static_f64[1061]/self.scalar_static_f64[1988]);
        self.scalar_static_f64[1990]=(self.scalar_static_f64[1989]*self.scalar_static_f64[1064]);
        self.scalar_static_f64[1991]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1990]}else{self.scalar_static_f64[272]});
        self.scalar_static_f64[1992]=(self.scalar_static_f64[1940]*self.scalar_static_f64[1080]);
        self.scalar_static_f64[1993]=(self.scalar_static_f64[1079]+self.scalar_static_f64[1992]);
        self.scalar_static_f64[1994]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1993]}else{0.0});
        self.scalar_static_bool[280]=(self.scalar_static_f64[1994]>0.0);
        self.scalar_static_f64[1995]=(if self.scalar_static_bool[280]{self.scalar_static_f64[1994]}else{0.0});
        self.scalar_static_f64[1996]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1995]}else{self.scalar_static_f64[278]});
        self.scalar_static_f64[1997]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1948]}else{self.scalar_static_f64[301]});
        self.scalar_static_f64[1998]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1951]}else{self.scalar_static_f64[306]});
        self.scalar_static_f64[1999]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1954]}else{0.0});
        self.scalar_static_f64[2000]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1956]}else{self.scalar_static_f64[320]});
        self.scalar_static_f64[2001]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1960]}else{self.scalar_static_f64[324]});
        self.scalar_static_f64[2002]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1136]}else{self.scalar_static_f64[1987]});
        self.scalar_static_f64[2003]=(self.scalar_static_f64[1100]+self.scalar_static_f64[2002]);
        self.scalar_static_f64[2004]=(self.scalar_static_f64[2003]+self.scalar_static_f64[1137]);
        self.scalar_static_f64[2005]=(self.scalar_static_f64[2004]+self.scalar_static_f64[1138]);
        self.scalar_static_f64[2006]=(if self.scalar_static_bool[80]{self.scalar_static_f64[2005]}else{self.scalar_static_f64[1997]});
        self.scalar_static_f64[2007]=(self.scalar_static_f64[2002]*self.scalar_static_f64[1150]);
        self.scalar_static_f64[2008]=(self.scalar_static_f64[1143]+self.scalar_static_f64[2007]);
        self.scalar_static_f64[2009]=(if self.scalar_static_bool[80]{self.scalar_static_f64[2008]}else{self.scalar_static_f64[1998]});
        self.scalar_static_f64[2010]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1198]}else{self.scalar_static_f64[2002]});
        self.scalar_static_f64[2011]=(self.scalar_static_f64[1184]*self.scalar_static_f64[2010]);
        self.scalar_static_f64[2012]=(if self.scalar_static_bool[80]{self.scalar_static_f64[2011]}else{self.scalar_static_f64[1999]});
        self.scalar_static_bool[281]=(self.scalar_static_f64[2012]>0.0);
        self.scalar_static_f64[2013]=(if self.scalar_static_bool[281]{self.scalar_static_f64[2012]}else{0.0});
        self.scalar_static_f64[2014]=(if self.scalar_static_bool[80]{self.scalar_static_f64[2013]}else{self.scalar_static_f64[2000]});
        self.scalar_static_f64[2015]=(self.scalar_static_f64[588]*self.scalar_static_f64[2014]);
        self.scalar_static_f64[2016]=(self.scalar_static_f64[464]*self.scalar_static_f64[2015]);
        self.scalar_static_f64[2017]=(self.scalar_static_f64[2016]/self.scalar_static_f64[458]);
        self.scalar_static_f64[2018]=(if self.scalar_static_bool[80]{self.scalar_static_f64[2017]}else{self.scalar_static_f64[2001]});
        self.scalar_static_f64[2019]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1312]}else{self.scalar_static_f64[2010]});
        self.scalar_static_f64[2020]=(self.scalar_static_f64[2019]*self.scalar_static_f64[1313]);
        self.scalar_static_f64[2021]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2020]}else{self.scalar_static_f64[341]});
        self.scalar_static_f64[2022]=(self.scalar_static_f64[2019]*self.scalar_static_f64[1314]);
        self.scalar_static_f64[2023]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2022]}else{self.scalar_static_f64[343]});
        self.scalar_static_f64[2024]=(self.scalar_static_f64[1943]*self.scalar_static_f64[1328]);
        self.scalar_static_f64[2025]=(self.scalar_static_f64[1327]+self.scalar_static_f64[2024]);
        self.scalar_static_f64[2026]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2025]}else{0.0});
        self.scalar_static_bool[282]=(self.scalar_static_f64[2026]>0.0);
        self.scalar_static_f64[2027]=(if self.scalar_static_bool[282]{self.scalar_static_f64[2026]}else{0.0});
        self.scalar_static_f64[2028]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2027]}else{self.scalar_static_f64[351]});
        self.scalar_static_f64[2029]=(self.scalar_static_f64[1943]*self.scalar_static_f64[1330]);
        self.scalar_static_f64[2030]=(self.scalar_static_f64[1329]+self.scalar_static_f64[2029]);
        self.scalar_static_f64[2031]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2030]}else{0.0});
        self.scalar_static_bool[283]=(self.scalar_static_f64[2031]>0.0);
        self.scalar_static_f64[2032]=(if self.scalar_static_bool[283]{self.scalar_static_f64[2031]}else{0.0});
        self.scalar_static_f64[2033]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2032]}else{self.scalar_static_f64[353]});
        self.scalar_static_f64[2034]=(self.scalar_static_f64[1940]*self.scalar_static_f64[1339]);
        self.scalar_static_f64[2035]=(1.0+self.scalar_static_f64[2034]);
        self.scalar_static_f64[2036]=(self.scalar_static_f64[1943]*self.scalar_static_f64[1340]);
        self.scalar_static_f64[2037]=(self.scalar_static_f64[2035]+self.scalar_static_f64[2036]);
        self.scalar_static_f64[2038]=(self.scalar_static_f64[1940]*self.scalar_static_f64[1341]);
        self.scalar_static_f64[2039]=(self.scalar_static_f64[1943]*self.scalar_static_f64[2038]);
        self.scalar_static_f64[2040]=(self.scalar_static_f64[2037]+self.scalar_static_f64[2039]);
        self.scalar_static_bool[284]=(self.scalar_static_f64[2040]>1e-10);
        self.scalar_static_f64[2041]=(if self.scalar_static_bool[284]{self.scalar_static_f64[2040]}else{1e-10});
        self.scalar_static_f64[2042]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2041]}else{self.scalar_static_f64[2019]});
        self.scalar_static_f64[2043]=(self.scalar_static_f64[2042]/self.scalar_static_f64[1389]);
        self.scalar_static_f64[2044]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2043]}else{self.scalar_static_f64[2042]});
        self.scalar_static_f64[2045]=(self.scalar_static_f64[10]/self.scalar_static_f64[2044]);
        self.scalar_static_f64[2046]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2045]}else{0.0});
        self.scalar_static_bool[285]=(self.scalar_static_f64[2046]>1e-6);
        self.scalar_static_f64[2047]=(if self.scalar_static_bool[285]{self.scalar_static_f64[2046]}else{1e-6});
        self.scalar_static_f64[2048]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2047]}else{self.scalar_static_f64[358]});
        self.scalar_static_f64[2049]=(self.scalar_static_f64[2044]*self.scalar_static_f64[1392]);
        self.scalar_static_f64[2050]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2049]}else{0.0});
        self.scalar_static_bool[286]=(self.scalar_static_f64[2050]>0.0);
        self.scalar_static_f64[2051]=(if self.scalar_static_bool[286]{self.scalar_static_f64[2050]}else{0.0});
        self.scalar_static_f64[2052]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2051]}else{self.scalar_static_f64[362]});
        self.scalar_static_f64[2053]=(self.scalar_static_f64[1934]-1.0);
        self.scalar_static_f64[2054]=(self.scalar_static_f64[1485]*self.scalar_static_f64[2053]);
        self.scalar_static_f64[2055]=(1.0+self.scalar_static_f64[2054]);
        self.scalar_static_f64[2056]=(self.scalar_static_f64[1484]*self.scalar_static_f64[2055]);
        self.scalar_static_f64[2057]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2056]}else{0.0});
        self.scalar_static_f64[2058]=(self.scalar_static_f64[1488]/self.scalar_static_f64[2057]);
        self.scalar_static_f64[2059]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2058]}else{0.0});
        self.scalar_static_f64[2060]=(1.0+self.scalar_static_f64[2059]);
        self.scalar_static_f64[2061]=(self.scalar_static_f64[2059]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[2062]=(1.0+self.scalar_static_f64[2061]);
        self.scalar_static_f64[2063]=(self.scalar_static_f64[2053]*self.scalar_static_f64[1551]);
        self.scalar_static_f64[2064]=(1.0+self.scalar_static_f64[2063]);
        self.scalar_static_f64[2065]=(self.scalar_static_f64[1550]/self.scalar_static_f64[2064]);
        self.scalar_static_f64[2066]=(if self.scalar_static_bool[179]{self.scalar_static_f64[2065]}else{0.0});
        self.scalar_static_f64[2067]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[2021]}else{self.scalar_static_f64[2023]});
        self.scalar_static_f64[2068]=(if (self.scalar_static_f64[1432]!=0.0){self.scalar_static_f64[2028]}else{self.scalar_static_f64[2033]});
        self.scalar_static_f64[2069]=(self.scalar_static_f64[1932]*0.000473);
        self.scalar_static_f64[2070]=(self.scalar_static_f64[1930]+636.0);
        self.scalar_static_f64[2071]=(self.scalar_static_f64[2069]/self.scalar_static_f64[2070]);
        self.scalar_static_f64[2072]=(1.17-self.scalar_static_f64[2071]);
        self.scalar_static_f64[2073]=(self.scalar_static_f64[1932]*0.0004774);
        self.scalar_static_f64[2074]=(self.scalar_static_f64[1930]+235.0);
        self.scalar_static_f64[2075]=(self.scalar_static_f64[2073]/self.scalar_static_f64[2074]);
        self.scalar_static_f64[2076]=(0.744-self.scalar_static_f64[2075]);
        self.scalar_static_f64[2077]=(self.scalar_static_f64[2076]-self.scalar_static_f64[2072]);
        self.scalar_static_f64[2078]=(self.scalar_static_f64[2077]+self.scalar_static_f64[1572]);
        self.scalar_static_f64[2079]=(self.scalar_static_f64[462]*self.scalar_static_f64[2078]);
        self.scalar_static_f64[2080]=(self.scalar_static_f64[2072]+self.scalar_static_f64[2079]);
        self.scalar_static_f64[2081]=(0.5*self.scalar_static_f64[2080]);
        self.scalar_static_f64[2082]=(self.scalar_static_f64[1937]*self.scalar_static_f64[2081]);
        self.scalar_static_f64[2083]=(0.5*self.scalar_static_f64[2079]);
        self.scalar_static_f64[2084]=(self.scalar_static_f64[1577]-self.scalar_static_f64[2083]);
        self.scalar_static_f64[2085]=(self.scalar_static_f64[1930]*0.0033333333333);
        self.scalar_static_f64[2086]=(self.scalar_static_f64[2085]).sqrt();
        self.scalar_static_f64[2087]=(self.scalar_static_f64[2086]*4.05e25);
        self.scalar_static_f64[2088]=(self.scalar_static_f64[2086]*self.scalar_static_f64[2087]);
        self.scalar_static_f64[2089]=(self.scalar_static_f64[2086]*self.scalar_static_f64[2088]);
        self.scalar_static_f64[2090]=(self.scalar_static_f64[1576]*self.scalar_static_f64[2089]);
        self.scalar_static_f64[2091]=(self.scalar_static_f64[1937]*self.scalar_static_f64[2083]);
        self.scalar_static_f64[2092]=(self.scalar_static_f64[2091]).exp();
        self.scalar_static_f64[2093]=(self.scalar_static_f64[2089]*self.scalar_static_f64[2092]);
        self.scalar_static_f64[2094]=(self.scalar_static_f64[1935]*self.scalar_static_f64[483]);
        self.scalar_static_f64[2095]=(1.0+self.scalar_static_f64[2094]);
        self.scalar_static_f64[2096]=(self.scalar_static_f64[1936]*self.scalar_static_f64[2095]);
        self.scalar_static_f64[2097]=(1.0/self.scalar_static_f64[2096]);
        self.scalar_static_f64[2098]=(self.scalar_static_f64[2081]*self.scalar_static_f64[2097]);
        self.scalar_static_f64[2099]=(self.scalar_static_f64[2090]*3.20435313e-19);
        self.scalar_static_f64[2100]=(self.scalar_static_f64[1571]*self.scalar_static_f64[2099]);
        self.scalar_static_f64[2101]=(self.scalar_static_f64[2097]*self.scalar_static_f64[2100]);
        self.scalar_static_f64[2102]=(self.scalar_static_f64[1614]/self.scalar_static_f64[2101]);
        self.scalar_static_f64[2103]=(self.scalar_static_f64[2102]).ln();
        self.scalar_static_f64[2104]=(self.scalar_static_f64[2103]-0.6931471805599);
        self.scalar_static_f64[2105]=(self.scalar_static_f64[2097]*self.scalar_static_f64[1618]);
        self.scalar_static_f64[2106]=(self.scalar_static_f64[1933]*self.scalar_static_f64[1962]);
        self.scalar_static_f64[2107]=(self.scalar_static_f64[591]*self.scalar_static_f64[2097]);
        self.scalar_static_f64[2108]=(self.scalar_static_f64[1937]*self.scalar_static_f64[1620]);
        self.scalar_static_f64[2109]=(self.scalar_static_f64[2108]).sqrt();
        self.scalar_static_f64[2110]=(self.scalar_static_f64[2109]/self.scalar_static_f64[1605]);
        self.scalar_static_f64[2111]=(self.scalar_static_f64[2110]*self.scalar_static_f64[2110]);
        self.scalar_static_f64[2112]=(1.0/self.scalar_static_f64[2111]);
        self.scalar_static_f64[2113]=(self.scalar_static_f64[2110]/1.4142135623731);
        self.scalar_static_f64[2114]=(1.0+self.scalar_static_f64[2113]);
        self.scalar_static_f64[2115]=(1.0/self.scalar_static_f64[2114]);
        self.scalar_static_f64[2116]=(self.scalar_static_f64[2114]*1e-5);
        self.scalar_static_f64[2117]=(self.scalar_static_f64[481]/self.scalar_static_f64[2093]);
        self.scalar_static_f64[2118]=(self.scalar_static_f64[2117]).ln();
        self.scalar_static_f64[2119]=(self.scalar_static_f64[2082]+self.scalar_static_f64[2118]);
        self.scalar_static_f64[2120]=(2.0*self.scalar_static_f64[2119]);
        self.scalar_static_f64[2121]=(self.scalar_static_f64[1936]*self.scalar_static_f64[476]);
        self.scalar_static_f64[2122]=(self.scalar_static_f64[2119]*self.scalar_static_f64[2121]);
        self.scalar_static_f64[2123]=(self.scalar_static_f64[534]/self.scalar_static_f64[2093]);
        self.scalar_static_f64[2124]=(self.scalar_static_f64[2123]).ln();
        self.scalar_static_f64[2125]=(self.scalar_static_f64[2082]+self.scalar_static_f64[2124]);
        self.scalar_static_f64[2126]=(self.scalar_static_f64[1936]*self.scalar_static_f64[2125]);
        self.scalar_static_f64[2127]=(if (self.scalar_static_f64[1624]!=0.0){self.scalar_static_f64[2126]}else{0.0});
        self.scalar_static_f64[2128]=(2970.0/self.scalar_static_f64[1930]);
        self.scalar_static_f64[2129]=(15.0+self.scalar_static_f64[2128]);
        self.scalar_static_f64[2130]=(15.0-self.scalar_static_f64[2128]);
        self.scalar_static_f64[2131]=(self.scalar_static_f64[2130]*self.scalar_static_f64[2130]);
        self.scalar_static_f64[2132]=(1e-6+self.scalar_static_f64[2131]);
        self.scalar_static_f64[2133]=(self.scalar_static_f64[2132]).sqrt();
        self.scalar_static_f64[2134]=(self.scalar_static_f64[2129]+self.scalar_static_f64[2133]);
        self.scalar_static_f64[2135]=(0.5*self.scalar_static_f64[2134]);
        self.scalar_static_f64[2136]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[2135]}else{15.0});
        self.scalar_static_f64[2137]=(self.scalar_static_f64[2096]*self.scalar_static_f64[1630]);
        self.scalar_static_f64[2138]=(self.scalar_static_f64[2137]).ln();
        self.scalar_static_f64[2139]=(-0.3333333333333*self.scalar_static_f64[2138]);
        self.scalar_static_f64[2140]=(self.scalar_static_f64[2139]).exp();
        self.scalar_static_f64[2141]=(self.scalar_static_f64[1637]*self.scalar_static_f64[2140]);
        self.scalar_static_f64[2142]=(if self.scalar_static_bool[197]{self.scalar_static_f64[2141]}else{0.0});
        self.scalar_static_f64[2143]=(self.scalar_static_f64[2140]*self.scalar_static_f64[1640]);
        self.scalar_static_f64[2144]=(if self.scalar_static_bool[199]{self.scalar_static_f64[2143]}else{self.scalar_static_f64[2142]});
        self.scalar_static_f64[2145]=(self.scalar_static_f64[1933]*self.scalar_static_f64[1641]);
        self.scalar_static_f64[2146]=(self.scalar_static_f64[1639]+self.scalar_static_f64[2145]);
        self.scalar_static_f64[2147]=(self.scalar_static_f64[2146]+self.scalar_static_f64[1642]);
        self.scalar_static_f64[2148]=(self.scalar_static_f64[2147]-self.scalar_static_f64[2127]);
        self.scalar_static_f64[2149]=(self.scalar_static_f64[1935]).ln();
        self.scalar_static_f64[2150]=(self.scalar_static_f64[696]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2151]=(self.scalar_static_f64[2150]).exp();
        self.scalar_static_f64[2152]=(self.scalar_static_f64[2151]*self.scalar_static_f64[1643]);
        self.scalar_static_f64[2153]=(self.scalar_static_f64[743]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2154]=(self.scalar_static_f64[2153]).exp();
        self.scalar_static_f64[2155]=(self.scalar_static_f64[741]*self.scalar_static_f64[2154]);
        self.scalar_static_f64[2156]=(self.scalar_static_f64[747]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2157]=(self.scalar_static_f64[2156]).exp();
        self.scalar_static_f64[2158]=(self.scalar_static_f64[745]*self.scalar_static_f64[2157]);
        self.scalar_static_f64[2159]=(self.scalar_static_f64[731]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2160]=(self.scalar_static_f64[2159]).exp();
        self.scalar_static_f64[2161]=(self.scalar_static_f64[713]*self.scalar_static_f64[2160]);
        self.scalar_static_f64[2162]=(self.scalar_static_f64[735]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2163]=(self.scalar_static_f64[2162]).exp();
        self.scalar_static_f64[2164]=(self.scalar_static_f64[733]*self.scalar_static_f64[2163]);
        self.scalar_static_f64[2165]=(self.scalar_static_f64[766]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2166]=(self.scalar_static_f64[2165]).exp();
        self.scalar_static_f64[2167]=(self.scalar_static_f64[762]*self.scalar_static_f64[2166]);
        self.scalar_static_f64[2168]=(self.scalar_static_f64[2096]*1e-8);
        self.scalar_static_f64[2169]=(self.scalar_static_f64[2168]/self.scalar_static_f64[460]);
        self.scalar_static_f64[2170]=(self.scalar_static_f64[2155]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2171]=(self.scalar_static_f64[781]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2172]=(self.scalar_static_f64[2171]).exp();
        self.scalar_static_f64[2173]=(self.scalar_static_f64[777]*self.scalar_static_f64[2172]);
        self.scalar_static_f64[2174]=(2.0*self.scalar_static_f64[2173]);
        self.scalar_static_f64[2175]=(self.scalar_static_f64[2096]*self.scalar_static_f64[2174]);
        self.scalar_static_f64[2176]=(self.scalar_static_f64[819]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2177]=(self.scalar_static_f64[2176]).exp();
        self.scalar_static_f64[2178]=(self.scalar_static_f64[874]*self.scalar_static_f64[2097]);
        self.scalar_static_f64[2179]=(self.scalar_static_f64[2149]*self.scalar_static_f64[1668]);
        self.scalar_static_f64[2180]=(self.scalar_static_f64[2179]).exp();
        self.scalar_static_f64[2181]=(self.scalar_static_f64[885]*self.scalar_static_f64[2180]);
        self.scalar_static_f64[2182]=(self.scalar_static_f64[888]*self.scalar_static_f64[2180]);
        self.scalar_static_f64[2183]=(self.scalar_static_f64[1562]*self.scalar_static_f64[2180]);
        self.scalar_static_f64[2184]=(self.scalar_static_f64[900]*self.scalar_static_f64[2180]);
        self.scalar_static_f64[2185]=(self.scalar_static_f64[1564]*self.scalar_static_f64[2180]);
        self.scalar_static_f64[2186]=(self.scalar_static_f64[2149]*self.scalar_static_f64[1669]);
        self.scalar_static_f64[2187]=(self.scalar_static_f64[2186]).exp();
        self.scalar_static_f64[2188]=(self.scalar_static_f64[894]*self.scalar_static_f64[2187]);
        self.scalar_static_f64[2189]=(self.scalar_static_f64[1563]*self.scalar_static_f64[2187]);
        self.scalar_static_f64[2190]=(self.scalar_static_f64[882]*self.scalar_static_f64[2096]);
        self.scalar_static_f64[2191]=(self.scalar_static_f64[1936]*self.scalar_static_f64[882]);
        self.scalar_static_f64[2192]=(self.scalar_static_f64[930]*self.scalar_static_f64[2098]);
        self.scalar_static_f64[2193]=(1.0+self.scalar_static_f64[2192]);
        self.scalar_static_f64[2194]=(1.0/self.scalar_static_f64[2193]);
        self.scalar_static_f64[2195]=(self.scalar_static_f64[1933]*self.scalar_static_f64[950]);
        self.scalar_static_f64[2196]=(1.0+self.scalar_static_f64[2195]);
        self.scalar_static_f64[2197]=(self.scalar_static_f64[2196]*self.scalar_static_f64[2196]);
        self.scalar_static_f64[2198]=(0.01+self.scalar_static_f64[2197]);
        self.scalar_static_f64[2199]=(self.scalar_static_f64[2198]).sqrt();
        self.scalar_static_f64[2200]=(self.scalar_static_f64[2196]+self.scalar_static_f64[2199]);
        self.scalar_static_f64[2201]=(0.5*self.scalar_static_f64[2200]);
        self.scalar_static_f64[2202]=(self.scalar_static_f64[946]*self.scalar_static_f64[2201]);
        self.scalar_static_f64[2203]=(self.scalar_static_f64[1692]*self.scalar_static_f64[2202]);
        self.scalar_static_f64[2204]=(self.scalar_static_f64[1933]*self.scalar_static_f64[1567]);
        self.scalar_static_f64[2205]=(1.0+self.scalar_static_f64[2204]);
        self.scalar_static_f64[2206]=(self.scalar_static_f64[2205]*self.scalar_static_f64[2205]);
        self.scalar_static_f64[2207]=(0.01+self.scalar_static_f64[2206]);
        self.scalar_static_f64[2208]=(self.scalar_static_f64[2207]).sqrt();
        self.scalar_static_f64[2209]=(self.scalar_static_f64[2205]+self.scalar_static_f64[2208]);
        self.scalar_static_f64[2210]=(0.5*self.scalar_static_f64[2209]);
        self.scalar_static_f64[2211]=(self.scalar_static_f64[1566]*self.scalar_static_f64[2210]);
        self.scalar_static_f64[2212]=(self.scalar_static_f64[1692]*self.scalar_static_f64[2211]);
        self.scalar_static_f64[2213]=(self.scalar_static_f64[2149]*self.scalar_static_f64[1693]);
        self.scalar_static_f64[2214]=(self.scalar_static_f64[2213]).exp();
        self.scalar_static_f64[2215]=(self.scalar_static_f64[980]*self.scalar_static_f64[2214]);
        self.scalar_static_f64[2216]=(self.scalar_static_f64[1935]*self.scalar_static_f64[1002]);
        self.scalar_static_f64[2217]=(1.0+self.scalar_static_f64[2216]);
        self.scalar_static_f64[2218]=(self.scalar_static_f64[1936]*self.scalar_static_f64[2217]);
        self.scalar_static_f64[2219]=(1.0/self.scalar_static_f64[2218]);
        self.scalar_static_f64[2220]=(self.scalar_static_f64[2100]*self.scalar_static_f64[2219]);
        self.scalar_static_f64[2221]=(self.scalar_static_f64[1933]*self.scalar_static_f64[1694]);
        self.scalar_static_f64[2222]=(self.scalar_static_f64[1639]+self.scalar_static_f64[2221]);
        self.scalar_static_f64[2223]=(self.scalar_static_f64[1967]+self.scalar_static_f64[2084]);
        self.scalar_static_f64[2224]=(self.scalar_static_f64[1593]+self.scalar_static_f64[2223]);
        self.scalar_static_f64[2225]=(self.scalar_static_f64[1632]*self.scalar_static_f64[2224]);
        self.scalar_static_f64[2226]=(self.scalar_static_f64[2222]+self.scalar_static_f64[2225]);
        self.scalar_static_f64[2227]=(self.scalar_static_f64[1642]+self.scalar_static_f64[2226]);
        self.scalar_static_f64[2228]=(self.scalar_static_f64[2227]-self.scalar_static_f64[2127]);
        self.scalar_static_f64[2229]=(self.scalar_static_f64[1013]+self.scalar_static_f64[2084]);
        self.scalar_static_f64[2230]=(self.scalar_static_f64[1595]+self.scalar_static_f64[2229]);
        self.scalar_static_f64[2231]=(self.scalar_static_f64[1632]*self.scalar_static_f64[2230]);
        self.scalar_static_f64[2232]=(self.scalar_static_f64[2222]+self.scalar_static_f64[2231]);
        self.scalar_static_f64[2233]=(self.scalar_static_f64[1076]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2234]=(self.scalar_static_f64[2233]).exp();
        self.scalar_static_f64[2235]=(self.scalar_static_f64[1643]*self.scalar_static_f64[2234]);
        self.scalar_static_f64[2236]=(self.scalar_static_f64[1991]*self.scalar_static_f64[2235]);
        self.scalar_static_f64[2237]=(self.scalar_static_f64[1078]*self.scalar_static_f64[2096]);
        self.scalar_static_f64[2238]=(self.scalar_static_f64[1571]*self.scalar_static_f64[2096]);
        self.scalar_static_f64[2239]=(self.scalar_static_f64[1695]/self.scalar_static_f64[2238]);
        self.scalar_static_f64[2240]=(self.scalar_static_f64[1083]/self.scalar_static_f64[2090]);
        self.scalar_static_f64[2241]=(self.scalar_static_f64[2240]).ln();
        self.scalar_static_f64[2242]=(self.scalar_static_f64[2096]*self.scalar_static_f64[1696]);
        self.scalar_static_f64[2243]=(self.scalar_static_f64[1391]*self.scalar_static_f64[2149]);
        self.scalar_static_f64[2244]=(self.scalar_static_f64[2243]).exp();
        self.scalar_static_f64[2245]=(self.scalar_static_f64[2048]*self.scalar_static_f64[2244]);
        self.scalar_static_f64[2246]=(self.scalar_static_f64[1930]*5.5225952e-23);
        self.scalar_static_f64[2247]=(self.scalar_static_f64[1394]*self.scalar_static_f64[2246]);
        self.scalar_static_f64[2248]=(10.0/self.scalar_static_f64[1936]);
        self.scalar_static_f64[2249]=(600.0+self.scalar_static_f64[2248]);
        self.scalar_static_f64[2250]=(self.scalar_static_f64[2248]-600.0);
        self.scalar_static_f64[2251]=(self.scalar_static_f64[2250]*self.scalar_static_f64[2250]);
        self.scalar_static_f64[2252]=(0.01+self.scalar_static_f64[2251]);
        self.scalar_static_f64[2253]=(self.scalar_static_f64[2252]).sqrt();
        self.scalar_static_f64[2254]=(self.scalar_static_f64[2249]+self.scalar_static_f64[2253]);
        self.scalar_static_f64[2255]=(0.5*self.scalar_static_f64[2254]);
        self.scalar_static_f64[2256]=(if self.scalar_static_bool[212]{self.scalar_static_f64[2255]}else{self.scalar_static_f64[1931]});
        self.scalar_static_f64[2257]=(if self.scalar_static_bool[213]{600.0}else{self.scalar_static_f64[2256]});
        self.scalar_static_f64[2258]=(if self.scalar_static_bool[212]{self.scalar_static_f64[2135]}else{self.scalar_static_f64[2136]});
        self.scalar_static_f64[2259]=(if (self.scalar_static_f64[1718]!=0.0){0.0}else{self.scalar_static_f64[2144]});
        self.scalar_static_f64[2260]=(-self.scalar_static_f64[2120]);
        self.scalar_static_f64[2261]=(self.scalar_static_f64[2260]).abs();
        self.scalar_static_bool[287]=(self.scalar_static_f64[2261]<80.0);
        self.scalar_static_f64[2262]=(if self.scalar_static_bool[287]{1.0}else{0.0});
        self.scalar_static_bool[288]=((self.scalar_static_f64[1622]!=0.0)&&(self.scalar_static_f64[2262]!=0.0));
        self.scalar_static_f64[2263]=(self.scalar_static_f64[2260]).exp();
        self.scalar_static_f64[2264]=(if self.scalar_static_bool[288]{self.scalar_static_f64[2263]}else{0.0});
        self.scalar_static_bool[289]=(self.scalar_static_f64[2260]< -80.0);
        self.scalar_static_f64[2265]=(if self.scalar_static_bool[289]{1.0}else{0.0});
        self.scalar_static_bool[290]=(!(self.scalar_static_f64[2262]!=0.0));
        self.scalar_static_bool[291]=((self.scalar_static_f64[1622]!=0.0)&&self.scalar_static_bool[290]);
        self.scalar_static_bool[292]=((self.scalar_static_f64[2265]!=0.0)&&self.scalar_static_bool[291]);
        self.scalar_static_f64[2266]=(self.scalar_static_f64[2120]-80.0);
        self.scalar_static_f64[2267]=(0.5*self.scalar_static_f64[2266]);
        self.scalar_static_f64[2268]=(0.3333333333333*self.scalar_static_f64[2266]);
        self.scalar_static_f64[2269]=(1.0+self.scalar_static_f64[2268]);
        self.scalar_static_f64[2270]=(self.scalar_static_f64[2267]*self.scalar_static_f64[2269]);
        self.scalar_static_f64[2271]=(1.0+self.scalar_static_f64[2270]);
        self.scalar_static_f64[2272]=(self.scalar_static_f64[2266]*self.scalar_static_f64[2271]);
        self.scalar_static_f64[2273]=(1.0+self.scalar_static_f64[2272]);
        self.scalar_static_f64[2274]=(1.80485e-35/self.scalar_static_f64[2273]);
        self.scalar_static_f64[2275]=(if self.scalar_static_bool[292]{self.scalar_static_f64[2274]}else{self.scalar_static_f64[2264]});
        self.scalar_static_bool[293]=(!(self.scalar_static_f64[2265]!=0.0));
        self.scalar_static_bool[294]=(self.scalar_static_bool[291]&&self.scalar_static_bool[293]);
        self.scalar_static_f64[2276]=(self.scalar_static_f64[2260]-80.0);
        self.scalar_static_f64[2277]=(0.5*self.scalar_static_f64[2276]);
        self.scalar_static_f64[2278]=(0.3333333333333*self.scalar_static_f64[2276]);
        self.scalar_static_f64[2279]=(1.0+self.scalar_static_f64[2278]);
        self.scalar_static_f64[2280]=(self.scalar_static_f64[2277]*self.scalar_static_f64[2279]);
        self.scalar_static_f64[2281]=(1.0+self.scalar_static_f64[2280]);
        self.scalar_static_f64[2282]=(self.scalar_static_f64[2276]*self.scalar_static_f64[2281]);
        self.scalar_static_f64[2283]=(1.0+self.scalar_static_f64[2282]);
        self.scalar_static_f64[2284]=(5.54062e34*self.scalar_static_f64[2283]);
        self.scalar_static_f64[2285]=(if self.scalar_static_bool[294]{self.scalar_static_f64[2284]}else{self.scalar_static_f64[2275]});
        self.scalar_static_f64[2286]=(self.scalar_static_f64[2115]*self.scalar_static_f64[2115]);
        self.scalar_static_f64[2287]=(self.scalar_static_f64[2286]*0.1666666666667);
        self.scalar_static_f64[2288]=(self.scalar_static_f64[2287]/1.4142135623731);
        self.scalar_static_f64[2289]=(1.0-self.scalar_static_f64[2285]);
        self.scalar_static_f64[2290]=(-self.scalar_static_f64[2116]);
        self.scalar_static_f64[2291]=(self.scalar_static_f64[2110]*0.732464877560822);
        self.scalar_static_f64[2292]=(1.25+self.scalar_static_f64[2291]);
        self.scalar_static_f64[2293]=(1.0/self.scalar_static_f64[2292]);
        self.scalar_static_f64[2294]=(self.scalar_static_f64[2114]*1.25);
        self.scalar_static_f64[2295]=(0.5*self.scalar_static_f64[2111]);
        self.scalar_static_f64[2296]=(self.scalar_static_f64[2111]*0.25);
        self.scalar_static_f64[2297]=(self.scalar_static_f64[2120]+3.0);
        self.scalar_static_f64[2298]=(self.scalar_static_f64[2258]*self.scalar_static_f64[2258]);
        self.scalar_static_bool[295]=(self.scalar_static_f64[2021]>0.0);
        self.scalar_static_f64[2299]=(if self.scalar_static_bool[295]{1.0}else{0.0});
        self.scalar_static_bool[296]=(self.scalar_static_f64[2067]>0.0);
        self.scalar_static_f64[2300]=(if self.scalar_static_bool[296]{1.0}else{0.0});
        self.scalar_static_f64[2301]=(1.0+self.scalar_static_f64[1971]);
        self.scalar_static_f64[2302]=(1.0/self.scalar_static_f64[2301]);
        self.scalar_static_f64[2303]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2302]}else{0.0});
        self.scalar_static_f64[2304]=(1.0+self.scalar_static_f64[1975]);
        self.scalar_static_f64[2305]=(1.0/self.scalar_static_f64[2304]);
        self.scalar_static_f64[2306]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2305]}else{0.0});
        self.scalar_static_f64[2307]=(self.scalar_static_f64[1051]*self.scalar_static_f64[2219]);
        self.scalar_static_f64[2308]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2307]}else{0.0});
        self.scalar_static_f64[2309]=(2.0*self.scalar_static_f64[2308]);
        self.scalar_static_f64[2310]=(self.scalar_static_f64[1607]/self.scalar_static_f64[2303]);
        self.scalar_static_f64[2311]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2310]}else{0.0});
        self.scalar_static_f64[2312]=(self.scalar_static_f64[1608]/self.scalar_static_f64[2306]);
        self.scalar_static_f64[2313]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2312]}else{0.0});
        self.scalar_static_f64[2314]=(1.0/self.scalar_static_f64[2311]);
        self.scalar_static_f64[2315]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2314]}else{0.0});
        self.scalar_static_f64[2316]=(1.0/self.scalar_static_f64[2313]);
        self.scalar_static_f64[2317]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2316]}else{0.0});
        self.scalar_static_f64[2318]=(1.0+self.scalar_static_f64[2315]);
        self.scalar_static_f64[2319]=(self.scalar_static_f64[2317]+self.scalar_static_f64[2318]);
        self.scalar_static_f64[2320]=(1.0/self.scalar_static_f64[2319]);
        self.scalar_static_f64[2321]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2320]}else{0.0});
        self.scalar_static_f64[2322]=(self.scalar_static_f64[2315]*self.scalar_static_f64[2321]);
        self.scalar_static_f64[2323]=(1.0-self.scalar_static_f64[2322]);
        self.scalar_static_f64[2324]=(self.scalar_static_f64[2317]*self.scalar_static_f64[2321]);
        self.scalar_static_f64[2325]=(self.scalar_static_f64[2323]-self.scalar_static_f64[2324]);
        self.scalar_static_f64[2326]=(0.5*self.scalar_static_f64[2315]);
        self.scalar_static_f64[2327]=(self.scalar_static_f64[2321]*self.scalar_static_f64[2326]);
        self.scalar_static_f64[2328]=(self.scalar_static_f64[2315]*self.scalar_static_f64[2327]);
        self.scalar_static_f64[2329]=(self.scalar_static_f64[2317]+self.scalar_static_f64[2328]);
        self.scalar_static_f64[2330]=(0.5*self.scalar_static_f64[2317]);
        self.scalar_static_f64[2331]=(self.scalar_static_f64[2321]*self.scalar_static_f64[2330]);
        self.scalar_static_f64[2332]=(self.scalar_static_f64[2317]*self.scalar_static_f64[2331]);
        self.scalar_static_f64[2333]=(self.scalar_static_f64[2329]-self.scalar_static_f64[2332]);
        self.scalar_static_f64[2334]=(0.5/self.scalar_static_f64[2321]);
        self.scalar_static_f64[2335]=(self.scalar_static_f64[2333]-self.scalar_static_f64[2334]);
        self.scalar_static_f64[2336]=(-self.scalar_static_f64[2315]);
        self.scalar_static_f64[2337]=(1.0/self.scalar_static_f64[2321]);
        self.scalar_static_f64[2338]=(self.scalar_static_f64[2317]-self.scalar_static_f64[2337]);
        self.scalar_static_f64[2339]=(self.scalar_static_f64[2218]*self.scalar_static_f64[2218]);
        self.scalar_static_f64[2340]=(self.scalar_static_f64[2236]*self.scalar_static_f64[2339]);
        self.scalar_static_f64[2341]=(if (self.scalar_static_f64[1775]!=0.0){self.scalar_static_f64[2340]}else{0.0});
        self.scalar_static_f64[2342]=(self.scalar_static_f64[1602]*self.scalar_static_f64[2341]);
        self.scalar_static_bool[297]=((self.scalar_static_f64[2262]!=0.0)&&self.scalar_static_bool[248]);
        self.scalar_static_f64[2343]=(if self.scalar_static_bool[297]{self.scalar_static_f64[2263]}else{0.0});
        self.scalar_static_bool[298]=(self.scalar_static_bool[290]&&self.scalar_static_bool[248]);
        self.scalar_static_bool[299]=((self.scalar_static_f64[2265]!=0.0)&&self.scalar_static_bool[298]);
        self.scalar_static_f64[2344]=(if self.scalar_static_bool[299]{self.scalar_static_f64[2274]}else{self.scalar_static_f64[2343]});
        self.scalar_static_bool[300]=(self.scalar_static_bool[293]&&self.scalar_static_bool[298]);
        self.scalar_static_f64[2345]=(if self.scalar_static_bool[300]{self.scalar_static_f64[2284]}else{self.scalar_static_f64[2344]});
        self.scalar_static_f64[2346]=(1.0-self.scalar_static_f64[2345]);
        self.scalar_static_f64[2347]=(self.scalar_static_f64[1962]*self.scalar_static_f64[1839]);
        self.scalar_static_f64[2348]=(self.scalar_static_f64[2125]*self.scalar_static_f64[1843]);
        self.scalar_static_f64[2349]=(if self.scalar_static_bool[214]{self.scalar_static_f64[2348]}else{0.0});
        self.scalar_static_f64[2350]=(self.scalar_static_f64[2028]*self.scalar_static_f64[1853]);
        self.scalar_static_f64[2351]=(self.scalar_static_f64[2028]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[2352]=(self.scalar_static_f64[2068]*self.scalar_static_f64[1857]);
        self.scalar_static_f64[2353]=(self.scalar_static_f64[2068]*self.scalar_static_f64[1856]);
        self.scalar_static_f64[2354]=(self.scalar_static_f64[2068]*self.scalar_static_f64[1854]);
        self.scalar_static_f64[2355]=(self.scalar_static_f64[2052]*self.scalar_static_f64[1837]);
        self.scalar_static_f64[2356]=(if (self.scalar_static_f64[1718]!=0.0){self.scalar_static_f64[2355]}else{0.0});
        self.scalar_static_f64[2357]=(if self.scalar_static_bool[247]{0.0}else{self.scalar_static_f64[2356]});
        self.scalar_static_f64[2358]=(self.scalar_static_f64[1827]*self.scalar_static_f64[2350]);
        self.scalar_static_f64[2359]=(self.scalar_static_f64[1827]*self.scalar_static_f64[2351]);
        self.scalar_static_f64[2360]=(self.scalar_static_f64[1827]*self.scalar_static_f64[2352]);
        self.scalar_static_f64[2361]=(self.scalar_static_f64[1827]*self.scalar_static_f64[2353]);
        self.scalar_static_f64[2362]=(self.scalar_static_f64[1827]*self.scalar_static_f64[2354]);
        self.scalar_static_f64[2363]=(self.scalar_static_f64[391]*self.scalar_static_f64[2357]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
