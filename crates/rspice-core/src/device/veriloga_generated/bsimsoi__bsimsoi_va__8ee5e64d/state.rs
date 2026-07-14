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
    pub p992: f64, pub p993: f64, pub p994: f64, pub p995: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 30] = [
                0.0, 5e-6, 5e-6, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 1e-5, 1.0, 1.0, 50.0, 50.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 30);
            {
                let params = &mut *ptr;
                params.p30 = params.p28;
                validate_parameter("AGBCPD", params.p30, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 9] = [
                0.0, 0.0, 0.0, 1.0, 4.6, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(31), 9);
            {
                let params = &mut *ptr;
                params.p40 = if (params.p35 >= 4.2) { 1.0 } else { 0.0 };
                validate_parameter("VGSTCVMOD", params.p40, true, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 13] = [
                0.0, 0.0, 1e-8, 3.9, 11.7, 14500000000.0, 1.16, 0.000702,
                1108.0, 4.05, 4.05, 1.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(41), 13);
            {
                let params = &mut *ptr;
                params.p54 = if (params.p34 == 1.0) { 1.5 } else { (-1.5) };
                validate_finite_parameter("VDDEOT", params.p54).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 10] = [
                300.15, 1.0, 1.0, 11.7, 2.0, 1.0, 0.0, 1.0,
                1.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(55), 10);
            {
                let params = &mut *ptr;
                params.p65 = params.p64;
                validate_parameter("TOXP", params.p65, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p66 = params.p64;
                validate_parameter("TOXM", params.p66, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 38] = [
                0.0, 0.00024, 0.0, 0.0, 0.0, 1.0, 80000.0, 33000.0,
                1.0, 0.0, 0.0, 1.0, -0.6, 6e16, 1.7e17, 0.0,
                1e20, 0.0, 0.0, 0.0, -3.0, 1.55e-7, 0.53, -0.11,
                0.0, 0.022, -0.0186, 0.0, 0.0, 2.5e-6, 0.0, 2.2,
                0.53, -0.032, 0.0, 5300000.0, -0.032, 0.56,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(67), 38);
            {
                let params = &mut *ptr;
                params.p105 = params.p104;
                validate_finite_parameter("DSUB", params.p105).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p106 = if (params.p34 == 1.0) { 0.7 } else { (-0.7) };
                validate_finite_parameter("VTHO", params.p106).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p107 = params.p106;
                validate_finite_parameter("VTH0", params.p107).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 5] = [
                -1.0, 2.25e-9, 4.31e-9, 5.87e-19, -7.61e-18,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(108), 5);
            {
                let params = &mut *ptr;
                params.p113 = if (params.p60 == 3.0) { (-0.0465) } else { (-4.65e-11) };
                validate_finite_parameter("UC", params.p113).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p114 = if (params.p60 == 3.0) { (-0.056) } else { (-5.6e-11) };
                validate_finite_parameter("UC1", params.p114).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p115 = if (params.p34 == 1.0) { 0.067 } else { 0.025 };
                validate_finite_parameter("U0", params.p115).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p116 = if (params.p34 == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("EU", params.p116).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 1] = [
                -1.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(117), 1);
            {
                let params = &mut *ptr;
                params.p118 = if (params.p34 == 1.0) { 1.67 } else { 1.0 };
                validate_finite_parameter("UCS", params.p118).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 20] = [
                -0.004775, 0.0, 0.0, -0.08, 27.0, 0.0, 0.0, 0.0,
                0.01, 0.0, 100.0, 50.0, 50.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.08, -0.07,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(119), 20);
            {
                let params = &mut *ptr;
                params.p139 = params.p137;
                validate_finite_parameter("ETA0CV", params.p139).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p140 = params.p138;
                validate_finite_parameter("ETABCV", params.p140).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 8] = [
                1.3, 0.39, 0.0086, 0.0, 0.0, 3e-7, 1e-7, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(141), 8);
            {
                let params = &mut *ptr;
                params.p149 = params.p147;
                validate_parameter("XJ", params.p149, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 6] = [
                0.0, 2300000000.0, 0.5, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(150), 6);
            {
                let params = &mut *ptr;
                params.p156 = params.p150;
                validate_finite_parameter("AGISL", params.p156).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p157 = params.p151;
                validate_finite_parameter("BGISL", params.p157).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p158 = params.p152;
                validate_finite_parameter("CGISL", params.p158).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p159 = params.p153;
                validate_finite_parameter("RGISL", params.p159).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p160 = params.p154;
                validate_finite_parameter("KGISL", params.p160).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p161 = params.p155;
                validate_finite_parameter("FGISL", params.p161).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(162), 1);
            {
                let params = &mut *ptr;
                params.p163 = params.p162;
                validate_finite_parameter("NDIODED", params.p163).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(164), 1);
            {
                let params = &mut *ptr;
                params.p165 = params.p164;
                validate_finite_parameter("XDIF", params.p165).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(166), 2);
            {
                let params = &mut *ptr;
                params.p168 = params.p165;
                validate_finite_parameter("XDIFD", params.p168).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p169 = params.p166;
                validate_finite_parameter("XRECD", params.p169).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p170 = params.p167;
                validate_finite_parameter("XTUND", params.p170).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 1] = [
                0.7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(171), 1);
            {
                let params = &mut *ptr;
                params.p172 = params.p171;
                validate_finite_parameter("PBSWGD", params.p172).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(173), 1);
            {
                let params = &mut *ptr;
                params.p174 = params.p173;
                validate_finite_parameter("MJSWGD", params.p174).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                1e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(175), 1);
            {
                let params = &mut *ptr;
                params.p176 = params.p175;
                validate_parameter("CJSWGD", params.p176, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 29] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.6, 0.0, 1e-8, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(177), 29);
            {
                let params = &mut *ptr;
                params.p206 = params.p187;
                validate_finite_parameter("DWC", params.p206).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p207 = params.p177;
                validate_finite_parameter("DLC", params.p207).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(208), 1);
            {
                let params = &mut *ptr;
                params.p209 = if (params.p34 == 1.0) { 6.25e41 } else { 6.188e40 };
                validate_finite_parameter("NOIA", params.p209).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p210 = if (params.p34 == 1.0) { 3.125e26 } else { 1.5e25 };
                validate_finite_parameter("NOIB", params.p210).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 29] = [
                8750000000.0, 1.0, 0.0, 1.5, 3.5, 0.577, 0.37, 1.0,
                1e-6, 1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(211), 29);
            {
                let params = &mut *ptr;
                params.p240 = params.p238;
                validate_finite_parameter("STETA0CV", params.p240).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p241 = params.p239;
                validate_finite_parameter("LODETA0CV", params.p241).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 68] = [
                1e-12, 2.0, 1e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1e-20, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                41000000.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.1, 0.9, 0.0, 0.0, 0.5,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 10000000.0, 10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(242), 68);
            {
                let params = &mut *ptr;
                params.p310 = params.p309;
                validate_parameter("NTUND", params.p310, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(311), 1);
            {
                let params = &mut *ptr;
                params.p312 = params.p311;
                validate_parameter("NRECF0D", params.p312, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(313), 1);
            {
                let params = &mut *ptr;
                params.p314 = params.p313;
                validate_parameter("NRECR0D", params.p314, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                1e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(315), 1);
            {
                let params = &mut *ptr;
                params.p316 = params.p315;
                validate_parameter("IDBJT", params.p316, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(317), 1);
            {
                let params = &mut *ptr;
                params.p318 = params.p317;
                validate_parameter("IDDIF", params.p318, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 1] = [
                1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(319), 1);
            {
                let params = &mut *ptr;
                params.p320 = params.p319;
                validate_parameter("IDREC", params.p320, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(321), 1);
            {
                let params = &mut *ptr;
                params.p322 = params.p321;
                validate_parameter("IDTUN", params.p322, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 2] = [
                2e-6, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(323), 2);
            {
                let params = &mut *ptr;
                params.p325 = params.p324;
                validate_finite_parameter("VREC0D", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(326), 1);
            {
                let params = &mut *ptr;
                params.p327 = params.p326;
                validate_finite_parameter("VTUN0D", params.p327).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 6] = [
                1.0, 2e-7, 1.0, 10.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(328), 6);
            {
                let params = &mut *ptr;
                params.p334 = params.p333;
                validate_finite_parameter("AHLID", params.p334).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 16] = [
                0.0, 0.0, 0.0, 1e-12, -1.0, 0.0, 0.0, 0.0,
                0.3, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(335), 16);
            {
                let params = &mut *ptr;
                params.p351 = params.p349;
                validate_finite_parameter("TCJSWGD", params.p351).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p352 = params.p350;
                validate_finite_parameter("TPBSWGD", params.p352).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 3] = [
                1.0, 15.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(353), 3);
            {
                let params = &mut *ptr;
                params.p356 = params.p355;
                validate_parameter("NOFF2", params.p356, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 5] = [
                0.0, 1.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(357), 5);
            {
                let params = &mut *ptr;
                params.p362 = params.p361;
                validate_parameter("IGMOD", params.p362, true, None, false, None, false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(363), 1);
            {
                let params = &mut *ptr;
                params.p364 = params.p64;
                validate_finite_parameter("TOXQM", params.p364).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 18] = [
                0.0, 1000000000000000.0, 1.0, 2.5e-9, 1.2, 0.075, 0.35, 0.03,
                300.0, 0.026, 0.43, 0.05, 17.0, 0.043, 0.0054, 0.0075,
                5.0, 0.005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(365), 18);
            {
                let params = &mut *ptr;
                params.p383 = if (params.p34 == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGC", params.p383).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p384 = if (params.p34 == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGC", params.p384).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p385 = if (params.p34 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params.p385).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p386 = if (params.p34 == 1.0) { 0.43 } else { 0.31 };
                validate_finite_parameter("AIGSD", params.p386).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p387 = if (params.p34 == 1.0) { 0.054 } else { 0.024 };
                validate_finite_parameter("BIGSD", params.p387).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p388 = if (params.p34 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGSD", params.p388).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 3] = [
                1.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(389), 3);
            {
                let params = &mut *ptr;
                params.p392 = params.p177;
                validate_finite_parameter("DLCIG", params.p392).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 56] = [
                0.0, 0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1000.0, 12.0, 1.0, 0.1, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(393), 56);
            {
                let params = &mut *ptr;
                params.p449 = params.p446;
                validate_finite_parameter("LXDIFD", params.p449).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p450 = params.p447;
                validate_finite_parameter("LXRECD", params.p450).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p451 = params.p448;
                validate_finite_parameter("LXTUND", params.p451).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(452), 60);
            {
                let params = &mut *ptr;
                params.p512 = params.p510;
                validate_finite_parameter("LETA0CV", params.p512).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p513 = params.p511;
                validate_finite_parameter("LETABCV", params.p513).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(514), 35);
            {
                let params = &mut *ptr;
                params.p549 = params.p543;
                validate_finite_parameter("LAGISL", params.p549).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p550 = params.p544;
                validate_finite_parameter("LBGISL", params.p550).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p551 = params.p545;
                validate_finite_parameter("LCGISL", params.p551).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p552 = params.p546;
                validate_finite_parameter("LRGISL", params.p552).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p553 = params.p547;
                validate_finite_parameter("LKGISL", params.p553).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p554 = params.p548;
                validate_finite_parameter("LFGISL", params.p554).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(555), 1);
            {
                let params = &mut *ptr;
                params.p556 = params.p555;
                validate_finite_parameter("LNTUND", params.p556).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(557), 1);
            {
                let params = &mut *ptr;
                params.p558 = params.p557;
                validate_finite_parameter("LNDIODED", params.p558).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(559), 1);
            {
                let params = &mut *ptr;
                params.p560 = params.p559;
                validate_finite_parameter("LNRECF0D", params.p560).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(561), 1);
            {
                let params = &mut *ptr;
                params.p562 = params.p561;
                validate_finite_parameter("LNRECR0D", params.p562).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (ptr as *mut f64).add(563), 1);
            {
                let params = &mut *ptr;
                params.p564 = params.p563;
                validate_finite_parameter("LIDBJT", params.p564).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (ptr as *mut f64).add(565), 1);
            {
                let params = &mut *ptr;
                params.p566 = params.p565;
                validate_finite_parameter("LIDDIF", params.p566).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (ptr as *mut f64).add(567), 1);
            {
                let params = &mut *ptr;
                params.p568 = params.p567;
                validate_finite_parameter("LIDREC", params.p568).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (ptr as *mut f64).add(569), 1);
            {
                let params = &mut *ptr;
                params.p570 = params.p569;
                validate_finite_parameter("LIDTUN", params.p570).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (ptr as *mut f64).add(571), 1);
            {
                let params = &mut *ptr;
                params.p572 = params.p571;
                validate_finite_parameter("LVREC0D", params.p572).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (ptr as *mut f64).add(573), 1);
            {
                let params = &mut *ptr;
                params.p574 = params.p573;
                validate_finite_parameter("LVTUN0D", params.p574).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (ptr as *mut f64).add(575), 5);
            {
                let params = &mut *ptr;
                params.p580 = params.p579;
                validate_finite_parameter("LAHLID", params.p580).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (ptr as *mut f64).add(581), 6);
            {
                let params = &mut *ptr;
                params.p587 = params.p586;
                validate_finite_parameter("LNOFF2", params.p587).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 42] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (ptr as *mut f64).add(588), 42);
            {
                let params = &mut *ptr;
                params.p630 = params.p627;
                validate_finite_parameter("WXDIFD", params.p630).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p631 = params.p628;
                validate_finite_parameter("WXRECD", params.p631).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p632 = params.p629;
                validate_finite_parameter("WXTUND", params.p632).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (ptr as *mut f64).add(633), 60);
            {
                let params = &mut *ptr;
                params.p693 = params.p691;
                validate_finite_parameter("WETA0CV", params.p693).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p694 = params.p692;
                validate_finite_parameter("WETABCV", params.p694).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (ptr as *mut f64).add(695), 35);
            {
                let params = &mut *ptr;
                params.p730 = params.p724;
                validate_finite_parameter("WAGISL", params.p730).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p731 = params.p725;
                validate_finite_parameter("WBGISL", params.p731).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p732 = params.p726;
                validate_finite_parameter("WCGISL", params.p732).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p733 = params.p727;
                validate_finite_parameter("WRGISL", params.p733).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p734 = params.p728;
                validate_finite_parameter("WKGISL", params.p734).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p735 = params.p729;
                validate_finite_parameter("WFGISL", params.p735).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (ptr as *mut f64).add(736), 1);
            {
                let params = &mut *ptr;
                params.p737 = params.p736;
                validate_finite_parameter("WNTUND", params.p737).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (ptr as *mut f64).add(738), 1);
            {
                let params = &mut *ptr;
                params.p739 = params.p738;
                validate_finite_parameter("WNDIODED", params.p739).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (ptr as *mut f64).add(740), 1);
            {
                let params = &mut *ptr;
                params.p741 = params.p740;
                validate_finite_parameter("WNRECF0D", params.p741).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (ptr as *mut f64).add(742), 1);
            {
                let params = &mut *ptr;
                params.p743 = params.p742;
                validate_finite_parameter("WNRECR0D", params.p743).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (ptr as *mut f64).add(744), 1);
            {
                let params = &mut *ptr;
                params.p745 = params.p744;
                validate_finite_parameter("WIDBJT", params.p745).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (ptr as *mut f64).add(746), 1);
            {
                let params = &mut *ptr;
                params.p747 = params.p746;
                validate_finite_parameter("WIDDIF", params.p747).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (ptr as *mut f64).add(748), 1);
            {
                let params = &mut *ptr;
                params.p749 = params.p748;
                validate_finite_parameter("WIDREC", params.p749).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (ptr as *mut f64).add(750), 1);
            {
                let params = &mut *ptr;
                params.p751 = params.p750;
                validate_finite_parameter("WIDTUN", params.p751).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (ptr as *mut f64).add(752), 1);
            {
                let params = &mut *ptr;
                params.p753 = params.p752;
                validate_finite_parameter("WVREC0D", params.p753).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (ptr as *mut f64).add(754), 1);
            {
                let params = &mut *ptr;
                params.p755 = params.p754;
                validate_finite_parameter("WVTUN0D", params.p755).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (ptr as *mut f64).add(756), 5);
            {
                let params = &mut *ptr;
                params.p761 = params.p760;
                validate_finite_parameter("WAHLID", params.p761).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (ptr as *mut f64).add(762), 6);
            {
                let params = &mut *ptr;
                params.p768 = params.p767;
                validate_finite_parameter("WNOFF2", params.p768).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 42] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (ptr as *mut f64).add(769), 42);
            {
                let params = &mut *ptr;
                params.p811 = params.p808;
                validate_finite_parameter("PXDIFD", params.p811).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p812 = params.p809;
                validate_finite_parameter("PXRECD", params.p812).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p813 = params.p810;
                validate_finite_parameter("PXTUND", params.p813).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 60] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (ptr as *mut f64).add(814), 60);
            {
                let params = &mut *ptr;
                params.p874 = params.p872;
                validate_finite_parameter("PETA0CV", params.p874).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p875 = params.p873;
                validate_finite_parameter("PETABCV", params.p875).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (ptr as *mut f64).add(876), 35);
            {
                let params = &mut *ptr;
                params.p911 = params.p905;
                validate_finite_parameter("PAGISL", params.p911).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p912 = params.p906;
                validate_finite_parameter("PBGISL", params.p912).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p913 = params.p907;
                validate_finite_parameter("PCGISL", params.p913).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p914 = params.p908;
                validate_finite_parameter("PRGISL", params.p914).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p915 = params.p909;
                validate_finite_parameter("PKGISL", params.p915).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p916 = params.p910;
                validate_finite_parameter("PFGISL", params.p916).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (ptr as *mut f64).add(917), 1);
            {
                let params = &mut *ptr;
                params.p918 = params.p917;
                validate_finite_parameter("PNTUND", params.p918).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (ptr as *mut f64).add(919), 1);
            {
                let params = &mut *ptr;
                params.p920 = params.p919;
                validate_finite_parameter("PNDIODED", params.p920).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (ptr as *mut f64).add(921), 1);
            {
                let params = &mut *ptr;
                params.p922 = params.p921;
                validate_finite_parameter("PNRECF0D", params.p922).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (ptr as *mut f64).add(923), 1);
            {
                let params = &mut *ptr;
                params.p924 = params.p923;
                validate_finite_parameter("PNRECR0D", params.p924).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (ptr as *mut f64).add(925), 1);
            {
                let params = &mut *ptr;
                params.p926 = params.p925;
                validate_finite_parameter("PIDBJT", params.p926).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (ptr as *mut f64).add(927), 1);
            {
                let params = &mut *ptr;
                params.p928 = params.p927;
                validate_finite_parameter("PIDDIF", params.p928).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (ptr as *mut f64).add(929), 1);
            {
                let params = &mut *ptr;
                params.p930 = params.p929;
                validate_finite_parameter("PIDREC", params.p930).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (ptr as *mut f64).add(931), 1);
            {
                let params = &mut *ptr;
                params.p932 = params.p931;
                validate_finite_parameter("PIDTUN", params.p932).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (ptr as *mut f64).add(933), 1);
            {
                let params = &mut *ptr;
                params.p934 = params.p933;
                validate_finite_parameter("PVREC0D", params.p934).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (ptr as *mut f64).add(935), 1);
            {
                let params = &mut *ptr;
                params.p936 = params.p935;
                validate_finite_parameter("PVTUN0D", params.p936).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (ptr as *mut f64).add(937), 5);
            {
                let params = &mut *ptr;
                params.p942 = params.p941;
                validate_finite_parameter("PAHLID", params.p942).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (ptr as *mut f64).add(943), 6);
            {
                let params = &mut *ptr;
                params.p949 = params.p948;
                validate_finite_parameter("PNOFF2", params.p949).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 23] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.74e-7,
                0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (ptr as *mut f64).add(950), 23);
            {
                let params = &mut *ptr;
                params.p973 = params.p965;
                validate_finite_parameter("LPE0", params.p973).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p974 = params.p969;
                validate_finite_parameter("EGIDL", params.p974).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p975 = params.p974;
                validate_finite_parameter("EGISL", params.p975).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p976 = params.p966;
                validate_finite_parameter("LLPE0", params.p976).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p977 = params.p970;
                validate_finite_parameter("LEGIDL", params.p977).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p978 = params.p977;
                validate_finite_parameter("LEGISL", params.p978).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p979 = params.p967;
                validate_finite_parameter("WLPE0", params.p979).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p980 = params.p971;
                validate_finite_parameter("WEGIDL", params.p980).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p981 = params.p980;
                validate_finite_parameter("WEGISL", params.p981).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p982 = params.p968;
                validate_finite_parameter("PLPE0", params.p982).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p983 = params.p972;
                validate_finite_parameter("PEGIDL", params.p983).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p984 = params.p983;
                validate_finite_parameter("PEGISL", params.p984).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 11] = [
                1.12, 1.12, 3.7622e-7, -31051000000.0, 4.9758e-7, -23570000000.0, 3.4254e-7, 4.9723e-7,
                1166500000000.0, 745670000000.0, 0.026,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (ptr as *mut f64).add(985), 11);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 996] = [
    ("dtemp", 0), ("l", 1), ("w", 2), ("nf", 3), ("sa", 4), ("sb", 5), ("sd", 6), ("ad", 7), ("as", 8), ("pd", 9), ("ps", 10), ("nrd", 11), ("nrs", 12), ("off", 13), ("bjtoff", 14), ("debug", 15),
    ("rth0", 16), ("cth0", 17), ("nrb", 18), ("frbody", 19), ("rbdb", 20), ("rbsb", 21), ("delvto", 22), ("soimod", 23), ("nbc", 24), ("nseg", 25), ("pdbcp", 26), ("psbcp", 27), ("agbcp", 28), ("agbcp2", 29), ("agbcpd", 30), ("aebcp", 31),
    ("tnodeout", 32), ("shmod", 33), ("type", 34), ("version", 35), ("vbsusr", 36), ("rgatemod", 37), ("rbodymod", 38), ("mtrlmod", 39), ("vgstcvmod", 40), ("gidlmod", 41), ("iiimod", 42), ("eot", 43), ("epsrox", 44), ("epsrsub", 45), ("ni0sub", 46), ("bg0sub", 47),
    ("tbgasub", 48), ("tbgbsub", 49), ("phig", 50), ("easub", 51), ("leffeot", 52), ("weffeot", 53), ("vddeot", 54), ("tempeot", 55), ("ados", 56), ("bdos", 57), ("epsrgate", 58), ("capmod", 59), ("mobmod", 60), ("paramchk", 61), ("nodechk", 62), ("binunit", 63),
    ("tox", 64), ("toxp", 65), ("toxm", 66), ("dtoxcv", 67), ("cdsc", 68), ("cdscb", 69), ("cdscd", 70), ("cit", 71), ("nfactor", 72), ("vsat", 73), ("at", 74), ("a0", 75), ("ags", 76), ("a1", 77), ("a2", 78), ("keta", 79),
    ("nsub", 80), ("nch", 81), ("ngate", 82), ("nsd", 83), ("gamma1", 84), ("gamma2", 85), ("vbx", 86), ("vbm", 87), ("xt", 88), ("k1", 89), ("kt1", 90), ("kt1l", 91), ("kt2", 92), ("k2", 93), ("k3", 94), ("k3b", 95),
    ("w0", 96), ("lpeb", 97), ("dvt0", 98), ("dvt1", 99), ("dvt2", 100), ("dvt0w", 101), ("dvt1w", 102), ("dvt2w", 103), ("drout", 104), ("dsub", 105), ("vtho", 106), ("vth0", 107), ("vfb", 108), ("ua", 109), ("ua1", 110), ("ub", 111),
    ("ub1", 112), ("uc", 113), ("uc1", 114), ("u0", 115), ("eu", 116), ("ute", 117), ("ucs", 118), ("ucste", 119), ("ud", 120), ("ud1", 121), ("voff", 122), ("tnom", 123), ("cgso", 124), ("cgdo", 125), ("xpart", 126), ("delta", 127),
    ("rsh", 128), ("rdsw", 129), ("rsw", 130), ("rdw", 131), ("rswmin", 132), ("rdwmin", 133), ("prwg", 134), ("prwb", 135), ("prt", 136), ("eta0", 137), ("etab", 138), ("eta0cv", 139), ("etabcv", 140), ("pclm", 141), ("pdiblc1", 142), ("pdiblc2", 143),
    ("pdiblcb", 144), ("pvag", 145), ("tbox", 146), ("tsi", 147), ("etsi", 148), ("xj", 149), ("agidl", 150), ("bgidl", 151), ("cgidl", 152), ("rgidl", 153), ("kgidl", 154), ("fgidl", 155), ("agisl", 156), ("bgisl", 157), ("cgisl", 158), ("rgisl", 159),
    ("kgisl", 160), ("fgisl", 161), ("ndiode", 162), ("ndioded", 163), ("xbjt", 164), ("xdif", 165), ("xrec", 166), ("xtun", 167), ("xdifd", 168), ("xrecd", 169), ("xtund", 170), ("pbswg", 171), ("pbswgd", 172), ("mjswg", 173), ("mjswgd", 174), ("cjswg", 175),
    ("cjswgd", 176), ("lint", 177), ("ll", 178), ("llc", 179), ("lln", 180), ("lw", 181), ("lwc", 182), ("lwn", 183), ("lwl", 184), ("lwlc", 185), ("wr", 186), ("wint", 187), ("dwg", 188), ("dwb", 189), ("wl", 190), ("wlc", 191),
    ("wln", 192), ("ww", 193), ("wwc", 194), ("wwn", 195), ("wwl", 196), ("wwlc", 197), ("b0", 198), ("b1", 199), ("cgsl", 200), ("cgdl", 201), ("ckappa", 202), ("cf", 203), ("clc", 204), ("cle", 205), ("dwc", 206), ("dlc", 207),
    ("alpha0", 208), ("noia", 209), ("noib", 210), ("noic", 211), ("fnoimod", 212), ("tnoimod", 213), ("tnoia", 214), ("tnoib", 215), ("rnoia", 216), ("rnoib", 217), ("ntnoi", 218), ("saref", 219), ("sbref", 220), ("wlod", 221), ("ku0", 222), ("kvsat", 223),
    ("kvth0", 224), ("tku0", 225), ("llodku0", 226), ("wlodku0", 227), ("llodvth", 228), ("wlodvth", 229), ("lku0", 230), ("wku0", 231), ("pku0", 232), ("lkvth0", 233), ("wkvth0", 234), ("pkvth0", 235), ("stk2", 236), ("lodk2", 237), ("steta0", 238), ("lodeta0", 239),
    ("steta0cv", 240), ("lodeta0cv", 241), ("gbmin", 242), ("bf", 243), ("w0flk", 244), ("dvtp0", 245), ("ldvtp0", 246), ("wdvtp0", 247), ("pdvtp0", 248), ("dvtp1", 249), ("ldvtp1", 250), ("wdvtp1", 251), ("pdvtp1", 252), ("dvtp2", 253), ("ldvtp2", 254), ("wdvtp2", 255),
    ("pdvtp2", 256), ("dvtp3", 257), ("ldvtp3", 258), ("wdvtp3", 259), ("pdvtp3", 260), ("dvtp4", 261), ("ldvtp4", 262), ("wdvtp4", 263), ("pdvtp4", 264), ("minv", 265), ("lminv", 266), ("wminv", 267), ("pminv", 268), ("pdits", 269), ("pditsl", 270), ("pditsd", 271),
    ("fprout", 272), ("lfprout", 273), ("lpdits", 274), ("lpditsd", 275), ("wfprout", 276), ("wpdits", 277), ("wpditsd", 278), ("pfprout", 279), ("ppdits", 280), ("ppditsd", 281), ("em", 282), ("ef", 283), ("af", 284), ("kf", 285), ("noif", 286), ("k1w1", 287),
    ("k1w2", 288), ("ketas", 289), ("dwbc", 290), ("beta0", 291), ("beta1", 292), ("beta2", 293), ("vdsatii0", 294), ("tii", 295), ("lii", 296), ("sii0", 297), ("sii1", 298), ("sii2", 299), ("siid", 300), ("fbjtii", 301), ("ebjtii", 302), ("cbjtii", 303),
    ("vbci", 304), ("abjtii", 305), ("mbjtii", 306), ("tvbci", 307), ("esatii", 308), ("ntun", 309), ("ntund", 310), ("nrecf0", 311), ("nrecf0d", 312), ("nrecr0", 313), ("nrecr0d", 314), ("isbjt", 315), ("idbjt", 316), ("isdif", 317), ("iddif", 318), ("isrec", 319),
    ("idrec", 320), ("istun", 321), ("idtun", 322), ("ln", 323), ("vrec0", 324), ("vrec0d", 325), ("vtun0", 326), ("vtun0d", 327), ("nbjt", 328), ("lbjt0", 329), ("ldif0", 330), ("vabjt", 331), ("aely", 332), ("ahli", 333), ("ahlid", 334), ("rbody", 335),
    ("rbsh", 336), ("cgeo", 337), ("tt", 338), ("ndif", 339), ("vsdfb", 340), ("vsdth", 341), ("csdmin", 342), ("asd", 343), ("csdesw", 344), ("ntrecf", 345), ("ntrecr", 346), ("dlcb", 347), ("fbody", 348), ("tcjswg", 349), ("tpbswg", 350), ("tcjswgd", 351),
    ("tpbswgd", 352), ("acde", 353), ("moin", 354), ("noff", 355), ("noff2", 356), ("delvt", 357), ("kb1", 358), ("dlbg", 359), ("cfrcoeff", 360), ("igbmod", 361), ("igmod", 362), ("igcmod", 363), ("toxqm", 364), ("wth0", 365), ("rhalo", 366), ("ntox", 367),
    ("toxref", 368), ("ebg", 369), ("vevb", 370), ("alphagb1", 371), ("betagb1", 372), ("vgb1", 373), ("vecb", 374), ("alphagb2", 375), ("betagb2", 376), ("vgb2", 377), ("aigbcp2", 378), ("bigbcp2", 379), ("cigbcp2", 380), ("voxh", 381), ("deltavox", 382), ("aigc", 383),
    ("bigc", 384), ("cigc", 385), ("aigsd", 386), ("bigsd", 387), ("cigsd", 388), ("nigc", 389), ("pigcd", 390), ("poxedge", 391), ("dlcig", 392), ("vbs0pd", 393), ("vbs0fd", 394), ("vbsa", 395), ("nofffd", 396), ("vofffd", 397), ("k1b", 398), ("k2b", 399),
    ("dk2b", 400), ("dvbd0", 401), ("dvbd1", 402), ("moinfd", 403), ("xrcrg1", 404), ("xrcrg2", 405), ("rshg", 406), ("ngcon", 407), ("xgw", 408), ("xgl", 409), ("rdsmod", 410), ("fdmod", 411), ("vsce", 412), ("cdsbs", 413), ("minvcv", 414), ("lminvcv", 415),
    ("wminvcv", 416), ("pminvcv", 417), ("voffcv", 418), ("lvoffcv", 419), ("wvoffcv", 420), ("pvoffcv", 421), ("lxj", 422), ("lalphagb1", 423), ("lbetagb1", 424), ("lalphagb2", 425), ("lbetagb2", 426), ("laigbcp2", 427), ("lbigbcp2", 428), ("lcigbcp2", 429), ("lcgsl", 430), ("lcgdl", 431),
    ("lckappa", 432), ("lndif", 433), ("lute", 434), ("lkt1", 435), ("lkt1l", 436), ("lkt2", 437), ("lua1", 438), ("lub1", 439), ("luc1", 440), ("lat", 441), ("lprt", 442), ("lntrecf", 443), ("lntrecr", 444), ("lxbjt", 445), ("lxdif", 446), ("lxrec", 447),
    ("lxtun", 448), ("lxdifd", 449), ("lxrecd", 450), ("lxtund", 451), ("laigc", 452), ("lbigc", 453), ("lcigc", 454), ("laigsd", 455), ("lbigsd", 456), ("lcigsd", 457), ("lnigc", 458), ("lpigcd", 459), ("lpoxedge", 460), ("lnch", 461), ("lnsub", 462), ("lngate", 463),
    ("lnsd", 464), ("lvth0", 465), ("lvfb", 466), ("lk1", 467), ("lk1w1", 468), ("lk1w2", 469), ("lk2", 470), ("lk3", 471), ("lk3b", 472), ("lkb1", 473), ("lw0", 474), ("llpeb", 475), ("ldvt0", 476), ("ldvt1", 477), ("ldvt2", 478), ("ldvt0w", 479),
    ("ldvt1w", 480), ("ldvt2w", 481), ("lu0", 482), ("leu", 483), ("lua", 484), ("lub", 485), ("luc", 486), ("lud", 487), ("lud1", 488), ("lucste", 489), ("lucs", 490), ("lvsat", 491), ("la0", 492), ("lags", 493), ("lb0", 494), ("lb1", 495),
    ("lketa", 496), ("lketas", 497), ("la1", 498), ("la2", 499), ("lrdsw", 500), ("lrsw", 501), ("lrdw", 502), ("lprwb", 503), ("lprwg", 504), ("lwr", 505), ("lnfactor", 506), ("ldwg", 507), ("ldwb", 508), ("lvoff", 509), ("leta0", 510), ("letab", 511),
    ("leta0cv", 512), ("letabcv", 513), ("ldsub", 514), ("lcit", 515), ("lcdsc", 516), ("lcdscb", 517), ("lcdscd", 518), ("lpclm", 519), ("lpdiblc1", 520), ("lpdiblc2", 521), ("lpdiblcb", 522), ("ldrout", 523), ("lpvag", 524), ("ldelta", 525), ("lalpha0", 526), ("lfbjtii", 527),
    ("labjtii", 528), ("lcbjtii", 529), ("lebjtii", 530), ("lmbjtii", 531), ("lvbci", 532), ("lbeta0", 533), ("lbeta1", 534), ("lbeta2", 535), ("lvdsatii0", 536), ("llii", 537), ("lesatii", 538), ("lsii0", 539), ("lsii1", 540), ("lsii2", 541), ("lsiid", 542), ("lagidl", 543),
    ("lbgidl", 544), ("lcgidl", 545), ("lrgidl", 546), ("lkgidl", 547), ("lfgidl", 548), ("lagisl", 549), ("lbgisl", 550), ("lcgisl", 551), ("lrgisl", 552), ("lkgisl", 553), ("lfgisl", 554), ("lntun", 555), ("lntund", 556), ("lndiode", 557), ("lndioded", 558), ("lnrecf0", 559),
    ("lnrecf0d", 560), ("lnrecr0", 561), ("lnrecr0d", 562), ("lisbjt", 563), ("lidbjt", 564), ("lisdif", 565), ("liddif", 566), ("lisrec", 567), ("lidrec", 568), ("listun", 569), ("lidtun", 570), ("lvrec0", 571), ("lvrec0d", 572), ("lvtun0", 573), ("lvtun0d", 574), ("lnbjt", 575),
    ("llbjt0", 576), ("lvabjt", 577), ("laely", 578), ("lahli", 579), ("lahlid", 580), ("lvsdfb", 581), ("lvsdth", 582), ("ldelvt", 583), ("lacde", 584), ("lmoin", 585), ("lnoff", 586), ("lnoff2", 587), ("lxrcrg1", 588), ("lxrcrg2", 589), ("lvbsa", 590), ("lvsce", 591),
    ("lcdsbs", 592), ("lnofffd", 593), ("lvofffd", 594), ("lk1b", 595), ("lk2b", 596), ("ldk2b", 597), ("ldvbd0", 598), ("ldvbd1", 599), ("lmoinfd", 600), ("lvbs0pd", 601), ("lvbs0fd", 602), ("wxj", 603), ("walphagb1", 604), ("wbetagb1", 605), ("walphagb2", 606), ("wbetagb2", 607),
    ("waigbcp2", 608), ("wbigbcp2", 609), ("wcigbcp2", 610), ("wcgsl", 611), ("wcgdl", 612), ("wckappa", 613), ("wndif", 614), ("wute", 615), ("wkt1", 616), ("wkt1l", 617), ("wkt2", 618), ("wua1", 619), ("wub1", 620), ("wuc1", 621), ("wat", 622), ("wprt", 623),
    ("wntrecf", 624), ("wntrecr", 625), ("wxbjt", 626), ("wxdif", 627), ("wxrec", 628), ("wxtun", 629), ("wxdifd", 630), ("wxrecd", 631), ("wxtund", 632), ("waigc", 633), ("wbigc", 634), ("wcigc", 635), ("waigsd", 636), ("wbigsd", 637), ("wcigsd", 638), ("wnigc", 639),
    ("wpigcd", 640), ("wpoxedge", 641), ("wnch", 642), ("wnsub", 643), ("wngate", 644), ("wnsd", 645), ("wvth0", 646), ("wvfb", 647), ("wk1", 648), ("wk1w1", 649), ("wk1w2", 650), ("wk2", 651), ("wk3", 652), ("wk3b", 653), ("wkb1", 654), ("ww0", 655),
    ("wlpeb", 656), ("wdvt0", 657), ("wdvt1", 658), ("wdvt2", 659), ("wdvt0w", 660), ("wdvt1w", 661), ("wdvt2w", 662), ("wu0", 663), ("weu", 664), ("wua", 665), ("wub", 666), ("wuc", 667), ("wud", 668), ("wud1", 669), ("wucste", 670), ("wucs", 671),
    ("wvsat", 672), ("wa0", 673), ("wags", 674), ("wb0", 675), ("wb1", 676), ("wketa", 677), ("wketas", 678), ("wa1", 679), ("wa2", 680), ("wrdsw", 681), ("wrsw", 682), ("wrdw", 683), ("wprwb", 684), ("wprwg", 685), ("wwr", 686), ("wnfactor", 687),
    ("wdwg", 688), ("wdwb", 689), ("wvoff", 690), ("weta0", 691), ("wetab", 692), ("weta0cv", 693), ("wetabcv", 694), ("wdsub", 695), ("wcit", 696), ("wcdsc", 697), ("wcdscb", 698), ("wcdscd", 699), ("wpclm", 700), ("wpdiblc1", 701), ("wpdiblc2", 702), ("wpdiblcb", 703),
    ("wdrout", 704), ("wpvag", 705), ("wdelta", 706), ("walpha0", 707), ("wfbjtii", 708), ("wabjtii", 709), ("wcbjtii", 710), ("webjtii", 711), ("wmbjtii", 712), ("wvbci", 713), ("wbeta0", 714), ("wbeta1", 715), ("wbeta2", 716), ("wvdsatii0", 717), ("wlii", 718), ("wesatii", 719),
    ("wsii0", 720), ("wsii1", 721), ("wsii2", 722), ("wsiid", 723), ("wagidl", 724), ("wbgidl", 725), ("wcgidl", 726), ("wrgidl", 727), ("wkgidl", 728), ("wfgidl", 729), ("wagisl", 730), ("wbgisl", 731), ("wcgisl", 732), ("wrgisl", 733), ("wkgisl", 734), ("wfgisl", 735),
    ("wntun", 736), ("wntund", 737), ("wndiode", 738), ("wndioded", 739), ("wnrecf0", 740), ("wnrecf0d", 741), ("wnrecr0", 742), ("wnrecr0d", 743), ("wisbjt", 744), ("widbjt", 745), ("wisdif", 746), ("widdif", 747), ("wisrec", 748), ("widrec", 749), ("wistun", 750), ("widtun", 751),
    ("wvrec0", 752), ("wvrec0d", 753), ("wvtun0", 754), ("wvtun0d", 755), ("wnbjt", 756), ("wlbjt0", 757), ("wvabjt", 758), ("waely", 759), ("wahli", 760), ("wahlid", 761), ("wvsdfb", 762), ("wvsdth", 763), ("wdelvt", 764), ("wacde", 765), ("wmoin", 766), ("wnoff", 767),
    ("wnoff2", 768), ("wxrcrg1", 769), ("wxrcrg2", 770), ("wvbsa", 771), ("wvsce", 772), ("wcdsbs", 773), ("wnofffd", 774), ("wvofffd", 775), ("wk1b", 776), ("wk2b", 777), ("wdk2b", 778), ("wdvbd0", 779), ("wdvbd1", 780), ("wmoinfd", 781), ("wvbs0pd", 782), ("wvbs0fd", 783),
    ("pxj", 784), ("palphagb1", 785), ("pbetagb1", 786), ("palphagb2", 787), ("pbetagb2", 788), ("paigbcp2", 789), ("pbigbcp2", 790), ("pcigbcp2", 791), ("pcgsl", 792), ("pcgdl", 793), ("pckappa", 794), ("pndif", 795), ("pute", 796), ("pkt1", 797), ("pkt1l", 798), ("pkt2", 799),
    ("pua1", 800), ("pub1", 801), ("puc1", 802), ("pat", 803), ("pprt", 804), ("pntrecf", 805), ("pntrecr", 806), ("pxbjt", 807), ("pxdif", 808), ("pxrec", 809), ("pxtun", 810), ("pxdifd", 811), ("pxrecd", 812), ("pxtund", 813), ("paigc", 814), ("pbigc", 815),
    ("pcigc", 816), ("paigsd", 817), ("pbigsd", 818), ("pcigsd", 819), ("pnigc", 820), ("ppigcd", 821), ("ppoxedge", 822), ("pnch", 823), ("pnsub", 824), ("pnsd", 825), ("pngate", 826), ("pvth0", 827), ("pvfb", 828), ("pk1", 829), ("pk1w1", 830), ("pk1w2", 831),
    ("pk2", 832), ("pk3", 833), ("pk3b", 834), ("pkb1", 835), ("pw0", 836), ("plpeb", 837), ("pdvt0", 838), ("pdvt1", 839), ("pdvt2", 840), ("pdvt0w", 841), ("pdvt1w", 842), ("pdvt2w", 843), ("pu0", 844), ("peu", 845), ("pua", 846), ("pub", 847),
    ("puc", 848), ("pud", 849), ("pud1", 850), ("pucste", 851), ("pucs", 852), ("pvsat", 853), ("pa0", 854), ("pags", 855), ("pb0", 856), ("pb1", 857), ("pketa", 858), ("pketas", 859), ("pa1", 860), ("pa2", 861), ("prdsw", 862), ("prsw", 863),
    ("prdw", 864), ("pprwb", 865), ("pprwg", 866), ("pwr", 867), ("pnfactor", 868), ("pdwg", 869), ("pdwb", 870), ("pvoff", 871), ("peta0", 872), ("petab", 873), ("peta0cv", 874), ("petabcv", 875), ("pdsub", 876), ("pcit", 877), ("pcdsc", 878), ("pcdscb", 879),
    ("pcdscd", 880), ("ppclm", 881), ("ppdiblc1", 882), ("ppdiblc2", 883), ("ppdiblcb", 884), ("pdrout", 885), ("ppvag", 886), ("pdelta", 887), ("palpha0", 888), ("pfbjtii", 889), ("pabjtii", 890), ("pcbjtii", 891), ("pebjtii", 892), ("pmbjtii", 893), ("pvbci", 894), ("pbeta0", 895),
    ("pbeta1", 896), ("pbeta2", 897), ("pvdsatii0", 898), ("plii", 899), ("pesatii", 900), ("psii0", 901), ("psii1", 902), ("psii2", 903), ("psiid", 904), ("pagidl", 905), ("pbgidl", 906), ("pcgidl", 907), ("prgidl", 908), ("pkgidl", 909), ("pfgidl", 910), ("pagisl", 911),
    ("pbgisl", 912), ("pcgisl", 913), ("prgisl", 914), ("pkgisl", 915), ("pfgisl", 916), ("pntun", 917), ("pntund", 918), ("pndiode", 919), ("pndioded", 920), ("pnrecf0", 921), ("pnrecf0d", 922), ("pnrecr0", 923), ("pnrecr0d", 924), ("pisbjt", 925), ("pidbjt", 926), ("pisdif", 927),
    ("piddif", 928), ("pisrec", 929), ("pidrec", 930), ("pistun", 931), ("pidtun", 932), ("pvrec0", 933), ("pvrec0d", 934), ("pvtun0", 935), ("pvtun0d", 936), ("pnbjt", 937), ("plbjt0", 938), ("pvabjt", 939), ("paely", 940), ("pahli", 941), ("pahlid", 942), ("pvsdfb", 943),
    ("pvsdth", 944), ("pdelvt", 945), ("pacde", 946), ("pmoin", 947), ("pnoff", 948), ("pnoff2", 949), ("pxrcrg1", 950), ("pxrcrg2", 951), ("pvbsa", 952), ("pvsce", 953), ("pcdsbs", 954), ("pnofffd", 955), ("pvofffd", 956), ("pk1b", 957), ("pk2b", 958), ("pdk2b", 959),
    ("pdvbd0", 960), ("pdvbd1", 961), ("pmoinfd", 962), ("pvbs0pd", 963), ("pvbs0fd", 964), ("nlx", 965), ("lnlx", 966), ("wnlx", 967), ("pnlx", 968), ("ngidl", 969), ("lngidl", 970), ("wngidl", 971), ("pngidl", 972), ("lpe0", 973), ("egidl", 974), ("egisl", 975),
    ("llpe0", 976), ("legidl", 977), ("legisl", 978), ("wlpe0", 979), ("wegidl", 980), ("wegisl", 981), ("plpe0", 982), ("pegidl", 983), ("pegisl", 984), ("eggbcp2", 985), ("eggdep", 986), ("agb1", 987), ("bgb1", 988), ("agb2", 989), ("bgb2", 990), ("agbc2n", 991),
    ("agbc2p", 992), ("bgbc2n", 993), ("bgbc2p", 994), ("vtm00", 995),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 996] = [
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

const PARAMETER_MAX_REFERENCES: [Option<usize>; 996] = [
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

const PARAMETER_DISPLAY_NAMES: [&str; 996] = [
    "DTEMP", "L", "W", "NF", "SA", "SB", "SD", "AD", "AS", "PD", "PS", "NRD", "NRS", "OFF", "BJTOFF", "DEBUG",
    "RTH0", "CTH0", "NRB", "FRBODY", "RBDB", "RBSB", "DELVTO", "SOIMOD", "NBC", "NSEG", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP",
    "TNODEOUT", "SHMOD", "TYPE", "VERSION", "VBSUSR", "RGATEMOD", "RBODYMOD", "MTRLMOD", "VGSTCVMOD", "GIDLMOD", "IIIMOD", "EOT", "EPSROX", "EPSRSUB", "NI0SUB", "BG0SUB",
    "TBGASUB", "TBGBSUB", "PHIG", "EASUB", "LEFFEOT", "WEFFEOT", "VDDEOT", "TEMPEOT", "ADOS", "BDOS", "EPSRGATE", "CAPMOD", "MOBMOD", "PARAMCHK", "NODECHK", "BINUNIT",
    "TOX", "TOXP", "TOXM", "DTOXCV", "CDSC", "CDSCB", "CDSCD", "CIT", "NFACTOR", "VSAT", "AT", "A0", "AGS", "A1", "A2", "KETA",
    "NSUB", "NCH", "NGATE", "NSD", "GAMMA1", "GAMMA2", "VBX", "VBM", "XT", "K1", "KT1", "KT1L", "KT2", "K2", "K3", "K3B",
    "W0", "LPEB", "DVT0", "DVT1", "DVT2", "DVT0W", "DVT1W", "DVT2W", "DROUT", "DSUB", "VTHO", "VTH0", "VFB", "UA", "UA1", "UB",
    "UB1", "UC", "UC1", "U0", "EU", "UTE", "UCS", "UCSTE", "UD", "UD1", "VOFF", "TNOM", "CGSO", "CGDO", "XPART", "DELTA",
    "RSH", "RDSW", "RSW", "RDW", "RSWMIN", "RDWMIN", "PRWG", "PRWB", "PRT", "ETA0", "ETAB", "ETA0CV", "ETABCV", "PCLM", "PDIBLC1", "PDIBLC2",
    "PDIBLCB", "PVAG", "TBOX", "TSI", "ETSI", "XJ", "AGIDL", "BGIDL", "CGIDL", "RGIDL", "KGIDL", "FGIDL", "AGISL", "BGISL", "CGISL", "RGISL",
    "KGISL", "FGISL", "NDIODE", "NDIODED", "XBJT", "XDIF", "XREC", "XTUN", "XDIFD", "XRECD", "XTUND", "PBSWG", "PBSWGD", "MJSWG", "MJSWGD", "CJSWG",
    "CJSWGD", "LINT", "LL", "LLC", "LLN", "LW", "LWC", "LWN", "LWL", "LWLC", "WR", "WINT", "DWG", "DWB", "WL", "WLC",
    "WLN", "WW", "WWC", "WWN", "WWL", "WWLC", "B0", "B1", "CGSL", "CGDL", "CKAPPA", "CF", "CLC", "CLE", "DWC", "DLC",
    "ALPHA0", "NOIA", "NOIB", "NOIC", "FNOIMOD", "TNOIMOD", "TNOIA", "TNOIB", "RNOIA", "RNOIB", "NTNOI", "SAREF", "SBREF", "WLOD", "KU0", "KVSAT",
    "KVTH0", "TKU0", "LLODKU0", "WLODKU0", "LLODVTH", "WLODVTH", "LKU0", "WKU0", "PKU0", "LKVTH0", "WKVTH0", "PKVTH0", "STK2", "LODK2", "STETA0", "LODETA0",
    "STETA0CV", "LODETA0CV", "GBMIN", "BF", "W0FLK", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2",
    "PDVTP2", "DVTP3", "LDVTP3", "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "MINV", "LMINV", "WMINV", "PMINV", "PDITS", "PDITSL", "PDITSD",
    "FPROUT", "LFPROUT", "LPDITS", "LPDITSD", "WFPROUT", "WPDITS", "WPDITSD", "PFPROUT", "PPDITS", "PPDITSD", "EM", "EF", "AF", "KF", "NOIF", "K1W1",
    "K1W2", "KETAS", "DWBC", "BETA0", "BETA1", "BETA2", "VDSATII0", "TII", "LII", "SII0", "SII1", "SII2", "SIID", "FBJTII", "EBJTII", "CBJTII",
    "VBCI", "ABJTII", "MBJTII", "TVBCI", "ESATII", "NTUN", "NTUND", "NRECF0", "NRECF0D", "NRECR0", "NRECR0D", "ISBJT", "IDBJT", "ISDIF", "IDDIF", "ISREC",
    "IDREC", "ISTUN", "IDTUN", "LN", "VREC0", "VREC0D", "VTUN0", "VTUN0D", "NBJT", "LBJT0", "LDIF0", "VABJT", "AELY", "AHLI", "AHLID", "RBODY",
    "RBSH", "CGEO", "TT", "NDIF", "VSDFB", "VSDTH", "CSDMIN", "ASD", "CSDESW", "NTRECF", "NTRECR", "DLCB", "FBODY", "TCJSWG", "TPBSWG", "TCJSWGD",
    "TPBSWGD", "ACDE", "MOIN", "NOFF", "NOFF2", "DELVT", "KB1", "DLBG", "CFRCOEFF", "IGBMOD", "IGMOD", "IGCMOD", "TOXQM", "WTH0", "RHALO", "NTOX",
    "TOXREF", "EBG", "VEVB", "ALPHAGB1", "BETAGB1", "VGB1", "VECB", "ALPHAGB2", "BETAGB2", "VGB2", "AIGBCP2", "BIGBCP2", "CIGBCP2", "VOXH", "DELTAVOX", "AIGC",
    "BIGC", "CIGC", "AIGSD", "BIGSD", "CIGSD", "NIGC", "PIGCD", "POXEDGE", "DLCIG", "VBS0PD", "VBS0FD", "VBSA", "NOFFFD", "VOFFFD", "K1B", "K2B",
    "DK2B", "DVBD0", "DVBD1", "MOINFD", "XRCRG1", "XRCRG2", "RSHG", "NGCON", "XGW", "XGL", "RDSMOD", "FDMOD", "VSCE", "CDSBS", "MINVCV", "LMINVCV",
    "WMINVCV", "PMINVCV", "VOFFCV", "LVOFFCV", "WVOFFCV", "PVOFFCV", "LXJ", "LALPHAGB1", "LBETAGB1", "LALPHAGB2", "LBETAGB2", "LAIGBCP2", "LBIGBCP2", "LCIGBCP2", "LCGSL", "LCGDL",
    "LCKAPPA", "LNDIF", "LUTE", "LKT1", "LKT1L", "LKT2", "LUA1", "LUB1", "LUC1", "LAT", "LPRT", "LNTRECF", "LNTRECR", "LXBJT", "LXDIF", "LXREC",
    "LXTUN", "LXDIFD", "LXRECD", "LXTUND", "LAIGC", "LBIGC", "LCIGC", "LAIGSD", "LBIGSD", "LCIGSD", "LNIGC", "LPIGCD", "LPOXEDGE", "LNCH", "LNSUB", "LNGATE",
    "LNSD", "LVTH0", "LVFB", "LK1", "LK1W1", "LK1W2", "LK2", "LK3", "LK3B", "LKB1", "LW0", "LLPEB", "LDVT0", "LDVT1", "LDVT2", "LDVT0W",
    "LDVT1W", "LDVT2W", "LU0", "LEU", "LUA", "LUB", "LUC", "LUD", "LUD1", "LUCSTE", "LUCS", "LVSAT", "LA0", "LAGS", "LB0", "LB1",
    "LKETA", "LKETAS", "LA1", "LA2", "LRDSW", "LRSW", "LRDW", "LPRWB", "LPRWG", "LWR", "LNFACTOR", "LDWG", "LDWB", "LVOFF", "LETA0", "LETAB",
    "LETA0CV", "LETABCV", "LDSUB", "LCIT", "LCDSC", "LCDSCB", "LCDSCD", "LPCLM", "LPDIBLC1", "LPDIBLC2", "LPDIBLCB", "LDROUT", "LPVAG", "LDELTA", "LALPHA0", "LFBJTII",
    "LABJTII", "LCBJTII", "LEBJTII", "LMBJTII", "LVBCI", "LBETA0", "LBETA1", "LBETA2", "LVDSATII0", "LLII", "LESATII", "LSII0", "LSII1", "LSII2", "LSIID", "LAGIDL",
    "LBGIDL", "LCGIDL", "LRGIDL", "LKGIDL", "LFGIDL", "LAGISL", "LBGISL", "LCGISL", "LRGISL", "LKGISL", "LFGISL", "LNTUN", "LNTUND", "LNDIODE", "LNDIODED", "LNRECF0",
    "LNRECF0D", "LNRECR0", "LNRECR0D", "LISBJT", "LIDBJT", "LISDIF", "LIDDIF", "LISREC", "LIDREC", "LISTUN", "LIDTUN", "LVREC0", "LVREC0D", "LVTUN0", "LVTUN0D", "LNBJT",
    "LLBJT0", "LVABJT", "LAELY", "LAHLI", "LAHLID", "LVSDFB", "LVSDTH", "LDELVT", "LACDE", "LMOIN", "LNOFF", "LNOFF2", "LXRCRG1", "LXRCRG2", "LVBSA", "LVSCE",
    "LCDSBS", "LNOFFFD", "LVOFFFD", "LK1B", "LK2B", "LDK2B", "LDVBD0", "LDVBD1", "LMOINFD", "LVBS0PD", "LVBS0FD", "WXJ", "WALPHAGB1", "WBETAGB1", "WALPHAGB2", "WBETAGB2",
    "WAIGBCP2", "WBIGBCP2", "WCIGBCP2", "WCGSL", "WCGDL", "WCKAPPA", "WNDIF", "WUTE", "WKT1", "WKT1L", "WKT2", "WUA1", "WUB1", "WUC1", "WAT", "WPRT",
    "WNTRECF", "WNTRECR", "WXBJT", "WXDIF", "WXREC", "WXTUN", "WXDIFD", "WXRECD", "WXTUND", "WAIGC", "WBIGC", "WCIGC", "WAIGSD", "WBIGSD", "WCIGSD", "WNIGC",
    "WPIGCD", "WPOXEDGE", "WNCH", "WNSUB", "WNGATE", "WNSD", "WVTH0", "WVFB", "WK1", "WK1W1", "WK1W2", "WK2", "WK3", "WK3B", "WKB1", "WW0",
    "WLPEB", "WDVT0", "WDVT1", "WDVT2", "WDVT0W", "WDVT1W", "WDVT2W", "WU0", "WEU", "WUA", "WUB", "WUC", "WUD", "WUD1", "WUCSTE", "WUCS",
    "WVSAT", "WA0", "WAGS", "WB0", "WB1", "WKETA", "WKETAS", "WA1", "WA2", "WRDSW", "WRSW", "WRDW", "WPRWB", "WPRWG", "WWR", "WNFACTOR",
    "WDWG", "WDWB", "WVOFF", "WETA0", "WETAB", "WETA0CV", "WETABCV", "WDSUB", "WCIT", "WCDSC", "WCDSCB", "WCDSCD", "WPCLM", "WPDIBLC1", "WPDIBLC2", "WPDIBLCB",
    "WDROUT", "WPVAG", "WDELTA", "WALPHA0", "WFBJTII", "WABJTII", "WCBJTII", "WEBJTII", "WMBJTII", "WVBCI", "WBETA0", "WBETA1", "WBETA2", "WVDSATII0", "WLII", "WESATII",
    "WSII0", "WSII1", "WSII2", "WSIID", "WAGIDL", "WBGIDL", "WCGIDL", "WRGIDL", "WKGIDL", "WFGIDL", "WAGISL", "WBGISL", "WCGISL", "WRGISL", "WKGISL", "WFGISL",
    "WNTUN", "WNTUND", "WNDIODE", "WNDIODED", "WNRECF0", "WNRECF0D", "WNRECR0", "WNRECR0D", "WISBJT", "WIDBJT", "WISDIF", "WIDDIF", "WISREC", "WIDREC", "WISTUN", "WIDTUN",
    "WVREC0", "WVREC0D", "WVTUN0", "WVTUN0D", "WNBJT", "WLBJT0", "WVABJT", "WAELY", "WAHLI", "WAHLID", "WVSDFB", "WVSDTH", "WDELVT", "WACDE", "WMOIN", "WNOFF",
    "WNOFF2", "WXRCRG1", "WXRCRG2", "WVBSA", "WVSCE", "WCDSBS", "WNOFFFD", "WVOFFFD", "WK1B", "WK2B", "WDK2B", "WDVBD0", "WDVBD1", "WMOINFD", "WVBS0PD", "WVBS0FD",
    "PXJ", "PALPHAGB1", "PBETAGB1", "PALPHAGB2", "PBETAGB2", "PAIGBCP2", "PBIGBCP2", "PCIGBCP2", "PCGSL", "PCGDL", "PCKAPPA", "PNDIF", "PUTE", "PKT1", "PKT1L", "PKT2",
    "PUA1", "PUB1", "PUC1", "PAT", "PPRT", "PNTRECF", "PNTRECR", "PXBJT", "PXDIF", "PXREC", "PXTUN", "PXDIFD", "PXRECD", "PXTUND", "PAIGC", "PBIGC",
    "PCIGC", "PAIGSD", "PBIGSD", "PCIGSD", "PNIGC", "PPIGCD", "PPOXEDGE", "PNCH", "PNSUB", "PNSD", "PNGATE", "PVTH0", "PVFB", "PK1", "PK1W1", "PK1W2",
    "PK2", "PK3", "PK3B", "PKB1", "PW0", "PLPEB", "PDVT0", "PDVT1", "PDVT2", "PDVT0W", "PDVT1W", "PDVT2W", "PU0", "PEU", "PUA", "PUB",
    "PUC", "PUD", "PUD1", "PUCSTE", "PUCS", "PVSAT", "PA0", "PAGS", "PB0", "PB1", "PKETA", "PKETAS", "PA1", "PA2", "PRDSW", "PRSW",
    "PRDW", "PPRWB", "PPRWG", "PWR", "PNFACTOR", "PDWG", "PDWB", "PVOFF", "PETA0", "PETAB", "PETA0CV", "PETABCV", "PDSUB", "PCIT", "PCDSC", "PCDSCB",
    "PCDSCD", "PPCLM", "PPDIBLC1", "PPDIBLC2", "PPDIBLCB", "PDROUT", "PPVAG", "PDELTA", "PALPHA0", "PFBJTII", "PABJTII", "PCBJTII", "PEBJTII", "PMBJTII", "PVBCI", "PBETA0",
    "PBETA1", "PBETA2", "PVDSATII0", "PLII", "PESATII", "PSII0", "PSII1", "PSII2", "PSIID", "PAGIDL", "PBGIDL", "PCGIDL", "PRGIDL", "PKGIDL", "PFGIDL", "PAGISL",
    "PBGISL", "PCGISL", "PRGISL", "PKGISL", "PFGISL", "PNTUN", "PNTUND", "PNDIODE", "PNDIODED", "PNRECF0", "PNRECF0D", "PNRECR0", "PNRECR0D", "PISBJT", "PIDBJT", "PISDIF",
    "PIDDIF", "PISREC", "PIDREC", "PISTUN", "PIDTUN", "PVREC0", "PVREC0D", "PVTUN0", "PVTUN0D", "PNBJT", "PLBJT0", "PVABJT", "PAELY", "PAHLI", "PAHLID", "PVSDFB",
    "PVSDTH", "PDELVT", "PACDE", "PMOIN", "PNOFF", "PNOFF2", "PXRCRG1", "PXRCRG2", "PVBSA", "PVSCE", "PCDSBS", "PNOFFFD", "PVOFFFD", "PK1B", "PK2B", "PDK2B",
    "PDVBD0", "PDVBD1", "PMOINFD", "PVBS0PD", "PVBS0FD", "NLX", "LNLX", "WNLX", "PNLX", "NGIDL", "LNGIDL", "WNGIDL", "PNGIDL", "LPE0", "EGIDL", "EGISL",
    "LLPE0", "LEGIDL", "LEGISL", "WLPE0", "WEGIDL", "WEGISL", "PLPE0", "PEGIDL", "PEGISL", "EGGBCP2", "EGGDEP", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N",
    "AGBC2P", "BGBC2N", "BGBC2P", "VTM00",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 996] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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

const PARAMETER_INTEGER_FLAGS: [bool; 996] = [
    false, false, false, true, false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false,
    true, true, true, false, false, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false,
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
    false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 996] = [
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 996] = [
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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

const PARAMETER_RANGE_FLAGS: [u8; 996] = [
    0, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 0, 2, 2, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2,
    0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 2, 2, 2, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
    3, 3, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0,
    2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
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
    0, 0, 0, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 996] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 13],
    pub branches: [usize; 9],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 996]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 15]>,
    pub(crate) ddt_state_previous: Box<[f64; 15]>,
    pub(crate) ddt_state_older: Box<[f64; 15]>,
    pub(crate) ddt_state_initialized: Box<[bool; 15]>,
    pub(crate) ddt_derivative_current: Box<[f64; 15]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 15]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<1871, 13, 9>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<1871, 13, 9>>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 9;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 9] = ["p", "b", "t", "di", "si", "gi", "gm", "sb", "db"];

    pub const BRANCH_COUNT: usize = 9;
    pub const PARAMETER_COUNT: usize = 996;
    pub const VARIABLE_COUNT: usize = 1871;
    pub const DDT_STATE_COUNT: usize = 15;
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimsoi_va'", name));
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
