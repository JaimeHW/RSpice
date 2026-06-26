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
    pub p296: f64,
    pub p297: f64,
    pub p298: f64,
    pub p299: f64,
    pub p300: f64,
    pub p301: f64,
    pub p302: f64,
    pub p303: f64,
    pub p304: f64,
    pub p305: f64,
    pub p306: f64,
    pub p307: f64,
    pub p308: f64,
    pub p309: f64,
    pub p310: f64,
    pub p311: f64,
    pub p312: f64,
    pub p313: f64,
    pub p314: f64,
    pub p315: f64,
    pub p316: f64,
    pub p317: f64,
    pub p318: f64,
    pub p319: f64,
    pub p320: f64,
    pub p321: f64,
    pub p322: f64,
    pub p323: f64,
    pub p324: f64,
    pub p325: f64,
    pub p326: f64,
    pub p327: f64,
    pub p328: f64,
    pub p329: f64,
    pub p330: f64,
    pub p331: f64,
}

impl Copy for Parameters {}

impl Clone for Parameters {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl Default for Parameters {
    fn default() -> Self {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut params: Self = unsafe { std::mem::zeroed::<Self>() };
        params.p0 = 5e-6;
        params.p1 = 5e-6;
        params.p2 = 1.0;
        params.p3 = 0.0;
        params.p4 = 0.0;
        params.p5 = 1.0;
        params.p6 = 0.0;
        params.p7 = 0.0;
        params.p8 = 0.0;
        params.p9 = 27.0;
        params.p10 = 0.0;
        params.p11 = 0.0;
        params.p12 = 0.0;
        params.p13 = 0.0;
        params.p14 = 1.0;
        params.p15 = 0.0;
        params.p16 = 0.0;
        params.p17 = 0.0;
        params.p18 = 0.0;
        params.p19 = 0.0;
        params.p20 = 0.0;
        params.p21 = 0.0;
        params.p22 = 0.0;
        params.p23 = 0.0;
        params.p24 = 0.0;
        params.p25 = 0.0;
        params.p26 = 0.0;
        params.p27 = 0.0;
        params.p28 = 0.0;
        params.p29 = 0.0;
        params.p30 = 1.0;
        params.p31 = 0.0;
        params.p32 = 0.0;
        params.p33 = 1.0;
        params.p34 = 7000000.0;
        params.p35 = 9.025e-5;
        params.p36 = 1e-7;
        params.p37 = 1.1785;
        params.p38 = 0.0;
        params.p39 = 0.0;
        params.p40 = 1e19;
        params.p41 = 0.0;
        params.p42 = params.p41;
        validate_finite_parameter("XWDC", params.p42).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p43 = 1e-6;
        params.p44 = 1e-6;
        params.p45 = 0.0;
        params.p46 = 0.0;
        params.p47 = 2.0;
        params.p48 = 0.0;
        params.p49 = -1.0;
        params.p50 = 0.0;
        params.p51 = 1.0;
        params.p52 = 0.0;
        params.p53 = 1.0;
        params.p54 = 0.0;
        params.p55 = 1.1;
        params.p56 = 1e-8;
        params.p57 = 1e-8;
        params.p58 = 0.0;
        params.p59 = 1e17;
        params.p60 = 0.0;
        params.p61 = 1.0;
        params.p62 = 0.0;
        params.p63 = 1.0;
        params.p64 = 0.0;
        params.p65 = 1.0;
        params.p66 = 0.0;
        params.p67 = 0.0;
        params.p68 = 0.0;
        params.p69 = 0.0;
        params.p70 = 0.0;
        params.p71 = 0.0;
        params.p72 = 0.0;
        params.p73 = 0.0;
        params.p74 = 0.23;
        params.p75 = 0.0;
        params.p76 = 1.0;
        params.p77 = 0.0;
        params.p78 = 1.0;
        params.p79 = 0.0;
        params.p80 = 0.0;
        params.p81 = 0.5;
        params.p82 = 0.0;
        params.p83 = 0.0;
        params.p84 = 0.0;
        params.p85 = 1.0;
        params.p86 = 300.0;
        params.p87 = 30.0;
        params.p88 = 0.0;
        params.p89 = 0.0;
        params.p90 = 0.0;
        params.p91 = 1.0;
        params.p92 = 0.0;
        params.p93 = 1.0;
        params.p94 = 0.3;
        params.p95 = 0.0;
        params.p96 = 1.0;
        params.p97 = 0.0;
        params.p98 = 1.0;
        params.p99 = 0.0;
        params.p100 = 1.0;
        params.p101 = 0.0;
        params.p102 = 0.0;
        params.p103 = 1.0;
        params.p104 = 0.0;
        params.p105 = 2000000000000000.0;
        params.p106 = 2.0;
        params.p107 = 0.0;
        params.p108 = 0.0;
        params.p109 = 1.0;
        params.p110 = 1.0;
        params.p111 = 1.5;
        params.p112 = 0.0;
        params.p113 = 1.0;
        params.p114 = if (params.p33 > 0.0) { 2.0 } else { 1.0 };
        validate_parameter("BB", params.p114, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p115 = 10.0;
        params.p116 = 50.0;
        params.p117 = 0.0;
        params.p118 = 10.0;
        params.p119 = 20.0;
        params.p120 = 0.0025;
        params.p121 = 1.0;
        params.p122 = 2e-6;
        params.p123 = 0.8;
        params.p124 = 3e-8;
        params.p125 = 0.5;
        params.p126 = 0.0;
        params.p127 = 1.0;
        params.p128 = 0.8;
        params.p129 = 0.0;
        params.p130 = 1.0;
        params.p131 = 0.0;
        params.p132 = 1.0;
        params.p133 = -1.0;
        params.p134 = 0.0;
        params.p135 = 1.0;
        params.p136 = 0.002;
        params.p137 = 1e-8;
        params.p138 = 1e-20;
        params.p139 = 1.5;
        params.p140 = 0.55;
        params.p141 = 0.0;
        params.p142 = 0.0;
        params.p143 = 0.0;
        params.p144 = 0.0;
        params.p145 = 0.0;
        params.p146 = 0.0;
        params.p147 = 5e17;
        params.p148 = 0.0;
        params.p149 = 1.0;
        params.p150 = 0.0;
        params.p151 = 1.0;
        params.p152 = 0.0;
        params.p153 = 0.0;
        params.p154 = 0.0;
        params.p155 = 1.0;
        params.p156 = 0.0;
        params.p157 = 1.0;
        params.p158 = 0.0;
        params.p159 = 0.0;
        params.p160 = 0.0;
        params.p161 = 0.0;
        params.p162 = 0.0;
        params.p163 = 0.0;
        params.p164 = 1.0;
        params.p165 = 0.0;
        params.p166 = 0.0;
        params.p167 = 1.0;
        params.p168 = 0.0;
        params.p169 = 0.0;
        params.p170 = 1.0;
        params.p171 = 0.0;
        params.p172 = 0.0;
        params.p173 = 0.0;
        params.p174 = 0.0;
        params.p175 = 3e-8;
        params.p176 = 0.7;
        params.p177 = 2.0;
        params.p178 = 1.0;
        params.p179 = 1.0;
        params.p180 = 0.0;
        params.p181 = 0.01;
        params.p182 = 0.1;
        params.p183 = 0.0;
        params.p184 = 1.0;
        params.p185 = 0.0;
        params.p186 = 1.0;
        params.p187 = 0.0;
        params.p188 = 1.0;
        params.p189 = 0.0;
        params.p190 = 0.0;
        params.p191 = 1.0;
        params.p192 = 5e18;
        params.p193 = 0.0;
        params.p194 = 0.0;
        params.p195 = 0.0;
        params.p196 = 5e-6;
        params.p197 = 1000000.0;
        params.p198 = 0.3;
        params.p199 = 0.0;
        params.p200 = 0.2;
        params.p201 = 1e-6;
        params.p202 = 0.0;
        params.p203 = 10000.0;
        params.p204 = 20000000.0;
        params.p205 = 0.3;
        params.p206 = 0.0;
        params.p207 = 7500.0;
        params.p208 = 0.25;
        params.p209 = 1e-6;
        params.p210 = 1e-15;
        params.p211 = 5000000.0;
        params.p212 = -5000000.0;
        params.p213 = 5e-16;
        params.p214 = 1.0;
        params.p215 = 0.0;
        params.p216 = 0.01;
        params.p217 = 0.005;
        params.p218 = 10000000000.0;
        params.p219 = 1e-16;
        params.p220 = 0.0;
        params.p221 = 1.0;
        params.p222 = 27.0;
        params.p223 = 1e-10;
        params.p224 = 0.7;
        params.p225 = 8e-7;
        params.p226 = 3.5e-9;
        params.p227 = 1e-8;
        params.p228 = params.p226;
        validate_parameter("TFOXGIDL", params.p228, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p229 = 1e-8;
        params.p230 = 1e17;
        params.p231 = 1e18;
        params.p232 = 0.0;
        params.p233 = 1.0;
        params.p234 = 0.0;
        params.p235 = 1.0;
        params.p236 = 100000000000000.0;
        params.p237 = 0.1;
        params.p238 = 1e-7;
        params.p239 = 0.0;
        params.p240 = 3.5;
        params.p241 = 0.0;
        params.p242 = 1.0;
        params.p243 = 0.0;
        params.p244 = 1.0;
        params.p245 = 100.0;
        params.p246 = 0.0;
        params.p247 = 0.0;
        params.p248 = 0.0;
        params.p249 = 25000.0;
        params.p250 = 0.0;
        params.p251 = 2e-8;
        params.p252 = 1e-8;
        params.p253 = 0.0;
        params.p254 = 3.0;
        params.p255 = 3.5;
        params.p256 = 1.0;
        params.p257 = 0.5;
        params.p258 = 0.0;
        params.p259 = 0.0;
        params.p260 = 1.0;
        params.p261 = 1.0;
        params.p262 = 1.0;
        params.p263 = 1.0;
        params.p264 = 1e-11;
        params.p265 = 1.5e-11;
        params.p266 = 5e-16;
        params.p267 = 1.0;
        params.p268 = 0.0;
        params.p269 = 1.0;
        params.p270 = 1.0;
        params.p271 = 1.0;
        params.p272 = 1e-11;
        params.p273 = 1.5e-11;
        params.p274 = 1.0;
        params.p275 = params.p94;
        validate_finite_parameter("MUEPH0B", params.p275).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p276 = params.p249;
        validate_finite_parameter("MUEPH1B", params.p276).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p277 = params.p95;
        validate_finite_parameter("MUEPHWB", params.p277).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p278 = params.p96;
        validate_finite_parameter("MUEPWPB", params.p278).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p279 = params.p99;
        validate_finite_parameter("MUEPHSB", params.p279).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p280 = params.p100;
        validate_finite_parameter("MUEPSPB", params.p280).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p281 = params.p97;
        validate_finite_parameter("MUEPHLB", params.p281).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p282 = params.p98;
        validate_finite_parameter("MUEPLPB", params.p282).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p283 = params.p106;
        validate_finite_parameter("MUESR0B", params.p283).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p284 = params.p105;
        validate_parameter("MUESR1B", params.p284, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p285 = params.p107;
        validate_finite_parameter("MUESRLB", params.p285).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p286 = params.p110;
        validate_finite_parameter("MUESLPB", params.p286).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p287 = params.p108;
        validate_finite_parameter("MUESRWB", params.p287).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p288 = params.p109;
        validate_finite_parameter("MUESWPB", params.p288).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p289 = params.p86;
        validate_finite_parameter("MUECB0B", params.p289).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p290 = params.p87;
        validate_finite_parameter("MUECB1B", params.p290).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p291 = params.p88;
        validate_finite_parameter("MUECB0LPB", params.p291).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p292 = params.p89;
        validate_finite_parameter("MUECB1LPB", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p293 = params.p90;
        validate_finite_parameter("MUECB0L2B", params.p293).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p294 = params.p91;
        validate_finite_parameter("MUECB0L2PB", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p295 = params.p92;
        validate_finite_parameter("MUECB1L2B", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p296 = params.p93;
        validate_finite_parameter("MUECB1L2PB", params.p296).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p297 = 0.0;
        params.p298 = 0.0;
        params.p299 = 0.0;
        params.p300 = 0.0;
        params.p301 = 1.0;
        params.p302 = params.p299;
        validate_finite_parameter("MUEQBB", params.p302).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p303 = 0.0;
        params.p304 = 0.0;
        params.p305 = 0.0;
        params.p306 = 0.0;
        params.p307 = 0.0;
        params.p308 = 1.0;
        params.p309 = 1.0;
        params.p310 = 1e-6;
        params.p311 = 1e-6;
        params.p312 = 0.0;
        params.p313 = 0.0;
        params.p314 = 0.0;
        params.p315 = 1e19;
        params.p316 = 1000.0;
        params.p317 = 1000.0;
        params.p318 = 30000000.0;
        params.p319 = 30000000.0;
        params.p320 = 0.0;
        params.p321 = 0.0;
        params.p322 = 1e-6;
        params.p323 = 1.0;
        params.p324 = 1.0;
        params.p325 = 0.0;
        params.p326 = 0.0;
        params.p327 = 1.0;
        params.p328 = 0.0;
        params.p329 = 1.0;
        params.p330 = 0.0;
        params.p331 = 1.0;
        params
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
pub struct Instance {
    pub nodes: [usize; 13],
    pub branches: [usize; 8],
    pub params: Parameters,
    pub(crate) param_given: [bool; 332],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 9],
    pub(crate) ddt_state_previous: [f64; 9],
    pub(crate) ddt_state_initialized: [bool; 9],
    pub(crate) idt_state_current: [f64; 0],
    pub(crate) idt_state_previous: [f64; 0],
    pub(crate) idt_state_initialized: [bool; 0],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<1096, 13, 8>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<1096, 13, 8>>>,
}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes,
            branches: self.branches,
            params: self.params,
            param_given: self.param_given,
            multiplicity: self.multiplicity,
            ddt_state_current: self.ddt_state_current,
            ddt_state_previous: self.ddt_state_previous,
            ddt_state_initialized: self.ddt_state_initialized,
            idt_state_current: self.idt_state_current,
            idt_state_previous: self.idt_state_previous,
            idt_state_initialized: self.idt_state_initialized,
            time: self.time,
            timestep: self.timestep,
            scratch: None,
            reactive_scratch: None,
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
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::default(),
            param_given: [false; Self::PARAMETER_COUNT],
            multiplicity: 1.0,
            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],
            idt_state_current: [0.0; Self::IDT_STATE_COUNT],
            idt_state_previous: [0.0; Self::IDT_STATE_COUNT],
            idt_state_initialized: [false; Self::IDT_STATE_COUNT],
            time: 0.0,
            timestep: 0.0,
            scratch: Some(Box::new(GenericScratch::new())),
            reactive_scratch: Some(Box::new(GenericReactiveScratch::new())),
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
            "ngcon" => { validate_parameter("NGCON", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "temp" => { validate_finite_parameter("TEMP", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "sca" => { validate_finite_parameter("SCA", value)?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "scb" => { validate_finite_parameter("SCB", value)?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "scc" => { validate_finite_parameter("SCC", value)?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "coadov" => { validate_parameter("COADOV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "coisub" => { validate_parameter("COISUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "cofbe" => { validate_parameter("COFBE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "coiigs" => { validate_parameter("COIIGS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "cogidl" => { validate_parameter("COGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "coovlp" => { validate_parameter("COOVLP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "coign" => { validate_parameter("COIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "coflick" => { validate_parameter("COFLICK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "coisti" => { validate_parameter("COISTI", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "cothrml" => { validate_parameter("COTHRML", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "conqs" => { validate_parameter("CONQS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "corg" => { validate_parameter("CORG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "coievb" => { validate_parameter("COIEVB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "cohist" => { validate_parameter("COHIST", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "coselfheat" => { validate_parameter("COSELFHEAT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "covbsbiz" => { validate_parameter("COVBSBIZ", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "coqovsm" => { validate_parameter("COQOVSM", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "info" => { validate_finite_parameter("INFO", value)?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "conewmub" => { validate_parameter("CONEWMUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "bgtmp1" => { validate_finite_parameter("BGTMP1", value)?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "bgtmp2" => { validate_finite_parameter("BGTMP2", value)?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "eg0" => { validate_finite_parameter("EG0", value)?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "xld" => { validate_finite_parameter("XLD", value)?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "vfbover" => { validate_finite_parameter("VFBOVER", value)?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "nover" => { validate_finite_parameter("NOVER", value)?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "xwd" => { validate_finite_parameter("XWD", value)?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "xwdc" => { validate_finite_parameter("XWDC", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "xqy" => { validate_parameter("XQY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "xqy1" => { validate_finite_parameter("XQY1", value)?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "xqy2" => { validate_finite_parameter("XQY2", value)?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "vfbc" => { validate_finite_parameter("VFBC", value)?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "vfbcl1" => { validate_finite_parameter("VFBCL1", value)?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "vfbcl1p" => { validate_finite_parameter("VFBCL1P", value)?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "vfbcl2" => { validate_finite_parameter("VFBCL2", value)?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "vfbcl2p" => { validate_finite_parameter("VFBCL2P", value)?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "vfbhamp" => { validate_finite_parameter("VFBHAMP", value)?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "vbi" => { validate_finite_parameter("VBI", value)?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "parl1" => { validate_finite_parameter("PARL1", value)?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "parl2" => { validate_finite_parameter("PARL2", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "lp" => { validate_finite_parameter("LP", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "nsubp" => { validate_parameter("NSUBP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "nsubp0" => { validate_finite_parameter("NSUBP0", value)?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "nsubwp" => { validate_finite_parameter("NSUBWP", value)?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "wl1" => { validate_finite_parameter("WL1", value)?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "wl1p" => { validate_finite_parameter("WL1P", value)?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "wl2" => { validate_finite_parameter("WL2", value)?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "wl2p" => { validate_finite_parameter("WL2P", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "scp1" => { validate_finite_parameter("SCP1", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "scp2" => { validate_finite_parameter("SCP2", value)?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "scp3" => { validate_finite_parameter("SCP3", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "sc1" => { validate_finite_parameter("SC1", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "sc2" => { validate_finite_parameter("SC2", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "sc3" => { validate_finite_parameter("SC3", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "scr1" => { validate_finite_parameter("SCR1", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "scr2" => { validate_finite_parameter("SCR2", value)?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "scr3" => { validate_finite_parameter("SCR3", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "pgd1" => { validate_finite_parameter("PGD1", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "pgd2" => { validate_finite_parameter("PGD2", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "pgd4" => { validate_finite_parameter("PGD4", value)?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "ndep" => { validate_finite_parameter("NDEP", value)?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "ndepl" => { validate_finite_parameter("NDEPL", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "ndeplp" => { validate_finite_parameter("NDEPLP", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "ninv" => { validate_finite_parameter("NINV", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "ninvl" => { validate_finite_parameter("NINVL", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "ninvlp" => { validate_finite_parameter("NINVLP", value)?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "ninvd" => { validate_finite_parameter("NINVD", value)?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "ninvdp" => { validate_finite_parameter("NINVDP", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "muecb0" => { validate_finite_parameter("MUECB0", value)?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "muecb1" => { validate_finite_parameter("MUECB1", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "muecb0lp" => { validate_finite_parameter("MUECB0LP", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "muecb1lp" => { validate_finite_parameter("MUECB1LP", value)?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "muecb0l2" => { validate_finite_parameter("MUECB0L2", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "muecb0l2p" => { validate_finite_parameter("MUECB0L2P", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "muecb1l2" => { validate_finite_parameter("MUECB1L2", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "muecb1l2p" => { validate_finite_parameter("MUECB1L2P", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "mueph0" => { validate_finite_parameter("MUEPH0", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "muephw" => { validate_finite_parameter("MUEPHW", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "muepwp" => { validate_finite_parameter("MUEPWP", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "muephl" => { validate_finite_parameter("MUEPHL", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "mueplp" => { validate_finite_parameter("MUEPLP", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "muephs" => { validate_finite_parameter("MUEPHS", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "muepsp" => { validate_finite_parameter("MUEPSP", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "vtmp" => { validate_finite_parameter("VTMP", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "vtmpl" => { validate_finite_parameter("VTMPL", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "vtmplp" => { validate_finite_parameter("VTMPLP", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "wvth0" => { validate_finite_parameter("WVTH0", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "muesr1" => { validate_parameter("MUESR1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "muesr0" => { validate_finite_parameter("MUESR0", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "muesrl" => { validate_finite_parameter("MUESRL", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "muesrw" => { validate_finite_parameter("MUESRW", value)?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "mueswp" => { validate_finite_parameter("MUESWP", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "mueslp" => { validate_finite_parameter("MUESLP", value)?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "muetmp" => { validate_finite_parameter("MUETMP", value)?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "muetmpl" => { validate_finite_parameter("MUETMPL", value)?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "muetmplp" => { validate_finite_parameter("MUETMPLP", value)?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "bb" => { validate_parameter("BB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "ddltmax" => { validate_parameter("DDLTMAX", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "ddltslp" => { validate_parameter("DDLTSLP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "ddltict" => { validate_finite_parameter("DDLTICT", value)?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "sub1" => { validate_finite_parameter("SUB1", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "sub2" => { validate_finite_parameter("SUB2", value)?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "sub1l" => { validate_finite_parameter("SUB1L", value)?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "sub1lp" => { validate_finite_parameter("SUB1LP", value)?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "sub2l" => { validate_finite_parameter("SUB2L", value)?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "svds" => { validate_finite_parameter("SVDS", value)?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "slg" => { validate_parameter("SLG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "svbs" => { validate_finite_parameter("SVBS", value)?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "svbsl" => { validate_finite_parameter("SVBSL", value)?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "svbslp" => { validate_finite_parameter("SVBSLP", value)?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "svgs" => { validate_finite_parameter("SVGS", value)?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "svgsl" => { validate_finite_parameter("SVGSL", value)?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "svgslp" => { validate_finite_parameter("SVGSLP", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "svgsw" => { validate_finite_parameter("SVGSW", value)?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "svgswp" => { validate_finite_parameter("SVGSWP", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "vfbsub" => { validate_finite_parameter("VFBSUB", value)?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "vfbsubl" => { validate_finite_parameter("VFBSUBL", value)?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "vfbsublp" => { validate_finite_parameter("VFBSUBLP", value)?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "subdlt" => { validate_finite_parameter("SUBDLT", value)?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "hist1" => { validate_finite_parameter("HIST1", value)?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "hist2" => { validate_finite_parameter("HIST2", value)?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "qhe1" => { validate_finite_parameter("QHE1", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "qhe2" => { validate_finite_parameter("QHE2", value)?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "evb1" => { validate_finite_parameter("EVB1", value)?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "evb2" => { validate_finite_parameter("EVB2", value)?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "evb3" => { validate_finite_parameter("EVB3", value)?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "fvbs" => { validate_finite_parameter("FVBS", value)?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "ibpc1" => { validate_finite_parameter("IBPC1", value)?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "ibpc2" => { validate_finite_parameter("IBPC2", value)?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "nsti" => { validate_parameter("NSTI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "nstil" => { validate_finite_parameter("NSTIL", value)?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "nstilp" => { validate_finite_parameter("NSTILP", value)?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "nstiw" => { validate_finite_parameter("NSTIW", value)?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "nstiwp" => { validate_finite_parameter("NSTIWP", value)?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "wsti" => { validate_finite_parameter("WSTI", value)?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "ratwsti" => { validate_finite_parameter("RATWSTI", value)?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "wstil" => { validate_finite_parameter("WSTIL", value)?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "wstilp" => { validate_finite_parameter("WSTILP", value)?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "wstiw" => { validate_finite_parameter("WSTIW", value)?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "wstiwp" => { validate_finite_parameter("WSTIWP", value)?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "scsti1" => { validate_finite_parameter("SCSTI1", value)?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "scsti2" => { validate_finite_parameter("SCSTI2", value)?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "vthsti" => { validate_finite_parameter("VTHSTI", value)?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "vdsti" => { validate_finite_parameter("VDSTI", value)?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "muesti1" => { validate_parameter("MUESTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "muesti2" => { validate_parameter("MUESTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "muesti3" => { validate_finite_parameter("MUESTI3", value)?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "nsubpsti1" => { validate_parameter("NSUBPSTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "nsubpsti2" => { validate_parameter("NSUBPSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "nsubpsti3" => { validate_finite_parameter("NSUBPSTI3", value)?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "nsubssti1" => { validate_parameter("NSUBSSTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "nsubssti2" => { validate_parameter("NSUBSSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "nsubssti3" => { validate_finite_parameter("NSUBSSTI3", value)?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "tpoly" => { validate_finite_parameter("TPOLY", value)?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "cgbo" => { validate_parameter("CGBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "cgdo" => { validate_parameter("CGDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "cgso" => { validate_parameter("CGSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "lover" => { validate_finite_parameter("LOVER", value)?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "clm1" => { validate_finite_parameter("CLM1", value)?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "clm2" => { validate_finite_parameter("CLM2", value)?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "clm3" => { validate_finite_parameter("CLM3", value)?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "clm5" => { validate_finite_parameter("CLM5", value)?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "clm6" => { validate_finite_parameter("CLM6", value)?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "vover" => { validate_finite_parameter("VOVER", value)?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "voverp" => { validate_finite_parameter("VOVERP", value)?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "vovers" => { validate_finite_parameter("VOVERS", value)?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "voversp" => { validate_finite_parameter("VOVERSP", value)?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "voverl" => { validate_finite_parameter("VOVERL", value)?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "voverlp" => { validate_finite_parameter("VOVERLP", value)?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "voverw" => { validate_finite_parameter("VOVERW", value)?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "voverwp" => { validate_finite_parameter("VOVERWP", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "wfc" => { validate_finite_parameter("WFC", value)?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "nsubsw" => { validate_finite_parameter("NSUBSW", value)?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "nsubcw" => { validate_finite_parameter("NSUBSW", value)?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "nsubswp" => { validate_finite_parameter("NSUBSWP", value)?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "nsubcwp" => { validate_finite_parameter("NSUBSWP", value)?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "nsubsmax" => { validate_finite_parameter("NSUBSMAX", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "nsubcmax" => { validate_finite_parameter("NSUBSMAX", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "qme1" => { validate_finite_parameter("QME1", value)?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "qme2" => { validate_finite_parameter("QME2", value)?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "qme3" => { validate_finite_parameter("QME3", value)?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "gidl1" => { validate_finite_parameter("GIDL1", value)?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "gidl2" => { validate_finite_parameter("GIDL2", value)?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "gidl3" => { validate_finite_parameter("GIDL3", value)?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "gidl4" => { validate_finite_parameter("GIDL4", value)?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "gidl5" => { validate_finite_parameter("GIDL5", value)?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "gidlbpl1" => { validate_finite_parameter("GIDLBPL1", value)?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "gidlbplt" => { validate_finite_parameter("GIDLBPLT", value)?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "gleak1" => { validate_finite_parameter("GLEAK1", value)?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "gleak2" => { validate_finite_parameter("GLEAK2", value)?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "gleak3" => { validate_finite_parameter("GLEAK3", value)?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "gleak4" => { validate_finite_parameter("GLEAK4", value)?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "gleak5" => { validate_parameter("GLEAK5", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "gleak6" => { validate_finite_parameter("GLEAK6", value)?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "gleak7" => { validate_finite_parameter("GLEAK7", value)?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "glksd1" => { validate_finite_parameter("GLKSD1", value)?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "glksd2" => { validate_finite_parameter("GLKSD2", value)?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "glksd3" => { validate_finite_parameter("GLKSD3", value)?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "glkb1" => { validate_finite_parameter("GLKB1", value)?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "glkb2" => { validate_finite_parameter("GLKB2", value)?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "glkb3" => { validate_finite_parameter("GLKB3", value)?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "vzadd0" => { validate_parameter("VZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "pzadd0" => { validate_parameter("PZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "nftrp" => { validate_finite_parameter("NFTRP", value)?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "nfalp" => { validate_finite_parameter("NFALP", value)?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "falph" => { validate_finite_parameter("FALPH", value)?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((22.0, "22.0")), false, Some((32.0, "32.0")), false, &[])?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "dly1" => { validate_finite_parameter("DLY1", value)?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "dly2" => { validate_finite_parameter("DLY2", value)?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "dly3" => { validate_finite_parameter("DLY3", value)?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "tfox" => { validate_parameter("TFOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "tsoi" => { validate_parameter("TSOI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "tfoxgidl" => { validate_parameter("TFOXGIDL", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "nsubs" => { validate_parameter("NSUBS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "nsubb" => { validate_parameter("NSUBB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "nsubbl" => { validate_finite_parameter("NSUBBL", value)?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "nsubblp" => { validate_finite_parameter("NSUBBLP", value)?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "nsubbw" => { validate_finite_parameter("NSUBBW", value)?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "nsubbwp" => { validate_finite_parameter("NSUBBWP", value)?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "nsubbmin" => { validate_finite_parameter("NSUBBMIN", value)?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "rth0" => { validate_finite_parameter("RTH0", value)?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "cth0" => { validate_finite_parameter("CTH0", value)?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "ptl" => { validate_finite_parameter("PTL", value)?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "ptp" => { validate_finite_parameter("PTP", value)?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "pt2" => { validate_finite_parameter("PT2", value)?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "ptlp" => { validate_finite_parameter("PTLP", value)?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "pt4" => { validate_finite_parameter("PT4", value)?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "pt4p" => { validate_finite_parameter("PT4P", value)?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "ptdlt" => { validate_finite_parameter("PTDLT", value)?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "gdl" => { validate_finite_parameter("GDL", value)?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "gdlp" => { validate_finite_parameter("GDLP", value)?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "gdld" => { validate_finite_parameter("GDLD", value)?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "mueph1" => { validate_finite_parameter("MUEPH1", value)?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "sc5" => { validate_finite_parameter("SC5", value)?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "xldl" => { validate_parameter("XLDL", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "xldlmin" => { validate_parameter("XLDLMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "muetmp1" => { validate_finite_parameter("MUETMP1", value)?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "vbsbnd" => { validate_finite_parameter("VBSBND", value)?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "vbsmax" => { validate_finite_parameter("VBSMAX", value)?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "gleak8" => { validate_finite_parameter("GLEAK8", value)?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "gleak9" => { validate_finite_parameter("GLEAK9", value)?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "gleak10" => { validate_finite_parameter("GLEAK10", value)?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "glksd4" => { validate_finite_parameter("GLKSD4", value)?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "glksd5" => { validate_finite_parameter("GLKSD5", value)?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "glkb4" => { validate_finite_parameter("GLKB4", value)?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "glkb5" => { validate_finite_parameter("GLKB5", value)?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "glkb6" => { validate_finite_parameter("GLKB6", value)?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "glkb7" => { validate_finite_parameter("GLKB7", value)?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "glkb8" => { validate_finite_parameter("GLKB8", value)?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "glkb21" => { validate_finite_parameter("GLKB21", value)?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "glkb22" => { validate_finite_parameter("GLKB22", value)?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "glkb23" => { validate_finite_parameter("GLKB23", value)?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "glkb24" => { validate_finite_parameter("GLKB24", value)?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "glkb25" => { validate_finite_parameter("GLKB25", value)?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "glkb26" => { validate_finite_parameter("GLKB26", value)?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "glkb27" => { validate_finite_parameter("GLKB27", value)?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "glkb28" => { validate_finite_parameter("GLKB28", value)?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "ptmueph" => { validate_finite_parameter("PTMUEPH", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "mueph0b" => { validate_finite_parameter("MUEPH0B", value)?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "mueph1b" => { validate_finite_parameter("MUEPH1B", value)?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "muephwb" => { validate_finite_parameter("MUEPHWB", value)?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "muepwpb" => { validate_finite_parameter("MUEPWPB", value)?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "muephsb" => { validate_finite_parameter("MUEPHSB", value)?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "muepspb" => { validate_finite_parameter("MUEPSPB", value)?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "muephlb" => { validate_finite_parameter("MUEPHLB", value)?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "mueplpb" => { validate_finite_parameter("MUEPLPB", value)?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "muesr0b" => { validate_finite_parameter("MUESR0B", value)?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "muesr1b" => { validate_parameter("MUESR1B", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "muesrlb" => { validate_finite_parameter("MUESRLB", value)?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "mueslpb" => { validate_finite_parameter("MUESLPB", value)?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
            "muesrwb" => { validate_finite_parameter("MUESRWB", value)?; self.params.p287 = value; self.mark_param_given(287); Ok(()) }
            "mueswpb" => { validate_finite_parameter("MUESWPB", value)?; self.params.p288 = value; self.mark_param_given(288); Ok(()) }
            "muecb0b" => { validate_finite_parameter("MUECB0B", value)?; self.params.p289 = value; self.mark_param_given(289); Ok(()) }
            "muecb1b" => { validate_finite_parameter("MUECB1B", value)?; self.params.p290 = value; self.mark_param_given(290); Ok(()) }
            "muecb0lpb" => { validate_finite_parameter("MUECB0LPB", value)?; self.params.p291 = value; self.mark_param_given(291); Ok(()) }
            "muecb1lpb" => { validate_finite_parameter("MUECB1LPB", value)?; self.params.p292 = value; self.mark_param_given(292); Ok(()) }
            "muecb0l2b" => { validate_finite_parameter("MUECB0L2B", value)?; self.params.p293 = value; self.mark_param_given(293); Ok(()) }
            "muecb0l2pb" => { validate_finite_parameter("MUECB0L2PB", value)?; self.params.p294 = value; self.mark_param_given(294); Ok(()) }
            "muecb1l2b" => { validate_finite_parameter("MUECB1L2B", value)?; self.params.p295 = value; self.mark_param_given(295); Ok(()) }
            "muecb1l2pb" => { validate_finite_parameter("MUECB1L2PB", value)?; self.params.p296 = value; self.mark_param_given(296); Ok(()) }
            "pthrou" => { validate_finite_parameter("PTHROU", value)?; self.params.p297 = value; self.mark_param_given(297); Ok(()) }
            "vfbshift" => { validate_finite_parameter("VFBSHIFT", value)?; self.params.p298 = value; self.mark_param_given(298); Ok(()) }
            "mueqb" => { validate_finite_parameter("MUEQB", value)?; self.params.p299 = value; self.mark_param_given(299); Ok(()) }
            "mueqbl" => { validate_finite_parameter("MUEQBL", value)?; self.params.p300 = value; self.mark_param_given(300); Ok(()) }
            "mueqblp" => { validate_finite_parameter("MUEQBLP", value)?; self.params.p301 = value; self.mark_param_given(301); Ok(()) }
            "mueqbb" => { validate_finite_parameter("MUEQBB", value)?; self.params.p302 = value; self.mark_param_given(302); Ok(()) }
            "cocinv" => { validate_parameter("COCINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p303 = value; self.mark_param_given(303); Ok(()) }
            "web" => { validate_finite_parameter("WEB", value)?; self.params.p304 = value; self.mark_param_given(304); Ok(()) }
            "wec" => { validate_finite_parameter("WEC", value)?; self.params.p305 = value; self.mark_param_given(305); Ok(()) }
            "nsubswpe" => { validate_finite_parameter("NSUBSWPE", value)?; self.params.p306 = value; self.mark_param_given(306); Ok(()) }
            "nsubpwpe" => { validate_finite_parameter("NSUBPWPE", value)?; self.params.p307 = value; self.mark_param_given(307); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p308 = value; self.mark_param_given(308); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p309 = value; self.mark_param_given(309); Ok(()) }
            "ldrift" => { validate_parameter("LDRIFT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); Ok(()) }
            "ldrifts" => { validate_parameter("LDRIFTS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p311 = value; self.mark_param_given(311); Ok(()) }
            "cors" => { validate_parameter("CORS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p312 = value; self.mark_param_given(312); Ok(()) }
            "cord" => { validate_parameter("CORD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p313 = value; self.mark_param_given(313); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p314 = value; self.mark_param_given(314); Ok(()) }
            "novers" => { validate_finite_parameter("NOVERS", value)?; self.params.p315 = value; self.mark_param_given(315); Ok(()) }
            "rdrmued" => { validate_parameter("RDRMUED", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); Ok(()) }
            "rdrmues" => { validate_parameter("RDRMUES", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p317 = value; self.mark_param_given(317); Ok(()) }
            "rdrvmaxd" => { validate_parameter("RDRVMAXD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p318 = value; self.mark_param_given(318); Ok(()) }
            "rdrvmaxs" => { validate_parameter("RDRVMAXS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p319 = value; self.mark_param_given(319); Ok(()) }
            "rdrmuetmp" => { validate_finite_parameter("RDRMUETMP", value)?; self.params.p320 = value; self.mark_param_given(320); Ok(()) }
            "rdrvtmp" => { validate_finite_parameter("RDRVTMP", value)?; self.params.p321 = value; self.mark_param_given(321); Ok(()) }
            "rdrdjunc" => { validate_parameter("RDRDJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p322 = value; self.mark_param_given(322); Ok(()) }
            "rdrbbd" => { validate_parameter("RDRBBD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); Ok(()) }
            "rdrbbs" => { validate_parameter("RDRBBS", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); Ok(()) }
            "rdrbbtmp" => { validate_finite_parameter("RDRBBTMP", value)?; self.params.p325 = value; self.mark_param_given(325); Ok(()) }
            "rdrvmaxw" => { validate_finite_parameter("RDRVMAXW", value)?; self.params.p326 = value; self.mark_param_given(326); Ok(()) }
            "rdrvmaxwp" => { validate_finite_parameter("RDRVMAXWP", value)?; self.params.p327 = value; self.mark_param_given(327); Ok(()) }
            "rdrvmaxl" => { validate_finite_parameter("RDRVMAXL", value)?; self.params.p328 = value; self.mark_param_given(328); Ok(()) }
            "rdrvmaxlp" => { validate_finite_parameter("RDRVMAXLP", value)?; self.params.p329 = value; self.mark_param_given(329); Ok(()) }
            "rdrmuel" => { validate_finite_parameter("RDRMUEL", value)?; self.params.p330 = value; self.mark_param_given(330); Ok(()) }
            "rdrmuelp" => { validate_finite_parameter("RDRMUELP", value)?; self.params.p331 = value; self.mark_param_given(331); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimsotb_va'", name)),
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
