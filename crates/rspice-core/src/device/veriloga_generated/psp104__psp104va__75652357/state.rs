#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;
use crate::device::veriloga_generated::kernel_runtime::{ReactiveScratch as KernelReactiveScratch, Scratch as KernelScratch};

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
    pub p864: f64, pub p865: f64, pub p866: f64, pub p867: f64, pub p868: f64, pub p869: f64, pub p870: f64, pub p871: f64,
    pub p872: f64, pub p873: f64, pub p874: f64, pub p875: f64, pub p876: f64, pub p877: f64, pub p878: f64, pub p879: f64,
    pub p880: f64, pub p881: f64, pub p882: f64, pub p883: f64, pub p884: f64, pub p885: f64, pub p886: f64, pub p887: f64,
    pub p888: f64, pub p889: f64, pub p890: f64, pub p891: f64, pub p892: f64, pub p893: f64, pub p894: f64, pub p895: f64,
    pub p896: f64, pub p897: f64, pub p898: f64, pub p899: f64, pub p900: f64, pub p901: f64, pub p902: f64, pub p903: f64,
    pub p904: f64, pub p905: f64, pub p906: f64, pub p907: f64, pub p908: f64, pub p909: f64, pub p910: f64, pub p911: f64,
    pub p912: f64, pub p913: f64, pub p914: f64, pub p915: f64, pub p916: f64, pub p917: f64, pub p918: f64, pub p919: f64,
    pub p920: f64, pub p921: f64, pub p922: f64, pub p923: f64, pub p924: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 34] = [
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 1e-7, 0.0, 0.0, 1e-6, 0.0,
                1.0, 0.0, 1.0, 1e-12, 1e-6, 1e-6, 1e-12, 1e-6,
                1e-6, 1e-12, 1e-6, 1e-12, 1e-6, 1.0, 1.0, 1.0,
                1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 34);
            {
                let params = &mut *ptr;
                params.p34 = params.p32;
                validate_parameter("MULT_FN", params.p34, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 890] = [
                0.0, 104.0, 1.0, 21.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0005, 0.0,
                2e-9, 3.9, 5e23, 1.0, 0.0, 1.0, 0.0, 1e26,
                2e-9, 2e-9, 5e25, 5e25, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 1.0,
                0.5, 0.0, 1.5, 1.5, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 1.0, 50.0, 1.0, 0.0, 0.0, 0.3,
                1.0, 0.0, 0.0, 1.0, 8.0, 0.01, 0.0, 0.0,
                0.05, 1.0, 10.0, 0.0, 1.0, 0.0, 10.0, 0.0,
                0.0, 0.0, 0.0, 2.0, 0.375, 0.063, 0.375, 0.063,
                0.375, 0.063, 3.1, 0.0, 0.0, 41.0, 41.0, 0.0,
                0.0, 0.0, 0.0, 1e-14, 0.0, 1.0, 0.1, 8.0,
                0.0, 0.0, 1e-15, 1e-15, 0.5, 0.5, 1.0, 1e-15,
                5e-16, 5e-16, 0.0, 0.3, 0.5, 0.4, 1e-15, 1e-15,
                1.0, 0.0, 8e22, 30000000.0, 0.0, 1.0, -1.0, 0.0005,
                0.0, 5e23, 0.0, 0.0006, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 4e24, 1500000000.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0005, 0.0,
                0.0, 0.0, 0.0, 2e-9, 3.9, 4e23, 0.0, 1e-8,
                1e24, 0.0, 1e-8, 1e-8, 0.0, 0.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 1e26, 0.0, 2e-9, 2e-9, 1e-8,
                1e-8, 5e25, 5e25, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0,
                1e-8, 0.0, 0.0, 1e-8, 0.0, 0.0, 1e-9, 1.0,
                0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 1.5, 1.5,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 50.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.3, 1.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 16.0,
                1.0, 0.01, 1.0, 0.0, 0.0, 0.5, 0.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.05, 1.0, 0.0, 0.0,
                10.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                10.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.375, 0.063,
                0.375, 0.063, 0.375, 0.063, 3.1, 0.0, 0.0, 41.0,
                41.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.1,
                1.0, 0.0, 0.0, 16.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.5, 0.5, 1.0, 1e-15,
                5e-16, 5e-16, 0.0, 0.3, 0.5, 0.4, 1e-15, 1e-15,
                1.0, 0.0, 8e22, 30000000.0, 0.0, 1.0, 0.0, 2.0,
                1e-8, 0.0, -1.0, 0.0005, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 5e23, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 1e-8, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 1.0, 8e22, 30000000.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
                0.0, 0.0005, 0.0, 0.0, 0.0, 5e23, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e26, 0.0, 0.0,
                0.0, 5e25, 0.0, 0.0, 0.0, 5e25, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.03, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.3, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0,
                0.0, 0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e-14, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.1, 0.0, 0.0, 0.0, 8.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-15, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0,
                0.0, 1e-15, 0.0, 0.0, 0.0, 5e-16, 0.0, 0.0,
                0.0, 5e-16, 0.0, 0.0, 0.0, 1e-15, 0.0, 0.0,
                0.0, 1e-15, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 8e22, 0.0, 0.0, 0.0, 30000000.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0,
                0.0, 0.0005, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 5e23, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.03, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 8e22, 0.0, 0.0, 0.0, 30000000.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 1e-6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1000.0, 21.0, 1000.0, 0.001,
                1e-9, 1e-9, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5,
                1.16, 1.16, 1.16, 1e-12, 1e-18, 1e-18, 100.0, 0.0001,
                0.0001, 1e-7, 1e-7, 100.0, 0.0001, 0.0001, 0.25, 0.25,
                0.25, 1e-12, 1e-18, 1e-18, 1000000000.0, 1000000000.0, 1000000000.0, -0.001,
                -0.001, -0.001, 10.0, 10.0, 10.0, 4.0, 4.0, 4.0,
                1.0, 1.0, 1.0, 1.0, -1.0, 0.1, 0.0, 0.5,
                0.0, 0.5, 0.001, 1e-9, 1e-9, 1.0, 1.0, 1.0,
                0.5, 0.5, 0.5, 1.16, 1.16, 1.16, 1e-12, 1e-18,
                1e-18, 100.0, 0.0001, 0.0001, 1e-7, 1e-7, 100.0, 0.0001,
                0.0001, 0.25, 0.25, 0.25, 1e-12, 1e-18, 1e-18, 1000000000.0,
                1000000000.0, 1000000000.0, -0.001, -0.001, -0.001, 10.0, 10.0, 10.0,
                4.0, 4.0, 4.0, 1.0, 1.0, 1.0, 1.0, -1.0,
                0.1, 0.0, 0.5, 0.0, 0.5, 0.0, 2.5, 0.03,
                2.5, 0.03,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(35), 890);
            let params = &*ptr;
            for index in 0..PARAMETER_DISPLAY_NAMES.len() {
                let value = read_parameter_slot(params, index);
                validate_parameter_metadata(params, index, value).expect("generated Verilog-A parameter defaults must satisfy declared ranges");
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

#[inline]
fn read_parameter_slot(parameters: &Parameters, index: usize) -> f64 {
    debug_assert!(index < PARAMETER_DISPLAY_NAMES.len(), "generated parameter index out of range");
    // SAFETY: Parameters is repr(C), contains only f64 fields, and every caller validates or generates the index.
    unsafe { *((parameters as *const Parameters as *const f64).add(index)) }
}

fn validate_parameter_scalar_metadata(index: usize, value: f64) -> Result<(), String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter index {} is out of range", index));
    };
    let flags = PARAMETER_RANGE_FLAGS[index];
    validate_finite_parameter(name, value)?;
    if PARAMETER_INTEGER_FLAGS[index] && value.fract() != 0.0 {
        return Err(format!("parameter '{}' must be an integer, got {}", name, value));
    }
    if PARAMETER_INTEGER_FLAGS[index] && (value < i32::MIN as f64 || value > i32::MAX as f64) {
        return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
    }
    validate_parameter_bounds(
        name,
        value,
        flags,
        PARAMETER_MIN_BOUNDS[index],
        PARAMETER_MAX_BOUNDS[index],
        PARAMETER_EXCLUDED_BOUNDS[index],
    )
}

