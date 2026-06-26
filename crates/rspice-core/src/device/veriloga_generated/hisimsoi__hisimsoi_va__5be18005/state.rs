#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::support::{ReactiveScratch as GenericReactiveScratch, Scratch as GenericScratch};

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
    pub p287: f64,
    pub p288: f64,
    pub p289: f64,
    pub p290: f64,
    pub p291: f64,
    pub p292: f64,
    pub p293: f64,
    pub p294: f64,
    pub p295: f64,
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
            params.p0 = 5e-6;
            params.p1 = 5e-6;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 1.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 1e-5;
            params.p16 = 0.0;
            params.p17 = 0.0;
            params.p18 = 1.0;
            params.p19 = 0.0;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 1.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 0.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = 0.0;
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 0.0;
            params.p38 = 0.0;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 1.0;
            params.p42 = 1.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.001;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 1.0;
            params.p51 = 1.5;
            params.p52 = 7000000.0;
            params.p53 = 9.025e-5;
            params.p54 = 1e-7;
            params.p55 = 1.1785;
            params.p56 = 0.0;
            params.p57 = params.p56;
            validate_finite_parameter("XLDC", params.p57).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p58 = 0.0;
            params.p59 = 1e19;
            params.p60 = 0.0;
            params.p61 = params.p60;
            validate_finite_parameter("XWDC", params.p61).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p62 = 1e-6;
            params.p63 = 1e-6;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 2.0;
            params.p67 = 0.0;
            params.p68 = -1.0;
            params.p69 = 1.1;
            params.p70 = 1e-8;
            params.p71 = 1e-8;
            params.p72 = 0.0;
            params.p73 = 1e17;
            params.p74 = 0.0;
            params.p75 = 1.0;
            params.p76 = 0.0;
            params.p77 = 1.0;
            params.p78 = 0.0;
            params.p79 = 1.0;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.0;
            params.p88 = 0.23;
            params.p89 = 0.0;
            params.p90 = 1.0;
            params.p91 = 0.0;
            params.p92 = 1.0;
            params.p93 = 0.5;
            params.p94 = 0.0;
            params.p95 = 300.0;
            params.p96 = 30.0;
            params.p97 = 0.3;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 0.0;
            params.p101 = 1.0;
            params.p102 = 0.0;
            params.p103 = 1.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 2000000000000000.0;
            params.p107 = 2.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 1.0;
            params.p111 = 1.0;
            params.p112 = 1.5;
            params.p113 = if (params.p50 > 0.0) { 2.0 } else { 1.0 };
            validate_parameter("BB", params.p113, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p114 = 10.0;
            params.p115 = 10.0;
            params.p116 = 0.0;
            params.p117 = 0.01;
            params.p118 = 20.0;
            params.p119 = 0.0025;
            params.p120 = 1.0;
            params.p121 = 2e-6;
            params.p122 = 3.0;
            params.p123 = 3e-8;
            params.p124 = 0.5;
            params.p125 = 0.0;
            params.p126 = 1.0;
            params.p127 = 0.8;
            params.p128 = 0.0;
            params.p129 = 1.0;
            params.p130 = 0.0;
            params.p131 = 1.0;
            params.p132 = 0.0;
            params.p133 = 1.0;
            params.p134 = -1.0;
            params.p135 = 0.0;
            params.p136 = 1.0;
            params.p137 = 0.002;
            params.p138 = 1e-8;
            params.p139 = 1e-20;
            params.p140 = 1.5;
            params.p141 = 0.35;
            params.p142 = 0.0;
            params.p143 = 0.0;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = 0.0;
            params.p147 = 0.0;
            params.p148 = 5e17;
            params.p149 = 0.0;
            params.p150 = 0.0;
            params.p151 = 1.0;
            params.p152 = 0.0;
            params.p153 = 1.0;
            params.p154 = 0.0;
            params.p155 = 0.0;
            params.p156 = 0.0;
            params.p157 = 0.0;
            params.p158 = 0.0;
            params.p159 = 0.0;
            params.p160 = 1.0;
            params.p161 = 0.0;
            params.p162 = 0.0;
            params.p163 = 1.0;
            params.p164 = 0.0;
            params.p165 = 0.0;
            params.p166 = 1.0;
            params.p167 = 0.0;
            params.p168 = 0.0;
            params.p169 = 0.0;
            params.p170 = 0.0;
            params.p171 = 2.1e-7;
            params.p172 = 0.6;
            params.p173 = 0.0001;
            params.p174 = 1.0;
            params.p175 = 2.0;
            params.p176 = 0.0;
            params.p177 = 0.0016;
            params.p178 = 0.0;
            params.p179 = 0.0005;
            params.p180 = 5e-10;
            params.p181 = 5e-10;
            params.p182 = 0.33;
            params.p183 = 0.33;
            params.p184 = 0.33;
            params.p185 = 1.0;
            params.p186 = 1.0;
            params.p187 = 1.0;
            params.p188 = 3e-8;
            params.p189 = 0.7;
            params.p190 = 2.0;
            params.p191 = 1.0;
            params.p192 = 1.0;
            params.p193 = 0.0;
            params.p194 = 0.01;
            params.p195 = 0.1;
            params.p196 = 0.0;
            params.p197 = 1.0;
            params.p198 = 0.0;
            params.p199 = 0.0;
            params.p200 = 1.0;
            params.p201 = 5e18;
            params.p202 = 0.0;
            params.p203 = 1.0;
            params.p204 = 0.0;
            params.p205 = 0.0;
            params.p206 = 0.0;
            params.p207 = 5e-6;
            params.p208 = 1000000.0;
            params.p209 = 0.3;
            params.p210 = 0.0;
            params.p211 = 0.2;
            params.p212 = 0.5;
            params.p213 = 10000.0;
            params.p214 = 20000000.0;
            params.p215 = 0.3;
            params.p216 = 4.0;
            params.p217 = 7500.0;
            params.p218 = 0.25;
            params.p219 = 1e-6;
            params.p220 = 1e-15;
            params.p221 = 5000000.0;
            params.p222 = -5000000.0;
            params.p223 = 5e-16;
            params.p224 = 1.0;
            params.p225 = 0.0;
            params.p226 = 0.01;
            params.p227 = 0.005;
            params.p228 = 10000000000.0;
            params.p229 = 1e-19;
            params.p230 = 0.0;
            params.p231 = 1.0;
            params.p232 = 27.0;
            params.p233 = 1e-10;
            params.p234 = 0.7;
            params.p235 = 8e-7;
            params.p236 = 3.5e-9;
            params.p237 = 5e-8;
            params.p238 = 5e-8;
            params.p239 = 1.1e-7;
            params.p240 = 3e17;
            params.p241 = 400000000000000.0;
            params.p242 = 0.1;
            params.p243 = 1e-7;
            params.p244 = 0.0;
            params.p245 = 3.5;
            params.p246 = 0.0;
            params.p247 = 1.0;
            params.p248 = 0.0;
            params.p249 = 0.0;
            params.p250 = 0.0;
            params.p251 = 0.0;
            params.p252 = 1.0;
            params.p253 = ((-5.0) * params.p50);
            validate_finite_parameter("VGSMIN", params.p253).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p254 = 25000.0;
            params.p255 = 1.0;
            params.p256 = 1.0;
            params.p257 = 1e-6;
            params.p258 = 1e-6;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 0.0;
            params.p262 = 0.0;
            params.p263 = 0.0;
            params.p264 = 1e19;
            params.p265 = 1000.0;
            params.p266 = 1000.0;
            params.p267 = 30000000.0;
            params.p268 = 30000000.0;
            params.p269 = 0.0;
            params.p270 = 0.0;
            params.p271 = 1e-6;
            params.p272 = 1.0;
            params.p273 = 1.0;
            params.p274 = 0.0;
            params.p275 = 0.0;
            params.p276 = 1.0;
            params.p277 = 0.0;
            params.p278 = 1.0;
            params.p279 = 0.0;
            params.p280 = 1.0;
            params.p281 = 0.0;
            params.p282 = 0.0;
            params.p283 = params.p237;
            validate_parameter("XJPT", params.p283, Some((0.0, "0.0")), true, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p284 = 1e20;
            params.p285 = 0.0;
            params.p286 = 0.0;
            params.p287 = 0.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = 50.0;
            params.p291 = 50.0;
            params.p292 = (params.p68 + 1.12);
            validate_finite_parameter("VFBBTP", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p293 = 0.0;
            params.p294 = 0.0;
            params.p295 = 0.0;
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
    pub nodes: [usize; 19],
    pub branches: [usize; 20],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 296]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 15]>,
    pub(crate) ddt_state_previous: Box<[f64; 15]>,
    pub(crate) ddt_state_initialized: Box<[bool; 15]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<1854, 19, 20>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<1854, 19, 20>>>,
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
            ddt_state_initialized: self.ddt_state_initialized.clone(),
            idt_state_current: self.idt_state_current.clone(),
            idt_state_previous: self.idt_state_previous.clone(),
            idt_state_initialized: self.idt_state_initialized.clone(),
            time: self.time,
            timestep: self.timestep,
            scratch: None,
            reactive_scratch: None,
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
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::new_box(),
            param_given: boxed_zero_bool_array::<{ Self::PARAMETER_COUNT }>(),
            multiplicity: 1.0,
            ddt_state_current: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_previous: boxed_zero_f64_array::<{ Self::DDT_STATE_COUNT }>(),
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: Some(GenericReactiveScratch::new_box()),
        }
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
        let scratch = self.scratch.take();
        let reactive_scratch = self.reactive_scratch.take();
        let Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            scratch: _,
            reactive_scratch: _,
        } = snapshot;
        *self = Self {
            nodes,
            branches,
            params,
            param_given,
            multiplicity,
            ddt_state_current,
            ddt_state_previous,
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
            scratch,
            reactive_scratch,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "xgw" => { validate_parameter("XGW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "xgl" => { validate_parameter("XGL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "pdbcp" => { validate_parameter("PDBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "psbcp" => { validate_parameter("PSBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "lod" => { validate_parameter("LOD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "temp" => { validate_parameter("TEMP", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "nbt" => { validate_finite_parameter("NBT", value)?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "lbt" => { validate_parameter("LBT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "wbtp" => { validate_parameter("WBTP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "wbtn" => { validate_parameter("WBTN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "abtn" => { validate_parameter("ABTN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "abtp" => { validate_parameter("ABTP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "coadov" => { validate_parameter("COADOV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "coisub" => { validate_parameter("COISUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "cofbe" => { validate_parameter("COFBE", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "coiigs" => { validate_parameter("COIIGS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "cogidl" => { validate_parameter("COGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "coovlp" => { validate_parameter("COOVLP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "coign" => { validate_parameter("COIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "coflick" => { validate_parameter("COFLICK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "cothrml" => { validate_parameter("COTHRML", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "coisti" => { validate_parameter("COISTI", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "conqs" => { validate_parameter("CONQS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "corg" => { validate_parameter("CORG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "coievb" => { validate_parameter("COIEVB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "cohist" => { validate_parameter("COHIST", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "coselfheat" => { validate_parameter("COSELFHEAT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "covbsbiz" => { validate_parameter("COVBSBIZ", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "colgleff" => { validate_parameter("COLGLEFF", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "coqovsm" => { validate_parameter("COQOVSM", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "coqbdsm" => { validate_parameter("COQBDSM", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "cobcnode" => { validate_finite_parameter("COBCNODE", value)?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "cosubscale" => { validate_parameter("COSUBSCALE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "coisubfb" => { validate_parameter("COISUBFB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "info" => { validate_finite_parameter("INFO", value)?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "qhsmax" => { validate_finite_parameter("QHSMAX", value)?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "dvgpsub" => { validate_finite_parameter("DVGPSUB", value)?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "dvbssub" => { validate_finite_parameter("DVBSSUB", value)?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "bgtmp1" => { validate_finite_parameter("BGTMP1", value)?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "bgtmp2" => { validate_finite_parameter("BGTMP2", value)?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "eg0" => { validate_parameter("EG0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "xld" => { validate_finite_parameter("XLD", value)?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "xldc" => { validate_finite_parameter("XLDC", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "vfbover" => { validate_finite_parameter("VFBOVER", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "nover" => { validate_parameter("NOVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "xwd" => { validate_finite_parameter("XWD", value)?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "xwdc" => { validate_finite_parameter("XWDC", value)?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "xqy" => { validate_parameter("XQY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "xqy1" => { validate_finite_parameter("XQY1", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "xqy2" => { validate_finite_parameter("XQY2", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "vfbc" => { validate_finite_parameter("VFBC", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "vbi" => { validate_finite_parameter("VBI", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "parl1" => { validate_finite_parameter("PARL1", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "parl2" => { validate_finite_parameter("PARL2", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "lp" => { validate_finite_parameter("LP", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "nsubp" => { validate_parameter("NSUBP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "nsubp0" => { validate_finite_parameter("NSUBP0", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "nsubwp" => { validate_finite_parameter("NSUBWP", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "wl1" => { validate_finite_parameter("WL1", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "wl1p" => { validate_finite_parameter("WL1P", value)?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "wl2" => { validate_finite_parameter("WL2", value)?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "wl2p" => { validate_finite_parameter("WL2P", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "scp1" => { validate_finite_parameter("SCP1", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "scp2" => { validate_finite_parameter("SCP2", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "scp3" => { validate_finite_parameter("SCP3", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "sc1" => { validate_finite_parameter("SC1", value)?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "sc2" => { validate_finite_parameter("SC2", value)?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "sc3" => { validate_finite_parameter("SC3", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "scr1" => { validate_finite_parameter("SCR1", value)?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "scr2" => { validate_finite_parameter("SCR2", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "scr3" => { validate_finite_parameter("SCR3", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "pgd1" => { validate_finite_parameter("PGD1", value)?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "pgd2" => { validate_finite_parameter("PGD2", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "pgd4" => { validate_finite_parameter("PGD4", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "ndep" => { validate_finite_parameter("NDEP", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "ninv" => { validate_finite_parameter("NINV", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "ninvd" => { validate_finite_parameter("NINVD", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "muecb0" => { validate_finite_parameter("MUECB0", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "muecb1" => { validate_finite_parameter("MUECB1", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "mueph0" => { validate_finite_parameter("MUEPH0", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "muephw" => { validate_finite_parameter("MUEPHW", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "muepwp" => { validate_finite_parameter("MUEPWP", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "muephl" => { validate_finite_parameter("MUEPHL", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "mueplp" => { validate_finite_parameter("MUEPLP", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "muephs" => { validate_finite_parameter("MUEPHS", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "muepsp" => { validate_finite_parameter("MUEPSP", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "vtmp" => { validate_finite_parameter("VTMP", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "wvth0" => { validate_finite_parameter("WVTH0", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "muesr1" => { validate_parameter("MUESR1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "muesr0" => { validate_finite_parameter("MUESR0", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "muesrl" => { validate_finite_parameter("MUESRL", value)?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "muesrw" => { validate_finite_parameter("MUESRW", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "mueswp" => { validate_finite_parameter("MUESWP", value)?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "mueslp" => { validate_finite_parameter("MUESLP", value)?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "muetmp" => { validate_finite_parameter("MUETMP", value)?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "bb" => { validate_parameter("BB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "ddltmax" => { validate_finite_parameter("DDLTMAX", value)?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "ddltslp" => { validate_finite_parameter("DDLTSLP", value)?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "ddltict" => { validate_finite_parameter("DDLTICT", value)?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "sub1" => { validate_finite_parameter("SUB1", value)?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "sub2" => { validate_finite_parameter("SUB2", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "sub1l" => { validate_finite_parameter("SUB1L", value)?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "sub1lp" => { validate_finite_parameter("SUB1LP", value)?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "sub2l" => { validate_finite_parameter("SUB2L", value)?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "svds" => { validate_finite_parameter("SVDS", value)?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "slg" => { validate_finite_parameter("SLG", value)?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "svbs" => { validate_finite_parameter("SVBS", value)?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "svbsl" => { validate_finite_parameter("SVBSL", value)?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "svbslp" => { validate_finite_parameter("SVBSLP", value)?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "svgs" => { validate_finite_parameter("SVGS", value)?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "svgsl" => { validate_finite_parameter("SVGSL", value)?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "svgslp" => { validate_finite_parameter("SVGSLP", value)?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "svgsw" => { validate_finite_parameter("SVGSW", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "svgswp" => { validate_finite_parameter("SVGSWP", value)?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "slgl" => { validate_finite_parameter("SLGL", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "slglp" => { validate_finite_parameter("SLGLP", value)?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "vfbsub" => { validate_finite_parameter("VFBSUB", value)?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "vfbsubl" => { validate_finite_parameter("VFBSUBL", value)?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "vfbsublp" => { validate_finite_parameter("VFBSUBLP", value)?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "subdlt" => { validate_finite_parameter("SUBDLT", value)?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "hist1" => { validate_parameter("HIST1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "hist2" => { validate_finite_parameter("HIST2", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "qhe1" => { validate_finite_parameter("QHE1", value)?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "qhe2" => { validate_finite_parameter("QHE2", value)?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "evb1" => { validate_finite_parameter("EVB1", value)?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "evb2" => { validate_finite_parameter("EVB2", value)?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "evb3" => { validate_finite_parameter("EVB3", value)?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "fvbs" => { validate_finite_parameter("FVBS", value)?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "ibpc1" => { validate_finite_parameter("IBPC1", value)?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "ibpc2" => { validate_finite_parameter("IBPC2", value)?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "nsti" => { validate_finite_parameter("NSTI", value)?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "wsti" => { validate_finite_parameter("WSTI", value)?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "wstil" => { validate_finite_parameter("WSTIL", value)?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "wstilp" => { validate_finite_parameter("WSTILP", value)?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "wstiw" => { validate_finite_parameter("WSTIW", value)?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "wstiwp" => { validate_finite_parameter("WSTIWP", value)?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "scsti1" => { validate_finite_parameter("SCSTI1", value)?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "scsti2" => { validate_finite_parameter("SCSTI2", value)?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "vthsti" => { validate_finite_parameter("VTHSTI", value)?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "vdsti" => { validate_finite_parameter("VDSTI", value)?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "muesti1" => { validate_finite_parameter("MUESTI1", value)?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "muesti2" => { validate_parameter("MUESTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "muesti3" => { validate_finite_parameter("MUESTI3", value)?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "nsubpsti1" => { validate_finite_parameter("NSUBPSTI1", value)?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "nsubpsti2" => { validate_parameter("NSUBPSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "nsubpsti3" => { validate_finite_parameter("NSUBPSTI3", value)?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "nsubcsti1" => { validate_finite_parameter("NSUBCSTI1", value)?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "nsubcsti2" => { validate_parameter("NSUBCSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "nsubcsti3" => { validate_finite_parameter("NSUBCSTI3", value)?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "tpoly" => { validate_finite_parameter("TPOLY", value)?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "cgbo" => { validate_parameter("CGBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "cgdo" => { validate_parameter("CGDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "cgso" => { validate_parameter("CGSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "ovslp" => { validate_finite_parameter("OVSLP", value)?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "ovmag" => { validate_finite_parameter("OVMAG", value)?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "js0" => { validate_finite_parameter("JS0", value)?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "nj" => { validate_parameter("NJ", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "xti" => { validate_finite_parameter("XTI", value)?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "xti2" => { validate_finite_parameter("XTI2", value)?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "vdiffj" => { validate_finite_parameter("VDIFFJ", value)?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "divx" => { validate_finite_parameter("DIVX", value)?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "cj" => { validate_finite_parameter("CJ", value)?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "cjsw" => { validate_finite_parameter("CJSW", value)?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "cjswg" => { validate_finite_parameter("CJSWG", value)?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "mj" => { validate_parameter("MJ", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "mjsw" => { validate_parameter("MJSW", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "mjswg" => { validate_parameter("MJSWG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "pb" => { validate_parameter("PB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "pbsw" => { validate_parameter("PBSW", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "pbswg" => { validate_parameter("PBSWG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "lover" => { validate_finite_parameter("LOVER", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "clm1" => { validate_finite_parameter("CLM1", value)?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "clm2" => { validate_finite_parameter("CLM2", value)?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "clm3" => { validate_finite_parameter("CLM3", value)?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "clm5" => { validate_finite_parameter("CLM5", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "clm6" => { validate_finite_parameter("CLM6", value)?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "vover" => { validate_finite_parameter("VOVER", value)?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "voverp" => { validate_finite_parameter("VOVERP", value)?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "vovers" => { validate_finite_parameter("VOVERS", value)?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "voversp" => { validate_finite_parameter("VOVERSP", value)?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "wfc" => { validate_finite_parameter("WFC", value)?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "nsubcw" => { validate_finite_parameter("NSUBCW", value)?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "nsubcwp" => { validate_finite_parameter("NSUBCWP", value)?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "nsubcmax" => { validate_finite_parameter("NSUBCMAX", value)?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "nsubcl" => { validate_finite_parameter("NSUBCL", value)?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "nsubclp" => { validate_finite_parameter("NSUBCLP", value)?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "qme1" => { validate_finite_parameter("QME1", value)?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "qme2" => { validate_finite_parameter("QME2", value)?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "qme3" => { validate_finite_parameter("QME3", value)?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "gidl1" => { validate_finite_parameter("GIDL1", value)?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "gidl2" => { validate_finite_parameter("GIDL2", value)?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "gidl3" => { validate_finite_parameter("GIDL3", value)?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "gidl4" => { validate_finite_parameter("GIDL4", value)?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "gidl5" => { validate_finite_parameter("GIDL5", value)?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "gidlvb" => { validate_parameter("GIDLVB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "gleak1" => { validate_finite_parameter("GLEAK1", value)?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "gleak2" => { validate_finite_parameter("GLEAK2", value)?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "gleak3" => { validate_finite_parameter("GLEAK3", value)?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "gleak4" => { validate_finite_parameter("GLEAK4", value)?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "gleak5" => { validate_parameter("GLEAK5", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "gleak6" => { validate_finite_parameter("GLEAK6", value)?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "gleak7" => { validate_finite_parameter("GLEAK7", value)?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "glksd1" => { validate_finite_parameter("GLKSD1", value)?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "glksd2" => { validate_finite_parameter("GLKSD2", value)?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "glksd3" => { validate_finite_parameter("GLKSD3", value)?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "glkb1" => { validate_finite_parameter("GLKB1", value)?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "glkb2" => { validate_finite_parameter("GLKB2", value)?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "glkb3" => { validate_finite_parameter("GLKB3", value)?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "vzadd0" => { validate_parameter("VZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "pzadd0" => { validate_parameter("PZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "nftrp" => { validate_finite_parameter("NFTRP", value)?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "nfalp" => { validate_finite_parameter("NFALP", value)?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "falph" => { validate_finite_parameter("FALPH", value)?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((22.0, "22.0")), false, Some((32.0, "32.0")), false, &[])?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "dly1" => { validate_finite_parameter("DLY1", value)?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "dly2" => { validate_finite_parameter("DLY2", value)?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "dly3" => { validate_finite_parameter("DLY3", value)?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "tfox" => { validate_parameter("TFOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "tsoi" => { validate_parameter("TSOI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "xj" => { validate_parameter("XJ", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "nsubs" => { validate_parameter("NSUBS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "nsubb" => { validate_parameter("NSUBB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "rth0" => { validate_finite_parameter("RTH0", value)?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "cth0" => { validate_finite_parameter("CTH0", value)?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "ptl" => { validate_finite_parameter("PTL", value)?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "ptp" => { validate_finite_parameter("PTP", value)?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "pt2" => { validate_finite_parameter("PT2", value)?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "ptlp" => { validate_finite_parameter("PTLP", value)?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "gdl" => { validate_finite_parameter("GDL", value)?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "gdlp" => { validate_finite_parameter("GDLP", value)?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "gdld" => { validate_finite_parameter("GDLD", value)?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "pt4" => { validate_finite_parameter("PT4", value)?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "pt4p" => { validate_finite_parameter("PT4P", value)?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "vgsmin" => { validate_finite_parameter("VGSMIN", value)?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "mueph1" => { validate_finite_parameter("MUEPH1", value)?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "ldrift" => { validate_parameter("LDRIFT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "ldrifts" => { validate_parameter("LDRIFTS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "cors" => { validate_parameter("CORS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "cord" => { validate_parameter("CORD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "corbulk" => { validate_parameter("CORBULK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "corbnet" => { validate_parameter("CORBNET", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "novers" => { validate_finite_parameter("NOVERS", value)?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "rdrmue" => { validate_parameter("RDRMUE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "rdrmues" => { validate_parameter("RDRMUES", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "rdrvmax" => { validate_parameter("RDRVMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "rdrvmaxs" => { validate_parameter("RDRVMAXS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "rdrmuetmp" => { validate_finite_parameter("RDRMUETMP", value)?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "rdrvtmp" => { validate_finite_parameter("RDRVTMP", value)?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "rdrdjunc" => { validate_parameter("RDRDJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "rdrbb" => { validate_parameter("RDRBB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "rdrbbs" => { validate_parameter("RDRBBS", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "rdrbbtmp" => { validate_finite_parameter("RDRBBTMP", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "rdrvmaxw" => { validate_finite_parameter("RDRVMAXW", value)?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "rdrvmaxwp" => { validate_finite_parameter("RDRVMAXWP", value)?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "rdrvmaxl" => { validate_finite_parameter("RDRVMAXL", value)?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "rdrvmaxlp" => { validate_finite_parameter("RDRVMAXLP", value)?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "rdrmuel" => { validate_finite_parameter("RDRMUEL", value)?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "rdrmuelp" => { validate_finite_parameter("RDRMUELP", value)?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "copt" => { validate_parameter("COPT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "copspt" => { validate_parameter("COPSPT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "xjpt" => { validate_parameter("XJPT", value, Some((0.0, "0.0")), true, None, false, &[])?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "njunc" => { validate_parameter("NJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "mupt" => { validate_parameter("MUPT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "vfbpt" => { validate_finite_parameter("VFBPT", value)?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
            "pslimpt" => { validate_finite_parameter("PSLIMPT", value)?; self.params.p287 = value; self.mark_param_given(287); Ok(()) }
            "rbulk0" => { validate_parameter("RBULK0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); Ok(()) }
            "rbulkw" => { validate_parameter("RBULKW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); Ok(()) }
            "rbdb" => { validate_parameter("RBDB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); Ok(()) }
            "rbsb" => { validate_parameter("RBSB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p291 = value; self.mark_param_given(291); Ok(()) }
            "vfbbtp" => { validate_finite_parameter("VFBBTP", value)?; self.params.p292 = value; self.mark_param_given(292); Ok(()) }
            "cbtbn" => { validate_finite_parameter("CBTBN", value)?; self.params.p293 = value; self.mark_param_given(293); Ok(()) }
            "cbtbp" => { validate_finite_parameter("CBTBP", value)?; self.params.p294 = value; self.mark_param_given(294); Ok(()) }
            "xwdbt" => { validate_finite_parameter("XWDBT", value)?; self.params.p295 = value; self.mark_param_given(295); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimsoi_va'", name)),
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
    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {
        self.time = time;
        self.timestep = timestep;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_previous[index] = self.ddt_state_current[index];
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
        self.ddt_state_current[slot] = value;
        if self.timestep.abs() > Self::DDT_EPSILON {
            (value - previous) / self.timestep
        } else {
            self.ddt_state_previous[slot] = value;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative / self.timestep
        } else {
            0.0
        }
    }
}
