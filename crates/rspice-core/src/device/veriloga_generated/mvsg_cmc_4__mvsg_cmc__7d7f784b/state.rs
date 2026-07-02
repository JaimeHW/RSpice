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
    pub p332: f64,
    pub p333: f64,
    pub p334: f64,
    pub p335: f64,
    pub p336: f64,
    pub p337: f64,
    pub p338: f64,
    pub p339: f64,
    pub p340: f64,
    pub p341: f64,
    pub p342: f64,
    pub p343: f64,
    pub p344: f64,
    pub p345: f64,
    pub p346: f64,
    pub p347: f64,
    pub p348: f64,
    pub p349: f64,
    pub p350: f64,
    pub p351: f64,
    pub p352: f64,
    pub p353: f64,
    pub p354: f64,
    pub p355: f64,
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
            params.p0 = 0.00018;
            params.p1 = 2.5e-7;
            params.p2 = 1.0;
            params.p3 = 0.0;
            params.p4 = 4.0;
            params.p5 = 27.0;
            params.p6 = 1.0;
            params.p7 = 0.004;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
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
            params.p27 = -50.0;
            params.p28 = 100.0;
            params.p29 = 150.0;
            params.p30 = 0.0008;
            params.p31 = 0.0008;
            params.p32 = 300000.0;
            params.p33 = 0.135;
            params.p34 = 2.0;
            params.p35 = -2.72;
            params.p36 = 0.12;
            params.p37 = 0.016;
            params.p38 = 0.0;
            params.p39 = 10.0;
            params.p40 = 0.0;
            params.p41 = 3.5;
            params.p42 = 0.0;
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 150000.0;
            params.p46 = -0.0004;
            params.p47 = 2.3;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 4e-5;
            params.p54 = 3e-6;
            params.p55 = -650.0;
            params.p56 = 0.005;
            params.p57 = 100000.0;
            params.p58 = 0.1;
            params.p59 = 1.0;
            params.p60 = 0.1;
            params.p61 = 0.1;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 3.5;
            params.p66 = 4.85e-6;
            params.p67 = -650.0;
            params.p68 = 0.0043;
            params.p69 = 100000.0;
            params.p70 = 0.1;
            params.p71 = 1.0;
            params.p72 = 0.35;
            params.p73 = 0.3;
            params.p74 = 3.8;
            params.p75 = 0.0;
            params.p76 = 0.0;
            params.p77 = 3.5;
            params.p78 = 1.0;
            params.p79 = 0.0;
            params.p80 = -44.5;
            params.p81 = 0.0002;
            params.p82 = 0.0;
            params.p83 = 1.0;
            params.p84 = 0.0;
            params.p85 = 1.0;
            params.p86 = 9e-11;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 0.0;
            params.p90 = 120000.0;
            params.p91 = 0.2;
            params.p92 = 1.0;
            params.p93 = 0.0;
            params.p94 = 3.2;
            params.p95 = 0.0;
            params.p96 = -0.0004;
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 0.01;
            params.p100 = 0.0;
            params.p101 = 0.0;
            params.p102 = -74.5;
            params.p103 = 0.0001;
            params.p104 = 0.0;
            params.p105 = 1.0;
            params.p106 = 0.0;
            params.p107 = 1.0;
            params.p108 = 3e-11;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 0.0;
            params.p112 = 120000.0;
            params.p113 = 0.2;
            params.p114 = 1.0;
            params.p115 = 0.0;
            params.p116 = 3.2;
            params.p117 = 0.0;
            params.p118 = -0.0004;
            params.p119 = 0.0;
            params.p120 = 0.0;
            params.p121 = 0.01;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = -74.5;
            params.p125 = 0.0001;
            params.p126 = 0.0;
            params.p127 = 1.0;
            params.p128 = 0.0;
            params.p129 = 1.0;
            params.p130 = 3e-11;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 120000.0;
            params.p135 = 0.2;
            params.p136 = 1.0;
            params.p137 = 0.0;
            params.p138 = 3.2;
            params.p139 = 0.0;
            params.p140 = -0.0004;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = 0.01;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = -74.5;
            params.p147 = 0.0001;
            params.p148 = 0.0;
            params.p149 = 1.0;
            params.p150 = 0.0;
            params.p151 = 1.0;
            params.p152 = 3e-11;
            params.p153 = 0.0;
            params.p154 = 0.0;
            params.p155 = 0.0;
            params.p156 = 120000.0;
            params.p157 = 0.2;
            params.p158 = 1.0;
            params.p159 = 0.0;
            params.p160 = 3.2;
            params.p161 = 0.0;
            params.p162 = -0.0004;
            params.p163 = 0.0;
            params.p164 = 0.0;
            params.p165 = 0.01;
            params.p166 = 1.0;
            params.p167 = 0.0;
            params.p168 = -44.5;
            params.p169 = 0.0002;
            params.p170 = 0.0;
            params.p171 = 1.0;
            params.p172 = 0.0;
            params.p173 = 1.0;
            params.p174 = 9e-11;
            params.p175 = 0.0;
            params.p176 = 0.0;
            params.p177 = 0.0;
            params.p178 = 120000.0;
            params.p179 = 0.2;
            params.p180 = 1.0;
            params.p181 = 0.0;
            params.p182 = 3.2;
            params.p183 = 0.0;
            params.p184 = -0.0004;
            params.p185 = 0.0;
            params.p186 = 0.0;
            params.p187 = 0.01;
            params.p188 = 0.0;
            params.p189 = 0.0;
            params.p190 = -74.5;
            params.p191 = 0.0001;
            params.p192 = 0.0;
            params.p193 = 1.0;
            params.p194 = 0.0;
            params.p195 = 1.0;
            params.p196 = 3e-11;
            params.p197 = 0.0;
            params.p198 = 0.0;
            params.p199 = 0.0;
            params.p200 = 120000.0;
            params.p201 = 0.2;
            params.p202 = 1.0;
            params.p203 = 0.0;
            params.p204 = 3.2;
            params.p205 = 0.0;
            params.p206 = -0.0004;
            params.p207 = 0.0;
            params.p208 = 0.0;
            params.p209 = 0.01;
            params.p210 = 0.0;
            params.p211 = 0.0;
            params.p212 = -74.5;
            params.p213 = 0.0002;
            params.p214 = 0.0;
            params.p215 = 1.0;
            params.p216 = 0.0;
            params.p217 = 1.0;
            params.p218 = 9e-11;
            params.p219 = 0.0;
            params.p220 = 0.0;
            params.p221 = 0.0;
            params.p222 = 120000.0;
            params.p223 = 0.2;
            params.p224 = 1.0;
            params.p225 = 0.0;
            params.p226 = 3.2;
            params.p227 = 0.0;
            params.p228 = -0.0004;
            params.p229 = 0.0;
            params.p230 = 0.0;
            params.p231 = 0.01;
            params.p232 = 0.0;
            params.p233 = 0.0;
            params.p234 = -74.5;
            params.p235 = 0.0002;
            params.p236 = 0.0;
            params.p237 = 1.0;
            params.p238 = 0.0;
            params.p239 = 1.0;
            params.p240 = 9e-11;
            params.p241 = 0.0;
            params.p242 = 0.0;
            params.p243 = 0.0;
            params.p244 = 120000.0;
            params.p245 = 0.2;
            params.p246 = 1.0;
            params.p247 = 0.0;
            params.p248 = 3.2;
            params.p249 = 0.0;
            params.p250 = -0.0004;
            params.p251 = 0.0;
            params.p252 = 0.0;
            params.p253 = 0.01;
            params.p254 = 0.0;
            params.p255 = 0.0;
            params.p256 = 1.1;
            params.p257 = 0.82;
            params.p258 = 1.0;
            params.p259 = 1e-12;
            params.p260 = 1.0;
            params.p261 = 0.5;
            params.p262 = 1.0;
            params.p263 = 1.0;
            params.p264 = 1e-12;
            params.p265 = 1.0;
            params.p266 = 0.5;
            params.p267 = 1.0;
            params.p268 = 0.5;
            params.p269 = 1e-18;
            params.p270 = 2.0;
            params.p271 = 2.0;
            params.p272 = 0.8;
            params.p273 = 2e-5;
            params.p274 = 0.8;
            params.p275 = 0.25;
            params.p276 = 0.0;
            params.p277 = 600.0;
            params.p278 = 4.0;
            params.p279 = 0.0;
            params.p280 = 600.0;
            params.p281 = 4.0;
            params.p282 = 0.0;
            params.p283 = 0.5;
            params.p284 = 1e-18;
            params.p285 = 2.0;
            params.p286 = 2.0;
            params.p287 = 0.8;
            params.p288 = 2e-5;
            params.p289 = 0.8;
            params.p290 = 0.25;
            params.p291 = 0.0;
            params.p292 = 0.05;
            params.p293 = 2e-5;
            params.p294 = 3.0;
            params.p295 = 0.4;
            params.p296 = 1.0;
            params.p297 = 0.5;
            params.p298 = 1e-21;
            params.p299 = 20000.0;
            params.p300 = 1.0;
            params.p301 = 0.0;
            params.p302 = 0.5;
            params.p303 = 1e-21;
            params.p304 = 20000.0;
            params.p305 = 1.0;
            params.p306 = 2.0;
            params.p307 = 6e-8;
            params.p308 = 0.5;
            params.p309 = 2.0;
            params.p310 = 0.0;
            params.p311 = 0.0;
            params.p312 = 0.0;
            params.p313 = 1.0;
            params.p314 = 1e-9;
            params.p315 = 1e-9;
            params.p316 = 50.0;
            params.p317 = 4.0;
            params.p318 = 50.0;
            params.p319 = 4.0;
            params.p320 = 25.0;
            params.p321 = 0.0001;
            params.p322 = 0.0;
            params.p323 = 0.001;
            params.p324 = 0.0;
            params.p325 = 1.0;
            params.p326 = 0.0;
            params.p327 = 1.0;
            params.p328 = 0.0;
            params.p329 = 1000000000.0;
            params.p330 = 0.001;
            params.p331 = 100.0;
            params.p332 = 3e-5;
            params.p333 = 0.001;
            params.p334 = 0.05;
            params.p335 = 0.001;
            params.p336 = 0.0001;
            params.p337 = 10.0;
            params.p338 = 100.0;
            params.p339 = 10.0;
            params.p340 = 0.05;
            params.p341 = 1e-6;
            params.p342 = -0.005;
            params.p343 = 0.005;
            params.p344 = 0.0;
            params.p345 = 0.0;
            params.p346 = 1e-9;
            params.p347 = 0.0;
            params.p348 = 3.0;
            params.p349 = 3.0;
            params.p350 = 0.0001;
            params.p351 = 2.0;
            params.p352 = 1.2;
            params.p353 = 0.001;
            validate_parameter("minr", params.p353, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p354 = 1e-9;
            params.p355 = 0.0;
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
    pub nodes: [usize; 30],
    pub branches: [usize; 36],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 356]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 146]>,
    pub(crate) ddt_state_previous: Box<[f64; 146]>,
    pub(crate) ddt_state_older: Box<[f64; 146]>,
    pub(crate) ddt_state_initialized: Box<[bool; 146]>,
    pub(crate) ddt_derivative_current: Box<[f64; 146]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 146]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 1724]>,
    pub(crate) scalar_static_bool: Box<[bool; 263]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 25;
    pub const NODE_COUNT: usize = 30;
    pub const INTERNAL_NODE_NAMES: [&str; 25] = ["di", "gi1", "gi2", "gi2p", "si", "fps1", "fps2", "fps3", "fps4", "fp1", "fp2", "fp3", "fp4", "drc", "src", "tr", "tr1", "dtrapin", "dtrapin2", "dtrapin3", "gtrapin", "gtrapin2", "gtrapin3", "xt1", "xt2"];

    pub const BRANCH_COUNT: usize = 36;
    pub const PARAMETER_COUNT: usize = 356;
    pub const VARIABLE_COUNT: usize = 2701;
    pub const DDT_STATE_COUNT: usize = 146;
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
            scalar_static_f64: boxed_zero_f64_array::<1724>(),
            scalar_static_bool: boxed_zero_bool_array::<263>(),
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
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngf" => { validate_parameter("ngf", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cg" => { validate_parameter("cg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcg" => { validate_finite_parameter("tcg", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofsm" => { validate_parameter("cofsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdm" => { validate_parameter("cofdm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdsm" => { validate_parameter("cofdsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdsubm" => { validate_parameter("cofdsubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofssubm" => { validate_parameter("cofssubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofgsubm" => { validate_parameter("cofgsubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofsm0" => { validate_parameter("cofsm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdm0" => { validate_parameter("cofdm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdsm0" => { validate_parameter("cofdsm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofdsubm0" => { validate_parameter("cofdsubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofssubm0" => { validate_parameter("cofssubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofgsubm0" => { validate_parameter("cofgsubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofs" => { validate_finite_parameter("tcofs", value)?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofd" => { validate_finite_parameter("tcofd", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofds" => { validate_finite_parameter("tcofds", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofssub" => { validate_finite_parameter("tcofssub", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofdsub" => { validate_finite_parameter("tcofdsub", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcofgsub" => { validate_finite_parameter("tcofgsub", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtfrin" => { validate_finite_parameter("vtfrin", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfrin" => { validate_parameter("nfrin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcs" => { validate_parameter("rcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcd" => { validate_parameter("rcd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0" => { validate_parameter("vx0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0" => { validate_parameter("mu0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "beta" => { validate_parameter("beta", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vto" => { validate_finite_parameter("vto", value)?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ss" => { validate_parameter("ss", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1" => { validate_parameter("delta1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta2" => { validate_parameter("delta2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dibsat" => { validate_parameter("dibsat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nd" => { validate_parameter("nd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpha" => { validate_parameter("alpha", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lambda" => { validate_parameter("lambda", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtheta" => { validate_parameter("vtheta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mtheta" => { validate_parameter("mtheta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzeta" => { validate_parameter("vzeta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzeta" => { validate_finite_parameter("vtzeta", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "epsilon" => { validate_parameter("epsilon", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct1" => { validate_finite_parameter("rct1", value)?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct2" => { validate_finite_parameter("rct2", value)?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagres" => { validate_parameter("flagres", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagsp" => { validate_parameter("flagsp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flaggum" => { validate_parameter("flaggum", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mmaxs" => { validate_parameter("mmaxs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgs" => { validate_parameter("lgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtors" => { validate_finite_parameter("vtors", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgrs" => { validate_parameter("cgrs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0rs" => { validate_parameter("vx0rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0rs" => { validate_parameter("mu0rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betars" => { validate_parameter("betars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1rs" => { validate_parameter("delta1rs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "srs" => { validate_parameter("srs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndrs" => { validate_parameter("ndrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetars" => { validate_parameter("vthetars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetars" => { validate_parameter("mthetars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphars" => { validate_parameter("alphars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgd" => { validate_parameter("lgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtord" => { validate_finite_parameter("vtord", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgrd" => { validate_parameter("cgrd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0rd" => { validate_parameter("vx0rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0rd" => { validate_parameter("mu0rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betard" => { validate_parameter("betard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1rd" => { validate_parameter("delta1rd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "srd" => { validate_parameter("srd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndrd" => { validate_parameter("ndrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetard" => { validate_parameter("vthetard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetard" => { validate_parameter("mthetard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphard" => { validate_parameter("alphard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps1" => { validate_parameter("flagfps1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfps1" => { validate_parameter("lgfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofps1" => { validate_finite_parameter("vtofps1", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfps1" => { validate_parameter("cgfps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfps1" => { validate_finite_parameter("tcgfps1", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps1s" => { validate_parameter("flagfps1s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfps1s" => { validate_parameter("cfps1s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps1b" => { validate_parameter("flagfps1b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfps1" => { validate_parameter("ccfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfps1" => { validate_finite_parameter("tccfps1", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfps1" => { validate_parameter("cbfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfps1" => { validate_finite_parameter("tcbfps1", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fps1" => { validate_parameter("vx0fps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fps1" => { validate_parameter("mu0fps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafps1" => { validate_parameter("betafps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fps1" => { validate_parameter("delta1fps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfps1" => { validate_parameter("sfps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfps1" => { validate_parameter("ndfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafps1" => { validate_finite_parameter("vtzetafps1", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafps1" => { validate_parameter("vthetafps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafps1" => { validate_parameter("mthetafps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafps1" => { validate_parameter("alphafps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps2" => { validate_parameter("flagfps2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfps2" => { validate_parameter("lgfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofps2" => { validate_finite_parameter("vtofps2", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfps2" => { validate_parameter("cgfps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfps2" => { validate_finite_parameter("tcgfps2", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps2s" => { validate_parameter("flagfps2s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfps2s" => { validate_parameter("cfps2s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps2b" => { validate_parameter("flagfps2b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfps2" => { validate_parameter("ccfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfps2" => { validate_finite_parameter("tccfps2", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfps2" => { validate_parameter("cbfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfps2" => { validate_finite_parameter("tcbfps2", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fps2" => { validate_parameter("vx0fps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fps2" => { validate_parameter("mu0fps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafps2" => { validate_parameter("betafps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fps2" => { validate_parameter("delta1fps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfps2" => { validate_parameter("sfps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfps2" => { validate_parameter("ndfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafps2" => { validate_finite_parameter("vtzetafps2", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafps2" => { validate_parameter("vthetafps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafps2" => { validate_parameter("mthetafps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafps2" => { validate_parameter("alphafps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps3" => { validate_parameter("flagfps3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfps3" => { validate_parameter("lgfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofps3" => { validate_finite_parameter("vtofps3", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfps3" => { validate_parameter("cgfps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfps3" => { validate_finite_parameter("tcgfps3", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps3s" => { validate_parameter("flagfps3s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfps3s" => { validate_parameter("cfps3s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps3b" => { validate_parameter("flagfps3b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfps3" => { validate_parameter("ccfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfps3" => { validate_finite_parameter("tccfps3", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfps3" => { validate_parameter("cbfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfps3" => { validate_finite_parameter("tcbfps3", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fps3" => { validate_parameter("vx0fps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fps3" => { validate_parameter("mu0fps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafps3" => { validate_parameter("betafps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fps3" => { validate_parameter("delta1fps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfps3" => { validate_parameter("sfps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfps3" => { validate_parameter("ndfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafps3" => { validate_finite_parameter("vtzetafps3", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafps3" => { validate_parameter("vthetafps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafps3" => { validate_parameter("mthetafps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafps3" => { validate_parameter("alphafps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps4" => { validate_parameter("flagfps4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfps4" => { validate_parameter("lgfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofps4" => { validate_finite_parameter("vtofps4", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfps4" => { validate_parameter("cgfps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfps4" => { validate_finite_parameter("tcgfps4", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps4s" => { validate_parameter("flagfps4s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfps4s" => { validate_parameter("cfps4s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfps4b" => { validate_parameter("flagfps4b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfps4" => { validate_parameter("ccfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfps4" => { validate_finite_parameter("tccfps4", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfps4" => { validate_parameter("cbfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfps4" => { validate_finite_parameter("tcbfps4", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fps4" => { validate_parameter("vx0fps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fps4" => { validate_parameter("mu0fps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafps4" => { validate_parameter("betafps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fps4" => { validate_parameter("delta1fps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfps4" => { validate_parameter("sfps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfps4" => { validate_parameter("ndfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafps4" => { validate_finite_parameter("vtzetafps4", value)?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafps4" => { validate_parameter("vthetafps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafps4" => { validate_parameter("mthetafps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafps4" => { validate_parameter("alphafps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp1" => { validate_parameter("flagfp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfp1" => { validate_parameter("lgfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofp1" => { validate_finite_parameter("vtofp1", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfp1" => { validate_parameter("cgfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfp1" => { validate_finite_parameter("tcgfp1", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp1s" => { validate_parameter("flagfp1s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfp1s" => { validate_parameter("cfp1s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp1b" => { validate_parameter("flagfp1b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfp1" => { validate_parameter("ccfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfp1" => { validate_finite_parameter("tccfp1", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfp1" => { validate_parameter("cbfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfp1" => { validate_finite_parameter("tcbfp1", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fp1" => { validate_parameter("vx0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fp1" => { validate_parameter("mu0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafp1" => { validate_parameter("betafp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fp1" => { validate_parameter("delta1fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfp1" => { validate_parameter("sfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfp1" => { validate_parameter("ndfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafp1" => { validate_finite_parameter("vtzetafp1", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafp1" => { validate_parameter("vthetafp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafp1" => { validate_parameter("mthetafp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafp1" => { validate_parameter("alphafp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp2" => { validate_parameter("flagfp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfp2" => { validate_parameter("lgfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofp2" => { validate_finite_parameter("vtofp2", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfp2" => { validate_parameter("cgfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfp2" => { validate_finite_parameter("tcgfp2", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp2s" => { validate_parameter("flagfp2s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfp2s" => { validate_parameter("cfp2s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp2b" => { validate_parameter("flagfp2b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfp2" => { validate_parameter("ccfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfp2" => { validate_finite_parameter("tccfp2", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfp2" => { validate_parameter("cbfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfp2" => { validate_finite_parameter("tcbfp2", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fp2" => { validate_parameter("vx0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fp2" => { validate_parameter("mu0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafp2" => { validate_parameter("betafp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fp2" => { validate_parameter("delta1fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfp2" => { validate_parameter("sfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfp2" => { validate_parameter("ndfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafp2" => { validate_finite_parameter("vtzetafp2", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafp2" => { validate_parameter("vthetafp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafp2" => { validate_parameter("mthetafp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafp2" => { validate_parameter("alphafp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp3" => { validate_parameter("flagfp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfp3" => { validate_parameter("lgfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofp3" => { validate_finite_parameter("vtofp3", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfp3" => { validate_parameter("cgfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfp3" => { validate_finite_parameter("tcgfp3", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp3s" => { validate_parameter("flagfp3s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfp3s" => { validate_parameter("cfp3s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp3b" => { validate_parameter("flagfp3b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfp3" => { validate_parameter("ccfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfp3" => { validate_finite_parameter("tccfp3", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfp3" => { validate_parameter("cbfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfp3" => { validate_finite_parameter("tcbfp3", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fp3" => { validate_parameter("vx0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fp3" => { validate_parameter("mu0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafp3" => { validate_parameter("betafp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fp3" => { validate_parameter("delta1fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfp3" => { validate_parameter("sfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfp3" => { validate_parameter("ndfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafp3" => { validate_finite_parameter("vtzetafp3", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafp3" => { validate_parameter("vthetafp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafp3" => { validate_parameter("mthetafp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafp3" => { validate_parameter("alphafp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp4" => { validate_parameter("flagfp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgfp4" => { validate_parameter("lgfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtofp4" => { validate_finite_parameter("vtofp4", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgfp4" => { validate_parameter("cgfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcgfp4" => { validate_finite_parameter("tcgfp4", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp4s" => { validate_parameter("flagfp4s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfp4s" => { validate_parameter("cfp4s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagfp4b" => { validate_parameter("flagfp4b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ccfp4" => { validate_parameter("ccfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tccfp4" => { validate_finite_parameter("tccfp4", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbfp4" => { validate_parameter("cbfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcbfp4" => { validate_finite_parameter("tcbfp4", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vx0fp4" => { validate_parameter("vx0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mu0fp4" => { validate_parameter("mu0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betafp4" => { validate_parameter("betafp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delta1fp4" => { validate_parameter("delta1fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sfp4" => { validate_parameter("sfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndfp4" => { validate_parameter("ndfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtzetafp4" => { validate_finite_parameter("vtzetafp4", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthetafp4" => { validate_parameter("vthetafp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mthetafp4" => { validate_parameter("mthetafp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphafp4" => { validate_parameter("alphafp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igmod" => { validate_parameter("igmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fracig" => { validate_parameter("fracig", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vjg" => { validate_parameter("vjg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg_param1" => { validate_parameter("pg_param1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg_params" => { validate_parameter("pg_params", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ijs" => { validate_parameter("ijs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsats" => { validate_parameter("vgsats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fracs" => { validate_parameter("fracs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphags" => { validate_parameter("alphags", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg_paramd" => { validate_parameter("pg_paramd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ijd" => { validate_parameter("ijd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatd" => { validate_parameter("vgsatd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fracd" => { validate_parameter("fracd", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphagd" => { validate_parameter("alphagd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrecs" => { validate_parameter("pgsrecs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irecs" => { validate_parameter("irecs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatqs" => { validate_parameter("vgsatqs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarecs" => { validate_parameter("betarecs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrecd" => { validate_parameter("pgsrecd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irecd" => { validate_parameter("irecd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatqd" => { validate_parameter("vgsatqd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarecd" => { validate_parameter("betarecd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbdgates" => { validate_parameter("kbdgates", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbdgs" => { validate_parameter("vbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbdgs" => { validate_parameter("pbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kbdgated" => { validate_parameter("kbdgated", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbdgd" => { validate_parameter("vbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbdgd" => { validate_parameter("pbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igrecmod" => { validate_parameter("igrecmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrecs2" => { validate_parameter("pgsrecs2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irecs2" => { validate_parameter("irecs2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatqs2" => { validate_parameter("vgsatqs2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarecs2" => { validate_parameter("betarecs2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrecd2" => { validate_parameter("pgsrecd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irecd2" => { validate_parameter("irecd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatqd2" => { validate_parameter("vgsatqd2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarecd2" => { validate_parameter("betarecd2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "flagpgan" => { validate_parameter("flagpgan", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pg_param_pgan" => { validate_parameter("pg_param_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ij_pgan" => { validate_parameter("ij_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsat_pgan" => { validate_parameter("vgsat_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "frac_pgan" => { validate_parameter("frac_pgan", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphag_pgan" => { validate_parameter("alphag_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrec_pgan" => { validate_parameter("pgsrec_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irec_pgan" => { validate_parameter("irec_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatq_pgan" => { validate_parameter("vgsatq_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarec_pgan" => { validate_parameter("betarec_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pganrecmod" => { validate_parameter("pganrecmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgsrec_pgan2" => { validate_parameter("pgsrec_pgan2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "irec_pgan2" => { validate_parameter("irec_pgan2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsatq_pgan2" => { validate_parameter("vgsatq_pgan2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betarec_pgan2" => { validate_parameter("betarec_pgan2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vcsh0" => { validate_parameter("vcsh0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csh0" => { validate_parameter("csh0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgancshorder" => { validate_parameter("pgancshorder", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsch0" => { validate_parameter("rsch0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ohmicratio" => { validate_parameter("ohmicratio", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "icbdmod" => { validate_parameter("icbdmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cbddbmod" => { validate_parameter("cbddbmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ijscbd" => { validate_parameter("ijscbd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ijdcbd" => { validate_parameter("ijdcbd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vchbdgs" => { validate_parameter("vchbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pchbdgs" => { validate_parameter("pchbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vchbdgd" => { validate_parameter("vchbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pchbdgd" => { validate_parameter("pchbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gmdisp" => { validate_parameter("gmdisp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taugmrf" => { validate_parameter("taugmrf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rgsp" => { validate_parameter("rgsp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("ngcon", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lovg" => { validate_parameter("lovg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agate" => { validate_parameter("agate", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "trapselect" => { validate_parameter("trapselect", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rintrap1" => { validate_parameter("rintrap1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctrap" => { validate_parameter("ctrap", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vttrap" => { validate_parameter("vttrap", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taut" => { validate_parameter("taut", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphat1" => { validate_parameter("alphat1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphat2" => { validate_parameter("alphat2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alphat3" => { validate_parameter("alphat3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tempt" => { validate_parameter("tempt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgltrapth" => { validate_parameter("vgltrapth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdltrapth" => { validate_parameter("vdltrapth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rcapture" => { validate_parameter("rcapture", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "remission" => { validate_parameter("remission", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cdglag" => { validate_parameter("cdglag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct1dl" => { validate_finite_parameter("rct1dl", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct1gl" => { validate_finite_parameter("rct1gl", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct2dl" => { validate_finite_parameter("rct2dl", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rct2gl" => { validate_finite_parameter("rct2gl", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "isat" => { validate_parameter("isat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "noisemod" => { validate_parameter("noisemod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shs" => { validate_parameter("shs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shd" => { validate_parameter("shd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ffe" => { validate_parameter("ffe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minl" => { validate_parameter("minl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "minc" => { validate_parameter("minc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'mvsg_cmc'", name)),
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
        self.scalar_static_f64[0]=p.p5;
        self.scalar_static_f64[1]=(self.scalar_static_f64[0]+273.15);
        self.scalar_static_f64[2]=p.p3;
        self.scalar_static_f64[3]=p.p50;
        self.scalar_static_bool[0]=(0.0==self.scalar_static_f64[3]);
        self.scalar_static_f64[4]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[5]=p.p30;
        self.scalar_static_f64[6]=p.p0;
        self.scalar_static_f64[7]=(self.scalar_static_f64[5]/self.scalar_static_f64[6]);
        self.scalar_static_f64[8]=p.p2;
        self.scalar_static_f64[9]=(self.scalar_static_f64[7]/self.scalar_static_f64[8]);
        self.scalar_static_f64[10]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[9]}else{0.0});
        self.scalar_static_f64[11]=p.p31;
        self.scalar_static_f64[12]=(self.scalar_static_f64[11]/self.scalar_static_f64[6]);
        self.scalar_static_f64[13]=(self.scalar_static_f64[12]/self.scalar_static_f64[8]);
        self.scalar_static_f64[14]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[13]}else{0.0});
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[15]=p.p29;
        self.scalar_static_f64[16]=p.p54;
        self.scalar_static_f64[17]=(self.scalar_static_f64[15]*self.scalar_static_f64[16]);
        self.scalar_static_f64[18]=(self.scalar_static_f64[17]/self.scalar_static_f64[6]);
        self.scalar_static_f64[19]=(self.scalar_static_f64[7]+self.scalar_static_f64[18]);
        self.scalar_static_f64[20]=(self.scalar_static_f64[19]/self.scalar_static_f64[8]);
        self.scalar_static_f64[21]=(if self.scalar_static_bool[1]{self.scalar_static_f64[20]}else{self.scalar_static_f64[10]});
        self.scalar_static_f64[22]=p.p66;
        self.scalar_static_f64[23]=(self.scalar_static_f64[15]*self.scalar_static_f64[22]);
        self.scalar_static_f64[24]=(self.scalar_static_f64[23]/self.scalar_static_f64[6]);
        self.scalar_static_f64[25]=(self.scalar_static_f64[12]+self.scalar_static_f64[24]);
        self.scalar_static_f64[26]=(self.scalar_static_f64[25]/self.scalar_static_f64[8]);
        self.scalar_static_f64[27]=(if self.scalar_static_bool[1]{self.scalar_static_f64[26]}else{self.scalar_static_f64[14]});
        self.scalar_static_f64[28]=p.p353;
        self.scalar_static_bool[2]=(self.scalar_static_f64[21]>=self.scalar_static_f64[28]);
        self.scalar_static_bool[3]=(self.scalar_static_f64[21]>0.0);
        self.scalar_static_bool[4]=(self.scalar_static_bool[2]&&self.scalar_static_bool[3]);
        self.scalar_static_f64[29]=(if self.scalar_static_bool[4]{1.0}else{0.0});
        self.scalar_static_f64[30]=p.p48;
        self.scalar_static_f64[31]=p.p49;
        self.scalar_static_f64[32]=(self.scalar_static_f64[21]*0.1);
        self.scalar_static_bool[5]=(!(self.scalar_static_f64[29]!=0.0));
        self.scalar_static_bool[6]=(self.scalar_static_f64[27]>=self.scalar_static_f64[28]);
        self.scalar_static_bool[7]=(self.scalar_static_f64[27]>0.0);
        self.scalar_static_bool[8]=(self.scalar_static_bool[6]&&self.scalar_static_bool[7]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[34]=(self.scalar_static_f64[27]*0.1);
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[33]!=0.0));
        self.scalar_static_f64[35]=p.p324;
        self.scalar_static_f64[36]=(self.scalar_static_f64[35]/self.scalar_static_f64[8]);
        self.scalar_static_f64[37]=p.p325;
        self.scalar_static_f64[38]=(self.scalar_static_f64[36]/self.scalar_static_f64[37]);
        self.scalar_static_f64[39]=p.p326;
        self.scalar_static_f64[40]=p.p327;
        self.scalar_static_f64[41]=(self.scalar_static_f64[6]*self.scalar_static_f64[40]);
        self.scalar_static_f64[42]=(self.scalar_static_f64[41]/self.scalar_static_f64[37]);
        self.scalar_static_f64[43]=(self.scalar_static_f64[39]+self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=(self.scalar_static_f64[38]*self.scalar_static_f64[43]);
        self.scalar_static_f64[45]=(1.0-self.scalar_static_f64[40]);
        self.scalar_static_f64[46]=(self.scalar_static_f64[6]*self.scalar_static_f64[45]);
        self.scalar_static_f64[47]=(self.scalar_static_f64[46]/self.scalar_static_f64[37]);
        self.scalar_static_f64[48]=(self.scalar_static_f64[38]*self.scalar_static_f64[47]);
        self.scalar_static_f64[49]=p.p336;
        self.scalar_static_f64[50]=p.p9;
        self.scalar_static_f64[51]=p.p21;
        self.scalar_static_f64[52]=p.p10;
        self.scalar_static_f64[53]=p.p22;
        self.scalar_static_f64[54]=p.p11;
        self.scalar_static_f64[55]=p.p23;
        self.scalar_static_f64[56]=p.p13;
        self.scalar_static_f64[57]=p.p24;
        self.scalar_static_f64[58]=p.p12;
        self.scalar_static_f64[59]=p.p25;
        self.scalar_static_f64[60]=p.p14;
        self.scalar_static_f64[61]=p.p26;
        self.scalar_static_f64[62]=p.p15;
        self.scalar_static_f64[63]=p.p16;
        self.scalar_static_f64[64]=p.p17;
        self.scalar_static_f64[65]=p.p19;
        self.scalar_static_f64[66]=p.p18;
        self.scalar_static_f64[67]=p.p20;
        self.scalar_static_f64[68]=p.p7;
        self.scalar_static_f64[69]=p.p8;
        self.scalar_static_f64[70]=p.p81;
        self.scalar_static_f64[71]=p.p82;
        self.scalar_static_f64[72]=p.p103;
        self.scalar_static_f64[73]=p.p104;
        self.scalar_static_f64[74]=p.p125;
        self.scalar_static_f64[75]=p.p126;
        self.scalar_static_f64[76]=p.p147;
        self.scalar_static_f64[77]=p.p148;
        self.scalar_static_f64[78]=p.p86;
        self.scalar_static_f64[79]=p.p87;
        self.scalar_static_f64[80]=p.p108;
        self.scalar_static_f64[81]=p.p109;
        self.scalar_static_f64[82]=p.p130;
        self.scalar_static_f64[83]=p.p131;
        self.scalar_static_f64[84]=p.p152;
        self.scalar_static_f64[85]=p.p153;
        self.scalar_static_f64[86]=p.p88;
        self.scalar_static_f64[87]=p.p89;
        self.scalar_static_f64[88]=p.p110;
        self.scalar_static_f64[89]=p.p111;
        self.scalar_static_f64[90]=p.p132;
        self.scalar_static_f64[91]=p.p133;
        self.scalar_static_f64[92]=p.p154;
        self.scalar_static_f64[93]=p.p155;
        self.scalar_static_f64[94]=p.p169;
        self.scalar_static_f64[95]=p.p170;
        self.scalar_static_f64[96]=p.p191;
        self.scalar_static_f64[97]=p.p192;
        self.scalar_static_f64[98]=p.p213;
        self.scalar_static_f64[99]=p.p214;
        self.scalar_static_f64[100]=p.p235;
        self.scalar_static_f64[101]=p.p236;
        self.scalar_static_f64[102]=p.p174;
        self.scalar_static_f64[103]=p.p175;
        self.scalar_static_f64[104]=p.p196;
        self.scalar_static_f64[105]=p.p197;
        self.scalar_static_f64[106]=p.p218;
        self.scalar_static_f64[107]=p.p219;
        self.scalar_static_f64[108]=p.p240;
        self.scalar_static_f64[109]=p.p241;
        self.scalar_static_f64[110]=p.p176;
        self.scalar_static_f64[111]=p.p177;
        self.scalar_static_f64[112]=p.p198;
        self.scalar_static_f64[113]=p.p199;
        self.scalar_static_f64[114]=p.p220;
        self.scalar_static_f64[115]=p.p221;
        self.scalar_static_f64[116]=p.p242;
        self.scalar_static_f64[117]=p.p243;
        self.scalar_static_f64[118]=p.p6;
        self.scalar_static_f64[119]=p.p52;
        self.scalar_static_bool[10]=(0.0==self.scalar_static_f64[119]);
        self.scalar_static_f64[120]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[120]!=0.0));
        self.scalar_static_bool[12]=(!self.scalar_static_bool[10]);
        self.scalar_static_f64[121]=p.p53;
        self.scalar_static_f64[122]=(0.001/self.scalar_static_f64[121]);
        self.scalar_static_f64[123]=p.p55;
        self.scalar_static_f64[124]=p.p56;
        self.scalar_static_f64[125]=(self.scalar_static_f64[15]*self.scalar_static_f64[124]);
        self.scalar_static_f64[126]=p.p33;
        self.scalar_static_f64[127]=(self.scalar_static_f64[125]*self.scalar_static_f64[126]);
        self.scalar_static_f64[128]=(1.0/self.scalar_static_f64[127]);
        self.scalar_static_f64[129]=(self.scalar_static_f64[123]+self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=p.p328;
        self.scalar_static_bool[13]=(1.0==self.scalar_static_f64[130]);
        self.scalar_static_f64[131]=(if self.scalar_static_bool[13]{1.0}else{0.0});
        self.scalar_static_f64[132]=p.p333;
        self.scalar_static_f64[133]=p.p331;
        self.scalar_static_f64[134]=p.p335;
        self.scalar_static_f64[135]=p.p334;
        self.scalar_static_bool[14]=(self.scalar_static_f64[130]==2.0);
        self.scalar_static_f64[136]=(if self.scalar_static_bool[14]{1.0}else{0.0});
        self.scalar_static_bool[15]=(!(self.scalar_static_f64[131]!=0.0));
        self.scalar_static_bool[16]=((self.scalar_static_f64[136]!=0.0)&&self.scalar_static_bool[15]);
        self.scalar_static_f64[137]=p.p338;
        self.scalar_static_f64[138]=p.p337;
        self.scalar_static_f64[139]=p.p67;
        self.scalar_static_f64[140]=p.p68;
        self.scalar_static_f64[141]=p.p78;
        self.scalar_static_bool[17]=(1.0==self.scalar_static_f64[141]);
        self.scalar_static_f64[142]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[18]=(!(self.scalar_static_f64[142]!=0.0));
        self.scalar_static_f64[143]=p.p100;
        self.scalar_static_bool[19]=(1.0==self.scalar_static_f64[143]);
        self.scalar_static_f64[144]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_bool[20]=(!(self.scalar_static_f64[144]!=0.0));
        self.scalar_static_f64[145]=p.p122;
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[145]);
        self.scalar_static_f64[146]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_bool[22]=(!(self.scalar_static_f64[146]!=0.0));
        self.scalar_static_f64[147]=p.p144;
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[147]);
        self.scalar_static_f64[148]=(if self.scalar_static_bool[23]{1.0}else{0.0});
        self.scalar_static_bool[24]=(!(self.scalar_static_f64[148]!=0.0));
        self.scalar_static_f64[149]=p.p166;
        self.scalar_static_bool[25]=(1.0==self.scalar_static_f64[149]);
        self.scalar_static_f64[150]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_bool[26]=(!(self.scalar_static_f64[150]!=0.0));
        self.scalar_static_f64[151]=p.p188;
        self.scalar_static_bool[27]=(1.0==self.scalar_static_f64[151]);
        self.scalar_static_f64[152]=(if self.scalar_static_bool[27]{1.0}else{0.0});
        self.scalar_static_bool[28]=(!(self.scalar_static_f64[152]!=0.0));
        self.scalar_static_f64[153]=p.p210;
        self.scalar_static_bool[29]=(1.0==self.scalar_static_f64[153]);
        self.scalar_static_f64[154]=(if self.scalar_static_bool[29]{1.0}else{0.0});
        self.scalar_static_bool[30]=(!(self.scalar_static_f64[154]!=0.0));
        self.scalar_static_f64[155]=p.p232;
        self.scalar_static_bool[31]=(1.0==self.scalar_static_f64[155]);
        self.scalar_static_f64[156]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[156]!=0.0));
        self.scalar_static_f64[157]=p.p233;
        self.scalar_static_f64[158]=p.p354;
        self.scalar_static_bool[33]=(self.scalar_static_f64[157]>self.scalar_static_f64[158]);
        self.scalar_static_f64[159]=(if self.scalar_static_bool[33]{1.0}else{0.0});
        self.scalar_static_f64[160]=p.p239;
        self.scalar_static_f64[161]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[160]}else{0.0});
        self.scalar_static_f64[162]=p.p237;
        self.scalar_static_f64[163]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[162]}else{0.0});
        self.scalar_static_f64[164]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[165]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[166]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[157]}else{0.0});
        self.scalar_static_f64[167]=p.p238;
        self.scalar_static_f64[168]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[167]}else{0.0});
        self.scalar_static_f64[169]=p.p234;
        self.scalar_static_f64[170]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[169]}else{0.0});
        self.scalar_static_f64[171]=p.p248;
        self.scalar_static_f64[172]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[171]}else{0.0});
        self.scalar_static_f64[173]=p.p247;
        self.scalar_static_f64[174]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[173]}else{0.0});
        self.scalar_static_f64[175]=p.p249;
        self.scalar_static_f64[176]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[175]}else{0.0});
        self.scalar_static_f64[177]=p.p253;
        self.scalar_static_f64[178]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[177]}else{0.0});
        self.scalar_static_f64[179]=p.p244;
        self.scalar_static_f64[180]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[179]}else{0.0});
        self.scalar_static_f64[181]=p.p245;
        self.scalar_static_f64[182]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[181]}else{0.0});
        self.scalar_static_f64[183]=p.p246;
        self.scalar_static_f64[184]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[183]}else{0.0});
        self.scalar_static_f64[185]=p.p252;
        self.scalar_static_f64[186]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[185]}else{0.0});
        self.scalar_static_f64[187]=p.p251;
        self.scalar_static_f64[188]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[187]}else{0.0});
        self.scalar_static_f64[189]=p.p250;
        self.scalar_static_f64[190]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[189]}else{0.0});
        self.scalar_static_f64[191]=p.p39;
        self.scalar_static_f64[192]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[193]=p.p47;
        self.scalar_static_f64[194]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[195]=p.p45;
        self.scalar_static_f64[196]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[197]=p.p42;
        self.scalar_static_f64[198]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[199]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[200]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[201]=(if (self.scalar_static_f64[159]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[34]=(0.0!=self.scalar_static_f64[192]);
        self.scalar_static_f64[202]=(if self.scalar_static_bool[34]{1.0}else{0.0});
        self.scalar_static_bool[35]=((self.scalar_static_f64[159]!=0.0)&&(self.scalar_static_f64[202]!=0.0));
        self.scalar_static_f64[203]=(1.0/self.scalar_static_f64[184]);
        self.scalar_static_bool[36]=(!(self.scalar_static_f64[202]!=0.0));
        self.scalar_static_bool[37]=((self.scalar_static_f64[159]!=0.0)&&self.scalar_static_bool[36]);
        self.scalar_static_f64[204]=p.p51;
        self.scalar_static_f64[205]=(0.1*self.scalar_static_f64[204]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[164]*self.scalar_static_f64[196]);
        self.scalar_static_f64[207]=(1.0+self.scalar_static_f64[206]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[165]*self.scalar_static_f64[200]);
        self.scalar_static_f64[209]=(self.scalar_static_f64[199]*self.scalar_static_f64[208]);
        self.scalar_static_f64[210]=(0.5*self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[165]*self.scalar_static_f64[199]);
        self.scalar_static_f64[212]=(self.scalar_static_f64[166]*self.scalar_static_f64[211]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[200]*self.scalar_static_f64[212]);
        self.scalar_static_bool[38]=(1.0==self.scalar_static_f64[161]);
        self.scalar_static_f64[214]=(if self.scalar_static_bool[38]{1.0}else{0.0});
        self.scalar_static_bool[39]=((self.scalar_static_f64[159]!=0.0)&&(self.scalar_static_f64[214]!=0.0));
        self.scalar_static_f64[215]=(0.5*self.scalar_static_f64[204]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[200]*self.scalar_static_f64[211]);
        self.scalar_static_bool[40]=(!(self.scalar_static_f64[214]!=0.0));
        self.scalar_static_bool[41]=((self.scalar_static_f64[159]!=0.0)&&self.scalar_static_bool[40]);
        self.scalar_static_bool[42]=(1.0==self.scalar_static_f64[163]);
        self.scalar_static_f64[217]=(if self.scalar_static_bool[42]{1.0}else{0.0});
        self.scalar_static_bool[43]=((self.scalar_static_f64[159]!=0.0)&&(self.scalar_static_f64[217]!=0.0));
        self.scalar_static_f64[218]=(self.scalar_static_f64[168]*self.scalar_static_f64[216]);
        self.scalar_static_bool[44]=(!(self.scalar_static_f64[217]!=0.0));
        self.scalar_static_bool[45]=((self.scalar_static_f64[159]!=0.0)&&self.scalar_static_bool[44]);
        self.scalar_static_f64[219]=p.p211;
        self.scalar_static_bool[46]=(self.scalar_static_f64[219]>self.scalar_static_f64[158]);
        self.scalar_static_f64[220]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_f64[221]=p.p217;
        self.scalar_static_f64[222]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[221]}else{0.0});
        self.scalar_static_f64[223]=p.p215;
        self.scalar_static_f64[224]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[223]}else{0.0});
        self.scalar_static_f64[225]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[226]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[227]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[219]}else{0.0});
        self.scalar_static_f64[228]=p.p216;
        self.scalar_static_f64[229]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[228]}else{0.0});
        self.scalar_static_f64[230]=p.p212;
        self.scalar_static_f64[231]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[230]}else{0.0});
        self.scalar_static_f64[232]=p.p226;
        self.scalar_static_f64[233]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[232]}else{0.0});
        self.scalar_static_f64[234]=p.p225;
        self.scalar_static_f64[235]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[234]}else{0.0});
        self.scalar_static_f64[236]=p.p227;
        self.scalar_static_f64[237]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[236]}else{0.0});
        self.scalar_static_f64[238]=p.p231;
        self.scalar_static_f64[239]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[238]}else{0.0});
        self.scalar_static_f64[240]=p.p222;
        self.scalar_static_f64[241]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[240]}else{0.0});
        self.scalar_static_f64[242]=p.p223;
        self.scalar_static_f64[243]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[242]}else{0.0});
        self.scalar_static_f64[244]=p.p224;
        self.scalar_static_f64[245]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[244]}else{0.0});
        self.scalar_static_f64[246]=p.p230;
        self.scalar_static_f64[247]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[246]}else{0.0});
        self.scalar_static_f64[248]=p.p229;
        self.scalar_static_f64[249]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[248]}else{0.0});
        self.scalar_static_f64[250]=p.p228;
        self.scalar_static_f64[251]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[250]}else{0.0});
        self.scalar_static_f64[252]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[253]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[254]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[255]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[256]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[257]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[258]=(if (self.scalar_static_f64[220]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[47]=(0.0!=self.scalar_static_f64[252]);
        self.scalar_static_f64[259]=(if self.scalar_static_bool[47]{1.0}else{0.0});
        self.scalar_static_bool[48]=((self.scalar_static_f64[220]!=0.0)&&(self.scalar_static_f64[259]!=0.0));
        self.scalar_static_f64[260]=(1.0/self.scalar_static_f64[245]);
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[259]!=0.0));
        self.scalar_static_bool[50]=((self.scalar_static_f64[220]!=0.0)&&self.scalar_static_bool[49]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[225]*self.scalar_static_f64[254]);
        self.scalar_static_f64[262]=(1.0+self.scalar_static_f64[261]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[226]*self.scalar_static_f64[257]);
        self.scalar_static_f64[264]=(self.scalar_static_f64[256]*self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=(0.5*self.scalar_static_f64[264]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[226]*self.scalar_static_f64[256]);
        self.scalar_static_f64[267]=(self.scalar_static_f64[227]*self.scalar_static_f64[266]);
        self.scalar_static_f64[268]=(self.scalar_static_f64[257]*self.scalar_static_f64[267]);
        self.scalar_static_bool[51]=(1.0==self.scalar_static_f64[222]);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[51]{1.0}else{0.0});
        self.scalar_static_bool[52]=((self.scalar_static_f64[220]!=0.0)&&(self.scalar_static_f64[269]!=0.0));
        self.scalar_static_f64[270]=(self.scalar_static_f64[257]*self.scalar_static_f64[266]);
        self.scalar_static_bool[53]=(!(self.scalar_static_f64[269]!=0.0));
        self.scalar_static_bool[54]=((self.scalar_static_f64[220]!=0.0)&&self.scalar_static_bool[53]);
        self.scalar_static_bool[55]=(1.0==self.scalar_static_f64[224]);
        self.scalar_static_f64[271]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_bool[56]=((self.scalar_static_f64[220]!=0.0)&&(self.scalar_static_f64[271]!=0.0));
        self.scalar_static_f64[272]=(self.scalar_static_f64[229]*self.scalar_static_f64[270]);
        self.scalar_static_bool[57]=(!(self.scalar_static_f64[271]!=0.0));
        self.scalar_static_bool[58]=((self.scalar_static_f64[220]!=0.0)&&self.scalar_static_bool[57]);
        self.scalar_static_f64[273]=p.p189;
        self.scalar_static_bool[59]=(self.scalar_static_f64[273]>self.scalar_static_f64[158]);
        self.scalar_static_f64[274]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_f64[275]=p.p195;
        self.scalar_static_f64[276]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=p.p193;
        self.scalar_static_f64[278]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_f64[279]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[280]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[281]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[273]}else{0.0});
        self.scalar_static_f64[282]=p.p194;
        self.scalar_static_f64[283]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[282]}else{0.0});
        self.scalar_static_f64[284]=p.p190;
        self.scalar_static_f64[285]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[284]}else{0.0});
        self.scalar_static_f64[286]=p.p204;
        self.scalar_static_f64[287]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[286]}else{0.0});
        self.scalar_static_f64[288]=p.p203;
        self.scalar_static_f64[289]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[288]}else{0.0});
        self.scalar_static_f64[290]=p.p205;
        self.scalar_static_f64[291]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[290]}else{0.0});
        self.scalar_static_f64[292]=p.p209;
        self.scalar_static_f64[293]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[292]}else{0.0});
        self.scalar_static_f64[294]=p.p200;
        self.scalar_static_f64[295]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[294]}else{0.0});
        self.scalar_static_f64[296]=p.p201;
        self.scalar_static_f64[297]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[296]}else{0.0});
        self.scalar_static_f64[298]=p.p202;
        self.scalar_static_f64[299]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[298]}else{0.0});
        self.scalar_static_f64[300]=p.p208;
        self.scalar_static_f64[301]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[300]}else{0.0});
        self.scalar_static_f64[302]=p.p207;
        self.scalar_static_f64[303]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[302]}else{0.0});
        self.scalar_static_f64[304]=p.p206;
        self.scalar_static_f64[305]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[304]}else{0.0});
        self.scalar_static_f64[306]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[307]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[308]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[309]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[310]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[311]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[312]=(if (self.scalar_static_f64[274]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[60]=(0.0!=self.scalar_static_f64[306]);
        self.scalar_static_f64[313]=(if self.scalar_static_bool[60]{1.0}else{0.0});
        self.scalar_static_bool[61]=((self.scalar_static_f64[274]!=0.0)&&(self.scalar_static_f64[313]!=0.0));
        self.scalar_static_f64[314]=(1.0/self.scalar_static_f64[299]);
        self.scalar_static_bool[62]=(!(self.scalar_static_f64[313]!=0.0));
        self.scalar_static_bool[63]=((self.scalar_static_f64[274]!=0.0)&&self.scalar_static_bool[62]);
        self.scalar_static_f64[315]=(self.scalar_static_f64[279]*self.scalar_static_f64[308]);
        self.scalar_static_f64[316]=(1.0+self.scalar_static_f64[315]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[280]*self.scalar_static_f64[311]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[310]*self.scalar_static_f64[317]);
        self.scalar_static_f64[319]=(0.5*self.scalar_static_f64[318]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[280]*self.scalar_static_f64[310]);
        self.scalar_static_f64[321]=(self.scalar_static_f64[281]*self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[311]*self.scalar_static_f64[321]);
        self.scalar_static_bool[64]=(1.0==self.scalar_static_f64[276]);
        self.scalar_static_f64[323]=(if self.scalar_static_bool[64]{1.0}else{0.0});
        self.scalar_static_bool[65]=((self.scalar_static_f64[274]!=0.0)&&(self.scalar_static_f64[323]!=0.0));
        self.scalar_static_f64[324]=(self.scalar_static_f64[311]*self.scalar_static_f64[320]);
        self.scalar_static_bool[66]=(!(self.scalar_static_f64[323]!=0.0));
        self.scalar_static_bool[67]=((self.scalar_static_f64[274]!=0.0)&&self.scalar_static_bool[66]);
        self.scalar_static_bool[68]=(1.0==self.scalar_static_f64[278]);
        self.scalar_static_f64[325]=(if self.scalar_static_bool[68]{1.0}else{0.0});
        self.scalar_static_bool[69]=((self.scalar_static_f64[274]!=0.0)&&(self.scalar_static_f64[325]!=0.0));
        self.scalar_static_f64[326]=(self.scalar_static_f64[283]*self.scalar_static_f64[324]);
        self.scalar_static_bool[70]=(!(self.scalar_static_f64[325]!=0.0));
        self.scalar_static_bool[71]=((self.scalar_static_f64[274]!=0.0)&&self.scalar_static_bool[70]);
        self.scalar_static_f64[327]=p.p167;
        self.scalar_static_bool[72]=(self.scalar_static_f64[327]>self.scalar_static_f64[158]);
        self.scalar_static_f64[328]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_f64[329]=p.p173;
        self.scalar_static_f64[330]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[329]}else{0.0});
        self.scalar_static_f64[331]=p.p171;
        self.scalar_static_f64[332]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[331]}else{0.0});
        self.scalar_static_f64[333]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[334]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[335]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[327]}else{0.0});
        self.scalar_static_f64[336]=p.p172;
        self.scalar_static_f64[337]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[336]}else{0.0});
        self.scalar_static_f64[338]=p.p168;
        self.scalar_static_f64[339]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[338]}else{0.0});
        self.scalar_static_f64[340]=p.p182;
        self.scalar_static_f64[341]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[340]}else{0.0});
        self.scalar_static_f64[342]=p.p181;
        self.scalar_static_f64[343]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[342]}else{0.0});
        self.scalar_static_f64[344]=p.p183;
        self.scalar_static_f64[345]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[344]}else{0.0});
        self.scalar_static_f64[346]=p.p187;
        self.scalar_static_f64[347]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[346]}else{0.0});
        self.scalar_static_f64[348]=p.p178;
        self.scalar_static_f64[349]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[348]}else{0.0});
        self.scalar_static_f64[350]=p.p179;
        self.scalar_static_f64[351]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[350]}else{0.0});
        self.scalar_static_f64[352]=p.p180;
        self.scalar_static_f64[353]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[354]=p.p186;
        self.scalar_static_f64[355]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_f64[356]=p.p185;
        self.scalar_static_f64[357]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[356]}else{0.0});
        self.scalar_static_f64[358]=p.p184;
        self.scalar_static_f64[359]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[358]}else{0.0});
        self.scalar_static_f64[360]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[361]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[362]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[363]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[364]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[365]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[366]=(if (self.scalar_static_f64[328]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[73]=(0.0!=self.scalar_static_f64[360]);
        self.scalar_static_f64[367]=(if self.scalar_static_bool[73]{1.0}else{0.0});
        self.scalar_static_bool[74]=((self.scalar_static_f64[328]!=0.0)&&(self.scalar_static_f64[367]!=0.0));
        self.scalar_static_f64[368]=(1.0/self.scalar_static_f64[353]);
        self.scalar_static_bool[75]=(!(self.scalar_static_f64[367]!=0.0));
        self.scalar_static_bool[76]=((self.scalar_static_f64[328]!=0.0)&&self.scalar_static_bool[75]);
        self.scalar_static_f64[369]=(self.scalar_static_f64[333]*self.scalar_static_f64[362]);
        self.scalar_static_f64[370]=(1.0+self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=(self.scalar_static_f64[334]*self.scalar_static_f64[365]);
        self.scalar_static_f64[372]=(self.scalar_static_f64[364]*self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=(0.5*self.scalar_static_f64[372]);
        self.scalar_static_f64[374]=(self.scalar_static_f64[334]*self.scalar_static_f64[364]);
        self.scalar_static_f64[375]=(self.scalar_static_f64[335]*self.scalar_static_f64[374]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[365]*self.scalar_static_f64[375]);
        self.scalar_static_bool[77]=(1.0==self.scalar_static_f64[330]);
        self.scalar_static_f64[377]=(if self.scalar_static_bool[77]{1.0}else{0.0});
        self.scalar_static_bool[78]=((self.scalar_static_f64[328]!=0.0)&&(self.scalar_static_f64[377]!=0.0));
        self.scalar_static_f64[378]=(self.scalar_static_f64[365]*self.scalar_static_f64[374]);
        self.scalar_static_bool[79]=(!(self.scalar_static_f64[377]!=0.0));
        self.scalar_static_bool[80]=((self.scalar_static_f64[328]!=0.0)&&self.scalar_static_bool[79]);
        self.scalar_static_bool[81]=(1.0==self.scalar_static_f64[332]);
        self.scalar_static_f64[379]=(if self.scalar_static_bool[81]{1.0}else{0.0});
        self.scalar_static_bool[82]=((self.scalar_static_f64[328]!=0.0)&&(self.scalar_static_f64[379]!=0.0));
        self.scalar_static_f64[380]=(self.scalar_static_f64[337]*self.scalar_static_f64[378]);
        self.scalar_static_bool[83]=(!(self.scalar_static_f64[379]!=0.0));
        self.scalar_static_bool[84]=((self.scalar_static_f64[328]!=0.0)&&self.scalar_static_bool[83]);
        self.scalar_static_f64[381]=p.p79;
        self.scalar_static_bool[85]=(self.scalar_static_f64[381]>self.scalar_static_f64[158]);
        self.scalar_static_f64[382]=(if self.scalar_static_bool[85]{1.0}else{0.0});
        self.scalar_static_f64[383]=p.p85;
        self.scalar_static_f64[384]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[383]}else{0.0});
        self.scalar_static_f64[385]=p.p83;
        self.scalar_static_f64[386]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[385]}else{0.0});
        self.scalar_static_f64[387]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[388]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[389]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[381]}else{0.0});
        self.scalar_static_f64[390]=p.p84;
        self.scalar_static_f64[391]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[390]}else{0.0});
        self.scalar_static_f64[392]=p.p80;
        self.scalar_static_f64[393]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[392]}else{0.0});
        self.scalar_static_f64[394]=p.p94;
        self.scalar_static_f64[395]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[394]}else{0.0});
        self.scalar_static_f64[396]=p.p93;
        self.scalar_static_f64[397]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[396]}else{0.0});
        self.scalar_static_f64[398]=p.p95;
        self.scalar_static_f64[399]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[398]}else{0.0});
        self.scalar_static_f64[400]=p.p99;
        self.scalar_static_f64[401]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[400]}else{0.0});
        self.scalar_static_f64[402]=p.p90;
        self.scalar_static_f64[403]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[402]}else{0.0});
        self.scalar_static_f64[404]=p.p91;
        self.scalar_static_f64[405]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[404]}else{0.0});
        self.scalar_static_f64[406]=p.p92;
        self.scalar_static_f64[407]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[406]}else{0.0});
        self.scalar_static_f64[408]=p.p98;
        self.scalar_static_f64[409]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[408]}else{0.0});
        self.scalar_static_f64[410]=p.p97;
        self.scalar_static_f64[411]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[410]}else{0.0});
        self.scalar_static_f64[412]=p.p96;
        self.scalar_static_f64[413]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[412]}else{0.0});
        self.scalar_static_f64[414]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[415]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[416]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[417]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[418]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[419]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[420]=(if (self.scalar_static_f64[382]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[86]=(0.0!=self.scalar_static_f64[414]);
        self.scalar_static_f64[421]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_bool[87]=((self.scalar_static_f64[382]!=0.0)&&(self.scalar_static_f64[421]!=0.0));
        self.scalar_static_f64[422]=(1.0/self.scalar_static_f64[407]);
        self.scalar_static_bool[88]=(!(self.scalar_static_f64[421]!=0.0));
        self.scalar_static_bool[89]=((self.scalar_static_f64[382]!=0.0)&&self.scalar_static_bool[88]);
        self.scalar_static_f64[423]=(self.scalar_static_f64[387]*self.scalar_static_f64[416]);
        self.scalar_static_f64[424]=(1.0+self.scalar_static_f64[423]);
        self.scalar_static_f64[425]=(self.scalar_static_f64[388]*self.scalar_static_f64[419]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[418]*self.scalar_static_f64[425]);
        self.scalar_static_f64[427]=(0.5*self.scalar_static_f64[426]);
        self.scalar_static_f64[428]=(self.scalar_static_f64[388]*self.scalar_static_f64[418]);
        self.scalar_static_f64[429]=(self.scalar_static_f64[389]*self.scalar_static_f64[428]);
        self.scalar_static_f64[430]=(self.scalar_static_f64[419]*self.scalar_static_f64[429]);
        self.scalar_static_bool[90]=(1.0==self.scalar_static_f64[384]);
        self.scalar_static_f64[431]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_bool[91]=((self.scalar_static_f64[382]!=0.0)&&(self.scalar_static_f64[431]!=0.0));
        self.scalar_static_f64[432]=(self.scalar_static_f64[419]*self.scalar_static_f64[428]);
        self.scalar_static_bool[92]=(!(self.scalar_static_f64[431]!=0.0));
        self.scalar_static_bool[93]=((self.scalar_static_f64[382]!=0.0)&&self.scalar_static_bool[92]);
        self.scalar_static_bool[94]=(1.0==self.scalar_static_f64[386]);
        self.scalar_static_f64[433]=(if self.scalar_static_bool[94]{1.0}else{0.0});
        self.scalar_static_bool[95]=((self.scalar_static_f64[382]!=0.0)&&(self.scalar_static_f64[433]!=0.0));
        self.scalar_static_f64[434]=(self.scalar_static_f64[391]*self.scalar_static_f64[432]);
        self.scalar_static_bool[96]=(!(self.scalar_static_f64[433]!=0.0));
        self.scalar_static_bool[97]=((self.scalar_static_f64[382]!=0.0)&&self.scalar_static_bool[96]);
        self.scalar_static_f64[435]=p.p101;
        self.scalar_static_bool[98]=(self.scalar_static_f64[435]>self.scalar_static_f64[158]);
        self.scalar_static_f64[436]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_f64[437]=p.p107;
        self.scalar_static_f64[438]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[437]}else{0.0});
        self.scalar_static_f64[439]=p.p105;
        self.scalar_static_f64[440]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[439]}else{0.0});
        self.scalar_static_f64[441]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[442]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[443]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[435]}else{0.0});
        self.scalar_static_f64[444]=p.p106;
        self.scalar_static_f64[445]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[444]}else{0.0});
        self.scalar_static_f64[446]=p.p102;
        self.scalar_static_f64[447]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[446]}else{0.0});
        self.scalar_static_f64[448]=p.p116;
        self.scalar_static_f64[449]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[448]}else{0.0});
        self.scalar_static_f64[450]=p.p115;
        self.scalar_static_f64[451]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[450]}else{0.0});
        self.scalar_static_f64[452]=p.p117;
        self.scalar_static_f64[453]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[452]}else{0.0});
        self.scalar_static_f64[454]=p.p121;
        self.scalar_static_f64[455]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[454]}else{0.0});
        self.scalar_static_f64[456]=p.p112;
        self.scalar_static_f64[457]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[456]}else{0.0});
        self.scalar_static_f64[458]=p.p113;
        self.scalar_static_f64[459]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[458]}else{0.0});
        self.scalar_static_f64[460]=p.p114;
        self.scalar_static_f64[461]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[460]}else{0.0});
        self.scalar_static_f64[462]=p.p120;
        self.scalar_static_f64[463]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[462]}else{0.0});
        self.scalar_static_f64[464]=p.p119;
        self.scalar_static_f64[465]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[464]}else{0.0});
        self.scalar_static_f64[466]=p.p118;
        self.scalar_static_f64[467]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[466]}else{0.0});
        self.scalar_static_f64[468]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[469]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[470]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[471]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[472]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[473]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[474]=(if (self.scalar_static_f64[436]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[99]=(0.0!=self.scalar_static_f64[468]);
        self.scalar_static_f64[475]=(if self.scalar_static_bool[99]{1.0}else{0.0});
        self.scalar_static_bool[100]=((self.scalar_static_f64[436]!=0.0)&&(self.scalar_static_f64[475]!=0.0));
        self.scalar_static_f64[476]=(1.0/self.scalar_static_f64[461]);
        self.scalar_static_bool[101]=(!(self.scalar_static_f64[475]!=0.0));
        self.scalar_static_bool[102]=((self.scalar_static_f64[436]!=0.0)&&self.scalar_static_bool[101]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[441]*self.scalar_static_f64[470]);
        self.scalar_static_f64[478]=(1.0+self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(self.scalar_static_f64[442]*self.scalar_static_f64[473]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[472]*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(0.5*self.scalar_static_f64[480]);
        self.scalar_static_f64[482]=(self.scalar_static_f64[442]*self.scalar_static_f64[472]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[443]*self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=(self.scalar_static_f64[473]*self.scalar_static_f64[483]);
        self.scalar_static_bool[103]=(1.0==self.scalar_static_f64[438]);
        self.scalar_static_f64[485]=(if self.scalar_static_bool[103]{1.0}else{0.0});
        self.scalar_static_bool[104]=((self.scalar_static_f64[436]!=0.0)&&(self.scalar_static_f64[485]!=0.0));
        self.scalar_static_f64[486]=(self.scalar_static_f64[473]*self.scalar_static_f64[482]);
        self.scalar_static_bool[105]=(!(self.scalar_static_f64[485]!=0.0));
        self.scalar_static_bool[106]=((self.scalar_static_f64[436]!=0.0)&&self.scalar_static_bool[105]);
        self.scalar_static_bool[107]=(1.0==self.scalar_static_f64[440]);
        self.scalar_static_f64[487]=(if self.scalar_static_bool[107]{1.0}else{0.0});
        self.scalar_static_bool[108]=((self.scalar_static_f64[436]!=0.0)&&(self.scalar_static_f64[487]!=0.0));
        self.scalar_static_f64[488]=(self.scalar_static_f64[445]*self.scalar_static_f64[486]);
        self.scalar_static_bool[109]=(!(self.scalar_static_f64[487]!=0.0));
        self.scalar_static_bool[110]=((self.scalar_static_f64[436]!=0.0)&&self.scalar_static_bool[109]);
        self.scalar_static_f64[489]=p.p123;
        self.scalar_static_bool[111]=(self.scalar_static_f64[489]>self.scalar_static_f64[158]);
        self.scalar_static_f64[490]=(if self.scalar_static_bool[111]{1.0}else{0.0});
        self.scalar_static_f64[491]=p.p129;
        self.scalar_static_f64[492]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[491]}else{0.0});
        self.scalar_static_f64[493]=p.p127;
        self.scalar_static_f64[494]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[493]}else{0.0});
        self.scalar_static_f64[495]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[496]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[497]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[489]}else{0.0});
        self.scalar_static_f64[498]=p.p128;
        self.scalar_static_f64[499]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[498]}else{0.0});
        self.scalar_static_f64[500]=p.p124;
        self.scalar_static_f64[501]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[500]}else{0.0});
        self.scalar_static_f64[502]=p.p138;
        self.scalar_static_f64[503]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[502]}else{0.0});
        self.scalar_static_f64[504]=p.p137;
        self.scalar_static_f64[505]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[504]}else{0.0});
        self.scalar_static_f64[506]=p.p139;
        self.scalar_static_f64[507]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[506]}else{0.0});
        self.scalar_static_f64[508]=p.p143;
        self.scalar_static_f64[509]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[508]}else{0.0});
        self.scalar_static_f64[510]=p.p134;
        self.scalar_static_f64[511]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[510]}else{0.0});
        self.scalar_static_f64[512]=p.p135;
        self.scalar_static_f64[513]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[512]}else{0.0});
        self.scalar_static_f64[514]=p.p136;
        self.scalar_static_f64[515]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[514]}else{0.0});
        self.scalar_static_f64[516]=p.p142;
        self.scalar_static_f64[517]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[516]}else{0.0});
        self.scalar_static_f64[518]=p.p141;
        self.scalar_static_f64[519]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[518]}else{0.0});
        self.scalar_static_f64[520]=p.p140;
        self.scalar_static_f64[521]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[520]}else{0.0});
        self.scalar_static_f64[522]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[523]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[524]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[525]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[526]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[527]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[528]=(if (self.scalar_static_f64[490]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[112]=(0.0!=self.scalar_static_f64[522]);
        self.scalar_static_f64[529]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_bool[113]=((self.scalar_static_f64[490]!=0.0)&&(self.scalar_static_f64[529]!=0.0));
        self.scalar_static_f64[530]=(1.0/self.scalar_static_f64[515]);
        self.scalar_static_bool[114]=(!(self.scalar_static_f64[529]!=0.0));
        self.scalar_static_bool[115]=((self.scalar_static_f64[490]!=0.0)&&self.scalar_static_bool[114]);
        self.scalar_static_f64[531]=(self.scalar_static_f64[495]*self.scalar_static_f64[524]);
        self.scalar_static_f64[532]=(1.0+self.scalar_static_f64[531]);
        self.scalar_static_f64[533]=(self.scalar_static_f64[496]*self.scalar_static_f64[527]);
        self.scalar_static_f64[534]=(self.scalar_static_f64[526]*self.scalar_static_f64[533]);
        self.scalar_static_f64[535]=(0.5*self.scalar_static_f64[534]);
        self.scalar_static_f64[536]=(self.scalar_static_f64[496]*self.scalar_static_f64[526]);
        self.scalar_static_f64[537]=(self.scalar_static_f64[497]*self.scalar_static_f64[536]);
        self.scalar_static_f64[538]=(self.scalar_static_f64[527]*self.scalar_static_f64[537]);
        self.scalar_static_bool[116]=(1.0==self.scalar_static_f64[492]);
        self.scalar_static_f64[539]=(if self.scalar_static_bool[116]{1.0}else{0.0});
        self.scalar_static_bool[117]=((self.scalar_static_f64[490]!=0.0)&&(self.scalar_static_f64[539]!=0.0));
        self.scalar_static_f64[540]=(self.scalar_static_f64[527]*self.scalar_static_f64[536]);
        self.scalar_static_bool[118]=(!(self.scalar_static_f64[539]!=0.0));
        self.scalar_static_bool[119]=((self.scalar_static_f64[490]!=0.0)&&self.scalar_static_bool[118]);
        self.scalar_static_bool[120]=(1.0==self.scalar_static_f64[494]);
        self.scalar_static_f64[541]=(if self.scalar_static_bool[120]{1.0}else{0.0});
        self.scalar_static_bool[121]=((self.scalar_static_f64[490]!=0.0)&&(self.scalar_static_f64[541]!=0.0));
        self.scalar_static_f64[542]=(self.scalar_static_f64[499]*self.scalar_static_f64[540]);
        self.scalar_static_bool[122]=(!(self.scalar_static_f64[541]!=0.0));
        self.scalar_static_bool[123]=((self.scalar_static_f64[490]!=0.0)&&self.scalar_static_bool[122]);
        self.scalar_static_f64[543]=p.p145;
        self.scalar_static_bool[124]=(self.scalar_static_f64[543]>self.scalar_static_f64[158]);
        self.scalar_static_f64[544]=(if self.scalar_static_bool[124]{1.0}else{0.0});
        self.scalar_static_f64[545]=p.p151;
        self.scalar_static_f64[546]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[545]}else{0.0});
        self.scalar_static_f64[547]=p.p149;
        self.scalar_static_f64[548]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[547]}else{0.0});
        self.scalar_static_f64[549]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[550]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[551]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[543]}else{0.0});
        self.scalar_static_f64[552]=p.p150;
        self.scalar_static_f64[553]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[552]}else{0.0});
        self.scalar_static_f64[554]=p.p146;
        self.scalar_static_f64[555]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[554]}else{0.0});
        self.scalar_static_f64[556]=p.p160;
        self.scalar_static_f64[557]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[556]}else{0.0});
        self.scalar_static_f64[558]=p.p159;
        self.scalar_static_f64[559]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[558]}else{0.0});
        self.scalar_static_f64[560]=p.p161;
        self.scalar_static_f64[561]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[560]}else{0.0});
        self.scalar_static_f64[562]=p.p165;
        self.scalar_static_f64[563]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[562]}else{0.0});
        self.scalar_static_f64[564]=p.p156;
        self.scalar_static_f64[565]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[564]}else{0.0});
        self.scalar_static_f64[566]=p.p157;
        self.scalar_static_f64[567]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[566]}else{0.0});
        self.scalar_static_f64[568]=p.p158;
        self.scalar_static_f64[569]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[568]}else{0.0});
        self.scalar_static_f64[570]=p.p164;
        self.scalar_static_f64[571]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[570]}else{0.0});
        self.scalar_static_f64[572]=p.p163;
        self.scalar_static_f64[573]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[572]}else{0.0});
        self.scalar_static_f64[574]=p.p162;
        self.scalar_static_f64[575]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[574]}else{0.0});
        self.scalar_static_f64[576]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[577]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[578]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[579]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[580]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[581]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[582]=(if (self.scalar_static_f64[544]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[125]=(0.0!=self.scalar_static_f64[576]);
        self.scalar_static_f64[583]=(if self.scalar_static_bool[125]{1.0}else{0.0});
        self.scalar_static_bool[126]=((self.scalar_static_f64[544]!=0.0)&&(self.scalar_static_f64[583]!=0.0));
        self.scalar_static_f64[584]=(1.0/self.scalar_static_f64[569]);
        self.scalar_static_bool[127]=(!(self.scalar_static_f64[583]!=0.0));
        self.scalar_static_bool[128]=((self.scalar_static_f64[544]!=0.0)&&self.scalar_static_bool[127]);
        self.scalar_static_f64[585]=(self.scalar_static_f64[549]*self.scalar_static_f64[578]);
        self.scalar_static_f64[586]=(1.0+self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=(self.scalar_static_f64[550]*self.scalar_static_f64[581]);
        self.scalar_static_f64[588]=(self.scalar_static_f64[580]*self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(0.5*self.scalar_static_f64[588]);
        self.scalar_static_f64[590]=(self.scalar_static_f64[550]*self.scalar_static_f64[580]);
        self.scalar_static_f64[591]=(self.scalar_static_f64[551]*self.scalar_static_f64[590]);
        self.scalar_static_f64[592]=(self.scalar_static_f64[581]*self.scalar_static_f64[591]);
        self.scalar_static_bool[129]=(1.0==self.scalar_static_f64[546]);
        self.scalar_static_f64[593]=(if self.scalar_static_bool[129]{1.0}else{0.0});
        self.scalar_static_bool[130]=((self.scalar_static_f64[544]!=0.0)&&(self.scalar_static_f64[593]!=0.0));
        self.scalar_static_f64[594]=(self.scalar_static_f64[581]*self.scalar_static_f64[590]);
        self.scalar_static_bool[131]=(!(self.scalar_static_f64[593]!=0.0));
        self.scalar_static_bool[132]=((self.scalar_static_f64[544]!=0.0)&&self.scalar_static_bool[131]);
        self.scalar_static_bool[133]=(1.0==self.scalar_static_f64[548]);
        self.scalar_static_f64[595]=(if self.scalar_static_bool[133]{1.0}else{0.0});
        self.scalar_static_bool[134]=((self.scalar_static_f64[544]!=0.0)&&(self.scalar_static_f64[595]!=0.0));
        self.scalar_static_f64[596]=(self.scalar_static_f64[553]*self.scalar_static_f64[594]);
        self.scalar_static_bool[135]=(!(self.scalar_static_f64[595]!=0.0));
        self.scalar_static_bool[136]=((self.scalar_static_f64[544]!=0.0)&&self.scalar_static_bool[135]);
        self.scalar_static_bool[137]=(self.scalar_static_f64[16]>self.scalar_static_f64[158]);
        self.scalar_static_bool[138]=(self.scalar_static_bool[0]&&self.scalar_static_bool[137]);
        self.scalar_static_f64[597]=(if self.scalar_static_bool[138]{1.0}else{0.0});
        self.scalar_static_f64[598]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[599]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[600]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[16]}else{0.0});
        self.scalar_static_f64[601]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[124]}else{0.0});
        self.scalar_static_f64[602]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[123]}else{0.0});
        self.scalar_static_f64[603]=p.p61;
        self.scalar_static_f64[604]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[603]}else{0.0});
        self.scalar_static_f64[605]=p.p60;
        self.scalar_static_f64[606]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[605]}else{0.0});
        self.scalar_static_f64[607]=p.p62;
        self.scalar_static_f64[608]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[607]}else{0.0});
        self.scalar_static_f64[609]=p.p65;
        self.scalar_static_f64[610]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[609]}else{0.0});
        self.scalar_static_f64[611]=p.p57;
        self.scalar_static_f64[612]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[611]}else{0.0});
        self.scalar_static_f64[613]=p.p58;
        self.scalar_static_f64[614]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[613]}else{0.0});
        self.scalar_static_f64[615]=p.p59;
        self.scalar_static_f64[616]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[615]}else{0.0});
        self.scalar_static_f64[617]=p.p64;
        self.scalar_static_f64[618]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[617]}else{0.0});
        self.scalar_static_f64[619]=p.p63;
        self.scalar_static_f64[620]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[619]}else{0.0});
        self.scalar_static_f64[621]=p.p46;
        self.scalar_static_f64[622]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[621]}else{0.0});
        self.scalar_static_f64[623]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[624]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[625]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[626]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[627]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[628]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[629]=(if (self.scalar_static_f64[597]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[139]=(0.0!=self.scalar_static_f64[623]);
        self.scalar_static_f64[630]=(if self.scalar_static_bool[139]{1.0}else{0.0});
        self.scalar_static_bool[140]=((self.scalar_static_f64[597]!=0.0)&&(self.scalar_static_f64[630]!=0.0));
        self.scalar_static_f64[631]=(1.0/self.scalar_static_f64[616]);
        self.scalar_static_bool[141]=(!(self.scalar_static_f64[630]!=0.0));
        self.scalar_static_bool[142]=((self.scalar_static_f64[597]!=0.0)&&self.scalar_static_bool[141]);
        self.scalar_static_f64[632]=(self.scalar_static_f64[598]*self.scalar_static_f64[625]);
        self.scalar_static_f64[633]=(1.0+self.scalar_static_f64[632]);
        self.scalar_static_f64[634]=(self.scalar_static_f64[599]*self.scalar_static_f64[628]);
        self.scalar_static_f64[635]=(self.scalar_static_f64[627]*self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(0.5*self.scalar_static_f64[635]);
        self.scalar_static_bool[143]=(self.scalar_static_f64[22]>self.scalar_static_f64[158]);
        self.scalar_static_bool[144]=(self.scalar_static_bool[0]&&self.scalar_static_bool[143]);
        self.scalar_static_f64[637]=(if self.scalar_static_bool[144]{1.0}else{0.0});
        self.scalar_static_f64[638]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[1]}else{0.0});
        self.scalar_static_f64[639]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[640]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[641]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[140]}else{0.0});
        self.scalar_static_f64[642]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[139]}else{0.0});
        self.scalar_static_f64[643]=p.p73;
        self.scalar_static_f64[644]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[643]}else{0.0});
        self.scalar_static_f64[645]=p.p72;
        self.scalar_static_f64[646]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[645]}else{0.0});
        self.scalar_static_f64[647]=p.p74;
        self.scalar_static_f64[648]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[647]}else{0.0});
        self.scalar_static_f64[649]=p.p77;
        self.scalar_static_f64[650]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[649]}else{0.0});
        self.scalar_static_f64[651]=p.p69;
        self.scalar_static_f64[652]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[651]}else{0.0});
        self.scalar_static_f64[653]=p.p70;
        self.scalar_static_f64[654]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[653]}else{0.0});
        self.scalar_static_f64[655]=p.p71;
        self.scalar_static_f64[656]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[655]}else{0.0});
        self.scalar_static_f64[657]=p.p76;
        self.scalar_static_f64[658]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[657]}else{0.0});
        self.scalar_static_f64[659]=p.p75;
        self.scalar_static_f64[660]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[659]}else{0.0});
        self.scalar_static_f64[661]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[621]}else{0.0});
        self.scalar_static_f64[662]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[191]}else{0.0});
        self.scalar_static_f64[663]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[193]}else{0.0});
        self.scalar_static_f64[664]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[195]}else{0.0});
        self.scalar_static_f64[665]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[197]}else{0.0});
        self.scalar_static_f64[666]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[667]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[668]=(if (self.scalar_static_f64[637]!=0.0){1.0}else{0.0});
        self.scalar_static_bool[145]=(0.0!=self.scalar_static_f64[662]);
        self.scalar_static_f64[669]=(if self.scalar_static_bool[145]{1.0}else{0.0});
        self.scalar_static_bool[146]=((self.scalar_static_f64[637]!=0.0)&&(self.scalar_static_f64[669]!=0.0));
        self.scalar_static_f64[670]=(1.0/self.scalar_static_f64[656]);
        self.scalar_static_bool[147]=(!(self.scalar_static_f64[669]!=0.0));
        self.scalar_static_bool[148]=((self.scalar_static_f64[637]!=0.0)&&self.scalar_static_bool[147]);
        self.scalar_static_f64[671]=(self.scalar_static_f64[638]*self.scalar_static_f64[664]);
        self.scalar_static_f64[672]=(1.0+self.scalar_static_f64[671]);
        self.scalar_static_f64[673]=(self.scalar_static_f64[639]*self.scalar_static_f64[667]);
        self.scalar_static_f64[674]=(self.scalar_static_f64[666]*self.scalar_static_f64[673]);
        self.scalar_static_f64[675]=(0.5*self.scalar_static_f64[674]);
        self.scalar_static_f64[676]=p.p1;
        self.scalar_static_f64[677]=p.p35;
        self.scalar_static_f64[678]=p.p36;
        self.scalar_static_f64[679]=p.p37;
        self.scalar_static_f64[680]=p.p38;
        self.scalar_static_f64[681]=p.p40;
        self.scalar_static_f64[682]=p.p41;
        self.scalar_static_f64[683]=p.p32;
        self.scalar_static_f64[684]=p.p34;
        self.scalar_static_f64[685]=p.p44;
        self.scalar_static_f64[686]=p.p43;
        self.scalar_static_bool[149]=(0.0!=self.scalar_static_f64[191]);
        self.scalar_static_f64[687]=(if self.scalar_static_bool[149]{1.0}else{0.0});
        self.scalar_static_f64[688]=(1.0/self.scalar_static_f64[684]);
        self.scalar_static_bool[150]=(!(self.scalar_static_f64[687]!=0.0));
        self.scalar_static_f64[689]=(self.scalar_static_f64[1]*self.scalar_static_f64[195]);
        self.scalar_static_f64[690]=(1.0+self.scalar_static_f64[689]);
        self.scalar_static_f64[691]=(self.scalar_static_f64[6]*self.scalar_static_f64[118]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[8]*self.scalar_static_f64[691]);
        self.scalar_static_f64[693]=(0.5*self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(self.scalar_static_f64[6]*self.scalar_static_f64[8]);
        self.scalar_static_f64[695]=(self.scalar_static_f64[676]*self.scalar_static_f64[694]);
        self.scalar_static_f64[696]=(self.scalar_static_f64[118]*self.scalar_static_f64[695]);
        self.scalar_static_f64[697]=p.p322;
        self.scalar_static_bool[151]=(0.0==self.scalar_static_f64[697]);
        self.scalar_static_f64[698]=(if self.scalar_static_bool[151]{1.0}else{0.0});
        self.scalar_static_f64[699]=p.p254;
        self.scalar_static_bool[152]=(1.0==self.scalar_static_f64[699]);
        self.scalar_static_f64[700]=(if self.scalar_static_bool[152]{1.0}else{0.0});
        self.scalar_static_f64[701]=p.p260;
        self.scalar_static_f64[702]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_f64[703]=p.p262;
        self.scalar_static_f64[704]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[703]}else{0.0});
        self.scalar_static_f64[705]=p.p261;
        self.scalar_static_f64[706]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[705]}else{0.0});
        self.scalar_static_f64[707]=p.p258;
        self.scalar_static_f64[708]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[707]}else{0.0});
        self.scalar_static_f64[709]=p.p278;
        self.scalar_static_f64[710]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[709]}else{0.0});
        self.scalar_static_f64[711]=p.p277;
        self.scalar_static_f64[712]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[713]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[714]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[715]=p.p255;
        self.scalar_static_f64[716]=(1.0-self.scalar_static_f64[715]);
        self.scalar_static_f64[717]=p.p259;
        self.scalar_static_f64[718]=(self.scalar_static_f64[716]*self.scalar_static_f64[717]);
        self.scalar_static_f64[719]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[718]}else{0.0});
        self.scalar_static_f64[720]=p.p276;
        self.scalar_static_f64[721]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[720]}else{0.0});
        self.scalar_static_f64[722]=p.p270;
        self.scalar_static_f64[723]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[722]}else{0.0});
        self.scalar_static_f64[724]=p.p271;
        self.scalar_static_f64[725]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[724]}else{0.0});
        self.scalar_static_f64[726]=p.p269;
        self.scalar_static_f64[727]=(self.scalar_static_f64[716]*self.scalar_static_f64[726]);
        self.scalar_static_f64[728]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[727]}else{0.0});
        self.scalar_static_f64[729]=p.p268;
        self.scalar_static_f64[730]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[729]}else{0.0});
        self.scalar_static_f64[731]=p.p257;
        self.scalar_static_f64[732]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[731]}else{0.0});
        self.scalar_static_f64[733]=p.p256;
        self.scalar_static_f64[734]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[735]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[736]=(-self.scalar_static_f64[734]);
        self.scalar_static_f64[737]=(-self.scalar_static_f64[710]);
        self.scalar_static_f64[738]=(self.scalar_static_f64[712]*self.scalar_static_f64[737]);
        self.scalar_static_f64[739]=(self.scalar_static_f64[713]*self.scalar_static_f64[735]);
        self.scalar_static_f64[740]=(self.scalar_static_f64[714]*self.scalar_static_f64[739]);
        self.scalar_static_f64[741]=(self.scalar_static_f64[719]*self.scalar_static_f64[740]);
        self.scalar_static_bool[153]=(1.0==self.scalar_static_f64[706]);
        self.scalar_static_f64[742]=(if self.scalar_static_bool[153]{1.0}else{0.0});
        self.scalar_static_bool[154]=((self.scalar_static_f64[700]!=0.0)&&(self.scalar_static_f64[742]!=0.0));
        self.scalar_static_bool[155]=(!(self.scalar_static_f64[742]!=0.0));
        self.scalar_static_bool[156]=((self.scalar_static_f64[700]!=0.0)&&self.scalar_static_bool[155]);
        self.scalar_static_f64[743]=(-self.scalar_static_f64[702]);
        self.scalar_static_f64[744]=(self.scalar_static_f64[743]-self.scalar_static_f64[712]);
        self.scalar_static_f64[745]=(self.scalar_static_f64[710]*self.scalar_static_f64[744]);
        self.scalar_static_bool[157]=(self.scalar_static_f64[706]>0.0);
        self.scalar_static_f64[746]=(if self.scalar_static_bool[157]{1.0}else{0.0});
        self.scalar_static_bool[158]=(self.scalar_static_bool[156]&&(self.scalar_static_f64[746]!=0.0));
        self.scalar_static_f64[747]=(self.scalar_static_f64[706]*self.scalar_static_f64[708]);
        self.scalar_static_f64[748]=(if self.scalar_static_bool[158]{self.scalar_static_f64[747]}else{0.0});
        self.scalar_static_bool[159]=(!(self.scalar_static_f64[746]!=0.0));
        self.scalar_static_bool[160]=(self.scalar_static_bool[156]&&self.scalar_static_bool[159]);
        self.scalar_static_f64[749]=(self.scalar_static_f64[704]*self.scalar_static_f64[704]);
        self.scalar_static_f64[750]=(1.0/self.scalar_static_f64[725]);
        self.scalar_static_f64[751]=(-self.scalar_static_f64[735]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[713]*self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(self.scalar_static_f64[714]*self.scalar_static_f64[752]);
        self.scalar_static_f64[754]=(self.scalar_static_f64[728]*self.scalar_static_f64[753]);
        self.scalar_static_f64[755]=p.p265;
        self.scalar_static_f64[756]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[755]}else{0.0});
        self.scalar_static_f64[757]=p.p267;
        self.scalar_static_f64[758]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[757]}else{0.0});
        self.scalar_static_f64[759]=p.p266;
        self.scalar_static_f64[760]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[759]}else{0.0});
        self.scalar_static_f64[761]=p.p263;
        self.scalar_static_f64[762]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[761]}else{0.0});
        self.scalar_static_f64[763]=p.p281;
        self.scalar_static_f64[764]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[763]}else{0.0});
        self.scalar_static_f64[765]=p.p280;
        self.scalar_static_f64[766]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_f64[767]=p.p264;
        self.scalar_static_f64[768]=(self.scalar_static_f64[716]*self.scalar_static_f64[767]);
        self.scalar_static_f64[769]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[768]}else{0.0});
        self.scalar_static_f64[770]=p.p279;
        self.scalar_static_f64[771]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[770]}else{0.0});
        self.scalar_static_f64[772]=p.p274;
        self.scalar_static_f64[773]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[772]}else{0.0});
        self.scalar_static_f64[774]=p.p275;
        self.scalar_static_f64[775]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[774]}else{0.0});
        self.scalar_static_f64[776]=p.p273;
        self.scalar_static_f64[777]=(self.scalar_static_f64[716]*self.scalar_static_f64[776]);
        self.scalar_static_f64[778]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[777]}else{0.0});
        self.scalar_static_f64[779]=p.p272;
        self.scalar_static_f64[780]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[779]}else{0.0});
        self.scalar_static_f64[781]=(-self.scalar_static_f64[764]);
        self.scalar_static_f64[782]=(self.scalar_static_f64[766]*self.scalar_static_f64[781]);
        self.scalar_static_f64[783]=(self.scalar_static_f64[740]*self.scalar_static_f64[769]);
        self.scalar_static_bool[161]=(1.0==self.scalar_static_f64[760]);
        self.scalar_static_f64[784]=(if self.scalar_static_bool[161]{1.0}else{0.0});
        self.scalar_static_bool[162]=((self.scalar_static_f64[700]!=0.0)&&(self.scalar_static_f64[784]!=0.0));
        self.scalar_static_bool[163]=(!(self.scalar_static_f64[784]!=0.0));
        self.scalar_static_bool[164]=((self.scalar_static_f64[700]!=0.0)&&self.scalar_static_bool[163]);
        self.scalar_static_f64[785]=(-self.scalar_static_f64[756]);
        self.scalar_static_f64[786]=(self.scalar_static_f64[785]-self.scalar_static_f64[766]);
        self.scalar_static_f64[787]=(self.scalar_static_f64[764]*self.scalar_static_f64[786]);
        self.scalar_static_bool[165]=(self.scalar_static_f64[760]>0.0);
        self.scalar_static_f64[788]=(if self.scalar_static_bool[165]{1.0}else{0.0});
        self.scalar_static_bool[166]=(self.scalar_static_bool[164]&&(self.scalar_static_f64[788]!=0.0));
        self.scalar_static_f64[789]=(self.scalar_static_f64[760]*self.scalar_static_f64[762]);
        self.scalar_static_f64[790]=(if self.scalar_static_bool[166]{self.scalar_static_f64[789]}else{0.0});
        self.scalar_static_bool[167]=(!(self.scalar_static_f64[788]!=0.0));
        self.scalar_static_bool[168]=(self.scalar_static_bool[164]&&self.scalar_static_bool[167]);
        self.scalar_static_f64[791]=(self.scalar_static_f64[758]*self.scalar_static_f64[758]);
        self.scalar_static_f64[792]=(1.0/self.scalar_static_f64[775]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[753]*self.scalar_static_f64[778]);
        self.scalar_static_f64[794]=p.p282;
        self.scalar_static_bool[169]=(1.0==self.scalar_static_f64[794]);
        self.scalar_static_f64[795]=(if self.scalar_static_bool[169]{1.0}else{0.0});
        self.scalar_static_bool[170]=((self.scalar_static_f64[700]!=0.0)&&(self.scalar_static_f64[795]!=0.0));
        self.scalar_static_f64[796]=(if self.scalar_static_bool[170]{self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_f64[797]=(if self.scalar_static_bool[170]{self.scalar_static_f64[703]}else{0.0});
        self.scalar_static_f64[798]=(if self.scalar_static_bool[170]{1.0}else{0.0});
        self.scalar_static_f64[799]=(if self.scalar_static_bool[170]{self.scalar_static_f64[707]}else{0.0});
        self.scalar_static_f64[800]=(if self.scalar_static_bool[170]{self.scalar_static_f64[709]}else{0.0});
        self.scalar_static_f64[801]=(if self.scalar_static_bool[170]{self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[802]=(if self.scalar_static_bool[170]{self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[803]=(if self.scalar_static_bool[170]{self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[804]=p.p285;
        self.scalar_static_f64[805]=(if self.scalar_static_bool[170]{self.scalar_static_f64[804]}else{0.0});
        self.scalar_static_f64[806]=p.p286;
        self.scalar_static_f64[807]=(if self.scalar_static_bool[170]{self.scalar_static_f64[806]}else{0.0});
        self.scalar_static_f64[808]=p.p284;
        self.scalar_static_f64[809]=(self.scalar_static_f64[716]*self.scalar_static_f64[808]);
        self.scalar_static_f64[810]=(if self.scalar_static_bool[170]{self.scalar_static_f64[809]}else{0.0});
        self.scalar_static_f64[811]=p.p283;
        self.scalar_static_f64[812]=(if self.scalar_static_bool[170]{self.scalar_static_f64[811]}else{0.0});
        self.scalar_static_f64[813]=(if self.scalar_static_bool[170]{self.scalar_static_f64[731]}else{0.0});
        self.scalar_static_f64[814]=(if self.scalar_static_bool[170]{self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[815]=(if self.scalar_static_bool[170]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[816]=(-self.scalar_static_f64[814]);
        self.scalar_static_f64[817]=(-self.scalar_static_f64[800]);
        self.scalar_static_f64[818]=(self.scalar_static_f64[801]*self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[802]*self.scalar_static_f64[815]);
        self.scalar_static_f64[820]=(self.scalar_static_f64[803]*self.scalar_static_f64[819]);
        self.scalar_static_f64[821]=(0.0*self.scalar_static_f64[820]);
        self.scalar_static_bool[171]=(1.0==self.scalar_static_f64[798]);
        self.scalar_static_f64[822]=(if self.scalar_static_bool[171]{1.0}else{0.0});
        self.scalar_static_bool[172]=(self.scalar_static_bool[170]&&(self.scalar_static_f64[822]!=0.0));
        self.scalar_static_bool[173]=(!(self.scalar_static_f64[822]!=0.0));
        self.scalar_static_bool[174]=(self.scalar_static_bool[170]&&self.scalar_static_bool[173]);
        self.scalar_static_f64[823]=(-self.scalar_static_f64[796]);
        self.scalar_static_f64[824]=(self.scalar_static_f64[823]-self.scalar_static_f64[801]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[800]*self.scalar_static_f64[824]);
        self.scalar_static_bool[175]=(self.scalar_static_f64[798]>0.0);
        self.scalar_static_f64[826]=(if self.scalar_static_bool[175]{1.0}else{0.0});
        self.scalar_static_bool[176]=(self.scalar_static_bool[174]&&(self.scalar_static_f64[826]!=0.0));
        self.scalar_static_f64[827]=(self.scalar_static_f64[798]*self.scalar_static_f64[799]);
        self.scalar_static_f64[828]=(if self.scalar_static_bool[176]{self.scalar_static_f64[827]}else{0.0});
        self.scalar_static_bool[177]=(!(self.scalar_static_f64[826]!=0.0));
        self.scalar_static_bool[178]=(self.scalar_static_bool[174]&&self.scalar_static_bool[177]);
        self.scalar_static_f64[829]=(self.scalar_static_f64[797]*self.scalar_static_f64[797]);
        self.scalar_static_f64[830]=(1.0/self.scalar_static_f64[807]);
        self.scalar_static_f64[831]=(-self.scalar_static_f64[815]);
        self.scalar_static_f64[832]=(self.scalar_static_f64[802]*self.scalar_static_f64[831]);
        self.scalar_static_f64[833]=(self.scalar_static_f64[803]*self.scalar_static_f64[832]);
        self.scalar_static_f64[834]=(self.scalar_static_f64[810]*self.scalar_static_f64[833]);
        self.scalar_static_f64[835]=(if self.scalar_static_bool[170]{self.scalar_static_f64[755]}else{0.0});
        self.scalar_static_f64[836]=(if self.scalar_static_bool[170]{self.scalar_static_f64[757]}else{0.0});
        self.scalar_static_f64[837]=(if self.scalar_static_bool[170]{self.scalar_static_f64[761]}else{0.0});
        self.scalar_static_f64[838]=(if self.scalar_static_bool[170]{self.scalar_static_f64[763]}else{0.0});
        self.scalar_static_f64[839]=(if self.scalar_static_bool[170]{self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_f64[840]=p.p289;
        self.scalar_static_f64[841]=(if self.scalar_static_bool[170]{self.scalar_static_f64[840]}else{0.0});
        self.scalar_static_f64[842]=p.p290;
        self.scalar_static_f64[843]=(if self.scalar_static_bool[170]{self.scalar_static_f64[842]}else{0.0});
        self.scalar_static_f64[844]=p.p288;
        self.scalar_static_f64[845]=(self.scalar_static_f64[716]*self.scalar_static_f64[844]);
        self.scalar_static_f64[846]=(if self.scalar_static_bool[170]{self.scalar_static_f64[845]}else{0.0});
        self.scalar_static_f64[847]=p.p287;
        self.scalar_static_f64[848]=(if self.scalar_static_bool[170]{self.scalar_static_f64[847]}else{0.0});
        self.scalar_static_f64[849]=(-self.scalar_static_f64[838]);
        self.scalar_static_f64[850]=(self.scalar_static_f64[839]*self.scalar_static_f64[849]);
        self.scalar_static_f64[851]=(-self.scalar_static_f64[835]);
        self.scalar_static_f64[852]=(self.scalar_static_f64[851]-self.scalar_static_f64[839]);
        self.scalar_static_f64[853]=(self.scalar_static_f64[838]*self.scalar_static_f64[852]);
        self.scalar_static_f64[854]=(self.scalar_static_f64[798]*self.scalar_static_f64[837]);
        self.scalar_static_f64[855]=(if self.scalar_static_bool[176]{self.scalar_static_f64[854]}else{0.0});
        self.scalar_static_f64[856]=(self.scalar_static_f64[836]*self.scalar_static_f64[836]);
        self.scalar_static_f64[857]=(1.0/self.scalar_static_f64[843]);
        self.scalar_static_f64[858]=(self.scalar_static_f64[833]*self.scalar_static_f64[846]);
        self.scalar_static_bool[179]=(0.0!=self.scalar_static_f64[715]);
        self.scalar_static_f64[859]=(if self.scalar_static_bool[179]{1.0}else{0.0});
        self.scalar_static_bool[180]=((self.scalar_static_f64[700]!=0.0)&&(self.scalar_static_f64[859]!=0.0));
        self.scalar_static_f64[860]=(if self.scalar_static_bool[180]{self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_f64[861]=(if self.scalar_static_bool[180]{self.scalar_static_f64[703]}else{0.0});
        self.scalar_static_f64[862]=(if self.scalar_static_bool[180]{self.scalar_static_f64[705]}else{0.0});
        self.scalar_static_f64[863]=(if self.scalar_static_bool[180]{self.scalar_static_f64[707]}else{0.0});
        self.scalar_static_f64[864]=(if self.scalar_static_bool[180]{self.scalar_static_f64[709]}else{0.0});
        self.scalar_static_f64[865]=(if self.scalar_static_bool[180]{self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[866]=(if self.scalar_static_bool[180]{self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[867]=(if self.scalar_static_bool[180]{self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[868]=(self.scalar_static_f64[715]*self.scalar_static_f64[717]);
        self.scalar_static_f64[869]=(if self.scalar_static_bool[180]{self.scalar_static_f64[868]}else{0.0});
        self.scalar_static_f64[870]=(if self.scalar_static_bool[180]{self.scalar_static_f64[720]}else{0.0});
        self.scalar_static_f64[871]=(if self.scalar_static_bool[180]{self.scalar_static_f64[722]}else{0.0});
        self.scalar_static_f64[872]=(if self.scalar_static_bool[180]{self.scalar_static_f64[724]}else{0.0});
        self.scalar_static_f64[873]=(self.scalar_static_f64[715]*self.scalar_static_f64[726]);
        self.scalar_static_f64[874]=(if self.scalar_static_bool[180]{self.scalar_static_f64[873]}else{0.0});
        self.scalar_static_f64[875]=(if self.scalar_static_bool[180]{self.scalar_static_f64[729]}else{0.0});
        self.scalar_static_f64[876]=(if self.scalar_static_bool[180]{self.scalar_static_f64[731]}else{0.0});
        self.scalar_static_f64[877]=(if self.scalar_static_bool[180]{self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[878]=(if self.scalar_static_bool[180]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[879]=(-self.scalar_static_f64[877]);
        self.scalar_static_f64[880]=(-self.scalar_static_f64[864]);
        self.scalar_static_f64[881]=(self.scalar_static_f64[865]*self.scalar_static_f64[880]);
        self.scalar_static_f64[882]=(self.scalar_static_f64[866]*self.scalar_static_f64[878]);
        self.scalar_static_f64[883]=(self.scalar_static_f64[867]*self.scalar_static_f64[882]);
        self.scalar_static_f64[884]=(self.scalar_static_f64[869]*self.scalar_static_f64[883]);
        self.scalar_static_bool[181]=(1.0==self.scalar_static_f64[862]);
        self.scalar_static_f64[885]=(if self.scalar_static_bool[181]{1.0}else{0.0});
        self.scalar_static_bool[182]=(self.scalar_static_bool[180]&&(self.scalar_static_f64[885]!=0.0));
        self.scalar_static_bool[183]=(!(self.scalar_static_f64[885]!=0.0));
        self.scalar_static_bool[184]=(self.scalar_static_bool[180]&&self.scalar_static_bool[183]);
        self.scalar_static_f64[886]=(-self.scalar_static_f64[860]);
        self.scalar_static_f64[887]=(self.scalar_static_f64[886]-self.scalar_static_f64[865]);
        self.scalar_static_f64[888]=(self.scalar_static_f64[864]*self.scalar_static_f64[887]);
        self.scalar_static_bool[185]=(self.scalar_static_f64[862]>0.0);
        self.scalar_static_f64[889]=(if self.scalar_static_bool[185]{1.0}else{0.0});
        self.scalar_static_bool[186]=(self.scalar_static_bool[184]&&(self.scalar_static_f64[889]!=0.0));
        self.scalar_static_f64[890]=(self.scalar_static_f64[862]*self.scalar_static_f64[863]);
        self.scalar_static_f64[891]=(if self.scalar_static_bool[186]{self.scalar_static_f64[890]}else{0.0});
        self.scalar_static_bool[187]=(!(self.scalar_static_f64[889]!=0.0));
        self.scalar_static_bool[188]=(self.scalar_static_bool[184]&&self.scalar_static_bool[187]);
        self.scalar_static_f64[892]=(self.scalar_static_f64[861]*self.scalar_static_f64[861]);
        self.scalar_static_f64[893]=(1.0/self.scalar_static_f64[872]);
        self.scalar_static_f64[894]=(-self.scalar_static_f64[878]);
        self.scalar_static_f64[895]=(self.scalar_static_f64[866]*self.scalar_static_f64[894]);
        self.scalar_static_f64[896]=(self.scalar_static_f64[867]*self.scalar_static_f64[895]);
        self.scalar_static_f64[897]=(self.scalar_static_f64[874]*self.scalar_static_f64[896]);
        self.scalar_static_f64[898]=(if self.scalar_static_bool[180]{self.scalar_static_f64[755]}else{0.0});
        self.scalar_static_f64[899]=(if self.scalar_static_bool[180]{self.scalar_static_f64[757]}else{0.0});
        self.scalar_static_f64[900]=(if self.scalar_static_bool[180]{self.scalar_static_f64[759]}else{0.0});
        self.scalar_static_f64[901]=(if self.scalar_static_bool[180]{self.scalar_static_f64[761]}else{0.0});
        self.scalar_static_f64[902]=(if self.scalar_static_bool[180]{self.scalar_static_f64[763]}else{0.0});
        self.scalar_static_f64[903]=(if self.scalar_static_bool[180]{self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_f64[904]=(self.scalar_static_f64[715]*self.scalar_static_f64[767]);
        self.scalar_static_f64[905]=(if self.scalar_static_bool[180]{self.scalar_static_f64[904]}else{0.0});
        self.scalar_static_f64[906]=(if self.scalar_static_bool[180]{self.scalar_static_f64[770]}else{0.0});
        self.scalar_static_f64[907]=(if self.scalar_static_bool[180]{self.scalar_static_f64[772]}else{0.0});
        self.scalar_static_f64[908]=(if self.scalar_static_bool[180]{self.scalar_static_f64[774]}else{0.0});
        self.scalar_static_f64[909]=(self.scalar_static_f64[715]*self.scalar_static_f64[776]);
        self.scalar_static_f64[910]=(if self.scalar_static_bool[180]{self.scalar_static_f64[909]}else{0.0});
        self.scalar_static_f64[911]=(if self.scalar_static_bool[180]{self.scalar_static_f64[779]}else{0.0});
        self.scalar_static_f64[912]=(-self.scalar_static_f64[902]);
        self.scalar_static_f64[913]=(self.scalar_static_f64[903]*self.scalar_static_f64[912]);
        self.scalar_static_f64[914]=(self.scalar_static_f64[883]*self.scalar_static_f64[905]);
        self.scalar_static_bool[189]=(1.0==self.scalar_static_f64[900]);
        self.scalar_static_f64[915]=(if self.scalar_static_bool[189]{1.0}else{0.0});
        self.scalar_static_bool[190]=(self.scalar_static_bool[180]&&(self.scalar_static_f64[915]!=0.0));
        self.scalar_static_bool[191]=(!(self.scalar_static_f64[915]!=0.0));
        self.scalar_static_bool[192]=(self.scalar_static_bool[180]&&self.scalar_static_bool[191]);
        self.scalar_static_f64[916]=(-self.scalar_static_f64[898]);
        self.scalar_static_f64[917]=(self.scalar_static_f64[916]-self.scalar_static_f64[903]);
        self.scalar_static_f64[918]=(self.scalar_static_f64[902]*self.scalar_static_f64[917]);
        self.scalar_static_bool[193]=(self.scalar_static_f64[900]>0.0);
        self.scalar_static_f64[919]=(if self.scalar_static_bool[193]{1.0}else{0.0});
        self.scalar_static_bool[194]=(self.scalar_static_bool[192]&&(self.scalar_static_f64[919]!=0.0));
        self.scalar_static_f64[920]=(self.scalar_static_f64[900]*self.scalar_static_f64[901]);
        self.scalar_static_f64[921]=(if self.scalar_static_bool[194]{self.scalar_static_f64[920]}else{0.0});
        self.scalar_static_bool[195]=(!(self.scalar_static_f64[919]!=0.0));
        self.scalar_static_bool[196]=(self.scalar_static_bool[192]&&self.scalar_static_bool[195]);
        self.scalar_static_f64[922]=(self.scalar_static_f64[899]*self.scalar_static_f64[899]);
        self.scalar_static_f64[923]=(1.0/self.scalar_static_f64[908]);
        self.scalar_static_f64[924]=(self.scalar_static_f64[896]*self.scalar_static_f64[910]);
        self.scalar_static_bool[197]=((self.scalar_static_f64[795]!=0.0)&&self.scalar_static_bool[180]);
        self.scalar_static_f64[925]=(if self.scalar_static_bool[197]{self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_f64[926]=(if self.scalar_static_bool[197]{self.scalar_static_f64[703]}else{0.0});
        self.scalar_static_f64[927]=(if self.scalar_static_bool[197]{1.0}else{0.0});
        self.scalar_static_f64[928]=(if self.scalar_static_bool[197]{self.scalar_static_f64[707]}else{0.0});
        self.scalar_static_f64[929]=(if self.scalar_static_bool[197]{self.scalar_static_f64[709]}else{0.0});
        self.scalar_static_f64[930]=(if self.scalar_static_bool[197]{self.scalar_static_f64[711]}else{0.0});
        self.scalar_static_f64[931]=(if self.scalar_static_bool[197]{self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[932]=(if self.scalar_static_bool[197]{self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[933]=(if self.scalar_static_bool[197]{self.scalar_static_f64[804]}else{0.0});
        self.scalar_static_f64[934]=(if self.scalar_static_bool[197]{self.scalar_static_f64[806]}else{0.0});
        self.scalar_static_f64[935]=(self.scalar_static_f64[715]*self.scalar_static_f64[808]);
        self.scalar_static_f64[936]=(if self.scalar_static_bool[197]{self.scalar_static_f64[935]}else{0.0});
        self.scalar_static_f64[937]=(if self.scalar_static_bool[197]{self.scalar_static_f64[811]}else{0.0});
        self.scalar_static_f64[938]=(if self.scalar_static_bool[197]{self.scalar_static_f64[731]}else{0.0});
        self.scalar_static_f64[939]=(if self.scalar_static_bool[197]{self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[940]=(if self.scalar_static_bool[197]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[941]=(-self.scalar_static_f64[939]);
        self.scalar_static_f64[942]=(-self.scalar_static_f64[929]);
        self.scalar_static_f64[943]=(self.scalar_static_f64[930]*self.scalar_static_f64[942]);
        self.scalar_static_f64[944]=(self.scalar_static_f64[931]*self.scalar_static_f64[940]);
        self.scalar_static_f64[945]=(self.scalar_static_f64[932]*self.scalar_static_f64[944]);
        self.scalar_static_f64[946]=(0.0*self.scalar_static_f64[945]);
        self.scalar_static_bool[198]=(1.0==self.scalar_static_f64[927]);
        self.scalar_static_f64[947]=(if self.scalar_static_bool[198]{1.0}else{0.0});
        self.scalar_static_bool[199]=(self.scalar_static_bool[197]&&(self.scalar_static_f64[947]!=0.0));
        self.scalar_static_bool[200]=(!(self.scalar_static_f64[947]!=0.0));
        self.scalar_static_bool[201]=(self.scalar_static_bool[197]&&self.scalar_static_bool[200]);
        self.scalar_static_f64[948]=(-self.scalar_static_f64[925]);
        self.scalar_static_f64[949]=(self.scalar_static_f64[948]-self.scalar_static_f64[930]);
        self.scalar_static_f64[950]=(self.scalar_static_f64[929]*self.scalar_static_f64[949]);
        self.scalar_static_bool[202]=(self.scalar_static_f64[927]>0.0);
        self.scalar_static_f64[951]=(if self.scalar_static_bool[202]{1.0}else{0.0});
        self.scalar_static_bool[203]=(self.scalar_static_bool[201]&&(self.scalar_static_f64[951]!=0.0));
        self.scalar_static_f64[952]=(self.scalar_static_f64[927]*self.scalar_static_f64[928]);
        self.scalar_static_f64[953]=(if self.scalar_static_bool[203]{self.scalar_static_f64[952]}else{0.0});
        self.scalar_static_bool[204]=(!(self.scalar_static_f64[951]!=0.0));
        self.scalar_static_bool[205]=(self.scalar_static_bool[201]&&self.scalar_static_bool[204]);
        self.scalar_static_f64[954]=(self.scalar_static_f64[926]*self.scalar_static_f64[926]);
        self.scalar_static_f64[955]=(1.0/self.scalar_static_f64[934]);
        self.scalar_static_f64[956]=(-self.scalar_static_f64[940]);
        self.scalar_static_f64[957]=(self.scalar_static_f64[931]*self.scalar_static_f64[956]);
        self.scalar_static_f64[958]=(self.scalar_static_f64[932]*self.scalar_static_f64[957]);
        self.scalar_static_f64[959]=(self.scalar_static_f64[936]*self.scalar_static_f64[958]);
        self.scalar_static_f64[960]=(if self.scalar_static_bool[197]{self.scalar_static_f64[755]}else{0.0});
        self.scalar_static_f64[961]=(if self.scalar_static_bool[197]{self.scalar_static_f64[757]}else{0.0});
        self.scalar_static_f64[962]=(if self.scalar_static_bool[197]{self.scalar_static_f64[761]}else{0.0});
        self.scalar_static_f64[963]=(if self.scalar_static_bool[197]{self.scalar_static_f64[763]}else{0.0});
        self.scalar_static_f64[964]=(if self.scalar_static_bool[197]{self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_f64[965]=(if self.scalar_static_bool[197]{self.scalar_static_f64[840]}else{0.0});
        self.scalar_static_f64[966]=(if self.scalar_static_bool[197]{self.scalar_static_f64[842]}else{0.0});
        self.scalar_static_f64[967]=(self.scalar_static_f64[715]*self.scalar_static_f64[844]);
        self.scalar_static_f64[968]=(if self.scalar_static_bool[197]{self.scalar_static_f64[967]}else{0.0});
        self.scalar_static_f64[969]=(if self.scalar_static_bool[197]{self.scalar_static_f64[847]}else{0.0});
        self.scalar_static_f64[970]=(-self.scalar_static_f64[963]);
        self.scalar_static_f64[971]=(self.scalar_static_f64[964]*self.scalar_static_f64[970]);
        self.scalar_static_f64[972]=(-self.scalar_static_f64[960]);
        self.scalar_static_f64[973]=(self.scalar_static_f64[972]-self.scalar_static_f64[964]);
        self.scalar_static_f64[974]=(self.scalar_static_f64[963]*self.scalar_static_f64[973]);
        self.scalar_static_f64[975]=(self.scalar_static_f64[927]*self.scalar_static_f64[962]);
        self.scalar_static_f64[976]=(if self.scalar_static_bool[203]{self.scalar_static_f64[975]}else{0.0});
        self.scalar_static_f64[977]=(self.scalar_static_f64[961]*self.scalar_static_f64[961]);
        self.scalar_static_f64[978]=(1.0/self.scalar_static_f64[966]);
        self.scalar_static_f64[979]=(self.scalar_static_f64[958]*self.scalar_static_f64[968]);
        self.scalar_static_f64[980]=p.p291;
        self.scalar_static_bool[206]=(1.0==self.scalar_static_f64[980]);
        self.scalar_static_f64[981]=(if self.scalar_static_bool[206]{1.0}else{0.0});
        self.scalar_static_f64[982]=p.p294;
        self.scalar_static_f64[983]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[982]}else{0.0});
        self.scalar_static_f64[984]=p.p296;
        self.scalar_static_f64[985]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[984]}else{0.0});
        self.scalar_static_f64[986]=p.p295;
        self.scalar_static_f64[987]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[986]}else{0.0});
        self.scalar_static_f64[988]=p.p292;
        self.scalar_static_f64[989]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[988]}else{0.0});
        self.scalar_static_f64[990]=(if (self.scalar_static_f64[981]!=0.0){4.0}else{0.0});
        self.scalar_static_f64[991]=(if (self.scalar_static_f64[981]!=0.0){600.0}else{0.0});
        self.scalar_static_f64[992]=p.p311;
        self.scalar_static_f64[993]=(1.0-self.scalar_static_f64[992]);
        self.scalar_static_f64[994]=(self.scalar_static_f64[6]*self.scalar_static_f64[993]);
        self.scalar_static_f64[995]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[994]}else{0.0});
        self.scalar_static_f64[996]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[997]=p.p293;
        self.scalar_static_f64[998]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[997]}else{0.0});
        self.scalar_static_f64[999]=p.p299;
        self.scalar_static_f64[1000]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[999]}else{0.0});
        self.scalar_static_f64[1001]=p.p300;
        self.scalar_static_f64[1002]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1001]}else{0.0});
        self.scalar_static_f64[1003]=p.p298;
        self.scalar_static_f64[1004]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1003]}else{0.0});
        self.scalar_static_f64[1005]=p.p297;
        self.scalar_static_f64[1006]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1005]}else{0.0});
        self.scalar_static_f64[1007]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1008]=(-self.scalar_static_f64[990]);
        self.scalar_static_f64[1009]=(self.scalar_static_f64[991]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1010]=(self.scalar_static_f64[995]*self.scalar_static_f64[1007]);
        self.scalar_static_f64[1011]=(self.scalar_static_f64[996]*self.scalar_static_f64[1010]);
        self.scalar_static_f64[1012]=(self.scalar_static_f64[998]*self.scalar_static_f64[1011]);
        self.scalar_static_bool[207]=(1.0==self.scalar_static_f64[987]);
        self.scalar_static_f64[1013]=(if self.scalar_static_bool[207]{1.0}else{0.0});
        self.scalar_static_bool[208]=((self.scalar_static_f64[981]!=0.0)&&(self.scalar_static_f64[1013]!=0.0));
        self.scalar_static_bool[209]=(!(self.scalar_static_f64[1013]!=0.0));
        self.scalar_static_bool[210]=((self.scalar_static_f64[981]!=0.0)&&self.scalar_static_bool[209]);
        self.scalar_static_f64[1014]=(-self.scalar_static_f64[983]);
        self.scalar_static_f64[1015]=(self.scalar_static_f64[1014]-self.scalar_static_f64[991]);
        self.scalar_static_f64[1016]=(self.scalar_static_f64[990]*self.scalar_static_f64[1015]);
        self.scalar_static_bool[211]=(self.scalar_static_f64[987]>0.0);
        self.scalar_static_f64[1017]=(if self.scalar_static_bool[211]{1.0}else{0.0});
        self.scalar_static_bool[212]=(self.scalar_static_bool[210]&&(self.scalar_static_f64[1017]!=0.0));
        self.scalar_static_f64[1018]=(self.scalar_static_f64[987]*self.scalar_static_f64[989]);
        self.scalar_static_f64[1019]=(if self.scalar_static_bool[212]{self.scalar_static_f64[1018]}else{0.0});
        self.scalar_static_bool[213]=(!(self.scalar_static_f64[1017]!=0.0));
        self.scalar_static_bool[214]=(self.scalar_static_bool[210]&&self.scalar_static_bool[213]);
        self.scalar_static_f64[1020]=(self.scalar_static_f64[985]*self.scalar_static_f64[985]);
        self.scalar_static_f64[1021]=(1.0/self.scalar_static_f64[1002]);
        self.scalar_static_f64[1022]=(-self.scalar_static_f64[1007]);
        self.scalar_static_f64[1023]=(self.scalar_static_f64[995]*self.scalar_static_f64[1022]);
        self.scalar_static_f64[1024]=(self.scalar_static_f64[996]*self.scalar_static_f64[1023]);
        self.scalar_static_f64[1025]=(self.scalar_static_f64[1004]*self.scalar_static_f64[1024]);
        self.scalar_static_f64[1026]=p.p301;
        self.scalar_static_bool[215]=(1.0==self.scalar_static_f64[1026]);
        self.scalar_static_f64[1027]=(if self.scalar_static_bool[215]{1.0}else{0.0});
        self.scalar_static_bool[216]=((self.scalar_static_f64[981]!=0.0)&&(self.scalar_static_f64[1027]!=0.0));
        self.scalar_static_f64[1028]=(if self.scalar_static_bool[216]{1.0}else{0.0});
        self.scalar_static_f64[1029]=(if self.scalar_static_bool[216]{10.0}else{0.0});
        self.scalar_static_f64[1030]=(if self.scalar_static_bool[216]{4.0}else{0.0});
        self.scalar_static_f64[1031]=(if self.scalar_static_bool[216]{600.0}else{0.0});
        self.scalar_static_f64[1032]=(if self.scalar_static_bool[216]{self.scalar_static_f64[994]}else{0.0});
        self.scalar_static_f64[1033]=(if self.scalar_static_bool[216]{self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[1034]=p.p304;
        self.scalar_static_f64[1035]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1034]}else{0.0});
        self.scalar_static_f64[1036]=p.p305;
        self.scalar_static_f64[1037]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1036]}else{0.0});
        self.scalar_static_f64[1038]=p.p303;
        self.scalar_static_f64[1039]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1038]}else{0.0});
        self.scalar_static_f64[1040]=p.p302;
        self.scalar_static_f64[1041]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1040]}else{0.0});
        self.scalar_static_f64[1042]=(if self.scalar_static_bool[216]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1043]=(-self.scalar_static_f64[1030]);
        self.scalar_static_f64[1044]=(self.scalar_static_f64[1031]*self.scalar_static_f64[1043]);
        self.scalar_static_f64[1045]=(self.scalar_static_f64[1032]*self.scalar_static_f64[1042]);
        self.scalar_static_f64[1046]=(self.scalar_static_f64[1033]*self.scalar_static_f64[1045]);
        self.scalar_static_f64[1047]=(0.0*self.scalar_static_f64[1046]);
        self.scalar_static_bool[217]=(1.0==self.scalar_static_f64[1028]);
        self.scalar_static_f64[1048]=(if self.scalar_static_bool[217]{1.0}else{0.0});
        self.scalar_static_bool[218]=(self.scalar_static_bool[216]&&(self.scalar_static_f64[1048]!=0.0));
        self.scalar_static_bool[219]=(!(self.scalar_static_f64[1048]!=0.0));
        self.scalar_static_bool[220]=(self.scalar_static_bool[216]&&self.scalar_static_bool[219]);
        self.scalar_static_f64[1049]=(-self.scalar_static_f64[1028]);
        self.scalar_static_f64[1050]=(self.scalar_static_f64[1049]-self.scalar_static_f64[1031]);
        self.scalar_static_f64[1051]=(self.scalar_static_f64[1030]*self.scalar_static_f64[1050]);
        self.scalar_static_bool[221]=(self.scalar_static_f64[1028]>0.0);
        self.scalar_static_f64[1052]=(if self.scalar_static_bool[221]{1.0}else{0.0});
        self.scalar_static_bool[222]=(self.scalar_static_bool[220]&&(self.scalar_static_f64[1052]!=0.0));
        self.scalar_static_f64[1053]=(0.0*self.scalar_static_f64[1028]);
        self.scalar_static_f64[1054]=(if self.scalar_static_bool[222]{self.scalar_static_f64[1053]}else{0.0});
        self.scalar_static_bool[223]=(!(self.scalar_static_f64[1052]!=0.0));
        self.scalar_static_bool[224]=(self.scalar_static_bool[220]&&self.scalar_static_bool[223]);
        self.scalar_static_f64[1055]=(self.scalar_static_f64[1029]*self.scalar_static_f64[1029]);
        self.scalar_static_f64[1056]=(1.0/self.scalar_static_f64[1037]);
        self.scalar_static_f64[1057]=(-self.scalar_static_f64[1042]);
        self.scalar_static_f64[1058]=(self.scalar_static_f64[1032]*self.scalar_static_f64[1057]);
        self.scalar_static_f64[1059]=(self.scalar_static_f64[1033]*self.scalar_static_f64[1058]);
        self.scalar_static_f64[1060]=(self.scalar_static_f64[1039]*self.scalar_static_f64[1059]);
        self.scalar_static_f64[1061]=p.p308;
        self.scalar_static_f64[1062]=p.p306;
        self.scalar_static_f64[1063]=(self.scalar_static_f64[1061]*self.scalar_static_f64[1062]);
        self.scalar_static_f64[1064]=(self.scalar_static_f64[118]*2.0);
        self.scalar_static_f64[1065]=p.p307;
        self.scalar_static_f64[1066]=(self.scalar_static_f64[1064]*self.scalar_static_f64[1065]);
        self.scalar_static_f64[1067]=(self.scalar_static_f64[6]*self.scalar_static_f64[1066]);
        self.scalar_static_f64[1068]=(self.scalar_static_f64[993]*self.scalar_static_f64[1067]);
        self.scalar_static_f64[1069]=(self.scalar_static_f64[8]*self.scalar_static_f64[1068]);
        self.scalar_static_f64[1070]=(self.scalar_static_f64[1062]*self.scalar_static_f64[1069]);
        self.scalar_static_f64[1071]=(1.0-self.scalar_static_f64[1061]);
        self.scalar_static_f64[1072]=(self.scalar_static_f64[1071]).sqrt();
        self.scalar_static_f64[1073]=(1.0-self.scalar_static_f64[1072]);
        self.scalar_static_f64[1074]=p.p309;
        self.scalar_static_bool[225]=(self.scalar_static_f64[1074]>=1.0);
        self.scalar_static_f64[1075]=(if self.scalar_static_bool[225]{1.0}else{0.0});
        self.scalar_static_f64[1076]=(2.0*self.scalar_static_f64[1062]);
        self.scalar_static_f64[1077]=(self.scalar_static_f64[1072]*self.scalar_static_f64[1076]);
        self.scalar_static_f64[1078]=(1.0/self.scalar_static_f64[1077]);
        self.scalar_static_bool[226]=(self.scalar_static_f64[1074]>=2.0);
        self.scalar_static_f64[1079]=(if self.scalar_static_bool[226]{1.0}else{0.0});
        self.scalar_static_f64[1080]=(4.0*self.scalar_static_f64[1062]);
        self.scalar_static_f64[1081]=(self.scalar_static_f64[1071]*self.scalar_static_f64[1080]);
        self.scalar_static_bool[227]=(self.scalar_static_f64[1074]>=3.0);
        self.scalar_static_f64[1082]=(if self.scalar_static_bool[227]{1.0}else{0.0});
        self.scalar_static_f64[1083]=(self.scalar_static_f64[1071]*self.scalar_static_f64[1076]);
        self.scalar_static_bool[228]=(self.scalar_static_f64[1074]>=4.0);
        self.scalar_static_f64[1084]=(if self.scalar_static_bool[228]{1.0}else{0.0});
        self.scalar_static_f64[1085]=(self.scalar_static_f64[1062]*8.0);
        self.scalar_static_f64[1086]=(self.scalar_static_f64[1071]*self.scalar_static_f64[1085]);
        self.scalar_static_bool[229]=(self.scalar_static_f64[1074]>=5.0);
        self.scalar_static_f64[1087]=(if self.scalar_static_bool[229]{1.0}else{0.0});
        self.scalar_static_f64[1088]=(10.0*self.scalar_static_f64[1062]);
        self.scalar_static_f64[1089]=(self.scalar_static_f64[1071]*self.scalar_static_f64[1088]);
        self.scalar_static_f64[1090]=p.p310;
        self.scalar_static_bool[230]=(0.0!=self.scalar_static_f64[1090]);
        self.scalar_static_bool[231]=(0.0!=self.scalar_static_f64[992]);
        self.scalar_static_bool[232]=(self.scalar_static_bool[230]&&self.scalar_static_bool[231]);
        self.scalar_static_f64[1091]=(if self.scalar_static_bool[232]{1.0}else{0.0});
        self.scalar_static_bool[233]=((self.scalar_static_f64[981]!=0.0)&&(self.scalar_static_f64[1091]!=0.0));
        self.scalar_static_f64[1092]=(self.scalar_static_f64[6]*self.scalar_static_f64[992]);
        self.scalar_static_f64[1093]=(self.scalar_static_f64[8]*self.scalar_static_f64[1092]);
        self.scalar_static_f64[1094]=(self.scalar_static_f64[1090]/self.scalar_static_f64[1093]);
        self.scalar_static_f64[1095]=(if self.scalar_static_bool[233]{self.scalar_static_f64[1094]}else{0.0});
        self.scalar_static_f64[1096]=p.p312;
        self.scalar_static_bool[234]=(1.0==self.scalar_static_f64[1096]);
        self.scalar_static_f64[1097]=(if self.scalar_static_bool[234]{1.0}else{0.0});
        self.scalar_static_f64[1098]=p.p313;
        self.scalar_static_bool[235]=(0.0==self.scalar_static_f64[1098]);
        self.scalar_static_f64[1099]=(if self.scalar_static_bool[235]{1.0}else{0.0});
        self.scalar_static_bool[236]=((self.scalar_static_f64[1097]!=0.0)&&(self.scalar_static_f64[1099]!=0.0));
        self.scalar_static_f64[1100]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_f64[1101]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[703]}else{0.0});
        self.scalar_static_f64[1102]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[705]}else{0.0});
        self.scalar_static_f64[1103]=p.p317;
        self.scalar_static_f64[1104]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1103]}else{0.0});
        self.scalar_static_f64[1105]=p.p316;
        self.scalar_static_f64[1106]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1105]}else{0.0});
        self.scalar_static_f64[1107]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[6]}else{0.0});
        self.scalar_static_f64[1108]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[1109]=p.p314;
        self.scalar_static_f64[1110]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1109]}else{0.0});
        self.scalar_static_f64[1111]=(if (self.scalar_static_f64[1097]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1112]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[722]}else{0.0});
        self.scalar_static_f64[1113]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[724]}else{0.0});
        self.scalar_static_f64[1114]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[729]}else{0.0});
        self.scalar_static_f64[1115]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[733]}else{0.0});
        self.scalar_static_f64[1116]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1117]=(-self.scalar_static_f64[1115]);
        self.scalar_static_f64[1118]=(-self.scalar_static_f64[1104]);
        self.scalar_static_f64[1119]=(self.scalar_static_f64[1106]*self.scalar_static_f64[1118]);
        self.scalar_static_f64[1120]=(self.scalar_static_f64[1107]*self.scalar_static_f64[1116]);
        self.scalar_static_f64[1121]=(self.scalar_static_f64[1108]*self.scalar_static_f64[1120]);
        self.scalar_static_f64[1122]=(self.scalar_static_f64[1110]*self.scalar_static_f64[1121]);
        self.scalar_static_bool[237]=(1.0==self.scalar_static_f64[1102]);
        self.scalar_static_f64[1123]=(if self.scalar_static_bool[237]{1.0}else{0.0});
        self.scalar_static_bool[238]=((self.scalar_static_f64[1097]!=0.0)&&(self.scalar_static_f64[1123]!=0.0));
        self.scalar_static_bool[239]=(!(self.scalar_static_f64[1123]!=0.0));
        self.scalar_static_bool[240]=((self.scalar_static_f64[1097]!=0.0)&&self.scalar_static_bool[239]);
        self.scalar_static_f64[1124]=(-self.scalar_static_f64[1100]);
        self.scalar_static_f64[1125]=(self.scalar_static_f64[1124]-self.scalar_static_f64[1106]);
        self.scalar_static_f64[1126]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1125]);
        self.scalar_static_bool[241]=(self.scalar_static_f64[1102]>0.0);
        self.scalar_static_f64[1127]=(if self.scalar_static_bool[241]{1.0}else{0.0});
        self.scalar_static_bool[242]=(self.scalar_static_bool[240]&&(self.scalar_static_f64[1127]!=0.0));
        self.scalar_static_f64[1128]=(0.0*self.scalar_static_f64[1102]);
        self.scalar_static_f64[1129]=(if self.scalar_static_bool[242]{self.scalar_static_f64[1128]}else{0.0});
        self.scalar_static_bool[243]=(!(self.scalar_static_f64[1127]!=0.0));
        self.scalar_static_bool[244]=(self.scalar_static_bool[240]&&self.scalar_static_bool[243]);
        self.scalar_static_f64[1130]=(self.scalar_static_f64[1101]*self.scalar_static_f64[1101]);
        self.scalar_static_f64[1131]=(1.0/self.scalar_static_f64[1113]);
        self.scalar_static_f64[1132]=(-self.scalar_static_f64[1116]);
        self.scalar_static_f64[1133]=(self.scalar_static_f64[1107]*self.scalar_static_f64[1132]);
        self.scalar_static_f64[1134]=(self.scalar_static_f64[1108]*self.scalar_static_f64[1133]);
        self.scalar_static_f64[1135]=(0.0*self.scalar_static_f64[1134]);
        self.scalar_static_f64[1136]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[755]}else{0.0});
        self.scalar_static_f64[1137]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[757]}else{0.0});
        self.scalar_static_f64[1138]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[759]}else{0.0});
        self.scalar_static_f64[1139]=p.p319;
        self.scalar_static_f64[1140]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1139]}else{0.0});
        self.scalar_static_f64[1141]=p.p318;
        self.scalar_static_f64[1142]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1141]}else{0.0});
        self.scalar_static_f64[1143]=p.p315;
        self.scalar_static_f64[1144]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1143]}else{0.0});
        self.scalar_static_f64[1145]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[772]}else{0.0});
        self.scalar_static_f64[1146]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[774]}else{0.0});
        self.scalar_static_f64[1147]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[779]}else{0.0});
        self.scalar_static_f64[1148]=(-self.scalar_static_f64[1140]);
        self.scalar_static_f64[1149]=(self.scalar_static_f64[1142]*self.scalar_static_f64[1148]);
        self.scalar_static_f64[1150]=(self.scalar_static_f64[1121]*self.scalar_static_f64[1144]);
        self.scalar_static_bool[245]=(1.0==self.scalar_static_f64[1138]);
        self.scalar_static_f64[1151]=(if self.scalar_static_bool[245]{1.0}else{0.0});
        self.scalar_static_bool[246]=((self.scalar_static_f64[1097]!=0.0)&&(self.scalar_static_f64[1151]!=0.0));
        self.scalar_static_bool[247]=(!(self.scalar_static_f64[1151]!=0.0));
        self.scalar_static_bool[248]=((self.scalar_static_f64[1097]!=0.0)&&self.scalar_static_bool[247]);
        self.scalar_static_f64[1152]=(-self.scalar_static_f64[1136]);
        self.scalar_static_f64[1153]=(self.scalar_static_f64[1152]-self.scalar_static_f64[1142]);
        self.scalar_static_f64[1154]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1153]);
        self.scalar_static_bool[249]=(self.scalar_static_f64[1138]>0.0);
        self.scalar_static_f64[1155]=(if self.scalar_static_bool[249]{1.0}else{0.0});
        self.scalar_static_bool[250]=(self.scalar_static_bool[248]&&(self.scalar_static_f64[1155]!=0.0));
        self.scalar_static_f64[1156]=(0.0*self.scalar_static_f64[1138]);
        self.scalar_static_f64[1157]=(if self.scalar_static_bool[250]{self.scalar_static_f64[1156]}else{0.0});
        self.scalar_static_bool[251]=(!(self.scalar_static_f64[1155]!=0.0));
        self.scalar_static_bool[252]=(self.scalar_static_bool[248]&&self.scalar_static_bool[251]);
        self.scalar_static_f64[1158]=(self.scalar_static_f64[1137]*self.scalar_static_f64[1137]);
        self.scalar_static_f64[1159]=(1.0/self.scalar_static_f64[1146]);
        self.scalar_static_bool[253]=(self.scalar_static_f64[44]>=self.scalar_static_f64[28]);
        self.scalar_static_bool[254]=(self.scalar_static_f64[44]>0.0);
        self.scalar_static_bool[255]=(self.scalar_static_bool[253]&&self.scalar_static_bool[254]);
        self.scalar_static_f64[1160]=(if self.scalar_static_bool[255]{1.0}else{0.0});
        self.scalar_static_bool[256]=(self.scalar_static_f64[48]>=self.scalar_static_f64[28]);
        self.scalar_static_bool[257]=(self.scalar_static_f64[48]>0.0);
        self.scalar_static_bool[258]=(self.scalar_static_bool[256]&&self.scalar_static_bool[257]);
        self.scalar_static_f64[1161]=(if self.scalar_static_bool[258]{1.0}else{0.0});
        self.scalar_static_f64[1162]=p.p27;
        self.scalar_static_f64[1163]=p.p28;
        self.scalar_static_f64[1164]=p.p320;
        self.scalar_static_bool[259]=(self.scalar_static_f64[1164]>0.0);
        self.scalar_static_f64[1165]=(if self.scalar_static_bool[259]{1.0}else{0.0});
        self.scalar_static_f64[1166]=p.p329;
        self.scalar_static_f64[1167]=p.p330;
        self.scalar_static_f64[1168]=p.p332;
        self.scalar_static_f64[1169]=p.p346;
        self.scalar_static_f64[1170]=p.p340;
        self.scalar_static_f64[1171]=p.p339;
        self.scalar_static_f64[1172]=p.p341;
        self.scalar_static_f64[1173]=p.p342;
        self.scalar_static_f64[1174]=p.p344;
        self.scalar_static_f64[1175]=p.p343;
        self.scalar_static_f64[1176]=p.p345;
        self.scalar_static_f64[1177]=p.p355;
        self.scalar_static_bool[260]=(!(self.scalar_static_f64[698]!=0.0));
        self.scalar_static_f64[1178]=p.p323;
        self.scalar_static_f64[1179]=(self.scalar_static_f64[1178]/3.0);
        self.scalar_static_bool[261]=(!(self.scalar_static_f64[1099]!=0.0));
        self.scalar_static_bool[262]=((self.scalar_static_f64[1097]!=0.0)&&self.scalar_static_bool[261]);
        self.scalar_static_f64[1180]=p.p321;
        self.scalar_static_f64[1181]=(-self.scalar_static_f64[118]);
        self.scalar_static_f64[1182]=(self.scalar_static_f64[118]+self.scalar_static_f64[118]);
        self.scalar_static_f64[1183]=(self.scalar_static_f64[118]-self.scalar_static_f64[118]);
        self.scalar_static_f64[1184]=(self.scalar_static_f64[122]*self.scalar_static_f64[1181]);
        self.scalar_static_f64[1185]=(self.scalar_static_f64[118]*self.scalar_static_f64[122]);
        self.scalar_static_f64[1186]=(self.scalar_static_f64[122]*self.scalar_static_f64[1183]);
        self.scalar_static_f64[1187]=(-self.scalar_static_f64[134]);
        self.scalar_static_f64[1188]=(1.0/self.scalar_static_f64[135]);
        self.scalar_static_f64[1189]=(-1.0/self.scalar_static_f64[135]);
        self.scalar_static_f64[1190]=(self.scalar_static_f64[1187]/self.scalar_static_f64[135]);
        self.scalar_static_f64[1191]=(5.184705528587072e21*self.scalar_static_f64[1188]);
        self.scalar_static_f64[1192]=(5.184705528587072e21*self.scalar_static_f64[1189]);
        self.scalar_static_f64[1193]=(5.184705528587072e21*self.scalar_static_f64[1190]);
        self.scalar_static_f64[1194]=(if (self.scalar_static_f64[131]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1195]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_f64[1196]=(if (self.scalar_static_f64[142]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1197]=(if (self.scalar_static_f64[142]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1198]=(if self.scalar_static_bool[18]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1199]=(if self.scalar_static_bool[18]{0.0}else{self.scalar_static_f64[1196]});
        self.scalar_static_f64[1200]=(if self.scalar_static_bool[18]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1197]});
        self.scalar_static_f64[1201]=(if (self.scalar_static_f64[144]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1202]=(if (self.scalar_static_f64[144]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1203]=(if self.scalar_static_bool[20]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1204]=(if self.scalar_static_bool[20]{0.0}else{self.scalar_static_f64[1201]});
        self.scalar_static_f64[1205]=(if self.scalar_static_bool[20]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1202]});
        self.scalar_static_f64[1206]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1207]=(if (self.scalar_static_f64[146]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1208]=(if self.scalar_static_bool[22]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1209]=(if self.scalar_static_bool[22]{0.0}else{self.scalar_static_f64[1206]});
        self.scalar_static_f64[1210]=(if self.scalar_static_bool[22]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1207]});
        self.scalar_static_f64[1211]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1212]=(if (self.scalar_static_f64[148]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1213]=(if self.scalar_static_bool[24]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1214]=(if self.scalar_static_bool[24]{0.0}else{self.scalar_static_f64[1211]});
        self.scalar_static_f64[1215]=(if self.scalar_static_bool[24]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1212]});
        self.scalar_static_f64[1216]=(if (self.scalar_static_f64[150]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1217]=(if (self.scalar_static_f64[150]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1218]=(if self.scalar_static_bool[26]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1219]=(if self.scalar_static_bool[26]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1216]});
        self.scalar_static_f64[1220]=(if self.scalar_static_bool[26]{0.0}else{self.scalar_static_f64[1217]});
        self.scalar_static_f64[1221]=(if (self.scalar_static_f64[152]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1222]=(if (self.scalar_static_f64[152]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1223]=(if self.scalar_static_bool[28]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1224]=(if self.scalar_static_bool[28]{0.0}else{self.scalar_static_f64[1221]});
        self.scalar_static_f64[1225]=(if self.scalar_static_bool[28]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1222]});
        self.scalar_static_f64[1226]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1227]=(if (self.scalar_static_f64[154]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1228]=(if self.scalar_static_bool[30]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1229]=(if self.scalar_static_bool[30]{0.0}else{self.scalar_static_f64[1226]});
        self.scalar_static_f64[1230]=(if self.scalar_static_bool[30]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1227]});
        self.scalar_static_f64[1231]=(if (self.scalar_static_f64[156]!=0.0){self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1232]=(if (self.scalar_static_f64[156]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1233]=(if self.scalar_static_bool[32]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1234]=(if self.scalar_static_bool[32]{0.0}else{self.scalar_static_f64[1231]});
        self.scalar_static_f64[1235]=(if self.scalar_static_bool[32]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1232]});
        self.scalar_static_f64[1236]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1233]}else{0.0});
        self.scalar_static_f64[1237]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1234]}else{0.0});
        self.scalar_static_f64[1238]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1235]}else{0.0});
        self.scalar_static_f64[1239]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1240]=(self.scalar_static_f64[122]*self.scalar_static_f64[1239]);
        self.scalar_static_f64[1241]=(self.scalar_static_f64[122]*self.scalar_static_f64[200]);
        self.scalar_static_f64[1242]=(self.scalar_static_f64[1238]-self.scalar_static_f64[1239]);
        self.scalar_static_f64[1243]=(-self.scalar_static_f64[200]);
        self.scalar_static_f64[1244]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1236]}else{0.0});
        self.scalar_static_f64[1245]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1237]}else{0.0});
        self.scalar_static_f64[1246]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1242]}else{0.0});
        self.scalar_static_f64[1247]=(if (self.scalar_static_f64[159]!=0.0){self.scalar_static_f64[1243]}else{0.0});
        self.scalar_static_f64[1248]=(self.scalar_static_f64[194]-1.0);
        self.scalar_static_f64[1249]=(self.scalar_static_f64[184]-1.0);
        self.scalar_static_f64[1250]=(self.scalar_static_f64[203]-1.0);
        self.scalar_static_f64[1251]=(self.scalar_static_f64[1236]+self.scalar_static_f64[1244]);
        self.scalar_static_f64[1252]=(self.scalar_static_f64[1237]+self.scalar_static_f64[1245]);
        self.scalar_static_f64[1253]=(self.scalar_static_f64[1238]+self.scalar_static_f64[1246]);
        self.scalar_static_f64[1254]=(self.scalar_static_f64[1236]-self.scalar_static_f64[1244]);
        self.scalar_static_f64[1255]=(self.scalar_static_f64[1237]-self.scalar_static_f64[1245]);
        self.scalar_static_f64[1256]=(self.scalar_static_f64[1238]-self.scalar_static_f64[1246]);
        self.scalar_static_f64[1257]=(-self.scalar_static_f64[1247]);
        self.scalar_static_f64[1258]=(self.scalar_static_f64[122]*self.scalar_static_f64[1254]);
        self.scalar_static_f64[1259]=(self.scalar_static_f64[122]*self.scalar_static_f64[1255]);
        self.scalar_static_f64[1260]=(self.scalar_static_f64[122]*self.scalar_static_f64[1256]);
        self.scalar_static_f64[1261]=(self.scalar_static_f64[122]*self.scalar_static_f64[1257]);
        self.scalar_static_f64[1262]=(-self.scalar_static_f64[1239]);
        self.scalar_static_f64[1263]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1228]}else{0.0});
        self.scalar_static_f64[1264]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1229]}else{0.0});
        self.scalar_static_f64[1265]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1230]}else{0.0});
        self.scalar_static_f64[1266]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1267]=(self.scalar_static_f64[122]*self.scalar_static_f64[1266]);
        self.scalar_static_f64[1268]=(self.scalar_static_f64[122]*self.scalar_static_f64[257]);
        self.scalar_static_f64[1269]=(self.scalar_static_f64[1265]-self.scalar_static_f64[1266]);
        self.scalar_static_f64[1270]=(-self.scalar_static_f64[257]);
        self.scalar_static_f64[1271]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1263]}else{0.0});
        self.scalar_static_f64[1272]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1264]}else{0.0});
        self.scalar_static_f64[1273]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1269]}else{0.0});
        self.scalar_static_f64[1274]=(if (self.scalar_static_f64[220]!=0.0){self.scalar_static_f64[1270]}else{0.0});
        self.scalar_static_f64[1275]=(self.scalar_static_f64[253]-1.0);
        self.scalar_static_f64[1276]=(self.scalar_static_f64[245]-1.0);
        self.scalar_static_f64[1277]=(self.scalar_static_f64[260]-1.0);
        self.scalar_static_f64[1278]=(self.scalar_static_f64[1263]+self.scalar_static_f64[1271]);
        self.scalar_static_f64[1279]=(self.scalar_static_f64[1264]+self.scalar_static_f64[1272]);
        self.scalar_static_f64[1280]=(self.scalar_static_f64[1265]+self.scalar_static_f64[1273]);
        self.scalar_static_f64[1281]=(self.scalar_static_f64[1263]-self.scalar_static_f64[1271]);
        self.scalar_static_f64[1282]=(self.scalar_static_f64[1264]-self.scalar_static_f64[1272]);
        self.scalar_static_f64[1283]=(self.scalar_static_f64[1265]-self.scalar_static_f64[1273]);
        self.scalar_static_f64[1284]=(-self.scalar_static_f64[1274]);
        self.scalar_static_f64[1285]=(self.scalar_static_f64[122]*self.scalar_static_f64[1281]);
        self.scalar_static_f64[1286]=(self.scalar_static_f64[122]*self.scalar_static_f64[1282]);
        self.scalar_static_f64[1287]=(self.scalar_static_f64[122]*self.scalar_static_f64[1283]);
        self.scalar_static_f64[1288]=(self.scalar_static_f64[122]*self.scalar_static_f64[1284]);
        self.scalar_static_f64[1289]=(-self.scalar_static_f64[1266]);
        self.scalar_static_f64[1290]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1223]}else{0.0});
        self.scalar_static_f64[1291]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1224]}else{0.0});
        self.scalar_static_f64[1292]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1225]}else{0.0});
        self.scalar_static_f64[1293]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1294]=(self.scalar_static_f64[122]*self.scalar_static_f64[1293]);
        self.scalar_static_f64[1295]=(self.scalar_static_f64[122]*self.scalar_static_f64[311]);
        self.scalar_static_f64[1296]=(self.scalar_static_f64[1292]-self.scalar_static_f64[1293]);
        self.scalar_static_f64[1297]=(-self.scalar_static_f64[311]);
        self.scalar_static_f64[1298]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1290]}else{0.0});
        self.scalar_static_f64[1299]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1291]}else{0.0});
        self.scalar_static_f64[1300]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1296]}else{0.0});
        self.scalar_static_f64[1301]=(if (self.scalar_static_f64[274]!=0.0){self.scalar_static_f64[1297]}else{0.0});
        self.scalar_static_f64[1302]=(self.scalar_static_f64[307]-1.0);
        self.scalar_static_f64[1303]=(self.scalar_static_f64[299]-1.0);
        self.scalar_static_f64[1304]=(self.scalar_static_f64[314]-1.0);
        self.scalar_static_f64[1305]=(self.scalar_static_f64[1290]+self.scalar_static_f64[1298]);
        self.scalar_static_f64[1306]=(self.scalar_static_f64[1291]+self.scalar_static_f64[1299]);
        self.scalar_static_f64[1307]=(self.scalar_static_f64[1292]+self.scalar_static_f64[1300]);
        self.scalar_static_f64[1308]=(self.scalar_static_f64[1290]-self.scalar_static_f64[1298]);
        self.scalar_static_f64[1309]=(self.scalar_static_f64[1291]-self.scalar_static_f64[1299]);
        self.scalar_static_f64[1310]=(self.scalar_static_f64[1292]-self.scalar_static_f64[1300]);
        self.scalar_static_f64[1311]=(-self.scalar_static_f64[1301]);
        self.scalar_static_f64[1312]=(self.scalar_static_f64[122]*self.scalar_static_f64[1308]);
        self.scalar_static_f64[1313]=(self.scalar_static_f64[122]*self.scalar_static_f64[1309]);
        self.scalar_static_f64[1314]=(self.scalar_static_f64[122]*self.scalar_static_f64[1310]);
        self.scalar_static_f64[1315]=(self.scalar_static_f64[122]*self.scalar_static_f64[1311]);
        self.scalar_static_f64[1316]=(-self.scalar_static_f64[1293]);
        self.scalar_static_f64[1317]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1218]}else{0.0});
        self.scalar_static_f64[1318]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1219]}else{0.0});
        self.scalar_static_f64[1319]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1220]}else{0.0});
        self.scalar_static_f64[1320]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1321]=(self.scalar_static_f64[122]*self.scalar_static_f64[1320]);
        self.scalar_static_f64[1322]=(self.scalar_static_f64[122]*self.scalar_static_f64[365]);
        self.scalar_static_f64[1323]=(self.scalar_static_f64[1318]-self.scalar_static_f64[1320]);
        self.scalar_static_f64[1324]=(-self.scalar_static_f64[365]);
        self.scalar_static_f64[1325]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1317]}else{0.0});
        self.scalar_static_f64[1326]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1323]}else{0.0});
        self.scalar_static_f64[1327]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1319]}else{0.0});
        self.scalar_static_f64[1328]=(if (self.scalar_static_f64[328]!=0.0){self.scalar_static_f64[1324]}else{0.0});
        self.scalar_static_f64[1329]=(self.scalar_static_f64[361]-1.0);
        self.scalar_static_f64[1330]=(self.scalar_static_f64[353]-1.0);
        self.scalar_static_f64[1331]=(self.scalar_static_f64[368]-1.0);
        self.scalar_static_f64[1332]=(self.scalar_static_f64[1317]+self.scalar_static_f64[1325]);
        self.scalar_static_f64[1333]=(self.scalar_static_f64[1318]+self.scalar_static_f64[1326]);
        self.scalar_static_f64[1334]=(self.scalar_static_f64[1319]+self.scalar_static_f64[1327]);
        self.scalar_static_f64[1335]=(self.scalar_static_f64[1317]-self.scalar_static_f64[1325]);
        self.scalar_static_f64[1336]=(self.scalar_static_f64[1318]-self.scalar_static_f64[1326]);
        self.scalar_static_f64[1337]=(self.scalar_static_f64[1319]-self.scalar_static_f64[1327]);
        self.scalar_static_f64[1338]=(-self.scalar_static_f64[1328]);
        self.scalar_static_f64[1339]=(self.scalar_static_f64[122]*self.scalar_static_f64[1335]);
        self.scalar_static_f64[1340]=(self.scalar_static_f64[122]*self.scalar_static_f64[1336]);
        self.scalar_static_f64[1341]=(self.scalar_static_f64[122]*self.scalar_static_f64[1337]);
        self.scalar_static_f64[1342]=(self.scalar_static_f64[122]*self.scalar_static_f64[1338]);
        self.scalar_static_f64[1343]=(-self.scalar_static_f64[1320]);
        self.scalar_static_f64[1344]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1198]}else{0.0});
        self.scalar_static_f64[1345]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1199]}else{0.0});
        self.scalar_static_f64[1346]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1200]}else{0.0});
        self.scalar_static_f64[1347]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1348]=(self.scalar_static_f64[122]*self.scalar_static_f64[419]);
        self.scalar_static_f64[1349]=(self.scalar_static_f64[122]*self.scalar_static_f64[1347]);
        self.scalar_static_f64[1350]=(-self.scalar_static_f64[419]);
        self.scalar_static_f64[1351]=(self.scalar_static_f64[1346]-self.scalar_static_f64[1347]);
        self.scalar_static_f64[1352]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1344]}else{0.0});
        self.scalar_static_f64[1353]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1345]}else{0.0});
        self.scalar_static_f64[1354]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1350]}else{0.0});
        self.scalar_static_f64[1355]=(if (self.scalar_static_f64[382]!=0.0){self.scalar_static_f64[1351]}else{0.0});
        self.scalar_static_f64[1356]=(self.scalar_static_f64[415]-1.0);
        self.scalar_static_f64[1357]=(self.scalar_static_f64[407]-1.0);
        self.scalar_static_f64[1358]=(self.scalar_static_f64[422]-1.0);
        self.scalar_static_f64[1359]=(self.scalar_static_f64[1344]+self.scalar_static_f64[1352]);
        self.scalar_static_f64[1360]=(self.scalar_static_f64[1345]+self.scalar_static_f64[1353]);
        self.scalar_static_f64[1361]=(self.scalar_static_f64[1346]+self.scalar_static_f64[1355]);
        self.scalar_static_f64[1362]=(self.scalar_static_f64[1344]-self.scalar_static_f64[1352]);
        self.scalar_static_f64[1363]=(self.scalar_static_f64[1345]-self.scalar_static_f64[1353]);
        self.scalar_static_f64[1364]=(-self.scalar_static_f64[1354]);
        self.scalar_static_f64[1365]=(self.scalar_static_f64[1346]-self.scalar_static_f64[1355]);
        self.scalar_static_f64[1366]=(self.scalar_static_f64[122]*self.scalar_static_f64[1362]);
        self.scalar_static_f64[1367]=(self.scalar_static_f64[122]*self.scalar_static_f64[1363]);
        self.scalar_static_f64[1368]=(self.scalar_static_f64[122]*self.scalar_static_f64[1364]);
        self.scalar_static_f64[1369]=(self.scalar_static_f64[122]*self.scalar_static_f64[1365]);
        self.scalar_static_f64[1370]=(-self.scalar_static_f64[1347]);
        self.scalar_static_f64[1371]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1203]}else{0.0});
        self.scalar_static_f64[1372]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1204]}else{0.0});
        self.scalar_static_f64[1373]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1205]}else{0.0});
        self.scalar_static_f64[1374]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1375]=(self.scalar_static_f64[122]*self.scalar_static_f64[473]);
        self.scalar_static_f64[1376]=(self.scalar_static_f64[122]*self.scalar_static_f64[1374]);
        self.scalar_static_f64[1377]=(-self.scalar_static_f64[473]);
        self.scalar_static_f64[1378]=(self.scalar_static_f64[1373]-self.scalar_static_f64[1374]);
        self.scalar_static_f64[1379]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1371]}else{0.0});
        self.scalar_static_f64[1380]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1372]}else{0.0});
        self.scalar_static_f64[1381]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1377]}else{0.0});
        self.scalar_static_f64[1382]=(if (self.scalar_static_f64[436]!=0.0){self.scalar_static_f64[1378]}else{0.0});
        self.scalar_static_f64[1383]=(self.scalar_static_f64[469]-1.0);
        self.scalar_static_f64[1384]=(self.scalar_static_f64[461]-1.0);
        self.scalar_static_f64[1385]=(self.scalar_static_f64[476]-1.0);
        self.scalar_static_f64[1386]=(self.scalar_static_f64[1371]+self.scalar_static_f64[1379]);
        self.scalar_static_f64[1387]=(self.scalar_static_f64[1372]+self.scalar_static_f64[1380]);
        self.scalar_static_f64[1388]=(self.scalar_static_f64[1373]+self.scalar_static_f64[1382]);
        self.scalar_static_f64[1389]=(self.scalar_static_f64[1371]-self.scalar_static_f64[1379]);
        self.scalar_static_f64[1390]=(self.scalar_static_f64[1372]-self.scalar_static_f64[1380]);
        self.scalar_static_f64[1391]=(-self.scalar_static_f64[1381]);
        self.scalar_static_f64[1392]=(self.scalar_static_f64[1373]-self.scalar_static_f64[1382]);
        self.scalar_static_f64[1393]=(self.scalar_static_f64[122]*self.scalar_static_f64[1389]);
        self.scalar_static_f64[1394]=(self.scalar_static_f64[122]*self.scalar_static_f64[1390]);
        self.scalar_static_f64[1395]=(self.scalar_static_f64[122]*self.scalar_static_f64[1391]);
        self.scalar_static_f64[1396]=(self.scalar_static_f64[122]*self.scalar_static_f64[1392]);
        self.scalar_static_f64[1397]=(-self.scalar_static_f64[1374]);
        self.scalar_static_f64[1398]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1208]}else{0.0});
        self.scalar_static_f64[1399]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1209]}else{0.0});
        self.scalar_static_f64[1400]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1210]}else{0.0});
        self.scalar_static_f64[1401]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1402]=(self.scalar_static_f64[122]*self.scalar_static_f64[527]);
        self.scalar_static_f64[1403]=(self.scalar_static_f64[122]*self.scalar_static_f64[1401]);
        self.scalar_static_f64[1404]=(-self.scalar_static_f64[527]);
        self.scalar_static_f64[1405]=(self.scalar_static_f64[1400]-self.scalar_static_f64[1401]);
        self.scalar_static_f64[1406]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1398]}else{0.0});
        self.scalar_static_f64[1407]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1399]}else{0.0});
        self.scalar_static_f64[1408]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1404]}else{0.0});
        self.scalar_static_f64[1409]=(if (self.scalar_static_f64[490]!=0.0){self.scalar_static_f64[1405]}else{0.0});
        self.scalar_static_f64[1410]=(self.scalar_static_f64[523]-1.0);
        self.scalar_static_f64[1411]=(self.scalar_static_f64[515]-1.0);
        self.scalar_static_f64[1412]=(self.scalar_static_f64[530]-1.0);
        self.scalar_static_f64[1413]=(self.scalar_static_f64[1398]+self.scalar_static_f64[1406]);
        self.scalar_static_f64[1414]=(self.scalar_static_f64[1399]+self.scalar_static_f64[1407]);
        self.scalar_static_f64[1415]=(self.scalar_static_f64[1400]+self.scalar_static_f64[1409]);
        self.scalar_static_f64[1416]=(self.scalar_static_f64[1398]-self.scalar_static_f64[1406]);
        self.scalar_static_f64[1417]=(self.scalar_static_f64[1399]-self.scalar_static_f64[1407]);
        self.scalar_static_f64[1418]=(-self.scalar_static_f64[1408]);
        self.scalar_static_f64[1419]=(self.scalar_static_f64[1400]-self.scalar_static_f64[1409]);
        self.scalar_static_f64[1420]=(self.scalar_static_f64[122]*self.scalar_static_f64[1416]);
        self.scalar_static_f64[1421]=(self.scalar_static_f64[122]*self.scalar_static_f64[1417]);
        self.scalar_static_f64[1422]=(self.scalar_static_f64[122]*self.scalar_static_f64[1418]);
        self.scalar_static_f64[1423]=(self.scalar_static_f64[122]*self.scalar_static_f64[1419]);
        self.scalar_static_f64[1424]=(-self.scalar_static_f64[1401]);
        self.scalar_static_f64[1425]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1213]}else{0.0});
        self.scalar_static_f64[1426]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1214]}else{0.0});
        self.scalar_static_f64[1427]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1215]}else{0.0});
        self.scalar_static_f64[1428]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1429]=(self.scalar_static_f64[122]*self.scalar_static_f64[581]);
        self.scalar_static_f64[1430]=(self.scalar_static_f64[122]*self.scalar_static_f64[1428]);
        self.scalar_static_f64[1431]=(-self.scalar_static_f64[581]);
        self.scalar_static_f64[1432]=(self.scalar_static_f64[1427]-self.scalar_static_f64[1428]);
        self.scalar_static_f64[1433]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1425]}else{0.0});
        self.scalar_static_f64[1434]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1426]}else{0.0});
        self.scalar_static_f64[1435]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1431]}else{0.0});
        self.scalar_static_f64[1436]=(if (self.scalar_static_f64[544]!=0.0){self.scalar_static_f64[1432]}else{0.0});
        self.scalar_static_f64[1437]=(self.scalar_static_f64[577]-1.0);
        self.scalar_static_f64[1438]=(self.scalar_static_f64[569]-1.0);
        self.scalar_static_f64[1439]=(self.scalar_static_f64[584]-1.0);
        self.scalar_static_f64[1440]=(self.scalar_static_f64[1425]+self.scalar_static_f64[1433]);
        self.scalar_static_f64[1441]=(self.scalar_static_f64[1426]+self.scalar_static_f64[1434]);
        self.scalar_static_f64[1442]=(self.scalar_static_f64[1427]+self.scalar_static_f64[1436]);
        self.scalar_static_f64[1443]=(self.scalar_static_f64[1425]-self.scalar_static_f64[1433]);
        self.scalar_static_f64[1444]=(self.scalar_static_f64[1426]-self.scalar_static_f64[1434]);
        self.scalar_static_f64[1445]=(-self.scalar_static_f64[1435]);
        self.scalar_static_f64[1446]=(self.scalar_static_f64[1427]-self.scalar_static_f64[1436]);
        self.scalar_static_f64[1447]=(self.scalar_static_f64[122]*self.scalar_static_f64[1443]);
        self.scalar_static_f64[1448]=(self.scalar_static_f64[122]*self.scalar_static_f64[1444]);
        self.scalar_static_f64[1449]=(self.scalar_static_f64[122]*self.scalar_static_f64[1445]);
        self.scalar_static_f64[1450]=(self.scalar_static_f64[122]*self.scalar_static_f64[1446]);
        self.scalar_static_f64[1451]=(-self.scalar_static_f64[1428]);
        self.scalar_static_f64[1452]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1453]=(self.scalar_static_f64[122]*self.scalar_static_f64[628]);
        self.scalar_static_f64[1454]=(self.scalar_static_f64[122]*self.scalar_static_f64[1452]);
        self.scalar_static_f64[1455]=(-self.scalar_static_f64[628]);
        self.scalar_static_f64[1456]=(if (self.scalar_static_f64[597]!=0.0){self.scalar_static_f64[1455]}else{0.0});
        self.scalar_static_f64[1457]=(self.scalar_static_f64[624]-1.0);
        self.scalar_static_f64[1458]=(self.scalar_static_f64[616]-1.0);
        self.scalar_static_f64[1459]=(self.scalar_static_f64[631]-1.0);
        self.scalar_static_f64[1460]=(-self.scalar_static_f64[1456]);
        self.scalar_static_f64[1461]=(self.scalar_static_f64[122]*self.scalar_static_f64[1460]);
        self.scalar_static_f64[1462]=(-self.scalar_static_f64[1452]);
        self.scalar_static_f64[1463]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1464]=(self.scalar_static_f64[122]*self.scalar_static_f64[1463]);
        self.scalar_static_f64[1465]=(self.scalar_static_f64[122]*self.scalar_static_f64[667]);
        self.scalar_static_f64[1466]=(-self.scalar_static_f64[667]);
        self.scalar_static_f64[1467]=(if (self.scalar_static_f64[637]!=0.0){self.scalar_static_f64[1466]}else{0.0});
        self.scalar_static_f64[1468]=(self.scalar_static_f64[663]-1.0);
        self.scalar_static_f64[1469]=(self.scalar_static_f64[656]-1.0);
        self.scalar_static_f64[1470]=(self.scalar_static_f64[670]-1.0);
        self.scalar_static_f64[1471]=(-self.scalar_static_f64[1467]);
        self.scalar_static_f64[1472]=(self.scalar_static_f64[122]*self.scalar_static_f64[1471]);
        self.scalar_static_f64[1473]=(-self.scalar_static_f64[1463]);
        self.scalar_static_f64[1474]=(self.scalar_static_f64[1181]-self.scalar_static_f64[1181]);
        self.scalar_static_f64[1475]=(self.scalar_static_f64[193]-1.0);
        self.scalar_static_f64[1476]=(self.scalar_static_f64[684]-1.0);
        self.scalar_static_f64[1477]=(self.scalar_static_f64[688]-1.0);
        self.scalar_static_f64[1478]=(self.scalar_static_f64[1181]+self.scalar_static_f64[1474]);
        self.scalar_static_f64[1479]=(self.scalar_static_f64[1181]-self.scalar_static_f64[1474]);
        self.scalar_static_f64[1480]=(self.scalar_static_f64[122]*self.scalar_static_f64[1479]);
        self.scalar_static_f64[1481]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1482]=(-self.scalar_static_f64[1481]);
        self.scalar_static_f64[1483]=(self.scalar_static_f64[710]*self.scalar_static_f64[751]);
        self.scalar_static_f64[1484]=(self.scalar_static_f64[710]*self.scalar_static_f64[1482]);
        self.scalar_static_f64[1485]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[1483]}else{0.0});
        self.scalar_static_f64[1486]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[1484]}else{0.0});
        self.scalar_static_f64[1487]=(5.184705528587072e21*self.scalar_static_f64[1485]);
        self.scalar_static_f64[1488]=(5.184705528587072e21*self.scalar_static_f64[1486]);
        self.scalar_static_f64[1489]=(self.scalar_static_f64[735]/self.scalar_static_f64[723]);
        self.scalar_static_f64[1490]=(self.scalar_static_f64[1481]/self.scalar_static_f64[723]);
        self.scalar_static_f64[1491]=(self.scalar_static_f64[122]*self.scalar_static_f64[1489]);
        self.scalar_static_f64[1492]=(self.scalar_static_f64[122]*self.scalar_static_f64[1490]);
        self.scalar_static_f64[1493]=(self.scalar_static_f64[725]-1.0);
        self.scalar_static_f64[1494]=(self.scalar_static_f64[750]-1.0);
        self.scalar_static_f64[1495]=(self.scalar_static_f64[751]*self.scalar_static_f64[764]);
        self.scalar_static_f64[1496]=(self.scalar_static_f64[764]*self.scalar_static_f64[1482]);
        self.scalar_static_f64[1497]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[1495]}else{0.0});
        self.scalar_static_f64[1498]=(if (self.scalar_static_f64[700]!=0.0){self.scalar_static_f64[1496]}else{0.0});
        self.scalar_static_f64[1499]=(5.184705528587072e21*self.scalar_static_f64[1497]);
        self.scalar_static_f64[1500]=(5.184705528587072e21*self.scalar_static_f64[1498]);
        self.scalar_static_f64[1501]=(self.scalar_static_f64[735]/self.scalar_static_f64[773]);
        self.scalar_static_f64[1502]=(self.scalar_static_f64[1481]/self.scalar_static_f64[773]);
        self.scalar_static_f64[1503]=(self.scalar_static_f64[122]*self.scalar_static_f64[1501]);
        self.scalar_static_f64[1504]=(self.scalar_static_f64[122]*self.scalar_static_f64[1502]);
        self.scalar_static_f64[1505]=(self.scalar_static_f64[775]-1.0);
        self.scalar_static_f64[1506]=(self.scalar_static_f64[792]-1.0);
        self.scalar_static_f64[1507]=(if self.scalar_static_bool[170]{self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1508]=(-self.scalar_static_f64[1507]);
        self.scalar_static_f64[1509]=(self.scalar_static_f64[800]*self.scalar_static_f64[831]);
        self.scalar_static_f64[1510]=(self.scalar_static_f64[800]*self.scalar_static_f64[1508]);
        self.scalar_static_f64[1511]=(if self.scalar_static_bool[170]{self.scalar_static_f64[1509]}else{0.0});
        self.scalar_static_f64[1512]=(if self.scalar_static_bool[170]{self.scalar_static_f64[1510]}else{0.0});
        self.scalar_static_f64[1513]=(5.184705528587072e21*self.scalar_static_f64[1511]);
        self.scalar_static_f64[1514]=(5.184705528587072e21*self.scalar_static_f64[1512]);
        self.scalar_static_f64[1515]=(self.scalar_static_f64[815]/self.scalar_static_f64[805]);
        self.scalar_static_f64[1516]=(self.scalar_static_f64[1507]/self.scalar_static_f64[805]);
        self.scalar_static_f64[1517]=(self.scalar_static_f64[122]*self.scalar_static_f64[1515]);
        self.scalar_static_f64[1518]=(self.scalar_static_f64[122]*self.scalar_static_f64[1516]);
        self.scalar_static_f64[1519]=(self.scalar_static_f64[807]-1.0);
        self.scalar_static_f64[1520]=(self.scalar_static_f64[830]-1.0);
        self.scalar_static_f64[1521]=(self.scalar_static_f64[831]*self.scalar_static_f64[838]);
        self.scalar_static_f64[1522]=(self.scalar_static_f64[838]*self.scalar_static_f64[1508]);
        self.scalar_static_f64[1523]=(if self.scalar_static_bool[170]{self.scalar_static_f64[1521]}else{0.0});
        self.scalar_static_f64[1524]=(if self.scalar_static_bool[170]{self.scalar_static_f64[1522]}else{0.0});
        self.scalar_static_f64[1525]=(5.184705528587072e21*self.scalar_static_f64[1523]);
        self.scalar_static_f64[1526]=(5.184705528587072e21*self.scalar_static_f64[1524]);
        self.scalar_static_f64[1527]=(self.scalar_static_f64[815]/self.scalar_static_f64[841]);
        self.scalar_static_f64[1528]=(self.scalar_static_f64[1507]/self.scalar_static_f64[841]);
        self.scalar_static_f64[1529]=(self.scalar_static_f64[122]*self.scalar_static_f64[1527]);
        self.scalar_static_f64[1530]=(self.scalar_static_f64[122]*self.scalar_static_f64[1528]);
        self.scalar_static_f64[1531]=(self.scalar_static_f64[843]-1.0);
        self.scalar_static_f64[1532]=(self.scalar_static_f64[857]-1.0);
        self.scalar_static_f64[1533]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1534]=(-self.scalar_static_f64[1533]);
        self.scalar_static_f64[1535]=(self.scalar_static_f64[864]*self.scalar_static_f64[894]);
        self.scalar_static_f64[1536]=(self.scalar_static_f64[864]*self.scalar_static_f64[1534]);
        self.scalar_static_f64[1537]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1535]}else{0.0});
        self.scalar_static_f64[1538]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1536]}else{0.0});
        self.scalar_static_f64[1539]=(5.184705528587072e21*self.scalar_static_f64[1537]);
        self.scalar_static_f64[1540]=(5.184705528587072e21*self.scalar_static_f64[1538]);
        self.scalar_static_f64[1541]=(self.scalar_static_f64[878]/self.scalar_static_f64[871]);
        self.scalar_static_f64[1542]=(self.scalar_static_f64[1533]/self.scalar_static_f64[871]);
        self.scalar_static_f64[1543]=(self.scalar_static_f64[122]*self.scalar_static_f64[1541]);
        self.scalar_static_f64[1544]=(self.scalar_static_f64[122]*self.scalar_static_f64[1542]);
        self.scalar_static_f64[1545]=(self.scalar_static_f64[872]-1.0);
        self.scalar_static_f64[1546]=(self.scalar_static_f64[893]-1.0);
        self.scalar_static_f64[1547]=(self.scalar_static_f64[902]*self.scalar_static_f64[1534]);
        self.scalar_static_f64[1548]=(self.scalar_static_f64[894]*self.scalar_static_f64[902]);
        self.scalar_static_f64[1549]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1547]}else{0.0});
        self.scalar_static_f64[1550]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1548]}else{0.0});
        self.scalar_static_f64[1551]=(5.184705528587072e21*self.scalar_static_f64[1549]);
        self.scalar_static_f64[1552]=(5.184705528587072e21*self.scalar_static_f64[1550]);
        self.scalar_static_f64[1553]=(self.scalar_static_f64[1533]/self.scalar_static_f64[907]);
        self.scalar_static_f64[1554]=(self.scalar_static_f64[878]/self.scalar_static_f64[907]);
        self.scalar_static_f64[1555]=(self.scalar_static_f64[122]*self.scalar_static_f64[1553]);
        self.scalar_static_f64[1556]=(self.scalar_static_f64[122]*self.scalar_static_f64[1554]);
        self.scalar_static_f64[1557]=(self.scalar_static_f64[908]-1.0);
        self.scalar_static_f64[1558]=(self.scalar_static_f64[923]-1.0);
        self.scalar_static_f64[1559]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1560]=(-self.scalar_static_f64[1559]);
        self.scalar_static_f64[1561]=(self.scalar_static_f64[929]*self.scalar_static_f64[956]);
        self.scalar_static_f64[1562]=(self.scalar_static_f64[929]*self.scalar_static_f64[1560]);
        self.scalar_static_f64[1563]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1561]}else{0.0});
        self.scalar_static_f64[1564]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1562]}else{0.0});
        self.scalar_static_f64[1565]=(5.184705528587072e21*self.scalar_static_f64[1563]);
        self.scalar_static_f64[1566]=(5.184705528587072e21*self.scalar_static_f64[1564]);
        self.scalar_static_f64[1567]=(self.scalar_static_f64[940]/self.scalar_static_f64[933]);
        self.scalar_static_f64[1568]=(self.scalar_static_f64[1559]/self.scalar_static_f64[933]);
        self.scalar_static_f64[1569]=(self.scalar_static_f64[122]*self.scalar_static_f64[1567]);
        self.scalar_static_f64[1570]=(self.scalar_static_f64[122]*self.scalar_static_f64[1568]);
        self.scalar_static_f64[1571]=(self.scalar_static_f64[934]-1.0);
        self.scalar_static_f64[1572]=(self.scalar_static_f64[955]-1.0);
        self.scalar_static_f64[1573]=(self.scalar_static_f64[963]*self.scalar_static_f64[1560]);
        self.scalar_static_f64[1574]=(self.scalar_static_f64[956]*self.scalar_static_f64[963]);
        self.scalar_static_f64[1575]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1573]}else{0.0});
        self.scalar_static_f64[1576]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1574]}else{0.0});
        self.scalar_static_f64[1577]=(5.184705528587072e21*self.scalar_static_f64[1575]);
        self.scalar_static_f64[1578]=(5.184705528587072e21*self.scalar_static_f64[1576]);
        self.scalar_static_f64[1579]=(self.scalar_static_f64[1559]/self.scalar_static_f64[965]);
        self.scalar_static_f64[1580]=(self.scalar_static_f64[940]/self.scalar_static_f64[965]);
        self.scalar_static_f64[1581]=(self.scalar_static_f64[122]*self.scalar_static_f64[1579]);
        self.scalar_static_f64[1582]=(self.scalar_static_f64[122]*self.scalar_static_f64[1580]);
        self.scalar_static_f64[1583]=(self.scalar_static_f64[966]-1.0);
        self.scalar_static_f64[1584]=(self.scalar_static_f64[978]-1.0);
        self.scalar_static_f64[1585]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1586]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1585]}else{0.0});
        self.scalar_static_f64[1587]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1007]}else{0.0});
        self.scalar_static_f64[1588]=(-self.scalar_static_f64[1586]);
        self.scalar_static_f64[1589]=(-self.scalar_static_f64[1587]);
        self.scalar_static_f64[1590]=(self.scalar_static_f64[990]*self.scalar_static_f64[1588]);
        self.scalar_static_f64[1591]=(self.scalar_static_f64[990]*self.scalar_static_f64[1589]);
        self.scalar_static_f64[1592]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1590]}else{0.0});
        self.scalar_static_f64[1593]=(if (self.scalar_static_f64[981]!=0.0){self.scalar_static_f64[1591]}else{0.0});
        self.scalar_static_f64[1594]=(5.184705528587072e21*self.scalar_static_f64[1592]);
        self.scalar_static_f64[1595]=(5.184705528587072e21*self.scalar_static_f64[1593]);
        self.scalar_static_f64[1596]=(self.scalar_static_f64[1586]/self.scalar_static_f64[1000]);
        self.scalar_static_f64[1597]=(self.scalar_static_f64[1587]/self.scalar_static_f64[1000]);
        self.scalar_static_f64[1598]=(self.scalar_static_f64[122]*self.scalar_static_f64[1596]);
        self.scalar_static_f64[1599]=(self.scalar_static_f64[122]*self.scalar_static_f64[1597]);
        self.scalar_static_f64[1600]=(self.scalar_static_f64[1002]-1.0);
        self.scalar_static_f64[1601]=(self.scalar_static_f64[1021]-1.0);
        self.scalar_static_f64[1602]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1585]}else{0.0});
        self.scalar_static_f64[1603]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1007]}else{0.0});
        self.scalar_static_f64[1604]=(-self.scalar_static_f64[1602]);
        self.scalar_static_f64[1605]=(-self.scalar_static_f64[1603]);
        self.scalar_static_f64[1606]=(self.scalar_static_f64[1030]*self.scalar_static_f64[1604]);
        self.scalar_static_f64[1607]=(self.scalar_static_f64[1030]*self.scalar_static_f64[1605]);
        self.scalar_static_f64[1608]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1606]}else{0.0});
        self.scalar_static_f64[1609]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1607]}else{0.0});
        self.scalar_static_f64[1610]=(5.184705528587072e21*self.scalar_static_f64[1608]);
        self.scalar_static_f64[1611]=(5.184705528587072e21*self.scalar_static_f64[1609]);
        self.scalar_static_f64[1612]=(self.scalar_static_f64[1602]/self.scalar_static_f64[1035]);
        self.scalar_static_f64[1613]=(self.scalar_static_f64[1603]/self.scalar_static_f64[1035]);
        self.scalar_static_f64[1614]=(self.scalar_static_f64[122]*self.scalar_static_f64[1612]);
        self.scalar_static_f64[1615]=(self.scalar_static_f64[122]*self.scalar_static_f64[1613]);
        self.scalar_static_f64[1616]=(self.scalar_static_f64[1037]-1.0);
        self.scalar_static_f64[1617]=(self.scalar_static_f64[1056]-1.0);
        self.scalar_static_f64[1618]=(self.scalar_static_f64[1585]/self.scalar_static_f64[1062]);
        self.scalar_static_f64[1619]=(self.scalar_static_f64[1007]/self.scalar_static_f64[1062]);
        self.scalar_static_f64[1620]=(-self.scalar_static_f64[1618]);
        self.scalar_static_f64[1621]=(-self.scalar_static_f64[1619]);
        self.scalar_static_f64[1622]=(if self.scalar_static_bool[236]{self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1623]=(if self.scalar_static_bool[236]{self.scalar_static_f64[1064]}else{0.0});
        self.scalar_static_f64[1624]=(if self.scalar_static_bool[236]{0.0}else{self.scalar_static_f64[1181]});
        self.scalar_static_f64[1625]=(if self.scalar_static_bool[236]{0.0}else{self.scalar_static_f64[1064]});
        self.scalar_static_f64[1626]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1622]}else{0.0});
        self.scalar_static_f64[1627]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1623]}else{0.0});
        self.scalar_static_f64[1628]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1629]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1624]}else{0.0});
        self.scalar_static_f64[1630]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1625]}else{0.0});
        self.scalar_static_f64[1631]=(-self.scalar_static_f64[1626]);
        self.scalar_static_f64[1632]=(-self.scalar_static_f64[1627]);
        self.scalar_static_f64[1633]=(-self.scalar_static_f64[1628]);
        self.scalar_static_f64[1634]=(-self.scalar_static_f64[1629]);
        self.scalar_static_f64[1635]=(-self.scalar_static_f64[1630]);
        self.scalar_static_f64[1636]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1631]);
        self.scalar_static_f64[1637]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1638]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1633]);
        self.scalar_static_f64[1639]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1634]);
        self.scalar_static_f64[1640]=(self.scalar_static_f64[1104]*self.scalar_static_f64[1635]);
        self.scalar_static_f64[1641]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1636]}else{0.0});
        self.scalar_static_f64[1642]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1637]}else{0.0});
        self.scalar_static_f64[1643]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1638]}else{0.0});
        self.scalar_static_f64[1644]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1639]}else{0.0});
        self.scalar_static_f64[1645]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1640]}else{0.0});
        self.scalar_static_f64[1646]=(5.184705528587072e21*self.scalar_static_f64[1641]);
        self.scalar_static_f64[1647]=(5.184705528587072e21*self.scalar_static_f64[1642]);
        self.scalar_static_f64[1648]=(5.184705528587072e21*self.scalar_static_f64[1643]);
        self.scalar_static_f64[1649]=(5.184705528587072e21*self.scalar_static_f64[1644]);
        self.scalar_static_f64[1650]=(5.184705528587072e21*self.scalar_static_f64[1645]);
        self.scalar_static_f64[1651]=(self.scalar_static_f64[1626]/self.scalar_static_f64[1112]);
        self.scalar_static_f64[1652]=(self.scalar_static_f64[1627]/self.scalar_static_f64[1112]);
        self.scalar_static_f64[1653]=(self.scalar_static_f64[1628]/self.scalar_static_f64[1112]);
        self.scalar_static_f64[1654]=(self.scalar_static_f64[1629]/self.scalar_static_f64[1112]);
        self.scalar_static_f64[1655]=(self.scalar_static_f64[1630]/self.scalar_static_f64[1112]);
        self.scalar_static_f64[1656]=(self.scalar_static_f64[122]*self.scalar_static_f64[1651]);
        self.scalar_static_f64[1657]=(self.scalar_static_f64[122]*self.scalar_static_f64[1652]);
        self.scalar_static_f64[1658]=(self.scalar_static_f64[122]*self.scalar_static_f64[1653]);
        self.scalar_static_f64[1659]=(self.scalar_static_f64[122]*self.scalar_static_f64[1654]);
        self.scalar_static_f64[1660]=(self.scalar_static_f64[122]*self.scalar_static_f64[1655]);
        self.scalar_static_f64[1661]=(self.scalar_static_f64[1113]-1.0);
        self.scalar_static_f64[1662]=(self.scalar_static_f64[1131]-1.0);
        self.scalar_static_f64[1663]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1632]);
        self.scalar_static_f64[1664]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1631]);
        self.scalar_static_f64[1665]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1633]);
        self.scalar_static_f64[1666]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1635]);
        self.scalar_static_f64[1667]=(self.scalar_static_f64[1140]*self.scalar_static_f64[1634]);
        self.scalar_static_f64[1668]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1663]}else{0.0});
        self.scalar_static_f64[1669]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1664]}else{0.0});
        self.scalar_static_f64[1670]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1665]}else{0.0});
        self.scalar_static_f64[1671]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1666]}else{0.0});
        self.scalar_static_f64[1672]=(if (self.scalar_static_f64[1097]!=0.0){self.scalar_static_f64[1667]}else{0.0});
        self.scalar_static_f64[1673]=(5.184705528587072e21*self.scalar_static_f64[1668]);
        self.scalar_static_f64[1674]=(5.184705528587072e21*self.scalar_static_f64[1669]);
        self.scalar_static_f64[1675]=(5.184705528587072e21*self.scalar_static_f64[1670]);
        self.scalar_static_f64[1676]=(5.184705528587072e21*self.scalar_static_f64[1671]);
        self.scalar_static_f64[1677]=(5.184705528587072e21*self.scalar_static_f64[1672]);
        self.scalar_static_f64[1678]=(self.scalar_static_f64[1627]/self.scalar_static_f64[1145]);
        self.scalar_static_f64[1679]=(self.scalar_static_f64[1626]/self.scalar_static_f64[1145]);
        self.scalar_static_f64[1680]=(self.scalar_static_f64[1628]/self.scalar_static_f64[1145]);
        self.scalar_static_f64[1681]=(self.scalar_static_f64[1630]/self.scalar_static_f64[1145]);
        self.scalar_static_f64[1682]=(self.scalar_static_f64[1629]/self.scalar_static_f64[1145]);
        self.scalar_static_f64[1683]=(self.scalar_static_f64[122]*self.scalar_static_f64[1678]);
        self.scalar_static_f64[1684]=(self.scalar_static_f64[122]*self.scalar_static_f64[1679]);
        self.scalar_static_f64[1685]=(self.scalar_static_f64[122]*self.scalar_static_f64[1680]);
        self.scalar_static_f64[1686]=(self.scalar_static_f64[122]*self.scalar_static_f64[1681]);
        self.scalar_static_f64[1687]=(self.scalar_static_f64[122]*self.scalar_static_f64[1682]);
        self.scalar_static_f64[1688]=(self.scalar_static_f64[1146]-1.0);
        self.scalar_static_f64[1689]=(self.scalar_static_f64[1159]-1.0);
        self.scalar_static_f64[1690]=(-1.0/self.scalar_static_f64[1163]);
        self.scalar_static_f64[1691]=(1.0/self.scalar_static_f64[1163]);
        self.scalar_static_f64[1692]=(1.0/self.scalar_static_f64[1166]);
        self.scalar_static_f64[1693]=(if (self.scalar_static_f64[131]!=0.0){self.scalar_static_f64[1692]}else{0.0});
        self.scalar_static_f64[1694]=(-self.scalar_static_f64[1167]);
        self.scalar_static_f64[1695]=(if self.scalar_static_bool[16]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[1696]=(if self.scalar_static_bool[16]{self.scalar_static_f64[1181]}else{0.0});
        self.scalar_static_f64[1697]=(1.0/self.scalar_static_f64[1170]);
        self.scalar_static_f64[1698]=(-1.0/self.scalar_static_f64[1170]);
        self.scalar_static_f64[1699]=(if self.scalar_static_bool[16]{self.scalar_static_f64[1697]}else{0.0});
        self.scalar_static_f64[1700]=(if self.scalar_static_bool[16]{self.scalar_static_f64[1698]}else{0.0});
        self.scalar_static_f64[1701]=(1.0/self.scalar_static_f64[1171]);
        self.scalar_static_f64[1702]=(-1.0/self.scalar_static_f64[1171]);
        self.scalar_static_f64[1703]=(if self.scalar_static_bool[16]{self.scalar_static_f64[1701]}else{0.0});
        self.scalar_static_f64[1704]=(if self.scalar_static_bool[16]{self.scalar_static_f64[1702]}else{0.0});
        self.scalar_static_f64[1705]=(-self.scalar_static_f64[1177]);
        self.scalar_static_f64[1706]=(if self.scalar_static_bool[260]{-1.0}else{0.0});
        self.scalar_static_f64[1707]=(if self.scalar_static_bool[260]{1.0}else{0.0});
        self.scalar_static_f64[1708]=(if self.scalar_static_bool[260]{-0.0}else{0.0});
        self.scalar_static_f64[1709]=(-1.0/self.scalar_static_f64[1095]);
        self.scalar_static_f64[1710]=(1.0/self.scalar_static_f64[1095]);
        self.scalar_static_f64[1711]=(if self.scalar_static_bool[233]{self.scalar_static_f64[1709]}else{0.0});
        self.scalar_static_f64[1712]=(if self.scalar_static_bool[233]{self.scalar_static_f64[1710]}else{0.0});
        self.scalar_static_f64[1713]=(1.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[1714]=(-1.0/self.scalar_static_f64[44]);
        self.scalar_static_f64[1715]=(if (self.scalar_static_f64[1160]!=0.0){self.scalar_static_f64[1713]}else{0.0});
        self.scalar_static_f64[1716]=(if (self.scalar_static_f64[1160]!=0.0){self.scalar_static_f64[1714]}else{0.0});
        self.scalar_static_f64[1717]=(1.0/self.scalar_static_f64[48]);
        self.scalar_static_f64[1718]=(-1.0/self.scalar_static_f64[48]);
        self.scalar_static_f64[1719]=(if (self.scalar_static_f64[1161]!=0.0){self.scalar_static_f64[1717]}else{0.0});
        self.scalar_static_f64[1720]=(if (self.scalar_static_f64[1161]!=0.0){self.scalar_static_f64[1718]}else{0.0});
        self.scalar_static_f64[1721]=(1.0/self.scalar_static_f64[1164]);
        self.scalar_static_f64[1722]=(if (self.scalar_static_f64[1165]!=0.0){self.scalar_static_f64[1721]}else{0.0});
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
        self.scalar_static_f64[1723]=(temperature+self.scalar_static_f64[2]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