fn validate_parameter_metadata(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    validate_parameter_scalar_metadata(index, value)?;
    let name = PARAMETER_DISPLAY_NAMES[index];
    let flags = PARAMETER_RANGE_FLAGS[index];
    let computed_min = parameter_computed_min_bound(parameters, index)?;
    let lower_source_count = usize::from(PARAMETER_MIN_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MIN_REFERENCES[index].is_some())
        + usize::from(computed_min.is_some());
    if lower_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting lower-bound sources", name));
    }
    let min = match PARAMETER_MIN_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_min.or(PARAMETER_MIN_BOUNDS[index]),
    };
    let computed_max = parameter_computed_max_bound(parameters, index)?;
    let upper_source_count = usize::from(PARAMETER_MAX_BOUNDS[index].is_some())
        + usize::from(PARAMETER_MAX_REFERENCES[index].is_some())
        + usize::from(computed_max.is_some());
    if upper_source_count > 1 {
        return Err(format!("parameter '{}' has conflicting upper-bound sources", name));
    }
    let max = match PARAMETER_MAX_REFERENCES[index] {
        Some(reference) => Some(parameter_bound_from_reference(parameters, reference)?),
        None => computed_max.or(PARAMETER_MAX_BOUNDS[index]),
    };
    if let (Some(min), Some(max)) = (min, max) {
        let empty = min.value > max.value
            || (min.value == max.value
                && flags & (PARAMETER_MIN_EXCLUSIVE_FLAG | PARAMETER_MAX_EXCLUSIVE_FLAG) != 0);
        if empty {
            return Err(format!(
                "parameter '{}' has an empty range: lower bound {}={} exceeds upper bound {}={}",
                name, min.label, min.value, max.label, max.value
            ));
        }
    }
    validate_parameter_bounds(name, value, flags, min, max, PARAMETER_EXCLUDED_BOUNDS[index])?;
    for &reference in PARAMETER_EXCLUDED_REFERENCES[index] {
        let excluded = parameter_bound_from_reference(parameters, reference)?;
        if value == excluded.value {
            return Err(format!(
                "parameter '{}' must not equal {}={}, got {}",
                name, excluded.label, excluded.value, value
            ));
        }
    }
    validate_parameter_computed_exclusions(parameters, index, value)?;
    Ok(())
}

fn parameter_bound_from_reference(
    parameters: &Parameters,
    index: usize,
) -> Result<ParameterBound, String> {
    let Some(&name) = PARAMETER_DISPLAY_NAMES.get(index) else {
        return Err(format!("generated parameter range reference {} is out of range", index));
    };
    let value = read_parameter_slot(parameters, index);
    validate_finite_parameter(name, value)?;
    Ok(ParameterBound { value, label: name })
}

