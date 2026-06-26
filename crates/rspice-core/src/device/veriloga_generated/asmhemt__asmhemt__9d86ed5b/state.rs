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
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 287]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 121]>,
    pub(crate) ddt_state_previous: Box<[f64; 121]>,
    pub(crate) ddt_state_initialized: Box<[bool; 121]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<612, 23, 57>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<612, 23, 57>>>,
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
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "tbar" => { validate_parameter("tbar", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "tepi" => { validate_parameter("tepi", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "l" => { validate_parameter("l", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "w" => { validate_parameter("w", value, Some((2e-8, "2e-8")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "mult_i" => { validate_parameter("mult_i", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "mult_q" => { validate_parameter("mult_q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "mult_fn" => { validate_parameter("mult_fn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "epsilon" => { validate_parameter("epsilon", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "voff" => { validate_parameter("voff", value, Some((-100.0, "-100.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "asub" => { validate_parameter("asub", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "ksub" => { validate_finite_parameter("ksub", value)?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "u0" => { validate_parameter("u0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "ua" => { validate_parameter("ua", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "ub" => { validate_parameter("ub", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "uc" => { validate_parameter("uc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "vsat" => { validate_parameter("vsat", value, Some((1000.0, "1000.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "delta" => { validate_parameter("delta", value, Some((2.0, "2.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "at" => { validate_finite_parameter("at", value)?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "ute" => { validate_parameter("ute", value, Some((-10.0, "-10.0")), false, Some((0.0, "0.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "lambda" => { validate_parameter("lambda", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "eta0" => { validate_parameter("eta0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "vdscale" => { validate_parameter("vdscale", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "kt1" => { validate_finite_parameter("kt1", value)?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "thesat" => { validate_parameter("thesat", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "nfactor" => { validate_parameter("nfactor", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "cdscd" => { validate_parameter("cdscd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "gamma0i" => { validate_parameter("gamma0i", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "gamma1i" => { validate_parameter("gamma1i", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "imin" => { validate_parameter("imin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "rdsmod" => { validate_parameter("rdsmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "vsataccs" => { validate_parameter("vsataccs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "ns0accs" => { validate_parameter("ns0accs", value, Some((100000.0, "100000.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "ns0accd" => { validate_parameter("ns0accd", value, Some((100000.0, "100000.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "k0accs" => { validate_parameter("k0accs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "k0accd" => { validate_parameter("k0accd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "u0accs" => { validate_parameter("u0accs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "u0accd" => { validate_parameter("u0accd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "mexpaccs" => { validate_parameter("mexpaccs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "mexpaccd" => { validate_parameter("mexpaccd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "ard" => { validate_parameter("ard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "ars" => { validate_parameter("ars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "lsg" => { validate_parameter("lsg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "ldg" => { validate_parameter("ldg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "rsc" => { validate_parameter("rsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "rdc" => { validate_parameter("rdc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "kns0" => { validate_parameter("kns0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "ats" => { validate_finite_parameter("ats", value)?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "utes" => { validate_finite_parameter("utes", value)?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "uted" => { validate_finite_parameter("uted", value)?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "krsc" => { validate_parameter("krsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "krdc" => { validate_parameter("krdc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "gatemod" => { validate_parameter("gatemod", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "njgs" => { validate_parameter("njgs", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "ags" => { validate_parameter("ags", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "agd" => { validate_parameter("agd", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "njgd" => { validate_parameter("njgd", value, Some((0.0, "0.0")), true, Some((50.0, "50.0")), true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "rnjgs" => { validate_parameter("rnjgs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "rnjgd" => { validate_parameter("rnjgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "igsdio" => { validate_parameter("igsdio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "igddio" => { validate_parameter("igddio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "rigsdio" => { validate_parameter("rigsdio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "rigddio" => { validate_parameter("rigddio", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "vbis" => { validate_parameter("vbis", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "vbid" => { validate_parameter("vbid", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "ebreaks" => { validate_parameter("ebreaks", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "ebreakd" => { validate_parameter("ebreakd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "ktgs" => { validate_finite_parameter("ktgs", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "ktgd" => { validate_finite_parameter("ktgd", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "rktgs" => { validate_finite_parameter("rktgs", value)?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "rktgd" => { validate_finite_parameter("rktgd", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "ktvbis" => { validate_finite_parameter("ktvbis", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "ktvbid" => { validate_finite_parameter("ktvbid", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "ktnjgs" => { validate_finite_parameter("ktnjgs", value)?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "ktnjgd" => { validate_finite_parameter("ktnjgd", value)?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "ktrnjgs" => { validate_finite_parameter("ktrnjgs", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "ktrnjgd" => { validate_finite_parameter("ktrnjgd", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "trapmod" => { validate_parameter("trapmod", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "remi" => { validate_parameter("remi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "cglag" => { validate_parameter("cglag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "remig" => { validate_parameter("remig", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "arcap" => { validate_finite_parameter("arcap", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "brcap" => { validate_parameter("brcap", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "arcapg" => { validate_finite_parameter("arcapg", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "brcapg" => { validate_parameter("brcapg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "vdlmax" => { validate_parameter("vdlmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "vglmax" => { validate_parameter("vglmax", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "dlvoff" => { validate_finite_parameter("dlvoff", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "glvoff" => { validate_finite_parameter("glvoff", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "glu0" => { validate_finite_parameter("glu0", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "glvsat" => { validate_finite_parameter("glvsat", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "dlns0s" => { validate_finite_parameter("dlns0s", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "dlns0d" => { validate_finite_parameter("dlns0d", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "cdlag" => { validate_parameter("cdlag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "rdlag" => { validate_parameter("rdlag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "idio" => { validate_parameter("idio", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "atrapvoff" => { validate_finite_parameter("atrapvoff", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "btrapvoff" => { validate_finite_parameter("btrapvoff", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "atrapeta0" => { validate_finite_parameter("atrapeta0", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "btrapeta0" => { validate_finite_parameter("btrapeta0", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "atraprs" => { validate_finite_parameter("atraprs", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "btraprs" => { validate_finite_parameter("btraprs", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "atraprd" => { validate_finite_parameter("atraprd", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "btraprd" => { validate_finite_parameter("btraprd", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "rtrap1" => { validate_parameter("rtrap1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "rtrap2" => { validate_parameter("rtrap2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "ctrap1" => { validate_parameter("ctrap1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "ctrap2" => { validate_parameter("ctrap2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "a1" => { validate_finite_parameter("a1", value)?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "vofftr" => { validate_finite_parameter("vofftr", value)?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "cdscdtr" => { validate_finite_parameter("cdscdtr", value)?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "eta0tr" => { validate_finite_parameter("eta0tr", value)?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "rontr1" => { validate_finite_parameter("rontr1", value)?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "rontr2" => { validate_finite_parameter("rontr2", value)?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "rontr3" => { validate_finite_parameter("rontr3", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "rtrap3" => { validate_parameter("rtrap3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "ctrap3" => { validate_parameter("ctrap3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "vatrap" => { validate_parameter("vatrap", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "sct" => { validate_parameter("sct", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "wd" => { validate_finite_parameter("wd", value)?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "vdlr1" => { validate_finite_parameter("vdlr1", value)?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "vdlr2" => { validate_finite_parameter("vdlr2", value)?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "talpha" => { validate_finite_parameter("talpha", value)?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "vtb" => { validate_parameter("vtb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "deltax" => { validate_parameter("deltax", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "alphax" => { validate_finite_parameter("alphax", value)?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "alphaxd" => { validate_finite_parameter("alphaxd", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "betax" => { validate_finite_parameter("betax", value)?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "gammax" => { validate_finite_parameter("gammax", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "etax" => { validate_finite_parameter("etax", value)?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "eno" => { validate_finite_parameter("eno", value)?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "cx" => { validate_finite_parameter("cx", value)?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "vxmax" => { validate_finite_parameter("vxmax", value)?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "ea" => { validate_finite_parameter("ea", value)?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "alphay" => { validate_finite_parameter("alphay", value)?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "alphayd" => { validate_finite_parameter("alphayd", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "betay" => { validate_finite_parameter("betay", value)?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "gammay" => { validate_finite_parameter("gammay", value)?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "etay" => { validate_finite_parameter("etay", value)?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "eno1" => { validate_finite_parameter("eno1", value)?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "cy" => { validate_finite_parameter("cy", value)?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "vymax" => { validate_finite_parameter("vymax", value)?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "ea1" => { validate_finite_parameter("ea1", value)?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "glns0s" => { validate_finite_parameter("glns0s", value)?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "glns0d" => { validate_finite_parameter("glns0d", value)?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "fastfpmod" => { validate_parameter("fastfpmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "fp1mod" => { validate_parameter("fp1mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "fp1smod" => { validate_parameter("fp1smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "fp2mod" => { validate_parameter("fp2mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "fp2smod" => { validate_parameter("fp2smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "fp3mod" => { validate_parameter("fp3mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "fp3smod" => { validate_parameter("fp3smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "fp4mod" => { validate_parameter("fp4mod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "fp4smod" => { validate_parameter("fp4smod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "iminfp1" => { validate_parameter("iminfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "vofffp1" => { validate_parameter("vofffp1", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "dfp1" => { validate_parameter("dfp1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "lfp1" => { validate_parameter("lfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "ktfp1" => { validate_finite_parameter("ktfp1", value)?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "u0fp1" => { validate_parameter("u0fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "vsatfp1" => { validate_parameter("vsatfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "nfactorfp1" => { validate_parameter("nfactorfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "cdscdfp1" => { validate_parameter("cdscdfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "eta0fp1" => { validate_parameter("eta0fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "vdscalefp1" => { validate_parameter("vdscalefp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "gamma0fp1" => { validate_parameter("gamma0fp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "gamma1fp1" => { validate_parameter("gamma1fp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "iminfp2" => { validate_parameter("iminfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "vofffp2" => { validate_parameter("vofffp2", value, Some((-100.0, "-100.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "dfp2" => { validate_parameter("dfp2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "lfp2" => { validate_parameter("lfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "ktfp2" => { validate_finite_parameter("ktfp2", value)?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "u0fp2" => { validate_parameter("u0fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "vsatfp2" => { validate_parameter("vsatfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "nfactorfp2" => { validate_parameter("nfactorfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "cdscdfp2" => { validate_parameter("cdscdfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "eta0fp2" => { validate_parameter("eta0fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "vdscalefp2" => { validate_parameter("vdscalefp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "gamma0fp2" => { validate_parameter("gamma0fp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "gamma1fp2" => { validate_parameter("gamma1fp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "iminfp3" => { validate_parameter("iminfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "vofffp3" => { validate_parameter("vofffp3", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "dfp3" => { validate_parameter("dfp3", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "lfp3" => { validate_parameter("lfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "ktfp3" => { validate_finite_parameter("ktfp3", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "u0fp3" => { validate_parameter("u0fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "vsatfp3" => { validate_parameter("vsatfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "nfactorfp3" => { validate_parameter("nfactorfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "cdscdfp3" => { validate_parameter("cdscdfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "eta0fp3" => { validate_parameter("eta0fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "vdscalefp3" => { validate_parameter("vdscalefp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "gamma0fp3" => { validate_parameter("gamma0fp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "gamma1fp3" => { validate_parameter("gamma1fp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "iminfp4" => { validate_parameter("iminfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "vofffp4" => { validate_parameter("vofffp4", value, Some((-500.0, "-500.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "dfp4" => { validate_parameter("dfp4", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "lfp4" => { validate_parameter("lfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "ktfp4" => { validate_finite_parameter("ktfp4", value)?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "u0fp4" => { validate_parameter("u0fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "vsatfp4" => { validate_parameter("vsatfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "nfactorfp4" => { validate_parameter("nfactorfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "cdscdfp4" => { validate_parameter("cdscdfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "eta0fp4" => { validate_parameter("eta0fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "vdscalefp4" => { validate_parameter("vdscalefp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "gamma0fp4" => { validate_parameter("gamma0fp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "gamma1fp4" => { validate_parameter("gamma1fp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "cgso" => { validate_parameter("cgso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "cgdo" => { validate_parameter("cgdo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "cdso" => { validate_parameter("cdso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "cgdl" => { validate_parameter("cgdl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "vdsatcv" => { validate_parameter("vdsatcv", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "cbdo" => { validate_parameter("cbdo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "cbso" => { validate_parameter("cbso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "cbgo" => { validate_parameter("cbgo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "cfg" => { validate_parameter("cfg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "cfd" => { validate_parameter("cfd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "cfgd" => { validate_parameter("cfgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "cfgdsm" => { validate_parameter("cfgdsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "cfgd0" => { validate_parameter("cfgd0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "cj0" => { validate_parameter("cj0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "vbi" => { validate_parameter("vbi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "ktvbi" => { validate_parameter("ktvbi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "ktcfg" => { validate_parameter("ktcfg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "ktcfgd" => { validate_parameter("ktcfgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "mz" => { validate_parameter("mz", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "aj" => { validate_parameter("aj", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "dj" => { validate_parameter("dj", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), true, &[])?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "adosi" => { validate_parameter("adosi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "bdosi" => { validate_parameter("bdosi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "qm0i" => { validate_parameter("qm0i", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "adosfp1" => { validate_parameter("adosfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "bdosfp1" => { validate_parameter("bdosfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "qm0fp1" => { validate_parameter("qm0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "adosfp2" => { validate_parameter("adosfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "bdosfp2" => { validate_parameter("bdosfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "qm0fp2" => { validate_parameter("qm0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "adosfp3" => { validate_parameter("adosfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "bdosfp3" => { validate_parameter("bdosfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "qm0fp3" => { validate_parameter("qm0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "adosfp4" => { validate_parameter("adosfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "bdosfp4" => { validate_parameter("bdosfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "qm0fp4" => { validate_parameter("qm0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "cfp1scale" => { validate_parameter("cfp1scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "cfp2scale" => { validate_parameter("cfp2scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "cfp3scale" => { validate_parameter("cfp3scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "cfp4scale" => { validate_parameter("cfp4scale", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "csubscalei" => { validate_parameter("csubscalei", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "csubscale1" => { validate_parameter("csubscale1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "csubscale2" => { validate_parameter("csubscale2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "csubscale3" => { validate_parameter("csubscale3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "csubscale4" => { validate_parameter("csubscale4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "rgatemod" => { validate_parameter("rgatemod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "xgw" => { validate_parameter("xgw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "ngcon" => { validate_parameter("ngcon", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "rshg" => { validate_parameter("rshg", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "fnmod" => { validate_parameter("fnmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "tnmod" => { validate_parameter("tnmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "noia" => { validate_finite_parameter("noia", value)?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "noib" => { validate_parameter("noib", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "noic" => { validate_parameter("noic", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "ef" => { validate_parameter("ef", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "tnsc" => { validate_parameter("tnsc", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "gdsmin" => { validate_parameter("gdsmin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "tgdsmin" => { validate_parameter("tgdsmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "bvdsl" => { validate_parameter("bvdsl", value, Some((10.0, "10.0")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "asl" => { validate_parameter("asl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "nsl" => { validate_parameter("nsl", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "kasl" => { validate_finite_parameter("kasl", value)?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "knsl" => { validate_finite_parameter("knsl", value)?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "kbvdsl" => { validate_finite_parameter("kbvdsl", value)?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "nsb" => { validate_parameter("nsb", value, Some((0.0, "0.0")), true, Some((5000.0, "5000.0")), true, &[])?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "ndb" => { validate_parameter("ndb", value, Some((0.0, "0.0")), true, Some((5000.0, "5000.0")), true, &[])?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "isbl" => { validate_parameter("isbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "idbl" => { validate_parameter("idbl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "vbisb" => { validate_parameter("vbisb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "vbidb" => { validate_parameter("vbidb", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "ktisb" => { validate_finite_parameter("ktisb", value)?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "ktidb" => { validate_finite_parameter("ktidb", value)?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "ktnsb" => { validate_finite_parameter("ktnsb", value)?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "ktndb" => { validate_finite_parameter("ktndb", value)?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "ktvbisb" => { validate_finite_parameter("ktvbisb", value)?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "ktvbidb" => { validate_finite_parameter("ktvbidb", value)?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
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
