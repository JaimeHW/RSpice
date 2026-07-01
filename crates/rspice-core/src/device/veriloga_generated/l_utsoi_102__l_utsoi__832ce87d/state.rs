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
    pub p356: f64,
    pub p357: f64,
    pub p358: f64,
    pub p359: f64,
    pub p360: f64,
    pub p361: f64,
    pub p362: f64,
    pub p363: f64,
    pub p364: f64,
    pub p365: f64,
    pub p366: f64,
    pub p367: f64,
    pub p368: f64,
    pub p369: f64,
    pub p370: f64,
    pub p371: f64,
    pub p372: f64,
    pub p373: f64,
    pub p374: f64,
    pub p375: f64,
    pub p376: f64,
    pub p377: f64,
    pub p378: f64,
    pub p379: f64,
    pub p380: f64,
    pub p381: f64,
    pub p382: f64,
    pub p383: f64,
    pub p384: f64,
    pub p385: f64,
    pub p386: f64,
    pub p387: f64,
    pub p388: f64,
    pub p389: f64,
    pub p390: f64,
    pub p391: f64,
    pub p392: f64,
    pub p393: f64,
    pub p394: f64,
    pub p395: f64,
    pub p396: f64,
    pub p397: f64,
    pub p398: f64,
    pub p399: f64,
    pub p400: f64,
    pub p401: f64,
    pub p402: f64,
    pub p403: f64,
    pub p404: f64,
    pub p405: f64,
    pub p406: f64,
    pub p407: f64,
    pub p408: f64,
    pub p409: f64,
    pub p410: f64,
    pub p411: f64,
    pub p412: f64,
    pub p413: f64,
    pub p414: f64,
    pub p415: f64,
    pub p416: f64,
    pub p417: f64,
    pub p418: f64,
    pub p419: f64,
    pub p420: f64,
    pub p421: f64,
    pub p422: f64,
    pub p423: f64,
    pub p424: f64,
    pub p425: f64,
    pub p426: f64,
    pub p427: f64,
    pub p428: f64,
    pub p429: f64,
    pub p430: f64,
    pub p431: f64,
    pub p432: f64,
    pub p433: f64,
    pub p434: f64,
    pub p435: f64,
    pub p436: f64,
    pub p437: f64,
    pub p438: f64,
    pub p439: f64,
    pub p440: f64,
    pub p441: f64,
    pub p442: f64,
    pub p443: f64,
    pub p444: f64,
    pub p445: f64,
    pub p446: f64,
    pub p447: f64,
    pub p448: f64,
    pub p449: f64,
    pub p450: f64,
    pub p451: f64,
    pub p452: f64,
    pub p453: f64,
    pub p454: f64,
    pub p455: f64,
    pub p456: f64,
    pub p457: f64,
    pub p458: f64,
    pub p459: f64,
    pub p460: f64,
    pub p461: f64,
    pub p462: f64,
    pub p463: f64,
    pub p464: f64,
    pub p465: f64,
    pub p466: f64,
    pub p467: f64,
    pub p468: f64,
    pub p469: f64,
    pub p470: f64,
    pub p471: f64,
    pub p472: f64,
    pub p473: f64,
    pub p474: f64,
    pub p475: f64,
    pub p476: f64,
    pub p477: f64,
    pub p478: f64,
    pub p479: f64,
    pub p480: f64,
    pub p481: f64,
    pub p482: f64,
    pub p483: f64,
    pub p484: f64,
    pub p485: f64,
    pub p486: f64,
    pub p487: f64,
    pub p488: f64,
    pub p489: f64,
    pub p490: f64,
    pub p491: f64,
    pub p492: f64,
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
            params.p0 = 0.0;
            params.p1 = 102.8;
            params.p2 = 0.0;
            params.p3 = 0.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 1.0;
            params.p14 = 1.0;
            params.p15 = 21.0;
            params.p16 = 150.0;
            params.p17 = 1.0;
            params.p18 = 0.0;
            params.p19 = 0.001;
            params.p20 = 1e-6;
            params.p21 = 1e-6;
            params.p22 = 1e-12;
            params.p23 = 1e-12;
            params.p24 = 1e-6;
            params.p25 = 1e-6;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 1.0;
            params.p30 = 1.0;
            params.p31 = 1.0;
            params.p32 = 1.0;
            params.p33 = params.p31;
            validate_parameter("MULT_FN", params.p33, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p34 = 0.0;
            params.p35 = 1.0;
            params.p36 = 0.0;
            params.p37 = 1.0;
            params.p38 = 1e-7;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 2e-9;
            params.p42 = 1e-8;
            params.p43 = 0.0;
            params.p44 = 1e-7;
            params.p45 = 0.0;
            params.p46 = 3e18;
            params.p47 = 0.0;
            params.p48 = 2e-9;
            params.p49 = 1e20;
            params.p50 = 1e20;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 1e21;
            params.p55 = 1.0;
            params.p56 = 1.0;
            params.p57 = 0.0;
            params.p58 = 1.0;
            params.p59 = 1e22;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 1.0;
            params.p64 = 0.0;
            params.p65 = 0.2;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.05;
            params.p69 = 1.0;
            params.p70 = 1.5;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 1.5;
            params.p76 = 0.0;
            params.p77 = 2.0;
            params.p78 = 1.0;
            params.p79 = 0.0;
            params.p80 = 0.0;
            params.p81 = 1.5;
            params.p82 = 0.0;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 0.0;
            params.p86 = 1.0;
            params.p87 = 30.0;
            params.p88 = 0.0;
            params.p89 = 0.0;
            params.p90 = 0.0;
            params.p91 = 2.0;
            params.p92 = 0.0;
            params.p93 = 0.0;
            params.p94 = -0.1;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 8.0;
            params.p98 = 0.0;
            params.p99 = 0.0;
            params.p100 = 0.0;
            params.p101 = 0.05;
            params.p102 = 0.0;
            params.p103 = 0.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 0.0;
            params.p108 = 0.0;
            params.p109 = 0.0;
            params.p110 = 0.375;
            params.p111 = 0.063;
            params.p112 = 0.375;
            params.p113 = 0.063;
            params.p114 = 0.375;
            params.p115 = 0.063;
            params.p116 = 0.0;
            params.p117 = 1.0;
            params.p118 = 3.1;
            params.p119 = 0.0;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.2;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = 0.0;
            params.p126 = 41.0;
            params.p127 = 41.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 0.0;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 0.0;
            params.p138 = 1.0;
            params.p139 = 1.0;
            params.p140 = 0.0;
            params.p141 = 1.0;
            params.p142 = 0.0;
            params.p143 = 1.0;
            params.p144 = 0.2;
            params.p145 = 0.05;
            params.p146 = 1.5;
            params.p147 = 1.0;
            params.p148 = 10.0;
            params.p149 = 0.0;
            params.p150 = 1.0;
            params.p151 = 1e-12;
            params.p152 = 0.0;
            params.p153 = 1e22;
            params.p154 = 0.0;
            params.p155 = 0.0;
            params.p156 = 0.0;
            params.p157 = 0.0;
            params.p158 = 0.0;
            params.p159 = 0.0;
            params.p160 = 0.0;
            params.p161 = 8.0;
            params.p162 = 0.0;
            params.p163 = 0.0;
            params.p164 = 0.0;
            params.p165 = 0.0;
            params.p166 = 0.0;
            params.p167 = 0.0;
            params.p168 = 0.0;
            params.p169 = 0.0;
            params.p170 = 1.04e-18;
            params.p171 = 0.0;
            params.p172 = 10000.0;
            params.p173 = 0.0;
            params.p174 = 1e-11;
            params.p175 = 1.0;
            params.p176 = 0.0;
            params.p177 = 8e22;
            params.p178 = 30000000.0;
            params.p179 = 0.0;
            params.p180 = 0.0;
            params.p181 = 0.0;
            params.p182 = 1.0;
            params.p183 = 0.0;
            params.p184 = 0.0;
            params.p185 = 0.0;
            params.p186 = 0.0;
            params.p187 = 0.0;
            params.p188 = 0.0;
            params.p189 = 0.0;
            params.p190 = 0.0;
            params.p191 = 0.0;
            params.p192 = 0.0;
            params.p193 = 0.0;
            params.p194 = 0.0;
            params.p195 = 0.0;
            params.p196 = 0.0;
            params.p197 = 2e-9;
            params.p198 = 1e-8;
            params.p199 = 0.0;
            params.p200 = 1e-7;
            params.p201 = 0.0;
            params.p202 = 3e18;
            params.p203 = 0.0;
            params.p204 = 2e-9;
            params.p205 = 1e20;
            params.p206 = 1e20;
            params.p207 = 0.0;
            params.p208 = 0.0;
            params.p209 = 2.0;
            params.p210 = 0.0;
            params.p211 = 2.0;
            params.p212 = 0.0;
            params.p213 = 0.0;
            params.p214 = 0.0;
            params.p215 = 0.0;
            params.p216 = 0.0;
            params.p217 = 0.0;
            params.p218 = 0.0;
            params.p219 = 0.0;
            params.p220 = 1e21;
            params.p221 = 0.0;
            params.p222 = 1.0;
            params.p223 = 1.0;
            params.p224 = 0.0;
            params.p225 = 2.0;
            params.p226 = 0.0;
            params.p227 = 1.0;
            params.p228 = 1e22;
            params.p229 = 0.0;
            params.p230 = 0.0;
            params.p231 = 0.0;
            params.p232 = 2.0;
            params.p233 = 0.0;
            params.p234 = 1.0;
            params.p235 = 0.0;
            params.p236 = 0.2;
            params.p237 = 0.0;
            params.p238 = 0.0;
            params.p239 = 0.0;
            params.p240 = 0.05;
            params.p241 = 0.0;
            params.p242 = 0.0;
            params.p243 = 1e-8;
            params.p244 = 0.0;
            params.p245 = 0.0;
            params.p246 = 1e-8;
            params.p247 = 0.0;
            params.p248 = 0.0;
            params.p249 = 1e-8;
            params.p250 = 1.0;
            params.p251 = 1.5;
            params.p252 = 0.0;
            params.p253 = 0.0;
            params.p254 = 0.0;
            params.p255 = 0.0;
            params.p256 = 0.0;
            params.p257 = 1.0;
            params.p258 = 0.0;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 0.0;
            params.p262 = 0.0;
            params.p263 = 0.0;
            params.p264 = 0.0;
            params.p265 = 0.0;
            params.p266 = 1.5;
            params.p267 = 0.0;
            params.p268 = 2.0;
            params.p269 = 1.0;
            params.p270 = 0.0;
            params.p271 = 0.0;
            params.p272 = 1.5;
            params.p273 = 0.0;
            params.p274 = 0.0;
            params.p275 = 0.0;
            params.p276 = 1.0;
            params.p277 = 0.0;
            params.p278 = 0.0;
            params.p279 = 1.0;
            params.p280 = 0.0;
            params.p281 = 1.0;
            params.p282 = 30.0;
            params.p283 = 0.0;
            params.p284 = 0.0;
            params.p285 = 0.0;
            params.p286 = 0.0;
            params.p287 = 2.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = 0.0;
            params.p291 = 1.0;
            params.p292 = 0.0;
            params.p293 = 0.0;
            params.p294 = -0.1;
            params.p295 = 0.0;
            params.p296 = 0.0;
            params.p297 = 0.0;
            params.p298 = 0.0;
            params.p299 = 0.0;
            params.p300 = 8.0;
            params.p301 = 0.0;
            params.p302 = 1.0;
            params.p303 = 0.0;
            params.p304 = 1.5;
            params.p305 = 0.0;
            params.p306 = 1.0;
            params.p307 = 0.0;
            params.p308 = 2.0;
            params.p309 = 0.0;
            params.p310 = 0.0;
            params.p311 = 0.5;
            params.p312 = 0.0;
            params.p313 = 1.5;
            params.p314 = 0.0;
            params.p315 = 0.0;
            params.p316 = 0.05;
            params.p317 = 0.0;
            params.p318 = 0.0;
            params.p319 = 0.0;
            params.p320 = 0.0;
            params.p321 = 0.0;
            params.p322 = 0.0;
            params.p323 = 0.0;
            params.p324 = 0.0;
            params.p325 = 0.375;
            params.p326 = 0.063;
            params.p327 = 0.375;
            params.p328 = 0.063;
            params.p329 = 0.375;
            params.p330 = 0.063;
            params.p331 = 0.0;
            params.p332 = 1.0;
            params.p333 = 3.1;
            params.p334 = 0.0;
            params.p335 = 0.0;
            params.p336 = 0.0;
            params.p337 = 0.2;
            params.p338 = 0.0;
            params.p339 = 0.0;
            params.p340 = 0.0;
            params.p341 = 0.0;
            params.p342 = 0.0;
            params.p343 = 41.0;
            params.p344 = 41.0;
            params.p345 = 0.0;
            params.p346 = 0.0;
            params.p347 = 0.0;
            params.p348 = 0.0;
            params.p349 = 0.0;
            params.p350 = 0.0;
            params.p351 = 0.0;
            params.p352 = 0.0;
            params.p353 = 1e-8;
            params.p354 = 0.0;
            params.p355 = 0.0;
            params.p356 = 0.0;
            params.p357 = 0.0;
            params.p358 = 2.0;
            params.p359 = 0.0;
            params.p360 = 0.0;
            params.p361 = 0.0;
            params.p362 = 0.0;
            params.p363 = 0.0;
            params.p364 = 0.0;
            params.p365 = 0.0;
            params.p366 = 1.0;
            params.p367 = 1.0;
            params.p368 = 0.0;
            params.p369 = 2.0;
            params.p370 = 0.0;
            params.p371 = 1.0;
            params.p372 = 0.0;
            params.p373 = 2.0;
            params.p374 = 0.0;
            params.p375 = 1.0;
            params.p376 = 0.2;
            params.p377 = 0.0;
            params.p378 = 1e-8;
            params.p379 = 0.0;
            params.p380 = 1.0;
            params.p381 = 0.0;
            params.p382 = 0.0;
            params.p383 = 0.0;
            params.p384 = 1.0;
            params.p385 = 0.0;
            params.p386 = 0.0;
            params.p387 = 10.0;
            params.p388 = 0.0;
            params.p389 = 1.0;
            params.p390 = 0.0;
            params.p391 = 0.0;
            params.p392 = 0.0;
            params.p393 = 0.0;
            params.p394 = 1e22;
            params.p395 = 0.0;
            params.p396 = 0.0;
            params.p397 = 0.0;
            params.p398 = 0.0;
            params.p399 = 2.0;
            params.p400 = 0.0;
            params.p401 = 2.0;
            params.p402 = 0.0;
            params.p403 = 0.0;
            params.p404 = 0.0;
            params.p405 = 0.0;
            params.p406 = 0.0;
            params.p407 = 2.0;
            params.p408 = 0.0;
            params.p409 = 0.0;
            params.p410 = 2.0;
            params.p411 = 0.0;
            params.p412 = 0.0;
            params.p413 = 0.0;
            params.p414 = 1.0;
            params.p415 = 0.0;
            params.p416 = 0.0;
            params.p417 = 8.0;
            params.p418 = 0.0;
            params.p419 = 1.0;
            params.p420 = 0.0;
            params.p421 = 1.5;
            params.p422 = 0.0;
            params.p423 = 1.0;
            params.p424 = 0.0;
            params.p425 = 2.0;
            params.p426 = 0.0;
            params.p427 = 0.0;
            params.p428 = 0.0;
            params.p429 = 0.0;
            params.p430 = 0.0;
            params.p431 = 0.0;
            params.p432 = 0.0;
            params.p433 = 0.0;
            params.p434 = 0.0;
            params.p435 = 0.0;
            params.p436 = 0.0;
            params.p437 = 1.0;
            params.p438 = 0.0;
            params.p439 = 100000.0;
            params.p440 = 1.5;
            params.p441 = 3.0;
            params.p442 = 4.5;
            params.p443 = 0.0;
            params.p444 = 1e-12;
            params.p445 = 1e-7;
            params.p446 = 0.0;
            params.p447 = 1.0;
            params.p448 = 0.0;
            params.p449 = 2.0;
            params.p450 = 8e22;
            params.p451 = 0.0;
            params.p452 = 30000000.0;
            params.p453 = 0.0;
            params.p454 = 0.0;
            params.p455 = 0.0;
            params.p456 = 1.0;
            params.p457 = 1.0;
            params.p458 = 1e-6;
            params.p459 = 1e-6;
            params.p460 = 0.0;
            params.p461 = 0.0;
            params.p462 = 0.0;
            params.p463 = 0.0;
            params.p464 = 0.0;
            params.p465 = 0.0;
            params.p466 = 0.0;
            params.p467 = 0.0;
            params.p468 = 0.0;
            params.p469 = 0.0;
            params.p470 = 0.0;
            params.p471 = 0.0;
            params.p472 = 0.0;
            params.p473 = 0.0;
            params.p474 = 0.0;
            params.p475 = 0.0;
            params.p476 = 1.0;
            params.p477 = 1e-7;
            params.p478 = 3.0;
            params.p479 = 0.0;
            params.p480 = 0.0;
            params.p481 = 0.0;
            params.p482 = 0.0;
            params.p483 = 0.0;
            params.p484 = 0.0;
            params.p485 = 0.0;
            params.p486 = 0.0;
            params.p487 = 0.0;
            params.p488 = 0.0;
            params.p489 = 0.0;
            params.p490 = 0.0;
            params.p491 = 0.0;
            params.p492 = 0.0;
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
    pub nodes: [usize; 10],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 493]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 19]>,
    pub(crate) ddt_state_previous: Box<[f64; 19]>,
    pub(crate) ddt_state_older: Box<[f64; 19]>,
    pub(crate) ddt_state_initialized: Box<[bool; 19]>,
    pub(crate) ddt_derivative_current: Box<[f64; 19]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 19]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v4: bool,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: bool,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v22: bool,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v28: f64,
    pub(crate) scalar_v29: f64,
    pub(crate) scalar_v30: f64,
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: f64,
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
    pub(crate) scalar_v51: f64,
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v56: bool,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: bool,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: bool,
    pub(crate) scalar_v64: f64,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: bool,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: f64,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v78: f64,
    pub(crate) scalar_v79: f64,
    pub(crate) scalar_v80: f64,
    pub(crate) scalar_v81: f64,
    pub(crate) scalar_v82: f64,
    pub(crate) scalar_v83: f64,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: bool,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: bool,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: bool,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v107: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v115: f64,
    pub(crate) scalar_v116: f64,
    pub(crate) scalar_v117: f64,
    pub(crate) scalar_v118: bool,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: bool,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: bool,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: bool,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: bool,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: bool,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: bool,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: bool,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
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
            scalar_v3: self.scalar_v3,
            scalar_v4: self.scalar_v4,
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
            scalar_v17: self.scalar_v17,
            scalar_v18: self.scalar_v18,
            scalar_v19: self.scalar_v19,
            scalar_v20: self.scalar_v20,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v28: self.scalar_v28,
            scalar_v29: self.scalar_v29,
            scalar_v30: self.scalar_v30,
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v35: self.scalar_v35,
            scalar_v36: self.scalar_v36,
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
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
            scalar_v60: self.scalar_v60,
            scalar_v61: self.scalar_v61,
            scalar_v62: self.scalar_v62,
            scalar_v63: self.scalar_v63,
            scalar_v64: self.scalar_v64,
            scalar_v65: self.scalar_v65,
            scalar_v66: self.scalar_v66,
            scalar_v67: self.scalar_v67,
            scalar_v68: self.scalar_v68,
            scalar_v69: self.scalar_v69,
            scalar_v70: self.scalar_v70,
            scalar_v71: self.scalar_v71,
            scalar_v72: self.scalar_v72,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v77: self.scalar_v77,
            scalar_v78: self.scalar_v78,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
            scalar_v83: self.scalar_v83,
            scalar_v84: self.scalar_v84,
            scalar_v85: self.scalar_v85,
            scalar_v86: self.scalar_v86,
            scalar_v87: self.scalar_v87,
            scalar_v88: self.scalar_v88,
            scalar_v89: self.scalar_v89,
            scalar_v90: self.scalar_v90,
            scalar_v91: self.scalar_v91,
            scalar_v92: self.scalar_v92,
            scalar_v93: self.scalar_v93,
            scalar_v94: self.scalar_v94,
            scalar_v95: self.scalar_v95,
            scalar_v96: self.scalar_v96,
            scalar_v97: self.scalar_v97,
            scalar_v98: self.scalar_v98,
            scalar_v99: self.scalar_v99,
            scalar_v100: self.scalar_v100,
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
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
            scalar_v127: self.scalar_v127,
            scalar_v128: self.scalar_v128,
            scalar_v129: self.scalar_v129,
            scalar_v130: self.scalar_v130,
            scalar_v131: self.scalar_v131,
            scalar_v132: self.scalar_v132,
            scalar_v133: self.scalar_v133,
            scalar_v134: self.scalar_v134,
            scalar_v135: self.scalar_v135,
            scalar_v136: self.scalar_v136,
            scalar_v137: self.scalar_v137,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v147: self.scalar_v147,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
            scalar_v158: self.scalar_v158,
            scalar_v159: self.scalar_v159,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v170: self.scalar_v170,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
            scalar_v177: self.scalar_v177,
            scalar_v178: self.scalar_v178,
            scalar_v179: self.scalar_v179,
            scalar_v180: self.scalar_v180,
            scalar_v181: self.scalar_v181,
            scalar_v182: self.scalar_v182,
            scalar_v183: self.scalar_v183,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 5;
    pub const NODE_COUNT: usize = 10;
    pub const INTERNAL_NODE_NAMES: [&str; 5] = ["NSIG", "si", "di", "bp", "gp"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 493;
    pub const VARIABLE_COUNT: usize = 1901;
    pub const DDT_STATE_COUNT: usize = 19;
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
            scalar_v3: 0.0,
            scalar_v4: false,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v9: 0.0,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: false,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v22: false,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v28: 0.0,
            scalar_v29: 0.0,
            scalar_v30: 0.0,
            scalar_v31: 0.0,
            scalar_v32: 0.0,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v35: 0.0,
            scalar_v36: 0.0,
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
            scalar_v51: 0.0,
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v56: false,
            scalar_v57: 0.0,
            scalar_v58: false,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v63: false,
            scalar_v64: 0.0,
            scalar_v65: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: false,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v74: 0.0,
            scalar_v75: 0.0,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
            scalar_v78: 0.0,
            scalar_v79: 0.0,
            scalar_v80: 0.0,
            scalar_v81: 0.0,
            scalar_v82: 0.0,
            scalar_v83: 0.0,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: false,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: false,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: false,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: false,
            scalar_v105: false,
            scalar_v106: 0.0,
            scalar_v107: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v115: 0.0,
            scalar_v116: 0.0,
            scalar_v117: 0.0,
            scalar_v118: false,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: false,
            scalar_v122: 0.0,
            scalar_v123: false,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v126: false,
            scalar_v127: 0.0,
            scalar_v128: false,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: false,
            scalar_v132: 0.0,
            scalar_v133: false,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: false,
            scalar_v137: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v147: 0.0,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v158: 0.0,
            scalar_v159: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
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
            scalar_v3,
            scalar_v4,
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
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
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
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
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
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v152,
            scalar_v153,
            scalar_v158,
            scalar_v159,
            scalar_v164,
            scalar_v165,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
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
            scalar_v3,
            scalar_v4,
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
            scalar_v17,
            scalar_v18,
            scalar_v19,
            scalar_v20,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v28,
            scalar_v29,
            scalar_v30,
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v35,
            scalar_v36,
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
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
            scalar_v60,
            scalar_v61,
            scalar_v62,
            scalar_v63,
            scalar_v64,
            scalar_v65,
            scalar_v66,
            scalar_v67,
            scalar_v68,
            scalar_v69,
            scalar_v70,
            scalar_v71,
            scalar_v72,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v78,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
            scalar_v83,
            scalar_v84,
            scalar_v85,
            scalar_v86,
            scalar_v87,
            scalar_v88,
            scalar_v89,
            scalar_v90,
            scalar_v91,
            scalar_v92,
            scalar_v93,
            scalar_v94,
            scalar_v95,
            scalar_v96,
            scalar_v97,
            scalar_v98,
            scalar_v99,
            scalar_v100,
            scalar_v101,
            scalar_v102,
            scalar_v103,
            scalar_v104,
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
            scalar_v127,
            scalar_v128,
            scalar_v129,
            scalar_v130,
            scalar_v131,
            scalar_v132,
            scalar_v133,
            scalar_v134,
            scalar_v135,
            scalar_v136,
            scalar_v137,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v147,
            scalar_v152,
            scalar_v153,
            scalar_v158,
            scalar_v159,
            scalar_v164,
            scalar_v165,
            scalar_v170,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
            scalar_v177,
            scalar_v178,
            scalar_v179,
            scalar_v180,
            scalar_v181,
            scalar_v182,
            scalar_v183,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "swscale" => { validate_parameter("SWSCALE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "swsubdep" => { validate_parameter("SWSUBDEP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "swgidl" => { validate_parameter("SWGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "swshe" => { validate_parameter("SWSHE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "swign" => { validate_parameter("SWIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "swjunasym" => { validate_parameter("SWJUNASYM", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "swimpact" => { validate_parameter("SWIMPACT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "swpdep" => { validate_parameter("SWPDEP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "swcryo" => { validate_parameter("SWCRYO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "swqmod" => { validate_parameter("SWQMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "swedge" => { validate_parameter("SWEDGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "atmin" => { validate_parameter("ATMIN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "btmin" => { validate_parameter("BTMIN", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "asource" => { validate_parameter("ASOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "adrain" => { validate_parameter("ADRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "psource" => { validate_parameter("PSOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "pdrain" => { validate_parameter("PDRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "mult" => { validate_parameter("MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "mult_fn" => { validate_parameter("MULT_FN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "delvto" => { validate_finite_parameter("DELVTO", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "factuo" => { validate_parameter("FACTUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "nrs" => { validate_finite_parameter("NRS", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "nrd" => { validate_finite_parameter("NRD", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "toxe" => { validate_parameter("TOXE", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "tsi" => { validate_parameter("TSI", value, Some((3e-9, "3e-9")), false, Some((2e-8, "2e-8")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "xge" => { validate_parameter("XGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "nch" => { validate_finite_parameter("NCH", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "nsub" => { validate_finite_parameter("NSUB", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "ct" => { validate_parameter("CT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "toxp" => { validate_parameter("TOXP", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "nov" => { validate_parameter("NOV", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "novd" => { validate_parameter("NOVD", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "vfb" => { validate_finite_parameter("VFB", value)?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "vfbb" => { validate_finite_parameter("VFBB", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "np" => { validate_parameter("NP", value, Some((1e19, "1e19")), false, Some((1e22, "1e22")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "cicf" => { validate_parameter("CICF", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "cic" => { validate_parameter("CIC", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "psce" => { validate_parameter("PSCE", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "psceb" => { validate_parameter("PSCEB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "nsddc" => { validate_parameter("NSDDC", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "pscedlb" => { validate_parameter("PSCEDLB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "pnce" => { validate_parameter("PNCE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "cf" => { validate_parameter("CF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "cfb" => { validate_parameter("CFB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "stcf" => { validate_finite_parameter("STCF", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "cfd" => { validate_parameter("CFD", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "cfdl" => { validate_finite_parameter("CFDL", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "cfdlb" => { validate_parameter("CFDLB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "betn" => { validate_parameter("BETN", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "betnb" => { validate_parameter("BETNB", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "stbet" => { validate_finite_parameter("STBET", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "cs" => { validate_parameter("CS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "csfi" => { validate_parameter("CSFI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "csbi" => { validate_parameter("CSBI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "stcs" => { validate_finite_parameter("STCS", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "thecs" => { validate_parameter("THECS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "stthecs" => { validate_finite_parameter("STTHECS", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "csthr" => { validate_parameter("CSTHR", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "csthrb" => { validate_parameter("CSTHRB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "mue" => { validate_parameter("MUE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "stmue" => { validate_finite_parameter("STMUE", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "themu" => { validate_parameter("THEMU", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "stthemu" => { validate_finite_parameter("STTHEMU", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "xcor" => { validate_finite_parameter("XCOR", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "xcorb" => { validate_finite_parameter("XCORB", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "stxcor" => { validate_finite_parameter("STXCOR", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "rsig" => { validate_parameter("RSIG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "strs" => { validate_finite_parameter("STRS", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "rsg" => { validate_parameter("RSG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "thersg" => { validate_finite_parameter("THERSG", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "rsb" => { validate_finite_parameter("RSB", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "thesat" => { validate_parameter("THESAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "stthesat" => { validate_finite_parameter("STTHESAT", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "thesatg" => { validate_parameter("THESATG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "thesatb" => { validate_parameter("THESATB", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "ax" => { validate_parameter("AX", value, Some((1.0, "1.0")), false, Some((16.0, "16.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "alp" => { validate_parameter("ALP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "alp1" => { validate_parameter("ALP1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "alpb" => { validate_finite_parameter("ALPB", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "vp" => { validate_parameter("VP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "vpg" => { validate_parameter("VPG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "gco" => { validate_parameter("GCO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "iginv" => { validate_parameter("IGINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "igovinv" => { validate_parameter("IGOVINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "igovinvd" => { validate_parameter("IGOVINVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "igovacc" => { validate_parameter("IGOVACC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "igovaccd" => { validate_parameter("IGOVACCD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "gc2ch" => { validate_parameter("GC2CH", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "gc3ch" => { validate_parameter("GC3CH", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "gc2ovinv" => { validate_parameter("GC2OVINV", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "gc3ovinv" => { validate_parameter("GC3OVINV", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "gc2ovacc" => { validate_parameter("GC2OVACC", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "gc3ovacc" => { validate_parameter("GC3OVACC", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "gcdov" => { validate_finite_parameter("GCDOV", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "gcvdov" => { validate_finite_parameter("GCVDOV", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "chib" => { validate_parameter("CHIB", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "niginv" => { validate_parameter("NIGINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "fnovinv" => { validate_parameter("FNOVINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "fnovinvd" => { validate_parameter("FNOVINVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "gcovinvfn" => { validate_parameter("GCOVINVFN", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "stigfn" => { validate_finite_parameter("STIGFN", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "agidl" => { validate_parameter("AGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "agidld" => { validate_parameter("AGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "bgidl" => { validate_parameter("BGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "bgidld" => { validate_parameter("BGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "stbgidl" => { validate_finite_parameter("STBGIDL", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "stbgidld" => { validate_finite_parameter("STBGIDLD", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "cgidl" => { validate_finite_parameter("CGIDL", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "cgidld" => { validate_finite_parameter("CGIDLD", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "dgidl" => { validate_finite_parameter("DGIDL", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "dgidld" => { validate_finite_parameter("DGIDLD", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "ctedge" => { validate_parameter("CTEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "vfbedge" => { validate_finite_parameter("VFBEDGE", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "vfbbedge" => { validate_finite_parameter("VFBBEDGE", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "stvfbedge" => { validate_finite_parameter("STVFBEDGE", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "cicfedge" => { validate_parameter("CICFEDGE", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "cicedge" => { validate_parameter("CICEDGE", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "psceedge" => { validate_parameter("PSCEEDGE", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "pscebedge" => { validate_parameter("PSCEBEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "cfedge" => { validate_parameter("CFEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "cfbedge" => { validate_parameter("CFBEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "cfdedge" => { validate_parameter("CFDEDGE", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "betnedge" => { validate_parameter("BETNEDGE", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "stbetedge" => { validate_finite_parameter("STBETEDGE", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "a1" => { validate_parameter("A1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "a2" => { validate_parameter("A2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "sta2" => { validate_finite_parameter("STA2", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "a3" => { validate_parameter("A3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "areaq" => { validate_parameter("AREAQ", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "cgbov" => { validate_parameter("CGBOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "nsdac" => { validate_parameter("NSDAC", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "fif" => { validate_parameter("FIF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "fsceac" => { validate_parameter("FSCEAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "vfbac" => { validate_finite_parameter("VFBAC", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "vfbbac" => { validate_finite_parameter("VFBBAC", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "psceac" => { validate_parameter("PSCEAC", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "cfac" => { validate_parameter("CFAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "thesatac" => { validate_parameter("THESATAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "axac" => { validate_parameter("AXAC", value, Some((1.0, "1.0")), false, Some((16.0, "16.0")), true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "alpac" => { validate_parameter("ALPAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "cov" => { validate_parameter("COV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "covd" => { validate_parameter("COVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "covdl" => { validate_finite_parameter("COVDL", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "covdlb" => { validate_finite_parameter("COVDLB", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "dvfbov" => { validate_finite_parameter("DVFBOV", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "cfr" => { validate_parameter("CFR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "cfrd" => { validate_parameter("CFRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "csd" => { validate_parameter("CSD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "csdbp" => { validate_parameter("CSDBP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "rth" => { validate_parameter("RTH", value, Some((1e-6, "1e-6")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "strth" => { validate_finite_parameter("STRTH", value)?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "cth" => { validate_parameter("CTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "fnt" => { validate_parameter("FNT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "fntexc" => { validate_parameter("FNTEXC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "nfa" => { validate_parameter("NFA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "nfb" => { validate_parameter("NFB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "nfc" => { validate_parameter("NFC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "nfe" => { validate_parameter("NFE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "nfeb" => { validate_parameter("NFEB", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "rg" => { validate_parameter("RG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "rse" => { validate_parameter("RSE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "rde" => { validate_parameter("RDE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "rwell" => { validate_parameter("RWELL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "lvaro" => { validate_finite_parameter("LVARO", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "lvarl" => { validate_finite_parameter("LVARL", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "lvarw" => { validate_finite_parameter("LVARW", value)?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "lap" => { validate_finite_parameter("LAP", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "wvaro" => { validate_finite_parameter("WVARO", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "wvarl" => { validate_finite_parameter("WVARL", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "wvarw" => { validate_finite_parameter("WVARW", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "wot" => { validate_finite_parameter("WOT", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "toxeo" => { validate_parameter("TOXEO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "tsio" => { validate_parameter("TSIO", value, Some((3e-9, "3e-9")), false, Some((2e-8, "2e-8")), false, &[])?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "xgeo" => { validate_parameter("XGEO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "tboxo" => { validate_parameter("TBOXO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "ncho" => { validate_finite_parameter("NCHO", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "nsubo" => { validate_finite_parameter("NSUBO", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "cto" => { validate_parameter("CTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "toxpo" => { validate_parameter("TOXPO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "novdo" => { validate_parameter("NOVDO", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "vfbl" => { validate_finite_parameter("VFBL", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "vfblexp" => { validate_finite_parameter("VFBLEXP", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "vfbl2" => { validate_parameter("VFBL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "vfblexp2" => { validate_finite_parameter("VFBLEXP2", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "vfbw" => { validate_finite_parameter("VFBW", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "vfblw" => { validate_finite_parameter("VFBLW", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "vfbbo" => { validate_finite_parameter("VFBBO", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "vfblbo" => { validate_parameter("VFBLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "stvfbo" => { validate_finite_parameter("STVFBO", value)?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "stvfbl" => { validate_finite_parameter("STVFBL", value)?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "stvfbw" => { validate_finite_parameter("STVFBW", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "stvfblw" => { validate_finite_parameter("STVFBLW", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "npl" => { validate_finite_parameter("NPL", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "cicfo" => { validate_parameter("CICFO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "cico" => { validate_parameter("CICO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "pscel" => { validate_finite_parameter("PSCEL", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "pscelexp" => { validate_finite_parameter("PSCELEXP", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "pscew" => { validate_finite_parameter("PSCEW", value)?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "pscebo" => { validate_parameter("PSCEBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "nsddco" => { validate_parameter("NSDDCO", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "pscedlbo" => { validate_parameter("PSCEDLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "pncew" => { validate_finite_parameter("PNCEW", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "cfl" => { validate_finite_parameter("CFL", value)?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "cflexp" => { validate_finite_parameter("CFLEXP", value)?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "cfw" => { validate_finite_parameter("CFW", value)?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "cfbo" => { validate_parameter("CFBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "stcfl" => { validate_finite_parameter("STCFL", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "cfdo" => { validate_parameter("CFDO", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "cfdll" => { validate_finite_parameter("CFDLL", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "cfdlw" => { validate_finite_parameter("CFDLW", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "cfdlbo" => { validate_parameter("CFDLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "uo" => { validate_parameter("UO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "fbet1" => { validate_finite_parameter("FBET1", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "fbet1w" => { validate_finite_parameter("FBET1W", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "lp1" => { validate_parameter("LP1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "lp1w" => { validate_finite_parameter("LP1W", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "fbet2" => { validate_finite_parameter("FBET2", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "lp2" => { validate_parameter("LP2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "betw1" => { validate_finite_parameter("BETW1", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "betw2" => { validate_finite_parameter("BETW2", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "wbet" => { validate_parameter("WBET", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "betnbo" => { validate_parameter("BETNBO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "stbeto" => { validate_finite_parameter("STBETO", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "stbetl" => { validate_finite_parameter("STBETL", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "stbetw" => { validate_finite_parameter("STBETW", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "stbetlw" => { validate_finite_parameter("STBETLW", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "cso" => { validate_finite_parameter("CSO", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "csl" => { validate_finite_parameter("CSL", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "cslexp" => { validate_finite_parameter("CSLEXP", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "csw" => { validate_finite_parameter("CSW", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "cslw" => { validate_finite_parameter("CSLW", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "csfio" => { validate_parameter("CSFIO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "csbio" => { validate_parameter("CSBIO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "stcso" => { validate_finite_parameter("STCSO", value)?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "stcsl" => { validate_finite_parameter("STCSL", value)?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "stcsw" => { validate_finite_parameter("STCSW", value)?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "stcslw" => { validate_finite_parameter("STCSLW", value)?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "thecso" => { validate_parameter("THECSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "stthecso" => { validate_finite_parameter("STTHECSO", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "csthro" => { validate_parameter("CSTHRO", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "csthrbo" => { validate_parameter("CSTHRBO", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "mueo" => { validate_parameter("MUEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "stmueo" => { validate_finite_parameter("STMUEO", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "themuo" => { validate_parameter("THEMUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "stthemuo" => { validate_finite_parameter("STTHEMUO", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "xcoro" => { validate_finite_parameter("XCORO", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "xcorl" => { validate_finite_parameter("XCORL", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "xcorlexp" => { validate_finite_parameter("XCORLEXP", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "xcorw" => { validate_finite_parameter("XCORW", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "xcorlw" => { validate_finite_parameter("XCORLW", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "xcorbo" => { validate_finite_parameter("XCORBO", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "stxcoro" => { validate_finite_parameter("STXCORO", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "fetao" => { validate_parameter("FETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "rsw1" => { validate_finite_parameter("RSW1", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "rsw2" => { validate_finite_parameter("RSW2", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "rsigo" => { validate_parameter("RSIGO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "strso" => { validate_finite_parameter("STRSO", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "rsgo" => { validate_parameter("RSGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            "thersgo" => { validate_finite_parameter("THERSGO", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); Ok(()) }
            "rsbo" => { validate_finite_parameter("RSBO", value)?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); Ok(()) }
            "thesato" => { validate_parameter("THESATO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); Ok(()) }
            "thesatl" => { validate_finite_parameter("THESATL", value)?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); Ok(()) }
            "thesatlexp" => { validate_finite_parameter("THESATLEXP", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); Ok(()) }
            "thesatw" => { validate_finite_parameter("THESATW", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); Ok(()) }
            "thesatlw" => { validate_finite_parameter("THESATLW", value)?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); Ok(()) }
            "stthesato" => { validate_finite_parameter("STTHESATO", value)?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); Ok(()) }
            "stthesatl" => { validate_finite_parameter("STTHESATL", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); Ok(()) }
            "stthesatw" => { validate_finite_parameter("STTHESATW", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); Ok(()) }
            "stthesatlw" => { validate_finite_parameter("STTHESATLW", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); Ok(()) }
            "thesatgo" => { validate_parameter("THESATGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); Ok(()) }
            "thesatbo" => { validate_parameter("THESATBO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); Ok(()) }
            "axo" => { validate_finite_parameter("AXO", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); Ok(()) }
            "axl" => { validate_parameter("AXL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); Ok(()) }
            "axlexp" => { validate_finite_parameter("AXLEXP", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); Ok(()) }
            "axl2" => { validate_parameter("AXL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); Ok(()) }
            "axlexp2" => { validate_finite_parameter("AXLEXP2", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); Ok(()) }
            "alpl1" => { validate_finite_parameter("ALPL1", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); Ok(()) }
            "alplexp" => { validate_finite_parameter("ALPLEXP", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); Ok(()) }
            "alpl2" => { validate_parameter("ALPL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); Ok(()) }
            "alplexp2" => { validate_finite_parameter("ALPLEXP2", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); Ok(()) }
            "alpw" => { validate_finite_parameter("ALPW", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); Ok(()) }
            "alp1l1" => { validate_finite_parameter("ALP1L1", value)?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); Ok(()) }
            "alp1lexp" => { validate_finite_parameter("ALP1LEXP", value)?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); Ok(()) }
            "alp1l2" => { validate_parameter("ALP1L2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); Ok(()) }
            "alp1lexp2" => { validate_finite_parameter("ALP1LEXP2", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); Ok(()) }
            "alp1w" => { validate_finite_parameter("ALP1W", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); Ok(()) }
            "alpbo" => { validate_finite_parameter("ALPBO", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); Ok(()) }
            "vpo" => { validate_parameter("VPO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); Ok(()) }
            "vpgo" => { validate_parameter("VPGO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); Ok(()) }
            "igovinvw" => { validate_parameter("IGOVINVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); Ok(()) }
            "igovinvdw" => { validate_parameter("IGOVINVDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); Ok(()) }
            "igovaccw" => { validate_parameter("IGOVACCW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); Ok(()) }
            "igovaccdw" => { validate_parameter("IGOVACCDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); Ok(()) }
            "stigo" => { validate_finite_parameter("STIGO", value)?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); Ok(()) }
            "gc2cho" => { validate_parameter("GC2CHO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); Ok(()) }
            "gc3cho" => { validate_parameter("GC3CHO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); Ok(()) }
            "gc2ovinvo" => { validate_parameter("GC2OVINVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); Ok(()) }
            "gc3ovinvo" => { validate_parameter("GC3OVINVO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); Ok(()) }
            "gc2ovacco" => { validate_parameter("GC2OVACCO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); Ok(()) }
            "gc3ovacco" => { validate_parameter("GC3OVACCO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); Ok(()) }
            "gcdovl" => { validate_finite_parameter("GCDOVL", value)?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); Ok(()) }
            "gcvdovo" => { validate_finite_parameter("GCVDOVO", value)?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); Ok(()) }
            "niginvo" => { validate_parameter("NIGINVO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); Ok(()) }
            "fnovinvw" => { validate_parameter("FNOVINVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); Ok(()) }
            "fnovinvdw" => { validate_parameter("FNOVINVDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); Ok(()) }
            "gcovinvfno" => { validate_parameter("GCOVINVFNO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); Ok(()) }
            "stigfno" => { validate_finite_parameter("STIGFNO", value)?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); Ok(()) }
            "agidlo" => { validate_finite_parameter("AGIDLO", value)?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); Ok(()) }
            "agidldo" => { validate_finite_parameter("AGIDLDO", value)?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); Ok(()) }
            "agidlw" => { validate_finite_parameter("AGIDLW", value)?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); Ok(()) }
            "agidldw" => { validate_finite_parameter("AGIDLDW", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); Ok(()) }
            "bgidlo" => { validate_parameter("BGIDLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); Ok(()) }
            "bgidldo" => { validate_parameter("BGIDLDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); Ok(()) }
            "stbgidlo" => { validate_finite_parameter("STBGIDLO", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); Ok(()) }
            "stbgidldo" => { validate_finite_parameter("STBGIDLDO", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); Ok(()) }
            "cgidlo" => { validate_finite_parameter("CGIDLO", value)?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); Ok(()) }
            "cgidldo" => { validate_finite_parameter("CGIDLDO", value)?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); Ok(()) }
            "dgidlo" => { validate_finite_parameter("DGIDLO", value)?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); Ok(()) }
            "dgidldo" => { validate_finite_parameter("DGIDLDO", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); Ok(()) }
            "dgidll" => { validate_finite_parameter("DGIDLL", value)?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); Ok(()) }
            "dgidldl" => { validate_finite_parameter("DGIDLDL", value)?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); Ok(()) }
            "wedge" => { validate_parameter("WEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); Ok(()) }
            "wedgew" => { validate_parameter("WEDGEW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); Ok(()) }
            "ctedgeo" => { validate_parameter("CTEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); Ok(()) }
            "vfbedgeo" => { validate_finite_parameter("VFBEDGEO", value)?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); Ok(()) }
            "vfbedgel" => { validate_finite_parameter("VFBEDGEL", value)?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); Ok(()) }
            "vfbedgelexp" => { validate_finite_parameter("VFBEDGELEXP", value)?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); Ok(()) }
            "vfbedgew" => { validate_finite_parameter("VFBEDGEW", value)?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); Ok(()) }
            "vfbedgelw" => { validate_finite_parameter("VFBEDGELW", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); Ok(()) }
            "vfbbedgeo" => { validate_finite_parameter("VFBBEDGEO", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); Ok(()) }
            "stvfbedgeo" => { validate_finite_parameter("STVFBEDGEO", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); Ok(()) }
            "stvfbedgel" => { validate_finite_parameter("STVFBEDGEL", value)?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); Ok(()) }
            "stvfbedgew" => { validate_finite_parameter("STVFBEDGEW", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); Ok(()) }
            "stvfbedgelw" => { validate_finite_parameter("STVFBEDGELW", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); Ok(()) }
            "cicfedgeo" => { validate_parameter("CICFEDGEO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); Ok(()) }
            "cicedgeo" => { validate_parameter("CICEDGEO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); Ok(()) }
            "psceedgel" => { validate_finite_parameter("PSCEEDGEL", value)?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); Ok(()) }
            "psceedgelexp" => { validate_finite_parameter("PSCEEDGELEXP", value)?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); Ok(()) }
            "psceedgew" => { validate_finite_parameter("PSCEEDGEW", value)?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); Ok(()) }
            "pscebedgeo" => { validate_parameter("PSCEBEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); Ok(()) }
            "cfedgel" => { validate_finite_parameter("CFEDGEL", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); Ok(()) }
            "cfedgelexp" => { validate_finite_parameter("CFEDGELEXP", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); Ok(()) }
            "cfedgew" => { validate_finite_parameter("CFEDGEW", value)?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); Ok(()) }
            "cfbedgeo" => { validate_parameter("CFBEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); Ok(()) }
            "cfdedgeo" => { validate_parameter("CFDEDGEO", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); Ok(()) }
            "fbetedge" => { validate_finite_parameter("FBETEDGE", value)?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); Ok(()) }
            "lpedge" => { validate_parameter("LPEDGE", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); Ok(()) }
            "betedgew" => { validate_finite_parameter("BETEDGEW", value)?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); Ok(()) }
            "stbetedgeo" => { validate_finite_parameter("STBETEDGEO", value)?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); Ok(()) }
            "stbetedgel" => { validate_finite_parameter("STBETEDGEL", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); Ok(()) }
            "stbetedgew" => { validate_finite_parameter("STBETEDGEW", value)?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); Ok(()) }
            "stbetedgelw" => { validate_finite_parameter("STBETEDGELW", value)?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); Ok(()) }
            "a1o" => { validate_finite_parameter("A1O", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); Ok(()) }
            "a1l" => { validate_finite_parameter("A1L", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); Ok(()) }
            "a1w" => { validate_finite_parameter("A1W", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); Ok(()) }
            "a2o" => { validate_parameter("A2O", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); Ok(()) }
            "sta2o" => { validate_finite_parameter("STA2O", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); Ok(()) }
            "a3o" => { validate_finite_parameter("A3O", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); Ok(()) }
            "a3l" => { validate_finite_parameter("A3L", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); Ok(()) }
            "a3w" => { validate_finite_parameter("A3W", value)?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); Ok(()) }
            "cgbovo" => { validate_finite_parameter("CGBOVO", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); Ok(()) }
            "cgbovl" => { validate_finite_parameter("CGBOVL", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); Ok(()) }
            "nsdaco" => { validate_parameter("NSDACO", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); Ok(()) }
            "fifw" => { validate_parameter("FIFW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); Ok(()) }
            "fsceaco" => { validate_parameter("FSCEACO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); Ok(()) }
            "vfbaco" => { validate_finite_parameter("VFBACO", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); Ok(()) }
            "vfbacl" => { validate_finite_parameter("VFBACL", value)?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); Ok(()) }
            "vfbaclexp" => { validate_finite_parameter("VFBACLEXP", value)?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); Ok(()) }
            "vfbacl2" => { validate_parameter("VFBACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); Ok(()) }
            "vfbaclexp2" => { validate_finite_parameter("VFBACLEXP2", value)?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); Ok(()) }
            "vfbacw" => { validate_finite_parameter("VFBACW", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); Ok(()) }
            "vfbaclw" => { validate_finite_parameter("VFBACLW", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); Ok(()) }
            "vfbbaco" => { validate_finite_parameter("VFBBACO", value)?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); Ok(()) }
            "vfblbaco" => { validate_parameter("VFBLBACO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); Ok(()) }
            "psceacl" => { validate_finite_parameter("PSCEACL", value)?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); Ok(()) }
            "psceaclexp" => { validate_finite_parameter("PSCEACLEXP", value)?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); Ok(()) }
            "psceacw" => { validate_finite_parameter("PSCEACW", value)?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); Ok(()) }
            "cfacl" => { validate_finite_parameter("CFACL", value)?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); Ok(()) }
            "cfaclexp" => { validate_finite_parameter("CFACLEXP", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); Ok(()) }
            "cfacw" => { validate_finite_parameter("CFACW", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); Ok(()) }
            "thesataco" => { validate_finite_parameter("THESATACO", value)?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); Ok(()) }
            "thesatacl" => { validate_finite_parameter("THESATACL", value)?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); Ok(()) }
            "thesataclexp" => { validate_finite_parameter("THESATACLEXP", value)?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); Ok(()) }
            "thesatacw" => { validate_finite_parameter("THESATACW", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); Ok(()) }
            "thesataclw" => { validate_finite_parameter("THESATACLW", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); Ok(()) }
            "axaco" => { validate_finite_parameter("AXACO", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); Ok(()) }
            "axacl" => { validate_parameter("AXACL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); Ok(()) }
            "axaclexp" => { validate_finite_parameter("AXACLEXP", value)?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); Ok(()) }
            "axacl2" => { validate_parameter("AXACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); Ok(()) }
            "axaclexp2" => { validate_finite_parameter("AXACLEXP2", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); Ok(()) }
            "alpacl1" => { validate_finite_parameter("ALPACL1", value)?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); Ok(()) }
            "alpaclexp" => { validate_finite_parameter("ALPACLEXP", value)?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); Ok(()) }
            "alpacl2" => { validate_parameter("ALPACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); Ok(()) }
            "alpaclexp2" => { validate_finite_parameter("ALPACLEXP2", value)?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); Ok(()) }
            "alpacw" => { validate_finite_parameter("ALPACW", value)?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); Ok(()) }
            "lovo" => { validate_parameter("LOVO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); Ok(()) }
            "lovdo" => { validate_parameter("LOVDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); Ok(()) }
            "covdlo" => { validate_finite_parameter("COVDLO", value)?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); Ok(()) }
            "covdlw" => { validate_finite_parameter("COVDLW", value)?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); Ok(()) }
            "covdlbo" => { validate_finite_parameter("COVDLBO", value)?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); Ok(()) }
            "dvfbovo" => { validate_finite_parameter("DVFBOVO", value)?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); Ok(()) }
            "cfro" => { validate_finite_parameter("CFRO", value)?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); Ok(()) }
            "cfrdo" => { validate_finite_parameter("CFRDO", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); Ok(()) }
            "cfrw" => { validate_finite_parameter("CFRW", value)?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); Ok(()) }
            "cfrdw" => { validate_finite_parameter("CFRDW", value)?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); Ok(()) }
            "csdo" => { validate_parameter("CSDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); Ok(()) }
            "csdbpo" => { validate_parameter("CSDBPO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); Ok(()) }
            "rtho" => { validate_parameter("RTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); Ok(()) }
            "rthl" => { validate_finite_parameter("RTHL", value)?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); Ok(()) }
            "rthw" => { validate_finite_parameter("RTHW", value)?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); Ok(()) }
            "rthlw" => { validate_finite_parameter("RTHLW", value)?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); Ok(()) }
            "strtho" => { validate_finite_parameter("STRTHO", value)?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); Ok(()) }
            "ctho" => { validate_parameter("CTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); Ok(()) }
            "lambtho" => { validate_parameter("LAMBTHO", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); Ok(()) }
            "ftho" => { validate_parameter("FTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); Ok(()) }
            "fnto" => { validate_parameter("FNTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); Ok(()) }
            "fntexcl" => { validate_parameter("FNTEXCL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); Ok(()) }
            "fntexclexp" => { validate_finite_parameter("FNTEXCLEXP", value)?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); Ok(()) }
            "nfalw" => { validate_finite_parameter("NFALW", value)?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); Ok(()) }
            "nfaw" => { validate_finite_parameter("NFAW", value)?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); Ok(()) }
            "nfblw" => { validate_parameter("NFBLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); Ok(()) }
            "nfclw" => { validate_parameter("NFCLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); Ok(()) }
            "nfeo" => { validate_parameter("NFEO", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); Ok(()) }
            "nfebo" => { validate_parameter("NFEBO", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); Ok(()) }
            "efo" => { validate_parameter("EFO", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); Ok(()) }
            "swstress" => { validate_parameter("SWSTRESS", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); Ok(()) }
            "wlod" => { validate_finite_parameter("WLOD", value)?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); Ok(()) }
            "kuo" => { validate_finite_parameter("KUO", value)?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); Ok(()) }
            "kvsat" => { validate_parameter("KVSAT", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); Ok(()) }
            "tkuo" => { validate_finite_parameter("TKUO", value)?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); Ok(()) }
            "lkuo" => { validate_finite_parameter("LKUO", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); Ok(()) }
            "wkuo" => { validate_finite_parameter("WKUO", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); Ok(()) }
            "pkuo" => { validate_finite_parameter("PKUO", value)?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); Ok(()) }
            "llodkuo" => { validate_parameter("LLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); Ok(()) }
            "wlodkuo" => { validate_parameter("WLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); Ok(()) }
            "kvtho" => { validate_finite_parameter("KVTHO", value)?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); Ok(()) }
            "lkvtho" => { validate_finite_parameter("LKVTHO", value)?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); Ok(()) }
            "wkvtho" => { validate_finite_parameter("WKVTHO", value)?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); Ok(()) }
            "pkvtho" => { validate_finite_parameter("PKVTHO", value)?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); Ok(()) }
            "llodvth" => { validate_parameter("LLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); Ok(()) }
            "wlodvth" => { validate_parameter("WLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); Ok(()) }
            "stetao" => { validate_finite_parameter("STETAO", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); Ok(()) }
            "lodetao" => { validate_parameter("LODETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); Ok(()) }
            "strlambda" => { validate_parameter("STRLAMBDA", value, Some((1e-9, "1e-9")), false, Some((1e-5, "1e-5")), false, &[])?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); Ok(()) }
            "stralpha" => { validate_parameter("STRALPHA", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); Ok(()) }
            "strdvfbo" => { validate_finite_parameter("STRDVFBO", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); Ok(()) }
            "strwdvfbo" => { validate_finite_parameter("STRWDVFBO", value)?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); Ok(()) }
            "strdcfl" => { validate_finite_parameter("STRDCFL", value)?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); Ok(()) }
            "strruo" => { validate_finite_parameter("STRRUO", value)?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); Ok(()) }
            "strtruo" => { validate_finite_parameter("STRTRUO", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); Ok(()) }
            "strrvsat" => { validate_finite_parameter("STRRVSAT", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); Ok(()) }
            "rgo" => { validate_finite_parameter("RGO", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); Ok(()) }
            "rint" => { validate_parameter("RINT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); Ok(()) }
            "rvpoly" => { validate_parameter("RVPOLY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); Ok(()) }
            "dlsil" => { validate_finite_parameter("DLSIL", value)?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); Ok(()) }
            "rshd" => { validate_parameter("RSHD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); Ok(()) }
            "rwello" => { validate_parameter("RWELLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'l_utsoi'", name)),
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
        let v3: f64 = p.p0;
        self.scalar_v3 = v3;
        let v4: bool = (0.0 == p.p0);
        self.scalar_v4 = v4;
        let v5: f64 = p.p30;
        self.scalar_v5 = v5;
        let v6: f64 = (if v4 { p.p30 } else { 0.0 });
        self.scalar_v6 = v6;
        let v7: f64 = p.p183;
        self.scalar_v7 = v7;
        let v8: f64 = (if v4 { p.p183 } else { 0.0 });
        self.scalar_v8 = v8;
        let v9: f64 = p.p184;
        self.scalar_v9 = v9;
        let v10: f64 = (if v4 { p.p184 } else { 0.0 });
        self.scalar_v10 = v10;
        let v11: f64 = p.p185;
        self.scalar_v11 = v11;
        let v12: f64 = (if v4 { p.p185 } else { 0.0 });
        self.scalar_v12 = v12;
        let v13: f64 = p.p186;
        self.scalar_v13 = v13;
        let v14: f64 = (if v4 { p.p186 } else { 0.0 });
        self.scalar_v14 = v14;
        let v15: bool = (!v4);
        self.scalar_v15 = v15;
        let v16: f64 = p.p29;
        self.scalar_v16 = v16;
        let v17: f64 = (1.0 / p.p29);
        self.scalar_v17 = v17;
        let v18: f64 = (if v15 { v17 } else { 0.0 });
        self.scalar_v18 = v18;
        let v19: f64 = p.p21;
        self.scalar_v19 = v19;
        let v20: f64 = (v18 * p.p21);
        self.scalar_v20 = v20;
        let v22: bool = (v20 > 1e-9);
        self.scalar_v22 = v22;
        let v23: f64 = (if v22 { v20 } else { 1e-9 });
        self.scalar_v23 = v23;
        let v24: f64 = (if v15 { v23 } else { 0.0 });
        self.scalar_v24 = v24;
        let v25: f64 = (p.p30 * p.p29);
        self.scalar_v25 = v25;
        let v26: f64 = (if v15 { v25 } else { v6 });
        self.scalar_v26 = v26;
        let v28: f64 = (if v15 { 1e-6 } else { 0.0 });
        self.scalar_v28 = v28;
        let v29: f64 = p.p20;
        self.scalar_v29 = v29;
        let v30: f64 = (v28 / p.p20);
        self.scalar_v30 = v30;
        let v31: f64 = (if v15 { v30 } else { 0.0 });
        self.scalar_v31 = v31;
        let v32: f64 = (v28 / v24);
        self.scalar_v32 = v32;
        let v33: f64 = (if v15 { v32 } else { 0.0 });
        self.scalar_v33 = v33;
        let v34: f64 = p.p187;
        self.scalar_v34 = v34;
        let v35: f64 = p.p188;
        self.scalar_v35 = v35;
        let v36: f64 = (v31 * p.p188);
        self.scalar_v36 = v36;
        let v37: f64 = (1.0 + v36);
        self.scalar_v37 = v37;
        let v38: f64 = (p.p187 * v37);
        self.scalar_v38 = v38;
        let v39: f64 = p.p189;
        self.scalar_v39 = v39;
        let v40: f64 = (v33 * p.p189);
        self.scalar_v40 = v40;
        let v41: f64 = (1.0 + v40);
        self.scalar_v41 = v41;
        let v42: f64 = (v38 * v41);
        self.scalar_v42 = v42;
        let v43: f64 = (if v15 { v42 } else { 0.0 });
        self.scalar_v43 = v43;
        let v44: f64 = p.p191;
        self.scalar_v44 = v44;
        let v45: f64 = p.p193;
        self.scalar_v45 = v45;
        let v46: f64 = (v33 * p.p193);
        self.scalar_v46 = v46;
        let v47: f64 = (1.0 + v46);
        self.scalar_v47 = v47;
        let v48: f64 = (p.p191 * v47);
        self.scalar_v48 = v48;
        let v49: f64 = p.p192;
        self.scalar_v49 = v49;
        let v50: f64 = (v31 * p.p192);
        self.scalar_v50 = v50;
        let v51: f64 = (1.0 + v50);
        self.scalar_v51 = v51;
        let v52: f64 = (v48 * v51);
        self.scalar_v52 = v52;
        let v53: f64 = (if v15 { v52 } else { 0.0 });
        self.scalar_v53 = v53;
        let v54: f64 = (p.p20 + v43);
        self.scalar_v54 = v54;
        let v55: f64 = (v24 + v53);
        self.scalar_v55 = v55;
        let v56: bool = (v54 > 1e-9);
        self.scalar_v56 = v56;
        let v57: f64 = (if v56 { v54 } else { 1e-9 });
        self.scalar_v57 = v57;
        let v58: bool = (v55 > 1e-9);
        self.scalar_v58 = v58;
        let v59: f64 = (if v58 { v55 } else { 1e-9 });
        self.scalar_v59 = v59;
        let v60: f64 = (if v15 { v57 } else { 0.0 });
        self.scalar_v60 = v60;
        let v61: f64 = p.p489;
        self.scalar_v61 = v61;
        let v62: f64 = (v60 + p.p489);
        self.scalar_v62 = v62;
        let v63: bool = (v62 > 1e-9);
        self.scalar_v63 = v63;
        let v64: f64 = (if v63 { v62 } else { 1e-9 });
        self.scalar_v64 = v64;
        let v65: f64 = (if v15 { v64 } else { 0.0 });
        self.scalar_v65 = v65;
        let v66: f64 = (if v15 { v59 } else { 0.0 });
        self.scalar_v66 = v66;
        let v67: f64 = p.p38;
        self.scalar_v67 = v67;
        let v68: f64 = (0.5 * v53);
        self.scalar_v68 = v68;
        let v69: f64 = (p.p38 - v68);
        self.scalar_v69 = v69;
        let v70: bool = (v69 > 1e-9);
        self.scalar_v70 = v70;
        let v71: f64 = (if v70 { v69 } else { 1e-9 });
        self.scalar_v71 = v71;
        let v72: f64 = (if v15 { v71 } else { 0.0 });
        self.scalar_v72 = v72;
        let v74: f64 = p.p488;
        self.scalar_v74 = v74;
        let v75: f64 = (v66 * 0.3333333333333);
        self.scalar_v75 = v75;
        let v76: f64 = p.p37;
        self.scalar_v76 = v76;
        let v77: f64 = (v75 / p.p37);
        self.scalar_v77 = v77;
        let v78: f64 = (v72 + v77);
        self.scalar_v78 = v78;
        let v79: f64 = (p.p488 * v78);
        self.scalar_v79 = v79;
        let v80: f64 = (v65 * p.p37);
        self.scalar_v80 = v80;
        let v81: f64 = (v79 / v80);
        self.scalar_v81 = v81;
        let v82: f64 = p.p486;
        self.scalar_v82 = v82;
        let v83: f64 = p.p487;
        self.scalar_v83 = v83;
        let v84: f64 = (p.p486 + p.p487);
        self.scalar_v84 = v84;
        let v85: f64 = (v60 * v66);
        self.scalar_v85 = v85;
        let v86: f64 = (v84 / v85);
        self.scalar_v86 = v86;
        let v87: f64 = (v81 + v86);
        self.scalar_v87 = v87;
        let v88: f64 = p.p485;
        self.scalar_v88 = v88;
        let v89: f64 = (p.p29 * p.p485);
        self.scalar_v89 = v89;
        let v90: f64 = (v87 + v89);
        self.scalar_v90 = v90;
        let v91: f64 = (if v15 { v90 } else { 0.0 });
        self.scalar_v91 = v91;
        let v92: bool = (v91 > 0.0);
        self.scalar_v92 = v92;
        let v93: f64 = (if v92 { v91 } else { 0.0 });
        self.scalar_v93 = v93;
        let v94: f64 = (if v15 { v93 } else { v8 });
        self.scalar_v94 = v94;
        let v95: f64 = p.p490;
        self.scalar_v95 = v95;
        let v96: bool = (p.p490 > 0.0);
        self.scalar_v96 = v96;
        let v97: f64 = (if v96 { p.p490 } else { 0.0 });
        self.scalar_v97 = v97;
        let v98: f64 = (if v15 { v97 } else { 0.0 });
        self.scalar_v98 = v98;
        let v99: f64 = p.p491;
        self.scalar_v99 = v99;
        let v100: bool = (p.p491 > 0.0);
        self.scalar_v100 = v100;
        let v101: f64 = (if v100 { p.p491 } else { 0.0 });
        self.scalar_v101 = v101;
        let v102: f64 = (if v15 { v101 } else { 0.0 });
        self.scalar_v102 = v102;
        let v103: f64 = p.p7;
        self.scalar_v103 = v103;
        let v104: bool = (0.0 == p.p7);
        self.scalar_v104 = v104;
        let v105: bool = (v15 && v104);
        self.scalar_v105 = v105;
        let v106: f64 = (if v105 { v98 } else { v102 });
        self.scalar_v106 = v106;
        let v107: f64 = p.p39;
        self.scalar_v107 = v107;
        let v108: f64 = (p.p29 * p.p39);
        self.scalar_v108 = v108;
        let v109: f64 = (v98 * v108);
        self.scalar_v109 = v109;
        let v110: f64 = (if v15 { v109 } else { v10 });
        self.scalar_v110 = v110;
        let v111: f64 = p.p40;
        self.scalar_v111 = v111;
        let v112: f64 = (p.p29 * p.p40);
        self.scalar_v112 = v112;
        let v113: f64 = (v106 * v112);
        self.scalar_v113 = v113;
        let v114: f64 = (if v15 { v113 } else { v12 });
        self.scalar_v114 = v114;
        let v115: f64 = p.p492;
        self.scalar_v115 = v115;
        let v116: f64 = (p.p29 * p.p492);
        self.scalar_v116 = v116;
        let v117: f64 = (if v15 { v116 } else { v14 });
        self.scalar_v117 = v117;
        let v118: bool = (v94 > 0.0);
        self.scalar_v118 = v118;
        let v119: f64 = (1.0 / v94);
        self.scalar_v119 = v119;
        let v120: f64 = (if v118 { v119 } else { 0.0 });
        self.scalar_v120 = v120;
        let v121: bool = (!v118);
        self.scalar_v121 = v121;
        let v122: f64 = (if v121 { 0.0 } else { v120 });
        self.scalar_v122 = v122;
        let v123: bool = (v110 > 0.0);
        self.scalar_v123 = v123;
        let v124: f64 = (1.0 / v110);
        self.scalar_v124 = v124;
        let v125: f64 = (if v123 { v124 } else { 0.0 });
        self.scalar_v125 = v125;
        let v126: bool = (!v123);
        self.scalar_v126 = v126;
        let v127: f64 = (if v126 { 0.0 } else { v125 });
        self.scalar_v127 = v127;
        let v128: bool = (v114 > 0.0);
        self.scalar_v128 = v128;
        let v129: f64 = (1.0 / v114);
        self.scalar_v129 = v129;
        let v130: f64 = (if v128 { v129 } else { 0.0 });
        self.scalar_v130 = v130;
        let v131: bool = (!v128);
        self.scalar_v131 = v131;
        let v132: f64 = (if v131 { 0.0 } else { v130 });
        self.scalar_v132 = v132;
        let v133: bool = (v117 > 0.0);
        self.scalar_v133 = v133;
        let v134: f64 = (1.0 / v117);
        self.scalar_v134 = v134;
        let v135: f64 = (if v133 { v134 } else { 0.0 });
        self.scalar_v135 = v135;
        let v136: bool = (!v133);
        self.scalar_v136 = v136;
        let v137: f64 = (if v136 { 0.0 } else { v135 });
        self.scalar_v137 = v137;
        let v143: f64 = p.p31;
        self.scalar_v143 = v143;
        let v144: f64 = (v26 * p.p31);
        self.scalar_v144 = v144;
        let v145: f64 = (0.0 * p.p31);
        self.scalar_v145 = v145;
        let v147: f64 = (v122 * v144);
        self.scalar_v147 = v147;
        let v152: f64 = (if v121 { 0.0 } else { 0.0 });
        self.scalar_v152 = v152;
        let v153: f64 = (v127 * v144);
        self.scalar_v153 = v153;
        let v158: f64 = (if v126 { 0.0 } else { 0.0 });
        self.scalar_v158 = v158;
        let v159: f64 = (v132 * v144);
        self.scalar_v159 = v159;
        let v164: f64 = (if v131 { 0.0 } else { 0.0 });
        self.scalar_v164 = v164;
        let v165: f64 = (v137 * v144);
        self.scalar_v165 = v165;
        let v170: f64 = (if v136 { 0.0 } else { 0.0 });
        self.scalar_v170 = v170;
        let v171: f64 = (-v145);
        self.scalar_v171 = v171;
        let v172: f64 = (-v147);
        self.scalar_v172 = v172;
        let v173: f64 = (if v118 { v147 } else { 0.0 });
        self.scalar_v173 = v173;
        let v174: f64 = (if v118 { v172 } else { 0.0 });
        self.scalar_v174 = v174;
        let v175: f64 = (-v153);
        self.scalar_v175 = v175;
        let v176: f64 = (if v123 { v153 } else { 0.0 });
        self.scalar_v176 = v176;
        let v177: f64 = (if v123 { v175 } else { 0.0 });
        self.scalar_v177 = v177;
        let v178: f64 = (-v159);
        self.scalar_v178 = v178;
        let v179: f64 = (if v128 { v159 } else { 0.0 });
        self.scalar_v179 = v179;
        let v180: f64 = (if v128 { v178 } else { 0.0 });
        self.scalar_v180 = v180;
        let v181: f64 = (-v165);
        self.scalar_v181 = v181;
        let v182: f64 = (if v133 { v165 } else { 0.0 });
        self.scalar_v182 = v182;
        let v183: f64 = (if v133 { v181 } else { 0.0 });
        self.scalar_v183 = v183;
    }
}
