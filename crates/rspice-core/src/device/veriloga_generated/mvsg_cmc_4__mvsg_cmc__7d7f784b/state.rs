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
    pub(crate) ddt_state_initialized: Box<[bool; 146]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: f64,
    pub(crate) scalar_v39: f64,
    pub(crate) scalar_v40: f64,
    pub(crate) scalar_v41: f64,
    pub(crate) scalar_v42: f64,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: f64,
    pub(crate) scalar_v45: f64,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: f64,
    pub(crate) scalar_v48: f64,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: bool,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v95: bool,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v97: bool,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: bool,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: bool,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: bool,
    pub(crate) scalar_v111: bool,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: bool,
    pub(crate) scalar_v114: bool,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: bool,
    pub(crate) scalar_v117: bool,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: bool,
    pub(crate) scalar_v120: bool,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: bool,
    pub(crate) scalar_v123: bool,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: bool,
    pub(crate) scalar_v126: bool,
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: bool,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: bool,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: bool,
    pub(crate) scalar_v157: bool,
    pub(crate) scalar_v158: bool,
    pub(crate) scalar_v159: bool,
    pub(crate) scalar_v160: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: bool,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v168: bool,
    pub(crate) scalar_v169: bool,
    pub(crate) scalar_v170: bool,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v242: bool,
    pub(crate) scalar_v266: f64,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: f64,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: f64,
    pub(crate) scalar_v304: f64,
    pub(crate) scalar_v305: f64,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v318: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scratch: Option<Box<GenericScratch<2701, 30, 36>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<2701, 30, 36>>>,
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
            scalar_v5: self.scalar_v5,
            scalar_v7: self.scalar_v7,
            scalar_v28: self.scalar_v28,
            scalar_v37: self.scalar_v37,
            scalar_v38: self.scalar_v38,
            scalar_v39: self.scalar_v39,
            scalar_v40: self.scalar_v40,
            scalar_v41: self.scalar_v41,
            scalar_v42: self.scalar_v42,
            scalar_v43: self.scalar_v43,
            scalar_v44: self.scalar_v44,
            scalar_v45: self.scalar_v45,
            scalar_v46: self.scalar_v46,
            scalar_v47: self.scalar_v47,
            scalar_v48: self.scalar_v48,
            scalar_v49: self.scalar_v49,
            scalar_v50: self.scalar_v50,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v65: self.scalar_v65,
            scalar_v68: self.scalar_v68,
            scalar_v71: self.scalar_v71,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v111: self.scalar_v111,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v115: self.scalar_v115,
            scalar_v116: self.scalar_v116,
            scalar_v117: self.scalar_v117,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
            scalar_v122: self.scalar_v122,
            scalar_v123: self.scalar_v123,
            scalar_v124: self.scalar_v124,
            scalar_v125: self.scalar_v125,
            scalar_v126: self.scalar_v126,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v154: self.scalar_v154,
            scalar_v155: self.scalar_v155,
            scalar_v156: self.scalar_v156,
            scalar_v157: self.scalar_v157,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v160: self.scalar_v160,
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v170: self.scalar_v170,
            scalar_v177: self.scalar_v177,
            scalar_v185: self.scalar_v185,
            scalar_v189: self.scalar_v189,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v215: self.scalar_v215,
            scalar_v216: self.scalar_v216,
            scalar_v219: self.scalar_v219,
            scalar_v220: self.scalar_v220,
            scalar_v223: self.scalar_v223,
            scalar_v224: self.scalar_v224,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v242: self.scalar_v242,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v269: self.scalar_v269,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v288: self.scalar_v288,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
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
            scratch: None,
            reactive_scratch: None,
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
            ddt_state_initialized: boxed_zero_bool_array::<{ Self::DDT_STATE_COUNT }>(),
            idt_state_current: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_previous: boxed_zero_f64_array::<{ Self::IDT_STATE_COUNT }>(),
            idt_state_initialized: boxed_zero_bool_array::<{ Self::IDT_STATE_COUNT }>(),
            time: 0.0,
            timestep: 0.0,
            scalar_v5: 0.0,
            scalar_v7: 0.0,
            scalar_v28: 0.0,
            scalar_v37: 0.0,
            scalar_v38: 0.0,
            scalar_v39: 0.0,
            scalar_v40: 0.0,
            scalar_v41: 0.0,
            scalar_v42: 0.0,
            scalar_v43: 0.0,
            scalar_v44: 0.0,
            scalar_v45: 0.0,
            scalar_v46: 0.0,
            scalar_v47: 0.0,
            scalar_v48: 0.0,
            scalar_v49: 0.0,
            scalar_v50: 0.0,
            scalar_v58: 0.0,
            scalar_v59: false,
            scalar_v60: 0.0,
            scalar_v65: 0.0,
            scalar_v68: 0.0,
            scalar_v71: 0.0,
            scalar_v95: false,
            scalar_v96: false,
            scalar_v97: false,
            scalar_v102: 0.0,
            scalar_v103: false,
            scalar_v105: false,
            scalar_v106: 0.0,
            scalar_v107: false,
            scalar_v108: false,
            scalar_v109: 0.0,
            scalar_v110: false,
            scalar_v111: false,
            scalar_v112: 0.0,
            scalar_v113: false,
            scalar_v114: false,
            scalar_v115: 0.0,
            scalar_v116: false,
            scalar_v117: false,
            scalar_v118: 0.0,
            scalar_v119: false,
            scalar_v120: false,
            scalar_v121: 0.0,
            scalar_v122: false,
            scalar_v123: false,
            scalar_v124: 0.0,
            scalar_v125: false,
            scalar_v126: false,
            scalar_v149: 0.0,
            scalar_v150: false,
            scalar_v151: 0.0,
            scalar_v152: false,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: false,
            scalar_v157: false,
            scalar_v158: false,
            scalar_v159: false,
            scalar_v160: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v165: false,
            scalar_v166: false,
            scalar_v167: false,
            scalar_v168: false,
            scalar_v169: false,
            scalar_v170: false,
            scalar_v177: 0.0,
            scalar_v185: 0.0,
            scalar_v189: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v223: 0.0,
            scalar_v224: 0.0,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v242: false,
            scalar_v266: 0.0,
            scalar_v267: 0.0,
            scalar_v268: 0.0,
            scalar_v269: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v288: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: 0.0,
            scalar_v304: 0.0,
            scalar_v305: 0.0,
            scalar_v306: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v314: 0.0,
            scalar_v315: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v318: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: Some(GenericReactiveScratch::new_box()),
        };
        instance.recompute_instance_static();
        instance
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
            scalar_v5,
            scalar_v7,
            scalar_v28,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v65,
            scalar_v68,
            scalar_v71,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v102,
            scalar_v103,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v177,
            scalar_v185,
            scalar_v189,
            scalar_v207,
            scalar_v208,
            scalar_v211,
            scalar_v212,
            scalar_v215,
            scalar_v216,
            scalar_v219,
            scalar_v220,
            scalar_v223,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v231,
            scalar_v232,
            scalar_v235,
            scalar_v236,
            scalar_v242,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v288,
            scalar_v295,
            scalar_v296,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
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
            scalar_v5,
            scalar_v7,
            scalar_v28,
            scalar_v37,
            scalar_v38,
            scalar_v39,
            scalar_v40,
            scalar_v41,
            scalar_v42,
            scalar_v43,
            scalar_v44,
            scalar_v45,
            scalar_v46,
            scalar_v47,
            scalar_v48,
            scalar_v49,
            scalar_v50,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v65,
            scalar_v68,
            scalar_v71,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v102,
            scalar_v103,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v111,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v115,
            scalar_v116,
            scalar_v117,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
            scalar_v122,
            scalar_v123,
            scalar_v124,
            scalar_v125,
            scalar_v126,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v154,
            scalar_v155,
            scalar_v156,
            scalar_v157,
            scalar_v158,
            scalar_v159,
            scalar_v160,
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v170,
            scalar_v177,
            scalar_v185,
            scalar_v189,
            scalar_v207,
            scalar_v208,
            scalar_v211,
            scalar_v212,
            scalar_v215,
            scalar_v216,
            scalar_v219,
            scalar_v220,
            scalar_v223,
            scalar_v224,
            scalar_v227,
            scalar_v228,
            scalar_v231,
            scalar_v232,
            scalar_v235,
            scalar_v236,
            scalar_v242,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v288,
            scalar_v295,
            scalar_v296,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
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
            "w" => { validate_parameter("w", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("l", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "ngf" => { validate_parameter("ngf", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_parameter("version", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "cg" => { validate_parameter("cg", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "tcg" => { validate_finite_parameter("tcg", value)?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "cofsm" => { validate_parameter("cofsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "cofdm" => { validate_parameter("cofdm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "cofdsm" => { validate_parameter("cofdsm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "cofdsubm" => { validate_parameter("cofdsubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "cofssubm" => { validate_parameter("cofssubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "cofgsubm" => { validate_parameter("cofgsubm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "cofsm0" => { validate_parameter("cofsm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cofdm0" => { validate_parameter("cofdm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "cofdsm0" => { validate_parameter("cofdsm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "cofdsubm0" => { validate_parameter("cofdsubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "cofssubm0" => { validate_parameter("cofssubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "cofgsubm0" => { validate_parameter("cofgsubm0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "tcofs" => { validate_finite_parameter("tcofs", value)?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "tcofd" => { validate_finite_parameter("tcofd", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "tcofds" => { validate_finite_parameter("tcofds", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "tcofssub" => { validate_finite_parameter("tcofssub", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "tcofdsub" => { validate_finite_parameter("tcofdsub", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "tcofgsub" => { validate_finite_parameter("tcofgsub", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "vtfrin" => { validate_finite_parameter("vtfrin", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "nfrin" => { validate_parameter("nfrin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "rsh" => { validate_parameter("rsh", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "rcs" => { validate_parameter("rcs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "rcd" => { validate_parameter("rcd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "vx0" => { validate_parameter("vx0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "mu0" => { validate_parameter("mu0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "beta" => { validate_parameter("beta", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "vto" => { validate_finite_parameter("vto", value)?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "ss" => { validate_parameter("ss", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "delta1" => { validate_parameter("delta1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "delta2" => { validate_parameter("delta2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "dibsat" => { validate_parameter("dibsat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "nd" => { validate_parameter("nd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "alpha" => { validate_parameter("alpha", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "lambda" => { validate_parameter("lambda", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "vtheta" => { validate_parameter("vtheta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "mtheta" => { validate_parameter("mtheta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "vzeta" => { validate_parameter("vzeta", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "vtzeta" => { validate_finite_parameter("vtzeta", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "epsilon" => { validate_parameter("epsilon", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "rct1" => { validate_finite_parameter("rct1", value)?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "rct2" => { validate_finite_parameter("rct2", value)?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "flagres" => { validate_parameter("flagres", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "flagsp" => { validate_parameter("flagsp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "flaggum" => { validate_parameter("flaggum", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "mmaxs" => { validate_parameter("mmaxs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "lgs" => { validate_parameter("lgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "vtors" => { validate_finite_parameter("vtors", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "cgrs" => { validate_parameter("cgrs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "vx0rs" => { validate_parameter("vx0rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "mu0rs" => { validate_parameter("mu0rs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "betars" => { validate_parameter("betars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "delta1rs" => { validate_parameter("delta1rs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "srs" => { validate_parameter("srs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "ndrs" => { validate_parameter("ndrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "vthetars" => { validate_parameter("vthetars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "mthetars" => { validate_parameter("mthetars", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "alphars" => { validate_parameter("alphars", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "lgd" => { validate_parameter("lgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "vtord" => { validate_finite_parameter("vtord", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "cgrd" => { validate_parameter("cgrd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "vx0rd" => { validate_parameter("vx0rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "mu0rd" => { validate_parameter("mu0rd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "betard" => { validate_parameter("betard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "delta1rd" => { validate_parameter("delta1rd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "srd" => { validate_parameter("srd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "ndrd" => { validate_parameter("ndrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "vthetard" => { validate_parameter("vthetard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "mthetard" => { validate_parameter("mthetard", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "alphard" => { validate_parameter("alphard", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "flagfps1" => { validate_parameter("flagfps1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "lgfps1" => { validate_parameter("lgfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "vtofps1" => { validate_finite_parameter("vtofps1", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "cgfps1" => { validate_parameter("cgfps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "tcgfps1" => { validate_finite_parameter("tcgfps1", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "flagfps1s" => { validate_parameter("flagfps1s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "cfps1s" => { validate_parameter("cfps1s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "flagfps1b" => { validate_parameter("flagfps1b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "ccfps1" => { validate_parameter("ccfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "tccfps1" => { validate_finite_parameter("tccfps1", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "cbfps1" => { validate_parameter("cbfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "tcbfps1" => { validate_finite_parameter("tcbfps1", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "vx0fps1" => { validate_parameter("vx0fps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "mu0fps1" => { validate_parameter("mu0fps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "betafps1" => { validate_parameter("betafps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "delta1fps1" => { validate_parameter("delta1fps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "sfps1" => { validate_parameter("sfps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "ndfps1" => { validate_parameter("ndfps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "vtzetafps1" => { validate_finite_parameter("vtzetafps1", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "vthetafps1" => { validate_parameter("vthetafps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "mthetafps1" => { validate_parameter("mthetafps1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "alphafps1" => { validate_parameter("alphafps1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "flagfps2" => { validate_parameter("flagfps2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "lgfps2" => { validate_parameter("lgfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "vtofps2" => { validate_finite_parameter("vtofps2", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "cgfps2" => { validate_parameter("cgfps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "tcgfps2" => { validate_finite_parameter("tcgfps2", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "flagfps2s" => { validate_parameter("flagfps2s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "cfps2s" => { validate_parameter("cfps2s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "flagfps2b" => { validate_parameter("flagfps2b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "ccfps2" => { validate_parameter("ccfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "tccfps2" => { validate_finite_parameter("tccfps2", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "cbfps2" => { validate_parameter("cbfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "tcbfps2" => { validate_finite_parameter("tcbfps2", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "vx0fps2" => { validate_parameter("vx0fps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "mu0fps2" => { validate_parameter("mu0fps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "betafps2" => { validate_parameter("betafps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "delta1fps2" => { validate_parameter("delta1fps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "sfps2" => { validate_parameter("sfps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "ndfps2" => { validate_parameter("ndfps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "vtzetafps2" => { validate_finite_parameter("vtzetafps2", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "vthetafps2" => { validate_parameter("vthetafps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "mthetafps2" => { validate_parameter("mthetafps2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "alphafps2" => { validate_parameter("alphafps2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "flagfps3" => { validate_parameter("flagfps3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "lgfps3" => { validate_parameter("lgfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "vtofps3" => { validate_finite_parameter("vtofps3", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "cgfps3" => { validate_parameter("cgfps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "tcgfps3" => { validate_finite_parameter("tcgfps3", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "flagfps3s" => { validate_parameter("flagfps3s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "cfps3s" => { validate_parameter("cfps3s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "flagfps3b" => { validate_parameter("flagfps3b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "ccfps3" => { validate_parameter("ccfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "tccfps3" => { validate_finite_parameter("tccfps3", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "cbfps3" => { validate_parameter("cbfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "tcbfps3" => { validate_finite_parameter("tcbfps3", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "vx0fps3" => { validate_parameter("vx0fps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "mu0fps3" => { validate_parameter("mu0fps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "betafps3" => { validate_parameter("betafps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "delta1fps3" => { validate_parameter("delta1fps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "sfps3" => { validate_parameter("sfps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "ndfps3" => { validate_parameter("ndfps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "vtzetafps3" => { validate_finite_parameter("vtzetafps3", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "vthetafps3" => { validate_parameter("vthetafps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "mthetafps3" => { validate_parameter("mthetafps3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "alphafps3" => { validate_parameter("alphafps3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "flagfps4" => { validate_parameter("flagfps4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "lgfps4" => { validate_parameter("lgfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "vtofps4" => { validate_finite_parameter("vtofps4", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "cgfps4" => { validate_parameter("cgfps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "tcgfps4" => { validate_finite_parameter("tcgfps4", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "flagfps4s" => { validate_parameter("flagfps4s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "cfps4s" => { validate_parameter("cfps4s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "flagfps4b" => { validate_parameter("flagfps4b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "ccfps4" => { validate_parameter("ccfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "tccfps4" => { validate_finite_parameter("tccfps4", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "cbfps4" => { validate_parameter("cbfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "tcbfps4" => { validate_finite_parameter("tcbfps4", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "vx0fps4" => { validate_parameter("vx0fps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "mu0fps4" => { validate_parameter("mu0fps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "betafps4" => { validate_parameter("betafps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "delta1fps4" => { validate_parameter("delta1fps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "sfps4" => { validate_parameter("sfps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "ndfps4" => { validate_parameter("ndfps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "vtzetafps4" => { validate_finite_parameter("vtzetafps4", value)?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "vthetafps4" => { validate_parameter("vthetafps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "mthetafps4" => { validate_parameter("mthetafps4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "alphafps4" => { validate_parameter("alphafps4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "flagfp1" => { validate_parameter("flagfp1", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "lgfp1" => { validate_parameter("lgfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "vtofp1" => { validate_finite_parameter("vtofp1", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "cgfp1" => { validate_parameter("cgfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "tcgfp1" => { validate_finite_parameter("tcgfp1", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "flagfp1s" => { validate_parameter("flagfp1s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "cfp1s" => { validate_parameter("cfp1s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "flagfp1b" => { validate_parameter("flagfp1b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "ccfp1" => { validate_parameter("ccfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "tccfp1" => { validate_finite_parameter("tccfp1", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "cbfp1" => { validate_parameter("cbfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "tcbfp1" => { validate_finite_parameter("tcbfp1", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "vx0fp1" => { validate_parameter("vx0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "mu0fp1" => { validate_parameter("mu0fp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "betafp1" => { validate_parameter("betafp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "delta1fp1" => { validate_parameter("delta1fp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "sfp1" => { validate_parameter("sfp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "ndfp1" => { validate_parameter("ndfp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "vtzetafp1" => { validate_finite_parameter("vtzetafp1", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "vthetafp1" => { validate_parameter("vthetafp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "mthetafp1" => { validate_parameter("mthetafp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "alphafp1" => { validate_parameter("alphafp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "flagfp2" => { validate_parameter("flagfp2", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "lgfp2" => { validate_parameter("lgfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "vtofp2" => { validate_finite_parameter("vtofp2", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "cgfp2" => { validate_parameter("cgfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "tcgfp2" => { validate_finite_parameter("tcgfp2", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "flagfp2s" => { validate_parameter("flagfp2s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "cfp2s" => { validate_parameter("cfp2s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "flagfp2b" => { validate_parameter("flagfp2b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "ccfp2" => { validate_parameter("ccfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "tccfp2" => { validate_finite_parameter("tccfp2", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "cbfp2" => { validate_parameter("cbfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "tcbfp2" => { validate_finite_parameter("tcbfp2", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "vx0fp2" => { validate_parameter("vx0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "mu0fp2" => { validate_parameter("mu0fp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "betafp2" => { validate_parameter("betafp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "delta1fp2" => { validate_parameter("delta1fp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "sfp2" => { validate_parameter("sfp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "ndfp2" => { validate_parameter("ndfp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "vtzetafp2" => { validate_finite_parameter("vtzetafp2", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "vthetafp2" => { validate_parameter("vthetafp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "mthetafp2" => { validate_parameter("mthetafp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "alphafp2" => { validate_parameter("alphafp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "flagfp3" => { validate_parameter("flagfp3", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "lgfp3" => { validate_parameter("lgfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "vtofp3" => { validate_finite_parameter("vtofp3", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "cgfp3" => { validate_parameter("cgfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "tcgfp3" => { validate_finite_parameter("tcgfp3", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "flagfp3s" => { validate_parameter("flagfp3s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "cfp3s" => { validate_parameter("cfp3s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "flagfp3b" => { validate_parameter("flagfp3b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "ccfp3" => { validate_parameter("ccfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "tccfp3" => { validate_finite_parameter("tccfp3", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "cbfp3" => { validate_parameter("cbfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "tcbfp3" => { validate_finite_parameter("tcbfp3", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "vx0fp3" => { validate_parameter("vx0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "mu0fp3" => { validate_parameter("mu0fp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "betafp3" => { validate_parameter("betafp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "delta1fp3" => { validate_parameter("delta1fp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "sfp3" => { validate_parameter("sfp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "ndfp3" => { validate_parameter("ndfp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "vtzetafp3" => { validate_finite_parameter("vtzetafp3", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "vthetafp3" => { validate_parameter("vthetafp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "mthetafp3" => { validate_parameter("mthetafp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "alphafp3" => { validate_parameter("alphafp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "flagfp4" => { validate_parameter("flagfp4", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "lgfp4" => { validate_parameter("lgfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "vtofp4" => { validate_finite_parameter("vtofp4", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "cgfp4" => { validate_parameter("cgfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "tcgfp4" => { validate_finite_parameter("tcgfp4", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "flagfp4s" => { validate_parameter("flagfp4s", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "cfp4s" => { validate_parameter("cfp4s", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "flagfp4b" => { validate_parameter("flagfp4b", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "ccfp4" => { validate_parameter("ccfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "tccfp4" => { validate_finite_parameter("tccfp4", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "cbfp4" => { validate_parameter("cbfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "tcbfp4" => { validate_finite_parameter("tcbfp4", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "vx0fp4" => { validate_parameter("vx0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "mu0fp4" => { validate_parameter("mu0fp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "betafp4" => { validate_parameter("betafp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "delta1fp4" => { validate_parameter("delta1fp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "sfp4" => { validate_parameter("sfp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "ndfp4" => { validate_parameter("ndfp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "vtzetafp4" => { validate_finite_parameter("vtzetafp4", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "vthetafp4" => { validate_parameter("vthetafp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "mthetafp4" => { validate_parameter("mthetafp4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "alphafp4" => { validate_parameter("alphafp4", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "igmod" => { validate_parameter("igmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "fracig" => { validate_parameter("fracig", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "vjg" => { validate_parameter("vjg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "pg_param1" => { validate_parameter("pg_param1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "pg_params" => { validate_parameter("pg_params", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "ijs" => { validate_parameter("ijs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "vgsats" => { validate_parameter("vgsats", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "fracs" => { validate_parameter("fracs", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "alphags" => { validate_parameter("alphags", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "pg_paramd" => { validate_parameter("pg_paramd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "ijd" => { validate_parameter("ijd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "vgsatd" => { validate_parameter("vgsatd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "fracd" => { validate_parameter("fracd", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "alphagd" => { validate_parameter("alphagd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "pgsrecs" => { validate_parameter("pgsrecs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "irecs" => { validate_parameter("irecs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "vgsatqs" => { validate_parameter("vgsatqs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "betarecs" => { validate_parameter("betarecs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "pgsrecd" => { validate_parameter("pgsrecd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "irecd" => { validate_parameter("irecd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "vgsatqd" => { validate_parameter("vgsatqd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "betarecd" => { validate_parameter("betarecd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "kbdgates" => { validate_parameter("kbdgates", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "vbdgs" => { validate_parameter("vbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "pbdgs" => { validate_parameter("pbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "kbdgated" => { validate_parameter("kbdgated", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "vbdgd" => { validate_parameter("vbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "pbdgd" => { validate_parameter("pbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "igrecmod" => { validate_parameter("igrecmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "pgsrecs2" => { validate_parameter("pgsrecs2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "irecs2" => { validate_parameter("irecs2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "vgsatqs2" => { validate_parameter("vgsatqs2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "betarecs2" => { validate_parameter("betarecs2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            "pgsrecd2" => { validate_parameter("pgsrecd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); Ok(()) }
            "irecd2" => { validate_parameter("irecd2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); Ok(()) }
            "vgsatqd2" => { validate_parameter("vgsatqd2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); Ok(()) }
            "betarecd2" => { validate_parameter("betarecd2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); Ok(()) }
            "flagpgan" => { validate_parameter("flagpgan", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); Ok(()) }
            "pg_param_pgan" => { validate_parameter("pg_param_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); Ok(()) }
            "ij_pgan" => { validate_parameter("ij_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); Ok(()) }
            "vgsat_pgan" => { validate_parameter("vgsat_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); Ok(()) }
            "frac_pgan" => { validate_parameter("frac_pgan", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); Ok(()) }
            "alphag_pgan" => { validate_parameter("alphag_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); Ok(()) }
            "pgsrec_pgan" => { validate_parameter("pgsrec_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); Ok(()) }
            "irec_pgan" => { validate_parameter("irec_pgan", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); Ok(()) }
            "vgsatq_pgan" => { validate_parameter("vgsatq_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); Ok(()) }
            "betarec_pgan" => { validate_parameter("betarec_pgan", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); Ok(()) }
            "pganrecmod" => { validate_parameter("pganrecmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); Ok(()) }
            "pgsrec_pgan2" => { validate_parameter("pgsrec_pgan2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); Ok(()) }
            "irec_pgan2" => { validate_parameter("irec_pgan2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); Ok(()) }
            "vgsatq_pgan2" => { validate_parameter("vgsatq_pgan2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); Ok(()) }
            "betarec_pgan2" => { validate_parameter("betarec_pgan2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); Ok(()) }
            "vcsh0" => { validate_parameter("vcsh0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); Ok(()) }
            "csh0" => { validate_parameter("csh0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); Ok(()) }
            "fc" => { validate_parameter("fc", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); Ok(()) }
            "pgancshorder" => { validate_parameter("pgancshorder", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); Ok(()) }
            "rsch0" => { validate_parameter("rsch0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); Ok(()) }
            "ohmicratio" => { validate_parameter("ohmicratio", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); Ok(()) }
            "icbdmod" => { validate_parameter("icbdmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); Ok(()) }
            "cbddbmod" => { validate_parameter("cbddbmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); Ok(()) }
            "ijscbd" => { validate_parameter("ijscbd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); Ok(()) }
            "ijdcbd" => { validate_parameter("ijdcbd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); Ok(()) }
            "vchbdgs" => { validate_parameter("vchbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); Ok(()) }
            "pchbdgs" => { validate_parameter("pchbdgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); Ok(()) }
            "vchbdgd" => { validate_parameter("vchbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); Ok(()) }
            "pchbdgd" => { validate_parameter("pchbdgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("rth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("cth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); Ok(()) }
            "gmdisp" => { validate_parameter("gmdisp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); Ok(()) }
            "taugmrf" => { validate_parameter("taugmrf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); Ok(()) }
            "rgsp" => { validate_parameter("rgsp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("ngcon", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); Ok(()) }
            "lovg" => { validate_parameter("lovg", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); Ok(()) }
            "agate" => { validate_parameter("agate", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); Ok(()) }
            "trapselect" => { validate_parameter("trapselect", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); Ok(()) }
            "rintrap1" => { validate_parameter("rintrap1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); Ok(()) }
            "ctrap" => { validate_parameter("ctrap", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); Ok(()) }
            "vttrap" => { validate_parameter("vttrap", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); Ok(()) }
            "taut" => { validate_parameter("taut", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); Ok(()) }
            "alphat1" => { validate_parameter("alphat1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); Ok(()) }
            "alphat2" => { validate_parameter("alphat2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); Ok(()) }
            "alphat3" => { validate_parameter("alphat3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); Ok(()) }
            "tempt" => { validate_parameter("tempt", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); Ok(()) }
            "vgltrapth" => { validate_parameter("vgltrapth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); Ok(()) }
            "vdltrapth" => { validate_parameter("vdltrapth", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); Ok(()) }
            "rcapture" => { validate_parameter("rcapture", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); Ok(()) }
            "remission" => { validate_parameter("remission", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); Ok(()) }
            "cdglag" => { validate_parameter("cdglag", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); Ok(()) }
            "rct1dl" => { validate_finite_parameter("rct1dl", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); Ok(()) }
            "rct1gl" => { validate_finite_parameter("rct1gl", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); Ok(()) }
            "rct2dl" => { validate_finite_parameter("rct2dl", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); Ok(()) }
            "rct2gl" => { validate_finite_parameter("rct2gl", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); Ok(()) }
            "isat" => { validate_parameter("isat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); Ok(()) }
            "noisemod" => { validate_parameter("noisemod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); Ok(()) }
            "shs" => { validate_parameter("shs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); Ok(()) }
            "shd" => { validate_parameter("shd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("kf", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_parameter("af", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); Ok(()) }
            "ffe" => { validate_parameter("ffe", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); Ok(()) }
            "minl" => { validate_parameter("minl", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); Ok(()) }
            "minc" => { validate_parameter("minc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); Ok(()) }
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

    #[inline]
    fn recompute_instance_static(&mut self) {
        let p = &(*self.params);
        let v5: f64 = p.p0;
        self.scalar_v5 = v5;
        let v7: f64 = p.p2;
        self.scalar_v7 = v7;
        let v28: f64 = p.p353;
        self.scalar_v28 = v28;
        let v37: f64 = p.p324;
        self.scalar_v37 = v37;
        let v38: f64 = (p.p324 / p.p2);
        self.scalar_v38 = v38;
        let v39: f64 = p.p325;
        self.scalar_v39 = v39;
        let v40: f64 = (v38 / p.p325);
        self.scalar_v40 = v40;
        let v41: f64 = p.p326;
        self.scalar_v41 = v41;
        let v42: f64 = p.p327;
        self.scalar_v42 = v42;
        let v43: f64 = (p.p327 * p.p0);
        self.scalar_v43 = v43;
        let v44: f64 = (v43 / p.p325);
        self.scalar_v44 = v44;
        let v45: f64 = (p.p326 + v44);
        self.scalar_v45 = v45;
        let v46: f64 = (v40 * v45);
        self.scalar_v46 = v46;
        let v47: f64 = (1.0 - p.p327);
        self.scalar_v47 = v47;
        let v48: f64 = (v47 * p.p0);
        self.scalar_v48 = v48;
        let v49: f64 = (v48 / p.p325);
        self.scalar_v49 = v49;
        let v50: f64 = (v40 * v49);
        self.scalar_v50 = v50;
        let v58: f64 = p.p328;
        self.scalar_v58 = v58;
        let v59: bool = (p.p328 == 1.0);
        self.scalar_v59 = v59;
        let v60: f64 = p.p333;
        self.scalar_v60 = v60;
        let v65: f64 = p.p331;
        self.scalar_v65 = v65;
        let v68: f64 = p.p335;
        self.scalar_v68 = v68;
        let v71: f64 = p.p334;
        self.scalar_v71 = v71;
        let v95: bool = (p.p328 == 2.0);
        self.scalar_v95 = v95;
        let v96: bool = (!v59);
        self.scalar_v96 = v96;
        let v97: bool = (v96 && v95);
        self.scalar_v97 = v97;
        let v102: f64 = p.p78;
        self.scalar_v102 = v102;
        let v103: bool = (p.p78 == 1.0);
        self.scalar_v103 = v103;
        let v105: bool = (!v103);
        self.scalar_v105 = v105;
        let v106: f64 = p.p100;
        self.scalar_v106 = v106;
        let v107: bool = (p.p100 == 1.0);
        self.scalar_v107 = v107;
        let v108: bool = (!v107);
        self.scalar_v108 = v108;
        let v109: f64 = p.p122;
        self.scalar_v109 = v109;
        let v110: bool = (p.p122 == 1.0);
        self.scalar_v110 = v110;
        let v111: bool = (!v110);
        self.scalar_v111 = v111;
        let v112: f64 = p.p144;
        self.scalar_v112 = v112;
        let v113: bool = (p.p144 == 1.0);
        self.scalar_v113 = v113;
        let v114: bool = (!v113);
        self.scalar_v114 = v114;
        let v115: f64 = p.p166;
        self.scalar_v115 = v115;
        let v116: bool = (p.p166 == 1.0);
        self.scalar_v116 = v116;
        let v117: bool = (!v116);
        self.scalar_v117 = v117;
        let v118: f64 = p.p188;
        self.scalar_v118 = v118;
        let v119: bool = (p.p188 == 1.0);
        self.scalar_v119 = v119;
        let v120: bool = (!v119);
        self.scalar_v120 = v120;
        let v121: f64 = p.p210;
        self.scalar_v121 = v121;
        let v122: bool = (p.p210 == 1.0);
        self.scalar_v122 = v122;
        let v123: bool = (!v122);
        self.scalar_v123 = v123;
        let v124: f64 = p.p232;
        self.scalar_v124 = v124;
        let v125: bool = (p.p232 == 1.0);
        self.scalar_v125 = v125;
        let v126: bool = (!v125);
        self.scalar_v126 = v126;
        let v149: f64 = p.p322;
        self.scalar_v149 = v149;
        let v150: bool = (p.p322 == 0.0);
        self.scalar_v150 = v150;
        let v151: f64 = p.p291;
        self.scalar_v151 = v151;
        let v152: bool = (p.p291 == 1.0);
        self.scalar_v152 = v152;
        let v154: f64 = p.p311;
        self.scalar_v154 = v154;
        let v155: f64 = p.p310;
        self.scalar_v155 = v155;
        let v156: bool = (p.p310 != 0.0);
        self.scalar_v156 = v156;
        let v157: bool = (p.p311 != 0.0);
        self.scalar_v157 = v157;
        let v158: bool = (v156 && v157);
        self.scalar_v158 = v158;
        let v159: bool = (v152 && v158);
        self.scalar_v159 = v159;
        let v160: f64 = (p.p0 * p.p311);
        self.scalar_v160 = v160;
        let v161: f64 = (v160 * p.p2);
        self.scalar_v161 = v161;
        let v162: f64 = (p.p310 / v161);
        self.scalar_v162 = v162;
        let v163: f64 = (if v159 { v162 } else { 0.0 });
        self.scalar_v163 = v163;
        let v165: bool = (v46 >= p.p353);
        self.scalar_v165 = v165;
        let v166: bool = (v46 > 0.0);
        self.scalar_v166 = v166;
        let v167: bool = (v165 && v166);
        self.scalar_v167 = v167;
        let v168: bool = (v50 >= p.p353);
        self.scalar_v168 = v168;
        let v169: bool = (v50 > 0.0);
        self.scalar_v169 = v169;
        let v170: bool = (v168 && v169);
        self.scalar_v170 = v170;
        let v177: f64 = p.p329;
        self.scalar_v177 = v177;
        let v185: f64 = p.p340;
        self.scalar_v185 = v185;
        let v189: f64 = p.p339;
        self.scalar_v189 = v189;
        let v207: f64 = (if v125 { 0.0 } else { 0.0 });
        self.scalar_v207 = v207;
        let v208: f64 = (if v126 { 0.0 } else { 0.0 });
        self.scalar_v208 = v208;
        let v211: f64 = (if v122 { 0.0 } else { 0.0 });
        self.scalar_v211 = v211;
        let v212: f64 = (if v123 { 0.0 } else { 0.0 });
        self.scalar_v212 = v212;
        let v215: f64 = (if v119 { 0.0 } else { 0.0 });
        self.scalar_v215 = v215;
        let v216: f64 = (if v120 { 0.0 } else { 0.0 });
        self.scalar_v216 = v216;
        let v219: f64 = (if v116 { 0.0 } else { 0.0 });
        self.scalar_v219 = v219;
        let v220: f64 = (if v117 { 0.0 } else { 0.0 });
        self.scalar_v220 = v220;
        let v223: f64 = (if v103 { 0.0 } else { 0.0 });
        self.scalar_v223 = v223;
        let v224: f64 = (if v105 { 0.0 } else { 0.0 });
        self.scalar_v224 = v224;
        let v227: f64 = (if v107 { 0.0 } else { 0.0 });
        self.scalar_v227 = v227;
        let v228: f64 = (if v108 { 0.0 } else { 0.0 });
        self.scalar_v228 = v228;
        let v231: f64 = (if v110 { 0.0 } else { 0.0 });
        self.scalar_v231 = v231;
        let v232: f64 = (if v111 { 0.0 } else { 0.0 });
        self.scalar_v232 = v232;
        let v235: f64 = (if v113 { 0.0 } else { 0.0 });
        self.scalar_v235 = v235;
        let v236: f64 = (if v114 { 0.0 } else { 0.0 });
        self.scalar_v236 = v236;
        let v242: bool = (!v150);
        self.scalar_v242 = v242;
        let v266: f64 = (-p.p335);
        self.scalar_v266 = v266;
        let v267: f64 = (1.0 / p.p334);
        self.scalar_v267 = v267;
        let v268: f64 = (-1.0 / p.p334);
        self.scalar_v268 = v268;
        let v269: f64 = (v266 / p.p334);
        self.scalar_v269 = v269;
        let v273: f64 = (5.184705528587072e21 * v267);
        self.scalar_v273 = v273;
        let v274: f64 = (5.184705528587072e21 * v268);
        self.scalar_v274 = v274;
        let v275: f64 = (5.184705528587072e21 * v269);
        self.scalar_v275 = v275;
        let v288: f64 = (if v59 { 1.0 } else { 0.0 });
        self.scalar_v288 = v288;
        let v295: f64 = (1.0 / p.p329);
        self.scalar_v295 = v295;
        let v296: f64 = (if v59 { v295 } else { 0.0 });
        self.scalar_v296 = v296;
        let v299: f64 = (1.0 / p.p340);
        self.scalar_v299 = v299;
        let v300: f64 = (-1.0 / p.p340);
        self.scalar_v300 = v300;
        let v301: f64 = (if v97 { v299 } else { 0.0 });
        self.scalar_v301 = v301;
        let v302: f64 = (if v97 { v300 } else { 0.0 });
        self.scalar_v302 = v302;
        let v303: f64 = (1.0 / p.p339);
        self.scalar_v303 = v303;
        let v304: f64 = (-1.0 / p.p339);
        self.scalar_v304 = v304;
        let v305: f64 = (if v97 { v303 } else { 0.0 });
        self.scalar_v305 = v305;
        let v306: f64 = (if v97 { v304 } else { 0.0 });
        self.scalar_v306 = v306;
        let v308: f64 = (if v242 { 0.0 } else { 0.0 });
        self.scalar_v308 = v308;
        let v309: f64 = (if v242 { -0.0 } else { 0.0 });
        self.scalar_v309 = v309;
        let v310: f64 = (if v242 { 1.0 } else { 0.0 });
        self.scalar_v310 = v310;
        let v311: f64 = (-1.0 / v163);
        self.scalar_v311 = v311;
        let v312: f64 = (1.0 / v163);
        self.scalar_v312 = v312;
        let v313: f64 = (if v159 { v311 } else { 0.0 });
        self.scalar_v313 = v313;
        let v314: f64 = (if v159 { v312 } else { 0.0 });
        self.scalar_v314 = v314;
        let v315: f64 = (1.0 / v46);
        self.scalar_v315 = v315;
        let v316: f64 = (-1.0 / v46);
        self.scalar_v316 = v316;
        let v317: f64 = (if v167 { v315 } else { 0.0 });
        self.scalar_v317 = v317;
        let v318: f64 = (if v167 { v316 } else { 0.0 });
        self.scalar_v318 = v318;
        let v319: f64 = (1.0 / v50);
        self.scalar_v319 = v319;
        let v320: f64 = (-1.0 / v50);
        self.scalar_v320 = v320;
        let v321: f64 = (if v170 { v319 } else { 0.0 });
        self.scalar_v321 = v321;
        let v322: f64 = (if v170 { v320 } else { 0.0 });
        self.scalar_v322 = v322;
    }
}
