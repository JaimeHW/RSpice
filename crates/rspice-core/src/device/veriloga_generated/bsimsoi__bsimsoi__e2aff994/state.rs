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
    pub p1400: f64,
}

impl Parameters {
    fn new_box() -> Box<Self> {
        // SAFETY: Parameters is repr(C) and every field is f64; zero bytes are valid 0.0 values, and numeric default chunks are copied into field-order slots.
        let mut boxed = Box::<Self>::new_uninit();
        unsafe {
            let ptr = boxed.as_mut_ptr();
            std::ptr::write_bytes(ptr, 0, 1);
            const DEFAULTS_0: [f64; 77] = [
                1e-5, 1e-5, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-5, 1.0, 1.0, 0.0, 1e-5, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 4e-8, 2e-7, 3e-9,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_0.as_ptr(), (ptr as *mut f64).add(0), 77);
            {
                let params = &mut *ptr;
                params.p77 = params.p76;
                validate_parameter("TOXP", params.p77, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_1: [f64; 13] = [
                0.0, 1e24, 0.0, 1.0, 0.0, 2.0, 0.0, 1.0,
                0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_1.as_ptr(), (ptr as *mut f64).add(78), 13);
            {
                let params = &mut *ptr;
                params.p91 = params.p79;
                validate_finite_parameter("NDEPCV", params.p91).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p92 = params.p80;
                validate_finite_parameter("NDEPCVL1", params.p92).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p93 = params.p81;
                validate_parameter("NDEPCVLEXP1", params.p93, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p94 = params.p82;
                validate_finite_parameter("NDEPCVL2", params.p94).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p95 = params.p83;
                validate_parameter("NDEPCVLEXP2", params.p95, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p96 = params.p84;
                validate_finite_parameter("NDEPCVW", params.p96).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p97 = params.p85;
                validate_parameter("NDEPCVWEXP", params.p97, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p98 = params.p86;
                validate_finite_parameter("NDEPCVWL", params.p98).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p99 = params.p87;
                validate_parameter("NDEPCVWLEXP", params.p99, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p100 = params.p88;
                validate_finite_parameter("LNDEPCV", params.p100).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p101 = params.p89;
                validate_finite_parameter("WNDEPCV", params.p101).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p102 = params.p90;
                validate_finite_parameter("PNDEPCV", params.p102).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_2: [f64; 26] = [
                5e25, 0.0, 0.0, 0.0, 1.1e16, 1.17, 11.9, 3.9,
                1.5e-7, 0.0, 0.0, 0.0, -0.5, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_2.as_ptr(), (ptr as *mut f64).add(103), 26);
            {
                let params = &mut *ptr;
                params.p129 = params.p115;
                validate_finite_parameter("VFBCV", params.p129).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p130 = params.p116;
                validate_finite_parameter("LVFBCV", params.p130).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p131 = params.p117;
                validate_finite_parameter("WVFBCV", params.p131).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p132 = params.p118;
                validate_finite_parameter("PVFBCV", params.p132).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p133 = params.p123;
                validate_finite_parameter("VFBCVL", params.p133).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p134 = params.p124;
                validate_parameter("VFBCVLEXP", params.p134, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p135 = params.p125;
                validate_finite_parameter("VFBCVW", params.p135).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p136 = params.p126;
                validate_parameter("VFBCVWEXP", params.p136, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p137 = params.p127;
                validate_finite_parameter("VFBCVWL", params.p137).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p138 = params.p128;
                validate_parameter("VFBCVWLEXP", params.p138, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_3: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_3.as_ptr(), (ptr as *mut f64).add(139), 1);
            {
                let params = &mut *ptr;
                params.p140 = params.p115;
                validate_finite_parameter("VFBAGBCP2", params.p140).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p141 = params.p79;
                validate_parameter("NDEPAGBCP2", params.p141, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_4: [f64; 56] = [
                1e26, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.045, 0.0, 0.0, 0.0, 0.08, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_4.as_ptr(), (ptr as *mut f64).add(142), 56);
            {
                let params = &mut *ptr;
                params.p198 = params.p194;
                validate_finite_parameter("ETA0R", params.p198).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p199 = params.p195;
                validate_finite_parameter("LETA0R", params.p199).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p200 = params.p196;
                validate_finite_parameter("WETA0R", params.p200).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p201 = params.p197;
                validate_finite_parameter("PETA0R", params.p201).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_5: [f64; 82] = [
                1.0, -0.07, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 0.001, 0.54, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_5.as_ptr(), (ptr as *mut f64).add(202), 82);
            {
                let params = &mut *ptr;
                params.p284 = params.p258;
                validate_finite_parameter("CDSCDR", params.p284).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p285 = params.p259;
                validate_finite_parameter("LCDSCDR", params.p285).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p286 = params.p260;
                validate_finite_parameter("WCDSCDR", params.p286).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p287 = params.p261;
                validate_finite_parameter("PCDSCDR", params.p287).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_6: [f64; 17] = [
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 100000.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_6.as_ptr(), (ptr as *mut f64).add(288), 17);
            {
                let params = &mut *ptr;
                params.p305 = params.p295;
                validate_finite_parameter("VSATR", params.p305).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p306 = params.p296;
                validate_finite_parameter("LVSATR", params.p306).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p307 = params.p297;
                validate_finite_parameter("WVSATR", params.p307).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p308 = params.p298;
                validate_finite_parameter("PVSATR", params.p308).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_7: [f64; 6] = [
                0.125, 0.0, 0.0, 0.0, 0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_7.as_ptr(), (ptr as *mut f64).add(309), 6);
            {
                let params = &mut *ptr;
                params.p315 = params.p295;
                validate_finite_parameter("VSATCV", params.p315).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p316 = params.p296;
                validate_finite_parameter("LVSATCV", params.p316).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p317 = params.p297;
                validate_finite_parameter("WVSATCV", params.p317).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p318 = params.p298;
                validate_finite_parameter("PVSATCV", params.p318).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p319 = params.p299;
                validate_finite_parameter("VSATCVL", params.p319).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p320 = params.p300;
                validate_parameter("VSATCVLEXP", params.p320, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p321 = params.p301;
                validate_finite_parameter("VSATCVW", params.p321).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p322 = params.p302;
                validate_parameter("VSATCVWEXP", params.p322, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p323 = params.p303;
                validate_finite_parameter("VSATCVWL", params.p323).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p324 = params.p304;
                validate_parameter("VSATCVWLEXP", params.p324, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_8: [f64; 18] = [
                0.3, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-8, 0.0, 1e-8, 0.067, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_8.as_ptr(), (ptr as *mut f64).add(325), 18);
            {
                let params = &mut *ptr;
                params.p343 = params.p337;
                validate_finite_parameter("U0R", params.p343).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p344 = params.p340;
                validate_finite_parameter("LU0R", params.p344).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p345 = params.p341;
                validate_finite_parameter("WU0R", params.p345).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p346 = params.p342;
                validate_finite_parameter("PU0R", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_9: [f64; 11] = [
                1.0, 0.001, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_9.as_ptr(), (ptr as *mut f64).add(347), 11);
            {
                let params = &mut *ptr;
                params.p358 = params.p348;
                validate_finite_parameter("UAR", params.p358).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p359 = params.p355;
                validate_finite_parameter("LUAR", params.p359).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p360 = params.p356;
                validate_finite_parameter("WUAR", params.p360).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p361 = params.p357;
                validate_finite_parameter("PUAR", params.p361).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_10: [f64; 16] = [
                1.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 0.001, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_10.as_ptr(), (ptr as *mut f64).add(362), 16);
            {
                let params = &mut *ptr;
                params.p378 = params.p372;
                validate_finite_parameter("UDR", params.p378).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p379 = params.p375;
                validate_finite_parameter("LUDR", params.p379).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p380 = params.p376;
                validate_finite_parameter("WUDR", params.p380).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p381 = params.p377;
                validate_finite_parameter("PUDR", params.p381).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_11: [f64; 4] = [
                2.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_11.as_ptr(), (ptr as *mut f64).add(382), 4);
            {
                let params = &mut *ptr;
                params.p386 = params.p382;
                validate_finite_parameter("UCSR", params.p386).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p387 = params.p383;
                validate_finite_parameter("LUCSR", params.p387).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p388 = params.p384;
                validate_finite_parameter("WUCSR", params.p388).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p389 = params.p385;
                validate_finite_parameter("PUCSR", params.p389).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_12: [f64; 10] = [
                0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_12.as_ptr(), (ptr as *mut f64).add(390), 10);
            {
                let params = &mut *ptr;
                params.p400 = params.p390;
                validate_finite_parameter("UCR", params.p400).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p401 = params.p397;
                validate_finite_parameter("LUCR", params.p401).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p402 = params.p398;
                validate_finite_parameter("WUCR", params.p402).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p403 = params.p399;
                validate_finite_parameter("PUCR", params.p403).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_13: [f64; 6] = [
                0.003, 0.0, 1.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_13.as_ptr(), (ptr as *mut f64).add(404), 6);
            {
                let params = &mut *ptr;
                params.p410 = params.p404;
                validate_finite_parameter("PCLMR", params.p410).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p411 = params.p407;
                validate_finite_parameter("LPCLMR", params.p411).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p412 = params.p408;
                validate_finite_parameter("WPCLMR", params.p412).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p413 = params.p409;
                validate_finite_parameter("PPCLMR", params.p413).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_14: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_14.as_ptr(), (ptr as *mut f64).add(414), 1);
            {
                let params = &mut *ptr;
                params.p415 = params.p404;
                validate_finite_parameter("PCLMCV", params.p415).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p416 = params.p405;
                validate_finite_parameter("PCLMCVL", params.p416).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p417 = params.p406;
                validate_parameter("PCLMCVLEXP", params.p417, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p418 = params.p407;
                validate_finite_parameter("LPCLMCV", params.p418).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p419 = params.p408;
                validate_finite_parameter("WPCLMCV", params.p419).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p420 = params.p409;
                validate_finite_parameter("PPCLMCV", params.p420).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_15: [f64; 42] = [
                424000000.0, 0.0, 0.0, 0.0, 1e-8, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                0.0, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_15.as_ptr(), (ptr as *mut f64).add(421), 42);
            {
                let params = &mut *ptr;
                params.p463 = params.p453;
                validate_finite_parameter("RDWMIN", params.p463).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p464 = params.p454;
                validate_finite_parameter("LRDWMIN", params.p464).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p465 = params.p455;
                validate_finite_parameter("WRDWMIN", params.p465).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p466 = params.p456;
                validate_finite_parameter("PRDWMIN", params.p466).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p467 = params.p457;
                validate_finite_parameter("RDW", params.p467).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p468 = params.p458;
                validate_finite_parameter("LRDW", params.p468).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p469 = params.p459;
                validate_finite_parameter("WRDW", params.p469).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p470 = params.p460;
                validate_finite_parameter("PRDW", params.p470).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p471 = params.p461;
                validate_finite_parameter("RDWL", params.p471).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p472 = params.p462;
                validate_parameter("RDWLEXP", params.p472, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_16: [f64; 17] = [
                0.0, 0.0, 0.0, 0.0, 20.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_16.as_ptr(), (ptr as *mut f64).add(473), 17);
            {
                let params = &mut *ptr;
                params.p490 = params.p483;
                validate_finite_parameter("PSATR", params.p490).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p491 = params.p484;
                validate_finite_parameter("LPSATR", params.p491).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p492 = params.p485;
                validate_finite_parameter("WPSATR", params.p492).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p493 = params.p486;
                validate_finite_parameter("PPSATR", params.p493).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_17: [f64; 12] = [
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.05, 0.01, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_17.as_ptr(), (ptr as *mut f64).add(494), 12);
            {
                let params = &mut *ptr;
                params.p506 = params.p498;
                validate_finite_parameter("PTWGR", params.p506).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p507 = params.p499;
                validate_finite_parameter("LPTWGR", params.p507).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p508 = params.p500;
                validate_finite_parameter("WPTWGR", params.p508).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p509 = params.p501;
                validate_finite_parameter("PPTWGR", params.p509).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_18: [f64; 26] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_18.as_ptr(), (ptr as *mut f64).add(510), 26);
            {
                let params = &mut *ptr;
                params.p536 = params.p530;
                validate_finite_parameter("PDIBLCR", params.p536).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p537 = params.p533;
                validate_finite_parameter("LPDIBLCR", params.p537).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p538 = params.p534;
                validate_finite_parameter("WPDIBLCR", params.p538).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p539 = params.p535;
                validate_finite_parameter("PPDIBLCR", params.p539).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_19: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 10.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_19.as_ptr(), (ptr as *mut f64).add(540), 24);
            {
                let params = &mut *ptr;
                params.p564 = params.p563;
                validate_finite_parameter("AHLID", params.p564).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_20: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_20.as_ptr(), (ptr as *mut f64).add(565), 1);
            {
                let params = &mut *ptr;
                params.p566 = params.p565;
                validate_finite_parameter("LAHLID", params.p566).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_21: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_21.as_ptr(), (ptr as *mut f64).add(567), 1);
            {
                let params = &mut *ptr;
                params.p568 = params.p567;
                validate_finite_parameter("WAHLID", params.p568).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_22: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_22.as_ptr(), (ptr as *mut f64).add(569), 1);
            {
                let params = &mut *ptr;
                params.p570 = params.p569;
                validate_finite_parameter("PAHLID", params.p570).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_23: [f64; 12] = [
                1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_23.as_ptr(), (ptr as *mut f64).add(571), 12);
            {
                let params = &mut *ptr;
                params.p583 = params.p579;
                validate_finite_parameter("IDBJT", params.p583).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p584 = params.p582;
                validate_finite_parameter("LIDBJT", params.p584).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p585 = params.p581;
                validate_finite_parameter("WIDBJT", params.p585).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p586 = params.p580;
                validate_finite_parameter("PIDBJT", params.p586).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_24: [f64; 120] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2e-7,
                2e-6, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0,
                0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 10000000.0, 0.0, 0.0, 0.0,
                0.1, 0.1, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.4, 0.0, 0.0, 0.0, 0.026, 0.35, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.03, 0.0, 0.0,
                0.0, 0.43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.05, 0.0, 0.0, 0.0, 17.0, 300.0, 3.7622e-7,
                -31051000000.0, 4.9758e-7, -23570000000.0, 3.4254e-7, 4.9723e-7, 1166500000000.0, 745670000000.0, 1.1,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_24.as_ptr(), (ptr as *mut f64).add(587), 120);
            {
                let params = &mut *ptr;
                params.p707 = if (params.p30 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGC", params.p707).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p708 = if (params.p30 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGC", params.p708).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p709 = if (params.p30 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGC", params.p709).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p710 = if (params.p30 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGS", params.p710).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_25: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_25.as_ptr(), (ptr as *mut f64).add(711), 1);
            {
                let params = &mut *ptr;
                params.p712 = if (params.p30 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGS", params.p712).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p713 = if (params.p30 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGS", params.p713).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p714 = if (params.p30 == 1.0) { 0.0136 } else { 0.0098 };
                validate_finite_parameter("AIGD", params.p714).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_26: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_26.as_ptr(), (ptr as *mut f64).add(715), 1);
            {
                let params = &mut *ptr;
                params.p716 = if (params.p30 == 1.0) { 0.00171 } else { 0.000759 };
                validate_finite_parameter("BIGD", params.p716).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p717 = if (params.p30 == 1.0) { 0.075 } else { 0.03 };
                validate_finite_parameter("CIGD", params.p717).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p718 = params.p54;
                validate_finite_parameter("DLCIG", params.p718).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p719 = params.p718;
                validate_finite_parameter("DLCIGD", params.p719).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_27: [f64; 101] = [
                1.0, 1.0, 3e-9, 1.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.043,
                0.0, 0.0054, 0.0075, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 2300000000.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0,
                0.0, 0.8, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_27.as_ptr(), (ptr as *mut f64).add(720), 101);
            {
                let params = &mut *ptr;
                params.p821 = params.p799;
                validate_finite_parameter("AGISL", params.p821).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p822 = params.p800;
                validate_finite_parameter("AGISLL", params.p822).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p823 = params.p801;
                validate_finite_parameter("AGISLW", params.p823).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p824 = params.p802;
                validate_finite_parameter("LAGISL", params.p824).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p825 = params.p803;
                validate_finite_parameter("WAGISL", params.p825).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p826 = params.p804;
                validate_finite_parameter("PAGISL", params.p826).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p827 = params.p805;
                validate_finite_parameter("BGISL", params.p827).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p828 = params.p806;
                validate_finite_parameter("BGISL1", params.p828).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p829 = params.p807;
                validate_finite_parameter("LBGISL", params.p829).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p830 = params.p808;
                validate_finite_parameter("WBGISL", params.p830).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p831 = params.p809;
                validate_finite_parameter("PBGISL", params.p831).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p832 = params.p810;
                validate_finite_parameter("LBGISL1", params.p832).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p833 = params.p811;
                validate_finite_parameter("WBGISL1", params.p833).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p834 = params.p812;
                validate_finite_parameter("PBGISL1", params.p834).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p835 = params.p813;
                validate_finite_parameter("CGISL", params.p835).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p836 = params.p814;
                validate_finite_parameter("LCGISL", params.p836).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p837 = params.p815;
                validate_finite_parameter("WCGISL", params.p837).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p838 = params.p816;
                validate_finite_parameter("PCGISL", params.p838).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p839 = params.p817;
                validate_finite_parameter("EGISL", params.p839).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p840 = params.p818;
                validate_finite_parameter("LEGISL", params.p840).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p841 = params.p819;
                validate_finite_parameter("WEGISL", params.p841).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p842 = params.p820;
                validate_finite_parameter("PEGISL", params.p842).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_28: [f64; 12] = [
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_28.as_ptr(), (ptr as *mut f64).add(843), 12);
            {
                let params = &mut *ptr;
                params.p855 = params.p843;
                validate_finite_parameter("RGISL", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p856 = params.p844;
                validate_finite_parameter("LRGISL", params.p856).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p857 = params.p845;
                validate_finite_parameter("WRGISL", params.p857).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p858 = params.p846;
                validate_finite_parameter("PRGISL", params.p858).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p859 = params.p847;
                validate_finite_parameter("KGISL", params.p859).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p860 = params.p848;
                validate_finite_parameter("LKGISL", params.p860).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p861 = params.p849;
                validate_finite_parameter("WKGISL", params.p861).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p862 = params.p850;
                validate_finite_parameter("PKGISL", params.p862).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p863 = params.p851;
                validate_finite_parameter("FGISL", params.p863).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p864 = params.p852;
                validate_finite_parameter("LFGISL", params.p864).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p865 = params.p853;
                validate_finite_parameter("WFGISL", params.p865).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p866 = params.p854;
                validate_finite_parameter("PFGISL", params.p866).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_29: [f64; 29] = [
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.6, 0.0, 0.0, 0.0, 0.6, 0.0, 0.0, 0.0,
                1000000.0, 1.0, 1000000.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_29.as_ptr(), (ptr as *mut f64).add(867), 29);
            {
                let params = &mut *ptr;
                params.p896 = params.p895;
                validate_parameter("DMCI", params.p896, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_30: [f64; 5] = [
                0.0, 0.0, 0.0, 0.1, 0.0005,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_30.as_ptr(), (ptr as *mut f64).add(897), 5);
            {
                let params = &mut *ptr;
                params.p902 = params.p901;
                validate_finite_parameter("CJD", params.p902).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_31: [f64; 1] = [
                5e-10,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_31.as_ptr(), (ptr as *mut f64).add(903), 1);
            {
                let params = &mut *ptr;
                params.p904 = params.p903;
                validate_finite_parameter("CJSWD", params.p904).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_32: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_32.as_ptr(), (ptr as *mut f64).add(905), 1);
            {
                let params = &mut *ptr;
                params.p906 = params.p905;
                validate_finite_parameter("CJSWGD", params.p906).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_33: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_33.as_ptr(), (ptr as *mut f64).add(907), 1);
            {
                let params = &mut *ptr;
                params.p908 = params.p907;
                validate_finite_parameter("PBD", params.p908).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_34: [f64; 1] = [
                1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_34.as_ptr(), (ptr as *mut f64).add(909), 1);
            {
                let params = &mut *ptr;
                params.p910 = params.p909;
                validate_finite_parameter("PBSWD", params.p910).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p911 = params.p909;
                validate_finite_parameter("PBSWGS", params.p911).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p912 = params.p911;
                validate_finite_parameter("PBSWGD", params.p912).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_35: [f64; 1] = [
                0.5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_35.as_ptr(), (ptr as *mut f64).add(913), 1);
            {
                let params = &mut *ptr;
                params.p914 = params.p913;
                validate_finite_parameter("MJD", params.p914).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_36: [f64; 1] = [
                0.33,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_36.as_ptr(), (ptr as *mut f64).add(915), 1);
            {
                let params = &mut *ptr;
                params.p916 = params.p915;
                validate_finite_parameter("MJSWD", params.p916).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p917 = params.p915;
                validate_finite_parameter("MJSWGS", params.p917).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p918 = params.p917;
                validate_finite_parameter("MJSWGD", params.p918).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_37: [f64; 8] = [
                1e-12, 1.0, -1.0, 0.0, 0.0, 0.0, 0.026, 1.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_37.as_ptr(), (ptr as *mut f64).add(919), 8);
            {
                let params = &mut *ptr;
                params.p927 = params.p70;
                validate_finite_parameter("DWJ", params.p927).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p928 = params.p571;
                validate_finite_parameter("XDIF", params.p928).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_38: [f64; 4] = [
                0.0, 0.0, 0.0, 1e-7,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_38.as_ptr(), (ptr as *mut f64).add(929), 4);
            {
                let params = &mut *ptr;
                params.p933 = params.p932;
                validate_finite_parameter("IDDIF", params.p933).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_39: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_39.as_ptr(), (ptr as *mut f64).add(934), 1);
            {
                let params = &mut *ptr;
                params.p935 = params.p934;
                validate_finite_parameter("LIDDIF", params.p935).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_40: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_40.as_ptr(), (ptr as *mut f64).add(936), 1);
            {
                let params = &mut *ptr;
                params.p937 = params.p936;
                validate_finite_parameter("WIDDIF", params.p937).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_41: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_41.as_ptr(), (ptr as *mut f64).add(938), 1);
            {
                let params = &mut *ptr;
                params.p939 = params.p938;
                validate_finite_parameter("PIDDIF", params.p939).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_42: [f64; 13] = [
                2.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 1e-5,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_42.as_ptr(), (ptr as *mut f64).add(940), 13);
            {
                let params = &mut *ptr;
                params.p953 = params.p952;
                validate_finite_parameter("IDREC", params.p953).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_43: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_43.as_ptr(), (ptr as *mut f64).add(954), 1);
            {
                let params = &mut *ptr;
                params.p955 = params.p954;
                validate_finite_parameter("LIDREC", params.p955).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_44: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_44.as_ptr(), (ptr as *mut f64).add(956), 1);
            {
                let params = &mut *ptr;
                params.p957 = params.p956;
                validate_finite_parameter("WIDREC", params.p957).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_45: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_45.as_ptr(), (ptr as *mut f64).add(958), 1);
            {
                let params = &mut *ptr;
                params.p959 = params.p958;
                validate_finite_parameter("PIDREC", params.p959).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_46: [f64; 9] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_46.as_ptr(), (ptr as *mut f64).add(960), 9);
            {
                let params = &mut *ptr;
                params.p969 = params.p968;
                validate_finite_parameter("IDTUN", params.p969).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_47: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_47.as_ptr(), (ptr as *mut f64).add(970), 1);
            {
                let params = &mut *ptr;
                params.p971 = params.p970;
                validate_finite_parameter("LIDTUN", params.p971).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_48: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_48.as_ptr(), (ptr as *mut f64).add(972), 1);
            {
                let params = &mut *ptr;
                params.p973 = params.p972;
                validate_finite_parameter("WIDTUN", params.p973).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_49: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_49.as_ptr(), (ptr as *mut f64).add(974), 1);
            {
                let params = &mut *ptr;
                params.p975 = params.p974;
                validate_finite_parameter("PIDTUN", params.p975).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_50: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_50.as_ptr(), (ptr as *mut f64).add(976), 1);
            {
                let params = &mut *ptr;
                params.p977 = params.p976;
                validate_finite_parameter("XTUND", params.p977).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_51: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_51.as_ptr(), (ptr as *mut f64).add(978), 1);
            {
                let params = &mut *ptr;
                params.p979 = params.p978;
                validate_finite_parameter("LXTUND", params.p979).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_52: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_52.as_ptr(), (ptr as *mut f64).add(980), 1);
            {
                let params = &mut *ptr;
                params.p981 = params.p980;
                validate_finite_parameter("WXTUND", params.p981).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_53: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_53.as_ptr(), (ptr as *mut f64).add(982), 1);
            {
                let params = &mut *ptr;
                params.p983 = params.p982;
                validate_finite_parameter("PXTUND", params.p983).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_54: [f64; 1] = [
                10.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_54.as_ptr(), (ptr as *mut f64).add(984), 1);
            {
                let params = &mut *ptr;
                params.p985 = params.p984;
                validate_finite_parameter("NTUND", params.p985).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_55: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_55.as_ptr(), (ptr as *mut f64).add(986), 1);
            {
                let params = &mut *ptr;
                params.p987 = params.p986;
                validate_finite_parameter("LNTUND", params.p987).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_56: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_56.as_ptr(), (ptr as *mut f64).add(988), 1);
            {
                let params = &mut *ptr;
                params.p989 = params.p988;
                validate_finite_parameter("WNTUND", params.p989).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_57: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_57.as_ptr(), (ptr as *mut f64).add(990), 1);
            {
                let params = &mut *ptr;
                params.p991 = params.p990;
                validate_finite_parameter("PNTUND", params.p991).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_58: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_58.as_ptr(), (ptr as *mut f64).add(992), 1);
            {
                let params = &mut *ptr;
                params.p993 = params.p992;
                validate_finite_parameter("VTUN0D", params.p993).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_59: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_59.as_ptr(), (ptr as *mut f64).add(994), 1);
            {
                let params = &mut *ptr;
                params.p995 = params.p994;
                validate_finite_parameter("LVTUN0D", params.p995).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_60: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_60.as_ptr(), (ptr as *mut f64).add(996), 1);
            {
                let params = &mut *ptr;
                params.p997 = params.p996;
                validate_finite_parameter("WVTUN0D", params.p997).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_61: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_61.as_ptr(), (ptr as *mut f64).add(998), 1);
            {
                let params = &mut *ptr;
                params.p999 = params.p998;
                validate_finite_parameter("PVTUN0D", params.p999).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_62: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_62.as_ptr(), (ptr as *mut f64).add(1000), 1);
            {
                let params = &mut *ptr;
                params.p1001 = params.p1000;
                validate_finite_parameter("VREC0D", params.p1001).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_63: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_63.as_ptr(), (ptr as *mut f64).add(1002), 1);
            {
                let params = &mut *ptr;
                params.p1003 = params.p1002;
                validate_finite_parameter("LVREC0D", params.p1003).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_64: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_64.as_ptr(), (ptr as *mut f64).add(1004), 1);
            {
                let params = &mut *ptr;
                params.p1005 = params.p1004;
                validate_finite_parameter("WVREC0D", params.p1005).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_65: [f64; 1] = [
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_65.as_ptr(), (ptr as *mut f64).add(1006), 1);
            {
                let params = &mut *ptr;
                params.p1007 = params.p1006;
                validate_finite_parameter("PVREC0D", params.p1007).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_66: [f64; 313] = [
                12.0, 1.0, 1.0, 41000000.0, 6.25e40, 3.125e25, 875000000.0, 0.0,
                0.0, 1.0, 1.0, 0.577, 0.5164, 0.395, 1.5, 3.5,
                0.0, 1.0, 0.0, 0.0, 27.0, 0.000473, 636.0, 0.0,
                -1.5, 0.0, 0.0, 0.0, 0.0, 0.001, 0.0, 0.0,
                0.0, 0.0, 5.6e-11, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -0.004775,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                -0.00156, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, -0.11, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.022, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 2.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-5, 0.0, 1e-6, 1e-6,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 1e-6, 400.0, 336000000.0, 0.185, 0.3, 1.4, 0.0,
                0.49, 1.42, 20.0, 1e-8, 0.0, 0.0, 1.0, 0.0,
                1e24, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                1e-9, 0.0, 0.0, 0.0, 1e-9, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.08, 0.0,
                0.0, 0.0, -0.07, 0.0, 0.0, 0.0, -0.11, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.022, 0.0,
                0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.2, 0.53,
                0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
                1e-5, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
                1e-8,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_66.as_ptr(), (ptr as *mut f64).add(1008), 313);
            {
                let params = &mut *ptr;
                params.p1321 = params.p1012;
                validate_finite_parameter("NOIA2", params.p1321).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1322 = params.p79;
                validate_parameter("HNDEP", params.p1322, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_67: [f64; 24] = [
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_67.as_ptr(), (ptr as *mut f64).add(1323), 24);
            {
                let params = &mut *ptr;
                params.p1347 = 0.001;
                validate_parameter("minr", params.p1347, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_68: [f64; 5] = [
                1.0, 0.0, 0.0, 1.0, 0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_68.as_ptr(), (ptr as *mut f64).add(1348), 5);
            {
                let params = &mut *ptr;
                params.p1353 = params.p1349;
                validate_finite_parameter("A0CV", params.p1353).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1354 = params.p1350;
                validate_finite_parameter("AGSCV", params.p1354).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1355 = params.p1352;
                validate_parameter("KETACV", params.p1355, false, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_69: [f64; 25] = [
                0.0, 1.0, 0.0, 1.0, 1000000000000000.0, 0.067, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0, 0.0, 5e24, 0.0, 0.0,
                0.0, 1.0, 0.001, 0.0, 0.0, 0.0, 0.0, 0.0,
                2e-12,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_69.as_ptr(), (ptr as *mut f64).add(1356), 25);
            {
                let params = &mut *ptr;
                params.p1381 = params.p1379;
                validate_finite_parameter("AGBCPD", params.p1381).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            const DEFAULTS_70: [f64; 17] = [
                0.0, 1.12, 6e22, 0.0, 0.0, 0.0, 1.0, 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.0,
                0.0,
            ];
            std::ptr::copy_nonoverlapping(DEFAULTS_70.as_ptr(), (ptr as *mut f64).add(1382), 17);
            {
                let params = &mut *ptr;
                params.p1399 = params.p1397;
                validate_parameter("ACEDB", params.p1399, false, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            }
            {
                let params = &mut *ptr;
                params.p1400 = params.p1398;
                validate_finite_parameter("BCEDB", params.p1400).expect("generated Verilog-A parameter default must satisfy declared range");
            }
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

const PARAMETER_NAME_LOOKUP: [(&str, usize); 1401] = [
    ("l", 0), ("w", 1), ("nf", 2), ("nrs", 3), ("nrd", 4), ("vfbsdoff", 5), ("minz", 6), ("rgatemod", 7), ("geomod", 8), ("rgeomod", 9), ("sa", 10), ("sb", 11), ("sd", 12), ("sca", 13), ("scb", 14), ("scc", 15),
    ("sc", 16), ("as", 17), ("ad", 18), ("ps", 19), ("pd", 20), ("xgw", 21), ("ngcon", 22), ("dtemp", 23), ("mulu0", 24), ("delvto", 25), ("ids0mult", 26), ("edgefet", 27), ("sslmod", 28), ("soimod", 29), ("type", 30), ("cvmod", 31),
    ("covmod", 32), ("rdsmod", 33), ("wpemod", 34), ("asymmod", 35), ("gidlmod", 36), ("igcmod", 37), ("igbmod", 38), ("tnoimod", 39), ("tnodeout", 40), ("shmod", 41), ("mobscale", 42), ("bodymod", 43), ("iiimod", 44), ("modagbcp2", 45), ("pdemod", 46), ("fbody1", 47),
    ("llong", 48), ("lmlt", 49), ("wmlt", 50), ("xl", 51), ("wwide", 52), ("xw", 53), ("lint", 54), ("ll", 55), ("lw", 56), ("lwl", 57), ("lln", 58), ("lwn", 59), ("wint", 60), ("wl", 61), ("ww", 62), ("wwl", 63),
    ("wln", 64), ("wwn", 65), ("dlc", 66), ("llc", 67), ("lwc", 68), ("lwlc", 69), ("dwc", 70), ("wlc", 71), ("wwc", 72), ("wwlc", 73), ("tsi", 74), ("tbox", 75), ("toxe", 76), ("toxp", 77), ("dtox", 78), ("ndep", 79),
    ("ndepl1", 80), ("ndeplexp1", 81), ("ndepl2", 82), ("ndeplexp2", 83), ("ndepw", 84), ("ndepwexp", 85), ("ndepwl", 86), ("ndepwlexp", 87), ("lndep", 88), ("wndep", 89), ("pndep", 90), ("ndepcv", 91), ("ndepcvl1", 92), ("ndepcvlexp1", 93), ("ndepcvl2", 94), ("ndepcvlexp2", 95),
    ("ndepcvw", 96), ("ndepcvwexp", 97), ("ndepcvwl", 98), ("ndepcvwlexp", 99), ("lndepcv", 100), ("wndepcv", 101), ("pndepcv", 102), ("ngate", 103), ("lngate", 104), ("wngate", 105), ("pngate", 106), ("ni0sub", 107), ("bg0sub", 108), ("epsrsub", 109), ("epsrox", 110), ("xj", 111),
    ("lxj", 112), ("wxj", 113), ("pxj", 114), ("vfb", 115), ("lvfb", 116), ("wvfb", 117), ("pvfb", 118), ("vfbb", 119), ("lvfbb", 120), ("wvfbb", 121), ("pvfbb", 122), ("vfbl", 123), ("vfblexp", 124), ("vfbw", 125), ("vfbwexp", 126), ("vfbwl", 127),
    ("vfbwlexp", 128), ("vfbcv", 129), ("lvfbcv", 130), ("wvfbcv", 131), ("pvfbcv", 132), ("vfbcvl", 133), ("vfbcvlexp", 134), ("vfbcvw", 135), ("vfbcvwexp", 136), ("vfbcvwl", 137), ("vfbcvwlexp", 138), ("delvfbacc", 139), ("vfbagbcp2", 140), ("ndepagbcp2", 141), ("nsd", 142), ("lnsd", 143),
    ("wnsd", 144), ("pnsd", 145), ("dvtp0", 146), ("ldvtp0", 147), ("wdvtp0", 148), ("pdvtp0", 149), ("dvtp1", 150), ("ldvtp1", 151), ("wdvtp1", 152), ("pdvtp1", 153), ("dvtp2", 154), ("ldvtp2", 155), ("wdvtp2", 156), ("pdvtp2", 157), ("dvtp3", 158), ("ldvtp3", 159),
    ("wdvtp3", 160), ("pdvtp3", 161), ("dvtp4", 162), ("ldvtp4", 163), ("wdvtp4", 164), ("pdvtp4", 165), ("dvtp5", 166), ("ldvtp5", 167), ("wdvtp5", 168), ("pdvtp5", 169), ("dvbd0", 170), ("ldvbd0", 171), ("wdvbd0", 172), ("pdvbd0", 173), ("dvbd1", 174), ("ldvbd1", 175),
    ("wdvbd1", 176), ("pdvbd1", 177), ("vsce", 178), ("lvsce", 179), ("wvsce", 180), ("pvsce", 181), ("cdsbs1", 182), ("lcdsbs1", 183), ("wcdsbs1", 184), ("pcdsbs1", 185), ("cdsbs", 186), ("lcdsbs", 187), ("wcdsbs", 188), ("pcdsbs", 189), ("phin", 190), ("lphin", 191),
    ("wphin", 192), ("pphin", 193), ("eta0", 194), ("leta0", 195), ("weta0", 196), ("peta0", 197), ("eta0r", 198), ("leta0r", 199), ("weta0r", 200), ("peta0r", 201), ("dsub", 202), ("etab", 203), ("etabexp", 204), ("letab", 205), ("wetab", 206), ("petab", 207),
    ("k1", 208), ("k1l", 209), ("k1lexp", 210), ("k1w", 211), ("k1wexp", 212), ("k1wl", 213), ("k1wlexp", 214), ("lk1", 215), ("wk1", 216), ("pk1", 217), ("k2", 218), ("k2l", 219), ("k2lexp", 220), ("k2w", 221), ("k2wexp", 222), ("k2wl", 223),
    ("k2wlexp", 224), ("lk2", 225), ("wk2", 226), ("pk2", 227), ("ados", 228), ("bdos", 229), ("qm0", 230), ("etaqm", 231), ("cit", 232), ("lcit", 233), ("wcit", 234), ("pcit", 235), ("nfactor", 236), ("nfactorl", 237), ("nfactorlexp", 238), ("nfactorw", 239),
    ("nfactorwexp", 240), ("nfactorwl", 241), ("nfactorwlexp", 242), ("lnfactor", 243), ("wnfactor", 244), ("pnfactor", 245), ("ascl", 246), ("lascl", 247), ("wascl", 248), ("pascl", 249), ("bscl", 250), ("lbscl", 251), ("wbscl", 252), ("pbscl", 253), ("dvt1", 254), ("ldvt1", 255),
    ("wdvt1", 256), ("pdvt1", 257), ("cdscd", 258), ("lcdscd", 259), ("wcdscd", 260), ("pcdscd", 261), ("cdsc", 262), ("lcdsc", 263), ("wcdsc", 264), ("pcdsc", 265), ("csecsed", 266), ("cbcbd", 267), ("csecse0", 268), ("csecse0p", 269), ("csecse", 270), ("lcsecse", 271),
    ("wcsecse", 272), ("pcsecse", 273), ("csecsep", 274), ("cbcb", 275), ("lcbcb", 276), ("wcbcb", 277), ("pcbcb", 278), ("cbcbp", 279), ("cbcb0", 280), ("cbcb0p", 281), ("cdscdl", 282), ("cdscdlexp", 283), ("cdscdr", 284), ("lcdscdr", 285), ("wcdscdr", 286), ("pcdscdr", 287),
    ("cdscb", 288), ("cdscbl", 289), ("cdscblexp", 290), ("lcdscb", 291), ("wcdscb", 292), ("pcdscb", 293), ("vbsa", 294), ("vsat", 295), ("lvsat", 296), ("wvsat", 297), ("pvsat", 298), ("vsatl", 299), ("vsatlexp", 300), ("vsatw", 301), ("vsatwexp", 302), ("vsatwl", 303),
    ("vsatwlexp", 304), ("vsatr", 305), ("lvsatr", 306), ("wvsatr", 307), ("pvsatr", 308), ("delta", 309), ("ldelta", 310), ("wdelta", 311), ("pdelta", 312), ("deltal", 313), ("deltalexp", 314), ("vsatcv", 315), ("lvsatcv", 316), ("wvsatcv", 317), ("pvsatcv", 318), ("vsatcvl", 319),
    ("vsatcvlexp", 320), ("vsatcvw", 321), ("vsatcvwexp", 322), ("vsatcvwl", 323), ("vsatcvwlexp", 324), ("thesat", 325), ("lthesat", 326), ("wthesat", 327), ("pthesat", 328), ("lpe1", 329), ("llpe1", 330), ("wlpe1", 331), ("plpe1", 332), ("up1", 333), ("lp1", 334), ("up2", 335),
    ("lp2", 336), ("u0", 337), ("u0l", 338), ("u0lexp", 339), ("lu0", 340), ("wu0", 341), ("pu0", 342), ("u0r", 343), ("lu0r", 344), ("wu0r", 345), ("pu0r", 346), ("etamob", 347), ("ua", 348), ("ual", 349), ("ualexp", 350), ("uaw", 351),
    ("uawexp", 352), ("uawl", 353), ("uawlexp", 354), ("lua", 355), ("wua", 356), ("pua", 357), ("uar", 358), ("luar", 359), ("wuar", 360), ("puar", 361), ("eu", 362), ("leu", 363), ("weu", 364), ("peu", 365), ("eul", 366), ("eulexp", 367),
    ("euw", 368), ("euwexp", 369), ("euwl", 370), ("euwlexp", 371), ("ud", 372), ("udl", 373), ("udlexp", 374), ("lud", 375), ("wud", 376), ("pud", 377), ("udr", 378), ("ludr", 379), ("wudr", 380), ("pudr", 381), ("ucs", 382), ("lucs", 383),
    ("wucs", 384), ("pucs", 385), ("ucsr", 386), ("lucsr", 387), ("wucsr", 388), ("pucsr", 389), ("uc", 390), ("ucl", 391), ("uclexp", 392), ("ucw", 393), ("ucwexp", 394), ("ucwl", 395), ("ucwlexp", 396), ("luc", 397), ("wuc", 398), ("puc", 399),
    ("ucr", 400), ("lucr", 401), ("wucr", 402), ("pucr", 403), ("pclm", 404), ("pclml", 405), ("pclmlexp", 406), ("lpclm", 407), ("wpclm", 408), ("ppclm", 409), ("pclmr", 410), ("lpclmr", 411), ("wpclmr", 412), ("ppclmr", 413), ("pclmg", 414), ("pclmcv", 415),
    ("pclmcvl", 416), ("pclmcvlexp", 417), ("lpclmcv", 418), ("wpclmcv", 419), ("ppclmcv", 420), ("pscbe1", 421), ("lpscbe1", 422), ("wpscbe1", 423), ("ppscbe1", 424), ("pscbe2", 425), ("lpscbe2", 426), ("wpscbe2", 427), ("ppscbe2", 428), ("pdits", 429), ("lpdits", 430), ("wpdits", 431),
    ("ppdits", 432), ("pditsl", 433), ("pditsd", 434), ("lpditsd", 435), ("wpditsd", 436), ("ppditsd", 437), ("rsh", 438), ("prwg", 439), ("lprwg", 440), ("wprwg", 441), ("pprwg", 442), ("prwb", 443), ("lprwb", 444), ("wprwb", 445), ("pprwb", 446), ("prwbl", 447),
    ("prwblexp", 448), ("wr", 449), ("lwr", 450), ("wwr", 451), ("pwr", 452), ("rswmin", 453), ("lrswmin", 454), ("wrswmin", 455), ("prswmin", 456), ("rsw", 457), ("lrsw", 458), ("wrsw", 459), ("prsw", 460), ("rswl", 461), ("rswlexp", 462), ("rdwmin", 463),
    ("lrdwmin", 464), ("wrdwmin", 465), ("prdwmin", 466), ("rdw", 467), ("lrdw", 468), ("wrdw", 469), ("prdw", 470), ("rdwl", 471), ("rdwlexp", 472), ("rdswmin", 473), ("lrdswmin", 474), ("wrdswmin", 475), ("prdswmin", 476), ("rdsw", 477), ("rdswl", 478), ("rdswlexp", 479),
    ("lrdsw", 480), ("wrdsw", 481), ("prdsw", 482), ("psat", 483), ("lpsat", 484), ("wpsat", 485), ("ppsat", 486), ("psatl", 487), ("psatlexp", 488), ("psatb", 489), ("psatr", 490), ("lpsatr", 491), ("wpsatr", 492), ("ppsatr", 493), ("lpsatb", 494), ("wpsatb", 495),
    ("ppsatb", 496), ("psatx", 497), ("ptwg", 498), ("lptwg", 499), ("wptwg", 500), ("pptwg", 501), ("ptwgl", 502), ("vp", 503), ("alp", 504), ("ptwglexp", 505), ("ptwgr", 506), ("lptwgr", 507), ("wptwgr", 508), ("pptwgr", 509), ("ksativ", 510), ("lksativ", 511),
    ("wksativ", 512), ("pksativ", 513), ("a1", 514), ("la1", 515), ("wa1", 516), ("pa1", 517), ("a11", 518), ("la11", 519), ("wa11", 520), ("pa11", 521), ("a2", 522), ("la2", 523), ("wa2", 524), ("pa2", 525), ("a21", 526), ("la21", 527),
    ("wa21", 528), ("pa21", 529), ("pdiblc", 530), ("pdiblcl", 531), ("pdiblclexp", 532), ("lpdiblc", 533), ("wpdiblc", 534), ("ppdiblc", 535), ("pdiblcr", 536), ("lpdiblcr", 537), ("wpdiblcr", 538), ("ppdiblcr", 539), ("pdiblcb", 540), ("lpdiblcb", 541), ("wpdiblcb", 542), ("ppdiblcb", 543),
    ("pvag", 544), ("lpvag", 545), ("wpvag", 546), ("ppvag", 547), ("fprout", 548), ("fproutl", 549), ("fproutlexp", 550), ("lfprout", 551), ("wfprout", 552), ("pfprout", 553), ("bjtoff", 554), ("vabjt", 555), ("lvabjt", 556), ("wvabjt", 557), ("pvabjt", 558), ("aely", 559),
    ("laely", 560), ("waely", 561), ("paely", 562), ("ahli", 563), ("ahlid", 564), ("lahli", 565), ("lahlid", 566), ("wahli", 567), ("wahlid", 568), ("pahli", 569), ("pahlid", 570), ("xbjt", 571), ("lxbjt", 572), ("wxbjt", 573), ("pxbjt", 574), ("ndiode", 575),
    ("lndiode", 576), ("wndiode", 577), ("pndiode", 578), ("isbjt", 579), ("pisbjt", 580), ("wisbjt", 581), ("lisbjt", 582), ("idbjt", 583), ("lidbjt", 584), ("widbjt", 585), ("pidbjt", 586), ("nbjt", 587), ("lnbjt", 588), ("llbjt0", 589), ("wnbjt", 590), ("wlbjt0", 591),
    ("pnbjt", 592), ("plbjt0", 593), ("lbjt0", 594), ("ln", 595), ("vdsatii0", 596), ("lvdsatii0", 597), ("wvdsatii0", 598), ("pvdsatii0", 599), ("tii", 600), ("alpha0", 601), ("alpha0l", 602), ("alpha0lexp", 603), ("lalpha0", 604), ("walpha0", 605), ("palpha0", 606), ("beta0", 607),
    ("lbeta0", 608), ("wbeta0", 609), ("pbeta0", 610), ("beta1", 611), ("lbeta1", 612), ("wbeta1", 613), ("pbeta1", 614), ("beta2", 615), ("lbeta2", 616), ("wbeta2", 617), ("pbeta2", 618), ("lii", 619), ("llii", 620), ("wlii", 621), ("plii", 622), ("sii0", 623),
    ("lsii0", 624), ("wsii0", 625), ("psii0", 626), ("sii1", 627), ("lsii1", 628), ("wsii1", 629), ("psii1", 630), ("sii2", 631), ("lsii2", 632), ("wsii2", 633), ("psii2", 634), ("siid", 635), ("lsiid", 636), ("wsiid", 637), ("psiid", 638), ("esatii", 639),
    ("lesatii", 640), ("wesatii", 641), ("pesatii", 642), ("iimod2clamp1", 643), ("iimod2clamp2", 644), ("iimod2clamp3", 645), ("fbjtii", 646), ("lfbjtii", 647), ("wfbjtii", 648), ("pfbjtii", 649), ("ebjtii", 650), ("cbjtii", 651), ("abjtii", 652), ("labjtii", 653), ("lcbjtii", 654), ("lebjtii", 655),
    ("wabjtii", 656), ("wcbjtii", 657), ("webjtii", 658), ("pabjtii", 659), ("pcbjtii", 660), ("pebjtii", 661), ("vbci", 662), ("lvbci", 663), ("wvbci", 664), ("pvbci", 665), ("tvbci", 666), ("mbjtii", 667), ("lmbjtii", 668), ("wmbjtii", 669), ("pmbjtii", 670), ("vecb", 671),
    ("alphagb1", 672), ("lalphagb1", 673), ("walphagb1", 674), ("palphagb1", 675), ("alphagb1_t", 676), ("lalphagb1_t", 677), ("walphagb1_t", 678), ("palphagb1_t", 679), ("betagb1", 680), ("lbetagb1", 681), ("wbetagb1", 682), ("pbetagb1", 683), ("alphagb2", 684), ("lalphagb2", 685), ("walphagb2", 686), ("palphagb2", 687),
    ("alphagb2_t", 688), ("lalphagb2_t", 689), ("walphagb2_t", 690), ("palphagb2_t", 691), ("betagb2", 692), ("lbetagb2", 693), ("wbetagb2", 694), ("pbetagb2", 695), ("vgb2", 696), ("vgb1", 697), ("agb1", 698), ("bgb1", 699), ("agb2", 700), ("bgb2", 701), ("agbc2n", 702), ("agbc2p", 703),
    ("bgbc2n", 704), ("bgbc2p", 705), ("eigbinv", 706), ("aigc", 707), ("bigc", 708), ("cigc", 709), ("aigs", 710), ("aigs1", 711), ("bigs", 712), ("cigs", 713), ("aigd", 714), ("aigd1", 715), ("bigd", 716), ("cigd", 717), ("dlcig", 718), ("dlcigd", 719),
    ("poxedge", 720), ("ntox", 721), ("toxref", 722), ("pigcd", 723), ("aigcl", 724), ("aigcw", 725), ("aigc1", 726), ("aigsl", 727), ("aigsw", 728), ("aigdl", 729), ("aigdw", 730), ("pigcdl", 731), ("leigbinv", 732), ("weigbinv", 733), ("peigbinv", 734), ("laigc", 735),
    ("laigc1", 736), ("waigc", 737), ("waigc1", 738), ("paigc", 739), ("paigc1", 740), ("lbigc", 741), ("wbigc", 742), ("pbigc", 743), ("lcigc", 744), ("wcigc", 745), ("pcigc", 746), ("laigs", 747), ("laigs1", 748), ("waigs", 749), ("waigs1", 750), ("paigs", 751),
    ("paigs1", 752), ("lbigs", 753), ("wbigs", 754), ("pbigs", 755), ("lcigs", 756), ("wcigs", 757), ("pcigs", 758), ("laigd", 759), ("laigd1", 760), ("waigd", 761), ("waigd1", 762), ("paigd", 763), ("paigd1", 764), ("lbigd", 765), ("wbigd", 766), ("pbigd", 767),
    ("lcigd", 768), ("wcigd", 769), ("pcigd", 770), ("lpoxedge", 771), ("wpoxedge", 772), ("ppoxedge", 773), ("ldlcig", 774), ("wdlcig", 775), ("pdlcig", 776), ("ldlcigd", 777), ("wdlcigd", 778), ("pdlcigd", 779), ("lntox", 780), ("wntox", 781), ("pntox", 782), ("aigbcp2", 783),
    ("aigbcp2_t", 784), ("bigbcp2", 785), ("cigbcp2", 786), ("laigbcp2", 787), ("laigbcp2_t", 788), ("lbigbcp2", 789), ("lcigbcp2", 790), ("waigbcp2", 791), ("waigbcp2_t", 792), ("wbigbcp2", 793), ("wcigbcp2", 794), ("paigbcp2", 795), ("paigbcp2_t", 796), ("pbigbcp2", 797), ("pcigbcp2", 798), ("agidl", 799),
    ("agidll", 800), ("agidlw", 801), ("lagidl", 802), ("wagidl", 803), ("pagidl", 804), ("bgidl", 805), ("bgidl1", 806), ("lbgidl", 807), ("wbgidl", 808), ("pbgidl", 809), ("lbgidl1", 810), ("wbgidl1", 811), ("pbgidl1", 812), ("cgidl", 813), ("lcgidl", 814), ("wcgidl", 815),
    ("pcgidl", 816), ("egidl", 817), ("legidl", 818), ("wegidl", 819), ("pegidl", 820), ("agisl", 821), ("agisll", 822), ("agislw", 823), ("lagisl", 824), ("wagisl", 825), ("pagisl", 826), ("bgisl", 827), ("bgisl1", 828), ("lbgisl", 829), ("wbgisl", 830), ("pbgisl", 831),
    ("lbgisl1", 832), ("wbgisl1", 833), ("pbgisl1", 834), ("cgisl", 835), ("lcgisl", 836), ("wcgisl", 837), ("pcgisl", 838), ("egisl", 839), ("legisl", 840), ("wegisl", 841), ("pegisl", 842), ("rgidl", 843), ("lrgidl", 844), ("wrgidl", 845), ("prgidl", 846), ("kgidl", 847),
    ("lkgidl", 848), ("wkgidl", 849), ("pkgidl", 850), ("fgidl", 851), ("lfgidl", 852), ("wfgidl", 853), ("pfgidl", 854), ("rgisl", 855), ("lrgisl", 856), ("wrgisl", 857), ("prgisl", 858), ("kgisl", 859), ("lkgisl", 860), ("wkgisl", 861), ("pkgisl", 862), ("fgisl", 863),
    ("lfgisl", 864), ("wfgisl", 865), ("pfgisl", 866), ("cf", 867), ("lcf", 868), ("wcf", 869), ("pcf", 870), ("cfrcoeff", 871), ("cgso", 872), ("cgdo", 873), ("cgbo", 874), ("cgsl", 875), ("lcgsl", 876), ("wcgsl", 877), ("pcgsl", 878), ("cgdl", 879),
    ("lcgdl", 880), ("wcgdl", 881), ("pcgdl", 882), ("ckappas", 883), ("lckappas", 884), ("wckappas", 885), ("pckappas", 886), ("ckappad", 887), ("lckappad", 888), ("wckappad", 889), ("pckappad", 890), ("ckappad1", 891), ("ckappad2", 892), ("ckappas1", 893), ("ckappas2", 894), ("dmcg", 895),
    ("dmci", 896), ("dmdg", 897), ("dmcgt", 898), ("xgl", 899), ("rshg", 900), ("cjs", 901), ("cjd", 902), ("cjsws", 903), ("cjswd", 904), ("cjswgs", 905), ("cjswgd", 906), ("pbs", 907), ("pbd", 908), ("pbsws", 909), ("pbswd", 910), ("pbswgs", 911),
    ("pbswgd", 912), ("mjs", 913), ("mjd", 914), ("mjsws", 915), ("mjswd", 916), ("mjswgs", 917), ("mjswgd", 918), ("tt", 919), ("ldif0", 920), ("ndif", 921), ("lndif", 922), ("wndif", 923), ("pndif", 924), ("vtm00", 925), ("permod", 926), ("dwj", 927),
    ("xdif", 928), ("lxdif", 929), ("wxdif", 930), ("pxdif", 931), ("isdif", 932), ("iddif", 933), ("lisdif", 934), ("liddif", 935), ("wisdif", 936), ("widdif", 937), ("pisdif", 938), ("piddif", 939), ("nrecf0", 940), ("lnrecf0", 941), ("wnrecf0", 942), ("pnrecf0", 943),
    ("nrecr0", 944), ("lnrecr0", 945), ("wnrecr0", 946), ("pnrecr0", 947), ("xrec", 948), ("lxrec", 949), ("wxrec", 950), ("pxrec", 951), ("isrec", 952), ("idrec", 953), ("lisrec", 954), ("lidrec", 955), ("wisrec", 956), ("widrec", 957), ("pisrec", 958), ("pidrec", 959),
    ("ntrecf", 960), ("ntrecr", 961), ("lntrecf", 962), ("lntrecr", 963), ("wntrecf", 964), ("wntrecr", 965), ("pntrecf", 966), ("pntrecr", 967), ("istun", 968), ("idtun", 969), ("listun", 970), ("lidtun", 971), ("wistun", 972), ("widtun", 973), ("pistun", 974), ("pidtun", 975),
    ("xtun", 976), ("xtund", 977), ("lxtun", 978), ("lxtund", 979), ("wxtun", 980), ("wxtund", 981), ("pxtun", 982), ("pxtund", 983), ("ntun", 984), ("ntund", 985), ("lntun", 986), ("lntund", 987), ("wntun", 988), ("wntund", 989), ("pntun", 990), ("pntund", 991),
    ("vtun0", 992), ("vtun0d", 993), ("lvtun0", 994), ("lvtun0d", 995), ("wvtun0", 996), ("wvtun0d", 997), ("pvtun0", 998), ("pvtun0d", 999), ("vrec0", 1000), ("vrec0d", 1001), ("lvrec0", 1002), ("lvrec0d", 1003), ("wvrec0", 1004), ("wvrec0d", 1005), ("pvrec0", 1006), ("pvrec0d", 1007),
    ("xrcrg1", 1008), ("xrcrg2", 1009), ("ef", 1010), ("em", 1011), ("noia", 1012), ("noib", 1013), ("noic", 1014), ("lintnoi", 1015), ("noia1", 1016), ("noiax", 1017), ("ntnoi", 1018), ("rnoia", 1019), ("rnoib", 1020), ("rnoic", 1021), ("tnoia", 1022), ("tnoib", 1023),
    ("tnoic", 1024), ("binunit", 1025), ("dlbin", 1026), ("dwbin", 1027), ("tnom", 1028), ("tbgasub", 1029), ("tbgbsub", 1030), ("tnfactor", 1031), ("ute", 1032), ("lute", 1033), ("wute", 1034), ("pute", 1035), ("utel", 1036), ("ua1", 1037), ("lua1", 1038), ("wua1", 1039),
    ("pua1", 1040), ("ua1l", 1041), ("uc1", 1042), ("luc1", 1043), ("wuc1", 1044), ("puc1", 1045), ("ud1", 1046), ("lud1", 1047), ("wud1", 1048), ("pud1", 1049), ("ud1l", 1050), ("eu1", 1051), ("leu1", 1052), ("weu1", 1053), ("peu1", 1054), ("ucste", 1055),
    ("lucste", 1056), ("wucste", 1057), ("pucste", 1058), ("teta0", 1059), ("prt", 1060), ("lprt", 1061), ("wprt", 1062), ("pprt", 1063), ("at", 1064), ("lat", 1065), ("wat", 1066), ("pat", 1067), ("atl", 1068), ("tdelta", 1069), ("ptwgt", 1070), ("lptwgt", 1071),
    ("wptwgt", 1072), ("pptwgt", 1073), ("ptwgtl", 1074), ("kt1", 1075), ("kt1exp", 1076), ("kt1l", 1077), ("lkt1", 1078), ("wkt1", 1079), ("pkt1", 1080), ("kt2", 1081), ("lkt2", 1082), ("wkt2", 1083), ("pkt2", 1084), ("iit", 1085), ("liit", 1086), ("wiit", 1087),
    ("piit", 1088), ("igt", 1089), ("ligt", 1090), ("wigt", 1091), ("pigt", 1092), ("tcj", 1093), ("tcjsw", 1094), ("tcjswg", 1095), ("tpb", 1096), ("tpbsw", 1097), ("tpbswg", 1098), ("rth0", 1099), ("cth0", 1100), ("wth0", 1101), ("saref", 1102), ("sbref", 1103),
    ("wlod", 1104), ("ku0", 1105), ("kvsat", 1106), ("tku0", 1107), ("lku0", 1108), ("wku0", 1109), ("pku0", 1110), ("llodku0", 1111), ("wlodku0", 1112), ("kvth0", 1113), ("lkvth0", 1114), ("wkvth0", 1115), ("pkvth0", 1116), ("llodvth", 1117), ("wlodvth", 1118), ("stk2", 1119),
    ("lodk2", 1120), ("steta0", 1121), ("lodeta0", 1122), ("web", 1123), ("wec", 1124), ("kvth0we", 1125), ("lkvth0we", 1126), ("wkvth0we", 1127), ("pkvth0we", 1128), ("k2we", 1129), ("lk2we", 1130), ("wk2we", 1131), ("pk2we", 1132), ("ku0we", 1133), ("lku0we", 1134), ("wku0we", 1135),
    ("pku0we", 1136), ("scref", 1137), ("ssl0", 1138), ("ssl1", 1139), ("ssl2", 1140), ("ssl3", 1141), ("ssl4", 1142), ("ssl5", 1143), ("sslexp1", 1144), ("sslexp2", 1145), ("avdsx", 1146), ("wedge", 1147), ("dgammaedge", 1148), ("dgammaedgel", 1149), ("dgammaedgelexp", 1150), ("dvtedge", 1151),
    ("ndepedge", 1152), ("lndepedge", 1153), ("wndepedge", 1154), ("pndepedge", 1155), ("nfactoredge", 1156), ("lnfactoredge", 1157), ("wnfactoredge", 1158), ("pnfactoredge", 1159), ("citedge", 1160), ("lcitedge", 1161), ("wcitedge", 1162), ("pcitedge", 1163), ("cdscedge", 1164), ("lcdscedge", 1165), ("wcdscedge", 1166), ("pcdscedge", 1167),
    ("cdscdedge", 1168), ("lcdscdedge", 1169), ("wcdscdedge", 1170), ("pcdscdedge", 1171), ("cdscdedger", 1172), ("lcdscdedger", 1173), ("wcdscdedger", 1174), ("pcdscdedger", 1175), ("csecseedge", 1176), ("lcsecseedge", 1177), ("wcsecseedge", 1178), ("pcsecseedge", 1179), ("csecsepedge", 1180), ("csecse0edge", 1181), ("csecse0pedge", 1182), ("csecsededge", 1183),
    ("cbcb0edge", 1184), ("cbcb0pedge", 1185), ("cdscbedge", 1186), ("lcdscbedge", 1187), ("wcdscbedge", 1188), ("pcdscbedge", 1189), ("cbcbpedge", 1190), ("cbcbedge", 1191), ("lcbcbedge", 1192), ("wcbcbedge", 1193), ("pcbcbedge", 1194), ("cbcbdedge", 1195), ("k1edge", 1196), ("k1ledge", 1197), ("k1lexpedge", 1198), ("k1wedge", 1199),
    ("k1wexpedge", 1200), ("k1wledge", 1201), ("k1wlexpedge", 1202), ("lk1edge", 1203), ("wk1edge", 1204), ("pk1edge", 1205), ("eta0edge", 1206), ("leta0edge", 1207), ("weta0edge", 1208), ("peta0edge", 1209), ("etabedge", 1210), ("letabedge", 1211), ("wetabedge", 1212), ("petabedge", 1213), ("kt1edge", 1214), ("lkt1edge", 1215),
    ("wkt1edge", 1216), ("pkt1edge", 1217), ("kt1ledge", 1218), ("lkt1ledge", 1219), ("wkt1ledge", 1220), ("pkt1ledge", 1221), ("kt2edge", 1222), ("lkt2edge", 1223), ("wkt2edge", 1224), ("pkt2edge", 1225), ("kt1expedge", 1226), ("lkt1expedge", 1227), ("wkt1expedge", 1228), ("pkt1expedge", 1229), ("tnfactoredge", 1230), ("ltnfactoredge", 1231),
    ("wtnfactoredge", 1232), ("ptnfactoredge", 1233), ("teta0edge", 1234), ("lteta0edge", 1235), ("wteta0edge", 1236), ("pteta0edge", 1237), ("dvtp0edge", 1238), ("ldvtp0edge", 1239), ("wdvtp0edge", 1240), ("pdvtp0edge", 1241), ("dvtp1edge", 1242), ("ldvtp1edge", 1243), ("wdvtp1edge", 1244), ("pdvtp1edge", 1245), ("dvtp2edge", 1246), ("ldvtp2edge", 1247),
    ("wdvtp2edge", 1248), ("pdvtp2edge", 1249), ("dvtp3edge", 1250), ("ldvtp3edge", 1251), ("wdvtp3edge", 1252), ("pdvtp3edge", 1253), ("dvtp4edge", 1254), ("ldvtp4edge", 1255), ("wdvtp4edge", 1256), ("pdvtp4edge", 1257), ("dvtp5edge", 1258), ("ldvtp5edge", 1259), ("wdvtp5edge", 1260), ("pdvtp5edge", 1261), ("dvt0edge", 1262), ("dvt1edge", 1263),
    ("dvt2edge", 1264), ("k2edge", 1265), ("k2ledge", 1266), ("k2lexpedge", 1267), ("k2wedge", 1268), ("k2wexpedge", 1269), ("k2wledge", 1270), ("k2wlexpedge", 1271), ("lk2edge", 1272), ("wk2edge", 1273), ("pk2edge", 1274), ("kvth0edge", 1275), ("lkvth0edge", 1276), ("wkvth0edge", 1277), ("pkvth0edge", 1278), ("kvth0edgewe", 1279),
    ("lkvth0edgewe", 1280), ("wkvth0edgewe", 1281), ("pkvth0edgewe", 1282), ("k2edgewe", 1283), ("lk2edgewe", 1284), ("wk2edgewe", 1285), ("pk2edgewe", 1286), ("stk2edge", 1287), ("lstk2edge", 1288), ("wstk2edge", 1289), ("pstk2edge", 1290), ("steta0edge", 1291), ("lsteta0edge", 1292), ("wsteta0edge", 1293), ("psteta0edge", 1294), ("igclamp", 1295),
    ("lp", 1296), ("rnoik", 1297), ("tnoik", 1298), ("tnoik2", 1299), ("k0", 1300), ("lk0", 1301), ("wk0", 1302), ("pk0", 1303), ("k01", 1304), ("lk01", 1305), ("wk01", 1306), ("pk01", 1307), ("m0", 1308), ("lm0", 1309), ("wm0", 1310), ("pm0", 1311),
    ("m01", 1312), ("lm01", 1313), ("wm01", 1314), ("pm01", 1315), ("nedge", 1316), ("noia1_edge", 1317), ("noiax_edge", 1318), ("fnoimod", 1319), ("lh", 1320), ("noia2", 1321), ("hndep", 1322), ("c0", 1323), ("lc0", 1324), ("wc0", 1325), ("pc0", 1326), ("c01", 1327),
    ("lc01", 1328), ("wc01", 1329), ("pc01", 1330), ("c0si", 1331), ("lc0si", 1332), ("wc0si", 1333), ("pc0si", 1334), ("c0si1", 1335), ("lc0si1", 1336), ("wc0si1", 1337), ("pc0si1", 1338), ("c0sisat", 1339), ("lc0sisat", 1340), ("wc0sisat", 1341), ("pc0sisat", 1342), ("c0sisat1", 1343),
    ("lc0sisat1", 1344), ("wc0sisat1", 1345), ("pc0sisat1", 1346), ("minr", 1347), ("abulk", 1348), ("a0", 1349), ("ags", 1350), ("ags1", 1351), ("keta", 1352), ("a0cv", 1353), ("agscv", 1354), ("ketacv", 1355), ("rbody", 1356), ("frbody", 1357), ("rbsh", 1358), ("nrb", 1359),
    ("rhalo", 1360), ("ub", 1361), ("lub", 1362), ("wub", 1363), ("pub", 1364), ("ubte", 1365), ("lubte", 1366), ("wubte", 1367), ("pubte", 1368), ("neff", 1369), ("lneff", 1370), ("wneff", 1371), ("pneff", 1372), ("nseg", 1373), ("rbodyagbcp2", 1374), ("nbc", 1375),
    ("dwbc", 1376), ("pdbcp", 1377), ("psbcp", 1378), ("agbcp", 1379), ("agbcp2", 1380), ("agbcpd", 1381), ("aebcp", 1382), ("eggbcp2", 1383), ("nsub", 1384), ("lnsub", 1385), ("wnsub", 1386), ("pnsub", 1387), ("fbody", 1388), ("kb1", 1389), ("lkb1", 1390), ("wkb1", 1391),
    ("pkb1", 1392), ("dlbg", 1393), ("dlcb", 1394), ("csdesw", 1395), ("csdmin", 1396), ("acesb", 1397), ("bcesb", 1398), ("acedb", 1399), ("bcedb", 1400),
];

const PARAMETER_MIN_REFERENCES: [Option<usize>; 1401] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

const PARAMETER_MAX_REFERENCES: [Option<usize>; 1401] = [
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, Some(0), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None, None,
];

const PARAMETER_DISPLAY_NAMES: [&str; 1401] = [
    "L", "W", "NF", "NRS", "NRD", "VFBSDOFF", "MINZ", "RGATEMOD", "GEOMOD", "RGEOMOD", "SA", "SB", "SD", "SCA", "SCB", "SCC",
    "SC", "AS", "AD", "PS", "PD", "XGW", "NGCON", "DTEMP", "MULU0", "DELVTO", "IDS0MULT", "EDGEFET", "SSLMOD", "SOIMOD", "TYPE", "CVMOD",
    "COVMOD", "RDSMOD", "WPEMOD", "ASYMMOD", "GIDLMOD", "IGCMOD", "IGBMOD", "TNOIMOD", "TNODEOUT", "SHMOD", "MOBSCALE", "BODYMOD", "IIIMOD", "MODAGBCP2", "PDEMOD", "FBODY1",
    "LLONG", "LMLT", "WMLT", "XL", "WWIDE", "XW", "LINT", "LL", "LW", "LWL", "LLN", "LWN", "WINT", "WL", "WW", "WWL",
    "WLN", "WWN", "DLC", "LLC", "LWC", "LWLC", "DWC", "WLC", "WWC", "WWLC", "TSI", "TBOX", "TOXE", "TOXP", "DTOX", "NDEP",
    "NDEPL1", "NDEPLEXP1", "NDEPL2", "NDEPLEXP2", "NDEPW", "NDEPWEXP", "NDEPWL", "NDEPWLEXP", "LNDEP", "WNDEP", "PNDEP", "NDEPCV", "NDEPCVL1", "NDEPCVLEXP1", "NDEPCVL2", "NDEPCVLEXP2",
    "NDEPCVW", "NDEPCVWEXP", "NDEPCVWL", "NDEPCVWLEXP", "LNDEPCV", "WNDEPCV", "PNDEPCV", "NGATE", "LNGATE", "WNGATE", "PNGATE", "NI0SUB", "BG0SUB", "EPSRSUB", "EPSROX", "XJ",
    "LXJ", "WXJ", "PXJ", "VFB", "LVFB", "WVFB", "PVFB", "VFBB", "LVFBB", "WVFBB", "PVFBB", "VFBL", "VFBLEXP", "VFBW", "VFBWEXP", "VFBWL",
    "VFBWLEXP", "VFBCV", "LVFBCV", "WVFBCV", "PVFBCV", "VFBCVL", "VFBCVLEXP", "VFBCVW", "VFBCVWEXP", "VFBCVWL", "VFBCVWLEXP", "DELVFBACC", "VFBAGBCP2", "NDEPAGBCP2", "NSD", "LNSD",
    "WNSD", "PNSD", "DVTP0", "LDVTP0", "WDVTP0", "PDVTP0", "DVTP1", "LDVTP1", "WDVTP1", "PDVTP1", "DVTP2", "LDVTP2", "WDVTP2", "PDVTP2", "DVTP3", "LDVTP3",
    "WDVTP3", "PDVTP3", "DVTP4", "LDVTP4", "WDVTP4", "PDVTP4", "DVTP5", "LDVTP5", "WDVTP5", "PDVTP5", "DVBD0", "LDVBD0", "WDVBD0", "PDVBD0", "DVBD1", "LDVBD1",
    "WDVBD1", "PDVBD1", "VSCE", "LVSCE", "WVSCE", "PVSCE", "CDSBS1", "LCDSBS1", "WCDSBS1", "PCDSBS1", "CDSBS", "LCDSBS", "WCDSBS", "PCDSBS", "PHIN", "LPHIN",
    "WPHIN", "PPHIN", "ETA0", "LETA0", "WETA0", "PETA0", "ETA0R", "LETA0R", "WETA0R", "PETA0R", "DSUB", "ETAB", "ETABEXP", "LETAB", "WETAB", "PETAB",
    "K1", "K1L", "K1LEXP", "K1W", "K1WEXP", "K1WL", "K1WLEXP", "LK1", "WK1", "PK1", "K2", "K2L", "K2LEXP", "K2W", "K2WEXP", "K2WL",
    "K2WLEXP", "LK2", "WK2", "PK2", "ADOS", "BDOS", "QM0", "ETAQM", "CIT", "LCIT", "WCIT", "PCIT", "NFACTOR", "NFACTORL", "NFACTORLEXP", "NFACTORW",
    "NFACTORWEXP", "NFACTORWL", "NFACTORWLEXP", "LNFACTOR", "WNFACTOR", "PNFACTOR", "ASCL", "LASCL", "WASCL", "PASCL", "BSCL", "LBSCL", "WBSCL", "PBSCL", "DVT1", "LDVT1",
    "WDVT1", "PDVT1", "CDSCD", "LCDSCD", "WCDSCD", "PCDSCD", "CDSC", "LCDSC", "WCDSC", "PCDSC", "CSECSED", "CBCBD", "CSECSE0", "CSECSE0P", "CSECSE", "LCSECSE",
    "WCSECSE", "PCSECSE", "CSECSEP", "CBCB", "LCBCB", "WCBCB", "PCBCB", "CBCBP", "CBCB0", "CBCB0P", "CDSCDL", "CDSCDLEXP", "CDSCDR", "LCDSCDR", "WCDSCDR", "PCDSCDR",
    "CDSCB", "CDSCBL", "CDSCBLEXP", "LCDSCB", "WCDSCB", "PCDSCB", "VBSA", "VSAT", "LVSAT", "WVSAT", "PVSAT", "VSATL", "VSATLEXP", "VSATW", "VSATWEXP", "VSATWL",
    "VSATWLEXP", "VSATR", "LVSATR", "WVSATR", "PVSATR", "DELTA", "LDELTA", "WDELTA", "PDELTA", "DELTAL", "DELTALEXP", "VSATCV", "LVSATCV", "WVSATCV", "PVSATCV", "VSATCVL",
    "VSATCVLEXP", "VSATCVW", "VSATCVWEXP", "VSATCVWL", "VSATCVWLEXP", "THESAT", "LTHESAT", "WTHESAT", "PTHESAT", "LPE1", "LLPE1", "WLPE1", "PLPE1", "UP1", "LP1", "UP2",
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
    "PPSATB", "PSATX", "PTWG", "LPTWG", "WPTWG", "PPTWG", "PTWGL", "VP", "ALP", "PTWGLEXP", "PTWGR", "LPTWGR", "WPTWGR", "PPTWGR", "KSATIV", "LKSATIV",
    "WKSATIV", "PKSATIV", "A1", "LA1", "WA1", "PA1", "A11", "LA11", "WA11", "PA11", "A2", "LA2", "WA2", "PA2", "A21", "LA21",
    "WA21", "PA21", "PDIBLC", "PDIBLCL", "PDIBLCLEXP", "LPDIBLC", "WPDIBLC", "PPDIBLC", "PDIBLCR", "LPDIBLCR", "WPDIBLCR", "PPDIBLCR", "PDIBLCB", "LPDIBLCB", "WPDIBLCB", "PPDIBLCB",
    "PVAG", "LPVAG", "WPVAG", "PPVAG", "FPROUT", "FPROUTL", "FPROUTLEXP", "LFPROUT", "WFPROUT", "PFPROUT", "BJTOFF", "VABJT", "LVABJT", "WVABJT", "PVABJT", "AELY",
    "LAELY", "WAELY", "PAELY", "AHLI", "AHLID", "LAHLI", "LAHLID", "WAHLI", "WAHLID", "PAHLI", "PAHLID", "XBJT", "LXBJT", "WXBJT", "PXBJT", "NDIODE",
    "LNDIODE", "WNDIODE", "PNDIODE", "ISBJT", "PISBJT", "WISBJT", "LISBJT", "IDBJT", "LIDBJT", "WIDBJT", "PIDBJT", "NBJT", "LNBJT", "LLBJT0", "WNBJT", "WLBJT0",
    "PNBJT", "PLBJT0", "LBJT0", "LN", "VDSATII0", "LVDSATII0", "WVDSATII0", "PVDSATII0", "TII", "ALPHA0", "ALPHA0L", "ALPHA0LEXP", "LALPHA0", "WALPHA0", "PALPHA0", "BETA0",
    "LBETA0", "WBETA0", "PBETA0", "BETA1", "LBETA1", "WBETA1", "PBETA1", "BETA2", "LBETA2", "WBETA2", "PBETA2", "LII", "LLII", "WLII", "PLII", "SII0",
    "LSII0", "WSII0", "PSII0", "SII1", "LSII1", "WSII1", "PSII1", "SII2", "LSII2", "WSII2", "PSII2", "SIID", "LSIID", "WSIID", "PSIID", "ESATII",
    "LESATII", "WESATII", "PESATII", "IIMOD2CLAMP1", "IIMOD2CLAMP2", "IIMOD2CLAMP3", "FBJTII", "LFBJTII", "WFBJTII", "PFBJTII", "EBJTII", "CBJTII", "ABJTII", "LABJTII", "LCBJTII", "LEBJTII",
    "WABJTII", "WCBJTII", "WEBJTII", "PABJTII", "PCBJTII", "PEBJTII", "VBCI", "LVBCI", "WVBCI", "PVBCI", "TVBCI", "MBJTII", "LMBJTII", "WMBJTII", "PMBJTII", "VECB",
    "ALPHAGB1", "LALPHAGB1", "WALPHAGB1", "PALPHAGB1", "ALPHAGB1_T", "LALPHAGB1_T", "WALPHAGB1_T", "PALPHAGB1_T", "BETAGB1", "LBETAGB1", "WBETAGB1", "PBETAGB1", "ALPHAGB2", "LALPHAGB2", "WALPHAGB2", "PALPHAGB2",
    "ALPHAGB2_T", "LALPHAGB2_T", "WALPHAGB2_T", "PALPHAGB2_T", "BETAGB2", "LBETAGB2", "WBETAGB2", "PBETAGB2", "VGB2", "VGB1", "AGB1", "BGB1", "AGB2", "BGB2", "AGBC2N", "AGBC2P",
    "BGBC2N", "BGBC2P", "EIGBINV", "AIGC", "BIGC", "CIGC", "AIGS", "AIGS1", "BIGS", "CIGS", "AIGD", "AIGD1", "BIGD", "CIGD", "DLCIG", "DLCIGD",
    "POXEDGE", "NTOX", "TOXREF", "PIGCD", "AIGCL", "AIGCW", "AIGC1", "AIGSL", "AIGSW", "AIGDL", "AIGDW", "PIGCDL", "LEIGBINV", "WEIGBINV", "PEIGBINV", "LAIGC",
    "LAIGC1", "WAIGC", "WAIGC1", "PAIGC", "PAIGC1", "LBIGC", "WBIGC", "PBIGC", "LCIGC", "WCIGC", "PCIGC", "LAIGS", "LAIGS1", "WAIGS", "WAIGS1", "PAIGS",
    "PAIGS1", "LBIGS", "WBIGS", "PBIGS", "LCIGS", "WCIGS", "PCIGS", "LAIGD", "LAIGD1", "WAIGD", "WAIGD1", "PAIGD", "PAIGD1", "LBIGD", "WBIGD", "PBIGD",
    "LCIGD", "WCIGD", "PCIGD", "LPOXEDGE", "WPOXEDGE", "PPOXEDGE", "LDLCIG", "WDLCIG", "PDLCIG", "LDLCIGD", "WDLCIGD", "PDLCIGD", "LNTOX", "WNTOX", "PNTOX", "AIGBCP2",
    "AIGBCP2_T", "BIGBCP2", "CIGBCP2", "LAIGBCP2", "LAIGBCP2_T", "LBIGBCP2", "LCIGBCP2", "WAIGBCP2", "WAIGBCP2_T", "WBIGBCP2", "WCIGBCP2", "PAIGBCP2", "PAIGBCP2_T", "PBIGBCP2", "PCIGBCP2", "AGIDL",
    "AGIDLL", "AGIDLW", "LAGIDL", "WAGIDL", "PAGIDL", "BGIDL", "BGIDL1", "LBGIDL", "WBGIDL", "PBGIDL", "LBGIDL1", "WBGIDL1", "PBGIDL1", "CGIDL", "LCGIDL", "WCGIDL",
    "PCGIDL", "EGIDL", "LEGIDL", "WEGIDL", "PEGIDL", "AGISL", "AGISLL", "AGISLW", "LAGISL", "WAGISL", "PAGISL", "BGISL", "BGISL1", "LBGISL", "WBGISL", "PBGISL",
    "LBGISL1", "WBGISL1", "PBGISL1", "CGISL", "LCGISL", "WCGISL", "PCGISL", "EGISL", "LEGISL", "WEGISL", "PEGISL", "RGIDL", "LRGIDL", "WRGIDL", "PRGIDL", "KGIDL",
    "LKGIDL", "WKGIDL", "PKGIDL", "FGIDL", "LFGIDL", "WFGIDL", "PFGIDL", "RGISL", "LRGISL", "WRGISL", "PRGISL", "KGISL", "LKGISL", "WKGISL", "PKGISL", "FGISL",
    "LFGISL", "WFGISL", "PFGISL", "CF", "LCF", "WCF", "PCF", "CFRCOEFF", "CGSO", "CGDO", "CGBO", "CGSL", "LCGSL", "WCGSL", "PCGSL", "CGDL",
    "LCGDL", "WCGDL", "PCGDL", "CKAPPAS", "LCKAPPAS", "WCKAPPAS", "PCKAPPAS", "CKAPPAD", "LCKAPPAD", "WCKAPPAD", "PCKAPPAD", "CKAPPAD1", "CKAPPAD2", "CKAPPAS1", "CKAPPAS2", "DMCG",
    "DMCI", "DMDG", "DMCGT", "XGL", "RSHG", "CJS", "CJD", "CJSWS", "CJSWD", "CJSWGS", "CJSWGD", "PBS", "PBD", "PBSWS", "PBSWD", "PBSWGS",
    "PBSWGD", "MJS", "MJD", "MJSWS", "MJSWD", "MJSWGS", "MJSWGD", "TT", "LDIF0", "NDIF", "LNDIF", "WNDIF", "PNDIF", "VTM00", "PERMOD", "DWJ",
    "XDIF", "LXDIF", "WXDIF", "PXDIF", "ISDIF", "IDDIF", "LISDIF", "LIDDIF", "WISDIF", "WIDDIF", "PISDIF", "PIDDIF", "NRECF0", "LNRECF0", "WNRECF0", "PNRECF0",
    "NRECR0", "LNRECR0", "WNRECR0", "PNRECR0", "XREC", "LXREC", "WXREC", "PXREC", "ISREC", "IDREC", "LISREC", "LIDREC", "WISREC", "WIDREC", "PISREC", "PIDREC",
    "NTRECF", "NTRECR", "LNTRECF", "LNTRECR", "WNTRECF", "WNTRECR", "PNTRECF", "PNTRECR", "ISTUN", "IDTUN", "LISTUN", "LIDTUN", "WISTUN", "WIDTUN", "PISTUN", "PIDTUN",
    "XTUN", "XTUND", "LXTUN", "LXTUND", "WXTUN", "WXTUND", "PXTUN", "PXTUND", "NTUN", "NTUND", "LNTUN", "LNTUND", "WNTUN", "WNTUND", "PNTUN", "PNTUND",
    "VTUN0", "VTUN0D", "LVTUN0", "LVTUN0D", "WVTUN0", "WVTUN0D", "PVTUN0", "PVTUN0D", "VREC0", "VREC0D", "LVREC0", "LVREC0D", "WVREC0", "WVREC0D", "PVREC0", "PVREC0D",
    "XRCRG1", "XRCRG2", "EF", "EM", "NOIA", "NOIB", "NOIC", "LINTNOI", "NOIA1", "NOIAX", "NTNOI", "RNOIA", "RNOIB", "RNOIC", "TNOIA", "TNOIB",
    "TNOIC", "BINUNIT", "DLBIN", "DWBIN", "TNOM", "TBGASUB", "TBGBSUB", "TNFACTOR", "UTE", "LUTE", "WUTE", "PUTE", "UTEL", "UA1", "LUA1", "WUA1",
    "PUA1", "UA1L", "UC1", "LUC1", "WUC1", "PUC1", "UD1", "LUD1", "WUD1", "PUD1", "UD1L", "EU1", "LEU1", "WEU1", "PEU1", "UCSTE",
    "LUCSTE", "WUCSTE", "PUCSTE", "TETA0", "PRT", "LPRT", "WPRT", "PPRT", "AT", "LAT", "WAT", "PAT", "ATL", "TDELTA", "PTWGT", "LPTWGT",
    "WPTWGT", "PPTWGT", "PTWGTL", "KT1", "KT1EXP", "KT1L", "LKT1", "WKT1", "PKT1", "KT2", "LKT2", "WKT2", "PKT2", "IIT", "LIIT", "WIIT",
    "PIIT", "IGT", "LIGT", "WIGT", "PIGT", "TCJ", "TCJSW", "TCJSWG", "TPB", "TPBSW", "TPBSWG", "RTH0", "CTH0", "WTH0", "SAREF", "SBREF",
    "WLOD", "KU0", "KVSAT", "TKU0", "LKU0", "WKU0", "PKU0", "LLODKU0", "WLODKU0", "KVTH0", "LKVTH0", "WKVTH0", "PKVTH0", "LLODVTH", "WLODVTH", "STK2",
    "LODK2", "STETA0", "LODETA0", "WEB", "WEC", "KVTH0WE", "LKVTH0WE", "WKVTH0WE", "PKVTH0WE", "K2WE", "LK2WE", "WK2WE", "PK2WE", "KU0WE", "LKU0WE", "WKU0WE",
    "PKU0WE", "SCREF", "SSL0", "SSL1", "SSL2", "SSL3", "SSL4", "SSL5", "SSLEXP1", "SSLEXP2", "AVDSX", "WEDGE", "DGAMMAEDGE", "DGAMMAEDGEL", "DGAMMAEDGELEXP", "DVTEDGE",
    "NDEPEDGE", "LNDEPEDGE", "WNDEPEDGE", "PNDEPEDGE", "NFACTOREDGE", "LNFACTOREDGE", "WNFACTOREDGE", "PNFACTOREDGE", "CITEDGE", "LCITEDGE", "WCITEDGE", "PCITEDGE", "CDSCEDGE", "LCDSCEDGE", "WCDSCEDGE", "PCDSCEDGE",
    "CDSCDEDGE", "LCDSCDEDGE", "WCDSCDEDGE", "PCDSCDEDGE", "CDSCDEDGER", "LCDSCDEDGER", "WCDSCDEDGER", "PCDSCDEDGER", "CSECSEEDGE", "LCSECSEEDGE", "WCSECSEEDGE", "PCSECSEEDGE", "CSECSEPEDGE", "CSECSE0EDGE", "CSECSE0PEDGE", "CSECSEDEDGE",
    "CBCB0EDGE", "CBCB0PEDGE", "CDSCBEDGE", "LCDSCBEDGE", "WCDSCBEDGE", "PCDSCBEDGE", "CBCBPEDGE", "CBCBEDGE", "LCBCBEDGE", "WCBCBEDGE", "PCBCBEDGE", "CBCBDEDGE", "K1EDGE", "K1LEDGE", "K1LEXPEDGE", "K1WEDGE",
    "K1WEXPEDGE", "K1WLEDGE", "K1WLEXPEDGE", "LK1EDGE", "WK1EDGE", "PK1EDGE", "ETA0EDGE", "LETA0EDGE", "WETA0EDGE", "PETA0EDGE", "ETABEDGE", "LETABEDGE", "WETABEDGE", "PETABEDGE", "KT1EDGE", "LKT1EDGE",
    "WKT1EDGE", "PKT1EDGE", "KT1LEDGE", "LKT1LEDGE", "WKT1LEDGE", "PKT1LEDGE", "KT2EDGE", "LKT2EDGE", "WKT2EDGE", "PKT2EDGE", "KT1EXPEDGE", "LKT1EXPEDGE", "WKT1EXPEDGE", "PKT1EXPEDGE", "TNFACTOREDGE", "LTNFACTOREDGE",
    "WTNFACTOREDGE", "PTNFACTOREDGE", "TETA0EDGE", "LTETA0EDGE", "WTETA0EDGE", "PTETA0EDGE", "DVTP0EDGE", "LDVTP0EDGE", "WDVTP0EDGE", "PDVTP0EDGE", "DVTP1EDGE", "LDVTP1EDGE", "WDVTP1EDGE", "PDVTP1EDGE", "DVTP2EDGE", "LDVTP2EDGE",
    "WDVTP2EDGE", "PDVTP2EDGE", "DVTP3EDGE", "LDVTP3EDGE", "WDVTP3EDGE", "PDVTP3EDGE", "DVTP4EDGE", "LDVTP4EDGE", "WDVTP4EDGE", "PDVTP4EDGE", "DVTP5EDGE", "LDVTP5EDGE", "WDVTP5EDGE", "PDVTP5EDGE", "DVT0EDGE", "DVT1EDGE",
    "DVT2EDGE", "K2EDGE", "K2LEDGE", "K2LEXPEDGE", "K2WEDGE", "K2WEXPEDGE", "K2WLEDGE", "K2WLEXPEDGE", "LK2EDGE", "WK2EDGE", "PK2EDGE", "KVTH0EDGE", "LKVTH0EDGE", "WKVTH0EDGE", "PKVTH0EDGE", "KVTH0EDGEWE",
    "LKVTH0EDGEWE", "WKVTH0EDGEWE", "PKVTH0EDGEWE", "K2EDGEWE", "LK2EDGEWE", "WK2EDGEWE", "PK2EDGEWE", "STK2EDGE", "LSTK2EDGE", "WSTK2EDGE", "PSTK2EDGE", "STETA0EDGE", "LSTETA0EDGE", "WSTETA0EDGE", "PSTETA0EDGE", "IGCLAMP",
    "LP", "RNOIK", "TNOIK", "TNOIK2", "K0", "LK0", "WK0", "PK0", "K01", "LK01", "WK01", "PK01", "M0", "LM0", "WM0", "PM0",
    "M01", "LM01", "WM01", "PM01", "NEDGE", "NOIA1_EDGE", "NOIAX_EDGE", "FNOIMOD", "LH", "NOIA2", "HNDEP", "C0", "LC0", "WC0", "PC0", "C01",
    "LC01", "WC01", "PC01", "C0SI", "LC0SI", "WC0SI", "PC0SI", "C0SI1", "LC0SI1", "WC0SI1", "PC0SI1", "C0SISAT", "LC0SISAT", "WC0SISAT", "PC0SISAT", "C0SISAT1",
    "LC0SISAT1", "WC0SISAT1", "PC0SISAT1", "minr", "ABULK", "A0", "AGS", "AGS1", "KETA", "A0CV", "AGSCV", "KETACV", "RBODY", "FRBODY", "RBSH", "NRB",
    "RHALO", "UB", "LUB", "WUB", "PUB", "UBTE", "LUBTE", "WUBTE", "PUBTE", "NEFF", "LNEFF", "WNEFF", "PNEFF", "NSEG", "RBODYAGBCP2", "NBC",
    "DWBC", "PDBCP", "PSBCP", "AGBCP", "AGBCP2", "AGBCPD", "AEBCP", "EGGBCP2", "NSUB", "LNSUB", "WNSUB", "PNSUB", "FBODY", "KB1", "LKB1", "WKB1",
    "PKB1", "DLBG", "DLCB", "CSDESW", "CSDMIN", "ACESB", "BCESB", "ACEDB", "BCEDB",
];

const PARAMETER_EXCLUDED_REFERENCES: [&[usize]; 1401] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[],
];

const PARAMETER_INTEGER_FLAGS: [bool; 1401] = [
    false, false, true, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
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
    false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false,
];

const PARAMETER_MIN_BOUNDS: [Option<ParameterBound>; 1401] = [
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -1.0, label: "-1.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: -50.0, label: "-50.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
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
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 5.0, label: "5.0" }), Some(ParameterBound { value: 1e-9, label: "1e-9" }), None, None, None, None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None,
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None,
    None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 0.0, label: "0.0" }), None, Some(ParameterBound { value: 0.0, label: "0.0" }),
    None,
];

const PARAMETER_MAX_BOUNDS: [Option<ParameterBound>; 1401] = [
    None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 3.0, label: "3.0" }),
    Some(ParameterBound { value: 10.0, label: "10.0" }), Some(ParameterBound { value: 8.0, label: "8.0" }), None, None, None, None, None, None,
    None, None, None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None,
    None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 2.0, label: "2.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, Some(ParameterBound { value: 50.0, label: "50.0" }), None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }), Some(ParameterBound { value: 1.0, label: "1.0" }), None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None, None, None,
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
    None, None, Some(ParameterBound { value: 100.0, label: "100.0" }), None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
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
    None, None, None, None, None, None, None, Some(ParameterBound { value: 1.0, label: "1.0" }),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, Some(ParameterBound { value: 2.0, label: "2.0" }), None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None,
];

const PARAMETER_RANGE_FLAGS: [u8; 1401] = [
    3, 3, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2, 2, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3,
    0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0,
    3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0,
    3, 0, 0, 0, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0,
    3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
    3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 2,
    2, 2, 2, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 1, 0, 0, 0, 0, 0, 2, 3, 2, 0, 0, 0, 3, 3,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 3, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3, 3, 3,
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 2, 3, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 2, 2, 0, 0, 3, 2, 0, 0, 2, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0,
];

const PARAMETER_EXCLUDED_BOUNDS: [&[ParameterBound]; 1401] = [
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[ParameterBound { value: 0.0, label: "0.0" }], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[], &[], &[], &[], &[], &[], &[], &[],
    &[],
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
        899 => Some(ParameterBound { value: ((params.p0 * params.p49) + params.p51), label: "computed upper-bound expression" }),
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
    pub branches: [usize; 12],
    pub params: Box<Parameters>,
    pub(crate) param_given: Box<[bool; 1401]>,
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: Box<[f64; 23]>,
    pub(crate) ddt_state_previous: Box<[f64; 23]>,
    pub(crate) ddt_state_older: Box<[f64; 23]>,
    pub(crate) ddt_state_initialized: Box<[bool; 23]>,
    pub(crate) ddt_derivative_current: Box<[f64; 23]>,
    pub(crate) ddt_derivative_previous: Box<[f64; 23]>,
    pub(crate) idt_state_current: Box<[f64; 0]>,
    pub(crate) idt_state_previous: Box<[f64; 0]>,
    pub(crate) idt_state_initialized: Box<[bool; 0]>,
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) ddt_coefficients: GeneratedDdtCoefficients,
    pub(crate) scratch: Option<Box<KernelScratch<2040, 14, 12>>>,
    pub(crate) reactive_scratch: Option<Box<KernelReactiveScratch<2040, 14, 12>>>,
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
    pub const TERMINAL_COUNT: usize = 6;
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 14;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["di", "si", "gi", "gm", "bi", "bi2", "N1", "N2"];

    pub const BRANCH_COUNT: usize = 12;
    pub const PARAMETER_COUNT: usize = 1401;
    pub const VARIABLE_COUNT: usize = 2040;
    pub const DDT_STATE_COUNT: usize = 23;
    pub const IDT_STATE_COUNT: usize = 0;
    pub const CHECKPOINT_MODEL_IDENTITY: &'static str = "9dee66c46450922ddc50a98a9aa172270d7162669fa1b48a6e2cd272266f8b88";
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
