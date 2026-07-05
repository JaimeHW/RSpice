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
    pub p280: f64, pub p281: f64, pub p282: f64, pub p283: f64, pub p284: f64, pub p285: f64, pub p286: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 8] = [
                27.0, 2.5e-8, 1.64e-6, 2.5e-7, 0.0002, 1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 8);
            {
                let params = &mut *ptr;
                params.p8 = params.p6;
                validate_parameter("mult_fn", params.p8, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 278] = [
                1.066e-10, -2.0, 0.0, 0.0, 0.17, 0.0, 0.0, 0.0,
                190000.0, 2.0, 0.0, -0.5, 0.0, 1e-9, 5.0, 0.0,
                1.0, 0.5, 0.001, 2.12e-12, 3.73e-12, 1e-15, 1.0, 5.0,
                1e-9, 0.0, 50000.0, 5e17, 5e17, 0.0, 0.0, 0.155,
                0.155, 2.0, 2.0, 1.0, 1.0, 1e-6, 1e-6, 0.0001,
                0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                2.5, 1.0, 1.0, 2.5, 80.0, 80.0, 1e-12, 1e-12,
                1e-15, 1e-15, 0.0001, 0.0001, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1e-5, 1.0, 0.0, 0.5, 0.0, 0.5,
                20.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-6, 1000000.0, 1.0, 0.1, 0.3, 0.0, 0.05, 0.1,
                0.6, 0.5, 0.6, 1.0, 1.0, 1e-5, 1e-6, 0.1,
                1e-9, 1e-15, 1e-15, 1e-12, 1e-13, 1e-13, 1.0, 0.0001,
                10.0, 1.0, 0.016, 2.0, 20.0, 1.0, 250.0, 0.01,
                0.0, 0.0, 0.05, 0.0, 0.0, 10000.0, 1e-7, 0.5,
                0.5, 0.0, 0.0, 0.05, 0.0, 0.0, 10000.0, 1e-7,
                0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e-15, -25.0, 5e-8,
                1e-6, 0.05, 0.1, 100000.0, 0.5, 0.0, 1e-9, 10.0,
                2.12e-12, 3.73e-12, 1e-15, -80.0, 1e-7, 1e-6, 0.05, 0.1,
                100000.0, 0.5, 0.0, 1e-9, 10.0, 2.12e-12, 3.73e-12, 1e-15,
                -75.0, 1.5e-7, 1e-6, 0.05, 0.1, 100000.0, 0.5, 0.0,
                1e-9, 10.0, 2.12e-12, 3.73e-12, 1e-15, -100.0, 2e-7, 1e-6,
                0.05, 0.1, 100000.0, 0.5, 0.0, 1e-9, 10.0, 2.12e-12,
                3.73e-12, 1e-14, 1e-14, 1e-14, 0.0, 100.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-24, 0.0, 0.0, 0.9,
                0.0, 0.0, 0.0, 0.5, 0.1, 1.0, 0.0, 1.0,
                0.001, 0.0, 1.0, 0.001, 0.0, 1.0, 0.001, 0.0,
                1.0, 0.001, 0.0, 1.0, 0.001, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.001, 0.0, 0.0, 1.5e-11, 0.0, 0.0, 1.0,
                1e27, 1e-12, 0.0, 200.0, 0.0, 10.0, 0.0, 0.0,
                0.0, 0.0, 100.0, 100.0, 0.0, 0.0, 50.0, 50.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(9), 278);
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
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
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
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 287] = [
    ("tnom", 0), ("tbar", 1), ("tepi", 2), ("l", 3), ("w", 4), ("nf", 5), ("mult_i", 6), ("mult_q", 7), ("mult_fn", 8), ("epsilon", 9), ("voff", 10), ("asub", 11), ("ksub", 12), ("u0", 13), ("ua", 14), ("ub", 15),
    ("uc", 16), ("vsat", 17), ("delta", 18), ("at", 19), ("ute", 20), ("lambda", 21), ("eta0", 22), ("vdscale", 23), ("kt1", 24), ("thesat", 25), ("nfactor", 26), ("cdscd", 27), ("gamma0i", 28), ("gamma1i", 29), ("imin", 30), ("shmod", 31),
    ("rth0", 32), ("cth0", 33), ("rdsmod", 34), ("vsataccs", 35), ("ns0accs", 36), ("ns0accd", 37), ("k0accs", 38), ("k0accd", 39), ("u0accs", 40), ("u0accd", 41), ("mexpaccs", 42), ("mexpaccd", 43), ("ard", 44), ("ars", 45), ("lsg", 46), ("ldg", 47),
    ("rsc", 48), ("rdc", 49), ("kns0", 50), ("ats", 51), ("utes", 52), ("uted", 53), ("krsc", 54), ("krdc", 55), ("gatemod", 56), ("njgs", 57), ("ags", 58), ("agd", 59), ("njgd", 60), ("rnjgs", 61), ("rnjgd", 62), ("igsdio", 63),
    ("igddio", 64), ("rigsdio", 65), ("rigddio", 66), ("vbis", 67), ("vbid", 68), ("ebreaks", 69), ("ebreakd", 70), ("ktgs", 71), ("ktgd", 72), ("rktgs", 73), ("rktgd", 74), ("ktvbis", 75), ("ktvbid", 76), ("ktnjgs", 77), ("ktnjgd", 78), ("ktrnjgs", 79),
    ("ktrnjgd", 80), ("trapmod", 81), ("remi", 82), ("cglag", 83), ("remig", 84), ("arcap", 85), ("brcap", 86), ("arcapg", 87), ("brcapg", 88), ("vdlmax", 89), ("vglmax", 90), ("dlvoff", 91), ("glvoff", 92), ("glu0", 93), ("glvsat", 94), ("dlns0s", 95),
    ("dlns0d", 96), ("cdlag", 97), ("rdlag", 98), ("idio", 99), ("atrapvoff", 100), ("btrapvoff", 101), ("atrapeta0", 102), ("btrapeta0", 103), ("atraprs", 104), ("btraprs", 105), ("atraprd", 106), ("btraprd", 107), ("rtrap1", 108), ("rtrap2", 109), ("ctrap1", 110), ("ctrap2", 111),
    ("a1", 112), ("vofftr", 113), ("cdscdtr", 114), ("eta0tr", 115), ("rontr1", 116), ("rontr2", 117), ("rontr3", 118), ("rtrap3", 119), ("ctrap3", 120), ("vatrap", 121), ("sct", 122), ("wd", 123), ("vdlr1", 124), ("vdlr2", 125), ("talpha", 126), ("vtb", 127),
    ("deltax", 128), ("alphax", 129), ("alphaxd", 130), ("betax", 131), ("gammax", 132), ("etax", 133), ("eno", 134), ("cx", 135), ("vxmax", 136), ("ea", 137), ("alphay", 138), ("alphayd", 139), ("betay", 140), ("gammay", 141), ("etay", 142), ("eno1", 143),
    ("cy", 144), ("vymax", 145), ("ea1", 146), ("glns0s", 147), ("glns0d", 148), ("fastfpmod", 149), ("fp1mod", 150), ("fp1smod", 151), ("fp2mod", 152), ("fp2smod", 153), ("fp3mod", 154), ("fp3smod", 155), ("fp4mod", 156), ("fp4smod", 157), ("iminfp1", 158), ("vofffp1", 159),
    ("dfp1", 160), ("lfp1", 161), ("ktfp1", 162), ("u0fp1", 163), ("vsatfp1", 164), ("nfactorfp1", 165), ("cdscdfp1", 166), ("eta0fp1", 167), ("vdscalefp1", 168), ("gamma0fp1", 169), ("gamma1fp1", 170), ("iminfp2", 171), ("vofffp2", 172), ("dfp2", 173), ("lfp2", 174), ("ktfp2", 175),
    ("u0fp2", 176), ("vsatfp2", 177), ("nfactorfp2", 178), ("cdscdfp2", 179), ("eta0fp2", 180), ("vdscalefp2", 181), ("gamma0fp2", 182), ("gamma1fp2", 183), ("iminfp3", 184), ("vofffp3", 185), ("dfp3", 186), ("lfp3", 187), ("ktfp3", 188), ("u0fp3", 189), ("vsatfp3", 190), ("nfactorfp3", 191),
    ("cdscdfp3", 192), ("eta0fp3", 193), ("vdscalefp3", 194), ("gamma0fp3", 195), ("gamma1fp3", 196), ("iminfp4", 197), ("vofffp4", 198), ("dfp4", 199), ("lfp4", 200), ("ktfp4", 201), ("u0fp4", 202), ("vsatfp4", 203), ("nfactorfp4", 204), ("cdscdfp4", 205), ("eta0fp4", 206), ("vdscalefp4", 207),
    ("gamma0fp4", 208), ("gamma1fp4", 209), ("cgso", 210), ("cgdo", 211), ("cdso", 212), ("cgdl", 213), ("vdsatcv", 214), ("cbdo", 215), ("cbso", 216), ("cbgo", 217), ("cfg", 218), ("cfd", 219), ("cfgd", 220), ("cfgdsm", 221), ("cfgd0", 222), ("cj0", 223),
    ("vbi", 224), ("ktvbi", 225), ("ktcfg", 226), ("ktcfgd", 227), ("mz", 228), ("aj", 229), ("dj", 230), ("adosi", 231), ("bdosi", 232), ("qm0i", 233), ("adosfp1", 234), ("bdosfp1", 235), ("qm0fp1", 236), ("adosfp2", 237), ("bdosfp2", 238), ("qm0fp2", 239),
    ("adosfp3", 240), ("bdosfp3", 241), ("qm0fp3", 242), ("adosfp4", 243), ("bdosfp4", 244), ("qm0fp4", 245), ("cfp1scale", 246), ("cfp2scale", 247), ("cfp3scale", 248), ("cfp4scale", 249), ("csubscalei", 250), ("csubscale1", 251), ("csubscale2", 252), ("csubscale3", 253), ("csubscale4", 254), ("rgatemod", 255),
    ("xgw", 256), ("ngcon", 257), ("rshg", 258), ("fnmod", 259), ("tnmod", 260), ("noia", 261), ("noib", 262), ("noic", 263), ("ef", 264), ("tnsc", 265), ("gdsmin", 266), ("tgdsmin", 267), ("bvdsl", 268), ("asl", 269), ("nsl", 270), ("kasl", 271),
    ("knsl", 272), ("kbvdsl", 273), ("dtemp", 274), ("nsb", 275), ("ndb", 276), ("isbl", 277), ("idbl", 278), ("vbisb", 279), ("vbidb", 280), ("ktisb", 281), ("ktidb", 282), ("ktnsb", 283), ("ktndb", 284), ("ktvbisb", 285), ("ktvbidb", 286),
];

const PARAMETER_DISPLAY_NAMES: [&str; 287] = [
    "tnom", "tbar", "tepi", "l", "w", "nf", "mult_i", "mult_q", "mult_fn", "epsilon", "voff", "asub", "ksub", "u0", "ua", "ub",
    "uc", "vsat", "delta", "at", "ute", "lambda", "eta0", "vdscale", "kt1", "thesat", "nfactor", "cdscd", "gamma0i", "gamma1i", "imin", "shmod",
    "rth0", "cth0", "rdsmod", "vsataccs", "ns0accs", "ns0accd", "k0accs", "k0accd", "u0accs", "u0accd", "mexpaccs", "mexpaccd", "ard", "ars", "lsg", "ldg",
    "rsc", "rdc", "kns0", "ats", "utes", "uted", "krsc", "krdc", "gatemod", "njgs", "ags", "agd", "njgd", "rnjgs", "rnjgd", "igsdio",
    "igddio", "rigsdio", "rigddio", "vbis", "vbid", "ebreaks", "ebreakd", "ktgs", "ktgd", "rktgs", "rktgd", "ktvbis", "ktvbid", "ktnjgs", "ktnjgd", "ktrnjgs",
    "ktrnjgd", "trapmod", "remi", "cglag", "remig", "arcap", "brcap", "arcapg", "brcapg", "vdlmax", "vglmax", "dlvoff", "glvoff", "glu0", "glvsat", "dlns0s",
    "dlns0d", "cdlag", "rdlag", "idio", "atrapvoff", "btrapvoff", "atrapeta0", "btrapeta0", "atraprs", "btraprs", "atraprd", "btraprd", "rtrap1", "rtrap2", "ctrap1", "ctrap2",
    "a1", "vofftr", "cdscdtr", "eta0tr", "rontr1", "rontr2", "rontr3", "rtrap3", "ctrap3", "vatrap", "sct", "wd", "vdlr1", "vdlr2", "talpha", "vtb",
    "deltax", "alphax", "alphaxd", "betax", "gammax", "etax", "eno", "cx", "vxmax", "ea", "alphay", "alphayd", "betay", "gammay", "etay", "eno1",
    "cy", "vymax", "ea1", "glns0s", "glns0d", "fastfpmod", "fp1mod", "fp1smod", "fp2mod", "fp2smod", "fp3mod", "fp3smod", "fp4mod", "fp4smod", "iminfp1", "vofffp1",
    "dfp1", "lfp1", "ktfp1", "u0fp1", "vsatfp1", "nfactorfp1", "cdscdfp1", "eta0fp1", "vdscalefp1", "gamma0fp1", "gamma1fp1", "iminfp2", "vofffp2", "dfp2", "lfp2", "ktfp2",
    "u0fp2", "vsatfp2", "nfactorfp2", "cdscdfp2", "eta0fp2", "vdscalefp2", "gamma0fp2", "gamma1fp2", "iminfp3", "vofffp3", "dfp3", "lfp3", "ktfp3", "u0fp3", "vsatfp3", "nfactorfp3",
    "cdscdfp3", "eta0fp3", "vdscalefp3", "gamma0fp3", "gamma1fp3", "iminfp4", "vofffp4", "dfp4", "lfp4", "ktfp4", "u0fp4", "vsatfp4", "nfactorfp4", "cdscdfp4", "eta0fp4", "vdscalefp4",
    "gamma0fp4", "gamma1fp4", "cgso", "cgdo", "cdso", "cgdl", "vdsatcv", "cbdo", "cbso", "cbgo", "cfg", "cfd", "cfgd", "cfgdsm", "cfgd0", "cj0",
    "vbi", "ktvbi", "ktcfg", "ktcfgd", "mz", "aj", "dj", "adosi", "bdosi", "qm0i", "adosfp1", "bdosfp1", "qm0fp1", "adosfp2", "bdosfp2", "qm0fp2",
    "adosfp3", "bdosfp3", "qm0fp3", "adosfp4", "bdosfp4", "qm0fp4", "cfp1scale", "cfp2scale", "cfp3scale", "cfp4scale", "csubscalei", "csubscale1", "csubscale2", "csubscale3", "csubscale4", "rgatemod",
    "xgw", "ngcon", "rshg", "fnmod", "tnmod", "noia", "noib", "noic", "ef", "tnsc", "gdsmin", "tgdsmin", "bvdsl", "asl", "nsl", "kasl",
    "knsl", "kbvdsl", "dtemp", "nsb", "ndb", "isbl", "idbl", "vbisb", "vbidb", "ktisb", "ktidb", "ktnsb", "ktndb", "ktvbisb", "ktvbidb",
];

