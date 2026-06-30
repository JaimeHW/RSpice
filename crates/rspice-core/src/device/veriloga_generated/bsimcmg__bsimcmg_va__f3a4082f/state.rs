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
    pub p864: f64,
    pub p865: f64,
    pub p866: f64,
    pub p867: f64,
    pub p868: f64,
    pub p869: f64,
    pub p870: f64,
    pub p871: f64,
    pub p872: f64,
    pub p873: f64,
    pub p874: f64,
    pub p875: f64,
    pub p876: f64,
    pub p877: f64,
    pub p878: f64,
    pub p879: f64,
    pub p880: f64,
    pub p881: f64,
    pub p882: f64,
    pub p883: f64,
    pub p884: f64,
    pub p885: f64,
    pub p886: f64,
    pub p887: f64,
    pub p888: f64,
    pub p889: f64,
    pub p890: f64,
    pub p891: f64,
    pub p892: f64,
    pub p893: f64,
    pub p894: f64,
    pub p895: f64,
    pub p896: f64,
    pub p897: f64,
    pub p898: f64,
    pub p899: f64,
    pub p900: f64,
    pub p901: f64,
    pub p902: f64,
    pub p903: f64,
    pub p904: f64,
    pub p905: f64,
    pub p906: f64,
    pub p907: f64,
    pub p908: f64,
    pub p909: f64,
    pub p910: f64,
    pub p911: f64,
    pub p912: f64,
    pub p913: f64,
    pub p914: f64,
    pub p915: f64,
    pub p916: f64,
    pub p917: f64,
    pub p918: f64,
    pub p919: f64,
    pub p920: f64,
    pub p921: f64,
    pub p922: f64,
    pub p923: f64,
    pub p924: f64,
    pub p925: f64,
    pub p926: f64,
    pub p927: f64,
    pub p928: f64,
    pub p929: f64,
    pub p930: f64,
    pub p931: f64,
    pub p932: f64,
    pub p933: f64,
    pub p934: f64,
    pub p935: f64,
    pub p936: f64,
    pub p937: f64,
    pub p938: f64,
    pub p939: f64,
    pub p940: f64,
    pub p941: f64,
    pub p942: f64,
    pub p943: f64,
    pub p944: f64,
    pub p945: f64,
    pub p946: f64,
    pub p947: f64,
    pub p948: f64,
    pub p949: f64,
    pub p950: f64,
    pub p951: f64,
    pub p952: f64,
    pub p953: f64,
    pub p954: f64,
    pub p955: f64,
    pub p956: f64,
    pub p957: f64,
    pub p958: f64,
    pub p959: f64,
    pub p960: f64,
    pub p961: f64,
    pub p962: f64,
    pub p963: f64,
    pub p964: f64,
    pub p965: f64,
    pub p966: f64,
    pub p967: f64,
    pub p968: f64,
    pub p969: f64,
    pub p970: f64,
    pub p971: f64,
    pub p972: f64,
    pub p973: f64,
    pub p974: f64,
    pub p975: f64,
    pub p976: f64,
    pub p977: f64,
    pub p978: f64,
    pub p979: f64,
    pub p980: f64,
    pub p981: f64,
    pub p982: f64,
    pub p983: f64,
    pub p984: f64,
    pub p985: f64,
    pub p986: f64,
    pub p987: f64,
    pub p988: f64,
    pub p989: f64,
    pub p990: f64,
    pub p991: f64,
    pub p992: f64,
    pub p993: f64,
    pub p994: f64,
    pub p995: f64,
    pub p996: f64,
    pub p997: f64,
    pub p998: f64,
    pub p999: f64,
    pub p1000: f64,
    pub p1001: f64,
    pub p1002: f64,
    pub p1003: f64,
    pub p1004: f64,
    pub p1005: f64,
    pub p1006: f64,
    pub p1007: f64,
    pub p1008: f64,
    pub p1009: f64,
    pub p1010: f64,
    pub p1011: f64,
    pub p1012: f64,
    pub p1013: f64,
    pub p1014: f64,
    pub p1015: f64,
    pub p1016: f64,
    pub p1017: f64,
    pub p1018: f64,
    pub p1019: f64,
    pub p1020: f64,
    pub p1021: f64,
    pub p1022: f64,
    pub p1023: f64,
    pub p1024: f64,
    pub p1025: f64,
    pub p1026: f64,
    pub p1027: f64,
    pub p1028: f64,
    pub p1029: f64,
    pub p1030: f64,
    pub p1031: f64,
    pub p1032: f64,
    pub p1033: f64,
    pub p1034: f64,
    pub p1035: f64,
    pub p1036: f64,
    pub p1037: f64,
    pub p1038: f64,
    pub p1039: f64,
    pub p1040: f64,
    pub p1041: f64,
    pub p1042: f64,
    pub p1043: f64,
    pub p1044: f64,
    pub p1045: f64,
    pub p1046: f64,
    pub p1047: f64,
    pub p1048: f64,
    pub p1049: f64,
    pub p1050: f64,
    pub p1051: f64,
    pub p1052: f64,
    pub p1053: f64,
    pub p1054: f64,
    pub p1055: f64,
    pub p1056: f64,
    pub p1057: f64,
    pub p1058: f64,
    pub p1059: f64,
    pub p1060: f64,
    pub p1061: f64,
    pub p1062: f64,
    pub p1063: f64,
    pub p1064: f64,
    pub p1065: f64,
    pub p1066: f64,
    pub p1067: f64,
    pub p1068: f64,
    pub p1069: f64,
    pub p1070: f64,
    pub p1071: f64,
    pub p1072: f64,
    pub p1073: f64,
    pub p1074: f64,
    pub p1075: f64,
    pub p1076: f64,
    pub p1077: f64,
    pub p1078: f64,
    pub p1079: f64,
    pub p1080: f64,
    pub p1081: f64,
    pub p1082: f64,
    pub p1083: f64,
    pub p1084: f64,
    pub p1085: f64,
    pub p1086: f64,
    pub p1087: f64,
    pub p1088: f64,
    pub p1089: f64,
    pub p1090: f64,
    pub p1091: f64,
    pub p1092: f64,
    pub p1093: f64,
    pub p1094: f64,
    pub p1095: f64,
    pub p1096: f64,
    pub p1097: f64,
    pub p1098: f64,
    pub p1099: f64,
    pub p1100: f64,
    pub p1101: f64,
    pub p1102: f64,
    pub p1103: f64,
    pub p1104: f64,
    pub p1105: f64,
    pub p1106: f64,
    pub p1107: f64,
    pub p1108: f64,
    pub p1109: f64,
    pub p1110: f64,
    pub p1111: f64,
    pub p1112: f64,
    pub p1113: f64,
    pub p1114: f64,
    pub p1115: f64,
    pub p1116: f64,
    pub p1117: f64,
    pub p1118: f64,
    pub p1119: f64,
    pub p1120: f64,
    pub p1121: f64,
    pub p1122: f64,
    pub p1123: f64,
    pub p1124: f64,
    pub p1125: f64,
    pub p1126: f64,
    pub p1127: f64,
    pub p1128: f64,
    pub p1129: f64,
    pub p1130: f64,
    pub p1131: f64,
    pub p1132: f64,
    pub p1133: f64,
    pub p1134: f64,
    pub p1135: f64,
    pub p1136: f64,
    pub p1137: f64,
    pub p1138: f64,
    pub p1139: f64,
    pub p1140: f64,
    pub p1141: f64,
    pub p1142: f64,
    pub p1143: f64,
    pub p1144: f64,
    pub p1145: f64,
    pub p1146: f64,
    pub p1147: f64,
    pub p1148: f64,
    pub p1149: f64,
    pub p1150: f64,
    pub p1151: f64,
    pub p1152: f64,
    pub p1153: f64,
    pub p1154: f64,
    pub p1155: f64,
    pub p1156: f64,
    pub p1157: f64,
    pub p1158: f64,
    pub p1159: f64,
    pub p1160: f64,
    pub p1161: f64,
    pub p1162: f64,
    pub p1163: f64,
    pub p1164: f64,
    pub p1165: f64,
    pub p1166: f64,
    pub p1167: f64,
    pub p1168: f64,
    pub p1169: f64,
    pub p1170: f64,
    pub p1171: f64,
    pub p1172: f64,
    pub p1173: f64,
    pub p1174: f64,
    pub p1175: f64,
    pub p1176: f64,
    pub p1177: f64,
    pub p1178: f64,
    pub p1179: f64,
    pub p1180: f64,
    pub p1181: f64,
    pub p1182: f64,
    pub p1183: f64,
    pub p1184: f64,
    pub p1185: f64,
    pub p1186: f64,
    pub p1187: f64,
    pub p1188: f64,
    pub p1189: f64,
    pub p1190: f64,
    pub p1191: f64,
    pub p1192: f64,
    pub p1193: f64,
    pub p1194: f64,
    pub p1195: f64,
    pub p1196: f64,
    pub p1197: f64,
    pub p1198: f64,
    pub p1199: f64,
    pub p1200: f64,
    pub p1201: f64,
    pub p1202: f64,
    pub p1203: f64,
    pub p1204: f64,
    pub p1205: f64,
    pub p1206: f64,
    pub p1207: f64,
    pub p1208: f64,
    pub p1209: f64,
    pub p1210: f64,
    pub p1211: f64,
    pub p1212: f64,
    pub p1213: f64,
    pub p1214: f64,
    pub p1215: f64,
    pub p1216: f64,
    pub p1217: f64,
    pub p1218: f64,
    pub p1219: f64,
    pub p1220: f64,
    pub p1221: f64,
    pub p1222: f64,
    pub p1223: f64,
    pub p1224: f64,
    pub p1225: f64,
    pub p1226: f64,
    pub p1227: f64,
    pub p1228: f64,
    pub p1229: f64,
    pub p1230: f64,
    pub p1231: f64,
    pub p1232: f64,
    pub p1233: f64,
    pub p1234: f64,
    pub p1235: f64,
    pub p1236: f64,
    pub p1237: f64,
    pub p1238: f64,
    pub p1239: f64,
    pub p1240: f64,
    pub p1241: f64,
    pub p1242: f64,
    pub p1243: f64,
    pub p1244: f64,
    pub p1245: f64,
    pub p1246: f64,
    pub p1247: f64,
    pub p1248: f64,
    pub p1249: f64,
    pub p1250: f64,
    pub p1251: f64,
    pub p1252: f64,
    pub p1253: f64,
    pub p1254: f64,
    pub p1255: f64,
    pub p1256: f64,
    pub p1257: f64,
    pub p1258: f64,
    pub p1259: f64,
    pub p1260: f64,
    pub p1261: f64,
    pub p1262: f64,
    pub p1263: f64,
    pub p1264: f64,
    pub p1265: f64,
    pub p1266: f64,
    pub p1267: f64,
    pub p1268: f64,
    pub p1269: f64,
    pub p1270: f64,
    pub p1271: f64,
    pub p1272: f64,
    pub p1273: f64,
    pub p1274: f64,
    pub p1275: f64,
    pub p1276: f64,
    pub p1277: f64,
    pub p1278: f64,
    pub p1279: f64,
    pub p1280: f64,
    pub p1281: f64,
    pub p1282: f64,
    pub p1283: f64,
    pub p1284: f64,
    pub p1285: f64,
    pub p1286: f64,
    pub p1287: f64,
    pub p1288: f64,
    pub p1289: f64,
    pub p1290: f64,
    pub p1291: f64,
    pub p1292: f64,
    pub p1293: f64,
    pub p1294: f64,
    pub p1295: f64,
    pub p1296: f64,
    pub p1297: f64,
    pub p1298: f64,
    pub p1299: f64,
    pub p1300: f64,
    pub p1301: f64,
    pub p1302: f64,
    pub p1303: f64,
    pub p1304: f64,
    pub p1305: f64,
    pub p1306: f64,
    pub p1307: f64,
    pub p1308: f64,
    pub p1309: f64,
    pub p1310: f64,
    pub p1311: f64,
    pub p1312: f64,
    pub p1313: f64,
    pub p1314: f64,
    pub p1315: f64,
    pub p1316: f64,
    pub p1317: f64,
    pub p1318: f64,
    pub p1319: f64,
    pub p1320: f64,
    pub p1321: f64,
    pub p1322: f64,
    pub p1323: f64,
    pub p1324: f64,
    pub p1325: f64,
    pub p1326: f64,
    pub p1327: f64,
    pub p1328: f64,
    pub p1329: f64,
    pub p1330: f64,
    pub p1331: f64,
    pub p1332: f64,
    pub p1333: f64,
    pub p1334: f64,
    pub p1335: f64,
    pub p1336: f64,
    pub p1337: f64,
    pub p1338: f64,
    pub p1339: f64,
    pub p1340: f64,
    pub p1341: f64,
    pub p1342: f64,
    pub p1343: f64,
    pub p1344: f64,
    pub p1345: f64,
    pub p1346: f64,
    pub p1347: f64,
    pub p1348: f64,
    pub p1349: f64,
    pub p1350: f64,
    pub p1351: f64,
    pub p1352: f64,
    pub p1353: f64,
    pub p1354: f64,
    pub p1355: f64,
    pub p1356: f64,
    pub p1357: f64,
    pub p1358: f64,
    pub p1359: f64,
    pub p1360: f64,
    pub p1361: f64,
    pub p1362: f64,
    pub p1363: f64,
    pub p1364: f64,
    pub p1365: f64,
    pub p1366: f64,
    pub p1367: f64,
    pub p1368: f64,
    pub p1369: f64,
    pub p1370: f64,
    pub p1371: f64,
    pub p1372: f64,
    pub p1373: f64,
    pub p1374: f64,
    pub p1375: f64,
    pub p1376: f64,
    pub p1377: f64,
    pub p1378: f64,
    pub p1379: f64,
    pub p1380: f64,
    pub p1381: f64,
    pub p1382: f64,
    pub p1383: f64,
    pub p1384: f64,
    pub p1385: f64,
    pub p1386: f64,
    pub p1387: f64,
    pub p1388: f64,
    pub p1389: f64,
    pub p1390: f64,
    pub p1391: f64,
    pub p1392: f64,
    pub p1393: f64,
    pub p1394: f64,
    pub p1395: f64,
    pub p1396: f64,
    pub p1397: f64,
    pub p1398: f64,
    pub p1399: f64,
    pub p1400: f64,
    pub p1401: f64,
    pub p1402: f64,
    pub p1403: f64,
    pub p1404: f64,
    pub p1405: f64,
    pub p1406: f64,
    pub p1407: f64,
    pub p1408: f64,
    pub p1409: f64,
    pub p1410: f64,
    pub p1411: f64,
    pub p1412: f64,
    pub p1413: f64,
    pub p1414: f64,
    pub p1415: f64,
    pub p1416: f64,
    pub p1417: f64,
    pub p1418: f64,
    pub p1419: f64,
    pub p1420: f64,
    pub p1421: f64,
    pub p1422: f64,
    pub p1423: f64,
    pub p1424: f64,
    pub p1425: f64,
    pub p1426: f64,
    pub p1427: f64,
    pub p1428: f64,
    pub p1429: f64,
    pub p1430: f64,
    pub p1431: f64,
    pub p1432: f64,
    pub p1433: f64,
    pub p1434: f64,
    pub p1435: f64,
    pub p1436: f64,
    pub p1437: f64,
    pub p1438: f64,
    pub p1439: f64,
    pub p1440: f64,
    pub p1441: f64,
    pub p1442: f64,
    pub p1443: f64,
    pub p1444: f64,
    pub p1445: f64,
    pub p1446: f64,
    pub p1447: f64,
    pub p1448: f64,
    pub p1449: f64,
    pub p1450: f64,
    pub p1451: f64,
    pub p1452: f64,
    pub p1453: f64,
    pub p1454: f64,
    pub p1455: f64,
    pub p1456: f64,
    pub p1457: f64,
    pub p1458: f64,
    pub p1459: f64,
    pub p1460: f64,
    pub p1461: f64,
    pub p1462: f64,
    pub p1463: f64,
    pub p1464: f64,
    pub p1465: f64,
    pub p1466: f64,
    pub p1467: f64,
    pub p1468: f64,
    pub p1469: f64,
    pub p1470: f64,
    pub p1471: f64,
    pub p1472: f64,
    pub p1473: f64,
    pub p1474: f64,
    pub p1475: f64,
    pub p1476: f64,
    pub p1477: f64,
    pub p1478: f64,
    pub p1479: f64,
    pub p1480: f64,
    pub p1481: f64,
    pub p1482: f64,
    pub p1483: f64,
    pub p1484: f64,
    pub p1485: f64,
    pub p1486: f64,
    pub p1487: f64,
    pub p1488: f64,
    pub p1489: f64,
    pub p1490: f64,
    pub p1491: f64,
    pub p1492: f64,
    pub p1493: f64,
    pub p1494: f64,
    pub p1495: f64,
    pub p1496: f64,
    pub p1497: f64,
    pub p1498: f64,
    pub p1499: f64,
    pub p1500: f64,
    pub p1501: f64,
    pub p1502: f64,
    pub p1503: f64,
    pub p1504: f64,
    pub p1505: f64,
    pub p1506: f64,
    pub p1507: f64,
    pub p1508: f64,
    pub p1509: f64,
    pub p1510: f64,
    pub p1511: f64,
    pub p1512: f64,
    pub p1513: f64,
    pub p1514: f64,
    pub p1515: f64,
    pub p1516: f64,
    pub p1517: f64,
    pub p1518: f64,
    pub p1519: f64,
    pub p1520: f64,
    pub p1521: f64,
    pub p1522: f64,
    pub p1523: f64,
    pub p1524: f64,
    pub p1525: f64,
    pub p1526: f64,
    pub p1527: f64,
    pub p1528: f64,
    pub p1529: f64,
    pub p1530: f64,
    pub p1531: f64,
    pub p1532: f64,
    pub p1533: f64,
    pub p1534: f64,
    pub p1535: f64,
    pub p1536: f64,
    pub p1537: f64,
    pub p1538: f64,
    pub p1539: f64,
    pub p1540: f64,
    pub p1541: f64,
    pub p1542: f64,
    pub p1543: f64,
    pub p1544: f64,
    pub p1545: f64,
    pub p1546: f64,
    pub p1547: f64,
    pub p1548: f64,
    pub p1549: f64,
    pub p1550: f64,
    pub p1551: f64,
    pub p1552: f64,
    pub p1553: f64,
    pub p1554: f64,
    pub p1555: f64,
    pub p1556: f64,
    pub p1557: f64,
    pub p1558: f64,
    pub p1559: f64,
    pub p1560: f64,
    pub p1561: f64,
    pub p1562: f64,
    pub p1563: f64,
    pub p1564: f64,
    pub p1565: f64,
    pub p1566: f64,
    pub p1567: f64,
    pub p1568: f64,
    pub p1569: f64,
    pub p1570: f64,
    pub p1571: f64,
    pub p1572: f64,
    pub p1573: f64,
    pub p1574: f64,
    pub p1575: f64,
    pub p1576: f64,
    pub p1577: f64,
    pub p1578: f64,
    pub p1579: f64,
    pub p1580: f64,
    pub p1581: f64,
    pub p1582: f64,
    pub p1583: f64,
    pub p1584: f64,
    pub p1585: f64,
    pub p1586: f64,
    pub p1587: f64,
    pub p1588: f64,
    pub p1589: f64,
    pub p1590: f64,
    pub p1591: f64,
    pub p1592: f64,
    pub p1593: f64,
    pub p1594: f64,
    pub p1595: f64,
    pub p1596: f64,
    pub p1597: f64,
    pub p1598: f64,
    pub p1599: f64,
    pub p1600: f64,
    pub p1601: f64,
    pub p1602: f64,
    pub p1603: f64,
    pub p1604: f64,
    pub p1605: f64,
    pub p1606: f64,
    pub p1607: f64,
    pub p1608: f64,
    pub p1609: f64,
    pub p1610: f64,
    pub p1611: f64,
    pub p1612: f64,
    pub p1613: f64,
    pub p1614: f64,
    pub p1615: f64,
    pub p1616: f64,
    pub p1617: f64,
    pub p1618: f64,
    pub p1619: f64,
    pub p1620: f64,
    pub p1621: f64,
    pub p1622: f64,
    pub p1623: f64,
    pub p1624: f64,
    pub p1625: f64,
    pub p1626: f64,
    pub p1627: f64,
    pub p1628: f64,
    pub p1629: f64,
    pub p1630: f64,
    pub p1631: f64,
    pub p1632: f64,
    pub p1633: f64,
    pub p1634: f64,
    pub p1635: f64,
    pub p1636: f64,
    pub p1637: f64,
    pub p1638: f64,
    pub p1639: f64,
    pub p1640: f64,
    pub p1641: f64,
    pub p1642: f64,
    pub p1643: f64,
    pub p1644: f64,
    pub p1645: f64,
    pub p1646: f64,
    pub p1647: f64,
    pub p1648: f64,
    pub p1649: f64,
    pub p1650: f64,
    pub p1651: f64,
    pub p1652: f64,
    pub p1653: f64,
    pub p1654: f64,
    pub p1655: f64,
    pub p1656: f64,
    pub p1657: f64,
    pub p1658: f64,
    pub p1659: f64,
    pub p1660: f64,
    pub p1661: f64,
    pub p1662: f64,
    pub p1663: f64,
    pub p1664: f64,
    pub p1665: f64,
    pub p1666: f64,
    pub p1667: f64,
    pub p1668: f64,
    pub p1669: f64,
    pub p1670: f64,
    pub p1671: f64,
    pub p1672: f64,
    pub p1673: f64,
    pub p1674: f64,
    pub p1675: f64,
    pub p1676: f64,
    pub p1677: f64,
    pub p1678: f64,
    pub p1679: f64,
    pub p1680: f64,
    pub p1681: f64,
    pub p1682: f64,
    pub p1683: f64,
    pub p1684: f64,
    pub p1685: f64,
    pub p1686: f64,
    pub p1687: f64,
    pub p1688: f64,
    pub p1689: f64,
    pub p1690: f64,
    pub p1691: f64,
    pub p1692: f64,
    pub p1693: f64,
    pub p1694: f64,
    pub p1695: f64,
    pub p1696: f64,
    pub p1697: f64,
    pub p1698: f64,
    pub p1699: f64,
    pub p1700: f64,
    pub p1701: f64,
    pub p1702: f64,
    pub p1703: f64,
    pub p1704: f64,
    pub p1705: f64,
    pub p1706: f64,
    pub p1707: f64,
    pub p1708: f64,
    pub p1709: f64,
    pub p1710: f64,
    pub p1711: f64,
    pub p1712: f64,
    pub p1713: f64,
    pub p1714: f64,
    pub p1715: f64,
    pub p1716: f64,
    pub p1717: f64,
    pub p1718: f64,
    pub p1719: f64,
    pub p1720: f64,
    pub p1721: f64,
    pub p1722: f64,
    pub p1723: f64,
    pub p1724: f64,
    pub p1725: f64,
    pub p1726: f64,
    pub p1727: f64,
    pub p1728: f64,
    pub p1729: f64,
    pub p1730: f64,
    pub p1731: f64,
    pub p1732: f64,
    pub p1733: f64,
    pub p1734: f64,
    pub p1735: f64,
    pub p1736: f64,
    pub p1737: f64,
    pub p1738: f64,
    pub p1739: f64,
    pub p1740: f64,
    pub p1741: f64,
    pub p1742: f64,
    pub p1743: f64,
    pub p1744: f64,
    pub p1745: f64,
    pub p1746: f64,
    pub p1747: f64,
    pub p1748: f64,
    pub p1749: f64,
    pub p1750: f64,
    pub p1751: f64,
    pub p1752: f64,
    pub p1753: f64,
    pub p1754: f64,
    pub p1755: f64,
    pub p1756: f64,
    pub p1757: f64,
    pub p1758: f64,
    pub p1759: f64,
    pub p1760: f64,
    pub p1761: f64,
    pub p1762: f64,
    pub p1763: f64,
    pub p1764: f64,
    pub p1765: f64,
    pub p1766: f64,
    pub p1767: f64,
    pub p1768: f64,
    pub p1769: f64,
    pub p1770: f64,
    pub p1771: f64,
    pub p1772: f64,
    pub p1773: f64,
    pub p1774: f64,
    pub p1775: f64,
    pub p1776: f64,
    pub p1777: f64,
    pub p1778: f64,
    pub p1779: f64,
    pub p1780: f64,
    pub p1781: f64,
    pub p1782: f64,
    pub p1783: f64,
    pub p1784: f64,
    pub p1785: f64,
    pub p1786: f64,
    pub p1787: f64,
    pub p1788: f64,
    pub p1789: f64,
    pub p1790: f64,
    pub p1791: f64,
    pub p1792: f64,
    pub p1793: f64,
    pub p1794: f64,
    pub p1795: f64,
    pub p1796: f64,
    pub p1797: f64,
    pub p1798: f64,
    pub p1799: f64,
    pub p1800: f64,
    pub p1801: f64,
    pub p1802: f64,
    pub p1803: f64,
    pub p1804: f64,
    pub p1805: f64,
    pub p1806: f64,
    pub p1807: f64,
    pub p1808: f64,
    pub p1809: f64,
    pub p1810: f64,
    pub p1811: f64,
    pub p1812: f64,
    pub p1813: f64,
    pub p1814: f64,
    pub p1815: f64,
    pub p1816: f64,
    pub p1817: f64,
    pub p1818: f64,
    pub p1819: f64,
    pub p1820: f64,
    pub p1821: f64,
    pub p1822: f64,
    pub p1823: f64,
    pub p1824: f64,
    pub p1825: f64,
    pub p1826: f64,
    pub p1827: f64,
    pub p1828: f64,
    pub p1829: f64,
    pub p1830: f64,
    pub p1831: f64,
    pub p1832: f64,
    pub p1833: f64,
    pub p1834: f64,
    pub p1835: f64,
    pub p1836: f64,
    pub p1837: f64,
    pub p1838: f64,
    pub p1839: f64,
    pub p1840: f64,
    pub p1841: f64,
    pub p1842: f64,
    pub p1843: f64,
    pub p1844: f64,
    pub p1845: f64,
    pub p1846: f64,
    pub p1847: f64,
    pub p1848: f64,
    pub p1849: f64,
    pub p1850: f64,
    pub p1851: f64,
    pub p1852: f64,
    pub p1853: f64,
    pub p1854: f64,
    pub p1855: f64,
    pub p1856: f64,
    pub p1857: f64,
    pub p1858: f64,
    pub p1859: f64,
    pub p1860: f64,
    pub p1861: f64,
    pub p1862: f64,
    pub p1863: f64,
    pub p1864: f64,
    pub p1865: f64,
    pub p1866: f64,
    pub p1867: f64,
    pub p1868: f64,
    pub p1869: f64,
    pub p1870: f64,
    pub p1871: f64,
    pub p1872: f64,
    pub p1873: f64,
    pub p1874: f64,
    pub p1875: f64,
    pub p1876: f64,
    pub p1877: f64,
    pub p1878: f64,
    pub p1879: f64,
    pub p1880: f64,
    pub p1881: f64,
    pub p1882: f64,
    pub p1883: f64,
    pub p1884: f64,
    pub p1885: f64,
    pub p1886: f64,
    pub p1887: f64,
    pub p1888: f64,
    pub p1889: f64,
    pub p1890: f64,
    pub p1891: f64,
    pub p1892: f64,
    pub p1893: f64,
    pub p1894: f64,
    pub p1895: f64,
    pub p1896: f64,
    pub p1897: f64,
    pub p1898: f64,
    pub p1899: f64,
    pub p1900: f64,
    pub p1901: f64,
    pub p1902: f64,
    pub p1903: f64,
    pub p1904: f64,
    pub p1905: f64,
    pub p1906: f64,
    pub p1907: f64,
    pub p1908: f64,
    pub p1909: f64,
    pub p1910: f64,
    pub p1911: f64,
    pub p1912: f64,
    pub p1913: f64,
    pub p1914: f64,
    pub p1915: f64,
    pub p1916: f64,
    pub p1917: f64,
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
            params.p1 = 3e-8;
            params.p2 = 4e-8;
            params.p3 = 1.5e-8;
            params.p4 = 8e-8;
            params.p5 = 1.0;
            params.p6 = 1.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 0.0;
            params.p12 = 0.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 0.0;
            params.p16 = 0.0;
            params.p17 = 0.0;
            params.p18 = 0.0;
            params.p19 = 0.0;
            params.p20 = params.p0;
            validate_parameter("lrsd", params.p20, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 0.0;
            params.p24 = 1.0;
            params.p25 = 1.0;
            params.p26 = 1.0;
            params.p27 = 1.0;
            params.p28 = 0.0;
            params.p29 = 0.0;
            params.p30 = 0.0;
            params.p31 = 0.0;
            params.p32 = 0.0;
            params.p33 = 0.0;
            params.p34 = params.p28;
            validate_finite_parameter("covd", params.p34).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p35 = params.p29;
            validate_finite_parameter("lcovd", params.p35).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p36 = params.p30;
            validate_finite_parameter("ncovd", params.p36).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p37 = params.p31;
            validate_finite_parameter("pcovd", params.p37).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p38 = params.p32;
            validate_finite_parameter("wcovd", params.p38).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p39 = params.p33;
            validate_finite_parameter("p2covd", params.p39).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p40 = 5e-9;
            params.p41 = 2e-9;
            params.p42 = 5e-9;
            params.p43 = 6e-9;
            params.p44 = 0.0;
            params.p45 = 0.0;
            params.p46 = params.p44;
            validate_parameter("dws2", params.p46, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p47 = params.p45;
            validate_parameter("dach2", params.p47, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p48 = params.p44;
            validate_parameter("dws3", params.p48, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p49 = params.p45;
            validate_parameter("dach3", params.p49, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p50 = params.p44;
            validate_parameter("dws4", params.p50, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p51 = params.p45;
            validate_parameter("dach4", params.p51, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p52 = params.p44;
            validate_parameter("dws5", params.p52, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p53 = params.p45;
            validate_parameter("dach5", params.p53, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p54 = params.p44;
            validate_parameter("dws6", params.p54, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p55 = params.p45;
            validate_parameter("dach6", params.p55, None, true, Some((0.0, "0.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p56 = 1.0;
            params.p57 = 0.0;
            params.p58 = 0.0;
            params.p59 = 1.0;
            params.p60 = 1.0;
            params.p61 = 0.0;
            params.p62 = 0.0;
            params.p63 = 0.0;
            params.p64 = 0.0;
            params.p65 = 0.0;
            params.p66 = 0.0;
            params.p67 = 0.0;
            params.p68 = 0.0;
            params.p69 = 0.0;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 0.0;
            params.p74 = 0.0;
            params.p75 = 0.0;
            params.p76 = 0.0;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 0.0;
            params.p80 = 0.0;
            params.p81 = 0.0;
            params.p82 = 1.0;
            params.p83 = 0.0;
            params.p84 = 1.0;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.0;
            params.p88 = 0.0;
            params.p89 = 1e-9;
            params.p90 = 1.2e-9;
            params.p91 = 1.4e-7;
            params.p92 = 3e-8;
            params.p93 = 0.0;
            params.p94 = 0.0;
            params.p95 = 0.0;
            params.p96 = 100000.0;
            params.p97 = 2e26;
            params.p98 = 0.0;
            params.p99 = 0.0;
            params.p100 = 0.0;
            params.p101 = 100000.0;
            params.p102 = 3.9;
            params.p103 = 11.9;
            params.p104 = 4.05;
            params.p105 = 1.1e16;
            params.p106 = 1.12;
            params.p107 = 2.86e25;
            params.p108 = 1e-15;
            params.p109 = 0.0;
            params.p110 = 0.0;
            params.p111 = 0.0;
            params.p112 = 0.0;
            params.p113 = 0.0;
            params.p114 = 0.0;
            params.p115 = 0.0;
            params.p116 = 0.0;
            params.p117 = 0.0;
            params.p118 = 0.0;
            params.p119 = 0.0;
            params.p120 = 0.0;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = 0.0;
            params.p126 = 0.0;
            params.p127 = 0.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.0;
            params.p131 = 0.0;
            params.p132 = 0.0;
            params.p133 = 1e22;
            params.p134 = 0.0;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 4.61;
            params.p138 = 0.0;
            params.p139 = 0.0;
            params.p140 = 0.0;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = -0.2;
            params.p144 = -0.2;
            params.p145 = 0.0;
            params.p146 = 0.0;
            params.p147 = 0.0;
            params.p148 = 0.0;
            params.p149 = 0.0;
            params.p150 = 0.0;
            params.p151 = 0.001;
            validate_parameter("minr", params.p151, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p152 = 0.0;
            params.p153 = 100000.0;
            params.p154 = 0.0;
            params.p155 = 100000.0;
            params.p156 = params.p154;
            validate_finite_parameter("cdscdrn1", params.p156).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p157 = params.p155;
            validate_finite_parameter("cdscdrn2", params.p157).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p158 = 0.0;
            params.p159 = 100000.0;
            params.p160 = 0.0;
            params.p161 = params.p158;
            validate_finite_parameter("eta0n1cv", params.p161).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p162 = params.p159;
            validate_parameter("eta0n2cv", params.p162, Some((1e-5, "1e-5")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p163 = params.p160;
            validate_finite_parameter("eta0ltcv", params.p163).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p164 = 0.0;
            params.p165 = params.p164;
            validate_finite_parameter("teta0cv", params.p165).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p166 = params.p164;
            validate_finite_parameter("teta0r", params.p166).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p167 = 0.0;
            params.p168 = 1e-7;
            params.p169 = 0.0;
            params.p170 = 1e-7;
            params.p171 = 0.0;
            params.p172 = 0.0;
            params.p173 = 0.0;
            params.p174 = 0.0;
            params.p175 = 0.0;
            params.p176 = 0.0;
            params.p177 = 0.0;
            params.p178 = 0.0;
            params.p179 = 0.0;
            params.p180 = 0.0;
            params.p181 = 0.0;
            params.p182 = 0.0;
            params.p183 = 0.0;
            params.p184 = 0.0;
            params.p185 = 0.0;
            params.p186 = 0.0;
            params.p187 = 0.0;
            params.p188 = 0.0;
            params.p189 = 0.0;
            params.p190 = 0.0;
            params.p191 = 0.0;
            params.p192 = 0.0;
            params.p193 = 0.0;
            params.p194 = params.p188;
            validate_finite_parameter("citr", params.p194).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p195 = params.p189;
            validate_finite_parameter("lcitr", params.p195).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p196 = params.p190;
            validate_finite_parameter("ncitr", params.p196).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p197 = params.p191;
            validate_finite_parameter("pcitr", params.p197).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p198 = params.p192;
            validate_finite_parameter("wcitr", params.p198).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p199 = params.p193;
            validate_finite_parameter("p2citr", params.p199).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p200 = 0.007;
            params.p201 = 0.0;
            params.p202 = 0.0;
            params.p203 = 0.0;
            params.p204 = 0.0;
            params.p205 = 0.0;
            params.p206 = 0.007;
            params.p207 = 0.0;
            params.p208 = 0.0;
            params.p209 = 0.0;
            params.p210 = 0.0;
            params.p211 = 0.0;
            params.p212 = params.p206;
            validate_finite_parameter("cdscdr", params.p212).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p213 = params.p207;
            validate_finite_parameter("lcdscdr", params.p213).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p214 = params.p208;
            validate_finite_parameter("ncdscdr", params.p214).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p215 = params.p209;
            validate_finite_parameter("pcdscdr", params.p215).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p216 = params.p210;
            validate_finite_parameter("wcdscdr", params.p216).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p217 = params.p211;
            validate_finite_parameter("p2cdscdr", params.p217).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p218 = 0.0;
            params.p219 = 0.0;
            params.p220 = 0.0;
            params.p221 = 0.0;
            params.p222 = 0.0;
            params.p223 = 0.0;
            params.p224 = 0.6;
            params.p225 = 0.0;
            params.p226 = 0.0;
            params.p227 = 0.0;
            params.p228 = 0.0;
            params.p229 = 0.0;
            params.p230 = params.p224;
            validate_finite_parameter("dvt1ss", params.p230).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p231 = params.p225;
            validate_finite_parameter("ldvt1ss", params.p231).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p232 = params.p226;
            validate_finite_parameter("ndvt1ss", params.p232).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p233 = params.p227;
            validate_finite_parameter("pdvt1ss", params.p233).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p234 = params.p228;
            validate_finite_parameter("wdvt1ss", params.p234).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p235 = params.p229;
            validate_finite_parameter("p2dvt1ss", params.p235).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p236 = 0.05;
            params.p237 = 0.0;
            params.p238 = 0.0;
            params.p239 = 0.0;
            params.p240 = 0.0;
            params.p241 = 0.0;
            params.p242 = 0.6;
            params.p243 = 0.0;
            params.p244 = 0.0;
            params.p245 = 0.0;
            params.p246 = 0.0;
            params.p247 = 0.0;
            params.p248 = 0.0;
            params.p249 = 0.0;
            params.p250 = 0.0;
            params.p251 = 0.0;
            params.p252 = 0.0;
            params.p253 = 0.0;
            params.p254 = params.p242;
            validate_finite_parameter("eta0r", params.p254).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p255 = params.p243;
            validate_finite_parameter("leta0r", params.p255).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p256 = params.p244;
            validate_finite_parameter("neta0r", params.p256).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p257 = params.p245;
            validate_finite_parameter("peta0r", params.p257).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p258 = params.p246;
            validate_finite_parameter("weta0r", params.p258).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p259 = params.p247;
            validate_finite_parameter("p2eta0r", params.p259).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p260 = params.p242;
            validate_finite_parameter("eta0cv", params.p260).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p261 = params.p243;
            validate_finite_parameter("leta0cv", params.p261).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p262 = params.p244;
            validate_finite_parameter("neta0cv", params.p262).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p263 = params.p245;
            validate_finite_parameter("peta0cv", params.p263).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p264 = params.p246;
            validate_finite_parameter("weta0cv", params.p264).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p265 = params.p247;
            validate_finite_parameter("p2eta0cv", params.p265).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p266 = 1.06;
            params.p267 = 0.0;
            params.p268 = 0.0;
            params.p269 = 0.0;
            params.p270 = 0.0;
            params.p271 = 0.0;
            params.p272 = 0.0;
            params.p273 = 0.0;
            params.p274 = 0.0;
            params.p275 = 0.0;
            params.p276 = 0.0;
            params.p277 = 0.0;
            params.p278 = 5e-9;
            params.p279 = 0.0;
            params.p280 = 0.0;
            params.p281 = 0.0;
            params.p282 = 0.0;
            params.p283 = 0.0;
            params.p284 = 0.0;
            params.p285 = 0.0;
            params.p286 = 0.0;
            params.p287 = 0.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = params.p284;
            validate_finite_parameter("dvtshiftr", params.p290).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p291 = params.p285;
            validate_finite_parameter("ldvtshiftr", params.p291).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p292 = params.p286;
            validate_finite_parameter("ndvtshiftr", params.p292).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p293 = params.p287;
            validate_finite_parameter("pdvtshiftr", params.p293).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p294 = params.p288;
            validate_finite_parameter("wdvtshiftr", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p295 = params.p289;
            validate_finite_parameter("p2dvtshiftr", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p296 = 0.0;
            params.p297 = 0.0;
            params.p298 = 0.0;
            params.p299 = 0.0;
            params.p300 = 0.0;
            params.p301 = 0.0;
            params.p302 = 0.0;
            params.p303 = 0.0;
            params.p304 = 0.0;
            params.p305 = 0.0;
            params.p306 = 0.0;
            params.p307 = 0.0;
            params.p308 = 1.0;
            params.p309 = 0.0;
            params.p310 = 0.0;
            params.p311 = 0.0;
            params.p312 = 0.0;
            params.p313 = 0.0;
            params.p314 = 0.0;
            params.p315 = 0.0;
            params.p316 = 0.0;
            params.p317 = 0.0;
            params.p318 = 0.0;
            params.p319 = 0.0;
            params.p320 = params.p308;
            validate_finite_parameter("k2si", params.p320).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p321 = params.p309;
            validate_finite_parameter("lk2si", params.p321).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p322 = params.p310;
            validate_finite_parameter("nk2si", params.p322).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p323 = params.p311;
            validate_finite_parameter("pk2si", params.p323).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p324 = params.p312;
            validate_finite_parameter("wk2si", params.p324).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p325 = params.p313;
            validate_finite_parameter("p2k2si", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p326 = params.p314;
            validate_finite_parameter("k2si1", params.p326).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p327 = params.p315;
            validate_finite_parameter("lk2si1", params.p327).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p328 = params.p316;
            validate_finite_parameter("nk2si1", params.p328).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p329 = params.p317;
            validate_finite_parameter("pk2si1", params.p329).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p330 = params.p318;
            validate_finite_parameter("wk2si1", params.p330).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p331 = params.p319;
            validate_finite_parameter("p2k2si1", params.p331).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p344 = params.p332;
            validate_finite_parameter("k2sisat", params.p344).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p345 = params.p333;
            validate_finite_parameter("lk2sisat", params.p345).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p346 = params.p334;
            validate_finite_parameter("nk2sisat", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p347 = params.p335;
            validate_finite_parameter("pk2sisat", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p348 = params.p336;
            validate_finite_parameter("wk2sisat", params.p348).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p349 = params.p337;
            validate_finite_parameter("p2k2sisat", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p350 = params.p338;
            validate_finite_parameter("k2sisat1", params.p350).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p351 = params.p339;
            validate_finite_parameter("lk2sisat1", params.p351).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p352 = params.p340;
            validate_finite_parameter("nk2sisat1", params.p352).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p353 = params.p341;
            validate_finite_parameter("pk2sisat1", params.p353).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p354 = params.p342;
            validate_finite_parameter("wk2sisat1", params.p354).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p355 = params.p343;
            validate_finite_parameter("p2k2sisat1", params.p355).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p356 = 0.7;
            params.p357 = 0.0;
            params.p358 = 0.0;
            params.p359 = 0.0;
            params.p360 = 0.0;
            params.p361 = 0.0;
            params.p362 = 1e-6;
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
            params.p399 = 1.2e-8;
            params.p400 = 0.001;
            params.p401 = 0.001;
            params.p402 = 0.66;
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
            params.p421 = 0.66;
            params.p422 = 0.0;
            params.p423 = 0.0;
            params.p424 = 0.0;
            params.p425 = 0.0;
            params.p426 = 0.0;
            params.p427 = 0.0;
            params.p428 = 0.0;
            params.p429 = 100000.0;
            params.p430 = 0.0;
            params.p431 = 1e-7;
            params.p432 = params.p428;
            validate_finite_parameter("vsat1n1", params.p432).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p433 = params.p429;
            validate_finite_parameter("vsat1n2", params.p433).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p434 = params.p432;
            validate_finite_parameter("vsat1rn1", params.p434).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p435 = params.p433;
            validate_finite_parameter("vsat1rn2", params.p435).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p436 = params.p430;
            validate_finite_parameter("avsat1", params.p436).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p437 = params.p431;
            validate_finite_parameter("bvsat1", params.p437).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p438 = 0.0;
            params.p439 = 1.0;
            params.p440 = params.p430;
            validate_finite_parameter("avsatcv", params.p440).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p441 = params.p431;
            validate_finite_parameter("bvsatcv", params.p441).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p442 = params.p438;
            validate_finite_parameter("apsatcv", params.p442).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p443 = params.p439;
            validate_finite_parameter("bpsatcv", params.p443).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p444 = 0.0;
            params.p445 = 1.0;
            params.p446 = params.p444;
            validate_finite_parameter("amexpr", params.p446).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p447 = params.p445;
            validate_finite_parameter("bmexpr", params.p447).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p448 = 0.0;
            params.p449 = 1e-7;
            params.p450 = 0.0;
            params.p451 = -4e-6;
            params.p452 = params.p450;
            validate_finite_parameter("tmexpr", params.p452).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p453 = 0.01;
            params.p454 = 85000.0;
            params.p455 = 85000.0;
            params.p456 = 0.0;
            params.p457 = 0.0;
            params.p458 = 0.0;
            params.p459 = 0.0;
            params.p460 = 0.0;
            params.p461 = params.p455;
            validate_finite_parameter("vsatr", params.p461).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p462 = params.p456;
            validate_finite_parameter("lvsatr", params.p462).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p463 = params.p457;
            validate_finite_parameter("nvsatr", params.p463).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p464 = params.p458;
            validate_finite_parameter("pvsatr", params.p464).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p465 = params.p459;
            validate_finite_parameter("wvsatr", params.p465).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p466 = params.p460;
            validate_finite_parameter("p2vsatr", params.p466).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p467 = params.p455;
            validate_finite_parameter("vsat1", params.p467).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p468 = params.p456;
            validate_finite_parameter("lvsat1", params.p468).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p469 = params.p457;
            validate_finite_parameter("nvsat1", params.p469).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p470 = params.p458;
            validate_finite_parameter("pvsat1", params.p470).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p471 = params.p459;
            validate_finite_parameter("wvsat1", params.p471).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p472 = params.p460;
            validate_finite_parameter("p2vsat1", params.p472).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p473 = params.p467;
            validate_finite_parameter("vsat1r", params.p473).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p474 = params.p468;
            validate_finite_parameter("lvsat1r", params.p474).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p475 = params.p469;
            validate_finite_parameter("nvsat1r", params.p475).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p476 = params.p470;
            validate_finite_parameter("pvsat1r", params.p476).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p477 = params.p471;
            validate_finite_parameter("wvsat1r", params.p477).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p478 = params.p472;
            validate_finite_parameter("p2vsat1r", params.p478).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p479 = 1.0;
            params.p480 = 0.0;
            params.p481 = 0.0;
            params.p482 = 0.0;
            params.p483 = 0.0;
            params.p484 = 0.0;
            params.p485 = 2.0;
            params.p486 = 0.0;
            params.p487 = 0.0;
            params.p488 = 0.0;
            params.p489 = 0.0;
            params.p490 = 0.0;
            params.p491 = 1.0;
            params.p492 = 1.0;
            params.p493 = 0.0;
            params.p494 = 0.0;
            params.p495 = 0.0;
            params.p496 = 0.0;
            params.p497 = 0.0;
            params.p498 = -0.0002;
            params.p499 = -2e-7;
            params.p500 = params.p492;
            validate_finite_parameter("ksativr", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p501 = params.p493;
            validate_finite_parameter("lksativr", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p502 = params.p494;
            validate_finite_parameter("nksativr", params.p502).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p503 = params.p495;
            validate_finite_parameter("pksativr", params.p503).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p504 = params.p496;
            validate_finite_parameter("wksativr", params.p504).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p505 = params.p497;
            validate_finite_parameter("p2ksativr", params.p505).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p506 = params.p455;
            validate_finite_parameter("vsatcv", params.p506).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p507 = params.p456;
            validate_finite_parameter("lvsatcv", params.p507).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p508 = params.p457;
            validate_finite_parameter("nvsatcv", params.p508).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p509 = params.p458;
            validate_finite_parameter("pvsatcv", params.p509).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p510 = params.p459;
            validate_finite_parameter("wvsatcv", params.p510).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p511 = params.p460;
            validate_finite_parameter("p2vsatcv", params.p511).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p512 = 1.0;
            params.p513 = 0.0;
            params.p514 = 0.0;
            params.p515 = 0.0;
            params.p516 = 0.0;
            params.p517 = 0.0;
            params.p518 = params.p479;
            validate_finite_parameter("deltavsatcv", params.p518).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p519 = params.p480;
            validate_finite_parameter("ldeltavsatcv", params.p519).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p520 = params.p481;
            validate_finite_parameter("ndeltavsatcv", params.p520).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p521 = params.p482;
            validate_finite_parameter("pdeltavsatcv", params.p521).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p522 = params.p483;
            validate_finite_parameter("wdeltavsatcv", params.p522).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p523 = params.p484;
            validate_finite_parameter("p2deltavsatcv", params.p523).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p524 = params.p485;
            validate_finite_parameter("psatcv", params.p524).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p525 = params.p486;
            validate_finite_parameter("lpsatcv", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p526 = params.p487;
            validate_finite_parameter("npsatcv", params.p526).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p527 = params.p488;
            validate_finite_parameter("ppsatcv", params.p527).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p528 = params.p489;
            validate_finite_parameter("wpsatcv", params.p528).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p529 = params.p490;
            validate_finite_parameter("p2psatcv", params.p529).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p530 = 4.0;
            params.p531 = 4.0;
            params.p532 = 0.0;
            params.p533 = 0.0;
            params.p534 = 0.0;
            params.p535 = 0.0;
            params.p536 = 0.0;
            params.p537 = params.p531;
            validate_finite_parameter("mexpr", params.p537).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p538 = params.p532;
            validate_finite_parameter("lmexpr", params.p538).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p539 = params.p533;
            validate_finite_parameter("nmexpr", params.p539).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p540 = params.p534;
            validate_finite_parameter("pmexpr", params.p540).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p541 = params.p535;
            validate_finite_parameter("wmexpr", params.p541).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p542 = params.p536;
            validate_finite_parameter("p2mexpr", params.p542).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p543 = 0.0;
            params.p544 = 0.0;
            params.p545 = 0.0;
            params.p546 = 0.0;
            params.p547 = 0.0;
            params.p548 = 0.0;
            params.p549 = params.p543;
            validate_finite_parameter("ptwgr", params.p549).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p550 = params.p544;
            validate_finite_parameter("lptwgr", params.p550).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p551 = params.p545;
            validate_finite_parameter("nptwgr", params.p551).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p552 = params.p546;
            validate_finite_parameter("pptwgr", params.p552).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p553 = params.p547;
            validate_finite_parameter("wptwgr", params.p553).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p554 = params.p548;
            validate_finite_parameter("p2ptwgr", params.p554).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p555 = -0.00156;
            params.p556 = 0.0;
            params.p557 = 0.0;
            params.p558 = 0.0;
            params.p559 = 0.0;
            params.p560 = 0.0;
            params.p561 = 2e-6;
            params.p562 = params.p555;
            validate_finite_parameter("atr", params.p562).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p563 = params.p556;
            validate_finite_parameter("latr", params.p563).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p564 = params.p557;
            validate_finite_parameter("natr", params.p564).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p565 = params.p558;
            validate_finite_parameter("patr", params.p565).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p566 = params.p559;
            validate_finite_parameter("watr", params.p566).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p567 = params.p560;
            validate_finite_parameter("p2atr", params.p567).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p568 = params.p555;
            validate_finite_parameter("atcv", params.p568).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p569 = params.p556;
            validate_finite_parameter("latcv", params.p569).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p570 = params.p557;
            validate_finite_parameter("natcv", params.p570).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p571 = params.p558;
            validate_finite_parameter("patcv", params.p571).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p572 = params.p559;
            validate_finite_parameter("watcv", params.p572).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p573 = params.p560;
            validate_finite_parameter("p2atcv", params.p573).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p574 = params.p561;
            validate_finite_parameter("at2cv", params.p574).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p575 = 0.004;
            params.p576 = 0.0;
            params.p577 = 0.0;
            params.p578 = 0.0;
            params.p579 = 0.0;
            params.p580 = 0.0;
            params.p581 = 0.0;
            params.p582 = params.p581;
            validate_finite_parameter("u0n1cv", params.p582).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p583 = params.p581;
            validate_finite_parameter("u0n1r", params.p583).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p584 = 100000.0;
            params.p585 = params.p584;
            validate_finite_parameter("u0n2cv", params.p585).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p586 = params.p584;
            validate_finite_parameter("u0n2r", params.p586).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p587 = 0.0;
            params.p588 = 0.0;
            params.p589 = 1.0;
            params.p590 = params.p589;
            validate_finite_parameter("lpar", params.p590).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p591 = 0.0;
            params.p592 = params.p591;
            validate_finite_parameter("auar", params.p592).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p593 = 1e-7;
            params.p594 = params.p593;
            validate_finite_parameter("buar", params.p594).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p595 = 0.0;
            params.p596 = params.p595;
            validate_finite_parameter("aeur", params.p596).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p597 = 1e-7;
            params.p598 = params.p597;
            validate_finite_parameter("beur", params.p598).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p599 = 0.0;
            params.p600 = params.p599;
            validate_finite_parameter("audr", params.p600).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p601 = 5e-8;
            params.p602 = params.p601;
            validate_finite_parameter("budr", params.p602).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p603 = 0.0;
            params.p604 = 0.01;
            params.p605 = 0.03;
            params.p606 = 0.0;
            params.p607 = 0.0;
            params.p608 = 0.0;
            params.p609 = 0.0;
            params.p610 = 0.0;
            params.p611 = params.p605;
            validate_finite_parameter("u0r", params.p611).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p612 = params.p606;
            validate_finite_parameter("lu0r", params.p612).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p613 = params.p607;
            validate_finite_parameter("nu0r", params.p613).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p614 = params.p608;
            validate_finite_parameter("pu0r", params.p614).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p615 = params.p609;
            validate_finite_parameter("wu0r", params.p615).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p616 = params.p610;
            validate_finite_parameter("p2u0r", params.p616).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p617 = params.p605;
            validate_finite_parameter("u0cv", params.p617).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p618 = params.p606;
            validate_finite_parameter("lu0cv", params.p618).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p619 = params.p607;
            validate_finite_parameter("nu0cv", params.p619).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p620 = params.p608;
            validate_finite_parameter("pu0cv", params.p620).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p621 = params.p609;
            validate_finite_parameter("wu0cv", params.p621).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p622 = params.p610;
            validate_finite_parameter("p2u0cv", params.p622).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p623 = 2.0;
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
            params.p635 = params.p629;
            validate_finite_parameter("upr", params.p635).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p636 = params.p630;
            validate_finite_parameter("lupr", params.p636).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p637 = params.p631;
            validate_finite_parameter("nupr", params.p637).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p638 = params.p632;
            validate_finite_parameter("pupr", params.p638).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p639 = params.p633;
            validate_finite_parameter("wupr", params.p639).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p640 = params.p634;
            validate_finite_parameter("p2upr", params.p640).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p641 = 0.3;
            params.p642 = 0.0;
            params.p643 = 0.0;
            params.p644 = 0.0;
            params.p645 = 0.0;
            params.p646 = 0.0;
            params.p647 = params.p641;
            validate_finite_parameter("uar", params.p647).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p648 = params.p642;
            validate_finite_parameter("luar", params.p648).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p649 = params.p643;
            validate_finite_parameter("nuar", params.p649).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p650 = params.p644;
            validate_finite_parameter("puar", params.p650).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p651 = params.p645;
            validate_finite_parameter("wuar", params.p651).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p652 = params.p646;
            validate_finite_parameter("p2uar", params.p652).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p653 = params.p641;
            validate_finite_parameter("uacv", params.p653).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p654 = params.p642;
            validate_finite_parameter("luacv", params.p654).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p655 = params.p643;
            validate_finite_parameter("nuacv", params.p655).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p656 = params.p644;
            validate_finite_parameter("puacv", params.p656).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p657 = params.p645;
            validate_finite_parameter("wuacv", params.p657).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p658 = params.p646;
            validate_finite_parameter("p2uacv", params.p658).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p659 = 0.0;
            params.p660 = 0.0;
            params.p661 = 0.0;
            params.p662 = 0.0;
            params.p663 = 0.0;
            params.p664 = 0.0;
            params.p665 = params.p659;
            validate_finite_parameter("ucr", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p666 = params.p660;
            validate_finite_parameter("lucr", params.p666).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p667 = params.p661;
            validate_finite_parameter("nucr", params.p667).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p668 = params.p662;
            validate_finite_parameter("pucr", params.p668).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p669 = params.p663;
            validate_finite_parameter("wucr", params.p669).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p670 = params.p664;
            validate_finite_parameter("p2ucr", params.p670).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p671 = params.p659;
            validate_finite_parameter("uccv", params.p671).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p672 = params.p660;
            validate_finite_parameter("luccv", params.p672).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p673 = params.p661;
            validate_finite_parameter("nuccv", params.p673).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p674 = params.p662;
            validate_finite_parameter("puccv", params.p674).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p675 = params.p663;
            validate_finite_parameter("wuccv", params.p675).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p676 = params.p664;
            validate_finite_parameter("p2uccv", params.p676).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p677 = 2.5;
            params.p678 = 0.0;
            params.p679 = 0.0;
            params.p680 = 0.0;
            params.p681 = 0.0;
            params.p682 = 0.0;
            params.p683 = params.p677;
            validate_finite_parameter("eur", params.p683).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p684 = params.p678;
            validate_finite_parameter("leur", params.p684).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p685 = params.p679;
            validate_finite_parameter("neur", params.p685).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p686 = params.p680;
            validate_finite_parameter("peur", params.p686).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p687 = params.p681;
            validate_finite_parameter("weur", params.p687).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p688 = params.p682;
            validate_finite_parameter("p2eur", params.p688).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p689 = 0.0;
            params.p690 = 0.0;
            params.p691 = 0.0;
            params.p692 = 0.0;
            params.p693 = 0.0;
            params.p694 = 0.0;
            params.p695 = params.p689;
            validate_finite_parameter("udr", params.p695).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p696 = params.p690;
            validate_finite_parameter("ludr", params.p696).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p697 = params.p691;
            validate_finite_parameter("nudr", params.p697).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p698 = params.p692;
            validate_finite_parameter("pudr", params.p698).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p699 = params.p693;
            validate_finite_parameter("wudr", params.p699).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p700 = params.p694;
            validate_finite_parameter("p2udr", params.p700).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p701 = params.p689;
            validate_finite_parameter("udcv", params.p701).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p702 = params.p690;
            validate_finite_parameter("ludcv", params.p702).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p703 = params.p691;
            validate_finite_parameter("nudcv", params.p703).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p704 = params.p692;
            validate_finite_parameter("pudcv", params.p704).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p705 = params.p693;
            validate_finite_parameter("wudcv", params.p705).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p706 = params.p694;
            validate_finite_parameter("p2udcv", params.p706).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p707 = 1.0;
            params.p708 = 0.0;
            params.p709 = 0.0;
            params.p710 = 0.0;
            params.p711 = 0.0;
            params.p712 = 0.0;
            params.p713 = 2e-5;
            params.p714 = 0.0;
            params.p715 = 0.0;
            params.p716 = 0.0;
            params.p717 = 0.0;
            params.p718 = 0.0;
            params.p719 = -10.0;
            params.p720 = 0.0;
            params.p721 = 0.0;
            params.p722 = 0.0;
            params.p723 = 0.0;
            params.p724 = 0.0;
            params.p725 = -2e-5;
            params.p726 = 0.0;
            params.p727 = 0.0;
            params.p728 = 0.0;
            params.p729 = 0.0;
            params.p730 = 0.0;
            params.p731 = -10.0;
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
            params.p743 = params.p737;
            validate_finite_parameter("uter", params.p743).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p744 = params.p738;
            validate_finite_parameter("luter", params.p744).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p745 = params.p739;
            validate_finite_parameter("nuter", params.p745).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p746 = params.p740;
            validate_finite_parameter("puter", params.p746).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p747 = params.p741;
            validate_finite_parameter("wuter", params.p747).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p748 = params.p742;
            validate_finite_parameter("p2uter", params.p748).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p749 = params.p737;
            validate_finite_parameter("utecv", params.p749).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p750 = params.p738;
            validate_finite_parameter("lutecv", params.p750).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p751 = params.p739;
            validate_finite_parameter("nutecv", params.p751).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p752 = params.p740;
            validate_finite_parameter("putecv", params.p752).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p753 = params.p741;
            validate_finite_parameter("wutecv", params.p753).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p754 = params.p742;
            validate_finite_parameter("p2utecv", params.p754).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p755 = -0.4;
            params.p756 = 0.0;
            params.p757 = 0.0;
            params.p758 = 0.0;
            params.p759 = 0.0;
            params.p760 = 0.0;
            params.p761 = params.p755;
            validate_finite_parameter("ute1cv", params.p761).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p762 = params.p756;
            validate_finite_parameter("lute1cv", params.p762).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p763 = params.p757;
            validate_finite_parameter("nute1cv", params.p763).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p764 = params.p758;
            validate_finite_parameter("pute1cv", params.p764).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p765 = params.p759;
            validate_finite_parameter("wute1cv", params.p765).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p766 = params.p760;
            validate_finite_parameter("p2ute1cv", params.p766).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p767 = -0.0015;
            params.p768 = 0.0;
            params.p769 = 0.0;
            params.p770 = 0.0;
            params.p771 = 0.0;
            params.p772 = 0.0;
            params.p773 = params.p767;
            validate_finite_parameter("utlr", params.p773).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p774 = params.p768;
            validate_finite_parameter("lutlr", params.p774).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p775 = params.p769;
            validate_finite_parameter("nutlr", params.p775).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p776 = params.p770;
            validate_finite_parameter("putlr", params.p776).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p777 = params.p771;
            validate_finite_parameter("wutlr", params.p777).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p778 = params.p772;
            validate_finite_parameter("p2utlr", params.p778).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p779 = params.p767;
            validate_finite_parameter("utlcv", params.p779).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p780 = params.p768;
            validate_finite_parameter("lutlcv", params.p780).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p781 = params.p769;
            validate_finite_parameter("nutlcv", params.p781).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p782 = params.p770;
            validate_finite_parameter("putlcv", params.p782).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p783 = params.p771;
            validate_finite_parameter("wutlcv", params.p783).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p784 = params.p772;
            validate_finite_parameter("p2utlcv", params.p784).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p785 = 0.0;
            params.p786 = 0.0;
            params.p787 = 0.0;
            params.p788 = 0.0;
            params.p789 = 0.0;
            params.p790 = 0.0;
            params.p791 = 0.001032;
            params.p792 = 0.0;
            params.p793 = 0.0;
            params.p794 = 0.0;
            params.p795 = 0.0;
            params.p796 = 0.0;
            params.p797 = params.p791;
            validate_finite_parameter("ua1r", params.p797).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p798 = params.p792;
            validate_finite_parameter("lua1r", params.p798).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p799 = params.p793;
            validate_finite_parameter("nua1r", params.p799).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p800 = params.p794;
            validate_finite_parameter("pua1r", params.p800).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p801 = params.p795;
            validate_finite_parameter("wua1r", params.p801).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p802 = params.p796;
            validate_finite_parameter("p2ua1r", params.p802).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p803 = params.p791;
            validate_finite_parameter("ua1cv", params.p803).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p804 = params.p792;
            validate_finite_parameter("lua1cv", params.p804).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p805 = params.p793;
            validate_finite_parameter("nua1cv", params.p805).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p806 = params.p794;
            validate_finite_parameter("pua1cv", params.p806).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p807 = params.p795;
            validate_finite_parameter("wua1cv", params.p807).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p808 = params.p796;
            validate_finite_parameter("p2ua1cv", params.p808).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p809 = -0.04;
            params.p810 = 0.0;
            params.p811 = 0.0;
            params.p812 = 0.0;
            params.p813 = 0.0;
            params.p814 = 0.0;
            params.p815 = params.p809;
            validate_finite_parameter("ua2cv", params.p815).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p816 = params.p810;
            validate_finite_parameter("lua2cv", params.p816).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p817 = params.p811;
            validate_finite_parameter("nua2cv", params.p817).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p818 = params.p812;
            validate_finite_parameter("pua2cv", params.p818).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p819 = params.p813;
            validate_finite_parameter("wua2cv", params.p819).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p820 = params.p814;
            validate_finite_parameter("p2ua2cv", params.p820).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p821 = -0.001;
            params.p822 = 0.0;
            params.p823 = 0.0;
            params.p824 = 0.0;
            params.p825 = 0.0;
            params.p826 = 0.0;
            params.p827 = 5.6e-11;
            params.p828 = 0.0;
            params.p829 = 0.0;
            params.p830 = 0.0;
            params.p831 = 0.0;
            params.p832 = 0.0;
            params.p833 = params.p827;
            validate_finite_parameter("uc1r", params.p833).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p834 = params.p828;
            validate_finite_parameter("luc1r", params.p834).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p835 = params.p829;
            validate_finite_parameter("nuc1r", params.p835).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p836 = params.p830;
            validate_finite_parameter("puc1r", params.p836).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p837 = params.p831;
            validate_finite_parameter("wuc1r", params.p837).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p838 = params.p832;
            validate_finite_parameter("p2uc1r", params.p838).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p839 = params.p827;
            validate_finite_parameter("uc1cv", params.p839).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p840 = params.p828;
            validate_finite_parameter("luc1cv", params.p840).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p841 = params.p829;
            validate_finite_parameter("nuc1cv", params.p841).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p842 = params.p830;
            validate_finite_parameter("puc1cv", params.p842).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p843 = params.p831;
            validate_finite_parameter("wuc1cv", params.p843).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p844 = params.p832;
            validate_finite_parameter("p2uc1cv", params.p844).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p845 = 0.0;
            params.p846 = 0.0;
            params.p847 = 0.0;
            params.p848 = 0.0;
            params.p849 = 0.0;
            params.p850 = 0.0;
            params.p851 = params.p845;
            validate_finite_parameter("ud1r", params.p851).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p852 = params.p846;
            validate_finite_parameter("lud1r", params.p852).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p853 = params.p847;
            validate_finite_parameter("nud1r", params.p853).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p854 = params.p848;
            validate_finite_parameter("pud1r", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p855 = params.p849;
            validate_finite_parameter("wud1r", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p856 = params.p850;
            validate_finite_parameter("p2ud1r", params.p856).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p857 = params.p845;
            validate_finite_parameter("ud1cv", params.p857).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p858 = params.p846;
            validate_finite_parameter("lud1cv", params.p858).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p859 = params.p847;
            validate_finite_parameter("nud1cv", params.p859).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p860 = params.p848;
            validate_finite_parameter("pud1cv", params.p860).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p861 = params.p849;
            validate_finite_parameter("wud1cv", params.p861).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p862 = params.p850;
            validate_finite_parameter("p2ud1cv", params.p862).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p863 = -0.04;
            params.p864 = 0.0;
            params.p865 = 0.0;
            params.p866 = 0.0;
            params.p867 = 0.0;
            params.p868 = 0.0;
            params.p869 = params.p863;
            validate_finite_parameter("ud2cv", params.p869).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p870 = params.p864;
            validate_finite_parameter("lud2cv", params.p870).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p871 = params.p865;
            validate_finite_parameter("nud2cv", params.p871).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p872 = params.p866;
            validate_finite_parameter("pud2cv", params.p872).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p873 = params.p867;
            validate_finite_parameter("wud2cv", params.p873).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p874 = params.p868;
            validate_finite_parameter("p2ud2cv", params.p874).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p875 = -0.004775;
            params.p876 = 0.0;
            params.p877 = 0.0;
            params.p878 = 0.0;
            params.p879 = 0.0;
            params.p880 = 0.0;
            params.p881 = -0.04;
            params.p882 = 0.0;
            params.p883 = 0.0;
            params.p884 = 0.0;
            params.p885 = 0.0;
            params.p886 = 0.0;
            params.p887 = 0.0;
            params.p888 = 0.0;
            params.p889 = params.p623;
            validate_finite_parameter("etamobthin", params.p889).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p890 = 7.5e-9;
            params.p891 = 0.1;
            params.p892 = params.p641;
            validate_finite_parameter("uathin", params.p892).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p893 = 9e-9;
            params.p894 = 0.09;
            params.p895 = 6.4e-9;
            params.p896 = 0.2;
            params.p897 = params.p677;
            validate_finite_parameter("euthin", params.p897).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p898 = 3.5;
            params.p899 = 6e-9;
            params.p900 = 0.2;
            params.p901 = params.p689;
            validate_finite_parameter("udthin", params.p901).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p902 = 8.1e-9;
            params.p903 = 1.3;
            params.p904 = 1.5;
            params.p905 = 1.1;
            params.p906 = 26.6;
            params.p907 = 4.0;
            params.p908 = 0.0;
            params.p909 = 0.0;
            params.p910 = 1e-7;
            params.p911 = 0.0;
            params.p912 = 0.0;
            params.p913 = 1e-7;
            params.p914 = 0.0;
            params.p915 = 0.0;
            params.p916 = 1e-7;
            params.p917 = 0.0;
            params.p918 = params.p917;
            validate_parameter("rsdrr", params.p918, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p919 = params.p917;
            validate_parameter("rddr", params.p919, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p920 = params.p919;
            validate_parameter("rddrr", params.p920, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p921 = 1.0;
            params.p922 = params.p921;
            validate_finite_parameter("prddr", params.p922).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p923 = 0.0;
            params.p924 = params.p923;
            validate_finite_parameter("trddr", params.p924).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p925 = 100.0;
            params.p926 = 0.0;
            params.p927 = 0.0;
            params.p928 = 0.0;
            params.p929 = 0.0;
            params.p930 = 0.0;
            params.p931 = 50.0;
            params.p932 = 0.0;
            params.p933 = 0.0;
            params.p934 = 0.0;
            params.p935 = 0.0;
            params.p936 = 0.0;
            params.p937 = 50.0;
            params.p938 = 0.0;
            params.p939 = 0.0;
            params.p940 = 0.0;
            params.p941 = 0.0;
            params.p942 = 0.0;
            params.p943 = 0.0;
            params.p944 = 0.0;
            params.p945 = 0.0;
            params.p946 = 0.0;
            params.p947 = 0.0;
            params.p948 = 0.0;
            params.p949 = params.p943;
            validate_finite_parameter("prwgd", params.p949).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p950 = 0.0;
            params.p951 = 0.0;
            params.p952 = 0.0;
            params.p953 = 0.0;
            params.p954 = 0.0;
            params.p955 = 1.0;
            params.p956 = 0.0;
            params.p957 = 0.0;
            params.p958 = 0.0;
            params.p959 = 0.0;
            params.p960 = 0.0;
            params.p961 = 0.001;
            params.p962 = 0.0;
            params.p963 = 0.0;
            params.p964 = 0.0;
            params.p965 = 0.0;
            params.p966 = 0.0;
            params.p967 = 0.0004;
            params.p968 = 0.0;
            params.p969 = 0.0;
            params.p970 = 0.0;
            params.p971 = 0.0;
            params.p972 = 0.0;
            params.p973 = 170.0;
            params.p974 = 0.0;
            params.p975 = 0.0;
            params.p976 = 0.0;
            params.p977 = 0.0;
            params.p978 = 0.0;
            params.p979 = 0.01;
            params.p980 = 0.0;
            params.p981 = 0.0;
            params.p982 = 0.0;
            params.p983 = 0.0;
            params.p984 = 0.0;
            params.p985 = 1.3;
            params.p986 = 0.0;
            params.p987 = 0.0;
            params.p988 = 0.0;
            params.p989 = 0.0;
            params.p990 = 0.0;
            params.p991 = 0.0002;
            params.p992 = 0.0;
            params.p993 = 0.0;
            params.p994 = 0.0;
            params.p995 = 0.0;
            params.p996 = 0.0;
            params.p997 = params.p985;
            validate_finite_parameter("pdibl1r", params.p997).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p998 = params.p986;
            validate_finite_parameter("lpdibl1r", params.p998).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p999 = params.p987;
            validate_finite_parameter("npdibl1r", params.p999).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1000 = params.p988;
            validate_finite_parameter("ppdibl1r", params.p1000).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1001 = params.p989;
            validate_finite_parameter("wpdibl1r", params.p1001).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1002 = params.p990;
            validate_finite_parameter("p2pdibl1r", params.p1002).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1003 = params.p991;
            validate_finite_parameter("pdibl2r", params.p1003).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1004 = params.p992;
            validate_finite_parameter("lpdibl2r", params.p1004).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1005 = params.p993;
            validate_finite_parameter("npdibl2r", params.p1005).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1006 = params.p994;
            validate_finite_parameter("ppdibl2r", params.p1006).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1007 = params.p995;
            validate_finite_parameter("wpdibl2r", params.p1007).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1008 = params.p996;
            validate_finite_parameter("p2pdibl2r", params.p1008).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1009 = 1.06;
            params.p1010 = 0.0;
            params.p1011 = 0.0;
            params.p1012 = 0.0;
            params.p1013 = 0.0;
            params.p1014 = 0.0;
            params.p1015 = 1.0;
            params.p1016 = 0.0;
            params.p1017 = 0.0;
            params.p1018 = 0.0;
            params.p1019 = 0.0;
            params.p1020 = 0.0;
            params.p1021 = 0.0;
            params.p1022 = params.p1021;
            validate_finite_parameter("apclmr", params.p1022).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1023 = 1e-7;
            params.p1024 = params.p1023;
            validate_finite_parameter("bpclmr", params.p1024).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1025 = 0.013;
            params.p1026 = -2e-5;
            params.p1027 = 0.0;
            params.p1028 = 0.0;
            params.p1029 = 0.0;
            params.p1030 = 0.0;
            params.p1031 = 0.0;
            params.p1032 = params.p1025;
            validate_finite_parameter("pclmr", params.p1032).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1033 = params.p1027;
            validate_finite_parameter("lpclmr", params.p1033).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1034 = params.p1028;
            validate_finite_parameter("npclmr", params.p1034).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1035 = params.p1029;
            validate_finite_parameter("ppclmr", params.p1035).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1036 = params.p1030;
            validate_finite_parameter("wpclmr", params.p1036).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1037 = params.p1031;
            validate_finite_parameter("p2pclmr", params.p1037).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1038 = 0.0;
            params.p1039 = 0.0;
            params.p1040 = 0.0;
            params.p1041 = 0.0;
            params.p1042 = 0.0;
            params.p1043 = 0.0;
            params.p1044 = params.p1025;
            validate_finite_parameter("pclmcv", params.p1044).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1045 = params.p1027;
            validate_finite_parameter("lpclmcv", params.p1045).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1046 = params.p1028;
            validate_finite_parameter("npclmcv", params.p1046).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1047 = params.p1029;
            validate_finite_parameter("ppclmcv", params.p1047).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1048 = params.p1030;
            validate_finite_parameter("wpclmcv", params.p1048).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1049 = params.p1031;
            validate_finite_parameter("p2pclmcv", params.p1049).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1050 = 0.0;
            params.p1051 = 0.0;
            params.p1052 = 0.0;
            params.p1053 = 0.0;
            params.p1054 = 0.0;
            params.p1055 = 0.0;
            params.p1056 = 0.0;
            params.p1057 = 0.0;
            params.p1058 = 0.0;
            params.p1059 = 0.0;
            params.p1060 = 0.0;
            params.p1061 = 0.0;
            params.p1062 = 0.0;
            params.p1063 = 0.0;
            params.p1064 = 0.0;
            params.p1065 = 0.0;
            params.p1066 = 0.0;
            params.p1067 = 0.0;
            params.p1068 = 0.0;
            params.p1069 = 0.0;
            params.p1070 = 0.0;
            params.p1071 = 0.0;
            params.p1072 = 0.0;
            params.p1073 = 0.0;
            params.p1074 = 0.0;
            params.p1075 = 0.001;
            params.p1076 = 0.0;
            params.p1077 = 0.0;
            params.p1078 = 0.0;
            params.p1079 = params.p1078;
            validate_finite_parameter("rshd", params.p1079).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1080 = 1e-8;
            params.p1081 = 1e-8;
            params.p1082 = 1e-12;
            params.p1083 = 1.0;
            params.p1084 = 0.5;
            params.p1085 = 0.0;
            params.p1086 = 0.0;
            params.p1087 = 6e-9;
            params.p1088 = 3.9;
            params.p1089 = 3e-8;
            params.p1090 = 3e-8;
            params.p1091 = 0.0;
            params.p1092 = 0.0;
            params.p1093 = 0.0;
            params.p1094 = 1.0;
            params.p1095 = 0.0;
            params.p1096 = 0.0;
            params.p1097 = 0.0;
            params.p1098 = 0.0;
            params.p1099 = 1.0;
            params.p1100 = 0.0;
            params.p1101 = 0.0;
            params.p1102 = 0.0;
            params.p1103 = 1.0;
            params.p1104 = 0.0;
            params.p1105 = params.p1104;
            validate_finite_parameter("dlcigd", params.p1105).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1106 = 0.0;
            params.p1107 = params.p1106;
            validate_finite_parameter("vfbsdcv", params.p1107).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1108 = 1.2e-9;
            params.p1109 = params.p90;
            validate_parameter("toxg", params.p1109, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1110 = 0.001;
            params.p1111 = 0.001;
            params.p1112 = 0.0005;
            params.p1113 = 1.0;
            params.p1114 = 0.0;
            params.p1115 = 0.0;
            params.p1116 = 0.0;
            params.p1117 = 0.0;
            params.p1118 = 0.0;
            params.p1119 = 0.0111;
            params.p1120 = 0.0;
            params.p1121 = 0.0;
            params.p1122 = 0.0;
            params.p1123 = 0.0;
            params.p1124 = 0.0;
            params.p1125 = 0.0;
            params.p1126 = 0.0;
            params.p1127 = 0.0;
            params.p1128 = 0.0;
            params.p1129 = 0.0;
            params.p1130 = 0.0;
            params.p1131 = 0.000949;
            params.p1132 = 0.0;
            params.p1133 = 0.0;
            params.p1134 = 0.0;
            params.p1135 = 0.0;
            params.p1136 = 0.0;
            params.p1137 = 0.006;
            params.p1138 = 0.0;
            params.p1139 = 0.0;
            params.p1140 = 0.0;
            params.p1141 = 0.0;
            params.p1142 = 0.0;
            params.p1143 = 1.1;
            params.p1144 = 0.0;
            params.p1145 = 0.0;
            params.p1146 = 0.0;
            params.p1147 = 0.0;
            params.p1148 = 0.0;
            params.p1149 = 3.0;
            params.p1150 = 0.0;
            params.p1151 = 0.0;
            params.p1152 = 0.0;
            params.p1153 = 0.0;
            params.p1154 = 0.0;
            params.p1155 = 0.0136;
            params.p1156 = 0.0;
            params.p1157 = 0.0;
            params.p1158 = 0.0;
            params.p1159 = 0.0;
            params.p1160 = 0.0;
            params.p1161 = 0.0;
            params.p1162 = 0.0;
            params.p1163 = 0.0;
            params.p1164 = 0.0;
            params.p1165 = 0.0;
            params.p1166 = 0.0;
            params.p1167 = 0.00171;
            params.p1168 = 0.0;
            params.p1169 = 0.0;
            params.p1170 = 0.0;
            params.p1171 = 0.0;
            params.p1172 = 0.0;
            params.p1173 = 0.075;
            params.p1174 = 0.0;
            params.p1175 = 0.0;
            params.p1176 = 0.0;
            params.p1177 = 0.0;
            params.p1178 = 0.0;
            params.p1179 = 1.0;
            params.p1180 = 0.0;
            params.p1181 = 0.0;
            params.p1182 = 0.0;
            params.p1183 = 0.0;
            params.p1184 = 0.0;
            params.p1185 = 0.0136;
            params.p1186 = 0.0;
            params.p1187 = 0.0;
            params.p1188 = 0.0;
            params.p1189 = 0.0;
            params.p1190 = 0.0;
            params.p1191 = 0.0;
            params.p1192 = 0.0;
            params.p1193 = 0.0;
            params.p1194 = 0.0;
            params.p1195 = 0.0;
            params.p1196 = 0.0;
            params.p1197 = 0.00171;
            params.p1198 = 0.0;
            params.p1199 = 0.0;
            params.p1200 = 0.0;
            params.p1201 = 0.0;
            params.p1202 = 0.0;
            params.p1203 = 0.075;
            params.p1204 = 0.0;
            params.p1205 = 0.0;
            params.p1206 = 0.0;
            params.p1207 = 0.0;
            params.p1208 = 0.0;
            params.p1209 = 1.0;
            params.p1210 = 0.0;
            params.p1211 = 0.0;
            params.p1212 = 0.0;
            params.p1213 = 0.0;
            params.p1214 = 0.0;
            params.p1215 = 0.0136;
            params.p1216 = 0.0;
            params.p1217 = 0.0;
            params.p1218 = 0.0;
            params.p1219 = 0.0;
            params.p1220 = 0.0;
            params.p1221 = 0.0;
            params.p1222 = 0.0;
            params.p1223 = 0.0;
            params.p1224 = 0.0;
            params.p1225 = 0.0;
            params.p1226 = 0.0;
            params.p1227 = 0.00171;
            params.p1228 = 0.0;
            params.p1229 = 0.0;
            params.p1230 = 0.0;
            params.p1231 = 0.0;
            params.p1232 = 0.0;
            params.p1233 = 0.075;
            params.p1234 = 0.0;
            params.p1235 = 0.0;
            params.p1236 = 0.0;
            params.p1237 = 0.0;
            params.p1238 = 0.0;
            params.p1239 = params.p1215;
            validate_finite_parameter("aigd", params.p1239).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1240 = params.p1216;
            validate_finite_parameter("laigd", params.p1240).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1241 = params.p1217;
            validate_finite_parameter("naigd", params.p1241).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1242 = params.p1218;
            validate_finite_parameter("paigd", params.p1242).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1243 = params.p1219;
            validate_finite_parameter("waigd", params.p1243).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1244 = params.p1220;
            validate_finite_parameter("p2aigd", params.p1244).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1245 = params.p1221;
            validate_finite_parameter("aigd1", params.p1245).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1246 = params.p1222;
            validate_finite_parameter("laigd1", params.p1246).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1247 = params.p1223;
            validate_finite_parameter("naigd1", params.p1247).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1248 = params.p1224;
            validate_finite_parameter("paigd1", params.p1248).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1249 = params.p1225;
            validate_finite_parameter("waigd1", params.p1249).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1250 = params.p1226;
            validate_finite_parameter("p2aigd1", params.p1250).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1251 = params.p1227;
            validate_finite_parameter("bigd", params.p1251).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1252 = params.p1228;
            validate_finite_parameter("lbigd", params.p1252).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1253 = params.p1229;
            validate_finite_parameter("nbigd", params.p1253).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1254 = params.p1230;
            validate_finite_parameter("pbigd", params.p1254).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1255 = params.p1231;
            validate_finite_parameter("wbigd", params.p1255).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1256 = params.p1232;
            validate_finite_parameter("p2bigd", params.p1256).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1257 = params.p1233;
            validate_finite_parameter("cigd", params.p1257).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1258 = params.p1234;
            validate_finite_parameter("lcigd", params.p1258).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1259 = params.p1235;
            validate_finite_parameter("ncigd", params.p1259).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1260 = params.p1236;
            validate_finite_parameter("pcigd", params.p1260).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1261 = params.p1237;
            validate_finite_parameter("wcigd", params.p1261).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1262 = params.p1238;
            validate_finite_parameter("p2cigd", params.p1262).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1263 = 1.0;
            params.p1264 = 0.0;
            params.p1265 = 0.0;
            params.p1266 = 0.0;
            params.p1267 = 0.0;
            params.p1268 = 0.0;
            params.p1269 = 6.055e-12;
            params.p1270 = 0.0;
            params.p1271 = 0.0;
            params.p1272 = 0.0;
            params.p1273 = 0.0;
            params.p1274 = 0.0;
            params.p1275 = 300000000.0;
            params.p1276 = 0.0;
            params.p1277 = 0.0;
            params.p1278 = 0.0;
            params.p1279 = 0.0;
            params.p1280 = 0.0;
            params.p1281 = 0.5;
            params.p1282 = 0.0;
            params.p1283 = 0.0;
            params.p1284 = 0.0;
            params.p1285 = 0.0;
            params.p1286 = 0.0;
            params.p1287 = 0.2;
            params.p1288 = 0.0;
            params.p1289 = 0.0;
            params.p1290 = 0.0;
            params.p1291 = 0.0;
            params.p1292 = 0.0;
            params.p1293 = 1.0;
            params.p1294 = 0.0;
            params.p1295 = 0.0;
            params.p1296 = 0.0;
            params.p1297 = 0.0;
            params.p1298 = 0.0;
            params.p1299 = params.p1269;
            validate_finite_parameter("agisl", params.p1299).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1300 = params.p1270;
            validate_finite_parameter("lagisl", params.p1300).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1301 = params.p1271;
            validate_finite_parameter("nagisl", params.p1301).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1302 = params.p1272;
            validate_finite_parameter("pagisl", params.p1302).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1303 = params.p1273;
            validate_finite_parameter("wagisl", params.p1303).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1304 = params.p1274;
            validate_finite_parameter("p2agisl", params.p1304).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1305 = params.p1275;
            validate_finite_parameter("bgisl", params.p1305).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1306 = params.p1276;
            validate_finite_parameter("lbgisl", params.p1306).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1307 = params.p1277;
            validate_finite_parameter("nbgisl", params.p1307).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1308 = params.p1278;
            validate_finite_parameter("pbgisl", params.p1308).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1309 = params.p1279;
            validate_finite_parameter("wbgisl", params.p1309).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1310 = params.p1280;
            validate_finite_parameter("p2bgisl", params.p1310).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1311 = params.p1281;
            validate_finite_parameter("cgisl", params.p1311).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1312 = params.p1282;
            validate_finite_parameter("lcgisl", params.p1312).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1313 = params.p1283;
            validate_finite_parameter("ncgisl", params.p1313).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1314 = params.p1284;
            validate_finite_parameter("pcgisl", params.p1314).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1315 = params.p1285;
            validate_finite_parameter("wcgisl", params.p1315).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1316 = params.p1286;
            validate_finite_parameter("p2cgisl", params.p1316).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1317 = params.p1287;
            validate_finite_parameter("egisl", params.p1317).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1318 = params.p1288;
            validate_finite_parameter("legisl", params.p1318).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1319 = params.p1289;
            validate_finite_parameter("negisl", params.p1319).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1320 = params.p1290;
            validate_finite_parameter("pegisl", params.p1320).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1321 = params.p1291;
            validate_finite_parameter("wegisl", params.p1321).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1322 = params.p1292;
            validate_finite_parameter("p2egisl", params.p1322).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1323 = params.p1293;
            validate_finite_parameter("pgisl", params.p1323).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1324 = params.p1294;
            validate_finite_parameter("lpgisl", params.p1324).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1325 = params.p1295;
            validate_finite_parameter("npgisl", params.p1325).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1326 = params.p1296;
            validate_finite_parameter("ppgisl", params.p1326).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1327 = params.p1297;
            validate_finite_parameter("wpgisl", params.p1327).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1328 = params.p1298;
            validate_finite_parameter("p2pgisl", params.p1328).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1329 = 1e-27;
            params.p1330 = 0.0;
            params.p1331 = 0.0;
            params.p1332 = 0.0;
            params.p1333 = 0.0;
            params.p1334 = 0.0;
            params.p1335 = 6.3e-5;
            params.p1336 = 0.0;
            params.p1337 = 0.0;
            params.p1338 = 0.0;
            params.p1339 = 0.0;
            params.p1340 = 0.0;
            params.p1341 = 0.215;
            params.p1342 = 0.0;
            params.p1343 = 0.0;
            params.p1344 = 0.0;
            params.p1345 = 0.0;
            params.p1346 = 0.0;
            params.p1347 = 0.382;
            params.p1348 = 0.0;
            params.p1349 = 0.0;
            params.p1350 = 0.0;
            params.p1351 = 0.0;
            params.p1352 = 0.0;
            params.p1353 = params.p1329;
            validate_finite_parameter("atats", params.p1353).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1354 = 0.0;
            params.p1355 = 0.0;
            params.p1356 = 0.0;
            params.p1357 = 0.0;
            params.p1358 = 0.0;
            params.p1359 = params.p1335;
            validate_finite_parameter("btats", params.p1359).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1360 = 0.0;
            params.p1361 = 0.0;
            params.p1362 = 0.0;
            params.p1363 = 0.0;
            params.p1364 = 0.0;
            params.p1365 = params.p1341;
            validate_finite_parameter("ctats", params.p1365).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1366 = 0.0;
            params.p1367 = 0.0;
            params.p1368 = 0.0;
            params.p1369 = 0.0;
            params.p1370 = 0.0;
            params.p1371 = params.p1347;
            validate_finite_parameter("dtats", params.p1371).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1372 = 0.0;
            params.p1373 = 0.0;
            params.p1374 = 0.0;
            params.p1375 = 0.0;
            params.p1376 = 0.0;
            params.p1377 = 6.055e-12;
            params.p1378 = 0.0;
            params.p1379 = 0.0;
            params.p1380 = 0.0;
            params.p1381 = 0.0;
            params.p1382 = 0.0;
            params.p1383 = 300000000.0;
            params.p1384 = 0.0;
            params.p1385 = 0.0;
            params.p1386 = 0.0;
            params.p1387 = 0.0;
            params.p1388 = 0.0;
            params.p1389 = 0.5;
            params.p1390 = 0.0;
            params.p1391 = 0.0;
            params.p1392 = 0.0;
            params.p1393 = 0.0;
            params.p1394 = 0.0;
            params.p1395 = 0.2;
            params.p1396 = 0.0;
            params.p1397 = 0.0;
            params.p1398 = 0.0;
            params.p1399 = 0.0;
            params.p1400 = 0.0;
            params.p1401 = 1.0;
            params.p1402 = 0.0;
            params.p1403 = 0.0;
            params.p1404 = 0.0;
            params.p1405 = 0.0;
            params.p1406 = 0.0;
            params.p1407 = params.p1377;
            validate_finite_parameter("agislb", params.p1407).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1408 = params.p1378;
            validate_finite_parameter("lagislb", params.p1408).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1409 = params.p1379;
            validate_finite_parameter("nagislb", params.p1409).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1410 = params.p1380;
            validate_finite_parameter("pagislb", params.p1410).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1411 = params.p1381;
            validate_finite_parameter("wagislb", params.p1411).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1412 = params.p1382;
            validate_finite_parameter("p2agislb", params.p1412).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1413 = params.p1383;
            validate_finite_parameter("bgislb", params.p1413).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1414 = params.p1384;
            validate_finite_parameter("lbgislb", params.p1414).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1415 = params.p1385;
            validate_finite_parameter("nbgislb", params.p1415).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1416 = params.p1386;
            validate_finite_parameter("pbgislb", params.p1416).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1417 = params.p1387;
            validate_finite_parameter("wbgislb", params.p1417).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1418 = params.p1388;
            validate_finite_parameter("p2bgislb", params.p1418).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1419 = params.p1389;
            validate_finite_parameter("cgislb", params.p1419).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1420 = params.p1390;
            validate_finite_parameter("lcgislb", params.p1420).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1421 = params.p1391;
            validate_finite_parameter("ncgislb", params.p1421).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1422 = params.p1392;
            validate_finite_parameter("pcgislb", params.p1422).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1423 = params.p1393;
            validate_finite_parameter("wcgislb", params.p1423).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1424 = params.p1394;
            validate_finite_parameter("p2cgislb", params.p1424).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1425 = params.p1395;
            validate_finite_parameter("egislb", params.p1425).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1426 = params.p1396;
            validate_finite_parameter("legislb", params.p1426).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1427 = params.p1397;
            validate_finite_parameter("negislb", params.p1427).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1428 = params.p1398;
            validate_finite_parameter("pegislb", params.p1428).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1429 = params.p1399;
            validate_finite_parameter("wegislb", params.p1429).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1430 = params.p1400;
            validate_finite_parameter("p2egislb", params.p1430).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1431 = params.p1401;
            validate_finite_parameter("pgislb", params.p1431).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1432 = params.p1402;
            validate_finite_parameter("lpgislb", params.p1432).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1433 = params.p1403;
            validate_finite_parameter("npgislb", params.p1433).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1434 = params.p1404;
            validate_finite_parameter("ppgislb", params.p1434).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1435 = params.p1405;
            validate_finite_parameter("wpgislb", params.p1435).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1436 = params.p1406;
            validate_finite_parameter("p2pgislb", params.p1436).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1437 = 0.0;
            params.p1438 = 0.0;
            params.p1439 = 0.0;
            params.p1440 = 0.0;
            params.p1441 = 0.1;
            params.p1442 = 0.1;
            params.p1443 = 0.1;
            params.p1444 = 0.0;
            params.p1445 = 0.0;
            params.p1446 = 0.0;
            params.p1447 = 0.0;
            params.p1448 = 0.0;
            params.p1449 = 0.0;
            params.p1450 = 0.0;
            params.p1451 = 0.0;
            params.p1452 = 0.0;
            params.p1453 = 0.0;
            params.p1454 = 0.0;
            params.p1455 = 0.0;
            params.p1456 = 0.0;
            params.p1457 = 0.0;
            params.p1458 = 0.0;
            params.p1459 = 0.0;
            params.p1460 = 0.0;
            params.p1461 = 0.0;
            params.p1462 = 0.0;
            params.p1463 = 0.0;
            params.p1464 = 0.0;
            params.p1465 = 0.0;
            params.p1466 = 0.0;
            params.p1467 = 0.0;
            params.p1468 = 0.0;
            params.p1469 = 0.0;
            params.p1470 = 0.0;
            params.p1471 = 0.0;
            params.p1472 = 0.0;
            params.p1473 = 0.0;
            params.p1474 = 0.0;
            params.p1475 = 0.0;
            params.p1476 = 0.0;
            params.p1477 = 0.0;
            params.p1478 = 0.0;
            params.p1479 = 0.0;
            params.p1480 = 0.0;
            params.p1481 = 0.0;
            params.p1482 = 0.0;
            params.p1483 = 0.0;
            params.p1484 = 0.0;
            params.p1485 = 0.0;
            params.p1486 = 0.1;
            params.p1487 = 0.0;
            params.p1488 = 0.0;
            params.p1489 = 0.0;
            params.p1490 = 0.0;
            params.p1491 = 0.0;
            params.p1492 = 10000000.0;
            params.p1493 = 0.0;
            params.p1494 = 0.0;
            params.p1495 = 0.0;
            params.p1496 = 0.0;
            params.p1497 = 0.0;
            params.p1498 = 5e-10;
            params.p1499 = 0.0;
            params.p1500 = 0.0;
            params.p1501 = 0.0;
            params.p1502 = 0.0;
            params.p1503 = 0.0;
            params.p1504 = 0.5;
            params.p1505 = 0.0;
            params.p1506 = 0.0;
            params.p1507 = 0.0;
            params.p1508 = 0.0;
            params.p1509 = 0.0;
            params.p1510 = 0.1;
            params.p1511 = 0.0;
            params.p1512 = 0.0;
            params.p1513 = 0.0;
            params.p1514 = 0.0;
            params.p1515 = 0.0;
            params.p1516 = 0.0;
            params.p1517 = 0.0;
            params.p1518 = 0.0;
            params.p1519 = 0.0;
            params.p1520 = 0.0;
            params.p1521 = 0.0;
            params.p1522 = 0.0;
            params.p1523 = 0.0;
            params.p1524 = 0.0;
            params.p1525 = 0.0;
            params.p1526 = 0.0;
            params.p1527 = 0.0;
            params.p1528 = params.p89;
            validate_parameter("eotacc", params.p1528, Some((1e-10, "1e-10")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1529 = 0.0;
            params.p1530 = 2.5e-11;
            params.p1531 = 0.0;
            params.p1532 = 0.0;
            params.p1533 = 0.0;
            params.p1534 = 0.0;
            params.p1535 = 0.0;
            params.p1536 = params.p1530;
            validate_finite_parameter("cfd", params.p1536).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1537 = params.p1531;
            validate_finite_parameter("lcfd", params.p1537).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1538 = params.p1532;
            validate_finite_parameter("ncfd", params.p1538).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1539 = params.p1533;
            validate_finite_parameter("pcfd", params.p1539).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1540 = params.p1534;
            validate_finite_parameter("wcfd", params.p1540).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1541 = params.p1535;
            validate_finite_parameter("p2cfd", params.p1541).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1542 = 0.0;
            params.p1543 = params.p1542;
            validate_parameter("cgdo", params.p1543, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1544 = 0.0;
            params.p1545 = 0.0;
            params.p1546 = 0.0;
            params.p1547 = 0.0;
            params.p1548 = 0.0;
            params.p1549 = 0.0;
            params.p1550 = 0.0;
            params.p1551 = 0.0;
            params.p1552 = 0.0;
            params.p1553 = params.p1547;
            validate_finite_parameter("cgdl", params.p1553).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1554 = params.p1548;
            validate_finite_parameter("lcgdl", params.p1554).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1555 = params.p1549;
            validate_finite_parameter("ncgdl", params.p1555).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1556 = params.p1550;
            validate_finite_parameter("pcgdl", params.p1556).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1557 = params.p1551;
            validate_finite_parameter("wcgdl", params.p1557).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1558 = params.p1552;
            validate_finite_parameter("p2cgdl", params.p1558).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1559 = 0.0;
            params.p1560 = 0.0;
            params.p1561 = 0.0;
            params.p1562 = 0.0;
            params.p1563 = 0.0;
            params.p1564 = 0.0;
            params.p1565 = 0.6;
            params.p1566 = 0.0;
            params.p1567 = 0.0;
            params.p1568 = 0.0;
            params.p1569 = 0.0;
            params.p1570 = 0.0;
            params.p1571 = params.p1565;
            validate_finite_parameter("ckappad", params.p1571).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1572 = params.p1566;
            validate_finite_parameter("lckappad", params.p1572).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1573 = params.p1567;
            validate_finite_parameter("nckappad", params.p1573).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1574 = params.p1568;
            validate_finite_parameter("pckappad", params.p1574).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1575 = params.p1569;
            validate_finite_parameter("wckappad", params.p1575).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1576 = params.p1570;
            validate_finite_parameter("p2ckappad", params.p1576).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1577 = 0.6;
            params.p1578 = 0.0;
            params.p1579 = 0.0;
            params.p1580 = 0.0;
            params.p1581 = 0.0;
            params.p1582 = 0.0;
            params.p1583 = 0.0;
            params.p1584 = 0.0005;
            params.p1585 = params.p1584;
            validate_parameter("cjd", params.p1585, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1586 = 5e-10;
            params.p1587 = params.p1586;
            validate_parameter("cjswd", params.p1587, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1588 = 0.0;
            params.p1589 = params.p1588;
            validate_parameter("cjswgd", params.p1589, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1590 = 1.0;
            params.p1591 = params.p1590;
            validate_finite_parameter("pbd", params.p1591).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1592 = 1.0;
            params.p1593 = params.p1592;
            validate_finite_parameter("pbswd", params.p1593).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1594 = params.p1592;
            validate_finite_parameter("pbswgs", params.p1594).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1595 = params.p1594;
            validate_finite_parameter("pbswgd", params.p1595).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1596 = 0.5;
            params.p1597 = params.p1596;
            validate_parameter("mjd", params.p1597, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1598 = 0.33;
            params.p1599 = params.p1598;
            validate_parameter("mjswd", params.p1599, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1600 = params.p1598;
            validate_parameter("mjswgs", params.p1600, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1601 = params.p1600;
            validate_parameter("mjswgd", params.p1601, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1602 = 0.0;
            params.p1603 = params.p1602;
            validate_parameter("sjd", params.p1603, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1604 = 0.0;
            params.p1605 = params.p1604;
            validate_parameter("sjswd", params.p1605, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1606 = 0.0;
            params.p1607 = params.p1606;
            validate_parameter("sjswgd", params.p1607, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1608 = 0.125;
            params.p1609 = params.p1608;
            validate_finite_parameter("mjd2", params.p1609).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1610 = 0.083;
            params.p1611 = params.p1610;
            validate_finite_parameter("mjswd2", params.p1611).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1612 = params.p1610;
            validate_finite_parameter("mjswgs2", params.p1612).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1613 = params.p1612;
            validate_finite_parameter("mjswgd2", params.p1613).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1614 = 0.0001;
            params.p1615 = params.p1614;
            validate_parameter("jsd", params.p1615, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1616 = 0.0;
            params.p1617 = params.p1616;
            validate_parameter("jswd", params.p1617, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1618 = 0.0;
            params.p1619 = params.p1618;
            validate_parameter("jswgd", params.p1619, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1620 = 1.0;
            params.p1621 = params.p1620;
            validate_parameter("njd", params.p1621, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1622 = 0.1;
            params.p1623 = params.p1622;
            validate_finite_parameter("ijthdfwd", params.p1623).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1624 = 0.1;
            params.p1625 = params.p1624;
            validate_finite_parameter("ijthdrev", params.p1625).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1626 = 10.0;
            params.p1627 = params.p1626;
            validate_finite_parameter("bvd", params.p1627).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1628 = 1.0;
            params.p1629 = params.p1628;
            validate_finite_parameter("xjbvd", params.p1629).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1630 = 0.0;
            params.p1631 = params.p1630;
            validate_finite_parameter("jtsd", params.p1631).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1632 = 0.0;
            params.p1633 = params.p1632;
            validate_finite_parameter("jtsswd", params.p1633).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1634 = 0.0;
            params.p1635 = params.p1634;
            validate_finite_parameter("jtsswgd", params.p1635).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1636 = 0.0;
            params.p1637 = 20.0;
            params.p1638 = params.p1637;
            validate_finite_parameter("njtsd", params.p1638).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1639 = 20.0;
            params.p1640 = params.p1639;
            validate_finite_parameter("njtsswd", params.p1640).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1641 = 20.0;
            params.p1642 = params.p1641;
            validate_finite_parameter("njtsswgd", params.p1642).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1643 = 10.0;
            params.p1644 = params.p1643;
            validate_finite_parameter("vtsd", params.p1644).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1645 = 10.0;
            params.p1646 = params.p1645;
            validate_finite_parameter("vtsswd", params.p1646).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1647 = 10.0;
            params.p1648 = params.p1647;
            validate_finite_parameter("vtsswgd", params.p1648).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1649 = 0.0;
            params.p1650 = 1.0;
            params.p1651 = 0.0;
            params.p1652 = 0.0;
            params.p1653 = 0.0;
            params.p1654 = 0.0;
            params.p1655 = 0.0;
            params.p1656 = 0.0;
            params.p1657 = 0.0;
            params.p1658 = 0.0;
            params.p1659 = 0.0;
            params.p1660 = 0.0;
            params.p1661 = 0.0;
            params.p1662 = 0.0;
            params.p1663 = 0.0;
            params.p1664 = 0.0;
            params.p1665 = 0.0;
            params.p1666 = 0.0;
            params.p1667 = 0.0;
            params.p1668 = 12.0;
            params.p1669 = 0.0;
            params.p1670 = 0.0;
            params.p1671 = 0.0;
            params.p1672 = 0.0;
            params.p1673 = 0.0;
            params.p1674 = 1.0;
            params.p1675 = 0.0;
            params.p1676 = 0.0;
            params.p1677 = 0.0;
            params.p1678 = 0.0;
            params.p1679 = 0.0;
            params.p1680 = 1.0;
            params.p1681 = 41000000.0;
            params.p1682 = 6.25e39;
            params.p1683 = 3.125e24;
            params.p1684 = 87500000.0;
            params.p1685 = 1.0;
            params.p1686 = 1.0;
            params.p1687 = 0.0;
            params.p1688 = 2.0;
            params.p1689 = params.p1682;
            validate_finite_parameter("noia2", params.p1689).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1690 = 0.0;
            params.p1691 = 0.0;
            params.p1692 = 0.0;
            params.p1693 = 0.0;
            params.p1694 = 0.0;
            params.p1695 = 1.2;
            params.p1696 = 0.0;
            params.p1697 = 0.0;
            params.p1698 = 0.0;
            params.p1699 = 0.0;
            params.p1700 = 0.0;
            params.p1701 = 0.05;
            params.p1702 = 0.0;
            params.p1703 = 0.0;
            params.p1704 = 0.0;
            params.p1705 = 0.0;
            params.p1706 = 0.0;
            params.p1707 = 1.0;
            params.p1708 = 0.5774;
            params.p1709 = 0.0;
            params.p1710 = 0.3652;
            params.p1711 = 0.0;
            params.p1712 = 0.3953;
            params.p1713 = 0.0;
            params.p1714 = 0.0;
            params.p1715 = 0.0;
            params.p1716 = 0.1;
            params.p1717 = 27.0;
            params.p1718 = 0.000702;
            params.p1719 = 1108.0;
            params.p1720 = 0.0;
            params.p1721 = 0.0;
            params.p1722 = 0.0;
            params.p1723 = 0.0;
            params.p1724 = 0.0;
            params.p1725 = 0.0;
            params.p1726 = 0.0;
            params.p1727 = 3.0;
            params.p1728 = params.p1727;
            validate_finite_parameter("xtid", params.p1728).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1729 = 0.02;
            params.p1730 = params.p1729;
            validate_finite_parameter("xtsd", params.p1730).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1731 = 0.02;
            params.p1732 = params.p1731;
            validate_finite_parameter("xtsswd", params.p1732).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1733 = 0.02;
            params.p1734 = params.p1733;
            validate_finite_parameter("xtsswgd", params.p1734).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1735 = 0.0;
            params.p1736 = params.p1735;
            validate_finite_parameter("tnjtsd", params.p1736).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1737 = 0.0;
            params.p1738 = params.p1737;
            validate_finite_parameter("tnjtsswd", params.p1738).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1739 = 0.0;
            params.p1740 = params.p1739;
            validate_finite_parameter("tnjtsswgd", params.p1740).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1741 = 0.0;
            params.p1742 = 0.0;
            params.p1743 = 0.0;
            params.p1744 = 0.0;
            params.p1745 = 0.0;
            params.p1746 = 0.0;
            params.p1747 = 0.01;
            params.p1748 = 0.1;
            params.p1749 = 40.0;
            params.p1750 = 0.0;
            params.p1751 = 0.0;
            params.p1752 = 0.0;
            params.p1753 = 0.0;
            params.p1754 = 0.0;
            params.p1755 = 0.0;
            params.p1756 = -0.5;
            params.p1757 = 0.0;
            params.p1758 = 0.0;
            params.p1759 = 0.0;
            params.p1760 = 0.0;
            params.p1761 = 0.0;
            params.p1762 = 0.0;
            params.p1763 = 0.0;
            params.p1764 = 0.0;
            params.p1765 = 0.0;
            params.p1766 = 0.0;
            params.p1767 = 0.0;
            params.p1768 = -0.003;
            params.p1769 = 0.0;
            params.p1770 = 0.0;
            params.p1771 = 0.0;
            params.p1772 = 0.0;
            params.p1773 = 0.0;
            params.p1774 = -1e-6;
            params.p1775 = 0.0;
            params.p1776 = 0.0;
            params.p1777 = 0.0;
            params.p1778 = 0.0;
            params.p1779 = 0.0;
            params.p1780 = 2.5;
            params.p1781 = 0.0;
            params.p1782 = 0.0;
            params.p1783 = 0.0;
            params.p1784 = 0.0;
            params.p1785 = 0.0;
            params.p1786 = 50.0;
            params.p1787 = 0.0;
            params.p1788 = 1.0;
            params.p1789 = 0.001;
            params.p1790 = 0.0;
            params.p1791 = 0.01;
            params.p1792 = 1e-5;
            params.p1793 = 0.0;
            params.p1794 = 1.0;
            params.p1795 = 1.0;
            params.p1796 = 1.0;
            params.p1797 = 1.0;
            params.p1798 = 1.0;
            params.p1799 = 1.0;
            params.p1800 = 1.0;
            params.p1801 = 1.0;
            params.p1802 = 0.0;
            params.p1803 = 0.0;
            params.p1804 = 0.0;
            params.p1805 = 0.5556;
            params.p1806 = 3.0;
            params.p1807 = 2.0;
            params.p1808 = 0.0;
            params.p1809 = 0.0;
            params.p1810 = 0.0;
            params.p1811 = 0.0;
            params.p1812 = 0.0;
            params.p1813 = 3.0;
            params.p1814 = 2.6;
            params.p1815 = 0.0;
            params.p1816 = 0.0;
            params.p1817 = 0.0;
            params.p1818 = 0.0;
            params.p1819 = 0.0;
            params.p1820 = 3.0;
            params.p1821 = 2.6;
            params.p1822 = 0.0;
            params.p1823 = 0.0;
            params.p1824 = 0.0;
            params.p1825 = 0.0;
            params.p1826 = 0.0;
            params.p1827 = 9.5e-9;
            params.p1828 = 0.1;
            params.p1829 = 14.0;
            params.p1830 = 0.0;
            params.p1831 = 0.0;
            params.p1832 = 0.0;
            params.p1833 = 0.0;
            params.p1834 = 0.0;
            params.p1835 = 24.0;
            params.p1836 = 0.0;
            params.p1837 = 0.0;
            params.p1838 = 0.0;
            params.p1839 = 0.0;
            params.p1840 = 0.0;
            params.p1841 = 24.0;
            params.p1842 = 0.0;
            params.p1843 = 0.0;
            params.p1844 = 0.0;
            params.p1845 = 0.0;
            params.p1846 = 0.0;
            params.p1847 = 2.0;
            params.p1848 = 0.0;
            params.p1849 = 0.0;
            params.p1850 = params.p1827;
            validate_parameter("wssp0", params.p1850, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1851 = params.p1828;
            validate_parameter("wsspr", params.p1851, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1852 = 8e-9;
            params.p1853 = 0.139;
            params.p1854 = 0.0;
            params.p1855 = 0.0;
            params.p1856 = 0.0;
            params.p1857 = 0.0;
            params.p1858 = 0.0;
            params.p1859 = 2.0;
            params.p1860 = 0.0;
            params.p1861 = 0.0;
            params.p1862 = 0.0;
            params.p1863 = 0.0;
            params.p1864 = 0.0;
            params.p1865 = 1.0;
            params.p1866 = 1.0;
            params.p1867 = 1.0;
            params.p1868 = 1.0;
            params.p1869 = 11.2;
            params.p1870 = 0.0;
            params.p1871 = 0.0;
            params.p1872 = 0.0;
            params.p1873 = 0.0;
            params.p1874 = 0.0;
            params.p1875 = 8.02;
            params.p1876 = 0.0;
            params.p1877 = 0.0;
            params.p1878 = 0.0;
            params.p1879 = 0.0;
            params.p1880 = 0.0;
            params.p1881 = 6.18;
            params.p1882 = 0.0;
            params.p1883 = 0.0;
            params.p1884 = 0.0;
            params.p1885 = 0.0;
            params.p1886 = 0.0;
            params.p1887 = 1.0;
            params.p1888 = 1.0;
            params.p1889 = 1.0;
            params.p1890 = 1.0;
            params.p1891 = 1.0;
            params.p1892 = 1.0;
            params.p1893 = 1.8;
            params.p1894 = 1.0;
            params.p1895 = 0.67;
            params.p1896 = 0.23;
            params.p1897 = 1.1;
            params.p1898 = 2.4;
            params.p1899 = 2.0;
            params.p1900 = 2.0;
            params.p1901 = 6.0;
            params.p1902 = 2.4;
            params.p1903 = 5e16;
            params.p1904 = 100000.0;
            params.p1905 = 0.0;
            params.p1906 = 0.0;
            params.p1907 = 60.0;
            params.p1908 = 1.0;
            params.p1909 = params.p1903;
            validate_parameter("nvsrs", params.p1909, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1910 = 0.0;
            params.p1911 = 0.0;
            params.p1912 = 0.0;
            params.p1913 = 0.0;
            params.p1914 = 0.001;
            params.p1915 = 0.001;
            params.p1916 = 8.0;
            params.p1917 = 0.0;
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
    pub nodes: [usize; 17],
    pub branches: [usize; 18],
    pub(crate) params: Box<Parameters>,
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
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v3: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v8: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: f64,
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
    pub(crate) scalar_v46: bool,
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
    pub(crate) scalar_v57: bool,
    pub(crate) scalar_v58: f64,
    pub(crate) scalar_v59: f64,
    pub(crate) scalar_v61: bool,
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
    pub(crate) scalar_v107: bool,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: f64,
    pub(crate) scalar_v112: bool,
    pub(crate) scalar_v113: bool,
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
    pub(crate) scalar_v128: f64,
    pub(crate) scalar_v129: bool,
    pub(crate) scalar_v130: f64,
    pub(crate) scalar_v131: bool,
    pub(crate) scalar_v132: bool,
    pub(crate) scalar_v133: f64,
    pub(crate) scalar_v134: f64,
    pub(crate) scalar_v135: f64,
    pub(crate) scalar_v136: f64,
    pub(crate) scalar_v137: f64,
    pub(crate) scalar_v138: f64,
    pub(crate) scalar_v139: f64,
    pub(crate) scalar_v140: f64,
    pub(crate) scalar_v141: f64,
    pub(crate) scalar_v142: f64,
    pub(crate) scalar_v143: f64,
    pub(crate) scalar_v144: f64,
    pub(crate) scalar_v145: f64,
    pub(crate) scalar_v146: f64,
    pub(crate) scalar_v147: f64,
    pub(crate) scalar_v148: f64,
    pub(crate) scalar_v149: bool,
    pub(crate) scalar_v150: bool,
    pub(crate) scalar_v151: bool,
    pub(crate) scalar_v153: bool,
    pub(crate) scalar_v154: bool,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: bool,
    pub(crate) scalar_v157: bool,
    pub(crate) scalar_v158: bool,
    pub(crate) scalar_v159: f64,
    pub(crate) scalar_v160: bool,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: f64,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: f64,
    pub(crate) scalar_v166: bool,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v170: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: bool,
    pub(crate) scalar_v174: f64,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v177: bool,
    pub(crate) scalar_v178: bool,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: bool,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: f64,
    pub(crate) scalar_v185: bool,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: f64,
    pub(crate) scalar_v188: f64,
    pub(crate) scalar_v189: bool,
    pub(crate) scalar_v194: bool,
    pub(crate) scalar_v195: bool,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: bool,
    pub(crate) scalar_v199: bool,
    pub(crate) scalar_v200: f64,
    pub(crate) scalar_v201: bool,
    pub(crate) scalar_v202: bool,
    pub(crate) scalar_v203: bool,
    pub(crate) scalar_v204: bool,
    pub(crate) scalar_v209: bool,
    pub(crate) scalar_v210: f64,
    pub(crate) scalar_v212: bool,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v228: bool,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: bool,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v237: f64,
    pub(crate) scalar_v238: f64,
    pub(crate) scalar_v239: f64,
    pub(crate) scalar_v240: f64,
    pub(crate) scalar_v241: f64,
    pub(crate) scalar_v242: f64,
    pub(crate) scalar_v243: f64,
    pub(crate) scalar_v244: f64,
    pub(crate) scalar_v245: f64,
    pub(crate) scalar_v246: f64,
    pub(crate) scalar_v247: f64,
    pub(crate) scalar_v248: f64,
    pub(crate) scratch: Option<Box<GenericScratch<1763, 17, 18>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<1763, 17, 18>>>,
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
            scalar_v5: self.scalar_v5,
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
            scalar_v51: self.scalar_v51,
            scalar_v52: self.scalar_v52,
            scalar_v53: self.scalar_v53,
            scalar_v54: self.scalar_v54,
            scalar_v55: self.scalar_v55,
            scalar_v56: self.scalar_v56,
            scalar_v57: self.scalar_v57,
            scalar_v58: self.scalar_v58,
            scalar_v59: self.scalar_v59,
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
            scalar_v194: self.scalar_v194,
            scalar_v195: self.scalar_v195,
            scalar_v196: self.scalar_v196,
            scalar_v197: self.scalar_v197,
            scalar_v199: self.scalar_v199,
            scalar_v200: self.scalar_v200,
            scalar_v201: self.scalar_v201,
            scalar_v202: self.scalar_v202,
            scalar_v203: self.scalar_v203,
            scalar_v204: self.scalar_v204,
            scalar_v209: self.scalar_v209,
            scalar_v210: self.scalar_v210,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
            scalar_v228: self.scalar_v228,
            scalar_v229: self.scalar_v229,
            scalar_v230: self.scalar_v230,
            scalar_v231: self.scalar_v231,
            scalar_v232: self.scalar_v232,
            scalar_v235: self.scalar_v235,
            scalar_v237: self.scalar_v237,
            scalar_v238: self.scalar_v238,
            scalar_v239: self.scalar_v239,
            scalar_v240: self.scalar_v240,
            scalar_v241: self.scalar_v241,
            scalar_v242: self.scalar_v242,
            scalar_v243: self.scalar_v243,
            scalar_v244: self.scalar_v244,
            scalar_v245: self.scalar_v245,
            scalar_v246: self.scalar_v246,
            scalar_v247: self.scalar_v247,
            scalar_v248: self.scalar_v248,
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
            scalar_v5: 0.0,
            scalar_v7: 0.0,
            scalar_v8: 0.0,
            scalar_v9: 0.0,
            scalar_v10: 0.0,
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
            scalar_v46: false,
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
            scalar_v57: false,
            scalar_v58: 0.0,
            scalar_v59: 0.0,
            scalar_v61: false,
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
            scalar_v107: false,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: 0.0,
            scalar_v112: false,
            scalar_v113: false,
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
            scalar_v128: 0.0,
            scalar_v129: false,
            scalar_v130: 0.0,
            scalar_v131: false,
            scalar_v132: false,
            scalar_v133: 0.0,
            scalar_v134: 0.0,
            scalar_v135: 0.0,
            scalar_v136: 0.0,
            scalar_v137: 0.0,
            scalar_v138: 0.0,
            scalar_v139: 0.0,
            scalar_v140: 0.0,
            scalar_v141: 0.0,
            scalar_v142: 0.0,
            scalar_v143: 0.0,
            scalar_v144: 0.0,
            scalar_v145: 0.0,
            scalar_v146: 0.0,
            scalar_v147: 0.0,
            scalar_v148: 0.0,
            scalar_v149: false,
            scalar_v150: false,
            scalar_v151: false,
            scalar_v153: false,
            scalar_v154: false,
            scalar_v155: 0.0,
            scalar_v156: false,
            scalar_v157: false,
            scalar_v158: false,
            scalar_v159: 0.0,
            scalar_v160: false,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: 0.0,
            scalar_v164: 0.0,
            scalar_v165: 0.0,
            scalar_v166: false,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v170: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: false,
            scalar_v174: 0.0,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v177: false,
            scalar_v178: false,
            scalar_v179: 0.0,
            scalar_v180: false,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: 0.0,
            scalar_v185: false,
            scalar_v186: 0.0,
            scalar_v187: 0.0,
            scalar_v188: 0.0,
            scalar_v189: false,
            scalar_v194: false,
            scalar_v195: false,
            scalar_v196: 0.0,
            scalar_v197: false,
            scalar_v199: false,
            scalar_v200: 0.0,
            scalar_v201: false,
            scalar_v202: false,
            scalar_v203: false,
            scalar_v204: false,
            scalar_v209: false,
            scalar_v210: 0.0,
            scalar_v212: false,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v228: false,
            scalar_v229: 0.0,
            scalar_v230: false,
            scalar_v231: 0.0,
            scalar_v232: 0.0,
            scalar_v235: 0.0,
            scalar_v237: 0.0,
            scalar_v238: 0.0,
            scalar_v239: 0.0,
            scalar_v240: 0.0,
            scalar_v241: 0.0,
            scalar_v242: 0.0,
            scalar_v243: 0.0,
            scalar_v244: 0.0,
            scalar_v245: 0.0,
            scalar_v246: 0.0,
            scalar_v247: 0.0,
            scalar_v248: 0.0,
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
            scalar_v5,
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
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
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
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v209,
            scalar_v210,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v235,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
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
            scalar_v5,
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
            scalar_v51,
            scalar_v52,
            scalar_v53,
            scalar_v54,
            scalar_v55,
            scalar_v56,
            scalar_v57,
            scalar_v58,
            scalar_v59,
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
            scalar_v194,
            scalar_v195,
            scalar_v196,
            scalar_v197,
            scalar_v199,
            scalar_v200,
            scalar_v201,
            scalar_v202,
            scalar_v203,
            scalar_v204,
            scalar_v209,
            scalar_v210,
            scalar_v212,
            scalar_v213,
            scalar_v214,
            scalar_v228,
            scalar_v229,
            scalar_v230,
            scalar_v231,
            scalar_v232,
            scalar_v235,
            scalar_v237,
            scalar_v238,
            scalar_v239,
            scalar_v240,
            scalar_v241,
            scalar_v242,
            scalar_v243,
            scalar_v244,
            scalar_v245,
            scalar_v246,
            scalar_v247,
            scalar_v248,
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
            "l" => { validate_parameter("l", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "lover" => { validate_parameter("lover", value, Some((1e-20, "1e-20")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "dia" => { validate_parameter("dia", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "tfin" => { validate_parameter("tfin", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "fpitch" => { validate_finite_parameter("fpitch", value)?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "nfin" => { validate_parameter("nfin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("ngcon", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "aseo" => { validate_parameter("aseo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "adeo" => { validate_parameter("adeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "pseo" => { validate_parameter("pseo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "pdeo" => { validate_parameter("pdeo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "asej" => { validate_parameter("asej", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "adej" => { validate_parameter("adej", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "psej" => { validate_parameter("psej", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "pdej" => { validate_parameter("pdej", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "cgsp" => { validate_parameter("cgsp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "cgdp" => { validate_parameter("cgdp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "cdsp" => { validate_parameter("cdsp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "nrs" => { validate_parameter("nrs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "nrd" => { validate_parameter("nrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "lrsd" => { validate_parameter("lrsd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "nfinnom" => { validate_parameter("nfinnom", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "dtemp" => { validate_finite_parameter("dtemp", value)?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "delvtrand" => { validate_finite_parameter("delvtrand", value)?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "u0mult" => { validate_parameter("u0mult", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "ids0mult" => { validate_parameter("ids0mult", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "igc0mult" => { validate_parameter("igc0mult", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "igb0mult" => { validate_parameter("igb0mult", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "covs" => { validate_finite_parameter("covs", value)?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "lcovs" => { validate_finite_parameter("lcovs", value)?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ncovs" => { validate_finite_parameter("ncovs", value)?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "pcovs" => { validate_finite_parameter("pcovs", value)?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "wcovs" => { validate_finite_parameter("wcovs", value)?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "p2covs" => { validate_finite_parameter("p2covs", value)?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "covd" => { validate_finite_parameter("covd", value)?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "lcovd" => { validate_finite_parameter("lcovd", value)?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "ncovd" => { validate_finite_parameter("ncovd", value)?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "pcovd" => { validate_finite_parameter("pcovd", value)?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "wcovd" => { validate_finite_parameter("wcovd", value)?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "p2covd" => { validate_finite_parameter("p2covd", value)?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "tgaa" => { validate_parameter("tgaa", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "tsus" => { validate_parameter("tsus", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "hpff" => { validate_parameter("hpff", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "wgaa" => { validate_parameter("wgaa", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "dws1" => { validate_parameter("dws1", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "dach1" => { validate_parameter("dach1", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "dws2" => { validate_parameter("dws2", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "dach2" => { validate_parameter("dach2", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "dws3" => { validate_parameter("dws3", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "dach3" => { validate_parameter("dach3", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "dws4" => { validate_parameter("dws4", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "dach4" => { validate_parameter("dach4", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "dws5" => { validate_parameter("dws5", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "dach5" => { validate_parameter("dach5", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "dws6" => { validate_parameter("dws6", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "dach6" => { validate_parameter("dach6", value, None, true, Some((0.0, "0.0")), false, &[])?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "ngaa" => { validate_parameter("ngaa", value, Some((0.0, "0.0")), false, Some((6.0, "6.0")), false, &[])?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "subbandmod" => { validate_parameter("subbandmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "mobscmod" => { validate_parameter("mobscmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("nf", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("type", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "bulkmod" => { validate_parameter("bulkmod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "geomod" => { validate_parameter("geomod", value, Some((0.0, "0.0")), false, Some((5.0, "5.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "cgeo1sw" => { validate_parameter("cgeo1sw", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "rdsmod" => { validate_parameter("rdsmod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "hvmod" => { validate_parameter("hvmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "asymmod" => { validate_parameter("asymmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "cvmod" => { validate_parameter("cvmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "igcmod" => { validate_parameter("igcmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "igbmod" => { validate_parameter("igbmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "gidlmod" => { validate_parameter("gidlmod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "iimod" => { validate_parameter("iimod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "tnoimod" => { validate_parameter("tnoimod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "nqsmod" => { validate_parameter("nqsmod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("shmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "tempmod" => { validate_parameter("tempmod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "rgatemod" => { validate_parameter("rgatemod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "rgeomod" => { validate_parameter("rgeomod", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "cgeomod" => { validate_parameter("cgeomod", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "fnmod" => { validate_parameter("fnmod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "cryomod" => { validate_parameter("cryomod", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "sh_warn" => { validate_parameter("sh_warn", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "igclamp" => { validate_parameter("igclamp", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "ll" => { validate_finite_parameter("ll", value)?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "lln" => { validate_finite_parameter("lln", value)?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "dlc" => { validate_finite_parameter("dlc", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "dlcacc" => { validate_finite_parameter("dlcacc", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "dwcacc" => { validate_finite_parameter("dwcacc", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "llc" => { validate_finite_parameter("llc", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "eot" => { validate_parameter("eot", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "toxp" => { validate_parameter("toxp", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "eotbox" => { validate_parameter("eotbox", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "hfin" => { validate_parameter("hfin", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "deltaw" => { validate_finite_parameter("deltaw", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "deltawcv" => { validate_finite_parameter("deltawcv", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "nbodyn1" => { validate_finite_parameter("nbodyn1", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "nbodyn2" => { validate_finite_parameter("nbodyn2", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "nsd" => { validate_parameter("nsd", value, Some((2e25, "2e25")), false, Some((1e27, "1e27")), false, &[])?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "phigl" => { validate_finite_parameter("phigl", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "phiglt" => { validate_finite_parameter("phiglt", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "phign1" => { validate_finite_parameter("phign1", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "phign2" => { validate_finite_parameter("phign2", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "epsrox" => { validate_parameter("epsrox", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "epsrsub" => { validate_parameter("epsrsub", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "easub" => { validate_parameter("easub", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "ni0sub" => { validate_parameter("ni0sub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "bg0sub" => { validate_parameter("bg0sub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "nc0sub" => { validate_parameter("nc0sub", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "imin" => { validate_parameter("imin", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "xl" => { validate_finite_parameter("xl", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "lxl" => { validate_finite_parameter("lxl", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "nxl" => { validate_finite_parameter("nxl", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "pxl" => { validate_finite_parameter("pxl", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "lint" => { validate_finite_parameter("lint", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "llint" => { validate_finite_parameter("llint", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "nlint" => { validate_finite_parameter("nlint", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "plint" => { validate_finite_parameter("plint", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "dlbin" => { validate_finite_parameter("dlbin", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "ldlbin" => { validate_finite_parameter("ldlbin", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "ndlbin" => { validate_finite_parameter("ndlbin", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "pdlbin" => { validate_finite_parameter("pdlbin", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "xw" => { validate_finite_parameter("xw", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "lxw" => { validate_finite_parameter("lxw", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "nxw" => { validate_finite_parameter("nxw", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "pxw" => { validate_finite_parameter("pxw", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "wxw" => { validate_finite_parameter("wxw", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "p2xw" => { validate_finite_parameter("p2xw", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "dwbin" => { validate_finite_parameter("dwbin", value)?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "ldwbin" => { validate_finite_parameter("ldwbin", value)?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "ndwbin" => { validate_finite_parameter("ndwbin", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "pdwbin" => { validate_finite_parameter("pdwbin", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "wdwbin" => { validate_finite_parameter("wdwbin", value)?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "p2dwbin" => { validate_finite_parameter("p2dwbin", value)?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "nbody" => { validate_finite_parameter("nbody", value)?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "lnbody" => { validate_finite_parameter("lnbody", value)?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "nnbody" => { validate_finite_parameter("nnbody", value)?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "pnbody" => { validate_finite_parameter("pnbody", value)?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "phig" => { validate_finite_parameter("phig", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "lphig" => { validate_finite_parameter("lphig", value)?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "nphig" => { validate_finite_parameter("nphig", value)?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "pphig" => { validate_finite_parameter("pphig", value)?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "wphig" => { validate_finite_parameter("wphig", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "p2phig" => { validate_finite_parameter("p2phig", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "vfbdriftd" => { validate_finite_parameter("vfbdriftd", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "vfbdrifts" => { validate_finite_parameter("vfbdrifts", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "ngate" => { validate_finite_parameter("ngate", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "lngate" => { validate_finite_parameter("lngate", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "nngate" => { validate_finite_parameter("nngate", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "pngate" => { validate_finite_parameter("pngate", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "wngate" => { validate_finite_parameter("wngate", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "p2ngate" => { validate_finite_parameter("p2ngate", value)?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "cdscn1" => { validate_finite_parameter("cdscn1", value)?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "cdscn2" => { validate_finite_parameter("cdscn2", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "cdscdn1" => { validate_finite_parameter("cdscdn1", value)?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "cdscdn2" => { validate_finite_parameter("cdscdn2", value)?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "cdscdrn1" => { validate_finite_parameter("cdscdrn1", value)?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "cdscdrn2" => { validate_finite_parameter("cdscdrn2", value)?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "eta0n1" => { validate_finite_parameter("eta0n1", value)?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "eta0n2" => { validate_parameter("eta0n2", value, Some((1e-5, "1e-5")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "eta0lt" => { validate_finite_parameter("eta0lt", value)?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "eta0n1cv" => { validate_finite_parameter("eta0n1cv", value)?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "eta0n2cv" => { validate_parameter("eta0n2cv", value, Some((1e-5, "1e-5")), false, None, true, &[])?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "eta0ltcv" => { validate_finite_parameter("eta0ltcv", value)?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "teta0" => { validate_finite_parameter("teta0", value)?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "teta0cv" => { validate_finite_parameter("teta0cv", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "teta0r" => { validate_finite_parameter("teta0r", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "advtp0" => { validate_finite_parameter("advtp0", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "bdvtp0" => { validate_finite_parameter("bdvtp0", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "advtp1" => { validate_finite_parameter("advtp1", value)?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "bdvtp1" => { validate_finite_parameter("bdvtp1", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "dvtp2" => { validate_finite_parameter("dvtp2", value)?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "thetasce" => { validate_finite_parameter("thetasce", value)?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "thetadibl" => { validate_finite_parameter("thetadibl", value)?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "thetasw" => { validate_finite_parameter("thetasw", value)?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "nvtm" => { validate_parameter("nvtm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "dvtp0" => { validate_finite_parameter("dvtp0", value)?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "ldvtp0" => { validate_finite_parameter("ldvtp0", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "ndvtp0" => { validate_finite_parameter("ndvtp0", value)?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "pdvtp0" => { validate_finite_parameter("pdvtp0", value)?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "wdvtp0" => { validate_finite_parameter("wdvtp0", value)?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "p2dvtp0" => { validate_finite_parameter("p2dvtp0", value)?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "dvtp1" => { validate_finite_parameter("dvtp1", value)?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "ldvtp1" => { validate_finite_parameter("ldvtp1", value)?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "ndvtp1" => { validate_finite_parameter("ndvtp1", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "pdvtp1" => { validate_finite_parameter("pdvtp1", value)?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "wdvtp1" => { validate_finite_parameter("wdvtp1", value)?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "p2dvtp1" => { validate_finite_parameter("p2dvtp1", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "cit" => { validate_finite_parameter("cit", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "lcit" => { validate_finite_parameter("lcit", value)?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "ncit" => { validate_finite_parameter("ncit", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "pcit" => { validate_finite_parameter("pcit", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "wcit" => { validate_finite_parameter("wcit", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "p2cit" => { validate_finite_parameter("p2cit", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "citr" => { validate_finite_parameter("citr", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "lcitr" => { validate_finite_parameter("lcitr", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "ncitr" => { validate_finite_parameter("ncitr", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "pcitr" => { validate_finite_parameter("pcitr", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "wcitr" => { validate_finite_parameter("wcitr", value)?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "p2citr" => { validate_finite_parameter("p2citr", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "cdsc" => { validate_finite_parameter("cdsc", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "lcdsc" => { validate_finite_parameter("lcdsc", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "ncdsc" => { validate_finite_parameter("ncdsc", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "pcdsc" => { validate_finite_parameter("pcdsc", value)?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "wcdsc" => { validate_finite_parameter("wcdsc", value)?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "p2cdsc" => { validate_finite_parameter("p2cdsc", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_finite_parameter("cdscd", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "lcdscd" => { validate_finite_parameter("lcdscd", value)?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "ncdscd" => { validate_finite_parameter("ncdscd", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "pcdscd" => { validate_finite_parameter("pcdscd", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "wcdscd" => { validate_finite_parameter("wcdscd", value)?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "p2cdscd" => { validate_finite_parameter("p2cdscd", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "cdscdr" => { validate_finite_parameter("cdscdr", value)?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "lcdscdr" => { validate_finite_parameter("lcdscdr", value)?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "ncdscdr" => { validate_finite_parameter("ncdscdr", value)?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "pcdscdr" => { validate_finite_parameter("pcdscdr", value)?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "wcdscdr" => { validate_finite_parameter("wcdscdr", value)?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "p2cdscdr" => { validate_finite_parameter("p2cdscdr", value)?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "dvt0" => { validate_finite_parameter("dvt0", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "ldvt0" => { validate_finite_parameter("ldvt0", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "ndvt0" => { validate_finite_parameter("ndvt0", value)?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "pdvt0" => { validate_finite_parameter("pdvt0", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "wdvt0" => { validate_finite_parameter("wdvt0", value)?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "p2dvt0" => { validate_finite_parameter("p2dvt0", value)?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "dvt1" => { validate_finite_parameter("dvt1", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "ldvt1" => { validate_finite_parameter("ldvt1", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "ndvt1" => { validate_finite_parameter("ndvt1", value)?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "pdvt1" => { validate_finite_parameter("pdvt1", value)?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "wdvt1" => { validate_finite_parameter("wdvt1", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "p2dvt1" => { validate_finite_parameter("p2dvt1", value)?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "dvt1ss" => { validate_finite_parameter("dvt1ss", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "ldvt1ss" => { validate_finite_parameter("ldvt1ss", value)?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "ndvt1ss" => { validate_finite_parameter("ndvt1ss", value)?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "pdvt1ss" => { validate_finite_parameter("pdvt1ss", value)?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "wdvt1ss" => { validate_finite_parameter("wdvt1ss", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "p2dvt1ss" => { validate_finite_parameter("p2dvt1ss", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "phin" => { validate_finite_parameter("phin", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "lphin" => { validate_finite_parameter("lphin", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "nphin" => { validate_finite_parameter("nphin", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "pphin" => { validate_finite_parameter("pphin", value)?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "wphin" => { validate_finite_parameter("wphin", value)?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "p2phin" => { validate_finite_parameter("p2phin", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_finite_parameter("eta0", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "leta0" => { validate_finite_parameter("leta0", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "neta0" => { validate_finite_parameter("neta0", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "peta0" => { validate_finite_parameter("peta0", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "weta0" => { validate_finite_parameter("weta0", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "p2eta0" => { validate_finite_parameter("p2eta0", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "eta1" => { validate_finite_parameter("eta1", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "leta1" => { validate_finite_parameter("leta1", value)?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "neta1" => { validate_finite_parameter("neta1", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "peta1" => { validate_finite_parameter("peta1", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "weta1" => { validate_finite_parameter("weta1", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "p2eta1" => { validate_finite_parameter("p2eta1", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "eta0r" => { validate_finite_parameter("eta0r", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "leta0r" => { validate_finite_parameter("leta0r", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "neta0r" => { validate_finite_parameter("neta0r", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "peta0r" => { validate_finite_parameter("peta0r", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "weta0r" => { validate_finite_parameter("weta0r", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "p2eta0r" => { validate_finite_parameter("p2eta0r", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "eta0cv" => { validate_finite_parameter("eta0cv", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "leta0cv" => { validate_finite_parameter("leta0cv", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "neta0cv" => { validate_finite_parameter("neta0cv", value)?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "peta0cv" => { validate_finite_parameter("peta0cv", value)?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "weta0cv" => { validate_finite_parameter("weta0cv", value)?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "p2eta0cv" => { validate_finite_parameter("p2eta0cv", value)?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "dsub" => { validate_finite_parameter("dsub", value)?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "ldsub" => { validate_finite_parameter("ldsub", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "ndsub" => { validate_finite_parameter("ndsub", value)?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "pdsub" => { validate_finite_parameter("pdsub", value)?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "wdsub" => { validate_finite_parameter("wdsub", value)?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "p2dsub" => { validate_finite_parameter("p2dsub", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "k1rsce" => { validate_finite_parameter("k1rsce", value)?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "lk1rsce" => { validate_finite_parameter("lk1rsce", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "nk1rsce" => { validate_finite_parameter("nk1rsce", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "pk1rsce" => { validate_finite_parameter("pk1rsce", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "wk1rsce" => { validate_finite_parameter("wk1rsce", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "p2k1rsce" => { validate_finite_parameter("p2k1rsce", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "lpe0" => { validate_finite_parameter("lpe0", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "llpe0" => { validate_finite_parameter("llpe0", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "nlpe0" => { validate_finite_parameter("nlpe0", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "plpe0" => { validate_finite_parameter("plpe0", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "wlpe0" => { validate_finite_parameter("wlpe0", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "p2lpe0" => { validate_finite_parameter("p2lpe0", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "dvtshift" => { validate_finite_parameter("dvtshift", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "ldvtshift" => { validate_finite_parameter("ldvtshift", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "ndvtshift" => { validate_finite_parameter("ndvtshift", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            "pdvtshift" => { validate_finite_parameter("pdvtshift", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); Ok(()) }
            "wdvtshift" => { validate_finite_parameter("wdvtshift", value)?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); Ok(()) }
            "p2dvtshift" => { validate_finite_parameter("p2dvtshift", value)?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); Ok(()) }
            "dvtshiftr" => { validate_finite_parameter("dvtshiftr", value)?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); Ok(()) }
            "ldvtshiftr" => { validate_finite_parameter("ldvtshiftr", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); Ok(()) }
            "ndvtshiftr" => { validate_finite_parameter("ndvtshiftr", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); Ok(()) }
            "pdvtshiftr" => { validate_finite_parameter("pdvtshiftr", value)?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); Ok(()) }
            "wdvtshiftr" => { validate_finite_parameter("wdvtshiftr", value)?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); Ok(()) }
            "p2dvtshiftr" => { validate_finite_parameter("p2dvtshiftr", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); Ok(()) }
            "k0" => { validate_finite_parameter("k0", value)?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); Ok(()) }
            "lk0" => { validate_finite_parameter("lk0", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); Ok(()) }
            "nk0" => { validate_finite_parameter("nk0", value)?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); Ok(()) }
            "pk0" => { validate_finite_parameter("pk0", value)?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); Ok(()) }
            "wk0" => { validate_finite_parameter("wk0", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); Ok(()) }
            "p2k0" => { validate_finite_parameter("p2k0", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); Ok(()) }
            "k01" => { validate_finite_parameter("k01", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); Ok(()) }
            "lk01" => { validate_finite_parameter("lk01", value)?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); Ok(()) }
            "nk01" => { validate_finite_parameter("nk01", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); Ok(()) }
            "pk01" => { validate_finite_parameter("pk01", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); Ok(()) }
            "wk01" => { validate_finite_parameter("wk01", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); Ok(()) }
            "p2k01" => { validate_finite_parameter("p2k01", value)?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); Ok(()) }
            "k0si" => { validate_finite_parameter("k0si", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); Ok(()) }
            "lk0si" => { validate_finite_parameter("lk0si", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); Ok(()) }
            "nk0si" => { validate_finite_parameter("nk0si", value)?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); Ok(()) }
            "pk0si" => { validate_finite_parameter("pk0si", value)?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); Ok(()) }
            "wk0si" => { validate_finite_parameter("wk0si", value)?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); Ok(()) }
            "p2k0si" => { validate_finite_parameter("p2k0si", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); Ok(()) }
            "k0si1" => { validate_finite_parameter("k0si1", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); Ok(()) }
            "lk0si1" => { validate_finite_parameter("lk0si1", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); Ok(()) }
            "nk0si1" => { validate_finite_parameter("nk0si1", value)?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); Ok(()) }
            "pk0si1" => { validate_finite_parameter("pk0si1", value)?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); Ok(()) }
            "wk0si1" => { validate_finite_parameter("wk0si1", value)?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); Ok(()) }
            "p2k0si1" => { validate_finite_parameter("p2k0si1", value)?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); Ok(()) }
            "k2si" => { validate_finite_parameter("k2si", value)?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); Ok(()) }
            "lk2si" => { validate_finite_parameter("lk2si", value)?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); Ok(()) }
            "nk2si" => { validate_finite_parameter("nk2si", value)?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); Ok(()) }
            "pk2si" => { validate_finite_parameter("pk2si", value)?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); Ok(()) }
            "wk2si" => { validate_finite_parameter("wk2si", value)?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); Ok(()) }
            "p2k2si" => { validate_finite_parameter("p2k2si", value)?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); Ok(()) }
            "k2si1" => { validate_finite_parameter("k2si1", value)?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); Ok(()) }
            "lk2si1" => { validate_finite_parameter("lk2si1", value)?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); Ok(()) }
            "nk2si1" => { validate_finite_parameter("nk2si1", value)?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); Ok(()) }
            "pk2si1" => { validate_finite_parameter("pk2si1", value)?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); Ok(()) }
            "wk2si1" => { validate_finite_parameter("wk2si1", value)?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); Ok(()) }
            "p2k2si1" => { validate_finite_parameter("p2k2si1", value)?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); Ok(()) }
            "k0sisat" => { validate_finite_parameter("k0sisat", value)?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); Ok(()) }
            "lk0sisat" => { validate_finite_parameter("lk0sisat", value)?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); Ok(()) }
            "nk0sisat" => { validate_finite_parameter("nk0sisat", value)?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); Ok(()) }
            "pk0sisat" => { validate_finite_parameter("pk0sisat", value)?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); Ok(()) }
            "wk0sisat" => { validate_finite_parameter("wk0sisat", value)?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); Ok(()) }
            "p2k0sisat" => { validate_finite_parameter("p2k0sisat", value)?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); Ok(()) }
            "k0sisat1" => { validate_finite_parameter("k0sisat1", value)?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); Ok(()) }
            "lk0sisat1" => { validate_finite_parameter("lk0sisat1", value)?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); Ok(()) }
            "nk0sisat1" => { validate_finite_parameter("nk0sisat1", value)?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); Ok(()) }
            "pk0sisat1" => { validate_finite_parameter("pk0sisat1", value)?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); Ok(()) }
            "wk0sisat1" => { validate_finite_parameter("wk0sisat1", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); Ok(()) }
            "p2k0sisat1" => { validate_finite_parameter("p2k0sisat1", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); Ok(()) }
            "k2sisat" => { validate_finite_parameter("k2sisat", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); Ok(()) }
            "lk2sisat" => { validate_finite_parameter("lk2sisat", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); Ok(()) }
            "nk2sisat" => { validate_finite_parameter("nk2sisat", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); Ok(()) }
            "pk2sisat" => { validate_finite_parameter("pk2sisat", value)?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); Ok(()) }
            "wk2sisat" => { validate_finite_parameter("wk2sisat", value)?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); Ok(()) }
            "p2k2sisat" => { validate_finite_parameter("p2k2sisat", value)?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); Ok(()) }
            "k2sisat1" => { validate_finite_parameter("k2sisat1", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); Ok(()) }
            "lk2sisat1" => { validate_finite_parameter("lk2sisat1", value)?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); Ok(()) }
            "nk2sisat1" => { validate_finite_parameter("nk2sisat1", value)?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); Ok(()) }
            "pk2sisat1" => { validate_finite_parameter("pk2sisat1", value)?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); Ok(()) }
            "wk2sisat1" => { validate_finite_parameter("wk2sisat1", value)?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); Ok(()) }
            "p2k2sisat1" => { validate_finite_parameter("p2k2sisat1", value)?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); Ok(()) }
            "phibe" => { validate_finite_parameter("phibe", value)?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); Ok(()) }
            "lphibe" => { validate_finite_parameter("lphibe", value)?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); Ok(()) }
            "nphibe" => { validate_finite_parameter("nphibe", value)?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); Ok(()) }
            "pphibe" => { validate_finite_parameter("pphibe", value)?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); Ok(()) }
            "wphibe" => { validate_finite_parameter("wphibe", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); Ok(()) }
            "p2phibe" => { validate_finite_parameter("p2phibe", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); Ok(()) }
            "k1" => { validate_finite_parameter("k1", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); Ok(()) }
            "lk1" => { validate_finite_parameter("lk1", value)?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); Ok(()) }
            "nk1" => { validate_finite_parameter("nk1", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); Ok(()) }
            "pk1" => { validate_finite_parameter("pk1", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); Ok(()) }
            "wk1" => { validate_finite_parameter("wk1", value)?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); Ok(()) }
            "p2k1" => { validate_finite_parameter("p2k1", value)?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); Ok(()) }
            "k11" => { validate_finite_parameter("k11", value)?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); Ok(()) }
            "lk11" => { validate_finite_parameter("lk11", value)?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); Ok(()) }
            "nk11" => { validate_finite_parameter("nk11", value)?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); Ok(()) }
            "pk11" => { validate_finite_parameter("pk11", value)?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); Ok(()) }
            "wk11" => { validate_finite_parameter("wk11", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); Ok(()) }
            "p2k11" => { validate_finite_parameter("p2k11", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); Ok(()) }
            "k2sat" => { validate_finite_parameter("k2sat", value)?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); Ok(()) }
            "lk2sat" => { validate_finite_parameter("lk2sat", value)?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); Ok(()) }
            "nk2sat" => { validate_finite_parameter("nk2sat", value)?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); Ok(()) }
            "pk2sat" => { validate_finite_parameter("pk2sat", value)?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); Ok(()) }
            "wk2sat" => { validate_finite_parameter("wk2sat", value)?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); Ok(()) }
            "p2k2sat" => { validate_finite_parameter("p2k2sat", value)?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); Ok(()) }
            "k2sat1" => { validate_finite_parameter("k2sat1", value)?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); Ok(()) }
            "lk2sat1" => { validate_finite_parameter("lk2sat1", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); Ok(()) }
            "nk2sat1" => { validate_finite_parameter("nk2sat1", value)?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); Ok(()) }
            "pk2sat1" => { validate_finite_parameter("pk2sat1", value)?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); Ok(()) }
            "wk2sat1" => { validate_finite_parameter("wk2sat1", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); Ok(()) }
            "p2k2sat1" => { validate_finite_parameter("p2k2sat1", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); Ok(()) }
            "k2" => { validate_finite_parameter("k2", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); Ok(()) }
            "lk2" => { validate_finite_parameter("lk2", value)?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); Ok(()) }
            "nk2" => { validate_finite_parameter("nk2", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); Ok(()) }
            "pk2" => { validate_finite_parameter("pk2", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); Ok(()) }
            "wk2" => { validate_finite_parameter("wk2", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); Ok(()) }
            "p2k2" => { validate_finite_parameter("p2k2", value)?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); Ok(()) }
            "k21" => { validate_finite_parameter("k21", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); Ok(()) }
            "lk21" => { validate_finite_parameter("lk21", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); Ok(()) }
            "nk21" => { validate_finite_parameter("nk21", value)?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); Ok(()) }
            "pk21" => { validate_finite_parameter("pk21", value)?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); Ok(()) }
            "wk21" => { validate_finite_parameter("wk21", value)?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); Ok(()) }
            "p2k21" => { validate_finite_parameter("p2k21", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); Ok(()) }
            "aqmtcen" => { validate_finite_parameter("aqmtcen", value)?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); Ok(()) }
            "bqmtcen" => { validate_finite_parameter("bqmtcen", value)?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); Ok(()) }
            "qm0" => { validate_parameter("qm0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); Ok(()) }
            "qm0acc" => { validate_parameter("qm0acc", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); Ok(()) }
            "pqmacc" => { validate_finite_parameter("pqmacc", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); Ok(()) }
            "qmfactor" => { validate_finite_parameter("qmfactor", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); Ok(()) }
            "lqmfactor" => { validate_finite_parameter("lqmfactor", value)?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); Ok(()) }
            "nqmfactor" => { validate_finite_parameter("nqmfactor", value)?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); Ok(()) }
            "pqmfactor" => { validate_finite_parameter("pqmfactor", value)?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); Ok(()) }
            "wqmfactor" => { validate_finite_parameter("wqmfactor", value)?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); Ok(()) }
            "p2qmfactor" => { validate_finite_parameter("p2qmfactor", value)?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); Ok(()) }
            "qmtcencv" => { validate_finite_parameter("qmtcencv", value)?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); Ok(()) }
            "lqmtcencv" => { validate_finite_parameter("lqmtcencv", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); Ok(()) }
            "nqmtcencv" => { validate_finite_parameter("nqmtcencv", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); Ok(()) }
            "pqmtcencv" => { validate_finite_parameter("pqmtcencv", value)?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); Ok(()) }
            "wqmtcencv" => { validate_finite_parameter("wqmtcencv", value)?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); Ok(()) }
            "p2qmtcencv" => { validate_finite_parameter("p2qmtcencv", value)?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); Ok(()) }
            "qmtcencva" => { validate_finite_parameter("qmtcencva", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); Ok(()) }
            "lqmtcencva" => { validate_finite_parameter("lqmtcencva", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); Ok(()) }
            "nqmtcencva" => { validate_finite_parameter("nqmtcencva", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); Ok(()) }
            "pqmtcencva" => { validate_finite_parameter("pqmtcencva", value)?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); Ok(()) }
            "wqmtcencva" => { validate_finite_parameter("wqmtcencva", value)?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); Ok(()) }
            "p2qmtcencva" => { validate_finite_parameter("p2qmtcencva", value)?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); Ok(()) }
            "pqm" => { validate_finite_parameter("pqm", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); Ok(()) }
            "lpqm" => { validate_finite_parameter("lpqm", value)?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); Ok(()) }
            "npqm" => { validate_finite_parameter("npqm", value)?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); Ok(()) }
            "ppqm" => { validate_finite_parameter("ppqm", value)?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); Ok(()) }
            "wpqm" => { validate_finite_parameter("wpqm", value)?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); Ok(()) }
            "p2pqm" => { validate_finite_parameter("p2pqm", value)?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); Ok(()) }
            "pqml" => { validate_finite_parameter("pqml", value)?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); Ok(()) }
            "vsatn1" => { validate_finite_parameter("vsatn1", value)?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); Ok(()) }
            "vsatn2" => { validate_finite_parameter("vsatn2", value)?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); Ok(()) }
            "avsat" => { validate_finite_parameter("avsat", value)?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); Ok(()) }
            "bvsat" => { validate_finite_parameter("bvsat", value)?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); Ok(()) }
            "vsat1n1" => { validate_finite_parameter("vsat1n1", value)?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); Ok(()) }
            "vsat1n2" => { validate_finite_parameter("vsat1n2", value)?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); Ok(()) }
            "vsat1rn1" => { validate_finite_parameter("vsat1rn1", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); Ok(()) }
            "vsat1rn2" => { validate_finite_parameter("vsat1rn2", value)?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); Ok(()) }
            "avsat1" => { validate_finite_parameter("avsat1", value)?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); Ok(()) }
            "bvsat1" => { validate_finite_parameter("bvsat1", value)?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); Ok(()) }
            "apsat" => { validate_finite_parameter("apsat", value)?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); Ok(()) }
            "bpsat" => { validate_finite_parameter("bpsat", value)?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); Ok(()) }
            "avsatcv" => { validate_finite_parameter("avsatcv", value)?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); Ok(()) }
            "bvsatcv" => { validate_finite_parameter("bvsatcv", value)?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); Ok(()) }
            "apsatcv" => { validate_finite_parameter("apsatcv", value)?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); Ok(()) }
            "bpsatcv" => { validate_finite_parameter("bpsatcv", value)?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); Ok(()) }
            "amexp" => { validate_finite_parameter("amexp", value)?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); Ok(()) }
            "bmexp" => { validate_finite_parameter("bmexp", value)?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); Ok(()) }
            "amexpr" => { validate_finite_parameter("amexpr", value)?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); Ok(()) }
            "bmexpr" => { validate_finite_parameter("bmexpr", value)?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); Ok(()) }
            "aptwg" => { validate_finite_parameter("aptwg", value)?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); Ok(()) }
            "bptwg" => { validate_finite_parameter("bptwg", value)?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); Ok(()) }
            "tmexp" => { validate_finite_parameter("tmexp", value)?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); Ok(()) }
            "tmexp2" => { validate_finite_parameter("tmexp2", value)?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); Ok(()) }
            "tmexpr" => { validate_finite_parameter("tmexpr", value)?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); Ok(()) }
            "dvsatclamp" => { validate_parameter("dvsatclamp", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); Ok(()) }
            "vsatdr" => { validate_finite_parameter("vsatdr", value)?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); Ok(()) }
            "vsat" => { validate_finite_parameter("vsat", value)?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); Ok(()) }
            "lvsat" => { validate_finite_parameter("lvsat", value)?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); Ok(()) }
            "nvsat" => { validate_finite_parameter("nvsat", value)?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); Ok(()) }
            "pvsat" => { validate_finite_parameter("pvsat", value)?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); Ok(()) }
            "wvsat" => { validate_finite_parameter("wvsat", value)?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); Ok(()) }
            "p2vsat" => { validate_finite_parameter("p2vsat", value)?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); Ok(()) }
            "vsatr" => { validate_finite_parameter("vsatr", value)?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); Ok(()) }
            "lvsatr" => { validate_finite_parameter("lvsatr", value)?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); Ok(()) }
            "nvsatr" => { validate_finite_parameter("nvsatr", value)?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); Ok(()) }
            "pvsatr" => { validate_finite_parameter("pvsatr", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); Ok(()) }
            "wvsatr" => { validate_finite_parameter("wvsatr", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); Ok(()) }
            "p2vsatr" => { validate_finite_parameter("p2vsatr", value)?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); Ok(()) }
            "vsat1" => { validate_finite_parameter("vsat1", value)?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); Ok(()) }
            "lvsat1" => { validate_finite_parameter("lvsat1", value)?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); Ok(()) }
            "nvsat1" => { validate_finite_parameter("nvsat1", value)?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); Ok(()) }
            "pvsat1" => { validate_finite_parameter("pvsat1", value)?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); Ok(()) }
            "wvsat1" => { validate_finite_parameter("wvsat1", value)?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); Ok(()) }
            "p2vsat1" => { validate_finite_parameter("p2vsat1", value)?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); Ok(()) }
            "vsat1r" => { validate_finite_parameter("vsat1r", value)?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); Ok(()) }
            "lvsat1r" => { validate_finite_parameter("lvsat1r", value)?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); Ok(()) }
            "nvsat1r" => { validate_finite_parameter("nvsat1r", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); Ok(()) }
            "pvsat1r" => { validate_finite_parameter("pvsat1r", value)?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); Ok(()) }
            "wvsat1r" => { validate_finite_parameter("wvsat1r", value)?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); Ok(()) }
            "p2vsat1r" => { validate_finite_parameter("p2vsat1r", value)?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); Ok(()) }
            "deltavsat" => { validate_finite_parameter("deltavsat", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); Ok(()) }
            "ldeltavsat" => { validate_finite_parameter("ldeltavsat", value)?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); Ok(()) }
            "ndeltavsat" => { validate_finite_parameter("ndeltavsat", value)?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); Ok(()) }
            "pdeltavsat" => { validate_finite_parameter("pdeltavsat", value)?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); Ok(()) }
            "wdeltavsat" => { validate_finite_parameter("wdeltavsat", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); Ok(()) }
            "p2deltavsat" => { validate_finite_parameter("p2deltavsat", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); Ok(()) }
            "psat" => { validate_finite_parameter("psat", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); Ok(()) }
            "lpsat" => { validate_finite_parameter("lpsat", value)?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); Ok(()) }
            "npsat" => { validate_finite_parameter("npsat", value)?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); Ok(()) }
            "ppsat" => { validate_finite_parameter("ppsat", value)?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); Ok(()) }
            "wpsat" => { validate_finite_parameter("wpsat", value)?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); Ok(()) }
            "p2psat" => { validate_finite_parameter("p2psat", value)?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); Ok(()) }
            "ksativdr" => { validate_finite_parameter("ksativdr", value)?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); Ok(()) }
            "ksativ" => { validate_finite_parameter("ksativ", value)?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); Ok(()) }
            "lksativ" => { validate_finite_parameter("lksativ", value)?; self.params.p493 = value; self.mark_param_given(493); self.recompute_instance_static(); Ok(()) }
            "nksativ" => { validate_finite_parameter("nksativ", value)?; self.params.p494 = value; self.mark_param_given(494); self.recompute_instance_static(); Ok(()) }
            "pksativ" => { validate_finite_parameter("pksativ", value)?; self.params.p495 = value; self.mark_param_given(495); self.recompute_instance_static(); Ok(()) }
            "wksativ" => { validate_finite_parameter("wksativ", value)?; self.params.p496 = value; self.mark_param_given(496); self.recompute_instance_static(); Ok(()) }
            "p2ksativ" => { validate_finite_parameter("p2ksativ", value)?; self.params.p497 = value; self.mark_param_given(497); self.recompute_instance_static(); Ok(()) }
            "ksativt1" => { validate_finite_parameter("ksativt1", value)?; self.params.p498 = value; self.mark_param_given(498); self.recompute_instance_static(); Ok(()) }
            "ksativt2" => { validate_finite_parameter("ksativt2", value)?; self.params.p499 = value; self.mark_param_given(499); self.recompute_instance_static(); Ok(()) }
            "ksativr" => { validate_finite_parameter("ksativr", value)?; self.params.p500 = value; self.mark_param_given(500); self.recompute_instance_static(); Ok(()) }
            "lksativr" => { validate_finite_parameter("lksativr", value)?; self.params.p501 = value; self.mark_param_given(501); self.recompute_instance_static(); Ok(()) }
            "nksativr" => { validate_finite_parameter("nksativr", value)?; self.params.p502 = value; self.mark_param_given(502); self.recompute_instance_static(); Ok(()) }
            "pksativr" => { validate_finite_parameter("pksativr", value)?; self.params.p503 = value; self.mark_param_given(503); self.recompute_instance_static(); Ok(()) }
            "wksativr" => { validate_finite_parameter("wksativr", value)?; self.params.p504 = value; self.mark_param_given(504); self.recompute_instance_static(); Ok(()) }
            "p2ksativr" => { validate_finite_parameter("p2ksativr", value)?; self.params.p505 = value; self.mark_param_given(505); self.recompute_instance_static(); Ok(()) }
            "vsatcv" => { validate_finite_parameter("vsatcv", value)?; self.params.p506 = value; self.mark_param_given(506); self.recompute_instance_static(); Ok(()) }
            "lvsatcv" => { validate_finite_parameter("lvsatcv", value)?; self.params.p507 = value; self.mark_param_given(507); self.recompute_instance_static(); Ok(()) }
            "nvsatcv" => { validate_finite_parameter("nvsatcv", value)?; self.params.p508 = value; self.mark_param_given(508); self.recompute_instance_static(); Ok(()) }
            "pvsatcv" => { validate_finite_parameter("pvsatcv", value)?; self.params.p509 = value; self.mark_param_given(509); self.recompute_instance_static(); Ok(()) }
            "wvsatcv" => { validate_finite_parameter("wvsatcv", value)?; self.params.p510 = value; self.mark_param_given(510); self.recompute_instance_static(); Ok(()) }
            "p2vsatcv" => { validate_finite_parameter("p2vsatcv", value)?; self.params.p511 = value; self.mark_param_given(511); self.recompute_instance_static(); Ok(()) }
            "asat" => { validate_finite_parameter("asat", value)?; self.params.p512 = value; self.mark_param_given(512); self.recompute_instance_static(); Ok(()) }
            "lasat" => { validate_finite_parameter("lasat", value)?; self.params.p513 = value; self.mark_param_given(513); self.recompute_instance_static(); Ok(()) }
            "nasat" => { validate_finite_parameter("nasat", value)?; self.params.p514 = value; self.mark_param_given(514); self.recompute_instance_static(); Ok(()) }
            "pasat" => { validate_finite_parameter("pasat", value)?; self.params.p515 = value; self.mark_param_given(515); self.recompute_instance_static(); Ok(()) }
            "wasat" => { validate_finite_parameter("wasat", value)?; self.params.p516 = value; self.mark_param_given(516); self.recompute_instance_static(); Ok(()) }
            "p2asat" => { validate_finite_parameter("p2asat", value)?; self.params.p517 = value; self.mark_param_given(517); self.recompute_instance_static(); Ok(()) }
            "deltavsatcv" => { validate_finite_parameter("deltavsatcv", value)?; self.params.p518 = value; self.mark_param_given(518); self.recompute_instance_static(); Ok(()) }
            "ldeltavsatcv" => { validate_finite_parameter("ldeltavsatcv", value)?; self.params.p519 = value; self.mark_param_given(519); self.recompute_instance_static(); Ok(()) }
            "ndeltavsatcv" => { validate_finite_parameter("ndeltavsatcv", value)?; self.params.p520 = value; self.mark_param_given(520); self.recompute_instance_static(); Ok(()) }
            "pdeltavsatcv" => { validate_finite_parameter("pdeltavsatcv", value)?; self.params.p521 = value; self.mark_param_given(521); self.recompute_instance_static(); Ok(()) }
            "wdeltavsatcv" => { validate_finite_parameter("wdeltavsatcv", value)?; self.params.p522 = value; self.mark_param_given(522); self.recompute_instance_static(); Ok(()) }
            "p2deltavsatcv" => { validate_finite_parameter("p2deltavsatcv", value)?; self.params.p523 = value; self.mark_param_given(523); self.recompute_instance_static(); Ok(()) }
            "psatcv" => { validate_finite_parameter("psatcv", value)?; self.params.p524 = value; self.mark_param_given(524); self.recompute_instance_static(); Ok(()) }
            "lpsatcv" => { validate_finite_parameter("lpsatcv", value)?; self.params.p525 = value; self.mark_param_given(525); self.recompute_instance_static(); Ok(()) }
            "npsatcv" => { validate_finite_parameter("npsatcv", value)?; self.params.p526 = value; self.mark_param_given(526); self.recompute_instance_static(); Ok(()) }
            "ppsatcv" => { validate_finite_parameter("ppsatcv", value)?; self.params.p527 = value; self.mark_param_given(527); self.recompute_instance_static(); Ok(()) }
            "wpsatcv" => { validate_finite_parameter("wpsatcv", value)?; self.params.p528 = value; self.mark_param_given(528); self.recompute_instance_static(); Ok(()) }
            "p2psatcv" => { validate_finite_parameter("p2psatcv", value)?; self.params.p529 = value; self.mark_param_given(529); self.recompute_instance_static(); Ok(()) }
            "mexpdr" => { validate_parameter("mexpdr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p530 = value; self.mark_param_given(530); self.recompute_instance_static(); Ok(()) }
            "mexp" => { validate_finite_parameter("mexp", value)?; self.params.p531 = value; self.mark_param_given(531); self.recompute_instance_static(); Ok(()) }
            "lmexp" => { validate_finite_parameter("lmexp", value)?; self.params.p532 = value; self.mark_param_given(532); self.recompute_instance_static(); Ok(()) }
            "nmexp" => { validate_finite_parameter("nmexp", value)?; self.params.p533 = value; self.mark_param_given(533); self.recompute_instance_static(); Ok(()) }
            "pmexp" => { validate_finite_parameter("pmexp", value)?; self.params.p534 = value; self.mark_param_given(534); self.recompute_instance_static(); Ok(()) }
            "wmexp" => { validate_finite_parameter("wmexp", value)?; self.params.p535 = value; self.mark_param_given(535); self.recompute_instance_static(); Ok(()) }
            "p2mexp" => { validate_finite_parameter("p2mexp", value)?; self.params.p536 = value; self.mark_param_given(536); self.recompute_instance_static(); Ok(()) }
            "mexpr" => { validate_finite_parameter("mexpr", value)?; self.params.p537 = value; self.mark_param_given(537); self.recompute_instance_static(); Ok(()) }
            "lmexpr" => { validate_finite_parameter("lmexpr", value)?; self.params.p538 = value; self.mark_param_given(538); self.recompute_instance_static(); Ok(()) }
            "nmexpr" => { validate_finite_parameter("nmexpr", value)?; self.params.p539 = value; self.mark_param_given(539); self.recompute_instance_static(); Ok(()) }
            "pmexpr" => { validate_finite_parameter("pmexpr", value)?; self.params.p540 = value; self.mark_param_given(540); self.recompute_instance_static(); Ok(()) }
            "wmexpr" => { validate_finite_parameter("wmexpr", value)?; self.params.p541 = value; self.mark_param_given(541); self.recompute_instance_static(); Ok(()) }
            "p2mexpr" => { validate_finite_parameter("p2mexpr", value)?; self.params.p542 = value; self.mark_param_given(542); self.recompute_instance_static(); Ok(()) }
            "ptwg" => { validate_finite_parameter("ptwg", value)?; self.params.p543 = value; self.mark_param_given(543); self.recompute_instance_static(); Ok(()) }
            "lptwg" => { validate_finite_parameter("lptwg", value)?; self.params.p544 = value; self.mark_param_given(544); self.recompute_instance_static(); Ok(()) }
            "nptwg" => { validate_finite_parameter("nptwg", value)?; self.params.p545 = value; self.mark_param_given(545); self.recompute_instance_static(); Ok(()) }
            "pptwg" => { validate_finite_parameter("pptwg", value)?; self.params.p546 = value; self.mark_param_given(546); self.recompute_instance_static(); Ok(()) }
            "wptwg" => { validate_finite_parameter("wptwg", value)?; self.params.p547 = value; self.mark_param_given(547); self.recompute_instance_static(); Ok(()) }
            "p2ptwg" => { validate_finite_parameter("p2ptwg", value)?; self.params.p548 = value; self.mark_param_given(548); self.recompute_instance_static(); Ok(()) }
            "ptwgr" => { validate_finite_parameter("ptwgr", value)?; self.params.p549 = value; self.mark_param_given(549); self.recompute_instance_static(); Ok(()) }
            "lptwgr" => { validate_finite_parameter("lptwgr", value)?; self.params.p550 = value; self.mark_param_given(550); self.recompute_instance_static(); Ok(()) }
            "nptwgr" => { validate_finite_parameter("nptwgr", value)?; self.params.p551 = value; self.mark_param_given(551); self.recompute_instance_static(); Ok(()) }
            "pptwgr" => { validate_finite_parameter("pptwgr", value)?; self.params.p552 = value; self.mark_param_given(552); self.recompute_instance_static(); Ok(()) }
            "wptwgr" => { validate_finite_parameter("wptwgr", value)?; self.params.p553 = value; self.mark_param_given(553); self.recompute_instance_static(); Ok(()) }
            "p2ptwgr" => { validate_finite_parameter("p2ptwgr", value)?; self.params.p554 = value; self.mark_param_given(554); self.recompute_instance_static(); Ok(()) }
            "at" => { validate_finite_parameter("at", value)?; self.params.p555 = value; self.mark_param_given(555); self.recompute_instance_static(); Ok(()) }
            "lat" => { validate_finite_parameter("lat", value)?; self.params.p556 = value; self.mark_param_given(556); self.recompute_instance_static(); Ok(()) }
            "nat" => { validate_finite_parameter("nat", value)?; self.params.p557 = value; self.mark_param_given(557); self.recompute_instance_static(); Ok(()) }
            "pat" => { validate_finite_parameter("pat", value)?; self.params.p558 = value; self.mark_param_given(558); self.recompute_instance_static(); Ok(()) }
            "wat" => { validate_finite_parameter("wat", value)?; self.params.p559 = value; self.mark_param_given(559); self.recompute_instance_static(); Ok(()) }
            "p2at" => { validate_finite_parameter("p2at", value)?; self.params.p560 = value; self.mark_param_given(560); self.recompute_instance_static(); Ok(()) }
            "at2" => { validate_finite_parameter("at2", value)?; self.params.p561 = value; self.mark_param_given(561); self.recompute_instance_static(); Ok(()) }
            "atr" => { validate_finite_parameter("atr", value)?; self.params.p562 = value; self.mark_param_given(562); self.recompute_instance_static(); Ok(()) }
            "latr" => { validate_finite_parameter("latr", value)?; self.params.p563 = value; self.mark_param_given(563); self.recompute_instance_static(); Ok(()) }
            "natr" => { validate_finite_parameter("natr", value)?; self.params.p564 = value; self.mark_param_given(564); self.recompute_instance_static(); Ok(()) }
            "patr" => { validate_finite_parameter("patr", value)?; self.params.p565 = value; self.mark_param_given(565); self.recompute_instance_static(); Ok(()) }
            "watr" => { validate_finite_parameter("watr", value)?; self.params.p566 = value; self.mark_param_given(566); self.recompute_instance_static(); Ok(()) }
            "p2atr" => { validate_finite_parameter("p2atr", value)?; self.params.p567 = value; self.mark_param_given(567); self.recompute_instance_static(); Ok(()) }
            "atcv" => { validate_finite_parameter("atcv", value)?; self.params.p568 = value; self.mark_param_given(568); self.recompute_instance_static(); Ok(()) }
            "latcv" => { validate_finite_parameter("latcv", value)?; self.params.p569 = value; self.mark_param_given(569); self.recompute_instance_static(); Ok(()) }
            "natcv" => { validate_finite_parameter("natcv", value)?; self.params.p570 = value; self.mark_param_given(570); self.recompute_instance_static(); Ok(()) }
            "patcv" => { validate_finite_parameter("patcv", value)?; self.params.p571 = value; self.mark_param_given(571); self.recompute_instance_static(); Ok(()) }
            "watcv" => { validate_finite_parameter("watcv", value)?; self.params.p572 = value; self.mark_param_given(572); self.recompute_instance_static(); Ok(()) }
            "p2atcv" => { validate_finite_parameter("p2atcv", value)?; self.params.p573 = value; self.mark_param_given(573); self.recompute_instance_static(); Ok(()) }
            "at2cv" => { validate_finite_parameter("at2cv", value)?; self.params.p574 = value; self.mark_param_given(574); self.recompute_instance_static(); Ok(()) }
            "ptwgt" => { validate_finite_parameter("ptwgt", value)?; self.params.p575 = value; self.mark_param_given(575); self.recompute_instance_static(); Ok(()) }
            "lptwgt" => { validate_finite_parameter("lptwgt", value)?; self.params.p576 = value; self.mark_param_given(576); self.recompute_instance_static(); Ok(()) }
            "nptwgt" => { validate_finite_parameter("nptwgt", value)?; self.params.p577 = value; self.mark_param_given(577); self.recompute_instance_static(); Ok(()) }
            "pptwgt" => { validate_finite_parameter("pptwgt", value)?; self.params.p578 = value; self.mark_param_given(578); self.recompute_instance_static(); Ok(()) }
            "wptwgt" => { validate_finite_parameter("wptwgt", value)?; self.params.p579 = value; self.mark_param_given(579); self.recompute_instance_static(); Ok(()) }
            "p2ptwgt" => { validate_finite_parameter("p2ptwgt", value)?; self.params.p580 = value; self.mark_param_given(580); self.recompute_instance_static(); Ok(()) }
            "u0n1" => { validate_finite_parameter("u0n1", value)?; self.params.p581 = value; self.mark_param_given(581); self.recompute_instance_static(); Ok(()) }
            "u0n1cv" => { validate_finite_parameter("u0n1cv", value)?; self.params.p582 = value; self.mark_param_given(582); self.recompute_instance_static(); Ok(()) }
            "u0n1r" => { validate_finite_parameter("u0n1r", value)?; self.params.p583 = value; self.mark_param_given(583); self.recompute_instance_static(); Ok(()) }
            "u0n2" => { validate_finite_parameter("u0n2", value)?; self.params.p584 = value; self.mark_param_given(584); self.recompute_instance_static(); Ok(()) }
            "u0n2cv" => { validate_finite_parameter("u0n2cv", value)?; self.params.p585 = value; self.mark_param_given(585); self.recompute_instance_static(); Ok(()) }
            "u0n2r" => { validate_finite_parameter("u0n2r", value)?; self.params.p586 = value; self.mark_param_given(586); self.recompute_instance_static(); Ok(()) }
            "u0lt" => { validate_finite_parameter("u0lt", value)?; self.params.p587 = value; self.mark_param_given(587); self.recompute_instance_static(); Ok(()) }
            "u0ltcv" => { validate_finite_parameter("u0ltcv", value)?; self.params.p588 = value; self.mark_param_given(588); self.recompute_instance_static(); Ok(()) }
            "lpa" => { validate_finite_parameter("lpa", value)?; self.params.p589 = value; self.mark_param_given(589); self.recompute_instance_static(); Ok(()) }
            "lpar" => { validate_finite_parameter("lpar", value)?; self.params.p590 = value; self.mark_param_given(590); self.recompute_instance_static(); Ok(()) }
            "aua" => { validate_finite_parameter("aua", value)?; self.params.p591 = value; self.mark_param_given(591); self.recompute_instance_static(); Ok(()) }
            "auar" => { validate_finite_parameter("auar", value)?; self.params.p592 = value; self.mark_param_given(592); self.recompute_instance_static(); Ok(()) }
            "bua" => { validate_finite_parameter("bua", value)?; self.params.p593 = value; self.mark_param_given(593); self.recompute_instance_static(); Ok(()) }
            "buar" => { validate_finite_parameter("buar", value)?; self.params.p594 = value; self.mark_param_given(594); self.recompute_instance_static(); Ok(()) }
            "aeu" => { validate_finite_parameter("aeu", value)?; self.params.p595 = value; self.mark_param_given(595); self.recompute_instance_static(); Ok(()) }
            "aeur" => { validate_finite_parameter("aeur", value)?; self.params.p596 = value; self.mark_param_given(596); self.recompute_instance_static(); Ok(()) }
            "beu" => { validate_finite_parameter("beu", value)?; self.params.p597 = value; self.mark_param_given(597); self.recompute_instance_static(); Ok(()) }
            "beur" => { validate_finite_parameter("beur", value)?; self.params.p598 = value; self.mark_param_given(598); self.recompute_instance_static(); Ok(()) }
            "aud" => { validate_finite_parameter("aud", value)?; self.params.p599 = value; self.mark_param_given(599); self.recompute_instance_static(); Ok(()) }
            "audr" => { validate_finite_parameter("audr", value)?; self.params.p600 = value; self.mark_param_given(600); self.recompute_instance_static(); Ok(()) }
            "bud" => { validate_finite_parameter("bud", value)?; self.params.p601 = value; self.mark_param_given(601); self.recompute_instance_static(); Ok(()) }
            "budr" => { validate_finite_parameter("budr", value)?; self.params.p602 = value; self.mark_param_given(602); self.recompute_instance_static(); Ok(()) }
            "chargewf" => { validate_parameter("chargewf", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p603 = value; self.mark_param_given(603); self.recompute_instance_static(); Ok(()) }
            "dmobclamp" => { validate_parameter("dmobclamp", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p604 = value; self.mark_param_given(604); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_finite_parameter("u0", value)?; self.params.p605 = value; self.mark_param_given(605); self.recompute_instance_static(); Ok(()) }
            "lu0" => { validate_finite_parameter("lu0", value)?; self.params.p606 = value; self.mark_param_given(606); self.recompute_instance_static(); Ok(()) }
            "nu0" => { validate_finite_parameter("nu0", value)?; self.params.p607 = value; self.mark_param_given(607); self.recompute_instance_static(); Ok(()) }
            "pu0" => { validate_finite_parameter("pu0", value)?; self.params.p608 = value; self.mark_param_given(608); self.recompute_instance_static(); Ok(()) }
            "wu0" => { validate_finite_parameter("wu0", value)?; self.params.p609 = value; self.mark_param_given(609); self.recompute_instance_static(); Ok(()) }
            "p2u0" => { validate_finite_parameter("p2u0", value)?; self.params.p610 = value; self.mark_param_given(610); self.recompute_instance_static(); Ok(()) }
            "u0r" => { validate_finite_parameter("u0r", value)?; self.params.p611 = value; self.mark_param_given(611); self.recompute_instance_static(); Ok(()) }
            "lu0r" => { validate_finite_parameter("lu0r", value)?; self.params.p612 = value; self.mark_param_given(612); self.recompute_instance_static(); Ok(()) }
            "nu0r" => { validate_finite_parameter("nu0r", value)?; self.params.p613 = value; self.mark_param_given(613); self.recompute_instance_static(); Ok(()) }
            "pu0r" => { validate_finite_parameter("pu0r", value)?; self.params.p614 = value; self.mark_param_given(614); self.recompute_instance_static(); Ok(()) }
            "wu0r" => { validate_finite_parameter("wu0r", value)?; self.params.p615 = value; self.mark_param_given(615); self.recompute_instance_static(); Ok(()) }
            "p2u0r" => { validate_finite_parameter("p2u0r", value)?; self.params.p616 = value; self.mark_param_given(616); self.recompute_instance_static(); Ok(()) }
            "u0cv" => { validate_finite_parameter("u0cv", value)?; self.params.p617 = value; self.mark_param_given(617); self.recompute_instance_static(); Ok(()) }
            "lu0cv" => { validate_finite_parameter("lu0cv", value)?; self.params.p618 = value; self.mark_param_given(618); self.recompute_instance_static(); Ok(()) }
            "nu0cv" => { validate_finite_parameter("nu0cv", value)?; self.params.p619 = value; self.mark_param_given(619); self.recompute_instance_static(); Ok(()) }
            "pu0cv" => { validate_finite_parameter("pu0cv", value)?; self.params.p620 = value; self.mark_param_given(620); self.recompute_instance_static(); Ok(()) }
            "wu0cv" => { validate_finite_parameter("wu0cv", value)?; self.params.p621 = value; self.mark_param_given(621); self.recompute_instance_static(); Ok(()) }
            "p2u0cv" => { validate_finite_parameter("p2u0cv", value)?; self.params.p622 = value; self.mark_param_given(622); self.recompute_instance_static(); Ok(()) }
            "etamob" => { validate_finite_parameter("etamob", value)?; self.params.p623 = value; self.mark_param_given(623); self.recompute_instance_static(); Ok(()) }
            "letamob" => { validate_finite_parameter("letamob", value)?; self.params.p624 = value; self.mark_param_given(624); self.recompute_instance_static(); Ok(()) }
            "netamob" => { validate_finite_parameter("netamob", value)?; self.params.p625 = value; self.mark_param_given(625); self.recompute_instance_static(); Ok(()) }
            "petamob" => { validate_finite_parameter("petamob", value)?; self.params.p626 = value; self.mark_param_given(626); self.recompute_instance_static(); Ok(()) }
            "wetamob" => { validate_finite_parameter("wetamob", value)?; self.params.p627 = value; self.mark_param_given(627); self.recompute_instance_static(); Ok(()) }
            "p2etamob" => { validate_finite_parameter("p2etamob", value)?; self.params.p628 = value; self.mark_param_given(628); self.recompute_instance_static(); Ok(()) }
            "up" => { validate_finite_parameter("up", value)?; self.params.p629 = value; self.mark_param_given(629); self.recompute_instance_static(); Ok(()) }
            "lup" => { validate_finite_parameter("lup", value)?; self.params.p630 = value; self.mark_param_given(630); self.recompute_instance_static(); Ok(()) }
            "nup" => { validate_finite_parameter("nup", value)?; self.params.p631 = value; self.mark_param_given(631); self.recompute_instance_static(); Ok(()) }
            "pup" => { validate_finite_parameter("pup", value)?; self.params.p632 = value; self.mark_param_given(632); self.recompute_instance_static(); Ok(()) }
            "wup" => { validate_finite_parameter("wup", value)?; self.params.p633 = value; self.mark_param_given(633); self.recompute_instance_static(); Ok(()) }
            "p2up" => { validate_finite_parameter("p2up", value)?; self.params.p634 = value; self.mark_param_given(634); self.recompute_instance_static(); Ok(()) }
            "upr" => { validate_finite_parameter("upr", value)?; self.params.p635 = value; self.mark_param_given(635); self.recompute_instance_static(); Ok(()) }
            "lupr" => { validate_finite_parameter("lupr", value)?; self.params.p636 = value; self.mark_param_given(636); self.recompute_instance_static(); Ok(()) }
            "nupr" => { validate_finite_parameter("nupr", value)?; self.params.p637 = value; self.mark_param_given(637); self.recompute_instance_static(); Ok(()) }
            "pupr" => { validate_finite_parameter("pupr", value)?; self.params.p638 = value; self.mark_param_given(638); self.recompute_instance_static(); Ok(()) }
            "wupr" => { validate_finite_parameter("wupr", value)?; self.params.p639 = value; self.mark_param_given(639); self.recompute_instance_static(); Ok(()) }
            "p2upr" => { validate_finite_parameter("p2upr", value)?; self.params.p640 = value; self.mark_param_given(640); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_finite_parameter("ua", value)?; self.params.p641 = value; self.mark_param_given(641); self.recompute_instance_static(); Ok(()) }
            "lua" => { validate_finite_parameter("lua", value)?; self.params.p642 = value; self.mark_param_given(642); self.recompute_instance_static(); Ok(()) }
            "nua" => { validate_finite_parameter("nua", value)?; self.params.p643 = value; self.mark_param_given(643); self.recompute_instance_static(); Ok(()) }
            "pua" => { validate_finite_parameter("pua", value)?; self.params.p644 = value; self.mark_param_given(644); self.recompute_instance_static(); Ok(()) }
            "wua" => { validate_finite_parameter("wua", value)?; self.params.p645 = value; self.mark_param_given(645); self.recompute_instance_static(); Ok(()) }
            "p2ua" => { validate_finite_parameter("p2ua", value)?; self.params.p646 = value; self.mark_param_given(646); self.recompute_instance_static(); Ok(()) }
            "uar" => { validate_finite_parameter("uar", value)?; self.params.p647 = value; self.mark_param_given(647); self.recompute_instance_static(); Ok(()) }
            "luar" => { validate_finite_parameter("luar", value)?; self.params.p648 = value; self.mark_param_given(648); self.recompute_instance_static(); Ok(()) }
            "nuar" => { validate_finite_parameter("nuar", value)?; self.params.p649 = value; self.mark_param_given(649); self.recompute_instance_static(); Ok(()) }
            "puar" => { validate_finite_parameter("puar", value)?; self.params.p650 = value; self.mark_param_given(650); self.recompute_instance_static(); Ok(()) }
            "wuar" => { validate_finite_parameter("wuar", value)?; self.params.p651 = value; self.mark_param_given(651); self.recompute_instance_static(); Ok(()) }
            "p2uar" => { validate_finite_parameter("p2uar", value)?; self.params.p652 = value; self.mark_param_given(652); self.recompute_instance_static(); Ok(()) }
            "uacv" => { validate_finite_parameter("uacv", value)?; self.params.p653 = value; self.mark_param_given(653); self.recompute_instance_static(); Ok(()) }
            "luacv" => { validate_finite_parameter("luacv", value)?; self.params.p654 = value; self.mark_param_given(654); self.recompute_instance_static(); Ok(()) }
            "nuacv" => { validate_finite_parameter("nuacv", value)?; self.params.p655 = value; self.mark_param_given(655); self.recompute_instance_static(); Ok(()) }
            "puacv" => { validate_finite_parameter("puacv", value)?; self.params.p656 = value; self.mark_param_given(656); self.recompute_instance_static(); Ok(()) }
            "wuacv" => { validate_finite_parameter("wuacv", value)?; self.params.p657 = value; self.mark_param_given(657); self.recompute_instance_static(); Ok(()) }
            "p2uacv" => { validate_finite_parameter("p2uacv", value)?; self.params.p658 = value; self.mark_param_given(658); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_finite_parameter("uc", value)?; self.params.p659 = value; self.mark_param_given(659); self.recompute_instance_static(); Ok(()) }
            "luc" => { validate_finite_parameter("luc", value)?; self.params.p660 = value; self.mark_param_given(660); self.recompute_instance_static(); Ok(()) }
            "nuc" => { validate_finite_parameter("nuc", value)?; self.params.p661 = value; self.mark_param_given(661); self.recompute_instance_static(); Ok(()) }
            "puc" => { validate_finite_parameter("puc", value)?; self.params.p662 = value; self.mark_param_given(662); self.recompute_instance_static(); Ok(()) }
            "wuc" => { validate_finite_parameter("wuc", value)?; self.params.p663 = value; self.mark_param_given(663); self.recompute_instance_static(); Ok(()) }
            "p2uc" => { validate_finite_parameter("p2uc", value)?; self.params.p664 = value; self.mark_param_given(664); self.recompute_instance_static(); Ok(()) }
            "ucr" => { validate_finite_parameter("ucr", value)?; self.params.p665 = value; self.mark_param_given(665); self.recompute_instance_static(); Ok(()) }
            "lucr" => { validate_finite_parameter("lucr", value)?; self.params.p666 = value; self.mark_param_given(666); self.recompute_instance_static(); Ok(()) }
            "nucr" => { validate_finite_parameter("nucr", value)?; self.params.p667 = value; self.mark_param_given(667); self.recompute_instance_static(); Ok(()) }
            "pucr" => { validate_finite_parameter("pucr", value)?; self.params.p668 = value; self.mark_param_given(668); self.recompute_instance_static(); Ok(()) }
            "wucr" => { validate_finite_parameter("wucr", value)?; self.params.p669 = value; self.mark_param_given(669); self.recompute_instance_static(); Ok(()) }
            "p2ucr" => { validate_finite_parameter("p2ucr", value)?; self.params.p670 = value; self.mark_param_given(670); self.recompute_instance_static(); Ok(()) }
            "uccv" => { validate_finite_parameter("uccv", value)?; self.params.p671 = value; self.mark_param_given(671); self.recompute_instance_static(); Ok(()) }
            "luccv" => { validate_finite_parameter("luccv", value)?; self.params.p672 = value; self.mark_param_given(672); self.recompute_instance_static(); Ok(()) }
            "nuccv" => { validate_finite_parameter("nuccv", value)?; self.params.p673 = value; self.mark_param_given(673); self.recompute_instance_static(); Ok(()) }
            "puccv" => { validate_finite_parameter("puccv", value)?; self.params.p674 = value; self.mark_param_given(674); self.recompute_instance_static(); Ok(()) }
            "wuccv" => { validate_finite_parameter("wuccv", value)?; self.params.p675 = value; self.mark_param_given(675); self.recompute_instance_static(); Ok(()) }
            "p2uccv" => { validate_finite_parameter("p2uccv", value)?; self.params.p676 = value; self.mark_param_given(676); self.recompute_instance_static(); Ok(()) }
            "eu" => { validate_finite_parameter("eu", value)?; self.params.p677 = value; self.mark_param_given(677); self.recompute_instance_static(); Ok(()) }
            "leu" => { validate_finite_parameter("leu", value)?; self.params.p678 = value; self.mark_param_given(678); self.recompute_instance_static(); Ok(()) }
            "neu" => { validate_finite_parameter("neu", value)?; self.params.p679 = value; self.mark_param_given(679); self.recompute_instance_static(); Ok(()) }
            "peu" => { validate_finite_parameter("peu", value)?; self.params.p680 = value; self.mark_param_given(680); self.recompute_instance_static(); Ok(()) }
            "weu" => { validate_finite_parameter("weu", value)?; self.params.p681 = value; self.mark_param_given(681); self.recompute_instance_static(); Ok(()) }
            "p2eu" => { validate_finite_parameter("p2eu", value)?; self.params.p682 = value; self.mark_param_given(682); self.recompute_instance_static(); Ok(()) }
            "eur" => { validate_finite_parameter("eur", value)?; self.params.p683 = value; self.mark_param_given(683); self.recompute_instance_static(); Ok(()) }
            "leur" => { validate_finite_parameter("leur", value)?; self.params.p684 = value; self.mark_param_given(684); self.recompute_instance_static(); Ok(()) }
            "neur" => { validate_finite_parameter("neur", value)?; self.params.p685 = value; self.mark_param_given(685); self.recompute_instance_static(); Ok(()) }
            "peur" => { validate_finite_parameter("peur", value)?; self.params.p686 = value; self.mark_param_given(686); self.recompute_instance_static(); Ok(()) }
            "weur" => { validate_finite_parameter("weur", value)?; self.params.p687 = value; self.mark_param_given(687); self.recompute_instance_static(); Ok(()) }
            "p2eur" => { validate_finite_parameter("p2eur", value)?; self.params.p688 = value; self.mark_param_given(688); self.recompute_instance_static(); Ok(()) }
            "ud" => { validate_finite_parameter("ud", value)?; self.params.p689 = value; self.mark_param_given(689); self.recompute_instance_static(); Ok(()) }
            "lud" => { validate_finite_parameter("lud", value)?; self.params.p690 = value; self.mark_param_given(690); self.recompute_instance_static(); Ok(()) }
            "nud" => { validate_finite_parameter("nud", value)?; self.params.p691 = value; self.mark_param_given(691); self.recompute_instance_static(); Ok(()) }
            "pud" => { validate_finite_parameter("pud", value)?; self.params.p692 = value; self.mark_param_given(692); self.recompute_instance_static(); Ok(()) }
            "wud" => { validate_finite_parameter("wud", value)?; self.params.p693 = value; self.mark_param_given(693); self.recompute_instance_static(); Ok(()) }
            "p2ud" => { validate_finite_parameter("p2ud", value)?; self.params.p694 = value; self.mark_param_given(694); self.recompute_instance_static(); Ok(()) }
            "udr" => { validate_finite_parameter("udr", value)?; self.params.p695 = value; self.mark_param_given(695); self.recompute_instance_static(); Ok(()) }
            "ludr" => { validate_finite_parameter("ludr", value)?; self.params.p696 = value; self.mark_param_given(696); self.recompute_instance_static(); Ok(()) }
            "nudr" => { validate_finite_parameter("nudr", value)?; self.params.p697 = value; self.mark_param_given(697); self.recompute_instance_static(); Ok(()) }
            "pudr" => { validate_finite_parameter("pudr", value)?; self.params.p698 = value; self.mark_param_given(698); self.recompute_instance_static(); Ok(()) }
            "wudr" => { validate_finite_parameter("wudr", value)?; self.params.p699 = value; self.mark_param_given(699); self.recompute_instance_static(); Ok(()) }
            "p2udr" => { validate_finite_parameter("p2udr", value)?; self.params.p700 = value; self.mark_param_given(700); self.recompute_instance_static(); Ok(()) }
            "udcv" => { validate_finite_parameter("udcv", value)?; self.params.p701 = value; self.mark_param_given(701); self.recompute_instance_static(); Ok(()) }
            "ludcv" => { validate_finite_parameter("ludcv", value)?; self.params.p702 = value; self.mark_param_given(702); self.recompute_instance_static(); Ok(()) }
            "nudcv" => { validate_finite_parameter("nudcv", value)?; self.params.p703 = value; self.mark_param_given(703); self.recompute_instance_static(); Ok(()) }
            "pudcv" => { validate_finite_parameter("pudcv", value)?; self.params.p704 = value; self.mark_param_given(704); self.recompute_instance_static(); Ok(()) }
            "wudcv" => { validate_finite_parameter("wudcv", value)?; self.params.p705 = value; self.mark_param_given(705); self.recompute_instance_static(); Ok(()) }
            "p2udcv" => { validate_finite_parameter("p2udcv", value)?; self.params.p706 = value; self.mark_param_given(706); self.recompute_instance_static(); Ok(()) }
            "ucs" => { validate_finite_parameter("ucs", value)?; self.params.p707 = value; self.mark_param_given(707); self.recompute_instance_static(); Ok(()) }
            "lucs" => { validate_finite_parameter("lucs", value)?; self.params.p708 = value; self.mark_param_given(708); self.recompute_instance_static(); Ok(()) }
            "nucs" => { validate_finite_parameter("nucs", value)?; self.params.p709 = value; self.mark_param_given(709); self.recompute_instance_static(); Ok(()) }
            "pucs" => { validate_finite_parameter("pucs", value)?; self.params.p710 = value; self.mark_param_given(710); self.recompute_instance_static(); Ok(()) }
            "wucs" => { validate_finite_parameter("wucs", value)?; self.params.p711 = value; self.mark_param_given(711); self.recompute_instance_static(); Ok(()) }
            "p2ucs" => { validate_finite_parameter("p2ucs", value)?; self.params.p712 = value; self.mark_param_given(712); self.recompute_instance_static(); Ok(()) }
            "uds" => { validate_finite_parameter("uds", value)?; self.params.p713 = value; self.mark_param_given(713); self.recompute_instance_static(); Ok(()) }
            "luds" => { validate_finite_parameter("luds", value)?; self.params.p714 = value; self.mark_param_given(714); self.recompute_instance_static(); Ok(()) }
            "nuds" => { validate_finite_parameter("nuds", value)?; self.params.p715 = value; self.mark_param_given(715); self.recompute_instance_static(); Ok(()) }
            "puds" => { validate_finite_parameter("puds", value)?; self.params.p716 = value; self.mark_param_given(716); self.recompute_instance_static(); Ok(()) }
            "wuds" => { validate_finite_parameter("wuds", value)?; self.params.p717 = value; self.mark_param_given(717); self.recompute_instance_static(); Ok(()) }
            "p2uds" => { validate_finite_parameter("p2uds", value)?; self.params.p718 = value; self.mark_param_given(718); self.recompute_instance_static(); Ok(()) }
            "uds1" => { validate_finite_parameter("uds1", value)?; self.params.p719 = value; self.mark_param_given(719); self.recompute_instance_static(); Ok(()) }
            "luds1" => { validate_finite_parameter("luds1", value)?; self.params.p720 = value; self.mark_param_given(720); self.recompute_instance_static(); Ok(()) }
            "nuds1" => { validate_finite_parameter("nuds1", value)?; self.params.p721 = value; self.mark_param_given(721); self.recompute_instance_static(); Ok(()) }
            "puds1" => { validate_finite_parameter("puds1", value)?; self.params.p722 = value; self.mark_param_given(722); self.recompute_instance_static(); Ok(()) }
            "wuds1" => { validate_finite_parameter("wuds1", value)?; self.params.p723 = value; self.mark_param_given(723); self.recompute_instance_static(); Ok(()) }
            "p2uds1" => { validate_finite_parameter("p2uds1", value)?; self.params.p724 = value; self.mark_param_given(724); self.recompute_instance_static(); Ok(()) }
            "udd" => { validate_finite_parameter("udd", value)?; self.params.p725 = value; self.mark_param_given(725); self.recompute_instance_static(); Ok(()) }
            "ludd" => { validate_finite_parameter("ludd", value)?; self.params.p726 = value; self.mark_param_given(726); self.recompute_instance_static(); Ok(()) }
            "nudd" => { validate_finite_parameter("nudd", value)?; self.params.p727 = value; self.mark_param_given(727); self.recompute_instance_static(); Ok(()) }
            "pudd" => { validate_finite_parameter("pudd", value)?; self.params.p728 = value; self.mark_param_given(728); self.recompute_instance_static(); Ok(()) }
            "wudd" => { validate_finite_parameter("wudd", value)?; self.params.p729 = value; self.mark_param_given(729); self.recompute_instance_static(); Ok(()) }
            "p2udd" => { validate_finite_parameter("p2udd", value)?; self.params.p730 = value; self.mark_param_given(730); self.recompute_instance_static(); Ok(()) }
            "udd1" => { validate_finite_parameter("udd1", value)?; self.params.p731 = value; self.mark_param_given(731); self.recompute_instance_static(); Ok(()) }
            "ludd1" => { validate_finite_parameter("ludd1", value)?; self.params.p732 = value; self.mark_param_given(732); self.recompute_instance_static(); Ok(()) }
            "nudd1" => { validate_finite_parameter("nudd1", value)?; self.params.p733 = value; self.mark_param_given(733); self.recompute_instance_static(); Ok(()) }
            "pudd1" => { validate_finite_parameter("pudd1", value)?; self.params.p734 = value; self.mark_param_given(734); self.recompute_instance_static(); Ok(()) }
            "wudd1" => { validate_finite_parameter("wudd1", value)?; self.params.p735 = value; self.mark_param_given(735); self.recompute_instance_static(); Ok(()) }
            "p2udd1" => { validate_finite_parameter("p2udd1", value)?; self.params.p736 = value; self.mark_param_given(736); self.recompute_instance_static(); Ok(()) }
            "ute" => { validate_finite_parameter("ute", value)?; self.params.p737 = value; self.mark_param_given(737); self.recompute_instance_static(); Ok(()) }
            "lute" => { validate_finite_parameter("lute", value)?; self.params.p738 = value; self.mark_param_given(738); self.recompute_instance_static(); Ok(()) }
            "nute" => { validate_finite_parameter("nute", value)?; self.params.p739 = value; self.mark_param_given(739); self.recompute_instance_static(); Ok(()) }
            "pute" => { validate_finite_parameter("pute", value)?; self.params.p740 = value; self.mark_param_given(740); self.recompute_instance_static(); Ok(()) }
            "wute" => { validate_finite_parameter("wute", value)?; self.params.p741 = value; self.mark_param_given(741); self.recompute_instance_static(); Ok(()) }
            "p2ute" => { validate_finite_parameter("p2ute", value)?; self.params.p742 = value; self.mark_param_given(742); self.recompute_instance_static(); Ok(()) }
            "uter" => { validate_finite_parameter("uter", value)?; self.params.p743 = value; self.mark_param_given(743); self.recompute_instance_static(); Ok(()) }
            "luter" => { validate_finite_parameter("luter", value)?; self.params.p744 = value; self.mark_param_given(744); self.recompute_instance_static(); Ok(()) }
            "nuter" => { validate_finite_parameter("nuter", value)?; self.params.p745 = value; self.mark_param_given(745); self.recompute_instance_static(); Ok(()) }
            "puter" => { validate_finite_parameter("puter", value)?; self.params.p746 = value; self.mark_param_given(746); self.recompute_instance_static(); Ok(()) }
            "wuter" => { validate_finite_parameter("wuter", value)?; self.params.p747 = value; self.mark_param_given(747); self.recompute_instance_static(); Ok(()) }
            "p2uter" => { validate_finite_parameter("p2uter", value)?; self.params.p748 = value; self.mark_param_given(748); self.recompute_instance_static(); Ok(()) }
            "utecv" => { validate_finite_parameter("utecv", value)?; self.params.p749 = value; self.mark_param_given(749); self.recompute_instance_static(); Ok(()) }
            "lutecv" => { validate_finite_parameter("lutecv", value)?; self.params.p750 = value; self.mark_param_given(750); self.recompute_instance_static(); Ok(()) }
            "nutecv" => { validate_finite_parameter("nutecv", value)?; self.params.p751 = value; self.mark_param_given(751); self.recompute_instance_static(); Ok(()) }
            "putecv" => { validate_finite_parameter("putecv", value)?; self.params.p752 = value; self.mark_param_given(752); self.recompute_instance_static(); Ok(()) }
            "wutecv" => { validate_finite_parameter("wutecv", value)?; self.params.p753 = value; self.mark_param_given(753); self.recompute_instance_static(); Ok(()) }
            "p2utecv" => { validate_finite_parameter("p2utecv", value)?; self.params.p754 = value; self.mark_param_given(754); self.recompute_instance_static(); Ok(()) }
            "ute1" => { validate_finite_parameter("ute1", value)?; self.params.p755 = value; self.mark_param_given(755); self.recompute_instance_static(); Ok(()) }
            "lute1" => { validate_finite_parameter("lute1", value)?; self.params.p756 = value; self.mark_param_given(756); self.recompute_instance_static(); Ok(()) }
            "nute1" => { validate_finite_parameter("nute1", value)?; self.params.p757 = value; self.mark_param_given(757); self.recompute_instance_static(); Ok(()) }
            "pute1" => { validate_finite_parameter("pute1", value)?; self.params.p758 = value; self.mark_param_given(758); self.recompute_instance_static(); Ok(()) }
            "wute1" => { validate_finite_parameter("wute1", value)?; self.params.p759 = value; self.mark_param_given(759); self.recompute_instance_static(); Ok(()) }
            "p2ute1" => { validate_finite_parameter("p2ute1", value)?; self.params.p760 = value; self.mark_param_given(760); self.recompute_instance_static(); Ok(()) }
            "ute1cv" => { validate_finite_parameter("ute1cv", value)?; self.params.p761 = value; self.mark_param_given(761); self.recompute_instance_static(); Ok(()) }
            "lute1cv" => { validate_finite_parameter("lute1cv", value)?; self.params.p762 = value; self.mark_param_given(762); self.recompute_instance_static(); Ok(()) }
            "nute1cv" => { validate_finite_parameter("nute1cv", value)?; self.params.p763 = value; self.mark_param_given(763); self.recompute_instance_static(); Ok(()) }
            "pute1cv" => { validate_finite_parameter("pute1cv", value)?; self.params.p764 = value; self.mark_param_given(764); self.recompute_instance_static(); Ok(()) }
            "wute1cv" => { validate_finite_parameter("wute1cv", value)?; self.params.p765 = value; self.mark_param_given(765); self.recompute_instance_static(); Ok(()) }
            "p2ute1cv" => { validate_finite_parameter("p2ute1cv", value)?; self.params.p766 = value; self.mark_param_given(766); self.recompute_instance_static(); Ok(()) }
            "utl" => { validate_finite_parameter("utl", value)?; self.params.p767 = value; self.mark_param_given(767); self.recompute_instance_static(); Ok(()) }
            "lutl" => { validate_finite_parameter("lutl", value)?; self.params.p768 = value; self.mark_param_given(768); self.recompute_instance_static(); Ok(()) }
            "nutl" => { validate_finite_parameter("nutl", value)?; self.params.p769 = value; self.mark_param_given(769); self.recompute_instance_static(); Ok(()) }
            "putl" => { validate_finite_parameter("putl", value)?; self.params.p770 = value; self.mark_param_given(770); self.recompute_instance_static(); Ok(()) }
            "wutl" => { validate_finite_parameter("wutl", value)?; self.params.p771 = value; self.mark_param_given(771); self.recompute_instance_static(); Ok(()) }
            "p2utl" => { validate_finite_parameter("p2utl", value)?; self.params.p772 = value; self.mark_param_given(772); self.recompute_instance_static(); Ok(()) }
            "utlr" => { validate_finite_parameter("utlr", value)?; self.params.p773 = value; self.mark_param_given(773); self.recompute_instance_static(); Ok(()) }
            "lutlr" => { validate_finite_parameter("lutlr", value)?; self.params.p774 = value; self.mark_param_given(774); self.recompute_instance_static(); Ok(()) }
            "nutlr" => { validate_finite_parameter("nutlr", value)?; self.params.p775 = value; self.mark_param_given(775); self.recompute_instance_static(); Ok(()) }
            "putlr" => { validate_finite_parameter("putlr", value)?; self.params.p776 = value; self.mark_param_given(776); self.recompute_instance_static(); Ok(()) }
            "wutlr" => { validate_finite_parameter("wutlr", value)?; self.params.p777 = value; self.mark_param_given(777); self.recompute_instance_static(); Ok(()) }
            "p2utlr" => { validate_finite_parameter("p2utlr", value)?; self.params.p778 = value; self.mark_param_given(778); self.recompute_instance_static(); Ok(()) }
            "utlcv" => { validate_finite_parameter("utlcv", value)?; self.params.p779 = value; self.mark_param_given(779); self.recompute_instance_static(); Ok(()) }
            "lutlcv" => { validate_finite_parameter("lutlcv", value)?; self.params.p780 = value; self.mark_param_given(780); self.recompute_instance_static(); Ok(()) }
            "nutlcv" => { validate_finite_parameter("nutlcv", value)?; self.params.p781 = value; self.mark_param_given(781); self.recompute_instance_static(); Ok(()) }
            "putlcv" => { validate_finite_parameter("putlcv", value)?; self.params.p782 = value; self.mark_param_given(782); self.recompute_instance_static(); Ok(()) }
            "wutlcv" => { validate_finite_parameter("wutlcv", value)?; self.params.p783 = value; self.mark_param_given(783); self.recompute_instance_static(); Ok(()) }
            "p2utlcv" => { validate_finite_parameter("p2utlcv", value)?; self.params.p784 = value; self.mark_param_given(784); self.recompute_instance_static(); Ok(()) }
            "emobt" => { validate_finite_parameter("emobt", value)?; self.params.p785 = value; self.mark_param_given(785); self.recompute_instance_static(); Ok(()) }
            "lemobt" => { validate_finite_parameter("lemobt", value)?; self.params.p786 = value; self.mark_param_given(786); self.recompute_instance_static(); Ok(()) }
            "nemobt" => { validate_finite_parameter("nemobt", value)?; self.params.p787 = value; self.mark_param_given(787); self.recompute_instance_static(); Ok(()) }
            "pemobt" => { validate_finite_parameter("pemobt", value)?; self.params.p788 = value; self.mark_param_given(788); self.recompute_instance_static(); Ok(()) }
            "wemobt" => { validate_finite_parameter("wemobt", value)?; self.params.p789 = value; self.mark_param_given(789); self.recompute_instance_static(); Ok(()) }
            "p2emobt" => { validate_finite_parameter("p2emobt", value)?; self.params.p790 = value; self.mark_param_given(790); self.recompute_instance_static(); Ok(()) }
            "ua1" => { validate_finite_parameter("ua1", value)?; self.params.p791 = value; self.mark_param_given(791); self.recompute_instance_static(); Ok(()) }
            "lua1" => { validate_finite_parameter("lua1", value)?; self.params.p792 = value; self.mark_param_given(792); self.recompute_instance_static(); Ok(()) }
            "nua1" => { validate_finite_parameter("nua1", value)?; self.params.p793 = value; self.mark_param_given(793); self.recompute_instance_static(); Ok(()) }
            "pua1" => { validate_finite_parameter("pua1", value)?; self.params.p794 = value; self.mark_param_given(794); self.recompute_instance_static(); Ok(()) }
            "wua1" => { validate_finite_parameter("wua1", value)?; self.params.p795 = value; self.mark_param_given(795); self.recompute_instance_static(); Ok(()) }
            "p2ua1" => { validate_finite_parameter("p2ua1", value)?; self.params.p796 = value; self.mark_param_given(796); self.recompute_instance_static(); Ok(()) }
            "ua1r" => { validate_finite_parameter("ua1r", value)?; self.params.p797 = value; self.mark_param_given(797); self.recompute_instance_static(); Ok(()) }
            "lua1r" => { validate_finite_parameter("lua1r", value)?; self.params.p798 = value; self.mark_param_given(798); self.recompute_instance_static(); Ok(()) }
            "nua1r" => { validate_finite_parameter("nua1r", value)?; self.params.p799 = value; self.mark_param_given(799); self.recompute_instance_static(); Ok(()) }
            "pua1r" => { validate_finite_parameter("pua1r", value)?; self.params.p800 = value; self.mark_param_given(800); self.recompute_instance_static(); Ok(()) }
            "wua1r" => { validate_finite_parameter("wua1r", value)?; self.params.p801 = value; self.mark_param_given(801); self.recompute_instance_static(); Ok(()) }
            "p2ua1r" => { validate_finite_parameter("p2ua1r", value)?; self.params.p802 = value; self.mark_param_given(802); self.recompute_instance_static(); Ok(()) }
            "ua1cv" => { validate_finite_parameter("ua1cv", value)?; self.params.p803 = value; self.mark_param_given(803); self.recompute_instance_static(); Ok(()) }
            "lua1cv" => { validate_finite_parameter("lua1cv", value)?; self.params.p804 = value; self.mark_param_given(804); self.recompute_instance_static(); Ok(()) }
            "nua1cv" => { validate_finite_parameter("nua1cv", value)?; self.params.p805 = value; self.mark_param_given(805); self.recompute_instance_static(); Ok(()) }
            "pua1cv" => { validate_finite_parameter("pua1cv", value)?; self.params.p806 = value; self.mark_param_given(806); self.recompute_instance_static(); Ok(()) }
            "wua1cv" => { validate_finite_parameter("wua1cv", value)?; self.params.p807 = value; self.mark_param_given(807); self.recompute_instance_static(); Ok(()) }
            "p2ua1cv" => { validate_finite_parameter("p2ua1cv", value)?; self.params.p808 = value; self.mark_param_given(808); self.recompute_instance_static(); Ok(()) }
            "ua2" => { validate_finite_parameter("ua2", value)?; self.params.p809 = value; self.mark_param_given(809); self.recompute_instance_static(); Ok(()) }
            "lua2" => { validate_finite_parameter("lua2", value)?; self.params.p810 = value; self.mark_param_given(810); self.recompute_instance_static(); Ok(()) }
            "nua2" => { validate_finite_parameter("nua2", value)?; self.params.p811 = value; self.mark_param_given(811); self.recompute_instance_static(); Ok(()) }
            "pua2" => { validate_finite_parameter("pua2", value)?; self.params.p812 = value; self.mark_param_given(812); self.recompute_instance_static(); Ok(()) }
            "wua2" => { validate_finite_parameter("wua2", value)?; self.params.p813 = value; self.mark_param_given(813); self.recompute_instance_static(); Ok(()) }
            "p2ua2" => { validate_finite_parameter("p2ua2", value)?; self.params.p814 = value; self.mark_param_given(814); self.recompute_instance_static(); Ok(()) }
            "ua2cv" => { validate_finite_parameter("ua2cv", value)?; self.params.p815 = value; self.mark_param_given(815); self.recompute_instance_static(); Ok(()) }
            "lua2cv" => { validate_finite_parameter("lua2cv", value)?; self.params.p816 = value; self.mark_param_given(816); self.recompute_instance_static(); Ok(()) }
            "nua2cv" => { validate_finite_parameter("nua2cv", value)?; self.params.p817 = value; self.mark_param_given(817); self.recompute_instance_static(); Ok(()) }
            "pua2cv" => { validate_finite_parameter("pua2cv", value)?; self.params.p818 = value; self.mark_param_given(818); self.recompute_instance_static(); Ok(()) }
            "wua2cv" => { validate_finite_parameter("wua2cv", value)?; self.params.p819 = value; self.mark_param_given(819); self.recompute_instance_static(); Ok(()) }
            "p2ua2cv" => { validate_finite_parameter("p2ua2cv", value)?; self.params.p820 = value; self.mark_param_given(820); self.recompute_instance_static(); Ok(()) }
            "eu1" => { validate_finite_parameter("eu1", value)?; self.params.p821 = value; self.mark_param_given(821); self.recompute_instance_static(); Ok(()) }
            "leu1" => { validate_finite_parameter("leu1", value)?; self.params.p822 = value; self.mark_param_given(822); self.recompute_instance_static(); Ok(()) }
            "neu1" => { validate_finite_parameter("neu1", value)?; self.params.p823 = value; self.mark_param_given(823); self.recompute_instance_static(); Ok(()) }
            "peu1" => { validate_finite_parameter("peu1", value)?; self.params.p824 = value; self.mark_param_given(824); self.recompute_instance_static(); Ok(()) }
            "weu1" => { validate_finite_parameter("weu1", value)?; self.params.p825 = value; self.mark_param_given(825); self.recompute_instance_static(); Ok(()) }
            "p2eu1" => { validate_finite_parameter("p2eu1", value)?; self.params.p826 = value; self.mark_param_given(826); self.recompute_instance_static(); Ok(()) }
            "uc1" => { validate_finite_parameter("uc1", value)?; self.params.p827 = value; self.mark_param_given(827); self.recompute_instance_static(); Ok(()) }
            "luc1" => { validate_finite_parameter("luc1", value)?; self.params.p828 = value; self.mark_param_given(828); self.recompute_instance_static(); Ok(()) }
            "nuc1" => { validate_finite_parameter("nuc1", value)?; self.params.p829 = value; self.mark_param_given(829); self.recompute_instance_static(); Ok(()) }
            "puc1" => { validate_finite_parameter("puc1", value)?; self.params.p830 = value; self.mark_param_given(830); self.recompute_instance_static(); Ok(()) }
            "wuc1" => { validate_finite_parameter("wuc1", value)?; self.params.p831 = value; self.mark_param_given(831); self.recompute_instance_static(); Ok(()) }
            "p2uc1" => { validate_finite_parameter("p2uc1", value)?; self.params.p832 = value; self.mark_param_given(832); self.recompute_instance_static(); Ok(()) }
            "uc1r" => { validate_finite_parameter("uc1r", value)?; self.params.p833 = value; self.mark_param_given(833); self.recompute_instance_static(); Ok(()) }
            "luc1r" => { validate_finite_parameter("luc1r", value)?; self.params.p834 = value; self.mark_param_given(834); self.recompute_instance_static(); Ok(()) }
            "nuc1r" => { validate_finite_parameter("nuc1r", value)?; self.params.p835 = value; self.mark_param_given(835); self.recompute_instance_static(); Ok(()) }
            "puc1r" => { validate_finite_parameter("puc1r", value)?; self.params.p836 = value; self.mark_param_given(836); self.recompute_instance_static(); Ok(()) }
            "wuc1r" => { validate_finite_parameter("wuc1r", value)?; self.params.p837 = value; self.mark_param_given(837); self.recompute_instance_static(); Ok(()) }
            "p2uc1r" => { validate_finite_parameter("p2uc1r", value)?; self.params.p838 = value; self.mark_param_given(838); self.recompute_instance_static(); Ok(()) }
            "uc1cv" => { validate_finite_parameter("uc1cv", value)?; self.params.p839 = value; self.mark_param_given(839); self.recompute_instance_static(); Ok(()) }
            "luc1cv" => { validate_finite_parameter("luc1cv", value)?; self.params.p840 = value; self.mark_param_given(840); self.recompute_instance_static(); Ok(()) }
            "nuc1cv" => { validate_finite_parameter("nuc1cv", value)?; self.params.p841 = value; self.mark_param_given(841); self.recompute_instance_static(); Ok(()) }
            "puc1cv" => { validate_finite_parameter("puc1cv", value)?; self.params.p842 = value; self.mark_param_given(842); self.recompute_instance_static(); Ok(()) }
            "wuc1cv" => { validate_finite_parameter("wuc1cv", value)?; self.params.p843 = value; self.mark_param_given(843); self.recompute_instance_static(); Ok(()) }
            "p2uc1cv" => { validate_finite_parameter("p2uc1cv", value)?; self.params.p844 = value; self.mark_param_given(844); self.recompute_instance_static(); Ok(()) }
            "ud1" => { validate_finite_parameter("ud1", value)?; self.params.p845 = value; self.mark_param_given(845); self.recompute_instance_static(); Ok(()) }
            "lud1" => { validate_finite_parameter("lud1", value)?; self.params.p846 = value; self.mark_param_given(846); self.recompute_instance_static(); Ok(()) }
            "nud1" => { validate_finite_parameter("nud1", value)?; self.params.p847 = value; self.mark_param_given(847); self.recompute_instance_static(); Ok(()) }
            "pud1" => { validate_finite_parameter("pud1", value)?; self.params.p848 = value; self.mark_param_given(848); self.recompute_instance_static(); Ok(()) }
            "wud1" => { validate_finite_parameter("wud1", value)?; self.params.p849 = value; self.mark_param_given(849); self.recompute_instance_static(); Ok(()) }
            "p2ud1" => { validate_finite_parameter("p2ud1", value)?; self.params.p850 = value; self.mark_param_given(850); self.recompute_instance_static(); Ok(()) }
            "ud1r" => { validate_finite_parameter("ud1r", value)?; self.params.p851 = value; self.mark_param_given(851); self.recompute_instance_static(); Ok(()) }
            "lud1r" => { validate_finite_parameter("lud1r", value)?; self.params.p852 = value; self.mark_param_given(852); self.recompute_instance_static(); Ok(()) }
            "nud1r" => { validate_finite_parameter("nud1r", value)?; self.params.p853 = value; self.mark_param_given(853); self.recompute_instance_static(); Ok(()) }
            "pud1r" => { validate_finite_parameter("pud1r", value)?; self.params.p854 = value; self.mark_param_given(854); self.recompute_instance_static(); Ok(()) }
            "wud1r" => { validate_finite_parameter("wud1r", value)?; self.params.p855 = value; self.mark_param_given(855); self.recompute_instance_static(); Ok(()) }
            "p2ud1r" => { validate_finite_parameter("p2ud1r", value)?; self.params.p856 = value; self.mark_param_given(856); self.recompute_instance_static(); Ok(()) }
            "ud1cv" => { validate_finite_parameter("ud1cv", value)?; self.params.p857 = value; self.mark_param_given(857); self.recompute_instance_static(); Ok(()) }
            "lud1cv" => { validate_finite_parameter("lud1cv", value)?; self.params.p858 = value; self.mark_param_given(858); self.recompute_instance_static(); Ok(()) }
            "nud1cv" => { validate_finite_parameter("nud1cv", value)?; self.params.p859 = value; self.mark_param_given(859); self.recompute_instance_static(); Ok(()) }
            "pud1cv" => { validate_finite_parameter("pud1cv", value)?; self.params.p860 = value; self.mark_param_given(860); self.recompute_instance_static(); Ok(()) }
            "wud1cv" => { validate_finite_parameter("wud1cv", value)?; self.params.p861 = value; self.mark_param_given(861); self.recompute_instance_static(); Ok(()) }
            "p2ud1cv" => { validate_finite_parameter("p2ud1cv", value)?; self.params.p862 = value; self.mark_param_given(862); self.recompute_instance_static(); Ok(()) }
            "ud2" => { validate_finite_parameter("ud2", value)?; self.params.p863 = value; self.mark_param_given(863); self.recompute_instance_static(); Ok(()) }
            "lud2" => { validate_finite_parameter("lud2", value)?; self.params.p864 = value; self.mark_param_given(864); self.recompute_instance_static(); Ok(()) }
            "nud2" => { validate_finite_parameter("nud2", value)?; self.params.p865 = value; self.mark_param_given(865); self.recompute_instance_static(); Ok(()) }
            "pud2" => { validate_finite_parameter("pud2", value)?; self.params.p866 = value; self.mark_param_given(866); self.recompute_instance_static(); Ok(()) }
            "wud2" => { validate_finite_parameter("wud2", value)?; self.params.p867 = value; self.mark_param_given(867); self.recompute_instance_static(); Ok(()) }
            "p2ud2" => { validate_finite_parameter("p2ud2", value)?; self.params.p868 = value; self.mark_param_given(868); self.recompute_instance_static(); Ok(()) }
            "ud2cv" => { validate_finite_parameter("ud2cv", value)?; self.params.p869 = value; self.mark_param_given(869); self.recompute_instance_static(); Ok(()) }
            "lud2cv" => { validate_finite_parameter("lud2cv", value)?; self.params.p870 = value; self.mark_param_given(870); self.recompute_instance_static(); Ok(()) }
            "nud2cv" => { validate_finite_parameter("nud2cv", value)?; self.params.p871 = value; self.mark_param_given(871); self.recompute_instance_static(); Ok(()) }
            "pud2cv" => { validate_finite_parameter("pud2cv", value)?; self.params.p872 = value; self.mark_param_given(872); self.recompute_instance_static(); Ok(()) }
            "wud2cv" => { validate_finite_parameter("wud2cv", value)?; self.params.p873 = value; self.mark_param_given(873); self.recompute_instance_static(); Ok(()) }
            "p2ud2cv" => { validate_finite_parameter("p2ud2cv", value)?; self.params.p874 = value; self.mark_param_given(874); self.recompute_instance_static(); Ok(()) }
            "ucste" => { validate_finite_parameter("ucste", value)?; self.params.p875 = value; self.mark_param_given(875); self.recompute_instance_static(); Ok(()) }
            "lucste" => { validate_finite_parameter("lucste", value)?; self.params.p876 = value; self.mark_param_given(876); self.recompute_instance_static(); Ok(()) }
            "nucste" => { validate_finite_parameter("nucste", value)?; self.params.p877 = value; self.mark_param_given(877); self.recompute_instance_static(); Ok(()) }
            "pucste" => { validate_finite_parameter("pucste", value)?; self.params.p878 = value; self.mark_param_given(878); self.recompute_instance_static(); Ok(()) }
            "wucste" => { validate_finite_parameter("wucste", value)?; self.params.p879 = value; self.mark_param_given(879); self.recompute_instance_static(); Ok(()) }
            "p2ucste" => { validate_finite_parameter("p2ucste", value)?; self.params.p880 = value; self.mark_param_given(880); self.recompute_instance_static(); Ok(()) }
            "ucste1" => { validate_finite_parameter("ucste1", value)?; self.params.p881 = value; self.mark_param_given(881); self.recompute_instance_static(); Ok(()) }
            "lucste1" => { validate_finite_parameter("lucste1", value)?; self.params.p882 = value; self.mark_param_given(882); self.recompute_instance_static(); Ok(()) }
            "nucste1" => { validate_finite_parameter("nucste1", value)?; self.params.p883 = value; self.mark_param_given(883); self.recompute_instance_static(); Ok(()) }
            "pucste1" => { validate_finite_parameter("pucste1", value)?; self.params.p884 = value; self.mark_param_given(884); self.recompute_instance_static(); Ok(()) }
            "wucste1" => { validate_finite_parameter("wucste1", value)?; self.params.p885 = value; self.mark_param_given(885); self.recompute_instance_static(); Ok(()) }
            "p2ucste1" => { validate_finite_parameter("p2ucste1", value)?; self.params.p886 = value; self.mark_param_given(886); self.recompute_instance_static(); Ok(()) }
            "muhc0" => { validate_parameter("muhc0", value, None, true, Some((1.0, "1.0")), true, &[])?; self.params.p887 = value; self.mark_param_given(887); self.recompute_instance_static(); Ok(()) }
            "muhc1" => { validate_parameter("muhc1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p888 = value; self.mark_param_given(888); self.recompute_instance_static(); Ok(()) }
            "etamobthin" => { validate_finite_parameter("etamobthin", value)?; self.params.p889 = value; self.mark_param_given(889); self.recompute_instance_static(); Ok(()) }
            "etamobtni" => { validate_parameter("etamobtni", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p890 = value; self.mark_param_given(890); self.recompute_instance_static(); Ok(()) }
            "etamobir" => { validate_parameter("etamobir", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p891 = value; self.mark_param_given(891); self.recompute_instance_static(); Ok(()) }
            "uathin" => { validate_finite_parameter("uathin", value)?; self.params.p892 = value; self.mark_param_given(892); self.recompute_instance_static(); Ok(()) }
            "uatsat" => { validate_parameter("uatsat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p893 = value; self.mark_param_given(893); self.recompute_instance_static(); Ok(()) }
            "uartsc" => { validate_parameter("uartsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p894 = value; self.mark_param_given(894); self.recompute_instance_static(); Ok(()) }
            "uatni" => { validate_parameter("uatni", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p895 = value; self.mark_param_given(895); self.recompute_instance_static(); Ok(()) }
            "uair" => { validate_parameter("uair", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p896 = value; self.mark_param_given(896); self.recompute_instance_static(); Ok(()) }
            "euthin" => { validate_finite_parameter("euthin", value)?; self.params.p897 = value; self.mark_param_given(897); self.recompute_instance_static(); Ok(()) }
            "euptsc" => { validate_parameter("euptsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p898 = value; self.mark_param_given(898); self.recompute_instance_static(); Ok(()) }
            "eutni" => { validate_parameter("eutni", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p899 = value; self.mark_param_given(899); self.recompute_instance_static(); Ok(()) }
            "euir" => { validate_parameter("euir", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p900 = value; self.mark_param_given(900); self.recompute_instance_static(); Ok(()) }
            "udthin" => { validate_finite_parameter("udthin", value)?; self.params.p901 = value; self.mark_param_given(901); self.recompute_instance_static(); Ok(()) }
            "udtsat" => { validate_parameter("udtsat", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p902 = value; self.mark_param_given(902); self.recompute_instance_static(); Ok(()) }
            "udptsc" => { validate_parameter("udptsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p903 = value; self.mark_param_given(903); self.recompute_instance_static(); Ok(()) }
            "u0etawsc" => { validate_parameter("u0etawsc", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p904 = value; self.mark_param_given(904); self.recompute_instance_static(); Ok(()) }
            "egbulk" => { validate_parameter("egbulk", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p905 = value; self.mark_param_given(905); self.recompute_instance_static(); Ok(()) }
            "u0emsm1" => { validate_parameter("u0emsm1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p906 = value; self.mark_param_given(906); self.recompute_instance_static(); Ok(()) }
            "u0emsm2" => { validate_finite_parameter("u0emsm2", value)?; self.params.p907 = value; self.mark_param_given(907); self.recompute_instance_static(); Ok(()) }
            "rdswmin" => { validate_parameter("rdswmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p908 = value; self.mark_param_given(908); self.recompute_instance_static(); Ok(()) }
            "ardsw" => { validate_finite_parameter("ardsw", value)?; self.params.p909 = value; self.mark_param_given(909); self.recompute_instance_static(); Ok(()) }
            "brdsw" => { validate_finite_parameter("brdsw", value)?; self.params.p910 = value; self.mark_param_given(910); self.recompute_instance_static(); Ok(()) }
            "rswmin" => { validate_parameter("rswmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p911 = value; self.mark_param_given(911); self.recompute_instance_static(); Ok(()) }
            "arsw" => { validate_finite_parameter("arsw", value)?; self.params.p912 = value; self.mark_param_given(912); self.recompute_instance_static(); Ok(()) }
            "brsw" => { validate_finite_parameter("brsw", value)?; self.params.p913 = value; self.mark_param_given(913); self.recompute_instance_static(); Ok(()) }
            "rdwmin" => { validate_parameter("rdwmin", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p914 = value; self.mark_param_given(914); self.recompute_instance_static(); Ok(()) }
            "ardw" => { validate_finite_parameter("ardw", value)?; self.params.p915 = value; self.mark_param_given(915); self.recompute_instance_static(); Ok(()) }
            "brdw" => { validate_finite_parameter("brdw", value)?; self.params.p916 = value; self.mark_param_given(916); self.recompute_instance_static(); Ok(()) }
            "rsdr" => { validate_parameter("rsdr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p917 = value; self.mark_param_given(917); self.recompute_instance_static(); Ok(()) }
            "rsdrr" => { validate_parameter("rsdrr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p918 = value; self.mark_param_given(918); self.recompute_instance_static(); Ok(()) }
            "rddr" => { validate_parameter("rddr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p919 = value; self.mark_param_given(919); self.recompute_instance_static(); Ok(()) }
            "rddrr" => { validate_parameter("rddrr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p920 = value; self.mark_param_given(920); self.recompute_instance_static(); Ok(()) }
            "prsdr" => { validate_finite_parameter("prsdr", value)?; self.params.p921 = value; self.mark_param_given(921); self.recompute_instance_static(); Ok(()) }
            "prddr" => { validate_finite_parameter("prddr", value)?; self.params.p922 = value; self.mark_param_given(922); self.recompute_instance_static(); Ok(()) }
            "trsdr" => { validate_finite_parameter("trsdr", value)?; self.params.p923 = value; self.mark_param_given(923); self.recompute_instance_static(); Ok(()) }
            "trddr" => { validate_finite_parameter("trddr", value)?; self.params.p924 = value; self.mark_param_given(924); self.recompute_instance_static(); Ok(()) }
            "rdsw" => { validate_finite_parameter("rdsw", value)?; self.params.p925 = value; self.mark_param_given(925); self.recompute_instance_static(); Ok(()) }
            "lrdsw" => { validate_finite_parameter("lrdsw", value)?; self.params.p926 = value; self.mark_param_given(926); self.recompute_instance_static(); Ok(()) }
            "nrdsw" => { validate_finite_parameter("nrdsw", value)?; self.params.p927 = value; self.mark_param_given(927); self.recompute_instance_static(); Ok(()) }
            "prdsw" => { validate_finite_parameter("prdsw", value)?; self.params.p928 = value; self.mark_param_given(928); self.recompute_instance_static(); Ok(()) }
            "wrdsw" => { validate_finite_parameter("wrdsw", value)?; self.params.p929 = value; self.mark_param_given(929); self.recompute_instance_static(); Ok(()) }
            "p2rdsw" => { validate_finite_parameter("p2rdsw", value)?; self.params.p930 = value; self.mark_param_given(930); self.recompute_instance_static(); Ok(()) }
            "rsw" => { validate_finite_parameter("rsw", value)?; self.params.p931 = value; self.mark_param_given(931); self.recompute_instance_static(); Ok(()) }
            "lrsw" => { validate_finite_parameter("lrsw", value)?; self.params.p932 = value; self.mark_param_given(932); self.recompute_instance_static(); Ok(()) }
            "nrsw" => { validate_finite_parameter("nrsw", value)?; self.params.p933 = value; self.mark_param_given(933); self.recompute_instance_static(); Ok(()) }
            "prsw" => { validate_finite_parameter("prsw", value)?; self.params.p934 = value; self.mark_param_given(934); self.recompute_instance_static(); Ok(()) }
            "wrsw" => { validate_finite_parameter("wrsw", value)?; self.params.p935 = value; self.mark_param_given(935); self.recompute_instance_static(); Ok(()) }
            "p2rsw" => { validate_finite_parameter("p2rsw", value)?; self.params.p936 = value; self.mark_param_given(936); self.recompute_instance_static(); Ok(()) }
            "rdw" => { validate_finite_parameter("rdw", value)?; self.params.p937 = value; self.mark_param_given(937); self.recompute_instance_static(); Ok(()) }
            "lrdw" => { validate_finite_parameter("lrdw", value)?; self.params.p938 = value; self.mark_param_given(938); self.recompute_instance_static(); Ok(()) }
            "nrdw" => { validate_finite_parameter("nrdw", value)?; self.params.p939 = value; self.mark_param_given(939); self.recompute_instance_static(); Ok(()) }
            "prdw" => { validate_finite_parameter("prdw", value)?; self.params.p940 = value; self.mark_param_given(940); self.recompute_instance_static(); Ok(()) }
            "wrdw" => { validate_finite_parameter("wrdw", value)?; self.params.p941 = value; self.mark_param_given(941); self.recompute_instance_static(); Ok(()) }
            "p2rdw" => { validate_finite_parameter("p2rdw", value)?; self.params.p942 = value; self.mark_param_given(942); self.recompute_instance_static(); Ok(()) }
            "prwgs" => { validate_finite_parameter("prwgs", value)?; self.params.p943 = value; self.mark_param_given(943); self.recompute_instance_static(); Ok(()) }
            "lprwgs" => { validate_finite_parameter("lprwgs", value)?; self.params.p944 = value; self.mark_param_given(944); self.recompute_instance_static(); Ok(()) }
            "nprwgs" => { validate_finite_parameter("nprwgs", value)?; self.params.p945 = value; self.mark_param_given(945); self.recompute_instance_static(); Ok(()) }
            "pprwgs" => { validate_finite_parameter("pprwgs", value)?; self.params.p946 = value; self.mark_param_given(946); self.recompute_instance_static(); Ok(()) }
            "wprwgs" => { validate_finite_parameter("wprwgs", value)?; self.params.p947 = value; self.mark_param_given(947); self.recompute_instance_static(); Ok(()) }
            "p2prwgs" => { validate_finite_parameter("p2prwgs", value)?; self.params.p948 = value; self.mark_param_given(948); self.recompute_instance_static(); Ok(()) }
            "prwgd" => { validate_finite_parameter("prwgd", value)?; self.params.p949 = value; self.mark_param_given(949); self.recompute_instance_static(); Ok(()) }
            "lprwgd" => { validate_finite_parameter("lprwgd", value)?; self.params.p950 = value; self.mark_param_given(950); self.recompute_instance_static(); Ok(()) }
            "nprwgd" => { validate_finite_parameter("nprwgd", value)?; self.params.p951 = value; self.mark_param_given(951); self.recompute_instance_static(); Ok(()) }
            "pprwgd" => { validate_finite_parameter("pprwgd", value)?; self.params.p952 = value; self.mark_param_given(952); self.recompute_instance_static(); Ok(()) }
            "wprwgd" => { validate_finite_parameter("wprwgd", value)?; self.params.p953 = value; self.mark_param_given(953); self.recompute_instance_static(); Ok(()) }
            "p2prwgd" => { validate_finite_parameter("p2prwgd", value)?; self.params.p954 = value; self.mark_param_given(954); self.recompute_instance_static(); Ok(()) }
            "wr" => { validate_finite_parameter("wr", value)?; self.params.p955 = value; self.mark_param_given(955); self.recompute_instance_static(); Ok(()) }
            "lwr" => { validate_finite_parameter("lwr", value)?; self.params.p956 = value; self.mark_param_given(956); self.recompute_instance_static(); Ok(()) }
            "nwr" => { validate_finite_parameter("nwr", value)?; self.params.p957 = value; self.mark_param_given(957); self.recompute_instance_static(); Ok(()) }
            "pwr" => { validate_finite_parameter("pwr", value)?; self.params.p958 = value; self.mark_param_given(958); self.recompute_instance_static(); Ok(()) }
            "wwr" => { validate_finite_parameter("wwr", value)?; self.params.p959 = value; self.mark_param_given(959); self.recompute_instance_static(); Ok(()) }
            "p2wr" => { validate_finite_parameter("p2wr", value)?; self.params.p960 = value; self.mark_param_given(960); self.recompute_instance_static(); Ok(()) }
            "prt" => { validate_finite_parameter("prt", value)?; self.params.p961 = value; self.mark_param_given(961); self.recompute_instance_static(); Ok(()) }
            "lprt" => { validate_finite_parameter("lprt", value)?; self.params.p962 = value; self.mark_param_given(962); self.recompute_instance_static(); Ok(()) }
            "nprt" => { validate_finite_parameter("nprt", value)?; self.params.p963 = value; self.mark_param_given(963); self.recompute_instance_static(); Ok(()) }
            "pprt" => { validate_finite_parameter("pprt", value)?; self.params.p964 = value; self.mark_param_given(964); self.recompute_instance_static(); Ok(()) }
            "wprt" => { validate_finite_parameter("wprt", value)?; self.params.p965 = value; self.mark_param_given(965); self.recompute_instance_static(); Ok(()) }
            "p2prt" => { validate_finite_parameter("p2prt", value)?; self.params.p966 = value; self.mark_param_given(966); self.recompute_instance_static(); Ok(()) }
            "prt1" => { validate_finite_parameter("prt1", value)?; self.params.p967 = value; self.mark_param_given(967); self.recompute_instance_static(); Ok(()) }
            "lprt1" => { validate_finite_parameter("lprt1", value)?; self.params.p968 = value; self.mark_param_given(968); self.recompute_instance_static(); Ok(()) }
            "nprt1" => { validate_finite_parameter("nprt1", value)?; self.params.p969 = value; self.mark_param_given(969); self.recompute_instance_static(); Ok(()) }
            "pprt1" => { validate_finite_parameter("pprt1", value)?; self.params.p970 = value; self.mark_param_given(970); self.recompute_instance_static(); Ok(()) }
            "wprt1" => { validate_finite_parameter("wprt1", value)?; self.params.p971 = value; self.mark_param_given(971); self.recompute_instance_static(); Ok(()) }
            "p2prt1" => { validate_finite_parameter("p2prt1", value)?; self.params.p972 = value; self.mark_param_given(972); self.recompute_instance_static(); Ok(()) }
            "tr0" => { validate_finite_parameter("tr0", value)?; self.params.p973 = value; self.mark_param_given(973); self.recompute_instance_static(); Ok(()) }
            "ltr0" => { validate_finite_parameter("ltr0", value)?; self.params.p974 = value; self.mark_param_given(974); self.recompute_instance_static(); Ok(()) }
            "ntr0" => { validate_finite_parameter("ntr0", value)?; self.params.p975 = value; self.mark_param_given(975); self.recompute_instance_static(); Ok(()) }
            "ptr0" => { validate_finite_parameter("ptr0", value)?; self.params.p976 = value; self.mark_param_given(976); self.recompute_instance_static(); Ok(()) }
            "wtr0" => { validate_finite_parameter("wtr0", value)?; self.params.p977 = value; self.mark_param_given(977); self.recompute_instance_static(); Ok(()) }
            "p2tr0" => { validate_finite_parameter("p2tr0", value)?; self.params.p978 = value; self.mark_param_given(978); self.recompute_instance_static(); Ok(()) }
            "sprt" => { validate_finite_parameter("sprt", value)?; self.params.p979 = value; self.mark_param_given(979); self.recompute_instance_static(); Ok(()) }
            "lsprt" => { validate_finite_parameter("lsprt", value)?; self.params.p980 = value; self.mark_param_given(980); self.recompute_instance_static(); Ok(()) }
            "nsprt" => { validate_finite_parameter("nsprt", value)?; self.params.p981 = value; self.mark_param_given(981); self.recompute_instance_static(); Ok(()) }
            "psprt" => { validate_finite_parameter("psprt", value)?; self.params.p982 = value; self.mark_param_given(982); self.recompute_instance_static(); Ok(()) }
            "wsprt" => { validate_finite_parameter("wsprt", value)?; self.params.p983 = value; self.mark_param_given(983); self.recompute_instance_static(); Ok(()) }
            "p2sprt" => { validate_finite_parameter("p2sprt", value)?; self.params.p984 = value; self.mark_param_given(984); self.recompute_instance_static(); Ok(()) }
            "pdibl1" => { validate_finite_parameter("pdibl1", value)?; self.params.p985 = value; self.mark_param_given(985); self.recompute_instance_static(); Ok(()) }
            "lpdibl1" => { validate_finite_parameter("lpdibl1", value)?; self.params.p986 = value; self.mark_param_given(986); self.recompute_instance_static(); Ok(()) }
            "npdibl1" => { validate_finite_parameter("npdibl1", value)?; self.params.p987 = value; self.mark_param_given(987); self.recompute_instance_static(); Ok(()) }
            "ppdibl1" => { validate_finite_parameter("ppdibl1", value)?; self.params.p988 = value; self.mark_param_given(988); self.recompute_instance_static(); Ok(()) }
            "wpdibl1" => { validate_finite_parameter("wpdibl1", value)?; self.params.p989 = value; self.mark_param_given(989); self.recompute_instance_static(); Ok(()) }
            "p2pdibl1" => { validate_finite_parameter("p2pdibl1", value)?; self.params.p990 = value; self.mark_param_given(990); self.recompute_instance_static(); Ok(()) }
            "pdibl2" => { validate_finite_parameter("pdibl2", value)?; self.params.p991 = value; self.mark_param_given(991); self.recompute_instance_static(); Ok(()) }
            "lpdibl2" => { validate_finite_parameter("lpdibl2", value)?; self.params.p992 = value; self.mark_param_given(992); self.recompute_instance_static(); Ok(()) }
            "npdibl2" => { validate_finite_parameter("npdibl2", value)?; self.params.p993 = value; self.mark_param_given(993); self.recompute_instance_static(); Ok(()) }
            "ppdibl2" => { validate_finite_parameter("ppdibl2", value)?; self.params.p994 = value; self.mark_param_given(994); self.recompute_instance_static(); Ok(()) }
            "wpdibl2" => { validate_finite_parameter("wpdibl2", value)?; self.params.p995 = value; self.mark_param_given(995); self.recompute_instance_static(); Ok(()) }
            "p2pdibl2" => { validate_finite_parameter("p2pdibl2", value)?; self.params.p996 = value; self.mark_param_given(996); self.recompute_instance_static(); Ok(()) }
            "pdibl1r" => { validate_finite_parameter("pdibl1r", value)?; self.params.p997 = value; self.mark_param_given(997); self.recompute_instance_static(); Ok(()) }
            "lpdibl1r" => { validate_finite_parameter("lpdibl1r", value)?; self.params.p998 = value; self.mark_param_given(998); self.recompute_instance_static(); Ok(()) }
            "npdibl1r" => { validate_finite_parameter("npdibl1r", value)?; self.params.p999 = value; self.mark_param_given(999); self.recompute_instance_static(); Ok(()) }
            "ppdibl1r" => { validate_finite_parameter("ppdibl1r", value)?; self.params.p1000 = value; self.mark_param_given(1000); self.recompute_instance_static(); Ok(()) }
            "wpdibl1r" => { validate_finite_parameter("wpdibl1r", value)?; self.params.p1001 = value; self.mark_param_given(1001); self.recompute_instance_static(); Ok(()) }
            "p2pdibl1r" => { validate_finite_parameter("p2pdibl1r", value)?; self.params.p1002 = value; self.mark_param_given(1002); self.recompute_instance_static(); Ok(()) }
            "pdibl2r" => { validate_finite_parameter("pdibl2r", value)?; self.params.p1003 = value; self.mark_param_given(1003); self.recompute_instance_static(); Ok(()) }
            "lpdibl2r" => { validate_finite_parameter("lpdibl2r", value)?; self.params.p1004 = value; self.mark_param_given(1004); self.recompute_instance_static(); Ok(()) }
            "npdibl2r" => { validate_finite_parameter("npdibl2r", value)?; self.params.p1005 = value; self.mark_param_given(1005); self.recompute_instance_static(); Ok(()) }
            "ppdibl2r" => { validate_finite_parameter("ppdibl2r", value)?; self.params.p1006 = value; self.mark_param_given(1006); self.recompute_instance_static(); Ok(()) }
            "wpdibl2r" => { validate_finite_parameter("wpdibl2r", value)?; self.params.p1007 = value; self.mark_param_given(1007); self.recompute_instance_static(); Ok(()) }
            "p2pdibl2r" => { validate_finite_parameter("p2pdibl2r", value)?; self.params.p1008 = value; self.mark_param_given(1008); self.recompute_instance_static(); Ok(()) }
            "drout" => { validate_finite_parameter("drout", value)?; self.params.p1009 = value; self.mark_param_given(1009); self.recompute_instance_static(); Ok(()) }
            "ldrout" => { validate_finite_parameter("ldrout", value)?; self.params.p1010 = value; self.mark_param_given(1010); self.recompute_instance_static(); Ok(()) }
            "ndrout" => { validate_finite_parameter("ndrout", value)?; self.params.p1011 = value; self.mark_param_given(1011); self.recompute_instance_static(); Ok(()) }
            "pdrout" => { validate_finite_parameter("pdrout", value)?; self.params.p1012 = value; self.mark_param_given(1012); self.recompute_instance_static(); Ok(()) }
            "wdrout" => { validate_finite_parameter("wdrout", value)?; self.params.p1013 = value; self.mark_param_given(1013); self.recompute_instance_static(); Ok(()) }
            "p2drout" => { validate_finite_parameter("p2drout", value)?; self.params.p1014 = value; self.mark_param_given(1014); self.recompute_instance_static(); Ok(()) }
            "pvag" => { validate_finite_parameter("pvag", value)?; self.params.p1015 = value; self.mark_param_given(1015); self.recompute_instance_static(); Ok(()) }
            "lpvag" => { validate_finite_parameter("lpvag", value)?; self.params.p1016 = value; self.mark_param_given(1016); self.recompute_instance_static(); Ok(()) }
            "npvag" => { validate_finite_parameter("npvag", value)?; self.params.p1017 = value; self.mark_param_given(1017); self.recompute_instance_static(); Ok(()) }
            "ppvag" => { validate_finite_parameter("ppvag", value)?; self.params.p1018 = value; self.mark_param_given(1018); self.recompute_instance_static(); Ok(()) }
            "wpvag" => { validate_finite_parameter("wpvag", value)?; self.params.p1019 = value; self.mark_param_given(1019); self.recompute_instance_static(); Ok(()) }
            "p2pvag" => { validate_finite_parameter("p2pvag", value)?; self.params.p1020 = value; self.mark_param_given(1020); self.recompute_instance_static(); Ok(()) }
            "apclm" => { validate_finite_parameter("apclm", value)?; self.params.p1021 = value; self.mark_param_given(1021); self.recompute_instance_static(); Ok(()) }
            "apclmr" => { validate_finite_parameter("apclmr", value)?; self.params.p1022 = value; self.mark_param_given(1022); self.recompute_instance_static(); Ok(()) }
            "bpclm" => { validate_finite_parameter("bpclm", value)?; self.params.p1023 = value; self.mark_param_given(1023); self.recompute_instance_static(); Ok(()) }
            "bpclmr" => { validate_finite_parameter("bpclmr", value)?; self.params.p1024 = value; self.mark_param_given(1024); self.recompute_instance_static(); Ok(()) }
            "pclm" => { validate_finite_parameter("pclm", value)?; self.params.p1025 = value; self.mark_param_given(1025); self.recompute_instance_static(); Ok(()) }
            "pclmt" => { validate_finite_parameter("pclmt", value)?; self.params.p1026 = value; self.mark_param_given(1026); self.recompute_instance_static(); Ok(()) }
            "lpclm" => { validate_finite_parameter("lpclm", value)?; self.params.p1027 = value; self.mark_param_given(1027); self.recompute_instance_static(); Ok(()) }
            "npclm" => { validate_finite_parameter("npclm", value)?; self.params.p1028 = value; self.mark_param_given(1028); self.recompute_instance_static(); Ok(()) }
            "ppclm" => { validate_finite_parameter("ppclm", value)?; self.params.p1029 = value; self.mark_param_given(1029); self.recompute_instance_static(); Ok(()) }
            "wpclm" => { validate_finite_parameter("wpclm", value)?; self.params.p1030 = value; self.mark_param_given(1030); self.recompute_instance_static(); Ok(()) }
            "p2pclm" => { validate_finite_parameter("p2pclm", value)?; self.params.p1031 = value; self.mark_param_given(1031); self.recompute_instance_static(); Ok(()) }
            "pclmr" => { validate_finite_parameter("pclmr", value)?; self.params.p1032 = value; self.mark_param_given(1032); self.recompute_instance_static(); Ok(()) }
            "lpclmr" => { validate_finite_parameter("lpclmr", value)?; self.params.p1033 = value; self.mark_param_given(1033); self.recompute_instance_static(); Ok(()) }
            "npclmr" => { validate_finite_parameter("npclmr", value)?; self.params.p1034 = value; self.mark_param_given(1034); self.recompute_instance_static(); Ok(()) }
            "ppclmr" => { validate_finite_parameter("ppclmr", value)?; self.params.p1035 = value; self.mark_param_given(1035); self.recompute_instance_static(); Ok(()) }
            "wpclmr" => { validate_finite_parameter("wpclmr", value)?; self.params.p1036 = value; self.mark_param_given(1036); self.recompute_instance_static(); Ok(()) }
            "p2pclmr" => { validate_finite_parameter("p2pclmr", value)?; self.params.p1037 = value; self.mark_param_given(1037); self.recompute_instance_static(); Ok(()) }
            "pclmg" => { validate_finite_parameter("pclmg", value)?; self.params.p1038 = value; self.mark_param_given(1038); self.recompute_instance_static(); Ok(()) }
            "lpclmg" => { validate_finite_parameter("lpclmg", value)?; self.params.p1039 = value; self.mark_param_given(1039); self.recompute_instance_static(); Ok(()) }
            "npclmg" => { validate_finite_parameter("npclmg", value)?; self.params.p1040 = value; self.mark_param_given(1040); self.recompute_instance_static(); Ok(()) }
            "ppclmg" => { validate_finite_parameter("ppclmg", value)?; self.params.p1041 = value; self.mark_param_given(1041); self.recompute_instance_static(); Ok(()) }
            "wpclmg" => { validate_finite_parameter("wpclmg", value)?; self.params.p1042 = value; self.mark_param_given(1042); self.recompute_instance_static(); Ok(()) }
            "p2pclmg" => { validate_finite_parameter("p2pclmg", value)?; self.params.p1043 = value; self.mark_param_given(1043); self.recompute_instance_static(); Ok(()) }
            "pclmcv" => { validate_finite_parameter("pclmcv", value)?; self.params.p1044 = value; self.mark_param_given(1044); self.recompute_instance_static(); Ok(()) }
            "lpclmcv" => { validate_finite_parameter("lpclmcv", value)?; self.params.p1045 = value; self.mark_param_given(1045); self.recompute_instance_static(); Ok(()) }
            "npclmcv" => { validate_finite_parameter("npclmcv", value)?; self.params.p1046 = value; self.mark_param_given(1046); self.recompute_instance_static(); Ok(()) }
            "ppclmcv" => { validate_finite_parameter("ppclmcv", value)?; self.params.p1047 = value; self.mark_param_given(1047); self.recompute_instance_static(); Ok(()) }
            "wpclmcv" => { validate_finite_parameter("wpclmcv", value)?; self.params.p1048 = value; self.mark_param_given(1048); self.recompute_instance_static(); Ok(()) }
            "p2pclmcv" => { validate_finite_parameter("p2pclmcv", value)?; self.params.p1049 = value; self.mark_param_given(1049); self.recompute_instance_static(); Ok(()) }
            "a1" => { validate_finite_parameter("a1", value)?; self.params.p1050 = value; self.mark_param_given(1050); self.recompute_instance_static(); Ok(()) }
            "la1" => { validate_finite_parameter("la1", value)?; self.params.p1051 = value; self.mark_param_given(1051); self.recompute_instance_static(); Ok(()) }
            "na1" => { validate_finite_parameter("na1", value)?; self.params.p1052 = value; self.mark_param_given(1052); self.recompute_instance_static(); Ok(()) }
            "pa1" => { validate_finite_parameter("pa1", value)?; self.params.p1053 = value; self.mark_param_given(1053); self.recompute_instance_static(); Ok(()) }
            "wa1" => { validate_finite_parameter("wa1", value)?; self.params.p1054 = value; self.mark_param_given(1054); self.recompute_instance_static(); Ok(()) }
            "p2a1" => { validate_finite_parameter("p2a1", value)?; self.params.p1055 = value; self.mark_param_given(1055); self.recompute_instance_static(); Ok(()) }
            "a11" => { validate_finite_parameter("a11", value)?; self.params.p1056 = value; self.mark_param_given(1056); self.recompute_instance_static(); Ok(()) }
            "la11" => { validate_finite_parameter("la11", value)?; self.params.p1057 = value; self.mark_param_given(1057); self.recompute_instance_static(); Ok(()) }
            "na11" => { validate_finite_parameter("na11", value)?; self.params.p1058 = value; self.mark_param_given(1058); self.recompute_instance_static(); Ok(()) }
            "pa11" => { validate_finite_parameter("pa11", value)?; self.params.p1059 = value; self.mark_param_given(1059); self.recompute_instance_static(); Ok(()) }
            "wa11" => { validate_finite_parameter("wa11", value)?; self.params.p1060 = value; self.mark_param_given(1060); self.recompute_instance_static(); Ok(()) }
            "p2a11" => { validate_finite_parameter("p2a11", value)?; self.params.p1061 = value; self.mark_param_given(1061); self.recompute_instance_static(); Ok(()) }
            "a2" => { validate_finite_parameter("a2", value)?; self.params.p1062 = value; self.mark_param_given(1062); self.recompute_instance_static(); Ok(()) }
            "la2" => { validate_finite_parameter("la2", value)?; self.params.p1063 = value; self.mark_param_given(1063); self.recompute_instance_static(); Ok(()) }
            "na2" => { validate_finite_parameter("na2", value)?; self.params.p1064 = value; self.mark_param_given(1064); self.recompute_instance_static(); Ok(()) }
            "pa2" => { validate_finite_parameter("pa2", value)?; self.params.p1065 = value; self.mark_param_given(1065); self.recompute_instance_static(); Ok(()) }
            "wa2" => { validate_finite_parameter("wa2", value)?; self.params.p1066 = value; self.mark_param_given(1066); self.recompute_instance_static(); Ok(()) }
            "p2a2" => { validate_finite_parameter("p2a2", value)?; self.params.p1067 = value; self.mark_param_given(1067); self.recompute_instance_static(); Ok(()) }
            "a21" => { validate_finite_parameter("a21", value)?; self.params.p1068 = value; self.mark_param_given(1068); self.recompute_instance_static(); Ok(()) }
            "la21" => { validate_finite_parameter("la21", value)?; self.params.p1069 = value; self.mark_param_given(1069); self.recompute_instance_static(); Ok(()) }
            "na21" => { validate_finite_parameter("na21", value)?; self.params.p1070 = value; self.mark_param_given(1070); self.recompute_instance_static(); Ok(()) }
            "pa21" => { validate_finite_parameter("pa21", value)?; self.params.p1071 = value; self.mark_param_given(1071); self.recompute_instance_static(); Ok(()) }
            "wa21" => { validate_finite_parameter("wa21", value)?; self.params.p1072 = value; self.mark_param_given(1072); self.recompute_instance_static(); Ok(()) }
            "p2a21" => { validate_finite_parameter("p2a21", value)?; self.params.p1073 = value; self.mark_param_given(1073); self.recompute_instance_static(); Ok(()) }
            "rgext" => { validate_parameter("rgext", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1074 = value; self.mark_param_given(1074); self.recompute_instance_static(); Ok(()) }
            "rgfin" => { validate_parameter("rgfin", value, Some((0.001, "0.001")), false, None, true, &[])?; self.params.p1075 = value; self.mark_param_given(1075); self.recompute_instance_static(); Ok(()) }
            "rgint" => { validate_parameter("rgint", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1076 = value; self.mark_param_given(1076); self.recompute_instance_static(); Ok(()) }
            "rgp" => { validate_parameter("rgp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1077 = value; self.mark_param_given(1077); self.recompute_instance_static(); Ok(()) }
            "rshs" => { validate_finite_parameter("rshs", value)?; self.params.p1078 = value; self.mark_param_given(1078); self.recompute_instance_static(); Ok(()) }
            "rshd" => { validate_finite_parameter("rshd", value)?; self.params.p1079 = value; self.mark_param_given(1079); self.recompute_instance_static(); Ok(()) }
            "hepi" => { validate_finite_parameter("hepi", value)?; self.params.p1080 = value; self.mark_param_given(1080); self.recompute_instance_static(); Ok(()) }
            "tsili" => { validate_finite_parameter("tsili", value)?; self.params.p1081 = value; self.mark_param_given(1081); self.recompute_instance_static(); Ok(()) }
            "rhoc" => { validate_parameter("rhoc", value, Some((1e-18, "1e-18")), false, Some((1e-9, "1e-9")), false, &[])?; self.params.p1082 = value; self.mark_param_given(1082); self.recompute_instance_static(); Ok(()) }
            "rhorsd" => { validate_parameter("rhorsd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1083 = value; self.mark_param_given(1083); self.recompute_instance_static(); Ok(()) }
            "cratio" => { validate_parameter("cratio", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1084 = value; self.mark_param_given(1084); self.recompute_instance_static(); Ok(()) }
            "deltaprsd" => { validate_finite_parameter("deltaprsd", value)?; self.params.p1085 = value; self.mark_param_given(1085); self.recompute_instance_static(); Ok(()) }
            "sdterm" => { validate_parameter("sdterm", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1086 = value; self.mark_param_given(1086); self.recompute_instance_static(); Ok(()) }
            "lsp" => { validate_parameter("lsp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1087 = value; self.mark_param_given(1087); self.recompute_instance_static(); Ok(()) }
            "epsrsp" => { validate_parameter("epsrsp", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p1088 = value; self.mark_param_given(1088); self.recompute_instance_static(); Ok(()) }
            "tgate" => { validate_parameter("tgate", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1089 = value; self.mark_param_given(1089); self.recompute_instance_static(); Ok(()) }
            "tmask" => { validate_parameter("tmask", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1090 = value; self.mark_param_given(1090); self.recompute_instance_static(); Ok(()) }
            "asiliend" => { validate_parameter("asiliend", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1091 = value; self.mark_param_given(1091); self.recompute_instance_static(); Ok(()) }
            "arsdend" => { validate_parameter("arsdend", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1092 = value; self.mark_param_given(1092); self.recompute_instance_static(); Ok(()) }
            "prsdend" => { validate_parameter("prsdend", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1093 = value; self.mark_param_given(1093); self.recompute_instance_static(); Ok(()) }
            "rgeoa" => { validate_finite_parameter("rgeoa", value)?; self.params.p1094 = value; self.mark_param_given(1094); self.recompute_instance_static(); Ok(()) }
            "rgeob" => { validate_finite_parameter("rgeob", value)?; self.params.p1095 = value; self.mark_param_given(1095); self.recompute_instance_static(); Ok(()) }
            "rgeoc" => { validate_finite_parameter("rgeoc", value)?; self.params.p1096 = value; self.mark_param_given(1096); self.recompute_instance_static(); Ok(()) }
            "rgeod" => { validate_finite_parameter("rgeod", value)?; self.params.p1097 = value; self.mark_param_given(1097); self.recompute_instance_static(); Ok(()) }
            "rgeoe" => { validate_finite_parameter("rgeoe", value)?; self.params.p1098 = value; self.mark_param_given(1098); self.recompute_instance_static(); Ok(()) }
            "cgeoa" => { validate_finite_parameter("cgeoa", value)?; self.params.p1099 = value; self.mark_param_given(1099); self.recompute_instance_static(); Ok(()) }
            "cgeob" => { validate_finite_parameter("cgeob", value)?; self.params.p1100 = value; self.mark_param_given(1100); self.recompute_instance_static(); Ok(()) }
            "cgeoc" => { validate_finite_parameter("cgeoc", value)?; self.params.p1101 = value; self.mark_param_given(1101); self.recompute_instance_static(); Ok(()) }
            "cgeod" => { validate_finite_parameter("cgeod", value)?; self.params.p1102 = value; self.mark_param_given(1102); self.recompute_instance_static(); Ok(()) }
            "cgeoe" => { validate_parameter("cgeoe", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1103 = value; self.mark_param_given(1103); self.recompute_instance_static(); Ok(()) }
            "dlcigs" => { validate_finite_parameter("dlcigs", value)?; self.params.p1104 = value; self.mark_param_given(1104); self.recompute_instance_static(); Ok(()) }
            "dlcigd" => { validate_finite_parameter("dlcigd", value)?; self.params.p1105 = value; self.mark_param_given(1105); self.recompute_instance_static(); Ok(()) }
            "vfbsd" => { validate_finite_parameter("vfbsd", value)?; self.params.p1106 = value; self.mark_param_given(1106); self.recompute_instance_static(); Ok(()) }
            "vfbsdcv" => { validate_finite_parameter("vfbsdcv", value)?; self.params.p1107 = value; self.mark_param_given(1107); self.recompute_instance_static(); Ok(()) }
            "toxref" => { validate_parameter("toxref", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1108 = value; self.mark_param_given(1108); self.recompute_instance_static(); Ok(()) }
            "toxg" => { validate_parameter("toxg", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p1109 = value; self.mark_param_given(1109); self.recompute_instance_static(); Ok(()) }
            "igbinvclamp" => { validate_parameter("igbinvclamp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1110 = value; self.mark_param_given(1110); self.recompute_instance_static(); Ok(()) }
            "igbaccclamp" => { validate_parameter("igbaccclamp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1111 = value; self.mark_param_given(1111); self.recompute_instance_static(); Ok(()) }
            "igcinvclamp" => { validate_parameter("igcinvclamp", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1112 = value; self.mark_param_given(1112); self.recompute_instance_static(); Ok(()) }
            "ntox" => { validate_finite_parameter("ntox", value)?; self.params.p1113 = value; self.mark_param_given(1113); self.recompute_instance_static(); Ok(()) }
            "lntox" => { validate_finite_parameter("lntox", value)?; self.params.p1114 = value; self.mark_param_given(1114); self.recompute_instance_static(); Ok(()) }
            "nntox" => { validate_finite_parameter("nntox", value)?; self.params.p1115 = value; self.mark_param_given(1115); self.recompute_instance_static(); Ok(()) }
            "pntox" => { validate_finite_parameter("pntox", value)?; self.params.p1116 = value; self.mark_param_given(1116); self.recompute_instance_static(); Ok(()) }
            "wntox" => { validate_finite_parameter("wntox", value)?; self.params.p1117 = value; self.mark_param_given(1117); self.recompute_instance_static(); Ok(()) }
            "p2ntox" => { validate_finite_parameter("p2ntox", value)?; self.params.p1118 = value; self.mark_param_given(1118); self.recompute_instance_static(); Ok(()) }
            "aigbinv" => { validate_finite_parameter("aigbinv", value)?; self.params.p1119 = value; self.mark_param_given(1119); self.recompute_instance_static(); Ok(()) }
            "laigbinv" => { validate_finite_parameter("laigbinv", value)?; self.params.p1120 = value; self.mark_param_given(1120); self.recompute_instance_static(); Ok(()) }
            "naigbinv" => { validate_finite_parameter("naigbinv", value)?; self.params.p1121 = value; self.mark_param_given(1121); self.recompute_instance_static(); Ok(()) }
            "paigbinv" => { validate_finite_parameter("paigbinv", value)?; self.params.p1122 = value; self.mark_param_given(1122); self.recompute_instance_static(); Ok(()) }
            "waigbinv" => { validate_finite_parameter("waigbinv", value)?; self.params.p1123 = value; self.mark_param_given(1123); self.recompute_instance_static(); Ok(()) }
            "p2aigbinv" => { validate_finite_parameter("p2aigbinv", value)?; self.params.p1124 = value; self.mark_param_given(1124); self.recompute_instance_static(); Ok(()) }
            "aigbinv1" => { validate_finite_parameter("aigbinv1", value)?; self.params.p1125 = value; self.mark_param_given(1125); self.recompute_instance_static(); Ok(()) }
            "laigbinv1" => { validate_finite_parameter("laigbinv1", value)?; self.params.p1126 = value; self.mark_param_given(1126); self.recompute_instance_static(); Ok(()) }
            "naigbinv1" => { validate_finite_parameter("naigbinv1", value)?; self.params.p1127 = value; self.mark_param_given(1127); self.recompute_instance_static(); Ok(()) }
            "paigbinv1" => { validate_finite_parameter("paigbinv1", value)?; self.params.p1128 = value; self.mark_param_given(1128); self.recompute_instance_static(); Ok(()) }
            "waigbinv1" => { validate_finite_parameter("waigbinv1", value)?; self.params.p1129 = value; self.mark_param_given(1129); self.recompute_instance_static(); Ok(()) }
            "p2aigbinv1" => { validate_finite_parameter("p2aigbinv1", value)?; self.params.p1130 = value; self.mark_param_given(1130); self.recompute_instance_static(); Ok(()) }
            "bigbinv" => { validate_finite_parameter("bigbinv", value)?; self.params.p1131 = value; self.mark_param_given(1131); self.recompute_instance_static(); Ok(()) }
            "lbigbinv" => { validate_finite_parameter("lbigbinv", value)?; self.params.p1132 = value; self.mark_param_given(1132); self.recompute_instance_static(); Ok(()) }
            "nbigbinv" => { validate_finite_parameter("nbigbinv", value)?; self.params.p1133 = value; self.mark_param_given(1133); self.recompute_instance_static(); Ok(()) }
            "pbigbinv" => { validate_finite_parameter("pbigbinv", value)?; self.params.p1134 = value; self.mark_param_given(1134); self.recompute_instance_static(); Ok(()) }
            "wbigbinv" => { validate_finite_parameter("wbigbinv", value)?; self.params.p1135 = value; self.mark_param_given(1135); self.recompute_instance_static(); Ok(()) }
            "p2bigbinv" => { validate_finite_parameter("p2bigbinv", value)?; self.params.p1136 = value; self.mark_param_given(1136); self.recompute_instance_static(); Ok(()) }
            "cigbinv" => { validate_finite_parameter("cigbinv", value)?; self.params.p1137 = value; self.mark_param_given(1137); self.recompute_instance_static(); Ok(()) }
            "lcigbinv" => { validate_finite_parameter("lcigbinv", value)?; self.params.p1138 = value; self.mark_param_given(1138); self.recompute_instance_static(); Ok(()) }
            "ncigbinv" => { validate_finite_parameter("ncigbinv", value)?; self.params.p1139 = value; self.mark_param_given(1139); self.recompute_instance_static(); Ok(()) }
            "pcigbinv" => { validate_finite_parameter("pcigbinv", value)?; self.params.p1140 = value; self.mark_param_given(1140); self.recompute_instance_static(); Ok(()) }
            "wcigbinv" => { validate_finite_parameter("wcigbinv", value)?; self.params.p1141 = value; self.mark_param_given(1141); self.recompute_instance_static(); Ok(()) }
            "p2cigbinv" => { validate_finite_parameter("p2cigbinv", value)?; self.params.p1142 = value; self.mark_param_given(1142); self.recompute_instance_static(); Ok(()) }
            "eigbinv" => { validate_finite_parameter("eigbinv", value)?; self.params.p1143 = value; self.mark_param_given(1143); self.recompute_instance_static(); Ok(()) }
            "leigbinv" => { validate_finite_parameter("leigbinv", value)?; self.params.p1144 = value; self.mark_param_given(1144); self.recompute_instance_static(); Ok(()) }
            "neigbinv" => { validate_finite_parameter("neigbinv", value)?; self.params.p1145 = value; self.mark_param_given(1145); self.recompute_instance_static(); Ok(()) }
            "peigbinv" => { validate_finite_parameter("peigbinv", value)?; self.params.p1146 = value; self.mark_param_given(1146); self.recompute_instance_static(); Ok(()) }
            "weigbinv" => { validate_finite_parameter("weigbinv", value)?; self.params.p1147 = value; self.mark_param_given(1147); self.recompute_instance_static(); Ok(()) }
            "p2eigbinv" => { validate_finite_parameter("p2eigbinv", value)?; self.params.p1148 = value; self.mark_param_given(1148); self.recompute_instance_static(); Ok(()) }
            "nigbinv" => { validate_finite_parameter("nigbinv", value)?; self.params.p1149 = value; self.mark_param_given(1149); self.recompute_instance_static(); Ok(()) }
            "lnigbinv" => { validate_finite_parameter("lnigbinv", value)?; self.params.p1150 = value; self.mark_param_given(1150); self.recompute_instance_static(); Ok(()) }
            "nnigbinv" => { validate_finite_parameter("nnigbinv", value)?; self.params.p1151 = value; self.mark_param_given(1151); self.recompute_instance_static(); Ok(()) }
            "pnigbinv" => { validate_finite_parameter("pnigbinv", value)?; self.params.p1152 = value; self.mark_param_given(1152); self.recompute_instance_static(); Ok(()) }
            "wnigbinv" => { validate_finite_parameter("wnigbinv", value)?; self.params.p1153 = value; self.mark_param_given(1153); self.recompute_instance_static(); Ok(()) }
            "p2nigbinv" => { validate_finite_parameter("p2nigbinv", value)?; self.params.p1154 = value; self.mark_param_given(1154); self.recompute_instance_static(); Ok(()) }
            "aigbacc" => { validate_finite_parameter("aigbacc", value)?; self.params.p1155 = value; self.mark_param_given(1155); self.recompute_instance_static(); Ok(()) }
            "laigbacc" => { validate_finite_parameter("laigbacc", value)?; self.params.p1156 = value; self.mark_param_given(1156); self.recompute_instance_static(); Ok(()) }
            "naigbacc" => { validate_finite_parameter("naigbacc", value)?; self.params.p1157 = value; self.mark_param_given(1157); self.recompute_instance_static(); Ok(()) }
            "paigbacc" => { validate_finite_parameter("paigbacc", value)?; self.params.p1158 = value; self.mark_param_given(1158); self.recompute_instance_static(); Ok(()) }
            "waigbacc" => { validate_finite_parameter("waigbacc", value)?; self.params.p1159 = value; self.mark_param_given(1159); self.recompute_instance_static(); Ok(()) }
            "p2aigbacc" => { validate_finite_parameter("p2aigbacc", value)?; self.params.p1160 = value; self.mark_param_given(1160); self.recompute_instance_static(); Ok(()) }
            "aigbacc1" => { validate_finite_parameter("aigbacc1", value)?; self.params.p1161 = value; self.mark_param_given(1161); self.recompute_instance_static(); Ok(()) }
            "laigbacc1" => { validate_finite_parameter("laigbacc1", value)?; self.params.p1162 = value; self.mark_param_given(1162); self.recompute_instance_static(); Ok(()) }
            "naigbacc1" => { validate_finite_parameter("naigbacc1", value)?; self.params.p1163 = value; self.mark_param_given(1163); self.recompute_instance_static(); Ok(()) }
            "paigbacc1" => { validate_finite_parameter("paigbacc1", value)?; self.params.p1164 = value; self.mark_param_given(1164); self.recompute_instance_static(); Ok(()) }
            "waigbacc1" => { validate_finite_parameter("waigbacc1", value)?; self.params.p1165 = value; self.mark_param_given(1165); self.recompute_instance_static(); Ok(()) }
            "p2aigbacc1" => { validate_finite_parameter("p2aigbacc1", value)?; self.params.p1166 = value; self.mark_param_given(1166); self.recompute_instance_static(); Ok(()) }
            "bigbacc" => { validate_finite_parameter("bigbacc", value)?; self.params.p1167 = value; self.mark_param_given(1167); self.recompute_instance_static(); Ok(()) }
            "lbigbacc" => { validate_finite_parameter("lbigbacc", value)?; self.params.p1168 = value; self.mark_param_given(1168); self.recompute_instance_static(); Ok(()) }
            "nbigbacc" => { validate_finite_parameter("nbigbacc", value)?; self.params.p1169 = value; self.mark_param_given(1169); self.recompute_instance_static(); Ok(()) }
            "pbigbacc" => { validate_finite_parameter("pbigbacc", value)?; self.params.p1170 = value; self.mark_param_given(1170); self.recompute_instance_static(); Ok(()) }
            "wbigbacc" => { validate_finite_parameter("wbigbacc", value)?; self.params.p1171 = value; self.mark_param_given(1171); self.recompute_instance_static(); Ok(()) }
            "p2bigbacc" => { validate_finite_parameter("p2bigbacc", value)?; self.params.p1172 = value; self.mark_param_given(1172); self.recompute_instance_static(); Ok(()) }
            "cigbacc" => { validate_finite_parameter("cigbacc", value)?; self.params.p1173 = value; self.mark_param_given(1173); self.recompute_instance_static(); Ok(()) }
            "lcigbacc" => { validate_finite_parameter("lcigbacc", value)?; self.params.p1174 = value; self.mark_param_given(1174); self.recompute_instance_static(); Ok(()) }
            "ncigbacc" => { validate_finite_parameter("ncigbacc", value)?; self.params.p1175 = value; self.mark_param_given(1175); self.recompute_instance_static(); Ok(()) }
            "pcigbacc" => { validate_finite_parameter("pcigbacc", value)?; self.params.p1176 = value; self.mark_param_given(1176); self.recompute_instance_static(); Ok(()) }
            "wcigbacc" => { validate_finite_parameter("wcigbacc", value)?; self.params.p1177 = value; self.mark_param_given(1177); self.recompute_instance_static(); Ok(()) }
            "p2cigbacc" => { validate_finite_parameter("p2cigbacc", value)?; self.params.p1178 = value; self.mark_param_given(1178); self.recompute_instance_static(); Ok(()) }
            "nigbacc" => { validate_finite_parameter("nigbacc", value)?; self.params.p1179 = value; self.mark_param_given(1179); self.recompute_instance_static(); Ok(()) }
            "lnigbacc" => { validate_finite_parameter("lnigbacc", value)?; self.params.p1180 = value; self.mark_param_given(1180); self.recompute_instance_static(); Ok(()) }
            "nnigbacc" => { validate_finite_parameter("nnigbacc", value)?; self.params.p1181 = value; self.mark_param_given(1181); self.recompute_instance_static(); Ok(()) }
            "pnigbacc" => { validate_finite_parameter("pnigbacc", value)?; self.params.p1182 = value; self.mark_param_given(1182); self.recompute_instance_static(); Ok(()) }
            "wnigbacc" => { validate_finite_parameter("wnigbacc", value)?; self.params.p1183 = value; self.mark_param_given(1183); self.recompute_instance_static(); Ok(()) }
            "p2nigbacc" => { validate_finite_parameter("p2nigbacc", value)?; self.params.p1184 = value; self.mark_param_given(1184); self.recompute_instance_static(); Ok(()) }
            "aigc" => { validate_finite_parameter("aigc", value)?; self.params.p1185 = value; self.mark_param_given(1185); self.recompute_instance_static(); Ok(()) }
            "laigc" => { validate_finite_parameter("laigc", value)?; self.params.p1186 = value; self.mark_param_given(1186); self.recompute_instance_static(); Ok(()) }
            "naigc" => { validate_finite_parameter("naigc", value)?; self.params.p1187 = value; self.mark_param_given(1187); self.recompute_instance_static(); Ok(()) }
            "paigc" => { validate_finite_parameter("paigc", value)?; self.params.p1188 = value; self.mark_param_given(1188); self.recompute_instance_static(); Ok(()) }
            "waigc" => { validate_finite_parameter("waigc", value)?; self.params.p1189 = value; self.mark_param_given(1189); self.recompute_instance_static(); Ok(()) }
            "p2aigc" => { validate_finite_parameter("p2aigc", value)?; self.params.p1190 = value; self.mark_param_given(1190); self.recompute_instance_static(); Ok(()) }
            "aigc1" => { validate_finite_parameter("aigc1", value)?; self.params.p1191 = value; self.mark_param_given(1191); self.recompute_instance_static(); Ok(()) }
            "laigc1" => { validate_finite_parameter("laigc1", value)?; self.params.p1192 = value; self.mark_param_given(1192); self.recompute_instance_static(); Ok(()) }
            "naigc1" => { validate_finite_parameter("naigc1", value)?; self.params.p1193 = value; self.mark_param_given(1193); self.recompute_instance_static(); Ok(()) }
            "paigc1" => { validate_finite_parameter("paigc1", value)?; self.params.p1194 = value; self.mark_param_given(1194); self.recompute_instance_static(); Ok(()) }
            "waigc1" => { validate_finite_parameter("waigc1", value)?; self.params.p1195 = value; self.mark_param_given(1195); self.recompute_instance_static(); Ok(()) }
            "p2aigc1" => { validate_finite_parameter("p2aigc1", value)?; self.params.p1196 = value; self.mark_param_given(1196); self.recompute_instance_static(); Ok(()) }
            "bigc" => { validate_finite_parameter("bigc", value)?; self.params.p1197 = value; self.mark_param_given(1197); self.recompute_instance_static(); Ok(()) }
            "lbigc" => { validate_finite_parameter("lbigc", value)?; self.params.p1198 = value; self.mark_param_given(1198); self.recompute_instance_static(); Ok(()) }
            "nbigc" => { validate_finite_parameter("nbigc", value)?; self.params.p1199 = value; self.mark_param_given(1199); self.recompute_instance_static(); Ok(()) }
            "pbigc" => { validate_finite_parameter("pbigc", value)?; self.params.p1200 = value; self.mark_param_given(1200); self.recompute_instance_static(); Ok(()) }
            "wbigc" => { validate_finite_parameter("wbigc", value)?; self.params.p1201 = value; self.mark_param_given(1201); self.recompute_instance_static(); Ok(()) }
            "p2bigc" => { validate_finite_parameter("p2bigc", value)?; self.params.p1202 = value; self.mark_param_given(1202); self.recompute_instance_static(); Ok(()) }
            "cigc" => { validate_finite_parameter("cigc", value)?; self.params.p1203 = value; self.mark_param_given(1203); self.recompute_instance_static(); Ok(()) }
            "lcigc" => { validate_finite_parameter("lcigc", value)?; self.params.p1204 = value; self.mark_param_given(1204); self.recompute_instance_static(); Ok(()) }
            "ncigc" => { validate_finite_parameter("ncigc", value)?; self.params.p1205 = value; self.mark_param_given(1205); self.recompute_instance_static(); Ok(()) }
            "pcigc" => { validate_finite_parameter("pcigc", value)?; self.params.p1206 = value; self.mark_param_given(1206); self.recompute_instance_static(); Ok(()) }
            "wcigc" => { validate_finite_parameter("wcigc", value)?; self.params.p1207 = value; self.mark_param_given(1207); self.recompute_instance_static(); Ok(()) }
            "p2cigc" => { validate_finite_parameter("p2cigc", value)?; self.params.p1208 = value; self.mark_param_given(1208); self.recompute_instance_static(); Ok(()) }
            "pigcd" => { validate_finite_parameter("pigcd", value)?; self.params.p1209 = value; self.mark_param_given(1209); self.recompute_instance_static(); Ok(()) }
            "lpigcd" => { validate_finite_parameter("lpigcd", value)?; self.params.p1210 = value; self.mark_param_given(1210); self.recompute_instance_static(); Ok(()) }
            "npigcd" => { validate_finite_parameter("npigcd", value)?; self.params.p1211 = value; self.mark_param_given(1211); self.recompute_instance_static(); Ok(()) }
            "ppigcd" => { validate_finite_parameter("ppigcd", value)?; self.params.p1212 = value; self.mark_param_given(1212); self.recompute_instance_static(); Ok(()) }
            "wpigcd" => { validate_finite_parameter("wpigcd", value)?; self.params.p1213 = value; self.mark_param_given(1213); self.recompute_instance_static(); Ok(()) }
            "p2pigcd" => { validate_finite_parameter("p2pigcd", value)?; self.params.p1214 = value; self.mark_param_given(1214); self.recompute_instance_static(); Ok(()) }
            "aigs" => { validate_finite_parameter("aigs", value)?; self.params.p1215 = value; self.mark_param_given(1215); self.recompute_instance_static(); Ok(()) }
            "laigs" => { validate_finite_parameter("laigs", value)?; self.params.p1216 = value; self.mark_param_given(1216); self.recompute_instance_static(); Ok(()) }
            "naigs" => { validate_finite_parameter("naigs", value)?; self.params.p1217 = value; self.mark_param_given(1217); self.recompute_instance_static(); Ok(()) }
            "paigs" => { validate_finite_parameter("paigs", value)?; self.params.p1218 = value; self.mark_param_given(1218); self.recompute_instance_static(); Ok(()) }
            "waigs" => { validate_finite_parameter("waigs", value)?; self.params.p1219 = value; self.mark_param_given(1219); self.recompute_instance_static(); Ok(()) }
            "p2aigs" => { validate_finite_parameter("p2aigs", value)?; self.params.p1220 = value; self.mark_param_given(1220); self.recompute_instance_static(); Ok(()) }
            "aigs1" => { validate_finite_parameter("aigs1", value)?; self.params.p1221 = value; self.mark_param_given(1221); self.recompute_instance_static(); Ok(()) }
            "laigs1" => { validate_finite_parameter("laigs1", value)?; self.params.p1222 = value; self.mark_param_given(1222); self.recompute_instance_static(); Ok(()) }
            "naigs1" => { validate_finite_parameter("naigs1", value)?; self.params.p1223 = value; self.mark_param_given(1223); self.recompute_instance_static(); Ok(()) }
            "paigs1" => { validate_finite_parameter("paigs1", value)?; self.params.p1224 = value; self.mark_param_given(1224); self.recompute_instance_static(); Ok(()) }
            "waigs1" => { validate_finite_parameter("waigs1", value)?; self.params.p1225 = value; self.mark_param_given(1225); self.recompute_instance_static(); Ok(()) }
            "p2aigs1" => { validate_finite_parameter("p2aigs1", value)?; self.params.p1226 = value; self.mark_param_given(1226); self.recompute_instance_static(); Ok(()) }
            "bigs" => { validate_finite_parameter("bigs", value)?; self.params.p1227 = value; self.mark_param_given(1227); self.recompute_instance_static(); Ok(()) }
            "lbigs" => { validate_finite_parameter("lbigs", value)?; self.params.p1228 = value; self.mark_param_given(1228); self.recompute_instance_static(); Ok(()) }
            "nbigs" => { validate_finite_parameter("nbigs", value)?; self.params.p1229 = value; self.mark_param_given(1229); self.recompute_instance_static(); Ok(()) }
            "pbigs" => { validate_finite_parameter("pbigs", value)?; self.params.p1230 = value; self.mark_param_given(1230); self.recompute_instance_static(); Ok(()) }
            "wbigs" => { validate_finite_parameter("wbigs", value)?; self.params.p1231 = value; self.mark_param_given(1231); self.recompute_instance_static(); Ok(()) }
            "p2bigs" => { validate_finite_parameter("p2bigs", value)?; self.params.p1232 = value; self.mark_param_given(1232); self.recompute_instance_static(); Ok(()) }
            "cigs" => { validate_finite_parameter("cigs", value)?; self.params.p1233 = value; self.mark_param_given(1233); self.recompute_instance_static(); Ok(()) }
            "lcigs" => { validate_finite_parameter("lcigs", value)?; self.params.p1234 = value; self.mark_param_given(1234); self.recompute_instance_static(); Ok(()) }
            "ncigs" => { validate_finite_parameter("ncigs", value)?; self.params.p1235 = value; self.mark_param_given(1235); self.recompute_instance_static(); Ok(()) }
            "pcigs" => { validate_finite_parameter("pcigs", value)?; self.params.p1236 = value; self.mark_param_given(1236); self.recompute_instance_static(); Ok(()) }
            "wcigs" => { validate_finite_parameter("wcigs", value)?; self.params.p1237 = value; self.mark_param_given(1237); self.recompute_instance_static(); Ok(()) }
            "p2cigs" => { validate_finite_parameter("p2cigs", value)?; self.params.p1238 = value; self.mark_param_given(1238); self.recompute_instance_static(); Ok(()) }
            "aigd" => { validate_finite_parameter("aigd", value)?; self.params.p1239 = value; self.mark_param_given(1239); self.recompute_instance_static(); Ok(()) }
            "laigd" => { validate_finite_parameter("laigd", value)?; self.params.p1240 = value; self.mark_param_given(1240); self.recompute_instance_static(); Ok(()) }
            "naigd" => { validate_finite_parameter("naigd", value)?; self.params.p1241 = value; self.mark_param_given(1241); self.recompute_instance_static(); Ok(()) }
            "paigd" => { validate_finite_parameter("paigd", value)?; self.params.p1242 = value; self.mark_param_given(1242); self.recompute_instance_static(); Ok(()) }
            "waigd" => { validate_finite_parameter("waigd", value)?; self.params.p1243 = value; self.mark_param_given(1243); self.recompute_instance_static(); Ok(()) }
            "p2aigd" => { validate_finite_parameter("p2aigd", value)?; self.params.p1244 = value; self.mark_param_given(1244); self.recompute_instance_static(); Ok(()) }
            "aigd1" => { validate_finite_parameter("aigd1", value)?; self.params.p1245 = value; self.mark_param_given(1245); self.recompute_instance_static(); Ok(()) }
            "laigd1" => { validate_finite_parameter("laigd1", value)?; self.params.p1246 = value; self.mark_param_given(1246); self.recompute_instance_static(); Ok(()) }
            "naigd1" => { validate_finite_parameter("naigd1", value)?; self.params.p1247 = value; self.mark_param_given(1247); self.recompute_instance_static(); Ok(()) }
            "paigd1" => { validate_finite_parameter("paigd1", value)?; self.params.p1248 = value; self.mark_param_given(1248); self.recompute_instance_static(); Ok(()) }
            "waigd1" => { validate_finite_parameter("waigd1", value)?; self.params.p1249 = value; self.mark_param_given(1249); self.recompute_instance_static(); Ok(()) }
            "p2aigd1" => { validate_finite_parameter("p2aigd1", value)?; self.params.p1250 = value; self.mark_param_given(1250); self.recompute_instance_static(); Ok(()) }
            "bigd" => { validate_finite_parameter("bigd", value)?; self.params.p1251 = value; self.mark_param_given(1251); self.recompute_instance_static(); Ok(()) }
            "lbigd" => { validate_finite_parameter("lbigd", value)?; self.params.p1252 = value; self.mark_param_given(1252); self.recompute_instance_static(); Ok(()) }
            "nbigd" => { validate_finite_parameter("nbigd", value)?; self.params.p1253 = value; self.mark_param_given(1253); self.recompute_instance_static(); Ok(()) }
            "pbigd" => { validate_finite_parameter("pbigd", value)?; self.params.p1254 = value; self.mark_param_given(1254); self.recompute_instance_static(); Ok(()) }
            "wbigd" => { validate_finite_parameter("wbigd", value)?; self.params.p1255 = value; self.mark_param_given(1255); self.recompute_instance_static(); Ok(()) }
            "p2bigd" => { validate_finite_parameter("p2bigd", value)?; self.params.p1256 = value; self.mark_param_given(1256); self.recompute_instance_static(); Ok(()) }
            "cigd" => { validate_finite_parameter("cigd", value)?; self.params.p1257 = value; self.mark_param_given(1257); self.recompute_instance_static(); Ok(()) }
            "lcigd" => { validate_finite_parameter("lcigd", value)?; self.params.p1258 = value; self.mark_param_given(1258); self.recompute_instance_static(); Ok(()) }
            "ncigd" => { validate_finite_parameter("ncigd", value)?; self.params.p1259 = value; self.mark_param_given(1259); self.recompute_instance_static(); Ok(()) }
            "pcigd" => { validate_finite_parameter("pcigd", value)?; self.params.p1260 = value; self.mark_param_given(1260); self.recompute_instance_static(); Ok(()) }
            "wcigd" => { validate_finite_parameter("wcigd", value)?; self.params.p1261 = value; self.mark_param_given(1261); self.recompute_instance_static(); Ok(()) }
            "p2cigd" => { validate_finite_parameter("p2cigd", value)?; self.params.p1262 = value; self.mark_param_given(1262); self.recompute_instance_static(); Ok(()) }
            "poxedge" => { validate_finite_parameter("poxedge", value)?; self.params.p1263 = value; self.mark_param_given(1263); self.recompute_instance_static(); Ok(()) }
            "lpoxedge" => { validate_finite_parameter("lpoxedge", value)?; self.params.p1264 = value; self.mark_param_given(1264); self.recompute_instance_static(); Ok(()) }
            "npoxedge" => { validate_finite_parameter("npoxedge", value)?; self.params.p1265 = value; self.mark_param_given(1265); self.recompute_instance_static(); Ok(()) }
            "ppoxedge" => { validate_finite_parameter("ppoxedge", value)?; self.params.p1266 = value; self.mark_param_given(1266); self.recompute_instance_static(); Ok(()) }
            "wpoxedge" => { validate_finite_parameter("wpoxedge", value)?; self.params.p1267 = value; self.mark_param_given(1267); self.recompute_instance_static(); Ok(()) }
            "p2poxedge" => { validate_finite_parameter("p2poxedge", value)?; self.params.p1268 = value; self.mark_param_given(1268); self.recompute_instance_static(); Ok(()) }
            "agidl" => { validate_finite_parameter("agidl", value)?; self.params.p1269 = value; self.mark_param_given(1269); self.recompute_instance_static(); Ok(()) }
            "lagidl" => { validate_finite_parameter("lagidl", value)?; self.params.p1270 = value; self.mark_param_given(1270); self.recompute_instance_static(); Ok(()) }
            "nagidl" => { validate_finite_parameter("nagidl", value)?; self.params.p1271 = value; self.mark_param_given(1271); self.recompute_instance_static(); Ok(()) }
            "pagidl" => { validate_finite_parameter("pagidl", value)?; self.params.p1272 = value; self.mark_param_given(1272); self.recompute_instance_static(); Ok(()) }
            "wagidl" => { validate_finite_parameter("wagidl", value)?; self.params.p1273 = value; self.mark_param_given(1273); self.recompute_instance_static(); Ok(()) }
            "p2agidl" => { validate_finite_parameter("p2agidl", value)?; self.params.p1274 = value; self.mark_param_given(1274); self.recompute_instance_static(); Ok(()) }
            "bgidl" => { validate_finite_parameter("bgidl", value)?; self.params.p1275 = value; self.mark_param_given(1275); self.recompute_instance_static(); Ok(()) }
            "lbgidl" => { validate_finite_parameter("lbgidl", value)?; self.params.p1276 = value; self.mark_param_given(1276); self.recompute_instance_static(); Ok(()) }
            "nbgidl" => { validate_finite_parameter("nbgidl", value)?; self.params.p1277 = value; self.mark_param_given(1277); self.recompute_instance_static(); Ok(()) }
            "pbgidl" => { validate_finite_parameter("pbgidl", value)?; self.params.p1278 = value; self.mark_param_given(1278); self.recompute_instance_static(); Ok(()) }
            "wbgidl" => { validate_finite_parameter("wbgidl", value)?; self.params.p1279 = value; self.mark_param_given(1279); self.recompute_instance_static(); Ok(()) }
            "p2bgidl" => { validate_finite_parameter("p2bgidl", value)?; self.params.p1280 = value; self.mark_param_given(1280); self.recompute_instance_static(); Ok(()) }
            "cgidl" => { validate_finite_parameter("cgidl", value)?; self.params.p1281 = value; self.mark_param_given(1281); self.recompute_instance_static(); Ok(()) }
            "lcgidl" => { validate_finite_parameter("lcgidl", value)?; self.params.p1282 = value; self.mark_param_given(1282); self.recompute_instance_static(); Ok(()) }
            "ncgidl" => { validate_finite_parameter("ncgidl", value)?; self.params.p1283 = value; self.mark_param_given(1283); self.recompute_instance_static(); Ok(()) }
            "pcgidl" => { validate_finite_parameter("pcgidl", value)?; self.params.p1284 = value; self.mark_param_given(1284); self.recompute_instance_static(); Ok(()) }
            "wcgidl" => { validate_finite_parameter("wcgidl", value)?; self.params.p1285 = value; self.mark_param_given(1285); self.recompute_instance_static(); Ok(()) }
            "p2cgidl" => { validate_finite_parameter("p2cgidl", value)?; self.params.p1286 = value; self.mark_param_given(1286); self.recompute_instance_static(); Ok(()) }
            "egidl" => { validate_finite_parameter("egidl", value)?; self.params.p1287 = value; self.mark_param_given(1287); self.recompute_instance_static(); Ok(()) }
            "legidl" => { validate_finite_parameter("legidl", value)?; self.params.p1288 = value; self.mark_param_given(1288); self.recompute_instance_static(); Ok(()) }
            "negidl" => { validate_finite_parameter("negidl", value)?; self.params.p1289 = value; self.mark_param_given(1289); self.recompute_instance_static(); Ok(()) }
            "pegidl" => { validate_finite_parameter("pegidl", value)?; self.params.p1290 = value; self.mark_param_given(1290); self.recompute_instance_static(); Ok(()) }
            "wegidl" => { validate_finite_parameter("wegidl", value)?; self.params.p1291 = value; self.mark_param_given(1291); self.recompute_instance_static(); Ok(()) }
            "p2egidl" => { validate_finite_parameter("p2egidl", value)?; self.params.p1292 = value; self.mark_param_given(1292); self.recompute_instance_static(); Ok(()) }
            "pgidl" => { validate_finite_parameter("pgidl", value)?; self.params.p1293 = value; self.mark_param_given(1293); self.recompute_instance_static(); Ok(()) }
            "lpgidl" => { validate_finite_parameter("lpgidl", value)?; self.params.p1294 = value; self.mark_param_given(1294); self.recompute_instance_static(); Ok(()) }
            "npgidl" => { validate_finite_parameter("npgidl", value)?; self.params.p1295 = value; self.mark_param_given(1295); self.recompute_instance_static(); Ok(()) }
            "ppgidl" => { validate_finite_parameter("ppgidl", value)?; self.params.p1296 = value; self.mark_param_given(1296); self.recompute_instance_static(); Ok(()) }
            "wpgidl" => { validate_finite_parameter("wpgidl", value)?; self.params.p1297 = value; self.mark_param_given(1297); self.recompute_instance_static(); Ok(()) }
            "p2pgidl" => { validate_finite_parameter("p2pgidl", value)?; self.params.p1298 = value; self.mark_param_given(1298); self.recompute_instance_static(); Ok(()) }
            "agisl" => { validate_finite_parameter("agisl", value)?; self.params.p1299 = value; self.mark_param_given(1299); self.recompute_instance_static(); Ok(()) }
            "lagisl" => { validate_finite_parameter("lagisl", value)?; self.params.p1300 = value; self.mark_param_given(1300); self.recompute_instance_static(); Ok(()) }
            "nagisl" => { validate_finite_parameter("nagisl", value)?; self.params.p1301 = value; self.mark_param_given(1301); self.recompute_instance_static(); Ok(()) }
            "pagisl" => { validate_finite_parameter("pagisl", value)?; self.params.p1302 = value; self.mark_param_given(1302); self.recompute_instance_static(); Ok(()) }
            "wagisl" => { validate_finite_parameter("wagisl", value)?; self.params.p1303 = value; self.mark_param_given(1303); self.recompute_instance_static(); Ok(()) }
            "p2agisl" => { validate_finite_parameter("p2agisl", value)?; self.params.p1304 = value; self.mark_param_given(1304); self.recompute_instance_static(); Ok(()) }
            "bgisl" => { validate_finite_parameter("bgisl", value)?; self.params.p1305 = value; self.mark_param_given(1305); self.recompute_instance_static(); Ok(()) }
            "lbgisl" => { validate_finite_parameter("lbgisl", value)?; self.params.p1306 = value; self.mark_param_given(1306); self.recompute_instance_static(); Ok(()) }
            "nbgisl" => { validate_finite_parameter("nbgisl", value)?; self.params.p1307 = value; self.mark_param_given(1307); self.recompute_instance_static(); Ok(()) }
            "pbgisl" => { validate_finite_parameter("pbgisl", value)?; self.params.p1308 = value; self.mark_param_given(1308); self.recompute_instance_static(); Ok(()) }
            "wbgisl" => { validate_finite_parameter("wbgisl", value)?; self.params.p1309 = value; self.mark_param_given(1309); self.recompute_instance_static(); Ok(()) }
            "p2bgisl" => { validate_finite_parameter("p2bgisl", value)?; self.params.p1310 = value; self.mark_param_given(1310); self.recompute_instance_static(); Ok(()) }
            "cgisl" => { validate_finite_parameter("cgisl", value)?; self.params.p1311 = value; self.mark_param_given(1311); self.recompute_instance_static(); Ok(()) }
            "lcgisl" => { validate_finite_parameter("lcgisl", value)?; self.params.p1312 = value; self.mark_param_given(1312); self.recompute_instance_static(); Ok(()) }
            "ncgisl" => { validate_finite_parameter("ncgisl", value)?; self.params.p1313 = value; self.mark_param_given(1313); self.recompute_instance_static(); Ok(()) }
            "pcgisl" => { validate_finite_parameter("pcgisl", value)?; self.params.p1314 = value; self.mark_param_given(1314); self.recompute_instance_static(); Ok(()) }
            "wcgisl" => { validate_finite_parameter("wcgisl", value)?; self.params.p1315 = value; self.mark_param_given(1315); self.recompute_instance_static(); Ok(()) }
            "p2cgisl" => { validate_finite_parameter("p2cgisl", value)?; self.params.p1316 = value; self.mark_param_given(1316); self.recompute_instance_static(); Ok(()) }
            "egisl" => { validate_finite_parameter("egisl", value)?; self.params.p1317 = value; self.mark_param_given(1317); self.recompute_instance_static(); Ok(()) }
            "legisl" => { validate_finite_parameter("legisl", value)?; self.params.p1318 = value; self.mark_param_given(1318); self.recompute_instance_static(); Ok(()) }
            "negisl" => { validate_finite_parameter("negisl", value)?; self.params.p1319 = value; self.mark_param_given(1319); self.recompute_instance_static(); Ok(()) }
            "pegisl" => { validate_finite_parameter("pegisl", value)?; self.params.p1320 = value; self.mark_param_given(1320); self.recompute_instance_static(); Ok(()) }
            "wegisl" => { validate_finite_parameter("wegisl", value)?; self.params.p1321 = value; self.mark_param_given(1321); self.recompute_instance_static(); Ok(()) }
            "p2egisl" => { validate_finite_parameter("p2egisl", value)?; self.params.p1322 = value; self.mark_param_given(1322); self.recompute_instance_static(); Ok(()) }
            "pgisl" => { validate_finite_parameter("pgisl", value)?; self.params.p1323 = value; self.mark_param_given(1323); self.recompute_instance_static(); Ok(()) }
            "lpgisl" => { validate_finite_parameter("lpgisl", value)?; self.params.p1324 = value; self.mark_param_given(1324); self.recompute_instance_static(); Ok(()) }
            "npgisl" => { validate_finite_parameter("npgisl", value)?; self.params.p1325 = value; self.mark_param_given(1325); self.recompute_instance_static(); Ok(()) }
            "ppgisl" => { validate_finite_parameter("ppgisl", value)?; self.params.p1326 = value; self.mark_param_given(1326); self.recompute_instance_static(); Ok(()) }
            "wpgisl" => { validate_finite_parameter("wpgisl", value)?; self.params.p1327 = value; self.mark_param_given(1327); self.recompute_instance_static(); Ok(()) }
            "p2pgisl" => { validate_finite_parameter("p2pgisl", value)?; self.params.p1328 = value; self.mark_param_given(1328); self.recompute_instance_static(); Ok(()) }
            "atatd" => { validate_finite_parameter("atatd", value)?; self.params.p1329 = value; self.mark_param_given(1329); self.recompute_instance_static(); Ok(()) }
            "latatd" => { validate_finite_parameter("latatd", value)?; self.params.p1330 = value; self.mark_param_given(1330); self.recompute_instance_static(); Ok(()) }
            "natatd" => { validate_finite_parameter("natatd", value)?; self.params.p1331 = value; self.mark_param_given(1331); self.recompute_instance_static(); Ok(()) }
            "patatd" => { validate_finite_parameter("patatd", value)?; self.params.p1332 = value; self.mark_param_given(1332); self.recompute_instance_static(); Ok(()) }
            "watatd" => { validate_finite_parameter("watatd", value)?; self.params.p1333 = value; self.mark_param_given(1333); self.recompute_instance_static(); Ok(()) }
            "p2atatd" => { validate_finite_parameter("p2atatd", value)?; self.params.p1334 = value; self.mark_param_given(1334); self.recompute_instance_static(); Ok(()) }
            "btatd" => { validate_finite_parameter("btatd", value)?; self.params.p1335 = value; self.mark_param_given(1335); self.recompute_instance_static(); Ok(()) }
            "lbtatd" => { validate_finite_parameter("lbtatd", value)?; self.params.p1336 = value; self.mark_param_given(1336); self.recompute_instance_static(); Ok(()) }
            "nbtatd" => { validate_finite_parameter("nbtatd", value)?; self.params.p1337 = value; self.mark_param_given(1337); self.recompute_instance_static(); Ok(()) }
            "pbtatd" => { validate_finite_parameter("pbtatd", value)?; self.params.p1338 = value; self.mark_param_given(1338); self.recompute_instance_static(); Ok(()) }
            "wbtatd" => { validate_finite_parameter("wbtatd", value)?; self.params.p1339 = value; self.mark_param_given(1339); self.recompute_instance_static(); Ok(()) }
            "p2btatd" => { validate_finite_parameter("p2btatd", value)?; self.params.p1340 = value; self.mark_param_given(1340); self.recompute_instance_static(); Ok(()) }
            "ctatd" => { validate_finite_parameter("ctatd", value)?; self.params.p1341 = value; self.mark_param_given(1341); self.recompute_instance_static(); Ok(()) }
            "lctatd" => { validate_finite_parameter("lctatd", value)?; self.params.p1342 = value; self.mark_param_given(1342); self.recompute_instance_static(); Ok(()) }
            "nctatd" => { validate_finite_parameter("nctatd", value)?; self.params.p1343 = value; self.mark_param_given(1343); self.recompute_instance_static(); Ok(()) }
            "pctatd" => { validate_finite_parameter("pctatd", value)?; self.params.p1344 = value; self.mark_param_given(1344); self.recompute_instance_static(); Ok(()) }
            "wctatd" => { validate_finite_parameter("wctatd", value)?; self.params.p1345 = value; self.mark_param_given(1345); self.recompute_instance_static(); Ok(()) }
            "p2ctatd" => { validate_finite_parameter("p2ctatd", value)?; self.params.p1346 = value; self.mark_param_given(1346); self.recompute_instance_static(); Ok(()) }
            "dtatd" => { validate_finite_parameter("dtatd", value)?; self.params.p1347 = value; self.mark_param_given(1347); self.recompute_instance_static(); Ok(()) }
            "ldtatd" => { validate_finite_parameter("ldtatd", value)?; self.params.p1348 = value; self.mark_param_given(1348); self.recompute_instance_static(); Ok(()) }
            "ndtatd" => { validate_finite_parameter("ndtatd", value)?; self.params.p1349 = value; self.mark_param_given(1349); self.recompute_instance_static(); Ok(()) }
            "pdtatd" => { validate_finite_parameter("pdtatd", value)?; self.params.p1350 = value; self.mark_param_given(1350); self.recompute_instance_static(); Ok(()) }
            "wdtatd" => { validate_finite_parameter("wdtatd", value)?; self.params.p1351 = value; self.mark_param_given(1351); self.recompute_instance_static(); Ok(()) }
            "p2dtatd" => { validate_finite_parameter("p2dtatd", value)?; self.params.p1352 = value; self.mark_param_given(1352); self.recompute_instance_static(); Ok(()) }
            "atats" => { validate_finite_parameter("atats", value)?; self.params.p1353 = value; self.mark_param_given(1353); self.recompute_instance_static(); Ok(()) }
            "latats" => { validate_finite_parameter("latats", value)?; self.params.p1354 = value; self.mark_param_given(1354); self.recompute_instance_static(); Ok(()) }
            "natats" => { validate_finite_parameter("natats", value)?; self.params.p1355 = value; self.mark_param_given(1355); self.recompute_instance_static(); Ok(()) }
            "patats" => { validate_finite_parameter("patats", value)?; self.params.p1356 = value; self.mark_param_given(1356); self.recompute_instance_static(); Ok(()) }
            "watats" => { validate_finite_parameter("watats", value)?; self.params.p1357 = value; self.mark_param_given(1357); self.recompute_instance_static(); Ok(()) }
            "p2atats" => { validate_finite_parameter("p2atats", value)?; self.params.p1358 = value; self.mark_param_given(1358); self.recompute_instance_static(); Ok(()) }
            "btats" => { validate_finite_parameter("btats", value)?; self.params.p1359 = value; self.mark_param_given(1359); self.recompute_instance_static(); Ok(()) }
            "lbtats" => { validate_finite_parameter("lbtats", value)?; self.params.p1360 = value; self.mark_param_given(1360); self.recompute_instance_static(); Ok(()) }
            "nbtats" => { validate_finite_parameter("nbtats", value)?; self.params.p1361 = value; self.mark_param_given(1361); self.recompute_instance_static(); Ok(()) }
            "pbtats" => { validate_finite_parameter("pbtats", value)?; self.params.p1362 = value; self.mark_param_given(1362); self.recompute_instance_static(); Ok(()) }
            "wbtats" => { validate_finite_parameter("wbtats", value)?; self.params.p1363 = value; self.mark_param_given(1363); self.recompute_instance_static(); Ok(()) }
            "p2btats" => { validate_finite_parameter("p2btats", value)?; self.params.p1364 = value; self.mark_param_given(1364); self.recompute_instance_static(); Ok(()) }
            "ctats" => { validate_finite_parameter("ctats", value)?; self.params.p1365 = value; self.mark_param_given(1365); self.recompute_instance_static(); Ok(()) }
            "lctats" => { validate_finite_parameter("lctats", value)?; self.params.p1366 = value; self.mark_param_given(1366); self.recompute_instance_static(); Ok(()) }
            "nctats" => { validate_finite_parameter("nctats", value)?; self.params.p1367 = value; self.mark_param_given(1367); self.recompute_instance_static(); Ok(()) }
            "pctats" => { validate_finite_parameter("pctats", value)?; self.params.p1368 = value; self.mark_param_given(1368); self.recompute_instance_static(); Ok(()) }
            "wctats" => { validate_finite_parameter("wctats", value)?; self.params.p1369 = value; self.mark_param_given(1369); self.recompute_instance_static(); Ok(()) }
            "p2ctats" => { validate_finite_parameter("p2ctats", value)?; self.params.p1370 = value; self.mark_param_given(1370); self.recompute_instance_static(); Ok(()) }
            "dtats" => { validate_finite_parameter("dtats", value)?; self.params.p1371 = value; self.mark_param_given(1371); self.recompute_instance_static(); Ok(()) }
            "ldtats" => { validate_finite_parameter("ldtats", value)?; self.params.p1372 = value; self.mark_param_given(1372); self.recompute_instance_static(); Ok(()) }
            "ndtats" => { validate_finite_parameter("ndtats", value)?; self.params.p1373 = value; self.mark_param_given(1373); self.recompute_instance_static(); Ok(()) }
            "pdtats" => { validate_finite_parameter("pdtats", value)?; self.params.p1374 = value; self.mark_param_given(1374); self.recompute_instance_static(); Ok(()) }
            "wdtats" => { validate_finite_parameter("wdtats", value)?; self.params.p1375 = value; self.mark_param_given(1375); self.recompute_instance_static(); Ok(()) }
            "p2dtats" => { validate_finite_parameter("p2dtats", value)?; self.params.p1376 = value; self.mark_param_given(1376); self.recompute_instance_static(); Ok(()) }
            "agidlb" => { validate_finite_parameter("agidlb", value)?; self.params.p1377 = value; self.mark_param_given(1377); self.recompute_instance_static(); Ok(()) }
            "lagidlb" => { validate_finite_parameter("lagidlb", value)?; self.params.p1378 = value; self.mark_param_given(1378); self.recompute_instance_static(); Ok(()) }
            "nagidlb" => { validate_finite_parameter("nagidlb", value)?; self.params.p1379 = value; self.mark_param_given(1379); self.recompute_instance_static(); Ok(()) }
            "pagidlb" => { validate_finite_parameter("pagidlb", value)?; self.params.p1380 = value; self.mark_param_given(1380); self.recompute_instance_static(); Ok(()) }
            "wagidlb" => { validate_finite_parameter("wagidlb", value)?; self.params.p1381 = value; self.mark_param_given(1381); self.recompute_instance_static(); Ok(()) }
            "p2agidlb" => { validate_finite_parameter("p2agidlb", value)?; self.params.p1382 = value; self.mark_param_given(1382); self.recompute_instance_static(); Ok(()) }
            "bgidlb" => { validate_finite_parameter("bgidlb", value)?; self.params.p1383 = value; self.mark_param_given(1383); self.recompute_instance_static(); Ok(()) }
            "lbgidlb" => { validate_finite_parameter("lbgidlb", value)?; self.params.p1384 = value; self.mark_param_given(1384); self.recompute_instance_static(); Ok(()) }
            "nbgidlb" => { validate_finite_parameter("nbgidlb", value)?; self.params.p1385 = value; self.mark_param_given(1385); self.recompute_instance_static(); Ok(()) }
            "pbgidlb" => { validate_finite_parameter("pbgidlb", value)?; self.params.p1386 = value; self.mark_param_given(1386); self.recompute_instance_static(); Ok(()) }
            "wbgidlb" => { validate_finite_parameter("wbgidlb", value)?; self.params.p1387 = value; self.mark_param_given(1387); self.recompute_instance_static(); Ok(()) }
            "p2bgidlb" => { validate_finite_parameter("p2bgidlb", value)?; self.params.p1388 = value; self.mark_param_given(1388); self.recompute_instance_static(); Ok(()) }
            "cgidlb" => { validate_finite_parameter("cgidlb", value)?; self.params.p1389 = value; self.mark_param_given(1389); self.recompute_instance_static(); Ok(()) }
            "lcgidlb" => { validate_finite_parameter("lcgidlb", value)?; self.params.p1390 = value; self.mark_param_given(1390); self.recompute_instance_static(); Ok(()) }
            "ncgidlb" => { validate_finite_parameter("ncgidlb", value)?; self.params.p1391 = value; self.mark_param_given(1391); self.recompute_instance_static(); Ok(()) }
            "pcgidlb" => { validate_finite_parameter("pcgidlb", value)?; self.params.p1392 = value; self.mark_param_given(1392); self.recompute_instance_static(); Ok(()) }
            "wcgidlb" => { validate_finite_parameter("wcgidlb", value)?; self.params.p1393 = value; self.mark_param_given(1393); self.recompute_instance_static(); Ok(()) }
            "p2cgidlb" => { validate_finite_parameter("p2cgidlb", value)?; self.params.p1394 = value; self.mark_param_given(1394); self.recompute_instance_static(); Ok(()) }
            "egidlb" => { validate_finite_parameter("egidlb", value)?; self.params.p1395 = value; self.mark_param_given(1395); self.recompute_instance_static(); Ok(()) }
            "legidlb" => { validate_finite_parameter("legidlb", value)?; self.params.p1396 = value; self.mark_param_given(1396); self.recompute_instance_static(); Ok(()) }
            "negidlb" => { validate_finite_parameter("negidlb", value)?; self.params.p1397 = value; self.mark_param_given(1397); self.recompute_instance_static(); Ok(()) }
            "pegidlb" => { validate_finite_parameter("pegidlb", value)?; self.params.p1398 = value; self.mark_param_given(1398); self.recompute_instance_static(); Ok(()) }
            "wegidlb" => { validate_finite_parameter("wegidlb", value)?; self.params.p1399 = value; self.mark_param_given(1399); self.recompute_instance_static(); Ok(()) }
            "p2egidlb" => { validate_finite_parameter("p2egidlb", value)?; self.params.p1400 = value; self.mark_param_given(1400); self.recompute_instance_static(); Ok(()) }
            "pgidlb" => { validate_finite_parameter("pgidlb", value)?; self.params.p1401 = value; self.mark_param_given(1401); self.recompute_instance_static(); Ok(()) }
            "lpgidlb" => { validate_finite_parameter("lpgidlb", value)?; self.params.p1402 = value; self.mark_param_given(1402); self.recompute_instance_static(); Ok(()) }
            "npgidlb" => { validate_finite_parameter("npgidlb", value)?; self.params.p1403 = value; self.mark_param_given(1403); self.recompute_instance_static(); Ok(()) }
            "ppgidlb" => { validate_finite_parameter("ppgidlb", value)?; self.params.p1404 = value; self.mark_param_given(1404); self.recompute_instance_static(); Ok(()) }
            "wpgidlb" => { validate_finite_parameter("wpgidlb", value)?; self.params.p1405 = value; self.mark_param_given(1405); self.recompute_instance_static(); Ok(()) }
            "p2pgidlb" => { validate_finite_parameter("p2pgidlb", value)?; self.params.p1406 = value; self.mark_param_given(1406); self.recompute_instance_static(); Ok(()) }
            "agislb" => { validate_finite_parameter("agislb", value)?; self.params.p1407 = value; self.mark_param_given(1407); self.recompute_instance_static(); Ok(()) }
            "lagislb" => { validate_finite_parameter("lagislb", value)?; self.params.p1408 = value; self.mark_param_given(1408); self.recompute_instance_static(); Ok(()) }
            "nagislb" => { validate_finite_parameter("nagislb", value)?; self.params.p1409 = value; self.mark_param_given(1409); self.recompute_instance_static(); Ok(()) }
            "pagislb" => { validate_finite_parameter("pagislb", value)?; self.params.p1410 = value; self.mark_param_given(1410); self.recompute_instance_static(); Ok(()) }
            "wagislb" => { validate_finite_parameter("wagislb", value)?; self.params.p1411 = value; self.mark_param_given(1411); self.recompute_instance_static(); Ok(()) }
            "p2agislb" => { validate_finite_parameter("p2agislb", value)?; self.params.p1412 = value; self.mark_param_given(1412); self.recompute_instance_static(); Ok(()) }
            "bgislb" => { validate_finite_parameter("bgislb", value)?; self.params.p1413 = value; self.mark_param_given(1413); self.recompute_instance_static(); Ok(()) }
            "lbgislb" => { validate_finite_parameter("lbgislb", value)?; self.params.p1414 = value; self.mark_param_given(1414); self.recompute_instance_static(); Ok(()) }
            "nbgislb" => { validate_finite_parameter("nbgislb", value)?; self.params.p1415 = value; self.mark_param_given(1415); self.recompute_instance_static(); Ok(()) }
            "pbgislb" => { validate_finite_parameter("pbgislb", value)?; self.params.p1416 = value; self.mark_param_given(1416); self.recompute_instance_static(); Ok(()) }
            "wbgislb" => { validate_finite_parameter("wbgislb", value)?; self.params.p1417 = value; self.mark_param_given(1417); self.recompute_instance_static(); Ok(()) }
            "p2bgislb" => { validate_finite_parameter("p2bgislb", value)?; self.params.p1418 = value; self.mark_param_given(1418); self.recompute_instance_static(); Ok(()) }
            "cgislb" => { validate_finite_parameter("cgislb", value)?; self.params.p1419 = value; self.mark_param_given(1419); self.recompute_instance_static(); Ok(()) }
            "lcgislb" => { validate_finite_parameter("lcgislb", value)?; self.params.p1420 = value; self.mark_param_given(1420); self.recompute_instance_static(); Ok(()) }
            "ncgislb" => { validate_finite_parameter("ncgislb", value)?; self.params.p1421 = value; self.mark_param_given(1421); self.recompute_instance_static(); Ok(()) }
            "pcgislb" => { validate_finite_parameter("pcgislb", value)?; self.params.p1422 = value; self.mark_param_given(1422); self.recompute_instance_static(); Ok(()) }
            "wcgislb" => { validate_finite_parameter("wcgislb", value)?; self.params.p1423 = value; self.mark_param_given(1423); self.recompute_instance_static(); Ok(()) }
            "p2cgislb" => { validate_finite_parameter("p2cgislb", value)?; self.params.p1424 = value; self.mark_param_given(1424); self.recompute_instance_static(); Ok(()) }
            "egislb" => { validate_finite_parameter("egislb", value)?; self.params.p1425 = value; self.mark_param_given(1425); self.recompute_instance_static(); Ok(()) }
            "legislb" => { validate_finite_parameter("legislb", value)?; self.params.p1426 = value; self.mark_param_given(1426); self.recompute_instance_static(); Ok(()) }
            "negislb" => { validate_finite_parameter("negislb", value)?; self.params.p1427 = value; self.mark_param_given(1427); self.recompute_instance_static(); Ok(()) }
            "pegislb" => { validate_finite_parameter("pegislb", value)?; self.params.p1428 = value; self.mark_param_given(1428); self.recompute_instance_static(); Ok(()) }
            "wegislb" => { validate_finite_parameter("wegislb", value)?; self.params.p1429 = value; self.mark_param_given(1429); self.recompute_instance_static(); Ok(()) }
            "p2egislb" => { validate_finite_parameter("p2egislb", value)?; self.params.p1430 = value; self.mark_param_given(1430); self.recompute_instance_static(); Ok(()) }
            "pgislb" => { validate_finite_parameter("pgislb", value)?; self.params.p1431 = value; self.mark_param_given(1431); self.recompute_instance_static(); Ok(()) }
            "lpgislb" => { validate_finite_parameter("lpgislb", value)?; self.params.p1432 = value; self.mark_param_given(1432); self.recompute_instance_static(); Ok(()) }
            "npgislb" => { validate_finite_parameter("npgislb", value)?; self.params.p1433 = value; self.mark_param_given(1433); self.recompute_instance_static(); Ok(()) }
            "ppgislb" => { validate_finite_parameter("ppgislb", value)?; self.params.p1434 = value; self.mark_param_given(1434); self.recompute_instance_static(); Ok(()) }
            "wpgislb" => { validate_finite_parameter("wpgislb", value)?; self.params.p1435 = value; self.mark_param_given(1435); self.recompute_instance_static(); Ok(()) }
            "p2pgislb" => { validate_finite_parameter("p2pgislb", value)?; self.params.p1436 = value; self.mark_param_given(1436); self.recompute_instance_static(); Ok(()) }
            "alpha01" => { validate_finite_parameter("alpha01", value)?; self.params.p1437 = value; self.mark_param_given(1437); self.recompute_instance_static(); Ok(()) }
            "alpha11" => { validate_finite_parameter("alpha11", value)?; self.params.p1438 = value; self.mark_param_given(1438); self.recompute_instance_static(); Ok(()) }
            "alphaii01" => { validate_finite_parameter("alphaii01", value)?; self.params.p1439 = value; self.mark_param_given(1439); self.recompute_instance_static(); Ok(()) }
            "alphaii11" => { validate_finite_parameter("alphaii11", value)?; self.params.p1440 = value; self.mark_param_given(1440); self.recompute_instance_static(); Ok(()) }
            "iimod2clamp1" => { validate_parameter("iimod2clamp1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1441 = value; self.mark_param_given(1441); self.recompute_instance_static(); Ok(()) }
            "iimod2clamp2" => { validate_parameter("iimod2clamp2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1442 = value; self.mark_param_given(1442); self.recompute_instance_static(); Ok(()) }
            "iimod2clamp3" => { validate_parameter("iimod2clamp3", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1443 = value; self.mark_param_given(1443); self.recompute_instance_static(); Ok(()) }
            "alpha0" => { validate_finite_parameter("alpha0", value)?; self.params.p1444 = value; self.mark_param_given(1444); self.recompute_instance_static(); Ok(()) }
            "lalpha0" => { validate_finite_parameter("lalpha0", value)?; self.params.p1445 = value; self.mark_param_given(1445); self.recompute_instance_static(); Ok(()) }
            "nalpha0" => { validate_finite_parameter("nalpha0", value)?; self.params.p1446 = value; self.mark_param_given(1446); self.recompute_instance_static(); Ok(()) }
            "palpha0" => { validate_finite_parameter("palpha0", value)?; self.params.p1447 = value; self.mark_param_given(1447); self.recompute_instance_static(); Ok(()) }
            "walpha0" => { validate_finite_parameter("walpha0", value)?; self.params.p1448 = value; self.mark_param_given(1448); self.recompute_instance_static(); Ok(()) }
            "p2alpha0" => { validate_finite_parameter("p2alpha0", value)?; self.params.p1449 = value; self.mark_param_given(1449); self.recompute_instance_static(); Ok(()) }
            "alpha1" => { validate_finite_parameter("alpha1", value)?; self.params.p1450 = value; self.mark_param_given(1450); self.recompute_instance_static(); Ok(()) }
            "lalpha1" => { validate_finite_parameter("lalpha1", value)?; self.params.p1451 = value; self.mark_param_given(1451); self.recompute_instance_static(); Ok(()) }
            "nalpha1" => { validate_finite_parameter("nalpha1", value)?; self.params.p1452 = value; self.mark_param_given(1452); self.recompute_instance_static(); Ok(()) }
            "palpha1" => { validate_finite_parameter("palpha1", value)?; self.params.p1453 = value; self.mark_param_given(1453); self.recompute_instance_static(); Ok(()) }
            "walpha1" => { validate_finite_parameter("walpha1", value)?; self.params.p1454 = value; self.mark_param_given(1454); self.recompute_instance_static(); Ok(()) }
            "p2alpha1" => { validate_finite_parameter("p2alpha1", value)?; self.params.p1455 = value; self.mark_param_given(1455); self.recompute_instance_static(); Ok(()) }
            "beta0" => { validate_finite_parameter("beta0", value)?; self.params.p1456 = value; self.mark_param_given(1456); self.recompute_instance_static(); Ok(()) }
            "lbeta0" => { validate_finite_parameter("lbeta0", value)?; self.params.p1457 = value; self.mark_param_given(1457); self.recompute_instance_static(); Ok(()) }
            "nbeta0" => { validate_finite_parameter("nbeta0", value)?; self.params.p1458 = value; self.mark_param_given(1458); self.recompute_instance_static(); Ok(()) }
            "pbeta0" => { validate_finite_parameter("pbeta0", value)?; self.params.p1459 = value; self.mark_param_given(1459); self.recompute_instance_static(); Ok(()) }
            "wbeta0" => { validate_finite_parameter("wbeta0", value)?; self.params.p1460 = value; self.mark_param_given(1460); self.recompute_instance_static(); Ok(()) }
            "p2beta0" => { validate_finite_parameter("p2beta0", value)?; self.params.p1461 = value; self.mark_param_given(1461); self.recompute_instance_static(); Ok(()) }
            "alphaii0" => { validate_finite_parameter("alphaii0", value)?; self.params.p1462 = value; self.mark_param_given(1462); self.recompute_instance_static(); Ok(()) }
            "lalphaii0" => { validate_finite_parameter("lalphaii0", value)?; self.params.p1463 = value; self.mark_param_given(1463); self.recompute_instance_static(); Ok(()) }
            "nalphaii0" => { validate_finite_parameter("nalphaii0", value)?; self.params.p1464 = value; self.mark_param_given(1464); self.recompute_instance_static(); Ok(()) }
            "palphaii0" => { validate_finite_parameter("palphaii0", value)?; self.params.p1465 = value; self.mark_param_given(1465); self.recompute_instance_static(); Ok(()) }
            "walphaii0" => { validate_finite_parameter("walphaii0", value)?; self.params.p1466 = value; self.mark_param_given(1466); self.recompute_instance_static(); Ok(()) }
            "p2alphaii0" => { validate_finite_parameter("p2alphaii0", value)?; self.params.p1467 = value; self.mark_param_given(1467); self.recompute_instance_static(); Ok(()) }
            "alphaii1" => { validate_finite_parameter("alphaii1", value)?; self.params.p1468 = value; self.mark_param_given(1468); self.recompute_instance_static(); Ok(()) }
            "lalphaii1" => { validate_finite_parameter("lalphaii1", value)?; self.params.p1469 = value; self.mark_param_given(1469); self.recompute_instance_static(); Ok(()) }
            "nalphaii1" => { validate_finite_parameter("nalphaii1", value)?; self.params.p1470 = value; self.mark_param_given(1470); self.recompute_instance_static(); Ok(()) }
            "palphaii1" => { validate_finite_parameter("palphaii1", value)?; self.params.p1471 = value; self.mark_param_given(1471); self.recompute_instance_static(); Ok(()) }
            "walphaii1" => { validate_finite_parameter("walphaii1", value)?; self.params.p1472 = value; self.mark_param_given(1472); self.recompute_instance_static(); Ok(()) }
            "p2alphaii1" => { validate_finite_parameter("p2alphaii1", value)?; self.params.p1473 = value; self.mark_param_given(1473); self.recompute_instance_static(); Ok(()) }
            "betaii0" => { validate_finite_parameter("betaii0", value)?; self.params.p1474 = value; self.mark_param_given(1474); self.recompute_instance_static(); Ok(()) }
            "lbetaii0" => { validate_finite_parameter("lbetaii0", value)?; self.params.p1475 = value; self.mark_param_given(1475); self.recompute_instance_static(); Ok(()) }
            "nbetaii0" => { validate_finite_parameter("nbetaii0", value)?; self.params.p1476 = value; self.mark_param_given(1476); self.recompute_instance_static(); Ok(()) }
            "pbetaii0" => { validate_finite_parameter("pbetaii0", value)?; self.params.p1477 = value; self.mark_param_given(1477); self.recompute_instance_static(); Ok(()) }
            "wbetaii0" => { validate_finite_parameter("wbetaii0", value)?; self.params.p1478 = value; self.mark_param_given(1478); self.recompute_instance_static(); Ok(()) }
            "p2betaii0" => { validate_finite_parameter("p2betaii0", value)?; self.params.p1479 = value; self.mark_param_given(1479); self.recompute_instance_static(); Ok(()) }
            "betaii1" => { validate_finite_parameter("betaii1", value)?; self.params.p1480 = value; self.mark_param_given(1480); self.recompute_instance_static(); Ok(()) }
            "lbetaii1" => { validate_finite_parameter("lbetaii1", value)?; self.params.p1481 = value; self.mark_param_given(1481); self.recompute_instance_static(); Ok(()) }
            "nbetaii1" => { validate_finite_parameter("nbetaii1", value)?; self.params.p1482 = value; self.mark_param_given(1482); self.recompute_instance_static(); Ok(()) }
            "pbetaii1" => { validate_finite_parameter("pbetaii1", value)?; self.params.p1483 = value; self.mark_param_given(1483); self.recompute_instance_static(); Ok(()) }
            "wbetaii1" => { validate_finite_parameter("wbetaii1", value)?; self.params.p1484 = value; self.mark_param_given(1484); self.recompute_instance_static(); Ok(()) }
            "p2betaii1" => { validate_finite_parameter("p2betaii1", value)?; self.params.p1485 = value; self.mark_param_given(1485); self.recompute_instance_static(); Ok(()) }
            "betaii2" => { validate_finite_parameter("betaii2", value)?; self.params.p1486 = value; self.mark_param_given(1486); self.recompute_instance_static(); Ok(()) }
            "lbetaii2" => { validate_finite_parameter("lbetaii2", value)?; self.params.p1487 = value; self.mark_param_given(1487); self.recompute_instance_static(); Ok(()) }
            "nbetaii2" => { validate_finite_parameter("nbetaii2", value)?; self.params.p1488 = value; self.mark_param_given(1488); self.recompute_instance_static(); Ok(()) }
            "pbetaii2" => { validate_finite_parameter("pbetaii2", value)?; self.params.p1489 = value; self.mark_param_given(1489); self.recompute_instance_static(); Ok(()) }
            "wbetaii2" => { validate_finite_parameter("wbetaii2", value)?; self.params.p1490 = value; self.mark_param_given(1490); self.recompute_instance_static(); Ok(()) }
            "p2betaii2" => { validate_finite_parameter("p2betaii2", value)?; self.params.p1491 = value; self.mark_param_given(1491); self.recompute_instance_static(); Ok(()) }
            "esatii" => { validate_finite_parameter("esatii", value)?; self.params.p1492 = value; self.mark_param_given(1492); self.recompute_instance_static(); Ok(()) }
            "lesatii" => { validate_finite_parameter("lesatii", value)?; self.params.p1493 = value; self.mark_param_given(1493); self.recompute_instance_static(); Ok(()) }
            "nesatii" => { validate_finite_parameter("nesatii", value)?; self.params.p1494 = value; self.mark_param_given(1494); self.recompute_instance_static(); Ok(()) }
            "pesatii" => { validate_finite_parameter("pesatii", value)?; self.params.p1495 = value; self.mark_param_given(1495); self.recompute_instance_static(); Ok(()) }
            "wesatii" => { validate_finite_parameter("wesatii", value)?; self.params.p1496 = value; self.mark_param_given(1496); self.recompute_instance_static(); Ok(()) }
            "p2esatii" => { validate_finite_parameter("p2esatii", value)?; self.params.p1497 = value; self.mark_param_given(1497); self.recompute_instance_static(); Ok(()) }
            "lii" => { validate_finite_parameter("lii", value)?; self.params.p1498 = value; self.mark_param_given(1498); self.recompute_instance_static(); Ok(()) }
            "llii" => { validate_finite_parameter("llii", value)?; self.params.p1499 = value; self.mark_param_given(1499); self.recompute_instance_static(); Ok(()) }
            "nlii" => { validate_finite_parameter("nlii", value)?; self.params.p1500 = value; self.mark_param_given(1500); self.recompute_instance_static(); Ok(()) }
            "plii" => { validate_finite_parameter("plii", value)?; self.params.p1501 = value; self.mark_param_given(1501); self.recompute_instance_static(); Ok(()) }
            "wlii" => { validate_finite_parameter("wlii", value)?; self.params.p1502 = value; self.mark_param_given(1502); self.recompute_instance_static(); Ok(()) }
            "p2lii" => { validate_finite_parameter("p2lii", value)?; self.params.p1503 = value; self.mark_param_given(1503); self.recompute_instance_static(); Ok(()) }
            "sii0" => { validate_finite_parameter("sii0", value)?; self.params.p1504 = value; self.mark_param_given(1504); self.recompute_instance_static(); Ok(()) }
            "lsii0" => { validate_finite_parameter("lsii0", value)?; self.params.p1505 = value; self.mark_param_given(1505); self.recompute_instance_static(); Ok(()) }
            "nsii0" => { validate_finite_parameter("nsii0", value)?; self.params.p1506 = value; self.mark_param_given(1506); self.recompute_instance_static(); Ok(()) }
            "psii0" => { validate_finite_parameter("psii0", value)?; self.params.p1507 = value; self.mark_param_given(1507); self.recompute_instance_static(); Ok(()) }
            "wsii0" => { validate_finite_parameter("wsii0", value)?; self.params.p1508 = value; self.mark_param_given(1508); self.recompute_instance_static(); Ok(()) }
            "p2sii0" => { validate_finite_parameter("p2sii0", value)?; self.params.p1509 = value; self.mark_param_given(1509); self.recompute_instance_static(); Ok(()) }
            "sii1" => { validate_finite_parameter("sii1", value)?; self.params.p1510 = value; self.mark_param_given(1510); self.recompute_instance_static(); Ok(()) }
            "lsii1" => { validate_finite_parameter("lsii1", value)?; self.params.p1511 = value; self.mark_param_given(1511); self.recompute_instance_static(); Ok(()) }
            "nsii1" => { validate_finite_parameter("nsii1", value)?; self.params.p1512 = value; self.mark_param_given(1512); self.recompute_instance_static(); Ok(()) }
            "psii1" => { validate_finite_parameter("psii1", value)?; self.params.p1513 = value; self.mark_param_given(1513); self.recompute_instance_static(); Ok(()) }
            "wsii1" => { validate_finite_parameter("wsii1", value)?; self.params.p1514 = value; self.mark_param_given(1514); self.recompute_instance_static(); Ok(()) }
            "p2sii1" => { validate_finite_parameter("p2sii1", value)?; self.params.p1515 = value; self.mark_param_given(1515); self.recompute_instance_static(); Ok(()) }
            "sii2" => { validate_finite_parameter("sii2", value)?; self.params.p1516 = value; self.mark_param_given(1516); self.recompute_instance_static(); Ok(()) }
            "lsii2" => { validate_finite_parameter("lsii2", value)?; self.params.p1517 = value; self.mark_param_given(1517); self.recompute_instance_static(); Ok(()) }
            "nsii2" => { validate_finite_parameter("nsii2", value)?; self.params.p1518 = value; self.mark_param_given(1518); self.recompute_instance_static(); Ok(()) }
            "psii2" => { validate_finite_parameter("psii2", value)?; self.params.p1519 = value; self.mark_param_given(1519); self.recompute_instance_static(); Ok(()) }
            "wsii2" => { validate_finite_parameter("wsii2", value)?; self.params.p1520 = value; self.mark_param_given(1520); self.recompute_instance_static(); Ok(()) }
            "p2sii2" => { validate_finite_parameter("p2sii2", value)?; self.params.p1521 = value; self.mark_param_given(1521); self.recompute_instance_static(); Ok(()) }
            "siid" => { validate_finite_parameter("siid", value)?; self.params.p1522 = value; self.mark_param_given(1522); self.recompute_instance_static(); Ok(()) }
            "lsiid" => { validate_finite_parameter("lsiid", value)?; self.params.p1523 = value; self.mark_param_given(1523); self.recompute_instance_static(); Ok(()) }
            "nsiid" => { validate_finite_parameter("nsiid", value)?; self.params.p1524 = value; self.mark_param_given(1524); self.recompute_instance_static(); Ok(()) }
            "psiid" => { validate_finite_parameter("psiid", value)?; self.params.p1525 = value; self.mark_param_given(1525); self.recompute_instance_static(); Ok(()) }
            "wsiid" => { validate_finite_parameter("wsiid", value)?; self.params.p1526 = value; self.mark_param_given(1526); self.recompute_instance_static(); Ok(()) }
            "p2siid" => { validate_finite_parameter("p2siid", value)?; self.params.p1527 = value; self.mark_param_given(1527); self.recompute_instance_static(); Ok(()) }
            "eotacc" => { validate_parameter("eotacc", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p1528 = value; self.mark_param_given(1528); self.recompute_instance_static(); Ok(()) }
            "delvfbacc" => { validate_finite_parameter("delvfbacc", value)?; self.params.p1529 = value; self.mark_param_given(1529); self.recompute_instance_static(); Ok(()) }
            "cfs" => { validate_finite_parameter("cfs", value)?; self.params.p1530 = value; self.mark_param_given(1530); self.recompute_instance_static(); Ok(()) }
            "lcfs" => { validate_finite_parameter("lcfs", value)?; self.params.p1531 = value; self.mark_param_given(1531); self.recompute_instance_static(); Ok(()) }
            "ncfs" => { validate_finite_parameter("ncfs", value)?; self.params.p1532 = value; self.mark_param_given(1532); self.recompute_instance_static(); Ok(()) }
            "pcfs" => { validate_finite_parameter("pcfs", value)?; self.params.p1533 = value; self.mark_param_given(1533); self.recompute_instance_static(); Ok(()) }
            "wcfs" => { validate_finite_parameter("wcfs", value)?; self.params.p1534 = value; self.mark_param_given(1534); self.recompute_instance_static(); Ok(()) }
            "p2cfs" => { validate_finite_parameter("p2cfs", value)?; self.params.p1535 = value; self.mark_param_given(1535); self.recompute_instance_static(); Ok(()) }
            "cfd" => { validate_finite_parameter("cfd", value)?; self.params.p1536 = value; self.mark_param_given(1536); self.recompute_instance_static(); Ok(()) }
            "lcfd" => { validate_finite_parameter("lcfd", value)?; self.params.p1537 = value; self.mark_param_given(1537); self.recompute_instance_static(); Ok(()) }
            "ncfd" => { validate_finite_parameter("ncfd", value)?; self.params.p1538 = value; self.mark_param_given(1538); self.recompute_instance_static(); Ok(()) }
            "pcfd" => { validate_finite_parameter("pcfd", value)?; self.params.p1539 = value; self.mark_param_given(1539); self.recompute_instance_static(); Ok(()) }
            "wcfd" => { validate_finite_parameter("wcfd", value)?; self.params.p1540 = value; self.mark_param_given(1540); self.recompute_instance_static(); Ok(()) }
            "p2cfd" => { validate_finite_parameter("p2cfd", value)?; self.params.p1541 = value; self.mark_param_given(1541); self.recompute_instance_static(); Ok(()) }
            "cgso" => { validate_parameter("cgso", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1542 = value; self.mark_param_given(1542); self.recompute_instance_static(); Ok(()) }
            "cgdo" => { validate_parameter("cgdo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1543 = value; self.mark_param_given(1543); self.recompute_instance_static(); Ok(()) }
            "cgbo" => { validate_parameter("cgbo", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1544 = value; self.mark_param_given(1544); self.recompute_instance_static(); Ok(()) }
            "cgbn" => { validate_parameter("cgbn", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1545 = value; self.mark_param_given(1545); self.recompute_instance_static(); Ok(()) }
            "cgbw" => { validate_parameter("cgbw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1546 = value; self.mark_param_given(1546); self.recompute_instance_static(); Ok(()) }
            "cgsl" => { validate_finite_parameter("cgsl", value)?; self.params.p1547 = value; self.mark_param_given(1547); self.recompute_instance_static(); Ok(()) }
            "lcgsl" => { validate_finite_parameter("lcgsl", value)?; self.params.p1548 = value; self.mark_param_given(1548); self.recompute_instance_static(); Ok(()) }
            "ncgsl" => { validate_finite_parameter("ncgsl", value)?; self.params.p1549 = value; self.mark_param_given(1549); self.recompute_instance_static(); Ok(()) }
            "pcgsl" => { validate_finite_parameter("pcgsl", value)?; self.params.p1550 = value; self.mark_param_given(1550); self.recompute_instance_static(); Ok(()) }
            "wcgsl" => { validate_finite_parameter("wcgsl", value)?; self.params.p1551 = value; self.mark_param_given(1551); self.recompute_instance_static(); Ok(()) }
            "p2cgsl" => { validate_finite_parameter("p2cgsl", value)?; self.params.p1552 = value; self.mark_param_given(1552); self.recompute_instance_static(); Ok(()) }
            "cgdl" => { validate_finite_parameter("cgdl", value)?; self.params.p1553 = value; self.mark_param_given(1553); self.recompute_instance_static(); Ok(()) }
            "lcgdl" => { validate_finite_parameter("lcgdl", value)?; self.params.p1554 = value; self.mark_param_given(1554); self.recompute_instance_static(); Ok(()) }
            "ncgdl" => { validate_finite_parameter("ncgdl", value)?; self.params.p1555 = value; self.mark_param_given(1555); self.recompute_instance_static(); Ok(()) }
            "pcgdl" => { validate_finite_parameter("pcgdl", value)?; self.params.p1556 = value; self.mark_param_given(1556); self.recompute_instance_static(); Ok(()) }
            "wcgdl" => { validate_finite_parameter("wcgdl", value)?; self.params.p1557 = value; self.mark_param_given(1557); self.recompute_instance_static(); Ok(()) }
            "p2cgdl" => { validate_finite_parameter("p2cgdl", value)?; self.params.p1558 = value; self.mark_param_given(1558); self.recompute_instance_static(); Ok(()) }
            "cgbl" => { validate_finite_parameter("cgbl", value)?; self.params.p1559 = value; self.mark_param_given(1559); self.recompute_instance_static(); Ok(()) }
            "lcgbl" => { validate_finite_parameter("lcgbl", value)?; self.params.p1560 = value; self.mark_param_given(1560); self.recompute_instance_static(); Ok(()) }
            "ncgbl" => { validate_finite_parameter("ncgbl", value)?; self.params.p1561 = value; self.mark_param_given(1561); self.recompute_instance_static(); Ok(()) }
            "pcgbl" => { validate_finite_parameter("pcgbl", value)?; self.params.p1562 = value; self.mark_param_given(1562); self.recompute_instance_static(); Ok(()) }
            "wcgbl" => { validate_finite_parameter("wcgbl", value)?; self.params.p1563 = value; self.mark_param_given(1563); self.recompute_instance_static(); Ok(()) }
            "p2cgbl" => { validate_finite_parameter("p2cgbl", value)?; self.params.p1564 = value; self.mark_param_given(1564); self.recompute_instance_static(); Ok(()) }
            "ckappas" => { validate_finite_parameter("ckappas", value)?; self.params.p1565 = value; self.mark_param_given(1565); self.recompute_instance_static(); Ok(()) }
            "lckappas" => { validate_finite_parameter("lckappas", value)?; self.params.p1566 = value; self.mark_param_given(1566); self.recompute_instance_static(); Ok(()) }
            "nckappas" => { validate_finite_parameter("nckappas", value)?; self.params.p1567 = value; self.mark_param_given(1567); self.recompute_instance_static(); Ok(()) }
            "pckappas" => { validate_finite_parameter("pckappas", value)?; self.params.p1568 = value; self.mark_param_given(1568); self.recompute_instance_static(); Ok(()) }
            "wckappas" => { validate_finite_parameter("wckappas", value)?; self.params.p1569 = value; self.mark_param_given(1569); self.recompute_instance_static(); Ok(()) }
            "p2ckappas" => { validate_finite_parameter("p2ckappas", value)?; self.params.p1570 = value; self.mark_param_given(1570); self.recompute_instance_static(); Ok(()) }
            "ckappad" => { validate_finite_parameter("ckappad", value)?; self.params.p1571 = value; self.mark_param_given(1571); self.recompute_instance_static(); Ok(()) }
            "lckappad" => { validate_finite_parameter("lckappad", value)?; self.params.p1572 = value; self.mark_param_given(1572); self.recompute_instance_static(); Ok(()) }
            "nckappad" => { validate_finite_parameter("nckappad", value)?; self.params.p1573 = value; self.mark_param_given(1573); self.recompute_instance_static(); Ok(()) }
            "pckappad" => { validate_finite_parameter("pckappad", value)?; self.params.p1574 = value; self.mark_param_given(1574); self.recompute_instance_static(); Ok(()) }
            "wckappad" => { validate_finite_parameter("wckappad", value)?; self.params.p1575 = value; self.mark_param_given(1575); self.recompute_instance_static(); Ok(()) }
            "p2ckappad" => { validate_finite_parameter("p2ckappad", value)?; self.params.p1576 = value; self.mark_param_given(1576); self.recompute_instance_static(); Ok(()) }
            "ckappab" => { validate_finite_parameter("ckappab", value)?; self.params.p1577 = value; self.mark_param_given(1577); self.recompute_instance_static(); Ok(()) }
            "lckappab" => { validate_finite_parameter("lckappab", value)?; self.params.p1578 = value; self.mark_param_given(1578); self.recompute_instance_static(); Ok(()) }
            "nckappab" => { validate_finite_parameter("nckappab", value)?; self.params.p1579 = value; self.mark_param_given(1579); self.recompute_instance_static(); Ok(()) }
            "pckappab" => { validate_finite_parameter("pckappab", value)?; self.params.p1580 = value; self.mark_param_given(1580); self.recompute_instance_static(); Ok(()) }
            "wckappab" => { validate_finite_parameter("wckappab", value)?; self.params.p1581 = value; self.mark_param_given(1581); self.recompute_instance_static(); Ok(()) }
            "p2ckappab" => { validate_finite_parameter("p2ckappab", value)?; self.params.p1582 = value; self.mark_param_given(1582); self.recompute_instance_static(); Ok(()) }
            "csdesw" => { validate_parameter("csdesw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1583 = value; self.mark_param_given(1583); self.recompute_instance_static(); Ok(()) }
            "cjs" => { validate_parameter("cjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1584 = value; self.mark_param_given(1584); self.recompute_instance_static(); Ok(()) }
            "cjd" => { validate_parameter("cjd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1585 = value; self.mark_param_given(1585); self.recompute_instance_static(); Ok(()) }
            "cjsws" => { validate_parameter("cjsws", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1586 = value; self.mark_param_given(1586); self.recompute_instance_static(); Ok(()) }
            "cjswd" => { validate_parameter("cjswd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1587 = value; self.mark_param_given(1587); self.recompute_instance_static(); Ok(()) }
            "cjswgs" => { validate_parameter("cjswgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1588 = value; self.mark_param_given(1588); self.recompute_instance_static(); Ok(()) }
            "cjswgd" => { validate_parameter("cjswgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1589 = value; self.mark_param_given(1589); self.recompute_instance_static(); Ok(()) }
            "pbs" => { validate_finite_parameter("pbs", value)?; self.params.p1590 = value; self.mark_param_given(1590); self.recompute_instance_static(); Ok(()) }
            "pbd" => { validate_finite_parameter("pbd", value)?; self.params.p1591 = value; self.mark_param_given(1591); self.recompute_instance_static(); Ok(()) }
            "pbsws" => { validate_finite_parameter("pbsws", value)?; self.params.p1592 = value; self.mark_param_given(1592); self.recompute_instance_static(); Ok(()) }
            "pbswd" => { validate_finite_parameter("pbswd", value)?; self.params.p1593 = value; self.mark_param_given(1593); self.recompute_instance_static(); Ok(()) }
            "pbswgs" => { validate_finite_parameter("pbswgs", value)?; self.params.p1594 = value; self.mark_param_given(1594); self.recompute_instance_static(); Ok(()) }
            "pbswgd" => { validate_finite_parameter("pbswgd", value)?; self.params.p1595 = value; self.mark_param_given(1595); self.recompute_instance_static(); Ok(()) }
            "mjs" => { validate_parameter("mjs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1596 = value; self.mark_param_given(1596); self.recompute_instance_static(); Ok(()) }
            "mjd" => { validate_parameter("mjd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1597 = value; self.mark_param_given(1597); self.recompute_instance_static(); Ok(()) }
            "mjsws" => { validate_parameter("mjsws", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1598 = value; self.mark_param_given(1598); self.recompute_instance_static(); Ok(()) }
            "mjswd" => { validate_parameter("mjswd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1599 = value; self.mark_param_given(1599); self.recompute_instance_static(); Ok(()) }
            "mjswgs" => { validate_parameter("mjswgs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1600 = value; self.mark_param_given(1600); self.recompute_instance_static(); Ok(()) }
            "mjswgd" => { validate_parameter("mjswgd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1601 = value; self.mark_param_given(1601); self.recompute_instance_static(); Ok(()) }
            "sjs" => { validate_parameter("sjs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1602 = value; self.mark_param_given(1602); self.recompute_instance_static(); Ok(()) }
            "sjd" => { validate_parameter("sjd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1603 = value; self.mark_param_given(1603); self.recompute_instance_static(); Ok(()) }
            "sjsws" => { validate_parameter("sjsws", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1604 = value; self.mark_param_given(1604); self.recompute_instance_static(); Ok(()) }
            "sjswd" => { validate_parameter("sjswd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1605 = value; self.mark_param_given(1605); self.recompute_instance_static(); Ok(()) }
            "sjswgs" => { validate_parameter("sjswgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1606 = value; self.mark_param_given(1606); self.recompute_instance_static(); Ok(()) }
            "sjswgd" => { validate_parameter("sjswgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1607 = value; self.mark_param_given(1607); self.recompute_instance_static(); Ok(()) }
            "mjs2" => { validate_finite_parameter("mjs2", value)?; self.params.p1608 = value; self.mark_param_given(1608); self.recompute_instance_static(); Ok(()) }
            "mjd2" => { validate_finite_parameter("mjd2", value)?; self.params.p1609 = value; self.mark_param_given(1609); self.recompute_instance_static(); Ok(()) }
            "mjsws2" => { validate_finite_parameter("mjsws2", value)?; self.params.p1610 = value; self.mark_param_given(1610); self.recompute_instance_static(); Ok(()) }
            "mjswd2" => { validate_finite_parameter("mjswd2", value)?; self.params.p1611 = value; self.mark_param_given(1611); self.recompute_instance_static(); Ok(()) }
            "mjswgs2" => { validate_finite_parameter("mjswgs2", value)?; self.params.p1612 = value; self.mark_param_given(1612); self.recompute_instance_static(); Ok(()) }
            "mjswgd2" => { validate_finite_parameter("mjswgd2", value)?; self.params.p1613 = value; self.mark_param_given(1613); self.recompute_instance_static(); Ok(()) }
            "jss" => { validate_parameter("jss", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1614 = value; self.mark_param_given(1614); self.recompute_instance_static(); Ok(()) }
            "jsd" => { validate_parameter("jsd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1615 = value; self.mark_param_given(1615); self.recompute_instance_static(); Ok(()) }
            "jsws" => { validate_parameter("jsws", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1616 = value; self.mark_param_given(1616); self.recompute_instance_static(); Ok(()) }
            "jswd" => { validate_parameter("jswd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1617 = value; self.mark_param_given(1617); self.recompute_instance_static(); Ok(()) }
            "jswgs" => { validate_parameter("jswgs", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1618 = value; self.mark_param_given(1618); self.recompute_instance_static(); Ok(()) }
            "jswgd" => { validate_parameter("jswgd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1619 = value; self.mark_param_given(1619); self.recompute_instance_static(); Ok(()) }
            "njs" => { validate_parameter("njs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1620 = value; self.mark_param_given(1620); self.recompute_instance_static(); Ok(()) }
            "njd" => { validate_parameter("njd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1621 = value; self.mark_param_given(1621); self.recompute_instance_static(); Ok(()) }
            "ijthsfwd" => { validate_finite_parameter("ijthsfwd", value)?; self.params.p1622 = value; self.mark_param_given(1622); self.recompute_instance_static(); Ok(()) }
            "ijthdfwd" => { validate_finite_parameter("ijthdfwd", value)?; self.params.p1623 = value; self.mark_param_given(1623); self.recompute_instance_static(); Ok(()) }
            "ijthsrev" => { validate_finite_parameter("ijthsrev", value)?; self.params.p1624 = value; self.mark_param_given(1624); self.recompute_instance_static(); Ok(()) }
            "ijthdrev" => { validate_finite_parameter("ijthdrev", value)?; self.params.p1625 = value; self.mark_param_given(1625); self.recompute_instance_static(); Ok(()) }
            "bvs" => { validate_finite_parameter("bvs", value)?; self.params.p1626 = value; self.mark_param_given(1626); self.recompute_instance_static(); Ok(()) }
            "bvd" => { validate_finite_parameter("bvd", value)?; self.params.p1627 = value; self.mark_param_given(1627); self.recompute_instance_static(); Ok(()) }
            "xjbvs" => { validate_finite_parameter("xjbvs", value)?; self.params.p1628 = value; self.mark_param_given(1628); self.recompute_instance_static(); Ok(()) }
            "xjbvd" => { validate_finite_parameter("xjbvd", value)?; self.params.p1629 = value; self.mark_param_given(1629); self.recompute_instance_static(); Ok(()) }
            "jtss" => { validate_finite_parameter("jtss", value)?; self.params.p1630 = value; self.mark_param_given(1630); self.recompute_instance_static(); Ok(()) }
            "jtsd" => { validate_finite_parameter("jtsd", value)?; self.params.p1631 = value; self.mark_param_given(1631); self.recompute_instance_static(); Ok(()) }
            "jtssws" => { validate_finite_parameter("jtssws", value)?; self.params.p1632 = value; self.mark_param_given(1632); self.recompute_instance_static(); Ok(()) }
            "jtsswd" => { validate_finite_parameter("jtsswd", value)?; self.params.p1633 = value; self.mark_param_given(1633); self.recompute_instance_static(); Ok(()) }
            "jtsswgs" => { validate_finite_parameter("jtsswgs", value)?; self.params.p1634 = value; self.mark_param_given(1634); self.recompute_instance_static(); Ok(()) }
            "jtsswgd" => { validate_finite_parameter("jtsswgd", value)?; self.params.p1635 = value; self.mark_param_given(1635); self.recompute_instance_static(); Ok(()) }
            "jtweff" => { validate_parameter("jtweff", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1636 = value; self.mark_param_given(1636); self.recompute_instance_static(); Ok(()) }
            "njts" => { validate_finite_parameter("njts", value)?; self.params.p1637 = value; self.mark_param_given(1637); self.recompute_instance_static(); Ok(()) }
            "njtsd" => { validate_finite_parameter("njtsd", value)?; self.params.p1638 = value; self.mark_param_given(1638); self.recompute_instance_static(); Ok(()) }
            "njtssw" => { validate_finite_parameter("njtssw", value)?; self.params.p1639 = value; self.mark_param_given(1639); self.recompute_instance_static(); Ok(()) }
            "njtsswd" => { validate_finite_parameter("njtsswd", value)?; self.params.p1640 = value; self.mark_param_given(1640); self.recompute_instance_static(); Ok(()) }
            "njtsswg" => { validate_finite_parameter("njtsswg", value)?; self.params.p1641 = value; self.mark_param_given(1641); self.recompute_instance_static(); Ok(()) }
            "njtsswgd" => { validate_finite_parameter("njtsswgd", value)?; self.params.p1642 = value; self.mark_param_given(1642); self.recompute_instance_static(); Ok(()) }
            "vtss" => { validate_finite_parameter("vtss", value)?; self.params.p1643 = value; self.mark_param_given(1643); self.recompute_instance_static(); Ok(()) }
            "vtsd" => { validate_finite_parameter("vtsd", value)?; self.params.p1644 = value; self.mark_param_given(1644); self.recompute_instance_static(); Ok(()) }
            "vtssws" => { validate_finite_parameter("vtssws", value)?; self.params.p1645 = value; self.mark_param_given(1645); self.recompute_instance_static(); Ok(()) }
            "vtsswd" => { validate_finite_parameter("vtsswd", value)?; self.params.p1646 = value; self.mark_param_given(1646); self.recompute_instance_static(); Ok(()) }
            "vtsswgs" => { validate_finite_parameter("vtsswgs", value)?; self.params.p1647 = value; self.mark_param_given(1647); self.recompute_instance_static(); Ok(()) }
            "vtsswgd" => { validate_finite_parameter("vtsswgd", value)?; self.params.p1648 = value; self.mark_param_given(1648); self.recompute_instance_static(); Ok(()) }
            "lintigen" => { validate_finite_parameter("lintigen", value)?; self.params.p1649 = value; self.mark_param_given(1649); self.recompute_instance_static(); Ok(()) }
            "ntgen" => { validate_finite_parameter("ntgen", value)?; self.params.p1650 = value; self.mark_param_given(1650); self.recompute_instance_static(); Ok(()) }
            "lntgen" => { validate_finite_parameter("lntgen", value)?; self.params.p1651 = value; self.mark_param_given(1651); self.recompute_instance_static(); Ok(()) }
            "nntgen" => { validate_finite_parameter("nntgen", value)?; self.params.p1652 = value; self.mark_param_given(1652); self.recompute_instance_static(); Ok(()) }
            "pntgen" => { validate_finite_parameter("pntgen", value)?; self.params.p1653 = value; self.mark_param_given(1653); self.recompute_instance_static(); Ok(()) }
            "wntgen" => { validate_finite_parameter("wntgen", value)?; self.params.p1654 = value; self.mark_param_given(1654); self.recompute_instance_static(); Ok(()) }
            "p2ntgen" => { validate_finite_parameter("p2ntgen", value)?; self.params.p1655 = value; self.mark_param_given(1655); self.recompute_instance_static(); Ok(()) }
            "aigen" => { validate_finite_parameter("aigen", value)?; self.params.p1656 = value; self.mark_param_given(1656); self.recompute_instance_static(); Ok(()) }
            "laigen" => { validate_finite_parameter("laigen", value)?; self.params.p1657 = value; self.mark_param_given(1657); self.recompute_instance_static(); Ok(()) }
            "naigen" => { validate_finite_parameter("naigen", value)?; self.params.p1658 = value; self.mark_param_given(1658); self.recompute_instance_static(); Ok(()) }
            "paigen" => { validate_finite_parameter("paigen", value)?; self.params.p1659 = value; self.mark_param_given(1659); self.recompute_instance_static(); Ok(()) }
            "waigen" => { validate_finite_parameter("waigen", value)?; self.params.p1660 = value; self.mark_param_given(1660); self.recompute_instance_static(); Ok(()) }
            "p2aigen" => { validate_finite_parameter("p2aigen", value)?; self.params.p1661 = value; self.mark_param_given(1661); self.recompute_instance_static(); Ok(()) }
            "bigen" => { validate_finite_parameter("bigen", value)?; self.params.p1662 = value; self.mark_param_given(1662); self.recompute_instance_static(); Ok(()) }
            "lbigen" => { validate_finite_parameter("lbigen", value)?; self.params.p1663 = value; self.mark_param_given(1663); self.recompute_instance_static(); Ok(()) }
            "nbigen" => { validate_finite_parameter("nbigen", value)?; self.params.p1664 = value; self.mark_param_given(1664); self.recompute_instance_static(); Ok(()) }
            "pbigen" => { validate_finite_parameter("pbigen", value)?; self.params.p1665 = value; self.mark_param_given(1665); self.recompute_instance_static(); Ok(()) }
            "wbigen" => { validate_finite_parameter("wbigen", value)?; self.params.p1666 = value; self.mark_param_given(1666); self.recompute_instance_static(); Ok(()) }
            "p2bigen" => { validate_finite_parameter("p2bigen", value)?; self.params.p1667 = value; self.mark_param_given(1667); self.recompute_instance_static(); Ok(()) }
            "xrcrg1" => { validate_finite_parameter("xrcrg1", value)?; self.params.p1668 = value; self.mark_param_given(1668); self.recompute_instance_static(); Ok(()) }
            "lxrcrg1" => { validate_finite_parameter("lxrcrg1", value)?; self.params.p1669 = value; self.mark_param_given(1669); self.recompute_instance_static(); Ok(()) }
            "nxrcrg1" => { validate_finite_parameter("nxrcrg1", value)?; self.params.p1670 = value; self.mark_param_given(1670); self.recompute_instance_static(); Ok(()) }
            "pxrcrg1" => { validate_finite_parameter("pxrcrg1", value)?; self.params.p1671 = value; self.mark_param_given(1671); self.recompute_instance_static(); Ok(()) }
            "wxrcrg1" => { validate_finite_parameter("wxrcrg1", value)?; self.params.p1672 = value; self.mark_param_given(1672); self.recompute_instance_static(); Ok(()) }
            "p2xrcrg1" => { validate_finite_parameter("p2xrcrg1", value)?; self.params.p1673 = value; self.mark_param_given(1673); self.recompute_instance_static(); Ok(()) }
            "xrcrg2" => { validate_finite_parameter("xrcrg2", value)?; self.params.p1674 = value; self.mark_param_given(1674); self.recompute_instance_static(); Ok(()) }
            "lxrcrg2" => { validate_finite_parameter("lxrcrg2", value)?; self.params.p1675 = value; self.mark_param_given(1675); self.recompute_instance_static(); Ok(()) }
            "nxrcrg2" => { validate_finite_parameter("nxrcrg2", value)?; self.params.p1676 = value; self.mark_param_given(1676); self.recompute_instance_static(); Ok(()) }
            "pxrcrg2" => { validate_finite_parameter("pxrcrg2", value)?; self.params.p1677 = value; self.mark_param_given(1677); self.recompute_instance_static(); Ok(()) }
            "wxrcrg2" => { validate_finite_parameter("wxrcrg2", value)?; self.params.p1678 = value; self.mark_param_given(1678); self.recompute_instance_static(); Ok(()) }
            "p2xrcrg2" => { validate_finite_parameter("p2xrcrg2", value)?; self.params.p1679 = value; self.mark_param_given(1679); self.recompute_instance_static(); Ok(()) }
            "ef" => { validate_parameter("ef", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1680 = value; self.mark_param_given(1680); self.recompute_instance_static(); Ok(()) }
            "em" => { validate_parameter("em", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1681 = value; self.mark_param_given(1681); self.recompute_instance_static(); Ok(()) }
            "noia" => { validate_parameter("noia", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1682 = value; self.mark_param_given(1682); self.recompute_instance_static(); Ok(()) }
            "noib" => { validate_parameter("noib", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1683 = value; self.mark_param_given(1683); self.recompute_instance_static(); Ok(()) }
            "noic" => { validate_parameter("noic", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1684 = value; self.mark_param_given(1684); self.recompute_instance_static(); Ok(()) }
            "k0noi" => { validate_parameter("k0noi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1685 = value; self.mark_param_given(1685); self.recompute_instance_static(); Ok(()) }
            "k1noi" => { validate_parameter("k1noi", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1686 = value; self.mark_param_given(1686); self.recompute_instance_static(); Ok(()) }
            "lintnoi" => { validate_finite_parameter("lintnoi", value)?; self.params.p1687 = value; self.mark_param_given(1687); self.recompute_instance_static(); Ok(()) }
            "smooth" => { validate_parameter("smooth", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1688 = value; self.mark_param_given(1688); self.recompute_instance_static(); Ok(()) }
            "noia2" => { validate_finite_parameter("noia2", value)?; self.params.p1689 = value; self.mark_param_given(1689); self.recompute_instance_static(); Ok(()) }
            "lnoia2" => { validate_finite_parameter("lnoia2", value)?; self.params.p1690 = value; self.mark_param_given(1690); self.recompute_instance_static(); Ok(()) }
            "nnoia2" => { validate_finite_parameter("nnoia2", value)?; self.params.p1691 = value; self.mark_param_given(1691); self.recompute_instance_static(); Ok(()) }
            "pnoia2" => { validate_finite_parameter("pnoia2", value)?; self.params.p1692 = value; self.mark_param_given(1692); self.recompute_instance_static(); Ok(()) }
            "wnoia2" => { validate_finite_parameter("wnoia2", value)?; self.params.p1693 = value; self.mark_param_given(1693); self.recompute_instance_static(); Ok(()) }
            "p2noia2" => { validate_finite_parameter("p2noia2", value)?; self.params.p1694 = value; self.mark_param_given(1694); self.recompute_instance_static(); Ok(()) }
            "mpower" => { validate_finite_parameter("mpower", value)?; self.params.p1695 = value; self.mark_param_given(1695); self.recompute_instance_static(); Ok(()) }
            "lmpower" => { validate_finite_parameter("lmpower", value)?; self.params.p1696 = value; self.mark_param_given(1696); self.recompute_instance_static(); Ok(()) }
            "nmpower" => { validate_finite_parameter("nmpower", value)?; self.params.p1697 = value; self.mark_param_given(1697); self.recompute_instance_static(); Ok(()) }
            "pmpower" => { validate_finite_parameter("pmpower", value)?; self.params.p1698 = value; self.mark_param_given(1698); self.recompute_instance_static(); Ok(()) }
            "wmpower" => { validate_finite_parameter("wmpower", value)?; self.params.p1699 = value; self.mark_param_given(1699); self.recompute_instance_static(); Ok(()) }
            "p2mpower" => { validate_finite_parameter("p2mpower", value)?; self.params.p1700 = value; self.mark_param_given(1700); self.recompute_instance_static(); Ok(()) }
            "qsref" => { validate_finite_parameter("qsref", value)?; self.params.p1701 = value; self.mark_param_given(1701); self.recompute_instance_static(); Ok(()) }
            "lqsref" => { validate_finite_parameter("lqsref", value)?; self.params.p1702 = value; self.mark_param_given(1702); self.recompute_instance_static(); Ok(()) }
            "nqsref" => { validate_finite_parameter("nqsref", value)?; self.params.p1703 = value; self.mark_param_given(1703); self.recompute_instance_static(); Ok(()) }
            "pqsref" => { validate_finite_parameter("pqsref", value)?; self.params.p1704 = value; self.mark_param_given(1704); self.recompute_instance_static(); Ok(()) }
            "wqsref" => { validate_finite_parameter("wqsref", value)?; self.params.p1705 = value; self.mark_param_given(1705); self.recompute_instance_static(); Ok(()) }
            "p2qsref" => { validate_finite_parameter("p2qsref", value)?; self.params.p1706 = value; self.mark_param_given(1706); self.recompute_instance_static(); Ok(()) }
            "ntnoi" => { validate_parameter("ntnoi", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1707 = value; self.mark_param_given(1707); self.recompute_instance_static(); Ok(()) }
            "rnoia" => { validate_parameter("rnoia", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1708 = value; self.mark_param_given(1708); self.recompute_instance_static(); Ok(()) }
            "tnoia" => { validate_finite_parameter("tnoia", value)?; self.params.p1709 = value; self.mark_param_given(1709); self.recompute_instance_static(); Ok(()) }
            "rnoib" => { validate_parameter("rnoib", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1710 = value; self.mark_param_given(1710); self.recompute_instance_static(); Ok(()) }
            "tnoib" => { validate_finite_parameter("tnoib", value)?; self.params.p1711 = value; self.mark_param_given(1711); self.recompute_instance_static(); Ok(()) }
            "rnoic" => { validate_parameter("rnoic", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1712 = value; self.mark_param_given(1712); self.recompute_instance_static(); Ok(()) }
            "tnoic" => { validate_finite_parameter("tnoic", value)?; self.params.p1713 = value; self.mark_param_given(1713); self.recompute_instance_static(); Ok(()) }
            "rnoik" => { validate_parameter("rnoik", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1714 = value; self.mark_param_given(1714); self.recompute_instance_static(); Ok(()) }
            "tnoik" => { validate_finite_parameter("tnoik", value)?; self.params.p1715 = value; self.mark_param_given(1715); self.recompute_instance_static(); Ok(()) }
            "tnoik2" => { validate_parameter("tnoik2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1716 = value; self.mark_param_given(1716); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_parameter("tnom", value, Some((-273.15, "-273.15")), false, None, true, &[])?; self.params.p1717 = value; self.mark_param_given(1717); self.recompute_instance_static(); Ok(()) }
            "tbgasub" => { validate_finite_parameter("tbgasub", value)?; self.params.p1718 = value; self.mark_param_given(1718); self.recompute_instance_static(); Ok(()) }
            "tbgbsub" => { validate_finite_parameter("tbgbsub", value)?; self.params.p1719 = value; self.mark_param_given(1719); self.recompute_instance_static(); Ok(()) }
            "kt1l" => { validate_finite_parameter("kt1l", value)?; self.params.p1720 = value; self.mark_param_given(1720); self.recompute_instance_static(); Ok(()) }
            "tcj" => { validate_finite_parameter("tcj", value)?; self.params.p1721 = value; self.mark_param_given(1721); self.recompute_instance_static(); Ok(()) }
            "tcjsw" => { validate_finite_parameter("tcjsw", value)?; self.params.p1722 = value; self.mark_param_given(1722); self.recompute_instance_static(); Ok(()) }
            "tcjswg" => { validate_finite_parameter("tcjswg", value)?; self.params.p1723 = value; self.mark_param_given(1723); self.recompute_instance_static(); Ok(()) }
            "tpb" => { validate_finite_parameter("tpb", value)?; self.params.p1724 = value; self.mark_param_given(1724); self.recompute_instance_static(); Ok(()) }
            "tpbsw" => { validate_finite_parameter("tpbsw", value)?; self.params.p1725 = value; self.mark_param_given(1725); self.recompute_instance_static(); Ok(()) }
            "tpbswg" => { validate_finite_parameter("tpbswg", value)?; self.params.p1726 = value; self.mark_param_given(1726); self.recompute_instance_static(); Ok(()) }
            "xtis" => { validate_finite_parameter("xtis", value)?; self.params.p1727 = value; self.mark_param_given(1727); self.recompute_instance_static(); Ok(()) }
            "xtid" => { validate_finite_parameter("xtid", value)?; self.params.p1728 = value; self.mark_param_given(1728); self.recompute_instance_static(); Ok(()) }
            "xtss" => { validate_finite_parameter("xtss", value)?; self.params.p1729 = value; self.mark_param_given(1729); self.recompute_instance_static(); Ok(()) }
            "xtsd" => { validate_finite_parameter("xtsd", value)?; self.params.p1730 = value; self.mark_param_given(1730); self.recompute_instance_static(); Ok(()) }
            "xtssws" => { validate_finite_parameter("xtssws", value)?; self.params.p1731 = value; self.mark_param_given(1731); self.recompute_instance_static(); Ok(()) }
            "xtsswd" => { validate_finite_parameter("xtsswd", value)?; self.params.p1732 = value; self.mark_param_given(1732); self.recompute_instance_static(); Ok(()) }
            "xtsswgs" => { validate_finite_parameter("xtsswgs", value)?; self.params.p1733 = value; self.mark_param_given(1733); self.recompute_instance_static(); Ok(()) }
            "xtsswgd" => { validate_finite_parameter("xtsswgd", value)?; self.params.p1734 = value; self.mark_param_given(1734); self.recompute_instance_static(); Ok(()) }
            "tnjts" => { validate_finite_parameter("tnjts", value)?; self.params.p1735 = value; self.mark_param_given(1735); self.recompute_instance_static(); Ok(()) }
            "tnjtsd" => { validate_finite_parameter("tnjtsd", value)?; self.params.p1736 = value; self.mark_param_given(1736); self.recompute_instance_static(); Ok(()) }
            "tnjtssw" => { validate_finite_parameter("tnjtssw", value)?; self.params.p1737 = value; self.mark_param_given(1737); self.recompute_instance_static(); Ok(()) }
            "tnjtsswd" => { validate_finite_parameter("tnjtsswd", value)?; self.params.p1738 = value; self.mark_param_given(1738); self.recompute_instance_static(); Ok(()) }
            "tnjtsswg" => { validate_finite_parameter("tnjtsswg", value)?; self.params.p1739 = value; self.mark_param_given(1739); self.recompute_instance_static(); Ok(()) }
            "tnjtsswgd" => { validate_finite_parameter("tnjtsswgd", value)?; self.params.p1740 = value; self.mark_param_given(1740); self.recompute_instance_static(); Ok(()) }
            "kt1" => { validate_finite_parameter("kt1", value)?; self.params.p1741 = value; self.mark_param_given(1741); self.recompute_instance_static(); Ok(()) }
            "lkt1" => { validate_finite_parameter("lkt1", value)?; self.params.p1742 = value; self.mark_param_given(1742); self.recompute_instance_static(); Ok(()) }
            "nkt1" => { validate_finite_parameter("nkt1", value)?; self.params.p1743 = value; self.mark_param_given(1743); self.recompute_instance_static(); Ok(()) }
            "pkt1" => { validate_finite_parameter("pkt1", value)?; self.params.p1744 = value; self.mark_param_given(1744); self.recompute_instance_static(); Ok(()) }
            "wkt1" => { validate_finite_parameter("wkt1", value)?; self.params.p1745 = value; self.mark_param_given(1745); self.recompute_instance_static(); Ok(()) }
            "p2kt1" => { validate_finite_parameter("p2kt1", value)?; self.params.p1746 = value; self.mark_param_given(1746); self.recompute_instance_static(); Ok(()) }
            "kt11" => { validate_finite_parameter("kt11", value)?; self.params.p1747 = value; self.mark_param_given(1747); self.recompute_instance_static(); Ok(()) }
            "kt12" => { validate_finite_parameter("kt12", value)?; self.params.p1748 = value; self.mark_param_given(1748); self.recompute_instance_static(); Ok(()) }
            "tvth" => { validate_finite_parameter("tvth", value)?; self.params.p1749 = value; self.mark_param_given(1749); self.recompute_instance_static(); Ok(()) }
            "tss" => { validate_finite_parameter("tss", value)?; self.params.p1750 = value; self.mark_param_given(1750); self.recompute_instance_static(); Ok(()) }
            "ltss" => { validate_finite_parameter("ltss", value)?; self.params.p1751 = value; self.mark_param_given(1751); self.recompute_instance_static(); Ok(()) }
            "ntss" => { validate_finite_parameter("ntss", value)?; self.params.p1752 = value; self.mark_param_given(1752); self.recompute_instance_static(); Ok(()) }
            "ptss" => { validate_finite_parameter("ptss", value)?; self.params.p1753 = value; self.mark_param_given(1753); self.recompute_instance_static(); Ok(()) }
            "wtss" => { validate_finite_parameter("wtss", value)?; self.params.p1754 = value; self.mark_param_given(1754); self.recompute_instance_static(); Ok(()) }
            "p2tss" => { validate_finite_parameter("p2tss", value)?; self.params.p1755 = value; self.mark_param_given(1755); self.recompute_instance_static(); Ok(()) }
            "iit" => { validate_finite_parameter("iit", value)?; self.params.p1756 = value; self.mark_param_given(1756); self.recompute_instance_static(); Ok(()) }
            "liit" => { validate_finite_parameter("liit", value)?; self.params.p1757 = value; self.mark_param_given(1757); self.recompute_instance_static(); Ok(()) }
            "niit" => { validate_finite_parameter("niit", value)?; self.params.p1758 = value; self.mark_param_given(1758); self.recompute_instance_static(); Ok(()) }
            "piit" => { validate_finite_parameter("piit", value)?; self.params.p1759 = value; self.mark_param_given(1759); self.recompute_instance_static(); Ok(()) }
            "wiit" => { validate_finite_parameter("wiit", value)?; self.params.p1760 = value; self.mark_param_given(1760); self.recompute_instance_static(); Ok(()) }
            "p2iit" => { validate_finite_parameter("p2iit", value)?; self.params.p1761 = value; self.mark_param_given(1761); self.recompute_instance_static(); Ok(()) }
            "tii" => { validate_finite_parameter("tii", value)?; self.params.p1762 = value; self.mark_param_given(1762); self.recompute_instance_static(); Ok(()) }
            "ltii" => { validate_finite_parameter("ltii", value)?; self.params.p1763 = value; self.mark_param_given(1763); self.recompute_instance_static(); Ok(()) }
            "ntii" => { validate_finite_parameter("ntii", value)?; self.params.p1764 = value; self.mark_param_given(1764); self.recompute_instance_static(); Ok(()) }
            "ptii" => { validate_finite_parameter("ptii", value)?; self.params.p1765 = value; self.mark_param_given(1765); self.recompute_instance_static(); Ok(()) }
            "wtii" => { validate_finite_parameter("wtii", value)?; self.params.p1766 = value; self.mark_param_given(1766); self.recompute_instance_static(); Ok(()) }
            "p2tii" => { validate_finite_parameter("p2tii", value)?; self.params.p1767 = value; self.mark_param_given(1767); self.recompute_instance_static(); Ok(()) }
            "tgidl" => { validate_finite_parameter("tgidl", value)?; self.params.p1768 = value; self.mark_param_given(1768); self.recompute_instance_static(); Ok(()) }
            "ltgidl" => { validate_finite_parameter("ltgidl", value)?; self.params.p1769 = value; self.mark_param_given(1769); self.recompute_instance_static(); Ok(()) }
            "ntgidl" => { validate_finite_parameter("ntgidl", value)?; self.params.p1770 = value; self.mark_param_given(1770); self.recompute_instance_static(); Ok(()) }
            "ptgidl" => { validate_finite_parameter("ptgidl", value)?; self.params.p1771 = value; self.mark_param_given(1771); self.recompute_instance_static(); Ok(()) }
            "wtgidl" => { validate_finite_parameter("wtgidl", value)?; self.params.p1772 = value; self.mark_param_given(1772); self.recompute_instance_static(); Ok(()) }
            "p2tgidl" => { validate_finite_parameter("p2tgidl", value)?; self.params.p1773 = value; self.mark_param_given(1773); self.recompute_instance_static(); Ok(()) }
            "ttat" => { validate_finite_parameter("ttat", value)?; self.params.p1774 = value; self.mark_param_given(1774); self.recompute_instance_static(); Ok(()) }
            "lttat" => { validate_finite_parameter("lttat", value)?; self.params.p1775 = value; self.mark_param_given(1775); self.recompute_instance_static(); Ok(()) }
            "nttat" => { validate_finite_parameter("nttat", value)?; self.params.p1776 = value; self.mark_param_given(1776); self.recompute_instance_static(); Ok(()) }
            "pttat" => { validate_finite_parameter("pttat", value)?; self.params.p1777 = value; self.mark_param_given(1777); self.recompute_instance_static(); Ok(()) }
            "wttat" => { validate_finite_parameter("wttat", value)?; self.params.p1778 = value; self.mark_param_given(1778); self.recompute_instance_static(); Ok(()) }
            "p2ttat" => { validate_finite_parameter("p2ttat", value)?; self.params.p1779 = value; self.mark_param_given(1779); self.recompute_instance_static(); Ok(()) }
            "igt" => { validate_finite_parameter("igt", value)?; self.params.p1780 = value; self.mark_param_given(1780); self.recompute_instance_static(); Ok(()) }
            "ligt" => { validate_finite_parameter("ligt", value)?; self.params.p1781 = value; self.mark_param_given(1781); self.recompute_instance_static(); Ok(()) }
            "nigt" => { validate_finite_parameter("nigt", value)?; self.params.p1782 = value; self.mark_param_given(1782); self.recompute_instance_static(); Ok(()) }
            "pigt" => { validate_finite_parameter("pigt", value)?; self.params.p1783 = value; self.mark_param_given(1783); self.recompute_instance_static(); Ok(()) }
            "wigt" => { validate_finite_parameter("wigt", value)?; self.params.p1784 = value; self.mark_param_given(1784); self.recompute_instance_static(); Ok(()) }
            "p2igt" => { validate_finite_parameter("p2igt", value)?; self.params.p1785 = value; self.mark_param_given(1785); self.recompute_instance_static(); Ok(()) }
            "tlow" => { validate_parameter("tlow", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1786 = value; self.mark_param_given(1786); self.recompute_instance_static(); Ok(()) }
            "tlow1" => { validate_parameter("tlow1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1787 = value; self.mark_param_given(1787); self.recompute_instance_static(); Ok(()) }
            "dtlow" => { validate_parameter("dtlow", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1788 = value; self.mark_param_given(1788); self.recompute_instance_static(); Ok(()) }
            "dtlow1" => { validate_parameter("dtlow1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1789 = value; self.mark_param_given(1789); self.recompute_instance_static(); Ok(()) }
            "klow1" => { validate_parameter("klow1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1790 = value; self.mark_param_given(1790); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("rth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1791 = value; self.mark_param_given(1791); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("cth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1792 = value; self.mark_param_given(1792); self.recompute_instance_static(); Ok(()) }
            "wth0" => { validate_parameter("wth0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1793 = value; self.mark_param_given(1793); self.recompute_instance_static(); Ok(()) }
            "ashexp" => { validate_parameter("ashexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1794 = value; self.mark_param_given(1794); self.recompute_instance_static(); Ok(()) }
            "bshexp" => { validate_parameter("bshexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1795 = value; self.mark_param_given(1795); self.recompute_instance_static(); Ok(()) }
            "cshexp" => { validate_parameter("cshexp", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1796 = value; self.mark_param_given(1796); self.recompute_instance_static(); Ok(()) }
            "ash" => { validate_parameter("ash", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1797 = value; self.mark_param_given(1797); self.recompute_instance_static(); Ok(()) }
            "csh" => { validate_parameter("csh", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1798 = value; self.mark_param_given(1798); self.recompute_instance_static(); Ok(()) }
            "ach_ufcm" => { validate_parameter("ach_ufcm", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1799 = value; self.mark_param_given(1799); self.recompute_instance_static(); Ok(()) }
            "cins_ufcm" => { validate_parameter("cins_ufcm", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1800 = value; self.mark_param_given(1800); self.recompute_instance_static(); Ok(()) }
            "w_ufcm" => { validate_parameter("w_ufcm", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1801 = value; self.mark_param_given(1801); self.recompute_instance_static(); Ok(()) }
            "tfin_top" => { validate_parameter("tfin_top", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1802 = value; self.mark_param_given(1802); self.recompute_instance_static(); Ok(()) }
            "tfin_base" => { validate_parameter("tfin_base", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1803 = value; self.mark_param_given(1803); self.recompute_instance_static(); Ok(()) }
            "qmfactorcv" => { validate_parameter("qmfactorcv", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1804 = value; self.mark_param_given(1804); self.recompute_instance_static(); Ok(()) }
            "alpha_ufcm" => { validate_parameter("alpha_ufcm", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1805 = value; self.mark_param_given(1805); self.recompute_instance_static(); Ok(()) }
            "dim1h" => { validate_parameter("dim1h", value, Some((1.0, "1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p1806 = value; self.mark_param_given(1806); self.recompute_instance_static(); Ok(()) }
            "dimension1" => { validate_finite_parameter("dimension1", value)?; self.params.p1807 = value; self.mark_param_given(1807); self.recompute_instance_static(); Ok(()) }
            "ldimension1" => { validate_finite_parameter("ldimension1", value)?; self.params.p1808 = value; self.mark_param_given(1808); self.recompute_instance_static(); Ok(()) }
            "ndimension1" => { validate_finite_parameter("ndimension1", value)?; self.params.p1809 = value; self.mark_param_given(1809); self.recompute_instance_static(); Ok(()) }
            "pdimension1" => { validate_finite_parameter("pdimension1", value)?; self.params.p1810 = value; self.mark_param_given(1810); self.recompute_instance_static(); Ok(()) }
            "wdimension1" => { validate_finite_parameter("wdimension1", value)?; self.params.p1811 = value; self.mark_param_given(1811); self.recompute_instance_static(); Ok(()) }
            "p2dimension1" => { validate_finite_parameter("p2dimension1", value)?; self.params.p1812 = value; self.mark_param_given(1812); self.recompute_instance_static(); Ok(()) }
            "dim2h" => { validate_parameter("dim2h", value, Some((1.0, "1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p1813 = value; self.mark_param_given(1813); self.recompute_instance_static(); Ok(()) }
            "dimension2" => { validate_finite_parameter("dimension2", value)?; self.params.p1814 = value; self.mark_param_given(1814); self.recompute_instance_static(); Ok(()) }
            "ldimension2" => { validate_finite_parameter("ldimension2", value)?; self.params.p1815 = value; self.mark_param_given(1815); self.recompute_instance_static(); Ok(()) }
            "ndimension2" => { validate_finite_parameter("ndimension2", value)?; self.params.p1816 = value; self.mark_param_given(1816); self.recompute_instance_static(); Ok(()) }
            "pdimension2" => { validate_finite_parameter("pdimension2", value)?; self.params.p1817 = value; self.mark_param_given(1817); self.recompute_instance_static(); Ok(()) }
            "wdimension2" => { validate_finite_parameter("wdimension2", value)?; self.params.p1818 = value; self.mark_param_given(1818); self.recompute_instance_static(); Ok(()) }
            "p2dimension2" => { validate_finite_parameter("p2dimension2", value)?; self.params.p1819 = value; self.mark_param_given(1819); self.recompute_instance_static(); Ok(()) }
            "dim3h" => { validate_parameter("dim3h", value, Some((1.0, "1.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p1820 = value; self.mark_param_given(1820); self.recompute_instance_static(); Ok(()) }
            "dimension3" => { validate_finite_parameter("dimension3", value)?; self.params.p1821 = value; self.mark_param_given(1821); self.recompute_instance_static(); Ok(()) }
            "ldimension3" => { validate_finite_parameter("ldimension3", value)?; self.params.p1822 = value; self.mark_param_given(1822); self.recompute_instance_static(); Ok(()) }
            "ndimension3" => { validate_finite_parameter("ndimension3", value)?; self.params.p1823 = value; self.mark_param_given(1823); self.recompute_instance_static(); Ok(()) }
            "pdimension3" => { validate_finite_parameter("pdimension3", value)?; self.params.p1824 = value; self.mark_param_given(1824); self.recompute_instance_static(); Ok(()) }
            "wdimension3" => { validate_finite_parameter("wdimension3", value)?; self.params.p1825 = value; self.mark_param_given(1825); self.recompute_instance_static(); Ok(()) }
            "p2dimension3" => { validate_finite_parameter("p2dimension3", value)?; self.params.p1826 = value; self.mark_param_given(1826); self.recompute_instance_static(); Ok(()) }
            "wdim0" => { validate_parameter("wdim0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1827 = value; self.mark_param_given(1827); self.recompute_instance_static(); Ok(()) }
            "wdimr" => { validate_parameter("wdimr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1828 = value; self.mark_param_given(1828); self.recompute_instance_static(); Ok(()) }
            "ssp1" => { validate_finite_parameter("ssp1", value)?; self.params.p1829 = value; self.mark_param_given(1829); self.recompute_instance_static(); Ok(()) }
            "lssp1" => { validate_finite_parameter("lssp1", value)?; self.params.p1830 = value; self.mark_param_given(1830); self.recompute_instance_static(); Ok(()) }
            "nssp1" => { validate_finite_parameter("nssp1", value)?; self.params.p1831 = value; self.mark_param_given(1831); self.recompute_instance_static(); Ok(()) }
            "pssp1" => { validate_finite_parameter("pssp1", value)?; self.params.p1832 = value; self.mark_param_given(1832); self.recompute_instance_static(); Ok(()) }
            "wssp1" => { validate_finite_parameter("wssp1", value)?; self.params.p1833 = value; self.mark_param_given(1833); self.recompute_instance_static(); Ok(()) }
            "p2ssp1" => { validate_finite_parameter("p2ssp1", value)?; self.params.p1834 = value; self.mark_param_given(1834); self.recompute_instance_static(); Ok(()) }
            "ssp2" => { validate_finite_parameter("ssp2", value)?; self.params.p1835 = value; self.mark_param_given(1835); self.recompute_instance_static(); Ok(()) }
            "lssp2" => { validate_finite_parameter("lssp2", value)?; self.params.p1836 = value; self.mark_param_given(1836); self.recompute_instance_static(); Ok(()) }
            "nssp2" => { validate_finite_parameter("nssp2", value)?; self.params.p1837 = value; self.mark_param_given(1837); self.recompute_instance_static(); Ok(()) }
            "pssp2" => { validate_finite_parameter("pssp2", value)?; self.params.p1838 = value; self.mark_param_given(1838); self.recompute_instance_static(); Ok(()) }
            "wssp2" => { validate_finite_parameter("wssp2", value)?; self.params.p1839 = value; self.mark_param_given(1839); self.recompute_instance_static(); Ok(()) }
            "p2ssp2" => { validate_finite_parameter("p2ssp2", value)?; self.params.p1840 = value; self.mark_param_given(1840); self.recompute_instance_static(); Ok(()) }
            "ssp3" => { validate_finite_parameter("ssp3", value)?; self.params.p1841 = value; self.mark_param_given(1841); self.recompute_instance_static(); Ok(()) }
            "lssp3" => { validate_finite_parameter("lssp3", value)?; self.params.p1842 = value; self.mark_param_given(1842); self.recompute_instance_static(); Ok(()) }
            "nssp3" => { validate_finite_parameter("nssp3", value)?; self.params.p1843 = value; self.mark_param_given(1843); self.recompute_instance_static(); Ok(()) }
            "pssp3" => { validate_finite_parameter("pssp3", value)?; self.params.p1844 = value; self.mark_param_given(1844); self.recompute_instance_static(); Ok(()) }
            "wssp3" => { validate_finite_parameter("wssp3", value)?; self.params.p1845 = value; self.mark_param_given(1845); self.recompute_instance_static(); Ok(()) }
            "p2ssp3" => { validate_finite_parameter("p2ssp3", value)?; self.params.p1846 = value; self.mark_param_given(1846); self.recompute_instance_static(); Ok(()) }
            "dssp1" => { validate_parameter("dssp1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1847 = value; self.mark_param_given(1847); self.recompute_instance_static(); Ok(()) }
            "dssp2" => { validate_parameter("dssp2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1848 = value; self.mark_param_given(1848); self.recompute_instance_static(); Ok(()) }
            "dssp3" => { validate_parameter("dssp3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1849 = value; self.mark_param_given(1849); self.recompute_instance_static(); Ok(()) }
            "wssp0" => { validate_parameter("wssp0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1850 = value; self.mark_param_given(1850); self.recompute_instance_static(); Ok(()) }
            "wsspr" => { validate_parameter("wsspr", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1851 = value; self.mark_param_given(1851); self.recompute_instance_static(); Ok(()) }
            "wgaanom" => { validate_parameter("wgaanom", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1852 = value; self.mark_param_given(1852); self.recompute_instance_static(); Ok(()) }
            "e2nom" => { validate_finite_parameter("e2nom", value)?; self.params.p1853 = value; self.mark_param_given(1853); self.recompute_instance_static(); Ok(()) }
            "le2nom" => { validate_finite_parameter("le2nom", value)?; self.params.p1854 = value; self.mark_param_given(1854); self.recompute_instance_static(); Ok(()) }
            "ne2nom" => { validate_finite_parameter("ne2nom", value)?; self.params.p1855 = value; self.mark_param_given(1855); self.recompute_instance_static(); Ok(()) }
            "pe2nom" => { validate_finite_parameter("pe2nom", value)?; self.params.p1856 = value; self.mark_param_given(1856); self.recompute_instance_static(); Ok(()) }
            "we2nom" => { validate_finite_parameter("we2nom", value)?; self.params.p1857 = value; self.mark_param_given(1857); self.recompute_instance_static(); Ok(()) }
            "p2e2nom" => { validate_finite_parameter("p2e2nom", value)?; self.params.p1858 = value; self.mark_param_given(1858); self.recompute_instance_static(); Ok(()) }
            "e3nom" => { validate_finite_parameter("e3nom", value)?; self.params.p1859 = value; self.mark_param_given(1859); self.recompute_instance_static(); Ok(()) }
            "le3nom" => { validate_finite_parameter("le3nom", value)?; self.params.p1860 = value; self.mark_param_given(1860); self.recompute_instance_static(); Ok(()) }
            "ne3nom" => { validate_finite_parameter("ne3nom", value)?; self.params.p1861 = value; self.mark_param_given(1861); self.recompute_instance_static(); Ok(()) }
            "pe3nom" => { validate_finite_parameter("pe3nom", value)?; self.params.p1862 = value; self.mark_param_given(1862); self.recompute_instance_static(); Ok(()) }
            "we3nom" => { validate_finite_parameter("we3nom", value)?; self.params.p1863 = value; self.mark_param_given(1863); self.recompute_instance_static(); Ok(()) }
            "p2e3nom" => { validate_finite_parameter("p2e3nom", value)?; self.params.p1864 = value; self.mark_param_given(1864); self.recompute_instance_static(); Ok(()) }
            "mfe2" => { validate_finite_parameter("mfe2", value)?; self.params.p1865 = value; self.mark_param_given(1865); self.recompute_instance_static(); Ok(()) }
            "mfe3" => { validate_finite_parameter("mfe3", value)?; self.params.p1866 = value; self.mark_param_given(1866); self.recompute_instance_static(); Ok(()) }
            "wsfe2" => { validate_parameter("wsfe2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1867 = value; self.mark_param_given(1867); self.recompute_instance_static(); Ok(()) }
            "wsfe3" => { validate_parameter("wsfe3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1868 = value; self.mark_param_given(1868); self.recompute_instance_static(); Ok(()) }
            "mfq1nom" => { validate_finite_parameter("mfq1nom", value)?; self.params.p1869 = value; self.mark_param_given(1869); self.recompute_instance_static(); Ok(()) }
            "lmfq1nom" => { validate_finite_parameter("lmfq1nom", value)?; self.params.p1870 = value; self.mark_param_given(1870); self.recompute_instance_static(); Ok(()) }
            "nmfq1nom" => { validate_finite_parameter("nmfq1nom", value)?; self.params.p1871 = value; self.mark_param_given(1871); self.recompute_instance_static(); Ok(()) }
            "pmfq1nom" => { validate_finite_parameter("pmfq1nom", value)?; self.params.p1872 = value; self.mark_param_given(1872); self.recompute_instance_static(); Ok(()) }
            "wmfq1nom" => { validate_finite_parameter("wmfq1nom", value)?; self.params.p1873 = value; self.mark_param_given(1873); self.recompute_instance_static(); Ok(()) }
            "p2mfq1nom" => { validate_finite_parameter("p2mfq1nom", value)?; self.params.p1874 = value; self.mark_param_given(1874); self.recompute_instance_static(); Ok(()) }
            "mfq2nom" => { validate_finite_parameter("mfq2nom", value)?; self.params.p1875 = value; self.mark_param_given(1875); self.recompute_instance_static(); Ok(()) }
            "lmfq2nom" => { validate_finite_parameter("lmfq2nom", value)?; self.params.p1876 = value; self.mark_param_given(1876); self.recompute_instance_static(); Ok(()) }
            "nmfq2nom" => { validate_finite_parameter("nmfq2nom", value)?; self.params.p1877 = value; self.mark_param_given(1877); self.recompute_instance_static(); Ok(()) }
            "pmfq2nom" => { validate_finite_parameter("pmfq2nom", value)?; self.params.p1878 = value; self.mark_param_given(1878); self.recompute_instance_static(); Ok(()) }
            "wmfq2nom" => { validate_finite_parameter("wmfq2nom", value)?; self.params.p1879 = value; self.mark_param_given(1879); self.recompute_instance_static(); Ok(()) }
            "p2mfq2nom" => { validate_finite_parameter("p2mfq2nom", value)?; self.params.p1880 = value; self.mark_param_given(1880); self.recompute_instance_static(); Ok(()) }
            "mfq3nom" => { validate_finite_parameter("mfq3nom", value)?; self.params.p1881 = value; self.mark_param_given(1881); self.recompute_instance_static(); Ok(()) }
            "lmfq3nom" => { validate_finite_parameter("lmfq3nom", value)?; self.params.p1882 = value; self.mark_param_given(1882); self.recompute_instance_static(); Ok(()) }
            "nmfq3nom" => { validate_finite_parameter("nmfq3nom", value)?; self.params.p1883 = value; self.mark_param_given(1883); self.recompute_instance_static(); Ok(()) }
            "pmfq3nom" => { validate_finite_parameter("pmfq3nom", value)?; self.params.p1884 = value; self.mark_param_given(1884); self.recompute_instance_static(); Ok(()) }
            "wmfq3nom" => { validate_finite_parameter("wmfq3nom", value)?; self.params.p1885 = value; self.mark_param_given(1885); self.recompute_instance_static(); Ok(()) }
            "p2mfq3nom" => { validate_finite_parameter("p2mfq3nom", value)?; self.params.p1886 = value; self.mark_param_given(1886); self.recompute_instance_static(); Ok(()) }
            "mfq1" => { validate_finite_parameter("mfq1", value)?; self.params.p1887 = value; self.mark_param_given(1887); self.recompute_instance_static(); Ok(()) }
            "mfq2" => { validate_finite_parameter("mfq2", value)?; self.params.p1888 = value; self.mark_param_given(1888); self.recompute_instance_static(); Ok(()) }
            "mfq3" => { validate_finite_parameter("mfq3", value)?; self.params.p1889 = value; self.mark_param_given(1889); self.recompute_instance_static(); Ok(()) }
            "wsfq1" => { validate_parameter("wsfq1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1890 = value; self.mark_param_given(1890); self.recompute_instance_static(); Ok(()) }
            "wsfq2" => { validate_parameter("wsfq2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1891 = value; self.mark_param_given(1891); self.recompute_instance_static(); Ok(()) }
            "wsfq3" => { validate_parameter("wsfq3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1892 = value; self.mark_param_given(1892); self.recompute_instance_static(); Ok(()) }
            "tsre2" => { validate_parameter("tsre2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1893 = value; self.mark_param_given(1893); self.recompute_instance_static(); Ok(()) }
            "tdwse2" => { validate_parameter("tdwse2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1894 = value; self.mark_param_given(1894); self.recompute_instance_static(); Ok(()) }
            "tsre3" => { validate_finite_parameter("tsre3", value)?; self.params.p1895 = value; self.mark_param_given(1895); self.recompute_instance_static(); Ok(()) }
            "tdwse3" => { validate_finite_parameter("tdwse3", value)?; self.params.p1896 = value; self.mark_param_given(1896); self.recompute_instance_static(); Ok(()) }
            "tsrq1" => { validate_parameter("tsrq1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1897 = value; self.mark_param_given(1897); self.recompute_instance_static(); Ok(()) }
            "tdwsq1" => { validate_parameter("tdwsq1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1898 = value; self.mark_param_given(1898); self.recompute_instance_static(); Ok(()) }
            "tsrq2" => { validate_parameter("tsrq2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1899 = value; self.mark_param_given(1899); self.recompute_instance_static(); Ok(()) }
            "tdwsq2" => { validate_parameter("tdwsq2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1900 = value; self.mark_param_given(1900); self.recompute_instance_static(); Ok(()) }
            "tsrq3" => { validate_finite_parameter("tsrq3", value)?; self.params.p1901 = value; self.mark_param_given(1901); self.recompute_instance_static(); Ok(()) }
            "tdwsq3" => { validate_finite_parameter("tdwsq3", value)?; self.params.p1902 = value; self.mark_param_given(1902); self.recompute_instance_static(); Ok(()) }
            "nvsrd" => { validate_parameter("nvsrd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1903 = value; self.mark_param_given(1903); self.recompute_instance_static(); Ok(()) }
            "vsatrsd" => { validate_parameter("vsatrsd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1904 = value; self.mark_param_given(1904); self.recompute_instance_static(); Ok(()) }
            "ptwgvsrsd" => { validate_parameter("ptwgvsrsd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1905 = value; self.mark_param_given(1905); self.recompute_instance_static(); Ok(()) }
            "ptwg1vsrsd" => { validate_finite_parameter("ptwg1vsrsd", value)?; self.params.p1906 = value; self.mark_param_given(1906); self.recompute_instance_static(); Ok(()) }
            "psatxvsrsd" => { validate_parameter("psatxvsrsd", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1907 = value; self.mark_param_given(1907); self.recompute_instance_static(); Ok(()) }
            "mvsrsd" => { validate_parameter("mvsrsd", value, Some((0.0, "0.0")), true, Some((4.0, "4.0")), true, &[])?; self.params.p1908 = value; self.mark_param_given(1908); self.recompute_instance_static(); Ok(()) }
            "nvsrs" => { validate_parameter("nvsrs", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1909 = value; self.mark_param_given(1909); self.recompute_instance_static(); Ok(()) }
            "rdlcw" => { validate_parameter("rdlcw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1910 = value; self.mark_param_given(1910); self.recompute_instance_static(); Ok(()) }
            "rslcw" => { validate_parameter("rslcw", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1911 = value; self.mark_param_given(1911); self.recompute_instance_static(); Ok(()) }
            "prtvsrsd" => { validate_finite_parameter("prtvsrsd", value)?; self.params.p1912 = value; self.mark_param_given(1912); self.recompute_instance_static(); Ok(()) }
            "atvsrsd" => { validate_finite_parameter("atvsrsd", value)?; self.params.p1913 = value; self.mark_param_given(1913); self.recompute_instance_static(); Ok(()) }
            "vsrdfactor" => { validate_parameter("vsrdfactor", value, Some((0.0001, "0.0001")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1914 = value; self.mark_param_given(1914); self.recompute_instance_static(); Ok(()) }
            "vsrsfactor" => { validate_parameter("vsrsfactor", value, Some((0.0001, "0.0001")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1915 = value; self.mark_param_given(1915); self.recompute_instance_static(); Ok(()) }
            "rdvds" => { validate_finite_parameter("rdvds", value)?; self.params.p1916 = value; self.mark_param_given(1916); self.recompute_instance_static(); Ok(()) }
            "gavsrd" => { validate_parameter("gavsrd", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1917 = value; self.mark_param_given(1917); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimcmg_va'", name)),
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
        let v2: f64 = p.p74;
        self.scalar_v2 = v2;
        let v3: f64 = p.p1791;
        self.scalar_v3 = v3;
        let v5: f64 = p.p109;
        self.scalar_v5 = v5;
        let v7: f64 = p.p110;
        self.scalar_v7 = v7;
        let v8: f64 = (1e-6 * p.p110);
        self.scalar_v8 = v8;
        let v9: f64 = p.p0;
        self.scalar_v9 = v9;
        let v10: f64 = (v8 / p.p0);
        self.scalar_v10 = v10;
        let v11: f64 = (p.p109 + v10);
        self.scalar_v11 = v11;
        let v12: f64 = p.p111;
        self.scalar_v12 = v12;
        let v13: f64 = p.p5;
        self.scalar_v13 = v13;
        let v14: f64 = (p.p111 / p.p5);
        self.scalar_v14 = v14;
        let v15: f64 = (v11 + v14);
        self.scalar_v15 = v15;
        let v16: f64 = p.p112;
        self.scalar_v16 = v16;
        let v17: f64 = (1e-6 * p.p112);
        self.scalar_v17 = v17;
        let v18: f64 = (p.p0 * p.p5);
        self.scalar_v18 = v18;
        let v19: f64 = (v17 / v18);
        self.scalar_v19 = v19;
        let v20: f64 = (v15 + v19);
        self.scalar_v20 = v20;
        let v21: f64 = p.p117;
        self.scalar_v21 = v21;
        let v22: f64 = p.p118;
        self.scalar_v22 = v22;
        let v23: f64 = (1e-6 * p.p118);
        self.scalar_v23 = v23;
        let v24: f64 = (v23 / p.p0);
        self.scalar_v24 = v24;
        let v25: f64 = (p.p117 + v24);
        self.scalar_v25 = v25;
        let v26: f64 = p.p119;
        self.scalar_v26 = v26;
        let v27: f64 = (p.p119 / p.p5);
        self.scalar_v27 = v27;
        let v28: f64 = (v25 + v27);
        self.scalar_v28 = v28;
        let v29: f64 = p.p120;
        self.scalar_v29 = v29;
        let v30: f64 = (1e-6 * p.p120);
        self.scalar_v30 = v30;
        let v31: f64 = (v30 / v18);
        self.scalar_v31 = v31;
        let v32: f64 = (v28 + v31);
        self.scalar_v32 = v32;
        let v33: f64 = p.p113;
        self.scalar_v33 = v33;
        let v34: f64 = p.p114;
        self.scalar_v34 = v34;
        let v35: f64 = (1e-6 * p.p114);
        self.scalar_v35 = v35;
        let v36: f64 = (v35 / p.p0);
        self.scalar_v36 = v36;
        let v37: f64 = (p.p113 + v36);
        self.scalar_v37 = v37;
        let v38: f64 = p.p115;
        self.scalar_v38 = v38;
        let v39: f64 = (p.p115 / p.p5);
        self.scalar_v39 = v39;
        let v40: f64 = (v37 + v39);
        self.scalar_v40 = v40;
        let v41: f64 = p.p116;
        self.scalar_v41 = v41;
        let v42: f64 = (1e-6 * p.p116);
        self.scalar_v42 = v42;
        let v43: f64 = (v42 / v18);
        self.scalar_v43 = v43;
        let v44: f64 = (v40 + v43);
        self.scalar_v44 = v44;
        let v45: f64 = (p.p0 + v20);
        self.scalar_v45 = v45;
        let v46: bool = (v45 <= 0.0);
        self.scalar_v46 = v46;
        let v47: f64 = (if v46 { p.p0 } else { v45 });
        self.scalar_v47 = v47;
        let v48: f64 = p.p84;
        self.scalar_v48 = v48;
        let v49: f64 = (-p.p84);
        self.scalar_v49 = v49;
        let v50: f64 = p.p83;
        self.scalar_v50 = v50;
        let v51: f64 = (v32 + v47);
        self.scalar_v51 = v51;
        let v52: f64 = f64::powf(v51, v49);
        self.scalar_v52 = v52;
        let v53: f64 = (p.p83 * v52);
        self.scalar_v53 = v53;
        let v54: f64 = (v44 + v53);
        self.scalar_v54 = v54;
        let v55: f64 = (2.0 * v54);
        self.scalar_v55 = v55;
        let v56: f64 = (v51 - v55);
        self.scalar_v56 = v56;
        let v57: bool = (v56 <= 0.0);
        self.scalar_v57 = v57;
        let v58: f64 = (if v57 { v47 } else { v56 });
        self.scalar_v58 = v58;
        let v59: f64 = p.p62;
        self.scalar_v59 = v59;
        let v61: bool = (p.p62 == 5.0);
        self.scalar_v61 = v61;
        let v62: f64 = p.p121;
        self.scalar_v62 = v62;
        let v63: f64 = p.p122;
        self.scalar_v63 = v63;
        let v64: f64 = (1e-6 * p.p122);
        self.scalar_v64 = v64;
        let v65: f64 = (v64 / p.p0);
        self.scalar_v65 = v65;
        let v66: f64 = (p.p121 + v65);
        self.scalar_v66 = v66;
        let v67: f64 = p.p123;
        self.scalar_v67 = v67;
        let v68: f64 = (p.p123 / p.p5);
        self.scalar_v68 = v68;
        let v69: f64 = (v66 + v68);
        self.scalar_v69 = v69;
        let v70: f64 = p.p124;
        self.scalar_v70 = v70;
        let v71: f64 = (1e-6 * p.p124);
        self.scalar_v71 = v71;
        let v72: f64 = (v71 / v18);
        self.scalar_v72 = v72;
        let v73: f64 = (v69 + v72);
        self.scalar_v73 = v73;
        let v74: f64 = p.p125;
        self.scalar_v74 = v74;
        let v75: f64 = (1e-6 * p.p125);
        self.scalar_v75 = v75;
        let v76: f64 = p.p43;
        self.scalar_v76 = v76;
        let v77: f64 = (v75 / p.p43);
        self.scalar_v77 = v77;
        let v78: f64 = (v73 + v77);
        self.scalar_v78 = v78;
        let v79: f64 = p.p126;
        self.scalar_v79 = v79;
        let v81: f64 = (p.p126 * 1e-12);
        self.scalar_v81 = v81;
        let v82: f64 = (p.p0 * p.p43);
        self.scalar_v82 = v82;
        let v83: f64 = (v81 / v82);
        self.scalar_v83 = v83;
        let v84: f64 = (v78 + v83);
        self.scalar_v84 = v84;
        let v85: f64 = (if v61 { v84 } else { 0.0 });
        self.scalar_v85 = v85;
        let v86: f64 = p.p127;
        self.scalar_v86 = v86;
        let v87: f64 = p.p128;
        self.scalar_v87 = v87;
        let v88: f64 = (1e-6 * p.p128);
        self.scalar_v88 = v88;
        let v89: f64 = (v88 / p.p0);
        self.scalar_v89 = v89;
        let v90: f64 = (p.p127 + v89);
        self.scalar_v90 = v90;
        let v91: f64 = p.p129;
        self.scalar_v91 = v91;
        let v92: f64 = (p.p129 / p.p5);
        self.scalar_v92 = v92;
        let v93: f64 = (v90 + v92);
        self.scalar_v93 = v93;
        let v94: f64 = p.p130;
        self.scalar_v94 = v94;
        let v95: f64 = (1e-6 * p.p130);
        self.scalar_v95 = v95;
        let v96: f64 = (v95 / v18);
        self.scalar_v96 = v96;
        let v97: f64 = (v93 + v96);
        self.scalar_v97 = v97;
        let v98: f64 = p.p131;
        self.scalar_v98 = v98;
        let v99: f64 = (1e-6 * p.p131);
        self.scalar_v99 = v99;
        let v100: f64 = (v99 / p.p43);
        self.scalar_v100 = v100;
        let v101: f64 = (v97 + v100);
        self.scalar_v101 = v101;
        let v102: f64 = p.p132;
        self.scalar_v102 = v102;
        let v103: f64 = (1e-12 * p.p132);
        self.scalar_v103 = v103;
        let v104: f64 = (v103 / v82);
        self.scalar_v104 = v104;
        let v105: f64 = (v101 + v104);
        self.scalar_v105 = v105;
        let v106: f64 = (if v61 { v105 } else { 0.0 });
        self.scalar_v106 = v106;
        let v107: bool = (!v61);
        self.scalar_v107 = v107;
        let v108: f64 = (if v107 { 0.0 } else { v85 });
        self.scalar_v108 = v108;
        let v109: f64 = (if v107 { 0.0 } else { v106 });
        self.scalar_v109 = v109;
        let v110: f64 = (p.p43 + v108);
        self.scalar_v110 = v110;
        let v111: f64 = (v109 + v110);
        self.scalar_v111 = v111;
        let v112: bool = (v111 <= 0.0);
        self.scalar_v112 = v112;
        let v113: bool = (v61 && v112);
        self.scalar_v113 = v113;
        let v114: f64 = (if v113 { p.p43 } else { v111 });
        self.scalar_v114 = v114;
        let v115: f64 = p.p59;
        self.scalar_v115 = v115;
        let v116: f64 = (1e-6 / v58);
        self.scalar_v116 = v116;
        let v117: f64 = (1.0 / p.p5);
        self.scalar_v117 = v117;
        let v118: f64 = (p.p5 * v58);
        self.scalar_v118 = v118;
        let v119: f64 = (1e-6 / v118);
        self.scalar_v119 = v119;
        let v120: f64 = (1e-6 / v114);
        self.scalar_v120 = v120;
        let v121: f64 = (if v61 { v120 } else { 0.0 });
        self.scalar_v121 = v121;
        let v122: f64 = (v58 * v114);
        self.scalar_v122 = v122;
        let v123: f64 = (1e-12 / v122);
        self.scalar_v123 = v123;
        let v124: f64 = (if v61 { v123 } else { 0.0 });
        self.scalar_v124 = v124;
        let v125: f64 = (if v107 { 0.0 } else { v121 });
        self.scalar_v125 = v125;
        let v126: f64 = (if v107 { 0.0 } else { v124 });
        self.scalar_v126 = v126;
        let v128: f64 = p.p73;
        self.scalar_v128 = v128;
        let v129: bool = (0.0 != p.p73);
        self.scalar_v129 = v129;
        let v130: f64 = p.p1668;
        self.scalar_v130 = v130;
        let v131: bool = (0.0 != p.p1668);
        self.scalar_v131 = v131;
        let v132: bool = (v129 && v131);
        self.scalar_v132 = v132;
        let v133: f64 = p.p1669;
        self.scalar_v133 = v133;
        let v134: f64 = (v116 * p.p1669);
        self.scalar_v134 = v134;
        let v135: f64 = (p.p1668 + v134);
        self.scalar_v135 = v135;
        let v136: f64 = p.p1670;
        self.scalar_v136 = v136;
        let v137: f64 = (v117 * p.p1670);
        self.scalar_v137 = v137;
        let v138: f64 = (v135 + v137);
        self.scalar_v138 = v138;
        let v139: f64 = p.p1671;
        self.scalar_v139 = v139;
        let v140: f64 = (v119 * p.p1671);
        self.scalar_v140 = v140;
        let v141: f64 = (v138 + v140);
        self.scalar_v141 = v141;
        let v142: f64 = p.p1672;
        self.scalar_v142 = v142;
        let v143: f64 = (v125 * p.p1672);
        self.scalar_v143 = v143;
        let v144: f64 = (v141 + v143);
        self.scalar_v144 = v144;
        let v145: f64 = p.p1673;
        self.scalar_v145 = v145;
        let v146: f64 = (v126 * p.p1673);
        self.scalar_v146 = v146;
        let v147: f64 = (v144 + v146);
        self.scalar_v147 = v147;
        let v148: f64 = (if v132 { v147 } else { 0.0 });
        self.scalar_v148 = v148;
        let v149: bool = (1.0 == p.p73);
        self.scalar_v149 = v149;
        let v150: bool = (0.0 != v148);
        self.scalar_v150 = v150;
        let v151: bool = (v149 && v150);
        self.scalar_v151 = v151;
        let v153: bool = (v148 < 0.001);
        self.scalar_v153 = v153;
        let v154: bool = (v151 && v153);
        self.scalar_v154 = v154;
        let v155: f64 = (if v154 { 0.0 } else { v148 });
        self.scalar_v155 = v155;
        let v156: bool = (0.0 != p.p74);
        self.scalar_v156 = v156;
        let v157: bool = (p.p1791 > 0.0);
        self.scalar_v157 = v157;
        let v158: bool = (v156 && v157);
        self.scalar_v158 = v158;
        let v159: f64 = p.p76;
        self.scalar_v159 = v159;
        let v160: bool = (0.0 != p.p76);
        self.scalar_v160 = v160;
        let v161: f64 = p.p1074;
        self.scalar_v161 = v161;
        let v162: f64 = p.p6;
        self.scalar_v162 = v162;
        let v163: f64 = (p.p1074 / p.p6);
        self.scalar_v163 = v163;
        let v164: f64 = p.p1075;
        self.scalar_v164 = v164;
        let v165: f64 = (p.p5 * p.p1075);
        self.scalar_v165 = v165;
        let v166: bool = (2.0 == p.p6);
        self.scalar_v166 = v166;
        let v168: f64 = (if v166 { 12.0 } else { 3.0 });
        self.scalar_v168 = v168;
        let v169: f64 = (v165 / v168);
        self.scalar_v169 = v169;
        let v170: f64 = (v163 + v169);
        self.scalar_v170 = v170;
        let v171: f64 = (v170 / p.p59);
        self.scalar_v171 = v171;
        let v172: f64 = (if v160 { v171 } else { 0.0 });
        self.scalar_v172 = v172;
        let v173: bool = (0.001 > v172);
        self.scalar_v173 = v173;
        let v174: f64 = (if v173 { 0.001 } else { v172 });
        self.scalar_v174 = v174;
        let v175: f64 = (1.0 / v174);
        self.scalar_v175 = v175;
        let v176: f64 = (if v160 { v175 } else { 0.0 });
        self.scalar_v176 = v176;
        let v177: bool = (2.0 == p.p76);
        self.scalar_v177 = v177;
        let v178: bool = (v160 && v177);
        self.scalar_v178 = v178;
        let v179: f64 = p.p1076;
        self.scalar_v179 = v179;
        let v180: bool = (0.001 > p.p1076);
        self.scalar_v180 = v180;
        let v181: f64 = (if v180 { 0.001 } else { p.p1076 });
        self.scalar_v181 = v181;
        let v182: f64 = (1.0 / v181);
        self.scalar_v182 = v182;
        let v183: f64 = (if v178 { v182 } else { 0.0 });
        self.scalar_v183 = v183;
        let v184: f64 = p.p1077;
        self.scalar_v184 = v184;
        let v185: bool = (0.001 > p.p1077);
        self.scalar_v185 = v185;
        let v186: f64 = (if v185 { 0.001 } else { p.p1077 });
        self.scalar_v186 = v186;
        let v187: f64 = (1.0 / v186);
        self.scalar_v187 = v187;
        let v188: f64 = (if v178 { v187 } else { 0.0 });
        self.scalar_v188 = v188;
        let v189: bool = (!v158);
        self.scalar_v189 = v189;
        let v194: bool = (2.0 == p.p73);
        self.scalar_v194 = v194;
        let v195: bool = (!v194);
        self.scalar_v195 = v195;
        let v196: f64 = p.p65;
        self.scalar_v196 = v196;
        let v197: bool = (1.0 == p.p65);
        self.scalar_v197 = v197;
        let v199: bool = (0.0 != v155);
        self.scalar_v199 = v199;
        let v200: f64 = p.p72;
        self.scalar_v200 = v200;
        let v201: bool = (0.0 == p.p72);
        self.scalar_v201 = v201;
        let v202: bool = (!v201);
        self.scalar_v202 = v202;
        let v203: bool = (v149 && v199);
        self.scalar_v203 = v203;
        let v204: bool = (!v177);
        self.scalar_v204 = v204;
        let v209: bool = (!v197);
        self.scalar_v209 = v209;
        let v210: f64 = (if v209 { 0.0 } else { 0.0 });
        self.scalar_v210 = v210;
        let v212: bool = (!v203);
        self.scalar_v212 = v212;
        let v213: f64 = (if v212 { 0.0 } else { 0.0 });
        self.scalar_v213 = v213;
        let v214: f64 = (if v195 { 0.0 } else { 0.0 });
        self.scalar_v214 = v214;
        let v228: bool = (v160 && v204);
        self.scalar_v228 = v228;
        let v229: f64 = (if v228 { 0.0 } else { 0.0 });
        self.scalar_v229 = v229;
        let v230: bool = (!v160);
        self.scalar_v230 = v230;
        let v231: f64 = (if v230 { 0.0 } else { 0.0 });
        self.scalar_v231 = v231;
        let v232: f64 = (if v201 { 0.0 } else { 0.0 });
        self.scalar_v232 = v232;
        let v235: f64 = (if v189 { 0.0 } else { 0.0 });
        self.scalar_v235 = v235;
        let v237: f64 = (if v197 { -1000.0 } else { 0.0 });
        self.scalar_v237 = v237;
        let v238: f64 = (if v197 { 1000.0 } else { 0.0 });
        self.scalar_v238 = v238;
        let v239: f64 = (-v176);
        self.scalar_v239 = v239;
        let v240: f64 = (if v160 { v176 } else { 0.0 });
        self.scalar_v240 = v240;
        let v241: f64 = (if v160 { v239 } else { 0.0 });
        self.scalar_v241 = v241;
        let v242: f64 = (-v183);
        self.scalar_v242 = v242;
        let v243: f64 = (if v178 { v183 } else { 0.0 });
        self.scalar_v243 = v243;
        let v244: f64 = (if v178 { v242 } else { 0.0 });
        self.scalar_v244 = v244;
        let v245: f64 = (-v188);
        self.scalar_v245 = v245;
        let v246: f64 = (if v178 { v188 } else { 0.0 });
        self.scalar_v246 = v246;
        let v247: f64 = (if v178 { v245 } else { 0.0 });
        self.scalar_v247 = v247;
        let v248: f64 = (if v202 { 1.0 } else { 0.0 });
        self.scalar_v248 = v248;
    }
}
