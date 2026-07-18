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
    pub p1136: f64, pub p1137: f64, pub p1138: f64, pub p1139: f64, pub p1140: f64, pub p1141: f64, pub p1142: f64, pub p1143: f64,
    pub p1144: f64, pub p1145: f64, pub p1146: f64, pub p1147: f64, pub p1148: f64, pub p1149: f64, pub p1150: f64, pub p1151: f64,
    pub p1152: f64, pub p1153: f64, pub p1154: f64, pub p1155: f64, pub p1156: f64, pub p1157: f64, pub p1158: f64, pub p1159: f64,
    pub p1160: f64, pub p1161: f64, pub p1162: f64, pub p1163: f64, pub p1164: f64, pub p1165: f64, pub p1166: f64, pub p1167: f64,
    pub p1168: f64, pub p1169: f64, pub p1170: f64, pub p1171: f64, pub p1172: f64, pub p1173: f64, pub p1174: f64, pub p1175: f64,
    pub p1176: f64, pub p1177: f64, pub p1178: f64, pub p1179: f64, pub p1180: f64, pub p1181: f64, pub p1182: f64, pub p1183: f64,
    pub p1184: f64, pub p1185: f64, pub p1186: f64, pub p1187: f64, pub p1188: f64, pub p1189: f64, pub p1190: f64, pub p1191: f64,
    pub p1192: f64, pub p1193: f64, pub p1194: f64, pub p1195: f64, pub p1196: f64, pub p1197: f64, pub p1198: f64, pub p1199: f64,
    pub p1200: f64, pub p1201: f64, pub p1202: f64, pub p1203: f64, pub p1204: f64, pub p1205: f64, pub p1206: f64, pub p1207: f64,
    pub p1208: f64, pub p1209: f64, pub p1210: f64, pub p1211: f64, pub p1212: f64, pub p1213: f64, pub p1214: f64, pub p1215: f64,
    pub p1216: f64, pub p1217: f64, pub p1218: f64, pub p1219: f64, pub p1220: f64, pub p1221: f64, pub p1222: f64, pub p1223: f64,
    pub p1224: f64, pub p1225: f64, pub p1226: f64, pub p1227: f64, pub p1228: f64, pub p1229: f64, pub p1230: f64, pub p1231: f64,
    pub p1232: f64, pub p1233: f64, pub p1234: f64, pub p1235: f64, pub p1236: f64, pub p1237: f64, pub p1238: f64, pub p1239: f64,
    pub p1240: f64, pub p1241: f64, pub p1242: f64, pub p1243: f64, pub p1244: f64, pub p1245: f64, pub p1246: f64, pub p1247: f64,
    pub p1248: f64, pub p1249: f64, pub p1250: f64, pub p1251: f64, pub p1252: f64, pub p1253: f64, pub p1254: f64, pub p1255: f64,
    pub p1256: f64, pub p1257: f64, pub p1258: f64, pub p1259: f64, pub p1260: f64, pub p1261: f64, pub p1262: f64, pub p1263: f64,
    pub p1264: f64, pub p1265: f64, pub p1266: f64, pub p1267: f64, pub p1268: f64, pub p1269: f64, pub p1270: f64, pub p1271: f64,
    pub p1272: f64, pub p1273: f64, pub p1274: f64, pub p1275: f64, pub p1276: f64, pub p1277: f64, pub p1278: f64, pub p1279: f64,
    pub p1280: f64, pub p1281: f64, pub p1282: f64, pub p1283: f64, pub p1284: f64, pub p1285: f64, pub p1286: f64, pub p1287: f64,
    pub p1288: f64, pub p1289: f64, pub p1290: f64, pub p1291: f64, pub p1292: f64, pub p1293: f64, pub p1294: f64, pub p1295: f64,
    pub p1296: f64, pub p1297: f64, pub p1298: f64, pub p1299: f64, pub p1300: f64, pub p1301: f64, pub p1302: f64, pub p1303: f64,
    pub p1304: f64, pub p1305: f64, pub p1306: f64, pub p1307: f64, pub p1308: f64, pub p1309: f64, pub p1310: f64, pub p1311: f64,
    pub p1312: f64, pub p1313: f64, pub p1314: f64, pub p1315: f64, pub p1316: f64, pub p1317: f64, pub p1318: f64, pub p1319: f64,
    pub p1320: f64, pub p1321: f64, pub p1322: f64, pub p1323: f64, pub p1324: f64, pub p1325: f64, pub p1326: f64, pub p1327: f64,
    pub p1328: f64, pub p1329: f64, pub p1330: f64, pub p1331: f64, pub p1332: f64, pub p1333: f64, pub p1334: f64, pub p1335: f64,
    pub p1336: f64, pub p1337: f64, pub p1338: f64, pub p1339: f64, pub p1340: f64, pub p1341: f64, pub p1342: f64, pub p1343: f64,
    pub p1344: f64, pub p1345: f64, pub p1346: f64, pub p1347: f64, pub p1348: f64, pub p1349: f64, pub p1350: f64, pub p1351: f64,
    pub p1352: f64, pub p1353: f64, pub p1354: f64, pub p1355: f64, pub p1356: f64, pub p1357: f64, pub p1358: f64, pub p1359: f64,
    pub p1360: f64, pub p1361: f64, pub p1362: f64, pub p1363: f64, pub p1364: f64, pub p1365: f64, pub p1366: f64, pub p1367: f64,
    pub p1368: f64, pub p1369: f64, pub p1370: f64, pub p1371: f64, pub p1372: f64, pub p1373: f64, pub p1374: f64, pub p1375: f64,
    pub p1376: f64, pub p1377: f64, pub p1378: f64, pub p1379: f64, pub p1380: f64, pub p1381: f64, pub p1382: f64, pub p1383: f64,
    pub p1384: f64, pub p1385: f64, pub p1386: f64, pub p1387: f64, pub p1388: f64, pub p1389: f64, pub p1390: f64, pub p1391: f64,
    pub p1392: f64, pub p1393: f64, pub p1394: f64, pub p1395: f64, pub p1396: f64, pub p1397: f64, pub p1398: f64, pub p1399: f64,
    pub p1400: f64, pub p1401: f64, pub p1402: f64, pub p1403: f64, pub p1404: f64, pub p1405: f64, pub p1406: f64, pub p1407: f64,
    pub p1408: f64, pub p1409: f64, pub p1410: f64, pub p1411: f64, pub p1412: f64, pub p1413: f64, pub p1414: f64, pub p1415: f64,
    pub p1416: f64, pub p1417: f64, pub p1418: f64, pub p1419: f64, pub p1420: f64, pub p1421: f64, pub p1422: f64, pub p1423: f64,
    pub p1424: f64, pub p1425: f64, pub p1426: f64, pub p1427: f64, pub p1428: f64, pub p1429: f64, pub p1430: f64, pub p1431: f64,
    pub p1432: f64, pub p1433: f64, pub p1434: f64, pub p1435: f64, pub p1436: f64, pub p1437: f64, pub p1438: f64, pub p1439: f64,
    pub p1440: f64, pub p1441: f64, pub p1442: f64, pub p1443: f64, pub p1444: f64, pub p1445: f64, pub p1446: f64, pub p1447: f64,
    pub p1448: f64, pub p1449: f64, pub p1450: f64, pub p1451: f64, pub p1452: f64, pub p1453: f64, pub p1454: f64, pub p1455: f64,
    pub p1456: f64, pub p1457: f64, pub p1458: f64, pub p1459: f64, pub p1460: f64, pub p1461: f64, pub p1462: f64, pub p1463: f64,
    pub p1464: f64, pub p1465: f64, pub p1466: f64, pub p1467: f64, pub p1468: f64, pub p1469: f64, pub p1470: f64, pub p1471: f64,
    pub p1472: f64, pub p1473: f64, pub p1474: f64, pub p1475: f64, pub p1476: f64, pub p1477: f64, pub p1478: f64, pub p1479: f64,
    pub p1480: f64, pub p1481: f64, pub p1482: f64, pub p1483: f64, pub p1484: f64, pub p1485: f64, pub p1486: f64, pub p1487: f64,
    pub p1488: f64, pub p1489: f64, pub p1490: f64, pub p1491: f64, pub p1492: f64, pub p1493: f64, pub p1494: f64, pub p1495: f64,
    pub p1496: f64, pub p1497: f64, pub p1498: f64, pub p1499: f64, pub p1500: f64, pub p1501: f64, pub p1502: f64, pub p1503: f64,
    pub p1504: f64, pub p1505: f64, pub p1506: f64, pub p1507: f64, pub p1508: f64, pub p1509: f64, pub p1510: f64, pub p1511: f64,
    pub p1512: f64, pub p1513: f64, pub p1514: f64, pub p1515: f64, pub p1516: f64, pub p1517: f64, pub p1518: f64, pub p1519: f64,
    pub p1520: f64, pub p1521: f64, pub p1522: f64, pub p1523: f64, pub p1524: f64, pub p1525: f64, pub p1526: f64, pub p1527: f64,
    pub p1528: f64, pub p1529: f64, pub p1530: f64, pub p1531: f64, pub p1532: f64, pub p1533: f64, pub p1534: f64, pub p1535: f64,
    pub p1536: f64, pub p1537: f64, pub p1538: f64, pub p1539: f64, pub p1540: f64, pub p1541: f64, pub p1542: f64, pub p1543: f64,
    pub p1544: f64, pub p1545: f64, pub p1546: f64, pub p1547: f64, pub p1548: f64, pub p1549: f64, pub p1550: f64, pub p1551: f64,
    pub p1552: f64, pub p1553: f64, pub p1554: f64, pub p1555: f64, pub p1556: f64, pub p1557: f64, pub p1558: f64, pub p1559: f64,
    pub p1560: f64, pub p1561: f64, pub p1562: f64, pub p1563: f64, pub p1564: f64, pub p1565: f64, pub p1566: f64, pub p1567: f64,
    pub p1568: f64, pub p1569: f64, pub p1570: f64, pub p1571: f64, pub p1572: f64, pub p1573: f64, pub p1574: f64, pub p1575: f64,
    pub p1576: f64, pub p1577: f64, pub p1578: f64, pub p1579: f64, pub p1580: f64, pub p1581: f64, pub p1582: f64, pub p1583: f64,
    pub p1584: f64, pub p1585: f64, pub p1586: f64, pub p1587: f64, pub p1588: f64, pub p1589: f64, pub p1590: f64, pub p1591: f64,
    pub p1592: f64, pub p1593: f64, pub p1594: f64, pub p1595: f64, pub p1596: f64, pub p1597: f64, pub p1598: f64, pub p1599: f64,
    pub p1600: f64, pub p1601: f64, pub p1602: f64, pub p1603: f64, pub p1604: f64, pub p1605: f64, pub p1606: f64, pub p1607: f64,
    pub p1608: f64, pub p1609: f64, pub p1610: f64, pub p1611: f64, pub p1612: f64, pub p1613: f64, pub p1614: f64, pub p1615: f64,
    pub p1616: f64, pub p1617: f64, pub p1618: f64, pub p1619: f64, pub p1620: f64, pub p1621: f64, pub p1622: f64, pub p1623: f64,
    pub p1624: f64, pub p1625: f64, pub p1626: f64, pub p1627: f64, pub p1628: f64, pub p1629: f64, pub p1630: f64, pub p1631: f64,
    pub p1632: f64, pub p1633: f64, pub p1634: f64, pub p1635: f64, pub p1636: f64, pub p1637: f64, pub p1638: f64, pub p1639: f64,
    pub p1640: f64, pub p1641: f64, pub p1642: f64, pub p1643: f64, pub p1644: f64, pub p1645: f64, pub p1646: f64, pub p1647: f64,
    pub p1648: f64, pub p1649: f64, pub p1650: f64, pub p1651: f64, pub p1652: f64, pub p1653: f64, pub p1654: f64, pub p1655: f64,
    pub p1656: f64, pub p1657: f64, pub p1658: f64, pub p1659: f64, pub p1660: f64, pub p1661: f64, pub p1662: f64, pub p1663: f64,
    pub p1664: f64, pub p1665: f64, pub p1666: f64, pub p1667: f64, pub p1668: f64, pub p1669: f64, pub p1670: f64, pub p1671: f64,
    pub p1672: f64, pub p1673: f64, pub p1674: f64, pub p1675: f64, pub p1676: f64, pub p1677: f64, pub p1678: f64, pub p1679: f64,
    pub p1680: f64, pub p1681: f64, pub p1682: f64, pub p1683: f64, pub p1684: f64, pub p1685: f64, pub p1686: f64, pub p1687: f64,
    pub p1688: f64, pub p1689: f64, pub p1690: f64, pub p1691: f64, pub p1692: f64, pub p1693: f64, pub p1694: f64, pub p1695: f64,
    pub p1696: f64, pub p1697: f64, pub p1698: f64, pub p1699: f64, pub p1700: f64, pub p1701: f64, pub p1702: f64, pub p1703: f64,
    pub p1704: f64, pub p1705: f64, pub p1706: f64, pub p1707: f64, pub p1708: f64, pub p1709: f64, pub p1710: f64, pub p1711: f64,
    pub p1712: f64, pub p1713: f64, pub p1714: f64, pub p1715: f64, pub p1716: f64, pub p1717: f64, pub p1718: f64, pub p1719: f64,
    pub p1720: f64, pub p1721: f64, pub p1722: f64, pub p1723: f64, pub p1724: f64, pub p1725: f64, pub p1726: f64, pub p1727: f64,
    pub p1728: f64, pub p1729: f64, pub p1730: f64, pub p1731: f64, pub p1732: f64, pub p1733: f64, pub p1734: f64, pub p1735: f64,
    pub p1736: f64, pub p1737: f64, pub p1738: f64, pub p1739: f64, pub p1740: f64, pub p1741: f64, pub p1742: f64, pub p1743: f64,
    pub p1744: f64, pub p1745: f64, pub p1746: f64, pub p1747: f64, pub p1748: f64, pub p1749: f64, pub p1750: f64, pub p1751: f64,
    pub p1752: f64, pub p1753: f64, pub p1754: f64, pub p1755: f64, pub p1756: f64, pub p1757: f64, pub p1758: f64, pub p1759: f64,
    pub p1760: f64, pub p1761: f64, pub p1762: f64, pub p1763: f64, pub p1764: f64, pub p1765: f64, pub p1766: f64, pub p1767: f64,
    pub p1768: f64, pub p1769: f64, pub p1770: f64, pub p1771: f64, pub p1772: f64, pub p1773: f64, pub p1774: f64, pub p1775: f64,
    pub p1776: f64, pub p1777: f64, pub p1778: f64, pub p1779: f64, pub p1780: f64, pub p1781: f64, pub p1782: f64, pub p1783: f64,
    pub p1784: f64, pub p1785: f64, pub p1786: f64, pub p1787: f64, pub p1788: f64, pub p1789: f64, pub p1790: f64, pub p1791: f64,
    pub p1792: f64, pub p1793: f64, pub p1794: f64, pub p1795: f64, pub p1796: f64, pub p1797: f64, pub p1798: f64, pub p1799: f64,
    pub p1800: f64, pub p1801: f64, pub p1802: f64, pub p1803: f64, pub p1804: f64, pub p1805: f64, pub p1806: f64, pub p1807: f64,
    pub p1808: f64, pub p1809: f64, pub p1810: f64, pub p1811: f64, pub p1812: f64, pub p1813: f64, pub p1814: f64, pub p1815: f64,
    pub p1816: f64, pub p1817: f64, pub p1818: f64, pub p1819: f64, pub p1820: f64, pub p1821: f64, pub p1822: f64, pub p1823: f64,
    pub p1824: f64, pub p1825: f64, pub p1826: f64, pub p1827: f64, pub p1828: f64, pub p1829: f64, pub p1830: f64, pub p1831: f64,
    pub p1832: f64, pub p1833: f64, pub p1834: f64, pub p1835: f64, pub p1836: f64, pub p1837: f64, pub p1838: f64, pub p1839: f64,
    pub p1840: f64, pub p1841: f64, pub p1842: f64, pub p1843: f64, pub p1844: f64, pub p1845: f64, pub p1846: f64, pub p1847: f64,
    pub p1848: f64, pub p1849: f64, pub p1850: f64, pub p1851: f64, pub p1852: f64, pub p1853: f64, pub p1854: f64, pub p1855: f64,
    pub p1856: f64, pub p1857: f64, pub p1858: f64, pub p1859: f64, pub p1860: f64, pub p1861: f64, pub p1862: f64, pub p1863: f64,
    pub p1864: f64, pub p1865: f64, pub p1866: f64, pub p1867: f64, pub p1868: f64, pub p1869: f64, pub p1870: f64, pub p1871: f64,
    pub p1872: f64, pub p1873: f64, pub p1874: f64, pub p1875: f64, pub p1876: f64, pub p1877: f64, pub p1878: f64, pub p1879: f64,
    pub p1880: f64, pub p1881: f64, pub p1882: f64, pub p1883: f64, pub p1884: f64, pub p1885: f64, pub p1886: f64, pub p1887: f64,
    pub p1888: f64, pub p1889: f64, pub p1890: f64, pub p1891: f64, pub p1892: f64, pub p1893: f64, pub p1894: f64, pub p1895: f64,
    pub p1896: f64, pub p1897: f64, pub p1898: f64, pub p1899: f64, pub p1900: f64, pub p1901: f64, pub p1902: f64, pub p1903: f64,
    pub p1904: f64, pub p1905: f64, pub p1906: f64, pub p1907: f64, pub p1908: f64, pub p1909: f64, pub p1910: f64, pub p1911: f64,
    pub p1912: f64, pub p1913: f64, pub p1914: f64, pub p1915: f64, pub p1916: f64, pub p1917: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 20] = [
                3e-8, 3e-8, 4e-8, 1.5e-8, 8e-8, 1.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 20);
            {
                let params = &mut *ptr;
                params.p20 = params.p0;
                validate_parameter("lrsd", params.p20, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 13] = [
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(21), 13);
            {
                let params = &mut *ptr;
                params.p34 = params.p28;
                validate_finite_parameter("covd", params.p34).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p35 = params.p29;
                validate_finite_parameter("lcovd", params.p35).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p36 = params.p30;
                validate_finite_parameter("ncovd", params.p36).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p37 = params.p31;
                validate_finite_parameter("pcovd", params.p37).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p38 = params.p32;
                validate_finite_parameter("wcovd", params.p38).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p39 = params.p33;
                validate_finite_parameter("p2covd", params.p39).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 6] = [
                5e-9, 2e-9, 5e-9, 6e-9, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(40), 6);
            {
                let params = &mut *ptr;
                params.p46 = params.p44;
                validate_parameter("dws2", params.p46, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p47 = params.p45;
                validate_parameter("dach2", params.p47, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p48 = params.p44;
                validate_parameter("dws3", params.p48, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p49 = params.p45;
                validate_parameter("dach3", params.p49, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p50 = params.p44;
                validate_parameter("dws4", params.p50, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p51 = params.p45;
                validate_parameter("dach4", params.p51, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p52 = params.p44;
                validate_parameter("dws5", params.p52, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p53 = params.p45;
                validate_parameter("dach5", params.p53, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p54 = params.p44;
                validate_parameter("dws6", params.p54, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p55 = params.p45;
                validate_parameter("dach6", params.p55, false, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 95] = [
                1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 1e-9, 1.2e-9, 1.4e-7, 3e-8, 0.0, 0.0, 0.0,
                100000.0, 2e26, 0.0, 0.0, 0.0, 100000.0, 3.9, 11.9,
                4.05, 1.1e16, 1.12, 2.86e25, 1e-15, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1e22, 0.0, 0.0,
                0.0, 4.61, 0.0, 0.0, 0.0, 0.0, 0.0, -0.2,
                -0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(56), 95);
            {
                let params = &mut *ptr;
                params.p151 = 0.001;
                validate_parameter("minr", params.p151, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 4] = [
                0.0, 100000.0, 0.0, 100000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(152), 4);
            {
                let params = &mut *ptr;
                params.p156 = params.p154;
                validate_finite_parameter("cdscdrn1", params.p156).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p157 = params.p155;
                validate_finite_parameter("cdscdrn2", params.p157).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 3] = [
                0.0, 100000.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(158), 3);
            {
                let params = &mut *ptr;
                params.p161 = params.p158;
                validate_finite_parameter("eta0n1cv", params.p161).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p162 = params.p159;
                validate_parameter("eta0n2cv", params.p162, false, Some((1e-5, "1e-5")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p163 = params.p160;
                validate_finite_parameter("eta0ltcv", params.p163).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(164), 1);
            {
                let params = &mut *ptr;
                params.p165 = params.p164;
                validate_finite_parameter("teta0cv", params.p165).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p166 = params.p164;
                validate_finite_parameter("teta0r", params.p166).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 27] = [
                0.0, 1e-7, 0.0, 1e-7, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(167), 27);
            {
                let params = &mut *ptr;
                params.p194 = params.p188;
                validate_finite_parameter("citr", params.p194).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p195 = params.p189;
                validate_finite_parameter("lcitr", params.p195).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p196 = params.p190;
                validate_finite_parameter("ncitr", params.p196).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p197 = params.p191;
                validate_finite_parameter("pcitr", params.p197).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p198 = params.p192;
                validate_finite_parameter("wcitr", params.p198).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p199 = params.p193;
                validate_finite_parameter("p2citr", params.p199).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 12] = [
                0.007, 0.0, 0.0, 0.0, 0.0, 0.0, 0.007, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(200), 12);
            {
                let params = &mut *ptr;
                params.p212 = params.p206;
                validate_finite_parameter("cdscdr", params.p212).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p213 = params.p207;
                validate_finite_parameter("lcdscdr", params.p213).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p214 = params.p208;
                validate_finite_parameter("ncdscdr", params.p214).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p215 = params.p209;
                validate_finite_parameter("pcdscdr", params.p215).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p216 = params.p210;
                validate_finite_parameter("wcdscdr", params.p216).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p217 = params.p211;
                validate_finite_parameter("p2cdscdr", params.p217).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(218), 12);
            {
                let params = &mut *ptr;
                params.p230 = params.p224;
                validate_finite_parameter("dvt1ss", params.p230).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p231 = params.p225;
                validate_finite_parameter("ldvt1ss", params.p231).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p232 = params.p226;
                validate_finite_parameter("ndvt1ss", params.p232).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p233 = params.p227;
                validate_finite_parameter("pdvt1ss", params.p233).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p234 = params.p228;
                validate_finite_parameter("wdvt1ss", params.p234).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p235 = params.p229;
                validate_finite_parameter("p2dvt1ss", params.p235).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 18] = [
                0.05, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(236), 18);
            {
                let params = &mut *ptr;
                params.p254 = params.p242;
                validate_finite_parameter("eta0r", params.p254).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p255 = params.p243;
                validate_finite_parameter("leta0r", params.p255).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p256 = params.p244;
                validate_finite_parameter("neta0r", params.p256).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p257 = params.p245;
                validate_finite_parameter("peta0r", params.p257).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p258 = params.p246;
                validate_finite_parameter("weta0r", params.p258).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p259 = params.p247;
                validate_finite_parameter("p2eta0r", params.p259).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p260 = params.p242;
                validate_finite_parameter("eta0cv", params.p260).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p261 = params.p243;
                validate_finite_parameter("leta0cv", params.p261).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p262 = params.p244;
                validate_finite_parameter("neta0cv", params.p262).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p263 = params.p245;
                validate_finite_parameter("peta0cv", params.p263).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p264 = params.p246;
                validate_finite_parameter("weta0cv", params.p264).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p265 = params.p247;
                validate_finite_parameter("p2eta0cv", params.p265).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 24] = [
                1.06, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 5e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(266), 24);
            {
                let params = &mut *ptr;
                params.p290 = params.p284;
                validate_finite_parameter("dvtshiftr", params.p290).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p291 = params.p285;
                validate_finite_parameter("ldvtshiftr", params.p291).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p292 = params.p286;
                validate_finite_parameter("ndvtshiftr", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p293 = params.p287;
                validate_finite_parameter("pdvtshiftr", params.p293).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p294 = params.p288;
                validate_finite_parameter("wdvtshiftr", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p295 = params.p289;
                validate_finite_parameter("p2dvtshiftr", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(296), 24);
            {
                let params = &mut *ptr;
                params.p320 = params.p308;
                validate_finite_parameter("k2si", params.p320).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p321 = params.p309;
                validate_finite_parameter("lk2si", params.p321).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p322 = params.p310;
                validate_finite_parameter("nk2si", params.p322).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p323 = params.p311;
                validate_finite_parameter("pk2si", params.p323).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p324 = params.p312;
                validate_finite_parameter("wk2si", params.p324).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p325 = params.p313;
                validate_finite_parameter("p2k2si", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p326 = params.p314;
                validate_finite_parameter("k2si1", params.p326).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p327 = params.p315;
                validate_finite_parameter("lk2si1", params.p327).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p328 = params.p316;
                validate_finite_parameter("nk2si1", params.p328).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p329 = params.p317;
                validate_finite_parameter("pk2si1", params.p329).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p330 = params.p318;
                validate_finite_parameter("wk2si1", params.p330).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p331 = params.p319;
                validate_finite_parameter("p2k2si1", params.p331).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(332), 12);
            {
                let params = &mut *ptr;
                params.p344 = params.p332;
                validate_finite_parameter("k2sisat", params.p344).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p345 = params.p333;
                validate_finite_parameter("lk2sisat", params.p345).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p346 = params.p334;
                validate_finite_parameter("nk2sisat", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p347 = params.p335;
                validate_finite_parameter("pk2sisat", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p348 = params.p336;
                validate_finite_parameter("wk2sisat", params.p348).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p349 = params.p337;
                validate_finite_parameter("p2k2sisat", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p350 = params.p338;
                validate_finite_parameter("k2sisat1", params.p350).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p351 = params.p339;
                validate_finite_parameter("lk2sisat1", params.p351).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p352 = params.p340;
                validate_finite_parameter("nk2sisat1", params.p352).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p353 = params.p341;
                validate_finite_parameter("pk2sisat1", params.p353).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p354 = params.p342;
                validate_finite_parameter("wk2sisat1", params.p354).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p355 = params.p343;
                validate_finite_parameter("p2k2sisat1", params.p355).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 76] = [
                0.7, 0.0, 0.0, 0.0, 0.0, 0.0, 1e-6, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.2e-8, 0.001, 0.001, 0.66, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.66, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 100000.0, 0.0, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(356), 76);
            {
                let params = &mut *ptr;
                params.p432 = params.p428;
                validate_finite_parameter("vsat1n1", params.p432).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p433 = params.p429;
                validate_finite_parameter("vsat1n2", params.p433).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p434 = params.p432;
                validate_finite_parameter("vsat1rn1", params.p434).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p435 = params.p433;
                validate_finite_parameter("vsat1rn2", params.p435).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p436 = params.p430;
                validate_finite_parameter("avsat1", params.p436).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p437 = params.p431;
                validate_finite_parameter("bvsat1", params.p437).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(438), 2);
            {
                let params = &mut *ptr;
                params.p440 = params.p430;
                validate_finite_parameter("avsatcv", params.p440).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p441 = params.p431;
                validate_finite_parameter("bvsatcv", params.p441).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p442 = params.p438;
                validate_finite_parameter("apsatcv", params.p442).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p443 = params.p439;
                validate_finite_parameter("bpsatcv", params.p443).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 2] = [
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(444), 2);
            {
                let params = &mut *ptr;
                params.p446 = params.p444;
                validate_finite_parameter("amexpr", params.p446).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p447 = params.p445;
                validate_finite_parameter("bmexpr", params.p447).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 4] = [
                0.0, 1e-7, 0.0, -4e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(448), 4);
            {
                let params = &mut *ptr;
                params.p452 = params.p450;
                validate_finite_parameter("tmexpr", params.p452).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 8] = [
                0.01, 85000.0, 85000.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(453), 8);
            {
                let params = &mut *ptr;
                params.p461 = params.p455;
                validate_finite_parameter("vsatr", params.p461).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p462 = params.p456;
                validate_finite_parameter("lvsatr", params.p462).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p463 = params.p457;
                validate_finite_parameter("nvsatr", params.p463).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p464 = params.p458;
                validate_finite_parameter("pvsatr", params.p464).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p465 = params.p459;
                validate_finite_parameter("wvsatr", params.p465).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p466 = params.p460;
                validate_finite_parameter("p2vsatr", params.p466).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p467 = params.p455;
                validate_finite_parameter("vsat1", params.p467).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p468 = params.p456;
                validate_finite_parameter("lvsat1", params.p468).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p469 = params.p457;
                validate_finite_parameter("nvsat1", params.p469).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p470 = params.p458;
                validate_finite_parameter("pvsat1", params.p470).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p471 = params.p459;
                validate_finite_parameter("wvsat1", params.p471).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p472 = params.p460;
                validate_finite_parameter("p2vsat1", params.p472).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p473 = params.p467;
                validate_finite_parameter("vsat1r", params.p473).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p474 = params.p468;
                validate_finite_parameter("lvsat1r", params.p474).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p475 = params.p469;
                validate_finite_parameter("nvsat1r", params.p475).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p476 = params.p470;
                validate_finite_parameter("pvsat1r", params.p476).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p477 = params.p471;
                validate_finite_parameter("wvsat1r", params.p477).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p478 = params.p472;
                validate_finite_parameter("p2vsat1r", params.p478).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 21] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.0002, -2e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(479), 21);
            {
                let params = &mut *ptr;
                params.p500 = params.p492;
                validate_finite_parameter("ksativr", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p501 = params.p493;
                validate_finite_parameter("lksativr", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p502 = params.p494;
                validate_finite_parameter("nksativr", params.p502).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p503 = params.p495;
                validate_finite_parameter("pksativr", params.p503).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p504 = params.p496;
                validate_finite_parameter("wksativr", params.p504).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p505 = params.p497;
                validate_finite_parameter("p2ksativr", params.p505).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p506 = params.p455;
                validate_finite_parameter("vsatcv", params.p506).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p507 = params.p456;
                validate_finite_parameter("lvsatcv", params.p507).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p508 = params.p457;
                validate_finite_parameter("nvsatcv", params.p508).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p509 = params.p458;
                validate_finite_parameter("pvsatcv", params.p509).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p510 = params.p459;
                validate_finite_parameter("wvsatcv", params.p510).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p511 = params.p460;
                validate_finite_parameter("p2vsatcv", params.p511).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 6] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(512), 6);
            {
                let params = &mut *ptr;
                params.p518 = params.p479;
                validate_finite_parameter("deltavsatcv", params.p518).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p519 = params.p480;
                validate_finite_parameter("ldeltavsatcv", params.p519).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p520 = params.p481;
                validate_finite_parameter("ndeltavsatcv", params.p520).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p521 = params.p482;
                validate_finite_parameter("pdeltavsatcv", params.p521).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p522 = params.p483;
                validate_finite_parameter("wdeltavsatcv", params.p522).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p523 = params.p484;
                validate_finite_parameter("p2deltavsatcv", params.p523).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p524 = params.p485;
                validate_finite_parameter("psatcv", params.p524).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p525 = params.p486;
                validate_finite_parameter("lpsatcv", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p526 = params.p487;
                validate_finite_parameter("npsatcv", params.p526).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p527 = params.p488;
                validate_finite_parameter("ppsatcv", params.p527).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p528 = params.p489;
                validate_finite_parameter("wpsatcv", params.p528).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p529 = params.p490;
                validate_finite_parameter("p2psatcv", params.p529).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 7] = [
                4.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(530), 7);
            {
                let params = &mut *ptr;
                params.p537 = params.p531;
                validate_finite_parameter("mexpr", params.p537).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p538 = params.p532;
                validate_finite_parameter("lmexpr", params.p538).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p539 = params.p533;
                validate_finite_parameter("nmexpr", params.p539).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p540 = params.p534;
                validate_finite_parameter("pmexpr", params.p540).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p541 = params.p535;
                validate_finite_parameter("wmexpr", params.p541).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p542 = params.p536;
                validate_finite_parameter("p2mexpr", params.p542).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(543), 6);
            {
                let params = &mut *ptr;
                params.p549 = params.p543;
                validate_finite_parameter("ptwgr", params.p549).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p550 = params.p544;
                validate_finite_parameter("lptwgr", params.p550).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p551 = params.p545;
                validate_finite_parameter("nptwgr", params.p551).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p552 = params.p546;
                validate_finite_parameter("pptwgr", params.p552).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p553 = params.p547;
                validate_finite_parameter("wptwgr", params.p553).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p554 = params.p548;
                validate_finite_parameter("p2ptwgr", params.p554).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 7] = [
                -0.00156, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-6,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(555), 7);
            {
                let params = &mut *ptr;
                params.p562 = params.p555;
                validate_finite_parameter("atr", params.p562).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p563 = params.p556;
                validate_finite_parameter("latr", params.p563).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p564 = params.p557;
                validate_finite_parameter("natr", params.p564).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p565 = params.p558;
                validate_finite_parameter("patr", params.p565).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p566 = params.p559;
                validate_finite_parameter("watr", params.p566).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p567 = params.p560;
                validate_finite_parameter("p2atr", params.p567).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p568 = params.p555;
                validate_finite_parameter("atcv", params.p568).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p569 = params.p556;
                validate_finite_parameter("latcv", params.p569).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p570 = params.p557;
                validate_finite_parameter("natcv", params.p570).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p571 = params.p558;
                validate_finite_parameter("patcv", params.p571).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p572 = params.p559;
                validate_finite_parameter("watcv", params.p572).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p573 = params.p560;
                validate_finite_parameter("p2atcv", params.p573).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p574 = params.p561;
                validate_finite_parameter("at2cv", params.p574).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 7] = [
                0.004, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(575), 7);
            {
                let params = &mut *ptr;
                params.p582 = params.p581;
                validate_finite_parameter("u0n1cv", params.p582).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p583 = params.p581;
                validate_finite_parameter("u0n1r", params.p583).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                100000.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(584), 1);
            {
                let params = &mut *ptr;
                params.p585 = params.p584;
                validate_finite_parameter("u0n2cv", params.p585).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p586 = params.p584;
                validate_finite_parameter("u0n2r", params.p586).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 3] = [
                0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(587), 3);
            {
                let params = &mut *ptr;
                params.p590 = params.p589;
                validate_finite_parameter("lpar", params.p590).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(591), 1);
            {
                let params = &mut *ptr;
                params.p592 = params.p591;
                validate_finite_parameter("auar", params.p592).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(593), 1);
            {
                let params = &mut *ptr;
                params.p594 = params.p593;
                validate_finite_parameter("buar", params.p594).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(595), 1);
            {
                let params = &mut *ptr;
                params.p596 = params.p595;
                validate_finite_parameter("aeur", params.p596).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(597), 1);
            {
                let params = &mut *ptr;
                params.p598 = params.p597;
                validate_finite_parameter("beur", params.p598).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(599), 1);
            {
                let params = &mut *ptr;
                params.p600 = params.p599;
                validate_finite_parameter("audr", params.p600).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                5e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(601), 1);
            {
                let params = &mut *ptr;
                params.p602 = params.p601;
                validate_finite_parameter("budr", params.p602).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 8] = [
                0.0, 0.01, 0.03, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(603), 8);
            {
                let params = &mut *ptr;
                params.p611 = params.p605;
                validate_finite_parameter("u0r", params.p611).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p612 = params.p606;
                validate_finite_parameter("lu0r", params.p612).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p613 = params.p607;
                validate_finite_parameter("nu0r", params.p613).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p614 = params.p608;
                validate_finite_parameter("pu0r", params.p614).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p615 = params.p609;
                validate_finite_parameter("wu0r", params.p615).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p616 = params.p610;
                validate_finite_parameter("p2u0r", params.p616).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p617 = params.p605;
                validate_finite_parameter("u0cv", params.p617).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p618 = params.p606;
                validate_finite_parameter("lu0cv", params.p618).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p619 = params.p607;
                validate_finite_parameter("nu0cv", params.p619).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p620 = params.p608;
                validate_finite_parameter("pu0cv", params.p620).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p621 = params.p609;
                validate_finite_parameter("wu0cv", params.p621).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p622 = params.p610;
                validate_finite_parameter("p2u0cv", params.p622).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 12] = [
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(623), 12);
            {
                let params = &mut *ptr;
                params.p635 = params.p629;
                validate_finite_parameter("upr", params.p635).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p636 = params.p630;
                validate_finite_parameter("lupr", params.p636).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p637 = params.p631;
                validate_finite_parameter("nupr", params.p637).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p638 = params.p632;
                validate_finite_parameter("pupr", params.p638).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p639 = params.p633;
                validate_finite_parameter("wupr", params.p639).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p640 = params.p634;
                validate_finite_parameter("p2upr", params.p640).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 6] = [
                0.3, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(641), 6);
            {
                let params = &mut *ptr;
                params.p647 = params.p641;
                validate_finite_parameter("uar", params.p647).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p648 = params.p642;
                validate_finite_parameter("luar", params.p648).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p649 = params.p643;
                validate_finite_parameter("nuar", params.p649).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p650 = params.p644;
                validate_finite_parameter("puar", params.p650).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p651 = params.p645;
                validate_finite_parameter("wuar", params.p651).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p652 = params.p646;
                validate_finite_parameter("p2uar", params.p652).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p653 = params.p641;
                validate_finite_parameter("uacv", params.p653).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p654 = params.p642;
                validate_finite_parameter("luacv", params.p654).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p655 = params.p643;
                validate_finite_parameter("nuacv", params.p655).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p656 = params.p644;
                validate_finite_parameter("puacv", params.p656).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p657 = params.p645;
                validate_finite_parameter("wuacv", params.p657).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p658 = params.p646;
                validate_finite_parameter("p2uacv", params.p658).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(659), 6);
            {
                let params = &mut *ptr;
                params.p665 = params.p659;
                validate_finite_parameter("ucr", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p666 = params.p660;
                validate_finite_parameter("lucr", params.p666).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p667 = params.p661;
                validate_finite_parameter("nucr", params.p667).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p668 = params.p662;
                validate_finite_parameter("pucr", params.p668).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p669 = params.p663;
                validate_finite_parameter("wucr", params.p669).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p670 = params.p664;
                validate_finite_parameter("p2ucr", params.p670).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p671 = params.p659;
                validate_finite_parameter("uccv", params.p671).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p672 = params.p660;
                validate_finite_parameter("luccv", params.p672).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p673 = params.p661;
                validate_finite_parameter("nuccv", params.p673).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p674 = params.p662;
                validate_finite_parameter("puccv", params.p674).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p675 = params.p663;
                validate_finite_parameter("wuccv", params.p675).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p676 = params.p664;
                validate_finite_parameter("p2uccv", params.p676).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 6] = [
                2.5, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(677), 6);
            {
                let params = &mut *ptr;
                params.p683 = params.p677;
                validate_finite_parameter("eur", params.p683).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p684 = params.p678;
                validate_finite_parameter("leur", params.p684).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p685 = params.p679;
                validate_finite_parameter("neur", params.p685).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p686 = params.p680;
                validate_finite_parameter("peur", params.p686).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p687 = params.p681;
                validate_finite_parameter("weur", params.p687).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p688 = params.p682;
                validate_finite_parameter("p2eur", params.p688).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(689), 6);
            {
                let params = &mut *ptr;
                params.p695 = params.p689;
                validate_finite_parameter("udr", params.p695).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p696 = params.p690;
                validate_finite_parameter("ludr", params.p696).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p697 = params.p691;
                validate_finite_parameter("nudr", params.p697).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p698 = params.p692;
                validate_finite_parameter("pudr", params.p698).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p699 = params.p693;
                validate_finite_parameter("wudr", params.p699).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p700 = params.p694;
                validate_finite_parameter("p2udr", params.p700).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p701 = params.p689;
                validate_finite_parameter("udcv", params.p701).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p702 = params.p690;
                validate_finite_parameter("ludcv", params.p702).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p703 = params.p691;
                validate_finite_parameter("nudcv", params.p703).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p704 = params.p692;
                validate_finite_parameter("pudcv", params.p704).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p705 = params.p693;
                validate_finite_parameter("wudcv", params.p705).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p706 = params.p694;
                validate_finite_parameter("p2udcv", params.p706).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 36] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-5, 0.0,
                0.0, 0.0, 0.0, 0.0, -10.0, 0.0, 0.0, 0.0,
                0.0, 0.0, -2e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
                -10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(707), 36);
            {
                let params = &mut *ptr;
                params.p743 = params.p737;
                validate_finite_parameter("uter", params.p743).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p744 = params.p738;
                validate_finite_parameter("luter", params.p744).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p745 = params.p739;
                validate_finite_parameter("nuter", params.p745).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p746 = params.p740;
                validate_finite_parameter("puter", params.p746).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p747 = params.p741;
                validate_finite_parameter("wuter", params.p747).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p748 = params.p742;
                validate_finite_parameter("p2uter", params.p748).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p749 = params.p737;
                validate_finite_parameter("utecv", params.p749).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p750 = params.p738;
                validate_finite_parameter("lutecv", params.p750).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p751 = params.p739;
                validate_finite_parameter("nutecv", params.p751).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p752 = params.p740;
                validate_finite_parameter("putecv", params.p752).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p753 = params.p741;
                validate_finite_parameter("wutecv", params.p753).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p754 = params.p742;
                validate_finite_parameter("p2utecv", params.p754).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 6] = [
                -0.4, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(755), 6);
            {
                let params = &mut *ptr;
                params.p761 = params.p755;
                validate_finite_parameter("ute1cv", params.p761).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p762 = params.p756;
                validate_finite_parameter("lute1cv", params.p762).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p763 = params.p757;
                validate_finite_parameter("nute1cv", params.p763).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p764 = params.p758;
                validate_finite_parameter("pute1cv", params.p764).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p765 = params.p759;
                validate_finite_parameter("wute1cv", params.p765).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p766 = params.p760;
                validate_finite_parameter("p2ute1cv", params.p766).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 6] = [
                -0.0015, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(767), 6);
            {
                let params = &mut *ptr;
                params.p773 = params.p767;
                validate_finite_parameter("utlr", params.p773).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p774 = params.p768;
                validate_finite_parameter("lutlr", params.p774).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p775 = params.p769;
                validate_finite_parameter("nutlr", params.p775).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p776 = params.p770;
                validate_finite_parameter("putlr", params.p776).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p777 = params.p771;
                validate_finite_parameter("wutlr", params.p777).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p778 = params.p772;
                validate_finite_parameter("p2utlr", params.p778).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p779 = params.p767;
                validate_finite_parameter("utlcv", params.p779).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p780 = params.p768;
                validate_finite_parameter("lutlcv", params.p780).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p781 = params.p769;
                validate_finite_parameter("nutlcv", params.p781).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p782 = params.p770;
                validate_finite_parameter("putlcv", params.p782).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p783 = params.p771;
                validate_finite_parameter("wutlcv", params.p783).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p784 = params.p772;
                validate_finite_parameter("p2utlcv", params.p784).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.001032, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (ptr as *mut f64).add(785), 12);
            {
                let params = &mut *ptr;
                params.p797 = params.p791;
                validate_finite_parameter("ua1r", params.p797).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p798 = params.p792;
                validate_finite_parameter("lua1r", params.p798).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p799 = params.p793;
                validate_finite_parameter("nua1r", params.p799).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p800 = params.p794;
                validate_finite_parameter("pua1r", params.p800).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p801 = params.p795;
                validate_finite_parameter("wua1r", params.p801).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p802 = params.p796;
                validate_finite_parameter("p2ua1r", params.p802).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p803 = params.p791;
                validate_finite_parameter("ua1cv", params.p803).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p804 = params.p792;
                validate_finite_parameter("lua1cv", params.p804).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p805 = params.p793;
                validate_finite_parameter("nua1cv", params.p805).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p806 = params.p794;
                validate_finite_parameter("pua1cv", params.p806).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p807 = params.p795;
                validate_finite_parameter("wua1cv", params.p807).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p808 = params.p796;
                validate_finite_parameter("p2ua1cv", params.p808).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 6] = [
                -0.04, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (ptr as *mut f64).add(809), 6);
            {
                let params = &mut *ptr;
                params.p815 = params.p809;
                validate_finite_parameter("ua2cv", params.p815).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p816 = params.p810;
                validate_finite_parameter("lua2cv", params.p816).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p817 = params.p811;
                validate_finite_parameter("nua2cv", params.p817).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p818 = params.p812;
                validate_finite_parameter("pua2cv", params.p818).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p819 = params.p813;
                validate_finite_parameter("wua2cv", params.p819).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p820 = params.p814;
                validate_finite_parameter("p2ua2cv", params.p820).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 12] = [
                -0.001, 0.0, 0.0, 0.0, 0.0, 0.0, 5.6e-11, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (ptr as *mut f64).add(821), 12);
            {
                let params = &mut *ptr;
                params.p833 = params.p827;
                validate_finite_parameter("uc1r", params.p833).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p834 = params.p828;
                validate_finite_parameter("luc1r", params.p834).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p835 = params.p829;
                validate_finite_parameter("nuc1r", params.p835).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p836 = params.p830;
                validate_finite_parameter("puc1r", params.p836).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p837 = params.p831;
                validate_finite_parameter("wuc1r", params.p837).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p838 = params.p832;
                validate_finite_parameter("p2uc1r", params.p838).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p839 = params.p827;
                validate_finite_parameter("uc1cv", params.p839).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p840 = params.p828;
                validate_finite_parameter("luc1cv", params.p840).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p841 = params.p829;
                validate_finite_parameter("nuc1cv", params.p841).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p842 = params.p830;
                validate_finite_parameter("puc1cv", params.p842).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p843 = params.p831;
                validate_finite_parameter("wuc1cv", params.p843).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p844 = params.p832;
                validate_finite_parameter("p2uc1cv", params.p844).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (ptr as *mut f64).add(845), 6);
            {
                let params = &mut *ptr;
                params.p851 = params.p845;
                validate_finite_parameter("ud1r", params.p851).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p852 = params.p846;
                validate_finite_parameter("lud1r", params.p852).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p853 = params.p847;
                validate_finite_parameter("nud1r", params.p853).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p854 = params.p848;
                validate_finite_parameter("pud1r", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p855 = params.p849;
                validate_finite_parameter("wud1r", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p856 = params.p850;
                validate_finite_parameter("p2ud1r", params.p856).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p857 = params.p845;
                validate_finite_parameter("ud1cv", params.p857).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p858 = params.p846;
                validate_finite_parameter("lud1cv", params.p858).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p859 = params.p847;
                validate_finite_parameter("nud1cv", params.p859).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p860 = params.p848;
                validate_finite_parameter("pud1cv", params.p860).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p861 = params.p849;
                validate_finite_parameter("wud1cv", params.p861).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p862 = params.p850;
                validate_finite_parameter("p2ud1cv", params.p862).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 6] = [
                -0.04, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (ptr as *mut f64).add(863), 6);
            {
                let params = &mut *ptr;
                params.p869 = params.p863;
                validate_finite_parameter("ud2cv", params.p869).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p870 = params.p864;
                validate_finite_parameter("lud2cv", params.p870).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p871 = params.p865;
                validate_finite_parameter("nud2cv", params.p871).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p872 = params.p866;
                validate_finite_parameter("pud2cv", params.p872).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p873 = params.p867;
                validate_finite_parameter("wud2cv", params.p873).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p874 = params.p868;
                validate_finite_parameter("p2ud2cv", params.p874).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 14] = [
                -0.004775, 0.0, 0.0, 0.0, 0.0, 0.0, -0.04, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (ptr as *mut f64).add(875), 14);
            {
                let params = &mut *ptr;
                params.p889 = params.p623;
                validate_finite_parameter("etamobthin", params.p889).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 2] = [
                7.5e-9, 0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (ptr as *mut f64).add(890), 2);
            {
                let params = &mut *ptr;
                params.p892 = params.p641;
                validate_finite_parameter("uathin", params.p892).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 4] = [
                9e-9, 0.09, 6.4e-9, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (ptr as *mut f64).add(893), 4);
            {
                let params = &mut *ptr;
                params.p897 = params.p677;
                validate_finite_parameter("euthin", params.p897).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 3] = [
                3.5, 6e-9, 0.2,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (ptr as *mut f64).add(898), 3);
            {
                let params = &mut *ptr;
                params.p901 = params.p689;
                validate_finite_parameter("udthin", params.p901).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 16] = [
                8.1e-9, 1.3, 1.5, 1.1, 26.6, 4.0, 0.0, 0.0,
                1e-7, 0.0, 0.0, 1e-7, 0.0, 0.0, 1e-7, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (ptr as *mut f64).add(902), 16);
            {
                let params = &mut *ptr;
                params.p918 = params.p917;
                validate_parameter("rsdrr", params.p918, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p919 = params.p917;
                validate_parameter("rddr", params.p919, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p920 = params.p919;
                validate_parameter("rddrr", params.p920, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (ptr as *mut f64).add(921), 1);
            {
                let params = &mut *ptr;
                params.p922 = params.p921;
                validate_finite_parameter("prddr", params.p922).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (ptr as *mut f64).add(923), 1);
            {
                let params = &mut *ptr;
                params.p924 = params.p923;
                validate_finite_parameter("trddr", params.p924).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 24] = [
                100.0, 0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 50.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (ptr as *mut f64).add(925), 24);
            {
                let params = &mut *ptr;
                params.p949 = params.p943;
                validate_finite_parameter("prwgd", params.p949).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 47] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.001, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0004, 0.0, 0.0, 0.0, 0.0, 0.0, 170.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.01, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.3, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0002, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (ptr as *mut f64).add(950), 47);
            {
                let params = &mut *ptr;
                params.p997 = params.p985;
                validate_finite_parameter("pdibl1r", params.p997).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p998 = params.p986;
                validate_finite_parameter("lpdibl1r", params.p998).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p999 = params.p987;
                validate_finite_parameter("npdibl1r", params.p999).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1000 = params.p988;
                validate_finite_parameter("ppdibl1r", params.p1000).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1001 = params.p989;
                validate_finite_parameter("wpdibl1r", params.p1001).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1002 = params.p990;
                validate_finite_parameter("p2pdibl1r", params.p1002).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1003 = params.p991;
                validate_finite_parameter("pdibl2r", params.p1003).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1004 = params.p992;
                validate_finite_parameter("lpdibl2r", params.p1004).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1005 = params.p993;
                validate_finite_parameter("npdibl2r", params.p1005).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1006 = params.p994;
                validate_finite_parameter("ppdibl2r", params.p1006).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1007 = params.p995;
                validate_finite_parameter("wpdibl2r", params.p1007).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1008 = params.p996;
                validate_finite_parameter("p2pdibl2r", params.p1008).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 13] = [
                1.06, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (ptr as *mut f64).add(1009), 13);
            {
                let params = &mut *ptr;
                params.p1022 = params.p1021;
                validate_finite_parameter("apclmr", params.p1022).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (ptr as *mut f64).add(1023), 1);
            {
                let params = &mut *ptr;
                params.p1024 = params.p1023;
                validate_finite_parameter("bpclmr", params.p1024).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 7] = [
                0.013, -2e-5, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (ptr as *mut f64).add(1025), 7);
            {
                let params = &mut *ptr;
                params.p1032 = params.p1025;
                validate_finite_parameter("pclmr", params.p1032).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1033 = params.p1027;
                validate_finite_parameter("lpclmr", params.p1033).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1034 = params.p1028;
                validate_finite_parameter("npclmr", params.p1034).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1035 = params.p1029;
                validate_finite_parameter("ppclmr", params.p1035).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1036 = params.p1030;
                validate_finite_parameter("wpclmr", params.p1036).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1037 = params.p1031;
                validate_finite_parameter("p2pclmr", params.p1037).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 6] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (ptr as *mut f64).add(1038), 6);
            {
                let params = &mut *ptr;
                params.p1044 = params.p1025;
                validate_finite_parameter("pclmcv", params.p1044).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1045 = params.p1027;
                validate_finite_parameter("lpclmcv", params.p1045).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1046 = params.p1028;
                validate_finite_parameter("npclmcv", params.p1046).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1047 = params.p1029;
                validate_finite_parameter("ppclmcv", params.p1047).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1048 = params.p1030;
                validate_finite_parameter("wpclmcv", params.p1048).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1049 = params.p1031;
                validate_finite_parameter("p2pclmcv", params.p1049).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.001, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (ptr as *mut f64).add(1050), 29);
            {
                let params = &mut *ptr;
                params.p1079 = params.p1078;
                validate_finite_parameter("rshd", params.p1079).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 25] = [
                1e-8, 1e-8, 1e-12, 1.0, 0.5, 0.0, 0.0, 6e-9,
                3.9, 3e-8, 3e-8, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (ptr as *mut f64).add(1080), 25);
            {
                let params = &mut *ptr;
                params.p1105 = params.p1104;
                validate_finite_parameter("dlcigd", params.p1105).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (ptr as *mut f64).add(1106), 1);
            {
                let params = &mut *ptr;
                params.p1107 = params.p1106;
                validate_finite_parameter("vfbsdcv", params.p1107).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                1.2e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (ptr as *mut f64).add(1108), 1);
            {
                let params = &mut *ptr;
                params.p1109 = params.p90;
                validate_parameter("toxg", params.p1109, false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 129] = [
                0.001, 0.001, 0.0005, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0111, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.000949, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.006, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.1, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0136, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.00171, 0.0, 0.0, 0.0, 0.0, 0.0, 0.075,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0136, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00171,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.075, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0136, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.00171, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.075, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (ptr as *mut f64).add(1110), 129);
            {
                let params = &mut *ptr;
                params.p1239 = params.p1215;
                validate_finite_parameter("aigd", params.p1239).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1240 = params.p1216;
                validate_finite_parameter("laigd", params.p1240).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1241 = params.p1217;
                validate_finite_parameter("naigd", params.p1241).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1242 = params.p1218;
                validate_finite_parameter("paigd", params.p1242).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1243 = params.p1219;
                validate_finite_parameter("waigd", params.p1243).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1244 = params.p1220;
                validate_finite_parameter("p2aigd", params.p1244).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1245 = params.p1221;
                validate_finite_parameter("aigd1", params.p1245).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1246 = params.p1222;
                validate_finite_parameter("laigd1", params.p1246).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1247 = params.p1223;
                validate_finite_parameter("naigd1", params.p1247).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1248 = params.p1224;
                validate_finite_parameter("paigd1", params.p1248).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1249 = params.p1225;
                validate_finite_parameter("waigd1", params.p1249).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1250 = params.p1226;
                validate_finite_parameter("p2aigd1", params.p1250).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1251 = params.p1227;
                validate_finite_parameter("bigd", params.p1251).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1252 = params.p1228;
                validate_finite_parameter("lbigd", params.p1252).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1253 = params.p1229;
                validate_finite_parameter("nbigd", params.p1253).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1254 = params.p1230;
                validate_finite_parameter("pbigd", params.p1254).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1255 = params.p1231;
                validate_finite_parameter("wbigd", params.p1255).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1256 = params.p1232;
                validate_finite_parameter("p2bigd", params.p1256).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1257 = params.p1233;
                validate_finite_parameter("cigd", params.p1257).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1258 = params.p1234;
                validate_finite_parameter("lcigd", params.p1258).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1259 = params.p1235;
                validate_finite_parameter("ncigd", params.p1259).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1260 = params.p1236;
                validate_finite_parameter("pcigd", params.p1260).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1261 = params.p1237;
                validate_finite_parameter("wcigd", params.p1261).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1262 = params.p1238;
                validate_finite_parameter("p2cigd", params.p1262).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 36] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 6.055e-12, 0.0,
                0.0, 0.0, 0.0, 0.0, 300000000.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (ptr as *mut f64).add(1263), 36);
            {
                let params = &mut *ptr;
                params.p1299 = params.p1269;
                validate_finite_parameter("agisl", params.p1299).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1300 = params.p1270;
                validate_finite_parameter("lagisl", params.p1300).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1301 = params.p1271;
                validate_finite_parameter("nagisl", params.p1301).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1302 = params.p1272;
                validate_finite_parameter("pagisl", params.p1302).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1303 = params.p1273;
                validate_finite_parameter("wagisl", params.p1303).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1304 = params.p1274;
                validate_finite_parameter("p2agisl", params.p1304).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1305 = params.p1275;
                validate_finite_parameter("bgisl", params.p1305).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1306 = params.p1276;
                validate_finite_parameter("lbgisl", params.p1306).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1307 = params.p1277;
                validate_finite_parameter("nbgisl", params.p1307).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1308 = params.p1278;
                validate_finite_parameter("pbgisl", params.p1308).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1309 = params.p1279;
                validate_finite_parameter("wbgisl", params.p1309).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1310 = params.p1280;
                validate_finite_parameter("p2bgisl", params.p1310).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1311 = params.p1281;
                validate_finite_parameter("cgisl", params.p1311).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1312 = params.p1282;
                validate_finite_parameter("lcgisl", params.p1312).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1313 = params.p1283;
                validate_finite_parameter("ncgisl", params.p1313).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1314 = params.p1284;
                validate_finite_parameter("pcgisl", params.p1314).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1315 = params.p1285;
                validate_finite_parameter("wcgisl", params.p1315).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1316 = params.p1286;
                validate_finite_parameter("p2cgisl", params.p1316).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1317 = params.p1287;
                validate_finite_parameter("egisl", params.p1317).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1318 = params.p1288;
                validate_finite_parameter("legisl", params.p1318).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1319 = params.p1289;
                validate_finite_parameter("negisl", params.p1319).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1320 = params.p1290;
                validate_finite_parameter("pegisl", params.p1320).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1321 = params.p1291;
                validate_finite_parameter("wegisl", params.p1321).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1322 = params.p1292;
                validate_finite_parameter("p2egisl", params.p1322).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1323 = params.p1293;
                validate_finite_parameter("pgisl", params.p1323).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1324 = params.p1294;
                validate_finite_parameter("lpgisl", params.p1324).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1325 = params.p1295;
                validate_finite_parameter("npgisl", params.p1325).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1326 = params.p1296;
                validate_finite_parameter("ppgisl", params.p1326).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1327 = params.p1297;
                validate_finite_parameter("wpgisl", params.p1327).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1328 = params.p1298;
                validate_finite_parameter("p2pgisl", params.p1328).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 24] = [
                1e-27, 0.0, 0.0, 0.0, 0.0, 0.0, 6.3e-5, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.215, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.382, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (ptr as *mut f64).add(1329), 24);
            {
                let params = &mut *ptr;
                params.p1353 = params.p1329;
                validate_finite_parameter("atats", params.p1353).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (ptr as *mut f64).add(1354), 5);
            {
                let params = &mut *ptr;
                params.p1359 = params.p1335;
                validate_finite_parameter("btats", params.p1359).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (ptr as *mut f64).add(1360), 5);
            {
                let params = &mut *ptr;
                params.p1365 = params.p1341;
                validate_finite_parameter("ctats", params.p1365).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 5] = [
                0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (ptr as *mut f64).add(1366), 5);
            {
                let params = &mut *ptr;
                params.p1371 = params.p1347;
                validate_finite_parameter("dtats", params.p1371).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 35] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 6.055e-12, 0.0, 0.0,
                0.0, 0.0, 0.0, 300000000.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (ptr as *mut f64).add(1372), 35);
            {
                let params = &mut *ptr;
                params.p1407 = params.p1377;
                validate_finite_parameter("agislb", params.p1407).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1408 = params.p1378;
                validate_finite_parameter("lagislb", params.p1408).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1409 = params.p1379;
                validate_finite_parameter("nagislb", params.p1409).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1410 = params.p1380;
                validate_finite_parameter("pagislb", params.p1410).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1411 = params.p1381;
                validate_finite_parameter("wagislb", params.p1411).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1412 = params.p1382;
                validate_finite_parameter("p2agislb", params.p1412).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1413 = params.p1383;
                validate_finite_parameter("bgislb", params.p1413).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1414 = params.p1384;
                validate_finite_parameter("lbgislb", params.p1414).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1415 = params.p1385;
                validate_finite_parameter("nbgislb", params.p1415).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1416 = params.p1386;
                validate_finite_parameter("pbgislb", params.p1416).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1417 = params.p1387;
                validate_finite_parameter("wbgislb", params.p1417).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1418 = params.p1388;
                validate_finite_parameter("p2bgislb", params.p1418).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1419 = params.p1389;
                validate_finite_parameter("cgislb", params.p1419).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1420 = params.p1390;
                validate_finite_parameter("lcgislb", params.p1420).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1421 = params.p1391;
                validate_finite_parameter("ncgislb", params.p1421).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1422 = params.p1392;
                validate_finite_parameter("pcgislb", params.p1422).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1423 = params.p1393;
                validate_finite_parameter("wcgislb", params.p1423).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1424 = params.p1394;
                validate_finite_parameter("p2cgislb", params.p1424).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1425 = params.p1395;
                validate_finite_parameter("egislb", params.p1425).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1426 = params.p1396;
                validate_finite_parameter("legislb", params.p1426).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1427 = params.p1397;
                validate_finite_parameter("negislb", params.p1427).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1428 = params.p1398;
                validate_finite_parameter("pegislb", params.p1428).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1429 = params.p1399;
                validate_finite_parameter("wegislb", params.p1429).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1430 = params.p1400;
                validate_finite_parameter("p2egislb", params.p1430).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1431 = params.p1401;
                validate_finite_parameter("pgislb", params.p1431).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1432 = params.p1402;
                validate_finite_parameter("lpgislb", params.p1432).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1433 = params.p1403;
                validate_finite_parameter("npgislb", params.p1433).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1434 = params.p1404;
                validate_finite_parameter("ppgislb", params.p1434).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1435 = params.p1405;
                validate_finite_parameter("wpgislb", params.p1435).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1436 = params.p1406;
                validate_finite_parameter("p2pgislb", params.p1436).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_71: [f64; 91] = [
                0.0, 0.0, 0.0, 0.0, 0.1, 0.1, 0.1, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 10000000.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 5e-10, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_71.as_ptr(), (ptr as *mut f64).add(1437), 91);
            {
                let params = &mut *ptr;
                params.p1528 = params.p89;
                validate_parameter("eotacc", params.p1528, false, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_72: [f64; 7] = [
                0.0, 2.5e-11, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_72.as_ptr(), (ptr as *mut f64).add(1529), 7);
            {
                let params = &mut *ptr;
                params.p1536 = params.p1530;
                validate_finite_parameter("cfd", params.p1536).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1537 = params.p1531;
                validate_finite_parameter("lcfd", params.p1537).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1538 = params.p1532;
                validate_finite_parameter("ncfd", params.p1538).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1539 = params.p1533;
                validate_finite_parameter("pcfd", params.p1539).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1540 = params.p1534;
                validate_finite_parameter("wcfd", params.p1540).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1541 = params.p1535;
                validate_finite_parameter("p2cfd", params.p1541).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_73: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_73.as_ptr(), (ptr as *mut f64).add(1542), 1);
            {
                let params = &mut *ptr;
                params.p1543 = params.p1542;
                validate_parameter("cgdo", params.p1543, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_74: [f64; 9] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_74.as_ptr(), (ptr as *mut f64).add(1544), 9);
            {
                let params = &mut *ptr;
                params.p1553 = params.p1547;
                validate_finite_parameter("cgdl", params.p1553).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1554 = params.p1548;
                validate_finite_parameter("lcgdl", params.p1554).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1555 = params.p1549;
                validate_finite_parameter("ncgdl", params.p1555).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1556 = params.p1550;
                validate_finite_parameter("pcgdl", params.p1556).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1557 = params.p1551;
                validate_finite_parameter("wcgdl", params.p1557).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1558 = params.p1552;
                validate_finite_parameter("p2cgdl", params.p1558).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_75: [f64; 12] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_75.as_ptr(), (ptr as *mut f64).add(1559), 12);
            {
                let params = &mut *ptr;
                params.p1571 = params.p1565;
                validate_finite_parameter("ckappad", params.p1571).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1572 = params.p1566;
                validate_finite_parameter("lckappad", params.p1572).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1573 = params.p1567;
                validate_finite_parameter("nckappad", params.p1573).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1574 = params.p1568;
                validate_finite_parameter("pckappad", params.p1574).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1575 = params.p1569;
                validate_finite_parameter("wckappad", params.p1575).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1576 = params.p1570;
                validate_finite_parameter("p2ckappad", params.p1576).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_76: [f64; 8] = [
                0.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_76.as_ptr(), (ptr as *mut f64).add(1577), 8);
            {
                let params = &mut *ptr;
                params.p1585 = params.p1584;
                validate_parameter("cjd", params.p1585, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_77: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_77.as_ptr(), (ptr as *mut f64).add(1586), 1);
            {
                let params = &mut *ptr;
                params.p1587 = params.p1586;
                validate_parameter("cjswd", params.p1587, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_78: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_78.as_ptr(), (ptr as *mut f64).add(1588), 1);
            {
                let params = &mut *ptr;
                params.p1589 = params.p1588;
                validate_parameter("cjswgd", params.p1589, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_79: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_79.as_ptr(), (ptr as *mut f64).add(1590), 1);
            {
                let params = &mut *ptr;
                params.p1591 = params.p1590;
                validate_finite_parameter("pbd", params.p1591).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_80: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_80.as_ptr(), (ptr as *mut f64).add(1592), 1);
            {
                let params = &mut *ptr;
                params.p1593 = params.p1592;
                validate_finite_parameter("pbswd", params.p1593).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1594 = params.p1592;
                validate_finite_parameter("pbswgs", params.p1594).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1595 = params.p1594;
                validate_finite_parameter("pbswgd", params.p1595).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_81: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_81.as_ptr(), (ptr as *mut f64).add(1596), 1);
            {
                let params = &mut *ptr;
                params.p1597 = params.p1596;
                validate_parameter("mjd", params.p1597, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_82: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_82.as_ptr(), (ptr as *mut f64).add(1598), 1);
            {
                let params = &mut *ptr;
                params.p1599 = params.p1598;
                validate_parameter("mjswd", params.p1599, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1600 = params.p1598;
                validate_parameter("mjswgs", params.p1600, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1601 = params.p1600;
                validate_parameter("mjswgd", params.p1601, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_83: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_83.as_ptr(), (ptr as *mut f64).add(1602), 1);
            {
                let params = &mut *ptr;
                params.p1603 = params.p1602;
                validate_parameter("sjd", params.p1603, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_84: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_84.as_ptr(), (ptr as *mut f64).add(1604), 1);
            {
                let params = &mut *ptr;
                params.p1605 = params.p1604;
                validate_parameter("sjswd", params.p1605, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_85: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_85.as_ptr(), (ptr as *mut f64).add(1606), 1);
            {
                let params = &mut *ptr;
                params.p1607 = params.p1606;
                validate_parameter("sjswgd", params.p1607, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_86: [f64; 1] = [
                0.125,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_86.as_ptr(), (ptr as *mut f64).add(1608), 1);
            {
                let params = &mut *ptr;
                params.p1609 = params.p1608;
                validate_finite_parameter("mjd2", params.p1609).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_87: [f64; 1] = [
                0.083,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_87.as_ptr(), (ptr as *mut f64).add(1610), 1);
            {
                let params = &mut *ptr;
                params.p1611 = params.p1610;
                validate_finite_parameter("mjswd2", params.p1611).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1612 = params.p1610;
                validate_finite_parameter("mjswgs2", params.p1612).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1613 = params.p1612;
                validate_finite_parameter("mjswgd2", params.p1613).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_88: [f64; 1] = [
                0.0001,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_88.as_ptr(), (ptr as *mut f64).add(1614), 1);
            {
                let params = &mut *ptr;
                params.p1615 = params.p1614;
                validate_parameter("jsd", params.p1615, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_89: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_89.as_ptr(), (ptr as *mut f64).add(1616), 1);
            {
                let params = &mut *ptr;
                params.p1617 = params.p1616;
                validate_parameter("jswd", params.p1617, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_90: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_90.as_ptr(), (ptr as *mut f64).add(1618), 1);
            {
                let params = &mut *ptr;
                params.p1619 = params.p1618;
                validate_parameter("jswgd", params.p1619, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_91: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_91.as_ptr(), (ptr as *mut f64).add(1620), 1);
            {
                let params = &mut *ptr;
                params.p1621 = params.p1620;
                validate_parameter("njd", params.p1621, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_92: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_92.as_ptr(), (ptr as *mut f64).add(1622), 1);
            {
                let params = &mut *ptr;
                params.p1623 = params.p1622;
                validate_finite_parameter("ijthdfwd", params.p1623).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_93: [f64; 1] = [
                0.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_93.as_ptr(), (ptr as *mut f64).add(1624), 1);
            {
                let params = &mut *ptr;
                params.p1625 = params.p1624;
                validate_finite_parameter("ijthdrev", params.p1625).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_94: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_94.as_ptr(), (ptr as *mut f64).add(1626), 1);
            {
                let params = &mut *ptr;
                params.p1627 = params.p1626;
                validate_finite_parameter("bvd", params.p1627).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_95: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_95.as_ptr(), (ptr as *mut f64).add(1628), 1);
            {
                let params = &mut *ptr;
                params.p1629 = params.p1628;
                validate_finite_parameter("xjbvd", params.p1629).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_96: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_96.as_ptr(), (ptr as *mut f64).add(1630), 1);
            {
                let params = &mut *ptr;
                params.p1631 = params.p1630;
                validate_finite_parameter("jtsd", params.p1631).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_97: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_97.as_ptr(), (ptr as *mut f64).add(1632), 1);
            {
                let params = &mut *ptr;
                params.p1633 = params.p1632;
                validate_finite_parameter("jtsswd", params.p1633).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_98: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_98.as_ptr(), (ptr as *mut f64).add(1634), 1);
            {
                let params = &mut *ptr;
                params.p1635 = params.p1634;
                validate_finite_parameter("jtsswgd", params.p1635).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_99: [f64; 2] = [
                0.0, 20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_99.as_ptr(), (ptr as *mut f64).add(1636), 2);
            {
                let params = &mut *ptr;
                params.p1638 = params.p1637;
                validate_finite_parameter("njtsd", params.p1638).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_100: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_100.as_ptr(), (ptr as *mut f64).add(1639), 1);
            {
                let params = &mut *ptr;
                params.p1640 = params.p1639;
                validate_finite_parameter("njtsswd", params.p1640).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_101: [f64; 1] = [
                20.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_101.as_ptr(), (ptr as *mut f64).add(1641), 1);
            {
                let params = &mut *ptr;
                params.p1642 = params.p1641;
                validate_finite_parameter("njtsswgd", params.p1642).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_102: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_102.as_ptr(), (ptr as *mut f64).add(1643), 1);
            {
                let params = &mut *ptr;
                params.p1644 = params.p1643;
                validate_finite_parameter("vtsd", params.p1644).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_103: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_103.as_ptr(), (ptr as *mut f64).add(1645), 1);
            {
                let params = &mut *ptr;
                params.p1646 = params.p1645;
                validate_finite_parameter("vtsswd", params.p1646).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_104: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_104.as_ptr(), (ptr as *mut f64).add(1647), 1);
            {
                let params = &mut *ptr;
                params.p1648 = params.p1647;
                validate_finite_parameter("vtsswgd", params.p1648).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_105: [f64; 40] = [
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 12.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                41000000.0, 6.25e39, 3.125e24, 87500000.0, 1.0, 1.0, 0.0, 2.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_105.as_ptr(), (ptr as *mut f64).add(1649), 40);
            {
                let params = &mut *ptr;
                params.p1689 = params.p1682;
                validate_finite_parameter("noia2", params.p1689).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_106: [f64; 38] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 1.2, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.05, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.5774, 0.0, 0.3652, 0.0, 0.3953, 0.0,
                0.0, 0.0, 0.1, 27.0, 0.000702, 1108.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_106.as_ptr(), (ptr as *mut f64).add(1690), 38);
            {
                let params = &mut *ptr;
                params.p1728 = params.p1727;
                validate_finite_parameter("xtid", params.p1728).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_107: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_107.as_ptr(), (ptr as *mut f64).add(1729), 1);
            {
                let params = &mut *ptr;
                params.p1730 = params.p1729;
                validate_finite_parameter("xtsd", params.p1730).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_108: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_108.as_ptr(), (ptr as *mut f64).add(1731), 1);
            {
                let params = &mut *ptr;
                params.p1732 = params.p1731;
                validate_finite_parameter("xtsswd", params.p1732).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_109: [f64; 1] = [
                0.02,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_109.as_ptr(), (ptr as *mut f64).add(1733), 1);
            {
                let params = &mut *ptr;
                params.p1734 = params.p1733;
                validate_finite_parameter("xtsswgd", params.p1734).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_110: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_110.as_ptr(), (ptr as *mut f64).add(1735), 1);
            {
                let params = &mut *ptr;
                params.p1736 = params.p1735;
                validate_finite_parameter("tnjtsd", params.p1736).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_111: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_111.as_ptr(), (ptr as *mut f64).add(1737), 1);
            {
                let params = &mut *ptr;
                params.p1738 = params.p1737;
                validate_finite_parameter("tnjtsswd", params.p1738).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_112: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_112.as_ptr(), (ptr as *mut f64).add(1739), 1);
            {
                let params = &mut *ptr;
                params.p1740 = params.p1739;
                validate_finite_parameter("tnjtsswgd", params.p1740).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_113: [f64; 109] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.01, 0.1,
                40.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.003, 0.0, 0.0, 0.0, 0.0,
                0.0, -1e-6, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5,
                0.0, 0.0, 0.0, 0.0, 0.0, 50.0, 0.0, 1.0,
                0.001, 0.0, 0.01, 1e-5, 0.0, 1.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.5556, 3.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                3.0, 2.6, 0.0, 0.0, 0.0, 0.0, 0.0, 3.0,
                2.6, 0.0, 0.0, 0.0, 0.0, 0.0, 9.5e-9, 0.1,
                14.0, 0.0, 0.0, 0.0, 0.0, 0.0, 24.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 24.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 2.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_113.as_ptr(), (ptr as *mut f64).add(1741), 109);
            {
                let params = &mut *ptr;
                params.p1850 = params.p1827;
                validate_parameter("wssp0", params.p1850, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1851 = params.p1828;
                validate_parameter("wsspr", params.p1851, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_114: [f64; 57] = [
                8e-9, 0.139, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
                1.0, 11.2, 0.0, 0.0, 0.0, 0.0, 0.0, 8.02,
                0.0, 0.0, 0.0, 0.0, 0.0, 6.18, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                1.0, 1.8, 1.0, 0.67, 0.23, 1.1, 2.4, 2.0,
                2.0, 6.0, 2.4, 5e16, 100000.0, 0.0, 0.0, 60.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_114.as_ptr(), (ptr as *mut f64).add(1852), 57);
            {
                let params = &mut *ptr;
                params.p1909 = params.p1903;
                validate_parameter("nvsrs", params.p1909, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_115: [f64; 8] = [
                0.0, 0.0, 0.0, 0.0, 0.001, 0.001, 8.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_115.as_ptr(), (ptr as *mut f64).add(1910), 8);
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1918] = [
    ("l", 0), ("lover", 1), ("dia", 2), ("tfin", 3), ("fpitch", 4), ("nfin", 5), ("ngcon", 6), ("aseo", 7), ("adeo", 8), ("pseo", 9), ("pdeo", 10), ("asej", 11), ("adej", 12), ("psej", 13), ("pdej", 14), ("cgsp", 15),
    ("cgdp", 16), ("cdsp", 17), ("nrs", 18), ("nrd", 19), ("lrsd", 20), ("nfinnom", 21), ("dtemp", 22), ("delvtrand", 23), ("u0mult", 24), ("ids0mult", 25), ("igc0mult", 26), ("igb0mult", 27), ("covs", 28), ("lcovs", 29), ("ncovs", 30), ("pcovs", 31),
    ("wcovs", 32), ("p2covs", 33), ("covd", 34), ("lcovd", 35), ("ncovd", 36), ("pcovd", 37), ("wcovd", 38), ("p2covd", 39), ("tgaa", 40), ("tsus", 41), ("hpff", 42), ("wgaa", 43), ("dws1", 44), ("dach1", 45), ("dws2", 46), ("dach2", 47),
    ("dws3", 48), ("dach3", 49), ("dws4", 50), ("dach4", 51), ("dws5", 52), ("dach5", 53), ("dws6", 54), ("dach6", 55), ("ngaa", 56), ("subbandmod", 57), ("mobscmod", 58), ("nf", 59), ("type", 60), ("bulkmod", 61), ("geomod", 62), ("cgeo1sw", 63),
    ("rdsmod", 64), ("hvmod", 65), ("asymmod", 66), ("cvmod", 67), ("igcmod", 68), ("igbmod", 69), ("gidlmod", 70), ("iimod", 71), ("tnoimod", 72), ("nqsmod", 73), ("shmod", 74), ("tempmod", 75), ("rgatemod", 76), ("rgeomod", 77), ("cgeomod", 78), ("fnmod", 79),
    ("cryomod", 80), ("sh_warn", 81), ("igclamp", 82), ("ll", 83), ("lln", 84), ("dlc", 85), ("dlcacc", 86), ("dwcacc", 87), ("llc", 88), ("eot", 89), ("toxp", 90), ("eotbox", 91), ("hfin", 92), ("deltaw", 93), ("deltawcv", 94), ("nbodyn1", 95),
    ("nbodyn2", 96), ("nsd", 97), ("phigl", 98), ("phiglt", 99), ("phign1", 100), ("phign2", 101), ("epsrox", 102), ("epsrsub", 103), ("easub", 104), ("ni0sub", 105), ("bg0sub", 106), ("nc0sub", 107), ("imin", 108), ("xl", 109), ("lxl", 110), ("nxl", 111),
    ("pxl", 112), ("lint", 113), ("llint", 114), ("nlint", 115), ("plint", 116), ("dlbin", 117), ("ldlbin", 118), ("ndlbin", 119), ("pdlbin", 120), ("xw", 121), ("lxw", 122), ("nxw", 123), ("pxw", 124), ("wxw", 125), ("p2xw", 126), ("dwbin", 127),
    ("ldwbin", 128), ("ndwbin", 129), ("pdwbin", 130), ("wdwbin", 131), ("p2dwbin", 132), ("nbody", 133), ("lnbody", 134), ("nnbody", 135), ("pnbody", 136), ("phig", 137), ("lphig", 138), ("nphig", 139), ("pphig", 140), ("wphig", 141), ("p2phig", 142), ("vfbdriftd", 143),
    ("vfbdrifts", 144), ("ngate", 145), ("lngate", 146), ("nngate", 147), ("pngate", 148), ("wngate", 149), ("p2ngate", 150), ("minr", 151), ("cdscn1", 152), ("cdscn2", 153), ("cdscdn1", 154), ("cdscdn2", 155), ("cdscdrn1", 156), ("cdscdrn2", 157), ("eta0n1", 158), ("eta0n2", 159),
    ("eta0lt", 160), ("eta0n1cv", 161), ("eta0n2cv", 162), ("eta0ltcv", 163), ("teta0", 164), ("teta0cv", 165), ("teta0r", 166), ("advtp0", 167), ("bdvtp0", 168), ("advtp1", 169), ("bdvtp1", 170), ("dvtp2", 171), ("thetasce", 172), ("thetadibl", 173), ("thetasw", 174), ("nvtm", 175),
    ("dvtp0", 176), ("ldvtp0", 177), ("ndvtp0", 178), ("pdvtp0", 179), ("wdvtp0", 180), ("p2dvtp0", 181), ("dvtp1", 182), ("ldvtp1", 183), ("ndvtp1", 184), ("pdvtp1", 185), ("wdvtp1", 186), ("p2dvtp1", 187), ("cit", 188), ("lcit", 189), ("ncit", 190), ("pcit", 191),
    ("wcit", 192), ("p2cit", 193), ("citr", 194), ("lcitr", 195), ("ncitr", 196), ("pcitr", 197), ("wcitr", 198), ("p2citr", 199), ("cdsc", 200), ("lcdsc", 201), ("ncdsc", 202), ("pcdsc", 203), ("wcdsc", 204), ("p2cdsc", 205), ("cdscd", 206), ("lcdscd", 207),
    ("ncdscd", 208), ("pcdscd", 209), ("wcdscd", 210), ("p2cdscd", 211), ("cdscdr", 212), ("lcdscdr", 213), ("ncdscdr", 214), ("pcdscdr", 215), ("wcdscdr", 216), ("p2cdscdr", 217), ("dvt0", 218), ("ldvt0", 219), ("ndvt0", 220), ("pdvt0", 221), ("wdvt0", 222), ("p2dvt0", 223),
    ("dvt1", 224), ("ldvt1", 225), ("ndvt1", 226), ("pdvt1", 227), ("wdvt1", 228), ("p2dvt1", 229), ("dvt1ss", 230), ("ldvt1ss", 231), ("ndvt1ss", 232), ("pdvt1ss", 233), ("wdvt1ss", 234), ("p2dvt1ss", 235), ("phin", 236), ("lphin", 237), ("nphin", 238), ("pphin", 239),
    ("wphin", 240), ("p2phin", 241), ("eta0", 242), ("leta0", 243), ("neta0", 244), ("peta0", 245), ("weta0", 246), ("p2eta0", 247), ("eta1", 248), ("leta1", 249), ("neta1", 250), ("peta1", 251), ("weta1", 252), ("p2eta1", 253), ("eta0r", 254), ("leta0r", 255),
    ("neta0r", 256), ("peta0r", 257), ("weta0r", 258), ("p2eta0r", 259), ("eta0cv", 260), ("leta0cv", 261), ("neta0cv", 262), ("peta0cv", 263), ("weta0cv", 264), ("p2eta0cv", 265), ("dsub", 266), ("ldsub", 267), ("ndsub", 268), ("pdsub", 269), ("wdsub", 270), ("p2dsub", 271),
    ("k1rsce", 272), ("lk1rsce", 273), ("nk1rsce", 274), ("pk1rsce", 275), ("wk1rsce", 276), ("p2k1rsce", 277), ("lpe0", 278), ("llpe0", 279), ("nlpe0", 280), ("plpe0", 281), ("wlpe0", 282), ("p2lpe0", 283), ("dvtshift", 284), ("ldvtshift", 285), ("ndvtshift", 286), ("pdvtshift", 287),
    ("wdvtshift", 288), ("p2dvtshift", 289), ("dvtshiftr", 290), ("ldvtshiftr", 291), ("ndvtshiftr", 292), ("pdvtshiftr", 293), ("wdvtshiftr", 294), ("p2dvtshiftr", 295), ("k0", 296), ("lk0", 297), ("nk0", 298), ("pk0", 299), ("wk0", 300), ("p2k0", 301), ("k01", 302), ("lk01", 303),
    ("nk01", 304), ("pk01", 305), ("wk01", 306), ("p2k01", 307), ("k0si", 308), ("lk0si", 309), ("nk0si", 310), ("pk0si", 311), ("wk0si", 312), ("p2k0si", 313), ("k0si1", 314), ("lk0si1", 315), ("nk0si1", 316), ("pk0si1", 317), ("wk0si1", 318), ("p2k0si1", 319),
    ("k2si", 320), ("lk2si", 321), ("nk2si", 322), ("pk2si", 323), ("wk2si", 324), ("p2k2si", 325), ("k2si1", 326), ("lk2si1", 327), ("nk2si1", 328), ("pk2si1", 329), ("wk2si1", 330), ("p2k2si1", 331), ("k0sisat", 332), ("lk0sisat", 333), ("nk0sisat", 334), ("pk0sisat", 335),
    ("wk0sisat", 336), ("p2k0sisat", 337), ("k0sisat1", 338), ("lk0sisat1", 339), ("nk0sisat1", 340), ("pk0sisat1", 341), ("wk0sisat1", 342), ("p2k0sisat1", 343), ("k2sisat", 344), ("lk2sisat", 345), ("nk2sisat", 346), ("pk2sisat", 347), ("wk2sisat", 348), ("p2k2sisat", 349), ("k2sisat1", 350), ("lk2sisat1", 351),
    ("nk2sisat1", 352), ("pk2sisat1", 353), ("wk2sisat1", 354), ("p2k2sisat1", 355), ("phibe", 356), ("lphibe", 357), ("nphibe", 358), ("pphibe", 359), ("wphibe", 360), ("p2phibe", 361), ("k1", 362), ("lk1", 363), ("nk1", 364), ("pk1", 365), ("wk1", 366), ("p2k1", 367),
    ("k11", 368), ("lk11", 369), ("nk11", 370), ("pk11", 371), ("wk11", 372), ("p2k11", 373), ("k2sat", 374), ("lk2sat", 375), ("nk2sat", 376), ("pk2sat", 377), ("wk2sat", 378), ("p2k2sat", 379), ("k2sat1", 380), ("lk2sat1", 381), ("nk2sat1", 382), ("pk2sat1", 383),
    ("wk2sat1", 384), ("p2k2sat1", 385), ("k2", 386), ("lk2", 387), ("nk2", 388), ("pk2", 389), ("wk2", 390), ("p2k2", 391), ("k21", 392), ("lk21", 393), ("nk21", 394), ("pk21", 395), ("wk21", 396), ("p2k21", 397), ("aqmtcen", 398), ("bqmtcen", 399),
    ("qm0", 400), ("qm0acc", 401), ("pqmacc", 402), ("qmfactor", 403), ("lqmfactor", 404), ("nqmfactor", 405), ("pqmfactor", 406), ("wqmfactor", 407), ("p2qmfactor", 408), ("qmtcencv", 409), ("lqmtcencv", 410), ("nqmtcencv", 411), ("pqmtcencv", 412), ("wqmtcencv", 413), ("p2qmtcencv", 414), ("qmtcencva", 415),
    ("lqmtcencva", 416), ("nqmtcencva", 417), ("pqmtcencva", 418), ("wqmtcencva", 419), ("p2qmtcencva", 420), ("pqm", 421), ("lpqm", 422), ("npqm", 423), ("ppqm", 424), ("wpqm", 425), ("p2pqm", 426), ("pqml", 427), ("vsatn1", 428), ("vsatn2", 429), ("avsat", 430), ("bvsat", 431),
    ("vsat1n1", 432), ("vsat1n2", 433), ("vsat1rn1", 434), ("vsat1rn2", 435), ("avsat1", 436), ("bvsat1", 437), ("apsat", 438), ("bpsat", 439), ("avsatcv", 440), ("bvsatcv", 441), ("apsatcv", 442), ("bpsatcv", 443), ("amexp", 444), ("bmexp", 445), ("amexpr", 446), ("bmexpr", 447),
    ("aptwg", 448), ("bptwg", 449), ("tmexp", 450), ("tmexp2", 451), ("tmexpr", 452), ("dvsatclamp", 453), ("vsatdr", 454), ("vsat", 455), ("lvsat", 456), ("nvsat", 457), ("pvsat", 458), ("wvsat", 459), ("p2vsat", 460), ("vsatr", 461), ("lvsatr", 462), ("nvsatr", 463),
    ("pvsatr", 464), ("wvsatr", 465), ("p2vsatr", 466), ("vsat1", 467), ("lvsat1", 468), ("nvsat1", 469), ("pvsat1", 470), ("wvsat1", 471), ("p2vsat1", 472), ("vsat1r", 473), ("lvsat1r", 474), ("nvsat1r", 475), ("pvsat1r", 476), ("wvsat1r", 477), ("p2vsat1r", 478), ("deltavsat", 479),
    ("ldeltavsat", 480), ("ndeltavsat", 481), ("pdeltavsat", 482), ("wdeltavsat", 483), ("p2deltavsat", 484), ("psat", 485), ("lpsat", 486), ("npsat", 487), ("ppsat", 488), ("wpsat", 489), ("p2psat", 490), ("ksativdr", 491), ("ksativ", 492), ("lksativ", 493), ("nksativ", 494), ("pksativ", 495),
    ("wksativ", 496), ("p2ksativ", 497), ("ksativt1", 498), ("ksativt2", 499), ("ksativr", 500), ("lksativr", 501), ("nksativr", 502), ("pksativr", 503), ("wksativr", 504), ("p2ksativr", 505), ("vsatcv", 506), ("lvsatcv", 507), ("nvsatcv", 508), ("pvsatcv", 509), ("wvsatcv", 510), ("p2vsatcv", 511),
    ("asat", 512), ("lasat", 513), ("nasat", 514), ("pasat", 515), ("wasat", 516), ("p2asat", 517), ("deltavsatcv", 518), ("ldeltavsatcv", 519), ("ndeltavsatcv", 520), ("pdeltavsatcv", 521), ("wdeltavsatcv", 522), ("p2deltavsatcv", 523), ("psatcv", 524), ("lpsatcv", 525), ("npsatcv", 526), ("ppsatcv", 527),
    ("wpsatcv", 528), ("p2psatcv", 529), ("mexpdr", 530), ("mexp", 531), ("lmexp", 532), ("nmexp", 533), ("pmexp", 534), ("wmexp", 535), ("p2mexp", 536), ("mexpr", 537), ("lmexpr", 538), ("nmexpr", 539), ("pmexpr", 540), ("wmexpr", 541), ("p2mexpr", 542), ("ptwg", 543),
    ("lptwg", 544), ("nptwg", 545), ("pptwg", 546), ("wptwg", 547), ("p2ptwg", 548), ("ptwgr", 549), ("lptwgr", 550), ("nptwgr", 551), ("pptwgr", 552), ("wptwgr", 553), ("p2ptwgr", 554), ("at", 555), ("lat", 556), ("nat", 557), ("pat", 558), ("wat", 559),
    ("p2at", 560), ("at2", 561), ("atr", 562), ("latr", 563), ("natr", 564), ("patr", 565), ("watr", 566), ("p2atr", 567), ("atcv", 568), ("latcv", 569), ("natcv", 570), ("patcv", 571), ("watcv", 572), ("p2atcv", 573), ("at2cv", 574), ("ptwgt", 575),
    ("lptwgt", 576), ("nptwgt", 577), ("pptwgt", 578), ("wptwgt", 579), ("p2ptwgt", 580), ("u0n1", 581), ("u0n1cv", 582), ("u0n1r", 583), ("u0n2", 584), ("u0n2cv", 585), ("u0n2r", 586), ("u0lt", 587), ("u0ltcv", 588), ("lpa", 589), ("lpar", 590), ("aua", 591),
    ("auar", 592), ("bua", 593), ("buar", 594), ("aeu", 595), ("aeur", 596), ("beu", 597), ("beur", 598), ("aud", 599), ("audr", 600), ("bud", 601), ("budr", 602), ("chargewf", 603), ("dmobclamp", 604), ("u0", 605), ("lu0", 606), ("nu0", 607),
    ("pu0", 608), ("wu0", 609), ("p2u0", 610), ("u0r", 611), ("lu0r", 612), ("nu0r", 613), ("pu0r", 614), ("wu0r", 615), ("p2u0r", 616), ("u0cv", 617), ("lu0cv", 618), ("nu0cv", 619), ("pu0cv", 620), ("wu0cv", 621), ("p2u0cv", 622), ("etamob", 623),
    ("letamob", 624), ("netamob", 625), ("petamob", 626), ("wetamob", 627), ("p2etamob", 628), ("up", 629), ("lup", 630), ("nup", 631), ("pup", 632), ("wup", 633), ("p2up", 634), ("upr", 635), ("lupr", 636), ("nupr", 637), ("pupr", 638), ("wupr", 639),
    ("p2upr", 640), ("ua", 641), ("lua", 642), ("nua", 643), ("pua", 644), ("wua", 645), ("p2ua", 646), ("uar", 647), ("luar", 648), ("nuar", 649), ("puar", 650), ("wuar", 651), ("p2uar", 652), ("uacv", 653), ("luacv", 654), ("nuacv", 655),
    ("puacv", 656), ("wuacv", 657), ("p2uacv", 658), ("uc", 659), ("luc", 660), ("nuc", 661), ("puc", 662), ("wuc", 663), ("p2uc", 664), ("ucr", 665), ("lucr", 666), ("nucr", 667), ("pucr", 668), ("wucr", 669), ("p2ucr", 670), ("uccv", 671),
    ("luccv", 672), ("nuccv", 673), ("puccv", 674), ("wuccv", 675), ("p2uccv", 676), ("eu", 677), ("leu", 678), ("neu", 679), ("peu", 680), ("weu", 681), ("p2eu", 682), ("eur", 683), ("leur", 684), ("neur", 685), ("peur", 686), ("weur", 687),
    ("p2eur", 688), ("ud", 689), ("lud", 690), ("nud", 691), ("pud", 692), ("wud", 693), ("p2ud", 694), ("udr", 695), ("ludr", 696), ("nudr", 697), ("pudr", 698), ("wudr", 699), ("p2udr", 700), ("udcv", 701), ("ludcv", 702), ("nudcv", 703),
    ("pudcv", 704), ("wudcv", 705), ("p2udcv", 706), ("ucs", 707), ("lucs", 708), ("nucs", 709), ("pucs", 710), ("wucs", 711), ("p2ucs", 712), ("uds", 713), ("luds", 714), ("nuds", 715), ("puds", 716), ("wuds", 717), ("p2uds", 718), ("uds1", 719),
    ("luds1", 720), ("nuds1", 721), ("puds1", 722), ("wuds1", 723), ("p2uds1", 724), ("udd", 725), ("ludd", 726), ("nudd", 727), ("pudd", 728), ("wudd", 729), ("p2udd", 730), ("udd1", 731), ("ludd1", 732), ("nudd1", 733), ("pudd1", 734), ("wudd1", 735),
    ("p2udd1", 736), ("ute", 737), ("lute", 738), ("nute", 739), ("pute", 740), ("wute", 741), ("p2ute", 742), ("uter", 743), ("luter", 744), ("nuter", 745), ("puter", 746), ("wuter", 747), ("p2uter", 748), ("utecv", 749), ("lutecv", 750), ("nutecv", 751),
    ("putecv", 752), ("wutecv", 753), ("p2utecv", 754), ("ute1", 755), ("lute1", 756), ("nute1", 757), ("pute1", 758), ("wute1", 759), ("p2ute1", 760), ("ute1cv", 761), ("lute1cv", 762), ("nute1cv", 763), ("pute1cv", 764), ("wute1cv", 765), ("p2ute1cv", 766), ("utl", 767),
    ("lutl", 768), ("nutl", 769), ("putl", 770), ("wutl", 771), ("p2utl", 772), ("utlr", 773), ("lutlr", 774), ("nutlr", 775), ("putlr", 776), ("wutlr", 777), ("p2utlr", 778), ("utlcv", 779), ("lutlcv", 780), ("nutlcv", 781), ("putlcv", 782), ("wutlcv", 783),
    ("p2utlcv", 784), ("emobt", 785), ("lemobt", 786), ("nemobt", 787), ("pemobt", 788), ("wemobt", 789), ("p2emobt", 790), ("ua1", 791), ("lua1", 792), ("nua1", 793), ("pua1", 794), ("wua1", 795), ("p2ua1", 796), ("ua1r", 797), ("lua1r", 798), ("nua1r", 799),
    ("pua1r", 800), ("wua1r", 801), ("p2ua1r", 802), ("ua1cv", 803), ("lua1cv", 804), ("nua1cv", 805), ("pua1cv", 806), ("wua1cv", 807), ("p2ua1cv", 808), ("ua2", 809), ("lua2", 810), ("nua2", 811), ("pua2", 812), ("wua2", 813), ("p2ua2", 814), ("ua2cv", 815),
    ("lua2cv", 816), ("nua2cv", 817), ("pua2cv", 818), ("wua2cv", 819), ("p2ua2cv", 820), ("eu1", 821), ("leu1", 822), ("neu1", 823), ("peu1", 824), ("weu1", 825), ("p2eu1", 826), ("uc1", 827), ("luc1", 828), ("nuc1", 829), ("puc1", 830), ("wuc1", 831),
    ("p2uc1", 832), ("uc1r", 833), ("luc1r", 834), ("nuc1r", 835), ("puc1r", 836), ("wuc1r", 837), ("p2uc1r", 838), ("uc1cv", 839), ("luc1cv", 840), ("nuc1cv", 841), ("puc1cv", 842), ("wuc1cv", 843), ("p2uc1cv", 844), ("ud1", 845), ("lud1", 846), ("nud1", 847),
    ("pud1", 848), ("wud1", 849), ("p2ud1", 850), ("ud1r", 851), ("lud1r", 852), ("nud1r", 853), ("pud1r", 854), ("wud1r", 855), ("p2ud1r", 856), ("ud1cv", 857), ("lud1cv", 858), ("nud1cv", 859), ("pud1cv", 860), ("wud1cv", 861), ("p2ud1cv", 862), ("ud2", 863),
    ("lud2", 864), ("nud2", 865), ("pud2", 866), ("wud2", 867), ("p2ud2", 868), ("ud2cv", 869), ("lud2cv", 870), ("nud2cv", 871), ("pud2cv", 872), ("wud2cv", 873), ("p2ud2cv", 874), ("ucste", 875), ("lucste", 876), ("nucste", 877), ("pucste", 878), ("wucste", 879),
    ("p2ucste", 880), ("ucste1", 881), ("lucste1", 882), ("nucste1", 883), ("pucste1", 884), ("wucste1", 885), ("p2ucste1", 886), ("muhc0", 887), ("muhc1", 888), ("etamobthin", 889), ("etamobtni", 890), ("etamobir", 891), ("uathin", 892), ("uatsat", 893), ("uartsc", 894), ("uatni", 895),
    ("uair", 896), ("euthin", 897), ("euptsc", 898), ("eutni", 899), ("euir", 900), ("udthin", 901), ("udtsat", 902), ("udptsc", 903), ("u0etawsc", 904), ("egbulk", 905), ("u0emsm1", 906), ("u0emsm2", 907), ("rdswmin", 908), ("ardsw", 909), ("brdsw", 910), ("rswmin", 911),
    ("arsw", 912), ("brsw", 913), ("rdwmin", 914), ("ardw", 915), ("brdw", 916), ("rsdr", 917), ("rsdrr", 918), ("rddr", 919), ("rddrr", 920), ("prsdr", 921), ("prddr", 922), ("trsdr", 923), ("trddr", 924), ("rdsw", 925), ("lrdsw", 926), ("nrdsw", 927),
    ("prdsw", 928), ("wrdsw", 929), ("p2rdsw", 930), ("rsw", 931), ("lrsw", 932), ("nrsw", 933), ("prsw", 934), ("wrsw", 935), ("p2rsw", 936), ("rdw", 937), ("lrdw", 938), ("nrdw", 939), ("prdw", 940), ("wrdw", 941), ("p2rdw", 942), ("prwgs", 943),
    ("lprwgs", 944), ("nprwgs", 945), ("pprwgs", 946), ("wprwgs", 947), ("p2prwgs", 948), ("prwgd", 949), ("lprwgd", 950), ("nprwgd", 951), ("pprwgd", 952), ("wprwgd", 953), ("p2prwgd", 954), ("wr", 955), ("lwr", 956), ("nwr", 957), ("pwr", 958), ("wwr", 959),
    ("p2wr", 960), ("prt", 961), ("lprt", 962), ("nprt", 963), ("pprt", 964), ("wprt", 965), ("p2prt", 966), ("prt1", 967), ("lprt1", 968), ("nprt1", 969), ("pprt1", 970), ("wprt1", 971), ("p2prt1", 972), ("tr0", 973), ("ltr0", 974), ("ntr0", 975),
    ("ptr0", 976), ("wtr0", 977), ("p2tr0", 978), ("sprt", 979), ("lsprt", 980), ("nsprt", 981), ("psprt", 982), ("wsprt", 983), ("p2sprt", 984), ("pdibl1", 985), ("lpdibl1", 986), ("npdibl1", 987), ("ppdibl1", 988), ("wpdibl1", 989), ("p2pdibl1", 990), ("pdibl2", 991),
    ("lpdibl2", 992), ("npdibl2", 993), ("ppdibl2", 994), ("wpdibl2", 995), ("p2pdibl2", 996), ("pdibl1r", 997), ("lpdibl1r", 998), ("npdibl1r", 999), ("ppdibl1r", 1000), ("wpdibl1r", 1001), ("p2pdibl1r", 1002), ("pdibl2r", 1003), ("lpdibl2r", 1004), ("npdibl2r", 1005), ("ppdibl2r", 1006), ("wpdibl2r", 1007),
    ("p2pdibl2r", 1008), ("drout", 1009), ("ldrout", 1010), ("ndrout", 1011), ("pdrout", 1012), ("wdrout", 1013), ("p2drout", 1014), ("pvag", 1015), ("lpvag", 1016), ("npvag", 1017), ("ppvag", 1018), ("wpvag", 1019), ("p2pvag", 1020), ("apclm", 1021), ("apclmr", 1022), ("bpclm", 1023),
    ("bpclmr", 1024), ("pclm", 1025), ("pclmt", 1026), ("lpclm", 1027), ("npclm", 1028), ("ppclm", 1029), ("wpclm", 1030), ("p2pclm", 1031), ("pclmr", 1032), ("lpclmr", 1033), ("npclmr", 1034), ("ppclmr", 1035), ("wpclmr", 1036), ("p2pclmr", 1037), ("pclmg", 1038), ("lpclmg", 1039),
    ("npclmg", 1040), ("ppclmg", 1041), ("wpclmg", 1042), ("p2pclmg", 1043), ("pclmcv", 1044), ("lpclmcv", 1045), ("npclmcv", 1046), ("ppclmcv", 1047), ("wpclmcv", 1048), ("p2pclmcv", 1049), ("a1", 1050), ("la1", 1051), ("na1", 1052), ("pa1", 1053), ("wa1", 1054), ("p2a1", 1055),
    ("a11", 1056), ("la11", 1057), ("na11", 1058), ("pa11", 1059), ("wa11", 1060), ("p2a11", 1061), ("a2", 1062), ("la2", 1063), ("na2", 1064), ("pa2", 1065), ("wa2", 1066), ("p2a2", 1067), ("a21", 1068), ("la21", 1069), ("na21", 1070), ("pa21", 1071),
    ("wa21", 1072), ("p2a21", 1073), ("rgext", 1074), ("rgfin", 1075), ("rgint", 1076), ("rgp", 1077), ("rshs", 1078), ("rshd", 1079), ("hepi", 1080), ("tsili", 1081), ("rhoc", 1082), ("rhorsd", 1083), ("cratio", 1084), ("deltaprsd", 1085), ("sdterm", 1086), ("lsp", 1087),
    ("epsrsp", 1088), ("tgate", 1089), ("tmask", 1090), ("asiliend", 1091), ("arsdend", 1092), ("prsdend", 1093), ("rgeoa", 1094), ("rgeob", 1095), ("rgeoc", 1096), ("rgeod", 1097), ("rgeoe", 1098), ("cgeoa", 1099), ("cgeob", 1100), ("cgeoc", 1101), ("cgeod", 1102), ("cgeoe", 1103),
    ("dlcigs", 1104), ("dlcigd", 1105), ("vfbsd", 1106), ("vfbsdcv", 1107), ("toxref", 1108), ("toxg", 1109), ("igbinvclamp", 1110), ("igbaccclamp", 1111), ("igcinvclamp", 1112), ("ntox", 1113), ("lntox", 1114), ("nntox", 1115), ("pntox", 1116), ("wntox", 1117), ("p2ntox", 1118), ("aigbinv", 1119),
    ("laigbinv", 1120), ("naigbinv", 1121), ("paigbinv", 1122), ("waigbinv", 1123), ("p2aigbinv", 1124), ("aigbinv1", 1125), ("laigbinv1", 1126), ("naigbinv1", 1127), ("paigbinv1", 1128), ("waigbinv1", 1129), ("p2aigbinv1", 1130), ("bigbinv", 1131), ("lbigbinv", 1132), ("nbigbinv", 1133), ("pbigbinv", 1134), ("wbigbinv", 1135),
    ("p2bigbinv", 1136), ("cigbinv", 1137), ("lcigbinv", 1138), ("ncigbinv", 1139), ("pcigbinv", 1140), ("wcigbinv", 1141), ("p2cigbinv", 1142), ("eigbinv", 1143), ("leigbinv", 1144), ("neigbinv", 1145), ("peigbinv", 1146), ("weigbinv", 1147), ("p2eigbinv", 1148), ("nigbinv", 1149), ("lnigbinv", 1150), ("nnigbinv", 1151),
    ("pnigbinv", 1152), ("wnigbinv", 1153), ("p2nigbinv", 1154), ("aigbacc", 1155), ("laigbacc", 1156), ("naigbacc", 1157), ("paigbacc", 1158), ("waigbacc", 1159), ("p2aigbacc", 1160), ("aigbacc1", 1161), ("laigbacc1", 1162), ("naigbacc1", 1163), ("paigbacc1", 1164), ("waigbacc1", 1165), ("p2aigbacc1", 1166), ("bigbacc", 1167),
    ("lbigbacc", 1168), ("nbigbacc", 1169), ("pbigbacc", 1170), ("wbigbacc", 1171), ("p2bigbacc", 1172), ("cigbacc", 1173), ("lcigbacc", 1174), ("ncigbacc", 1175), ("pcigbacc", 1176), ("wcigbacc", 1177), ("p2cigbacc", 1178), ("nigbacc", 1179), ("lnigbacc", 1180), ("nnigbacc", 1181), ("pnigbacc", 1182), ("wnigbacc", 1183),
    ("p2nigbacc", 1184), ("aigc", 1185), ("laigc", 1186), ("naigc", 1187), ("paigc", 1188), ("waigc", 1189), ("p2aigc", 1190), ("aigc1", 1191), ("laigc1", 1192), ("naigc1", 1193), ("paigc1", 1194), ("waigc1", 1195), ("p2aigc1", 1196), ("bigc", 1197), ("lbigc", 1198), ("nbigc", 1199),
    ("pbigc", 1200), ("wbigc", 1201), ("p2bigc", 1202), ("cigc", 1203), ("lcigc", 1204), ("ncigc", 1205), ("pcigc", 1206), ("wcigc", 1207), ("p2cigc", 1208), ("pigcd", 1209), ("lpigcd", 1210), ("npigcd", 1211), ("ppigcd", 1212), ("wpigcd", 1213), ("p2pigcd", 1214), ("aigs", 1215),
    ("laigs", 1216), ("naigs", 1217), ("paigs", 1218), ("waigs", 1219), ("p2aigs", 1220), ("aigs1", 1221), ("laigs1", 1222), ("naigs1", 1223), ("paigs1", 1224), ("waigs1", 1225), ("p2aigs1", 1226), ("bigs", 1227), ("lbigs", 1228), ("nbigs", 1229), ("pbigs", 1230), ("wbigs", 1231),
    ("p2bigs", 1232), ("cigs", 1233), ("lcigs", 1234), ("ncigs", 1235), ("pcigs", 1236), ("wcigs", 1237), ("p2cigs", 1238), ("aigd", 1239), ("laigd", 1240), ("naigd", 1241), ("paigd", 1242), ("waigd", 1243), ("p2aigd", 1244), ("aigd1", 1245), ("laigd1", 1246), ("naigd1", 1247),
    ("paigd1", 1248), ("waigd1", 1249), ("p2aigd1", 1250), ("bigd", 1251), ("lbigd", 1252), ("nbigd", 1253), ("pbigd", 1254), ("wbigd", 1255), ("p2bigd", 1256), ("cigd", 1257), ("lcigd", 1258), ("ncigd", 1259), ("pcigd", 1260), ("wcigd", 1261), ("p2cigd", 1262), ("poxedge", 1263),
    ("lpoxedge", 1264), ("npoxedge", 1265), ("ppoxedge", 1266), ("wpoxedge", 1267), ("p2poxedge", 1268), ("agidl", 1269), ("lagidl", 1270), ("nagidl", 1271), ("pagidl", 1272), ("wagidl", 1273), ("p2agidl", 1274), ("bgidl", 1275), ("lbgidl", 1276), ("nbgidl", 1277), ("pbgidl", 1278), ("wbgidl", 1279),
    ("p2bgidl", 1280), ("cgidl", 1281), ("lcgidl", 1282), ("ncgidl", 1283), ("pcgidl", 1284), ("wcgidl", 1285), ("p2cgidl", 1286), ("egidl", 1287), ("legidl", 1288), ("negidl", 1289), ("pegidl", 1290), ("wegidl", 1291), ("p2egidl", 1292), ("pgidl", 1293), ("lpgidl", 1294), ("npgidl", 1295),
    ("ppgidl", 1296), ("wpgidl", 1297), ("p2pgidl", 1298), ("agisl", 1299), ("lagisl", 1300), ("nagisl", 1301), ("pagisl", 1302), ("wagisl", 1303), ("p2agisl", 1304), ("bgisl", 1305), ("lbgisl", 1306), ("nbgisl", 1307), ("pbgisl", 1308), ("wbgisl", 1309), ("p2bgisl", 1310), ("cgisl", 1311),
    ("lcgisl", 1312), ("ncgisl", 1313), ("pcgisl", 1314), ("wcgisl", 1315), ("p2cgisl", 1316), ("egisl", 1317), ("legisl", 1318), ("negisl", 1319), ("pegisl", 1320), ("wegisl", 1321), ("p2egisl", 1322), ("pgisl", 1323), ("lpgisl", 1324), ("npgisl", 1325), ("ppgisl", 1326), ("wpgisl", 1327),
    ("p2pgisl", 1328), ("atatd", 1329), ("latatd", 1330), ("natatd", 1331), ("patatd", 1332), ("watatd", 1333), ("p2atatd", 1334), ("btatd", 1335), ("lbtatd", 1336), ("nbtatd", 1337), ("pbtatd", 1338), ("wbtatd", 1339), ("p2btatd", 1340), ("ctatd", 1341), ("lctatd", 1342), ("nctatd", 1343),
    ("pctatd", 1344), ("wctatd", 1345), ("p2ctatd", 1346), ("dtatd", 1347), ("ldtatd", 1348), ("ndtatd", 1349), ("pdtatd", 1350), ("wdtatd", 1351), ("p2dtatd", 1352), ("atats", 1353), ("latats", 1354), ("natats", 1355), ("patats", 1356), ("watats", 1357), ("p2atats", 1358), ("btats", 1359),
    ("lbtats", 1360), ("nbtats", 1361), ("pbtats", 1362), ("wbtats", 1363), ("p2btats", 1364), ("ctats", 1365), ("lctats", 1366), ("nctats", 1367), ("pctats", 1368), ("wctats", 1369), ("p2ctats", 1370), ("dtats", 1371), ("ldtats", 1372), ("ndtats", 1373), ("pdtats", 1374), ("wdtats", 1375),
    ("p2dtats", 1376), ("agidlb", 1377), ("lagidlb", 1378), ("nagidlb", 1379), ("pagidlb", 1380), ("wagidlb", 1381), ("p2agidlb", 1382), ("bgidlb", 1383), ("lbgidlb", 1384), ("nbgidlb", 1385), ("pbgidlb", 1386), ("wbgidlb", 1387), ("p2bgidlb", 1388), ("cgidlb", 1389), ("lcgidlb", 1390), ("ncgidlb", 1391),
    ("pcgidlb", 1392), ("wcgidlb", 1393), ("p2cgidlb", 1394), ("egidlb", 1395), ("legidlb", 1396), ("negidlb", 1397), ("pegidlb", 1398), ("wegidlb", 1399), ("p2egidlb", 1400), ("pgidlb", 1401), ("lpgidlb", 1402), ("npgidlb", 1403), ("ppgidlb", 1404), ("wpgidlb", 1405), ("p2pgidlb", 1406), ("agislb", 1407),
    ("lagislb", 1408), ("nagislb", 1409), ("pagislb", 1410), ("wagislb", 1411), ("p2agislb", 1412), ("bgislb", 1413), ("lbgislb", 1414), ("nbgislb", 1415), ("pbgislb", 1416), ("wbgislb", 1417), ("p2bgislb", 1418), ("cgislb", 1419), ("lcgislb", 1420), ("ncgislb", 1421), ("pcgislb", 1422), ("wcgislb", 1423),
    ("p2cgislb", 1424), ("egislb", 1425), ("legislb", 1426), ("negislb", 1427), ("pegislb", 1428), ("wegislb", 1429), ("p2egislb", 1430), ("pgislb", 1431), ("lpgislb", 1432), ("npgislb", 1433), ("ppgislb", 1434), ("wpgislb", 1435), ("p2pgislb", 1436), ("alpha01", 1437), ("alpha11", 1438), ("alphaii01", 1439),
    ("alphaii11", 1440), ("iimod2clamp1", 1441), ("iimod2clamp2", 1442), ("iimod2clamp3", 1443), ("alpha0", 1444), ("lalpha0", 1445), ("nalpha0", 1446), ("palpha0", 1447), ("walpha0", 1448), ("p2alpha0", 1449), ("alpha1", 1450), ("lalpha1", 1451), ("nalpha1", 1452), ("palpha1", 1453), ("walpha1", 1454), ("p2alpha1", 1455),
    ("beta0", 1456), ("lbeta0", 1457), ("nbeta0", 1458), ("pbeta0", 1459), ("wbeta0", 1460), ("p2beta0", 1461), ("alphaii0", 1462), ("lalphaii0", 1463), ("nalphaii0", 1464), ("palphaii0", 1465), ("walphaii0", 1466), ("p2alphaii0", 1467), ("alphaii1", 1468), ("lalphaii1", 1469), ("nalphaii1", 1470), ("palphaii1", 1471),
    ("walphaii1", 1472), ("p2alphaii1", 1473), ("betaii0", 1474), ("lbetaii0", 1475), ("nbetaii0", 1476), ("pbetaii0", 1477), ("wbetaii0", 1478), ("p2betaii0", 1479), ("betaii1", 1480), ("lbetaii1", 1481), ("nbetaii1", 1482), ("pbetaii1", 1483), ("wbetaii1", 1484), ("p2betaii1", 1485), ("betaii2", 1486), ("lbetaii2", 1487),
    ("nbetaii2", 1488), ("pbetaii2", 1489), ("wbetaii2", 1490), ("p2betaii2", 1491), ("esatii", 1492), ("lesatii", 1493), ("nesatii", 1494), ("pesatii", 1495), ("wesatii", 1496), ("p2esatii", 1497), ("lii", 1498), ("llii", 1499), ("nlii", 1500), ("plii", 1501), ("wlii", 1502), ("p2lii", 1503),
    ("sii0", 1504), ("lsii0", 1505), ("nsii0", 1506), ("psii0", 1507), ("wsii0", 1508), ("p2sii0", 1509), ("sii1", 1510), ("lsii1", 1511), ("nsii1", 1512), ("psii1", 1513), ("wsii1", 1514), ("p2sii1", 1515), ("sii2", 1516), ("lsii2", 1517), ("nsii2", 1518), ("psii2", 1519),
    ("wsii2", 1520), ("p2sii2", 1521), ("siid", 1522), ("lsiid", 1523), ("nsiid", 1524), ("psiid", 1525), ("wsiid", 1526), ("p2siid", 1527), ("eotacc", 1528), ("delvfbacc", 1529), ("cfs", 1530), ("lcfs", 1531), ("ncfs", 1532), ("pcfs", 1533), ("wcfs", 1534), ("p2cfs", 1535),
    ("cfd", 1536), ("lcfd", 1537), ("ncfd", 1538), ("pcfd", 1539), ("wcfd", 1540), ("p2cfd", 1541), ("cgso", 1542), ("cgdo", 1543), ("cgbo", 1544), ("cgbn", 1545), ("cgbw", 1546), ("cgsl", 1547), ("lcgsl", 1548), ("ncgsl", 1549), ("pcgsl", 1550), ("wcgsl", 1551),
    ("p2cgsl", 1552), ("cgdl", 1553), ("lcgdl", 1554), ("ncgdl", 1555), ("pcgdl", 1556), ("wcgdl", 1557), ("p2cgdl", 1558), ("cgbl", 1559), ("lcgbl", 1560), ("ncgbl", 1561), ("pcgbl", 1562), ("wcgbl", 1563), ("p2cgbl", 1564), ("ckappas", 1565), ("lckappas", 1566), ("nckappas", 1567),
    ("pckappas", 1568), ("wckappas", 1569), ("p2ckappas", 1570), ("ckappad", 1571), ("lckappad", 1572), ("nckappad", 1573), ("pckappad", 1574), ("wckappad", 1575), ("p2ckappad", 1576), ("ckappab", 1577), ("lckappab", 1578), ("nckappab", 1579), ("pckappab", 1580), ("wckappab", 1581), ("p2ckappab", 1582), ("csdesw", 1583),
    ("cjs", 1584), ("cjd", 1585), ("cjsws", 1586), ("cjswd", 1587), ("cjswgs", 1588), ("cjswgd", 1589), ("pbs", 1590), ("pbd", 1591), ("pbsws", 1592), ("pbswd", 1593), ("pbswgs", 1594), ("pbswgd", 1595), ("mjs", 1596), ("mjd", 1597), ("mjsws", 1598), ("mjswd", 1599),
    ("mjswgs", 1600), ("mjswgd", 1601), ("sjs", 1602), ("sjd", 1603), ("sjsws", 1604), ("sjswd", 1605), ("sjswgs", 1606), ("sjswgd", 1607), ("mjs2", 1608), ("mjd2", 1609), ("mjsws2", 1610), ("mjswd2", 1611), ("mjswgs2", 1612), ("mjswgd2", 1613), ("jss", 1614), ("jsd", 1615),
    ("jsws", 1616), ("jswd", 1617), ("jswgs", 1618), ("jswgd", 1619), ("njs", 1620), ("njd", 1621), ("ijthsfwd", 1622), ("ijthdfwd", 1623), ("ijthsrev", 1624), ("ijthdrev", 1625), ("bvs", 1626), ("bvd", 1627), ("xjbvs", 1628), ("xjbvd", 1629), ("jtss", 1630), ("jtsd", 1631),
    ("jtssws", 1632), ("jtsswd", 1633), ("jtsswgs", 1634), ("jtsswgd", 1635), ("jtweff", 1636), ("njts", 1637), ("njtsd", 1638), ("njtssw", 1639), ("njtsswd", 1640), ("njtsswg", 1641), ("njtsswgd", 1642), ("vtss", 1643), ("vtsd", 1644), ("vtssws", 1645), ("vtsswd", 1646), ("vtsswgs", 1647),
    ("vtsswgd", 1648), ("lintigen", 1649), ("ntgen", 1650), ("lntgen", 1651), ("nntgen", 1652), ("pntgen", 1653), ("wntgen", 1654), ("p2ntgen", 1655), ("aigen", 1656), ("laigen", 1657), ("naigen", 1658), ("paigen", 1659), ("waigen", 1660), ("p2aigen", 1661), ("bigen", 1662), ("lbigen", 1663),
    ("nbigen", 1664), ("pbigen", 1665), ("wbigen", 1666), ("p2bigen", 1667), ("xrcrg1", 1668), ("lxrcrg1", 1669), ("nxrcrg1", 1670), ("pxrcrg1", 1671), ("wxrcrg1", 1672), ("p2xrcrg1", 1673), ("xrcrg2", 1674), ("lxrcrg2", 1675), ("nxrcrg2", 1676), ("pxrcrg2", 1677), ("wxrcrg2", 1678), ("p2xrcrg2", 1679),
    ("ef", 1680), ("em", 1681), ("noia", 1682), ("noib", 1683), ("noic", 1684), ("k0noi", 1685), ("k1noi", 1686), ("lintnoi", 1687), ("smooth", 1688), ("noia2", 1689), ("lnoia2", 1690), ("nnoia2", 1691), ("pnoia2", 1692), ("wnoia2", 1693), ("p2noia2", 1694), ("mpower", 1695),
    ("lmpower", 1696), ("nmpower", 1697), ("pmpower", 1698), ("wmpower", 1699), ("p2mpower", 1700), ("qsref", 1701), ("lqsref", 1702), ("nqsref", 1703), ("pqsref", 1704), ("wqsref", 1705), ("p2qsref", 1706), ("ntnoi", 1707), ("rnoia", 1708), ("tnoia", 1709), ("rnoib", 1710), ("tnoib", 1711),
    ("rnoic", 1712), ("tnoic", 1713), ("rnoik", 1714), ("tnoik", 1715), ("tnoik2", 1716), ("tnom", 1717), ("tbgasub", 1718), ("tbgbsub", 1719), ("kt1l", 1720), ("tcj", 1721), ("tcjsw", 1722), ("tcjswg", 1723), ("tpb", 1724), ("tpbsw", 1725), ("tpbswg", 1726), ("xtis", 1727),
    ("xtid", 1728), ("xtss", 1729), ("xtsd", 1730), ("xtssws", 1731), ("xtsswd", 1732), ("xtsswgs", 1733), ("xtsswgd", 1734), ("tnjts", 1735), ("tnjtsd", 1736), ("tnjtssw", 1737), ("tnjtsswd", 1738), ("tnjtsswg", 1739), ("tnjtsswgd", 1740), ("kt1", 1741), ("lkt1", 1742), ("nkt1", 1743),
    ("pkt1", 1744), ("wkt1", 1745), ("p2kt1", 1746), ("kt11", 1747), ("kt12", 1748), ("tvth", 1749), ("tss", 1750), ("ltss", 1751), ("ntss", 1752), ("ptss", 1753), ("wtss", 1754), ("p2tss", 1755), ("iit", 1756), ("liit", 1757), ("niit", 1758), ("piit", 1759),
    ("wiit", 1760), ("p2iit", 1761), ("tii", 1762), ("ltii", 1763), ("ntii", 1764), ("ptii", 1765), ("wtii", 1766), ("p2tii", 1767), ("tgidl", 1768), ("ltgidl", 1769), ("ntgidl", 1770), ("ptgidl", 1771), ("wtgidl", 1772), ("p2tgidl", 1773), ("ttat", 1774), ("lttat", 1775),
    ("nttat", 1776), ("pttat", 1777), ("wttat", 1778), ("p2ttat", 1779), ("igt", 1780), ("ligt", 1781), ("nigt", 1782), ("pigt", 1783), ("wigt", 1784), ("p2igt", 1785), ("tlow", 1786), ("tlow1", 1787), ("dtlow", 1788), ("dtlow1", 1789), ("klow1", 1790), ("rth0", 1791),
    ("cth0", 1792), ("wth0", 1793), ("ashexp", 1794), ("bshexp", 1795), ("cshexp", 1796), ("ash", 1797), ("csh", 1798), ("ach_ufcm", 1799), ("cins_ufcm", 1800), ("w_ufcm", 1801), ("tfin_top", 1802), ("tfin_base", 1803), ("qmfactorcv", 1804), ("alpha_ufcm", 1805), ("dim1h", 1806), ("dimension1", 1807),
    ("ldimension1", 1808), ("ndimension1", 1809), ("pdimension1", 1810), ("wdimension1", 1811), ("p2dimension1", 1812), ("dim2h", 1813), ("dimension2", 1814), ("ldimension2", 1815), ("ndimension2", 1816), ("pdimension2", 1817), ("wdimension2", 1818), ("p2dimension2", 1819), ("dim3h", 1820), ("dimension3", 1821), ("ldimension3", 1822), ("ndimension3", 1823),
    ("pdimension3", 1824), ("wdimension3", 1825), ("p2dimension3", 1826), ("wdim0", 1827), ("wdimr", 1828), ("ssp1", 1829), ("lssp1", 1830), ("nssp1", 1831), ("pssp1", 1832), ("wssp1", 1833), ("p2ssp1", 1834), ("ssp2", 1835), ("lssp2", 1836), ("nssp2", 1837), ("pssp2", 1838), ("wssp2", 1839),
    ("p2ssp2", 1840), ("ssp3", 1841), ("lssp3", 1842), ("nssp3", 1843), ("pssp3", 1844), ("wssp3", 1845), ("p2ssp3", 1846), ("dssp1", 1847), ("dssp2", 1848), ("dssp3", 1849), ("wssp0", 1850), ("wsspr", 1851), ("wgaanom", 1852), ("e2nom", 1853), ("le2nom", 1854), ("ne2nom", 1855),
    ("pe2nom", 1856), ("we2nom", 1857), ("p2e2nom", 1858), ("e3nom", 1859), ("le3nom", 1860), ("ne3nom", 1861), ("pe3nom", 1862), ("we3nom", 1863), ("p2e3nom", 1864), ("mfe2", 1865), ("mfe3", 1866), ("wsfe2", 1867), ("wsfe3", 1868), ("mfq1nom", 1869), ("lmfq1nom", 1870), ("nmfq1nom", 1871),
    ("pmfq1nom", 1872), ("wmfq1nom", 1873), ("p2mfq1nom", 1874), ("mfq2nom", 1875), ("lmfq2nom", 1876), ("nmfq2nom", 1877), ("pmfq2nom", 1878), ("wmfq2nom", 1879), ("p2mfq2nom", 1880), ("mfq3nom", 1881), ("lmfq3nom", 1882), ("nmfq3nom", 1883), ("pmfq3nom", 1884), ("wmfq3nom", 1885), ("p2mfq3nom", 1886), ("mfq1", 1887),
    ("mfq2", 1888), ("mfq3", 1889), ("wsfq1", 1890), ("wsfq2", 1891), ("wsfq3", 1892), ("tsre2", 1893), ("tdwse2", 1894), ("tsre3", 1895), ("tdwse3", 1896), ("tsrq1", 1897), ("tdwsq1", 1898), ("tsrq2", 1899), ("tdwsq2", 1900), ("tsrq3", 1901), ("tdwsq3", 1902), ("nvsrd", 1903),
    ("vsatrsd", 1904), ("ptwgvsrsd", 1905), ("ptwg1vsrsd", 1906), ("psatxvsrsd", 1907), ("mvsrsd", 1908), ("nvsrs", 1909), ("rdlcw", 1910), ("rslcw", 1911), ("prtvsrsd", 1912), ("atvsrsd", 1913), ("vsrdfactor", 1914), ("vsrsfactor", 1915), ("rdvds", 1916), ("gavsrd", 1917),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1918] = [
    None, None, None, None, Some(3), None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1918] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1918] = [
    "l", "lover", "dia", "tfin", "fpitch", "nfin", "ngcon", "aseo", "adeo", "pseo", "pdeo", "asej", "adej", "psej", "pdej", "cgsp",
    "cgdp", "cdsp", "nrs", "nrd", "lrsd", "nfinnom", "dtemp", "delvtrand", "u0mult", "ids0mult", "igc0mult", "igb0mult", "covs", "lcovs", "ncovs", "pcovs",
    "wcovs", "p2covs", "covd", "lcovd", "ncovd", "pcovd", "wcovd", "p2covd", "tgaa", "tsus", "hpff", "wgaa", "dws1", "dach1", "dws2", "dach2",
    "dws3", "dach3", "dws4", "dach4", "dws5", "dach5", "dws6", "dach6", "ngaa", "subbandmod", "mobscmod", "nf", "type", "bulkmod", "geomod", "cgeo1sw",
    "rdsmod", "hvmod", "asymmod", "cvmod", "igcmod", "igbmod", "gidlmod", "iimod", "tnoimod", "nqsmod", "shmod", "tempmod", "rgatemod", "rgeomod", "cgeomod", "fnmod",
    "cryomod", "sh_warn", "igclamp", "ll", "lln", "dlc", "dlcacc", "dwcacc", "llc", "eot", "toxp", "eotbox", "hfin", "deltaw", "deltawcv", "nbodyn1",
    "nbodyn2", "nsd", "phigl", "phiglt", "phign1", "phign2", "epsrox", "epsrsub", "easub", "ni0sub", "bg0sub", "nc0sub", "imin", "xl", "lxl", "nxl",
    "pxl", "lint", "llint", "nlint", "plint", "dlbin", "ldlbin", "ndlbin", "pdlbin", "xw", "lxw", "nxw", "pxw", "wxw", "p2xw", "dwbin",
    "ldwbin", "ndwbin", "pdwbin", "wdwbin", "p2dwbin", "nbody", "lnbody", "nnbody", "pnbody", "phig", "lphig", "nphig", "pphig", "wphig", "p2phig", "vfbdriftd",
    "vfbdrifts", "ngate", "lngate", "nngate", "pngate", "wngate", "p2ngate", "minr", "cdscn1", "cdscn2", "cdscdn1", "cdscdn2", "cdscdrn1", "cdscdrn2", "eta0n1", "eta0n2",
    "eta0lt", "eta0n1cv", "eta0n2cv", "eta0ltcv", "teta0", "teta0cv", "teta0r", "advtp0", "bdvtp0", "advtp1", "bdvtp1", "dvtp2", "thetasce", "thetadibl", "thetasw", "nvtm",
    "dvtp0", "ldvtp0", "ndvtp0", "pdvtp0", "wdvtp0", "p2dvtp0", "dvtp1", "ldvtp1", "ndvtp1", "pdvtp1", "wdvtp1", "p2dvtp1", "cit", "lcit", "ncit", "pcit",
    "wcit", "p2cit", "citr", "lcitr", "ncitr", "pcitr", "wcitr", "p2citr", "cdsc", "lcdsc", "ncdsc", "pcdsc", "wcdsc", "p2cdsc", "cdscd", "lcdscd",
    "ncdscd", "pcdscd", "wcdscd", "p2cdscd", "cdscdr", "lcdscdr", "ncdscdr", "pcdscdr", "wcdscdr", "p2cdscdr", "dvt0", "ldvt0", "ndvt0", "pdvt0", "wdvt0", "p2dvt0",
    "dvt1", "ldvt1", "ndvt1", "pdvt1", "wdvt1", "p2dvt1", "dvt1ss", "ldvt1ss", "ndvt1ss", "pdvt1ss", "wdvt1ss", "p2dvt1ss", "phin", "lphin", "nphin", "pphin",
    "wphin", "p2phin", "eta0", "leta0", "neta0", "peta0", "weta0", "p2eta0", "eta1", "leta1", "neta1", "peta1", "weta1", "p2eta1", "eta0r", "leta0r",
    "neta0r", "peta0r", "weta0r", "p2eta0r", "eta0cv", "leta0cv", "neta0cv", "peta0cv", "weta0cv", "p2eta0cv", "dsub", "ldsub", "ndsub", "pdsub", "wdsub", "p2dsub",
    "k1rsce", "lk1rsce", "nk1rsce", "pk1rsce", "wk1rsce", "p2k1rsce", "lpe0", "llpe0", "nlpe0", "plpe0", "wlpe0", "p2lpe0", "dvtshift", "ldvtshift", "ndvtshift", "pdvtshift",
    "wdvtshift", "p2dvtshift", "dvtshiftr", "ldvtshiftr", "ndvtshiftr", "pdvtshiftr", "wdvtshiftr", "p2dvtshiftr", "k0", "lk0", "nk0", "pk0", "wk0", "p2k0", "k01", "lk01",
    "nk01", "pk01", "wk01", "p2k01", "k0si", "lk0si", "nk0si", "pk0si", "wk0si", "p2k0si", "k0si1", "lk0si1", "nk0si1", "pk0si1", "wk0si1", "p2k0si1",
    "k2si", "lk2si", "nk2si", "pk2si", "wk2si", "p2k2si", "k2si1", "lk2si1", "nk2si1", "pk2si1", "wk2si1", "p2k2si1", "k0sisat", "lk0sisat", "nk0sisat", "pk0sisat",
    "wk0sisat", "p2k0sisat", "k0sisat1", "lk0sisat1", "nk0sisat1", "pk0sisat1", "wk0sisat1", "p2k0sisat1", "k2sisat", "lk2sisat", "nk2sisat", "pk2sisat", "wk2sisat", "p2k2sisat", "k2sisat1", "lk2sisat1",
    "nk2sisat1", "pk2sisat1", "wk2sisat1", "p2k2sisat1", "phibe", "lphibe", "nphibe", "pphibe", "wphibe", "p2phibe", "k1", "lk1", "nk1", "pk1", "wk1", "p2k1",
    "k11", "lk11", "nk11", "pk11", "wk11", "p2k11", "k2sat", "lk2sat", "nk2sat", "pk2sat", "wk2sat", "p2k2sat", "k2sat1", "lk2sat1", "nk2sat1", "pk2sat1",
    "wk2sat1", "p2k2sat1", "k2", "lk2", "nk2", "pk2", "wk2", "p2k2", "k21", "lk21", "nk21", "pk21", "wk21", "p2k21", "aqmtcen", "bqmtcen",
    "qm0", "qm0acc", "pqmacc", "qmfactor", "lqmfactor", "nqmfactor", "pqmfactor", "wqmfactor", "p2qmfactor", "qmtcencv", "lqmtcencv", "nqmtcencv", "pqmtcencv", "wqmtcencv", "p2qmtcencv", "qmtcencva",
    "lqmtcencva", "nqmtcencva", "pqmtcencva", "wqmtcencva", "p2qmtcencva", "pqm", "lpqm", "npqm", "ppqm", "wpqm", "p2pqm", "pqml", "vsatn1", "vsatn2", "avsat", "bvsat",
    "vsat1n1", "vsat1n2", "vsat1rn1", "vsat1rn2", "avsat1", "bvsat1", "apsat", "bpsat", "avsatcv", "bvsatcv", "apsatcv", "bpsatcv", "amexp", "bmexp", "amexpr", "bmexpr",
    "aptwg", "bptwg", "tmexp", "tmexp2", "tmexpr", "dvsatclamp", "vsatdr", "vsat", "lvsat", "nvsat", "pvsat", "wvsat", "p2vsat", "vsatr", "lvsatr", "nvsatr",
    "pvsatr", "wvsatr", "p2vsatr", "vsat1", "lvsat1", "nvsat1", "pvsat1", "wvsat1", "p2vsat1", "vsat1r", "lvsat1r", "nvsat1r", "pvsat1r", "wvsat1r", "p2vsat1r", "deltavsat",
    "ldeltavsat", "ndeltavsat", "pdeltavsat", "wdeltavsat", "p2deltavsat", "psat", "lpsat", "npsat", "ppsat", "wpsat", "p2psat", "ksativdr", "ksativ", "lksativ", "nksativ", "pksativ",
    "wksativ", "p2ksativ", "ksativt1", "ksativt2", "ksativr", "lksativr", "nksativr", "pksativr", "wksativr", "p2ksativr", "vsatcv", "lvsatcv", "nvsatcv", "pvsatcv", "wvsatcv", "p2vsatcv",
    "asat", "lasat", "nasat", "pasat", "wasat", "p2asat", "deltavsatcv", "ldeltavsatcv", "ndeltavsatcv", "pdeltavsatcv", "wdeltavsatcv", "p2deltavsatcv", "psatcv", "lpsatcv", "npsatcv", "ppsatcv",
    "wpsatcv", "p2psatcv", "mexpdr", "mexp", "lmexp", "nmexp", "pmexp", "wmexp", "p2mexp", "mexpr", "lmexpr", "nmexpr", "pmexpr", "wmexpr", "p2mexpr", "ptwg",
    "lptwg", "nptwg", "pptwg", "wptwg", "p2ptwg", "ptwgr", "lptwgr", "nptwgr", "pptwgr", "wptwgr", "p2ptwgr", "at", "lat", "nat", "pat", "wat",
    "p2at", "at2", "atr", "latr", "natr", "patr", "watr", "p2atr", "atcv", "latcv", "natcv", "patcv", "watcv", "p2atcv", "at2cv", "ptwgt",
    "lptwgt", "nptwgt", "pptwgt", "wptwgt", "p2ptwgt", "u0n1", "u0n1cv", "u0n1r", "u0n2", "u0n2cv", "u0n2r", "u0lt", "u0ltcv", "lpa", "lpar", "aua",
    "auar", "bua", "buar", "aeu", "aeur", "beu", "beur", "aud", "audr", "bud", "budr", "chargewf", "dmobclamp", "u0", "lu0", "nu0",
    "pu0", "wu0", "p2u0", "u0r", "lu0r", "nu0r", "pu0r", "wu0r", "p2u0r", "u0cv", "lu0cv", "nu0cv", "pu0cv", "wu0cv", "p2u0cv", "etamob",
    "letamob", "netamob", "petamob", "wetamob", "p2etamob", "up", "lup", "nup", "pup", "wup", "p2up", "upr", "lupr", "nupr", "pupr", "wupr",
    "p2upr", "ua", "lua", "nua", "pua", "wua", "p2ua", "uar", "luar", "nuar", "puar", "wuar", "p2uar", "uacv", "luacv", "nuacv",
    "puacv", "wuacv", "p2uacv", "uc", "luc", "nuc", "puc", "wuc", "p2uc", "ucr", "lucr", "nucr", "pucr", "wucr", "p2ucr", "uccv",
    "luccv", "nuccv", "puccv", "wuccv", "p2uccv", "eu", "leu", "neu", "peu", "weu", "p2eu", "eur", "leur", "neur", "peur", "weur",
    "p2eur", "ud", "lud", "nud", "pud", "wud", "p2ud", "udr", "ludr", "nudr", "pudr", "wudr", "p2udr", "udcv", "ludcv", "nudcv",
    "pudcv", "wudcv", "p2udcv", "ucs", "lucs", "nucs", "pucs", "wucs", "p2ucs", "uds", "luds", "nuds", "puds", "wuds", "p2uds", "uds1",
    "luds1", "nuds1", "puds1", "wuds1", "p2uds1", "udd", "ludd", "nudd", "pudd", "wudd", "p2udd", "udd1", "ludd1", "nudd1", "pudd1", "wudd1",
    "p2udd1", "ute", "lute", "nute", "pute", "wute", "p2ute", "uter", "luter", "nuter", "puter", "wuter", "p2uter", "utecv", "lutecv", "nutecv",
    "putecv", "wutecv", "p2utecv", "ute1", "lute1", "nute1", "pute1", "wute1", "p2ute1", "ute1cv", "lute1cv", "nute1cv", "pute1cv", "wute1cv", "p2ute1cv", "utl",
    "lutl", "nutl", "putl", "wutl", "p2utl", "utlr", "lutlr", "nutlr", "putlr", "wutlr", "p2utlr", "utlcv", "lutlcv", "nutlcv", "putlcv", "wutlcv",
    "p2utlcv", "emobt", "lemobt", "nemobt", "pemobt", "wemobt", "p2emobt", "ua1", "lua1", "nua1", "pua1", "wua1", "p2ua1", "ua1r", "lua1r", "nua1r",
    "pua1r", "wua1r", "p2ua1r", "ua1cv", "lua1cv", "nua1cv", "pua1cv", "wua1cv", "p2ua1cv", "ua2", "lua2", "nua2", "pua2", "wua2", "p2ua2", "ua2cv",
    "lua2cv", "nua2cv", "pua2cv", "wua2cv", "p2ua2cv", "eu1", "leu1", "neu1", "peu1", "weu1", "p2eu1", "uc1", "luc1", "nuc1", "puc1", "wuc1",
    "p2uc1", "uc1r", "luc1r", "nuc1r", "puc1r", "wuc1r", "p2uc1r", "uc1cv", "luc1cv", "nuc1cv", "puc1cv", "wuc1cv", "p2uc1cv", "ud1", "lud1", "nud1",
    "pud1", "wud1", "p2ud1", "ud1r", "lud1r", "nud1r", "pud1r", "wud1r", "p2ud1r", "ud1cv", "lud1cv", "nud1cv", "pud1cv", "wud1cv", "p2ud1cv", "ud2",
    "lud2", "nud2", "pud2", "wud2", "p2ud2", "ud2cv", "lud2cv", "nud2cv", "pud2cv", "wud2cv", "p2ud2cv", "ucste", "lucste", "nucste", "pucste", "wucste",
    "p2ucste", "ucste1", "lucste1", "nucste1", "pucste1", "wucste1", "p2ucste1", "muhc0", "muhc1", "etamobthin", "etamobtni", "etamobir", "uathin", "uatsat", "uartsc", "uatni",
    "uair", "euthin", "euptsc", "eutni", "euir", "udthin", "udtsat", "udptsc", "u0etawsc", "egbulk", "u0emsm1", "u0emsm2", "rdswmin", "ardsw", "brdsw", "rswmin",
    "arsw", "brsw", "rdwmin", "ardw", "brdw", "rsdr", "rsdrr", "rddr", "rddrr", "prsdr", "prddr", "trsdr", "trddr", "rdsw", "lrdsw", "nrdsw",
    "prdsw", "wrdsw", "p2rdsw", "rsw", "lrsw", "nrsw", "prsw", "wrsw", "p2rsw", "rdw", "lrdw", "nrdw", "prdw", "wrdw", "p2rdw", "prwgs",
    "lprwgs", "nprwgs", "pprwgs", "wprwgs", "p2prwgs", "prwgd", "lprwgd", "nprwgd", "pprwgd", "wprwgd", "p2prwgd", "wr", "lwr", "nwr", "pwr", "wwr",
    "p2wr", "prt", "lprt", "nprt", "pprt", "wprt", "p2prt", "prt1", "lprt1", "nprt1", "pprt1", "wprt1", "p2prt1", "tr0", "ltr0", "ntr0",
    "ptr0", "wtr0", "p2tr0", "sprt", "lsprt", "nsprt", "psprt", "wsprt", "p2sprt", "pdibl1", "lpdibl1", "npdibl1", "ppdibl1", "wpdibl1", "p2pdibl1", "pdibl2",
    "lpdibl2", "npdibl2", "ppdibl2", "wpdibl2", "p2pdibl2", "pdibl1r", "lpdibl1r", "npdibl1r", "ppdibl1r", "wpdibl1r", "p2pdibl1r", "pdibl2r", "lpdibl2r", "npdibl2r", "ppdibl2r", "wpdibl2r",
    "p2pdibl2r", "drout", "ldrout", "ndrout", "pdrout", "wdrout", "p2drout", "pvag", "lpvag", "npvag", "ppvag", "wpvag", "p2pvag", "apclm", "apclmr", "bpclm",
    "bpclmr", "pclm", "pclmt", "lpclm", "npclm", "ppclm", "wpclm", "p2pclm", "pclmr", "lpclmr", "npclmr", "ppclmr", "wpclmr", "p2pclmr", "pclmg", "lpclmg",
    "npclmg", "ppclmg", "wpclmg", "p2pclmg", "pclmcv", "lpclmcv", "npclmcv", "ppclmcv", "wpclmcv", "p2pclmcv", "a1", "la1", "na1", "pa1", "wa1", "p2a1",
    "a11", "la11", "na11", "pa11", "wa11", "p2a11", "a2", "la2", "na2", "pa2", "wa2", "p2a2", "a21", "la21", "na21", "pa21",
    "wa21", "p2a21", "rgext", "rgfin", "rgint", "rgp", "rshs", "rshd", "hepi", "tsili", "rhoc", "rhorsd", "cratio", "deltaprsd", "sdterm", "lsp",
    "epsrsp", "tgate", "tmask", "asiliend", "arsdend", "prsdend", "rgeoa", "rgeob", "rgeoc", "rgeod", "rgeoe", "cgeoa", "cgeob", "cgeoc", "cgeod", "cgeoe",
    "dlcigs", "dlcigd", "vfbsd", "vfbsdcv", "toxref", "toxg", "igbinvclamp", "igbaccclamp", "igcinvclamp", "ntox", "lntox", "nntox", "pntox", "wntox", "p2ntox", "aigbinv",
    "laigbinv", "naigbinv", "paigbinv", "waigbinv", "p2aigbinv", "aigbinv1", "laigbinv1", "naigbinv1", "paigbinv1", "waigbinv1", "p2aigbinv1", "bigbinv", "lbigbinv", "nbigbinv", "pbigbinv", "wbigbinv",
    "p2bigbinv", "cigbinv", "lcigbinv", "ncigbinv", "pcigbinv", "wcigbinv", "p2cigbinv", "eigbinv", "leigbinv", "neigbinv", "peigbinv", "weigbinv", "p2eigbinv", "nigbinv", "lnigbinv", "nnigbinv",
    "pnigbinv", "wnigbinv", "p2nigbinv", "aigbacc", "laigbacc", "naigbacc", "paigbacc", "waigbacc", "p2aigbacc", "aigbacc1", "laigbacc1", "naigbacc1", "paigbacc1", "waigbacc1", "p2aigbacc1", "bigbacc",
    "lbigbacc", "nbigbacc", "pbigbacc", "wbigbacc", "p2bigbacc", "cigbacc", "lcigbacc", "ncigbacc", "pcigbacc", "wcigbacc", "p2cigbacc", "nigbacc", "lnigbacc", "nnigbacc", "pnigbacc", "wnigbacc",
    "p2nigbacc", "aigc", "laigc", "naigc", "paigc", "waigc", "p2aigc", "aigc1", "laigc1", "naigc1", "paigc1", "waigc1", "p2aigc1", "bigc", "lbigc", "nbigc",
    "pbigc", "wbigc", "p2bigc", "cigc", "lcigc", "ncigc", "pcigc", "wcigc", "p2cigc", "pigcd", "lpigcd", "npigcd", "ppigcd", "wpigcd", "p2pigcd", "aigs",
    "laigs", "naigs", "paigs", "waigs", "p2aigs", "aigs1", "laigs1", "naigs1", "paigs1", "waigs1", "p2aigs1", "bigs", "lbigs", "nbigs", "pbigs", "wbigs",
    "p2bigs", "cigs", "lcigs", "ncigs", "pcigs", "wcigs", "p2cigs", "aigd", "laigd", "naigd", "paigd", "waigd", "p2aigd", "aigd1", "laigd1", "naigd1",
    "paigd1", "waigd1", "p2aigd1", "bigd", "lbigd", "nbigd", "pbigd", "wbigd", "p2bigd", "cigd", "lcigd", "ncigd", "pcigd", "wcigd", "p2cigd", "poxedge",
    "lpoxedge", "npoxedge", "ppoxedge", "wpoxedge", "p2poxedge", "agidl", "lagidl", "nagidl", "pagidl", "wagidl", "p2agidl", "bgidl", "lbgidl", "nbgidl", "pbgidl", "wbgidl",
    "p2bgidl", "cgidl", "lcgidl", "ncgidl", "pcgidl", "wcgidl", "p2cgidl", "egidl", "legidl", "negidl", "pegidl", "wegidl", "p2egidl", "pgidl", "lpgidl", "npgidl",
    "ppgidl", "wpgidl", "p2pgidl", "agisl", "lagisl", "nagisl", "pagisl", "wagisl", "p2agisl", "bgisl", "lbgisl", "nbgisl", "pbgisl", "wbgisl", "p2bgisl", "cgisl",
    "lcgisl", "ncgisl", "pcgisl", "wcgisl", "p2cgisl", "egisl", "legisl", "negisl", "pegisl", "wegisl", "p2egisl", "pgisl", "lpgisl", "npgisl", "ppgisl", "wpgisl",
    "p2pgisl", "atatd", "latatd", "natatd", "patatd", "watatd", "p2atatd", "btatd", "lbtatd", "nbtatd", "pbtatd", "wbtatd", "p2btatd", "ctatd", "lctatd", "nctatd",
    "pctatd", "wctatd", "p2ctatd", "dtatd", "ldtatd", "ndtatd", "pdtatd", "wdtatd", "p2dtatd", "atats", "latats", "natats", "patats", "watats", "p2atats", "btats",
    "lbtats", "nbtats", "pbtats", "wbtats", "p2btats", "ctats", "lctats", "nctats", "pctats", "wctats", "p2ctats", "dtats", "ldtats", "ndtats", "pdtats", "wdtats",
    "p2dtats", "agidlb", "lagidlb", "nagidlb", "pagidlb", "wagidlb", "p2agidlb", "bgidlb", "lbgidlb", "nbgidlb", "pbgidlb", "wbgidlb", "p2bgidlb", "cgidlb", "lcgidlb", "ncgidlb",
    "pcgidlb", "wcgidlb", "p2cgidlb", "egidlb", "legidlb", "negidlb", "pegidlb", "wegidlb", "p2egidlb", "pgidlb", "lpgidlb", "npgidlb", "ppgidlb", "wpgidlb", "p2pgidlb", "agislb",
    "lagislb", "nagislb", "pagislb", "wagislb", "p2agislb", "bgislb", "lbgislb", "nbgislb", "pbgislb", "wbgislb", "p2bgislb", "cgislb", "lcgislb", "ncgislb", "pcgislb", "wcgislb",
    "p2cgislb", "egislb", "legislb", "negislb", "pegislb", "wegislb", "p2egislb", "pgislb", "lpgislb", "npgislb", "ppgislb", "wpgislb", "p2pgislb", "alpha01", "alpha11", "alphaii01",
    "alphaii11", "iimod2clamp1", "iimod2clamp2", "iimod2clamp3", "alpha0", "lalpha0", "nalpha0", "palpha0", "walpha0", "p2alpha0", "alpha1", "lalpha1", "nalpha1", "palpha1", "walpha1", "p2alpha1",
    "beta0", "lbeta0", "nbeta0", "pbeta0", "wbeta0", "p2beta0", "alphaii0", "lalphaii0", "nalphaii0", "palphaii0", "walphaii0", "p2alphaii0", "alphaii1", "lalphaii1", "nalphaii1", "palphaii1",
    "walphaii1", "p2alphaii1", "betaii0", "lbetaii0", "nbetaii0", "pbetaii0", "wbetaii0", "p2betaii0", "betaii1", "lbetaii1", "nbetaii1", "pbetaii1", "wbetaii1", "p2betaii1", "betaii2", "lbetaii2",
    "nbetaii2", "pbetaii2", "wbetaii2", "p2betaii2", "esatii", "lesatii", "nesatii", "pesatii", "wesatii", "p2esatii", "lii", "llii", "nlii", "plii", "wlii", "p2lii",
    "sii0", "lsii0", "nsii0", "psii0", "wsii0", "p2sii0", "sii1", "lsii1", "nsii1", "psii1", "wsii1", "p2sii1", "sii2", "lsii2", "nsii2", "psii2",
    "wsii2", "p2sii2", "siid", "lsiid", "nsiid", "psiid", "wsiid", "p2siid", "eotacc", "delvfbacc", "cfs", "lcfs", "ncfs", "pcfs", "wcfs", "p2cfs",
    "cfd", "lcfd", "ncfd", "pcfd", "wcfd", "p2cfd", "cgso", "cgdo", "cgbo", "cgbn", "cgbw", "cgsl", "lcgsl", "ncgsl", "pcgsl", "wcgsl",
    "p2cgsl", "cgdl", "lcgdl", "ncgdl", "pcgdl", "wcgdl", "p2cgdl", "cgbl", "lcgbl", "ncgbl", "pcgbl", "wcgbl", "p2cgbl", "ckappas", "lckappas", "nckappas",
    "pckappas", "wckappas", "p2ckappas", "ckappad", "lckappad", "nckappad", "pckappad", "wckappad", "p2ckappad", "ckappab", "lckappab", "nckappab", "pckappab", "wckappab", "p2ckappab", "csdesw",
    "cjs", "cjd", "cjsws", "cjswd", "cjswgs", "cjswgd", "pbs", "pbd", "pbsws", "pbswd", "pbswgs", "pbswgd", "mjs", "mjd", "mjsws", "mjswd",
    "mjswgs", "mjswgd", "sjs", "sjd", "sjsws", "sjswd", "sjswgs", "sjswgd", "mjs2", "mjd2", "mjsws2", "mjswd2", "mjswgs2", "mjswgd2", "jss", "jsd",
    "jsws", "jswd", "jswgs", "jswgd", "njs", "njd", "ijthsfwd", "ijthdfwd", "ijthsrev", "ijthdrev", "bvs", "bvd", "xjbvs", "xjbvd", "jtss", "jtsd",
    "jtssws", "jtsswd", "jtsswgs", "jtsswgd", "jtweff", "njts", "njtsd", "njtssw", "njtsswd", "njtsswg", "njtsswgd", "vtss", "vtsd", "vtssws", "vtsswd", "vtsswgs",
    "vtsswgd", "lintigen", "ntgen", "lntgen", "nntgen", "pntgen", "wntgen", "p2ntgen", "aigen", "laigen", "naigen", "paigen", "waigen", "p2aigen", "bigen", "lbigen",
    "nbigen", "pbigen", "wbigen", "p2bigen", "xrcrg1", "lxrcrg1", "nxrcrg1", "pxrcrg1", "wxrcrg1", "p2xrcrg1", "xrcrg2", "lxrcrg2", "nxrcrg2", "pxrcrg2", "wxrcrg2", "p2xrcrg2",
    "ef", "em", "noia", "noib", "noic", "k0noi", "k1noi", "lintnoi", "smooth", "noia2", "lnoia2", "nnoia2", "pnoia2", "wnoia2", "p2noia2", "mpower",
    "lmpower", "nmpower", "pmpower", "wmpower", "p2mpower", "qsref", "lqsref", "nqsref", "pqsref", "wqsref", "p2qsref", "ntnoi", "rnoia", "tnoia", "rnoib", "tnoib",
    "rnoic", "tnoic", "rnoik", "tnoik", "tnoik2", "tnom", "tbgasub", "tbgbsub", "kt1l", "tcj", "tcjsw", "tcjswg", "tpb", "tpbsw", "tpbswg", "xtis",
    "xtid", "xtss", "xtsd", "xtssws", "xtsswd", "xtsswgs", "xtsswgd", "tnjts", "tnjtsd", "tnjtssw", "tnjtsswd", "tnjtsswg", "tnjtsswgd", "kt1", "lkt1", "nkt1",
    "pkt1", "wkt1", "p2kt1", "kt11", "kt12", "tvth", "tss", "ltss", "ntss", "ptss", "wtss", "p2tss", "iit", "liit", "niit", "piit",
    "wiit", "p2iit", "tii", "ltii", "ntii", "ptii", "wtii", "p2tii", "tgidl", "ltgidl", "ntgidl", "ptgidl", "wtgidl", "p2tgidl", "ttat", "lttat",
    "nttat", "pttat", "wttat", "p2ttat", "igt", "ligt", "nigt", "pigt", "wigt", "p2igt", "tlow", "tlow1", "dtlow", "dtlow1", "klow1", "rth0",
    "cth0", "wth0", "ashexp", "bshexp", "cshexp", "ash", "csh", "ach_ufcm", "cins_ufcm", "w_ufcm", "tfin_top", "tfin_base", "qmfactorcv", "alpha_ufcm", "dim1h", "dimension1",
    "ldimension1", "ndimension1", "pdimension1", "wdimension1", "p2dimension1", "dim2h", "dimension2", "ldimension2", "ndimension2", "pdimension2", "wdimension2", "p2dimension2", "dim3h", "dimension3", "ldimension3", "ndimension3",
    "pdimension3", "wdimension3", "p2dimension3", "wdim0", "wdimr", "ssp1", "lssp1", "nssp1", "pssp1", "wssp1", "p2ssp1", "ssp2", "lssp2", "nssp2", "pssp2", "wssp2",
    "p2ssp2", "ssp3", "lssp3", "nssp3", "pssp3", "wssp3", "p2ssp3", "dssp1", "dssp2", "dssp3", "wssp0", "wsspr", "wgaanom", "e2nom", "le2nom", "ne2nom",
    "pe2nom", "we2nom", "p2e2nom", "e3nom", "le3nom", "ne3nom", "pe3nom", "we3nom", "p2e3nom", "mfe2", "mfe3", "wsfe2", "wsfe3", "mfq1nom", "lmfq1nom", "nmfq1nom",
    "pmfq1nom", "wmfq1nom", "p2mfq1nom", "mfq2nom", "lmfq2nom", "nmfq2nom", "pmfq2nom", "wmfq2nom", "p2mfq2nom", "mfq3nom", "lmfq3nom", "nmfq3nom", "pmfq3nom", "wmfq3nom", "p2mfq3nom", "mfq1",
    "mfq2", "mfq3", "wsfq1", "wsfq2", "wsfq3", "tsre2", "tdwse2", "tsre3", "tdwse3", "tsrq1", "tdwsq1", "tsrq2", "tdwsq2", "tsrq3", "tdwsq3", "nvsrd",
    "vsatrsd", "ptwgvsrsd", "ptwg1vsrsd", "psatxvsrsd", "mvsrsd", "nvsrs", "rdlcw", "rslcw", "prtvsrsd", "atvsrsd", "vsrdfactor", "vsrsfactor", "rdvds", "gavsrd",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1918] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1918] = [
    false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false,
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
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1918] = [
    Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-20, label: "1e-20" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None,
    None, Some(ParameterBound { value: 2e25, label: "2e25" }), None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1e-5, label: "1e-5" }),
    None, None, Some(ParameterBound { value: 1e-5, label: "1e-5" }), None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.01, label: "0.01" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.01, label: "0.01" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.001, label: "0.001" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, Some(ParameterBound { value: 1e-18, label: "1e-18" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1e-10, label: "1e-10" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
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
    Some(ParameterBound { value: 1e-10, label: "1e-10" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -273.15, label: "-273.15" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, Some(ParameterBound { value: 0.0001, label: "0.0001" }), Some(ParameterBound { value: 0.0001, label: "0.0001" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1918] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 6.0, label: "6.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }),
    Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 1e27, label: "1e27" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None,
    None, None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None,
    None, None, None, None, Some(ParameterBound { value: 3.0, label: "3.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 4.0, label: "4.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1918] = [
    2, 2, 2, 2, 2, 3, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 2, 0, 0, 3, 2, 2, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 2, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 2, 2, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2,
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 0, 2, 3, 0, 2, 2, 2,
    3, 0, 2, 2, 3, 0, 2, 2, 2, 2, 2, 0, 2, 0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3,
    2, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 3, 2, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0,
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
    0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3,
    3, 3, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 2, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 2, 2, 2, 3, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 2, 3, 3, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 0, 0, 2, 2, 2, 2, 0, 0, 3, 3, 2, 0, 3, 3, 3, 2, 2, 0, 0, 0, 0, 0, 2,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1918] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[],
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
    pub nodes: [usize; 17],
    pub branches: [usize; 18],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1918]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 28]>,
    pub(crate) ddt_state_previous: Box<[f64; 28]>,
    pub(crate) ddt_state_older: Box<[f64; 28]>,
    pub(crate) ddt_state_initialized: Box<[bool; 28]>,
    pub(crate) ddt_derivative_current: Box<[f64; 28]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 28]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<1763, 17, 18>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<1763, 17, 18>>>,
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
    pub const INTERNAL_NODE_NAMES: [&str; 12] = ["di", "si", "di1", "si1", "di2", "ge", "gi", "gint", "gints", "gintd", "q", "n"];

    pub const BRANCH_COUNT: usize = 18;
    pub const PARAMETER_COUNT: usize = 1918;
    pub const VARIABLE_COUNT: usize = 1763;
    pub const DDT_STATE_COUNT: usize = 28;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "740d749e25ffd6c6ef9f88cde1428aafa416ef273b666a272bc0a97f3bda5a2d";
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
            return Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimcmg_va'", name));
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
