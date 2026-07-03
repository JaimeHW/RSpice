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
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 57] = [
                5e-6, 5e-6, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-5,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.001,
                0.0, 0.0, 1.0, 1.5, 7000000.0, 9.025e-5, 1e-7, 1.1785,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 57);
            {
                let params = &mut *ptr;
                params.p57 = params.p56;
                validate_finite_parameter("XLDC", params.p57).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 3] = [
                0.0, 1e19, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(58), 3);
            {
                let params = &mut *ptr;
                params.p61 = params.p60;
                validate_finite_parameter("XWDC", params.p61).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 51] = [
                1e-6, 1e-6, 0.0, 0.0, 2.0, 0.0, -1.0, 1.1,
                1e-8, 1e-8, 0.0, 1e17, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.23, 0.0, 1.0, 0.0, 1.0, 0.5,
                0.0, 300.0, 30.0, 0.3, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 2000000000000000.0, 2.0, 0.0, 0.0,
                1.0, 1.0, 1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(62), 51);
            {
                let params = &mut *ptr;
                params.p113 = if (params.p50 > 0.0) { 2.0 } else { 1.0 };
                validate_parameter("BB", params.p113, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 139] = [
                10.0, 10.0, 0.0, 0.01, 20.0, 0.0025, 1.0, 2e-6,
                3.0, 3e-8, 0.5, 0.0, 1.0, 0.8, 0.0, 1.0,
                0.0, 1.0, 0.0, 1.0, -1.0, 0.0, 1.0, 0.002,
                1e-8, 1e-20, 1.5, 0.35, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 5e17, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 2.1e-7, 0.6, 0.0001, 1.0, 2.0, 0.0, 0.0016,
                0.0, 0.0005, 5e-10, 5e-10, 0.33, 0.33, 0.33, 1.0,
                1.0, 1.0, 3e-8, 0.7, 2.0, 1.0, 1.0, 0.0,
                0.01, 0.1, 0.0, 1.0, 0.0, 0.0, 1.0, 5e18,
                0.0, 1.0, 0.0, 0.0, 0.0, 5e-6, 1000000.0, 0.3,
                0.0, 0.2, 0.5, 10000.0, 20000000.0, 0.3, 4.0, 7500.0,
                0.25, 1e-6, 1e-15, 5000000.0, -5000000.0, 5e-16, 1.0, 0.0,
                0.01, 0.005, 10000000000.0, 1e-19, 0.0, 1.0, 27.0, 1e-10,
                0.7, 8e-7, 3.5e-9, 5e-8, 5e-8, 1.1e-7, 3e17, 400000000000000.0,
                0.1, 1e-7, 0.0, 3.5, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(114), 139);
            {
                let params = &mut *ptr;
                params.p253 = ((-5.0) * params.p50);
                validate_finite_parameter("VGSMIN", params.p253).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 29] = [
                25000.0, 1.0, 1.0, 1e-6, 1e-6, 0.0, 0.0, 0.0,
                0.0, 0.0, 1e19, 1000.0, 1000.0, 30000000.0, 30000000.0, 0.0,
                0.0, 1e-6, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(254), 29);
            {
                let params = &mut *ptr;
                params.p283 = params.p237;
                validate_parameter("XJPT", params.p283, Some((0.0, "0.0")), true, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 8] = [
                1e20, 0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 50.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(284), 8);
            {
                let params = &mut *ptr;
                params.p292 = (params.p68 + 1.12);
                validate_finite_parameter("VFBBTP", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(293), 3);
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 296] = [
    ("l", 0), ("w", 1), ("ad", 2), ("as", 3), ("pd", 4), ("ps", 5), ("ngcon", 6), ("xgw", 7), ("xgl", 8), ("nf", 9), ("sa", 10), ("sb", 11), ("sd", 12), ("pdbcp", 13), ("psbcp", 14), ("lod", 15), 
    ("temp", 16), ("dtemp", 17), ("nbt", 18), ("lbt", 19), ("wbtp", 20), ("wbtn", 21), ("abtn", 22), ("abtp", 23), ("coadov", 24), ("coisub", 25), ("cofbe", 26), ("coiigs", 27), ("cogidl", 28), ("coovlp", 29), ("coign", 30), ("coflick", 31), 
    ("cothrml", 32), ("coisti", 33), ("conqs", 34), ("corg", 35), ("coievb", 36), ("cohist", 37), ("coselfheat", 38), ("covbsbiz", 39), ("colgleff", 40), ("coqovsm", 41), ("coqbdsm", 42), ("cobcnode", 43), ("cosubscale", 44), ("coisubfb", 45), ("info", 46), ("qhsmax", 47), 
    ("dvgpsub", 48), ("dvbssub", 49), ("type", 50), ("version", 51), ("vmax", 52), ("bgtmp1", 53), ("bgtmp2", 54), ("eg0", 55), ("xld", 56), ("xldc", 57), ("vfbover", 58), ("nover", 59), ("xwd", 60), ("xwdc", 61), ("saref", 62), ("sbref", 63), 
    ("xqy", 64), ("xqy1", 65), ("xqy2", 66), ("rshg", 67), ("vfbc", 68), ("vbi", 69), ("parl1", 70), ("parl2", 71), ("lp", 72), ("nsubp", 73), ("nsubp0", 74), ("nsubwp", 75), ("wl1", 76), ("wl1p", 77), ("wl2", 78), ("wl2p", 79), 
    ("scp1", 80), ("scp2", 81), ("scp3", 82), ("sc1", 83), ("sc2", 84), ("sc3", 85), ("scr1", 86), ("scr2", 87), ("scr3", 88), ("pgd1", 89), ("pgd2", 90), ("pgd4", 91), ("ndep", 92), ("ninv", 93), ("ninvd", 94), ("muecb0", 95), 
    ("muecb1", 96), ("mueph0", 97), ("muephw", 98), ("muepwp", 99), ("muephl", 100), ("mueplp", 101), ("muephs", 102), ("muepsp", 103), ("vtmp", 104), ("wvth0", 105), ("muesr1", 106), ("muesr0", 107), ("muesrl", 108), ("muesrw", 109), ("mueswp", 110), ("mueslp", 111), 
    ("muetmp", 112), ("bb", 113), ("ddltmax", 114), ("ddltslp", 115), ("ddltict", 116), ("sub1", 117), ("sub2", 118), ("sub1l", 119), ("sub1lp", 120), ("sub2l", 121), ("svds", 122), ("slg", 123), ("svbs", 124), ("svbsl", 125), ("svbslp", 126), ("svgs", 127), 
    ("svgsl", 128), ("svgslp", 129), ("svgsw", 130), ("svgswp", 131), ("slgl", 132), ("slglp", 133), ("vfbsub", 134), ("vfbsubl", 135), ("vfbsublp", 136), ("subdlt", 137), ("hist1", 138), ("hist2", 139), ("qhe1", 140), ("qhe2", 141), ("evb1", 142), ("evb2", 143), 
    ("evb3", 144), ("fvbs", 145), ("ibpc1", 146), ("ibpc2", 147), ("nsti", 148), ("wsti", 149), ("wstil", 150), ("wstilp", 151), ("wstiw", 152), ("wstiwp", 153), ("scsti1", 154), ("scsti2", 155), ("vthsti", 156), ("vdsti", 157), ("muesti1", 158), ("muesti2", 159), 
    ("muesti3", 160), ("nsubpsti1", 161), ("nsubpsti2", 162), ("nsubpsti3", 163), ("nsubcsti1", 164), ("nsubcsti2", 165), ("nsubcsti3", 166), ("tpoly", 167), ("cgbo", 168), ("cgdo", 169), ("cgso", 170), ("ovslp", 171), ("ovmag", 172), ("js0", 173), ("nj", 174), ("xti", 175), 
    ("xti2", 176), ("vdiffj", 177), ("divx", 178), ("cj", 179), ("cjsw", 180), ("cjswg", 181), ("mj", 182), ("mjsw", 183), ("mjswg", 184), ("pb", 185), ("pbsw", 186), ("pbswg", 187), ("lover", 188), ("clm1", 189), ("clm2", 190), ("clm3", 191), 
    ("clm5", 192), ("clm6", 193), ("vover", 194), ("voverp", 195), ("vovers", 196), ("voversp", 197), ("wfc", 198), ("nsubcw", 199), ("nsubcwp", 200), ("nsubcmax", 201), ("nsubcl", 202), ("nsubclp", 203), ("qme1", 204), ("qme2", 205), ("qme3", 206), ("gidl1", 207), 
    ("gidl2", 208), ("gidl3", 209), ("gidl4", 210), ("gidl5", 211), ("gidlvb", 212), ("gleak1", 213), ("gleak2", 214), ("gleak3", 215), ("gleak4", 216), ("gleak5", 217), ("gleak6", 218), ("gleak7", 219), ("glksd1", 220), ("glksd2", 221), ("glksd3", 222), ("glkb1", 223), 
    ("glkb2", 224), ("glkb3", 225), ("vzadd0", 226), ("pzadd0", 227), ("nftrp", 228), ("nfalp", 229), ("cit", 230), ("falph", 231), ("tnom", 232), ("dly1", 233), ("dly2", 234), ("dly3", 235), ("tfox", 236), ("tsoi", 237), ("xj", 238), ("tbox", 239), 
    ("nsubs", 240), ("nsubb", 241), ("rth0", 242), ("cth0", 243), ("ptl", 244), ("ptp", 245), ("pt2", 246), ("ptlp", 247), ("gdl", 248), ("gdlp", 249), ("gdld", 250), ("pt4", 251), ("pt4p", 252), ("vgsmin", 253), ("mueph1", 254), ("nrs", 255), 
    ("nrd", 256), ("ldrift", 257), ("ldrifts", 258), ("cors", 259), ("cord", 260), ("corbulk", 261), ("corbnet", 262), ("rsh", 263), ("novers", 264), ("rdrmue", 265), ("rdrmues", 266), ("rdrvmax", 267), ("rdrvmaxs", 268), ("rdrmuetmp", 269), ("rdrvtmp", 270), ("rdrdjunc", 271), 
    ("rdrbb", 272), ("rdrbbs", 273), ("rdrbbtmp", 274), ("rdrvmaxw", 275), ("rdrvmaxwp", 276), ("rdrvmaxl", 277), ("rdrvmaxlp", 278), ("rdrmuel", 279), ("rdrmuelp", 280), ("copt", 281), ("copspt", 282), ("xjpt", 283), ("njunc", 284), ("mupt", 285), ("vfbpt", 286), ("pslimpt", 287), 
    ("rbulk0", 288), ("rbulkw", 289), ("rbdb", 290), ("rbsb", 291), ("vfbbtp", 292), ("cbtbn", 293), ("cbtbp", 294), ("xwdbt", 295), 
];

const PARAMETER_DISPLAY_NAMES: [&str; 296] = [
    "L", "W", "AD", "AS", "PD", "PS", "NGCON", "XGW", "XGL", "NF", "SA", "SB", "SD", "PDBCP", "PSBCP", "LOD", 
    "TEMP", "DTEMP", "NBT", "LBT", "WBTP", "WBTN", "ABTN", "ABTP", "COADOV", "COISUB", "COFBE", "COIIGS", "COGIDL", "COOVLP", "COIGN", "COFLICK", 
    "COTHRML", "COISTI", "CONQS", "CORG", "COIEVB", "COHIST", "COSELFHEAT", "COVBSBIZ", "COLGLEFF", "COQOVSM", "COQBDSM", "COBCNODE", "COSUBSCALE", "COISUBFB", "INFO", "QHSMAX", 
    "DVGPSUB", "DVBSSUB", "TYPE", "VERSION", "VMAX", "BGTMP1", "BGTMP2", "EG0", "XLD", "XLDC", "VFBOVER", "NOVER", "XWD", "XWDC", "SAREF", "SBREF", 
    "XQY", "XQY1", "XQY2", "RSHG", "VFBC", "VBI", "PARL1", "PARL2", "LP", "NSUBP", "NSUBP0", "NSUBWP", "WL1", "WL1P", "WL2", "WL2P", 
    "SCP1", "SCP2", "SCP3", "SC1", "SC2", "SC3", "SCR1", "SCR2", "SCR3", "PGD1", "PGD2", "PGD4", "NDEP", "NINV", "NINVD", "MUECB0", 
    "MUECB1", "MUEPH0", "MUEPHW", "MUEPWP", "MUEPHL", "MUEPLP", "MUEPHS", "MUEPSP", "VTMP", "WVTH0", "MUESR1", "MUESR0", "MUESRL", "MUESRW", "MUESWP", "MUESLP", 
    "MUETMP", "BB", "DDLTMAX", "DDLTSLP", "DDLTICT", "SUB1", "SUB2", "SUB1L", "SUB1LP", "SUB2L", "SVDS", "SLG", "SVBS", "SVBSL", "SVBSLP", "SVGS", 
    "SVGSL", "SVGSLP", "SVGSW", "SVGSWP", "SLGL", "SLGLP", "VFBSUB", "VFBSUBL", "VFBSUBLP", "SUBDLT", "HIST1", "HIST2", "QHE1", "QHE2", "EVB1", "EVB2", 
    "EVB3", "FVBS", "IBPC1", "IBPC2", "NSTI", "WSTI", "WSTIL", "WSTILP", "WSTIW", "WSTIWP", "SCSTI1", "SCSTI2", "VTHSTI", "VDSTI", "MUESTI1", "MUESTI2", 
    "MUESTI3", "NSUBPSTI1", "NSUBPSTI2", "NSUBPSTI3", "NSUBCSTI1", "NSUBCSTI2", "NSUBCSTI3", "TPOLY", "CGBO", "CGDO", "CGSO", "OVSLP", "OVMAG", "JS0", "NJ", "XTI", 
    "XTI2", "VDIFFJ", "DIVX", "CJ", "CJSW", "CJSWG", "MJ", "MJSW", "MJSWG", "PB", "PBSW", "PBSWG", "LOVER", "CLM1", "CLM2", "CLM3", 
    "CLM5", "CLM6", "VOVER", "VOVERP", "VOVERS", "VOVERSP", "WFC", "NSUBCW", "NSUBCWP", "NSUBCMAX", "NSUBCL", "NSUBCLP", "QME1", "QME2", "QME3", "GIDL1", 
    "GIDL2", "GIDL3", "GIDL4", "GIDL5", "GIDLVB", "GLEAK1", "GLEAK2", "GLEAK3", "GLEAK4", "GLEAK5", "GLEAK6", "GLEAK7", "GLKSD1", "GLKSD2", "GLKSD3", "GLKB1", 
    "GLKB2", "GLKB3", "VZADD0", "PZADD0", "NFTRP", "NFALP", "CIT", "FALPH", "TNOM", "DLY1", "DLY2", "DLY3", "TFOX", "TSOI", "XJ", "TBOX", 
    "NSUBS", "NSUBB", "RTH0", "CTH0", "PTL", "PTP", "PT2", "PTLP", "GDL", "GDLP", "GDLD", "PT4", "PT4P", "VGSMIN", "MUEPH1", "NRS", 
    "NRD", "LDRIFT", "LDRIFTS", "CORS", "CORD", "CORBULK", "CORBNET", "RSH", "NOVERS", "RDRMUE", "RDRMUES", "RDRVMAX", "RDRVMAXS", "RDRMUETMP", "RDRVTMP", "RDRDJUNC", 
    "RDRBB", "RDRBBS", "RDRBBTMP", "RDRVMAXW", "RDRVMAXWP", "RDRVMAXL", "RDRVMAXLP", "RDRMUEL", "RDRMUELP", "COPT", "COPSPT", "XJPT", "NJUNC", "MUPT", "VFBPT", "PSLIMPT", 
    "RBULK0", "RBULKW", "RBDB", "RBSB", "VFBBTP", "CBTBN", "CBTBP", "XWDBT", 
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 296] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, 
    None, Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), 
    None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, 
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, 
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
    Some(ParameterBound { value: 22.0, label: "22.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, 
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), 
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, 
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, 
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 296] = [
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, 
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, 
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
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), 
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    Some(ParameterBound { value: 32.0, label: "32.0" }), None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, 
    None, None, None, None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, 
    None, None, None, None, None, None, None, None, 
];

