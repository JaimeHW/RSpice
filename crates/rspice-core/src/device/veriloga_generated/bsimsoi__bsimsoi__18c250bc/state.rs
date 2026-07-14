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
    pub p920: f64, pub p921: f64, pub p922: f64, pub p923: f64, pub p924: f64, pub p925: f64, pub p926: f64, pub p927: f64,
    pub p928: f64, pub p929: f64, pub p930: f64, pub p931: f64, pub p932: f64, pub p933: f64, pub p934: f64, pub p935: f64,
    pub p936: f64, pub p937: f64, pub p938: f64, pub p939: f64, pub p940: f64, pub p941: f64, pub p942: f64, pub p943: f64,
    pub p944: f64, pub p945: f64, pub p946: f64, pub p947: f64, pub p948: f64, pub p949: f64, pub p950: f64, pub p951: f64,
    pub p952: f64, pub p953: f64, pub p954: f64, pub p955: f64, pub p956: f64, pub p957: f64, pub p958: f64, pub p959: f64,
    pub p960: f64, pub p961: f64, pub p962: f64, pub p963: f64, pub p964: f64, pub p965: f64, pub p966: f64, pub p967: f64,
    pub p968: f64, pub p969: f64, pub p970: f64, pub p971: f64, pub p972: f64, pub p973: f64, pub p974: f64, pub p975: f64,
    pub p976: f64, pub p977: f64, pub p978: f64, pub p979: f64, pub p980: f64, pub p981: f64, pub p982: f64, pub p983: f64,
    pub p984: f64, pub p985: f64, pub p986: f64, pub p987: f64, pub p988: f64, pub p989: f64, pub p990: f64, pub p991: f64,
    pub p992: f64, pub p993: f64, pub p994: f64, pub p995: f64, pub p996: f64, pub p997: f64, pub p998: f64, pub p999: f64,
    pub p1000: f64, pub p1001: f64, pub p1002: f64, pub p1003: f64, pub p1004: f64, pub p1005: f64, pub p1006: f64, pub p1007: f64,
    pub p1008: f64, pub p1009: f64, pub p1010: f64, pub p1011: f64, pub p1012: f64, pub p1013: f64, pub p1014: f64, pub p1015: f64,
    pub p1016: f64, pub p1017: f64, pub p1018: f64, pub p1019: f64, pub p1020: f64, pub p1021: f64, pub p1022: f64, pub p1023: f64,
    pub p1024: f64, pub p1025: f64, pub p1026: f64, pub p1027: f64, pub p1028: f64, pub p1029: f64, pub p1030: f64, pub p1031: f64,
    pub p1032: f64, pub p1033: f64, pub p1034: f64, pub p1035: f64, pub p1036: f64, pub p1037: f64, pub p1038: f64, pub p1039: f64,
    pub p1040: f64, pub p1041: f64, pub p1042: f64, pub p1043: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 28] = [
                0.0, 5e-6, 5e-6, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1e-5,
                1.0, 1.0, 50.0, 50.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 28);
            {
                let params = &mut *ptr;
                params.p28 = params.p26;
                validate_parameter("AGBCPD", params.p28, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 5] = [
                0.0, 1.0, 1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(29), 5);
            {
                let params = &mut *ptr;
                params.p34 = params.p32;
                validate_parameter("MULT_FN", params.p34, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 7] = [
                0.0, 0.0, 1.0, 4.7, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(35), 7);
            {
                let params = &mut *ptr;
                params.p42 = if (params.p38 >= 4.2) { 1.0 } else { 0.0 };
                validate_parameter("VGSTCVMOD", params.p42, true, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 13] = [
                0.0, 0.0, 1e-8, 3.9, 11.7, 14500000000.0, 1.16, 0.000702,
                1108.0, 4.05, 4.05, 1.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(43), 13);
            {
                let params = &mut *ptr;
                params.p56 = if (params.p37 == 1.0) { 1.5 } else { (-1.5) };
                validate_finite_parameter("VDDEOT", params.p56).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 10] = [
                300.15, 1.0, 1.0, 11.7, 2.0, 1.0, 0.0, 1.0,
                1.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(57), 10);
            {
                let params = &mut *ptr;
                params.p67 = params.p66;
                validate_parameter("TOXM", params.p67, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 38] = [
                0.0, 0.00024, 0.0, 0.0, 0.0, 1.0, 80000.0, 33000.0,
                1.0, 0.0, 0.0, 1.0, -0.6, 6e16, 1.7e17, 0.0,
                1e20, 0.0, 0.0, 0.0, -3.0, 1.55e-7, 0.53, -0.11,
                0.0, 0.022, -0.0186, 0.0, 0.0, 2.5e-6, 0.0, 2.2,
                0.53, -0.032, 0.0, 5300000.0, -0.032, 0.56,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(68), 38);
            {
                let params = &mut *ptr;
                params.p106 = params.p105;
                validate_finite_parameter("DSUB", params.p106).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p107 = if (params.p37 == 1.0) { 0.7 } else { (-0.7) };
                validate_finite_parameter("VTHO", params.p107).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p108 = params.p107;
                validate_finite_parameter("VTH0", params.p108).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 5] = [
                -1.0, 2.25e-9, 4.31e-9, 5.87e-19, -7.61e-18,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(109), 5);
            {
                let params = &mut *ptr;
                params.p114 = if (params.p62 == 3.0) { (-0.0465) } else { (-4.65e-11) };
                validate_finite_parameter("UC", params.p114).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p115 = if (params.p62 == 3.0) { (-0.056) } else { (-5.6e-11) };
                validate_finite_parameter("UC1", params.p115).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p116 = if (params.p37 == 1.0) { 0.067 } else { 0.025 };
                validate_finite_parameter("U0", params.p116).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p117 = if (params.p37 == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("EU", params.p117).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 1] = [
                -1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(118), 1);
            {
                let params = &mut *ptr;
                params.p119 = if (params.p37 == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("UCS", params.p119).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 27] = [
                -0.004775, 0.0, 0.0, 0.0, 0.0, -0.08, 27.0, 0.0,
                0.0, 0.0, 0.01, 0.0, 100.0, 50.0, 50.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.08, -0.07,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(120), 27);
            {
                let params = &mut *ptr;
                params.p147 = params.p145;
                validate_finite_parameter("ETA0CV", params.p147).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p148 = params.p146;
                validate_finite_parameter("ETABCV", params.p148).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 8] = [
                1.3, 0.39, 0.0086, 0.0, 0.0, 3e-7, 1e-7, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(149), 8);
            {
                let params = &mut *ptr;
                params.p157 = params.p155;
                validate_parameter("XJ", params.p157, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 7] = [
                0.0, 2300000000.0, 0.0, 0.5, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(158), 7);
            {
                let params = &mut *ptr;
                params.p165 = params.p158;
                validate_finite_parameter("AGISL", params.p165).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p166 = params.p159;
                validate_finite_parameter("BGISL", params.p166).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p167 = params.p160;
                validate_finite_parameter("BGISL1", params.p167).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p168 = params.p161;
                validate_finite_parameter("CGISL", params.p168).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p169 = params.p162;
                validate_finite_parameter("RGISL", params.p169).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p170 = params.p163;
                validate_finite_parameter("KGISL", params.p170).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p171 = params.p164;
                validate_finite_parameter("FGISL", params.p171).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(172), 1);
            {
                let params = &mut *ptr;
                params.p173 = params.p172;
                validate_parameter("NDIODED", params.p173, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(174), 1);
            {
                let params = &mut *ptr;
                params.p175 = params.p174;
                validate_finite_parameter("XDIF", params.p175).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(176), 2);
            {
                let params = &mut *ptr;
                params.p178 = params.p175;
                validate_finite_parameter("XDIFD", params.p178).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p179 = params.p176;
                validate_finite_parameter("XRECD", params.p179).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p180 = params.p177;
                validate_finite_parameter("XTUND", params.p180).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(181), 1);
            {
                let params = &mut *ptr;
                params.p182 = params.p181;
                validate_parameter("PBSWGD", params.p182, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(183), 1);
            {
                let params = &mut *ptr;
                params.p184 = params.p183;
                validate_finite_parameter("MJSWGD", params.p184).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 1] = [
                1e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(185), 1);
            {
                let params = &mut *ptr;
                params.p186 = params.p185;
                validate_parameter("CJSWGD", params.p186, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 29] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.6, 0.0, 1e-8, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(187), 29);
            {
                let params = &mut *ptr;
                params.p216 = params.p197;
                validate_finite_parameter("DWC", params.p216).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p217 = params.p187;
                validate_finite_parameter("DLC", params.p217).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(218), 1);
            {
                let params = &mut *ptr;
                params.p219 = if (params.p37 == 1.0) { 6.25e41 } else { 6.188e40 };
                validate_finite_parameter("NOIA", params.p219).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p220 = if (params.p37 == 1.0) { 3.125e26 } else { 1.5e25 };
                validate_finite_parameter("NOIB", params.p220).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 32] = [
                8750000000.0, 1.0, 0.0, 3.5, 0.395, 100000.0, 1.5, 3.5,
                0.577, 0.37, 1.0, 1e-6, 1e-6, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(221), 32);
            {
                let params = &mut *ptr;
                params.p253 = params.p251;
                validate_finite_parameter("STETA0CV", params.p253).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p254 = params.p252;
                validate_finite_parameter("LODETA0CV", params.p254).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 68] = [
                1e-12, 2.0, 1e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1e-20, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                41000000.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.1, 0.9, 0.0, 0.0, 0.5,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 10000000.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(255), 68);
            {
                let params = &mut *ptr;
                params.p323 = params.p322;
                validate_parameter("NTUND", params.p323, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(324), 1);
            {
                let params = &mut *ptr;
                params.p325 = params.p324;
                validate_parameter("NRECF0D", params.p325, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(326), 1);
            {
                let params = &mut *ptr;
                params.p327 = params.p326;
                validate_parameter("NRECR0D", params.p327, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 1] = [
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(328), 1);
            {
                let params = &mut *ptr;
                params.p329 = params.p328;
                validate_parameter("IDBJT", params.p329, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(330), 1);
            {
                let params = &mut *ptr;
                params.p331 = params.p330;
                validate_parameter("IDDIF", params.p331, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(332), 1);
            {
                let params = &mut *ptr;
                params.p333 = params.p332;
                validate_parameter("IDREC", params.p333, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(334), 1);
            {
                let params = &mut *ptr;
                params.p335 = params.p334;
                validate_parameter("IDTUN", params.p335, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 2] = [
                2e-6, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(336), 2);
            {
                let params = &mut *ptr;
                params.p338 = params.p337;
                validate_finite_parameter("VREC0D", params.p338).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(339), 1);
            {
                let params = &mut *ptr;
                params.p340 = params.p339;
                validate_finite_parameter("VTUN0D", params.p340).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 6] = [
                1.0, 2e-7, 1.0, 10.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(341), 6);
            {
                let params = &mut *ptr;
                params.p347 = params.p346;
                validate_finite_parameter("AHLID", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 16] = [
                0.0, 0.0, 0.0, 1e-12, -1.0, 0.0, 0.0, 0.0,
                0.3, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(348), 16);
            {
                let params = &mut *ptr;
                params.p364 = params.p362;
                validate_finite_parameter("TCJSWGD", params.p364).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p365 = params.p363;
                validate_finite_parameter("TPBSWGD", params.p365).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 3] = [
                1.0, 15.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(366), 3);
            {
                let params = &mut *ptr;
                params.p369 = params.p368;
                validate_parameter("NOFF2", params.p369, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 6] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(370), 6);
            {
                let params = &mut *ptr;
                params.p376 = params.p66;
                validate_parameter("TOXQM", params.p376, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 21] = [
                0.0, 1000000000000000.0, 1.0, 2.5e-9, 1.2, 0.075, 0.35, 0.0,
                0.03, 300.0, 0.026, 0.43, 0.0, 0.05, 17.0, 0.043,
                0.0, 0.0054, 0.0075, 5.0, 0.005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(377), 21);
            {
                let params = &mut *ptr;
                params.p398 = if (params.p37 == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGC", params.p398).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(399), 1);
            {
                let params = &mut *ptr;
                params.p400 = if (params.p37 == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGC", params.p400).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p401 = if (params.p37 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params.p401).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p402 = if (params.p37 == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGSD", params.p402).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(403), 1);
            {
                let params = &mut *ptr;
                params.p404 = if (params.p37 == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGSD", params.p404).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p405 = if (params.p37 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGSD", params.p405).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 4] = [
                1.0, 1.0, 1.0, 2.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(406), 4);
            {
                let params = &mut *ptr;
                params.p410 = params.p187;
                validate_finite_parameter("DLCIG", params.p410).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 20] = [
                0.0, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1000.0, 12.0, 1.0, 0.1, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(411), 20);
            {
                let params = &mut *ptr;
                params.p431 = 0.001;
                validate_parameter("MINR", params.p431, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 41] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(432), 41);
            {
                let params = &mut *ptr;
                params.p473 = params.p470;
                validate_finite_parameter("LXDIFD", params.p473).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p474 = params.p471;
                validate_finite_parameter("LXRECD", params.p474).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p475 = params.p472;
                validate_finite_parameter("LXTUND", params.p475).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(476), 64);
            {
                let params = &mut *ptr;
                params.p540 = params.p538;
                validate_finite_parameter("LETA0CV", params.p540).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p541 = params.p539;
                validate_finite_parameter("LETABCV", params.p541).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(542), 36);
            {
                let params = &mut *ptr;
                params.p578 = params.p571;
                validate_finite_parameter("LAGISL", params.p578).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p579 = params.p572;
                validate_finite_parameter("LBGISL", params.p579).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p580 = params.p573;
                validate_finite_parameter("LBGISL1", params.p580).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p581 = params.p574;
                validate_finite_parameter("LCGISL", params.p581).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p582 = params.p575;
                validate_finite_parameter("LRGISL", params.p582).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p583 = params.p576;
                validate_finite_parameter("LKGISL", params.p583).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p584 = params.p577;
                validate_finite_parameter("LFGISL", params.p584).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(585), 1);
            {
                let params = &mut *ptr;
                params.p586 = params.p585;
                validate_finite_parameter("LNTUND", params.p586).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (ptr as *mut f64).add(587), 1);
            {
                let params = &mut *ptr;
                params.p588 = params.p587;
                validate_finite_parameter("LNDIODED", params.p588).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (ptr as *mut f64).add(589), 1);
            {
                let params = &mut *ptr;
                params.p590 = params.p589;
                validate_finite_parameter("LNRECF0D", params.p590).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (ptr as *mut f64).add(591), 1);
            {
                let params = &mut *ptr;
                params.p592 = params.p591;
                validate_finite_parameter("LNRECR0D", params.p592).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (ptr as *mut f64).add(593), 1);
            {
                let params = &mut *ptr;
                params.p594 = params.p593;
                validate_finite_parameter("LIDBJT", params.p594).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (ptr as *mut f64).add(595), 1);
            {
                let params = &mut *ptr;
                params.p596 = params.p595;
                validate_finite_parameter("LIDDIF", params.p596).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (ptr as *mut f64).add(597), 1);
            {
                let params = &mut *ptr;
                params.p598 = params.p597;
                validate_finite_parameter("LIDREC", params.p598).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (ptr as *mut f64).add(599), 1);
            {
                let params = &mut *ptr;
                params.p600 = params.p599;
                validate_finite_parameter("LIDTUN", params.p600).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (ptr as *mut f64).add(601), 1);
            {
                let params = &mut *ptr;
                params.p602 = params.p601;
                validate_finite_parameter("LVREC0D", params.p602).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (ptr as *mut f64).add(603), 1);
            {
                let params = &mut *ptr;
                params.p604 = params.p603;
                validate_finite_parameter("LVTUN0D", params.p604).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (ptr as *mut f64).add(605), 5);
            {
                let params = &mut *ptr;
                params.p610 = params.p609;
                validate_finite_parameter("LAHLID", params.p610).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (ptr as *mut f64).add(611), 6);
            {
                let params = &mut *ptr;
                params.p617 = params.p616;
                validate_finite_parameter("LNOFF2", params.p617).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 45] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (ptr as *mut f64).add(618), 45);
            {
                let params = &mut *ptr;
                params.p663 = params.p660;
                validate_finite_parameter("WXDIFD", params.p663).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p664 = params.p661;
                validate_finite_parameter("WXRECD", params.p664).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p665 = params.p662;
                validate_finite_parameter("WXTUND", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (ptr as *mut f64).add(666), 64);
            {
                let params = &mut *ptr;
                params.p730 = params.p728;
                validate_finite_parameter("WETA0CV", params.p730).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p731 = params.p729;
                validate_finite_parameter("WETABCV", params.p731).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (ptr as *mut f64).add(732), 36);
            {
                let params = &mut *ptr;
                params.p768 = params.p761;
                validate_finite_parameter("WAGISL", params.p768).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p769 = params.p762;
                validate_finite_parameter("WBGISL", params.p769).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p770 = params.p763;
                validate_finite_parameter("WBGISL1", params.p770).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p771 = params.p764;
                validate_finite_parameter("WCGISL", params.p771).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p772 = params.p765;
                validate_finite_parameter("WRGISL", params.p772).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p773 = params.p766;
                validate_finite_parameter("WKGISL", params.p773).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p774 = params.p767;
                validate_finite_parameter("WFGISL", params.p774).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (ptr as *mut f64).add(775), 1);
            {
                let params = &mut *ptr;
                params.p776 = params.p775;
                validate_finite_parameter("WNTUND", params.p776).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (ptr as *mut f64).add(777), 1);
            {
                let params = &mut *ptr;
                params.p778 = params.p777;
                validate_finite_parameter("WNDIODED", params.p778).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (ptr as *mut f64).add(779), 1);
            {
                let params = &mut *ptr;
                params.p780 = params.p779;
                validate_finite_parameter("WNRECF0D", params.p780).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (ptr as *mut f64).add(781), 1);
            {
                let params = &mut *ptr;
                params.p782 = params.p781;
                validate_finite_parameter("WNRECR0D", params.p782).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (ptr as *mut f64).add(783), 1);
            {
                let params = &mut *ptr;
                params.p784 = params.p783;
                validate_finite_parameter("WIDBJT", params.p784).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (ptr as *mut f64).add(785), 1);
            {
                let params = &mut *ptr;
                params.p786 = params.p785;
                validate_finite_parameter("WIDDIF", params.p786).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (ptr as *mut f64).add(787), 1);
            {
                let params = &mut *ptr;
                params.p788 = params.p787;
                validate_finite_parameter("WIDREC", params.p788).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (ptr as *mut f64).add(789), 1);
            {
                let params = &mut *ptr;
                params.p790 = params.p789;
                validate_finite_parameter("WIDTUN", params.p790).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (ptr as *mut f64).add(791), 1);
            {
                let params = &mut *ptr;
                params.p792 = params.p791;
                validate_finite_parameter("WVREC0D", params.p792).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (ptr as *mut f64).add(793), 1);
            {
                let params = &mut *ptr;
                params.p794 = params.p793;
                validate_finite_parameter("WVTUN0D", params.p794).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (ptr as *mut f64).add(795), 5);
            {
                let params = &mut *ptr;
                params.p800 = params.p799;
                validate_finite_parameter("WAHLID", params.p800).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (ptr as *mut f64).add(801), 6);
            {
                let params = &mut *ptr;
                params.p807 = params.p806;
                validate_finite_parameter("WNOFF2", params.p807).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 45] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (ptr as *mut f64).add(808), 45);
            {
                let params = &mut *ptr;
                params.p853 = params.p850;
                validate_finite_parameter("PXDIFD", params.p853).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p854 = params.p851;
                validate_finite_parameter("PXRECD", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p855 = params.p852;
                validate_finite_parameter("PXTUND", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 64] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (ptr as *mut f64).add(856), 64);
            {
                let params = &mut *ptr;
                params.p920 = params.p918;
                validate_finite_parameter("PETA0CV", params.p920).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p921 = params.p919;
                validate_finite_parameter("PETABCV", params.p921).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 36] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (ptr as *mut f64).add(922), 36);
            {
                let params = &mut *ptr;
                params.p958 = params.p951;
                validate_finite_parameter("PAGISL", params.p958).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p959 = params.p952;
                validate_finite_parameter("PBGISL", params.p959).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p960 = params.p953;
                validate_finite_parameter("PBGISL1", params.p960).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p961 = params.p954;
                validate_finite_parameter("PCGISL", params.p961).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p962 = params.p955;
                validate_finite_parameter("PRGISL", params.p962).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p963 = params.p956;
                validate_finite_parameter("PKGISL", params.p963).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p964 = params.p957;
                validate_finite_parameter("PFGISL", params.p964).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (ptr as *mut f64).add(965), 1);
            {
                let params = &mut *ptr;
                params.p966 = params.p965;
                validate_finite_parameter("PNTUND", params.p966).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (ptr as *mut f64).add(967), 1);
            {
                let params = &mut *ptr;
                params.p968 = params.p967;
                validate_finite_parameter("PNDIODED", params.p968).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (ptr as *mut f64).add(969), 1);
            {
                let params = &mut *ptr;
                params.p970 = params.p969;
                validate_finite_parameter("PNRECF0D", params.p970).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (ptr as *mut f64).add(971), 1);
            {
                let params = &mut *ptr;
                params.p972 = params.p971;
                validate_finite_parameter("PNRECR0D", params.p972).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (ptr as *mut f64).add(973), 1);
            {
                let params = &mut *ptr;
                params.p974 = params.p973;
                validate_finite_parameter("PIDBJT", params.p974).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (ptr as *mut f64).add(975), 1);
            {
                let params = &mut *ptr;
                params.p976 = params.p975;
                validate_finite_parameter("PIDDIF", params.p976).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (ptr as *mut f64).add(977), 1);
            {
                let params = &mut *ptr;
                params.p978 = params.p977;
                validate_finite_parameter("PIDREC", params.p978).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (ptr as *mut f64).add(979), 1);
            {
                let params = &mut *ptr;
                params.p980 = params.p979;
                validate_finite_parameter("PIDTUN", params.p980).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (ptr as *mut f64).add(981), 1);
            {
                let params = &mut *ptr;
                params.p982 = params.p981;
                validate_finite_parameter("PVREC0D", params.p982).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (ptr as *mut f64).add(983), 1);
            {
                let params = &mut *ptr;
                params.p984 = params.p983;
                validate_finite_parameter("PVTUN0D", params.p984).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (ptr as *mut f64).add(985), 5);
            {
                let params = &mut *ptr;
                params.p990 = params.p989;
                validate_finite_parameter("PAHLID", params.p990).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_82: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_82.as_ptr(), (ptr as *mut f64).add(991), 6);
            {
                let params = &mut *ptr;
                params.p997 = params.p996;
                validate_finite_parameter("PNOFF2", params.p997).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_83: [f64; 23] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.74e-7,
                0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_83.as_ptr(), (ptr as *mut f64).add(998), 23);
            {
                let params = &mut *ptr;
                params.p1021 = params.p1013;
                validate_finite_parameter("LPE0", params.p1021).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1022 = params.p1017;
                validate_finite_parameter("EGIDL", params.p1022).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1023 = params.p1022;
                validate_finite_parameter("EGISL", params.p1023).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1024 = params.p1014;
                validate_finite_parameter("LLPE0", params.p1024).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1025 = params.p1018;
                validate_finite_parameter("LEGIDL", params.p1025).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1026 = params.p1025;
                validate_finite_parameter("LEGISL", params.p1026).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1027 = params.p1015;
                validate_finite_parameter("WLPE0", params.p1027).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1028 = params.p1019;
                validate_finite_parameter("WEGIDL", params.p1028).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1029 = params.p1028;
                validate_finite_parameter("WEGISL", params.p1029).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1030 = params.p1016;
                validate_finite_parameter("PLPE0", params.p1030).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1031 = params.p1020;
                validate_finite_parameter("PEGIDL", params.p1031).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1032 = params.p1031;
                validate_finite_parameter("PEGISL", params.p1032).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_84: [f64; 11] = [
                1.12, 1.12, 3.7622e-7, -31051000000.0, 4.9758e-7, -23570000000.0, 3.42537e-7, 4.97232e-7,
                1166450000000.0, 745669000000.0, 0.026,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_84.as_ptr(), (ptr as *mut f64).add(1033), 11);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1044] = [
    ("dtemp", 0), ("l", 1), ("w", 2), ("nf", 3), ("sa", 4), ("sb", 5), ("sd", 6), ("ad", 7), ("as", 8), ("pd", 9), ("ps", 10), ("nrd", 11), ("nrs", 12), ("bjtoff", 13), ("rth0", 14), ("cth0", 15),
    ("nrb", 16), ("frbody", 17), ("rbdb", 18), ("rbsb", 19), ("delvto", 20), ("soimod", 21), ("nbc", 22), ("nseg", 23), ("pdbcp", 24), ("psbcp", 25), ("agbcp", 26), ("agbcp2", 27), ("agbcpd", 28), ("aebcp", 29), ("ids0mult", 30), ("u0mult", 31),
    ("mult_i", 32), ("mult_q", 33), ("mult_fn", 34), ("tnodeout", 35), ("shmod", 36), ("type", 37), ("version", 38), ("rgatemod", 39), ("rbodymod", 40), ("mtrlmod", 41), ("vgstcvmod", 42), ("gidlmod", 43), ("iiimod", 44), ("eot", 45), ("epsrox", 46), ("epsrsub", 47),
    ("ni0sub", 48), ("bg0sub", 49), ("tbgasub", 50), ("tbgbsub", 51), ("phig", 52), ("easub", 53), ("leffeot", 54), ("weffeot", 55), ("vddeot", 56), ("tempeot", 57), ("ados", 58), ("bdos", 59), ("epsrgate", 60), ("capmod", 61), ("mobmod", 62), ("paramchk", 63),
    ("nodechk", 64), ("binunit", 65), ("tox", 66), ("toxm", 67), ("dtoxcv", 68), ("cdsc", 69), ("cdscb", 70), ("cdscd", 71), ("cit", 72), ("nfactor", 73), ("vsat", 74), ("at", 75), ("a0", 76), ("ags", 77), ("a1", 78), ("a2", 79),
    ("keta", 80), ("nsub", 81), ("nch", 82), ("ngate", 83), ("nsd", 84), ("gamma1", 85), ("gamma2", 86), ("vbx", 87), ("vbm", 88), ("xt", 89), ("k1", 90), ("kt1", 91), ("kt1l", 92), ("kt2", 93), ("k2", 94), ("k3", 95),
    ("k3b", 96), ("w0", 97), ("lpeb", 98), ("dvt0", 99), ("dvt1", 100), ("dvt2", 101), ("dvt0w", 102), ("dvt1w", 103), ("dvt2w", 104), ("drout", 105), ("dsub", 106), ("vtho", 107), ("vth0", 108), ("vfb", 109), ("ua", 110), ("ua1", 111),
    ("ub", 112), ("ub1", 113), ("uc", 114), ("uc1", 115), ("u0", 116), ("eu", 117), ("ute", 118), ("ucs", 119), ("ucste", 120), ("ud", 121), ("ud1", 122), ("ubg1", 123), ("ubg2", 124), ("voff", 125), ("tnom", 126), ("cgso", 127),
    ("cgdo", 128), ("xpart", 129), ("delta", 130), ("rsh", 131), ("rdsw", 132), ("rsw", 133), ("rdw", 134), ("rsc", 135), ("rdc", 136), ("trs", 137), ("trd", 138), ("rswmin", 139), ("rdwmin", 140), ("prwg", 141), ("prwb", 142), ("prwe", 143),
    ("prt", 144), ("eta0", 145), ("etab", 146), ("eta0cv", 147), ("etabcv", 148), ("pclm", 149), ("pdiblc1", 150), ("pdiblc2", 151), ("pdiblcb", 152), ("pvag", 153), ("tbox", 154), ("tsi", 155), ("etsi", 156), ("xj", 157), ("agidl", 158), ("bgidl", 159),
    ("bgidl1", 160), ("cgidl", 161), ("rgidl", 162), ("kgidl", 163), ("fgidl", 164), ("agisl", 165), ("bgisl", 166), ("bgisl1", 167), ("cgisl", 168), ("rgisl", 169), ("kgisl", 170), ("fgisl", 171), ("ndiode", 172), ("ndioded", 173), ("xbjt", 174), ("xdif", 175),
    ("xrec", 176), ("xtun", 177), ("xdifd", 178), ("xrecd", 179), ("xtund", 180), ("pbswg", 181), ("pbswgd", 182), ("mjswg", 183), ("mjswgd", 184), ("cjswg", 185), ("cjswgd", 186), ("lint", 187), ("ll", 188), ("llc", 189), ("lln", 190), ("lw", 191),
    ("lwc", 192), ("lwn", 193), ("lwl", 194), ("lwlc", 195), ("wr", 196), ("wint", 197), ("dwg", 198), ("dwb", 199), ("wl", 200), ("wlc", 201), ("wln", 202), ("ww", 203), ("wwc", 204), ("wwn", 205), ("wwl", 206), ("wwlc", 207),
    ("b0", 208), ("b1", 209), ("cgsl", 210), ("cgdl", 211), ("ckappa", 212), ("cf", 213), ("clc", 214), ("cle", 215), ("dwc", 216), ("dlc", 217), ("alpha0", 218), ("noia", 219), ("noib", 220), ("noic", 221), ("fnoimod", 222), ("tnoimod", 223),
    ("tnoic", 224), ("rnoic", 225), ("scalen", 226), ("tnoia", 227), ("tnoib", 228), ("rnoia", 229), ("rnoib", 230), ("ntnoi", 231), ("saref", 232), ("sbref", 233), ("wlod", 234), ("ku0", 235), ("kvsat", 236), ("kvth0", 237), ("tku0", 238), ("llodku0", 239),
    ("wlodku0", 240), ("llodvth", 241), ("wlodvth", 242), ("lku0", 243), ("wku0", 244), ("pku0", 245), ("lkvth0", 246), ("wkvth0", 247), ("pkvth0", 248), ("stk2", 249), ("lodk2", 250), ("steta0", 251), ("lodeta0", 252), ("steta0cv", 253), ("lodeta0cv", 254), ("gbmin", 255),
    ("bf", 256), ("w0flk", 257), ("dvtp0", 258), ("ldvtp0", 259), ("wdvtp0", 260), ("pdvtp0", 261), ("dvtp1", 262), ("ldvtp1", 263), ("wdvtp1", 264), ("pdvtp1", 265), ("dvtp2", 266), ("ldvtp2", 267), ("wdvtp2", 268), ("pdvtp2", 269), ("dvtp3", 270), ("ldvtp3", 271),
    ("wdvtp3", 272), ("pdvtp3", 273), ("dvtp4", 274), ("ldvtp4", 275), ("wdvtp4", 276), ("pdvtp4", 277), ("minv", 278), ("lminv", 279), ("wminv", 280), ("pminv", 281), ("pdits", 282), ("pditsl", 283), ("pditsd", 284), ("fprout", 285), ("lfprout", 286), ("lpdits", 287),
    ("lpditsd", 288), ("wfprout", 289), ("wpdits", 290), ("wpditsd", 291), ("pfprout", 292), ("ppdits", 293), ("ppditsd", 294), ("em", 295), ("ef", 296), ("af", 297), ("kf", 298), ("noif", 299), ("k1w1", 300), ("k1w2", 301), ("ketas", 302), ("dwbc", 303),
    ("beta0", 304), ("beta1", 305), ("beta2", 306), ("vdsatii0", 307), ("tii", 308), ("lii", 309), ("sii0", 310), ("sii1", 311), ("sii2", 312), ("siid", 313), ("fbjtii", 314), ("ebjtii", 315), ("cbjtii", 316), ("vbci", 317), ("abjtii", 318), ("mbjtii", 319),
    ("tvbci", 320), ("esatii", 321), ("ntun", 322), ("ntund", 323), ("nrecf0", 324), ("nrecf0d", 325), ("nrecr0", 326), ("nrecr0d", 327), ("isbjt", 328), ("idbjt", 329), ("isdif", 330), ("iddif", 331), ("isrec", 332), ("idrec", 333), ("istun", 334), ("idtun", 335),
    ("ln", 336), ("vrec0", 337), ("vrec0d", 338), ("vtun0", 339), ("vtun0d", 340), ("nbjt", 341), ("lbjt0", 342), ("ldif0", 343), ("vabjt", 344), ("aely", 345), ("ahli", 346), ("ahlid", 347), ("rbody", 348), ("rbsh", 349), ("cgeo", 350), ("tt", 351),
    ("ndif", 352), ("vsdfb", 353), ("vsdth", 354), ("csdmin", 355), ("asd", 356), ("csdesw", 357), ("ntrecf", 358), ("ntrecr", 359), ("dlcb", 360), ("fbody", 361), ("tcjswg", 362), ("tpbswg", 363), ("tcjswgd", 364), ("tpbswgd", 365), ("acde", 366), ("moin", 367),
    ("noff", 368), ("noff2", 369), ("delvt", 370), ("kb1", 371), ("dlbg", 372), ("cfrcoeff", 373), ("igbmod", 374), ("igcmod", 375), ("toxqm", 376), ("wth0", 377), ("rhalo", 378), ("ntox", 379), ("toxref", 380), ("ebg", 381), ("vevb", 382), ("alphagb1", 383),
    ("alphagb1_t", 384), ("betagb1", 385), ("vgb1", 386), ("vecb", 387), ("alphagb2", 388), ("alphagb2_t", 389), ("betagb2", 390), ("vgb2", 391), ("aigbcp2", 392), ("aigbcp2_t", 393), ("bigbcp2", 394), ("cigbcp2", 395), ("voxh", 396), ("deltavox", 397), ("aigc", 398), ("aigc1", 399),
    ("bigc", 400), ("cigc", 401), ("aigsd", 402), ("aigsd1", 403), ("bigsd", 404), ("cigsd", 405), ("nigc", 406), ("pigcd", 407), ("poxedge", 408), ("igt", 409), ("dlcig", 410), ("vbs0pd", 411), ("vbs0fd", 412), ("vbsa", 413), ("nofffd", 414), ("vofffd", 415),
    ("k1b", 416), ("k2b", 417), ("dk2b", 418), ("dvbd0", 419), ("dvbd1", 420), ("moinfd", 421), ("xrcrg1", 422), ("xrcrg2", 423), ("rshg", 424), ("ngcon", 425), ("rver", 426), ("xgw", 427), ("xgl", 428), ("rdsmod", 429), ("ids0multmod", 430), ("minr", 431),
    ("fdmod", 432), ("vsce", 433), ("cdsbs", 434), ("minvcv", 435), ("lminvcv", 436), ("wminvcv", 437), ("pminvcv", 438), ("voffcv", 439), ("lvoffcv", 440), ("wvoffcv", 441), ("pvoffcv", 442), ("lxj", 443), ("lalphagb1", 444), ("lalphagb1_t", 445), ("lbetagb1", 446), ("lalphagb2", 447),
    ("lalphagb2_t", 448), ("lbetagb2", 449), ("laigbcp2", 450), ("laigbcp2_t", 451), ("lbigbcp2", 452), ("lcigbcp2", 453), ("lcgsl", 454), ("lcgdl", 455), ("lckappa", 456), ("lndif", 457), ("lute", 458), ("lkt1", 459), ("lkt1l", 460), ("lkt2", 461), ("lua1", 462), ("lub1", 463),
    ("luc1", 464), ("lat", 465), ("lprt", 466), ("lntrecf", 467), ("lntrecr", 468), ("lxbjt", 469), ("lxdif", 470), ("lxrec", 471), ("lxtun", 472), ("lxdifd", 473), ("lxrecd", 474), ("lxtund", 475), ("laigc", 476), ("laigc1", 477), ("lbigc", 478), ("lcigc", 479),
    ("laigsd", 480), ("laigsd1", 481), ("lbigsd", 482), ("lcigsd", 483), ("lnigc", 484), ("lpigcd", 485), ("lpoxedge", 486), ("ligt", 487), ("lnch", 488), ("lnsub", 489), ("lngate", 490), ("lnsd", 491), ("lvth0", 492), ("lvfb", 493), ("lk1", 494), ("lk1w1", 495),
    ("lk1w2", 496), ("lk2", 497), ("lk3", 498), ("lk3b", 499), ("lkb1", 500), ("lw0", 501), ("llpeb", 502), ("ldvt0", 503), ("ldvt1", 504), ("ldvt2", 505), ("ldvt0w", 506), ("ldvt1w", 507), ("ldvt2w", 508), ("lu0", 509), ("leu", 510), ("lua", 511),
    ("lub", 512), ("luc", 513), ("lud", 514), ("lud1", 515), ("lucste", 516), ("lucs", 517), ("lvsat", 518), ("la0", 519), ("lags", 520), ("lb0", 521), ("lb1", 522), ("lketa", 523), ("lketas", 524), ("la1", 525), ("la2", 526), ("lrdsw", 527),
    ("lrsw", 528), ("lrdw", 529), ("lprwb", 530), ("lprwe", 531), ("lprwg", 532), ("lwr", 533), ("lnfactor", 534), ("ldwg", 535), ("ldwb", 536), ("lvoff", 537), ("leta0", 538), ("letab", 539), ("leta0cv", 540), ("letabcv", 541), ("ldsub", 542), ("lcit", 543),
    ("lcdsc", 544), ("lcdscb", 545), ("lcdscd", 546), ("lpclm", 547), ("lpdiblc1", 548), ("lpdiblc2", 549), ("lpdiblcb", 550), ("ldrout", 551), ("lpvag", 552), ("ldelta", 553), ("lalpha0", 554), ("lfbjtii", 555), ("labjtii", 556), ("lcbjtii", 557), ("lebjtii", 558), ("lmbjtii", 559),
    ("lvbci", 560), ("lbeta0", 561), ("lbeta1", 562), ("lbeta2", 563), ("lvdsatii0", 564), ("llii", 565), ("lesatii", 566), ("lsii0", 567), ("lsii1", 568), ("lsii2", 569), ("lsiid", 570), ("lagidl", 571), ("lbgidl", 572), ("lbgidl1", 573), ("lcgidl", 574), ("lrgidl", 575),
    ("lkgidl", 576), ("lfgidl", 577), ("lagisl", 578), ("lbgisl", 579), ("lbgisl1", 580), ("lcgisl", 581), ("lrgisl", 582), ("lkgisl", 583), ("lfgisl", 584), ("lntun", 585), ("lntund", 586), ("lndiode", 587), ("lndioded", 588), ("lnrecf0", 589), ("lnrecf0d", 590), ("lnrecr0", 591),
    ("lnrecr0d", 592), ("lisbjt", 593), ("lidbjt", 594), ("lisdif", 595), ("liddif", 596), ("lisrec", 597), ("lidrec", 598), ("listun", 599), ("lidtun", 600), ("lvrec0", 601), ("lvrec0d", 602), ("lvtun0", 603), ("lvtun0d", 604), ("lnbjt", 605), ("llbjt0", 606), ("lvabjt", 607),
    ("laely", 608), ("lahli", 609), ("lahlid", 610), ("lvsdfb", 611), ("lvsdth", 612), ("ldelvt", 613), ("lacde", 614), ("lmoin", 615), ("lnoff", 616), ("lnoff2", 617), ("lxrcrg1", 618), ("lxrcrg2", 619), ("lvbsa", 620), ("lvsce", 621), ("lcdsbs", 622), ("lnofffd", 623),
    ("lvofffd", 624), ("lk1b", 625), ("lk2b", 626), ("ldk2b", 627), ("ldvbd0", 628), ("ldvbd1", 629), ("lmoinfd", 630), ("lvbs0pd", 631), ("lvbs0fd", 632), ("wxj", 633), ("walphagb1", 634), ("walphagb1_t", 635), ("wbetagb1", 636), ("walphagb2", 637), ("walphagb2_t", 638), ("wbetagb2", 639),
    ("waigbcp2", 640), ("waigbcp2_t", 641), ("wbigbcp2", 642), ("wcigbcp2", 643), ("wcgsl", 644), ("wcgdl", 645), ("wckappa", 646), ("wndif", 647), ("wute", 648), ("wkt1", 649), ("wkt1l", 650), ("wkt2", 651), ("wua1", 652), ("wub1", 653), ("wuc1", 654), ("wat", 655),
    ("wprt", 656), ("wntrecf", 657), ("wntrecr", 658), ("wxbjt", 659), ("wxdif", 660), ("wxrec", 661), ("wxtun", 662), ("wxdifd", 663), ("wxrecd", 664), ("wxtund", 665), ("waigc", 666), ("waigc1", 667), ("wbigc", 668), ("wcigc", 669), ("waigsd", 670), ("waigsd1", 671),
    ("wbigsd", 672), ("wcigsd", 673), ("wnigc", 674), ("wpigcd", 675), ("wpoxedge", 676), ("wigt", 677), ("wnch", 678), ("wnsub", 679), ("wngate", 680), ("wnsd", 681), ("wvth0", 682), ("wvfb", 683), ("wk1", 684), ("wk1w1", 685), ("wk1w2", 686), ("wk2", 687),
    ("wk3", 688), ("wk3b", 689), ("wkb1", 690), ("ww0", 691), ("wlpeb", 692), ("wdvt0", 693), ("wdvt1", 694), ("wdvt2", 695), ("wdvt0w", 696), ("wdvt1w", 697), ("wdvt2w", 698), ("wu0", 699), ("weu", 700), ("wua", 701), ("wub", 702), ("wuc", 703),
    ("wud", 704), ("wud1", 705), ("wucste", 706), ("wucs", 707), ("wvsat", 708), ("wa0", 709), ("wags", 710), ("wb0", 711), ("wb1", 712), ("wketa", 713), ("wketas", 714), ("wa1", 715), ("wa2", 716), ("wrdsw", 717), ("wrsw", 718), ("wrdw", 719),
    ("wprwb", 720), ("wprwe", 721), ("wprwg", 722), ("wwr", 723), ("wnfactor", 724), ("wdwg", 725), ("wdwb", 726), ("wvoff", 727), ("weta0", 728), ("wetab", 729), ("weta0cv", 730), ("wetabcv", 731), ("wdsub", 732), ("wcit", 733), ("wcdsc", 734), ("wcdscb", 735),
    ("wcdscd", 736), ("wpclm", 737), ("wpdiblc1", 738), ("wpdiblc2", 739), ("wpdiblcb", 740), ("wdrout", 741), ("wpvag", 742), ("wdelta", 743), ("walpha0", 744), ("wfbjtii", 745), ("wabjtii", 746), ("wcbjtii", 747), ("webjtii", 748), ("wmbjtii", 749), ("wvbci", 750), ("wbeta0", 751),
    ("wbeta1", 752), ("wbeta2", 753), ("wvdsatii0", 754), ("wlii", 755), ("wesatii", 756), ("wsii0", 757), ("wsii1", 758), ("wsii2", 759), ("wsiid", 760), ("wagidl", 761), ("wbgidl", 762), ("wbgidl1", 763), ("wcgidl", 764), ("wrgidl", 765), ("wkgidl", 766), ("wfgidl", 767),
    ("wagisl", 768), ("wbgisl", 769), ("wbgisl1", 770), ("wcgisl", 771), ("wrgisl", 772), ("wkgisl", 773), ("wfgisl", 774), ("wntun", 775), ("wntund", 776), ("wndiode", 777), ("wndioded", 778), ("wnrecf0", 779), ("wnrecf0d", 780), ("wnrecr0", 781), ("wnrecr0d", 782), ("wisbjt", 783),
    ("widbjt", 784), ("wisdif", 785), ("widdif", 786), ("wisrec", 787), ("widrec", 788), ("wistun", 789), ("widtun", 790), ("wvrec0", 791), ("wvrec0d", 792), ("wvtun0", 793), ("wvtun0d", 794), ("wnbjt", 795), ("wlbjt0", 796), ("wvabjt", 797), ("waely", 798), ("wahli", 799),
    ("wahlid", 800), ("wvsdfb", 801), ("wvsdth", 802), ("wdelvt", 803), ("wacde", 804), ("wmoin", 805), ("wnoff", 806), ("wnoff2", 807), ("wxrcrg1", 808), ("wxrcrg2", 809), ("wvbsa", 810), ("wvsce", 811), ("wcdsbs", 812), ("wnofffd", 813), ("wvofffd", 814), ("wk1b", 815),
    ("wk2b", 816), ("wdk2b", 817), ("wdvbd0", 818), ("wdvbd1", 819), ("wmoinfd", 820), ("wvbs0pd", 821), ("wvbs0fd", 822), ("pxj", 823), ("palphagb1", 824), ("palphagb1_t", 825), ("pbetagb1", 826), ("palphagb2", 827), ("palphagb2_t", 828), ("pbetagb2", 829), ("paigbcp2", 830), ("paigbcp2_t", 831),
    ("pbigbcp2", 832), ("pcigbcp2", 833), ("pcgsl", 834), ("pcgdl", 835), ("pckappa", 836), ("pndif", 837), ("pute", 838), ("pkt1", 839), ("pkt1l", 840), ("pkt2", 841), ("pua1", 842), ("pub1", 843), ("puc1", 844), ("pat", 845), ("pprt", 846), ("pntrecf", 847),
    ("pntrecr", 848), ("pxbjt", 849), ("pxdif", 850), ("pxrec", 851), ("pxtun", 852), ("pxdifd", 853), ("pxrecd", 854), ("pxtund", 855), ("paigc", 856), ("paigc1", 857), ("pbigc", 858), ("pcigc", 859), ("paigsd", 860), ("paigsd1", 861), ("pbigsd", 862), ("pcigsd", 863),
    ("pnigc", 864), ("ppigcd", 865), ("ppoxedge", 866), ("pigt", 867), ("pnch", 868), ("pnsub", 869), ("pnsd", 870), ("pngate", 871), ("pvth0", 872), ("pvfb", 873), ("pk1", 874), ("pk1w1", 875), ("pk1w2", 876), ("pk2", 877), ("pk3", 878), ("pk3b", 879),
    ("pkb1", 880), ("pw0", 881), ("plpeb", 882), ("pdvt0", 883), ("pdvt1", 884), ("pdvt2", 885), ("pdvt0w", 886), ("pdvt1w", 887), ("pdvt2w", 888), ("pu0", 889), ("peu", 890), ("pua", 891), ("pub", 892), ("puc", 893), ("pud", 894), ("pud1", 895),
    ("pucste", 896), ("pucs", 897), ("pvsat", 898), ("pa0", 899), ("pags", 900), ("pb0", 901), ("pb1", 902), ("pketa", 903), ("pketas", 904), ("pa1", 905), ("pa2", 906), ("prdsw", 907), ("prsw", 908), ("prdw", 909), ("pprwb", 910), ("pprwe", 911),
    ("pprwg", 912), ("pwr", 913), ("pnfactor", 914), ("pdwg", 915), ("pdwb", 916), ("pvoff", 917), ("peta0", 918), ("petab", 919), ("peta0cv", 920), ("petabcv", 921), ("pdsub", 922), ("pcit", 923), ("pcdsc", 924), ("pcdscb", 925), ("pcdscd", 926), ("ppclm", 927),
    ("ppdiblc1", 928), ("ppdiblc2", 929), ("ppdiblcb", 930), ("pdrout", 931), ("ppvag", 932), ("pdelta", 933), ("palpha0", 934), ("pfbjtii", 935), ("pabjtii", 936), ("pcbjtii", 937), ("pebjtii", 938), ("pmbjtii", 939), ("pvbci", 940), ("pbeta0", 941), ("pbeta1", 942), ("pbeta2", 943),
    ("pvdsatii0", 944), ("plii", 945), ("pesatii", 946), ("psii0", 947), ("psii1", 948), ("psii2", 949), ("psiid", 950), ("pagidl", 951), ("pbgidl", 952), ("pbgidl1", 953), ("pcgidl", 954), ("prgidl", 955), ("pkgidl", 956), ("pfgidl", 957), ("pagisl", 958), ("pbgisl", 959),
    ("pbgisl1", 960), ("pcgisl", 961), ("prgisl", 962), ("pkgisl", 963), ("pfgisl", 964), ("pntun", 965), ("pntund", 966), ("pndiode", 967), ("pndioded", 968), ("pnrecf0", 969), ("pnrecf0d", 970), ("pnrecr0", 971), ("pnrecr0d", 972), ("pisbjt", 973), ("pidbjt", 974), ("pisdif", 975),
    ("piddif", 976), ("pisrec", 977), ("pidrec", 978), ("pistun", 979), ("pidtun", 980), ("pvrec0", 981), ("pvrec0d", 982), ("pvtun0", 983), ("pvtun0d", 984), ("pnbjt", 985), ("plbjt0", 986), ("pvabjt", 987), ("paely", 988), ("pahli", 989), ("pahlid", 990), ("pvsdfb", 991),
    ("pvsdth", 992), ("pdelvt", 993), ("pacde", 994), ("pmoin", 995), ("pnoff", 996), ("pnoff2", 997), ("pxrcrg1", 998), ("pxrcrg2", 999), ("pvbsa", 1000), ("pvsce", 1001), ("pcdsbs", 1002), ("pnofffd", 1003), ("pvofffd", 1004), ("pk1b", 1005), ("pk2b", 1006), ("pdk2b", 1007),
    ("pdvbd0", 1008), ("pdvbd1", 1009), ("pmoinfd", 1010), ("pvbs0pd", 1011), ("pvbs0fd", 1012), ("nlx", 1013), ("lnlx", 1014), ("wnlx", 1015), ("pnlx", 1016), ("ngidl", 1017), ("lngidl", 1018), ("wngidl", 1019), ("pngidl", 1020), ("lpe0", 1021), ("egidl", 1022), ("egisl", 1023),
    ("llpe0", 1024), ("legidl", 1025), ("legisl", 1026), ("wlpe0", 1027), ("wegidl", 1028), ("wegisl", 1029), ("plpe0", 1030), ("pegidl", 1031), ("pegisl", 1032), ("eggbcp2", 1033), ("eggdep", 1034), ("agb1", 1035), ("bgb1", 1036), ("agb2", 1037), ("bgb2", 1038), ("agbc2n", 1039),
    ("agbc2p", 1040), ("bgbc2n", 1041), ("bgbc2p", 1042), ("vtm00", 1043),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1044] = [
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
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1044] = [
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
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1044] = [
    "DTEMP", "L", "W", "NF", "SA", "SB", "SD", "AD", "AS", "PD", "PS", "NRD", "NRS", "BJTOFF", "RTH0", "CTH0",
    "NRB", "FRBODY", "RBDB", "RBSB", "DELVTO", "SOIMOD", "NBC", "NSEG", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP", "IDS0MULT", "U0MULT",
    "MULT_I", "MULT_Q", "MULT_FN", "TNODEOUT", "SHMOD", "TYPE", "VERSION", "RGATEMOD", "RBODYMOD", "MTRLMOD", "VGSTCVMOD", "GIDLMOD", "IIIMOD", "EOT", "EPSROX", "EPSRSUB",
    "NI0SUB", "BG0SUB", "TBGASUB", "TBGBSUB", "PHIG", "EASUB", "LEFFEOT", "WEFFEOT", "VDDEOT", "TEMPEOT", "ADOS", "BDOS", "EPSRGATE", "CAPMOD", "MOBMOD", "PARAMCHK",
    "NODECHK", "BINUNIT", "TOX", "TOXM", "DTOXCV", "CDSC", "CDSCB", "CDSCD", "CIT", "NFACTOR", "VSAT", "AT", "A0", "AGS", "A1", "A2",
    "KETA", "NSUB", "NCH", "NGATE", "NSD", "GAMMA1", "GAMMA2", "VBX", "VBM", "XT", "K1", "KT1", "KT1L", "KT2", "K2", "K3",
    "K3B", "W0", "LPEB", "DVT0", "DVT1", "DVT2", "DVT0W", "DVT1W", "DVT2W", "DROUT", "DSUB", "VTHO", "VTH0", "VFB", "UA", "UA1",
    "UB", "UB1", "UC", "UC1", "U0", "EU", "UTE", "UCS", "UCSTE", "UD", "UD1", "UBG1", "UBG2", "VOFF", "TNOM", "CGSO",
    "CGDO", "XPART", "DELTA", "RSH", "RDSW", "RSW", "RDW", "RSC", "RDC", "TRS", "TRD", "RSWMIN", "RDWMIN", "PRWG", "PRWB", "PRWE",
    "PRT", "ETA0", "ETAB", "ETA0CV", "ETABCV", "PCLM", "PDIBLC1", "PDIBLC2", "PDIBLCB", "PVAG", "TBOX", "TSI", "ETSI", "XJ", "AGIDL", "BGIDL",
    "BGIDL1", "CGIDL", "RGIDL", "KGIDL", "FGIDL", "AGISL", "BGISL", "BGISL1", "CGISL", "RGISL", "KGISL", "FGISL", "NDIODE", "NDIODED", "XBJT", "XDIF",
    "XREC", "XTUN", "XDIFD", "XRECD", "XTUND", "PBSWG", "PBSWGD", "MJSWG", "MJSWGD", "CJSWG", "CJSWGD", "LINT", "LL", "LLC", "LLN", "LW",
    "LWC", "LWN", "LWL", "LWLC", "WR", "WINT", "DWG", "DWB", "WL", "WLC", "WLN", "WW", "WWC", "WWN", "WWL", "WWLC",
    "B0", "B1", "CGSL", "CGDL", "CKAPPA", "CF", "CLC", "CLE", "DWC", "DLC", "ALPHA0", "NOIA", "NOIB", "NOIC", "FNOIMOD", "TNOIMOD",
    "TNOIC", "RNOIC", "SCALEN", "TNOIA", "TNOIB", "RNOIA", "RNOIB", "NTNOI", "SAREF", "SBREF", "WLOD", "KU0", "KVSAT", "KVTH0", "TKU0", "LLODKU0",
    "WLODKU0", "LLODVTH", "WLODVTH", "LKU0", "WKU0", "PKU0", "LKVTH0", "WKVTH0", "PKVTH0", "STK2", "LODK2", "STETA0", "LODETA0", "STETA0CV", "LODETA0CV", "GBMIN",
    "BF", "W0FLK", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3",
    "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "MINV", "LMINV", "WMINV", "PMINV", "PDITS", "PDITSL", "PDITSD", "FPROUT", "LFPROUT", "LPDITS",
    "LPDITSD", "WFPROUT", "WPDITS", "WPDITSD", "PFPROUT", "PPDITS", "PPDITSD", "EM", "EF", "AF", "KF", "NOIF", "K1W1", "K1W2", "KETAS", "DWBC",
    "BETA0", "BETA1", "BETA2", "VDSATII0", "TII", "LII", "SII0", "SII1", "SII2", "SIID", "FBJTII", "EBJTII", "CBJTII", "VBCI", "ABJTII", "MBJTII",
    "TVBCI", "ESATII", "NTUN", "NTUND", "NRECF0", "NRECF0D", "NRECR0", "NRECR0D", "ISBJT", "IDBJT", "ISDIF", "IDDIF", "ISREC", "IDREC", "ISTUN", "IDTUN",
    "LN", "VREC0", "VREC0D", "VTUN0", "VTUN0D", "NBJT", "LBJT0", "LDIF0", "VABJT", "AELY", "AHLI", "AHLID", "RBODY", "RBSH", "CGEO", "TT",
    "NDIF", "VSDFB", "VSDTH", "CSDMIN", "ASD", "CSDESW", "NTRECF", "NTRECR", "DLCB", "FBODY", "TCJSWG", "TPBSWG", "TCJSWGD", "TPBSWGD", "ACDE", "MOIN",
    "NOFF", "NOFF2", "DELVT", "KB1", "DLBG", "CFRCOEFF", "IGBMOD", "IGCMOD", "TOXQM", "WTH0", "RHALO", "NTOX", "TOXREF", "EBG", "VEVB", "ALPHAGB1",
    "ALPHAGB1_T", "BETAGB1", "VGB1", "VECB", "ALPHAGB2", "ALPHAGB2_T", "BETAGB2", "VGB2", "AIGBCP2", "AIGBCP2_T", "BIGBCP2", "CIGBCP2", "VOXH", "DELTAVOX", "AIGC", "AIGC1",
    "BIGC", "CIGC", "AIGSD", "AIGSD1", "BIGSD", "CIGSD", "NIGC", "PIGCD", "POXEDGE", "IGT", "DLCIG", "VBS0PD", "VBS0FD", "VBSA", "NOFFFD", "VOFFFD",
    "K1B", "K2B", "DK2B", "DVBD0", "DVBD1", "MOINFD", "XRCRG1", "XRCRG2", "RSHG", "NGCON", "RVER", "XGW", "XGL", "RDSMOD", "IDS0MULTMOD", "MINR",
    "FDMOD", "VSCE", "CDSBS", "MINVCV", "LMINVCV", "WMINVCV", "PMINVCV", "VOFFCV", "LVOFFCV", "WVOFFCV", "PVOFFCV", "LXJ", "LALPHAGB1", "LALPHAGB1_T", "LBETAGB1", "LALPHAGB2",
    "LALPHAGB2_T", "LBETAGB2", "LAIGBCP2", "LAIGBCP2_T", "LBIGBCP2", "LCIGBCP2", "LCGSL", "LCGDL", "LCKAPPA", "LNDIF", "LUTE", "LKT1", "LKT1L", "LKT2", "LUA1", "LUB1",
    "LUC1", "LAT", "LPRT", "LNTRECF", "LNTRECR", "LXBJT", "LXDIF", "LXREC", "LXTUN", "LXDIFD", "LXRECD", "LXTUND", "LAIGC", "LAIGC1", "LBIGC", "LCIGC",
    "LAIGSD", "LAIGSD1", "LBIGSD", "LCIGSD", "LNIGC", "LPIGCD", "LPOXEDGE", "LIGT", "LNCH", "LNSUB", "LNGATE", "LNSD", "LVTH0", "LVFB", "LK1", "LK1W1",
    "LK1W2", "LK2", "LK3", "LK3B", "LKB1", "LW0", "LLPEB", "LDVT0", "LDVT1", "LDVT2", "LDVT0W", "LDVT1W", "LDVT2W", "LU0", "LEU", "LUA",
    "LUB", "LUC", "LUD", "LUD1", "LUCSTE", "LUCS", "LVSAT", "LA0", "LAGS", "LB0", "LB1", "LKETA", "LKETAS", "LA1", "LA2", "LRDSW",
    "LRSW", "LRDW", "LPRWB", "LPRWE", "LPRWG", "LWR", "LNFACTOR", "LDWG", "LDWB", "LVOFF", "LETA0", "LETAB", "LETA0CV", "LETABCV", "LDSUB", "LCIT",
    "LCDSC", "LCDSCB", "LCDSCD", "LPCLM", "LPDIBLC1", "LPDIBLC2", "LPDIBLCB", "LDROUT", "LPVAG", "LDELTA", "LALPHA0", "LFBJTII", "LABJTII", "LCBJTII", "LEBJTII", "LMBJTII",
    "LVBCI", "LBETA0", "LBETA1", "LBETA2", "LVDSATII0", "LLII", "LESATII", "LSII0", "LSII1", "LSII2", "LSIID", "LAGIDL", "LBGIDL", "LBGIDL1", "LCGIDL", "LRGIDL",
    "LKGIDL", "LFGIDL", "LAGISL", "LBGISL", "LBGISL1", "LCGISL", "LRGISL", "LKGISL", "LFGISL", "LNTUN", "LNTUND", "LNDIODE", "LNDIODED", "LNRECF0", "LNRECF0D", "LNRECR0",
    "LNRECR0D", "LISBJT", "LIDBJT", "LISDIF", "LIDDIF", "LISREC", "LIDREC", "LISTUN", "LIDTUN", "LVREC0", "LVREC0D", "LVTUN0", "LVTUN0D", "LNBJT", "LLBJT0", "LVABJT",
    "LAELY", "LAHLI", "LAHLID", "LVSDFB", "LVSDTH", "LDELVT", "LACDE", "LMOIN", "LNOFF", "LNOFF2", "LXRCRG1", "LXRCRG2", "LVBSA", "LVSCE", "LCDSBS", "LNOFFFD",
    "LVOFFFD", "LK1B", "LK2B", "LDK2B", "LDVBD0", "LDVBD1", "LMOINFD", "LVBS0PD", "LVBS0FD", "WXJ", "WALPHAGB1", "WALPHAGB1_T", "WBETAGB1", "WALPHAGB2", "WALPHAGB2_T", "WBETAGB2",
    "WAIGBCP2", "WAIGBCP2_T", "WBIGBCP2", "WCIGBCP2", "WCGSL", "WCGDL", "WCKAPPA", "WNDIF", "WUTE", "WKT1", "WKT1L", "WKT2", "WUA1", "WUB1", "WUC1", "WAT",
    "WPRT", "WNTRECF", "WNTRECR", "WXBJT", "WXDIF", "WXREC", "WXTUN", "WXDIFD", "WXRECD", "WXTUND", "WAIGC", "WAIGC1", "WBIGC", "WCIGC", "WAIGSD", "WAIGSD1",
    "WBIGSD", "WCIGSD", "WNIGC", "WPIGCD", "WPOXEDGE", "WIGT", "WNCH", "WNSUB", "WNGATE", "WNSD", "WVTH0", "WVFB", "WK1", "WK1W1", "WK1W2", "WK2",
    "WK3", "WK3B", "WKB1", "WW0", "WLPEB", "WDVT0", "WDVT1", "WDVT2", "WDVT0W", "WDVT1W", "WDVT2W", "WU0", "WEU", "WUA", "WUB", "WUC",
    "WUD", "WUD1", "WUCSTE", "WUCS", "WVSAT", "WA0", "WAGS", "WB0", "WB1", "WKETA", "WKETAS", "WA1", "WA2", "WRDSW", "WRSW", "WRDW",
    "WPRWB", "WPRWE", "WPRWG", "WWR", "WNFACTOR", "WDWG", "WDWB", "WVOFF", "WETA0", "WETAB", "WETA0CV", "WETABCV", "WDSUB", "WCIT", "WCDSC", "WCDSCB",
    "WCDSCD", "WPCLM", "WPDIBLC1", "WPDIBLC2", "WPDIBLCB", "WDROUT", "WPVAG", "WDELTA", "WALPHA0", "WFBJTII", "WABJTII", "WCBJTII", "WEBJTII", "WMBJTII", "WVBCI", "WBETA0",
    "WBETA1", "WBETA2", "WVDSATII0", "WLII", "WESATII", "WSII0", "WSII1", "WSII2", "WSIID", "WAGIDL", "WBGIDL", "WBGIDL1", "WCGIDL", "WRGIDL", "WKGIDL", "WFGIDL",
    "WAGISL", "WBGISL", "WBGISL1", "WCGISL", "WRGISL", "WKGISL", "WFGISL", "WNTUN", "WNTUND", "WNDIODE", "WNDIODED", "WNRECF0", "WNRECF0D", "WNRECR0", "WNRECR0D", "WISBJT",
    "WIDBJT", "WISDIF", "WIDDIF", "WISREC", "WIDREC", "WISTUN", "WIDTUN", "WVREC0", "WVREC0D", "WVTUN0", "WVTUN0D", "WNBJT", "WLBJT0", "WVABJT", "WAELY", "WAHLI",
    "WAHLID", "WVSDFB", "WVSDTH", "WDELVT", "WACDE", "WMOIN", "WNOFF", "WNOFF2", "WXRCRG1", "WXRCRG2", "WVBSA", "WVSCE", "WCDSBS", "WNOFFFD", "WVOFFFD", "WK1B",
    "WK2B", "WDK2B", "WDVBD0", "WDVBD1", "WMOINFD", "WVBS0PD", "WVBS0FD", "PXJ", "PALPHAGB1", "PALPHAGB1_T", "PBETAGB1", "PALPHAGB2", "PALPHAGB2_T", "PBETAGB2", "PAIGBCP2", "PAIGBCP2_T",
    "PBIGBCP2", "PCIGBCP2", "PCGSL", "PCGDL", "PCKAPPA", "PNDIF", "PUTE", "PKT1", "PKT1L", "PKT2", "PUA1", "PUB1", "PUC1", "PAT", "PPRT", "PNTRECF",
    "PNTRECR", "PXBJT", "PXDIF", "PXREC", "PXTUN", "PXDIFD", "PXRECD", "PXTUND", "PAIGC", "PAIGC1", "PBIGC", "PCIGC", "PAIGSD", "PAIGSD1", "PBIGSD", "PCIGSD",
    "PNIGC", "PPIGCD", "PPOXEDGE", "PIGT", "PNCH", "PNSUB", "PNSD", "PNGATE", "PVTH0", "PVFB", "PK1", "PK1W1", "PK1W2", "PK2", "PK3", "PK3B",
    "PKB1", "PW0", "PLPEB", "PDVT0", "PDVT1", "PDVT2", "PDVT0W", "PDVT1W", "PDVT2W", "PU0", "PEU", "PUA", "PUB", "PUC", "PUD", "PUD1",
    "PUCSTE", "PUCS", "PVSAT", "PA0", "PAGS", "PB0", "PB1", "PKETA", "PKETAS", "PA1", "PA2", "PRDSW", "PRSW", "PRDW", "PPRWB", "PPRWE",
    "PPRWG", "PWR", "PNFACTOR", "PDWG", "PDWB", "PVOFF", "PETA0", "PETAB", "PETA0CV", "PETABCV", "PDSUB", "PCIT", "PCDSC", "PCDSCB", "PCDSCD", "PPCLM",
    "PPDIBLC1", "PPDIBLC2", "PPDIBLCB", "PDROUT", "PPVAG", "PDELTA", "PALPHA0", "PFBJTII", "PABJTII", "PCBJTII", "PEBJTII", "PMBJTII", "PVBCI", "PBETA0", "PBETA1", "PBETA2",
    "PVDSATII0", "PLII", "PESATII", "PSII0", "PSII1", "PSII2", "PSIID", "PAGIDL", "PBGIDL", "PBGIDL1", "PCGIDL", "PRGIDL", "PKGIDL", "PFGIDL", "PAGISL", "PBGISL",
    "PBGISL1", "PCGISL", "PRGISL", "PKGISL", "PFGISL", "PNTUN", "PNTUND", "PNDIODE", "PNDIODED", "PNRECF0", "PNRECF0D", "PNRECR0", "PNRECR0D", "PISBJT", "PIDBJT", "PISDIF",
    "PIDDIF", "PISREC", "PIDREC", "PISTUN", "PIDTUN", "PVREC0", "PVREC0D", "PVTUN0", "PVTUN0D", "PNBJT", "PLBJT0", "PVABJT", "PAELY", "PAHLI", "PAHLID", "PVSDFB",
    "PVSDTH", "PDELVT", "PACDE", "PMOIN", "PNOFF", "PNOFF2", "PXRCRG1", "PXRCRG2", "PVBSA", "PVSCE", "PCDSBS", "PNOFFFD", "PVOFFFD", "PK1B", "PK2B", "PDK2B",
    "PDVBD0", "PDVBD1", "PMOINFD", "PVBS0PD", "PVBS0FD", "NLX", "LNLX", "WNLX", "PNLX", "NGIDL", "LNGIDL", "WNGIDL", "PNGIDL", "LPE0", "EGIDL", "EGISL",
    "LLPE0", "LEGIDL", "LEGISL", "WLPE0", "WEGIDL", "WEGISL", "PLPE0", "PEGIDL", "PEGISL", "EGGBCP2", "EGGDEP", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N",
    "AGBC2P", "BGBC2N", "BGBC2P", "VTM00",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1044] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1044] = [
    false, false, false, true, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, true, true, true, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true,
    true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1044] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
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
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1044] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
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
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
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
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None,
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
    None, None, None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1044] = [
    0, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3,
    2, 2, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 2, 3, 2, 2, 2, 2, 2, 0, 3, 0, 0, 3, 2, 2, 3, 0, 0, 0,
    0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 2, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,
    2, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 3, 3, 0, 0, 2, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2,
    0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 2, 2, 0, 3, 0, 3, 0,
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
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
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1044] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[],
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
    pub nodes: [usize; 14],
    pub branches: [usize; 19],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1044]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 22]>,
    pub(crate) ddt_state_previous: Box<[f64; 22]>,
    pub(crate) ddt_state_older: Box<[f64; 22]>,
    pub(crate) ddt_state_initialized: Box<[bool; 22]>,
    pub(crate) ddt_derivative_current: Box<[f64; 22]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 22]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<1569, 14, 19>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<1569, 14, 19>>>,
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
    pub const TERMINAL_COUNT: usize = 7;
    pub const INTERNAL_NODE_COUNT: usize = 7;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 7] = ["di", "si", "gi", "gm", "sb", "db", "N"];

    pub const BRANCH_COUNT: usize = 19;
    pub const PARAMETER_COUNT: usize = 1044;
    pub const VARIABLE_COUNT: usize = 1569;
    pub const DDT_STATE_COUNT: usize = 22;
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimsoi'", name));
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
    #[inline]
    pub fn limiter_converged(&self) -> bool {
        true
    }
}
