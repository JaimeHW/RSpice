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
    pub p493: f64,
    pub p494: f64,
    pub p495: f64,
    pub p496: f64,
    pub p497: f64,
    pub p498: f64,
    pub p499: f64,
    pub p500: f64,
    pub p501: f64,
    pub p502: f64,
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
            params.p183 = 1.0;
            params.p184 = 1.0;
            params.p185 = 1.0;
            params.p186 = 1e-15;
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
            params.p197 = 0.0;
            params.p198 = 0.0;
            params.p199 = 0.0;
            params.p200 = 0.0;
            params.p201 = 2e-9;
            params.p202 = 1e-8;
            params.p203 = 0.0;
            params.p204 = 1e-7;
            params.p205 = 0.0;
            params.p206 = 3e18;
            params.p207 = 0.0;
            params.p208 = 2e-9;
            params.p209 = 1e20;
            params.p210 = 1e20;
            params.p211 = 0.0;
            params.p212 = 0.0;
            params.p213 = 2.0;
            params.p214 = 0.0;
            params.p215 = 2.0;
            params.p216 = 0.0;
            params.p217 = 0.0;
            params.p218 = 0.0;
            params.p219 = 0.0;
            params.p220 = 0.0;
            params.p221 = 0.0;
            params.p222 = 0.0;
            params.p223 = 0.0;
            params.p224 = 1e21;
            params.p225 = 0.0;
            params.p226 = 1.0;
            params.p227 = 1.0;
            params.p228 = 0.0;
            params.p229 = 2.0;
            params.p230 = 0.0;
            params.p231 = 1.0;
            params.p232 = 1e22;
            params.p233 = 0.0;
            params.p234 = 0.0;
            params.p235 = 0.0;
            params.p236 = 2.0;
            params.p237 = 0.0;
            params.p238 = 1.0;
            params.p239 = 0.0;
            params.p240 = 0.2;
            params.p241 = 0.0;
            params.p242 = 0.0;
            params.p243 = 0.0;
            params.p244 = 0.05;
            params.p245 = 0.0;
            params.p246 = 0.0;
            params.p247 = 1e-8;
            params.p248 = 0.0;
            params.p249 = 0.0;
            params.p250 = 1e-8;
            params.p251 = 0.0;
            params.p252 = 0.0;
            params.p253 = 1e-8;
            params.p254 = 1.0;
            params.p255 = 1.5;
            params.p256 = 0.0;
            params.p257 = 0.0;
            params.p258 = 0.0;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 1.0;
            params.p262 = 0.0;
            params.p263 = 0.0;
            params.p264 = 0.0;
            params.p265 = 0.0;
            params.p266 = 0.0;
            params.p267 = 0.0;
            params.p268 = 0.0;
            params.p269 = 0.0;
            params.p270 = 1.5;
            params.p271 = 0.0;
            params.p272 = 2.0;
            params.p273 = 1.0;
            params.p274 = 0.0;
            params.p275 = 0.0;
            params.p276 = 1.5;
            params.p277 = 0.0;
            params.p278 = 0.0;
            params.p279 = 0.0;
            params.p280 = 1.0;
            params.p281 = 0.0;
            params.p282 = 0.0;
            params.p283 = 1.0;
            params.p284 = 0.0;
            params.p285 = 1.0;
            params.p286 = 30.0;
            params.p287 = 0.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = 0.0;
            params.p291 = 2.0;
            params.p292 = 0.0;
            params.p293 = 0.0;
            params.p294 = 0.0;
            params.p295 = 1.0;
            params.p296 = 0.0;
            params.p297 = 0.0;
            params.p298 = -0.1;
            params.p299 = 0.0;
            params.p300 = 0.0;
            params.p301 = 0.0;
            params.p302 = 0.0;
            params.p303 = 0.0;
            params.p304 = 8.0;
            params.p305 = 0.0;
            params.p306 = 1.0;
            params.p307 = 0.0;
            params.p308 = 1.5;
            params.p309 = 0.0;
            params.p310 = 1.0;
            params.p311 = 0.0;
            params.p312 = 2.0;
            params.p313 = 0.0;
            params.p314 = 0.0;
            params.p315 = 0.5;
            params.p316 = 0.0;
            params.p317 = 1.5;
            params.p318 = 0.0;
            params.p319 = 0.0;
            params.p320 = 0.05;
            params.p321 = 0.0;
            params.p322 = 0.0;
            params.p323 = 0.0;
            params.p324 = 0.0;
            params.p325 = 0.0;
            params.p326 = 0.0;
            params.p327 = 0.0;
            params.p328 = 0.0;
            params.p329 = 0.375;
            params.p330 = 0.063;
            params.p331 = 0.375;
            params.p332 = 0.063;
            params.p333 = 0.375;
            params.p334 = 0.063;
            params.p335 = 0.0;
            params.p336 = 1.0;
            params.p337 = 3.1;
            params.p338 = 0.0;
            params.p339 = 0.0;
            params.p340 = 0.0;
            params.p341 = 0.2;
            params.p342 = 0.0;
            params.p343 = 0.0;
            params.p344 = 0.0;
            params.p345 = 0.0;
            params.p346 = 0.0;
            params.p347 = 41.0;
            params.p348 = 41.0;
            params.p349 = 0.0;
            params.p350 = 0.0;
            params.p351 = 0.0;
            params.p352 = 0.0;
            params.p353 = 0.0;
            params.p354 = 0.0;
            params.p355 = 0.0;
            params.p356 = 0.0;
            params.p357 = 1e-8;
            params.p358 = 0.0;
            params.p359 = 0.0;
            params.p360 = 0.0;
            params.p361 = 0.0;
            params.p362 = 2.0;
            params.p363 = 0.0;
            params.p364 = 0.0;
            params.p365 = 0.0;
            params.p366 = 0.0;
            params.p367 = 0.0;
            params.p368 = 0.0;
            params.p369 = 0.0;
            params.p370 = 1.0;
            params.p371 = 1.0;
            params.p372 = 0.0;
            params.p373 = 2.0;
            params.p374 = 0.0;
            params.p375 = 1.0;
            params.p376 = 0.0;
            params.p377 = 2.0;
            params.p378 = 0.0;
            params.p379 = 1.0;
            params.p380 = 0.2;
            params.p381 = 0.0;
            params.p382 = 1e-8;
            params.p383 = 0.0;
            params.p384 = 1.0;
            params.p385 = 0.0;
            params.p386 = 0.0;
            params.p387 = 0.0;
            params.p388 = 1.0;
            params.p389 = 0.0;
            params.p390 = 0.0;
            params.p391 = 10.0;
            params.p392 = 0.0;
            params.p393 = 1.0;
            params.p394 = 0.0;
            params.p395 = 0.0;
            params.p396 = 0.0;
            params.p397 = 0.0;
            params.p398 = 1e22;
            params.p399 = 0.0;
            params.p400 = 0.0;
            params.p401 = 0.0;
            params.p402 = 0.0;
            params.p403 = 2.0;
            params.p404 = 0.0;
            params.p405 = 2.0;
            params.p406 = 0.0;
            params.p407 = 0.0;
            params.p408 = 0.0;
            params.p409 = 0.0;
            params.p410 = 0.0;
            params.p411 = 2.0;
            params.p412 = 0.0;
            params.p413 = 0.0;
            params.p414 = 2.0;
            params.p415 = 0.0;
            params.p416 = 0.0;
            params.p417 = 0.0;
            params.p418 = 1.0;
            params.p419 = 0.0;
            params.p420 = 0.0;
            params.p421 = 8.0;
            params.p422 = 0.0;
            params.p423 = 1.0;
            params.p424 = 0.0;
            params.p425 = 1.5;
            params.p426 = 0.0;
            params.p427 = 1.0;
            params.p428 = 0.0;
            params.p429 = 2.0;
            params.p430 = 0.0;
            params.p431 = 0.0;
            params.p432 = 0.0;
            params.p433 = 0.0;
            params.p434 = 0.0;
            params.p435 = 0.0;
            params.p436 = 0.0;
            params.p437 = 0.0;
            params.p438 = 0.0;
            params.p439 = 0.0;
            params.p440 = 0.0;
            params.p441 = 1.0;
            params.p442 = 0.0;
            params.p443 = 100000.0;
            params.p444 = 1.5;
            params.p445 = 3.0;
            params.p446 = 4.5;
            params.p447 = 0.0;
            params.p448 = 1e-12;
            params.p449 = 1e-7;
            params.p450 = 0.0;
            params.p451 = 1.0;
            params.p452 = 0.0;
            params.p453 = 2.0;
            params.p454 = 8e22;
            params.p455 = 0.0;
            params.p456 = 30000000.0;
            params.p457 = 0.0;
            params.p458 = 0.0;
            params.p459 = 0.0;
            params.p460 = 1.0;
            params.p461 = 1.0;
            params.p462 = 1e-6;
            params.p463 = 1e-6;
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
            params.p476 = 0.0;
            params.p477 = 0.0;
            params.p478 = 0.0;
            params.p479 = 0.0;
            params.p480 = 1.0;
            params.p481 = 1e-7;
            params.p482 = 3.0;
            params.p483 = 0.0;
            params.p484 = 0.0;
            params.p485 = 0.0;
            params.p486 = 0.0;
            params.p487 = 0.0;
            params.p488 = 0.0;
            params.p489 = 1.0;
            params.p490 = 0.0;
            params.p491 = 1.0;
            params.p492 = 0.0;
            params.p493 = 1.0;
            params.p494 = 1e-15;
            params.p495 = 0.0;
            params.p496 = 0.0;
            params.p497 = 0.0;
            params.p498 = 0.0;
            params.p499 = 0.0;
            params.p500 = 0.0;
            params.p501 = 0.0;
            params.p502 = 0.0;
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
    pub nodes: [usize; 14],
    pub branches: [usize; 4],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 503]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 24]>,
    pub(crate) ddt_state_previous: Box<[f64; 24]>,
    pub(crate) ddt_state_older: Box<[f64; 24]>,
    pub(crate) ddt_state_initialized: Box<[bool; 24]>,
    pub(crate) ddt_derivative_current: Box<[f64; 24]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 24]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 489]>,
    pub(crate) scalar_static_bool: Box<[bool; 74]>,
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
    pub const INTERNAL_NODE_COUNT: usize = 9;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["NSIG", "si", "di", "bp", "gp", "Gnqs", "Gnqs2", "Dnqs", "gndnqs"];

    pub const BRANCH_COUNT: usize = 4;
    pub const PARAMETER_COUNT: usize = 503;
    pub const VARIABLE_COUNT: usize = 1911;
    pub const DDT_STATE_COUNT: usize = 24;
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
            scalar_static_f64: boxed_zero_f64_array::<489>(),
            scalar_static_bool: boxed_zero_bool_array::<74>(),
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
            "swscale" => { validate_parameter("SWSCALE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swsubdep" => { validate_parameter("SWSUBDEP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swgidl" => { validate_parameter("SWGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swshe" => { validate_parameter("SWSHE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swign" => { validate_parameter("SWIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swjunasym" => { validate_parameter("SWJUNASYM", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swimpact" => { validate_parameter("SWIMPACT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swpdep" => { validate_parameter("SWPDEP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swcryo" => { validate_parameter("SWCRYO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swqmod" => { validate_parameter("SWQMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swedge" => { validate_parameter("SWEDGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmax" => { validate_parameter("TMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tmin" => { validate_parameter("TMIN", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "atmin" => { validate_parameter("ATMIN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "btmin" => { validate_parameter("BTMIN", value, Some((0.0, "0.0")), false, Some((1000.0, "1000.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "asource" => { validate_parameter("ASOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "adrain" => { validate_parameter("ADRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psource" => { validate_parameter("PSOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pdrain" => { validate_parameter("PDRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult" => { validate_parameter("MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mult_fn" => { validate_parameter("MULT_FN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "delvto" => { validate_finite_parameter("DELVTO", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "factuo" => { validate_parameter("FACTUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrs" => { validate_finite_parameter("NRS", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrd" => { validate_finite_parameter("NRD", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxe" => { validate_parameter("TOXE", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tsi" => { validate_parameter("TSI", value, Some((3e-9, "3e-9")), false, Some((2e-8, "2e-8")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xge" => { validate_parameter("XGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nch" => { validate_finite_parameter("NCH", value)?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsub" => { validate_finite_parameter("NSUB", value)?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ct" => { validate_parameter("CT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxp" => { validate_parameter("TOXP", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nov" => { validate_parameter("NOV", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novd" => { validate_parameter("NOVD", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfb" => { validate_finite_parameter("VFB", value)?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbb" => { validate_finite_parameter("VFBB", value)?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "np" => { validate_parameter("NP", value, Some((1e19, "1e19")), false, Some((1e22, "1e22")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicf" => { validate_parameter("CICF", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cic" => { validate_parameter("CIC", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psce" => { validate_parameter("PSCE", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceb" => { validate_parameter("PSCEB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsddc" => { validate_parameter("NSDDC", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscedlb" => { validate_parameter("PSCEDLB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnce" => { validate_parameter("PNCE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cf" => { validate_parameter("CF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfb" => { validate_parameter("CFB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcf" => { validate_finite_parameter("STCF", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfd" => { validate_parameter("CFD", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdl" => { validate_finite_parameter("CFDL", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdlb" => { validate_parameter("CFDLB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betn" => { validate_parameter("BETN", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betnb" => { validate_parameter("BETNB", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbet" => { validate_finite_parameter("STBET", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cs" => { validate_parameter("CS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csfi" => { validate_parameter("CSFI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csbi" => { validate_parameter("CSBI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcs" => { validate_finite_parameter("STCS", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thecs" => { validate_parameter("THECS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthecs" => { validate_finite_parameter("STTHECS", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csthr" => { validate_parameter("CSTHR", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csthrb" => { validate_parameter("CSTHRB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mue" => { validate_parameter("MUE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stmue" => { validate_finite_parameter("STMUE", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "themu" => { validate_parameter("THEMU", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthemu" => { validate_finite_parameter("STTHEMU", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcor" => { validate_finite_parameter("XCOR", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorb" => { validate_finite_parameter("XCORB", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stxcor" => { validate_finite_parameter("STXCOR", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsig" => { validate_parameter("RSIG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strs" => { validate_finite_parameter("STRS", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsg" => { validate_parameter("RSG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thersg" => { validate_finite_parameter("THERSG", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsb" => { validate_finite_parameter("RSB", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesat" => { validate_parameter("THESAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthesat" => { validate_finite_parameter("STTHESAT", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatg" => { validate_parameter("THESATG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatb" => { validate_parameter("THESATB", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ax" => { validate_parameter("AX", value, Some((1.0, "1.0")), false, Some((16.0, "16.0")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp" => { validate_parameter("ALP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1" => { validate_parameter("ALP1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpb" => { validate_finite_parameter("ALPB", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vp" => { validate_parameter("VP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpg" => { validate_parameter("VPG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gco" => { validate_parameter("GCO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iginv" => { validate_parameter("IGINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovinv" => { validate_parameter("IGOVINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovinvd" => { validate_parameter("IGOVINVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovacc" => { validate_parameter("IGOVACC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovaccd" => { validate_parameter("IGOVACCD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2ch" => { validate_parameter("GC2CH", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3ch" => { validate_parameter("GC3CH", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2ovinv" => { validate_parameter("GC2OVINV", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3ovinv" => { validate_parameter("GC3OVINV", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2ovacc" => { validate_parameter("GC2OVACC", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3ovacc" => { validate_parameter("GC3OVACC", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcdov" => { validate_finite_parameter("GCDOV", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcvdov" => { validate_finite_parameter("GCVDOV", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chib" => { validate_parameter("CHIB", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "niginv" => { validate_parameter("NIGINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnovinv" => { validate_parameter("FNOVINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnovinvd" => { validate_parameter("FNOVINVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcovinvfn" => { validate_parameter("GCOVINVFN", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stigfn" => { validate_finite_parameter("STIGFN", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidl" => { validate_parameter("AGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidld" => { validate_parameter("AGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgidl" => { validate_parameter("BGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgidld" => { validate_parameter("BGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbgidl" => { validate_finite_parameter("STBGIDL", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbgidld" => { validate_finite_parameter("STBGIDLD", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgidl" => { validate_finite_parameter("CGIDL", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgidld" => { validate_finite_parameter("CGIDLD", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidl" => { validate_finite_parameter("DGIDL", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidld" => { validate_finite_parameter("DGIDLD", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctedge" => { validate_parameter("CTEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedge" => { validate_finite_parameter("VFBEDGE", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbbedge" => { validate_finite_parameter("VFBBEDGE", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbedge" => { validate_finite_parameter("STVFBEDGE", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicfedge" => { validate_parameter("CICFEDGE", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicedge" => { validate_parameter("CICEDGE", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceedge" => { validate_parameter("PSCEEDGE", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscebedge" => { validate_parameter("PSCEBEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfedge" => { validate_parameter("CFEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfbedge" => { validate_parameter("CFBEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdedge" => { validate_parameter("CFDEDGE", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betnedge" => { validate_parameter("BETNEDGE", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetedge" => { validate_finite_parameter("STBETEDGE", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a1" => { validate_parameter("A1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a2" => { validate_parameter("A2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sta2" => { validate_finite_parameter("STA2", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a3" => { validate_parameter("A3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "areaq" => { validate_parameter("AREAQ", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgbov" => { validate_parameter("CGBOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsdac" => { validate_parameter("NSDAC", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fif" => { validate_parameter("FIF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fsceac" => { validate_parameter("FSCEAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbac" => { validate_finite_parameter("VFBAC", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbbac" => { validate_finite_parameter("VFBBAC", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceac" => { validate_parameter("PSCEAC", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfac" => { validate_parameter("CFAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatac" => { validate_parameter("THESATAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axac" => { validate_parameter("AXAC", value, Some((1.0, "1.0")), false, Some((16.0, "16.0")), true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpac" => { validate_parameter("ALPAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cov" => { validate_parameter("COV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covd" => { validate_parameter("COVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdl" => { validate_finite_parameter("COVDL", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdlb" => { validate_finite_parameter("COVDLB", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvfbov" => { validate_finite_parameter("DVFBOV", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfr" => { validate_parameter("CFR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrd" => { validate_parameter("CFRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csd" => { validate_parameter("CSD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csdbp" => { validate_parameter("CSDBP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth" => { validate_parameter("RTH", value, Some((1e-6, "1e-6")), false, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strth" => { validate_finite_parameter("STRTH", value)?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth" => { validate_parameter("CTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnt" => { validate_parameter("FNT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fntexc" => { validate_parameter("FNTEXC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfa" => { validate_parameter("NFA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfb" => { validate_parameter("NFB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfc" => { validate_parameter("NFC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfe" => { validate_parameter("NFE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfeb" => { validate_parameter("NFEB", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdrift" => { validate_parameter("KDRIFT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdiff" => { validate_parameter("KDIFF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fracinv" => { validate_parameter("FRACINV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfracinv" => { validate_parameter("KFRACINV", value, Some((1e-15, "1e-15")), false, Some((1.0, "1.0")), false, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rg" => { validate_parameter("RG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rse" => { validate_parameter("RSE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rde" => { validate_parameter("RDE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rwell" => { validate_parameter("RWELL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvaro" => { validate_finite_parameter("LVARO", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvarl" => { validate_finite_parameter("LVARL", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvarw" => { validate_finite_parameter("LVARW", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lap" => { validate_finite_parameter("LAP", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvaro" => { validate_finite_parameter("WVARO", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvarl" => { validate_finite_parameter("WVARL", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvarw" => { validate_finite_parameter("WVARW", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wot" => { validate_finite_parameter("WOT", value)?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxeo" => { validate_parameter("TOXEO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tsio" => { validate_parameter("TSIO", value, Some((3e-9, "3e-9")), false, Some((2e-8, "2e-8")), false, &[])?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgeo" => { validate_parameter("XGEO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tboxo" => { validate_parameter("TBOXO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ncho" => { validate_finite_parameter("NCHO", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubo" => { validate_finite_parameter("NSUBO", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cto" => { validate_parameter("CTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxpo" => { validate_parameter("TOXPO", value, Some((3e-10, "3e-10")), false, Some((1e-6, "1e-6")), false, &[])?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novdo" => { validate_parameter("NOVDO", value, Some((1000000000000000.0, "1000000000000000.0")), false, Some((1e21, "1e21")), false, &[])?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbl" => { validate_finite_parameter("VFBL", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfblexp" => { validate_finite_parameter("VFBLEXP", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbl2" => { validate_parameter("VFBL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfblexp2" => { validate_finite_parameter("VFBLEXP2", value)?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbw" => { validate_finite_parameter("VFBW", value)?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfblw" => { validate_finite_parameter("VFBLW", value)?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbbo" => { validate_finite_parameter("VFBBO", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfblbo" => { validate_parameter("VFBLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbo" => { validate_finite_parameter("STVFBO", value)?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbl" => { validate_finite_parameter("STVFBL", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbw" => { validate_finite_parameter("STVFBW", value)?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfblw" => { validate_finite_parameter("STVFBLW", value)?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npo" => { validate_parameter("NPO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npl" => { validate_finite_parameter("NPL", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicfo" => { validate_parameter("CICFO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cico" => { validate_parameter("CICO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscel" => { validate_finite_parameter("PSCEL", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscelexp" => { validate_finite_parameter("PSCELEXP", value)?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscew" => { validate_finite_parameter("PSCEW", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscebo" => { validate_parameter("PSCEBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsddco" => { validate_parameter("NSDDCO", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscedlbo" => { validate_parameter("PSCEDLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pncew" => { validate_finite_parameter("PNCEW", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfl" => { validate_finite_parameter("CFL", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cflexp" => { validate_finite_parameter("CFLEXP", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfw" => { validate_finite_parameter("CFW", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfbo" => { validate_parameter("CFBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcfl" => { validate_finite_parameter("STCFL", value)?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdo" => { validate_parameter("CFDO", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdll" => { validate_finite_parameter("CFDLL", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdlw" => { validate_finite_parameter("CFDLW", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdlbo" => { validate_parameter("CFDLBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "uo" => { validate_parameter("UO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbet1" => { validate_finite_parameter("FBET1", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbet1w" => { validate_finite_parameter("FBET1W", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lp1" => { validate_parameter("LP1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lp1w" => { validate_finite_parameter("LP1W", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbet2" => { validate_finite_parameter("FBET2", value)?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lp2" => { validate_parameter("LP2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betw1" => { validate_finite_parameter("BETW1", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betw2" => { validate_finite_parameter("BETW2", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbet" => { validate_parameter("WBET", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betnbo" => { validate_parameter("BETNBO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbeto" => { validate_finite_parameter("STBETO", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetl" => { validate_finite_parameter("STBETL", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetw" => { validate_finite_parameter("STBETW", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetlw" => { validate_finite_parameter("STBETLW", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cso" => { validate_finite_parameter("CSO", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csl" => { validate_finite_parameter("CSL", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cslexp" => { validate_finite_parameter("CSLEXP", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csw" => { validate_finite_parameter("CSW", value)?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cslw" => { validate_finite_parameter("CSLW", value)?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csfio" => { validate_parameter("CSFIO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csbio" => { validate_parameter("CSBIO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcso" => { validate_finite_parameter("STCSO", value)?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcsl" => { validate_finite_parameter("STCSL", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcsw" => { validate_finite_parameter("STCSW", value)?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stcslw" => { validate_finite_parameter("STCSLW", value)?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thecso" => { validate_parameter("THECSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthecso" => { validate_finite_parameter("STTHECSO", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csthro" => { validate_parameter("CSTHRO", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csthrbo" => { validate_parameter("CSTHRBO", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueo" => { validate_parameter("MUEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stmueo" => { validate_finite_parameter("STMUEO", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "themuo" => { validate_parameter("THEMUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthemuo" => { validate_finite_parameter("STTHEMUO", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcoro" => { validate_finite_parameter("XCORO", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorl" => { validate_finite_parameter("XCORL", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorlexp" => { validate_finite_parameter("XCORLEXP", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorw" => { validate_finite_parameter("XCORW", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorlw" => { validate_finite_parameter("XCORLW", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xcorbo" => { validate_finite_parameter("XCORBO", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stxcoro" => { validate_finite_parameter("STXCORO", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fetao" => { validate_parameter("FETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsw1" => { validate_finite_parameter("RSW1", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsw2" => { validate_finite_parameter("RSW2", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsigo" => { validate_parameter("RSIGO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strso" => { validate_finite_parameter("STRSO", value)?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsgo" => { validate_parameter("RSGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thersgo" => { validate_finite_parameter("THERSGO", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsbo" => { validate_finite_parameter("RSBO", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesato" => { validate_parameter("THESATO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatl" => { validate_finite_parameter("THESATL", value)?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatlexp" => { validate_finite_parameter("THESATLEXP", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatw" => { validate_finite_parameter("THESATW", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatlw" => { validate_finite_parameter("THESATLW", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthesato" => { validate_finite_parameter("STTHESATO", value)?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthesatl" => { validate_finite_parameter("STTHESATL", value)?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthesatw" => { validate_finite_parameter("STTHESATW", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stthesatlw" => { validate_finite_parameter("STTHESATLW", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatgo" => { validate_parameter("THESATGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatbo" => { validate_parameter("THESATBO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axo" => { validate_finite_parameter("AXO", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axl" => { validate_parameter("AXL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axlexp" => { validate_finite_parameter("AXLEXP", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axl2" => { validate_parameter("AXL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axlexp2" => { validate_finite_parameter("AXLEXP2", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpl1" => { validate_finite_parameter("ALPL1", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alplexp" => { validate_finite_parameter("ALPLEXP", value)?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpl2" => { validate_parameter("ALPL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alplexp2" => { validate_finite_parameter("ALPLEXP2", value)?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpw" => { validate_finite_parameter("ALPW", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1l1" => { validate_finite_parameter("ALP1L1", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1lexp" => { validate_finite_parameter("ALP1LEXP", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1l2" => { validate_parameter("ALP1L2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1lexp2" => { validate_finite_parameter("ALP1LEXP2", value)?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alp1w" => { validate_finite_parameter("ALP1W", value)?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpbo" => { validate_finite_parameter("ALPBO", value)?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpo" => { validate_parameter("VPO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vpgo" => { validate_parameter("VPGO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovinvw" => { validate_parameter("IGOVINVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovinvdw" => { validate_parameter("IGOVINVDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovaccw" => { validate_parameter("IGOVACCW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igovaccdw" => { validate_parameter("IGOVACCDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stigo" => { validate_finite_parameter("STIGO", value)?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2cho" => { validate_parameter("GC2CHO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3cho" => { validate_parameter("GC3CHO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2ovinvo" => { validate_parameter("GC2OVINVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3ovinvo" => { validate_parameter("GC3OVINVO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc2ovacco" => { validate_parameter("GC2OVACCO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gc3ovacco" => { validate_parameter("GC3OVACCO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcdovl" => { validate_finite_parameter("GCDOVL", value)?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcvdovo" => { validate_finite_parameter("GCVDOVO", value)?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "niginvo" => { validate_parameter("NIGINVO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnovinvw" => { validate_parameter("FNOVINVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnovinvdw" => { validate_parameter("FNOVINVDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gcovinvfno" => { validate_parameter("GCOVINVFNO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stigfno" => { validate_finite_parameter("STIGFNO", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidlo" => { validate_finite_parameter("AGIDLO", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidldo" => { validate_finite_parameter("AGIDLDO", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidlw" => { validate_finite_parameter("AGIDLW", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "agidldw" => { validate_finite_parameter("AGIDLDW", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgidlo" => { validate_parameter("BGIDLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgidldo" => { validate_parameter("BGIDLDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbgidlo" => { validate_finite_parameter("STBGIDLO", value)?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbgidldo" => { validate_finite_parameter("STBGIDLDO", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgidlo" => { validate_finite_parameter("CGIDLO", value)?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgidldo" => { validate_finite_parameter("CGIDLDO", value)?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidlo" => { validate_finite_parameter("DGIDLO", value)?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidldo" => { validate_finite_parameter("DGIDLDO", value)?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidll" => { validate_finite_parameter("DGIDLL", value)?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dgidldl" => { validate_finite_parameter("DGIDLDL", value)?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wedge" => { validate_parameter("WEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wedgew" => { validate_parameter("WEDGEW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctedgeo" => { validate_parameter("CTEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedgeo" => { validate_finite_parameter("VFBEDGEO", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedgel" => { validate_finite_parameter("VFBEDGEL", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedgelexp" => { validate_finite_parameter("VFBEDGELEXP", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedgew" => { validate_finite_parameter("VFBEDGEW", value)?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbedgelw" => { validate_finite_parameter("VFBEDGELW", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbbedgeo" => { validate_finite_parameter("VFBBEDGEO", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbedgeo" => { validate_finite_parameter("STVFBEDGEO", value)?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbedgel" => { validate_finite_parameter("STVFBEDGEL", value)?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbedgew" => { validate_finite_parameter("STVFBEDGEW", value)?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stvfbedgelw" => { validate_finite_parameter("STVFBEDGELW", value)?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicfedgeo" => { validate_parameter("CICFEDGEO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cicedgeo" => { validate_parameter("CICEDGEO", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceedgel" => { validate_finite_parameter("PSCEEDGEL", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceedgelexp" => { validate_finite_parameter("PSCEEDGELEXP", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceedgew" => { validate_finite_parameter("PSCEEDGEW", value)?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscebedgeo" => { validate_parameter("PSCEBEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfedgel" => { validate_finite_parameter("CFEDGEL", value)?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfedgelexp" => { validate_finite_parameter("CFEDGELEXP", value)?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfedgew" => { validate_finite_parameter("CFEDGEW", value)?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfbedgeo" => { validate_parameter("CFBEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfdedgeo" => { validate_parameter("CFDEDGEO", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fbetedge" => { validate_finite_parameter("FBETEDGE", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lpedge" => { validate_parameter("LPEDGE", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "betedgew" => { validate_finite_parameter("BETEDGEW", value)?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetedgeo" => { validate_finite_parameter("STBETEDGEO", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetedgel" => { validate_finite_parameter("STBETEDGEL", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetedgew" => { validate_finite_parameter("STBETEDGEW", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stbetedgelw" => { validate_finite_parameter("STBETEDGELW", value)?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a1o" => { validate_finite_parameter("A1O", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a1l" => { validate_finite_parameter("A1L", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a1w" => { validate_finite_parameter("A1W", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a2o" => { validate_parameter("A2O", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sta2o" => { validate_finite_parameter("STA2O", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a3o" => { validate_finite_parameter("A3O", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a3l" => { validate_finite_parameter("A3L", value)?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "a3w" => { validate_finite_parameter("A3W", value)?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgbovo" => { validate_finite_parameter("CGBOVO", value)?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgbovl" => { validate_finite_parameter("CGBOVL", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsdaco" => { validate_parameter("NSDACO", value, Some((1e18, "1e18")), false, Some((1e22, "1e22")), false, &[])?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fifw" => { validate_parameter("FIFW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fsceaco" => { validate_parameter("FSCEACO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbaco" => { validate_finite_parameter("VFBACO", value)?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbacl" => { validate_finite_parameter("VFBACL", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbaclexp" => { validate_finite_parameter("VFBACLEXP", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbacl2" => { validate_parameter("VFBACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbaclexp2" => { validate_finite_parameter("VFBACLEXP2", value)?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbacw" => { validate_finite_parameter("VFBACW", value)?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbaclw" => { validate_finite_parameter("VFBACLW", value)?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbbaco" => { validate_finite_parameter("VFBBACO", value)?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfblbaco" => { validate_parameter("VFBLBACO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceacl" => { validate_finite_parameter("PSCEACL", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceaclexp" => { validate_finite_parameter("PSCEACLEXP", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psceacw" => { validate_finite_parameter("PSCEACW", value)?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfacl" => { validate_finite_parameter("CFACL", value)?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfaclexp" => { validate_finite_parameter("CFACLEXP", value)?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfacw" => { validate_finite_parameter("CFACW", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesataco" => { validate_finite_parameter("THESATACO", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatacl" => { validate_finite_parameter("THESATACL", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesataclexp" => { validate_finite_parameter("THESATACLEXP", value)?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesatacw" => { validate_finite_parameter("THESATACW", value)?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "thesataclw" => { validate_finite_parameter("THESATACLW", value)?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axaco" => { validate_finite_parameter("AXACO", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axacl" => { validate_parameter("AXACL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axaclexp" => { validate_finite_parameter("AXACLEXP", value)?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axacl2" => { validate_parameter("AXACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "axaclexp2" => { validate_finite_parameter("AXACLEXP2", value)?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpacl1" => { validate_finite_parameter("ALPACL1", value)?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpaclexp" => { validate_finite_parameter("ALPACLEXP", value)?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpacl2" => { validate_parameter("ALPACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpaclexp2" => { validate_finite_parameter("ALPACLEXP2", value)?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "alpacw" => { validate_finite_parameter("ALPACW", value)?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lovo" => { validate_parameter("LOVO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lovdo" => { validate_parameter("LOVDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdlo" => { validate_finite_parameter("COVDLO", value)?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdlw" => { validate_finite_parameter("COVDLW", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdlbo" => { validate_finite_parameter("COVDLBO", value)?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dvfbovo" => { validate_finite_parameter("DVFBOVO", value)?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfro" => { validate_finite_parameter("CFRO", value)?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrdo" => { validate_finite_parameter("CFRDO", value)?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrw" => { validate_finite_parameter("CFRW", value)?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cfrdw" => { validate_finite_parameter("CFRDW", value)?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csdo" => { validate_parameter("CSDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "csdbpo" => { validate_parameter("CSDBPO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rtho" => { validate_parameter("RTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthl" => { validate_finite_parameter("RTHL", value)?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthw" => { validate_finite_parameter("RTHW", value)?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthlw" => { validate_finite_parameter("RTHLW", value)?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strtho" => { validate_finite_parameter("STRTHO", value)?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctho" => { validate_parameter("CTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lambtho" => { validate_parameter("LAMBTHO", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ftho" => { validate_parameter("FTHO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fnto" => { validate_parameter("FNTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fntexcl" => { validate_parameter("FNTEXCL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fntexclexp" => { validate_finite_parameter("FNTEXCLEXP", value)?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfalw" => { validate_finite_parameter("NFALW", value)?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfaw" => { validate_finite_parameter("NFAW", value)?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfblw" => { validate_parameter("NFBLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfclw" => { validate_parameter("NFCLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfeo" => { validate_parameter("NFEO", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfebo" => { validate_parameter("NFEBO", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "efo" => { validate_parameter("EFO", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "swstress" => { validate_parameter("SWSTRESS", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wlod" => { validate_finite_parameter("WLOD", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kuo" => { validate_finite_parameter("KUO", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kvsat" => { validate_parameter("KVSAT", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tkuo" => { validate_finite_parameter("TKUO", value)?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lkuo" => { validate_finite_parameter("LKUO", value)?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wkuo" => { validate_finite_parameter("WKUO", value)?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pkuo" => { validate_finite_parameter("PKUO", value)?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "llodkuo" => { validate_parameter("LLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wlodkuo" => { validate_parameter("WLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kvtho" => { validate_finite_parameter("KVTHO", value)?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lkvtho" => { validate_finite_parameter("LKVTHO", value)?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wkvtho" => { validate_finite_parameter("WKVTHO", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pkvtho" => { validate_finite_parameter("PKVTHO", value)?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "llodvth" => { validate_parameter("LLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wlodvth" => { validate_parameter("WLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stetao" => { validate_finite_parameter("STETAO", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lodetao" => { validate_parameter("LODETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strlambda" => { validate_parameter("STRLAMBDA", value, Some((1e-9, "1e-9")), false, Some((1e-5, "1e-5")), false, &[])?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "stralpha" => { validate_parameter("STRALPHA", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strdvfbo" => { validate_finite_parameter("STRDVFBO", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strwdvfbo" => { validate_finite_parameter("STRWDVFBO", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strdcfl" => { validate_finite_parameter("STRDCFL", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strruo" => { validate_finite_parameter("STRRUO", value)?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strtruo" => { validate_finite_parameter("STRTRUO", value)?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "strrvsat" => { validate_finite_parameter("STRRVSAT", value)?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdrifto" => { validate_parameter("KDRIFTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdriftl" => { validate_finite_parameter("KDRIFTL", value)?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdiffo" => { validate_parameter("KDIFFO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kdiffl" => { validate_finite_parameter("KDIFFL", value)?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fracinvo" => { validate_parameter("FRACINVO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p493 = value; self.mark_param_given(493); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kfracinvo" => { validate_parameter("KFRACINVO", value, Some((1e-15, "1e-15")), false, Some((1.0, "1.0")), false, &[])?; self.params.p494 = value; self.mark_param_given(494); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rgo" => { validate_finite_parameter("RGO", value)?; self.params.p495 = value; self.mark_param_given(495); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rint" => { validate_parameter("RINT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p496 = value; self.mark_param_given(496); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rvpoly" => { validate_parameter("RVPOLY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p497 = value; self.mark_param_given(497); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p498 = value; self.mark_param_given(498); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dlsil" => { validate_finite_parameter("DLSIL", value)?; self.params.p499 = value; self.mark_param_given(499); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p500 = value; self.mark_param_given(500); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshd" => { validate_parameter("RSHD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p501 = value; self.mark_param_given(501); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rwello" => { validate_parameter("RWELLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p502 = value; self.mark_param_given(502); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
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
        let param_given = self.param_given.as_ref();
        self.scalar_static_f64[0]=p.p15;
        self.scalar_static_f64[1]=(273.15+self.scalar_static_f64[0]);
        self.scalar_static_f64[2]=p.p36;
        self.scalar_static_f64[3]=p.p10;
        self.scalar_static_bool[0]=(self.scalar_static_f64[3]==1.0);
        self.scalar_static_f64[4]=(if self.scalar_static_bool[0]{1.0}else{0.0});
        self.scalar_static_f64[5]=p.p17;
        self.scalar_static_f64[6]=p.p18;
        self.scalar_static_f64[7]=p.p19;
        self.scalar_static_bool[1]=(!(self.scalar_static_f64[4]!=0.0));
        self.scalar_static_f64[8]=p.p0;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[8]);
        self.scalar_static_f64[9]=p.p172;
        self.scalar_static_bool[3]=(self.scalar_static_f64[9]>0.0);
        self.scalar_static_bool[4]=(self.scalar_static_bool[2]&&self.scalar_static_bool[3]);
        self.scalar_static_bool[5]=(self.scalar_static_f64[8]>0.0);
        self.scalar_static_f64[10]=p.p443;
        self.scalar_static_bool[6]=(self.scalar_static_f64[10]>0.0);
        self.scalar_static_bool[7]=(self.scalar_static_bool[5]&&self.scalar_static_bool[6]);
        self.scalar_static_bool[8]=(self.scalar_static_bool[4]||self.scalar_static_bool[7]);
        self.scalar_static_f64[11]=(if self.scalar_static_bool[8]{1.0}else{0.0});
        self.scalar_static_f64[12]=p.p5;
        self.scalar_static_f64[13]=(if (self.scalar_static_f64[11]!=0.0){self.scalar_static_f64[12]}else{0.0});
        self.scalar_static_bool[9]=(!(self.scalar_static_f64[11]!=0.0));
        self.scalar_static_f64[14]=(if self.scalar_static_bool[9]{0.0}else{self.scalar_static_f64[13]});
        self.scalar_static_f64[15]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[16]=p.p30;
        self.scalar_static_f64[17]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[16]}else{0.0});
        self.scalar_static_f64[18]=p.p41;
        self.scalar_static_f64[19]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[18]}else{0.0});
        self.scalar_static_f64[20]=p.p42;
        self.scalar_static_f64[21]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[20]}else{0.0});
        self.scalar_static_f64[22]=p.p43;
        self.scalar_static_f64[23]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[22]}else{0.0});
        self.scalar_static_f64[24]=p.p11;
        self.scalar_static_bool[10]=(self.scalar_static_f64[24]>0.0);
        self.scalar_static_f64[25]=(if self.scalar_static_bool[10]{1.0}else{0.0});
        self.scalar_static_f64[26]=p.p167;
        self.scalar_static_f64[27]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[26]}else{0.0});
        self.scalar_static_f64[28]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[9]}else{0.0});
        self.scalar_static_f64[29]=p.p173;
        self.scalar_static_f64[30]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[29]}else{0.0});
        self.scalar_static_f64[31]=p.p174;
        self.scalar_static_f64[32]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[31]}else{0.0});
        self.scalar_static_f64[33]=p.p187;
        self.scalar_static_f64[34]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[33]}else{0.0});
        self.scalar_static_f64[35]=p.p188;
        self.scalar_static_f64[36]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[35]}else{0.0});
        self.scalar_static_f64[37]=p.p189;
        self.scalar_static_f64[38]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[37]}else{0.0});
        self.scalar_static_f64[39]=p.p190;
        self.scalar_static_f64[40]=(if (self.scalar_static_f64[15]!=0.0){self.scalar_static_f64[39]}else{0.0});
        self.scalar_static_bool[11]=(!(self.scalar_static_f64[15]!=0.0));
        self.scalar_static_f64[41]=p.p29;
        self.scalar_static_f64[42]=(1.0/self.scalar_static_f64[41]);
        self.scalar_static_f64[43]=(if self.scalar_static_bool[11]{self.scalar_static_f64[42]}else{0.0});
        self.scalar_static_f64[44]=p.p21;
        self.scalar_static_f64[45]=(self.scalar_static_f64[43]*self.scalar_static_f64[44]);
        self.scalar_static_bool[12]=(self.scalar_static_f64[45]>1e-9);
        self.scalar_static_f64[46]=(if self.scalar_static_bool[12]{self.scalar_static_f64[45]}else{1e-9});
        self.scalar_static_f64[47]=(if self.scalar_static_bool[11]{self.scalar_static_f64[46]}else{0.0});
        self.scalar_static_f64[48]=(self.scalar_static_f64[16]*self.scalar_static_f64[41]);
        self.scalar_static_f64[49]=(if self.scalar_static_bool[11]{self.scalar_static_f64[48]}else{self.scalar_static_f64[17]});
        self.scalar_static_f64[50]=(if self.scalar_static_bool[11]{1e-6}else{0.0});
        self.scalar_static_f64[51]=p.p20;
        self.scalar_static_f64[52]=(self.scalar_static_f64[50]/self.scalar_static_f64[51]);
        self.scalar_static_f64[53]=(if self.scalar_static_bool[11]{self.scalar_static_f64[52]}else{0.0});
        self.scalar_static_f64[54]=(self.scalar_static_f64[50]/self.scalar_static_f64[47]);
        self.scalar_static_f64[55]=(if self.scalar_static_bool[11]{self.scalar_static_f64[54]}else{0.0});
        self.scalar_static_f64[56]=p.p191;
        self.scalar_static_f64[57]=p.p192;
        self.scalar_static_f64[58]=(self.scalar_static_f64[53]*self.scalar_static_f64[57]);
        self.scalar_static_f64[59]=(1.0+self.scalar_static_f64[58]);
        self.scalar_static_f64[60]=(self.scalar_static_f64[56]*self.scalar_static_f64[59]);
        self.scalar_static_f64[61]=p.p193;
        self.scalar_static_f64[62]=(self.scalar_static_f64[55]*self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(1.0+self.scalar_static_f64[62]);
        self.scalar_static_f64[64]=(self.scalar_static_f64[60]*self.scalar_static_f64[63]);
        self.scalar_static_f64[65]=(if self.scalar_static_bool[11]{self.scalar_static_f64[64]}else{0.0});
        self.scalar_static_f64[66]=p.p195;
        self.scalar_static_f64[67]=p.p197;
        self.scalar_static_f64[68]=(self.scalar_static_f64[55]*self.scalar_static_f64[67]);
        self.scalar_static_f64[69]=(1.0+self.scalar_static_f64[68]);
        self.scalar_static_f64[70]=(self.scalar_static_f64[66]*self.scalar_static_f64[69]);
        self.scalar_static_f64[71]=p.p196;
        self.scalar_static_f64[72]=(self.scalar_static_f64[53]*self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=(1.0+self.scalar_static_f64[72]);
        self.scalar_static_f64[74]=(self.scalar_static_f64[70]*self.scalar_static_f64[73]);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[11]{self.scalar_static_f64[74]}else{0.0});
        self.scalar_static_f64[76]=(self.scalar_static_f64[51]+self.scalar_static_f64[65]);
        self.scalar_static_f64[77]=p.p194;
        self.scalar_static_f64[78]=(2.0*self.scalar_static_f64[77]);
        self.scalar_static_f64[79]=(self.scalar_static_f64[76]-self.scalar_static_f64[78]);
        self.scalar_static_bool[13]=(self.scalar_static_f64[79]>1e-9);
        self.scalar_static_f64[80]=(if self.scalar_static_bool[13]{self.scalar_static_f64[79]}else{1e-9});
        self.scalar_static_f64[81]=(if self.scalar_static_bool[11]{self.scalar_static_f64[80]}else{0.0});
        self.scalar_static_f64[82]=(self.scalar_static_f64[47]+self.scalar_static_f64[75]);
        self.scalar_static_f64[83]=p.p198;
        self.scalar_static_f64[84]=(2.0*self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(self.scalar_static_f64[82]-self.scalar_static_f64[84]);
        self.scalar_static_bool[14]=(self.scalar_static_f64[85]>1e-9);
        self.scalar_static_f64[86]=(if self.scalar_static_bool[14]{self.scalar_static_f64[85]}else{1e-9});
        self.scalar_static_f64[87]=(if self.scalar_static_bool[11]{self.scalar_static_f64[86]}else{0.0});
        self.scalar_static_f64[88]=p.p200;
        self.scalar_static_f64[89]=(self.scalar_static_f64[85]+self.scalar_static_f64[88]);
        self.scalar_static_bool[15]=(self.scalar_static_f64[89]>1e-9);
        self.scalar_static_f64[90]=(if self.scalar_static_bool[15]{self.scalar_static_f64[89]}else{1e-9});
        self.scalar_static_f64[91]=(if self.scalar_static_bool[11]{self.scalar_static_f64[90]}else{0.0});
        self.scalar_static_f64[92]=(self.scalar_static_f64[50]/self.scalar_static_f64[81]);
        self.scalar_static_f64[93]=(if self.scalar_static_bool[11]{self.scalar_static_f64[92]}else{0.0});
        self.scalar_static_f64[94]=(self.scalar_static_f64[50]/self.scalar_static_f64[87]);
        self.scalar_static_f64[95]=(if self.scalar_static_bool[11]{self.scalar_static_f64[94]}else{0.0});
        self.scalar_static_bool[16]=(self.scalar_static_f64[76]>1e-9);
        self.scalar_static_f64[96]=(if self.scalar_static_bool[16]{self.scalar_static_f64[76]}else{1e-9});
        self.scalar_static_bool[17]=(self.scalar_static_f64[82]>1e-9);
        self.scalar_static_f64[97]=(if self.scalar_static_bool[17]{self.scalar_static_f64[82]}else{1e-9});
        self.scalar_static_f64[98]=(if self.scalar_static_bool[11]{self.scalar_static_f64[96]}else{0.0});
        self.scalar_static_f64[99]=p.p499;
        self.scalar_static_f64[100]=(self.scalar_static_f64[98]+self.scalar_static_f64[99]);
        self.scalar_static_bool[18]=(self.scalar_static_f64[100]>1e-9);
        self.scalar_static_f64[101]=(if self.scalar_static_bool[18]{self.scalar_static_f64[100]}else{1e-9});
        self.scalar_static_f64[102]=(if self.scalar_static_bool[11]{self.scalar_static_f64[101]}else{0.0});
        self.scalar_static_f64[103]=(if self.scalar_static_bool[11]{self.scalar_static_f64[97]}else{0.0});
        self.scalar_static_f64[104]=p.p38;
        self.scalar_static_f64[105]=(0.5*self.scalar_static_f64[75]);
        self.scalar_static_f64[106]=(self.scalar_static_f64[104]-self.scalar_static_f64[105]);
        self.scalar_static_bool[19]=(self.scalar_static_f64[106]>1e-9);
        self.scalar_static_f64[107]=(if self.scalar_static_bool[19]{self.scalar_static_f64[106]}else{1e-9});
        self.scalar_static_f64[108]=(if self.scalar_static_bool[11]{self.scalar_static_f64[107]}else{0.0});
        self.scalar_static_f64[109]=p.p201;
        self.scalar_static_f64[110]=(if self.scalar_static_bool[11]{self.scalar_static_f64[109]}else{self.scalar_static_f64[19]});
        self.scalar_static_f64[111]=p.p202;
        self.scalar_static_f64[112]=(if self.scalar_static_bool[11]{self.scalar_static_f64[111]}else{self.scalar_static_f64[21]});
        self.scalar_static_f64[113]=p.p203;
        self.scalar_static_f64[114]=(if self.scalar_static_bool[11]{self.scalar_static_f64[113]}else{self.scalar_static_f64[23]});
        self.scalar_static_f64[115]=p.p212;
        self.scalar_static_f64[116]=p.p213;
        self.scalar_static_f64[117]=f64::powf(self.scalar_static_f64[93],self.scalar_static_f64[116]);
        self.scalar_static_f64[118]=(self.scalar_static_f64[115]*self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=p.p214;
        self.scalar_static_f64[120]=p.p215;
        self.scalar_static_f64[121]=f64::powf(self.scalar_static_f64[93],self.scalar_static_f64[120]);
        self.scalar_static_f64[122]=(self.scalar_static_f64[119]*self.scalar_static_f64[121]);
        self.scalar_static_f64[123]=(1.0+self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=(self.scalar_static_f64[118]/self.scalar_static_f64[123]);
        self.scalar_static_f64[125]=(1.0-self.scalar_static_f64[114]);
        self.scalar_static_f64[126]=(if self.scalar_static_bool[11]{self.scalar_static_f64[125]}else{0.0});
        self.scalar_static_f64[127]=(self.scalar_static_f64[126]*1.04479e-10);
        self.scalar_static_f64[128]=(self.scalar_static_f64[114]*1.43438e-10);
        self.scalar_static_f64[129]=(self.scalar_static_f64[127]+self.scalar_static_f64[128]);
        self.scalar_static_f64[130]=(if self.scalar_static_bool[11]{self.scalar_static_f64[129]}else{0.0});
        self.scalar_static_f64[131]=(self.scalar_static_f64[130]/3.45313e-11);
        self.scalar_static_f64[132]=(self.scalar_static_f64[112]*self.scalar_static_f64[131]);
        self.scalar_static_f64[133]=(self.scalar_static_f64[110]+4e-10);
        self.scalar_static_f64[134]=(self.scalar_static_f64[132]*self.scalar_static_f64[133]);
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]).sqrt();
        self.scalar_static_f64[136]=(self.scalar_static_f64[135]/self.scalar_static_f64[81]);
        self.scalar_static_f64[137]=(if self.scalar_static_bool[11]{self.scalar_static_f64[136]}else{0.0});
        self.scalar_static_f64[138]=p.p236;
        self.scalar_static_f64[139]=f64::powf(self.scalar_static_f64[137],self.scalar_static_f64[138]);
        self.scalar_static_f64[140]=p.p237;
        self.scalar_static_f64[141]=(self.scalar_static_f64[95]*self.scalar_static_f64[140]);
        self.scalar_static_f64[142]=(1.0+self.scalar_static_f64[141]);
        self.scalar_static_f64[143]=(self.scalar_static_f64[139]*self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=(-self.scalar_static_f64[81]);
        self.scalar_static_f64[145]=p.p247;
        self.scalar_static_f64[146]=p.p248;
        self.scalar_static_f64[147]=(self.scalar_static_f64[95]*self.scalar_static_f64[146]);
        self.scalar_static_f64[148]=(1.0+self.scalar_static_f64[147]);
        self.scalar_static_bool[20]=(self.scalar_static_f64[148]>0.001);
        self.scalar_static_f64[149]=(if self.scalar_static_bool[20]{self.scalar_static_f64[148]}else{0.001});
        self.scalar_static_f64[150]=(self.scalar_static_f64[145]*self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=(self.scalar_static_f64[144]/self.scalar_static_f64[150]);
        self.scalar_static_f64[152]=(if self.scalar_static_bool[11]{self.scalar_static_f64[151]}else{0.0});
        self.scalar_static_bool[21]=(self.scalar_static_f64[152]> -80.0);
        self.scalar_static_f64[153]=(if self.scalar_static_bool[21]{1.0}else{0.0});
        self.scalar_static_bool[22]=(self.scalar_static_bool[11]&&(self.scalar_static_f64[153]!=0.0));
        self.scalar_static_f64[154]=(self.scalar_static_f64[152]).exp();
        self.scalar_static_f64[155]=(if self.scalar_static_bool[22]{self.scalar_static_f64[154]}else{0.0});
        self.scalar_static_bool[23]=(!(self.scalar_static_f64[153]!=0.0));
        self.scalar_static_bool[24]=(self.scalar_static_bool[11]&&self.scalar_static_bool[23]);
        self.scalar_static_f64[156]=(-self.scalar_static_f64[152]);
        self.scalar_static_f64[157]=(self.scalar_static_f64[156]-80.0);
        self.scalar_static_f64[158]=(0.5*self.scalar_static_f64[157]);
        self.scalar_static_f64[159]=(self.scalar_static_f64[157]*0.3333333333333);
        self.scalar_static_f64[160]=(1.0+self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=(self.scalar_static_f64[158]*self.scalar_static_f64[160]);
        self.scalar_static_f64[162]=(1.0+self.scalar_static_f64[161]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[157]*self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=(1.0+self.scalar_static_f64[163]);
        self.scalar_static_f64[165]=(1.80485e-35/self.scalar_static_f64[164]);
        self.scalar_static_f64[166]=(if self.scalar_static_bool[24]{self.scalar_static_f64[165]}else{self.scalar_static_f64[155]});
        self.scalar_static_f64[167]=p.p250;
        self.scalar_static_f64[168]=(self.scalar_static_f64[144]/self.scalar_static_f64[167]);
        self.scalar_static_f64[169]=(if self.scalar_static_bool[11]{self.scalar_static_f64[168]}else{0.0});
        self.scalar_static_bool[25]=(self.scalar_static_f64[169]> -80.0);
        self.scalar_static_f64[170]=(if self.scalar_static_bool[25]{1.0}else{0.0});
        self.scalar_static_bool[26]=(self.scalar_static_bool[11]&&(self.scalar_static_f64[170]!=0.0));
        self.scalar_static_f64[171]=(self.scalar_static_f64[169]).exp();
        self.scalar_static_f64[172]=(if self.scalar_static_bool[26]{self.scalar_static_f64[171]}else{0.0});
        self.scalar_static_bool[27]=(!(self.scalar_static_f64[170]!=0.0));
        self.scalar_static_bool[28]=(self.scalar_static_bool[11]&&self.scalar_static_bool[27]);
        self.scalar_static_f64[173]=(-self.scalar_static_f64[169]);
        self.scalar_static_f64[174]=(self.scalar_static_f64[173]-80.0);
        self.scalar_static_f64[175]=(0.5*self.scalar_static_f64[174]);
        self.scalar_static_f64[176]=(0.3333333333333*self.scalar_static_f64[174]);
        self.scalar_static_f64[177]=(1.0+self.scalar_static_f64[176]);
        self.scalar_static_f64[178]=(self.scalar_static_f64[175]*self.scalar_static_f64[177]);
        self.scalar_static_f64[179]=(1.0+self.scalar_static_f64[178]);
        self.scalar_static_f64[180]=(self.scalar_static_f64[174]*self.scalar_static_f64[179]);
        self.scalar_static_f64[181]=(1.0+self.scalar_static_f64[180]);
        self.scalar_static_f64[182]=(1.80485e-35/self.scalar_static_f64[181]);
        self.scalar_static_f64[183]=(if self.scalar_static_bool[28]{self.scalar_static_f64[182]}else{self.scalar_static_f64[172]});
        self.scalar_static_f64[184]=p.p361;
        self.scalar_static_f64[185]=p.p362;
        self.scalar_static_f64[186]=f64::powf(self.scalar_static_f64[93],self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=(self.scalar_static_f64[184]*self.scalar_static_f64[186]);
        self.scalar_static_f64[188]=p.p372;
        self.scalar_static_f64[189]=(2.0*self.scalar_static_f64[188]);
        self.scalar_static_f64[190]=p.p373;
        self.scalar_static_f64[191]=f64::powf(self.scalar_static_f64[137],self.scalar_static_f64[190]);
        self.scalar_static_f64[192]=(self.scalar_static_f64[189]*self.scalar_static_f64[191]);
        self.scalar_static_f64[193]=p.p374;
        self.scalar_static_f64[194]=(self.scalar_static_f64[95]*self.scalar_static_f64[193]);
        self.scalar_static_f64[195]=(1.0+self.scalar_static_f64[194]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[192]*self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=p.p377;
        self.scalar_static_f64[198]=f64::powf(self.scalar_static_f64[137],self.scalar_static_f64[197]);
        self.scalar_static_f64[199]=p.p378;
        self.scalar_static_f64[200]=(self.scalar_static_f64[95]*self.scalar_static_f64[199]);
        self.scalar_static_f64[201]=(1.0+self.scalar_static_f64[200]);
        self.scalar_static_f64[202]=(self.scalar_static_f64[198]*self.scalar_static_f64[201]);
        self.scalar_static_f64[203]=p.p376;
        self.scalar_static_f64[204]=p.p381;
        self.scalar_static_f64[205]=p.p382;
        self.scalar_static_f64[206]=(self.scalar_static_f64[204]*self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=(self.scalar_static_f64[206]/self.scalar_static_f64[81]);
        self.scalar_static_f64[208]=(self.scalar_static_f64[144]/self.scalar_static_f64[205]);
        self.scalar_static_f64[209]=(self.scalar_static_f64[208]).exp();
        self.scalar_static_f64[210]=(1.0-self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=(self.scalar_static_f64[207]*self.scalar_static_f64[210]);
        self.scalar_static_f64[212]=(1.0+self.scalar_static_f64[211]);
        self.scalar_static_bool[29]=((self.scalar_static_f64[25]!=0.0)&&self.scalar_static_bool[11]);
        self.scalar_static_f64[213]=(if self.scalar_static_bool[29]{self.scalar_static_f64[115]}else{0.0});
        self.scalar_static_f64[214]=if param_given[402] { 1.0 } else { 0.0 };
        self.scalar_static_bool[30]=(1.0==self.scalar_static_f64[214]);
        self.scalar_static_f64[215]=(if self.scalar_static_bool[30]{1.0}else{0.0});
        self.scalar_static_bool[31]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[215]!=0.0));
        self.scalar_static_f64[216]=p.p402;
        self.scalar_static_f64[217]=(if self.scalar_static_bool[31]{self.scalar_static_f64[216]}else{self.scalar_static_f64[213]});
        self.scalar_static_f64[218]=(if self.scalar_static_bool[29]{self.scalar_static_f64[116]}else{0.0});
        self.scalar_static_f64[219]=if param_given[403] { 1.0 } else { 0.0 };
        self.scalar_static_bool[32]=(1.0==self.scalar_static_f64[219]);
        self.scalar_static_f64[220]=(if self.scalar_static_bool[32]{1.0}else{0.0});
        self.scalar_static_bool[33]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[220]!=0.0));
        self.scalar_static_f64[221]=p.p403;
        self.scalar_static_f64[222]=(if self.scalar_static_bool[33]{self.scalar_static_f64[221]}else{self.scalar_static_f64[218]});
        self.scalar_static_f64[223]=(if self.scalar_static_bool[29]{self.scalar_static_f64[119]}else{0.0});
        self.scalar_static_f64[224]=if param_given[404] { 1.0 } else { 0.0 };
        self.scalar_static_bool[34]=(1.0==self.scalar_static_f64[224]);
        self.scalar_static_f64[225]=(if self.scalar_static_bool[34]{1.0}else{0.0});
        self.scalar_static_bool[35]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[225]!=0.0));
        self.scalar_static_f64[226]=p.p404;
        self.scalar_static_f64[227]=(if self.scalar_static_bool[35]{self.scalar_static_f64[226]}else{self.scalar_static_f64[223]});
        self.scalar_static_f64[228]=(if self.scalar_static_bool[29]{self.scalar_static_f64[120]}else{0.0});
        self.scalar_static_f64[229]=if param_given[405] { 1.0 } else { 0.0 };
        self.scalar_static_bool[36]=(1.0==self.scalar_static_f64[229]);
        self.scalar_static_f64[230]=(if self.scalar_static_bool[36]{1.0}else{0.0});
        self.scalar_static_bool[37]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[230]!=0.0));
        self.scalar_static_f64[231]=p.p405;
        self.scalar_static_f64[232]=(if self.scalar_static_bool[37]{self.scalar_static_f64[231]}else{self.scalar_static_f64[228]});
        self.scalar_static_f64[233]=f64::powf(self.scalar_static_f64[93],self.scalar_static_f64[222]);
        self.scalar_static_f64[234]=(self.scalar_static_f64[217]*self.scalar_static_f64[233]);
        self.scalar_static_f64[235]=f64::powf(self.scalar_static_f64[93],self.scalar_static_f64[232]);
        self.scalar_static_f64[236]=(self.scalar_static_f64[227]*self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=(1.0+self.scalar_static_f64[236]);
        self.scalar_static_f64[238]=(self.scalar_static_f64[234]/self.scalar_static_f64[237]);
        self.scalar_static_f64[239]=(if self.scalar_static_bool[29]{self.scalar_static_f64[138]}else{0.0});
        self.scalar_static_f64[240]=if param_given[414] { 1.0 } else { 0.0 };
        self.scalar_static_bool[38]=(1.0==self.scalar_static_f64[240]);
        self.scalar_static_f64[241]=(if self.scalar_static_bool[38]{1.0}else{0.0});
        self.scalar_static_bool[39]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[241]!=0.0));
        self.scalar_static_f64[242]=p.p414;
        self.scalar_static_f64[243]=(if self.scalar_static_bool[39]{self.scalar_static_f64[242]}else{self.scalar_static_f64[239]});
        self.scalar_static_f64[244]=(if self.scalar_static_bool[29]{self.scalar_static_f64[140]}else{0.0});
        self.scalar_static_f64[245]=if param_given[415] { 1.0 } else { 0.0 };
        self.scalar_static_bool[40]=(1.0==self.scalar_static_f64[245]);
        self.scalar_static_f64[246]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_bool[41]=(self.scalar_static_bool[29]&&(self.scalar_static_f64[246]!=0.0));
        self.scalar_static_f64[247]=p.p415;
        self.scalar_static_f64[248]=(if self.scalar_static_bool[41]{self.scalar_static_f64[247]}else{self.scalar_static_f64[244]});
        self.scalar_static_f64[249]=f64::powf(self.scalar_static_f64[137],self.scalar_static_f64[243]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[95]*self.scalar_static_f64[248]);
        self.scalar_static_f64[251]=(1.0+self.scalar_static_f64[250]);
        self.scalar_static_f64[252]=(self.scalar_static_f64[249]*self.scalar_static_f64[251]);
        self.scalar_static_f64[253]=(3.45313e-11/self.scalar_static_f64[110]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[91]*self.scalar_static_f64[253]);
        self.scalar_static_f64[255]=p.p436;
        self.scalar_static_f64[256]=(if self.scalar_static_bool[11]{self.scalar_static_f64[255]}else{self.scalar_static_f64[27]});
        self.scalar_static_f64[257]=p.p444;
        self.scalar_static_f64[258]=p.p445;
        self.scalar_static_f64[259]=p.p446;
        self.scalar_static_f64[260]=(if self.scalar_static_bool[11]{0.0}else{self.scalar_static_f64[152]});
        self.scalar_static_bool[42]=(self.scalar_static_f64[41]>1.0);
        self.scalar_static_f64[261]=p.p28;
        self.scalar_static_bool[43]=(self.scalar_static_f64[261]>0.0);
        self.scalar_static_bool[44]=(self.scalar_static_bool[42]&&self.scalar_static_bool[43]);
        self.scalar_static_f64[262]=(if self.scalar_static_bool[44]{1.0}else{0.0});
        self.scalar_static_bool[45]=(self.scalar_static_bool[11]&&(self.scalar_static_f64[262]!=0.0));
        self.scalar_static_f64[263]=(self.scalar_static_f64[51]+self.scalar_static_f64[261]);
        self.scalar_static_f64[264]=(-self.scalar_static_f64[263]);
        self.scalar_static_f64[265]=p.p449;
        self.scalar_static_f64[266]=(self.scalar_static_f64[264]/self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=(if self.scalar_static_bool[45]{self.scalar_static_f64[266]}else{self.scalar_static_f64[166]});
        self.scalar_static_f64[268]=(self.scalar_static_f64[267]).abs();
        self.scalar_static_bool[46]=(self.scalar_static_f64[268]<80.0);
        self.scalar_static_f64[269]=(if self.scalar_static_bool[46]{1.0}else{0.0});
        self.scalar_static_bool[47]=(self.scalar_static_bool[45]&&(self.scalar_static_f64[269]!=0.0));
        self.scalar_static_f64[270]=(self.scalar_static_f64[267]).exp();
        self.scalar_static_f64[271]=(if self.scalar_static_bool[47]{self.scalar_static_f64[270]}else{self.scalar_static_f64[169]});
        self.scalar_static_bool[48]=(self.scalar_static_f64[267]< -80.0);
        self.scalar_static_f64[272]=(if self.scalar_static_bool[48]{1.0}else{0.0});
        self.scalar_static_bool[49]=(!(self.scalar_static_f64[269]!=0.0));
        self.scalar_static_bool[50]=(self.scalar_static_bool[45]&&self.scalar_static_bool[49]);
        self.scalar_static_bool[51]=((self.scalar_static_f64[272]!=0.0)&&self.scalar_static_bool[50]);
        self.scalar_static_f64[273]=(-self.scalar_static_f64[267]);
        self.scalar_static_f64[274]=(self.scalar_static_f64[273]-80.0);
        self.scalar_static_f64[275]=(0.5*self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=(0.3333333333333*self.scalar_static_f64[274]);
        self.scalar_static_f64[277]=(1.0+self.scalar_static_f64[276]);
        self.scalar_static_f64[278]=(self.scalar_static_f64[275]*self.scalar_static_f64[277]);
        self.scalar_static_f64[279]=(1.0+self.scalar_static_f64[278]);
        self.scalar_static_f64[280]=(self.scalar_static_f64[274]*self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=(1.0+self.scalar_static_f64[280]);
        self.scalar_static_f64[282]=(1.80485e-35/self.scalar_static_f64[281]);
        self.scalar_static_f64[283]=(if self.scalar_static_bool[51]{self.scalar_static_f64[282]}else{self.scalar_static_f64[271]});
        self.scalar_static_bool[52]=(!(self.scalar_static_f64[272]!=0.0));
        self.scalar_static_bool[53]=(self.scalar_static_bool[50]&&self.scalar_static_bool[52]);
        self.scalar_static_f64[284]=(self.scalar_static_f64[267]-80.0);
        self.scalar_static_f64[285]=(0.5*self.scalar_static_f64[284]);
        self.scalar_static_f64[286]=(0.3333333333333*self.scalar_static_f64[284]);
        self.scalar_static_f64[287]=(1.0+self.scalar_static_f64[286]);
        self.scalar_static_f64[288]=(self.scalar_static_f64[285]*self.scalar_static_f64[287]);
        self.scalar_static_f64[289]=(1.0+self.scalar_static_f64[288]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[284]*self.scalar_static_f64[289]);
        self.scalar_static_f64[291]=(1.0+self.scalar_static_f64[290]);
        self.scalar_static_f64[292]=(5.54062e34*self.scalar_static_f64[291]);
        self.scalar_static_f64[293]=(if self.scalar_static_bool[53]{self.scalar_static_f64[292]}else{self.scalar_static_f64[283]});
        self.scalar_static_f64[294]=(1.0-self.scalar_static_f64[293]);
        self.scalar_static_f64[295]=(if self.scalar_static_bool[45]{self.scalar_static_f64[294]}else{self.scalar_static_f64[183]});
        self.scalar_static_f64[296]=p.p450;
        self.scalar_static_f64[297]=(2.0*self.scalar_static_f64[296]);
        self.scalar_static_f64[298]=(self.scalar_static_f64[293]*self.scalar_static_f64[297]);
        self.scalar_static_f64[299]=f64::powf(self.scalar_static_f64[293],self.scalar_static_f64[41]);
        self.scalar_static_f64[300]=(1.0-self.scalar_static_f64[299]);
        self.scalar_static_f64[301]=(self.scalar_static_f64[300]/self.scalar_static_f64[41]);
        self.scalar_static_f64[302]=(self.scalar_static_f64[295]-self.scalar_static_f64[301]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[298]*self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[295]*self.scalar_static_f64[295]);
        self.scalar_static_f64[305]=(self.scalar_static_f64[303]/self.scalar_static_f64[304]);
        self.scalar_static_f64[306]=(if self.scalar_static_bool[45]{self.scalar_static_f64[305]}else{self.scalar_static_f64[260]});
        self.scalar_static_f64[307]=(1.0+self.scalar_static_f64[306]);
        self.scalar_static_f64[308]=p.p447;
        self.scalar_static_f64[309]=(if self.scalar_static_bool[11]{self.scalar_static_f64[308]}else{self.scalar_static_f64[30]});
        self.scalar_static_f64[310]=p.p448;
        self.scalar_static_f64[311]=p.p498;
        self.scalar_static_f64[312]=(self.scalar_static_f64[103]*0.3333333333333);
        self.scalar_static_f64[313]=p.p37;
        self.scalar_static_f64[314]=(self.scalar_static_f64[312]/self.scalar_static_f64[313]);
        self.scalar_static_f64[315]=(self.scalar_static_f64[108]+self.scalar_static_f64[314]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[311]*self.scalar_static_f64[315]);
        self.scalar_static_f64[317]=(self.scalar_static_f64[102]*self.scalar_static_f64[313]);
        self.scalar_static_f64[318]=(self.scalar_static_f64[316]/self.scalar_static_f64[317]);
        self.scalar_static_f64[319]=p.p496;
        self.scalar_static_f64[320]=p.p497;
        self.scalar_static_f64[321]=(self.scalar_static_f64[319]+self.scalar_static_f64[320]);
        self.scalar_static_f64[322]=(self.scalar_static_f64[98]*self.scalar_static_f64[103]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[321]/self.scalar_static_f64[322]);
        self.scalar_static_f64[324]=(self.scalar_static_f64[318]+self.scalar_static_f64[323]);
        self.scalar_static_f64[325]=p.p495;
        self.scalar_static_f64[326]=(self.scalar_static_f64[41]*self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=(self.scalar_static_f64[324]+self.scalar_static_f64[326]);
        self.scalar_static_f64[328]=(if self.scalar_static_bool[11]{self.scalar_static_f64[327]}else{0.0});
        self.scalar_static_bool[54]=(self.scalar_static_f64[328]>0.0);
        self.scalar_static_f64[329]=(if self.scalar_static_bool[54]{self.scalar_static_f64[328]}else{0.0});
        self.scalar_static_f64[330]=(if self.scalar_static_bool[11]{self.scalar_static_f64[329]}else{self.scalar_static_f64[34]});
        self.scalar_static_f64[331]=p.p500;
        self.scalar_static_bool[55]=(self.scalar_static_f64[331]>0.0);
        self.scalar_static_f64[332]=(if self.scalar_static_bool[55]{self.scalar_static_f64[331]}else{0.0});
        self.scalar_static_f64[333]=(if self.scalar_static_bool[11]{self.scalar_static_f64[332]}else{0.0});
        self.scalar_static_f64[334]=p.p501;
        self.scalar_static_bool[56]=(self.scalar_static_f64[334]>0.0);
        self.scalar_static_f64[335]=(if self.scalar_static_bool[56]{self.scalar_static_f64[334]}else{0.0});
        self.scalar_static_f64[336]=(if self.scalar_static_bool[11]{self.scalar_static_f64[335]}else{0.0});
        self.scalar_static_f64[337]=p.p7;
        self.scalar_static_bool[57]=(0.0==self.scalar_static_f64[337]);
        self.scalar_static_f64[338]=(if self.scalar_static_bool[57]{1.0}else{0.0});
        self.scalar_static_bool[58]=(self.scalar_static_bool[11]&&(self.scalar_static_f64[338]!=0.0));
        self.scalar_static_f64[339]=(if self.scalar_static_bool[58]{self.scalar_static_f64[333]}else{self.scalar_static_f64[336]});
        self.scalar_static_f64[340]=p.p39;
        self.scalar_static_f64[341]=(self.scalar_static_f64[41]*self.scalar_static_f64[340]);
        self.scalar_static_f64[342]=(self.scalar_static_f64[333]*self.scalar_static_f64[341]);
        self.scalar_static_f64[343]=(if self.scalar_static_bool[11]{self.scalar_static_f64[342]}else{self.scalar_static_f64[36]});
        self.scalar_static_f64[344]=p.p40;
        self.scalar_static_f64[345]=(self.scalar_static_f64[41]*self.scalar_static_f64[344]);
        self.scalar_static_f64[346]=(self.scalar_static_f64[339]*self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=(if self.scalar_static_bool[11]{self.scalar_static_f64[346]}else{self.scalar_static_f64[38]});
        self.scalar_static_f64[348]=p.p502;
        self.scalar_static_f64[349]=(self.scalar_static_f64[41]*self.scalar_static_f64[348]);
        self.scalar_static_f64[350]=(if self.scalar_static_bool[11]{self.scalar_static_f64[349]}else{self.scalar_static_f64[40]});
        self.scalar_static_f64[351]=(self.scalar_static_f64[125]* -0.4);
        self.scalar_static_f64[352]=p.p14;
        self.scalar_static_bool[59]=(self.scalar_static_f64[330]>0.0);
        self.scalar_static_f64[353]=(if self.scalar_static_bool[59]{1.0}else{0.0});
        self.scalar_static_f64[354]=(1.0/self.scalar_static_f64[330]);
        self.scalar_static_f64[355]=(if (self.scalar_static_f64[353]!=0.0){self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_bool[60]=(!(self.scalar_static_f64[353]!=0.0));
        self.scalar_static_f64[356]=(if self.scalar_static_bool[60]{0.0}else{self.scalar_static_f64[355]});
        self.scalar_static_bool[61]=(self.scalar_static_f64[343]>0.0);
        self.scalar_static_f64[357]=(if self.scalar_static_bool[61]{1.0}else{0.0});
        self.scalar_static_f64[358]=(1.0/self.scalar_static_f64[343]);
        self.scalar_static_f64[359]=(if (self.scalar_static_f64[357]!=0.0){self.scalar_static_f64[358]}else{0.0});
        self.scalar_static_bool[62]=(!(self.scalar_static_f64[357]!=0.0));
        self.scalar_static_f64[360]=(if self.scalar_static_bool[62]{0.0}else{self.scalar_static_f64[359]});
        self.scalar_static_bool[63]=(self.scalar_static_f64[347]>0.0);
        self.scalar_static_f64[361]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_f64[362]=(1.0/self.scalar_static_f64[347]);
        self.scalar_static_f64[363]=(if (self.scalar_static_f64[361]!=0.0){self.scalar_static_f64[362]}else{0.0});
        self.scalar_static_bool[64]=(!(self.scalar_static_f64[361]!=0.0));
        self.scalar_static_f64[364]=(if self.scalar_static_bool[64]{0.0}else{self.scalar_static_f64[363]});
        self.scalar_static_bool[65]=(self.scalar_static_f64[350]>0.0);
        self.scalar_static_f64[365]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[366]=(1.0/self.scalar_static_f64[350]);
        self.scalar_static_f64[367]=(if (self.scalar_static_f64[365]!=0.0){self.scalar_static_f64[366]}else{0.0});
        self.scalar_static_bool[66]=(!(self.scalar_static_f64[365]!=0.0));
        self.scalar_static_f64[368]=(if self.scalar_static_bool[66]{0.0}else{self.scalar_static_f64[367]});
        self.scalar_static_bool[67]=(self.scalar_static_f64[14]>0.0);
        self.scalar_static_f64[369]=(if self.scalar_static_bool[67]{1.0}else{0.0});
        self.scalar_static_f64[370]=(self.scalar_static_f64[256]*self.scalar_static_f64[352]);
        self.scalar_static_bool[68]=(!(self.scalar_static_f64[369]!=0.0));
        self.scalar_static_f64[371]=p.p16;
        self.scalar_static_f64[372]=(0.25/self.scalar_static_f64[371]);
        self.scalar_static_f64[373]=(self.scalar_static_f64[371]+self.scalar_static_f64[372]);
        self.scalar_static_f64[374]=(-self.scalar_static_f64[373]);
        self.scalar_static_f64[375]=(self.scalar_static_f64[371]*100000000.0);
        self.scalar_static_f64[376]=p.p31;
        self.scalar_static_f64[377]=(self.scalar_static_f64[49]*self.scalar_static_f64[376]);
        self.scalar_static_f64[378]=(0.0*self.scalar_static_f64[377]);
        self.scalar_static_f64[379]=(self.scalar_static_f64[352]*self.scalar_static_f64[378]);
        self.scalar_static_f64[380]=(0.0*self.scalar_static_f64[376]);
        self.scalar_static_f64[381]=(self.scalar_static_f64[356]*self.scalar_static_f64[377]);
        self.scalar_static_f64[382]=(self.scalar_static_f64[360]*self.scalar_static_f64[377]);
        self.scalar_static_f64[383]=(self.scalar_static_f64[364]*self.scalar_static_f64[377]);
        self.scalar_static_f64[384]=(self.scalar_static_f64[368]*self.scalar_static_f64[377]);
        self.scalar_static_f64[385]=(if (self.scalar_static_f64[369]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[386]=(if (self.scalar_static_f64[369]!=0.0){self.scalar_static_f64[385]}else{0.0});
        self.scalar_static_f64[387]=(self.scalar_static_f64[1]*self.scalar_static_f64[386]);
        self.scalar_static_f64[388]=(-self.scalar_static_f64[387]);
        self.scalar_static_f64[389]=(8.617332384961e-5*self.scalar_static_f64[386]);
        self.scalar_static_f64[390]=(if (self.scalar_static_f64[369]!=0.0){self.scalar_static_f64[389]}else{0.0});
        self.scalar_static_f64[391]=(-self.scalar_static_f64[390]);
        self.scalar_static_f64[392]=(-self.scalar_static_f64[380]);
        self.scalar_static_f64[393]=(-self.scalar_static_f64[381]);
        self.scalar_static_f64[394]=(if (self.scalar_static_f64[353]!=0.0){self.scalar_static_f64[381]}else{0.0});
        self.scalar_static_f64[395]=(if (self.scalar_static_f64[353]!=0.0){self.scalar_static_f64[393]}else{0.0});
        self.scalar_static_f64[396]=(-self.scalar_static_f64[382]);
        self.scalar_static_f64[397]=(if (self.scalar_static_f64[357]!=0.0){self.scalar_static_f64[382]}else{0.0});
        self.scalar_static_f64[398]=(if (self.scalar_static_f64[357]!=0.0){self.scalar_static_f64[396]}else{0.0});
        self.scalar_static_f64[399]=(-self.scalar_static_f64[383]);
        self.scalar_static_f64[400]=(if (self.scalar_static_f64[361]!=0.0){self.scalar_static_f64[383]}else{0.0});
        self.scalar_static_f64[401]=(if (self.scalar_static_f64[361]!=0.0){self.scalar_static_f64[399]}else{0.0});
        self.scalar_static_f64[402]=(-self.scalar_static_f64[384]);
        self.scalar_static_f64[403]=(if (self.scalar_static_f64[365]!=0.0){self.scalar_static_f64[384]}else{0.0});
        self.scalar_static_f64[404]=(if (self.scalar_static_f64[365]!=0.0){self.scalar_static_f64[402]}else{0.0});
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
        self.scalar_static_f64[405]=(temperature+self.scalar_static_f64[2]);
        self.scalar_static_bool[69]=(self.scalar_static_f64[405]<1000.0);
        self.scalar_static_f64[406]=(if self.scalar_static_bool[69]{self.scalar_static_f64[405]}else{1000.0});
        self.scalar_static_f64[407]=(self.scalar_static_f64[406]*self.scalar_static_f64[6]);
        self.scalar_static_f64[408]=(self.scalar_static_f64[5]+self.scalar_static_f64[407]);
        self.scalar_static_f64[409]=(self.scalar_static_f64[406]+self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[406]-self.scalar_static_f64[408]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[410]*self.scalar_static_f64[410]);
        self.scalar_static_f64[412]=(self.scalar_static_f64[411]+self.scalar_static_f64[7]);
        self.scalar_static_f64[413]=(self.scalar_static_f64[412]).sqrt();
        self.scalar_static_f64[414]=(self.scalar_static_f64[409]+self.scalar_static_f64[413]);
        self.scalar_static_f64[415]=(0.5*self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=(if (self.scalar_static_f64[4]!=0.0){self.scalar_static_f64[415]}else{0.0});
        self.scalar_static_f64[417]=(self.scalar_static_f64[406]+1.0);
        self.scalar_static_f64[418]=(self.scalar_static_f64[406]-1.0);
        self.scalar_static_f64[419]=(self.scalar_static_f64[418]*self.scalar_static_f64[418]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[419]+0.001);
        self.scalar_static_f64[421]=(self.scalar_static_f64[420]).sqrt();
        self.scalar_static_f64[422]=(self.scalar_static_f64[417]+self.scalar_static_f64[421]);
        self.scalar_static_f64[423]=(0.5*self.scalar_static_f64[422]);
        self.scalar_static_f64[424]=(if self.scalar_static_bool[1]{self.scalar_static_f64[423]}else{self.scalar_static_f64[416]});
        self.scalar_static_f64[425]=(self.scalar_static_f64[424]*self.scalar_static_f64[424]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[1]/self.scalar_static_f64[424]);
        self.scalar_static_f64[427]=(8.617332384961e-5*self.scalar_static_f64[424]);
        self.scalar_static_f64[428]=(1.0/self.scalar_static_f64[427]);
        self.scalar_static_f64[429]=(if self.scalar_static_bool[11]{self.scalar_static_f64[96]}else{self.scalar_static_f64[406]});
        self.scalar_static_f64[430]=(self.scalar_static_f64[429]/self.scalar_static_f64[50]);
        self.scalar_static_f64[431]=(if self.scalar_static_bool[11]{self.scalar_static_f64[430]}else{0.0});
        self.scalar_static_f64[432]=(if self.scalar_static_bool[11]{self.scalar_static_f64[97]}else{self.scalar_static_f64[429]});
        self.scalar_static_f64[433]=(self.scalar_static_f64[432]/self.scalar_static_f64[50]);
        self.scalar_static_f64[434]=(if self.scalar_static_bool[11]{self.scalar_static_f64[433]}else{0.0});
        self.scalar_static_f64[435]=(if self.scalar_static_bool[11]{self.scalar_static_f64[124]}else{self.scalar_static_f64[432]});
        self.scalar_static_f64[436]=(if self.scalar_static_bool[11]{self.scalar_static_f64[143]}else{self.scalar_static_f64[435]});
        self.scalar_static_f64[437]=(if self.scalar_static_bool[11]{self.scalar_static_f64[187]}else{self.scalar_static_f64[436]});
        self.scalar_static_f64[438]=(if self.scalar_static_bool[11]{self.scalar_static_f64[196]}else{self.scalar_static_f64[437]});
        self.scalar_static_f64[439]=(if self.scalar_static_bool[11]{self.scalar_static_f64[202]}else{self.scalar_static_f64[438]});
        self.scalar_static_f64[440]=(self.scalar_static_f64[439]*self.scalar_static_f64[203]);
        self.scalar_static_f64[441]=(if self.scalar_static_bool[11]{self.scalar_static_f64[440]}else{self.scalar_static_f64[439]});
        self.scalar_static_f64[442]=(if self.scalar_static_bool[11]{self.scalar_static_f64[212]}else{self.scalar_static_f64[441]});
        self.scalar_static_bool[70]=(self.scalar_static_f64[442]>1e-15);
        self.scalar_static_f64[443]=(if self.scalar_static_bool[70]{self.scalar_static_f64[442]}else{1e-15});
        self.scalar_static_f64[444]=(if self.scalar_static_bool[11]{self.scalar_static_f64[443]}else{self.scalar_static_f64[442]});
        self.scalar_static_f64[445]=(if self.scalar_static_bool[29]{self.scalar_static_f64[238]}else{self.scalar_static_f64[444]});
        self.scalar_static_f64[446]=(if self.scalar_static_bool[29]{self.scalar_static_f64[252]}else{self.scalar_static_f64[445]});
        self.scalar_static_f64[447]=(if self.scalar_static_bool[11]{self.scalar_static_f64[254]}else{self.scalar_static_f64[446]});
        self.scalar_static_f64[448]=(self.scalar_static_f64[431]*self.scalar_static_f64[257]);
        self.scalar_static_f64[449]=(1.0+self.scalar_static_f64[448]);
        self.scalar_static_f64[450]=(self.scalar_static_f64[434]*self.scalar_static_f64[258]);
        self.scalar_static_f64[451]=(self.scalar_static_f64[449]+self.scalar_static_f64[450]);
        self.scalar_static_f64[452]=(self.scalar_static_f64[431]*self.scalar_static_f64[259]);
        self.scalar_static_f64[453]=(self.scalar_static_f64[434]*self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=(self.scalar_static_f64[451]+self.scalar_static_f64[453]);
        self.scalar_static_bool[71]=(self.scalar_static_f64[454]>1e-10);
        self.scalar_static_f64[455]=(if self.scalar_static_bool[71]{self.scalar_static_f64[454]}else{1e-10});
        self.scalar_static_f64[456]=(if self.scalar_static_bool[11]{self.scalar_static_f64[455]}else{self.scalar_static_f64[447]});
        self.scalar_static_f64[457]=(self.scalar_static_f64[456]/self.scalar_static_f64[307]);
        self.scalar_static_f64[458]=(if self.scalar_static_bool[11]{self.scalar_static_f64[457]}else{self.scalar_static_f64[456]});
        self.scalar_static_f64[459]=(self.scalar_static_f64[10]/self.scalar_static_f64[458]);
        self.scalar_static_f64[460]=(if self.scalar_static_bool[11]{self.scalar_static_f64[459]}else{0.0});
        self.scalar_static_bool[72]=(self.scalar_static_f64[460]>1e-6);
        self.scalar_static_f64[461]=(if self.scalar_static_bool[72]{self.scalar_static_f64[460]}else{1e-6});
        self.scalar_static_f64[462]=(if self.scalar_static_bool[11]{self.scalar_static_f64[461]}else{self.scalar_static_f64[28]});
        self.scalar_static_f64[463]=(self.scalar_static_f64[458]*self.scalar_static_f64[310]);
        self.scalar_static_f64[464]=(if self.scalar_static_bool[11]{self.scalar_static_f64[463]}else{0.0});
        self.scalar_static_bool[73]=(self.scalar_static_f64[464]>0.0);
        self.scalar_static_f64[465]=(if self.scalar_static_bool[73]{self.scalar_static_f64[464]}else{0.0});
        self.scalar_static_f64[466]=(if self.scalar_static_bool[11]{self.scalar_static_f64[465]}else{self.scalar_static_f64[32]});
        self.scalar_static_f64[467]=(self.scalar_static_f64[425]*0.000473);
        self.scalar_static_f64[468]=(self.scalar_static_f64[424]+636.0);
        self.scalar_static_f64[469]=(self.scalar_static_f64[467]/self.scalar_static_f64[468]);
        self.scalar_static_f64[470]=(1.17-self.scalar_static_f64[469]);
        self.scalar_static_f64[471]=(self.scalar_static_f64[425]*0.0004774);
        self.scalar_static_f64[472]=(self.scalar_static_f64[424]+235.0);
        self.scalar_static_f64[473]=(self.scalar_static_f64[471]/self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=(0.744-self.scalar_static_f64[473]);
        self.scalar_static_f64[475]=(self.scalar_static_f64[474]-self.scalar_static_f64[470]);
        self.scalar_static_f64[476]=(self.scalar_static_f64[475]+self.scalar_static_f64[351]);
        self.scalar_static_f64[477]=(self.scalar_static_f64[114]*self.scalar_static_f64[476]);
        self.scalar_static_f64[478]=(self.scalar_static_f64[470]+self.scalar_static_f64[477]);
        self.scalar_static_f64[479]=(0.5*self.scalar_static_f64[478]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[428]*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(self.scalar_static_f64[426]).ln();
        self.scalar_static_f64[482]=(self.scalar_static_f64[309]*self.scalar_static_f64[481]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[482]).exp();
        self.scalar_static_f64[484]=(self.scalar_static_f64[462]*self.scalar_static_f64[483]);
        self.scalar_static_f64[485]=(self.scalar_static_f64[466]*self.scalar_static_f64[385]);
        self.scalar_static_f64[486]=(if (self.scalar_static_f64[369]!=0.0){self.scalar_static_f64[485]}else{0.0});
        self.scalar_static_f64[487]=(if self.scalar_static_bool[68]{0.0}else{self.scalar_static_f64[486]});
        self.scalar_static_f64[488]=(self.scalar_static_f64[49]*self.scalar_static_f64[487]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