fn validate_parameter_bounds(
    name: &str,
    value: f64,
    flags: u8,
    min: Option<ParameterBound>,
    max: Option<ParameterBound>,
    excluded: &[ParameterBound],
) -> Result<(), String> {
    if let Some(min) = min {
        if flags & PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
            if value <= min.value {
                return Err(format!("parameter '{}' must be > {}, got {}", name, min.label, value));
            }
        } else if value < min.value {
            return Err(format!("parameter '{}' must be >= {}, got {}", name, min.label, value));
        }
    }
    if let Some(max) = max {
        if flags & PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
            if value >= max.value {
                return Err(format!("parameter '{}' must be < {}, got {}", name, max.label, value));
            }
        } else if value > max.value {
            return Err(format!("parameter '{}' must be <= {}, got {}", name, max.label, value));
        }
    }
    for excluded in excluded {
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 927] = [
    ("l", 0), ("w", 1), ("sa", 2), ("sb", 3), ("sd", 4), ("sca", 5), ("scb", 6), ("scc", 7), ("sc", 8), ("nf", 9), ("ngcon", 10), ("xgw", 11), ("nrs", 12), ("nrd", 13), ("jw", 14), ("delvto", 15),
    ("factuo", 16), ("delvtoedge", 17), ("factuoedge", 18), ("absource", 19), ("lssource", 20), ("lgsource", 21), ("abdrain", 22), ("lsdrain", 23), ("lgdrain", 24), ("as", 25), ("ps", 26), ("ad", 27), ("pd", 28), ("ifactor", 29), ("cfactor", 30), ("mult", 31),
    ("mult_i", 32), ("mult_q", 33), ("mult_fn", 34), ("trise", 35), ("dtemp", 35), ("level", 36), ("type", 37), ("tr", 38), ("tref", 38), ("swgeo", 39), ("swigate", 40), ("swimpact", 41), ("swgidl", 42), ("swjuncap", 43), ("swjunasym", 44), ("swnud", 45),
    ("swedge", 46), ("swdelvtac", 47), ("swqsat", 48), ("swqpart", 49), ("swign", 50), ("qmc", 51), ("swoprext", 52), ("swoppmos", 53), ("swopdrain", 54), ("dta", 55), ("vfb", 56), ("stvfb", 57), ("st2vfb", 58), ("tox", 59), ("epsrox", 60), ("neff", 61),
    ("gfacnud", 62), ("vsbnud", 63), ("dvsbnud", 64), ("dphib", 65), ("np", 66), ("toxov", 67), ("toxovd", 68), ("nov", 69), ("novd", 70), ("ct", 71), ("ctb", 72), ("ctg", 73), ("stct", 74), ("cf", 75), ("cfb", 76), ("cfd", 77),
    ("psce", 78), ("psceb", 79), ("psced", 80), ("betn", 81), ("stbet", 82), ("mue", 83), ("stmue", 84), ("themu", 85), ("stthemu", 86), ("cs", 87), ("stcs", 88), ("thecs", 89), ("stthecs", 90), ("xcor", 91), ("stxcor", 92), ("feta", 93),
    ("rs", 94), ("strs", 95), ("rsb", 96), ("rsg", 97), ("thesat", 98), ("stthesat", 99), ("thesatb", 100), ("thesatg", 101), ("thesatt", 102), ("ax", 103), ("alp", 104), ("alp1", 105), ("alp2", 106), ("vp", 107), ("a1", 108), ("a2", 109),
    ("sta2", 110), ("a3", 111), ("a4", 112), ("imaxii", 113), ("gco", 114), ("iginv", 115), ("igov", 116), ("igovd", 117), ("stig", 118), ("gc2", 119), ("gc3", 120), ("gc2ov", 121), ("gc3ov", 122), ("gc2ovd", 123), ("gc3ovd", 124), ("chib", 125),
    ("agidl", 126), ("agidld", 127), ("bgidl", 128), ("bgidld", 129), ("stbgidl", 130), ("stbgidld", 131), ("cgidl", 132), ("cgidld", 133), ("cox", 134), ("delvtac", 135), ("facneffac", 136), ("thesatac", 137), ("axac", 138), ("alpac", 139), ("alp1ac", 140), ("cgov", 141),
    ("cgovd", 142), ("fcgovacc", 143), ("fcgovaccd", 144), ("cgovaccg", 145), ("cgbov", 146), ("cinr", 147), ("cinrd", 148), ("dvfbinr", 149), ("fcinrdep", 150), ("fcinracc", 151), ("axinr", 152), ("cfr", 153), ("cfrd", 154), ("fnt", 155), ("fntexc", 156), ("nfa", 157),
    ("nfb", 158), ("nfc", 159), ("ef", 160), ("vfbedge", 161), ("stvfbedge", 162), ("dphibedge", 163), ("neffedge", 164), ("ctedge", 165), ("betnedge", 166), ("stbetedge", 167), ("psceedge", 168), ("pscebedge", 169), ("pscededge", 170), ("cfedge", 171), ("cfbedge", 172), ("cfdedge", 173),
    ("fntedge", 174), ("nfaedge", 175), ("nfbedge", 176), ("nfcedge", 177), ("efedge", 178), ("rg", 179), ("rse", 180), ("rde", 181), ("rbulk", 182), ("rwell", 183), ("rjuns", 184), ("rjund", 185), ("lvaro", 186), ("lvarl", 187), ("lvarw", 188), ("lap", 189),
    ("wvaro", 190), ("wvarl", 191), ("wvarw", 192), ("wot", 193), ("dlq", 194), ("dwq", 195), ("vfbo", 196), ("vfbl", 197), ("vfblexp", 198), ("vfbw", 199), ("vfblw", 200), ("stvfbo", 201), ("stvfbl", 202), ("stvfbw", 203), ("stvfblw", 204), ("st2vfbo", 205),
    ("toxo", 206), ("epsroxo", 207), ("nsubo", 208), ("nsubw", 209), ("wseg", 210), ("npck", 211), ("npckw", 212), ("wsegp", 213), ("lpck", 214), ("lpckw", 215), ("fol1", 216), ("fol2", 217), ("gfacnudo", 218), ("gfacnudl", 219), ("gfacnudlexp", 220), ("gfacnudw", 221),
    ("gfacnudlw", 222), ("vsbnudo", 223), ("dvsbnudo", 224), ("dphibo", 225), ("dphibl", 226), ("dphiblexp", 227), ("dphibw", 228), ("dphiblw", 229), ("npo", 230), ("npl", 231), ("toxovo", 232), ("toxovdo", 233), ("lov", 234), ("lovd", 235), ("novo", 236), ("novdo", 237),
    ("cto", 238), ("ctl", 239), ("ctlexp", 240), ("ctw", 241), ("ctlw", 242), ("ctbo", 243), ("ctgo", 244), ("stcto", 245), ("cfl", 246), ("cflexp", 247), ("cfw", 248), ("cfbo", 249), ("cfdo", 250), ("pscel", 251), ("pscelexp", 252), ("pscew", 253),
    ("pscebo", 254), ("pscedo", 255), ("uo", 256), ("fbet1", 257), ("fbet1w", 258), ("lp1", 259), ("lp1w", 260), ("fbet2", 261), ("lp2", 262), ("betw1", 263), ("betw2", 264), ("wbet", 265), ("stbeto", 266), ("stbetl", 267), ("stbetw", 268), ("stbetlw", 269),
    ("mueo", 270), ("muew", 271), ("stmueo", 272), ("themuo", 273), ("stthemuo", 274), ("cso", 275), ("csl", 276), ("cslexp", 277), ("csw", 278), ("cslw", 279), ("stcso", 280), ("thecso", 281), ("stthecso", 282), ("xcoro", 283), ("xcorl", 284), ("xcorw", 285),
    ("xcorlw", 286), ("stxcoro", 287), ("fetao", 288), ("rsw1", 289), ("rsw2", 290), ("strso", 291), ("rsbo", 292), ("rsgo", 293), ("thesato", 294), ("thesatl", 295), ("thesatlexp", 296), ("thesatw", 297), ("thesatlw", 298), ("stthesato", 299), ("stthesatl", 300), ("stthesatw", 301),
    ("stthesatlw", 302), ("thesatbo", 303), ("thesatgo", 304), ("thesatto", 305), ("axo", 306), ("axl", 307), ("alpl", 308), ("alplexp", 309), ("alpw", 310), ("alp1l1", 311), ("alp1lexp", 312), ("alp1l2", 313), ("alp1w", 314), ("alp2l1", 315), ("alp2lexp", 316), ("alp2l2", 317),
    ("alp2w", 318), ("vpo", 319), ("a1o", 320), ("a1l", 321), ("a1w", 322), ("a2o", 323), ("sta2o", 324), ("a3o", 325), ("a3l", 326), ("a3w", 327), ("a4o", 328), ("a4l", 329), ("a4w", 330), ("imaxiio", 331), ("gcoo", 332), ("iginvlw", 333),
    ("igovw", 334), ("igovdw", 335), ("stigo", 336), ("gc2o", 337), ("gc3o", 338), ("gc2ovo", 339), ("gc3ovo", 340), ("gc2ovdo", 341), ("gc3ovdo", 342), ("chibo", 343), ("agidlw", 344), ("agidldw", 345), ("bgidlo", 346), ("bgidldo", 347), ("stbgidlo", 348), ("stbgidldo", 349),
    ("cgidlo", 350), ("cgidldo", 351), ("delvtaco", 352), ("delvtacl", 353), ("delvtaclexp", 354), ("delvtacw", 355), ("delvtaclw", 356), ("facneffaco", 357), ("facneffacl", 358), ("facneffacw", 359), ("facneffaclw", 360), ("thesataco", 361), ("thesatacl", 362), ("thesataclexp", 363), ("thesatacw", 364), ("thesataclw", 365),
    ("axaco", 366), ("axacl", 367), ("alpacl", 368), ("alpaclexp", 369), ("alpacw", 370), ("alp1acl1", 371), ("alp1aclexp", 372), ("alp1acl2", 373), ("alp1acw", 374), ("fcgovacco", 375), ("fcgovaccdo", 376), ("cgovaccgo", 377), ("cgbovl", 378), ("cinrw", 379), ("cinrdw", 380), ("dvfbinro", 381),
    ("fcinrdepo", 382), ("fcinracco", 383), ("axinro", 384), ("cfrw", 385), ("cfrdw", 386), ("fnto", 387), ("fntexcl", 388), ("nfalw", 389), ("nfblw", 390), ("nfclw", 391), ("efo", 392), ("lintnoi", 393), ("alpnoi", 394), ("wedge", 395), ("wedgew", 396), ("vfbedgeo", 397),
    ("stvfbedgeo", 398), ("stvfbedgel", 399), ("stvfbedgew", 400), ("stvfbedgelw", 401), ("dphibedgeo", 402), ("dphibedgel", 403), ("dphibedgelexp", 404), ("dphibedgew", 405), ("dphibedgelw", 406), ("nsubedgeo", 407), ("nsubedgel", 408), ("nsubedgelexp", 409), ("nsubedgew", 410), ("nsubedgelw", 411), ("ctedgeo", 412), ("ctedgel", 413),
    ("ctedgelexp", 414), ("fbetedge", 415), ("lpedge", 416), ("betedgew", 417), ("stbetedgeo", 418), ("stbetedgel", 419), ("stbetedgew", 420), ("stbetedgelw", 421), ("psceedgel", 422), ("psceedgelexp", 423), ("psceedgew", 424), ("pscebedgeo", 425), ("pscededgeo", 426), ("cfedgel", 427), ("cfedgelexp", 428), ("cfedgew", 429),
    ("cfbedgeo", 430), ("cfdedgeo", 431), ("fntedgeo", 432), ("nfaedgelw", 433), ("nfbedgelw", 434), ("nfcedgelw", 435), ("efedgeo", 436), ("rgo", 437), ("rint", 438), ("rvpoly", 439), ("rshg", 440), ("dlsil", 441), ("rsh", 442), ("rshd", 443), ("rbulko", 444), ("rwello", 445),
    ("rjunso", 446), ("rjundo", 447), ("povfb", 448), ("plvfb", 449), ("pwvfb", 450), ("plwvfb", 451), ("postvfb", 452), ("plstvfb", 453), ("pwstvfb", 454), ("plwstvfb", 455), ("poneff", 456), ("plneff", 457), ("pwneff", 458), ("plwneff", 459), ("pogfacnud", 460), ("plgfacnud", 461),
    ("pwgfacnud", 462), ("plwgfacnud", 463), ("povsbnud", 464), ("plvsbnud", 465), ("pwvsbnud", 466), ("plwvsbnud", 467), ("podphib", 468), ("pldphib", 469), ("pwdphib", 470), ("plwdphib", 471), ("ponp", 472), ("plnp", 473), ("pwnp", 474), ("plwnp", 475), ("ponov", 476), ("plnov", 477),
    ("pwnov", 478), ("plwnov", 479), ("ponovd", 480), ("plnovd", 481), ("pwnovd", 482), ("plwnovd", 483), ("poct", 484), ("plct", 485), ("pwct", 486), ("plwct", 487), ("poctb", 488), ("plctb", 489), ("pwctb", 490), ("plwctb", 491), ("poctg", 492), ("plctg", 493),
    ("pwctg", 494), ("plwctg", 495), ("postct", 496), ("plstct", 497), ("pwstct", 498), ("plwstct", 499), ("pocf", 500), ("plcf", 501), ("pwcf", 502), ("plwcf", 503), ("pocfb", 504), ("plcfb", 505), ("pwcfb", 506), ("plwcfb", 507), ("pocfd", 508), ("plcfd", 509),
    ("pwcfd", 510), ("plwcfd", 511), ("popsce", 512), ("plpsce", 513), ("pwpsce", 514), ("plwpsce", 515), ("popsceb", 516), ("plpsceb", 517), ("pwpsceb", 518), ("plwpsceb", 519), ("popsced", 520), ("plpsced", 521), ("pwpsced", 522), ("plwpsced", 523), ("pobetn", 524), ("plbetn", 525),
    ("pwbetn", 526), ("plwbetn", 527), ("postbet", 528), ("plstbet", 529), ("pwstbet", 530), ("plwstbet", 531), ("pomue", 532), ("plmue", 533), ("pwmue", 534), ("plwmue", 535), ("pothemu", 536), ("plthemu", 537), ("pwthemu", 538), ("plwthemu", 539), ("pocs", 540), ("plcs", 541),
    ("pwcs", 542), ("plwcs", 543), ("pothecs", 544), ("plthecs", 545), ("pwthecs", 546), ("plwthecs", 547), ("poxcor", 548), ("plxcor", 549), ("pwxcor", 550), ("plwxcor", 551), ("pors", 552), ("plrs", 553), ("pwrs", 554), ("plwrs", 555), ("postrs", 556), ("plstrs", 557),
    ("pwstrs", 558), ("plwstrs", 559), ("porsb", 560), ("plrsb", 561), ("pwrsb", 562), ("plwrsb", 563), ("porsg", 564), ("plrsg", 565), ("pwrsg", 566), ("plwrsg", 567), ("pothesat", 568), ("plthesat", 569), ("pwthesat", 570), ("plwthesat", 571), ("postthesat", 572), ("plstthesat", 573),
    ("pwstthesat", 574), ("plwstthesat", 575), ("pothesatb", 576), ("plthesatb", 577), ("pwthesatb", 578), ("plwthesatb", 579), ("pothesatg", 580), ("plthesatg", 581), ("pwthesatg", 582), ("plwthesatg", 583), ("poax", 584), ("plax", 585), ("pwax", 586), ("plwax", 587), ("poalp", 588), ("plalp", 589),
    ("pwalp", 590), ("plwalp", 591), ("poalp1", 592), ("plalp1", 593), ("pwalp1", 594), ("plwalp1", 595), ("poalp2", 596), ("plalp2", 597), ("pwalp2", 598), ("plwalp2", 599), ("poa1", 600), ("pla1", 601), ("pwa1", 602), ("plwa1", 603), ("posta2", 604), ("plsta2", 605),
    ("pwsta2", 606), ("plwsta2", 607), ("poa3", 608), ("pla3", 609), ("pwa3", 610), ("plwa3", 611), ("poa4", 612), ("pla4", 613), ("pwa4", 614), ("plwa4", 615), ("poiginv", 616), ("pliginv", 617), ("pwiginv", 618), ("plwiginv", 619), ("poigov", 620), ("pligov", 621),
    ("pwigov", 622), ("plwigov", 623), ("poigovd", 624), ("pligovd", 625), ("pwigovd", 626), ("plwigovd", 627), ("postig", 628), ("plstig", 629), ("pwstig", 630), ("plwstig", 631), ("poagidl", 632), ("plagidl", 633), ("pwagidl", 634), ("plwagidl", 635), ("poagidld", 636), ("plagidld", 637),
    ("pwagidld", 638), ("plwagidld", 639), ("postbgidl", 640), ("plstbgidl", 641), ("pwstbgidl", 642), ("plwstbgidl", 643), ("postbgidld", 644), ("plstbgidld", 645), ("pwstbgidld", 646), ("plwstbgidld", 647), ("pocox", 648), ("plcox", 649), ("pwcox", 650), ("plwcox", 651), ("podelvtac", 652), ("pldelvtac", 653),
    ("pwdelvtac", 654), ("plwdelvtac", 655), ("pofacneffac", 656), ("plfacneffac", 657), ("pwfacneffac", 658), ("plwfacneffac", 659), ("pothesatac", 660), ("plthesatac", 661), ("pwthesatac", 662), ("plwthesatac", 663), ("poaxac", 664), ("plaxac", 665), ("pwaxac", 666), ("plwaxac", 667), ("poalpac", 668), ("plalpac", 669),
    ("pwalpac", 670), ("plwalpac", 671), ("poalp1ac", 672), ("plalp1ac", 673), ("pwalp1ac", 674), ("plwalp1ac", 675), ("pocgov", 676), ("plcgov", 677), ("pwcgov", 678), ("plwcgov", 679), ("pocgovd", 680), ("plcgovd", 681), ("pwcgovd", 682), ("plwcgovd", 683), ("pocgbov", 684), ("plcgbov", 685),
    ("pwcgbov", 686), ("plwcgbov", 687), ("pocinr", 688), ("plcinr", 689), ("pwcinr", 690), ("plwcinr", 691), ("pocinrd", 692), ("plcinrd", 693), ("pwcinrd", 694), ("plwcinrd", 695), ("pocfr", 696), ("plcfr", 697), ("pwcfr", 698), ("plwcfr", 699), ("pocfrd", 700), ("plcfrd", 701),
    ("pwcfrd", 702), ("plwcfrd", 703), ("pofntexc", 704), ("plfntexc", 705), ("pwfntexc", 706), ("plwfntexc", 707), ("ponfa", 708), ("plnfa", 709), ("pwnfa", 710), ("plwnfa", 711), ("ponfb", 712), ("plnfb", 713), ("pwnfb", 714), ("plwnfb", 715), ("ponfc", 716), ("plnfc", 717),
    ("pwnfc", 718), ("plwnfc", 719), ("povfbedge", 720), ("plvfbedge", 721), ("pwvfbedge", 722), ("plwvfbedge", 723), ("postvfbedge", 724), ("plstvfbedge", 725), ("pwstvfbedge", 726), ("plwstvfbedge", 727), ("podphibedge", 728), ("pldphibedge", 729), ("pwdphibedge", 730), ("plwdphibedge", 731), ("poneffedge", 732), ("plneffedge", 733),
    ("pwneffedge", 734), ("plwneffedge", 735), ("poctedge", 736), ("plctedge", 737), ("pwctedge", 738), ("plwctedge", 739), ("pobetnedge", 740), ("plbetnedge", 741), ("pwbetnedge", 742), ("plwbetnedge", 743), ("postbetedge", 744), ("plstbetedge", 745), ("pwstbetedge", 746), ("plwstbetedge", 747), ("popsceedge", 748), ("plpsceedge", 749),
    ("pwpsceedge", 750), ("plwpsceedge", 751), ("popscebedge", 752), ("plpscebedge", 753), ("pwpscebedge", 754), ("plwpscebedge", 755), ("popscededge", 756), ("plpscededge", 757), ("pwpscededge", 758), ("plwpscededge", 759), ("pocfedge", 760), ("plcfedge", 761), ("pwcfedge", 762), ("plwcfedge", 763), ("pocfbedge", 764), ("plcfbedge", 765),
    ("pwcfbedge", 766), ("plwcfbedge", 767), ("pocfdedge", 768), ("plcfdedge", 769), ("pwcfdedge", 770), ("plwcfdedge", 771), ("ponfaedge", 772), ("plnfaedge", 773), ("pwnfaedge", 774), ("plwnfaedge", 775), ("ponfbedge", 776), ("plnfbedge", 777), ("pwnfbedge", 778), ("plwnfbedge", 779), ("ponfcedge", 780), ("plnfcedge", 781),
    ("pwnfcedge", 782), ("plwnfcedge", 783), ("saref", 784), ("sbref", 785), ("wlod", 786), ("kuo", 787), ("kvsat", 788), ("kvsatac", 789), ("tkuo", 790), ("lkuo", 791), ("wkuo", 792), ("pkuo", 793), ("llodkuo", 794), ("wlodkuo", 795), ("kvtho", 796), ("lkvtho", 797),
    ("wkvtho", 798), ("pkvtho", 799), ("llodvth", 800), ("wlodvth", 801), ("stetao", 802), ("lodetao", 803), ("scref", 804), ("web", 805), ("wec", 806), ("kvthoweo", 807), ("kvthowel", 808), ("kvthowew", 809), ("kvthowelw", 810), ("kuoweo", 811), ("kuowel", 812), ("kuowew", 813),
    ("kuowelw", 814), ("imax", 815), ("trj", 816), ("frev", 817), ("cjorbot", 818), ("cjorsti", 819), ("cjorgat", 820), ("vbirbot", 821), ("vbirsti", 822), ("vbirgat", 823), ("pbot", 824), ("psti", 825), ("pgat", 826), ("phigbot", 827), ("phigsti", 828), ("phiggat", 829),
    ("idsatrbot", 830), ("idsatrsti", 831), ("idsatrgat", 832), ("csrhbot", 833), ("csrhsti", 834), ("csrhgat", 835), ("xjunsti", 836), ("xjungat", 837), ("ctatbot", 838), ("ctatsti", 839), ("ctatgat", 840), ("mefftatbot", 841), ("mefftatsti", 842), ("mefftatgat", 843), ("cbbtbot", 844), ("cbbtsti", 845),
    ("cbbtgat", 846), ("fbbtrbot", 847), ("fbbtrsti", 848), ("fbbtrgat", 849), ("stfbbtbot", 850), ("stfbbtsti", 851), ("stfbbtgat", 852), ("vbrbot", 853), ("vbrsti", 854), ("vbrgat", 855), ("pbrbot", 856), ("pbrsti", 857), ("pbrgat", 858), ("fcjorgat2", 859), ("fvbirgat2", 860), ("fpgat2", 861),
    ("fphiggat2", 862), ("vtrgat", 863), ("anugat", 864), ("advbrgat", 865), ("bdvbrgat", 866), ("adbbtgat", 867), ("bdbbtgat", 868), ("cjorbotd", 869), ("cjorstid", 870), ("cjorgatd", 871), ("vbirbotd", 872), ("vbirstid", 873), ("vbirgatd", 874), ("pbotd", 875), ("pstid", 876), ("pgatd", 877),
    ("phigbotd", 878), ("phigstid", 879), ("phiggatd", 880), ("idsatrbotd", 881), ("idsatrstid", 882), ("idsatrgatd", 883), ("csrhbotd", 884), ("csrhstid", 885), ("csrhgatd", 886), ("xjunstid", 887), ("xjungatd", 888), ("ctatbotd", 889), ("ctatstid", 890), ("ctatgatd", 891), ("mefftatbotd", 892), ("mefftatstid", 893),
    ("mefftatgatd", 894), ("cbbtbotd", 895), ("cbbtstid", 896), ("cbbtgatd", 897), ("fbbtrbotd", 898), ("fbbtrstid", 899), ("fbbtrgatd", 900), ("stfbbtbotd", 901), ("stfbbtstid", 902), ("stfbbtgatd", 903), ("vbrbotd", 904), ("vbrstid", 905), ("vbrgatd", 906), ("pbrbotd", 907), ("pbrstid", 908), ("pbrgatd", 909),
    ("fcjorgat2d", 910), ("fvbirgat2d", 911), ("fpgat2d", 912), ("fphiggat2d", 913), ("vtrgatd", 914), ("anugatd", 915), ("advbrgatd", 916), ("bdvbrgatd", 917), ("adbbtgatd", 918), ("bdbbtgatd", 919), ("swjunexp", 920), ("vjunref", 921), ("fjunq", 922), ("vjunrefd", 923), ("fjunqd", 924),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 925] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 925] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 925] = [
    "L", "W", "SA", "SB", "SD", "SCA", "SCB", "SCC", "SC", "NF", "NGCON", "XGW", "NRS", "NRD", "JW", "DELVTO",
    "FACTUO", "DELVTOEDGE", "FACTUOEDGE", "ABSOURCE", "LSSOURCE", "LGSOURCE", "ABDRAIN", "LSDRAIN", "LGDRAIN", "AS", "PS", "AD", "PD", "IFACTOR", "CFACTOR", "MULT",
    "MULT_I", "MULT_Q", "MULT_FN", "TRISE", "LEVEL", "TYPE", "TR", "SWGEO", "SWIGATE", "SWIMPACT", "SWGIDL", "SWJUNCAP", "SWJUNASYM", "SWNUD", "SWEDGE", "SWDELVTAC",
    "SWQSAT", "SWQPART", "SWIGN", "QMC", "SWOPREXT", "SWOPPMOS", "SWOPDRAIN", "DTA", "VFB", "STVFB", "ST2VFB", "TOX", "EPSROX", "NEFF", "GFACNUD", "VSBNUD",
    "DVSBNUD", "DPHIB", "NP", "TOXOV", "TOXOVD", "NOV", "NOVD", "CT", "CTB", "CTG", "STCT", "CF", "CFB", "CFD", "PSCE", "PSCEB",
    "PSCED", "BETN", "STBET", "MUE", "STMUE", "THEMU", "STTHEMU", "CS", "STCS", "THECS", "STTHECS", "XCOR", "STXCOR", "FETA", "RS", "STRS",
    "RSB", "RSG", "THESAT", "STTHESAT", "THESATB", "THESATG", "THESATT", "AX", "ALP", "ALP1", "ALP2", "VP", "A1", "A2", "STA2", "A3",
    "A4", "IMAXII", "GCO", "IGINV", "IGOV", "IGOVD", "STIG", "GC2", "GC3", "GC2OV", "GC3OV", "GC2OVD", "GC3OVD", "CHIB", "AGIDL", "AGIDLD",
    "BGIDL", "BGIDLD", "STBGIDL", "STBGIDLD", "CGIDL", "CGIDLD", "COX", "DELVTAC", "FACNEFFAC", "THESATAC", "AXAC", "ALPAC", "ALP1AC", "CGOV", "CGOVD", "FCGOVACC",
    "FCGOVACCD", "CGOVACCG", "CGBOV", "CINR", "CINRD", "DVFBINR", "FCINRDEP", "FCINRACC", "AXINR", "CFR", "CFRD", "FNT", "FNTEXC", "NFA", "NFB", "NFC",
    "EF", "VFBEDGE", "STVFBEDGE", "DPHIBEDGE", "NEFFEDGE", "CTEDGE", "BETNEDGE", "STBETEDGE", "PSCEEDGE", "PSCEBEDGE", "PSCEDEDGE", "CFEDGE", "CFBEDGE", "CFDEDGE", "FNTEDGE", "NFAEDGE",
    "NFBEDGE", "NFCEDGE", "EFEDGE", "RG", "RSE", "RDE", "RBULK", "RWELL", "RJUNS", "RJUND", "LVARO", "LVARL", "LVARW", "LAP", "WVARO", "WVARL",
    "WVARW", "WOT", "DLQ", "DWQ", "VFBO", "VFBL", "VFBLEXP", "VFBW", "VFBLW", "STVFBO", "STVFBL", "STVFBW", "STVFBLW", "ST2VFBO", "TOXO", "EPSROXO",
    "NSUBO", "NSUBW", "WSEG", "NPCK", "NPCKW", "WSEGP", "LPCK", "LPCKW", "FOL1", "FOL2", "GFACNUDO", "GFACNUDL", "GFACNUDLEXP", "GFACNUDW", "GFACNUDLW", "VSBNUDO",
    "DVSBNUDO", "DPHIBO", "DPHIBL", "DPHIBLEXP", "DPHIBW", "DPHIBLW", "NPO", "NPL", "TOXOVO", "TOXOVDO", "LOV", "LOVD", "NOVO", "NOVDO", "CTO", "CTL",
    "CTLEXP", "CTW", "CTLW", "CTBO", "CTGO", "STCTO", "CFL", "CFLEXP", "CFW", "CFBO", "CFDO", "PSCEL", "PSCELEXP", "PSCEW", "PSCEBO", "PSCEDO",
    "UO", "FBET1", "FBET1W", "LP1", "LP1W", "FBET2", "LP2", "BETW1", "BETW2", "WBET", "STBETO", "STBETL", "STBETW", "STBETLW", "MUEO", "MUEW",
    "STMUEO", "THEMUO", "STTHEMUO", "CSO", "CSL", "CSLEXP", "CSW", "CSLW", "STCSO", "THECSO", "STTHECSO", "XCORO", "XCORL", "XCORW", "XCORLW", "STXCORO",
    "FETAO", "RSW1", "RSW2", "STRSO", "RSBO", "RSGO", "THESATO", "THESATL", "THESATLEXP", "THESATW", "THESATLW", "STTHESATO", "STTHESATL", "STTHESATW", "STTHESATLW", "THESATBO",
    "THESATGO", "THESATTO", "AXO", "AXL", "ALPL", "ALPLEXP", "ALPW", "ALP1L1", "ALP1LEXP", "ALP1L2", "ALP1W", "ALP2L1", "ALP2LEXP", "ALP2L2", "ALP2W", "VPO",
    "A1O", "A1L", "A1W", "A2O", "STA2O", "A3O", "A3L", "A3W", "A4O", "A4L", "A4W", "IMAXIIO", "GCOO", "IGINVLW", "IGOVW", "IGOVDW",
    "STIGO", "GC2O", "GC3O", "GC2OVO", "GC3OVO", "GC2OVDO", "GC3OVDO", "CHIBO", "AGIDLW", "AGIDLDW", "BGIDLO", "BGIDLDO", "STBGIDLO", "STBGIDLDO", "CGIDLO", "CGIDLDO",
    "DELVTACO", "DELVTACL", "DELVTACLEXP", "DELVTACW", "DELVTACLW", "FACNEFFACO", "FACNEFFACL", "FACNEFFACW", "FACNEFFACLW", "THESATACO", "THESATACL", "THESATACLEXP", "THESATACW", "THESATACLW", "AXACO", "AXACL",
    "ALPACL", "ALPACLEXP", "ALPACW", "ALP1ACL1", "ALP1ACLEXP", "ALP1ACL2", "ALP1ACW", "FCGOVACCO", "FCGOVACCDO", "CGOVACCGO", "CGBOVL", "CINRW", "CINRDW", "DVFBINRO", "FCINRDEPO", "FCINRACCO",
    "AXINRO", "CFRW", "CFRDW", "FNTO", "FNTEXCL", "NFALW", "NFBLW", "NFCLW", "EFO", "LINTNOI", "ALPNOI", "WEDGE", "WEDGEW", "VFBEDGEO", "STVFBEDGEO", "STVFBEDGEL",
    "STVFBEDGEW", "STVFBEDGELW", "DPHIBEDGEO", "DPHIBEDGEL", "DPHIBEDGELEXP", "DPHIBEDGEW", "DPHIBEDGELW", "NSUBEDGEO", "NSUBEDGEL", "NSUBEDGELEXP", "NSUBEDGEW", "NSUBEDGELW", "CTEDGEO", "CTEDGEL", "CTEDGELEXP", "FBETEDGE",
    "LPEDGE", "BETEDGEW", "STBETEDGEO", "STBETEDGEL", "STBETEDGEW", "STBETEDGELW", "PSCEEDGEL", "PSCEEDGELEXP", "PSCEEDGEW", "PSCEBEDGEO", "PSCEDEDGEO", "CFEDGEL", "CFEDGELEXP", "CFEDGEW", "CFBEDGEO", "CFDEDGEO",
    "FNTEDGEO", "NFAEDGELW", "NFBEDGELW", "NFCEDGELW", "EFEDGEO", "RGO", "RINT", "RVPOLY", "RSHG", "DLSIL", "RSH", "RSHD", "RBULKO", "RWELLO", "RJUNSO", "RJUNDO",
    "POVFB", "PLVFB", "PWVFB", "PLWVFB", "POSTVFB", "PLSTVFB", "PWSTVFB", "PLWSTVFB", "PONEFF", "PLNEFF", "PWNEFF", "PLWNEFF", "POGFACNUD", "PLGFACNUD", "PWGFACNUD", "PLWGFACNUD",
    "POVSBNUD", "PLVSBNUD", "PWVSBNUD", "PLWVSBNUD", "PODPHIB", "PLDPHIB", "PWDPHIB", "PLWDPHIB", "PONP", "PLNP", "PWNP", "PLWNP", "PONOV", "PLNOV", "PWNOV", "PLWNOV",
    "PONOVD", "PLNOVD", "PWNOVD", "PLWNOVD", "POCT", "PLCT", "PWCT", "PLWCT", "POCTB", "PLCTB", "PWCTB", "PLWCTB", "POCTG", "PLCTG", "PWCTG", "PLWCTG",
    "POSTCT", "PLSTCT", "PWSTCT", "PLWSTCT", "POCF", "PLCF", "PWCF", "PLWCF", "POCFB", "PLCFB", "PWCFB", "PLWCFB", "POCFD", "PLCFD", "PWCFD", "PLWCFD",
    "POPSCE", "PLPSCE", "PWPSCE", "PLWPSCE", "POPSCEB", "PLPSCEB", "PWPSCEB", "PLWPSCEB", "POPSCED", "PLPSCED", "PWPSCED", "PLWPSCED", "POBETN", "PLBETN", "PWBETN", "PLWBETN",
    "POSTBET", "PLSTBET", "PWSTBET", "PLWSTBET", "POMUE", "PLMUE", "PWMUE", "PLWMUE", "POTHEMU", "PLTHEMU", "PWTHEMU", "PLWTHEMU", "POCS", "PLCS", "PWCS", "PLWCS",
    "POTHECS", "PLTHECS", "PWTHECS", "PLWTHECS", "POXCOR", "PLXCOR", "PWXCOR", "PLWXCOR", "PORS", "PLRS", "PWRS", "PLWRS", "POSTRS", "PLSTRS", "PWSTRS", "PLWSTRS",
    "PORSB", "PLRSB", "PWRSB", "PLWRSB", "PORSG", "PLRSG", "PWRSG", "PLWRSG", "POTHESAT", "PLTHESAT", "PWTHESAT", "PLWTHESAT", "POSTTHESAT", "PLSTTHESAT", "PWSTTHESAT", "PLWSTTHESAT",
    "POTHESATB", "PLTHESATB", "PWTHESATB", "PLWTHESATB", "POTHESATG", "PLTHESATG", "PWTHESATG", "PLWTHESATG", "POAX", "PLAX", "PWAX", "PLWAX", "POALP", "PLALP", "PWALP", "PLWALP",
    "POALP1", "PLALP1", "PWALP1", "PLWALP1", "POALP2", "PLALP2", "PWALP2", "PLWALP2", "POA1", "PLA1", "PWA1", "PLWA1", "POSTA2", "PLSTA2", "PWSTA2", "PLWSTA2",
    "POA3", "PLA3", "PWA3", "PLWA3", "POA4", "PLA4", "PWA4", "PLWA4", "POIGINV", "PLIGINV", "PWIGINV", "PLWIGINV", "POIGOV", "PLIGOV", "PWIGOV", "PLWIGOV",
    "POIGOVD", "PLIGOVD", "PWIGOVD", "PLWIGOVD", "POSTIG", "PLSTIG", "PWSTIG", "PLWSTIG", "POAGIDL", "PLAGIDL", "PWAGIDL", "PLWAGIDL", "POAGIDLD", "PLAGIDLD", "PWAGIDLD", "PLWAGIDLD",
    "POSTBGIDL", "PLSTBGIDL", "PWSTBGIDL", "PLWSTBGIDL", "POSTBGIDLD", "PLSTBGIDLD", "PWSTBGIDLD", "PLWSTBGIDLD", "POCOX", "PLCOX", "PWCOX", "PLWCOX", "PODELVTAC", "PLDELVTAC", "PWDELVTAC", "PLWDELVTAC",
    "POFACNEFFAC", "PLFACNEFFAC", "PWFACNEFFAC", "PLWFACNEFFAC", "POTHESATAC", "PLTHESATAC", "PWTHESATAC", "PLWTHESATAC", "POAXAC", "PLAXAC", "PWAXAC", "PLWAXAC", "POALPAC", "PLALPAC", "PWALPAC", "PLWALPAC",
    "POALP1AC", "PLALP1AC", "PWALP1AC", "PLWALP1AC", "POCGOV", "PLCGOV", "PWCGOV", "PLWCGOV", "POCGOVD", "PLCGOVD", "PWCGOVD", "PLWCGOVD", "POCGBOV", "PLCGBOV", "PWCGBOV", "PLWCGBOV",
    "POCINR", "PLCINR", "PWCINR", "PLWCINR", "POCINRD", "PLCINRD", "PWCINRD", "PLWCINRD", "POCFR", "PLCFR", "PWCFR", "PLWCFR", "POCFRD", "PLCFRD", "PWCFRD", "PLWCFRD",
    "POFNTEXC", "PLFNTEXC", "PWFNTEXC", "PLWFNTEXC", "PONFA", "PLNFA", "PWNFA", "PLWNFA", "PONFB", "PLNFB", "PWNFB", "PLWNFB", "PONFC", "PLNFC", "PWNFC", "PLWNFC",
    "POVFBEDGE", "PLVFBEDGE", "PWVFBEDGE", "PLWVFBEDGE", "POSTVFBEDGE", "PLSTVFBEDGE", "PWSTVFBEDGE", "PLWSTVFBEDGE", "PODPHIBEDGE", "PLDPHIBEDGE", "PWDPHIBEDGE", "PLWDPHIBEDGE", "PONEFFEDGE", "PLNEFFEDGE", "PWNEFFEDGE", "PLWNEFFEDGE",
    "POCTEDGE", "PLCTEDGE", "PWCTEDGE", "PLWCTEDGE", "POBETNEDGE", "PLBETNEDGE", "PWBETNEDGE", "PLWBETNEDGE", "POSTBETEDGE", "PLSTBETEDGE", "PWSTBETEDGE", "PLWSTBETEDGE", "POPSCEEDGE", "PLPSCEEDGE", "PWPSCEEDGE", "PLWPSCEEDGE",
    "POPSCEBEDGE", "PLPSCEBEDGE", "PWPSCEBEDGE", "PLWPSCEBEDGE", "POPSCEDEDGE", "PLPSCEDEDGE", "PWPSCEDEDGE", "PLWPSCEDEDGE", "POCFEDGE", "PLCFEDGE", "PWCFEDGE", "PLWCFEDGE", "POCFBEDGE", "PLCFBEDGE", "PWCFBEDGE", "PLWCFBEDGE",
    "POCFDEDGE", "PLCFDEDGE", "PWCFDEDGE", "PLWCFDEDGE", "PONFAEDGE", "PLNFAEDGE", "PWNFAEDGE", "PLWNFAEDGE", "PONFBEDGE", "PLNFBEDGE", "PWNFBEDGE", "PLWNFBEDGE", "PONFCEDGE", "PLNFCEDGE", "PWNFCEDGE", "PLWNFCEDGE",
    "SAREF", "SBREF", "WLOD", "KUO", "KVSAT", "KVSATAC", "TKUO", "LKUO", "WKUO", "PKUO", "LLODKUO", "WLODKUO", "KVTHO", "LKVTHO", "WKVTHO", "PKVTHO",
    "LLODVTH", "WLODVTH", "STETAO", "LODETAO", "SCREF", "WEB", "WEC", "KVTHOWEO", "KVTHOWEL", "KVTHOWEW", "KVTHOWELW", "KUOWEO", "KUOWEL", "KUOWEW", "KUOWELW", "IMAX",
    "TRJ", "FREV", "CJORBOT", "CJORSTI", "CJORGAT", "VBIRBOT", "VBIRSTI", "VBIRGAT", "PBOT", "PSTI", "PGAT", "PHIGBOT", "PHIGSTI", "PHIGGAT", "IDSATRBOT", "IDSATRSTI",
    "IDSATRGAT", "CSRHBOT", "CSRHSTI", "CSRHGAT", "XJUNSTI", "XJUNGAT", "CTATBOT", "CTATSTI", "CTATGAT", "MEFFTATBOT", "MEFFTATSTI", "MEFFTATGAT", "CBBTBOT", "CBBTSTI", "CBBTGAT", "FBBTRBOT",
    "FBBTRSTI", "FBBTRGAT", "STFBBTBOT", "STFBBTSTI", "STFBBTGAT", "VBRBOT", "VBRSTI", "VBRGAT", "PBRBOT", "PBRSTI", "PBRGAT", "FCJORGAT2", "FVBIRGAT2", "FPGAT2", "FPHIGGAT2", "VTRGAT",
    "ANUGAT", "ADVBRGAT", "BDVBRGAT", "ADBBTGAT", "BDBBTGAT", "CJORBOTD", "CJORSTID", "CJORGATD", "VBIRBOTD", "VBIRSTID", "VBIRGATD", "PBOTD", "PSTID", "PGATD", "PHIGBOTD", "PHIGSTID",
    "PHIGGATD", "IDSATRBOTD", "IDSATRSTID", "IDSATRGATD", "CSRHBOTD", "CSRHSTID", "CSRHGATD", "XJUNSTID", "XJUNGATD", "CTATBOTD", "CTATSTID", "CTATGATD", "MEFFTATBOTD", "MEFFTATSTID", "MEFFTATGATD", "CBBTBOTD",
    "CBBTSTID", "CBBTGATD", "FBBTRBOTD", "FBBTRSTID", "FBBTRGATD", "STFBBTBOTD", "STFBBTSTID", "STFBBTGATD", "VBRBOTD", "VBRSTID", "VBRGATD", "PBRBOTD", "PBRSTID", "PBRGATD", "FCJORGAT2D", "FVBIRGAT2D",
    "FPGAT2D", "FPHIGGAT2D", "VTRGATD", "ANUGATD", "ADVBRGATD", "BDVBRGATD", "ADBBTGATD", "BDBBTGATD", "SWJUNEXP", "VJUNREF", "FJUNQ", "VJUNREFD", "FJUNQD",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 925] = [
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
    &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 925] = [
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, true, true, true, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 925] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -273.0, label: "-273.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 1e20, label: "1e20" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1e20, label: "1e20" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e23, label: "1e23" }), Some(ParameterBound { value: 1e23, label: "1e23" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None,
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: -0.5, label: "-0.5" }), None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: -0.5, label: "-0.5" }),
    Some(ParameterBound { value: -0.5, label: "-0.5" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1e-10, label: "1e-10" }),
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: -10.0, label: "-10.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -2.0, label: "-2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e20, label: "1e20" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e-12, label: "1e-12" }),
    Some(ParameterBound { value: -250.0, label: "-250.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }),
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }),
    Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 1e-12, label: "1e-12" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 1e-18, label: "1e-18" }),
    Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), Some(ParameterBound { value: 0.05, label: "0.05" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }),
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.1, label: "0.1" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }),
    Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: 0.01, label: "0.01" }), Some(ParameterBound { value: -100.0, label: "-100.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.2, label: "0.2" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 925] = [
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }), Some(ParameterBound { value: 1e27, label: "1e27" }), None,
    Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1e26, label: "1e26" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1e27, label: "1e27" }), Some(ParameterBound { value: 1e27, label: "1e27" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None,
    None, Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
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
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 10000000000.0, label: "10000000000.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), Some(ParameterBound { value: 0.95, label: "0.95" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 925] = [
    2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2,
    2, 0, 2, 2, 2, 0, 0, 2, 0, 0, 0, 2, 0, 2, 2, 0, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0,
    0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2,
    2, 2, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 0, 2, 0, 2, 2, 2, 2, 2, 2, 2,
    2, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2,
    2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 2,
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 0, 0, 2,
    0, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2,
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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0,
    2, 2, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0,
    0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 925] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[],
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
    &[], &[], &[], &[], &[],
];

