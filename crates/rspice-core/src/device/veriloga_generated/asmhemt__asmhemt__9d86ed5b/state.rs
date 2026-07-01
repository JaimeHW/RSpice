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
    pub p147: f64,
    pub p148: f64,
    pub p149: f64,
    pub p150: f64,
    pub p151: f64,
    pub p152: f64,
    pub p153: f64,
    pub p154: f64,
    pub p155: f64,
    pub p156: f64,
    pub p157: f64,
    pub p158: f64,
    pub p159: f64,
    pub p160: f64,
    pub p161: f64,
    pub p162: f64,
    pub p163: f64,
    pub p164: f64,
    pub p165: f64,
    pub p166: f64,
    pub p167: f64,
    pub p168: f64,
    pub p169: f64,
    pub p170: f64,
    pub p171: f64,
    pub p172: f64,
    pub p173: f64,
    pub p174: f64,
    pub p175: f64,
    pub p176: f64,
    pub p177: f64,
    pub p178: f64,
    pub p179: f64,
    pub p180: f64,
    pub p181: f64,
    pub p182: f64,
    pub p183: f64,
    pub p184: f64,
    pub p185: f64,
    pub p186: f64,
    pub p187: f64,
    pub p188: f64,
    pub p189: f64,
    pub p190: f64,
    pub p191: f64,
    pub p192: f64,
    pub p193: f64,
    pub p194: f64,
    pub p195: f64,
    pub p196: f64,
    pub p197: f64,
    pub p198: f64,
    pub p199: f64,
    pub p200: f64,
    pub p201: f64,
    pub p202: f64,
    pub p203: f64,
    pub p204: f64,
    pub p205: f64,
    pub p206: f64,
    pub p207: f64,
    pub p208: f64,
    pub p209: f64,
    pub p210: f64,
    pub p211: f64,
    pub p212: f64,
    pub p213: f64,
    pub p214: f64,
    pub p215: f64,
    pub p216: f64,
    pub p217: f64,
    pub p218: f64,
    pub p219: f64,
    pub p220: f64,
    pub p221: f64,
    pub p222: f64,
    pub p223: f64,
    pub p224: f64,
    pub p225: f64,
    pub p226: f64,
    pub p227: f64,
    pub p228: f64,
    pub p229: f64,
    pub p230: f64,
    pub p231: f64,
    pub p232: f64,
    pub p233: f64,
    pub p234: f64,
    pub p235: f64,
    pub p236: f64,
    pub p237: f64,
    pub p238: f64,
    pub p239: f64,
    pub p240: f64,
    pub p241: f64,
    pub p242: f64,
    pub p243: f64,
    pub p244: f64,
    pub p245: f64,
    pub p246: f64,
    pub p247: f64,
    pub p248: f64,
    pub p249: f64,
    pub p250: f64,
    pub p251: f64,
    pub p252: f64,
    pub p253: f64,
    pub p254: f64,
    pub p255: f64,
    pub p256: f64,
    pub p257: f64,
    pub p258: f64,
    pub p259: f64,
    pub p260: f64,
    pub p261: f64,
    pub p262: f64,
    pub p263: f64,
    pub p264: f64,
    pub p265: f64,
    pub p266: f64,
    pub p267: f64,
    pub p268: f64,
    pub p269: f64,
    pub p270: f64,
    pub p271: f64,
    pub p272: f64,
    pub p273: f64,
    pub p274: f64,
    pub p275: f64,
    pub p276: f64,
    pub p277: f64,
    pub p278: f64,
    pub p279: f64,
    pub p280: f64,
    pub p281: f64,
    pub p282: f64,
    pub p283: f64,
    pub p284: f64,
    pub p285: f64,
    pub p286: f64,
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
            params.p1 = 2.5e-8;
            params.p2 = 1.64e-6;
            params.p3 = 2.5e-7;
            params.p4 = 0.0002;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 1.0;
            params.p8 = params.p6;
            validate_parameter("mult_fn", params.p8, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p9 = 1.066e-10;
            params.p10 = -2.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.17;
            params.p14 = 0.0;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 190000.0;
            params.p18 = 2.0;
            params.p19 = 0.0;
            params.p20 = -0.5;
            params.p21 = 0.0;
            params.p22 = 1e-9;
            params.p23 = 5.0;
            params.p24 = 0.0;
            params.p25 = 1.0;
            params.p26 = 0.5;
            params.p27 = 0.001;
            params.p28 = 2.12e-12;
            params.p29 = 3.73e-12;
            params.p30 = 1e-15;
            params.p31 = 1.0;
            params.p32 = 5.0;
            params.p33 = 1e-9;
            params.p34 = 0.0;
            params.p35 = 50000.0;
            params.p36 = 5e17;
            params.p37 = 5e17;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.155;
            params.p41 = 0.155;
            params.p42 = 2.0;
            params.p43 = 2.0;
            params.p44 = 1.0;
            params.p45 = 1.0;
            params.p46 = 1e-6;
            params.p47 = 1e-6;
            params.p48 = 0.0001;
            params.p49 = 0.0001;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 0.0;
            params.p57 = 2.5;
            params.p58 = 1.0;
            params.p59 = 1.0;
            params.p60 = 2.5;
            params.p61 = 80.0;
            params.p62 = 80.0;
            params.p63 = 1e-12;
            params.p64 = 1e-12;
            params.p65 = 1e-15;
            params.p66 = 1e-15;
            params.p67 = 0.0001;
            params.p68 = 0.0001;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.0;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 1.0;
            params.p83 = 1e-5;
            params.p84 = 1.0;
            params.p85 = 0.0;
            params.p86 = 0.5;
            params.p87 = 0.0;
            params.p88 = 0.5;
            params.p89 = 20.0;
            params.p90 = 5.0;
            params.p91 = 0.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 1e-6;
            params.p98 = 1000000.0;
            params.p99 = 1.0;
            params.p100 = 0.1;
            params.p101 = 0.3;
            params.p102 = 0.0;
            params.p103 = 0.05;
            params.p104 = 0.1;
            params.p105 = 0.6;
            params.p106 = 0.5;
            params.p107 = 0.6;
            params.p108 = 1.0;
            params.p109 = 1.0;
            params.p110 = 1e-5;
            params.p111 = 1e-6;
            params.p112 = 0.1;
            params.p113 = 1e-9;
            params.p114 = 1e-15;
            params.p115 = 1e-15;
            params.p116 = 1e-12;
            params.p117 = 1e-13;
            params.p118 = 1e-13;
            params.p119 = 1.0;
            params.p120 = 0.0001;
            params.p121 = 10.0;
            params.p122 = 1.0;
            params.p123 = 0.016;
            params.p124 = 2.0;
            params.p125 = 20.0;
            params.p126 = 1.0;
            params.p127 = 250.0;
            params.p128 = 0.01;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.05;
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 10000.0;
            params.p135 = 1e-7;
            params.p136 = 0.5;
            params.p137 = 0.5;
            params.p138 = 0.0;
            params.p139 = 0.0;
            params.p140 = 0.05;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = 10000.0;
            params.p144 = 1e-7;
            params.p145 = 0.5;
            params.p146 = 0.5;
            params.p147 = 0.0;
            params.p148 = 0.0;
            params.p149 = 0.0;
            params.p150 = 0.0;
            params.p151 = 0.0;
            params.p152 = 0.0;
            params.p153 = 0.0;
            params.p154 = 0.0;
            params.p155 = 0.0;
            params.p156 = 0.0;
            params.p157 = 0.0;
            params.p158 = 1e-15;
            params.p159 = -25.0;
            params.p160 = 5e-8;
            params.p161 = 1e-6;
            params.p162 = 0.05;
            params.p163 = 0.1;
            params.p164 = 100000.0;
            params.p165 = 0.5;
            params.p166 = 0.0;
            params.p167 = 1e-9;
            params.p168 = 10.0;
            params.p169 = 2.12e-12;
            params.p170 = 3.73e-12;
            params.p171 = 1e-15;
            params.p172 = -80.0;
            params.p173 = 1e-7;
            params.p174 = 1e-6;
            params.p175 = 0.05;
            params.p176 = 0.1;
            params.p177 = 100000.0;
            params.p178 = 0.5;
            params.p179 = 0.0;
            params.p180 = 1e-9;
            params.p181 = 10.0;
            params.p182 = 2.12e-12;
            params.p183 = 3.73e-12;
            params.p184 = 1e-15;
            params.p185 = -75.0;
            params.p186 = 1.5e-7;
            params.p187 = 1e-6;
            params.p188 = 0.05;
            params.p189 = 0.1;
            params.p190 = 100000.0;
            params.p191 = 0.5;
            params.p192 = 0.0;
            params.p193 = 1e-9;
            params.p194 = 10.0;
            params.p195 = 2.12e-12;
            params.p196 = 3.73e-12;
            params.p197 = 1e-15;
            params.p198 = -100.0;
            params.p199 = 2e-7;
            params.p200 = 1e-6;
            params.p201 = 0.05;
            params.p202 = 0.1;
            params.p203 = 100000.0;
            params.p204 = 0.5;
            params.p205 = 0.0;
            params.p206 = 1e-9;
            params.p207 = 10.0;
            params.p208 = 2.12e-12;
            params.p209 = 3.73e-12;
            params.p210 = 1e-14;
            params.p211 = 1e-14;
            params.p212 = 1e-14;
            params.p213 = 0.0;
            params.p214 = 100.0;
            params.p215 = 0.0;
            params.p216 = 0.0;
            params.p217 = 0.0;
            params.p218 = 0.0;
            params.p219 = 0.0;
            params.p220 = 0.0;
            params.p221 = 1e-24;
            params.p222 = 0.0;
            params.p223 = 0.0;
            params.p224 = 0.9;
            params.p225 = 0.0;
            params.p226 = 0.0;
            params.p227 = 0.0;
            params.p228 = 0.5;
            params.p229 = 0.1;
            params.p230 = 1.0;
            params.p231 = 0.0;
            params.p232 = 1.0;
            params.p233 = 0.001;
            params.p234 = 0.0;
            params.p235 = 1.0;
            params.p236 = 0.001;
            params.p237 = 0.0;
            params.p238 = 1.0;
            params.p239 = 0.001;
            params.p240 = 0.0;
            params.p241 = 1.0;
            params.p242 = 0.001;
            params.p243 = 0.0;
            params.p244 = 1.0;
            params.p245 = 0.001;
            params.p246 = 0.0;
            params.p247 = 0.0;
            params.p248 = 0.0;
            params.p249 = 0.0;
            params.p250 = 0.0;
            params.p251 = 0.0;
            params.p252 = 0.0;
            params.p253 = 0.0;
            params.p254 = 0.0;
            params.p255 = 0.0;
            params.p256 = 0.0;
            params.p257 = 1.0;
            params.p258 = 0.001;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 1.5e-11;
            params.p262 = 0.0;
            params.p263 = 0.0;
            params.p264 = 1.0;
            params.p265 = 1e27;
            params.p266 = 1e-12;
            params.p267 = 0.0;
            params.p268 = 200.0;
            params.p269 = 0.0;
            params.p270 = 10.0;
            params.p271 = 0.0;
            params.p272 = 0.0;
            params.p273 = 0.0;
            params.p274 = 0.0;
            params.p275 = 100.0;
            params.p276 = 100.0;
            params.p277 = 0.0;
            params.p278 = 0.0;
            params.p279 = 50.0;
            params.p280 = 50.0;
            params.p281 = 0.0;
            params.p282 = 0.0;
            params.p283 = 0.0;
            params.p284 = 0.0;
            params.p285 = 0.0;
            params.p286 = 0.0;
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
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: bool,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: bool,
    pub(crate) scalar_v9: bool,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: bool,
    pub(crate) scalar_v13: bool,
    pub(crate) scalar_v14: bool,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: bool,
    pub(crate) scalar_v48: bool,
    pub(crate) scalar_v50: bool,
    pub(crate) scalar_v52: bool,
    pub(crate) scalar_v54: bool,
    pub(crate) scalar_v56: bool,
    pub(crate) scalar_v57: bool,
    pub(crate) scalar_v58: bool,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: bool,
    pub(crate) scalar_v66: bool,
    pub(crate) scalar_v67: bool,
    pub(crate) scalar_v69: bool,
    pub(crate) scalar_v70: bool,
    pub(crate) scalar_v71: bool,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v97: bool,
    pub(crate) scalar_v98: bool,
    pub(crate) scalar_v99: bool,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v125: bool,
    pub(crate) scalar_v126: bool,
    pub(crate) scalar_v127: bool,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: bool,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: bool,
    pub(crate) scalar_v244: f64,
    pub(crate) scalar_v245: bool,
    pub(crate) scalar_v246: bool,
    pub(crate) scalar_v247: bool,
    pub(crate) scalar_v248: bool,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: bool,
    pub(crate) scalar_v251: bool,
    pub(crate) scalar_v252: bool,
    pub(crate) scalar_v253: bool,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: bool,
    pub(crate) scalar_v256: bool,
    pub(crate) scalar_v257: bool,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v259: bool,
    pub(crate) scalar_v260: bool,
    pub(crate) scalar_v261: bool,
    pub(crate) scalar_v262: bool,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: bool,
    pub(crate) scalar_v265: bool,
    pub(crate) scalar_v266: bool,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v269: bool,
    pub(crate) scalar_v270: bool,
    pub(crate) scalar_v271: bool,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: bool,
    pub(crate) scalar_v274: bool,
    pub(crate) scalar_v275: bool,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: bool,
    pub(crate) scalar_v278: bool,
    pub(crate) scalar_v279: bool,
    pub(crate) scalar_v280: bool,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: bool,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: bool,
    pub(crate) scalar_v295: bool,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: bool,
    pub(crate) scalar_v299: bool,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: bool,
    pub(crate) scalar_v302: bool,
    pub(crate) scalar_v303: bool,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: bool,
    pub(crate) scalar_v311: bool,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: bool,
    pub(crate) scalar_v315: bool,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: bool,
    pub(crate) scalar_v318: bool,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: bool,
    pub(crate) scalar_v322: bool,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v327: f64,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v331: f64,
    pub(crate) scalar_v332: f64,
    pub(crate) scalar_v333: f64,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: f64,
    pub(crate) scalar_v336: f64,
    pub(crate) scalar_v337: f64,
    pub(crate) scalar_v341: bool,
    pub(crate) scalar_v342: bool,
    pub(crate) scalar_v343: bool,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v346: bool,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v358: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v365: f64,
    pub(crate) scalar_v370: f64,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: f64,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v385: f64,
    pub(crate) scalar_v387: f64,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: f64,
    pub(crate) scalar_v402: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v417: bool,
    pub(crate) scalar_v418: bool,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: bool,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v425: f64,
    pub(crate) scalar_v426: f64,
    pub(crate) scalar_v427: f64,
    pub(crate) scalar_v428: f64,
    pub(crate) scalar_v429: f64,
    pub(crate) scalar_v430: f64,
    pub(crate) scalar_v431: f64,
    pub(crate) scalar_v432: f64,
    pub(crate) scalar_v433: f64,
    pub(crate) scalar_v434: f64,
    pub(crate) scalar_v435: f64,
    pub(crate) scalar_v439: f64,
    pub(crate) scalar_v440: f64,
    pub(crate) scalar_v444: f64,
    pub(crate) scalar_v448: bool,
    pub(crate) scalar_v449: f64,
    pub(crate) scalar_v452: bool,
    pub(crate) scalar_v453: f64,
    pub(crate) scalar_v468: f64,
    pub(crate) scalar_v469: f64,
    pub(crate) scalar_v470: f64,
    pub(crate) scalar_v471: f64,
    pub(crate) scalar_v472: f64,
    pub(crate) scalar_v473: f64,
    pub(crate) scalar_v476: f64,
    pub(crate) scalar_v477: f64,
    pub(crate) scalar_v487: f64,
    pub(crate) scalar_v488: f64,
    pub(crate) scalar_v489: f64,
    pub(crate) scalar_v490: f64,
    pub(crate) scalar_v508: f64,
    pub(crate) scalar_v509: f64,
    pub(crate) scalar_v510: f64,
    pub(crate) scalar_v524: f64,
    pub(crate) scalar_v525: f64,
    pub(crate) scalar_v539: f64,
    pub(crate) scalar_v540: f64,
    pub(crate) scalar_v541: f64,
    pub(crate) scalar_v551: f64,
    pub(crate) scalar_v552: f64,
    pub(crate) scalar_v558: f64,
    pub(crate) scalar_v559: f64,
    pub(crate) scalar_v560: f64,
    pub(crate) scalar_v570: f64,
    pub(crate) scalar_v571: f64,
    pub(crate) scalar_v576: f64,
    pub(crate) scalar_v577: f64,
    pub(crate) scalar_v578: f64,
    pub(crate) scalar_v579: f64,
    pub(crate) scalar_v580: f64,
    pub(crate) scalar_v581: f64,
    pub(crate) scalar_v582: f64,
    pub(crate) scalar_v676: f64,
    pub(crate) scalar_v677: f64,
    pub(crate) scalar_v678: f64,
    pub(crate) scalar_v695: f64,
    pub(crate) scalar_v696: f64,
    pub(crate) scalar_v697: f64,
    pub(crate) scalar_v698: f64,
    pub(crate) scalar_v699: f64,
    pub(crate) scalar_v700: f64,
    pub(crate) scalar_v701: f64,
    pub(crate) scalar_v702: f64,
    pub(crate) scalar_v703: f64,
    pub(crate) scalar_v704: f64,
    pub(crate) scalar_v709: f64,
    pub(crate) scalar_v795: f64,
    pub(crate) scalar_v796: f64,
    pub(crate) scalar_v797: f64,
    pub(crate) scalar_v798: f64,
    pub(crate) scalar_v799: f64,
    pub(crate) scalar_v800: f64,
    pub(crate) scalar_v801: f64,
    pub(crate) scalar_v802: f64,
    pub(crate) scalar_v803: f64,
    pub(crate) scalar_v804: f64,
    pub(crate) scalar_v805: f64,
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
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v9: self.scalar_v9,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v12: self.scalar_v12,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v18: self.scalar_v18,
            scalar_v42: self.scalar_v42,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v50: self.scalar_v50,
            scalar_v52: self.scalar_v52,
            scalar_v54: self.scalar_v54,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v82: self.scalar_v82,
            scalar_v83: self.scalar_v83,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v106: self.scalar_v106,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v118: self.scalar_v118,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v130: self.scalar_v130,
            scalar_v134: self.scalar_v134,
            scalar_v137: self.scalar_v137,
            scalar_v140: self.scalar_v140,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v153: self.scalar_v153,
            scalar_v155: self.scalar_v155,
            scalar_v158: self.scalar_v158,
            scalar_v161: self.scalar_v161,
            scalar_v164: self.scalar_v164,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v171: self.scalar_v171,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
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
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v325: self.scalar_v325,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v350: self.scalar_v350,
            scalar_v358: self.scalar_v358,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v365: self.scalar_v365,
            scalar_v370: self.scalar_v370,
            scalar_v371: self.scalar_v371,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v384: self.scalar_v384,
            scalar_v385: self.scalar_v385,
            scalar_v387: self.scalar_v387,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v402: self.scalar_v402,
            scalar_v414: self.scalar_v414,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v423: self.scalar_v423,
            scalar_v424: self.scalar_v424,
            scalar_v425: self.scalar_v425,
            scalar_v426: self.scalar_v426,
            scalar_v427: self.scalar_v427,
            scalar_v428: self.scalar_v428,
            scalar_v429: self.scalar_v429,
            scalar_v430: self.scalar_v430,
            scalar_v431: self.scalar_v431,
            scalar_v432: self.scalar_v432,
            scalar_v433: self.scalar_v433,
            scalar_v434: self.scalar_v434,
            scalar_v435: self.scalar_v435,
            scalar_v439: self.scalar_v439,
            scalar_v440: self.scalar_v440,
            scalar_v444: self.scalar_v444,
            scalar_v448: self.scalar_v448,
            scalar_v449: self.scalar_v449,
            scalar_v452: self.scalar_v452,
            scalar_v453: self.scalar_v453,
            scalar_v468: self.scalar_v468,
            scalar_v469: self.scalar_v469,
            scalar_v470: self.scalar_v470,
            scalar_v471: self.scalar_v471,
            scalar_v472: self.scalar_v472,
            scalar_v473: self.scalar_v473,
            scalar_v476: self.scalar_v476,
            scalar_v477: self.scalar_v477,
            scalar_v487: self.scalar_v487,
            scalar_v488: self.scalar_v488,
            scalar_v489: self.scalar_v489,
            scalar_v490: self.scalar_v490,
            scalar_v508: self.scalar_v508,
            scalar_v509: self.scalar_v509,
            scalar_v510: self.scalar_v510,
            scalar_v524: self.scalar_v524,
            scalar_v525: self.scalar_v525,
            scalar_v539: self.scalar_v539,
            scalar_v540: self.scalar_v540,
            scalar_v541: self.scalar_v541,
            scalar_v551: self.scalar_v551,
            scalar_v552: self.scalar_v552,
            scalar_v558: self.scalar_v558,
            scalar_v559: self.scalar_v559,
            scalar_v560: self.scalar_v560,
            scalar_v570: self.scalar_v570,
            scalar_v571: self.scalar_v571,
            scalar_v576: self.scalar_v576,
            scalar_v577: self.scalar_v577,
            scalar_v578: self.scalar_v578,
            scalar_v579: self.scalar_v579,
            scalar_v580: self.scalar_v580,
            scalar_v581: self.scalar_v581,
            scalar_v582: self.scalar_v582,
            scalar_v676: self.scalar_v676,
            scalar_v677: self.scalar_v677,
            scalar_v678: self.scalar_v678,
            scalar_v695: self.scalar_v695,
            scalar_v696: self.scalar_v696,
            scalar_v697: self.scalar_v697,
            scalar_v698: self.scalar_v698,
            scalar_v699: self.scalar_v699,
            scalar_v700: self.scalar_v700,
            scalar_v701: self.scalar_v701,
            scalar_v702: self.scalar_v702,
            scalar_v703: self.scalar_v703,
            scalar_v704: self.scalar_v704,
            scalar_v709: self.scalar_v709,
            scalar_v795: self.scalar_v795,
            scalar_v796: self.scalar_v796,
            scalar_v797: self.scalar_v797,
            scalar_v798: self.scalar_v798,
            scalar_v799: self.scalar_v799,
            scalar_v800: self.scalar_v800,
            scalar_v801: self.scalar_v801,
            scalar_v802: self.scalar_v802,
            scalar_v803: self.scalar_v803,
            scalar_v804: self.scalar_v804,
            scalar_v805: self.scalar_v805,
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
            scalar_v5: 0.0,
            scalar_v6: false,
            scalar_v7: 0.0,
            scalar_v8: false,
            scalar_v9: false,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v12: false,
            scalar_v13: false,
            scalar_v14: false,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v18: 0.0,
            scalar_v42: 0.0,
            scalar_v46: 0.0,
            scalar_v47: false,
            scalar_v48: false,
            scalar_v50: false,
            scalar_v52: false,
            scalar_v54: false,
            scalar_v56: false,
            scalar_v57: false,
            scalar_v58: false,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
            scalar_v65: false,
            scalar_v66: false,
            scalar_v67: false,
            scalar_v69: false,
            scalar_v70: false,
            scalar_v71: false,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v97: false,
            scalar_v98: false,
            scalar_v99: false,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v106: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v118: 0.0,
            scalar_v125: false,
            scalar_v126: false,
            scalar_v127: false,
            scalar_v128: 0.0,
            scalar_v130: 0.0,
            scalar_v134: 0.0,
            scalar_v137: 0.0,
            scalar_v140: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v153: 0.0,
            scalar_v155: 0.0,
            scalar_v158: 0.0,
            scalar_v161: 0.0,
            scalar_v164: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v171: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v240: 0.0,
            scalar_v241: false,
            scalar_v242: 0.0,
            scalar_v243: false,
            scalar_v244: 0.0,
            scalar_v245: false,
            scalar_v246: false,
            scalar_v247: false,
            scalar_v248: false,
            scalar_v249: 0.0,
            scalar_v250: false,
            scalar_v251: false,
            scalar_v252: false,
            scalar_v253: false,
            scalar_v254: 0.0,
            scalar_v255: false,
            scalar_v256: false,
            scalar_v257: false,
            scalar_v258: 0.0,
            scalar_v259: false,
            scalar_v260: false,
            scalar_v261: false,
            scalar_v262: false,
            scalar_v263: 0.0,
            scalar_v264: false,
            scalar_v265: false,
            scalar_v266: false,
            scalar_v267: 0.0,
            scalar_v268: false,
            scalar_v269: false,
            scalar_v270: false,
            scalar_v271: false,
            scalar_v272: 0.0,
            scalar_v273: false,
            scalar_v274: false,
            scalar_v275: false,
            scalar_v276: 0.0,
            scalar_v277: false,
            scalar_v278: false,
            scalar_v279: false,
            scalar_v280: false,
            scalar_v281: 0.0,
            scalar_v282: false,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: false,
            scalar_v295: false,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v298: false,
            scalar_v299: false,
            scalar_v300: 0.0,
            scalar_v301: false,
            scalar_v302: false,
            scalar_v303: false,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: false,
            scalar_v311: false,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v314: false,
            scalar_v315: false,
            scalar_v316: 0.0,
            scalar_v317: false,
            scalar_v318: false,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v321: false,
            scalar_v322: false,
            scalar_v323: 0.0,
            scalar_v325: false,
            scalar_v327: 0.0,
            scalar_v328: 0.0,
            scalar_v331: 0.0,
            scalar_v332: 0.0,
            scalar_v333: 0.0,
            scalar_v334: 0.0,
            scalar_v335: 0.0,
            scalar_v336: 0.0,
            scalar_v337: 0.0,
            scalar_v341: false,
            scalar_v342: false,
            scalar_v343: false,
            scalar_v344: 0.0,
            scalar_v346: false,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v350: 0.0,
            scalar_v358: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v365: 0.0,
            scalar_v370: 0.0,
            scalar_v371: 0.0,
            scalar_v376: 0.0,
            scalar_v377: 0.0,
            scalar_v384: 0.0,
            scalar_v385: 0.0,
            scalar_v387: 0.0,
            scalar_v399: 0.0,
            scalar_v400: 0.0,
            scalar_v402: 0.0,
            scalar_v414: 0.0,
            scalar_v417: false,
            scalar_v418: false,
            scalar_v419: 0.0,
            scalar_v420: false,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v423: 0.0,
            scalar_v424: 0.0,
            scalar_v425: 0.0,
            scalar_v426: 0.0,
            scalar_v427: 0.0,
            scalar_v428: 0.0,
            scalar_v429: 0.0,
            scalar_v430: 0.0,
            scalar_v431: 0.0,
            scalar_v432: 0.0,
            scalar_v433: 0.0,
            scalar_v434: 0.0,
            scalar_v435: 0.0,
            scalar_v439: 0.0,
            scalar_v440: 0.0,
            scalar_v444: 0.0,
            scalar_v448: false,
            scalar_v449: 0.0,
            scalar_v452: false,
            scalar_v453: 0.0,
            scalar_v468: 0.0,
            scalar_v469: 0.0,
            scalar_v470: 0.0,
            scalar_v471: 0.0,
            scalar_v472: 0.0,
            scalar_v473: 0.0,
            scalar_v476: 0.0,
            scalar_v477: 0.0,
            scalar_v487: 0.0,
            scalar_v488: 0.0,
            scalar_v489: 0.0,
            scalar_v490: 0.0,
            scalar_v508: 0.0,
            scalar_v509: 0.0,
            scalar_v510: 0.0,
            scalar_v524: 0.0,
            scalar_v525: 0.0,
            scalar_v539: 0.0,
            scalar_v540: 0.0,
            scalar_v541: 0.0,
            scalar_v551: 0.0,
            scalar_v552: 0.0,
            scalar_v558: 0.0,
            scalar_v559: 0.0,
            scalar_v560: 0.0,
            scalar_v570: 0.0,
            scalar_v571: 0.0,
            scalar_v576: 0.0,
            scalar_v577: 0.0,
            scalar_v578: 0.0,
            scalar_v579: 0.0,
            scalar_v580: 0.0,
            scalar_v581: 0.0,
            scalar_v582: 0.0,
            scalar_v676: 0.0,
            scalar_v677: 0.0,
            scalar_v678: 0.0,
            scalar_v695: 0.0,
            scalar_v696: 0.0,
            scalar_v697: 0.0,
            scalar_v698: 0.0,
            scalar_v699: 0.0,
            scalar_v700: 0.0,
            scalar_v701: 0.0,
            scalar_v702: 0.0,
            scalar_v703: 0.0,
            scalar_v704: 0.0,
            scalar_v709: 0.0,
            scalar_v795: 0.0,
            scalar_v796: 0.0,
            scalar_v797: 0.0,
            scalar_v798: 0.0,
            scalar_v799: 0.0,
            scalar_v800: 0.0,
            scalar_v801: 0.0,
            scalar_v802: 0.0,
            scalar_v803: 0.0,
            scalar_v804: 0.0,
            scalar_v805: 0.0,
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
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v18,
            scalar_v42,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v50,
            scalar_v52,
            scalar_v54,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v75,
            scalar_v76,
            scalar_v82,
            scalar_v83,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v101,
            scalar_v102,
            scalar_v106,
            scalar_v113,
            scalar_v114,
            scalar_v118,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v130,
            scalar_v134,
            scalar_v137,
            scalar_v140,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v148,
            scalar_v153,
            scalar_v155,
            scalar_v158,
            scalar_v161,
            scalar_v164,
            scalar_v168,
            scalar_v169,
            scalar_v171,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v185,
            scalar_v186,
            scalar_v190,
            scalar_v191,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
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
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v325,
            scalar_v327,
            scalar_v328,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v350,
            scalar_v358,
            scalar_v361,
            scalar_v362,
            scalar_v365,
            scalar_v370,
            scalar_v371,
            scalar_v376,
            scalar_v377,
            scalar_v384,
            scalar_v385,
            scalar_v387,
            scalar_v399,
            scalar_v400,
            scalar_v402,
            scalar_v414,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v452,
            scalar_v453,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v476,
            scalar_v477,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v524,
            scalar_v525,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v551,
            scalar_v552,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v570,
            scalar_v571,
            scalar_v576,
            scalar_v577,
            scalar_v578,
            scalar_v579,
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v676,
            scalar_v677,
            scalar_v678,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v701,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v709,
            scalar_v795,
            scalar_v796,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v805,
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
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v12,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v18,
            scalar_v42,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v50,
            scalar_v52,
            scalar_v54,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v75,
            scalar_v76,
            scalar_v82,
            scalar_v83,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v101,
            scalar_v102,
            scalar_v106,
            scalar_v113,
            scalar_v114,
            scalar_v118,
            scalar_v125,
            scalar_v126,
            scalar_v127,
            scalar_v128,
            scalar_v130,
            scalar_v134,
            scalar_v137,
            scalar_v140,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v148,
            scalar_v153,
            scalar_v155,
            scalar_v158,
            scalar_v161,
            scalar_v164,
            scalar_v168,
            scalar_v169,
            scalar_v171,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v185,
            scalar_v186,
            scalar_v190,
            scalar_v191,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
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
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v325,
            scalar_v327,
            scalar_v328,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v350,
            scalar_v358,
            scalar_v361,
            scalar_v362,
            scalar_v365,
            scalar_v370,
            scalar_v371,
            scalar_v376,
            scalar_v377,
            scalar_v384,
            scalar_v385,
            scalar_v387,
            scalar_v399,
            scalar_v400,
            scalar_v402,
            scalar_v414,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v423,
            scalar_v424,
            scalar_v425,
            scalar_v426,
            scalar_v427,
            scalar_v428,
            scalar_v429,
            scalar_v430,
            scalar_v431,
            scalar_v432,
            scalar_v433,
            scalar_v434,
            scalar_v435,
            scalar_v439,
            scalar_v440,
            scalar_v444,
            scalar_v448,
            scalar_v449,
            scalar_v452,
            scalar_v453,
            scalar_v468,
            scalar_v469,
            scalar_v470,
            scalar_v471,
            scalar_v472,
            scalar_v473,
            scalar_v476,
            scalar_v477,
            scalar_v487,
            scalar_v488,
            scalar_v489,
            scalar_v490,
            scalar_v508,
            scalar_v509,
            scalar_v510,
            scalar_v524,
            scalar_v525,
            scalar_v539,
            scalar_v540,
            scalar_v541,
            scalar_v551,
            scalar_v552,
            scalar_v558,
            scalar_v559,
            scalar_v560,
            scalar_v570,
            scalar_v571,
            scalar_v576,
            scalar_v577,
            scalar_v578,
            scalar_v579,
            scalar_v580,
            scalar_v581,
            scalar_v582,
            scalar_v676,
            scalar_v677,
            scalar_v678,
            scalar_v695,
            scalar_v696,
            scalar_v697,
            scalar_v698,
            scalar_v699,
            scalar_v700,
            scalar_v701,
            scalar_v702,
            scalar_v703,
            scalar_v704,
            scalar_v709,
            scalar_v795,
            scalar_v796,
            scalar_v797,
            scalar_v798,
            scalar_v799,
            scalar_v800,
            scalar_v801,
            scalar_v802,
            scalar_v803,
            scalar_v804,
            scalar_v805,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "tbar" => { validate_parameter("tbar", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("w", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "mult_i" => { validate_parameter("mult_i", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "mult_q" => { validate_parameter("mult_q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "mult_fn" => { validate_parameter("mult_fn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "epsilon" => { validate_parameter("epsilon", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "voff" => { validate_parameter("voff", value, Some((-100.0, "-100.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "asub" => { validate_parameter("asub", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "ksub" => { validate_finite_parameter("ksub", value)?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_parameter("u0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_parameter("ua", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "ub" => { validate_parameter("ub", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_parameter("uc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "vsat" => { validate_parameter("vsat", value, Some((1000.0, "1000.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "delta" => { validate_parameter("delta", value, Some((2.0, "2.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "at" => { validate_finite_parameter("at", value)?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "ute" => { validate_parameter("ute", value, Some((-10.0, "-10.0")), false, Some((0.0, "0.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "lambda" => { validate_parameter("lambda", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_parameter("eta0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "vdscale" => { validate_parameter("vdscale", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "kt1" => { validate_finite_parameter("kt1", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "thesat" => { validate_parameter("thesat", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "nfactor" => { validate_parameter("nfactor", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_parameter("cdscd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "gamma0i" => { validate_parameter("gamma0i", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "gamma1i" => { validate_parameter("gamma1i", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "imin" => { validate_parameter("imin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "rdsmod" => { validate_parameter("rdsmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "vsataccs" => { validate_parameter("vsataccs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "ns0accs" => { validate_parameter("ns0accs", value, Some((100000.0, "100000.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ns0accd" => { validate_parameter("ns0accd", value, Some((100000.0, "100000.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "k0accs" => { validate_parameter("k0accs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "k0accd" => { validate_parameter("k0accd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "u0accs" => { validate_parameter("u0accs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "u0accd" => { validate_parameter("u0accd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "mexpaccs" => { validate_parameter("mexpaccs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "mexpaccd" => { validate_parameter("mexpaccd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "ard" => { validate_parameter("ard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "ars" => { validate_parameter("ars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "lsg" => { validate_parameter("lsg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ldg" => { validate_parameter("ldg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "rsc" => { validate_parameter("rsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "rdc" => { validate_parameter("rdc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "kns0" => { validate_parameter("kns0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "ats" => { validate_finite_parameter("ats", value)?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "utes" => { validate_finite_parameter("utes", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "uted" => { validate_finite_parameter("uted", value)?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "krsc" => { validate_parameter("krsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "krdc" => { validate_parameter("krdc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "gatemod" => { validate_parameter("gatemod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "njgs" => { validate_parameter("njgs", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "ags" => { validate_parameter("ags", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "agd" => { validate_parameter("agd", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "njgd" => { validate_parameter("njgd", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "rnjgs" => { validate_parameter("rnjgs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "rnjgd" => { validate_parameter("rnjgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "igsdio" => { validate_parameter("igsdio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "igddio" => { validate_parameter("igddio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "rigsdio" => { validate_parameter("rigsdio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "rigddio" => { validate_parameter("rigddio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "vbis" => { validate_parameter("vbis", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "vbid" => { validate_parameter("vbid", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "ebreaks" => { validate_parameter("ebreaks", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "ebreakd" => { validate_parameter("ebreakd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "ktgs" => { validate_finite_parameter("ktgs", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "ktgd" => { validate_finite_parameter("ktgd", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "rktgs" => { validate_finite_parameter("rktgs", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "rktgd" => { validate_finite_parameter("rktgd", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "ktvbis" => { validate_finite_parameter("ktvbis", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "ktvbid" => { validate_finite_parameter("ktvbid", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "ktnjgs" => { validate_finite_parameter("ktnjgs", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "ktnjgd" => { validate_finite_parameter("ktnjgd", value)?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "ktrnjgs" => { validate_finite_parameter("ktrnjgs", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "ktrnjgd" => { validate_finite_parameter("ktrnjgd", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "trapmod" => { validate_parameter("trapmod", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "remi" => { validate_parameter("remi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "cglag" => { validate_parameter("cglag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "remig" => { validate_parameter("remig", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "arcap" => { validate_finite_parameter("arcap", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "brcap" => { validate_parameter("brcap", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "arcapg" => { validate_finite_parameter("arcapg", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "brcapg" => { validate_parameter("brcapg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "vdlmax" => { validate_parameter("vdlmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "vglmax" => { validate_parameter("vglmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "dlvoff" => { validate_finite_parameter("dlvoff", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "glvoff" => { validate_finite_parameter("glvoff", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "glu0" => { validate_finite_parameter("glu0", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "glvsat" => { validate_finite_parameter("glvsat", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "dlns0s" => { validate_finite_parameter("dlns0s", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "dlns0d" => { validate_finite_parameter("dlns0d", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "cdlag" => { validate_parameter("cdlag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "rdlag" => { validate_parameter("rdlag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "idio" => { validate_parameter("idio", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "atrapvoff" => { validate_finite_parameter("atrapvoff", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "btrapvoff" => { validate_finite_parameter("btrapvoff", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "atrapeta0" => { validate_finite_parameter("atrapeta0", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "btrapeta0" => { validate_finite_parameter("btrapeta0", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "atraprs" => { validate_finite_parameter("atraprs", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "btraprs" => { validate_finite_parameter("btraprs", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "atraprd" => { validate_finite_parameter("atraprd", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "btraprd" => { validate_finite_parameter("btraprd", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "rtrap1" => { validate_parameter("rtrap1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "rtrap2" => { validate_parameter("rtrap2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "ctrap1" => { validate_parameter("ctrap1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "ctrap2" => { validate_parameter("ctrap2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "a1" => { validate_finite_parameter("a1", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "vofftr" => { validate_finite_parameter("vofftr", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "cdscdtr" => { validate_finite_parameter("cdscdtr", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "eta0tr" => { validate_finite_parameter("eta0tr", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "rontr1" => { validate_finite_parameter("rontr1", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "rontr2" => { validate_finite_parameter("rontr2", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "rontr3" => { validate_finite_parameter("rontr3", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "rtrap3" => { validate_parameter("rtrap3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "ctrap3" => { validate_parameter("ctrap3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "vatrap" => { validate_parameter("vatrap", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "sct" => { validate_parameter("sct", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "wd" => { validate_finite_parameter("wd", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "vdlr1" => { validate_finite_parameter("vdlr1", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "vdlr2" => { validate_finite_parameter("vdlr2", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "talpha" => { validate_finite_parameter("talpha", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "vtb" => { validate_parameter("vtb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "deltax" => { validate_parameter("deltax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "alphax" => { validate_finite_parameter("alphax", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "alphaxd" => { validate_finite_parameter("alphaxd", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "betax" => { validate_finite_parameter("betax", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "gammax" => { validate_finite_parameter("gammax", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "etax" => { validate_finite_parameter("etax", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "eno" => { validate_finite_parameter("eno", value)?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "cx" => { validate_finite_parameter("cx", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "vxmax" => { validate_finite_parameter("vxmax", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "alphay" => { validate_finite_parameter("alphay", value)?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "alphayd" => { validate_finite_parameter("alphayd", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "betay" => { validate_finite_parameter("betay", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "gammay" => { validate_finite_parameter("gammay", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "etay" => { validate_finite_parameter("etay", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "eno1" => { validate_finite_parameter("eno1", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "cy" => { validate_finite_parameter("cy", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "vymax" => { validate_finite_parameter("vymax", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "ea1" => { validate_finite_parameter("ea1", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "glns0s" => { validate_finite_parameter("glns0s", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "glns0d" => { validate_finite_parameter("glns0d", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "fastfpmod" => { validate_parameter("fastfpmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "fp1mod" => { validate_parameter("fp1mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "fp1smod" => { validate_parameter("fp1smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "fp2mod" => { validate_parameter("fp2mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "fp2smod" => { validate_parameter("fp2smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "fp3mod" => { validate_parameter("fp3mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "fp3smod" => { validate_parameter("fp3smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "fp4mod" => { validate_parameter("fp4mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "fp4smod" => { validate_parameter("fp4smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "iminfp1" => { validate_parameter("iminfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "vofffp1" => { validate_parameter("vofffp1", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "dfp1" => { validate_parameter("dfp1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "lfp1" => { validate_parameter("lfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "ktfp1" => { validate_finite_parameter("ktfp1", value)?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "u0fp1" => { validate_parameter("u0fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "vsatfp1" => { validate_parameter("vsatfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "nfactorfp1" => { validate_parameter("nfactorfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "cdscdfp1" => { validate_parameter("cdscdfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "eta0fp1" => { validate_parameter("eta0fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "vdscalefp1" => { validate_parameter("vdscalefp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "gamma0fp1" => { validate_parameter("gamma0fp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "gamma1fp1" => { validate_parameter("gamma1fp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "iminfp2" => { validate_parameter("iminfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "vofffp2" => { validate_parameter("vofffp2", value, Some((-100.0, "-100.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "dfp2" => { validate_parameter("dfp2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "lfp2" => { validate_parameter("lfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "ktfp2" => { validate_finite_parameter("ktfp2", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "u0fp2" => { validate_parameter("u0fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "vsatfp2" => { validate_parameter("vsatfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "nfactorfp2" => { validate_parameter("nfactorfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "cdscdfp2" => { validate_parameter("cdscdfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "eta0fp2" => { validate_parameter("eta0fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "vdscalefp2" => { validate_parameter("vdscalefp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "gamma0fp2" => { validate_parameter("gamma0fp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "gamma1fp2" => { validate_parameter("gamma1fp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "iminfp3" => { validate_parameter("iminfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "vofffp3" => { validate_parameter("vofffp3", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "dfp3" => { validate_parameter("dfp3", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "lfp3" => { validate_parameter("lfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "ktfp3" => { validate_finite_parameter("ktfp3", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "u0fp3" => { validate_parameter("u0fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "vsatfp3" => { validate_parameter("vsatfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "nfactorfp3" => { validate_parameter("nfactorfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "cdscdfp3" => { validate_parameter("cdscdfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "eta0fp3" => { validate_parameter("eta0fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "vdscalefp3" => { validate_parameter("vdscalefp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "gamma0fp3" => { validate_parameter("gamma0fp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "gamma1fp3" => { validate_parameter("gamma1fp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "iminfp4" => { validate_parameter("iminfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "vofffp4" => { validate_parameter("vofffp4", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "dfp4" => { validate_parameter("dfp4", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "lfp4" => { validate_parameter("lfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "ktfp4" => { validate_finite_parameter("ktfp4", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "u0fp4" => { validate_parameter("u0fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "vsatfp4" => { validate_parameter("vsatfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "nfactorfp4" => { validate_parameter("nfactorfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "cdscdfp4" => { validate_parameter("cdscdfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "eta0fp4" => { validate_parameter("eta0fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "vdscalefp4" => { validate_parameter("vdscalefp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "gamma0fp4" => { validate_parameter("gamma0fp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "gamma1fp4" => { validate_parameter("gamma1fp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "cgso" => { validate_parameter("cgso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "cgdo" => { validate_parameter("cgdo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "cdso" => { validate_parameter("cdso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "cgdl" => { validate_parameter("cgdl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "vdsatcv" => { validate_parameter("vdsatcv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "cbdo" => { validate_parameter("cbdo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "cbso" => { validate_parameter("cbso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "cbgo" => { validate_parameter("cbgo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "cfg" => { validate_parameter("cfg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "cfd" => { validate_parameter("cfd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "cfgd" => { validate_parameter("cfgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "cfgdsm" => { validate_parameter("cfgdsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "cfgd0" => { validate_parameter("cfgd0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "cj0" => { validate_parameter("cj0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "vbi" => { validate_parameter("vbi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "ktvbi" => { validate_parameter("ktvbi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "ktcfg" => { validate_parameter("ktcfg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "ktcfgd" => { validate_parameter("ktcfgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "mz" => { validate_parameter("mz", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "aj" => { validate_parameter("aj", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "dj" => { validate_parameter("dj", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), true, &[])?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "adosi" => { validate_parameter("adosi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "bdosi" => { validate_parameter("bdosi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "qm0i" => { validate_parameter("qm0i", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "adosfp1" => { validate_parameter("adosfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "bdosfp1" => { validate_parameter("bdosfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "qm0fp1" => { validate_parameter("qm0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "adosfp2" => { validate_parameter("adosfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "bdosfp2" => { validate_parameter("bdosfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "qm0fp2" => { validate_parameter("qm0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "adosfp3" => { validate_parameter("adosfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "bdosfp3" => { validate_parameter("bdosfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "qm0fp3" => { validate_parameter("qm0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "adosfp4" => { validate_parameter("adosfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "bdosfp4" => { validate_parameter("bdosfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "qm0fp4" => { validate_parameter("qm0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "cfp1scale" => { validate_parameter("cfp1scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "cfp2scale" => { validate_parameter("cfp2scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "cfp3scale" => { validate_parameter("cfp3scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "cfp4scale" => { validate_parameter("cfp4scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "csubscalei" => { validate_parameter("csubscalei", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "csubscale1" => { validate_parameter("csubscale1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "csubscale2" => { validate_parameter("csubscale2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "csubscale3" => { validate_parameter("csubscale3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "csubscale4" => { validate_parameter("csubscale4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "rgatemod" => { validate_parameter("rgatemod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "xgw" => { validate_parameter("xgw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("ngcon", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "rshg" => { validate_parameter("rshg", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "fnmod" => { validate_parameter("fnmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "tnmod" => { validate_parameter("tnmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "noia" => { validate_finite_parameter("noia", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "noib" => { validate_parameter("noib", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "noic" => { validate_parameter("noic", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "ef" => { validate_parameter("ef", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "tnsc" => { validate_parameter("tnsc", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "gdsmin" => { validate_parameter("gdsmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "tgdsmin" => { validate_parameter("tgdsmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "bvdsl" => { validate_parameter("bvdsl", value, Some((10.0, "10.0")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "asl" => { validate_parameter("asl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "nsl" => { validate_parameter("nsl", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "kasl" => { validate_finite_parameter("kasl", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "knsl" => { validate_finite_parameter("knsl", value)?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "kbvdsl" => { validate_finite_parameter("kbvdsl", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "nsb" => { validate_parameter("nsb", value, Some((0.0, "0.0")), true, Some((5000.0, "5000.0")), true, &[])?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "ndb" => { validate_parameter("ndb", value, Some((0.0, "0.0")), true, Some((5000.0, "5000.0")), true, &[])?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "isbl" => { validate_parameter("isbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "idbl" => { validate_parameter("idbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "vbisb" => { validate_parameter("vbisb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "vbidb" => { validate_parameter("vbidb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "ktisb" => { validate_finite_parameter("ktisb", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "ktidb" => { validate_finite_parameter("ktidb", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "ktnsb" => { validate_finite_parameter("ktnsb", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "ktndb" => { validate_finite_parameter("ktndb", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "ktvbisb" => { validate_finite_parameter("ktvbisb", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "ktvbidb" => { validate_finite_parameter("ktvbidb", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'asmhemt'", name)),
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
        let v5: f64 = p.p31;
        self.scalar_v5 = v5;
        let v6: bool = (0.0 == p.p31);
        self.scalar_v6 = v6;
        let v7: f64 = p.p32;
        self.scalar_v7 = v7;
        let v8: bool = (0.0 == p.p32);
        self.scalar_v8 = v8;
        let v9: bool = (v6 || v8);
        self.scalar_v9 = v9;
        let v10: f64 = p.p34;
        self.scalar_v10 = v10;
        let v11: f64 = p.p149;
        self.scalar_v11 = v11;
        let v12: bool = (1.0 == p.p149);
        self.scalar_v12 = v12;
        let v13: bool = (0.0 == p.p34);
        self.scalar_v13 = v13;
        let v14: bool = (v12 && v13);
        self.scalar_v14 = v14;
        let v15: f64 = (if v14 { 1.0 } else { p.p34 });
        self.scalar_v15 = v15;
        let v16: f64 = p.p0;
        self.scalar_v16 = v16;
        let v18: f64 = (p.p0 + 273.15);
        self.scalar_v18 = v18;
        let v42: f64 = p.p274;
        self.scalar_v42 = v42;
        let v46: f64 = p.p81;
        self.scalar_v46 = v46;
        let v47: bool = (0.0 == p.p81);
        self.scalar_v47 = v47;
        let v48: bool = (1.0 == p.p81);
        self.scalar_v48 = v48;
        let v50: bool = (p.p81 == 2.0);
        self.scalar_v50 = v50;
        let v52: bool = (p.p81 == 3.0);
        self.scalar_v52 = v52;
        let v54: bool = (p.p81 == 4.0);
        self.scalar_v54 = v54;
        let v56: bool = (p.p81 == 5.0);
        self.scalar_v56 = v56;
        let v57: bool = (!v47);
        self.scalar_v57 = v57;
        let v58: bool = (v48 && v57);
        self.scalar_v58 = v58;
        let v62: f64 = p.p128;
        self.scalar_v62 = v62;
        let v63: f64 = (0.25 * p.p128);
        self.scalar_v63 = v63;
        let v64: f64 = (p.p128 * v63);
        self.scalar_v64 = v64;
        let v65: bool = (v47 || v48);
        self.scalar_v65 = v65;
        let v66: bool = (!v65);
        self.scalar_v66 = v66;
        let v67: bool = (v50 && v66);
        self.scalar_v67 = v67;
        let v69: bool = (v50 || v65);
        self.scalar_v69 = v69;
        let v70: bool = (!v69);
        self.scalar_v70 = v70;
        let v71: bool = (v52 && v70);
        self.scalar_v71 = v71;
        let v75: f64 = p.p124;
        self.scalar_v75 = v75;
        let v76: f64 = p.p123;
        self.scalar_v76 = v76;
        let v82: f64 = p.p125;
        self.scalar_v82 = v82;
        let v83: f64 = p.p127;
        self.scalar_v83 = v83;
        let v97: bool = (v52 || v69);
        self.scalar_v97 = v97;
        let v98: bool = (!v97);
        self.scalar_v98 = v98;
        let v99: bool = (v54 && v98);
        self.scalar_v99 = v99;
        let v101: f64 = p.p82;
        self.scalar_v101 = v101;
        let v102: f64 = p.p85;
        self.scalar_v102 = v102;
        let v106: f64 = p.p86;
        self.scalar_v106 = v106;
        let v113: f64 = p.p84;
        self.scalar_v113 = v113;
        let v114: f64 = p.p87;
        self.scalar_v114 = v114;
        let v118: f64 = p.p88;
        self.scalar_v118 = v118;
        let v125: bool = (v54 || v97);
        self.scalar_v125 = v125;
        let v126: bool = (!v125);
        self.scalar_v126 = v126;
        let v127: bool = (v56 && v126);
        self.scalar_v127 = v127;
        let v128: f64 = p.p129;
        self.scalar_v128 = v128;
        let v130: f64 = p.p130;
        self.scalar_v130 = v130;
        let v134: f64 = p.p131;
        self.scalar_v134 = v134;
        let v137: f64 = p.p132;
        self.scalar_v137 = v137;
        let v140: f64 = p.p133;
        self.scalar_v140 = v140;
        let v144: f64 = p.p134;
        self.scalar_v144 = v144;
        let v145: f64 = p.p137;
        self.scalar_v145 = v145;
        let v147: f64 = (v18 * 8.617087e-5);
        self.scalar_v147 = v147;
        let v148: f64 = (p.p137 / v147);
        self.scalar_v148 = v148;
        let v153: f64 = p.p138;
        self.scalar_v153 = v153;
        let v155: f64 = p.p139;
        self.scalar_v155 = v155;
        let v158: f64 = p.p140;
        self.scalar_v158 = v158;
        let v161: f64 = p.p141;
        self.scalar_v161 = v161;
        let v164: f64 = p.p142;
        self.scalar_v164 = v164;
        let v168: f64 = p.p143;
        self.scalar_v168 = v168;
        let v169: f64 = p.p146;
        self.scalar_v169 = v169;
        let v171: f64 = (p.p146 / v147);
        self.scalar_v171 = v171;
        let v177: f64 = p.p3;
        self.scalar_v177 = v177;
        let v178: f64 = p.p4;
        self.scalar_v178 = v178;
        let v179: f64 = (2.0 * p.p4);
        self.scalar_v179 = v179;
        let v180: f64 = p.p269;
        self.scalar_v180 = v180;
        let v181: f64 = p.p271;
        self.scalar_v181 = v181;
        let v185: f64 = p.p270;
        self.scalar_v185 = v185;
        let v186: f64 = p.p272;
        self.scalar_v186 = v186;
        let v190: f64 = p.p268;
        self.scalar_v190 = v190;
        let v191: f64 = p.p273;
        self.scalar_v191 = v191;
        let v240: f64 = p.p5;
        self.scalar_v240 = v240;
        let v241: bool = (1.0 == v15);
        self.scalar_v241 = v241;
        let v242: f64 = (p.p4 * p.p5);
        self.scalar_v242 = v242;
        let v243: bool = (0.0 == p.p149);
        self.scalar_v243 = v243;
        let v244: f64 = p.p150;
        self.scalar_v244 = v244;
        let v245: bool = (0.0 != p.p150);
        self.scalar_v245 = v245;
        let v246: bool = (!v245);
        self.scalar_v246 = v246;
        let v247: bool = (v243 && v246);
        self.scalar_v247 = v247;
        let v248: bool = (!v243);
        self.scalar_v248 = v248;
        let v249: f64 = p.p151;
        self.scalar_v249 = v249;
        let v250: bool = (0.0 != p.p151);
        self.scalar_v250 = v250;
        let v251: bool = (!v250);
        self.scalar_v251 = v251;
        let v252: bool = (v243 && v251);
        self.scalar_v252 = v252;
        let v253: bool = (v248 && v251);
        self.scalar_v253 = v253;
        let v254: f64 = p.p152;
        self.scalar_v254 = v254;
        let v255: bool = (0.0 != p.p152);
        self.scalar_v255 = v255;
        let v256: bool = (!v255);
        self.scalar_v256 = v256;
        let v257: bool = (v243 && v256);
        self.scalar_v257 = v257;
        let v258: f64 = p.p153;
        self.scalar_v258 = v258;
        let v259: bool = (0.0 != p.p153);
        self.scalar_v259 = v259;
        let v260: bool = (!v259);
        self.scalar_v260 = v260;
        let v261: bool = (v243 && v260);
        self.scalar_v261 = v261;
        let v262: bool = (v248 && v260);
        self.scalar_v262 = v262;
        let v263: f64 = p.p154;
        self.scalar_v263 = v263;
        let v264: bool = (0.0 != p.p154);
        self.scalar_v264 = v264;
        let v265: bool = (!v264);
        self.scalar_v265 = v265;
        let v266: bool = (v243 && v265);
        self.scalar_v266 = v266;
        let v267: f64 = p.p155;
        self.scalar_v267 = v267;
        let v268: bool = (0.0 != p.p155);
        self.scalar_v268 = v268;
        let v269: bool = (!v268);
        self.scalar_v269 = v269;
        let v270: bool = (v243 && v269);
        self.scalar_v270 = v270;
        let v271: bool = (v248 && v269);
        self.scalar_v271 = v271;
        let v272: f64 = p.p156;
        self.scalar_v272 = v272;
        let v273: bool = (0.0 != p.p156);
        self.scalar_v273 = v273;
        let v274: bool = (!v273);
        self.scalar_v274 = v274;
        let v275: bool = (v243 && v274);
        self.scalar_v275 = v275;
        let v276: f64 = p.p157;
        self.scalar_v276 = v276;
        let v277: bool = (0.0 != p.p157);
        self.scalar_v277 = v277;
        let v278: bool = (!v277);
        self.scalar_v278 = v278;
        let v279: bool = (v243 && v278);
        self.scalar_v279 = v279;
        let v280: bool = (v248 && v278);
        self.scalar_v280 = v280;
        let v281: f64 = p.p255;
        self.scalar_v281 = v281;
        let v282: bool = (1.0 == p.p255);
        self.scalar_v282 = v282;
        let v283: f64 = p.p258;
        self.scalar_v283 = v283;
        let v284: f64 = p.p256;
        self.scalar_v284 = v284;
        let v285: f64 = (p.p4 / 3.0);
        self.scalar_v285 = v285;
        let v286: f64 = p.p257;
        self.scalar_v286 = v286;
        let v287: f64 = (v285 / p.p257);
        self.scalar_v287 = v287;
        let v288: f64 = (p.p256 + v287);
        self.scalar_v288 = v288;
        let v289: f64 = (p.p258 * v288);
        self.scalar_v289 = v289;
        let v290: f64 = (p.p5 * p.p257);
        self.scalar_v290 = v290;
        let v291: f64 = (p.p3 * v290);
        self.scalar_v291 = v291;
        let v292: f64 = (v289 / v291);
        self.scalar_v292 = v292;
        let v293: f64 = (if v282 { v292 } else { 1000.0 });
        self.scalar_v293 = v293;
        let v294: bool = (v293 > 0.0);
        self.scalar_v294 = v294;
        let v295: bool = (v282 && v294);
        self.scalar_v295 = v295;
        let v296: f64 = (1.0 / v293);
        self.scalar_v296 = v296;
        let v297: f64 = (if v295 { v296 } else { v293 });
        self.scalar_v297 = v297;
        let v298: bool = (!v294);
        self.scalar_v298 = v298;
        let v299: bool = (v282 && v298);
        self.scalar_v299 = v299;
        let v300: f64 = (if v299 { 1000.0 } else { v297 });
        self.scalar_v300 = v300;
        let v301: bool = (2.0 == p.p255);
        self.scalar_v301 = v301;
        let v302: bool = (!v282);
        self.scalar_v302 = v302;
        let v303: bool = (v301 && v302);
        self.scalar_v303 = v303;
        let v304: f64 = (if v303 { v292 } else { 1000.0 });
        self.scalar_v304 = v304;
        let v305: f64 = (v179 / 3.0);
        self.scalar_v305 = v305;
        let v306: f64 = (v305 / p.p257);
        self.scalar_v306 = v306;
        let v307: f64 = (p.p258 * v306);
        self.scalar_v307 = v307;
        let v308: f64 = (v307 / v291);
        self.scalar_v308 = v308;
        let v309: f64 = (if v303 { v308 } else { 1000.0 });
        self.scalar_v309 = v309;
        let v310: bool = (v304 > 0.0);
        self.scalar_v310 = v310;
        let v311: bool = (v303 && v310);
        self.scalar_v311 = v311;
        let v312: f64 = (1.0 / v304);
        self.scalar_v312 = v312;
        let v313: f64 = (if v311 { v312 } else { v304 });
        self.scalar_v313 = v313;
        let v314: bool = (!v310);
        self.scalar_v314 = v314;
        let v315: bool = (v303 && v314);
        self.scalar_v315 = v315;
        let v316: f64 = (if v315 { 1000.0 } else { v313 });
        self.scalar_v316 = v316;
        let v317: bool = (v309 > 0.0);
        self.scalar_v317 = v317;
        let v318: bool = (v303 && v317);
        self.scalar_v318 = v318;
        let v319: f64 = (1.0 / v309);
        self.scalar_v319 = v319;
        let v320: f64 = (if v318 { v319 } else { v309 });
        self.scalar_v320 = v320;
        let v321: bool = (!v317);
        self.scalar_v321 = v321;
        let v322: bool = (v303 && v321);
        self.scalar_v322 = v322;
        let v323: f64 = (if v322 { 1000.0 } else { v320 });
        self.scalar_v323 = v323;
        let v325: bool = (!v301);
        self.scalar_v325 = v325;
        let v327: f64 = p.p224;
        self.scalar_v327 = v327;
        let v328: f64 = p.p225;
        self.scalar_v328 = v328;
        let v331: f64 = p.p229;
        self.scalar_v331 = v331;
        let v332: f64 = ((p.p229) as f64).ln();
        self.scalar_v332 = v332;
        let v333: f64 = (-v332);
        self.scalar_v333 = v333;
        let v334: f64 = p.p228;
        self.scalar_v334 = v334;
        let v335: f64 = (v333 / p.p228);
        self.scalar_v335 = v335;
        let v336: f64 = { let limited_exp_arg = v335; if limited_exp_arg > 80.0 { 5.54062238439351e34 * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        self.scalar_v336 = v336;
        let v337: f64 = (1.0 - v336);
        self.scalar_v337 = v337;
        let v341: bool = (1.0 == p.p31);
        self.scalar_v341 = v341;
        let v342: bool = (p.p32 > 0.0);
        self.scalar_v342 = v342;
        let v343: bool = (v341 && v342);
        self.scalar_v343 = v343;
        let v344: f64 = p.p6;
        self.scalar_v344 = v344;
        let v346: bool = (false && v9);
        self.scalar_v346 = v346;
        let v347: f64 = (if v346 { 0.0 } else { 0.0 });
        self.scalar_v347 = v347;
        let v348: f64 = (if v47 { 0.0 } else { 0.0 });
        self.scalar_v348 = v348;
        let v350: f64 = p.p99;
        self.scalar_v350 = v350;
        let v358: f64 = p.p98;
        self.scalar_v358 = v358;
        let v361: f64 = (if v58 { 0.0 } else { 0.0 });
        self.scalar_v361 = v361;
        let v362: f64 = p.p108;
        self.scalar_v362 = v362;
        let v365: f64 = p.p109;
        self.scalar_v365 = v365;
        let v370: f64 = (if v67 { 0.0 } else { 0.0 });
        self.scalar_v370 = v370;
        let v371: f64 = p.p119;
        self.scalar_v371 = v371;
        let v376: f64 = (if v71 { 0.0 } else { 0.0 });
        self.scalar_v376 = v376;
        let v377: f64 = (if v99 { 0.0 } else { 0.0 });
        self.scalar_v377 = v377;
        let v384: f64 = p.p135;
        self.scalar_v384 = v384;
        let v385: f64 = (-p.p135);
        self.scalar_v385 = v385;
        let v387: f64 = p.p136;
        self.scalar_v387 = v387;
        let v399: f64 = p.p144;
        self.scalar_v399 = v399;
        let v400: f64 = (-p.p144);
        self.scalar_v400 = v400;
        let v402: f64 = p.p145;
        self.scalar_v402 = v402;
        let v414: f64 = (if v127 { 0.0 } else { 0.0 });
        self.scalar_v414 = v414;
        let v417: bool = (!v241);
        self.scalar_v417 = v417;
        let v418: bool = (v243 && v417);
        self.scalar_v418 = v418;
        let v419: f64 = (if v418 { 0.0 } else { 0.0 });
        self.scalar_v419 = v419;
        let v420: bool = (v248 && v417);
        self.scalar_v420 = v420;
        let v421: f64 = (if v420 { 0.0 } else { 0.0 });
        self.scalar_v421 = v421;
        let v422: f64 = (if v247 { 0.0 } else { 0.0 });
        self.scalar_v422 = v422;
        let v423: f64 = (if v248 { 0.0 } else { 0.0 });
        self.scalar_v423 = v423;
        let v424: f64 = (if v252 { 0.0 } else { 0.0 });
        self.scalar_v424 = v424;
        let v425: f64 = (if v253 { 0.0 } else { 0.0 });
        self.scalar_v425 = v425;
        let v426: f64 = (if v257 { 0.0 } else { 0.0 });
        self.scalar_v426 = v426;
        let v427: f64 = (if v261 { 0.0 } else { 0.0 });
        self.scalar_v427 = v427;
        let v428: f64 = (if v262 { 0.0 } else { 0.0 });
        self.scalar_v428 = v428;
        let v429: f64 = (if v266 { 0.0 } else { 0.0 });
        self.scalar_v429 = v429;
        let v430: f64 = (if v270 { 0.0 } else { 0.0 });
        self.scalar_v430 = v430;
        let v431: f64 = (if v271 { 0.0 } else { 0.0 });
        self.scalar_v431 = v431;
        let v432: f64 = (if v275 { 0.0 } else { 0.0 });
        self.scalar_v432 = v432;
        let v433: f64 = (if v279 { 0.0 } else { 0.0 });
        self.scalar_v433 = v433;
        let v434: f64 = (if v280 { 0.0 } else { 0.0 });
        self.scalar_v434 = v434;
        let v435: f64 = (v300 * p.p6);
        self.scalar_v435 = v435;
        let v439: f64 = (if v282 { 0.0 } else { 0.0 });
        self.scalar_v439 = v439;
        let v440: f64 = (v316 * p.p6);
        self.scalar_v440 = v440;
        let v444: f64 = (v323 * p.p6);
        self.scalar_v444 = v444;
        let v448: bool = (v302 && v325);
        self.scalar_v448 = v448;
        let v449: f64 = (if v448 { 0.0 } else { 0.0 });
        self.scalar_v449 = v449;
        let v452: bool = (!v343);
        self.scalar_v452 = v452;
        let v453: f64 = (if v452 { 0.0 } else { 0.0 });
        self.scalar_v453 = v453;
        let v468: f64 = (if v71 { 1.0 } else { 0.0 });
        self.scalar_v468 = v468;
        let v469: f64 = (if v71 { -1.0 } else { 0.0 });
        self.scalar_v469 = v469;
        let v470: f64 = (p.p123 * v468);
        self.scalar_v470 = v470;
        let v471: f64 = (p.p123 * v469);
        self.scalar_v471 = v471;
        let v472: f64 = (p.p124 * v470);
        self.scalar_v472 = v472;
        let v473: f64 = (-v472);
        self.scalar_v473 = v473;
        let v476: f64 = (p.p124 * v471);
        self.scalar_v476 = v476;
        let v477: f64 = (-v476);
        self.scalar_v477 = v477;
        let v487: f64 = (p.p125 * v468);
        self.scalar_v487 = v487;
        let v488: f64 = (p.p125 * v469);
        self.scalar_v488 = v488;
        let v489: f64 = (if v71 { v487 } else { 0.0 });
        self.scalar_v489 = v489;
        let v490: f64 = (if v71 { v488 } else { 0.0 });
        self.scalar_v490 = v490;
        let v508: f64 = (1.0 / v18);
        self.scalar_v508 = v508;
        let v509: f64 = (1.0 / p.p86);
        self.scalar_v509 = v509;
        let v510: f64 = (-1.0 / p.p86);
        self.scalar_v510 = v510;
        let v524: f64 = (1.0 / p.p88);
        self.scalar_v524 = v524;
        let v525: f64 = (-1.0 / p.p88);
        self.scalar_v525 = v525;
        let v539: f64 = (-p.p129);
        self.scalar_v539 = v539;
        let v540: f64 = (-p.p130);
        self.scalar_v540 = v540;
        let v541: f64 = (p.p129 + p.p130);
        self.scalar_v541 = v541;
        let v551: f64 = (8.617087e-5 * p.p137);
        self.scalar_v551 = v551;
        let v552: f64 = (-v551);
        self.scalar_v552 = v552;
        let v558: f64 = (-p.p138);
        self.scalar_v558 = v558;
        let v559: f64 = (-p.p139);
        self.scalar_v559 = v559;
        let v560: f64 = (p.p138 + p.p139);
        self.scalar_v560 = v560;
        let v570: f64 = (8.617087e-5 * p.p146);
        self.scalar_v570 = v570;
        let v571: f64 = (-v570);
        self.scalar_v571 = v571;
        let v576: f64 = (p.p271 * v508);
        self.scalar_v576 = v576;
        let v577: f64 = (p.p269 * v576);
        self.scalar_v577 = v577;
        let v578: f64 = (p.p272 * v508);
        self.scalar_v578 = v578;
        let v579: f64 = (p.p270 * v578);
        self.scalar_v579 = v579;
        let v580: f64 = (p.p273 * v508);
        self.scalar_v580 = v580;
        let v581: f64 = (p.p268 * v580);
        self.scalar_v581 = v581;
        let v582: f64 = (-v581);
        self.scalar_v582 = v582;
        let v676: f64 = (p.p225 * v508);
        self.scalar_v676 = v676;
        let v677: f64 = (-v676);
        self.scalar_v677 = v677;
        let v678: f64 = (v337 * v677);
        self.scalar_v678 = v678;
        let v695: f64 = (1.0 / p.p98);
        self.scalar_v695 = v695;
        let v696: f64 = (if v58 { v695 } else { 0.0 });
        self.scalar_v696 = v696;
        let v697: f64 = (1.0 / p.p108);
        self.scalar_v697 = v697;
        let v698: f64 = (if v67 { v697 } else { 0.0 });
        self.scalar_v698 = v698;
        let v699: f64 = (1.0 / p.p109);
        self.scalar_v699 = v699;
        let v700: f64 = (if v67 { v699 } else { 0.0 });
        self.scalar_v700 = v700;
        let v701: f64 = (if v67 { -1.0 } else { 0.0 });
        self.scalar_v701 = v701;
        let v702: f64 = (if v67 { 1.0 } else { 0.0 });
        self.scalar_v702 = v702;
        let v703: f64 = (1.0 / p.p119);
        self.scalar_v703 = v703;
        let v704: f64 = (if v71 { v703 } else { 0.0 });
        self.scalar_v704 = v704;
        let v709: f64 = (if v99 { v678 } else { 0.0 });
        self.scalar_v709 = v709;
        let v795: f64 = (-v435);
        self.scalar_v795 = v795;
        let v796: f64 = (if v282 { v435 } else { 0.0 });
        self.scalar_v796 = v796;
        let v797: f64 = (if v282 { v795 } else { 0.0 });
        self.scalar_v797 = v797;
        let v798: f64 = (-v440);
        self.scalar_v798 = v798;
        let v799: f64 = (if v303 { v440 } else { 0.0 });
        self.scalar_v799 = v799;
        let v800: f64 = (if v303 { v798 } else { 0.0 });
        self.scalar_v800 = v800;
        let v801: f64 = (-v444);
        self.scalar_v801 = v801;
        let v802: f64 = (if v303 { v801 } else { 0.0 });
        self.scalar_v802 = v802;
        let v803: f64 = (if v303 { v444 } else { 0.0 });
        self.scalar_v803 = v803;
        let v804: f64 = (1.0 / p.p32);
        self.scalar_v804 = v804;
        let v805: f64 = (if v343 { v804 } else { 0.0 });
        self.scalar_v805 = v805;
    }
}
