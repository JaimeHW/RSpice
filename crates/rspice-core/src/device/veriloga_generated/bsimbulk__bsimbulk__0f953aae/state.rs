#![allow(dead_code, unused_parens, unused_variables)]

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
}

impl Copy for Parameters {}

impl Clone for Parameters {
    #[inline]
    fn clone(&self) -> Self { *self }
}

impl Default for Parameters {
    fn default() -> Self {
        // SAFETY: every generated Parameters field is f64; all-zero bytes are a valid 0.0 value for f64.
        let mut params: Self = unsafe { std::mem::zeroed::<Self>() };
        params.p0 = 1e-5;
        params.p1 = 1e-5;
        params.p2 = 1.0;
        params.p3 = 1.0;
        params.p4 = 1.0;
        params.p5 = 0.0;
        params.p6 = 0.0;
        params.p7 = 0.0;
        params.p8 = 0.0;
        params.p9 = 0.0;
        params.p10 = 0.0;
        params.p11 = 50.0;
        params.p12 = 50.0;
        params.p13 = 50.0;
        params.p14 = 50.0;
        params.p15 = 50.0;
        params.p16 = 50.0;
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
        params.p28 = 1.0;
        params.p29 = 1.0;
        params.p30 = params.p28;
        validate_parameter("MULT_FN", params.p30, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p31 = 0.0;
        params.p32 = 1.0;
        params.p33 = 0.0;
        params.p34 = 1.0;
        params.p35 = 0.0;
        params.p36 = 1.0;
        params.p37 = 0.0;
        params.p38 = 0.0;
        params.p39 = 1.0;
        params.p40 = 0.0;
        params.p41 = 0.0;
        params.p42 = 0.0;
        params.p43 = 0.0;
        params.p44 = 0.0;
        params.p45 = 0.0;
        params.p46 = 0.0;
        params.p47 = 0.0;
        params.p48 = 0.0;
        params.p49 = 0.0;
        params.p50 = 0.0;
        params.p51 = 1e-5;
        params.p52 = 1.0;
        params.p53 = 1.0;
        params.p54 = 0.0;
        params.p55 = 1e-5;
        params.p56 = 0.0;
        params.p57 = 0.0;
        params.p58 = 0.0;
        params.p59 = 0.0;
        params.p60 = 0.0;
        params.p61 = 1.0;
        params.p62 = 1.0;
        params.p63 = 0.0;
        params.p64 = 0.0;
        params.p65 = 0.0;
        params.p66 = 0.0;
        params.p67 = 1.0;
        params.p68 = 1.0;
        params.p69 = 0.0;
        params.p70 = 0.0;
        params.p71 = 0.0;
        params.p72 = 0.0;
        params.p73 = 0.0;
        params.p74 = 0.0;
        params.p75 = 0.0;
        params.p76 = 0.0;
        params.p77 = 3e-9;
        params.p78 = params.p77;
        validate_parameter("TOXP", params.p78, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p79 = 0.0;
        params.p80 = 1e24;
        params.p81 = 0.0;
        params.p82 = 1.0;
        params.p83 = 0.0;
        params.p84 = 2.0;
        params.p85 = 0.0;
        params.p86 = 1.0;
        params.p87 = 0.0;
        params.p88 = 1.0;
        params.p89 = 0.0;
        params.p90 = 0.0;
        params.p91 = 0.0;
        params.p92 = params.p80;
        validate_finite_parameter("NDEPCV", params.p92).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p93 = params.p81;
        validate_finite_parameter("NDEPCVL1", params.p93).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p94 = params.p82;
        validate_parameter("NDEPCVLEXP1", params.p94, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p95 = params.p83;
        validate_finite_parameter("NDEPCVL2", params.p95).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p96 = params.p84;
        validate_parameter("NDEPCVLEXP2", params.p96, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p97 = params.p85;
        validate_finite_parameter("NDEPCVW", params.p97).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p98 = params.p86;
        validate_parameter("NDEPCVWEXP", params.p98, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p99 = params.p87;
        validate_finite_parameter("NDEPCVWL", params.p99).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p100 = params.p88;
        validate_parameter("NDEPCVWLEXP", params.p100, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p101 = params.p89;
        validate_finite_parameter("LNDEPCV", params.p101).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p102 = params.p90;
        validate_finite_parameter("WNDEPCV", params.p102).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p103 = params.p91;
        validate_finite_parameter("PNDEPCV", params.p103).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p104 = 5e25;
        params.p105 = 0.0;
        params.p106 = 0.0;
        params.p107 = 0.0;
        params.p108 = 1.1e16;
        params.p109 = 1.17;
        params.p110 = 11.9;
        params.p111 = 3.9;
        params.p112 = 1.5e-7;
        params.p113 = 0.0;
        params.p114 = 0.0;
        params.p115 = 0.0;
        params.p116 = -0.5;
        params.p117 = 0.0;
        params.p118 = 0.0;
        params.p119 = 0.0;
        params.p120 = 0.0;
        params.p121 = 1.0;
        params.p122 = 0.0;
        params.p123 = 1.0;
        params.p124 = 0.0;
        params.p125 = 1.0;
        params.p126 = params.p116;
        validate_finite_parameter("VFBCV", params.p126).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p127 = params.p117;
        validate_finite_parameter("LVFBCV", params.p127).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p128 = params.p118;
        validate_finite_parameter("WVFBCV", params.p128).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p129 = params.p119;
        validate_finite_parameter("PVFBCV", params.p129).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p130 = params.p120;
        validate_finite_parameter("VFBCVL", params.p130).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p131 = params.p121;
        validate_parameter("VFBCVLEXP", params.p131, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p132 = params.p122;
        validate_finite_parameter("VFBCVW", params.p132).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p133 = params.p123;
        validate_parameter("VFBCVWEXP", params.p133, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p134 = params.p124;
        validate_finite_parameter("VFBCVWL", params.p134).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p135 = params.p125;
        validate_parameter("VFBCVWLEXP", params.p135, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p136 = 0.0;
        params.p137 = 1.0;
        params.p138 = params.p73;
        validate_finite_parameter("DWJ", params.p138).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p139 = 1e26;
        params.p140 = 0.0;
        params.p141 = 0.0;
        params.p142 = 0.0;
        params.p143 = 0.0;
        params.p144 = 0.0;
        params.p145 = 0.0;
        params.p146 = 0.0;
        params.p147 = 0.0;
        params.p148 = 0.0;
        params.p149 = 0.0;
        params.p150 = 0.0;
        params.p151 = 0.0;
        params.p152 = 0.0;
        params.p153 = 0.0;
        params.p154 = 0.0;
        params.p155 = 0.0;
        params.p156 = 0.0;
        params.p157 = 0.0;
        params.p158 = 0.0;
        params.p159 = 0.0;
        params.p160 = 0.0;
        params.p161 = 0.0;
        params.p162 = 0.0;
        params.p163 = 0.0;
        params.p164 = 0.0;
        params.p165 = 0.0;
        params.p166 = 0.0;
        params.p167 = 0.045;
        params.p168 = 0.0;
        params.p169 = 0.0;
        params.p170 = 0.0;
        params.p171 = 0.08;
        params.p172 = 0.0;
        params.p173 = 0.0;
        params.p174 = 0.0;
        params.p175 = params.p171;
        validate_finite_parameter("ETA0R", params.p175).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p176 = params.p172;
        validate_finite_parameter("LETA0R", params.p176).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p177 = params.p173;
        validate_finite_parameter("WETA0R", params.p177).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p178 = params.p174;
        validate_finite_parameter("PETA0R", params.p178).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p179 = 1.0;
        params.p180 = -0.07;
        params.p181 = 1.0;
        params.p182 = 0.0;
        params.p183 = 0.0;
        params.p184 = 0.0;
        params.p185 = 0.0;
        params.p186 = 0.0;
        params.p187 = 1.0;
        params.p188 = 0.0;
        params.p189 = 1.0;
        params.p190 = 0.0;
        params.p191 = 1.0;
        params.p192 = 0.0;
        params.p193 = 0.0;
        params.p194 = 0.0;
        params.p195 = 0.0;
        params.p196 = 0.0;
        params.p197 = 1.0;
        params.p198 = 0.0;
        params.p199 = 1.0;
        params.p200 = 0.0;
        params.p201 = 1.0;
        params.p202 = 0.0;
        params.p203 = 0.0;
        params.p204 = 0.0;
        params.p205 = 0.0;
        params.p206 = 1.0;
        params.p207 = 0.001;
        params.p208 = 0.54;
        params.p209 = 0.0;
        params.p210 = 0.0;
        params.p211 = 0.0;
        params.p212 = 0.0;
        params.p213 = 0.0;
        params.p214 = 0.0;
        params.p215 = 1.0;
        params.p216 = 0.0;
        params.p217 = 1.0;
        params.p218 = 0.0;
        params.p219 = 1.0;
        params.p220 = 0.0;
        params.p221 = 0.0;
        params.p222 = 0.0;
        params.p223 = 1e-9;
        params.p224 = 0.0;
        params.p225 = 1.0;
        params.p226 = 0.0;
        params.p227 = 0.0;
        params.p228 = 0.0;
        params.p229 = params.p223;
        validate_finite_parameter("CDSCDR", params.p229).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p230 = params.p226;
        validate_finite_parameter("LCDSCDR", params.p230).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p231 = params.p227;
        validate_finite_parameter("WCDSCDR", params.p231).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p232 = params.p228;
        validate_finite_parameter("PCDSCDR", params.p232).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p233 = 0.0;
        params.p234 = 0.0;
        params.p235 = 1.0;
        params.p236 = 0.0;
        params.p237 = 0.0;
        params.p238 = 0.0;
        params.p239 = 100000.0;
        params.p240 = 0.0;
        params.p241 = 0.0;
        params.p242 = 0.0;
        params.p243 = 0.0;
        params.p244 = 1.0;
        params.p245 = 0.0;
        params.p246 = 1.0;
        params.p247 = 0.0;
        params.p248 = 1.0;
        params.p249 = params.p239;
        validate_finite_parameter("VSATR", params.p249).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p250 = params.p240;
        validate_finite_parameter("LVSATR", params.p250).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p251 = params.p241;
        validate_finite_parameter("WVSATR", params.p251).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p252 = params.p242;
        validate_finite_parameter("PVSATR", params.p252).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p253 = 0.125;
        params.p254 = 0.0;
        params.p255 = 0.0;
        params.p256 = 0.0;
        params.p257 = 0.0;
        params.p258 = 1.0;
        params.p259 = params.p239;
        validate_finite_parameter("VSATCV", params.p259).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p260 = params.p240;
        validate_finite_parameter("LVSATCV", params.p260).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p261 = params.p241;
        validate_finite_parameter("WVSATCV", params.p261).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p262 = params.p242;
        validate_finite_parameter("PVSATCV", params.p262).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p263 = params.p243;
        validate_finite_parameter("VSATCVL", params.p263).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p264 = params.p244;
        validate_parameter("VSATCVLEXP", params.p264, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p265 = params.p245;
        validate_finite_parameter("VSATCVW", params.p265).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p266 = params.p246;
        validate_parameter("VSATCVWEXP", params.p266, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p267 = params.p247;
        validate_finite_parameter("VSATCVWL", params.p267).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p268 = params.p248;
        validate_parameter("VSATCVWLEXP", params.p268, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p269 = 0.0;
        params.p270 = 1e-8;
        params.p271 = 0.0;
        params.p272 = 1e-8;
        params.p273 = 0.067;
        params.p274 = 0.0;
        params.p275 = 1.0;
        params.p276 = 0.0;
        params.p277 = 0.0;
        params.p278 = 0.0;
        params.p279 = params.p273;
        validate_finite_parameter("U0R", params.p279).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p280 = params.p276;
        validate_finite_parameter("LU0R", params.p280).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p281 = params.p277;
        validate_finite_parameter("WU0R", params.p281).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p282 = params.p278;
        validate_finite_parameter("PU0R", params.p282).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p283 = 1.0;
        params.p284 = 0.001;
        params.p285 = 0.0;
        params.p286 = 1.0;
        params.p287 = 0.0;
        params.p288 = 1.0;
        params.p289 = 0.0;
        params.p290 = 1.0;
        params.p291 = 0.0;
        params.p292 = 0.0;
        params.p293 = 0.0;
        params.p294 = params.p284;
        validate_finite_parameter("UAR", params.p294).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p295 = params.p291;
        validate_finite_parameter("LUAR", params.p295).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p296 = params.p292;
        validate_finite_parameter("WUAR", params.p296).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p297 = params.p293;
        validate_finite_parameter("PUAR", params.p297).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p298 = 1.5;
        params.p299 = 0.0;
        params.p300 = 0.0;
        params.p301 = 0.0;
        params.p302 = 0.0;
        params.p303 = 1.0;
        params.p304 = 0.0;
        params.p305 = 1.0;
        params.p306 = 0.0;
        params.p307 = 1.0;
        params.p308 = 0.001;
        params.p309 = 0.0;
        params.p310 = 1.0;
        params.p311 = 0.0;
        params.p312 = 0.0;
        params.p313 = 0.0;
        params.p314 = params.p308;
        validate_finite_parameter("UDR", params.p314).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p315 = params.p311;
        validate_finite_parameter("LUDR", params.p315).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p316 = params.p312;
        validate_finite_parameter("WUDR", params.p316).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p317 = params.p313;
        validate_finite_parameter("PUDR", params.p317).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p318 = 2.0;
        params.p319 = 0.0;
        params.p320 = 0.0;
        params.p321 = 0.0;
        params.p322 = params.p318;
        validate_finite_parameter("UCSR", params.p322).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p323 = params.p319;
        validate_finite_parameter("LUCSR", params.p323).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p324 = params.p320;
        validate_finite_parameter("WUCSR", params.p324).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p325 = params.p321;
        validate_finite_parameter("PUCSR", params.p325).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p326 = 0.0;
        params.p327 = 0.0;
        params.p328 = 1.0;
        params.p329 = 0.0;
        params.p330 = 1.0;
        params.p331 = 0.0;
        params.p332 = 1.0;
        params.p333 = 0.0;
        params.p334 = 0.0;
        params.p335 = 0.0;
        params.p336 = params.p326;
        validate_finite_parameter("UCR", params.p336).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p337 = params.p333;
        validate_finite_parameter("LUCR", params.p337).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p338 = params.p334;
        validate_finite_parameter("WUCR", params.p338).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p339 = params.p335;
        validate_finite_parameter("PUCR", params.p339).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p340 = 0.0;
        params.p341 = 0.0;
        params.p342 = 1.0;
        params.p343 = 0.0;
        params.p344 = 0.0;
        params.p345 = 0.0;
        params.p346 = params.p340;
        validate_finite_parameter("PCLMR", params.p346).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p347 = params.p343;
        validate_finite_parameter("LPCLMR", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p348 = params.p344;
        validate_finite_parameter("WPCLMR", params.p348).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p349 = params.p345;
        validate_finite_parameter("PPCLMR", params.p349).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p350 = 0.0;
        params.p351 = params.p340;
        validate_finite_parameter("PCLMCV", params.p351).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p352 = params.p341;
        validate_finite_parameter("PCLMCVL", params.p352).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p353 = params.p342;
        validate_parameter("PCLMCVLEXP", params.p353, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p354 = params.p343;
        validate_finite_parameter("LPCLMCV", params.p354).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p355 = params.p344;
        validate_finite_parameter("WPCLMCV", params.p355).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p356 = params.p345;
        validate_finite_parameter("PPCLMCV", params.p356).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p357 = 424000000.0;
        params.p358 = 0.0;
        params.p359 = 0.0;
        params.p360 = 0.0;
        params.p361 = 1e-8;
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
        params.p375 = 1.0;
        params.p376 = 0.0;
        params.p377 = 0.0;
        params.p378 = 0.0;
        params.p379 = 0.0;
        params.p380 = 0.0;
        params.p381 = 0.0;
        params.p382 = 0.0;
        params.p383 = 0.0;
        params.p384 = 1.0;
        params.p385 = 1.0;
        params.p386 = 0.0;
        params.p387 = 0.0;
        params.p388 = 0.0;
        params.p389 = 0.0;
        params.p390 = 0.0;
        params.p391 = 0.0;
        params.p392 = 0.0;
        params.p393 = 10.0;
        params.p394 = 0.0;
        params.p395 = 0.0;
        params.p396 = 0.0;
        params.p397 = 0.0;
        params.p398 = 1.0;
        params.p399 = params.p389;
        validate_finite_parameter("RDWMIN", params.p399).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p400 = params.p390;
        validate_finite_parameter("LRDWMIN", params.p400).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p401 = params.p391;
        validate_finite_parameter("WRDWMIN", params.p401).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p402 = params.p392;
        validate_finite_parameter("PRDWMIN", params.p402).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p403 = params.p393;
        validate_finite_parameter("RDW", params.p403).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p404 = params.p394;
        validate_finite_parameter("LRDW", params.p404).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p405 = params.p395;
        validate_finite_parameter("WRDW", params.p405).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p406 = params.p396;
        validate_finite_parameter("PRDW", params.p406).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p407 = params.p397;
        validate_finite_parameter("RDWL", params.p407).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p408 = params.p398;
        validate_parameter("RDWLEXP", params.p408, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p409 = 0.0;
        params.p410 = 0.0;
        params.p411 = 0.0;
        params.p412 = 0.0;
        params.p413 = 20.0;
        params.p414 = 0.0;
        params.p415 = 1.0;
        params.p416 = 0.0;
        params.p417 = 0.0;
        params.p418 = 0.0;
        params.p419 = 1.0;
        params.p420 = 0.0;
        params.p421 = 0.0;
        params.p422 = 0.0;
        params.p423 = 0.0;
        params.p424 = 1.0;
        params.p425 = 0.0;
        params.p426 = params.p419;
        validate_finite_parameter("PSATR", params.p426).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p427 = params.p420;
        validate_finite_parameter("LPSATR", params.p427).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p428 = params.p421;
        validate_finite_parameter("WPSATR", params.p428).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p429 = params.p422;
        validate_finite_parameter("PPSATR", params.p429).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p430 = 0.0;
        params.p431 = 0.0;
        params.p432 = 0.0;
        params.p433 = 1.0;
        params.p434 = 0.0;
        params.p435 = 0.0;
        params.p436 = 0.0;
        params.p437 = 0.0;
        params.p438 = 0.0;
        params.p439 = 1.0;
        params.p440 = params.p434;
        validate_finite_parameter("PTWGR", params.p440).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p441 = params.p435;
        validate_finite_parameter("LPTWGR", params.p441).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p442 = params.p436;
        validate_finite_parameter("WPTWGR", params.p442).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p443 = params.p437;
        validate_finite_parameter("PPTWGR", params.p443).expect("generated Verilog-A parameter default must satisfy declared range");
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
        params.p462 = 1.0;
        params.p463 = 0.0;
        params.p464 = 0.0;
        params.p465 = 0.0;
        params.p466 = params.p460;
        validate_finite_parameter("PDIBLCR", params.p466).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p467 = params.p463;
        validate_finite_parameter("LPDIBLCR", params.p467).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p468 = params.p464;
        validate_finite_parameter("WPDIBLCR", params.p468).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p469 = params.p465;
        validate_finite_parameter("PPDIBLCR", params.p469).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p470 = 0.0;
        params.p471 = 0.0;
        params.p472 = 0.0;
        params.p473 = 0.0;
        params.p474 = 1.0;
        params.p475 = 0.0;
        params.p476 = 0.0;
        params.p477 = 0.0;
        params.p478 = 0.0;
        params.p479 = 0.0;
        params.p480 = 1.0;
        params.p481 = 0.0;
        params.p482 = 0.0;
        params.p483 = 0.0;
        params.p484 = 0.0;
        params.p485 = 0.0;
        params.p486 = 1.0;
        params.p487 = 0.0;
        params.p488 = 1.0;
        params.p489 = 0.0;
        params.p490 = 0.0;
        params.p491 = 0.0;
        params.p492 = 0.0;
        params.p493 = 0.0;
        params.p494 = 0.0;
        params.p495 = 0.0;
        params.p496 = 1.0;
        params.p497 = 0.0;
        params.p498 = 0.0;
        params.p499 = 0.0;
        params.p500 = params.p484;
        validate_finite_parameter("ALPHADR", params.p500).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p501 = params.p494;
        validate_finite_parameter("BETADR", params.p501).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p502 = 1.0;
        params.p503 = 5.0;
        params.p504 = 0.5;
        params.p505 = 0.0;
        params.p506 = 0.0;
        params.p507 = 0.0;
        params.p508 = 0.0;
        params.p509 = 0.0;
        params.p510 = 0.0;
        params.p511 = 1.0;
        params.p512 = 1.0;
        params.p513 = 0.0;
        params.p514 = 0.0;
        params.p515 = 1.0;
        params.p516 = 0.0;
        params.p517 = 1.0;
        params.p518 = 0.0;
        params.p519 = 0.0;
        params.p520 = 1.0;
        params.p521 = 0.0;
        params.p522 = 0.0;
        params.p523 = 1.0;
        params.p524 = 1.0;
        params.p525 = params.p484;
        validate_finite_parameter("ALPHA0R", params.p525).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p526 = params.p489;
        validate_finite_parameter("LALPHA0R", params.p526).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p527 = params.p490;
        validate_finite_parameter("WALPHA0R", params.p527).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p528 = params.p491;
        validate_finite_parameter("PALPHA0R", params.p528).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p529 = params.p494;
        validate_finite_parameter("BETA0R", params.p529).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p530 = params.p497;
        validate_finite_parameter("LBETA0R", params.p530).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p531 = params.p498;
        validate_finite_parameter("WBETA0R", params.p531).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p532 = params.p499;
        validate_finite_parameter("PBETA0R", params.p532).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p533 = 0.0136;
        params.p534 = 0.00171;
        params.p535 = 0.075;
        params.p536 = 1.0;
        params.p537 = 0.0111;
        params.p538 = 0.000949;
        params.p539 = 0.006;
        params.p540 = 1.1;
        params.p541 = 3.0;
        params.p542 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
        validate_finite_parameter("AIGC", params.p542).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p543 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
        validate_finite_parameter("BIGC", params.p543).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p544 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
        validate_finite_parameter("CIGC", params.p544).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p545 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
        validate_finite_parameter("AIGS", params.p545).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p546 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
        validate_finite_parameter("BIGS", params.p546).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p547 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
        validate_finite_parameter("CIGS", params.p547).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p548 = if (params.p39 == 1.0) { 0.0136 } else { 0.0098 };
        validate_finite_parameter("AIGD", params.p548).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p549 = if (params.p39 == 1.0) { 0.00171 } else { 0.000759 };
        validate_finite_parameter("BIGD", params.p549).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p550 = if (params.p39 == 1.0) { 0.075 } else { 0.03 };
        validate_finite_parameter("CIGD", params.p550).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p551 = params.p57;
        validate_finite_parameter("DLCIG", params.p551).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p552 = params.p551;
        validate_finite_parameter("DLCIGD", params.p552).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p553 = 1.0;
        params.p554 = 1.0;
        params.p555 = 3e-9;
        params.p556 = 1.0;
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
        params.p636 = 2300000000.0;
        params.p637 = 0.0;
        params.p638 = 0.0;
        params.p639 = 0.0;
        params.p640 = 0.5;
        params.p641 = 0.0;
        params.p642 = 0.0;
        params.p643 = 0.0;
        params.p644 = 0.8;
        params.p645 = 0.0;
        params.p646 = 0.0;
        params.p647 = 0.0;
        params.p648 = params.p630;
        validate_finite_parameter("AGISL", params.p648).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p649 = params.p631;
        validate_finite_parameter("AGISLL", params.p649).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p650 = params.p632;
        validate_finite_parameter("AGISLW", params.p650).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p651 = params.p633;
        validate_finite_parameter("LAGISL", params.p651).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p652 = params.p634;
        validate_finite_parameter("WAGISL", params.p652).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p653 = params.p635;
        validate_finite_parameter("PAGISL", params.p653).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p654 = params.p636;
        validate_finite_parameter("BGISL", params.p654).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p655 = params.p637;
        validate_finite_parameter("LBGISL", params.p655).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p656 = params.p638;
        validate_finite_parameter("WBGISL", params.p656).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p657 = params.p639;
        validate_finite_parameter("PBGISL", params.p657).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p658 = params.p640;
        validate_finite_parameter("CGISL", params.p658).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p659 = params.p641;
        validate_finite_parameter("LCGISL", params.p659).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p660 = params.p642;
        validate_finite_parameter("WCGISL", params.p660).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p661 = params.p643;
        validate_finite_parameter("PCGISL", params.p661).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p662 = params.p644;
        validate_finite_parameter("EGISL", params.p662).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p663 = params.p645;
        validate_finite_parameter("LEGISL", params.p663).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p664 = params.p646;
        validate_finite_parameter("WEGISL", params.p664).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p665 = params.p647;
        validate_finite_parameter("PEGISL", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p666 = 0.0;
        params.p667 = 0.0;
        params.p668 = 0.0;
        params.p669 = 0.0;
        params.p670 = 1.0;
        params.p671 = 0.0;
        params.p672 = 0.0;
        params.p673 = 0.0;
        params.p674 = 0.0;
        params.p675 = 0.0;
        params.p676 = 0.0;
        params.p677 = 0.0;
        params.p678 = 0.0;
        params.p679 = 0.0;
        params.p680 = 0.0;
        params.p681 = 0.0;
        params.p682 = 0.6;
        params.p683 = 0.0;
        params.p684 = 0.0;
        params.p685 = 0.0;
        params.p686 = 0.6;
        params.p687 = 0.0;
        params.p688 = 0.0;
        params.p689 = 0.0;
        params.p690 = 1000000.0;
        params.p691 = 1.0;
        params.p692 = 1000000.0;
        params.p693 = 1.0;
        params.p694 = 0.1;
        params.p695 = 0.0;
        params.p696 = params.p695;
        validate_parameter("DMCI", params.p696, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p697 = 0.0;
        params.p698 = 0.0;
        params.p699 = 0.0;
        params.p700 = 0.1;
        params.p701 = 0.0005;
        params.p702 = params.p701;
        validate_finite_parameter("CJD", params.p702).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p703 = 5e-10;
        params.p704 = params.p703;
        validate_finite_parameter("CJSWD", params.p704).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p705 = 0.0;
        params.p706 = params.p705;
        validate_finite_parameter("CJSWGD", params.p706).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p707 = 1.0;
        params.p708 = params.p707;
        validate_finite_parameter("PBD", params.p708).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p709 = 1.0;
        params.p710 = params.p709;
        validate_finite_parameter("PBSWD", params.p710).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p711 = params.p709;
        validate_finite_parameter("PBSWGS", params.p711).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p712 = params.p711;
        validate_finite_parameter("PBSWGD", params.p712).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p713 = 0.5;
        params.p714 = params.p713;
        validate_finite_parameter("MJD", params.p714).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p715 = 0.33;
        params.p716 = params.p715;
        validate_finite_parameter("MJSWD", params.p716).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p717 = params.p715;
        validate_finite_parameter("MJSWGS", params.p717).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p718 = params.p717;
        validate_finite_parameter("MJSWGD", params.p718).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p719 = 0.0001;
        params.p720 = params.p719;
        validate_finite_parameter("JSD", params.p720).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p721 = 0.0;
        params.p722 = params.p721;
        validate_finite_parameter("JSWD", params.p722).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p723 = 0.0;
        params.p724 = params.p723;
        validate_finite_parameter("JSWGD", params.p724).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p725 = 1.0;
        params.p726 = params.p725;
        validate_parameter("NJD", params.p726, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p727 = 0.1;
        params.p728 = params.p727;
        validate_finite_parameter("IJTHDFWD", params.p728).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p729 = 0.1;
        params.p730 = params.p729;
        validate_finite_parameter("IJTHDREV", params.p730).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p731 = 10.0;
        params.p732 = params.p731;
        validate_finite_parameter("BVD", params.p732).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p733 = 1.0;
        params.p734 = params.p733;
        validate_parameter("XJBVD", params.p734, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p735 = 0.0;
        params.p736 = params.p735;
        validate_finite_parameter("JTSD", params.p736).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p737 = 0.0;
        params.p738 = params.p737;
        validate_finite_parameter("JTSSWD", params.p738).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p739 = 0.0;
        params.p740 = params.p739;
        validate_finite_parameter("JTSSWGD", params.p740).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p741 = 0.0;
        params.p742 = 20.0;
        params.p743 = params.p742;
        validate_finite_parameter("NJTSD", params.p743).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p744 = 20.0;
        params.p745 = params.p744;
        validate_finite_parameter("NJTSSWD", params.p745).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p746 = 20.0;
        params.p747 = params.p746;
        validate_finite_parameter("NJTSSWGD", params.p747).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p748 = 10.0;
        params.p749 = params.p748;
        validate_finite_parameter("VTSD", params.p749).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p750 = 10.0;
        params.p751 = params.p750;
        validate_finite_parameter("VTSSWD", params.p751).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p752 = 10.0;
        params.p753 = params.p752;
        validate_finite_parameter("VTSSWGD", params.p753).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p754 = 12.0;
        params.p755 = 1.0;
        params.p756 = 1e-12;
        params.p757 = 50.0;
        params.p758 = 0.0;
        params.p759 = 0.0;
        params.p760 = 0.0;
        params.p761 = 50.0;
        params.p762 = 0.0;
        params.p763 = 0.0;
        params.p764 = 0.0;
        params.p765 = 100.0;
        params.p766 = 0.0;
        params.p767 = 0.0;
        params.p768 = 0.0;
        params.p769 = 100.0;
        params.p770 = 0.0;
        params.p771 = 0.0;
        params.p772 = 0.0;
        params.p773 = 100.0;
        params.p774 = 100.0;
        params.p775 = 100.0;
        params.p776 = 100.0;
        params.p777 = 0.0;
        params.p778 = 0.0;
        params.p779 = 0.0;
        params.p780 = 0.0;
        params.p781 = 0.0;
        params.p782 = 0.0;
        params.p783 = 1.0;
        params.p784 = 41000000.0;
        params.p785 = 6.25e40;
        params.p786 = 0.0;
        params.p787 = 0.0;
        params.p788 = 0.0;
        params.p789 = 0.0;
        params.p790 = 1.2;
        params.p791 = 0.0;
        params.p792 = 0.0;
        params.p793 = 0.0;
        params.p794 = 0.05;
        params.p795 = 0.0;
        params.p796 = 0.0;
        params.p797 = 0.0;
        params.p798 = 2.0;
        params.p799 = 3.125e25;
        params.p800 = 875000000.0;
        params.p801 = 0.0;
        params.p802 = 0.0;
        params.p803 = 1.0;
        params.p804 = 1.0;
        params.p805 = 1.0;
        params.p806 = 0.0;
        params.p807 = 0.0;
        params.p808 = 2.0;
        params.p809 = 2.0;
        params.p810 = 1.0;
        params.p811 = 0.577;
        params.p812 = 0.5164;
        params.p813 = 0.395;
        params.p814 = 1.5;
        params.p815 = 3.5;
        params.p816 = 0.0;
        params.p817 = 1.0;
        params.p818 = 0.0;
        params.p819 = 0.0;
        params.p820 = 27.0;
        params.p821 = 0.000473;
        params.p822 = 636.0;
        params.p823 = 0.0;
        params.p824 = -1.5;
        params.p825 = 0.0;
        params.p826 = 0.0;
        params.p827 = 0.0;
        params.p828 = 0.0;
        params.p829 = 0.001;
        params.p830 = 0.0;
        params.p831 = 0.0;
        params.p832 = 0.0;
        params.p833 = 0.0;
        params.p834 = 5.6e-11;
        params.p835 = 0.0;
        params.p836 = 0.0;
        params.p837 = 0.0;
        params.p838 = 0.0;
        params.p839 = 0.0;
        params.p840 = 0.0;
        params.p841 = 0.0;
        params.p842 = 0.0;
        params.p843 = 0.0;
        params.p844 = 0.0;
        params.p845 = 0.0;
        params.p846 = 0.0;
        params.p847 = -0.004775;
        params.p848 = 0.0;
        params.p849 = 0.0;
        params.p850 = 0.0;
        params.p851 = 0.0;
        params.p852 = 0.0;
        params.p853 = 0.0;
        params.p854 = 0.0;
        params.p855 = 0.0;
        params.p856 = -0.00156;
        params.p857 = 0.0;
        params.p858 = 0.0;
        params.p859 = 0.0;
        params.p860 = 0.0;
        params.p861 = 0.0;
        params.p862 = 0.0;
        params.p863 = 0.0;
        params.p864 = 0.0;
        params.p865 = 0.0;
        params.p866 = 0.0;
        params.p867 = -0.11;
        params.p868 = 1.0;
        params.p869 = 0.0;
        params.p870 = 0.0;
        params.p871 = 0.0;
        params.p872 = 0.0;
        params.p873 = 0.022;
        params.p874 = 0.0;
        params.p875 = 0.0;
        params.p876 = 0.0;
        params.p877 = 0.0;
        params.p878 = 0.0;
        params.p879 = 0.0;
        params.p880 = 0.0;
        params.p881 = 2.5;
        params.p882 = 0.0;
        params.p883 = 0.0;
        params.p884 = 0.0;
        params.p885 = 0.0;
        params.p886 = 0.0;
        params.p887 = 0.0;
        params.p888 = 0.0;
        params.p889 = 0.0;
        params.p890 = 0.0;
        params.p891 = 0.0;
        params.p892 = 0.0;
        params.p893 = 0.0;
        params.p894 = 0.0;
        params.p895 = 3.0;
        params.p896 = params.p895;
        validate_finite_parameter("XTID", params.p896).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p897 = 0.02;
        params.p898 = params.p897;
        validate_finite_parameter("XTSD", params.p898).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p899 = 0.02;
        params.p900 = params.p899;
        validate_finite_parameter("XTSSWD", params.p900).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p901 = 0.02;
        params.p902 = params.p901;
        validate_finite_parameter("XTSSWGD", params.p902).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p903 = 0.0;
        params.p904 = params.p903;
        validate_finite_parameter("TNJTSD", params.p904).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p905 = 0.0;
        params.p906 = params.p905;
        validate_finite_parameter("TNJTSSWD", params.p906).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p907 = 0.0;
        params.p908 = params.p907;
        validate_finite_parameter("TNJTSSWGD", params.p908).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p909 = 0.0;
        params.p910 = 1e-5;
        params.p911 = 0.0;
        params.p912 = 1e-6;
        params.p913 = 1e-6;
        params.p914 = 0.0;
        params.p915 = 0.0;
        params.p916 = 0.0;
        params.p917 = 0.0;
        params.p918 = 0.0;
        params.p919 = 0.0;
        params.p920 = 0.0;
        params.p921 = 0.0;
        params.p922 = 0.0;
        params.p923 = 0.0;
        params.p924 = 0.0;
        params.p925 = 0.0;
        params.p926 = 0.0;
        params.p927 = 0.0;
        params.p928 = 0.0;
        params.p929 = 0.0;
        params.p930 = 0.0;
        params.p931 = 0.0;
        params.p932 = 0.0;
        params.p933 = 0.0;
        params.p934 = 0.0;
        params.p935 = 0.0;
        params.p936 = 0.0;
        params.p937 = 0.0;
        params.p938 = 0.0;
        params.p939 = 0.0;
        params.p940 = 0.0;
        params.p941 = 0.0;
        params.p942 = 0.0;
        params.p943 = 0.0;
        params.p944 = 0.0;
        params.p945 = 0.0;
        params.p946 = 0.0;
        params.p947 = 1e-6;
        params.p948 = 400.0;
        params.p949 = 336000000.0;
        params.p950 = 0.185;
        params.p951 = 0.3;
        params.p952 = 1.4;
        params.p953 = 0.0;
        params.p954 = 0.49;
        params.p955 = 1.42;
        params.p956 = 20.0;
        params.p957 = 1e-8;
        params.p958 = 0.0;
        params.p959 = 0.0;
        params.p960 = 1.0;
        params.p961 = 0.0;
        params.p962 = 1e24;
        params.p963 = 0.0;
        params.p964 = 0.0;
        params.p965 = 0.0;
        params.p966 = 0.0;
        params.p967 = 0.0;
        params.p968 = 0.0;
        params.p969 = 0.0;
        params.p970 = 0.0;
        params.p971 = 0.0;
        params.p972 = 0.0;
        params.p973 = 0.0;
        params.p974 = 1e-9;
        params.p975 = 0.0;
        params.p976 = 0.0;
        params.p977 = 0.0;
        params.p978 = 0.0;
        params.p979 = 0.0;
        params.p980 = 0.0;
        params.p981 = 0.0;
        params.p982 = 0.08;
        params.p983 = 0.0;
        params.p984 = 0.0;
        params.p985 = 0.0;
        params.p986 = -0.07;
        params.p987 = 0.0;
        params.p988 = 0.0;
        params.p989 = 0.0;
        params.p990 = -0.11;
        params.p991 = 0.0;
        params.p992 = 0.0;
        params.p993 = 0.0;
        params.p994 = 0.0;
        params.p995 = 0.0;
        params.p996 = 0.0;
        params.p997 = 0.0;
        params.p998 = 0.022;
        params.p999 = 0.0;
        params.p1000 = 0.0;
        params.p1001 = 0.0;
        params.p1002 = 1.0;
        params.p1003 = 0.0;
        params.p1004 = 0.0;
        params.p1005 = 0.0;
        params.p1006 = 0.0;
        params.p1007 = 0.0;
        params.p1008 = 0.0;
        params.p1009 = 0.0;
        params.p1010 = 0.0;
        params.p1011 = 0.0;
        params.p1012 = 0.0;
        params.p1013 = 0.0;
        params.p1014 = 2.2;
        params.p1015 = 0.53;
        params.p1016 = 0.0;
        params.p1017 = 0.0;
        params.p1018 = 0.0;
        params.p1019 = 0.0;
        params.p1020 = 0.0;
        params.p1021 = 0.0;
        params.p1022 = 0.0;
        params.p1023 = 0.0;
        params.p1024 = 0.0;
        params.p1025 = 0.0;
        params.p1026 = 0.0;
        params.p1027 = 0.0;
        params.p1028 = 0.0;
        params.p1029 = 0.0;
        params.p1030 = 0.0;
        params.p1031 = 0.0;
        params.p1032 = 0.0;
        params.p1033 = 0.0;
        params.p1034 = 0.0;
        params.p1035 = 0.0;
        params.p1036 = 0.0;
        params.p1037 = 0.0;
        params.p1038 = 0.0;
        params.p1039 = 0.0;
        params.p1040 = 0.0;
        params.p1041 = 1.0;
        params.p1042 = 1e-5;
        params.p1043 = 0.0;
        params.p1044 = 0.0;
        params.p1045 = 0.1;
        params.p1046 = 0.0;
        params.p1047 = 0.0;
        params.p1048 = 0.0;
        params.p1049 = 0.0;
        params.p1050 = 0.0;
        params.p1051 = 0.0;
        params.p1052 = 0.0;
        params.p1053 = 0.0;
        params.p1054 = 1.0;
        params.p1055 = 0.0;
        params.p1056 = 0.0;
        params.p1057 = 0.0;
        params.p1058 = 0.0;
        params.p1059 = 0.0;
        params.p1060 = 0.0;
        params.p1061 = 0.0;
        params.p1062 = 1.0;
        params.p1063 = 0.0;
        params.p1064 = 1.0;
        params.p1065 = 0.0;
        params.p1066 = 1e-8;
        params.p1067 = params.p785;
        validate_finite_parameter("NOIA2", params.p1067).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1068 = params.p80;
        validate_parameter("HNDEP", params.p1068, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1069 = 0.0;
        params.p1070 = 0.0;
        params.p1071 = 0.0;
        params.p1072 = 0.0;
        params.p1073 = 0.0;
        params.p1074 = 0.0;
        params.p1075 = 0.0;
        params.p1076 = 0.0;
        params.p1077 = 1.0;
        params.p1078 = 0.0;
        params.p1079 = 0.0;
        params.p1080 = 0.0;
        params.p1081 = 0.0;
        params.p1082 = 0.0;
        params.p1083 = 0.0;
        params.p1084 = 0.0;
        params.p1085 = 0.0;
        params.p1086 = 0.0;
        params.p1087 = 0.0;
        params.p1088 = 0.0;
        params.p1089 = 0.0;
        params.p1090 = 0.0;
        params.p1091 = 0.0;
        params.p1092 = 0.0;
        params.p1093 = 0.001;
        validate_parameter("minr", params.p1093, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1094 = 0.0;
        params.p1095 = 0.0;
        params.p1096 = 0.0;
        params.p1097 = 0.0;
        params.p1098 = 0.0;
        params.p1099 = 5e16;
        params.p1100 = 100000.0;
        params.p1101 = 0.0;
        params.p1102 = 0.0;
        params.p1103 = 60.0;
        params.p1104 = params.p1101;
        validate_parameter("PTWGHVII", params.p1104, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1105 = params.p1102;
        validate_finite_parameter("PTWGHV1II", params.p1105).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1106 = params.p1103;
        validate_parameter("PSATXHVII", params.p1106, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1107 = 1.0;
        params.p1108 = 0.0;
        params.p1109 = params.p1099;
        validate_parameter("NDRIFTS", params.p1109, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1110 = 100.0;
        params.p1111 = params.p1110;
        validate_parameter("RDLCWCV", params.p1111, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1112 = 0.0;
        params.p1113 = 0.0;
        params.p1114 = -1.0;
        params.p1115 = 5.000000000000001e-7;
        params.p1116 = params.p1115;
        validate_finite_parameter("LOVERACC", params.p1116).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1117 = params.p80;
        validate_parameter("NDR", params.p1117, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1118 = 0.0;
        params.p1119 = 1.0;
        params.p1120 = 0.0;
        params.p1121 = 0.0;
        params.p1122 = 0.001;
        params.p1123 = 0.6;
        params.p1124 = 0.0;
        params.p1125 = 0.0;
        params.p1126 = 8.0;
        params.p1127 = 0.0;
        params.p1128 = 0.0;
        params.p1129 = 1.0;
        params.p1130 = 0.0;
        params.p1131 = 0.0;
        params.p1132 = 1.0;
        params.p1133 = 0.0;
        params.p1134 = params.p1130;
        validate_finite_parameter("A0CV", params.p1134).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1135 = params.p1131;
        validate_finite_parameter("AGSCV", params.p1135).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1136 = params.p1133;
        validate_parameter("KETACV", params.p1136, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p1137 = 1.0;
        params
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
pub struct Instance {
    pub nodes: [usize; 17],
    pub branches: [usize; 14],
    pub params: Parameters,
    pub(crate) param_given: [bool; 1138],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 16],
    pub(crate) ddt_state_previous: [f64; 16],
    pub(crate) ddt_state_initialized: [bool; 16],
    pub(crate) idt_state_current: [f64; 0],
    pub(crate) idt_state_previous: [f64; 0],
    pub(crate) idt_state_initialized: [bool; 0],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<1631, 17, 14>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<1631, 17, 14>>>,
}

impl Clone for Instance {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes,
            branches: self.branches,
            params: self.params,
            param_given: self.param_given,
            multiplicity: self.multiplicity,
            ddt_state_current: self.ddt_state_current,
            ddt_state_previous: self.ddt_state_previous,
            ddt_state_initialized: self.ddt_state_initialized,
            idt_state_current: self.idt_state_current,
            idt_state_previous: self.idt_state_previous,
            idt_state_initialized: self.idt_state_initialized,
            time: self.time,
            timestep: self.timestep,
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
    pub const MAX_ANALOG_LOOP_ITERATIONS: usize = 1_000_000;
    pub const DDT_EPSILON: f64 = 1.0e-20;

    pub fn new(nodes: &[usize]) -> Self {
        assert_eq!(nodes.len(), Self::NODE_COUNT, "generated Verilog-A node count mismatch");
        let mut mapped = [0usize; Self::NODE_COUNT];
        mapped.copy_from_slice(nodes);
        Self {
            nodes: mapped,
            branches: [0usize; Self::BRANCH_COUNT],
            params: Parameters::default(),
            param_given: [false; Self::PARAMETER_COUNT],
            multiplicity: 1.0,
            ddt_state_current: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_previous: [0.0; Self::DDT_STATE_COUNT],
            ddt_state_initialized: [false; Self::DDT_STATE_COUNT],
            idt_state_current: [0.0; Self::IDT_STATE_COUNT],
            idt_state_previous: [0.0; Self::IDT_STATE_COUNT],
            idt_state_initialized: [false; Self::IDT_STATE_COUNT],
            time: 0.0,
            timestep: 0.0,
            scratch: Some(Box::new(GenericScratch::new())),
            reactive_scratch: Some(Box::new(GenericReactiveScratch::new())),
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
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
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
            ddt_state_initialized,
            idt_state_current,
            idt_state_previous,
            idt_state_initialized,
            time,
            timestep,
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
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "vfbsdoff" => { validate_finite_parameter("VFBSDOFF", value)?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "minz" => { validate_parameter("MINZ", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "rgatemod" => { validate_parameter("RGATEMOD", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "rbodymod" => { validate_parameter("RBODYMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "geomod" => { validate_parameter("GEOMOD", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "rgeomod" => { validate_parameter("RGEOMOD", value, Some((0.0, "0.0")), false, Some((8.0, "8.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "rbpb" => { validate_parameter("RBPB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "rbpd" => { validate_parameter("RBPD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "rbps" => { validate_parameter("RBPS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "rbdb" => { validate_parameter("RBDB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "rbsb" => { validate_parameter("RBSB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "rdb" => { validate_parameter("RDB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "sa" => { validate_finite_parameter("SA", value)?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "sb" => { validate_finite_parameter("SB", value)?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "sd" => { validate_finite_parameter("SD", value)?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "sca" => { validate_finite_parameter("SCA", value)?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "scb" => { validate_finite_parameter("SCB", value)?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "scc" => { validate_finite_parameter("SCC", value)?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "sc" => { validate_finite_parameter("SC", value)?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "mult_fn" => { validate_parameter("MULT_FN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "mulu0" => { validate_finite_parameter("MULU0", value)?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "delvto" => { validate_finite_parameter("DELVTO", value)?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "ids0mult" => { validate_parameter("IDS0MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "edgefet" => { validate_parameter("EDGEFET", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "sslmod" => { validate_parameter("SSLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "cvmod" => { validate_parameter("CVMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "covmod" => { validate_parameter("COVMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "rdsmod" => { validate_parameter("RDSMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "wpemod" => { validate_parameter("WPEMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "asymmod" => { validate_parameter("ASYMMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "gidlmod" => { validate_parameter("GIDLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "igcmod" => { validate_parameter("IGCMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "igbmod" => { validate_parameter("IGBMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "tnoimod" => { validate_parameter("TNOIMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "shmod" => { validate_parameter("SHMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "mobscale" => { validate_parameter("MOBSCALE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "llong" => { validate_parameter("LLONG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "lmlt" => { validate_parameter("LMLT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "wmlt" => { validate_parameter("WMLT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "xl" => { validate_finite_parameter("XL", value)?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "wwide" => { validate_parameter("WWIDE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "xw" => { validate_finite_parameter("XW", value)?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "lint" => { validate_finite_parameter("LINT", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "ll" => { validate_finite_parameter("LL", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "lw" => { validate_finite_parameter("LW", value)?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "lwl" => { validate_finite_parameter("LWL", value)?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "lln" => { validate_finite_parameter("LLN", value)?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "lwn" => { validate_finite_parameter("LWN", value)?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "wint" => { validate_finite_parameter("WINT", value)?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "wl" => { validate_finite_parameter("WL", value)?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "ww" => { validate_finite_parameter("WW", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "wwl" => { validate_finite_parameter("WWL", value)?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "wln" => { validate_finite_parameter("WLN", value)?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "wwn" => { validate_finite_parameter("WWN", value)?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "dlc" => { validate_finite_parameter("DLC", value)?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "llc" => { validate_finite_parameter("LLC", value)?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "lwc" => { validate_finite_parameter("LWC", value)?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "lwlc" => { validate_finite_parameter("LWLC", value)?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "dwc" => { validate_finite_parameter("DWC", value)?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "wlc" => { validate_finite_parameter("WLC", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "wwc" => { validate_finite_parameter("WWC", value)?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "wwlc" => { validate_finite_parameter("WWLC", value)?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "toxe" => { validate_parameter("TOXE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "toxp" => { validate_parameter("TOXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "dtox" => { validate_finite_parameter("DTOX", value)?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "ndep" => { validate_finite_parameter("NDEP", value)?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "ndepl1" => { validate_finite_parameter("NDEPL1", value)?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "ndeplexp1" => { validate_parameter("NDEPLEXP1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "ndepl2" => { validate_finite_parameter("NDEPL2", value)?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "ndeplexp2" => { validate_parameter("NDEPLEXP2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "ndepw" => { validate_finite_parameter("NDEPW", value)?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "ndepwexp" => { validate_parameter("NDEPWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "ndepwl" => { validate_finite_parameter("NDEPWL", value)?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "ndepwlexp" => { validate_parameter("NDEPWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "lndep" => { validate_finite_parameter("LNDEP", value)?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "wndep" => { validate_finite_parameter("WNDEP", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "pndep" => { validate_finite_parameter("PNDEP", value)?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "ndepcv" => { validate_finite_parameter("NDEPCV", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "ndepcvl1" => { validate_finite_parameter("NDEPCVL1", value)?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "ndepcvlexp1" => { validate_parameter("NDEPCVLEXP1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "ndepcvl2" => { validate_finite_parameter("NDEPCVL2", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "ndepcvlexp2" => { validate_parameter("NDEPCVLEXP2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "ndepcvw" => { validate_finite_parameter("NDEPCVW", value)?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "ndepcvwexp" => { validate_parameter("NDEPCVWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "ndepcvwl" => { validate_finite_parameter("NDEPCVWL", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "ndepcvwlexp" => { validate_parameter("NDEPCVWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "lndepcv" => { validate_finite_parameter("LNDEPCV", value)?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "wndepcv" => { validate_finite_parameter("WNDEPCV", value)?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "pndepcv" => { validate_finite_parameter("PNDEPCV", value)?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "ngate" => { validate_finite_parameter("NGATE", value)?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "lngate" => { validate_finite_parameter("LNGATE", value)?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "wngate" => { validate_finite_parameter("WNGATE", value)?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "pngate" => { validate_finite_parameter("PNGATE", value)?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "ni0sub" => { validate_parameter("NI0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "bg0sub" => { validate_parameter("BG0SUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "epsrsub" => { validate_parameter("EPSRSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "epsrox" => { validate_parameter("EPSROX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "xj" => { validate_finite_parameter("XJ", value)?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "lxj" => { validate_finite_parameter("LXJ", value)?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "wxj" => { validate_finite_parameter("WXJ", value)?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "pxj" => { validate_finite_parameter("PXJ", value)?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "vfb" => { validate_finite_parameter("VFB", value)?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "lvfb" => { validate_finite_parameter("LVFB", value)?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "wvfb" => { validate_finite_parameter("WVFB", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "pvfb" => { validate_finite_parameter("PVFB", value)?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "vfbl" => { validate_finite_parameter("VFBL", value)?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "vfblexp" => { validate_parameter("VFBLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "vfbw" => { validate_finite_parameter("VFBW", value)?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "vfbwexp" => { validate_parameter("VFBWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "vfbwl" => { validate_finite_parameter("VFBWL", value)?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "vfbwlexp" => { validate_parameter("VFBWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "vfbcv" => { validate_finite_parameter("VFBCV", value)?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "lvfbcv" => { validate_finite_parameter("LVFBCV", value)?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "wvfbcv" => { validate_finite_parameter("WVFBCV", value)?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "pvfbcv" => { validate_finite_parameter("PVFBCV", value)?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "vfbcvl" => { validate_finite_parameter("VFBCVL", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "vfbcvlexp" => { validate_parameter("VFBCVLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "vfbcvw" => { validate_finite_parameter("VFBCVW", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "vfbcvwexp" => { validate_parameter("VFBCVWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "vfbcvwl" => { validate_finite_parameter("VFBCVWL", value)?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "vfbcvwlexp" => { validate_parameter("VFBCVWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "delvfbacc" => { validate_finite_parameter("DELVFBACC", value)?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "permod" => { validate_parameter("PERMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "dwj" => { validate_finite_parameter("DWJ", value)?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "nsd" => { validate_finite_parameter("NSD", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "lnsd" => { validate_finite_parameter("LNSD", value)?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "wnsd" => { validate_finite_parameter("WNSD", value)?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "pnsd" => { validate_finite_parameter("PNSD", value)?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "dvtp0" => { validate_finite_parameter("DVTP0", value)?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "ldvtp0" => { validate_finite_parameter("LDVTP0", value)?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "wdvtp0" => { validate_finite_parameter("WDVTP0", value)?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "pdvtp0" => { validate_finite_parameter("PDVTP0", value)?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "dvtp1" => { validate_finite_parameter("DVTP1", value)?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "ldvtp1" => { validate_finite_parameter("LDVTP1", value)?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "wdvtp1" => { validate_finite_parameter("WDVTP1", value)?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "pdvtp1" => { validate_finite_parameter("PDVTP1", value)?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "dvtp2" => { validate_finite_parameter("DVTP2", value)?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "ldvtp2" => { validate_finite_parameter("LDVTP2", value)?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "wdvtp2" => { validate_finite_parameter("WDVTP2", value)?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "pdvtp2" => { validate_finite_parameter("PDVTP2", value)?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "dvtp3" => { validate_finite_parameter("DVTP3", value)?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "ldvtp3" => { validate_finite_parameter("LDVTP3", value)?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "wdvtp3" => { validate_finite_parameter("WDVTP3", value)?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "pdvtp3" => { validate_finite_parameter("PDVTP3", value)?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "dvtp4" => { validate_finite_parameter("DVTP4", value)?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "ldvtp4" => { validate_finite_parameter("LDVTP4", value)?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "wdvtp4" => { validate_finite_parameter("WDVTP4", value)?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "pdvtp4" => { validate_finite_parameter("PDVTP4", value)?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "dvtp5" => { validate_finite_parameter("DVTP5", value)?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "ldvtp5" => { validate_finite_parameter("LDVTP5", value)?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "wdvtp5" => { validate_finite_parameter("WDVTP5", value)?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "pdvtp5" => { validate_finite_parameter("PDVTP5", value)?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "phin" => { validate_finite_parameter("PHIN", value)?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "lphin" => { validate_finite_parameter("LPHIN", value)?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "wphin" => { validate_finite_parameter("WPHIN", value)?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "pphin" => { validate_finite_parameter("PPHIN", value)?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "leta0" => { validate_finite_parameter("LETA0", value)?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "weta0" => { validate_finite_parameter("WETA0", value)?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "peta0" => { validate_finite_parameter("PETA0", value)?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "eta0r" => { validate_finite_parameter("ETA0R", value)?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "leta0r" => { validate_finite_parameter("LETA0R", value)?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "weta0r" => { validate_finite_parameter("WETA0R", value)?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "peta0r" => { validate_finite_parameter("PETA0R", value)?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "dsub" => { validate_finite_parameter("DSUB", value)?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "etabexp" => { validate_parameter("ETABEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "letab" => { validate_finite_parameter("LETAB", value)?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "wetab" => { validate_finite_parameter("WETAB", value)?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "petab" => { validate_finite_parameter("PETAB", value)?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "k1" => { validate_finite_parameter("K1", value)?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "k1l" => { validate_finite_parameter("K1L", value)?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "k1lexp" => { validate_parameter("K1LEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "k1w" => { validate_finite_parameter("K1W", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "k1wexp" => { validate_parameter("K1WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "k1wl" => { validate_finite_parameter("K1WL", value)?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "k1wlexp" => { validate_parameter("K1WLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "lk1" => { validate_finite_parameter("LK1", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "wk1" => { validate_finite_parameter("WK1", value)?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "pk1" => { validate_finite_parameter("PK1", value)?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "k2" => { validate_finite_parameter("K2", value)?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "k2l" => { validate_finite_parameter("K2L", value)?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "k2lexp" => { validate_parameter("K2LEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "k2w" => { validate_finite_parameter("K2W", value)?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "k2wexp" => { validate_parameter("K2WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "k2wl" => { validate_finite_parameter("K2WL", value)?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "k2wlexp" => { validate_parameter("K2WLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "lk2" => { validate_finite_parameter("LK2", value)?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "wk2" => { validate_finite_parameter("WK2", value)?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "pk2" => { validate_finite_parameter("PK2", value)?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "ados" => { validate_parameter("ADOS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "bdos" => { validate_parameter("BDOS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "qm0" => { validate_parameter("QM0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "etaqm" => { validate_parameter("ETAQM", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "lcit" => { validate_finite_parameter("LCIT", value)?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "wcit" => { validate_finite_parameter("WCIT", value)?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "pcit" => { validate_finite_parameter("PCIT", value)?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "nfactor" => { validate_finite_parameter("NFACTOR", value)?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "nfactorl" => { validate_finite_parameter("NFACTORL", value)?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "nfactorlexp" => { validate_parameter("NFACTORLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "nfactorw" => { validate_finite_parameter("NFACTORW", value)?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "nfactorwexp" => { validate_parameter("NFACTORWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "nfactorwl" => { validate_finite_parameter("NFACTORWL", value)?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "nfactorwlexp" => { validate_parameter("NFACTORWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "lnfactor" => { validate_finite_parameter("LNFACTOR", value)?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "wnfactor" => { validate_finite_parameter("WNFACTOR", value)?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "pnfactor" => { validate_finite_parameter("PNFACTOR", value)?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "cdscd" => { validate_finite_parameter("CDSCD", value)?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "cdscdl" => { validate_finite_parameter("CDSCDL", value)?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "cdscdlexp" => { validate_parameter("CDSCDLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "lcdscd" => { validate_finite_parameter("LCDSCD", value)?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "wcdscd" => { validate_finite_parameter("WCDSCD", value)?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "pcdscd" => { validate_finite_parameter("PCDSCD", value)?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "cdscdr" => { validate_finite_parameter("CDSCDR", value)?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "lcdscdr" => { validate_finite_parameter("LCDSCDR", value)?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "wcdscdr" => { validate_finite_parameter("WCDSCDR", value)?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "pcdscdr" => { validate_finite_parameter("PCDSCDR", value)?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "cdscb" => { validate_finite_parameter("CDSCB", value)?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "cdscbl" => { validate_finite_parameter("CDSCBL", value)?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "cdscblexp" => { validate_parameter("CDSCBLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "lcdscb" => { validate_finite_parameter("LCDSCB", value)?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "wcdscb" => { validate_finite_parameter("WCDSCB", value)?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "pcdscb" => { validate_finite_parameter("PCDSCB", value)?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "vsat" => { validate_finite_parameter("VSAT", value)?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "lvsat" => { validate_finite_parameter("LVSAT", value)?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "wvsat" => { validate_finite_parameter("WVSAT", value)?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "pvsat" => { validate_finite_parameter("PVSAT", value)?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "vsatl" => { validate_finite_parameter("VSATL", value)?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "vsatlexp" => { validate_parameter("VSATLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "vsatw" => { validate_finite_parameter("VSATW", value)?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "vsatwexp" => { validate_parameter("VSATWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "vsatwl" => { validate_finite_parameter("VSATWL", value)?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "vsatwlexp" => { validate_parameter("VSATWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "vsatr" => { validate_finite_parameter("VSATR", value)?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "lvsatr" => { validate_finite_parameter("LVSATR", value)?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "wvsatr" => { validate_finite_parameter("WVSATR", value)?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "pvsatr" => { validate_finite_parameter("PVSATR", value)?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "delta" => { validate_finite_parameter("DELTA", value)?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "ldelta" => { validate_finite_parameter("LDELTA", value)?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "wdelta" => { validate_finite_parameter("WDELTA", value)?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "pdelta" => { validate_finite_parameter("PDELTA", value)?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "deltal" => { validate_finite_parameter("DELTAL", value)?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "deltalexp" => { validate_parameter("DELTALEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "vsatcv" => { validate_finite_parameter("VSATCV", value)?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "lvsatcv" => { validate_finite_parameter("LVSATCV", value)?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "wvsatcv" => { validate_finite_parameter("WVSATCV", value)?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "pvsatcv" => { validate_finite_parameter("PVSATCV", value)?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "vsatcvl" => { validate_finite_parameter("VSATCVL", value)?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "vsatcvlexp" => { validate_parameter("VSATCVLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "vsatcvw" => { validate_finite_parameter("VSATCVW", value)?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "vsatcvwexp" => { validate_parameter("VSATCVWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "vsatcvwl" => { validate_finite_parameter("VSATCVWL", value)?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "vsatcvwlexp" => { validate_parameter("VSATCVWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "up1" => { validate_finite_parameter("UP1", value)?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "lp1" => { validate_finite_parameter("LP1", value)?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "up2" => { validate_finite_parameter("UP2", value)?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "lp2" => { validate_finite_parameter("LP2", value)?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "u0" => { validate_parameter("U0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "u0l" => { validate_finite_parameter("U0L", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "u0lexp" => { validate_parameter("U0LEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "lu0" => { validate_finite_parameter("LU0", value)?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "wu0" => { validate_finite_parameter("WU0", value)?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "pu0" => { validate_finite_parameter("PU0", value)?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "u0r" => { validate_finite_parameter("U0R", value)?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "lu0r" => { validate_finite_parameter("LU0R", value)?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "wu0r" => { validate_finite_parameter("WU0R", value)?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "pu0r" => { validate_finite_parameter("PU0R", value)?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "etamob" => { validate_finite_parameter("ETAMOB", value)?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "ua" => { validate_finite_parameter("UA", value)?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "ual" => { validate_finite_parameter("UAL", value)?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "ualexp" => { validate_parameter("UALEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
            "uaw" => { validate_finite_parameter("UAW", value)?; self.params.p287 = value; self.mark_param_given(287); Ok(()) }
            "uawexp" => { validate_parameter("UAWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p288 = value; self.mark_param_given(288); Ok(()) }
            "uawl" => { validate_finite_parameter("UAWL", value)?; self.params.p289 = value; self.mark_param_given(289); Ok(()) }
            "uawlexp" => { validate_parameter("UAWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p290 = value; self.mark_param_given(290); Ok(()) }
            "lua" => { validate_finite_parameter("LUA", value)?; self.params.p291 = value; self.mark_param_given(291); Ok(()) }
            "wua" => { validate_finite_parameter("WUA", value)?; self.params.p292 = value; self.mark_param_given(292); Ok(()) }
            "pua" => { validate_finite_parameter("PUA", value)?; self.params.p293 = value; self.mark_param_given(293); Ok(()) }
            "uar" => { validate_finite_parameter("UAR", value)?; self.params.p294 = value; self.mark_param_given(294); Ok(()) }
            "luar" => { validate_finite_parameter("LUAR", value)?; self.params.p295 = value; self.mark_param_given(295); Ok(()) }
            "wuar" => { validate_finite_parameter("WUAR", value)?; self.params.p296 = value; self.mark_param_given(296); Ok(()) }
            "puar" => { validate_finite_parameter("PUAR", value)?; self.params.p297 = value; self.mark_param_given(297); Ok(()) }
            "eu" => { validate_finite_parameter("EU", value)?; self.params.p298 = value; self.mark_param_given(298); Ok(()) }
            "leu" => { validate_finite_parameter("LEU", value)?; self.params.p299 = value; self.mark_param_given(299); Ok(()) }
            "weu" => { validate_finite_parameter("WEU", value)?; self.params.p300 = value; self.mark_param_given(300); Ok(()) }
            "peu" => { validate_finite_parameter("PEU", value)?; self.params.p301 = value; self.mark_param_given(301); Ok(()) }
            "eul" => { validate_finite_parameter("EUL", value)?; self.params.p302 = value; self.mark_param_given(302); Ok(()) }
            "eulexp" => { validate_parameter("EULEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p303 = value; self.mark_param_given(303); Ok(()) }
            "euw" => { validate_finite_parameter("EUW", value)?; self.params.p304 = value; self.mark_param_given(304); Ok(()) }
            "euwexp" => { validate_parameter("EUWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p305 = value; self.mark_param_given(305); Ok(()) }
            "euwl" => { validate_finite_parameter("EUWL", value)?; self.params.p306 = value; self.mark_param_given(306); Ok(()) }
            "euwlexp" => { validate_parameter("EUWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); Ok(()) }
            "ud" => { validate_finite_parameter("UD", value)?; self.params.p308 = value; self.mark_param_given(308); Ok(()) }
            "udl" => { validate_finite_parameter("UDL", value)?; self.params.p309 = value; self.mark_param_given(309); Ok(()) }
            "udlexp" => { validate_parameter("UDLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); Ok(()) }
            "lud" => { validate_finite_parameter("LUD", value)?; self.params.p311 = value; self.mark_param_given(311); Ok(()) }
            "wud" => { validate_finite_parameter("WUD", value)?; self.params.p312 = value; self.mark_param_given(312); Ok(()) }
            "pud" => { validate_finite_parameter("PUD", value)?; self.params.p313 = value; self.mark_param_given(313); Ok(()) }
            "udr" => { validate_finite_parameter("UDR", value)?; self.params.p314 = value; self.mark_param_given(314); Ok(()) }
            "ludr" => { validate_finite_parameter("LUDR", value)?; self.params.p315 = value; self.mark_param_given(315); Ok(()) }
            "wudr" => { validate_finite_parameter("WUDR", value)?; self.params.p316 = value; self.mark_param_given(316); Ok(()) }
            "pudr" => { validate_finite_parameter("PUDR", value)?; self.params.p317 = value; self.mark_param_given(317); Ok(()) }
            "ucs" => { validate_finite_parameter("UCS", value)?; self.params.p318 = value; self.mark_param_given(318); Ok(()) }
            "lucs" => { validate_finite_parameter("LUCS", value)?; self.params.p319 = value; self.mark_param_given(319); Ok(()) }
            "wucs" => { validate_finite_parameter("WUCS", value)?; self.params.p320 = value; self.mark_param_given(320); Ok(()) }
            "pucs" => { validate_finite_parameter("PUCS", value)?; self.params.p321 = value; self.mark_param_given(321); Ok(()) }
            "ucsr" => { validate_finite_parameter("UCSR", value)?; self.params.p322 = value; self.mark_param_given(322); Ok(()) }
            "lucsr" => { validate_finite_parameter("LUCSR", value)?; self.params.p323 = value; self.mark_param_given(323); Ok(()) }
            "wucsr" => { validate_finite_parameter("WUCSR", value)?; self.params.p324 = value; self.mark_param_given(324); Ok(()) }
            "pucsr" => { validate_finite_parameter("PUCSR", value)?; self.params.p325 = value; self.mark_param_given(325); Ok(()) }
            "uc" => { validate_finite_parameter("UC", value)?; self.params.p326 = value; self.mark_param_given(326); Ok(()) }
            "ucl" => { validate_finite_parameter("UCL", value)?; self.params.p327 = value; self.mark_param_given(327); Ok(()) }
            "uclexp" => { validate_parameter("UCLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p328 = value; self.mark_param_given(328); Ok(()) }
            "ucw" => { validate_finite_parameter("UCW", value)?; self.params.p329 = value; self.mark_param_given(329); Ok(()) }
            "ucwexp" => { validate_parameter("UCWEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p330 = value; self.mark_param_given(330); Ok(()) }
            "ucwl" => { validate_finite_parameter("UCWL", value)?; self.params.p331 = value; self.mark_param_given(331); Ok(()) }
            "ucwlexp" => { validate_parameter("UCWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p332 = value; self.mark_param_given(332); Ok(()) }
            "luc" => { validate_finite_parameter("LUC", value)?; self.params.p333 = value; self.mark_param_given(333); Ok(()) }
            "wuc" => { validate_finite_parameter("WUC", value)?; self.params.p334 = value; self.mark_param_given(334); Ok(()) }
            "puc" => { validate_finite_parameter("PUC", value)?; self.params.p335 = value; self.mark_param_given(335); Ok(()) }
            "ucr" => { validate_finite_parameter("UCR", value)?; self.params.p336 = value; self.mark_param_given(336); Ok(()) }
            "lucr" => { validate_finite_parameter("LUCR", value)?; self.params.p337 = value; self.mark_param_given(337); Ok(()) }
            "wucr" => { validate_finite_parameter("WUCR", value)?; self.params.p338 = value; self.mark_param_given(338); Ok(()) }
            "pucr" => { validate_finite_parameter("PUCR", value)?; self.params.p339 = value; self.mark_param_given(339); Ok(()) }
            "pclm" => { validate_finite_parameter("PCLM", value)?; self.params.p340 = value; self.mark_param_given(340); Ok(()) }
            "pclml" => { validate_finite_parameter("PCLML", value)?; self.params.p341 = value; self.mark_param_given(341); Ok(()) }
            "pclmlexp" => { validate_parameter("PCLMLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p342 = value; self.mark_param_given(342); Ok(()) }
            "lpclm" => { validate_finite_parameter("LPCLM", value)?; self.params.p343 = value; self.mark_param_given(343); Ok(()) }
            "wpclm" => { validate_finite_parameter("WPCLM", value)?; self.params.p344 = value; self.mark_param_given(344); Ok(()) }
            "ppclm" => { validate_finite_parameter("PPCLM", value)?; self.params.p345 = value; self.mark_param_given(345); Ok(()) }
            "pclmr" => { validate_finite_parameter("PCLMR", value)?; self.params.p346 = value; self.mark_param_given(346); Ok(()) }
            "lpclmr" => { validate_finite_parameter("LPCLMR", value)?; self.params.p347 = value; self.mark_param_given(347); Ok(()) }
            "wpclmr" => { validate_finite_parameter("WPCLMR", value)?; self.params.p348 = value; self.mark_param_given(348); Ok(()) }
            "ppclmr" => { validate_finite_parameter("PPCLMR", value)?; self.params.p349 = value; self.mark_param_given(349); Ok(()) }
            "pclmg" => { validate_finite_parameter("PCLMG", value)?; self.params.p350 = value; self.mark_param_given(350); Ok(()) }
            "pclmcv" => { validate_finite_parameter("PCLMCV", value)?; self.params.p351 = value; self.mark_param_given(351); Ok(()) }
            "pclmcvl" => { validate_finite_parameter("PCLMCVL", value)?; self.params.p352 = value; self.mark_param_given(352); Ok(()) }
            "pclmcvlexp" => { validate_parameter("PCLMCVLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p353 = value; self.mark_param_given(353); Ok(()) }
            "lpclmcv" => { validate_finite_parameter("LPCLMCV", value)?; self.params.p354 = value; self.mark_param_given(354); Ok(()) }
            "wpclmcv" => { validate_finite_parameter("WPCLMCV", value)?; self.params.p355 = value; self.mark_param_given(355); Ok(()) }
            "ppclmcv" => { validate_finite_parameter("PPCLMCV", value)?; self.params.p356 = value; self.mark_param_given(356); Ok(()) }
            "pscbe1" => { validate_finite_parameter("PSCBE1", value)?; self.params.p357 = value; self.mark_param_given(357); Ok(()) }
            "lpscbe1" => { validate_finite_parameter("LPSCBE1", value)?; self.params.p358 = value; self.mark_param_given(358); Ok(()) }
            "wpscbe1" => { validate_finite_parameter("WPSCBE1", value)?; self.params.p359 = value; self.mark_param_given(359); Ok(()) }
            "ppscbe1" => { validate_finite_parameter("PPSCBE1", value)?; self.params.p360 = value; self.mark_param_given(360); Ok(()) }
            "pscbe2" => { validate_finite_parameter("PSCBE2", value)?; self.params.p361 = value; self.mark_param_given(361); Ok(()) }
            "lpscbe2" => { validate_finite_parameter("LPSCBE2", value)?; self.params.p362 = value; self.mark_param_given(362); Ok(()) }
            "wpscbe2" => { validate_finite_parameter("WPSCBE2", value)?; self.params.p363 = value; self.mark_param_given(363); Ok(()) }
            "ppscbe2" => { validate_finite_parameter("PPSCBE2", value)?; self.params.p364 = value; self.mark_param_given(364); Ok(()) }
            "pdits" => { validate_finite_parameter("PDITS", value)?; self.params.p365 = value; self.mark_param_given(365); Ok(()) }
            "lpdits" => { validate_finite_parameter("LPDITS", value)?; self.params.p366 = value; self.mark_param_given(366); Ok(()) }
            "wpdits" => { validate_finite_parameter("WPDITS", value)?; self.params.p367 = value; self.mark_param_given(367); Ok(()) }
            "ppdits" => { validate_finite_parameter("PPDITS", value)?; self.params.p368 = value; self.mark_param_given(368); Ok(()) }
            "pditsl" => { validate_parameter("PDITSL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p369 = value; self.mark_param_given(369); Ok(()) }
            "pditsd" => { validate_finite_parameter("PDITSD", value)?; self.params.p370 = value; self.mark_param_given(370); Ok(()) }
            "lpditsd" => { validate_finite_parameter("LPDITSD", value)?; self.params.p371 = value; self.mark_param_given(371); Ok(()) }
            "wpditsd" => { validate_finite_parameter("WPDITSD", value)?; self.params.p372 = value; self.mark_param_given(372); Ok(()) }
            "ppditsd" => { validate_finite_parameter("PPDITSD", value)?; self.params.p373 = value; self.mark_param_given(373); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p374 = value; self.mark_param_given(374); Ok(()) }
            "prwg" => { validate_finite_parameter("PRWG", value)?; self.params.p375 = value; self.mark_param_given(375); Ok(()) }
            "lprwg" => { validate_finite_parameter("LPRWG", value)?; self.params.p376 = value; self.mark_param_given(376); Ok(()) }
            "wprwg" => { validate_finite_parameter("WPRWG", value)?; self.params.p377 = value; self.mark_param_given(377); Ok(()) }
            "pprwg" => { validate_finite_parameter("PPRWG", value)?; self.params.p378 = value; self.mark_param_given(378); Ok(()) }
            "prwb" => { validate_finite_parameter("PRWB", value)?; self.params.p379 = value; self.mark_param_given(379); Ok(()) }
            "lprwb" => { validate_finite_parameter("LPRWB", value)?; self.params.p380 = value; self.mark_param_given(380); Ok(()) }
            "wprwb" => { validate_finite_parameter("WPRWB", value)?; self.params.p381 = value; self.mark_param_given(381); Ok(()) }
            "pprwb" => { validate_finite_parameter("PPRWB", value)?; self.params.p382 = value; self.mark_param_given(382); Ok(()) }
            "prwbl" => { validate_finite_parameter("PRWBL", value)?; self.params.p383 = value; self.mark_param_given(383); Ok(()) }
            "prwblexp" => { validate_parameter("PRWBLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p384 = value; self.mark_param_given(384); Ok(()) }
            "wr" => { validate_finite_parameter("WR", value)?; self.params.p385 = value; self.mark_param_given(385); Ok(()) }
            "lwr" => { validate_finite_parameter("LWR", value)?; self.params.p386 = value; self.mark_param_given(386); Ok(()) }
            "wwr" => { validate_finite_parameter("WWR", value)?; self.params.p387 = value; self.mark_param_given(387); Ok(()) }
            "pwr" => { validate_finite_parameter("PWR", value)?; self.params.p388 = value; self.mark_param_given(388); Ok(()) }
            "rswmin" => { validate_finite_parameter("RSWMIN", value)?; self.params.p389 = value; self.mark_param_given(389); Ok(()) }
            "lrswmin" => { validate_finite_parameter("LRSWMIN", value)?; self.params.p390 = value; self.mark_param_given(390); Ok(()) }
            "wrswmin" => { validate_finite_parameter("WRSWMIN", value)?; self.params.p391 = value; self.mark_param_given(391); Ok(()) }
            "prswmin" => { validate_finite_parameter("PRSWMIN", value)?; self.params.p392 = value; self.mark_param_given(392); Ok(()) }
            "rsw" => { validate_finite_parameter("RSW", value)?; self.params.p393 = value; self.mark_param_given(393); Ok(()) }
            "lrsw" => { validate_finite_parameter("LRSW", value)?; self.params.p394 = value; self.mark_param_given(394); Ok(()) }
            "wrsw" => { validate_finite_parameter("WRSW", value)?; self.params.p395 = value; self.mark_param_given(395); Ok(()) }
            "prsw" => { validate_finite_parameter("PRSW", value)?; self.params.p396 = value; self.mark_param_given(396); Ok(()) }
            "rswl" => { validate_finite_parameter("RSWL", value)?; self.params.p397 = value; self.mark_param_given(397); Ok(()) }
            "rswlexp" => { validate_parameter("RSWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p398 = value; self.mark_param_given(398); Ok(()) }
            "rdwmin" => { validate_finite_parameter("RDWMIN", value)?; self.params.p399 = value; self.mark_param_given(399); Ok(()) }
            "lrdwmin" => { validate_finite_parameter("LRDWMIN", value)?; self.params.p400 = value; self.mark_param_given(400); Ok(()) }
            "wrdwmin" => { validate_finite_parameter("WRDWMIN", value)?; self.params.p401 = value; self.mark_param_given(401); Ok(()) }
            "prdwmin" => { validate_finite_parameter("PRDWMIN", value)?; self.params.p402 = value; self.mark_param_given(402); Ok(()) }
            "rdw" => { validate_finite_parameter("RDW", value)?; self.params.p403 = value; self.mark_param_given(403); Ok(()) }
            "lrdw" => { validate_finite_parameter("LRDW", value)?; self.params.p404 = value; self.mark_param_given(404); Ok(()) }
            "wrdw" => { validate_finite_parameter("WRDW", value)?; self.params.p405 = value; self.mark_param_given(405); Ok(()) }
            "prdw" => { validate_finite_parameter("PRDW", value)?; self.params.p406 = value; self.mark_param_given(406); Ok(()) }
            "rdwl" => { validate_finite_parameter("RDWL", value)?; self.params.p407 = value; self.mark_param_given(407); Ok(()) }
            "rdwlexp" => { validate_parameter("RDWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p408 = value; self.mark_param_given(408); Ok(()) }
            "rdswmin" => { validate_finite_parameter("RDSWMIN", value)?; self.params.p409 = value; self.mark_param_given(409); Ok(()) }
            "lrdswmin" => { validate_finite_parameter("LRDSWMIN", value)?; self.params.p410 = value; self.mark_param_given(410); Ok(()) }
            "wrdswmin" => { validate_finite_parameter("WRDSWMIN", value)?; self.params.p411 = value; self.mark_param_given(411); Ok(()) }
            "prdswmin" => { validate_finite_parameter("PRDSWMIN", value)?; self.params.p412 = value; self.mark_param_given(412); Ok(()) }
            "rdsw" => { validate_finite_parameter("RDSW", value)?; self.params.p413 = value; self.mark_param_given(413); Ok(()) }
            "rdswl" => { validate_finite_parameter("RDSWL", value)?; self.params.p414 = value; self.mark_param_given(414); Ok(()) }
            "rdswlexp" => { validate_parameter("RDSWLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p415 = value; self.mark_param_given(415); Ok(()) }
            "lrdsw" => { validate_finite_parameter("LRDSW", value)?; self.params.p416 = value; self.mark_param_given(416); Ok(()) }
            "wrdsw" => { validate_finite_parameter("WRDSW", value)?; self.params.p417 = value; self.mark_param_given(417); Ok(()) }
            "prdsw" => { validate_finite_parameter("PRDSW", value)?; self.params.p418 = value; self.mark_param_given(418); Ok(()) }
            "psat" => { validate_finite_parameter("PSAT", value)?; self.params.p419 = value; self.mark_param_given(419); Ok(()) }
            "lpsat" => { validate_finite_parameter("LPSAT", value)?; self.params.p420 = value; self.mark_param_given(420); Ok(()) }
            "wpsat" => { validate_finite_parameter("WPSAT", value)?; self.params.p421 = value; self.mark_param_given(421); Ok(()) }
            "ppsat" => { validate_finite_parameter("PPSAT", value)?; self.params.p422 = value; self.mark_param_given(422); Ok(()) }
            "psatl" => { validate_finite_parameter("PSATL", value)?; self.params.p423 = value; self.mark_param_given(423); Ok(()) }
            "psatlexp" => { validate_parameter("PSATLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p424 = value; self.mark_param_given(424); Ok(()) }
            "psatb" => { validate_finite_parameter("PSATB", value)?; self.params.p425 = value; self.mark_param_given(425); Ok(()) }
            "psatr" => { validate_finite_parameter("PSATR", value)?; self.params.p426 = value; self.mark_param_given(426); Ok(()) }
            "lpsatr" => { validate_finite_parameter("LPSATR", value)?; self.params.p427 = value; self.mark_param_given(427); Ok(()) }
            "wpsatr" => { validate_finite_parameter("WPSATR", value)?; self.params.p428 = value; self.mark_param_given(428); Ok(()) }
            "ppsatr" => { validate_finite_parameter("PPSATR", value)?; self.params.p429 = value; self.mark_param_given(429); Ok(()) }
            "lpsatb" => { validate_finite_parameter("LPSATB", value)?; self.params.p430 = value; self.mark_param_given(430); Ok(()) }
            "wpsatb" => { validate_finite_parameter("WPSATB", value)?; self.params.p431 = value; self.mark_param_given(431); Ok(()) }
            "ppsatb" => { validate_finite_parameter("PPSATB", value)?; self.params.p432 = value; self.mark_param_given(432); Ok(()) }
            "psatx" => { validate_parameter("PSATX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p433 = value; self.mark_param_given(433); Ok(()) }
            "ptwg" => { validate_finite_parameter("PTWG", value)?; self.params.p434 = value; self.mark_param_given(434); Ok(()) }
            "lptwg" => { validate_finite_parameter("LPTWG", value)?; self.params.p435 = value; self.mark_param_given(435); Ok(()) }
            "wptwg" => { validate_finite_parameter("WPTWG", value)?; self.params.p436 = value; self.mark_param_given(436); Ok(()) }
            "pptwg" => { validate_finite_parameter("PPTWG", value)?; self.params.p437 = value; self.mark_param_given(437); Ok(()) }
            "ptwgl" => { validate_finite_parameter("PTWGL", value)?; self.params.p438 = value; self.mark_param_given(438); Ok(()) }
            "ptwglexp" => { validate_parameter("PTWGLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p439 = value; self.mark_param_given(439); Ok(()) }
            "ptwgr" => { validate_finite_parameter("PTWGR", value)?; self.params.p440 = value; self.mark_param_given(440); Ok(()) }
            "lptwgr" => { validate_finite_parameter("LPTWGR", value)?; self.params.p441 = value; self.mark_param_given(441); Ok(()) }
            "wptwgr" => { validate_finite_parameter("WPTWGR", value)?; self.params.p442 = value; self.mark_param_given(442); Ok(()) }
            "pptwgr" => { validate_finite_parameter("PPTWGR", value)?; self.params.p443 = value; self.mark_param_given(443); Ok(()) }
            "a1" => { validate_finite_parameter("A1", value)?; self.params.p444 = value; self.mark_param_given(444); Ok(()) }
            "la1" => { validate_finite_parameter("LA1", value)?; self.params.p445 = value; self.mark_param_given(445); Ok(()) }
            "wa1" => { validate_finite_parameter("WA1", value)?; self.params.p446 = value; self.mark_param_given(446); Ok(()) }
            "pa1" => { validate_finite_parameter("PA1", value)?; self.params.p447 = value; self.mark_param_given(447); Ok(()) }
            "a11" => { validate_finite_parameter("A11", value)?; self.params.p448 = value; self.mark_param_given(448); Ok(()) }
            "la11" => { validate_finite_parameter("LA11", value)?; self.params.p449 = value; self.mark_param_given(449); Ok(()) }
            "wa11" => { validate_finite_parameter("WA11", value)?; self.params.p450 = value; self.mark_param_given(450); Ok(()) }
            "pa11" => { validate_finite_parameter("PA11", value)?; self.params.p451 = value; self.mark_param_given(451); Ok(()) }
            "a2" => { validate_finite_parameter("A2", value)?; self.params.p452 = value; self.mark_param_given(452); Ok(()) }
            "la2" => { validate_finite_parameter("LA2", value)?; self.params.p453 = value; self.mark_param_given(453); Ok(()) }
            "wa2" => { validate_finite_parameter("WA2", value)?; self.params.p454 = value; self.mark_param_given(454); Ok(()) }
            "pa2" => { validate_finite_parameter("PA2", value)?; self.params.p455 = value; self.mark_param_given(455); Ok(()) }
            "a21" => { validate_finite_parameter("A21", value)?; self.params.p456 = value; self.mark_param_given(456); Ok(()) }
            "la21" => { validate_finite_parameter("LA21", value)?; self.params.p457 = value; self.mark_param_given(457); Ok(()) }
            "wa21" => { validate_finite_parameter("WA21", value)?; self.params.p458 = value; self.mark_param_given(458); Ok(()) }
            "pa21" => { validate_finite_parameter("PA21", value)?; self.params.p459 = value; self.mark_param_given(459); Ok(()) }
            "pdiblc" => { validate_finite_parameter("PDIBLC", value)?; self.params.p460 = value; self.mark_param_given(460); Ok(()) }
            "pdiblcl" => { validate_finite_parameter("PDIBLCL", value)?; self.params.p461 = value; self.mark_param_given(461); Ok(()) }
            "pdiblclexp" => { validate_parameter("PDIBLCLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p462 = value; self.mark_param_given(462); Ok(()) }
            "lpdiblc" => { validate_finite_parameter("LPDIBLC", value)?; self.params.p463 = value; self.mark_param_given(463); Ok(()) }
            "wpdiblc" => { validate_finite_parameter("WPDIBLC", value)?; self.params.p464 = value; self.mark_param_given(464); Ok(()) }
            "ppdiblc" => { validate_finite_parameter("PPDIBLC", value)?; self.params.p465 = value; self.mark_param_given(465); Ok(()) }
            "pdiblcr" => { validate_finite_parameter("PDIBLCR", value)?; self.params.p466 = value; self.mark_param_given(466); Ok(()) }
            "lpdiblcr" => { validate_finite_parameter("LPDIBLCR", value)?; self.params.p467 = value; self.mark_param_given(467); Ok(()) }
            "wpdiblcr" => { validate_finite_parameter("WPDIBLCR", value)?; self.params.p468 = value; self.mark_param_given(468); Ok(()) }
            "ppdiblcr" => { validate_finite_parameter("PPDIBLCR", value)?; self.params.p469 = value; self.mark_param_given(469); Ok(()) }
            "pdiblcb" => { validate_finite_parameter("PDIBLCB", value)?; self.params.p470 = value; self.mark_param_given(470); Ok(()) }
            "lpdiblcb" => { validate_finite_parameter("LPDIBLCB", value)?; self.params.p471 = value; self.mark_param_given(471); Ok(()) }
            "wpdiblcb" => { validate_finite_parameter("WPDIBLCB", value)?; self.params.p472 = value; self.mark_param_given(472); Ok(()) }
            "ppdiblcb" => { validate_finite_parameter("PPDIBLCB", value)?; self.params.p473 = value; self.mark_param_given(473); Ok(()) }
            "pvag" => { validate_finite_parameter("PVAG", value)?; self.params.p474 = value; self.mark_param_given(474); Ok(()) }
            "lpvag" => { validate_finite_parameter("LPVAG", value)?; self.params.p475 = value; self.mark_param_given(475); Ok(()) }
            "wpvag" => { validate_finite_parameter("WPVAG", value)?; self.params.p476 = value; self.mark_param_given(476); Ok(()) }
            "ppvag" => { validate_finite_parameter("PPVAG", value)?; self.params.p477 = value; self.mark_param_given(477); Ok(()) }
            "fprout" => { validate_finite_parameter("FPROUT", value)?; self.params.p478 = value; self.mark_param_given(478); Ok(()) }
            "fproutl" => { validate_finite_parameter("FPROUTL", value)?; self.params.p479 = value; self.mark_param_given(479); Ok(()) }
            "fproutlexp" => { validate_parameter("FPROUTLEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p480 = value; self.mark_param_given(480); Ok(()) }
            "lfprout" => { validate_finite_parameter("LFPROUT", value)?; self.params.p481 = value; self.mark_param_given(481); Ok(()) }
            "wfprout" => { validate_finite_parameter("WFPROUT", value)?; self.params.p482 = value; self.mark_param_given(482); Ok(()) }
            "pfprout" => { validate_finite_parameter("PFPROUT", value)?; self.params.p483 = value; self.mark_param_given(483); Ok(()) }
            "alpha0" => { validate_finite_parameter("ALPHA0", value)?; self.params.p484 = value; self.mark_param_given(484); Ok(()) }
            "alpha0l" => { validate_finite_parameter("ALPHA0L", value)?; self.params.p485 = value; self.mark_param_given(485); Ok(()) }
            "alpha0lexp" => { validate_parameter("ALPHA0LEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p486 = value; self.mark_param_given(486); Ok(()) }
            "alpha0w" => { validate_finite_parameter("ALPHA0W", value)?; self.params.p487 = value; self.mark_param_given(487); Ok(()) }
            "alpha0wexp" => { validate_parameter("ALPHA0WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p488 = value; self.mark_param_given(488); Ok(()) }
            "lalpha0" => { validate_finite_parameter("LALPHA0", value)?; self.params.p489 = value; self.mark_param_given(489); Ok(()) }
            "walpha0" => { validate_finite_parameter("WALPHA0", value)?; self.params.p490 = value; self.mark_param_given(490); Ok(()) }
            "palpha0" => { validate_finite_parameter("PALPHA0", value)?; self.params.p491 = value; self.mark_param_given(491); Ok(()) }
            "alpha3" => { validate_finite_parameter("ALPHA3", value)?; self.params.p492 = value; self.mark_param_given(492); Ok(()) }
            "alpha4" => { validate_parameter("ALPHA4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p493 = value; self.mark_param_given(493); Ok(()) }
            "beta0" => { validate_finite_parameter("BETA0", value)?; self.params.p494 = value; self.mark_param_given(494); Ok(()) }
            "beta0w" => { validate_finite_parameter("BETA0W", value)?; self.params.p495 = value; self.mark_param_given(495); Ok(()) }
            "beta0wexp" => { validate_parameter("BETA0WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p496 = value; self.mark_param_given(496); Ok(()) }
            "lbeta0" => { validate_finite_parameter("LBETA0", value)?; self.params.p497 = value; self.mark_param_given(497); Ok(()) }
            "wbeta0" => { validate_finite_parameter("WBETA0", value)?; self.params.p498 = value; self.mark_param_given(498); Ok(()) }
            "pbeta0" => { validate_finite_parameter("PBETA0", value)?; self.params.p499 = value; self.mark_param_given(499); Ok(()) }
            "alphadr" => { validate_finite_parameter("ALPHADR", value)?; self.params.p500 = value; self.mark_param_given(500); Ok(()) }
            "betadr" => { validate_finite_parameter("BETADR", value)?; self.params.p501 = value; self.mark_param_given(501); Ok(()) }
            "drii1" => { validate_parameter("DRII1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p502 = value; self.mark_param_given(502); Ok(()) }
            "drii2" => { validate_parameter("DRII2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p503 = value; self.mark_param_given(503); Ok(()) }
            "deltaii" => { validate_parameter("DELTAII", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p504 = value; self.mark_param_given(504); Ok(()) }
            "alpha1" => { validate_finite_parameter("ALPHA1", value)?; self.params.p505 = value; self.mark_param_given(505); Ok(()) }
            "alpha2" => { validate_finite_parameter("ALPHA2", value)?; self.params.p506 = value; self.mark_param_given(506); Ok(()) }
            "alphadr1" => { validate_finite_parameter("ALPHADR1", value)?; self.params.p507 = value; self.mark_param_given(507); Ok(()) }
            "alphadr2" => { validate_finite_parameter("ALPHADR2", value)?; self.params.p508 = value; self.mark_param_given(508); Ok(()) }
            "alphadr3" => { validate_finite_parameter("ALPHADR3", value)?; self.params.p509 = value; self.mark_param_given(509); Ok(()) }
            "alphadr4" => { validate_finite_parameter("ALPHADR4", value)?; self.params.p510 = value; self.mark_param_given(510); Ok(()) }
            "drexp" => { validate_parameter("DREXP", value, Some((0.0, "0.0")), true, Some((5.0, "5.0")), false, &[])?; self.params.p511 = value; self.mark_param_given(511); Ok(()) }
            "drii3" => { validate_parameter("DRII3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p512 = value; self.mark_param_given(512); Ok(()) }
            "drii4" => { validate_parameter("DRII4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p513 = value; self.mark_param_given(513); Ok(()) }
            "cmd1" => { validate_parameter("CMD1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p514 = value; self.mark_param_given(514); Ok(()) }
            "cmd2" => { validate_parameter("CMD2", value, Some((0.5, "0.5")), false, Some((5.0, "5.0")), false, &[])?; self.params.p515 = value; self.mark_param_given(515); Ok(()) }
            "cms1" => { validate_parameter("CMS1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p516 = value; self.mark_param_given(516); Ok(()) }
            "cms2" => { validate_parameter("CMS2", value, Some((0.5, "0.5")), false, Some((5.0, "5.0")), false, &[])?; self.params.p517 = value; self.mark_param_given(517); Ok(()) }
            "beta1" => { validate_parameter("BETA1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p518 = value; self.mark_param_given(518); Ok(()) }
            "beta1w" => { validate_finite_parameter("BETA1W", value)?; self.params.p519 = value; self.mark_param_given(519); Ok(()) }
            "beta1wexp" => { validate_parameter("BETA1WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p520 = value; self.mark_param_given(520); Ok(()) }
            "beta2" => { validate_finite_parameter("BETA2", value)?; self.params.p521 = value; self.mark_param_given(521); Ok(()) }
            "beta2w" => { validate_finite_parameter("BETA2W", value)?; self.params.p522 = value; self.mark_param_given(522); Ok(()) }
            "beta2wexp" => { validate_parameter("BETA2WEXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p523 = value; self.mark_param_given(523); Ok(()) }
            "beta3" => { validate_parameter("BETA3", value, Some((0.1, "0.1")), false, Some((10.0, "10.0")), false, &[])?; self.params.p524 = value; self.mark_param_given(524); Ok(()) }
            "alpha0r" => { validate_finite_parameter("ALPHA0R", value)?; self.params.p525 = value; self.mark_param_given(525); Ok(()) }
            "lalpha0r" => { validate_finite_parameter("LALPHA0R", value)?; self.params.p526 = value; self.mark_param_given(526); Ok(()) }
            "walpha0r" => { validate_finite_parameter("WALPHA0R", value)?; self.params.p527 = value; self.mark_param_given(527); Ok(()) }
            "palpha0r" => { validate_finite_parameter("PALPHA0R", value)?; self.params.p528 = value; self.mark_param_given(528); Ok(()) }
            "beta0r" => { validate_finite_parameter("BETA0R", value)?; self.params.p529 = value; self.mark_param_given(529); Ok(()) }
            "lbeta0r" => { validate_finite_parameter("LBETA0R", value)?; self.params.p530 = value; self.mark_param_given(530); Ok(()) }
            "wbeta0r" => { validate_finite_parameter("WBETA0R", value)?; self.params.p531 = value; self.mark_param_given(531); Ok(()) }
            "pbeta0r" => { validate_finite_parameter("PBETA0R", value)?; self.params.p532 = value; self.mark_param_given(532); Ok(()) }
            "aigbacc" => { validate_finite_parameter("AIGBACC", value)?; self.params.p533 = value; self.mark_param_given(533); Ok(()) }
            "bigbacc" => { validate_finite_parameter("BIGBACC", value)?; self.params.p534 = value; self.mark_param_given(534); Ok(()) }
            "cigbacc" => { validate_finite_parameter("CIGBACC", value)?; self.params.p535 = value; self.mark_param_given(535); Ok(()) }
            "nigbacc" => { validate_finite_parameter("NIGBACC", value)?; self.params.p536 = value; self.mark_param_given(536); Ok(()) }
            "aigbinv" => { validate_finite_parameter("AIGBINV", value)?; self.params.p537 = value; self.mark_param_given(537); Ok(()) }
            "bigbinv" => { validate_finite_parameter("BIGBINV", value)?; self.params.p538 = value; self.mark_param_given(538); Ok(()) }
            "cigbinv" => { validate_finite_parameter("CIGBINV", value)?; self.params.p539 = value; self.mark_param_given(539); Ok(()) }
            "eigbinv" => { validate_finite_parameter("EIGBINV", value)?; self.params.p540 = value; self.mark_param_given(540); Ok(()) }
            "nigbinv" => { validate_finite_parameter("NIGBINV", value)?; self.params.p541 = value; self.mark_param_given(541); Ok(()) }
            "aigc" => { validate_finite_parameter("AIGC", value)?; self.params.p542 = value; self.mark_param_given(542); Ok(()) }
            "bigc" => { validate_finite_parameter("BIGC", value)?; self.params.p543 = value; self.mark_param_given(543); Ok(()) }
            "cigc" => { validate_finite_parameter("CIGC", value)?; self.params.p544 = value; self.mark_param_given(544); Ok(()) }
            "aigs" => { validate_finite_parameter("AIGS", value)?; self.params.p545 = value; self.mark_param_given(545); Ok(()) }
            "bigs" => { validate_finite_parameter("BIGS", value)?; self.params.p546 = value; self.mark_param_given(546); Ok(()) }
            "cigs" => { validate_finite_parameter("CIGS", value)?; self.params.p547 = value; self.mark_param_given(547); Ok(()) }
            "aigd" => { validate_finite_parameter("AIGD", value)?; self.params.p548 = value; self.mark_param_given(548); Ok(()) }
            "bigd" => { validate_finite_parameter("BIGD", value)?; self.params.p549 = value; self.mark_param_given(549); Ok(()) }
            "cigd" => { validate_finite_parameter("CIGD", value)?; self.params.p550 = value; self.mark_param_given(550); Ok(()) }
            "dlcig" => { validate_finite_parameter("DLCIG", value)?; self.params.p551 = value; self.mark_param_given(551); Ok(()) }
            "dlcigd" => { validate_finite_parameter("DLCIGD", value)?; self.params.p552 = value; self.mark_param_given(552); Ok(()) }
            "poxedge" => { validate_finite_parameter("POXEDGE", value)?; self.params.p553 = value; self.mark_param_given(553); Ok(()) }
            "ntox" => { validate_finite_parameter("NTOX", value)?; self.params.p554 = value; self.mark_param_given(554); Ok(()) }
            "toxref" => { validate_parameter("TOXREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p555 = value; self.mark_param_given(555); Ok(()) }
            "pigcd" => { validate_parameter("PIGCD", value, Some((-50.0, "-50.0")), false, Some((50.0, "50.0")), false, &[])?; self.params.p556 = value; self.mark_param_given(556); Ok(()) }
            "aigcl" => { validate_finite_parameter("AIGCL", value)?; self.params.p557 = value; self.mark_param_given(557); Ok(()) }
            "aigcw" => { validate_finite_parameter("AIGCW", value)?; self.params.p558 = value; self.mark_param_given(558); Ok(()) }
            "aigsl" => { validate_finite_parameter("AIGSL", value)?; self.params.p559 = value; self.mark_param_given(559); Ok(()) }
            "aigsw" => { validate_finite_parameter("AIGSW", value)?; self.params.p560 = value; self.mark_param_given(560); Ok(()) }
            "aigdl" => { validate_finite_parameter("AIGDL", value)?; self.params.p561 = value; self.mark_param_given(561); Ok(()) }
            "aigdw" => { validate_finite_parameter("AIGDW", value)?; self.params.p562 = value; self.mark_param_given(562); Ok(()) }
            "pigcdl" => { validate_finite_parameter("PIGCDL", value)?; self.params.p563 = value; self.mark_param_given(563); Ok(()) }
            "laigbinv" => { validate_finite_parameter("LAIGBINV", value)?; self.params.p564 = value; self.mark_param_given(564); Ok(()) }
            "waigbinv" => { validate_finite_parameter("WAIGBINV", value)?; self.params.p565 = value; self.mark_param_given(565); Ok(()) }
            "paigbinv" => { validate_finite_parameter("PAIGBINV", value)?; self.params.p566 = value; self.mark_param_given(566); Ok(()) }
            "lbigbinv" => { validate_finite_parameter("LBIGBINV", value)?; self.params.p567 = value; self.mark_param_given(567); Ok(()) }
            "wbigbinv" => { validate_finite_parameter("WBIGBINV", value)?; self.params.p568 = value; self.mark_param_given(568); Ok(()) }
            "pbigbinv" => { validate_finite_parameter("PBIGBINV", value)?; self.params.p569 = value; self.mark_param_given(569); Ok(()) }
            "lcigbinv" => { validate_finite_parameter("LCIGBINV", value)?; self.params.p570 = value; self.mark_param_given(570); Ok(()) }
            "wcigbinv" => { validate_finite_parameter("WCIGBINV", value)?; self.params.p571 = value; self.mark_param_given(571); Ok(()) }
            "pcigbinv" => { validate_finite_parameter("PCIGBINV", value)?; self.params.p572 = value; self.mark_param_given(572); Ok(()) }
            "leigbinv" => { validate_finite_parameter("LEIGBINV", value)?; self.params.p573 = value; self.mark_param_given(573); Ok(()) }
            "weigbinv" => { validate_finite_parameter("WEIGBINV", value)?; self.params.p574 = value; self.mark_param_given(574); Ok(()) }
            "peigbinv" => { validate_finite_parameter("PEIGBINV", value)?; self.params.p575 = value; self.mark_param_given(575); Ok(()) }
            "lnigbinv" => { validate_finite_parameter("LNIGBINV", value)?; self.params.p576 = value; self.mark_param_given(576); Ok(()) }
            "wnigbinv" => { validate_finite_parameter("WNIGBINV", value)?; self.params.p577 = value; self.mark_param_given(577); Ok(()) }
            "pnigbinv" => { validate_finite_parameter("PNIGBINV", value)?; self.params.p578 = value; self.mark_param_given(578); Ok(()) }
            "laigbacc" => { validate_finite_parameter("LAIGBACC", value)?; self.params.p579 = value; self.mark_param_given(579); Ok(()) }
            "waigbacc" => { validate_finite_parameter("WAIGBACC", value)?; self.params.p580 = value; self.mark_param_given(580); Ok(()) }
            "paigbacc" => { validate_finite_parameter("PAIGBACC", value)?; self.params.p581 = value; self.mark_param_given(581); Ok(()) }
            "lbigbacc" => { validate_finite_parameter("LBIGBACC", value)?; self.params.p582 = value; self.mark_param_given(582); Ok(()) }
            "wbigbacc" => { validate_finite_parameter("WBIGBACC", value)?; self.params.p583 = value; self.mark_param_given(583); Ok(()) }
            "pbigbacc" => { validate_finite_parameter("PBIGBACC", value)?; self.params.p584 = value; self.mark_param_given(584); Ok(()) }
            "lcigbacc" => { validate_finite_parameter("LCIGBACC", value)?; self.params.p585 = value; self.mark_param_given(585); Ok(()) }
            "wcigbacc" => { validate_finite_parameter("WCIGBACC", value)?; self.params.p586 = value; self.mark_param_given(586); Ok(()) }
            "pcigbacc" => { validate_finite_parameter("PCIGBACC", value)?; self.params.p587 = value; self.mark_param_given(587); Ok(()) }
            "lnigbacc" => { validate_finite_parameter("LNIGBACC", value)?; self.params.p588 = value; self.mark_param_given(588); Ok(()) }
            "wnigbacc" => { validate_finite_parameter("WNIGBACC", value)?; self.params.p589 = value; self.mark_param_given(589); Ok(()) }
            "pnigbacc" => { validate_finite_parameter("PNIGBACC", value)?; self.params.p590 = value; self.mark_param_given(590); Ok(()) }
            "laigc" => { validate_finite_parameter("LAIGC", value)?; self.params.p591 = value; self.mark_param_given(591); Ok(()) }
            "waigc" => { validate_finite_parameter("WAIGC", value)?; self.params.p592 = value; self.mark_param_given(592); Ok(()) }
            "paigc" => { validate_finite_parameter("PAIGC", value)?; self.params.p593 = value; self.mark_param_given(593); Ok(()) }
            "lbigc" => { validate_finite_parameter("LBIGC", value)?; self.params.p594 = value; self.mark_param_given(594); Ok(()) }
            "wbigc" => { validate_finite_parameter("WBIGC", value)?; self.params.p595 = value; self.mark_param_given(595); Ok(()) }
            "pbigc" => { validate_finite_parameter("PBIGC", value)?; self.params.p596 = value; self.mark_param_given(596); Ok(()) }
            "lcigc" => { validate_finite_parameter("LCIGC", value)?; self.params.p597 = value; self.mark_param_given(597); Ok(()) }
            "wcigc" => { validate_finite_parameter("WCIGC", value)?; self.params.p598 = value; self.mark_param_given(598); Ok(()) }
            "pcigc" => { validate_finite_parameter("PCIGC", value)?; self.params.p599 = value; self.mark_param_given(599); Ok(()) }
            "laigs" => { validate_finite_parameter("LAIGS", value)?; self.params.p600 = value; self.mark_param_given(600); Ok(()) }
            "waigs" => { validate_finite_parameter("WAIGS", value)?; self.params.p601 = value; self.mark_param_given(601); Ok(()) }
            "paigs" => { validate_finite_parameter("PAIGS", value)?; self.params.p602 = value; self.mark_param_given(602); Ok(()) }
            "lbigs" => { validate_finite_parameter("LBIGS", value)?; self.params.p603 = value; self.mark_param_given(603); Ok(()) }
            "wbigs" => { validate_finite_parameter("WBIGS", value)?; self.params.p604 = value; self.mark_param_given(604); Ok(()) }
            "pbigs" => { validate_finite_parameter("PBIGS", value)?; self.params.p605 = value; self.mark_param_given(605); Ok(()) }
            "lcigs" => { validate_finite_parameter("LCIGS", value)?; self.params.p606 = value; self.mark_param_given(606); Ok(()) }
            "wcigs" => { validate_finite_parameter("WCIGS", value)?; self.params.p607 = value; self.mark_param_given(607); Ok(()) }
            "pcigs" => { validate_finite_parameter("PCIGS", value)?; self.params.p608 = value; self.mark_param_given(608); Ok(()) }
            "laigd" => { validate_finite_parameter("LAIGD", value)?; self.params.p609 = value; self.mark_param_given(609); Ok(()) }
            "waigd" => { validate_finite_parameter("WAIGD", value)?; self.params.p610 = value; self.mark_param_given(610); Ok(()) }
            "paigd" => { validate_finite_parameter("PAIGD", value)?; self.params.p611 = value; self.mark_param_given(611); Ok(()) }
            "lbigd" => { validate_finite_parameter("LBIGD", value)?; self.params.p612 = value; self.mark_param_given(612); Ok(()) }
            "wbigd" => { validate_finite_parameter("WBIGD", value)?; self.params.p613 = value; self.mark_param_given(613); Ok(()) }
            "pbigd" => { validate_finite_parameter("PBIGD", value)?; self.params.p614 = value; self.mark_param_given(614); Ok(()) }
            "lcigd" => { validate_finite_parameter("LCIGD", value)?; self.params.p615 = value; self.mark_param_given(615); Ok(()) }
            "wcigd" => { validate_finite_parameter("WCIGD", value)?; self.params.p616 = value; self.mark_param_given(616); Ok(()) }
            "pcigd" => { validate_finite_parameter("PCIGD", value)?; self.params.p617 = value; self.mark_param_given(617); Ok(()) }
            "lpoxedge" => { validate_finite_parameter("LPOXEDGE", value)?; self.params.p618 = value; self.mark_param_given(618); Ok(()) }
            "wpoxedge" => { validate_finite_parameter("WPOXEDGE", value)?; self.params.p619 = value; self.mark_param_given(619); Ok(()) }
            "ppoxedge" => { validate_finite_parameter("PPOXEDGE", value)?; self.params.p620 = value; self.mark_param_given(620); Ok(()) }
            "ldlcig" => { validate_finite_parameter("LDLCIG", value)?; self.params.p621 = value; self.mark_param_given(621); Ok(()) }
            "wdlcig" => { validate_finite_parameter("WDLCIG", value)?; self.params.p622 = value; self.mark_param_given(622); Ok(()) }
            "pdlcig" => { validate_finite_parameter("PDLCIG", value)?; self.params.p623 = value; self.mark_param_given(623); Ok(()) }
            "ldlcigd" => { validate_finite_parameter("LDLCIGD", value)?; self.params.p624 = value; self.mark_param_given(624); Ok(()) }
            "wdlcigd" => { validate_finite_parameter("WDLCIGD", value)?; self.params.p625 = value; self.mark_param_given(625); Ok(()) }
            "pdlcigd" => { validate_finite_parameter("PDLCIGD", value)?; self.params.p626 = value; self.mark_param_given(626); Ok(()) }
            "lntox" => { validate_finite_parameter("LNTOX", value)?; self.params.p627 = value; self.mark_param_given(627); Ok(()) }
            "wntox" => { validate_finite_parameter("WNTOX", value)?; self.params.p628 = value; self.mark_param_given(628); Ok(()) }
            "pntox" => { validate_finite_parameter("PNTOX", value)?; self.params.p629 = value; self.mark_param_given(629); Ok(()) }
            "agidl" => { validate_finite_parameter("AGIDL", value)?; self.params.p630 = value; self.mark_param_given(630); Ok(()) }
            "agidll" => { validate_finite_parameter("AGIDLL", value)?; self.params.p631 = value; self.mark_param_given(631); Ok(()) }
            "agidlw" => { validate_finite_parameter("AGIDLW", value)?; self.params.p632 = value; self.mark_param_given(632); Ok(()) }
            "lagidl" => { validate_finite_parameter("LAGIDL", value)?; self.params.p633 = value; self.mark_param_given(633); Ok(()) }
            "wagidl" => { validate_finite_parameter("WAGIDL", value)?; self.params.p634 = value; self.mark_param_given(634); Ok(()) }
            "pagidl" => { validate_finite_parameter("PAGIDL", value)?; self.params.p635 = value; self.mark_param_given(635); Ok(()) }
            "bgidl" => { validate_finite_parameter("BGIDL", value)?; self.params.p636 = value; self.mark_param_given(636); Ok(()) }
            "lbgidl" => { validate_finite_parameter("LBGIDL", value)?; self.params.p637 = value; self.mark_param_given(637); Ok(()) }
            "wbgidl" => { validate_finite_parameter("WBGIDL", value)?; self.params.p638 = value; self.mark_param_given(638); Ok(()) }
            "pbgidl" => { validate_finite_parameter("PBGIDL", value)?; self.params.p639 = value; self.mark_param_given(639); Ok(()) }
            "cgidl" => { validate_finite_parameter("CGIDL", value)?; self.params.p640 = value; self.mark_param_given(640); Ok(()) }
            "lcgidl" => { validate_finite_parameter("LCGIDL", value)?; self.params.p641 = value; self.mark_param_given(641); Ok(()) }
            "wcgidl" => { validate_finite_parameter("WCGIDL", value)?; self.params.p642 = value; self.mark_param_given(642); Ok(()) }
            "pcgidl" => { validate_finite_parameter("PCGIDL", value)?; self.params.p643 = value; self.mark_param_given(643); Ok(()) }
            "egidl" => { validate_finite_parameter("EGIDL", value)?; self.params.p644 = value; self.mark_param_given(644); Ok(()) }
            "legidl" => { validate_finite_parameter("LEGIDL", value)?; self.params.p645 = value; self.mark_param_given(645); Ok(()) }
            "wegidl" => { validate_finite_parameter("WEGIDL", value)?; self.params.p646 = value; self.mark_param_given(646); Ok(()) }
            "pegidl" => { validate_finite_parameter("PEGIDL", value)?; self.params.p647 = value; self.mark_param_given(647); Ok(()) }
            "agisl" => { validate_finite_parameter("AGISL", value)?; self.params.p648 = value; self.mark_param_given(648); Ok(()) }
            "agisll" => { validate_finite_parameter("AGISLL", value)?; self.params.p649 = value; self.mark_param_given(649); Ok(()) }
            "agislw" => { validate_finite_parameter("AGISLW", value)?; self.params.p650 = value; self.mark_param_given(650); Ok(()) }
            "lagisl" => { validate_finite_parameter("LAGISL", value)?; self.params.p651 = value; self.mark_param_given(651); Ok(()) }
            "wagisl" => { validate_finite_parameter("WAGISL", value)?; self.params.p652 = value; self.mark_param_given(652); Ok(()) }
            "pagisl" => { validate_finite_parameter("PAGISL", value)?; self.params.p653 = value; self.mark_param_given(653); Ok(()) }
            "bgisl" => { validate_finite_parameter("BGISL", value)?; self.params.p654 = value; self.mark_param_given(654); Ok(()) }
            "lbgisl" => { validate_finite_parameter("LBGISL", value)?; self.params.p655 = value; self.mark_param_given(655); Ok(()) }
            "wbgisl" => { validate_finite_parameter("WBGISL", value)?; self.params.p656 = value; self.mark_param_given(656); Ok(()) }
            "pbgisl" => { validate_finite_parameter("PBGISL", value)?; self.params.p657 = value; self.mark_param_given(657); Ok(()) }
            "cgisl" => { validate_finite_parameter("CGISL", value)?; self.params.p658 = value; self.mark_param_given(658); Ok(()) }
            "lcgisl" => { validate_finite_parameter("LCGISL", value)?; self.params.p659 = value; self.mark_param_given(659); Ok(()) }
            "wcgisl" => { validate_finite_parameter("WCGISL", value)?; self.params.p660 = value; self.mark_param_given(660); Ok(()) }
            "pcgisl" => { validate_finite_parameter("PCGISL", value)?; self.params.p661 = value; self.mark_param_given(661); Ok(()) }
            "egisl" => { validate_finite_parameter("EGISL", value)?; self.params.p662 = value; self.mark_param_given(662); Ok(()) }
            "legisl" => { validate_finite_parameter("LEGISL", value)?; self.params.p663 = value; self.mark_param_given(663); Ok(()) }
            "wegisl" => { validate_finite_parameter("WEGISL", value)?; self.params.p664 = value; self.mark_param_given(664); Ok(()) }
            "pegisl" => { validate_finite_parameter("PEGISL", value)?; self.params.p665 = value; self.mark_param_given(665); Ok(()) }
            "cf" => { validate_finite_parameter("CF", value)?; self.params.p666 = value; self.mark_param_given(666); Ok(()) }
            "lcf" => { validate_finite_parameter("LCF", value)?; self.params.p667 = value; self.mark_param_given(667); Ok(()) }
            "wcf" => { validate_finite_parameter("WCF", value)?; self.params.p668 = value; self.mark_param_given(668); Ok(()) }
            "pcf" => { validate_finite_parameter("PCF", value)?; self.params.p669 = value; self.mark_param_given(669); Ok(()) }
            "cfrcoeff" => { validate_parameter("CFRCOEFF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p670 = value; self.mark_param_given(670); Ok(()) }
            "cgso" => { validate_finite_parameter("CGSO", value)?; self.params.p671 = value; self.mark_param_given(671); Ok(()) }
            "cgdo" => { validate_finite_parameter("CGDO", value)?; self.params.p672 = value; self.mark_param_given(672); Ok(()) }
            "cgbo" => { validate_finite_parameter("CGBO", value)?; self.params.p673 = value; self.mark_param_given(673); Ok(()) }
            "cgsl" => { validate_finite_parameter("CGSL", value)?; self.params.p674 = value; self.mark_param_given(674); Ok(()) }
            "lcgsl" => { validate_finite_parameter("LCGSL", value)?; self.params.p675 = value; self.mark_param_given(675); Ok(()) }
            "wcgsl" => { validate_finite_parameter("WCGSL", value)?; self.params.p676 = value; self.mark_param_given(676); Ok(()) }
            "pcgsl" => { validate_finite_parameter("PCGSL", value)?; self.params.p677 = value; self.mark_param_given(677); Ok(()) }
            "cgdl" => { validate_finite_parameter("CGDL", value)?; self.params.p678 = value; self.mark_param_given(678); Ok(()) }
            "lcgdl" => { validate_finite_parameter("LCGDL", value)?; self.params.p679 = value; self.mark_param_given(679); Ok(()) }
            "wcgdl" => { validate_finite_parameter("WCGDL", value)?; self.params.p680 = value; self.mark_param_given(680); Ok(()) }
            "pcgdl" => { validate_finite_parameter("PCGDL", value)?; self.params.p681 = value; self.mark_param_given(681); Ok(()) }
            "ckappas" => { validate_finite_parameter("CKAPPAS", value)?; self.params.p682 = value; self.mark_param_given(682); Ok(()) }
            "lckappas" => { validate_finite_parameter("LCKAPPAS", value)?; self.params.p683 = value; self.mark_param_given(683); Ok(()) }
            "wckappas" => { validate_finite_parameter("WCKAPPAS", value)?; self.params.p684 = value; self.mark_param_given(684); Ok(()) }
            "pckappas" => { validate_finite_parameter("PCKAPPAS", value)?; self.params.p685 = value; self.mark_param_given(685); Ok(()) }
            "ckappad" => { validate_finite_parameter("CKAPPAD", value)?; self.params.p686 = value; self.mark_param_given(686); Ok(()) }
            "lckappad" => { validate_finite_parameter("LCKAPPAD", value)?; self.params.p687 = value; self.mark_param_given(687); Ok(()) }
            "wckappad" => { validate_finite_parameter("WCKAPPAD", value)?; self.params.p688 = value; self.mark_param_given(688); Ok(()) }
            "pckappad" => { validate_finite_parameter("PCKAPPAD", value)?; self.params.p689 = value; self.mark_param_given(689); Ok(()) }
            "ckappad1" => { validate_parameter("CKAPPAD1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p690 = value; self.mark_param_given(690); Ok(()) }
            "ckappad2" => { validate_parameter("CKAPPAD2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p691 = value; self.mark_param_given(691); Ok(()) }
            "ckappas1" => { validate_parameter("CKAPPAS1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p692 = value; self.mark_param_given(692); Ok(()) }
            "ckappas2" => { validate_parameter("CKAPPAS2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p693 = value; self.mark_param_given(693); Ok(()) }
            "spqbacv" => { validate_parameter("SPQBACV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p694 = value; self.mark_param_given(694); Ok(()) }
            "dmcg" => { validate_parameter("DMCG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p695 = value; self.mark_param_given(695); Ok(()) }
            "dmci" => { validate_parameter("DMCI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p696 = value; self.mark_param_given(696); Ok(()) }
            "dmdg" => { validate_parameter("DMDG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p697 = value; self.mark_param_given(697); Ok(()) }
            "dmcgt" => { validate_parameter("DMCGT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p698 = value; self.mark_param_given(698); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p699 = value; self.mark_param_given(699); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p700 = value; self.mark_param_given(700); Ok(()) }
            "cjs" => { validate_finite_parameter("CJS", value)?; self.params.p701 = value; self.mark_param_given(701); Ok(()) }
            "cjd" => { validate_finite_parameter("CJD", value)?; self.params.p702 = value; self.mark_param_given(702); Ok(()) }
            "cjsws" => { validate_finite_parameter("CJSWS", value)?; self.params.p703 = value; self.mark_param_given(703); Ok(()) }
            "cjswd" => { validate_finite_parameter("CJSWD", value)?; self.params.p704 = value; self.mark_param_given(704); Ok(()) }
            "cjswgs" => { validate_finite_parameter("CJSWGS", value)?; self.params.p705 = value; self.mark_param_given(705); Ok(()) }
            "cjswgd" => { validate_finite_parameter("CJSWGD", value)?; self.params.p706 = value; self.mark_param_given(706); Ok(()) }
            "pbs" => { validate_finite_parameter("PBS", value)?; self.params.p707 = value; self.mark_param_given(707); Ok(()) }
            "pbd" => { validate_finite_parameter("PBD", value)?; self.params.p708 = value; self.mark_param_given(708); Ok(()) }
            "pbsws" => { validate_finite_parameter("PBSWS", value)?; self.params.p709 = value; self.mark_param_given(709); Ok(()) }
            "pbswd" => { validate_finite_parameter("PBSWD", value)?; self.params.p710 = value; self.mark_param_given(710); Ok(()) }
            "pbswgs" => { validate_finite_parameter("PBSWGS", value)?; self.params.p711 = value; self.mark_param_given(711); Ok(()) }
            "pbswgd" => { validate_finite_parameter("PBSWGD", value)?; self.params.p712 = value; self.mark_param_given(712); Ok(()) }
            "mjs" => { validate_finite_parameter("MJS", value)?; self.params.p713 = value; self.mark_param_given(713); Ok(()) }
            "mjd" => { validate_finite_parameter("MJD", value)?; self.params.p714 = value; self.mark_param_given(714); Ok(()) }
            "mjsws" => { validate_finite_parameter("MJSWS", value)?; self.params.p715 = value; self.mark_param_given(715); Ok(()) }
            "mjswd" => { validate_finite_parameter("MJSWD", value)?; self.params.p716 = value; self.mark_param_given(716); Ok(()) }
            "mjswgs" => { validate_finite_parameter("MJSWGS", value)?; self.params.p717 = value; self.mark_param_given(717); Ok(()) }
            "mjswgd" => { validate_finite_parameter("MJSWGD", value)?; self.params.p718 = value; self.mark_param_given(718); Ok(()) }
            "jss" => { validate_finite_parameter("JSS", value)?; self.params.p719 = value; self.mark_param_given(719); Ok(()) }
            "jsd" => { validate_finite_parameter("JSD", value)?; self.params.p720 = value; self.mark_param_given(720); Ok(()) }
            "jsws" => { validate_finite_parameter("JSWS", value)?; self.params.p721 = value; self.mark_param_given(721); Ok(()) }
            "jswd" => { validate_finite_parameter("JSWD", value)?; self.params.p722 = value; self.mark_param_given(722); Ok(()) }
            "jswgs" => { validate_finite_parameter("JSWGS", value)?; self.params.p723 = value; self.mark_param_given(723); Ok(()) }
            "jswgd" => { validate_finite_parameter("JSWGD", value)?; self.params.p724 = value; self.mark_param_given(724); Ok(()) }
            "njs" => { validate_parameter("NJS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p725 = value; self.mark_param_given(725); Ok(()) }
            "njd" => { validate_parameter("NJD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p726 = value; self.mark_param_given(726); Ok(()) }
            "ijthsfwd" => { validate_finite_parameter("IJTHSFWD", value)?; self.params.p727 = value; self.mark_param_given(727); Ok(()) }
            "ijthdfwd" => { validate_finite_parameter("IJTHDFWD", value)?; self.params.p728 = value; self.mark_param_given(728); Ok(()) }
            "ijthsrev" => { validate_finite_parameter("IJTHSREV", value)?; self.params.p729 = value; self.mark_param_given(729); Ok(()) }
            "ijthdrev" => { validate_finite_parameter("IJTHDREV", value)?; self.params.p730 = value; self.mark_param_given(730); Ok(()) }
            "bvs" => { validate_finite_parameter("BVS", value)?; self.params.p731 = value; self.mark_param_given(731); Ok(()) }
            "bvd" => { validate_finite_parameter("BVD", value)?; self.params.p732 = value; self.mark_param_given(732); Ok(()) }
            "xjbvs" => { validate_parameter("XJBVS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p733 = value; self.mark_param_given(733); Ok(()) }
            "xjbvd" => { validate_parameter("XJBVD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p734 = value; self.mark_param_given(734); Ok(()) }
            "jtss" => { validate_finite_parameter("JTSS", value)?; self.params.p735 = value; self.mark_param_given(735); Ok(()) }
            "jtsd" => { validate_finite_parameter("JTSD", value)?; self.params.p736 = value; self.mark_param_given(736); Ok(()) }
            "jtssws" => { validate_finite_parameter("JTSSWS", value)?; self.params.p737 = value; self.mark_param_given(737); Ok(()) }
            "jtsswd" => { validate_finite_parameter("JTSSWD", value)?; self.params.p738 = value; self.mark_param_given(738); Ok(()) }
            "jtsswgs" => { validate_finite_parameter("JTSSWGS", value)?; self.params.p739 = value; self.mark_param_given(739); Ok(()) }
            "jtsswgd" => { validate_finite_parameter("JTSSWGD", value)?; self.params.p740 = value; self.mark_param_given(740); Ok(()) }
            "jtweff" => { validate_parameter("JTWEFF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p741 = value; self.mark_param_given(741); Ok(()) }
            "njts" => { validate_finite_parameter("NJTS", value)?; self.params.p742 = value; self.mark_param_given(742); Ok(()) }
            "njtsd" => { validate_finite_parameter("NJTSD", value)?; self.params.p743 = value; self.mark_param_given(743); Ok(()) }
            "njtssw" => { validate_finite_parameter("NJTSSW", value)?; self.params.p744 = value; self.mark_param_given(744); Ok(()) }
            "njtsswd" => { validate_finite_parameter("NJTSSWD", value)?; self.params.p745 = value; self.mark_param_given(745); Ok(()) }
            "njtsswg" => { validate_finite_parameter("NJTSSWG", value)?; self.params.p746 = value; self.mark_param_given(746); Ok(()) }
            "njtsswgd" => { validate_finite_parameter("NJTSSWGD", value)?; self.params.p747 = value; self.mark_param_given(747); Ok(()) }
            "vtss" => { validate_finite_parameter("VTSS", value)?; self.params.p748 = value; self.mark_param_given(748); Ok(()) }
            "vtsd" => { validate_finite_parameter("VTSD", value)?; self.params.p749 = value; self.mark_param_given(749); Ok(()) }
            "vtssws" => { validate_finite_parameter("VTSSWS", value)?; self.params.p750 = value; self.mark_param_given(750); Ok(()) }
            "vtsswd" => { validate_finite_parameter("VTSSWD", value)?; self.params.p751 = value; self.mark_param_given(751); Ok(()) }
            "vtsswgs" => { validate_finite_parameter("VTSSWGS", value)?; self.params.p752 = value; self.mark_param_given(752); Ok(()) }
            "vtsswgd" => { validate_finite_parameter("VTSSWGD", value)?; self.params.p753 = value; self.mark_param_given(753); Ok(()) }
            "xrcrg1" => { validate_parameter("XRCRG1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p754 = value; self.mark_param_given(754); Ok(()) }
            "xrcrg2" => { validate_parameter("XRCRG2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p755 = value; self.mark_param_given(755); Ok(()) }
            "gbmin" => { validate_parameter("GBMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p756 = value; self.mark_param_given(756); Ok(()) }
            "rbps0" => { validate_parameter("RBPS0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p757 = value; self.mark_param_given(757); Ok(()) }
            "rbpsl" => { validate_parameter("RBPSL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p758 = value; self.mark_param_given(758); Ok(()) }
            "rbpsw" => { validate_parameter("RBPSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p759 = value; self.mark_param_given(759); Ok(()) }
            "rbpsnf" => { validate_parameter("RBPSNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p760 = value; self.mark_param_given(760); Ok(()) }
            "rbpd0" => { validate_parameter("RBPD0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p761 = value; self.mark_param_given(761); Ok(()) }
            "rbpdl" => { validate_parameter("RBPDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p762 = value; self.mark_param_given(762); Ok(()) }
            "rbpdw" => { validate_parameter("RBPDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p763 = value; self.mark_param_given(763); Ok(()) }
            "rbpdnf" => { validate_parameter("RBPDNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p764 = value; self.mark_param_given(764); Ok(()) }
            "rbpbx0" => { validate_parameter("RBPBX0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p765 = value; self.mark_param_given(765); Ok(()) }
            "rbpbxl" => { validate_parameter("RBPBXL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p766 = value; self.mark_param_given(766); Ok(()) }
            "rbpbxw" => { validate_parameter("RBPBXW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p767 = value; self.mark_param_given(767); Ok(()) }
            "rbpbxnf" => { validate_parameter("RBPBXNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p768 = value; self.mark_param_given(768); Ok(()) }
            "rbpby0" => { validate_parameter("RBPBY0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p769 = value; self.mark_param_given(769); Ok(()) }
            "rbpbyl" => { validate_parameter("RBPBYL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p770 = value; self.mark_param_given(770); Ok(()) }
            "rbpbyw" => { validate_parameter("RBPBYW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p771 = value; self.mark_param_given(771); Ok(()) }
            "rbpbynf" => { validate_parameter("RBPBYNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p772 = value; self.mark_param_given(772); Ok(()) }
            "rbsbx0" => { validate_parameter("RBSBX0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p773 = value; self.mark_param_given(773); Ok(()) }
            "rbsby0" => { validate_parameter("RBSBY0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p774 = value; self.mark_param_given(774); Ok(()) }
            "rbdbx0" => { validate_parameter("RBDBX0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p775 = value; self.mark_param_given(775); Ok(()) }
            "rbdby0" => { validate_parameter("RBDBY0", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p776 = value; self.mark_param_given(776); Ok(()) }
            "rbsdbxl" => { validate_parameter("RBSDBXL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p777 = value; self.mark_param_given(777); Ok(()) }
            "rbsdbxw" => { validate_parameter("RBSDBXW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p778 = value; self.mark_param_given(778); Ok(()) }
            "rbsdbxnf" => { validate_parameter("RBSDBXNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p779 = value; self.mark_param_given(779); Ok(()) }
            "rbsdbyl" => { validate_parameter("RBSDBYL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p780 = value; self.mark_param_given(780); Ok(()) }
            "rbsdbyw" => { validate_parameter("RBSDBYW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p781 = value; self.mark_param_given(781); Ok(()) }
            "rbsdbynf" => { validate_parameter("RBSDBYNF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p782 = value; self.mark_param_given(782); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.0, "0.0")), true, Some((2.0, "2.0")), false, &[])?; self.params.p783 = value; self.mark_param_given(783); Ok(()) }
            "em" => { validate_finite_parameter("EM", value)?; self.params.p784 = value; self.mark_param_given(784); Ok(()) }
            "noia" => { validate_finite_parameter("NOIA", value)?; self.params.p785 = value; self.mark_param_given(785); Ok(()) }
            "noia3" => { validate_finite_parameter("NOIA3", value)?; self.params.p786 = value; self.mark_param_given(786); Ok(()) }
            "lnoia3" => { validate_finite_parameter("LNOIA3", value)?; self.params.p787 = value; self.mark_param_given(787); Ok(()) }
            "wnoia3" => { validate_finite_parameter("WNOIA3", value)?; self.params.p788 = value; self.mark_param_given(788); Ok(()) }
            "pnoia3" => { validate_finite_parameter("PNOIA3", value)?; self.params.p789 = value; self.mark_param_given(789); Ok(()) }
            "mpower" => { validate_parameter("MPOWER", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p790 = value; self.mark_param_given(790); Ok(()) }
            "lmpower" => { validate_finite_parameter("LMPOWER", value)?; self.params.p791 = value; self.mark_param_given(791); Ok(()) }
            "wmpower" => { validate_finite_parameter("WMPOWER", value)?; self.params.p792 = value; self.mark_param_given(792); Ok(()) }
            "pmpower" => { validate_finite_parameter("PMPOWER", value)?; self.params.p793 = value; self.mark_param_given(793); Ok(()) }
            "qsref" => { validate_parameter("QSREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p794 = value; self.mark_param_given(794); Ok(()) }
            "lqsref" => { validate_finite_parameter("LQSREF", value)?; self.params.p795 = value; self.mark_param_given(795); Ok(()) }
            "wqsref" => { validate_finite_parameter("WQSREF", value)?; self.params.p796 = value; self.mark_param_given(796); Ok(()) }
            "pqsref" => { validate_finite_parameter("PQSREF", value)?; self.params.p797 = value; self.mark_param_given(797); Ok(()) }
            "spfn" => { validate_parameter("SPFN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p798 = value; self.mark_param_given(798); Ok(()) }
            "noib" => { validate_finite_parameter("NOIB", value)?; self.params.p799 = value; self.mark_param_given(799); Ok(()) }
            "noic" => { validate_finite_parameter("NOIC", value)?; self.params.p800 = value; self.mark_param_given(800); Ok(()) }
            "lintnoi" => { validate_finite_parameter("LINTNOI", value)?; self.params.p801 = value; self.mark_param_given(801); Ok(()) }
            "noia1" => { validate_parameter("NOIA1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p802 = value; self.mark_param_given(802); Ok(()) }
            "noiax" => { validate_parameter("NOIAX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p803 = value; self.mark_param_given(803); Ok(()) }
            "bfns" => { validate_parameter("BFNS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p804 = value; self.mark_param_given(804); Ok(()) }
            "bfnd" => { validate_parameter("BFND", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p805 = value; self.mark_param_given(805); Ok(()) }
            "kfns" => { validate_parameter("KFNS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p806 = value; self.mark_param_given(806); Ok(()) }
            "kfnd" => { validate_parameter("KFND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p807 = value; self.mark_param_given(807); Ok(()) }
            "afns" => { validate_parameter("AFNS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p808 = value; self.mark_param_given(808); Ok(()) }
            "afnd" => { validate_parameter("AFND", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p809 = value; self.mark_param_given(809); Ok(()) }
            "ntnoi" => { validate_parameter("NTNOI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p810 = value; self.mark_param_given(810); Ok(()) }
            "rnoia" => { validate_finite_parameter("RNOIA", value)?; self.params.p811 = value; self.mark_param_given(811); Ok(()) }
            "rnoib" => { validate_finite_parameter("RNOIB", value)?; self.params.p812 = value; self.mark_param_given(812); Ok(()) }
            "rnoic" => { validate_finite_parameter("RNOIC", value)?; self.params.p813 = value; self.mark_param_given(813); Ok(()) }
            "tnoia" => { validate_finite_parameter("TNOIA", value)?; self.params.p814 = value; self.mark_param_given(814); Ok(()) }
            "tnoib" => { validate_finite_parameter("TNOIB", value)?; self.params.p815 = value; self.mark_param_given(815); Ok(()) }
            "tnoic" => { validate_finite_parameter("TNOIC", value)?; self.params.p816 = value; self.mark_param_given(816); Ok(()) }
            "binunit" => { validate_parameter("BINUNIT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p817 = value; self.mark_param_given(817); Ok(()) }
            "dlbin" => { validate_finite_parameter("DLBIN", value)?; self.params.p818 = value; self.mark_param_given(818); Ok(()) }
            "dwbin" => { validate_finite_parameter("DWBIN", value)?; self.params.p819 = value; self.mark_param_given(819); Ok(()) }
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p820 = value; self.mark_param_given(820); Ok(()) }
            "tbgasub" => { validate_finite_parameter("TBGASUB", value)?; self.params.p821 = value; self.mark_param_given(821); Ok(()) }
            "tbgbsub" => { validate_finite_parameter("TBGBSUB", value)?; self.params.p822 = value; self.mark_param_given(822); Ok(()) }
            "tnfactor" => { validate_finite_parameter("TNFACTOR", value)?; self.params.p823 = value; self.mark_param_given(823); Ok(()) }
            "ute" => { validate_finite_parameter("UTE", value)?; self.params.p824 = value; self.mark_param_given(824); Ok(()) }
            "lute" => { validate_finite_parameter("LUTE", value)?; self.params.p825 = value; self.mark_param_given(825); Ok(()) }
            "wute" => { validate_finite_parameter("WUTE", value)?; self.params.p826 = value; self.mark_param_given(826); Ok(()) }
            "pute" => { validate_finite_parameter("PUTE", value)?; self.params.p827 = value; self.mark_param_given(827); Ok(()) }
            "utel" => { validate_finite_parameter("UTEL", value)?; self.params.p828 = value; self.mark_param_given(828); Ok(()) }
            "ua1" => { validate_finite_parameter("UA1", value)?; self.params.p829 = value; self.mark_param_given(829); Ok(()) }
            "lua1" => { validate_finite_parameter("LUA1", value)?; self.params.p830 = value; self.mark_param_given(830); Ok(()) }
            "wua1" => { validate_finite_parameter("WUA1", value)?; self.params.p831 = value; self.mark_param_given(831); Ok(()) }
            "pua1" => { validate_finite_parameter("PUA1", value)?; self.params.p832 = value; self.mark_param_given(832); Ok(()) }
            "ua1l" => { validate_finite_parameter("UA1L", value)?; self.params.p833 = value; self.mark_param_given(833); Ok(()) }
            "uc1" => { validate_finite_parameter("UC1", value)?; self.params.p834 = value; self.mark_param_given(834); Ok(()) }
            "luc1" => { validate_finite_parameter("LUC1", value)?; self.params.p835 = value; self.mark_param_given(835); Ok(()) }
            "wuc1" => { validate_finite_parameter("WUC1", value)?; self.params.p836 = value; self.mark_param_given(836); Ok(()) }
            "puc1" => { validate_finite_parameter("PUC1", value)?; self.params.p837 = value; self.mark_param_given(837); Ok(()) }
            "ud1" => { validate_finite_parameter("UD1", value)?; self.params.p838 = value; self.mark_param_given(838); Ok(()) }
            "lud1" => { validate_finite_parameter("LUD1", value)?; self.params.p839 = value; self.mark_param_given(839); Ok(()) }
            "wud1" => { validate_finite_parameter("WUD1", value)?; self.params.p840 = value; self.mark_param_given(840); Ok(()) }
            "pud1" => { validate_finite_parameter("PUD1", value)?; self.params.p841 = value; self.mark_param_given(841); Ok(()) }
            "ud1l" => { validate_finite_parameter("UD1L", value)?; self.params.p842 = value; self.mark_param_given(842); Ok(()) }
            "eu1" => { validate_finite_parameter("EU1", value)?; self.params.p843 = value; self.mark_param_given(843); Ok(()) }
            "leu1" => { validate_finite_parameter("LEU1", value)?; self.params.p844 = value; self.mark_param_given(844); Ok(()) }
            "weu1" => { validate_finite_parameter("WEU1", value)?; self.params.p845 = value; self.mark_param_given(845); Ok(()) }
            "peu1" => { validate_finite_parameter("PEU1", value)?; self.params.p846 = value; self.mark_param_given(846); Ok(()) }
            "ucste" => { validate_finite_parameter("UCSTE", value)?; self.params.p847 = value; self.mark_param_given(847); Ok(()) }
            "lucste" => { validate_finite_parameter("LUCSTE", value)?; self.params.p848 = value; self.mark_param_given(848); Ok(()) }
            "wucste" => { validate_finite_parameter("WUCSTE", value)?; self.params.p849 = value; self.mark_param_given(849); Ok(()) }
            "pucste" => { validate_finite_parameter("PUCSTE", value)?; self.params.p850 = value; self.mark_param_given(850); Ok(()) }
            "teta0" => { validate_finite_parameter("TETA0", value)?; self.params.p851 = value; self.mark_param_given(851); Ok(()) }
            "prt" => { validate_finite_parameter("PRT", value)?; self.params.p852 = value; self.mark_param_given(852); Ok(()) }
            "lprt" => { validate_finite_parameter("LPRT", value)?; self.params.p853 = value; self.mark_param_given(853); Ok(()) }
            "wprt" => { validate_finite_parameter("WPRT", value)?; self.params.p854 = value; self.mark_param_given(854); Ok(()) }
            "pprt" => { validate_finite_parameter("PPRT", value)?; self.params.p855 = value; self.mark_param_given(855); Ok(()) }
            "at" => { validate_finite_parameter("AT", value)?; self.params.p856 = value; self.mark_param_given(856); Ok(()) }
            "lat" => { validate_finite_parameter("LAT", value)?; self.params.p857 = value; self.mark_param_given(857); Ok(()) }
            "wat" => { validate_finite_parameter("WAT", value)?; self.params.p858 = value; self.mark_param_given(858); Ok(()) }
            "pat" => { validate_finite_parameter("PAT", value)?; self.params.p859 = value; self.mark_param_given(859); Ok(()) }
            "atl" => { validate_finite_parameter("ATL", value)?; self.params.p860 = value; self.mark_param_given(860); Ok(()) }
            "tdelta" => { validate_finite_parameter("TDELTA", value)?; self.params.p861 = value; self.mark_param_given(861); Ok(()) }
            "ptwgt" => { validate_finite_parameter("PTWGT", value)?; self.params.p862 = value; self.mark_param_given(862); Ok(()) }
            "lptwgt" => { validate_finite_parameter("LPTWGT", value)?; self.params.p863 = value; self.mark_param_given(863); Ok(()) }
            "wptwgt" => { validate_finite_parameter("WPTWGT", value)?; self.params.p864 = value; self.mark_param_given(864); Ok(()) }
            "pptwgt" => { validate_finite_parameter("PPTWGT", value)?; self.params.p865 = value; self.mark_param_given(865); Ok(()) }
            "ptwgtl" => { validate_finite_parameter("PTWGTL", value)?; self.params.p866 = value; self.mark_param_given(866); Ok(()) }
            "kt1" => { validate_finite_parameter("KT1", value)?; self.params.p867 = value; self.mark_param_given(867); Ok(()) }
            "kt1exp" => { validate_parameter("KT1EXP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p868 = value; self.mark_param_given(868); Ok(()) }
            "kt1l" => { validate_finite_parameter("KT1L", value)?; self.params.p869 = value; self.mark_param_given(869); Ok(()) }
            "lkt1" => { validate_finite_parameter("LKT1", value)?; self.params.p870 = value; self.mark_param_given(870); Ok(()) }
            "wkt1" => { validate_finite_parameter("WKT1", value)?; self.params.p871 = value; self.mark_param_given(871); Ok(()) }
            "pkt1" => { validate_finite_parameter("PKT1", value)?; self.params.p872 = value; self.mark_param_given(872); Ok(()) }
            "kt2" => { validate_finite_parameter("KT2", value)?; self.params.p873 = value; self.mark_param_given(873); Ok(()) }
            "lkt2" => { validate_finite_parameter("LKT2", value)?; self.params.p874 = value; self.mark_param_given(874); Ok(()) }
            "wkt2" => { validate_finite_parameter("WKT2", value)?; self.params.p875 = value; self.mark_param_given(875); Ok(()) }
            "pkt2" => { validate_finite_parameter("PKT2", value)?; self.params.p876 = value; self.mark_param_given(876); Ok(()) }
            "iit" => { validate_finite_parameter("IIT", value)?; self.params.p877 = value; self.mark_param_given(877); Ok(()) }
            "liit" => { validate_finite_parameter("LIIT", value)?; self.params.p878 = value; self.mark_param_given(878); Ok(()) }
            "wiit" => { validate_finite_parameter("WIIT", value)?; self.params.p879 = value; self.mark_param_given(879); Ok(()) }
            "piit" => { validate_finite_parameter("PIIT", value)?; self.params.p880 = value; self.mark_param_given(880); Ok(()) }
            "igt" => { validate_finite_parameter("IGT", value)?; self.params.p881 = value; self.mark_param_given(881); Ok(()) }
            "ligt" => { validate_finite_parameter("LIGT", value)?; self.params.p882 = value; self.mark_param_given(882); Ok(()) }
            "wigt" => { validate_finite_parameter("WIGT", value)?; self.params.p883 = value; self.mark_param_given(883); Ok(()) }
            "pigt" => { validate_finite_parameter("PIGT", value)?; self.params.p884 = value; self.mark_param_given(884); Ok(()) }
            "tgidl" => { validate_finite_parameter("TGIDL", value)?; self.params.p885 = value; self.mark_param_given(885); Ok(()) }
            "ltgidl" => { validate_finite_parameter("LTGIDL", value)?; self.params.p886 = value; self.mark_param_given(886); Ok(()) }
            "wtgidl" => { validate_finite_parameter("WTGIDL", value)?; self.params.p887 = value; self.mark_param_given(887); Ok(()) }
            "ptgidl" => { validate_finite_parameter("PTGIDL", value)?; self.params.p888 = value; self.mark_param_given(888); Ok(()) }
            "tcj" => { validate_finite_parameter("TCJ", value)?; self.params.p889 = value; self.mark_param_given(889); Ok(()) }
            "tcjsw" => { validate_finite_parameter("TCJSW", value)?; self.params.p890 = value; self.mark_param_given(890); Ok(()) }
            "tcjswg" => { validate_finite_parameter("TCJSWG", value)?; self.params.p891 = value; self.mark_param_given(891); Ok(()) }
            "tpb" => { validate_finite_parameter("TPB", value)?; self.params.p892 = value; self.mark_param_given(892); Ok(()) }
            "tpbsw" => { validate_finite_parameter("TPBSW", value)?; self.params.p893 = value; self.mark_param_given(893); Ok(()) }
            "tpbswg" => { validate_finite_parameter("TPBSWG", value)?; self.params.p894 = value; self.mark_param_given(894); Ok(()) }
            "xtis" => { validate_finite_parameter("XTIS", value)?; self.params.p895 = value; self.mark_param_given(895); Ok(()) }
            "xtid" => { validate_finite_parameter("XTID", value)?; self.params.p896 = value; self.mark_param_given(896); Ok(()) }
            "xtss" => { validate_finite_parameter("XTSS", value)?; self.params.p897 = value; self.mark_param_given(897); Ok(()) }
            "xtsd" => { validate_finite_parameter("XTSD", value)?; self.params.p898 = value; self.mark_param_given(898); Ok(()) }
            "xtssws" => { validate_finite_parameter("XTSSWS", value)?; self.params.p899 = value; self.mark_param_given(899); Ok(()) }
            "xtsswd" => { validate_finite_parameter("XTSSWD", value)?; self.params.p900 = value; self.mark_param_given(900); Ok(()) }
            "xtsswgs" => { validate_finite_parameter("XTSSWGS", value)?; self.params.p901 = value; self.mark_param_given(901); Ok(()) }
            "xtsswgd" => { validate_finite_parameter("XTSSWGD", value)?; self.params.p902 = value; self.mark_param_given(902); Ok(()) }
            "tnjts" => { validate_finite_parameter("TNJTS", value)?; self.params.p903 = value; self.mark_param_given(903); Ok(()) }
            "tnjtsd" => { validate_finite_parameter("TNJTSD", value)?; self.params.p904 = value; self.mark_param_given(904); Ok(()) }
            "tnjtssw" => { validate_finite_parameter("TNJTSSW", value)?; self.params.p905 = value; self.mark_param_given(905); Ok(()) }
            "tnjtsswd" => { validate_finite_parameter("TNJTSSWD", value)?; self.params.p906 = value; self.mark_param_given(906); Ok(()) }
            "tnjtsswg" => { validate_finite_parameter("TNJTSSWG", value)?; self.params.p907 = value; self.mark_param_given(907); Ok(()) }
            "tnjtsswgd" => { validate_finite_parameter("TNJTSSWGD", value)?; self.params.p908 = value; self.mark_param_given(908); Ok(()) }
            "rth0" => { validate_parameter("RTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p909 = value; self.mark_param_given(909); Ok(()) }
            "cth0" => { validate_parameter("CTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p910 = value; self.mark_param_given(910); Ok(()) }
            "wth0" => { validate_finite_parameter("WTH0", value)?; self.params.p911 = value; self.mark_param_given(911); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p912 = value; self.mark_param_given(912); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p913 = value; self.mark_param_given(913); Ok(()) }
            "wlod" => { validate_parameter("WLOD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p914 = value; self.mark_param_given(914); Ok(()) }
            "ku0" => { validate_finite_parameter("KU0", value)?; self.params.p915 = value; self.mark_param_given(915); Ok(()) }
            "kvsat" => { validate_finite_parameter("KVSAT", value)?; self.params.p916 = value; self.mark_param_given(916); Ok(()) }
            "tku0" => { validate_finite_parameter("TKU0", value)?; self.params.p917 = value; self.mark_param_given(917); Ok(()) }
            "lku0" => { validate_finite_parameter("LKU0", value)?; self.params.p918 = value; self.mark_param_given(918); Ok(()) }
            "wku0" => { validate_finite_parameter("WKU0", value)?; self.params.p919 = value; self.mark_param_given(919); Ok(()) }
            "pku0" => { validate_finite_parameter("PKU0", value)?; self.params.p920 = value; self.mark_param_given(920); Ok(()) }
            "llodku0" => { validate_finite_parameter("LLODKU0", value)?; self.params.p921 = value; self.mark_param_given(921); Ok(()) }
            "wlodku0" => { validate_finite_parameter("WLODKU0", value)?; self.params.p922 = value; self.mark_param_given(922); Ok(()) }
            "kvth0" => { validate_finite_parameter("KVTH0", value)?; self.params.p923 = value; self.mark_param_given(923); Ok(()) }
            "lkvth0" => { validate_finite_parameter("LKVTH0", value)?; self.params.p924 = value; self.mark_param_given(924); Ok(()) }
            "wkvth0" => { validate_finite_parameter("WKVTH0", value)?; self.params.p925 = value; self.mark_param_given(925); Ok(()) }
            "pkvth0" => { validate_finite_parameter("PKVTH0", value)?; self.params.p926 = value; self.mark_param_given(926); Ok(()) }
            "llodvth" => { validate_finite_parameter("LLODVTH", value)?; self.params.p927 = value; self.mark_param_given(927); Ok(()) }
            "wlodvth" => { validate_finite_parameter("WLODVTH", value)?; self.params.p928 = value; self.mark_param_given(928); Ok(()) }
            "stk2" => { validate_finite_parameter("STK2", value)?; self.params.p929 = value; self.mark_param_given(929); Ok(()) }
            "lodk2" => { validate_finite_parameter("LODK2", value)?; self.params.p930 = value; self.mark_param_given(930); Ok(()) }
            "steta0" => { validate_finite_parameter("STETA0", value)?; self.params.p931 = value; self.mark_param_given(931); Ok(()) }
            "lodeta0" => { validate_finite_parameter("LODETA0", value)?; self.params.p932 = value; self.mark_param_given(932); Ok(()) }
            "web" => { validate_parameter("WEB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p933 = value; self.mark_param_given(933); Ok(()) }
            "wec" => { validate_parameter("WEC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p934 = value; self.mark_param_given(934); Ok(()) }
            "kvth0we" => { validate_finite_parameter("KVTH0WE", value)?; self.params.p935 = value; self.mark_param_given(935); Ok(()) }
            "lkvth0we" => { validate_finite_parameter("LKVTH0WE", value)?; self.params.p936 = value; self.mark_param_given(936); Ok(()) }
            "wkvth0we" => { validate_finite_parameter("WKVTH0WE", value)?; self.params.p937 = value; self.mark_param_given(937); Ok(()) }
            "pkvth0we" => { validate_finite_parameter("PKVTH0WE", value)?; self.params.p938 = value; self.mark_param_given(938); Ok(()) }
            "k2we" => { validate_finite_parameter("K2WE", value)?; self.params.p939 = value; self.mark_param_given(939); Ok(()) }
            "lk2we" => { validate_finite_parameter("LK2WE", value)?; self.params.p940 = value; self.mark_param_given(940); Ok(()) }
            "wk2we" => { validate_finite_parameter("WK2WE", value)?; self.params.p941 = value; self.mark_param_given(941); Ok(()) }
            "pk2we" => { validate_finite_parameter("PK2WE", value)?; self.params.p942 = value; self.mark_param_given(942); Ok(()) }
            "ku0we" => { validate_finite_parameter("KU0WE", value)?; self.params.p943 = value; self.mark_param_given(943); Ok(()) }
            "lku0we" => { validate_finite_parameter("LKU0WE", value)?; self.params.p944 = value; self.mark_param_given(944); Ok(()) }
            "wku0we" => { validate_finite_parameter("WKU0WE", value)?; self.params.p945 = value; self.mark_param_given(945); Ok(()) }
            "pku0we" => { validate_finite_parameter("PKU0WE", value)?; self.params.p946 = value; self.mark_param_given(946); Ok(()) }
            "scref" => { validate_parameter("SCREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p947 = value; self.mark_param_given(947); Ok(()) }
            "ssl0" => { validate_finite_parameter("SSL0", value)?; self.params.p948 = value; self.mark_param_given(948); Ok(()) }
            "ssl1" => { validate_finite_parameter("SSL1", value)?; self.params.p949 = value; self.mark_param_given(949); Ok(()) }
            "ssl2" => { validate_finite_parameter("SSL2", value)?; self.params.p950 = value; self.mark_param_given(950); Ok(()) }
            "ssl3" => { validate_finite_parameter("SSL3", value)?; self.params.p951 = value; self.mark_param_given(951); Ok(()) }
            "ssl4" => { validate_finite_parameter("SSL4", value)?; self.params.p952 = value; self.mark_param_given(952); Ok(()) }
            "ssl5" => { validate_finite_parameter("SSL5", value)?; self.params.p953 = value; self.mark_param_given(953); Ok(()) }
            "sslexp1" => { validate_finite_parameter("SSLEXP1", value)?; self.params.p954 = value; self.mark_param_given(954); Ok(()) }
            "sslexp2" => { validate_finite_parameter("SSLEXP2", value)?; self.params.p955 = value; self.mark_param_given(955); Ok(()) }
            "avdsx" => { validate_parameter("AVDSX", value, Some((5.0, "5.0")), false, Some((100.0, "100.0")), true, &[])?; self.params.p956 = value; self.mark_param_given(956); Ok(()) }
            "wedge" => { validate_parameter("WEDGE", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p957 = value; self.mark_param_given(957); Ok(()) }
            "dgammaedge" => { validate_finite_parameter("DGAMMAEDGE", value)?; self.params.p958 = value; self.mark_param_given(958); Ok(()) }
            "dgammaedgel" => { validate_finite_parameter("DGAMMAEDGEL", value)?; self.params.p959 = value; self.mark_param_given(959); Ok(()) }
            "dgammaedgelexp" => { validate_finite_parameter("DGAMMAEDGELEXP", value)?; self.params.p960 = value; self.mark_param_given(960); Ok(()) }
            "dvtedge" => { validate_finite_parameter("DVTEDGE", value)?; self.params.p961 = value; self.mark_param_given(961); Ok(()) }
            "ndepedge" => { validate_finite_parameter("NDEPEDGE", value)?; self.params.p962 = value; self.mark_param_given(962); Ok(()) }
            "lndepedge" => { validate_finite_parameter("LNDEPEDGE", value)?; self.params.p963 = value; self.mark_param_given(963); Ok(()) }
            "wndepedge" => { validate_finite_parameter("WNDEPEDGE", value)?; self.params.p964 = value; self.mark_param_given(964); Ok(()) }
            "pndepedge" => { validate_finite_parameter("PNDEPEDGE", value)?; self.params.p965 = value; self.mark_param_given(965); Ok(()) }
            "nfactoredge" => { validate_finite_parameter("NFACTOREDGE", value)?; self.params.p966 = value; self.mark_param_given(966); Ok(()) }
            "lnfactoredge" => { validate_finite_parameter("LNFACTOREDGE", value)?; self.params.p967 = value; self.mark_param_given(967); Ok(()) }
            "wnfactoredge" => { validate_finite_parameter("WNFACTOREDGE", value)?; self.params.p968 = value; self.mark_param_given(968); Ok(()) }
            "pnfactoredge" => { validate_finite_parameter("PNFACTOREDGE", value)?; self.params.p969 = value; self.mark_param_given(969); Ok(()) }
            "citedge" => { validate_finite_parameter("CITEDGE", value)?; self.params.p970 = value; self.mark_param_given(970); Ok(()) }
            "lcitedge" => { validate_finite_parameter("LCITEDGE", value)?; self.params.p971 = value; self.mark_param_given(971); Ok(()) }
            "wcitedge" => { validate_finite_parameter("WCITEDGE", value)?; self.params.p972 = value; self.mark_param_given(972); Ok(()) }
            "pcitedge" => { validate_finite_parameter("PCITEDGE", value)?; self.params.p973 = value; self.mark_param_given(973); Ok(()) }
            "cdscdedge" => { validate_finite_parameter("CDSCDEDGE", value)?; self.params.p974 = value; self.mark_param_given(974); Ok(()) }
            "lcdscdedge" => { validate_finite_parameter("LCDSCDEDGE", value)?; self.params.p975 = value; self.mark_param_given(975); Ok(()) }
            "wcdscdedge" => { validate_finite_parameter("WCDSCDEDGE", value)?; self.params.p976 = value; self.mark_param_given(976); Ok(()) }
            "pcdscdedge" => { validate_finite_parameter("PCDSCDEDGE", value)?; self.params.p977 = value; self.mark_param_given(977); Ok(()) }
            "cdscbedge" => { validate_finite_parameter("CDSCBEDGE", value)?; self.params.p978 = value; self.mark_param_given(978); Ok(()) }
            "lcdscbedge" => { validate_finite_parameter("LCDSCBEDGE", value)?; self.params.p979 = value; self.mark_param_given(979); Ok(()) }
            "wcdscbedge" => { validate_finite_parameter("WCDSCBEDGE", value)?; self.params.p980 = value; self.mark_param_given(980); Ok(()) }
            "pcdscbedge" => { validate_finite_parameter("PCDSCBEDGE", value)?; self.params.p981 = value; self.mark_param_given(981); Ok(()) }
            "eta0edge" => { validate_finite_parameter("ETA0EDGE", value)?; self.params.p982 = value; self.mark_param_given(982); Ok(()) }
            "leta0edge" => { validate_finite_parameter("LETA0EDGE", value)?; self.params.p983 = value; self.mark_param_given(983); Ok(()) }
            "weta0edge" => { validate_finite_parameter("WETA0EDGE", value)?; self.params.p984 = value; self.mark_param_given(984); Ok(()) }
            "peta0edge" => { validate_finite_parameter("PETA0EDGE", value)?; self.params.p985 = value; self.mark_param_given(985); Ok(()) }
            "etabedge" => { validate_finite_parameter("ETABEDGE", value)?; self.params.p986 = value; self.mark_param_given(986); Ok(()) }
            "letabedge" => { validate_finite_parameter("LETABEDGE", value)?; self.params.p987 = value; self.mark_param_given(987); Ok(()) }
            "wetabedge" => { validate_finite_parameter("WETABEDGE", value)?; self.params.p988 = value; self.mark_param_given(988); Ok(()) }
            "petabedge" => { validate_finite_parameter("PETABEDGE", value)?; self.params.p989 = value; self.mark_param_given(989); Ok(()) }
            "kt1edge" => { validate_finite_parameter("KT1EDGE", value)?; self.params.p990 = value; self.mark_param_given(990); Ok(()) }
            "lkt1edge" => { validate_finite_parameter("LKT1EDGE", value)?; self.params.p991 = value; self.mark_param_given(991); Ok(()) }
            "wkt1edge" => { validate_finite_parameter("WKT1EDGE", value)?; self.params.p992 = value; self.mark_param_given(992); Ok(()) }
            "pkt1edge" => { validate_finite_parameter("PKT1EDGE", value)?; self.params.p993 = value; self.mark_param_given(993); Ok(()) }
            "kt1ledge" => { validate_finite_parameter("KT1LEDGE", value)?; self.params.p994 = value; self.mark_param_given(994); Ok(()) }
            "lkt1ledge" => { validate_finite_parameter("LKT1LEDGE", value)?; self.params.p995 = value; self.mark_param_given(995); Ok(()) }
            "wkt1ledge" => { validate_finite_parameter("WKT1LEDGE", value)?; self.params.p996 = value; self.mark_param_given(996); Ok(()) }
            "pkt1ledge" => { validate_finite_parameter("PKT1LEDGE", value)?; self.params.p997 = value; self.mark_param_given(997); Ok(()) }
            "kt2edge" => { validate_finite_parameter("KT2EDGE", value)?; self.params.p998 = value; self.mark_param_given(998); Ok(()) }
            "lkt2edge" => { validate_finite_parameter("LKT2EDGE", value)?; self.params.p999 = value; self.mark_param_given(999); Ok(()) }
            "wkt2edge" => { validate_finite_parameter("WKT2EDGE", value)?; self.params.p1000 = value; self.mark_param_given(1000); Ok(()) }
            "pkt2edge" => { validate_finite_parameter("PKT2EDGE", value)?; self.params.p1001 = value; self.mark_param_given(1001); Ok(()) }
            "kt1expedge" => { validate_finite_parameter("KT1EXPEDGE", value)?; self.params.p1002 = value; self.mark_param_given(1002); Ok(()) }
            "lkt1expedge" => { validate_finite_parameter("LKT1EXPEDGE", value)?; self.params.p1003 = value; self.mark_param_given(1003); Ok(()) }
            "wkt1expedge" => { validate_finite_parameter("WKT1EXPEDGE", value)?; self.params.p1004 = value; self.mark_param_given(1004); Ok(()) }
            "pkt1expedge" => { validate_finite_parameter("PKT1EXPEDGE", value)?; self.params.p1005 = value; self.mark_param_given(1005); Ok(()) }
            "tnfactoredge" => { validate_finite_parameter("TNFACTOREDGE", value)?; self.params.p1006 = value; self.mark_param_given(1006); Ok(()) }
            "ltnfactoredge" => { validate_finite_parameter("LTNFACTOREDGE", value)?; self.params.p1007 = value; self.mark_param_given(1007); Ok(()) }
            "wtnfactoredge" => { validate_finite_parameter("WTNFACTOREDGE", value)?; self.params.p1008 = value; self.mark_param_given(1008); Ok(()) }
            "ptnfactoredge" => { validate_finite_parameter("PTNFACTOREDGE", value)?; self.params.p1009 = value; self.mark_param_given(1009); Ok(()) }
            "teta0edge" => { validate_finite_parameter("TETA0EDGE", value)?; self.params.p1010 = value; self.mark_param_given(1010); Ok(()) }
            "lteta0edge" => { validate_finite_parameter("LTETA0EDGE", value)?; self.params.p1011 = value; self.mark_param_given(1011); Ok(()) }
            "wteta0edge" => { validate_finite_parameter("WTETA0EDGE", value)?; self.params.p1012 = value; self.mark_param_given(1012); Ok(()) }
            "pteta0edge" => { validate_finite_parameter("PTETA0EDGE", value)?; self.params.p1013 = value; self.mark_param_given(1013); Ok(()) }
            "dvt0edge" => { validate_finite_parameter("DVT0EDGE", value)?; self.params.p1014 = value; self.mark_param_given(1014); Ok(()) }
            "dvt1edge" => { validate_finite_parameter("DVT1EDGE", value)?; self.params.p1015 = value; self.mark_param_given(1015); Ok(()) }
            "dvt2edge" => { validate_finite_parameter("DVT2EDGE", value)?; self.params.p1016 = value; self.mark_param_given(1016); Ok(()) }
            "k2edge" => { validate_finite_parameter("K2EDGE", value)?; self.params.p1017 = value; self.mark_param_given(1017); Ok(()) }
            "lk2edge" => { validate_finite_parameter("LK2EDGE", value)?; self.params.p1018 = value; self.mark_param_given(1018); Ok(()) }
            "wk2edge" => { validate_finite_parameter("WK2EDGE", value)?; self.params.p1019 = value; self.mark_param_given(1019); Ok(()) }
            "pk2edge" => { validate_finite_parameter("PK2EDGE", value)?; self.params.p1020 = value; self.mark_param_given(1020); Ok(()) }
            "kvth0edge" => { validate_finite_parameter("KVTH0EDGE", value)?; self.params.p1021 = value; self.mark_param_given(1021); Ok(()) }
            "lkvth0edge" => { validate_finite_parameter("LKVTH0EDGE", value)?; self.params.p1022 = value; self.mark_param_given(1022); Ok(()) }
            "wkvth0edge" => { validate_finite_parameter("WKVTH0EDGE", value)?; self.params.p1023 = value; self.mark_param_given(1023); Ok(()) }
            "pkvth0edge" => { validate_finite_parameter("PKVTH0EDGE", value)?; self.params.p1024 = value; self.mark_param_given(1024); Ok(()) }
            "kvth0edgewe" => { validate_finite_parameter("KVTH0EDGEWE", value)?; self.params.p1025 = value; self.mark_param_given(1025); Ok(()) }
            "lkvth0edgewe" => { validate_finite_parameter("LKVTH0EDGEWE", value)?; self.params.p1026 = value; self.mark_param_given(1026); Ok(()) }
            "wkvth0edgewe" => { validate_finite_parameter("WKVTH0EDGEWE", value)?; self.params.p1027 = value; self.mark_param_given(1027); Ok(()) }
            "pkvth0edgewe" => { validate_finite_parameter("PKVTH0EDGEWE", value)?; self.params.p1028 = value; self.mark_param_given(1028); Ok(()) }
            "k2edgewe" => { validate_finite_parameter("K2EDGEWE", value)?; self.params.p1029 = value; self.mark_param_given(1029); Ok(()) }
            "lk2edgewe" => { validate_finite_parameter("LK2EDGEWE", value)?; self.params.p1030 = value; self.mark_param_given(1030); Ok(()) }
            "wk2edgewe" => { validate_finite_parameter("WK2EDGEWE", value)?; self.params.p1031 = value; self.mark_param_given(1031); Ok(()) }
            "pk2edgewe" => { validate_finite_parameter("PK2EDGEWE", value)?; self.params.p1032 = value; self.mark_param_given(1032); Ok(()) }
            "stk2edge" => { validate_finite_parameter("STK2EDGE", value)?; self.params.p1033 = value; self.mark_param_given(1033); Ok(()) }
            "lstk2edge" => { validate_finite_parameter("LSTK2EDGE", value)?; self.params.p1034 = value; self.mark_param_given(1034); Ok(()) }
            "wstk2edge" => { validate_finite_parameter("WSTK2EDGE", value)?; self.params.p1035 = value; self.mark_param_given(1035); Ok(()) }
            "pstk2edge" => { validate_finite_parameter("PSTK2EDGE", value)?; self.params.p1036 = value; self.mark_param_given(1036); Ok(()) }
            "steta0edge" => { validate_finite_parameter("STETA0EDGE", value)?; self.params.p1037 = value; self.mark_param_given(1037); Ok(()) }
            "lsteta0edge" => { validate_finite_parameter("LSTETA0EDGE", value)?; self.params.p1038 = value; self.mark_param_given(1038); Ok(()) }
            "wsteta0edge" => { validate_finite_parameter("WSTETA0EDGE", value)?; self.params.p1039 = value; self.mark_param_given(1039); Ok(()) }
            "psteta0edge" => { validate_finite_parameter("PSTETA0EDGE", value)?; self.params.p1040 = value; self.mark_param_given(1040); Ok(()) }
            "igclamp" => { validate_parameter("IGCLAMP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1041 = value; self.mark_param_given(1041); Ok(()) }
            "lp" => { validate_parameter("LP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1042 = value; self.mark_param_given(1042); Ok(()) }
            "rnoik" => { validate_finite_parameter("RNOIK", value)?; self.params.p1043 = value; self.mark_param_given(1043); Ok(()) }
            "tnoik" => { validate_finite_parameter("TNOIK", value)?; self.params.p1044 = value; self.mark_param_given(1044); Ok(()) }
            "tnoik2" => { validate_parameter("TNOIK2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1045 = value; self.mark_param_given(1045); Ok(()) }
            "k0" => { validate_finite_parameter("K0", value)?; self.params.p1046 = value; self.mark_param_given(1046); Ok(()) }
            "lk0" => { validate_finite_parameter("LK0", value)?; self.params.p1047 = value; self.mark_param_given(1047); Ok(()) }
            "wk0" => { validate_finite_parameter("WK0", value)?; self.params.p1048 = value; self.mark_param_given(1048); Ok(()) }
            "pk0" => { validate_finite_parameter("PK0", value)?; self.params.p1049 = value; self.mark_param_given(1049); Ok(()) }
            "k01" => { validate_finite_parameter("K01", value)?; self.params.p1050 = value; self.mark_param_given(1050); Ok(()) }
            "lk01" => { validate_finite_parameter("LK01", value)?; self.params.p1051 = value; self.mark_param_given(1051); Ok(()) }
            "wk01" => { validate_finite_parameter("WK01", value)?; self.params.p1052 = value; self.mark_param_given(1052); Ok(()) }
            "pk01" => { validate_finite_parameter("PK01", value)?; self.params.p1053 = value; self.mark_param_given(1053); Ok(()) }
            "m0" => { validate_finite_parameter("M0", value)?; self.params.p1054 = value; self.mark_param_given(1054); Ok(()) }
            "lm0" => { validate_finite_parameter("LM0", value)?; self.params.p1055 = value; self.mark_param_given(1055); Ok(()) }
            "wm0" => { validate_finite_parameter("WM0", value)?; self.params.p1056 = value; self.mark_param_given(1056); Ok(()) }
            "pm0" => { validate_finite_parameter("PM0", value)?; self.params.p1057 = value; self.mark_param_given(1057); Ok(()) }
            "m01" => { validate_finite_parameter("M01", value)?; self.params.p1058 = value; self.mark_param_given(1058); Ok(()) }
            "lm01" => { validate_finite_parameter("LM01", value)?; self.params.p1059 = value; self.mark_param_given(1059); Ok(()) }
            "wm01" => { validate_finite_parameter("WM01", value)?; self.params.p1060 = value; self.mark_param_given(1060); Ok(()) }
            "pm01" => { validate_finite_parameter("PM01", value)?; self.params.p1061 = value; self.mark_param_given(1061); Ok(()) }
            "nedge" => { validate_parameter("NEDGE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1062 = value; self.mark_param_given(1062); Ok(()) }
            "noia1_edge" => { validate_parameter("NOIA1_EDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1063 = value; self.mark_param_given(1063); Ok(()) }
            "noiax_edge" => { validate_parameter("NOIAX_EDGE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1064 = value; self.mark_param_given(1064); Ok(()) }
            "fnoimod" => { validate_parameter("FNOIMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1065 = value; self.mark_param_given(1065); Ok(()) }
            "lh" => { validate_parameter("LH", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1066 = value; self.mark_param_given(1066); Ok(()) }
            "noia2" => { validate_finite_parameter("NOIA2", value)?; self.params.p1067 = value; self.mark_param_given(1067); Ok(()) }
            "hndep" => { validate_parameter("HNDEP", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1068 = value; self.mark_param_given(1068); Ok(()) }
            "c0" => { validate_finite_parameter("C0", value)?; self.params.p1069 = value; self.mark_param_given(1069); Ok(()) }
            "lc0" => { validate_finite_parameter("LC0", value)?; self.params.p1070 = value; self.mark_param_given(1070); Ok(()) }
            "wc0" => { validate_finite_parameter("WC0", value)?; self.params.p1071 = value; self.mark_param_given(1071); Ok(()) }
            "pc0" => { validate_finite_parameter("PC0", value)?; self.params.p1072 = value; self.mark_param_given(1072); Ok(()) }
            "c01" => { validate_finite_parameter("C01", value)?; self.params.p1073 = value; self.mark_param_given(1073); Ok(()) }
            "lc01" => { validate_finite_parameter("LC01", value)?; self.params.p1074 = value; self.mark_param_given(1074); Ok(()) }
            "wc01" => { validate_finite_parameter("WC01", value)?; self.params.p1075 = value; self.mark_param_given(1075); Ok(()) }
            "pc01" => { validate_finite_parameter("PC01", value)?; self.params.p1076 = value; self.mark_param_given(1076); Ok(()) }
            "c0si" => { validate_finite_parameter("C0SI", value)?; self.params.p1077 = value; self.mark_param_given(1077); Ok(()) }
            "lc0si" => { validate_finite_parameter("LC0SI", value)?; self.params.p1078 = value; self.mark_param_given(1078); Ok(()) }
            "wc0si" => { validate_finite_parameter("WC0SI", value)?; self.params.p1079 = value; self.mark_param_given(1079); Ok(()) }
            "pc0si" => { validate_finite_parameter("PC0SI", value)?; self.params.p1080 = value; self.mark_param_given(1080); Ok(()) }
            "c0si1" => { validate_finite_parameter("C0SI1", value)?; self.params.p1081 = value; self.mark_param_given(1081); Ok(()) }
            "lc0si1" => { validate_finite_parameter("LC0SI1", value)?; self.params.p1082 = value; self.mark_param_given(1082); Ok(()) }
            "wc0si1" => { validate_finite_parameter("WC0SI1", value)?; self.params.p1083 = value; self.mark_param_given(1083); Ok(()) }
            "pc0si1" => { validate_finite_parameter("PC0SI1", value)?; self.params.p1084 = value; self.mark_param_given(1084); Ok(()) }
            "c0sisat" => { validate_finite_parameter("C0SISAT", value)?; self.params.p1085 = value; self.mark_param_given(1085); Ok(()) }
            "lc0sisat" => { validate_finite_parameter("LC0SISAT", value)?; self.params.p1086 = value; self.mark_param_given(1086); Ok(()) }
            "wc0sisat" => { validate_finite_parameter("WC0SISAT", value)?; self.params.p1087 = value; self.mark_param_given(1087); Ok(()) }
            "pc0sisat" => { validate_finite_parameter("PC0SISAT", value)?; self.params.p1088 = value; self.mark_param_given(1088); Ok(()) }
            "c0sisat1" => { validate_finite_parameter("C0SISAT1", value)?; self.params.p1089 = value; self.mark_param_given(1089); Ok(()) }
            "lc0sisat1" => { validate_finite_parameter("LC0SISAT1", value)?; self.params.p1090 = value; self.mark_param_given(1090); Ok(()) }
            "wc0sisat1" => { validate_finite_parameter("WC0SISAT1", value)?; self.params.p1091 = value; self.mark_param_given(1091); Ok(()) }
            "pc0sisat1" => { validate_finite_parameter("PC0SISAT1", value)?; self.params.p1092 = value; self.mark_param_given(1092); Ok(()) }
            "minr" => { validate_parameter("minr", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1093 = value; self.mark_param_given(1093); Ok(()) }
            "hvmod" => { validate_parameter("HVMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1094 = value; self.mark_param_given(1094); Ok(()) }
            "hvcap" => { validate_parameter("HVCAP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1095 = value; self.mark_param_given(1095); Ok(()) }
            "hvcaps" => { validate_parameter("HVCAPS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1096 = value; self.mark_param_given(1096); Ok(()) }
            "rbodyhvmod" => { validate_parameter("RBODYHVMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1097 = value; self.mark_param_given(1097); Ok(()) }
            "iimod" => { validate_parameter("IIMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1098 = value; self.mark_param_given(1098); Ok(()) }
            "ndriftd" => { validate_parameter("NDRIFTD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1099 = value; self.mark_param_given(1099); Ok(()) }
            "vdrift" => { validate_parameter("VDRIFT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1100 = value; self.mark_param_given(1100); Ok(()) }
            "ptwghv" => { validate_parameter("PTWGHV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1101 = value; self.mark_param_given(1101); Ok(()) }
            "ptwghv1" => { validate_finite_parameter("PTWGHV1", value)?; self.params.p1102 = value; self.mark_param_given(1102); Ok(()) }
            "psatxhv" => { validate_parameter("PSATXHV", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1103 = value; self.mark_param_given(1103); Ok(()) }
            "ptwghvii" => { validate_parameter("PTWGHVII", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1104 = value; self.mark_param_given(1104); Ok(()) }
            "ptwghv1ii" => { validate_finite_parameter("PTWGHV1II", value)?; self.params.p1105 = value; self.mark_param_given(1105); Ok(()) }
            "psatxhvii" => { validate_parameter("PSATXHVII", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1106 = value; self.mark_param_given(1106); Ok(()) }
            "mdrift" => { validate_parameter("MDRIFT", value, Some((0.5, "0.5")), true, Some((4.0, "4.0")), true, &[])?; self.params.p1107 = value; self.mark_param_given(1107); Ok(()) }
            "dsmooth" => { validate_parameter("DSMOOTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1108 = value; self.mark_param_given(1108); Ok(()) }
            "ndrifts" => { validate_parameter("NDRIFTS", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1109 = value; self.mark_param_given(1109); Ok(()) }
            "rdlcw" => { validate_parameter("RDLCW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1110 = value; self.mark_param_given(1110); Ok(()) }
            "rdlcwcv" => { validate_parameter("RDLCWCV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1111 = value; self.mark_param_given(1111); Ok(()) }
            "rslcw" => { validate_parameter("RSLCW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1112 = value; self.mark_param_given(1112); Ok(()) }
            "pdrwb" => { validate_finite_parameter("PDRWB", value)?; self.params.p1113 = value; self.mark_param_given(1113); Ok(()) }
            "vfbov" => { validate_finite_parameter("VFBOV", value)?; self.params.p1114 = value; self.mark_param_given(1114); Ok(()) }
            "lover" => { validate_finite_parameter("LOVER", value)?; self.params.p1115 = value; self.mark_param_given(1115); Ok(()) }
            "loveracc" => { validate_finite_parameter("LOVERACC", value)?; self.params.p1116 = value; self.mark_param_given(1116); Ok(()) }
            "ndr" => { validate_parameter("NDR", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1117 = value; self.mark_param_given(1117); Ok(()) }
            "slhv" => { validate_parameter("SLHV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1118 = value; self.mark_param_given(1118); Ok(()) }
            "slhv1" => { validate_parameter("SLHV1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1119 = value; self.mark_param_given(1119); Ok(()) }
            "prthv" => { validate_finite_parameter("PRTHV", value)?; self.params.p1120 = value; self.mark_param_given(1120); Ok(()) }
            "athv" => { validate_finite_parameter("ATHV", value)?; self.params.p1121 = value; self.mark_param_given(1121); Ok(()) }
            "hvfactor" => { validate_parameter("HVFACTOR", value, Some((0.0001, "0.0001")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1122 = value; self.mark_param_given(1122); Ok(()) }
            "asymp" => { validate_finite_parameter("ASYMP", value)?; self.params.p1123 = value; self.mark_param_given(1123); Ok(()) }
            "drb1" => { validate_finite_parameter("DRB1", value)?; self.params.p1124 = value; self.mark_param_given(1124); Ok(()) }
            "drb2" => { validate_finite_parameter("DRB2", value)?; self.params.p1125 = value; self.mark_param_given(1125); Ok(()) }
            "rdvds" => { validate_finite_parameter("RDVDS", value)?; self.params.p1126 = value; self.mark_param_given(1126); Ok(()) }
            "gadrift" => { validate_parameter("GADRIFT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1127 = value; self.mark_param_given(1127); Ok(()) }
            "xpart" => { validate_parameter("XPART", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p1128 = value; self.mark_param_given(1128); Ok(()) }
            "abulk" => { validate_parameter("ABULK", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), true, &[])?; self.params.p1129 = value; self.mark_param_given(1129); Ok(()) }
            "a0" => { validate_finite_parameter("A0", value)?; self.params.p1130 = value; self.mark_param_given(1130); Ok(()) }
            "ags" => { validate_finite_parameter("AGS", value)?; self.params.p1131 = value; self.mark_param_given(1131); Ok(()) }
            "ags1" => { validate_parameter("AGS1", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1132 = value; self.mark_param_given(1132); Ok(()) }
            "keta" => { validate_parameter("KETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1133 = value; self.mark_param_given(1133); Ok(()) }
            "a0cv" => { validate_finite_parameter("A0CV", value)?; self.params.p1134 = value; self.mark_param_given(1134); Ok(()) }
            "agscv" => { validate_finite_parameter("AGSCV", value)?; self.params.p1135 = value; self.mark_param_given(1135); Ok(()) }
            "ketacv" => { validate_parameter("KETACV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p1136 = value; self.mark_param_given(1136); Ok(()) }
            "cvslope" => { validate_parameter("CVSLOPE", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p1137 = value; self.mark_param_given(1137); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimbulk'", name)),
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
    pub fn set_timepoint(&mut self, time: f64, timestep: f64) {
        self.time = time;
        self.timestep = timestep;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        let mut index = 0usize;
        while index < Self::DDT_STATE_COUNT {
            self.ddt_state_previous[index] = self.ddt_state_current[index];
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
        self.ddt_state_current[slot] = value;
        if self.timestep.abs() > Self::DDT_EPSILON {
            (value - previous) / self.timestep
        } else {
            self.ddt_state_previous[slot] = value;
            self.ddt_state_initialized[slot] = true;
            0.0
        }
    }

    #[inline]
    pub(crate) fn ddt_jacobian(&self, derivative: f64) -> f64 {
        if self.timestep.abs() > Self::DDT_EPSILON {
            derivative / self.timestep
        } else {
            0.0
        }
    }
}