fn parameter_computed_min_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn parameter_computed_max_bound(parameters: &Parameters, index: usize) -> Result<Option<ParameterBound>, String> {
    let params = parameters;
    let bound: Option<ParameterBound> = match index {
        _ => None,
    };
    if let Some(bound) = bound {
        validate_finite_parameter(bound.label, bound.value)?;
    }
    Ok(bound)
}

fn validate_parameter_computed_exclusions(
    parameters: &Parameters,
    index: usize,
    value: f64,
) -> Result<(), String> {
    let params = parameters;
    match index {
        _ => {}
    }
    Ok(())
}

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
    pub nodes: [usize; 12],
    pub branches: [usize; 7],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 925]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 11]>,
    pub(crate) ddt_state_previous: Box<[f64; 11]>,
    pub(crate) ddt_state_older: Box<[f64; 11]>,
    pub(crate) ddt_state_initialized: Box<[bool; 11]>,
    pub(crate) ddt_derivative_current: Box<[f64; 11]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 11]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<2913, 12, 7>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<2913, 12, 7>>>,
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
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 12;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["noi", "gp", "si", "di", "bp", "bi", "bs", "bd"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 925;
    pub const VARIABLE_COUNT: usize = 2913;
    pub const DDT_STATE_COUNT: usize = 11;
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
            scratch: None,
            reactive_scratch: None,
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
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'PSP104VA'", name));
        };
        validate_parameter_scalar_metadata(index, value)?;
        self.write_parameter_slot(index, value);
        self.finish_set_parameter(index);
        Ok(())
    }

    /// Validate the complete parameter vector after applying all instance overrides.
    pub fn validate_parameters(&self) -> Result<(), String> {
        for index in 0..Self::PARAMETER_COUNT {
            let value = read_parameter_slot(self.params.as_ref(), index);
            validate_parameter_metadata(self.params.as_ref(), index, value)?;
        }
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
}
