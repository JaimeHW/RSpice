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
    pub(crate) scalar_static_f64: Box<[f64; 954]>,
    pub(crate) scalar_static_bool: Box<[bool; 118]>,
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
            scalar_static_f64: boxed_zero_f64_array::<954>(),
            scalar_static_bool: boxed_zero_bool_array::<118>(),
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
        match name.to_ascii_lowercase().as_str() {
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "temp" => { validate_finite_parameter("TEMP", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sca" => { validate_finite_parameter("SCA", value)?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scb" => { validate_finite_parameter("SCB", value)?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scc" => { validate_finite_parameter("SCC", value)?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coadov" => { validate_parameter("COADOV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coisub" => { validate_parameter("COISUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofbe" => { validate_parameter("COFBE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coiigs" => { validate_parameter("COIIGS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cogidl" => { validate_parameter("COGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coovlp" => { validate_parameter("COOVLP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coign" => { validate_parameter("COIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coflick" => { validate_parameter("COFLICK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coisti" => { validate_parameter("COISTI", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cothrml" => { validate_parameter("COTHRML", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "conqs" => { validate_parameter("CONQS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "corg" => { validate_parameter("CORG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coievb" => { validate_parameter("COIEVB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cohist" => { validate_parameter("COHIST", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coselfheat" => { validate_parameter("COSELFHEAT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covbsbiz" => { validate_parameter("COVBSBIZ", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coqovsm" => { validate_parameter("COQOVSM", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "info" => { validate_finite_parameter("INFO", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "conewmub" => { validate_parameter("CONEWMUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgtmp1" => { validate_finite_parameter("BGTMP1", value)?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgtmp2" => { validate_finite_parameter("BGTMP2", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eg0" => { validate_finite_parameter("EG0", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xld" => { validate_finite_parameter("XLD", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbover" => { validate_finite_parameter("VFBOVER", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nover" => { validate_finite_parameter("NOVER", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xwd" => { validate_finite_parameter("XWD", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xwdc" => { validate_finite_parameter("XWDC", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy" => { validate_parameter("XQY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy1" => { validate_finite_parameter("XQY1", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy2" => { validate_finite_parameter("XQY2", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbc" => { validate_finite_parameter("VFBC", value)?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbcl1" => { validate_finite_parameter("VFBCL1", value)?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbcl1p" => { validate_finite_parameter("VFBCL1P", value)?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbcl2" => { validate_finite_parameter("VFBCL2", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbcl2p" => { validate_finite_parameter("VFBCL2P", value)?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbhamp" => { validate_finite_parameter("VFBHAMP", value)?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbi" => { validate_finite_parameter("VBI", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "parl1" => { validate_finite_parameter("PARL1", value)?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "parl2" => { validate_finite_parameter("PARL2", value)?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lp" => { validate_finite_parameter("LP", value)?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubp" => { validate_parameter("NSUBP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubp0" => { validate_finite_parameter("NSUBP0", value)?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubwp" => { validate_finite_parameter("NSUBWP", value)?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl1" => { validate_finite_parameter("WL1", value)?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl1p" => { validate_finite_parameter("WL1P", value)?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl2" => { validate_finite_parameter("WL2", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl2p" => { validate_finite_parameter("WL2P", value)?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp1" => { validate_finite_parameter("SCP1", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp2" => { validate_finite_parameter("SCP2", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp3" => { validate_finite_parameter("SCP3", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc1" => { validate_finite_parameter("SC1", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc2" => { validate_finite_parameter("SC2", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc3" => { validate_finite_parameter("SC3", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scr1" => { validate_finite_parameter("SCR1", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scr2" => { validate_finite_parameter("SCR2", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scr3" => { validate_finite_parameter("SCR3", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd1" => { validate_finite_parameter("PGD1", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd2" => { validate_finite_parameter("PGD2", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd4" => { validate_finite_parameter("PGD4", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndep" => { validate_finite_parameter("NDEP", value)?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndepl" => { validate_finite_parameter("NDEPL", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndeplp" => { validate_finite_parameter("NDEPLP", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninv" => { validate_finite_parameter("NINV", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvl" => { validate_finite_parameter("NINVL", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvlp" => { validate_finite_parameter("NINVLP", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvd" => { validate_finite_parameter("NINVD", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdp" => { validate_finite_parameter("NINVDP", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0" => { validate_finite_parameter("MUECB0", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1" => { validate_finite_parameter("MUECB1", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0lp" => { validate_finite_parameter("MUECB0LP", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1lp" => { validate_finite_parameter("MUECB1LP", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0l2" => { validate_finite_parameter("MUECB0L2", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0l2p" => { validate_finite_parameter("MUECB0L2P", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1l2" => { validate_finite_parameter("MUECB1L2", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1l2p" => { validate_finite_parameter("MUECB1L2P", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph0" => { validate_finite_parameter("MUEPH0", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephw" => { validate_finite_parameter("MUEPHW", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepwp" => { validate_finite_parameter("MUEPWP", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephl" => { validate_finite_parameter("MUEPHL", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueplp" => { validate_finite_parameter("MUEPLP", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephs" => { validate_finite_parameter("MUEPHS", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepsp" => { validate_finite_parameter("MUEPSP", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtmp" => { validate_finite_parameter("VTMP", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtmpl" => { validate_finite_parameter("VTMPL", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtmplp" => { validate_finite_parameter("VTMPLP", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvth0" => { validate_finite_parameter("WVTH0", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr1" => { validate_parameter("MUESR1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr0" => { validate_finite_parameter("MUESR0", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrl" => { validate_finite_parameter("MUESRL", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrw" => { validate_finite_parameter("MUESRW", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueswp" => { validate_finite_parameter("MUESWP", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueslp" => { validate_finite_parameter("MUESLP", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muetmp" => { validate_finite_parameter("MUETMP", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muetmpl" => { validate_finite_parameter("MUETMPL", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muetmplp" => { validate_finite_parameter("MUETMPLP", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bb" => { validate_parameter("BB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltmax" => { validate_parameter("DDLTMAX", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltslp" => { validate_parameter("DDLTSLP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltict" => { validate_finite_parameter("DDLTICT", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1" => { validate_finite_parameter("SUB1", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub2" => { validate_finite_parameter("SUB2", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1l" => { validate_finite_parameter("SUB1L", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1lp" => { validate_finite_parameter("SUB1LP", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub2l" => { validate_finite_parameter("SUB2L", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svds" => { validate_finite_parameter("SVDS", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "slg" => { validate_parameter("SLG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbs" => { validate_finite_parameter("SVBS", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbsl" => { validate_finite_parameter("SVBSL", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbslp" => { validate_finite_parameter("SVBSLP", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgs" => { validate_finite_parameter("SVGS", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgsl" => { validate_finite_parameter("SVGSL", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgslp" => { validate_finite_parameter("SVGSLP", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgsw" => { validate_finite_parameter("SVGSW", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgswp" => { validate_finite_parameter("SVGSWP", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbsub" => { validate_finite_parameter("VFBSUB", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbsubl" => { validate_finite_parameter("VFBSUBL", value)?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbsublp" => { validate_finite_parameter("VFBSUBLP", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subdlt" => { validate_finite_parameter("SUBDLT", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hist1" => { validate_finite_parameter("HIST1", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hist2" => { validate_finite_parameter("HIST2", value)?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qhe1" => { validate_finite_parameter("QHE1", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qhe2" => { validate_finite_parameter("QHE2", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "evb1" => { validate_finite_parameter("EVB1", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "evb2" => { validate_finite_parameter("EVB2", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "evb3" => { validate_finite_parameter("EVB3", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fvbs" => { validate_finite_parameter("FVBS", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc1" => { validate_finite_parameter("IBPC1", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc2" => { validate_finite_parameter("IBPC2", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsti" => { validate_parameter("NSTI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nstil" => { validate_finite_parameter("NSTIL", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nstilp" => { validate_finite_parameter("NSTILP", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nstiw" => { validate_finite_parameter("NSTIW", value)?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nstiwp" => { validate_finite_parameter("NSTIWP", value)?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsti" => { validate_finite_parameter("WSTI", value)?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ratwsti" => { validate_finite_parameter("RATWSTI", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstil" => { validate_finite_parameter("WSTIL", value)?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstilp" => { validate_finite_parameter("WSTILP", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstiw" => { validate_finite_parameter("WSTIW", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstiwp" => { validate_finite_parameter("WSTIWP", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scsti1" => { validate_finite_parameter("SCSTI1", value)?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scsti2" => { validate_finite_parameter("SCSTI2", value)?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthsti" => { validate_finite_parameter("VTHSTI", value)?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdsti" => { validate_finite_parameter("VDSTI", value)?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti1" => { validate_parameter("MUESTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti2" => { validate_parameter("MUESTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti3" => { validate_finite_parameter("MUESTI3", value)?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti1" => { validate_parameter("NSUBPSTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti2" => { validate_parameter("NSUBPSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti3" => { validate_finite_parameter("NSUBPSTI3", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubssti1" => { validate_parameter("NSUBSSTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubssti2" => { validate_parameter("NSUBSSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubssti3" => { validate_finite_parameter("NSUBSSTI3", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpoly" => { validate_finite_parameter("TPOLY", value)?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgbo" => { validate_parameter("CGBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdo" => { validate_parameter("CGDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgso" => { validate_parameter("CGSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lover" => { validate_finite_parameter("LOVER", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm1" => { validate_finite_parameter("CLM1", value)?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm2" => { validate_finite_parameter("CLM2", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm3" => { validate_finite_parameter("CLM3", value)?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm5" => { validate_finite_parameter("CLM5", value)?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm6" => { validate_finite_parameter("CLM6", value)?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vover" => { validate_finite_parameter("VOVER", value)?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverp" => { validate_finite_parameter("VOVERP", value)?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vovers" => { validate_finite_parameter("VOVERS", value)?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voversp" => { validate_finite_parameter("VOVERSP", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverl" => { validate_finite_parameter("VOVERL", value)?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverlp" => { validate_finite_parameter("VOVERLP", value)?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverw" => { validate_finite_parameter("VOVERW", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverwp" => { validate_finite_parameter("VOVERWP", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfc" => { validate_finite_parameter("WFC", value)?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubsw" => { validate_finite_parameter("NSUBSW", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcw" => { validate_finite_parameter("NSUBSW", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubswp" => { validate_finite_parameter("NSUBSWP", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcwp" => { validate_finite_parameter("NSUBSWP", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubsmax" => { validate_finite_parameter("NSUBSMAX", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcmax" => { validate_finite_parameter("NSUBSMAX", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme1" => { validate_finite_parameter("QME1", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme2" => { validate_finite_parameter("QME2", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme3" => { validate_finite_parameter("QME3", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl1" => { validate_finite_parameter("GIDL1", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl2" => { validate_finite_parameter("GIDL2", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl3" => { validate_finite_parameter("GIDL3", value)?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl4" => { validate_finite_parameter("GIDL4", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl5" => { validate_finite_parameter("GIDL5", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidlbpl1" => { validate_finite_parameter("GIDLBPL1", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidlbplt" => { validate_finite_parameter("GIDLBPLT", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak1" => { validate_finite_parameter("GLEAK1", value)?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak2" => { validate_finite_parameter("GLEAK2", value)?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak3" => { validate_finite_parameter("GLEAK3", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak4" => { validate_finite_parameter("GLEAK4", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak5" => { validate_parameter("GLEAK5", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak6" => { validate_finite_parameter("GLEAK6", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak7" => { validate_finite_parameter("GLEAK7", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd1" => { validate_finite_parameter("GLKSD1", value)?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd2" => { validate_finite_parameter("GLKSD2", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd3" => { validate_finite_parameter("GLKSD3", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb1" => { validate_finite_parameter("GLKB1", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb2" => { validate_finite_parameter("GLKB2", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb3" => { validate_finite_parameter("GLKB3", value)?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzadd0" => { validate_parameter("VZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pzadd0" => { validate_parameter("PZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nftrp" => { validate_finite_parameter("NFTRP", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfalp" => { validate_finite_parameter("NFALP", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "falph" => { validate_finite_parameter("FALPH", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((22.0, "22.0")), false, Some((32.0, "32.0")), false, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly1" => { validate_finite_parameter("DLY1", value)?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly2" => { validate_finite_parameter("DLY2", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly3" => { validate_finite_parameter("DLY3", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tfox" => { validate_parameter("TFOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tsoi" => { validate_parameter("TSOI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tfoxgidl" => { validate_parameter("TFOXGIDL", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubs" => { validate_parameter("NSUBS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubb" => { validate_parameter("NSUBB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubbl" => { validate_finite_parameter("NSUBBL", value)?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubblp" => { validate_finite_parameter("NSUBBLP", value)?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubbw" => { validate_finite_parameter("NSUBBW", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubbwp" => { validate_finite_parameter("NSUBBWP", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubbmin" => { validate_finite_parameter("NSUBBMIN", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0" => { validate_finite_parameter("RTH0", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth0" => { validate_finite_parameter("CTH0", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptl" => { validate_finite_parameter("PTL", value)?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptp" => { validate_finite_parameter("PTP", value)?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt2" => { validate_finite_parameter("PT2", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptlp" => { validate_finite_parameter("PTLP", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt4" => { validate_finite_parameter("PT4", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt4p" => { validate_finite_parameter("PT4P", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptdlt" => { validate_finite_parameter("PTDLT", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdl" => { validate_finite_parameter("GDL", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdlp" => { validate_finite_parameter("GDLP", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdld" => { validate_finite_parameter("GDLD", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph1" => { validate_finite_parameter("MUEPH1", value)?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc5" => { validate_finite_parameter("SC5", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xldl" => { validate_parameter("XLDL", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xldlmin" => { validate_parameter("XLDLMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muetmp1" => { validate_finite_parameter("MUETMP1", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbsbnd" => { validate_finite_parameter("VBSBND", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbsmax" => { validate_finite_parameter("VBSMAX", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak8" => { validate_finite_parameter("GLEAK8", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak9" => { validate_finite_parameter("GLEAK9", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak10" => { validate_finite_parameter("GLEAK10", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd4" => { validate_finite_parameter("GLKSD4", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd5" => { validate_finite_parameter("GLKSD5", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb4" => { validate_finite_parameter("GLKB4", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb5" => { validate_finite_parameter("GLKB5", value)?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb6" => { validate_finite_parameter("GLKB6", value)?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb7" => { validate_finite_parameter("GLKB7", value)?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb8" => { validate_finite_parameter("GLKB8", value)?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb21" => { validate_finite_parameter("GLKB21", value)?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb22" => { validate_finite_parameter("GLKB22", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb23" => { validate_finite_parameter("GLKB23", value)?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb24" => { validate_finite_parameter("GLKB24", value)?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb25" => { validate_finite_parameter("GLKB25", value)?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb26" => { validate_finite_parameter("GLKB26", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb27" => { validate_finite_parameter("GLKB27", value)?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb28" => { validate_finite_parameter("GLKB28", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptmueph" => { validate_finite_parameter("PTMUEPH", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph0b" => { validate_finite_parameter("MUEPH0B", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph1b" => { validate_finite_parameter("MUEPH1B", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephwb" => { validate_finite_parameter("MUEPHWB", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepwpb" => { validate_finite_parameter("MUEPWPB", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephsb" => { validate_finite_parameter("MUEPHSB", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepspb" => { validate_finite_parameter("MUEPSPB", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephlb" => { validate_finite_parameter("MUEPHLB", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueplpb" => { validate_finite_parameter("MUEPLPB", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr0b" => { validate_finite_parameter("MUESR0B", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr1b" => { validate_parameter("MUESR1B", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrlb" => { validate_finite_parameter("MUESRLB", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueslpb" => { validate_finite_parameter("MUESLPB", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrwb" => { validate_finite_parameter("MUESRWB", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueswpb" => { validate_finite_parameter("MUESWPB", value)?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0b" => { validate_finite_parameter("MUECB0B", value)?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1b" => { validate_finite_parameter("MUECB1B", value)?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0lpb" => { validate_finite_parameter("MUECB0LPB", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1lpb" => { validate_finite_parameter("MUECB1LPB", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0l2b" => { validate_finite_parameter("MUECB0L2B", value)?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0l2pb" => { validate_finite_parameter("MUECB0L2PB", value)?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1l2b" => { validate_finite_parameter("MUECB1L2B", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1l2pb" => { validate_finite_parameter("MUECB1L2PB", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pthrou" => { validate_finite_parameter("PTHROU", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbshift" => { validate_finite_parameter("VFBSHIFT", value)?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueqb" => { validate_finite_parameter("MUEQB", value)?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueqbl" => { validate_finite_parameter("MUEQBL", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueqblp" => { validate_finite_parameter("MUEQBLP", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueqbb" => { validate_finite_parameter("MUEQBB", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cocinv" => { validate_parameter("COCINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "web" => { validate_finite_parameter("WEB", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wec" => { validate_finite_parameter("WEC", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubswpe" => { validate_finite_parameter("NSUBSWPE", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpwpe" => { validate_finite_parameter("NSUBPWPE", value)?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrift" => { validate_parameter("LDRIFT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrifts" => { validate_parameter("LDRIFTS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cors" => { validate_parameter("CORS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cord" => { validate_parameter("CORD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novers" => { validate_finite_parameter("NOVERS", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmued" => { validate_parameter("RDRMUED", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmues" => { validate_parameter("RDRMUES", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxd" => { validate_parameter("RDRVMAXD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxs" => { validate_parameter("RDRVMAXS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuetmp" => { validate_finite_parameter("RDRMUETMP", value)?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvtmp" => { validate_finite_parameter("RDRVTMP", value)?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrdjunc" => { validate_parameter("RDRDJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbd" => { validate_parameter("RDRBBD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbs" => { validate_parameter("RDRBBS", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbtmp" => { validate_finite_parameter("RDRBBTMP", value)?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxw" => { validate_finite_parameter("RDRVMAXW", value)?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxwp" => { validate_finite_parameter("RDRVMAXWP", value)?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxl" => { validate_finite_parameter("RDRVMAXL", value)?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxlp" => { validate_finite_parameter("RDRVMAXLP", value)?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuel" => { validate_finite_parameter("RDRMUEL", value)?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuelp" => { validate_finite_parameter("RDRMUELP", value)?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
        self.scalar_static_f64[6]=p.p207;
        self.scalar_static_f64[7]=p.p17;
        self.scalar_static_f64[8]=p.p228;
        self.scalar_static_f64[9]=p.p18;
        self.scalar_static_f64[10]=p.p201;
        self.scalar_static_f64[11]=p.p162;
        self.scalar_static_f64[12]=p.p164;
        self.scalar_static_f64[13]=if param_given[177] { 1.0 } else { 0.0 };
        self.scalar_static_f64[14]=p.p177;
        self.scalar_static_f64[15]=p.p227;
        self.scalar_static_f64[16]=p.p230;
        self.scalar_static_f64[17]=(self.scalar_static_f64[15]*self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(5000000000.0/self.scalar_static_f64[17]);
        self.scalar_static_f64[19]=(if (self.scalar_static_f64[13]!=0.0){self.scalar_static_f64[14]}else{self.scalar_static_f64[18]});
        self.scalar_static_bool[1]=(self.scalar_static_f64[19]<2.1);
        self.scalar_static_bool[2]=(self.scalar_static_bool[1]&&true);
        self.scalar_static_f64[20]=(2.1-self.scalar_static_f64[19]);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[2]{self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[22]=(self.scalar_static_f64[21]*self.scalar_static_f64[21]);
        self.scalar_static_f64[23]=(if self.scalar_static_bool[2]{self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=(if self.scalar_static_bool[2]{0.010000000000000002}else{0.0});
        self.scalar_static_f64[25]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[26]=(self.scalar_static_f64[23]*self.scalar_static_f64[25]);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[2]{self.scalar_static_f64[26]}else{self.scalar_static_f64[25]});
        self.scalar_static_f64[28]=(self.scalar_static_f64[24]*self.scalar_static_f64[25]);
        self.scalar_static_f64[29]=(if self.scalar_static_bool[2]{self.scalar_static_f64[28]}else{self.scalar_static_f64[25]});
        self.scalar_static_f64[30]=(self.scalar_static_f64[23]*self.scalar_static_f64[27]);
        self.scalar_static_f64[31]=(if self.scalar_static_bool[2]{self.scalar_static_f64[30]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[32]=(self.scalar_static_f64[24]*self.scalar_static_f64[29]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[2]{self.scalar_static_f64[32]}else{self.scalar_static_f64[29]});
        self.scalar_static_f64[34]=(self.scalar_static_f64[31]+self.scalar_static_f64[33]);
        self.scalar_static_f64[35]=(if self.scalar_static_bool[2]{self.scalar_static_f64[34]}else{0.0});
        self.scalar_static_f64[36]=(0.1*self.scalar_static_f64[21]);
        self.scalar_static_f64[37]=(if self.scalar_static_bool[2]{1e50}else{0.0});
        self.scalar_static_f64[38]=(self.scalar_static_f64[36]*self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=(if self.scalar_static_bool[2]{self.scalar_static_f64[38]}else{0.0});
        self.scalar_static_f64[40]=(2.1-self.scalar_static_f64[39]);
        self.scalar_static_f64[41]=(if self.scalar_static_bool[2]{self.scalar_static_f64[40]}else{self.scalar_static_f64[19]});
        self.scalar_static_f64[42]=p.p34;
        self.scalar_static_f64[43]=(self.scalar_static_f64[42]*0.01);
        self.scalar_static_f64[44]=p.p59;
        self.scalar_static_f64[45]=(self.scalar_static_f64[44]/1e-6);
        self.scalar_static_f64[46]=p.p101;
        self.scalar_static_f64[47]=(0.01*self.scalar_static_f64[46]);
        self.scalar_static_f64[48]=p.p192;
        self.scalar_static_f64[49]=(self.scalar_static_f64[48]/1e-6);
        self.scalar_static_f64[50]=p.p219;
        self.scalar_static_f64[51]=(0.01*self.scalar_static_f64[50]);
        self.scalar_static_f64[52]=p.p220;
        self.scalar_static_f64[53]=(self.scalar_static_f64[52]/0.0001);
        self.scalar_static_f64[54]=(self.scalar_static_f64[16]/1e-6);
        self.scalar_static_f64[55]=p.p231;
        self.scalar_static_f64[56]=(self.scalar_static_f64[55]/1e-6);
        self.scalar_static_f64[57]=p.p237;
        self.scalar_static_f64[58]=(0.01*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=p.p238;
        self.scalar_static_f64[60]=(self.scalar_static_f64[59]/0.01);
        self.scalar_static_f64[61]=p.p40;
        self.scalar_static_f64[62]=(self.scalar_static_f64[61]/1e-6);
        self.scalar_static_f64[63]=p.p236;
        self.scalar_static_f64[64]=(self.scalar_static_f64[63]/1e-6);
        self.scalar_static_f64[65]=p.p197;
        self.scalar_static_f64[66]=(self.scalar_static_f64[65]/0.01);
        self.scalar_static_f64[67]=p.p306;
        self.scalar_static_f64[68]=(self.scalar_static_f64[67]/1e-6);
        self.scalar_static_f64[69]=p.p307;
        self.scalar_static_f64[70]=(self.scalar_static_f64[69]/1e-6);
        self.scalar_static_f64[71]=p.p189;
        self.scalar_static_f64[72]=(self.scalar_static_f64[71]*10000.0);
        self.scalar_static_f64[73]=p.p147;
        self.scalar_static_f64[74]=(self.scalar_static_f64[73]/1e-6);
        self.scalar_static_f64[75]=p.p196;
        self.scalar_static_f64[76]=(self.scalar_static_f64[75]/10.0);
        self.scalar_static_f64[77]=p.p222;
        self.scalar_static_f64[78]=(self.scalar_static_f64[77]+273.15);
        self.scalar_static_f64[79]=p.p9;
        self.scalar_static_f64[80]=(273.15+self.scalar_static_f64[79]);
        self.scalar_static_f64[81]=p.p41;
        self.scalar_static_f64[82]=p.p42;
        self.scalar_static_f64[83]=p.p0;
        self.scalar_static_f64[84]=p.p1;
        self.scalar_static_f64[85]=p.p5;
        self.scalar_static_f64[86]=(self.scalar_static_f64[84]/self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=(self.scalar_static_f64[83]*1000000.0);
        self.scalar_static_f64[88]=(self.scalar_static_f64[86]*1000000.0);
        self.scalar_static_f64[89]=(self.scalar_static_f64[87]*self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=p.p62;
        self.scalar_static_f64[91]=p.p63;
        self.scalar_static_f64[92]=f64::powf(self.scalar_static_f64[89],self.scalar_static_f64[91]);
        self.scalar_static_f64[93]=(self.scalar_static_f64[90]/self.scalar_static_f64[92]);
        self.scalar_static_f64[94]=(self.scalar_static_f64[83]+self.scalar_static_f64[93]);
        self.scalar_static_f64[95]=(self.scalar_static_f64[86]+self.scalar_static_f64[93]);
        self.scalar_static_f64[96]=p.p64;
        self.scalar_static_f64[97]=p.p65;
        self.scalar_static_f64[98]=f64::powf(self.scalar_static_f64[89],self.scalar_static_f64[97]);
        self.scalar_static_f64[99]=(self.scalar_static_f64[96]/self.scalar_static_f64[98]);
        self.scalar_static_f64[100]=p.p148;
        self.scalar_static_f64[101]=(1000000.0*self.scalar_static_f64[94]);
        self.scalar_static_f64[102]=p.p149;
        self.scalar_static_f64[103]=f64::powf(self.scalar_static_f64[101],self.scalar_static_f64[102]);
        self.scalar_static_f64[104]=(self.scalar_static_f64[100]/self.scalar_static_f64[103]);
        self.scalar_static_f64[105]=(1.0+self.scalar_static_f64[104]);
        self.scalar_static_f64[106]=p.p150;
        self.scalar_static_f64[107]=(1000000.0*self.scalar_static_f64[95]);
        self.scalar_static_f64[108]=p.p151;
        self.scalar_static_f64[109]=f64::powf(self.scalar_static_f64[107],self.scalar_static_f64[108]);
        self.scalar_static_f64[110]=(self.scalar_static_f64[106]/self.scalar_static_f64[109]);
        self.scalar_static_f64[111]=(1.0+self.scalar_static_f64[110]);
        self.scalar_static_f64[112]=(self.scalar_static_f64[74]*self.scalar_static_f64[105]);
        self.scalar_static_f64[113]=(self.scalar_static_f64[111]*self.scalar_static_f64[112]);
        self.scalar_static_f64[114]=p.p154;
        self.scalar_static_f64[115]=p.p155;
        self.scalar_static_f64[116]=f64::powf(self.scalar_static_f64[101],self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=(self.scalar_static_f64[114]/self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(1.0+self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=p.p156;
        self.scalar_static_f64[120]=p.p157;
        self.scalar_static_f64[121]=f64::powf(self.scalar_static_f64[107],self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[119]/self.scalar_static_f64[121]);
        self.scalar_static_f64[123]=(1.0+self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=p.p152;
        self.scalar_static_f64[125]=(self.scalar_static_f64[118]*self.scalar_static_f64[124]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[123]*self.scalar_static_f64[125]);
        self.scalar_static_f64[127]=(2.0*self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=p.p153;
        self.scalar_static_f64[129]=(self.scalar_static_f64[127]*self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=(2.0*self.scalar_static_f64[81]);
        self.scalar_static_f64[131]=(self.scalar_static_f64[86]-self.scalar_static_f64[130]);
        self.scalar_static_f64[132]=(self.scalar_static_f64[131]-self.scalar_static_f64[129]);
        self.scalar_static_f64[133]=(2.0*self.scalar_static_f64[82]);
        self.scalar_static_f64[134]=(self.scalar_static_f64[86]-self.scalar_static_f64[133]);
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]-self.scalar_static_f64[129]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[85]*self.scalar_static_f64[132]);
        self.scalar_static_f64[137]=(self.scalar_static_f64[85]*self.scalar_static_f64[135]);
        self.scalar_static_f64[138]=(self.scalar_static_f64[58]/self.scalar_static_f64[136]);
        self.scalar_static_f64[139]=(self.scalar_static_f64[60]*self.scalar_static_f64[137]);
        self.scalar_static_f64[140]=p.p11;
        self.scalar_static_f64[141]=p.p304;
        self.scalar_static_f64[142]=p.p12;
        self.scalar_static_f64[143]=(self.scalar_static_f64[141]*self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(self.scalar_static_f64[140]+self.scalar_static_f64[143]);
        self.scalar_static_f64[145]=p.p305;
        self.scalar_static_f64[146]=p.p13;
        self.scalar_static_f64[147]=(self.scalar_static_f64[145]*self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=(self.scalar_static_f64[144]+self.scalar_static_f64[147]);
        self.scalar_static_f64[149]=(self.scalar_static_f64[68]*self.scalar_static_f64[148]);
        self.scalar_static_f64[150]=(self.scalar_static_f64[54]+self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=(self.scalar_static_f64[150]-1e21);
        self.scalar_static_f64[152]=(self.scalar_static_f64[151]-10000.0);
        self.scalar_static_f64[153]=(self.scalar_static_f64[152]*self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=(4e25+self.scalar_static_f64[153]);
        self.scalar_static_f64[155]=(self.scalar_static_f64[154]).sqrt();
        self.scalar_static_f64[156]=(self.scalar_static_f64[152]+self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=(0.5*self.scalar_static_f64[156]);
        self.scalar_static_f64[158]=(1e21+self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=(self.scalar_static_f64[70]*self.scalar_static_f64[148]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[45]+self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[160]-1e21);
        self.scalar_static_f64[162]=(self.scalar_static_f64[161]-10000.0);
        self.scalar_static_f64[163]=(self.scalar_static_f64[162]*self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=(4e25+self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=(self.scalar_static_f64[164]).sqrt();
        self.scalar_static_f64[166]=(self.scalar_static_f64[162]+self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=(0.5*self.scalar_static_f64[166]);
        self.scalar_static_f64[168]=(1e21+self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=p.p86;
        self.scalar_static_f64[170]=p.p88;
        self.scalar_static_f64[171]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[170]);
        self.scalar_static_f64[172]=(self.scalar_static_f64[169]*self.scalar_static_f64[171]);
        self.scalar_static_f64[173]=p.p90;
        self.scalar_static_f64[174]=p.p91;
        self.scalar_static_f64[175]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[173]/self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=(1.0+self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[172]*self.scalar_static_f64[177]);
        self.scalar_static_f64[179]=p.p87;
        self.scalar_static_f64[180]=p.p89;
        self.scalar_static_f64[181]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[180]);
        self.scalar_static_f64[182]=(self.scalar_static_f64[179]*self.scalar_static_f64[181]);
        self.scalar_static_f64[183]=p.p92;
        self.scalar_static_f64[184]=p.p93;
        self.scalar_static_f64[185]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[184]);
        self.scalar_static_f64[186]=(self.scalar_static_f64[183]/self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=(1.0+self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=(self.scalar_static_f64[182]*self.scalar_static_f64[187]);
        self.scalar_static_f64[189]=p.p289;
        self.scalar_static_f64[190]=p.p291;
        self.scalar_static_f64[191]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[189]*self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=p.p293;
        self.scalar_static_f64[194]=p.p294;
        self.scalar_static_f64[195]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[193]/self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=(1.0+self.scalar_static_f64[196]);
        self.scalar_static_f64[198]=(self.scalar_static_f64[192]*self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=p.p290;
        self.scalar_static_f64[200]=p.p292;
        self.scalar_static_f64[201]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[200]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[199]*self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=p.p295;
        self.scalar_static_f64[204]=p.p296;
        self.scalar_static_f64[205]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[203]/self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=(1.0+self.scalar_static_f64[206]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[202]*self.scalar_static_f64[207]);
        self.scalar_static_f64[209]=p.p106;
        self.scalar_static_f64[210]=p.p107;
        self.scalar_static_f64[211]=p.p110;
        self.scalar_static_f64[212]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[210]/self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=(1.0+self.scalar_static_f64[213]);
        self.scalar_static_f64[215]=(self.scalar_static_f64[209]*self.scalar_static_f64[214]);
        self.scalar_static_f64[216]=p.p108;
        self.scalar_static_f64[217]=p.p109;
        self.scalar_static_f64[218]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[217]);
        self.scalar_static_f64[219]=(self.scalar_static_f64[216]/self.scalar_static_f64[218]);
        self.scalar_static_f64[220]=(1.0+self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=(self.scalar_static_f64[215]*self.scalar_static_f64[220]);
        self.scalar_static_f64[222]=p.p283;
        self.scalar_static_f64[223]=p.p285;
        self.scalar_static_f64[224]=p.p286;
        self.scalar_static_f64[225]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[224]);
        self.scalar_static_f64[226]=(self.scalar_static_f64[223]/self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=(1.0+self.scalar_static_f64[226]);
        self.scalar_static_f64[228]=(self.scalar_static_f64[222]*self.scalar_static_f64[227]);
        self.scalar_static_f64[229]=p.p287;
        self.scalar_static_f64[230]=p.p288;
        self.scalar_static_f64[231]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[230]);
        self.scalar_static_f64[232]=(self.scalar_static_f64[229]/self.scalar_static_f64[231]);
        self.scalar_static_f64[233]=(1.0+self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=(self.scalar_static_f64[228]*self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=p.p232;
        self.scalar_static_f64[236]=p.p233;
        self.scalar_static_f64[237]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[236]);
        self.scalar_static_f64[238]=(self.scalar_static_f64[235]/self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=(1.0+self.scalar_static_f64[238]);
        self.scalar_static_f64[240]=(self.scalar_static_f64[56]*self.scalar_static_f64[239]);
        self.scalar_static_f64[241]=(self.scalar_static_f64[240]-self.scalar_static_f64[64]);
        self.scalar_static_f64[242]=(self.scalar_static_f64[56]*0.001);
        self.scalar_static_f64[243]=(self.scalar_static_f64[241]-self.scalar_static_f64[242]);
        self.scalar_static_f64[244]=(4.0*self.scalar_static_f64[64]);
        self.scalar_static_f64[245]=(self.scalar_static_f64[242]*self.scalar_static_f64[244]);
        self.scalar_static_bool[3]=(self.scalar_static_f64[245]>0.0);
        self.scalar_static_f64[246]=(-self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=(if self.scalar_static_bool[3]{self.scalar_static_f64[245]}else{self.scalar_static_f64[246]});
        self.scalar_static_f64[248]=(self.scalar_static_f64[243]*self.scalar_static_f64[243]);
        self.scalar_static_f64[249]=(self.scalar_static_f64[247]+self.scalar_static_f64[248]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[249]).sqrt();
        self.scalar_static_f64[251]=(self.scalar_static_f64[243]+self.scalar_static_f64[250]);
        self.scalar_static_f64[252]=(0.5*self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[64]+self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=p.p32;
        self.scalar_static_f64[255]=p.p234;
        self.scalar_static_f64[256]=p.p235;
        self.scalar_static_f64[257]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[256]);
        self.scalar_static_f64[258]=(self.scalar_static_f64[255]/self.scalar_static_f64[257]);
        self.scalar_static_f64[259]=(1.0+self.scalar_static_f64[258]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[253]*self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[260]}else{self.scalar_static_f64[240]});
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]-self.scalar_static_f64[64]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[262]-self.scalar_static_f64[242]);
        self.scalar_static_f64[264]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[263]}else{self.scalar_static_f64[243]});
        self.scalar_static_f64[265]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[245]}else{self.scalar_static_f64[250]});
        self.scalar_static_bool[4]=(self.scalar_static_f64[265]>0.0);
        self.scalar_static_f64[266]=(-self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[4]{self.scalar_static_f64[265]}else{self.scalar_static_f64[266]});
        self.scalar_static_f64[268]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[267]}else{self.scalar_static_f64[265]});
        self.scalar_static_f64[269]=(self.scalar_static_f64[264]*self.scalar_static_f64[264]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[268]+self.scalar_static_f64[269]);
        self.scalar_static_f64[271]=(self.scalar_static_f64[270]).sqrt();
        self.scalar_static_f64[272]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[271]}else{self.scalar_static_f64[268]});
        self.scalar_static_f64[273]=(self.scalar_static_f64[264]+self.scalar_static_f64[272]);
        self.scalar_static_f64[274]=(0.5*self.scalar_static_f64[273]);
        self.scalar_static_f64[275]=(self.scalar_static_f64[64]+self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=(if (self.scalar_static_f64[254]!=0.0){self.scalar_static_f64[275]}else{self.scalar_static_f64[253]});
        self.scalar_static_f64[277]=p.p60;
        self.scalar_static_f64[278]=p.p61;
        self.scalar_static_f64[279]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[278]);
        self.scalar_static_f64[280]=(self.scalar_static_f64[277]/self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(1.0+self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(self.scalar_static_f64[168]*self.scalar_static_f64[281]);
        self.scalar_static_f64[283]=p.p43;
        self.scalar_static_f64[284]=(0.5*self.scalar_static_f64[83]);
        self.scalar_static_f64[285]=(self.scalar_static_f64[283]+self.scalar_static_f64[284]);
        self.scalar_static_f64[286]=(1.0/self.scalar_static_f64[285]);
        self.scalar_static_f64[287]=p.p44;
        self.scalar_static_f64[288]=(self.scalar_static_f64[284]+self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(1.0/self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[286]+self.scalar_static_f64[289]);
        self.scalar_static_f64[291]=(2.0/self.scalar_static_f64[290]);
        self.scalar_static_f64[292]=p.p6;
        self.scalar_static_bool[5]=(self.scalar_static_f64[292]>0.0);
        self.scalar_static_f64[293]=p.p7;
        self.scalar_static_bool[6]=(self.scalar_static_f64[293]>0.0);
        self.scalar_static_bool[7]=(self.scalar_static_bool[5]&&self.scalar_static_bool[6]);
        self.scalar_static_bool[8]=(1.0==self.scalar_static_f64[85]);
        self.scalar_static_bool[9]=(self.scalar_static_f64[85]>1.0);
        self.scalar_static_f64[294]=p.p8;
        self.scalar_static_bool[10]=(self.scalar_static_f64[294]>0.0);
        self.scalar_static_bool[11]=(self.scalar_static_bool[9]&&self.scalar_static_bool[10]);
        self.scalar_static_bool[12]=(self.scalar_static_bool[8]||self.scalar_static_bool[11]);
        self.scalar_static_bool[13]=(self.scalar_static_bool[7]&&self.scalar_static_bool[12]);
        self.scalar_static_f64[295]=(if self.scalar_static_bool[13]{0.0}else{self.scalar_static_f64[290]});
        self.scalar_static_f64[296]=(self.scalar_static_f64[284]+self.scalar_static_f64[292]);
        self.scalar_static_f64[297]=(self.scalar_static_f64[83]+self.scalar_static_f64[294]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[284]+self.scalar_static_f64[293]);
        self.scalar_static_f64[299]=(2.0*self.scalar_static_f64[85]);
        self.scalar_static_bool[14]=(!self.scalar_static_bool[13]);
        self.scalar_static_f64[300]=p.p166;
        self.scalar_static_f64[301]=(1.0+self.scalar_static_f64[300]);
        self.scalar_static_f64[302]=(1.0/self.scalar_static_f64[301]);
        self.scalar_static_f64[303]=p.p169;
        self.scalar_static_f64[304]=(1.0+self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=(1.0/self.scalar_static_f64[304]);
        self.scalar_static_f64[306]=p.p168;
        self.scalar_static_f64[307]=p.p170;
        self.scalar_static_f64[308]=(self.scalar_static_f64[306]/self.scalar_static_f64[291]);
        self.scalar_static_f64[309]=f64::powf(self.scalar_static_f64[308],self.scalar_static_f64[307]);
        self.scalar_static_f64[310]=p.p190;
        self.scalar_static_f64[311]=p.p191;
        self.scalar_static_f64[312]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[311]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[310]/self.scalar_static_f64[312]);
        self.scalar_static_f64[314]=(1.0+self.scalar_static_f64[313]);
        self.scalar_static_f64[315]=p.p58;
        self.scalar_static_bool[15]=(self.scalar_static_f64[83]>self.scalar_static_f64[315]);
        self.scalar_static_bool[16]=(self.scalar_static_f64[315]<=0.0);
        self.scalar_static_bool[17]=(self.scalar_static_bool[15]||self.scalar_static_bool[16]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[83]-self.scalar_static_f64[315]);
        self.scalar_static_bool[18]=(!self.scalar_static_bool[17]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[315]-self.scalar_static_f64[83]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[276]*1.6021918e-19);
        self.scalar_static_f64[319]=(1.034943e-10*self.scalar_static_f64[318]);
        self.scalar_static_f64[320]=p.p242;
        self.scalar_static_f64[321]=(-self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[321]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[5]*self.scalar_static_f64[322]);
        self.scalar_static_f64[324]=p.p243;
        self.scalar_static_f64[325]=p.p244;
        self.scalar_static_f64[326]=(-self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[326]);
        self.scalar_static_f64[328]=(self.scalar_static_f64[324]*self.scalar_static_f64[327]);
        self.scalar_static_f64[329]=p.p246;
        self.scalar_static_f64[330]=p.p248;
        self.scalar_static_f64[331]=(self.scalar_static_f64[87]+self.scalar_static_f64[330]);
        self.scalar_static_f64[332]=p.p247;
        self.scalar_static_f64[333]=(-self.scalar_static_f64[332]);
        self.scalar_static_f64[334]=f64::powf(self.scalar_static_f64[331],self.scalar_static_f64[333]);
        self.scalar_static_f64[335]=(self.scalar_static_f64[329]*self.scalar_static_f64[334]);
        self.scalar_static_f64[336]=(2.0*self.scalar_static_f64[315]);
        self.scalar_static_bool[19]=(self.scalar_static_f64[83]<=self.scalar_static_f64[336]);
        self.scalar_static_bool[20]=(self.scalar_static_f64[315]>0.0);
        self.scalar_static_bool[21]=(self.scalar_static_bool[19]&&self.scalar_static_bool[20]);
        self.scalar_static_bool[22]=(!self.scalar_static_bool[21]);
        self.scalar_static_f64[337]=(1.0/self.scalar_static_f64[87]);
        self.scalar_static_f64[338]=(1.0+self.scalar_static_f64[337]);
        self.scalar_static_f64[339]=p.p77;
        self.scalar_static_f64[340]=f64::powf(self.scalar_static_f64[338],self.scalar_static_f64[339]);
        self.scalar_static_f64[341]=p.p75;
        self.scalar_static_f64[342]=(self.scalar_static_f64[340]*self.scalar_static_f64[341]);
        self.scalar_static_f64[343]=p.p116;
        self.scalar_static_f64[344]=(self.scalar_static_f64[87]*self.scalar_static_f64[343]);
        self.scalar_static_f64[345]=p.p115;
        self.scalar_static_f64[346]=(self.scalar_static_f64[344]*self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=(self.scalar_static_f64[344]+self.scalar_static_f64[345]);
        self.scalar_static_f64[348]=(self.scalar_static_f64[346]/self.scalar_static_f64[347]);
        self.scalar_static_f64[349]=p.p117;
        self.scalar_static_f64[350]=(self.scalar_static_f64[348]+self.scalar_static_f64[349]);
        self.scalar_static_f64[351]=(1e-50+self.scalar_static_f64[350]);
        self.scalar_static_f64[352]=p.p179;
        self.scalar_static_f64[353]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[352]);
        self.scalar_static_f64[354]=p.p180;
        self.scalar_static_f64[355]=(self.scalar_static_f64[353]*self.scalar_static_f64[354]);
        self.scalar_static_f64[356]=(1.0+self.scalar_static_f64[355]);
        self.scalar_static_f64[357]=p.p25;
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[357]);
        self.scalar_static_f64[358]=p.p3;
        self.scalar_static_f64[359]=p.p2;
        self.scalar_static_f64[360]=(3.0*self.scalar_static_f64[359]);
        self.scalar_static_f64[361]=(self.scalar_static_f64[132]/self.scalar_static_f64[360]);
        self.scalar_static_f64[362]=(self.scalar_static_f64[358]+self.scalar_static_f64[361]);
        self.scalar_static_f64[363]=(if self.scalar_static_bool[23]{self.scalar_static_f64[362]}else{self.scalar_static_f64[344]});
        self.scalar_static_f64[364]=p.p48;
        self.scalar_static_f64[365]=(self.scalar_static_f64[363]*self.scalar_static_f64[364]);
        self.scalar_static_f64[366]=p.p4;
        self.scalar_static_f64[367]=(self.scalar_static_f64[83]-self.scalar_static_f64[366]);
        self.scalar_static_f64[368]=(self.scalar_static_f64[359]*self.scalar_static_f64[367]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[85]*self.scalar_static_f64[368]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[365]/self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=(if self.scalar_static_bool[23]{self.scalar_static_f64[370]}else{0.0});
        self.scalar_static_bool[24]=(self.scalar_static_f64[371]>0.001);
        self.scalar_static_bool[25]=(self.scalar_static_bool[23]&&self.scalar_static_bool[24]);
        self.scalar_static_f64[372]=(1.0/self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=(if self.scalar_static_bool[25]{self.scalar_static_f64[372]}else{self.scalar_static_f64[371]});
        self.scalar_static_bool[26]=(!self.scalar_static_bool[24]);
        self.scalar_static_bool[27]=(self.scalar_static_bool[23]&&self.scalar_static_bool[26]);
        self.scalar_static_f64[374]=(if self.scalar_static_bool[27]{1000.0}else{self.scalar_static_f64[373]});
        self.scalar_static_bool[28]=(!self.scalar_static_bool[23]);
        self.scalar_static_f64[375]=(if self.scalar_static_bool[28]{1000.0}else{self.scalar_static_f64[374]});
        self.scalar_static_f64[376]=p.p131;
        self.scalar_static_f64[377]=p.p132;
        self.scalar_static_f64[378]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[377]);
        self.scalar_static_f64[379]=(self.scalar_static_f64[376]/self.scalar_static_f64[378]);
        self.scalar_static_f64[380]=(1.0+self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=p.p125;
        self.scalar_static_f64[382]=p.p126;
        self.scalar_static_f64[383]=p.p127;
        self.scalar_static_f64[384]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[383]);
        self.scalar_static_f64[385]=(self.scalar_static_f64[382]/self.scalar_static_f64[384]);
        self.scalar_static_f64[386]=(1.0+self.scalar_static_f64[385]);
        self.scalar_static_f64[387]=(self.scalar_static_f64[381]*self.scalar_static_f64[386]);
        self.scalar_static_f64[388]=p.p124;
        self.scalar_static_f64[389]=(self.scalar_static_f64[87]+self.scalar_static_f64[388]);
        self.scalar_static_f64[390]=(self.scalar_static_f64[87]/self.scalar_static_f64[389]);
        self.scalar_static_f64[391]=p.p118;
        self.scalar_static_f64[392]=p.p120;
        self.scalar_static_f64[393]=p.p121;
        self.scalar_static_f64[394]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[393]);
        self.scalar_static_f64[395]=(self.scalar_static_f64[392]/self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=(1.0+self.scalar_static_f64[395]);
        self.scalar_static_f64[397]=(self.scalar_static_f64[391]*self.scalar_static_f64[396]);
        self.scalar_static_f64[398]=p.p119;
        self.scalar_static_f64[399]=p.p122;
        self.scalar_static_f64[400]=(self.scalar_static_f64[399]/self.scalar_static_f64[87]);
        self.scalar_static_f64[401]=(1.0+self.scalar_static_f64[400]);
        self.scalar_static_f64[402]=(self.scalar_static_f64[398]*self.scalar_static_f64[401]);
        self.scalar_static_f64[403]=(10000.0*self.scalar_static_f64[137]);
        self.scalar_static_f64[404]=p.p46;
        self.scalar_static_f64[405]=(self.scalar_static_f64[403]*self.scalar_static_f64[404]);
        self.scalar_static_f64[406]=p.p47;
        self.scalar_static_f64[407]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[406]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[405]/self.scalar_static_f64[407]);
        self.scalar_static_f64[409]=p.p133;
        self.scalar_static_f64[410]=p.p134;
        self.scalar_static_f64[411]=p.p135;
        self.scalar_static_f64[412]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[411]);
        self.scalar_static_f64[413]=(self.scalar_static_f64[410]/self.scalar_static_f64[412]);
        self.scalar_static_f64[414]=(1.0+self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(self.scalar_static_f64[409]*self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=p.p128;
        self.scalar_static_f64[417]=p.p129;
        self.scalar_static_f64[418]=p.p130;
        self.scalar_static_f64[419]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[418]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[417]/self.scalar_static_f64[419]);
        self.scalar_static_f64[421]=(1.0+self.scalar_static_f64[420]);
        self.scalar_static_f64[422]=(self.scalar_static_f64[416]*self.scalar_static_f64[421]);
        self.scalar_static_f64[423]=p.p33;
        self.scalar_static_f64[424]=p.p28;
        self.scalar_static_bool[29]=(self.scalar_static_f64[57]>0.0);
        self.scalar_static_bool[30]=((self.scalar_static_f64[424]!=0.0)&&self.scalar_static_bool[29]);
        self.scalar_static_bool[31]=(!self.scalar_static_bool[30]);
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[0]!=0.0));
        self.scalar_static_f64[425]=p.p10;
        self.scalar_static_f64[426]=p.p37;
        self.scalar_static_f64[427]=(self.scalar_static_f64[78]*1e-7);
        self.scalar_static_f64[428]=(9.025e-5+self.scalar_static_f64[427]);
        self.scalar_static_f64[429]=(self.scalar_static_f64[78]*self.scalar_static_f64[428]);
        self.scalar_static_f64[430]=(self.scalar_static_f64[426]-self.scalar_static_f64[429]);
        self.scalar_static_f64[431]=(self.scalar_static_f64[78]*self.scalar_static_f64[78]);
        self.scalar_static_f64[432]=p.p35;
        self.scalar_static_f64[433]=p.p36;
        self.scalar_static_f64[434]=(self.scalar_static_f64[78]*1.3806226e-23);
        self.scalar_static_f64[435]=(1.6021918e-19/self.scalar_static_f64[434]);
        self.scalar_static_f64[436]=p.p202;
        self.scalar_static_f64[437]=p.p249;
        self.scalar_static_f64[438]=p.p95;
        self.scalar_static_f64[439]=p.p96;
        self.scalar_static_f64[440]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[439]);
        self.scalar_static_f64[441]=(self.scalar_static_f64[438]/self.scalar_static_f64[440]);
        self.scalar_static_f64[442]=(1.0+self.scalar_static_f64[441]);
        self.scalar_static_f64[443]=(self.scalar_static_f64[437]*self.scalar_static_f64[442]);
        self.scalar_static_f64[444]=p.p97;
        self.scalar_static_f64[445]=p.p98;
        self.scalar_static_f64[446]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[445]);
        self.scalar_static_f64[447]=(self.scalar_static_f64[444]/self.scalar_static_f64[446]);
        self.scalar_static_f64[448]=(1.0+self.scalar_static_f64[447]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[443]*self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=p.p99;
        self.scalar_static_f64[451]=p.p100;
        self.scalar_static_f64[452]=f64::powf(self.scalar_static_f64[89],self.scalar_static_f64[451]);
        self.scalar_static_f64[453]=(self.scalar_static_f64[450]/self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(1.0+self.scalar_static_f64[453]);
        self.scalar_static_f64[455]=(self.scalar_static_f64[449]*self.scalar_static_f64[454]);
        self.scalar_static_f64[456]=p.p276;
        self.scalar_static_f64[457]=p.p277;
        self.scalar_static_f64[458]=p.p278;
        self.scalar_static_f64[459]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[458]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[457]/self.scalar_static_f64[459]);
        self.scalar_static_f64[461]=(1.0+self.scalar_static_f64[460]);
        self.scalar_static_f64[462]=(self.scalar_static_f64[456]*self.scalar_static_f64[461]);
        self.scalar_static_f64[463]=p.p281;
        self.scalar_static_f64[464]=p.p282;
        self.scalar_static_f64[465]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[464]);
        self.scalar_static_f64[466]=(self.scalar_static_f64[463]/self.scalar_static_f64[465]);
        self.scalar_static_f64[467]=(1.0+self.scalar_static_f64[466]);
        self.scalar_static_f64[468]=(self.scalar_static_f64[462]*self.scalar_static_f64[467]);
        self.scalar_static_f64[469]=p.p279;
        self.scalar_static_f64[470]=p.p280;
        self.scalar_static_f64[471]=f64::powf(self.scalar_static_f64[89],self.scalar_static_f64[470]);
        self.scalar_static_f64[472]=(self.scalar_static_f64[469]/self.scalar_static_f64[471]);
        self.scalar_static_f64[473]=(1.0+self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=(self.scalar_static_f64[468]*self.scalar_static_f64[473]);
        self.scalar_static_f64[475]=p.p163;
        self.scalar_static_f64[476]=(1.0+self.scalar_static_f64[475]);
        self.scalar_static_f64[477]=(1.0/self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(self.scalar_static_f64[11]/self.scalar_static_f64[291]);
        self.scalar_static_f64[479]=f64::powf(self.scalar_static_f64[478],self.scalar_static_f64[12]);
        self.scalar_static_f64[480]=p.p112;
        self.scalar_static_f64[481]=p.p113;
        self.scalar_static_f64[482]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[480]/self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=(1.0+self.scalar_static_f64[483]);
        self.scalar_static_f64[485]=p.p111;
        self.scalar_static_f64[486]=(self.scalar_static_f64[484]*self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=p.p253;
        self.scalar_static_f64[488]=p.p181;
        self.scalar_static_f64[489]=p.p182;
        self.scalar_static_f64[490]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[489]);
        self.scalar_static_f64[491]=(self.scalar_static_f64[488]/self.scalar_static_f64[490]);
        self.scalar_static_f64[492]=(1.0+self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=p.p185;
        self.scalar_static_f64[494]=p.p186;
        self.scalar_static_f64[495]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[494]);
        self.scalar_static_f64[496]=(self.scalar_static_f64[493]/self.scalar_static_f64[495]);
        self.scalar_static_f64[497]=(1.0+self.scalar_static_f64[496]);
        self.scalar_static_f64[498]=(self.scalar_static_f64[492]*self.scalar_static_f64[497]);
        self.scalar_static_f64[499]=p.p187;
        self.scalar_static_f64[500]=p.p188;
        self.scalar_static_f64[501]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[500]);
        self.scalar_static_f64[502]=(self.scalar_static_f64[499]/self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(1.0+self.scalar_static_f64[502]);
        self.scalar_static_f64[504]=(self.scalar_static_f64[498]*self.scalar_static_f64[503]);
        self.scalar_static_f64[505]=p.p183;
        self.scalar_static_f64[506]=p.p184;
        self.scalar_static_f64[507]=f64::powf(self.scalar_static_f64[89],self.scalar_static_f64[506]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[505]/self.scalar_static_f64[507]);
        self.scalar_static_f64[509]=(1.0+self.scalar_static_f64[508]);
        self.scalar_static_f64[510]=(self.scalar_static_f64[504]*self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(self.scalar_static_f64[510]*self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[511]+4e-6);
        self.scalar_static_f64[513]=(self.scalar_static_f64[512]).sqrt();
        self.scalar_static_f64[514]=(self.scalar_static_f64[510]+self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(0.5*self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[515]+1e-13);
        self.scalar_static_bool[33]=(self.scalar_static_f64[516]<0.0);
        self.scalar_static_f64[517]=(if self.scalar_static_bool[33]{0.0}else{self.scalar_static_f64[516]});
        self.scalar_static_f64[518]=p.p102;
        self.scalar_static_f64[519]=p.p103;
        self.scalar_static_f64[520]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[519]);
        self.scalar_static_f64[521]=(self.scalar_static_f64[518]/self.scalar_static_f64[520]);
        self.scalar_static_f64[522]=(1.0+self.scalar_static_f64[521]);
        self.scalar_static_f64[523]=(self.scalar_static_f64[43]*self.scalar_static_f64[517]);
        self.scalar_static_f64[524]=(self.scalar_static_f64[47]*self.scalar_static_f64[522]);
        self.scalar_static_f64[525]=(self.scalar_static_f64[430]/2.0);
        self.scalar_static_f64[526]=(self.scalar_static_f64[435]*self.scalar_static_f64[525]);
        self.scalar_static_f64[527]=(self.scalar_static_f64[113]*3.2043836e-19);
        self.scalar_static_f64[528]=(1.034943e-10*self.scalar_static_f64[527]);
        self.scalar_static_f64[529]=(self.scalar_static_f64[528]).sqrt();
        self.scalar_static_f64[530]=(self.scalar_static_f64[113]*self.scalar_static_f64[113]);
        self.scalar_static_f64[531]=(1.0/self.scalar_static_f64[530]);
        self.scalar_static_f64[532]=p.p38;
        self.scalar_static_f64[533]=p.p251;
        self.scalar_static_f64[534]=p.p252;
        self.scalar_static_f64[535]=(self.scalar_static_f64[533]+self.scalar_static_f64[534]);
        self.scalar_static_f64[536]=(self.scalar_static_f64[532]/self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=(self.scalar_static_f64[83]*self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=(0.001*self.scalar_static_f64[532]);
        self.scalar_static_f64[539]=(self.scalar_static_f64[538]+2.2204460492503132e-17);
        self.scalar_static_f64[540]=(self.scalar_static_f64[539]).abs();
        self.scalar_static_bool[34]=(self.scalar_static_f64[532]>0.0);
        self.scalar_static_f64[541]=(self.scalar_static_f64[532]-self.scalar_static_f64[537]);
        self.scalar_static_f64[542]=(self.scalar_static_f64[541]-self.scalar_static_f64[540]);
        self.scalar_static_f64[543]=(4.0*self.scalar_static_f64[532]);
        self.scalar_static_f64[544]=(self.scalar_static_f64[540]*self.scalar_static_f64[543]);
        self.scalar_static_f64[545]=(if self.scalar_static_bool[34]{self.scalar_static_f64[544]}else{self.scalar_static_f64[513]});
        self.scalar_static_bool[35]=(self.scalar_static_f64[545]>0.0);
        self.scalar_static_f64[546]=(-self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=(if self.scalar_static_bool[35]{self.scalar_static_f64[545]}else{self.scalar_static_f64[546]});
        self.scalar_static_f64[548]=(if self.scalar_static_bool[34]{self.scalar_static_f64[547]}else{self.scalar_static_f64[545]});
        self.scalar_static_bool[36]=(!self.scalar_static_bool[34]);
        self.scalar_static_f64[549]=p.p49;
        self.scalar_static_f64[550]=(-self.scalar_static_f64[549]);
        self.scalar_static_f64[551]=p.p50;
        self.scalar_static_f64[552]=p.p51;
        self.scalar_static_f64[553]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=(self.scalar_static_f64[551]/self.scalar_static_f64[553]);
        self.scalar_static_f64[555]=(1.0+self.scalar_static_f64[554]);
        self.scalar_static_f64[556]=(self.scalar_static_f64[550]*self.scalar_static_f64[555]);
        self.scalar_static_f64[557]=p.p52;
        self.scalar_static_f64[558]=p.p53;
        self.scalar_static_f64[559]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[557]/self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=(1.0+self.scalar_static_f64[560]);
        self.scalar_static_f64[562]=(self.scalar_static_f64[550]*self.scalar_static_f64[561]);
        self.scalar_static_f64[563]=p.p54;
        self.scalar_static_f64[564]=(self.scalar_static_f64[87]*self.scalar_static_f64[563]);
        self.scalar_static_f64[565]=(self.scalar_static_f64[549]+self.scalar_static_f64[564]);
        self.scalar_static_f64[566]=(-self.scalar_static_f64[565]);
        self.scalar_static_f64[567]=(self.scalar_static_f64[556]-self.scalar_static_f64[562]);
        self.scalar_static_f64[568]=(self.scalar_static_f64[567]-1e-12);
        self.scalar_static_f64[569]=(4.0*self.scalar_static_f64[562]);
        self.scalar_static_f64[570]=(1e-12*self.scalar_static_f64[569]);
        self.scalar_static_bool[37]=(self.scalar_static_f64[570]>0.0);
        self.scalar_static_f64[571]=(-self.scalar_static_f64[570]);
        self.scalar_static_f64[572]=(if self.scalar_static_bool[37]{self.scalar_static_f64[570]}else{self.scalar_static_f64[571]});
        self.scalar_static_f64[573]=(self.scalar_static_f64[568]*self.scalar_static_f64[568]);
        self.scalar_static_f64[574]=(self.scalar_static_f64[572]+self.scalar_static_f64[573]);
        self.scalar_static_f64[575]=(self.scalar_static_f64[574]).sqrt();
        self.scalar_static_f64[576]=(self.scalar_static_f64[568]+self.scalar_static_f64[575]);
        self.scalar_static_f64[577]=(0.5*self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=(self.scalar_static_f64[562]+self.scalar_static_f64[577]);
        self.scalar_static_f64[579]=(self.scalar_static_f64[578]-self.scalar_static_f64[566]);
        self.scalar_static_f64[580]=(self.scalar_static_f64[579]-1e-12);
        self.scalar_static_f64[581]=(4.0*self.scalar_static_f64[566]);
        self.scalar_static_f64[582]=(1e-12*self.scalar_static_f64[581]);
        self.scalar_static_bool[38]=(self.scalar_static_f64[582]>0.0);
        self.scalar_static_f64[583]=(-self.scalar_static_f64[582]);
        self.scalar_static_f64[584]=(if self.scalar_static_bool[38]{self.scalar_static_f64[582]}else{self.scalar_static_f64[583]});
        self.scalar_static_f64[585]=(self.scalar_static_f64[580]*self.scalar_static_f64[580]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[584]+self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=(self.scalar_static_f64[586]).sqrt();
        self.scalar_static_f64[588]=(self.scalar_static_f64[580]+self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(0.5*self.scalar_static_f64[588]);
        self.scalar_static_f64[590]=(self.scalar_static_f64[566]+self.scalar_static_f64[589]);
        self.scalar_static_f64[591]=(-self.scalar_static_f64[590]);
        self.scalar_static_f64[592]=(2.0*self.scalar_static_f64[319]);
        self.scalar_static_f64[593]=p.p226;
        self.scalar_static_f64[594]=(3.453133e-11/self.scalar_static_f64[593]);
        self.scalar_static_f64[595]=(self.scalar_static_f64[593]/3.453133e-11);
        self.scalar_static_f64[596]=p.p229;
        self.scalar_static_f64[597]=(3.453133e-11/self.scalar_static_f64[596]);
        self.scalar_static_f64[598]=(self.scalar_static_f64[596]/3.453133e-11);
        self.scalar_static_f64[599]=(1.034943e-10/self.scalar_static_f64[15]);
        self.scalar_static_f64[600]=(1.0/self.scalar_static_f64[599]);
        self.scalar_static_f64[601]=(self.scalar_static_f64[598]+self.scalar_static_f64[600]);
        self.scalar_static_f64[602]=p.p254;
        self.scalar_static_f64[603]=p.p255;
        self.scalar_static_f64[604]=(0.5*self.scalar_static_f64[603]);
        self.scalar_static_bool[39]=(self.scalar_static_f64[602]>self.scalar_static_f64[604]);
        self.scalar_static_f64[605]=(if self.scalar_static_bool[39]{self.scalar_static_f64[604]}else{self.scalar_static_f64[602]});
        self.scalar_static_f64[606]=(self.scalar_static_f64[603]-self.scalar_static_f64[605]);
        self.scalar_static_f64[607]=p.p216;
        self.scalar_static_f64[608]=p.p193;
        self.scalar_static_bool[40]=(0.0==self.scalar_static_f64[608]);
        self.scalar_static_f64[609]=p.p195;
        self.scalar_static_bool[41]=(0.0==self.scalar_static_f64[609]);
        self.scalar_static_bool[42]=(self.scalar_static_bool[40]&&self.scalar_static_bool[41]);
        self.scalar_static_f64[610]=p.p194;
        self.scalar_static_bool[43]=(0.0==self.scalar_static_f64[610]);
        self.scalar_static_bool[44]=(self.scalar_static_bool[42]||self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=(!self.scalar_static_bool[44]);
        self.scalar_static_f64[611]=(if self.scalar_static_bool[45]{1.0}else{0.0});
        self.scalar_static_bool[46]=(0.0==self.scalar_static_f64[611]);
        self.scalar_static_f64[612]=(if self.scalar_static_bool[46]{self.scalar_static_f64[593]}else{0.0});
        self.scalar_static_f64[613]=(if self.scalar_static_bool[46]{self.scalar_static_f64[594]}else{0.0});
        self.scalar_static_f64[614]=(if self.scalar_static_bool[46]{self.scalar_static_f64[595]}else{0.0});
        self.scalar_static_bool[47]=(!self.scalar_static_bool[46]);
        self.scalar_static_bool[48]=(0.0!=self.scalar_static_f64[315]);
        self.scalar_static_f64[615]=(2.0*self.scalar_static_f64[15]);
        self.scalar_static_f64[616]=(self.scalar_static_f64[315]*self.scalar_static_f64[315]);
        self.scalar_static_f64[617]=(self.scalar_static_f64[615]/self.scalar_static_f64[616]);
        self.scalar_static_f64[618]=p.p55;
        self.scalar_static_f64[619]=p.p66;
        self.scalar_static_f64[620]=p.p68;
        self.scalar_static_f64[621]=(self.scalar_static_f64[620]/self.scalar_static_f64[315]);
        self.scalar_static_f64[622]=p.p67;
        self.scalar_static_bool[49]=(!self.scalar_static_bool[48]);
        self.scalar_static_f64[623]=p.p297;
        self.scalar_static_bool[50]=(0.0!=self.scalar_static_f64[623]);
        self.scalar_static_f64[624]=p.p57;
        self.scalar_static_f64[625]=(self.scalar_static_f64[83]-self.scalar_static_f64[624]);
        self.scalar_static_f64[626]=(self.scalar_static_f64[625]*self.scalar_static_f64[625]);
        self.scalar_static_f64[627]=p.p69;
        self.scalar_static_f64[628]=p.p71;
        self.scalar_static_f64[629]=(self.scalar_static_f64[628]/self.scalar_static_f64[83]);
        self.scalar_static_f64[630]=p.p70;
        self.scalar_static_f64[631]=p.p250;
        self.scalar_static_f64[632]=p.p72;
        self.scalar_static_bool[51]=(self.scalar_static_f64[632]>0.0);
        self.scalar_static_f64[633]=p.p74;
        self.scalar_static_f64[634]=(2.0*self.scalar_static_f64[633]);
        self.scalar_static_f64[635]=p.p73;
        self.scalar_static_f64[636]=p.p56;
        self.scalar_static_f64[637]=(self.scalar_static_f64[284]+self.scalar_static_f64[636]);
        self.scalar_static_f64[638]=(self.scalar_static_f64[15]*self.scalar_static_f64[632]);
        self.scalar_static_bool[52]=(!self.scalar_static_bool[51]);
        self.scalar_static_f64[639]=(self.scalar_static_f64[72]/self.scalar_static_f64[132]);
        self.scalar_static_f64[640]=p.p104;
        self.scalar_static_f64[641]=(self.scalar_static_f64[640]/self.scalar_static_f64[88]);
        self.scalar_static_bool[53]=(0.0==self.scalar_static_f64[341]);
        self.scalar_static_f64[642]=(if self.scalar_static_bool[53]{0.0}else{1.0});
        self.scalar_static_bool[54]=(!self.scalar_static_bool[53]);
        self.scalar_static_f64[643]=(if self.scalar_static_bool[54]{1.0}else{self.scalar_static_f64[642]});
        self.scalar_static_bool[55]=(0.0==self.scalar_static_f64[643]);
        self.scalar_static_bool[56]=(!self.scalar_static_bool[55]);
        self.scalar_static_f64[644]=p.p76;
        self.scalar_static_f64[645]=p.p29;
        self.scalar_static_bool[57]=(!(self.scalar_static_f64[645]!=0.0));
        self.scalar_static_f64[646]=(self.scalar_static_f64[15]*0.99);
        self.scalar_static_f64[647]=(1.0/self.scalar_static_f64[597]);
        self.scalar_static_f64[648]=(0.5*self.scalar_static_f64[600]);
        self.scalar_static_f64[649]=p.p298;
        self.scalar_static_f64[650]=(self.scalar_static_f64[276]*3.3163543761348e-29);
        self.scalar_static_f64[651]=(self.scalar_static_f64[597]/self.scalar_static_f64[650]);
        self.scalar_static_f64[652]=(self.scalar_static_f64[597]*self.scalar_static_f64[600]);
        self.scalar_static_f64[653]=(1.0+self.scalar_static_f64[652]);
        self.scalar_static_f64[654]=(2.0*self.scalar_static_f64[597]);
        self.scalar_static_f64[655]=(self.scalar_static_f64[597]*self.scalar_static_f64[597]);
        self.scalar_static_f64[656]=(self.scalar_static_f64[15]/1.034943e-10);
        self.scalar_static_f64[657]=(0.5*self.scalar_static_f64[656]);
        self.scalar_static_f64[658]=(self.scalar_static_f64[647]+self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=p.p15;
        self.scalar_static_bool[58]=(1.0==self.scalar_static_f64[659]);
        self.scalar_static_f64[660]=p.p136;
        self.scalar_static_f64[661]=(2.0*self.scalar_static_f64[132]);
        self.scalar_static_f64[662]=(100.0*self.scalar_static_f64[593]);
        self.scalar_static_f64[663]=(self.scalar_static_f64[136]*100.0);
        self.scalar_static_f64[664]=p.p26;
        self.scalar_static_bool[59]=(0.0==self.scalar_static_f64[664]);
        self.scalar_static_bool[60]=(!self.scalar_static_bool[59]);
        self.scalar_static_f64[665]=p.p141;
        self.scalar_static_f64[666]=(1.6021918e-19*self.scalar_static_f64[665]);
        self.scalar_static_f64[667]=p.p144;
        self.scalar_static_f64[668]=p.p143;
        self.scalar_static_f64[669]=p.p142;
        self.scalar_static_bool[61]=(self.scalar_static_f64[397]<=0.0);
        self.scalar_static_f64[670]=p.p123;
        self.scalar_static_f64[671]=(self.scalar_static_f64[380]*self.scalar_static_f64[390]);
        self.scalar_static_f64[672]=(-self.scalar_static_f64[402]);
        self.scalar_static_f64[673]=p.p16;
        self.scalar_static_bool[62]=(1.0==self.scalar_static_f64[673]);
        self.scalar_static_f64[674]=(self.scalar_static_f64[15]*1.6021918e-19);
        self.scalar_static_f64[675]=(self.scalar_static_f64[136]*self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=p.p140;
        self.scalar_static_f64[677]=p.p139;
        self.scalar_static_f64[678]=p.p27;
        self.scalar_static_f64[679]=p.p137;
        self.scalar_static_f64[680]=p.p138;
        self.scalar_static_bool[63]=(!self.scalar_static_bool[62]);
        self.scalar_static_f64[681]=(self.scalar_static_f64[351]-1.0);
        self.scalar_static_f64[682]=(1.0/self.scalar_static_f64[351]);
        self.scalar_static_f64[683]=(self.scalar_static_f64[682]-1.0);
        self.scalar_static_bool[64]=(self.scalar_static_f64[41]<2.220446049250313e-15);
        self.scalar_static_f64[684]=p.p178;
        self.scalar_static_bool[65]=(self.scalar_static_f64[684]<2.220446049250313e-15);
        self.scalar_static_bool[66]=(self.scalar_static_bool[64]&&self.scalar_static_bool[65]);
        self.scalar_static_bool[67]=(!self.scalar_static_bool[66]);
        self.scalar_static_f64[685]=p.p176;
        self.scalar_static_f64[686]=(1.0-self.scalar_static_f64[685]);
        self.scalar_static_f64[687]=(-self.scalar_static_f64[137]);
        self.scalar_static_f64[688]=p.p217;
        self.scalar_static_f64[689]=(100.0*self.scalar_static_f64[596]);
        self.scalar_static_f64[690]=p.p81;
        self.scalar_static_f64[691]=p.p82;
        self.scalar_static_f64[692]=p.p83;
        self.scalar_static_f64[693]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(self.scalar_static_f64[691]/self.scalar_static_f64[693]);
        self.scalar_static_f64[695]=(1.0+self.scalar_static_f64[694]);
        self.scalar_static_f64[696]=(self.scalar_static_f64[690]*self.scalar_static_f64[695]);
        self.scalar_static_f64[697]=(self.scalar_static_f64[696]/1.034943e-12);
        self.scalar_static_f64[698]=p.p78;
        self.scalar_static_f64[699]=p.p79;
        self.scalar_static_f64[700]=p.p80;
        self.scalar_static_f64[701]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[700]);
        self.scalar_static_f64[702]=(self.scalar_static_f64[699]/self.scalar_static_f64[701]);
        self.scalar_static_f64[703]=(1.0+self.scalar_static_f64[702]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[698]*self.scalar_static_f64[703]);
        self.scalar_static_f64[705]=(self.scalar_static_f64[704]/1.034943e-12);
        self.scalar_static_f64[706]=(self.scalar_static_f64[607]).sqrt();
        self.scalar_static_f64[707]=p.p85;
        self.scalar_static_f64[708]=p.p84;
        self.scalar_static_f64[709]=p.p299;
        self.scalar_static_f64[710]=p.p300;
        self.scalar_static_f64[711]=p.p301;
        self.scalar_static_f64[712]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[711]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[710]/self.scalar_static_f64[712]);
        self.scalar_static_f64[714]=(1.0+self.scalar_static_f64[713]);
        self.scalar_static_f64[715]=(self.scalar_static_f64[709]*self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(self.scalar_static_f64[689]*11.7);
        self.scalar_static_bool[68]=(!(self.scalar_static_f64[254]!=0.0));
        self.scalar_static_f64[717]=p.p94;
        self.scalar_static_f64[718]=p.p105;
        self.scalar_static_f64[719]=p.p302;
        self.scalar_static_f64[720]=(self.scalar_static_f64[714]*self.scalar_static_f64[719]);
        self.scalar_static_f64[721]=(if self.scalar_static_bool[68]{self.scalar_static_f64[720]}else{0.0});
        self.scalar_static_f64[722]=p.p275;
        self.scalar_static_f64[723]=p.p284;
        self.scalar_static_f64[724]=p.p114;
        self.scalar_static_bool[69]=(0.9999999999999978<=self.scalar_static_f64[724]);
        self.scalar_static_bool[70]=(self.scalar_static_f64[724]<=1.0000000000000022);
        self.scalar_static_bool[71]=(self.scalar_static_bool[69]&&self.scalar_static_bool[70]);
        self.scalar_static_bool[72]=(1.9999999999999978<=self.scalar_static_f64[724]);
        self.scalar_static_bool[73]=(self.scalar_static_f64[724]<=2.000000000000002);
        self.scalar_static_bool[74]=(self.scalar_static_bool[72]&&self.scalar_static_bool[73]);
        self.scalar_static_bool[75]=(!self.scalar_static_bool[71]);
        self.scalar_static_bool[76]=(self.scalar_static_bool[74]&&self.scalar_static_bool[75]);
        self.scalar_static_bool[77]=(!self.scalar_static_bool[74]);
        self.scalar_static_bool[78]=(self.scalar_static_bool[75]&&self.scalar_static_bool[77]);
        self.scalar_static_f64[725]=(self.scalar_static_f64[724]-1.0);
        self.scalar_static_f64[726]=(-1.0/self.scalar_static_f64[724]);
        self.scalar_static_f64[727]=(self.scalar_static_f64[726]-1.0);
        self.scalar_static_f64[728]=p.p240;
        self.scalar_static_f64[729]=p.p241;
        self.scalar_static_bool[79]=(!self.scalar_static_bool[0]);
        self.scalar_static_bool[80]=(0.0!=self.scalar_static_f64[329]);
        self.scalar_static_bool[81]=(!self.scalar_static_bool[80]);
        self.scalar_static_f64[730]=p.p245;
        self.scalar_static_f64[731]=(-self.scalar_static_f64[730]);
        self.scalar_static_f64[732]=p.p22;
        self.scalar_static_bool[82]=(0.0!=self.scalar_static_f64[732]);
        self.scalar_static_f64[733]=(self.scalar_static_f64[94]-self.scalar_static_f64[624]);
        self.scalar_static_f64[734]=p.p158;
        self.scalar_static_f64[735]=p.p159;
        self.scalar_static_f64[736]=p.p160;
        self.scalar_static_f64[737]=p.p161;
        self.scalar_static_f64[738]=(4.0*self.scalar_static_f64[660]);
        self.scalar_static_f64[739]=(if self.scalar_static_bool[82]{1e50}else{0.0});
        self.scalar_static_f64[740]=(self.scalar_static_f64[85]*self.scalar_static_f64[127]);
        self.scalar_static_f64[741]=p.p20;
        self.scalar_static_bool[83]=(0.0!=self.scalar_static_f64[741]);
        self.scalar_static_f64[742]=p.p23;
        self.scalar_static_bool[84]=(0.0!=self.scalar_static_f64[742]);
        self.scalar_static_bool[85]=(self.scalar_static_bool[83]&&self.scalar_static_bool[84]);
        self.scalar_static_f64[743]=p.p145;
        self.scalar_static_bool[86]=(0.0!=self.scalar_static_f64[743]);
        self.scalar_static_f64[744]=p.p146;
        self.scalar_static_bool[87]=(0.0==self.scalar_static_f64[7]);
        self.scalar_static_bool[88]=(!self.scalar_static_bool[87]);
        self.scalar_static_f64[745]=p.p256;
        self.scalar_static_f64[746]=(self.scalar_static_f64[591]*self.scalar_static_f64[745]);
        self.scalar_static_f64[747]=p.p258;
        self.scalar_static_f64[748]=(-self.scalar_static_f64[747]);
        self.scalar_static_f64[749]=p.p206;
        self.scalar_static_f64[750]=p.p205;
        self.scalar_static_f64[751]=p.p209;
        self.scalar_static_f64[752]=p.p208;
        self.scalar_static_f64[753]=p.p204;
        self.scalar_static_f64[754]=(-self.scalar_static_f64[753]);
        self.scalar_static_f64[755]=p.p203;
        self.scalar_static_f64[756]=p.p257;
        self.scalar_static_f64[757]=p.p211;
        self.scalar_static_f64[758]=(-self.scalar_static_f64[757]);
        self.scalar_static_f64[759]=p.p212;
        self.scalar_static_f64[760]=p.p260;
        self.scalar_static_f64[761]=(1.0/self.scalar_static_f64[662]);
        self.scalar_static_f64[762]=(self.scalar_static_f64[761]/self.scalar_static_f64[662]);
        self.scalar_static_f64[763]=p.p210;
        self.scalar_static_f64[764]=(self.scalar_static_f64[763]/1000000.0);
        self.scalar_static_f64[765]=(self.scalar_static_f64[663]*self.scalar_static_f64[764]);
        self.scalar_static_f64[766]=p.p259;
        self.scalar_static_f64[767]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[766]);
        self.scalar_static_f64[768]=(self.scalar_static_f64[765]*self.scalar_static_f64[767]);
        self.scalar_static_f64[769]=p.p261;
        self.scalar_static_f64[770]=p.p215;
        self.scalar_static_f64[771]=p.p214;
        self.scalar_static_f64[772]=(-self.scalar_static_f64[771]);
        self.scalar_static_f64[773]=p.p263;
        self.scalar_static_f64[774]=p.p264;
        self.scalar_static_f64[775]=(self.scalar_static_f64[87]+self.scalar_static_f64[774]);
        self.scalar_static_f64[776]=p.p265;
        self.scalar_static_f64[777]=(4.0*self.scalar_static_f64[776]);
        self.scalar_static_f64[778]=p.p213;
        self.scalar_static_f64[779]=p.p262;
        self.scalar_static_f64[780]=p.p269;
        self.scalar_static_f64[781]=p.p268;
        self.scalar_static_f64[782]=p.p267;
        self.scalar_static_f64[783]=(-self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=p.p271;
        self.scalar_static_f64[785]=p.p272;
        self.scalar_static_f64[786]=(self.scalar_static_f64[87]+self.scalar_static_f64[785]);
        self.scalar_static_f64[787]=p.p273;
        self.scalar_static_f64[788]=(4.0*self.scalar_static_f64[787]);
        self.scalar_static_f64[789]=p.p266;
        self.scalar_static_f64[790]=p.p270;
        self.scalar_static_f64[791]=(if self.scalar_static_bool[88]{0.5}else{0.0});
        self.scalar_static_bool[89]=(0.0==self.scalar_static_f64[9]);
        self.scalar_static_bool[90]=(!self.scalar_static_bool[89]);
        self.scalar_static_f64[792]=p.p198;
        self.scalar_static_f64[793]=p.p199;
        self.scalar_static_f64[794]=p.p200;
        self.scalar_static_f64[795]=(-self.scalar_static_f64[66]);
        self.scalar_static_f64[796]=p.p45;
        self.scalar_static_bool[91]=(0.0!=self.scalar_static_f64[796]);
        self.scalar_static_f64[797]=(1.0/self.scalar_static_f64[594]);
        self.scalar_static_f64[798]=p.p19;
        self.scalar_static_bool[92]=(self.scalar_static_f64[798]>=1.0);
        self.scalar_static_f64[799]=p.p175;
        self.scalar_static_bool[93]=(self.scalar_static_f64[799]>0.0);
        self.scalar_static_bool[94]=(self.scalar_static_bool[92]&&self.scalar_static_bool[93]);
        self.scalar_static_bool[95]=(self.scalar_static_f64[62]>0.0);
        self.scalar_static_bool[96]=(self.scalar_static_bool[94]&&self.scalar_static_bool[95]);
        self.scalar_static_f64[800]=(if self.scalar_static_bool[96]{self.scalar_static_f64[799]}else{0.0});
        self.scalar_static_f64[801]=(if self.scalar_static_bool[96]{1.0}else{0.0});
        self.scalar_static_bool[97]=(self.scalar_static_bool[96]&&(self.scalar_static_f64[801]!=0.0));
        self.scalar_static_bool[98]=((0.0!=0.0)&&self.scalar_static_bool[96]);
        self.scalar_static_f64[802]=p.p39;
        self.scalar_static_f64[803]=p.p30;
        self.scalar_static_bool[99]=(self.scalar_static_f64[803]>0.0);
        self.scalar_static_bool[100]=(1.0==self.scalar_static_f64[803]);
        self.scalar_static_f64[804]=(self.scalar_static_f64[137]*self.scalar_static_f64[800]);
        self.scalar_static_f64[805]=(if self.scalar_static_bool[96]{0.0}else{self.scalar_static_f64[801]});
        self.scalar_static_bool[101]=(self.scalar_static_bool[96]&&(self.scalar_static_f64[805]!=0.0));
        self.scalar_static_f64[806]=p.p174;
        self.scalar_static_f64[807]=p.p173;
        self.scalar_static_bool[102]=(!(self.scalar_static_f64[2]!=0.0));
        self.scalar_static_bool[103]=(!(self.scalar_static_f64[3]!=0.0));
        self.scalar_static_bool[104]=(!self.scalar_static_bool[96]);
        self.scalar_static_f64[808]=(-self.scalar_static_f64[594]);
        self.scalar_static_f64[809]=(self.scalar_static_f64[799]*self.scalar_static_f64[808]);
        self.scalar_static_f64[810]=(self.scalar_static_f64[137]*self.scalar_static_f64[809]);
        self.scalar_static_bool[105]=(!self.scalar_static_bool[93]);
        self.scalar_static_f64[811]=p.p223;
        self.scalar_static_f64[812]=p.p224;
        self.scalar_static_f64[813]=(self.scalar_static_f64[811]*self.scalar_static_f64[812]);
        self.scalar_static_f64[814]=(1e-50+self.scalar_static_f64[811]);
        self.scalar_static_f64[815]=p.p225;
        self.scalar_static_f64[816]=p.p21;
        self.scalar_static_bool[106]=(0.0!=self.scalar_static_f64[816]);
        self.scalar_static_f64[817]=p.p172;
        self.scalar_static_f64[818]=(-self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[83]*self.scalar_static_f64[818]);
        self.scalar_static_f64[820]=(if (self.scalar_static_f64[1]!=0.0){self.scalar_static_f64[819]}else{0.0});
        self.scalar_static_bool[107]=(!(self.scalar_static_f64[1]!=0.0));
        self.scalar_static_f64[821]=(self.scalar_static_f64[137]*2.1983327444149834e-11);
        self.scalar_static_f64[822]=(0.0*self.scalar_static_f64[821]);
        self.scalar_static_f64[823]=(0.1*self.scalar_static_f64[597]);
        self.scalar_static_f64[824]=p.p303;
        self.scalar_static_bool[108]=(!(self.scalar_static_f64[824]!=0.0));
        self.scalar_static_bool[109]=(0.0==self.scalar_static_f64[796]);
        self.scalar_static_bool[110]=(!self.scalar_static_bool[109]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[137]*1.034943e-10);
        self.scalar_static_bool[111]=(0.0!=self.scalar_static_f64[404]);
        self.scalar_static_f64[826]=p.p14;
        self.scalar_static_bool[112]=(1.0==self.scalar_static_f64[826]);
        self.scalar_static_f64[827]=(1.0-self.scalar_static_f64[791]);
        self.scalar_static_f64[828]=p.p312;
        self.scalar_static_bool[113]=(1.0==self.scalar_static_f64[828]);
        self.scalar_static_f64[829]=p.p315;
        self.scalar_static_f64[830]=(self.scalar_static_f64[829]/1e-6);
        self.scalar_static_f64[831]=(if self.scalar_static_bool[113]{self.scalar_static_f64[830]}else{0.0});
        self.scalar_static_f64[832]=p.p317;
        self.scalar_static_f64[833]=(if self.scalar_static_bool[113]{self.scalar_static_f64[832]}else{0.0});
        self.scalar_static_f64[834]=p.p319;
        self.scalar_static_f64[835]=(if self.scalar_static_bool[113]{self.scalar_static_f64[834]}else{0.0});
        self.scalar_static_f64[836]=p.p324;
        self.scalar_static_f64[837]=(if self.scalar_static_bool[113]{self.scalar_static_f64[836]}else{0.0});
        self.scalar_static_f64[838]=p.p314;
        self.scalar_static_bool[114]=(self.scalar_static_f64[838]>0.0);
        self.scalar_static_f64[839]=p.p308;
        self.scalar_static_f64[840]=(self.scalar_static_f64[838]*self.scalar_static_f64[839]);
        self.scalar_static_f64[841]=(if self.scalar_static_bool[114]{self.scalar_static_f64[840]}else{0.0});
        self.scalar_static_f64[842]=(if self.scalar_static_bool[113]{self.scalar_static_f64[841]}else{0.0});
        self.scalar_static_f64[843]=p.p311;
        self.scalar_static_f64[844]=(if self.scalar_static_bool[113]{self.scalar_static_f64[843]}else{0.0});
        self.scalar_static_f64[845]=p.p322;
        self.scalar_static_f64[846]=(self.scalar_static_f64[845]*self.scalar_static_f64[845]);
        self.scalar_static_f64[847]=(self.scalar_static_f64[532]*self.scalar_static_f64[532]);
        self.scalar_static_f64[848]=(self.scalar_static_f64[846]+self.scalar_static_f64[847]);
        self.scalar_static_f64[849]=(self.scalar_static_f64[848]).sqrt();
        self.scalar_static_f64[850]=(if self.scalar_static_bool[113]{self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[851]=(if self.scalar_static_bool[113]{self.scalar_static_f64[136]}else{0.0});
        self.scalar_static_f64[852]=(self.scalar_static_f64[833]/10000.0);
        self.scalar_static_f64[853]=(if self.scalar_static_bool[113]{self.scalar_static_f64[852]}else{self.scalar_static_f64[833]});
        self.scalar_static_f64[854]=(self.scalar_static_f64[835]/100.0);
        self.scalar_static_f64[855]=(if self.scalar_static_bool[113]{self.scalar_static_f64[854]}else{self.scalar_static_f64[835]});
        self.scalar_static_f64[856]=p.p320;
        self.scalar_static_f64[857]=p.p321;
        self.scalar_static_f64[858]=p.p325;
        self.scalar_static_f64[859]=p.p330;
        self.scalar_static_f64[860]=p.p331;
        self.scalar_static_f64[861]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[860]);
        self.scalar_static_f64[862]=(self.scalar_static_f64[859]/self.scalar_static_f64[861]);
        self.scalar_static_f64[863]=(1.0+self.scalar_static_f64[862]);
        self.scalar_static_f64[864]=(if self.scalar_static_bool[113]{self.scalar_static_f64[863]}else{0.0});
        self.scalar_static_f64[865]=p.p328;
        self.scalar_static_f64[866]=p.p329;
        self.scalar_static_f64[867]=f64::powf(self.scalar_static_f64[87],self.scalar_static_f64[866]);
        self.scalar_static_f64[868]=(self.scalar_static_f64[865]/self.scalar_static_f64[867]);
        self.scalar_static_f64[869]=(1.0+self.scalar_static_f64[868]);
        self.scalar_static_f64[870]=(if self.scalar_static_bool[113]{self.scalar_static_f64[869]}else{0.0});
        self.scalar_static_f64[871]=p.p326;
        self.scalar_static_f64[872]=p.p327;
        self.scalar_static_f64[873]=f64::powf(self.scalar_static_f64[88],self.scalar_static_f64[872]);
        self.scalar_static_f64[874]=(self.scalar_static_f64[871]/self.scalar_static_f64[873]);
        self.scalar_static_f64[875]=(1.0+self.scalar_static_f64[874]);
        self.scalar_static_f64[876]=(if self.scalar_static_bool[113]{self.scalar_static_f64[875]}else{0.0});
        self.scalar_static_f64[877]=(1.6021918e-19/self.scalar_static_f64[844]);
        self.scalar_static_f64[878]=p.p313;
        self.scalar_static_bool[115]=(1.0==self.scalar_static_f64[878]);
        self.scalar_static_f64[879]=(if self.scalar_static_bool[115]{self.scalar_static_f64[62]}else{0.0});
        self.scalar_static_f64[880]=p.p316;
        self.scalar_static_f64[881]=(if self.scalar_static_bool[115]{self.scalar_static_f64[880]}else{0.0});
        self.scalar_static_f64[882]=p.p318;
        self.scalar_static_f64[883]=(if self.scalar_static_bool[115]{self.scalar_static_f64[882]}else{0.0});
        self.scalar_static_f64[884]=p.p323;
        self.scalar_static_f64[885]=(if self.scalar_static_bool[115]{self.scalar_static_f64[884]}else{0.0});
        self.scalar_static_f64[886]=p.p309;
        self.scalar_static_f64[887]=(self.scalar_static_f64[838]*self.scalar_static_f64[886]);
        self.scalar_static_f64[888]=(if self.scalar_static_bool[114]{self.scalar_static_f64[887]}else{0.0});
        self.scalar_static_f64[889]=(if self.scalar_static_bool[115]{self.scalar_static_f64[888]}else{0.0});
        self.scalar_static_f64[890]=p.p310;
        self.scalar_static_f64[891]=(if self.scalar_static_bool[115]{self.scalar_static_f64[890]}else{0.0});
        self.scalar_static_f64[892]=(if self.scalar_static_bool[115]{self.scalar_static_f64[849]}else{0.0});
        self.scalar_static_f64[893]=(if self.scalar_static_bool[115]{self.scalar_static_f64[136]}else{0.0});
        self.scalar_static_f64[894]=(self.scalar_static_f64[881]/10000.0);
        self.scalar_static_f64[895]=(if self.scalar_static_bool[115]{self.scalar_static_f64[894]}else{self.scalar_static_f64[881]});
        self.scalar_static_f64[896]=(self.scalar_static_f64[883]/100.0);
        self.scalar_static_f64[897]=(if self.scalar_static_bool[115]{self.scalar_static_f64[896]}else{self.scalar_static_f64[883]});
        self.scalar_static_f64[898]=(if self.scalar_static_bool[115]{self.scalar_static_f64[863]}else{0.0});
        self.scalar_static_f64[899]=(if self.scalar_static_bool[115]{self.scalar_static_f64[869]}else{0.0});
        self.scalar_static_f64[900]=(if self.scalar_static_bool[115]{self.scalar_static_f64[875]}else{0.0});
        self.scalar_static_f64[901]=(1.6021918e-19/self.scalar_static_f64[891]);
        self.scalar_static_f64[902]=(if self.scalar_static_bool[30]{self.scalar_static_f64[139]}else{0.0});
        self.scalar_static_f64[903]=(1.0/self.scalar_static_f64[138]);
        self.scalar_static_f64[904]=(if self.scalar_static_bool[30]{self.scalar_static_f64[903]}else{0.0});
        self.scalar_static_f64[905]=(if self.scalar_static_bool[31]{0.0}else{self.scalar_static_f64[902]});
        self.scalar_static_f64[906]=(if self.scalar_static_bool[31]{0.0}else{self.scalar_static_f64[904]});
        self.scalar_static_bool[116]=((self.scalar_static_f64[659]!=0.0)&&(self.scalar_static_f64[678]!=0.0));
        self.scalar_static_bool[117]=((self.scalar_static_f64[673]!=0.0)&&self.scalar_static_bool[116]);
        self.scalar_static_f64[907]=(self.scalar_static_f64[307]-1.0);
        self.scalar_static_f64[908]=(-self.scalar_static_f64[423]);
        self.scalar_static_f64[909]=(if (self.scalar_static_f64[0]!=0.0){1e-9}else{0.0});
        self.scalar_static_f64[910]=(if self.scalar_static_bool[32]{0.0}else{self.scalar_static_f64[909]});
        self.scalar_static_f64[911]=(self.scalar_static_f64[908]-self.scalar_static_f64[908]);
        self.scalar_static_f64[912]=(self.scalar_static_f64[436]-1.0);
        self.scalar_static_f64[913]=(self.scalar_static_f64[12]-1.0);
        self.scalar_static_f64[914]=(self.scalar_static_f64[681]-1.0);
        self.scalar_static_f64[915]=(self.scalar_static_f64[683]-1.0);
        self.scalar_static_f64[916]=(self.scalar_static_f64[707]-1.0);
        self.scalar_static_f64[917]=(self.scalar_static_f64[717]-1.0);
        self.scalar_static_f64[918]=(self.scalar_static_f64[221]-1.0);
        self.scalar_static_f64[919]=(self.scalar_static_f64[722]-1.0);
        self.scalar_static_f64[920]=(self.scalar_static_f64[234]-1.0);
        self.scalar_static_f64[921]=(self.scalar_static_f64[725]-1.0);
        self.scalar_static_f64[922]=(self.scalar_static_f64[727]-1.0);
        self.scalar_static_f64[923]=(self.scalar_static_f64[728]-1.0);
        self.scalar_static_f64[924]=(self.scalar_static_f64[756]-1.0);
        self.scalar_static_f64[925]=(self.scalar_static_f64[773]-1.0);
        self.scalar_static_f64[926]=(self.scalar_static_f64[779]-1.0);
        self.scalar_static_f64[927]=(self.scalar_static_f64[784]-1.0);
        self.scalar_static_f64[928]=(self.scalar_static_f64[790]-1.0);
        self.scalar_static_f64[929]=(if self.scalar_static_bool[113]{self.scalar_static_f64[908]}else{0.0});
        self.scalar_static_f64[930]=(if self.scalar_static_bool[113]{self.scalar_static_f64[423]}else{0.0});
        self.scalar_static_f64[931]=(self.scalar_static_f64[856]-1.0);
        self.scalar_static_f64[932]=(self.scalar_static_f64[929]/self.scalar_static_f64[844]);
        self.scalar_static_f64[933]=(self.scalar_static_f64[930]/self.scalar_static_f64[844]);
        self.scalar_static_f64[934]=(if self.scalar_static_bool[113]{self.scalar_static_f64[932]}else{0.0});
        self.scalar_static_f64[935]=(if self.scalar_static_bool[113]{self.scalar_static_f64[933]}else{0.0});
        self.scalar_static_f64[936]=(if self.scalar_static_bool[115]{self.scalar_static_f64[423]}else{0.0});
        self.scalar_static_f64[937]=(if self.scalar_static_bool[115]{self.scalar_static_f64[908]}else{0.0});
        self.scalar_static_f64[938]=(self.scalar_static_f64[936]/self.scalar_static_f64[891]);
        self.scalar_static_f64[939]=(self.scalar_static_f64[937]/self.scalar_static_f64[891]);
        self.scalar_static_f64[940]=(if self.scalar_static_bool[115]{self.scalar_static_f64[938]}else{0.0});
        self.scalar_static_f64[941]=(if self.scalar_static_bool[115]{self.scalar_static_f64[939]}else{0.0});
        self.scalar_static_f64[942]=(-self.scalar_static_f64[910]);
        self.scalar_static_f64[943]=(if (self.scalar_static_f64[0]!=0.0){self.scalar_static_f64[942]}else{0.0});
        self.scalar_static_f64[944]=(0.5*self.scalar_static_f64[910]);
        self.scalar_static_f64[945]=(if (self.scalar_static_f64[0]!=0.0){self.scalar_static_f64[944]}else{0.0});
        self.scalar_static_f64[946]=(if self.scalar_static_bool[32]{0.0}else{self.scalar_static_f64[945]});
        self.scalar_static_f64[947]=(if self.scalar_static_bool[32]{0.0}else{self.scalar_static_f64[943]});
        self.scalar_static_f64[948]=(if self.scalar_static_bool[32]{0.0}else{self.scalar_static_f64[910]});
        self.scalar_static_f64[949]=(-self.scalar_static_f64[375]);
        self.scalar_static_f64[950]=(if (self.scalar_static_f64[357]!=0.0){self.scalar_static_f64[375]}else{0.0});
        self.scalar_static_f64[951]=(if (self.scalar_static_f64[357]!=0.0){self.scalar_static_f64[949]}else{0.0});
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
        self.scalar_static_f64[952]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[80]}else{temperature});
        self.scalar_static_f64[953]=(self.scalar_static_f64[952]+self.scalar_static_f64[425]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
