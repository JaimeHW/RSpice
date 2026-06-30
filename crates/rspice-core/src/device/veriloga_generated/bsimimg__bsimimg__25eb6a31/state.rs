#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;
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

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            let params = &mut *ptr;
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
    pub nodes: [usize; 9],
    pub branches: [usize; 5],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 760]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 8]>,
    pub(crate) ddt_state_previous: Box<[f64; 8]>,
    pub(crate) ddt_state_older: Box<[f64; 8]>,
    pub(crate) ddt_state_initialized: Box<[bool; 8]>,
    pub(crate) ddt_derivative_current: Box<[f64; 8]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 8]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: bool,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: bool,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: f64,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v18: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v20: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v23: f64,
    pub(crate) scalar_v24: f64,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: f64,
    pub(crate) scalar_v27: f64,
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
    pub(crate) scalar_v52: f64,
    pub(crate) scalar_v53: f64,
    pub(crate) scalar_v54: f64,
    pub(crate) scalar_v55: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: bool,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: bool,
    pub(crate) scalar_v64: bool,
    pub(crate) scalar_v65: f64,
    pub(crate) scalar_v66: f64,
    pub(crate) scalar_v67: f64,
    pub(crate) scalar_v68: f64,
    pub(crate) scalar_v69: f64,
    pub(crate) scalar_v70: f64,
    pub(crate) scalar_v71: f64,
    pub(crate) scalar_v72: f64,
    pub(crate) scalar_v73: f64,
    pub(crate) scalar_v74: f64,
    pub(crate) scalar_v75: bool,
    pub(crate) scalar_v76: f64,
    pub(crate) scalar_v77: f64,
    pub(crate) scalar_v79: bool,
    pub(crate) scalar_v80: bool,
    pub(crate) scalar_v81: bool,
    pub(crate) scalar_v82: bool,
    pub(crate) scalar_v84: f64,
    pub(crate) scalar_v85: f64,
    pub(crate) scalar_v86: f64,
    pub(crate) scalar_v87: f64,
    pub(crate) scalar_v88: f64,
    pub(crate) scalar_v89: f64,
    pub(crate) scalar_v90: f64,
    pub(crate) scalar_v91: f64,
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: bool,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: bool,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v104: bool,
    pub(crate) scalar_v105: bool,
    pub(crate) scalar_v106: bool,
    pub(crate) scalar_v107: bool,
    pub(crate) scalar_v108: bool,
    pub(crate) scalar_v109: bool,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v112: f64,
    pub(crate) scalar_v113: f64,
    pub(crate) scalar_v114: f64,
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scratch: Option<Box<GenericScratch<676, 9, 5>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<676, 9, 5>>>,
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
            scalar_v2: self.scalar_v2,
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
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v23: self.scalar_v23,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
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
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
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
            scalar_v73: self.scalar_v73,
            scalar_v74: self.scalar_v74,
            scalar_v75: self.scalar_v75,
            scalar_v76: self.scalar_v76,
            scalar_v77: self.scalar_v77,
            scalar_v79: self.scalar_v79,
            scalar_v80: self.scalar_v80,
            scalar_v81: self.scalar_v81,
            scalar_v82: self.scalar_v82,
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
            scalar_v101: self.scalar_v101,
            scalar_v102: self.scalar_v102,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
            scalar_v107: self.scalar_v107,
            scalar_v108: self.scalar_v108,
            scalar_v109: self.scalar_v109,
            scalar_v110: self.scalar_v110,
            scalar_v112: self.scalar_v112,
            scalar_v113: self.scalar_v113,
            scalar_v114: self.scalar_v114,
            scalar_v118: self.scalar_v118,
            scalar_v119: self.scalar_v119,
            scalar_v120: self.scalar_v120,
            scalar_v121: self.scalar_v121,
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
            scalar_v2: 0.0,
            scalar_v3: 0.0,
            scalar_v4: 0.0,
            scalar_v5: false,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v9: 0.0,
            scalar_v10: false,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: 0.0,
            scalar_v17: 0.0,
            scalar_v18: 0.0,
            scalar_v19: 0.0,
            scalar_v20: 0.0,
            scalar_v21: 0.0,
            scalar_v22: 0.0,
            scalar_v23: 0.0,
            scalar_v24: 0.0,
            scalar_v25: 0.0,
            scalar_v26: 0.0,
            scalar_v27: 0.0,
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
            scalar_v52: 0.0,
            scalar_v53: 0.0,
            scalar_v54: 0.0,
            scalar_v55: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: false,
            scalar_v62: 0.0,
            scalar_v63: false,
            scalar_v64: false,
            scalar_v65: 0.0,
            scalar_v66: 0.0,
            scalar_v67: 0.0,
            scalar_v68: 0.0,
            scalar_v69: 0.0,
            scalar_v70: 0.0,
            scalar_v71: 0.0,
            scalar_v72: 0.0,
            scalar_v73: 0.0,
            scalar_v74: 0.0,
            scalar_v75: false,
            scalar_v76: 0.0,
            scalar_v77: 0.0,
            scalar_v79: false,
            scalar_v80: false,
            scalar_v81: false,
            scalar_v82: false,
            scalar_v84: 0.0,
            scalar_v85: 0.0,
            scalar_v86: 0.0,
            scalar_v87: 0.0,
            scalar_v88: 0.0,
            scalar_v89: 0.0,
            scalar_v90: 0.0,
            scalar_v91: 0.0,
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: false,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: false,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v104: false,
            scalar_v105: false,
            scalar_v106: false,
            scalar_v107: false,
            scalar_v108: false,
            scalar_v109: false,
            scalar_v110: 0.0,
            scalar_v112: 0.0,
            scalar_v113: 0.0,
            scalar_v114: 0.0,
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scratch: Some(GenericScratch::new_box()),
            reactive_scratch: None,
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
            scalar_v2,
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
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
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
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
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
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
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
            scalar_v101,
            scalar_v102,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
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
            scalar_v2,
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
            scalar_v21,
            scalar_v22,
            scalar_v23,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
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
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
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
            scalar_v73,
            scalar_v74,
            scalar_v75,
            scalar_v76,
            scalar_v77,
            scalar_v79,
            scalar_v80,
            scalar_v81,
            scalar_v82,
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
            scalar_v101,
            scalar_v102,
            scalar_v104,
            scalar_v105,
            scalar_v106,
            scalar_v107,
            scalar_v108,
            scalar_v109,
            scalar_v110,
            scalar_v112,
            scalar_v113,
            scalar_v114,
            scalar_v118,
            scalar_v119,
            scalar_v120,
            scalar_v121,
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
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "delvtrand" => { validate_finite_parameter("DELVTRAND", value)?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "u0mult" => { validate_parameter("U0MULT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "welltype" => { validate_parameter("WELLTYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "rdsmod" => { validate_parameter("RDSMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "gidlmod" => { validate_parameter("GIDLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "igcmod" => { validate_parameter("IGCMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "igbmod" => { validate_parameter("IGBMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("SHMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "rgatemod" => { validate_parameter("RGATEMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "nqsmod" => { validate_parameter("NQSMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "nfmod" => { validate_parameter("NFMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "fnmod" => { validate_parameter("FNMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "xl" => { validate_finite_parameter("XL", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "xw" => { validate_finite_parameter("XW", value)?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "lint" => { validate_finite_parameter("LINT", value)?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "ll" => { validate_finite_parameter("LL", value)?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "lw" => { validate_finite_parameter("LW", value)?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "lwl" => { validate_finite_parameter("LWL", value)?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "lln" => { validate_finite_parameter("LLN", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "lwn" => { validate_finite_parameter("LWN", value)?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "wint" => { validate_finite_parameter("WINT", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "wl" => { validate_finite_parameter("WL", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "ww" => { validate_finite_parameter("WW", value)?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "wwl" => { validate_finite_parameter("WWL", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "wln" => { validate_finite_parameter("WLN", value)?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "wwn" => { validate_finite_parameter("WWN", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "dlc" => { validate_finite_parameter("DLC", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "llc" => { validate_finite_parameter("LLC", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "lwc" => { validate_finite_parameter("LWC", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "lwlc" => { validate_finite_parameter("LWLC", value)?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "dwc" => { validate_finite_parameter("DWC", value)?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "wlc" => { validate_finite_parameter("WLC", value)?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "wwc" => { validate_finite_parameter("WWC", value)?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "wwlc" => { validate_finite_parameter("WWLC", value)?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "eot1" => { validate_parameter("EOT1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "eot2" => { validate_parameter("EOT2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "eot1p" => { validate_parameter("EOT1P", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "dtox1" => { validate_finite_parameter("DTOX1", value)?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "tsi" => { validate_parameter("TSI", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "nbody" => { validate_parameter("NBODY", value, Some((1e18, "1e18")), false, Some((5e24, "5e24")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "nsd" => { validate_parameter("NSD", value, Some((2e25, "2e25")), false, Some((1e27, "1e27")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "nbg" => { validate_parameter("NBG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "easub" => { validate_parameter("EASUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "ni0sub" => { validate_parameter("NI0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "bg0sub" => { validate_parameter("BG0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "nc0sub" => { validate_parameter("NC0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "phig1" => { validate_parameter("PHIG1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "phig2" => { validate_parameter("PHIG2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "epsrsub" => { validate_parameter("EPSRSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "epsrox1" => { validate_parameter("EPSROX1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "ascl" => { validate_finite_parameter("ASCL", value)?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "bscl" => { validate_finite_parameter("BSCL", value)?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "cdsc" => { validate_finite_parameter("CDSC", value)?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_finite_parameter("CDSCD", value)?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "cbgcbg0" => { validate_finite_parameter("CBGCBG0", value)?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "cbgcbg0p" => { validate_finite_parameter("CBGCBG0P", value)?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "cbgcbg" => { validate_finite_parameter("CBGCBG", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cbgcbgp" => { validate_finite_parameter("CBGCBGP", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "cbgcbgd" => { validate_finite_parameter("CBGCBGD", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "dvt0" => { validate_finite_parameter("DVT0", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "dvt1" => { validate_finite_parameter("DVT1", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "phin" => { validate_finite_parameter("PHIN", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "eta1" => { validate_finite_parameter("ETA1", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "dsub" => { validate_finite_parameter("DSUB", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "dvtp0" => { validate_finite_parameter("DVTP0", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "dvtp1" => { validate_finite_parameter("DVTP1", value)?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "advtp0" => { validate_finite_parameter("ADVTP0", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "bdvtp0" => { validate_finite_parameter("BDVTP0", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "advtp1" => { validate_finite_parameter("ADVTP1", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "bdvtp1" => { validate_finite_parameter("BDVTP1", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "dvtp2" => { validate_finite_parameter("DVTP2", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "k1rsce" => { validate_finite_parameter("K1RSCE", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "lpe0" => { validate_finite_parameter("LPE0", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "dsc0" => { validate_finite_parameter("DSC0", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "dsc1" => { validate_finite_parameter("DSC1", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "k0" => { validate_finite_parameter("K0", value)?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "k01" => { validate_finite_parameter("K01", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "k0si" => { validate_finite_parameter("K0SI", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "k0si1" => { validate_finite_parameter("K0SI1", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "k0sisat" => { validate_finite_parameter("K0SISAT", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "k0sisat1" => { validate_finite_parameter("K0SISAT1", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "qmtcencv" => { validate_finite_parameter("QMTCENCV", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "etaqm" => { validate_finite_parameter("ETAQM", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "qm0" => { validate_parameter("QM0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "pqm" => { validate_finite_parameter("PQM", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "toxp" => { validate_parameter("TOXP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "vsat" => { validate_finite_parameter("VSAT", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "avsat" => { validate_finite_parameter("AVSAT", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "bvsat" => { validate_finite_parameter("BVSAT", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "vsat1" => { validate_finite_parameter("VSAT1", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "avsat1" => { validate_finite_parameter("AVSAT1", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "bvsat1" => { validate_finite_parameter("BVSAT1", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "vsatcv" => { validate_finite_parameter("VSATCV", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "avsatcv" => { validate_finite_parameter("AVSATCV", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "bvsatcv" => { validate_finite_parameter("BVSATCV", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "deltavsat" => { validate_finite_parameter("DELTAVSAT", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "ksativ" => { validate_finite_parameter("KSATIV", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "ksubiv" => { validate_finite_parameter("KSUBIV", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "ksativb" => { validate_finite_parameter("KSATIVB", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "mexp" => { validate_finite_parameter("MEXP", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "amexp" => { validate_finite_parameter("AMEXP", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "bmexp" => { validate_finite_parameter("BMEXP", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "ptwg" => { validate_finite_parameter("PTWG", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "aptwg" => { validate_finite_parameter("APTWG", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "bptwg" => { validate_finite_parameter("BPTWG", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "at" => { validate_finite_parameter("AT", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "atl" => { validate_finite_parameter("ATL", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "tmexp" => { validate_finite_parameter("TMEXP", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "ptwgt" => { validate_finite_parameter("PTWGT", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "ptwgb" => { validate_finite_parameter("PTWGB", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "ptwgb2" => { validate_finite_parameter("PTWGB2", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "aptwgb" => { validate_finite_parameter("APTWGB", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "bptwgb" => { validate_finite_parameter("BPTWGB", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "aptwgb2" => { validate_finite_parameter("APTWGB2", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "bptwgb2" => { validate_finite_parameter("BPTWGB2", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "vsatb" => { validate_finite_parameter("VSATB", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "atb" => { validate_finite_parameter("ATB", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "atbl" => { validate_finite_parameter("ATBL", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "avsatb" => { validate_finite_parameter("AVSATB", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "bvsatb" => { validate_finite_parameter("BVSATB", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "dvsatclamp" => { validate_parameter("DVSATCLAMP", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_finite_parameter("U0", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "etamob" => { validate_finite_parameter("ETAMOB", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "up" => { validate_finite_parameter("UP", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "lpa" => { validate_finite_parameter("LPA", value)?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_finite_parameter("UA", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "aua" => { validate_finite_parameter("AUA", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "bua" => { validate_finite_parameter("BUA", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "eu" => { validate_finite_parameter("EU", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "aeu" => { validate_finite_parameter("AEU", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "beu" => { validate_finite_parameter("BEU", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_finite_parameter("UC", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "auc" => { validate_finite_parameter("AUC", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "buc" => { validate_finite_parameter("BUC", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "ud" => { validate_finite_parameter("UD", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "aud" => { validate_finite_parameter("AUD", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "bud" => { validate_finite_parameter("BUD", value)?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "udb" => { validate_finite_parameter("UDB", value)?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "audb" => { validate_finite_parameter("AUDB", value)?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "budb" => { validate_finite_parameter("BUDB", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "dmobclamp" => { validate_parameter("DMOBCLAMP", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "ucs" => { validate_finite_parameter("UCS", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "ute" => { validate_finite_parameter("UTE", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "utl" => { validate_finite_parameter("UTL", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "ua1" => { validate_finite_parameter("UA1", value)?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "uc1" => { validate_finite_parameter("UC1", value)?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "ud1" => { validate_finite_parameter("UD1", value)?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "ucste" => { validate_finite_parameter("UCSTE", value)?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "chargewf" => { validate_parameter("CHARGEWF", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "eub" => { validate_finite_parameter("EUB", value)?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "aeub" => { validate_finite_parameter("AEUB", value)?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "beub" => { validate_finite_parameter("BEUB", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "u02" => { validate_finite_parameter("U02", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "ua2" => { validate_finite_parameter("UA2", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "aua2" => { validate_finite_parameter("AUA2", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "bua2" => { validate_finite_parameter("BUA2", value)?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "eu2" => { validate_finite_parameter("EU2", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "aeu2" => { validate_finite_parameter("AEU2", value)?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "beu2" => { validate_finite_parameter("BEU2", value)?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "uc2" => { validate_finite_parameter("UC2", value)?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "auc2" => { validate_finite_parameter("AUC2", value)?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "buc2" => { validate_finite_parameter("BUC2", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "ud2" => { validate_finite_parameter("UD2", value)?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "aud2" => { validate_finite_parameter("AUD2", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "bud2" => { validate_finite_parameter("BUD2", value)?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "udb2" => { validate_finite_parameter("UDB2", value)?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "audb2" => { validate_finite_parameter("AUDB2", value)?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "budb2" => { validate_finite_parameter("BUDB2", value)?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "ucs2" => { validate_finite_parameter("UCS2", value)?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "eub2" => { validate_finite_parameter("EUB2", value)?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "aeub2" => { validate_finite_parameter("AEUB2", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "beub2" => { validate_finite_parameter("BEUB2", value)?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "etamob2" => { validate_finite_parameter("ETAMOB2", value)?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "up2" => { validate_finite_parameter("UP2", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "lpa2" => { validate_finite_parameter("LPA2", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "chargewf2" => { validate_parameter("CHARGEWF2", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "rdswmin" => { validate_parameter("RDSWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "rdsw" => { validate_parameter("RDSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "ardsw" => { validate_finite_parameter("ARDSW", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "brdsw" => { validate_finite_parameter("BRDSW", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "rswmin" => { validate_parameter("RSWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "rsw" => { validate_parameter("RSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "arsw" => { validate_finite_parameter("ARSW", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "brsw" => { validate_finite_parameter("BRSW", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "rdwmin" => { validate_parameter("RDWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "rdw" => { validate_parameter("RDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "ardw" => { validate_finite_parameter("ARDW", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "brdw" => { validate_finite_parameter("BRDW", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "prwg" => { validate_finite_parameter("PRWG", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "prwb" => { validate_finite_parameter("PRWB", value)?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "wr" => { validate_finite_parameter("WR", value)?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "prt" => { validate_finite_parameter("PRT", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "pdibl1" => { validate_finite_parameter("PDIBL1", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "pdibl2" => { validate_finite_parameter("PDIBL2", value)?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "drout" => { validate_finite_parameter("DROUT", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "pvag" => { validate_finite_parameter("PVAG", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "pclm" => { validate_finite_parameter("PCLM", value)?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "apclm" => { validate_finite_parameter("APCLM", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "bpclm" => { validate_finite_parameter("BPCLM", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "pclmg" => { validate_finite_parameter("PCLMG", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "pclmcv" => { validate_finite_parameter("PCLMCV", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "rshd" => { validate_parameter("RSHD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "aigbinv" => { validate_finite_parameter("AIGBINV", value)?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "bigbinv" => { validate_finite_parameter("BIGBINV", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "cigbinv" => { validate_finite_parameter("CIGBINV", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "eigbinv" => { validate_finite_parameter("EIGBINV", value)?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "nigbinv" => { validate_finite_parameter("NIGBINV", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "aigbacc" => { validate_finite_parameter("AIGBACC", value)?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "bigbacc" => { validate_finite_parameter("BIGBACC", value)?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "cigbacc" => { validate_finite_parameter("CIGBACC", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "nigbacc" => { validate_finite_parameter("NIGBACC", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "aigc" => { validate_finite_parameter("AIGC", value)?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "bigc" => { validate_finite_parameter("BIGC", value)?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "cigc" => { validate_finite_parameter("CIGC", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "pigcd" => { validate_finite_parameter("PIGCD", value)?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "digc" => { validate_finite_parameter("DIGC", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "aigs" => { validate_finite_parameter("AIGS", value)?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "bigs" => { validate_finite_parameter("BIGS", value)?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "cigs" => { validate_finite_parameter("CIGS", value)?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "dlcigs" => { validate_parameter("DLCIGS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "dlcigd" => { validate_parameter("DLCIGD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "aigd" => { validate_finite_parameter("AIGD", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "bigd" => { validate_finite_parameter("BIGD", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "cigd" => { validate_finite_parameter("CIGD", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "toxref" => { validate_parameter("TOXREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "ntox" => { validate_finite_parameter("NTOX", value)?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "poxedge" => { validate_finite_parameter("POXEDGE", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "digs" => { validate_finite_parameter("DIGS", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "digd" => { validate_finite_parameter("DIGD", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "agidl" => { validate_finite_parameter("AGIDL", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "bgidl" => { validate_finite_parameter("BGIDL", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "egidl" => { validate_finite_parameter("EGIDL", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "pgidl" => { validate_finite_parameter("PGIDL", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "vbgidl" => { validate_finite_parameter("VBGIDL", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "vbegidl" => { validate_finite_parameter("VBEGIDL", value)?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "agisl" => { validate_finite_parameter("AGISL", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "bgisl" => { validate_finite_parameter("BGISL", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "egisl" => { validate_finite_parameter("EGISL", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "pgisl" => { validate_finite_parameter("PGISL", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "vbgisl" => { validate_finite_parameter("VBGISL", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "vbegisl" => { validate_finite_parameter("VBEGISL", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "alpha0" => { validate_finite_parameter("ALPHA0", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "alpha1" => { validate_finite_parameter("ALPHA1", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "beta0" => { validate_finite_parameter("BETA0", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "lovs" => { validate_finite_parameter("LOVS", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "lovd" => { validate_finite_parameter("LOVD", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "cfs" => { validate_parameter("CFS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "cfd" => { validate_parameter("CFD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "cgsl" => { validate_parameter("CGSL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "cgdl" => { validate_parameter("CGDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "ckappas" => { validate_parameter("CKAPPAS", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "ckappad" => { validate_parameter("CKAPPAD", value, Some((0.02, "0.02")), false, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "csdbgsw" => { validate_finite_parameter("CSDBGSW", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "pcovbs0" => { validate_finite_parameter("PCOVBS0", value)?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "pcovbs1" => { validate_finite_parameter("PCOVBS1", value)?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "pcovbd0" => { validate_finite_parameter("PCOVBD0", value)?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "pcovbd1" => { validate_finite_parameter("PCOVBD1", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "kbg0pw" => { validate_finite_parameter("KBG0PW", value)?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "kbg1pw" => { validate_finite_parameter("KBG1PW", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "kbg2pw" => { validate_finite_parameter("KBG2PW", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "dbgpw" => { validate_finite_parameter("DBGPW", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "bpfactorpw" => { validate_finite_parameter("BPFACTORPW", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "vknee1pw" => { validate_finite_parameter("VKNEE1PW", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "vknee2pw" => { validate_parameter("VKNEE2PW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "kbg0nw" => { validate_finite_parameter("KBG0NW", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "kbg1nw" => { validate_finite_parameter("KBG1NW", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "kbg2nw" => { validate_finite_parameter("KBG2NW", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "dbgnw" => { validate_finite_parameter("DBGNW", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "bpfactornw" => { validate_finite_parameter("BPFACTORNW", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "vknee1nw" => { validate_finite_parameter("VKNEE1NW", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "vknee2nw" => { validate_parameter("VKNEE2NW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.0, "0.0")), true, Some((2.0, "2.0")), false, &[])?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            "em" => { validate_parameter("EM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); Ok(()) }
            "noia" => { validate_parameter("NOIA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); Ok(()) }
            "noib" => { validate_parameter("NOIB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); Ok(()) }
            "noic" => { validate_parameter("NOIC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); Ok(()) }
            "noia2" => { validate_parameter("NOIA2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); Ok(()) }
            "smooth" => { validate_parameter("SMOOTH", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); Ok(()) }
            "mpower" => { validate_parameter("MPOWER", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); Ok(()) }
            "qsref" => { validate_parameter("QSREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); Ok(()) }
            "ntnoi" => { validate_parameter("NTNOI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); Ok(()) }
            "lintnoi" => { validate_finite_parameter("LINTNOI", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); Ok(()) }
            "tmaxc" => { validate_finite_parameter("TMAXC", value)?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); Ok(()) }
            "tbgasub" => { validate_finite_parameter("TBGASUB", value)?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); Ok(()) }
            "tbgbsub" => { validate_finite_parameter("TBGBSUB", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); Ok(()) }
            "kt1" => { validate_finite_parameter("KT1", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); Ok(()) }
            "kt1l" => { validate_finite_parameter("KT1L", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); Ok(()) }
            "kt2" => { validate_finite_parameter("KT2", value)?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); Ok(()) }
            "kt2l" => { validate_finite_parameter("KT2L", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); Ok(()) }
            "iit" => { validate_finite_parameter("IIT", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); Ok(()) }
            "tgidl" => { validate_finite_parameter("TGIDL", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); Ok(()) }
            "tgisl" => { validate_finite_parameter("TGISL", value)?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); Ok(()) }
            "igt" => { validate_finite_parameter("IGT", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); Ok(()) }
            "teta0" => { validate_finite_parameter("TETA0", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("RTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("CTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); Ok(()) }
            "wth0" => { validate_parameter("WTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); Ok(()) }
            "xrcrg1" => { validate_finite_parameter("XRCRG1", value)?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); Ok(()) }
            "xrcrg2" => { validate_finite_parameter("XRCRG2", value)?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); Ok(()) }
            "lrdsw" => { validate_finite_parameter("LRDSW", value)?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); Ok(()) }
            "wrdsw" => { validate_finite_parameter("WRDSW", value)?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); Ok(()) }
            "prdsw" => { validate_finite_parameter("PRDSW", value)?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); Ok(()) }
            "lrsw" => { validate_finite_parameter("LRSW", value)?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); Ok(()) }
            "wrsw" => { validate_finite_parameter("WRSW", value)?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); Ok(()) }
            "prsw" => { validate_finite_parameter("PRSW", value)?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); Ok(()) }
            "lrdw" => { validate_finite_parameter("LRDW", value)?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); Ok(()) }
            "wrdw" => { validate_finite_parameter("WRDW", value)?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); Ok(()) }
            "prdw" => { validate_finite_parameter("PRDW", value)?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); Ok(()) }
            "lprwg" => { validate_finite_parameter("LPRWG", value)?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); Ok(()) }
            "wprwg" => { validate_finite_parameter("WPRWG", value)?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); Ok(()) }
            "pprwg" => { validate_finite_parameter("PPRWG", value)?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); Ok(()) }
            "lprwb" => { validate_finite_parameter("LPRWB", value)?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); Ok(()) }
            "wprwb" => { validate_finite_parameter("WPRWB", value)?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); Ok(()) }
            "pprwb" => { validate_finite_parameter("PPRWB", value)?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); Ok(()) }
            "lwr" => { validate_finite_parameter("LWR", value)?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); Ok(()) }
            "wwr" => { validate_finite_parameter("WWR", value)?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); Ok(()) }
            "pwr" => { validate_finite_parameter("PWR", value)?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); Ok(()) }
            "lphig1" => { validate_finite_parameter("LPHIG1", value)?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); Ok(()) }
            "wphig1" => { validate_finite_parameter("WPHIG1", value)?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); Ok(()) }
            "pphig1" => { validate_finite_parameter("PPHIG1", value)?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); Ok(()) }
            "lphig2" => { validate_finite_parameter("LPHIG2", value)?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); Ok(()) }
            "wphig2" => { validate_finite_parameter("WPHIG2", value)?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); Ok(()) }
            "pphig2" => { validate_finite_parameter("PPHIG2", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); Ok(()) }
            "lnsd" => { validate_finite_parameter("LNSD", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); Ok(()) }
            "wnsd" => { validate_finite_parameter("WNSD", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); Ok(()) }
            "pnsd" => { validate_finite_parameter("PNSD", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); Ok(()) }
            "lnbody" => { validate_finite_parameter("LNBODY", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); Ok(()) }
            "wnbody" => { validate_finite_parameter("WNBODY", value)?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); Ok(()) }
            "pnbody" => { validate_finite_parameter("PNBODY", value)?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); Ok(()) }
            "lcit" => { validate_finite_parameter("LCIT", value)?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); Ok(()) }
            "wcit" => { validate_finite_parameter("WCIT", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); Ok(()) }
            "pcit" => { validate_finite_parameter("PCIT", value)?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); Ok(()) }
            "lcdsc" => { validate_finite_parameter("LCDSC", value)?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); Ok(()) }
            "wcdsc" => { validate_finite_parameter("WCDSC", value)?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); Ok(()) }
            "pcdsc" => { validate_finite_parameter("PCDSC", value)?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); Ok(()) }
            "lcdscd" => { validate_finite_parameter("LCDSCD", value)?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); Ok(()) }
            "wcdscd" => { validate_finite_parameter("WCDSCD", value)?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); Ok(()) }
            "pcdscd" => { validate_finite_parameter("PCDSCD", value)?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); Ok(()) }
            "lcbgcbg" => { validate_finite_parameter("LCBGCBG", value)?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); Ok(()) }
            "wcbgcbg" => { validate_finite_parameter("WCBGCBG", value)?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); Ok(()) }
            "pcbgcbg" => { validate_finite_parameter("PCBGCBG", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); Ok(()) }
            "lbpfactorpw" => { validate_finite_parameter("LBPFACTORPW", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); Ok(()) }
            "wbpfactorpw" => { validate_finite_parameter("WBPFACTORPW", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); Ok(()) }
            "pbpfactorpw" => { validate_finite_parameter("PBPFACTORPW", value)?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); Ok(()) }
            "lvknee1pw" => { validate_finite_parameter("LVKNEE1PW", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); Ok(()) }
            "wvknee1pw" => { validate_finite_parameter("WVKNEE1PW", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); Ok(()) }
            "pvknee1pw" => { validate_finite_parameter("PVKNEE1PW", value)?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); Ok(()) }
            "lvknee2pw" => { validate_finite_parameter("LVKNEE2PW", value)?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); Ok(()) }
            "wvknee2pw" => { validate_finite_parameter("WVKNEE2PW", value)?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); Ok(()) }
            "pvknee2pw" => { validate_finite_parameter("PVKNEE2PW", value)?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); Ok(()) }
            "ldbgpw" => { validate_finite_parameter("LDBGPW", value)?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); Ok(()) }
            "wdbgpw" => { validate_finite_parameter("WDBGPW", value)?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); Ok(()) }
            "pdbgpw" => { validate_finite_parameter("PDBGPW", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); Ok(()) }
            "lkbg0pw" => { validate_finite_parameter("LKBG0PW", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); Ok(()) }
            "wkbg0pw" => { validate_finite_parameter("WKBG0PW", value)?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); Ok(()) }
            "pkbg0pw" => { validate_finite_parameter("PKBG0PW", value)?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); Ok(()) }
            "lkbg1pw" => { validate_finite_parameter("LKBG1PW", value)?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); Ok(()) }
            "wkbg1pw" => { validate_finite_parameter("WKBG1PW", value)?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); Ok(()) }
            "pkbg1pw" => { validate_finite_parameter("PKBG1PW", value)?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); Ok(()) }
            "lkbg2pw" => { validate_finite_parameter("LKBG2PW", value)?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); Ok(()) }
            "wkbg2pw" => { validate_finite_parameter("WKBG2PW", value)?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); Ok(()) }
            "pkbg2pw" => { validate_finite_parameter("PKBG2PW", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); Ok(()) }
            "lbpfactornw" => { validate_finite_parameter("LBPFACTORNW", value)?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); Ok(()) }
            "wbpfactornw" => { validate_finite_parameter("WBPFACTORNW", value)?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); Ok(()) }
            "pbpfactornw" => { validate_finite_parameter("PBPFACTORNW", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); Ok(()) }
            "lvknee1nw" => { validate_finite_parameter("LVKNEE1NW", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); Ok(()) }
            "wvknee1nw" => { validate_finite_parameter("WVKNEE1NW", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); Ok(()) }
            "pvknee1nw" => { validate_finite_parameter("PVKNEE1NW", value)?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); Ok(()) }
            "lvknee2nw" => { validate_finite_parameter("LVKNEE2NW", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); Ok(()) }
            "wvknee2nw" => { validate_finite_parameter("WVKNEE2NW", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); Ok(()) }
            "pvknee2nw" => { validate_finite_parameter("PVKNEE2NW", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); Ok(()) }
            "ldbgnw" => { validate_finite_parameter("LDBGNW", value)?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); Ok(()) }
            "wdbgnw" => { validate_finite_parameter("WDBGNW", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); Ok(()) }
            "pdbgnw" => { validate_finite_parameter("PDBGNW", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); Ok(()) }
            "lkbg0nw" => { validate_finite_parameter("LKBG0NW", value)?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); Ok(()) }
            "wkbg0nw" => { validate_finite_parameter("WKBG0NW", value)?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); Ok(()) }
            "pkbg0nw" => { validate_finite_parameter("PKBG0NW", value)?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); Ok(()) }
            "lkbg1nw" => { validate_finite_parameter("LKBG1NW", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); Ok(()) }
            "wkbg1nw" => { validate_finite_parameter("WKBG1NW", value)?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); Ok(()) }
            "pkbg1nw" => { validate_finite_parameter("PKBG1NW", value)?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); Ok(()) }
            "lkbg2nw" => { validate_finite_parameter("LKBG2NW", value)?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); Ok(()) }
            "wkbg2nw" => { validate_finite_parameter("WKBG2NW", value)?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); Ok(()) }
            "pkbg2nw" => { validate_finite_parameter("PKBG2NW", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); Ok(()) }
            "ldvt0" => { validate_finite_parameter("LDVT0", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); Ok(()) }
            "wdvt0" => { validate_finite_parameter("WDVT0", value)?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); Ok(()) }
            "pdvt0" => { validate_finite_parameter("PDVT0", value)?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); Ok(()) }
            "ldvt1" => { validate_finite_parameter("LDVT1", value)?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); Ok(()) }
            "wdvt1" => { validate_finite_parameter("WDVT1", value)?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); Ok(()) }
            "pdvt1" => { validate_finite_parameter("PDVT1", value)?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); Ok(()) }
            "lphin" => { validate_finite_parameter("LPHIN", value)?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); Ok(()) }
            "wphin" => { validate_finite_parameter("WPHIN", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); Ok(()) }
            "pphin" => { validate_finite_parameter("PPHIN", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); Ok(()) }
            "leta0" => { validate_finite_parameter("LETA0", value)?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); Ok(()) }
            "weta0" => { validate_finite_parameter("WETA0", value)?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); Ok(()) }
            "peta0" => { validate_finite_parameter("PETA0", value)?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); Ok(()) }
            "leta1" => { validate_finite_parameter("LETA1", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); Ok(()) }
            "weta1" => { validate_finite_parameter("WETA1", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); Ok(()) }
            "peta1" => { validate_finite_parameter("PETA1", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); Ok(()) }
            "letab" => { validate_finite_parameter("LETAB", value)?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); Ok(()) }
            "wetab" => { validate_finite_parameter("WETAB", value)?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); Ok(()) }
            "petab" => { validate_finite_parameter("PETAB", value)?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); Ok(()) }
            "ldsub" => { validate_finite_parameter("LDSUB", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); Ok(()) }
            "wdsub" => { validate_finite_parameter("WDSUB", value)?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); Ok(()) }
            "pdsub" => { validate_finite_parameter("PDSUB", value)?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); Ok(()) }
            "lk1rsce" => { validate_finite_parameter("LK1RSCE", value)?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); Ok(()) }
            "wk1rsce" => { validate_finite_parameter("WK1RSCE", value)?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); Ok(()) }
            "pk1rsce" => { validate_finite_parameter("PK1RSCE", value)?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); Ok(()) }
            "llpe0" => { validate_finite_parameter("LLPE0", value)?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); Ok(()) }
            "wlpe0" => { validate_finite_parameter("WLPE0", value)?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); Ok(()) }
            "plpe0" => { validate_finite_parameter("PLPE0", value)?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); Ok(()) }
            "ldsc0" => { validate_finite_parameter("LDSC0", value)?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); Ok(()) }
            "wdsc0" => { validate_finite_parameter("WDSC0", value)?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); Ok(()) }
            "pdsc0" => { validate_finite_parameter("PDSC0", value)?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); Ok(()) }
            "ldsc1" => { validate_finite_parameter("LDSC1", value)?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); Ok(()) }
            "wdsc1" => { validate_finite_parameter("WDSC1", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); Ok(()) }
            "pdsc1" => { validate_finite_parameter("PDSC1", value)?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); Ok(()) }
            "lascl" => { validate_finite_parameter("LASCL", value)?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); Ok(()) }
            "wascl" => { validate_finite_parameter("WASCL", value)?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); Ok(()) }
            "pascl" => { validate_finite_parameter("PASCL", value)?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); Ok(()) }
            "lbscl" => { validate_finite_parameter("LBSCL", value)?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); Ok(()) }
            "wbscl" => { validate_finite_parameter("WBSCL", value)?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); Ok(()) }
            "pbscl" => { validate_finite_parameter("PBSCL", value)?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); Ok(()) }
            "lk0" => { validate_finite_parameter("LK0", value)?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); Ok(()) }
            "wk0" => { validate_finite_parameter("WK0", value)?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); Ok(()) }
            "pk0" => { validate_finite_parameter("PK0", value)?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); Ok(()) }
            "lk01" => { validate_finite_parameter("LK01", value)?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); Ok(()) }
            "wk01" => { validate_finite_parameter("WK01", value)?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); Ok(()) }
            "pk01" => { validate_finite_parameter("PK01", value)?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); Ok(()) }
            "lk0si" => { validate_finite_parameter("LK0SI", value)?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); Ok(()) }
            "wk0si" => { validate_finite_parameter("WK0SI", value)?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); Ok(()) }
            "pk0si" => { validate_finite_parameter("PK0SI", value)?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); Ok(()) }
            "lk0si1" => { validate_finite_parameter("LK0SI1", value)?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); Ok(()) }
            "wk0si1" => { validate_finite_parameter("WK0SI1", value)?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); Ok(()) }
            "pk0si1" => { validate_finite_parameter("PK0SI1", value)?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); Ok(()) }
            "lk0sisat" => { validate_finite_parameter("LK0SISAT", value)?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); Ok(()) }
            "nk0sisat" => { validate_finite_parameter("NK0SISAT", value)?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); Ok(()) }
            "pk0sisat" => { validate_finite_parameter("PK0SISAT", value)?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); Ok(()) }
            "lk0sisat1" => { validate_finite_parameter("LK0SISAT1", value)?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); Ok(()) }
            "nk0sisat1" => { validate_finite_parameter("NK0SISAT1", value)?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); Ok(()) }
            "pk0sisat1" => { validate_finite_parameter("PK0SISAT1", value)?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); Ok(()) }
            "lmexp" => { validate_finite_parameter("LMEXP", value)?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); Ok(()) }
            "wmexp" => { validate_finite_parameter("WMEXP", value)?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); Ok(()) }
            "pmexp" => { validate_finite_parameter("PMEXP", value)?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); Ok(()) }
            "lptwg" => { validate_finite_parameter("LPTWG", value)?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); Ok(()) }
            "wptwg" => { validate_finite_parameter("WPTWG", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); Ok(()) }
            "pptwg" => { validate_finite_parameter("PPTWG", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); Ok(()) }
            "lptwgb" => { validate_finite_parameter("LPTWGB", value)?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); Ok(()) }
            "wptwgb" => { validate_finite_parameter("WPTWGB", value)?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); Ok(()) }
            "pptwgb" => { validate_finite_parameter("PPTWGB", value)?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); Ok(()) }
            "lptwgb2" => { validate_finite_parameter("LPTWGB2", value)?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); Ok(()) }
            "wptwgb2" => { validate_finite_parameter("WPTWGB2", value)?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); Ok(()) }
            "pptwgb2" => { validate_finite_parameter("PPTWGB2", value)?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); Ok(()) }
            "lptwgt" => { validate_finite_parameter("LPTWGT", value)?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); Ok(()) }
            "wptwgt" => { validate_finite_parameter("WPTWGT", value)?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); Ok(()) }
            "pptwgt" => { validate_finite_parameter("PPTWGT", value)?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); Ok(()) }
            "lu0" => { validate_finite_parameter("LU0", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); Ok(()) }
            "wu0" => { validate_finite_parameter("WU0", value)?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); Ok(()) }
            "pu0" => { validate_finite_parameter("PU0", value)?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); Ok(()) }
            "lua" => { validate_finite_parameter("LUA", value)?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); Ok(()) }
            "wua" => { validate_finite_parameter("WUA", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); Ok(()) }
            "pua" => { validate_finite_parameter("PUA", value)?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); Ok(()) }
            "luc" => { validate_finite_parameter("LUC", value)?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); Ok(()) }
            "wuc" => { validate_finite_parameter("WUC", value)?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); Ok(()) }
            "puc" => { validate_finite_parameter("PUC", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); Ok(()) }
            "lud" => { validate_finite_parameter("LUD", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); Ok(()) }
            "wud" => { validate_finite_parameter("WUD", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); Ok(()) }
            "pud" => { validate_finite_parameter("PUD", value)?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); Ok(()) }
            "lucs" => { validate_finite_parameter("LUCS", value)?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); Ok(()) }
            "wucs" => { validate_finite_parameter("WUCS", value)?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); Ok(()) }
            "pucs" => { validate_finite_parameter("PUCS", value)?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); Ok(()) }
            "leu" => { validate_finite_parameter("LEU", value)?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); Ok(()) }
            "weu" => { validate_finite_parameter("WEU", value)?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); Ok(()) }
            "peu" => { validate_finite_parameter("PEU", value)?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); Ok(()) }
            "leub" => { validate_finite_parameter("LEUB", value)?; self.params.p493 = value; self.mark_param_given(493); self.recompute_instance_static(); Ok(()) }
            "weub" => { validate_finite_parameter("WEUB", value)?; self.params.p494 = value; self.mark_param_given(494); self.recompute_instance_static(); Ok(()) }
            "peub" => { validate_finite_parameter("PEUB", value)?; self.params.p495 = value; self.mark_param_given(495); self.recompute_instance_static(); Ok(()) }
            "lutl" => { validate_finite_parameter("LUTL", value)?; self.params.p496 = value; self.mark_param_given(496); self.recompute_instance_static(); Ok(()) }
            "wutl" => { validate_finite_parameter("WUTL", value)?; self.params.p497 = value; self.mark_param_given(497); self.recompute_instance_static(); Ok(()) }
            "putl" => { validate_finite_parameter("PUTL", value)?; self.params.p498 = value; self.mark_param_given(498); self.recompute_instance_static(); Ok(()) }
            "lute" => { validate_finite_parameter("LUTE", value)?; self.params.p499 = value; self.mark_param_given(499); self.recompute_instance_static(); Ok(()) }
            "wute" => { validate_finite_parameter("WUTE", value)?; self.params.p500 = value; self.mark_param_given(500); self.recompute_instance_static(); Ok(()) }
            "pute" => { validate_finite_parameter("PUTE", value)?; self.params.p501 = value; self.mark_param_given(501); self.recompute_instance_static(); Ok(()) }
            "lua1" => { validate_finite_parameter("LUA1", value)?; self.params.p502 = value; self.mark_param_given(502); self.recompute_instance_static(); Ok(()) }
            "wua1" => { validate_finite_parameter("WUA1", value)?; self.params.p503 = value; self.mark_param_given(503); self.recompute_instance_static(); Ok(()) }
            "pua1" => { validate_finite_parameter("PUA1", value)?; self.params.p504 = value; self.mark_param_given(504); self.recompute_instance_static(); Ok(()) }
            "lud1" => { validate_finite_parameter("LUD1", value)?; self.params.p505 = value; self.mark_param_given(505); self.recompute_instance_static(); Ok(()) }
            "wud1" => { validate_finite_parameter("WUD1", value)?; self.params.p506 = value; self.mark_param_given(506); self.recompute_instance_static(); Ok(()) }
            "pud1" => { validate_finite_parameter("PUD1", value)?; self.params.p507 = value; self.mark_param_given(507); self.recompute_instance_static(); Ok(()) }
            "lucste" => { validate_finite_parameter("LUCSTE", value)?; self.params.p508 = value; self.mark_param_given(508); self.recompute_instance_static(); Ok(()) }
            "wucste" => { validate_finite_parameter("WUCSTE", value)?; self.params.p509 = value; self.mark_param_given(509); self.recompute_instance_static(); Ok(()) }
            "pucste" => { validate_finite_parameter("PUCSTE", value)?; self.params.p510 = value; self.mark_param_given(510); self.recompute_instance_static(); Ok(()) }
            "letamob" => { validate_finite_parameter("LETAMOB", value)?; self.params.p511 = value; self.mark_param_given(511); self.recompute_instance_static(); Ok(()) }
            "wetamob" => { validate_finite_parameter("WETAMOB", value)?; self.params.p512 = value; self.mark_param_given(512); self.recompute_instance_static(); Ok(()) }
            "petamob" => { validate_finite_parameter("PETAMOB", value)?; self.params.p513 = value; self.mark_param_given(513); self.recompute_instance_static(); Ok(()) }
            "lu02" => { validate_finite_parameter("LU02", value)?; self.params.p514 = value; self.mark_param_given(514); self.recompute_instance_static(); Ok(()) }
            "wu02" => { validate_finite_parameter("WU02", value)?; self.params.p515 = value; self.mark_param_given(515); self.recompute_instance_static(); Ok(()) }
            "pu02" => { validate_finite_parameter("PU02", value)?; self.params.p516 = value; self.mark_param_given(516); self.recompute_instance_static(); Ok(()) }
            "lua2" => { validate_finite_parameter("LUA2", value)?; self.params.p517 = value; self.mark_param_given(517); self.recompute_instance_static(); Ok(()) }
            "wua2" => { validate_finite_parameter("WUA2", value)?; self.params.p518 = value; self.mark_param_given(518); self.recompute_instance_static(); Ok(()) }
            "pua2" => { validate_finite_parameter("PUA2", value)?; self.params.p519 = value; self.mark_param_given(519); self.recompute_instance_static(); Ok(()) }
            "luc2" => { validate_finite_parameter("LUC2", value)?; self.params.p520 = value; self.mark_param_given(520); self.recompute_instance_static(); Ok(()) }
            "wuc2" => { validate_finite_parameter("WUC2", value)?; self.params.p521 = value; self.mark_param_given(521); self.recompute_instance_static(); Ok(()) }
            "puc2" => { validate_finite_parameter("PUC2", value)?; self.params.p522 = value; self.mark_param_given(522); self.recompute_instance_static(); Ok(()) }
            "lud2" => { validate_finite_parameter("LUD2", value)?; self.params.p523 = value; self.mark_param_given(523); self.recompute_instance_static(); Ok(()) }
            "wud2" => { validate_finite_parameter("WUD2", value)?; self.params.p524 = value; self.mark_param_given(524); self.recompute_instance_static(); Ok(()) }
            "pud2" => { validate_finite_parameter("PUD2", value)?; self.params.p525 = value; self.mark_param_given(525); self.recompute_instance_static(); Ok(()) }
            "lucs2" => { validate_finite_parameter("LUCS2", value)?; self.params.p526 = value; self.mark_param_given(526); self.recompute_instance_static(); Ok(()) }
            "wucs2" => { validate_finite_parameter("WUCS2", value)?; self.params.p527 = value; self.mark_param_given(527); self.recompute_instance_static(); Ok(()) }
            "pucs2" => { validate_finite_parameter("PUCS2", value)?; self.params.p528 = value; self.mark_param_given(528); self.recompute_instance_static(); Ok(()) }
            "leu2" => { validate_finite_parameter("LEU2", value)?; self.params.p529 = value; self.mark_param_given(529); self.recompute_instance_static(); Ok(()) }
            "weu2" => { validate_finite_parameter("WEU2", value)?; self.params.p530 = value; self.mark_param_given(530); self.recompute_instance_static(); Ok(()) }
            "peu2" => { validate_finite_parameter("PEU2", value)?; self.params.p531 = value; self.mark_param_given(531); self.recompute_instance_static(); Ok(()) }
            "leub2" => { validate_finite_parameter("LEUB2", value)?; self.params.p532 = value; self.mark_param_given(532); self.recompute_instance_static(); Ok(()) }
            "weub2" => { validate_finite_parameter("WEUB2", value)?; self.params.p533 = value; self.mark_param_given(533); self.recompute_instance_static(); Ok(()) }
            "peub2" => { validate_finite_parameter("PEUB2", value)?; self.params.p534 = value; self.mark_param_given(534); self.recompute_instance_static(); Ok(()) }
            "letamob2" => { validate_finite_parameter("LETAMOB2", value)?; self.params.p535 = value; self.mark_param_given(535); self.recompute_instance_static(); Ok(()) }
            "wetamob2" => { validate_finite_parameter("WETAMOB2", value)?; self.params.p536 = value; self.mark_param_given(536); self.recompute_instance_static(); Ok(()) }
            "petamob2" => { validate_finite_parameter("PETAMOB2", value)?; self.params.p537 = value; self.mark_param_given(537); self.recompute_instance_static(); Ok(()) }
            "lat" => { validate_finite_parameter("LAT", value)?; self.params.p538 = value; self.mark_param_given(538); self.recompute_instance_static(); Ok(()) }
            "wat" => { validate_finite_parameter("WAT", value)?; self.params.p539 = value; self.mark_param_given(539); self.recompute_instance_static(); Ok(()) }
            "pat" => { validate_finite_parameter("PAT", value)?; self.params.p540 = value; self.mark_param_given(540); self.recompute_instance_static(); Ok(()) }
            "latb" => { validate_finite_parameter("LATB", value)?; self.params.p541 = value; self.mark_param_given(541); self.recompute_instance_static(); Ok(()) }
            "watb" => { validate_finite_parameter("WATB", value)?; self.params.p542 = value; self.mark_param_given(542); self.recompute_instance_static(); Ok(()) }
            "patb" => { validate_finite_parameter("PATB", value)?; self.params.p543 = value; self.mark_param_given(543); self.recompute_instance_static(); Ok(()) }
            "lprt" => { validate_finite_parameter("LPRT", value)?; self.params.p544 = value; self.mark_param_given(544); self.recompute_instance_static(); Ok(()) }
            "wprt" => { validate_finite_parameter("WPRT", value)?; self.params.p545 = value; self.mark_param_given(545); self.recompute_instance_static(); Ok(()) }
            "pprt" => { validate_finite_parameter("PPRT", value)?; self.params.p546 = value; self.mark_param_given(546); self.recompute_instance_static(); Ok(()) }
            "liit" => { validate_finite_parameter("LIIT", value)?; self.params.p547 = value; self.mark_param_given(547); self.recompute_instance_static(); Ok(()) }
            "wiit" => { validate_finite_parameter("WIIT", value)?; self.params.p548 = value; self.mark_param_given(548); self.recompute_instance_static(); Ok(()) }
            "piit" => { validate_finite_parameter("PIIT", value)?; self.params.p549 = value; self.mark_param_given(549); self.recompute_instance_static(); Ok(()) }
            "ltgidl" => { validate_finite_parameter("LTGIDL", value)?; self.params.p550 = value; self.mark_param_given(550); self.recompute_instance_static(); Ok(()) }
            "wtgidl" => { validate_finite_parameter("WTGIDL", value)?; self.params.p551 = value; self.mark_param_given(551); self.recompute_instance_static(); Ok(()) }
            "ptgidl" => { validate_finite_parameter("PTGIDL", value)?; self.params.p552 = value; self.mark_param_given(552); self.recompute_instance_static(); Ok(()) }
            "ltgisl" => { validate_finite_parameter("LTGISL", value)?; self.params.p553 = value; self.mark_param_given(553); self.recompute_instance_static(); Ok(()) }
            "wtgisl" => { validate_finite_parameter("WTGISL", value)?; self.params.p554 = value; self.mark_param_given(554); self.recompute_instance_static(); Ok(()) }
            "ptgisl" => { validate_finite_parameter("PTGISL", value)?; self.params.p555 = value; self.mark_param_given(555); self.recompute_instance_static(); Ok(()) }
            "ligt" => { validate_finite_parameter("LIGT", value)?; self.params.p556 = value; self.mark_param_given(556); self.recompute_instance_static(); Ok(()) }
            "wigt" => { validate_finite_parameter("WIGT", value)?; self.params.p557 = value; self.mark_param_given(557); self.recompute_instance_static(); Ok(()) }
            "pigt" => { validate_finite_parameter("PIGT", value)?; self.params.p558 = value; self.mark_param_given(558); self.recompute_instance_static(); Ok(()) }
            "lpclm" => { validate_finite_parameter("LPCLM", value)?; self.params.p559 = value; self.mark_param_given(559); self.recompute_instance_static(); Ok(()) }
            "wpclm" => { validate_finite_parameter("WPCLM", value)?; self.params.p560 = value; self.mark_param_given(560); self.recompute_instance_static(); Ok(()) }
            "ppclm" => { validate_finite_parameter("PPCLM", value)?; self.params.p561 = value; self.mark_param_given(561); self.recompute_instance_static(); Ok(()) }
            "lpclmcv" => { validate_finite_parameter("LPCLMCV", value)?; self.params.p562 = value; self.mark_param_given(562); self.recompute_instance_static(); Ok(()) }
            "wpclmcv" => { validate_finite_parameter("WPCLMCV", value)?; self.params.p563 = value; self.mark_param_given(563); self.recompute_instance_static(); Ok(()) }
            "ppclmcv" => { validate_finite_parameter("PPCLMCV", value)?; self.params.p564 = value; self.mark_param_given(564); self.recompute_instance_static(); Ok(()) }
            "ldrout" => { validate_finite_parameter("LDROUT", value)?; self.params.p565 = value; self.mark_param_given(565); self.recompute_instance_static(); Ok(()) }
            "wdrout" => { validate_finite_parameter("WDROUT", value)?; self.params.p566 = value; self.mark_param_given(566); self.recompute_instance_static(); Ok(()) }
            "pdrout" => { validate_finite_parameter("PDROUT", value)?; self.params.p567 = value; self.mark_param_given(567); self.recompute_instance_static(); Ok(()) }
            "lpdibl1" => { validate_finite_parameter("LPDIBL1", value)?; self.params.p568 = value; self.mark_param_given(568); self.recompute_instance_static(); Ok(()) }
            "wpdibl1" => { validate_finite_parameter("WPDIBL1", value)?; self.params.p569 = value; self.mark_param_given(569); self.recompute_instance_static(); Ok(()) }
            "ppdibl1" => { validate_finite_parameter("PPDIBL1", value)?; self.params.p570 = value; self.mark_param_given(570); self.recompute_instance_static(); Ok(()) }
            "lpdibl2" => { validate_finite_parameter("LPDIBL2", value)?; self.params.p571 = value; self.mark_param_given(571); self.recompute_instance_static(); Ok(()) }
            "wpdibl2" => { validate_finite_parameter("WPDIBL2", value)?; self.params.p572 = value; self.mark_param_given(572); self.recompute_instance_static(); Ok(()) }
            "ppdibl2" => { validate_finite_parameter("PPDIBL2", value)?; self.params.p573 = value; self.mark_param_given(573); self.recompute_instance_static(); Ok(()) }
            "lpvag" => { validate_finite_parameter("LPVAG", value)?; self.params.p574 = value; self.mark_param_given(574); self.recompute_instance_static(); Ok(()) }
            "wpvag" => { validate_finite_parameter("WPVAG", value)?; self.params.p575 = value; self.mark_param_given(575); self.recompute_instance_static(); Ok(()) }
            "ppvag" => { validate_finite_parameter("PPVAG", value)?; self.params.p576 = value; self.mark_param_given(576); self.recompute_instance_static(); Ok(()) }
            "lalpha0" => { validate_finite_parameter("LALPHA0", value)?; self.params.p577 = value; self.mark_param_given(577); self.recompute_instance_static(); Ok(()) }
            "walpha0" => { validate_finite_parameter("WALPHA0", value)?; self.params.p578 = value; self.mark_param_given(578); self.recompute_instance_static(); Ok(()) }
            "palpha0" => { validate_finite_parameter("PALPHA0", value)?; self.params.p579 = value; self.mark_param_given(579); self.recompute_instance_static(); Ok(()) }
            "lalpha1" => { validate_finite_parameter("LALPHA1", value)?; self.params.p580 = value; self.mark_param_given(580); self.recompute_instance_static(); Ok(()) }
            "walpha1" => { validate_finite_parameter("WALPHA1", value)?; self.params.p581 = value; self.mark_param_given(581); self.recompute_instance_static(); Ok(()) }
            "palpha1" => { validate_finite_parameter("PALPHA1", value)?; self.params.p582 = value; self.mark_param_given(582); self.recompute_instance_static(); Ok(()) }
            "lbeta0" => { validate_finite_parameter("LBETA0", value)?; self.params.p583 = value; self.mark_param_given(583); self.recompute_instance_static(); Ok(()) }
            "wbeta0" => { validate_finite_parameter("WBETA0", value)?; self.params.p584 = value; self.mark_param_given(584); self.recompute_instance_static(); Ok(()) }
            "pbeta0" => { validate_finite_parameter("PBETA0", value)?; self.params.p585 = value; self.mark_param_given(585); self.recompute_instance_static(); Ok(()) }
            "laigc" => { validate_finite_parameter("LAIGC", value)?; self.params.p586 = value; self.mark_param_given(586); self.recompute_instance_static(); Ok(()) }
            "waigc" => { validate_finite_parameter("WAIGC", value)?; self.params.p587 = value; self.mark_param_given(587); self.recompute_instance_static(); Ok(()) }
            "paigc" => { validate_finite_parameter("PAIGC", value)?; self.params.p588 = value; self.mark_param_given(588); self.recompute_instance_static(); Ok(()) }
            "lbigc" => { validate_finite_parameter("LBIGC", value)?; self.params.p589 = value; self.mark_param_given(589); self.recompute_instance_static(); Ok(()) }
            "wbigc" => { validate_finite_parameter("WBIGC", value)?; self.params.p590 = value; self.mark_param_given(590); self.recompute_instance_static(); Ok(()) }
            "pbigc" => { validate_finite_parameter("PBIGC", value)?; self.params.p591 = value; self.mark_param_given(591); self.recompute_instance_static(); Ok(()) }
            "lcigc" => { validate_finite_parameter("LCIGC", value)?; self.params.p592 = value; self.mark_param_given(592); self.recompute_instance_static(); Ok(()) }
            "wcigc" => { validate_finite_parameter("WCIGC", value)?; self.params.p593 = value; self.mark_param_given(593); self.recompute_instance_static(); Ok(()) }
            "pcigc" => { validate_finite_parameter("PCIGC", value)?; self.params.p594 = value; self.mark_param_given(594); self.recompute_instance_static(); Ok(()) }
            "ldigc" => { validate_finite_parameter("LDIGC", value)?; self.params.p595 = value; self.mark_param_given(595); self.recompute_instance_static(); Ok(()) }
            "wdigc" => { validate_finite_parameter("WDIGC", value)?; self.params.p596 = value; self.mark_param_given(596); self.recompute_instance_static(); Ok(()) }
            "pdigc" => { validate_finite_parameter("PDIGC", value)?; self.params.p597 = value; self.mark_param_given(597); self.recompute_instance_static(); Ok(()) }
            "lpigcd" => { validate_finite_parameter("LPIGCD", value)?; self.params.p598 = value; self.mark_param_given(598); self.recompute_instance_static(); Ok(()) }
            "wpigcd" => { validate_finite_parameter("WPIGCD", value)?; self.params.p599 = value; self.mark_param_given(599); self.recompute_instance_static(); Ok(()) }
            "ppigcd" => { validate_finite_parameter("PPIGCD", value)?; self.params.p600 = value; self.mark_param_given(600); self.recompute_instance_static(); Ok(()) }
            "lagidl" => { validate_finite_parameter("LAGIDL", value)?; self.params.p601 = value; self.mark_param_given(601); self.recompute_instance_static(); Ok(()) }
            "wagidl" => { validate_finite_parameter("WAGIDL", value)?; self.params.p602 = value; self.mark_param_given(602); self.recompute_instance_static(); Ok(()) }
            "pagidl" => { validate_finite_parameter("PAGIDL", value)?; self.params.p603 = value; self.mark_param_given(603); self.recompute_instance_static(); Ok(()) }
            "lbgidl" => { validate_finite_parameter("LBGIDL", value)?; self.params.p604 = value; self.mark_param_given(604); self.recompute_instance_static(); Ok(()) }
            "wbgidl" => { validate_finite_parameter("WBGIDL", value)?; self.params.p605 = value; self.mark_param_given(605); self.recompute_instance_static(); Ok(()) }
            "pbgidl" => { validate_finite_parameter("PBGIDL", value)?; self.params.p606 = value; self.mark_param_given(606); self.recompute_instance_static(); Ok(()) }
            "legidl" => { validate_finite_parameter("LEGIDL", value)?; self.params.p607 = value; self.mark_param_given(607); self.recompute_instance_static(); Ok(()) }
            "wegidl" => { validate_finite_parameter("WEGIDL", value)?; self.params.p608 = value; self.mark_param_given(608); self.recompute_instance_static(); Ok(()) }
            "pegidl" => { validate_finite_parameter("PEGIDL", value)?; self.params.p609 = value; self.mark_param_given(609); self.recompute_instance_static(); Ok(()) }
            "lpgidl" => { validate_finite_parameter("LPGIDL", value)?; self.params.p610 = value; self.mark_param_given(610); self.recompute_instance_static(); Ok(()) }
            "wpgidl" => { validate_finite_parameter("WPGIDL", value)?; self.params.p611 = value; self.mark_param_given(611); self.recompute_instance_static(); Ok(()) }
            "ppgidl" => { validate_finite_parameter("PPGIDL", value)?; self.params.p612 = value; self.mark_param_given(612); self.recompute_instance_static(); Ok(()) }
            "lvbgidl" => { validate_finite_parameter("LVBGIDL", value)?; self.params.p613 = value; self.mark_param_given(613); self.recompute_instance_static(); Ok(()) }
            "wvbgidl" => { validate_finite_parameter("WVBGIDL", value)?; self.params.p614 = value; self.mark_param_given(614); self.recompute_instance_static(); Ok(()) }
            "pvbgidl" => { validate_finite_parameter("PVBGIDL", value)?; self.params.p615 = value; self.mark_param_given(615); self.recompute_instance_static(); Ok(()) }
            "lvbegidl" => { validate_finite_parameter("LVBEGIDL", value)?; self.params.p616 = value; self.mark_param_given(616); self.recompute_instance_static(); Ok(()) }
            "wvbegidl" => { validate_finite_parameter("WVBEGIDL", value)?; self.params.p617 = value; self.mark_param_given(617); self.recompute_instance_static(); Ok(()) }
            "pvbegidl" => { validate_finite_parameter("PVBEGIDL", value)?; self.params.p618 = value; self.mark_param_given(618); self.recompute_instance_static(); Ok(()) }
            "lagisl" => { validate_finite_parameter("LAGISL", value)?; self.params.p619 = value; self.mark_param_given(619); self.recompute_instance_static(); Ok(()) }
            "wagisl" => { validate_finite_parameter("WAGISL", value)?; self.params.p620 = value; self.mark_param_given(620); self.recompute_instance_static(); Ok(()) }
            "pagisl" => { validate_finite_parameter("PAGISL", value)?; self.params.p621 = value; self.mark_param_given(621); self.recompute_instance_static(); Ok(()) }
            "lbgisl" => { validate_finite_parameter("LBGISL", value)?; self.params.p622 = value; self.mark_param_given(622); self.recompute_instance_static(); Ok(()) }
            "wbgisl" => { validate_finite_parameter("WBGISL", value)?; self.params.p623 = value; self.mark_param_given(623); self.recompute_instance_static(); Ok(()) }
            "pbgisl" => { validate_finite_parameter("PBGISL", value)?; self.params.p624 = value; self.mark_param_given(624); self.recompute_instance_static(); Ok(()) }
            "legisl" => { validate_finite_parameter("LEGISL", value)?; self.params.p625 = value; self.mark_param_given(625); self.recompute_instance_static(); Ok(()) }
            "wegisl" => { validate_finite_parameter("WEGISL", value)?; self.params.p626 = value; self.mark_param_given(626); self.recompute_instance_static(); Ok(()) }
            "pegisl" => { validate_finite_parameter("PEGISL", value)?; self.params.p627 = value; self.mark_param_given(627); self.recompute_instance_static(); Ok(()) }
            "lpgisl" => { validate_finite_parameter("LPGISL", value)?; self.params.p628 = value; self.mark_param_given(628); self.recompute_instance_static(); Ok(()) }
            "wpgisl" => { validate_finite_parameter("WPGISL", value)?; self.params.p629 = value; self.mark_param_given(629); self.recompute_instance_static(); Ok(()) }
            "ppgisl" => { validate_finite_parameter("PPGISL", value)?; self.params.p630 = value; self.mark_param_given(630); self.recompute_instance_static(); Ok(()) }
            "lvbgisl" => { validate_finite_parameter("LVBGISL", value)?; self.params.p631 = value; self.mark_param_given(631); self.recompute_instance_static(); Ok(()) }
            "wvbgisl" => { validate_finite_parameter("WVBGISL", value)?; self.params.p632 = value; self.mark_param_given(632); self.recompute_instance_static(); Ok(()) }
            "pvbgisl" => { validate_finite_parameter("PVBGISL", value)?; self.params.p633 = value; self.mark_param_given(633); self.recompute_instance_static(); Ok(()) }
            "lvbegisl" => { validate_finite_parameter("LVBEGISL", value)?; self.params.p634 = value; self.mark_param_given(634); self.recompute_instance_static(); Ok(()) }
            "wvbegisl" => { validate_finite_parameter("WVBEGISL", value)?; self.params.p635 = value; self.mark_param_given(635); self.recompute_instance_static(); Ok(()) }
            "pvbegisl" => { validate_finite_parameter("PVBEGISL", value)?; self.params.p636 = value; self.mark_param_given(636); self.recompute_instance_static(); Ok(()) }
            "laigs" => { validate_finite_parameter("LAIGS", value)?; self.params.p637 = value; self.mark_param_given(637); self.recompute_instance_static(); Ok(()) }
            "waigs" => { validate_finite_parameter("WAIGS", value)?; self.params.p638 = value; self.mark_param_given(638); self.recompute_instance_static(); Ok(()) }
            "paigs" => { validate_finite_parameter("PAIGS", value)?; self.params.p639 = value; self.mark_param_given(639); self.recompute_instance_static(); Ok(()) }
            "laigd" => { validate_finite_parameter("LAIGD", value)?; self.params.p640 = value; self.mark_param_given(640); self.recompute_instance_static(); Ok(()) }
            "waigd" => { validate_finite_parameter("WAIGD", value)?; self.params.p641 = value; self.mark_param_given(641); self.recompute_instance_static(); Ok(()) }
            "paigd" => { validate_finite_parameter("PAIGD", value)?; self.params.p642 = value; self.mark_param_given(642); self.recompute_instance_static(); Ok(()) }
            "lbigs" => { validate_finite_parameter("LBIGS", value)?; self.params.p643 = value; self.mark_param_given(643); self.recompute_instance_static(); Ok(()) }
            "wbigs" => { validate_finite_parameter("WBIGS", value)?; self.params.p644 = value; self.mark_param_given(644); self.recompute_instance_static(); Ok(()) }
            "pbigs" => { validate_finite_parameter("PBIGS", value)?; self.params.p645 = value; self.mark_param_given(645); self.recompute_instance_static(); Ok(()) }
            "lbigd" => { validate_finite_parameter("LBIGD", value)?; self.params.p646 = value; self.mark_param_given(646); self.recompute_instance_static(); Ok(()) }
            "wbigd" => { validate_finite_parameter("WBIGD", value)?; self.params.p647 = value; self.mark_param_given(647); self.recompute_instance_static(); Ok(()) }
            "pbigd" => { validate_finite_parameter("PBIGD", value)?; self.params.p648 = value; self.mark_param_given(648); self.recompute_instance_static(); Ok(()) }
            "lcigs" => { validate_finite_parameter("LCIGS", value)?; self.params.p649 = value; self.mark_param_given(649); self.recompute_instance_static(); Ok(()) }
            "wcigs" => { validate_finite_parameter("WCIGS", value)?; self.params.p650 = value; self.mark_param_given(650); self.recompute_instance_static(); Ok(()) }
            "pcigs" => { validate_finite_parameter("PCIGS", value)?; self.params.p651 = value; self.mark_param_given(651); self.recompute_instance_static(); Ok(()) }
            "lcigd" => { validate_finite_parameter("LCIGD", value)?; self.params.p652 = value; self.mark_param_given(652); self.recompute_instance_static(); Ok(()) }
            "wcigd" => { validate_finite_parameter("WCIGD", value)?; self.params.p653 = value; self.mark_param_given(653); self.recompute_instance_static(); Ok(()) }
            "pcigd" => { validate_finite_parameter("PCIGD", value)?; self.params.p654 = value; self.mark_param_given(654); self.recompute_instance_static(); Ok(()) }
            "ldigs" => { validate_finite_parameter("LDIGS", value)?; self.params.p655 = value; self.mark_param_given(655); self.recompute_instance_static(); Ok(()) }
            "wdigs" => { validate_finite_parameter("WDIGS", value)?; self.params.p656 = value; self.mark_param_given(656); self.recompute_instance_static(); Ok(()) }
            "pdigs" => { validate_finite_parameter("PDIGS", value)?; self.params.p657 = value; self.mark_param_given(657); self.recompute_instance_static(); Ok(()) }
            "ldigd" => { validate_finite_parameter("LDIGD", value)?; self.params.p658 = value; self.mark_param_given(658); self.recompute_instance_static(); Ok(()) }
            "wdigd" => { validate_finite_parameter("WDIGD", value)?; self.params.p659 = value; self.mark_param_given(659); self.recompute_instance_static(); Ok(()) }
            "pdigd" => { validate_finite_parameter("PDIGD", value)?; self.params.p660 = value; self.mark_param_given(660); self.recompute_instance_static(); Ok(()) }
            "lntox" => { validate_finite_parameter("LNTOX", value)?; self.params.p661 = value; self.mark_param_given(661); self.recompute_instance_static(); Ok(()) }
            "wntox" => { validate_finite_parameter("WNTOX", value)?; self.params.p662 = value; self.mark_param_given(662); self.recompute_instance_static(); Ok(()) }
            "pntox" => { validate_finite_parameter("PNTOX", value)?; self.params.p663 = value; self.mark_param_given(663); self.recompute_instance_static(); Ok(()) }
            "lpoxedge" => { validate_finite_parameter("LPOXEDGE", value)?; self.params.p664 = value; self.mark_param_given(664); self.recompute_instance_static(); Ok(()) }
            "wpoxedge" => { validate_finite_parameter("WPOXEDGE", value)?; self.params.p665 = value; self.mark_param_given(665); self.recompute_instance_static(); Ok(()) }
            "ppoxedge" => { validate_finite_parameter("PPOXEDGE", value)?; self.params.p666 = value; self.mark_param_given(666); self.recompute_instance_static(); Ok(()) }
            "llovs" => { validate_finite_parameter("LLOVS", value)?; self.params.p667 = value; self.mark_param_given(667); self.recompute_instance_static(); Ok(()) }
            "wlovs" => { validate_finite_parameter("WLOVS", value)?; self.params.p668 = value; self.mark_param_given(668); self.recompute_instance_static(); Ok(()) }
            "plovs" => { validate_finite_parameter("PLOVS", value)?; self.params.p669 = value; self.mark_param_given(669); self.recompute_instance_static(); Ok(()) }
            "llovd" => { validate_finite_parameter("LLOVD", value)?; self.params.p670 = value; self.mark_param_given(670); self.recompute_instance_static(); Ok(()) }
            "wlovd" => { validate_finite_parameter("WLOVD", value)?; self.params.p671 = value; self.mark_param_given(671); self.recompute_instance_static(); Ok(()) }
            "plovd" => { validate_finite_parameter("PLOVD", value)?; self.params.p672 = value; self.mark_param_given(672); self.recompute_instance_static(); Ok(()) }
            "lcfs" => { validate_finite_parameter("LCFS", value)?; self.params.p673 = value; self.mark_param_given(673); self.recompute_instance_static(); Ok(()) }
            "wcfs" => { validate_finite_parameter("WCFS", value)?; self.params.p674 = value; self.mark_param_given(674); self.recompute_instance_static(); Ok(()) }
            "pcfs" => { validate_finite_parameter("PCFS", value)?; self.params.p675 = value; self.mark_param_given(675); self.recompute_instance_static(); Ok(()) }
            "lcfd" => { validate_finite_parameter("LCFD", value)?; self.params.p676 = value; self.mark_param_given(676); self.recompute_instance_static(); Ok(()) }
            "wcfd" => { validate_finite_parameter("WCFD", value)?; self.params.p677 = value; self.mark_param_given(677); self.recompute_instance_static(); Ok(()) }
            "pcfd" => { validate_finite_parameter("PCFD", value)?; self.params.p678 = value; self.mark_param_given(678); self.recompute_instance_static(); Ok(()) }
            "lvsat" => { validate_finite_parameter("LVSAT", value)?; self.params.p679 = value; self.mark_param_given(679); self.recompute_instance_static(); Ok(()) }
            "wvsat" => { validate_finite_parameter("WVSAT", value)?; self.params.p680 = value; self.mark_param_given(680); self.recompute_instance_static(); Ok(()) }
            "pvsat" => { validate_finite_parameter("PVSAT", value)?; self.params.p681 = value; self.mark_param_given(681); self.recompute_instance_static(); Ok(()) }
            "lvsatb" => { validate_finite_parameter("LVSATB", value)?; self.params.p682 = value; self.mark_param_given(682); self.recompute_instance_static(); Ok(()) }
            "wvsatb" => { validate_finite_parameter("WVSATB", value)?; self.params.p683 = value; self.mark_param_given(683); self.recompute_instance_static(); Ok(()) }
            "pvsatb" => { validate_finite_parameter("PVSATB", value)?; self.params.p684 = value; self.mark_param_given(684); self.recompute_instance_static(); Ok(()) }
            "lvsat1" => { validate_finite_parameter("LVSAT1", value)?; self.params.p685 = value; self.mark_param_given(685); self.recompute_instance_static(); Ok(()) }
            "wvsat1" => { validate_finite_parameter("WVSAT1", value)?; self.params.p686 = value; self.mark_param_given(686); self.recompute_instance_static(); Ok(()) }
            "pvsat1" => { validate_finite_parameter("PVSAT1", value)?; self.params.p687 = value; self.mark_param_given(687); self.recompute_instance_static(); Ok(()) }
            "lvsatcv" => { validate_finite_parameter("LVSATCV", value)?; self.params.p688 = value; self.mark_param_given(688); self.recompute_instance_static(); Ok(()) }
            "wvsatcv" => { validate_finite_parameter("WVSATCV", value)?; self.params.p689 = value; self.mark_param_given(689); self.recompute_instance_static(); Ok(()) }
            "pvsatcv" => { validate_finite_parameter("PVSATCV", value)?; self.params.p690 = value; self.mark_param_given(690); self.recompute_instance_static(); Ok(()) }
            "lksativ" => { validate_finite_parameter("LKSATIV", value)?; self.params.p691 = value; self.mark_param_given(691); self.recompute_instance_static(); Ok(()) }
            "wksativ" => { validate_finite_parameter("WKSATIV", value)?; self.params.p692 = value; self.mark_param_given(692); self.recompute_instance_static(); Ok(()) }
            "pksativ" => { validate_finite_parameter("PKSATIV", value)?; self.params.p693 = value; self.mark_param_given(693); self.recompute_instance_static(); Ok(()) }
            "lksubiv" => { validate_finite_parameter("LKSUBIV", value)?; self.params.p694 = value; self.mark_param_given(694); self.recompute_instance_static(); Ok(()) }
            "wksubiv" => { validate_finite_parameter("WKSUBIV", value)?; self.params.p695 = value; self.mark_param_given(695); self.recompute_instance_static(); Ok(()) }
            "pksubiv" => { validate_finite_parameter("PKSUBIV", value)?; self.params.p696 = value; self.mark_param_given(696); self.recompute_instance_static(); Ok(()) }
            "lksativb" => { validate_finite_parameter("LKSATIVB", value)?; self.params.p697 = value; self.mark_param_given(697); self.recompute_instance_static(); Ok(()) }
            "wksativb" => { validate_finite_parameter("WKSATIVB", value)?; self.params.p698 = value; self.mark_param_given(698); self.recompute_instance_static(); Ok(()) }
            "pksativb" => { validate_finite_parameter("PKSATIVB", value)?; self.params.p699 = value; self.mark_param_given(699); self.recompute_instance_static(); Ok(()) }
            "lup" => { validate_finite_parameter("LUP", value)?; self.params.p700 = value; self.mark_param_given(700); self.recompute_instance_static(); Ok(()) }
            "wup" => { validate_finite_parameter("WUP", value)?; self.params.p701 = value; self.mark_param_given(701); self.recompute_instance_static(); Ok(()) }
            "pup" => { validate_finite_parameter("PUP", value)?; self.params.p702 = value; self.mark_param_given(702); self.recompute_instance_static(); Ok(()) }
            "lup2" => { validate_finite_parameter("LUP2", value)?; self.params.p703 = value; self.mark_param_given(703); self.recompute_instance_static(); Ok(()) }
            "wup2" => { validate_finite_parameter("WUP2", value)?; self.params.p704 = value; self.mark_param_given(704); self.recompute_instance_static(); Ok(()) }
            "pup2" => { validate_finite_parameter("PUP2", value)?; self.params.p705 = value; self.mark_param_given(705); self.recompute_instance_static(); Ok(()) }
            "laigbinv" => { validate_finite_parameter("LAIGBINV", value)?; self.params.p706 = value; self.mark_param_given(706); self.recompute_instance_static(); Ok(()) }
            "waigbinv" => { validate_finite_parameter("WAIGBINV", value)?; self.params.p707 = value; self.mark_param_given(707); self.recompute_instance_static(); Ok(()) }
            "paigbinv" => { validate_finite_parameter("PAIGBINV", value)?; self.params.p708 = value; self.mark_param_given(708); self.recompute_instance_static(); Ok(()) }
            "lbigbinv" => { validate_finite_parameter("LBIGBINV", value)?; self.params.p709 = value; self.mark_param_given(709); self.recompute_instance_static(); Ok(()) }
            "wbigbinv" => { validate_finite_parameter("WBIGBINV", value)?; self.params.p710 = value; self.mark_param_given(710); self.recompute_instance_static(); Ok(()) }
            "pbigbinv" => { validate_finite_parameter("PBIGBINV", value)?; self.params.p711 = value; self.mark_param_given(711); self.recompute_instance_static(); Ok(()) }
            "lcigbinv" => { validate_finite_parameter("LCIGBINV", value)?; self.params.p712 = value; self.mark_param_given(712); self.recompute_instance_static(); Ok(()) }
            "wcigbinv" => { validate_finite_parameter("WCIGBINV", value)?; self.params.p713 = value; self.mark_param_given(713); self.recompute_instance_static(); Ok(()) }
            "pcigbinv" => { validate_finite_parameter("PCIGBINV", value)?; self.params.p714 = value; self.mark_param_given(714); self.recompute_instance_static(); Ok(()) }
            "leigbinv" => { validate_finite_parameter("LEIGBINV", value)?; self.params.p715 = value; self.mark_param_given(715); self.recompute_instance_static(); Ok(()) }
            "weigbinv" => { validate_finite_parameter("WEIGBINV", value)?; self.params.p716 = value; self.mark_param_given(716); self.recompute_instance_static(); Ok(()) }
            "peigbinv" => { validate_finite_parameter("PEIGBINV", value)?; self.params.p717 = value; self.mark_param_given(717); self.recompute_instance_static(); Ok(()) }
            "lnigbinv" => { validate_finite_parameter("LNIGBINV", value)?; self.params.p718 = value; self.mark_param_given(718); self.recompute_instance_static(); Ok(()) }
            "wnigbinv" => { validate_finite_parameter("WNIGBINV", value)?; self.params.p719 = value; self.mark_param_given(719); self.recompute_instance_static(); Ok(()) }
            "pnigbinv" => { validate_finite_parameter("PNIGBINV", value)?; self.params.p720 = value; self.mark_param_given(720); self.recompute_instance_static(); Ok(()) }
            "laigbacc" => { validate_finite_parameter("LAIGBACC", value)?; self.params.p721 = value; self.mark_param_given(721); self.recompute_instance_static(); Ok(()) }
            "waigbacc" => { validate_finite_parameter("WAIGBACC", value)?; self.params.p722 = value; self.mark_param_given(722); self.recompute_instance_static(); Ok(()) }
            "paigbacc" => { validate_finite_parameter("PAIGBACC", value)?; self.params.p723 = value; self.mark_param_given(723); self.recompute_instance_static(); Ok(()) }
            "lbigbacc" => { validate_finite_parameter("LBIGBACC", value)?; self.params.p724 = value; self.mark_param_given(724); self.recompute_instance_static(); Ok(()) }
            "wbigbacc" => { validate_finite_parameter("WBIGBACC", value)?; self.params.p725 = value; self.mark_param_given(725); self.recompute_instance_static(); Ok(()) }
            "pbigbacc" => { validate_finite_parameter("PBIGBACC", value)?; self.params.p726 = value; self.mark_param_given(726); self.recompute_instance_static(); Ok(()) }
            "lcigbacc" => { validate_finite_parameter("LCIGBACC", value)?; self.params.p727 = value; self.mark_param_given(727); self.recompute_instance_static(); Ok(()) }
            "wcigbacc" => { validate_finite_parameter("WCIGBACC", value)?; self.params.p728 = value; self.mark_param_given(728); self.recompute_instance_static(); Ok(()) }
            "pcigbacc" => { validate_finite_parameter("PCIGBACC", value)?; self.params.p729 = value; self.mark_param_given(729); self.recompute_instance_static(); Ok(()) }
            "lnigbacc" => { validate_finite_parameter("LNIGBACC", value)?; self.params.p730 = value; self.mark_param_given(730); self.recompute_instance_static(); Ok(()) }
            "wnigbacc" => { validate_finite_parameter("WNIGBACC", value)?; self.params.p731 = value; self.mark_param_given(731); self.recompute_instance_static(); Ok(()) }
            "pnigbacc" => { validate_finite_parameter("PNIGBACC", value)?; self.params.p732 = value; self.mark_param_given(732); self.recompute_instance_static(); Ok(()) }
            "lxrcrg1" => { validate_finite_parameter("LXRCRG1", value)?; self.params.p733 = value; self.mark_param_given(733); self.recompute_instance_static(); Ok(()) }
            "wxrcrg1" => { validate_finite_parameter("WXRCRG1", value)?; self.params.p734 = value; self.mark_param_given(734); self.recompute_instance_static(); Ok(()) }
            "pxrcrg1" => { validate_finite_parameter("PXRCRG1", value)?; self.params.p735 = value; self.mark_param_given(735); self.recompute_instance_static(); Ok(()) }
            "lxrcrg2" => { validate_finite_parameter("LXRCRG2", value)?; self.params.p736 = value; self.mark_param_given(736); self.recompute_instance_static(); Ok(()) }
            "wxrcrg2" => { validate_finite_parameter("WXRCRG2", value)?; self.params.p737 = value; self.mark_param_given(737); self.recompute_instance_static(); Ok(()) }
            "pxrcrg2" => { validate_finite_parameter("PXRCRG2", value)?; self.params.p738 = value; self.mark_param_given(738); self.recompute_instance_static(); Ok(()) }
            "lqmtcencv" => { validate_finite_parameter("LQMTCENCV", value)?; self.params.p739 = value; self.mark_param_given(739); self.recompute_instance_static(); Ok(()) }
            "wqmtcencv" => { validate_finite_parameter("WQMTCENCV", value)?; self.params.p740 = value; self.mark_param_given(740); self.recompute_instance_static(); Ok(()) }
            "pqmtcencv" => { validate_finite_parameter("PQMTCENCV", value)?; self.params.p741 = value; self.mark_param_given(741); self.recompute_instance_static(); Ok(()) }
            "letaqm" => { validate_finite_parameter("LETAQM", value)?; self.params.p742 = value; self.mark_param_given(742); self.recompute_instance_static(); Ok(()) }
            "wetaqm" => { validate_finite_parameter("WETAQM", value)?; self.params.p743 = value; self.mark_param_given(743); self.recompute_instance_static(); Ok(()) }
            "petaqm" => { validate_finite_parameter("PETAQM", value)?; self.params.p744 = value; self.mark_param_given(744); self.recompute_instance_static(); Ok(()) }
            "lqm0" => { validate_finite_parameter("LQM0", value)?; self.params.p745 = value; self.mark_param_given(745); self.recompute_instance_static(); Ok(()) }
            "wqm0" => { validate_finite_parameter("WQM0", value)?; self.params.p746 = value; self.mark_param_given(746); self.recompute_instance_static(); Ok(()) }
            "pqm0" => { validate_finite_parameter("PQM0", value)?; self.params.p747 = value; self.mark_param_given(747); self.recompute_instance_static(); Ok(()) }
            "lpqm" => { validate_finite_parameter("LPQM", value)?; self.params.p748 = value; self.mark_param_given(748); self.recompute_instance_static(); Ok(()) }
            "wpqm" => { validate_finite_parameter("WPQM", value)?; self.params.p749 = value; self.mark_param_given(749); self.recompute_instance_static(); Ok(()) }
            "ppqm" => { validate_finite_parameter("PPQM", value)?; self.params.p750 = value; self.mark_param_given(750); self.recompute_instance_static(); Ok(()) }
            "lnoia2" => { validate_finite_parameter("LNOIA2", value)?; self.params.p751 = value; self.mark_param_given(751); self.recompute_instance_static(); Ok(()) }
            "wnoia2" => { validate_finite_parameter("WNOIA2", value)?; self.params.p752 = value; self.mark_param_given(752); self.recompute_instance_static(); Ok(()) }
            "pnoia2" => { validate_finite_parameter("PNOIA2", value)?; self.params.p753 = value; self.mark_param_given(753); self.recompute_instance_static(); Ok(()) }
            "lmpower" => { validate_finite_parameter("LMPOWER", value)?; self.params.p754 = value; self.mark_param_given(754); self.recompute_instance_static(); Ok(()) }
            "wmpower" => { validate_finite_parameter("WMPOWER", value)?; self.params.p755 = value; self.mark_param_given(755); self.recompute_instance_static(); Ok(()) }
            "pmpower" => { validate_finite_parameter("PMPOWER", value)?; self.params.p756 = value; self.mark_param_given(756); self.recompute_instance_static(); Ok(()) }
            "lqsref" => { validate_finite_parameter("LQSREF", value)?; self.params.p757 = value; self.mark_param_given(757); self.recompute_instance_static(); Ok(()) }
            "wqsref" => { validate_finite_parameter("WQSREF", value)?; self.params.p758 = value; self.mark_param_given(758); self.recompute_instance_static(); Ok(()) }
            "pqsref" => { validate_finite_parameter("PQSREF", value)?; self.params.p759 = value; self.mark_param_given(759); self.recompute_instance_static(); Ok(()) }
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
        let v2: f64 = p.p18;
        self.scalar_v2 = v2;
        let v3: f64 = p.p310;
        self.scalar_v3 = v3;
        let v4: f64 = p.p21;
        self.scalar_v4 = v4;
        let v5: bool = (0.0 == p.p21);
        self.scalar_v5 = v5;
        let v6: f64 = p.p1;
        self.scalar_v6 = v6;
        let v7: f64 = p.p2;
        self.scalar_v7 = v7;
        let v8: f64 = (p.p1 / p.p2);
        self.scalar_v8 = v8;
        let v9: f64 = (if v5 { v8 } else { 0.0 });
        self.scalar_v9 = v9;
        let v10: bool = (!v5);
        self.scalar_v10 = v10;
        let v11: f64 = (if v10 { p.p1 } else { v9 });
        self.scalar_v11 = v11;
        let v12: f64 = p.p0;
        self.scalar_v12 = v12;
        let v13: f64 = p.p23;
        self.scalar_v13 = v13;
        let v14: f64 = (p.p0 + p.p23);
        self.scalar_v14 = v14;
        let v15: f64 = p.p24;
        self.scalar_v15 = v15;
        let v16: f64 = (v11 + p.p24);
        self.scalar_v16 = v16;
        let v17: f64 = p.p29;
        self.scalar_v17 = v17;
        let v18: f64 = (-p.p29);
        self.scalar_v18 = v18;
        let v19: f64 = f64::powf(v14, v18);
        self.scalar_v19 = v19;
        let v20: f64 = p.p30;
        self.scalar_v20 = v20;
        let v21: f64 = (-p.p30);
        self.scalar_v21 = v21;
        let v22: f64 = f64::powf(v16, v21);
        self.scalar_v22 = v22;
        let v23: f64 = (v19 * v22);
        self.scalar_v23 = v23;
        let v24: f64 = p.p25;
        self.scalar_v24 = v24;
        let v25: f64 = p.p26;
        self.scalar_v25 = v25;
        let v26: f64 = (v19 * p.p26);
        self.scalar_v26 = v26;
        let v27: f64 = (p.p25 + v26);
        self.scalar_v27 = v27;
        let v28: f64 = p.p27;
        self.scalar_v28 = v28;
        let v29: f64 = (v22 * p.p27);
        self.scalar_v29 = v29;
        let v30: f64 = (v27 + v29);
        self.scalar_v30 = v30;
        let v31: f64 = p.p28;
        self.scalar_v31 = v31;
        let v32: f64 = (v23 * p.p28);
        self.scalar_v32 = v32;
        let v33: f64 = (v30 + v32);
        self.scalar_v33 = v33;
        let v34: f64 = p.p35;
        self.scalar_v34 = v34;
        let v35: f64 = (-p.p35);
        self.scalar_v35 = v35;
        let v36: f64 = f64::powf(v14, v35);
        self.scalar_v36 = v36;
        let v37: f64 = p.p36;
        self.scalar_v37 = v37;
        let v38: f64 = (-p.p36);
        self.scalar_v38 = v38;
        let v39: f64 = f64::powf(v16, v38);
        self.scalar_v39 = v39;
        let v40: f64 = (v36 * v39);
        self.scalar_v40 = v40;
        let v41: f64 = p.p31;
        self.scalar_v41 = v41;
        let v42: f64 = p.p32;
        self.scalar_v42 = v42;
        let v43: f64 = (v36 * p.p32);
        self.scalar_v43 = v43;
        let v44: f64 = (p.p31 + v43);
        self.scalar_v44 = v44;
        let v45: f64 = p.p33;
        self.scalar_v45 = v45;
        let v46: f64 = (v39 * p.p33);
        self.scalar_v46 = v46;
        let v47: f64 = (v44 + v46);
        self.scalar_v47 = v47;
        let v48: f64 = p.p34;
        self.scalar_v48 = v48;
        let v49: f64 = (v40 * p.p34);
        self.scalar_v49 = v49;
        let v50: f64 = (v47 + v49);
        self.scalar_v50 = v50;
        let v52: f64 = (v33 * 2.0);
        self.scalar_v52 = v52;
        let v53: f64 = (v14 - v52);
        self.scalar_v53 = v53;
        let v54: f64 = (v50 * 2.0);
        self.scalar_v54 = v54;
        let v55: f64 = (v16 - v54);
        self.scalar_v55 = v55;
        let v57: f64 = (1e-6 / v53);
        self.scalar_v57 = v57;
        let v58: f64 = (1e-6 / v55);
        self.scalar_v58 = v58;
        let v59: f64 = (v57 * v58);
        self.scalar_v59 = v59;
        let v60: f64 = p.p20;
        self.scalar_v60 = v60;
        let v61: bool = (1.0 == p.p20);
        self.scalar_v61 = v61;
        let v62: f64 = p.p317;
        self.scalar_v62 = v62;
        let v63: bool = (0.0 != p.p317);
        self.scalar_v63 = v63;
        let v64: bool = (v61 && v63);
        self.scalar_v64 = v64;
        let v65: f64 = p.p733;
        self.scalar_v65 = v65;
        let v66: f64 = (v57 * p.p733);
        self.scalar_v66 = v66;
        let v67: f64 = (p.p317 + v66);
        self.scalar_v67 = v67;
        let v68: f64 = p.p734;
        self.scalar_v68 = v68;
        let v69: f64 = (v58 * p.p734);
        self.scalar_v69 = v69;
        let v70: f64 = (v67 + v69);
        self.scalar_v70 = v70;
        let v71: f64 = p.p735;
        self.scalar_v71 = v71;
        let v72: f64 = (v59 * p.p735);
        self.scalar_v72 = v72;
        let v73: f64 = (v70 + v72);
        self.scalar_v73 = v73;
        let v74: f64 = (if v64 { v73 } else { 0.0 });
        self.scalar_v74 = v74;
        let v75: bool = (!v64);
        self.scalar_v75 = v75;
        let v76: f64 = (if v75 { 0.0 } else { v74 });
        self.scalar_v76 = v76;
        let v77: f64 = p.p14;
        self.scalar_v77 = v77;
        let v79: bool = (0.0 != p.p18);
        self.scalar_v79 = v79;
        let v80: bool = (p.p310 > 0.0);
        self.scalar_v80 = v80;
        let v81: bool = (v79 && v80);
        self.scalar_v81 = v81;
        let v82: bool = (!v81);
        self.scalar_v82 = v82;
        let v84: f64 = p.p316;
        self.scalar_v84 = v84;
        let v85: f64 = p.p313;
        self.scalar_v85 = v85;
        let v86: f64 = (v55 / 3.0);
        self.scalar_v86 = v86;
        let v87: f64 = p.p315;
        self.scalar_v87 = v87;
        let v88: f64 = (v86 / p.p315);
        self.scalar_v88 = v88;
        let v89: f64 = (p.p313 + v88);
        self.scalar_v89 = v89;
        let v90: f64 = (p.p316 * v89);
        self.scalar_v90 = v90;
        let v91: f64 = (p.p2 * p.p315);
        self.scalar_v91 = v91;
        let v92: f64 = p.p314;
        self.scalar_v92 = v92;
        let v93: f64 = (v14 - p.p314);
        self.scalar_v93 = v93;
        let v94: f64 = (v91 * v93);
        self.scalar_v94 = v94;
        let v95: f64 = (v90 / v94);
        self.scalar_v95 = v95;
        let v96: bool = (v95 > 0.001);
        self.scalar_v96 = v96;
        let v97: f64 = (1.0 / v95);
        self.scalar_v97 = v97;
        let v98: f64 = (if v96 { v97 } else { v95 });
        self.scalar_v98 = v98;
        let v99: bool = (!v96);
        self.scalar_v99 = v99;
        let v101: f64 = (if v99 { 1000.0 } else { v98 });
        self.scalar_v101 = v101;
        let v102: f64 = p.p19;
        self.scalar_v102 = v102;
        let v104: bool = (2.0 == p.p14);
        self.scalar_v104 = v104;
        let v105: bool = (0.0 != v76);
        self.scalar_v105 = v105;
        let v106: bool = (v61 && v105);
        self.scalar_v106 = v106;
        let v107: bool = (!v106);
        self.scalar_v107 = v107;
        let v108: bool = (0.0 == p.p19);
        self.scalar_v108 = v108;
        let v109: bool = (!v108);
        self.scalar_v109 = v109;
        let v110: f64 = (if v109 { v101 } else { 0.0 });
        self.scalar_v110 = v110;
        let v112: f64 = (if v104 { 0.0 } else { 0.0 });
        self.scalar_v112 = v112;
        let v113: f64 = (if v107 { 0.0 } else { 0.0 });
        self.scalar_v113 = v113;
        let v114: f64 = (if v108 { 0.0 } else { 0.0 });
        self.scalar_v114 = v114;
        let v118: f64 = (if v82 { 0.0 } else { 0.0 });
        self.scalar_v118 = v118;
        let v119: f64 = (-v110);
        self.scalar_v119 = v119;
        let v120: f64 = (if v109 { v110 } else { 0.0 });
        self.scalar_v120 = v120;
        let v121: f64 = (if v109 { v119 } else { 0.0 });
        self.scalar_v121 = v121;
    }
}
