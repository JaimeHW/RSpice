#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Parameters {
    pub p0: f64, pub p1: f64, pub p2: f64, pub p3: f64, pub p4: f64, pub p5: f64, pub p6: f64, pub p7: f64,
    pub p8: f64, pub p9: f64, pub p10: f64, pub p11: f64, pub p12: f64, pub p13: f64, pub p14: f64, pub p15: f64,
    pub p16: f64, pub p17: f64, pub p18: f64, pub p19: f64, pub p20: f64, pub p21: f64, pub p22: f64, pub p23: f64,
    pub p24: f64, pub p25: f64, pub p26: f64, pub p27: f64, pub p28: f64, pub p29: f64, pub p30: f64, pub p31: f64,
    pub p32: f64, pub p33: f64, pub p34: f64, pub p35: f64, pub p36: f64, pub p37: f64, pub p38: f64, pub p39: f64,
    pub p40: f64, pub p41: f64, pub p42: f64, pub p43: f64, pub p44: f64, pub p45: f64, pub p46: f64, pub p47: f64,
    pub p48: f64, pub p49: f64, pub p50: f64, pub p51: f64, pub p52: f64, pub p53: f64, pub p54: f64, pub p55: f64,
    pub p56: f64, pub p57: f64, pub p58: f64, pub p59: f64, pub p60: f64, pub p61: f64, pub p62: f64, pub p63: f64,
    pub p64: f64, pub p65: f64, pub p66: f64, pub p67: f64, pub p68: f64, pub p69: f64, pub p70: f64, pub p71: f64,
    pub p72: f64, pub p73: f64, pub p74: f64, pub p75: f64, pub p76: f64, pub p77: f64, pub p78: f64, pub p79: f64,
    pub p80: f64, pub p81: f64, pub p82: f64, pub p83: f64, pub p84: f64, pub p85: f64, pub p86: f64, pub p87: f64,
    pub p88: f64, pub p89: f64, pub p90: f64, pub p91: f64, pub p92: f64, pub p93: f64, pub p94: f64, pub p95: f64,
    pub p96: f64, pub p97: f64, pub p98: f64, pub p99: f64, pub p100: f64, pub p101: f64, pub p102: f64, pub p103: f64,
    pub p104: f64, pub p105: f64, pub p106: f64, pub p107: f64, pub p108: f64, pub p109: f64, pub p110: f64, pub p111: f64,
    pub p112: f64, pub p113: f64, pub p114: f64, pub p115: f64, pub p116: f64, pub p117: f64, pub p118: f64, pub p119: f64,
    pub p120: f64, pub p121: f64, pub p122: f64, pub p123: f64, pub p124: f64, pub p125: f64, pub p126: f64, pub p127: f64,
    pub p128: f64, pub p129: f64, pub p130: f64, pub p131: f64, pub p132: f64, pub p133: f64, pub p134: f64, pub p135: f64,
    pub p136: f64, pub p137: f64, pub p138: f64, pub p139: f64, pub p140: f64, pub p141: f64, pub p142: f64, pub p143: f64,
    pub p144: f64, pub p145: f64, pub p146: f64, pub p147: f64, pub p148: f64, pub p149: f64, pub p150: f64, pub p151: f64,
    pub p152: f64, pub p153: f64, pub p154: f64, pub p155: f64, pub p156: f64, pub p157: f64, pub p158: f64, pub p159: f64,
    pub p160: f64, pub p161: f64, pub p162: f64, pub p163: f64, pub p164: f64, pub p165: f64, pub p166: f64, pub p167: f64,
    pub p168: f64, pub p169: f64, pub p170: f64, pub p171: f64, pub p172: f64, pub p173: f64, pub p174: f64, pub p175: f64,
    pub p176: f64, pub p177: f64, pub p178: f64, pub p179: f64, pub p180: f64, pub p181: f64, pub p182: f64, pub p183: f64,
    pub p184: f64, pub p185: f64, pub p186: f64, pub p187: f64, pub p188: f64, pub p189: f64, pub p190: f64, pub p191: f64,
    pub p192: f64, pub p193: f64, pub p194: f64, pub p195: f64, pub p196: f64, pub p197: f64, pub p198: f64, pub p199: f64,
    pub p200: f64, pub p201: f64, pub p202: f64, pub p203: f64, pub p204: f64, pub p205: f64, pub p206: f64, pub p207: f64,
    pub p208: f64, pub p209: f64, pub p210: f64, pub p211: f64, pub p212: f64, pub p213: f64, pub p214: f64, pub p215: f64,
    pub p216: f64, pub p217: f64, pub p218: f64, pub p219: f64, pub p220: f64, pub p221: f64, pub p222: f64, pub p223: f64,
    pub p224: f64, pub p225: f64, pub p226: f64, pub p227: f64, pub p228: f64, pub p229: f64, pub p230: f64, pub p231: f64,
    pub p232: f64, pub p233: f64, pub p234: f64, pub p235: f64, pub p236: f64, pub p237: f64, pub p238: f64, pub p239: f64,
    pub p240: f64, pub p241: f64, pub p242: f64, pub p243: f64, pub p244: f64, pub p245: f64, pub p246: f64, pub p247: f64,
    pub p248: f64, pub p249: f64, pub p250: f64, pub p251: f64, pub p252: f64, pub p253: f64, pub p254: f64, pub p255: f64,
    pub p256: f64, pub p257: f64, pub p258: f64, pub p259: f64, pub p260: f64, pub p261: f64, pub p262: f64, pub p263: f64,
    pub p264: f64, pub p265: f64, pub p266: f64, pub p267: f64, pub p268: f64, pub p269: f64, pub p270: f64, pub p271: f64,
    pub p272: f64, pub p273: f64, pub p274: f64, pub p275: f64, pub p276: f64, pub p277: f64, pub p278: f64, pub p279: f64,
    pub p280: f64, pub p281: f64, pub p282: f64, pub p283: f64, pub p284: f64, pub p285: f64, pub p286: f64, pub p287: f64,
    pub p288: f64, pub p289: f64, pub p290: f64, pub p291: f64, pub p292: f64, pub p293: f64, pub p294: f64, pub p295: f64,
    pub p296: f64, pub p297: f64, pub p298: f64, pub p299: f64, pub p300: f64, pub p301: f64, pub p302: f64, pub p303: f64,
    pub p304: f64, pub p305: f64, pub p306: f64, pub p307: f64, pub p308: f64, pub p309: f64, pub p310: f64, pub p311: f64,
    pub p312: f64, pub p313: f64, pub p314: f64, pub p315: f64, pub p316: f64, pub p317: f64, pub p318: f64, pub p319: f64,
    pub p320: f64, pub p321: f64, pub p322: f64, pub p323: f64, pub p324: f64, pub p325: f64, pub p326: f64, pub p327: f64,
    pub p328: f64, pub p329: f64, pub p330: f64, pub p331: f64, pub p332: f64, pub p333: f64, pub p334: f64, pub p335: f64,
    pub p336: f64, pub p337: f64, pub p338: f64, pub p339: f64, pub p340: f64, pub p341: f64, pub p342: f64, pub p343: f64,
    pub p344: f64, pub p345: f64, pub p346: f64, pub p347: f64, pub p348: f64, pub p349: f64, pub p350: f64, pub p351: f64,
    pub p352: f64, pub p353: f64, pub p354: f64, pub p355: f64, pub p356: f64, pub p357: f64, pub p358: f64, pub p359: f64,
    pub p360: f64, pub p361: f64, pub p362: f64, pub p363: f64, pub p364: f64, pub p365: f64, pub p366: f64, pub p367: f64,
    pub p368: f64, pub p369: f64, pub p370: f64, pub p371: f64, pub p372: f64, pub p373: f64, pub p374: f64, pub p375: f64,
    pub p376: f64, pub p377: f64, pub p378: f64, pub p379: f64, pub p380: f64, pub p381: f64, pub p382: f64, pub p383: f64,
    pub p384: f64, pub p385: f64, pub p386: f64, pub p387: f64, pub p388: f64, pub p389: f64, pub p390: f64, pub p391: f64,
    pub p392: f64, pub p393: f64, pub p394: f64, pub p395: f64, pub p396: f64, pub p397: f64, pub p398: f64, pub p399: f64,
    pub p400: f64, pub p401: f64, pub p402: f64, pub p403: f64, pub p404: f64, pub p405: f64, pub p406: f64, pub p407: f64,
    pub p408: f64, pub p409: f64, pub p410: f64, pub p411: f64, pub p412: f64, pub p413: f64, pub p414: f64, pub p415: f64,
    pub p416: f64, pub p417: f64, pub p418: f64, pub p419: f64, pub p420: f64, pub p421: f64, pub p422: f64, pub p423: f64,
    pub p424: f64, pub p425: f64, pub p426: f64, pub p427: f64, pub p428: f64, pub p429: f64, pub p430: f64, pub p431: f64,
    pub p432: f64, pub p433: f64, pub p434: f64, pub p435: f64, pub p436: f64, pub p437: f64, pub p438: f64, pub p439: f64,
    pub p440: f64, pub p441: f64, pub p442: f64, pub p443: f64, pub p444: f64, pub p445: f64, pub p446: f64, pub p447: f64,
    pub p448: f64, pub p449: f64, pub p450: f64, pub p451: f64, pub p452: f64, pub p453: f64, pub p454: f64, pub p455: f64,
    pub p456: f64, pub p457: f64, pub p458: f64, pub p459: f64, pub p460: f64, pub p461: f64, pub p462: f64, pub p463: f64,
    pub p464: f64, pub p465: f64, pub p466: f64, pub p467: f64, pub p468: f64, pub p469: f64, pub p470: f64, pub p471: f64,
    pub p472: f64, pub p473: f64, pub p474: f64, pub p475: f64, pub p476: f64, pub p477: f64, pub p478: f64, pub p479: f64,
    pub p480: f64, pub p481: f64, pub p482: f64, pub p483: f64, pub p484: f64, pub p485: f64, pub p486: f64, pub p487: f64,
    pub p488: f64, pub p489: f64, pub p490: f64, pub p491: f64, pub p492: f64, pub p493: f64, pub p494: f64, pub p495: f64,
    pub p496: f64, pub p497: f64, pub p498: f64, pub p499: f64, pub p500: f64, pub p501: f64, pub p502: f64, pub p503: f64,
    pub p504: f64, pub p505: f64, pub p506: f64, pub p507: f64, pub p508: f64, pub p509: f64, pub p510: f64, pub p511: f64,
    pub p512: f64, pub p513: f64, pub p514: f64, pub p515: f64, pub p516: f64, pub p517: f64, pub p518: f64, pub p519: f64,
    pub p520: f64, pub p521: f64, pub p522: f64, pub p523: f64, pub p524: f64, pub p525: f64, pub p526: f64, pub p527: f64,
    pub p528: f64, pub p529: f64, pub p530: f64, pub p531: f64, pub p532: f64, pub p533: f64, pub p534: f64, pub p535: f64,
    pub p536: f64, pub p537: f64, pub p538: f64, pub p539: f64, pub p540: f64, pub p541: f64, pub p542: f64, pub p543: f64,
    pub p544: f64, pub p545: f64, pub p546: f64, pub p547: f64, pub p548: f64, pub p549: f64, pub p550: f64, pub p551: f64,
    pub p552: f64, pub p553: f64, pub p554: f64, pub p555: f64, pub p556: f64, pub p557: f64, pub p558: f64, pub p559: f64,
    pub p560: f64, pub p561: f64, pub p562: f64, pub p563: f64, pub p564: f64, pub p565: f64, pub p566: f64, pub p567: f64,
    pub p568: f64, pub p569: f64, pub p570: f64, pub p571: f64, pub p572: f64, pub p573: f64, pub p574: f64, pub p575: f64,
    pub p576: f64, pub p577: f64, pub p578: f64, pub p579: f64, pub p580: f64, pub p581: f64, pub p582: f64, pub p583: f64,
    pub p584: f64, pub p585: f64, pub p586: f64, pub p587: f64, pub p588: f64, pub p589: f64, pub p590: f64, pub p591: f64,
    pub p592: f64, pub p593: f64, pub p594: f64, pub p595: f64, pub p596: f64, pub p597: f64, pub p598: f64, pub p599: f64,
    pub p600: f64, pub p601: f64, pub p602: f64, pub p603: f64, pub p604: f64, pub p605: f64, pub p606: f64, pub p607: f64,
    pub p608: f64, pub p609: f64, pub p610: f64, pub p611: f64, pub p612: f64, pub p613: f64, pub p614: f64, pub p615: f64,
    pub p616: f64, pub p617: f64, pub p618: f64, pub p619: f64, pub p620: f64, pub p621: f64, pub p622: f64, pub p623: f64,
    pub p624: f64, pub p625: f64, pub p626: f64, pub p627: f64, pub p628: f64, pub p629: f64, pub p630: f64, pub p631: f64,
    pub p632: f64, pub p633: f64, pub p634: f64, pub p635: f64, pub p636: f64, pub p637: f64, pub p638: f64, pub p639: f64,
    pub p640: f64, pub p641: f64, pub p642: f64, pub p643: f64, pub p644: f64, pub p645: f64, pub p646: f64, pub p647: f64,
    pub p648: f64, pub p649: f64, pub p650: f64, pub p651: f64, pub p652: f64, pub p653: f64, pub p654: f64, pub p655: f64,
    pub p656: f64, pub p657: f64, pub p658: f64, pub p659: f64, pub p660: f64, pub p661: f64, pub p662: f64, pub p663: f64,
    pub p664: f64, pub p665: f64, pub p666: f64, pub p667: f64, pub p668: f64, pub p669: f64, pub p670: f64, pub p671: f64,
    pub p672: f64, pub p673: f64, pub p674: f64, pub p675: f64, pub p676: f64, pub p677: f64, pub p678: f64, pub p679: f64,
    pub p680: f64, pub p681: f64, pub p682: f64, pub p683: f64, pub p684: f64, pub p685: f64, pub p686: f64, pub p687: f64,
    pub p688: f64, pub p689: f64, pub p690: f64, pub p691: f64, pub p692: f64, pub p693: f64, pub p694: f64, pub p695: f64,
    pub p696: f64, pub p697: f64, pub p698: f64, pub p699: f64, pub p700: f64, pub p701: f64, pub p702: f64, pub p703: f64,
    pub p704: f64, pub p705: f64, pub p706: f64, pub p707: f64, pub p708: f64, pub p709: f64, pub p710: f64, pub p711: f64,
    pub p712: f64, pub p713: f64, pub p714: f64, pub p715: f64, pub p716: f64, pub p717: f64, pub p718: f64, pub p719: f64,
    pub p720: f64, pub p721: f64, pub p722: f64, pub p723: f64, pub p724: f64, pub p725: f64, pub p726: f64, pub p727: f64,
    pub p728: f64, pub p729: f64, pub p730: f64, pub p731: f64, pub p732: f64, pub p733: f64, pub p734: f64, pub p735: f64,
    pub p736: f64, pub p737: f64, pub p738: f64, pub p739: f64, pub p740: f64, pub p741: f64, pub p742: f64, pub p743: f64,
    pub p744: f64, pub p745: f64, pub p746: f64, pub p747: f64, pub p748: f64, pub p749: f64, pub p750: f64, pub p751: f64,
    pub p752: f64, pub p753: f64, pub p754: f64, pub p755: f64, pub p756: f64, pub p757: f64, pub p758: f64, pub p759: f64,
    pub p760: f64, pub p761: f64, pub p762: f64, pub p763: f64, pub p764: f64, pub p765: f64, pub p766: f64, pub p767: f64,
    pub p768: f64, pub p769: f64, pub p770: f64, pub p771: f64, pub p772: f64, pub p773: f64, pub p774: f64, pub p775: f64,
    pub p776: f64, pub p777: f64, pub p778: f64, pub p779: f64, pub p780: f64, pub p781: f64, pub p782: f64, pub p783: f64,
    pub p784: f64, pub p785: f64, pub p786: f64, pub p787: f64, pub p788: f64, pub p789: f64, pub p790: f64, pub p791: f64,
    pub p792: f64, pub p793: f64, pub p794: f64, pub p795: f64, pub p796: f64, pub p797: f64, pub p798: f64, pub p799: f64,
    pub p800: f64, pub p801: f64, pub p802: f64, pub p803: f64, pub p804: f64, pub p805: f64, pub p806: f64, pub p807: f64,
    pub p808: f64, pub p809: f64, pub p810: f64, pub p811: f64, pub p812: f64, pub p813: f64, pub p814: f64, pub p815: f64,
    pub p816: f64, pub p817: f64, pub p818: f64, pub p819: f64, pub p820: f64, pub p821: f64, pub p822: f64, pub p823: f64,
    pub p824: f64, pub p825: f64, pub p826: f64, pub p827: f64, pub p828: f64, pub p829: f64, pub p830: f64, pub p831: f64,
    pub p832: f64, pub p833: f64, pub p834: f64, pub p835: f64, pub p836: f64, pub p837: f64, pub p838: f64, pub p839: f64,
    pub p840: f64, pub p841: f64, pub p842: f64, pub p843: f64, pub p844: f64, pub p845: f64, pub p846: f64, pub p847: f64,
    pub p848: f64, pub p849: f64, pub p850: f64, pub p851: f64, pub p852: f64, pub p853: f64, pub p854: f64, pub p855: f64,
    pub p856: f64, pub p857: f64, pub p858: f64, pub p859: f64, pub p860: f64, pub p861: f64, pub p862: f64, pub p863: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 36] = [
                2e-6, 5e-6, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 3e17, 0.0, 0.0, 0.0,
                0.0, 3.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 36);
            {
                let params = &mut *ptr;
                params.p36 = if (params.p34 != 0.0) { params.p35 } else { 0.0 };
                validate_parameter("COOVLPS", params.p36, true, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 28] = [
                1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 2.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 50.0, 50.0, 1.0, 0.0,
                0.0, 1.0, 1e-6, 1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(37), 28);
            {
                let params = &mut *ptr;
                params.p65 = if (params.p34 != 0.0) { params.p63 } else { 3e-8 };
                validate_parameter("LOVER", params.p65, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p66 = if (params.p34 != 0.0) { params.p63 } else { params.p65 };
                validate_parameter("LOVERS", params.p66, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 2] = [
                1e-6, 1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(67), 2);
            {
                let params = &mut *ptr;
                params.p69 = if (params.p34 != 0.0) { params.p67 } else { 0.0 };
                validate_parameter("LDRIFT1S", params.p69, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p70 = if (params.p34 != 0.0) { params.p68 } else { 1e-6 };
                validate_parameter("LDRIFT2S", params.p70, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p71 = (params.p69 + params.p70);
                validate_parameter("LDRIFTS", params.p71, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 3] = [
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(72), 3);
            {
                let params = &mut *ptr;
                params.p75 = if (params.p34 != 0.0) { params.p74 } else { 0.0 };
                validate_parameter("RS", params.p75, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 20] = [
                5e17, 0.3, 1.0, 0.0, 0.1, 1.0, 0.07, 0.005,
                0.0, 0.0, 0.0, 1.0, 2.51, 10000000.0, 0.0, 0.0,
                9.025e-5, 1e-7, 1.1785, 7e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(76), 20);
            {
                let params = &mut *ptr;
                params.p96 = params.p95;
                validate_finite_parameter("TOXB", params.p96).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 11] = [
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(97), 11);
            {
                let params = &mut *ptr;
                params.p108 = if (((params.p88 * 10.0) % 10.0) < 3.0) { 0.0 } else { 10.0 };
                validate_parameter("DDLTSLP", params.p108, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p109 = if (((params.p88 * 10.0) % 10.0) < 3.0) { 10.0 } else { 0.0 };
                validate_finite_parameter("DDLTICT", params.p109).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 2] = [
                -0.5, 3e16,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(110), 2);
            {
                let params = &mut *ptr;
                params.p112 = if (params.p34 != 0.0) { params.p111 } else { 1e17 };
                validate_parameter("NOVERS", params.p112, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 2] = [
                5.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(113), 2);
            {
                let params = &mut *ptr;
                params.p115 = params.p114;
                validate_finite_parameter("XWDC", params.p115).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 15] = [
                0.0, 0.0, 1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(116), 15);
            {
                let params = &mut *ptr;
                params.p131 = params.p130;
                validate_parameter("RSHS", params.p131, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 4] = [
                0.0, 0.0, 0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(132), 4);
            {
                let params = &mut *ptr;
                params.p136 = if (params.p42 != 0.0) { (-0.2) } else { (-1.0) };
                validate_finite_parameter("VFBC", params.p136).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 1] = [
                1.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(137), 1);
            {
                let params = &mut *ptr;
                params.p138 = if (params.p42 != 0.0) { 5e16 } else { 3e17 };
                validate_parameter("NSUBC", params.p138, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(139), 1);
            {
                let params = &mut *ptr;
                params.p140 = if (params.p42 != 0.0) { 0.0 } else { 1.5e-8 };
                validate_parameter("LP", params.p140, false, Some((0.0, "0.0")), false, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p141 = if (params.p42 != 0.0) { 1e17 } else { 1e18 };
                validate_parameter("NSUBP", params.p141, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 19] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.5,
                1000.0, 100.0, 0.3,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(142), 19);
            {
                let params = &mut *ptr;
                params.p161 = if (params.p87 > 0.0) { 20000.0 } else { 9000.0 };
                validate_parameter("MUEPH1", params.p161, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 10] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(162), 10);
            {
                let params = &mut *ptr;
                params.p172 = if (params.p42 != 0.0) { 5000000000000000.0 } else { 600000000000000.0 };
                validate_parameter("MUESR1", params.p172, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 5] = [
                0.0, 0.0, 1.0, 1.0, 1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(173), 5);
            {
                let params = &mut *ptr;
                params.p178 = if (params.p87 > 0.0) { 2.0 } else { 1.0 };
                validate_parameter("BB", params.p178, false, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 8] = [
                10.0, 25.0, 0.8, 0.5, 0.0, 1.0, 0.8, 3e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(179), 8);
            {
                let params = &mut *ptr;
                params.p187 = params.p179;
                validate_finite_parameter("SUB1SNP", params.p187).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p188 = (0.6 * params.p180);
                validate_finite_parameter("SUB2SNP", params.p188).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p189 = params.p185;
                validate_finite_parameter("SVDSSNP", params.p189).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 79] = [
                0.0025, 1.0, 2e-6, 0.0, 50.0, 0.00017, 0.0, 0.012,
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 5e17, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1e-50, 0.0,
                0.0, 0.0, 0.9, 2e-7, 0.05, 2.0, 1.0, 1.0,
                0.0, 0.3, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 2.0, 30000000.0, 0.9, 0.0, 0.2, 50.0,
                10000000.0, 0.06, 4.0, 7500.0, 0.25, 1e-6, 0.5, 1e-15,
                1000.0, -1000.0, 5e-16, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.01, 0.005, 10000000000.0, 1e-19, 0.0, 3.9, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(190), 79);
            {
                let params = &mut *ptr;
                params.p269 = if (params.p34 != 0.0) { params.p268 } else { 0.0 };
                validate_parameter("CGSO", params.p269, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 25] = [
                1e-10, 0.7, 8e-7, 8e-5, 27.0, 2.1e-7, 0.6, 1e-12,
                0.0, 0.0, -1.0, 0.0, -0.3, 0.0, 3.5, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 100.0, 1e-7,
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(270), 25);
            {
                let params = &mut *ptr;
                params.p295 = params.p114;
                validate_finite_parameter("XWDLD", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 44] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0, -10.5, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.3, 1e-6, 0.7,
                1000000000000000.0, 0.1, 0.8, 0.4,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(296), 44);
            {
                let params = &mut *ptr;
                params.p340 = if (params.p42 < 3.0) { 1e17 } else { 4e16 };
                validate_parameter("NDEPM", params.p340, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(341), 2);
            {
                let params = &mut *ptr;
                params.p343 = if (params.p42 < 3.0) { 2.0000000000000002e-7 } else { 3.0000000000000004e-7 };
                validate_parameter("TNDEP", params.p343, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p344 = (params.p343 * 1e-6);
                validate_finite_parameter("TNDEPMIN", params.p344).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(345), 1);
            {
                let params = &mut *ptr;
                params.p346 = if (params.p42 < 3.0) { 1000.0 } else { 100000000.0 };
                validate_finite_parameter("DEPMUE0", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(347), 2);
            {
                let params = &mut *ptr;
                params.p349 = if (params.p42 < 3.0) { 0.0 } else { 100.0 };
                validate_finite_parameter("DEPMUE1", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 10] = [
                0.0, 1.0, 1000.0, 0.0, 100.0, 0.0, 0.0, 1.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(350), 10);
            {
                let params = &mut *ptr;
                params.p360 = if (params.p42 < 3.0) { 0.5 } else { 0.1 };
                validate_finite_parameter("DEPLEAK", params.p360).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 6] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(361), 6);
            {
                let params = &mut *ptr;
                params.p367 = if (params.p42 < 3.0) { 30000000.0 } else { 10000000.0 };
                validate_parameter("DEPVMAX", params.p367, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 8] = [
                0.0, 1.0, 2.0, 0.5, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(368), 8);
            {
                let params = &mut *ptr;
                params.p376 = if (params.p42 < 3.0) { 0.3 } else { 0.0 };
                validate_finite_parameter("DEPMUEPH0", params.p376).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p377 = if (params.p42 < 3.0) { 5000.0 } else { 400.0 };
                validate_parameter("DEPMUEPH1", params.p377, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p378 = if (params.p42 < 3.0) { 1.0 } else { 2.0 };
                validate_parameter("DEPBB", params.p378, false, Some((0.01, "0.01")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 4] = [
                0.0, 1.5, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(379), 4);
            {
                let params = &mut *ptr;
                params.p383 = if (params.p42 < 3.0) { 3.0 } else { 1.0 };
                validate_parameter("DEPDDLT", params.p383, false, Some((0.1, "0.1")), false, Some((20.0, "20.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 8] = [
                100.0, 10.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(384), 8);
            {
                let params = &mut *ptr;
                params.p392 = params.p136;
                validate_finite_parameter("DEPVFBC", params.p392).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 2] = [
                0.1, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(393), 2);
            {
                let params = &mut *ptr;
                params.p395 = params.p394;
                validate_parameter("DEPSUBSL0", params.p395, false, Some((1e-8, "1e-8")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 11] = [
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.01,
                0.01, 0.05, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(396), 11);
            {
                let params = &mut *ptr;
                params.p407 = if (params.p42 < 3.0) { 0.0 } else { 0.2 };
                validate_parameter("DEPVGPSL", params.p407, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 5] = [
                0.5, 1000.0, 0.0, 0.0, 30000000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(408), 5);
            {
                let params = &mut *ptr;
                params.p413 = params.p409;
                validate_parameter("RDRMUES", params.p413, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p414 = params.p412;
                validate_parameter("RDRVMAXS", params.p414, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(415), 1);
            {
                let params = &mut *ptr;
                params.p416 = params.p415;
                validate_finite_parameter("RDRMUETMPS", params.p416).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(417), 1);
            {
                let params = &mut *ptr;
                params.p418 = params.p417;
                validate_finite_parameter("RDRVTMPS", params.p418).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 15] = [
                1e-6, 0.0, 1e-8, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 100000.0, 0.0, 0.0, 500.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(419), 15);
            {
                let params = &mut *ptr;
                params.p434 = ((-100.0) * params.p87);
                validate_finite_parameter("VGSMIN", params.p434).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(435), 2);
            {
                let params = &mut *ptr;
                params.p437 = params.p436;
                validate_parameter("RDRBBS", params.p437, false, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(438), 1);
            {
                let params = &mut *ptr;
                params.p439 = params.p438;
                validate_finite_parameter("RDRBBTMPS", params.p439).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 53] = [
                1.0, 1.0, 1.0, 0.0, 0.0001, 0.0, 0.0, 100.0,
                0.0, 1.0, 0.0, 0.0, 3e-8, 1e20, 0.0, 0.0,
                0.0, 0.0, 5e-7, 0.0, 0.0, 1.0, 1.0, 1.0,
                2.0, 0.0005, 5e-10, 5e-10, 0.5, 0.33, 0.33, 1.0,
                1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(440), 53);
            {
                let params = &mut *ptr;
                params.p493 = params.p458;
                validate_finite_parameter("JS0D", params.p493).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p494 = params.p459;
                validate_finite_parameter("JS0SWD", params.p494).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p495 = params.p460;
                validate_finite_parameter("JS0SWGD", params.p495).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p496 = params.p461;
                validate_parameter("NJD", params.p496, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p497 = params.p462;
                validate_parameter("NJSWD", params.p497, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p498 = params.p463;
                validate_parameter("NJSWGD", params.p498, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p499 = params.p464;
                validate_finite_parameter("XTID", params.p499).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p500 = params.p465;
                validate_finite_parameter("CJD", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p501 = params.p466;
                validate_finite_parameter("CJSWD", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p502 = params.p467;
                validate_finite_parameter("CJSWGD", params.p502).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p503 = params.p468;
                validate_parameter("MJD", params.p503, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p504 = params.p469;
                validate_parameter("MJSWD", params.p504, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p505 = params.p470;
                validate_parameter("MJSWGD", params.p505, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p506 = params.p471;
                validate_parameter("PBD", params.p506, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p507 = params.p472;
                validate_parameter("PBSWD", params.p507, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p508 = params.p473;
                validate_parameter("PBSWGD", params.p508, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p509 = params.p474;
                validate_finite_parameter("XTI2D", params.p509).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p510 = params.p475;
                validate_finite_parameter("CISBD", params.p510).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p511 = params.p476;
                validate_finite_parameter("CVBD", params.p511).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p512 = params.p477;
                validate_finite_parameter("CTEMPD", params.p512).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p513 = params.p478;
                validate_finite_parameter("CISBKD", params.p513).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p514 = params.p479;
                validate_finite_parameter("DIVXD", params.p514).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p515 = params.p480;
                validate_finite_parameter("VDIFFJD", params.p515).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p516 = params.p493;
                validate_finite_parameter("JS0S", params.p516).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p517 = params.p494;
                validate_finite_parameter("JS0SWS", params.p517).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p518 = params.p495;
                validate_finite_parameter("JS0SWGS", params.p518).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p519 = params.p496;
                validate_parameter("NJS", params.p519, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p520 = params.p497;
                validate_parameter("NJSWS", params.p520, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p521 = params.p498;
                validate_parameter("NJSWGS", params.p521, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p522 = params.p499;
                validate_finite_parameter("XTIS", params.p522).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p523 = params.p500;
                validate_finite_parameter("CJS", params.p523).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p524 = params.p501;
                validate_finite_parameter("CJSWS", params.p524).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p525 = params.p502;
                validate_finite_parameter("CJSWGS", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p526 = params.p503;
                validate_parameter("MJS", params.p526, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p527 = params.p504;
                validate_parameter("MJSWS", params.p527, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p528 = params.p505;
                validate_parameter("MJSWGS", params.p528, false, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p529 = params.p506;
                validate_parameter("PBS", params.p529, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p530 = params.p507;
                validate_parameter("PBSWS", params.p530, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p531 = params.p508;
                validate_parameter("PBSWGS", params.p531, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p532 = params.p509;
                validate_finite_parameter("XTI2S", params.p532).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p533 = params.p510;
                validate_finite_parameter("CISBS", params.p533).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p534 = params.p511;
                validate_finite_parameter("CVBS", params.p534).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p535 = params.p512;
                validate_finite_parameter("CTEMPS", params.p535).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p536 = params.p513;
                validate_finite_parameter("CISBKS", params.p536).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p537 = params.p514;
                validate_finite_parameter("DIVXS", params.p537).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p538 = params.p515;
                validate_finite_parameter("VDIFFJS", params.p538).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 48] = [
                0.0, 1e16, 1.0, 10.0, 5e-9, 2e-7, 5e-6, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(539), 48);
            {
                let params = &mut *ptr;
                params.p587 = params.p582;
                validate_finite_parameter("LSUB1SNP", params.p587).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p588 = params.p583;
                validate_finite_parameter("LSUB2SNP", params.p588).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p589 = params.p584;
                validate_finite_parameter("LSVDSSNP", params.p589).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 85] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(590), 85);
            {
                let params = &mut *ptr;
                params.p675 = params.p670;
                validate_finite_parameter("WSUB1SNP", params.p675).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p676 = params.p671;
                validate_finite_parameter("WSUB2SNP", params.p676).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p677 = params.p672;
                validate_finite_parameter("WSVDSSNP", params.p677).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 85] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(678), 85);
            {
                let params = &mut *ptr;
                params.p763 = params.p758;
                validate_finite_parameter("PSUB1SNP", params.p763).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p764 = params.p759;
                validate_finite_parameter("PSUB2SNP", params.p764).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p765 = params.p760;
                validate_finite_parameter("PSVDSSNP", params.p765).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 58] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(766), 58);
            {
                let params = &mut *ptr;
                params.p824 = params.p819;
                validate_finite_parameter("LJS0D", params.p824).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p825 = params.p820;
                validate_finite_parameter("LJS0SWD", params.p825).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p826 = params.p821;
                validate_finite_parameter("LNJD", params.p826).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p827 = params.p822;
                validate_finite_parameter("LCISBKD", params.p827).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p828 = params.p823;
                validate_finite_parameter("LVDIFFJD", params.p828).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p829 = params.p824;
                validate_finite_parameter("LJS0S", params.p829).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p830 = params.p825;
                validate_finite_parameter("LJS0SWS", params.p830).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p831 = params.p826;
                validate_finite_parameter("LNJS", params.p831).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p832 = params.p827;
                validate_finite_parameter("LCISBKS", params.p832).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p833 = params.p828;
                validate_finite_parameter("LVDIFFJS", params.p833).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(834), 5);
            {
                let params = &mut *ptr;
                params.p839 = params.p834;
                validate_finite_parameter("WJS0D", params.p839).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p840 = params.p835;
                validate_finite_parameter("WJS0SWD", params.p840).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p841 = params.p836;
                validate_finite_parameter("WNJD", params.p841).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p842 = params.p837;
                validate_finite_parameter("WCISBKD", params.p842).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p843 = params.p838;
                validate_finite_parameter("WVDIFFJD", params.p843).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p844 = params.p839;
                validate_finite_parameter("WJS0S", params.p844).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p845 = params.p840;
                validate_finite_parameter("WJS0SWS", params.p845).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p846 = params.p841;
                validate_finite_parameter("WNJS", params.p846).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p847 = params.p842;
                validate_finite_parameter("WCISBKS", params.p847).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p848 = params.p843;
                validate_finite_parameter("WVDIFFJS", params.p848).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(849), 5);
            {
                let params = &mut *ptr;
                params.p854 = params.p849;
                validate_finite_parameter("PJS0D", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p855 = params.p850;
                validate_finite_parameter("PJS0SWD", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p856 = params.p851;
                validate_finite_parameter("PNJD", params.p856).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p857 = params.p852;
                validate_finite_parameter("PCISBKD", params.p857).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p858 = params.p853;
                validate_finite_parameter("PVDIFFJD", params.p858).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p859 = params.p854;
                validate_finite_parameter("PJS0S", params.p859).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p860 = params.p855;
                validate_finite_parameter("PJS0SWS", params.p860).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p861 = params.p856;
                validate_finite_parameter("PNJS", params.p861).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p862 = params.p857;
                validate_finite_parameter("PCISBKS", params.p862).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p863 = params.p858;
                validate_finite_parameter("PVDIFFJS", params.p863).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            boxed.assume_init()
        }
    }
}

impl Default for Parameters {
    fn default() -> Self {
        *Self::new_box()
    }
}

#[derive(Copy, Clone)]
struct ParameterBound {
    value: f64,
    label: &'static str,
}

const PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
const PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

fn validate_parameter_metadata(index: usize, value: f64) -> Result<(), String> {
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    if let Some(min) = PARAMETER_MIN_BOUNDS[index] {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = PARAMETER_MAX_BOUNDS[index] {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in PARAMETER_EXCLUDED_BOUNDS[index] {
        if value == excluded.value {
            return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
        }
    }
    Ok(())
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
    integer: bool,
    min: Option<(f64, &str)>,
    min_exclusive: bool,
    max: Option<(f64, &str)>,
    max_exclusive: bool,
    excluded: &[(f64, &str)],
) -> Result<(), String> {
    validate_finite_parameter(name, value)?;
    if integer && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
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
const PARAMETER_NAME_LOOKUP: [(&str, usize); 864] = [
    ("l", 0), ("w", 1), ("nrd", 2), ("nrs", 3), ("ngcon", 4), ("xgw", 5), ("xgl", 6), ("nf", 7), ("sa", 8), ("sb", 9), ("sd", 10), ("dtemp", 11), ("nsubcdfm", 12), ("ad", 13), ("as", 14), ("pd", 15),
    ("ps", 16), ("corsrd", 17), ("cors", 18), ("cord", 19), ("coiprv", 20), ("copprv", 21), ("coadov", 22), ("coisub", 23), ("coiigs", 24), ("cogidl", 25), ("coflick", 26), ("coisti", 27), ("conqs", 28), ("conqsov", 29), ("cothrml", 30), ("coign", 31),
    ("codfm", 32), ("coqovsm", 33), ("cosym", 34), ("coovlp", 35), ("coovlps", 36), ("covbscl", 37), ("coqovcl", 38), ("cotemp", 39), ("cordrift", 40), ("coerrrep", 41), ("codep", 42), ("covdsres", 43), ("coddlt", 44), ("cohbd", 45), ("cosnp", 46), ("info", 47),
    ("codio", 48), ("cofixcss", 49), ("coovjunc", 50), ("corg", 51), ("corbnet", 52), ("coselfheat", 53), ("cosubnode", 54), ("cotrench", 55), ("rbpb", 56), ("rbpd", 57), ("rbps", 58), ("rdov13", 59), ("rdslp1", 60), ("rdvg11", 61), ("rdict1", 62), ("loverld", 63),
    ("wtrench", 64), ("lover", 65), ("lovers", 66), ("ldrift1", 67), ("ldrift2", 68), ("ldrift1s", 69), ("ldrift2s", 70), ("ldrifts", 71), ("subld1", 72), ("subld2", 73), ("rd", 74), ("rs", 75), ("npext", 76), ("vover", 77), ("falph", 78), ("cgbo", 79),
    ("rth0", 80), ("powrat", 81), ("rdvd", 82), ("rd23", 83), ("rd24", 84), ("rdvb", 85), ("cvdsover", 86), ("type", 87), ("version", 88), ("vmax", 89), ("vmaxt1", 90), ("vmaxt2", 91), ("bgtmp1", 92), ("bgtmp2", 93), ("eg0", 94), ("tox", 95),
    ("toxb", 96), ("xld", 97), ("rdov11", 98), ("rdov12", 99), ("rdslp2", 100), ("rdict2", 101), ("subld1l", 102), ("subld1lp", 103), ("xpdv", 104), ("xpvdth", 105), ("xpvdthg", 106), ("ddltmax", 107), ("ddltslp", 108), ("ddltict", 109), ("vfbover", 110), ("nover", 111),
    ("novers", 112), ("olmdlt", 113), ("xwd", 114), ("xwdc", 115), ("xl", 116), ("xw", 117), ("saref", 118), ("sbref", 119), ("ll", 120), ("lld", 121), ("lln", 122), ("wl", 123), ("wl1", 124), ("wl1p", 125), ("wl2", 126), ("wl2p", 127),
    ("wld", 128), ("wln", 129), ("rsh", 130), ("rshs", 131), ("rshg", 132), ("xqy", 133), ("xqy1", 134), ("xqy2", 135), ("vfbc", 136), ("vbi", 137), ("nsubc", 138), ("parl2", 139), ("lp", 140), ("nsubp", 141), ("nsubp0", 142), ("nsubwp", 143),
    ("scp1", 144), ("scp2", 145), ("scp3", 146), ("sc1", 147), ("sc2", 148), ("sc3", 149), ("sc4", 150), ("pgd1", 151), ("pgd2", 152), ("pgd4", 153), ("ndep", 154), ("ndepl", 155), ("ndeplp", 156), ("ninv", 157), ("muecb0", 158), ("muecb1", 159),
    ("mueph0", 160), ("mueph1", 161), ("muephw", 162), ("muepwp", 163), ("muephl", 164), ("mueplp", 165), ("mueefb", 166), ("muephs", 167), ("muepsp", 168), ("vtmp", 169), ("wvth0", 170), ("muesr0", 171), ("muesr1", 172), ("muesrl", 173), ("muesrw", 174), ("mueswp", 175),
    ("mueslp", 176), ("muetmp", 177), ("bb", 178), ("sub1", 179), ("sub2", 180), ("svgs", 181), ("svbs", 182), ("svbsl", 183), ("svbslp", 184), ("svds", 185), ("slg", 186), ("sub1snp", 187), ("sub2snp", 188), ("svdssnp", 189), ("sub1l", 190), ("sub1lp", 191),
    ("sub2l", 192), ("subtmp", 193), ("fn1", 194), ("fn2", 195), ("fn3", 196), ("fvbs", 197), ("svgsl", 198), ("svgslp", 199), ("svgsw", 200), ("svgswp", 201), ("slgl", 202), ("slglp", 203), ("nsti", 204), ("wsti", 205), ("wstil", 206), ("wstilp", 207),
    ("wstiw", 208), ("wstiwp", 209), ("scsti1", 210), ("scsti2", 211), ("vthsti", 212), ("vdsti", 213), ("muesti1", 214), ("muesti2", 215), ("muesti3", 216), ("nsubpsti1", 217), ("nsubpsti2", 218), ("nsubpsti3", 219), ("lpext", 220), ("scp21", 221), ("scp22", 222), ("bs1", 223),
    ("bs2", 224), ("tpoly", 225), ("clm1", 226), ("clm2", 227), ("clm3", 228), ("clm5", 229), ("clm6", 230), ("voverp", 231), ("wfc", 232), ("nsubcw", 233), ("nsubcwp", 234), ("qme1", 235), ("qme2", 236), ("qme3", 237), ("vovers", 238), ("voversp", 239),
    ("gidl1", 240), ("gidl2", 241), ("gidl3", 242), ("gidl4", 243), ("gidl5", 244), ("gleak1", 245), ("gleak2", 246), ("gleak3", 247), ("gleak4", 248), ("gleak5", 249), ("gleak6", 250), ("gleak7", 251), ("glpart1", 252), ("glksd1", 253), ("glksd2", 254), ("glksd3", 255),
    ("glkb1", 256), ("glkb2", 257), ("glkb3", 258), ("egig", 259), ("igtemp2", 260), ("igtemp3", 261), ("vzadd0", 262), ("pzadd0", 263), ("nftrp", 264), ("nfalp", 265), ("cit", 266), ("kappa", 267), ("cgdo", 268), ("cgso", 269), ("dly1", 270), ("dly2", 271),
    ("dly3", 272), ("dlyov", 273), ("tnom", 274), ("ovslp", 275), ("ovmag", 276), ("gbmin", 277), ("ibpc1", 278), ("ibpc1l", 279), ("ibpc1lp", 280), ("ibpc2", 281), ("mphdfm", 282), ("ptl", 283), ("ptp", 284), ("pt2", 285), ("ptlp", 286), ("gdl", 287),
    ("gdlp", 288), ("gdld", 289), ("pt4", 290), ("pt4p", 291), ("rdvg12", 292), ("cth0", 293), ("xldld", 294), ("xwdld", 295), ("rd20", 296), ("rd21", 297), ("rd22", 298), ("rd22d", 299), ("rd25", 300), ("rdvdl", 301), ("rdvdlp", 302), ("rdvds", 303),
    ("rdvdsp", 304), ("rd23l", 305), ("rd23lp", 306), ("rd23s", 307), ("rd23sp", 308), ("rds", 309), ("rdsp", 310), ("rdtemp1", 311), ("rdtemp2", 312), ("rdvdtemp1", 313), ("rdvdtemp2", 314), ("rth0w", 315), ("rth0wp", 316), ("rth0l", 317), ("rth0lp", 318), ("ninvd", 319),
    ("ninvdl", 320), ("ninvdlp", 321), ("ninvdw", 322), ("ninvdwp", 323), ("ninvdt1", 324), ("ninvdt2", 325), ("vbsmin", 326), ("rth0nf", 327), ("rthtemp1", 328), ("rthtemp2", 329), ("prattemp1", 330), ("prattemp2", 331), ("rdvsub", 332), ("rdvdsub", 333), ("ddrift", 334), ("vbisub", 335),
    ("nsubsub", 336), ("shemaxdlt", 337), ("vbfwdmx", 338), ("vbfwdbnd", 339), ("ndepm", 340), ("ndepml", 341), ("ndepmlp", 342), ("tndep", 343), ("tndepmin", 344), ("tndepv", 345), ("depmue0", 346), ("depmue0l", 347), ("depmue0lp", 348), ("depmue1", 349), ("depmue1l", 350), ("depmue1lp", 351),
    ("depmue2", 352), ("depmuea1", 353), ("depmueback0", 354), ("depmueback1", 355), ("depmueback0l", 356), ("depmueback0lp", 357), ("depmueback1l", 358), ("depmueback1lp", 359), ("depleak", 360), ("depleakl", 361), ("depleaklp", 362), ("depjleak", 363), ("depwlp", 364), ("depwlpt", 365), ("depeta", 366), ("depvmax", 367),
    ("depvmaxl", 368), ("depvmaxlp", 369), ("depvdsef1", 370), ("depvdsef2", 371), ("depvdsef1l", 372), ("depvdsef1lp", 373), ("depvdsef2l", 374), ("depvdsef2lp", 375), ("depmueph0", 376), ("depmueph1", 377), ("depbb", 378), ("depvtmp", 379), ("depmuetmp", 380), ("depmue0tmp", 381), ("depmue2tmp", 382), ("depddlt", 383),
    ("depninvdc", 384), ("depninvdh", 385), ("depninvdl", 386), ("depninvdlp", 387), ("depninvdw", 388), ("depninvdwp", 389), ("depninvdt1", 390), ("depninvdt2", 391), ("depvfbc", 392), ("depdvfbc", 393), ("depsubsl", 394), ("depsubsl0", 395), ("depvsatr", 396), ("depvsata", 397), ("deprbr", 398), ("depvleak", 399),
    ("depcar", 400), ("deprdrdl1", 401), ("deprdrdl2", 402), ("depps", 403), ("depqf", 404), ("depqfres", 405), ("depfdpd", 406), ("depvgpsl", 407), ("deppb0", 408), ("rdrmue", 409), ("rdrmuebs1", 410), ("rdrmuebs2", 411), ("rdrvmax", 412), ("rdrmues", 413), ("rdrvmaxs", 414), ("rdrmuetmp", 415),
    ("rdrmuetmps", 416), ("rdrvtmp", 417), ("rdrvtmps", 418), ("rdrdjunc", 419), ("rdrcx", 420), ("rdrcar", 421), ("rdrdl1", 422), ("rdrdl2", 423), ("rdrvmaxw", 424), ("rdrvmaxwp", 425), ("rdrvmaxl", 426), ("rdrvmaxlp", 427), ("rdrmuel", 428), ("rdrmuelp", 429), ("rdrqover", 430), ("qovadd", 431),
    ("qovjunc", 432), ("shemax", 433), ("vgsmin", 434), ("gdsleak", 435), ("rdrbb", 436), ("rdrbbs", 437), ("rdrbbtmp", 438), ("rdrbbtmps", 439), ("ndrilim", 440), ("ndridlt", 441), ("ndripw", 442), ("gmin", 443), ("rmin", 444), ("hbda", 445), ("hbdb", 446), ("hbdc", 447),
    ("hbdctmp", 448), ("hbdf", 449), ("copt", 450), ("copspt", 451), ("xjpt", 452), ("njunc", 453), ("mupt", 454), ("vfbpt", 455), ("pslimpt", 456), ("ps0pt", 457), ("js0", 458), ("js0sw", 459), ("js0swg", 460), ("nj", 461), ("njsw", 462), ("njswg", 463),
    ("xti", 464), ("cj", 465), ("cjsw", 466), ("cjswg", 467), ("mj", 468), ("mjsw", 469), ("mjswg", 470), ("pb", 471), ("pbsw", 472), ("pbswg", 473), ("xti2", 474), ("cisb", 475), ("cvb", 476), ("ctemp", 477), ("cisbk", 478), ("divx", 479),
    ("vdiffj", 480), ("tcjbd", 481), ("tcjbs", 482), ("tcjbdsw", 483), ("tcjbssw", 484), ("tcjbdswg", 485), ("tcjbsswg", 486), ("tpbbd", 487), ("tpbbs", 488), ("tpbbdsw", 489), ("tpbbssw", 490), ("tpbbdswg", 491), ("tpbbsswg", 492), ("js0d", 493), ("js0swd", 494), ("js0swgd", 495),
    ("njd", 496), ("njswd", 497), ("njswgd", 498), ("xtid", 499), ("cjd", 500), ("cjswd", 501), ("cjswgd", 502), ("mjd", 503), ("mjswd", 504), ("mjswgd", 505), ("pbd", 506), ("pbswd", 507), ("pbswgd", 508), ("xti2d", 509), ("cisbd", 510), ("cvbd", 511),
    ("ctempd", 512), ("cisbkd", 513), ("divxd", 514), ("vdiffjd", 515), ("js0s", 516), ("js0sws", 517), ("js0swgs", 518), ("njs", 519), ("njsws", 520), ("njswgs", 521), ("xtis", 522), ("cjs", 523), ("cjsws", 524), ("cjswgs", 525), ("mjs", 526), ("mjsws", 527),
    ("mjswgs", 528), ("pbs", 529), ("pbsws", 530), ("pbswgs", 531), ("xti2s", 532), ("cisbs", 533), ("cvbs", 534), ("ctemps", 535), ("cisbks", 536), ("divxs", 537), ("vdiffjs", 538), ("corecovery", 539), ("ndibot", 540), ("inj1", 541), ("inj2", 542), ("nqs", 543),
    ("tau", 544), ("wi", 545), ("depnqs", 546), ("taut", 547), ("injt", 548), ("lmin", 549), ("lmax", 550), ("wmin", 551), ("wmax", 552), ("lbinn", 553), ("wbinn", 554), ("lvmax", 555), ("lbgtmp1", 556), ("lbgtmp2", 557), ("leg0", 558), ("lvfbover", 559),
    ("lnover", 560), ("lnovers", 561), ("lwl2", 562), ("lvfbc", 563), ("lnsubc", 564), ("lnsubp", 565), ("lscp1", 566), ("lscp2", 567), ("lscp3", 568), ("lsc1", 569), ("lsc2", 570), ("lsc3", 571), ("lpgd1", 572), ("lndep", 573), ("lninv", 574), ("lmuecb0", 575),
    ("lmuecb1", 576), ("lmueph1", 577), ("lvtmp", 578), ("lwvth0", 579), ("lmuesr1", 580), ("lmuetmp", 581), ("lsub1", 582), ("lsub2", 583), ("lsvds", 584), ("lsvbs", 585), ("lsvgs", 586), ("lsub1snp", 587), ("lsub2snp", 588), ("lsvdssnp", 589), ("lfn1", 590), ("lfn2", 591),
    ("lfn3", 592), ("lfvbs", 593), ("lnsti", 594), ("lwsti", 595), ("lscsti1", 596), ("lscsti2", 597), ("lvthsti", 598), ("lmuesti1", 599), ("lmuesti2", 600), ("lmuesti3", 601), ("lnsubpsti1", 602), ("lnsubpsti2", 603), ("lnsubpsti3", 604), ("lcgso", 605), ("lcgdo", 606), ("lclm1", 607),
    ("lclm2", 608), ("lclm3", 609), ("lwfc", 610), ("lgidl1", 611), ("lgidl2", 612), ("lgleak1", 613), ("lgleak2", 614), ("lgleak3", 615), ("lgleak6", 616), ("lglksd1", 617), ("lglksd2", 618), ("lglkb1", 619), ("lglkb2", 620), ("lnftrp", 621), ("lnfalp", 622), ("libpc1", 623),
    ("libpc2", 624), ("lcgbo", 625), ("lcvdsover", 626), ("lfalph", 627), ("lnpext", 628), ("lpowrat", 629), ("lrd", 630), ("lrd22", 631), ("lrd23", 632), ("lrd24", 633), ("lrdict1", 634), ("lrdov13", 635), ("lrdslp1", 636), ("lrdvb", 637), ("lrdvd", 638), ("lrdvg11", 639),
    ("lrs", 640), ("lrth0", 641), ("lvover", 642), ("wvmax", 643), ("wbgtmp1", 644), ("wbgtmp2", 645), ("weg0", 646), ("wvfbover", 647), ("wnover", 648), ("wnovers", 649), ("wwl2", 650), ("wvfbc", 651), ("wnsubc", 652), ("wnsubp", 653), ("wscp1", 654), ("wscp2", 655),
    ("wscp3", 656), ("wsc1", 657), ("wsc2", 658), ("wsc3", 659), ("wpgd1", 660), ("wndep", 661), ("wninv", 662), ("wmuecb0", 663), ("wmuecb1", 664), ("wmueph1", 665), ("wvtmp", 666), ("wwvth0", 667), ("wmuesr1", 668), ("wmuetmp", 669), ("wsub1", 670), ("wsub2", 671),
    ("wsvds", 672), ("wsvbs", 673), ("wsvgs", 674), ("wsub1snp", 675), ("wsub2snp", 676), ("wsvdssnp", 677), ("wfn1", 678), ("wfn2", 679), ("wfn3", 680), ("wfvbs", 681), ("wnsti", 682), ("wwsti", 683), ("wscsti1", 684), ("wscsti2", 685), ("wvthsti", 686), ("wmuesti1", 687),
    ("wmuesti2", 688), ("wmuesti3", 689), ("wnsubpsti1", 690), ("wnsubpsti2", 691), ("wnsubpsti3", 692), ("wcgso", 693), ("wcgdo", 694), ("wclm1", 695), ("wclm2", 696), ("wclm3", 697), ("wwfc", 698), ("wgidl1", 699), ("wgidl2", 700), ("wgleak1", 701), ("wgleak2", 702), ("wgleak3", 703),
    ("wgleak6", 704), ("wglksd1", 705), ("wglksd2", 706), ("wglkb1", 707), ("wglkb2", 708), ("wnftrp", 709), ("wnfalp", 710), ("wibpc1", 711), ("wibpc2", 712), ("wcgbo", 713), ("wcvdsover", 714), ("wfalph", 715), ("wnpext", 716), ("wpowrat", 717), ("wrd", 718), ("wrd22", 719),
    ("wrd23", 720), ("wrd24", 721), ("wrdict1", 722), ("wrdov13", 723), ("wrdslp1", 724), ("wrdvb", 725), ("wrdvd", 726), ("wrdvg11", 727), ("wrs", 728), ("wrth0", 729), ("wvover", 730), ("pvmax", 731), ("pbgtmp1", 732), ("pbgtmp2", 733), ("peg0", 734), ("pvfbover", 735),
    ("pnover", 736), ("pnovers", 737), ("pwl2", 738), ("pvfbc", 739), ("pnsubc", 740), ("pnsubp", 741), ("pscp1", 742), ("pscp2", 743), ("pscp3", 744), ("psc1", 745), ("psc2", 746), ("psc3", 747), ("ppgd1", 748), ("pndep", 749), ("pninv", 750), ("pmuecb0", 751),
    ("pmuecb1", 752), ("pmueph1", 753), ("pvtmp", 754), ("pwvth0", 755), ("pmuesr1", 756), ("pmuetmp", 757), ("psub1", 758), ("psub2", 759), ("psvds", 760), ("psvbs", 761), ("psvgs", 762), ("psub1snp", 763), ("psub2snp", 764), ("psvdssnp", 765), ("pfn1", 766), ("pfn2", 767),
    ("pfn3", 768), ("pfvbs", 769), ("pnsti", 770), ("pwsti", 771), ("pscsti1", 772), ("pscsti2", 773), ("pvthsti", 774), ("pmuesti1", 775), ("pmuesti2", 776), ("pmuesti3", 777), ("pnsubpsti1", 778), ("pnsubpsti2", 779), ("pnsubpsti3", 780), ("pcgso", 781), ("pcgdo", 782), ("pclm1", 783),
    ("pclm2", 784), ("pclm3", 785), ("pwfc", 786), ("pgidl1", 787), ("pgidl2", 788), ("pgleak1", 789), ("pgleak2", 790), ("pgleak3", 791), ("pgleak6", 792), ("pglksd1", 793), ("pglksd2", 794), ("pglkb1", 795), ("pglkb2", 796), ("pnftrp", 797), ("pnfalp", 798), ("pibpc1", 799),
    ("pibpc2", 800), ("pcgbo", 801), ("pcvdsover", 802), ("pfalph", 803), ("pnpext", 804), ("ppowrat", 805), ("prd", 806), ("prd22", 807), ("prd23", 808), ("prd24", 809), ("prdict1", 810), ("prdov13", 811), ("prdslp1", 812), ("prdvb", 813), ("prdvd", 814), ("prdvg11", 815),
    ("prs", 816), ("prth0", 817), ("pvover", 818), ("ljs0", 819), ("ljs0sw", 820), ("lnj", 821), ("lcisbk", 822), ("lvdiffj", 823), ("ljs0d", 824), ("ljs0swd", 825), ("lnjd", 826), ("lcisbkd", 827), ("lvdiffjd", 828), ("ljs0s", 829), ("ljs0sws", 830), ("lnjs", 831),
    ("lcisbks", 832), ("lvdiffjs", 833), ("wjs0", 834), ("wjs0sw", 835), ("wnj", 836), ("wcisbk", 837), ("wvdiffj", 838), ("wjs0d", 839), ("wjs0swd", 840), ("wnjd", 841), ("wcisbkd", 842), ("wvdiffjd", 843), ("wjs0s", 844), ("wjs0sws", 845), ("wnjs", 846), ("wcisbks", 847),
    ("wvdiffjs", 848), ("pjs0", 849), ("pjs0sw", 850), ("pnj", 851), ("pcisbk", 852), ("pvdiffj", 853), ("pjs0d", 854), ("pjs0swd", 855), ("pnjd", 856), ("pcisbkd", 857), ("pvdiffjd", 858), ("pjs0s", 859), ("pjs0sws", 860), ("pnjs", 861), ("pcisbks", 862), ("pvdiffjs", 863),
];

const PARAMETER_DISPLAY_NAMES: [&str; 864] = [
    "L", "W", "NRD", "NRS", "NGCON", "XGW", "XGL", "NF", "SA", "SB", "SD", "DTEMP", "NSUBCDFM", "AD", "AS", "PD",
    "PS", "CORSRD", "CORS", "CORD", "COIPRV", "COPPRV", "COADOV", "COISUB", "COIIGS", "COGIDL", "COFLICK", "COISTI", "CONQS", "CONQSOV", "COTHRML", "COIGN",
    "CODFM", "COQOVSM", "COSYM", "COOVLP", "COOVLPS", "COVBSCL", "COQOVCL", "COTEMP", "CORDRIFT", "COERRREP", "CODEP", "COVDSRES", "CODDLT", "COHBD", "COSNP", "INFO",
    "CODIO", "COFIXCSS", "COOVJUNC", "CORG", "CORBNET", "COSELFHEAT", "COSUBNODE", "COTRENCH", "RBPB", "RBPD", "RBPS", "RDOV13", "RDSLP1", "RDVG11", "RDICT1", "LOVERLD",
    "WTRENCH", "LOVER", "LOVERS", "LDRIFT1", "LDRIFT2", "LDRIFT1S", "LDRIFT2S", "LDRIFTS", "SUBLD1", "SUBLD2", "RD", "RS", "NPEXT", "VOVER", "FALPH", "CGBO",
    "RTH0", "POWRAT", "RDVD", "RD23", "RD24", "RDVB", "CVDSOVER", "TYPE", "VERSION", "VMAX", "VMAXT1", "VMAXT2", "BGTMP1", "BGTMP2", "EG0", "TOX",
    "TOXB", "XLD", "RDOV11", "RDOV12", "RDSLP2", "RDICT2", "SUBLD1L", "SUBLD1LP", "XPDV", "XPVDTH", "XPVDTHG", "DDLTMAX", "DDLTSLP", "DDLTICT", "VFBOVER", "NOVER",
    "NOVERS", "OLMDLT", "XWD", "XWDC", "XL", "XW", "SAREF", "SBREF", "LL", "LLD", "LLN", "WL", "WL1", "WL1P", "WL2", "WL2P",
    "WLD", "WLN", "RSH", "RSHS", "RSHG", "XQY", "XQY1", "XQY2", "VFBC", "VBI", "NSUBC", "PARL2", "LP", "NSUBP", "NSUBP0", "NSUBWP",
    "SCP1", "SCP2", "SCP3", "SC1", "SC2", "SC3", "SC4", "PGD1", "PGD2", "PGD4", "NDEP", "NDEPL", "NDEPLP", "NINV", "MUECB0", "MUECB1",
    "MUEPH0", "MUEPH1", "MUEPHW", "MUEPWP", "MUEPHL", "MUEPLP", "MUEEFB", "MUEPHS", "MUEPSP", "VTMP", "WVTH0", "MUESR0", "MUESR1", "MUESRL", "MUESRW", "MUESWP",
    "MUESLP", "MUETMP", "BB", "SUB1", "SUB2", "SVGS", "SVBS", "SVBSL", "SVBSLP", "SVDS", "SLG", "SUB1SNP", "SUB2SNP", "SVDSSNP", "SUB1L", "SUB1LP",
    "SUB2L", "SUBTMP", "FN1", "FN2", "FN3", "FVBS", "SVGSL", "SVGSLP", "SVGSW", "SVGSWP", "SLGL", "SLGLP", "NSTI", "WSTI", "WSTIL", "WSTILP",
    "WSTIW", "WSTIWP", "SCSTI1", "SCSTI2", "VTHSTI", "VDSTI", "MUESTI1", "MUESTI2", "MUESTI3", "NSUBPSTI1", "NSUBPSTI2", "NSUBPSTI3", "LPEXT", "SCP21", "SCP22", "BS1",
    "BS2", "TPOLY", "CLM1", "CLM2", "CLM3", "CLM5", "CLM6", "VOVERP", "WFC", "NSUBCW", "NSUBCWP", "QME1", "QME2", "QME3", "VOVERS", "VOVERSP",
    "GIDL1", "GIDL2", "GIDL3", "GIDL4", "GIDL5", "GLEAK1", "GLEAK2", "GLEAK3", "GLEAK4", "GLEAK5", "GLEAK6", "GLEAK7", "GLPART1", "GLKSD1", "GLKSD2", "GLKSD3",
    "GLKB1", "GLKB2", "GLKB3", "EGIG", "IGTEMP2", "IGTEMP3", "VZADD0", "PZADD0", "NFTRP", "NFALP", "CIT", "KAPPA", "CGDO", "CGSO", "DLY1", "DLY2",
    "DLY3", "DLYOV", "TNOM", "OVSLP", "OVMAG", "GBMIN", "IBPC1", "IBPC1L", "IBPC1LP", "IBPC2", "MPHDFM", "PTL", "PTP", "PT2", "PTLP", "GDL",
    "GDLP", "GDLD", "PT4", "PT4P", "RDVG12", "CTH0", "XLDLD", "XWDLD", "RD20", "RD21", "RD22", "RD22D", "RD25", "RDVDL", "RDVDLP", "RDVDS",
    "RDVDSP", "RD23L", "RD23LP", "RD23S", "RD23SP", "RDS", "RDSP", "RDTEMP1", "RDTEMP2", "RDVDTEMP1", "RDVDTEMP2", "RTH0W", "RTH0WP", "RTH0L", "RTH0LP", "NINVD",
    "NINVDL", "NINVDLP", "NINVDW", "NINVDWP", "NINVDT1", "NINVDT2", "VBSMIN", "RTH0NF", "RTHTEMP1", "RTHTEMP2", "PRATTEMP1", "PRATTEMP2", "RDVSUB", "RDVDSUB", "DDRIFT", "VBISUB",
    "NSUBSUB", "SHEMAXDLT", "VBFWDMX", "VBFWDBND", "NDEPM", "NDEPML", "NDEPMLP", "TNDEP", "TNDEPMIN", "TNDEPV", "DEPMUE0", "DEPMUE0L", "DEPMUE0LP", "DEPMUE1", "DEPMUE1L", "DEPMUE1LP",
    "DEPMUE2", "DEPMUEA1", "DEPMUEBACK0", "DEPMUEBACK1", "DEPMUEBACK0L", "DEPMUEBACK0LP", "DEPMUEBACK1L", "DEPMUEBACK1LP", "DEPLEAK", "DEPLEAKL", "DEPLEAKLP", "DEPJLEAK", "DEPWLP", "DEPWLPT", "DEPETA", "DEPVMAX",
    "DEPVMAXL", "DEPVMAXLP", "DEPVDSEF1", "DEPVDSEF2", "DEPVDSEF1L", "DEPVDSEF1LP", "DEPVDSEF2L", "DEPVDSEF2LP", "DEPMUEPH0", "DEPMUEPH1", "DEPBB", "DEPVTMP", "DEPMUETMP", "DEPMUE0TMP", "DEPMUE2TMP", "DEPDDLT",
    "DEPNINVDC", "DEPNINVDH", "DEPNINVDL", "DEPNINVDLP", "DEPNINVDW", "DEPNINVDWP", "DEPNINVDT1", "DEPNINVDT2", "DEPVFBC", "DEPDVFBC", "DEPSUBSL", "DEPSUBSL0", "DEPVSATR", "DEPVSATA", "DEPRBR", "DEPVLEAK",
    "DEPCAR", "DEPRDRDL1", "DEPRDRDL2", "DEPPS", "DEPQF", "DEPQFRES", "DEPFDPD", "DEPVGPSL", "DEPPB0", "RDRMUE", "RDRMUEBS1", "RDRMUEBS2", "RDRVMAX", "RDRMUES", "RDRVMAXS", "RDRMUETMP",
    "RDRMUETMPS", "RDRVTMP", "RDRVTMPS", "RDRDJUNC", "RDRCX", "RDRCAR", "RDRDL1", "RDRDL2", "RDRVMAXW", "RDRVMAXWP", "RDRVMAXL", "RDRVMAXLP", "RDRMUEL", "RDRMUELP", "RDRQOVER", "QOVADD",
    "QOVJUNC", "SHEMAX", "VGSMIN", "GDSLEAK", "RDRBB", "RDRBBS", "RDRBBTMP", "RDRBBTMPS", "NDRILIM", "NDRIDLT", "NDRIPW", "GMIN", "RMIN", "HBDA", "HBDB", "HBDC",
    "HBDCTMP", "HBDF", "COPT", "COPSPT", "XJPT", "NJUNC", "MUPT", "VFBPT", "PSLIMPT", "PS0PT", "JS0", "JS0SW", "JS0SWG", "NJ", "NJSW", "NJSWG",
    "XTI", "CJ", "CJSW", "CJSWG", "MJ", "MJSW", "MJSWG", "PB", "PBSW", "PBSWG", "XTI2", "CISB", "CVB", "CTEMP", "CISBK", "DIVX",
    "VDIFFJ", "TCJBD", "TCJBS", "TCJBDSW", "TCJBSSW", "TCJBDSWG", "TCJBSSWG", "TPBBD", "TPBBS", "TPBBDSW", "TPBBSSW", "TPBBDSWG", "TPBBSSWG", "JS0D", "JS0SWD", "JS0SWGD",
    "NJD", "NJSWD", "NJSWGD", "XTID", "CJD", "CJSWD", "CJSWGD", "MJD", "MJSWD", "MJSWGD", "PBD", "PBSWD", "PBSWGD", "XTI2D", "CISBD", "CVBD",
    "CTEMPD", "CISBKD", "DIVXD", "VDIFFJD", "JS0S", "JS0SWS", "JS0SWGS", "NJS", "NJSWS", "NJSWGS", "XTIS", "CJS", "CJSWS", "CJSWGS", "MJS", "MJSWS",
    "MJSWGS", "PBS", "PBSWS", "PBSWGS", "XTI2S", "CISBS", "CVBS", "CTEMPS", "CISBKS", "DIVXS", "VDIFFJS", "CORECOVERY", "NDIBOT", "INJ1", "INJ2", "NQS",
    "TAU", "WI", "DEPNQS", "TAUT", "INJT", "LMIN", "LMAX", "WMIN", "WMAX", "LBINN", "WBINN", "LVMAX", "LBGTMP1", "LBGTMP2", "LEG0", "LVFBOVER",
    "LNOVER", "LNOVERS", "LWL2", "LVFBC", "LNSUBC", "LNSUBP", "LSCP1", "LSCP2", "LSCP3", "LSC1", "LSC2", "LSC3", "LPGD1", "LNDEP", "LNINV", "LMUECB0",
    "LMUECB1", "LMUEPH1", "LVTMP", "LWVTH0", "LMUESR1", "LMUETMP", "LSUB1", "LSUB2", "LSVDS", "LSVBS", "LSVGS", "LSUB1SNP", "LSUB2SNP", "LSVDSSNP", "LFN1", "LFN2",
    "LFN3", "LFVBS", "LNSTI", "LWSTI", "LSCSTI1", "LSCSTI2", "LVTHSTI", "LMUESTI1", "LMUESTI2", "LMUESTI3", "LNSUBPSTI1", "LNSUBPSTI2", "LNSUBPSTI3", "LCGSO", "LCGDO", "LCLM1",
    "LCLM2", "LCLM3", "LWFC", "LGIDL1", "LGIDL2", "LGLEAK1", "LGLEAK2", "LGLEAK3", "LGLEAK6", "LGLKSD1", "LGLKSD2", "LGLKB1", "LGLKB2", "LNFTRP", "LNFALP", "LIBPC1",
    "LIBPC2", "LCGBO", "LCVDSOVER", "LFALPH", "LNPEXT", "LPOWRAT", "LRD", "LRD22", "LRD23", "LRD24", "LRDICT1", "LRDOV13", "LRDSLP1", "LRDVB", "LRDVD", "LRDVG11",
    "LRS", "LRTH0", "LVOVER", "WVMAX", "WBGTMP1", "WBGTMP2", "WEG0", "WVFBOVER", "WNOVER", "WNOVERS", "WWL2", "WVFBC", "WNSUBC", "WNSUBP", "WSCP1", "WSCP2",
    "WSCP3", "WSC1", "WSC2", "WSC3", "WPGD1", "WNDEP", "WNINV", "WMUECB0", "WMUECB1", "WMUEPH1", "WVTMP", "WWVTH0", "WMUESR1", "WMUETMP", "WSUB1", "WSUB2",
    "WSVDS", "WSVBS", "WSVGS", "WSUB1SNP", "WSUB2SNP", "WSVDSSNP", "WFN1", "WFN2", "WFN3", "WFVBS", "WNSTI", "WWSTI", "WSCSTI1", "WSCSTI2", "WVTHSTI", "WMUESTI1",
    "WMUESTI2", "WMUESTI3", "WNSUBPSTI1", "WNSUBPSTI2", "WNSUBPSTI3", "WCGSO", "WCGDO", "WCLM1", "WCLM2", "WCLM3", "WWFC", "WGIDL1", "WGIDL2", "WGLEAK1", "WGLEAK2", "WGLEAK3",
    "WGLEAK6", "WGLKSD1", "WGLKSD2", "WGLKB1", "WGLKB2", "WNFTRP", "WNFALP", "WIBPC1", "WIBPC2", "WCGBO", "WCVDSOVER", "WFALPH", "WNPEXT", "WPOWRAT", "WRD", "WRD22",
    "WRD23", "WRD24", "WRDICT1", "WRDOV13", "WRDSLP1", "WRDVB", "WRDVD", "WRDVG11", "WRS", "WRTH0", "WVOVER", "PVMAX", "PBGTMP1", "PBGTMP2", "PEG0", "PVFBOVER",
    "PNOVER", "PNOVERS", "PWL2", "PVFBC", "PNSUBC", "PNSUBP", "PSCP1", "PSCP2", "PSCP3", "PSC1", "PSC2", "PSC3", "PPGD1", "PNDEP", "PNINV", "PMUECB0",
    "PMUECB1", "PMUEPH1", "PVTMP", "PWVTH0", "PMUESR1", "PMUETMP", "PSUB1", "PSUB2", "PSVDS", "PSVBS", "PSVGS", "PSUB1SNP", "PSUB2SNP", "PSVDSSNP", "PFN1", "PFN2",
    "PFN3", "PFVBS", "PNSTI", "PWSTI", "PSCSTI1", "PSCSTI2", "PVTHSTI", "PMUESTI1", "PMUESTI2", "PMUESTI3", "PNSUBPSTI1", "PNSUBPSTI2", "PNSUBPSTI3", "PCGSO", "PCGDO", "PCLM1",
    "PCLM2", "PCLM3", "PWFC", "PGIDL1", "PGIDL2", "PGLEAK1", "PGLEAK2", "PGLEAK3", "PGLEAK6", "PGLKSD1", "PGLKSD2", "PGLKB1", "PGLKB2", "PNFTRP", "PNFALP", "PIBPC1",
    "PIBPC2", "PCGBO", "PCVDSOVER", "PFALPH", "PNPEXT", "PPOWRAT", "PRD", "PRD22", "PRD23", "PRD24", "PRDICT1", "PRDOV13", "PRDSLP1", "PRDVB", "PRDVD", "PRDVG11",
    "PRS", "PRTH0", "PVOVER", "LJS0", "LJS0SW", "LNJ", "LCISBK", "LVDIFFJ", "LJS0D", "LJS0SWD", "LNJD", "LCISBKD", "LVDIFFJD", "LJS0S", "LJS0SWS", "LNJS",
    "LCISBKS", "LVDIFFJS", "WJS0", "WJS0SW", "WNJ", "WCISBK", "WVDIFFJ", "WJS0D", "WJS0SWD", "WNJD", "WCISBKD", "WVDIFFJD", "WJS0S", "WJS0SWS", "WNJS", "WCISBKS",
    "WVDIFFJS", "PJS0", "PJS0SW", "PNJ", "PCISBK", "PVDIFFJ", "PJS0D", "PJS0SWD", "PNJD", "PCISBKD", "PVDIFFJD", "PJS0S", "PJS0SWS", "PNJS", "PCISBKS", "PVDIFFJS",
];

const PARAMETER_INTEGER_FLAGS: [bool; 864] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 864] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }),
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -0.5, label: "-0.5" }),
    None, None, None, None, Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 1e-8, label: "1e-8" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 864] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 20.0, label: "20.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, Some(ParameterBound { value: 8.0, label: "8.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None,
    Some(ParameterBound { value: 0.5, label: "0.5" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 10000.0, label: "10000.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 864] = [
    2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 3, 0, 3, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 3,
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 0, 2, 3, 0, 3, 0, 0, 0,
    0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 3, 2, 2, 3, 0, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 3, 3, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 3, 3, 3, 0,
    0, 0, 0, 3, 0, 0, 3, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 0, 0, 2, 2, 3, 0, 3, 0, 0, 0,
    0, 0, 0, 0, 2, 3, 2, 0, 0, 2, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 864] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
];

fn parameter_index_for_name(name: &str) -> Option<usize> {
    PARAMETER_NAME_LOOKUP
        .iter()
        .find_map(|(candidate, index)| (*candidate == name).then_some(*index))
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
    pub nodes: [usize; 19],
    pub branches: [usize; 13],
    pub(crate) params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 864]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 21]>,
    pub(crate) ddt_state_previous: Box<[f64; 21]>,
    pub(crate) ddt_state_older: Box<[f64; 21]>,
    pub(crate) ddt_state_initialized: Box<[bool; 21]>,
    pub(crate) ddt_derivative_current: Box<[f64; 21]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 21]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scalar_static_f64: Box<[f64; 1542]>,
    pub(crate) scalar_static_bool: Box<[bool; 256]>,
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
    pub const TERMINAL_COUNT: usize = 6;
    pub const INTERNAL_NODE_COUNT: usize = 13;
    pub const NODE_COUNT: usize = 19;
    pub const INTERNAL_NODE_NAMES: [&str; 13] = ["dp", "gp", "sp", "bp", "db", "sb", "qi", "qb", "qbd", "n", "charge_A", "charge_K", "depl_A"];

    pub const BRANCH_COUNT: usize = 13;
    pub const PARAMETER_COUNT: usize = 864;
    pub const VARIABLE_COUNT: usize = 3414;
    pub const DDT_STATE_COUNT: usize = 21;
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
            scalar_static_f64: boxed_zero_f64_array::<1542>(),
            scalar_static_bool: boxed_zero_bool_array::<256>(),
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
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimhv_va'", name));
        };
        validate_parameter_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
        Ok(())
    }

    #[inline]
    fn write_parameter_slot(&mut self, index: usize, value: f64) {
        debug_assert!(index < Self::PARAMETER_COUNT, "generated parameter index out of range");
        // SAFETY: Parameters is repr(C), contains only f64 fields, and index is produced from generated parameter metadata.
        unsafe {
            let ptr = self.params.as_mut() as *mut Parameters as *mut f64;
            *ptr.add(index) = value;
        }
    }

    #[inline]
    fn finish_set_parameter(&mut self, index: usize) {
        self.mark_param_given(index);
        self.recompute_instance_static();
        self.invalidate_temperature_static();
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
        self.scalar_static_f64[0]=p.p54;
        self.scalar_static_bool[0]=(1.0==self.scalar_static_f64[0]);
        self.scalar_static_f64[1]=if param_given[12]{1.0}else{0.0};
        self.scalar_static_f64[2]=p.p334;
        self.scalar_static_f64[3]=p.p40;
        self.scalar_static_f64[4]=p.p17;
        self.scalar_static_f64[5]=(if (self.scalar_static_f64[3]!=0.0){0.0}else{self.scalar_static_f64[4]});
        self.scalar_static_f64[6]=p.p294;
        self.scalar_static_f64[7]=p.p222;
        self.scalar_static_f64[8]=p.p95;
        self.scalar_static_bool[1]=(self.scalar_static_f64[7]<0.0);
        self.scalar_static_f64[9]=(if self.scalar_static_bool[1]{1.0}else{0.0});
        self.scalar_static_f64[10]=(if (self.scalar_static_f64[9]!=0.0){0.0}else{self.scalar_static_f64[7]});
        self.scalar_static_bool[2]=(self.scalar_static_f64[10]>0.0);
        self.scalar_static_f64[11]=(if self.scalar_static_bool[2]{1.0}else{0.0});
        self.scalar_static_f64[12]=(if (self.scalar_static_f64[11]!=0.0){0.0}else{self.scalar_static_f64[10]});
        self.scalar_static_bool[3]=(self.scalar_static_f64[6]<0.0);
        self.scalar_static_f64[13]=(if self.scalar_static_bool[3]{1.0}else{0.0});
        self.scalar_static_f64[14]=(if (self.scalar_static_f64[13]!=0.0){0.0}else{self.scalar_static_f64[6]});
        self.scalar_static_f64[15]=p.p433;
        self.scalar_static_f64[16]=p.p97;
        self.scalar_static_f64[17]=p.p114;
        self.scalar_static_f64[18]=p.p132;
        self.scalar_static_f64[19]=p.p137;
        self.scalar_static_f64[20]=p.p139;
        self.scalar_static_f64[21]=p.p140;
        self.scalar_static_f64[22]=p.p152;
        self.scalar_static_f64[23]=p.p153;
        self.scalar_static_f64[24]=p.p220;
        self.scalar_static_f64[25]=p.p150;
        self.scalar_static_f64[26]=p.p221;
        self.scalar_static_f64[27]=p.p223;
        self.scalar_static_f64[28]=p.p224;
        self.scalar_static_f64[29]=p.p235;
        self.scalar_static_f64[30]=p.p236;
        self.scalar_static_f64[31]=p.p237;
        self.scalar_static_f64[32]=p.p252;
        self.scalar_static_f64[33]=p.p274;
        self.scalar_static_f64[34]=p.p107;
        self.scalar_static_f64[35]=p.p109;
        self.scalar_static_f64[36]=p.p108;
        self.scalar_static_f64[37]=p.p311;
        self.scalar_static_f64[38]=p.p312;
        self.scalar_static_f64[39]=p.p313;
        self.scalar_static_f64[40]=p.p314;
        self.scalar_static_f64[41]=p.p292;
        self.scalar_static_f64[42]=p.p328;
        self.scalar_static_f64[43]=p.p329;
        self.scalar_static_f64[44]=p.p315;
        self.scalar_static_f64[45]=p.p316;
        self.scalar_static_f64[46]=p.p317;
        self.scalar_static_f64[47]=p.p318;
        self.scalar_static_f64[48]=p.p327;
        self.scalar_static_f64[49]=p.p330;
        self.scalar_static_f64[50]=p.p331;
        self.scalar_static_f64[51]=p.p67;
        self.scalar_static_f64[52]=p.p69;
        self.scalar_static_f64[53]=p.p68;
        self.scalar_static_f64[54]=p.p70;
        self.scalar_static_f64[55]=p.p309;
        self.scalar_static_f64[56]=p.p310;
        self.scalar_static_f64[57]=p.p301;
        self.scalar_static_f64[58]=p.p302;
        self.scalar_static_f64[59]=p.p303;
        self.scalar_static_f64[60]=p.p304;
        self.scalar_static_f64[61]=p.p305;
        self.scalar_static_f64[62]=p.p306;
        self.scalar_static_f64[63]=p.p307;
        self.scalar_static_f64[64]=p.p308;
        self.scalar_static_f64[65]=p.p98;
        self.scalar_static_f64[66]=p.p99;
        self.scalar_static_f64[67]=p.p100;
        self.scalar_static_f64[68]=p.p101;
        self.scalar_static_f64[69]=p.p415;
        self.scalar_static_f64[70]=p.p340;
        self.scalar_static_f64[71]=p.p42;
        self.scalar_static_bool[4]=(self.scalar_static_f64[71]<3.0);
        self.scalar_static_bool[5]=(self.scalar_static_f64[71]>0.0);
        self.scalar_static_bool[6]=(self.scalar_static_bool[4]&&self.scalar_static_bool[5]);
        self.scalar_static_f64[72]=(if self.scalar_static_bool[6]{1.0}else{0.0});
        self.scalar_static_bool[7]=(self.scalar_static_f64[70]<5000000000000000.0);
        self.scalar_static_f64[73]=(if self.scalar_static_bool[7]{1.0}else{0.0});
        self.scalar_static_bool[8]=((self.scalar_static_f64[72]!=0.0)&&(self.scalar_static_f64[73]!=0.0));
        self.scalar_static_f64[74]=(if self.scalar_static_bool[8]{5000000000000000.0}else{self.scalar_static_f64[70]});
        self.scalar_static_bool[9]=(self.scalar_static_f64[74]>1e18);
        self.scalar_static_f64[75]=(if self.scalar_static_bool[9]{1.0}else{0.0});
        self.scalar_static_bool[10]=((self.scalar_static_f64[72]!=0.0)&&(self.scalar_static_f64[75]!=0.0));
        self.scalar_static_f64[76]=(if self.scalar_static_bool[10]{1e18}else{self.scalar_static_f64[74]});
        self.scalar_static_bool[11]=(3.0==self.scalar_static_f64[71]);
        self.scalar_static_f64[77]=(if self.scalar_static_bool[11]{1.0}else{0.0});
        self.scalar_static_bool[12]=(self.scalar_static_f64[76]<5000000000000000.0);
        self.scalar_static_f64[78]=(if self.scalar_static_bool[12]{1.0}else{0.0});
        self.scalar_static_bool[13]=(!(self.scalar_static_f64[72]!=0.0));
        self.scalar_static_bool[14]=((self.scalar_static_f64[77]!=0.0)&&self.scalar_static_bool[13]);
        self.scalar_static_bool[15]=((self.scalar_static_f64[78]!=0.0)&&self.scalar_static_bool[14]);
        self.scalar_static_f64[79]=(if self.scalar_static_bool[15]{5000000000000000.0}else{self.scalar_static_f64[76]});
        self.scalar_static_bool[16]=(self.scalar_static_f64[79]>1e18);
        self.scalar_static_f64[80]=(if self.scalar_static_bool[16]{1.0}else{0.0});
        self.scalar_static_bool[17]=(self.scalar_static_bool[14]&&(self.scalar_static_f64[80]!=0.0));
        self.scalar_static_f64[81]=(if self.scalar_static_bool[17]{1e18}else{self.scalar_static_f64[79]});
        self.scalar_static_bool[18]=(0.0!=self.scalar_static_f64[71]);
        self.scalar_static_f64[82]=p.p120;
        self.scalar_static_f64[83]=p.p122;
        self.scalar_static_f64[84]=f64::powf(100.0,self.scalar_static_f64[83]);
        self.scalar_static_f64[85]=(self.scalar_static_f64[82]/self.scalar_static_f64[84]);
        self.scalar_static_f64[86]=p.p123;
        self.scalar_static_f64[87]=p.p129;
        self.scalar_static_f64[88]=f64::powf(100.0,self.scalar_static_f64[87]);
        self.scalar_static_f64[89]=(self.scalar_static_f64[86]/self.scalar_static_f64[88]);
        self.scalar_static_f64[90]=p.p201;
        self.scalar_static_f64[91]=(self.scalar_static_f64[37]/100.0);
        self.scalar_static_f64[92]=(self.scalar_static_f64[38]/100.0);
        self.scalar_static_f64[93]=(self.scalar_static_f64[39]/100.0);
        self.scalar_static_f64[94]=(self.scalar_static_f64[40]/100.0);
        self.scalar_static_f64[95]=p.p336;
        self.scalar_static_f64[96]=(self.scalar_static_f64[95]/1e-6);
        self.scalar_static_f64[97]=p.p272;
        self.scalar_static_f64[98]=(self.scalar_static_f64[97]/10000.0);
        self.scalar_static_f64[99]=p.p293;
        self.scalar_static_f64[100]=(100.0*self.scalar_static_f64[99]);
        self.scalar_static_f64[101]=(self.scalar_static_f64[81]/1e-6);
        self.scalar_static_f64[102]=p.p453;
        self.scalar_static_f64[103]=(self.scalar_static_f64[102]/1e-6);
        self.scalar_static_f64[104]=(self.scalar_static_f64[33]+273.15);
        self.scalar_static_f64[105]=p.p0;
        self.scalar_static_f64[106]=p.p1;
        self.scalar_static_f64[107]=p.p7;
        self.scalar_static_f64[108]=(self.scalar_static_f64[106]/self.scalar_static_f64[107]);
        self.scalar_static_f64[109]=p.p116;
        self.scalar_static_f64[110]=(self.scalar_static_f64[105]+self.scalar_static_f64[109]);
        self.scalar_static_f64[111]=p.p117;
        self.scalar_static_f64[112]=(self.scalar_static_f64[108]+self.scalar_static_f64[111]);
        self.scalar_static_f64[113]=p.p6;
        self.scalar_static_f64[114]=(self.scalar_static_f64[110]-self.scalar_static_f64[113]);
        self.scalar_static_f64[115]=p.p121;
        self.scalar_static_f64[116]=(self.scalar_static_f64[110]+self.scalar_static_f64[115]);
        self.scalar_static_f64[117]=p.p128;
        self.scalar_static_f64[118]=(self.scalar_static_f64[112]+self.scalar_static_f64[117]);
        self.scalar_static_f64[119]=(self.scalar_static_f64[51]+self.scalar_static_f64[53]);
        self.scalar_static_f64[120]=(1000000.0*self.scalar_static_f64[110]);
        self.scalar_static_f64[121]=(1000000.0*self.scalar_static_f64[112]);
        self.scalar_static_f64[122]=p.p553;
        self.scalar_static_f64[123]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[122]);
        self.scalar_static_f64[124]=p.p554;
        self.scalar_static_f64[125]=f64::powf(self.scalar_static_f64[121],self.scalar_static_f64[124]);
        self.scalar_static_f64[126]=(self.scalar_static_f64[123]*self.scalar_static_f64[125]);
        self.scalar_static_f64[127]=p.p92;
        self.scalar_static_f64[128]=p.p556;
        self.scalar_static_f64[129]=(self.scalar_static_f64[128]/self.scalar_static_f64[123]);
        self.scalar_static_f64[130]=(self.scalar_static_f64[127]+self.scalar_static_f64[129]);
        self.scalar_static_f64[131]=p.p644;
        self.scalar_static_f64[132]=(self.scalar_static_f64[131]/self.scalar_static_f64[125]);
        self.scalar_static_f64[133]=(self.scalar_static_f64[130]+self.scalar_static_f64[132]);
        self.scalar_static_f64[134]=p.p732;
        self.scalar_static_f64[135]=(self.scalar_static_f64[134]/self.scalar_static_f64[126]);
        self.scalar_static_f64[136]=(self.scalar_static_f64[133]+self.scalar_static_f64[135]);
        self.scalar_static_f64[137]=p.p93;
        self.scalar_static_f64[138]=p.p557;
        self.scalar_static_f64[139]=(self.scalar_static_f64[138]/self.scalar_static_f64[123]);
        self.scalar_static_f64[140]=(self.scalar_static_f64[137]+self.scalar_static_f64[139]);
        self.scalar_static_f64[141]=p.p645;
        self.scalar_static_f64[142]=(self.scalar_static_f64[141]/self.scalar_static_f64[125]);
        self.scalar_static_f64[143]=(self.scalar_static_f64[140]+self.scalar_static_f64[142]);
        self.scalar_static_f64[144]=p.p733;
        self.scalar_static_f64[145]=(self.scalar_static_f64[144]/self.scalar_static_f64[126]);
        self.scalar_static_f64[146]=(self.scalar_static_f64[143]+self.scalar_static_f64[145]);
        self.scalar_static_f64[147]=p.p94;
        self.scalar_static_f64[148]=p.p558;
        self.scalar_static_f64[149]=(self.scalar_static_f64[148]/self.scalar_static_f64[123]);
        self.scalar_static_f64[150]=(self.scalar_static_f64[147]+self.scalar_static_f64[149]);
        self.scalar_static_f64[151]=p.p646;
        self.scalar_static_f64[152]=(self.scalar_static_f64[151]/self.scalar_static_f64[125]);
        self.scalar_static_f64[153]=(self.scalar_static_f64[150]+self.scalar_static_f64[152]);
        self.scalar_static_f64[154]=p.p734;
        self.scalar_static_f64[155]=(self.scalar_static_f64[154]/self.scalar_static_f64[126]);
        self.scalar_static_f64[156]=(self.scalar_static_f64[153]+self.scalar_static_f64[155]);
        self.scalar_static_f64[157]=p.p111;
        self.scalar_static_f64[158]=p.p560;
        self.scalar_static_f64[159]=(self.scalar_static_f64[158]/self.scalar_static_f64[123]);
        self.scalar_static_f64[160]=(self.scalar_static_f64[157]+self.scalar_static_f64[159]);
        self.scalar_static_f64[161]=p.p648;
        self.scalar_static_f64[162]=(self.scalar_static_f64[161]/self.scalar_static_f64[125]);
        self.scalar_static_f64[163]=(self.scalar_static_f64[160]+self.scalar_static_f64[162]);
        self.scalar_static_f64[164]=p.p736;
        self.scalar_static_f64[165]=(self.scalar_static_f64[164]/self.scalar_static_f64[126]);
        self.scalar_static_f64[166]=(self.scalar_static_f64[163]+self.scalar_static_f64[165]);
        self.scalar_static_f64[167]=p.p112;
        self.scalar_static_f64[168]=p.p561;
        self.scalar_static_f64[169]=(self.scalar_static_f64[168]/self.scalar_static_f64[123]);
        self.scalar_static_f64[170]=(self.scalar_static_f64[167]+self.scalar_static_f64[169]);
        self.scalar_static_f64[171]=p.p649;
        self.scalar_static_f64[172]=(self.scalar_static_f64[171]/self.scalar_static_f64[125]);
        self.scalar_static_f64[173]=(self.scalar_static_f64[170]+self.scalar_static_f64[172]);
        self.scalar_static_f64[174]=p.p737;
        self.scalar_static_f64[175]=(self.scalar_static_f64[174]/self.scalar_static_f64[126]);
        self.scalar_static_f64[176]=(self.scalar_static_f64[173]+self.scalar_static_f64[175]);
        self.scalar_static_f64[177]=p.p126;
        self.scalar_static_f64[178]=p.p562;
        self.scalar_static_f64[179]=(self.scalar_static_f64[178]/self.scalar_static_f64[123]);
        self.scalar_static_f64[180]=(self.scalar_static_f64[177]+self.scalar_static_f64[179]);
        self.scalar_static_f64[181]=p.p650;
        self.scalar_static_f64[182]=(self.scalar_static_f64[181]/self.scalar_static_f64[125]);
        self.scalar_static_f64[183]=(self.scalar_static_f64[180]+self.scalar_static_f64[182]);
        self.scalar_static_f64[184]=p.p738;
        self.scalar_static_f64[185]=(self.scalar_static_f64[184]/self.scalar_static_f64[126]);
        self.scalar_static_f64[186]=(self.scalar_static_f64[183]+self.scalar_static_f64[185]);
        self.scalar_static_f64[187]=p.p136;
        self.scalar_static_f64[188]=p.p563;
        self.scalar_static_f64[189]=(self.scalar_static_f64[188]/self.scalar_static_f64[123]);
        self.scalar_static_f64[190]=(self.scalar_static_f64[187]+self.scalar_static_f64[189]);
        self.scalar_static_f64[191]=p.p651;
        self.scalar_static_f64[192]=(self.scalar_static_f64[191]/self.scalar_static_f64[125]);
        self.scalar_static_f64[193]=(self.scalar_static_f64[190]+self.scalar_static_f64[192]);
        self.scalar_static_f64[194]=p.p739;
        self.scalar_static_f64[195]=(self.scalar_static_f64[194]/self.scalar_static_f64[126]);
        self.scalar_static_f64[196]=(self.scalar_static_f64[193]+self.scalar_static_f64[195]);
        self.scalar_static_f64[197]=p.p138;
        self.scalar_static_f64[198]=p.p564;
        self.scalar_static_f64[199]=(self.scalar_static_f64[198]/self.scalar_static_f64[123]);
        self.scalar_static_f64[200]=(self.scalar_static_f64[197]+self.scalar_static_f64[199]);
        self.scalar_static_f64[201]=p.p652;
        self.scalar_static_f64[202]=(self.scalar_static_f64[201]/self.scalar_static_f64[125]);
        self.scalar_static_f64[203]=(self.scalar_static_f64[200]+self.scalar_static_f64[202]);
        self.scalar_static_f64[204]=p.p740;
        self.scalar_static_f64[205]=(self.scalar_static_f64[204]/self.scalar_static_f64[126]);
        self.scalar_static_f64[206]=(self.scalar_static_f64[203]+self.scalar_static_f64[205]);
        self.scalar_static_f64[207]=p.p141;
        self.scalar_static_f64[208]=p.p565;
        self.scalar_static_f64[209]=(self.scalar_static_f64[208]/self.scalar_static_f64[123]);
        self.scalar_static_f64[210]=(self.scalar_static_f64[207]+self.scalar_static_f64[209]);
        self.scalar_static_f64[211]=p.p653;
        self.scalar_static_f64[212]=(self.scalar_static_f64[211]/self.scalar_static_f64[125]);
        self.scalar_static_f64[213]=(self.scalar_static_f64[210]+self.scalar_static_f64[212]);
        self.scalar_static_f64[214]=p.p741;
        self.scalar_static_f64[215]=(self.scalar_static_f64[214]/self.scalar_static_f64[126]);
        self.scalar_static_f64[216]=(self.scalar_static_f64[213]+self.scalar_static_f64[215]);
        self.scalar_static_f64[217]=p.p144;
        self.scalar_static_f64[218]=p.p566;
        self.scalar_static_f64[219]=(self.scalar_static_f64[218]/self.scalar_static_f64[123]);
        self.scalar_static_f64[220]=(self.scalar_static_f64[217]+self.scalar_static_f64[219]);
        self.scalar_static_f64[221]=p.p654;
        self.scalar_static_f64[222]=(self.scalar_static_f64[221]/self.scalar_static_f64[125]);
        self.scalar_static_f64[223]=(self.scalar_static_f64[220]+self.scalar_static_f64[222]);
        self.scalar_static_f64[224]=p.p742;
        self.scalar_static_f64[225]=(self.scalar_static_f64[224]/self.scalar_static_f64[126]);
        self.scalar_static_f64[226]=(self.scalar_static_f64[223]+self.scalar_static_f64[225]);
        self.scalar_static_f64[227]=p.p145;
        self.scalar_static_f64[228]=p.p567;
        self.scalar_static_f64[229]=(self.scalar_static_f64[228]/self.scalar_static_f64[123]);
        self.scalar_static_f64[230]=(self.scalar_static_f64[227]+self.scalar_static_f64[229]);
        self.scalar_static_f64[231]=p.p655;
        self.scalar_static_f64[232]=(self.scalar_static_f64[231]/self.scalar_static_f64[125]);
        self.scalar_static_f64[233]=(self.scalar_static_f64[230]+self.scalar_static_f64[232]);
        self.scalar_static_f64[234]=p.p743;
        self.scalar_static_f64[235]=(self.scalar_static_f64[234]/self.scalar_static_f64[126]);
        self.scalar_static_f64[236]=(self.scalar_static_f64[233]+self.scalar_static_f64[235]);
        self.scalar_static_f64[237]=p.p146;
        self.scalar_static_f64[238]=p.p568;
        self.scalar_static_f64[239]=(self.scalar_static_f64[238]/self.scalar_static_f64[123]);
        self.scalar_static_f64[240]=(self.scalar_static_f64[237]+self.scalar_static_f64[239]);
        self.scalar_static_f64[241]=p.p656;
        self.scalar_static_f64[242]=(self.scalar_static_f64[241]/self.scalar_static_f64[125]);
        self.scalar_static_f64[243]=(self.scalar_static_f64[240]+self.scalar_static_f64[242]);
        self.scalar_static_f64[244]=p.p744;
        self.scalar_static_f64[245]=(self.scalar_static_f64[244]/self.scalar_static_f64[126]);
        self.scalar_static_f64[246]=(self.scalar_static_f64[243]+self.scalar_static_f64[245]);
        self.scalar_static_f64[247]=p.p147;
        self.scalar_static_f64[248]=p.p569;
        self.scalar_static_f64[249]=(self.scalar_static_f64[248]/self.scalar_static_f64[123]);
        self.scalar_static_f64[250]=(self.scalar_static_f64[247]+self.scalar_static_f64[249]);
        self.scalar_static_f64[251]=p.p657;
        self.scalar_static_f64[252]=(self.scalar_static_f64[251]/self.scalar_static_f64[125]);
        self.scalar_static_f64[253]=(self.scalar_static_f64[250]+self.scalar_static_f64[252]);
        self.scalar_static_f64[254]=p.p745;
        self.scalar_static_f64[255]=(self.scalar_static_f64[254]/self.scalar_static_f64[126]);
        self.scalar_static_f64[256]=(self.scalar_static_f64[253]+self.scalar_static_f64[255]);
        self.scalar_static_f64[257]=p.p148;
        self.scalar_static_f64[258]=p.p570;
        self.scalar_static_f64[259]=(self.scalar_static_f64[258]/self.scalar_static_f64[123]);
        self.scalar_static_f64[260]=(self.scalar_static_f64[257]+self.scalar_static_f64[259]);
        self.scalar_static_f64[261]=p.p658;
        self.scalar_static_f64[262]=(self.scalar_static_f64[261]/self.scalar_static_f64[125]);
        self.scalar_static_f64[263]=(self.scalar_static_f64[260]+self.scalar_static_f64[262]);
        self.scalar_static_f64[264]=p.p746;
        self.scalar_static_f64[265]=(self.scalar_static_f64[264]/self.scalar_static_f64[126]);
        self.scalar_static_f64[266]=(self.scalar_static_f64[263]+self.scalar_static_f64[265]);
        self.scalar_static_f64[267]=p.p149;
        self.scalar_static_f64[268]=p.p571;
        self.scalar_static_f64[269]=(self.scalar_static_f64[268]/self.scalar_static_f64[123]);
        self.scalar_static_f64[270]=(self.scalar_static_f64[267]+self.scalar_static_f64[269]);
        self.scalar_static_f64[271]=p.p659;
        self.scalar_static_f64[272]=(self.scalar_static_f64[271]/self.scalar_static_f64[125]);
        self.scalar_static_f64[273]=(self.scalar_static_f64[270]+self.scalar_static_f64[272]);
        self.scalar_static_f64[274]=p.p747;
        self.scalar_static_f64[275]=(self.scalar_static_f64[274]/self.scalar_static_f64[126]);
        self.scalar_static_f64[276]=(self.scalar_static_f64[273]+self.scalar_static_f64[275]);
        self.scalar_static_f64[277]=p.p151;
        self.scalar_static_f64[278]=p.p572;
        self.scalar_static_f64[279]=(self.scalar_static_f64[278]/self.scalar_static_f64[123]);
        self.scalar_static_f64[280]=(self.scalar_static_f64[277]+self.scalar_static_f64[279]);
        self.scalar_static_f64[281]=p.p660;
        self.scalar_static_f64[282]=(self.scalar_static_f64[281]/self.scalar_static_f64[125]);
        self.scalar_static_f64[283]=(self.scalar_static_f64[280]+self.scalar_static_f64[282]);
        self.scalar_static_f64[284]=p.p748;
        self.scalar_static_f64[285]=(self.scalar_static_f64[284]/self.scalar_static_f64[126]);
        self.scalar_static_f64[286]=(self.scalar_static_f64[283]+self.scalar_static_f64[285]);
        self.scalar_static_f64[287]=p.p170;
        self.scalar_static_f64[288]=p.p579;
        self.scalar_static_f64[289]=(self.scalar_static_f64[288]/self.scalar_static_f64[123]);
        self.scalar_static_f64[290]=(self.scalar_static_f64[287]+self.scalar_static_f64[289]);
        self.scalar_static_f64[291]=p.p667;
        self.scalar_static_f64[292]=(self.scalar_static_f64[291]/self.scalar_static_f64[125]);
        self.scalar_static_f64[293]=(self.scalar_static_f64[290]+self.scalar_static_f64[292]);
        self.scalar_static_f64[294]=p.p755;
        self.scalar_static_f64[295]=(self.scalar_static_f64[294]/self.scalar_static_f64[126]);
        self.scalar_static_f64[296]=(self.scalar_static_f64[293]+self.scalar_static_f64[295]);
        self.scalar_static_f64[297]=p.p177;
        self.scalar_static_f64[298]=p.p581;
        self.scalar_static_f64[299]=(self.scalar_static_f64[298]/self.scalar_static_f64[123]);
        self.scalar_static_f64[300]=(self.scalar_static_f64[297]+self.scalar_static_f64[299]);
        self.scalar_static_f64[301]=p.p669;
        self.scalar_static_f64[302]=(self.scalar_static_f64[301]/self.scalar_static_f64[125]);
        self.scalar_static_f64[303]=(self.scalar_static_f64[300]+self.scalar_static_f64[302]);
        self.scalar_static_f64[304]=p.p757;
        self.scalar_static_f64[305]=(self.scalar_static_f64[304]/self.scalar_static_f64[126]);
        self.scalar_static_f64[306]=(self.scalar_static_f64[303]+self.scalar_static_f64[305]);
        self.scalar_static_f64[307]=p.p194;
        self.scalar_static_f64[308]=p.p590;
        self.scalar_static_f64[309]=(self.scalar_static_f64[308]/self.scalar_static_f64[123]);
        self.scalar_static_f64[310]=(self.scalar_static_f64[307]+self.scalar_static_f64[309]);
        self.scalar_static_f64[311]=p.p678;
        self.scalar_static_f64[312]=(self.scalar_static_f64[311]/self.scalar_static_f64[125]);
        self.scalar_static_f64[313]=(self.scalar_static_f64[310]+self.scalar_static_f64[312]);
        self.scalar_static_f64[314]=p.p766;
        self.scalar_static_f64[315]=(self.scalar_static_f64[314]/self.scalar_static_f64[126]);
        self.scalar_static_f64[316]=(self.scalar_static_f64[313]+self.scalar_static_f64[315]);
        self.scalar_static_f64[317]=p.p195;
        self.scalar_static_f64[318]=p.p591;
        self.scalar_static_f64[319]=(self.scalar_static_f64[318]/self.scalar_static_f64[123]);
        self.scalar_static_f64[320]=(self.scalar_static_f64[317]+self.scalar_static_f64[319]);
        self.scalar_static_f64[321]=p.p679;
        self.scalar_static_f64[322]=(self.scalar_static_f64[321]/self.scalar_static_f64[125]);
        self.scalar_static_f64[323]=(self.scalar_static_f64[320]+self.scalar_static_f64[322]);
        self.scalar_static_f64[324]=p.p767;
        self.scalar_static_f64[325]=(self.scalar_static_f64[324]/self.scalar_static_f64[126]);
        self.scalar_static_f64[326]=(self.scalar_static_f64[323]+self.scalar_static_f64[325]);
        self.scalar_static_f64[327]=p.p196;
        self.scalar_static_f64[328]=p.p592;
        self.scalar_static_f64[329]=(self.scalar_static_f64[328]/self.scalar_static_f64[123]);
        self.scalar_static_f64[330]=(self.scalar_static_f64[327]+self.scalar_static_f64[329]);
        self.scalar_static_f64[331]=p.p680;
        self.scalar_static_f64[332]=(self.scalar_static_f64[331]/self.scalar_static_f64[125]);
        self.scalar_static_f64[333]=(self.scalar_static_f64[330]+self.scalar_static_f64[332]);
        self.scalar_static_f64[334]=p.p768;
        self.scalar_static_f64[335]=(self.scalar_static_f64[334]/self.scalar_static_f64[126]);
        self.scalar_static_f64[336]=(self.scalar_static_f64[333]+self.scalar_static_f64[335]);
        self.scalar_static_f64[337]=p.p197;
        self.scalar_static_f64[338]=p.p593;
        self.scalar_static_f64[339]=(self.scalar_static_f64[338]/self.scalar_static_f64[123]);
        self.scalar_static_f64[340]=(self.scalar_static_f64[337]+self.scalar_static_f64[339]);
        self.scalar_static_f64[341]=p.p681;
        self.scalar_static_f64[342]=(self.scalar_static_f64[341]/self.scalar_static_f64[125]);
        self.scalar_static_f64[343]=(self.scalar_static_f64[340]+self.scalar_static_f64[342]);
        self.scalar_static_f64[344]=p.p769;
        self.scalar_static_f64[345]=(self.scalar_static_f64[344]/self.scalar_static_f64[126]);
        self.scalar_static_f64[346]=(self.scalar_static_f64[343]+self.scalar_static_f64[345]);
        self.scalar_static_f64[347]=p.p214;
        self.scalar_static_f64[348]=p.p599;
        self.scalar_static_f64[349]=(self.scalar_static_f64[348]/self.scalar_static_f64[123]);
        self.scalar_static_f64[350]=(self.scalar_static_f64[347]+self.scalar_static_f64[349]);
        self.scalar_static_f64[351]=p.p687;
        self.scalar_static_f64[352]=(self.scalar_static_f64[351]/self.scalar_static_f64[125]);
        self.scalar_static_f64[353]=(self.scalar_static_f64[350]+self.scalar_static_f64[352]);
        self.scalar_static_f64[354]=p.p775;
        self.scalar_static_f64[355]=(self.scalar_static_f64[354]/self.scalar_static_f64[126]);
        self.scalar_static_f64[356]=(self.scalar_static_f64[353]+self.scalar_static_f64[355]);
        self.scalar_static_f64[357]=p.p216;
        self.scalar_static_f64[358]=p.p601;
        self.scalar_static_f64[359]=(self.scalar_static_f64[358]/self.scalar_static_f64[123]);
        self.scalar_static_f64[360]=(self.scalar_static_f64[357]+self.scalar_static_f64[359]);
        self.scalar_static_f64[361]=p.p689;
        self.scalar_static_f64[362]=(self.scalar_static_f64[361]/self.scalar_static_f64[125]);
        self.scalar_static_f64[363]=(self.scalar_static_f64[360]+self.scalar_static_f64[362]);
        self.scalar_static_f64[364]=p.p777;
        self.scalar_static_f64[365]=(self.scalar_static_f64[364]/self.scalar_static_f64[126]);
        self.scalar_static_f64[366]=(self.scalar_static_f64[363]+self.scalar_static_f64[365]);
        self.scalar_static_f64[367]=p.p217;
        self.scalar_static_f64[368]=p.p602;
        self.scalar_static_f64[369]=(self.scalar_static_f64[368]/self.scalar_static_f64[123]);
        self.scalar_static_f64[370]=(self.scalar_static_f64[367]+self.scalar_static_f64[369]);
        self.scalar_static_f64[371]=p.p690;
        self.scalar_static_f64[372]=(self.scalar_static_f64[371]/self.scalar_static_f64[125]);
        self.scalar_static_f64[373]=(self.scalar_static_f64[370]+self.scalar_static_f64[372]);
        self.scalar_static_f64[374]=p.p778;
        self.scalar_static_f64[375]=(self.scalar_static_f64[374]/self.scalar_static_f64[126]);
        self.scalar_static_f64[376]=(self.scalar_static_f64[373]+self.scalar_static_f64[375]);
        self.scalar_static_f64[377]=p.p218;
        self.scalar_static_f64[378]=p.p603;
        self.scalar_static_f64[379]=(self.scalar_static_f64[378]/self.scalar_static_f64[123]);
        self.scalar_static_f64[380]=(self.scalar_static_f64[377]+self.scalar_static_f64[379]);
        self.scalar_static_f64[381]=p.p691;
        self.scalar_static_f64[382]=(self.scalar_static_f64[381]/self.scalar_static_f64[125]);
        self.scalar_static_f64[383]=(self.scalar_static_f64[380]+self.scalar_static_f64[382]);
        self.scalar_static_f64[384]=p.p779;
        self.scalar_static_f64[385]=(self.scalar_static_f64[384]/self.scalar_static_f64[126]);
        self.scalar_static_f64[386]=(self.scalar_static_f64[383]+self.scalar_static_f64[385]);
        self.scalar_static_f64[387]=p.p219;
        self.scalar_static_f64[388]=p.p604;
        self.scalar_static_f64[389]=(self.scalar_static_f64[388]/self.scalar_static_f64[123]);
        self.scalar_static_f64[390]=(self.scalar_static_f64[387]+self.scalar_static_f64[389]);
        self.scalar_static_f64[391]=p.p692;
        self.scalar_static_f64[392]=(self.scalar_static_f64[391]/self.scalar_static_f64[125]);
        self.scalar_static_f64[393]=(self.scalar_static_f64[390]+self.scalar_static_f64[392]);
        self.scalar_static_f64[394]=p.p780;
        self.scalar_static_f64[395]=(self.scalar_static_f64[394]/self.scalar_static_f64[126]);
        self.scalar_static_f64[396]=(self.scalar_static_f64[393]+self.scalar_static_f64[395]);
        self.scalar_static_f64[397]=p.p232;
        self.scalar_static_f64[398]=p.p610;
        self.scalar_static_f64[399]=(self.scalar_static_f64[398]/self.scalar_static_f64[123]);
        self.scalar_static_f64[400]=(self.scalar_static_f64[397]+self.scalar_static_f64[399]);
        self.scalar_static_f64[401]=p.p698;
        self.scalar_static_f64[402]=(self.scalar_static_f64[401]/self.scalar_static_f64[125]);
        self.scalar_static_f64[403]=(self.scalar_static_f64[400]+self.scalar_static_f64[402]);
        self.scalar_static_f64[404]=p.p786;
        self.scalar_static_f64[405]=(self.scalar_static_f64[404]/self.scalar_static_f64[126]);
        self.scalar_static_f64[406]=(self.scalar_static_f64[403]+self.scalar_static_f64[405]);
        self.scalar_static_f64[407]=p.p253;
        self.scalar_static_f64[408]=p.p617;
        self.scalar_static_f64[409]=(self.scalar_static_f64[408]/self.scalar_static_f64[123]);
        self.scalar_static_f64[410]=(self.scalar_static_f64[407]+self.scalar_static_f64[409]);
        self.scalar_static_f64[411]=p.p705;
        self.scalar_static_f64[412]=(self.scalar_static_f64[411]/self.scalar_static_f64[125]);
        self.scalar_static_f64[413]=(self.scalar_static_f64[410]+self.scalar_static_f64[412]);
        self.scalar_static_f64[414]=p.p793;
        self.scalar_static_f64[415]=(self.scalar_static_f64[414]/self.scalar_static_f64[126]);
        self.scalar_static_f64[416]=(self.scalar_static_f64[413]+self.scalar_static_f64[415]);
        self.scalar_static_f64[417]=p.p256;
        self.scalar_static_f64[418]=p.p619;
        self.scalar_static_f64[419]=(self.scalar_static_f64[418]/self.scalar_static_f64[123]);
        self.scalar_static_f64[420]=(self.scalar_static_f64[417]+self.scalar_static_f64[419]);
        self.scalar_static_f64[421]=p.p707;
        self.scalar_static_f64[422]=(self.scalar_static_f64[421]/self.scalar_static_f64[125]);
        self.scalar_static_f64[423]=(self.scalar_static_f64[420]+self.scalar_static_f64[422]);
        self.scalar_static_f64[424]=p.p795;
        self.scalar_static_f64[425]=(self.scalar_static_f64[424]/self.scalar_static_f64[126]);
        self.scalar_static_f64[426]=(self.scalar_static_f64[423]+self.scalar_static_f64[425]);
        self.scalar_static_f64[427]=p.p257;
        self.scalar_static_f64[428]=p.p620;
        self.scalar_static_f64[429]=(self.scalar_static_f64[428]/self.scalar_static_f64[123]);
        self.scalar_static_f64[430]=(self.scalar_static_f64[427]+self.scalar_static_f64[429]);
        self.scalar_static_f64[431]=p.p708;
        self.scalar_static_f64[432]=(self.scalar_static_f64[431]/self.scalar_static_f64[125]);
        self.scalar_static_f64[433]=(self.scalar_static_f64[430]+self.scalar_static_f64[432]);
        self.scalar_static_f64[434]=p.p796;
        self.scalar_static_f64[435]=(self.scalar_static_f64[434]/self.scalar_static_f64[126]);
        self.scalar_static_f64[436]=(self.scalar_static_f64[433]+self.scalar_static_f64[435]);
        self.scalar_static_f64[437]=p.p76;
        self.scalar_static_f64[438]=p.p628;
        self.scalar_static_f64[439]=(self.scalar_static_f64[438]/self.scalar_static_f64[123]);
        self.scalar_static_f64[440]=(self.scalar_static_f64[437]+self.scalar_static_f64[439]);
        self.scalar_static_f64[441]=p.p716;
        self.scalar_static_f64[442]=(self.scalar_static_f64[441]/self.scalar_static_f64[125]);
        self.scalar_static_f64[443]=(self.scalar_static_f64[440]+self.scalar_static_f64[442]);
        self.scalar_static_f64[444]=p.p804;
        self.scalar_static_f64[445]=(self.scalar_static_f64[444]/self.scalar_static_f64[126]);
        self.scalar_static_f64[446]=(self.scalar_static_f64[443]+self.scalar_static_f64[445]);
        self.scalar_static_f64[447]=p.p81;
        self.scalar_static_f64[448]=p.p629;
        self.scalar_static_f64[449]=(self.scalar_static_f64[448]/self.scalar_static_f64[123]);
        self.scalar_static_f64[450]=(self.scalar_static_f64[447]+self.scalar_static_f64[449]);
        self.scalar_static_f64[451]=p.p717;
        self.scalar_static_f64[452]=(self.scalar_static_f64[451]/self.scalar_static_f64[125]);
        self.scalar_static_f64[453]=(self.scalar_static_f64[450]+self.scalar_static_f64[452]);
        self.scalar_static_f64[454]=p.p805;
        self.scalar_static_f64[455]=(self.scalar_static_f64[454]/self.scalar_static_f64[126]);
        self.scalar_static_f64[456]=(self.scalar_static_f64[453]+self.scalar_static_f64[455]);
        self.scalar_static_f64[457]=p.p74;
        self.scalar_static_f64[458]=p.p630;
        self.scalar_static_f64[459]=(self.scalar_static_f64[458]/self.scalar_static_f64[123]);
        self.scalar_static_f64[460]=(self.scalar_static_f64[457]+self.scalar_static_f64[459]);
        self.scalar_static_f64[461]=p.p718;
        self.scalar_static_f64[462]=(self.scalar_static_f64[461]/self.scalar_static_f64[125]);
        self.scalar_static_f64[463]=(self.scalar_static_f64[460]+self.scalar_static_f64[462]);
        self.scalar_static_f64[464]=p.p806;
        self.scalar_static_f64[465]=(self.scalar_static_f64[464]/self.scalar_static_f64[126]);
        self.scalar_static_f64[466]=(self.scalar_static_f64[463]+self.scalar_static_f64[465]);
        self.scalar_static_f64[467]=p.p83;
        self.scalar_static_f64[468]=p.p632;
        self.scalar_static_f64[469]=(self.scalar_static_f64[468]/self.scalar_static_f64[123]);
        self.scalar_static_f64[470]=(self.scalar_static_f64[467]+self.scalar_static_f64[469]);
        self.scalar_static_f64[471]=p.p720;
        self.scalar_static_f64[472]=(self.scalar_static_f64[471]/self.scalar_static_f64[125]);
        self.scalar_static_f64[473]=(self.scalar_static_f64[470]+self.scalar_static_f64[472]);
        self.scalar_static_f64[474]=p.p808;
        self.scalar_static_f64[475]=(self.scalar_static_f64[474]/self.scalar_static_f64[126]);
        self.scalar_static_f64[476]=(self.scalar_static_f64[473]+self.scalar_static_f64[475]);
        self.scalar_static_f64[477]=p.p62;
        self.scalar_static_f64[478]=p.p634;
        self.scalar_static_f64[479]=(self.scalar_static_f64[478]/self.scalar_static_f64[123]);
        self.scalar_static_f64[480]=(self.scalar_static_f64[477]+self.scalar_static_f64[479]);
        self.scalar_static_f64[481]=p.p722;
        self.scalar_static_f64[482]=(self.scalar_static_f64[481]/self.scalar_static_f64[125]);
        self.scalar_static_f64[483]=(self.scalar_static_f64[480]+self.scalar_static_f64[482]);
        self.scalar_static_f64[484]=p.p810;
        self.scalar_static_f64[485]=(self.scalar_static_f64[484]/self.scalar_static_f64[126]);
        self.scalar_static_f64[486]=(self.scalar_static_f64[483]+self.scalar_static_f64[485]);
        self.scalar_static_f64[487]=p.p59;
        self.scalar_static_f64[488]=p.p635;
        self.scalar_static_f64[489]=(self.scalar_static_f64[488]/self.scalar_static_f64[123]);
        self.scalar_static_f64[490]=(self.scalar_static_f64[487]+self.scalar_static_f64[489]);
        self.scalar_static_f64[491]=p.p723;
        self.scalar_static_f64[492]=(self.scalar_static_f64[491]/self.scalar_static_f64[125]);
        self.scalar_static_f64[493]=(self.scalar_static_f64[490]+self.scalar_static_f64[492]);
        self.scalar_static_f64[494]=p.p811;
        self.scalar_static_f64[495]=(self.scalar_static_f64[494]/self.scalar_static_f64[126]);
        self.scalar_static_f64[496]=(self.scalar_static_f64[493]+self.scalar_static_f64[495]);
        self.scalar_static_f64[497]=p.p60;
        self.scalar_static_f64[498]=p.p636;
        self.scalar_static_f64[499]=(self.scalar_static_f64[498]/self.scalar_static_f64[123]);
        self.scalar_static_f64[500]=(self.scalar_static_f64[497]+self.scalar_static_f64[499]);
        self.scalar_static_f64[501]=p.p724;
        self.scalar_static_f64[502]=(self.scalar_static_f64[501]/self.scalar_static_f64[125]);
        self.scalar_static_f64[503]=(self.scalar_static_f64[500]+self.scalar_static_f64[502]);
        self.scalar_static_f64[504]=p.p812;
        self.scalar_static_f64[505]=(self.scalar_static_f64[504]/self.scalar_static_f64[126]);
        self.scalar_static_f64[506]=(self.scalar_static_f64[503]+self.scalar_static_f64[505]);
        self.scalar_static_f64[507]=p.p85;
        self.scalar_static_f64[508]=p.p637;
        self.scalar_static_f64[509]=(self.scalar_static_f64[508]/self.scalar_static_f64[123]);
        self.scalar_static_f64[510]=(self.scalar_static_f64[507]+self.scalar_static_f64[509]);
        self.scalar_static_f64[511]=p.p725;
        self.scalar_static_f64[512]=(self.scalar_static_f64[511]/self.scalar_static_f64[125]);
        self.scalar_static_f64[513]=(self.scalar_static_f64[510]+self.scalar_static_f64[512]);
        self.scalar_static_f64[514]=p.p813;
        self.scalar_static_f64[515]=(self.scalar_static_f64[514]/self.scalar_static_f64[126]);
        self.scalar_static_f64[516]=(self.scalar_static_f64[513]+self.scalar_static_f64[515]);
        self.scalar_static_f64[517]=p.p82;
        self.scalar_static_f64[518]=p.p638;
        self.scalar_static_f64[519]=(self.scalar_static_f64[518]/self.scalar_static_f64[123]);
        self.scalar_static_f64[520]=(self.scalar_static_f64[517]+self.scalar_static_f64[519]);
        self.scalar_static_f64[521]=p.p726;
        self.scalar_static_f64[522]=(self.scalar_static_f64[521]/self.scalar_static_f64[125]);
        self.scalar_static_f64[523]=(self.scalar_static_f64[520]+self.scalar_static_f64[522]);
        self.scalar_static_f64[524]=p.p814;
        self.scalar_static_f64[525]=(self.scalar_static_f64[524]/self.scalar_static_f64[126]);
        self.scalar_static_f64[526]=(self.scalar_static_f64[523]+self.scalar_static_f64[525]);
        self.scalar_static_f64[527]=p.p61;
        self.scalar_static_f64[528]=p.p639;
        self.scalar_static_f64[529]=(self.scalar_static_f64[528]/self.scalar_static_f64[123]);
        self.scalar_static_f64[530]=(self.scalar_static_f64[527]+self.scalar_static_f64[529]);
        self.scalar_static_f64[531]=p.p727;
        self.scalar_static_f64[532]=(self.scalar_static_f64[531]/self.scalar_static_f64[125]);
        self.scalar_static_f64[533]=(self.scalar_static_f64[530]+self.scalar_static_f64[532]);
        self.scalar_static_f64[534]=p.p815;
        self.scalar_static_f64[535]=(self.scalar_static_f64[534]/self.scalar_static_f64[126]);
        self.scalar_static_f64[536]=(self.scalar_static_f64[533]+self.scalar_static_f64[535]);
        self.scalar_static_f64[537]=p.p75;
        self.scalar_static_f64[538]=p.p640;
        self.scalar_static_f64[539]=(self.scalar_static_f64[538]/self.scalar_static_f64[123]);
        self.scalar_static_f64[540]=(self.scalar_static_f64[537]+self.scalar_static_f64[539]);
        self.scalar_static_f64[541]=p.p728;
        self.scalar_static_f64[542]=(self.scalar_static_f64[541]/self.scalar_static_f64[125]);
        self.scalar_static_f64[543]=(self.scalar_static_f64[540]+self.scalar_static_f64[542]);
        self.scalar_static_f64[544]=p.p816;
        self.scalar_static_f64[545]=(self.scalar_static_f64[544]/self.scalar_static_f64[126]);
        self.scalar_static_f64[546]=(self.scalar_static_f64[543]+self.scalar_static_f64[545]);
        self.scalar_static_f64[547]=p.p80;
        self.scalar_static_f64[548]=p.p641;
        self.scalar_static_f64[549]=(self.scalar_static_f64[548]/self.scalar_static_f64[123]);
        self.scalar_static_f64[550]=(self.scalar_static_f64[547]+self.scalar_static_f64[549]);
        self.scalar_static_f64[551]=p.p729;
        self.scalar_static_f64[552]=(self.scalar_static_f64[551]/self.scalar_static_f64[125]);
        self.scalar_static_f64[553]=(self.scalar_static_f64[550]+self.scalar_static_f64[552]);
        self.scalar_static_f64[554]=p.p817;
        self.scalar_static_f64[555]=(self.scalar_static_f64[554]/self.scalar_static_f64[126]);
        self.scalar_static_f64[556]=(self.scalar_static_f64[553]+self.scalar_static_f64[555]);
        self.scalar_static_f64[557]=p.p493;
        self.scalar_static_f64[558]=p.p824;
        self.scalar_static_f64[559]=(self.scalar_static_f64[558]/self.scalar_static_f64[123]);
        self.scalar_static_f64[560]=(self.scalar_static_f64[557]+self.scalar_static_f64[559]);
        self.scalar_static_f64[561]=p.p839;
        self.scalar_static_f64[562]=(self.scalar_static_f64[561]/self.scalar_static_f64[125]);
        self.scalar_static_f64[563]=(self.scalar_static_f64[560]+self.scalar_static_f64[562]);
        self.scalar_static_f64[564]=p.p854;
        self.scalar_static_f64[565]=(self.scalar_static_f64[564]/self.scalar_static_f64[126]);
        self.scalar_static_f64[566]=(self.scalar_static_f64[563]+self.scalar_static_f64[565]);
        self.scalar_static_f64[567]=p.p494;
        self.scalar_static_f64[568]=p.p825;
        self.scalar_static_f64[569]=(self.scalar_static_f64[568]/self.scalar_static_f64[123]);
        self.scalar_static_f64[570]=(self.scalar_static_f64[567]+self.scalar_static_f64[569]);
        self.scalar_static_f64[571]=p.p840;
        self.scalar_static_f64[572]=(self.scalar_static_f64[571]/self.scalar_static_f64[125]);
        self.scalar_static_f64[573]=(self.scalar_static_f64[570]+self.scalar_static_f64[572]);
        self.scalar_static_f64[574]=p.p855;
        self.scalar_static_f64[575]=(self.scalar_static_f64[574]/self.scalar_static_f64[126]);
        self.scalar_static_f64[576]=(self.scalar_static_f64[573]+self.scalar_static_f64[575]);
        self.scalar_static_f64[577]=p.p496;
        self.scalar_static_f64[578]=p.p826;
        self.scalar_static_f64[579]=(self.scalar_static_f64[578]/self.scalar_static_f64[123]);
        self.scalar_static_f64[580]=(self.scalar_static_f64[577]+self.scalar_static_f64[579]);
        self.scalar_static_f64[581]=p.p841;
        self.scalar_static_f64[582]=(self.scalar_static_f64[581]/self.scalar_static_f64[125]);
        self.scalar_static_f64[583]=(self.scalar_static_f64[580]+self.scalar_static_f64[582]);
        self.scalar_static_f64[584]=p.p856;
        self.scalar_static_f64[585]=(self.scalar_static_f64[584]/self.scalar_static_f64[126]);
        self.scalar_static_f64[586]=(self.scalar_static_f64[583]+self.scalar_static_f64[585]);
        self.scalar_static_f64[587]=p.p516;
        self.scalar_static_f64[588]=p.p829;
        self.scalar_static_f64[589]=(self.scalar_static_f64[588]/self.scalar_static_f64[123]);
        self.scalar_static_f64[590]=(self.scalar_static_f64[587]+self.scalar_static_f64[589]);
        self.scalar_static_f64[591]=p.p844;
        self.scalar_static_f64[592]=(self.scalar_static_f64[591]/self.scalar_static_f64[125]);
        self.scalar_static_f64[593]=(self.scalar_static_f64[590]+self.scalar_static_f64[592]);
        self.scalar_static_f64[594]=p.p859;
        self.scalar_static_f64[595]=(self.scalar_static_f64[594]/self.scalar_static_f64[126]);
        self.scalar_static_f64[596]=(self.scalar_static_f64[593]+self.scalar_static_f64[595]);
        self.scalar_static_f64[597]=p.p517;
        self.scalar_static_f64[598]=p.p830;
        self.scalar_static_f64[599]=(self.scalar_static_f64[598]/self.scalar_static_f64[123]);
        self.scalar_static_f64[600]=(self.scalar_static_f64[597]+self.scalar_static_f64[599]);
        self.scalar_static_f64[601]=p.p845;
        self.scalar_static_f64[602]=(self.scalar_static_f64[601]/self.scalar_static_f64[125]);
        self.scalar_static_f64[603]=(self.scalar_static_f64[600]+self.scalar_static_f64[602]);
        self.scalar_static_f64[604]=p.p860;
        self.scalar_static_f64[605]=(self.scalar_static_f64[604]/self.scalar_static_f64[126]);
        self.scalar_static_f64[606]=(self.scalar_static_f64[603]+self.scalar_static_f64[605]);
        self.scalar_static_f64[607]=p.p519;
        self.scalar_static_f64[608]=p.p831;
        self.scalar_static_f64[609]=(self.scalar_static_f64[608]/self.scalar_static_f64[123]);
        self.scalar_static_f64[610]=(self.scalar_static_f64[607]+self.scalar_static_f64[609]);
        self.scalar_static_f64[611]=p.p846;
        self.scalar_static_f64[612]=(self.scalar_static_f64[611]/self.scalar_static_f64[125]);
        self.scalar_static_f64[613]=(self.scalar_static_f64[610]+self.scalar_static_f64[612]);
        self.scalar_static_f64[614]=p.p861;
        self.scalar_static_f64[615]=(self.scalar_static_f64[614]/self.scalar_static_f64[126]);
        self.scalar_static_f64[616]=(self.scalar_static_f64[613]+self.scalar_static_f64[615]);
        self.scalar_static_f64[617]=(if self.scalar_static_bool[18]{1.0}else{0.0});
        self.scalar_static_f64[618]=p.p342;
        self.scalar_static_f64[619]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[618]);
        self.scalar_static_f64[620]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[619]}else{0.0});
        self.scalar_static_f64[621]=p.p341;
        self.scalar_static_f64[622]=(self.scalar_static_f64[621]/self.scalar_static_f64[620]);
        self.scalar_static_f64[623]=(1.0+self.scalar_static_f64[622]);
        self.scalar_static_f64[624]=(self.scalar_static_f64[101]*self.scalar_static_f64[623]);
        self.scalar_static_f64[625]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[624]}else{self.scalar_static_f64[101]});
        self.scalar_static_bool[19]=(self.scalar_static_f64[625]<1e21);
        self.scalar_static_f64[626]=(if self.scalar_static_bool[19]{1.0}else{0.0});
        self.scalar_static_bool[20]=((self.scalar_static_f64[617]!=0.0)&&(self.scalar_static_f64[626]!=0.0));
        self.scalar_static_f64[627]=(if self.scalar_static_bool[20]{1e21}else{self.scalar_static_f64[625]});
        self.scalar_static_f64[628]=p.p369;
        self.scalar_static_f64[629]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[628]);
        self.scalar_static_f64[630]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[629]}else{self.scalar_static_f64[620]});
        self.scalar_static_f64[631]=p.p362;
        self.scalar_static_f64[632]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[631]);
        self.scalar_static_f64[633]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[632]}else{self.scalar_static_f64[630]});
        self.scalar_static_f64[634]=p.p348;
        self.scalar_static_f64[635]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[634]);
        self.scalar_static_f64[636]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[635]}else{self.scalar_static_f64[633]});
        self.scalar_static_f64[637]=p.p351;
        self.scalar_static_f64[638]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[637]);
        self.scalar_static_f64[639]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[638]}else{self.scalar_static_f64[636]});
        self.scalar_static_f64[640]=p.p357;
        self.scalar_static_f64[641]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[640]);
        self.scalar_static_f64[642]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[641]}else{self.scalar_static_f64[639]});
        self.scalar_static_f64[643]=p.p359;
        self.scalar_static_f64[644]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[643]);
        self.scalar_static_f64[645]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[644]}else{self.scalar_static_f64[642]});
        self.scalar_static_f64[646]=p.p373;
        self.scalar_static_f64[647]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[646]);
        self.scalar_static_f64[648]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[647]}else{self.scalar_static_f64[645]});
        self.scalar_static_f64[649]=p.p375;
        self.scalar_static_f64[650]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[649]);
        self.scalar_static_f64[651]=(if (self.scalar_static_f64[617]!=0.0){self.scalar_static_f64[650]}else{self.scalar_static_f64[648]});
        self.scalar_static_bool[21]=(!(self.scalar_static_f64[617]!=0.0));
        self.scalar_static_f64[652]=(if self.scalar_static_bool[21]{0.0}else{self.scalar_static_f64[627]});
        self.scalar_static_bool[22]=(0.0==self.scalar_static_f64[71]);
        self.scalar_static_f64[653]=(if self.scalar_static_bool[22]{1.0}else{0.0});
        self.scalar_static_f64[654]=p.p267;
        self.scalar_static_bool[23]=(1.0==self.scalar_static_f64[3]);
        self.scalar_static_f64[655]=p.p19;
        self.scalar_static_bool[24]=(self.scalar_static_f64[655]>0.0);
        self.scalar_static_bool[25]=(0.0==self.scalar_static_f64[166]);
        self.scalar_static_bool[26]=(self.scalar_static_bool[24]&&self.scalar_static_bool[25]);
        self.scalar_static_f64[656]=p.p18;
        self.scalar_static_bool[27]=(self.scalar_static_f64[656]>0.0);
        self.scalar_static_bool[28]=(0.0==self.scalar_static_f64[176]);
        self.scalar_static_bool[29]=(self.scalar_static_bool[27]&&self.scalar_static_bool[28]);
        self.scalar_static_bool[30]=(self.scalar_static_bool[26]||self.scalar_static_bool[29]);
        self.scalar_static_bool[31]=(self.scalar_static_bool[23]&&self.scalar_static_bool[30]);
        self.scalar_static_f64[657]=(if self.scalar_static_bool[31]{1.0}else{0.0});
        self.scalar_static_bool[32]=(!(self.scalar_static_f64[657]!=0.0));
        self.scalar_static_f64[658]=(if self.scalar_static_bool[32]{self.scalar_static_f64[3]}else{0.0});
        self.scalar_static_bool[33]=(1.0==self.scalar_static_f64[658]);
        self.scalar_static_f64[659]=(self.scalar_static_f64[51]*self.scalar_static_f64[506]);
        self.scalar_static_f64[660]=(1000000.0*self.scalar_static_f64[659]);
        self.scalar_static_f64[661]=(self.scalar_static_f64[486]+self.scalar_static_f64[660]);
        self.scalar_static_f64[662]=(self.scalar_static_f64[53]*self.scalar_static_f64[67]);
        self.scalar_static_f64[663]=(1000000.0*self.scalar_static_f64[662]);
        self.scalar_static_f64[664]=(self.scalar_static_f64[68]+self.scalar_static_f64[663]);
        self.scalar_static_f64[665]=(self.scalar_static_f64[52]*self.scalar_static_f64[506]);
        self.scalar_static_f64[666]=(1000000.0*self.scalar_static_f64[665]);
        self.scalar_static_f64[667]=(self.scalar_static_f64[486]+self.scalar_static_f64[666]);
        self.scalar_static_f64[668]=(self.scalar_static_f64[54]*self.scalar_static_f64[67]);
        self.scalar_static_f64[669]=(1000000.0*self.scalar_static_f64[668]);
        self.scalar_static_f64[670]=(self.scalar_static_f64[68]+self.scalar_static_f64[669]);
        self.scalar_static_f64[671]=p.p12;
        self.scalar_static_f64[672]=(self.scalar_static_f64[671]/1e-6);
        self.scalar_static_f64[673]=(self.scalar_static_f64[206]/1e-6);
        self.scalar_static_f64[674]=(self.scalar_static_f64[216]/1e-6);
        self.scalar_static_f64[675]=(self.scalar_static_f64[166]/1e-6);
        self.scalar_static_f64[676]=(self.scalar_static_f64[376]/100.0);
        self.scalar_static_f64[677]=(self.scalar_static_f64[356]/100.0);
        self.scalar_static_f64[678]=(10000.0*self.scalar_static_f64[406]);
        self.scalar_static_f64[679]=(self.scalar_static_f64[416]/100.0);
        self.scalar_static_f64[680]=(100.0*self.scalar_static_f64[436]);
        self.scalar_static_f64[681]=(100.0*self.scalar_static_f64[326]);
        self.scalar_static_f64[682]=(self.scalar_static_f64[446]/1e-6);
        self.scalar_static_f64[683]=(self.scalar_static_f64[476]/100.0);
        self.scalar_static_f64[684]=(self.scalar_static_f64[526]/100.0);
        self.scalar_static_f64[685]=(self.scalar_static_f64[556]/100.0);
        self.scalar_static_f64[686]=p.p28;
        self.scalar_static_bool[34]=(0.0==self.scalar_static_f64[29]);
        self.scalar_static_bool[35]=(0.0==self.scalar_static_f64[31]);
        self.scalar_static_bool[36]=(self.scalar_static_bool[34]&&self.scalar_static_bool[35]);
        self.scalar_static_bool[37]=(0.0==self.scalar_static_f64[30]);
        self.scalar_static_bool[38]=(self.scalar_static_bool[36]||self.scalar_static_bool[37]);
        self.scalar_static_f64[687]=(if self.scalar_static_bool[38]{1.0}else{0.0});
        self.scalar_static_bool[39]=(!(self.scalar_static_f64[687]!=0.0));
        self.scalar_static_f64[688]=(if self.scalar_static_bool[39]{1.0}else{0.0});
        self.scalar_static_f64[689]=(self.scalar_static_f64[120]*self.scalar_static_f64[121]);
        self.scalar_static_f64[690]=(1e-7*self.scalar_static_f64[104]);
        self.scalar_static_f64[691]=(9.025e-5+self.scalar_static_f64[690]);
        self.scalar_static_f64[692]=(self.scalar_static_f64[104]*self.scalar_static_f64[691]);
        self.scalar_static_f64[693]=(self.scalar_static_f64[156]-self.scalar_static_f64[692]);
        self.scalar_static_f64[694]=(8.8541878e-12*self.scalar_static_f64[654]);
        self.scalar_static_bool[40]=(0.0==self.scalar_static_f64[286]);
        self.scalar_static_f64[695]=(if self.scalar_static_bool[40]{1.0}else{0.0});
        self.scalar_static_bool[41]=(!(self.scalar_static_f64[695]!=0.0));
        self.scalar_static_f64[696]=(if self.scalar_static_bool[41]{1.0}else{0.0});
        self.scalar_static_f64[697]=(1.0/self.scalar_static_f64[120]);
        self.scalar_static_f64[698]=(1.0+self.scalar_static_f64[697]);
        self.scalar_static_f64[699]=f64::powf(self.scalar_static_f64[698],self.scalar_static_f64[23]);
        self.scalar_static_f64[700]=(self.scalar_static_f64[286]*self.scalar_static_f64[699]);
        self.scalar_static_f64[701]=(if self.scalar_static_bool[41]{self.scalar_static_f64[700]}else{0.0});
        self.scalar_static_f64[702]=p.p118;
        self.scalar_static_f64[703]=(0.5*self.scalar_static_f64[105]);
        self.scalar_static_f64[704]=(self.scalar_static_f64[702]+self.scalar_static_f64[703]);
        self.scalar_static_f64[705]=(1.0/self.scalar_static_f64[704]);
        self.scalar_static_f64[706]=p.p119;
        self.scalar_static_f64[707]=(self.scalar_static_f64[703]+self.scalar_static_f64[706]);
        self.scalar_static_f64[708]=(1.0/self.scalar_static_f64[707]);
        self.scalar_static_f64[709]=(self.scalar_static_f64[705]+self.scalar_static_f64[708]);
        self.scalar_static_f64[710]=(2.0/self.scalar_static_f64[709]);
        self.scalar_static_f64[711]=p.p8;
        self.scalar_static_bool[42]=(self.scalar_static_f64[711]>0.0);
        self.scalar_static_f64[712]=p.p9;
        self.scalar_static_bool[43]=(self.scalar_static_f64[712]>0.0);
        self.scalar_static_bool[44]=(self.scalar_static_bool[42]&&self.scalar_static_bool[43]);
        self.scalar_static_bool[45]=(1.0==self.scalar_static_f64[107]);
        self.scalar_static_bool[46]=(self.scalar_static_f64[107]>1.0);
        self.scalar_static_f64[713]=p.p10;
        self.scalar_static_bool[47]=(self.scalar_static_f64[713]>0.0);
        self.scalar_static_bool[48]=(self.scalar_static_bool[46]&&self.scalar_static_bool[47]);
        self.scalar_static_bool[49]=(self.scalar_static_bool[45]||self.scalar_static_bool[48]);
        self.scalar_static_bool[50]=(self.scalar_static_bool[44]&&self.scalar_static_bool[49]);
        self.scalar_static_f64[714]=(if self.scalar_static_bool[50]{1.0}else{0.0});
        self.scalar_static_f64[715]=(if (self.scalar_static_f64[714]!=0.0){0.0}else{self.scalar_static_f64[709]});
        self.scalar_static_f64[716]=(2.0*self.scalar_static_f64[107]);
        self.scalar_static_bool[51]=(!(self.scalar_static_f64[714]!=0.0));
        self.scalar_static_f64[717]=p.p32;
        self.scalar_static_bool[52]=(1.0==self.scalar_static_f64[717]);
        self.scalar_static_bool[53]=((self.scalar_static_f64[1]!=0.0)&&self.scalar_static_bool[52]);
        self.scalar_static_f64[718]=(if self.scalar_static_bool[53]{1.0}else{0.0});
        self.scalar_static_f64[719]=(self.scalar_static_f64[672]+self.scalar_static_f64[674]);
        self.scalar_static_f64[720]=(self.scalar_static_f64[719]-self.scalar_static_f64[673]);
        self.scalar_static_f64[721]=(if (self.scalar_static_f64[718]!=0.0){self.scalar_static_f64[720]}else{self.scalar_static_f64[674]});
        self.scalar_static_f64[722]=(self.scalar_static_f64[672]+self.scalar_static_f64[682]);
        self.scalar_static_f64[723]=(self.scalar_static_f64[722]-self.scalar_static_f64[673]);
        self.scalar_static_f64[724]=(if (self.scalar_static_f64[718]!=0.0){self.scalar_static_f64[723]}else{self.scalar_static_f64[682]});
        self.scalar_static_f64[725]=(if (self.scalar_static_f64[718]!=0.0){self.scalar_static_f64[672]}else{self.scalar_static_f64[673]});
        self.scalar_static_f64[726]=(self.scalar_static_f64[677]/self.scalar_static_f64[710]);
        self.scalar_static_f64[727]=f64::powf(self.scalar_static_f64[726],self.scalar_static_f64[366]);
        self.scalar_static_f64[728]=f64::powf(self.scalar_static_f64[116],self.scalar_static_f64[83]);
        self.scalar_static_f64[729]=(self.scalar_static_f64[85]/self.scalar_static_f64[728]);
        self.scalar_static_f64[730]=(self.scalar_static_f64[16]+self.scalar_static_f64[729]);
        self.scalar_static_f64[731]=(self.scalar_static_f64[14]+self.scalar_static_f64[729]);
        self.scalar_static_f64[732]=f64::powf(self.scalar_static_f64[118],self.scalar_static_f64[87]);
        self.scalar_static_f64[733]=(self.scalar_static_f64[89]/self.scalar_static_f64[732]);
        self.scalar_static_f64[734]=(self.scalar_static_f64[17]+self.scalar_static_f64[733]);
        self.scalar_static_f64[735]=(self.scalar_static_f64[730]+self.scalar_static_f64[731]);
        self.scalar_static_f64[736]=(self.scalar_static_f64[110]-self.scalar_static_f64[735]);
        self.scalar_static_f64[737]=p.p127;
        self.scalar_static_f64[738]=f64::powf(self.scalar_static_f64[689],self.scalar_static_f64[737]);
        self.scalar_static_f64[739]=(self.scalar_static_f64[186]/self.scalar_static_f64[738]);
        self.scalar_static_f64[740]=(2.0*self.scalar_static_f64[734]);
        self.scalar_static_f64[741]=(self.scalar_static_f64[112]-self.scalar_static_f64[740]);
        self.scalar_static_f64[742]=(self.scalar_static_f64[107]*self.scalar_static_f64[741]);
        self.scalar_static_f64[743]=p.p142;
        self.scalar_static_f64[744]=p.p143;
        self.scalar_static_f64[745]=f64::powf(self.scalar_static_f64[121],self.scalar_static_f64[744]);
        self.scalar_static_f64[746]=(self.scalar_static_f64[743]/self.scalar_static_f64[745]);
        self.scalar_static_f64[747]=(1.0+self.scalar_static_f64[746]);
        self.scalar_static_f64[748]=(self.scalar_static_f64[721]*self.scalar_static_f64[747]);
        self.scalar_static_f64[749]=p.p233;
        self.scalar_static_f64[750]=p.p234;
        self.scalar_static_f64[751]=f64::powf(self.scalar_static_f64[121],self.scalar_static_f64[750]);
        self.scalar_static_f64[752]=(self.scalar_static_f64[749]/self.scalar_static_f64[751]);
        self.scalar_static_f64[753]=(1.0+self.scalar_static_f64[752]);
        self.scalar_static_f64[754]=(self.scalar_static_f64[725]*self.scalar_static_f64[753]);
        self.scalar_static_f64[755]=(1e-6*self.scalar_static_f64[754]);
        self.scalar_static_f64[756]=(1e-6*self.scalar_static_f64[748]);
        self.scalar_static_bool[54]=(self.scalar_static_f64[755]<1000000000000000.0);
        self.scalar_static_f64[757]=(if self.scalar_static_bool[54]{1.0}else{0.0});
        self.scalar_static_f64[758]=(if (self.scalar_static_f64[757]!=0.0){1000000000000000.0}else{self.scalar_static_f64[755]});
        self.scalar_static_f64[759]=(self.scalar_static_f64[758]/1e-6);
        self.scalar_static_bool[55]=(self.scalar_static_f64[756]<1000000000000000.0);
        self.scalar_static_f64[760]=(if self.scalar_static_bool[55]{1.0}else{0.0});
        self.scalar_static_f64[761]=(if (self.scalar_static_f64[760]!=0.0){1000000000000000.0}else{self.scalar_static_f64[756]});
        self.scalar_static_f64[762]=(self.scalar_static_f64[761]/1e-6);
        self.scalar_static_f64[763]=(1.0+self.scalar_static_f64[386]);
        self.scalar_static_f64[764]=(1.0/self.scalar_static_f64[763]);
        self.scalar_static_f64[765]=(self.scalar_static_f64[676]/self.scalar_static_f64[710]);
        self.scalar_static_f64[766]=f64::powf(self.scalar_static_f64[765],self.scalar_static_f64[396]);
        self.scalar_static_bool[56]=(self.scalar_static_f64[110]>self.scalar_static_f64[21]);
        self.scalar_static_bool[57]=(self.scalar_static_f64[21]<=0.0);
        self.scalar_static_bool[58]=(self.scalar_static_bool[56]||self.scalar_static_bool[57]);
        self.scalar_static_f64[767]=(if self.scalar_static_bool[58]{1.0}else{0.0});
        self.scalar_static_f64[768]=(self.scalar_static_f64[110]-self.scalar_static_f64[21]);
        self.scalar_static_f64[769]=(self.scalar_static_f64[759]*self.scalar_static_f64[768]);
        self.scalar_static_bool[59]=(!(self.scalar_static_f64[767]!=0.0));
        self.scalar_static_f64[770]=(self.scalar_static_f64[21]-self.scalar_static_f64[110]);
        self.scalar_static_f64[771]=(0.5*self.scalar_static_f64[110]);
        self.scalar_static_f64[772]=(self.scalar_static_f64[771]-self.scalar_static_f64[21]);
        self.scalar_static_f64[773]=(self.scalar_static_f64[772]-1e-9);
        self.scalar_static_f64[774]=(self.scalar_static_f64[773]-1e-10);
        self.scalar_static_f64[775]=(self.scalar_static_f64[774]*self.scalar_static_f64[774]);
        self.scalar_static_f64[776]=(4.0000000000000004e-19+self.scalar_static_f64[775]);
        self.scalar_static_f64[777]=(self.scalar_static_f64[776]).sqrt();
        self.scalar_static_f64[778]=(self.scalar_static_f64[774]+self.scalar_static_f64[777]);
        self.scalar_static_f64[779]=(0.5*self.scalar_static_f64[778]);
        self.scalar_static_f64[780]=(1e-9+self.scalar_static_f64[779]);
        self.scalar_static_f64[781]=(1.0/self.scalar_static_f64[780]);
        self.scalar_static_f64[782]=(1.0/self.scalar_static_f64[24]);
        self.scalar_static_f64[783]=(self.scalar_static_f64[781]+self.scalar_static_f64[782]);
        self.scalar_static_f64[784]=(1.0/self.scalar_static_f64[783]);
        self.scalar_static_bool[60]=(0.0>=self.scalar_static_f64[784]);
        self.scalar_static_f64[785]=(if self.scalar_static_bool[60]{0.0}else{self.scalar_static_f64[784]});
        self.scalar_static_f64[786]=(self.scalar_static_f64[724]-self.scalar_static_f64[759]);
        self.scalar_static_f64[787]=(self.scalar_static_f64[785]*self.scalar_static_f64[786]);
        self.scalar_static_f64[788]=(self.scalar_static_f64[787]/self.scalar_static_f64[110]);
        self.scalar_static_f64[789]=(self.scalar_static_f64[21]*2.0);
        self.scalar_static_bool[61]=(self.scalar_static_f64[110]<=self.scalar_static_f64[789]);
        self.scalar_static_bool[62]=(self.scalar_static_f64[21]>0.0);
        self.scalar_static_bool[63]=(self.scalar_static_bool[61]&&self.scalar_static_bool[62]);
        self.scalar_static_f64[790]=(if self.scalar_static_bool[63]{1.0}else{0.0});
        self.scalar_static_bool[64]=(!(self.scalar_static_f64[790]!=0.0));
        self.scalar_static_f64[791]=(self.scalar_static_f64[759]/1.04e16);
        self.scalar_static_f64[792]=(self.scalar_static_f64[791]).ln();
        self.scalar_static_f64[793]=(0.051702525384001115*self.scalar_static_f64[792]);
        self.scalar_static_f64[794]=p.p51;
        self.scalar_static_bool[65]=(1.0==self.scalar_static_f64[794]);
        self.scalar_static_f64[795]=(if self.scalar_static_bool[65]{1.0}else{0.0});
        self.scalar_static_f64[796]=p.p5;
        self.scalar_static_f64[797]=p.p4;
        self.scalar_static_f64[798]=(3.0*self.scalar_static_f64[797]);
        self.scalar_static_f64[799]=(self.scalar_static_f64[741]/self.scalar_static_f64[798]);
        self.scalar_static_f64[800]=(self.scalar_static_f64[796]+self.scalar_static_f64[799]);
        self.scalar_static_f64[801]=(if (self.scalar_static_f64[795]!=0.0){self.scalar_static_f64[800]}else{self.scalar_static_f64[784]});
        self.scalar_static_f64[802]=(if (self.scalar_static_f64[795]!=0.0){self.scalar_static_f64[114]}else{self.scalar_static_f64[785]});
        self.scalar_static_f64[803]=(self.scalar_static_f64[18]*self.scalar_static_f64[801]);
        self.scalar_static_f64[804]=(self.scalar_static_f64[797]*self.scalar_static_f64[802]);
        self.scalar_static_f64[805]=(self.scalar_static_f64[107]*self.scalar_static_f64[804]);
        self.scalar_static_f64[806]=(self.scalar_static_f64[803]/self.scalar_static_f64[805]);
        self.scalar_static_f64[807]=(if (self.scalar_static_f64[795]!=0.0){self.scalar_static_f64[806]}else{0.0});
        self.scalar_static_bool[66]=(self.scalar_static_f64[807]>0.001);
        self.scalar_static_f64[808]=(if self.scalar_static_bool[66]{1.0}else{0.0});
        self.scalar_static_bool[67]=((self.scalar_static_f64[795]!=0.0)&&(self.scalar_static_f64[808]!=0.0));
        self.scalar_static_f64[809]=(1.0/self.scalar_static_f64[807]);
        self.scalar_static_f64[810]=(if self.scalar_static_bool[67]{self.scalar_static_f64[809]}else{self.scalar_static_f64[807]});
        self.scalar_static_bool[68]=(!(self.scalar_static_f64[808]!=0.0));
        self.scalar_static_bool[69]=((self.scalar_static_f64[795]!=0.0)&&self.scalar_static_bool[68]);
        self.scalar_static_f64[811]=(if self.scalar_static_bool[69]{1000.0}else{self.scalar_static_f64[810]});
        self.scalar_static_bool[70]=(!(self.scalar_static_f64[795]!=0.0));
        self.scalar_static_f64[812]=p.p444;
        self.scalar_static_f64[813]=(1.0/self.scalar_static_f64[812]);
        self.scalar_static_f64[814]=(if self.scalar_static_bool[70]{self.scalar_static_f64[813]}else{self.scalar_static_f64[811]});
        self.scalar_static_bool[71]=(0.0==self.scalar_static_f64[658]);
        self.scalar_static_f64[815]=(if self.scalar_static_bool[71]{1.0}else{0.0});
        self.scalar_static_bool[72]=(self.scalar_static_f64[466]>0.0);
        self.scalar_static_bool[73]=(self.scalar_static_f64[546]>0.0);
        self.scalar_static_bool[74]=(self.scalar_static_bool[72]||self.scalar_static_bool[73]);
        self.scalar_static_f64[816]=(if self.scalar_static_bool[74]{1.0}else{0.0});
        self.scalar_static_bool[75]=((self.scalar_static_f64[815]!=0.0)&&(self.scalar_static_f64[816]!=0.0));
        self.scalar_static_f64[817]=f64::powf(self.scalar_static_f64[689],self.scalar_static_f64[56]);
        self.scalar_static_f64[818]=(self.scalar_static_f64[55]/self.scalar_static_f64[817]);
        self.scalar_static_f64[819]=(1.0+self.scalar_static_f64[818]);
        self.scalar_static_f64[820]=(if self.scalar_static_bool[75]{self.scalar_static_f64[819]}else{0.0});
        self.scalar_static_bool[76]=(0.0!=self.scalar_static_f64[684]);
        self.scalar_static_f64[821]=(if self.scalar_static_bool[76]{1.0}else{0.0});
        self.scalar_static_bool[77]=(self.scalar_static_bool[75]&&(self.scalar_static_f64[821]!=0.0));
        self.scalar_static_f64[822]=f64::powf(self.scalar_static_f64[689],self.scalar_static_f64[60]);
        self.scalar_static_f64[823]=(self.scalar_static_f64[59]/self.scalar_static_f64[822]);
        self.scalar_static_f64[824]=(1.0+self.scalar_static_f64[823]);
        self.scalar_static_f64[825]=(if self.scalar_static_bool[77]{self.scalar_static_f64[824]}else{0.0});
        self.scalar_static_f64[826]=(-self.scalar_static_f64[57]);
        self.scalar_static_f64[827]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[58]);
        self.scalar_static_f64[828]=(self.scalar_static_f64[826]*self.scalar_static_f64[827]);
        self.scalar_static_f64[829]=(if self.scalar_static_bool[77]{self.scalar_static_f64[828]}else{0.0});
        self.scalar_static_bool[78]=(self.scalar_static_f64[829]>60.0);
        self.scalar_static_f64[830]=(if self.scalar_static_bool[78]{1.0}else{0.0});
        self.scalar_static_bool[79]=(self.scalar_static_bool[77]&&(self.scalar_static_f64[830]!=0.0));
        self.scalar_static_f64[831]=(if self.scalar_static_bool[79]{60.0}else{self.scalar_static_f64[829]});
        self.scalar_static_f64[832]=(self.scalar_static_f64[831]).exp();
        self.scalar_static_f64[833]=(if self.scalar_static_bool[77]{self.scalar_static_f64[832]}else{self.scalar_static_f64[831]});
        self.scalar_static_f64[834]=(self.scalar_static_f64[825]*self.scalar_static_f64[833]);
        self.scalar_static_f64[835]=(if self.scalar_static_bool[77]{self.scalar_static_f64[834]}else{0.0});
        self.scalar_static_bool[80]=(!(self.scalar_static_f64[821]!=0.0));
        self.scalar_static_bool[81]=(self.scalar_static_bool[75]&&self.scalar_static_bool[80]);
        self.scalar_static_f64[836]=(if self.scalar_static_bool[81]{0.0}else{self.scalar_static_f64[835]});
        self.scalar_static_bool[82]=(!(self.scalar_static_f64[816]!=0.0));
        self.scalar_static_bool[83]=((self.scalar_static_f64[815]!=0.0)&&self.scalar_static_bool[82]);
        self.scalar_static_f64[837]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[820]});
        self.scalar_static_f64[838]=(if self.scalar_static_bool[83]{0.0}else{self.scalar_static_f64[836]});
        self.scalar_static_bool[84]=(0.0!=self.scalar_static_f64[683]);
        self.scalar_static_f64[839]=(if self.scalar_static_bool[84]{1.0}else{0.0});
        self.scalar_static_bool[85]=((self.scalar_static_f64[815]!=0.0)&&(self.scalar_static_f64[839]!=0.0));
        self.scalar_static_f64[840]=f64::powf(self.scalar_static_f64[689],self.scalar_static_f64[64]);
        self.scalar_static_f64[841]=(self.scalar_static_f64[63]/self.scalar_static_f64[840]);
        self.scalar_static_f64[842]=(1.0+self.scalar_static_f64[841]);
        self.scalar_static_f64[843]=(if self.scalar_static_bool[85]{self.scalar_static_f64[842]}else{self.scalar_static_f64[802]});
        self.scalar_static_f64[844]=(-self.scalar_static_f64[61]);
        self.scalar_static_f64[845]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[62]);
        self.scalar_static_f64[846]=(self.scalar_static_f64[844]*self.scalar_static_f64[845]);
        self.scalar_static_f64[847]=(if self.scalar_static_bool[85]{self.scalar_static_f64[846]}else{self.scalar_static_f64[801]});
        self.scalar_static_bool[86]=(self.scalar_static_f64[847]>60.0);
        self.scalar_static_f64[848]=(if self.scalar_static_bool[86]{1.0}else{0.0});
        self.scalar_static_bool[87]=(self.scalar_static_bool[85]&&(self.scalar_static_f64[848]!=0.0));
        self.scalar_static_f64[849]=(if self.scalar_static_bool[87]{60.0}else{self.scalar_static_f64[847]});
        self.scalar_static_f64[850]=(self.scalar_static_f64[849]).exp();
        self.scalar_static_f64[851]=(if self.scalar_static_bool[85]{self.scalar_static_f64[850]}else{self.scalar_static_f64[849]});
        self.scalar_static_f64[852]=(self.scalar_static_f64[683]*self.scalar_static_f64[843]);
        self.scalar_static_f64[853]=(self.scalar_static_f64[851]*self.scalar_static_f64[852]);
        self.scalar_static_f64[854]=(if self.scalar_static_bool[85]{self.scalar_static_f64[853]}else{self.scalar_static_f64[780]});
        self.scalar_static_bool[88]=(!(self.scalar_static_f64[815]!=0.0));
        self.scalar_static_f64[855]=(if self.scalar_static_bool[88]{0.0}else{self.scalar_static_f64[837]});
        self.scalar_static_f64[856]=(if self.scalar_static_bool[88]{0.0}else{self.scalar_static_f64[838]});
        self.scalar_static_f64[857]=p.p52;
        self.scalar_static_bool[89]=(1.0==self.scalar_static_f64[857]);
        self.scalar_static_f64[858]=(if self.scalar_static_bool[89]{1.0}else{0.0});
        self.scalar_static_f64[859]=p.p56;
        self.scalar_static_bool[90]=(self.scalar_static_f64[859]<0.001);
        self.scalar_static_f64[860]=(if self.scalar_static_bool[90]{1.0}else{0.0});
        self.scalar_static_bool[91]=((self.scalar_static_f64[858]!=0.0)&&(self.scalar_static_f64[860]!=0.0));
        self.scalar_static_f64[861]=(if self.scalar_static_bool[91]{1000.0}else{0.0});
        self.scalar_static_bool[92]=(!(self.scalar_static_f64[860]!=0.0));
        self.scalar_static_bool[93]=((self.scalar_static_f64[858]!=0.0)&&self.scalar_static_bool[92]);
        self.scalar_static_f64[862]=p.p277;
        self.scalar_static_f64[863]=(1.0/self.scalar_static_f64[859]);
        self.scalar_static_f64[864]=(self.scalar_static_f64[862]+self.scalar_static_f64[863]);
        self.scalar_static_f64[865]=(if self.scalar_static_bool[93]{self.scalar_static_f64[864]}else{self.scalar_static_f64[861]});
        self.scalar_static_f64[866]=p.p58;
        self.scalar_static_bool[94]=(self.scalar_static_f64[866]<0.001);
        self.scalar_static_f64[867]=(if self.scalar_static_bool[94]{1.0}else{0.0});
        self.scalar_static_bool[95]=((self.scalar_static_f64[858]!=0.0)&&(self.scalar_static_f64[867]!=0.0));
        self.scalar_static_f64[868]=(if self.scalar_static_bool[95]{1000.0}else{0.0});
        self.scalar_static_bool[96]=(!(self.scalar_static_f64[867]!=0.0));
        self.scalar_static_bool[97]=((self.scalar_static_f64[858]!=0.0)&&self.scalar_static_bool[96]);
        self.scalar_static_f64[869]=(1.0/self.scalar_static_f64[866]);
        self.scalar_static_f64[870]=(self.scalar_static_f64[862]+self.scalar_static_f64[869]);
        self.scalar_static_f64[871]=(if self.scalar_static_bool[97]{self.scalar_static_f64[870]}else{self.scalar_static_f64[868]});
        self.scalar_static_f64[872]=p.p57;
        self.scalar_static_bool[98]=(self.scalar_static_f64[872]<0.001);
        self.scalar_static_f64[873]=(if self.scalar_static_bool[98]{1.0}else{0.0});
        self.scalar_static_bool[99]=((self.scalar_static_f64[858]!=0.0)&&(self.scalar_static_f64[873]!=0.0));
        self.scalar_static_f64[874]=(if self.scalar_static_bool[99]{1000.0}else{0.0});
        self.scalar_static_bool[100]=(!(self.scalar_static_f64[873]!=0.0));
        self.scalar_static_bool[101]=((self.scalar_static_f64[858]!=0.0)&&self.scalar_static_bool[100]);
        self.scalar_static_f64[875]=(1.0/self.scalar_static_f64[872]);
        self.scalar_static_f64[876]=(self.scalar_static_f64[862]+self.scalar_static_f64[875]);
        self.scalar_static_f64[877]=(if self.scalar_static_bool[101]{self.scalar_static_f64[876]}else{self.scalar_static_f64[874]});
        self.scalar_static_bool[102]=(!(self.scalar_static_f64[858]!=0.0));
        self.scalar_static_f64[878]=(if self.scalar_static_bool[102]{1000.0}else{self.scalar_static_f64[865]});
        self.scalar_static_f64[879]=(if self.scalar_static_bool[102]{self.scalar_static_f64[878]}else{self.scalar_static_f64[871]});
        self.scalar_static_f64[880]=(if self.scalar_static_bool[102]{self.scalar_static_f64[878]}else{self.scalar_static_f64[877]});
        self.scalar_static_f64[881]=p.p44;
        self.scalar_static_bool[103]=(0.0==self.scalar_static_f64[881]);
        self.scalar_static_f64[882]=(if self.scalar_static_bool[103]{1.0}else{0.0});
        self.scalar_static_f64[883]=(self.scalar_static_f64[36]*self.scalar_static_f64[120]);
        self.scalar_static_f64[884]=(self.scalar_static_f64[35]+self.scalar_static_f64[883]);
        self.scalar_static_f64[885]=(if (self.scalar_static_f64[882]!=0.0){self.scalar_static_f64[884]}else{self.scalar_static_f64[851]});
        self.scalar_static_bool[104]=(self.scalar_static_f64[885]<0.0);
        self.scalar_static_f64[886]=(if self.scalar_static_bool[104]{1.0}else{0.0});
        self.scalar_static_bool[105]=((self.scalar_static_f64[882]!=0.0)&&(self.scalar_static_f64[886]!=0.0));
        self.scalar_static_f64[887]=(if self.scalar_static_bool[105]{0.0}else{self.scalar_static_f64[885]});
        self.scalar_static_f64[888]=(self.scalar_static_f64[34]*self.scalar_static_f64[887]);
        self.scalar_static_f64[889]=(self.scalar_static_f64[34]+self.scalar_static_f64[887]);
        self.scalar_static_f64[890]=(self.scalar_static_f64[888]/self.scalar_static_f64[889]);
        self.scalar_static_f64[891]=(1.0+self.scalar_static_f64[890]);
        self.scalar_static_f64[892]=(if (self.scalar_static_f64[882]!=0.0){self.scalar_static_f64[891]}else{0.0});
        self.scalar_static_bool[106]=(!(self.scalar_static_f64[882]!=0.0));
        self.scalar_static_f64[893]=(if self.scalar_static_bool[106]{self.scalar_static_f64[883]}else{self.scalar_static_f64[887]});
        self.scalar_static_bool[107]=(self.scalar_static_f64[893]<0.0);
        self.scalar_static_f64[894]=(if self.scalar_static_bool[107]{1.0}else{0.0});
        self.scalar_static_bool[108]=(self.scalar_static_bool[106]&&(self.scalar_static_f64[894]!=0.0));
        self.scalar_static_f64[895]=(if self.scalar_static_bool[108]{0.0}else{self.scalar_static_f64[893]});
        self.scalar_static_f64[896]=(self.scalar_static_f64[34]*self.scalar_static_f64[895]);
        self.scalar_static_f64[897]=(self.scalar_static_f64[34]+self.scalar_static_f64[895]);
        self.scalar_static_f64[898]=(self.scalar_static_f64[896]/self.scalar_static_f64[897]);
        self.scalar_static_f64[899]=(self.scalar_static_f64[35]+self.scalar_static_f64[898]);
        self.scalar_static_f64[900]=(1e-25+self.scalar_static_f64[899]);
        self.scalar_static_f64[901]=(if self.scalar_static_bool[106]{self.scalar_static_f64[900]}else{self.scalar_static_f64[892]});
        self.scalar_static_bool[109]=(self.scalar_static_f64[901]<0.1);
        self.scalar_static_f64[902]=(if self.scalar_static_bool[109]{1.0}else{0.0});
        self.scalar_static_f64[903]=(if (self.scalar_static_f64[902]!=0.0){0.1}else{self.scalar_static_f64[901]});
        self.scalar_static_f64[904]=p.p23;
        self.scalar_static_f64[905]=f64::powf(self.scalar_static_f64[741],self.scalar_static_f64[90]);
        self.scalar_static_f64[906]=(if (self.scalar_static_f64[904]!=0.0){self.scalar_static_f64[905]}else{self.scalar_static_f64[843]});
        self.scalar_static_f64[907]=(self.scalar_static_f64[685]/self.scalar_static_f64[742]);
        self.scalar_static_f64[908]=f64::powf(self.scalar_static_f64[120],self.scalar_static_f64[47]);
        self.scalar_static_f64[909]=(self.scalar_static_f64[46]/self.scalar_static_f64[908]);
        self.scalar_static_f64[910]=(1.0+self.scalar_static_f64[909]);
        self.scalar_static_f64[911]=(self.scalar_static_f64[907]*self.scalar_static_f64[910]);
        self.scalar_static_f64[912]=f64::powf(self.scalar_static_f64[121],self.scalar_static_f64[45]);
        self.scalar_static_f64[913]=(self.scalar_static_f64[44]/self.scalar_static_f64[912]);
        self.scalar_static_f64[914]=(1.0+self.scalar_static_f64[913]);
        self.scalar_static_f64[915]=(self.scalar_static_f64[911]*self.scalar_static_f64[914]);
        self.scalar_static_f64[916]=(self.scalar_static_f64[100]*self.scalar_static_f64[742]);
        self.scalar_static_f64[917]=f64::powf(self.scalar_static_f64[107],self.scalar_static_f64[48]);
        self.scalar_static_f64[918]=(1.0/self.scalar_static_f64[917]);
        self.scalar_static_f64[919]=(self.scalar_static_f64[915]*self.scalar_static_f64[918]);
        self.scalar_static_f64[920]=(self.scalar_static_f64[918]/self.scalar_static_f64[742]);
        self.scalar_static_f64[921]=(self.scalar_static_f64[910]*self.scalar_static_f64[920]);
        self.scalar_static_f64[922]=(self.scalar_static_f64[914]*self.scalar_static_f64[921]);
        self.scalar_static_f64[923]=p.p53;
        self.scalar_static_bool[110]=(0.0==self.scalar_static_f64[923]);
        self.scalar_static_bool[111]=(0.0==self.scalar_static_f64[685]);
        self.scalar_static_bool[112]=(self.scalar_static_bool[110]||self.scalar_static_bool[111]);
        self.scalar_static_f64[924]=(if self.scalar_static_bool[112]{1.0}else{0.0});
        self.scalar_static_f64[925]=p.p11;
        self.scalar_static_f64[926]=(self.scalar_static_f64[104]*self.scalar_static_f64[104]);
        self.scalar_static_f64[927]=(1.0/self.scalar_static_f64[104]);
        self.scalar_static_f64[928]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[927]}else{self.scalar_static_f64[906]});
        self.scalar_static_f64[929]=p.p259;
        self.scalar_static_f64[930]=(self.scalar_static_f64[693]+self.scalar_static_f64[929]);
        self.scalar_static_f64[931]=p.p260;
        self.scalar_static_f64[932]=p.p261;
        self.scalar_static_f64[933]=(self.scalar_static_f64[928]*self.scalar_static_f64[928]);
        self.scalar_static_f64[934]=(self.scalar_static_f64[104]*1.3806226e-23);
        self.scalar_static_f64[935]=(1.6021918e-19/self.scalar_static_f64[934]);
        self.scalar_static_f64[936]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[935]}else{0.0});
        self.scalar_static_f64[937]=(self.scalar_static_f64[693]/2.0);
        self.scalar_static_f64[938]=(self.scalar_static_f64[936]*self.scalar_static_f64[937]);
        self.scalar_static_bool[113]=(self.scalar_static_bool[4]&&self.scalar_static_bool[18]);
        self.scalar_static_f64[939]=(if self.scalar_static_bool[113]{1.0}else{0.0});
        self.scalar_static_bool[114]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[939]!=0.0));
        self.scalar_static_f64[940]=(self.scalar_static_f64[652]*self.scalar_static_f64[759]);
        self.scalar_static_f64[941]=p.p380;
        self.scalar_static_bool[115]=(!(self.scalar_static_f64[939]!=0.0));
        self.scalar_static_bool[116]=((self.scalar_static_f64[924]!=0.0)&&self.scalar_static_bool[115]);
        self.scalar_static_bool[117]=((self.scalar_static_f64[77]!=0.0)&&self.scalar_static_bool[116]);
        self.scalar_static_bool[118]=(!(self.scalar_static_f64[77]!=0.0));
        self.scalar_static_bool[119]=(self.scalar_static_bool[116]&&self.scalar_static_bool[118]);
        self.scalar_static_f64[942]=p.p39;
        self.scalar_static_bool[120]=(2.0!=self.scalar_static_f64[942]);
        self.scalar_static_f64[943]=(if self.scalar_static_bool[120]{1.0}else{0.0});
        self.scalar_static_bool[121]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[943]!=0.0));
        self.scalar_static_bool[122]=(!(self.scalar_static_f64[943]!=0.0));
        self.scalar_static_bool[123]=((self.scalar_static_f64[924]!=0.0)&&self.scalar_static_bool[122]);
        self.scalar_static_f64[944]=p.p390;
        self.scalar_static_f64[945]=p.p391;
        self.scalar_static_bool[124]=((self.scalar_static_f64[923]!=0.0)&&(self.scalar_static_f64[924]!=0.0));
        self.scalar_static_f64[946]=(if (self.scalar_static_f64[924]!=0.0){0.0}else{self.scalar_static_f64[777]});
        self.scalar_static_bool[125]=(self.scalar_static_f64[946]>0.0);
        self.scalar_static_f64[947]=(-self.scalar_static_f64[946]);
        self.scalar_static_f64[948]=(if self.scalar_static_bool[125]{self.scalar_static_f64[946]}else{self.scalar_static_f64[947]});
        self.scalar_static_f64[949]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[948]}else{self.scalar_static_f64[946]});
        self.scalar_static_bool[126]=((self.scalar_static_f64[653]!=0.0)&&(self.scalar_static_f64[924]!=0.0));
        self.scalar_static_f64[950]=(if self.scalar_static_bool[72]{1.0}else{0.0});
        self.scalar_static_bool[127]=((self.scalar_static_f64[815]!=0.0)&&(self.scalar_static_f64[924]!=0.0));
        self.scalar_static_bool[128]=((self.scalar_static_f64[950]!=0.0)&&self.scalar_static_bool[127]);
        self.scalar_static_f64[951]=(self.scalar_static_f64[661]*self.scalar_static_f64[855]);
        self.scalar_static_f64[952]=(self.scalar_static_f64[664]*self.scalar_static_f64[951]);
        self.scalar_static_bool[129]=(1.0==self.scalar_static_f64[942]);
        self.scalar_static_f64[953]=(if self.scalar_static_bool[129]{1.0}else{0.0});
        self.scalar_static_bool[130]=(self.scalar_static_bool[128]&&(self.scalar_static_f64[953]!=0.0));
        self.scalar_static_f64[954]=(self.scalar_static_f64[466]*0.005);
        self.scalar_static_f64[955]=(self.scalar_static_f64[466]*0.01);
        self.scalar_static_f64[956]=(4.0*self.scalar_static_f64[954]);
        self.scalar_static_f64[957]=(self.scalar_static_f64[955]*self.scalar_static_f64[956]);
        self.scalar_static_bool[131]=(!(self.scalar_static_f64[953]!=0.0));
        self.scalar_static_bool[132]=(self.scalar_static_bool[128]&&self.scalar_static_bool[131]);
        self.scalar_static_bool[133]=(!(self.scalar_static_f64[950]!=0.0));
        self.scalar_static_bool[134]=(self.scalar_static_bool[127]&&self.scalar_static_bool[133]);
        self.scalar_static_f64[958]=(if self.scalar_static_bool[73]{1.0}else{0.0});
        self.scalar_static_bool[135]=(self.scalar_static_bool[127]&&(self.scalar_static_f64[958]!=0.0));
        self.scalar_static_f64[959]=(self.scalar_static_f64[667]*self.scalar_static_f64[855]);
        self.scalar_static_f64[960]=(self.scalar_static_f64[670]*self.scalar_static_f64[959]);
        self.scalar_static_bool[136]=((self.scalar_static_f64[953]!=0.0)&&self.scalar_static_bool[135]);
        self.scalar_static_f64[961]=(self.scalar_static_f64[546]*0.005);
        self.scalar_static_f64[962]=(self.scalar_static_f64[546]*0.01);
        self.scalar_static_f64[963]=(4.0*self.scalar_static_f64[961]);
        self.scalar_static_f64[964]=(self.scalar_static_f64[962]*self.scalar_static_f64[963]);
        self.scalar_static_bool[137]=(self.scalar_static_bool[131]&&self.scalar_static_bool[135]);
        self.scalar_static_bool[138]=(!(self.scalar_static_f64[958]!=0.0));
        self.scalar_static_bool[139]=(self.scalar_static_bool[127]&&self.scalar_static_bool[138]);
        self.scalar_static_bool[140]=(self.scalar_static_f64[684]>0.0);
        self.scalar_static_f64[965]=(if self.scalar_static_bool[140]{1.0}else{0.0});
        self.scalar_static_bool[141]=(self.scalar_static_bool[127]&&(self.scalar_static_f64[965]!=0.0));
        self.scalar_static_f64[966]=(self.scalar_static_f64[661]*self.scalar_static_f64[856]);
        self.scalar_static_f64[967]=(self.scalar_static_f64[664]*self.scalar_static_f64[966]);
        self.scalar_static_f64[968]=(if self.scalar_static_bool[141]{self.scalar_static_f64[967]}else{0.0});
        self.scalar_static_f64[969]=(1.0-self.scalar_static_f64[496]);
        self.scalar_static_f64[970]=p.p63;
        self.scalar_static_f64[971]=(self.scalar_static_f64[969]*self.scalar_static_f64[970]);
        self.scalar_static_f64[972]=(1000000.0*self.scalar_static_f64[971]);
        self.scalar_static_f64[973]=(self.scalar_static_f64[66]*self.scalar_static_f64[66]);
        self.scalar_static_f64[974]=(self.scalar_static_f64[973]+4.0000000000000015e-12);
        self.scalar_static_f64[975]=(self.scalar_static_f64[974]).sqrt();
        self.scalar_static_f64[976]=(-self.scalar_static_f64[65]);
        self.scalar_static_f64[977]=(4.0*self.scalar_static_f64[968]);
        self.scalar_static_f64[978]=(0.01*self.scalar_static_f64[977]);
        self.scalar_static_f64[979]=(1.0+self.scalar_static_f64[65]);
        self.scalar_static_f64[980]=(self.scalar_static_f64[968]*self.scalar_static_f64[979]);
        self.scalar_static_f64[981]=(4.0*self.scalar_static_f64[980]);
        self.scalar_static_f64[982]=(5e-5*self.scalar_static_f64[981]);
        self.scalar_static_bool[142]=(0.0==self.scalar_static_f64[942]);
        self.scalar_static_bool[143]=(self.scalar_static_bool[129]||self.scalar_static_bool[142]);
        self.scalar_static_f64[983]=(if self.scalar_static_bool[143]{1.0}else{0.0});
        self.scalar_static_bool[144]=(self.scalar_static_bool[141]&&(self.scalar_static_f64[983]!=0.0));
        self.scalar_static_f64[984]=(self.scalar_static_f64[684]*0.005);
        self.scalar_static_f64[985]=(0.01*self.scalar_static_f64[684]);
        self.scalar_static_f64[986]=(4.0*self.scalar_static_f64[984]);
        self.scalar_static_f64[987]=(self.scalar_static_f64[985]*self.scalar_static_f64[986]);
        self.scalar_static_bool[145]=(!(self.scalar_static_f64[983]!=0.0));
        self.scalar_static_bool[146]=(self.scalar_static_bool[141]&&self.scalar_static_bool[145]);
        self.scalar_static_f64[988]=(self.scalar_static_f64[667]*self.scalar_static_f64[856]);
        self.scalar_static_f64[989]=(self.scalar_static_f64[670]*self.scalar_static_f64[988]);
        self.scalar_static_f64[990]=(if self.scalar_static_bool[141]{self.scalar_static_f64[989]}else{self.scalar_static_f64[968]});
        self.scalar_static_f64[991]=p.p66;
        self.scalar_static_f64[992]=(self.scalar_static_f64[969]*self.scalar_static_f64[991]);
        self.scalar_static_f64[993]=(1000000.0*self.scalar_static_f64[992]);
        self.scalar_static_f64[994]=(4.0*self.scalar_static_f64[990]);
        self.scalar_static_f64[995]=(0.01*self.scalar_static_f64[994]);
        self.scalar_static_f64[996]=(self.scalar_static_f64[979]*self.scalar_static_f64[990]);
        self.scalar_static_f64[997]=(4.0*self.scalar_static_f64[996]);
        self.scalar_static_f64[998]=(5e-5*self.scalar_static_f64[997]);
        self.scalar_static_bool[147]=(!(self.scalar_static_f64[965]!=0.0));
        self.scalar_static_bool[148]=(self.scalar_static_bool[127]&&self.scalar_static_bool[147]);
        self.scalar_static_bool[149]=(self.scalar_static_f64[658]>0.0);
        self.scalar_static_f64[999]=(if self.scalar_static_bool[149]{1.0}else{0.0});
        self.scalar_static_bool[150]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[999]!=0.0));
        self.scalar_static_f64[1000]=p.p416;
        self.scalar_static_f64[1001]=(self.scalar_static_f64[693]*self.scalar_static_f64[936]);
        self.scalar_static_f64[1002]=p.p499;
        self.scalar_static_f64[1003]=p.p497;
        self.scalar_static_f64[1004]=p.p495;
        self.scalar_static_f64[1005]=p.p498;
        self.scalar_static_f64[1006]=p.p48;
        self.scalar_static_bool[151]=(self.scalar_static_f64[1006]>0.0);
        self.scalar_static_f64[1007]=(if self.scalar_static_bool[151]{1.0}else{0.0});
        self.scalar_static_f64[1008]=p.p15;
        self.scalar_static_bool[152]=(self.scalar_static_f64[1008]>self.scalar_static_f64[742]);
        self.scalar_static_f64[1009]=(if self.scalar_static_bool[152]{1.0}else{0.0});
        self.scalar_static_bool[153]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[1007]!=0.0));
        self.scalar_static_bool[154]=((self.scalar_static_f64[1009]!=0.0)&&self.scalar_static_bool[153]);
        self.scalar_static_f64[1010]=p.p13;
        self.scalar_static_f64[1011]=(self.scalar_static_f64[1008]-self.scalar_static_f64[742]);
        self.scalar_static_bool[155]=(!(self.scalar_static_f64[1009]!=0.0));
        self.scalar_static_bool[156]=(self.scalar_static_bool[153]&&self.scalar_static_bool[155]);
        self.scalar_static_bool[157]=(!(self.scalar_static_f64[1007]!=0.0));
        self.scalar_static_bool[158]=((self.scalar_static_f64[924]!=0.0)&&self.scalar_static_bool[157]);
        self.scalar_static_f64[1012]=p.p522;
        self.scalar_static_f64[1013]=p.p520;
        self.scalar_static_f64[1014]=p.p518;
        self.scalar_static_f64[1015]=p.p521;
        self.scalar_static_f64[1016]=p.p16;
        self.scalar_static_bool[159]=(self.scalar_static_f64[1016]>self.scalar_static_f64[742]);
        self.scalar_static_f64[1017]=(if self.scalar_static_bool[159]{1.0}else{0.0});
        self.scalar_static_bool[160]=(self.scalar_static_bool[153]&&(self.scalar_static_f64[1017]!=0.0));
        self.scalar_static_f64[1018]=p.p14;
        self.scalar_static_f64[1019]=(self.scalar_static_f64[1016]-self.scalar_static_f64[742]);
        self.scalar_static_bool[161]=(!(self.scalar_static_f64[1017]!=0.0));
        self.scalar_static_bool[162]=(self.scalar_static_bool[153]&&self.scalar_static_bool[161]);
        self.scalar_static_f64[1020]=p.p87;
        self.scalar_static_bool[163]=(!(self.scalar_static_f64[686]!=0.0));
        self.scalar_static_bool[164]=(self.scalar_static_f64[923]>0.0);
        self.scalar_static_bool[165]=(0.0!=self.scalar_static_f64[685]);
        self.scalar_static_bool[166]=(self.scalar_static_bool[164]&&self.scalar_static_bool[165]);
        self.scalar_static_f64[1021]=(if self.scalar_static_bool[166]{1.0}else{0.0});
        self.scalar_static_bool[167]=(2.0==self.scalar_static_f64[923]);
        self.scalar_static_f64[1022]=(if self.scalar_static_bool[167]{1.0}else{0.0});
        self.scalar_static_bool[168]=((self.scalar_static_f64[1021]!=0.0)&&(self.scalar_static_f64[1022]!=0.0));
        self.scalar_static_f64[1023]=p.p337;
        self.scalar_static_f64[1024]=(10.0*self.scalar_static_f64[1023]);
        self.scalar_static_f64[1025]=(self.scalar_static_f64[15]*4.0);
        self.scalar_static_f64[1026]=(self.scalar_static_f64[1024]*self.scalar_static_f64[1025]);
        self.scalar_static_f64[1027]=(if (self.scalar_static_f64[1021]!=0.0){self.scalar_static_f64[935]}else{self.scalar_static_f64[936]});
        self.scalar_static_f64[1028]=(self.scalar_static_f64[937]*self.scalar_static_f64[1027]);
        self.scalar_static_bool[169]=((self.scalar_static_f64[939]!=0.0)&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[170]=(self.scalar_static_bool[115]&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[171]=((self.scalar_static_f64[77]!=0.0)&&self.scalar_static_bool[170]);
        self.scalar_static_bool[172]=(self.scalar_static_bool[118]&&self.scalar_static_bool[170]);
        self.scalar_static_bool[173]=((self.scalar_static_f64[943]!=0.0)&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[174]=(self.scalar_static_bool[122]&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[175]=((self.scalar_static_f64[923]!=0.0)&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[176]=((self.scalar_static_f64[653]!=0.0)&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[177]=((self.scalar_static_f64[815]!=0.0)&&(self.scalar_static_f64[1021]!=0.0));
        self.scalar_static_bool[178]=((self.scalar_static_f64[950]!=0.0)&&self.scalar_static_bool[177]);
        self.scalar_static_bool[179]=((self.scalar_static_f64[953]!=0.0)&&self.scalar_static_bool[178]);
        self.scalar_static_bool[180]=(self.scalar_static_bool[131]&&self.scalar_static_bool[178]);
        self.scalar_static_bool[181]=(self.scalar_static_bool[133]&&self.scalar_static_bool[177]);
        self.scalar_static_bool[182]=((self.scalar_static_f64[958]!=0.0)&&self.scalar_static_bool[177]);
        self.scalar_static_bool[183]=((self.scalar_static_f64[953]!=0.0)&&self.scalar_static_bool[182]);
        self.scalar_static_bool[184]=(self.scalar_static_bool[131]&&self.scalar_static_bool[182]);
        self.scalar_static_bool[185]=(self.scalar_static_bool[138]&&self.scalar_static_bool[177]);
        self.scalar_static_bool[186]=((self.scalar_static_f64[965]!=0.0)&&self.scalar_static_bool[177]);
        self.scalar_static_f64[1029]=(if self.scalar_static_bool[186]{self.scalar_static_f64[967]}else{self.scalar_static_f64[990]});
        self.scalar_static_f64[1030]=(4.0*self.scalar_static_f64[1029]);
        self.scalar_static_f64[1031]=(0.01*self.scalar_static_f64[1030]);
        self.scalar_static_f64[1032]=(self.scalar_static_f64[979]*self.scalar_static_f64[1029]);
        self.scalar_static_f64[1033]=(4.0*self.scalar_static_f64[1032]);
        self.scalar_static_f64[1034]=(5e-5*self.scalar_static_f64[1033]);
        self.scalar_static_bool[187]=((self.scalar_static_f64[983]!=0.0)&&self.scalar_static_bool[186]);
        self.scalar_static_bool[188]=(self.scalar_static_bool[145]&&self.scalar_static_bool[186]);
        self.scalar_static_f64[1035]=(if self.scalar_static_bool[186]{self.scalar_static_f64[989]}else{self.scalar_static_f64[1029]});
        self.scalar_static_f64[1036]=(4.0*self.scalar_static_f64[1035]);
        self.scalar_static_f64[1037]=(0.01*self.scalar_static_f64[1036]);
        self.scalar_static_f64[1038]=(self.scalar_static_f64[979]*self.scalar_static_f64[1035]);
        self.scalar_static_f64[1039]=(4.0*self.scalar_static_f64[1038]);
        self.scalar_static_f64[1040]=(5e-5*self.scalar_static_f64[1039]);
        self.scalar_static_bool[189]=(self.scalar_static_bool[147]&&self.scalar_static_bool[177]);
        self.scalar_static_f64[1041]=(self.scalar_static_f64[694]/self.scalar_static_f64[8]);
        self.scalar_static_f64[1042]=(1.0/self.scalar_static_f64[1041]);
        self.scalar_static_f64[1043]=p.p262;
        self.scalar_static_f64[1044]=(self.scalar_static_f64[793]-self.scalar_static_f64[1043]);
        self.scalar_static_bool[190]=(self.scalar_static_f64[71]<=3.0);
        self.scalar_static_bool[191]=(self.scalar_static_bool[5]&&self.scalar_static_bool[190]);
        self.scalar_static_f64[1045]=(if self.scalar_static_bool[191]{1.0}else{0.0});
        self.scalar_static_f64[1046]=if param_given[338]{1.0}else{0.0};
        self.scalar_static_f64[1047]=p.p338;
        self.scalar_static_f64[1048]=(if (self.scalar_static_f64[1046]!=0.0){self.scalar_static_f64[1047]}else{0.0});
        self.scalar_static_bool[192]=(!(self.scalar_static_f64[1046]!=0.0));
        self.scalar_static_f64[1049]=if param_given[339]{1.0}else{0.0};
        self.scalar_static_f64[1050]=p.p339;
        self.scalar_static_f64[1051]=(if (self.scalar_static_f64[1049]!=0.0){self.scalar_static_f64[1050]}else{0.0});
        self.scalar_static_bool[193]=(!(self.scalar_static_f64[1049]!=0.0));
        self.scalar_static_bool[194]=((self.scalar_static_f64[1046]!=0.0)&&self.scalar_static_bool[193]);
        self.scalar_static_bool[195]=(self.scalar_static_bool[192]&&self.scalar_static_bool[193]);
        self.scalar_static_bool[196]=(1.0==self.scalar_static_f64[5]);
        self.scalar_static_f64[1052]=(if self.scalar_static_bool[196]{1.0}else{0.0});
        self.scalar_static_bool[197]=(self.scalar_static_f64[5]==2.0);
        self.scalar_static_f64[1053]=(if self.scalar_static_bool[197]{1.0}else{0.0});
        self.scalar_static_bool[198]=(3.0==self.scalar_static_f64[5]);
        self.scalar_static_f64[1054]=(if self.scalar_static_bool[198]{1.0}else{0.0});
        self.scalar_static_bool[199]=(self.scalar_static_bool[0]&&self.scalar_static_bool[33]);
        self.scalar_static_f64[1055]=(self.scalar_static_f64[96]+self.scalar_static_f64[675]);
        self.scalar_static_f64[1056]=(self.scalar_static_f64[675]*self.scalar_static_f64[1055]);
        self.scalar_static_bool[200]=(self.scalar_static_f64[1056]>0.0);
        self.scalar_static_bool[201]=(self.scalar_static_bool[199]&&self.scalar_static_bool[200]);
        self.scalar_static_f64[1057]=(if self.scalar_static_bool[201]{1.0}else{0.0});
        self.scalar_static_f64[1058]=p.p335;
        self.scalar_static_f64[1059]=p.p333;
        self.scalar_static_f64[1060]=p.p332;
        self.scalar_static_f64[1061]=(self.scalar_static_f64[96]/self.scalar_static_f64[1056]);
        self.scalar_static_f64[1062]=(self.scalar_static_f64[2]*0.1);
        self.scalar_static_f64[1063]=(self.scalar_static_f64[2]*4.0);
        self.scalar_static_f64[1064]=(self.scalar_static_f64[1062]*self.scalar_static_f64[1063]);
        self.scalar_static_bool[202]=(!(self.scalar_static_f64[1057]!=0.0));
        self.scalar_static_bool[203]=(self.scalar_static_f64[536]>0.0);
        self.scalar_static_bool[204]=(self.scalar_static_f64[516]>0.0);
        self.scalar_static_f64[1065]=p.p34;
        self.scalar_static_bool[205]=(1.0==self.scalar_static_f64[1065]);
        self.scalar_static_f64[1066]=(self.scalar_static_f64[41]*self.scalar_static_f64[41]);
        self.scalar_static_f64[1067]=(4.0000000000000015e-12+self.scalar_static_f64[1066]);
        self.scalar_static_f64[1068]=(self.scalar_static_f64[1067]).sqrt();
        self.scalar_static_f64[1069]=(1.0+self.scalar_static_f64[536]);
        self.scalar_static_f64[1070]=(2.0*self.scalar_static_f64[1043]);
        self.scalar_static_f64[1071]=(self.scalar_static_f64[41]+1e-25);
        self.scalar_static_bool[206]=(0.0==self.scalar_static_f64[1065]);
        self.scalar_static_bool[207]=(self.scalar_static_bool[0]&&self.scalar_static_bool[206]);
        self.scalar_static_bool[208]=(self.scalar_static_bool[200]&&self.scalar_static_bool[207]);
        self.scalar_static_f64[1072]=(if self.scalar_static_bool[208]{1.0}else{0.0});
        self.scalar_static_f64[1073]=(1.0/self.scalar_static_f64[1027]);
        self.scalar_static_bool[209]=(0.0==self.scalar_static_f64[688]);
        self.scalar_static_f64[1074]=(if self.scalar_static_bool[209]{1.0}else{0.0});
        self.scalar_static_bool[210]=(!(self.scalar_static_f64[1074]!=0.0));
        self.scalar_static_f64[1075]=(if self.scalar_static_bool[210]{1.0}else{0.0});
        self.scalar_static_bool[211]=(0.0==self.scalar_static_f64[1075]);
        self.scalar_static_f64[1076]=(if self.scalar_static_bool[211]{1.0}else{0.0});
        self.scalar_static_f64[1077]=(if (self.scalar_static_f64[1076]!=0.0){self.scalar_static_f64[8]}else{0.0});
        self.scalar_static_f64[1078]=(if (self.scalar_static_f64[1076]!=0.0){self.scalar_static_f64[1041]}else{0.0});
        self.scalar_static_f64[1079]=(if (self.scalar_static_f64[1076]!=0.0){self.scalar_static_f64[1042]}else{0.0});
        self.scalar_static_bool[212]=(!(self.scalar_static_f64[1076]!=0.0));
        self.scalar_static_bool[213]=(self.scalar_static_f64[71]>1.0);
        self.scalar_static_f64[1080]=(if self.scalar_static_bool[213]{0.0}else{1.0});
        self.scalar_static_bool[214]=(1.0==self.scalar_static_f64[71]);
        self.scalar_static_f64[1081]=p.p366;
        self.scalar_static_bool[215]=(0.0!=self.scalar_static_f64[21]);
        self.scalar_static_f64[1082]=(if self.scalar_static_bool[215]{1.0}else{0.0});
        self.scalar_static_f64[1083]=(self.scalar_static_f64[759]*3.2043836e-19);
        self.scalar_static_f64[1084]=(1.034943e-10*self.scalar_static_f64[1083]);
        self.scalar_static_f64[1085]=(self.scalar_static_f64[196]+self.scalar_static_f64[793]);
        self.scalar_static_f64[1086]=(self.scalar_static_f64[21]*self.scalar_static_f64[21]);
        self.scalar_static_f64[1087]=(1.0/self.scalar_static_f64[1086]);
        self.scalar_static_bool[216]=(!(self.scalar_static_f64[1082]!=0.0));
        self.scalar_static_f64[1088]=(self.scalar_static_f64[110]-self.scalar_static_f64[20]);
        self.scalar_static_f64[1089]=(self.scalar_static_f64[1088]*self.scalar_static_f64[1088]);
        self.scalar_static_f64[1090]=(1.0/self.scalar_static_f64[1089]);
        self.scalar_static_f64[1091]=(self.scalar_static_f64[276]/self.scalar_static_f64[110]);
        self.scalar_static_f64[1092]=(self.scalar_static_f64[678]/self.scalar_static_f64[741]);
        self.scalar_static_f64[1093]=(self.scalar_static_f64[296]/self.scalar_static_f64[121]);
        self.scalar_static_bool[217]=(1.0==self.scalar_static_f64[696]);
        self.scalar_static_f64[1094]=(if self.scalar_static_bool[217]{1.0}else{0.0});
        self.scalar_static_f64[1095]=p.p37;
        self.scalar_static_bool[218]=(!(self.scalar_static_f64[1095]!=0.0));
        self.scalar_static_f64[1096]=(if self.scalar_static_bool[214]{1.0}else{0.0});
        self.scalar_static_bool[219]=(2.0==self.scalar_static_f64[71]);
        self.scalar_static_f64[1097]=(if self.scalar_static_bool[219]{1.0}else{0.0});
        self.scalar_static_bool[220]=((self.scalar_static_f64[617]!=0.0)&&(self.scalar_static_f64[1096]!=0.0));
        self.scalar_static_bool[221]=(!(self.scalar_static_f64[1096]!=0.0));
        self.scalar_static_bool[222]=((self.scalar_static_f64[1097]!=0.0)&&self.scalar_static_bool[221]);
        self.scalar_static_bool[223]=((self.scalar_static_f64[617]!=0.0)&&self.scalar_static_bool[222]);
        self.scalar_static_f64[1098]=(self.scalar_static_f64[903]-1.0);
        self.scalar_static_f64[1099]=(1.0/self.scalar_static_f64[903]);
        self.scalar_static_f64[1100]=(self.scalar_static_f64[1099]-1.0);
        self.scalar_static_bool[224]=((self.scalar_static_f64[1096]!=0.0)||(self.scalar_static_f64[1097]!=0.0));
        self.scalar_static_bool[225]=(!self.scalar_static_bool[224]);
        self.scalar_static_bool[226]=((self.scalar_static_f64[77]!=0.0)&&self.scalar_static_bool[225]);
        self.scalar_static_bool[227]=((self.scalar_static_f64[617]!=0.0)&&self.scalar_static_bool[226]);
        self.scalar_static_f64[1101]=(self.scalar_static_f64[741]/self.scalar_static_f64[736]);
        self.scalar_static_f64[1102]=p.p435;
        self.scalar_static_f64[1103]=(self.scalar_static_f64[1101]*self.scalar_static_f64[1102]);
        self.scalar_static_f64[1104]=p.p24;
        self.scalar_static_bool[228]=(0.0!=self.scalar_static_f64[1104]);
        self.scalar_static_f64[1105]=(if self.scalar_static_bool[228]{1.0}else{0.0});
        self.scalar_static_f64[1106]=(self.scalar_static_f64[679]/1000000.0);
        self.scalar_static_f64[1107]=(self.scalar_static_f64[742]*self.scalar_static_f64[1106]);
        self.scalar_static_f64[1108]=p.p258;
        self.scalar_static_f64[1109]=(-self.scalar_static_f64[680]);
        self.scalar_static_f64[1110]=(self.scalar_static_f64[316]*1.6021918e-19);
        self.scalar_static_f64[1111]=(self.scalar_static_f64[742]*self.scalar_static_f64[1110]);
        self.scalar_static_f64[1112]=(self.scalar_static_f64[110]*self.scalar_static_f64[1111]);
        self.scalar_static_f64[1113]=(self.scalar_static_f64[426]*self.scalar_static_f64[742]);
        self.scalar_static_f64[1114]=(self.scalar_static_f64[736]*self.scalar_static_f64[1113]);
        self.scalar_static_f64[1115]=p.p29;
        self.scalar_static_f64[1116]=(0.0*self.scalar_static_f64[32]);
        self.scalar_static_f64[1117]=(1.0-self.scalar_static_f64[32]);
        self.scalar_static_f64[1118]=(0.0*self.scalar_static_f64[1117]);
        self.scalar_static_f64[1119]=(if self.scalar_static_bool[164]{1.0}else{0.0});
        self.scalar_static_bool[229]=(!(self.scalar_static_f64[1119]!=0.0));
        self.scalar_static_f64[1120]=(0.0*self.scalar_static_f64[1020]);
        self.scalar_static_bool[230]=(1.0==self.scalar_static_f64[1104]);
        self.scalar_static_f64[1121]=(if self.scalar_static_bool[230]{1.0}else{0.0});
        self.scalar_static_bool[231]=(self.scalar_static_f64[18]>0.0);
        self.scalar_static_bool[232]=(self.scalar_static_bool[65]&&self.scalar_static_bool[231]);
        self.scalar_static_f64[1122]=(if self.scalar_static_bool[232]{1.0}else{0.0});
        self.scalar_static_f64[1123]=(if self.scalar_static_bool[229]{0.0}else{self.scalar_static_f64[916]});
        self.scalar_static_f64[1124]=(if (self.scalar_static_f64[686]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1125]=(if (self.scalar_static_f64[1007]!=0.0){self.scalar_static_f64[1120]}else{0.0});
        self.scalar_static_f64[1126]=(-self.scalar_static_f64[1020]);
        self.scalar_static_f64[1127]=(self.scalar_static_f64[396]-1.0);
        self.scalar_static_f64[1128]=(if self.scalar_static_bool[163]{0.0}else{self.scalar_static_f64[1124]});
        self.scalar_static_f64[1129]=(self.scalar_static_f64[1126]-self.scalar_static_f64[1126]);
        self.scalar_static_f64[1130]=(if (self.scalar_static_f64[1021]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1131]=(-self.scalar_static_f64[1130]);
        self.scalar_static_f64[1132]=(if (self.scalar_static_f64[1057]!=0.0){self.scalar_static_f64[1020]}else{0.0});
        self.scalar_static_f64[1133]=(if (self.scalar_static_f64[1057]!=0.0){self.scalar_static_f64[1126]}else{0.0});
        self.scalar_static_f64[1134]=(-self.scalar_static_f64[1132]);
        self.scalar_static_f64[1135]=(-self.scalar_static_f64[1133]);
        self.scalar_static_f64[1136]=(self.scalar_static_f64[1126]-self.scalar_static_f64[1133]);
        self.scalar_static_f64[1137]=(self.scalar_static_f64[1098]-1.0);
        self.scalar_static_f64[1138]=(self.scalar_static_f64[1100]-1.0);
        self.scalar_static_f64[1139]=(if (self.scalar_static_f64[1115]!=0.0){1.0}else{0.0});
        self.scalar_static_f64[1140]=(self.scalar_static_f64[1139]/0.0);
        self.scalar_static_f64[1141]=(if (self.scalar_static_f64[1115]!=0.0){self.scalar_static_f64[1140]}else{0.0});
        self.scalar_static_f64[1142]=(-self.scalar_static_f64[814]);
        self.scalar_static_f64[1143]=(if (self.scalar_static_f64[1122]!=0.0){self.scalar_static_f64[814]}else{0.0});
        self.scalar_static_f64[1144]=(if (self.scalar_static_f64[1122]!=0.0){self.scalar_static_f64[1142]}else{0.0});
        self.scalar_static_f64[1145]=(-self.scalar_static_f64[879]);
        self.scalar_static_f64[1146]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[1145]}else{0.0});
        self.scalar_static_f64[1147]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[879]}else{0.0});
        self.scalar_static_f64[1148]=(-self.scalar_static_f64[880]);
        self.scalar_static_f64[1149]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[1148]}else{0.0});
        self.scalar_static_f64[1150]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[880]}else{0.0});
        self.scalar_static_f64[1151]=(-self.scalar_static_f64[878]);
        self.scalar_static_f64[1152]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[878]}else{0.0});
        self.scalar_static_f64[1153]=(if (self.scalar_static_f64[857]!=0.0){self.scalar_static_f64[1151]}else{0.0});
        self.scalar_static_f64[1154]=(if self.scalar_static_bool[229]{10000.0}else{0.0});
        self.scalar_static_f64[1155]=(if (self.scalar_static_f64[1115]!=0.0){self.scalar_static_f64[1141]}else{0.0});
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
        self.scalar_static_f64[1156]=(temperature+self.scalar_static_f64[925]);
        self.scalar_static_f64[1157]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1156]}else{0.0});
        self.scalar_static_f64[1158]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1157]}else{0.0});
        self.scalar_static_f64[1159]=(self.scalar_static_f64[1158]-self.scalar_static_f64[104]);
        self.scalar_static_f64[1160]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1159]}else{0.0});
        self.scalar_static_f64[1161]=(self.scalar_static_f64[1158]*self.scalar_static_f64[1158]);
        self.scalar_static_f64[1162]=(self.scalar_static_f64[1161]-self.scalar_static_f64[926]);
        self.scalar_static_f64[1163]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1162]}else{0.0});
        self.scalar_static_f64[1164]=(self.scalar_static_f64[1157]-self.scalar_static_f64[104]);
        self.scalar_static_f64[1165]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1164]}else{0.0});
        self.scalar_static_f64[1166]=(self.scalar_static_f64[1157]*self.scalar_static_f64[1157]);
        self.scalar_static_f64[1167]=(self.scalar_static_f64[1166]-self.scalar_static_f64[926]);
        self.scalar_static_f64[1168]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1167]}else{0.0});
        self.scalar_static_f64[1169]=(self.scalar_static_f64[1157]/self.scalar_static_f64[104]);
        self.scalar_static_f64[1170]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1169]}else{0.0});
        self.scalar_static_f64[1171]=(self.scalar_static_f64[1170]).ln();
        self.scalar_static_f64[1172]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1171]}else{0.0});
        self.scalar_static_f64[1173]=(self.scalar_static_f64[136]*self.scalar_static_f64[1165]);
        self.scalar_static_f64[1174]=(self.scalar_static_f64[693]-self.scalar_static_f64[1173]);
        self.scalar_static_f64[1175]=(self.scalar_static_f64[146]*self.scalar_static_f64[1168]);
        self.scalar_static_f64[1176]=(self.scalar_static_f64[1174]-self.scalar_static_f64[1175]);
        self.scalar_static_f64[1177]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1176]}else{0.0});
        self.scalar_static_f64[1178]=(self.scalar_static_f64[1177]).sqrt();
        self.scalar_static_f64[1179]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1178]}else{0.0});
        self.scalar_static_f64[1180]=(1.0/self.scalar_static_f64[1157]);
        self.scalar_static_f64[1181]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1180]}else{self.scalar_static_f64[895]});
        self.scalar_static_f64[1182]=(self.scalar_static_f64[1181]-self.scalar_static_f64[928]);
        self.scalar_static_f64[1183]=(self.scalar_static_f64[931]*self.scalar_static_f64[1182]);
        self.scalar_static_f64[1184]=(self.scalar_static_f64[930]+self.scalar_static_f64[1183]);
        self.scalar_static_f64[1185]=(self.scalar_static_f64[1181]*self.scalar_static_f64[1181]);
        self.scalar_static_f64[1186]=(self.scalar_static_f64[1185]-self.scalar_static_f64[933]);
        self.scalar_static_f64[1187]=(self.scalar_static_f64[932]*self.scalar_static_f64[1186]);
        self.scalar_static_f64[1188]=(self.scalar_static_f64[1184]+self.scalar_static_f64[1187]);
        self.scalar_static_f64[1189]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1188]}else{self.scalar_static_f64[854]});
        self.scalar_static_f64[1190]=(self.scalar_static_f64[1157]*1.3806226e-23);
        self.scalar_static_f64[1191]=(1.6021918e-19/self.scalar_static_f64[1190]);
        self.scalar_static_f64[1192]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1191]}else{0.0});
        self.scalar_static_f64[1193]=(1.0/self.scalar_static_f64[1192]);
        self.scalar_static_f64[1194]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1193]}else{0.0});
        self.scalar_static_f64[1195]=(1.5*self.scalar_static_f64[1172]);
        self.scalar_static_f64[1196]=(self.scalar_static_f64[1195]).exp();
        self.scalar_static_f64[1197]=(1.04e16*self.scalar_static_f64[1196]);
        self.scalar_static_f64[1198]=(-self.scalar_static_f64[1177]);
        self.scalar_static_f64[1199]=(self.scalar_static_f64[1198]/2.0);
        self.scalar_static_f64[1200]=(self.scalar_static_f64[1192]*self.scalar_static_f64[1199]);
        self.scalar_static_f64[1201]=(self.scalar_static_f64[1200]+self.scalar_static_f64[938]);
        self.scalar_static_f64[1202]=(self.scalar_static_f64[1201]).exp();
        self.scalar_static_f64[1203]=(self.scalar_static_f64[1197]*self.scalar_static_f64[1202]);
        self.scalar_static_f64[1204]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1203]}else{0.0});
        self.scalar_static_f64[1205]=(self.scalar_static_f64[306]*self.scalar_static_f64[1172]);
        self.scalar_static_f64[1206]=(self.scalar_static_f64[1205]).exp();
        self.scalar_static_f64[1207]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1206]}else{self.scalar_static_f64[1181]});
        self.scalar_static_f64[1208]=(2.0*self.scalar_static_f64[1194]);
        self.scalar_static_f64[1209]=(self.scalar_static_f64[652]/self.scalar_static_f64[1204]);
        self.scalar_static_f64[1210]=(self.scalar_static_f64[1209]).ln();
        self.scalar_static_f64[1211]=(self.scalar_static_f64[1208]*self.scalar_static_f64[1210]);
        self.scalar_static_f64[1212]=(if self.scalar_static_bool[114]{self.scalar_static_f64[1211]}else{0.0});
        self.scalar_static_f64[1213]=(self.scalar_static_f64[940]/self.scalar_static_f64[1204]);
        self.scalar_static_f64[1214]=(self.scalar_static_f64[1213]/self.scalar_static_f64[1204]);
        self.scalar_static_f64[1215]=(self.scalar_static_f64[1214]).ln();
        self.scalar_static_f64[1216]=(self.scalar_static_f64[1194]*self.scalar_static_f64[1215]);
        self.scalar_static_f64[1217]=(if self.scalar_static_bool[114]{self.scalar_static_f64[1216]}else{0.0});
        self.scalar_static_f64[1218]=(self.scalar_static_f64[1172]*self.scalar_static_f64[941]);
        self.scalar_static_f64[1219]=(self.scalar_static_f64[1218]).exp();
        self.scalar_static_f64[1220]=(if self.scalar_static_bool[114]{self.scalar_static_f64[1219]}else{self.scalar_static_f64[1207]});
        self.scalar_static_f64[1221]=(if self.scalar_static_bool[117]{self.scalar_static_f64[1211]}else{self.scalar_static_f64[1212]});
        self.scalar_static_f64[1222]=(if self.scalar_static_bool[117]{self.scalar_static_f64[1216]}else{self.scalar_static_f64[1217]});
        self.scalar_static_f64[1223]=(if self.scalar_static_bool[117]{self.scalar_static_f64[1219]}else{self.scalar_static_f64[1220]});
        self.scalar_static_f64[1224]=(if self.scalar_static_bool[119]{0.0}else{self.scalar_static_f64[1221]});
        self.scalar_static_f64[1225]=(self.scalar_static_f64[103]/self.scalar_static_f64[1204]);
        self.scalar_static_f64[1226]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1169]}else{self.scalar_static_f64[1223]});
        self.scalar_static_f64[1227]=(self.scalar_static_f64[1160]*self.scalar_static_f64[944]);
        self.scalar_static_f64[1228]=(1.0+self.scalar_static_f64[1227]);
        self.scalar_static_f64[1229]=(self.scalar_static_f64[1163]*self.scalar_static_f64[945]);
        self.scalar_static_f64[1230]=(self.scalar_static_f64[1228]+self.scalar_static_f64[1229]);
        self.scalar_static_f64[1231]=(if self.scalar_static_bool[121]{self.scalar_static_f64[1230]}else{self.scalar_static_f64[1226]});
        self.scalar_static_f64[1232]=(self.scalar_static_f64[1165]*self.scalar_static_f64[944]);
        self.scalar_static_f64[1233]=(1.0+self.scalar_static_f64[1232]);
        self.scalar_static_f64[1234]=(self.scalar_static_f64[1168]*self.scalar_static_f64[945]);
        self.scalar_static_f64[1235]=(self.scalar_static_f64[1233]+self.scalar_static_f64[1234]);
        self.scalar_static_f64[1236]=(if self.scalar_static_bool[123]{self.scalar_static_f64[1235]}else{self.scalar_static_f64[1231]});
        self.scalar_static_f64[1237]=(self.scalar_static_f64[42]*self.scalar_static_f64[1160]);
        self.scalar_static_f64[1238]=(self.scalar_static_f64[685]+self.scalar_static_f64[1237]);
        self.scalar_static_f64[1239]=(self.scalar_static_f64[43]*self.scalar_static_f64[1163]);
        self.scalar_static_f64[1240]=(self.scalar_static_f64[1238]+self.scalar_static_f64[1239]);
        self.scalar_static_f64[1241]=(self.scalar_static_f64[922]*self.scalar_static_f64[1240]);
        self.scalar_static_f64[1242]=(if self.scalar_static_bool[124]{self.scalar_static_f64[1241]}else{self.scalar_static_f64[919]});
        self.scalar_static_bool[233]=(self.scalar_static_f64[1242]<0.0001);
        self.scalar_static_f64[1243]=(if self.scalar_static_bool[233]{1.0}else{0.0});
        self.scalar_static_bool[234]=(self.scalar_static_bool[124]&&(self.scalar_static_f64[1243]!=0.0));
        self.scalar_static_f64[1244]=(if self.scalar_static_bool[234]{0.0001}else{self.scalar_static_f64[1242]});
        self.scalar_static_f64[1245]=(self.scalar_static_f64[49]*self.scalar_static_f64[1160]);
        self.scalar_static_f64[1246]=(self.scalar_static_f64[456]+self.scalar_static_f64[1245]);
        self.scalar_static_f64[1247]=(self.scalar_static_f64[50]*self.scalar_static_f64[1163]);
        self.scalar_static_f64[1248]=(self.scalar_static_f64[1246]+self.scalar_static_f64[1247]);
        self.scalar_static_f64[1249]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1248]}else{self.scalar_static_f64[928]});
        self.scalar_static_f64[1250]=(self.scalar_static_f64[1249]-0.05);
        self.scalar_static_f64[1251]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1250]}else{self.scalar_static_f64[774]});
        self.scalar_static_f64[1252]=(self.scalar_static_f64[1251]*self.scalar_static_f64[1251]);
        self.scalar_static_f64[1253]=(self.scalar_static_f64[949]+self.scalar_static_f64[1252]);
        self.scalar_static_f64[1254]=(self.scalar_static_f64[1253]).sqrt();
        self.scalar_static_f64[1255]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1254]}else{self.scalar_static_f64[949]});
        self.scalar_static_f64[1256]=(self.scalar_static_f64[1251]+self.scalar_static_f64[1255]);
        self.scalar_static_f64[1257]=(0.5*self.scalar_static_f64[1256]);
        self.scalar_static_f64[1258]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1257]}else{self.scalar_static_f64[1249]});
        self.scalar_static_f64[1259]=(1.0-self.scalar_static_f64[1258]);
        self.scalar_static_f64[1260]=(self.scalar_static_f64[1259]-0.05);
        self.scalar_static_f64[1261]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1260]}else{self.scalar_static_f64[1251]});
        self.scalar_static_f64[1262]=(if (self.scalar_static_f64[924]!=0.0){0.2}else{self.scalar_static_f64[1255]});
        self.scalar_static_bool[235]=(self.scalar_static_f64[1262]>0.0);
        self.scalar_static_f64[1263]=(-self.scalar_static_f64[1262]);
        self.scalar_static_f64[1264]=(if self.scalar_static_bool[235]{self.scalar_static_f64[1262]}else{self.scalar_static_f64[1263]});
        self.scalar_static_f64[1265]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1264]}else{self.scalar_static_f64[1262]});
        self.scalar_static_f64[1266]=(self.scalar_static_f64[1261]*self.scalar_static_f64[1261]);
        self.scalar_static_f64[1267]=(self.scalar_static_f64[1265]+self.scalar_static_f64[1266]);
        self.scalar_static_f64[1268]=(self.scalar_static_f64[1267]).sqrt();
        self.scalar_static_f64[1269]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1268]}else{self.scalar_static_f64[1265]});
        self.scalar_static_f64[1270]=(if (self.scalar_static_f64[924]!=0.0){1291908996.1638799}else{self.scalar_static_f64[1236]});
        self.scalar_static_f64[1271]=(self.scalar_static_f64[1270]/self.scalar_static_f64[721]);
        self.scalar_static_f64[1272]=(self.scalar_static_f64[1271]).sqrt();
        self.scalar_static_f64[1273]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1272]}else{0.0});
        self.scalar_static_f64[1274]=(if self.scalar_static_bool[128]{self.scalar_static_f64[952]}else{self.scalar_static_f64[1258]});
        self.scalar_static_f64[1275]=(self.scalar_static_f64[91]*self.scalar_static_f64[1160]);
        self.scalar_static_f64[1276]=(self.scalar_static_f64[466]+self.scalar_static_f64[1275]);
        self.scalar_static_f64[1277]=(self.scalar_static_f64[92]*self.scalar_static_f64[1163]);
        self.scalar_static_f64[1278]=(self.scalar_static_f64[1276]+self.scalar_static_f64[1277]);
        self.scalar_static_f64[1279]=(self.scalar_static_f64[1274]*self.scalar_static_f64[1278]);
        self.scalar_static_f64[1280]=(if self.scalar_static_bool[130]{self.scalar_static_f64[1279]}else{0.0});
        self.scalar_static_f64[1281]=(self.scalar_static_f64[1280]-self.scalar_static_f64[954]);
        self.scalar_static_f64[1282]=(self.scalar_static_f64[1281]-self.scalar_static_f64[955]);
        self.scalar_static_f64[1283]=(if self.scalar_static_bool[130]{self.scalar_static_f64[1282]}else{self.scalar_static_f64[1261]});
        self.scalar_static_f64[1284]=(if self.scalar_static_bool[130]{self.scalar_static_f64[957]}else{self.scalar_static_f64[1269]});
        self.scalar_static_bool[236]=(self.scalar_static_f64[1284]>0.0);
        self.scalar_static_f64[1285]=(-self.scalar_static_f64[1284]);
        self.scalar_static_f64[1286]=(if self.scalar_static_bool[236]{self.scalar_static_f64[1284]}else{self.scalar_static_f64[1285]});
        self.scalar_static_f64[1287]=(if self.scalar_static_bool[130]{self.scalar_static_f64[1286]}else{self.scalar_static_f64[1284]});
        self.scalar_static_f64[1288]=(self.scalar_static_f64[1283]*self.scalar_static_f64[1283]);
        self.scalar_static_f64[1289]=(self.scalar_static_f64[1287]+self.scalar_static_f64[1288]);
        self.scalar_static_f64[1290]=(self.scalar_static_f64[1289]).sqrt();
        self.scalar_static_f64[1291]=(if self.scalar_static_bool[130]{self.scalar_static_f64[1290]}else{self.scalar_static_f64[1287]});
        self.scalar_static_f64[1292]=(self.scalar_static_f64[1283]+self.scalar_static_f64[1291]);
        self.scalar_static_f64[1293]=(0.5*self.scalar_static_f64[1292]);
        self.scalar_static_f64[1294]=(self.scalar_static_f64[954]+self.scalar_static_f64[1293]);
        self.scalar_static_f64[1295]=(if self.scalar_static_bool[130]{self.scalar_static_f64[1294]}else{self.scalar_static_f64[1280]});
        self.scalar_static_f64[1296]=(self.scalar_static_f64[91]*self.scalar_static_f64[1165]);
        self.scalar_static_f64[1297]=(self.scalar_static_f64[466]+self.scalar_static_f64[1296]);
        self.scalar_static_f64[1298]=(self.scalar_static_f64[92]*self.scalar_static_f64[1168]);
        self.scalar_static_f64[1299]=(self.scalar_static_f64[1297]+self.scalar_static_f64[1298]);
        self.scalar_static_f64[1300]=(self.scalar_static_f64[1274]*self.scalar_static_f64[1299]);
        self.scalar_static_f64[1301]=(if self.scalar_static_bool[132]{self.scalar_static_f64[1300]}else{self.scalar_static_f64[1295]});
        self.scalar_static_f64[1302]=(self.scalar_static_f64[1301]-self.scalar_static_f64[954]);
        self.scalar_static_f64[1303]=(self.scalar_static_f64[1302]-self.scalar_static_f64[955]);
        self.scalar_static_f64[1304]=(if self.scalar_static_bool[132]{self.scalar_static_f64[1303]}else{self.scalar_static_f64[1283]});
        self.scalar_static_f64[1305]=(if self.scalar_static_bool[132]{self.scalar_static_f64[957]}else{self.scalar_static_f64[1291]});
        self.scalar_static_bool[237]=(self.scalar_static_f64[1305]>0.0);
        self.scalar_static_f64[1306]=(-self.scalar_static_f64[1305]);
        self.scalar_static_f64[1307]=(if self.scalar_static_bool[237]{self.scalar_static_f64[1305]}else{self.scalar_static_f64[1306]});
        self.scalar_static_f64[1308]=(if self.scalar_static_bool[132]{self.scalar_static_f64[1307]}else{self.scalar_static_f64[1305]});
        self.scalar_static_f64[1309]=(self.scalar_static_f64[1304]*self.scalar_static_f64[1304]);
        self.scalar_static_f64[1310]=(self.scalar_static_f64[1308]+self.scalar_static_f64[1309]);
        self.scalar_static_f64[1311]=(self.scalar_static_f64[1310]).sqrt();
        self.scalar_static_f64[1312]=(if self.scalar_static_bool[132]{self.scalar_static_f64[1311]}else{self.scalar_static_f64[1308]});
        self.scalar_static_f64[1313]=(self.scalar_static_f64[1304]+self.scalar_static_f64[1312]);
        self.scalar_static_f64[1314]=(0.5*self.scalar_static_f64[1313]);
        self.scalar_static_f64[1315]=(self.scalar_static_f64[954]+self.scalar_static_f64[1314]);
        self.scalar_static_f64[1316]=(if self.scalar_static_bool[132]{self.scalar_static_f64[1315]}else{self.scalar_static_f64[1301]});
        self.scalar_static_f64[1317]=(if self.scalar_static_bool[134]{0.0}else{self.scalar_static_f64[1316]});
        self.scalar_static_f64[1318]=(if self.scalar_static_bool[135]{self.scalar_static_f64[960]}else{self.scalar_static_f64[1274]});
        self.scalar_static_f64[1319]=(self.scalar_static_f64[546]+self.scalar_static_f64[1275]);
        self.scalar_static_f64[1320]=(self.scalar_static_f64[1277]+self.scalar_static_f64[1319]);
        self.scalar_static_f64[1321]=(self.scalar_static_f64[1318]*self.scalar_static_f64[1320]);
        self.scalar_static_f64[1322]=(if self.scalar_static_bool[136]{self.scalar_static_f64[1321]}else{0.0});
        self.scalar_static_f64[1323]=(self.scalar_static_f64[1322]-self.scalar_static_f64[961]);
        self.scalar_static_f64[1324]=(self.scalar_static_f64[1323]-self.scalar_static_f64[962]);
        self.scalar_static_f64[1325]=(if self.scalar_static_bool[136]{self.scalar_static_f64[1324]}else{self.scalar_static_f64[1304]});
        self.scalar_static_f64[1326]=(if self.scalar_static_bool[136]{self.scalar_static_f64[964]}else{self.scalar_static_f64[1312]});
        self.scalar_static_bool[238]=(self.scalar_static_f64[1326]>0.0);
        self.scalar_static_f64[1327]=(-self.scalar_static_f64[1326]);
        self.scalar_static_f64[1328]=(if self.scalar_static_bool[238]{self.scalar_static_f64[1326]}else{self.scalar_static_f64[1327]});
        self.scalar_static_f64[1329]=(if self.scalar_static_bool[136]{self.scalar_static_f64[1328]}else{self.scalar_static_f64[1326]});
        self.scalar_static_f64[1330]=(self.scalar_static_f64[1325]*self.scalar_static_f64[1325]);
        self.scalar_static_f64[1331]=(self.scalar_static_f64[1329]+self.scalar_static_f64[1330]);
        self.scalar_static_f64[1332]=(self.scalar_static_f64[1331]).sqrt();
        self.scalar_static_f64[1333]=(if self.scalar_static_bool[136]{self.scalar_static_f64[1332]}else{self.scalar_static_f64[1329]});
        self.scalar_static_f64[1334]=(self.scalar_static_f64[1325]+self.scalar_static_f64[1333]);
        self.scalar_static_f64[1335]=(0.5*self.scalar_static_f64[1334]);
        self.scalar_static_f64[1336]=(self.scalar_static_f64[961]+self.scalar_static_f64[1335]);
        self.scalar_static_f64[1337]=(if self.scalar_static_bool[136]{self.scalar_static_f64[1336]}else{self.scalar_static_f64[1322]});
        self.scalar_static_f64[1338]=(self.scalar_static_f64[546]+self.scalar_static_f64[1296]);
        self.scalar_static_f64[1339]=(self.scalar_static_f64[1298]+self.scalar_static_f64[1338]);
        self.scalar_static_f64[1340]=(self.scalar_static_f64[1318]*self.scalar_static_f64[1339]);
        self.scalar_static_f64[1341]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1340]}else{self.scalar_static_f64[1337]});
        self.scalar_static_f64[1342]=(self.scalar_static_f64[1341]-self.scalar_static_f64[961]);
        self.scalar_static_f64[1343]=(self.scalar_static_f64[1342]-self.scalar_static_f64[962]);
        self.scalar_static_f64[1344]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1343]}else{self.scalar_static_f64[1325]});
        self.scalar_static_f64[1345]=(if self.scalar_static_bool[137]{self.scalar_static_f64[964]}else{self.scalar_static_f64[1333]});
        self.scalar_static_bool[239]=(self.scalar_static_f64[1345]>0.0);
        self.scalar_static_f64[1346]=(-self.scalar_static_f64[1345]);
        self.scalar_static_f64[1347]=(if self.scalar_static_bool[239]{self.scalar_static_f64[1345]}else{self.scalar_static_f64[1346]});
        self.scalar_static_f64[1348]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1347]}else{self.scalar_static_f64[1345]});
        self.scalar_static_f64[1349]=(self.scalar_static_f64[1344]*self.scalar_static_f64[1344]);
        self.scalar_static_f64[1350]=(self.scalar_static_f64[1348]+self.scalar_static_f64[1349]);
        self.scalar_static_f64[1351]=(self.scalar_static_f64[1350]).sqrt();
        self.scalar_static_f64[1352]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1351]}else{self.scalar_static_f64[1348]});
        self.scalar_static_f64[1353]=(self.scalar_static_f64[1344]+self.scalar_static_f64[1352]);
        self.scalar_static_f64[1354]=(0.5*self.scalar_static_f64[1353]);
        self.scalar_static_f64[1355]=(self.scalar_static_f64[961]+self.scalar_static_f64[1354]);
        self.scalar_static_f64[1356]=(if self.scalar_static_bool[137]{self.scalar_static_f64[1355]}else{self.scalar_static_f64[1341]});
        self.scalar_static_f64[1357]=(if self.scalar_static_bool[139]{0.0}else{self.scalar_static_f64[1356]});
        self.scalar_static_f64[1358]=(if self.scalar_static_bool[141]{self.scalar_static_f64[975]}else{self.scalar_static_f64[1352]});
        self.scalar_static_f64[1359]=(self.scalar_static_f64[66]+self.scalar_static_f64[1358]);
        self.scalar_static_f64[1360]=(0.5*self.scalar_static_f64[1359]);
        self.scalar_static_f64[1361]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1360]}else{self.scalar_static_f64[1318]});
        self.scalar_static_bool[240]=(self.scalar_static_f64[1361]<0.0);
        self.scalar_static_f64[1362]=(if self.scalar_static_bool[240]{1.0}else{0.0});
        self.scalar_static_bool[241]=(self.scalar_static_bool[141]&&(self.scalar_static_f64[1362]!=0.0));
        self.scalar_static_f64[1363]=(if self.scalar_static_bool[241]{0.0}else{self.scalar_static_f64[1361]});
        self.scalar_static_f64[1364]=(self.scalar_static_f64[976]/self.scalar_static_f64[1363]);
        self.scalar_static_f64[1365]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1364]}else{0.0});
        self.scalar_static_f64[1366]=(self.scalar_static_f64[970]*self.scalar_static_f64[1365]);
        self.scalar_static_f64[1367]=(1000000.0*self.scalar_static_f64[1366]);
        self.scalar_static_f64[1368]=(1.0+self.scalar_static_f64[1367]);
        self.scalar_static_f64[1369]=(self.scalar_static_f64[65]+self.scalar_static_f64[1368]);
        self.scalar_static_f64[1370]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1369]}else{self.scalar_static_f64[1189]});
        self.scalar_static_f64[1371]=(self.scalar_static_f64[968]*self.scalar_static_f64[1370]);
        self.scalar_static_f64[1372]=(self.scalar_static_f64[1371]-self.scalar_static_f64[968]);
        self.scalar_static_f64[1373]=(self.scalar_static_f64[1372]-0.01);
        self.scalar_static_f64[1374]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1373]}else{self.scalar_static_f64[1344]});
        self.scalar_static_f64[1375]=(if self.scalar_static_bool[141]{self.scalar_static_f64[978]}else{self.scalar_static_f64[1358]});
        self.scalar_static_bool[242]=(self.scalar_static_f64[1375]>0.0);
        self.scalar_static_f64[1376]=(-self.scalar_static_f64[1375]);
        self.scalar_static_f64[1377]=(if self.scalar_static_bool[242]{self.scalar_static_f64[1375]}else{self.scalar_static_f64[1376]});
        self.scalar_static_f64[1378]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1377]}else{self.scalar_static_f64[1375]});
        self.scalar_static_f64[1379]=(self.scalar_static_f64[1374]*self.scalar_static_f64[1374]);
        self.scalar_static_f64[1380]=(self.scalar_static_f64[1378]+self.scalar_static_f64[1379]);
        self.scalar_static_f64[1381]=(self.scalar_static_f64[1380]).sqrt();
        self.scalar_static_f64[1382]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1381]}else{self.scalar_static_f64[1378]});
        self.scalar_static_f64[1383]=(self.scalar_static_f64[1374]/self.scalar_static_f64[1382]);
        self.scalar_static_f64[1384]=(1.0+self.scalar_static_f64[1383]);
        self.scalar_static_f64[1385]=(0.5*self.scalar_static_f64[1384]);
        self.scalar_static_f64[1386]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1385]}else{self.scalar_static_f64[833]});
        self.scalar_static_f64[1387]=(self.scalar_static_f64[1374]+self.scalar_static_f64[1382]);
        self.scalar_static_f64[1388]=(0.5*self.scalar_static_f64[1387]);
        self.scalar_static_f64[1389]=(self.scalar_static_f64[968]+self.scalar_static_f64[1388]);
        self.scalar_static_f64[1390]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1389]}else{0.0});
        self.scalar_static_f64[1391]=(self.scalar_static_f64[980]-self.scalar_static_f64[1390]);
        self.scalar_static_f64[1392]=(self.scalar_static_f64[1391]-5e-5);
        self.scalar_static_f64[1393]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1392]}else{self.scalar_static_f64[1374]});
        self.scalar_static_f64[1394]=(if self.scalar_static_bool[141]{self.scalar_static_f64[982]}else{self.scalar_static_f64[1382]});
        self.scalar_static_bool[243]=(self.scalar_static_f64[1394]>0.0);
        self.scalar_static_f64[1395]=(-self.scalar_static_f64[1394]);
        self.scalar_static_f64[1396]=(if self.scalar_static_bool[243]{self.scalar_static_f64[1394]}else{self.scalar_static_f64[1395]});
        self.scalar_static_f64[1397]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1396]}else{self.scalar_static_f64[1394]});
        self.scalar_static_f64[1398]=(self.scalar_static_f64[1393]*self.scalar_static_f64[1393]);
        self.scalar_static_f64[1399]=(self.scalar_static_f64[1397]+self.scalar_static_f64[1398]);
        self.scalar_static_f64[1400]=(self.scalar_static_f64[1399]).sqrt();
        self.scalar_static_f64[1401]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1400]}else{self.scalar_static_f64[1397]});
        self.scalar_static_f64[1402]=(self.scalar_static_f64[1393]/self.scalar_static_f64[1401]);
        self.scalar_static_f64[1403]=(1.0+self.scalar_static_f64[1402]);
        self.scalar_static_f64[1404]=(0.5*self.scalar_static_f64[1403]);
        self.scalar_static_f64[1405]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1404]}else{self.scalar_static_f64[1386]});
        self.scalar_static_f64[1406]=(self.scalar_static_f64[1393]+self.scalar_static_f64[1401]);
        self.scalar_static_f64[1407]=(0.5*self.scalar_static_f64[1406]);
        self.scalar_static_f64[1408]=(self.scalar_static_f64[980]-self.scalar_static_f64[1407]);
        self.scalar_static_f64[1409]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1408]}else{self.scalar_static_f64[825]});
        self.scalar_static_f64[1410]=(if self.scalar_static_bool[141]{0.0}else{self.scalar_static_f64[1401]});
        self.scalar_static_bool[244]=(self.scalar_static_f64[1410]>0.0);
        self.scalar_static_f64[1411]=(-self.scalar_static_f64[1410]);
        self.scalar_static_f64[1412]=(if self.scalar_static_bool[244]{self.scalar_static_f64[1410]}else{self.scalar_static_f64[1411]});
        self.scalar_static_f64[1413]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1412]}else{self.scalar_static_f64[1410]});
        self.scalar_static_f64[1414]=(self.scalar_static_f64[93]*self.scalar_static_f64[1160]);
        self.scalar_static_f64[1415]=(self.scalar_static_f64[684]+self.scalar_static_f64[1414]);
        self.scalar_static_f64[1416]=(self.scalar_static_f64[94]*self.scalar_static_f64[1163]);
        self.scalar_static_f64[1417]=(self.scalar_static_f64[1415]+self.scalar_static_f64[1416]);
        self.scalar_static_f64[1418]=(self.scalar_static_f64[93]*self.scalar_static_f64[1165]);
        self.scalar_static_f64[1419]=(self.scalar_static_f64[684]+self.scalar_static_f64[1418]);
        self.scalar_static_f64[1420]=(self.scalar_static_f64[94]*self.scalar_static_f64[1168]);
        self.scalar_static_f64[1421]=(self.scalar_static_f64[1419]+self.scalar_static_f64[1420]);
        self.scalar_static_f64[1422]=(self.scalar_static_f64[1365]*self.scalar_static_f64[991]);
        self.scalar_static_f64[1423]=(1000000.0*self.scalar_static_f64[1422]);
        self.scalar_static_f64[1424]=(1.0+self.scalar_static_f64[1423]);
        self.scalar_static_f64[1425]=(self.scalar_static_f64[65]+self.scalar_static_f64[1424]);
        self.scalar_static_f64[1426]=(if self.scalar_static_bool[141]{self.scalar_static_f64[1425]}else{self.scalar_static_f64[1370]});
        self.scalar_static_f64[1427]=(self.scalar_static_f64[990]*self.scalar_static_f64[1426]);
        self.scalar_static_f64[1428]=(self.scalar_static_f64[1427]-self.scalar_static_f64[990]);
        self.scalar_static_f64[1429]=(self.scalar_static_f64[1428]-0.01);
        self.scalar_static_bool[245]=(0.0==self.scalar_static_f64[1170]);
        self.scalar_static_f64[1430]=f64::powf(self.scalar_static_f64[1170],self.scalar_static_f64[1000]);
        self.scalar_static_f64[1431]=(if self.scalar_static_bool[245]{0.0}else{self.scalar_static_f64[1430]});
        self.scalar_static_f64[1432]=f64::powf(self.scalar_static_f64[1170],self.scalar_static_f64[69]);
        self.scalar_static_f64[1433]=(if self.scalar_static_bool[245]{0.0}else{self.scalar_static_f64[1432]});
        self.scalar_static_f64[1434]=(self.scalar_static_f64[1177]*self.scalar_static_f64[1192]);
        self.scalar_static_f64[1435]=(self.scalar_static_f64[1001]-self.scalar_static_f64[1434]);
        self.scalar_static_f64[1436]=(self.scalar_static_f64[1172]*self.scalar_static_f64[1002]);
        self.scalar_static_f64[1437]=(self.scalar_static_f64[1435]+self.scalar_static_f64[1436]);
        self.scalar_static_f64[1438]=(self.scalar_static_f64[1437]/self.scalar_static_f64[586]);
        self.scalar_static_f64[1439]=(self.scalar_static_f64[1438]).exp();
        self.scalar_static_f64[1440]=(self.scalar_static_f64[566]*self.scalar_static_f64[1439]);
        self.scalar_static_f64[1441]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1440]}else{0.0});
        self.scalar_static_f64[1442]=(self.scalar_static_f64[1437]/self.scalar_static_f64[1003]);
        self.scalar_static_f64[1443]=(self.scalar_static_f64[1442]).exp();
        self.scalar_static_f64[1444]=(self.scalar_static_f64[576]*self.scalar_static_f64[1443]);
        self.scalar_static_f64[1445]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1444]}else{0.0});
        self.scalar_static_f64[1446]=(self.scalar_static_f64[1437]/self.scalar_static_f64[1005]);
        self.scalar_static_f64[1447]=(self.scalar_static_f64[1446]).exp();
        self.scalar_static_f64[1448]=(self.scalar_static_f64[1004]*self.scalar_static_f64[1447]);
        self.scalar_static_f64[1449]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1448]}else{0.0});
        self.scalar_static_f64[1450]=(self.scalar_static_f64[1441]*self.scalar_static_f64[1010]);
        self.scalar_static_f64[1451]=(if self.scalar_static_bool[154]{self.scalar_static_f64[1450]}else{0.0});
        self.scalar_static_f64[1452]=(self.scalar_static_f64[1445]*self.scalar_static_f64[1011]);
        self.scalar_static_f64[1453]=(if self.scalar_static_bool[154]{self.scalar_static_f64[1452]}else{0.0});
        self.scalar_static_f64[1454]=(self.scalar_static_f64[742]*self.scalar_static_f64[1449]);
        self.scalar_static_f64[1455]=(if self.scalar_static_bool[154]{self.scalar_static_f64[1454]}else{0.0});
        self.scalar_static_f64[1456]=(if self.scalar_static_bool[156]{self.scalar_static_f64[1450]}else{self.scalar_static_f64[1451]});
        self.scalar_static_f64[1457]=(if self.scalar_static_bool[156]{0.0}else{self.scalar_static_f64[1453]});
        self.scalar_static_f64[1458]=(self.scalar_static_f64[1449]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1459]=(if self.scalar_static_bool[156]{self.scalar_static_f64[1458]}else{self.scalar_static_f64[1455]});
        self.scalar_static_f64[1460]=(if self.scalar_static_bool[158]{self.scalar_static_f64[1450]}else{self.scalar_static_f64[1456]});
        self.scalar_static_f64[1461]=(self.scalar_static_f64[1445]*self.scalar_static_f64[1008]);
        self.scalar_static_f64[1462]=(if self.scalar_static_bool[158]{self.scalar_static_f64[1461]}else{self.scalar_static_f64[1457]});
        self.scalar_static_f64[1463]=(if self.scalar_static_bool[158]{0.0}else{self.scalar_static_f64[1459]});
        self.scalar_static_f64[1464]=(self.scalar_static_f64[1460]+self.scalar_static_f64[1462]);
        self.scalar_static_f64[1465]=(self.scalar_static_f64[1463]+self.scalar_static_f64[1464]);
        self.scalar_static_f64[1466]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1465]}else{0.0});
        self.scalar_static_bool[246]=(self.scalar_static_f64[1466]>0.0);
        self.scalar_static_f64[1467]=(if self.scalar_static_bool[246]{1.0}else{0.0});
        self.scalar_static_bool[247]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[1467]!=0.0));
        self.scalar_static_f64[1468]=(1e-25+self.scalar_static_f64[1466]);
        self.scalar_static_f64[1469]=(self.scalar_static_f64[1172]*self.scalar_static_f64[1012]);
        self.scalar_static_f64[1470]=(self.scalar_static_f64[1435]+self.scalar_static_f64[1469]);
        self.scalar_static_f64[1471]=(self.scalar_static_f64[1470]/self.scalar_static_f64[616]);
        self.scalar_static_f64[1472]=(self.scalar_static_f64[1471]).exp();
        self.scalar_static_f64[1473]=(self.scalar_static_f64[596]*self.scalar_static_f64[1472]);
        self.scalar_static_f64[1474]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1473]}else{self.scalar_static_f64[1441]});
        self.scalar_static_f64[1475]=(self.scalar_static_f64[1470]/self.scalar_static_f64[1013]);
        self.scalar_static_f64[1476]=(self.scalar_static_f64[1475]).exp();
        self.scalar_static_f64[1477]=(self.scalar_static_f64[606]*self.scalar_static_f64[1476]);
        self.scalar_static_f64[1478]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1477]}else{self.scalar_static_f64[1445]});
        self.scalar_static_f64[1479]=(self.scalar_static_f64[1470]/self.scalar_static_f64[1015]);
        self.scalar_static_f64[1480]=(self.scalar_static_f64[1479]).exp();
        self.scalar_static_f64[1481]=(self.scalar_static_f64[1014]*self.scalar_static_f64[1480]);
        self.scalar_static_f64[1482]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1481]}else{self.scalar_static_f64[1449]});
        self.scalar_static_f64[1483]=(self.scalar_static_f64[1474]*self.scalar_static_f64[1018]);
        self.scalar_static_f64[1484]=(if self.scalar_static_bool[160]{self.scalar_static_f64[1483]}else{0.0});
        self.scalar_static_f64[1485]=(self.scalar_static_f64[1478]*self.scalar_static_f64[1019]);
        self.scalar_static_f64[1486]=(if self.scalar_static_bool[160]{self.scalar_static_f64[1485]}else{0.0});
        self.scalar_static_f64[1487]=(self.scalar_static_f64[742]*self.scalar_static_f64[1482]);
        self.scalar_static_f64[1488]=(if self.scalar_static_bool[160]{self.scalar_static_f64[1487]}else{0.0});
        self.scalar_static_f64[1489]=(if self.scalar_static_bool[162]{self.scalar_static_f64[1483]}else{self.scalar_static_f64[1484]});
        self.scalar_static_f64[1490]=(if self.scalar_static_bool[162]{0.0}else{self.scalar_static_f64[1486]});
        self.scalar_static_f64[1491]=(self.scalar_static_f64[1482]*self.scalar_static_f64[1016]);
        self.scalar_static_f64[1492]=(if self.scalar_static_bool[162]{self.scalar_static_f64[1491]}else{self.scalar_static_f64[1488]});
        self.scalar_static_f64[1493]=(if self.scalar_static_bool[158]{self.scalar_static_f64[1483]}else{self.scalar_static_f64[1489]});
        self.scalar_static_f64[1494]=(self.scalar_static_f64[1478]*self.scalar_static_f64[1016]);
        self.scalar_static_f64[1495]=(if self.scalar_static_bool[158]{self.scalar_static_f64[1494]}else{self.scalar_static_f64[1490]});
        self.scalar_static_f64[1496]=(if self.scalar_static_bool[158]{0.0}else{self.scalar_static_f64[1492]});
        self.scalar_static_f64[1497]=(self.scalar_static_f64[1493]+self.scalar_static_f64[1495]);
        self.scalar_static_f64[1498]=(self.scalar_static_f64[1496]+self.scalar_static_f64[1497]);
        self.scalar_static_f64[1499]=(if (self.scalar_static_f64[924]!=0.0){self.scalar_static_f64[1498]}else{0.0});
        self.scalar_static_bool[248]=(self.scalar_static_f64[1499]>0.0);
        self.scalar_static_f64[1500]=(if self.scalar_static_bool[248]{1.0}else{0.0});
        self.scalar_static_bool[249]=((self.scalar_static_f64[924]!=0.0)&&(self.scalar_static_f64[1500]!=0.0));
        self.scalar_static_f64[1501]=(1e-25+self.scalar_static_f64[1499]);
        self.scalar_static_f64[1502]=(if self.scalar_static_bool[249]{self.scalar_static_f64[1501]}else{self.scalar_static_f64[1426]});
        self.scalar_static_f64[1503]=(if (self.scalar_static_f64[1021]!=0.0){self.scalar_static_f64[1156]}else{self.scalar_static_f64[1157]});
        self.scalar_static_f64[1504]=(if (self.scalar_static_f64[1021]!=0.0){self.scalar_static_f64[1503]}else{self.scalar_static_f64[1158]});
        self.scalar_static_f64[1505]=(self.scalar_static_f64[1504]-self.scalar_static_f64[104]);
        self.scalar_static_f64[1506]=(if (self.scalar_static_f64[1021]!=0.0){self.scalar_static_f64[1505]}else{self.scalar_static_f64[1160]});
        self.scalar_static_f64[1507]=(self.scalar_static_f64[1504]*self.scalar_static_f64[1504]);
        self.scalar_static_f64[1508]=(self.scalar_static_f64[1507]-self.scalar_static_f64[926]);
        self.scalar_static_f64[1509]=(if (self.scalar_static_f64[1021]!=0.0){self.scalar_static_f64[1508]}else{self.scalar_static_f64[1163]});
        self.scalar_static_f64[1510]=(self.scalar_static_f64[944]*self.scalar_static_f64[1506]);
        self.scalar_static_f64[1511]=(1.0+self.scalar_static_f64[1510]);
        self.scalar_static_f64[1512]=(self.scalar_static_f64[945]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[1513]=(self.scalar_static_f64[1511]+self.scalar_static_f64[1512]);
        self.scalar_static_f64[1514]=(self.scalar_static_f64[42]*self.scalar_static_f64[1506]);
        self.scalar_static_f64[1515]=(self.scalar_static_f64[685]+self.scalar_static_f64[1514]);
        self.scalar_static_f64[1516]=(self.scalar_static_f64[43]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[1517]=(self.scalar_static_f64[1515]+self.scalar_static_f64[1516]);
        self.scalar_static_f64[1518]=(self.scalar_static_f64[922]*self.scalar_static_f64[1517]);
        self.scalar_static_f64[1519]=(if self.scalar_static_bool[175]{self.scalar_static_f64[1518]}else{self.scalar_static_f64[1244]});
        self.scalar_static_bool[250]=(self.scalar_static_f64[1519]<0.0001);
        self.scalar_static_f64[1520]=(if self.scalar_static_bool[250]{1.0}else{0.0});
        self.scalar_static_bool[251]=(self.scalar_static_bool[175]&&(self.scalar_static_f64[1520]!=0.0));
        self.scalar_static_f64[1521]=(if self.scalar_static_bool[251]{0.0001}else{self.scalar_static_f64[1519]});
        self.scalar_static_f64[1522]=(self.scalar_static_f64[49]*self.scalar_static_f64[1506]);
        self.scalar_static_f64[1523]=(self.scalar_static_f64[456]+self.scalar_static_f64[1522]);
        self.scalar_static_f64[1524]=(self.scalar_static_f64[50]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[1525]=(self.scalar_static_f64[1523]+self.scalar_static_f64[1524]);
        self.scalar_static_f64[1526]=(self.scalar_static_f64[91]*self.scalar_static_f64[1506]);
        self.scalar_static_f64[1527]=(self.scalar_static_f64[466]+self.scalar_static_f64[1526]);
        self.scalar_static_f64[1528]=(self.scalar_static_f64[92]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[1529]=(self.scalar_static_f64[1527]+self.scalar_static_f64[1528]);
        self.scalar_static_f64[1530]=(self.scalar_static_f64[546]+self.scalar_static_f64[1526]);
        self.scalar_static_f64[1531]=(self.scalar_static_f64[1528]+self.scalar_static_f64[1530]);
        self.scalar_static_f64[1532]=(self.scalar_static_f64[93]*self.scalar_static_f64[1506]);
        self.scalar_static_f64[1533]=(self.scalar_static_f64[684]+self.scalar_static_f64[1532]);
        self.scalar_static_f64[1534]=(self.scalar_static_f64[94]*self.scalar_static_f64[1509]);
        self.scalar_static_f64[1535]=(self.scalar_static_f64[1533]+self.scalar_static_f64[1534]);
        self.scalar_static_bool[252]=(self.scalar_static_f64[1521]>0.0001);
        self.scalar_static_f64[1536]=(if self.scalar_static_bool[252]{1.0}else{0.0});
        self.scalar_static_bool[253]=((self.scalar_static_f64[1119]!=0.0)&&(self.scalar_static_f64[1536]!=0.0));
        self.scalar_static_f64[1537]=(1.0/self.scalar_static_f64[1521]);
        self.scalar_static_f64[1538]=(if self.scalar_static_bool[253]{self.scalar_static_f64[1537]}else{0.0});
        self.scalar_static_bool[254]=(!(self.scalar_static_f64[1536]!=0.0));
        self.scalar_static_bool[255]=((self.scalar_static_f64[1119]!=0.0)&&self.scalar_static_bool[254]);
        self.scalar_static_f64[1539]=(if self.scalar_static_bool[255]{10000.0}else{self.scalar_static_f64[1538]});
        self.scalar_static_f64[1540]=(if self.scalar_static_bool[229]{0.0}else{self.scalar_static_f64[1539]});
        self.scalar_static_f64[1541]=(if (self.scalar_static_f64[1119]!=0.0){self.scalar_static_f64[1540]}else{0.0});
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
