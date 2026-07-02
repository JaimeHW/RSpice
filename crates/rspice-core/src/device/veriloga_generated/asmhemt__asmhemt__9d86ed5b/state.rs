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
    pub(crate) scalar_static_f64: Box<[f64; 900]>,
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
            scalar_static_f64: boxed_zero_f64_array::<900>(),
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
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=p.p31;
        self.scalar_static_f64[1]=p.p32;
        self.scalar_static_f64[2]=p.p34;
        self.scalar_static_f64[3]=p.p149;
        self.scalar_static_bool[0]=(1.0==self.scalar_static_f64[3]);
        self.scalar_static_bool[1]=(0.0==self.scalar_static_f64[2]);
        self.scalar_static_bool[2]=(self.scalar_static_bool[0]&&self.scalar_static_bool[1]);
        self.scalar_static_f64[4]=(if self.scalar_static_bool[2]{1.0}else{self.scalar_static_f64[2]});
        self.scalar_static_f64[5]=p.p0;
        self.scalar_static_f64[6]=(self.scalar_static_f64[5]+273.15);
        self.scalar_static_f64[7]=p.p274;
        self.scalar_static_f64[8]=p.p81;
        self.scalar_static_bool[3]=(0.0==self.scalar_static_f64[8]);
        self.scalar_static_bool[4]=(1.0==self.scalar_static_f64[8]);
        self.scalar_static_bool[5]=(self.scalar_static_f64[8]==2.0);
        self.scalar_static_bool[6]=(self.scalar_static_f64[8]==3.0);
        self.scalar_static_bool[7]=(self.scalar_static_f64[8]==4.0);
        self.scalar_static_bool[8]=(self.scalar_static_f64[8]==5.0);
        self.scalar_static_bool[9]=(!self.scalar_static_bool[3]);
        self.scalar_static_bool[10]=(self.scalar_static_bool[4]&&self.scalar_static_bool[9]);
        self.scalar_static_f64[9]=p.p128;
        self.scalar_static_f64[10]=(0.25*self.scalar_static_f64[9]);
        self.scalar_static_f64[11]=(self.scalar_static_f64[9]*self.scalar_static_f64[10]);
        self.scalar_static_f64[12]=p.p100;
        self.scalar_static_f64[13]=p.p101;
        self.scalar_static_f64[14]=p.p104;
        self.scalar_static_f64[15]=p.p105;
        self.scalar_static_f64[16]=p.p106;
        self.scalar_static_f64[17]=p.p107;
        self.scalar_static_f64[18]=p.p102;
        self.scalar_static_f64[19]=p.p103;
        self.scalar_static_bool[11]=(self.scalar_static_bool[3]||self.scalar_static_bool[4]);
        self.scalar_static_bool[12]=(!self.scalar_static_bool[11]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[5]&&self.scalar_static_bool[12]);
        self.scalar_static_f64[20]=p.p112;
        self.scalar_static_f64[21]=p.p113;
        self.scalar_static_f64[22]=p.p116;
        self.scalar_static_f64[23]=(-self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=p.p117;
        self.scalar_static_f64[25]=p.p118;
        self.scalar_static_f64[26]=p.p114;
        self.scalar_static_f64[27]=p.p115;
        self.scalar_static_bool[14]=(self.scalar_static_bool[5]||self.scalar_static_bool[11]);
        self.scalar_static_bool[15]=(!self.scalar_static_bool[14]);
        self.scalar_static_bool[16]=(self.scalar_static_bool[6]&&self.scalar_static_bool[15]);
        self.scalar_static_f64[28]=p.p124;
        self.scalar_static_f64[29]=p.p123;
        self.scalar_static_f64[30]=p.p125;
        self.scalar_static_f64[31]=p.p127;
        self.scalar_static_f64[32]=p.p10;
        self.scalar_static_f64[33]=p.p122;
        self.scalar_static_f64[34]=p.p120;
        self.scalar_static_f64[35]=(self.scalar_static_f64[34]-1e-9);
        self.scalar_static_f64[36]=(0.5*self.scalar_static_f64[35]);
        self.scalar_static_f64[37]=p.p121;
        self.scalar_static_f64[38]=p.p126;
        self.scalar_static_bool[17]=(self.scalar_static_bool[6]||self.scalar_static_bool[14]);
        self.scalar_static_bool[18]=(!self.scalar_static_bool[17]);
        self.scalar_static_bool[19]=(self.scalar_static_bool[7]&&self.scalar_static_bool[18]);
        self.scalar_static_f64[39]=p.p82;
        self.scalar_static_f64[40]=p.p85;
        self.scalar_static_f64[41]=p.p86;
        self.scalar_static_f64[42]=p.p84;
        self.scalar_static_f64[43]=p.p87;
        self.scalar_static_f64[44]=p.p88;
        self.scalar_static_f64[45]=p.p89;
        self.scalar_static_f64[46]=(self.scalar_static_f64[45]*self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=p.p91;
        self.scalar_static_f64[48]=(self.scalar_static_f64[32]*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=(self.scalar_static_f64[48]).abs();
        self.scalar_static_f64[50]=p.p90;
        self.scalar_static_f64[51]=(self.scalar_static_f64[50]*self.scalar_static_f64[50]);
        self.scalar_static_f64[52]=p.p92;
        self.scalar_static_f64[53]=(self.scalar_static_f64[32]*self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(self.scalar_static_f64[53]).abs();
        self.scalar_static_f64[55]=p.p93;
        self.scalar_static_f64[56]=p.p13;
        self.scalar_static_f64[57]=(self.scalar_static_f64[55]*self.scalar_static_f64[56]);
        self.scalar_static_f64[58]=(self.scalar_static_f64[57]).abs();
        self.scalar_static_f64[59]=p.p94;
        self.scalar_static_f64[60]=p.p17;
        self.scalar_static_f64[61]=(self.scalar_static_f64[59]*self.scalar_static_f64[60]);
        self.scalar_static_f64[62]=(self.scalar_static_f64[61]).abs();
        self.scalar_static_f64[63]=p.p95;
        self.scalar_static_f64[64]=p.p36;
        self.scalar_static_f64[65]=(self.scalar_static_f64[63]*self.scalar_static_f64[64]);
        self.scalar_static_f64[66]=(self.scalar_static_f64[65]).abs();
        self.scalar_static_f64[67]=p.p96;
        self.scalar_static_f64[68]=p.p37;
        self.scalar_static_f64[69]=(self.scalar_static_f64[67]*self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(self.scalar_static_f64[69]).abs();
        self.scalar_static_bool[20]=(self.scalar_static_bool[7]||self.scalar_static_bool[17]);
        self.scalar_static_bool[21]=(!self.scalar_static_bool[20]);
        self.scalar_static_bool[22]=(self.scalar_static_bool[8]&&self.scalar_static_bool[21]);
        self.scalar_static_f64[71]=p.p129;
        self.scalar_static_f64[72]=p.p130;
        self.scalar_static_f64[73]=p.p131;
        self.scalar_static_f64[74]=p.p132;
        self.scalar_static_f64[75]=p.p133;
        self.scalar_static_f64[76]=p.p134;
        self.scalar_static_f64[77]=p.p137;
        self.scalar_static_f64[78]=(self.scalar_static_f64[6]*8.617087e-5);
        self.scalar_static_f64[79]=(self.scalar_static_f64[77]/self.scalar_static_f64[78]);
        self.scalar_static_f64[80]=p.p138;
        self.scalar_static_f64[81]=p.p139;
        self.scalar_static_f64[82]=p.p140;
        self.scalar_static_f64[83]=p.p141;
        self.scalar_static_f64[84]=p.p142;
        self.scalar_static_f64[85]=p.p143;
        self.scalar_static_f64[86]=p.p146;
        self.scalar_static_f64[87]=(self.scalar_static_f64[86]/self.scalar_static_f64[78]);
        self.scalar_static_f64[88]=p.p147;
        self.scalar_static_f64[89]=(self.scalar_static_f64[64]*self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=(self.scalar_static_f64[89]).abs();
        self.scalar_static_f64[91]=p.p148;
        self.scalar_static_f64[92]=(self.scalar_static_f64[68]*self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(self.scalar_static_f64[92]).abs();
        self.scalar_static_f64[94]=p.p9;
        self.scalar_static_f64[95]=p.p1;
        self.scalar_static_f64[96]=(self.scalar_static_f64[94]/self.scalar_static_f64[95]);
        self.scalar_static_f64[97]=p.p2;
        self.scalar_static_f64[98]=(self.scalar_static_f64[94]/self.scalar_static_f64[97]);
        self.scalar_static_f64[99]=p.p26;
        self.scalar_static_f64[100]=(1.0+self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=p.p27;
        self.scalar_static_f64[102]=p.p22;
        self.scalar_static_f64[103]=p.p23;
        self.scalar_static_f64[104]=(self.scalar_static_f64[103]*self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=p.p266;
        self.scalar_static_f64[106]=p.p267;
        self.scalar_static_f64[107]=p.p24;
        self.scalar_static_f64[108]=(self.scalar_static_f64[96]+self.scalar_static_f64[98]);
        self.scalar_static_f64[109]=(self.scalar_static_f64[98]/self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=p.p11;
        self.scalar_static_f64[111]=(self.scalar_static_f64[109]*self.scalar_static_f64[110]);
        self.scalar_static_f64[112]=p.p3;
        self.scalar_static_f64[113]=p.p4;
        self.scalar_static_f64[114]=(2.0*self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=(self.scalar_static_f64[114]*1.602176634e-19);
        self.scalar_static_f64[116]=(self.scalar_static_f64[115]*3.24e17);
        self.scalar_static_f64[117]=p.p30;
        self.scalar_static_f64[118]=(self.scalar_static_f64[96]/1.602176634e-19);
        self.scalar_static_f64[119]=p.p28;
        self.scalar_static_f64[120]=(self.scalar_static_f64[119]/3.0);
        self.scalar_static_f64[121]=(2.0*self.scalar_static_f64[119]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[121]/3.0);
        self.scalar_static_f64[123]=(self.scalar_static_f64[118]/3.24e17);
        self.scalar_static_f64[124]=f64::powf(self.scalar_static_f64[118],0.6666666666666666);
        self.scalar_static_f64[125]=p.p29;
        self.scalar_static_f64[126]=(-self.scalar_static_f64[118]);
        self.scalar_static_f64[127]=p.p20;
        self.scalar_static_f64[128]=p.p19;
        self.scalar_static_f64[129]=(self.scalar_static_f64[96]/self.scalar_static_f64[94]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[98]/self.scalar_static_f64[94]);
        self.scalar_static_f64[131]=p.p14;
        self.scalar_static_f64[132]=p.p15;
        self.scalar_static_f64[133]=p.p16;
        self.scalar_static_f64[134]=p.p18;
        self.scalar_static_f64[135]=(-1.0/self.scalar_static_f64[134]);
        self.scalar_static_f64[136]=p.p5;
        self.scalar_static_f64[137]=p.p21;
        self.scalar_static_f64[138]=p.p25;
        self.scalar_static_f64[139]=(self.scalar_static_f64[138]*self.scalar_static_f64[138]);
        self.scalar_static_f64[140]=p.p269;
        self.scalar_static_f64[141]=p.p271;
        self.scalar_static_f64[142]=p.p270;
        self.scalar_static_f64[143]=p.p272;
        self.scalar_static_f64[144]=p.p268;
        self.scalar_static_f64[145]=p.p273;
        self.scalar_static_f64[146]=(self.scalar_static_f64[96]*self.scalar_static_f64[113]);
        self.scalar_static_f64[147]=(self.scalar_static_f64[136]*self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[112]*self.scalar_static_f64[147]);
        self.scalar_static_f64[149]=p.p233;
        self.scalar_static_f64[150]=p.p232;
        self.scalar_static_f64[151]=p.p231;
        self.scalar_static_f64[152]=p.p56;
        self.scalar_static_bool[23]=(0.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[24]=(1.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[25]=(2.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[26]=(3.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[27]=(4.0==self.scalar_static_f64[152]);
        self.scalar_static_bool[28]=(!self.scalar_static_bool[23]);
        self.scalar_static_bool[29]=(self.scalar_static_bool[24]&&self.scalar_static_bool[28]);
        self.scalar_static_f64[153]=p.p57;
        self.scalar_static_f64[154]=(8.617087e-5*self.scalar_static_f64[153]);
        self.scalar_static_f64[155]=p.p63;
        self.scalar_static_f64[156]=p.p71;
        self.scalar_static_f64[157]=(self.scalar_static_f64[112]*self.scalar_static_f64[113]);
        self.scalar_static_f64[158]=(self.scalar_static_f64[136]*self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=p.p60;
        self.scalar_static_f64[160]=(8.617087e-5*self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=p.p64;
        self.scalar_static_f64[162]=p.p72;
        self.scalar_static_bool[30]=(self.scalar_static_bool[23]||self.scalar_static_bool[24]);
        self.scalar_static_bool[31]=(!self.scalar_static_bool[30]);
        self.scalar_static_bool[32]=(self.scalar_static_bool[25]&&self.scalar_static_bool[31]);
        self.scalar_static_f64[163]=p.p67;
        self.scalar_static_f64[164]=p.p75;
        self.scalar_static_f64[165]=p.p77;
        self.scalar_static_f64[166]=p.p61;
        self.scalar_static_f64[167]=p.p79;
        self.scalar_static_f64[168]=p.p69;
        self.scalar_static_f64[169]=p.p65;
        self.scalar_static_f64[170]=p.p73;
        self.scalar_static_f64[171]=p.p68;
        self.scalar_static_f64[172]=p.p76;
        self.scalar_static_f64[173]=p.p78;
        self.scalar_static_f64[174]=p.p62;
        self.scalar_static_f64[175]=p.p80;
        self.scalar_static_f64[176]=p.p70;
        self.scalar_static_f64[177]=p.p66;
        self.scalar_static_f64[178]=p.p74;
        self.scalar_static_bool[33]=(self.scalar_static_bool[25]||self.scalar_static_bool[30]);
        self.scalar_static_bool[34]=(!self.scalar_static_bool[33]);
        self.scalar_static_bool[35]=(self.scalar_static_bool[26]&&self.scalar_static_bool[34]);
        self.scalar_static_f64[179]=(self.scalar_static_f64[155]*self.scalar_static_f64[158]);
        self.scalar_static_f64[180]=p.p58;
        self.scalar_static_f64[181]=(self.scalar_static_f64[158]*self.scalar_static_f64[161]);
        self.scalar_static_f64[182]=p.p59;
        self.scalar_static_bool[36]=(self.scalar_static_bool[26]||self.scalar_static_bool[33]);
        self.scalar_static_bool[37]=(!self.scalar_static_bool[36]);
        self.scalar_static_bool[38]=(self.scalar_static_bool[27]&&self.scalar_static_bool[37]);
        self.scalar_static_f64[183]=(self.scalar_static_f64[158]*self.scalar_static_f64[169]);
        self.scalar_static_f64[184]=(self.scalar_static_f64[158]*self.scalar_static_f64[177]);
        self.scalar_static_f64[185]=if param_given[45] { 1.0 } else { 0.0 };
        self.scalar_static_f64[186]=if param_given[44] { 1.0 } else { 0.0 };
        self.scalar_static_bool[39]=(1.0==self.scalar_static_f64[4]);
        self.scalar_static_f64[187]=p.p50;
        self.scalar_static_f64[188]=p.p12;
        self.scalar_static_f64[189]=(self.scalar_static_f64[188]/1.602176634e-19);
        self.scalar_static_f64[190]=p.p38;
        self.scalar_static_f64[191]=p.p35;
        self.scalar_static_f64[192]=p.p51;
        self.scalar_static_f64[193]=(self.scalar_static_f64[113]*self.scalar_static_f64[136]);
        self.scalar_static_f64[194]=p.p40;
        self.scalar_static_f64[195]=p.p52;
        self.scalar_static_f64[196]=p.p46;
        self.scalar_static_bool[40]=(0.0!=self.scalar_static_f64[185]);
        self.scalar_static_bool[41]=(self.scalar_static_bool[39]&&self.scalar_static_bool[40]);
        self.scalar_static_f64[197]=p.p45;
        self.scalar_static_f64[198]=(1.0+self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=(if self.scalar_static_bool[41]{self.scalar_static_f64[198]}else{0.0});
        self.scalar_static_f64[200]=(self.scalar_static_f64[199]).sqrt();
        self.scalar_static_bool[42]=(!self.scalar_static_bool[40]);
        self.scalar_static_bool[43]=(self.scalar_static_bool[39]&&self.scalar_static_bool[42]);
        self.scalar_static_f64[201]=p.p42;
        self.scalar_static_f64[202]=(1.0/self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=p.p48;
        self.scalar_static_f64[204]=p.p54;
        self.scalar_static_f64[205]=p.p39;
        self.scalar_static_f64[206]=p.p41;
        self.scalar_static_f64[207]=p.p53;
        self.scalar_static_f64[208]=p.p47;
        self.scalar_static_bool[44]=(0.0!=self.scalar_static_f64[186]);
        self.scalar_static_bool[45]=(self.scalar_static_bool[39]&&self.scalar_static_bool[44]);
        self.scalar_static_f64[209]=p.p44;
        self.scalar_static_f64[210]=(1.0+self.scalar_static_f64[209]);
        self.scalar_static_bool[46]=(!self.scalar_static_bool[44]);
        self.scalar_static_bool[47]=(self.scalar_static_bool[39]&&self.scalar_static_bool[46]);
        self.scalar_static_f64[211]=p.p43;
        self.scalar_static_f64[212]=(1.0/self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=p.p49;
        self.scalar_static_f64[214]=p.p55;
        self.scalar_static_bool[48]=(0.0==self.scalar_static_f64[3]);
        self.scalar_static_f64[215]=p.p150;
        self.scalar_static_bool[49]=(0.0!=self.scalar_static_f64[215]);
        self.scalar_static_bool[50]=(self.scalar_static_bool[48]&&self.scalar_static_bool[49]);
        self.scalar_static_bool[51]=(1.0==self.scalar_static_f64[215]);
        self.scalar_static_bool[52]=(self.scalar_static_bool[50]&&self.scalar_static_bool[51]);
        self.scalar_static_bool[53]=(!self.scalar_static_bool[51]);
        self.scalar_static_bool[54]=(self.scalar_static_bool[50]&&self.scalar_static_bool[53]);
        self.scalar_static_f64[216]=p.p165;
        self.scalar_static_f64[217]=(1.0+self.scalar_static_f64[216]);
        self.scalar_static_f64[218]=p.p166;
        self.scalar_static_f64[219]=p.p159;
        self.scalar_static_f64[220]=p.p162;
        self.scalar_static_f64[221]=p.p167;
        self.scalar_static_f64[222]=p.p168;
        self.scalar_static_f64[223]=(self.scalar_static_f64[222]*self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=p.p160;
        self.scalar_static_f64[225]=(self.scalar_static_f64[94]/self.scalar_static_f64[224]);
        self.scalar_static_f64[226]=(if self.scalar_static_bool[50]{self.scalar_static_f64[225]}else{0.0});
        self.scalar_static_f64[227]=p.p161;
        self.scalar_static_f64[228]=p.p158;
        self.scalar_static_f64[229]=(self.scalar_static_f64[226]/1.602176634e-19);
        self.scalar_static_f64[230]=(if self.scalar_static_bool[50]{self.scalar_static_f64[229]}else{self.scalar_static_f64[118]});
        self.scalar_static_f64[231]=p.p169;
        self.scalar_static_f64[232]=(self.scalar_static_f64[231]/3.0);
        self.scalar_static_f64[233]=(2.0*self.scalar_static_f64[231]);
        self.scalar_static_f64[234]=(self.scalar_static_f64[233]/3.0);
        self.scalar_static_f64[235]=(self.scalar_static_f64[230]/3.24e17);
        self.scalar_static_f64[236]=f64::powf(self.scalar_static_f64[230],0.6666666666666666);
        self.scalar_static_f64[237]=p.p170;
        self.scalar_static_f64[238]=(-self.scalar_static_f64[230]);
        self.scalar_static_f64[239]=p.p163;
        self.scalar_static_f64[240]=p.p164;
        self.scalar_static_f64[241]=(self.scalar_static_f64[226]/self.scalar_static_f64[94]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[113]*self.scalar_static_f64[226]);
        self.scalar_static_f64[243]=(self.scalar_static_f64[136]*self.scalar_static_f64[242]);
        self.scalar_static_f64[244]=(self.scalar_static_f64[227]*self.scalar_static_f64[243]);
        self.scalar_static_f64[245]=p.p236;
        self.scalar_static_f64[246]=p.p235;
        self.scalar_static_f64[247]=p.p234;
        self.scalar_static_bool[55]=(!self.scalar_static_bool[49]);
        self.scalar_static_bool[56]=(self.scalar_static_bool[48]&&self.scalar_static_bool[55]);
        self.scalar_static_bool[57]=(!self.scalar_static_bool[48]);
        self.scalar_static_bool[58]=(self.scalar_static_bool[49]&&self.scalar_static_bool[57]);
        self.scalar_static_bool[59]=(self.scalar_static_bool[51]&&self.scalar_static_bool[58]);
        self.scalar_static_bool[60]=(self.scalar_static_bool[53]&&self.scalar_static_bool[58]);
        self.scalar_static_f64[248]=(if self.scalar_static_bool[58]{self.scalar_static_f64[225]}else{self.scalar_static_f64[226]});
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]/1.602176634e-19);
        self.scalar_static_f64[250]=(if self.scalar_static_bool[58]{self.scalar_static_f64[249]}else{self.scalar_static_f64[230]});
        self.scalar_static_f64[251]=(self.scalar_static_f64[250]/3.24e17);
        self.scalar_static_f64[252]=f64::powf(self.scalar_static_f64[250],0.6666666666666666);
        self.scalar_static_f64[253]=(-self.scalar_static_f64[250]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[248]/self.scalar_static_f64[94]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[113]*self.scalar_static_f64[248]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[136]*self.scalar_static_f64[255]);
        self.scalar_static_f64[257]=(self.scalar_static_f64[227]*self.scalar_static_f64[256]);
        self.scalar_static_bool[61]=(self.scalar_static_bool[55]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[258]=p.p151;
        self.scalar_static_bool[62]=(0.0!=self.scalar_static_f64[258]);
        self.scalar_static_bool[63]=(self.scalar_static_bool[48]&&self.scalar_static_bool[62]);
        self.scalar_static_bool[64]=(1.0==self.scalar_static_f64[258]);
        self.scalar_static_bool[65]=(self.scalar_static_bool[63]&&self.scalar_static_bool[64]);
        self.scalar_static_bool[66]=(!self.scalar_static_bool[64]);
        self.scalar_static_bool[67]=(self.scalar_static_bool[63]&&self.scalar_static_bool[66]);
        self.scalar_static_f64[259]=(if self.scalar_static_bool[63]{self.scalar_static_f64[225]}else{0.0});
        self.scalar_static_f64[260]=(self.scalar_static_f64[259]/1.602176634e-19);
        self.scalar_static_f64[261]=(if self.scalar_static_bool[63]{self.scalar_static_f64[260]}else{self.scalar_static_f64[250]});
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]/3.24e17);
        self.scalar_static_f64[263]=f64::powf(self.scalar_static_f64[261],0.6666666666666666);
        self.scalar_static_f64[264]=(-self.scalar_static_f64[261]);
        self.scalar_static_f64[265]=(self.scalar_static_f64[259]/self.scalar_static_f64[94]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[113]*self.scalar_static_f64[259]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[136]*self.scalar_static_f64[266]);
        self.scalar_static_f64[268]=(self.scalar_static_f64[227]*self.scalar_static_f64[267]);
        self.scalar_static_bool[68]=(!self.scalar_static_bool[62]);
        self.scalar_static_bool[69]=(self.scalar_static_bool[48]&&self.scalar_static_bool[68]);
        self.scalar_static_bool[70]=(self.scalar_static_bool[57]&&self.scalar_static_bool[62]);
        self.scalar_static_bool[71]=(self.scalar_static_bool[64]&&self.scalar_static_bool[70]);
        self.scalar_static_bool[72]=(self.scalar_static_bool[66]&&self.scalar_static_bool[70]);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[70]{self.scalar_static_f64[225]}else{self.scalar_static_f64[259]});
        self.scalar_static_f64[270]=(self.scalar_static_f64[269]/1.602176634e-19);
        self.scalar_static_f64[271]=(if self.scalar_static_bool[70]{self.scalar_static_f64[270]}else{self.scalar_static_f64[261]});
        self.scalar_static_f64[272]=(self.scalar_static_f64[271]/3.24e17);
        self.scalar_static_f64[273]=f64::powf(self.scalar_static_f64[271],0.6666666666666666);
        self.scalar_static_f64[274]=(-self.scalar_static_f64[271]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[269]/self.scalar_static_f64[94]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[113]*self.scalar_static_f64[269]);
        self.scalar_static_f64[277]=(self.scalar_static_f64[136]*self.scalar_static_f64[276]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[227]*self.scalar_static_f64[277]);
        self.scalar_static_bool[73]=(self.scalar_static_bool[57]&&self.scalar_static_bool[68]);
        self.scalar_static_f64[279]=p.p152;
        self.scalar_static_bool[74]=(0.0!=self.scalar_static_f64[279]);
        self.scalar_static_bool[75]=(self.scalar_static_bool[48]&&self.scalar_static_bool[74]);
        self.scalar_static_bool[76]=(1.0==self.scalar_static_f64[279]);
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]&&self.scalar_static_bool[76]);
        self.scalar_static_bool[78]=(!self.scalar_static_bool[76]);
        self.scalar_static_bool[79]=(self.scalar_static_bool[75]&&self.scalar_static_bool[78]);
        self.scalar_static_f64[280]=p.p178;
        self.scalar_static_f64[281]=(1.0+self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=p.p179;
        self.scalar_static_f64[283]=p.p172;
        self.scalar_static_f64[284]=p.p175;
        self.scalar_static_f64[285]=p.p180;
        self.scalar_static_f64[286]=p.p181;
        self.scalar_static_f64[287]=(self.scalar_static_f64[286]*self.scalar_static_f64[286]);
        self.scalar_static_f64[288]=p.p173;
        self.scalar_static_f64[289]=(self.scalar_static_f64[94]/self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(if self.scalar_static_bool[75]{self.scalar_static_f64[289]}else{0.0});
        self.scalar_static_f64[291]=p.p174;
        self.scalar_static_f64[292]=p.p171;
        self.scalar_static_f64[293]=(self.scalar_static_f64[290]/1.602176634e-19);
        self.scalar_static_f64[294]=(if self.scalar_static_bool[75]{self.scalar_static_f64[293]}else{self.scalar_static_f64[271]});
        self.scalar_static_f64[295]=p.p182;
        self.scalar_static_f64[296]=(self.scalar_static_f64[295]/3.0);
        self.scalar_static_f64[297]=(2.0*self.scalar_static_f64[295]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[297]/3.0);
        self.scalar_static_f64[299]=(self.scalar_static_f64[294]/3.24e17);
        self.scalar_static_f64[300]=f64::powf(self.scalar_static_f64[294],0.6666666666666666);
        self.scalar_static_f64[301]=p.p183;
        self.scalar_static_f64[302]=(-self.scalar_static_f64[294]);
        self.scalar_static_f64[303]=p.p176;
        self.scalar_static_f64[304]=p.p177;
        self.scalar_static_f64[305]=(self.scalar_static_f64[290]/self.scalar_static_f64[94]);
        self.scalar_static_f64[306]=(self.scalar_static_f64[113]*self.scalar_static_f64[290]);
        self.scalar_static_f64[307]=(self.scalar_static_f64[136]*self.scalar_static_f64[306]);
        self.scalar_static_f64[308]=(self.scalar_static_f64[291]*self.scalar_static_f64[307]);
        self.scalar_static_f64[309]=p.p239;
        self.scalar_static_f64[310]=p.p238;
        self.scalar_static_f64[311]=p.p237;
        self.scalar_static_bool[80]=(!self.scalar_static_bool[74]);
        self.scalar_static_bool[81]=(self.scalar_static_bool[48]&&self.scalar_static_bool[80]);
        self.scalar_static_bool[82]=(self.scalar_static_bool[57]&&self.scalar_static_bool[74]);
        self.scalar_static_bool[83]=(self.scalar_static_bool[76]&&self.scalar_static_bool[82]);
        self.scalar_static_bool[84]=(self.scalar_static_bool[78]&&self.scalar_static_bool[82]);
        self.scalar_static_f64[312]=(if self.scalar_static_bool[82]{self.scalar_static_f64[289]}else{self.scalar_static_f64[290]});
        self.scalar_static_f64[313]=(self.scalar_static_f64[312]/1.602176634e-19);
        self.scalar_static_f64[314]=(if self.scalar_static_bool[82]{self.scalar_static_f64[313]}else{self.scalar_static_f64[294]});
        self.scalar_static_f64[315]=(self.scalar_static_f64[314]/3.24e17);
        self.scalar_static_f64[316]=f64::powf(self.scalar_static_f64[314],0.6666666666666666);
        self.scalar_static_f64[317]=(-self.scalar_static_f64[314]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[312]/self.scalar_static_f64[94]);
        self.scalar_static_f64[319]=(self.scalar_static_f64[113]*self.scalar_static_f64[312]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[136]*self.scalar_static_f64[319]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[291]*self.scalar_static_f64[320]);
        self.scalar_static_bool[85]=(self.scalar_static_bool[57]&&self.scalar_static_bool[80]);
        self.scalar_static_f64[322]=p.p153;
        self.scalar_static_bool[86]=(0.0!=self.scalar_static_f64[322]);
        self.scalar_static_bool[87]=(self.scalar_static_bool[48]&&self.scalar_static_bool[86]);
        self.scalar_static_bool[88]=(1.0==self.scalar_static_f64[322]);
        self.scalar_static_bool[89]=(self.scalar_static_bool[87]&&self.scalar_static_bool[88]);
        self.scalar_static_bool[90]=(!self.scalar_static_bool[88]);
        self.scalar_static_bool[91]=(self.scalar_static_bool[87]&&self.scalar_static_bool[90]);
        self.scalar_static_f64[323]=(if self.scalar_static_bool[87]{self.scalar_static_f64[289]}else{0.0});
        self.scalar_static_f64[324]=(self.scalar_static_f64[323]/1.602176634e-19);
        self.scalar_static_f64[325]=(if self.scalar_static_bool[87]{self.scalar_static_f64[324]}else{self.scalar_static_f64[314]});
        self.scalar_static_f64[326]=(self.scalar_static_f64[325]/3.24e17);
        self.scalar_static_f64[327]=f64::powf(self.scalar_static_f64[325],0.6666666666666666);
        self.scalar_static_f64[328]=(-self.scalar_static_f64[325]);
        self.scalar_static_f64[329]=(self.scalar_static_f64[323]/self.scalar_static_f64[94]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[113]*self.scalar_static_f64[323]);
        self.scalar_static_f64[331]=(self.scalar_static_f64[136]*self.scalar_static_f64[330]);
        self.scalar_static_f64[332]=(self.scalar_static_f64[291]*self.scalar_static_f64[331]);
        self.scalar_static_bool[92]=(!self.scalar_static_bool[86]);
        self.scalar_static_bool[93]=(self.scalar_static_bool[48]&&self.scalar_static_bool[92]);
        self.scalar_static_bool[94]=(self.scalar_static_bool[57]&&self.scalar_static_bool[86]);
        self.scalar_static_bool[95]=(self.scalar_static_bool[88]&&self.scalar_static_bool[94]);
        self.scalar_static_bool[96]=(self.scalar_static_bool[90]&&self.scalar_static_bool[94]);
        self.scalar_static_f64[333]=(if self.scalar_static_bool[94]{self.scalar_static_f64[289]}else{self.scalar_static_f64[323]});
        self.scalar_static_f64[334]=(self.scalar_static_f64[333]/1.602176634e-19);
        self.scalar_static_f64[335]=(if self.scalar_static_bool[94]{self.scalar_static_f64[334]}else{self.scalar_static_f64[325]});
        self.scalar_static_f64[336]=(self.scalar_static_f64[335]/3.24e17);
        self.scalar_static_f64[337]=f64::powf(self.scalar_static_f64[335],0.6666666666666666);
        self.scalar_static_f64[338]=(-self.scalar_static_f64[335]);
        self.scalar_static_f64[339]=(self.scalar_static_f64[333]/self.scalar_static_f64[94]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[113]*self.scalar_static_f64[333]);
        self.scalar_static_f64[341]=(self.scalar_static_f64[136]*self.scalar_static_f64[340]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[291]*self.scalar_static_f64[341]);
        self.scalar_static_bool[97]=(self.scalar_static_bool[57]&&self.scalar_static_bool[92]);
        self.scalar_static_f64[343]=p.p154;
        self.scalar_static_bool[98]=(0.0!=self.scalar_static_f64[343]);
        self.scalar_static_bool[99]=(self.scalar_static_bool[48]&&self.scalar_static_bool[98]);
        self.scalar_static_bool[100]=(1.0==self.scalar_static_f64[343]);
        self.scalar_static_bool[101]=(self.scalar_static_bool[99]&&self.scalar_static_bool[100]);
        self.scalar_static_bool[102]=(!self.scalar_static_bool[100]);
        self.scalar_static_bool[103]=(self.scalar_static_bool[99]&&self.scalar_static_bool[102]);
        self.scalar_static_f64[344]=p.p191;
        self.scalar_static_f64[345]=(1.0+self.scalar_static_f64[344]);
        self.scalar_static_f64[346]=p.p192;
        self.scalar_static_f64[347]=p.p185;
        self.scalar_static_f64[348]=p.p188;
        self.scalar_static_f64[349]=p.p193;
        self.scalar_static_f64[350]=p.p194;
        self.scalar_static_f64[351]=(self.scalar_static_f64[350]*self.scalar_static_f64[350]);
        self.scalar_static_f64[352]=p.p186;
        self.scalar_static_f64[353]=(self.scalar_static_f64[94]/self.scalar_static_f64[352]);
        self.scalar_static_f64[354]=(if self.scalar_static_bool[99]{self.scalar_static_f64[353]}else{0.0});
        self.scalar_static_f64[355]=p.p187;
        self.scalar_static_f64[356]=p.p184;
        self.scalar_static_f64[357]=(self.scalar_static_f64[354]/1.602176634e-19);
        self.scalar_static_f64[358]=(if self.scalar_static_bool[99]{self.scalar_static_f64[357]}else{self.scalar_static_f64[335]});
        self.scalar_static_f64[359]=p.p195;
        self.scalar_static_f64[360]=(self.scalar_static_f64[359]/3.0);
        self.scalar_static_f64[361]=(2.0*self.scalar_static_f64[359]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[361]/3.0);
        self.scalar_static_f64[363]=(self.scalar_static_f64[358]/3.24e17);
        self.scalar_static_f64[364]=f64::powf(self.scalar_static_f64[358],0.6666666666666666);
        self.scalar_static_f64[365]=p.p196;
        self.scalar_static_f64[366]=(-self.scalar_static_f64[358]);
        self.scalar_static_f64[367]=p.p189;
        self.scalar_static_f64[368]=p.p190;
        self.scalar_static_f64[369]=(self.scalar_static_f64[354]/self.scalar_static_f64[94]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[113]*self.scalar_static_f64[354]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[136]*self.scalar_static_f64[370]);
        self.scalar_static_f64[372]=(self.scalar_static_f64[355]*self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=p.p242;
        self.scalar_static_f64[374]=p.p241;
        self.scalar_static_f64[375]=p.p240;
        self.scalar_static_bool[104]=(!self.scalar_static_bool[98]);
        self.scalar_static_bool[105]=(self.scalar_static_bool[48]&&self.scalar_static_bool[104]);
        self.scalar_static_bool[106]=(self.scalar_static_bool[57]&&self.scalar_static_bool[98]);
        self.scalar_static_bool[107]=(self.scalar_static_bool[100]&&self.scalar_static_bool[106]);
        self.scalar_static_bool[108]=(self.scalar_static_bool[102]&&self.scalar_static_bool[106]);
        self.scalar_static_f64[376]=(if self.scalar_static_bool[106]{self.scalar_static_f64[353]}else{self.scalar_static_f64[354]});
        self.scalar_static_f64[377]=(self.scalar_static_f64[376]/1.602176634e-19);
        self.scalar_static_f64[378]=(if self.scalar_static_bool[106]{self.scalar_static_f64[377]}else{self.scalar_static_f64[358]});
        self.scalar_static_f64[379]=(self.scalar_static_f64[378]/3.24e17);
        self.scalar_static_f64[380]=f64::powf(self.scalar_static_f64[378],0.6666666666666666);
        self.scalar_static_f64[381]=(-self.scalar_static_f64[378]);
        self.scalar_static_f64[382]=(self.scalar_static_f64[376]/self.scalar_static_f64[94]);
        self.scalar_static_f64[383]=(self.scalar_static_f64[113]*self.scalar_static_f64[376]);
        self.scalar_static_f64[384]=(self.scalar_static_f64[136]*self.scalar_static_f64[383]);
        self.scalar_static_f64[385]=(self.scalar_static_f64[355]*self.scalar_static_f64[384]);
        self.scalar_static_bool[109]=(self.scalar_static_bool[57]&&self.scalar_static_bool[104]);
        self.scalar_static_f64[386]=p.p155;
        self.scalar_static_bool[110]=(0.0!=self.scalar_static_f64[386]);
        self.scalar_static_bool[111]=(self.scalar_static_bool[48]&&self.scalar_static_bool[110]);
        self.scalar_static_bool[112]=(1.0==self.scalar_static_f64[386]);
        self.scalar_static_bool[113]=(self.scalar_static_bool[111]&&self.scalar_static_bool[112]);
        self.scalar_static_bool[114]=(!self.scalar_static_bool[112]);
        self.scalar_static_bool[115]=(self.scalar_static_bool[111]&&self.scalar_static_bool[114]);
        self.scalar_static_f64[387]=(if self.scalar_static_bool[111]{self.scalar_static_f64[353]}else{0.0});
        self.scalar_static_f64[388]=(self.scalar_static_f64[387]/1.602176634e-19);
        self.scalar_static_f64[389]=(if self.scalar_static_bool[111]{self.scalar_static_f64[388]}else{self.scalar_static_f64[378]});
        self.scalar_static_f64[390]=(self.scalar_static_f64[389]/3.24e17);
        self.scalar_static_f64[391]=f64::powf(self.scalar_static_f64[389],0.6666666666666666);
        self.scalar_static_f64[392]=(-self.scalar_static_f64[389]);
        self.scalar_static_f64[393]=(self.scalar_static_f64[387]/self.scalar_static_f64[94]);
        self.scalar_static_f64[394]=(self.scalar_static_f64[113]*self.scalar_static_f64[387]);
        self.scalar_static_f64[395]=(self.scalar_static_f64[136]*self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=(self.scalar_static_f64[355]*self.scalar_static_f64[395]);
        self.scalar_static_bool[116]=(!self.scalar_static_bool[110]);
        self.scalar_static_bool[117]=(self.scalar_static_bool[48]&&self.scalar_static_bool[116]);
        self.scalar_static_bool[118]=(self.scalar_static_bool[57]&&self.scalar_static_bool[110]);
        self.scalar_static_bool[119]=(self.scalar_static_bool[112]&&self.scalar_static_bool[118]);
        self.scalar_static_bool[120]=(self.scalar_static_bool[114]&&self.scalar_static_bool[118]);
        self.scalar_static_f64[397]=(if self.scalar_static_bool[118]{self.scalar_static_f64[353]}else{self.scalar_static_f64[387]});
        self.scalar_static_f64[398]=(self.scalar_static_f64[397]/1.602176634e-19);
        self.scalar_static_f64[399]=(if self.scalar_static_bool[118]{self.scalar_static_f64[398]}else{self.scalar_static_f64[389]});
        self.scalar_static_f64[400]=(self.scalar_static_f64[399]/3.24e17);
        self.scalar_static_f64[401]=f64::powf(self.scalar_static_f64[399],0.6666666666666666);
        self.scalar_static_f64[402]=(-self.scalar_static_f64[399]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[397]/self.scalar_static_f64[94]);
        self.scalar_static_f64[404]=(self.scalar_static_f64[113]*self.scalar_static_f64[397]);
        self.scalar_static_f64[405]=(self.scalar_static_f64[136]*self.scalar_static_f64[404]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[355]*self.scalar_static_f64[405]);
        self.scalar_static_bool[121]=(self.scalar_static_bool[57]&&self.scalar_static_bool[116]);
        self.scalar_static_f64[407]=p.p156;
        self.scalar_static_bool[122]=(0.0!=self.scalar_static_f64[407]);
        self.scalar_static_bool[123]=(self.scalar_static_bool[48]&&self.scalar_static_bool[122]);
        self.scalar_static_bool[124]=(1.0==self.scalar_static_f64[407]);
        self.scalar_static_bool[125]=(self.scalar_static_bool[123]&&self.scalar_static_bool[124]);
        self.scalar_static_bool[126]=(!self.scalar_static_bool[124]);
        self.scalar_static_bool[127]=(self.scalar_static_bool[123]&&self.scalar_static_bool[126]);
        self.scalar_static_f64[408]=p.p204;
        self.scalar_static_f64[409]=(1.0+self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=p.p205;
        self.scalar_static_f64[411]=p.p198;
        self.scalar_static_f64[412]=p.p201;
        self.scalar_static_f64[413]=p.p206;
        self.scalar_static_f64[414]=p.p207;
        self.scalar_static_f64[415]=(self.scalar_static_f64[414]*self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=p.p199;
        self.scalar_static_f64[417]=(self.scalar_static_f64[94]/self.scalar_static_f64[416]);
        self.scalar_static_f64[418]=(if self.scalar_static_bool[123]{self.scalar_static_f64[417]}else{0.0});
        self.scalar_static_f64[419]=p.p200;
        self.scalar_static_f64[420]=p.p197;
        self.scalar_static_f64[421]=(self.scalar_static_f64[418]/1.602176634e-19);
        self.scalar_static_f64[422]=(if self.scalar_static_bool[123]{self.scalar_static_f64[421]}else{self.scalar_static_f64[399]});
        self.scalar_static_f64[423]=p.p208;
        self.scalar_static_f64[424]=(self.scalar_static_f64[423]/3.0);
        self.scalar_static_f64[425]=(2.0*self.scalar_static_f64[423]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[425]/3.0);
        self.scalar_static_f64[427]=(self.scalar_static_f64[422]/3.24e17);
        self.scalar_static_f64[428]=f64::powf(self.scalar_static_f64[422],0.6666666666666666);
        self.scalar_static_f64[429]=p.p209;
        self.scalar_static_f64[430]=(-self.scalar_static_f64[422]);
        self.scalar_static_f64[431]=p.p202;
        self.scalar_static_f64[432]=p.p203;
        self.scalar_static_f64[433]=(self.scalar_static_f64[418]/self.scalar_static_f64[94]);
        self.scalar_static_f64[434]=(self.scalar_static_f64[113]*self.scalar_static_f64[418]);
        self.scalar_static_f64[435]=(self.scalar_static_f64[136]*self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=(self.scalar_static_f64[419]*self.scalar_static_f64[435]);
        self.scalar_static_f64[437]=p.p245;
        self.scalar_static_f64[438]=p.p244;
        self.scalar_static_f64[439]=p.p243;
        self.scalar_static_bool[128]=(!self.scalar_static_bool[122]);
        self.scalar_static_bool[129]=(self.scalar_static_bool[48]&&self.scalar_static_bool[128]);
        self.scalar_static_bool[130]=(self.scalar_static_bool[57]&&self.scalar_static_bool[122]);
        self.scalar_static_bool[131]=(self.scalar_static_bool[124]&&self.scalar_static_bool[130]);
        self.scalar_static_bool[132]=(self.scalar_static_bool[126]&&self.scalar_static_bool[130]);
        self.scalar_static_f64[440]=(if self.scalar_static_bool[130]{self.scalar_static_f64[417]}else{self.scalar_static_f64[418]});
        self.scalar_static_f64[441]=(self.scalar_static_f64[440]/1.602176634e-19);
        self.scalar_static_f64[442]=(if self.scalar_static_bool[130]{self.scalar_static_f64[441]}else{self.scalar_static_f64[422]});
        self.scalar_static_f64[443]=(self.scalar_static_f64[442]/3.24e17);
        self.scalar_static_f64[444]=f64::powf(self.scalar_static_f64[442],0.6666666666666666);
        self.scalar_static_f64[445]=(-self.scalar_static_f64[442]);
        self.scalar_static_f64[446]=(self.scalar_static_f64[440]/self.scalar_static_f64[94]);
        self.scalar_static_f64[447]=(self.scalar_static_f64[113]*self.scalar_static_f64[440]);
        self.scalar_static_f64[448]=(self.scalar_static_f64[136]*self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[419]*self.scalar_static_f64[448]);
        self.scalar_static_bool[133]=(self.scalar_static_bool[57]&&self.scalar_static_bool[128]);
        self.scalar_static_f64[450]=p.p157;
        self.scalar_static_bool[134]=(0.0!=self.scalar_static_f64[450]);
        self.scalar_static_bool[135]=(self.scalar_static_bool[48]&&self.scalar_static_bool[134]);
        self.scalar_static_bool[136]=(1.0==self.scalar_static_f64[450]);
        self.scalar_static_bool[137]=(self.scalar_static_bool[135]&&self.scalar_static_bool[136]);
        self.scalar_static_bool[138]=(!self.scalar_static_bool[136]);
        self.scalar_static_bool[139]=(self.scalar_static_bool[135]&&self.scalar_static_bool[138]);
        self.scalar_static_f64[451]=(if self.scalar_static_bool[135]{self.scalar_static_f64[417]}else{0.0});
        self.scalar_static_f64[452]=(self.scalar_static_f64[451]/1.602176634e-19);
        self.scalar_static_f64[453]=(if self.scalar_static_bool[135]{self.scalar_static_f64[452]}else{self.scalar_static_f64[442]});
        self.scalar_static_f64[454]=(self.scalar_static_f64[453]/3.24e17);
        self.scalar_static_f64[455]=f64::powf(self.scalar_static_f64[453],0.6666666666666666);
        self.scalar_static_f64[456]=(-self.scalar_static_f64[453]);
        self.scalar_static_f64[457]=(self.scalar_static_f64[451]/self.scalar_static_f64[94]);
        self.scalar_static_f64[458]=(self.scalar_static_f64[113]*self.scalar_static_f64[451]);
        self.scalar_static_f64[459]=(self.scalar_static_f64[136]*self.scalar_static_f64[458]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[419]*self.scalar_static_f64[459]);
        self.scalar_static_bool[140]=(!self.scalar_static_bool[134]);
        self.scalar_static_bool[141]=(self.scalar_static_bool[48]&&self.scalar_static_bool[140]);
        self.scalar_static_bool[142]=(self.scalar_static_bool[57]&&self.scalar_static_bool[134]);
        self.scalar_static_bool[143]=(self.scalar_static_bool[136]&&self.scalar_static_bool[142]);
        self.scalar_static_bool[144]=(self.scalar_static_bool[138]&&self.scalar_static_bool[142]);
        self.scalar_static_f64[461]=(if self.scalar_static_bool[142]{self.scalar_static_f64[417]}else{self.scalar_static_f64[451]});
        self.scalar_static_f64[462]=(self.scalar_static_f64[461]/1.602176634e-19);
        self.scalar_static_f64[463]=(if self.scalar_static_bool[142]{self.scalar_static_f64[462]}else{self.scalar_static_f64[453]});
        self.scalar_static_f64[464]=(self.scalar_static_f64[463]/3.24e17);
        self.scalar_static_f64[465]=f64::powf(self.scalar_static_f64[463],0.6666666666666666);
        self.scalar_static_f64[466]=(-self.scalar_static_f64[463]);
        self.scalar_static_f64[467]=(self.scalar_static_f64[461]/self.scalar_static_f64[94]);
        self.scalar_static_f64[468]=(self.scalar_static_f64[113]*self.scalar_static_f64[461]);
        self.scalar_static_f64[469]=(self.scalar_static_f64[136]*self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[419]*self.scalar_static_f64[469]);
        self.scalar_static_bool[145]=(self.scalar_static_bool[57]&&self.scalar_static_bool[140]);
        self.scalar_static_f64[471]=p.p255;
        self.scalar_static_bool[146]=(1.0==self.scalar_static_f64[471]);
        self.scalar_static_f64[472]=p.p258;
        self.scalar_static_f64[473]=p.p256;
        self.scalar_static_f64[474]=(self.scalar_static_f64[113]/3.0);
        self.scalar_static_f64[475]=p.p257;
        self.scalar_static_f64[476]=(self.scalar_static_f64[474]/self.scalar_static_f64[475]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[473]+self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(self.scalar_static_f64[472]*self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[136]*self.scalar_static_f64[475]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[112]*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(self.scalar_static_f64[478]/self.scalar_static_f64[480]);
        self.scalar_static_f64[482]=(if self.scalar_static_bool[146]{self.scalar_static_f64[481]}else{1000.0});
        self.scalar_static_bool[147]=(self.scalar_static_f64[482]>0.0);
        self.scalar_static_bool[148]=(self.scalar_static_bool[146]&&self.scalar_static_bool[147]);
        self.scalar_static_f64[483]=(1.0/self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=(if self.scalar_static_bool[148]{self.scalar_static_f64[483]}else{self.scalar_static_f64[482]});
        self.scalar_static_bool[149]=(!self.scalar_static_bool[147]);
        self.scalar_static_bool[150]=(self.scalar_static_bool[146]&&self.scalar_static_bool[149]);
        self.scalar_static_f64[485]=(if self.scalar_static_bool[150]{1000.0}else{self.scalar_static_f64[484]});
        self.scalar_static_bool[151]=(2.0==self.scalar_static_f64[471]);
        self.scalar_static_bool[152]=(!self.scalar_static_bool[146]);
        self.scalar_static_bool[153]=(self.scalar_static_bool[151]&&self.scalar_static_bool[152]);
        self.scalar_static_f64[486]=(if self.scalar_static_bool[153]{self.scalar_static_f64[481]}else{1000.0});
        self.scalar_static_f64[487]=(self.scalar_static_f64[114]/3.0);
        self.scalar_static_f64[488]=(self.scalar_static_f64[487]/self.scalar_static_f64[475]);
        self.scalar_static_f64[489]=(self.scalar_static_f64[472]*self.scalar_static_f64[488]);
        self.scalar_static_f64[490]=(self.scalar_static_f64[489]/self.scalar_static_f64[480]);
        self.scalar_static_f64[491]=(if self.scalar_static_bool[153]{self.scalar_static_f64[490]}else{1000.0});
        self.scalar_static_bool[154]=(self.scalar_static_f64[486]>0.0);
        self.scalar_static_bool[155]=(self.scalar_static_bool[153]&&self.scalar_static_bool[154]);
        self.scalar_static_f64[492]=(1.0/self.scalar_static_f64[486]);
        self.scalar_static_f64[493]=(if self.scalar_static_bool[155]{self.scalar_static_f64[492]}else{self.scalar_static_f64[486]});
        self.scalar_static_bool[156]=(!self.scalar_static_bool[154]);
        self.scalar_static_bool[157]=(self.scalar_static_bool[153]&&self.scalar_static_bool[156]);
        self.scalar_static_f64[494]=(if self.scalar_static_bool[157]{1000.0}else{self.scalar_static_f64[493]});
        self.scalar_static_bool[158]=(self.scalar_static_f64[491]>0.0);
        self.scalar_static_bool[159]=(self.scalar_static_bool[153]&&self.scalar_static_bool[158]);
        self.scalar_static_f64[495]=(1.0/self.scalar_static_f64[491]);
        self.scalar_static_f64[496]=(if self.scalar_static_bool[159]{self.scalar_static_f64[495]}else{self.scalar_static_f64[491]});
        self.scalar_static_bool[160]=(!self.scalar_static_bool[158]);
        self.scalar_static_bool[161]=(self.scalar_static_bool[153]&&self.scalar_static_bool[160]);
        self.scalar_static_f64[497]=(if self.scalar_static_bool[161]{1000.0}else{self.scalar_static_f64[496]});
        self.scalar_static_f64[498]=p.p210;
        self.scalar_static_f64[499]=(self.scalar_static_f64[193]*self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=p.p214;
        self.scalar_static_f64[501]=(self.scalar_static_f64[500]*self.scalar_static_f64[500]);
        self.scalar_static_f64[502]=p.p213;
        self.scalar_static_f64[503]=p.p211;
        self.scalar_static_f64[504]=(2.0*self.scalar_static_f64[500]);
        self.scalar_static_f64[505]=(self.scalar_static_f64[503]/self.scalar_static_f64[504]);
        self.scalar_static_bool[162]=(self.scalar_static_f64[502]<self.scalar_static_f64[505]);
        self.scalar_static_f64[506]=(if self.scalar_static_bool[162]{self.scalar_static_f64[502]}else{self.scalar_static_f64[505]});
        self.scalar_static_f64[507]=(if self.scalar_static_bool[151]{self.scalar_static_f64[506]}else{0.0});
        self.scalar_static_f64[508]=(self.scalar_static_f64[193]*self.scalar_static_f64[503]);
        self.scalar_static_f64[509]=(self.scalar_static_f64[193]*self.scalar_static_f64[507]);
        self.scalar_static_bool[163]=(!self.scalar_static_bool[151]);
        self.scalar_static_f64[510]=(if self.scalar_static_bool[163]{self.scalar_static_f64[506]}else{self.scalar_static_f64[507]});
        self.scalar_static_f64[511]=(self.scalar_static_f64[193]*self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=p.p212;
        self.scalar_static_f64[513]=(self.scalar_static_f64[193]*self.scalar_static_f64[512]);
        self.scalar_static_f64[514]=p.p215;
        self.scalar_static_f64[515]=(self.scalar_static_f64[193]*self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=p.p216;
        self.scalar_static_f64[517]=(self.scalar_static_f64[193]*self.scalar_static_f64[516]);
        self.scalar_static_f64[518]=p.p217;
        self.scalar_static_f64[519]=(self.scalar_static_f64[193]*self.scalar_static_f64[518]);
        self.scalar_static_f64[520]=p.p279;
        self.scalar_static_f64[521]=p.p285;
        self.scalar_static_f64[522]=p.p275;
        self.scalar_static_f64[523]=p.p283;
        self.scalar_static_f64[524]=p.p277;
        self.scalar_static_f64[525]=p.p281;
        self.scalar_static_f64[526]=p.p280;
        self.scalar_static_f64[527]=p.p286;
        self.scalar_static_f64[528]=p.p276;
        self.scalar_static_f64[529]=p.p284;
        self.scalar_static_f64[530]=p.p278;
        self.scalar_static_f64[531]=p.p282;
        self.scalar_static_f64[532]=p.p222;
        self.scalar_static_f64[533]=p.p220;
        self.scalar_static_f64[534]=p.p227;
        self.scalar_static_f64[535]=p.p221;
        self.scalar_static_f64[536]=p.p218;
        self.scalar_static_f64[537]=p.p226;
        self.scalar_static_f64[538]=p.p219;
        self.scalar_static_f64[539]=(self.scalar_static_f64[193]*self.scalar_static_f64[538]);
        self.scalar_static_f64[540]=p.p224;
        self.scalar_static_f64[541]=p.p225;
        self.scalar_static_f64[542]=p.p229;
        self.scalar_static_f64[543]=(self.scalar_static_f64[542]).ln();
        self.scalar_static_f64[544]=(-self.scalar_static_f64[543]);
        self.scalar_static_f64[545]=p.p228;
        self.scalar_static_f64[546]=(self.scalar_static_f64[544]/self.scalar_static_f64[545]);
        self.scalar_static_f64[547]={ let limited_exp_arg = self.scalar_static_f64[546]; if limited_exp_arg > 80.0 { 5.54062238439351e34 * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        self.scalar_static_f64[548]=(1.0-self.scalar_static_f64[547]);
        self.scalar_static_f64[549]=p.p230;
        self.scalar_static_f64[550]=p.p223;
        self.scalar_static_f64[551]=(1.0-self.scalar_static_f64[545]);
        self.scalar_static_f64[552]=(self.scalar_static_f64[542]*self.scalar_static_f64[550]);
        self.scalar_static_bool[164]=(1.0==self.scalar_static_f64[0]);
        self.scalar_static_bool[165]=(self.scalar_static_f64[1]>0.0);
        self.scalar_static_bool[166]=(self.scalar_static_bool[164]&&self.scalar_static_bool[165]);
        self.scalar_static_f64[553]=p.p6;
        self.scalar_static_f64[554]=p.p7;
        self.scalar_static_f64[555]=p.p250;
        self.scalar_static_f64[556]=p.p99;
        self.scalar_static_f64[557]=p.p97;
        self.scalar_static_f64[558]=p.p98;
        self.scalar_static_f64[559]=p.p108;
        self.scalar_static_f64[560]=p.p110;
        self.scalar_static_f64[561]=p.p109;
        self.scalar_static_f64[562]=p.p111;
        self.scalar_static_f64[563]=p.p119;
        self.scalar_static_f64[564]=p.p83;
        self.scalar_static_f64[565]=p.p135;
        self.scalar_static_f64[566]=(-self.scalar_static_f64[565]);
        self.scalar_static_f64[567]=p.p136;
        self.scalar_static_f64[568]=p.p144;
        self.scalar_static_f64[569]=(-self.scalar_static_f64[568]);
        self.scalar_static_f64[570]=p.p145;
        self.scalar_static_bool[167]=(self.scalar_static_bool[39]&&self.scalar_static_bool[48]);
        self.scalar_static_bool[168]=(self.scalar_static_bool[39]&&self.scalar_static_bool[57]);
        self.scalar_static_f64[571]=(self.scalar_static_f64[485]*self.scalar_static_f64[553]);
        self.scalar_static_f64[572]=(self.scalar_static_f64[494]*self.scalar_static_f64[553]);
        self.scalar_static_f64[573]=(self.scalar_static_f64[497]*self.scalar_static_f64[553]);
        self.scalar_static_f64[574]=p.p246;
        self.scalar_static_f64[575]=p.p251;
        self.scalar_static_f64[576]=p.p247;
        self.scalar_static_f64[577]=(self.scalar_static_f64[554]*self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=p.p252;
        self.scalar_static_f64[579]=p.p248;
        self.scalar_static_f64[580]=p.p253;
        self.scalar_static_f64[581]=p.p249;
        self.scalar_static_f64[582]=p.p254;
        self.scalar_static_f64[583]=(self.scalar_static_f64[193]*self.scalar_static_f64[533]);
        self.scalar_static_f64[584]=p.p33;
        self.scalar_static_f64[585]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[586]=(-self.scalar_static_f64[20]);
        self.scalar_static_f64[587]=(if self.scalar_static_bool[13]{self.scalar_static_f64[21]}else{0.0});
        self.scalar_static_f64[588]=(if self.scalar_static_bool[13]{self.scalar_static_f64[23]}else{0.0});
        self.scalar_static_f64[589]=(if self.scalar_static_bool[13]{self.scalar_static_f64[24]}else{0.0});
        self.scalar_static_f64[590]=(if self.scalar_static_bool[13]{self.scalar_static_f64[26]}else{0.0});
        self.scalar_static_f64[591]=(if self.scalar_static_bool[13]{self.scalar_static_f64[27]}else{0.0});
        self.scalar_static_f64[592]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_f64[593]=(if self.scalar_static_bool[16]{-1.0}else{0.0});
        self.scalar_static_f64[594]=(self.scalar_static_f64[29]*self.scalar_static_f64[592]);
        self.scalar_static_f64[595]=(self.scalar_static_f64[29]*self.scalar_static_f64[593]);
        self.scalar_static_f64[596]=(self.scalar_static_f64[28]*self.scalar_static_f64[594]);
        self.scalar_static_f64[597]=(-self.scalar_static_f64[596]);
        self.scalar_static_f64[598]=(self.scalar_static_f64[28]*self.scalar_static_f64[595]);
        self.scalar_static_f64[599]=(-self.scalar_static_f64[598]);
        self.scalar_static_f64[600]=(self.scalar_static_f64[30]*self.scalar_static_f64[592]);
        self.scalar_static_f64[601]=(self.scalar_static_f64[30]*self.scalar_static_f64[593]);
        self.scalar_static_f64[602]=(if self.scalar_static_bool[16]{self.scalar_static_f64[600]}else{0.0});
        self.scalar_static_f64[603]=(if self.scalar_static_bool[16]{self.scalar_static_f64[601]}else{0.0});
        self.scalar_static_f64[604]=(-2.0/self.scalar_static_f64[33]);
        self.scalar_static_f64[605]=(2.0/self.scalar_static_f64[33]);
        self.scalar_static_f64[606]=(1.0/self.scalar_static_f64[37]);
        self.scalar_static_f64[607]=(if self.scalar_static_bool[16]{self.scalar_static_f64[606]}else{0.0});
        self.scalar_static_f64[608]=(1.0/self.scalar_static_f64[6]);
        self.scalar_static_f64[609]=(self.scalar_static_f64[38]-1.0);
        self.scalar_static_f64[610]=(1.0/self.scalar_static_f64[41]);
        self.scalar_static_f64[611]=(-1.0/self.scalar_static_f64[41]);
        self.scalar_static_f64[612]=(1.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[613]=(-1.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[614]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_f64[615]=(-self.scalar_static_f64[71]);
        self.scalar_static_f64[616]=(-self.scalar_static_f64[72]);
        self.scalar_static_f64[617]=(self.scalar_static_f64[71]+self.scalar_static_f64[72]);
        self.scalar_static_f64[618]=(8.617087e-5*self.scalar_static_f64[77]);
        self.scalar_static_f64[619]=(-self.scalar_static_f64[618]);
        self.scalar_static_f64[620]=(-self.scalar_static_f64[80]);
        self.scalar_static_f64[621]=(-self.scalar_static_f64[81]);
        self.scalar_static_f64[622]=(self.scalar_static_f64[80]+self.scalar_static_f64[81]);
        self.scalar_static_f64[623]=(8.617087e-5*self.scalar_static_f64[86]);
        self.scalar_static_f64[624]=(-self.scalar_static_f64[623]);
        self.scalar_static_f64[625]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[626]=(self.scalar_static_f64[45]*self.scalar_static_f64[625]);
        self.scalar_static_f64[627]=(if self.scalar_static_bool[22]{self.scalar_static_f64[626]}else{0.0});
        self.scalar_static_f64[628]=(if self.scalar_static_bool[22]{self.scalar_static_f64[626]}else{self.scalar_static_f64[627]});
        self.scalar_static_f64[629]=(if self.scalar_static_bool[22]{self.scalar_static_f64[626]}else{self.scalar_static_f64[628]});
        self.scalar_static_f64[630]=(self.scalar_static_f64[50]*self.scalar_static_f64[625]);
        self.scalar_static_f64[631]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[629]});
        self.scalar_static_f64[632]=(if self.scalar_static_bool[22]{self.scalar_static_f64[630]}else{0.0});
        self.scalar_static_f64[633]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[631]});
        self.scalar_static_f64[634]=(if self.scalar_static_bool[22]{self.scalar_static_f64[630]}else{self.scalar_static_f64[632]});
        self.scalar_static_f64[635]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[633]});
        self.scalar_static_f64[636]=(if self.scalar_static_bool[22]{self.scalar_static_f64[630]}else{self.scalar_static_f64[634]});
        self.scalar_static_f64[637]=(self.scalar_static_f64[106]*self.scalar_static_f64[608]);
        self.scalar_static_f64[638]=(-self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=(self.scalar_static_f64[107]*self.scalar_static_f64[608]);
        self.scalar_static_f64[640]=(self.scalar_static_f64[127]-1.0);
        self.scalar_static_f64[641]=(self.scalar_static_f64[128]-1.0);
        self.scalar_static_f64[642]=(self.scalar_static_f64[134]-1.0);
        self.scalar_static_f64[643]=(self.scalar_static_f64[135]-1.0);
        self.scalar_static_f64[644]=(self.scalar_static_f64[141]*self.scalar_static_f64[608]);
        self.scalar_static_f64[645]=(self.scalar_static_f64[140]*self.scalar_static_f64[644]);
        self.scalar_static_f64[646]=(self.scalar_static_f64[143]*self.scalar_static_f64[608]);
        self.scalar_static_f64[647]=(self.scalar_static_f64[142]*self.scalar_static_f64[646]);
        self.scalar_static_f64[648]=(self.scalar_static_f64[145]*self.scalar_static_f64[608]);
        self.scalar_static_f64[649]=(self.scalar_static_f64[144]*self.scalar_static_f64[648]);
        self.scalar_static_f64[650]=(-self.scalar_static_f64[649]);
        self.scalar_static_f64[651]=(self.scalar_static_f64[150]-1.0);
        self.scalar_static_f64[652]=(self.scalar_static_f64[156]*self.scalar_static_f64[608]);
        self.scalar_static_f64[653]=(self.scalar_static_f64[162]*self.scalar_static_f64[608]);
        self.scalar_static_f64[654]=(self.scalar_static_f64[164]*self.scalar_static_f64[608]);
        self.scalar_static_f64[655]=(if self.scalar_static_bool[32]{self.scalar_static_f64[654]}else{0.0});
        self.scalar_static_f64[656]=(self.scalar_static_f64[165]*self.scalar_static_f64[608]);
        self.scalar_static_f64[657]=(if self.scalar_static_bool[32]{self.scalar_static_f64[656]}else{0.0});
        self.scalar_static_f64[658]=(self.scalar_static_f64[167]*self.scalar_static_f64[608]);
        self.scalar_static_f64[659]=(if self.scalar_static_bool[32]{self.scalar_static_f64[658]}else{0.0});
        self.scalar_static_f64[660]=(-self.scalar_static_f64[655]);
        self.scalar_static_f64[661]=(8.617087e-5*self.scalar_static_f64[657]);
        self.scalar_static_f64[662]=(self.scalar_static_f64[6]*self.scalar_static_f64[661]);
        self.scalar_static_f64[663]=(8.617087e-5*self.scalar_static_f64[659]);
        self.scalar_static_f64[664]=(self.scalar_static_f64[6]*self.scalar_static_f64[663]);
        self.scalar_static_f64[665]=(self.scalar_static_f64[170]*self.scalar_static_f64[608]);
        self.scalar_static_f64[666]=(self.scalar_static_f64[172]*self.scalar_static_f64[608]);
        self.scalar_static_f64[667]=(if self.scalar_static_bool[32]{self.scalar_static_f64[666]}else{0.0});
        self.scalar_static_f64[668]=(self.scalar_static_f64[173]*self.scalar_static_f64[608]);
        self.scalar_static_f64[669]=(if self.scalar_static_bool[32]{self.scalar_static_f64[668]}else{0.0});
        self.scalar_static_f64[670]=(self.scalar_static_f64[175]*self.scalar_static_f64[608]);
        self.scalar_static_f64[671]=(if self.scalar_static_bool[32]{self.scalar_static_f64[670]}else{0.0});
        self.scalar_static_f64[672]=(-self.scalar_static_f64[667]);
        self.scalar_static_f64[673]=(8.617087e-5*self.scalar_static_f64[669]);
        self.scalar_static_f64[674]=(self.scalar_static_f64[6]*self.scalar_static_f64[673]);
        self.scalar_static_f64[675]=(8.617087e-5*self.scalar_static_f64[671]);
        self.scalar_static_f64[676]=(self.scalar_static_f64[6]*self.scalar_static_f64[675]);
        self.scalar_static_f64[677]=(self.scalar_static_f64[178]*self.scalar_static_f64[608]);
        self.scalar_static_f64[678]=(if self.scalar_static_bool[35]{self.scalar_static_f64[654]}else{self.scalar_static_f64[655]});
        self.scalar_static_f64[679]=(if self.scalar_static_bool[35]{self.scalar_static_f64[656]}else{self.scalar_static_f64[657]});
        self.scalar_static_f64[680]=(if self.scalar_static_bool[35]{self.scalar_static_f64[658]}else{self.scalar_static_f64[659]});
        self.scalar_static_f64[681]=(self.scalar_static_f64[180]-1.0);
        self.scalar_static_f64[682]=(-self.scalar_static_f64[678]);
        self.scalar_static_f64[683]=(if self.scalar_static_bool[35]{self.scalar_static_f64[666]}else{self.scalar_static_f64[667]});
        self.scalar_static_f64[684]=(if self.scalar_static_bool[35]{self.scalar_static_f64[668]}else{self.scalar_static_f64[669]});
        self.scalar_static_f64[685]=(if self.scalar_static_bool[35]{self.scalar_static_f64[670]}else{self.scalar_static_f64[671]});
        self.scalar_static_f64[686]=(self.scalar_static_f64[182]-1.0);
        self.scalar_static_f64[687]=(-self.scalar_static_f64[683]);
        self.scalar_static_f64[688]=(if self.scalar_static_bool[38]{self.scalar_static_f64[654]}else{self.scalar_static_f64[678]});
        self.scalar_static_f64[689]=(if self.scalar_static_bool[38]{self.scalar_static_f64[656]}else{self.scalar_static_f64[679]});
        self.scalar_static_f64[690]=(if self.scalar_static_bool[38]{self.scalar_static_f64[658]}else{self.scalar_static_f64[680]});
        self.scalar_static_f64[691]=(-self.scalar_static_f64[688]);
        self.scalar_static_f64[692]=(if self.scalar_static_bool[38]{self.scalar_static_f64[666]}else{self.scalar_static_f64[683]});
        self.scalar_static_f64[693]=(if self.scalar_static_bool[38]{self.scalar_static_f64[668]}else{self.scalar_static_f64[684]});
        self.scalar_static_f64[694]=(if self.scalar_static_bool[38]{self.scalar_static_f64[670]}else{self.scalar_static_f64[685]});
        self.scalar_static_f64[695]=(-self.scalar_static_f64[692]);
        self.scalar_static_f64[696]=(self.scalar_static_f64[187]*self.scalar_static_f64[608]);
        self.scalar_static_f64[697]=(-self.scalar_static_f64[696]);
        self.scalar_static_f64[698]=(self.scalar_static_f64[64]*self.scalar_static_f64[697]);
        self.scalar_static_f64[699]=(if self.scalar_static_bool[39]{self.scalar_static_f64[698]}else{0.0});
        self.scalar_static_f64[700]=(self.scalar_static_f64[192]-1.0);
        self.scalar_static_f64[701]=(self.scalar_static_f64[195]-1.0);
        self.scalar_static_f64[702]=(self.scalar_static_f64[202]-1.0);
        self.scalar_static_f64[703]=(self.scalar_static_f64[204]*self.scalar_static_f64[608]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[203]*self.scalar_static_f64[703]);
        self.scalar_static_f64[705]=(if self.scalar_static_bool[39]{self.scalar_static_f64[704]}else{0.0});
        self.scalar_static_f64[706]=(self.scalar_static_f64[705]/self.scalar_static_f64[193]);
        self.scalar_static_f64[707]=(self.scalar_static_f64[68]*self.scalar_static_f64[697]);
        self.scalar_static_f64[708]=(self.scalar_static_f64[207]-1.0);
        self.scalar_static_f64[709]=(self.scalar_static_f64[212]-1.0);
        self.scalar_static_f64[710]=(self.scalar_static_f64[214]*self.scalar_static_f64[608]);
        self.scalar_static_f64[711]=(self.scalar_static_f64[213]*self.scalar_static_f64[710]);
        self.scalar_static_f64[712]=(if self.scalar_static_bool[39]{self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[713]=(self.scalar_static_f64[712]/self.scalar_static_f64[193]);
        self.scalar_static_f64[714]=(if self.scalar_static_bool[50]{-1.0}else{0.0});
        self.scalar_static_f64[715]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[716]=(if self.scalar_static_bool[52]{-1.0}else{0.0});
        self.scalar_static_f64[717]=(if self.scalar_static_bool[52]{1.0}else{0.0});
        self.scalar_static_f64[718]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_f64[719]=(if self.scalar_static_bool[54]{-1.0}else{self.scalar_static_f64[716]});
        self.scalar_static_f64[720]=(if self.scalar_static_bool[54]{0.0}else{self.scalar_static_f64[717]});
        self.scalar_static_f64[721]=(self.scalar_static_f64[220]*self.scalar_static_f64[608]);
        self.scalar_static_f64[722]=(self.scalar_static_f64[246]-1.0);
        self.scalar_static_f64[723]=(if self.scalar_static_bool[59]{0.0}else{self.scalar_static_f64[718]});
        self.scalar_static_f64[724]=(if self.scalar_static_bool[59]{-1.0}else{self.scalar_static_f64[719]});
        self.scalar_static_f64[725]=(if self.scalar_static_bool[59]{1.0}else{self.scalar_static_f64[720]});
        self.scalar_static_f64[726]=(if self.scalar_static_bool[60]{1.0}else{self.scalar_static_f64[723]});
        self.scalar_static_f64[727]=(if self.scalar_static_bool[60]{-1.0}else{self.scalar_static_f64[724]});
        self.scalar_static_f64[728]=(if self.scalar_static_bool[60]{0.0}else{self.scalar_static_f64[725]});
        self.scalar_static_f64[729]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_f64[730]=(if self.scalar_static_bool[63]{-1.0}else{0.0});
        self.scalar_static_f64[731]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[732]=(if self.scalar_static_bool[65]{-1.0}else{0.0});
        self.scalar_static_f64[733]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_f64[734]=(if self.scalar_static_bool[67]{0.0}else{self.scalar_static_f64[731]});
        self.scalar_static_f64[735]=(if self.scalar_static_bool[67]{-1.0}else{self.scalar_static_f64[732]});
        self.scalar_static_f64[736]=(if self.scalar_static_bool[71]{0.0}else{self.scalar_static_f64[733]});
        self.scalar_static_f64[737]=(if self.scalar_static_bool[71]{-1.0}else{0.0});
        self.scalar_static_f64[738]=(if self.scalar_static_bool[71]{1.0}else{self.scalar_static_f64[734]});
        self.scalar_static_f64[739]=(if self.scalar_static_bool[71]{0.0}else{self.scalar_static_f64[735]});
        self.scalar_static_f64[740]=(if self.scalar_static_bool[72]{1.0}else{self.scalar_static_f64[736]});
        self.scalar_static_f64[741]=(if self.scalar_static_bool[72]{-1.0}else{self.scalar_static_f64[737]});
        self.scalar_static_f64[742]=(if self.scalar_static_bool[72]{0.0}else{self.scalar_static_f64[738]});
        self.scalar_static_f64[743]=(if self.scalar_static_bool[72]{0.0}else{self.scalar_static_f64[739]});
        self.scalar_static_f64[744]=(if self.scalar_static_bool[75]{-1.0}else{0.0});
        self.scalar_static_f64[745]=(if self.scalar_static_bool[75]{1.0}else{0.0});
        self.scalar_static_f64[746]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_f64[747]=(if self.scalar_static_bool[77]{-1.0}else{0.0});
        self.scalar_static_f64[748]=(if self.scalar_static_bool[79]{1.0}else{0.0});
        self.scalar_static_f64[749]=(if self.scalar_static_bool[79]{0.0}else{self.scalar_static_f64[746]});
        self.scalar_static_f64[750]=(if self.scalar_static_bool[79]{-1.0}else{self.scalar_static_f64[747]});
        self.scalar_static_f64[751]=(self.scalar_static_f64[284]*self.scalar_static_f64[608]);
        self.scalar_static_f64[752]=(-self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(self.scalar_static_f64[310]-1.0);
        self.scalar_static_f64[754]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[748]});
        self.scalar_static_f64[755]=(if self.scalar_static_bool[83]{-1.0}else{0.0});
        self.scalar_static_f64[756]=(if self.scalar_static_bool[83]{1.0}else{self.scalar_static_f64[749]});
        self.scalar_static_f64[757]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[750]});
        self.scalar_static_f64[758]=(if self.scalar_static_bool[84]{1.0}else{self.scalar_static_f64[754]});
        self.scalar_static_f64[759]=(if self.scalar_static_bool[84]{-1.0}else{self.scalar_static_f64[755]});
        self.scalar_static_f64[760]=(if self.scalar_static_bool[84]{0.0}else{self.scalar_static_f64[756]});
        self.scalar_static_f64[761]=(if self.scalar_static_bool[84]{0.0}else{self.scalar_static_f64[757]});
        self.scalar_static_f64[762]=(if self.scalar_static_bool[82]{self.scalar_static_f64[759]}else{0.0});
        self.scalar_static_f64[763]=(if self.scalar_static_bool[87]{1.0}else{0.0});
        self.scalar_static_f64[764]=(if self.scalar_static_bool[87]{-1.0}else{0.0});
        self.scalar_static_f64[765]=(if self.scalar_static_bool[89]{1.0}else{0.0});
        self.scalar_static_f64[766]=(if self.scalar_static_bool[89]{-1.0}else{0.0});
        self.scalar_static_f64[767]=(if self.scalar_static_bool[91]{1.0}else{0.0});
        self.scalar_static_f64[768]=(if self.scalar_static_bool[91]{0.0}else{self.scalar_static_f64[765]});
        self.scalar_static_f64[769]=(if self.scalar_static_bool[91]{-1.0}else{self.scalar_static_f64[766]});
        self.scalar_static_f64[770]=(if self.scalar_static_bool[95]{0.0}else{self.scalar_static_f64[767]});
        self.scalar_static_f64[771]=(if self.scalar_static_bool[95]{-1.0}else{0.0});
        self.scalar_static_f64[772]=(if self.scalar_static_bool[95]{1.0}else{self.scalar_static_f64[768]});
        self.scalar_static_f64[773]=(if self.scalar_static_bool[95]{0.0}else{self.scalar_static_f64[769]});
        self.scalar_static_f64[774]=(if self.scalar_static_bool[96]{1.0}else{self.scalar_static_f64[770]});
        self.scalar_static_f64[775]=(if self.scalar_static_bool[96]{-1.0}else{self.scalar_static_f64[771]});
        self.scalar_static_f64[776]=(if self.scalar_static_bool[96]{0.0}else{self.scalar_static_f64[772]});
        self.scalar_static_f64[777]=(if self.scalar_static_bool[96]{0.0}else{self.scalar_static_f64[773]});
        self.scalar_static_f64[778]=(if self.scalar_static_bool[94]{self.scalar_static_f64[775]}else{0.0});
        self.scalar_static_f64[779]=(if self.scalar_static_bool[99]{-1.0}else{0.0});
        self.scalar_static_f64[780]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_f64[781]=(if self.scalar_static_bool[101]{1.0}else{0.0});
        self.scalar_static_f64[782]=(if self.scalar_static_bool[101]{-1.0}else{0.0});
        self.scalar_static_f64[783]=(if self.scalar_static_bool[103]{1.0}else{0.0});
        self.scalar_static_f64[784]=(if self.scalar_static_bool[103]{0.0}else{self.scalar_static_f64[781]});
        self.scalar_static_f64[785]=(if self.scalar_static_bool[103]{-1.0}else{self.scalar_static_f64[782]});
        self.scalar_static_f64[786]=(self.scalar_static_f64[348]*self.scalar_static_f64[608]);
        self.scalar_static_f64[787]=(-self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=(self.scalar_static_f64[374]-1.0);
        self.scalar_static_f64[789]=(if self.scalar_static_bool[107]{0.0}else{self.scalar_static_f64[783]});
        self.scalar_static_f64[790]=(if self.scalar_static_bool[107]{-1.0}else{0.0});
        self.scalar_static_f64[791]=(if self.scalar_static_bool[107]{1.0}else{self.scalar_static_f64[784]});
        self.scalar_static_f64[792]=(if self.scalar_static_bool[107]{0.0}else{self.scalar_static_f64[785]});
        self.scalar_static_f64[793]=(if self.scalar_static_bool[108]{1.0}else{self.scalar_static_f64[789]});
        self.scalar_static_f64[794]=(if self.scalar_static_bool[108]{-1.0}else{self.scalar_static_f64[790]});
        self.scalar_static_f64[795]=(if self.scalar_static_bool[108]{0.0}else{self.scalar_static_f64[791]});
        self.scalar_static_f64[796]=(if self.scalar_static_bool[108]{0.0}else{self.scalar_static_f64[792]});
        self.scalar_static_f64[797]=(if self.scalar_static_bool[106]{self.scalar_static_f64[794]}else{0.0});
        self.scalar_static_f64[798]=(if self.scalar_static_bool[111]{1.0}else{0.0});
        self.scalar_static_f64[799]=(if self.scalar_static_bool[111]{-1.0}else{0.0});
        self.scalar_static_f64[800]=(if self.scalar_static_bool[113]{1.0}else{0.0});
        self.scalar_static_f64[801]=(if self.scalar_static_bool[113]{-1.0}else{0.0});
        self.scalar_static_f64[802]=(if self.scalar_static_bool[115]{1.0}else{0.0});
        self.scalar_static_f64[803]=(if self.scalar_static_bool[115]{0.0}else{self.scalar_static_f64[800]});
        self.scalar_static_f64[804]=(if self.scalar_static_bool[115]{-1.0}else{self.scalar_static_f64[801]});
        self.scalar_static_f64[805]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[802]});
        self.scalar_static_f64[806]=(if self.scalar_static_bool[119]{-1.0}else{0.0});
        self.scalar_static_f64[807]=(if self.scalar_static_bool[119]{1.0}else{self.scalar_static_f64[803]});
        self.scalar_static_f64[808]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[804]});
        self.scalar_static_f64[809]=(if self.scalar_static_bool[120]{1.0}else{self.scalar_static_f64[805]});
        self.scalar_static_f64[810]=(if self.scalar_static_bool[120]{-1.0}else{self.scalar_static_f64[806]});
        self.scalar_static_f64[811]=(if self.scalar_static_bool[120]{0.0}else{self.scalar_static_f64[807]});
        self.scalar_static_f64[812]=(if self.scalar_static_bool[120]{0.0}else{self.scalar_static_f64[808]});
        self.scalar_static_f64[813]=(if self.scalar_static_bool[118]{self.scalar_static_f64[810]}else{0.0});
        self.scalar_static_f64[814]=(if self.scalar_static_bool[123]{-1.0}else{0.0});
        self.scalar_static_f64[815]=(if self.scalar_static_bool[123]{1.0}else{0.0});
        self.scalar_static_f64[816]=(if self.scalar_static_bool[125]{1.0}else{0.0});
        self.scalar_static_f64[817]=(if self.scalar_static_bool[125]{-1.0}else{0.0});
        self.scalar_static_f64[818]=(if self.scalar_static_bool[127]{1.0}else{0.0});
        self.scalar_static_f64[819]=(if self.scalar_static_bool[127]{0.0}else{self.scalar_static_f64[816]});
        self.scalar_static_f64[820]=(if self.scalar_static_bool[127]{-1.0}else{self.scalar_static_f64[817]});
        self.scalar_static_f64[821]=(self.scalar_static_f64[412]*self.scalar_static_f64[608]);
        self.scalar_static_f64[822]=(-self.scalar_static_f64[821]);
        self.scalar_static_f64[823]=(self.scalar_static_f64[438]-1.0);
        self.scalar_static_f64[824]=(if self.scalar_static_bool[131]{0.0}else{self.scalar_static_f64[818]});
        self.scalar_static_f64[825]=(if self.scalar_static_bool[131]{-1.0}else{0.0});
        self.scalar_static_f64[826]=(if self.scalar_static_bool[131]{1.0}else{self.scalar_static_f64[819]});
        self.scalar_static_f64[827]=(if self.scalar_static_bool[131]{0.0}else{self.scalar_static_f64[820]});
        self.scalar_static_f64[828]=(if self.scalar_static_bool[132]{1.0}else{self.scalar_static_f64[824]});
        self.scalar_static_f64[829]=(if self.scalar_static_bool[132]{-1.0}else{self.scalar_static_f64[825]});
        self.scalar_static_f64[830]=(if self.scalar_static_bool[132]{0.0}else{self.scalar_static_f64[826]});
        self.scalar_static_f64[831]=(if self.scalar_static_bool[132]{0.0}else{self.scalar_static_f64[827]});
        self.scalar_static_f64[832]=(if self.scalar_static_bool[130]{self.scalar_static_f64[829]}else{0.0});
        self.scalar_static_f64[833]=(if self.scalar_static_bool[135]{1.0}else{0.0});
        self.scalar_static_f64[834]=(if self.scalar_static_bool[135]{-1.0}else{0.0});
        self.scalar_static_f64[835]=(if self.scalar_static_bool[137]{1.0}else{0.0});
        self.scalar_static_f64[836]=(if self.scalar_static_bool[137]{-1.0}else{0.0});
        self.scalar_static_f64[837]=(if self.scalar_static_bool[139]{1.0}else{0.0});
        self.scalar_static_f64[838]=(if self.scalar_static_bool[139]{0.0}else{self.scalar_static_f64[835]});
        self.scalar_static_f64[839]=(if self.scalar_static_bool[139]{-1.0}else{self.scalar_static_f64[836]});
        self.scalar_static_f64[840]=(if self.scalar_static_bool[143]{0.0}else{self.scalar_static_f64[837]});
        self.scalar_static_f64[841]=(if self.scalar_static_bool[143]{-1.0}else{0.0});
        self.scalar_static_f64[842]=(if self.scalar_static_bool[143]{1.0}else{self.scalar_static_f64[838]});
        self.scalar_static_f64[843]=(if self.scalar_static_bool[143]{0.0}else{self.scalar_static_f64[839]});
        self.scalar_static_f64[844]=(if self.scalar_static_bool[144]{1.0}else{self.scalar_static_f64[840]});
        self.scalar_static_f64[845]=(if self.scalar_static_bool[144]{-1.0}else{self.scalar_static_f64[841]});
        self.scalar_static_f64[846]=(if self.scalar_static_bool[144]{0.0}else{self.scalar_static_f64[842]});
        self.scalar_static_f64[847]=(if self.scalar_static_bool[144]{0.0}else{self.scalar_static_f64[843]});
        self.scalar_static_f64[848]=(if self.scalar_static_bool[142]{self.scalar_static_f64[845]}else{0.0});
        self.scalar_static_f64[849]=(-self.scalar_static_f64[499]);
        self.scalar_static_f64[850]=(if self.scalar_static_bool[151]{self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[851]=(if self.scalar_static_bool[151]{self.scalar_static_f64[499]}else{0.0});
        self.scalar_static_f64[852]=(-self.scalar_static_f64[500]);
        self.scalar_static_f64[853]=(if self.scalar_static_bool[163]{self.scalar_static_f64[499]}else{0.0});
        self.scalar_static_f64[854]=(if self.scalar_static_bool[163]{self.scalar_static_f64[849]}else{self.scalar_static_f64[850]});
        self.scalar_static_f64[855]=(if self.scalar_static_bool[163]{0.0}else{self.scalar_static_f64[851]});
        self.scalar_static_f64[856]=(-self.scalar_static_f64[513]);
        self.scalar_static_f64[857]=(-self.scalar_static_f64[515]);
        self.scalar_static_f64[858]=(-self.scalar_static_f64[517]);
        self.scalar_static_f64[859]=(-self.scalar_static_f64[519]);
        self.scalar_static_f64[860]=(self.scalar_static_f64[521]*self.scalar_static_f64[608]);
        self.scalar_static_f64[861]=(self.scalar_static_f64[523]*self.scalar_static_f64[608]);
        self.scalar_static_f64[862]=(self.scalar_static_f64[525]*self.scalar_static_f64[608]);
        self.scalar_static_f64[863]=(self.scalar_static_f64[527]*self.scalar_static_f64[608]);
        self.scalar_static_f64[864]=(self.scalar_static_f64[529]*self.scalar_static_f64[608]);
        self.scalar_static_f64[865]=(self.scalar_static_f64[531]*self.scalar_static_f64[608]);
        self.scalar_static_f64[866]=(-self.scalar_static_f64[863]);
        self.scalar_static_f64[867]=(-self.scalar_static_f64[860]);
        self.scalar_static_f64[868]=(self.scalar_static_f64[534]*self.scalar_static_f64[608]);
        self.scalar_static_f64[869]=(self.scalar_static_f64[537]*self.scalar_static_f64[608]);
        self.scalar_static_f64[870]=(-self.scalar_static_f64[869]);
        self.scalar_static_f64[871]=(-self.scalar_static_f64[539]);
        self.scalar_static_f64[872]=(self.scalar_static_f64[541]*self.scalar_static_f64[608]);
        self.scalar_static_f64[873]=(-self.scalar_static_f64[872]);
        self.scalar_static_f64[874]=(self.scalar_static_f64[548]*self.scalar_static_f64[873]);
        self.scalar_static_f64[875]=(self.scalar_static_f64[550]*self.scalar_static_f64[873]);
        self.scalar_static_f64[876]=(1.0/self.scalar_static_f64[558]);
        self.scalar_static_f64[877]=(if self.scalar_static_bool[10]{self.scalar_static_f64[876]}else{0.0});
        self.scalar_static_f64[878]=(1.0/self.scalar_static_f64[559]);
        self.scalar_static_f64[879]=(if self.scalar_static_bool[13]{self.scalar_static_f64[878]}else{0.0});
        self.scalar_static_f64[880]=(1.0/self.scalar_static_f64[561]);
        self.scalar_static_f64[881]=(if self.scalar_static_bool[13]{self.scalar_static_f64[880]}else{0.0});
        self.scalar_static_f64[882]=(if self.scalar_static_bool[13]{-1.0}else{0.0});
        self.scalar_static_f64[883]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_f64[884]=(1.0/self.scalar_static_f64[563]);
        self.scalar_static_f64[885]=(if self.scalar_static_bool[16]{self.scalar_static_f64[884]}else{0.0});
        self.scalar_static_f64[886]=(if self.scalar_static_bool[19]{self.scalar_static_f64[874]}else{0.0});
        self.scalar_static_f64[887]=(self.scalar_static_f64[553]*self.scalar_static_f64[638]);
        self.scalar_static_f64[888]=(-self.scalar_static_f64[571]);
        self.scalar_static_f64[889]=(if self.scalar_static_bool[146]{self.scalar_static_f64[571]}else{0.0});
        self.scalar_static_f64[890]=(if self.scalar_static_bool[146]{self.scalar_static_f64[888]}else{0.0});
        self.scalar_static_f64[891]=(-self.scalar_static_f64[572]);
        self.scalar_static_f64[892]=(if self.scalar_static_bool[153]{self.scalar_static_f64[572]}else{0.0});
        self.scalar_static_f64[893]=(if self.scalar_static_bool[153]{self.scalar_static_f64[891]}else{0.0});
        self.scalar_static_f64[894]=(-self.scalar_static_f64[573]);
        self.scalar_static_f64[895]=(if self.scalar_static_bool[153]{self.scalar_static_f64[894]}else{0.0});
        self.scalar_static_f64[896]=(if self.scalar_static_bool[153]{self.scalar_static_f64[573]}else{0.0});
        self.scalar_static_f64[897]=(-self.scalar_static_f64[583]);
        self.scalar_static_f64[898]=(1.0/self.scalar_static_f64[1]);
        self.scalar_static_f64[899]=(if self.scalar_static_bool[166]{self.scalar_static_f64[898]}else{0.0});
    }
}
