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
    pub p760: f64,
    pub p761: f64,
    pub p762: f64,
    pub p763: f64,
    pub p764: f64,
    pub p765: f64,
    pub p766: f64,
    pub p767: f64,
    pub p768: f64,
    pub p769: f64,
    pub p770: f64,
    pub p771: f64,
    pub p772: f64,
    pub p773: f64,
    pub p774: f64,
    pub p775: f64,
    pub p776: f64,
    pub p777: f64,
    pub p778: f64,
    pub p779: f64,
    pub p780: f64,
    pub p781: f64,
    pub p782: f64,
    pub p783: f64,
    pub p784: f64,
    pub p785: f64,
    pub p786: f64,
    pub p787: f64,
    pub p788: f64,
    pub p789: f64,
    pub p790: f64,
    pub p791: f64,
    pub p792: f64,
    pub p793: f64,
    pub p794: f64,
    pub p795: f64,
    pub p796: f64,
    pub p797: f64,
    pub p798: f64,
    pub p799: f64,
    pub p800: f64,
    pub p801: f64,
    pub p802: f64,
    pub p803: f64,
    pub p804: f64,
    pub p805: f64,
    pub p806: f64,
    pub p807: f64,
    pub p808: f64,
    pub p809: f64,
    pub p810: f64,
    pub p811: f64,
    pub p812: f64,
    pub p813: f64,
    pub p814: f64,
    pub p815: f64,
    pub p816: f64,
    pub p817: f64,
    pub p818: f64,
    pub p819: f64,
    pub p820: f64,
    pub p821: f64,
    pub p822: f64,
    pub p823: f64,
    pub p824: f64,
    pub p825: f64,
    pub p826: f64,
    pub p827: f64,
    pub p828: f64,
    pub p829: f64,
    pub p830: f64,
    pub p831: f64,
    pub p832: f64,
    pub p833: f64,
    pub p834: f64,
    pub p835: f64,
    pub p836: f64,
    pub p837: f64,
    pub p838: f64,
    pub p839: f64,
    pub p840: f64,
    pub p841: f64,
    pub p842: f64,
    pub p843: f64,
    pub p844: f64,
    pub p845: f64,
    pub p846: f64,
    pub p847: f64,
    pub p848: f64,
    pub p849: f64,
    pub p850: f64,
    pub p851: f64,
    pub p852: f64,
    pub p853: f64,
    pub p854: f64,
    pub p855: f64,
    pub p856: f64,
    pub p857: f64,
    pub p858: f64,
    pub p859: f64,
    pub p860: f64,
    pub p861: f64,
    pub p862: f64,
    pub p863: f64,
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
            params.p0 = 2e-6;
            params.p1 = 5e-6;
            params.p2 = 1.0;
            params.p3 = 1.0;
            params.p4 = 1.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 1.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 3e17;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 3.0;
            params.p18 = 0.0;
            params.p19 = 1.0;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 1.0;
            params.p23 = 0.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 0.0;
            params.p32 = 0.0;
            params.p33 = 1.0;
            params.p34 = 0.0;
            params.p35 = 1.0;
            params.p36 = if (params.p34 != 0.0) { params.p35 } else { 0.0 };
            validate_parameter("COOVLPS", params.p36, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p37 = 1.0;
            params.p38 = 1.0;
            params.p39 = 0.0;
            params.p40 = 1.0;
            params.p41 = 1.0;
            params.p42 = 0.0;
            params.p43 = 2.0;
            params.p44 = 1.0;
            params.p45 = 0.0;
            params.p46 = 0.0;
            params.p47 = 0.0;
            params.p48 = 0.0;
            params.p49 = 0.0;
            params.p50 = 0.0;
            params.p51 = 0.0;
            params.p52 = 0.0;
            params.p53 = 0.0;
            params.p54 = 0.0;
            params.p55 = 0.0;
            params.p56 = 50.0;
            params.p57 = 50.0;
            params.p58 = 50.0;
            params.p59 = 1.0;
            params.p60 = 0.0;
            params.p61 = 0.0;
            params.p62 = 1.0;
            params.p63 = 1e-6;
            params.p64 = 1e-6;
            params.p65 = if (params.p34 != 0.0) { params.p63 } else { 3e-8 };
            validate_parameter("LOVER", params.p65, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p66 = if (params.p34 != 0.0) { params.p63 } else { params.p65 };
            validate_parameter("LOVERS", params.p66, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p67 = 1e-6;
            params.p68 = 1e-6;
            params.p69 = if (params.p34 != 0.0) { params.p67 } else { 0.0 };
            validate_parameter("LDRIFT1S", params.p69, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p70 = if (params.p34 != 0.0) { params.p68 } else { 1e-6 };
            validate_parameter("LDRIFT2S", params.p70, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p71 = (params.p69 + params.p70);
            validate_parameter("LDRIFTS", params.p71, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = if (params.p34 != 0.0) { params.p74 } else { 0.0 };
            validate_parameter("RS", params.p75, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p76 = 5e17;
            params.p77 = 0.3;
            params.p78 = 1.0;
            params.p79 = 0.0;
            params.p80 = 0.1;
            params.p81 = 1.0;
            params.p82 = 0.07;
            params.p83 = 0.005;
            params.p84 = 0.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 1.0;
            params.p88 = 2.51;
            params.p89 = 10000000.0;
            params.p90 = 0.0;
            params.p91 = 0.0;
            params.p92 = 9.025e-5;
            params.p93 = 1e-7;
            params.p94 = 1.1785;
            params.p95 = 7e-9;
            params.p96 = params.p95;
            validate_finite_parameter("TOXB", params.p96).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p97 = 0.0;
            params.p98 = 0.0;
            params.p99 = 1.0;
            params.p100 = 1.0;
            params.p101 = 0.0;
            params.p102 = 0.0;
            params.p103 = 1.0;
            params.p104 = 0.0;
            params.p105 = 0.0;
            params.p106 = 0.0;
            params.p107 = 10.0;
            params.p108 = if (((params.p88 * 10.0) % 10.0) < 3.0) { 0.0 } else { 10.0 };
            validate_parameter("DDLTSLP", params.p108, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p109 = if (((params.p88 * 10.0) % 10.0) < 3.0) { 10.0 } else { 0.0 };
            validate_finite_parameter("DDLTICT", params.p109).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p110 = -0.5;
            params.p111 = 3e16;
            params.p112 = if (params.p34 != 0.0) { params.p111 } else { 1e17 };
            validate_parameter("NOVERS", params.p112, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p113 = 5.0;
            params.p114 = 0.0;
            params.p115 = params.p114;
            validate_finite_parameter("XWDC", params.p115).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p116 = 0.0;
            params.p117 = 0.0;
            params.p118 = 1e-6;
            params.p119 = 1e-6;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = 1.0;
            params.p126 = 0.0;
            params.p127 = 1.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = params.p130;
            validate_parameter("RSHS", params.p131, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p132 = 0.0;
            params.p133 = 0.0;
            params.p134 = 0.0;
            params.p135 = 2.0;
            params.p136 = if (params.p42 != 0.0) { (-0.2) } else { (-1.0) };
            validate_finite_parameter("VFBC", params.p136).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p137 = 1.1;
            params.p138 = if (params.p42 != 0.0) { 5e16 } else { 3e17 };
            validate_parameter("NSUBC", params.p138, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p139 = 1e-8;
            params.p140 = if (params.p42 != 0.0) { 0.0 } else { 1.5e-8 };
            validate_parameter("LP", params.p140, Some((0.0, "0.0")), false, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p141 = if (params.p42 != 0.0) { 1e17 } else { 1e18 };
            validate_parameter("NSUBP", params.p141, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p142 = 0.0;
            params.p143 = 1.0;
            params.p144 = 0.0;
            params.p145 = 0.0;
            params.p146 = 0.0;
            params.p147 = 0.0;
            params.p148 = 0.0;
            params.p149 = 0.0;
            params.p150 = 0.0;
            params.p151 = 0.0;
            params.p152 = 1.0;
            params.p153 = 0.0;
            params.p154 = 1.0;
            params.p155 = 0.0;
            params.p156 = 1.0;
            params.p157 = 0.5;
            params.p158 = 1000.0;
            params.p159 = 100.0;
            params.p160 = 0.3;
            params.p161 = if (params.p87 > 0.0) { 20000.0 } else { 9000.0 };
            validate_parameter("MUEPH1", params.p161, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p162 = 0.0;
            params.p163 = 1.0;
            params.p164 = 0.0;
            params.p165 = 1.0;
            params.p166 = 0.0;
            params.p167 = 0.0;
            params.p168 = 1.0;
            params.p169 = 0.0;
            params.p170 = 0.0;
            params.p171 = 2.0;
            params.p172 = if (params.p42 != 0.0) { 5000000000000000.0 } else { 600000000000000.0 };
            validate_parameter("MUESR1", params.p172, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p173 = 0.0;
            params.p174 = 0.0;
            params.p175 = 1.0;
            params.p176 = 1.0;
            params.p177 = 1.5;
            params.p178 = if (params.p87 > 0.0) { 2.0 } else { 1.0 };
            validate_parameter("BB", params.p178, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p179 = 10.0;
            params.p180 = 25.0;
            params.p181 = 0.8;
            params.p182 = 0.5;
            params.p183 = 0.0;
            params.p184 = 1.0;
            params.p185 = 0.8;
            params.p186 = 3e-8;
            params.p187 = params.p179;
            validate_finite_parameter("SUB1SNP", params.p187).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p188 = (0.6 * params.p180);
            validate_finite_parameter("SUB2SNP", params.p188).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p189 = params.p185;
            validate_finite_parameter("SVDSSNP", params.p189).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p190 = 0.0025;
            params.p191 = 1.0;
            params.p192 = 2e-6;
            params.p193 = 0.0;
            params.p194 = 50.0;
            params.p195 = 0.00017;
            params.p196 = 0.0;
            params.p197 = 0.012;
            params.p198 = 0.0;
            params.p199 = 1.0;
            params.p200 = 0.0;
            params.p201 = 1.0;
            params.p202 = 0.0;
            params.p203 = 1.0;
            params.p204 = 5e17;
            params.p205 = 0.0;
            params.p206 = 0.0;
            params.p207 = 1.0;
            params.p208 = 0.0;
            params.p209 = 1.0;
            params.p210 = 0.0;
            params.p211 = 0.0;
            params.p212 = 0.0;
            params.p213 = 0.0;
            params.p214 = 0.0;
            params.p215 = 0.0;
            params.p216 = 1.0;
            params.p217 = 0.0;
            params.p218 = 0.0;
            params.p219 = 1.0;
            params.p220 = 1e-50;
            params.p221 = 0.0;
            params.p222 = 0.0;
            params.p223 = 0.0;
            params.p224 = 0.9;
            params.p225 = 2e-7;
            params.p226 = 0.05;
            params.p227 = 2.0;
            params.p228 = 1.0;
            params.p229 = 1.0;
            params.p230 = 0.0;
            params.p231 = 0.3;
            params.p232 = 0.0;
            params.p233 = 0.0;
            params.p234 = 1.0;
            params.p235 = 0.0;
            params.p236 = 2.0;
            params.p237 = 0.0;
            params.p238 = 0.0;
            params.p239 = 0.0;
            params.p240 = 2.0;
            params.p241 = 30000000.0;
            params.p242 = 0.9;
            params.p243 = 0.0;
            params.p244 = 0.2;
            params.p245 = 50.0;
            params.p246 = 10000000.0;
            params.p247 = 0.06;
            params.p248 = 4.0;
            params.p249 = 7500.0;
            params.p250 = 0.25;
            params.p251 = 1e-6;
            params.p252 = 0.5;
            params.p253 = 1e-15;
            params.p254 = 1000.0;
            params.p255 = -1000.0;
            params.p256 = 5e-16;
            params.p257 = 1.0;
            params.p258 = 0.0;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 0.0;
            params.p262 = 0.01;
            params.p263 = 0.005;
            params.p264 = 10000000000.0;
            params.p265 = 1e-19;
            params.p266 = 0.0;
            params.p267 = 3.9;
            params.p268 = 0.0;
            params.p269 = if (params.p34 != 0.0) { params.p268 } else { 0.0 };
            validate_parameter("CGSO", params.p269, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p270 = 1e-10;
            params.p271 = 0.7;
            params.p272 = 8e-7;
            params.p273 = 8e-5;
            params.p274 = 27.0;
            params.p275 = 2.1e-7;
            params.p276 = 0.6;
            params.p277 = 1e-12;
            params.p278 = 0.0;
            params.p279 = 0.0;
            params.p280 = -1.0;
            params.p281 = 0.0;
            params.p282 = -0.3;
            params.p283 = 0.0;
            params.p284 = 3.5;
            params.p285 = 0.0;
            params.p286 = 1.0;
            params.p287 = 0.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = 0.0;
            params.p291 = 1.0;
            params.p292 = 100.0;
            params.p293 = 1e-7;
            params.p294 = 1e-6;
            params.p295 = params.p114;
            validate_finite_parameter("XWDLD", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p296 = 0.0;
            params.p297 = 1.0;
            params.p298 = 0.0;
            params.p299 = 0.0;
            params.p300 = 0.0;
            params.p301 = 0.0;
            params.p302 = 1.0;
            params.p303 = 0.0;
            params.p304 = 1.0;
            params.p305 = 0.0;
            params.p306 = 1.0;
            params.p307 = 0.0;
            params.p308 = 1.0;
            params.p309 = 0.0;
            params.p310 = 1.0;
            params.p311 = 0.0;
            params.p312 = 0.0;
            params.p313 = 0.0;
            params.p314 = 0.0;
            params.p315 = 0.0;
            params.p316 = 1.0;
            params.p317 = 0.0;
            params.p318 = 1.0;
            params.p319 = 0.0;
            params.p320 = 0.0;
            params.p321 = 1.0;
            params.p322 = 0.0;
            params.p323 = 1.0;
            params.p324 = 0.0;
            params.p325 = 0.0;
            params.p326 = -10.5;
            params.p327 = 0.0;
            params.p328 = 0.0;
            params.p329 = 0.0;
            params.p330 = 0.0;
            params.p331 = 0.0;
            params.p332 = 1.0;
            params.p333 = 0.3;
            params.p334 = 1e-6;
            params.p335 = 0.7;
            params.p336 = 1000000000000000.0;
            params.p337 = 0.1;
            params.p338 = 0.8;
            params.p339 = 0.4;
            params.p340 = if (params.p42 < 3.0) { 1e17 } else { 4e16 };
            validate_parameter("NDEPM", params.p340, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p341 = 0.0;
            params.p342 = 1.0;
            params.p343 = if (params.p42 < 3.0) { 2.0000000000000002e-7 } else { 3.0000000000000004e-7 };
            validate_parameter("TNDEP", params.p343, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p344 = (params.p343 * 1e-6);
            validate_finite_parameter("TNDEPMIN", params.p344).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p345 = 0.0;
            params.p346 = if (params.p42 < 3.0) { 1000.0 } else { 100000000.0 };
            validate_finite_parameter("DEPMUE0", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p347 = 0.0;
            params.p348 = 1.0;
            params.p349 = if (params.p42 < 3.0) { 0.0 } else { 100.0 };
            validate_finite_parameter("DEPMUE1", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p350 = 0.0;
            params.p351 = 1.0;
            params.p352 = 1000.0;
            params.p353 = 0.0;
            params.p354 = 100.0;
            params.p355 = 0.0;
            params.p356 = 0.0;
            params.p357 = 1.0;
            params.p358 = 0.0;
            params.p359 = 1.0;
            params.p360 = if (params.p42 < 3.0) { 0.5 } else { 0.1 };
            validate_finite_parameter("DEPLEAK", params.p360).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p361 = 0.0;
            params.p362 = 1.0;
            params.p363 = 0.0;
            params.p364 = 0.0;
            params.p365 = 0.0;
            params.p366 = 0.0;
            params.p367 = if (params.p42 < 3.0) { 30000000.0 } else { 10000000.0 };
            validate_parameter("DEPVMAX", params.p367, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p368 = 0.0;
            params.p369 = 1.0;
            params.p370 = 2.0;
            params.p371 = 0.5;
            params.p372 = 0.0;
            params.p373 = 1.0;
            params.p374 = 0.0;
            params.p375 = 1.0;
            params.p376 = if (params.p42 < 3.0) { 0.3 } else { 0.0 };
            validate_finite_parameter("DEPMUEPH0", params.p376).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p377 = if (params.p42 < 3.0) { 5000.0 } else { 400.0 };
            validate_parameter("DEPMUEPH1", params.p377, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p378 = if (params.p42 < 3.0) { 1.0 } else { 2.0 };
            validate_parameter("DEPBB", params.p378, Some((0.01, "0.01")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p379 = 0.0;
            params.p380 = 1.5;
            params.p381 = 0.0;
            params.p382 = 0.0;
            params.p383 = if (params.p42 < 3.0) { 3.0 } else { 1.0 };
            validate_parameter("DEPDDLT", params.p383, Some((0.1, "0.1")), false, Some((20.0, "20.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p384 = 100.0;
            params.p385 = 10.0;
            params.p386 = 0.0;
            params.p387 = 1.0;
            params.p388 = 0.0;
            params.p389 = 1.0;
            params.p390 = 0.0;
            params.p391 = 0.0;
            params.p392 = params.p136;
            validate_finite_parameter("DEPVFBC", params.p392).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p393 = 0.1;
            params.p394 = 2.0;
            params.p395 = params.p394;
            validate_parameter("DEPSUBSL0", params.p395, Some((1e-8, "1e-8")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p396 = 0.0;
            params.p397 = 0.0;
            params.p398 = 1.0;
            params.p399 = 1.0;
            params.p400 = 0.0;
            params.p401 = 0.0;
            params.p402 = 0.0;
            params.p403 = 0.01;
            params.p404 = 0.01;
            params.p405 = 0.05;
            params.p406 = 0.2;
            params.p407 = if (params.p42 < 3.0) { 0.0 } else { 0.2 };
            validate_parameter("DEPVGPSL", params.p407, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p408 = 0.5;
            params.p409 = 1000.0;
            params.p410 = 0.0;
            params.p411 = 0.0;
            params.p412 = 30000000.0;
            params.p413 = params.p409;
            validate_parameter("RDRMUES", params.p413, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p414 = params.p412;
            validate_parameter("RDRVMAXS", params.p414, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p415 = 0.0;
            params.p416 = params.p415;
            validate_finite_parameter("RDRMUETMPS", params.p416).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p417 = 0.0;
            params.p418 = params.p417;
            validate_finite_parameter("RDRVTMPS", params.p418).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p419 = 1e-6;
            params.p420 = 0.0;
            params.p421 = 1e-8;
            params.p422 = 0.0;
            params.p423 = 0.0;
            params.p424 = 0.0;
            params.p425 = 1.0;
            params.p426 = 0.0;
            params.p427 = 1.0;
            params.p428 = 0.0;
            params.p429 = 1.0;
            params.p430 = 100000.0;
            params.p431 = 0.0;
            params.p432 = 0.0;
            params.p433 = 500.0;
            params.p434 = ((-100.0) * params.p87);
            validate_finite_parameter("VGSMIN", params.p434).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p435 = 0.0;
            params.p436 = 1.0;
            params.p437 = params.p436;
            validate_parameter("RDRBBS", params.p437, Some((0.1, "0.1")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p438 = 0.0;
            params.p439 = params.p438;
            validate_finite_parameter("RDRBBTMPS", params.p439).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p440 = 1.0;
            params.p441 = 1.0;
            params.p442 = 1.0;
            params.p443 = 0.0;
            params.p444 = 0.0001;
            params.p445 = 0.0;
            params.p446 = 0.0;
            params.p447 = 100.0;
            params.p448 = 0.0;
            params.p449 = 1.0;
            params.p450 = 0.0;
            params.p451 = 0.0;
            params.p452 = 3e-8;
            params.p453 = 1e20;
            params.p454 = 0.0;
            params.p455 = 0.0;
            params.p456 = 0.0;
            params.p457 = 0.0;
            params.p458 = 5e-7;
            params.p459 = 0.0;
            params.p460 = 0.0;
            params.p461 = 1.0;
            params.p462 = 1.0;
            params.p463 = 1.0;
            params.p464 = 2.0;
            params.p465 = 0.0005;
            params.p466 = 5e-10;
            params.p467 = 5e-10;
            params.p468 = 0.5;
            params.p469 = 0.33;
            params.p470 = 0.33;
            params.p471 = 1.0;
            params.p472 = 1.0;
            params.p473 = 1.0;
            params.p474 = 0.0;
            params.p475 = 0.0;
            params.p476 = 0.0;
            params.p477 = 0.0;
            params.p478 = 0.0;
            params.p479 = 0.0;
            params.p480 = 0.0006;
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
            params.p493 = params.p458;
            validate_finite_parameter("JS0D", params.p493).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p494 = params.p459;
            validate_finite_parameter("JS0SWD", params.p494).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p495 = params.p460;
            validate_finite_parameter("JS0SWGD", params.p495).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p496 = params.p461;
            validate_parameter("NJD", params.p496, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p497 = params.p462;
            validate_parameter("NJSWD", params.p497, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p498 = params.p463;
            validate_parameter("NJSWGD", params.p498, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p499 = params.p464;
            validate_finite_parameter("XTID", params.p499).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p500 = params.p465;
            validate_finite_parameter("CJD", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p501 = params.p466;
            validate_finite_parameter("CJSWD", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p502 = params.p467;
            validate_finite_parameter("CJSWGD", params.p502).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p503 = params.p468;
            validate_parameter("MJD", params.p503, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p504 = params.p469;
            validate_parameter("MJSWD", params.p504, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p505 = params.p470;
            validate_parameter("MJSWGD", params.p505, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p506 = params.p471;
            validate_parameter("PBD", params.p506, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p507 = params.p472;
            validate_parameter("PBSWD", params.p507, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p508 = params.p473;
            validate_parameter("PBSWGD", params.p508, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p509 = params.p474;
            validate_finite_parameter("XTI2D", params.p509).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p510 = params.p475;
            validate_finite_parameter("CISBD", params.p510).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p511 = params.p476;
            validate_finite_parameter("CVBD", params.p511).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p512 = params.p477;
            validate_finite_parameter("CTEMPD", params.p512).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p513 = params.p478;
            validate_finite_parameter("CISBKD", params.p513).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p514 = params.p479;
            validate_finite_parameter("DIVXD", params.p514).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p515 = params.p480;
            validate_finite_parameter("VDIFFJD", params.p515).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p516 = params.p493;
            validate_finite_parameter("JS0S", params.p516).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p517 = params.p494;
            validate_finite_parameter("JS0SWS", params.p517).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p518 = params.p495;
            validate_finite_parameter("JS0SWGS", params.p518).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p519 = params.p496;
            validate_parameter("NJS", params.p519, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p520 = params.p497;
            validate_parameter("NJSWS", params.p520, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p521 = params.p498;
            validate_parameter("NJSWGS", params.p521, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p522 = params.p499;
            validate_finite_parameter("XTIS", params.p522).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p523 = params.p500;
            validate_finite_parameter("CJS", params.p523).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p524 = params.p501;
            validate_finite_parameter("CJSWS", params.p524).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p525 = params.p502;
            validate_finite_parameter("CJSWGS", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p526 = params.p503;
            validate_parameter("MJS", params.p526, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p527 = params.p504;
            validate_parameter("MJSWS", params.p527, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p528 = params.p505;
            validate_parameter("MJSWGS", params.p528, None, true, Some((1.0, "1.0")), true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p529 = params.p506;
            validate_parameter("PBS", params.p529, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p530 = params.p507;
            validate_parameter("PBSWS", params.p530, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p531 = params.p508;
            validate_parameter("PBSWGS", params.p531, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p532 = params.p509;
            validate_finite_parameter("XTI2S", params.p532).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p533 = params.p510;
            validate_finite_parameter("CISBS", params.p533).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p534 = params.p511;
            validate_finite_parameter("CVBS", params.p534).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p535 = params.p512;
            validate_finite_parameter("CTEMPS", params.p535).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p536 = params.p513;
            validate_finite_parameter("CISBKS", params.p536).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p537 = params.p514;
            validate_finite_parameter("DIVXS", params.p537).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p538 = params.p515;
            validate_finite_parameter("VDIFFJS", params.p538).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p539 = 0.0;
            params.p540 = 1e16;
            params.p541 = 1.0;
            params.p542 = 10.0;
            params.p543 = 5e-9;
            params.p544 = 2e-7;
            params.p545 = 5e-6;
            params.p546 = 0.0;
            params.p547 = 0.0;
            params.p548 = 0.0;
            params.p549 = 0.0;
            params.p550 = 1.0;
            params.p551 = 0.0;
            params.p552 = 1.0;
            params.p553 = 1.0;
            params.p554 = 1.0;
            params.p555 = 0.0;
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
            params.p587 = params.p582;
            validate_finite_parameter("LSUB1SNP", params.p587).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p588 = params.p583;
            validate_finite_parameter("LSUB2SNP", params.p588).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p589 = params.p584;
            validate_finite_parameter("LSVDSSNP", params.p589).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p619 = 0.0;
            params.p620 = 0.0;
            params.p621 = 0.0;
            params.p622 = 0.0;
            params.p623 = 0.0;
            params.p624 = 0.0;
            params.p625 = 0.0;
            params.p626 = 0.0;
            params.p627 = 0.0;
            params.p628 = 0.0;
            params.p629 = 0.0;
            params.p630 = 0.0;
            params.p631 = 0.0;
            params.p632 = 0.0;
            params.p633 = 0.0;
            params.p634 = 0.0;
            params.p635 = 0.0;
            params.p636 = 0.0;
            params.p637 = 0.0;
            params.p638 = 0.0;
            params.p639 = 0.0;
            params.p640 = 0.0;
            params.p641 = 0.0;
            params.p642 = 0.0;
            params.p643 = 0.0;
            params.p644 = 0.0;
            params.p645 = 0.0;
            params.p646 = 0.0;
            params.p647 = 0.0;
            params.p648 = 0.0;
            params.p649 = 0.0;
            params.p650 = 0.0;
            params.p651 = 0.0;
            params.p652 = 0.0;
            params.p653 = 0.0;
            params.p654 = 0.0;
            params.p655 = 0.0;
            params.p656 = 0.0;
            params.p657 = 0.0;
            params.p658 = 0.0;
            params.p659 = 0.0;
            params.p660 = 0.0;
            params.p661 = 0.0;
            params.p662 = 0.0;
            params.p663 = 0.0;
            params.p664 = 0.0;
            params.p665 = 0.0;
            params.p666 = 0.0;
            params.p667 = 0.0;
            params.p668 = 0.0;
            params.p669 = 0.0;
            params.p670 = 0.0;
            params.p671 = 0.0;
            params.p672 = 0.0;
            params.p673 = 0.0;
            params.p674 = 0.0;
            params.p675 = params.p670;
            validate_finite_parameter("WSUB1SNP", params.p675).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p676 = params.p671;
            validate_finite_parameter("WSUB2SNP", params.p676).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p677 = params.p672;
            validate_finite_parameter("WSVDSSNP", params.p677).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p678 = 0.0;
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
            params.p760 = 0.0;
            params.p761 = 0.0;
            params.p762 = 0.0;
            params.p763 = params.p758;
            validate_finite_parameter("PSUB1SNP", params.p763).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p764 = params.p759;
            validate_finite_parameter("PSUB2SNP", params.p764).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p765 = params.p760;
            validate_finite_parameter("PSVDSSNP", params.p765).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p766 = 0.0;
            params.p767 = 0.0;
            params.p768 = 0.0;
            params.p769 = 0.0;
            params.p770 = 0.0;
            params.p771 = 0.0;
            params.p772 = 0.0;
            params.p773 = 0.0;
            params.p774 = 0.0;
            params.p775 = 0.0;
            params.p776 = 0.0;
            params.p777 = 0.0;
            params.p778 = 0.0;
            params.p779 = 0.0;
            params.p780 = 0.0;
            params.p781 = 0.0;
            params.p782 = 0.0;
            params.p783 = 0.0;
            params.p784 = 0.0;
            params.p785 = 0.0;
            params.p786 = 0.0;
            params.p787 = 0.0;
            params.p788 = 0.0;
            params.p789 = 0.0;
            params.p790 = 0.0;
            params.p791 = 0.0;
            params.p792 = 0.0;
            params.p793 = 0.0;
            params.p794 = 0.0;
            params.p795 = 0.0;
            params.p796 = 0.0;
            params.p797 = 0.0;
            params.p798 = 0.0;
            params.p799 = 0.0;
            params.p800 = 0.0;
            params.p801 = 0.0;
            params.p802 = 0.0;
            params.p803 = 0.0;
            params.p804 = 0.0;
            params.p805 = 0.0;
            params.p806 = 0.0;
            params.p807 = 0.0;
            params.p808 = 0.0;
            params.p809 = 0.0;
            params.p810 = 0.0;
            params.p811 = 0.0;
            params.p812 = 0.0;
            params.p813 = 0.0;
            params.p814 = 0.0;
            params.p815 = 0.0;
            params.p816 = 0.0;
            params.p817 = 0.0;
            params.p818 = 0.0;
            params.p819 = 0.0;
            params.p820 = 0.0;
            params.p821 = 0.0;
            params.p822 = 0.0;
            params.p823 = 0.0;
            params.p824 = params.p819;
            validate_finite_parameter("LJS0D", params.p824).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p825 = params.p820;
            validate_finite_parameter("LJS0SWD", params.p825).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p826 = params.p821;
            validate_finite_parameter("LNJD", params.p826).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p827 = params.p822;
            validate_finite_parameter("LCISBKD", params.p827).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p828 = params.p823;
            validate_finite_parameter("LVDIFFJD", params.p828).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p829 = params.p824;
            validate_finite_parameter("LJS0S", params.p829).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p830 = params.p825;
            validate_finite_parameter("LJS0SWS", params.p830).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p831 = params.p826;
            validate_finite_parameter("LNJS", params.p831).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p832 = params.p827;
            validate_finite_parameter("LCISBKS", params.p832).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p833 = params.p828;
            validate_finite_parameter("LVDIFFJS", params.p833).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p834 = 0.0;
            params.p835 = 0.0;
            params.p836 = 0.0;
            params.p837 = 0.0;
            params.p838 = 0.0;
            params.p839 = params.p834;
            validate_finite_parameter("WJS0D", params.p839).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p840 = params.p835;
            validate_finite_parameter("WJS0SWD", params.p840).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p841 = params.p836;
            validate_finite_parameter("WNJD", params.p841).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p842 = params.p837;
            validate_finite_parameter("WCISBKD", params.p842).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p843 = params.p838;
            validate_finite_parameter("WVDIFFJD", params.p843).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p844 = params.p839;
            validate_finite_parameter("WJS0S", params.p844).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p845 = params.p840;
            validate_finite_parameter("WJS0SWS", params.p845).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p846 = params.p841;
            validate_finite_parameter("WNJS", params.p846).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p847 = params.p842;
            validate_finite_parameter("WCISBKS", params.p847).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p848 = params.p843;
            validate_finite_parameter("WVDIFFJS", params.p848).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p849 = 0.0;
            params.p850 = 0.0;
            params.p851 = 0.0;
            params.p852 = 0.0;
            params.p853 = 0.0;
            params.p854 = params.p849;
            validate_finite_parameter("PJS0D", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p855 = params.p850;
            validate_finite_parameter("PJS0SWD", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p856 = params.p851;
            validate_finite_parameter("PNJD", params.p856).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p857 = params.p852;
            validate_finite_parameter("PCISBKD", params.p857).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p858 = params.p853;
            validate_finite_parameter("PVDIFFJD", params.p858).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p859 = params.p854;
            validate_finite_parameter("PJS0S", params.p859).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p860 = params.p855;
            validate_finite_parameter("PJS0SWS", params.p860).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p861 = params.p856;
            validate_finite_parameter("PNJS", params.p861).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p862 = params.p857;
            validate_finite_parameter("PCISBKS", params.p862).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p863 = params.p858;
            validate_finite_parameter("PVDIFFJS", params.p863).expect("generated Verilog-A parameter default must satisfy declared range");
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
    pub nodes: [usize; 18],
    pub branches: [usize; 12],
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
    pub(crate) scalar_v4: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v12: f64,
    pub(crate) scalar_v15: f64,
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
    pub(crate) scalar_v31: f64,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: f64,
    pub(crate) scalar_v34: f64,
    pub(crate) scalar_v36: f64,
    pub(crate) scalar_v37: f64,
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
    pub(crate) scalar_v56: f64,
    pub(crate) scalar_v57: f64,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v60: f64,
    pub(crate) scalar_v61: f64,
    pub(crate) scalar_v62: f64,
    pub(crate) scalar_v63: f64,
    pub(crate) scalar_v64: f64,
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
    pub(crate) scalar_v92: f64,
    pub(crate) scalar_v93: f64,
    pub(crate) scalar_v94: f64,
    pub(crate) scalar_v95: f64,
    pub(crate) scalar_v96: f64,
    pub(crate) scalar_v97: f64,
    pub(crate) scalar_v98: f64,
    pub(crate) scalar_v99: f64,
    pub(crate) scalar_v100: f64,
    pub(crate) scalar_v101: f64,
    pub(crate) scalar_v102: f64,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
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
    pub(crate) scalar_v118: f64,
    pub(crate) scalar_v119: f64,
    pub(crate) scalar_v120: f64,
    pub(crate) scalar_v121: f64,
    pub(crate) scalar_v122: f64,
    pub(crate) scalar_v123: f64,
    pub(crate) scalar_v124: f64,
    pub(crate) scalar_v125: f64,
    pub(crate) scalar_v126: f64,
    pub(crate) scalar_v127: f64,
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: f64,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: f64,
    pub(crate) scalar_v132: f64,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: bool,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: bool,
    pub(crate) scalar_v142: bool,
    pub(crate) scalar_v143: bool,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: bool,
    pub(crate) scalar_v146: bool,
    pub(crate) scalar_v147: bool,
    pub(crate) scalar_v148: bool,
    pub(crate) scalar_v149: bool,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: bool,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: bool,
    pub(crate) scalar_v159: bool,
    pub(crate) scalar_v160: bool,
    pub(crate) scalar_v161: bool,
    pub(crate) scalar_v162: bool,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: bool,
    pub(crate) scalar_v167: f64,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
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
    pub(crate) scalar_v181: bool,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: f64,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: f64,
    pub(crate) scalar_v195: f64,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: bool,
    pub(crate) scalar_v198: f64,
    pub(crate) scalar_v199: f64,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: f64,
    pub(crate) scalar_v202: f64,
    pub(crate) scalar_v203: f64,
    pub(crate) scalar_v204: f64,
    pub(crate) scalar_v205: f64,
    pub(crate) scalar_v206: f64,
    pub(crate) scalar_v207: f64,
    pub(crate) scalar_v208: f64,
    pub(crate) scalar_v209: f64,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v215: f64,
    pub(crate) scalar_v216: f64,
    pub(crate) scalar_v217: f64,
    pub(crate) scalar_v218: f64,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: f64,
    pub(crate) scalar_v221: f64,
    pub(crate) scalar_v222: f64,
    pub(crate) scalar_v223: f64,
    pub(crate) scalar_v224: bool,
    pub(crate) scalar_v225: f64,
    pub(crate) scalar_v226: f64,
    pub(crate) scalar_v227: bool,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v233: f64,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: bool,
    pub(crate) scalar_v241: bool,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: bool,
    pub(crate) scalar_v245: bool,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v248: bool,
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: bool,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: bool,
    pub(crate) scalar_v256: bool,
    pub(crate) scalar_v257: f64,
    pub(crate) scalar_v258: bool,
    pub(crate) scalar_v259: bool,
    pub(crate) scalar_v260: f64,
    pub(crate) scalar_v261: f64,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v265: bool,
    pub(crate) scalar_v266: bool,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v269: bool,
    pub(crate) scalar_v270: f64,
    pub(crate) scalar_v271: f64,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: bool,
    pub(crate) scalar_v275: bool,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: bool,
    pub(crate) scalar_v278: bool,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v280: f64,
    pub(crate) scalar_v281: f64,
    pub(crate) scalar_v282: bool,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: f64,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: f64,
    pub(crate) scalar_v289: f64,
    pub(crate) scalar_v290: f64,
    pub(crate) scalar_v291: f64,
    pub(crate) scalar_v292: f64,
    pub(crate) scalar_v293: f64,
    pub(crate) scalar_v294: f64,
    pub(crate) scalar_v295: f64,
    pub(crate) scalar_v296: f64,
    pub(crate) scalar_v297: f64,
    pub(crate) scalar_v298: f64,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: f64,
    pub(crate) scalar_v302: f64,
    pub(crate) scalar_v303: bool,
    pub(crate) scalar_v304: bool,
    pub(crate) scalar_v305: bool,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v315: f64,
    pub(crate) scalar_v318: bool,
    pub(crate) scalar_v333: bool,
    pub(crate) scalar_v334: bool,
    pub(crate) scalar_v335: bool,
    pub(crate) scalar_v336: bool,
    pub(crate) scalar_v345: bool,
    pub(crate) scalar_v355: f64,
    pub(crate) scalar_v356: bool,
    pub(crate) scalar_v357: f64,
    pub(crate) scalar_v358: bool,
    pub(crate) scalar_v359: bool,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: bool,
    pub(crate) scalar_v362: bool,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v364: bool,
    pub(crate) scalar_v372: bool,
    pub(crate) scalar_v374: bool,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v376: f64,
    pub(crate) scalar_v377: bool,
    pub(crate) scalar_v378: f64,
    pub(crate) scalar_v379: bool,
    pub(crate) scalar_v380: f64,
    pub(crate) scalar_v381: bool,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v383: bool,
    pub(crate) scalar_v384: f64,
    pub(crate) scalar_v390: bool,
    pub(crate) scalar_v391: f64,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
    pub(crate) scalar_v418: f64,
    pub(crate) scalar_v419: f64,
    pub(crate) scalar_v420: f64,
    pub(crate) scalar_v421: f64,
    pub(crate) scalar_v422: f64,
    pub(crate) scalar_v424: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: f64,
    pub(crate) scalar_v311: f64,
    pub(crate) scalar_v312: f64,
    pub(crate) scalar_v313: f64,
    pub(crate) scalar_v314: f64,
    pub(crate) scalar_v316: f64,
    pub(crate) scalar_v317: f64,
    pub(crate) scalar_v319: f64,
    pub(crate) scalar_v320: f64,
    pub(crate) scalar_v321: f64,
    pub(crate) scalar_v322: f64,
    pub(crate) scalar_v323: f64,
    pub(crate) scalar_v324: f64,
    pub(crate) scalar_v326: bool,
    pub(crate) scalar_v327: bool,
    pub(crate) scalar_v328: f64,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: f64,
    pub(crate) scalar_v340: f64,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: f64,
    pub(crate) scalar_v343: f64,
    pub(crate) scalar_v344: f64,
    pub(crate) scalar_v346: f64,
    pub(crate) scalar_v347: f64,
    pub(crate) scalar_v348: f64,
    pub(crate) scalar_v349: f64,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: f64,
    pub(crate) scalar_v352: bool,
    pub(crate) scalar_v353: bool,
    pub(crate) scalar_v354: f64,
    pub(crate) scalar_v365: bool,
    pub(crate) scalar_v366: bool,
    pub(crate) scalar_v367: f64,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: bool,
    pub(crate) scalar_v370: bool,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v423: f64,
    pub(crate) scalar_temperature_static_valid: bool,
    pub(crate) scalar_temperature_static_temperature: f64,
    pub(crate) scalar_temperature_static_thermal_voltage: f64,
    pub(crate) scratch: Option<Box<GenericScratch<3410, 18, 12>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<3410, 18, 12>>>,
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
            scalar_v4: self.scalar_v4,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v8: self.scalar_v8,
            scalar_v9: self.scalar_v9,
            scalar_v11: self.scalar_v11,
            scalar_v12: self.scalar_v12,
            scalar_v15: self.scalar_v15,
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
            scalar_v31: self.scalar_v31,
            scalar_v32: self.scalar_v32,
            scalar_v33: self.scalar_v33,
            scalar_v34: self.scalar_v34,
            scalar_v36: self.scalar_v36,
            scalar_v37: self.scalar_v37,
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
            scalar_v73: self.scalar_v73,
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
            scalar_v138: self.scalar_v138,
            scalar_v139: self.scalar_v139,
            scalar_v140: self.scalar_v140,
            scalar_v141: self.scalar_v141,
            scalar_v142: self.scalar_v142,
            scalar_v143: self.scalar_v143,
            scalar_v144: self.scalar_v144,
            scalar_v145: self.scalar_v145,
            scalar_v146: self.scalar_v146,
            scalar_v147: self.scalar_v147,
            scalar_v148: self.scalar_v148,
            scalar_v149: self.scalar_v149,
            scalar_v150: self.scalar_v150,
            scalar_v151: self.scalar_v151,
            scalar_v152: self.scalar_v152,
            scalar_v153: self.scalar_v153,
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
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
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
            scalar_v184: self.scalar_v184,
            scalar_v185: self.scalar_v185,
            scalar_v186: self.scalar_v186,
            scalar_v187: self.scalar_v187,
            scalar_v188: self.scalar_v188,
            scalar_v189: self.scalar_v189,
            scalar_v190: self.scalar_v190,
            scalar_v191: self.scalar_v191,
            scalar_v192: self.scalar_v192,
            scalar_v193: self.scalar_v193,
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v198: self.scalar_v198,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v204: self.scalar_v204,
            scalar_v205: self.scalar_v205,
            scalar_v206: self.scalar_v206,
            scalar_v207: self.scalar_v207,
            scalar_v208: self.scalar_v208,
            scalar_v209: self.scalar_v209,
            scalar_v210: self.scalar_v210,
            scalar_v212: self.scalar_v212,
            scalar_v214: self.scalar_v214,
            scalar_v215: self.scalar_v215,
            scalar_v216: self.scalar_v216,
            scalar_v217: self.scalar_v217,
            scalar_v218: self.scalar_v218,
            scalar_v219: self.scalar_v219,
            scalar_v220: self.scalar_v220,
            scalar_v221: self.scalar_v221,
            scalar_v222: self.scalar_v222,
            scalar_v223: self.scalar_v223,
            scalar_v224: self.scalar_v224,
            scalar_v225: self.scalar_v225,
            scalar_v226: self.scalar_v226,
            scalar_v227: self.scalar_v227,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v233: self.scalar_v233,
            scalar_v234: self.scalar_v234,
            scalar_v235: self.scalar_v235,
            scalar_v236: self.scalar_v236,
            scalar_v237: self.scalar_v237,
            scalar_v238: self.scalar_v238,
            scalar_v239: self.scalar_v239,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v247: self.scalar_v247,
            scalar_v248: self.scalar_v248,
            scalar_v249: self.scalar_v249,
            scalar_v250: self.scalar_v250,
            scalar_v251: self.scalar_v251,
            scalar_v252: self.scalar_v252,
            scalar_v253: self.scalar_v253,
            scalar_v254: self.scalar_v254,
            scalar_v255: self.scalar_v255,
            scalar_v256: self.scalar_v256,
            scalar_v257: self.scalar_v257,
            scalar_v258: self.scalar_v258,
            scalar_v259: self.scalar_v259,
            scalar_v260: self.scalar_v260,
            scalar_v261: self.scalar_v261,
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
            scalar_v265: self.scalar_v265,
            scalar_v266: self.scalar_v266,
            scalar_v267: self.scalar_v267,
            scalar_v268: self.scalar_v268,
            scalar_v269: self.scalar_v269,
            scalar_v270: self.scalar_v270,
            scalar_v271: self.scalar_v271,
            scalar_v272: self.scalar_v272,
            scalar_v273: self.scalar_v273,
            scalar_v274: self.scalar_v274,
            scalar_v275: self.scalar_v275,
            scalar_v276: self.scalar_v276,
            scalar_v277: self.scalar_v277,
            scalar_v278: self.scalar_v278,
            scalar_v279: self.scalar_v279,
            scalar_v280: self.scalar_v280,
            scalar_v281: self.scalar_v281,
            scalar_v282: self.scalar_v282,
            scalar_v283: self.scalar_v283,
            scalar_v284: self.scalar_v284,
            scalar_v285: self.scalar_v285,
            scalar_v286: self.scalar_v286,
            scalar_v287: self.scalar_v287,
            scalar_v288: self.scalar_v288,
            scalar_v289: self.scalar_v289,
            scalar_v290: self.scalar_v290,
            scalar_v291: self.scalar_v291,
            scalar_v292: self.scalar_v292,
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v295: self.scalar_v295,
            scalar_v296: self.scalar_v296,
            scalar_v297: self.scalar_v297,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v307: self.scalar_v307,
            scalar_v315: self.scalar_v315,
            scalar_v318: self.scalar_v318,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v345: self.scalar_v345,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v360: self.scalar_v360,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v364: self.scalar_v364,
            scalar_v372: self.scalar_v372,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v376: self.scalar_v376,
            scalar_v377: self.scalar_v377,
            scalar_v378: self.scalar_v378,
            scalar_v379: self.scalar_v379,
            scalar_v380: self.scalar_v380,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v383: self.scalar_v383,
            scalar_v384: self.scalar_v384,
            scalar_v390: self.scalar_v390,
            scalar_v391: self.scalar_v391,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
            scalar_v418: self.scalar_v418,
            scalar_v419: self.scalar_v419,
            scalar_v420: self.scalar_v420,
            scalar_v421: self.scalar_v421,
            scalar_v422: self.scalar_v422,
            scalar_v424: self.scalar_v424,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v314: self.scalar_v314,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v365: self.scalar_v365,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v371: self.scalar_v371,
            scalar_v373: self.scalar_v373,
            scalar_v423: self.scalar_v423,
            scalar_temperature_static_valid: self.scalar_temperature_static_valid,
            scalar_temperature_static_temperature: self.scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage: self.scalar_temperature_static_thermal_voltage,
            scratch: None,
            reactive_scratch: None,
        }
    }
}

impl Instance {
    pub const TERMINAL_COUNT: usize = 4;
    pub const INTERNAL_NODE_COUNT: usize = 14;
    pub const NODE_COUNT: usize = 18;
    pub const INTERNAL_NODE_NAMES: [&str; 14] = ["temp", "dp", "gp", "sp", "bp", "db", "sb", "qi", "qb", "qbd", "n", "charge_A", "charge_K", "depl_A"];

    pub const BRANCH_COUNT: usize = 12;
    pub const PARAMETER_COUNT: usize = 864;
    pub const VARIABLE_COUNT: usize = 3410;
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
            scalar_v4: 0.0,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v9: 0.0,
            scalar_v11: 0.0,
            scalar_v12: 0.0,
            scalar_v15: 0.0,
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
            scalar_v31: 0.0,
            scalar_v32: 0.0,
            scalar_v33: 0.0,
            scalar_v34: 0.0,
            scalar_v36: 0.0,
            scalar_v37: 0.0,
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
            scalar_v56: 0.0,
            scalar_v57: 0.0,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v60: 0.0,
            scalar_v61: 0.0,
            scalar_v62: 0.0,
            scalar_v63: 0.0,
            scalar_v64: 0.0,
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
            scalar_v92: 0.0,
            scalar_v93: 0.0,
            scalar_v94: 0.0,
            scalar_v95: 0.0,
            scalar_v96: 0.0,
            scalar_v97: 0.0,
            scalar_v98: 0.0,
            scalar_v99: 0.0,
            scalar_v100: 0.0,
            scalar_v101: 0.0,
            scalar_v102: 0.0,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
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
            scalar_v118: 0.0,
            scalar_v119: 0.0,
            scalar_v120: 0.0,
            scalar_v121: 0.0,
            scalar_v122: 0.0,
            scalar_v123: 0.0,
            scalar_v124: 0.0,
            scalar_v125: 0.0,
            scalar_v126: 0.0,
            scalar_v127: 0.0,
            scalar_v128: 0.0,
            scalar_v129: 0.0,
            scalar_v130: 0.0,
            scalar_v131: 0.0,
            scalar_v132: 0.0,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: false,
            scalar_v140: 0.0,
            scalar_v141: false,
            scalar_v142: false,
            scalar_v143: false,
            scalar_v144: 0.0,
            scalar_v145: false,
            scalar_v146: false,
            scalar_v147: false,
            scalar_v148: false,
            scalar_v149: false,
            scalar_v150: 0.0,
            scalar_v151: false,
            scalar_v152: 0.0,
            scalar_v153: false,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: false,
            scalar_v159: false,
            scalar_v160: false,
            scalar_v161: false,
            scalar_v162: false,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: false,
            scalar_v166: false,
            scalar_v167: 0.0,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
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
            scalar_v181: false,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: 0.0,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: 0.0,
            scalar_v195: 0.0,
            scalar_v196: 0.0,
            scalar_v197: false,
            scalar_v198: 0.0,
            scalar_v199: 0.0,
            scalar_v200: 0.0,
            scalar_v201: 0.0,
            scalar_v202: 0.0,
            scalar_v203: 0.0,
            scalar_v204: 0.0,
            scalar_v205: 0.0,
            scalar_v206: 0.0,
            scalar_v207: 0.0,
            scalar_v208: 0.0,
            scalar_v209: 0.0,
            scalar_v210: 0.0,
            scalar_v212: 0.0,
            scalar_v214: 0.0,
            scalar_v215: 0.0,
            scalar_v216: 0.0,
            scalar_v217: 0.0,
            scalar_v218: 0.0,
            scalar_v219: 0.0,
            scalar_v220: 0.0,
            scalar_v221: 0.0,
            scalar_v222: 0.0,
            scalar_v223: 0.0,
            scalar_v224: false,
            scalar_v225: 0.0,
            scalar_v226: 0.0,
            scalar_v227: false,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v233: 0.0,
            scalar_v234: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v237: 0.0,
            scalar_v238: 0.0,
            scalar_v239: 0.0,
            scalar_v240: false,
            scalar_v241: false,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v244: false,
            scalar_v245: false,
            scalar_v247: 0.0,
            scalar_v248: false,
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: false,
            scalar_v254: 0.0,
            scalar_v255: false,
            scalar_v256: false,
            scalar_v257: 0.0,
            scalar_v258: false,
            scalar_v259: false,
            scalar_v260: 0.0,
            scalar_v261: 0.0,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v264: 0.0,
            scalar_v265: false,
            scalar_v266: false,
            scalar_v267: 0.0,
            scalar_v268: false,
            scalar_v269: false,
            scalar_v270: 0.0,
            scalar_v271: 0.0,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v274: false,
            scalar_v275: false,
            scalar_v276: 0.0,
            scalar_v277: false,
            scalar_v278: false,
            scalar_v279: 0.0,
            scalar_v280: 0.0,
            scalar_v281: 0.0,
            scalar_v282: false,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: 0.0,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v288: 0.0,
            scalar_v289: 0.0,
            scalar_v290: 0.0,
            scalar_v291: 0.0,
            scalar_v292: 0.0,
            scalar_v293: 0.0,
            scalar_v294: 0.0,
            scalar_v295: 0.0,
            scalar_v296: 0.0,
            scalar_v297: 0.0,
            scalar_v298: 0.0,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: 0.0,
            scalar_v302: 0.0,
            scalar_v303: false,
            scalar_v304: false,
            scalar_v305: false,
            scalar_v307: 0.0,
            scalar_v315: 0.0,
            scalar_v318: false,
            scalar_v333: false,
            scalar_v334: false,
            scalar_v335: false,
            scalar_v336: false,
            scalar_v345: false,
            scalar_v355: 0.0,
            scalar_v356: false,
            scalar_v357: 0.0,
            scalar_v358: false,
            scalar_v359: false,
            scalar_v360: 0.0,
            scalar_v361: false,
            scalar_v362: false,
            scalar_v363: 0.0,
            scalar_v364: false,
            scalar_v372: false,
            scalar_v374: false,
            scalar_v375: false,
            scalar_v376: 0.0,
            scalar_v377: false,
            scalar_v378: 0.0,
            scalar_v379: false,
            scalar_v380: 0.0,
            scalar_v381: false,
            scalar_v382: 0.0,
            scalar_v383: false,
            scalar_v384: 0.0,
            scalar_v390: false,
            scalar_v391: 0.0,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
            scalar_v418: 0.0,
            scalar_v419: 0.0,
            scalar_v420: 0.0,
            scalar_v421: 0.0,
            scalar_v422: 0.0,
            scalar_v424: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: 0.0,
            scalar_v311: 0.0,
            scalar_v312: 0.0,
            scalar_v313: 0.0,
            scalar_v314: 0.0,
            scalar_v316: 0.0,
            scalar_v317: 0.0,
            scalar_v319: 0.0,
            scalar_v320: 0.0,
            scalar_v321: 0.0,
            scalar_v322: 0.0,
            scalar_v323: 0.0,
            scalar_v324: 0.0,
            scalar_v326: false,
            scalar_v327: false,
            scalar_v328: 0.0,
            scalar_v338: 0.0,
            scalar_v339: 0.0,
            scalar_v340: 0.0,
            scalar_v341: 0.0,
            scalar_v342: 0.0,
            scalar_v343: 0.0,
            scalar_v344: 0.0,
            scalar_v346: 0.0,
            scalar_v347: 0.0,
            scalar_v348: 0.0,
            scalar_v349: 0.0,
            scalar_v350: 0.0,
            scalar_v351: 0.0,
            scalar_v352: false,
            scalar_v353: false,
            scalar_v354: 0.0,
            scalar_v365: false,
            scalar_v366: false,
            scalar_v367: 0.0,
            scalar_v368: 0.0,
            scalar_v369: false,
            scalar_v370: false,
            scalar_v371: 0.0,
            scalar_v373: 0.0,
            scalar_v423: 0.0,
            scalar_temperature_static_valid: false,
            scalar_temperature_static_temperature: 0.0,
            scalar_temperature_static_thermal_voltage: 0.0,
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
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v11,
            scalar_v12,
            scalar_v15,
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
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v36,
            scalar_v37,
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
            scalar_v73,
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
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
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
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
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
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v212,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v234,
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v307,
            scalar_v315,
            scalar_v318,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v345,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v372,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v390,
            scalar_v391,
            scalar_v402,
            scalar_v403,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v424,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v316,
            scalar_v317,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v365,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v373,
            scalar_v423,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            scalar_v4,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v8,
            scalar_v9,
            scalar_v11,
            scalar_v12,
            scalar_v15,
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
            scalar_v31,
            scalar_v32,
            scalar_v33,
            scalar_v34,
            scalar_v36,
            scalar_v37,
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
            scalar_v73,
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
            scalar_v138,
            scalar_v139,
            scalar_v140,
            scalar_v141,
            scalar_v142,
            scalar_v143,
            scalar_v144,
            scalar_v145,
            scalar_v146,
            scalar_v147,
            scalar_v148,
            scalar_v149,
            scalar_v150,
            scalar_v151,
            scalar_v152,
            scalar_v153,
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
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
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
            scalar_v184,
            scalar_v185,
            scalar_v186,
            scalar_v187,
            scalar_v188,
            scalar_v189,
            scalar_v190,
            scalar_v191,
            scalar_v192,
            scalar_v193,
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v198,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v205,
            scalar_v206,
            scalar_v207,
            scalar_v208,
            scalar_v209,
            scalar_v210,
            scalar_v212,
            scalar_v214,
            scalar_v215,
            scalar_v216,
            scalar_v217,
            scalar_v218,
            scalar_v219,
            scalar_v220,
            scalar_v221,
            scalar_v222,
            scalar_v223,
            scalar_v224,
            scalar_v225,
            scalar_v226,
            scalar_v227,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v233,
            scalar_v234,
            scalar_v235,
            scalar_v236,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v247,
            scalar_v248,
            scalar_v249,
            scalar_v250,
            scalar_v251,
            scalar_v252,
            scalar_v253,
            scalar_v254,
            scalar_v255,
            scalar_v256,
            scalar_v257,
            scalar_v258,
            scalar_v259,
            scalar_v260,
            scalar_v261,
            scalar_v262,
            scalar_v263,
            scalar_v264,
            scalar_v265,
            scalar_v266,
            scalar_v267,
            scalar_v268,
            scalar_v269,
            scalar_v270,
            scalar_v271,
            scalar_v272,
            scalar_v273,
            scalar_v274,
            scalar_v275,
            scalar_v276,
            scalar_v277,
            scalar_v278,
            scalar_v279,
            scalar_v280,
            scalar_v281,
            scalar_v282,
            scalar_v283,
            scalar_v284,
            scalar_v285,
            scalar_v286,
            scalar_v287,
            scalar_v288,
            scalar_v289,
            scalar_v290,
            scalar_v291,
            scalar_v292,
            scalar_v293,
            scalar_v294,
            scalar_v295,
            scalar_v296,
            scalar_v297,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v307,
            scalar_v315,
            scalar_v318,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v345,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v364,
            scalar_v372,
            scalar_v374,
            scalar_v375,
            scalar_v376,
            scalar_v377,
            scalar_v378,
            scalar_v379,
            scalar_v380,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v384,
            scalar_v390,
            scalar_v391,
            scalar_v402,
            scalar_v403,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
            scalar_v418,
            scalar_v419,
            scalar_v420,
            scalar_v421,
            scalar_v422,
            scalar_v424,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v316,
            scalar_v317,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v365,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v373,
            scalar_v423,
            scalar_temperature_static_valid,
            scalar_temperature_static_temperature,
            scalar_temperature_static_thermal_voltage,
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
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcdfm" => { validate_parameter("NSUBCDFM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "corsrd" => { validate_parameter("CORSRD", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cors" => { validate_parameter("CORS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cord" => { validate_parameter("CORD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coiprv" => { validate_parameter("COIPRV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "copprv" => { validate_parameter("COPPRV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coadov" => { validate_parameter("COADOV", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coisub" => { validate_parameter("COISUB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coiigs" => { validate_parameter("COIIGS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cogidl" => { validate_parameter("COGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coflick" => { validate_parameter("COFLICK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coisti" => { validate_parameter("COISTI", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "conqs" => { validate_parameter("CONQS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "conqsov" => { validate_parameter("CONQSOV", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cothrml" => { validate_parameter("COTHRML", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coign" => { validate_parameter("COIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "codfm" => { validate_parameter("CODFM", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coqovsm" => { validate_parameter("COQOVSM", value, Some((0.0, "0.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cosym" => { validate_parameter("COSYM", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coovlp" => { validate_parameter("COOVLP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coovlps" => { validate_parameter("COOVLPS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covbscl" => { validate_parameter("COVBSCL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coqovcl" => { validate_parameter("COQOVCL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cotemp" => { validate_parameter("COTEMP", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cordrift" => { validate_parameter("CORDRIFT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coerrrep" => { validate_parameter("COERRREP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "codep" => { validate_parameter("CODEP", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "covdsres" => { validate_parameter("COVDSRES", value, Some((-1.0, "-1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coddlt" => { validate_parameter("CODDLT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cohbd" => { validate_parameter("COHBD", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cosnp" => { validate_parameter("COSNP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "info" => { validate_finite_parameter("INFO", value)?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "codio" => { validate_parameter("CODIO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cofixcss" => { validate_parameter("COFIXCSS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coovjunc" => { validate_parameter("COOVJUNC", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "corg" => { validate_parameter("CORG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "corbnet" => { validate_parameter("CORBNET", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "coselfheat" => { validate_parameter("COSELFHEAT", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cosubnode" => { validate_parameter("COSUBNODE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cotrench" => { validate_parameter("COTRENCH", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbpb" => { validate_parameter("RBPB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbpd" => { validate_parameter("RBPD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rbps" => { validate_parameter("RBPS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdov13" => { validate_finite_parameter("RDOV13", value)?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdslp1" => { validate_finite_parameter("RDSLP1", value)?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvg11" => { validate_finite_parameter("RDVG11", value)?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdict1" => { validate_finite_parameter("RDICT1", value)?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "loverld" => { validate_parameter("LOVERLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wtrench" => { validate_parameter("WTRENCH", value, Some((0.0, "0.0")), false, Some((1e-5, "1e-5")), true, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lover" => { validate_parameter("LOVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lovers" => { validate_parameter("LOVERS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrift1" => { validate_parameter("LDRIFT1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrift2" => { validate_parameter("LDRIFT2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrift1s" => { validate_parameter("LDRIFT1S", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrift2s" => { validate_parameter("LDRIFT2S", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ldrifts" => { validate_parameter("LDRIFTS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subld1" => { validate_finite_parameter("SUBLD1", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subld2" => { validate_finite_parameter("SUBLD2", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd" => { validate_parameter("RD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "npext" => { validate_parameter("NPEXT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vover" => { validate_finite_parameter("VOVER", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "falph" => { validate_parameter("FALPH", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgbo" => { validate_parameter("CGBO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0" => { validate_parameter("RTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "powrat" => { validate_finite_parameter("POWRAT", value)?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvd" => { validate_finite_parameter("RDVD", value)?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd23" => { validate_finite_parameter("RD23", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd24" => { validate_finite_parameter("RD24", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvb" => { validate_finite_parameter("RDVB", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cvdsover" => { validate_parameter("CVDSOVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "version" => { validate_finite_parameter("VERSION", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmax" => { validate_parameter("VMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmaxt1" => { validate_finite_parameter("VMAXT1", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vmaxt2" => { validate_finite_parameter("VMAXT2", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgtmp1" => { validate_finite_parameter("BGTMP1", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bgtmp2" => { validate_finite_parameter("BGTMP2", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "eg0" => { validate_finite_parameter("EG0", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tox" => { validate_parameter("TOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "toxb" => { validate_finite_parameter("TOXB", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xld" => { validate_parameter("XLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdov11" => { validate_finite_parameter("RDOV11", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdov12" => { validate_finite_parameter("RDOV12", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdslp2" => { validate_finite_parameter("RDSLP2", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdict2" => { validate_finite_parameter("RDICT2", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subld1l" => { validate_finite_parameter("SUBLD1L", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subld1lp" => { validate_finite_parameter("SUBLD1LP", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xpdv" => { validate_finite_parameter("XPDV", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xpvdth" => { validate_finite_parameter("XPVDTH", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xpvdthg" => { validate_finite_parameter("XPVDTHG", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltmax" => { validate_parameter("DDLTMAX", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltslp" => { validate_parameter("DDLTSLP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddltict" => { validate_finite_parameter("DDLTICT", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbover" => { validate_finite_parameter("VFBOVER", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nover" => { validate_parameter("NOVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "novers" => { validate_parameter("NOVERS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "olmdlt" => { validate_parameter("OLMDLT", value, Some((0.0, "0.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xwd" => { validate_finite_parameter("XWD", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xwdc" => { validate_finite_parameter("XWDC", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xl" => { validate_finite_parameter("XL", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xw" => { validate_finite_parameter("XW", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ll" => { validate_finite_parameter("LL", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lld" => { validate_finite_parameter("LLD", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lln" => { validate_finite_parameter("LLN", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl" => { validate_finite_parameter("WL", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl1" => { validate_finite_parameter("WL1", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl1p" => { validate_finite_parameter("WL1P", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl2" => { validate_finite_parameter("WL2", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wl2p" => { validate_finite_parameter("WL2P", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wld" => { validate_finite_parameter("WLD", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wln" => { validate_finite_parameter("WLN", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshs" => { validate_parameter("RSHS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy" => { validate_parameter("XQY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy1" => { validate_finite_parameter("XQY1", value)?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xqy2" => { validate_finite_parameter("XQY2", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbc" => { validate_finite_parameter("VFBC", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbi" => { validate_finite_parameter("VBI", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubc" => { validate_parameter("NSUBC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "parl2" => { validate_finite_parameter("PARL2", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lp" => { validate_parameter("LP", value, Some((0.0, "0.0")), false, None, false, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubp" => { validate_parameter("NSUBP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubp0" => { validate_finite_parameter("NSUBP0", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubwp" => { validate_finite_parameter("NSUBWP", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp1" => { validate_finite_parameter("SCP1", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp2" => { validate_finite_parameter("SCP2", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp3" => { validate_finite_parameter("SCP3", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc1" => { validate_finite_parameter("SC1", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc2" => { validate_finite_parameter("SC2", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc3" => { validate_finite_parameter("SC3", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sc4" => { validate_finite_parameter("SC4", value)?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd1" => { validate_finite_parameter("PGD1", value)?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd2" => { validate_finite_parameter("PGD2", value)?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgd4" => { validate_finite_parameter("PGD4", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndep" => { validate_finite_parameter("NDEP", value)?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndepl" => { validate_finite_parameter("NDEPL", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndeplp" => { validate_finite_parameter("NDEPLP", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninv" => { validate_finite_parameter("NINV", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb0" => { validate_finite_parameter("MUECB0", value)?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muecb1" => { validate_finite_parameter("MUECB1", value)?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph0" => { validate_finite_parameter("MUEPH0", value)?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueph1" => { validate_parameter("MUEPH1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephw" => { validate_finite_parameter("MUEPHW", value)?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepwp" => { validate_finite_parameter("MUEPWP", value)?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephl" => { validate_finite_parameter("MUEPHL", value)?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueplp" => { validate_finite_parameter("MUEPLP", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueefb" => { validate_finite_parameter("MUEEFB", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muephs" => { validate_finite_parameter("MUEPHS", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muepsp" => { validate_finite_parameter("MUEPSP", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vtmp" => { validate_finite_parameter("VTMP", value)?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvth0" => { validate_finite_parameter("WVTH0", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr0" => { validate_finite_parameter("MUESR0", value)?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesr1" => { validate_parameter("MUESR1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrl" => { validate_finite_parameter("MUESRL", value)?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesrw" => { validate_finite_parameter("MUESRW", value)?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueswp" => { validate_finite_parameter("MUESWP", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mueslp" => { validate_finite_parameter("MUESLP", value)?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muetmp" => { validate_finite_parameter("MUETMP", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bb" => { validate_parameter("BB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1" => { validate_finite_parameter("SUB1", value)?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub2" => { validate_finite_parameter("SUB2", value)?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgs" => { validate_finite_parameter("SVGS", value)?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbs" => { validate_finite_parameter("SVBS", value)?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbsl" => { validate_finite_parameter("SVBSL", value)?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svbslp" => { validate_finite_parameter("SVBSLP", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svds" => { validate_finite_parameter("SVDS", value)?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "slg" => { validate_parameter("SLG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1snp" => { validate_finite_parameter("SUB1SNP", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub2snp" => { validate_finite_parameter("SUB2SNP", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svdssnp" => { validate_finite_parameter("SVDSSNP", value)?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1l" => { validate_finite_parameter("SUB1L", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub1lp" => { validate_finite_parameter("SUB1LP", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "sub2l" => { validate_finite_parameter("SUB2L", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "subtmp" => { validate_finite_parameter("SUBTMP", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fn1" => { validate_finite_parameter("FN1", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fn2" => { validate_finite_parameter("FN2", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fn3" => { validate_finite_parameter("FN3", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "fvbs" => { validate_finite_parameter("FVBS", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgsl" => { validate_finite_parameter("SVGSL", value)?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgslp" => { validate_finite_parameter("SVGSLP", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgsw" => { validate_finite_parameter("SVGSW", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "svgswp" => { validate_finite_parameter("SVGSWP", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "slgl" => { validate_finite_parameter("SLGL", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "slglp" => { validate_finite_parameter("SLGLP", value)?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsti" => { validate_parameter("NSTI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsti" => { validate_finite_parameter("WSTI", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstil" => { validate_finite_parameter("WSTIL", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstilp" => { validate_finite_parameter("WSTILP", value)?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstiw" => { validate_finite_parameter("WSTIW", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wstiwp" => { validate_finite_parameter("WSTIWP", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scsti1" => { validate_finite_parameter("SCSTI1", value)?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scsti2" => { validate_finite_parameter("SCSTI2", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vthsti" => { validate_finite_parameter("VTHSTI", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdsti" => { validate_finite_parameter("VDSTI", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti1" => { validate_parameter("MUESTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti2" => { validate_parameter("MUESTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "muesti3" => { validate_finite_parameter("MUESTI3", value)?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti1" => { validate_parameter("NSUBPSTI1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti2" => { validate_parameter("NSUBPSTI2", value, Some((-1.0, "-1.0")), true, None, true, &[])?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubpsti3" => { validate_finite_parameter("NSUBPSTI3", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lpext" => { validate_parameter("LPEXT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp21" => { validate_finite_parameter("SCP21", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "scp22" => { validate_finite_parameter("SCP22", value)?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bs1" => { validate_finite_parameter("BS1", value)?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "bs2" => { validate_finite_parameter("BS2", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpoly" => { validate_parameter("TPOLY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm1" => { validate_finite_parameter("CLM1", value)?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm2" => { validate_finite_parameter("CLM2", value)?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm3" => { validate_finite_parameter("CLM3", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm5" => { validate_finite_parameter("CLM5", value)?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "clm6" => { validate_finite_parameter("CLM6", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voverp" => { validate_finite_parameter("VOVERP", value)?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfc" => { validate_finite_parameter("WFC", value)?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcw" => { validate_finite_parameter("NSUBCW", value)?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubcwp" => { validate_finite_parameter("NSUBCWP", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme1" => { validate_finite_parameter("QME1", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme2" => { validate_finite_parameter("QME2", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qme3" => { validate_finite_parameter("QME3", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vovers" => { validate_finite_parameter("VOVERS", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "voversp" => { validate_finite_parameter("VOVERSP", value)?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl1" => { validate_finite_parameter("GIDL1", value)?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl2" => { validate_finite_parameter("GIDL2", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl3" => { validate_finite_parameter("GIDL3", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl4" => { validate_finite_parameter("GIDL4", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gidl5" => { validate_finite_parameter("GIDL5", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak1" => { validate_finite_parameter("GLEAK1", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak2" => { validate_finite_parameter("GLEAK2", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak3" => { validate_finite_parameter("GLEAK3", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak4" => { validate_finite_parameter("GLEAK4", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak5" => { validate_parameter("GLEAK5", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak6" => { validate_finite_parameter("GLEAK6", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gleak7" => { validate_finite_parameter("GLEAK7", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glpart1" => { validate_finite_parameter("GLPART1", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd1" => { validate_finite_parameter("GLKSD1", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd2" => { validate_finite_parameter("GLKSD2", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glksd3" => { validate_finite_parameter("GLKSD3", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb1" => { validate_finite_parameter("GLKB1", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb2" => { validate_finite_parameter("GLKB2", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "glkb3" => { validate_finite_parameter("GLKB3", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "egig" => { validate_finite_parameter("EGIG", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igtemp2" => { validate_finite_parameter("IGTEMP2", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "igtemp3" => { validate_finite_parameter("IGTEMP3", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vzadd0" => { validate_parameter("VZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pzadd0" => { validate_parameter("PZADD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nftrp" => { validate_finite_parameter("NFTRP", value)?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nfalp" => { validate_finite_parameter("NFALP", value)?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "kappa" => { validate_parameter("KAPPA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgdo" => { validate_parameter("CGDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cgso" => { validate_parameter("CGSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly1" => { validate_parameter("DLY1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly2" => { validate_finite_parameter("DLY2", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dly3" => { validate_parameter("DLY3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "dlyov" => { validate_parameter("DLYOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tnom" => { validate_parameter("TNOM", value, Some((-273.15, "-273.15")), true, None, true, &[])?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ovslp" => { validate_finite_parameter("OVSLP", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ovmag" => { validate_finite_parameter("OVMAG", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gbmin" => { validate_parameter("GBMIN", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc1" => { validate_finite_parameter("IBPC1", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc1l" => { validate_finite_parameter("IBPC1L", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc1lp" => { validate_finite_parameter("IBPC1LP", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ibpc2" => { validate_finite_parameter("IBPC2", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mphdfm" => { validate_finite_parameter("MPHDFM", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptl" => { validate_finite_parameter("PTL", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptp" => { validate_finite_parameter("PTP", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt2" => { validate_finite_parameter("PT2", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ptlp" => { validate_finite_parameter("PTLP", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdl" => { validate_finite_parameter("GDL", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdlp" => { validate_finite_parameter("GDLP", value)?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdld" => { validate_finite_parameter("GDLD", value)?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt4" => { validate_finite_parameter("PT4", value)?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pt4p" => { validate_finite_parameter("PT4P", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvg12" => { validate_finite_parameter("RDVG12", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cth0" => { validate_parameter("CTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xldld" => { validate_parameter("XLDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xwdld" => { validate_finite_parameter("XWDLD", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd20" => { validate_finite_parameter("RD20", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd21" => { validate_finite_parameter("RD21", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd22" => { validate_finite_parameter("RD22", value)?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd22d" => { validate_finite_parameter("RD22D", value)?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd25" => { validate_finite_parameter("RD25", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdl" => { validate_finite_parameter("RDVDL", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdlp" => { validate_finite_parameter("RDVDLP", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvds" => { validate_finite_parameter("RDVDS", value)?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdsp" => { validate_finite_parameter("RDVDSP", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd23l" => { validate_finite_parameter("RD23L", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd23lp" => { validate_finite_parameter("RD23LP", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd23s" => { validate_finite_parameter("RD23S", value)?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rd23sp" => { validate_finite_parameter("RD23SP", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rds" => { validate_finite_parameter("RDS", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdsp" => { validate_finite_parameter("RDSP", value)?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdtemp1" => { validate_finite_parameter("RDTEMP1", value)?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdtemp2" => { validate_finite_parameter("RDTEMP2", value)?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdtemp1" => { validate_finite_parameter("RDVDTEMP1", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdtemp2" => { validate_finite_parameter("RDVDTEMP2", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0w" => { validate_finite_parameter("RTH0W", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0wp" => { validate_finite_parameter("RTH0WP", value)?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0l" => { validate_finite_parameter("RTH0L", value)?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0lp" => { validate_finite_parameter("RTH0LP", value)?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvd" => { validate_finite_parameter("NINVD", value)?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdl" => { validate_finite_parameter("NINVDL", value)?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdlp" => { validate_finite_parameter("NINVDLP", value)?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdw" => { validate_finite_parameter("NINVDW", value)?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdwp" => { validate_finite_parameter("NINVDWP", value)?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdt1" => { validate_finite_parameter("NINVDT1", value)?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ninvdt2" => { validate_finite_parameter("NINVDT2", value)?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbsmin" => { validate_finite_parameter("VBSMIN", value)?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rth0nf" => { validate_finite_parameter("RTH0NF", value)?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthtemp1" => { validate_finite_parameter("RTHTEMP1", value)?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rthtemp2" => { validate_finite_parameter("RTHTEMP2", value)?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prattemp1" => { validate_finite_parameter("PRATTEMP1", value)?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prattemp2" => { validate_finite_parameter("PRATTEMP2", value)?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvsub" => { validate_finite_parameter("RDVSUB", value)?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdvdsub" => { validate_finite_parameter("RDVDSUB", value)?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ddrift" => { validate_parameter("DDRIFT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbisub" => { validate_finite_parameter("VBISUB", value)?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nsubsub" => { validate_parameter("NSUBSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shemaxdlt" => { validate_parameter("SHEMAXDLT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbfwdmx" => { validate_parameter("VBFWDMX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vbfwdbnd" => { validate_parameter("VBFWDBND", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndepm" => { validate_parameter("NDEPM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndepml" => { validate_finite_parameter("NDEPML", value)?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndepmlp" => { validate_finite_parameter("NDEPMLP", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tndep" => { validate_parameter("TNDEP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tndepmin" => { validate_finite_parameter("TNDEPMIN", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tndepv" => { validate_finite_parameter("TNDEPV", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue0" => { validate_finite_parameter("DEPMUE0", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue0l" => { validate_finite_parameter("DEPMUE0L", value)?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue0lp" => { validate_finite_parameter("DEPMUE0LP", value)?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue1" => { validate_finite_parameter("DEPMUE1", value)?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue1l" => { validate_finite_parameter("DEPMUE1L", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue1lp" => { validate_finite_parameter("DEPMUE1LP", value)?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue2" => { validate_parameter("DEPMUE2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmuea1" => { validate_finite_parameter("DEPMUEA1", value)?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback0" => { validate_finite_parameter("DEPMUEBACK0", value)?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback1" => { validate_finite_parameter("DEPMUEBACK1", value)?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback0l" => { validate_finite_parameter("DEPMUEBACK0L", value)?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback0lp" => { validate_finite_parameter("DEPMUEBACK0LP", value)?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback1l" => { validate_finite_parameter("DEPMUEBACK1L", value)?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueback1lp" => { validate_finite_parameter("DEPMUEBACK1LP", value)?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depleak" => { validate_finite_parameter("DEPLEAK", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depleakl" => { validate_finite_parameter("DEPLEAKL", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depleaklp" => { validate_finite_parameter("DEPLEAKLP", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depjleak" => { validate_parameter("DEPJLEAK", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depwlp" => { validate_finite_parameter("DEPWLP", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depwlpt" => { validate_finite_parameter("DEPWLPT", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depeta" => { validate_finite_parameter("DEPETA", value)?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvmax" => { validate_parameter("DEPVMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvmaxl" => { validate_finite_parameter("DEPVMAXL", value)?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvmaxlp" => { validate_finite_parameter("DEPVMAXLP", value)?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef1" => { validate_finite_parameter("DEPVDSEF1", value)?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef2" => { validate_finite_parameter("DEPVDSEF2", value)?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef1l" => { validate_finite_parameter("DEPVDSEF1L", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef1lp" => { validate_finite_parameter("DEPVDSEF1LP", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef2l" => { validate_finite_parameter("DEPVDSEF2L", value)?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvdsef2lp" => { validate_finite_parameter("DEPVDSEF2LP", value)?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueph0" => { validate_finite_parameter("DEPMUEPH0", value)?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmueph1" => { validate_parameter("DEPMUEPH1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depbb" => { validate_parameter("DEPBB", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvtmp" => { validate_finite_parameter("DEPVTMP", value)?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmuetmp" => { validate_finite_parameter("DEPMUETMP", value)?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue0tmp" => { validate_finite_parameter("DEPMUE0TMP", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depmue2tmp" => { validate_finite_parameter("DEPMUE2TMP", value)?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depddlt" => { validate_parameter("DEPDDLT", value, Some((0.1, "0.1")), false, Some((20.0, "20.0")), false, &[])?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdc" => { validate_finite_parameter("DEPNINVDC", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdh" => { validate_finite_parameter("DEPNINVDH", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdl" => { validate_finite_parameter("DEPNINVDL", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdlp" => { validate_finite_parameter("DEPNINVDLP", value)?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdw" => { validate_finite_parameter("DEPNINVDW", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdwp" => { validate_finite_parameter("DEPNINVDWP", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdt1" => { validate_finite_parameter("DEPNINVDT1", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depninvdt2" => { validate_finite_parameter("DEPNINVDT2", value)?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvfbc" => { validate_finite_parameter("DEPVFBC", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depdvfbc" => { validate_finite_parameter("DEPDVFBC", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depsubsl" => { validate_parameter("DEPSUBSL", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depsubsl0" => { validate_parameter("DEPSUBSL0", value, Some((1e-8, "1e-8")), false, None, true, &[])?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvsatr" => { validate_finite_parameter("DEPVSATR", value)?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvsata" => { validate_finite_parameter("DEPVSATA", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deprbr" => { validate_parameter("DEPRBR", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvleak" => { validate_parameter("DEPVLEAK", value, Some((-0.5, "-0.5")), false, Some((1.0, "1.0")), false, &[])?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depcar" => { validate_finite_parameter("DEPCAR", value)?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deprdrdl1" => { validate_finite_parameter("DEPRDRDL1", value)?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deprdrdl2" => { validate_finite_parameter("DEPRDRDL2", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depps" => { validate_finite_parameter("DEPPS", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depqf" => { validate_parameter("DEPQF", value, Some((1e-8, "1e-8")), false, Some((8.0, "8.0")), false, &[])?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depqfres" => { validate_parameter("DEPQFRES", value, Some((1e-8, "1e-8")), false, Some((8.0, "8.0")), false, &[])?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depfdpd" => { validate_parameter("DEPFDPD", value, Some((1e-8, "1e-8")), false, Some((4.0, "4.0")), false, &[])?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depvgpsl" => { validate_parameter("DEPVGPSL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "deppb0" => { validate_parameter("DEPPB0", value, Some((0.0, "0.0")), false, Some((0.5, "0.5")), false, &[])?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmue" => { validate_parameter("RDRMUE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuebs1" => { validate_finite_parameter("RDRMUEBS1", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuebs2" => { validate_finite_parameter("RDRMUEBS2", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmax" => { validate_parameter("RDRVMAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmues" => { validate_parameter("RDRMUES", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxs" => { validate_parameter("RDRVMAXS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuetmp" => { validate_finite_parameter("RDRMUETMP", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuetmps" => { validate_finite_parameter("RDRMUETMPS", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvtmp" => { validate_finite_parameter("RDRVTMP", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvtmps" => { validate_finite_parameter("RDRVTMPS", value)?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrdjunc" => { validate_parameter("RDRDJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrcx" => { validate_finite_parameter("RDRCX", value)?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrcar" => { validate_finite_parameter("RDRCAR", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrdl1" => { validate_finite_parameter("RDRDL1", value)?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrdl2" => { validate_parameter("RDRDL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxw" => { validate_finite_parameter("RDRVMAXW", value)?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxwp" => { validate_finite_parameter("RDRVMAXWP", value)?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxl" => { validate_finite_parameter("RDRVMAXL", value)?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrvmaxlp" => { validate_finite_parameter("RDRVMAXLP", value)?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuel" => { validate_finite_parameter("RDRMUEL", value)?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrmuelp" => { validate_finite_parameter("RDRMUELP", value)?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrqover" => { validate_parameter("RDRQOVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qovadd" => { validate_parameter("QOVADD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "qovjunc" => { validate_parameter("QOVJUNC", value, Some((-1.0, "-1.0")), false, Some((50.0, "50.0")), true, &[])?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "shemax" => { validate_parameter("SHEMAX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vgsmin" => { validate_finite_parameter("VGSMIN", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gdsleak" => { validate_parameter("GDSLEAK", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), true, &[])?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbb" => { validate_parameter("RDRBB", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbs" => { validate_parameter("RDRBBS", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbtmp" => { validate_finite_parameter("RDRBBTMP", value)?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rdrbbtmps" => { validate_finite_parameter("RDRBBTMPS", value)?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndrilim" => { validate_parameter("NDRILIM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndridlt" => { validate_parameter("NDRIDLT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndripw" => { validate_parameter("NDRIPW", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "gmin" => { validate_parameter("GMIN", value, Some((0.0, "0.0")), false, Some((10000.0, "10000.0")), false, &[])?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "rmin" => { validate_parameter("RMIN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hbda" => { validate_finite_parameter("HBDA", value)?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hbdb" => { validate_finite_parameter("HBDB", value)?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hbdc" => { validate_finite_parameter("HBDC", value)?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hbdctmp" => { validate_finite_parameter("HBDCTMP", value)?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "hbdf" => { validate_finite_parameter("HBDF", value)?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "copt" => { validate_parameter("COPT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "copspt" => { validate_parameter("COPSPT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xjpt" => { validate_parameter("XJPT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), true, &[])?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njunc" => { validate_parameter("NJUNC", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mupt" => { validate_parameter("MUPT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vfbpt" => { validate_finite_parameter("VFBPT", value)?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pslimpt" => { validate_finite_parameter("PSLIMPT", value)?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ps0pt" => { validate_parameter("PS0PT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0" => { validate_finite_parameter("JS0", value)?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0sw" => { validate_finite_parameter("JS0SW", value)?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0swg" => { validate_finite_parameter("JS0SWG", value)?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nj" => { validate_parameter("NJ", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njsw" => { validate_parameter("NJSW", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njswg" => { validate_parameter("NJSWG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xti" => { validate_finite_parameter("XTI", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cj" => { validate_finite_parameter("CJ", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjsw" => { validate_finite_parameter("CJSW", value)?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjswg" => { validate_finite_parameter("CJSWG", value)?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mj" => { validate_parameter("MJ", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjsw" => { validate_parameter("MJSW", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjswg" => { validate_parameter("MJSWG", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pb" => { validate_parameter("PB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbsw" => { validate_parameter("PBSW", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbswg" => { validate_parameter("PBSWG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xti2" => { validate_finite_parameter("XTI2", value)?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisb" => { validate_finite_parameter("CISB", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cvb" => { validate_finite_parameter("CVB", value)?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctemp" => { validate_finite_parameter("CTEMP", value)?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisbk" => { validate_finite_parameter("CISBK", value)?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "divx" => { validate_finite_parameter("DIVX", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdiffj" => { validate_finite_parameter("VDIFFJ", value)?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbd" => { validate_finite_parameter("TCJBD", value)?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbs" => { validate_finite_parameter("TCJBS", value)?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbdsw" => { validate_finite_parameter("TCJBDSW", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbssw" => { validate_finite_parameter("TCJBSSW", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbdswg" => { validate_finite_parameter("TCJBDSWG", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tcjbsswg" => { validate_finite_parameter("TCJBSSWG", value)?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbd" => { validate_finite_parameter("TPBBD", value)?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbs" => { validate_finite_parameter("TPBBS", value)?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbdsw" => { validate_finite_parameter("TPBBDSW", value)?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbssw" => { validate_finite_parameter("TPBBSSW", value)?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbdswg" => { validate_finite_parameter("TPBBDSWG", value)?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tpbbsswg" => { validate_finite_parameter("TPBBSSWG", value)?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0d" => { validate_finite_parameter("JS0D", value)?; self.params.p493 = value; self.mark_param_given(493); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0swd" => { validate_finite_parameter("JS0SWD", value)?; self.params.p494 = value; self.mark_param_given(494); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0swgd" => { validate_finite_parameter("JS0SWGD", value)?; self.params.p495 = value; self.mark_param_given(495); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njd" => { validate_parameter("NJD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p496 = value; self.mark_param_given(496); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njswd" => { validate_parameter("NJSWD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p497 = value; self.mark_param_given(497); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njswgd" => { validate_parameter("NJSWGD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p498 = value; self.mark_param_given(498); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xtid" => { validate_finite_parameter("XTID", value)?; self.params.p499 = value; self.mark_param_given(499); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjd" => { validate_finite_parameter("CJD", value)?; self.params.p500 = value; self.mark_param_given(500); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjswd" => { validate_finite_parameter("CJSWD", value)?; self.params.p501 = value; self.mark_param_given(501); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjswgd" => { validate_finite_parameter("CJSWGD", value)?; self.params.p502 = value; self.mark_param_given(502); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjd" => { validate_parameter("MJD", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p503 = value; self.mark_param_given(503); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjswd" => { validate_parameter("MJSWD", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p504 = value; self.mark_param_given(504); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjswgd" => { validate_parameter("MJSWGD", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p505 = value; self.mark_param_given(505); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbd" => { validate_parameter("PBD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p506 = value; self.mark_param_given(506); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbswd" => { validate_parameter("PBSWD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p507 = value; self.mark_param_given(507); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbswgd" => { validate_parameter("PBSWGD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p508 = value; self.mark_param_given(508); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xti2d" => { validate_finite_parameter("XTI2D", value)?; self.params.p509 = value; self.mark_param_given(509); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisbd" => { validate_finite_parameter("CISBD", value)?; self.params.p510 = value; self.mark_param_given(510); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cvbd" => { validate_finite_parameter("CVBD", value)?; self.params.p511 = value; self.mark_param_given(511); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctempd" => { validate_finite_parameter("CTEMPD", value)?; self.params.p512 = value; self.mark_param_given(512); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisbkd" => { validate_finite_parameter("CISBKD", value)?; self.params.p513 = value; self.mark_param_given(513); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "divxd" => { validate_finite_parameter("DIVXD", value)?; self.params.p514 = value; self.mark_param_given(514); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdiffjd" => { validate_finite_parameter("VDIFFJD", value)?; self.params.p515 = value; self.mark_param_given(515); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0s" => { validate_finite_parameter("JS0S", value)?; self.params.p516 = value; self.mark_param_given(516); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0sws" => { validate_finite_parameter("JS0SWS", value)?; self.params.p517 = value; self.mark_param_given(517); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "js0swgs" => { validate_finite_parameter("JS0SWGS", value)?; self.params.p518 = value; self.mark_param_given(518); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njs" => { validate_parameter("NJS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p519 = value; self.mark_param_given(519); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njsws" => { validate_parameter("NJSWS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p520 = value; self.mark_param_given(520); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "njswgs" => { validate_parameter("NJSWGS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p521 = value; self.mark_param_given(521); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xtis" => { validate_finite_parameter("XTIS", value)?; self.params.p522 = value; self.mark_param_given(522); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjs" => { validate_finite_parameter("CJS", value)?; self.params.p523 = value; self.mark_param_given(523); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjsws" => { validate_finite_parameter("CJSWS", value)?; self.params.p524 = value; self.mark_param_given(524); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cjswgs" => { validate_finite_parameter("CJSWGS", value)?; self.params.p525 = value; self.mark_param_given(525); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjs" => { validate_parameter("MJS", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p526 = value; self.mark_param_given(526); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjsws" => { validate_parameter("MJSWS", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p527 = value; self.mark_param_given(527); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "mjswgs" => { validate_parameter("MJSWGS", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p528 = value; self.mark_param_given(528); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbs" => { validate_parameter("PBS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p529 = value; self.mark_param_given(529); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbsws" => { validate_parameter("PBSWS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p530 = value; self.mark_param_given(530); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbswgs" => { validate_parameter("PBSWGS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p531 = value; self.mark_param_given(531); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "xti2s" => { validate_finite_parameter("XTI2S", value)?; self.params.p532 = value; self.mark_param_given(532); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisbs" => { validate_finite_parameter("CISBS", value)?; self.params.p533 = value; self.mark_param_given(533); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cvbs" => { validate_finite_parameter("CVBS", value)?; self.params.p534 = value; self.mark_param_given(534); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ctemps" => { validate_finite_parameter("CTEMPS", value)?; self.params.p535 = value; self.mark_param_given(535); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "cisbks" => { validate_finite_parameter("CISBKS", value)?; self.params.p536 = value; self.mark_param_given(536); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "divxs" => { validate_finite_parameter("DIVXS", value)?; self.params.p537 = value; self.mark_param_given(537); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "vdiffjs" => { validate_finite_parameter("VDIFFJS", value)?; self.params.p538 = value; self.mark_param_given(538); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "corecovery" => { validate_finite_parameter("CORECOVERY", value)?; self.params.p539 = value; self.mark_param_given(539); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ndibot" => { validate_finite_parameter("NDIBOT", value)?; self.params.p540 = value; self.mark_param_given(540); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "inj1" => { validate_finite_parameter("INJ1", value)?; self.params.p541 = value; self.mark_param_given(541); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "inj2" => { validate_finite_parameter("INJ2", value)?; self.params.p542 = value; self.mark_param_given(542); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "nqs" => { validate_finite_parameter("NQS", value)?; self.params.p543 = value; self.mark_param_given(543); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "tau" => { validate_finite_parameter("TAU", value)?; self.params.p544 = value; self.mark_param_given(544); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wi" => { validate_finite_parameter("WI", value)?; self.params.p545 = value; self.mark_param_given(545); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "depnqs" => { validate_finite_parameter("DEPNQS", value)?; self.params.p546 = value; self.mark_param_given(546); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "taut" => { validate_finite_parameter("TAUT", value)?; self.params.p547 = value; self.mark_param_given(547); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "injt" => { validate_finite_parameter("INJT", value)?; self.params.p548 = value; self.mark_param_given(548); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmin" => { validate_finite_parameter("LMIN", value)?; self.params.p549 = value; self.mark_param_given(549); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmax" => { validate_finite_parameter("LMAX", value)?; self.params.p550 = value; self.mark_param_given(550); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmin" => { validate_finite_parameter("WMIN", value)?; self.params.p551 = value; self.mark_param_given(551); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmax" => { validate_finite_parameter("WMAX", value)?; self.params.p552 = value; self.mark_param_given(552); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lbinn" => { validate_finite_parameter("LBINN", value)?; self.params.p553 = value; self.mark_param_given(553); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbinn" => { validate_finite_parameter("WBINN", value)?; self.params.p554 = value; self.mark_param_given(554); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvmax" => { validate_finite_parameter("LVMAX", value)?; self.params.p555 = value; self.mark_param_given(555); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lbgtmp1" => { validate_finite_parameter("LBGTMP1", value)?; self.params.p556 = value; self.mark_param_given(556); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lbgtmp2" => { validate_finite_parameter("LBGTMP2", value)?; self.params.p557 = value; self.mark_param_given(557); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "leg0" => { validate_finite_parameter("LEG0", value)?; self.params.p558 = value; self.mark_param_given(558); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvfbover" => { validate_finite_parameter("LVFBOVER", value)?; self.params.p559 = value; self.mark_param_given(559); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnover" => { validate_finite_parameter("LNOVER", value)?; self.params.p560 = value; self.mark_param_given(560); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnovers" => { validate_finite_parameter("LNOVERS", value)?; self.params.p561 = value; self.mark_param_given(561); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lwl2" => { validate_finite_parameter("LWL2", value)?; self.params.p562 = value; self.mark_param_given(562); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvfbc" => { validate_finite_parameter("LVFBC", value)?; self.params.p563 = value; self.mark_param_given(563); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsubc" => { validate_finite_parameter("LNSUBC", value)?; self.params.p564 = value; self.mark_param_given(564); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsubp" => { validate_finite_parameter("LNSUBP", value)?; self.params.p565 = value; self.mark_param_given(565); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lscp1" => { validate_finite_parameter("LSCP1", value)?; self.params.p566 = value; self.mark_param_given(566); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lscp2" => { validate_finite_parameter("LSCP2", value)?; self.params.p567 = value; self.mark_param_given(567); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lscp3" => { validate_finite_parameter("LSCP3", value)?; self.params.p568 = value; self.mark_param_given(568); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsc1" => { validate_finite_parameter("LSC1", value)?; self.params.p569 = value; self.mark_param_given(569); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsc2" => { validate_finite_parameter("LSC2", value)?; self.params.p570 = value; self.mark_param_given(570); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsc3" => { validate_finite_parameter("LSC3", value)?; self.params.p571 = value; self.mark_param_given(571); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lpgd1" => { validate_finite_parameter("LPGD1", value)?; self.params.p572 = value; self.mark_param_given(572); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lndep" => { validate_finite_parameter("LNDEP", value)?; self.params.p573 = value; self.mark_param_given(573); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lninv" => { validate_finite_parameter("LNINV", value)?; self.params.p574 = value; self.mark_param_given(574); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuecb0" => { validate_finite_parameter("LMUECB0", value)?; self.params.p575 = value; self.mark_param_given(575); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuecb1" => { validate_finite_parameter("LMUECB1", value)?; self.params.p576 = value; self.mark_param_given(576); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmueph1" => { validate_finite_parameter("LMUEPH1", value)?; self.params.p577 = value; self.mark_param_given(577); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvtmp" => { validate_finite_parameter("LVTMP", value)?; self.params.p578 = value; self.mark_param_given(578); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lwvth0" => { validate_finite_parameter("LWVTH0", value)?; self.params.p579 = value; self.mark_param_given(579); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuesr1" => { validate_finite_parameter("LMUESR1", value)?; self.params.p580 = value; self.mark_param_given(580); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuetmp" => { validate_finite_parameter("LMUETMP", value)?; self.params.p581 = value; self.mark_param_given(581); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsub1" => { validate_finite_parameter("LSUB1", value)?; self.params.p582 = value; self.mark_param_given(582); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsub2" => { validate_finite_parameter("LSUB2", value)?; self.params.p583 = value; self.mark_param_given(583); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsvds" => { validate_finite_parameter("LSVDS", value)?; self.params.p584 = value; self.mark_param_given(584); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsvbs" => { validate_finite_parameter("LSVBS", value)?; self.params.p585 = value; self.mark_param_given(585); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsvgs" => { validate_finite_parameter("LSVGS", value)?; self.params.p586 = value; self.mark_param_given(586); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsub1snp" => { validate_finite_parameter("LSUB1SNP", value)?; self.params.p587 = value; self.mark_param_given(587); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsub2snp" => { validate_finite_parameter("LSUB2SNP", value)?; self.params.p588 = value; self.mark_param_given(588); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lsvdssnp" => { validate_finite_parameter("LSVDSSNP", value)?; self.params.p589 = value; self.mark_param_given(589); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lfn1" => { validate_finite_parameter("LFN1", value)?; self.params.p590 = value; self.mark_param_given(590); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lfn2" => { validate_finite_parameter("LFN2", value)?; self.params.p591 = value; self.mark_param_given(591); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lfn3" => { validate_finite_parameter("LFN3", value)?; self.params.p592 = value; self.mark_param_given(592); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lfvbs" => { validate_finite_parameter("LFVBS", value)?; self.params.p593 = value; self.mark_param_given(593); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsti" => { validate_finite_parameter("LNSTI", value)?; self.params.p594 = value; self.mark_param_given(594); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lwsti" => { validate_finite_parameter("LWSTI", value)?; self.params.p595 = value; self.mark_param_given(595); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lscsti1" => { validate_finite_parameter("LSCSTI1", value)?; self.params.p596 = value; self.mark_param_given(596); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lscsti2" => { validate_finite_parameter("LSCSTI2", value)?; self.params.p597 = value; self.mark_param_given(597); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvthsti" => { validate_finite_parameter("LVTHSTI", value)?; self.params.p598 = value; self.mark_param_given(598); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuesti1" => { validate_finite_parameter("LMUESTI1", value)?; self.params.p599 = value; self.mark_param_given(599); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuesti2" => { validate_finite_parameter("LMUESTI2", value)?; self.params.p600 = value; self.mark_param_given(600); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lmuesti3" => { validate_finite_parameter("LMUESTI3", value)?; self.params.p601 = value; self.mark_param_given(601); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsubpsti1" => { validate_finite_parameter("LNSUBPSTI1", value)?; self.params.p602 = value; self.mark_param_given(602); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsubpsti2" => { validate_finite_parameter("LNSUBPSTI2", value)?; self.params.p603 = value; self.mark_param_given(603); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnsubpsti3" => { validate_finite_parameter("LNSUBPSTI3", value)?; self.params.p604 = value; self.mark_param_given(604); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcgso" => { validate_finite_parameter("LCGSO", value)?; self.params.p605 = value; self.mark_param_given(605); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcgdo" => { validate_finite_parameter("LCGDO", value)?; self.params.p606 = value; self.mark_param_given(606); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lclm1" => { validate_finite_parameter("LCLM1", value)?; self.params.p607 = value; self.mark_param_given(607); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lclm2" => { validate_finite_parameter("LCLM2", value)?; self.params.p608 = value; self.mark_param_given(608); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lclm3" => { validate_finite_parameter("LCLM3", value)?; self.params.p609 = value; self.mark_param_given(609); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lwfc" => { validate_finite_parameter("LWFC", value)?; self.params.p610 = value; self.mark_param_given(610); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgidl1" => { validate_finite_parameter("LGIDL1", value)?; self.params.p611 = value; self.mark_param_given(611); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgidl2" => { validate_finite_parameter("LGIDL2", value)?; self.params.p612 = value; self.mark_param_given(612); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgleak1" => { validate_finite_parameter("LGLEAK1", value)?; self.params.p613 = value; self.mark_param_given(613); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgleak2" => { validate_finite_parameter("LGLEAK2", value)?; self.params.p614 = value; self.mark_param_given(614); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgleak3" => { validate_finite_parameter("LGLEAK3", value)?; self.params.p615 = value; self.mark_param_given(615); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lgleak6" => { validate_finite_parameter("LGLEAK6", value)?; self.params.p616 = value; self.mark_param_given(616); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lglksd1" => { validate_finite_parameter("LGLKSD1", value)?; self.params.p617 = value; self.mark_param_given(617); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lglksd2" => { validate_finite_parameter("LGLKSD2", value)?; self.params.p618 = value; self.mark_param_given(618); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lglkb1" => { validate_finite_parameter("LGLKB1", value)?; self.params.p619 = value; self.mark_param_given(619); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lglkb2" => { validate_finite_parameter("LGLKB2", value)?; self.params.p620 = value; self.mark_param_given(620); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnftrp" => { validate_finite_parameter("LNFTRP", value)?; self.params.p621 = value; self.mark_param_given(621); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnfalp" => { validate_finite_parameter("LNFALP", value)?; self.params.p622 = value; self.mark_param_given(622); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "libpc1" => { validate_finite_parameter("LIBPC1", value)?; self.params.p623 = value; self.mark_param_given(623); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "libpc2" => { validate_finite_parameter("LIBPC2", value)?; self.params.p624 = value; self.mark_param_given(624); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcgbo" => { validate_finite_parameter("LCGBO", value)?; self.params.p625 = value; self.mark_param_given(625); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcvdsover" => { validate_finite_parameter("LCVDSOVER", value)?; self.params.p626 = value; self.mark_param_given(626); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lfalph" => { validate_finite_parameter("LFALPH", value)?; self.params.p627 = value; self.mark_param_given(627); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnpext" => { validate_finite_parameter("LNPEXT", value)?; self.params.p628 = value; self.mark_param_given(628); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lpowrat" => { validate_finite_parameter("LPOWRAT", value)?; self.params.p629 = value; self.mark_param_given(629); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrd" => { validate_finite_parameter("LRD", value)?; self.params.p630 = value; self.mark_param_given(630); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrd22" => { validate_finite_parameter("LRD22", value)?; self.params.p631 = value; self.mark_param_given(631); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrd23" => { validate_finite_parameter("LRD23", value)?; self.params.p632 = value; self.mark_param_given(632); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrd24" => { validate_finite_parameter("LRD24", value)?; self.params.p633 = value; self.mark_param_given(633); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdict1" => { validate_finite_parameter("LRDICT1", value)?; self.params.p634 = value; self.mark_param_given(634); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdov13" => { validate_finite_parameter("LRDOV13", value)?; self.params.p635 = value; self.mark_param_given(635); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdslp1" => { validate_finite_parameter("LRDSLP1", value)?; self.params.p636 = value; self.mark_param_given(636); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdvb" => { validate_finite_parameter("LRDVB", value)?; self.params.p637 = value; self.mark_param_given(637); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdvd" => { validate_finite_parameter("LRDVD", value)?; self.params.p638 = value; self.mark_param_given(638); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrdvg11" => { validate_finite_parameter("LRDVG11", value)?; self.params.p639 = value; self.mark_param_given(639); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrs" => { validate_finite_parameter("LRS", value)?; self.params.p640 = value; self.mark_param_given(640); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lrth0" => { validate_finite_parameter("LRTH0", value)?; self.params.p641 = value; self.mark_param_given(641); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvover" => { validate_finite_parameter("LVOVER", value)?; self.params.p642 = value; self.mark_param_given(642); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvmax" => { validate_finite_parameter("WVMAX", value)?; self.params.p643 = value; self.mark_param_given(643); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbgtmp1" => { validate_finite_parameter("WBGTMP1", value)?; self.params.p644 = value; self.mark_param_given(644); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wbgtmp2" => { validate_finite_parameter("WBGTMP2", value)?; self.params.p645 = value; self.mark_param_given(645); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "weg0" => { validate_finite_parameter("WEG0", value)?; self.params.p646 = value; self.mark_param_given(646); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvfbover" => { validate_finite_parameter("WVFBOVER", value)?; self.params.p647 = value; self.mark_param_given(647); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnover" => { validate_finite_parameter("WNOVER", value)?; self.params.p648 = value; self.mark_param_given(648); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnovers" => { validate_finite_parameter("WNOVERS", value)?; self.params.p649 = value; self.mark_param_given(649); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wwl2" => { validate_finite_parameter("WWL2", value)?; self.params.p650 = value; self.mark_param_given(650); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvfbc" => { validate_finite_parameter("WVFBC", value)?; self.params.p651 = value; self.mark_param_given(651); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsubc" => { validate_finite_parameter("WNSUBC", value)?; self.params.p652 = value; self.mark_param_given(652); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsubp" => { validate_finite_parameter("WNSUBP", value)?; self.params.p653 = value; self.mark_param_given(653); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wscp1" => { validate_finite_parameter("WSCP1", value)?; self.params.p654 = value; self.mark_param_given(654); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wscp2" => { validate_finite_parameter("WSCP2", value)?; self.params.p655 = value; self.mark_param_given(655); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wscp3" => { validate_finite_parameter("WSCP3", value)?; self.params.p656 = value; self.mark_param_given(656); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsc1" => { validate_finite_parameter("WSC1", value)?; self.params.p657 = value; self.mark_param_given(657); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsc2" => { validate_finite_parameter("WSC2", value)?; self.params.p658 = value; self.mark_param_given(658); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsc3" => { validate_finite_parameter("WSC3", value)?; self.params.p659 = value; self.mark_param_given(659); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wpgd1" => { validate_finite_parameter("WPGD1", value)?; self.params.p660 = value; self.mark_param_given(660); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wndep" => { validate_finite_parameter("WNDEP", value)?; self.params.p661 = value; self.mark_param_given(661); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wninv" => { validate_finite_parameter("WNINV", value)?; self.params.p662 = value; self.mark_param_given(662); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuecb0" => { validate_finite_parameter("WMUECB0", value)?; self.params.p663 = value; self.mark_param_given(663); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuecb1" => { validate_finite_parameter("WMUECB1", value)?; self.params.p664 = value; self.mark_param_given(664); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmueph1" => { validate_finite_parameter("WMUEPH1", value)?; self.params.p665 = value; self.mark_param_given(665); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvtmp" => { validate_finite_parameter("WVTMP", value)?; self.params.p666 = value; self.mark_param_given(666); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wwvth0" => { validate_finite_parameter("WWVTH0", value)?; self.params.p667 = value; self.mark_param_given(667); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuesr1" => { validate_finite_parameter("WMUESR1", value)?; self.params.p668 = value; self.mark_param_given(668); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuetmp" => { validate_finite_parameter("WMUETMP", value)?; self.params.p669 = value; self.mark_param_given(669); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsub1" => { validate_finite_parameter("WSUB1", value)?; self.params.p670 = value; self.mark_param_given(670); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsub2" => { validate_finite_parameter("WSUB2", value)?; self.params.p671 = value; self.mark_param_given(671); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsvds" => { validate_finite_parameter("WSVDS", value)?; self.params.p672 = value; self.mark_param_given(672); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsvbs" => { validate_finite_parameter("WSVBS", value)?; self.params.p673 = value; self.mark_param_given(673); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsvgs" => { validate_finite_parameter("WSVGS", value)?; self.params.p674 = value; self.mark_param_given(674); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsub1snp" => { validate_finite_parameter("WSUB1SNP", value)?; self.params.p675 = value; self.mark_param_given(675); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsub2snp" => { validate_finite_parameter("WSUB2SNP", value)?; self.params.p676 = value; self.mark_param_given(676); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wsvdssnp" => { validate_finite_parameter("WSVDSSNP", value)?; self.params.p677 = value; self.mark_param_given(677); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfn1" => { validate_finite_parameter("WFN1", value)?; self.params.p678 = value; self.mark_param_given(678); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfn2" => { validate_finite_parameter("WFN2", value)?; self.params.p679 = value; self.mark_param_given(679); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfn3" => { validate_finite_parameter("WFN3", value)?; self.params.p680 = value; self.mark_param_given(680); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfvbs" => { validate_finite_parameter("WFVBS", value)?; self.params.p681 = value; self.mark_param_given(681); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsti" => { validate_finite_parameter("WNSTI", value)?; self.params.p682 = value; self.mark_param_given(682); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wwsti" => { validate_finite_parameter("WWSTI", value)?; self.params.p683 = value; self.mark_param_given(683); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wscsti1" => { validate_finite_parameter("WSCSTI1", value)?; self.params.p684 = value; self.mark_param_given(684); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wscsti2" => { validate_finite_parameter("WSCSTI2", value)?; self.params.p685 = value; self.mark_param_given(685); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvthsti" => { validate_finite_parameter("WVTHSTI", value)?; self.params.p686 = value; self.mark_param_given(686); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuesti1" => { validate_finite_parameter("WMUESTI1", value)?; self.params.p687 = value; self.mark_param_given(687); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuesti2" => { validate_finite_parameter("WMUESTI2", value)?; self.params.p688 = value; self.mark_param_given(688); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wmuesti3" => { validate_finite_parameter("WMUESTI3", value)?; self.params.p689 = value; self.mark_param_given(689); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsubpsti1" => { validate_finite_parameter("WNSUBPSTI1", value)?; self.params.p690 = value; self.mark_param_given(690); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsubpsti2" => { validate_finite_parameter("WNSUBPSTI2", value)?; self.params.p691 = value; self.mark_param_given(691); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnsubpsti3" => { validate_finite_parameter("WNSUBPSTI3", value)?; self.params.p692 = value; self.mark_param_given(692); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcgso" => { validate_finite_parameter("WCGSO", value)?; self.params.p693 = value; self.mark_param_given(693); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcgdo" => { validate_finite_parameter("WCGDO", value)?; self.params.p694 = value; self.mark_param_given(694); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wclm1" => { validate_finite_parameter("WCLM1", value)?; self.params.p695 = value; self.mark_param_given(695); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wclm2" => { validate_finite_parameter("WCLM2", value)?; self.params.p696 = value; self.mark_param_given(696); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wclm3" => { validate_finite_parameter("WCLM3", value)?; self.params.p697 = value; self.mark_param_given(697); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wwfc" => { validate_finite_parameter("WWFC", value)?; self.params.p698 = value; self.mark_param_given(698); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgidl1" => { validate_finite_parameter("WGIDL1", value)?; self.params.p699 = value; self.mark_param_given(699); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgidl2" => { validate_finite_parameter("WGIDL2", value)?; self.params.p700 = value; self.mark_param_given(700); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgleak1" => { validate_finite_parameter("WGLEAK1", value)?; self.params.p701 = value; self.mark_param_given(701); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgleak2" => { validate_finite_parameter("WGLEAK2", value)?; self.params.p702 = value; self.mark_param_given(702); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgleak3" => { validate_finite_parameter("WGLEAK3", value)?; self.params.p703 = value; self.mark_param_given(703); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wgleak6" => { validate_finite_parameter("WGLEAK6", value)?; self.params.p704 = value; self.mark_param_given(704); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wglksd1" => { validate_finite_parameter("WGLKSD1", value)?; self.params.p705 = value; self.mark_param_given(705); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wglksd2" => { validate_finite_parameter("WGLKSD2", value)?; self.params.p706 = value; self.mark_param_given(706); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wglkb1" => { validate_finite_parameter("WGLKB1", value)?; self.params.p707 = value; self.mark_param_given(707); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wglkb2" => { validate_finite_parameter("WGLKB2", value)?; self.params.p708 = value; self.mark_param_given(708); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnftrp" => { validate_finite_parameter("WNFTRP", value)?; self.params.p709 = value; self.mark_param_given(709); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnfalp" => { validate_finite_parameter("WNFALP", value)?; self.params.p710 = value; self.mark_param_given(710); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wibpc1" => { validate_finite_parameter("WIBPC1", value)?; self.params.p711 = value; self.mark_param_given(711); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wibpc2" => { validate_finite_parameter("WIBPC2", value)?; self.params.p712 = value; self.mark_param_given(712); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcgbo" => { validate_finite_parameter("WCGBO", value)?; self.params.p713 = value; self.mark_param_given(713); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcvdsover" => { validate_finite_parameter("WCVDSOVER", value)?; self.params.p714 = value; self.mark_param_given(714); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wfalph" => { validate_finite_parameter("WFALPH", value)?; self.params.p715 = value; self.mark_param_given(715); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnpext" => { validate_finite_parameter("WNPEXT", value)?; self.params.p716 = value; self.mark_param_given(716); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wpowrat" => { validate_finite_parameter("WPOWRAT", value)?; self.params.p717 = value; self.mark_param_given(717); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrd" => { validate_finite_parameter("WRD", value)?; self.params.p718 = value; self.mark_param_given(718); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrd22" => { validate_finite_parameter("WRD22", value)?; self.params.p719 = value; self.mark_param_given(719); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrd23" => { validate_finite_parameter("WRD23", value)?; self.params.p720 = value; self.mark_param_given(720); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrd24" => { validate_finite_parameter("WRD24", value)?; self.params.p721 = value; self.mark_param_given(721); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdict1" => { validate_finite_parameter("WRDICT1", value)?; self.params.p722 = value; self.mark_param_given(722); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdov13" => { validate_finite_parameter("WRDOV13", value)?; self.params.p723 = value; self.mark_param_given(723); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdslp1" => { validate_finite_parameter("WRDSLP1", value)?; self.params.p724 = value; self.mark_param_given(724); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdvb" => { validate_finite_parameter("WRDVB", value)?; self.params.p725 = value; self.mark_param_given(725); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdvd" => { validate_finite_parameter("WRDVD", value)?; self.params.p726 = value; self.mark_param_given(726); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrdvg11" => { validate_finite_parameter("WRDVG11", value)?; self.params.p727 = value; self.mark_param_given(727); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrs" => { validate_finite_parameter("WRS", value)?; self.params.p728 = value; self.mark_param_given(728); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wrth0" => { validate_finite_parameter("WRTH0", value)?; self.params.p729 = value; self.mark_param_given(729); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvover" => { validate_finite_parameter("WVOVER", value)?; self.params.p730 = value; self.mark_param_given(730); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvmax" => { validate_finite_parameter("PVMAX", value)?; self.params.p731 = value; self.mark_param_given(731); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbgtmp1" => { validate_finite_parameter("PBGTMP1", value)?; self.params.p732 = value; self.mark_param_given(732); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pbgtmp2" => { validate_finite_parameter("PBGTMP2", value)?; self.params.p733 = value; self.mark_param_given(733); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "peg0" => { validate_finite_parameter("PEG0", value)?; self.params.p734 = value; self.mark_param_given(734); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvfbover" => { validate_finite_parameter("PVFBOVER", value)?; self.params.p735 = value; self.mark_param_given(735); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnover" => { validate_finite_parameter("PNOVER", value)?; self.params.p736 = value; self.mark_param_given(736); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnovers" => { validate_finite_parameter("PNOVERS", value)?; self.params.p737 = value; self.mark_param_given(737); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwl2" => { validate_finite_parameter("PWL2", value)?; self.params.p738 = value; self.mark_param_given(738); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvfbc" => { validate_finite_parameter("PVFBC", value)?; self.params.p739 = value; self.mark_param_given(739); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsubc" => { validate_finite_parameter("PNSUBC", value)?; self.params.p740 = value; self.mark_param_given(740); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsubp" => { validate_finite_parameter("PNSUBP", value)?; self.params.p741 = value; self.mark_param_given(741); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscp1" => { validate_finite_parameter("PSCP1", value)?; self.params.p742 = value; self.mark_param_given(742); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscp2" => { validate_finite_parameter("PSCP2", value)?; self.params.p743 = value; self.mark_param_given(743); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscp3" => { validate_finite_parameter("PSCP3", value)?; self.params.p744 = value; self.mark_param_given(744); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psc1" => { validate_finite_parameter("PSC1", value)?; self.params.p745 = value; self.mark_param_given(745); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psc2" => { validate_finite_parameter("PSC2", value)?; self.params.p746 = value; self.mark_param_given(746); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psc3" => { validate_finite_parameter("PSC3", value)?; self.params.p747 = value; self.mark_param_given(747); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ppgd1" => { validate_finite_parameter("PPGD1", value)?; self.params.p748 = value; self.mark_param_given(748); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pndep" => { validate_finite_parameter("PNDEP", value)?; self.params.p749 = value; self.mark_param_given(749); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pninv" => { validate_finite_parameter("PNINV", value)?; self.params.p750 = value; self.mark_param_given(750); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuecb0" => { validate_finite_parameter("PMUECB0", value)?; self.params.p751 = value; self.mark_param_given(751); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuecb1" => { validate_finite_parameter("PMUECB1", value)?; self.params.p752 = value; self.mark_param_given(752); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmueph1" => { validate_finite_parameter("PMUEPH1", value)?; self.params.p753 = value; self.mark_param_given(753); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvtmp" => { validate_finite_parameter("PVTMP", value)?; self.params.p754 = value; self.mark_param_given(754); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwvth0" => { validate_finite_parameter("PWVTH0", value)?; self.params.p755 = value; self.mark_param_given(755); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuesr1" => { validate_finite_parameter("PMUESR1", value)?; self.params.p756 = value; self.mark_param_given(756); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuetmp" => { validate_finite_parameter("PMUETMP", value)?; self.params.p757 = value; self.mark_param_given(757); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psub1" => { validate_finite_parameter("PSUB1", value)?; self.params.p758 = value; self.mark_param_given(758); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psub2" => { validate_finite_parameter("PSUB2", value)?; self.params.p759 = value; self.mark_param_given(759); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psvds" => { validate_finite_parameter("PSVDS", value)?; self.params.p760 = value; self.mark_param_given(760); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psvbs" => { validate_finite_parameter("PSVBS", value)?; self.params.p761 = value; self.mark_param_given(761); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psvgs" => { validate_finite_parameter("PSVGS", value)?; self.params.p762 = value; self.mark_param_given(762); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psub1snp" => { validate_finite_parameter("PSUB1SNP", value)?; self.params.p763 = value; self.mark_param_given(763); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psub2snp" => { validate_finite_parameter("PSUB2SNP", value)?; self.params.p764 = value; self.mark_param_given(764); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "psvdssnp" => { validate_finite_parameter("PSVDSSNP", value)?; self.params.p765 = value; self.mark_param_given(765); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pfn1" => { validate_finite_parameter("PFN1", value)?; self.params.p766 = value; self.mark_param_given(766); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pfn2" => { validate_finite_parameter("PFN2", value)?; self.params.p767 = value; self.mark_param_given(767); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pfn3" => { validate_finite_parameter("PFN3", value)?; self.params.p768 = value; self.mark_param_given(768); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pfvbs" => { validate_finite_parameter("PFVBS", value)?; self.params.p769 = value; self.mark_param_given(769); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsti" => { validate_finite_parameter("PNSTI", value)?; self.params.p770 = value; self.mark_param_given(770); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwsti" => { validate_finite_parameter("PWSTI", value)?; self.params.p771 = value; self.mark_param_given(771); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscsti1" => { validate_finite_parameter("PSCSTI1", value)?; self.params.p772 = value; self.mark_param_given(772); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pscsti2" => { validate_finite_parameter("PSCSTI2", value)?; self.params.p773 = value; self.mark_param_given(773); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvthsti" => { validate_finite_parameter("PVTHSTI", value)?; self.params.p774 = value; self.mark_param_given(774); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuesti1" => { validate_finite_parameter("PMUESTI1", value)?; self.params.p775 = value; self.mark_param_given(775); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuesti2" => { validate_finite_parameter("PMUESTI2", value)?; self.params.p776 = value; self.mark_param_given(776); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pmuesti3" => { validate_finite_parameter("PMUESTI3", value)?; self.params.p777 = value; self.mark_param_given(777); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsubpsti1" => { validate_finite_parameter("PNSUBPSTI1", value)?; self.params.p778 = value; self.mark_param_given(778); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsubpsti2" => { validate_finite_parameter("PNSUBPSTI2", value)?; self.params.p779 = value; self.mark_param_given(779); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnsubpsti3" => { validate_finite_parameter("PNSUBPSTI3", value)?; self.params.p780 = value; self.mark_param_given(780); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcgso" => { validate_finite_parameter("PCGSO", value)?; self.params.p781 = value; self.mark_param_given(781); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcgdo" => { validate_finite_parameter("PCGDO", value)?; self.params.p782 = value; self.mark_param_given(782); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pclm1" => { validate_finite_parameter("PCLM1", value)?; self.params.p783 = value; self.mark_param_given(783); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pclm2" => { validate_finite_parameter("PCLM2", value)?; self.params.p784 = value; self.mark_param_given(784); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pclm3" => { validate_finite_parameter("PCLM3", value)?; self.params.p785 = value; self.mark_param_given(785); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pwfc" => { validate_finite_parameter("PWFC", value)?; self.params.p786 = value; self.mark_param_given(786); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgidl1" => { validate_finite_parameter("PGIDL1", value)?; self.params.p787 = value; self.mark_param_given(787); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgidl2" => { validate_finite_parameter("PGIDL2", value)?; self.params.p788 = value; self.mark_param_given(788); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgleak1" => { validate_finite_parameter("PGLEAK1", value)?; self.params.p789 = value; self.mark_param_given(789); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgleak2" => { validate_finite_parameter("PGLEAK2", value)?; self.params.p790 = value; self.mark_param_given(790); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgleak3" => { validate_finite_parameter("PGLEAK3", value)?; self.params.p791 = value; self.mark_param_given(791); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pgleak6" => { validate_finite_parameter("PGLEAK6", value)?; self.params.p792 = value; self.mark_param_given(792); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pglksd1" => { validate_finite_parameter("PGLKSD1", value)?; self.params.p793 = value; self.mark_param_given(793); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pglksd2" => { validate_finite_parameter("PGLKSD2", value)?; self.params.p794 = value; self.mark_param_given(794); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pglkb1" => { validate_finite_parameter("PGLKB1", value)?; self.params.p795 = value; self.mark_param_given(795); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pglkb2" => { validate_finite_parameter("PGLKB2", value)?; self.params.p796 = value; self.mark_param_given(796); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnftrp" => { validate_finite_parameter("PNFTRP", value)?; self.params.p797 = value; self.mark_param_given(797); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnfalp" => { validate_finite_parameter("PNFALP", value)?; self.params.p798 = value; self.mark_param_given(798); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pibpc1" => { validate_finite_parameter("PIBPC1", value)?; self.params.p799 = value; self.mark_param_given(799); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pibpc2" => { validate_finite_parameter("PIBPC2", value)?; self.params.p800 = value; self.mark_param_given(800); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcgbo" => { validate_finite_parameter("PCGBO", value)?; self.params.p801 = value; self.mark_param_given(801); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcvdsover" => { validate_finite_parameter("PCVDSOVER", value)?; self.params.p802 = value; self.mark_param_given(802); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pfalph" => { validate_finite_parameter("PFALPH", value)?; self.params.p803 = value; self.mark_param_given(803); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnpext" => { validate_finite_parameter("PNPEXT", value)?; self.params.p804 = value; self.mark_param_given(804); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ppowrat" => { validate_finite_parameter("PPOWRAT", value)?; self.params.p805 = value; self.mark_param_given(805); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prd" => { validate_finite_parameter("PRD", value)?; self.params.p806 = value; self.mark_param_given(806); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prd22" => { validate_finite_parameter("PRD22", value)?; self.params.p807 = value; self.mark_param_given(807); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prd23" => { validate_finite_parameter("PRD23", value)?; self.params.p808 = value; self.mark_param_given(808); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prd24" => { validate_finite_parameter("PRD24", value)?; self.params.p809 = value; self.mark_param_given(809); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdict1" => { validate_finite_parameter("PRDICT1", value)?; self.params.p810 = value; self.mark_param_given(810); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdov13" => { validate_finite_parameter("PRDOV13", value)?; self.params.p811 = value; self.mark_param_given(811); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdslp1" => { validate_finite_parameter("PRDSLP1", value)?; self.params.p812 = value; self.mark_param_given(812); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdvb" => { validate_finite_parameter("PRDVB", value)?; self.params.p813 = value; self.mark_param_given(813); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdvd" => { validate_finite_parameter("PRDVD", value)?; self.params.p814 = value; self.mark_param_given(814); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prdvg11" => { validate_finite_parameter("PRDVG11", value)?; self.params.p815 = value; self.mark_param_given(815); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prs" => { validate_finite_parameter("PRS", value)?; self.params.p816 = value; self.mark_param_given(816); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "prth0" => { validate_finite_parameter("PRTH0", value)?; self.params.p817 = value; self.mark_param_given(817); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvover" => { validate_finite_parameter("PVOVER", value)?; self.params.p818 = value; self.mark_param_given(818); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0" => { validate_finite_parameter("LJS0", value)?; self.params.p819 = value; self.mark_param_given(819); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0sw" => { validate_finite_parameter("LJS0SW", value)?; self.params.p820 = value; self.mark_param_given(820); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnj" => { validate_finite_parameter("LNJ", value)?; self.params.p821 = value; self.mark_param_given(821); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcisbk" => { validate_finite_parameter("LCISBK", value)?; self.params.p822 = value; self.mark_param_given(822); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvdiffj" => { validate_finite_parameter("LVDIFFJ", value)?; self.params.p823 = value; self.mark_param_given(823); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0d" => { validate_finite_parameter("LJS0D", value)?; self.params.p824 = value; self.mark_param_given(824); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0swd" => { validate_finite_parameter("LJS0SWD", value)?; self.params.p825 = value; self.mark_param_given(825); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnjd" => { validate_finite_parameter("LNJD", value)?; self.params.p826 = value; self.mark_param_given(826); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcisbkd" => { validate_finite_parameter("LCISBKD", value)?; self.params.p827 = value; self.mark_param_given(827); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvdiffjd" => { validate_finite_parameter("LVDIFFJD", value)?; self.params.p828 = value; self.mark_param_given(828); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0s" => { validate_finite_parameter("LJS0S", value)?; self.params.p829 = value; self.mark_param_given(829); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "ljs0sws" => { validate_finite_parameter("LJS0SWS", value)?; self.params.p830 = value; self.mark_param_given(830); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lnjs" => { validate_finite_parameter("LNJS", value)?; self.params.p831 = value; self.mark_param_given(831); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lcisbks" => { validate_finite_parameter("LCISBKS", value)?; self.params.p832 = value; self.mark_param_given(832); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "lvdiffjs" => { validate_finite_parameter("LVDIFFJS", value)?; self.params.p833 = value; self.mark_param_given(833); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0" => { validate_finite_parameter("WJS0", value)?; self.params.p834 = value; self.mark_param_given(834); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0sw" => { validate_finite_parameter("WJS0SW", value)?; self.params.p835 = value; self.mark_param_given(835); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnj" => { validate_finite_parameter("WNJ", value)?; self.params.p836 = value; self.mark_param_given(836); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcisbk" => { validate_finite_parameter("WCISBK", value)?; self.params.p837 = value; self.mark_param_given(837); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvdiffj" => { validate_finite_parameter("WVDIFFJ", value)?; self.params.p838 = value; self.mark_param_given(838); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0d" => { validate_finite_parameter("WJS0D", value)?; self.params.p839 = value; self.mark_param_given(839); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0swd" => { validate_finite_parameter("WJS0SWD", value)?; self.params.p840 = value; self.mark_param_given(840); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnjd" => { validate_finite_parameter("WNJD", value)?; self.params.p841 = value; self.mark_param_given(841); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcisbkd" => { validate_finite_parameter("WCISBKD", value)?; self.params.p842 = value; self.mark_param_given(842); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvdiffjd" => { validate_finite_parameter("WVDIFFJD", value)?; self.params.p843 = value; self.mark_param_given(843); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0s" => { validate_finite_parameter("WJS0S", value)?; self.params.p844 = value; self.mark_param_given(844); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wjs0sws" => { validate_finite_parameter("WJS0SWS", value)?; self.params.p845 = value; self.mark_param_given(845); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wnjs" => { validate_finite_parameter("WNJS", value)?; self.params.p846 = value; self.mark_param_given(846); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wcisbks" => { validate_finite_parameter("WCISBKS", value)?; self.params.p847 = value; self.mark_param_given(847); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "wvdiffjs" => { validate_finite_parameter("WVDIFFJS", value)?; self.params.p848 = value; self.mark_param_given(848); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0" => { validate_finite_parameter("PJS0", value)?; self.params.p849 = value; self.mark_param_given(849); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0sw" => { validate_finite_parameter("PJS0SW", value)?; self.params.p850 = value; self.mark_param_given(850); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnj" => { validate_finite_parameter("PNJ", value)?; self.params.p851 = value; self.mark_param_given(851); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcisbk" => { validate_finite_parameter("PCISBK", value)?; self.params.p852 = value; self.mark_param_given(852); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvdiffj" => { validate_finite_parameter("PVDIFFJ", value)?; self.params.p853 = value; self.mark_param_given(853); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0d" => { validate_finite_parameter("PJS0D", value)?; self.params.p854 = value; self.mark_param_given(854); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0swd" => { validate_finite_parameter("PJS0SWD", value)?; self.params.p855 = value; self.mark_param_given(855); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnjd" => { validate_finite_parameter("PNJD", value)?; self.params.p856 = value; self.mark_param_given(856); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcisbkd" => { validate_finite_parameter("PCISBKD", value)?; self.params.p857 = value; self.mark_param_given(857); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvdiffjd" => { validate_finite_parameter("PVDIFFJD", value)?; self.params.p858 = value; self.mark_param_given(858); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0s" => { validate_finite_parameter("PJS0S", value)?; self.params.p859 = value; self.mark_param_given(859); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pjs0sws" => { validate_finite_parameter("PJS0SWS", value)?; self.params.p860 = value; self.mark_param_given(860); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pnjs" => { validate_finite_parameter("PNJS", value)?; self.params.p861 = value; self.mark_param_given(861); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pcisbks" => { validate_finite_parameter("PCISBKS", value)?; self.params.p862 = value; self.mark_param_given(862); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            "pvdiffjs" => { validate_finite_parameter("PVDIFFJS", value)?; self.params.p863 = value; self.mark_param_given(863); self.recompute_instance_static(); self.invalidate_temperature_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'hisimhv_n4_va'", name)),
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
        let v4: f64 = p.p40;
        self.scalar_v4 = v4;
        let v5: f64 = p.p17;
        self.scalar_v5 = v5;
        let v6: f64 = p.p114;
        self.scalar_v6 = v6;
        let v7: f64 = p.p130;
        self.scalar_v7 = v7;
        let v8: f64 = p.p131;
        self.scalar_v8 = v8;
        let v9: f64 = p.p132;
        self.scalar_v9 = v9;
        let v11: f64 = p.p140;
        self.scalar_v11 = v11;
        let v12: f64 = p.p220;
        self.scalar_v12 = v12;
        let v15: f64 = p.p274;
        self.scalar_v15 = v15;
        let v17: f64 = p.p328;
        self.scalar_v17 = v17;
        let v18: f64 = p.p329;
        self.scalar_v18 = v18;
        let v19: f64 = p.p315;
        self.scalar_v19 = v19;
        let v20: f64 = p.p316;
        self.scalar_v20 = v20;
        let v21: f64 = p.p317;
        self.scalar_v21 = v21;
        let v22: f64 = p.p318;
        self.scalar_v22 = v22;
        let v23: f64 = p.p327;
        self.scalar_v23 = v23;
        let v24: f64 = p.p67;
        self.scalar_v24 = v24;
        let v25: f64 = p.p69;
        self.scalar_v25 = v25;
        let v26: f64 = p.p68;
        self.scalar_v26 = v26;
        let v27: f64 = p.p70;
        self.scalar_v27 = v27;
        let v28: f64 = p.p100;
        self.scalar_v28 = v28;
        let v29: f64 = p.p101;
        self.scalar_v29 = v29;
        let v31: f64 = p.p123;
        self.scalar_v31 = v31;
        let v32: f64 = p.p129;
        self.scalar_v32 = v32;
        let v33: f64 = f64::powf(100.0, p.p129);
        self.scalar_v33 = v33;
        let v34: f64 = (p.p123 / v33);
        self.scalar_v34 = v34;
        let v36: f64 = p.p293;
        self.scalar_v36 = v36;
        let v37: f64 = (100.0 * p.p293);
        self.scalar_v37 = v37;
        let v39: f64 = (p.p274 + 273.15);
        self.scalar_v39 = v39;
        let v40: f64 = p.p0;
        self.scalar_v40 = v40;
        let v41: f64 = p.p1;
        self.scalar_v41 = v41;
        let v42: f64 = p.p7;
        self.scalar_v42 = v42;
        let v43: f64 = (p.p1 / p.p7);
        self.scalar_v43 = v43;
        let v44: f64 = p.p116;
        self.scalar_v44 = v44;
        let v45: f64 = (p.p0 + p.p116);
        self.scalar_v45 = v45;
        let v46: f64 = p.p117;
        self.scalar_v46 = v46;
        let v47: f64 = (v43 + p.p117);
        self.scalar_v47 = v47;
        let v48: f64 = p.p6;
        self.scalar_v48 = v48;
        let v49: f64 = (v45 - p.p6);
        self.scalar_v49 = v49;
        let v50: f64 = p.p128;
        self.scalar_v50 = v50;
        let v51: f64 = (v47 + p.p128);
        self.scalar_v51 = v51;
        let v52: f64 = (1000000.0 * v45);
        self.scalar_v52 = v52;
        let v53: f64 = (1000000.0 * v47);
        self.scalar_v53 = v53;
        let v54: f64 = p.p553;
        self.scalar_v54 = v54;
        let v55: f64 = f64::powf(v52, p.p553);
        self.scalar_v55 = v55;
        let v56: f64 = p.p554;
        self.scalar_v56 = v56;
        let v57: f64 = f64::powf(v53, p.p554);
        self.scalar_v57 = v57;
        let v58: f64 = (v55 * v57);
        self.scalar_v58 = v58;
        let v59: f64 = p.p111;
        self.scalar_v59 = v59;
        let v60: f64 = p.p560;
        self.scalar_v60 = v60;
        let v61: f64 = (p.p560 / v55);
        self.scalar_v61 = v61;
        let v62: f64 = (p.p111 + v61);
        self.scalar_v62 = v62;
        let v63: f64 = p.p648;
        self.scalar_v63 = v63;
        let v64: f64 = (p.p648 / v57);
        self.scalar_v64 = v64;
        let v65: f64 = (v62 + v64);
        self.scalar_v65 = v65;
        let v66: f64 = p.p736;
        self.scalar_v66 = v66;
        let v67: f64 = (p.p736 / v58);
        self.scalar_v67 = v67;
        let v68: f64 = (v65 + v67);
        self.scalar_v68 = v68;
        let v69: f64 = p.p112;
        self.scalar_v69 = v69;
        let v70: f64 = p.p561;
        self.scalar_v70 = v70;
        let v71: f64 = (p.p561 / v55);
        self.scalar_v71 = v71;
        let v72: f64 = (p.p112 + v71);
        self.scalar_v72 = v72;
        let v73: f64 = p.p649;
        self.scalar_v73 = v73;
        let v74: f64 = (p.p649 / v57);
        self.scalar_v74 = v74;
        let v75: f64 = (v72 + v74);
        self.scalar_v75 = v75;
        let v76: f64 = p.p737;
        self.scalar_v76 = v76;
        let v77: f64 = (p.p737 / v58);
        self.scalar_v77 = v77;
        let v78: f64 = (v75 + v77);
        self.scalar_v78 = v78;
        let v79: f64 = p.p74;
        self.scalar_v79 = v79;
        let v80: f64 = p.p630;
        self.scalar_v80 = v80;
        let v81: f64 = (p.p630 / v55);
        self.scalar_v81 = v81;
        let v82: f64 = (p.p74 + v81);
        self.scalar_v82 = v82;
        let v83: f64 = p.p718;
        self.scalar_v83 = v83;
        let v84: f64 = (p.p718 / v57);
        self.scalar_v84 = v84;
        let v85: f64 = (v82 + v84);
        self.scalar_v85 = v85;
        let v86: f64 = p.p806;
        self.scalar_v86 = v86;
        let v87: f64 = (p.p806 / v58);
        self.scalar_v87 = v87;
        let v88: f64 = (v85 + v87);
        self.scalar_v88 = v88;
        let v89: f64 = p.p62;
        self.scalar_v89 = v89;
        let v90: f64 = p.p634;
        self.scalar_v90 = v90;
        let v91: f64 = (p.p634 / v55);
        self.scalar_v91 = v91;
        let v92: f64 = (p.p62 + v91);
        self.scalar_v92 = v92;
        let v93: f64 = p.p722;
        self.scalar_v93 = v93;
        let v94: f64 = (p.p722 / v57);
        self.scalar_v94 = v94;
        let v95: f64 = (v92 + v94);
        self.scalar_v95 = v95;
        let v96: f64 = p.p810;
        self.scalar_v96 = v96;
        let v97: f64 = (p.p810 / v58);
        self.scalar_v97 = v97;
        let v98: f64 = (v95 + v97);
        self.scalar_v98 = v98;
        let v99: f64 = p.p60;
        self.scalar_v99 = v99;
        let v100: f64 = p.p636;
        self.scalar_v100 = v100;
        let v101: f64 = (p.p636 / v55);
        self.scalar_v101 = v101;
        let v102: f64 = (p.p60 + v101);
        self.scalar_v102 = v102;
        let v103: f64 = p.p724;
        self.scalar_v103 = v103;
        let v104: f64 = (p.p724 / v57);
        self.scalar_v104 = v104;
        let v105: f64 = (v102 + v104);
        self.scalar_v105 = v105;
        let v106: f64 = p.p812;
        self.scalar_v106 = v106;
        let v107: f64 = (p.p812 / v58);
        self.scalar_v107 = v107;
        let v108: f64 = (v105 + v107);
        self.scalar_v108 = v108;
        let v109: f64 = p.p82;
        self.scalar_v109 = v109;
        let v110: f64 = p.p638;
        self.scalar_v110 = v110;
        let v111: f64 = (p.p638 / v55);
        self.scalar_v111 = v111;
        let v112: f64 = (p.p82 + v111);
        self.scalar_v112 = v112;
        let v113: f64 = p.p726;
        self.scalar_v113 = v113;
        let v114: f64 = (p.p726 / v57);
        self.scalar_v114 = v114;
        let v115: f64 = (v112 + v114);
        self.scalar_v115 = v115;
        let v116: f64 = p.p814;
        self.scalar_v116 = v116;
        let v117: f64 = (p.p814 / v58);
        self.scalar_v117 = v117;
        let v118: f64 = (v115 + v117);
        self.scalar_v118 = v118;
        let v119: f64 = p.p75;
        self.scalar_v119 = v119;
        let v120: f64 = p.p640;
        self.scalar_v120 = v120;
        let v121: f64 = (p.p640 / v55);
        self.scalar_v121 = v121;
        let v122: f64 = (p.p75 + v121);
        self.scalar_v122 = v122;
        let v123: f64 = p.p728;
        self.scalar_v123 = v123;
        let v124: f64 = (p.p728 / v57);
        self.scalar_v124 = v124;
        let v125: f64 = (v122 + v124);
        self.scalar_v125 = v125;
        let v126: f64 = p.p816;
        self.scalar_v126 = v126;
        let v127: f64 = (p.p816 / v58);
        self.scalar_v127 = v127;
        let v128: f64 = (v125 + v127);
        self.scalar_v128 = v128;
        let v129: f64 = p.p80;
        self.scalar_v129 = v129;
        let v130: f64 = p.p641;
        self.scalar_v130 = v130;
        let v131: f64 = (p.p641 / v55);
        self.scalar_v131 = v131;
        let v132: f64 = (p.p80 + v131);
        self.scalar_v132 = v132;
        let v133: f64 = p.p729;
        self.scalar_v133 = v133;
        let v134: f64 = (p.p729 / v57);
        self.scalar_v134 = v134;
        let v135: f64 = (v132 + v134);
        self.scalar_v135 = v135;
        let v136: f64 = p.p817;
        self.scalar_v136 = v136;
        let v137: f64 = (p.p817 / v58);
        self.scalar_v137 = v137;
        let v138: f64 = (v135 + v137);
        self.scalar_v138 = v138;
        let v139: bool = (1.0 == p.p40);
        self.scalar_v139 = v139;
        let v140: f64 = p.p19;
        self.scalar_v140 = v140;
        let v141: bool = (p.p19 > 0.0);
        self.scalar_v141 = v141;
        let v142: bool = (0.0 == v68);
        self.scalar_v142 = v142;
        let v143: bool = (v141 && v142);
        self.scalar_v143 = v143;
        let v144: f64 = p.p18;
        self.scalar_v144 = v144;
        let v145: bool = (p.p18 > 0.0);
        self.scalar_v145 = v145;
        let v146: bool = (0.0 == v78);
        self.scalar_v146 = v146;
        let v147: bool = (v145 && v146);
        self.scalar_v147 = v147;
        let v148: bool = (v143 || v147);
        self.scalar_v148 = v148;
        let v149: bool = (v139 && v148);
        self.scalar_v149 = v149;
        let v150: f64 = (if v149 { 0.0 } else { 0.0 });
        self.scalar_v150 = v150;
        let v151: bool = (!v149);
        self.scalar_v151 = v151;
        let v152: f64 = (if v151 { p.p40 } else { v150 });
        self.scalar_v152 = v152;
        let v153: bool = (1.0 == v152);
        self.scalar_v153 = v153;
        let v154: f64 = (if v141 { 1.0 } else { 0.0 });
        self.scalar_v154 = v154;
        let v155: f64 = (if v153 { v154 } else { 0.0 });
        self.scalar_v155 = v155;
        let v156: f64 = (if v145 { 1.0 } else { 0.0 });
        self.scalar_v156 = v156;
        let v157: f64 = (if v153 { v156 } else { 0.0 });
        self.scalar_v157 = v157;
        let v158: bool = (0.0 == p.p17);
        self.scalar_v158 = v158;
        let v159: bool = (p.p17 == 2.0);
        self.scalar_v159 = v159;
        let v160: bool = (v158 || v159);
        self.scalar_v160 = v160;
        let v161: bool = (!v153);
        self.scalar_v161 = v161;
        let v162: bool = (v160 && v161);
        self.scalar_v162 = v162;
        let v163: f64 = (if v162 { 0.0 } else { v155 });
        self.scalar_v163 = v163;
        let v164: f64 = (if v162 { 0.0 } else { v157 });
        self.scalar_v164 = v164;
        let v165: bool = (!v160);
        self.scalar_v165 = v165;
        let v166: bool = (v161 && v165);
        self.scalar_v166 = v166;
        let v167: f64 = p.p2;
        self.scalar_v167 = v167;
        let v168: f64 = (p.p130 * p.p2);
        self.scalar_v168 = v168;
        let v169: f64 = (p.p7 * v168);
        self.scalar_v169 = v169;
        let v170: f64 = (v88 + v118);
        self.scalar_v170 = v170;
        let v171: f64 = (p.p67 * v108);
        self.scalar_v171 = v171;
        let v172: f64 = (1000000.0 * v171);
        self.scalar_v172 = v172;
        let v173: f64 = (v98 + v172);
        self.scalar_v173 = v173;
        let v174: f64 = (v170 * v173);
        self.scalar_v174 = v174;
        let v175: f64 = (p.p68 * p.p100);
        self.scalar_v175 = v175;
        let v176: f64 = (1000000.0 * v175);
        self.scalar_v176 = v176;
        let v177: f64 = (p.p101 + v176);
        self.scalar_v177 = v177;
        let v178: f64 = (v174 * v177);
        self.scalar_v178 = v178;
        let v179: f64 = (v169 + v178);
        self.scalar_v179 = v179;
        let v180: f64 = (if v166 { v179 } else { 0.0 });
        self.scalar_v180 = v180;
        let v181: bool = (v180 > 0.0);
        self.scalar_v181 = v181;
        let v182: f64 = (if v181 { 1.0 } else { 0.0 });
        self.scalar_v182 = v182;
        let v183: f64 = (if v166 { v182 } else { v163 });
        self.scalar_v183 = v183;
        let v184: f64 = p.p3;
        self.scalar_v184 = v184;
        let v185: f64 = (p.p131 * p.p3);
        self.scalar_v185 = v185;
        let v186: f64 = (p.p7 * v185);
        self.scalar_v186 = v186;
        let v187: f64 = (p.p69 * v108);
        self.scalar_v187 = v187;
        let v188: f64 = (1000000.0 * v187);
        self.scalar_v188 = v188;
        let v189: f64 = (v98 + v188);
        self.scalar_v189 = v189;
        let v190: f64 = (v128 * v189);
        self.scalar_v190 = v190;
        let v191: f64 = (p.p70 * p.p100);
        self.scalar_v191 = v191;
        let v192: f64 = (1000000.0 * v191);
        self.scalar_v192 = v192;
        let v193: f64 = (p.p101 + v192);
        self.scalar_v193 = v193;
        let v194: f64 = (v190 * v193);
        self.scalar_v194 = v194;
        let v195: f64 = (v186 + v194);
        self.scalar_v195 = v195;
        let v196: f64 = (if v166 { v195 } else { v180 });
        self.scalar_v196 = v196;
        let v197: bool = (v196 > 0.0);
        self.scalar_v197 = v197;
        let v198: f64 = (if v197 { 1.0 } else { 0.0 });
        self.scalar_v198 = v198;
        let v199: f64 = (if v166 { v198 } else { v164 });
        self.scalar_v199 = v199;
        let v200: f64 = (v138 / 100.0);
        self.scalar_v200 = v200;
        let v201: f64 = p.p28;
        self.scalar_v201 = v201;
        let v202: f64 = f64::powf(v51, p.p129);
        self.scalar_v202 = v202;
        let v203: f64 = (v34 / v202);
        self.scalar_v203 = v203;
        let v204: f64 = (p.p114 + v203);
        self.scalar_v204 = v204;
        let v205: f64 = (2.0 * v204);
        self.scalar_v205 = v205;
        let v206: f64 = (v47 - v205);
        self.scalar_v206 = v206;
        let v207: f64 = (p.p7 * v206);
        self.scalar_v207 = v207;
        let v208: f64 = (0.5 * v45);
        self.scalar_v208 = v208;
        let v209: f64 = (v208 - p.p140);
        self.scalar_v209 = v209;
        let v210: f64 = (v209 - 1e-9);
        self.scalar_v210 = v210;
        let v212: f64 = (v210 - 1e-10);
        self.scalar_v212 = v212;
        let v214: f64 = (v212 * v212);
        self.scalar_v214 = v214;
        let v215: f64 = (4.0000000000000004e-19 + v214);
        self.scalar_v215 = v215;
        let v216: f64 = ((v215) as f64).sqrt();
        self.scalar_v216 = v216;
        let v217: f64 = (v212 + v216);
        self.scalar_v217 = v217;
        let v218: f64 = (0.5 * v217);
        self.scalar_v218 = v218;
        let v219: f64 = (1e-9 + v218);
        self.scalar_v219 = v219;
        let v220: f64 = (1.0 / v219);
        self.scalar_v220 = v220;
        let v221: f64 = (1.0 / p.p220);
        self.scalar_v221 = v221;
        let v222: f64 = (v220 + v221);
        self.scalar_v222 = v222;
        let v223: f64 = (1.0 / v222);
        self.scalar_v223 = v223;
        let v224: bool = (0.0 >= v223);
        self.scalar_v224 = v224;
        let v225: f64 = (if v224 { 0.0 } else { v223 });
        self.scalar_v225 = v225;
        let v226: f64 = p.p51;
        self.scalar_v226 = v226;
        let v227: bool = (1.0 == p.p51);
        self.scalar_v227 = v227;
        let v228: f64 = p.p5;
        self.scalar_v228 = v228;
        let v229: f64 = p.p4;
        self.scalar_v229 = v229;
        let v230: f64 = (3.0 * p.p4);
        self.scalar_v230 = v230;
        let v231: f64 = (v206 / v230);
        self.scalar_v231 = v231;
        let v232: f64 = (p.p5 + v231);
        self.scalar_v232 = v232;
        let v233: f64 = (if v227 { v232 } else { v223 });
        self.scalar_v233 = v233;
        let v234: f64 = (if v227 { v49 } else { v225 });
        self.scalar_v234 = v234;
        let v235: f64 = (p.p132 * v233);
        self.scalar_v235 = v235;
        let v236: f64 = (p.p4 * v234);
        self.scalar_v236 = v236;
        let v237: f64 = (p.p7 * v236);
        self.scalar_v237 = v237;
        let v238: f64 = (v235 / v237);
        self.scalar_v238 = v238;
        let v239: f64 = (if v227 { v238 } else { 0.0 });
        self.scalar_v239 = v239;
        let v240: bool = (v239 > 0.001);
        self.scalar_v240 = v240;
        let v241: bool = (v227 && v240);
        self.scalar_v241 = v241;
        let v242: f64 = (1.0 / v239);
        self.scalar_v242 = v242;
        let v243: f64 = (if v241 { v242 } else { v239 });
        self.scalar_v243 = v243;
        let v244: bool = (!v240);
        self.scalar_v244 = v244;
        let v245: bool = (v227 && v244);
        self.scalar_v245 = v245;
        let v247: f64 = (if v245 { 1000.0 } else { v243 });
        self.scalar_v247 = v247;
        let v248: bool = (!v227);
        self.scalar_v248 = v248;
        let v249: f64 = p.p444;
        self.scalar_v249 = v249;
        let v250: f64 = (1.0 / p.p444);
        self.scalar_v250 = v250;
        let v251: f64 = (if v248 { v250 } else { v247 });
        self.scalar_v251 = v251;
        let v252: f64 = p.p52;
        self.scalar_v252 = v252;
        let v253: bool = (1.0 == p.p52);
        self.scalar_v253 = v253;
        let v254: f64 = p.p56;
        self.scalar_v254 = v254;
        let v255: bool = (p.p56 < 0.001);
        self.scalar_v255 = v255;
        let v256: bool = (v253 && v255);
        self.scalar_v256 = v256;
        let v257: f64 = (if v256 { 1000.0 } else { 0.0 });
        self.scalar_v257 = v257;
        let v258: bool = (!v255);
        self.scalar_v258 = v258;
        let v259: bool = (v253 && v258);
        self.scalar_v259 = v259;
        let v260: f64 = p.p277;
        self.scalar_v260 = v260;
        let v261: f64 = (1.0 / p.p56);
        self.scalar_v261 = v261;
        let v262: f64 = (p.p277 + v261);
        self.scalar_v262 = v262;
        let v263: f64 = (if v259 { v262 } else { v257 });
        self.scalar_v263 = v263;
        let v264: f64 = p.p58;
        self.scalar_v264 = v264;
        let v265: bool = (p.p58 < 0.001);
        self.scalar_v265 = v265;
        let v266: bool = (v253 && v265);
        self.scalar_v266 = v266;
        let v267: f64 = (if v266 { 1000.0 } else { 0.0 });
        self.scalar_v267 = v267;
        let v268: bool = (!v265);
        self.scalar_v268 = v268;
        let v269: bool = (v253 && v268);
        self.scalar_v269 = v269;
        let v270: f64 = (1.0 / p.p58);
        self.scalar_v270 = v270;
        let v271: f64 = (p.p277 + v270);
        self.scalar_v271 = v271;
        let v272: f64 = (if v269 { v271 } else { v267 });
        self.scalar_v272 = v272;
        let v273: f64 = p.p57;
        self.scalar_v273 = v273;
        let v274: bool = (p.p57 < 0.001);
        self.scalar_v274 = v274;
        let v275: bool = (v253 && v274);
        self.scalar_v275 = v275;
        let v276: f64 = (if v275 { 1000.0 } else { 0.0 });
        self.scalar_v276 = v276;
        let v277: bool = (!v274);
        self.scalar_v277 = v277;
        let v278: bool = (v253 && v277);
        self.scalar_v278 = v278;
        let v279: f64 = (1.0 / p.p57);
        self.scalar_v279 = v279;
        let v280: f64 = (p.p277 + v279);
        self.scalar_v280 = v280;
        let v281: f64 = (if v278 { v280 } else { v276 });
        self.scalar_v281 = v281;
        let v282: bool = (!v253);
        self.scalar_v282 = v282;
        let v283: f64 = (if v282 { 1000.0 } else { v263 });
        self.scalar_v283 = v283;
        let v284: f64 = (if v282 { v283 } else { v272 });
        self.scalar_v284 = v284;
        let v285: f64 = (if v282 { v283 } else { v281 });
        self.scalar_v285 = v285;
        let v286: f64 = (v200 / v207);
        self.scalar_v286 = v286;
        let v287: f64 = f64::powf(v52, p.p318);
        self.scalar_v287 = v287;
        let v288: f64 = (p.p317 / v287);
        self.scalar_v288 = v288;
        let v289: f64 = (1.0 + v288);
        self.scalar_v289 = v289;
        let v290: f64 = (v286 * v289);
        self.scalar_v290 = v290;
        let v291: f64 = f64::powf(v53, p.p316);
        self.scalar_v291 = v291;
        let v292: f64 = (p.p315 / v291);
        self.scalar_v292 = v292;
        let v293: f64 = (1.0 + v292);
        self.scalar_v293 = v293;
        let v294: f64 = (v290 * v293);
        self.scalar_v294 = v294;
        let v295: f64 = (v37 * v207);
        self.scalar_v295 = v295;
        let v296: f64 = f64::powf(p.p7, p.p327);
        self.scalar_v296 = v296;
        let v297: f64 = (1.0 / v296);
        self.scalar_v297 = v297;
        let v298: f64 = (v294 * v297);
        self.scalar_v298 = v298;
        let v299: f64 = (v297 / v207);
        self.scalar_v299 = v299;
        let v300: f64 = (v289 * v299);
        self.scalar_v300 = v300;
        let v301: f64 = (v293 * v300);
        self.scalar_v301 = v301;
        let v302: f64 = p.p53;
        self.scalar_v302 = v302;
        let v303: bool = (0.0 == p.p53);
        self.scalar_v303 = v303;
        let v304: bool = (0.0 == v200);
        self.scalar_v304 = v304;
        let v305: bool = (v303 || v304);
        self.scalar_v305 = v305;
        let v307: f64 = p.p11;
        self.scalar_v307 = v307;
        let v315: f64 = (v39 * v39);
        self.scalar_v315 = v315;
        let v318: bool = ((p.p53 != 0.0) && v305);
        self.scalar_v318 = v318;
        let v333: bool = (!(p.p28 != 0.0));
        self.scalar_v333 = v333;
        let v334: bool = (p.p53 > 0.0);
        self.scalar_v334 = v334;
        let v335: bool = (0.0 != v200);
        self.scalar_v335 = v335;
        let v336: bool = (v334 && v335);
        self.scalar_v336 = v336;
        let v345: bool = ((p.p53 != 0.0) && v336);
        self.scalar_v345 = v345;
        let v355: f64 = p.p539;
        self.scalar_v355 = v355;
        let v356: bool = (p.p539 > 0.0);
        self.scalar_v356 = v356;
        let v357: f64 = p.p543;
        self.scalar_v357 = v357;
        let v358: bool = (p.p543 > 0.0);
        self.scalar_v358 = v358;
        let v359: bool = (v356 && v358);
        self.scalar_v359 = v359;
        let v360: f64 = p.p546;
        self.scalar_v360 = v360;
        let v361: bool = (p.p546 > 0.0);
        self.scalar_v361 = v361;
        let v362: bool = (v356 && v361);
        self.scalar_v362 = v362;
        let v363: f64 = p.p29;
        self.scalar_v363 = v363;
        let v364: bool = (!(p.p29 != 0.0));
        self.scalar_v364 = v364;
        let v372: bool = (!v334);
        self.scalar_v372 = v372;
        let v374: bool = (p.p132 > 0.0);
        self.scalar_v374 = v374;
        let v375: bool = (v227 && v374);
        self.scalar_v375 = v375;
        let v376: f64 = (if v372 { 0.0 } else { v295 });
        self.scalar_v376 = v376;
        let v377: bool = (!v359);
        self.scalar_v377 = v377;
        let v378: f64 = (if v377 { 0.0 } else { 0.0 });
        self.scalar_v378 = v378;
        let v379: bool = (!v362);
        self.scalar_v379 = v379;
        let v380: f64 = (if v379 { 0.0 } else { 0.0 });
        self.scalar_v380 = v380;
        let v381: bool = (!(v183 != 0.0));
        self.scalar_v381 = v381;
        let v382: f64 = (if v381 { 0.0 } else { 0.0 });
        self.scalar_v382 = v382;
        let v383: bool = (!(v199 != 0.0));
        self.scalar_v383 = v383;
        let v384: f64 = (if v383 { 0.0 } else { 0.0 });
        self.scalar_v384 = v384;
        let v390: bool = (!v375);
        self.scalar_v390 = v390;
        let v391: f64 = (if v390 { 0.0 } else { 0.0 });
        self.scalar_v391 = v391;
        let v402: bool = (!(p.p52 != 0.0));
        self.scalar_v402 = v402;
        let v403: f64 = (if v402 { 0.0 } else { 0.0 });
        self.scalar_v403 = v403;
        let v409: f64 = (if v333 { 0.0 } else { 0.0 });
        self.scalar_v409 = v409;
        let v410: f64 = (if v364 { 0.0 } else { 0.0 });
        self.scalar_v410 = v410;
        let v411: f64 = (-v251);
        self.scalar_v411 = v411;
        let v412: f64 = (if v375 { v251 } else { 0.0 });
        self.scalar_v412 = v412;
        let v413: f64 = (if v375 { v411 } else { 0.0 });
        self.scalar_v413 = v413;
        let v414: f64 = (-v284);
        self.scalar_v414 = v414;
        let v415: f64 = (if (p.p52 != 0.0) { v414 } else { 0.0 });
        self.scalar_v415 = v415;
        let v416: f64 = (if (p.p52 != 0.0) { v284 } else { 0.0 });
        self.scalar_v416 = v416;
        let v417: f64 = (-v285);
        self.scalar_v417 = v417;
        let v418: f64 = (if (p.p52 != 0.0) { v417 } else { 0.0 });
        self.scalar_v418 = v418;
        let v419: f64 = (if (p.p52 != 0.0) { v285 } else { 0.0 });
        self.scalar_v419 = v419;
        let v420: f64 = (-v283);
        self.scalar_v420 = v420;
        let v421: f64 = (if (p.p52 != 0.0) { v283 } else { 0.0 });
        self.scalar_v421 = v421;
        let v422: f64 = (if (p.p52 != 0.0) { v420 } else { 0.0 });
        self.scalar_v422 = v422;
        let v424: f64 = (if v372 { 10000.0 } else { 0.0 });
        self.scalar_v424 = v424;
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
        let v308: f64 = (temperature + self.scalar_v307);
        self.scalar_v308 = v308;
        let v309: f64 = (if self.scalar_v305 { self.scalar_v308 } else { 0.0 });
        self.scalar_v309 = v309;
        let v310: f64 = (if self.scalar_v305 { self.scalar_v309 } else { 0.0 });
        self.scalar_v310 = v310;
        let v311: f64 = (if self.scalar_v305 { self.scalar_v309 } else { self.scalar_v309 });
        self.scalar_v311 = v311;
        let v312: f64 = (self.scalar_v310 - self.scalar_v39);
        self.scalar_v312 = v312;
        let v313: f64 = (if self.scalar_v305 { self.scalar_v312 } else { 0.0 });
        self.scalar_v313 = v313;
        let v314: f64 = (self.scalar_v310 * self.scalar_v310);
        self.scalar_v314 = v314;
        let v316: f64 = (self.scalar_v314 - self.scalar_v315);
        self.scalar_v316 = v316;
        let v317: f64 = (if self.scalar_v305 { self.scalar_v316 } else { 0.0 });
        self.scalar_v317 = v317;
        let v319: f64 = (self.scalar_v17 * self.scalar_v313);
        self.scalar_v319 = v319;
        let v320: f64 = (self.scalar_v200 + self.scalar_v319);
        self.scalar_v320 = v320;
        let v321: f64 = (self.scalar_v18 * self.scalar_v317);
        self.scalar_v321 = v321;
        let v322: f64 = (self.scalar_v320 + self.scalar_v321);
        self.scalar_v322 = v322;
        let v323: f64 = (self.scalar_v301 * self.scalar_v322);
        self.scalar_v323 = v323;
        let v324: f64 = (if self.scalar_v318 { self.scalar_v323 } else { self.scalar_v298 });
        self.scalar_v324 = v324;
        let v326: bool = (self.scalar_v324 < 0.0001);
        self.scalar_v326 = v326;
        let v327: bool = (self.scalar_v318 && self.scalar_v326);
        self.scalar_v327 = v327;
        let v328: f64 = (if self.scalar_v327 { 0.0001 } else { self.scalar_v324 });
        self.scalar_v328 = v328;
        let v338: f64 = (if self.scalar_v336 { self.scalar_v308 } else { self.scalar_v311 });
        self.scalar_v338 = v338;
        let v339: f64 = (if self.scalar_v336 { self.scalar_v338 } else { self.scalar_v310 });
        self.scalar_v339 = v339;
        let v340: f64 = (self.scalar_v339 - self.scalar_v39);
        self.scalar_v340 = v340;
        let v341: f64 = (if self.scalar_v336 { self.scalar_v340 } else { self.scalar_v313 });
        self.scalar_v341 = v341;
        let v342: f64 = (self.scalar_v339 * self.scalar_v339);
        self.scalar_v342 = v342;
        let v343: f64 = (self.scalar_v342 - self.scalar_v315);
        self.scalar_v343 = v343;
        let v344: f64 = (if self.scalar_v336 { self.scalar_v343 } else { self.scalar_v317 });
        self.scalar_v344 = v344;
        let v346: f64 = (self.scalar_v17 * self.scalar_v341);
        self.scalar_v346 = v346;
        let v347: f64 = (self.scalar_v200 + self.scalar_v346);
        self.scalar_v347 = v347;
        let v348: f64 = (self.scalar_v18 * self.scalar_v344);
        self.scalar_v348 = v348;
        let v349: f64 = (self.scalar_v347 + self.scalar_v348);
        self.scalar_v349 = v349;
        let v350: f64 = (self.scalar_v301 * self.scalar_v349);
        self.scalar_v350 = v350;
        let v351: f64 = (if self.scalar_v345 { self.scalar_v350 } else { self.scalar_v328 });
        self.scalar_v351 = v351;
        let v352: bool = (self.scalar_v351 < 0.0001);
        self.scalar_v352 = v352;
        let v353: bool = (self.scalar_v345 && self.scalar_v352);
        self.scalar_v353 = v353;
        let v354: f64 = (if self.scalar_v353 { 0.0001 } else { self.scalar_v351 });
        self.scalar_v354 = v354;
        let v365: bool = (self.scalar_v354 > 0.0001);
        self.scalar_v365 = v365;
        let v366: bool = (self.scalar_v334 && self.scalar_v365);
        self.scalar_v366 = v366;
        let v367: f64 = (1.0 / self.scalar_v354);
        self.scalar_v367 = v367;
        let v368: f64 = (if self.scalar_v366 { self.scalar_v367 } else { 0.0 });
        self.scalar_v368 = v368;
        let v369: bool = (!self.scalar_v365);
        self.scalar_v369 = v369;
        let v370: bool = (self.scalar_v334 && self.scalar_v369);
        self.scalar_v370 = v370;
        let v371: f64 = (if self.scalar_v370 { 10000.0 } else { self.scalar_v368 });
        self.scalar_v371 = v371;
        let v373: f64 = (if self.scalar_v372 { 0.0 } else { self.scalar_v371 });
        self.scalar_v373 = v373;
        let v423: f64 = (if self.scalar_v334 { self.scalar_v373 } else { 0.0 });
        self.scalar_v423 = v423;
        self.scalar_temperature_static_temperature = temperature;
        self.scalar_temperature_static_thermal_voltage = thermal_voltage;
        self.scalar_temperature_static_valid = true;
    }
}
