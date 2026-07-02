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
    pub(crate) scalar_static_f64: Box<[f64; 2388]>,
    pub(crate) scalar_static_bool: Box<[bool; 314]>,
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
            scalar_static_f64: boxed_zero_f64_array::<2388>(),
            scalar_static_bool: boxed_zero_bool_array::<314>(),
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
        self.scalar_static_f64[4]=p.p17;
        self.scalar_static_f64[5]=p.p18;
        self.scalar_static_f64[6]=p.p19;
        self.scalar_static_bool[1]=(!self.scalar_static_bool[0]);
        self.scalar_static_f64[7]=p.p0;
        self.scalar_static_bool[2]=(0.0==self.scalar_static_f64[7]);
        self.scalar_static_f64[8]=p.p172;
        self.scalar_static_bool[3]=(self.scalar_static_f64[8]>0.0);
        self.scalar_static_bool[4]=(self.scalar_static_bool[2]&&self.scalar_static_bool[3]);
        self.scalar_static_bool[5]=(self.scalar_static_f64[7]>0.0);
        self.scalar_static_f64[9]=p.p443;
        self.scalar_static_bool[6]=(self.scalar_static_f64[9]>0.0);
        self.scalar_static_bool[7]=(self.scalar_static_bool[5]&&self.scalar_static_bool[6]);
        self.scalar_static_bool[8]=(self.scalar_static_bool[4]||self.scalar_static_bool[7]);
        self.scalar_static_f64[10]=p.p5;
        self.scalar_static_f64[11]=(if self.scalar_static_bool[8]{self.scalar_static_f64[10]}else{0.0});
        self.scalar_static_bool[9]=(!self.scalar_static_bool[8]);
        self.scalar_static_f64[12]=(if self.scalar_static_bool[9]{0.0}else{self.scalar_static_f64[11]});
        self.scalar_static_f64[13]=p.p23;
        self.scalar_static_f64[14]=(if self.scalar_static_bool[2]{self.scalar_static_f64[13]}else{0.0});
        self.scalar_static_f64[15]=p.p22;
        self.scalar_static_f64[16]=(if self.scalar_static_bool[2]{self.scalar_static_f64[15]}else{0.0});
        self.scalar_static_f64[17]=p.p25;
        self.scalar_static_f64[18]=(if self.scalar_static_bool[2]{self.scalar_static_f64[17]}else{0.0});
        self.scalar_static_f64[19]=p.p24;
        self.scalar_static_f64[20]=(if self.scalar_static_bool[2]{self.scalar_static_f64[19]}else{0.0});
        self.scalar_static_f64[21]=p.p30;
        self.scalar_static_f64[22]=(if self.scalar_static_bool[2]{self.scalar_static_f64[21]}else{0.0});
        self.scalar_static_f64[23]=p.p41;
        self.scalar_static_f64[24]=(if self.scalar_static_bool[2]{self.scalar_static_f64[23]}else{0.0});
        self.scalar_static_f64[25]=p.p42;
        self.scalar_static_f64[26]=(if self.scalar_static_bool[2]{self.scalar_static_f64[25]}else{0.0});
        self.scalar_static_f64[27]=p.p43;
        self.scalar_static_f64[28]=(if self.scalar_static_bool[2]{self.scalar_static_f64[27]}else{0.0});
        self.scalar_static_f64[29]=p.p44;
        self.scalar_static_f64[30]=(if self.scalar_static_bool[2]{self.scalar_static_f64[29]}else{0.0});
        self.scalar_static_f64[31]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[32]=p.p45;
        self.scalar_static_bool[10]=(self.scalar_static_f64[32]<0.0);
        self.scalar_static_bool[11]=(self.scalar_static_bool[2]&&self.scalar_static_bool[10]);
        self.scalar_static_f64[33]=(if self.scalar_static_bool[11]{-1.0}else{self.scalar_static_f64[31]});
        self.scalar_static_f64[34]=(self.scalar_static_f64[32]).abs();
        self.scalar_static_bool[12]=(self.scalar_static_f64[34]<1e19);
        self.scalar_static_f64[35]=(if self.scalar_static_bool[12]{self.scalar_static_f64[34]}else{1e19});
        self.scalar_static_f64[36]=(self.scalar_static_f64[35]*1000000.0);
        self.scalar_static_f64[37]=(if self.scalar_static_bool[2]{self.scalar_static_f64[36]}else{0.0});
        self.scalar_static_f64[38]=p.p46;
        self.scalar_static_bool[13]=(self.scalar_static_f64[38]<0.0);
        self.scalar_static_bool[14]=(self.scalar_static_bool[2]&&self.scalar_static_bool[13]);
        self.scalar_static_f64[39]=(if self.scalar_static_bool[14]{-1.0}else{self.scalar_static_f64[31]});
        self.scalar_static_f64[40]=(self.scalar_static_f64[38]).abs();
        self.scalar_static_bool[15]=(self.scalar_static_f64[40]>1e16);
        self.scalar_static_f64[41]=(if self.scalar_static_bool[15]{self.scalar_static_f64[40]}else{1e16});
        self.scalar_static_bool[16]=(self.scalar_static_f64[41]<1e21);
        self.scalar_static_f64[42]=(if self.scalar_static_bool[16]{self.scalar_static_f64[41]}else{1e21});
        self.scalar_static_f64[43]=(1000000.0*self.scalar_static_f64[42]);
        self.scalar_static_f64[44]=(if self.scalar_static_bool[2]{self.scalar_static_f64[43]}else{0.0});
        self.scalar_static_f64[45]=p.p47;
        self.scalar_static_f64[46]=(if self.scalar_static_bool[2]{self.scalar_static_f64[45]}else{0.0});
        self.scalar_static_f64[47]=p.p48;
        self.scalar_static_f64[48]=(if self.scalar_static_bool[2]{self.scalar_static_f64[47]}else{0.0});
        self.scalar_static_f64[49]=p.p49;
        self.scalar_static_f64[50]=(1000000.0*self.scalar_static_f64[49]);
        self.scalar_static_f64[51]=(if self.scalar_static_bool[2]{self.scalar_static_f64[50]}else{0.0});
        self.scalar_static_f64[52]=p.p50;
        self.scalar_static_f64[53]=(1000000.0*self.scalar_static_f64[52]);
        self.scalar_static_f64[54]=(if self.scalar_static_bool[2]{self.scalar_static_f64[53]}else{0.0});
        self.scalar_static_f64[55]=p.p51;
        self.scalar_static_f64[56]=(if self.scalar_static_bool[2]{self.scalar_static_f64[55]}else{0.0});
        self.scalar_static_f64[57]=p.p52;
        self.scalar_static_f64[58]=(if self.scalar_static_bool[2]{self.scalar_static_f64[57]}else{0.0});
        self.scalar_static_f64[59]=p.p53;
        self.scalar_static_f64[60]=(if self.scalar_static_bool[2]{self.scalar_static_f64[59]}else{0.0});
        self.scalar_static_f64[61]=p.p54;
        self.scalar_static_f64[62]=(1000000.0*self.scalar_static_f64[61]);
        self.scalar_static_f64[63]=(if self.scalar_static_bool[2]{self.scalar_static_f64[62]}else{0.0});
        self.scalar_static_f64[64]=p.p55;
        self.scalar_static_f64[65]=(if self.scalar_static_bool[2]{self.scalar_static_f64[64]}else{0.0});
        self.scalar_static_f64[66]=p.p56;
        self.scalar_static_f64[67]=(if self.scalar_static_bool[2]{self.scalar_static_f64[66]}else{0.0});
        self.scalar_static_f64[68]=p.p57;
        self.scalar_static_f64[69]=(if self.scalar_static_bool[2]{self.scalar_static_f64[68]}else{0.0});
        self.scalar_static_f64[70]=p.p58;
        self.scalar_static_f64[71]=(self.scalar_static_f64[69]*self.scalar_static_f64[70]);
        self.scalar_static_f64[72]=(self.scalar_static_f64[30]*self.scalar_static_f64[71]);
        self.scalar_static_f64[73]=(self.scalar_static_f64[72]/self.scalar_static_f64[24]);
        self.scalar_static_f64[74]=(if self.scalar_static_bool[2]{self.scalar_static_f64[73]}else{0.0});
        self.scalar_static_f64[75]=p.p59;
        self.scalar_static_f64[76]=(1000000.0*self.scalar_static_f64[75]);
        self.scalar_static_f64[77]=(if self.scalar_static_bool[2]{self.scalar_static_f64[76]}else{0.0});
        self.scalar_static_f64[78]=p.p60;
        self.scalar_static_f64[79]=(if self.scalar_static_bool[2]{self.scalar_static_f64[78]}else{0.0});
        self.scalar_static_f64[80]=p.p61;
        self.scalar_static_f64[81]=(if self.scalar_static_bool[2]{self.scalar_static_f64[80]}else{0.0});
        self.scalar_static_f64[82]=p.p62;
        self.scalar_static_f64[83]=(if self.scalar_static_bool[2]{self.scalar_static_f64[82]}else{0.0});
        self.scalar_static_f64[84]=p.p63;
        self.scalar_static_f64[85]=(self.scalar_static_f64[83]*self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=(self.scalar_static_f64[30]*self.scalar_static_f64[85]);
        self.scalar_static_f64[87]=(self.scalar_static_f64[86]/self.scalar_static_f64[24]);
        self.scalar_static_f64[88]=(if self.scalar_static_bool[2]{self.scalar_static_f64[87]}else{0.0});
        self.scalar_static_f64[89]=p.p64;
        self.scalar_static_f64[90]=(if self.scalar_static_bool[2]{self.scalar_static_f64[89]}else{0.0});
        self.scalar_static_f64[91]=p.p65;
        self.scalar_static_f64[92]=(if self.scalar_static_bool[2]{self.scalar_static_f64[91]}else{0.0});
        self.scalar_static_f64[93]=p.p66;
        self.scalar_static_f64[94]=(if self.scalar_static_bool[2]{self.scalar_static_f64[93]}else{0.0});
        self.scalar_static_f64[95]=p.p67;
        self.scalar_static_f64[96]=(if self.scalar_static_bool[2]{self.scalar_static_f64[95]}else{0.0});
        self.scalar_static_f64[97]=p.p68;
        self.scalar_static_f64[98]=(if self.scalar_static_bool[2]{self.scalar_static_f64[97]}else{0.0});
        self.scalar_static_f64[99]=p.p69;
        self.scalar_static_f64[100]=(self.scalar_static_f64[98]*self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=(if self.scalar_static_bool[2]{self.scalar_static_f64[100]}else{0.0});
        self.scalar_static_f64[102]=p.p70;
        self.scalar_static_f64[103]=(if self.scalar_static_bool[2]{self.scalar_static_f64[102]}else{0.0});
        self.scalar_static_f64[104]=p.p71;
        self.scalar_static_f64[105]=(if self.scalar_static_bool[2]{self.scalar_static_f64[104]}else{0.0});
        self.scalar_static_f64[106]=p.p72;
        self.scalar_static_f64[107]=(if self.scalar_static_bool[2]{self.scalar_static_f64[106]}else{0.0});
        self.scalar_static_f64[108]=p.p73;
        self.scalar_static_f64[109]=(if self.scalar_static_bool[2]{self.scalar_static_f64[108]}else{0.0});
        self.scalar_static_f64[110]=p.p74;
        self.scalar_static_f64[111]=(if self.scalar_static_bool[2]{self.scalar_static_f64[110]}else{0.0});
        self.scalar_static_f64[112]=p.p75;
        self.scalar_static_f64[113]=(if self.scalar_static_bool[2]{self.scalar_static_f64[112]}else{0.0});
        self.scalar_static_f64[114]=p.p76;
        self.scalar_static_f64[115]=(if self.scalar_static_bool[2]{self.scalar_static_f64[114]}else{0.0});
        self.scalar_static_f64[116]=p.p77;
        self.scalar_static_f64[117]=(if self.scalar_static_bool[2]{self.scalar_static_f64[116]}else{0.0});
        self.scalar_static_f64[118]=p.p78;
        self.scalar_static_f64[119]=(if self.scalar_static_bool[2]{self.scalar_static_f64[118]}else{0.0});
        self.scalar_static_f64[120]=p.p79;
        self.scalar_static_f64[121]=(if self.scalar_static_bool[2]{self.scalar_static_f64[120]}else{0.0});
        self.scalar_static_f64[122]=p.p80;
        self.scalar_static_f64[123]=(if self.scalar_static_bool[2]{self.scalar_static_f64[122]}else{0.0});
        self.scalar_static_f64[124]=p.p81;
        self.scalar_static_f64[125]=(if self.scalar_static_bool[2]{self.scalar_static_f64[124]}else{0.0});
        self.scalar_static_f64[126]=p.p82;
        self.scalar_static_f64[127]=(if self.scalar_static_bool[2]{self.scalar_static_f64[126]}else{0.0});
        self.scalar_static_f64[128]=p.p83;
        self.scalar_static_f64[129]=(if self.scalar_static_bool[2]{self.scalar_static_f64[128]}else{0.0});
        self.scalar_static_f64[130]=p.p84;
        self.scalar_static_f64[131]=(if self.scalar_static_bool[2]{self.scalar_static_f64[130]}else{0.0});
        self.scalar_static_f64[132]=p.p85;
        self.scalar_static_f64[133]=(if self.scalar_static_bool[2]{self.scalar_static_f64[132]}else{0.0});
        self.scalar_static_f64[134]=p.p86;
        self.scalar_static_f64[135]=(if self.scalar_static_bool[2]{self.scalar_static_f64[134]}else{0.0});
        self.scalar_static_f64[136]=p.p87;
        self.scalar_static_f64[137]=(if self.scalar_static_bool[2]{self.scalar_static_f64[136]}else{0.0});
        self.scalar_static_f64[138]=p.p88;
        self.scalar_static_f64[139]=(if self.scalar_static_bool[2]{self.scalar_static_f64[138]}else{0.0});
        self.scalar_static_f64[140]=p.p89;
        self.scalar_static_f64[141]=(if self.scalar_static_bool[2]{self.scalar_static_f64[140]}else{0.0});
        self.scalar_static_f64[142]=p.p90;
        self.scalar_static_f64[143]=(if self.scalar_static_bool[2]{self.scalar_static_f64[142]}else{0.0});
        self.scalar_static_f64[144]=p.p91;
        self.scalar_static_f64[145]=(if self.scalar_static_bool[2]{self.scalar_static_f64[144]}else{0.0});
        self.scalar_static_f64[146]=p.p92;
        self.scalar_static_f64[147]=(if self.scalar_static_bool[2]{self.scalar_static_f64[146]}else{0.0});
        self.scalar_static_f64[148]=p.p93;
        self.scalar_static_f64[149]=(if self.scalar_static_bool[2]{self.scalar_static_f64[148]}else{0.0});
        self.scalar_static_f64[150]=p.p94;
        self.scalar_static_f64[151]=(if self.scalar_static_bool[2]{self.scalar_static_f64[150]}else{0.0});
        self.scalar_static_f64[152]=p.p95;
        self.scalar_static_f64[153]=(if self.scalar_static_bool[2]{self.scalar_static_f64[152]}else{0.0});
        self.scalar_static_f64[154]=p.p96;
        self.scalar_static_f64[155]=(if self.scalar_static_bool[2]{self.scalar_static_f64[154]}else{0.0});
        self.scalar_static_f64[156]=p.p97;
        self.scalar_static_f64[157]=(if self.scalar_static_bool[2]{self.scalar_static_f64[156]}else{0.0});
        self.scalar_static_f64[158]=p.p98;
        self.scalar_static_f64[159]=(if self.scalar_static_bool[2]{self.scalar_static_f64[158]}else{0.0});
        self.scalar_static_f64[160]=p.p99;
        self.scalar_static_f64[161]=(if self.scalar_static_bool[2]{self.scalar_static_f64[160]}else{0.0});
        self.scalar_static_f64[162]=p.p100;
        self.scalar_static_f64[163]=(if self.scalar_static_bool[2]{self.scalar_static_f64[162]}else{0.0});
        self.scalar_static_f64[164]=p.p101;
        self.scalar_static_f64[165]=(if self.scalar_static_bool[2]{self.scalar_static_f64[164]}else{0.0});
        self.scalar_static_f64[166]=p.p102;
        self.scalar_static_f64[167]=(if self.scalar_static_bool[2]{self.scalar_static_f64[166]}else{0.0});
        self.scalar_static_f64[168]=p.p103;
        self.scalar_static_f64[169]=(if self.scalar_static_bool[2]{self.scalar_static_f64[168]}else{0.0});
        self.scalar_static_f64[170]=p.p104;
        self.scalar_static_f64[171]=(if self.scalar_static_bool[2]{self.scalar_static_f64[170]}else{0.0});
        self.scalar_static_f64[172]=p.p105;
        self.scalar_static_f64[173]=(if self.scalar_static_bool[2]{self.scalar_static_f64[172]}else{0.0});
        self.scalar_static_f64[174]=p.p106;
        self.scalar_static_f64[175]=(if self.scalar_static_bool[2]{self.scalar_static_f64[174]}else{0.0});
        self.scalar_static_f64[176]=p.p120;
        self.scalar_static_f64[177]=(if self.scalar_static_bool[2]{self.scalar_static_f64[176]}else{0.0});
        self.scalar_static_f64[178]=p.p121;
        self.scalar_static_f64[179]=(if self.scalar_static_bool[2]{self.scalar_static_f64[178]}else{0.0});
        self.scalar_static_f64[180]=p.p107;
        self.scalar_static_f64[181]=(if self.scalar_static_bool[2]{self.scalar_static_f64[180]}else{0.0});
        self.scalar_static_f64[182]=p.p108;
        self.scalar_static_f64[183]=(if self.scalar_static_bool[2]{self.scalar_static_f64[182]}else{0.0});
        self.scalar_static_f64[184]=p.p109;
        self.scalar_static_f64[185]=(if self.scalar_static_bool[2]{self.scalar_static_f64[184]}else{0.0});
        self.scalar_static_f64[186]=p.p123;
        self.scalar_static_f64[187]=(if self.scalar_static_bool[2]{self.scalar_static_f64[186]}else{0.0});
        self.scalar_static_f64[188]=p.p110;
        self.scalar_static_f64[189]=(if self.scalar_static_bool[2]{self.scalar_static_f64[188]}else{0.0});
        self.scalar_static_f64[190]=p.p111;
        self.scalar_static_f64[191]=(if self.scalar_static_bool[2]{self.scalar_static_f64[190]}else{0.0});
        self.scalar_static_f64[192]=p.p112;
        self.scalar_static_f64[193]=(if self.scalar_static_bool[2]{self.scalar_static_f64[192]}else{0.0});
        self.scalar_static_f64[194]=p.p122;
        self.scalar_static_f64[195]=(if self.scalar_static_bool[2]{self.scalar_static_f64[194]}else{0.0});
        self.scalar_static_f64[196]=p.p113;
        self.scalar_static_f64[197]=(if self.scalar_static_bool[2]{self.scalar_static_f64[196]}else{0.0});
        self.scalar_static_f64[198]=p.p114;
        self.scalar_static_f64[199]=(if self.scalar_static_bool[2]{self.scalar_static_f64[198]}else{0.0});
        self.scalar_static_f64[200]=p.p115;
        self.scalar_static_f64[201]=(if self.scalar_static_bool[2]{self.scalar_static_f64[200]}else{0.0});
        self.scalar_static_f64[202]=p.p116;
        self.scalar_static_f64[203]=(if self.scalar_static_bool[2]{self.scalar_static_f64[202]}else{0.0});
        self.scalar_static_f64[204]=p.p117;
        self.scalar_static_f64[205]=(if self.scalar_static_bool[2]{self.scalar_static_f64[204]}else{0.0});
        self.scalar_static_f64[206]=p.p118;
        self.scalar_static_f64[207]=(if self.scalar_static_bool[2]{self.scalar_static_f64[206]}else{0.0});
        self.scalar_static_f64[208]=p.p119;
        self.scalar_static_f64[209]=(if self.scalar_static_bool[2]{self.scalar_static_f64[208]}else{0.0});
        self.scalar_static_f64[210]=p.p124;
        self.scalar_static_f64[211]=(if self.scalar_static_bool[2]{self.scalar_static_f64[210]}else{0.0});
        self.scalar_static_f64[212]=p.p125;
        self.scalar_static_f64[213]=(if self.scalar_static_bool[2]{self.scalar_static_f64[212]}else{0.0});
        self.scalar_static_f64[214]=p.p126;
        self.scalar_static_f64[215]=(if self.scalar_static_bool[2]{self.scalar_static_f64[214]}else{0.0});
        self.scalar_static_f64[216]=p.p127;
        self.scalar_static_f64[217]=(if self.scalar_static_bool[2]{self.scalar_static_f64[216]}else{0.0});
        self.scalar_static_f64[218]=p.p128;
        self.scalar_static_f64[219]=(if self.scalar_static_bool[2]{self.scalar_static_f64[218]}else{0.0});
        self.scalar_static_f64[220]=p.p129;
        self.scalar_static_f64[221]=(if self.scalar_static_bool[2]{self.scalar_static_f64[220]}else{0.0});
        self.scalar_static_f64[222]=p.p130;
        self.scalar_static_f64[223]=(if self.scalar_static_bool[2]{self.scalar_static_f64[222]}else{0.0});
        self.scalar_static_f64[224]=p.p131;
        self.scalar_static_f64[225]=(if self.scalar_static_bool[2]{self.scalar_static_f64[224]}else{0.0});
        self.scalar_static_f64[226]=p.p132;
        self.scalar_static_f64[227]=(if self.scalar_static_bool[2]{self.scalar_static_f64[226]}else{0.0});
        self.scalar_static_f64[228]=p.p133;
        self.scalar_static_f64[229]=(if self.scalar_static_bool[2]{self.scalar_static_f64[228]}else{0.0});
        self.scalar_static_f64[230]=p.p147;
        self.scalar_static_f64[231]=(if self.scalar_static_bool[2]{self.scalar_static_f64[230]}else{0.0});
        self.scalar_static_f64[232]=p.p148;
        self.scalar_static_f64[233]=(if self.scalar_static_bool[2]{self.scalar_static_f64[232]}else{0.0});
        self.scalar_static_f64[234]=p.p149;
        self.scalar_static_f64[235]=(if self.scalar_static_bool[2]{self.scalar_static_f64[234]}else{0.0});
        self.scalar_static_f64[236]=p.p150;
        self.scalar_static_f64[237]=(if self.scalar_static_bool[2]{self.scalar_static_f64[236]}else{0.0});
        self.scalar_static_f64[238]=p.p134;
        self.scalar_static_f64[239]=(if self.scalar_static_bool[2]{self.scalar_static_f64[238]}else{0.0});
        self.scalar_static_f64[240]=p.p135;
        self.scalar_static_f64[241]=(if self.scalar_static_bool[2]{self.scalar_static_f64[240]}else{0.0});
        self.scalar_static_f64[242]=p.p136;
        self.scalar_static_f64[243]=(if self.scalar_static_bool[2]{self.scalar_static_f64[242]}else{0.0});
        self.scalar_static_f64[244]=p.p137;
        self.scalar_static_f64[245]=(if self.scalar_static_bool[2]{self.scalar_static_f64[244]}else{0.0});
        self.scalar_static_f64[246]=p.p138;
        self.scalar_static_f64[247]=(if self.scalar_static_bool[2]{self.scalar_static_f64[246]}else{0.0});
        self.scalar_static_f64[248]=p.p139;
        self.scalar_static_f64[249]=(if self.scalar_static_bool[2]{self.scalar_static_f64[248]}else{0.0});
        self.scalar_static_f64[250]=p.p140;
        self.scalar_static_f64[251]=(if self.scalar_static_bool[2]{self.scalar_static_f64[250]}else{0.0});
        self.scalar_static_f64[252]=p.p141;
        self.scalar_static_f64[253]=(self.scalar_static_f64[251]*self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=(self.scalar_static_f64[30]*self.scalar_static_f64[253]);
        self.scalar_static_f64[255]=(self.scalar_static_f64[254]/self.scalar_static_f64[24]);
        self.scalar_static_f64[256]=(if self.scalar_static_bool[2]{self.scalar_static_f64[255]}else{0.0});
        self.scalar_static_f64[257]=p.p142;
        self.scalar_static_f64[258]=(if self.scalar_static_bool[2]{self.scalar_static_f64[257]}else{0.0});
        self.scalar_static_f64[259]=p.p143;
        self.scalar_static_f64[260]=(self.scalar_static_f64[258]*self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=(self.scalar_static_f64[30]*self.scalar_static_f64[260]);
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]/self.scalar_static_f64[24]);
        self.scalar_static_f64[263]=(if self.scalar_static_bool[2]{self.scalar_static_f64[262]}else{0.0});
        self.scalar_static_f64[264]=p.p144;
        self.scalar_static_f64[265]=(if self.scalar_static_bool[2]{self.scalar_static_f64[264]}else{0.0});
        self.scalar_static_f64[266]=p.p145;
        self.scalar_static_f64[267]=(if self.scalar_static_bool[2]{self.scalar_static_f64[266]}else{0.0});
        self.scalar_static_f64[268]=p.p146;
        self.scalar_static_f64[269]=(if self.scalar_static_bool[2]{self.scalar_static_f64[268]}else{0.0});
        self.scalar_static_f64[270]=p.p151;
        self.scalar_static_f64[271]=(if self.scalar_static_bool[2]{self.scalar_static_f64[270]}else{0.0});
        self.scalar_static_f64[272]=p.p152;
        self.scalar_static_f64[273]=(if self.scalar_static_bool[2]{self.scalar_static_f64[272]}else{0.0});
        self.scalar_static_f64[274]=p.p153;
        self.scalar_static_f64[275]=(1000000.0*self.scalar_static_f64[274]);
        self.scalar_static_f64[276]=(if self.scalar_static_bool[2]{self.scalar_static_f64[275]}else{0.0});
        self.scalar_static_f64[277]=p.p154;
        self.scalar_static_f64[278]=(if self.scalar_static_bool[2]{self.scalar_static_f64[277]}else{0.0});
        self.scalar_static_f64[279]=p.p155;
        self.scalar_static_f64[280]=(if self.scalar_static_bool[2]{self.scalar_static_f64[279]}else{0.0});
        self.scalar_static_f64[281]=(if self.scalar_static_bool[2]{self.scalar_static_f64[56]}else{0.0});
        self.scalar_static_f64[282]=(if self.scalar_static_bool[2]{self.scalar_static_f64[58]}else{0.0});
        self.scalar_static_f64[283]=(if self.scalar_static_bool[2]{self.scalar_static_f64[69]}else{0.0});
        self.scalar_static_f64[284]=(if self.scalar_static_bool[2]{self.scalar_static_f64[74]}else{0.0});
        self.scalar_static_f64[285]=(if self.scalar_static_bool[2]{self.scalar_static_f64[83]}else{0.0});
        self.scalar_static_f64[286]=(if self.scalar_static_bool[2]{self.scalar_static_f64[88]}else{0.0});
        self.scalar_static_f64[287]=(if self.scalar_static_bool[2]{self.scalar_static_f64[149]}else{0.0});
        self.scalar_static_f64[288]=(if self.scalar_static_bool[2]{self.scalar_static_f64[157]}else{0.0});
        self.scalar_static_f64[289]=(if self.scalar_static_bool[2]{self.scalar_static_f64[159]}else{0.0});
        self.scalar_static_f64[290]=p.p11;
        self.scalar_static_bool[17]=(self.scalar_static_f64[290]>0.0);
        self.scalar_static_bool[18]=(self.scalar_static_bool[2]&&self.scalar_static_bool[17]);
        self.scalar_static_f64[291]=(if self.scalar_static_bool[18]{self.scalar_static_f64[55]}else{self.scalar_static_f64[281]});
        self.scalar_static_f64[292]=if param_given[156] { 1.0 } else { 0.0 };
        self.scalar_static_bool[19]=(1.0==self.scalar_static_f64[292]);
        self.scalar_static_bool[20]=(self.scalar_static_bool[18]&&self.scalar_static_bool[19]);
        self.scalar_static_f64[293]=p.p156;
        self.scalar_static_f64[294]=(if self.scalar_static_bool[20]{self.scalar_static_f64[293]}else{self.scalar_static_f64[291]});
        self.scalar_static_f64[295]=(if self.scalar_static_bool[18]{self.scalar_static_f64[57]}else{self.scalar_static_f64[282]});
        self.scalar_static_f64[296]=if param_given[157] { 1.0 } else { 0.0 };
        self.scalar_static_bool[21]=(1.0==self.scalar_static_f64[296]);
        self.scalar_static_bool[22]=(self.scalar_static_bool[18]&&self.scalar_static_bool[21]);
        self.scalar_static_f64[297]=p.p157;
        self.scalar_static_f64[298]=(if self.scalar_static_bool[22]{self.scalar_static_f64[297]}else{self.scalar_static_f64[295]});
        self.scalar_static_f64[299]=(if self.scalar_static_bool[18]{self.scalar_static_f64[68]}else{self.scalar_static_f64[283]});
        self.scalar_static_f64[300]=if param_given[158] { 1.0 } else { 0.0 };
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[300]);
        self.scalar_static_bool[24]=(self.scalar_static_bool[18]&&self.scalar_static_bool[23]);
        self.scalar_static_f64[301]=p.p158;
        self.scalar_static_f64[302]=(if self.scalar_static_bool[24]{self.scalar_static_f64[301]}else{self.scalar_static_f64[299]});
        self.scalar_static_f64[303]=(self.scalar_static_f64[70]*self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=(self.scalar_static_f64[30]*self.scalar_static_f64[303]);
        self.scalar_static_f64[305]=(self.scalar_static_f64[304]/self.scalar_static_f64[24]);
        self.scalar_static_f64[306]=(if self.scalar_static_bool[18]{self.scalar_static_f64[305]}else{self.scalar_static_f64[284]});
        self.scalar_static_f64[307]=(if self.scalar_static_bool[18]{self.scalar_static_f64[82]}else{self.scalar_static_f64[285]});
        self.scalar_static_f64[308]=if param_given[159] { 1.0 } else { 0.0 };
        self.scalar_static_bool[25]=(1.0==self.scalar_static_f64[308]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[18]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[309]=p.p159;
        self.scalar_static_f64[310]=(if self.scalar_static_bool[26]{self.scalar_static_f64[309]}else{self.scalar_static_f64[307]});
        self.scalar_static_f64[311]=(self.scalar_static_f64[84]*self.scalar_static_f64[310]);
        self.scalar_static_f64[312]=(self.scalar_static_f64[30]*self.scalar_static_f64[311]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[312]/self.scalar_static_f64[24]);
        self.scalar_static_f64[314]=(if self.scalar_static_bool[18]{self.scalar_static_f64[313]}else{self.scalar_static_f64[286]});
        self.scalar_static_f64[315]=(if self.scalar_static_bool[18]{self.scalar_static_f64[148]}else{self.scalar_static_f64[287]});
        self.scalar_static_f64[316]=if param_given[160] { 1.0 } else { 0.0 };
        self.scalar_static_bool[27]=(1.0==self.scalar_static_f64[316]);
        self.scalar_static_bool[28]=(self.scalar_static_bool[18]&&self.scalar_static_bool[27]);
        self.scalar_static_f64[317]=p.p160;
        self.scalar_static_f64[318]=(if self.scalar_static_bool[28]{self.scalar_static_f64[317]}else{self.scalar_static_f64[315]});
        self.scalar_static_f64[319]=(if self.scalar_static_bool[18]{self.scalar_static_f64[156]}else{self.scalar_static_f64[288]});
        self.scalar_static_f64[320]=if param_given[161] { 1.0 } else { 0.0 };
        self.scalar_static_bool[29]=(1.0==self.scalar_static_f64[320]);
        self.scalar_static_bool[30]=(self.scalar_static_bool[18]&&self.scalar_static_bool[29]);
        self.scalar_static_f64[321]=p.p161;
        self.scalar_static_f64[322]=(if self.scalar_static_bool[30]{self.scalar_static_f64[321]}else{self.scalar_static_f64[319]});
        self.scalar_static_f64[323]=(if self.scalar_static_bool[18]{self.scalar_static_f64[158]}else{self.scalar_static_f64[289]});
        self.scalar_static_f64[324]=if param_given[162] { 1.0 } else { 0.0 };
        self.scalar_static_bool[31]=(1.0==self.scalar_static_f64[324]);
        self.scalar_static_bool[32]=(self.scalar_static_bool[18]&&self.scalar_static_bool[31]);
        self.scalar_static_f64[325]=p.p162;
        self.scalar_static_f64[326]=(if self.scalar_static_bool[32]{self.scalar_static_f64[325]}else{self.scalar_static_f64[323]});
        self.scalar_static_f64[327]=p.p163;
        self.scalar_static_f64[328]=(if self.scalar_static_bool[2]{self.scalar_static_f64[327]}else{0.0});
        self.scalar_static_f64[329]=p.p164;
        self.scalar_static_f64[330]=(if self.scalar_static_bool[2]{self.scalar_static_f64[329]}else{0.0});
        self.scalar_static_f64[331]=p.p165;
        self.scalar_static_f64[332]=(if self.scalar_static_bool[2]{self.scalar_static_f64[331]}else{0.0});
        self.scalar_static_f64[333]=p.p166;
        self.scalar_static_f64[334]=(if self.scalar_static_bool[2]{self.scalar_static_f64[333]}else{0.0});
        self.scalar_static_f64[335]=p.p167;
        self.scalar_static_f64[336]=(if self.scalar_static_bool[2]{self.scalar_static_f64[335]}else{0.0});
        self.scalar_static_f64[337]=p.p168;
        self.scalar_static_f64[338]=(if self.scalar_static_bool[2]{self.scalar_static_f64[337]}else{0.0});
        self.scalar_static_f64[339]=p.p169;
        self.scalar_static_f64[340]=(if self.scalar_static_bool[2]{self.scalar_static_f64[339]}else{0.0});
        self.scalar_static_f64[341]=p.p170;
        self.scalar_static_f64[342]=(if self.scalar_static_bool[2]{self.scalar_static_f64[341]}else{0.0});
        self.scalar_static_f64[343]=p.p171;
        self.scalar_static_f64[344]=(if self.scalar_static_bool[2]{self.scalar_static_f64[343]}else{0.0});
        self.scalar_static_f64[345]=(if self.scalar_static_bool[2]{self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[346]=p.p173;
        self.scalar_static_f64[347]=(if self.scalar_static_bool[2]{self.scalar_static_f64[346]}else{0.0});
        self.scalar_static_f64[348]=p.p174;
        self.scalar_static_f64[349]=(if self.scalar_static_bool[2]{self.scalar_static_f64[348]}else{0.0});
        self.scalar_static_f64[350]=p.p175;
        self.scalar_static_f64[351]=(if self.scalar_static_bool[2]{self.scalar_static_f64[350]}else{0.0});
        self.scalar_static_f64[352]=p.p176;
        self.scalar_static_f64[353]=(if self.scalar_static_bool[2]{self.scalar_static_f64[352]}else{0.0});
        self.scalar_static_f64[354]=p.p183;
        self.scalar_static_f64[355]=(if self.scalar_static_bool[2]{self.scalar_static_f64[354]}else{0.0});
        self.scalar_static_f64[356]=p.p184;
        self.scalar_static_f64[357]=(if self.scalar_static_bool[2]{self.scalar_static_f64[356]}else{0.0});
        self.scalar_static_f64[358]=p.p185;
        self.scalar_static_f64[359]=(if self.scalar_static_bool[2]{self.scalar_static_f64[358]}else{0.0});
        self.scalar_static_f64[360]=p.p186;
        self.scalar_static_f64[361]=(if self.scalar_static_bool[2]{self.scalar_static_f64[360]}else{0.0});
        self.scalar_static_f64[362]=p.p187;
        self.scalar_static_f64[363]=(if self.scalar_static_bool[2]{self.scalar_static_f64[362]}else{0.0});
        self.scalar_static_f64[364]=p.p188;
        self.scalar_static_f64[365]=(if self.scalar_static_bool[2]{self.scalar_static_f64[364]}else{0.0});
        self.scalar_static_f64[366]=p.p189;
        self.scalar_static_f64[367]=(if self.scalar_static_bool[2]{self.scalar_static_f64[366]}else{0.0});
        self.scalar_static_f64[368]=p.p190;
        self.scalar_static_f64[369]=(if self.scalar_static_bool[2]{self.scalar_static_f64[368]}else{0.0});
        self.scalar_static_bool[33]=(!self.scalar_static_bool[2]);
        self.scalar_static_f64[370]=p.p29;
        self.scalar_static_f64[371]=(1.0/self.scalar_static_f64[370]);
        self.scalar_static_f64[372]=(if self.scalar_static_bool[33]{self.scalar_static_f64[371]}else{0.0});
        self.scalar_static_f64[373]=p.p21;
        self.scalar_static_f64[374]=(self.scalar_static_f64[372]*self.scalar_static_f64[373]);
        self.scalar_static_bool[34]=(self.scalar_static_f64[374]>1e-9);
        self.scalar_static_f64[375]=(if self.scalar_static_bool[34]{self.scalar_static_f64[374]}else{1e-9});
        self.scalar_static_f64[376]=(if self.scalar_static_bool[33]{self.scalar_static_f64[375]}else{0.0});
        self.scalar_static_f64[377]=(self.scalar_static_f64[13]*self.scalar_static_f64[372]);
        self.scalar_static_f64[378]=(if self.scalar_static_bool[33]{self.scalar_static_f64[377]}else{self.scalar_static_f64[14]});
        self.scalar_static_f64[379]=(self.scalar_static_f64[15]*self.scalar_static_f64[372]);
        self.scalar_static_f64[380]=(if self.scalar_static_bool[33]{self.scalar_static_f64[379]}else{self.scalar_static_f64[16]});
        self.scalar_static_f64[381]=(self.scalar_static_f64[17]*self.scalar_static_f64[372]);
        self.scalar_static_f64[382]=(if self.scalar_static_bool[33]{self.scalar_static_f64[381]}else{self.scalar_static_f64[18]});
        self.scalar_static_f64[383]=(self.scalar_static_f64[19]*self.scalar_static_f64[372]);
        self.scalar_static_f64[384]=(if self.scalar_static_bool[33]{self.scalar_static_f64[383]}else{self.scalar_static_f64[20]});
        self.scalar_static_f64[385]=(self.scalar_static_f64[21]*self.scalar_static_f64[370]);
        self.scalar_static_f64[386]=(if self.scalar_static_bool[33]{self.scalar_static_f64[385]}else{self.scalar_static_f64[22]});
        self.scalar_static_f64[387]=(if self.scalar_static_bool[33]{1e-6}else{0.0});
        self.scalar_static_f64[388]=p.p20;
        self.scalar_static_f64[389]=(self.scalar_static_f64[387]/self.scalar_static_f64[388]);
        self.scalar_static_f64[390]=(if self.scalar_static_bool[33]{self.scalar_static_f64[389]}else{0.0});
        self.scalar_static_f64[391]=(self.scalar_static_f64[387]/self.scalar_static_f64[376]);
        self.scalar_static_f64[392]=(if self.scalar_static_bool[33]{self.scalar_static_f64[391]}else{0.0});
        self.scalar_static_f64[393]=p.p191;
        self.scalar_static_f64[394]=p.p192;
        self.scalar_static_f64[395]=(self.scalar_static_f64[390]*self.scalar_static_f64[394]);
        self.scalar_static_f64[396]=(1.0+self.scalar_static_f64[395]);
        self.scalar_static_f64[397]=(self.scalar_static_f64[393]*self.scalar_static_f64[396]);
        self.scalar_static_f64[398]=p.p193;
        self.scalar_static_f64[399]=(self.scalar_static_f64[392]*self.scalar_static_f64[398]);
        self.scalar_static_f64[400]=(1.0+self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=(self.scalar_static_f64[397]*self.scalar_static_f64[400]);
        self.scalar_static_f64[402]=(if self.scalar_static_bool[33]{self.scalar_static_f64[401]}else{0.0});
        self.scalar_static_f64[403]=p.p195;
        self.scalar_static_f64[404]=p.p197;
        self.scalar_static_f64[405]=(self.scalar_static_f64[392]*self.scalar_static_f64[404]);
        self.scalar_static_f64[406]=(1.0+self.scalar_static_f64[405]);
        self.scalar_static_f64[407]=(self.scalar_static_f64[403]*self.scalar_static_f64[406]);
        self.scalar_static_f64[408]=p.p196;
        self.scalar_static_f64[409]=(self.scalar_static_f64[390]*self.scalar_static_f64[408]);
        self.scalar_static_f64[410]=(1.0+self.scalar_static_f64[409]);
        self.scalar_static_f64[411]=(self.scalar_static_f64[407]*self.scalar_static_f64[410]);
        self.scalar_static_f64[412]=(if self.scalar_static_bool[33]{self.scalar_static_f64[411]}else{0.0});
        self.scalar_static_f64[413]=(self.scalar_static_f64[388]+self.scalar_static_f64[402]);
        self.scalar_static_f64[414]=p.p194;
        self.scalar_static_f64[415]=(2.0*self.scalar_static_f64[414]);
        self.scalar_static_f64[416]=(self.scalar_static_f64[413]-self.scalar_static_f64[415]);
        self.scalar_static_bool[35]=(self.scalar_static_f64[416]>1e-9);
        self.scalar_static_f64[417]=(if self.scalar_static_bool[35]{self.scalar_static_f64[416]}else{1e-9});
        self.scalar_static_f64[418]=(if self.scalar_static_bool[33]{self.scalar_static_f64[417]}else{0.0});
        self.scalar_static_f64[419]=(self.scalar_static_f64[376]+self.scalar_static_f64[412]);
        self.scalar_static_f64[420]=p.p198;
        self.scalar_static_f64[421]=(2.0*self.scalar_static_f64[420]);
        self.scalar_static_f64[422]=(self.scalar_static_f64[419]-self.scalar_static_f64[421]);
        self.scalar_static_bool[36]=(self.scalar_static_f64[422]>1e-9);
        self.scalar_static_f64[423]=(if self.scalar_static_bool[36]{self.scalar_static_f64[422]}else{1e-9});
        self.scalar_static_f64[424]=(if self.scalar_static_bool[33]{self.scalar_static_f64[423]}else{0.0});
        self.scalar_static_f64[425]=p.p199;
        self.scalar_static_f64[426]=(self.scalar_static_f64[416]+self.scalar_static_f64[425]);
        self.scalar_static_bool[37]=(self.scalar_static_f64[426]>1e-9);
        self.scalar_static_f64[427]=(if self.scalar_static_bool[37]{self.scalar_static_f64[426]}else{1e-9});
        self.scalar_static_f64[428]=(if self.scalar_static_bool[33]{self.scalar_static_f64[427]}else{0.0});
        self.scalar_static_f64[429]=p.p200;
        self.scalar_static_f64[430]=(self.scalar_static_f64[422]+self.scalar_static_f64[429]);
        self.scalar_static_bool[38]=(self.scalar_static_f64[430]>1e-9);
        self.scalar_static_f64[431]=(if self.scalar_static_bool[38]{self.scalar_static_f64[430]}else{1e-9});
        self.scalar_static_f64[432]=(if self.scalar_static_bool[33]{self.scalar_static_f64[431]}else{0.0});
        self.scalar_static_f64[433]=(self.scalar_static_f64[387]/self.scalar_static_f64[418]);
        self.scalar_static_f64[434]=(if self.scalar_static_bool[33]{self.scalar_static_f64[433]}else{0.0});
        self.scalar_static_f64[435]=(self.scalar_static_f64[387]/self.scalar_static_f64[424]);
        self.scalar_static_f64[436]=(if self.scalar_static_bool[33]{self.scalar_static_f64[435]}else{0.0});
        self.scalar_static_f64[437]=(self.scalar_static_f64[434]*self.scalar_static_f64[436]);
        self.scalar_static_f64[438]=(if self.scalar_static_bool[33]{self.scalar_static_f64[437]}else{0.0});
        self.scalar_static_bool[39]=(self.scalar_static_f64[413]>1e-9);
        self.scalar_static_f64[439]=(if self.scalar_static_bool[39]{self.scalar_static_f64[413]}else{1e-9});
        self.scalar_static_bool[40]=(self.scalar_static_f64[419]>1e-9);
        self.scalar_static_f64[440]=(if self.scalar_static_bool[40]{self.scalar_static_f64[419]}else{1e-9});
        self.scalar_static_f64[441]=(if self.scalar_static_bool[33]{self.scalar_static_f64[439]}else{0.0});
        self.scalar_static_f64[442]=p.p499;
        self.scalar_static_f64[443]=(self.scalar_static_f64[441]+self.scalar_static_f64[442]);
        self.scalar_static_bool[41]=(self.scalar_static_f64[443]>1e-9);
        self.scalar_static_f64[444]=(if self.scalar_static_bool[41]{self.scalar_static_f64[443]}else{1e-9});
        self.scalar_static_f64[445]=(if self.scalar_static_bool[33]{self.scalar_static_f64[444]}else{0.0});
        self.scalar_static_f64[446]=(if self.scalar_static_bool[33]{self.scalar_static_f64[440]}else{0.0});
        self.scalar_static_f64[447]=p.p38;
        self.scalar_static_f64[448]=(0.5*self.scalar_static_f64[412]);
        self.scalar_static_f64[449]=(self.scalar_static_f64[447]-self.scalar_static_f64[448]);
        self.scalar_static_bool[42]=(self.scalar_static_f64[449]>1e-9);
        self.scalar_static_f64[450]=(if self.scalar_static_bool[42]{self.scalar_static_f64[449]}else{1e-9});
        self.scalar_static_f64[451]=(if self.scalar_static_bool[33]{self.scalar_static_f64[450]}else{0.0});
        self.scalar_static_f64[452]=p.p201;
        self.scalar_static_f64[453]=(if self.scalar_static_bool[33]{self.scalar_static_f64[452]}else{self.scalar_static_f64[24]});
        self.scalar_static_f64[454]=p.p202;
        self.scalar_static_f64[455]=(if self.scalar_static_bool[33]{self.scalar_static_f64[454]}else{self.scalar_static_f64[26]});
        self.scalar_static_f64[456]=p.p203;
        self.scalar_static_f64[457]=(if self.scalar_static_bool[33]{self.scalar_static_f64[456]}else{self.scalar_static_f64[28]});
        self.scalar_static_f64[458]=p.p204;
        self.scalar_static_f64[459]=(if self.scalar_static_bool[33]{self.scalar_static_f64[458]}else{self.scalar_static_f64[30]});
        self.scalar_static_f64[460]=(if self.scalar_static_bool[33]{1.0}else{self.scalar_static_f64[33]});
        self.scalar_static_f64[461]=p.p205;
        self.scalar_static_bool[43]=(self.scalar_static_f64[461]<0.0);
        self.scalar_static_bool[44]=(self.scalar_static_bool[33]&&self.scalar_static_bool[43]);
        self.scalar_static_f64[462]=(if self.scalar_static_bool[44]{-1.0}else{self.scalar_static_f64[460]});
        self.scalar_static_f64[463]=(self.scalar_static_f64[461]).abs();
        self.scalar_static_bool[45]=(self.scalar_static_f64[463]<1e19);
        self.scalar_static_f64[464]=(if self.scalar_static_bool[45]{self.scalar_static_f64[463]}else{1e19});
        self.scalar_static_f64[465]=(1000000.0*self.scalar_static_f64[464]);
        self.scalar_static_f64[466]=(if self.scalar_static_bool[33]{self.scalar_static_f64[465]}else{self.scalar_static_f64[37]});
        self.scalar_static_f64[467]=(if self.scalar_static_bool[33]{1.0}else{self.scalar_static_f64[39]});
        self.scalar_static_f64[468]=p.p206;
        self.scalar_static_bool[46]=(self.scalar_static_f64[468]<0.0);
        self.scalar_static_bool[47]=(self.scalar_static_bool[33]&&self.scalar_static_bool[46]);
        self.scalar_static_f64[469]=(if self.scalar_static_bool[47]{-1.0}else{self.scalar_static_f64[467]});
        self.scalar_static_f64[470]=(self.scalar_static_f64[468]).abs();
        self.scalar_static_bool[48]=(self.scalar_static_f64[470]>1e16);
        self.scalar_static_f64[471]=(if self.scalar_static_bool[48]{self.scalar_static_f64[470]}else{1e16});
        self.scalar_static_bool[49]=(self.scalar_static_f64[471]<1e21);
        self.scalar_static_f64[472]=(if self.scalar_static_bool[49]{self.scalar_static_f64[471]}else{1e21});
        self.scalar_static_f64[473]=(1000000.0*self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=(if self.scalar_static_bool[33]{self.scalar_static_f64[473]}else{self.scalar_static_f64[44]});
        self.scalar_static_f64[475]=p.p207;
        self.scalar_static_f64[476]=(if self.scalar_static_bool[33]{self.scalar_static_f64[475]}else{self.scalar_static_f64[46]});
        self.scalar_static_f64[477]=p.p208;
        self.scalar_static_f64[478]=(if self.scalar_static_bool[33]{self.scalar_static_f64[477]}else{self.scalar_static_f64[48]});
        self.scalar_static_f64[479]=p.p209;
        self.scalar_static_f64[480]=(1000000.0*self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=(if self.scalar_static_bool[33]{self.scalar_static_f64[480]}else{self.scalar_static_f64[51]});
        self.scalar_static_f64[482]=p.p210;
        self.scalar_static_f64[483]=(1000000.0*self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=(if self.scalar_static_bool[33]{self.scalar_static_f64[483]}else{self.scalar_static_f64[54]});
        self.scalar_static_f64[485]=p.p212;
        self.scalar_static_f64[486]=p.p213;
        self.scalar_static_f64[487]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[486]);
        self.scalar_static_f64[488]=(self.scalar_static_f64[485]*self.scalar_static_f64[487]);
        self.scalar_static_f64[489]=p.p214;
        self.scalar_static_f64[490]=p.p215;
        self.scalar_static_f64[491]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[490]);
        self.scalar_static_f64[492]=(self.scalar_static_f64[489]*self.scalar_static_f64[491]);
        self.scalar_static_f64[493]=(1.0+self.scalar_static_f64[492]);
        self.scalar_static_f64[494]=(self.scalar_static_f64[488]/self.scalar_static_f64[493]);
        self.scalar_static_f64[495]=p.p211;
        self.scalar_static_f64[496]=p.p216;
        self.scalar_static_f64[497]=(self.scalar_static_f64[436]*self.scalar_static_f64[496]);
        self.scalar_static_f64[498]=p.p217;
        self.scalar_static_f64[499]=(self.scalar_static_f64[438]*self.scalar_static_f64[498]);
        self.scalar_static_f64[500]=p.p218;
        self.scalar_static_f64[501]=p.p219;
        self.scalar_static_f64[502]=(self.scalar_static_f64[459]*self.scalar_static_f64[501]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[502]/self.scalar_static_f64[453]);
        self.scalar_static_f64[504]=p.p220;
        self.scalar_static_f64[505]=p.p221;
        self.scalar_static_f64[506]=(self.scalar_static_f64[434]*self.scalar_static_f64[505]);
        self.scalar_static_f64[507]=(1.0+self.scalar_static_f64[506]);
        self.scalar_static_f64[508]=(self.scalar_static_f64[504]*self.scalar_static_f64[507]);
        self.scalar_static_f64[509]=p.p222;
        self.scalar_static_f64[510]=(self.scalar_static_f64[436]*self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=(1.0+self.scalar_static_f64[510]);
        self.scalar_static_f64[512]=(self.scalar_static_f64[508]*self.scalar_static_f64[511]);
        self.scalar_static_f64[513]=p.p223;
        self.scalar_static_f64[514]=(self.scalar_static_f64[438]*self.scalar_static_f64[513]);
        self.scalar_static_f64[515]=(1.0+self.scalar_static_f64[514]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[512]*self.scalar_static_f64[515]);
        self.scalar_static_f64[517]=(if self.scalar_static_bool[33]{self.scalar_static_f64[516]}else{self.scalar_static_f64[60]});
        self.scalar_static_f64[518]=p.p224;
        self.scalar_static_f64[519]=p.p225;
        self.scalar_static_f64[520]=(self.scalar_static_f64[434]*self.scalar_static_f64[519]);
        self.scalar_static_f64[521]=(1.0+self.scalar_static_f64[520]);
        self.scalar_static_f64[522]=(self.scalar_static_f64[518]*self.scalar_static_f64[521]);
        self.scalar_static_f64[523]=(1000000.0*self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=(if self.scalar_static_bool[33]{self.scalar_static_f64[523]}else{0.0});
        self.scalar_static_bool[50]=(self.scalar_static_f64[524]>1e25);
        self.scalar_static_f64[525]=(if self.scalar_static_bool[50]{self.scalar_static_f64[524]}else{1e25});
        self.scalar_static_bool[51]=(self.scalar_static_f64[525]<1e28);
        self.scalar_static_f64[526]=(if self.scalar_static_bool[51]{self.scalar_static_f64[525]}else{1e28});
        self.scalar_static_f64[527]=(if self.scalar_static_bool[33]{self.scalar_static_f64[526]}else{self.scalar_static_f64[63]});
        self.scalar_static_f64[528]=p.p226;
        self.scalar_static_f64[529]=(if self.scalar_static_bool[33]{self.scalar_static_f64[528]}else{self.scalar_static_f64[65]});
        self.scalar_static_f64[530]=p.p227;
        self.scalar_static_f64[531]=(if self.scalar_static_bool[33]{self.scalar_static_f64[530]}else{self.scalar_static_f64[67]});
        self.scalar_static_f64[532]=(1.0-self.scalar_static_f64[457]);
        self.scalar_static_f64[533]=(if self.scalar_static_bool[33]{self.scalar_static_f64[532]}else{0.0});
        self.scalar_static_f64[534]=(self.scalar_static_f64[533]*1.04479e-10);
        self.scalar_static_f64[535]=(self.scalar_static_f64[457]*1.43438e-10);
        self.scalar_static_f64[536]=(self.scalar_static_f64[534]+self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=(if self.scalar_static_bool[33]{self.scalar_static_f64[536]}else{0.0});
        self.scalar_static_f64[538]=(self.scalar_static_f64[537]/3.45313e-11);
        self.scalar_static_f64[539]=(self.scalar_static_f64[455]*self.scalar_static_f64[538]);
        self.scalar_static_f64[540]=(self.scalar_static_f64[453]+4e-10);
        self.scalar_static_f64[541]=(self.scalar_static_f64[539]*self.scalar_static_f64[540]);
        self.scalar_static_f64[542]=(self.scalar_static_f64[541]).sqrt();
        self.scalar_static_f64[543]=(self.scalar_static_f64[542]/self.scalar_static_f64[418]);
        self.scalar_static_f64[544]=(if self.scalar_static_bool[33]{self.scalar_static_f64[543]}else{0.0});
        self.scalar_static_f64[545]=p.p228;
        self.scalar_static_f64[546]=(2.0*self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=p.p229;
        self.scalar_static_f64[548]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[547]);
        self.scalar_static_f64[549]=(self.scalar_static_f64[546]*self.scalar_static_f64[548]);
        self.scalar_static_f64[550]=p.p230;
        self.scalar_static_f64[551]=(self.scalar_static_f64[436]*self.scalar_static_f64[550]);
        self.scalar_static_f64[552]=(1.0+self.scalar_static_f64[551]);
        self.scalar_static_f64[553]=(self.scalar_static_f64[549]*self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=(if self.scalar_static_bool[33]{self.scalar_static_f64[553]}else{0.0});
        self.scalar_static_bool[52]=(self.scalar_static_f64[554]>0.0);
        self.scalar_static_f64[555]=(if self.scalar_static_bool[52]{self.scalar_static_f64[554]}else{0.0});
        self.scalar_static_bool[53]=(self.scalar_static_f64[555]<5.0);
        self.scalar_static_f64[556]=(if self.scalar_static_bool[53]{self.scalar_static_f64[555]}else{5.0});
        self.scalar_static_f64[557]=(if self.scalar_static_bool[33]{self.scalar_static_f64[556]}else{self.scalar_static_f64[69]});
        self.scalar_static_f64[558]=p.p231;
        self.scalar_static_f64[559]=(self.scalar_static_f64[557]*self.scalar_static_f64[558]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[459]*self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=(self.scalar_static_f64[560]/self.scalar_static_f64[453]);
        self.scalar_static_f64[562]=(if self.scalar_static_bool[33]{self.scalar_static_f64[561]}else{self.scalar_static_f64[74]});
        self.scalar_static_f64[563]=p.p232;
        self.scalar_static_f64[564]=(1000000.0*self.scalar_static_f64[563]);
        self.scalar_static_f64[565]=(if self.scalar_static_bool[33]{self.scalar_static_f64[564]}else{self.scalar_static_f64[77]});
        self.scalar_static_f64[566]=p.p233;
        self.scalar_static_f64[567]=(if self.scalar_static_bool[33]{self.scalar_static_f64[566]}else{self.scalar_static_f64[79]});
        self.scalar_static_f64[568]=p.p234;
        self.scalar_static_f64[569]=(self.scalar_static_f64[436]*self.scalar_static_f64[568]);
        self.scalar_static_f64[570]=(if self.scalar_static_bool[33]{self.scalar_static_f64[569]}else{0.0});
        self.scalar_static_bool[54]=(self.scalar_static_f64[570]> -1.0);
        self.scalar_static_f64[571]=(if self.scalar_static_bool[54]{self.scalar_static_f64[570]}else{-1.0});
        self.scalar_static_bool[55]=(self.scalar_static_f64[571]<1.0);
        self.scalar_static_f64[572]=(if self.scalar_static_bool[55]{self.scalar_static_f64[571]}else{1.0});
        self.scalar_static_f64[573]=(if self.scalar_static_bool[33]{self.scalar_static_f64[572]}else{self.scalar_static_f64[81]});
        self.scalar_static_f64[574]=p.p236;
        self.scalar_static_f64[575]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[574]);
        self.scalar_static_f64[576]=p.p237;
        self.scalar_static_f64[577]=(self.scalar_static_f64[436]*self.scalar_static_f64[576]);
        self.scalar_static_f64[578]=(1.0+self.scalar_static_f64[577]);
        self.scalar_static_f64[579]=(self.scalar_static_f64[575]*self.scalar_static_f64[578]);
        self.scalar_static_f64[580]=p.p235;
        self.scalar_static_f64[581]=p.p238;
        self.scalar_static_f64[582]=p.p239;
        self.scalar_static_f64[583]=p.p240;
        self.scalar_static_f64[584]=(if self.scalar_static_bool[33]{self.scalar_static_f64[583]}else{self.scalar_static_f64[92]});
        self.scalar_static_f64[585]=p.p241;
        self.scalar_static_f64[586]=(self.scalar_static_f64[434]*self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=p.p242;
        self.scalar_static_f64[588]=(self.scalar_static_f64[436]*self.scalar_static_f64[587]);
        self.scalar_static_f64[589]=(1.0+self.scalar_static_f64[588]);
        self.scalar_static_bool[56]=(self.scalar_static_f64[589]>0.001);
        self.scalar_static_f64[590]=(if self.scalar_static_bool[56]{self.scalar_static_f64[589]}else{0.001});
        self.scalar_static_f64[591]=(self.scalar_static_f64[586]/self.scalar_static_f64[590]);
        self.scalar_static_f64[592]=(if self.scalar_static_bool[33]{self.scalar_static_f64[591]}else{self.scalar_static_f64[94]});
        self.scalar_static_f64[593]=p.p243;
        self.scalar_static_f64[594]=(if self.scalar_static_bool[33]{self.scalar_static_f64[593]}else{self.scalar_static_f64[96]});
        self.scalar_static_f64[595]=(-self.scalar_static_f64[418]);
        self.scalar_static_f64[596]=p.p247;
        self.scalar_static_f64[597]=p.p248;
        self.scalar_static_f64[598]=(self.scalar_static_f64[436]*self.scalar_static_f64[597]);
        self.scalar_static_f64[599]=(1.0+self.scalar_static_f64[598]);
        self.scalar_static_bool[57]=(self.scalar_static_f64[599]>0.001);
        self.scalar_static_f64[600]=(if self.scalar_static_bool[57]{self.scalar_static_f64[599]}else{0.001});
        self.scalar_static_f64[601]=(self.scalar_static_f64[596]*self.scalar_static_f64[600]);
        self.scalar_static_f64[602]=(self.scalar_static_f64[595]/self.scalar_static_f64[601]);
        self.scalar_static_f64[603]=(if self.scalar_static_bool[33]{self.scalar_static_f64[602]}else{0.0});
        self.scalar_static_bool[58]=(self.scalar_static_f64[603]> -80.0);
        self.scalar_static_bool[59]=(self.scalar_static_bool[33]&&self.scalar_static_bool[58]);
        self.scalar_static_f64[604]=(self.scalar_static_f64[603]).exp();
        self.scalar_static_f64[605]=(if self.scalar_static_bool[59]{self.scalar_static_f64[604]}else{0.0});
        self.scalar_static_bool[60]=(!self.scalar_static_bool[58]);
        self.scalar_static_bool[61]=(self.scalar_static_bool[33]&&self.scalar_static_bool[60]);
        self.scalar_static_f64[606]=(-self.scalar_static_f64[603]);
        self.scalar_static_f64[607]=(self.scalar_static_f64[606]-80.0);
        self.scalar_static_f64[608]=(0.5*self.scalar_static_f64[607]);
        self.scalar_static_f64[609]=(self.scalar_static_f64[607]*0.3333333333333);
        self.scalar_static_f64[610]=(1.0+self.scalar_static_f64[609]);
        self.scalar_static_f64[611]=(self.scalar_static_f64[608]*self.scalar_static_f64[610]);
        self.scalar_static_f64[612]=(1.0+self.scalar_static_f64[611]);
        self.scalar_static_f64[613]=(self.scalar_static_f64[607]*self.scalar_static_f64[612]);
        self.scalar_static_f64[614]=(1.0+self.scalar_static_f64[613]);
        self.scalar_static_f64[615]=(1.80485e-35/self.scalar_static_f64[614]);
        self.scalar_static_f64[616]=(if self.scalar_static_bool[61]{self.scalar_static_f64[615]}else{self.scalar_static_f64[605]});
        self.scalar_static_f64[617]=p.p250;
        self.scalar_static_f64[618]=(self.scalar_static_f64[595]/self.scalar_static_f64[617]);
        self.scalar_static_f64[619]=(if self.scalar_static_bool[33]{self.scalar_static_f64[618]}else{0.0});
        self.scalar_static_bool[62]=(self.scalar_static_f64[619]> -80.0);
        self.scalar_static_bool[63]=(self.scalar_static_bool[33]&&self.scalar_static_bool[62]);
        self.scalar_static_f64[620]=(self.scalar_static_f64[619]).exp();
        self.scalar_static_f64[621]=(if self.scalar_static_bool[63]{self.scalar_static_f64[620]}else{0.0});
        self.scalar_static_bool[64]=(!self.scalar_static_bool[62]);
        self.scalar_static_bool[65]=(self.scalar_static_bool[33]&&self.scalar_static_bool[64]);
        self.scalar_static_f64[622]=(-self.scalar_static_f64[619]);
        self.scalar_static_f64[623]=(self.scalar_static_f64[622]-80.0);
        self.scalar_static_f64[624]=(0.5*self.scalar_static_f64[623]);
        self.scalar_static_f64[625]=(0.3333333333333*self.scalar_static_f64[623]);
        self.scalar_static_f64[626]=(1.0+self.scalar_static_f64[625]);
        self.scalar_static_f64[627]=(self.scalar_static_f64[624]*self.scalar_static_f64[626]);
        self.scalar_static_f64[628]=(1.0+self.scalar_static_f64[627]);
        self.scalar_static_f64[629]=(self.scalar_static_f64[623]*self.scalar_static_f64[628]);
        self.scalar_static_f64[630]=(1.0+self.scalar_static_f64[629]);
        self.scalar_static_f64[631]=(1.80485e-35/self.scalar_static_f64[630]);
        self.scalar_static_f64[632]=(if self.scalar_static_bool[65]{self.scalar_static_f64[631]}else{self.scalar_static_f64[621]});
        self.scalar_static_f64[633]=p.p245;
        self.scalar_static_f64[634]=p.p246;
        self.scalar_static_f64[635]=(self.scalar_static_f64[436]*self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(1.0+self.scalar_static_f64[635]);
        self.scalar_static_f64[637]=(self.scalar_static_f64[633]*self.scalar_static_f64[636]);
        self.scalar_static_f64[638]=(self.scalar_static_f64[616]-1.0);
        self.scalar_static_f64[639]=(self.scalar_static_f64[637]*self.scalar_static_f64[638]);
        self.scalar_static_f64[640]=(self.scalar_static_f64[639]/self.scalar_static_f64[603]);
        self.scalar_static_f64[641]=(1.0+self.scalar_static_f64[640]);
        self.scalar_static_f64[642]=p.p249;
        self.scalar_static_f64[643]=(self.scalar_static_f64[632]-1.0);
        self.scalar_static_f64[644]=(self.scalar_static_f64[642]*self.scalar_static_f64[643]);
        self.scalar_static_f64[645]=(self.scalar_static_f64[644]/self.scalar_static_f64[619]);
        self.scalar_static_f64[646]=(self.scalar_static_f64[641]+self.scalar_static_f64[645]);
        self.scalar_static_bool[66]=(self.scalar_static_f64[646]>1e-6);
        self.scalar_static_f64[647]=(if self.scalar_static_bool[66]{self.scalar_static_f64[646]}else{1e-6});
        self.scalar_static_f64[648]=(if self.scalar_static_bool[33]{self.scalar_static_f64[647]}else{0.0});
        self.scalar_static_f64[649]=p.p251;
        self.scalar_static_f64[650]=(self.scalar_static_f64[436]*self.scalar_static_f64[649]);
        self.scalar_static_f64[651]=(1.0+self.scalar_static_f64[650]);
        self.scalar_static_f64[652]=p.p252;
        self.scalar_static_f64[653]=(self.scalar_static_f64[436]*self.scalar_static_f64[652]);
        self.scalar_static_f64[654]=p.p253;
        self.scalar_static_f64[655]=(self.scalar_static_f64[424]/self.scalar_static_f64[654]);
        self.scalar_static_f64[656]=(1.0+self.scalar_static_f64[655]);
        self.scalar_static_f64[657]=(self.scalar_static_f64[656]).ln();
        self.scalar_static_f64[658]=(self.scalar_static_f64[653]*self.scalar_static_f64[657]);
        self.scalar_static_f64[659]=(self.scalar_static_f64[651]+self.scalar_static_f64[658]);
        self.scalar_static_bool[67]=(self.scalar_static_f64[659]>1e-6);
        self.scalar_static_f64[660]=(if self.scalar_static_bool[67]{self.scalar_static_f64[659]}else{1e-6});
        self.scalar_static_f64[661]=(if self.scalar_static_bool[33]{self.scalar_static_f64[660]}else{0.0});
        self.scalar_static_f64[662]=p.p244;
        self.scalar_static_f64[663]=(self.scalar_static_f64[662]/self.scalar_static_f64[648]);
        self.scalar_static_f64[664]=(self.scalar_static_f64[661]*self.scalar_static_f64[663]);
        self.scalar_static_f64[665]=(if self.scalar_static_bool[33]{self.scalar_static_f64[664]}else{0.0});
        self.scalar_static_f64[666]=(self.scalar_static_f64[424]*self.scalar_static_f64[665]);
        self.scalar_static_f64[667]=(self.scalar_static_f64[666]/self.scalar_static_f64[418]);
        self.scalar_static_f64[668]=(if self.scalar_static_bool[33]{self.scalar_static_f64[667]}else{0.0});
        self.scalar_static_bool[68]=(self.scalar_static_f64[668]>1e-10);
        self.scalar_static_f64[669]=(if self.scalar_static_bool[68]{self.scalar_static_f64[668]}else{1e-10});
        self.scalar_static_f64[670]=(if self.scalar_static_bool[33]{self.scalar_static_f64[669]}else{self.scalar_static_f64[98]});
        self.scalar_static_f64[671]=p.p254;
        self.scalar_static_f64[672]=(self.scalar_static_f64[670]*self.scalar_static_f64[671]);
        self.scalar_static_f64[673]=(if self.scalar_static_bool[33]{self.scalar_static_f64[672]}else{self.scalar_static_f64[101]});
        self.scalar_static_f64[674]=p.p255;
        self.scalar_static_f64[675]=p.p256;
        self.scalar_static_f64[676]=(self.scalar_static_f64[434]*self.scalar_static_f64[675]);
        self.scalar_static_f64[677]=(1.0+self.scalar_static_f64[676]);
        self.scalar_static_f64[678]=(self.scalar_static_f64[674]*self.scalar_static_f64[677]);
        self.scalar_static_f64[679]=p.p257;
        self.scalar_static_f64[680]=(self.scalar_static_f64[436]*self.scalar_static_f64[679]);
        self.scalar_static_f64[681]=(1.0+self.scalar_static_f64[680]);
        self.scalar_static_f64[682]=(self.scalar_static_f64[678]*self.scalar_static_f64[681]);
        self.scalar_static_f64[683]=p.p258;
        self.scalar_static_f64[684]=(self.scalar_static_f64[438]*self.scalar_static_f64[683]);
        self.scalar_static_f64[685]=(1.0+self.scalar_static_f64[684]);
        self.scalar_static_f64[686]=(self.scalar_static_f64[682]*self.scalar_static_f64[685]);
        self.scalar_static_f64[687]=(if self.scalar_static_bool[33]{self.scalar_static_f64[686]}else{self.scalar_static_f64[103]});
        self.scalar_static_f64[688]=p.p259;
        self.scalar_static_f64[689]=p.p260;
        self.scalar_static_f64[690]=p.p261;
        self.scalar_static_f64[691]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[690]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[689]*self.scalar_static_f64[691]);
        self.scalar_static_f64[693]=(self.scalar_static_f64[688]+self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=p.p262;
        self.scalar_static_f64[695]=(self.scalar_static_f64[436]*self.scalar_static_f64[694]);
        self.scalar_static_f64[696]=(1.0+self.scalar_static_f64[695]);
        self.scalar_static_f64[697]=(self.scalar_static_f64[693]*self.scalar_static_f64[696]);
        self.scalar_static_f64[698]=p.p263;
        self.scalar_static_f64[699]=(self.scalar_static_f64[438]*self.scalar_static_f64[698]);
        self.scalar_static_f64[700]=(1.0+self.scalar_static_f64[699]);
        self.scalar_static_f64[701]=(self.scalar_static_f64[697]*self.scalar_static_f64[700]);
        self.scalar_static_f64[702]=(if self.scalar_static_bool[33]{self.scalar_static_f64[701]}else{0.0});
        self.scalar_static_bool[69]=(self.scalar_static_f64[702]>0.0);
        self.scalar_static_f64[703]=(if self.scalar_static_bool[69]{self.scalar_static_f64[702]}else{0.0});
        self.scalar_static_f64[704]=(if self.scalar_static_bool[33]{self.scalar_static_f64[703]}else{self.scalar_static_f64[105]});
        self.scalar_static_f64[705]=p.p264;
        self.scalar_static_f64[706]=(if self.scalar_static_bool[33]{self.scalar_static_f64[705]}else{self.scalar_static_f64[107]});
        self.scalar_static_f64[707]=p.p265;
        self.scalar_static_f64[708]=(if self.scalar_static_bool[33]{self.scalar_static_f64[707]}else{self.scalar_static_f64[109]});
        self.scalar_static_f64[709]=p.p266;
        self.scalar_static_f64[710]=p.p267;
        self.scalar_static_f64[711]=(self.scalar_static_f64[434]*self.scalar_static_f64[710]);
        self.scalar_static_f64[712]=(1.0+self.scalar_static_f64[711]);
        self.scalar_static_f64[713]=(self.scalar_static_f64[709]*self.scalar_static_f64[712]);
        self.scalar_static_f64[714]=p.p268;
        self.scalar_static_f64[715]=(self.scalar_static_f64[436]*self.scalar_static_f64[714]);
        self.scalar_static_f64[716]=(1.0+self.scalar_static_f64[715]);
        self.scalar_static_f64[717]=(self.scalar_static_f64[713]*self.scalar_static_f64[716]);
        self.scalar_static_f64[718]=p.p269;
        self.scalar_static_f64[719]=(self.scalar_static_f64[438]*self.scalar_static_f64[718]);
        self.scalar_static_f64[720]=(1.0+self.scalar_static_f64[719]);
        self.scalar_static_f64[721]=(self.scalar_static_f64[717]*self.scalar_static_f64[720]);
        self.scalar_static_f64[722]=(if self.scalar_static_bool[33]{self.scalar_static_f64[721]}else{self.scalar_static_f64[111]});
        self.scalar_static_f64[723]=p.p270;
        self.scalar_static_f64[724]=(if self.scalar_static_bool[33]{self.scalar_static_f64[723]}else{self.scalar_static_f64[113]});
        self.scalar_static_f64[725]=p.p271;
        self.scalar_static_f64[726]=(if self.scalar_static_bool[33]{self.scalar_static_f64[725]}else{self.scalar_static_f64[115]});
        self.scalar_static_f64[727]=p.p272;
        self.scalar_static_f64[728]=(if self.scalar_static_bool[33]{self.scalar_static_f64[727]}else{self.scalar_static_f64[117]});
        self.scalar_static_f64[729]=p.p273;
        self.scalar_static_f64[730]=(if self.scalar_static_bool[33]{self.scalar_static_f64[729]}else{self.scalar_static_f64[119]});
        self.scalar_static_f64[731]=p.p274;
        self.scalar_static_f64[732]=(if self.scalar_static_bool[33]{self.scalar_static_f64[731]}else{self.scalar_static_f64[121]});
        self.scalar_static_f64[733]=p.p275;
        self.scalar_static_f64[734]=(if self.scalar_static_bool[33]{self.scalar_static_f64[733]}else{self.scalar_static_f64[123]});
        self.scalar_static_f64[735]=p.p276;
        self.scalar_static_f64[736]=(if self.scalar_static_bool[33]{self.scalar_static_f64[735]}else{self.scalar_static_f64[125]});
        self.scalar_static_f64[737]=p.p277;
        self.scalar_static_f64[738]=(if self.scalar_static_bool[33]{self.scalar_static_f64[737]}else{self.scalar_static_f64[127]});
        self.scalar_static_f64[739]=p.p278;
        self.scalar_static_f64[740]=p.p279;
        self.scalar_static_f64[741]=p.p280;
        self.scalar_static_f64[742]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[741]);
        self.scalar_static_f64[743]=(self.scalar_static_f64[740]*self.scalar_static_f64[742]);
        self.scalar_static_f64[744]=(self.scalar_static_f64[739]+self.scalar_static_f64[743]);
        self.scalar_static_f64[745]=p.p281;
        self.scalar_static_f64[746]=(self.scalar_static_f64[436]*self.scalar_static_f64[745]);
        self.scalar_static_f64[747]=(1.0+self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=(self.scalar_static_f64[744]*self.scalar_static_f64[747]);
        self.scalar_static_f64[749]=p.p282;
        self.scalar_static_f64[750]=(self.scalar_static_f64[438]*self.scalar_static_f64[749]);
        self.scalar_static_f64[751]=(1.0+self.scalar_static_f64[750]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[748]*self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(if self.scalar_static_bool[33]{self.scalar_static_f64[752]}else{self.scalar_static_f64[129]});
        self.scalar_static_f64[754]=p.p283;
        self.scalar_static_f64[755]=(if self.scalar_static_bool[33]{self.scalar_static_f64[754]}else{self.scalar_static_f64[131]});
        self.scalar_static_f64[756]=p.p284;
        self.scalar_static_f64[757]=(if self.scalar_static_bool[33]{self.scalar_static_f64[756]}else{self.scalar_static_f64[133]});
        self.scalar_static_f64[758]=p.p285;
        self.scalar_static_f64[759]=(if self.scalar_static_bool[33]{self.scalar_static_f64[758]}else{self.scalar_static_f64[135]});
        self.scalar_static_f64[760]=p.p286;
        self.scalar_static_f64[761]=(self.scalar_static_f64[436]*self.scalar_static_f64[760]);
        self.scalar_static_f64[762]=p.p287;
        self.scalar_static_f64[763]=(self.scalar_static_f64[436]*self.scalar_static_f64[762]);
        self.scalar_static_f64[764]=(1.0+self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=(self.scalar_static_f64[761]*self.scalar_static_f64[764]);
        self.scalar_static_f64[766]=(if self.scalar_static_bool[33]{self.scalar_static_f64[765]}else{0.0});
        self.scalar_static_bool[70]=(self.scalar_static_f64[766]>0.0);
        self.scalar_static_f64[767]=(if self.scalar_static_bool[70]{self.scalar_static_f64[766]}else{0.0});
        self.scalar_static_f64[768]=(if self.scalar_static_bool[33]{self.scalar_static_f64[767]}else{self.scalar_static_f64[137]});
        self.scalar_static_f64[769]=p.p288;
        self.scalar_static_f64[770]=(if self.scalar_static_bool[33]{self.scalar_static_f64[769]}else{self.scalar_static_f64[139]});
        self.scalar_static_f64[771]=p.p289;
        self.scalar_static_f64[772]=(if self.scalar_static_bool[33]{self.scalar_static_f64[771]}else{self.scalar_static_f64[141]});
        self.scalar_static_f64[773]=p.p290;
        self.scalar_static_f64[774]=(if self.scalar_static_bool[33]{self.scalar_static_f64[773]}else{self.scalar_static_f64[143]});
        self.scalar_static_f64[775]=p.p291;
        self.scalar_static_f64[776]=(if self.scalar_static_bool[33]{self.scalar_static_f64[775]}else{self.scalar_static_f64[145]});
        self.scalar_static_f64[777]=p.p292;
        self.scalar_static_f64[778]=(if self.scalar_static_bool[33]{self.scalar_static_f64[777]}else{self.scalar_static_f64[147]});
        self.scalar_static_f64[779]=p.p293;
        self.scalar_static_f64[780]=p.p294;
        self.scalar_static_f64[781]=p.p295;
        self.scalar_static_f64[782]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[781]);
        self.scalar_static_f64[783]=(self.scalar_static_f64[780]*self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=(self.scalar_static_f64[779]+self.scalar_static_f64[783]);
        self.scalar_static_f64[785]=(self.scalar_static_f64[665]*self.scalar_static_f64[784]);
        self.scalar_static_f64[786]=p.p296;
        self.scalar_static_f64[787]=(self.scalar_static_f64[436]*self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=(1.0+self.scalar_static_f64[787]);
        self.scalar_static_f64[789]=(self.scalar_static_f64[785]*self.scalar_static_f64[788]);
        self.scalar_static_f64[790]=p.p297;
        self.scalar_static_f64[791]=(self.scalar_static_f64[438]*self.scalar_static_f64[790]);
        self.scalar_static_f64[792]=(1.0+self.scalar_static_f64[791]);
        self.scalar_static_f64[793]=(self.scalar_static_f64[789]*self.scalar_static_f64[792]);
        self.scalar_static_f64[794]=(if self.scalar_static_bool[33]{self.scalar_static_f64[793]}else{0.0});
        self.scalar_static_bool[71]=(self.scalar_static_f64[794]>0.0);
        self.scalar_static_f64[795]=(if self.scalar_static_bool[71]{self.scalar_static_f64[794]}else{0.0});
        self.scalar_static_f64[796]=(if self.scalar_static_bool[33]{self.scalar_static_f64[795]}else{self.scalar_static_f64[149]});
        self.scalar_static_f64[797]=p.p298;
        self.scalar_static_f64[798]=p.p299;
        self.scalar_static_f64[799]=(self.scalar_static_f64[434]*self.scalar_static_f64[798]);
        self.scalar_static_f64[800]=(1.0+self.scalar_static_f64[799]);
        self.scalar_static_f64[801]=(self.scalar_static_f64[797]*self.scalar_static_f64[800]);
        self.scalar_static_f64[802]=p.p300;
        self.scalar_static_f64[803]=(self.scalar_static_f64[436]*self.scalar_static_f64[802]);
        self.scalar_static_f64[804]=(1.0+self.scalar_static_f64[803]);
        self.scalar_static_f64[805]=(self.scalar_static_f64[801]*self.scalar_static_f64[804]);
        self.scalar_static_f64[806]=p.p301;
        self.scalar_static_f64[807]=(self.scalar_static_f64[438]*self.scalar_static_f64[806]);
        self.scalar_static_f64[808]=(1.0+self.scalar_static_f64[807]);
        self.scalar_static_f64[809]=(self.scalar_static_f64[805]*self.scalar_static_f64[808]);
        self.scalar_static_f64[810]=(if self.scalar_static_bool[33]{self.scalar_static_f64[809]}else{self.scalar_static_f64[151]});
        self.scalar_static_f64[811]=p.p302;
        self.scalar_static_f64[812]=(if self.scalar_static_bool[33]{self.scalar_static_f64[811]}else{self.scalar_static_f64[153]});
        self.scalar_static_f64[813]=p.p303;
        self.scalar_static_f64[814]=(if self.scalar_static_bool[33]{self.scalar_static_f64[813]}else{self.scalar_static_f64[155]});
        self.scalar_static_f64[815]=p.p304;
        self.scalar_static_f64[816]=p.p305;
        self.scalar_static_f64[817]=p.p306;
        self.scalar_static_f64[818]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(self.scalar_static_f64[816]*self.scalar_static_f64[818]);
        self.scalar_static_f64[820]=p.p307;
        self.scalar_static_f64[821]=p.p308;
        self.scalar_static_f64[822]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[821]);
        self.scalar_static_f64[823]=(self.scalar_static_f64[820]*self.scalar_static_f64[822]);
        self.scalar_static_f64[824]=(1.0+self.scalar_static_f64[823]);
        self.scalar_static_f64[825]=(self.scalar_static_f64[819]/self.scalar_static_f64[824]);
        self.scalar_static_f64[826]=(1.0+self.scalar_static_f64[825]);
        self.scalar_static_f64[827]=(self.scalar_static_f64[815]/self.scalar_static_f64[826]);
        self.scalar_static_f64[828]=(if self.scalar_static_bool[33]{self.scalar_static_f64[827]}else{0.0});
        self.scalar_static_bool[72]=(self.scalar_static_f64[828]>1.0);
        self.scalar_static_f64[829]=(if self.scalar_static_bool[72]{self.scalar_static_f64[828]}else{1.0});
        self.scalar_static_bool[73]=(self.scalar_static_f64[829]<16.0);
        self.scalar_static_f64[830]=(if self.scalar_static_bool[73]{self.scalar_static_f64[829]}else{16.0});
        self.scalar_static_f64[831]=(if self.scalar_static_bool[33]{self.scalar_static_f64[830]}else{self.scalar_static_f64[157]});
        self.scalar_static_f64[832]=p.p309;
        self.scalar_static_f64[833]=p.p310;
        self.scalar_static_f64[834]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[833]);
        self.scalar_static_f64[835]=(self.scalar_static_f64[832]*self.scalar_static_f64[834]);
        self.scalar_static_f64[836]=p.p313;
        self.scalar_static_f64[837]=(self.scalar_static_f64[436]*self.scalar_static_f64[836]);
        self.scalar_static_f64[838]=(1.0+self.scalar_static_f64[837]);
        self.scalar_static_f64[839]=(self.scalar_static_f64[835]*self.scalar_static_f64[838]);
        self.scalar_static_f64[840]=p.p311;
        self.scalar_static_f64[841]=p.p312;
        self.scalar_static_f64[842]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[841]);
        self.scalar_static_f64[843]=(self.scalar_static_f64[840]*self.scalar_static_f64[842]);
        self.scalar_static_f64[844]=(1.0+self.scalar_static_f64[843]);
        self.scalar_static_f64[845]=(self.scalar_static_f64[839]/self.scalar_static_f64[844]);
        self.scalar_static_f64[846]=(if self.scalar_static_bool[33]{self.scalar_static_f64[845]}else{0.0});
        self.scalar_static_bool[74]=(self.scalar_static_f64[846]>0.0);
        self.scalar_static_f64[847]=(if self.scalar_static_bool[74]{self.scalar_static_f64[846]}else{0.0});
        self.scalar_static_f64[848]=(if self.scalar_static_bool[33]{self.scalar_static_f64[847]}else{self.scalar_static_f64[159]});
        self.scalar_static_f64[849]=p.p314;
        self.scalar_static_f64[850]=p.p315;
        self.scalar_static_f64[851]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[850]);
        self.scalar_static_f64[852]=(self.scalar_static_f64[849]*self.scalar_static_f64[851]);
        self.scalar_static_f64[853]=p.p318;
        self.scalar_static_f64[854]=(self.scalar_static_f64[436]*self.scalar_static_f64[853]);
        self.scalar_static_f64[855]=(1.0+self.scalar_static_f64[854]);
        self.scalar_static_f64[856]=(self.scalar_static_f64[852]*self.scalar_static_f64[855]);
        self.scalar_static_f64[857]=p.p316;
        self.scalar_static_f64[858]=p.p317;
        self.scalar_static_f64[859]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[858]);
        self.scalar_static_f64[860]=(self.scalar_static_f64[857]*self.scalar_static_f64[859]);
        self.scalar_static_f64[861]=(1.0+self.scalar_static_f64[860]);
        self.scalar_static_f64[862]=(self.scalar_static_f64[856]/self.scalar_static_f64[861]);
        self.scalar_static_f64[863]=(if self.scalar_static_bool[33]{self.scalar_static_f64[862]}else{0.0});
        self.scalar_static_bool[75]=(self.scalar_static_f64[863]>0.0);
        self.scalar_static_f64[864]=(if self.scalar_static_bool[75]{self.scalar_static_f64[863]}else{0.0});
        self.scalar_static_f64[865]=(if self.scalar_static_bool[33]{self.scalar_static_f64[864]}else{self.scalar_static_f64[161]});
        self.scalar_static_f64[866]=p.p319;
        self.scalar_static_f64[867]=(if self.scalar_static_bool[33]{self.scalar_static_f64[866]}else{self.scalar_static_f64[163]});
        self.scalar_static_f64[868]=p.p320;
        self.scalar_static_f64[869]=(if self.scalar_static_bool[33]{self.scalar_static_f64[868]}else{self.scalar_static_f64[165]});
        self.scalar_static_f64[870]=p.p321;
        self.scalar_static_f64[871]=(if self.scalar_static_bool[33]{self.scalar_static_f64[870]}else{self.scalar_static_f64[167]});
        self.scalar_static_f64[872]=p.p322;
        self.scalar_static_f64[873]=(if self.scalar_static_bool[33]{self.scalar_static_f64[872]}else{self.scalar_static_f64[169]});
        self.scalar_static_f64[874]=p.p323;
        self.scalar_static_f64[875]=(self.scalar_static_f64[874]/self.scalar_static_f64[438]);
        self.scalar_static_f64[876]=(if self.scalar_static_bool[33]{self.scalar_static_f64[875]}else{self.scalar_static_f64[171]});
        self.scalar_static_f64[877]=p.p324;
        self.scalar_static_f64[878]=(self.scalar_static_f64[877]/self.scalar_static_f64[436]);
        self.scalar_static_f64[879]=(if self.scalar_static_bool[33]{self.scalar_static_f64[878]}else{self.scalar_static_f64[173]});
        self.scalar_static_f64[880]=p.p325;
        self.scalar_static_f64[881]=(self.scalar_static_f64[880]/self.scalar_static_f64[436]);
        self.scalar_static_f64[882]=(if self.scalar_static_bool[33]{self.scalar_static_f64[881]}else{self.scalar_static_f64[175]});
        self.scalar_static_f64[883]=p.p339;
        self.scalar_static_f64[884]=(self.scalar_static_f64[883]/self.scalar_static_f64[436]);
        self.scalar_static_f64[885]=(if self.scalar_static_bool[33]{self.scalar_static_f64[884]}else{self.scalar_static_f64[177]});
        self.scalar_static_f64[886]=p.p340;
        self.scalar_static_f64[887]=(self.scalar_static_f64[886]/self.scalar_static_f64[436]);
        self.scalar_static_f64[888]=(if self.scalar_static_bool[33]{self.scalar_static_f64[887]}else{self.scalar_static_f64[179]});
        self.scalar_static_f64[889]=p.p326;
        self.scalar_static_f64[890]=(self.scalar_static_f64[889]/self.scalar_static_f64[436]);
        self.scalar_static_f64[891]=(if self.scalar_static_bool[33]{self.scalar_static_f64[890]}else{self.scalar_static_f64[181]});
        self.scalar_static_f64[892]=p.p327;
        self.scalar_static_f64[893]=(self.scalar_static_f64[892]/self.scalar_static_f64[436]);
        self.scalar_static_f64[894]=(if self.scalar_static_bool[33]{self.scalar_static_f64[893]}else{self.scalar_static_f64[183]});
        self.scalar_static_f64[895]=p.p328;
        self.scalar_static_f64[896]=(if self.scalar_static_bool[33]{self.scalar_static_f64[895]}else{self.scalar_static_f64[185]});
        self.scalar_static_f64[897]=p.p342;
        self.scalar_static_f64[898]=(if self.scalar_static_bool[33]{self.scalar_static_f64[897]}else{self.scalar_static_f64[187]});
        self.scalar_static_f64[899]=p.p329;
        self.scalar_static_f64[900]=(if self.scalar_static_bool[33]{self.scalar_static_f64[899]}else{self.scalar_static_f64[189]});
        self.scalar_static_f64[901]=p.p330;
        self.scalar_static_f64[902]=(if self.scalar_static_bool[33]{self.scalar_static_f64[901]}else{self.scalar_static_f64[191]});
        self.scalar_static_f64[903]=p.p331;
        self.scalar_static_f64[904]=(if self.scalar_static_bool[33]{self.scalar_static_f64[903]}else{self.scalar_static_f64[193]});
        self.scalar_static_f64[905]=p.p341;
        self.scalar_static_f64[906]=(if self.scalar_static_bool[33]{self.scalar_static_f64[905]}else{self.scalar_static_f64[195]});
        self.scalar_static_f64[907]=p.p332;
        self.scalar_static_f64[908]=(if self.scalar_static_bool[33]{self.scalar_static_f64[907]}else{self.scalar_static_f64[197]});
        self.scalar_static_f64[909]=p.p333;
        self.scalar_static_f64[910]=(if self.scalar_static_bool[33]{self.scalar_static_f64[909]}else{self.scalar_static_f64[199]});
        self.scalar_static_f64[911]=p.p334;
        self.scalar_static_f64[912]=(if self.scalar_static_bool[33]{self.scalar_static_f64[911]}else{self.scalar_static_f64[201]});
        self.scalar_static_f64[913]=p.p335;
        self.scalar_static_f64[914]=(self.scalar_static_f64[434]*self.scalar_static_f64[913]);
        self.scalar_static_f64[915]=(if self.scalar_static_bool[33]{self.scalar_static_f64[914]}else{self.scalar_static_f64[203]});
        self.scalar_static_f64[916]=p.p336;
        self.scalar_static_f64[917]=(if self.scalar_static_bool[33]{self.scalar_static_f64[916]}else{self.scalar_static_f64[205]});
        self.scalar_static_f64[918]=p.p337;
        self.scalar_static_f64[919]=(if self.scalar_static_bool[33]{self.scalar_static_f64[918]}else{self.scalar_static_f64[207]});
        self.scalar_static_f64[920]=p.p338;
        self.scalar_static_f64[921]=(if self.scalar_static_bool[33]{self.scalar_static_f64[920]}else{self.scalar_static_f64[209]});
        self.scalar_static_f64[922]=p.p343;
        self.scalar_static_f64[923]=p.p345;
        self.scalar_static_f64[924]=(self.scalar_static_f64[923]/self.scalar_static_f64[436]);
        self.scalar_static_f64[925]=(self.scalar_static_f64[922]+self.scalar_static_f64[924]);
        self.scalar_static_f64[926]=(if self.scalar_static_bool[33]{self.scalar_static_f64[925]}else{0.0});
        self.scalar_static_bool[76]=(self.scalar_static_f64[926]>0.0);
        self.scalar_static_f64[927]=(if self.scalar_static_bool[76]{self.scalar_static_f64[926]}else{0.0});
        self.scalar_static_f64[928]=(if self.scalar_static_bool[33]{self.scalar_static_f64[927]}else{self.scalar_static_f64[211]});
        self.scalar_static_f64[929]=p.p344;
        self.scalar_static_f64[930]=p.p346;
        self.scalar_static_f64[931]=(self.scalar_static_f64[930]/self.scalar_static_f64[436]);
        self.scalar_static_f64[932]=(self.scalar_static_f64[929]+self.scalar_static_f64[931]);
        self.scalar_static_f64[933]=(if self.scalar_static_bool[33]{self.scalar_static_f64[932]}else{0.0});
        self.scalar_static_bool[77]=(self.scalar_static_f64[933]>0.0);
        self.scalar_static_f64[934]=(if self.scalar_static_bool[77]{self.scalar_static_f64[933]}else{0.0});
        self.scalar_static_f64[935]=(if self.scalar_static_bool[33]{self.scalar_static_f64[934]}else{self.scalar_static_f64[213]});
        self.scalar_static_f64[936]=p.p347;
        self.scalar_static_f64[937]=(if self.scalar_static_bool[33]{self.scalar_static_f64[936]}else{self.scalar_static_f64[215]});
        self.scalar_static_f64[938]=p.p348;
        self.scalar_static_f64[939]=(if self.scalar_static_bool[33]{self.scalar_static_f64[938]}else{self.scalar_static_f64[217]});
        self.scalar_static_f64[940]=p.p349;
        self.scalar_static_f64[941]=(if self.scalar_static_bool[33]{self.scalar_static_f64[940]}else{self.scalar_static_f64[219]});
        self.scalar_static_f64[942]=p.p350;
        self.scalar_static_f64[943]=(if self.scalar_static_bool[33]{self.scalar_static_f64[942]}else{self.scalar_static_f64[221]});
        self.scalar_static_f64[944]=p.p351;
        self.scalar_static_f64[945]=(if self.scalar_static_bool[33]{self.scalar_static_f64[944]}else{self.scalar_static_f64[223]});
        self.scalar_static_f64[946]=p.p352;
        self.scalar_static_f64[947]=(if self.scalar_static_bool[33]{self.scalar_static_f64[946]}else{self.scalar_static_f64[225]});
        self.scalar_static_f64[948]=p.p353;
        self.scalar_static_f64[949]=p.p355;
        self.scalar_static_f64[950]=(self.scalar_static_f64[434]*self.scalar_static_f64[949]);
        self.scalar_static_f64[951]=(self.scalar_static_f64[948]+self.scalar_static_f64[950]);
        self.scalar_static_f64[952]=(if self.scalar_static_bool[33]{self.scalar_static_f64[951]}else{self.scalar_static_f64[227]});
        self.scalar_static_f64[953]=p.p354;
        self.scalar_static_f64[954]=p.p356;
        self.scalar_static_f64[955]=(self.scalar_static_f64[434]*self.scalar_static_f64[954]);
        self.scalar_static_f64[956]=(self.scalar_static_f64[953]+self.scalar_static_f64[955]);
        self.scalar_static_f64[957]=(if self.scalar_static_bool[33]{self.scalar_static_f64[956]}else{self.scalar_static_f64[229]});
        self.scalar_static_f64[958]=p.p388;
        self.scalar_static_f64[959]=p.p389;
        self.scalar_static_f64[960]=(self.scalar_static_f64[434]*self.scalar_static_f64[959]);
        self.scalar_static_f64[961]=(1.0+self.scalar_static_f64[960]);
        self.scalar_static_f64[962]=(self.scalar_static_f64[958]*self.scalar_static_f64[961]);
        self.scalar_static_f64[963]=p.p390;
        self.scalar_static_f64[964]=(self.scalar_static_f64[436]*self.scalar_static_f64[963]);
        self.scalar_static_f64[965]=(1.0+self.scalar_static_f64[964]);
        self.scalar_static_f64[966]=(self.scalar_static_f64[962]*self.scalar_static_f64[965]);
        self.scalar_static_f64[967]=(if self.scalar_static_bool[33]{self.scalar_static_f64[966]}else{0.0});
        self.scalar_static_bool[78]=(self.scalar_static_f64[967]>0.0);
        self.scalar_static_f64[968]=(if self.scalar_static_bool[78]{self.scalar_static_f64[967]}else{0.0});
        self.scalar_static_f64[969]=(if self.scalar_static_bool[33]{self.scalar_static_f64[968]}else{self.scalar_static_f64[231]});
        self.scalar_static_f64[970]=p.p391;
        self.scalar_static_f64[971]=(if self.scalar_static_bool[33]{self.scalar_static_f64[970]}else{self.scalar_static_f64[233]});
        self.scalar_static_f64[972]=p.p392;
        self.scalar_static_f64[973]=(if self.scalar_static_bool[33]{self.scalar_static_f64[972]}else{self.scalar_static_f64[235]});
        self.scalar_static_f64[974]=p.p393;
        self.scalar_static_f64[975]=p.p394;
        self.scalar_static_f64[976]=(self.scalar_static_f64[434]*self.scalar_static_f64[975]);
        self.scalar_static_f64[977]=(1.0+self.scalar_static_f64[976]);
        self.scalar_static_f64[978]=(self.scalar_static_f64[974]*self.scalar_static_f64[977]);
        self.scalar_static_f64[979]=p.p395;
        self.scalar_static_f64[980]=(self.scalar_static_f64[436]*self.scalar_static_f64[979]);
        self.scalar_static_f64[981]=(1.0+self.scalar_static_f64[980]);
        self.scalar_static_f64[982]=(self.scalar_static_f64[978]*self.scalar_static_f64[981]);
        self.scalar_static_f64[983]=(if self.scalar_static_bool[33]{self.scalar_static_f64[982]}else{0.0});
        self.scalar_static_bool[79]=(self.scalar_static_f64[983]>0.0);
        self.scalar_static_f64[984]=(if self.scalar_static_bool[79]{self.scalar_static_f64[983]}else{0.0});
        self.scalar_static_f64[985]=(if self.scalar_static_bool[33]{self.scalar_static_f64[984]}else{self.scalar_static_f64[237]});
        self.scalar_static_f64[986]=p.p357;
        self.scalar_static_f64[987]=(2.0*self.scalar_static_f64[986]);
        self.scalar_static_f64[988]=p.p358;
        self.scalar_static_f64[989]=(self.scalar_static_f64[424]*self.scalar_static_f64[988]);
        self.scalar_static_f64[990]=(self.scalar_static_f64[987]+self.scalar_static_f64[989]);
        self.scalar_static_f64[991]=(if self.scalar_static_bool[33]{self.scalar_static_f64[990]}else{0.0});
        self.scalar_static_f64[992]=p.p359;
        self.scalar_static_f64[993]=(if self.scalar_static_bool[33]{self.scalar_static_f64[992]}else{self.scalar_static_f64[239]});
        self.scalar_static_f64[994]=p.p361;
        self.scalar_static_f64[995]=p.p362;
        self.scalar_static_f64[996]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[995]);
        self.scalar_static_f64[997]=(self.scalar_static_f64[994]*self.scalar_static_f64[996]);
        self.scalar_static_f64[998]=p.p360;
        self.scalar_static_f64[999]=p.p363;
        self.scalar_static_f64[1000]=(self.scalar_static_f64[436]*self.scalar_static_f64[999]);
        self.scalar_static_f64[1001]=p.p364;
        self.scalar_static_f64[1002]=(self.scalar_static_f64[438]*self.scalar_static_f64[1001]);
        self.scalar_static_f64[1003]=p.p365;
        self.scalar_static_f64[1004]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1003]}else{self.scalar_static_f64[243]});
        self.scalar_static_f64[1005]=p.p366;
        self.scalar_static_f64[1006]=p.p367;
        self.scalar_static_f64[1007]=(self.scalar_static_f64[434]*self.scalar_static_f64[1006]);
        self.scalar_static_f64[1008]=(1.0+self.scalar_static_f64[1007]);
        self.scalar_static_f64[1009]=(self.scalar_static_f64[1005]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1010]=p.p368;
        self.scalar_static_f64[1011]=(self.scalar_static_f64[436]*self.scalar_static_f64[1010]);
        self.scalar_static_f64[1012]=(1.0+self.scalar_static_f64[1011]);
        self.scalar_static_f64[1013]=(self.scalar_static_f64[1009]*self.scalar_static_f64[1012]);
        self.scalar_static_f64[1014]=p.p369;
        self.scalar_static_f64[1015]=(self.scalar_static_f64[438]*self.scalar_static_f64[1014]);
        self.scalar_static_f64[1016]=(1.0+self.scalar_static_f64[1015]);
        self.scalar_static_f64[1017]=(self.scalar_static_f64[1013]*self.scalar_static_f64[1016]);
        self.scalar_static_f64[1018]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1017]}else{self.scalar_static_f64[245]});
        self.scalar_static_f64[1019]=p.p370;
        self.scalar_static_f64[1020]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1019]}else{self.scalar_static_f64[247]});
        self.scalar_static_f64[1021]=p.p371;
        self.scalar_static_f64[1022]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1021]}else{self.scalar_static_f64[249]});
        self.scalar_static_f64[1023]=p.p372;
        self.scalar_static_f64[1024]=(2.0*self.scalar_static_f64[1023]);
        self.scalar_static_f64[1025]=p.p373;
        self.scalar_static_f64[1026]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[1025]);
        self.scalar_static_f64[1027]=(self.scalar_static_f64[1024]*self.scalar_static_f64[1026]);
        self.scalar_static_f64[1028]=p.p374;
        self.scalar_static_f64[1029]=(self.scalar_static_f64[436]*self.scalar_static_f64[1028]);
        self.scalar_static_f64[1030]=(1.0+self.scalar_static_f64[1029]);
        self.scalar_static_f64[1031]=(self.scalar_static_f64[1027]*self.scalar_static_f64[1030]);
        self.scalar_static_f64[1032]=p.p375;
        self.scalar_static_f64[1033]=p.p377;
        self.scalar_static_f64[1034]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[1033]);
        self.scalar_static_f64[1035]=p.p378;
        self.scalar_static_f64[1036]=(self.scalar_static_f64[436]*self.scalar_static_f64[1035]);
        self.scalar_static_f64[1037]=(1.0+self.scalar_static_f64[1036]);
        self.scalar_static_f64[1038]=(self.scalar_static_f64[1034]*self.scalar_static_f64[1037]);
        self.scalar_static_f64[1039]=p.p376;
        self.scalar_static_f64[1040]=p.p379;
        self.scalar_static_f64[1041]=p.p380;
        self.scalar_static_f64[1042]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1041]}else{self.scalar_static_f64[265]});
        self.scalar_static_f64[1043]=p.p381;
        self.scalar_static_f64[1044]=p.p382;
        self.scalar_static_f64[1045]=(self.scalar_static_f64[1043]*self.scalar_static_f64[1044]);
        self.scalar_static_f64[1046]=(self.scalar_static_f64[1045]/self.scalar_static_f64[418]);
        self.scalar_static_f64[1047]=(self.scalar_static_f64[595]/self.scalar_static_f64[1044]);
        self.scalar_static_f64[1048]=(self.scalar_static_f64[1047]).exp();
        self.scalar_static_f64[1049]=(1.0-self.scalar_static_f64[1048]);
        self.scalar_static_f64[1050]=(self.scalar_static_f64[1046]*self.scalar_static_f64[1049]);
        self.scalar_static_f64[1051]=(1.0+self.scalar_static_f64[1050]);
        self.scalar_static_f64[1052]=(self.scalar_static_f64[662]*self.scalar_static_f64[991]);
        self.scalar_static_f64[1053]=p.p383;
        self.scalar_static_f64[1054]=(self.scalar_static_f64[436]*self.scalar_static_f64[1053]);
        self.scalar_static_f64[1055]=(1.0+self.scalar_static_f64[1054]);
        self.scalar_static_f64[1056]=p.p384;
        self.scalar_static_f64[1057]=p.p385;
        self.scalar_static_f64[1058]=(self.scalar_static_f64[434]*self.scalar_static_f64[1057]);
        self.scalar_static_f64[1059]=(self.scalar_static_f64[1056]+self.scalar_static_f64[1058]);
        self.scalar_static_f64[1060]=p.p386;
        self.scalar_static_f64[1061]=(self.scalar_static_f64[436]*self.scalar_static_f64[1060]);
        self.scalar_static_f64[1062]=(self.scalar_static_f64[1059]+self.scalar_static_f64[1061]);
        self.scalar_static_f64[1063]=p.p387;
        self.scalar_static_f64[1064]=(self.scalar_static_f64[434]*self.scalar_static_f64[1063]);
        self.scalar_static_f64[1065]=(self.scalar_static_f64[436]*self.scalar_static_f64[1064]);
        self.scalar_static_f64[1066]=(self.scalar_static_f64[1062]+self.scalar_static_f64[1065]);
        self.scalar_static_f64[1067]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1066]}else{self.scalar_static_f64[269]});
        self.scalar_static_f64[1068]=(self.scalar_static_f64[428]*self.scalar_static_f64[432]);
        self.scalar_static_f64[1069]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1068]}else{self.scalar_static_f64[271]});
        self.scalar_static_f64[1070]=p.p396;
        self.scalar_static_f64[1071]=p.p397;
        self.scalar_static_f64[1072]=p.p398;
        self.scalar_static_f64[1073]=(1000000.0*self.scalar_static_f64[1072]);
        self.scalar_static_f64[1074]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1073]}else{self.scalar_static_f64[276]});
        self.scalar_static_f64[1075]=p.p399;
        self.scalar_static_f64[1076]=(self.scalar_static_f64[432]*self.scalar_static_f64[1075]);
        self.scalar_static_f64[1077]=(self.scalar_static_f64[1076]/self.scalar_static_f64[387]);
        self.scalar_static_f64[1078]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1077]}else{self.scalar_static_f64[278]});
        self.scalar_static_f64[1079]=p.p400;
        self.scalar_static_f64[1080]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1079]}else{self.scalar_static_f64[280]});
        self.scalar_static_f64[1081]=(if self.scalar_static_bool[33]{self.scalar_static_f64[557]}else{self.scalar_static_f64[302]});
        self.scalar_static_f64[1082]=(if self.scalar_static_bool[33]{self.scalar_static_f64[562]}else{self.scalar_static_f64[306]});
        self.scalar_static_f64[1083]=(if self.scalar_static_bool[33]{self.scalar_static_f64[794]}else{0.0});
        self.scalar_static_f64[1084]=(if self.scalar_static_bool[33]{self.scalar_static_f64[796]}else{self.scalar_static_f64[318]});
        self.scalar_static_f64[1085]=(if self.scalar_static_bool[33]{self.scalar_static_f64[831]}else{self.scalar_static_f64[322]});
        self.scalar_static_f64[1086]=(if self.scalar_static_bool[33]{self.scalar_static_f64[848]}else{self.scalar_static_f64[326]});
        self.scalar_static_bool[80]=(self.scalar_static_bool[17]&&self.scalar_static_bool[33]);
        self.scalar_static_f64[1087]=(if self.scalar_static_bool[80]{self.scalar_static_f64[495]}else{0.0});
        self.scalar_static_f64[1088]=if param_given[401] { 1.0 } else { 0.0 };
        self.scalar_static_bool[81]=(1.0==self.scalar_static_f64[1088]);
        self.scalar_static_bool[82]=(self.scalar_static_bool[80]&&self.scalar_static_bool[81]);
        self.scalar_static_f64[1089]=p.p401;
        self.scalar_static_f64[1090]=(if self.scalar_static_bool[82]{self.scalar_static_f64[1089]}else{self.scalar_static_f64[1087]});
        self.scalar_static_f64[1091]=(if self.scalar_static_bool[80]{self.scalar_static_f64[485]}else{0.0});
        self.scalar_static_f64[1092]=if param_given[402] { 1.0 } else { 0.0 };
        self.scalar_static_bool[83]=(1.0==self.scalar_static_f64[1092]);
        self.scalar_static_bool[84]=(self.scalar_static_bool[80]&&self.scalar_static_bool[83]);
        self.scalar_static_f64[1093]=p.p402;
        self.scalar_static_f64[1094]=(if self.scalar_static_bool[84]{self.scalar_static_f64[1093]}else{self.scalar_static_f64[1091]});
        self.scalar_static_f64[1095]=(if self.scalar_static_bool[80]{self.scalar_static_f64[486]}else{0.0});
        self.scalar_static_f64[1096]=if param_given[403] { 1.0 } else { 0.0 };
        self.scalar_static_bool[85]=(1.0==self.scalar_static_f64[1096]);
        self.scalar_static_bool[86]=(self.scalar_static_bool[80]&&self.scalar_static_bool[85]);
        self.scalar_static_f64[1097]=p.p403;
        self.scalar_static_f64[1098]=(if self.scalar_static_bool[86]{self.scalar_static_f64[1097]}else{self.scalar_static_f64[1095]});
        self.scalar_static_f64[1099]=(if self.scalar_static_bool[80]{self.scalar_static_f64[496]}else{0.0});
        self.scalar_static_f64[1100]=if param_given[406] { 1.0 } else { 0.0 };
        self.scalar_static_bool[87]=(1.0==self.scalar_static_f64[1100]);
        self.scalar_static_bool[88]=(self.scalar_static_bool[80]&&self.scalar_static_bool[87]);
        self.scalar_static_f64[1101]=p.p406;
        self.scalar_static_f64[1102]=(if self.scalar_static_bool[88]{self.scalar_static_f64[1101]}else{self.scalar_static_f64[1099]});
        self.scalar_static_f64[1103]=(if self.scalar_static_bool[80]{self.scalar_static_f64[498]}else{0.0});
        self.scalar_static_f64[1104]=if param_given[407] { 1.0 } else { 0.0 };
        self.scalar_static_bool[89]=(1.0==self.scalar_static_f64[1104]);
        self.scalar_static_bool[90]=(self.scalar_static_bool[80]&&self.scalar_static_bool[89]);
        self.scalar_static_f64[1105]=p.p407;
        self.scalar_static_f64[1106]=(if self.scalar_static_bool[90]{self.scalar_static_f64[1105]}else{self.scalar_static_f64[1103]});
        self.scalar_static_f64[1107]=(if self.scalar_static_bool[80]{self.scalar_static_f64[489]}else{0.0});
        self.scalar_static_f64[1108]=if param_given[404] { 1.0 } else { 0.0 };
        self.scalar_static_bool[91]=(1.0==self.scalar_static_f64[1108]);
        self.scalar_static_bool[92]=(self.scalar_static_bool[80]&&self.scalar_static_bool[91]);
        self.scalar_static_f64[1109]=p.p404;
        self.scalar_static_f64[1110]=(if self.scalar_static_bool[92]{self.scalar_static_f64[1109]}else{self.scalar_static_f64[1107]});
        self.scalar_static_f64[1111]=(if self.scalar_static_bool[80]{self.scalar_static_f64[490]}else{0.0});
        self.scalar_static_f64[1112]=if param_given[405] { 1.0 } else { 0.0 };
        self.scalar_static_bool[93]=(1.0==self.scalar_static_f64[1112]);
        self.scalar_static_bool[94]=(self.scalar_static_bool[80]&&self.scalar_static_bool[93]);
        self.scalar_static_f64[1113]=p.p405;
        self.scalar_static_f64[1114]=(if self.scalar_static_bool[94]{self.scalar_static_f64[1113]}else{self.scalar_static_f64[1111]});
        self.scalar_static_f64[1115]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1098]);
        self.scalar_static_f64[1116]=(self.scalar_static_f64[1094]*self.scalar_static_f64[1115]);
        self.scalar_static_f64[1117]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1114]);
        self.scalar_static_f64[1118]=(self.scalar_static_f64[1110]*self.scalar_static_f64[1117]);
        self.scalar_static_f64[1119]=(1.0+self.scalar_static_f64[1118]);
        self.scalar_static_f64[1120]=(self.scalar_static_f64[1116]/self.scalar_static_f64[1119]);
        self.scalar_static_f64[1121]=(self.scalar_static_f64[436]*self.scalar_static_f64[1102]);
        self.scalar_static_f64[1122]=(self.scalar_static_f64[438]*self.scalar_static_f64[1106]);
        self.scalar_static_f64[1123]=(if self.scalar_static_bool[80]{self.scalar_static_f64[500]}else{0.0});
        self.scalar_static_f64[1124]=if param_given[408] { 1.0 } else { 0.0 };
        self.scalar_static_bool[95]=(1.0==self.scalar_static_f64[1124]);
        self.scalar_static_bool[96]=(self.scalar_static_bool[80]&&self.scalar_static_bool[95]);
        self.scalar_static_f64[1125]=p.p408;
        self.scalar_static_f64[1126]=(if self.scalar_static_bool[96]{self.scalar_static_f64[1125]}else{self.scalar_static_f64[1123]});
        self.scalar_static_f64[1127]=(if self.scalar_static_bool[80]{self.scalar_static_f64[501]}else{0.0});
        self.scalar_static_f64[1128]=if param_given[409] { 1.0 } else { 0.0 };
        self.scalar_static_bool[97]=(1.0==self.scalar_static_f64[1128]);
        self.scalar_static_bool[98]=(self.scalar_static_bool[80]&&self.scalar_static_bool[97]);
        self.scalar_static_f64[1129]=p.p409;
        self.scalar_static_f64[1130]=(if self.scalar_static_bool[98]{self.scalar_static_f64[1129]}else{self.scalar_static_f64[1127]});
        self.scalar_static_f64[1131]=(self.scalar_static_f64[459]*self.scalar_static_f64[1130]);
        self.scalar_static_f64[1132]=(self.scalar_static_f64[1131]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1133]=(if self.scalar_static_bool[80]{self.scalar_static_f64[545]}else{0.0});
        self.scalar_static_f64[1134]=if param_given[410] { 1.0 } else { 0.0 };
        self.scalar_static_bool[99]=(1.0==self.scalar_static_f64[1134]);
        self.scalar_static_bool[100]=(self.scalar_static_bool[80]&&self.scalar_static_bool[99]);
        self.scalar_static_f64[1135]=p.p410;
        self.scalar_static_f64[1136]=(if self.scalar_static_bool[100]{self.scalar_static_f64[1135]}else{self.scalar_static_f64[1133]});
        self.scalar_static_f64[1137]=(if self.scalar_static_bool[80]{self.scalar_static_f64[547]}else{0.0});
        self.scalar_static_f64[1138]=if param_given[411] { 1.0 } else { 0.0 };
        self.scalar_static_bool[101]=(1.0==self.scalar_static_f64[1138]);
        self.scalar_static_bool[102]=(self.scalar_static_bool[80]&&self.scalar_static_bool[101]);
        self.scalar_static_f64[1139]=p.p411;
        self.scalar_static_f64[1140]=(if self.scalar_static_bool[102]{self.scalar_static_f64[1139]}else{self.scalar_static_f64[1137]});
        self.scalar_static_f64[1141]=(if self.scalar_static_bool[80]{self.scalar_static_f64[550]}else{0.0});
        self.scalar_static_f64[1142]=if param_given[412] { 1.0 } else { 0.0 };
        self.scalar_static_bool[103]=(1.0==self.scalar_static_f64[1142]);
        self.scalar_static_bool[104]=(self.scalar_static_bool[80]&&self.scalar_static_bool[103]);
        self.scalar_static_f64[1143]=p.p412;
        self.scalar_static_f64[1144]=(if self.scalar_static_bool[104]{self.scalar_static_f64[1143]}else{self.scalar_static_f64[1141]});
        self.scalar_static_f64[1145]=(2.0*self.scalar_static_f64[1136]);
        self.scalar_static_f64[1146]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[1140]);
        self.scalar_static_f64[1147]=(self.scalar_static_f64[1145]*self.scalar_static_f64[1146]);
        self.scalar_static_f64[1148]=(self.scalar_static_f64[436]*self.scalar_static_f64[1144]);
        self.scalar_static_f64[1149]=(1.0+self.scalar_static_f64[1148]);
        self.scalar_static_f64[1150]=(self.scalar_static_f64[1147]*self.scalar_static_f64[1149]);
        self.scalar_static_f64[1151]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1150]}else{0.0});
        self.scalar_static_bool[105]=(self.scalar_static_f64[1151]>0.0);
        self.scalar_static_f64[1152]=(if self.scalar_static_bool[105]{self.scalar_static_f64[1151]}else{0.0});
        self.scalar_static_bool[106]=(self.scalar_static_f64[1152]<5.0);
        self.scalar_static_f64[1153]=(if self.scalar_static_bool[106]{self.scalar_static_f64[1152]}else{5.0});
        self.scalar_static_f64[1154]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1153]}else{self.scalar_static_f64[1081]});
        self.scalar_static_f64[1155]=(self.scalar_static_f64[558]*self.scalar_static_f64[1154]);
        self.scalar_static_f64[1156]=(self.scalar_static_f64[459]*self.scalar_static_f64[1155]);
        self.scalar_static_f64[1157]=(self.scalar_static_f64[1156]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1158]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1157]}else{self.scalar_static_f64[1082]});
        self.scalar_static_f64[1159]=(if self.scalar_static_bool[80]{self.scalar_static_f64[580]}else{0.0});
        self.scalar_static_f64[1160]=if param_given[413] { 1.0 } else { 0.0 };
        self.scalar_static_bool[107]=(1.0==self.scalar_static_f64[1160]);
        self.scalar_static_bool[108]=(self.scalar_static_bool[80]&&self.scalar_static_bool[107]);
        self.scalar_static_f64[1161]=p.p413;
        self.scalar_static_f64[1162]=(if self.scalar_static_bool[108]{self.scalar_static_f64[1161]}else{self.scalar_static_f64[1159]});
        self.scalar_static_f64[1163]=(if self.scalar_static_bool[80]{self.scalar_static_f64[574]}else{0.0});
        self.scalar_static_f64[1164]=if param_given[414] { 1.0 } else { 0.0 };
        self.scalar_static_bool[109]=(1.0==self.scalar_static_f64[1164]);
        self.scalar_static_bool[110]=(self.scalar_static_bool[80]&&self.scalar_static_bool[109]);
        self.scalar_static_f64[1165]=p.p414;
        self.scalar_static_f64[1166]=(if self.scalar_static_bool[110]{self.scalar_static_f64[1165]}else{self.scalar_static_f64[1163]});
        self.scalar_static_f64[1167]=(if self.scalar_static_bool[80]{self.scalar_static_f64[576]}else{0.0});
        self.scalar_static_f64[1168]=if param_given[415] { 1.0 } else { 0.0 };
        self.scalar_static_bool[111]=(1.0==self.scalar_static_f64[1168]);
        self.scalar_static_bool[112]=(self.scalar_static_bool[80]&&self.scalar_static_bool[111]);
        self.scalar_static_f64[1169]=p.p415;
        self.scalar_static_f64[1170]=(if self.scalar_static_bool[112]{self.scalar_static_f64[1169]}else{self.scalar_static_f64[1167]});
        self.scalar_static_f64[1171]=f64::powf(self.scalar_static_f64[544],self.scalar_static_f64[1166]);
        self.scalar_static_f64[1172]=(self.scalar_static_f64[436]*self.scalar_static_f64[1170]);
        self.scalar_static_f64[1173]=(1.0+self.scalar_static_f64[1172]);
        self.scalar_static_f64[1174]=(self.scalar_static_f64[1171]*self.scalar_static_f64[1173]);
        self.scalar_static_f64[1175]=(if self.scalar_static_bool[80]{self.scalar_static_f64[779]}else{0.0});
        self.scalar_static_f64[1176]=if param_given[416] { 1.0 } else { 0.0 };
        self.scalar_static_bool[113]=(1.0==self.scalar_static_f64[1176]);
        self.scalar_static_bool[114]=(self.scalar_static_bool[80]&&self.scalar_static_bool[113]);
        self.scalar_static_f64[1177]=p.p416;
        self.scalar_static_f64[1178]=(if self.scalar_static_bool[114]{self.scalar_static_f64[1177]}else{self.scalar_static_f64[1175]});
        self.scalar_static_f64[1179]=(if self.scalar_static_bool[80]{self.scalar_static_f64[780]}else{0.0});
        self.scalar_static_f64[1180]=if param_given[417] { 1.0 } else { 0.0 };
        self.scalar_static_bool[115]=(1.0==self.scalar_static_f64[1180]);
        self.scalar_static_bool[116]=(self.scalar_static_bool[80]&&self.scalar_static_bool[115]);
        self.scalar_static_f64[1181]=p.p417;
        self.scalar_static_f64[1182]=(if self.scalar_static_bool[116]{self.scalar_static_f64[1181]}else{self.scalar_static_f64[1179]});
        self.scalar_static_f64[1183]=(if self.scalar_static_bool[80]{self.scalar_static_f64[781]}else{0.0});
        self.scalar_static_f64[1184]=if param_given[418] { 1.0 } else { 0.0 };
        self.scalar_static_bool[117]=(1.0==self.scalar_static_f64[1184]);
        self.scalar_static_bool[118]=(self.scalar_static_bool[80]&&self.scalar_static_bool[117]);
        self.scalar_static_f64[1185]=p.p418;
        self.scalar_static_f64[1186]=(if self.scalar_static_bool[118]{self.scalar_static_f64[1185]}else{self.scalar_static_f64[1183]});
        self.scalar_static_f64[1187]=(if self.scalar_static_bool[80]{self.scalar_static_f64[786]}else{0.0});
        self.scalar_static_f64[1188]=if param_given[419] { 1.0 } else { 0.0 };
        self.scalar_static_bool[119]=(1.0==self.scalar_static_f64[1188]);
        self.scalar_static_bool[120]=(self.scalar_static_bool[80]&&self.scalar_static_bool[119]);
        self.scalar_static_f64[1189]=p.p419;
        self.scalar_static_f64[1190]=(if self.scalar_static_bool[120]{self.scalar_static_f64[1189]}else{self.scalar_static_f64[1187]});
        self.scalar_static_f64[1191]=(if self.scalar_static_bool[80]{self.scalar_static_f64[790]}else{0.0});
        self.scalar_static_f64[1192]=if param_given[420] { 1.0 } else { 0.0 };
        self.scalar_static_bool[121]=(1.0==self.scalar_static_f64[1192]);
        self.scalar_static_bool[122]=(self.scalar_static_bool[80]&&self.scalar_static_bool[121]);
        self.scalar_static_f64[1193]=p.p420;
        self.scalar_static_f64[1194]=(if self.scalar_static_bool[122]{self.scalar_static_f64[1193]}else{self.scalar_static_f64[1191]});
        self.scalar_static_f64[1195]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1186]);
        self.scalar_static_f64[1196]=(self.scalar_static_f64[1182]*self.scalar_static_f64[1195]);
        self.scalar_static_f64[1197]=(self.scalar_static_f64[1178]+self.scalar_static_f64[1196]);
        self.scalar_static_f64[1198]=(self.scalar_static_f64[665]*self.scalar_static_f64[1197]);
        self.scalar_static_f64[1199]=(self.scalar_static_f64[436]*self.scalar_static_f64[1190]);
        self.scalar_static_f64[1200]=(1.0+self.scalar_static_f64[1199]);
        self.scalar_static_f64[1201]=(self.scalar_static_f64[1198]*self.scalar_static_f64[1200]);
        self.scalar_static_f64[1202]=(self.scalar_static_f64[438]*self.scalar_static_f64[1194]);
        self.scalar_static_f64[1203]=(1.0+self.scalar_static_f64[1202]);
        self.scalar_static_f64[1204]=(self.scalar_static_f64[1201]*self.scalar_static_f64[1203]);
        self.scalar_static_f64[1205]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1204]}else{self.scalar_static_f64[1083]});
        self.scalar_static_bool[123]=(self.scalar_static_f64[1205]>0.0);
        self.scalar_static_f64[1206]=(if self.scalar_static_bool[123]{self.scalar_static_f64[1205]}else{0.0});
        self.scalar_static_f64[1207]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1206]}else{self.scalar_static_f64[1084]});
        self.scalar_static_f64[1208]=(if self.scalar_static_bool[80]{self.scalar_static_f64[815]}else{0.0});
        self.scalar_static_f64[1209]=if param_given[421] { 1.0 } else { 0.0 };
        self.scalar_static_bool[124]=(1.0==self.scalar_static_f64[1209]);
        self.scalar_static_bool[125]=(self.scalar_static_bool[80]&&self.scalar_static_bool[124]);
        self.scalar_static_f64[1210]=p.p421;
        self.scalar_static_f64[1211]=(if self.scalar_static_bool[125]{self.scalar_static_f64[1210]}else{self.scalar_static_f64[1208]});
        self.scalar_static_f64[1212]=(if self.scalar_static_bool[80]{self.scalar_static_f64[816]}else{0.0});
        self.scalar_static_f64[1213]=if param_given[422] { 1.0 } else { 0.0 };
        self.scalar_static_bool[126]=(1.0==self.scalar_static_f64[1213]);
        self.scalar_static_bool[127]=(self.scalar_static_bool[80]&&self.scalar_static_bool[126]);
        self.scalar_static_f64[1214]=p.p422;
        self.scalar_static_f64[1215]=(if self.scalar_static_bool[127]{self.scalar_static_f64[1214]}else{self.scalar_static_f64[1212]});
        self.scalar_static_f64[1216]=(if self.scalar_static_bool[80]{self.scalar_static_f64[817]}else{0.0});
        self.scalar_static_f64[1217]=if param_given[423] { 1.0 } else { 0.0 };
        self.scalar_static_bool[128]=(1.0==self.scalar_static_f64[1217]);
        self.scalar_static_bool[129]=(self.scalar_static_bool[80]&&self.scalar_static_bool[128]);
        self.scalar_static_f64[1218]=p.p423;
        self.scalar_static_f64[1219]=(if self.scalar_static_bool[129]{self.scalar_static_f64[1218]}else{self.scalar_static_f64[1216]});
        self.scalar_static_f64[1220]=(if self.scalar_static_bool[80]{self.scalar_static_f64[820]}else{0.0});
        self.scalar_static_f64[1221]=if param_given[424] { 1.0 } else { 0.0 };
        self.scalar_static_bool[130]=(1.0==self.scalar_static_f64[1221]);
        self.scalar_static_bool[131]=(self.scalar_static_bool[80]&&self.scalar_static_bool[130]);
        self.scalar_static_f64[1222]=p.p424;
        self.scalar_static_f64[1223]=(if self.scalar_static_bool[131]{self.scalar_static_f64[1222]}else{self.scalar_static_f64[1220]});
        self.scalar_static_f64[1224]=(if self.scalar_static_bool[80]{self.scalar_static_f64[821]}else{0.0});
        self.scalar_static_f64[1225]=if param_given[425] { 1.0 } else { 0.0 };
        self.scalar_static_bool[132]=(1.0==self.scalar_static_f64[1225]);
        self.scalar_static_bool[133]=(self.scalar_static_bool[80]&&self.scalar_static_bool[132]);
        self.scalar_static_f64[1226]=p.p425;
        self.scalar_static_f64[1227]=(if self.scalar_static_bool[133]{self.scalar_static_f64[1226]}else{self.scalar_static_f64[1224]});
        self.scalar_static_f64[1228]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1219]);
        self.scalar_static_f64[1229]=(self.scalar_static_f64[1215]*self.scalar_static_f64[1228]);
        self.scalar_static_f64[1230]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1227]);
        self.scalar_static_f64[1231]=(self.scalar_static_f64[1223]*self.scalar_static_f64[1230]);
        self.scalar_static_f64[1232]=(1.0+self.scalar_static_f64[1231]);
        self.scalar_static_f64[1233]=(self.scalar_static_f64[1229]/self.scalar_static_f64[1232]);
        self.scalar_static_f64[1234]=(1.0+self.scalar_static_f64[1233]);
        self.scalar_static_f64[1235]=(self.scalar_static_f64[1211]/self.scalar_static_f64[1234]);
        self.scalar_static_f64[1236]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1235]}else{0.0});
        self.scalar_static_bool[134]=(self.scalar_static_f64[1236]>1.0);
        self.scalar_static_f64[1237]=(if self.scalar_static_bool[134]{self.scalar_static_f64[1236]}else{1.0});
        self.scalar_static_bool[135]=(self.scalar_static_f64[1237]<16.0);
        self.scalar_static_f64[1238]=(if self.scalar_static_bool[135]{self.scalar_static_f64[1237]}else{16.0});
        self.scalar_static_f64[1239]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1238]}else{self.scalar_static_f64[1085]});
        self.scalar_static_f64[1240]=(if self.scalar_static_bool[80]{self.scalar_static_f64[832]}else{0.0});
        self.scalar_static_f64[1241]=if param_given[426] { 1.0 } else { 0.0 };
        self.scalar_static_bool[136]=(1.0==self.scalar_static_f64[1241]);
        self.scalar_static_bool[137]=(self.scalar_static_bool[80]&&self.scalar_static_bool[136]);
        self.scalar_static_f64[1242]=p.p426;
        self.scalar_static_f64[1243]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1242]}else{self.scalar_static_f64[1240]});
        self.scalar_static_f64[1244]=(if self.scalar_static_bool[80]{self.scalar_static_f64[833]}else{0.0});
        self.scalar_static_f64[1245]=if param_given[427] { 1.0 } else { 0.0 };
        self.scalar_static_bool[138]=(1.0==self.scalar_static_f64[1245]);
        self.scalar_static_bool[139]=(self.scalar_static_bool[80]&&self.scalar_static_bool[138]);
        self.scalar_static_f64[1246]=p.p427;
        self.scalar_static_f64[1247]=(if self.scalar_static_bool[139]{self.scalar_static_f64[1246]}else{self.scalar_static_f64[1244]});
        self.scalar_static_f64[1248]=(if self.scalar_static_bool[80]{self.scalar_static_f64[840]}else{0.0});
        self.scalar_static_f64[1249]=if param_given[428] { 1.0 } else { 0.0 };
        self.scalar_static_bool[140]=(1.0==self.scalar_static_f64[1249]);
        self.scalar_static_bool[141]=(self.scalar_static_bool[80]&&self.scalar_static_bool[140]);
        self.scalar_static_f64[1250]=p.p428;
        self.scalar_static_f64[1251]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1250]}else{self.scalar_static_f64[1248]});
        self.scalar_static_f64[1252]=(if self.scalar_static_bool[80]{self.scalar_static_f64[841]}else{0.0});
        self.scalar_static_f64[1253]=if param_given[429] { 1.0 } else { 0.0 };
        self.scalar_static_bool[142]=(1.0==self.scalar_static_f64[1253]);
        self.scalar_static_bool[143]=(self.scalar_static_bool[80]&&self.scalar_static_bool[142]);
        self.scalar_static_f64[1254]=p.p429;
        self.scalar_static_f64[1255]=(if self.scalar_static_bool[143]{self.scalar_static_f64[1254]}else{self.scalar_static_f64[1252]});
        self.scalar_static_f64[1256]=(if self.scalar_static_bool[80]{self.scalar_static_f64[836]}else{0.0});
        self.scalar_static_f64[1257]=if param_given[430] { 1.0 } else { 0.0 };
        self.scalar_static_bool[144]=(1.0==self.scalar_static_f64[1257]);
        self.scalar_static_bool[145]=(self.scalar_static_bool[80]&&self.scalar_static_bool[144]);
        self.scalar_static_f64[1258]=p.p430;
        self.scalar_static_f64[1259]=(if self.scalar_static_bool[145]{self.scalar_static_f64[1258]}else{self.scalar_static_f64[1256]});
        self.scalar_static_f64[1260]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1247]);
        self.scalar_static_f64[1261]=(self.scalar_static_f64[1243]*self.scalar_static_f64[1260]);
        self.scalar_static_f64[1262]=(self.scalar_static_f64[436]*self.scalar_static_f64[1259]);
        self.scalar_static_f64[1263]=(1.0+self.scalar_static_f64[1262]);
        self.scalar_static_f64[1264]=(self.scalar_static_f64[1261]*self.scalar_static_f64[1263]);
        self.scalar_static_f64[1265]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1255]);
        self.scalar_static_f64[1266]=(self.scalar_static_f64[1251]*self.scalar_static_f64[1265]);
        self.scalar_static_f64[1267]=(1.0+self.scalar_static_f64[1266]);
        self.scalar_static_f64[1268]=(self.scalar_static_f64[1264]/self.scalar_static_f64[1267]);
        self.scalar_static_f64[1269]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1268]}else{0.0});
        self.scalar_static_bool[146]=(self.scalar_static_f64[1269]>0.0);
        self.scalar_static_f64[1270]=(if self.scalar_static_bool[146]{self.scalar_static_f64[1269]}else{0.0});
        self.scalar_static_f64[1271]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1270]}else{self.scalar_static_f64[1086]});
        self.scalar_static_f64[1272]=(3.45313e-11/self.scalar_static_f64[453]);
        self.scalar_static_f64[1273]=(self.scalar_static_f64[432]*self.scalar_static_f64[1272]);
        self.scalar_static_f64[1274]=p.p431;
        self.scalar_static_f64[1275]=p.p432;
        self.scalar_static_f64[1276]=p.p433;
        self.scalar_static_f64[1277]=p.p434;
        self.scalar_static_f64[1278]=(self.scalar_static_f64[387]*self.scalar_static_f64[1277]);
        self.scalar_static_f64[1279]=(self.scalar_static_f64[1278]/self.scalar_static_f64[432]);
        self.scalar_static_f64[1280]=(1.0+self.scalar_static_f64[1279]);
        self.scalar_static_bool[147]=(self.scalar_static_f64[1280]>0.001);
        self.scalar_static_f64[1281]=(if self.scalar_static_bool[147]{self.scalar_static_f64[1280]}else{0.001});
        self.scalar_static_f64[1282]=(self.scalar_static_f64[1276]/self.scalar_static_f64[1281]);
        self.scalar_static_f64[1283]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1282]}else{self.scalar_static_f64[332]});
        self.scalar_static_f64[1284]=p.p435;
        self.scalar_static_f64[1285]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1284]}else{self.scalar_static_f64[334]});
        self.scalar_static_f64[1286]=p.p436;
        self.scalar_static_f64[1287]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1286]}else{self.scalar_static_f64[336]});
        self.scalar_static_f64[1288]=p.p437;
        self.scalar_static_f64[1289]=p.p439;
        self.scalar_static_f64[1290]=p.p438;
        self.scalar_static_f64[1291]=p.p440;
        self.scalar_static_f64[1292]=p.p441;
        self.scalar_static_f64[1293]=(self.scalar_static_f64[537]*self.scalar_static_f64[1292]);
        self.scalar_static_f64[1294]=(self.scalar_static_f64[455]*self.scalar_static_f64[1293]);
        self.scalar_static_f64[1295]=(self.scalar_static_f64[424]*self.scalar_static_f64[1294]);
        self.scalar_static_f64[1296]=(self.scalar_static_f64[1295]/self.scalar_static_f64[418]);
        self.scalar_static_f64[1297]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1296]}else{self.scalar_static_f64[342]});
        self.scalar_static_f64[1298]=p.p442;
        self.scalar_static_f64[1299]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1298]}else{self.scalar_static_f64[344]});
        self.scalar_static_f64[1300]=p.p444;
        self.scalar_static_f64[1301]=p.p445;
        self.scalar_static_f64[1302]=p.p446;
        self.scalar_static_f64[1303]=(if self.scalar_static_bool[33]{0.0}else{self.scalar_static_f64[603]});
        self.scalar_static_bool[148]=(self.scalar_static_f64[370]>1.0);
        self.scalar_static_f64[1304]=p.p28;
        self.scalar_static_bool[149]=(self.scalar_static_f64[1304]>0.0);
        self.scalar_static_bool[150]=(self.scalar_static_bool[148]&&self.scalar_static_bool[149]);
        self.scalar_static_bool[151]=(self.scalar_static_bool[33]&&self.scalar_static_bool[150]);
        self.scalar_static_f64[1305]=(self.scalar_static_f64[388]+self.scalar_static_f64[1304]);
        self.scalar_static_f64[1306]=(-self.scalar_static_f64[1305]);
        self.scalar_static_f64[1307]=p.p449;
        self.scalar_static_f64[1308]=(self.scalar_static_f64[1306]/self.scalar_static_f64[1307]);
        self.scalar_static_f64[1309]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1308]}else{self.scalar_static_f64[616]});
        self.scalar_static_f64[1310]=(self.scalar_static_f64[1309]).abs();
        self.scalar_static_bool[152]=(self.scalar_static_f64[1310]<80.0);
        self.scalar_static_bool[153]=(self.scalar_static_bool[151]&&self.scalar_static_bool[152]);
        self.scalar_static_f64[1311]=(self.scalar_static_f64[1309]).exp();
        self.scalar_static_f64[1312]=(if self.scalar_static_bool[153]{self.scalar_static_f64[1311]}else{self.scalar_static_f64[619]});
        self.scalar_static_bool[154]=(self.scalar_static_f64[1309]< -80.0);
        self.scalar_static_bool[155]=(!self.scalar_static_bool[152]);
        self.scalar_static_bool[156]=(self.scalar_static_bool[151]&&self.scalar_static_bool[155]);
        self.scalar_static_bool[157]=(self.scalar_static_bool[154]&&self.scalar_static_bool[156]);
        self.scalar_static_f64[1313]=(-self.scalar_static_f64[1309]);
        self.scalar_static_f64[1314]=(self.scalar_static_f64[1313]-80.0);
        self.scalar_static_f64[1315]=(0.5*self.scalar_static_f64[1314]);
        self.scalar_static_f64[1316]=(0.3333333333333*self.scalar_static_f64[1314]);
        self.scalar_static_f64[1317]=(1.0+self.scalar_static_f64[1316]);
        self.scalar_static_f64[1318]=(self.scalar_static_f64[1315]*self.scalar_static_f64[1317]);
        self.scalar_static_f64[1319]=(1.0+self.scalar_static_f64[1318]);
        self.scalar_static_f64[1320]=(self.scalar_static_f64[1314]*self.scalar_static_f64[1319]);
        self.scalar_static_f64[1321]=(1.0+self.scalar_static_f64[1320]);
        self.scalar_static_f64[1322]=(1.80485e-35/self.scalar_static_f64[1321]);
        self.scalar_static_f64[1323]=(if self.scalar_static_bool[157]{self.scalar_static_f64[1322]}else{self.scalar_static_f64[1312]});
        self.scalar_static_bool[158]=(!self.scalar_static_bool[154]);
        self.scalar_static_bool[159]=(self.scalar_static_bool[156]&&self.scalar_static_bool[158]);
        self.scalar_static_f64[1324]=(self.scalar_static_f64[1309]-80.0);
        self.scalar_static_f64[1325]=(0.5*self.scalar_static_f64[1324]);
        self.scalar_static_f64[1326]=(0.3333333333333*self.scalar_static_f64[1324]);
        self.scalar_static_f64[1327]=(1.0+self.scalar_static_f64[1326]);
        self.scalar_static_f64[1328]=(self.scalar_static_f64[1325]*self.scalar_static_f64[1327]);
        self.scalar_static_f64[1329]=(1.0+self.scalar_static_f64[1328]);
        self.scalar_static_f64[1330]=(self.scalar_static_f64[1324]*self.scalar_static_f64[1329]);
        self.scalar_static_f64[1331]=(1.0+self.scalar_static_f64[1330]);
        self.scalar_static_f64[1332]=(5.54062e34*self.scalar_static_f64[1331]);
        self.scalar_static_f64[1333]=(if self.scalar_static_bool[159]{self.scalar_static_f64[1332]}else{self.scalar_static_f64[1323]});
        self.scalar_static_f64[1334]=(1.0-self.scalar_static_f64[1333]);
        self.scalar_static_f64[1335]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1334]}else{self.scalar_static_f64[632]});
        self.scalar_static_f64[1336]=p.p450;
        self.scalar_static_f64[1337]=(2.0*self.scalar_static_f64[1336]);
        self.scalar_static_f64[1338]=(self.scalar_static_f64[1333]*self.scalar_static_f64[1337]);
        self.scalar_static_f64[1339]=f64::powf(self.scalar_static_f64[1333],self.scalar_static_f64[370]);
        self.scalar_static_f64[1340]=(1.0-self.scalar_static_f64[1339]);
        self.scalar_static_f64[1341]=(self.scalar_static_f64[1340]/self.scalar_static_f64[370]);
        self.scalar_static_f64[1342]=(self.scalar_static_f64[1335]-self.scalar_static_f64[1341]);
        self.scalar_static_f64[1343]=(self.scalar_static_f64[1338]*self.scalar_static_f64[1342]);
        self.scalar_static_f64[1344]=(self.scalar_static_f64[1335]*self.scalar_static_f64[1335]);
        self.scalar_static_f64[1345]=(self.scalar_static_f64[1343]/self.scalar_static_f64[1344]);
        self.scalar_static_f64[1346]=(if self.scalar_static_bool[151]{self.scalar_static_f64[1345]}else{self.scalar_static_f64[1303]});
        self.scalar_static_f64[1347]=(1.0+self.scalar_static_f64[1346]);
        self.scalar_static_f64[1348]=p.p447;
        self.scalar_static_f64[1349]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1348]}else{self.scalar_static_f64[347]});
        self.scalar_static_f64[1350]=p.p448;
        self.scalar_static_f64[1351]=p.p451;
        self.scalar_static_f64[1352]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1351]}else{self.scalar_static_f64[351]});
        self.scalar_static_f64[1353]=p.p452;
        self.scalar_static_f64[1354]=(self.scalar_static_f64[668]*self.scalar_static_f64[1353]);
        self.scalar_static_f64[1355]=(self.scalar_static_f64[668]*self.scalar_static_f64[1354]);
        self.scalar_static_f64[1356]=(self.scalar_static_f64[436]*self.scalar_static_f64[1355]);
        self.scalar_static_f64[1357]=(self.scalar_static_f64[436]*self.scalar_static_f64[1356]);
        self.scalar_static_f64[1358]=p.p453;
        self.scalar_static_f64[1359]=(self.scalar_static_f64[1358]-2.0);
        self.scalar_static_f64[1360]=f64::powf(self.scalar_static_f64[434],self.scalar_static_f64[1359]);
        self.scalar_static_f64[1361]=(self.scalar_static_f64[1357]*self.scalar_static_f64[1360]);
        self.scalar_static_f64[1362]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1361]}else{self.scalar_static_f64[353]});
        self.scalar_static_f64[1363]=p.p489;
        self.scalar_static_f64[1364]=p.p490;
        self.scalar_static_f64[1365]=(self.scalar_static_f64[434]*self.scalar_static_f64[1364]);
        self.scalar_static_f64[1366]=(self.scalar_static_f64[1363]+self.scalar_static_f64[1365]);
        self.scalar_static_f64[1367]=p.p491;
        self.scalar_static_f64[1368]=p.p492;
        self.scalar_static_f64[1369]=(self.scalar_static_f64[434]*self.scalar_static_f64[1368]);
        self.scalar_static_f64[1370]=(self.scalar_static_f64[1367]+self.scalar_static_f64[1369]);
        self.scalar_static_f64[1371]=p.p493;
        self.scalar_static_f64[1372]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1371]}else{self.scalar_static_f64[359]});
        self.scalar_static_f64[1373]=p.p494;
        self.scalar_static_f64[1374]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1373]}else{self.scalar_static_f64[361]});
        self.scalar_static_f64[1375]=p.p498;
        self.scalar_static_f64[1376]=(self.scalar_static_f64[446]*0.3333333333333);
        self.scalar_static_f64[1377]=p.p37;
        self.scalar_static_f64[1378]=(self.scalar_static_f64[1376]/self.scalar_static_f64[1377]);
        self.scalar_static_f64[1379]=(self.scalar_static_f64[451]+self.scalar_static_f64[1378]);
        self.scalar_static_f64[1380]=(self.scalar_static_f64[1375]*self.scalar_static_f64[1379]);
        self.scalar_static_f64[1381]=(self.scalar_static_f64[445]*self.scalar_static_f64[1377]);
        self.scalar_static_f64[1382]=(self.scalar_static_f64[1380]/self.scalar_static_f64[1381]);
        self.scalar_static_f64[1383]=p.p496;
        self.scalar_static_f64[1384]=p.p497;
        self.scalar_static_f64[1385]=(self.scalar_static_f64[1383]+self.scalar_static_f64[1384]);
        self.scalar_static_f64[1386]=(self.scalar_static_f64[441]*self.scalar_static_f64[446]);
        self.scalar_static_f64[1387]=(self.scalar_static_f64[1385]/self.scalar_static_f64[1386]);
        self.scalar_static_f64[1388]=(self.scalar_static_f64[1382]+self.scalar_static_f64[1387]);
        self.scalar_static_f64[1389]=p.p495;
        self.scalar_static_f64[1390]=(self.scalar_static_f64[370]*self.scalar_static_f64[1389]);
        self.scalar_static_f64[1391]=(self.scalar_static_f64[1388]+self.scalar_static_f64[1390]);
        self.scalar_static_f64[1392]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1391]}else{0.0});
        self.scalar_static_bool[160]=(self.scalar_static_f64[1392]>0.0);
        self.scalar_static_f64[1393]=(if self.scalar_static_bool[160]{self.scalar_static_f64[1392]}else{0.0});
        self.scalar_static_f64[1394]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1393]}else{self.scalar_static_f64[363]});
        self.scalar_static_f64[1395]=p.p500;
        self.scalar_static_bool[161]=(self.scalar_static_f64[1395]>0.0);
        self.scalar_static_f64[1396]=(if self.scalar_static_bool[161]{self.scalar_static_f64[1395]}else{0.0});
        self.scalar_static_f64[1397]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1396]}else{0.0});
        self.scalar_static_f64[1398]=p.p501;
        self.scalar_static_bool[162]=(self.scalar_static_f64[1398]>0.0);
        self.scalar_static_f64[1399]=(if self.scalar_static_bool[162]{self.scalar_static_f64[1398]}else{0.0});
        self.scalar_static_f64[1400]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1399]}else{0.0});
        self.scalar_static_f64[1401]=p.p7;
        self.scalar_static_bool[163]=(0.0==self.scalar_static_f64[1401]);
        self.scalar_static_bool[164]=(self.scalar_static_bool[33]&&self.scalar_static_bool[163]);
        self.scalar_static_f64[1402]=(if self.scalar_static_bool[164]{self.scalar_static_f64[1397]}else{self.scalar_static_f64[1400]});
        self.scalar_static_f64[1403]=p.p39;
        self.scalar_static_f64[1404]=(self.scalar_static_f64[370]*self.scalar_static_f64[1403]);
        self.scalar_static_f64[1405]=(self.scalar_static_f64[1397]*self.scalar_static_f64[1404]);
        self.scalar_static_f64[1406]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1405]}else{self.scalar_static_f64[365]});
        self.scalar_static_f64[1407]=p.p40;
        self.scalar_static_f64[1408]=(self.scalar_static_f64[370]*self.scalar_static_f64[1407]);
        self.scalar_static_f64[1409]=(self.scalar_static_f64[1402]*self.scalar_static_f64[1408]);
        self.scalar_static_f64[1410]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1409]}else{self.scalar_static_f64[367]});
        self.scalar_static_f64[1411]=p.p502;
        self.scalar_static_f64[1412]=(self.scalar_static_f64[370]*self.scalar_static_f64[1411]);
        self.scalar_static_f64[1413]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1412]}else{self.scalar_static_f64[369]});
        self.scalar_static_f64[1414]=p.p461;
        self.scalar_static_bool[165]=(self.scalar_static_f64[1414]>0.0);
        self.scalar_static_f64[1415]=p.p26;
        self.scalar_static_bool[166]=(self.scalar_static_f64[1415]>0.0);
        self.scalar_static_bool[167]=(self.scalar_static_bool[165]&&self.scalar_static_bool[166]);
        self.scalar_static_f64[1416]=p.p27;
        self.scalar_static_bool[168]=(self.scalar_static_f64[1416]>0.0);
        self.scalar_static_bool[169]=(self.scalar_static_bool[167]&&self.scalar_static_bool[168]);
        self.scalar_static_bool[170]=(1.0==self.scalar_static_f64[370]);
        self.scalar_static_bool[171]=(self.scalar_static_bool[150]||self.scalar_static_bool[170]);
        self.scalar_static_bool[172]=(self.scalar_static_bool[169]&&self.scalar_static_bool[171]);
        self.scalar_static_bool[173]=(1.0==self.scalar_static_f64[1414]);
        self.scalar_static_bool[174]=(self.scalar_static_bool[33]&&self.scalar_static_bool[172]);
        self.scalar_static_bool[175]=(self.scalar_static_bool[173]&&self.scalar_static_bool[174]);
        self.scalar_static_f64[1417]=p.p462;
        self.scalar_static_f64[1418]=(0.5*self.scalar_static_f64[388]);
        self.scalar_static_f64[1419]=(self.scalar_static_f64[1417]+self.scalar_static_f64[1418]);
        self.scalar_static_f64[1420]=p.p463;
        self.scalar_static_f64[1421]=(self.scalar_static_f64[1418]+self.scalar_static_f64[1420]);
        self.scalar_static_f64[1422]=(if self.scalar_static_bool[175]{self.scalar_static_f64[439]}else{0.0});
        self.scalar_static_f64[1423]=p.p464;
        self.scalar_static_f64[1424]=(self.scalar_static_f64[419]+self.scalar_static_f64[1423]);
        self.scalar_static_bool[176]=(self.scalar_static_f64[1424]>1e-9);
        self.scalar_static_f64[1425]=(if self.scalar_static_bool[176]{self.scalar_static_f64[1424]}else{1e-9});
        self.scalar_static_f64[1426]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1425]}else{0.0});
        self.scalar_static_f64[1427]=p.p471;
        self.scalar_static_f64[1428]=f64::powf(self.scalar_static_f64[1422],self.scalar_static_f64[1427]);
        self.scalar_static_f64[1429]=(1.0/self.scalar_static_f64[1428]);
        self.scalar_static_f64[1430]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1429]}else{0.0});
        self.scalar_static_f64[1431]=p.p472;
        self.scalar_static_f64[1432]=f64::powf(self.scalar_static_f64[1426],self.scalar_static_f64[1431]);
        self.scalar_static_f64[1433]=(1.0/self.scalar_static_f64[1432]);
        self.scalar_static_f64[1434]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1433]}else{0.0});
        self.scalar_static_f64[1435]=p.p477;
        self.scalar_static_f64[1436]=f64::powf(self.scalar_static_f64[1422],self.scalar_static_f64[1435]);
        self.scalar_static_f64[1437]=(1.0/self.scalar_static_f64[1436]);
        self.scalar_static_f64[1438]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1437]}else{self.scalar_static_f64[1430]});
        self.scalar_static_f64[1439]=p.p478;
        self.scalar_static_f64[1440]=f64::powf(self.scalar_static_f64[1426],self.scalar_static_f64[1439]);
        self.scalar_static_f64[1441]=(1.0/self.scalar_static_f64[1440]);
        self.scalar_static_f64[1442]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1441]}else{self.scalar_static_f64[1434]});
        self.scalar_static_f64[1443]=p.p474;
        self.scalar_static_f64[1444]=(self.scalar_static_f64[1438]*self.scalar_static_f64[1443]);
        self.scalar_static_f64[1445]=(1.0+self.scalar_static_f64[1444]);
        self.scalar_static_f64[1446]=p.p475;
        self.scalar_static_f64[1447]=(self.scalar_static_f64[1442]*self.scalar_static_f64[1446]);
        self.scalar_static_f64[1448]=(self.scalar_static_f64[1445]+self.scalar_static_f64[1447]);
        self.scalar_static_f64[1449]=p.p476;
        self.scalar_static_f64[1450]=(self.scalar_static_f64[1438]*self.scalar_static_f64[1449]);
        self.scalar_static_f64[1451]=(self.scalar_static_f64[1442]*self.scalar_static_f64[1450]);
        self.scalar_static_f64[1452]=(self.scalar_static_f64[1448]+self.scalar_static_f64[1451]);
        self.scalar_static_bool[177]=(self.scalar_static_f64[1452]>1e-20);
        self.scalar_static_f64[1453]=(if self.scalar_static_bool[177]{self.scalar_static_f64[1452]}else{1e-20});
        self.scalar_static_f64[1454]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1453]}else{0.0});
        self.scalar_static_bool[178]=(self.scalar_static_f64[667]>1e-10);
        self.scalar_static_f64[1455]=(if self.scalar_static_bool[178]{self.scalar_static_f64[667]}else{1e-10});
        self.scalar_static_f64[1456]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1455]}else{self.scalar_static_f64[670]});
        self.scalar_static_f64[1457]=(self.scalar_static_f64[671]*self.scalar_static_f64[1456]);
        self.scalar_static_f64[1458]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1457]}else{self.scalar_static_f64[673]});
        self.scalar_static_f64[1459]=(self.scalar_static_f64[794]*self.scalar_static_f64[1370]);
        self.scalar_static_f64[1460]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1459]}else{self.scalar_static_f64[794]});
        self.scalar_static_bool[179]=(self.scalar_static_f64[1460]>0.0);
        self.scalar_static_f64[1461]=(if self.scalar_static_bool[179]{self.scalar_static_f64[1460]}else{0.0});
        self.scalar_static_f64[1462]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1461]}else{self.scalar_static_f64[796]});
        self.scalar_static_f64[1463]=(self.scalar_static_f64[1205]*self.scalar_static_f64[1370]);
        self.scalar_static_f64[1464]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1463]}else{self.scalar_static_f64[1205]});
        self.scalar_static_bool[180]=(self.scalar_static_f64[1464]>0.0);
        self.scalar_static_f64[1465]=(if self.scalar_static_bool[180]{self.scalar_static_f64[1464]}else{0.0});
        self.scalar_static_f64[1466]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1465]}else{self.scalar_static_f64[1207]});
        self.scalar_static_f64[1467]=p.p473;
        self.scalar_static_f64[1468]=(self.scalar_static_f64[523]*self.scalar_static_f64[1467]);
        self.scalar_static_f64[1469]=(self.scalar_static_f64[1468]/self.scalar_static_f64[1454]);
        self.scalar_static_f64[1470]=p.p479;
        self.scalar_static_f64[1471]=(self.scalar_static_f64[523]*self.scalar_static_f64[1470]);
        self.scalar_static_f64[1472]=p.p480;
        self.scalar_static_f64[1473]=f64::powf(self.scalar_static_f64[1454],self.scalar_static_f64[1472]);
        self.scalar_static_f64[1474]=(self.scalar_static_f64[1471]/self.scalar_static_f64[1473]);
        self.scalar_static_f64[1475]=(self.scalar_static_f64[459]*self.scalar_static_f64[581]);
        self.scalar_static_f64[1476]=(self.scalar_static_f64[1475]/self.scalar_static_f64[453]);
        self.scalar_static_bool[181]=(!self.scalar_static_bool[173]);
        self.scalar_static_bool[182]=(self.scalar_static_bool[174]&&self.scalar_static_bool[181]);
        self.scalar_static_f64[1477]=p.p482;
        self.scalar_static_f64[1478]=(-1.0/self.scalar_static_f64[1477]);
        self.scalar_static_f64[1479]=(-self.scalar_static_f64[1419]);
        self.scalar_static_f64[1480]=p.p481;
        self.scalar_static_f64[1481]=(self.scalar_static_f64[1479]/self.scalar_static_f64[1480]);
        self.scalar_static_bool[183]=(self.scalar_static_f64[1481]> -80.0);
        self.scalar_static_bool[184]=(self.scalar_static_bool[182]&&self.scalar_static_bool[183]);
        self.scalar_static_f64[1482]=(self.scalar_static_f64[1481]).exp();
        self.scalar_static_f64[1483]=(if self.scalar_static_bool[184]{self.scalar_static_f64[1482]}else{0.0});
        self.scalar_static_f64[1484]=(if self.scalar_static_bool[184]{self.scalar_static_f64[1482]}else{self.scalar_static_f64[1483]});
        self.scalar_static_bool[185]=(!self.scalar_static_bool[183]);
        self.scalar_static_bool[186]=(self.scalar_static_bool[182]&&self.scalar_static_bool[185]);
        self.scalar_static_f64[1485]=(-self.scalar_static_f64[1481]);
        self.scalar_static_f64[1486]=(self.scalar_static_f64[1485]-80.0);
        self.scalar_static_f64[1487]=(0.5*self.scalar_static_f64[1486]);
        self.scalar_static_f64[1488]=(0.3333333333333*self.scalar_static_f64[1486]);
        self.scalar_static_f64[1489]=(1.0+self.scalar_static_f64[1488]);
        self.scalar_static_f64[1490]=(self.scalar_static_f64[1487]*self.scalar_static_f64[1489]);
        self.scalar_static_f64[1491]=(1.0+self.scalar_static_f64[1490]);
        self.scalar_static_f64[1492]=(self.scalar_static_f64[1486]*self.scalar_static_f64[1491]);
        self.scalar_static_f64[1493]=(1.0+self.scalar_static_f64[1492]);
        self.scalar_static_f64[1494]=(1.80485e-35/self.scalar_static_f64[1493]);
        self.scalar_static_f64[1495]=(if self.scalar_static_bool[186]{self.scalar_static_f64[1494]}else{self.scalar_static_f64[1484]});
        self.scalar_static_f64[1496]=(-self.scalar_static_f64[1421]);
        self.scalar_static_f64[1497]=(self.scalar_static_f64[1496]/self.scalar_static_f64[1480]);
        self.scalar_static_bool[187]=(self.scalar_static_f64[1497]> -80.0);
        self.scalar_static_bool[188]=(self.scalar_static_bool[182]&&self.scalar_static_bool[187]);
        self.scalar_static_f64[1498]=(self.scalar_static_f64[1497]).exp();
        self.scalar_static_f64[1499]=(if self.scalar_static_bool[188]{self.scalar_static_f64[1498]}else{0.0});
        self.scalar_static_f64[1500]=(if self.scalar_static_bool[188]{self.scalar_static_f64[1498]}else{self.scalar_static_f64[1499]});
        self.scalar_static_bool[189]=(!self.scalar_static_bool[187]);
        self.scalar_static_bool[190]=(self.scalar_static_bool[182]&&self.scalar_static_bool[189]);
        self.scalar_static_f64[1501]=(-self.scalar_static_f64[1497]);
        self.scalar_static_f64[1502]=(self.scalar_static_f64[1501]-80.0);
        self.scalar_static_f64[1503]=(0.5*self.scalar_static_f64[1502]);
        self.scalar_static_f64[1504]=(0.3333333333333*self.scalar_static_f64[1502]);
        self.scalar_static_f64[1505]=(1.0+self.scalar_static_f64[1504]);
        self.scalar_static_f64[1506]=(self.scalar_static_f64[1503]*self.scalar_static_f64[1505]);
        self.scalar_static_f64[1507]=(1.0+self.scalar_static_f64[1506]);
        self.scalar_static_f64[1508]=(self.scalar_static_f64[1502]*self.scalar_static_f64[1507]);
        self.scalar_static_f64[1509]=(1.0+self.scalar_static_f64[1508]);
        self.scalar_static_f64[1510]=(1.80485e-35/self.scalar_static_f64[1509]);
        self.scalar_static_f64[1511]=(if self.scalar_static_bool[190]{self.scalar_static_f64[1510]}else{self.scalar_static_f64[1500]});
        self.scalar_static_f64[1512]=(1.0-self.scalar_static_f64[1495]);
        self.scalar_static_f64[1513]=(-self.scalar_static_f64[1477]);
        self.scalar_static_f64[1514]=f64::powf(self.scalar_static_f64[1512],self.scalar_static_f64[1513]);
        self.scalar_static_f64[1515]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1514]}else{0.0});
        self.scalar_static_f64[1516]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1514]}else{self.scalar_static_f64[1515]});
        self.scalar_static_f64[1517]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1425]}else{self.scalar_static_f64[1426]});
        self.scalar_static_f64[1518]=p.p484;
        self.scalar_static_f64[1519]=(self.scalar_static_f64[1517]*self.scalar_static_f64[1518]);
        self.scalar_static_f64[1520]=(self.scalar_static_f64[1519]/self.scalar_static_f64[387]);
        self.scalar_static_f64[1521]=(1.0+self.scalar_static_f64[1520]);
        self.scalar_static_bool[191]=(self.scalar_static_f64[1521]>1e-20);
        self.scalar_static_f64[1522]=(if self.scalar_static_bool[191]{self.scalar_static_f64[1521]}else{1e-20});
        self.scalar_static_f64[1523]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1522]}else{self.scalar_static_f64[1454]});
        self.scalar_static_f64[1524]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1455]}else{self.scalar_static_f64[1456]});
        self.scalar_static_f64[1525]=(self.scalar_static_f64[671]*self.scalar_static_f64[1524]);
        self.scalar_static_f64[1526]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1525]}else{self.scalar_static_f64[1458]});
        self.scalar_static_f64[1527]=(self.scalar_static_f64[1460]*self.scalar_static_f64[1478]);
        self.scalar_static_f64[1528]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1527]}else{self.scalar_static_f64[1460]});
        self.scalar_static_bool[192]=(self.scalar_static_f64[1528]>0.0);
        self.scalar_static_f64[1529]=(if self.scalar_static_bool[192]{self.scalar_static_f64[1528]}else{0.0});
        self.scalar_static_f64[1530]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1529]}else{self.scalar_static_f64[1462]});
        self.scalar_static_f64[1531]=(self.scalar_static_f64[1464]*self.scalar_static_f64[1478]);
        self.scalar_static_f64[1532]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1531]}else{self.scalar_static_f64[1464]});
        self.scalar_static_bool[193]=(self.scalar_static_f64[1532]>0.0);
        self.scalar_static_f64[1533]=(if self.scalar_static_bool[193]{self.scalar_static_f64[1532]}else{0.0});
        self.scalar_static_f64[1534]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1533]}else{self.scalar_static_f64[1466]});
        self.scalar_static_f64[1535]=p.p483;
        self.scalar_static_f64[1536]=(self.scalar_static_f64[523]*self.scalar_static_f64[1535]);
        self.scalar_static_f64[1537]=(self.scalar_static_f64[1536]/self.scalar_static_f64[1523]);
        self.scalar_static_f64[1538]=p.p485;
        self.scalar_static_f64[1539]=(self.scalar_static_f64[523]*self.scalar_static_f64[1538]);
        self.scalar_static_f64[1540]=(self.scalar_static_f64[575]*self.scalar_static_f64[1539]);
        self.scalar_static_f64[1541]=(self.scalar_static_f64[578]*self.scalar_static_f64[1540]);
        self.scalar_static_f64[1542]=(if self.scalar_static_bool[163]{self.scalar_static_f64[481]}else{self.scalar_static_f64[484]});
        self.scalar_static_f64[1543]=(if self.scalar_static_bool[163]{self.scalar_static_f64[879]}else{self.scalar_static_f64[882]});
        self.scalar_static_f64[1544]=(if self.scalar_static_bool[163]{self.scalar_static_f64[885]}else{self.scalar_static_f64[888]});
        self.scalar_static_f64[1545]=(if self.scalar_static_bool[163]{self.scalar_static_f64[891]}else{self.scalar_static_f64[894]});
        self.scalar_static_f64[1546]=(if self.scalar_static_bool[163]{self.scalar_static_f64[928]}else{self.scalar_static_f64[935]});
        self.scalar_static_f64[1547]=(if self.scalar_static_bool[163]{self.scalar_static_f64[937]}else{self.scalar_static_f64[939]});
        self.scalar_static_f64[1548]=(if self.scalar_static_bool[163]{self.scalar_static_f64[941]}else{self.scalar_static_f64[943]});
        self.scalar_static_f64[1549]=(if self.scalar_static_bool[163]{self.scalar_static_f64[945]}else{self.scalar_static_f64[947]});
        self.scalar_static_f64[1550]=(if self.scalar_static_bool[163]{self.scalar_static_f64[952]}else{self.scalar_static_f64[957]});
        self.scalar_static_f64[1551]=(self.scalar_static_f64[532]*1.04479e-10);
        self.scalar_static_f64[1552]=(self.scalar_static_f64[535]+self.scalar_static_f64[1551]);
        self.scalar_static_f64[1553]=(self.scalar_static_f64[532]* -0.4);
        self.scalar_static_f64[1554]=(10.0*self.scalar_static_f64[457]);
        self.scalar_static_f64[1555]=(self.scalar_static_f64[1554]).sqrt();
        self.scalar_static_f64[1556]=(1.0+self.scalar_static_f64[1555]);
        self.scalar_static_f64[1557]=(1.0/self.scalar_static_f64[1556]);
        self.scalar_static_f64[1558]=(self.scalar_static_f64[457]*0.05);
        self.scalar_static_f64[1559]=(self.scalar_static_f64[466]*1.602176565e-19);
        self.scalar_static_f64[1560]=(0.5*self.scalar_static_f64[1559]);
        self.scalar_static_f64[1561]=(self.scalar_static_f64[455]*self.scalar_static_f64[1560]);
        self.scalar_static_f64[1562]=(self.scalar_static_f64[1561]/3.45313e-11);
        self.scalar_static_bool[194]=(self.scalar_static_f64[462]>0.0);
        self.scalar_static_f64[1563]=p.p13;
        self.scalar_static_f64[1564]=(4e-10*self.scalar_static_f64[1563]);
        self.scalar_static_f64[1565]=(self.scalar_static_f64[453]+self.scalar_static_f64[1564]);
        self.scalar_static_f64[1566]=(self.scalar_static_f64[1562]*self.scalar_static_f64[1565]);
        self.scalar_static_f64[1567]=(if self.scalar_static_bool[194]{self.scalar_static_f64[1566]}else{0.0});
        self.scalar_static_f64[1568]=(self.scalar_static_f64[459]+self.scalar_static_f64[1564]);
        self.scalar_static_f64[1569]=(self.scalar_static_f64[1562]*self.scalar_static_f64[1568]);
        self.scalar_static_f64[1570]=(if self.scalar_static_bool[194]{self.scalar_static_f64[1569]}else{0.0});
        self.scalar_static_bool[195]=(!self.scalar_static_bool[194]);
        self.scalar_static_f64[1571]=(-self.scalar_static_f64[1562]);
        self.scalar_static_f64[1572]=(self.scalar_static_f64[1565]*self.scalar_static_f64[1571]);
        self.scalar_static_f64[1573]=(if self.scalar_static_bool[195]{self.scalar_static_f64[1572]}else{self.scalar_static_f64[1567]});
        self.scalar_static_f64[1574]=(self.scalar_static_f64[1568]*self.scalar_static_f64[1571]);
        self.scalar_static_f64[1575]=(if self.scalar_static_bool[195]{self.scalar_static_f64[1574]}else{self.scalar_static_f64[1570]});
        self.scalar_static_f64[1576]=(3.45313e-11/self.scalar_static_f64[459]);
        self.scalar_static_bool[196]=(self.scalar_static_f64[573]>0.0);
        self.scalar_static_f64[1577]=(1.0+self.scalar_static_f64[573]);
        self.scalar_static_f64[1578]=(self.scalar_static_f64[1272]*self.scalar_static_f64[1577]);
        self.scalar_static_f64[1579]=(if self.scalar_static_bool[196]{self.scalar_static_f64[1578]}else{0.0});
        self.scalar_static_f64[1580]=(if self.scalar_static_bool[196]{self.scalar_static_f64[1576]}else{0.0});
        self.scalar_static_bool[197]=(!self.scalar_static_bool[196]);
        self.scalar_static_f64[1581]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1272]}else{self.scalar_static_f64[1579]});
        self.scalar_static_f64[1582]=(1.0-self.scalar_static_f64[573]);
        self.scalar_static_f64[1583]=(self.scalar_static_f64[1576]*self.scalar_static_f64[1582]);
        self.scalar_static_f64[1584]=(if self.scalar_static_bool[197]{self.scalar_static_f64[1583]}else{self.scalar_static_f64[1580]});
        self.scalar_static_f64[1585]=(self.scalar_static_f64[1552]/self.scalar_static_f64[455]);
        self.scalar_static_f64[1586]=(self.scalar_static_f64[1581]/self.scalar_static_f64[1585]);
        self.scalar_static_f64[1587]=(self.scalar_static_f64[1584]/self.scalar_static_f64[1585]);
        self.scalar_static_f64[1588]=(1.0/self.scalar_static_f64[1586]);
        self.scalar_static_f64[1589]=(1.0+self.scalar_static_f64[1588]);
        self.scalar_static_f64[1590]=(1.0/self.scalar_static_f64[1587]);
        self.scalar_static_f64[1591]=(self.scalar_static_f64[1589]+self.scalar_static_f64[1590]);
        self.scalar_static_f64[1592]=(1.0/self.scalar_static_f64[1591]);
        self.scalar_static_f64[1593]=(self.scalar_static_f64[1585]*self.scalar_static_f64[1585]);
        self.scalar_static_f64[1594]=(self.scalar_static_f64[565]*8.010882825e-20);
        self.scalar_static_f64[1595]=(self.scalar_static_f64[455]*self.scalar_static_f64[1594]);
        self.scalar_static_f64[1596]=(self.scalar_static_f64[1581]+self.scalar_static_f64[1584]);
        self.scalar_static_f64[1597]=(self.scalar_static_f64[1595]/self.scalar_static_f64[1596]);
        self.scalar_static_f64[1598]=(self.scalar_static_f64[474]*3.20435313e-19);
        self.scalar_static_f64[1599]=(1.04479e-10*self.scalar_static_f64[1598]);
        self.scalar_static_f64[1600]=p.p2;
        self.scalar_static_bool[198]=(self.scalar_static_f64[1600]>0.0);
        self.scalar_static_f64[1601]=p.p9;
        self.scalar_static_bool[199]=(self.scalar_static_f64[1601]>0.0);
        self.scalar_static_f64[1602]=(self.scalar_static_f64[1552]*3.20435313e-19);
        self.scalar_static_f64[1603]=(self.scalar_static_f64[527]*self.scalar_static_f64[1602]);
        self.scalar_static_f64[1604]=(self.scalar_static_f64[1603]).sqrt();
        self.scalar_static_f64[1605]=(self.scalar_static_f64[1604]/self.scalar_static_f64[1272]);
        self.scalar_static_f64[1606]=(self.scalar_static_f64[455]*1e18);
        self.scalar_static_f64[1607]=(self.scalar_static_f64[455]*self.scalar_static_f64[1606]);
        self.scalar_static_bool[200]=(self.scalar_static_f64[1563]>0.0);
        self.scalar_static_f64[1608]=p.p14;
        self.scalar_static_bool[201]=(1.0==self.scalar_static_f64[1608]);
        self.scalar_static_bool[202]=(self.scalar_static_bool[200]&&self.scalar_static_bool[201]);
        self.scalar_static_f64[1609]=(0.409618895/self.scalar_static_f64[1607]);
        self.scalar_static_f64[1610]=(if self.scalar_static_bool[202]{self.scalar_static_f64[1609]}else{0.0});
        self.scalar_static_f64[1611]=(0.4*self.scalar_static_f64[1563]);
        self.scalar_static_f64[1612]=(self.scalar_static_f64[1611]*1.27520989);
        self.scalar_static_bool[203]=(!self.scalar_static_bool[201]);
        self.scalar_static_bool[204]=(self.scalar_static_bool[200]&&self.scalar_static_bool[203]);
        self.scalar_static_f64[1613]=(0.723134895/self.scalar_static_f64[1607]);
        self.scalar_static_f64[1614]=(if self.scalar_static_bool[204]{self.scalar_static_f64[1613]}else{self.scalar_static_f64[1610]});
        self.scalar_static_f64[1615]=(self.scalar_static_f64[1611]*1.5412087);
        self.scalar_static_f64[1616]=(self.scalar_static_f64[517]*self.scalar_static_f64[1608]);
        self.scalar_static_f64[1617]=p.p34;
        self.scalar_static_f64[1618]=p.p35;
        self.scalar_static_f64[1619]=(0.5*self.scalar_static_f64[728]);
        self.scalar_static_f64[1620]=(1.0/self.scalar_static_f64[1619]);
        self.scalar_static_f64[1621]=(self.scalar_static_f64[1620]/self.scalar_static_f64[730]);
        self.scalar_static_f64[1622]=(0.5*self.scalar_static_f64[759]);
        self.scalar_static_f64[1623]=(if self.scalar_static_bool[201]{self.scalar_static_f64[1622]}else{0.0});
        self.scalar_static_f64[1624]=(0.3333333333333*self.scalar_static_f64[759]);
        self.scalar_static_f64[1625]=(if self.scalar_static_bool[203]{self.scalar_static_f64[1624]}else{self.scalar_static_f64[1623]});
        self.scalar_static_f64[1626]=(1.0-self.scalar_static_f64[1625]);
        self.scalar_static_f64[1627]=(16.0/self.scalar_static_f64[831]);
        self.scalar_static_f64[1628]=(0.6931471805599*self.scalar_static_f64[1627]);
        self.scalar_static_f64[1629]=(self.scalar_static_f64[1628]).exp();
        self.scalar_static_f64[1630]=(self.scalar_static_f64[1629]-1.0);
        self.scalar_static_f64[1631]=(self.scalar_static_f64[1630]).ln();
        self.scalar_static_f64[1632]=(0.375*self.scalar_static_f64[1631]);
        self.scalar_static_f64[1633]=(self.scalar_static_f64[1632]).exp();
        self.scalar_static_f64[1634]=(self.scalar_static_f64[1633]-1.0);
        self.scalar_static_f64[1635]=(16.0/self.scalar_static_f64[1239]);
        self.scalar_static_f64[1636]=(0.6931471805599*self.scalar_static_f64[1635]);
        self.scalar_static_f64[1637]=(self.scalar_static_f64[1636]).exp();
        self.scalar_static_f64[1638]=(self.scalar_static_f64[1637]-1.0);
        self.scalar_static_f64[1639]=(self.scalar_static_f64[1638]).ln();
        self.scalar_static_f64[1640]=(0.375*self.scalar_static_f64[1639]);
        self.scalar_static_f64[1641]=(self.scalar_static_f64[1640]).exp();
        self.scalar_static_f64[1642]=(self.scalar_static_f64[1641]-1.0);
        self.scalar_static_f64[1643]=(-self.scalar_static_f64[896]);
        self.scalar_static_f64[1644]=(-self.scalar_static_f64[898]);
        self.scalar_static_f64[1645]=(1.0/self.scalar_static_f64[919]);
        self.scalar_static_f64[1646]=(self.scalar_static_f64[919]*2.9189679640027008e-49);
        self.scalar_static_f64[1647]=(self.scalar_static_f64[1646]).sqrt();
        self.scalar_static_f64[1648]=(1.3333333333332*self.scalar_static_f64[1647]);
        self.scalar_static_f64[1649]=(self.scalar_static_f64[1648]/1.054571726e-34);
        self.scalar_static_f64[1650]=(self.scalar_static_f64[478]*self.scalar_static_f64[1649]);
        self.scalar_static_bool[205]=(self.scalar_static_f64[902]<0.0);
        self.scalar_static_f64[1651]=(self.scalar_static_f64[900]* -0.495);
        self.scalar_static_f64[1652]=(self.scalar_static_f64[1651]/self.scalar_static_f64[902]);
        self.scalar_static_f64[1653]=(if self.scalar_static_bool[205]{self.scalar_static_f64[1652]}else{0.0});
        self.scalar_static_bool[206]=(self.scalar_static_f64[908]<0.0);
        self.scalar_static_f64[1654]=(self.scalar_static_f64[904]* -0.495);
        self.scalar_static_f64[1655]=(self.scalar_static_f64[1654]/self.scalar_static_f64[908]);
        self.scalar_static_f64[1656]=(if self.scalar_static_bool[206]{self.scalar_static_f64[1655]}else{0.0});
        self.scalar_static_bool[207]=(self.scalar_static_f64[912]<0.0);
        self.scalar_static_f64[1657]=(self.scalar_static_f64[910]* -0.495);
        self.scalar_static_f64[1658]=(self.scalar_static_f64[1657]/self.scalar_static_f64[912]);
        self.scalar_static_f64[1659]=(if self.scalar_static_bool[207]{self.scalar_static_f64[1658]}else{0.0});
        self.scalar_static_f64[1660]=(self.scalar_static_f64[478]*self.scalar_static_f64[478]);
        self.scalar_static_f64[1661]=(4e-18/self.scalar_static_f64[1660]);
        self.scalar_static_f64[1662]=(self.scalar_static_f64[928]*self.scalar_static_f64[1661]);
        self.scalar_static_f64[1663]=(self.scalar_static_f64[1546]*self.scalar_static_f64[1661]);
        self.scalar_static_f64[1664]=(self.scalar_static_f64[478]*500000000.0);
        self.scalar_static_f64[1665]=(-self.scalar_static_f64[973]);
        self.scalar_static_f64[1666]=(self.scalar_static_f64[1018]*self.scalar_static_f64[1608]);
        self.scalar_static_f64[1667]=(self.scalar_static_f64[1074]*4.0054414125e-20);
        self.scalar_static_f64[1668]=(self.scalar_static_f64[1078]*1.25e-6);
        self.scalar_static_f64[1669]=(self.scalar_static_f64[1552]/3.45313e-11);
        self.scalar_static_f64[1670]=(self.scalar_static_f64[455]*self.scalar_static_f64[1669]);
        self.scalar_static_f64[1671]=(self.scalar_static_f64[540]*self.scalar_static_f64[1670]);
        self.scalar_static_f64[1672]=(self.scalar_static_f64[1671]).sqrt();
        self.scalar_static_f64[1673]=(self.scalar_static_f64[1362]*9.10938291e-19);
        self.scalar_static_bool[208]=(self.scalar_static_f64[1394]>0.0);
        self.scalar_static_f64[1674]=(1.0/self.scalar_static_f64[1394]);
        self.scalar_static_f64[1675]=(if self.scalar_static_bool[208]{self.scalar_static_f64[1674]}else{0.0});
        self.scalar_static_bool[209]=(!self.scalar_static_bool[208]);
        self.scalar_static_f64[1676]=(if self.scalar_static_bool[209]{0.0}else{self.scalar_static_f64[1675]});
        self.scalar_static_bool[210]=(self.scalar_static_f64[1406]>0.0);
        self.scalar_static_f64[1677]=(1.0/self.scalar_static_f64[1406]);
        self.scalar_static_f64[1678]=(if self.scalar_static_bool[210]{self.scalar_static_f64[1677]}else{0.0});
        self.scalar_static_bool[211]=(!self.scalar_static_bool[210]);
        self.scalar_static_f64[1679]=(if self.scalar_static_bool[211]{0.0}else{self.scalar_static_f64[1678]});
        self.scalar_static_bool[212]=(self.scalar_static_f64[1410]>0.0);
        self.scalar_static_f64[1680]=(1.0/self.scalar_static_f64[1410]);
        self.scalar_static_f64[1681]=(if self.scalar_static_bool[212]{self.scalar_static_f64[1680]}else{0.0});
        self.scalar_static_bool[213]=(!self.scalar_static_bool[212]);
        self.scalar_static_f64[1682]=(if self.scalar_static_bool[213]{0.0}else{self.scalar_static_f64[1681]});
        self.scalar_static_bool[214]=(self.scalar_static_f64[1413]>0.0);
        self.scalar_static_f64[1683]=(1.0/self.scalar_static_f64[1413]);
        self.scalar_static_f64[1684]=(if self.scalar_static_bool[214]{self.scalar_static_f64[1683]}else{0.0});
        self.scalar_static_bool[215]=(!self.scalar_static_bool[214]);
        self.scalar_static_f64[1685]=(if self.scalar_static_bool[215]{0.0}else{self.scalar_static_f64[1684]});
        self.scalar_static_bool[216]=(self.scalar_static_f64[12]>0.0);
        self.scalar_static_bool[217]=(self.scalar_static_bool[0]&&self.scalar_static_bool[216]);
        self.scalar_static_bool[218]=(self.scalar_static_bool[1]&&self.scalar_static_bool[216]);
        self.scalar_static_bool[219]=(self.scalar_static_bool[199]&&self.scalar_static_bool[216]);
        self.scalar_static_bool[220]=(self.scalar_static_bool[200]&&self.scalar_static_bool[216]);
        self.scalar_static_bool[221]=(self.scalar_static_bool[201]&&self.scalar_static_bool[220]);
        self.scalar_static_bool[222]=(self.scalar_static_bool[203]&&self.scalar_static_bool[220]);
        self.scalar_static_f64[1686]=(self.scalar_static_f64[469]*self.scalar_static_f64[1608]);
        self.scalar_static_f64[1687]=(1.0+self.scalar_static_f64[1586]);
        self.scalar_static_f64[1688]=(1.0+self.scalar_static_f64[1587]);
        self.scalar_static_f64[1689]=(self.scalar_static_f64[1687]/self.scalar_static_f64[1688]);
        self.scalar_static_f64[1690]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1689]}else{0.0});
        self.scalar_static_f64[1691]=(self.scalar_static_f64[1690]).ln();
        self.scalar_static_f64[1692]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1691]}else{0.0});
        self.scalar_static_bool[223]=(self.scalar_static_f64[1692]>1e-8);
        self.scalar_static_bool[224]=(self.scalar_static_bool[198]&&self.scalar_static_bool[223]);
        self.scalar_static_f64[1693]=(2.0*self.scalar_static_f64[1692]);
        self.scalar_static_f64[1694]=(1.0+self.scalar_static_f64[1690]);
        self.scalar_static_f64[1695]=(self.scalar_static_f64[1693]*self.scalar_static_f64[1694]);
        self.scalar_static_f64[1696]=(self.scalar_static_f64[1690]-1.0);
        self.scalar_static_f64[1697]=(self.scalar_static_f64[1695]/self.scalar_static_f64[1696]);
        self.scalar_static_f64[1698]=(if self.scalar_static_bool[224]{self.scalar_static_f64[1697]}else{0.0});
        self.scalar_static_bool[225]=(!self.scalar_static_bool[223]);
        self.scalar_static_bool[226]=(self.scalar_static_bool[198]&&self.scalar_static_bool[225]);
        self.scalar_static_f64[1699]=(2.0+self.scalar_static_f64[1692]);
        self.scalar_static_f64[1700]=(2.0*self.scalar_static_f64[1699]);
        self.scalar_static_f64[1701]=(if self.scalar_static_bool[226]{self.scalar_static_f64[1700]}else{self.scalar_static_f64[1698]});
        self.scalar_static_f64[1702]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1588]}else{0.0});
        self.scalar_static_f64[1703]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1590]}else{0.0});
        self.scalar_static_f64[1704]=(1.0+self.scalar_static_f64[1702]);
        self.scalar_static_f64[1705]=(self.scalar_static_f64[1703]+self.scalar_static_f64[1704]);
        self.scalar_static_f64[1706]=(1.0/self.scalar_static_f64[1705]);
        self.scalar_static_f64[1707]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1706]}else{0.0});
        self.scalar_static_f64[1708]=(1.0/self.scalar_static_f64[1687]);
        self.scalar_static_f64[1709]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1708]}else{0.0});
        self.scalar_static_f64[1710]=(1.0/self.scalar_static_f64[1688]);
        self.scalar_static_f64[1711]=(if self.scalar_static_bool[198]{self.scalar_static_f64[1710]}else{0.0});
        self.scalar_static_f64[1712]=(self.scalar_static_f64[1587]*self.scalar_static_f64[1711]);
        self.scalar_static_f64[1713]=(self.scalar_static_f64[1586]+self.scalar_static_f64[1712]);
        self.scalar_static_f64[1714]=(self.scalar_static_f64[1701]*self.scalar_static_f64[1713]);
        self.scalar_static_f64[1715]=(self.scalar_static_f64[1586]*self.scalar_static_f64[1709]);
        self.scalar_static_f64[1716]=(self.scalar_static_f64[1587]+self.scalar_static_f64[1715]);
        self.scalar_static_f64[1717]=(self.scalar_static_f64[1701]*self.scalar_static_f64[1716]);
        self.scalar_static_bool[227]=(!self.scalar_static_bool[198]);
        self.scalar_static_bool[228]=(!self.scalar_static_bool[200]);
        self.scalar_static_bool[229]=(0.0==self.scalar_static_f64[774]);
        self.scalar_static_bool[230]=(self.scalar_static_f64[774]<0.0);
        self.scalar_static_bool[231]=(!self.scalar_static_bool[229]);
        self.scalar_static_bool[232]=(self.scalar_static_bool[230]&&self.scalar_static_bool[231]);
        self.scalar_static_bool[233]=(!self.scalar_static_bool[230]);
        self.scalar_static_bool[234]=(self.scalar_static_bool[231]&&self.scalar_static_bool[233]);
        self.scalar_static_bool[235]=(self.scalar_static_f64[812]<0.0);
        self.scalar_static_bool[236]=(!self.scalar_static_bool[235]);
        self.scalar_static_bool[237]=(self.scalar_static_f64[814]<0.0);
        self.scalar_static_bool[238]=(!self.scalar_static_bool[237]);
        self.scalar_static_f64[1718]=(self.scalar_static_f64[1605]*0.25);
        self.scalar_static_f64[1719]=(self.scalar_static_f64[1605]*self.scalar_static_f64[1718]);
        self.scalar_static_f64[1720]=(0.5*self.scalar_static_f64[1605]);
        self.scalar_static_bool[239]=(self.scalar_static_f64[867]>0.0);
        self.scalar_static_bool[240]=(!self.scalar_static_bool[239]);
        self.scalar_static_f64[1721]=(self.scalar_static_f64[1287]*self.scalar_static_f64[1608]);
        self.scalar_static_f64[1722]=(self.scalar_static_f64[481]*3.20435313e-19);
        self.scalar_static_f64[1723]=(self.scalar_static_f64[1552]*self.scalar_static_f64[1722]);
        self.scalar_static_f64[1724]=p.p3;
        self.scalar_static_bool[241]=(self.scalar_static_f64[1724]>0.0);
        self.scalar_static_f64[1725]=p.p4;
        self.scalar_static_bool[242]=(self.scalar_static_f64[1725]>0.0);
        self.scalar_static_bool[243]=(self.scalar_static_f64[1662]>0.0);
        self.scalar_static_bool[244]=(self.scalar_static_bool[242]&&self.scalar_static_bool[243]);
        self.scalar_static_f64[1726]=(self.scalar_static_f64[1542]*3.20435313e-19);
        self.scalar_static_f64[1727]=(self.scalar_static_f64[1552]*self.scalar_static_f64[1726]);
        self.scalar_static_bool[245]=(self.scalar_static_f64[1663]>0.0);
        self.scalar_static_bool[246]=(self.scalar_static_bool[242]&&self.scalar_static_bool[245]);
        self.scalar_static_f64[1728]=(-self.scalar_static_f64[906]);
        self.scalar_static_bool[247]=(0.0==self.scalar_static_f64[900]);
        self.scalar_static_bool[248]=(0.0==self.scalar_static_f64[902]);
        self.scalar_static_bool[249]=(self.scalar_static_bool[247]&&self.scalar_static_bool[248]);
        self.scalar_static_f64[1729]=(2.0*self.scalar_static_f64[902]);
        self.scalar_static_f64[1730]=(self.scalar_static_f64[945]*self.scalar_static_f64[945]);
        self.scalar_static_f64[1731]=(-self.scalar_static_f64[1662]);
        self.scalar_static_f64[1732]=(self.scalar_static_f64[1549]*self.scalar_static_f64[1549]);
        self.scalar_static_f64[1733]=(-self.scalar_static_f64[1663]);
        self.scalar_static_f64[1734]=p.p12;
        self.scalar_static_bool[250]=(self.scalar_static_f64[1734]>0.0);
        self.scalar_static_f64[1735]=p.p8;
        self.scalar_static_bool[251]=(0.0!=self.scalar_static_f64[1735]);
        self.scalar_static_f64[1736]=p.p16;
        self.scalar_static_f64[1737]=(100000000.0*self.scalar_static_f64[1736]);
        self.scalar_static_f64[1738]=(0.25/self.scalar_static_f64[1736]);
        self.scalar_static_f64[1739]=(self.scalar_static_f64[1736]+self.scalar_static_f64[1738]);
        self.scalar_static_f64[1740]=(-self.scalar_static_f64[1739]);
        self.scalar_static_bool[252]=(!self.scalar_static_bool[216]);
        self.scalar_static_f64[1741]=(if self.scalar_static_bool[17]{self.scalar_static_f64[1154]}else{0.0});
        self.scalar_static_f64[1742]=(if self.scalar_static_bool[17]{self.scalar_static_f64[1158]}else{0.0});
        self.scalar_static_f64[1743]=(if self.scalar_static_bool[17]{self.scalar_static_f64[1642]}else{0.0});
        self.scalar_static_f64[1744]=(if self.scalar_static_bool[17]{self.scalar_static_f64[1271]}else{0.0});
        self.scalar_static_bool[253]=(self.scalar_static_bool[17]&&self.scalar_static_bool[198]);
        self.scalar_static_f64[1745]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1689]}else{0.0});
        self.scalar_static_f64[1746]=(self.scalar_static_f64[1745]).ln();
        self.scalar_static_f64[1747]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1746]}else{0.0});
        self.scalar_static_bool[254]=(self.scalar_static_f64[1747]>1e-8);
        self.scalar_static_bool[255]=(self.scalar_static_bool[253]&&self.scalar_static_bool[254]);
        self.scalar_static_f64[1748]=(2.0*self.scalar_static_f64[1747]);
        self.scalar_static_f64[1749]=(1.0+self.scalar_static_f64[1745]);
        self.scalar_static_f64[1750]=(self.scalar_static_f64[1748]*self.scalar_static_f64[1749]);
        self.scalar_static_f64[1751]=(self.scalar_static_f64[1745]-1.0);
        self.scalar_static_f64[1752]=(self.scalar_static_f64[1750]/self.scalar_static_f64[1751]);
        self.scalar_static_f64[1753]=(if self.scalar_static_bool[255]{self.scalar_static_f64[1752]}else{0.0});
        self.scalar_static_bool[256]=(!self.scalar_static_bool[254]);
        self.scalar_static_bool[257]=(self.scalar_static_bool[253]&&self.scalar_static_bool[256]);
        self.scalar_static_f64[1754]=(2.0+self.scalar_static_f64[1747]);
        self.scalar_static_f64[1755]=(2.0*self.scalar_static_f64[1754]);
        self.scalar_static_f64[1756]=(if self.scalar_static_bool[257]{self.scalar_static_f64[1755]}else{self.scalar_static_f64[1753]});
        self.scalar_static_f64[1757]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1588]}else{0.0});
        self.scalar_static_f64[1758]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1590]}else{0.0});
        self.scalar_static_f64[1759]=(1.0+self.scalar_static_f64[1757]);
        self.scalar_static_f64[1760]=(self.scalar_static_f64[1758]+self.scalar_static_f64[1759]);
        self.scalar_static_f64[1761]=(1.0/self.scalar_static_f64[1760]);
        self.scalar_static_f64[1762]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1761]}else{0.0});
        self.scalar_static_f64[1763]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1708]}else{0.0});
        self.scalar_static_f64[1764]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1710]}else{0.0});
        self.scalar_static_f64[1765]=(self.scalar_static_f64[1587]*self.scalar_static_f64[1764]);
        self.scalar_static_f64[1766]=(self.scalar_static_f64[1586]+self.scalar_static_f64[1765]);
        self.scalar_static_f64[1767]=(self.scalar_static_f64[1756]*self.scalar_static_f64[1766]);
        self.scalar_static_f64[1768]=(self.scalar_static_f64[1586]*self.scalar_static_f64[1763]);
        self.scalar_static_f64[1769]=(self.scalar_static_f64[1587]+self.scalar_static_f64[1768]);
        self.scalar_static_f64[1770]=(self.scalar_static_f64[1756]*self.scalar_static_f64[1769]);
        self.scalar_static_bool[258]=(self.scalar_static_bool[17]&&self.scalar_static_bool[227]);
        self.scalar_static_bool[259]=(self.scalar_static_bool[17]&&self.scalar_static_bool[200]);
        self.scalar_static_bool[260]=(self.scalar_static_bool[17]&&self.scalar_static_bool[228]);
        self.scalar_static_bool[261]=(self.scalar_static_bool[0]&&self.scalar_static_bool[17]);
        self.scalar_static_bool[262]=(self.scalar_static_bool[17]&&self.scalar_static_bool[229]);
        self.scalar_static_bool[263]=(self.scalar_static_bool[17]&&self.scalar_static_bool[231]);
        self.scalar_static_bool[264]=(self.scalar_static_bool[230]&&self.scalar_static_bool[263]);
        self.scalar_static_bool[265]=(self.scalar_static_bool[233]&&self.scalar_static_bool[263]);
        self.scalar_static_f64[1771]=(if self.scalar_static_bool[17]{1.0}else{0.0});
        self.scalar_static_bool[266]=(self.scalar_static_bool[17]&&self.scalar_static_bool[199]);
        self.scalar_static_bool[267]=(self.scalar_static_bool[17]&&self.scalar_static_bool[239]);
        self.scalar_static_bool[268]=(self.scalar_static_bool[17]&&self.scalar_static_bool[240]);
        self.scalar_static_bool[269]=(self.scalar_static_bool[17]&&self.scalar_static_bool[235]);
        self.scalar_static_bool[270]=(self.scalar_static_bool[17]&&self.scalar_static_bool[236]);
        self.scalar_static_bool[271]=(self.scalar_static_bool[17]&&self.scalar_static_bool[237]);
        self.scalar_static_bool[272]=(self.scalar_static_bool[17]&&self.scalar_static_bool[238]);
        self.scalar_static_bool[273]=(!self.scalar_static_bool[17]);
        self.scalar_static_bool[274]=(self.scalar_static_f64[1078]>0.0);
        self.scalar_static_bool[275]=(!self.scalar_static_bool[274]);
        self.scalar_static_f64[1772]=(self.scalar_static_f64[380]*self.scalar_static_f64[1576]);
        self.scalar_static_f64[1773]=(self.scalar_static_f64[384]*self.scalar_static_f64[1299]);
        self.scalar_static_f64[1774]=(self.scalar_static_f64[1772]+self.scalar_static_f64[1773]);
        self.scalar_static_f64[1775]=(-self.scalar_static_f64[1774]);
        self.scalar_static_f64[1776]=(self.scalar_static_f64[378]*self.scalar_static_f64[1576]);
        self.scalar_static_f64[1777]=(self.scalar_static_f64[382]*self.scalar_static_f64[1299]);
        self.scalar_static_f64[1778]=(self.scalar_static_f64[1776]+self.scalar_static_f64[1777]);
        self.scalar_static_f64[1779]=(-self.scalar_static_f64[1778]);
        self.scalar_static_f64[1780]=p.p31;
        self.scalar_static_f64[1781]=(self.scalar_static_f64[386]*self.scalar_static_f64[1780]);
        self.scalar_static_f64[1782]=p.p32;
        self.scalar_static_f64[1783]=(self.scalar_static_f64[386]*self.scalar_static_f64[1782]);
        self.scalar_static_bool[276]=(self.scalar_static_f64[386]>0.0);
        self.scalar_static_f64[1784]=(self.scalar_static_f64[1069]*self.scalar_static_f64[1272]);
        self.scalar_static_bool[277]=(!self.scalar_static_bool[276]);
        self.scalar_static_bool[278]=(self.scalar_static_f64[1362]>0.0);
        self.scalar_static_f64[1785]=p.p6;
        self.scalar_static_bool[279]=(self.scalar_static_f64[1785]>0.0);
        self.scalar_static_bool[280]=(!self.scalar_static_bool[279]);
        self.scalar_static_f64[1786]=(0.0*self.scalar_static_f64[1780]);
        self.scalar_static_f64[1787]=(self.scalar_static_f64[1676]*self.scalar_static_f64[1781]);
        self.scalar_static_f64[1788]=(self.scalar_static_f64[1679]*self.scalar_static_f64[1781]);
        self.scalar_static_f64[1789]=(self.scalar_static_f64[1682]*self.scalar_static_f64[1781]);
        self.scalar_static_f64[1790]=(self.scalar_static_f64[1685]*self.scalar_static_f64[1781]);
        self.scalar_static_f64[1791]=(self.scalar_static_f64[1374]).sqrt();
        self.scalar_static_f64[1792]=(1.0-self.scalar_static_f64[1372]);
        self.scalar_static_f64[1793]=(if self.scalar_static_bool[216]{1.0}else{0.0});
        self.scalar_static_f64[1794]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1793]}else{0.0});
        self.scalar_static_f64[1795]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1794]}else{0.0});
        self.scalar_static_f64[1796]=(self.scalar_static_f64[1]*self.scalar_static_f64[1794]);
        self.scalar_static_f64[1797]=(-self.scalar_static_f64[1796]);
        self.scalar_static_f64[1798]=(8.617332384961e-5*self.scalar_static_f64[1794]);
        self.scalar_static_f64[1799]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1798]}else{0.0});
        self.scalar_static_f64[1800]=(-self.scalar_static_f64[1799]);
        self.scalar_static_f64[1801]=(0.0033333333333*self.scalar_static_f64[1794]);
        self.scalar_static_f64[1802]=(self.scalar_static_f64[1616]*self.scalar_static_f64[1795]);
        self.scalar_static_f64[1803]=(self.scalar_static_f64[873]*self.scalar_static_f64[1799]);
        self.scalar_static_f64[1804]=(if self.scalar_static_bool[216]{self.scalar_static_f64[1803]}else{0.0});
        self.scalar_static_f64[1805]=(self.scalar_static_f64[941]*self.scalar_static_f64[1795]);
        self.scalar_static_f64[1806]=(self.scalar_static_f64[1548]*self.scalar_static_f64[1795]);
        self.scalar_static_f64[1807]=(if self.scalar_static_bool[201]{-1.0}else{0.0});
        self.scalar_static_f64[1808]=(if self.scalar_static_bool[201]{1.0}else{0.0});
        self.scalar_static_f64[1809]=(if self.scalar_static_bool[203]{1.0}else{self.scalar_static_f64[1807]});
        self.scalar_static_f64[1810]=(if self.scalar_static_bool[203]{-1.0}else{self.scalar_static_f64[1808]});
        self.scalar_static_f64[1811]=(-self.scalar_static_f64[1809]);
        self.scalar_static_f64[1812]=(-self.scalar_static_f64[1810]);
        self.scalar_static_f64[1813]=(self.scalar_static_f64[1809]+self.scalar_static_f64[1811]);
        self.scalar_static_f64[1814]=(self.scalar_static_f64[1809]+self.scalar_static_f64[1810]);
        self.scalar_static_f64[1815]=(-self.scalar_static_f64[1813]);
        self.scalar_static_f64[1816]=(self.scalar_static_f64[915]*self.scalar_static_f64[1813]);
        self.scalar_static_f64[1817]=(self.scalar_static_f64[915]*self.scalar_static_f64[1812]);
        self.scalar_static_f64[1818]=(self.scalar_static_f64[915]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1819]=(self.scalar_static_f64[915]*self.scalar_static_f64[1811]);
        self.scalar_static_f64[1820]=(self.scalar_static_f64[915]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1821]=(self.scalar_static_f64[1730]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1822]=(self.scalar_static_f64[1730]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1823]=(self.scalar_static_f64[952]*self.scalar_static_f64[1811]);
        self.scalar_static_f64[1824]=(self.scalar_static_f64[952]*self.scalar_static_f64[1812]);
        self.scalar_static_f64[1825]=(self.scalar_static_f64[1731]*self.scalar_static_f64[1811]);
        self.scalar_static_f64[1826]=(self.scalar_static_f64[1731]*self.scalar_static_f64[1812]);
        self.scalar_static_f64[1827]=(self.scalar_static_f64[1732]*self.scalar_static_f64[1814]);
        self.scalar_static_f64[1828]=(self.scalar_static_f64[1732]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1829]=(self.scalar_static_f64[1732]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1830]=(self.scalar_static_f64[1550]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1831]=(self.scalar_static_f64[1550]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1832]=(self.scalar_static_f64[1733]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1833]=(self.scalar_static_f64[1733]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1834]=(self.scalar_static_f64[1775]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1835]=(self.scalar_static_f64[1775]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1836]=(self.scalar_static_f64[1779]*self.scalar_static_f64[1814]);
        self.scalar_static_f64[1837]=(self.scalar_static_f64[1779]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[1838]=(self.scalar_static_f64[1779]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[1839]=(self.scalar_static_f64[1783]*self.scalar_static_f64[1834]);
        self.scalar_static_f64[1840]=(self.scalar_static_f64[1783]*self.scalar_static_f64[1835]);
        self.scalar_static_f64[1841]=(self.scalar_static_f64[1783]*self.scalar_static_f64[1836]);
        self.scalar_static_f64[1842]=(self.scalar_static_f64[1783]*self.scalar_static_f64[1837]);
        self.scalar_static_f64[1843]=(self.scalar_static_f64[1783]*self.scalar_static_f64[1838]);
        self.scalar_static_f64[1844]=(-self.scalar_static_f64[1786]);
        self.scalar_static_f64[1845]=(-self.scalar_static_f64[1787]);
        self.scalar_static_f64[1846]=(if self.scalar_static_bool[208]{self.scalar_static_f64[1787]}else{0.0});
        self.scalar_static_f64[1847]=(if self.scalar_static_bool[208]{self.scalar_static_f64[1845]}else{0.0});
        self.scalar_static_f64[1848]=(-self.scalar_static_f64[1788]);
        self.scalar_static_f64[1849]=(if self.scalar_static_bool[210]{self.scalar_static_f64[1788]}else{0.0});
        self.scalar_static_f64[1850]=(if self.scalar_static_bool[210]{self.scalar_static_f64[1848]}else{0.0});
        self.scalar_static_f64[1851]=(-self.scalar_static_f64[1789]);
        self.scalar_static_f64[1852]=(if self.scalar_static_bool[212]{self.scalar_static_f64[1789]}else{0.0});
        self.scalar_static_f64[1853]=(if self.scalar_static_bool[212]{self.scalar_static_f64[1851]}else{0.0});
        self.scalar_static_f64[1854]=(-self.scalar_static_f64[1790]);
        self.scalar_static_f64[1855]=(if self.scalar_static_bool[214]{self.scalar_static_f64[1790]}else{0.0});
        self.scalar_static_f64[1856]=(if self.scalar_static_bool[214]{self.scalar_static_f64[1854]}else{0.0});
        self.scalar_static_f64[1857]=(1.0/self.scalar_static_f64[1791]);
        self.scalar_static_f64[1858]=(-1.0/self.scalar_static_f64[1791]);
        self.scalar_static_f64[1859]=(-1.0+self.scalar_static_f64[1858]);
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
        self.scalar_static_f64[1860]=(temperature+self.scalar_static_f64[2]);
        self.scalar_static_bool[281]=(self.scalar_static_f64[1860]<1000.0);
        self.scalar_static_f64[1861]=(if self.scalar_static_bool[281]{self.scalar_static_f64[1860]}else{1000.0});
        self.scalar_static_f64[1862]=(self.scalar_static_f64[1861]*self.scalar_static_f64[5]);
        self.scalar_static_f64[1863]=(self.scalar_static_f64[4]+self.scalar_static_f64[1862]);
        self.scalar_static_f64[1864]=(self.scalar_static_f64[1861]+self.scalar_static_f64[1863]);
        self.scalar_static_f64[1865]=(self.scalar_static_f64[1861]-self.scalar_static_f64[1863]);
        self.scalar_static_f64[1866]=(self.scalar_static_f64[1865]*self.scalar_static_f64[1865]);
        self.scalar_static_f64[1867]=(self.scalar_static_f64[1866]+self.scalar_static_f64[6]);
        self.scalar_static_f64[1868]=(self.scalar_static_f64[1867]).sqrt();
        self.scalar_static_f64[1869]=(self.scalar_static_f64[1864]+self.scalar_static_f64[1868]);
        self.scalar_static_f64[1870]=(0.5*self.scalar_static_f64[1869]);
        self.scalar_static_f64[1871]=(if self.scalar_static_bool[0]{self.scalar_static_f64[1870]}else{0.0});
        self.scalar_static_f64[1872]=(self.scalar_static_f64[1871]*8.617332384961e-5);
        self.scalar_static_f64[1873]=(10.0/self.scalar_static_f64[1872]);
        self.scalar_static_f64[1874]=(self.scalar_static_f64[1873]+600.0);
        self.scalar_static_f64[1875]=(self.scalar_static_f64[1873]-600.0);
        self.scalar_static_f64[1876]=(self.scalar_static_f64[1875]*self.scalar_static_f64[1875]);
        self.scalar_static_f64[1877]=(self.scalar_static_f64[1876]+0.01);
        self.scalar_static_f64[1878]=(self.scalar_static_f64[1877]).sqrt();
        self.scalar_static_f64[1879]=(self.scalar_static_f64[1874]+self.scalar_static_f64[1878]);
        self.scalar_static_f64[1880]=(0.5*self.scalar_static_f64[1879]);
        self.scalar_static_f64[1881]=(if self.scalar_static_bool[0]{self.scalar_static_f64[1880]}else{0.0});
        self.scalar_static_f64[1882]=(self.scalar_static_f64[1861]+1.0);
        self.scalar_static_f64[1883]=(self.scalar_static_f64[1861]-1.0);
        self.scalar_static_f64[1884]=(self.scalar_static_f64[1883]*self.scalar_static_f64[1883]);
        self.scalar_static_f64[1885]=(self.scalar_static_f64[1884]+0.001);
        self.scalar_static_f64[1886]=(self.scalar_static_f64[1885]).sqrt();
        self.scalar_static_f64[1887]=(self.scalar_static_f64[1882]+self.scalar_static_f64[1886]);
        self.scalar_static_f64[1888]=(0.5*self.scalar_static_f64[1887]);
        self.scalar_static_f64[1889]=(if self.scalar_static_bool[1]{self.scalar_static_f64[1888]}else{self.scalar_static_f64[1871]});
        self.scalar_static_f64[1890]=(if self.scalar_static_bool[1]{600.0}else{self.scalar_static_f64[1881]});
        self.scalar_static_f64[1891]=(self.scalar_static_f64[1889]*self.scalar_static_f64[1889]);
        self.scalar_static_f64[1892]=(self.scalar_static_f64[1889]-self.scalar_static_f64[1]);
        self.scalar_static_f64[1893]=(self.scalar_static_f64[1]/self.scalar_static_f64[1889]);
        self.scalar_static_f64[1894]=(8.617332384961e-5*self.scalar_static_f64[1889]);
        self.scalar_static_f64[1895]=(1.0/self.scalar_static_f64[1894]);
        self.scalar_static_f64[1896]=(if self.scalar_static_bool[33]{self.scalar_static_f64[439]}else{self.scalar_static_f64[1861]});
        self.scalar_static_f64[1897]=(self.scalar_static_f64[1896]/self.scalar_static_f64[387]);
        self.scalar_static_f64[1898]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1897]}else{0.0});
        self.scalar_static_f64[1899]=(if self.scalar_static_bool[33]{self.scalar_static_f64[440]}else{self.scalar_static_f64[1896]});
        self.scalar_static_f64[1900]=(self.scalar_static_f64[1899]/self.scalar_static_f64[387]);
        self.scalar_static_f64[1901]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1900]}else{0.0});
        self.scalar_static_f64[1902]=(if self.scalar_static_bool[33]{self.scalar_static_f64[494]}else{self.scalar_static_f64[1899]});
        self.scalar_static_f64[1903]=(self.scalar_static_f64[1902]+self.scalar_static_f64[495]);
        self.scalar_static_f64[1904]=(self.scalar_static_f64[1903]+self.scalar_static_f64[497]);
        self.scalar_static_f64[1905]=(self.scalar_static_f64[1904]+self.scalar_static_f64[499]);
        self.scalar_static_f64[1906]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1905]}else{self.scalar_static_f64[56]});
        self.scalar_static_f64[1907]=(self.scalar_static_f64[1902]*self.scalar_static_f64[503]);
        self.scalar_static_f64[1908]=(self.scalar_static_f64[500]+self.scalar_static_f64[1907]);
        self.scalar_static_f64[1909]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1908]}else{self.scalar_static_f64[58]});
        self.scalar_static_f64[1910]=(if self.scalar_static_bool[33]{self.scalar_static_f64[579]}else{self.scalar_static_f64[1902]});
        self.scalar_static_f64[1911]=(self.scalar_static_f64[1910]*self.scalar_static_f64[580]);
        self.scalar_static_f64[1912]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1911]}else{0.0});
        self.scalar_static_bool[282]=(self.scalar_static_f64[1912]>0.0);
        self.scalar_static_f64[1913]=(if self.scalar_static_bool[282]{self.scalar_static_f64[1912]}else{0.0});
        self.scalar_static_f64[1914]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1913]}else{self.scalar_static_f64[83]});
        self.scalar_static_f64[1915]=(self.scalar_static_f64[1914]*self.scalar_static_f64[581]);
        self.scalar_static_f64[1916]=(self.scalar_static_f64[459]*self.scalar_static_f64[1915]);
        self.scalar_static_f64[1917]=(self.scalar_static_f64[1916]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1918]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1917]}else{self.scalar_static_f64[88]});
        self.scalar_static_f64[1919]=(self.scalar_static_f64[1910]*self.scalar_static_f64[582]);
        self.scalar_static_f64[1920]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1919]}else{self.scalar_static_f64[90]});
        self.scalar_static_f64[1921]=(if self.scalar_static_bool[33]{self.scalar_static_f64[997]}else{self.scalar_static_f64[1910]});
        self.scalar_static_f64[1922]=(self.scalar_static_f64[1921]+self.scalar_static_f64[998]);
        self.scalar_static_f64[1923]=(self.scalar_static_f64[1922]+self.scalar_static_f64[1000]);
        self.scalar_static_f64[1924]=(self.scalar_static_f64[1923]+self.scalar_static_f64[1002]);
        self.scalar_static_f64[1925]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1924]}else{self.scalar_static_f64[241]});
        self.scalar_static_f64[1926]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1031]}else{self.scalar_static_f64[1921]});
        self.scalar_static_bool[283]=(self.scalar_static_f64[1926]>0.0);
        self.scalar_static_f64[1927]=(if self.scalar_static_bool[283]{self.scalar_static_f64[1926]}else{0.0});
        self.scalar_static_bool[284]=(self.scalar_static_f64[1927]<5.0);
        self.scalar_static_f64[1928]=(if self.scalar_static_bool[284]{self.scalar_static_f64[1927]}else{5.0});
        self.scalar_static_f64[1929]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1928]}else{self.scalar_static_f64[251]});
        self.scalar_static_f64[1930]=(self.scalar_static_f64[1929]*self.scalar_static_f64[1032]);
        self.scalar_static_f64[1931]=(self.scalar_static_f64[459]*self.scalar_static_f64[1930]);
        self.scalar_static_f64[1932]=(self.scalar_static_f64[1931]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1933]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1932]}else{self.scalar_static_f64[256]});
        self.scalar_static_f64[1934]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1038]}else{self.scalar_static_f64[1926]});
        self.scalar_static_f64[1935]=(self.scalar_static_f64[1934]*self.scalar_static_f64[1039]);
        self.scalar_static_f64[1936]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1935]}else{self.scalar_static_f64[1934]});
        self.scalar_static_bool[285]=(self.scalar_static_f64[1936]>0.0);
        self.scalar_static_f64[1937]=(if self.scalar_static_bool[285]{self.scalar_static_f64[1936]}else{0.0});
        self.scalar_static_f64[1938]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1937]}else{self.scalar_static_f64[258]});
        self.scalar_static_f64[1939]=(self.scalar_static_f64[1938]*self.scalar_static_f64[1040]);
        self.scalar_static_f64[1940]=(self.scalar_static_f64[459]*self.scalar_static_f64[1939]);
        self.scalar_static_f64[1941]=(self.scalar_static_f64[1940]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1942]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1941]}else{self.scalar_static_f64[263]});
        self.scalar_static_f64[1943]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1051]}else{self.scalar_static_f64[1936]});
        self.scalar_static_bool[286]=(self.scalar_static_f64[1943]>1e-15);
        self.scalar_static_f64[1944]=(if self.scalar_static_bool[286]{self.scalar_static_f64[1943]}else{1e-15});
        self.scalar_static_f64[1945]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1944]}else{self.scalar_static_f64[1943]});
        self.scalar_static_f64[1946]=(self.scalar_static_f64[418]*self.scalar_static_f64[1945]);
        self.scalar_static_f64[1947]=(self.scalar_static_f64[1052]/self.scalar_static_f64[1946]);
        self.scalar_static_f64[1948]=(self.scalar_static_f64[1947]*self.scalar_static_f64[1055]);
        self.scalar_static_f64[1949]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1948]}else{self.scalar_static_f64[267]});
        self.scalar_static_f64[1950]=(self.scalar_static_f64[1898]*self.scalar_static_f64[1071]);
        self.scalar_static_f64[1951]=(self.scalar_static_f64[1070]+self.scalar_static_f64[1950]);
        self.scalar_static_f64[1952]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1951]}else{0.0});
        self.scalar_static_bool[287]=(self.scalar_static_f64[1952]>0.0);
        self.scalar_static_f64[1953]=(if self.scalar_static_bool[287]{self.scalar_static_f64[1952]}else{0.0});
        self.scalar_static_f64[1954]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1953]}else{self.scalar_static_f64[273]});
        self.scalar_static_f64[1955]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1906]}else{self.scalar_static_f64[294]});
        self.scalar_static_f64[1956]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1909]}else{self.scalar_static_f64[298]});
        self.scalar_static_f64[1957]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1912]}else{0.0});
        self.scalar_static_f64[1958]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1914]}else{self.scalar_static_f64[310]});
        self.scalar_static_f64[1959]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1918]}else{self.scalar_static_f64[314]});
        self.scalar_static_f64[1960]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1120]}else{self.scalar_static_f64[1945]});
        self.scalar_static_f64[1961]=(self.scalar_static_f64[1090]+self.scalar_static_f64[1960]);
        self.scalar_static_f64[1962]=(self.scalar_static_f64[1961]+self.scalar_static_f64[1121]);
        self.scalar_static_f64[1963]=(self.scalar_static_f64[1962]+self.scalar_static_f64[1122]);
        self.scalar_static_f64[1964]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1963]}else{self.scalar_static_f64[1955]});
        self.scalar_static_f64[1965]=(self.scalar_static_f64[1960]*self.scalar_static_f64[1132]);
        self.scalar_static_f64[1966]=(self.scalar_static_f64[1126]+self.scalar_static_f64[1965]);
        self.scalar_static_f64[1967]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1966]}else{self.scalar_static_f64[1956]});
        self.scalar_static_f64[1968]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1174]}else{self.scalar_static_f64[1960]});
        self.scalar_static_f64[1969]=(self.scalar_static_f64[1162]*self.scalar_static_f64[1968]);
        self.scalar_static_f64[1970]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1969]}else{self.scalar_static_f64[1957]});
        self.scalar_static_bool[288]=(self.scalar_static_f64[1970]>0.0);
        self.scalar_static_f64[1971]=(if self.scalar_static_bool[288]{self.scalar_static_f64[1970]}else{0.0});
        self.scalar_static_f64[1972]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1971]}else{self.scalar_static_f64[1958]});
        self.scalar_static_f64[1973]=(self.scalar_static_f64[581]*self.scalar_static_f64[1972]);
        self.scalar_static_f64[1974]=(self.scalar_static_f64[459]*self.scalar_static_f64[1973]);
        self.scalar_static_f64[1975]=(self.scalar_static_f64[1974]/self.scalar_static_f64[453]);
        self.scalar_static_f64[1976]=(if self.scalar_static_bool[80]{self.scalar_static_f64[1975]}else{self.scalar_static_f64[1959]});
        self.scalar_static_f64[1977]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1273]}else{self.scalar_static_f64[1968]});
        self.scalar_static_f64[1978]=(self.scalar_static_f64[1977]*self.scalar_static_f64[1274]);
        self.scalar_static_f64[1979]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1978]}else{self.scalar_static_f64[328]});
        self.scalar_static_f64[1980]=(self.scalar_static_f64[1977]*self.scalar_static_f64[1275]);
        self.scalar_static_f64[1981]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1980]}else{self.scalar_static_f64[330]});
        self.scalar_static_f64[1982]=(self.scalar_static_f64[1901]*self.scalar_static_f64[1289]);
        self.scalar_static_f64[1983]=(self.scalar_static_f64[1288]+self.scalar_static_f64[1982]);
        self.scalar_static_f64[1984]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1983]}else{0.0});
        self.scalar_static_bool[289]=(self.scalar_static_f64[1984]>0.0);
        self.scalar_static_f64[1985]=(if self.scalar_static_bool[289]{self.scalar_static_f64[1984]}else{0.0});
        self.scalar_static_f64[1986]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1985]}else{self.scalar_static_f64[338]});
        self.scalar_static_f64[1987]=(self.scalar_static_f64[1901]*self.scalar_static_f64[1291]);
        self.scalar_static_f64[1988]=(self.scalar_static_f64[1290]+self.scalar_static_f64[1987]);
        self.scalar_static_f64[1989]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1988]}else{0.0});
        self.scalar_static_bool[290]=(self.scalar_static_f64[1989]>0.0);
        self.scalar_static_f64[1990]=(if self.scalar_static_bool[290]{self.scalar_static_f64[1989]}else{0.0});
        self.scalar_static_f64[1991]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1990]}else{self.scalar_static_f64[340]});
        self.scalar_static_f64[1992]=(self.scalar_static_f64[1898]*self.scalar_static_f64[1300]);
        self.scalar_static_f64[1993]=(1.0+self.scalar_static_f64[1992]);
        self.scalar_static_f64[1994]=(self.scalar_static_f64[1901]*self.scalar_static_f64[1301]);
        self.scalar_static_f64[1995]=(self.scalar_static_f64[1993]+self.scalar_static_f64[1994]);
        self.scalar_static_f64[1996]=(self.scalar_static_f64[1898]*self.scalar_static_f64[1302]);
        self.scalar_static_f64[1997]=(self.scalar_static_f64[1901]*self.scalar_static_f64[1996]);
        self.scalar_static_f64[1998]=(self.scalar_static_f64[1995]+self.scalar_static_f64[1997]);
        self.scalar_static_bool[291]=(self.scalar_static_f64[1998]>1e-10);
        self.scalar_static_f64[1999]=(if self.scalar_static_bool[291]{self.scalar_static_f64[1998]}else{1e-10});
        self.scalar_static_f64[2000]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1999]}else{self.scalar_static_f64[1977]});
        self.scalar_static_f64[2001]=(self.scalar_static_f64[2000]/self.scalar_static_f64[1347]);
        self.scalar_static_f64[2002]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2001]}else{self.scalar_static_f64[2000]});
        self.scalar_static_f64[2003]=(self.scalar_static_f64[9]/self.scalar_static_f64[2002]);
        self.scalar_static_f64[2004]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2003]}else{0.0});
        self.scalar_static_bool[292]=(self.scalar_static_f64[2004]>1e-6);
        self.scalar_static_f64[2005]=(if self.scalar_static_bool[292]{self.scalar_static_f64[2004]}else{1e-6});
        self.scalar_static_f64[2006]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2005]}else{self.scalar_static_f64[345]});
        self.scalar_static_f64[2007]=(self.scalar_static_f64[2002]*self.scalar_static_f64[1350]);
        self.scalar_static_f64[2008]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2007]}else{0.0});
        self.scalar_static_bool[293]=(self.scalar_static_f64[2008]>0.0);
        self.scalar_static_f64[2009]=(if self.scalar_static_bool[293]{self.scalar_static_f64[2008]}else{0.0});
        self.scalar_static_f64[2010]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2009]}else{self.scalar_static_f64[349]});
        self.scalar_static_f64[2011]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1366]}else{self.scalar_static_f64[2002]});
        self.scalar_static_bool[294]=(self.scalar_static_f64[2011]>0.0);
        self.scalar_static_f64[2012]=(if self.scalar_static_bool[294]{self.scalar_static_f64[2011]}else{0.0});
        self.scalar_static_f64[2013]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2012]}else{self.scalar_static_f64[355]});
        self.scalar_static_f64[2014]=(if self.scalar_static_bool[33]{self.scalar_static_f64[1370]}else{self.scalar_static_f64[2011]});
        self.scalar_static_bool[295]=(self.scalar_static_f64[2014]>0.0);
        self.scalar_static_f64[2015]=(if self.scalar_static_bool[295]{self.scalar_static_f64[2014]}else{0.0});
        self.scalar_static_f64[2016]=(if self.scalar_static_bool[33]{self.scalar_static_f64[2015]}else{self.scalar_static_f64[357]});
        self.scalar_static_f64[2017]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1469]}else{self.scalar_static_f64[1861]});
        self.scalar_static_f64[2018]=(self.scalar_static_f64[1906]+self.scalar_static_f64[2017]);
        self.scalar_static_f64[2019]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2018]}else{self.scalar_static_f64[1906]});
        self.scalar_static_f64[2020]=(self.scalar_static_f64[1909]+self.scalar_static_f64[2017]);
        self.scalar_static_f64[2021]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2020]}else{self.scalar_static_f64[1909]});
        self.scalar_static_f64[2022]=(self.scalar_static_f64[1964]+self.scalar_static_f64[2017]);
        self.scalar_static_f64[2023]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2022]}else{self.scalar_static_f64[1964]});
        self.scalar_static_f64[2024]=(self.scalar_static_f64[1967]+self.scalar_static_f64[2017]);
        self.scalar_static_f64[2025]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2024]}else{self.scalar_static_f64[1967]});
        self.scalar_static_f64[2026]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1474]}else{self.scalar_static_f64[2017]});
        self.scalar_static_f64[2027]=(self.scalar_static_f64[1912]+self.scalar_static_f64[2026]);
        self.scalar_static_f64[2028]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2027]}else{self.scalar_static_f64[1912]});
        self.scalar_static_bool[296]=(self.scalar_static_f64[2028]>0.0);
        self.scalar_static_f64[2029]=(if self.scalar_static_bool[296]{self.scalar_static_f64[2028]}else{0.0});
        self.scalar_static_f64[2030]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2029]}else{self.scalar_static_f64[1914]});
        self.scalar_static_f64[2031]=(self.scalar_static_f64[1970]+self.scalar_static_f64[2026]);
        self.scalar_static_f64[2032]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2031]}else{self.scalar_static_f64[1970]});
        self.scalar_static_bool[297]=(self.scalar_static_f64[2032]>0.0);
        self.scalar_static_f64[2033]=(if self.scalar_static_bool[297]{self.scalar_static_f64[2032]}else{0.0});
        self.scalar_static_f64[2034]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2033]}else{self.scalar_static_f64[1972]});
        self.scalar_static_f64[2035]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1476]}else{self.scalar_static_f64[2026]});
        self.scalar_static_f64[2036]=(self.scalar_static_f64[2030]*self.scalar_static_f64[2035]);
        self.scalar_static_f64[2037]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2036]}else{self.scalar_static_f64[1918]});
        self.scalar_static_f64[2038]=(self.scalar_static_f64[2034]*self.scalar_static_f64[2035]);
        self.scalar_static_f64[2039]=(if self.scalar_static_bool[175]{self.scalar_static_f64[2038]}else{self.scalar_static_f64[1976]});
        self.scalar_static_f64[2040]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1537]}else{self.scalar_static_f64[1861]});
        self.scalar_static_f64[2041]=(self.scalar_static_f64[2019]+self.scalar_static_f64[2040]);
        self.scalar_static_f64[2042]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2041]}else{self.scalar_static_f64[2019]});
        self.scalar_static_f64[2043]=(self.scalar_static_f64[2021]+self.scalar_static_f64[2040]);
        self.scalar_static_f64[2044]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2043]}else{self.scalar_static_f64[2021]});
        self.scalar_static_f64[2045]=(self.scalar_static_f64[2023]+self.scalar_static_f64[2040]);
        self.scalar_static_f64[2046]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2045]}else{self.scalar_static_f64[2023]});
        self.scalar_static_f64[2047]=(self.scalar_static_f64[2025]+self.scalar_static_f64[2040]);
        self.scalar_static_f64[2048]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2047]}else{self.scalar_static_f64[2025]});
        self.scalar_static_f64[2049]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1541]}else{self.scalar_static_f64[2040]});
        self.scalar_static_f64[2050]=(self.scalar_static_f64[2028]+self.scalar_static_f64[2049]);
        self.scalar_static_f64[2051]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2050]}else{self.scalar_static_f64[2028]});
        self.scalar_static_bool[298]=(self.scalar_static_f64[2051]>0.0);
        self.scalar_static_f64[2052]=(if self.scalar_static_bool[298]{self.scalar_static_f64[2051]}else{0.0});
        self.scalar_static_f64[2053]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2052]}else{self.scalar_static_f64[2030]});
        self.scalar_static_f64[2054]=(self.scalar_static_f64[2032]+self.scalar_static_f64[2049]);
        self.scalar_static_f64[2055]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2054]}else{self.scalar_static_f64[2032]});
        self.scalar_static_bool[299]=(self.scalar_static_f64[2055]>0.0);
        self.scalar_static_f64[2056]=(if self.scalar_static_bool[299]{self.scalar_static_f64[2055]}else{0.0});
        self.scalar_static_f64[2057]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2056]}else{self.scalar_static_f64[2034]});
        self.scalar_static_f64[2058]=(if self.scalar_static_bool[182]{self.scalar_static_f64[1476]}else{self.scalar_static_f64[2049]});
        self.scalar_static_f64[2059]=(self.scalar_static_f64[2053]*self.scalar_static_f64[2058]);
        self.scalar_static_f64[2060]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2059]}else{self.scalar_static_f64[2037]});
        self.scalar_static_f64[2061]=(self.scalar_static_f64[2057]*self.scalar_static_f64[2058]);
        self.scalar_static_f64[2062]=(if self.scalar_static_bool[182]{self.scalar_static_f64[2061]}else{self.scalar_static_f64[2039]});
        self.scalar_static_f64[2063]=(if self.scalar_static_bool[163]{self.scalar_static_f64[1979]}else{self.scalar_static_f64[1981]});
        self.scalar_static_f64[2064]=(if self.scalar_static_bool[163]{self.scalar_static_f64[1986]}else{self.scalar_static_f64[1991]});
        self.scalar_static_f64[2065]=(self.scalar_static_f64[1891]*0.000473);
        self.scalar_static_f64[2066]=(self.scalar_static_f64[1889]+636.0);
        self.scalar_static_f64[2067]=(self.scalar_static_f64[2065]/self.scalar_static_f64[2066]);
        self.scalar_static_f64[2068]=(1.17-self.scalar_static_f64[2067]);
        self.scalar_static_f64[2069]=(self.scalar_static_f64[1891]*0.0004774);
        self.scalar_static_f64[2070]=(self.scalar_static_f64[1889]+235.0);
        self.scalar_static_f64[2071]=(self.scalar_static_f64[2069]/self.scalar_static_f64[2070]);
        self.scalar_static_f64[2072]=(0.744-self.scalar_static_f64[2071]);
        self.scalar_static_f64[2073]=(self.scalar_static_f64[2072]-self.scalar_static_f64[2068]);
        self.scalar_static_f64[2074]=(self.scalar_static_f64[2073]+self.scalar_static_f64[1553]);
        self.scalar_static_f64[2075]=(self.scalar_static_f64[457]*self.scalar_static_f64[2074]);
        self.scalar_static_f64[2076]=(self.scalar_static_f64[2068]+self.scalar_static_f64[2075]);
        self.scalar_static_f64[2077]=(0.5*self.scalar_static_f64[2076]);
        self.scalar_static_f64[2078]=(self.scalar_static_f64[1895]*self.scalar_static_f64[2077]);
        self.scalar_static_f64[2079]=(0.5*self.scalar_static_f64[2075]);
        self.scalar_static_f64[2080]=(self.scalar_static_f64[1558]-self.scalar_static_f64[2079]);
        self.scalar_static_f64[2081]=(self.scalar_static_f64[1889]*0.0033333333333);
        self.scalar_static_f64[2082]=(self.scalar_static_f64[2081]).sqrt();
        self.scalar_static_f64[2083]=(self.scalar_static_f64[2082]*4.05e25);
        self.scalar_static_f64[2084]=(self.scalar_static_f64[2082]*self.scalar_static_f64[2083]);
        self.scalar_static_f64[2085]=(self.scalar_static_f64[2082]*self.scalar_static_f64[2084]);
        self.scalar_static_f64[2086]=(self.scalar_static_f64[1557]*self.scalar_static_f64[2085]);
        self.scalar_static_f64[2087]=(self.scalar_static_f64[1895]*self.scalar_static_f64[2079]);
        self.scalar_static_f64[2088]=(self.scalar_static_f64[2087]).exp();
        self.scalar_static_f64[2089]=(self.scalar_static_f64[2085]*self.scalar_static_f64[2088]);
        self.scalar_static_f64[2090]=(self.scalar_static_f64[1893]*self.scalar_static_f64[476]);
        self.scalar_static_f64[2091]=(1.0+self.scalar_static_f64[2090]);
        self.scalar_static_f64[2092]=(self.scalar_static_f64[1894]*self.scalar_static_f64[2091]);
        self.scalar_static_f64[2093]=(1.0/self.scalar_static_f64[2092]);
        self.scalar_static_f64[2094]=(self.scalar_static_f64[2077]*self.scalar_static_f64[2093]);
        self.scalar_static_f64[2095]=(self.scalar_static_f64[2086]*3.20435313e-19);
        self.scalar_static_f64[2096]=(self.scalar_static_f64[1552]*self.scalar_static_f64[2095]);
        self.scalar_static_f64[2097]=(self.scalar_static_f64[2093]*self.scalar_static_f64[2096]);
        self.scalar_static_f64[2098]=(self.scalar_static_f64[1593]/self.scalar_static_f64[2097]);
        self.scalar_static_f64[2099]=(self.scalar_static_f64[2098]).ln();
        self.scalar_static_f64[2100]=(self.scalar_static_f64[2099]-0.6931471805599);
        self.scalar_static_f64[2101]=(self.scalar_static_f64[2093]*self.scalar_static_f64[1597]);
        self.scalar_static_f64[2102]=(self.scalar_static_f64[1892]*self.scalar_static_f64[1920]);
        self.scalar_static_f64[2103]=(self.scalar_static_f64[2053]+self.scalar_static_f64[2102]);
        self.scalar_static_f64[2104]=(self.scalar_static_f64[2060]+self.scalar_static_f64[2102]);
        self.scalar_static_f64[2105]=(self.scalar_static_f64[2057]+self.scalar_static_f64[2102]);
        self.scalar_static_f64[2106]=(self.scalar_static_f64[2062]+self.scalar_static_f64[2102]);
        self.scalar_static_f64[2107]=(self.scalar_static_f64[584]*self.scalar_static_f64[2093]);
        self.scalar_static_f64[2108]=(self.scalar_static_f64[1895]*self.scalar_static_f64[1599]);
        self.scalar_static_f64[2109]=(self.scalar_static_f64[2108]).sqrt();
        self.scalar_static_f64[2110]=(self.scalar_static_f64[2109]/self.scalar_static_f64[1584]);
        self.scalar_static_f64[2111]=(self.scalar_static_f64[2110]*self.scalar_static_f64[2110]);
        self.scalar_static_f64[2112]=(1.0/self.scalar_static_f64[2111]);
        self.scalar_static_f64[2113]=(self.scalar_static_f64[2110]/1.4142135623731);
        self.scalar_static_f64[2114]=(1.0+self.scalar_static_f64[2113]);
        self.scalar_static_f64[2115]=(1.0/self.scalar_static_f64[2114]);
        self.scalar_static_f64[2116]=(self.scalar_static_f64[2114]*1e-5);
        self.scalar_static_f64[2117]=(self.scalar_static_f64[474]/self.scalar_static_f64[2089]);
        self.scalar_static_f64[2118]=(self.scalar_static_f64[2117]).ln();
        self.scalar_static_f64[2119]=(self.scalar_static_f64[2078]+self.scalar_static_f64[2118]);
        self.scalar_static_f64[2120]=(2.0*self.scalar_static_f64[2119]);
        self.scalar_static_f64[2121]=(self.scalar_static_f64[1894]*self.scalar_static_f64[469]);
        self.scalar_static_f64[2122]=(self.scalar_static_f64[2119]*self.scalar_static_f64[2121]);
        self.scalar_static_f64[2123]=(self.scalar_static_f64[2044]+self.scalar_static_f64[2122]);
        self.scalar_static_f64[2124]=(if self.scalar_static_bool[198]{self.scalar_static_f64[2123]}else{self.scalar_static_f64[2044]});
        self.scalar_static_f64[2125]=(self.scalar_static_f64[2048]+self.scalar_static_f64[2122]);
        self.scalar_static_f64[2126]=(if self.scalar_static_bool[198]{self.scalar_static_f64[2125]}else{self.scalar_static_f64[2048]});
        self.scalar_static_f64[2127]=(self.scalar_static_f64[527]/self.scalar_static_f64[2089]);
        self.scalar_static_f64[2128]=(self.scalar_static_f64[2127]).ln();
        self.scalar_static_f64[2129]=(self.scalar_static_f64[2078]+self.scalar_static_f64[2128]);
        self.scalar_static_f64[2130]=(self.scalar_static_f64[1894]*self.scalar_static_f64[2129]);
        self.scalar_static_f64[2131]=(if self.scalar_static_bool[199]{self.scalar_static_f64[2130]}else{0.0});
        self.scalar_static_f64[2132]=(2970.0/self.scalar_static_f64[1889]);
        self.scalar_static_f64[2133]=(15.0+self.scalar_static_f64[2132]);
        self.scalar_static_f64[2134]=(15.0-self.scalar_static_f64[2132]);
        self.scalar_static_f64[2135]=(self.scalar_static_f64[2134]*self.scalar_static_f64[2134]);
        self.scalar_static_f64[2136]=(1e-6+self.scalar_static_f64[2135]);
        self.scalar_static_f64[2137]=(self.scalar_static_f64[2136]).sqrt();
        self.scalar_static_f64[2138]=(self.scalar_static_f64[2133]+self.scalar_static_f64[2137]);
        self.scalar_static_f64[2139]=(0.5*self.scalar_static_f64[2138]);
        self.scalar_static_f64[2140]=(if self.scalar_static_bool[0]{self.scalar_static_f64[2139]}else{15.0});
        self.scalar_static_f64[2141]=(self.scalar_static_f64[2092]*self.scalar_static_f64[1607]);
        self.scalar_static_f64[2142]=(self.scalar_static_f64[2141]).ln();
        self.scalar_static_f64[2143]=(-0.3333333333333*self.scalar_static_f64[2142]);
        self.scalar_static_f64[2144]=(self.scalar_static_f64[2143]).exp();
        self.scalar_static_f64[2145]=(self.scalar_static_f64[1612]*self.scalar_static_f64[2144]);
        self.scalar_static_f64[2146]=(if self.scalar_static_bool[202]{self.scalar_static_f64[2145]}else{0.0});
        self.scalar_static_f64[2147]=(self.scalar_static_f64[2144]*self.scalar_static_f64[1615]);
        self.scalar_static_f64[2148]=(if self.scalar_static_bool[204]{self.scalar_static_f64[2147]}else{self.scalar_static_f64[2146]});
        self.scalar_static_f64[2149]=(self.scalar_static_f64[1892]*self.scalar_static_f64[1616]);
        self.scalar_static_f64[2150]=(self.scalar_static_f64[1614]+self.scalar_static_f64[2149]);
        self.scalar_static_f64[2151]=(self.scalar_static_f64[2150]+self.scalar_static_f64[1617]);
        self.scalar_static_f64[2152]=(self.scalar_static_f64[2151]-self.scalar_static_f64[2131]);
        self.scalar_static_f64[2153]=(self.scalar_static_f64[2042]+self.scalar_static_f64[2080]);
        self.scalar_static_f64[2154]=(self.scalar_static_f64[1573]+self.scalar_static_f64[2153]);
        self.scalar_static_f64[2155]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2154]);
        self.scalar_static_f64[2156]=(self.scalar_static_f64[2152]+self.scalar_static_f64[2155]);
        self.scalar_static_f64[2157]=(self.scalar_static_f64[2080]+self.scalar_static_f64[2124]);
        self.scalar_static_f64[2158]=(self.scalar_static_f64[1575]+self.scalar_static_f64[2157]);
        self.scalar_static_f64[2159]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2158]);
        self.scalar_static_f64[2160]=(self.scalar_static_f64[2150]+self.scalar_static_f64[2159]);
        self.scalar_static_f64[2161]=(self.scalar_static_f64[2046]+self.scalar_static_f64[2080]);
        self.scalar_static_f64[2162]=(self.scalar_static_f64[1573]+self.scalar_static_f64[2161]);
        self.scalar_static_f64[2163]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2162]);
        self.scalar_static_f64[2164]=(self.scalar_static_f64[2152]+self.scalar_static_f64[2163]);
        self.scalar_static_f64[2165]=(self.scalar_static_f64[2080]+self.scalar_static_f64[2126]);
        self.scalar_static_f64[2166]=(self.scalar_static_f64[1575]+self.scalar_static_f64[2165]);
        self.scalar_static_f64[2167]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2166]);
        self.scalar_static_f64[2168]=(self.scalar_static_f64[2150]+self.scalar_static_f64[2167]);
        self.scalar_static_f64[2169]=(self.scalar_static_f64[1893]).ln();
        self.scalar_static_f64[2170]=(self.scalar_static_f64[687]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2171]=(self.scalar_static_f64[2170]).exp();
        self.scalar_static_f64[2172]=(self.scalar_static_f64[2171]*self.scalar_static_f64[1618]);
        self.scalar_static_f64[2173]=(self.scalar_static_f64[1524]*self.scalar_static_f64[2172]);
        self.scalar_static_f64[2174]=(self.scalar_static_f64[1526]*self.scalar_static_f64[2172]);
        self.scalar_static_f64[2175]=(self.scalar_static_f64[734]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2176]=(self.scalar_static_f64[2175]).exp();
        self.scalar_static_f64[2177]=(self.scalar_static_f64[732]*self.scalar_static_f64[2176]);
        self.scalar_static_f64[2178]=(self.scalar_static_f64[738]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2179]=(self.scalar_static_f64[2178]).exp();
        self.scalar_static_f64[2180]=(self.scalar_static_f64[736]*self.scalar_static_f64[2179]);
        self.scalar_static_f64[2181]=(self.scalar_static_f64[722]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2182]=(self.scalar_static_f64[2181]).exp();
        self.scalar_static_f64[2183]=(self.scalar_static_f64[704]*self.scalar_static_f64[2182]);
        self.scalar_static_f64[2184]=(self.scalar_static_f64[726]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2185]=(self.scalar_static_f64[2184]).exp();
        self.scalar_static_f64[2186]=(self.scalar_static_f64[724]*self.scalar_static_f64[2185]);
        self.scalar_static_f64[2187]=(self.scalar_static_f64[757]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2188]=(self.scalar_static_f64[2187]).exp();
        self.scalar_static_f64[2189]=(self.scalar_static_f64[753]*self.scalar_static_f64[2188]);
        self.scalar_static_f64[2190]=(self.scalar_static_f64[2092]*1e-8);
        self.scalar_static_f64[2191]=(self.scalar_static_f64[2190]/self.scalar_static_f64[455]);
        self.scalar_static_f64[2192]=(self.scalar_static_f64[2177]*self.scalar_static_f64[2191]);
        self.scalar_static_f64[2193]=(self.scalar_static_f64[772]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2194]=(self.scalar_static_f64[2193]).exp();
        self.scalar_static_f64[2195]=(self.scalar_static_f64[768]*self.scalar_static_f64[2194]);
        self.scalar_static_f64[2196]=(2.0*self.scalar_static_f64[2195]);
        self.scalar_static_f64[2197]=(self.scalar_static_f64[2092]*self.scalar_static_f64[2196]);
        self.scalar_static_f64[2198]=(self.scalar_static_f64[810]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2199]=(self.scalar_static_f64[2198]).exp();
        self.scalar_static_f64[2200]=(self.scalar_static_f64[1530]*self.scalar_static_f64[2199]);
        self.scalar_static_f64[2201]=(self.scalar_static_f64[2172]*self.scalar_static_f64[2200]);
        self.scalar_static_f64[2202]=(self.scalar_static_f64[2092]*self.scalar_static_f64[2201]);
        self.scalar_static_f64[2203]=(self.scalar_static_f64[1534]*self.scalar_static_f64[2199]);
        self.scalar_static_f64[2204]=(self.scalar_static_f64[2172]*self.scalar_static_f64[2203]);
        self.scalar_static_f64[2205]=(self.scalar_static_f64[2092]*self.scalar_static_f64[2204]);
        self.scalar_static_f64[2206]=(self.scalar_static_f64[865]*self.scalar_static_f64[2093]);
        self.scalar_static_f64[2207]=(self.scalar_static_f64[2169]*self.scalar_static_f64[1643]);
        self.scalar_static_f64[2208]=(self.scalar_static_f64[2207]).exp();
        self.scalar_static_f64[2209]=(self.scalar_static_f64[876]*self.scalar_static_f64[2208]);
        self.scalar_static_f64[2210]=(self.scalar_static_f64[879]*self.scalar_static_f64[2208]);
        self.scalar_static_f64[2211]=(self.scalar_static_f64[1543]*self.scalar_static_f64[2208]);
        self.scalar_static_f64[2212]=(self.scalar_static_f64[891]*self.scalar_static_f64[2208]);
        self.scalar_static_f64[2213]=(self.scalar_static_f64[1545]*self.scalar_static_f64[2208]);
        self.scalar_static_f64[2214]=(self.scalar_static_f64[2169]*self.scalar_static_f64[1644]);
        self.scalar_static_f64[2215]=(self.scalar_static_f64[2214]).exp();
        self.scalar_static_f64[2216]=(self.scalar_static_f64[885]*self.scalar_static_f64[2215]);
        self.scalar_static_f64[2217]=(self.scalar_static_f64[1544]*self.scalar_static_f64[2215]);
        self.scalar_static_f64[2218]=(self.scalar_static_f64[873]*self.scalar_static_f64[2092]);
        self.scalar_static_f64[2219]=(self.scalar_static_f64[1894]*self.scalar_static_f64[873]);
        self.scalar_static_f64[2220]=(self.scalar_static_f64[921]*self.scalar_static_f64[2094]);
        self.scalar_static_f64[2221]=(1.0+self.scalar_static_f64[2220]);
        self.scalar_static_f64[2222]=(1.0/self.scalar_static_f64[2221]);
        self.scalar_static_f64[2223]=(self.scalar_static_f64[1892]*self.scalar_static_f64[941]);
        self.scalar_static_f64[2224]=(1.0+self.scalar_static_f64[2223]);
        self.scalar_static_f64[2225]=(self.scalar_static_f64[2224]*self.scalar_static_f64[2224]);
        self.scalar_static_f64[2226]=(0.01+self.scalar_static_f64[2225]);
        self.scalar_static_f64[2227]=(self.scalar_static_f64[2226]).sqrt();
        self.scalar_static_f64[2228]=(self.scalar_static_f64[2224]+self.scalar_static_f64[2227]);
        self.scalar_static_f64[2229]=(0.5*self.scalar_static_f64[2228]);
        self.scalar_static_f64[2230]=(self.scalar_static_f64[937]*self.scalar_static_f64[2229]);
        self.scalar_static_f64[2231]=(self.scalar_static_f64[1664]*self.scalar_static_f64[2230]);
        self.scalar_static_f64[2232]=(self.scalar_static_f64[1892]*self.scalar_static_f64[1548]);
        self.scalar_static_f64[2233]=(1.0+self.scalar_static_f64[2232]);
        self.scalar_static_f64[2234]=(self.scalar_static_f64[2233]*self.scalar_static_f64[2233]);
        self.scalar_static_f64[2235]=(0.01+self.scalar_static_f64[2234]);
        self.scalar_static_f64[2236]=(self.scalar_static_f64[2235]).sqrt();
        self.scalar_static_f64[2237]=(self.scalar_static_f64[2233]+self.scalar_static_f64[2236]);
        self.scalar_static_f64[2238]=(0.5*self.scalar_static_f64[2237]);
        self.scalar_static_f64[2239]=(self.scalar_static_f64[1547]*self.scalar_static_f64[2238]);
        self.scalar_static_f64[2240]=(self.scalar_static_f64[1664]*self.scalar_static_f64[2239]);
        self.scalar_static_f64[2241]=(self.scalar_static_f64[2169]*self.scalar_static_f64[1665]);
        self.scalar_static_f64[2242]=(self.scalar_static_f64[2241]).exp();
        self.scalar_static_f64[2243]=(self.scalar_static_f64[971]*self.scalar_static_f64[2242]);
        self.scalar_static_f64[2244]=(self.scalar_static_f64[1893]*self.scalar_static_f64[993]);
        self.scalar_static_f64[2245]=(1.0+self.scalar_static_f64[2244]);
        self.scalar_static_f64[2246]=(self.scalar_static_f64[1894]*self.scalar_static_f64[2245]);
        self.scalar_static_f64[2247]=(1.0/self.scalar_static_f64[2246]);
        self.scalar_static_f64[2248]=(self.scalar_static_f64[2096]*self.scalar_static_f64[2247]);
        self.scalar_static_f64[2249]=(self.scalar_static_f64[1892]*self.scalar_static_f64[1666]);
        self.scalar_static_f64[2250]=(self.scalar_static_f64[1614]+self.scalar_static_f64[2249]);
        self.scalar_static_f64[2251]=(self.scalar_static_f64[1925]+self.scalar_static_f64[2080]);
        self.scalar_static_f64[2252]=(self.scalar_static_f64[1573]+self.scalar_static_f64[2251]);
        self.scalar_static_f64[2253]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2252]);
        self.scalar_static_f64[2254]=(self.scalar_static_f64[2250]+self.scalar_static_f64[2253]);
        self.scalar_static_f64[2255]=(self.scalar_static_f64[1617]+self.scalar_static_f64[2254]);
        self.scalar_static_f64[2256]=(self.scalar_static_f64[2255]-self.scalar_static_f64[2131]);
        self.scalar_static_f64[2257]=(self.scalar_static_f64[1004]+self.scalar_static_f64[2080]);
        self.scalar_static_f64[2258]=(self.scalar_static_f64[1575]+self.scalar_static_f64[2257]);
        self.scalar_static_f64[2259]=(self.scalar_static_f64[1608]*self.scalar_static_f64[2258]);
        self.scalar_static_f64[2260]=(self.scalar_static_f64[2250]+self.scalar_static_f64[2259]);
        self.scalar_static_f64[2261]=(self.scalar_static_f64[1067]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2262]=(self.scalar_static_f64[2261]).exp();
        self.scalar_static_f64[2263]=(self.scalar_static_f64[1618]*self.scalar_static_f64[2262]);
        self.scalar_static_f64[2264]=(self.scalar_static_f64[1949]*self.scalar_static_f64[2263]);
        self.scalar_static_f64[2265]=(self.scalar_static_f64[1069]*self.scalar_static_f64[2092]);
        self.scalar_static_f64[2266]=(self.scalar_static_f64[1552]*self.scalar_static_f64[2092]);
        self.scalar_static_f64[2267]=(self.scalar_static_f64[1667]/self.scalar_static_f64[2266]);
        self.scalar_static_f64[2268]=(self.scalar_static_f64[1074]/self.scalar_static_f64[2086]);
        self.scalar_static_f64[2269]=(self.scalar_static_f64[2268]).ln();
        self.scalar_static_f64[2270]=(self.scalar_static_f64[2092]*self.scalar_static_f64[1668]);
        self.scalar_static_f64[2271]=(self.scalar_static_f64[1349]*self.scalar_static_f64[2169]);
        self.scalar_static_f64[2272]=(self.scalar_static_f64[2271]).exp();
        self.scalar_static_f64[2273]=(self.scalar_static_f64[2006]*self.scalar_static_f64[2272]);
        self.scalar_static_f64[2274]=(self.scalar_static_f64[1889]*5.5225952e-23);
        self.scalar_static_f64[2275]=(self.scalar_static_f64[1352]*self.scalar_static_f64[2274]);
        self.scalar_static_f64[2276]=(10.0/self.scalar_static_f64[1894]);
        self.scalar_static_f64[2277]=(600.0+self.scalar_static_f64[2276]);
        self.scalar_static_f64[2278]=(self.scalar_static_f64[2276]-600.0);
        self.scalar_static_f64[2279]=(self.scalar_static_f64[2278]*self.scalar_static_f64[2278]);
        self.scalar_static_f64[2280]=(0.01+self.scalar_static_f64[2279]);
        self.scalar_static_f64[2281]=(self.scalar_static_f64[2280]).sqrt();
        self.scalar_static_f64[2282]=(self.scalar_static_f64[2277]+self.scalar_static_f64[2281]);
        self.scalar_static_f64[2283]=(0.5*self.scalar_static_f64[2282]);
        self.scalar_static_f64[2284]=(if self.scalar_static_bool[217]{self.scalar_static_f64[2283]}else{self.scalar_static_f64[1890]});
        self.scalar_static_f64[2285]=(if self.scalar_static_bool[218]{600.0}else{self.scalar_static_f64[2284]});
        self.scalar_static_f64[2286]=(if self.scalar_static_bool[217]{self.scalar_static_f64[2139]}else{self.scalar_static_f64[2140]});
        self.scalar_static_f64[2287]=(if self.scalar_static_bool[216]{0.0}else{self.scalar_static_f64[2148]});
        self.scalar_static_f64[2288]=(-self.scalar_static_f64[2120]);
        self.scalar_static_f64[2289]=(self.scalar_static_f64[2288]).abs();
        self.scalar_static_bool[300]=(self.scalar_static_f64[2289]<80.0);
        self.scalar_static_bool[301]=(self.scalar_static_bool[198]&&self.scalar_static_bool[300]);
        self.scalar_static_f64[2290]=(self.scalar_static_f64[2288]).exp();
        self.scalar_static_f64[2291]=(if self.scalar_static_bool[301]{self.scalar_static_f64[2290]}else{0.0});
        self.scalar_static_bool[302]=(self.scalar_static_f64[2288]< -80.0);
        self.scalar_static_bool[303]=(!self.scalar_static_bool[300]);
        self.scalar_static_bool[304]=(self.scalar_static_bool[198]&&self.scalar_static_bool[303]);
        self.scalar_static_bool[305]=(self.scalar_static_bool[302]&&self.scalar_static_bool[304]);
        self.scalar_static_f64[2292]=(self.scalar_static_f64[2120]-80.0);
        self.scalar_static_f64[2293]=(0.5*self.scalar_static_f64[2292]);
        self.scalar_static_f64[2294]=(0.3333333333333*self.scalar_static_f64[2292]);
        self.scalar_static_f64[2295]=(1.0+self.scalar_static_f64[2294]);
        self.scalar_static_f64[2296]=(self.scalar_static_f64[2293]*self.scalar_static_f64[2295]);
        self.scalar_static_f64[2297]=(1.0+self.scalar_static_f64[2296]);
        self.scalar_static_f64[2298]=(self.scalar_static_f64[2292]*self.scalar_static_f64[2297]);
        self.scalar_static_f64[2299]=(1.0+self.scalar_static_f64[2298]);
        self.scalar_static_f64[2300]=(1.80485e-35/self.scalar_static_f64[2299]);
        self.scalar_static_f64[2301]=(if self.scalar_static_bool[305]{self.scalar_static_f64[2300]}else{self.scalar_static_f64[2291]});
        self.scalar_static_bool[306]=(!self.scalar_static_bool[302]);
        self.scalar_static_bool[307]=(self.scalar_static_bool[304]&&self.scalar_static_bool[306]);
        self.scalar_static_f64[2302]=(self.scalar_static_f64[2288]-80.0);
        self.scalar_static_f64[2303]=(0.5*self.scalar_static_f64[2302]);
        self.scalar_static_f64[2304]=(0.3333333333333*self.scalar_static_f64[2302]);
        self.scalar_static_f64[2305]=(1.0+self.scalar_static_f64[2304]);
        self.scalar_static_f64[2306]=(self.scalar_static_f64[2303]*self.scalar_static_f64[2305]);
        self.scalar_static_f64[2307]=(1.0+self.scalar_static_f64[2306]);
        self.scalar_static_f64[2308]=(self.scalar_static_f64[2302]*self.scalar_static_f64[2307]);
        self.scalar_static_f64[2309]=(1.0+self.scalar_static_f64[2308]);
        self.scalar_static_f64[2310]=(5.54062e34*self.scalar_static_f64[2309]);
        self.scalar_static_f64[2311]=(if self.scalar_static_bool[307]{self.scalar_static_f64[2310]}else{self.scalar_static_f64[2301]});
        self.scalar_static_f64[2312]=(self.scalar_static_f64[2115]*self.scalar_static_f64[2115]);
        self.scalar_static_f64[2313]=(self.scalar_static_f64[2312]*0.1666666666667);
        self.scalar_static_f64[2314]=(self.scalar_static_f64[2313]/1.4142135623731);
        self.scalar_static_f64[2315]=(1.0-self.scalar_static_f64[2311]);
        self.scalar_static_f64[2316]=(-self.scalar_static_f64[2116]);
        self.scalar_static_f64[2317]=(self.scalar_static_f64[2110]*0.732464877560822);
        self.scalar_static_f64[2318]=(1.25+self.scalar_static_f64[2317]);
        self.scalar_static_f64[2319]=(1.0/self.scalar_static_f64[2318]);
        self.scalar_static_f64[2320]=(self.scalar_static_f64[2114]*1.25);
        self.scalar_static_f64[2321]=(0.5*self.scalar_static_f64[2111]);
        self.scalar_static_f64[2322]=(self.scalar_static_f64[2111]*0.25);
        self.scalar_static_f64[2323]=(self.scalar_static_f64[2120]+3.0);
        self.scalar_static_f64[2324]=(self.scalar_static_f64[2286]*self.scalar_static_f64[2286]);
        self.scalar_static_bool[308]=(self.scalar_static_f64[1979]>0.0);
        self.scalar_static_bool[309]=(self.scalar_static_f64[2063]>0.0);
        self.scalar_static_f64[2325]=(1.0+self.scalar_static_f64[1929]);
        self.scalar_static_f64[2326]=(1.0/self.scalar_static_f64[2325]);
        self.scalar_static_f64[2327]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2326]}else{0.0});
        self.scalar_static_f64[2328]=(1.0+self.scalar_static_f64[1933]);
        self.scalar_static_f64[2329]=(1.0/self.scalar_static_f64[2328]);
        self.scalar_static_f64[2330]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2329]}else{0.0});
        self.scalar_static_f64[2331]=(self.scalar_static_f64[1042]*self.scalar_static_f64[2247]);
        self.scalar_static_f64[2332]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2331]}else{0.0});
        self.scalar_static_f64[2333]=(2.0*self.scalar_static_f64[2332]);
        self.scalar_static_f64[2334]=(self.scalar_static_f64[1586]/self.scalar_static_f64[2327]);
        self.scalar_static_f64[2335]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2334]}else{0.0});
        self.scalar_static_f64[2336]=(self.scalar_static_f64[1587]/self.scalar_static_f64[2330]);
        self.scalar_static_f64[2337]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2336]}else{0.0});
        self.scalar_static_f64[2338]=(1.0/self.scalar_static_f64[2335]);
        self.scalar_static_f64[2339]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2338]}else{0.0});
        self.scalar_static_f64[2340]=(1.0/self.scalar_static_f64[2337]);
        self.scalar_static_f64[2341]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2340]}else{0.0});
        self.scalar_static_f64[2342]=(1.0+self.scalar_static_f64[2339]);
        self.scalar_static_f64[2343]=(self.scalar_static_f64[2341]+self.scalar_static_f64[2342]);
        self.scalar_static_f64[2344]=(1.0/self.scalar_static_f64[2343]);
        self.scalar_static_f64[2345]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2344]}else{0.0});
        self.scalar_static_f64[2346]=(self.scalar_static_f64[2339]*self.scalar_static_f64[2345]);
        self.scalar_static_f64[2347]=(1.0-self.scalar_static_f64[2346]);
        self.scalar_static_f64[2348]=(self.scalar_static_f64[2341]*self.scalar_static_f64[2345]);
        self.scalar_static_f64[2349]=(self.scalar_static_f64[2347]-self.scalar_static_f64[2348]);
        self.scalar_static_f64[2350]=(0.5*self.scalar_static_f64[2339]);
        self.scalar_static_f64[2351]=(self.scalar_static_f64[2345]*self.scalar_static_f64[2350]);
        self.scalar_static_f64[2352]=(self.scalar_static_f64[2339]*self.scalar_static_f64[2351]);
        self.scalar_static_f64[2353]=(self.scalar_static_f64[2341]+self.scalar_static_f64[2352]);
        self.scalar_static_f64[2354]=(0.5*self.scalar_static_f64[2341]);
        self.scalar_static_f64[2355]=(self.scalar_static_f64[2345]*self.scalar_static_f64[2354]);
        self.scalar_static_f64[2356]=(self.scalar_static_f64[2341]*self.scalar_static_f64[2355]);
        self.scalar_static_f64[2357]=(self.scalar_static_f64[2353]-self.scalar_static_f64[2356]);
        self.scalar_static_f64[2358]=(0.5/self.scalar_static_f64[2345]);
        self.scalar_static_f64[2359]=(self.scalar_static_f64[2357]-self.scalar_static_f64[2358]);
        self.scalar_static_f64[2360]=(-self.scalar_static_f64[2339]);
        self.scalar_static_f64[2361]=(1.0/self.scalar_static_f64[2345]);
        self.scalar_static_f64[2362]=(self.scalar_static_f64[2341]-self.scalar_static_f64[2361]);
        self.scalar_static_f64[2363]=(self.scalar_static_f64[2246]*self.scalar_static_f64[2246]);
        self.scalar_static_f64[2364]=(self.scalar_static_f64[2264]*self.scalar_static_f64[2363]);
        self.scalar_static_f64[2365]=(if self.scalar_static_bool[250]{self.scalar_static_f64[2364]}else{0.0});
        self.scalar_static_f64[2366]=(self.scalar_static_f64[1581]*self.scalar_static_f64[2365]);
        self.scalar_static_bool[310]=(self.scalar_static_bool[300]&&self.scalar_static_bool[253]);
        self.scalar_static_f64[2367]=(if self.scalar_static_bool[310]{self.scalar_static_f64[2290]}else{0.0});
        self.scalar_static_bool[311]=(self.scalar_static_bool[303]&&self.scalar_static_bool[253]);
        self.scalar_static_bool[312]=(self.scalar_static_bool[302]&&self.scalar_static_bool[311]);
        self.scalar_static_f64[2368]=(if self.scalar_static_bool[312]{self.scalar_static_f64[2300]}else{self.scalar_static_f64[2367]});
        self.scalar_static_bool[313]=(self.scalar_static_bool[306]&&self.scalar_static_bool[311]);
        self.scalar_static_f64[2369]=(if self.scalar_static_bool[313]{self.scalar_static_f64[2310]}else{self.scalar_static_f64[2368]});
        self.scalar_static_f64[2370]=(1.0-self.scalar_static_f64[2369]);
        self.scalar_static_f64[2371]=(self.scalar_static_f64[1920]*self.scalar_static_f64[1795]);
        self.scalar_static_f64[2372]=(self.scalar_static_f64[2129]*self.scalar_static_f64[1799]);
        self.scalar_static_f64[2373]=(if self.scalar_static_bool[219]{self.scalar_static_f64[2372]}else{0.0});
        self.scalar_static_f64[2374]=(self.scalar_static_f64[1986]*self.scalar_static_f64[1809]);
        self.scalar_static_f64[2375]=(self.scalar_static_f64[1986]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[2376]=(self.scalar_static_f64[2064]*self.scalar_static_f64[1813]);
        self.scalar_static_f64[2377]=(self.scalar_static_f64[2064]*self.scalar_static_f64[1812]);
        self.scalar_static_f64[2378]=(self.scalar_static_f64[2064]*self.scalar_static_f64[1810]);
        self.scalar_static_f64[2379]=(self.scalar_static_f64[2010]*self.scalar_static_f64[1793]);
        self.scalar_static_f64[2380]=(if self.scalar_static_bool[216]{self.scalar_static_f64[2379]}else{0.0});
        self.scalar_static_f64[2381]=(if self.scalar_static_bool[252]{0.0}else{self.scalar_static_f64[2380]});
        self.scalar_static_f64[2382]=(self.scalar_static_f64[1783]*self.scalar_static_f64[2374]);
        self.scalar_static_f64[2383]=(self.scalar_static_f64[1783]*self.scalar_static_f64[2375]);
        self.scalar_static_f64[2384]=(self.scalar_static_f64[1783]*self.scalar_static_f64[2376]);
        self.scalar_static_f64[2385]=(self.scalar_static_f64[1783]*self.scalar_static_f64[2377]);
        self.scalar_static_f64[2386]=(self.scalar_static_f64[1783]*self.scalar_static_f64[2378]);
        self.scalar_static_f64[2387]=(self.scalar_static_f64[386]*self.scalar_static_f64[2381]);
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