const PARAMETER_INTEGER_FLAGS: [bool; 287] = [
    false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true,
    false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 287] = [
    Some(ParameterBound { value: -273.15, label: "-273.15" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 2e-8, label: "2e-8" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1000.0, label: "1000.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 100000.0, label: "100000.0" }), Some(ParameterBound { value: 100000.0, label: "100000.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -500.0, label: "-500.0" }),
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -500.0, label: "-500.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -500.0, label: "-500.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 287] = [
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 50.0, label: "50.0" }), Some(ParameterBound { value: 50.0, label: "50.0" }), Some(ParameterBound { value: 50.0, label: "50.0" }), Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 5.0, label: "5.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 5.0, label: "5.0" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }),
    None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 5000.0, label: "5000.0" }), Some(ParameterBound { value: 5000.0, label: "5000.0" }), None, None, None,
    None, None, None, None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 287] = [
    2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 0, 0, 3, 2, 2, 2, 2, 2, 2, 3, 0, 2, 2, 3, 3, 2, 2, 2, 0, 0, 3, 0,
    2, 2, 0, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 0, 3, 3, 3, 3, 3, 3, 2,
    2, 2, 2, 3, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 2, 3, 3, 3, 3, 3, 3, 2,
    2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    2, 3, 3, 2, 2, 2, 2, 2, 3, 0, 0, 3, 0, 2, 3, 3, 2, 2, 2, 2, 2, 3, 0, 0, 3, 0, 2, 3, 3, 2, 2, 2,
    2, 2, 3, 0, 0, 3, 0, 2, 3, 3, 2, 2, 2, 2, 2, 3, 0, 0, 2, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 2, 2, 2, 3, 3, 2, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0,
    2, 0, 2, 0, 0, 3, 2, 2, 3, 3, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 287] = [
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
    &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 23],
    pub branches: [usize; 57],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 287]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 121]>,
    pub(crate) ddt_state_previous: Box<[f64; 121]>,
    pub(crate) ddt_state_older: Box<[f64; 121]>,
    pub(crate) ddt_state_initialized: Box<[bool; 121]>,
    pub(crate) ddt_derivative_current: Box<[f64; 121]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 121]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 939]>,
    pub(crate) scalar_static_bool: Box<[bool; 169]>,
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
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 18;
    pub const NODE_COUNT: usize = 23;
    pub const INTERNAL_NODE_NAMES: [&str; 18] = ["trap1", "trap2", "di", "si", "gi", "gin", "n1", "nt", "n2", "ntg", "fp1", "fp2", "fp3", "fp4", "fp1s", "fp2s", "fp3s", "fp4s"];

    pub const BRANCH_COUNT: usize = 57;
    pub const PARAMETER_COUNT: usize = 287;
    pub const VARIABLE_COUNT: usize = 612;
    pub const DDT_STATE_COUNT: usize = 121;
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
            scalar_static_f64: boxed_zero_f64_array::<939>(),
            scalar_static_bool: boxed_zero_bool_array::<169>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmhemt'", name));
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
        self.recompute_instance_static();
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
        self.scalar_static_f64[0]=p.p31;
        self.scalar_static_f64[1]=p.p32;
        self.scalar_static_f64[2]=p.p34;
        self.scalar_static_f64[3]=p.p149;
        self.scalar_static_bool[0]=(1.0==self.scalar_static_f64[3]);
        self.scalar_static_f64[4]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_bool[1]=(0.0==self.scalar_static_f64[2]);
        self.scalar_static_f64[5]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_bool[2]=((self.scalar_static_f64[4]!=0.0)&&(self.scalar_static_f64[5]!=0.0));
        self.scalar_static_f64[6]=(if self.scalar_static_bool[2]{1.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[7]=p.p0;
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]+273.15);
        self.scalar_static_f64[9]=p.p274;
        self.scalar_static_f64[10]=p.p81;
        self.scalar_static_bool[3]=(0.0==self.scalar_static_f64[10]);
        self.scalar_static_f64[11]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_bool[4]=(1.0==self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_bool[5]=(self.scalar_static_f64[10]==2.0);
        self.scalar_static_f64[13]=(if self.scalar_static_bool[5]{1.0}else{0.0});
        self.scalar_static_bool[6]=(self.scalar_static_f64[10]==3.0);
        self.scalar_static_f64[14]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_bool[7]=(self.scalar_static_f64[10]==4.0);
        self.scalar_static_f64[15]=(if self.scalar_static_bool[7]{1.0}else{0.0});
        self.scalar_static_bool[8]=(self.scalar_static_f64[10]==5.0);
        self.scalar_static_f64[16]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[11]!=0.0));
        self.scalar_static_bool[10]=((self.scalar_static_f64[12]!=0.0)&&self.scalar_static_bool[9]);
        self.scalar_static_f64[17]=p.p128;
        self.scalar_static_f64[18]=(0.25*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(self.scalar_static_f64[17]*self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=p.p100;
        self.scalar_static_f64[21]=p.p101;
        self.scalar_static_f64[22]=p.p104;
        self.scalar_static_f64[23]=p.p105;
        self.scalar_static_f64[24]=p.p106;
        self.scalar_static_f64[25]=p.p107;
        self.scalar_static_f64[26]=p.p102;
        self.scalar_static_f64[27]=p.p103;
        self.scalar_static_bool[11]=((self.scalar_static_f64[11]!=0.0)||(self.scalar_static_f64[12]!=0.0));
        self.scalar_static_bool[12]=(!self.scalar_static_bool[11]);
        self.scalar_static_bool[13]=((self.scalar_static_f64[13]!=0.0)&&self.scalar_static_bool[12]);
        self.scalar_static_f64[28]=p.p112;
        self.scalar_static_f64[29]=p.p113;
        self.scalar_static_f64[30]=p.p116;
        self.scalar_static_f64[31]=(-self.scalar_static_f64[30]);
        self.scalar_static_f64[32]=p.p117;
        self.scalar_static_f64[33]=p.p118;
        self.scalar_static_f64[34]=p.p114;
        self.scalar_static_f64[35]=p.p115;
        self.scalar_static_bool[14]=((self.scalar_static_f64[13]!=0.0)||self.scalar_static_bool[11]);
        self.scalar_static_bool[15]=(!self.scalar_static_bool[14]);
        self.scalar_static_bool[16]=((self.scalar_static_f64[14]!=0.0)&&self.scalar_static_bool[15]);
        self.scalar_static_f64[36]=p.p124;
        self.scalar_static_f64[37]=p.p123;
        self.scalar_static_f64[38]=p.p125;
        self.scalar_static_f64[39]=p.p127;
        self.scalar_static_f64[40]=p.p10;
        self.scalar_static_f64[41]=p.p122;
        self.scalar_static_f64[42]=p.p120;
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]-1e-9);
        self.scalar_static_f64[44]=(0.5*self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=p.p121;
        self.scalar_static_f64[46]=p.p126;
        self.scalar_static_bool[17]=((self.scalar_static_f64[14]!=0.0)||self.scalar_static_bool[14]);
        self.scalar_static_bool[18]=(!self.scalar_static_bool[17]);
        self.scalar_static_bool[19]=((self.scalar_static_f64[15]!=0.0)&&self.scalar_static_bool[18]);
        self.scalar_static_f64[47]=p.p82;
        self.scalar_static_f64[48]=p.p85;
        self.scalar_static_f64[49]=p.p86;
        self.scalar_static_f64[50]=p.p84;
        self.scalar_static_f64[51]=p.p87;
        self.scalar_static_f64[52]=p.p88;
        self.scalar_static_f64[53]=p.p89;
        self.scalar_static_f64[54]=(self.scalar_static_f64[53]*self.scalar_static_f64[53]);
        self.scalar_static_f64[55]=p.p91;
        self.scalar_static_f64[56]=(self.scalar_static_f64[40]*self.scalar_static_f64[55]);
        self.scalar_static_f64[57]=(self.scalar_static_f64[56]).abs();
        self.scalar_static_f64[58]=p.p90;
        self.scalar_static_f64[59]=(self.scalar_static_f64[58]*self.scalar_static_f64[58]);
        self.scalar_static_f64[60]=p.p92;
        self.scalar_static_f64[61]=(self.scalar_static_f64[40]*self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[61]).abs();
        self.scalar_static_f64[63]=p.p93;
        self.scalar_static_f64[64]=p.p13;
        self.scalar_static_f64[65]=(self.scalar_static_f64[63]*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[65]).abs();
        self.scalar_static_f64[67]=p.p94;
        self.scalar_static_f64[68]=p.p17;
        self.scalar_static_f64[69]=(self.scalar_static_f64[67]*self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(self.scalar_static_f64[69]).abs();
        self.scalar_static_f64[71]=p.p95;
        self.scalar_static_f64[72]=p.p36;
        self.scalar_static_f64[73]=(self.scalar_static_f64[71]*self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(self.scalar_static_f64[73]).abs();
        self.scalar_static_f64[75]=p.p96;
        self.scalar_static_f64[76]=p.p37;
        self.scalar_static_f64[77]=(self.scalar_static_f64[75]*self.scalar_static_f64[76]);
        self.scalar_static_f64[78]=(self.scalar_static_f64[77]).abs();
        self.scalar_static_bool[20]=((self.scalar_static_f64[15]!=0.0)||self.scalar_static_bool[17]);
        self.scalar_static_bool[21]=(!self.scalar_static_bool[20]);
        self.scalar_static_bool[22]=((self.scalar_static_f64[16]!=0.0)&&self.scalar_static_bool[21]);
        self.scalar_static_f64[79]=p.p129;
        self.scalar_static_f64[80]=p.p130;
        self.scalar_static_f64[81]=p.p131;
        self.scalar_static_f64[82]=p.p132;
        self.scalar_static_f64[83]=p.p133;
        self.scalar_static_f64[84]=p.p134;
        self.scalar_static_f64[85]=p.p137;
        self.scalar_static_f64[86]=(self.scalar_static_f64[8]*8.617087e-5);
        self.scalar_static_f64[87]=(self.scalar_static_f64[85]/self.scalar_static_f64[86]);
        self.scalar_static_f64[88]=p.p138;
        self.scalar_static_f64[89]=p.p139;
        self.scalar_static_f64[90]=p.p140;
        self.scalar_static_f64[91]=p.p141;
        self.scalar_static_f64[92]=p.p142;
        self.scalar_static_f64[93]=p.p143;
        self.scalar_static_f64[94]=p.p146;
        self.scalar_static_f64[95]=(self.scalar_static_f64[94]/self.scalar_static_f64[86]);
        self.scalar_static_f64[96]=p.p147;
        self.scalar_static_f64[97]=(self.scalar_static_f64[72]*self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=(self.scalar_static_f64[97]).abs();
        self.scalar_static_f64[99]=p.p148;
        self.scalar_static_f64[100]=(self.scalar_static_f64[76]*self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=(self.scalar_static_f64[100]).abs();
        self.scalar_static_f64[102]=p.p9;
        self.scalar_static_f64[103]=p.p1;
        self.scalar_static_f64[104]=(self.scalar_static_f64[102]/self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=p.p2;
        self.scalar_static_f64[106]=(self.scalar_static_f64[102]/self.scalar_static_f64[105]);
        self.scalar_static_f64[107]=p.p26;
        self.scalar_static_f64[108]=(1.0+self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=p.p27;
        self.scalar_static_f64[110]=p.p22;
        self.scalar_static_f64[111]=p.p23;
        self.scalar_static_f64[112]=(self.scalar_static_f64[111]*self.scalar_static_f64[111]);
        self.scalar_static_f64[113]=p.p266;
        self.scalar_static_f64[114]=p.p267;
        self.scalar_static_f64[115]=p.p24;
        self.scalar_static_f64[116]=(self.scalar_static_f64[104]+self.scalar_static_f64[106]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[106]/self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=p.p11;
        self.scalar_static_f64[119]=(self.scalar_static_f64[117]*self.scalar_static_f64[118]);
        self.scalar_static_f64[120]=p.p3;
        self.scalar_static_f64[121]=p.p4;
        self.scalar_static_f64[122]=(2.0*self.scalar_static_f64[121]);
        self.scalar_static_f64[123]=(self.scalar_static_f64[122]*1.602176634e-19);
        self.scalar_static_f64[124]=(self.scalar_static_f64[123]*3.24e17);
        self.scalar_static_f64[125]=p.p30;
        self.scalar_static_f64[126]=(self.scalar_static_f64[104]/1.602176634e-19);
        self.scalar_static_f64[127]=p.p28;
        self.scalar_static_f64[128]=(self.scalar_static_f64[127]/3.0);
        self.scalar_static_f64[129]=(2.0*self.scalar_static_f64[127]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[129]/3.0);
        self.scalar_static_f64[131]=(self.scalar_static_f64[126]/3.24e17);
        self.scalar_static_f64[132]=f64::powf(self.scalar_static_f64[126],0.6666666666666666);
        self.scalar_static_f64[133]=p.p29;
        self.scalar_static_f64[134]=(-self.scalar_static_f64[126]);
        self.scalar_static_f64[135]=p.p20;
        self.scalar_static_f64[136]=p.p19;
        self.scalar_static_f64[137]=(self.scalar_static_f64[104]/self.scalar_static_f64[102]);
        self.scalar_static_f64[138]=(self.scalar_static_f64[106]/self.scalar_static_f64[102]);
        self.scalar_static_f64[139]=p.p14;
        self.scalar_static_f64[140]=p.p15;
        self.scalar_static_f64[141]=p.p16;
        self.scalar_static_f64[142]=p.p18;
        self.scalar_static_f64[143]=(-1.0/self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=p.p5;
        self.scalar_static_f64[145]=p.p21;
        self.scalar_static_f64[146]=p.p25;
        self.scalar_static_f64[147]=(self.scalar_static_f64[146]*self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=p.p269;
        self.scalar_static_f64[149]=p.p271;
        self.scalar_static_f64[150]=p.p270;
        self.scalar_static_f64[151]=p.p272;
        self.scalar_static_f64[152]=p.p268;
        self.scalar_static_f64[153]=p.p273;
        self.scalar_static_f64[154]=(self.scalar_static_f64[104]*self.scalar_static_f64[121]);
        self.scalar_static_f64[155]=(self.scalar_static_f64[144]*self.scalar_static_f64[154]);
        self.scalar_static_f64[156]=(self.scalar_static_f64[120]*self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=p.p233;
        self.scalar_static_f64[158]=p.p232;
        self.scalar_static_f64[159]=p.p231;
        self.scalar_static_f64[160]=p.p56;
        self.scalar_static_bool[23]=(0.0==self.scalar_static_f64[160]);
        self.scalar_static_f64[161]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(1.0==self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=(if self.scalar_static_bool[24]{1.0}else{0.0});
        self.scalar_static_bool[25]=(2.0==self.scalar_static_f64[160]);
        self.scalar_static_f64[163]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_bool[26]=(3.0==self.scalar_static_f64[160]);
        self.scalar_static_f64[164]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_bool[27]=(4.0==self.scalar_static_f64[160]);
        self.scalar_static_f64[165]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_bool[28]=(!(self.scalar_static_f64[161]!=0.0));
        self.scalar_static_bool[29]=((self.scalar_static_f64[162]!=0.0)&&self.scalar_static_bool[28]);
        self.scalar_static_f64[166]=p.p57;
        self.scalar_static_f64[167]=(8.617087e-5*self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=p.p63;
        self.scalar_static_f64[169]=p.p71;
        self.scalar_static_f64[170]=(self.scalar_static_f64[120]*self.scalar_static_f64[121]);
        self.scalar_static_f64[171]=(self.scalar_static_f64[144]*self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=p.p60;
        self.scalar_static_f64[173]=(8.617087e-5*self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=p.p64;
        self.scalar_static_f64[175]=p.p72;
        self.scalar_static_bool[30]=((self.scalar_static_f64[161]!=0.0)||(self.scalar_static_f64[162]!=0.0));
        self.scalar_static_bool[31]=(!self.scalar_static_bool[30]);
        self.scalar_static_bool[32]=((self.scalar_static_f64[163]!=0.0)&&self.scalar_static_bool[31]);
        self.scalar_static_f64[176]=p.p67;
        self.scalar_static_f64[177]=p.p75;
        self.scalar_static_f64[178]=p.p77;
        self.scalar_static_f64[179]=p.p61;
        self.scalar_static_f64[180]=p.p79;
        self.scalar_static_f64[181]=p.p69;
        self.scalar_static_f64[182]=p.p65;
        self.scalar_static_f64[183]=p.p73;
        self.scalar_static_f64[184]=p.p68;
        self.scalar_static_f64[185]=p.p76;
        self.scalar_static_f64[186]=p.p78;
        self.scalar_static_f64[187]=p.p62;
        self.scalar_static_f64[188]=p.p80;
        self.scalar_static_f64[189]=p.p70;
        self.scalar_static_f64[190]=p.p66;
        self.scalar_static_f64[191]=p.p74;
        self.scalar_static_bool[33]=((self.scalar_static_f64[163]!=0.0)||self.scalar_static_bool[30]);
        self.scalar_static_bool[34]=(!self.scalar_static_bool[33]);
        self.scalar_static_bool[35]=((self.scalar_static_f64[164]!=0.0)&&self.scalar_static_bool[34]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[168]*self.scalar_static_f64[171]);
        self.scalar_static_f64[193]=p.p58;
        self.scalar_static_f64[194]=(self.scalar_static_f64[171]*self.scalar_static_f64[174]);
        self.scalar_static_f64[195]=p.p59;
        self.scalar_static_bool[36]=((self.scalar_static_f64[164]!=0.0)||self.scalar_static_bool[33]);
        self.scalar_static_bool[37]=(!self.scalar_static_bool[36]);
        self.scalar_static_bool[38]=((self.scalar_static_f64[165]!=0.0)&&self.scalar_static_bool[37]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[171]*self.scalar_static_f64[182]);
        self.scalar_static_f64[197]=(self.scalar_static_f64[171]*self.scalar_static_f64[190]);
        self.scalar_static_f64[198]=if param_given[45]{1.0}else{0.0};
        self.scalar_static_f64[199]=if param_given[44]{1.0}else{0.0};
        self.scalar_static_bool[39]=(1.0==self.scalar_static_f64[6]);
        self.scalar_static_f64[200]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_f64[201]=p.p50;
        self.scalar_static_f64[202]=p.p12;
        self.scalar_static_f64[203]=(self.scalar_static_f64[202]/1.602176634e-19);
        self.scalar_static_f64[204]=p.p38;
        self.scalar_static_f64[205]=p.p35;
        self.scalar_static_f64[206]=p.p51;
        self.scalar_static_f64[207]=(self.scalar_static_f64[121]*self.scalar_static_f64[144]);
        self.scalar_static_f64[208]=p.p40;
        self.scalar_static_f64[209]=p.p52;
        self.scalar_static_f64[210]=p.p46;
        self.scalar_static_bool[40]=(0.0!=self.scalar_static_f64[198]);
        self.scalar_static_f64[211]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_bool[41]=((self.scalar_static_f64[200]!=0.0)&&(self.scalar_static_f64[211]!=0.0));
        self.scalar_static_f64[212]=p.p45;
        self.scalar_static_f64[213]=(1.0+self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=(if self.scalar_static_bool[41]{self.scalar_static_f64[213]}else{0.0});
        self.scalar_static_f64[215]=(self.scalar_static_f64[214]).sqrt();
        self.scalar_static_bool[42]=(!(self.scalar_static_f64[211]!=0.0));
        self.scalar_static_bool[43]=((self.scalar_static_f64[200]!=0.0)&&self.scalar_static_bool[42]);
        self.scalar_static_f64[216]=p.p42;
        self.scalar_static_f64[217]=(1.0/self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=p.p48;
        self.scalar_static_f64[219]=p.p54;
        self.scalar_static_f64[220]=p.p39;
        self.scalar_static_f64[221]=p.p41;
        self.scalar_static_f64[222]=p.p53;
        self.scalar_static_f64[223]=p.p47;
        self.scalar_static_bool[44]=(0.0!=self.scalar_static_f64[199]);
        self.scalar_static_f64[224]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_bool[45]=((self.scalar_static_f64[200]!=0.0)&&(self.scalar_static_f64[224]!=0.0));
        self.scalar_static_f64[225]=p.p44;
        self.scalar_static_f64[226]=(1.0+self.scalar_static_f64[225]);
        self.scalar_static_bool[46]=(!(self.scalar_static_f64[224]!=0.0));
        self.scalar_static_bool[47]=((self.scalar_static_f64[200]!=0.0)&&self.scalar_static_bool[46]);
        self.scalar_static_f64[227]=p.p43;
        self.scalar_static_f64[228]=(1.0/self.scalar_static_f64[227]);
        self.scalar_static_f64[229]=p.p49;
        self.scalar_static_f64[230]=p.p55;
        self.scalar_static_bool[48]=(0.0==self.scalar_static_f64[3]);
        self.scalar_static_f64[231]=(if self.scalar_static_bool[48]{1.0}else{0.0});
        self.scalar_static_f64[232]=p.p150;
        self.scalar_static_bool[49]=(0.0!=self.scalar_static_f64[232]);
        self.scalar_static_f64[233]=(if self.scalar_static_bool[49]{1.0}else{0.0});
        self.scalar_static_bool[50]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[233]!=0.0));
        self.scalar_static_bool[51]=(1.0==self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_bool[52]=(self.scalar_static_bool[50]&&(self.scalar_static_f64[234]!=0.0));
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[234]!=0.0));
        self.scalar_static_bool[54]=(self.scalar_static_bool[50]&&self.scalar_static_bool[53]);
        self.scalar_static_f64[235]=p.p165;
        self.scalar_static_f64[236]=(1.0+self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=p.p166;
        self.scalar_static_f64[238]=p.p159;
        self.scalar_static_f64[239]=p.p162;
        self.scalar_static_f64[240]=p.p167;
        self.scalar_static_f64[241]=p.p168;
        self.scalar_static_f64[242]=(self.scalar_static_f64[241]*self.scalar_static_f64[241]);
        self.scalar_static_f64[243]=p.p160;
        self.scalar_static_f64[244]=(self.scalar_static_f64[102]/self.scalar_static_f64[243]);
        self.scalar_static_f64[245]=(if self.scalar_static_bool[50]{self.scalar_static_f64[244]}else{0.0});
        self.scalar_static_f64[246]=p.p161;
        self.scalar_static_f64[247]=p.p158;
        self.scalar_static_f64[248]=(self.scalar_static_f64[245]/1.602176634e-19);
        self.scalar_static_f64[249]=(if self.scalar_static_bool[50]{self.scalar_static_f64[248]}else{self.scalar_static_f64[126]});
        self.scalar_static_f64[250]=p.p169;
        self.scalar_static_f64[251]=(self.scalar_static_f64[250]/3.0);
        self.scalar_static_f64[252]=(2.0*self.scalar_static_f64[250]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[252]/3.0);
        self.scalar_static_f64[254]=(self.scalar_static_f64[249]/3.24e17);
        self.scalar_static_f64[255]=f64::powf(self.scalar_static_f64[249],0.6666666666666666);
        self.scalar_static_f64[256]=p.p170;
        self.scalar_static_f64[257]=(-self.scalar_static_f64[249]);
        self.scalar_static_f64[258]=p.p163;
        self.scalar_static_f64[259]=p.p164;
        self.scalar_static_f64[260]=(self.scalar_static_f64[245]/self.scalar_static_f64[102]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[121]*self.scalar_static_f64[245]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[144]*self.scalar_static_f64[261]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[246]*self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=p.p236;
        self.scalar_static_f64[265]=p.p235;
        self.scalar_static_f64[266]=p.p234;
        self.scalar_static_bool[55]=(!(self.scalar_static_f64[233]!=0.0));
        self.scalar_static_bool[56]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[55]);
        self.scalar_static_bool[57]=(!(self.scalar_static_f64[231]!=0.0));
        self.scalar_static_bool[58]=((self.scalar_static_f64[233]!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_bool[59]=((self.scalar_static_f64[234]!=0.0)&&self.scalar_static_bool[58]);
        self.scalar_static_bool[60]=(self.scalar_static_bool[53]&&self.scalar_static_bool[58]);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[58]{self.scalar_static_f64[244]}else{self.scalar_static_f64[245]});
        self.scalar_static_f64[268]=(self.scalar_static_f64[267]/1.602176634e-19);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[58]{self.scalar_static_f64[268]}else{self.scalar_static_f64[249]});
        self.scalar_static_f64[270]=(self.scalar_static_f64[269]/3.24e17);
        self.scalar_static_f64[271]=f64::powf(self.scalar_static_f64[269],0.6666666666666666);
        self.scalar_static_f64[272]=(-self.scalar_static_f64[269]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[267]/self.scalar_static_f64[102]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[121]*self.scalar_static_f64[267]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[144]*self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[246]*self.scalar_static_f64[275]);
        self.scalar_static_bool[61]=(self.scalar_static_bool[55]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[277]=p.p151;
        self.scalar_static_bool[62]=(0.0!=self.scalar_static_f64[277]);
        self.scalar_static_f64[278]=(if self.scalar_static_bool[62]{1.0}else{0.0});
        self.scalar_static_bool[63]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[278]!=0.0));
        self.scalar_static_bool[64]=(1.0==self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(if self.scalar_static_bool[64]{1.0}else{0.0});
        self.scalar_static_bool[65]=(self.scalar_static_bool[63]&&(self.scalar_static_f64[279]!=0.0));
        self.scalar_static_bool[66]=(!(self.scalar_static_f64[279]!=0.0));
        self.scalar_static_bool[67]=(self.scalar_static_bool[63]&&self.scalar_static_bool[66]);
        self.scalar_static_f64[280]=(if self.scalar_static_bool[63]{self.scalar_static_f64[244]}else{0.0});
        self.scalar_static_f64[281]=(self.scalar_static_f64[280]/1.602176634e-19);
        self.scalar_static_f64[282]=(if self.scalar_static_bool[63]{self.scalar_static_f64[281]}else{self.scalar_static_f64[269]});
        self.scalar_static_f64[283]=(self.scalar_static_f64[282]/3.24e17);
        self.scalar_static_f64[284]=f64::powf(self.scalar_static_f64[282],0.6666666666666666);
        self.scalar_static_f64[285]=(-self.scalar_static_f64[282]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[280]/self.scalar_static_f64[102]);
        self.scalar_static_f64[287]=(self.scalar_static_f64[121]*self.scalar_static_f64[280]);
        self.scalar_static_f64[288]=(self.scalar_static_f64[144]*self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(self.scalar_static_f64[246]*self.scalar_static_f64[288]);
        self.scalar_static_bool[68]=(!(self.scalar_static_f64[278]!=0.0));
        self.scalar_static_bool[69]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[68]);
        self.scalar_static_bool[70]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[278]!=0.0));
        self.scalar_static_bool[71]=((self.scalar_static_f64[279]!=0.0)&&self.scalar_static_bool[70]);
        self.scalar_static_bool[72]=(self.scalar_static_bool[66]&&self.scalar_static_bool[70]);
        self.scalar_static_f64[290]=(if self.scalar_static_bool[70]{self.scalar_static_f64[244]}else{self.scalar_static_f64[280]});
        self.scalar_static_f64[291]=(self.scalar_static_f64[290]/1.602176634e-19);
        self.scalar_static_f64[292]=(if self.scalar_static_bool[70]{self.scalar_static_f64[291]}else{self.scalar_static_f64[282]});
        self.scalar_static_f64[293]=(self.scalar_static_f64[292]/3.24e17);
        self.scalar_static_f64[294]=f64::powf(self.scalar_static_f64[292],0.6666666666666666);
        self.scalar_static_f64[295]=(-self.scalar_static_f64[292]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[290]/self.scalar_static_f64[102]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[121]*self.scalar_static_f64[290]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[144]*self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=(self.scalar_static_f64[246]*self.scalar_static_f64[298]);
        self.scalar_static_bool[73]=(self.scalar_static_bool[57]&&self.scalar_static_bool[68]);
        self.scalar_static_f64[300]=p.p152;
        self.scalar_static_bool[74]=(0.0!=self.scalar_static_f64[300]);
        self.scalar_static_f64[301]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_bool[75]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[301]!=0.0));
        self.scalar_static_bool[76]=(1.0==self.scalar_static_f64[300]);
        self.scalar_static_f64[302]=(if self.scalar_static_bool[76]{1.0}else{0.0});
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]&&(self.scalar_static_f64[302]!=0.0));
        self.scalar_static_bool[78]=(!(self.scalar_static_f64[302]!=0.0));
        self.scalar_static_bool[79]=(self.scalar_static_bool[75]&&self.scalar_static_bool[78]);
        self.scalar_static_f64[303]=p.p178;
        self.scalar_static_f64[304]=(1.0+self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=p.p179;
        self.scalar_static_f64[306]=p.p172;
        self.scalar_static_f64[307]=p.p175;
        self.scalar_static_f64[308]=p.p180;
        self.scalar_static_f64[309]=p.p181;
        self.scalar_static_f64[310]=(self.scalar_static_f64[309]*self.scalar_static_f64[309]);
        self.scalar_static_f64[311]=p.p173;
        self.scalar_static_f64[312]=(self.scalar_static_f64[102]/self.scalar_static_f64[311]);
        self.scalar_static_f64[313]=(if self.scalar_static_bool[75]{self.scalar_static_f64[312]}else{0.0});
        self.scalar_static_f64[314]=p.p174;
        self.scalar_static_f64[315]=p.p171;
        self.scalar_static_f64[316]=(self.scalar_static_f64[313]/1.602176634e-19);
        self.scalar_static_f64[317]=(if self.scalar_static_bool[75]{self.scalar_static_f64[316]}else{self.scalar_static_f64[292]});
        self.scalar_static_f64[318]=p.p182;
        self.scalar_static_f64[319]=(self.scalar_static_f64[318]/3.0);
        self.scalar_static_f64[320]=(2.0*self.scalar_static_f64[318]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[320]/3.0);
        self.scalar_static_f64[322]=(self.scalar_static_f64[317]/3.24e17);
        self.scalar_static_f64[323]=f64::powf(self.scalar_static_f64[317],0.6666666666666666);
        self.scalar_static_f64[324]=p.p183;
        self.scalar_static_f64[325]=(-self.scalar_static_f64[317]);
        self.scalar_static_f64[326]=p.p176;
        self.scalar_static_f64[327]=p.p177;
        self.scalar_static_f64[328]=(self.scalar_static_f64[313]/self.scalar_static_f64[102]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[121]*self.scalar_static_f64[313]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[144]*self.scalar_static_f64[329]);
        self.scalar_static_f64[331]=(self.scalar_static_f64[314]*self.scalar_static_f64[330]);
        self.scalar_static_f64[332]=p.p239;
        self.scalar_static_f64[333]=p.p238;
        self.scalar_static_f64[334]=p.p237;
        self.scalar_static_bool[80]=(!(self.scalar_static_f64[301]!=0.0));
        self.scalar_static_bool[81]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[80]);
        self.scalar_static_bool[82]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[301]!=0.0));
        self.scalar_static_bool[83]=((self.scalar_static_f64[302]!=0.0)&&self.scalar_static_bool[82]);
        self.scalar_static_bool[84]=(self.scalar_static_bool[78]&&self.scalar_static_bool[82]);
        self.scalar_static_f64[335]=(if self.scalar_static_bool[82]{self.scalar_static_f64[312]}else{self.scalar_static_f64[313]});
        self.scalar_static_f64[336]=(self.scalar_static_f64[335]/1.602176634e-19);
        self.scalar_static_f64[337]=(if self.scalar_static_bool[82]{self.scalar_static_f64[336]}else{self.scalar_static_f64[317]});
        self.scalar_static_f64[338]=(self.scalar_static_f64[337]/3.24e17);
        self.scalar_static_f64[339]=f64::powf(self.scalar_static_f64[337],0.6666666666666666);
        self.scalar_static_f64[340]=(-self.scalar_static_f64[337]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[335]/self.scalar_static_f64[102]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[121]*self.scalar_static_f64[335]);
        self.scalar_static_f64[343]=(self.scalar_static_f64[144]*self.scalar_static_f64[342]);
        self.scalar_static_f64[344]=(self.scalar_static_f64[314]*self.scalar_static_f64[343]);
        self.scalar_static_bool[85]=(self.scalar_static_bool[57]&&self.scalar_static_bool[80]);
        self.scalar_static_f64[345]=p.p153;
        self.scalar_static_bool[86]=(0.0!=self.scalar_static_f64[345]);
        self.scalar_static_f64[346]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_bool[87]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[346]!=0.0));
        self.scalar_static_bool[88]=(1.0==self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_bool[89]=(self.scalar_static_bool[87]&&(self.scalar_static_f64[347]!=0.0));
        self.scalar_static_bool[90]=(!(self.scalar_static_f64[347]!=0.0));
        self.scalar_static_bool[91]=(self.scalar_static_bool[87]&&self.scalar_static_bool[90]);
        self.scalar_static_f64[348]=(if self.scalar_static_bool[87]{self.scalar_static_f64[312]}else{0.0});
        self.scalar_static_f64[349]=(self.scalar_static_f64[348]/1.602176634e-19);
        self.scalar_static_f64[350]=(if self.scalar_static_bool[87]{self.scalar_static_f64[349]}else{self.scalar_static_f64[337]});
        self.scalar_static_f64[351]=(self.scalar_static_f64[350]/3.24e17);
        self.scalar_static_f64[352]=f64::powf(self.scalar_static_f64[350],0.6666666666666666);
        self.scalar_static_f64[353]=(-self.scalar_static_f64[350]);
        self.scalar_static_f64[354]=(self.scalar_static_f64[348]/self.scalar_static_f64[102]);
        self.scalar_static_f64[355]=(self.scalar_static_f64[121]*self.scalar_static_f64[348]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[144]*self.scalar_static_f64[355]);
        self.scalar_static_f64[357]=(self.scalar_static_f64[314]*self.scalar_static_f64[356]);
        self.scalar_static_bool[92]=(!(self.scalar_static_f64[346]!=0.0));
        self.scalar_static_bool[93]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[92]);
        self.scalar_static_bool[94]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[346]!=0.0));
        self.scalar_static_bool[95]=((self.scalar_static_f64[347]!=0.0)&&self.scalar_static_bool[94]);
        self.scalar_static_bool[96]=(self.scalar_static_bool[90]&&self.scalar_static_bool[94]);
        self.scalar_static_f64[358]=(if self.scalar_static_bool[94]{self.scalar_static_f64[312]}else{self.scalar_static_f64[348]});
        self.scalar_static_f64[359]=(self.scalar_static_f64[358]/1.602176634e-19);
        self.scalar_static_f64[360]=(if self.scalar_static_bool[94]{self.scalar_static_f64[359]}else{self.scalar_static_f64[350]});
        self.scalar_static_f64[361]=(self.scalar_static_f64[360]/3.24e17);
        self.scalar_static_f64[362]=f64::powf(self.scalar_static_f64[360],0.6666666666666666);
        self.scalar_static_f64[363]=(-self.scalar_static_f64[360]);
        self.scalar_static_f64[364]=(self.scalar_static_f64[358]/self.scalar_static_f64[102]);
        self.scalar_static_f64[365]=(self.scalar_static_f64[121]*self.scalar_static_f64[358]);
        self.scalar_static_f64[366]=(self.scalar_static_f64[144]*self.scalar_static_f64[365]);
        self.scalar_static_f64[367]=(self.scalar_static_f64[314]*self.scalar_static_f64[366]);
        self.scalar_static_bool[97]=(self.scalar_static_bool[57]&&self.scalar_static_bool[92]);
        self.scalar_static_f64[368]=p.p154;
        self.scalar_static_bool[98]=(0.0!=self.scalar_static_f64[368]);
        self.scalar_static_f64[369]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_bool[99]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[369]!=0.0));
        self.scalar_static_bool[100]=(1.0==self.scalar_static_f64[368]);
        self.scalar_static_f64[370]=(if self.scalar_static_bool[100]{1.0}else{0.0});
        self.scalar_static_bool[101]=(self.scalar_static_bool[99]&&(self.scalar_static_f64[370]!=0.0));
        self.scalar_static_bool[102]=(!(self.scalar_static_f64[370]!=0.0));
        self.scalar_static_bool[103]=(self.scalar_static_bool[99]&&self.scalar_static_bool[102]);
        self.scalar_static_f64[371]=p.p191;
        self.scalar_static_f64[372]=(1.0+self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=p.p192;
        self.scalar_static_f64[374]=p.p185;
        self.scalar_static_f64[375]=p.p188;
        self.scalar_static_f64[376]=p.p193;
        self.scalar_static_f64[377]=p.p194;
        self.scalar_static_f64[378]=(self.scalar_static_f64[377]*self.scalar_static_f64[377]);
        self.scalar_static_f64[379]=p.p186;
        self.scalar_static_f64[380]=(self.scalar_static_f64[102]/self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=(if self.scalar_static_bool[99]{self.scalar_static_f64[380]}else{0.0});
        self.scalar_static_f64[382]=p.p187;
        self.scalar_static_f64[383]=p.p184;
        self.scalar_static_f64[384]=(self.scalar_static_f64[381]/1.602176634e-19);
        self.scalar_static_f64[385]=(if self.scalar_static_bool[99]{self.scalar_static_f64[384]}else{self.scalar_static_f64[360]});
        self.scalar_static_f64[386]=p.p195;
        self.scalar_static_f64[387]=(self.scalar_static_f64[386]/3.0);
        self.scalar_static_f64[388]=(2.0*self.scalar_static_f64[386]);
        self.scalar_static_f64[389]=(self.scalar_static_f64[388]/3.0);
        self.scalar_static_f64[390]=(self.scalar_static_f64[385]/3.24e17);
        self.scalar_static_f64[391]=f64::powf(self.scalar_static_f64[385],0.6666666666666666);
        self.scalar_static_f64[392]=p.p196;
        self.scalar_static_f64[393]=(-self.scalar_static_f64[385]);
        self.scalar_static_f64[394]=p.p189;
        self.scalar_static_f64[395]=p.p190;
        self.scalar_static_f64[396]=(self.scalar_static_f64[381]/self.scalar_static_f64[102]);
        self.scalar_static_f64[397]=(self.scalar_static_f64[121]*self.scalar_static_f64[381]);
        self.scalar_static_f64[398]=(self.scalar_static_f64[144]*self.scalar_static_f64[397]);
        self.scalar_static_f64[399]=(self.scalar_static_f64[382]*self.scalar_static_f64[398]);
        self.scalar_static_f64[400]=p.p242;
        self.scalar_static_f64[401]=p.p241;
        self.scalar_static_f64[402]=p.p240;
        self.scalar_static_bool[104]=(!(self.scalar_static_f64[369]!=0.0));
        self.scalar_static_bool[105]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[104]);
        self.scalar_static_bool[106]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[369]!=0.0));
        self.scalar_static_bool[107]=((self.scalar_static_f64[370]!=0.0)&&self.scalar_static_bool[106]);
        self.scalar_static_bool[108]=(self.scalar_static_bool[102]&&self.scalar_static_bool[106]);
        self.scalar_static_f64[403]=(if self.scalar_static_bool[106]{self.scalar_static_f64[380]}else{self.scalar_static_f64[381]});
        self.scalar_static_f64[404]=(self.scalar_static_f64[403]/1.602176634e-19);
        self.scalar_static_f64[405]=(if self.scalar_static_bool[106]{self.scalar_static_f64[404]}else{self.scalar_static_f64[385]});
        self.scalar_static_f64[406]=(self.scalar_static_f64[405]/3.24e17);
        self.scalar_static_f64[407]=f64::powf(self.scalar_static_f64[405],0.6666666666666666);
        self.scalar_static_f64[408]=(-self.scalar_static_f64[405]);
        self.scalar_static_f64[409]=(self.scalar_static_f64[403]/self.scalar_static_f64[102]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[121]*self.scalar_static_f64[403]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[144]*self.scalar_static_f64[410]);
        self.scalar_static_f64[412]=(self.scalar_static_f64[382]*self.scalar_static_f64[411]);
        self.scalar_static_bool[109]=(self.scalar_static_bool[57]&&self.scalar_static_bool[104]);
        self.scalar_static_f64[413]=p.p155;
        self.scalar_static_bool[110]=(0.0!=self.scalar_static_f64[413]);
        self.scalar_static_f64[414]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_bool[111]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[414]!=0.0));
        self.scalar_static_bool[112]=(1.0==self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_bool[113]=(self.scalar_static_bool[111]&&(self.scalar_static_f64[415]!=0.0));
        self.scalar_static_bool[114]=(!(self.scalar_static_f64[415]!=0.0));
        self.scalar_static_bool[115]=(self.scalar_static_bool[111]&&self.scalar_static_bool[114]);
        self.scalar_static_f64[416]=(if self.scalar_static_bool[111]{self.scalar_static_f64[380]}else{0.0});
        self.scalar_static_f64[417]=(self.scalar_static_f64[416]/1.602176634e-19);
        self.scalar_static_f64[418]=(if self.scalar_static_bool[111]{self.scalar_static_f64[417]}else{self.scalar_static_f64[405]});
        self.scalar_static_f64[419]=(self.scalar_static_f64[418]/3.24e17);
        self.scalar_static_f64[420]=f64::powf(self.scalar_static_f64[418],0.6666666666666666);
        self.scalar_static_f64[421]=(-self.scalar_static_f64[418]);
        self.scalar_static_f64[422]=(self.scalar_static_f64[416]/self.scalar_static_f64[102]);
        self.scalar_static_f64[423]=(self.scalar_static_f64[121]*self.scalar_static_f64[416]);
        self.scalar_static_f64[424]=(self.scalar_static_f64[144]*self.scalar_static_f64[423]);
        self.scalar_static_f64[425]=(self.scalar_static_f64[382]*self.scalar_static_f64[424]);
        self.scalar_static_bool[116]=(!(self.scalar_static_f64[414]!=0.0));
        self.scalar_static_bool[117]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[116]);
        self.scalar_static_bool[118]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[414]!=0.0));
        self.scalar_static_bool[119]=((self.scalar_static_f64[415]!=0.0)&&self.scalar_static_bool[118]);
        self.scalar_static_bool[120]=(self.scalar_static_bool[114]&&self.scalar_static_bool[118]);
        self.scalar_static_f64[426]=(if self.scalar_static_bool[118]{self.scalar_static_f64[380]}else{self.scalar_static_f64[416]});
        self.scalar_static_f64[427]=(self.scalar_static_f64[426]/1.602176634e-19);
        self.scalar_static_f64[428]=(if self.scalar_static_bool[118]{self.scalar_static_f64[427]}else{self.scalar_static_f64[418]});
        self.scalar_static_f64[429]=(self.scalar_static_f64[428]/3.24e17);
        self.scalar_static_f64[430]=f64::powf(self.scalar_static_f64[428],0.6666666666666666);
        self.scalar_static_f64[431]=(-self.scalar_static_f64[428]);
        self.scalar_static_f64[432]=(self.scalar_static_f64[426]/self.scalar_static_f64[102]);
        self.scalar_static_f64[433]=(self.scalar_static_f64[121]*self.scalar_static_f64[426]);
        self.scalar_static_f64[434]=(self.scalar_static_f64[144]*self.scalar_static_f64[433]);
        self.scalar_static_f64[435]=(self.scalar_static_f64[382]*self.scalar_static_f64[434]);
        self.scalar_static_bool[121]=(self.scalar_static_bool[57]&&self.scalar_static_bool[116]);
        self.scalar_static_f64[436]=p.p156;
        self.scalar_static_bool[122]=(0.0!=self.scalar_static_f64[436]);
        self.scalar_static_f64[437]=(if self.scalar_static_bool[122]{1.0}else{0.0});
        self.scalar_static_bool[123]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[437]!=0.0));
        self.scalar_static_bool[124]=(1.0==self.scalar_static_f64[436]);
        self.scalar_static_f64[438]=(if self.scalar_static_bool[124]{1.0}else{0.0});
        self.scalar_static_bool[125]=(self.scalar_static_bool[123]&&(self.scalar_static_f64[438]!=0.0));
        self.scalar_static_bool[126]=(!(self.scalar_static_f64[438]!=0.0));
        self.scalar_static_bool[127]=(self.scalar_static_bool[123]&&self.scalar_static_bool[126]);
        self.scalar_static_f64[439]=p.p204;
        self.scalar_static_f64[440]=(1.0+self.scalar_static_f64[439]);
        self.scalar_static_f64[441]=p.p205;
        self.scalar_static_f64[442]=p.p198;
        self.scalar_static_f64[443]=p.p201;
        self.scalar_static_f64[444]=p.p206;
        self.scalar_static_f64[445]=p.p207;
        self.scalar_static_f64[446]=(self.scalar_static_f64[445]*self.scalar_static_f64[445]);
        self.scalar_static_f64[447]=p.p199;
        self.scalar_static_f64[448]=(self.scalar_static_f64[102]/self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=(if self.scalar_static_bool[123]{self.scalar_static_f64[448]}else{0.0});
        self.scalar_static_f64[450]=p.p200;
        self.scalar_static_f64[451]=p.p197;
        self.scalar_static_f64[452]=(self.scalar_static_f64[449]/1.602176634e-19);
        self.scalar_static_f64[453]=(if self.scalar_static_bool[123]{self.scalar_static_f64[452]}else{self.scalar_static_f64[428]});
        self.scalar_static_f64[454]=p.p208;
        self.scalar_static_f64[455]=(self.scalar_static_f64[454]/3.0);
        self.scalar_static_f64[456]=(2.0*self.scalar_static_f64[454]);
        self.scalar_static_f64[457]=(self.scalar_static_f64[456]/3.0);
        self.scalar_static_f64[458]=(self.scalar_static_f64[453]/3.24e17);
        self.scalar_static_f64[459]=f64::powf(self.scalar_static_f64[453],0.6666666666666666);
        self.scalar_static_f64[460]=p.p209;
        self.scalar_static_f64[461]=(-self.scalar_static_f64[453]);
        self.scalar_static_f64[462]=p.p202;
        self.scalar_static_f64[463]=p.p203;
        self.scalar_static_f64[464]=(self.scalar_static_f64[449]/self.scalar_static_f64[102]);
        self.scalar_static_f64[465]=(self.scalar_static_f64[121]*self.scalar_static_f64[449]);
        self.scalar_static_f64[466]=(self.scalar_static_f64[144]*self.scalar_static_f64[465]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[450]*self.scalar_static_f64[466]);
        self.scalar_static_f64[468]=p.p245;
        self.scalar_static_f64[469]=p.p244;
        self.scalar_static_f64[470]=p.p243;
        self.scalar_static_bool[128]=(!(self.scalar_static_f64[437]!=0.0));
        self.scalar_static_bool[129]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[128]);
        self.scalar_static_bool[130]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[437]!=0.0));
        self.scalar_static_bool[131]=((self.scalar_static_f64[438]!=0.0)&&self.scalar_static_bool[130]);
        self.scalar_static_bool[132]=(self.scalar_static_bool[126]&&self.scalar_static_bool[130]);
        self.scalar_static_f64[471]=(if self.scalar_static_bool[130]{self.scalar_static_f64[448]}else{self.scalar_static_f64[449]});
        self.scalar_static_f64[472]=(self.scalar_static_f64[471]/1.602176634e-19);
        self.scalar_static_f64[473]=(if self.scalar_static_bool[130]{self.scalar_static_f64[472]}else{self.scalar_static_f64[453]});
        self.scalar_static_f64[474]=(self.scalar_static_f64[473]/3.24e17);
        self.scalar_static_f64[475]=f64::powf(self.scalar_static_f64[473],0.6666666666666666);
        self.scalar_static_f64[476]=(-self.scalar_static_f64[473]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[471]/self.scalar_static_f64[102]);
        self.scalar_static_f64[478]=(self.scalar_static_f64[121]*self.scalar_static_f64[471]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[144]*self.scalar_static_f64[478]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[450]*self.scalar_static_f64[479]);
        self.scalar_static_bool[133]=(self.scalar_static_bool[57]&&self.scalar_static_bool[128]);
        self.scalar_static_f64[481]=p.p157;
        self.scalar_static_bool[134]=(0.0!=self.scalar_static_f64[481]);
        self.scalar_static_f64[482]=(if self.scalar_static_bool[134]{1.0}else{0.0});
        self.scalar_static_bool[135]=((self.scalar_static_f64[231]!=0.0)&&(self.scalar_static_f64[482]!=0.0));
        self.scalar_static_bool[136]=(1.0==self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=(if self.scalar_static_bool[136]{1.0}else{0.0});
        self.scalar_static_bool[137]=(self.scalar_static_bool[135]&&(self.scalar_static_f64[483]!=0.0));
        self.scalar_static_bool[138]=(!(self.scalar_static_f64[483]!=0.0));
        self.scalar_static_bool[139]=(self.scalar_static_bool[135]&&self.scalar_static_bool[138]);
        self.scalar_static_f64[484]=(if self.scalar_static_bool[135]{self.scalar_static_f64[448]}else{0.0});
        self.scalar_static_f64[485]=(self.scalar_static_f64[484]/1.602176634e-19);
        self.scalar_static_f64[486]=(if self.scalar_static_bool[135]{self.scalar_static_f64[485]}else{self.scalar_static_f64[473]});
        self.scalar_static_f64[487]=(self.scalar_static_f64[486]/3.24e17);
        self.scalar_static_f64[488]=f64::powf(self.scalar_static_f64[486],0.6666666666666666);
        self.scalar_static_f64[489]=(-self.scalar_static_f64[486]);
        self.scalar_static_f64[490]=(self.scalar_static_f64[484]/self.scalar_static_f64[102]);
        self.scalar_static_f64[491]=(self.scalar_static_f64[121]*self.scalar_static_f64[484]);
        self.scalar_static_f64[492]=(self.scalar_static_f64[144]*self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=(self.scalar_static_f64[450]*self.scalar_static_f64[492]);
        self.scalar_static_bool[140]=(!(self.scalar_static_f64[482]!=0.0));
        self.scalar_static_bool[141]=((self.scalar_static_f64[231]!=0.0)&&self.scalar_static_bool[140]);
        self.scalar_static_bool[142]=(self.scalar_static_bool[57]&&(self.scalar_static_f64[482]!=0.0));
        self.scalar_static_bool[143]=((self.scalar_static_f64[483]!=0.0)&&self.scalar_static_bool[142]);
        self.scalar_static_bool[144]=(self.scalar_static_bool[138]&&self.scalar_static_bool[142]);
        self.scalar_static_f64[494]=(if self.scalar_static_bool[142]{self.scalar_static_f64[448]}else{self.scalar_static_f64[484]});
        self.scalar_static_f64[495]=(self.scalar_static_f64[494]/1.602176634e-19);
        self.scalar_static_f64[496]=(if self.scalar_static_bool[142]{self.scalar_static_f64[495]}else{self.scalar_static_f64[486]});
        self.scalar_static_f64[497]=(self.scalar_static_f64[496]/3.24e17);
        self.scalar_static_f64[498]=f64::powf(self.scalar_static_f64[496],0.6666666666666666);
        self.scalar_static_f64[499]=(-self.scalar_static_f64[496]);
        self.scalar_static_f64[500]=(self.scalar_static_f64[494]/self.scalar_static_f64[102]);
        self.scalar_static_f64[501]=(self.scalar_static_f64[121]*self.scalar_static_f64[494]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[144]*self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[450]*self.scalar_static_f64[502]);
        self.scalar_static_bool[145]=(self.scalar_static_bool[57]&&self.scalar_static_bool[140]);
        self.scalar_static_f64[504]=p.p255;
        self.scalar_static_bool[146]=(1.0==self.scalar_static_f64[504]);
        self.scalar_static_f64[505]=(if self.scalar_static_bool[146]{1.0}else{0.0});
        self.scalar_static_f64[506]=p.p258;
        self.scalar_static_f64[507]=p.p256;
        self.scalar_static_f64[508]=(self.scalar_static_f64[121]/3.0);
        self.scalar_static_f64[509]=p.p257;
        self.scalar_static_f64[510]=(self.scalar_static_f64[508]/self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[507]+self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[506]*self.scalar_static_f64[511]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[144]*self.scalar_static_f64[509]);
        self.scalar_static_f64[514]=(self.scalar_static_f64[120]*self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(self.scalar_static_f64[512]/self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=(if (self.scalar_static_f64[505]!=0.0){self.scalar_static_f64[515]}else{1000.0});
        self.scalar_static_bool[147]=(self.scalar_static_f64[516]>0.0);
        self.scalar_static_f64[517]=(if self.scalar_static_bool[147]{1.0}else{0.0});
        self.scalar_static_bool[148]=((self.scalar_static_f64[505]!=0.0)&&(self.scalar_static_f64[517]!=0.0));
        self.scalar_static_f64[518]=(1.0/self.scalar_static_f64[516]);
        self.scalar_static_f64[519]=(if self.scalar_static_bool[148]{self.scalar_static_f64[518]}else{self.scalar_static_f64[516]});
        self.scalar_static_bool[149]=(!(self.scalar_static_f64[517]!=0.0));
        self.scalar_static_bool[150]=((self.scalar_static_f64[505]!=0.0)&&self.scalar_static_bool[149]);
        self.scalar_static_f64[520]=(if self.scalar_static_bool[150]{1000.0}else{self.scalar_static_f64[519]});
        self.scalar_static_bool[151]=(2.0==self.scalar_static_f64[504]);
        self.scalar_static_f64[521]=(if self.scalar_static_bool[151]{1.0}else{0.0});
        self.scalar_static_bool[152]=(!(self.scalar_static_f64[505]!=0.0));
        self.scalar_static_bool[153]=((self.scalar_static_f64[521]!=0.0)&&self.scalar_static_bool[152]);
        self.scalar_static_f64[522]=(if self.scalar_static_bool[153]{self.scalar_static_f64[515]}else{1000.0});
        self.scalar_static_f64[523]=(self.scalar_static_f64[122]/3.0);
        self.scalar_static_f64[524]=(self.scalar_static_f64[523]/self.scalar_static_f64[509]);
        self.scalar_static_f64[525]=(self.scalar_static_f64[506]*self.scalar_static_f64[524]);
        self.scalar_static_f64[526]=(self.scalar_static_f64[525]/self.scalar_static_f64[514]);
        self.scalar_static_f64[527]=(if self.scalar_static_bool[153]{self.scalar_static_f64[526]}else{1000.0});
        self.scalar_static_bool[154]=(self.scalar_static_f64[522]>0.0);
        self.scalar_static_f64[528]=(if self.scalar_static_bool[154]{1.0}else{0.0});
        self.scalar_static_bool[155]=(self.scalar_static_bool[153]&&(self.scalar_static_f64[528]!=0.0));
        self.scalar_static_f64[529]=(1.0/self.scalar_static_f64[522]);
        self.scalar_static_f64[530]=(if self.scalar_static_bool[155]{self.scalar_static_f64[529]}else{self.scalar_static_f64[522]});
        self.scalar_static_bool[156]=(!(self.scalar_static_f64[528]!=0.0));
        self.scalar_static_bool[157]=(self.scalar_static_bool[153]&&self.scalar_static_bool[156]);
        self.scalar_static_f64[531]=(if self.scalar_static_bool[157]{1000.0}else{self.scalar_static_f64[530]});
        self.scalar_static_bool[158]=(self.scalar_static_f64[527]>0.0);
        self.scalar_static_f64[532]=(if self.scalar_static_bool[158]{1.0}else{0.0});
        self.scalar_static_bool[159]=(self.scalar_static_bool[153]&&(self.scalar_static_f64[532]!=0.0));
        self.scalar_static_f64[533]=(1.0/self.scalar_static_f64[527]);
        self.scalar_static_f64[534]=(if self.scalar_static_bool[159]{self.scalar_static_f64[533]}else{self.scalar_static_f64[527]});
        self.scalar_static_bool[160]=(!(self.scalar_static_f64[532]!=0.0));
        self.scalar_static_bool[161]=(self.scalar_static_bool[153]&&self.scalar_static_bool[160]);
        self.scalar_static_f64[535]=(if self.scalar_static_bool[161]{1000.0}else{self.scalar_static_f64[534]});
        self.scalar_static_f64[536]=p.p210;
        self.scalar_static_f64[537]=(self.scalar_static_f64[207]*self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=p.p214;
        self.scalar_static_f64[539]=(self.scalar_static_f64[538]*self.scalar_static_f64[538]);
        self.scalar_static_f64[540]=p.p213;
        self.scalar_static_f64[541]=p.p211;
        self.scalar_static_f64[542]=(2.0*self.scalar_static_f64[538]);
        self.scalar_static_f64[543]=(self.scalar_static_f64[541]/self.scalar_static_f64[542]);
        self.scalar_static_bool[162]=(self.scalar_static_f64[540]<self.scalar_static_f64[543]);
        self.scalar_static_f64[544]=(if self.scalar_static_bool[162]{self.scalar_static_f64[540]}else{self.scalar_static_f64[543]});
        self.scalar_static_f64[545]=(if (self.scalar_static_f64[521]!=0.0){self.scalar_static_f64[544]}else{0.0});
        self.scalar_static_f64[546]=(self.scalar_static_f64[207]*self.scalar_static_f64[541]);
        self.scalar_static_f64[547]=(self.scalar_static_f64[207]*self.scalar_static_f64[545]);
        self.scalar_static_bool[163]=(!(self.scalar_static_f64[521]!=0.0));
        self.scalar_static_f64[548]=(if self.scalar_static_bool[163]{self.scalar_static_f64[544]}else{self.scalar_static_f64[545]});
        self.scalar_static_f64[549]=(self.scalar_static_f64[207]*self.scalar_static_f64[548]);
        self.scalar_static_f64[550]=p.p212;
        self.scalar_static_f64[551]=(self.scalar_static_f64[207]*self.scalar_static_f64[550]);
        self.scalar_static_f64[552]=p.p215;
        self.scalar_static_f64[553]=(self.scalar_static_f64[207]*self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=p.p216;
        self.scalar_static_f64[555]=(self.scalar_static_f64[207]*self.scalar_static_f64[554]);
        self.scalar_static_f64[556]=p.p217;
        self.scalar_static_f64[557]=(self.scalar_static_f64[207]*self.scalar_static_f64[556]);
        self.scalar_static_f64[558]=p.p279;
        self.scalar_static_f64[559]=p.p285;
        self.scalar_static_f64[560]=p.p275;
        self.scalar_static_f64[561]=p.p283;
        self.scalar_static_f64[562]=p.p277;
        self.scalar_static_f64[563]=p.p281;
        self.scalar_static_f64[564]=p.p280;
        self.scalar_static_f64[565]=p.p286;
        self.scalar_static_f64[566]=p.p276;
        self.scalar_static_f64[567]=p.p284;
        self.scalar_static_f64[568]=p.p278;
        self.scalar_static_f64[569]=p.p282;
        self.scalar_static_f64[570]=p.p222;
        self.scalar_static_f64[571]=p.p220;
        self.scalar_static_f64[572]=p.p227;
        self.scalar_static_f64[573]=p.p221;
        self.scalar_static_f64[574]=p.p218;
        self.scalar_static_f64[575]=p.p226;
        self.scalar_static_f64[576]=p.p219;
        self.scalar_static_f64[577]=(self.scalar_static_f64[207]*self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=p.p224;
        self.scalar_static_f64[579]=p.p225;
        self.scalar_static_f64[580]=p.p229;
        self.scalar_static_f64[581]=(self.scalar_static_f64[580]).ln();
        self.scalar_static_f64[582]=(-self.scalar_static_f64[581]);
        self.scalar_static_f64[583]=p.p228;
        self.scalar_static_f64[584]=(self.scalar_static_f64[582]/self.scalar_static_f64[583]);
        self.scalar_static_f64[585]={ let limited_exp_arg = self.scalar_static_f64[584]; if limited_exp_arg > 80.0 { 5.54062238439351e34 * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        self.scalar_static_f64[586]=(1.0-self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=p.p230;
        self.scalar_static_f64[588]=p.p223;
        self.scalar_static_f64[589]=(1.0-self.scalar_static_f64[583]);
        self.scalar_static_f64[590]=(self.scalar_static_f64[580]*self.scalar_static_f64[588]);
        self.scalar_static_bool[164]=(1.0==self.scalar_static_f64[0]);
        self.scalar_static_bool[165]=(self.scalar_static_f64[1]>0.0);
        self.scalar_static_bool[166]=(self.scalar_static_bool[164]&&self.scalar_static_bool[165]);
        self.scalar_static_f64[591]=(if self.scalar_static_bool[166]{1.0}else{0.0});
        self.scalar_static_f64[592]=p.p6;
        self.scalar_static_f64[593]=p.p7;
        self.scalar_static_f64[594]=p.p250;
        self.scalar_static_f64[595]=p.p99;
        self.scalar_static_f64[596]=p.p97;
        self.scalar_static_f64[597]=p.p98;
        self.scalar_static_f64[598]=p.p108;
        self.scalar_static_f64[599]=p.p110;
        self.scalar_static_f64[600]=p.p109;
        self.scalar_static_f64[601]=p.p111;
        self.scalar_static_f64[602]=p.p119;
        self.scalar_static_f64[603]=p.p83;
        self.scalar_static_f64[604]=p.p135;
        self.scalar_static_f64[605]=(-self.scalar_static_f64[604]);
        self.scalar_static_f64[606]=p.p136;
        self.scalar_static_f64[607]=p.p144;
        self.scalar_static_f64[608]=(-self.scalar_static_f64[607]);
        self.scalar_static_f64[609]=p.p145;
        self.scalar_static_bool[167]=((self.scalar_static_f64[200]!=0.0)&&(self.scalar_static_f64[231]!=0.0));
        self.scalar_static_bool[168]=((self.scalar_static_f64[200]!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_f64[610]=(self.scalar_static_f64[520]*self.scalar_static_f64[592]);
        self.scalar_static_f64[611]=(self.scalar_static_f64[531]*self.scalar_static_f64[592]);
        self.scalar_static_f64[612]=(self.scalar_static_f64[535]*self.scalar_static_f64[592]);
        self.scalar_static_f64[613]=p.p246;
        self.scalar_static_f64[614]=p.p251;
        self.scalar_static_f64[615]=p.p247;
        self.scalar_static_f64[616]=(self.scalar_static_f64[593]*self.scalar_static_f64[615]);
        self.scalar_static_f64[617]=p.p252;
        self.scalar_static_f64[618]=p.p248;
        self.scalar_static_f64[619]=p.p253;
        self.scalar_static_f64[620]=p.p249;
        self.scalar_static_f64[621]=p.p254;
        self.scalar_static_f64[622]=(self.scalar_static_f64[207]*self.scalar_static_f64[571]);
        self.scalar_static_f64[623]=p.p33;
        self.scalar_static_f64[624]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[625]=(-self.scalar_static_f64[28]);
        self.scalar_static_f64[626]=(if self.scalar_static_bool[13]{self.scalar_static_f64[29]}else{0.0});
        self.scalar_static_f64[627]=(if self.scalar_static_bool[13]{self.scalar_static_f64[31]}else{0.0});
        self.scalar_static_f64[628]=(if self.scalar_static_bool[13]{self.scalar_static_f64[32]}else{0.0});
        self.scalar_static_f64[629]=(if self.scalar_static_bool[13]{self.scalar_static_f64[34]}else{0.0});
        self.scalar_static_f64[630]=(if self.scalar_static_bool[13]{self.scalar_static_f64[35]}else{0.0});
        self.scalar_static_f64[631]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_f64[632]=(if self.scalar_static_bool[16]{-1.0}else{0.0});
        self.scalar_static_f64[633]=(self.scalar_static_f64[37]*self.scalar_static_f64[631]);
        self.scalar_static_f64[634]=(self.scalar_static_f64[37]*self.scalar_static_f64[632]);
        self.scalar_static_f64[635]=(self.scalar_static_f64[36]*self.scalar_static_f64[633]);
        self.scalar_static_f64[636]=(-self.scalar_static_f64[635]);
        self.scalar_static_f64[637]=(self.scalar_static_f64[36]*self.scalar_static_f64[634]);
        self.scalar_static_f64[638]=(-self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=(self.scalar_static_f64[38]*self.scalar_static_f64[631]);
        self.scalar_static_f64[640]=(self.scalar_static_f64[38]*self.scalar_static_f64[632]);
        self.scalar_static_f64[641]=(if self.scalar_static_bool[16]{self.scalar_static_f64[639]}else{0.0});
        self.scalar_static_f64[642]=(if self.scalar_static_bool[16]{self.scalar_static_f64[640]}else{0.0});
        self.scalar_static_f64[643]=(-2.0/self.scalar_static_f64[41]);
        self.scalar_static_f64[644]=(2.0/self.scalar_static_f64[41]);
        self.scalar_static_f64[645]=(1.0/self.scalar_static_f64[45]);
        self.scalar_static_f64[646]=(if self.scalar_static_bool[16]{self.scalar_static_f64[645]}else{0.0});
        self.scalar_static_f64[647]=(1.0/self.scalar_static_f64[8]);
        self.scalar_static_f64[648]=(self.scalar_static_f64[46]-1.0);
        self.scalar_static_f64[649]=(1.0/self.scalar_static_f64[49]);
        self.scalar_static_f64[650]=(-1.0/self.scalar_static_f64[49]);
        self.scalar_static_f64[651]=(1.0/self.scalar_static_f64[52]);
        self.scalar_static_f64[652]=(-1.0/self.scalar_static_f64[52]);
        self.scalar_static_f64[653]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_f64[654]=(-self.scalar_static_f64[79]);
        self.scalar_static_f64[655]=(-self.scalar_static_f64[80]);
        self.scalar_static_f64[656]=(self.scalar_static_f64[79]+self.scalar_static_f64[80]);
        self.scalar_static_f64[657]=(8.617087e-5*self.scalar_static_f64[85]);
        self.scalar_static_f64[658]=(-self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=(-self.scalar_static_f64[88]);
        self.scalar_static_f64[660]=(-self.scalar_static_f64[89]);
        self.scalar_static_f64[661]=(self.scalar_static_f64[88]+self.scalar_static_f64[89]);
        self.scalar_static_f64[662]=(8.617087e-5*self.scalar_static_f64[94]);
        self.scalar_static_f64[663]=(-self.scalar_static_f64[662]);
        self.scalar_static_f64[664]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[665]=(self.scalar_static_f64[53]*self.scalar_static_f64[664]);
        self.scalar_static_f64[666]=(if self.scalar_static_bool[22]{self.scalar_static_f64[665]}else{0.0});
        self.scalar_static_f64[667]=(if self.scalar_static_bool[22]{self.scalar_static_f64[665]}else{self.scalar_static_f64[666]});
        self.scalar_static_f64[668]=(if self.scalar_static_bool[22]{self.scalar_static_f64[665]}else{self.scalar_static_f64[667]});
        self.scalar_static_f64[669]=(self.scalar_static_f64[58]*self.scalar_static_f64[664]);
        self.scalar_static_f64[670]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[668]});
        self.scalar_static_f64[671]=(if self.scalar_static_bool[22]{self.scalar_static_f64[669]}else{0.0});
        self.scalar_static_f64[672]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[670]});
        self.scalar_static_f64[673]=(if self.scalar_static_bool[22]{self.scalar_static_f64[669]}else{self.scalar_static_f64[671]});
        self.scalar_static_f64[674]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[672]});
        self.scalar_static_f64[675]=(if self.scalar_static_bool[22]{self.scalar_static_f64[669]}else{self.scalar_static_f64[673]});
        self.scalar_static_f64[676]=(self.scalar_static_f64[114]*self.scalar_static_f64[647]);
        self.scalar_static_f64[677]=(-self.scalar_static_f64[676]);
        self.scalar_static_f64[678]=(self.scalar_static_f64[115]*self.scalar_static_f64[647]);
        self.scalar_static_f64[679]=(self.scalar_static_f64[135]-1.0);
        self.scalar_static_f64[680]=(self.scalar_static_f64[136]-1.0);
        self.scalar_static_f64[681]=(self.scalar_static_f64[142]-1.0);
        self.scalar_static_f64[682]=(self.scalar_static_f64[143]-1.0);
        self.scalar_static_f64[683]=(self.scalar_static_f64[149]*self.scalar_static_f64[647]);
        self.scalar_static_f64[684]=(self.scalar_static_f64[148]*self.scalar_static_f64[683]);
        self.scalar_static_f64[685]=(self.scalar_static_f64[151]*self.scalar_static_f64[647]);
        self.scalar_static_f64[686]=(self.scalar_static_f64[150]*self.scalar_static_f64[685]);
        self.scalar_static_f64[687]=(self.scalar_static_f64[153]*self.scalar_static_f64[647]);
        self.scalar_static_f64[688]=(self.scalar_static_f64[152]*self.scalar_static_f64[687]);
        self.scalar_static_f64[689]=(-self.scalar_static_f64[688]);
        self.scalar_static_f64[690]=(self.scalar_static_f64[158]-1.0);
        self.scalar_static_f64[691]=(self.scalar_static_f64[169]*self.scalar_static_f64[647]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[175]*self.scalar_static_f64[647]);
        self.scalar_static_f64[693]=(self.scalar_static_f64[177]*self.scalar_static_f64[647]);
        self.scalar_static_f64[694]=(if self.scalar_static_bool[32]{self.scalar_static_f64[693]}else{0.0});
        self.scalar_static_f64[695]=(self.scalar_static_f64[178]*self.scalar_static_f64[647]);
        self.scalar_static_f64[696]=(if self.scalar_static_bool[32]{self.scalar_static_f64[695]}else{0.0});
        self.scalar_static_f64[697]=(self.scalar_static_f64[180]*self.scalar_static_f64[647]);
        self.scalar_static_f64[698]=(if self.scalar_static_bool[32]{self.scalar_static_f64[697]}else{0.0});
        self.scalar_static_f64[699]=(-self.scalar_static_f64[694]);
        self.scalar_static_f64[700]=(8.617087e-5*self.scalar_static_f64[696]);
        self.scalar_static_f64[701]=(self.scalar_static_f64[8]*self.scalar_static_f64[700]);
        self.scalar_static_f64[702]=(8.617087e-5*self.scalar_static_f64[698]);
        self.scalar_static_f64[703]=(self.scalar_static_f64[8]*self.scalar_static_f64[702]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[183]*self.scalar_static_f64[647]);
        self.scalar_static_f64[705]=(self.scalar_static_f64[185]*self.scalar_static_f64[647]);
        self.scalar_static_f64[706]=(if self.scalar_static_bool[32]{self.scalar_static_f64[705]}else{0.0});
        self.scalar_static_f64[707]=(self.scalar_static_f64[186]*self.scalar_static_f64[647]);
        self.scalar_static_f64[708]=(if self.scalar_static_bool[32]{self.scalar_static_f64[707]}else{0.0});
        self.scalar_static_f64[709]=(self.scalar_static_f64[188]*self.scalar_static_f64[647]);
        self.scalar_static_f64[710]=(if self.scalar_static_bool[32]{self.scalar_static_f64[709]}else{0.0});
        self.scalar_static_f64[711]=(-self.scalar_static_f64[706]);
        self.scalar_static_f64[712]=(8.617087e-5*self.scalar_static_f64[708]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[8]*self.scalar_static_f64[712]);
        self.scalar_static_f64[714]=(8.617087e-5*self.scalar_static_f64[710]);
        self.scalar_static_f64[715]=(self.scalar_static_f64[8]*self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(self.scalar_static_f64[191]*self.scalar_static_f64[647]);
        self.scalar_static_f64[717]=(if self.scalar_static_bool[35]{self.scalar_static_f64[693]}else{self.scalar_static_f64[694]});
        self.scalar_static_f64[718]=(if self.scalar_static_bool[35]{self.scalar_static_f64[695]}else{self.scalar_static_f64[696]});
        self.scalar_static_f64[719]=(if self.scalar_static_bool[35]{self.scalar_static_f64[697]}else{self.scalar_static_f64[698]});
        self.scalar_static_f64[720]=(self.scalar_static_f64[193]-1.0);
        self.scalar_static_f64[721]=(-self.scalar_static_f64[717]);
        self.scalar_static_f64[722]=(if self.scalar_static_bool[35]{self.scalar_static_f64[705]}else{self.scalar_static_f64[706]});
        self.scalar_static_f64[723]=(if self.scalar_static_bool[35]{self.scalar_static_f64[707]}else{self.scalar_static_f64[708]});
        self.scalar_static_f64[724]=(if self.scalar_static_bool[35]{self.scalar_static_f64[709]}else{self.scalar_static_f64[710]});
        self.scalar_static_f64[725]=(self.scalar_static_f64[195]-1.0);
        self.scalar_static_f64[726]=(-self.scalar_static_f64[722]);
        self.scalar_static_f64[727]=(if self.scalar_static_bool[38]{self.scalar_static_f64[693]}else{self.scalar_static_f64[717]});
        self.scalar_static_f64[728]=(if self.scalar_static_bool[38]{self.scalar_static_f64[695]}else{self.scalar_static_f64[718]});
        self.scalar_static_f64[729]=(if self.scalar_static_bool[38]{self.scalar_static_f64[697]}else{self.scalar_static_f64[719]});
        self.scalar_static_f64[730]=(-self.scalar_static_f64[727]);
        self.scalar_static_f64[731]=(if self.scalar_static_bool[38]{self.scalar_static_f64[705]}else{self.scalar_static_f64[722]});
        self.scalar_static_f64[732]=(if self.scalar_static_bool[38]{self.scalar_static_f64[707]}else{self.scalar_static_f64[723]});
        self.scalar_static_f64[733]=(if self.scalar_static_bool[38]{self.scalar_static_f64[709]}else{self.scalar_static_f64[724]});
        self.scalar_static_f64[734]=(-self.scalar_static_f64[731]);
        self.scalar_static_f64[735]=(self.scalar_static_f64[201]*self.scalar_static_f64[647]);
        self.scalar_static_f64[736]=(-self.scalar_static_f64[735]);
        self.scalar_static_f64[737]=(self.scalar_static_f64[72]*self.scalar_static_f64[736]);
        self.scalar_static_f64[738]=(if (self.scalar_static_f64[200]!=0.0){self.scalar_static_f64[737]}else{0.0});
        self.scalar_static_f64[739]=(self.scalar_static_f64[206]-1.0);
        self.scalar_static_f64[740]=(self.scalar_static_f64[209]-1.0);
        self.scalar_static_f64[741]=(self.scalar_static_f64[217]-1.0);
        self.scalar_static_f64[742]=(self.scalar_static_f64[219]*self.scalar_static_f64[647]);
        self.scalar_static_f64[743]=(self.scalar_static_f64[218]*self.scalar_static_f64[742]);
        self.scalar_static_f64[744]=(if (self.scalar_static_f64[200]!=0.0){self.scalar_static_f64[743]}else{0.0});
        self.scalar_static_f64[745]=(self.scalar_static_f64[744]/self.scalar_static_f64[207]);
        self.scalar_static_f64[746]=(self.scalar_static_f64[76]*self.scalar_static_f64[736]);
        self.scalar_static_f64[747]=(self.scalar_static_f64[222]-1.0);
        self.scalar_static_f64[748]=(self.scalar_static_f64[228]-1.0);
        self.scalar_static_f64[749]=(self.scalar_static_f64[230]*self.scalar_static_f64[647]);
        self.scalar_static_f64[750]=(self.scalar_static_f64[229]*self.scalar_static_f64[749]);
        self.scalar_static_f64[751]=(if (self.scalar_static_f64[200]!=0.0){self.scalar_static_f64[750]}else{0.0});
        self.scalar_static_f64[752]=(self.scalar_static_f64[751]/self.scalar_static_f64[207]);
        self.scalar_static_f64[753]=(if self.scalar_static_bool[50]{-1.0}else{0.0});
        self.scalar_static_f64[754]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[755]=(if self.scalar_static_bool[52]{-1.0}else{0.0});
        self.scalar_static_f64[756]=(if self.scalar_static_bool[52]{1.0}else{0.0});
        self.scalar_static_f64[757]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_f64[758]=(if self.scalar_static_bool[54]{-1.0}else{self.scalar_static_f64[755]});
        self.scalar_static_f64[759]=(if self.scalar_static_bool[54]{0.0}else{self.scalar_static_f64[756]});
        self.scalar_static_f64[760]=(self.scalar_static_f64[239]*self.scalar_static_f64[647]);
        self.scalar_static_f64[761]=(self.scalar_static_f64[265]-1.0);
        self.scalar_static_f64[762]=(if self.scalar_static_bool[59]{0.0}else{self.scalar_static_f64[757]});
        self.scalar_static_f64[763]=(if self.scalar_static_bool[59]{-1.0}else{self.scalar_static_f64[758]});
        self.scalar_static_f64[764]=(if self.scalar_static_bool[59]{1.0}else{self.scalar_static_f64[759]});
        self.scalar_static_f64[765]=(if self.scalar_static_bool[60]{1.0}else{self.scalar_static_f64[762]});
        self.scalar_static_f64[766]=(if self.scalar_static_bool[60]{-1.0}else{self.scalar_static_f64[763]});
        self.scalar_static_f64[767]=(if self.scalar_static_bool[60]{0.0}else{self.scalar_static_f64[764]});
        self.scalar_static_f64[768]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_f64[769]=(if self.scalar_static_bool[63]{-1.0}else{0.0});
        self.scalar_static_f64[770]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[771]=(if self.scalar_static_bool[65]{-1.0}else{0.0});
        self.scalar_static_f64[772]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_f64[773]=(if self.scalar_static_bool[67]{0.0}else{self.scalar_static_f64[770]});
        self.scalar_static_f64[774]=(if self.scalar_static_bool[67]{-1.0}else{self.scalar_static_f64[771]});
        self.scalar_static_f64[775]=(if self.scalar_static_bool[71]{0.0}else{self.scalar_static_f64[772]});
        self.scalar_static_f64[776]=(if self.scalar_static_bool[71]{-1.0}else{0.0});
        self.scalar_static_f64[777]=(if self.scalar_static_bool[71]{1.0}else{self.scalar_static_f64[773]});
        self.scalar_static_f64[778]=(if self.scalar_static_bool[71]{0.0}else{self.scalar_static_f64[774]});
        self.scalar_static_f64[779]=(if self.scalar_static_bool[72]{1.0}else{self.scalar_static_f64[775]});
        self.scalar_static_f64[780]=(if self.scalar_static_bool[72]{-1.0}else{self.scalar_static_f64[776]});
        self.scalar_static_f64[781]=(if self.scalar_static_bool[72]{0.0}else{self.scalar_static_f64[777]});
        self.scalar_static_f64[782]=(if self.scalar_static_bool[72]{0.0}else{self.scalar_static_f64[778]});
        self.scalar_static_f64[783]=(if self.scalar_static_bool[75]{-1.0}else{0.0});
        self.scalar_static_f64[784]=(if self.scalar_static_bool[75]{1.0}else{0.0});
        self.scalar_static_f64[785]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_f64[786]=(if self.scalar_static_bool[77]{-1.0}else{0.0});
        self.scalar_static_f64[787]=(if self.scalar_static_bool[79]{1.0}else{0.0});
        self.scalar_static_f64[788]=(if self.scalar_static_bool[79]{0.0}else{self.scalar_static_f64[785]});
        self.scalar_static_f64[789]=(if self.scalar_static_bool[79]{-1.0}else{self.scalar_static_f64[786]});
        self.scalar_static_f64[790]=(self.scalar_static_f64[307]*self.scalar_static_f64[647]);
        self.scalar_static_f64[791]=(-self.scalar_static_f64[790]);
        self.scalar_static_f64[792]=(self.scalar_static_f64[333]-1.0);
        self.scalar_static_f64[793]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[787]});
        self.scalar_static_f64[794]=(if self.scalar_static_bool[83]{-1.0}else{0.0});
        self.scalar_static_f64[795]=(if self.scalar_static_bool[83]{1.0}else{self.scalar_static_f64[788]});
        self.scalar_static_f64[796]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[789]});
        self.scalar_static_f64[797]=(if self.scalar_static_bool[84]{1.0}else{self.scalar_static_f64[793]});
        self.scalar_static_f64[798]=(if self.scalar_static_bool[84]{-1.0}else{self.scalar_static_f64[794]});
        self.scalar_static_f64[799]=(if self.scalar_static_bool[84]{0.0}else{self.scalar_static_f64[795]});
        self.scalar_static_f64[800]=(if self.scalar_static_bool[84]{0.0}else{self.scalar_static_f64[796]});
        self.scalar_static_f64[801]=(if self.scalar_static_bool[82]{self.scalar_static_f64[798]}else{0.0});
        self.scalar_static_f64[802]=(if self.scalar_static_bool[87]{1.0}else{0.0});
        self.scalar_static_f64[803]=(if self.scalar_static_bool[87]{-1.0}else{0.0});
        self.scalar_static_f64[804]=(if self.scalar_static_bool[89]{1.0}else{0.0});
        self.scalar_static_f64[805]=(if self.scalar_static_bool[89]{-1.0}else{0.0});
        self.scalar_static_f64[806]=(if self.scalar_static_bool[91]{1.0}else{0.0});
        self.scalar_static_f64[807]=(if self.scalar_static_bool[91]{0.0}else{self.scalar_static_f64[804]});
        self.scalar_static_f64[808]=(if self.scalar_static_bool[91]{-1.0}else{self.scalar_static_f64[805]});
        self.scalar_static_f64[809]=(if self.scalar_static_bool[95]{0.0}else{self.scalar_static_f64[806]});
        self.scalar_static_f64[810]=(if self.scalar_static_bool[95]{-1.0}else{0.0});
        self.scalar_static_f64[811]=(if self.scalar_static_bool[95]{1.0}else{self.scalar_static_f64[807]});
        self.scalar_static_f64[812]=(if self.scalar_static_bool[95]{0.0}else{self.scalar_static_f64[808]});
        self.scalar_static_f64[813]=(if self.scalar_static_bool[96]{1.0}else{self.scalar_static_f64[809]});
        self.scalar_static_f64[814]=(if self.scalar_static_bool[96]{-1.0}else{self.scalar_static_f64[810]});
        self.scalar_static_f64[815]=(if self.scalar_static_bool[96]{0.0}else{self.scalar_static_f64[811]});
        self.scalar_static_f64[816]=(if self.scalar_static_bool[96]{0.0}else{self.scalar_static_f64[812]});
        self.scalar_static_f64[817]=(if self.scalar_static_bool[94]{self.scalar_static_f64[814]}else{0.0});
        self.scalar_static_f64[818]=(if self.scalar_static_bool[99]{-1.0}else{0.0});
        self.scalar_static_f64[819]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_f64[820]=(if self.scalar_static_bool[101]{1.0}else{0.0});
        self.scalar_static_f64[821]=(if self.scalar_static_bool[101]{-1.0}else{0.0});
        self.scalar_static_f64[822]=(if self.scalar_static_bool[103]{1.0}else{0.0});
        self.scalar_static_f64[823]=(if self.scalar_static_bool[103]{0.0}else{self.scalar_static_f64[820]});
        self.scalar_static_f64[824]=(if self.scalar_static_bool[103]{-1.0}else{self.scalar_static_f64[821]});
        self.scalar_static_f64[825]=(self.scalar_static_f64[375]*self.scalar_static_f64[647]);
        self.scalar_static_f64[826]=(-self.scalar_static_f64[825]);
        self.scalar_static_f64[827]=(self.scalar_static_f64[401]-1.0);
        self.scalar_static_f64[828]=(if self.scalar_static_bool[107]{0.0}else{self.scalar_static_f64[822]});
        self.scalar_static_f64[829]=(if self.scalar_static_bool[107]{-1.0}else{0.0});
        self.scalar_static_f64[830]=(if self.scalar_static_bool[107]{1.0}else{self.scalar_static_f64[823]});
        self.scalar_static_f64[831]=(if self.scalar_static_bool[107]{0.0}else{self.scalar_static_f64[824]});
        self.scalar_static_f64[832]=(if self.scalar_static_bool[108]{1.0}else{self.scalar_static_f64[828]});
        self.scalar_static_f64[833]=(if self.scalar_static_bool[108]{-1.0}else{self.scalar_static_f64[829]});
        self.scalar_static_f64[834]=(if self.scalar_static_bool[108]{0.0}else{self.scalar_static_f64[830]});
        self.scalar_static_f64[835]=(if self.scalar_static_bool[108]{0.0}else{self.scalar_static_f64[831]});
        self.scalar_static_f64[836]=(if self.scalar_static_bool[106]{self.scalar_static_f64[833]}else{0.0});
        self.scalar_static_f64[837]=(if self.scalar_static_bool[111]{1.0}else{0.0});
        self.scalar_static_f64[838]=(if self.scalar_static_bool[111]{-1.0}else{0.0});
        self.scalar_static_f64[839]=(if self.scalar_static_bool[113]{1.0}else{0.0});
        self.scalar_static_f64[840]=(if self.scalar_static_bool[113]{-1.0}else{0.0});
        self.scalar_static_f64[841]=(if self.scalar_static_bool[115]{1.0}else{0.0});
        self.scalar_static_f64[842]=(if self.scalar_static_bool[115]{0.0}else{self.scalar_static_f64[839]});
        self.scalar_static_f64[843]=(if self.scalar_static_bool[115]{-1.0}else{self.scalar_static_f64[840]});
        self.scalar_static_f64[844]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[841]});
        self.scalar_static_f64[845]=(if self.scalar_static_bool[119]{-1.0}else{0.0});
        self.scalar_static_f64[846]=(if self.scalar_static_bool[119]{1.0}else{self.scalar_static_f64[842]});
        self.scalar_static_f64[847]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[843]});
        self.scalar_static_f64[848]=(if self.scalar_static_bool[120]{1.0}else{self.scalar_static_f64[844]});
        self.scalar_static_f64[849]=(if self.scalar_static_bool[120]{-1.0}else{self.scalar_static_f64[845]});
        self.scalar_static_f64[850]=(if self.scalar_static_bool[120]{0.0}else{self.scalar_static_f64[846]});
        self.scalar_static_f64[851]=(if self.scalar_static_bool[120]{0.0}else{self.scalar_static_f64[847]});
        self.scalar_static_f64[852]=(if self.scalar_static_bool[118]{self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[853]=(if self.scalar_static_bool[123]{-1.0}else{0.0});
        self.scalar_static_f64[854]=(if self.scalar_static_bool[123]{1.0}else{0.0});
        self.scalar_static_f64[855]=(if self.scalar_static_bool[125]{1.0}else{0.0});
        self.scalar_static_f64[856]=(if self.scalar_static_bool[125]{-1.0}else{0.0});
        self.scalar_static_f64[857]=(if self.scalar_static_bool[127]{1.0}else{0.0});
        self.scalar_static_f64[858]=(if self.scalar_static_bool[127]{0.0}else{self.scalar_static_f64[855]});
        self.scalar_static_f64[859]=(if self.scalar_static_bool[127]{-1.0}else{self.scalar_static_f64[856]});
        self.scalar_static_f64[860]=(self.scalar_static_f64[443]*self.scalar_static_f64[647]);
        self.scalar_static_f64[861]=(-self.scalar_static_f64[860]);
        self.scalar_static_f64[862]=(self.scalar_static_f64[469]-1.0);
        self.scalar_static_f64[863]=(if self.scalar_static_bool[131]{0.0}else{self.scalar_static_f64[857]});
        self.scalar_static_f64[864]=(if self.scalar_static_bool[131]{-1.0}else{0.0});
        self.scalar_static_f64[865]=(if self.scalar_static_bool[131]{1.0}else{self.scalar_static_f64[858]});
        self.scalar_static_f64[866]=(if self.scalar_static_bool[131]{0.0}else{self.scalar_static_f64[859]});
        self.scalar_static_f64[867]=(if self.scalar_static_bool[132]{1.0}else{self.scalar_static_f64[863]});
        self.scalar_static_f64[868]=(if self.scalar_static_bool[132]{-1.0}else{self.scalar_static_f64[864]});
        self.scalar_static_f64[869]=(if self.scalar_static_bool[132]{0.0}else{self.scalar_static_f64[865]});
        self.scalar_static_f64[870]=(if self.scalar_static_bool[132]{0.0}else{self.scalar_static_f64[866]});
        self.scalar_static_f64[871]=(if self.scalar_static_bool[130]{self.scalar_static_f64[868]}else{0.0});
        self.scalar_static_f64[872]=(if self.scalar_static_bool[135]{1.0}else{0.0});
        self.scalar_static_f64[873]=(if self.scalar_static_bool[135]{-1.0}else{0.0});
        self.scalar_static_f64[874]=(if self.scalar_static_bool[137]{1.0}else{0.0});
        self.scalar_static_f64[875]=(if self.scalar_static_bool[137]{-1.0}else{0.0});
        self.scalar_static_f64[876]=(if self.scalar_static_bool[139]{1.0}else{0.0});
        self.scalar_static_f64[877]=(if self.scalar_static_bool[139]{0.0}else{self.scalar_static_f64[874]});
        self.scalar_static_f64[878]=(if self.scalar_static_bool[139]{-1.0}else{self.scalar_static_f64[875]});
        self.scalar_static_f64[879]=(if self.scalar_static_bool[143]{0.0}else{self.scalar_static_f64[876]});
        self.scalar_static_f64[880]=(if self.scalar_static_bool[143]{-1.0}else{0.0});
        self.scalar_static_f64[881]=(if self.scalar_static_bool[143]{1.0}else{self.scalar_static_f64[877]});
        self.scalar_static_f64[882]=(if self.scalar_static_bool[143]{0.0}else{self.scalar_static_f64[878]});
        self.scalar_static_f64[883]=(if self.scalar_static_bool[144]{1.0}else{self.scalar_static_f64[879]});
        self.scalar_static_f64[884]=(if self.scalar_static_bool[144]{-1.0}else{self.scalar_static_f64[880]});
        self.scalar_static_f64[885]=(if self.scalar_static_bool[144]{0.0}else{self.scalar_static_f64[881]});
        self.scalar_static_f64[886]=(if self.scalar_static_bool[144]{0.0}else{self.scalar_static_f64[882]});
        self.scalar_static_f64[887]=(if self.scalar_static_bool[142]{self.scalar_static_f64[884]}else{0.0});
        self.scalar_static_f64[888]=(-self.scalar_static_f64[537]);
        self.scalar_static_f64[889]=(if (self.scalar_static_f64[521]!=0.0){self.scalar_static_f64[888]}else{0.0});
        self.scalar_static_f64[890]=(if (self.scalar_static_f64[521]!=0.0){self.scalar_static_f64[537]}else{0.0});
        self.scalar_static_f64[891]=(-self.scalar_static_f64[538]);
        self.scalar_static_f64[892]=(if self.scalar_static_bool[163]{self.scalar_static_f64[537]}else{0.0});
        self.scalar_static_f64[893]=(if self.scalar_static_bool[163]{self.scalar_static_f64[888]}else{self.scalar_static_f64[889]});
        self.scalar_static_f64[894]=(if self.scalar_static_bool[163]{0.0}else{self.scalar_static_f64[890]});
        self.scalar_static_f64[895]=(-self.scalar_static_f64[551]);
        self.scalar_static_f64[896]=(-self.scalar_static_f64[553]);
        self.scalar_static_f64[897]=(-self.scalar_static_f64[555]);
        self.scalar_static_f64[898]=(-self.scalar_static_f64[557]);
        self.scalar_static_f64[899]=(self.scalar_static_f64[559]*self.scalar_static_f64[647]);
        self.scalar_static_f64[900]=(self.scalar_static_f64[561]*self.scalar_static_f64[647]);
        self.scalar_static_f64[901]=(self.scalar_static_f64[563]*self.scalar_static_f64[647]);
        self.scalar_static_f64[902]=(self.scalar_static_f64[565]*self.scalar_static_f64[647]);
        self.scalar_static_f64[903]=(self.scalar_static_f64[567]*self.scalar_static_f64[647]);
        self.scalar_static_f64[904]=(self.scalar_static_f64[569]*self.scalar_static_f64[647]);
        self.scalar_static_f64[905]=(-self.scalar_static_f64[902]);
        self.scalar_static_f64[906]=(-self.scalar_static_f64[899]);
        self.scalar_static_f64[907]=(self.scalar_static_f64[572]*self.scalar_static_f64[647]);
        self.scalar_static_f64[908]=(self.scalar_static_f64[575]*self.scalar_static_f64[647]);
        self.scalar_static_f64[909]=(-self.scalar_static_f64[908]);
        self.scalar_static_f64[910]=(-self.scalar_static_f64[577]);
        self.scalar_static_f64[911]=(self.scalar_static_f64[579]*self.scalar_static_f64[647]);
        self.scalar_static_f64[912]=(-self.scalar_static_f64[911]);
        self.scalar_static_f64[913]=(self.scalar_static_f64[586]*self.scalar_static_f64[912]);
        self.scalar_static_f64[914]=(self.scalar_static_f64[588]*self.scalar_static_f64[912]);
        self.scalar_static_f64[915]=(1.0/self.scalar_static_f64[597]);
        self.scalar_static_f64[916]=(if self.scalar_static_bool[10]{self.scalar_static_f64[915]}else{0.0});
        self.scalar_static_f64[917]=(1.0/self.scalar_static_f64[598]);
        self.scalar_static_f64[918]=(if self.scalar_static_bool[13]{self.scalar_static_f64[917]}else{0.0});
        self.scalar_static_f64[919]=(1.0/self.scalar_static_f64[600]);
        self.scalar_static_f64[920]=(if self.scalar_static_bool[13]{self.scalar_static_f64[919]}else{0.0});
        self.scalar_static_f64[921]=(if self.scalar_static_bool[13]{-1.0}else{0.0});
        self.scalar_static_f64[922]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_f64[923]=(1.0/self.scalar_static_f64[602]);
        self.scalar_static_f64[924]=(if self.scalar_static_bool[16]{self.scalar_static_f64[923]}else{0.0});
        self.scalar_static_f64[925]=(if self.scalar_static_bool[19]{self.scalar_static_f64[913]}else{0.0});
        self.scalar_static_f64[926]=(self.scalar_static_f64[592]*self.scalar_static_f64[677]);
        self.scalar_static_f64[927]=(-self.scalar_static_f64[610]);
        self.scalar_static_f64[928]=(if (self.scalar_static_f64[505]!=0.0){self.scalar_static_f64[610]}else{0.0});
        self.scalar_static_f64[929]=(if (self.scalar_static_f64[505]!=0.0){self.scalar_static_f64[927]}else{0.0});
        self.scalar_static_f64[930]=(-self.scalar_static_f64[611]);
        self.scalar_static_f64[931]=(if self.scalar_static_bool[153]{self.scalar_static_f64[611]}else{0.0});
        self.scalar_static_f64[932]=(if self.scalar_static_bool[153]{self.scalar_static_f64[930]}else{0.0});
        self.scalar_static_f64[933]=(-self.scalar_static_f64[612]);
        self.scalar_static_f64[934]=(if self.scalar_static_bool[153]{self.scalar_static_f64[933]}else{0.0});
        self.scalar_static_f64[935]=(if self.scalar_static_bool[153]{self.scalar_static_f64[612]}else{0.0});
        self.scalar_static_f64[936]=(-self.scalar_static_f64[622]);
        self.scalar_static_f64[937]=(1.0/self.scalar_static_f64[1]);
        self.scalar_static_f64[938]=(if (self.scalar_static_f64[591]!=0.0){self.scalar_static_f64[937]}else{0.0});
    }
}