const PARAMETER_RANGE_FLAGS: [u8; 296] = [
    3, 3, 2, 2, 2, 2, 3, 2, 2, 3, 2, 2, 2, 2, 2, 2, 3, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 2, 0, 0, 2, 2, 
    2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 
    0, 0, 3, 0, 0, 3, 0, 0, 2, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 
    0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 
    2, 2, 2, 0, 0, 0, 0, 2, 0, 3, 3, 3, 3, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 2, 0, 0, 
    2, 2, 2, 2, 0, 0, 0, 0, 
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 296] = [
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[], &[], &[], &[], &[], &[], 
    &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[], 
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
    pub nodes: [usize; 19],
    pub branches: [usize; 20],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 296]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 15]>,
    pub(crate) ddt_state_previous: Box<[f64; 15]>,
    pub(crate) ddt_state_older: Box<[f64; 15]>,
    pub(crate) ddt_state_initialized: Box<[bool; 15]>,
    pub(crate) ddt_derivative_current: Box<[f64; 15]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 15]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 507]>,
    pub(crate) scalar_static_bool: Box<[bool; 128]>,
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
    pub const TERMINAL_COUNT: usize = 6;
    pub const INTERNAL_NODE_COUNT: usize = 13;
    pub const NODE_COUNT: usize = 19;
    pub const INTERNAL_NODE_NAMES: [&str; 13] = ["dp", "sp", "db", "sb", "temp", "gp", "bp", "nqs_qb", "n", "nqs_qd", "nqs_qs", "nqs_qhs", "nqs_qi"];

    pub const BRANCH_COUNT: usize = 20;
    pub const PARAMETER_COUNT: usize = 296;
    pub const VARIABLE_COUNT: usize = 1854;
    pub const DDT_STATE_COUNT: usize = 15;
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
            scalar_static_f64: boxed_zero_f64_array::<507>(),
            scalar_static_bool: boxed_zero_bool_array::<128>(),
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimsoi_va'", name));
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
        self.scalar_static_f64[0]=p.p43;
        self.scalar_static_bool[0]=(0.0==self.scalar_static_f64[0]);
        self.scalar_static_f64[1]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_bool[1]=(1.0==self.scalar_static_f64[0]);
        self.scalar_static_f64[2]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_f64[3]=p.p237;
        self.scalar_static_f64[4]=p.p51;
        self.scalar_static_f64[5]=(self.scalar_static_f64[4]*10.0);
        self.scalar_static_f64[6]=((self.scalar_static_f64[5]).trunc()%(10.0).trunc());
        self.scalar_static_f64[7]=p.p52;
        self.scalar_static_f64[8]=(self.scalar_static_f64[7]*0.01);
        self.scalar_static_f64[9]=p.p73;
        self.scalar_static_f64[10]=(self.scalar_static_f64[9]/1e-6);
        self.scalar_static_f64[11]=p.p201;
        self.scalar_static_f64[12]=(self.scalar_static_f64[11]/1e-6);
        self.scalar_static_f64[13]=p.p240;
        self.scalar_static_f64[14]=(self.scalar_static_f64[13]/1e-6);
        self.scalar_static_f64[15]=p.p241;
        self.scalar_static_f64[16]=(self.scalar_static_f64[15]/1e-6);
        self.scalar_static_f64[17]=p.p242;
        self.scalar_static_f64[18]=(0.01*self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=p.p148;
        self.scalar_static_f64[20]=(self.scalar_static_f64[19]/1e-6);
        self.scalar_static_f64[21]=p.p198;
        self.scalar_static_f64[22]=(self.scalar_static_f64[21]/0.0001);
        self.scalar_static_f64[23]=p.p70;
        self.scalar_static_f64[24]=(0.01*self.scalar_static_f64[23]);
        self.scalar_static_f64[25]=p.p83;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[25]);
        self.scalar_static_f64[26]=p.p84;
        self.scalar_static_f64[27]=(if self.scalar_static_bool[2]{0.0}else{self.scalar_static_f64[26]});
        self.scalar_static_f64[28]=p.p85;
        self.scalar_static_f64[29]=(if self.scalar_static_bool[2]{0.0}else{self.scalar_static_f64[28]});
        self.scalar_static_f64[30]=p.p80;
        self.scalar_static_bool[3]=(0.0==self.scalar_static_f64[30]);
        self.scalar_static_f64[31]=p.p81;
        self.scalar_static_f64[32]=(if self.scalar_static_bool[3]{0.0}else{self.scalar_static_f64[31]});
        self.scalar_static_f64[33]=p.p82;
        self.scalar_static_f64[34]=(if self.scalar_static_bool[2]{0.0}else{self.scalar_static_f64[33]});
        self.scalar_static_f64[35]=p.p232;
        self.scalar_static_f64[36]=(self.scalar_static_f64[35]+273.15);
        self.scalar_static_f64[37]=p.p34;
        self.scalar_static_f64[38]=if param_given[190]{1.0}else{0.0};
        self.scalar_static_f64[39]=p.p190;
        self.scalar_static_f64[40]=(self.scalar_static_f64[3]*self.scalar_static_f64[13]);
        self.scalar_static_f64[41]=(5000000000.0/self.scalar_static_f64[40]);
        self.scalar_static_f64[42]=(if (self.scalar_static_f64[38]!=0.0){self.scalar_static_f64[39]}else{self.scalar_static_f64[41]});
        self.scalar_static_bool[4]=(self.scalar_static_f64[42]<2.1);
        self.scalar_static_bool[5]=(true&&self.scalar_static_bool[4]);
        self.scalar_static_f64[43]=(if self.scalar_static_bool[5]{1.0}else{0.0});
        self.scalar_static_f64[44]=(2.1-self.scalar_static_f64[42]);
        self.scalar_static_f64[45]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[44]}else{0.0});
        self.scalar_static_f64[46]=(self.scalar_static_f64[45]*self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[46]}else{0.0});
        self.scalar_static_f64[48]=(if (self.scalar_static_f64[43]!=0.0){0.010000000000000002}else{0.0});
        self.scalar_static_f64[49]=(if (self.scalar_static_f64[43]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[50]=(self.scalar_static_f64[47]*self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[50]}else{self.scalar_static_f64[49]});
        self.scalar_static_f64[52]=(self.scalar_static_f64[48]*self.scalar_static_f64[49]);
        self.scalar_static_f64[53]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[52]}else{self.scalar_static_f64[49]});
        self.scalar_static_f64[54]=(self.scalar_static_f64[47]*self.scalar_static_f64[51]);
        self.scalar_static_f64[55]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[54]}else{self.scalar_static_f64[51]});
        self.scalar_static_f64[56]=(self.scalar_static_f64[48]*self.scalar_static_f64[53]);
        self.scalar_static_f64[57]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[56]}else{self.scalar_static_f64[53]});
        self.scalar_static_f64[58]=(self.scalar_static_f64[55]+self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[58]}else{0.0});
        self.scalar_static_f64[60]=(if (self.scalar_static_f64[43]!=0.0){self.scalar_static_f64[59]}else{0.0});
        self.scalar_static_bool[6]=(false&&(self.scalar_static_f64[43]!=0.0));
        self.scalar_static_f64[61]=(0.1*self.scalar_static_f64[45]);
        self.scalar_static_f64[62]=p.p55;
        self.scalar_static_f64[63]=(self.scalar_static_f64[36]*1e-7);
        self.scalar_static_f64[64]=(9.025e-5+self.scalar_static_f64[63]);
        self.scalar_static_f64[65]=(self.scalar_static_f64[36]*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[62]-self.scalar_static_f64[65]);
        self.scalar_static_f64[67]=p.p236;
        self.scalar_static_f64[68]=(1.034943e-10/self.scalar_static_f64[3]);
        self.scalar_static_f64[69]=(1.0/self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(3.453133e-11/self.scalar_static_f64[67]);
        self.scalar_static_f64[71]=(self.scalar_static_f64[67]/3.453133e-11);
        self.scalar_static_f64[72]=p.p239;
        self.scalar_static_f64[73]=(3.453133e-11/self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(self.scalar_static_f64[72]/3.453133e-11);
        self.scalar_static_f64[75]=(self.scalar_static_f64[69]+self.scalar_static_f64[74]);
        self.scalar_static_f64[76]=p.p0;
        self.scalar_static_f64[77]=p.p56;
        self.scalar_static_f64[78]=(2.0*self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=(self.scalar_static_f64[76]-self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=p.p40;
        self.scalar_static_bool[7]=(0.0==self.scalar_static_f64[80]);
        self.scalar_static_f64[81]=(if self.scalar_static_bool[7]{self.scalar_static_f64[76]}else{self.scalar_static_f64[79]});
        self.scalar_static_f64[82]=(1000000.0*self.scalar_static_f64[81]);
        self.scalar_static_f64[83]=p.p1;
        self.scalar_static_f64[84]=p.p9;
        self.scalar_static_f64[85]=(self.scalar_static_f64[83]/self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=p.p60;
        self.scalar_static_bool[8]=(self.scalar_static_f64[6]<1.0);
        self.scalar_static_f64[87]=p.p295;
        self.scalar_static_f64[88]=(if self.scalar_static_bool[8]{0.0}else{self.scalar_static_f64[87]});
        self.scalar_static_f64[89]=(2.0*self.scalar_static_f64[86]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[85]-self.scalar_static_f64[89]);
        self.scalar_static_f64[91]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[90]}else{0.0});
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[1]!=0.0));
        self.scalar_static_f64[92]=p.p18;
        self.scalar_static_f64[93]=(self.scalar_static_f64[88]*self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=(self.scalar_static_f64[85]-self.scalar_static_f64[93]);
        self.scalar_static_f64[95]=(2.0-self.scalar_static_f64[92]);
        self.scalar_static_f64[96]=(self.scalar_static_f64[86]*self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=(self.scalar_static_f64[94]-self.scalar_static_f64[96]);
        self.scalar_static_f64[98]=(if self.scalar_static_bool[9]{self.scalar_static_f64[97]}else{self.scalar_static_f64[91]});
        self.scalar_static_f64[99]=(self.scalar_static_f64[84]*self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=(1000000.0*self.scalar_static_f64[85]);
        self.scalar_static_f64[101]=(self.scalar_static_f64[82]*self.scalar_static_f64[100]);
        self.scalar_static_bool[10]=(self.scalar_static_f64[6]>3.0);
        self.scalar_static_bool[11]=(self.scalar_static_f64[10]<self.scalar_static_f64[14]);
        self.scalar_static_bool[12]=(self.scalar_static_bool[10]&&self.scalar_static_bool[11]);
        self.scalar_static_f64[102]=p.p72;
        self.scalar_static_bool[13]=(self.scalar_static_f64[102]>0.0);
        self.scalar_static_bool[14]=(self.scalar_static_bool[12]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[103]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_f64[104]=(if (self.scalar_static_f64[103]!=0.0){self.scalar_static_f64[14]}else{self.scalar_static_f64[10]});
        self.scalar_static_f64[105]=p.p74;
        self.scalar_static_f64[106]=p.p75;
        self.scalar_static_f64[107]=f64::powf(self.scalar_static_f64[100],self.scalar_static_f64[106]);
        self.scalar_static_f64[108]=(self.scalar_static_f64[105]/self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=(1.0+self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=(self.scalar_static_f64[104]*self.scalar_static_f64[109]);
        self.scalar_static_f64[111]=p.p62;
        self.scalar_static_f64[112]=(0.5*self.scalar_static_f64[76]);
        self.scalar_static_f64[113]=(self.scalar_static_f64[111]+self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=(1.0/self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=p.p63;
        self.scalar_static_f64[116]=(self.scalar_static_f64[112]+self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=(1.0/self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(self.scalar_static_f64[114]+self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=(2.0/self.scalar_static_f64[118]);
        self.scalar_static_f64[120]=(self.scalar_static_f64[36]*1.3806226e-23);
        self.scalar_static_f64[121]=(1.6021918e-19/self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[16]*1.6021918e-19);
        self.scalar_static_f64[123]=(1.034943e-10*self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[20]*3.2043836e-19);
        self.scalar_static_f64[125]=(1.034943e-10*self.scalar_static_f64[124]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[125]).sqrt();
        self.scalar_static_f64[127]=(self.scalar_static_f64[20]*self.scalar_static_f64[20]);
        self.scalar_static_f64[128]=(1.0/self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(1.0/self.scalar_static_f64[82]);
        self.scalar_static_f64[130]=(1.0+self.scalar_static_f64[129]);
        self.scalar_static_f64[131]=p.p91;
        self.scalar_static_f64[132]=f64::powf(self.scalar_static_f64[130],self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=p.p89;
        self.scalar_static_f64[134]=(self.scalar_static_f64[132]*self.scalar_static_f64[133]);
        self.scalar_static_f64[135]=p.p68;
        self.scalar_static_f64[136]=p.p78;
        self.scalar_static_f64[137]=p.p79;
        self.scalar_static_f64[138]=f64::powf(self.scalar_static_f64[101],self.scalar_static_f64[137]);
        self.scalar_static_f64[139]=(self.scalar_static_f64[136]/self.scalar_static_f64[138]);
        self.scalar_static_f64[140]=p.p67;
        self.scalar_static_f64[141]=p.p7;
        self.scalar_static_f64[142]=p.p6;
        self.scalar_static_f64[143]=(3.0*self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[98]/self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=(self.scalar_static_f64[141]+self.scalar_static_f64[144]);
        self.scalar_static_f64[146]=(self.scalar_static_f64[140]*self.scalar_static_f64[145]);
        self.scalar_static_f64[147]=p.p8;
        self.scalar_static_f64[148]=(self.scalar_static_f64[76]-self.scalar_static_f64[147]);
        self.scalar_static_f64[149]=(self.scalar_static_f64[142]*self.scalar_static_f64[148]);
        self.scalar_static_f64[150]=(self.scalar_static_f64[84]*self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=(self.scalar_static_f64[146]/self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=p.p44;
        self.scalar_static_bool[15]=(self.scalar_static_f64[152]<=0.0);
        self.scalar_static_f64[153]=(if self.scalar_static_bool[15]{1.0}else{0.0});
        self.scalar_static_f64[154]=p.p130;
        self.scalar_static_f64[155]=p.p131;
        self.scalar_static_f64[156]=f64::powf(self.scalar_static_f64[100],self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=(self.scalar_static_f64[154]/self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=(1.0+self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[158]}else{0.0});
        self.scalar_static_f64[160]=p.p124;
        self.scalar_static_f64[161]=p.p125;
        self.scalar_static_f64[162]=p.p126;
        self.scalar_static_f64[163]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=(self.scalar_static_f64[161]/self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=(1.0+self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=(self.scalar_static_f64[160]*self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[166]}else{0.0});
        self.scalar_static_f64[168]=p.p123;
        self.scalar_static_f64[169]=(self.scalar_static_f64[82]+self.scalar_static_f64[168]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[82]/self.scalar_static_f64[169]);
        self.scalar_static_f64[171]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[170]}else{0.0});
        self.scalar_static_f64[172]=p.p117;
        self.scalar_static_f64[173]=p.p119;
        self.scalar_static_f64[174]=p.p120;
        self.scalar_static_f64[175]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[173]/self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=(1.0+self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[172]*self.scalar_static_f64[177]);
        self.scalar_static_f64[179]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[178]}else{0.0});
        self.scalar_static_f64[180]=p.p118;
        self.scalar_static_f64[181]=p.p121;
        self.scalar_static_f64[182]=(self.scalar_static_f64[181]/self.scalar_static_f64[82]);
        self.scalar_static_f64[183]=(1.0+self.scalar_static_f64[182]);
        self.scalar_static_f64[184]=(self.scalar_static_f64[180]*self.scalar_static_f64[183]);
        self.scalar_static_f64[185]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[184]}else{0.0});
        self.scalar_static_bool[16]=(!(self.scalar_static_f64[153]!=0.0));
        self.scalar_static_f64[186]=(if self.scalar_static_bool[16]{self.scalar_static_f64[156]}else{0.0});
        self.scalar_static_f64[187]=p.p127;
        self.scalar_static_f64[188]=p.p128;
        self.scalar_static_f64[189]=p.p129;
        self.scalar_static_f64[190]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=(self.scalar_static_f64[188]/self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(1.0+self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[187]*self.scalar_static_f64[192]);
        self.scalar_static_f64[194]=(self.scalar_static_f64[154]+self.scalar_static_f64[186]);
        self.scalar_static_f64[195]=(self.scalar_static_f64[186]/self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[193]*self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=(if self.scalar_static_bool[16]{self.scalar_static_f64[196]}else{0.0});
        self.scalar_static_f64[198]=(if self.scalar_static_bool[16]{self.scalar_static_f64[166]}else{self.scalar_static_f64[167]});
        self.scalar_static_f64[199]=p.p132;
        self.scalar_static_f64[200]=p.p133;
        self.scalar_static_f64[201]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[200]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[199]/self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=(1.0+self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=(self.scalar_static_f64[168]*self.scalar_static_f64[203]);
        self.scalar_static_f64[205]=(if self.scalar_static_bool[16]{self.scalar_static_f64[204]}else{self.scalar_static_f64[171]});
        self.scalar_static_f64[206]=(if self.scalar_static_bool[16]{self.scalar_static_f64[178]}else{self.scalar_static_f64[179]});
        self.scalar_static_f64[207]=(if self.scalar_static_bool[16]{self.scalar_static_f64[184]}else{self.scalar_static_f64[185]});
        self.scalar_static_f64[208]=p.p134;
        self.scalar_static_f64[209]=p.p135;
        self.scalar_static_f64[210]=p.p136;
        self.scalar_static_f64[211]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(self.scalar_static_f64[209]/self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(1.0+self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=(self.scalar_static_f64[208]*self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=(if (self.scalar_static_f64[153]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[216]=p.p50;
        self.scalar_static_f64[217]=if param_given[293]{1.0}else{0.0};
        self.scalar_static_f64[218]=if param_given[22]{1.0}else{0.0};
        self.scalar_static_f64[219]=if param_given[16]{1.0}else{0.0};
        self.scalar_static_f64[220]=p.p17;
        self.scalar_static_bool[17]=(0.0==self.scalar_static_f64[220]);
        self.scalar_static_f64[221]=(if self.scalar_static_bool[17]{0.0}else{1.0});
        self.scalar_static_f64[222]=p.p16;
        self.scalar_static_f64[223]=(273.15+self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=(self.scalar_static_f64[18]/self.scalar_static_f64[99]);
        self.scalar_static_f64[225]=p.p10;
        self.scalar_static_bool[18]=(self.scalar_static_f64[225]>0.0);
        self.scalar_static_f64[226]=p.p11;
        self.scalar_static_bool[19]=(self.scalar_static_f64[226]>0.0);
        self.scalar_static_bool[20]=(self.scalar_static_bool[18]&&self.scalar_static_bool[19]);
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[84]);
        self.scalar_static_bool[22]=(self.scalar_static_f64[84]>1.0);
        self.scalar_static_f64[227]=p.p12;
        self.scalar_static_bool[23]=(self.scalar_static_f64[227]>0.0);
        self.scalar_static_bool[24]=(self.scalar_static_bool[22]&&self.scalar_static_bool[23]);
        self.scalar_static_bool[25]=(self.scalar_static_bool[21]||self.scalar_static_bool[24]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[20]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[228]=(if self.scalar_static_bool[26]{1.0}else{0.0});
        self.scalar_static_f64[229]=(2.0*self.scalar_static_f64[84]);
        self.scalar_static_bool[27]=(!(self.scalar_static_f64[228]!=0.0));
        self.scalar_static_f64[230]=p.p162;
        self.scalar_static_f64[231]=(1.0+self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(1.0/self.scalar_static_f64[231]);
        self.scalar_static_f64[233]=p.p161;
        self.scalar_static_f64[234]=p.p163;
        self.scalar_static_f64[235]=(self.scalar_static_f64[233]/self.scalar_static_f64[119]);
        self.scalar_static_f64[236]=f64::powf(self.scalar_static_f64[235],self.scalar_static_f64[234]);
        self.scalar_static_f64[237]=p.p199;
        self.scalar_static_f64[238]=p.p200;
        self.scalar_static_f64[239]=f64::powf(self.scalar_static_f64[100],self.scalar_static_f64[238]);
        self.scalar_static_f64[240]=(self.scalar_static_f64[237]/self.scalar_static_f64[239]);
        self.scalar_static_f64[241]=(1.0+self.scalar_static_f64[240]);
        self.scalar_static_f64[242]=p.p202;
        self.scalar_static_f64[243]=p.p203;
        self.scalar_static_f64[244]=f64::powf(self.scalar_static_f64[82],self.scalar_static_f64[243]);
        self.scalar_static_f64[245]=(self.scalar_static_f64[242]/self.scalar_static_f64[244]);
        self.scalar_static_f64[246]=(1.0+self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=(self.scalar_static_f64[241]*self.scalar_static_f64[246]);
        self.scalar_static_f64[248]=(self.scalar_static_f64[12]/self.scalar_static_f64[14]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]-self.scalar_static_f64[247]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[249]-0.01);
        self.scalar_static_f64[251]=(4.0*self.scalar_static_f64[248]);
        self.scalar_static_f64[252]=(0.01*self.scalar_static_f64[251]);
        self.scalar_static_bool[28]=(self.scalar_static_f64[252]>0.0);
        self.scalar_static_f64[253]=(-self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=(if self.scalar_static_bool[28]{self.scalar_static_f64[252]}else{self.scalar_static_f64[253]});
        self.scalar_static_f64[255]=(self.scalar_static_f64[250]*self.scalar_static_f64[250]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[254]+self.scalar_static_f64[255]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[256]).sqrt();
        self.scalar_static_f64[258]=(self.scalar_static_f64[250]+self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(0.5*self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[248]-self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[14]*self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=p.p165;
        self.scalar_static_f64[263]=(1.0+self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=(1.0/self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=p.p164;
        self.scalar_static_f64[266]=p.p166;
        self.scalar_static_f64[267]=(self.scalar_static_f64[265]/self.scalar_static_f64[119]);
        self.scalar_static_f64[268]=f64::powf(self.scalar_static_f64[267],self.scalar_static_f64[266]);
        self.scalar_static_bool[29]=(self.scalar_static_f64[81]>self.scalar_static_f64[102]);
        self.scalar_static_bool[30]=(self.scalar_static_f64[102]<=0.0);
        self.scalar_static_bool[31]=(self.scalar_static_bool[29]||self.scalar_static_bool[30]);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_f64[270]=(self.scalar_static_f64[81]-self.scalar_static_f64[102]);
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[269]!=0.0));
        self.scalar_static_f64[271]=(self.scalar_static_f64[102]-self.scalar_static_f64[81]);
        self.scalar_static_f64[272]=(2.0*self.scalar_static_f64[102]);
        self.scalar_static_bool[33]=(self.scalar_static_f64[81]<=self.scalar_static_f64[272]);
        self.scalar_static_bool[34]=(self.scalar_static_bool[13]&&self.scalar_static_bool[33]);
        self.scalar_static_f64[273]=(if self.scalar_static_bool[34]{1.0}else{0.0});
        self.scalar_static_bool[35]=(!(self.scalar_static_f64[273]!=0.0));
        self.scalar_static_f64[274]=p.p35;
        self.scalar_static_bool[36]=(1.0==self.scalar_static_f64[274]);
        self.scalar_static_f64[275]=(if self.scalar_static_bool[36]{1.0}else{0.0});
        self.scalar_static_bool[37]=(self.scalar_static_f64[151]>0.001);
        self.scalar_static_f64[276]=(if self.scalar_static_bool[37]{1.0}else{0.0});
        self.scalar_static_bool[38]=((self.scalar_static_f64[275]!=0.0)&&(self.scalar_static_f64[276]!=0.0));
        self.scalar_static_f64[277]=(1.0/self.scalar_static_f64[151]);
        self.scalar_static_f64[278]=(if self.scalar_static_bool[38]{self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_bool[39]=(!(self.scalar_static_f64[276]!=0.0));
        self.scalar_static_bool[40]=((self.scalar_static_f64[275]!=0.0)&&self.scalar_static_bool[39]);
        self.scalar_static_f64[279]=(if self.scalar_static_bool[40]{1000.0}else{self.scalar_static_f64[278]});
        self.scalar_static_bool[41]=(!(self.scalar_static_f64[275]!=0.0));
        self.scalar_static_f64[280]=(if self.scalar_static_bool[41]{1000.0}else{self.scalar_static_f64[279]});
        self.scalar_static_f64[281]=p.p261;
        self.scalar_static_bool[42]=(1.0==self.scalar_static_f64[281]);
        self.scalar_static_f64[282]=(if self.scalar_static_bool[42]{1.0}else{0.0});
        self.scalar_static_f64[283]=p.p289;
        self.scalar_static_f64[284]=(self.scalar_static_f64[99]*self.scalar_static_f64[283]);
        self.scalar_static_f64[285]=p.p288;
        self.scalar_static_f64[286]=(self.scalar_static_f64[284]+self.scalar_static_f64[285]);
        self.scalar_static_f64[287]=(if (self.scalar_static_f64[282]!=0.0){self.scalar_static_f64[286]}else{0.0});
        self.scalar_static_f64[288]=(if (self.scalar_static_f64[282]!=0.0){self.scalar_static_f64[287]}else{0.0});
        self.scalar_static_bool[43]=(self.scalar_static_f64[288]<0.0001);
        self.scalar_static_f64[289]=(if self.scalar_static_bool[43]{1.0}else{0.0});
        self.scalar_static_bool[44]=((self.scalar_static_f64[282]!=0.0)&&(self.scalar_static_f64[289]!=0.0));
        self.scalar_static_f64[290]=(if self.scalar_static_bool[44]{0.0001}else{self.scalar_static_f64[288]});
        self.scalar_static_bool[45]=(!(self.scalar_static_f64[282]!=0.0));
        self.scalar_static_f64[291]=(if self.scalar_static_bool[45]{0.0001}else{self.scalar_static_f64[290]});
        self.scalar_static_f64[292]=p.p262;
        self.scalar_static_bool[46]=(1.0==self.scalar_static_f64[292]);
        self.scalar_static_f64[293]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_f64[294]=p.p290;
        self.scalar_static_bool[47]=(self.scalar_static_f64[294]<0.0001);
        self.scalar_static_f64[295]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_bool[48]=((self.scalar_static_f64[293]!=0.0)&&(self.scalar_static_f64[295]!=0.0));
        self.scalar_static_f64[296]=(if self.scalar_static_bool[48]{10000.0}else{0.0});
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[295]!=0.0));
        self.scalar_static_bool[50]=((self.scalar_static_f64[293]!=0.0)&&self.scalar_static_bool[49]);
        self.scalar_static_f64[297]=(1.0/self.scalar_static_f64[294]);
        self.scalar_static_f64[298]=(1e-6+self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=(if self.scalar_static_bool[50]{self.scalar_static_f64[298]}else{self.scalar_static_f64[296]});
        self.scalar_static_f64[300]=p.p291;
        self.scalar_static_bool[51]=(self.scalar_static_f64[300]<0.0001);
        self.scalar_static_f64[301]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_bool[52]=((self.scalar_static_f64[293]!=0.0)&&(self.scalar_static_f64[301]!=0.0));
        self.scalar_static_f64[302]=(if self.scalar_static_bool[52]{10000.0}else{0.0});
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[301]!=0.0));
        self.scalar_static_bool[54]=((self.scalar_static_f64[293]!=0.0)&&self.scalar_static_bool[53]);
        self.scalar_static_f64[303]=(1.0/self.scalar_static_f64[300]);
        self.scalar_static_f64[304]=(1e-6+self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=(if self.scalar_static_bool[54]{self.scalar_static_f64[304]}else{self.scalar_static_f64[302]});
        self.scalar_static_f64[306]=p.p24;
        self.scalar_static_bool[55]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[306]!=0.0));
        self.scalar_static_f64[307]=p.p19;
        self.scalar_static_f64[308]=p.p22;
        self.scalar_static_f64[309]=p.p21;
        self.scalar_static_f64[310]=(self.scalar_static_f64[84]*self.scalar_static_f64[309]);
        self.scalar_static_f64[311]=(self.scalar_static_f64[307]*self.scalar_static_f64[310]);
        self.scalar_static_f64[312]=(if (self.scalar_static_f64[218]!=0.0){self.scalar_static_f64[308]}else{self.scalar_static_f64[311]});
        self.scalar_static_f64[313]=(if self.scalar_static_bool[55]{self.scalar_static_f64[312]}else{0.0});
        self.scalar_static_bool[56]=(self.scalar_static_f64[313]>0.0);
        self.scalar_static_bool[57]=((self.scalar_static_f64[217]!=0.0)&&self.scalar_static_bool[56]);
        self.scalar_static_f64[314]=(if self.scalar_static_bool[57]{1.0}else{0.0});
        self.scalar_static_bool[58]=(self.scalar_static_bool[55]&&(self.scalar_static_f64[314]!=0.0));
        self.scalar_static_f64[315]=(if self.scalar_static_bool[58]{0.0}else{self.scalar_static_f64[313]});
        self.scalar_static_bool[59]=(!(self.scalar_static_f64[306]!=0.0));
        self.scalar_static_bool[60]=((self.scalar_static_f64[2]!=0.0)&&self.scalar_static_bool[59]);
        self.scalar_static_f64[316]=(if self.scalar_static_bool[60]{0.0}else{self.scalar_static_f64[315]});
        self.scalar_static_bool[61]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_f64[317]=(if self.scalar_static_bool[61]{0.0}else{self.scalar_static_f64[316]});
        self.scalar_static_bool[62]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[37]!=0.0));
        self.scalar_static_bool[63]=(!(self.scalar_static_f64[37]!=0.0));
        self.scalar_static_bool[64]=((self.scalar_static_f64[2]!=0.0)&&self.scalar_static_bool[63]);
        self.scalar_static_bool[65]=((self.scalar_static_f64[37]!=0.0)&&self.scalar_static_bool[61]);
        self.scalar_static_bool[66]=(self.scalar_static_bool[61]&&self.scalar_static_bool[63]);
        self.scalar_static_f64[318]=p.p38;
        self.scalar_static_bool[67]=(self.scalar_static_f64[318]>0.0);
        self.scalar_static_bool[68]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_bool[69]=(self.scalar_static_bool[67]&&self.scalar_static_bool[68]);
        self.scalar_static_f64[319]=p.p53;
        self.scalar_static_f64[320]=p.p54;
        self.scalar_static_f64[321]=(self.scalar_static_f64[66]/2.0);
        self.scalar_static_f64[322]=(self.scalar_static_f64[121]*self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_bool[70]=(!(self.scalar_static_f64[323]!=0.0));
        self.scalar_static_f64[324]=(2.0*self.scalar_static_f64[123]);
        self.scalar_static_f64[325]=(if (self.scalar_static_f64[2]!=0.0){0.4}else{0.0});
        self.scalar_static_f64[326]=(if (self.scalar_static_f64[2]!=0.0){0.8}else{0.0});
        self.scalar_static_f64[327]=(if self.scalar_static_bool[61]{0.8}else{self.scalar_static_f64[325]});
        self.scalar_static_f64[328]=(if self.scalar_static_bool[61]{1.2}else{self.scalar_static_f64[326]});
        self.scalar_static_f64[329]=(0.5*self.scalar_static_f64[328]);
        self.scalar_static_bool[71]=(self.scalar_static_f64[327]>self.scalar_static_f64[329]);
        self.scalar_static_f64[330]=(if self.scalar_static_bool[71]{1.0}else{0.0});
        self.scalar_static_f64[331]=(if (self.scalar_static_f64[330]!=0.0){self.scalar_static_f64[329]}else{self.scalar_static_f64[327]});
        self.scalar_static_f64[332]=(self.scalar_static_f64[328]-self.scalar_static_f64[331]);
        self.scalar_static_f64[333]=p.p226;
        self.scalar_static_bool[72]=(self.scalar_static_f64[6]<3.0);
        self.scalar_static_f64[334]=p.p204;
        self.scalar_static_bool[73]=(0.0==self.scalar_static_f64[334]);
        self.scalar_static_f64[335]=p.p206;
        self.scalar_static_bool[74]=(0.0==self.scalar_static_f64[335]);
        self.scalar_static_bool[75]=(self.scalar_static_bool[73]&&self.scalar_static_bool[74]);
        self.scalar_static_f64[336]=p.p205;
        self.scalar_static_bool[76]=(0.0==self.scalar_static_f64[336]);
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]||self.scalar_static_bool[76]);
        self.scalar_static_f64[337]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_bool[78]=(!(self.scalar_static_f64[337]!=0.0));
        self.scalar_static_f64[338]=(if self.scalar_static_bool[78]{1.0}else{0.0});
        self.scalar_static_bool[79]=(0.0==self.scalar_static_f64[338]);
        self.scalar_static_f64[339]=(if self.scalar_static_bool[79]{1.0}else{0.0});
        self.scalar_static_f64[340]=(if (self.scalar_static_f64[339]!=0.0){self.scalar_static_f64[67]}else{0.0});
        self.scalar_static_f64[341]=(if (self.scalar_static_f64[339]!=0.0){self.scalar_static_f64[70]}else{0.0});
        self.scalar_static_f64[342]=(if (self.scalar_static_f64[339]!=0.0){self.scalar_static_f64[71]}else{0.0});
        self.scalar_static_bool[80]=(!(self.scalar_static_f64[339]!=0.0));
        self.scalar_static_bool[81]=(self.scalar_static_bool[1]||self.scalar_static_bool[72]);
        self.scalar_static_f64[343]=(if self.scalar_static_bool[81]{1.0}else{0.0});
        self.scalar_static_f64[344]=(-self.scalar_static_f64[3]);
        self.scalar_static_f64[345]=(self.scalar_static_f64[3]*self.scalar_static_f64[344]);
        self.scalar_static_bool[82]=(self.scalar_static_f64[6]>2.0);
        self.scalar_static_f64[346]=(if self.scalar_static_bool[82]{1.0}else{0.0});
        self.scalar_static_bool[83]=((self.scalar_static_f64[343]!=0.0)&&(self.scalar_static_f64[346]!=0.0));
        self.scalar_static_bool[84]=(!(self.scalar_static_f64[343]!=0.0));
        self.scalar_static_f64[347]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_f64[348]=(if (self.scalar_static_f64[347]!=0.0){self.scalar_static_f64[3]}else{0.0});
        self.scalar_static_bool[85]=(!(self.scalar_static_f64[347]!=0.0));
        self.scalar_static_bool[86]=(0.0!=self.scalar_static_f64[102]);
        self.scalar_static_f64[349]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_f64[350]=(self.scalar_static_f64[102]*self.scalar_static_f64[102]);
        self.scalar_static_f64[351]=(1.0/self.scalar_static_f64[350]);
        self.scalar_static_f64[352]=(if (self.scalar_static_f64[349]!=0.0){self.scalar_static_f64[351]}else{0.0});
        self.scalar_static_f64[353]=p.p69;
        self.scalar_static_f64[354]=(self.scalar_static_f64[34]/self.scalar_static_f64[102]);
        self.scalar_static_f64[355]=(if (self.scalar_static_f64[349]!=0.0){self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_bool[87]=(!(self.scalar_static_f64[349]!=0.0));
        self.scalar_static_f64[356]=p.p71;
        self.scalar_static_f64[357]=(self.scalar_static_f64[81]-self.scalar_static_f64[356]);
        self.scalar_static_f64[358]=(self.scalar_static_f64[357]*self.scalar_static_f64[357]);
        self.scalar_static_f64[359]=(1.0/self.scalar_static_f64[358]);
        self.scalar_static_f64[360]=(self.scalar_static_f64[29]/self.scalar_static_f64[81]);
        self.scalar_static_f64[361]=p.p86;
        self.scalar_static_bool[88]=(self.scalar_static_f64[361]>0.0);
        self.scalar_static_f64[362]=(if self.scalar_static_bool[88]{1.0}else{0.0});
        self.scalar_static_f64[363]=p.p88;
        self.scalar_static_f64[364]=(2.0*self.scalar_static_f64[363]);
        self.scalar_static_f64[365]=p.p87;
        self.scalar_static_f64[366]=(0.5*self.scalar_static_f64[81]);
        self.scalar_static_f64[367]=(self.scalar_static_f64[24]+self.scalar_static_f64[366]);
        self.scalar_static_f64[368]=(if (self.scalar_static_f64[362]!=0.0){self.scalar_static_f64[367]}else{0.0});
        self.scalar_static_f64[369]=(self.scalar_static_f64[3]*self.scalar_static_f64[361]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[369]/self.scalar_static_f64[368]);
        self.scalar_static_f64[371]=(if (self.scalar_static_f64[362]!=0.0){self.scalar_static_f64[370]}else{0.0});
        self.scalar_static_bool[89]=(!(self.scalar_static_f64[362]!=0.0));
        self.scalar_static_f64[372]=(self.scalar_static_f64[22]/self.scalar_static_f64[98]);
        self.scalar_static_f64[373]=p.p105;
        self.scalar_static_f64[374]=(self.scalar_static_f64[373]/self.scalar_static_f64[100]);
        self.scalar_static_bool[90]=(0.0==self.scalar_static_f64[133]);
        self.scalar_static_f64[375]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_bool[91]=(!(self.scalar_static_f64[375]!=0.0));
        self.scalar_static_f64[376]=(if self.scalar_static_bool[91]{1.0}else{0.0});
        self.scalar_static_bool[92]=(0.0==self.scalar_static_f64[376]);
        self.scalar_static_f64[377]=(if self.scalar_static_bool[92]{1.0}else{0.0});
        self.scalar_static_bool[93]=(!(self.scalar_static_f64[377]!=0.0));
        self.scalar_static_f64[378]=(if self.scalar_static_bool[93]{self.scalar_static_f64[134]}else{0.0});
        self.scalar_static_f64[379]=p.p90;
        self.scalar_static_f64[380]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[3]}else{0.0});
        self.scalar_static_f64[381]=(if (self.scalar_static_f64[1]!=0.0){1.5}else{0.0});
        self.scalar_static_f64[382]=(1.034943e-10/self.scalar_static_f64[380]);
        self.scalar_static_f64[383]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[382]}else{0.0});
        self.scalar_static_f64[384]=(1.0/self.scalar_static_f64[383]);
        self.scalar_static_f64[385]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[384]}else{0.0});
        self.scalar_static_f64[386]=p.p39;
        self.scalar_static_bool[94]=((self.scalar_static_f64[1]!=0.0)&&(self.scalar_static_f64[386]!=0.0));
        self.scalar_static_bool[95]=(!(self.scalar_static_f64[386]!=0.0));
        self.scalar_static_bool[96]=((self.scalar_static_f64[1]!=0.0)&&self.scalar_static_bool[95]);
        self.scalar_static_f64[387]=(self.scalar_static_f64[380]*0.99);
        self.scalar_static_f64[388]=(self.scalar_static_f64[380]/1.034943e-10);
        self.scalar_static_f64[389]=(1.0/self.scalar_static_f64[73]);
        self.scalar_static_f64[390]=(7.0*self.scalar_static_f64[380]);
        self.scalar_static_bool[97]=(self.scalar_static_f64[390]>=0.0);
        self.scalar_static_f64[391]=(self.scalar_static_f64[390]*self.scalar_static_f64[390]);
        self.scalar_static_bool[98]=(self.scalar_static_f64[380]>=0.0);
        self.scalar_static_f64[392]=(self.scalar_static_f64[380]*self.scalar_static_f64[380]);
        self.scalar_static_bool[99]=(true&&(self.scalar_static_f64[1]!=0.0));
        self.scalar_static_f64[393]=(self.scalar_static_f64[74]+self.scalar_static_f64[385]);
        self.scalar_static_f64[394]=p.p25;
        self.scalar_static_bool[100]=(1.0==self.scalar_static_f64[394]);
        self.scalar_static_f64[395]=p.p137;
        self.scalar_static_f64[396]=(2.0*self.scalar_static_f64[98]);
        self.scalar_static_f64[397]=(1.0/self.scalar_static_f64[79]);
        self.scalar_static_f64[398]=(100.0*self.scalar_static_f64[67]);
        self.scalar_static_f64[399]=(100.0*self.scalar_static_f64[99]);
        self.scalar_static_f64[400]=(100.0*self.scalar_static_f64[79]);
        self.scalar_static_f64[401]=p.p36;
        self.scalar_static_bool[101]=(0.0==self.scalar_static_f64[401]);
        self.scalar_static_f64[402]=(if self.scalar_static_bool[101]{1.0}else{0.0});
        self.scalar_static_bool[102]=(!(self.scalar_static_f64[402]!=0.0));
        self.scalar_static_bool[103]=(self.scalar_static_f64[172]<=0.0);
        self.scalar_static_bool[104]=(self.scalar_static_f64[8]<=0.0);
        self.scalar_static_bool[105]=(self.scalar_static_bool[103]||self.scalar_static_bool[104]);
        self.scalar_static_f64[403]=(if self.scalar_static_bool[105]{1.0}else{0.0});
        self.scalar_static_bool[106]=(!(self.scalar_static_f64[403]!=0.0));
        self.scalar_static_f64[404]=p.p122;
        self.scalar_static_f64[405]=(self.scalar_static_f64[159]*self.scalar_static_f64[205]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[82]+self.scalar_static_f64[205]);
        self.scalar_static_f64[407]=(self.scalar_static_f64[82]/self.scalar_static_f64[406]);
        self.scalar_static_f64[408]=(-self.scalar_static_f64[207]);
        self.scalar_static_f64[409]=p.p26;
        self.scalar_static_bool[107]=(1.0==self.scalar_static_f64[409]);
        self.scalar_static_f64[410]=(if self.scalar_static_bool[107]{1.0}else{0.0});
        self.scalar_static_f64[411]=p.p141;
        self.scalar_static_f64[412]=p.p140;
        self.scalar_static_f64[413]=p.p37;
        self.scalar_static_f64[414]=p.p138;
        self.scalar_static_f64[415]=p.p139;
        self.scalar_static_bool[108]=(2.0==self.scalar_static_f64[409]);
        self.scalar_static_bool[109]=(self.scalar_static_bool[100]&&self.scalar_static_bool[108]);
        self.scalar_static_bool[110]=(self.scalar_static_bool[1]&&self.scalar_static_bool[109]);
        self.scalar_static_f64[416]=(if self.scalar_static_bool[110]{1.0}else{0.0});
        self.scalar_static_f64[417]=(self.scalar_static_f64[3]*1.6021918e-19);
        self.scalar_static_f64[418]=(self.scalar_static_f64[99]*self.scalar_static_f64[417]);
        self.scalar_static_bool[111]=(self.scalar_static_f64[414]>0.0);
        self.scalar_static_f64[419]=(if self.scalar_static_bool[111]{self.scalar_static_f64[414]}else{1.0});
        self.scalar_static_f64[420]=p.p27;
        self.scalar_static_f64[421]=p.p225;
        self.scalar_static_f64[422]=p.p224;
        self.scalar_static_f64[423]=(-self.scalar_static_f64[422]);
        self.scalar_static_f64[424]=p.p28;
        self.scalar_static_bool[112]=(0.0==self.scalar_static_f64[424]);
        self.scalar_static_f64[425]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_bool[113]=(!(self.scalar_static_f64[425]!=0.0));
        self.scalar_static_f64[426]=p.p209;
        self.scalar_static_f64[427]=p.p210;
        self.scalar_static_f64[428]=p.p211;
        self.scalar_static_f64[429]=(1.0/self.scalar_static_f64[67]);
        self.scalar_static_f64[430]=(if self.scalar_static_bool[113]{self.scalar_static_f64[429]}else{0.0});
        self.scalar_static_f64[431]=p.p208;
        self.scalar_static_f64[432]=(-self.scalar_static_f64[431]);
        self.scalar_static_f64[433]=p.p207;
        self.scalar_static_f64[434]=p.p212;
        self.scalar_static_f64[435]=(-self.scalar_static_f64[317]);
        self.scalar_static_f64[436]=(if (self.scalar_static_f64[2]!=0.0){self.scalar_static_f64[435]}else{0.0});
        self.scalar_static_f64[437]=(0.0*self.scalar_static_f64[436]);
        self.scalar_static_f64[438]=(if (self.scalar_static_f64[2]!=0.0){self.scalar_static_f64[437]}else{0.0});
        self.scalar_static_f64[439]=(0.5*self.scalar_static_f64[438]);
        self.scalar_static_f64[440]=(if (self.scalar_static_f64[2]!=0.0){self.scalar_static_f64[439]}else{0.0});
        self.scalar_static_f64[441]=(self.scalar_static_f64[438]-self.scalar_static_f64[440]);
        self.scalar_static_f64[442]=p.p235;
        self.scalar_static_f64[443]=(if self.scalar_static_bool[62]{0.5}else{0.0});
        self.scalar_static_bool[114]=(1.0==self.scalar_static_f64[306]);
        self.scalar_static_f64[444]=(if self.scalar_static_bool[114]{1.0}else{0.0});
        self.scalar_static_bool[115]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[444]!=0.0));
        self.scalar_static_f64[445]=(if self.scalar_static_bool[115]{self.scalar_static_f64[441]}else{0.0});
        self.scalar_static_f64[446]=p.p223;
        self.scalar_static_f64[447]=(self.scalar_static_f64[399]*self.scalar_static_f64[446]);
        self.scalar_static_f64[448]=(self.scalar_static_f64[400]*self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=p.p259;
        self.scalar_static_f64[450]=p.p260;
        self.scalar_static_f64[451]=(1.0-self.scalar_static_f64[443]);
        self.scalar_static_bool[116]=(1.0==self.scalar_static_f64[318]);
        self.scalar_static_bool[117]=(self.scalar_static_bool[68]&&self.scalar_static_bool[116]);
        self.scalar_static_f64[452]=(if self.scalar_static_bool[117]{1.0}else{0.0});
        self.scalar_static_f64[453]=(1.0/self.scalar_static_f64[224]);
        self.scalar_static_f64[454]=(if (self.scalar_static_f64[452]!=0.0){self.scalar_static_f64[453]}else{0.0});
        self.scalar_static_bool[118]=(!(self.scalar_static_f64[452]!=0.0));
        self.scalar_static_f64[455]=(if self.scalar_static_bool[118]{0.0}else{self.scalar_static_f64[454]});
        self.scalar_static_bool[119]=(1.0==self.scalar_static_f64[420]);
        self.scalar_static_f64[456]=(if self.scalar_static_bool[119]{1.0}else{0.0});
        self.scalar_static_bool[120]=(self.scalar_static_f64[17]>0.0);
        self.scalar_static_bool[121]=(self.scalar_static_bool[67]&&self.scalar_static_bool[120]);
        self.scalar_static_f64[457]=(if self.scalar_static_bool[121]{1.0}else{0.0});
        self.scalar_static_bool[122]=((self.scalar_static_f64[413]!=0.0)||self.scalar_static_bool[109]);
        self.scalar_static_f64[458]=(if self.scalar_static_bool[122]{1.0}else{0.0});
        self.scalar_static_bool[123]=(!(self.scalar_static_f64[457]!=0.0));
        self.scalar_static_bool[124]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[281]!=0.0));
        self.scalar_static_bool[125]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[292]!=0.0));
        self.scalar_static_bool[126]=((self.scalar_static_f64[2]!=0.0)&&(self.scalar_static_f64[458]!=0.0));
        self.scalar_static_bool[127]=(self.scalar_static_bool[61]&&(self.scalar_static_f64[413]!=0.0));
        self.scalar_static_f64[459]=(self.scalar_static_f64[234]-1.0);
        self.scalar_static_f64[460]=(self.scalar_static_f64[266]-1.0);
        self.scalar_static_f64[461]=(-self.scalar_static_f64[216]);
        self.scalar_static_f64[462]=(if self.scalar_static_bool[62]{1e-5}else{0.0});
        self.scalar_static_f64[463]=(if self.scalar_static_bool[64]{0.0}else{self.scalar_static_f64[462]});
        self.scalar_static_f64[464]=(if self.scalar_static_bool[65]{1e-5}else{0.0});
        self.scalar_static_f64[465]=(if self.scalar_static_bool[65]{1e-5}else{self.scalar_static_f64[463]});
        self.scalar_static_f64[466]=(if self.scalar_static_bool[66]{0.0}else{self.scalar_static_f64[464]});
        self.scalar_static_f64[467]=(if self.scalar_static_bool[66]{0.0}else{self.scalar_static_f64[465]});
        self.scalar_static_f64[468]=(self.scalar_static_f64[461]-self.scalar_static_f64[461]);
        self.scalar_static_f64[469]=(self.scalar_static_f64[463]/0.0);
        self.scalar_static_f64[470]=(if self.scalar_static_bool[62]{self.scalar_static_f64[469]}else{0.0});
        self.scalar_static_f64[471]=(if self.scalar_static_bool[62]{0.0}else{self.scalar_static_f64[466]});
        self.scalar_static_f64[472]=(if self.scalar_static_bool[64]{0.0}else{self.scalar_static_f64[470]});
        self.scalar_static_f64[473]=(if self.scalar_static_bool[64]{0.0}else{self.scalar_static_f64[471]});
        self.scalar_static_f64[474]=(if self.scalar_static_bool[64]{0.0}else{self.scalar_static_f64[467]});
        self.scalar_static_f64[475]=(self.scalar_static_f64[473]/0.0);
        self.scalar_static_f64[476]=(if self.scalar_static_bool[65]{self.scalar_static_f64[475]}else{0.0});
        self.scalar_static_f64[477]=(if self.scalar_static_bool[66]{0.0}else{self.scalar_static_f64[476]});
        self.scalar_static_f64[478]=(if (self.scalar_static_f64[449]!=0.0){f64::NEG_INFINITY}else{0.0});
        self.scalar_static_f64[479]=(if (self.scalar_static_f64[449]!=0.0){f64::INFINITY}else{0.0});
        self.scalar_static_f64[480]=(if (self.scalar_static_f64[450]!=0.0){f64::INFINITY}else{0.0});
        self.scalar_static_f64[481]=(if (self.scalar_static_f64[450]!=0.0){f64::NEG_INFINITY}else{0.0});
        self.scalar_static_f64[482]=(-self.scalar_static_f64[280]);
        self.scalar_static_f64[483]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[280]}else{0.0});
        self.scalar_static_f64[484]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[482]}else{0.0});
        self.scalar_static_f64[485]=(if (self.scalar_static_f64[457]!=0.0){self.scalar_static_f64[455]}else{0.0});
        self.scalar_static_f64[486]=(if (self.scalar_static_f64[457]!=0.0){1e-12}else{0.0});
        self.scalar_static_f64[487]=(if self.scalar_static_bool[123]{10000.0}else{0.0});
        self.scalar_static_f64[488]=(1.0/self.scalar_static_f64[291]);
        self.scalar_static_f64[489]=(-1.0/self.scalar_static_f64[291]);
        self.scalar_static_f64[490]=(if self.scalar_static_bool[124]{self.scalar_static_f64[488]}else{0.0});
        self.scalar_static_f64[491]=(if self.scalar_static_bool[124]{self.scalar_static_f64[489]}else{0.0});
        self.scalar_static_f64[492]=(-self.scalar_static_f64[305]);
        self.scalar_static_f64[493]=(if self.scalar_static_bool[125]{self.scalar_static_f64[305]}else{0.0});
        self.scalar_static_f64[494]=(if self.scalar_static_bool[125]{self.scalar_static_f64[492]}else{0.0});
        self.scalar_static_f64[495]=(-self.scalar_static_f64[299]);
        self.scalar_static_f64[496]=(if self.scalar_static_bool[125]{self.scalar_static_f64[299]}else{0.0});
        self.scalar_static_f64[497]=(if self.scalar_static_bool[125]{self.scalar_static_f64[495]}else{0.0});
        self.scalar_static_f64[498]=(if self.scalar_static_bool[62]{self.scalar_static_f64[472]}else{0.0});
        self.scalar_static_f64[499]=(if self.scalar_static_bool[62]{1e-12}else{0.0});
        self.scalar_static_f64[500]=(if self.scalar_static_bool[126]{1e-12}else{0.0});
        self.scalar_static_f64[501]=(if self.scalar_static_bool[127]{1e-12}else{0.0});
        self.scalar_static_f64[502]=(if self.scalar_static_bool[65]{self.scalar_static_f64[477]}else{0.0});
        self.scalar_static_f64[503]=(if self.scalar_static_bool[65]{1e-12}else{0.0});
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
        self.scalar_static_f64[504]=(if (self.scalar_static_f64[219]!=0.0){self.scalar_static_f64[223]}else{temperature});
        self.scalar_static_f64[505]=(self.scalar_static_f64[220]+self.scalar_static_f64[504]);
        self.scalar_static_f64[506]=(if (self.scalar_static_f64[221]!=0.0){self.scalar_static_f64[505]}else{self.scalar_static_f64[504]});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
