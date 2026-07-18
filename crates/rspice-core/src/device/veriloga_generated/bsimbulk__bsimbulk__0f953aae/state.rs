#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]

use crate::device::veriloga_generated::{GeneratedDdtCoefficients, GeneratedVerilogAPersistentState};
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
    pub p1040: f64, pub p1041: f64, pub p1042: f64, pub p1043: f64, pub p1044: f64, pub p1045: f64, pub p1046: f64, pub p1047: f64,
    pub p1048: f64, pub p1049: f64, pub p1050: f64, pub p1051: f64, pub p1052: f64, pub p1053: f64, pub p1054: f64, pub p1055: f64,
    pub p1056: f64, pub p1057: f64, pub p1058: f64, pub p1059: f64, pub p1060: f64, pub p1061: f64, pub p1062: f64, pub p1063: f64,
    pub p1064: f64, pub p1065: f64, pub p1066: f64, pub p1067: f64, pub p1068: f64, pub p1069: f64, pub p1070: f64, pub p1071: f64,
    pub p1072: f64, pub p1073: f64, pub p1074: f64, pub p1075: f64, pub p1076: f64, pub p1077: f64, pub p1078: f64, pub p1079: f64,
    pub p1080: f64, pub p1081: f64, pub p1082: f64, pub p1083: f64, pub p1084: f64, pub p1085: f64, pub p1086: f64, pub p1087: f64,
    pub p1088: f64, pub p1089: f64, pub p1090: f64, pub p1091: f64, pub p1092: f64, pub p1093: f64, pub p1094: f64, pub p1095: f64,
    pub p1096: f64, pub p1097: f64, pub p1098: f64, pub p1099: f64, pub p1100: f64, pub p1101: f64, pub p1102: f64, pub p1103: f64,
    pub p1104: f64, pub p1105: f64, pub p1106: f64, pub p1107: f64, pub p1108: f64, pub p1109: f64, pub p1110: f64, pub p1111: f64,
    pub p1112: f64, pub p1113: f64, pub p1114: f64, pub p1115: f64, pub p1116: f64, pub p1117: f64, pub p1118: f64, pub p1119: f64,
    pub p1120: f64, pub p1121: f64, pub p1122: f64, pub p1123: f64, pub p1124: f64, pub p1125: f64, pub p1126: f64, pub p1127: f64,
    pub p1128: f64, pub p1129: f64, pub p1130: f64, pub p1131: f64, pub p1132: f64, pub p1133: f64, pub p1134: f64, pub p1135: f64,
    pub p1136: f64, pub p1137: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 30] = [
                1e-5, 1e-5, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 50.0, 50.0, 50.0, 50.0, 50.0,
                50.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 30);
            {
                let params = &mut *ptr;
                params.p30 = params.p28;
                validate_parameter("MULT_FN", params.p30, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 47] = [
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-5, 1.0, 1.0, 0.0,
                1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(31), 47);
            {
                let params = &mut *ptr;
                params.p78 = params.p77;
                validate_parameter("TOXP", params.p78, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 13] = [
                0.0, 1e24, 0.0, 1.0, 0.0, 2.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(79), 13);
            {
                let params = &mut *ptr;
                params.p92 = params.p80;
                validate_finite_parameter("NDEPCV", params.p92).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p93 = params.p81;
                validate_finite_parameter("NDEPCVL1", params.p93).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p94 = params.p82;
                validate_parameter("NDEPCVLEXP1", params.p94, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p95 = params.p83;
                validate_finite_parameter("NDEPCVL2", params.p95).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p96 = params.p84;
                validate_parameter("NDEPCVLEXP2", params.p96, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p97 = params.p85;
                validate_finite_parameter("NDEPCVW", params.p97).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p98 = params.p86;
                validate_parameter("NDEPCVWEXP", params.p98, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p99 = params.p87;
                validate_finite_parameter("NDEPCVWL", params.p99).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p100 = params.p88;
                validate_parameter("NDEPCVWLEXP", params.p100, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p101 = params.p89;
                validate_finite_parameter("LNDEPCV", params.p101).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p102 = params.p90;
                validate_finite_parameter("WNDEPCV", params.p102).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p103 = params.p91;
                validate_finite_parameter("PNDEPCV", params.p103).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 22] = [
                5e25, 0.0, 0.0, 0.0, 1.1e16, 1.17, 11.9, 3.9,
                1.5e-7, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(104), 22);
            {
                let params = &mut *ptr;
                params.p126 = params.p116;
                validate_finite_parameter("VFBCV", params.p126).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p127 = params.p117;
                validate_finite_parameter("LVFBCV", params.p127).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p128 = params.p118;
                validate_finite_parameter("WVFBCV", params.p128).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p129 = params.p119;
                validate_finite_parameter("PVFBCV", params.p129).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p130 = params.p120;
                validate_finite_parameter("VFBCVL", params.p130).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p131 = params.p121;
                validate_parameter("VFBCVLEXP", params.p131, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p132 = params.p122;
                validate_finite_parameter("VFBCVW", params.p132).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p133 = params.p123;
                validate_parameter("VFBCVWEXP", params.p133, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p134 = params.p124;
                validate_finite_parameter("VFBCVWL", params.p134).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p135 = params.p125;
                validate_parameter("VFBCVWLEXP", params.p135, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(136), 2);
            {
                let params = &mut *ptr;
                params.p138 = params.p73;
                validate_finite_parameter("DWJ", params.p138).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 36] = [
                1e26, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.045, 0.0, 0.0, 0.0,
                0.08, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(139), 36);
            {
                let params = &mut *ptr;
                params.p175 = params.p171;
                validate_finite_parameter("ETA0R", params.p175).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p176 = params.p172;
                validate_finite_parameter("LETA0R", params.p176).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p177 = params.p173;
                validate_finite_parameter("WETA0R", params.p177).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p178 = params.p174;
                validate_finite_parameter("PETA0R", params.p178).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 50] = [
                1.0, -0.07, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.001, 0.54, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(179), 50);
            {
                let params = &mut *ptr;
                params.p229 = params.p223;
                validate_finite_parameter("CDSCDR", params.p229).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p230 = params.p226;
                validate_finite_parameter("LCDSCDR", params.p230).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p231 = params.p227;
                validate_finite_parameter("WCDSCDR", params.p231).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p232 = params.p228;
                validate_finite_parameter("PCDSCDR", params.p232).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 16] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 100000.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(233), 16);
            {
                let params = &mut *ptr;
                params.p249 = params.p239;
                validate_finite_parameter("VSATR", params.p249).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p250 = params.p240;
                validate_finite_parameter("LVSATR", params.p250).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p251 = params.p241;
                validate_finite_parameter("WVSATR", params.p251).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p252 = params.p242;
                validate_finite_parameter("PVSATR", params.p252).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 6] = [
                0.125, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(253), 6);
            {
                let params = &mut *ptr;
                params.p259 = params.p239;
                validate_finite_parameter("VSATCV", params.p259).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p260 = params.p240;
                validate_finite_parameter("LVSATCV", params.p260).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p261 = params.p241;
                validate_finite_parameter("WVSATCV", params.p261).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p262 = params.p242;
                validate_finite_parameter("PVSATCV", params.p262).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p263 = params.p243;
                validate_finite_parameter("VSATCVL", params.p263).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p264 = params.p244;
                validate_parameter("VSATCVLEXP", params.p264, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p265 = params.p245;
                validate_finite_parameter("VSATCVW", params.p265).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p266 = params.p246;
                validate_parameter("VSATCVWEXP", params.p266, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p267 = params.p247;
                validate_finite_parameter("VSATCVWL", params.p267).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p268 = params.p248;
                validate_parameter("VSATCVWLEXP", params.p268, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 10] = [
                0.0, 1e-8, 0.0, 1e-8, 0.067, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(269), 10);
            {
                let params = &mut *ptr;
                params.p279 = params.p273;
                validate_finite_parameter("U0R", params.p279).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p280 = params.p276;
                validate_finite_parameter("LU0R", params.p280).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p281 = params.p277;
                validate_finite_parameter("WU0R", params.p281).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p282 = params.p278;
                validate_finite_parameter("PU0R", params.p282).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 11] = [
                1.0, 0.001, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(283), 11);
            {
                let params = &mut *ptr;
                params.p294 = params.p284;
                validate_finite_parameter("UAR", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p295 = params.p291;
                validate_finite_parameter("LUAR", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p296 = params.p292;
                validate_finite_parameter("WUAR", params.p296).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p297 = params.p293;
                validate_finite_parameter("PUAR", params.p297).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 16] = [
                1.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.001, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(298), 16);
            {
                let params = &mut *ptr;
                params.p314 = params.p308;
                validate_finite_parameter("UDR", params.p314).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p315 = params.p311;
                validate_finite_parameter("LUDR", params.p315).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p316 = params.p312;
                validate_finite_parameter("WUDR", params.p316).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p317 = params.p313;
                validate_finite_parameter("PUDR", params.p317).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 4] = [
                2.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(318), 4);
            {
                let params = &mut *ptr;
                params.p322 = params.p318;
                validate_finite_parameter("UCSR", params.p322).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p323 = params.p319;
                validate_finite_parameter("LUCSR", params.p323).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p324 = params.p320;
                validate_finite_parameter("WUCSR", params.p324).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p325 = params.p321;
                validate_finite_parameter("PUCSR", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 10] = [
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(326), 10);
            {
                let params = &mut *ptr;
                params.p336 = params.p326;
                validate_finite_parameter("UCR", params.p336).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p337 = params.p333;
                validate_finite_parameter("LUCR", params.p337).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p338 = params.p334;
                validate_finite_parameter("WUCR", params.p338).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p339 = params.p335;
                validate_finite_parameter("PUCR", params.p339).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 6] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(340), 6);
            {
                let params = &mut *ptr;
                params.p346 = params.p340;
                validate_finite_parameter("PCLMR", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p347 = params.p343;
                validate_finite_parameter("LPCLMR", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p348 = params.p344;
                validate_finite_parameter("WPCLMR", params.p348).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p349 = params.p345;
                validate_finite_parameter("PPCLMR", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(350), 1);
            {
                let params = &mut *ptr;
                params.p351 = params.p340;
                validate_finite_parameter("PCLMCV", params.p351).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p352 = params.p341;
                validate_finite_parameter("PCLMCVL", params.p352).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p353 = params.p342;
                validate_parameter("PCLMCVLEXP", params.p353, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p354 = params.p343;
                validate_finite_parameter("LPCLMCV", params.p354).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p355 = params.p344;
                validate_finite_parameter("WPCLMCV", params.p355).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p356 = params.p345;
                validate_finite_parameter("PPCLMCV", params.p356).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 42] = [
                424000000.0, 0.0, 0.0, 0.0, 1e-8, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(357), 42);
            {
                let params = &mut *ptr;
                params.p399 = params.p389;
                validate_finite_parameter("RDWMIN", params.p399).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p400 = params.p390;
                validate_finite_parameter("LRDWMIN", params.p400).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p401 = params.p391;
                validate_finite_parameter("WRDWMIN", params.p401).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p402 = params.p392;
                validate_finite_parameter("PRDWMIN", params.p402).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p403 = params.p393;
                validate_finite_parameter("RDW", params.p403).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p404 = params.p394;
                validate_finite_parameter("LRDW", params.p404).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p405 = params.p395;
                validate_finite_parameter("WRDW", params.p405).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p406 = params.p396;
                validate_finite_parameter("PRDW", params.p406).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p407 = params.p397;
                validate_finite_parameter("RDWL", params.p407).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p408 = params.p398;
                validate_parameter("RDWLEXP", params.p408, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 17] = [
                0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(409), 17);
            {
                let params = &mut *ptr;
                params.p426 = params.p419;
                validate_finite_parameter("PSATR", params.p426).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p427 = params.p420;
                validate_finite_parameter("LPSATR", params.p427).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p428 = params.p421;
                validate_finite_parameter("WPSATR", params.p428).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p429 = params.p422;
                validate_finite_parameter("PPSATR", params.p429).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 10] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(430), 10);
            {
                let params = &mut *ptr;
                params.p440 = params.p434;
                validate_finite_parameter("PTWGR", params.p440).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p441 = params.p435;
                validate_finite_parameter("LPTWGR", params.p441).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p442 = params.p436;
                validate_finite_parameter("WPTWGR", params.p442).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p443 = params.p437;
                validate_finite_parameter("PPTWGR", params.p443).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 22] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(444), 22);
            {
                let params = &mut *ptr;
                params.p466 = params.p460;
                validate_finite_parameter("PDIBLCR", params.p466).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p467 = params.p463;
                validate_finite_parameter("LPDIBLCR", params.p467).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p468 = params.p464;
                validate_finite_parameter("WPDIBLCR", params.p468).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p469 = params.p465;
                validate_finite_parameter("PPDIBLCR", params.p469).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 30] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(470), 30);
            {
                let params = &mut *ptr;
                params.p500 = params.p484;
                validate_finite_parameter("ALPHADR", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p501 = params.p494;
                validate_finite_parameter("BETADR", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 23] = [
                1.0, 5.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(502), 23);
            {
                let params = &mut *ptr;
                params.p525 = params.p484;
                validate_finite_parameter("ALPHA0R", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p526 = params.p489;
                validate_finite_parameter("LALPHA0R", params.p526).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p527 = params.p490;
                validate_finite_parameter("WALPHA0R", params.p527).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p528 = params.p491;
                validate_finite_parameter("PALPHA0R", params.p528).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p529 = params.p494;
                validate_finite_parameter("BETA0R", params.p529).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p530 = params.p497;
                validate_finite_parameter("LBETA0R", params.p530).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p531 = params.p498;
                validate_finite_parameter("WBETA0R", params.p531).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p532 = params.p499;
                validate_finite_parameter("PBETA0R", params.p532).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 9] = [
                0.0136, 0.00171, 0.075, 1.0, 0.0111, 0.000949, 0.006, 1.1,
                3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(533), 9);
            {
                let params = &mut *ptr;
                params.p542 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGC", params.p542).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p543 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGC", params.p543).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p544 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params.p544).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p545 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGS", params.p545).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p546 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGS", params.p546).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p547 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGS", params.p547).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p548 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGD", params.p548).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p549 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGD", params.p549).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p550 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGD", params.p550).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p551 = params.p57;
                validate_finite_parameter("DLCIG", params.p551).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p552 = params.p551;
                validate_finite_parameter("DLCIGD", params.p552).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 95] = [
                1.0, 1.0, 3e-9, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 2300000000.0, 0.0, 0.0, 0.0, 0.5,
                0.0, 0.0, 0.0, 0.8, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(553), 95);
            {
                let params = &mut *ptr;
                params.p648 = params.p630;
                validate_finite_parameter("AGISL", params.p648).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p649 = params.p631;
                validate_finite_parameter("AGISLL", params.p649).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p650 = params.p632;
                validate_finite_parameter("AGISLW", params.p650).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p651 = params.p633;
                validate_finite_parameter("LAGISL", params.p651).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p652 = params.p634;
                validate_finite_parameter("WAGISL", params.p652).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p653 = params.p635;
                validate_finite_parameter("PAGISL", params.p653).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p654 = params.p636;
                validate_finite_parameter("BGISL", params.p654).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p655 = params.p637;
                validate_finite_parameter("LBGISL", params.p655).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p656 = params.p638;
                validate_finite_parameter("WBGISL", params.p656).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p657 = params.p639;
                validate_finite_parameter("PBGISL", params.p657).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p658 = params.p640;
                validate_finite_parameter("CGISL", params.p658).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p659 = params.p641;
                validate_finite_parameter("LCGISL", params.p659).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p660 = params.p642;
                validate_finite_parameter("WCGISL", params.p660).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p661 = params.p643;
                validate_finite_parameter("PCGISL", params.p661).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p662 = params.p644;
                validate_finite_parameter("EGISL", params.p662).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p663 = params.p645;
                validate_finite_parameter("LEGISL", params.p663).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p664 = params.p646;
                validate_finite_parameter("WEGISL", params.p664).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p665 = params.p647;
                validate_finite_parameter("PEGISL", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 30] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.6, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0,
                1000000.0, 1.0, 1000000.0, 1.0, 0.1, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(666), 30);
            {
                let params = &mut *ptr;
                params.p696 = params.p695;
                validate_parameter("DMCI", params.p696, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 5] = [
                0.0, 0.0, 0.0, 0.1, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(697), 5);
            {
                let params = &mut *ptr;
                params.p702 = params.p701;
                validate_finite_parameter("CJD", params.p702).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(703), 1);
            {
                let params = &mut *ptr;
                params.p704 = params.p703;
                validate_finite_parameter("CJSWD", params.p704).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(705), 1);
            {
                let params = &mut *ptr;
                params.p706 = params.p705;
                validate_finite_parameter("CJSWGD", params.p706).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(707), 1);
            {
                let params = &mut *ptr;
                params.p708 = params.p707;
                validate_finite_parameter("PBD", params.p708).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(709), 1);
            {
                let params = &mut *ptr;
                params.p710 = params.p709;
                validate_finite_parameter("PBSWD", params.p710).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p711 = params.p709;
                validate_finite_parameter("PBSWGS", params.p711).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p712 = params.p711;
                validate_finite_parameter("PBSWGD", params.p712).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(713), 1);
            {
                let params = &mut *ptr;
                params.p714 = params.p713;
                validate_finite_parameter("MJD", params.p714).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(715), 1);
            {
                let params = &mut *ptr;
                params.p716 = params.p715;
                validate_finite_parameter("MJSWD", params.p716).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p717 = params.p715;
                validate_finite_parameter("MJSWGS", params.p717).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p718 = params.p717;
                validate_finite_parameter("MJSWGD", params.p718).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0001,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(719), 1);
            {
                let params = &mut *ptr;
                params.p720 = params.p719;
                validate_finite_parameter("JSD", params.p720).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(721), 1);
            {
                let params = &mut *ptr;
                params.p722 = params.p721;
                validate_finite_parameter("JSWD", params.p722).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(723), 1);
            {
                let params = &mut *ptr;
                params.p724 = params.p723;
                validate_finite_parameter("JSWGD", params.p724).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(725), 1);
            {
                let params = &mut *ptr;
                params.p726 = params.p725;
                validate_parameter("NJD", params.p726, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(727), 1);
            {
                let params = &mut *ptr;
                params.p728 = params.p727;
                validate_finite_parameter("IJTHDFWD", params.p728).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(729), 1);
            {
                let params = &mut *ptr;
                params.p730 = params.p729;
                validate_finite_parameter("IJTHDREV", params.p730).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(731), 1);
            {
                let params = &mut *ptr;
                params.p732 = params.p731;
                validate_finite_parameter("BVD", params.p732).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(733), 1);
            {
                let params = &mut *ptr;
                params.p734 = params.p733;
                validate_parameter("XJBVD", params.p734, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(735), 1);
            {
                let params = &mut *ptr;
                params.p736 = params.p735;
                validate_finite_parameter("JTSD", params.p736).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(737), 1);
            {
                let params = &mut *ptr;
                params.p738 = params.p737;
                validate_finite_parameter("JTSSWD", params.p738).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (ptr as *mut f64).add(739), 1);
            {
                let params = &mut *ptr;
                params.p740 = params.p739;
                validate_finite_parameter("JTSSWGD", params.p740).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 2] = [
                0.0, 20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (ptr as *mut f64).add(741), 2);
            {
                let params = &mut *ptr;
                params.p743 = params.p742;
                validate_finite_parameter("NJTSD", params.p743).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (ptr as *mut f64).add(744), 1);
            {
                let params = &mut *ptr;
                params.p745 = params.p744;
                validate_finite_parameter("NJTSSWD", params.p745).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (ptr as *mut f64).add(746), 1);
            {
                let params = &mut *ptr;
                params.p747 = params.p746;
                validate_finite_parameter("NJTSSWGD", params.p747).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (ptr as *mut f64).add(748), 1);
            {
                let params = &mut *ptr;
                params.p749 = params.p748;
                validate_finite_parameter("VTSD", params.p749).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (ptr as *mut f64).add(750), 1);
            {
                let params = &mut *ptr;
                params.p751 = params.p750;
                validate_finite_parameter("VTSSWD", params.p751).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (ptr as *mut f64).add(752), 1);
            {
                let params = &mut *ptr;
                params.p753 = params.p752;
                validate_finite_parameter("VTSSWGD", params.p753).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 142] = [
                12.0, 1.0, 1e-12, 50.0, 0.0, 0.0, 0.0, 50.0,
                0.0, 0.0, 0.0, 100.0, 0.0, 0.0, 0.0, 100.0,
                0.0, 0.0, 0.0, 100.0, 100.0, 100.0, 100.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 41000000.0, 6.25e40,
                0.0, 0.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0,
                0.05, 0.0, 0.0, 0.0, 2.0, 3.125e25, 875000000.0, 0.0,
                0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 2.0, 2.0,
                1.0, 0.577, 0.5164, 0.395, 1.5, 3.5, 0.0, 1.0,
                0.0, 0.0, 27.0, 0.000473, 636.0, 0.0, -1.5, 0.0,
                0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0, 0.0,
                5.6e-11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, -0.004775, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.00156, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, -0.11, 1.0, 0.0, 0.0, 0.0, 0.0, 0.022,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (ptr as *mut f64).add(754), 142);
            {
                let params = &mut *ptr;
                params.p896 = params.p895;
                validate_finite_parameter("XTID", params.p896).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (ptr as *mut f64).add(897), 1);
            {
                let params = &mut *ptr;
                params.p898 = params.p897;
                validate_finite_parameter("XTSD", params.p898).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (ptr as *mut f64).add(899), 1);
            {
                let params = &mut *ptr;
                params.p900 = params.p899;
                validate_finite_parameter("XTSSWD", params.p900).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (ptr as *mut f64).add(901), 1);
            {
                let params = &mut *ptr;
                params.p902 = params.p901;
                validate_finite_parameter("XTSSWGD", params.p902).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (ptr as *mut f64).add(903), 1);
            {
                let params = &mut *ptr;
                params.p904 = params.p903;
                validate_finite_parameter("TNJTSD", params.p904).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (ptr as *mut f64).add(905), 1);
            {
                let params = &mut *ptr;
                params.p906 = params.p905;
                validate_finite_parameter("TNJTSSWD", params.p906).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (ptr as *mut f64).add(907), 1);
            {
                let params = &mut *ptr;
                params.p908 = params.p907;
                validate_finite_parameter("TNJTSSWGD", params.p908).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 158] = [
                0.0, 1e-5, 0.0, 1e-6, 1e-6, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 400.0,
                336000000.0, 0.185, 0.3, 1.4, 0.0, 0.49, 1.42, 20.0,
                1e-8, 0.0, 0.0, 1.0, 0.0, 1e24, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.08, 0.0, 0.0, 0.0, -0.07, 0.0, 0.0,
                0.0, -0.11, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.022, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.2, 0.53, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1e-5, 0.0, 0.0,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 1.0, 0.0, 1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (ptr as *mut f64).add(909), 158);
            {
                let params = &mut *ptr;
                params.p1067 = params.p785;
                validate_finite_parameter("NOIA2", params.p1067).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1068 = params.p80;
                validate_parameter("HNDEP", params.p1068, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (ptr as *mut f64).add(1069), 24);
            {
                let params = &mut *ptr;
                params.p1093 = 0.001;
                validate_parameter("minr", params.p1093, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 10] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 5e16, 100000.0, 0.0,
                0.0, 60.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (ptr as *mut f64).add(1094), 10);
            {
                let params = &mut *ptr;
                params.p1104 = params.p1101;
                validate_parameter("PTWGHVII", params.p1104, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1105 = params.p1102;
                validate_finite_parameter("PTWGHV1II", params.p1105).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1106 = params.p1103;
                validate_parameter("PSATXHVII", params.p1106, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 2] = [
                1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (ptr as *mut f64).add(1107), 2);
            {
                let params = &mut *ptr;
                params.p1109 = params.p1099;
                validate_parameter("NDRIFTS", params.p1109, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                100.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (ptr as *mut f64).add(1110), 1);
            {
                let params = &mut *ptr;
                params.p1111 = params.p1110;
                validate_parameter("RDLCWCV", params.p1111, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 4] = [
                0.0, 0.0, -1.0, 5.000000000000001e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (ptr as *mut f64).add(1112), 4);
            {
                let params = &mut *ptr;
                params.p1116 = params.p1115;
                validate_finite_parameter("LOVERACC", params.p1116).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1117 = params.p80;
                validate_parameter("NDR", params.p1117, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 16] = [
                0.0, 1.0, 0.0, 0.0, 0.001, 0.6, 0.0, 0.0,
                8.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (ptr as *mut f64).add(1118), 16);
            {
                let params = &mut *ptr;
                params.p1134 = params.p1130;
                validate_finite_parameter("A0CV", params.p1134).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1135 = params.p1131;
                validate_finite_parameter("AGSCV", params.p1135).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1136 = params.p1133;
                validate_parameter("KETACV", params.p1136, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (ptr as *mut f64).add(1137), 1);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1138] = [
    ("l", 0), ("w", 1), ("nf", 2), ("nrs", 3), ("nrd", 4), ("vfbsdoff", 5), ("minz", 6), ("rgatemod", 7), ("rbodymod", 8), ("geomod", 9), ("rgeomod", 10), ("rbpb", 11), ("rbpd", 12), ("rbps", 13), ("rbdb", 14), ("rbsb", 15),
    ("rdb", 16), ("sa", 17), ("sb", 18), ("sd", 19), ("sca", 20), ("scb", 21), ("scc", 22), ("sc", 23), ("as", 24), ("ad", 25), ("ps", 26), ("pd", 27), ("mult_i", 28), ("mult_q", 29), ("mult_fn", 30), ("xgw", 31),
    ("ngcon", 32), ("dtemp", 33), ("mulu0", 34), ("delvto", 35), ("ids0mult", 36), ("edgefet", 37), ("sslmod", 38), ("type", 39), ("cvmod", 40), ("covmod", 41), ("rdsmod", 42), ("wpemod", 43), ("asymmod", 44), ("gidlmod", 45), ("igcmod", 46), ("igbmod", 47),
    ("tnoimod", 48), ("shmod", 49), ("mobscale", 50), ("llong", 51), ("lmlt", 52), ("wmlt", 53), ("xl", 54), ("wwide", 55), ("xw", 56), ("lint", 57), ("ll", 58), ("lw", 59), ("lwl", 60), ("lln", 61), ("lwn", 62), ("wint", 63),
    ("wl", 64), ("ww", 65), ("wwl", 66), ("wln", 67), ("wwn", 68), ("dlc", 69), ("llc", 70), ("lwc", 71), ("lwlc", 72), ("dwc", 73), ("wlc", 74), ("wwc", 75), ("wwlc", 76), ("toxe", 77), ("toxp", 78), ("dtox", 79),
    ("ndep", 80), ("ndepl1", 81), ("ndeplexp1", 82), ("ndepl2", 83), ("ndeplexp2", 84), ("ndepw", 85), ("ndepwexp", 86), ("ndepwl", 87), ("ndepwlexp", 88), ("lndep", 89), ("wndep", 90), ("pndep", 91), ("ndepcv", 92), ("ndepcvl1", 93), ("ndepcvlexp1", 94), ("ndepcvl2", 95),
    ("ndepcvlexp2", 96), ("ndepcvw", 97), ("ndepcvwexp", 98), ("ndepcvwl", 99), ("ndepcvwlexp", 100), ("lndepcv", 101), ("wndepcv", 102), ("pndepcv", 103), ("ngate", 104), ("lngate", 105), ("wngate", 106), ("pngate", 107), ("ni0sub", 108), ("bg0sub", 109), ("epsrsub", 110), ("epsrox", 111),
    ("xj", 112), ("lxj", 113), ("wxj", 114), ("pxj", 115), ("vfb", 116), ("lvfb", 117), ("wvfb", 118), ("pvfb", 119), ("vfbl", 120), ("vfblexp", 121), ("vfbw", 122), ("vfbwexp", 123), ("vfbwl", 124), ("vfbwlexp", 125), ("vfbcv", 126), ("lvfbcv", 127),
    ("wvfbcv", 128), ("pvfbcv", 129), ("vfbcvl", 130), ("vfbcvlexp", 131), ("vfbcvw", 132), ("vfbcvwexp", 133), ("vfbcvwl", 134), ("vfbcvwlexp", 135), ("delvfbacc", 136), ("permod", 137), ("dwj", 138), ("nsd", 139), ("lnsd", 140), ("wnsd", 141), ("pnsd", 142), ("dvtp0", 143),
    ("ldvtp0", 144), ("wdvtp0", 145), ("pdvtp0", 146), ("dvtp1", 147), ("ldvtp1", 148), ("wdvtp1", 149), ("pdvtp1", 150), ("dvtp2", 151), ("ldvtp2", 152), ("wdvtp2", 153), ("pdvtp2", 154), ("dvtp3", 155), ("ldvtp3", 156), ("wdvtp3", 157), ("pdvtp3", 158), ("dvtp4", 159),
    ("ldvtp4", 160), ("wdvtp4", 161), ("pdvtp4", 162), ("dvtp5", 163), ("ldvtp5", 164), ("wdvtp5", 165), ("pdvtp5", 166), ("phin", 167), ("lphin", 168), ("wphin", 169), ("pphin", 170), ("eta0", 171), ("leta0", 172), ("weta0", 173), ("peta0", 174), ("eta0r", 175),
    ("leta0r", 176), ("weta0r", 177), ("peta0r", 178), ("dsub", 179), ("etab", 180), ("etabexp", 181), ("letab", 182), ("wetab", 183), ("petab", 184), ("k1", 185), ("k1l", 186), ("k1lexp", 187), ("k1w", 188), ("k1wexp", 189), ("k1wl", 190), ("k1wlexp", 191),
    ("lk1", 192), ("wk1", 193), ("pk1", 194), ("k2", 195), ("k2l", 196), ("k2lexp", 197), ("k2w", 198), ("k2wexp", 199), ("k2wl", 200), ("k2wlexp", 201), ("lk2", 202), ("wk2", 203), ("pk2", 204), ("ados", 205), ("bdos", 206), ("qm0", 207),
    ("etaqm", 208), ("cit", 209), ("lcit", 210), ("wcit", 211), ("pcit", 212), ("nfactor", 213), ("nfactorl", 214), ("nfactorlexp", 215), ("nfactorw", 216), ("nfactorwexp", 217), ("nfactorwl", 218), ("nfactorwlexp", 219), ("lnfactor", 220), ("wnfactor", 221), ("pnfactor", 222), ("cdscd", 223),
    ("cdscdl", 224), ("cdscdlexp", 225), ("lcdscd", 226), ("wcdscd", 227), ("pcdscd", 228), ("cdscdr", 229), ("lcdscdr", 230), ("wcdscdr", 231), ("pcdscdr", 232), ("cdscb", 233), ("cdscbl", 234), ("cdscblexp", 235), ("lcdscb", 236), ("wcdscb", 237), ("pcdscb", 238), ("vsat", 239),
    ("lvsat", 240), ("wvsat", 241), ("pvsat", 242), ("vsatl", 243), ("vsatlexp", 244), ("vsatw", 245), ("vsatwexp", 246), ("vsatwl", 247), ("vsatwlexp", 248), ("vsatr", 249), ("lvsatr", 250), ("wvsatr", 251), ("pvsatr", 252), ("delta", 253), ("ldelta", 254), ("wdelta", 255),
    ("pdelta", 256), ("deltal", 257), ("deltalexp", 258), ("vsatcv", 259), ("lvsatcv", 260), ("wvsatcv", 261), ("pvsatcv", 262), ("vsatcvl", 263), ("vsatcvlexp", 264), ("vsatcvw", 265), ("vsatcvwexp", 266), ("vsatcvwl", 267), ("vsatcvwlexp", 268), ("up1", 269), ("lp1", 270), ("up2", 271),
    ("lp2", 272), ("u0", 273), ("u0l", 274), ("u0lexp", 275), ("lu0", 276), ("wu0", 277), ("pu0", 278), ("u0r", 279), ("lu0r", 280), ("wu0r", 281), ("pu0r", 282), ("etamob", 283), ("ua", 284), ("ual", 285), ("ualexp", 286), ("uaw", 287),
    ("uawexp", 288), ("uawl", 289), ("uawlexp", 290), ("lua", 291), ("wua", 292), ("pua", 293), ("uar", 294), ("luar", 295), ("wuar", 296), ("puar", 297), ("eu", 298), ("leu", 299), ("weu", 300), ("peu", 301), ("eul", 302), ("eulexp", 303),
    ("euw", 304), ("euwexp", 305), ("euwl", 306), ("euwlexp", 307), ("ud", 308), ("udl", 309), ("udlexp", 310), ("lud", 311), ("wud", 312), ("pud", 313), ("udr", 314), ("ludr", 315), ("wudr", 316), ("pudr", 317), ("ucs", 318), ("lucs", 319),
    ("wucs", 320), ("pucs", 321), ("ucsr", 322), ("lucsr", 323), ("wucsr", 324), ("pucsr", 325), ("uc", 326), ("ucl", 327), ("uclexp", 328), ("ucw", 329), ("ucwexp", 330), ("ucwl", 331), ("ucwlexp", 332), ("luc", 333), ("wuc", 334), ("puc", 335),
    ("ucr", 336), ("lucr", 337), ("wucr", 338), ("pucr", 339), ("pclm", 340), ("pclml", 341), ("pclmlexp", 342), ("lpclm", 343), ("wpclm", 344), ("ppclm", 345), ("pclmr", 346), ("lpclmr", 347), ("wpclmr", 348), ("ppclmr", 349), ("pclmg", 350), ("pclmcv", 351),
    ("pclmcvl", 352), ("pclmcvlexp", 353), ("lpclmcv", 354), ("wpclmcv", 355), ("ppclmcv", 356), ("pscbe1", 357), ("lpscbe1", 358), ("wpscbe1", 359), ("ppscbe1", 360), ("pscbe2", 361), ("lpscbe2", 362), ("wpscbe2", 363), ("ppscbe2", 364), ("pdits", 365), ("lpdits", 366), ("wpdits", 367),
    ("ppdits", 368), ("pditsl", 369), ("pditsd", 370), ("lpditsd", 371), ("wpditsd", 372), ("ppditsd", 373), ("rsh", 374), ("prwg", 375), ("lprwg", 376), ("wprwg", 377), ("pprwg", 378), ("prwb", 379), ("lprwb", 380), ("wprwb", 381), ("pprwb", 382), ("prwbl", 383),
    ("prwblexp", 384), ("wr", 385), ("lwr", 386), ("wwr", 387), ("pwr", 388), ("rswmin", 389), ("lrswmin", 390), ("wrswmin", 391), ("prswmin", 392), ("rsw", 393), ("lrsw", 394), ("wrsw", 395), ("prsw", 396), ("rswl", 397), ("rswlexp", 398), ("rdwmin", 399),
    ("lrdwmin", 400), ("wrdwmin", 401), ("prdwmin", 402), ("rdw", 403), ("lrdw", 404), ("wrdw", 405), ("prdw", 406), ("rdwl", 407), ("rdwlexp", 408), ("rdswmin", 409), ("lrdswmin", 410), ("wrdswmin", 411), ("prdswmin", 412), ("rdsw", 413), ("rdswl", 414), ("rdswlexp", 415),
    ("lrdsw", 416), ("wrdsw", 417), ("prdsw", 418), ("psat", 419), ("lpsat", 420), ("wpsat", 421), ("ppsat", 422), ("psatl", 423), ("psatlexp", 424), ("psatb", 425), ("psatr", 426), ("lpsatr", 427), ("wpsatr", 428), ("ppsatr", 429), ("lpsatb", 430), ("wpsatb", 431),
    ("ppsatb", 432), ("psatx", 433), ("ptwg", 434), ("lptwg", 435), ("wptwg", 436), ("pptwg", 437), ("ptwgl", 438), ("ptwglexp", 439), ("ptwgr", 440), ("lptwgr", 441), ("wptwgr", 442), ("pptwgr", 443), ("a1", 444), ("la1", 445), ("wa1", 446), ("pa1", 447),
    ("a11", 448), ("la11", 449), ("wa11", 450), ("pa11", 451), ("a2", 452), ("la2", 453), ("wa2", 454), ("pa2", 455), ("a21", 456), ("la21", 457), ("wa21", 458), ("pa21", 459), ("pdiblc", 460), ("pdiblcl", 461), ("pdiblclexp", 462), ("lpdiblc", 463),
    ("wpdiblc", 464), ("ppdiblc", 465), ("pdiblcr", 466), ("lpdiblcr", 467), ("wpdiblcr", 468), ("ppdiblcr", 469), ("pdiblcb", 470), ("lpdiblcb", 471), ("wpdiblcb", 472), ("ppdiblcb", 473), ("pvag", 474), ("lpvag", 475), ("wpvag", 476), ("ppvag", 477), ("fprout", 478), ("fproutl", 479),
    ("fproutlexp", 480), ("lfprout", 481), ("wfprout", 482), ("pfprout", 483), ("alpha0", 484), ("alpha0l", 485), ("alpha0lexp", 486), ("alpha0w", 487), ("alpha0wexp", 488), ("lalpha0", 489), ("walpha0", 490), ("palpha0", 491), ("alpha3", 492), ("alpha4", 493), ("beta0", 494), ("beta0w", 495),
    ("beta0wexp", 496), ("lbeta0", 497), ("wbeta0", 498), ("pbeta0", 499), ("alphadr", 500), ("betadr", 501), ("drii1", 502), ("drii2", 503), ("deltaii", 504), ("alpha1", 505), ("alpha2", 506), ("alphadr1", 507), ("alphadr2", 508), ("alphadr3", 509), ("alphadr4", 510), ("drexp", 511),
    ("drii3", 512), ("drii4", 513), ("cmd1", 514), ("cmd2", 515), ("cms1", 516), ("cms2", 517), ("beta1", 518), ("beta1w", 519), ("beta1wexp", 520), ("beta2", 521), ("beta2w", 522), ("beta2wexp", 523), ("beta3", 524), ("alpha0r", 525), ("lalpha0r", 526), ("walpha0r", 527),
    ("palpha0r", 528), ("beta0r", 529), ("lbeta0r", 530), ("wbeta0r", 531), ("pbeta0r", 532), ("aigbacc", 533), ("bigbacc", 534), ("cigbacc", 535), ("nigbacc", 536), ("aigbinv", 537), ("bigbinv", 538), ("cigbinv", 539), ("eigbinv", 540), ("nigbinv", 541), ("aigc", 542), ("bigc", 543),
    ("cigc", 544), ("aigs", 545), ("bigs", 546), ("cigs", 547), ("aigd", 548), ("bigd", 549), ("cigd", 550), ("dlcig", 551), ("dlcigd", 552), ("poxedge", 553), ("ntox", 554), ("toxref", 555), ("pigcd", 556), ("aigcl", 557), ("aigcw", 558), ("aigsl", 559),
    ("aigsw", 560), ("aigdl", 561), ("aigdw", 562), ("pigcdl", 563), ("laigbinv", 564), ("waigbinv", 565), ("paigbinv", 566), ("lbigbinv", 567), ("wbigbinv", 568), ("pbigbinv", 569), ("lcigbinv", 570), ("wcigbinv", 571), ("pcigbinv", 572), ("leigbinv", 573), ("weigbinv", 574), ("peigbinv", 575),
    ("lnigbinv", 576), ("wnigbinv", 577), ("pnigbinv", 578), ("laigbacc", 579), ("waigbacc", 580), ("paigbacc", 581), ("lbigbacc", 582), ("wbigbacc", 583), ("pbigbacc", 584), ("lcigbacc", 585), ("wcigbacc", 586), ("pcigbacc", 587), ("lnigbacc", 588), ("wnigbacc", 589), ("pnigbacc", 590), ("laigc", 591),
    ("waigc", 592), ("paigc", 593), ("lbigc", 594), ("wbigc", 595), ("pbigc", 596), ("lcigc", 597), ("wcigc", 598), ("pcigc", 599), ("laigs", 600), ("waigs", 601), ("paigs", 602), ("lbigs", 603), ("wbigs", 604), ("pbigs", 605), ("lcigs", 606), ("wcigs", 607),
    ("pcigs", 608), ("laigd", 609), ("waigd", 610), ("paigd", 611), ("lbigd", 612), ("wbigd", 613), ("pbigd", 614), ("lcigd", 615), ("wcigd", 616), ("pcigd", 617), ("lpoxedge", 618), ("wpoxedge", 619), ("ppoxedge", 620), ("ldlcig", 621), ("wdlcig", 622), ("pdlcig", 623),
    ("ldlcigd", 624), ("wdlcigd", 625), ("pdlcigd", 626), ("lntox", 627), ("wntox", 628), ("pntox", 629), ("agidl", 630), ("agidll", 631), ("agidlw", 632), ("lagidl", 633), ("wagidl", 634), ("pagidl", 635), ("bgidl", 636), ("lbgidl", 637), ("wbgidl", 638), ("pbgidl", 639),
    ("cgidl", 640), ("lcgidl", 641), ("wcgidl", 642), ("pcgidl", 643), ("egidl", 644), ("legidl", 645), ("wegidl", 646), ("pegidl", 647), ("agisl", 648), ("agisll", 649), ("agislw", 650), ("lagisl", 651), ("wagisl", 652), ("pagisl", 653), ("bgisl", 654), ("lbgisl", 655),
    ("wbgisl", 656), ("pbgisl", 657), ("cgisl", 658), ("lcgisl", 659), ("wcgisl", 660), ("pcgisl", 661), ("egisl", 662), ("legisl", 663), ("wegisl", 664), ("pegisl", 665), ("cf", 666), ("lcf", 667), ("wcf", 668), ("pcf", 669), ("cfrcoeff", 670), ("cgso", 671),
    ("cgdo", 672), ("cgbo", 673), ("cgsl", 674), ("lcgsl", 675), ("wcgsl", 676), ("pcgsl", 677), ("cgdl", 678), ("lcgdl", 679), ("wcgdl", 680), ("pcgdl", 681), ("ckappas", 682), ("lckappas", 683), ("wckappas", 684), ("pckappas", 685), ("ckappad", 686), ("lckappad", 687),
    ("wckappad", 688), ("pckappad", 689), ("ckappad1", 690), ("ckappad2", 691), ("ckappas1", 692), ("ckappas2", 693), ("spqbacv", 694), ("dmcg", 695), ("dmci", 696), ("dmdg", 697), ("dmcgt", 698), ("xgl", 699), ("rshg", 700), ("cjs", 701), ("cjd", 702), ("cjsws", 703),
    ("cjswd", 704), ("cjswgs", 705), ("cjswgd", 706), ("pbs", 707), ("pbd", 708), ("pbsws", 709), ("pbswd", 710), ("pbswgs", 711), ("pbswgd", 712), ("mjs", 713), ("mjd", 714), ("mjsws", 715), ("mjswd", 716), ("mjswgs", 717), ("mjswgd", 718), ("jss", 719),
    ("jsd", 720), ("jsws", 721), ("jswd", 722), ("jswgs", 723), ("jswgd", 724), ("njs", 725), ("njd", 726), ("ijthsfwd", 727), ("ijthdfwd", 728), ("ijthsrev", 729), ("ijthdrev", 730), ("bvs", 731), ("bvd", 732), ("xjbvs", 733), ("xjbvd", 734), ("jtss", 735),
    ("jtsd", 736), ("jtssws", 737), ("jtsswd", 738), ("jtsswgs", 739), ("jtsswgd", 740), ("jtweff", 741), ("njts", 742), ("njtsd", 743), ("njtssw", 744), ("njtsswd", 745), ("njtsswg", 746), ("njtsswgd", 747), ("vtss", 748), ("vtsd", 749), ("vtssws", 750), ("vtsswd", 751),
    ("vtsswgs", 752), ("vtsswgd", 753), ("xrcrg1", 754), ("xrcrg2", 755), ("gbmin", 756), ("rbps0", 757), ("rbpsl", 758), ("rbpsw", 759), ("rbpsnf", 760), ("rbpd0", 761), ("rbpdl", 762), ("rbpdw", 763), ("rbpdnf", 764), ("rbpbx0", 765), ("rbpbxl", 766), ("rbpbxw", 767),
    ("rbpbxnf", 768), ("rbpby0", 769), ("rbpbyl", 770), ("rbpbyw", 771), ("rbpbynf", 772), ("rbsbx0", 773), ("rbsby0", 774), ("rbdbx0", 775), ("rbdby0", 776), ("rbsdbxl", 777), ("rbsdbxw", 778), ("rbsdbxnf", 779), ("rbsdbyl", 780), ("rbsdbyw", 781), ("rbsdbynf", 782), ("ef", 783),
    ("em", 784), ("noia", 785), ("noia3", 786), ("lnoia3", 787), ("wnoia3", 788), ("pnoia3", 789), ("mpower", 790), ("lmpower", 791), ("wmpower", 792), ("pmpower", 793), ("qsref", 794), ("lqsref", 795), ("wqsref", 796), ("pqsref", 797), ("spfn", 798), ("noib", 799),
    ("noic", 800), ("lintnoi", 801), ("noia1", 802), ("noiax", 803), ("bfns", 804), ("bfnd", 805), ("kfns", 806), ("kfnd", 807), ("afns", 808), ("afnd", 809), ("ntnoi", 810), ("rnoia", 811), ("rnoib", 812), ("rnoic", 813), ("tnoia", 814), ("tnoib", 815),
    ("tnoic", 816), ("binunit", 817), ("dlbin", 818), ("dwbin", 819), ("tnom", 820), ("tbgasub", 821), ("tbgbsub", 822), ("tnfactor", 823), ("ute", 824), ("lute", 825), ("wute", 826), ("pute", 827), ("utel", 828), ("ua1", 829), ("lua1", 830), ("wua1", 831),
    ("pua1", 832), ("ua1l", 833), ("uc1", 834), ("luc1", 835), ("wuc1", 836), ("puc1", 837), ("ud1", 838), ("lud1", 839), ("wud1", 840), ("pud1", 841), ("ud1l", 842), ("eu1", 843), ("leu1", 844), ("weu1", 845), ("peu1", 846), ("ucste", 847),
    ("lucste", 848), ("wucste", 849), ("pucste", 850), ("teta0", 851), ("prt", 852), ("lprt", 853), ("wprt", 854), ("pprt", 855), ("at", 856), ("lat", 857), ("wat", 858), ("pat", 859), ("atl", 860), ("tdelta", 861), ("ptwgt", 862), ("lptwgt", 863),
    ("wptwgt", 864), ("pptwgt", 865), ("ptwgtl", 866), ("kt1", 867), ("kt1exp", 868), ("kt1l", 869), ("lkt1", 870), ("wkt1", 871), ("pkt1", 872), ("kt2", 873), ("lkt2", 874), ("wkt2", 875), ("pkt2", 876), ("iit", 877), ("liit", 878), ("wiit", 879),
    ("piit", 880), ("igt", 881), ("ligt", 882), ("wigt", 883), ("pigt", 884), ("tgidl", 885), ("ltgidl", 886), ("wtgidl", 887), ("ptgidl", 888), ("tcj", 889), ("tcjsw", 890), ("tcjswg", 891), ("tpb", 892), ("tpbsw", 893), ("tpbswg", 894), ("xtis", 895),
    ("xtid", 896), ("xtss", 897), ("xtsd", 898), ("xtssws", 899), ("xtsswd", 900), ("xtsswgs", 901), ("xtsswgd", 902), ("tnjts", 903), ("tnjtsd", 904), ("tnjtssw", 905), ("tnjtsswd", 906), ("tnjtsswg", 907), ("tnjtsswgd", 908), ("rth0", 909), ("cth0", 910), ("wth0", 911),
    ("saref", 912), ("sbref", 913), ("wlod", 914), ("ku0", 915), ("kvsat", 916), ("tku0", 917), ("lku0", 918), ("wku0", 919), ("pku0", 920), ("llodku0", 921), ("wlodku0", 922), ("kvth0", 923), ("lkvth0", 924), ("wkvth0", 925), ("pkvth0", 926), ("llodvth", 927),
    ("wlodvth", 928), ("stk2", 929), ("lodk2", 930), ("steta0", 931), ("lodeta0", 932), ("web", 933), ("wec", 934), ("kvth0we", 935), ("lkvth0we", 936), ("wkvth0we", 937), ("pkvth0we", 938), ("k2we", 939), ("lk2we", 940), ("wk2we", 941), ("pk2we", 942), ("ku0we", 943),
    ("lku0we", 944), ("wku0we", 945), ("pku0we", 946), ("scref", 947), ("ssl0", 948), ("ssl1", 949), ("ssl2", 950), ("ssl3", 951), ("ssl4", 952), ("ssl5", 953), ("sslexp1", 954), ("sslexp2", 955), ("avdsx", 956), ("wedge", 957), ("dgammaedge", 958), ("dgammaedgel", 959),
    ("dgammaedgelexp", 960), ("dvtedge", 961), ("ndepedge", 962), ("lndepedge", 963), ("wndepedge", 964), ("pndepedge", 965), ("nfactoredge", 966), ("lnfactoredge", 967), ("wnfactoredge", 968), ("pnfactoredge", 969), ("citedge", 970), ("lcitedge", 971), ("wcitedge", 972), ("pcitedge", 973), ("cdscdedge", 974), ("lcdscdedge", 975),
    ("wcdscdedge", 976), ("pcdscdedge", 977), ("cdscbedge", 978), ("lcdscbedge", 979), ("wcdscbedge", 980), ("pcdscbedge", 981), ("eta0edge", 982), ("leta0edge", 983), ("weta0edge", 984), ("peta0edge", 985), ("etabedge", 986), ("letabedge", 987), ("wetabedge", 988), ("petabedge", 989), ("kt1edge", 990), ("lkt1edge", 991),
    ("wkt1edge", 992), ("pkt1edge", 993), ("kt1ledge", 994), ("lkt1ledge", 995), ("wkt1ledge", 996), ("pkt1ledge", 997), ("kt2edge", 998), ("lkt2edge", 999), ("wkt2edge", 1000), ("pkt2edge", 1001), ("kt1expedge", 1002), ("lkt1expedge", 1003), ("wkt1expedge", 1004), ("pkt1expedge", 1005), ("tnfactoredge", 1006), ("ltnfactoredge", 1007),
    ("wtnfactoredge", 1008), ("ptnfactoredge", 1009), ("teta0edge", 1010), ("lteta0edge", 1011), ("wteta0edge", 1012), ("pteta0edge", 1013), ("dvt0edge", 1014), ("dvt1edge", 1015), ("dvt2edge", 1016), ("k2edge", 1017), ("lk2edge", 1018), ("wk2edge", 1019), ("pk2edge", 1020), ("kvth0edge", 1021), ("lkvth0edge", 1022), ("wkvth0edge", 1023),
    ("pkvth0edge", 1024), ("kvth0edgewe", 1025), ("lkvth0edgewe", 1026), ("wkvth0edgewe", 1027), ("pkvth0edgewe", 1028), ("k2edgewe", 1029), ("lk2edgewe", 1030), ("wk2edgewe", 1031), ("pk2edgewe", 1032), ("stk2edge", 1033), ("lstk2edge", 1034), ("wstk2edge", 1035), ("pstk2edge", 1036), ("steta0edge", 1037), ("lsteta0edge", 1038), ("wsteta0edge", 1039),
    ("psteta0edge", 1040), ("igclamp", 1041), ("lp", 1042), ("rnoik", 1043), ("tnoik", 1044), ("tnoik2", 1045), ("k0", 1046), ("lk0", 1047), ("wk0", 1048), ("pk0", 1049), ("k01", 1050), ("lk01", 1051), ("wk01", 1052), ("pk01", 1053), ("m0", 1054), ("lm0", 1055),
    ("wm0", 1056), ("pm0", 1057), ("m01", 1058), ("lm01", 1059), ("wm01", 1060), ("pm01", 1061), ("nedge", 1062), ("noia1_edge", 1063), ("noiax_edge", 1064), ("fnoimod", 1065), ("lh", 1066), ("noia2", 1067), ("hndep", 1068), ("c0", 1069), ("lc0", 1070), ("wc0", 1071),
    ("pc0", 1072), ("c01", 1073), ("lc01", 1074), ("wc01", 1075), ("pc01", 1076), ("c0si", 1077), ("lc0si", 1078), ("wc0si", 1079), ("pc0si", 1080), ("c0si1", 1081), ("lc0si1", 1082), ("wc0si1", 1083), ("pc0si1", 1084), ("c0sisat", 1085), ("lc0sisat", 1086), ("wc0sisat", 1087),
    ("pc0sisat", 1088), ("c0sisat1", 1089), ("lc0sisat1", 1090), ("wc0sisat1", 1091), ("pc0sisat1", 1092), ("minr", 1093), ("hvmod", 1094), ("hvcap", 1095), ("hvcaps", 1096), ("rbodyhvmod", 1097), ("iimod", 1098), ("ndriftd", 1099), ("vdrift", 1100), ("ptwghv", 1101), ("ptwghv1", 1102), ("psatxhv", 1103),
    ("ptwghvii", 1104), ("ptwghv1ii", 1105), ("psatxhvii", 1106), ("mdrift", 1107), ("dsmooth", 1108), ("ndrifts", 1109), ("rdlcw", 1110), ("rdlcwcv", 1111), ("rslcw", 1112), ("pdrwb", 1113), ("vfbov", 1114), ("lover", 1115), ("loveracc", 1116), ("ndr", 1117), ("slhv", 1118), ("slhv1", 1119),
    ("prthv", 1120), ("athv", 1121), ("hvfactor", 1122), ("asymp", 1123), ("drb1", 1124), ("drb2", 1125), ("rdvds", 1126), ("gadrift", 1127), ("xpart", 1128), ("abulk", 1129), ("a0", 1130), ("ags", 1131), ("ags1", 1132), ("keta", 1133), ("a0cv", 1134), ("agscv", 1135),
    ("ketacv", 1136), ("cvslope", 1137),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1138] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1138] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, Some(0), None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1138] = [
    "L", "W", "NF", "NRS", "NRD", "VFBSDOFF", "MINZ", "RGATEMOD", "RBODYMOD", "GEOMOD", "RGEOMOD", "RBPB", "RBPD", "RBPS", "RBDB", "RBSB",
    "RDB", "SA", "SB", "SD", "SCA", "SCB", "SCC", "SC", "AS", "AD", "PS", "PD", "MULT_I", "MULT_Q", "MULT_FN", "XGW",
    "NGCON", "DTEMP", "MULU0", "DELVTO", "IDS0MULT", "EDGEFET", "SSLMOD", "TYPE", "CVMOD", "COVMOD", "RDSMOD", "WPEMOD", "ASYMMOD", "GIDLMOD", "IGCMOD", "IGBMOD",
    "TNOIMOD", "SHMOD", "MOBSCALE", "LLONG", "LMLT", "WMLT", "XL", "WWIDE", "XW", "LINT", "LL", "LW", "LWL", "LLN", "LWN", "WINT",
    "WL", "WW", "WWL", "WLN", "WWN", "DLC", "LLC", "LWC", "LWLC", "DWC", "WLC", "WWC", "WWLC", "TOXE", "TOXP", "DTOX",
    "NDEP", "NDEPL1", "NDEPLEXP1", "NDEPL2", "NDEPLEXP2", "NDEPW", "NDEPWEXP", "NDEPWL", "NDEPWLEXP", "LNDEP", "WNDEP", "PNDEP", "NDEPCV", "NDEPCVL1", "NDEPCVLEXP1", "NDEPCVL2",
    "NDEPCVLEXP2", "NDEPCVW", "NDEPCVWEXP", "NDEPCVWL", "NDEPCVWLEXP", "LNDEPCV", "WNDEPCV", "PNDEPCV", "NGATE", "LNGATE", "WNGATE", "PNGATE", "NI0SUB", "BG0SUB", "EPSRSUB", "EPSROX",
    "XJ", "LXJ", "WXJ", "PXJ", "VFB", "LVFB", "WVFB", "PVFB", "VFBL", "VFBLEXP", "VFBW", "VFBWEXP", "VFBWL", "VFBWLEXP", "VFBCV", "LVFBCV",
    "WVFBCV", "PVFBCV", "VFBCVL", "VFBCVLEXP", "VFBCVW", "VFBCVWEXP", "VFBCVWL", "VFBCVWLEXP", "DELVFBACC", "PERMOD", "DWJ", "NSD", "LNSD", "WNSD", "PNSD", "DVTP0",
    "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3", "WDVTP3", "PDVTP3", "DVTP4",
    "LDVTP4", "WDVTP4", "PDVTP4", "DVTP5", "LDVTP5", "WDVTP5", "PDVTP5", "PHIN", "LPHIN", "WPHIN", "PPHIN", "ETA0", "LETA0", "WETA0", "PETA0", "ETA0R",
    "LETA0R", "WETA0R", "PETA0R", "DSUB", "ETAB", "ETABEXP", "LETAB", "WETAB", "PETAB", "K1", "K1L", "K1LEXP", "K1W", "K1WEXP", "K1WL", "K1WLEXP",
    "LK1", "WK1", "PK1", "K2", "K2L", "K2LEXP", "K2W", "K2WEXP", "K2WL", "K2WLEXP", "LK2", "WK2", "PK2", "ADOS", "BDOS", "QM0",
    "ETAQM", "CIT", "LCIT", "WCIT", "PCIT", "NFACTOR", "NFACTORL", "NFACTORLEXP", "NFACTORW", "NFACTORWEXP", "NFACTORWL", "NFACTORWLEXP", "LNFACTOR", "WNFACTOR", "PNFACTOR", "CDSCD",
    "CDSCDL", "CDSCDLEXP", "LCDSCD", "WCDSCD", "PCDSCD", "CDSCDR", "LCDSCDR", "WCDSCDR", "PCDSCDR", "CDSCB", "CDSCBL", "CDSCBLEXP", "LCDSCB", "WCDSCB", "PCDSCB", "VSAT",
    "LVSAT", "WVSAT", "PVSAT", "VSATL", "VSATLEXP", "VSATW", "VSATWEXP", "VSATWL", "VSATWLEXP", "VSATR", "LVSATR", "WVSATR", "PVSATR", "DELTA", "LDELTA", "WDELTA",
    "PDELTA", "DELTAL", "DELTALEXP", "VSATCV", "LVSATCV", "WVSATCV", "PVSATCV", "VSATCVL", "VSATCVLEXP", "VSATCVW", "VSATCVWEXP", "VSATCVWL", "VSATCVWLEXP", "UP1", "LP1", "UP2",
    "LP2", "U0", "U0L", "U0LEXP", "LU0", "WU0", "PU0", "U0R", "LU0R", "WU0R", "PU0R", "ETAMOB", "UA", "UAL", "UALEXP", "UAW",
    "UAWEXP", "UAWL", "UAWLEXP", "LUA", "WUA", "PUA", "UAR", "LUAR", "WUAR", "PUAR", "EU", "LEU", "WEU", "PEU", "EUL", "EULEXP",
    "EUW", "EUWEXP", "EUWL", "EUWLEXP", "UD", "UDL", "UDLEXP", "LUD", "WUD", "PUD", "UDR", "LUDR", "WUDR", "PUDR", "UCS", "LUCS",
    "WUCS", "PUCS", "UCSR", "LUCSR", "WUCSR", "PUCSR", "UC", "UCL", "UCLEXP", "UCW", "UCWEXP", "UCWL", "UCWLEXP", "LUC", "WUC", "PUC",
    "UCR", "LUCR", "WUCR", "PUCR", "PCLM", "PCLML", "PCLMLEXP", "LPCLM", "WPCLM", "PPCLM", "PCLMR", "LPCLMR", "WPCLMR", "PPCLMR", "PCLMG", "PCLMCV",
    "PCLMCVL", "PCLMCVLEXP", "LPCLMCV", "WPCLMCV", "PPCLMCV", "PSCBE1", "LPSCBE1", "WPSCBE1", "PPSCBE1", "PSCBE2", "LPSCBE2", "WPSCBE2", "PPSCBE2", "PDITS", "LPDITS", "WPDITS",
    "PPDITS", "PDITSL", "PDITSD", "LPDITSD", "WPDITSD", "PPDITSD", "RSH", "PRWG", "LPRWG", "WPRWG", "PPRWG", "PRWB", "LPRWB", "WPRWB", "PPRWB", "PRWBL",
    "PRWBLEXP", "WR", "LWR", "WWR", "PWR", "RSWMIN", "LRSWMIN", "WRSWMIN", "PRSWMIN", "RSW", "LRSW", "WRSW", "PRSW", "RSWL", "RSWLEXP", "RDWMIN",
    "LRDWMIN", "WRDWMIN", "PRDWMIN", "RDW", "LRDW", "WRDW", "PRDW", "RDWL", "RDWLEXP", "RDSWMIN", "LRDSWMIN", "WRDSWMIN", "PRDSWMIN", "RDSW", "RDSWL", "RDSWLEXP",
    "LRDSW", "WRDSW", "PRDSW", "PSAT", "LPSAT", "WPSAT", "PPSAT", "PSATL", "PSATLEXP", "PSATB", "PSATR", "LPSATR", "WPSATR", "PPSATR", "LPSATB", "WPSATB",
    "PPSATB", "PSATX", "PTWG", "LPTWG", "WPTWG", "PPTWG", "PTWGL", "PTWGLEXP", "PTWGR", "LPTWGR", "WPTWGR", "PPTWGR", "A1", "LA1", "WA1", "PA1",
    "A11", "LA11", "WA11", "PA11", "A2", "LA2", "WA2", "PA2", "A21", "LA21", "WA21", "PA21", "PDIBLC", "PDIBLCL", "PDIBLCLEXP", "LPDIBLC",
    "WPDIBLC", "PPDIBLC", "PDIBLCR", "LPDIBLCR", "WPDIBLCR", "PPDIBLCR", "PDIBLCB", "LPDIBLCB", "WPDIBLCB", "PPDIBLCB", "PVAG", "LPVAG", "WPVAG", "PPVAG", "FPROUT", "FPROUTL",
    "FPROUTLEXP", "LFPROUT", "WFPROUT", "PFPROUT", "ALPHA0", "ALPHA0L", "ALPHA0LEXP", "ALPHA0W", "ALPHA0WEXP", "LALPHA0", "WALPHA0", "PALPHA0", "ALPHA3", "ALPHA4", "BETA0", "BETA0W",
    "BETA0WEXP", "LBETA0", "WBETA0", "PBETA0", "ALPHADR", "BETADR", "DRII1", "DRII2", "DELTAII", "ALPHA1", "ALPHA2", "ALPHADR1", "ALPHADR2", "ALPHADR3", "ALPHADR4", "DREXP",
    "DRII3", "DRII4", "CMD1", "CMD2", "CMS1", "CMS2", "BETA1", "BETA1W", "BETA1WEXP", "BETA2", "BETA2W", "BETA2WEXP", "BETA3", "ALPHA0R", "LALPHA0R", "WALPHA0R",
    "PALPHA0R", "BETA0R", "LBETA0R", "WBETA0R", "PBETA0R", "AIGBACC", "BIGBACC", "CIGBACC", "NIGBACC", "AIGBINV", "BIGBINV", "CIGBINV", "EIGBINV", "NIGBINV", "AIGC", "BIGC",
    "CIGC", "AIGS", "BIGS", "CIGS", "AIGD", "BIGD", "CIGD", "DLCIG", "DLCIGD", "POXEDGE", "NTOX", "TOXREF", "PIGCD", "AIGCL", "AIGCW", "AIGSL",
    "AIGSW", "AIGDL", "AIGDW", "PIGCDL", "LAIGBINV", "WAIGBINV", "PAIGBINV", "LBIGBINV", "WBIGBINV", "PBIGBINV", "LCIGBINV", "WCIGBINV", "PCIGBINV", "LEIGBINV", "WEIGBINV", "PEIGBINV",
    "LNIGBINV", "WNIGBINV", "PNIGBINV", "LAIGBACC", "WAIGBACC", "PAIGBACC", "LBIGBACC", "WBIGBACC", "PBIGBACC", "LCIGBACC", "WCIGBACC", "PCIGBACC", "LNIGBACC", "WNIGBACC", "PNIGBACC", "LAIGC",
    "WAIGC", "PAIGC", "LBIGC", "WBIGC", "PBIGC", "LCIGC", "WCIGC", "PCIGC", "LAIGS", "WAIGS", "PAIGS", "LBIGS", "WBIGS", "PBIGS", "LCIGS", "WCIGS",
    "PCIGS", "LAIGD", "WAIGD", "PAIGD", "LBIGD", "WBIGD", "PBIGD", "LCIGD", "WCIGD", "PCIGD", "LPOXEDGE", "WPOXEDGE", "PPOXEDGE", "LDLCIG", "WDLCIG", "PDLCIG",
    "LDLCIGD", "WDLCIGD", "PDLCIGD", "LNTOX", "WNTOX", "PNTOX", "AGIDL", "AGIDLL", "AGIDLW", "LAGIDL", "WAGIDL", "PAGIDL", "BGIDL", "LBGIDL", "WBGIDL", "PBGIDL",
    "CGIDL", "LCGIDL", "WCGIDL", "PCGIDL", "EGIDL", "LEGIDL", "WEGIDL", "PEGIDL", "AGISL", "AGISLL", "AGISLW", "LAGISL", "WAGISL", "PAGISL", "BGISL", "LBGISL",
    "WBGISL", "PBGISL", "CGISL", "LCGISL", "WCGISL", "PCGISL", "EGISL", "LEGISL", "WEGISL", "PEGISL", "CF", "LCF", "WCF", "PCF", "CFRCOEFF", "CGSO",
    "CGDO", "CGBO", "CGSL", "LCGSL", "WCGSL", "PCGSL", "CGDL", "LCGDL", "WCGDL", "PCGDL", "CKAPPAS", "LCKAPPAS", "WCKAPPAS", "PCKAPPAS", "CKAPPAD", "LCKAPPAD",
    "WCKAPPAD", "PCKAPPAD", "CKAPPAD1", "CKAPPAD2", "CKAPPAS1", "CKAPPAS2", "SPQBACV", "DMCG", "DMCI", "DMDG", "DMCGT", "XGL", "RSHG", "CJS", "CJD", "CJSWS",
    "CJSWD", "CJSWGS", "CJSWGD", "PBS", "PBD", "PBSWS", "PBSWD", "PBSWGS", "PBSWGD", "MJS", "MJD", "MJSWS", "MJSWD", "MJSWGS", "MJSWGD", "JSS",
    "JSD", "JSWS", "JSWD", "JSWGS", "JSWGD", "NJS", "NJD", "IJTHSFWD", "IJTHDFWD", "IJTHSREV", "IJTHDREV", "BVS", "BVD", "XJBVS", "XJBVD", "JTSS",
    "JTSD", "JTSSWS", "JTSSWD", "JTSSWGS", "JTSSWGD", "JTWEFF", "NJTS", "NJTSD", "NJTSSW", "NJTSSWD", "NJTSSWG", "NJTSSWGD", "VTSS", "VTSD", "VTSSWS", "VTSSWD",
    "VTSSWGS", "VTSSWGD", "XRCRG1", "XRCRG2", "GBMIN", "RBPS0", "RBPSL", "RBPSW", "RBPSNF", "RBPD0", "RBPDL", "RBPDW", "RBPDNF", "RBPBX0", "RBPBXL", "RBPBXW",
    "RBPBXNF", "RBPBY0", "RBPBYL", "RBPBYW", "RBPBYNF", "RBSBX0", "RBSBY0", "RBDBX0", "RBDBY0", "RBSDBXL", "RBSDBXW", "RBSDBXNF", "RBSDBYL", "RBSDBYW", "RBSDBYNF", "EF",
    "EM", "NOIA", "NOIA3", "LNOIA3", "WNOIA3", "PNOIA3", "MPOWER", "LMPOWER", "WMPOWER", "PMPOWER", "QSREF", "LQSREF", "WQSREF", "PQSREF", "SPFN", "NOIB",
    "NOIC", "LINTNOI", "NOIA1", "NOIAX", "BFNS", "BFND", "KFNS", "KFND", "AFNS", "AFND", "NTNOI", "RNOIA", "RNOIB", "RNOIC", "TNOIA", "TNOIB",
    "TNOIC", "BINUNIT", "DLBIN", "DWBIN", "TNOM", "TBGASUB", "TBGBSUB", "TNFACTOR", "UTE", "LUTE", "WUTE", "PUTE", "UTEL", "UA1", "LUA1", "WUA1",
    "PUA1", "UA1L", "UC1", "LUC1", "WUC1", "PUC1", "UD1", "LUD1", "WUD1", "PUD1", "UD1L", "EU1", "LEU1", "WEU1", "PEU1", "UCSTE",
    "LUCSTE", "WUCSTE", "PUCSTE", "TETA0", "PRT", "LPRT", "WPRT", "PPRT", "AT", "LAT", "WAT", "PAT", "ATL", "TDELTA", "PTWGT", "LPTWGT",
    "WPTWGT", "PPTWGT", "PTWGTL", "KT1", "KT1EXP", "KT1L", "LKT1", "WKT1", "PKT1", "KT2", "LKT2", "WKT2", "PKT2", "IIT", "LIIT", "WIIT",
    "PIIT", "IGT", "LIGT", "WIGT", "PIGT", "TGIDL", "LTGIDL", "WTGIDL", "PTGIDL", "TCJ", "TCJSW", "TCJSWG", "TPB", "TPBSW", "TPBSWG", "XTIS",
    "XTID", "XTSS", "XTSD", "XTSSWS", "XTSSWD", "XTSSWGS", "XTSSWGD", "TNJTS", "TNJTSD", "TNJTSSW", "TNJTSSWD", "TNJTSSWG", "TNJTSSWGD", "RTH0", "CTH0", "WTH0",
    "SAREF", "SBREF", "WLOD", "KU0", "KVSAT", "TKU0", "LKU0", "WKU0", "PKU0", "LLODKU0", "WLODKU0", "KVTH0", "LKVTH0", "WKVTH0", "PKVTH0", "LLODVTH",
    "WLODVTH", "STK2", "LODK2", "STETA0", "LODETA0", "WEB", "WEC", "KVTH0WE", "LKVTH0WE", "WKVTH0WE", "PKVTH0WE", "K2WE", "LK2WE", "WK2WE", "PK2WE", "KU0WE",
    "LKU0WE", "WKU0WE", "PKU0WE", "SCREF", "SSL0", "SSL1", "SSL2", "SSL3", "SSL4", "SSL5", "SSLEXP1", "SSLEXP2", "AVDSX", "WEDGE", "DGAMMAEDGE", "DGAMMAEDGEL",
    "DGAMMAEDGELEXP", "DVTEDGE", "NDEPEDGE", "LNDEPEDGE", "WNDEPEDGE", "PNDEPEDGE", "NFACTOREDGE", "LNFACTOREDGE", "WNFACTOREDGE", "PNFACTOREDGE", "CITEDGE", "LCITEDGE", "WCITEDGE", "PCITEDGE", "CDSCDEDGE", "LCDSCDEDGE",
    "WCDSCDEDGE", "PCDSCDEDGE", "CDSCBEDGE", "LCDSCBEDGE", "WCDSCBEDGE", "PCDSCBEDGE", "ETA0EDGE", "LETA0EDGE", "WETA0EDGE", "PETA0EDGE", "ETABEDGE", "LETABEDGE", "WETABEDGE", "PETABEDGE", "KT1EDGE", "LKT1EDGE",
    "WKT1EDGE", "PKT1EDGE", "KT1LEDGE", "LKT1LEDGE", "WKT1LEDGE", "PKT1LEDGE", "KT2EDGE", "LKT2EDGE", "WKT2EDGE", "PKT2EDGE", "KT1EXPEDGE", "LKT1EXPEDGE", "WKT1EXPEDGE", "PKT1EXPEDGE", "TNFACTOREDGE", "LTNFACTOREDGE",
    "WTNFACTOREDGE", "PTNFACTOREDGE", "TETA0EDGE", "LTETA0EDGE", "WTETA0EDGE", "PTETA0EDGE", "DVT0EDGE", "DVT1EDGE", "DVT2EDGE", "K2EDGE", "LK2EDGE", "WK2EDGE", "PK2EDGE", "KVTH0EDGE", "LKVTH0EDGE", "WKVTH0EDGE",
    "PKVTH0EDGE", "KVTH0EDGEWE", "LKVTH0EDGEWE", "WKVTH0EDGEWE", "PKVTH0EDGEWE", "K2EDGEWE", "LK2EDGEWE", "WK2EDGEWE", "PK2EDGEWE", "STK2EDGE", "LSTK2EDGE", "WSTK2EDGE", "PSTK2EDGE", "STETA0EDGE", "LSTETA0EDGE", "WSTETA0EDGE",
    "PSTETA0EDGE", "IGCLAMP", "LP", "RNOIK", "TNOIK", "TNOIK2", "K0", "LK0", "WK0", "PK0", "K01", "LK01", "WK01", "PK01", "M0", "LM0",
    "WM0", "PM0", "M01", "LM01", "WM01", "PM01", "NEDGE", "NOIA1_EDGE", "NOIAX_EDGE", "FNOIMOD", "LH", "NOIA2", "HNDEP", "C0", "LC0", "WC0",
    "PC0", "C01", "LC01", "WC01", "PC01", "C0SI", "LC0SI", "WC0SI", "PC0SI", "C0SI1", "LC0SI1", "WC0SI1", "PC0SI1", "C0SISAT", "LC0SISAT", "WC0SISAT",
    "PC0SISAT", "C0SISAT1", "LC0SISAT1", "WC0SISAT1", "PC0SISAT1", "minr", "HVMOD", "HVCAP", "HVCAPS", "RBODYHVMOD", "IIMOD", "NDRIFTD", "VDRIFT", "PTWGHV", "PTWGHV1", "PSATXHV",
    "PTWGHVII", "PTWGHV1II", "PSATXHVII", "MDRIFT", "DSMOOTH", "NDRIFTS", "RDLCW", "RDLCWCV", "RSLCW", "PDRWB", "VFBOV", "LOVER", "LOVERACC", "NDR", "SLHV", "SLHV1",
    "PRTHV", "ATHV", "HVFACTOR", "ASYMP", "DRB1", "DRB2", "RDVDS", "GADRIFT", "XPART", "ABULK", "A0", "AGS", "AGS1", "KETA", "A0CV", "AGSCV",
    "KETACV", "CVSLOPE",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1138] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1138] = [
    false, false, true, false, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    true, false, false, false, false, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1138] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.1, label: "0.1" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -50.0, label: "-50.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.5, label: "0.5" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0001, label: "0.0001" }), None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1138] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }),
    None, None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, Some(ParameterBound { value: 5.0, label: "5.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 10.0, label: "10.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None, None,
    None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1138] = [
    3, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 0,
    0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0,
    0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3,
    0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 2, 0, 0, 3, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1,
    2, 2, 2, 0, 2, 0, 2, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 2, 3, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 3, 3, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 2, 2, 2, 3, 2, 2, 2, 3, 2, 2,
    2, 3, 2, 2, 2, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 1, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 2, 0,
    0, 0, 2, 3, 3, 3, 2, 2, 3, 3, 2, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3,
    3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 2, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3, 3, 2, 0, 3, 2, 0, 3, 3, 2, 3, 2, 2, 2, 0, 0, 0, 0, 3, 2, 3,
    0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 3, 2, 0, 0, 2, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1138] = [
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
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[],
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
        699 => Some(ParameterBound { value: ((params.p0 * params.p52) + params.p54), label: "computed upper-bound expression" }),
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
    pub nodes: [usize; 17],
    pub branches: [usize; 14],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1138]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 16]>,
    pub(crate) ddt_state_previous: Box<[f64; 16]>,
    pub(crate) ddt_state_older: Box<[f64; 16]>,
    pub(crate) ddt_state_initialized: Box<[bool; 16]>,
    pub(crate) ddt_derivative_current: Box<[f64; 16]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 16]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<1631, 17, 14>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<1631, 17, 14>>>,
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
    pub const TERMINAL_COUNT: usize = 5;
    pub const INTERNAL_NODE_COUNT: usize = 12;
    pub const NODE_COUNT: usize = 17;
    pub const INTERNAL_NODE_NAMES: [&str; 12] = ["di", "di1", "si", "si1", "gi", "gm", "bi", "sbulk", "dbulk", "ddbulk", "N1", "N2"];

    pub const BRANCH_COUNT: usize = 14;
    pub const PARAMETER_COUNT: usize = 1138;
    pub const VARIABLE_COUNT: usize = 1631;
    pub const DDT_STATE_COUNT: usize = 16;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "f5b8776f769992f9861c5d0594cfe6dadf07a1bdbe36a2ffbca9339f82e63b57";
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

    pub(crate) fn capture_persistent_state(&self) -> GeneratedVerilogAPersistentState {
        GeneratedVerilogAPersistentState {
            ddt_previous: self.ddt_state_previous.to_vec(),
            ddt_older: self.ddt_state_older.to_vec(),
            ddt_derivative_previous: self.ddt_derivative_previous.to_vec(),
            ddt_initialized: self.ddt_state_initialized.to_vec(),
            idt_previous: self.idt_state_previous.to_vec(),
            idt_initialized: self.idt_state_initialized.to_vec(),
            limiter_anchor: Vec::new(),
            limiter_initialized: Vec::new(),
        }
    }

    pub(crate) fn validate_persistent_state_shape(&self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        if state.ddt_previous.len() != Self::DDT_STATE_COUNT || state.ddt_older.len() != Self::DDT_STATE_COUNT || state.ddt_derivative_previous.len() != Self::DDT_STATE_COUNT || state.ddt_initialized.len() != Self::DDT_STATE_COUNT {
            return Err(format!("generated ddt checkpoint shape mismatch: expected {}, found {} / {} / {} / {}", Self::DDT_STATE_COUNT, state.ddt_previous.len(), state.ddt_older.len(), state.ddt_derivative_previous.len(), state.ddt_initialized.len()));
        }
        if state.idt_previous.len() != Self::IDT_STATE_COUNT || state.idt_initialized.len() != Self::IDT_STATE_COUNT {
            return Err(format!("generated idt checkpoint shape mismatch: expected {}, found {} / {}", Self::IDT_STATE_COUNT, state.idt_previous.len(), state.idt_initialized.len()));
        }
        if state.ddt_previous.iter().chain(&state.ddt_older).chain(&state.ddt_derivative_previous).chain(&state.idt_previous).chain(&state.limiter_anchor).any(|value| !value.is_finite()) {
            return Err("generated Verilog-A checkpoint contains non-finite persistent state".to_string());
        }
        Ok(())
    }

    pub(crate) fn restore_persistent_state(&mut self, state: &GeneratedVerilogAPersistentState) -> Result<(), String> {
        self.validate_persistent_state_shape(state)?;
        self.ddt_state_previous.copy_from_slice(&state.ddt_previous);
        self.ddt_state_current.copy_from_slice(&state.ddt_previous);
        self.ddt_state_older.copy_from_slice(&state.ddt_older);
        self.ddt_derivative_previous.copy_from_slice(&state.ddt_derivative_previous);
        self.ddt_derivative_current.copy_from_slice(&state.ddt_derivative_previous);
        self.ddt_state_initialized.copy_from_slice(&state.ddt_initialized);
        self.idt_state_previous.copy_from_slice(&state.idt_previous);
        self.idt_state_current.copy_from_slice(&state.idt_previous);
        self.idt_state_initialized.copy_from_slice(&state.idt_initialized);
        self.scratch = None;
        self.reactive_scratch = None;
        Ok(())
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        let lower = name.to_ascii_lowercase();
        let Some(index) = parameter_index_for_name(lower.as_str()) else {
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimbulk'", name));
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
