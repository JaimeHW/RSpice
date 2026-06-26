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
    pub p503: f64,
    pub p504: f64,
    pub p505: f64,
    pub p506: f64,
    pub p507: f64,
    pub p508: f64,
    pub p509: f64,
    pub p510: f64,
    pub p511: f64,
    pub p512: f64,
    pub p513: f64,
    pub p514: f64,
    pub p515: f64,
    pub p516: f64,
    pub p517: f64,
    pub p518: f64,
    pub p519: f64,
    pub p520: f64,
    pub p521: f64,
    pub p522: f64,
    pub p523: f64,
    pub p524: f64,
    pub p525: f64,
    pub p526: f64,
    pub p527: f64,
    pub p528: f64,
    pub p529: f64,
    pub p530: f64,
    pub p531: f64,
    pub p532: f64,
    pub p533: f64,
    pub p534: f64,
    pub p535: f64,
    pub p536: f64,
    pub p537: f64,
    pub p538: f64,
    pub p539: f64,
    pub p540: f64,
    pub p541: f64,
    pub p542: f64,
    pub p543: f64,
    pub p544: f64,
    pub p545: f64,
    pub p546: f64,
    pub p547: f64,
    pub p548: f64,
    pub p549: f64,
    pub p550: f64,
    pub p551: f64,
    pub p552: f64,
    pub p553: f64,
    pub p554: f64,
    pub p555: f64,
    pub p556: f64,
    pub p557: f64,
    pub p558: f64,
    pub p559: f64,
    pub p560: f64,
    pub p561: f64,
    pub p562: f64,
    pub p563: f64,
    pub p564: f64,
    pub p565: f64,
    pub p566: f64,
    pub p567: f64,
    pub p568: f64,
    pub p569: f64,
    pub p570: f64,
    pub p571: f64,
    pub p572: f64,
    pub p573: f64,
    pub p574: f64,
    pub p575: f64,
    pub p576: f64,
    pub p577: f64,
    pub p578: f64,
    pub p579: f64,
    pub p580: f64,
    pub p581: f64,
    pub p582: f64,
    pub p583: f64,
    pub p584: f64,
    pub p585: f64,
    pub p586: f64,
    pub p587: f64,
    pub p588: f64,
    pub p589: f64,
    pub p590: f64,
    pub p591: f64,
    pub p592: f64,
    pub p593: f64,
    pub p594: f64,
    pub p595: f64,
    pub p596: f64,
    pub p597: f64,
    pub p598: f64,
    pub p599: f64,
    pub p600: f64,
    pub p601: f64,
    pub p602: f64,
    pub p603: f64,
    pub p604: f64,
    pub p605: f64,
    pub p606: f64,
    pub p607: f64,
    pub p608: f64,
    pub p609: f64,
    pub p610: f64,
    pub p611: f64,
    pub p612: f64,
    pub p613: f64,
    pub p614: f64,
    pub p615: f64,
    pub p616: f64,
    pub p617: f64,
    pub p618: f64,
    pub p619: f64,
    pub p620: f64,
    pub p621: f64,
    pub p622: f64,
    pub p623: f64,
    pub p624: f64,
    pub p625: f64,
    pub p626: f64,
    pub p627: f64,
    pub p628: f64,
    pub p629: f64,
    pub p630: f64,
    pub p631: f64,
    pub p632: f64,
    pub p633: f64,
    pub p634: f64,
    pub p635: f64,
    pub p636: f64,
    pub p637: f64,
    pub p638: f64,
    pub p639: f64,
    pub p640: f64,
    pub p641: f64,
    pub p642: f64,
    pub p643: f64,
    pub p644: f64,
    pub p645: f64,
    pub p646: f64,
    pub p647: f64,
    pub p648: f64,
    pub p649: f64,
    pub p650: f64,
    pub p651: f64,
    pub p652: f64,
    pub p653: f64,
    pub p654: f64,
    pub p655: f64,
    pub p656: f64,
    pub p657: f64,
    pub p658: f64,
    pub p659: f64,
    pub p660: f64,
    pub p661: f64,
    pub p662: f64,
    pub p663: f64,
    pub p664: f64,
    pub p665: f64,
    pub p666: f64,
    pub p667: f64,
    pub p668: f64,
    pub p669: f64,
    pub p670: f64,
    pub p671: f64,
    pub p672: f64,
    pub p673: f64,
    pub p674: f64,
    pub p675: f64,
    pub p676: f64,
    pub p677: f64,
    pub p678: f64,
    pub p679: f64,
    pub p680: f64,
    pub p681: f64,
    pub p682: f64,
    pub p683: f64,
    pub p684: f64,
    pub p685: f64,
    pub p686: f64,
    pub p687: f64,
    pub p688: f64,
    pub p689: f64,
    pub p690: f64,
    pub p691: f64,
    pub p692: f64,
    pub p693: f64,
    pub p694: f64,
    pub p695: f64,
    pub p696: f64,
    pub p697: f64,
    pub p698: f64,
    pub p699: f64,
    pub p700: f64,
    pub p701: f64,
    pub p702: f64,
    pub p703: f64,
    pub p704: f64,
    pub p705: f64,
    pub p706: f64,
    pub p707: f64,
    pub p708: f64,
    pub p709: f64,
    pub p710: f64,
    pub p711: f64,
    pub p712: f64,
    pub p713: f64,
    pub p714: f64,
    pub p715: f64,
    pub p716: f64,
    pub p717: f64,
    pub p718: f64,
    pub p719: f64,
    pub p720: f64,
    pub p721: f64,
    pub p722: f64,
    pub p723: f64,
    pub p724: f64,
    pub p725: f64,
    pub p726: f64,
    pub p727: f64,
    pub p728: f64,
    pub p729: f64,
    pub p730: f64,
    pub p731: f64,
    pub p732: f64,
    pub p733: f64,
    pub p734: f64,
    pub p735: f64,
    pub p736: f64,
    pub p737: f64,
    pub p738: f64,
    pub p739: f64,
    pub p740: f64,
    pub p741: f64,
    pub p742: f64,
    pub p743: f64,
    pub p744: f64,
    pub p745: f64,
    pub p746: f64,
    pub p747: f64,
    pub p748: f64,
    pub p749: f64,
    pub p750: f64,
    pub p751: f64,
    pub p752: f64,
    pub p753: f64,
    pub p754: f64,
    pub p755: f64,
    pub p756: f64,
    pub p757: f64,
    pub p758: f64,
    pub p759: f64,
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
        params.p0 = 3e-8;
        params.p1 = 1e-6;
        params.p2 = 1.0;
        params.p3 = 0.0;
        params.p4 = 0.0;
        params.p5 = 0.0;
        params.p6 = 0.0;
        params.p7 = 0.0;
        params.p8 = 0.0;
        params.p9 = 0.0;
        params.p10 = 0.0;
        params.p11 = 1.0;
        params.p12 = 1.0;
        params.p13 = (-params.p12);
        validate_parameter("WELLTYPE", params.p13, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")]).expect("generated Verilog-A parameter default must satisfy declared range");
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
        params.p27 = 0.0;
        params.p28 = 0.0;
        params.p29 = 1.0;
        params.p30 = 1.0;
        params.p31 = 0.0;
        params.p32 = 0.0;
        params.p33 = 0.0;
        params.p34 = 0.0;
        params.p35 = 1.0;
        params.p36 = 1.0;
        params.p37 = 0.0;
        params.p38 = 0.0;
        params.p39 = 0.0;
        params.p40 = 0.0;
        params.p41 = 0.0;
        params.p42 = 0.0;
        params.p43 = 0.0;
        params.p44 = 0.0;
        params.p45 = 1e-9;
        params.p46 = 1e-8;
        params.p47 = params.p45;
        validate_parameter("EOT1P", params.p47, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p48 = 0.0;
        params.p49 = 8e-9;
        params.p50 = 1e22;
        params.p51 = 2e26;
        params.p52 = 5e23;
        params.p53 = 4.05;
        params.p54 = 1.1e16;
        params.p55 = 1.12;
        params.p56 = 2.86e25;
        params.p57 = 4.61;
        params.p58 = if (params.p13 == (-1.0)) { (params.p53 + params.p55) } else { params.p53 };
        validate_parameter("PHIG2", params.p58, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p59 = 11.9;
        params.p60 = 3.9;
        params.p61 = 0.0;
        params.p62 = 0.0;
        params.p63 = 0.0;
        params.p64 = 0.14;
        params.p65 = 0.14;
        params.p66 = 0.0;
        params.p67 = 0.0;
        params.p68 = 0.0;
        params.p69 = 0.0;
        params.p70 = 0.0;
        params.p71 = 19.2;
        params.p72 = 0.45;
        params.p73 = 0.045;
        params.p74 = 2.0;
        params.p75 = 0.0;
        params.p76 = 0.375;
        params.p77 = 0.0;
        params.p78 = 0.0;
        params.p79 = 0.0;
        params.p80 = 1e-7;
        params.p81 = 0.0;
        params.p82 = 1e-7;
        params.p83 = 0.0;
        params.p84 = 0.0;
        params.p85 = -0.32;
        params.p86 = 8.2e-9;
        params.p87 = 0.0;
        params.p88 = 1e-9;
        params.p89 = 0.0;
        params.p90 = 0.0;
        params.p91 = 1.0;
        params.p92 = 0.0;
        params.p93 = 0.0;
        params.p94 = 0.0;
        params.p95 = 0.0;
        params.p96 = 0.54;
        params.p97 = 0.001;
        params.p98 = 0.66;
        params.p99 = params.p45;
        validate_parameter("TOXP", params.p99, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p100 = 85000.0;
        params.p101 = 0.0;
        params.p102 = 1e-7;
        params.p103 = params.p100;
        validate_finite_parameter("VSAT1", params.p103).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p104 = params.p101;
        validate_finite_parameter("AVSAT1", params.p104).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p105 = params.p102;
        validate_finite_parameter("BVSAT1", params.p105).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p106 = params.p100;
        validate_finite_parameter("VSATCV", params.p106).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p107 = params.p101;
        validate_finite_parameter("AVSATCV", params.p107).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p108 = params.p102;
        validate_finite_parameter("BVSATCV", params.p108).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p109 = 1.0;
        params.p110 = 1.0;
        params.p111 = 1.0;
        params.p112 = 0.0;
        params.p113 = 4.0;
        params.p114 = 0.0;
        params.p115 = 1.0;
        params.p116 = 0.0;
        params.p117 = 0.0;
        params.p118 = 1e-7;
        params.p119 = -0.00156;
        params.p120 = 0.0;
        params.p121 = 0.0;
        params.p122 = 0.004;
        params.p123 = 0.0;
        params.p124 = 0.0;
        params.p125 = 0.0;
        params.p126 = 1e-7;
        params.p127 = 0.0;
        params.p128 = 1e-7;
        params.p129 = 0.0;
        params.p130 = 0.0;
        params.p131 = 0.0;
        params.p132 = 0.0;
        params.p133 = 1e-7;
        params.p134 = 0.01;
        params.p135 = 0.03;
        params.p136 = 2.0;
        params.p137 = 0.0;
        params.p138 = 1.0;
        params.p139 = 0.3;
        params.p140 = 0.0;
        params.p141 = 1e-7;
        params.p142 = 2.5;
        params.p143 = 0.0;
        params.p144 = 1e-7;
        params.p145 = 0.0;
        params.p146 = 0.0;
        params.p147 = 1e-7;
        params.p148 = 0.0;
        params.p149 = 0.0;
        params.p150 = 5e-8;
        params.p151 = 0.0;
        params.p152 = 0.0;
        params.p153 = 5e-8;
        params.p154 = 0.01;
        params.p155 = 1.0;
        params.p156 = 0.0;
        params.p157 = -0.0015;
        params.p158 = 0.001032;
        params.p159 = 0.0;
        params.p160 = 0.0;
        params.p161 = -0.004775;
        params.p162 = 0.0;
        params.p163 = 0.0;
        params.p164 = 0.0;
        params.p165 = 1e-7;
        params.p166 = 0.03;
        params.p167 = 0.3;
        params.p168 = 0.0;
        params.p169 = 1e-7;
        params.p170 = 2.5;
        params.p171 = 0.0;
        params.p172 = 1e-7;
        params.p173 = 0.0;
        params.p174 = 0.0;
        params.p175 = 1e-7;
        params.p176 = 0.0;
        params.p177 = 0.0;
        params.p178 = 5e-8;
        params.p179 = 0.0;
        params.p180 = 0.0;
        params.p181 = 5e-8;
        params.p182 = 1.0;
        params.p183 = 0.0;
        params.p184 = 0.0;
        params.p185 = 1e-7;
        params.p186 = 2.0;
        params.p187 = 0.0;
        params.p188 = 1.0;
        params.p189 = 0.0;
        params.p190 = 0.0;
        params.p191 = 100.0;
        params.p192 = 0.0;
        params.p193 = 1e-7;
        params.p194 = 0.0;
        params.p195 = 50.0;
        params.p196 = 0.0;
        params.p197 = 1e-7;
        params.p198 = params.p194;
        validate_parameter("RDWMIN", params.p198, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p199 = params.p195;
        validate_parameter("RDW", params.p199, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p200 = params.p196;
        validate_finite_parameter("ARDW", params.p200).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p201 = params.p197;
        validate_finite_parameter("BRDW", params.p201).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p202 = 0.0;
        params.p203 = 0.0;
        params.p204 = 1.0;
        params.p205 = 0.001;
        params.p206 = 1.3;
        params.p207 = 0.0002;
        params.p208 = 1.06;
        params.p209 = 1.0;
        params.p210 = 0.013;
        params.p211 = 0.0;
        params.p212 = 1e-7;
        params.p213 = 0.0;
        params.p214 = 0.013;
        params.p215 = 0.0;
        params.p216 = params.p215;
        validate_parameter("RSHD", params.p216, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p217 = 0.0111;
        params.p218 = 0.000949;
        params.p219 = 0.006;
        params.p220 = 1.1;
        params.p221 = 3.0;
        params.p222 = 0.0136;
        params.p223 = 0.00171;
        params.p224 = 0.075;
        params.p225 = 1.0;
        params.p226 = 0.0136;
        params.p227 = 0.00171;
        params.p228 = 0.075;
        params.p229 = 1.0;
        params.p230 = 1.0;
        params.p231 = 0.0136;
        params.p232 = 0.00171;
        params.p233 = 0.075;
        params.p234 = 0.0;
        params.p235 = params.p234;
        validate_parameter("DLCIGD", params.p235, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p236 = params.p231;
        validate_finite_parameter("AIGD", params.p236).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p237 = params.p232;
        validate_finite_parameter("BIGD", params.p237).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p238 = params.p233;
        validate_finite_parameter("CIGD", params.p238).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p239 = 1.2e-9;
        params.p240 = 1.0;
        params.p241 = 1.0;
        params.p242 = 1.0;
        params.p243 = params.p242;
        validate_finite_parameter("DIGD", params.p243).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p244 = 6.055e-12;
        params.p245 = 300000000.0;
        params.p246 = 0.2;
        params.p247 = 1.0;
        params.p248 = 1.0;
        params.p249 = 0.5;
        params.p250 = params.p244;
        validate_finite_parameter("AGISL", params.p250).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p251 = params.p245;
        validate_finite_parameter("BGISL", params.p251).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p252 = params.p246;
        validate_finite_parameter("EGISL", params.p252).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p253 = params.p247;
        validate_finite_parameter("PGISL", params.p253).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p254 = params.p248;
        validate_finite_parameter("VBGISL", params.p254).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p255 = params.p249;
        validate_finite_parameter("VBEGISL", params.p255).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p256 = 0.0;
        params.p257 = 0.0;
        params.p258 = 0.0;
        params.p259 = 0.0;
        params.p260 = params.p259;
        validate_finite_parameter("LOVD", params.p260).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p261 = 0.0;
        params.p262 = params.p261;
        validate_parameter("CFD", params.p262, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p263 = 0.0;
        params.p264 = params.p263;
        validate_parameter("CGDL", params.p264, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p265 = 0.6;
        params.p266 = params.p265;
        validate_parameter("CKAPPAD", params.p266, Some((0.02, "0.02")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p267 = 0.0;
        params.p268 = 0.0;
        params.p269 = 0.0;
        params.p270 = params.p268;
        validate_finite_parameter("PCOVBD0", params.p270).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p271 = params.p269;
        validate_finite_parameter("PCOVBD1", params.p271).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p272 = 1.0;
        params.p273 = 0.0;
        params.p274 = -1.0;
        params.p275 = 0.12;
        params.p276 = 0.0;
        params.p277 = 0.0;
        params.p278 = 1.0;
        params.p279 = params.p272;
        validate_finite_parameter("KBG0NW", params.p279).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p280 = params.p273;
        validate_finite_parameter("KBG1NW", params.p280).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p281 = params.p274;
        validate_finite_parameter("KBG2NW", params.p281).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p282 = params.p275;
        validate_finite_parameter("DBGNW", params.p282).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p283 = params.p276;
        validate_finite_parameter("BPFACTORNW", params.p283).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p284 = params.p277;
        validate_finite_parameter("VKNEE1NW", params.p284).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p285 = params.p278;
        validate_parameter("VKNEE2NW", params.p285, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p286 = 1.0;
        params.p287 = 41000000.0;
        params.p288 = 6.25e39;
        params.p289 = 3.125e24;
        params.p290 = 87500000.0;
        params.p291 = params.p288;
        validate_parameter("NOIA2", params.p291, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p292 = 2.0;
        params.p293 = 1.2;
        params.p294 = 0.05;
        params.p295 = 1.0;
        params.p296 = 0.0;
        params.p297 = 27.0;
        params.p298 = 400.0;
        params.p299 = 0.000702;
        params.p300 = 1108.0;
        params.p301 = 0.0;
        params.p302 = 0.0;
        params.p303 = 0.0;
        params.p304 = 0.0;
        params.p305 = -0.5;
        params.p306 = -0.003;
        params.p307 = params.p306;
        validate_finite_parameter("TGISL", params.p307).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p308 = 2.5;
        params.p309 = 0.0;
        params.p310 = 0.01;
        params.p311 = 1e-5;
        params.p312 = 0.0;
        params.p313 = 0.0;
        params.p314 = 0.0;
        params.p315 = 1.0;
        params.p316 = 0.1;
        params.p317 = 12.0;
        params.p318 = 1.0;
        params.p319 = 0.0;
        params.p320 = 0.0;
        params.p321 = 0.0;
        params.p322 = 0.0;
        params.p323 = 0.0;
        params.p324 = 0.0;
        params.p325 = params.p322;
        validate_finite_parameter("LRDW", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p326 = params.p323;
        validate_finite_parameter("WRDW", params.p326).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p327 = params.p324;
        validate_finite_parameter("PRDW", params.p327).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p328 = 0.0;
        params.p329 = 0.0;
        params.p330 = 0.0;
        params.p331 = 0.0;
        params.p332 = 0.0;
        params.p333 = 0.0;
        params.p334 = 0.0;
        params.p335 = 0.0;
        params.p336 = 0.0;
        params.p337 = 0.0;
        params.p338 = 0.0;
        params.p339 = 0.0;
        params.p340 = 0.0;
        params.p341 = 0.0;
        params.p342 = 0.0;
        params.p343 = 0.0;
        params.p344 = 0.0;
        params.p345 = 0.0;
        params.p346 = 0.0;
        params.p347 = 0.0;
        params.p348 = 0.0;
        params.p349 = 0.0;
        params.p350 = 0.0;
        params.p351 = 0.0;
        params.p352 = 0.0;
        params.p353 = 0.0;
        params.p354 = 0.0;
        params.p355 = 0.0;
        params.p356 = 0.0;
        params.p357 = 0.0;
        params.p358 = 0.0;
        params.p359 = 0.0;
        params.p360 = 0.0;
        params.p361 = 0.0;
        params.p362 = 0.0;
        params.p363 = 0.0;
        params.p364 = 0.0;
        params.p365 = 0.0;
        params.p366 = 0.0;
        params.p367 = 0.0;
        params.p368 = 0.0;
        params.p369 = 0.0;
        params.p370 = 0.0;
        params.p371 = 0.0;
        params.p372 = 0.0;
        params.p373 = 0.0;
        params.p374 = 0.0;
        params.p375 = 0.0;
        params.p376 = 0.0;
        params.p377 = 0.0;
        params.p378 = 0.0;
        params.p379 = 0.0;
        params.p380 = 0.0;
        params.p381 = 0.0;
        params.p382 = 0.0;
        params.p383 = 0.0;
        params.p384 = 0.0;
        params.p385 = 0.0;
        params.p386 = 0.0;
        params.p387 = 0.0;
        params.p388 = 0.0;
        params.p389 = 0.0;
        params.p390 = 0.0;
        params.p391 = 0.0;
        params.p392 = 0.0;
        params.p393 = 0.0;
        params.p394 = 0.0;
        params.p395 = 0.0;
        params.p396 = 0.0;
        params.p397 = 0.0;
        params.p398 = 0.0;
        params.p399 = 0.0;
        params.p400 = 0.0;
        params.p401 = 0.0;
        params.p402 = 0.0;
        params.p403 = 0.0;
        params.p404 = 0.0;
        params.p405 = 0.0;
        params.p406 = 0.0;
        params.p407 = 0.0;
        params.p408 = 0.0;
        params.p409 = 0.0;
        params.p410 = 0.0;
        params.p411 = 0.0;
        params.p412 = 0.0;
        params.p413 = 0.0;
        params.p414 = 0.0;
        params.p415 = 0.0;
        params.p416 = 0.0;
        params.p417 = 0.0;
        params.p418 = 0.0;
        params.p419 = 0.0;
        params.p420 = 0.0;
        params.p421 = 0.0;
        params.p422 = 0.0;
        params.p423 = 0.0;
        params.p424 = 0.0;
        params.p425 = 0.0;
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
        params.p437 = 0.0;
        params.p438 = 0.0;
        params.p439 = 0.0;
        params.p440 = 0.0;
        params.p441 = 0.0;
        params.p442 = 0.0;
        params.p443 = 0.0;
        params.p444 = 0.0;
        params.p445 = 0.0;
        params.p446 = 0.0;
        params.p447 = 0.0;
        params.p448 = 0.0;
        params.p449 = 0.0;
        params.p450 = 0.0;
        params.p451 = 0.0;
        params.p452 = 0.0;
        params.p453 = 0.0;
        params.p454 = 0.0;
        params.p455 = 0.0;
        params.p456 = 0.0;
        params.p457 = 0.0;
        params.p458 = 0.0;
        params.p459 = 0.0;
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
        params.p476 = 0.0;
        params.p477 = 0.0;
        params.p478 = 0.0;
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
        params.p493 = 0.0;
        params.p494 = 0.0;
        params.p495 = 0.0;
        params.p496 = 0.0;
        params.p497 = 0.0;
        params.p498 = 0.0;
        params.p499 = 0.0;
        params.p500 = 0.0;
        params.p501 = 0.0;
        params.p502 = 0.0;
        params.p503 = 0.0;
        params.p504 = 0.0;
        params.p505 = 0.0;
        params.p506 = 0.0;
        params.p507 = 0.0;
        params.p508 = 0.0;
        params.p509 = 0.0;
        params.p510 = 0.0;
        params.p511 = 0.0;
        params.p512 = 0.0;
        params.p513 = 0.0;
        params.p514 = 0.0;
        params.p515 = 0.0;
        params.p516 = 0.0;
        params.p517 = 0.0;
        params.p518 = 0.0;
        params.p519 = 0.0;
        params.p520 = 0.0;
        params.p521 = 0.0;
        params.p522 = 0.0;
        params.p523 = 0.0;
        params.p524 = 0.0;
        params.p525 = 0.0;
        params.p526 = 0.0;
        params.p527 = 0.0;
        params.p528 = 0.0;
        params.p529 = 0.0;
        params.p530 = 0.0;
        params.p531 = 0.0;
        params.p532 = 0.0;
        params.p533 = 0.0;
        params.p534 = 0.0;
        params.p535 = 0.0;
        params.p536 = 0.0;
        params.p537 = 0.0;
        params.p538 = 0.0;
        params.p539 = 0.0;
        params.p540 = 0.0;
        params.p541 = 0.0;
        params.p542 = 0.0;
        params.p543 = 0.0;
        params.p544 = 0.0;
        params.p545 = 0.0;
        params.p546 = 0.0;
        params.p547 = 0.0;
        params.p548 = 0.0;
        params.p549 = 0.0;
        params.p550 = 0.0;
        params.p551 = 0.0;
        params.p552 = 0.0;
        params.p553 = params.p550;
        validate_finite_parameter("LTGISL", params.p553).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p554 = params.p551;
        validate_finite_parameter("WTGISL", params.p554).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p555 = params.p552;
        validate_finite_parameter("PTGISL", params.p555).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p556 = 0.0;
        params.p557 = 0.0;
        params.p558 = 0.0;
        params.p559 = 0.0;
        params.p560 = 0.0;
        params.p561 = 0.0;
        params.p562 = 0.0;
        params.p563 = 0.0;
        params.p564 = 0.0;
        params.p565 = 0.0;
        params.p566 = 0.0;
        params.p567 = 0.0;
        params.p568 = 0.0;
        params.p569 = 0.0;
        params.p570 = 0.0;
        params.p571 = 0.0;
        params.p572 = 0.0;
        params.p573 = 0.0;
        params.p574 = 0.0;
        params.p575 = 0.0;
        params.p576 = 0.0;
        params.p577 = 0.0;
        params.p578 = 0.0;
        params.p579 = 0.0;
        params.p580 = 0.0;
        params.p581 = 0.0;
        params.p582 = 0.0;
        params.p583 = 0.0;
        params.p584 = 0.0;
        params.p585 = 0.0;
        params.p586 = 0.0;
        params.p587 = 0.0;
        params.p588 = 0.0;
        params.p589 = 0.0;
        params.p590 = 0.0;
        params.p591 = 0.0;
        params.p592 = 0.0;
        params.p593 = 0.0;
        params.p594 = 0.0;
        params.p595 = 0.0;
        params.p596 = 0.0;
        params.p597 = 0.0;
        params.p598 = 0.0;
        params.p599 = 0.0;
        params.p600 = 0.0;
        params.p601 = 0.0;
        params.p602 = 0.0;
        params.p603 = 0.0;
        params.p604 = 0.0;
        params.p605 = 0.0;
        params.p606 = 0.0;
        params.p607 = 0.0;
        params.p608 = 0.0;
        params.p609 = 0.0;
        params.p610 = 0.0;
        params.p611 = 0.0;
        params.p612 = 0.0;
        params.p613 = 0.0;
        params.p614 = 0.0;
        params.p615 = 0.0;
        params.p616 = 0.0;
        params.p617 = 0.0;
        params.p618 = 0.0;
        params.p619 = params.p601;
        validate_finite_parameter("LAGISL", params.p619).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p620 = params.p602;
        validate_finite_parameter("WAGISL", params.p620).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p621 = params.p603;
        validate_finite_parameter("PAGISL", params.p621).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p622 = params.p604;
        validate_finite_parameter("LBGISL", params.p622).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p623 = params.p605;
        validate_finite_parameter("WBGISL", params.p623).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p624 = params.p606;
        validate_finite_parameter("PBGISL", params.p624).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p625 = params.p607;
        validate_finite_parameter("LEGISL", params.p625).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p626 = params.p608;
        validate_finite_parameter("WEGISL", params.p626).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p627 = params.p609;
        validate_finite_parameter("PEGISL", params.p627).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p628 = params.p610;
        validate_finite_parameter("LPGISL", params.p628).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p629 = params.p611;
        validate_finite_parameter("WPGISL", params.p629).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p630 = params.p612;
        validate_finite_parameter("PPGISL", params.p630).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p631 = params.p613;
        validate_finite_parameter("LVBGISL", params.p631).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p632 = params.p614;
        validate_finite_parameter("WVBGISL", params.p632).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p633 = params.p615;
        validate_finite_parameter("PVBGISL", params.p633).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p634 = params.p616;
        validate_finite_parameter("LVBEGISL", params.p634).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p635 = params.p617;
        validate_finite_parameter("WVBEGISL", params.p635).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p636 = params.p618;
        validate_finite_parameter("PVBEGISL", params.p636).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p637 = 0.0;
        params.p638 = 0.0;
        params.p639 = 0.0;
        params.p640 = params.p637;
        validate_finite_parameter("LAIGD", params.p640).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p641 = params.p638;
        validate_finite_parameter("WAIGD", params.p641).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p642 = params.p639;
        validate_finite_parameter("PAIGD", params.p642).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p643 = 0.0;
        params.p644 = 0.0;
        params.p645 = 0.0;
        params.p646 = params.p643;
        validate_finite_parameter("LBIGD", params.p646).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p647 = params.p644;
        validate_finite_parameter("WBIGD", params.p647).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p648 = params.p645;
        validate_finite_parameter("PBIGD", params.p648).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p649 = 0.0;
        params.p650 = 0.0;
        params.p651 = 0.0;
        params.p652 = params.p649;
        validate_finite_parameter("LCIGD", params.p652).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p653 = params.p650;
        validate_finite_parameter("WCIGD", params.p653).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p654 = params.p651;
        validate_finite_parameter("PCIGD", params.p654).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p655 = 0.0;
        params.p656 = 0.0;
        params.p657 = 0.0;
        params.p658 = params.p655;
        validate_finite_parameter("LDIGD", params.p658).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p659 = params.p656;
        validate_finite_parameter("WDIGD", params.p659).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p660 = params.p657;
        validate_finite_parameter("PDIGD", params.p660).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p661 = 0.0;
        params.p662 = 0.0;
        params.p663 = 0.0;
        params.p664 = 0.0;
        params.p665 = 0.0;
        params.p666 = 0.0;
        params.p667 = 0.0;
        params.p668 = 0.0;
        params.p669 = 0.0;
        params.p670 = params.p667;
        validate_finite_parameter("LLOVD", params.p670).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p671 = params.p668;
        validate_finite_parameter("WLOVD", params.p671).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p672 = params.p669;
        validate_finite_parameter("PLOVD", params.p672).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p673 = 0.0;
        params.p674 = 0.0;
        params.p675 = 0.0;
        params.p676 = params.p673;
        validate_finite_parameter("LCFD", params.p676).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p677 = params.p674;
        validate_finite_parameter("WCFD", params.p677).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p678 = params.p675;
        validate_finite_parameter("PCFD", params.p678).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p679 = 0.0;
        params.p680 = 0.0;
        params.p681 = 0.0;
        params.p682 = 0.0;
        params.p683 = 0.0;
        params.p684 = 0.0;
        params.p685 = 0.0;
        params.p686 = 0.0;
        params.p687 = 0.0;
        params.p688 = 0.0;
        params.p689 = 0.0;
        params.p690 = 0.0;
        params.p691 = 0.0;
        params.p692 = 0.0;
        params.p693 = 0.0;
        params.p694 = 0.0;
        params.p695 = 0.0;
        params.p696 = 0.0;
        params.p697 = 0.0;
        params.p698 = 0.0;
        params.p699 = 0.0;
        params.p700 = 0.0;
        params.p701 = 0.0;
        params.p702 = 0.0;
        params.p703 = 0.0;
        params.p704 = 0.0;
        params.p705 = 0.0;
        params.p706 = 0.0;
        params.p707 = 0.0;
        params.p708 = 0.0;
        params.p709 = 0.0;
        params.p710 = 0.0;
        params.p711 = 0.0;
        params.p712 = 0.0;
        params.p713 = 0.0;
        params.p714 = 0.0;
        params.p715 = 0.0;
        params.p716 = 0.0;
        params.p717 = 0.0;
        params.p718 = 0.0;
        params.p719 = 0.0;
        params.p720 = 0.0;
        params.p721 = 0.0;
        params.p722 = 0.0;
        params.p723 = 0.0;
        params.p724 = 0.0;
        params.p725 = 0.0;
        params.p726 = 0.0;
        params.p727 = 0.0;
        params.p728 = 0.0;
        params.p729 = 0.0;
        params.p730 = 0.0;
        params.p731 = 0.0;
        params.p732 = 0.0;
        params.p733 = 0.0;
        params.p734 = 0.0;
        params.p735 = 0.0;
        params.p736 = 0.0;
        params.p737 = 0.0;
        params.p738 = 0.0;
        params.p739 = 0.0;
        params.p740 = 0.0;
        params.p741 = 0.0;
        params.p742 = 0.0;
        params.p743 = 0.0;
        params.p744 = 0.0;
        params.p745 = 0.0;
        params.p746 = 0.0;
        params.p747 = 0.0;
        params.p748 = 0.0;
        params.p749 = 0.0;
        params.p750 = 0.0;
        params.p751 = 0.0;
        params.p752 = 0.0;
        params.p753 = 0.0;
        params.p754 = 0.0;
        params.p755 = 0.0;
        params.p756 = 0.0;
        params.p757 = 0.0;
        params.p758 = 0.0;
        params.p759 = 0.0;
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
    pub nodes: [usize; 9],
    pub branches: [usize; 5],
    pub params: Parameters,
    pub(crate) param_given: [bool; 760],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 8],
    pub(crate) ddt_state_previous: [f64; 8],
    pub(crate) ddt_state_initialized: [bool; 8],
    pub(crate) idt_state_current: [f64; 0],
    pub(crate) idt_state_previous: [f64; 0],
    pub(crate) idt_state_initialized: [bool; 0],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<676, 9, 5>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<676, 9, 5>>>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 4;
    pub const NODE_COUNT: usize = 9;
    pub const INTERNAL_NODE_NAMES: [&str; 4] = ["di", "si", "ge", "gi"];

    pub const BRANCH_COUNT: usize = 5;
    pub const PARAMETER_COUNT: usize = 760;
    pub const VARIABLE_COUNT: usize = 676;
    pub const DDT_STATE_COUNT: usize = 8;
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
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "delvtrand" => { validate_finite_parameter("DELVTRAND", value)?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "u0mult" => { validate_parameter("U0MULT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "welltype" => { validate_parameter("WELLTYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "rdsmod" => { validate_parameter("RDSMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "gidlmod" => { validate_parameter("GIDLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "igcmod" => { validate_parameter("IGCMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "igbmod" => { validate_parameter("IGBMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "shmod" => { validate_parameter("SHMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "rgatemod" => { validate_parameter("RGATEMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "nqsmod" => { validate_parameter("NQSMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "nfmod" => { validate_parameter("NFMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "fnmod" => { validate_parameter("FNMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "xl" => { validate_finite_parameter("XL", value)?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "xw" => { validate_finite_parameter("XW", value)?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "lint" => { validate_finite_parameter("LINT", value)?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "ll" => { validate_finite_parameter("LL", value)?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "lw" => { validate_finite_parameter("LW", value)?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "lwl" => { validate_finite_parameter("LWL", value)?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "lln" => { validate_finite_parameter("LLN", value)?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "lwn" => { validate_finite_parameter("LWN", value)?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "wint" => { validate_finite_parameter("WINT", value)?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "wl" => { validate_finite_parameter("WL", value)?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "ww" => { validate_finite_parameter("WW", value)?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "wwl" => { validate_finite_parameter("WWL", value)?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "wln" => { validate_finite_parameter("WLN", value)?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "wwn" => { validate_finite_parameter("WWN", value)?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "dlc" => { validate_finite_parameter("DLC", value)?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "llc" => { validate_finite_parameter("LLC", value)?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "lwc" => { validate_finite_parameter("LWC", value)?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "lwlc" => { validate_finite_parameter("LWLC", value)?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "dwc" => { validate_finite_parameter("DWC", value)?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "wlc" => { validate_finite_parameter("WLC", value)?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "wwc" => { validate_finite_parameter("WWC", value)?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "wwlc" => { validate_finite_parameter("WWLC", value)?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "eot1" => { validate_parameter("EOT1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "eot2" => { validate_parameter("EOT2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "eot1p" => { validate_parameter("EOT1P", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "dtox1" => { validate_finite_parameter("DTOX1", value)?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "tsi" => { validate_parameter("TSI", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "nbody" => { validate_parameter("NBODY", value, Some((1e18, "1e18")), false, Some((5e24, "5e24")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "nsd" => { validate_parameter("NSD", value, Some((2e25, "2e25")), false, Some((1e27, "1e27")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "nbg" => { validate_parameter("NBG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "easub" => { validate_parameter("EASUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "ni0sub" => { validate_parameter("NI0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "bg0sub" => { validate_parameter("BG0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "nc0sub" => { validate_parameter("NC0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "phig1" => { validate_parameter("PHIG1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "phig2" => { validate_parameter("PHIG2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "epsrsub" => { validate_parameter("EPSRSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "epsrox1" => { validate_parameter("EPSROX1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "ascl" => { validate_finite_parameter("ASCL", value)?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "bscl" => { validate_finite_parameter("BSCL", value)?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "cdsc" => { validate_finite_parameter("CDSC", value)?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "cdscd" => { validate_finite_parameter("CDSCD", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "cbgcbg0" => { validate_finite_parameter("CBGCBG0", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "cbgcbg0p" => { validate_finite_parameter("CBGCBG0P", value)?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "cbgcbg" => { validate_finite_parameter("CBGCBG", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "cbgcbgp" => { validate_finite_parameter("CBGCBGP", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "cbgcbgd" => { validate_finite_parameter("CBGCBGD", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "dvt0" => { validate_finite_parameter("DVT0", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "dvt1" => { validate_finite_parameter("DVT1", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "phin" => { validate_finite_parameter("PHIN", value)?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "eta1" => { validate_finite_parameter("ETA1", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "dsub" => { validate_finite_parameter("DSUB", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "dvtp0" => { validate_finite_parameter("DVTP0", value)?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "dvtp1" => { validate_finite_parameter("DVTP1", value)?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "advtp0" => { validate_finite_parameter("ADVTP0", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "bdvtp0" => { validate_finite_parameter("BDVTP0", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "advtp1" => { validate_finite_parameter("ADVTP1", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "bdvtp1" => { validate_finite_parameter("BDVTP1", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "dvtp2" => { validate_finite_parameter("DVTP2", value)?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "k1rsce" => { validate_finite_parameter("K1RSCE", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "lpe0" => { validate_finite_parameter("LPE0", value)?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "dsc0" => { validate_finite_parameter("DSC0", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "dsc1" => { validate_finite_parameter("DSC1", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "k0" => { validate_finite_parameter("K0", value)?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "k01" => { validate_finite_parameter("K01", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "k0si" => { validate_finite_parameter("K0SI", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "k0si1" => { validate_finite_parameter("K0SI1", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "k0sisat" => { validate_finite_parameter("K0SISAT", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "k0sisat1" => { validate_finite_parameter("K0SISAT1", value)?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "qmtcencv" => { validate_finite_parameter("QMTCENCV", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "etaqm" => { validate_finite_parameter("ETAQM", value)?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "qm0" => { validate_parameter("QM0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "pqm" => { validate_finite_parameter("PQM", value)?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "toxp" => { validate_parameter("TOXP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "vsat" => { validate_finite_parameter("VSAT", value)?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "avsat" => { validate_finite_parameter("AVSAT", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "bvsat" => { validate_finite_parameter("BVSAT", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "vsat1" => { validate_finite_parameter("VSAT1", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "avsat1" => { validate_finite_parameter("AVSAT1", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "bvsat1" => { validate_finite_parameter("BVSAT1", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "vsatcv" => { validate_finite_parameter("VSATCV", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "avsatcv" => { validate_finite_parameter("AVSATCV", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "bvsatcv" => { validate_finite_parameter("BVSATCV", value)?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "deltavsat" => { validate_finite_parameter("DELTAVSAT", value)?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "ksativ" => { validate_finite_parameter("KSATIV", value)?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "ksubiv" => { validate_finite_parameter("KSUBIV", value)?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "ksativb" => { validate_finite_parameter("KSATIVB", value)?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "mexp" => { validate_finite_parameter("MEXP", value)?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "amexp" => { validate_finite_parameter("AMEXP", value)?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "bmexp" => { validate_finite_parameter("BMEXP", value)?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "ptwg" => { validate_finite_parameter("PTWG", value)?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "aptwg" => { validate_finite_parameter("APTWG", value)?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "bptwg" => { validate_finite_parameter("BPTWG", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "at" => { validate_finite_parameter("AT", value)?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "atl" => { validate_finite_parameter("ATL", value)?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "tmexp" => { validate_finite_parameter("TMEXP", value)?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "ptwgt" => { validate_finite_parameter("PTWGT", value)?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "ptwgb" => { validate_finite_parameter("PTWGB", value)?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "ptwgb2" => { validate_finite_parameter("PTWGB2", value)?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "aptwgb" => { validate_finite_parameter("APTWGB", value)?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "bptwgb" => { validate_finite_parameter("BPTWGB", value)?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "aptwgb2" => { validate_finite_parameter("APTWGB2", value)?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "bptwgb2" => { validate_finite_parameter("BPTWGB2", value)?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "vsatb" => { validate_finite_parameter("VSATB", value)?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "atb" => { validate_finite_parameter("ATB", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "atbl" => { validate_finite_parameter("ATBL", value)?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "avsatb" => { validate_finite_parameter("AVSATB", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "bvsatb" => { validate_finite_parameter("BVSATB", value)?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "dvsatclamp" => { validate_parameter("DVSATCLAMP", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "u0" => { validate_finite_parameter("U0", value)?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "etamob" => { validate_finite_parameter("ETAMOB", value)?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "up" => { validate_finite_parameter("UP", value)?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "lpa" => { validate_finite_parameter("LPA", value)?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "ua" => { validate_finite_parameter("UA", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "aua" => { validate_finite_parameter("AUA", value)?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "bua" => { validate_finite_parameter("BUA", value)?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "eu" => { validate_finite_parameter("EU", value)?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "aeu" => { validate_finite_parameter("AEU", value)?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "beu" => { validate_finite_parameter("BEU", value)?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "uc" => { validate_finite_parameter("UC", value)?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "auc" => { validate_finite_parameter("AUC", value)?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "buc" => { validate_finite_parameter("BUC", value)?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "ud" => { validate_finite_parameter("UD", value)?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "aud" => { validate_finite_parameter("AUD", value)?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "bud" => { validate_finite_parameter("BUD", value)?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "udb" => { validate_finite_parameter("UDB", value)?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "audb" => { validate_finite_parameter("AUDB", value)?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "budb" => { validate_finite_parameter("BUDB", value)?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "dmobclamp" => { validate_parameter("DMOBCLAMP", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "ucs" => { validate_finite_parameter("UCS", value)?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "ute" => { validate_finite_parameter("UTE", value)?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "utl" => { validate_finite_parameter("UTL", value)?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "ua1" => { validate_finite_parameter("UA1", value)?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "uc1" => { validate_finite_parameter("UC1", value)?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "ud1" => { validate_finite_parameter("UD1", value)?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "ucste" => { validate_finite_parameter("UCSTE", value)?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "chargewf" => { validate_parameter("CHARGEWF", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "eub" => { validate_finite_parameter("EUB", value)?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "aeub" => { validate_finite_parameter("AEUB", value)?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "beub" => { validate_finite_parameter("BEUB", value)?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "u02" => { validate_finite_parameter("U02", value)?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "ua2" => { validate_finite_parameter("UA2", value)?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "aua2" => { validate_finite_parameter("AUA2", value)?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "bua2" => { validate_finite_parameter("BUA2", value)?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "eu2" => { validate_finite_parameter("EU2", value)?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "aeu2" => { validate_finite_parameter("AEU2", value)?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "beu2" => { validate_finite_parameter("BEU2", value)?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "uc2" => { validate_finite_parameter("UC2", value)?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "auc2" => { validate_finite_parameter("AUC2", value)?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "buc2" => { validate_finite_parameter("BUC2", value)?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "ud2" => { validate_finite_parameter("UD2", value)?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "aud2" => { validate_finite_parameter("AUD2", value)?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "bud2" => { validate_finite_parameter("BUD2", value)?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "udb2" => { validate_finite_parameter("UDB2", value)?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "audb2" => { validate_finite_parameter("AUDB2", value)?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "budb2" => { validate_finite_parameter("BUDB2", value)?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "ucs2" => { validate_finite_parameter("UCS2", value)?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "eub2" => { validate_finite_parameter("EUB2", value)?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "aeub2" => { validate_finite_parameter("AEUB2", value)?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "beub2" => { validate_finite_parameter("BEUB2", value)?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "etamob2" => { validate_finite_parameter("ETAMOB2", value)?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "up2" => { validate_finite_parameter("UP2", value)?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "lpa2" => { validate_finite_parameter("LPA2", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "chargewf2" => { validate_parameter("CHARGEWF2", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "rdswmin" => { validate_parameter("RDSWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "rdsw" => { validate_parameter("RDSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "ardsw" => { validate_finite_parameter("ARDSW", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "brdsw" => { validate_finite_parameter("BRDSW", value)?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "rswmin" => { validate_parameter("RSWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "rsw" => { validate_parameter("RSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "arsw" => { validate_finite_parameter("ARSW", value)?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "brsw" => { validate_finite_parameter("BRSW", value)?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "rdwmin" => { validate_parameter("RDWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "rdw" => { validate_parameter("RDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "ardw" => { validate_finite_parameter("ARDW", value)?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "brdw" => { validate_finite_parameter("BRDW", value)?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "prwg" => { validate_finite_parameter("PRWG", value)?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "prwb" => { validate_finite_parameter("PRWB", value)?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "wr" => { validate_finite_parameter("WR", value)?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "prt" => { validate_finite_parameter("PRT", value)?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "pdibl1" => { validate_finite_parameter("PDIBL1", value)?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "pdibl2" => { validate_finite_parameter("PDIBL2", value)?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "drout" => { validate_finite_parameter("DROUT", value)?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "pvag" => { validate_finite_parameter("PVAG", value)?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "pclm" => { validate_finite_parameter("PCLM", value)?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "apclm" => { validate_finite_parameter("APCLM", value)?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "bpclm" => { validate_finite_parameter("BPCLM", value)?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "pclmg" => { validate_finite_parameter("PCLMG", value)?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "pclmcv" => { validate_finite_parameter("PCLMCV", value)?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "rshd" => { validate_parameter("RSHD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "aigbinv" => { validate_finite_parameter("AIGBINV", value)?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "bigbinv" => { validate_finite_parameter("BIGBINV", value)?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "cigbinv" => { validate_finite_parameter("CIGBINV", value)?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "eigbinv" => { validate_finite_parameter("EIGBINV", value)?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "nigbinv" => { validate_finite_parameter("NIGBINV", value)?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "aigbacc" => { validate_finite_parameter("AIGBACC", value)?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "bigbacc" => { validate_finite_parameter("BIGBACC", value)?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "cigbacc" => { validate_finite_parameter("CIGBACC", value)?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "nigbacc" => { validate_finite_parameter("NIGBACC", value)?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "aigc" => { validate_finite_parameter("AIGC", value)?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "bigc" => { validate_finite_parameter("BIGC", value)?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "cigc" => { validate_finite_parameter("CIGC", value)?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "pigcd" => { validate_finite_parameter("PIGCD", value)?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "digc" => { validate_finite_parameter("DIGC", value)?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "aigs" => { validate_finite_parameter("AIGS", value)?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "bigs" => { validate_finite_parameter("BIGS", value)?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "cigs" => { validate_finite_parameter("CIGS", value)?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "dlcigs" => { validate_parameter("DLCIGS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "dlcigd" => { validate_parameter("DLCIGD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "aigd" => { validate_finite_parameter("AIGD", value)?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "bigd" => { validate_finite_parameter("BIGD", value)?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "cigd" => { validate_finite_parameter("CIGD", value)?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "toxref" => { validate_parameter("TOXREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "ntox" => { validate_finite_parameter("NTOX", value)?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "poxedge" => { validate_finite_parameter("POXEDGE", value)?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "digs" => { validate_finite_parameter("DIGS", value)?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "digd" => { validate_finite_parameter("DIGD", value)?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "agidl" => { validate_finite_parameter("AGIDL", value)?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "bgidl" => { validate_finite_parameter("BGIDL", value)?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "egidl" => { validate_finite_parameter("EGIDL", value)?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "pgidl" => { validate_finite_parameter("PGIDL", value)?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "vbgidl" => { validate_finite_parameter("VBGIDL", value)?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "vbegidl" => { validate_finite_parameter("VBEGIDL", value)?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "agisl" => { validate_finite_parameter("AGISL", value)?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "bgisl" => { validate_finite_parameter("BGISL", value)?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "egisl" => { validate_finite_parameter("EGISL", value)?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "pgisl" => { validate_finite_parameter("PGISL", value)?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "vbgisl" => { validate_finite_parameter("VBGISL", value)?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "vbegisl" => { validate_finite_parameter("VBEGISL", value)?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "alpha0" => { validate_finite_parameter("ALPHA0", value)?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "alpha1" => { validate_finite_parameter("ALPHA1", value)?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "beta0" => { validate_finite_parameter("BETA0", value)?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "lovs" => { validate_finite_parameter("LOVS", value)?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "lovd" => { validate_finite_parameter("LOVD", value)?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "cfs" => { validate_parameter("CFS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "cfd" => { validate_parameter("CFD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "cgsl" => { validate_parameter("CGSL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "cgdl" => { validate_parameter("CGDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "ckappas" => { validate_parameter("CKAPPAS", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "ckappad" => { validate_parameter("CKAPPAD", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "csdbgsw" => { validate_finite_parameter("CSDBGSW", value)?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "pcovbs0" => { validate_finite_parameter("PCOVBS0", value)?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "pcovbs1" => { validate_finite_parameter("PCOVBS1", value)?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "pcovbd0" => { validate_finite_parameter("PCOVBD0", value)?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "pcovbd1" => { validate_finite_parameter("PCOVBD1", value)?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "kbg0pw" => { validate_finite_parameter("KBG0PW", value)?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "kbg1pw" => { validate_finite_parameter("KBG1PW", value)?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "kbg2pw" => { validate_finite_parameter("KBG2PW", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "dbgpw" => { validate_finite_parameter("DBGPW", value)?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "bpfactorpw" => { validate_finite_parameter("BPFACTORPW", value)?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "vknee1pw" => { validate_finite_parameter("VKNEE1PW", value)?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "vknee2pw" => { validate_parameter("VKNEE2PW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "kbg0nw" => { validate_finite_parameter("KBG0NW", value)?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "kbg1nw" => { validate_finite_parameter("KBG1NW", value)?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "kbg2nw" => { validate_finite_parameter("KBG2NW", value)?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "dbgnw" => { validate_finite_parameter("DBGNW", value)?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "bpfactornw" => { validate_finite_parameter("BPFACTORNW", value)?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "vknee1nw" => { validate_finite_parameter("VKNEE1NW", value)?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "vknee2nw" => { validate_parameter("VKNEE2NW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.0, "0.0")), true, Some((2.0, "2.0")), false, &[])?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
            "em" => { validate_parameter("EM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p287 = value; self.mark_param_given(287); Ok(()) }
            "noia" => { validate_parameter("NOIA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); Ok(()) }
            "noib" => { validate_parameter("NOIB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); Ok(()) }
            "noic" => { validate_parameter("NOIC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); Ok(()) }
            "noia2" => { validate_parameter("NOIA2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p291 = value; self.mark_param_given(291); Ok(()) }
            "smooth" => { validate_parameter("SMOOTH", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p292 = value; self.mark_param_given(292); Ok(()) }
            "mpower" => { validate_parameter("MPOWER", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); Ok(()) }
            "qsref" => { validate_parameter("QSREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p294 = value; self.mark_param_given(294); Ok(()) }
            "ntnoi" => { validate_parameter("NTNOI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p295 = value; self.mark_param_given(295); Ok(()) }
            "lintnoi" => { validate_finite_parameter("LINTNOI", value)?; self.params.p296 = value; self.mark_param_given(296); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p297 = value; self.mark_param_given(297); Ok(()) }
            "tmaxc" => { validate_finite_parameter("TMAXC", value)?; self.params.p298 = value; self.mark_param_given(298); Ok(()) }
            "tbgasub" => { validate_finite_parameter("TBGASUB", value)?; self.params.p299 = value; self.mark_param_given(299); Ok(()) }
            "tbgbsub" => { validate_finite_parameter("TBGBSUB", value)?; self.params.p300 = value; self.mark_param_given(300); Ok(()) }
            "kt1" => { validate_finite_parameter("KT1", value)?; self.params.p301 = value; self.mark_param_given(301); Ok(()) }
            "kt1l" => { validate_finite_parameter("KT1L", value)?; self.params.p302 = value; self.mark_param_given(302); Ok(()) }
            "kt2" => { validate_finite_parameter("KT2", value)?; self.params.p303 = value; self.mark_param_given(303); Ok(()) }
            "kt2l" => { validate_finite_parameter("KT2L", value)?; self.params.p304 = value; self.mark_param_given(304); Ok(()) }
            "iit" => { validate_finite_parameter("IIT", value)?; self.params.p305 = value; self.mark_param_given(305); Ok(()) }
            "tgidl" => { validate_finite_parameter("TGIDL", value)?; self.params.p306 = value; self.mark_param_given(306); Ok(()) }
            "tgisl" => { validate_finite_parameter("TGISL", value)?; self.params.p307 = value; self.mark_param_given(307); Ok(()) }
            "igt" => { validate_finite_parameter("IGT", value)?; self.params.p308 = value; self.mark_param_given(308); Ok(()) }
            "teta0" => { validate_finite_parameter("TETA0", value)?; self.params.p309 = value; self.mark_param_given(309); Ok(()) }
            "rth0" => { validate_parameter("RTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); Ok(()) }
            "cth0" => { validate_parameter("CTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p311 = value; self.mark_param_given(311); Ok(()) }
            "wth0" => { validate_parameter("WTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p312 = value; self.mark_param_given(312); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p313 = value; self.mark_param_given(313); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p314 = value; self.mark_param_given(314); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p315 = value; self.mark_param_given(315); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); Ok(()) }
            "xrcrg1" => { validate_finite_parameter("XRCRG1", value)?; self.params.p317 = value; self.mark_param_given(317); Ok(()) }
            "xrcrg2" => { validate_finite_parameter("XRCRG2", value)?; self.params.p318 = value; self.mark_param_given(318); Ok(()) }
            "lrdsw" => { validate_finite_parameter("LRDSW", value)?; self.params.p319 = value; self.mark_param_given(319); Ok(()) }
            "wrdsw" => { validate_finite_parameter("WRDSW", value)?; self.params.p320 = value; self.mark_param_given(320); Ok(()) }
            "prdsw" => { validate_finite_parameter("PRDSW", value)?; self.params.p321 = value; self.mark_param_given(321); Ok(()) }
            "lrsw" => { validate_finite_parameter("LRSW", value)?; self.params.p322 = value; self.mark_param_given(322); Ok(()) }
            "wrsw" => { validate_finite_parameter("WRSW", value)?; self.params.p323 = value; self.mark_param_given(323); Ok(()) }
            "prsw" => { validate_finite_parameter("PRSW", value)?; self.params.p324 = value; self.mark_param_given(324); Ok(()) }
            "lrdw" => { validate_finite_parameter("LRDW", value)?; self.params.p325 = value; self.mark_param_given(325); Ok(()) }
            "wrdw" => { validate_finite_parameter("WRDW", value)?; self.params.p326 = value; self.mark_param_given(326); Ok(()) }
            "prdw" => { validate_finite_parameter("PRDW", value)?; self.params.p327 = value; self.mark_param_given(327); Ok(()) }
            "lprwg" => { validate_finite_parameter("LPRWG", value)?; self.params.p328 = value; self.mark_param_given(328); Ok(()) }
            "wprwg" => { validate_finite_parameter("WPRWG", value)?; self.params.p329 = value; self.mark_param_given(329); Ok(()) }
            "pprwg" => { validate_finite_parameter("PPRWG", value)?; self.params.p330 = value; self.mark_param_given(330); Ok(()) }
            "lprwb" => { validate_finite_parameter("LPRWB", value)?; self.params.p331 = value; self.mark_param_given(331); Ok(()) }
            "wprwb" => { validate_finite_parameter("WPRWB", value)?; self.params.p332 = value; self.mark_param_given(332); Ok(()) }
            "pprwb" => { validate_finite_parameter("PPRWB", value)?; self.params.p333 = value; self.mark_param_given(333); Ok(()) }
            "lwr" => { validate_finite_parameter("LWR", value)?; self.params.p334 = value; self.mark_param_given(334); Ok(()) }
            "wwr" => { validate_finite_parameter("WWR", value)?; self.params.p335 = value; self.mark_param_given(335); Ok(()) }
            "pwr" => { validate_finite_parameter("PWR", value)?; self.params.p336 = value; self.mark_param_given(336); Ok(()) }
            "lphig1" => { validate_finite_parameter("LPHIG1", value)?; self.params.p337 = value; self.mark_param_given(337); Ok(()) }
            "wphig1" => { validate_finite_parameter("WPHIG1", value)?; self.params.p338 = value; self.mark_param_given(338); Ok(()) }
            "pphig1" => { validate_finite_parameter("PPHIG1", value)?; self.params.p339 = value; self.mark_param_given(339); Ok(()) }
            "lphig2" => { validate_finite_parameter("LPHIG2", value)?; self.params.p340 = value; self.mark_param_given(340); Ok(()) }
            "wphig2" => { validate_finite_parameter("WPHIG2", value)?; self.params.p341 = value; self.mark_param_given(341); Ok(()) }
            "pphig2" => { validate_finite_parameter("PPHIG2", value)?; self.params.p342 = value; self.mark_param_given(342); Ok(()) }
            "lnsd" => { validate_finite_parameter("LNSD", value)?; self.params.p343 = value; self.mark_param_given(343); Ok(()) }
            "wnsd" => { validate_finite_parameter("WNSD", value)?; self.params.p344 = value; self.mark_param_given(344); Ok(()) }
            "pnsd" => { validate_finite_parameter("PNSD", value)?; self.params.p345 = value; self.mark_param_given(345); Ok(()) }
            "lnbody" => { validate_finite_parameter("LNBODY", value)?; self.params.p346 = value; self.mark_param_given(346); Ok(()) }
            "wnbody" => { validate_finite_parameter("WNBODY", value)?; self.params.p347 = value; self.mark_param_given(347); Ok(()) }
            "pnbody" => { validate_finite_parameter("PNBODY", value)?; self.params.p348 = value; self.mark_param_given(348); Ok(()) }
            "lcit" => { validate_finite_parameter("LCIT", value)?; self.params.p349 = value; self.mark_param_given(349); Ok(()) }
            "wcit" => { validate_finite_parameter("WCIT", value)?; self.params.p350 = value; self.mark_param_given(350); Ok(()) }
            "pcit" => { validate_finite_parameter("PCIT", value)?; self.params.p351 = value; self.mark_param_given(351); Ok(()) }
            "lcdsc" => { validate_finite_parameter("LCDSC", value)?; self.params.p352 = value; self.mark_param_given(352); Ok(()) }
            "wcdsc" => { validate_finite_parameter("WCDSC", value)?; self.params.p353 = value; self.mark_param_given(353); Ok(()) }
            "pcdsc" => { validate_finite_parameter("PCDSC", value)?; self.params.p354 = value; self.mark_param_given(354); Ok(()) }
            "lcdscd" => { validate_finite_parameter("LCDSCD", value)?; self.params.p355 = value; self.mark_param_given(355); Ok(()) }
            "wcdscd" => { validate_finite_parameter("WCDSCD", value)?; self.params.p356 = value; self.mark_param_given(356); Ok(()) }
            "pcdscd" => { validate_finite_parameter("PCDSCD", value)?; self.params.p357 = value; self.mark_param_given(357); Ok(()) }
            "lcbgcbg" => { validate_finite_parameter("LCBGCBG", value)?; self.params.p358 = value; self.mark_param_given(358); Ok(()) }
            "wcbgcbg" => { validate_finite_parameter("WCBGCBG", value)?; self.params.p359 = value; self.mark_param_given(359); Ok(()) }
            "pcbgcbg" => { validate_finite_parameter("PCBGCBG", value)?; self.params.p360 = value; self.mark_param_given(360); Ok(()) }
            "lbpfactorpw" => { validate_finite_parameter("LBPFACTORPW", value)?; self.params.p361 = value; self.mark_param_given(361); Ok(()) }
            "wbpfactorpw" => { validate_finite_parameter("WBPFACTORPW", value)?; self.params.p362 = value; self.mark_param_given(362); Ok(()) }
            "pbpfactorpw" => { validate_finite_parameter("PBPFACTORPW", value)?; self.params.p363 = value; self.mark_param_given(363); Ok(()) }
            "lvknee1pw" => { validate_finite_parameter("LVKNEE1PW", value)?; self.params.p364 = value; self.mark_param_given(364); Ok(()) }
            "wvknee1pw" => { validate_finite_parameter("WVKNEE1PW", value)?; self.params.p365 = value; self.mark_param_given(365); Ok(()) }
            "pvknee1pw" => { validate_finite_parameter("PVKNEE1PW", value)?; self.params.p366 = value; self.mark_param_given(366); Ok(()) }
            "lvknee2pw" => { validate_finite_parameter("LVKNEE2PW", value)?; self.params.p367 = value; self.mark_param_given(367); Ok(()) }
            "wvknee2pw" => { validate_finite_parameter("WVKNEE2PW", value)?; self.params.p368 = value; self.mark_param_given(368); Ok(()) }
            "pvknee2pw" => { validate_finite_parameter("PVKNEE2PW", value)?; self.params.p369 = value; self.mark_param_given(369); Ok(()) }
            "ldbgpw" => { validate_finite_parameter("LDBGPW", value)?; self.params.p370 = value; self.mark_param_given(370); Ok(()) }
            "wdbgpw" => { validate_finite_parameter("WDBGPW", value)?; self.params.p371 = value; self.mark_param_given(371); Ok(()) }
            "pdbgpw" => { validate_finite_parameter("PDBGPW", value)?; self.params.p372 = value; self.mark_param_given(372); Ok(()) }
            "lkbg0pw" => { validate_finite_parameter("LKBG0PW", value)?; self.params.p373 = value; self.mark_param_given(373); Ok(()) }
            "wkbg0pw" => { validate_finite_parameter("WKBG0PW", value)?; self.params.p374 = value; self.mark_param_given(374); Ok(()) }
            "pkbg0pw" => { validate_finite_parameter("PKBG0PW", value)?; self.params.p375 = value; self.mark_param_given(375); Ok(()) }
            "lkbg1pw" => { validate_finite_parameter("LKBG1PW", value)?; self.params.p376 = value; self.mark_param_given(376); Ok(()) }
            "wkbg1pw" => { validate_finite_parameter("WKBG1PW", value)?; self.params.p377 = value; self.mark_param_given(377); Ok(()) }
            "pkbg1pw" => { validate_finite_parameter("PKBG1PW", value)?; self.params.p378 = value; self.mark_param_given(378); Ok(()) }
            "lkbg2pw" => { validate_finite_parameter("LKBG2PW", value)?; self.params.p379 = value; self.mark_param_given(379); Ok(()) }
            "wkbg2pw" => { validate_finite_parameter("WKBG2PW", value)?; self.params.p380 = value; self.mark_param_given(380); Ok(()) }
            "pkbg2pw" => { validate_finite_parameter("PKBG2PW", value)?; self.params.p381 = value; self.mark_param_given(381); Ok(()) }
            "lbpfactornw" => { validate_finite_parameter("LBPFACTORNW", value)?; self.params.p382 = value; self.mark_param_given(382); Ok(()) }
            "wbpfactornw" => { validate_finite_parameter("WBPFACTORNW", value)?; self.params.p383 = value; self.mark_param_given(383); Ok(()) }
            "pbpfactornw" => { validate_finite_parameter("PBPFACTORNW", value)?; self.params.p384 = value; self.mark_param_given(384); Ok(()) }
            "lvknee1nw" => { validate_finite_parameter("LVKNEE1NW", value)?; self.params.p385 = value; self.mark_param_given(385); Ok(()) }
            "wvknee1nw" => { validate_finite_parameter("WVKNEE1NW", value)?; self.params.p386 = value; self.mark_param_given(386); Ok(()) }
            "pvknee1nw" => { validate_finite_parameter("PVKNEE1NW", value)?; self.params.p387 = value; self.mark_param_given(387); Ok(()) }
            "lvknee2nw" => { validate_finite_parameter("LVKNEE2NW", value)?; self.params.p388 = value; self.mark_param_given(388); Ok(()) }
            "wvknee2nw" => { validate_finite_parameter("WVKNEE2NW", value)?; self.params.p389 = value; self.mark_param_given(389); Ok(()) }
            "pvknee2nw" => { validate_finite_parameter("PVKNEE2NW", value)?; self.params.p390 = value; self.mark_param_given(390); Ok(()) }
            "ldbgnw" => { validate_finite_parameter("LDBGNW", value)?; self.params.p391 = value; self.mark_param_given(391); Ok(()) }
            "wdbgnw" => { validate_finite_parameter("WDBGNW", value)?; self.params.p392 = value; self.mark_param_given(392); Ok(()) }
            "pdbgnw" => { validate_finite_parameter("PDBGNW", value)?; self.params.p393 = value; self.mark_param_given(393); Ok(()) }
            "lkbg0nw" => { validate_finite_parameter("LKBG0NW", value)?; self.params.p394 = value; self.mark_param_given(394); Ok(()) }
            "wkbg0nw" => { validate_finite_parameter("WKBG0NW", value)?; self.params.p395 = value; self.mark_param_given(395); Ok(()) }
            "pkbg0nw" => { validate_finite_parameter("PKBG0NW", value)?; self.params.p396 = value; self.mark_param_given(396); Ok(()) }
            "lkbg1nw" => { validate_finite_parameter("LKBG1NW", value)?; self.params.p397 = value; self.mark_param_given(397); Ok(()) }
            "wkbg1nw" => { validate_finite_parameter("WKBG1NW", value)?; self.params.p398 = value; self.mark_param_given(398); Ok(()) }
            "pkbg1nw" => { validate_finite_parameter("PKBG1NW", value)?; self.params.p399 = value; self.mark_param_given(399); Ok(()) }
            "lkbg2nw" => { validate_finite_parameter("LKBG2NW", value)?; self.params.p400 = value; self.mark_param_given(400); Ok(()) }
            "wkbg2nw" => { validate_finite_parameter("WKBG2NW", value)?; self.params.p401 = value; self.mark_param_given(401); Ok(()) }
            "pkbg2nw" => { validate_finite_parameter("PKBG2NW", value)?; self.params.p402 = value; self.mark_param_given(402); Ok(()) }
            "ldvt0" => { validate_finite_parameter("LDVT0", value)?; self.params.p403 = value; self.mark_param_given(403); Ok(()) }
            "wdvt0" => { validate_finite_parameter("WDVT0", value)?; self.params.p404 = value; self.mark_param_given(404); Ok(()) }
            "pdvt0" => { validate_finite_parameter("PDVT0", value)?; self.params.p405 = value; self.mark_param_given(405); Ok(()) }
            "ldvt1" => { validate_finite_parameter("LDVT1", value)?; self.params.p406 = value; self.mark_param_given(406); Ok(()) }
            "wdvt1" => { validate_finite_parameter("WDVT1", value)?; self.params.p407 = value; self.mark_param_given(407); Ok(()) }
            "pdvt1" => { validate_finite_parameter("PDVT1", value)?; self.params.p408 = value; self.mark_param_given(408); Ok(()) }
            "lphin" => { validate_finite_parameter("LPHIN", value)?; self.params.p409 = value; self.mark_param_given(409); Ok(()) }
            "wphin" => { validate_finite_parameter("WPHIN", value)?; self.params.p410 = value; self.mark_param_given(410); Ok(()) }
            "pphin" => { validate_finite_parameter("PPHIN", value)?; self.params.p411 = value; self.mark_param_given(411); Ok(()) }
            "leta0" => { validate_finite_parameter("LETA0", value)?; self.params.p412 = value; self.mark_param_given(412); Ok(()) }
            "weta0" => { validate_finite_parameter("WETA0", value)?; self.params.p413 = value; self.mark_param_given(413); Ok(()) }
            "peta0" => { validate_finite_parameter("PETA0", value)?; self.params.p414 = value; self.mark_param_given(414); Ok(()) }
            "leta1" => { validate_finite_parameter("LETA1", value)?; self.params.p415 = value; self.mark_param_given(415); Ok(()) }
            "weta1" => { validate_finite_parameter("WETA1", value)?; self.params.p416 = value; self.mark_param_given(416); Ok(()) }
            "peta1" => { validate_finite_parameter("PETA1", value)?; self.params.p417 = value; self.mark_param_given(417); Ok(()) }
            "letab" => { validate_finite_parameter("LETAB", value)?; self.params.p418 = value; self.mark_param_given(418); Ok(()) }
            "wetab" => { validate_finite_parameter("WETAB", value)?; self.params.p419 = value; self.mark_param_given(419); Ok(()) }
            "petab" => { validate_finite_parameter("PETAB", value)?; self.params.p420 = value; self.mark_param_given(420); Ok(()) }
            "ldsub" => { validate_finite_parameter("LDSUB", value)?; self.params.p421 = value; self.mark_param_given(421); Ok(()) }
            "wdsub" => { validate_finite_parameter("WDSUB", value)?; self.params.p422 = value; self.mark_param_given(422); Ok(()) }
            "pdsub" => { validate_finite_parameter("PDSUB", value)?; self.params.p423 = value; self.mark_param_given(423); Ok(()) }
            "lk1rsce" => { validate_finite_parameter("LK1RSCE", value)?; self.params.p424 = value; self.mark_param_given(424); Ok(()) }
            "wk1rsce" => { validate_finite_parameter("WK1RSCE", value)?; self.params.p425 = value; self.mark_param_given(425); Ok(()) }
            "pk1rsce" => { validate_finite_parameter("PK1RSCE", value)?; self.params.p426 = value; self.mark_param_given(426); Ok(()) }
            "llpe0" => { validate_finite_parameter("LLPE0", value)?; self.params.p427 = value; self.mark_param_given(427); Ok(()) }
            "wlpe0" => { validate_finite_parameter("WLPE0", value)?; self.params.p428 = value; self.mark_param_given(428); Ok(()) }
            "plpe0" => { validate_finite_parameter("PLPE0", value)?; self.params.p429 = value; self.mark_param_given(429); Ok(()) }
            "ldsc0" => { validate_finite_parameter("LDSC0", value)?; self.params.p430 = value; self.mark_param_given(430); Ok(()) }
            "wdsc0" => { validate_finite_parameter("WDSC0", value)?; self.params.p431 = value; self.mark_param_given(431); Ok(()) }
            "pdsc0" => { validate_finite_parameter("PDSC0", value)?; self.params.p432 = value; self.mark_param_given(432); Ok(()) }
            "ldsc1" => { validate_finite_parameter("LDSC1", value)?; self.params.p433 = value; self.mark_param_given(433); Ok(()) }
            "wdsc1" => { validate_finite_parameter("WDSC1", value)?; self.params.p434 = value; self.mark_param_given(434); Ok(()) }
            "pdsc1" => { validate_finite_parameter("PDSC1", value)?; self.params.p435 = value; self.mark_param_given(435); Ok(()) }
            "lascl" => { validate_finite_parameter("LASCL", value)?; self.params.p436 = value; self.mark_param_given(436); Ok(()) }
            "wascl" => { validate_finite_parameter("WASCL", value)?; self.params.p437 = value; self.mark_param_given(437); Ok(()) }
            "pascl" => { validate_finite_parameter("PASCL", value)?; self.params.p438 = value; self.mark_param_given(438); Ok(()) }
            "lbscl" => { validate_finite_parameter("LBSCL", value)?; self.params.p439 = value; self.mark_param_given(439); Ok(()) }
            "wbscl" => { validate_finite_parameter("WBSCL", value)?; self.params.p440 = value; self.mark_param_given(440); Ok(()) }
            "pbscl" => { validate_finite_parameter("PBSCL", value)?; self.params.p441 = value; self.mark_param_given(441); Ok(()) }
            "lk0" => { validate_finite_parameter("LK0", value)?; self.params.p442 = value; self.mark_param_given(442); Ok(()) }
            "wk0" => { validate_finite_parameter("WK0", value)?; self.params.p443 = value; self.mark_param_given(443); Ok(()) }
            "pk0" => { validate_finite_parameter("PK0", value)?; self.params.p444 = value; self.mark_param_given(444); Ok(()) }
            "lk01" => { validate_finite_parameter("LK01", value)?; self.params.p445 = value; self.mark_param_given(445); Ok(()) }
            "wk01" => { validate_finite_parameter("WK01", value)?; self.params.p446 = value; self.mark_param_given(446); Ok(()) }
            "pk01" => { validate_finite_parameter("PK01", value)?; self.params.p447 = value; self.mark_param_given(447); Ok(()) }
            "lk0si" => { validate_finite_parameter("LK0SI", value)?; self.params.p448 = value; self.mark_param_given(448); Ok(()) }
            "wk0si" => { validate_finite_parameter("WK0SI", value)?; self.params.p449 = value; self.mark_param_given(449); Ok(()) }
            "pk0si" => { validate_finite_parameter("PK0SI", value)?; self.params.p450 = value; self.mark_param_given(450); Ok(()) }
            "lk0si1" => { validate_finite_parameter("LK0SI1", value)?; self.params.p451 = value; self.mark_param_given(451); Ok(()) }
            "wk0si1" => { validate_finite_parameter("WK0SI1", value)?; self.params.p452 = value; self.mark_param_given(452); Ok(()) }
            "pk0si1" => { validate_finite_parameter("PK0SI1", value)?; self.params.p453 = value; self.mark_param_given(453); Ok(()) }
            "lk0sisat" => { validate_finite_parameter("LK0SISAT", value)?; self.params.p454 = value; self.mark_param_given(454); Ok(()) }
            "nk0sisat" => { validate_finite_parameter("NK0SISAT", value)?; self.params.p455 = value; self.mark_param_given(455); Ok(()) }
            "pk0sisat" => { validate_finite_parameter("PK0SISAT", value)?; self.params.p456 = value; self.mark_param_given(456); Ok(()) }
            "lk0sisat1" => { validate_finite_parameter("LK0SISAT1", value)?; self.params.p457 = value; self.mark_param_given(457); Ok(()) }
            "nk0sisat1" => { validate_finite_parameter("NK0SISAT1", value)?; self.params.p458 = value; self.mark_param_given(458); Ok(()) }
            "pk0sisat1" => { validate_finite_parameter("PK0SISAT1", value)?; self.params.p459 = value; self.mark_param_given(459); Ok(()) }
            "lmexp" => { validate_finite_parameter("LMEXP", value)?; self.params.p460 = value; self.mark_param_given(460); Ok(()) }
            "wmexp" => { validate_finite_parameter("WMEXP", value)?; self.params.p461 = value; self.mark_param_given(461); Ok(()) }
            "pmexp" => { validate_finite_parameter("PMEXP", value)?; self.params.p462 = value; self.mark_param_given(462); Ok(()) }
            "lptwg" => { validate_finite_parameter("LPTWG", value)?; self.params.p463 = value; self.mark_param_given(463); Ok(()) }
            "wptwg" => { validate_finite_parameter("WPTWG", value)?; self.params.p464 = value; self.mark_param_given(464); Ok(()) }
            "pptwg" => { validate_finite_parameter("PPTWG", value)?; self.params.p465 = value; self.mark_param_given(465); Ok(()) }
            "lptwgb" => { validate_finite_parameter("LPTWGB", value)?; self.params.p466 = value; self.mark_param_given(466); Ok(()) }
            "wptwgb" => { validate_finite_parameter("WPTWGB", value)?; self.params.p467 = value; self.mark_param_given(467); Ok(()) }
            "pptwgb" => { validate_finite_parameter("PPTWGB", value)?; self.params.p468 = value; self.mark_param_given(468); Ok(()) }
            "lptwgb2" => { validate_finite_parameter("LPTWGB2", value)?; self.params.p469 = value; self.mark_param_given(469); Ok(()) }
            "wptwgb2" => { validate_finite_parameter("WPTWGB2", value)?; self.params.p470 = value; self.mark_param_given(470); Ok(()) }
            "pptwgb2" => { validate_finite_parameter("PPTWGB2", value)?; self.params.p471 = value; self.mark_param_given(471); Ok(()) }
            "lptwgt" => { validate_finite_parameter("LPTWGT", value)?; self.params.p472 = value; self.mark_param_given(472); Ok(()) }
            "wptwgt" => { validate_finite_parameter("WPTWGT", value)?; self.params.p473 = value; self.mark_param_given(473); Ok(()) }
            "pptwgt" => { validate_finite_parameter("PPTWGT", value)?; self.params.p474 = value; self.mark_param_given(474); Ok(()) }
            "lu0" => { validate_finite_parameter("LU0", value)?; self.params.p475 = value; self.mark_param_given(475); Ok(()) }
            "wu0" => { validate_finite_parameter("WU0", value)?; self.params.p476 = value; self.mark_param_given(476); Ok(()) }
            "pu0" => { validate_finite_parameter("PU0", value)?; self.params.p477 = value; self.mark_param_given(477); Ok(()) }
            "lua" => { validate_finite_parameter("LUA", value)?; self.params.p478 = value; self.mark_param_given(478); Ok(()) }
            "wua" => { validate_finite_parameter("WUA", value)?; self.params.p479 = value; self.mark_param_given(479); Ok(()) }
            "pua" => { validate_finite_parameter("PUA", value)?; self.params.p480 = value; self.mark_param_given(480); Ok(()) }
            "luc" => { validate_finite_parameter("LUC", value)?; self.params.p481 = value; self.mark_param_given(481); Ok(()) }
            "wuc" => { validate_finite_parameter("WUC", value)?; self.params.p482 = value; self.mark_param_given(482); Ok(()) }
            "puc" => { validate_finite_parameter("PUC", value)?; self.params.p483 = value; self.mark_param_given(483); Ok(()) }
            "lud" => { validate_finite_parameter("LUD", value)?; self.params.p484 = value; self.mark_param_given(484); Ok(()) }
            "wud" => { validate_finite_parameter("WUD", value)?; self.params.p485 = value; self.mark_param_given(485); Ok(()) }
            "pud" => { validate_finite_parameter("PUD", value)?; self.params.p486 = value; self.mark_param_given(486); Ok(()) }
            "lucs" => { validate_finite_parameter("LUCS", value)?; self.params.p487 = value; self.mark_param_given(487); Ok(()) }
            "wucs" => { validate_finite_parameter("WUCS", value)?; self.params.p488 = value; self.mark_param_given(488); Ok(()) }
            "pucs" => { validate_finite_parameter("PUCS", value)?; self.params.p489 = value; self.mark_param_given(489); Ok(()) }
            "leu" => { validate_finite_parameter("LEU", value)?; self.params.p490 = value; self.mark_param_given(490); Ok(()) }
            "weu" => { validate_finite_parameter("WEU", value)?; self.params.p491 = value; self.mark_param_given(491); Ok(()) }
            "peu" => { validate_finite_parameter("PEU", value)?; self.params.p492 = value; self.mark_param_given(492); Ok(()) }
            "leub" => { validate_finite_parameter("LEUB", value)?; self.params.p493 = value; self.mark_param_given(493); Ok(()) }
            "weub" => { validate_finite_parameter("WEUB", value)?; self.params.p494 = value; self.mark_param_given(494); Ok(()) }
            "peub" => { validate_finite_parameter("PEUB", value)?; self.params.p495 = value; self.mark_param_given(495); Ok(()) }
            "lutl" => { validate_finite_parameter("LUTL", value)?; self.params.p496 = value; self.mark_param_given(496); Ok(()) }
            "wutl" => { validate_finite_parameter("WUTL", value)?; self.params.p497 = value; self.mark_param_given(497); Ok(()) }
            "putl" => { validate_finite_parameter("PUTL", value)?; self.params.p498 = value; self.mark_param_given(498); Ok(()) }
            "lute" => { validate_finite_parameter("LUTE", value)?; self.params.p499 = value; self.mark_param_given(499); Ok(()) }
            "wute" => { validate_finite_parameter("WUTE", value)?; self.params.p500 = value; self.mark_param_given(500); Ok(()) }
            "pute" => { validate_finite_parameter("PUTE", value)?; self.params.p501 = value; self.mark_param_given(501); Ok(()) }
            "lua1" => { validate_finite_parameter("LUA1", value)?; self.params.p502 = value; self.mark_param_given(502); Ok(()) }
            "wua1" => { validate_finite_parameter("WUA1", value)?; self.params.p503 = value; self.mark_param_given(503); Ok(()) }
            "pua1" => { validate_finite_parameter("PUA1", value)?; self.params.p504 = value; self.mark_param_given(504); Ok(()) }
            "lud1" => { validate_finite_parameter("LUD1", value)?; self.params.p505 = value; self.mark_param_given(505); Ok(()) }
            "wud1" => { validate_finite_parameter("WUD1", value)?; self.params.p506 = value; self.mark_param_given(506); Ok(()) }
            "pud1" => { validate_finite_parameter("PUD1", value)?; self.params.p507 = value; self.mark_param_given(507); Ok(()) }
            "lucste" => { validate_finite_parameter("LUCSTE", value)?; self.params.p508 = value; self.mark_param_given(508); Ok(()) }
            "wucste" => { validate_finite_parameter("WUCSTE", value)?; self.params.p509 = value; self.mark_param_given(509); Ok(()) }
            "pucste" => { validate_finite_parameter("PUCSTE", value)?; self.params.p510 = value; self.mark_param_given(510); Ok(()) }
            "letamob" => { validate_finite_parameter("LETAMOB", value)?; self.params.p511 = value; self.mark_param_given(511); Ok(()) }
            "wetamob" => { validate_finite_parameter("WETAMOB", value)?; self.params.p512 = value; self.mark_param_given(512); Ok(()) }
            "petamob" => { validate_finite_parameter("PETAMOB", value)?; self.params.p513 = value; self.mark_param_given(513); Ok(()) }
            "lu02" => { validate_finite_parameter("LU02", value)?; self.params.p514 = value; self.mark_param_given(514); Ok(()) }
            "wu02" => { validate_finite_parameter("WU02", value)?; self.params.p515 = value; self.mark_param_given(515); Ok(()) }
            "pu02" => { validate_finite_parameter("PU02", value)?; self.params.p516 = value; self.mark_param_given(516); Ok(()) }
            "lua2" => { validate_finite_parameter("LUA2", value)?; self.params.p517 = value; self.mark_param_given(517); Ok(()) }
            "wua2" => { validate_finite_parameter("WUA2", value)?; self.params.p518 = value; self.mark_param_given(518); Ok(()) }
            "pua2" => { validate_finite_parameter("PUA2", value)?; self.params.p519 = value; self.mark_param_given(519); Ok(()) }
            "luc2" => { validate_finite_parameter("LUC2", value)?; self.params.p520 = value; self.mark_param_given(520); Ok(()) }
            "wuc2" => { validate_finite_parameter("WUC2", value)?; self.params.p521 = value; self.mark_param_given(521); Ok(()) }
            "puc2" => { validate_finite_parameter("PUC2", value)?; self.params.p522 = value; self.mark_param_given(522); Ok(()) }
            "lud2" => { validate_finite_parameter("LUD2", value)?; self.params.p523 = value; self.mark_param_given(523); Ok(()) }
            "wud2" => { validate_finite_parameter("WUD2", value)?; self.params.p524 = value; self.mark_param_given(524); Ok(()) }
            "pud2" => { validate_finite_parameter("PUD2", value)?; self.params.p525 = value; self.mark_param_given(525); Ok(()) }
            "lucs2" => { validate_finite_parameter("LUCS2", value)?; self.params.p526 = value; self.mark_param_given(526); Ok(()) }
            "wucs2" => { validate_finite_parameter("WUCS2", value)?; self.params.p527 = value; self.mark_param_given(527); Ok(()) }
            "pucs2" => { validate_finite_parameter("PUCS2", value)?; self.params.p528 = value; self.mark_param_given(528); Ok(()) }
            "leu2" => { validate_finite_parameter("LEU2", value)?; self.params.p529 = value; self.mark_param_given(529); Ok(()) }
            "weu2" => { validate_finite_parameter("WEU2", value)?; self.params.p530 = value; self.mark_param_given(530); Ok(()) }
            "peu2" => { validate_finite_parameter("PEU2", value)?; self.params.p531 = value; self.mark_param_given(531); Ok(()) }
            "leub2" => { validate_finite_parameter("LEUB2", value)?; self.params.p532 = value; self.mark_param_given(532); Ok(()) }
            "weub2" => { validate_finite_parameter("WEUB2", value)?; self.params.p533 = value; self.mark_param_given(533); Ok(()) }
            "peub2" => { validate_finite_parameter("PEUB2", value)?; self.params.p534 = value; self.mark_param_given(534); Ok(()) }
            "letamob2" => { validate_finite_parameter("LETAMOB2", value)?; self.params.p535 = value; self.mark_param_given(535); Ok(()) }
            "wetamob2" => { validate_finite_parameter("WETAMOB2", value)?; self.params.p536 = value; self.mark_param_given(536); Ok(()) }
            "petamob2" => { validate_finite_parameter("PETAMOB2", value)?; self.params.p537 = value; self.mark_param_given(537); Ok(()) }
            "lat" => { validate_finite_parameter("LAT", value)?; self.params.p538 = value; self.mark_param_given(538); Ok(()) }
            "wat" => { validate_finite_parameter("WAT", value)?; self.params.p539 = value; self.mark_param_given(539); Ok(()) }
            "pat" => { validate_finite_parameter("PAT", value)?; self.params.p540 = value; self.mark_param_given(540); Ok(()) }
            "latb" => { validate_finite_parameter("LATB", value)?; self.params.p541 = value; self.mark_param_given(541); Ok(()) }
            "watb" => { validate_finite_parameter("WATB", value)?; self.params.p542 = value; self.mark_param_given(542); Ok(()) }
            "patb" => { validate_finite_parameter("PATB", value)?; self.params.p543 = value; self.mark_param_given(543); Ok(()) }
            "lprt" => { validate_finite_parameter("LPRT", value)?; self.params.p544 = value; self.mark_param_given(544); Ok(()) }
            "wprt" => { validate_finite_parameter("WPRT", value)?; self.params.p545 = value; self.mark_param_given(545); Ok(()) }
            "pprt" => { validate_finite_parameter("PPRT", value)?; self.params.p546 = value; self.mark_param_given(546); Ok(()) }
            "liit" => { validate_finite_parameter("LIIT", value)?; self.params.p547 = value; self.mark_param_given(547); Ok(()) }
            "wiit" => { validate_finite_parameter("WIIT", value)?; self.params.p548 = value; self.mark_param_given(548); Ok(()) }
            "piit" => { validate_finite_parameter("PIIT", value)?; self.params.p549 = value; self.mark_param_given(549); Ok(()) }
            "ltgidl" => { validate_finite_parameter("LTGIDL", value)?; self.params.p550 = value; self.mark_param_given(550); Ok(()) }
            "wtgidl" => { validate_finite_parameter("WTGIDL", value)?; self.params.p551 = value; self.mark_param_given(551); Ok(()) }
            "ptgidl" => { validate_finite_parameter("PTGIDL", value)?; self.params.p552 = value; self.mark_param_given(552); Ok(()) }
            "ltgisl" => { validate_finite_parameter("LTGISL", value)?; self.params.p553 = value; self.mark_param_given(553); Ok(()) }
            "wtgisl" => { validate_finite_parameter("WTGISL", value)?; self.params.p554 = value; self.mark_param_given(554); Ok(()) }
            "ptgisl" => { validate_finite_parameter("PTGISL", value)?; self.params.p555 = value; self.mark_param_given(555); Ok(()) }
            "ligt" => { validate_finite_parameter("LIGT", value)?; self.params.p556 = value; self.mark_param_given(556); Ok(()) }
            "wigt" => { validate_finite_parameter("WIGT", value)?; self.params.p557 = value; self.mark_param_given(557); Ok(()) }
            "pigt" => { validate_finite_parameter("PIGT", value)?; self.params.p558 = value; self.mark_param_given(558); Ok(()) }
            "lpclm" => { validate_finite_parameter("LPCLM", value)?; self.params.p559 = value; self.mark_param_given(559); Ok(()) }
            "wpclm" => { validate_finite_parameter("WPCLM", value)?; self.params.p560 = value; self.mark_param_given(560); Ok(()) }
            "ppclm" => { validate_finite_parameter("PPCLM", value)?; self.params.p561 = value; self.mark_param_given(561); Ok(()) }
            "lpclmcv" => { validate_finite_parameter("LPCLMCV", value)?; self.params.p562 = value; self.mark_param_given(562); Ok(()) }
            "wpclmcv" => { validate_finite_parameter("WPCLMCV", value)?; self.params.p563 = value; self.mark_param_given(563); Ok(()) }
            "ppclmcv" => { validate_finite_parameter("PPCLMCV", value)?; self.params.p564 = value; self.mark_param_given(564); Ok(()) }
            "ldrout" => { validate_finite_parameter("LDROUT", value)?; self.params.p565 = value; self.mark_param_given(565); Ok(()) }
            "wdrout" => { validate_finite_parameter("WDROUT", value)?; self.params.p566 = value; self.mark_param_given(566); Ok(()) }
            "pdrout" => { validate_finite_parameter("PDROUT", value)?; self.params.p567 = value; self.mark_param_given(567); Ok(()) }
            "lpdibl1" => { validate_finite_parameter("LPDIBL1", value)?; self.params.p568 = value; self.mark_param_given(568); Ok(()) }
            "wpdibl1" => { validate_finite_parameter("WPDIBL1", value)?; self.params.p569 = value; self.mark_param_given(569); Ok(()) }
            "ppdibl1" => { validate_finite_parameter("PPDIBL1", value)?; self.params.p570 = value; self.mark_param_given(570); Ok(()) }
            "lpdibl2" => { validate_finite_parameter("LPDIBL2", value)?; self.params.p571 = value; self.mark_param_given(571); Ok(()) }
            "wpdibl2" => { validate_finite_parameter("WPDIBL2", value)?; self.params.p572 = value; self.mark_param_given(572); Ok(()) }
            "ppdibl2" => { validate_finite_parameter("PPDIBL2", value)?; self.params.p573 = value; self.mark_param_given(573); Ok(()) }
            "lpvag" => { validate_finite_parameter("LPVAG", value)?; self.params.p574 = value; self.mark_param_given(574); Ok(()) }
            "wpvag" => { validate_finite_parameter("WPVAG", value)?; self.params.p575 = value; self.mark_param_given(575); Ok(()) }
            "ppvag" => { validate_finite_parameter("PPVAG", value)?; self.params.p576 = value; self.mark_param_given(576); Ok(()) }
            "lalpha0" => { validate_finite_parameter("LALPHA0", value)?; self.params.p577 = value; self.mark_param_given(577); Ok(()) }
            "walpha0" => { validate_finite_parameter("WALPHA0", value)?; self.params.p578 = value; self.mark_param_given(578); Ok(()) }
            "palpha0" => { validate_finite_parameter("PALPHA0", value)?; self.params.p579 = value; self.mark_param_given(579); Ok(()) }
            "lalpha1" => { validate_finite_parameter("LALPHA1", value)?; self.params.p580 = value; self.mark_param_given(580); Ok(()) }
            "walpha1" => { validate_finite_parameter("WALPHA1", value)?; self.params.p581 = value; self.mark_param_given(581); Ok(()) }
            "palpha1" => { validate_finite_parameter("PALPHA1", value)?; self.params.p582 = value; self.mark_param_given(582); Ok(()) }
            "lbeta0" => { validate_finite_parameter("LBETA0", value)?; self.params.p583 = value; self.mark_param_given(583); Ok(()) }
            "wbeta0" => { validate_finite_parameter("WBETA0", value)?; self.params.p584 = value; self.mark_param_given(584); Ok(()) }
            "pbeta0" => { validate_finite_parameter("PBETA0", value)?; self.params.p585 = value; self.mark_param_given(585); Ok(()) }
            "laigc" => { validate_finite_parameter("LAIGC", value)?; self.params.p586 = value; self.mark_param_given(586); Ok(()) }
            "waigc" => { validate_finite_parameter("WAIGC", value)?; self.params.p587 = value; self.mark_param_given(587); Ok(()) }
            "paigc" => { validate_finite_parameter("PAIGC", value)?; self.params.p588 = value; self.mark_param_given(588); Ok(()) }
            "lbigc" => { validate_finite_parameter("LBIGC", value)?; self.params.p589 = value; self.mark_param_given(589); Ok(()) }
            "wbigc" => { validate_finite_parameter("WBIGC", value)?; self.params.p590 = value; self.mark_param_given(590); Ok(()) }
            "pbigc" => { validate_finite_parameter("PBIGC", value)?; self.params.p591 = value; self.mark_param_given(591); Ok(()) }
            "lcigc" => { validate_finite_parameter("LCIGC", value)?; self.params.p592 = value; self.mark_param_given(592); Ok(()) }
            "wcigc" => { validate_finite_parameter("WCIGC", value)?; self.params.p593 = value; self.mark_param_given(593); Ok(()) }
            "pcigc" => { validate_finite_parameter("PCIGC", value)?; self.params.p594 = value; self.mark_param_given(594); Ok(()) }
            "ldigc" => { validate_finite_parameter("LDIGC", value)?; self.params.p595 = value; self.mark_param_given(595); Ok(()) }
            "wdigc" => { validate_finite_parameter("WDIGC", value)?; self.params.p596 = value; self.mark_param_given(596); Ok(()) }
            "pdigc" => { validate_finite_parameter("PDIGC", value)?; self.params.p597 = value; self.mark_param_given(597); Ok(()) }
            "lpigcd" => { validate_finite_parameter("LPIGCD", value)?; self.params.p598 = value; self.mark_param_given(598); Ok(()) }
            "wpigcd" => { validate_finite_parameter("WPIGCD", value)?; self.params.p599 = value; self.mark_param_given(599); Ok(()) }
            "ppigcd" => { validate_finite_parameter("PPIGCD", value)?; self.params.p600 = value; self.mark_param_given(600); Ok(()) }
            "lagidl" => { validate_finite_parameter("LAGIDL", value)?; self.params.p601 = value; self.mark_param_given(601); Ok(()) }
            "wagidl" => { validate_finite_parameter("WAGIDL", value)?; self.params.p602 = value; self.mark_param_given(602); Ok(()) }
            "pagidl" => { validate_finite_parameter("PAGIDL", value)?; self.params.p603 = value; self.mark_param_given(603); Ok(()) }
            "lbgidl" => { validate_finite_parameter("LBGIDL", value)?; self.params.p604 = value; self.mark_param_given(604); Ok(()) }
            "wbgidl" => { validate_finite_parameter("WBGIDL", value)?; self.params.p605 = value; self.mark_param_given(605); Ok(()) }
            "pbgidl" => { validate_finite_parameter("PBGIDL", value)?; self.params.p606 = value; self.mark_param_given(606); Ok(()) }
            "legidl" => { validate_finite_parameter("LEGIDL", value)?; self.params.p607 = value; self.mark_param_given(607); Ok(()) }
            "wegidl" => { validate_finite_parameter("WEGIDL", value)?; self.params.p608 = value; self.mark_param_given(608); Ok(()) }
            "pegidl" => { validate_finite_parameter("PEGIDL", value)?; self.params.p609 = value; self.mark_param_given(609); Ok(()) }
            "lpgidl" => { validate_finite_parameter("LPGIDL", value)?; self.params.p610 = value; self.mark_param_given(610); Ok(()) }
            "wpgidl" => { validate_finite_parameter("WPGIDL", value)?; self.params.p611 = value; self.mark_param_given(611); Ok(()) }
            "ppgidl" => { validate_finite_parameter("PPGIDL", value)?; self.params.p612 = value; self.mark_param_given(612); Ok(()) }
            "lvbgidl" => { validate_finite_parameter("LVBGIDL", value)?; self.params.p613 = value; self.mark_param_given(613); Ok(()) }
            "wvbgidl" => { validate_finite_parameter("WVBGIDL", value)?; self.params.p614 = value; self.mark_param_given(614); Ok(()) }
            "pvbgidl" => { validate_finite_parameter("PVBGIDL", value)?; self.params.p615 = value; self.mark_param_given(615); Ok(()) }
            "lvbegidl" => { validate_finite_parameter("LVBEGIDL", value)?; self.params.p616 = value; self.mark_param_given(616); Ok(()) }
            "wvbegidl" => { validate_finite_parameter("WVBEGIDL", value)?; self.params.p617 = value; self.mark_param_given(617); Ok(()) }
            "pvbegidl" => { validate_finite_parameter("PVBEGIDL", value)?; self.params.p618 = value; self.mark_param_given(618); Ok(()) }
            "lagisl" => { validate_finite_parameter("LAGISL", value)?; self.params.p619 = value; self.mark_param_given(619); Ok(()) }
            "wagisl" => { validate_finite_parameter("WAGISL", value)?; self.params.p620 = value; self.mark_param_given(620); Ok(()) }
            "pagisl" => { validate_finite_parameter("PAGISL", value)?; self.params.p621 = value; self.mark_param_given(621); Ok(()) }
            "lbgisl" => { validate_finite_parameter("LBGISL", value)?; self.params.p622 = value; self.mark_param_given(622); Ok(()) }
            "wbgisl" => { validate_finite_parameter("WBGISL", value)?; self.params.p623 = value; self.mark_param_given(623); Ok(()) }
            "pbgisl" => { validate_finite_parameter("PBGISL", value)?; self.params.p624 = value; self.mark_param_given(624); Ok(()) }
            "legisl" => { validate_finite_parameter("LEGISL", value)?; self.params.p625 = value; self.mark_param_given(625); Ok(()) }
            "wegisl" => { validate_finite_parameter("WEGISL", value)?; self.params.p626 = value; self.mark_param_given(626); Ok(()) }
            "pegisl" => { validate_finite_parameter("PEGISL", value)?; self.params.p627 = value; self.mark_param_given(627); Ok(()) }
            "lpgisl" => { validate_finite_parameter("LPGISL", value)?; self.params.p628 = value; self.mark_param_given(628); Ok(()) }
            "wpgisl" => { validate_finite_parameter("WPGISL", value)?; self.params.p629 = value; self.mark_param_given(629); Ok(()) }
            "ppgisl" => { validate_finite_parameter("PPGISL", value)?; self.params.p630 = value; self.mark_param_given(630); Ok(()) }
            "lvbgisl" => { validate_finite_parameter("LVBGISL", value)?; self.params.p631 = value; self.mark_param_given(631); Ok(()) }
            "wvbgisl" => { validate_finite_parameter("WVBGISL", value)?; self.params.p632 = value; self.mark_param_given(632); Ok(()) }
            "pvbgisl" => { validate_finite_parameter("PVBGISL", value)?; self.params.p633 = value; self.mark_param_given(633); Ok(()) }
            "lvbegisl" => { validate_finite_parameter("LVBEGISL", value)?; self.params.p634 = value; self.mark_param_given(634); Ok(()) }
            "wvbegisl" => { validate_finite_parameter("WVBEGISL", value)?; self.params.p635 = value; self.mark_param_given(635); Ok(()) }
            "pvbegisl" => { validate_finite_parameter("PVBEGISL", value)?; self.params.p636 = value; self.mark_param_given(636); Ok(()) }
            "laigs" => { validate_finite_parameter("LAIGS", value)?; self.params.p637 = value; self.mark_param_given(637); Ok(()) }
            "waigs" => { validate_finite_parameter("WAIGS", value)?; self.params.p638 = value; self.mark_param_given(638); Ok(()) }
            "paigs" => { validate_finite_parameter("PAIGS", value)?; self.params.p639 = value; self.mark_param_given(639); Ok(()) }
            "laigd" => { validate_finite_parameter("LAIGD", value)?; self.params.p640 = value; self.mark_param_given(640); Ok(()) }
            "waigd" => { validate_finite_parameter("WAIGD", value)?; self.params.p641 = value; self.mark_param_given(641); Ok(()) }
            "paigd" => { validate_finite_parameter("PAIGD", value)?; self.params.p642 = value; self.mark_param_given(642); Ok(()) }
            "lbigs" => { validate_finite_parameter("LBIGS", value)?; self.params.p643 = value; self.mark_param_given(643); Ok(()) }
            "wbigs" => { validate_finite_parameter("WBIGS", value)?; self.params.p644 = value; self.mark_param_given(644); Ok(()) }
            "pbigs" => { validate_finite_parameter("PBIGS", value)?; self.params.p645 = value; self.mark_param_given(645); Ok(()) }
            "lbigd" => { validate_finite_parameter("LBIGD", value)?; self.params.p646 = value; self.mark_param_given(646); Ok(()) }
            "wbigd" => { validate_finite_parameter("WBIGD", value)?; self.params.p647 = value; self.mark_param_given(647); Ok(()) }
            "pbigd" => { validate_finite_parameter("PBIGD", value)?; self.params.p648 = value; self.mark_param_given(648); Ok(()) }
            "lcigs" => { validate_finite_parameter("LCIGS", value)?; self.params.p649 = value; self.mark_param_given(649); Ok(()) }
            "wcigs" => { validate_finite_parameter("WCIGS", value)?; self.params.p650 = value; self.mark_param_given(650); Ok(()) }
            "pcigs" => { validate_finite_parameter("PCIGS", value)?; self.params.p651 = value; self.mark_param_given(651); Ok(()) }
            "lcigd" => { validate_finite_parameter("LCIGD", value)?; self.params.p652 = value; self.mark_param_given(652); Ok(()) }
            "wcigd" => { validate_finite_parameter("WCIGD", value)?; self.params.p653 = value; self.mark_param_given(653); Ok(()) }
            "pcigd" => { validate_finite_parameter("PCIGD", value)?; self.params.p654 = value; self.mark_param_given(654); Ok(()) }
            "ldigs" => { validate_finite_parameter("LDIGS", value)?; self.params.p655 = value; self.mark_param_given(655); Ok(()) }
            "wdigs" => { validate_finite_parameter("WDIGS", value)?; self.params.p656 = value; self.mark_param_given(656); Ok(()) }
            "pdigs" => { validate_finite_parameter("PDIGS", value)?; self.params.p657 = value; self.mark_param_given(657); Ok(()) }
            "ldigd" => { validate_finite_parameter("LDIGD", value)?; self.params.p658 = value; self.mark_param_given(658); Ok(()) }
            "wdigd" => { validate_finite_parameter("WDIGD", value)?; self.params.p659 = value; self.mark_param_given(659); Ok(()) }
            "pdigd" => { validate_finite_parameter("PDIGD", value)?; self.params.p660 = value; self.mark_param_given(660); Ok(()) }
            "lntox" => { validate_finite_parameter("LNTOX", value)?; self.params.p661 = value; self.mark_param_given(661); Ok(()) }
            "wntox" => { validate_finite_parameter("WNTOX", value)?; self.params.p662 = value; self.mark_param_given(662); Ok(()) }
            "pntox" => { validate_finite_parameter("PNTOX", value)?; self.params.p663 = value; self.mark_param_given(663); Ok(()) }
            "lpoxedge" => { validate_finite_parameter("LPOXEDGE", value)?; self.params.p664 = value; self.mark_param_given(664); Ok(()) }
            "wpoxedge" => { validate_finite_parameter("WPOXEDGE", value)?; self.params.p665 = value; self.mark_param_given(665); Ok(()) }
            "ppoxedge" => { validate_finite_parameter("PPOXEDGE", value)?; self.params.p666 = value; self.mark_param_given(666); Ok(()) }
            "llovs" => { validate_finite_parameter("LLOVS", value)?; self.params.p667 = value; self.mark_param_given(667); Ok(()) }
            "wlovs" => { validate_finite_parameter("WLOVS", value)?; self.params.p668 = value; self.mark_param_given(668); Ok(()) }
            "plovs" => { validate_finite_parameter("PLOVS", value)?; self.params.p669 = value; self.mark_param_given(669); Ok(()) }
            "llovd" => { validate_finite_parameter("LLOVD", value)?; self.params.p670 = value; self.mark_param_given(670); Ok(()) }
            "wlovd" => { validate_finite_parameter("WLOVD", value)?; self.params.p671 = value; self.mark_param_given(671); Ok(()) }
            "plovd" => { validate_finite_parameter("PLOVD", value)?; self.params.p672 = value; self.mark_param_given(672); Ok(()) }
            "lcfs" => { validate_finite_parameter("LCFS", value)?; self.params.p673 = value; self.mark_param_given(673); Ok(()) }
            "wcfs" => { validate_finite_parameter("WCFS", value)?; self.params.p674 = value; self.mark_param_given(674); Ok(()) }
            "pcfs" => { validate_finite_parameter("PCFS", value)?; self.params.p675 = value; self.mark_param_given(675); Ok(()) }
            "lcfd" => { validate_finite_parameter("LCFD", value)?; self.params.p676 = value; self.mark_param_given(676); Ok(()) }
            "wcfd" => { validate_finite_parameter("WCFD", value)?; self.params.p677 = value; self.mark_param_given(677); Ok(()) }
            "pcfd" => { validate_finite_parameter("PCFD", value)?; self.params.p678 = value; self.mark_param_given(678); Ok(()) }
            "lvsat" => { validate_finite_parameter("LVSAT", value)?; self.params.p679 = value; self.mark_param_given(679); Ok(()) }
            "wvsat" => { validate_finite_parameter("WVSAT", value)?; self.params.p680 = value; self.mark_param_given(680); Ok(()) }
            "pvsat" => { validate_finite_parameter("PVSAT", value)?; self.params.p681 = value; self.mark_param_given(681); Ok(()) }
            "lvsatb" => { validate_finite_parameter("LVSATB", value)?; self.params.p682 = value; self.mark_param_given(682); Ok(()) }
            "wvsatb" => { validate_finite_parameter("WVSATB", value)?; self.params.p683 = value; self.mark_param_given(683); Ok(()) }
            "pvsatb" => { validate_finite_parameter("PVSATB", value)?; self.params.p684 = value; self.mark_param_given(684); Ok(()) }
            "lvsat1" => { validate_finite_parameter("LVSAT1", value)?; self.params.p685 = value; self.mark_param_given(685); Ok(()) }
            "wvsat1" => { validate_finite_parameter("WVSAT1", value)?; self.params.p686 = value; self.mark_param_given(686); Ok(()) }
            "pvsat1" => { validate_finite_parameter("PVSAT1", value)?; self.params.p687 = value; self.mark_param_given(687); Ok(()) }
            "lvsatcv" => { validate_finite_parameter("LVSATCV", value)?; self.params.p688 = value; self.mark_param_given(688); Ok(()) }
            "wvsatcv" => { validate_finite_parameter("WVSATCV", value)?; self.params.p689 = value; self.mark_param_given(689); Ok(()) }
            "pvsatcv" => { validate_finite_parameter("PVSATCV", value)?; self.params.p690 = value; self.mark_param_given(690); Ok(()) }
            "lksativ" => { validate_finite_parameter("LKSATIV", value)?; self.params.p691 = value; self.mark_param_given(691); Ok(()) }
            "wksativ" => { validate_finite_parameter("WKSATIV", value)?; self.params.p692 = value; self.mark_param_given(692); Ok(()) }
            "pksativ" => { validate_finite_parameter("PKSATIV", value)?; self.params.p693 = value; self.mark_param_given(693); Ok(()) }
            "lksubiv" => { validate_finite_parameter("LKSUBIV", value)?; self.params.p694 = value; self.mark_param_given(694); Ok(()) }
            "wksubiv" => { validate_finite_parameter("WKSUBIV", value)?; self.params.p695 = value; self.mark_param_given(695); Ok(()) }
            "pksubiv" => { validate_finite_parameter("PKSUBIV", value)?; self.params.p696 = value; self.mark_param_given(696); Ok(()) }
            "lksativb" => { validate_finite_parameter("LKSATIVB", value)?; self.params.p697 = value; self.mark_param_given(697); Ok(()) }
            "wksativb" => { validate_finite_parameter("WKSATIVB", value)?; self.params.p698 = value; self.mark_param_given(698); Ok(()) }
            "pksativb" => { validate_finite_parameter("PKSATIVB", value)?; self.params.p699 = value; self.mark_param_given(699); Ok(()) }
            "lup" => { validate_finite_parameter("LUP", value)?; self.params.p700 = value; self.mark_param_given(700); Ok(()) }
            "wup" => { validate_finite_parameter("WUP", value)?; self.params.p701 = value; self.mark_param_given(701); Ok(()) }
            "pup" => { validate_finite_parameter("PUP", value)?; self.params.p702 = value; self.mark_param_given(702); Ok(()) }
            "lup2" => { validate_finite_parameter("LUP2", value)?; self.params.p703 = value; self.mark_param_given(703); Ok(()) }
            "wup2" => { validate_finite_parameter("WUP2", value)?; self.params.p704 = value; self.mark_param_given(704); Ok(()) }
            "pup2" => { validate_finite_parameter("PUP2", value)?; self.params.p705 = value; self.mark_param_given(705); Ok(()) }
            "laigbinv" => { validate_finite_parameter("LAIGBINV", value)?; self.params.p706 = value; self.mark_param_given(706); Ok(()) }
            "waigbinv" => { validate_finite_parameter("WAIGBINV", value)?; self.params.p707 = value; self.mark_param_given(707); Ok(()) }
            "paigbinv" => { validate_finite_parameter("PAIGBINV", value)?; self.params.p708 = value; self.mark_param_given(708); Ok(()) }
            "lbigbinv" => { validate_finite_parameter("LBIGBINV", value)?; self.params.p709 = value; self.mark_param_given(709); Ok(()) }
            "wbigbinv" => { validate_finite_parameter("WBIGBINV", value)?; self.params.p710 = value; self.mark_param_given(710); Ok(()) }
            "pbigbinv" => { validate_finite_parameter("PBIGBINV", value)?; self.params.p711 = value; self.mark_param_given(711); Ok(()) }
            "lcigbinv" => { validate_finite_parameter("LCIGBINV", value)?; self.params.p712 = value; self.mark_param_given(712); Ok(()) }
            "wcigbinv" => { validate_finite_parameter("WCIGBINV", value)?; self.params.p713 = value; self.mark_param_given(713); Ok(()) }
            "pcigbinv" => { validate_finite_parameter("PCIGBINV", value)?; self.params.p714 = value; self.mark_param_given(714); Ok(()) }
            "leigbinv" => { validate_finite_parameter("LEIGBINV", value)?; self.params.p715 = value; self.mark_param_given(715); Ok(()) }
            "weigbinv" => { validate_finite_parameter("WEIGBINV", value)?; self.params.p716 = value; self.mark_param_given(716); Ok(()) }
            "peigbinv" => { validate_finite_parameter("PEIGBINV", value)?; self.params.p717 = value; self.mark_param_given(717); Ok(()) }
            "lnigbinv" => { validate_finite_parameter("LNIGBINV", value)?; self.params.p718 = value; self.mark_param_given(718); Ok(()) }
            "wnigbinv" => { validate_finite_parameter("WNIGBINV", value)?; self.params.p719 = value; self.mark_param_given(719); Ok(()) }
            "pnigbinv" => { validate_finite_parameter("PNIGBINV", value)?; self.params.p720 = value; self.mark_param_given(720); Ok(()) }
            "laigbacc" => { validate_finite_parameter("LAIGBACC", value)?; self.params.p721 = value; self.mark_param_given(721); Ok(()) }
            "waigbacc" => { validate_finite_parameter("WAIGBACC", value)?; self.params.p722 = value; self.mark_param_given(722); Ok(()) }
            "paigbacc" => { validate_finite_parameter("PAIGBACC", value)?; self.params.p723 = value; self.mark_param_given(723); Ok(()) }
            "lbigbacc" => { validate_finite_parameter("LBIGBACC", value)?; self.params.p724 = value; self.mark_param_given(724); Ok(()) }
            "wbigbacc" => { validate_finite_parameter("WBIGBACC", value)?; self.params.p725 = value; self.mark_param_given(725); Ok(()) }
            "pbigbacc" => { validate_finite_parameter("PBIGBACC", value)?; self.params.p726 = value; self.mark_param_given(726); Ok(()) }
            "lcigbacc" => { validate_finite_parameter("LCIGBACC", value)?; self.params.p727 = value; self.mark_param_given(727); Ok(()) }
            "wcigbacc" => { validate_finite_parameter("WCIGBACC", value)?; self.params.p728 = value; self.mark_param_given(728); Ok(()) }
            "pcigbacc" => { validate_finite_parameter("PCIGBACC", value)?; self.params.p729 = value; self.mark_param_given(729); Ok(()) }
            "lnigbacc" => { validate_finite_parameter("LNIGBACC", value)?; self.params.p730 = value; self.mark_param_given(730); Ok(()) }
            "wnigbacc" => { validate_finite_parameter("WNIGBACC", value)?; self.params.p731 = value; self.mark_param_given(731); Ok(()) }
            "pnigbacc" => { validate_finite_parameter("PNIGBACC", value)?; self.params.p732 = value; self.mark_param_given(732); Ok(()) }
            "lxrcrg1" => { validate_finite_parameter("LXRCRG1", value)?; self.params.p733 = value; self.mark_param_given(733); Ok(()) }
            "wxrcrg1" => { validate_finite_parameter("WXRCRG1", value)?; self.params.p734 = value; self.mark_param_given(734); Ok(()) }
            "pxrcrg1" => { validate_finite_parameter("PXRCRG1", value)?; self.params.p735 = value; self.mark_param_given(735); Ok(()) }
            "lxrcrg2" => { validate_finite_parameter("LXRCRG2", value)?; self.params.p736 = value; self.mark_param_given(736); Ok(()) }
            "wxrcrg2" => { validate_finite_parameter("WXRCRG2", value)?; self.params.p737 = value; self.mark_param_given(737); Ok(()) }
            "pxrcrg2" => { validate_finite_parameter("PXRCRG2", value)?; self.params.p738 = value; self.mark_param_given(738); Ok(()) }
            "lqmtcencv" => { validate_finite_parameter("LQMTCENCV", value)?; self.params.p739 = value; self.mark_param_given(739); Ok(()) }
            "wqmtcencv" => { validate_finite_parameter("WQMTCENCV", value)?; self.params.p740 = value; self.mark_param_given(740); Ok(()) }
            "pqmtcencv" => { validate_finite_parameter("PQMTCENCV", value)?; self.params.p741 = value; self.mark_param_given(741); Ok(()) }
            "letaqm" => { validate_finite_parameter("LETAQM", value)?; self.params.p742 = value; self.mark_param_given(742); Ok(()) }
            "wetaqm" => { validate_finite_parameter("WETAQM", value)?; self.params.p743 = value; self.mark_param_given(743); Ok(()) }
            "petaqm" => { validate_finite_parameter("PETAQM", value)?; self.params.p744 = value; self.mark_param_given(744); Ok(()) }
            "lqm0" => { validate_finite_parameter("LQM0", value)?; self.params.p745 = value; self.mark_param_given(745); Ok(()) }
            "wqm0" => { validate_finite_parameter("WQM0", value)?; self.params.p746 = value; self.mark_param_given(746); Ok(()) }
            "pqm0" => { validate_finite_parameter("PQM0", value)?; self.params.p747 = value; self.mark_param_given(747); Ok(()) }
            "lpqm" => { validate_finite_parameter("LPQM", value)?; self.params.p748 = value; self.mark_param_given(748); Ok(()) }
            "wpqm" => { validate_finite_parameter("WPQM", value)?; self.params.p749 = value; self.mark_param_given(749); Ok(()) }
            "ppqm" => { validate_finite_parameter("PPQM", value)?; self.params.p750 = value; self.mark_param_given(750); Ok(()) }
            "lnoia2" => { validate_finite_parameter("LNOIA2", value)?; self.params.p751 = value; self.mark_param_given(751); Ok(()) }
            "wnoia2" => { validate_finite_parameter("WNOIA2", value)?; self.params.p752 = value; self.mark_param_given(752); Ok(()) }
            "pnoia2" => { validate_finite_parameter("PNOIA2", value)?; self.params.p753 = value; self.mark_param_given(753); Ok(()) }
            "lmpower" => { validate_finite_parameter("LMPOWER", value)?; self.params.p754 = value; self.mark_param_given(754); Ok(()) }
            "wmpower" => { validate_finite_parameter("WMPOWER", value)?; self.params.p755 = value; self.mark_param_given(755); Ok(()) }
            "pmpower" => { validate_finite_parameter("PMPOWER", value)?; self.params.p756 = value; self.mark_param_given(756); Ok(()) }
            "lqsref" => { validate_finite_parameter("LQSREF", value)?; self.params.p757 = value; self.mark_param_given(757); Ok(()) }
            "wqsref" => { validate_finite_parameter("WQSREF", value)?; self.params.p758 = value; self.mark_param_given(758); Ok(()) }
            "pqsref" => { validate_finite_parameter("PQSREF", value)?; self.params.p759 = value; self.mark_param_given(759); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimimg'", name)),
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
