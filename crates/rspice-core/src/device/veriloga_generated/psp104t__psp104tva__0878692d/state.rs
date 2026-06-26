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
        params.p0 = 1e-6;
        params.p1 = 1e-6;
        params.p2 = 0.0;
        params.p3 = 0.0;
        params.p4 = 0.0;
        params.p5 = 0.0;
        params.p6 = 0.0;
        params.p7 = 0.0;
        params.p8 = 0.0;
        params.p9 = 1.0;
        params.p10 = 1.0;
        params.p11 = 1e-7;
        params.p12 = 0.0;
        params.p13 = 0.0;
        params.p14 = 1e-6;
        params.p15 = 0.0;
        params.p16 = 1.0;
        params.p17 = 0.0;
        params.p18 = 1.0;
        params.p19 = 1e-12;
        params.p20 = 1e-6;
        params.p21 = 1e-6;
        params.p22 = 1e-12;
        params.p23 = 1e-6;
        params.p24 = 1e-6;
        params.p25 = 1e-12;
        params.p26 = 1e-6;
        params.p27 = 1e-12;
        params.p28 = 1e-6;
        params.p29 = 1.0;
        params.p30 = 1.0;
        params.p31 = 1.0;
        params.p32 = 1.0;
        params.p33 = 1.0;
        params.p34 = params.p32;
        validate_parameter("MULT_FN", params.p34, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
        params.p35 = 0.0;
        params.p36 = 104.0;
        params.p37 = 1.0;
        params.p38 = 21.0;
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
        params.p50 = 1.0;
        params.p51 = 1.0;
        params.p52 = 0.0;
        params.p53 = 0.0;
        params.p54 = 0.0;
        params.p55 = 0.0;
        params.p56 = -1.0;
        params.p57 = 0.0005;
        params.p58 = 0.0;
        params.p59 = 2e-9;
        params.p60 = 3.9;
        params.p61 = 5e23;
        params.p62 = 1.0;
        params.p63 = 0.0;
        params.p64 = 1.0;
        params.p65 = 0.0;
        params.p66 = 1e26;
        params.p67 = 2e-9;
        params.p68 = 2e-9;
        params.p69 = 5e25;
        params.p70 = 5e25;
        params.p71 = 0.0;
        params.p72 = 0.0;
        params.p73 = 0.0;
        params.p74 = 1.0;
        params.p75 = 0.0;
        params.p76 = 0.0;
        params.p77 = 0.0;
        params.p78 = 0.0;
        params.p79 = 0.0;
        params.p80 = 0.0;
        params.p81 = 0.03;
        params.p82 = 1.0;
        params.p83 = 0.5;
        params.p84 = 0.0;
        params.p85 = 1.5;
        params.p86 = 1.5;
        params.p87 = 0.0;
        params.p88 = 0.0;
        params.p89 = 2.0;
        params.p90 = 0.0;
        params.p91 = 0.0;
        params.p92 = 0.0;
        params.p93 = 1.0;
        params.p94 = 50.0;
        params.p95 = 1.0;
        params.p96 = 0.0;
        params.p97 = 0.0;
        params.p98 = 0.3;
        params.p99 = 1.0;
        params.p100 = 0.0;
        params.p101 = 0.0;
        params.p102 = 1.0;
        params.p103 = 8.0;
        params.p104 = 0.01;
        params.p105 = 0.0;
        params.p106 = 0.0;
        params.p107 = 0.05;
        params.p108 = 1.0;
        params.p109 = 10.0;
        params.p110 = 0.0;
        params.p111 = 1.0;
        params.p112 = 0.0;
        params.p113 = 10.0;
        params.p114 = 0.0;
        params.p115 = 0.0;
        params.p116 = 0.0;
        params.p117 = 0.0;
        params.p118 = 2.0;
        params.p119 = 0.375;
        params.p120 = 0.063;
        params.p121 = 0.375;
        params.p122 = 0.063;
        params.p123 = 0.375;
        params.p124 = 0.063;
        params.p125 = 3.1;
        params.p126 = 0.0;
        params.p127 = 0.0;
        params.p128 = 41.0;
        params.p129 = 41.0;
        params.p130 = 0.0;
        params.p131 = 0.0;
        params.p132 = 0.0;
        params.p133 = 0.0;
        params.p134 = 1e-14;
        params.p135 = 0.0;
        params.p136 = 1.0;
        params.p137 = 0.1;
        params.p138 = 8.0;
        params.p139 = 0.0;
        params.p140 = 0.0;
        params.p141 = 1e-15;
        params.p142 = 1e-15;
        params.p143 = 0.5;
        params.p144 = 0.5;
        params.p145 = 1.0;
        params.p146 = 1e-15;
        params.p147 = 5e-16;
        params.p148 = 5e-16;
        params.p149 = 0.0;
        params.p150 = 0.3;
        params.p151 = 0.5;
        params.p152 = 0.4;
        params.p153 = 1e-15;
        params.p154 = 1e-15;
        params.p155 = 1.0;
        params.p156 = 0.0;
        params.p157 = 8e22;
        params.p158 = 30000000.0;
        params.p159 = 0.0;
        params.p160 = 1.0;
        params.p161 = -1.0;
        params.p162 = 0.0005;
        params.p163 = 0.0;
        params.p164 = 5e23;
        params.p165 = 0.0;
        params.p166 = 0.0006;
        params.p167 = 1.0;
        params.p168 = 0.0;
        params.p169 = 0.0;
        params.p170 = 0.0;
        params.p171 = 0.0;
        params.p172 = 0.0;
        params.p173 = 0.0;
        params.p174 = 1.0;
        params.p175 = 4e24;
        params.p176 = 1500000000.0;
        params.p177 = 0.0;
        params.p178 = 1.0;
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
        params.p194 = 0.0;
        params.p195 = 0.0;
        params.p196 = 0.0;
        params.p197 = 0.0;
        params.p198 = 0.0;
        params.p199 = -1.0;
        params.p200 = 0.0;
        params.p201 = 1.0;
        params.p202 = 0.0;
        params.p203 = 0.0;
        params.p204 = 0.0005;
        params.p205 = 0.0;
        params.p206 = 0.0;
        params.p207 = 0.0;
        params.p208 = 0.0;
        params.p209 = 2e-9;
        params.p210 = 3.9;
        params.p211 = 4e23;
        params.p212 = 0.0;
        params.p213 = 1e-8;
        params.p214 = 1e24;
        params.p215 = 0.0;
        params.p216 = 1e-8;
        params.p217 = 1e-8;
        params.p218 = 0.0;
        params.p219 = 0.0;
        params.p220 = 0.0;
        params.p221 = 1.0;
        params.p222 = 0.0;
        params.p223 = 1.0;
        params.p224 = 0.0;
        params.p225 = 0.0;
        params.p226 = 0.0;
        params.p227 = 1.0;
        params.p228 = 0.0;
        params.p229 = 0.0;
        params.p230 = 1.0;
        params.p231 = 0.0;
        params.p232 = 0.0;
        params.p233 = 1e26;
        params.p234 = 0.0;
        params.p235 = 2e-9;
        params.p236 = 2e-9;
        params.p237 = 1e-8;
        params.p238 = 1e-8;
        params.p239 = 5e25;
        params.p240 = 5e25;
        params.p241 = 0.0;
        params.p242 = 0.0;
        params.p243 = 1.0;
        params.p244 = 0.0;
        params.p245 = 0.0;
        params.p246 = 0.0;
        params.p247 = 0.0;
        params.p248 = 1.0;
        params.p249 = 0.0;
        params.p250 = 2.0;
        params.p251 = 0.0;
        params.p252 = 0.0;
        params.p253 = 0.0;
        params.p254 = 0.0;
        params.p255 = 2.0;
        params.p256 = 0.0;
        params.p257 = 0.0;
        params.p258 = 0.0;
        params.p259 = 0.03;
        params.p260 = 0.0;
        params.p261 = 0.0;
        params.p262 = 1e-8;
        params.p263 = 0.0;
        params.p264 = 0.0;
        params.p265 = 1e-8;
        params.p266 = 0.0;
        params.p267 = 0.0;
        params.p268 = 1e-9;
        params.p269 = 1.0;
        params.p270 = 0.0;
        params.p271 = 0.0;
        params.p272 = 0.0;
        params.p273 = 0.5;
        params.p274 = 0.0;
        params.p275 = 0.0;
        params.p276 = 1.5;
        params.p277 = 1.5;
        params.p278 = 0.0;
        params.p279 = 0.0;
        params.p280 = 1.0;
        params.p281 = 0.0;
        params.p282 = 0.0;
        params.p283 = 0.0;
        params.p284 = 2.0;
        params.p285 = 0.0;
        params.p286 = 0.0;
        params.p287 = 0.0;
        params.p288 = 0.0;
        params.p289 = 0.0;
        params.p290 = 0.0;
        params.p291 = 1.0;
        params.p292 = 50.0;
        params.p293 = 0.0;
        params.p294 = 1.0;
        params.p295 = 0.0;
        params.p296 = 0.0;
        params.p297 = 0.0;
        params.p298 = 0.3;
        params.p299 = 1.0;
        params.p300 = 0.0;
        params.p301 = 0.0;
        params.p302 = 1.0;
        params.p303 = 0.0;
        params.p304 = 0.0;
        params.p305 = 0.0;
        params.p306 = 0.0;
        params.p307 = 0.0;
        params.p308 = 1.0;
        params.p309 = 16.0;
        params.p310 = 1.0;
        params.p311 = 0.01;
        params.p312 = 1.0;
        params.p313 = 0.0;
        params.p314 = 0.0;
        params.p315 = 0.5;
        params.p316 = 0.0;
        params.p317 = 0.0;
        params.p318 = 0.0;
        params.p319 = 0.5;
        params.p320 = 0.0;
        params.p321 = 0.0;
        params.p322 = 0.05;
        params.p323 = 1.0;
        params.p324 = 0.0;
        params.p325 = 0.0;
        params.p326 = 10.0;
        params.p327 = 0.0;
        params.p328 = 1.0;
        params.p329 = 0.0;
        params.p330 = 0.0;
        params.p331 = 0.0;
        params.p332 = 0.0;
        params.p333 = 0.0;
        params.p334 = 10.0;
        params.p335 = 0.0;
        params.p336 = 0.0;
        params.p337 = 0.0;
        params.p338 = 0.0;
        params.p339 = 2.0;
        params.p340 = 0.375;
        params.p341 = 0.063;
        params.p342 = 0.375;
        params.p343 = 0.063;
        params.p344 = 0.375;
        params.p345 = 0.063;
        params.p346 = 3.1;
        params.p347 = 0.0;
        params.p348 = 0.0;
        params.p349 = 41.0;
        params.p350 = 41.0;
        params.p351 = 0.0;
        params.p352 = 0.0;
        params.p353 = 0.0;
        params.p354 = 0.0;
        params.p355 = 0.0;
        params.p356 = 0.0;
        params.p357 = 1.0;
        params.p358 = 0.0;
        params.p359 = 0.0;
        params.p360 = 1.0;
        params.p361 = 0.0;
        params.p362 = 0.0;
        params.p363 = 0.0;
        params.p364 = 0.0;
        params.p365 = 0.1;
        params.p366 = 1.0;
        params.p367 = 0.0;
        params.p368 = 0.0;
        params.p369 = 16.0;
        params.p370 = 1.0;
        params.p371 = 0.0;
        params.p372 = 1.0;
        params.p373 = 0.0;
        params.p374 = 0.0;
        params.p375 = 0.5;
        params.p376 = 0.0;
        params.p377 = 0.0;
        params.p378 = 0.5;
        params.p379 = 0.5;
        params.p380 = 1.0;
        params.p381 = 1e-15;
        params.p382 = 5e-16;
        params.p383 = 5e-16;
        params.p384 = 0.0;
        params.p385 = 0.3;
        params.p386 = 0.5;
        params.p387 = 0.4;
        params.p388 = 1e-15;
        params.p389 = 1e-15;
        params.p390 = 1.0;
        params.p391 = 0.0;
        params.p392 = 8e22;
        params.p393 = 30000000.0;
        params.p394 = 0.0;
        params.p395 = 1.0;
        params.p396 = 0.0;
        params.p397 = 2.0;
        params.p398 = 1e-8;
        params.p399 = 0.0;
        params.p400 = -1.0;
        params.p401 = 0.0005;
        params.p402 = 0.0;
        params.p403 = 0.0;
        params.p404 = 0.0;
        params.p405 = 0.0;
        params.p406 = 0.0;
        params.p407 = 1.0;
        params.p408 = 0.0;
        params.p409 = 0.0;
        params.p410 = 5e23;
        params.p411 = 0.0;
        params.p412 = 1.0;
        params.p413 = 0.0;
        params.p414 = 0.0;
        params.p415 = 0.0;
        params.p416 = 0.0;
        params.p417 = 1.0;
        params.p418 = 0.0;
        params.p419 = 1e-8;
        params.p420 = 0.0;
        params.p421 = 1.0;
        params.p422 = 0.0;
        params.p423 = 0.0;
        params.p424 = 0.0;
        params.p425 = 0.0;
        params.p426 = 2.0;
        params.p427 = 0.0;
        params.p428 = 0.0;
        params.p429 = 0.0;
        params.p430 = 0.0;
        params.p431 = 2.0;
        params.p432 = 0.0;
        params.p433 = 0.0;
        params.p434 = 0.0;
        params.p435 = 1.0;
        params.p436 = 8e22;
        params.p437 = 30000000.0;
        params.p438 = 0.0;
        params.p439 = 1.0;
        params.p440 = 0.0;
        params.p441 = 0.0;
        params.p442 = 0.0;
        params.p443 = 0.0;
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
        params.p460 = -1.0;
        params.p461 = 0.0;
        params.p462 = 0.0;
        params.p463 = 0.0;
        params.p464 = 0.0005;
        params.p465 = 0.0;
        params.p466 = 0.0;
        params.p467 = 0.0;
        params.p468 = 5e23;
        params.p469 = 0.0;
        params.p470 = 0.0;
        params.p471 = 0.0;
        params.p472 = 1.0;
        params.p473 = 0.0;
        params.p474 = 0.0;
        params.p475 = 0.0;
        params.p476 = 0.0;
        params.p477 = 0.0;
        params.p478 = 0.0;
        params.p479 = 0.0;
        params.p480 = 0.0;
        params.p481 = 0.0;
        params.p482 = 0.0;
        params.p483 = 0.0;
        params.p484 = 1e26;
        params.p485 = 0.0;
        params.p486 = 0.0;
        params.p487 = 0.0;
        params.p488 = 5e25;
        params.p489 = 0.0;
        params.p490 = 0.0;
        params.p491 = 0.0;
        params.p492 = 5e25;
        params.p493 = 0.0;
        params.p494 = 0.0;
        params.p495 = 0.0;
        params.p496 = 0.0;
        params.p497 = 0.0;
        params.p498 = 0.0;
        params.p499 = 0.0;
        params.p500 = 0.0;
        params.p501 = 0.0;
        params.p502 = 0.0;
        params.p503 = 0.0;
        params.p504 = 0.0;
        params.p505 = 0.0;
        params.p506 = 0.0;
        params.p507 = 0.0;
        params.p508 = 1.0;
        params.p509 = 0.0;
        params.p510 = 0.0;
        params.p511 = 0.0;
        params.p512 = 0.0;
        params.p513 = 0.0;
        params.p514 = 0.0;
        params.p515 = 0.0;
        params.p516 = 0.0;
        params.p517 = 0.0;
        params.p518 = 0.0;
        params.p519 = 0.0;
        params.p520 = 0.0;
        params.p521 = 0.0;
        params.p522 = 0.0;
        params.p523 = 0.0;
        params.p524 = 0.0;
        params.p525 = 0.0;
        params.p526 = 0.0;
        params.p527 = 0.0;
        params.p528 = 0.0;
        params.p529 = 0.0;
        params.p530 = 0.0;
        params.p531 = 0.0;
        params.p532 = 0.0;
        params.p533 = 0.0;
        params.p534 = 0.0;
        params.p535 = 0.0;
        params.p536 = 0.03;
        params.p537 = 0.0;
        params.p538 = 0.0;
        params.p539 = 0.0;
        params.p540 = 1.0;
        params.p541 = 0.0;
        params.p542 = 0.0;
        params.p543 = 0.0;
        params.p544 = 0.5;
        params.p545 = 0.0;
        params.p546 = 0.0;
        params.p547 = 0.0;
        params.p548 = 1.5;
        params.p549 = 0.0;
        params.p550 = 0.0;
        params.p551 = 0.0;
        params.p552 = 0.0;
        params.p553 = 0.0;
        params.p554 = 0.0;
        params.p555 = 0.0;
        params.p556 = 2.0;
        params.p557 = 0.0;
        params.p558 = 0.0;
        params.p559 = 0.0;
        params.p560 = 0.0;
        params.p561 = 0.0;
        params.p562 = 0.0;
        params.p563 = 0.0;
        params.p564 = 50.0;
        params.p565 = 0.0;
        params.p566 = 0.0;
        params.p567 = 0.0;
        params.p568 = 1.0;
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
        params.p580 = 0.3;
        params.p581 = 0.0;
        params.p582 = 0.0;
        params.p583 = 0.0;
        params.p584 = 1.0;
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
        params.p596 = 8.0;
        params.p597 = 0.0;
        params.p598 = 0.0;
        params.p599 = 0.0;
        params.p600 = 0.01;
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
        params.p612 = 1.0;
        params.p613 = 0.0;
        params.p614 = 0.0;
        params.p615 = 0.0;
        params.p616 = 0.0;
        params.p617 = 0.0;
        params.p618 = 0.0;
        params.p619 = 0.0;
        params.p620 = 1.0;
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
        params.p640 = 2.0;
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
        params.p660 = 1e-14;
        params.p661 = 0.0;
        params.p662 = 0.0;
        params.p663 = 0.0;
        params.p664 = 0.0;
        params.p665 = 0.0;
        params.p666 = 0.0;
        params.p667 = 0.0;
        params.p668 = 1.0;
        params.p669 = 0.0;
        params.p670 = 0.0;
        params.p671 = 0.0;
        params.p672 = 0.1;
        params.p673 = 0.0;
        params.p674 = 0.0;
        params.p675 = 0.0;
        params.p676 = 8.0;
        params.p677 = 0.0;
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
        params.p688 = 1e-15;
        params.p689 = 0.0;
        params.p690 = 0.0;
        params.p691 = 0.0;
        params.p692 = 1e-15;
        params.p693 = 0.0;
        params.p694 = 0.0;
        params.p695 = 0.0;
        params.p696 = 1e-15;
        params.p697 = 0.0;
        params.p698 = 0.0;
        params.p699 = 0.0;
        params.p700 = 5e-16;
        params.p701 = 0.0;
        params.p702 = 0.0;
        params.p703 = 0.0;
        params.p704 = 5e-16;
        params.p705 = 0.0;
        params.p706 = 0.0;
        params.p707 = 0.0;
        params.p708 = 1e-15;
        params.p709 = 0.0;
        params.p710 = 0.0;
        params.p711 = 0.0;
        params.p712 = 1e-15;
        params.p713 = 0.0;
        params.p714 = 0.0;
        params.p715 = 0.0;
        params.p716 = 0.0;
        params.p717 = 0.0;
        params.p718 = 0.0;
        params.p719 = 0.0;
        params.p720 = 8e22;
        params.p721 = 0.0;
        params.p722 = 0.0;
        params.p723 = 0.0;
        params.p724 = 30000000.0;
        params.p725 = 0.0;
        params.p726 = 0.0;
        params.p727 = 0.0;
        params.p728 = 0.0;
        params.p729 = 0.0;
        params.p730 = 0.0;
        params.p731 = 0.0;
        params.p732 = -1.0;
        params.p733 = 0.0;
        params.p734 = 0.0;
        params.p735 = 0.0;
        params.p736 = 0.0005;
        params.p737 = 0.0;
        params.p738 = 0.0;
        params.p739 = 0.0;
        params.p740 = 0.0;
        params.p741 = 0.0;
        params.p742 = 0.0;
        params.p743 = 0.0;
        params.p744 = 5e23;
        params.p745 = 0.0;
        params.p746 = 0.0;
        params.p747 = 0.0;
        params.p748 = 0.0;
        params.p749 = 0.0;
        params.p750 = 0.0;
        params.p751 = 0.0;
        params.p752 = 0.03;
        params.p753 = 0.0;
        params.p754 = 0.0;
        params.p755 = 0.0;
        params.p756 = 1.0;
        params.p757 = 0.0;
        params.p758 = 0.0;
        params.p759 = 0.0;
        params.p760 = 0.0;
        params.p761 = 0.0;
        params.p762 = 0.0;
        params.p763 = 0.0;
        params.p764 = 0.0;
        params.p765 = 0.0;
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
        params.p784 = 8e22;
        params.p785 = 0.0;
        params.p786 = 0.0;
        params.p787 = 0.0;
        params.p788 = 30000000.0;
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
        params.p808 = 1e-6;
        params.p809 = 1e-6;
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
        params.p824 = 0.0;
        params.p825 = 0.0;
        params.p826 = 0.0;
        params.p827 = 1.0;
        params.p828 = 1e-6;
        params.p829 = 0.0;
        params.p830 = 0.0;
        params.p831 = 0.0;
        params.p832 = 0.0;
        params.p833 = 0.0;
        params.p834 = 0.0;
        params.p835 = 0.0;
        params.p836 = 0.0;
        params.p837 = 0.0;
        params.p838 = 0.0;
        params.p839 = 1000.0;
        params.p840 = 21.0;
        params.p841 = 1000.0;
        params.p842 = 0.001;
        params.p843 = 1e-9;
        params.p844 = 1e-9;
        params.p845 = 1.0;
        params.p846 = 1.0;
        params.p847 = 1.0;
        params.p848 = 0.5;
        params.p849 = 0.5;
        params.p850 = 0.5;
        params.p851 = 1.16;
        params.p852 = 1.16;
        params.p853 = 1.16;
        params.p854 = 1e-12;
        params.p855 = 1e-18;
        params.p856 = 1e-18;
        params.p857 = 100.0;
        params.p858 = 0.0001;
        params.p859 = 0.0001;
        params.p860 = 1e-7;
        params.p861 = 1e-7;
        params.p862 = 100.0;
        params.p863 = 0.0001;
        params.p864 = 0.0001;
        params.p865 = 0.25;
        params.p866 = 0.25;
        params.p867 = 0.25;
        params.p868 = 1e-12;
        params.p869 = 1e-18;
        params.p870 = 1e-18;
        params.p871 = 1000000000.0;
        params.p872 = 1000000000.0;
        params.p873 = 1000000000.0;
        params.p874 = -0.001;
        params.p875 = -0.001;
        params.p876 = -0.001;
        params.p877 = 10.0;
        params.p878 = 10.0;
        params.p879 = 10.0;
        params.p880 = 4.0;
        params.p881 = 4.0;
        params.p882 = 4.0;
        params.p883 = 1.0;
        params.p884 = 1.0;
        params.p885 = 1.0;
        params.p886 = 1.0;
        params.p887 = -1.0;
        params.p888 = 0.1;
        params.p889 = 0.0;
        params.p890 = 0.5;
        params.p891 = 0.0;
        params.p892 = 0.5;
        params.p893 = 0.001;
        params.p894 = 1e-9;
        params.p895 = 1e-9;
        params.p896 = 1.0;
        params.p897 = 1.0;
        params.p898 = 1.0;
        params.p899 = 0.5;
        params.p900 = 0.5;
        params.p901 = 0.5;
        params.p902 = 1.16;
        params.p903 = 1.16;
        params.p904 = 1.16;
        params.p905 = 1e-12;
        params.p906 = 1e-18;
        params.p907 = 1e-18;
        params.p908 = 100.0;
        params.p909 = 0.0001;
        params.p910 = 0.0001;
        params.p911 = 1e-7;
        params.p912 = 1e-7;
        params.p913 = 100.0;
        params.p914 = 0.0001;
        params.p915 = 0.0001;
        params.p916 = 0.25;
        params.p917 = 0.25;
        params.p918 = 0.25;
        params.p919 = 1e-12;
        params.p920 = 1e-18;
        params.p921 = 1e-18;
        params.p922 = 1000000000.0;
        params.p923 = 1000000000.0;
        params.p924 = 1000000000.0;
        params.p925 = -0.001;
        params.p926 = -0.001;
        params.p927 = -0.001;
        params.p928 = 10.0;
        params.p929 = 10.0;
        params.p930 = 10.0;
        params.p931 = 4.0;
        params.p932 = 4.0;
        params.p933 = 4.0;
        params.p934 = 1.0;
        params.p935 = 1.0;
        params.p936 = 1.0;
        params.p937 = 1.0;
        params.p938 = -1.0;
        params.p939 = 0.1;
        params.p940 = 0.0;
        params.p941 = 0.5;
        params.p942 = 0.0;
        params.p943 = 0.5;
        params.p944 = 0.0;
        params.p945 = 2.5;
        params.p946 = 0.03;
        params.p947 = 2.5;
        params.p948 = 0.03;
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
    pub nodes: [usize; 13],
    pub branches: [usize; 7],
    pub params: Parameters,
    pub(crate) param_given: [bool; 949],
    pub(crate) multiplicity: f64,
    pub(crate) ddt_state_current: [f64; 12],
    pub(crate) ddt_state_previous: [f64; 12],
    pub(crate) ddt_state_initialized: [bool; 12],
    pub(crate) idt_state_current: [f64; 0],
    pub(crate) idt_state_previous: [f64; 0],
    pub(crate) idt_state_initialized: [bool; 0],
    pub(crate) time: f64,
    pub(crate) timestep: f64,
    pub(crate) scratch: Option<Box<GenericScratch<2932, 13, 7>>>,
    pub(crate) reactive_scratch: Option<Box<GenericReactiveScratch<2932, 13, 7>>>,
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
    pub const INTERNAL_NODE_COUNT: usize = 8;
    pub const NODE_COUNT: usize = 13;
    pub const INTERNAL_NODE_NAMES: [&str; 8] = ["noi", "gp", "si", "di", "bp", "bi", "bs", "bd"];

    pub const BRANCH_COUNT: usize = 7;
    pub const PARAMETER_COUNT: usize = 949;
    pub const VARIABLE_COUNT: usize = 2932;
    pub const DDT_STATE_COUNT: usize = 12;
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
            "l" => { validate_parameter("L", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p0 = value; self.mark_param_given(0); Ok(()) }
            "w" => { validate_parameter("W", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); Ok(()) }
            "sa" => { validate_finite_parameter("SA", value)?; self.params.p2 = value; self.mark_param_given(2); Ok(()) }
            "sb" => { validate_finite_parameter("SB", value)?; self.params.p3 = value; self.mark_param_given(3); Ok(()) }
            "sd" => { validate_finite_parameter("SD", value)?; self.params.p4 = value; self.mark_param_given(4); Ok(()) }
            "sca" => { validate_parameter("SCA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); Ok(()) }
            "scb" => { validate_parameter("SCB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); Ok(()) }
            "scc" => { validate_parameter("SCC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); Ok(()) }
            "sc" => { validate_finite_parameter("SC", value)?; self.params.p8 = value; self.mark_param_given(8); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p10 = value; self.mark_param_given(10); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p11 = value; self.mark_param_given(11); Ok(()) }
            "nrs" => { validate_finite_parameter("NRS", value)?; self.params.p12 = value; self.mark_param_given(12); Ok(()) }
            "nrd" => { validate_finite_parameter("NRD", value)?; self.params.p13 = value; self.mark_param_given(13); Ok(()) }
            "jw" => { validate_parameter("JW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); Ok(()) }
            "delvto" => { validate_finite_parameter("DELVTO", value)?; self.params.p15 = value; self.mark_param_given(15); Ok(()) }
            "factuo" => { validate_parameter("FACTUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); Ok(()) }
            "delvtoedge" => { validate_finite_parameter("DELVTOEDGE", value)?; self.params.p17 = value; self.mark_param_given(17); Ok(()) }
            "factuoedge" => { validate_parameter("FACTUOEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); Ok(()) }
            "absource" => { validate_parameter("ABSOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); Ok(()) }
            "lssource" => { validate_parameter("LSSOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p20 = value; self.mark_param_given(20); Ok(()) }
            "lgsource" => { validate_parameter("LGSOURCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p21 = value; self.mark_param_given(21); Ok(()) }
            "abdrain" => { validate_parameter("ABDRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); Ok(()) }
            "lsdrain" => { validate_parameter("LSDRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); Ok(()) }
            "lgdrain" => { validate_parameter("LGDRAIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); Ok(()) }
            "ifactor" => { validate_parameter("IFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); Ok(()) }
            "cfactor" => { validate_parameter("CFACTOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); Ok(()) }
            "mult" => { validate_parameter("MULT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); Ok(()) }
            "mult_fn" => { validate_parameter("MULT_FN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); Ok(()) }
            "trise" => { validate_finite_parameter("TRISE", value)?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "dtemp" => { validate_finite_parameter("TRISE", value)?; self.params.p35 = value; self.mark_param_given(35); Ok(()) }
            "level" => { validate_finite_parameter("LEVEL", value)?; self.params.p36 = value; self.mark_param_given(36); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p37 = value; self.mark_param_given(37); Ok(()) }
            "tr" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "tref" => { validate_parameter("TR", value, Some((-273.0, "-273.0")), false, None, true, &[])?; self.params.p38 = value; self.mark_param_given(38); Ok(()) }
            "swgeo" => { validate_parameter("SWGEO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); Ok(()) }
            "swigate" => { validate_parameter("SWIGATE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); Ok(()) }
            "swimpact" => { validate_parameter("SWIMPACT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); Ok(()) }
            "swgidl" => { validate_parameter("SWGIDL", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); Ok(()) }
            "swjuncap" => { validate_parameter("SWJUNCAP", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); Ok(()) }
            "swjunasym" => { validate_parameter("SWJUNASYM", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); Ok(()) }
            "swnud" => { validate_parameter("SWNUD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p45 = value; self.mark_param_given(45); Ok(()) }
            "swedge" => { validate_parameter("SWEDGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p46 = value; self.mark_param_given(46); Ok(()) }
            "swdelvtac" => { validate_parameter("SWDELVTAC", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p47 = value; self.mark_param_given(47); Ok(()) }
            "swqsat" => { validate_parameter("SWQSAT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p48 = value; self.mark_param_given(48); Ok(()) }
            "swqpart" => { validate_parameter("SWQPART", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p49 = value; self.mark_param_given(49); Ok(()) }
            "swign" => { validate_parameter("SWIGN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p50 = value; self.mark_param_given(50); Ok(()) }
            "qmc" => { validate_parameter("QMC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); Ok(()) }
            "swoprext" => { validate_parameter("SWOPREXT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p52 = value; self.mark_param_given(52); Ok(()) }
            "swoppmos" => { validate_parameter("SWOPPMOS", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p53 = value; self.mark_param_given(53); Ok(()) }
            "swopdrain" => { validate_parameter("SWOPDRAIN", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p54 = value; self.mark_param_given(54); Ok(()) }
            "dta" => { validate_finite_parameter("DTA", value)?; self.params.p55 = value; self.mark_param_given(55); Ok(()) }
            "vfb" => { validate_finite_parameter("VFB", value)?; self.params.p56 = value; self.mark_param_given(56); Ok(()) }
            "stvfb" => { validate_finite_parameter("STVFB", value)?; self.params.p57 = value; self.mark_param_given(57); Ok(()) }
            "st2vfb" => { validate_finite_parameter("ST2VFB", value)?; self.params.p58 = value; self.mark_param_given(58); Ok(()) }
            "tox" => { validate_parameter("TOX", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); Ok(()) }
            "epsrox" => { validate_parameter("EPSROX", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); Ok(()) }
            "neff" => { validate_parameter("NEFF", value, Some((1e20, "1e20")), false, Some((1e26, "1e26")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); Ok(()) }
            "gfacnud" => { validate_parameter("GFACNUD", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p62 = value; self.mark_param_given(62); Ok(()) }
            "vsbnud" => { validate_parameter("VSBNUD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p63 = value; self.mark_param_given(63); Ok(()) }
            "dvsbnud" => { validate_parameter("DVSBNUD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p64 = value; self.mark_param_given(64); Ok(()) }
            "dphib" => { validate_finite_parameter("DPHIB", value)?; self.params.p65 = value; self.mark_param_given(65); Ok(()) }
            "np" => { validate_parameter("NP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); Ok(()) }
            "toxov" => { validate_parameter("TOXOV", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); Ok(()) }
            "toxovd" => { validate_parameter("TOXOVD", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p68 = value; self.mark_param_given(68); Ok(()) }
            "nov" => { validate_parameter("NOV", value, Some((1e23, "1e23")), false, Some((1e27, "1e27")), false, &[])?; self.params.p69 = value; self.mark_param_given(69); Ok(()) }
            "novd" => { validate_parameter("NOVD", value, Some((1e23, "1e23")), false, Some((1e27, "1e27")), false, &[])?; self.params.p70 = value; self.mark_param_given(70); Ok(()) }
            "ct" => { validate_parameter("CT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p71 = value; self.mark_param_given(71); Ok(()) }
            "ctb" => { validate_parameter("CTB", value, Some((0.0, "0.0")), false, Some((0.5, "0.5")), false, &[])?; self.params.p72 = value; self.mark_param_given(72); Ok(()) }
            "ctg" => { validate_parameter("CTG", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p73 = value; self.mark_param_given(73); Ok(()) }
            "stct" => { validate_finite_parameter("STCT", value)?; self.params.p74 = value; self.mark_param_given(74); Ok(()) }
            "cf" => { validate_parameter("CF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p75 = value; self.mark_param_given(75); Ok(()) }
            "cfb" => { validate_parameter("CFB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p76 = value; self.mark_param_given(76); Ok(()) }
            "cfd" => { validate_parameter("CFD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p77 = value; self.mark_param_given(77); Ok(()) }
            "psce" => { validate_parameter("PSCE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p78 = value; self.mark_param_given(78); Ok(()) }
            "psceb" => { validate_parameter("PSCEB", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p79 = value; self.mark_param_given(79); Ok(()) }
            "psced" => { validate_parameter("PSCED", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p80 = value; self.mark_param_given(80); Ok(()) }
            "betn" => { validate_parameter("BETN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); Ok(()) }
            "stbet" => { validate_finite_parameter("STBET", value)?; self.params.p82 = value; self.mark_param_given(82); Ok(()) }
            "mue" => { validate_parameter("MUE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); Ok(()) }
            "stmue" => { validate_finite_parameter("STMUE", value)?; self.params.p84 = value; self.mark_param_given(84); Ok(()) }
            "themu" => { validate_parameter("THEMU", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p85 = value; self.mark_param_given(85); Ok(()) }
            "stthemu" => { validate_finite_parameter("STTHEMU", value)?; self.params.p86 = value; self.mark_param_given(86); Ok(()) }
            "cs" => { validate_parameter("CS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p87 = value; self.mark_param_given(87); Ok(()) }
            "stcs" => { validate_finite_parameter("STCS", value)?; self.params.p88 = value; self.mark_param_given(88); Ok(()) }
            "thecs" => { validate_parameter("THECS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); Ok(()) }
            "stthecs" => { validate_finite_parameter("STTHECS", value)?; self.params.p90 = value; self.mark_param_given(90); Ok(()) }
            "xcor" => { validate_parameter("XCOR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p91 = value; self.mark_param_given(91); Ok(()) }
            "stxcor" => { validate_finite_parameter("STXCOR", value)?; self.params.p92 = value; self.mark_param_given(92); Ok(()) }
            "feta" => { validate_parameter("FETA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p93 = value; self.mark_param_given(93); Ok(()) }
            "rs" => { validate_parameter("RS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p94 = value; self.mark_param_given(94); Ok(()) }
            "strs" => { validate_finite_parameter("STRS", value)?; self.params.p95 = value; self.mark_param_given(95); Ok(()) }
            "rsb" => { validate_parameter("RSB", value, Some((-0.5, "-0.5")), false, Some((1.0, "1.0")), false, &[])?; self.params.p96 = value; self.mark_param_given(96); Ok(()) }
            "rsg" => { validate_parameter("RSG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p97 = value; self.mark_param_given(97); Ok(()) }
            "thesat" => { validate_parameter("THESAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p98 = value; self.mark_param_given(98); Ok(()) }
            "stthesat" => { validate_finite_parameter("STTHESAT", value)?; self.params.p99 = value; self.mark_param_given(99); Ok(()) }
            "thesatb" => { validate_parameter("THESATB", value, Some((-0.5, "-0.5")), false, Some((1.0, "1.0")), false, &[])?; self.params.p100 = value; self.mark_param_given(100); Ok(()) }
            "thesatg" => { validate_parameter("THESATG", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p101 = value; self.mark_param_given(101); Ok(()) }
            "thesatt" => { validate_parameter("THESATT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p102 = value; self.mark_param_given(102); Ok(()) }
            "ax" => { validate_parameter("AX", value, Some((2.0, "2.0")), false, None, true, &[])?; self.params.p103 = value; self.mark_param_given(103); Ok(()) }
            "alp" => { validate_parameter("ALP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p104 = value; self.mark_param_given(104); Ok(()) }
            "alp1" => { validate_parameter("ALP1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p105 = value; self.mark_param_given(105); Ok(()) }
            "alp2" => { validate_parameter("ALP2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p106 = value; self.mark_param_given(106); Ok(()) }
            "vp" => { validate_parameter("VP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p107 = value; self.mark_param_given(107); Ok(()) }
            "a1" => { validate_parameter("A1", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p108 = value; self.mark_param_given(108); Ok(()) }
            "a2" => { validate_parameter("A2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p109 = value; self.mark_param_given(109); Ok(()) }
            "sta2" => { validate_finite_parameter("STA2", value)?; self.params.p110 = value; self.mark_param_given(110); Ok(()) }
            "a3" => { validate_parameter("A3", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p111 = value; self.mark_param_given(111); Ok(()) }
            "a4" => { validate_parameter("A4", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p112 = value; self.mark_param_given(112); Ok(()) }
            "imaxii" => { validate_parameter("IMAXII", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p113 = value; self.mark_param_given(113); Ok(()) }
            "gco" => { validate_parameter("GCO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p114 = value; self.mark_param_given(114); Ok(()) }
            "iginv" => { validate_parameter("IGINV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p115 = value; self.mark_param_given(115); Ok(()) }
            "igov" => { validate_parameter("IGOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p116 = value; self.mark_param_given(116); Ok(()) }
            "igovd" => { validate_parameter("IGOVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p117 = value; self.mark_param_given(117); Ok(()) }
            "stig" => { validate_finite_parameter("STIG", value)?; self.params.p118 = value; self.mark_param_given(118); Ok(()) }
            "gc2" => { validate_parameter("GC2", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p119 = value; self.mark_param_given(119); Ok(()) }
            "gc3" => { validate_parameter("GC3", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p120 = value; self.mark_param_given(120); Ok(()) }
            "gc2ov" => { validate_parameter("GC2OV", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p121 = value; self.mark_param_given(121); Ok(()) }
            "gc3ov" => { validate_parameter("GC3OV", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p122 = value; self.mark_param_given(122); Ok(()) }
            "gc2ovd" => { validate_parameter("GC2OVD", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p123 = value; self.mark_param_given(123); Ok(()) }
            "gc3ovd" => { validate_parameter("GC3OVD", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p124 = value; self.mark_param_given(124); Ok(()) }
            "chib" => { validate_parameter("CHIB", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p125 = value; self.mark_param_given(125); Ok(()) }
            "agidl" => { validate_parameter("AGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p126 = value; self.mark_param_given(126); Ok(()) }
            "agidld" => { validate_parameter("AGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); Ok(()) }
            "bgidl" => { validate_parameter("BGIDL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); Ok(()) }
            "bgidld" => { validate_parameter("BGIDLD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p129 = value; self.mark_param_given(129); Ok(()) }
            "stbgidl" => { validate_finite_parameter("STBGIDL", value)?; self.params.p130 = value; self.mark_param_given(130); Ok(()) }
            "stbgidld" => { validate_finite_parameter("STBGIDLD", value)?; self.params.p131 = value; self.mark_param_given(131); Ok(()) }
            "cgidl" => { validate_finite_parameter("CGIDL", value)?; self.params.p132 = value; self.mark_param_given(132); Ok(()) }
            "cgidld" => { validate_finite_parameter("CGIDLD", value)?; self.params.p133 = value; self.mark_param_given(133); Ok(()) }
            "cox" => { validate_parameter("COX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); Ok(()) }
            "delvtac" => { validate_finite_parameter("DELVTAC", value)?; self.params.p135 = value; self.mark_param_given(135); Ok(()) }
            "facneffac" => { validate_parameter("FACNEFFAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); Ok(()) }
            "thesatac" => { validate_parameter("THESATAC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p137 = value; self.mark_param_given(137); Ok(()) }
            "axac" => { validate_parameter("AXAC", value, Some((2.0, "2.0")), false, None, true, &[])?; self.params.p138 = value; self.mark_param_given(138); Ok(()) }
            "alpac" => { validate_finite_parameter("ALPAC", value)?; self.params.p139 = value; self.mark_param_given(139); Ok(()) }
            "alp1ac" => { validate_parameter("ALP1AC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p140 = value; self.mark_param_given(140); Ok(()) }
            "cgov" => { validate_parameter("CGOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p141 = value; self.mark_param_given(141); Ok(()) }
            "cgovd" => { validate_parameter("CGOVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p142 = value; self.mark_param_given(142); Ok(()) }
            "fcgovacc" => { validate_parameter("FCGOVACC", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p143 = value; self.mark_param_given(143); Ok(()) }
            "fcgovaccd" => { validate_parameter("FCGOVACCD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p144 = value; self.mark_param_given(144); Ok(()) }
            "cgovaccg" => { validate_parameter("CGOVACCG", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p145 = value; self.mark_param_given(145); Ok(()) }
            "cgbov" => { validate_parameter("CGBOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p146 = value; self.mark_param_given(146); Ok(()) }
            "cinr" => { validate_parameter("CINR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p147 = value; self.mark_param_given(147); Ok(()) }
            "cinrd" => { validate_parameter("CINRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p148 = value; self.mark_param_given(148); Ok(()) }
            "dvfbinr" => { validate_finite_parameter("DVFBINR", value)?; self.params.p149 = value; self.mark_param_given(149); Ok(()) }
            "fcinrdep" => { validate_parameter("FCINRDEP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p150 = value; self.mark_param_given(150); Ok(()) }
            "fcinracc" => { validate_parameter("FCINRACC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p151 = value; self.mark_param_given(151); Ok(()) }
            "axinr" => { validate_parameter("AXINR", value, Some((0.1, "0.1")), false, Some((4.0, "4.0")), false, &[])?; self.params.p152 = value; self.mark_param_given(152); Ok(()) }
            "cfr" => { validate_parameter("CFR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p153 = value; self.mark_param_given(153); Ok(()) }
            "cfrd" => { validate_parameter("CFRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); Ok(()) }
            "fnt" => { validate_parameter("FNT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p155 = value; self.mark_param_given(155); Ok(()) }
            "fntexc" => { validate_parameter("FNTEXC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p156 = value; self.mark_param_given(156); Ok(()) }
            "nfa" => { validate_parameter("NFA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p157 = value; self.mark_param_given(157); Ok(()) }
            "nfb" => { validate_parameter("NFB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p158 = value; self.mark_param_given(158); Ok(()) }
            "nfc" => { validate_parameter("NFC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p159 = value; self.mark_param_given(159); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p160 = value; self.mark_param_given(160); Ok(()) }
            "vfbedge" => { validate_finite_parameter("VFBEDGE", value)?; self.params.p161 = value; self.mark_param_given(161); Ok(()) }
            "stvfbedge" => { validate_finite_parameter("STVFBEDGE", value)?; self.params.p162 = value; self.mark_param_given(162); Ok(()) }
            "dphibedge" => { validate_finite_parameter("DPHIBEDGE", value)?; self.params.p163 = value; self.mark_param_given(163); Ok(()) }
            "neffedge" => { validate_parameter("NEFFEDGE", value, Some((1e20, "1e20")), false, Some((1e26, "1e26")), false, &[])?; self.params.p164 = value; self.mark_param_given(164); Ok(()) }
            "ctedge" => { validate_parameter("CTEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p165 = value; self.mark_param_given(165); Ok(()) }
            "betnedge" => { validate_parameter("BETNEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p166 = value; self.mark_param_given(166); Ok(()) }
            "stbetedge" => { validate_finite_parameter("STBETEDGE", value)?; self.params.p167 = value; self.mark_param_given(167); Ok(()) }
            "psceedge" => { validate_parameter("PSCEEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p168 = value; self.mark_param_given(168); Ok(()) }
            "pscebedge" => { validate_parameter("PSCEBEDGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p169 = value; self.mark_param_given(169); Ok(()) }
            "pscededge" => { validate_parameter("PSCEDEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p170 = value; self.mark_param_given(170); Ok(()) }
            "cfedge" => { validate_parameter("CFEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p171 = value; self.mark_param_given(171); Ok(()) }
            "cfbedge" => { validate_parameter("CFBEDGE", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p172 = value; self.mark_param_given(172); Ok(()) }
            "cfdedge" => { validate_parameter("CFDEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); Ok(()) }
            "fntedge" => { validate_parameter("FNTEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p174 = value; self.mark_param_given(174); Ok(()) }
            "nfaedge" => { validate_parameter("NFAEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p175 = value; self.mark_param_given(175); Ok(()) }
            "nfbedge" => { validate_parameter("NFBEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p176 = value; self.mark_param_given(176); Ok(()) }
            "nfcedge" => { validate_parameter("NFCEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p177 = value; self.mark_param_given(177); Ok(()) }
            "efedge" => { validate_parameter("EFEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p178 = value; self.mark_param_given(178); Ok(()) }
            "rg" => { validate_parameter("RG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p179 = value; self.mark_param_given(179); Ok(()) }
            "rse" => { validate_parameter("RSE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p180 = value; self.mark_param_given(180); Ok(()) }
            "rde" => { validate_parameter("RDE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); Ok(()) }
            "rbulk" => { validate_parameter("RBULK", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); Ok(()) }
            "rwell" => { validate_parameter("RWELL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p183 = value; self.mark_param_given(183); Ok(()) }
            "rjuns" => { validate_parameter("RJUNS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p184 = value; self.mark_param_given(184); Ok(()) }
            "rjund" => { validate_parameter("RJUND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); Ok(()) }
            "rth" => { validate_parameter("RTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); Ok(()) }
            "cth" => { validate_parameter("CTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p187 = value; self.mark_param_given(187); Ok(()) }
            "strth" => { validate_finite_parameter("STRTH", value)?; self.params.p188 = value; self.mark_param_given(188); Ok(()) }
            "lvaro" => { validate_finite_parameter("LVARO", value)?; self.params.p189 = value; self.mark_param_given(189); Ok(()) }
            "lvarl" => { validate_finite_parameter("LVARL", value)?; self.params.p190 = value; self.mark_param_given(190); Ok(()) }
            "lvarw" => { validate_finite_parameter("LVARW", value)?; self.params.p191 = value; self.mark_param_given(191); Ok(()) }
            "lap" => { validate_finite_parameter("LAP", value)?; self.params.p192 = value; self.mark_param_given(192); Ok(()) }
            "wvaro" => { validate_finite_parameter("WVARO", value)?; self.params.p193 = value; self.mark_param_given(193); Ok(()) }
            "wvarl" => { validate_finite_parameter("WVARL", value)?; self.params.p194 = value; self.mark_param_given(194); Ok(()) }
            "wvarw" => { validate_finite_parameter("WVARW", value)?; self.params.p195 = value; self.mark_param_given(195); Ok(()) }
            "wot" => { validate_finite_parameter("WOT", value)?; self.params.p196 = value; self.mark_param_given(196); Ok(()) }
            "dlq" => { validate_finite_parameter("DLQ", value)?; self.params.p197 = value; self.mark_param_given(197); Ok(()) }
            "dwq" => { validate_finite_parameter("DWQ", value)?; self.params.p198 = value; self.mark_param_given(198); Ok(()) }
            "vfbo" => { validate_finite_parameter("VFBO", value)?; self.params.p199 = value; self.mark_param_given(199); Ok(()) }
            "vfbl" => { validate_finite_parameter("VFBL", value)?; self.params.p200 = value; self.mark_param_given(200); Ok(()) }
            "vfblexp" => { validate_finite_parameter("VFBLEXP", value)?; self.params.p201 = value; self.mark_param_given(201); Ok(()) }
            "vfbw" => { validate_finite_parameter("VFBW", value)?; self.params.p202 = value; self.mark_param_given(202); Ok(()) }
            "vfblw" => { validate_finite_parameter("VFBLW", value)?; self.params.p203 = value; self.mark_param_given(203); Ok(()) }
            "stvfbo" => { validate_finite_parameter("STVFBO", value)?; self.params.p204 = value; self.mark_param_given(204); Ok(()) }
            "stvfbl" => { validate_finite_parameter("STVFBL", value)?; self.params.p205 = value; self.mark_param_given(205); Ok(()) }
            "stvfbw" => { validate_finite_parameter("STVFBW", value)?; self.params.p206 = value; self.mark_param_given(206); Ok(()) }
            "stvfblw" => { validate_finite_parameter("STVFBLW", value)?; self.params.p207 = value; self.mark_param_given(207); Ok(()) }
            "st2vfbo" => { validate_finite_parameter("ST2VFBO", value)?; self.params.p208 = value; self.mark_param_given(208); Ok(()) }
            "toxo" => { validate_parameter("TOXO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p209 = value; self.mark_param_given(209); Ok(()) }
            "epsroxo" => { validate_parameter("EPSROXO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p210 = value; self.mark_param_given(210); Ok(()) }
            "nsubo" => { validate_parameter("NSUBO", value, Some((1e20, "1e20")), false, None, true, &[])?; self.params.p211 = value; self.mark_param_given(211); Ok(()) }
            "nsubw" => { validate_finite_parameter("NSUBW", value)?; self.params.p212 = value; self.mark_param_given(212); Ok(()) }
            "wseg" => { validate_parameter("WSEG", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); Ok(()) }
            "npck" => { validate_parameter("NPCK", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); Ok(()) }
            "npckw" => { validate_finite_parameter("NPCKW", value)?; self.params.p215 = value; self.mark_param_given(215); Ok(()) }
            "wsegp" => { validate_parameter("WSEGP", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p216 = value; self.mark_param_given(216); Ok(()) }
            "lpck" => { validate_parameter("LPCK", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p217 = value; self.mark_param_given(217); Ok(()) }
            "lpckw" => { validate_finite_parameter("LPCKW", value)?; self.params.p218 = value; self.mark_param_given(218); Ok(()) }
            "fol1" => { validate_finite_parameter("FOL1", value)?; self.params.p219 = value; self.mark_param_given(219); Ok(()) }
            "fol2" => { validate_finite_parameter("FOL2", value)?; self.params.p220 = value; self.mark_param_given(220); Ok(()) }
            "gfacnudo" => { validate_finite_parameter("GFACNUDO", value)?; self.params.p221 = value; self.mark_param_given(221); Ok(()) }
            "gfacnudl" => { validate_finite_parameter("GFACNUDL", value)?; self.params.p222 = value; self.mark_param_given(222); Ok(()) }
            "gfacnudlexp" => { validate_finite_parameter("GFACNUDLEXP", value)?; self.params.p223 = value; self.mark_param_given(223); Ok(()) }
            "gfacnudw" => { validate_finite_parameter("GFACNUDW", value)?; self.params.p224 = value; self.mark_param_given(224); Ok(()) }
            "gfacnudlw" => { validate_finite_parameter("GFACNUDLW", value)?; self.params.p225 = value; self.mark_param_given(225); Ok(()) }
            "vsbnudo" => { validate_parameter("VSBNUDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); Ok(()) }
            "dvsbnudo" => { validate_parameter("DVSBNUDO", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p227 = value; self.mark_param_given(227); Ok(()) }
            "dphibo" => { validate_finite_parameter("DPHIBO", value)?; self.params.p228 = value; self.mark_param_given(228); Ok(()) }
            "dphibl" => { validate_finite_parameter("DPHIBL", value)?; self.params.p229 = value; self.mark_param_given(229); Ok(()) }
            "dphiblexp" => { validate_finite_parameter("DPHIBLEXP", value)?; self.params.p230 = value; self.mark_param_given(230); Ok(()) }
            "dphibw" => { validate_finite_parameter("DPHIBW", value)?; self.params.p231 = value; self.mark_param_given(231); Ok(()) }
            "dphiblw" => { validate_finite_parameter("DPHIBLW", value)?; self.params.p232 = value; self.mark_param_given(232); Ok(()) }
            "npo" => { validate_finite_parameter("NPO", value)?; self.params.p233 = value; self.mark_param_given(233); Ok(()) }
            "npl" => { validate_finite_parameter("NPL", value)?; self.params.p234 = value; self.mark_param_given(234); Ok(()) }
            "toxovo" => { validate_parameter("TOXOVO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p235 = value; self.mark_param_given(235); Ok(()) }
            "toxovdo" => { validate_parameter("TOXOVDO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p236 = value; self.mark_param_given(236); Ok(()) }
            "lov" => { validate_parameter("LOV", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p237 = value; self.mark_param_given(237); Ok(()) }
            "lovd" => { validate_parameter("LOVD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p238 = value; self.mark_param_given(238); Ok(()) }
            "novo" => { validate_parameter("NOVO", value, Some((1e23, "1e23")), false, Some((1e27, "1e27")), false, &[])?; self.params.p239 = value; self.mark_param_given(239); Ok(()) }
            "novdo" => { validate_parameter("NOVDO", value, Some((1e23, "1e23")), false, Some((1e27, "1e27")), false, &[])?; self.params.p240 = value; self.mark_param_given(240); Ok(()) }
            "cto" => { validate_finite_parameter("CTO", value)?; self.params.p241 = value; self.mark_param_given(241); Ok(()) }
            "ctl" => { validate_finite_parameter("CTL", value)?; self.params.p242 = value; self.mark_param_given(242); Ok(()) }
            "ctlexp" => { validate_finite_parameter("CTLEXP", value)?; self.params.p243 = value; self.mark_param_given(243); Ok(()) }
            "ctw" => { validate_finite_parameter("CTW", value)?; self.params.p244 = value; self.mark_param_given(244); Ok(()) }
            "ctlw" => { validate_finite_parameter("CTLW", value)?; self.params.p245 = value; self.mark_param_given(245); Ok(()) }
            "ctbo" => { validate_parameter("CTBO", value, Some((0.0, "0.0")), false, Some((0.5, "0.5")), false, &[])?; self.params.p246 = value; self.mark_param_given(246); Ok(()) }
            "ctgo" => { validate_parameter("CTGO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p247 = value; self.mark_param_given(247); Ok(()) }
            "stcto" => { validate_finite_parameter("STCTO", value)?; self.params.p248 = value; self.mark_param_given(248); Ok(()) }
            "cfl" => { validate_finite_parameter("CFL", value)?; self.params.p249 = value; self.mark_param_given(249); Ok(()) }
            "cflexp" => { validate_finite_parameter("CFLEXP", value)?; self.params.p250 = value; self.mark_param_given(250); Ok(()) }
            "cfw" => { validate_finite_parameter("CFW", value)?; self.params.p251 = value; self.mark_param_given(251); Ok(()) }
            "cfbo" => { validate_parameter("CFBO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p252 = value; self.mark_param_given(252); Ok(()) }
            "cfdo" => { validate_parameter("CFDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p253 = value; self.mark_param_given(253); Ok(()) }
            "pscel" => { validate_finite_parameter("PSCEL", value)?; self.params.p254 = value; self.mark_param_given(254); Ok(()) }
            "pscelexp" => { validate_finite_parameter("PSCELEXP", value)?; self.params.p255 = value; self.mark_param_given(255); Ok(()) }
            "pscew" => { validate_finite_parameter("PSCEW", value)?; self.params.p256 = value; self.mark_param_given(256); Ok(()) }
            "pscebo" => { validate_parameter("PSCEBO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p257 = value; self.mark_param_given(257); Ok(()) }
            "pscedo" => { validate_parameter("PSCEDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p258 = value; self.mark_param_given(258); Ok(()) }
            "uo" => { validate_parameter("UO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p259 = value; self.mark_param_given(259); Ok(()) }
            "fbet1" => { validate_finite_parameter("FBET1", value)?; self.params.p260 = value; self.mark_param_given(260); Ok(()) }
            "fbet1w" => { validate_finite_parameter("FBET1W", value)?; self.params.p261 = value; self.mark_param_given(261); Ok(()) }
            "lp1" => { validate_parameter("LP1", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p262 = value; self.mark_param_given(262); Ok(()) }
            "lp1w" => { validate_finite_parameter("LP1W", value)?; self.params.p263 = value; self.mark_param_given(263); Ok(()) }
            "fbet2" => { validate_finite_parameter("FBET2", value)?; self.params.p264 = value; self.mark_param_given(264); Ok(()) }
            "lp2" => { validate_parameter("LP2", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p265 = value; self.mark_param_given(265); Ok(()) }
            "betw1" => { validate_finite_parameter("BETW1", value)?; self.params.p266 = value; self.mark_param_given(266); Ok(()) }
            "betw2" => { validate_finite_parameter("BETW2", value)?; self.params.p267 = value; self.mark_param_given(267); Ok(()) }
            "wbet" => { validate_parameter("WBET", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p268 = value; self.mark_param_given(268); Ok(()) }
            "stbeto" => { validate_finite_parameter("STBETO", value)?; self.params.p269 = value; self.mark_param_given(269); Ok(()) }
            "stbetl" => { validate_finite_parameter("STBETL", value)?; self.params.p270 = value; self.mark_param_given(270); Ok(()) }
            "stbetw" => { validate_finite_parameter("STBETW", value)?; self.params.p271 = value; self.mark_param_given(271); Ok(()) }
            "stbetlw" => { validate_finite_parameter("STBETLW", value)?; self.params.p272 = value; self.mark_param_given(272); Ok(()) }
            "mueo" => { validate_finite_parameter("MUEO", value)?; self.params.p273 = value; self.mark_param_given(273); Ok(()) }
            "muew" => { validate_finite_parameter("MUEW", value)?; self.params.p274 = value; self.mark_param_given(274); Ok(()) }
            "stmueo" => { validate_finite_parameter("STMUEO", value)?; self.params.p275 = value; self.mark_param_given(275); Ok(()) }
            "themuo" => { validate_parameter("THEMUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p276 = value; self.mark_param_given(276); Ok(()) }
            "stthemuo" => { validate_finite_parameter("STTHEMUO", value)?; self.params.p277 = value; self.mark_param_given(277); Ok(()) }
            "cso" => { validate_finite_parameter("CSO", value)?; self.params.p278 = value; self.mark_param_given(278); Ok(()) }
            "csl" => { validate_finite_parameter("CSL", value)?; self.params.p279 = value; self.mark_param_given(279); Ok(()) }
            "cslexp" => { validate_finite_parameter("CSLEXP", value)?; self.params.p280 = value; self.mark_param_given(280); Ok(()) }
            "csw" => { validate_finite_parameter("CSW", value)?; self.params.p281 = value; self.mark_param_given(281); Ok(()) }
            "cslw" => { validate_finite_parameter("CSLW", value)?; self.params.p282 = value; self.mark_param_given(282); Ok(()) }
            "stcso" => { validate_finite_parameter("STCSO", value)?; self.params.p283 = value; self.mark_param_given(283); Ok(()) }
            "thecso" => { validate_parameter("THECSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p284 = value; self.mark_param_given(284); Ok(()) }
            "stthecso" => { validate_finite_parameter("STTHECSO", value)?; self.params.p285 = value; self.mark_param_given(285); Ok(()) }
            "xcoro" => { validate_finite_parameter("XCORO", value)?; self.params.p286 = value; self.mark_param_given(286); Ok(()) }
            "xcorl" => { validate_finite_parameter("XCORL", value)?; self.params.p287 = value; self.mark_param_given(287); Ok(()) }
            "xcorw" => { validate_finite_parameter("XCORW", value)?; self.params.p288 = value; self.mark_param_given(288); Ok(()) }
            "xcorlw" => { validate_finite_parameter("XCORLW", value)?; self.params.p289 = value; self.mark_param_given(289); Ok(()) }
            "stxcoro" => { validate_finite_parameter("STXCORO", value)?; self.params.p290 = value; self.mark_param_given(290); Ok(()) }
            "fetao" => { validate_parameter("FETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p291 = value; self.mark_param_given(291); Ok(()) }
            "rsw1" => { validate_finite_parameter("RSW1", value)?; self.params.p292 = value; self.mark_param_given(292); Ok(()) }
            "rsw2" => { validate_finite_parameter("RSW2", value)?; self.params.p293 = value; self.mark_param_given(293); Ok(()) }
            "strso" => { validate_finite_parameter("STRSO", value)?; self.params.p294 = value; self.mark_param_given(294); Ok(()) }
            "rsbo" => { validate_parameter("RSBO", value, Some((-0.5, "-0.5")), false, Some((1.0, "1.0")), false, &[])?; self.params.p295 = value; self.mark_param_given(295); Ok(()) }
            "rsgo" => { validate_parameter("RSGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p296 = value; self.mark_param_given(296); Ok(()) }
            "thesato" => { validate_finite_parameter("THESATO", value)?; self.params.p297 = value; self.mark_param_given(297); Ok(()) }
            "thesatl" => { validate_finite_parameter("THESATL", value)?; self.params.p298 = value; self.mark_param_given(298); Ok(()) }
            "thesatlexp" => { validate_finite_parameter("THESATLEXP", value)?; self.params.p299 = value; self.mark_param_given(299); Ok(()) }
            "thesatw" => { validate_finite_parameter("THESATW", value)?; self.params.p300 = value; self.mark_param_given(300); Ok(()) }
            "thesatlw" => { validate_finite_parameter("THESATLW", value)?; self.params.p301 = value; self.mark_param_given(301); Ok(()) }
            "stthesato" => { validate_finite_parameter("STTHESATO", value)?; self.params.p302 = value; self.mark_param_given(302); Ok(()) }
            "stthesatl" => { validate_finite_parameter("STTHESATL", value)?; self.params.p303 = value; self.mark_param_given(303); Ok(()) }
            "stthesatw" => { validate_finite_parameter("STTHESATW", value)?; self.params.p304 = value; self.mark_param_given(304); Ok(()) }
            "stthesatlw" => { validate_finite_parameter("STTHESATLW", value)?; self.params.p305 = value; self.mark_param_given(305); Ok(()) }
            "thesatbo" => { validate_parameter("THESATBO", value, Some((-0.5, "-0.5")), false, Some((1.0, "1.0")), false, &[])?; self.params.p306 = value; self.mark_param_given(306); Ok(()) }
            "thesatgo" => { validate_parameter("THESATGO", value, Some((-0.5, "-0.5")), false, None, true, &[])?; self.params.p307 = value; self.mark_param_given(307); Ok(()) }
            "thesatto" => { validate_parameter("THESATTO", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p308 = value; self.mark_param_given(308); Ok(()) }
            "axo" => { validate_finite_parameter("AXO", value)?; self.params.p309 = value; self.mark_param_given(309); Ok(()) }
            "axl" => { validate_parameter("AXL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p310 = value; self.mark_param_given(310); Ok(()) }
            "alpl" => { validate_finite_parameter("ALPL", value)?; self.params.p311 = value; self.mark_param_given(311); Ok(()) }
            "alplexp" => { validate_finite_parameter("ALPLEXP", value)?; self.params.p312 = value; self.mark_param_given(312); Ok(()) }
            "alpw" => { validate_finite_parameter("ALPW", value)?; self.params.p313 = value; self.mark_param_given(313); Ok(()) }
            "alp1l1" => { validate_finite_parameter("ALP1L1", value)?; self.params.p314 = value; self.mark_param_given(314); Ok(()) }
            "alp1lexp" => { validate_finite_parameter("ALP1LEXP", value)?; self.params.p315 = value; self.mark_param_given(315); Ok(()) }
            "alp1l2" => { validate_parameter("ALP1L2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p316 = value; self.mark_param_given(316); Ok(()) }
            "alp1w" => { validate_finite_parameter("ALP1W", value)?; self.params.p317 = value; self.mark_param_given(317); Ok(()) }
            "alp2l1" => { validate_finite_parameter("ALP2L1", value)?; self.params.p318 = value; self.mark_param_given(318); Ok(()) }
            "alp2lexp" => { validate_finite_parameter("ALP2LEXP", value)?; self.params.p319 = value; self.mark_param_given(319); Ok(()) }
            "alp2l2" => { validate_parameter("ALP2L2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p320 = value; self.mark_param_given(320); Ok(()) }
            "alp2w" => { validate_finite_parameter("ALP2W", value)?; self.params.p321 = value; self.mark_param_given(321); Ok(()) }
            "vpo" => { validate_parameter("VPO", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p322 = value; self.mark_param_given(322); Ok(()) }
            "a1o" => { validate_finite_parameter("A1O", value)?; self.params.p323 = value; self.mark_param_given(323); Ok(()) }
            "a1l" => { validate_finite_parameter("A1L", value)?; self.params.p324 = value; self.mark_param_given(324); Ok(()) }
            "a1w" => { validate_finite_parameter("A1W", value)?; self.params.p325 = value; self.mark_param_given(325); Ok(()) }
            "a2o" => { validate_parameter("A2O", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p326 = value; self.mark_param_given(326); Ok(()) }
            "sta2o" => { validate_finite_parameter("STA2O", value)?; self.params.p327 = value; self.mark_param_given(327); Ok(()) }
            "a3o" => { validate_finite_parameter("A3O", value)?; self.params.p328 = value; self.mark_param_given(328); Ok(()) }
            "a3l" => { validate_finite_parameter("A3L", value)?; self.params.p329 = value; self.mark_param_given(329); Ok(()) }
            "a3w" => { validate_finite_parameter("A3W", value)?; self.params.p330 = value; self.mark_param_given(330); Ok(()) }
            "a4o" => { validate_finite_parameter("A4O", value)?; self.params.p331 = value; self.mark_param_given(331); Ok(()) }
            "a4l" => { validate_finite_parameter("A4L", value)?; self.params.p332 = value; self.mark_param_given(332); Ok(()) }
            "a4w" => { validate_finite_parameter("A4W", value)?; self.params.p333 = value; self.mark_param_given(333); Ok(()) }
            "imaxiio" => { validate_parameter("IMAXIIO", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); Ok(()) }
            "gcoo" => { validate_parameter("GCOO", value, Some((-10.0, "-10.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p335 = value; self.mark_param_given(335); Ok(()) }
            "iginvlw" => { validate_parameter("IGINVLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); Ok(()) }
            "igovw" => { validate_parameter("IGOVW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p337 = value; self.mark_param_given(337); Ok(()) }
            "igovdw" => { validate_parameter("IGOVDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p338 = value; self.mark_param_given(338); Ok(()) }
            "stigo" => { validate_finite_parameter("STIGO", value)?; self.params.p339 = value; self.mark_param_given(339); Ok(()) }
            "gc2o" => { validate_parameter("GC2O", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p340 = value; self.mark_param_given(340); Ok(()) }
            "gc3o" => { validate_parameter("GC3O", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p341 = value; self.mark_param_given(341); Ok(()) }
            "gc2ovo" => { validate_parameter("GC2OVO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p342 = value; self.mark_param_given(342); Ok(()) }
            "gc3ovo" => { validate_parameter("GC3OVO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p343 = value; self.mark_param_given(343); Ok(()) }
            "gc2ovdo" => { validate_parameter("GC2OVDO", value, Some((0.0, "0.0")), false, Some((10.0, "10.0")), false, &[])?; self.params.p344 = value; self.mark_param_given(344); Ok(()) }
            "gc3ovdo" => { validate_parameter("GC3OVDO", value, Some((-2.0, "-2.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p345 = value; self.mark_param_given(345); Ok(()) }
            "chibo" => { validate_parameter("CHIBO", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p346 = value; self.mark_param_given(346); Ok(()) }
            "agidlw" => { validate_parameter("AGIDLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p347 = value; self.mark_param_given(347); Ok(()) }
            "agidldw" => { validate_parameter("AGIDLDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p348 = value; self.mark_param_given(348); Ok(()) }
            "bgidlo" => { validate_parameter("BGIDLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p349 = value; self.mark_param_given(349); Ok(()) }
            "bgidldo" => { validate_parameter("BGIDLDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p350 = value; self.mark_param_given(350); Ok(()) }
            "stbgidlo" => { validate_finite_parameter("STBGIDLO", value)?; self.params.p351 = value; self.mark_param_given(351); Ok(()) }
            "stbgidldo" => { validate_finite_parameter("STBGIDLDO", value)?; self.params.p352 = value; self.mark_param_given(352); Ok(()) }
            "cgidlo" => { validate_finite_parameter("CGIDLO", value)?; self.params.p353 = value; self.mark_param_given(353); Ok(()) }
            "cgidldo" => { validate_finite_parameter("CGIDLDO", value)?; self.params.p354 = value; self.mark_param_given(354); Ok(()) }
            "delvtaco" => { validate_finite_parameter("DELVTACO", value)?; self.params.p355 = value; self.mark_param_given(355); Ok(()) }
            "delvtacl" => { validate_finite_parameter("DELVTACL", value)?; self.params.p356 = value; self.mark_param_given(356); Ok(()) }
            "delvtaclexp" => { validate_finite_parameter("DELVTACLEXP", value)?; self.params.p357 = value; self.mark_param_given(357); Ok(()) }
            "delvtacw" => { validate_finite_parameter("DELVTACW", value)?; self.params.p358 = value; self.mark_param_given(358); Ok(()) }
            "delvtaclw" => { validate_finite_parameter("DELVTACLW", value)?; self.params.p359 = value; self.mark_param_given(359); Ok(()) }
            "facneffaco" => { validate_finite_parameter("FACNEFFACO", value)?; self.params.p360 = value; self.mark_param_given(360); Ok(()) }
            "facneffacl" => { validate_finite_parameter("FACNEFFACL", value)?; self.params.p361 = value; self.mark_param_given(361); Ok(()) }
            "facneffacw" => { validate_finite_parameter("FACNEFFACW", value)?; self.params.p362 = value; self.mark_param_given(362); Ok(()) }
            "facneffaclw" => { validate_finite_parameter("FACNEFFACLW", value)?; self.params.p363 = value; self.mark_param_given(363); Ok(()) }
            "thesataco" => { validate_finite_parameter("THESATACO", value)?; self.params.p364 = value; self.mark_param_given(364); Ok(()) }
            "thesatacl" => { validate_finite_parameter("THESATACL", value)?; self.params.p365 = value; self.mark_param_given(365); Ok(()) }
            "thesataclexp" => { validate_finite_parameter("THESATACLEXP", value)?; self.params.p366 = value; self.mark_param_given(366); Ok(()) }
            "thesatacw" => { validate_finite_parameter("THESATACW", value)?; self.params.p367 = value; self.mark_param_given(367); Ok(()) }
            "thesataclw" => { validate_finite_parameter("THESATACLW", value)?; self.params.p368 = value; self.mark_param_given(368); Ok(()) }
            "axaco" => { validate_finite_parameter("AXACO", value)?; self.params.p369 = value; self.mark_param_given(369); Ok(()) }
            "axacl" => { validate_parameter("AXACL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p370 = value; self.mark_param_given(370); Ok(()) }
            "alpacl" => { validate_finite_parameter("ALPACL", value)?; self.params.p371 = value; self.mark_param_given(371); Ok(()) }
            "alpaclexp" => { validate_finite_parameter("ALPACLEXP", value)?; self.params.p372 = value; self.mark_param_given(372); Ok(()) }
            "alpacw" => { validate_finite_parameter("ALPACW", value)?; self.params.p373 = value; self.mark_param_given(373); Ok(()) }
            "alp1acl1" => { validate_finite_parameter("ALP1ACL1", value)?; self.params.p374 = value; self.mark_param_given(374); Ok(()) }
            "alp1aclexp" => { validate_finite_parameter("ALP1ACLEXP", value)?; self.params.p375 = value; self.mark_param_given(375); Ok(()) }
            "alp1acl2" => { validate_parameter("ALP1ACL2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p376 = value; self.mark_param_given(376); Ok(()) }
            "alp1acw" => { validate_finite_parameter("ALP1ACW", value)?; self.params.p377 = value; self.mark_param_given(377); Ok(()) }
            "fcgovacco" => { validate_parameter("FCGOVACCO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p378 = value; self.mark_param_given(378); Ok(()) }
            "fcgovaccdo" => { validate_parameter("FCGOVACCDO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p379 = value; self.mark_param_given(379); Ok(()) }
            "cgovaccgo" => { validate_parameter("CGOVACCGO", value, Some((0.1, "0.1")), false, Some((1.0, "1.0")), false, &[])?; self.params.p380 = value; self.mark_param_given(380); Ok(()) }
            "cgbovl" => { validate_parameter("CGBOVL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p381 = value; self.mark_param_given(381); Ok(()) }
            "cinrw" => { validate_parameter("CINRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p382 = value; self.mark_param_given(382); Ok(()) }
            "cinrdw" => { validate_parameter("CINRDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p383 = value; self.mark_param_given(383); Ok(()) }
            "dvfbinro" => { validate_finite_parameter("DVFBINRO", value)?; self.params.p384 = value; self.mark_param_given(384); Ok(()) }
            "fcinrdepo" => { validate_parameter("FCINRDEPO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p385 = value; self.mark_param_given(385); Ok(()) }
            "fcinracco" => { validate_parameter("FCINRACCO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p386 = value; self.mark_param_given(386); Ok(()) }
            "axinro" => { validate_parameter("AXINRO", value, Some((0.1, "0.1")), false, Some((4.0, "4.0")), false, &[])?; self.params.p387 = value; self.mark_param_given(387); Ok(()) }
            "cfrw" => { validate_parameter("CFRW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p388 = value; self.mark_param_given(388); Ok(()) }
            "cfrdw" => { validate_parameter("CFRDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p389 = value; self.mark_param_given(389); Ok(()) }
            "fnto" => { validate_parameter("FNTO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p390 = value; self.mark_param_given(390); Ok(()) }
            "fntexcl" => { validate_parameter("FNTEXCL", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p391 = value; self.mark_param_given(391); Ok(()) }
            "nfalw" => { validate_parameter("NFALW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p392 = value; self.mark_param_given(392); Ok(()) }
            "nfblw" => { validate_parameter("NFBLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p393 = value; self.mark_param_given(393); Ok(()) }
            "nfclw" => { validate_parameter("NFCLW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p394 = value; self.mark_param_given(394); Ok(()) }
            "efo" => { validate_parameter("EFO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p395 = value; self.mark_param_given(395); Ok(()) }
            "lintnoi" => { validate_finite_parameter("LINTNOI", value)?; self.params.p396 = value; self.mark_param_given(396); Ok(()) }
            "alpnoi" => { validate_finite_parameter("ALPNOI", value)?; self.params.p397 = value; self.mark_param_given(397); Ok(()) }
            "wedge" => { validate_parameter("WEDGE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p398 = value; self.mark_param_given(398); Ok(()) }
            "wedgew" => { validate_parameter("WEDGEW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p399 = value; self.mark_param_given(399); Ok(()) }
            "vfbedgeo" => { validate_finite_parameter("VFBEDGEO", value)?; self.params.p400 = value; self.mark_param_given(400); Ok(()) }
            "stvfbedgeo" => { validate_finite_parameter("STVFBEDGEO", value)?; self.params.p401 = value; self.mark_param_given(401); Ok(()) }
            "stvfbedgel" => { validate_finite_parameter("STVFBEDGEL", value)?; self.params.p402 = value; self.mark_param_given(402); Ok(()) }
            "stvfbedgew" => { validate_finite_parameter("STVFBEDGEW", value)?; self.params.p403 = value; self.mark_param_given(403); Ok(()) }
            "stvfbedgelw" => { validate_finite_parameter("STVFBEDGELW", value)?; self.params.p404 = value; self.mark_param_given(404); Ok(()) }
            "dphibedgeo" => { validate_finite_parameter("DPHIBEDGEO", value)?; self.params.p405 = value; self.mark_param_given(405); Ok(()) }
            "dphibedgel" => { validate_finite_parameter("DPHIBEDGEL", value)?; self.params.p406 = value; self.mark_param_given(406); Ok(()) }
            "dphibedgelexp" => { validate_finite_parameter("DPHIBEDGELEXP", value)?; self.params.p407 = value; self.mark_param_given(407); Ok(()) }
            "dphibedgew" => { validate_finite_parameter("DPHIBEDGEW", value)?; self.params.p408 = value; self.mark_param_given(408); Ok(()) }
            "dphibedgelw" => { validate_finite_parameter("DPHIBEDGELW", value)?; self.params.p409 = value; self.mark_param_given(409); Ok(()) }
            "nsubedgeo" => { validate_parameter("NSUBEDGEO", value, Some((1e20, "1e20")), false, None, true, &[])?; self.params.p410 = value; self.mark_param_given(410); Ok(()) }
            "nsubedgel" => { validate_finite_parameter("NSUBEDGEL", value)?; self.params.p411 = value; self.mark_param_given(411); Ok(()) }
            "nsubedgelexp" => { validate_finite_parameter("NSUBEDGELEXP", value)?; self.params.p412 = value; self.mark_param_given(412); Ok(()) }
            "nsubedgew" => { validate_finite_parameter("NSUBEDGEW", value)?; self.params.p413 = value; self.mark_param_given(413); Ok(()) }
            "nsubedgelw" => { validate_finite_parameter("NSUBEDGELW", value)?; self.params.p414 = value; self.mark_param_given(414); Ok(()) }
            "ctedgeo" => { validate_finite_parameter("CTEDGEO", value)?; self.params.p415 = value; self.mark_param_given(415); Ok(()) }
            "ctedgel" => { validate_finite_parameter("CTEDGEL", value)?; self.params.p416 = value; self.mark_param_given(416); Ok(()) }
            "ctedgelexp" => { validate_finite_parameter("CTEDGELEXP", value)?; self.params.p417 = value; self.mark_param_given(417); Ok(()) }
            "fbetedge" => { validate_finite_parameter("FBETEDGE", value)?; self.params.p418 = value; self.mark_param_given(418); Ok(()) }
            "lpedge" => { validate_parameter("LPEDGE", value, Some((1e-10, "1e-10")), false, None, true, &[])?; self.params.p419 = value; self.mark_param_given(419); Ok(()) }
            "betedgew" => { validate_finite_parameter("BETEDGEW", value)?; self.params.p420 = value; self.mark_param_given(420); Ok(()) }
            "stbetedgeo" => { validate_finite_parameter("STBETEDGEO", value)?; self.params.p421 = value; self.mark_param_given(421); Ok(()) }
            "stbetedgel" => { validate_finite_parameter("STBETEDGEL", value)?; self.params.p422 = value; self.mark_param_given(422); Ok(()) }
            "stbetedgew" => { validate_finite_parameter("STBETEDGEW", value)?; self.params.p423 = value; self.mark_param_given(423); Ok(()) }
            "stbetedgelw" => { validate_finite_parameter("STBETEDGELW", value)?; self.params.p424 = value; self.mark_param_given(424); Ok(()) }
            "psceedgel" => { validate_finite_parameter("PSCEEDGEL", value)?; self.params.p425 = value; self.mark_param_given(425); Ok(()) }
            "psceedgelexp" => { validate_finite_parameter("PSCEEDGELEXP", value)?; self.params.p426 = value; self.mark_param_given(426); Ok(()) }
            "psceedgew" => { validate_finite_parameter("PSCEEDGEW", value)?; self.params.p427 = value; self.mark_param_given(427); Ok(()) }
            "pscebedgeo" => { validate_parameter("PSCEBEDGEO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p428 = value; self.mark_param_given(428); Ok(()) }
            "pscededgeo" => { validate_parameter("PSCEDEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p429 = value; self.mark_param_given(429); Ok(()) }
            "cfedgel" => { validate_finite_parameter("CFEDGEL", value)?; self.params.p430 = value; self.mark_param_given(430); Ok(()) }
            "cfedgelexp" => { validate_finite_parameter("CFEDGELEXP", value)?; self.params.p431 = value; self.mark_param_given(431); Ok(()) }
            "cfedgew" => { validate_finite_parameter("CFEDGEW", value)?; self.params.p432 = value; self.mark_param_given(432); Ok(()) }
            "cfbedgeo" => { validate_parameter("CFBEDGEO", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p433 = value; self.mark_param_given(433); Ok(()) }
            "cfdedgeo" => { validate_parameter("CFDEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p434 = value; self.mark_param_given(434); Ok(()) }
            "fntedgeo" => { validate_parameter("FNTEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p435 = value; self.mark_param_given(435); Ok(()) }
            "nfaedgelw" => { validate_parameter("NFAEDGELW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p436 = value; self.mark_param_given(436); Ok(()) }
            "nfbedgelw" => { validate_parameter("NFBEDGELW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p437 = value; self.mark_param_given(437); Ok(()) }
            "nfcedgelw" => { validate_parameter("NFCEDGELW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p438 = value; self.mark_param_given(438); Ok(()) }
            "efedgeo" => { validate_parameter("EFEDGEO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p439 = value; self.mark_param_given(439); Ok(()) }
            "rgo" => { validate_finite_parameter("RGO", value)?; self.params.p440 = value; self.mark_param_given(440); Ok(()) }
            "rint" => { validate_parameter("RINT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p441 = value; self.mark_param_given(441); Ok(()) }
            "rvpoly" => { validate_parameter("RVPOLY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p442 = value; self.mark_param_given(442); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p443 = value; self.mark_param_given(443); Ok(()) }
            "dlsil" => { validate_finite_parameter("DLSIL", value)?; self.params.p444 = value; self.mark_param_given(444); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p445 = value; self.mark_param_given(445); Ok(()) }
            "rshd" => { validate_parameter("RSHD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p446 = value; self.mark_param_given(446); Ok(()) }
            "rbulko" => { validate_parameter("RBULKO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p447 = value; self.mark_param_given(447); Ok(()) }
            "rwello" => { validate_parameter("RWELLO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p448 = value; self.mark_param_given(448); Ok(()) }
            "rjunso" => { validate_parameter("RJUNSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p449 = value; self.mark_param_given(449); Ok(()) }
            "rjundo" => { validate_parameter("RJUNDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p450 = value; self.mark_param_given(450); Ok(()) }
            "rtho" => { validate_finite_parameter("RTHO", value)?; self.params.p451 = value; self.mark_param_given(451); Ok(()) }
            "rthw1" => { validate_finite_parameter("RTHW1", value)?; self.params.p452 = value; self.mark_param_given(452); Ok(()) }
            "rthw2" => { validate_finite_parameter("RTHW2", value)?; self.params.p453 = value; self.mark_param_given(453); Ok(()) }
            "rthlw" => { validate_finite_parameter("RTHLW", value)?; self.params.p454 = value; self.mark_param_given(454); Ok(()) }
            "ctho" => { validate_finite_parameter("CTHO", value)?; self.params.p455 = value; self.mark_param_given(455); Ok(()) }
            "cthw1" => { validate_finite_parameter("CTHW1", value)?; self.params.p456 = value; self.mark_param_given(456); Ok(()) }
            "cthw2" => { validate_finite_parameter("CTHW2", value)?; self.params.p457 = value; self.mark_param_given(457); Ok(()) }
            "cthlw" => { validate_finite_parameter("CTHLW", value)?; self.params.p458 = value; self.mark_param_given(458); Ok(()) }
            "strtho" => { validate_finite_parameter("STRTHO", value)?; self.params.p459 = value; self.mark_param_given(459); Ok(()) }
            "povfb" => { validate_finite_parameter("POVFB", value)?; self.params.p460 = value; self.mark_param_given(460); Ok(()) }
            "plvfb" => { validate_finite_parameter("PLVFB", value)?; self.params.p461 = value; self.mark_param_given(461); Ok(()) }
            "pwvfb" => { validate_finite_parameter("PWVFB", value)?; self.params.p462 = value; self.mark_param_given(462); Ok(()) }
            "plwvfb" => { validate_finite_parameter("PLWVFB", value)?; self.params.p463 = value; self.mark_param_given(463); Ok(()) }
            "postvfb" => { validate_finite_parameter("POSTVFB", value)?; self.params.p464 = value; self.mark_param_given(464); Ok(()) }
            "plstvfb" => { validate_finite_parameter("PLSTVFB", value)?; self.params.p465 = value; self.mark_param_given(465); Ok(()) }
            "pwstvfb" => { validate_finite_parameter("PWSTVFB", value)?; self.params.p466 = value; self.mark_param_given(466); Ok(()) }
            "plwstvfb" => { validate_finite_parameter("PLWSTVFB", value)?; self.params.p467 = value; self.mark_param_given(467); Ok(()) }
            "poneff" => { validate_finite_parameter("PONEFF", value)?; self.params.p468 = value; self.mark_param_given(468); Ok(()) }
            "plneff" => { validate_finite_parameter("PLNEFF", value)?; self.params.p469 = value; self.mark_param_given(469); Ok(()) }
            "pwneff" => { validate_finite_parameter("PWNEFF", value)?; self.params.p470 = value; self.mark_param_given(470); Ok(()) }
            "plwneff" => { validate_finite_parameter("PLWNEFF", value)?; self.params.p471 = value; self.mark_param_given(471); Ok(()) }
            "pogfacnud" => { validate_finite_parameter("POGFACNUD", value)?; self.params.p472 = value; self.mark_param_given(472); Ok(()) }
            "plgfacnud" => { validate_finite_parameter("PLGFACNUD", value)?; self.params.p473 = value; self.mark_param_given(473); Ok(()) }
            "pwgfacnud" => { validate_finite_parameter("PWGFACNUD", value)?; self.params.p474 = value; self.mark_param_given(474); Ok(()) }
            "plwgfacnud" => { validate_finite_parameter("PLWGFACNUD", value)?; self.params.p475 = value; self.mark_param_given(475); Ok(()) }
            "povsbnud" => { validate_finite_parameter("POVSBNUD", value)?; self.params.p476 = value; self.mark_param_given(476); Ok(()) }
            "plvsbnud" => { validate_finite_parameter("PLVSBNUD", value)?; self.params.p477 = value; self.mark_param_given(477); Ok(()) }
            "pwvsbnud" => { validate_finite_parameter("PWVSBNUD", value)?; self.params.p478 = value; self.mark_param_given(478); Ok(()) }
            "plwvsbnud" => { validate_finite_parameter("PLWVSBNUD", value)?; self.params.p479 = value; self.mark_param_given(479); Ok(()) }
            "podphib" => { validate_finite_parameter("PODPHIB", value)?; self.params.p480 = value; self.mark_param_given(480); Ok(()) }
            "pldphib" => { validate_finite_parameter("PLDPHIB", value)?; self.params.p481 = value; self.mark_param_given(481); Ok(()) }
            "pwdphib" => { validate_finite_parameter("PWDPHIB", value)?; self.params.p482 = value; self.mark_param_given(482); Ok(()) }
            "plwdphib" => { validate_finite_parameter("PLWDPHIB", value)?; self.params.p483 = value; self.mark_param_given(483); Ok(()) }
            "ponp" => { validate_finite_parameter("PONP", value)?; self.params.p484 = value; self.mark_param_given(484); Ok(()) }
            "plnp" => { validate_finite_parameter("PLNP", value)?; self.params.p485 = value; self.mark_param_given(485); Ok(()) }
            "pwnp" => { validate_finite_parameter("PWNP", value)?; self.params.p486 = value; self.mark_param_given(486); Ok(()) }
            "plwnp" => { validate_finite_parameter("PLWNP", value)?; self.params.p487 = value; self.mark_param_given(487); Ok(()) }
            "ponov" => { validate_finite_parameter("PONOV", value)?; self.params.p488 = value; self.mark_param_given(488); Ok(()) }
            "plnov" => { validate_finite_parameter("PLNOV", value)?; self.params.p489 = value; self.mark_param_given(489); Ok(()) }
            "pwnov" => { validate_finite_parameter("PWNOV", value)?; self.params.p490 = value; self.mark_param_given(490); Ok(()) }
            "plwnov" => { validate_finite_parameter("PLWNOV", value)?; self.params.p491 = value; self.mark_param_given(491); Ok(()) }
            "ponovd" => { validate_finite_parameter("PONOVD", value)?; self.params.p492 = value; self.mark_param_given(492); Ok(()) }
            "plnovd" => { validate_finite_parameter("PLNOVD", value)?; self.params.p493 = value; self.mark_param_given(493); Ok(()) }
            "pwnovd" => { validate_finite_parameter("PWNOVD", value)?; self.params.p494 = value; self.mark_param_given(494); Ok(()) }
            "plwnovd" => { validate_finite_parameter("PLWNOVD", value)?; self.params.p495 = value; self.mark_param_given(495); Ok(()) }
            "poct" => { validate_finite_parameter("POCT", value)?; self.params.p496 = value; self.mark_param_given(496); Ok(()) }
            "plct" => { validate_finite_parameter("PLCT", value)?; self.params.p497 = value; self.mark_param_given(497); Ok(()) }
            "pwct" => { validate_finite_parameter("PWCT", value)?; self.params.p498 = value; self.mark_param_given(498); Ok(()) }
            "plwct" => { validate_finite_parameter("PLWCT", value)?; self.params.p499 = value; self.mark_param_given(499); Ok(()) }
            "poctb" => { validate_finite_parameter("POCTB", value)?; self.params.p500 = value; self.mark_param_given(500); Ok(()) }
            "plctb" => { validate_finite_parameter("PLCTB", value)?; self.params.p501 = value; self.mark_param_given(501); Ok(()) }
            "pwctb" => { validate_finite_parameter("PWCTB", value)?; self.params.p502 = value; self.mark_param_given(502); Ok(()) }
            "plwctb" => { validate_finite_parameter("PLWCTB", value)?; self.params.p503 = value; self.mark_param_given(503); Ok(()) }
            "poctg" => { validate_finite_parameter("POCTG", value)?; self.params.p504 = value; self.mark_param_given(504); Ok(()) }
            "plctg" => { validate_finite_parameter("PLCTG", value)?; self.params.p505 = value; self.mark_param_given(505); Ok(()) }
            "pwctg" => { validate_finite_parameter("PWCTG", value)?; self.params.p506 = value; self.mark_param_given(506); Ok(()) }
            "plwctg" => { validate_finite_parameter("PLWCTG", value)?; self.params.p507 = value; self.mark_param_given(507); Ok(()) }
            "postct" => { validate_finite_parameter("POSTCT", value)?; self.params.p508 = value; self.mark_param_given(508); Ok(()) }
            "plstct" => { validate_finite_parameter("PLSTCT", value)?; self.params.p509 = value; self.mark_param_given(509); Ok(()) }
            "pwstct" => { validate_finite_parameter("PWSTCT", value)?; self.params.p510 = value; self.mark_param_given(510); Ok(()) }
            "plwstct" => { validate_finite_parameter("PLWSTCT", value)?; self.params.p511 = value; self.mark_param_given(511); Ok(()) }
            "pocf" => { validate_finite_parameter("POCF", value)?; self.params.p512 = value; self.mark_param_given(512); Ok(()) }
            "plcf" => { validate_finite_parameter("PLCF", value)?; self.params.p513 = value; self.mark_param_given(513); Ok(()) }
            "pwcf" => { validate_finite_parameter("PWCF", value)?; self.params.p514 = value; self.mark_param_given(514); Ok(()) }
            "plwcf" => { validate_finite_parameter("PLWCF", value)?; self.params.p515 = value; self.mark_param_given(515); Ok(()) }
            "pocfb" => { validate_finite_parameter("POCFB", value)?; self.params.p516 = value; self.mark_param_given(516); Ok(()) }
            "plcfb" => { validate_finite_parameter("PLCFB", value)?; self.params.p517 = value; self.mark_param_given(517); Ok(()) }
            "pwcfb" => { validate_finite_parameter("PWCFB", value)?; self.params.p518 = value; self.mark_param_given(518); Ok(()) }
            "plwcfb" => { validate_finite_parameter("PLWCFB", value)?; self.params.p519 = value; self.mark_param_given(519); Ok(()) }
            "pocfd" => { validate_finite_parameter("POCFD", value)?; self.params.p520 = value; self.mark_param_given(520); Ok(()) }
            "plcfd" => { validate_finite_parameter("PLCFD", value)?; self.params.p521 = value; self.mark_param_given(521); Ok(()) }
            "pwcfd" => { validate_finite_parameter("PWCFD", value)?; self.params.p522 = value; self.mark_param_given(522); Ok(()) }
            "plwcfd" => { validate_finite_parameter("PLWCFD", value)?; self.params.p523 = value; self.mark_param_given(523); Ok(()) }
            "popsce" => { validate_finite_parameter("POPSCE", value)?; self.params.p524 = value; self.mark_param_given(524); Ok(()) }
            "plpsce" => { validate_finite_parameter("PLPSCE", value)?; self.params.p525 = value; self.mark_param_given(525); Ok(()) }
            "pwpsce" => { validate_finite_parameter("PWPSCE", value)?; self.params.p526 = value; self.mark_param_given(526); Ok(()) }
            "plwpsce" => { validate_finite_parameter("PLWPSCE", value)?; self.params.p527 = value; self.mark_param_given(527); Ok(()) }
            "popsceb" => { validate_finite_parameter("POPSCEB", value)?; self.params.p528 = value; self.mark_param_given(528); Ok(()) }
            "plpsceb" => { validate_finite_parameter("PLPSCEB", value)?; self.params.p529 = value; self.mark_param_given(529); Ok(()) }
            "pwpsceb" => { validate_finite_parameter("PWPSCEB", value)?; self.params.p530 = value; self.mark_param_given(530); Ok(()) }
            "plwpsceb" => { validate_finite_parameter("PLWPSCEB", value)?; self.params.p531 = value; self.mark_param_given(531); Ok(()) }
            "popsced" => { validate_finite_parameter("POPSCED", value)?; self.params.p532 = value; self.mark_param_given(532); Ok(()) }
            "plpsced" => { validate_finite_parameter("PLPSCED", value)?; self.params.p533 = value; self.mark_param_given(533); Ok(()) }
            "pwpsced" => { validate_finite_parameter("PWPSCED", value)?; self.params.p534 = value; self.mark_param_given(534); Ok(()) }
            "plwpsced" => { validate_finite_parameter("PLWPSCED", value)?; self.params.p535 = value; self.mark_param_given(535); Ok(()) }
            "pobetn" => { validate_finite_parameter("POBETN", value)?; self.params.p536 = value; self.mark_param_given(536); Ok(()) }
            "plbetn" => { validate_finite_parameter("PLBETN", value)?; self.params.p537 = value; self.mark_param_given(537); Ok(()) }
            "pwbetn" => { validate_finite_parameter("PWBETN", value)?; self.params.p538 = value; self.mark_param_given(538); Ok(()) }
            "plwbetn" => { validate_finite_parameter("PLWBETN", value)?; self.params.p539 = value; self.mark_param_given(539); Ok(()) }
            "postbet" => { validate_finite_parameter("POSTBET", value)?; self.params.p540 = value; self.mark_param_given(540); Ok(()) }
            "plstbet" => { validate_finite_parameter("PLSTBET", value)?; self.params.p541 = value; self.mark_param_given(541); Ok(()) }
            "pwstbet" => { validate_finite_parameter("PWSTBET", value)?; self.params.p542 = value; self.mark_param_given(542); Ok(()) }
            "plwstbet" => { validate_finite_parameter("PLWSTBET", value)?; self.params.p543 = value; self.mark_param_given(543); Ok(()) }
            "pomue" => { validate_finite_parameter("POMUE", value)?; self.params.p544 = value; self.mark_param_given(544); Ok(()) }
            "plmue" => { validate_finite_parameter("PLMUE", value)?; self.params.p545 = value; self.mark_param_given(545); Ok(()) }
            "pwmue" => { validate_finite_parameter("PWMUE", value)?; self.params.p546 = value; self.mark_param_given(546); Ok(()) }
            "plwmue" => { validate_finite_parameter("PLWMUE", value)?; self.params.p547 = value; self.mark_param_given(547); Ok(()) }
            "pothemu" => { validate_finite_parameter("POTHEMU", value)?; self.params.p548 = value; self.mark_param_given(548); Ok(()) }
            "plthemu" => { validate_finite_parameter("PLTHEMU", value)?; self.params.p549 = value; self.mark_param_given(549); Ok(()) }
            "pwthemu" => { validate_finite_parameter("PWTHEMU", value)?; self.params.p550 = value; self.mark_param_given(550); Ok(()) }
            "plwthemu" => { validate_finite_parameter("PLWTHEMU", value)?; self.params.p551 = value; self.mark_param_given(551); Ok(()) }
            "pocs" => { validate_finite_parameter("POCS", value)?; self.params.p552 = value; self.mark_param_given(552); Ok(()) }
            "plcs" => { validate_finite_parameter("PLCS", value)?; self.params.p553 = value; self.mark_param_given(553); Ok(()) }
            "pwcs" => { validate_finite_parameter("PWCS", value)?; self.params.p554 = value; self.mark_param_given(554); Ok(()) }
            "plwcs" => { validate_finite_parameter("PLWCS", value)?; self.params.p555 = value; self.mark_param_given(555); Ok(()) }
            "pothecs" => { validate_finite_parameter("POTHECS", value)?; self.params.p556 = value; self.mark_param_given(556); Ok(()) }
            "plthecs" => { validate_finite_parameter("PLTHECS", value)?; self.params.p557 = value; self.mark_param_given(557); Ok(()) }
            "pwthecs" => { validate_finite_parameter("PWTHECS", value)?; self.params.p558 = value; self.mark_param_given(558); Ok(()) }
            "plwthecs" => { validate_finite_parameter("PLWTHECS", value)?; self.params.p559 = value; self.mark_param_given(559); Ok(()) }
            "poxcor" => { validate_finite_parameter("POXCOR", value)?; self.params.p560 = value; self.mark_param_given(560); Ok(()) }
            "plxcor" => { validate_finite_parameter("PLXCOR", value)?; self.params.p561 = value; self.mark_param_given(561); Ok(()) }
            "pwxcor" => { validate_finite_parameter("PWXCOR", value)?; self.params.p562 = value; self.mark_param_given(562); Ok(()) }
            "plwxcor" => { validate_finite_parameter("PLWXCOR", value)?; self.params.p563 = value; self.mark_param_given(563); Ok(()) }
            "pors" => { validate_finite_parameter("PORS", value)?; self.params.p564 = value; self.mark_param_given(564); Ok(()) }
            "plrs" => { validate_finite_parameter("PLRS", value)?; self.params.p565 = value; self.mark_param_given(565); Ok(()) }
            "pwrs" => { validate_finite_parameter("PWRS", value)?; self.params.p566 = value; self.mark_param_given(566); Ok(()) }
            "plwrs" => { validate_finite_parameter("PLWRS", value)?; self.params.p567 = value; self.mark_param_given(567); Ok(()) }
            "postrs" => { validate_finite_parameter("POSTRS", value)?; self.params.p568 = value; self.mark_param_given(568); Ok(()) }
            "plstrs" => { validate_finite_parameter("PLSTRS", value)?; self.params.p569 = value; self.mark_param_given(569); Ok(()) }
            "pwstrs" => { validate_finite_parameter("PWSTRS", value)?; self.params.p570 = value; self.mark_param_given(570); Ok(()) }
            "plwstrs" => { validate_finite_parameter("PLWSTRS", value)?; self.params.p571 = value; self.mark_param_given(571); Ok(()) }
            "porsb" => { validate_finite_parameter("PORSB", value)?; self.params.p572 = value; self.mark_param_given(572); Ok(()) }
            "plrsb" => { validate_finite_parameter("PLRSB", value)?; self.params.p573 = value; self.mark_param_given(573); Ok(()) }
            "pwrsb" => { validate_finite_parameter("PWRSB", value)?; self.params.p574 = value; self.mark_param_given(574); Ok(()) }
            "plwrsb" => { validate_finite_parameter("PLWRSB", value)?; self.params.p575 = value; self.mark_param_given(575); Ok(()) }
            "porsg" => { validate_finite_parameter("PORSG", value)?; self.params.p576 = value; self.mark_param_given(576); Ok(()) }
            "plrsg" => { validate_finite_parameter("PLRSG", value)?; self.params.p577 = value; self.mark_param_given(577); Ok(()) }
            "pwrsg" => { validate_finite_parameter("PWRSG", value)?; self.params.p578 = value; self.mark_param_given(578); Ok(()) }
            "plwrsg" => { validate_finite_parameter("PLWRSG", value)?; self.params.p579 = value; self.mark_param_given(579); Ok(()) }
            "pothesat" => { validate_finite_parameter("POTHESAT", value)?; self.params.p580 = value; self.mark_param_given(580); Ok(()) }
            "plthesat" => { validate_finite_parameter("PLTHESAT", value)?; self.params.p581 = value; self.mark_param_given(581); Ok(()) }
            "pwthesat" => { validate_finite_parameter("PWTHESAT", value)?; self.params.p582 = value; self.mark_param_given(582); Ok(()) }
            "plwthesat" => { validate_finite_parameter("PLWTHESAT", value)?; self.params.p583 = value; self.mark_param_given(583); Ok(()) }
            "postthesat" => { validate_finite_parameter("POSTTHESAT", value)?; self.params.p584 = value; self.mark_param_given(584); Ok(()) }
            "plstthesat" => { validate_finite_parameter("PLSTTHESAT", value)?; self.params.p585 = value; self.mark_param_given(585); Ok(()) }
            "pwstthesat" => { validate_finite_parameter("PWSTTHESAT", value)?; self.params.p586 = value; self.mark_param_given(586); Ok(()) }
            "plwstthesat" => { validate_finite_parameter("PLWSTTHESAT", value)?; self.params.p587 = value; self.mark_param_given(587); Ok(()) }
            "pothesatb" => { validate_finite_parameter("POTHESATB", value)?; self.params.p588 = value; self.mark_param_given(588); Ok(()) }
            "plthesatb" => { validate_finite_parameter("PLTHESATB", value)?; self.params.p589 = value; self.mark_param_given(589); Ok(()) }
            "pwthesatb" => { validate_finite_parameter("PWTHESATB", value)?; self.params.p590 = value; self.mark_param_given(590); Ok(()) }
            "plwthesatb" => { validate_finite_parameter("PLWTHESATB", value)?; self.params.p591 = value; self.mark_param_given(591); Ok(()) }
            "pothesatg" => { validate_finite_parameter("POTHESATG", value)?; self.params.p592 = value; self.mark_param_given(592); Ok(()) }
            "plthesatg" => { validate_finite_parameter("PLTHESATG", value)?; self.params.p593 = value; self.mark_param_given(593); Ok(()) }
            "pwthesatg" => { validate_finite_parameter("PWTHESATG", value)?; self.params.p594 = value; self.mark_param_given(594); Ok(()) }
            "plwthesatg" => { validate_finite_parameter("PLWTHESATG", value)?; self.params.p595 = value; self.mark_param_given(595); Ok(()) }
            "poax" => { validate_finite_parameter("POAX", value)?; self.params.p596 = value; self.mark_param_given(596); Ok(()) }
            "plax" => { validate_finite_parameter("PLAX", value)?; self.params.p597 = value; self.mark_param_given(597); Ok(()) }
            "pwax" => { validate_finite_parameter("PWAX", value)?; self.params.p598 = value; self.mark_param_given(598); Ok(()) }
            "plwax" => { validate_finite_parameter("PLWAX", value)?; self.params.p599 = value; self.mark_param_given(599); Ok(()) }
            "poalp" => { validate_finite_parameter("POALP", value)?; self.params.p600 = value; self.mark_param_given(600); Ok(()) }
            "plalp" => { validate_finite_parameter("PLALP", value)?; self.params.p601 = value; self.mark_param_given(601); Ok(()) }
            "pwalp" => { validate_finite_parameter("PWALP", value)?; self.params.p602 = value; self.mark_param_given(602); Ok(()) }
            "plwalp" => { validate_finite_parameter("PLWALP", value)?; self.params.p603 = value; self.mark_param_given(603); Ok(()) }
            "poalp1" => { validate_finite_parameter("POALP1", value)?; self.params.p604 = value; self.mark_param_given(604); Ok(()) }
            "plalp1" => { validate_finite_parameter("PLALP1", value)?; self.params.p605 = value; self.mark_param_given(605); Ok(()) }
            "pwalp1" => { validate_finite_parameter("PWALP1", value)?; self.params.p606 = value; self.mark_param_given(606); Ok(()) }
            "plwalp1" => { validate_finite_parameter("PLWALP1", value)?; self.params.p607 = value; self.mark_param_given(607); Ok(()) }
            "poalp2" => { validate_finite_parameter("POALP2", value)?; self.params.p608 = value; self.mark_param_given(608); Ok(()) }
            "plalp2" => { validate_finite_parameter("PLALP2", value)?; self.params.p609 = value; self.mark_param_given(609); Ok(()) }
            "pwalp2" => { validate_finite_parameter("PWALP2", value)?; self.params.p610 = value; self.mark_param_given(610); Ok(()) }
            "plwalp2" => { validate_finite_parameter("PLWALP2", value)?; self.params.p611 = value; self.mark_param_given(611); Ok(()) }
            "poa1" => { validate_finite_parameter("POA1", value)?; self.params.p612 = value; self.mark_param_given(612); Ok(()) }
            "pla1" => { validate_finite_parameter("PLA1", value)?; self.params.p613 = value; self.mark_param_given(613); Ok(()) }
            "pwa1" => { validate_finite_parameter("PWA1", value)?; self.params.p614 = value; self.mark_param_given(614); Ok(()) }
            "plwa1" => { validate_finite_parameter("PLWA1", value)?; self.params.p615 = value; self.mark_param_given(615); Ok(()) }
            "posta2" => { validate_finite_parameter("POSTA2", value)?; self.params.p616 = value; self.mark_param_given(616); Ok(()) }
            "plsta2" => { validate_finite_parameter("PLSTA2", value)?; self.params.p617 = value; self.mark_param_given(617); Ok(()) }
            "pwsta2" => { validate_finite_parameter("PWSTA2", value)?; self.params.p618 = value; self.mark_param_given(618); Ok(()) }
            "plwsta2" => { validate_finite_parameter("PLWSTA2", value)?; self.params.p619 = value; self.mark_param_given(619); Ok(()) }
            "poa3" => { validate_finite_parameter("POA3", value)?; self.params.p620 = value; self.mark_param_given(620); Ok(()) }
            "pla3" => { validate_finite_parameter("PLA3", value)?; self.params.p621 = value; self.mark_param_given(621); Ok(()) }
            "pwa3" => { validate_finite_parameter("PWA3", value)?; self.params.p622 = value; self.mark_param_given(622); Ok(()) }
            "plwa3" => { validate_finite_parameter("PLWA3", value)?; self.params.p623 = value; self.mark_param_given(623); Ok(()) }
            "poa4" => { validate_finite_parameter("POA4", value)?; self.params.p624 = value; self.mark_param_given(624); Ok(()) }
            "pla4" => { validate_finite_parameter("PLA4", value)?; self.params.p625 = value; self.mark_param_given(625); Ok(()) }
            "pwa4" => { validate_finite_parameter("PWA4", value)?; self.params.p626 = value; self.mark_param_given(626); Ok(()) }
            "plwa4" => { validate_finite_parameter("PLWA4", value)?; self.params.p627 = value; self.mark_param_given(627); Ok(()) }
            "poiginv" => { validate_finite_parameter("POIGINV", value)?; self.params.p628 = value; self.mark_param_given(628); Ok(()) }
            "pliginv" => { validate_finite_parameter("PLIGINV", value)?; self.params.p629 = value; self.mark_param_given(629); Ok(()) }
            "pwiginv" => { validate_finite_parameter("PWIGINV", value)?; self.params.p630 = value; self.mark_param_given(630); Ok(()) }
            "plwiginv" => { validate_finite_parameter("PLWIGINV", value)?; self.params.p631 = value; self.mark_param_given(631); Ok(()) }
            "poigov" => { validate_finite_parameter("POIGOV", value)?; self.params.p632 = value; self.mark_param_given(632); Ok(()) }
            "pligov" => { validate_finite_parameter("PLIGOV", value)?; self.params.p633 = value; self.mark_param_given(633); Ok(()) }
            "pwigov" => { validate_finite_parameter("PWIGOV", value)?; self.params.p634 = value; self.mark_param_given(634); Ok(()) }
            "plwigov" => { validate_finite_parameter("PLWIGOV", value)?; self.params.p635 = value; self.mark_param_given(635); Ok(()) }
            "poigovd" => { validate_finite_parameter("POIGOVD", value)?; self.params.p636 = value; self.mark_param_given(636); Ok(()) }
            "pligovd" => { validate_finite_parameter("PLIGOVD", value)?; self.params.p637 = value; self.mark_param_given(637); Ok(()) }
            "pwigovd" => { validate_finite_parameter("PWIGOVD", value)?; self.params.p638 = value; self.mark_param_given(638); Ok(()) }
            "plwigovd" => { validate_finite_parameter("PLWIGOVD", value)?; self.params.p639 = value; self.mark_param_given(639); Ok(()) }
            "postig" => { validate_finite_parameter("POSTIG", value)?; self.params.p640 = value; self.mark_param_given(640); Ok(()) }
            "plstig" => { validate_finite_parameter("PLSTIG", value)?; self.params.p641 = value; self.mark_param_given(641); Ok(()) }
            "pwstig" => { validate_finite_parameter("PWSTIG", value)?; self.params.p642 = value; self.mark_param_given(642); Ok(()) }
            "plwstig" => { validate_finite_parameter("PLWSTIG", value)?; self.params.p643 = value; self.mark_param_given(643); Ok(()) }
            "poagidl" => { validate_finite_parameter("POAGIDL", value)?; self.params.p644 = value; self.mark_param_given(644); Ok(()) }
            "plagidl" => { validate_finite_parameter("PLAGIDL", value)?; self.params.p645 = value; self.mark_param_given(645); Ok(()) }
            "pwagidl" => { validate_finite_parameter("PWAGIDL", value)?; self.params.p646 = value; self.mark_param_given(646); Ok(()) }
            "plwagidl" => { validate_finite_parameter("PLWAGIDL", value)?; self.params.p647 = value; self.mark_param_given(647); Ok(()) }
            "poagidld" => { validate_finite_parameter("POAGIDLD", value)?; self.params.p648 = value; self.mark_param_given(648); Ok(()) }
            "plagidld" => { validate_finite_parameter("PLAGIDLD", value)?; self.params.p649 = value; self.mark_param_given(649); Ok(()) }
            "pwagidld" => { validate_finite_parameter("PWAGIDLD", value)?; self.params.p650 = value; self.mark_param_given(650); Ok(()) }
            "plwagidld" => { validate_finite_parameter("PLWAGIDLD", value)?; self.params.p651 = value; self.mark_param_given(651); Ok(()) }
            "postbgidl" => { validate_finite_parameter("POSTBGIDL", value)?; self.params.p652 = value; self.mark_param_given(652); Ok(()) }
            "plstbgidl" => { validate_finite_parameter("PLSTBGIDL", value)?; self.params.p653 = value; self.mark_param_given(653); Ok(()) }
            "pwstbgidl" => { validate_finite_parameter("PWSTBGIDL", value)?; self.params.p654 = value; self.mark_param_given(654); Ok(()) }
            "plwstbgidl" => { validate_finite_parameter("PLWSTBGIDL", value)?; self.params.p655 = value; self.mark_param_given(655); Ok(()) }
            "postbgidld" => { validate_finite_parameter("POSTBGIDLD", value)?; self.params.p656 = value; self.mark_param_given(656); Ok(()) }
            "plstbgidld" => { validate_finite_parameter("PLSTBGIDLD", value)?; self.params.p657 = value; self.mark_param_given(657); Ok(()) }
            "pwstbgidld" => { validate_finite_parameter("PWSTBGIDLD", value)?; self.params.p658 = value; self.mark_param_given(658); Ok(()) }
            "plwstbgidld" => { validate_finite_parameter("PLWSTBGIDLD", value)?; self.params.p659 = value; self.mark_param_given(659); Ok(()) }
            "pocox" => { validate_finite_parameter("POCOX", value)?; self.params.p660 = value; self.mark_param_given(660); Ok(()) }
            "plcox" => { validate_finite_parameter("PLCOX", value)?; self.params.p661 = value; self.mark_param_given(661); Ok(()) }
            "pwcox" => { validate_finite_parameter("PWCOX", value)?; self.params.p662 = value; self.mark_param_given(662); Ok(()) }
            "plwcox" => { validate_finite_parameter("PLWCOX", value)?; self.params.p663 = value; self.mark_param_given(663); Ok(()) }
            "podelvtac" => { validate_finite_parameter("PODELVTAC", value)?; self.params.p664 = value; self.mark_param_given(664); Ok(()) }
            "pldelvtac" => { validate_finite_parameter("PLDELVTAC", value)?; self.params.p665 = value; self.mark_param_given(665); Ok(()) }
            "pwdelvtac" => { validate_finite_parameter("PWDELVTAC", value)?; self.params.p666 = value; self.mark_param_given(666); Ok(()) }
            "plwdelvtac" => { validate_finite_parameter("PLWDELVTAC", value)?; self.params.p667 = value; self.mark_param_given(667); Ok(()) }
            "pofacneffac" => { validate_finite_parameter("POFACNEFFAC", value)?; self.params.p668 = value; self.mark_param_given(668); Ok(()) }
            "plfacneffac" => { validate_finite_parameter("PLFACNEFFAC", value)?; self.params.p669 = value; self.mark_param_given(669); Ok(()) }
            "pwfacneffac" => { validate_finite_parameter("PWFACNEFFAC", value)?; self.params.p670 = value; self.mark_param_given(670); Ok(()) }
            "plwfacneffac" => { validate_finite_parameter("PLWFACNEFFAC", value)?; self.params.p671 = value; self.mark_param_given(671); Ok(()) }
            "pothesatac" => { validate_finite_parameter("POTHESATAC", value)?; self.params.p672 = value; self.mark_param_given(672); Ok(()) }
            "plthesatac" => { validate_finite_parameter("PLTHESATAC", value)?; self.params.p673 = value; self.mark_param_given(673); Ok(()) }
            "pwthesatac" => { validate_finite_parameter("PWTHESATAC", value)?; self.params.p674 = value; self.mark_param_given(674); Ok(()) }
            "plwthesatac" => { validate_finite_parameter("PLWTHESATAC", value)?; self.params.p675 = value; self.mark_param_given(675); Ok(()) }
            "poaxac" => { validate_finite_parameter("POAXAC", value)?; self.params.p676 = value; self.mark_param_given(676); Ok(()) }
            "plaxac" => { validate_finite_parameter("PLAXAC", value)?; self.params.p677 = value; self.mark_param_given(677); Ok(()) }
            "pwaxac" => { validate_finite_parameter("PWAXAC", value)?; self.params.p678 = value; self.mark_param_given(678); Ok(()) }
            "plwaxac" => { validate_finite_parameter("PLWAXAC", value)?; self.params.p679 = value; self.mark_param_given(679); Ok(()) }
            "poalpac" => { validate_finite_parameter("POALPAC", value)?; self.params.p680 = value; self.mark_param_given(680); Ok(()) }
            "plalpac" => { validate_finite_parameter("PLALPAC", value)?; self.params.p681 = value; self.mark_param_given(681); Ok(()) }
            "pwalpac" => { validate_finite_parameter("PWALPAC", value)?; self.params.p682 = value; self.mark_param_given(682); Ok(()) }
            "plwalpac" => { validate_finite_parameter("PLWALPAC", value)?; self.params.p683 = value; self.mark_param_given(683); Ok(()) }
            "poalp1ac" => { validate_finite_parameter("POALP1AC", value)?; self.params.p684 = value; self.mark_param_given(684); Ok(()) }
            "plalp1ac" => { validate_finite_parameter("PLALP1AC", value)?; self.params.p685 = value; self.mark_param_given(685); Ok(()) }
            "pwalp1ac" => { validate_finite_parameter("PWALP1AC", value)?; self.params.p686 = value; self.mark_param_given(686); Ok(()) }
            "plwalp1ac" => { validate_finite_parameter("PLWALP1AC", value)?; self.params.p687 = value; self.mark_param_given(687); Ok(()) }
            "pocgov" => { validate_finite_parameter("POCGOV", value)?; self.params.p688 = value; self.mark_param_given(688); Ok(()) }
            "plcgov" => { validate_finite_parameter("PLCGOV", value)?; self.params.p689 = value; self.mark_param_given(689); Ok(()) }
            "pwcgov" => { validate_finite_parameter("PWCGOV", value)?; self.params.p690 = value; self.mark_param_given(690); Ok(()) }
            "plwcgov" => { validate_finite_parameter("PLWCGOV", value)?; self.params.p691 = value; self.mark_param_given(691); Ok(()) }
            "pocgovd" => { validate_finite_parameter("POCGOVD", value)?; self.params.p692 = value; self.mark_param_given(692); Ok(()) }
            "plcgovd" => { validate_finite_parameter("PLCGOVD", value)?; self.params.p693 = value; self.mark_param_given(693); Ok(()) }
            "pwcgovd" => { validate_finite_parameter("PWCGOVD", value)?; self.params.p694 = value; self.mark_param_given(694); Ok(()) }
            "plwcgovd" => { validate_finite_parameter("PLWCGOVD", value)?; self.params.p695 = value; self.mark_param_given(695); Ok(()) }
            "pocgbov" => { validate_finite_parameter("POCGBOV", value)?; self.params.p696 = value; self.mark_param_given(696); Ok(()) }
            "plcgbov" => { validate_finite_parameter("PLCGBOV", value)?; self.params.p697 = value; self.mark_param_given(697); Ok(()) }
            "pwcgbov" => { validate_finite_parameter("PWCGBOV", value)?; self.params.p698 = value; self.mark_param_given(698); Ok(()) }
            "plwcgbov" => { validate_finite_parameter("PLWCGBOV", value)?; self.params.p699 = value; self.mark_param_given(699); Ok(()) }
            "pocinr" => { validate_finite_parameter("POCINR", value)?; self.params.p700 = value; self.mark_param_given(700); Ok(()) }
            "plcinr" => { validate_finite_parameter("PLCINR", value)?; self.params.p701 = value; self.mark_param_given(701); Ok(()) }
            "pwcinr" => { validate_finite_parameter("PWCINR", value)?; self.params.p702 = value; self.mark_param_given(702); Ok(()) }
            "plwcinr" => { validate_finite_parameter("PLWCINR", value)?; self.params.p703 = value; self.mark_param_given(703); Ok(()) }
            "pocinrd" => { validate_finite_parameter("POCINRD", value)?; self.params.p704 = value; self.mark_param_given(704); Ok(()) }
            "plcinrd" => { validate_finite_parameter("PLCINRD", value)?; self.params.p705 = value; self.mark_param_given(705); Ok(()) }
            "pwcinrd" => { validate_finite_parameter("PWCINRD", value)?; self.params.p706 = value; self.mark_param_given(706); Ok(()) }
            "plwcinrd" => { validate_finite_parameter("PLWCINRD", value)?; self.params.p707 = value; self.mark_param_given(707); Ok(()) }
            "pocfr" => { validate_finite_parameter("POCFR", value)?; self.params.p708 = value; self.mark_param_given(708); Ok(()) }
            "plcfr" => { validate_finite_parameter("PLCFR", value)?; self.params.p709 = value; self.mark_param_given(709); Ok(()) }
            "pwcfr" => { validate_finite_parameter("PWCFR", value)?; self.params.p710 = value; self.mark_param_given(710); Ok(()) }
            "plwcfr" => { validate_finite_parameter("PLWCFR", value)?; self.params.p711 = value; self.mark_param_given(711); Ok(()) }
            "pocfrd" => { validate_finite_parameter("POCFRD", value)?; self.params.p712 = value; self.mark_param_given(712); Ok(()) }
            "plcfrd" => { validate_finite_parameter("PLCFRD", value)?; self.params.p713 = value; self.mark_param_given(713); Ok(()) }
            "pwcfrd" => { validate_finite_parameter("PWCFRD", value)?; self.params.p714 = value; self.mark_param_given(714); Ok(()) }
            "plwcfrd" => { validate_finite_parameter("PLWCFRD", value)?; self.params.p715 = value; self.mark_param_given(715); Ok(()) }
            "pofntexc" => { validate_finite_parameter("POFNTEXC", value)?; self.params.p716 = value; self.mark_param_given(716); Ok(()) }
            "plfntexc" => { validate_finite_parameter("PLFNTEXC", value)?; self.params.p717 = value; self.mark_param_given(717); Ok(()) }
            "pwfntexc" => { validate_finite_parameter("PWFNTEXC", value)?; self.params.p718 = value; self.mark_param_given(718); Ok(()) }
            "plwfntexc" => { validate_finite_parameter("PLWFNTEXC", value)?; self.params.p719 = value; self.mark_param_given(719); Ok(()) }
            "ponfa" => { validate_finite_parameter("PONFA", value)?; self.params.p720 = value; self.mark_param_given(720); Ok(()) }
            "plnfa" => { validate_finite_parameter("PLNFA", value)?; self.params.p721 = value; self.mark_param_given(721); Ok(()) }
            "pwnfa" => { validate_finite_parameter("PWNFA", value)?; self.params.p722 = value; self.mark_param_given(722); Ok(()) }
            "plwnfa" => { validate_finite_parameter("PLWNFA", value)?; self.params.p723 = value; self.mark_param_given(723); Ok(()) }
            "ponfb" => { validate_finite_parameter("PONFB", value)?; self.params.p724 = value; self.mark_param_given(724); Ok(()) }
            "plnfb" => { validate_finite_parameter("PLNFB", value)?; self.params.p725 = value; self.mark_param_given(725); Ok(()) }
            "pwnfb" => { validate_finite_parameter("PWNFB", value)?; self.params.p726 = value; self.mark_param_given(726); Ok(()) }
            "plwnfb" => { validate_finite_parameter("PLWNFB", value)?; self.params.p727 = value; self.mark_param_given(727); Ok(()) }
            "ponfc" => { validate_finite_parameter("PONFC", value)?; self.params.p728 = value; self.mark_param_given(728); Ok(()) }
            "plnfc" => { validate_finite_parameter("PLNFC", value)?; self.params.p729 = value; self.mark_param_given(729); Ok(()) }
            "pwnfc" => { validate_finite_parameter("PWNFC", value)?; self.params.p730 = value; self.mark_param_given(730); Ok(()) }
            "plwnfc" => { validate_finite_parameter("PLWNFC", value)?; self.params.p731 = value; self.mark_param_given(731); Ok(()) }
            "povfbedge" => { validate_finite_parameter("POVFBEDGE", value)?; self.params.p732 = value; self.mark_param_given(732); Ok(()) }
            "plvfbedge" => { validate_finite_parameter("PLVFBEDGE", value)?; self.params.p733 = value; self.mark_param_given(733); Ok(()) }
            "pwvfbedge" => { validate_finite_parameter("PWVFBEDGE", value)?; self.params.p734 = value; self.mark_param_given(734); Ok(()) }
            "plwvfbedge" => { validate_finite_parameter("PLWVFBEDGE", value)?; self.params.p735 = value; self.mark_param_given(735); Ok(()) }
            "postvfbedge" => { validate_finite_parameter("POSTVFBEDGE", value)?; self.params.p736 = value; self.mark_param_given(736); Ok(()) }
            "plstvfbedge" => { validate_finite_parameter("PLSTVFBEDGE", value)?; self.params.p737 = value; self.mark_param_given(737); Ok(()) }
            "pwstvfbedge" => { validate_finite_parameter("PWSTVFBEDGE", value)?; self.params.p738 = value; self.mark_param_given(738); Ok(()) }
            "plwstvfbedge" => { validate_finite_parameter("PLWSTVFBEDGE", value)?; self.params.p739 = value; self.mark_param_given(739); Ok(()) }
            "podphibedge" => { validate_finite_parameter("PODPHIBEDGE", value)?; self.params.p740 = value; self.mark_param_given(740); Ok(()) }
            "pldphibedge" => { validate_finite_parameter("PLDPHIBEDGE", value)?; self.params.p741 = value; self.mark_param_given(741); Ok(()) }
            "pwdphibedge" => { validate_finite_parameter("PWDPHIBEDGE", value)?; self.params.p742 = value; self.mark_param_given(742); Ok(()) }
            "plwdphibedge" => { validate_finite_parameter("PLWDPHIBEDGE", value)?; self.params.p743 = value; self.mark_param_given(743); Ok(()) }
            "poneffedge" => { validate_finite_parameter("PONEFFEDGE", value)?; self.params.p744 = value; self.mark_param_given(744); Ok(()) }
            "plneffedge" => { validate_finite_parameter("PLNEFFEDGE", value)?; self.params.p745 = value; self.mark_param_given(745); Ok(()) }
            "pwneffedge" => { validate_finite_parameter("PWNEFFEDGE", value)?; self.params.p746 = value; self.mark_param_given(746); Ok(()) }
            "plwneffedge" => { validate_finite_parameter("PLWNEFFEDGE", value)?; self.params.p747 = value; self.mark_param_given(747); Ok(()) }
            "poctedge" => { validate_finite_parameter("POCTEDGE", value)?; self.params.p748 = value; self.mark_param_given(748); Ok(()) }
            "plctedge" => { validate_finite_parameter("PLCTEDGE", value)?; self.params.p749 = value; self.mark_param_given(749); Ok(()) }
            "pwctedge" => { validate_finite_parameter("PWCTEDGE", value)?; self.params.p750 = value; self.mark_param_given(750); Ok(()) }
            "plwctedge" => { validate_finite_parameter("PLWCTEDGE", value)?; self.params.p751 = value; self.mark_param_given(751); Ok(()) }
            "pobetnedge" => { validate_finite_parameter("POBETNEDGE", value)?; self.params.p752 = value; self.mark_param_given(752); Ok(()) }
            "plbetnedge" => { validate_finite_parameter("PLBETNEDGE", value)?; self.params.p753 = value; self.mark_param_given(753); Ok(()) }
            "pwbetnedge" => { validate_finite_parameter("PWBETNEDGE", value)?; self.params.p754 = value; self.mark_param_given(754); Ok(()) }
            "plwbetnedge" => { validate_finite_parameter("PLWBETNEDGE", value)?; self.params.p755 = value; self.mark_param_given(755); Ok(()) }
            "postbetedge" => { validate_finite_parameter("POSTBETEDGE", value)?; self.params.p756 = value; self.mark_param_given(756); Ok(()) }
            "plstbetedge" => { validate_finite_parameter("PLSTBETEDGE", value)?; self.params.p757 = value; self.mark_param_given(757); Ok(()) }
            "pwstbetedge" => { validate_finite_parameter("PWSTBETEDGE", value)?; self.params.p758 = value; self.mark_param_given(758); Ok(()) }
            "plwstbetedge" => { validate_finite_parameter("PLWSTBETEDGE", value)?; self.params.p759 = value; self.mark_param_given(759); Ok(()) }
            "popsceedge" => { validate_finite_parameter("POPSCEEDGE", value)?; self.params.p760 = value; self.mark_param_given(760); Ok(()) }
            "plpsceedge" => { validate_finite_parameter("PLPSCEEDGE", value)?; self.params.p761 = value; self.mark_param_given(761); Ok(()) }
            "pwpsceedge" => { validate_finite_parameter("PWPSCEEDGE", value)?; self.params.p762 = value; self.mark_param_given(762); Ok(()) }
            "plwpsceedge" => { validate_finite_parameter("PLWPSCEEDGE", value)?; self.params.p763 = value; self.mark_param_given(763); Ok(()) }
            "popscebedge" => { validate_finite_parameter("POPSCEBEDGE", value)?; self.params.p764 = value; self.mark_param_given(764); Ok(()) }
            "plpscebedge" => { validate_finite_parameter("PLPSCEBEDGE", value)?; self.params.p765 = value; self.mark_param_given(765); Ok(()) }
            "pwpscebedge" => { validate_finite_parameter("PWPSCEBEDGE", value)?; self.params.p766 = value; self.mark_param_given(766); Ok(()) }
            "plwpscebedge" => { validate_finite_parameter("PLWPSCEBEDGE", value)?; self.params.p767 = value; self.mark_param_given(767); Ok(()) }
            "popscededge" => { validate_finite_parameter("POPSCEDEDGE", value)?; self.params.p768 = value; self.mark_param_given(768); Ok(()) }
            "plpscededge" => { validate_finite_parameter("PLPSCEDEDGE", value)?; self.params.p769 = value; self.mark_param_given(769); Ok(()) }
            "pwpscededge" => { validate_finite_parameter("PWPSCEDEDGE", value)?; self.params.p770 = value; self.mark_param_given(770); Ok(()) }
            "plwpscededge" => { validate_finite_parameter("PLWPSCEDEDGE", value)?; self.params.p771 = value; self.mark_param_given(771); Ok(()) }
            "pocfedge" => { validate_finite_parameter("POCFEDGE", value)?; self.params.p772 = value; self.mark_param_given(772); Ok(()) }
            "plcfedge" => { validate_finite_parameter("PLCFEDGE", value)?; self.params.p773 = value; self.mark_param_given(773); Ok(()) }
            "pwcfedge" => { validate_finite_parameter("PWCFEDGE", value)?; self.params.p774 = value; self.mark_param_given(774); Ok(()) }
            "plwcfedge" => { validate_finite_parameter("PLWCFEDGE", value)?; self.params.p775 = value; self.mark_param_given(775); Ok(()) }
            "pocfbedge" => { validate_finite_parameter("POCFBEDGE", value)?; self.params.p776 = value; self.mark_param_given(776); Ok(()) }
            "plcfbedge" => { validate_finite_parameter("PLCFBEDGE", value)?; self.params.p777 = value; self.mark_param_given(777); Ok(()) }
            "pwcfbedge" => { validate_finite_parameter("PWCFBEDGE", value)?; self.params.p778 = value; self.mark_param_given(778); Ok(()) }
            "plwcfbedge" => { validate_finite_parameter("PLWCFBEDGE", value)?; self.params.p779 = value; self.mark_param_given(779); Ok(()) }
            "pocfdedge" => { validate_finite_parameter("POCFDEDGE", value)?; self.params.p780 = value; self.mark_param_given(780); Ok(()) }
            "plcfdedge" => { validate_finite_parameter("PLCFDEDGE", value)?; self.params.p781 = value; self.mark_param_given(781); Ok(()) }
            "pwcfdedge" => { validate_finite_parameter("PWCFDEDGE", value)?; self.params.p782 = value; self.mark_param_given(782); Ok(()) }
            "plwcfdedge" => { validate_finite_parameter("PLWCFDEDGE", value)?; self.params.p783 = value; self.mark_param_given(783); Ok(()) }
            "ponfaedge" => { validate_finite_parameter("PONFAEDGE", value)?; self.params.p784 = value; self.mark_param_given(784); Ok(()) }
            "plnfaedge" => { validate_finite_parameter("PLNFAEDGE", value)?; self.params.p785 = value; self.mark_param_given(785); Ok(()) }
            "pwnfaedge" => { validate_finite_parameter("PWNFAEDGE", value)?; self.params.p786 = value; self.mark_param_given(786); Ok(()) }
            "plwnfaedge" => { validate_finite_parameter("PLWNFAEDGE", value)?; self.params.p787 = value; self.mark_param_given(787); Ok(()) }
            "ponfbedge" => { validate_finite_parameter("PONFBEDGE", value)?; self.params.p788 = value; self.mark_param_given(788); Ok(()) }
            "plnfbedge" => { validate_finite_parameter("PLNFBEDGE", value)?; self.params.p789 = value; self.mark_param_given(789); Ok(()) }
            "pwnfbedge" => { validate_finite_parameter("PWNFBEDGE", value)?; self.params.p790 = value; self.mark_param_given(790); Ok(()) }
            "plwnfbedge" => { validate_finite_parameter("PLWNFBEDGE", value)?; self.params.p791 = value; self.mark_param_given(791); Ok(()) }
            "ponfcedge" => { validate_finite_parameter("PONFCEDGE", value)?; self.params.p792 = value; self.mark_param_given(792); Ok(()) }
            "plnfcedge" => { validate_finite_parameter("PLNFCEDGE", value)?; self.params.p793 = value; self.mark_param_given(793); Ok(()) }
            "pwnfcedge" => { validate_finite_parameter("PWNFCEDGE", value)?; self.params.p794 = value; self.mark_param_given(794); Ok(()) }
            "plwnfcedge" => { validate_finite_parameter("PLWNFCEDGE", value)?; self.params.p795 = value; self.mark_param_given(795); Ok(()) }
            "porth" => { validate_finite_parameter("PORTH", value)?; self.params.p796 = value; self.mark_param_given(796); Ok(()) }
            "plrth" => { validate_finite_parameter("PLRTH", value)?; self.params.p797 = value; self.mark_param_given(797); Ok(()) }
            "pwrth" => { validate_finite_parameter("PWRTH", value)?; self.params.p798 = value; self.mark_param_given(798); Ok(()) }
            "plwrth" => { validate_finite_parameter("PLWRTH", value)?; self.params.p799 = value; self.mark_param_given(799); Ok(()) }
            "pocth" => { validate_finite_parameter("POCTH", value)?; self.params.p800 = value; self.mark_param_given(800); Ok(()) }
            "plcth" => { validate_finite_parameter("PLCTH", value)?; self.params.p801 = value; self.mark_param_given(801); Ok(()) }
            "pwcth" => { validate_finite_parameter("PWCTH", value)?; self.params.p802 = value; self.mark_param_given(802); Ok(()) }
            "plwcth" => { validate_finite_parameter("PLWCTH", value)?; self.params.p803 = value; self.mark_param_given(803); Ok(()) }
            "postrth" => { validate_finite_parameter("POSTRTH", value)?; self.params.p804 = value; self.mark_param_given(804); Ok(()) }
            "plstrth" => { validate_finite_parameter("PLSTRTH", value)?; self.params.p805 = value; self.mark_param_given(805); Ok(()) }
            "pwstrth" => { validate_finite_parameter("PWSTRTH", value)?; self.params.p806 = value; self.mark_param_given(806); Ok(()) }
            "plwstrth" => { validate_finite_parameter("PLWSTRTH", value)?; self.params.p807 = value; self.mark_param_given(807); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p808 = value; self.mark_param_given(808); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p809 = value; self.mark_param_given(809); Ok(()) }
            "wlod" => { validate_finite_parameter("WLOD", value)?; self.params.p810 = value; self.mark_param_given(810); Ok(()) }
            "kuo" => { validate_finite_parameter("KUO", value)?; self.params.p811 = value; self.mark_param_given(811); Ok(()) }
            "kvsat" => { validate_parameter("KVSAT", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p812 = value; self.mark_param_given(812); Ok(()) }
            "kvsatac" => { validate_parameter("KVSATAC", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p813 = value; self.mark_param_given(813); Ok(()) }
            "tkuo" => { validate_finite_parameter("TKUO", value)?; self.params.p814 = value; self.mark_param_given(814); Ok(()) }
            "lkuo" => { validate_finite_parameter("LKUO", value)?; self.params.p815 = value; self.mark_param_given(815); Ok(()) }
            "wkuo" => { validate_finite_parameter("WKUO", value)?; self.params.p816 = value; self.mark_param_given(816); Ok(()) }
            "pkuo" => { validate_finite_parameter("PKUO", value)?; self.params.p817 = value; self.mark_param_given(817); Ok(()) }
            "llodkuo" => { validate_parameter("LLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p818 = value; self.mark_param_given(818); Ok(()) }
            "wlodkuo" => { validate_parameter("WLODKUO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p819 = value; self.mark_param_given(819); Ok(()) }
            "kvtho" => { validate_finite_parameter("KVTHO", value)?; self.params.p820 = value; self.mark_param_given(820); Ok(()) }
            "lkvtho" => { validate_finite_parameter("LKVTHO", value)?; self.params.p821 = value; self.mark_param_given(821); Ok(()) }
            "wkvtho" => { validate_finite_parameter("WKVTHO", value)?; self.params.p822 = value; self.mark_param_given(822); Ok(()) }
            "pkvtho" => { validate_finite_parameter("PKVTHO", value)?; self.params.p823 = value; self.mark_param_given(823); Ok(()) }
            "llodvth" => { validate_parameter("LLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p824 = value; self.mark_param_given(824); Ok(()) }
            "wlodvth" => { validate_parameter("WLODVTH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p825 = value; self.mark_param_given(825); Ok(()) }
            "stetao" => { validate_finite_parameter("STETAO", value)?; self.params.p826 = value; self.mark_param_given(826); Ok(()) }
            "lodetao" => { validate_parameter("LODETAO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p827 = value; self.mark_param_given(827); Ok(()) }
            "scref" => { validate_parameter("SCREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p828 = value; self.mark_param_given(828); Ok(()) }
            "web" => { validate_finite_parameter("WEB", value)?; self.params.p829 = value; self.mark_param_given(829); Ok(()) }
            "wec" => { validate_finite_parameter("WEC", value)?; self.params.p830 = value; self.mark_param_given(830); Ok(()) }
            "kvthoweo" => { validate_finite_parameter("KVTHOWEO", value)?; self.params.p831 = value; self.mark_param_given(831); Ok(()) }
            "kvthowel" => { validate_finite_parameter("KVTHOWEL", value)?; self.params.p832 = value; self.mark_param_given(832); Ok(()) }
            "kvthowew" => { validate_finite_parameter("KVTHOWEW", value)?; self.params.p833 = value; self.mark_param_given(833); Ok(()) }
            "kvthowelw" => { validate_finite_parameter("KVTHOWELW", value)?; self.params.p834 = value; self.mark_param_given(834); Ok(()) }
            "kuoweo" => { validate_finite_parameter("KUOWEO", value)?; self.params.p835 = value; self.mark_param_given(835); Ok(()) }
            "kuowel" => { validate_finite_parameter("KUOWEL", value)?; self.params.p836 = value; self.mark_param_given(836); Ok(()) }
            "kuowew" => { validate_finite_parameter("KUOWEW", value)?; self.params.p837 = value; self.mark_param_given(837); Ok(()) }
            "kuowelw" => { validate_finite_parameter("KUOWELW", value)?; self.params.p838 = value; self.mark_param_given(838); Ok(()) }
            "imax" => { validate_parameter("IMAX", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p839 = value; self.mark_param_given(839); Ok(()) }
            "trj" => { validate_parameter("TRJ", value, Some((-250.0, "-250.0")), false, None, true, &[])?; self.params.p840 = value; self.mark_param_given(840); Ok(()) }
            "frev" => { validate_parameter("FREV", value, Some((10.0, "10.0")), false, Some((10000000000.0, "10000000000.0")), false, &[])?; self.params.p841 = value; self.mark_param_given(841); Ok(()) }
            "cjorbot" => { validate_parameter("CJORBOT", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p842 = value; self.mark_param_given(842); Ok(()) }
            "cjorsti" => { validate_parameter("CJORSTI", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p843 = value; self.mark_param_given(843); Ok(()) }
            "cjorgat" => { validate_parameter("CJORGAT", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p844 = value; self.mark_param_given(844); Ok(()) }
            "vbirbot" => { validate_parameter("VBIRBOT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p845 = value; self.mark_param_given(845); Ok(()) }
            "vbirsti" => { validate_parameter("VBIRSTI", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p846 = value; self.mark_param_given(846); Ok(()) }
            "vbirgat" => { validate_parameter("VBIRGAT", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p847 = value; self.mark_param_given(847); Ok(()) }
            "pbot" => { validate_parameter("PBOT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p848 = value; self.mark_param_given(848); Ok(()) }
            "psti" => { validate_parameter("PSTI", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p849 = value; self.mark_param_given(849); Ok(()) }
            "pgat" => { validate_parameter("PGAT", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p850 = value; self.mark_param_given(850); Ok(()) }
            "phigbot" => { validate_finite_parameter("PHIGBOT", value)?; self.params.p851 = value; self.mark_param_given(851); Ok(()) }
            "phigsti" => { validate_finite_parameter("PHIGSTI", value)?; self.params.p852 = value; self.mark_param_given(852); Ok(()) }
            "phiggat" => { validate_finite_parameter("PHIGGAT", value)?; self.params.p853 = value; self.mark_param_given(853); Ok(()) }
            "idsatrbot" => { validate_parameter("IDSATRBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p854 = value; self.mark_param_given(854); Ok(()) }
            "idsatrsti" => { validate_parameter("IDSATRSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p855 = value; self.mark_param_given(855); Ok(()) }
            "idsatrgat" => { validate_parameter("IDSATRGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p856 = value; self.mark_param_given(856); Ok(()) }
            "csrhbot" => { validate_parameter("CSRHBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p857 = value; self.mark_param_given(857); Ok(()) }
            "csrhsti" => { validate_parameter("CSRHSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p858 = value; self.mark_param_given(858); Ok(()) }
            "csrhgat" => { validate_parameter("CSRHGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p859 = value; self.mark_param_given(859); Ok(()) }
            "xjunsti" => { validate_parameter("XJUNSTI", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p860 = value; self.mark_param_given(860); Ok(()) }
            "xjungat" => { validate_parameter("XJUNGAT", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p861 = value; self.mark_param_given(861); Ok(()) }
            "ctatbot" => { validate_parameter("CTATBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p862 = value; self.mark_param_given(862); Ok(()) }
            "ctatsti" => { validate_parameter("CTATSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p863 = value; self.mark_param_given(863); Ok(()) }
            "ctatgat" => { validate_parameter("CTATGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p864 = value; self.mark_param_given(864); Ok(()) }
            "mefftatbot" => { validate_parameter("MEFFTATBOT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p865 = value; self.mark_param_given(865); Ok(()) }
            "mefftatsti" => { validate_parameter("MEFFTATSTI", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p866 = value; self.mark_param_given(866); Ok(()) }
            "mefftatgat" => { validate_parameter("MEFFTATGAT", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p867 = value; self.mark_param_given(867); Ok(()) }
            "cbbtbot" => { validate_parameter("CBBTBOT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p868 = value; self.mark_param_given(868); Ok(()) }
            "cbbtsti" => { validate_parameter("CBBTSTI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p869 = value; self.mark_param_given(869); Ok(()) }
            "cbbtgat" => { validate_parameter("CBBTGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p870 = value; self.mark_param_given(870); Ok(()) }
            "fbbtrbot" => { validate_finite_parameter("FBBTRBOT", value)?; self.params.p871 = value; self.mark_param_given(871); Ok(()) }
            "fbbtrsti" => { validate_finite_parameter("FBBTRSTI", value)?; self.params.p872 = value; self.mark_param_given(872); Ok(()) }
            "fbbtrgat" => { validate_finite_parameter("FBBTRGAT", value)?; self.params.p873 = value; self.mark_param_given(873); Ok(()) }
            "stfbbtbot" => { validate_finite_parameter("STFBBTBOT", value)?; self.params.p874 = value; self.mark_param_given(874); Ok(()) }
            "stfbbtsti" => { validate_finite_parameter("STFBBTSTI", value)?; self.params.p875 = value; self.mark_param_given(875); Ok(()) }
            "stfbbtgat" => { validate_finite_parameter("STFBBTGAT", value)?; self.params.p876 = value; self.mark_param_given(876); Ok(()) }
            "vbrbot" => { validate_parameter("VBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p877 = value; self.mark_param_given(877); Ok(()) }
            "vbrsti" => { validate_parameter("VBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p878 = value; self.mark_param_given(878); Ok(()) }
            "vbrgat" => { validate_parameter("VBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p879 = value; self.mark_param_given(879); Ok(()) }
            "pbrbot" => { validate_parameter("PBRBOT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p880 = value; self.mark_param_given(880); Ok(()) }
            "pbrsti" => { validate_parameter("PBRSTI", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p881 = value; self.mark_param_given(881); Ok(()) }
            "pbrgat" => { validate_parameter("PBRGAT", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p882 = value; self.mark_param_given(882); Ok(()) }
            "fcjorgat2" => { validate_parameter("FCJORGAT2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p883 = value; self.mark_param_given(883); Ok(()) }
            "fvbirgat2" => { validate_parameter("FVBIRGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p884 = value; self.mark_param_given(884); Ok(()) }
            "fpgat2" => { validate_parameter("FPGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p885 = value; self.mark_param_given(885); Ok(()) }
            "fphiggat2" => { validate_parameter("FPHIGGAT2", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p886 = value; self.mark_param_given(886); Ok(()) }
            "vtrgat" => { validate_parameter("VTRGAT", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p887 = value; self.mark_param_given(887); Ok(()) }
            "anugat" => { validate_parameter("ANUGAT", value, Some((0.001, "0.001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p888 = value; self.mark_param_given(888); Ok(()) }
            "advbrgat" => { validate_parameter("ADVBRGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p889 = value; self.mark_param_given(889); Ok(()) }
            "bdvbrgat" => { validate_parameter("BDVBRGAT", value, Some((0.2, "0.2")), false, None, true, &[])?; self.params.p890 = value; self.mark_param_given(890); Ok(()) }
            "adbbtgat" => { validate_parameter("ADBBTGAT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p891 = value; self.mark_param_given(891); Ok(()) }
            "bdbbtgat" => { validate_parameter("BDBBTGAT", value, Some((0.2, "0.2")), false, None, true, &[])?; self.params.p892 = value; self.mark_param_given(892); Ok(()) }
            "cjorbotd" => { validate_parameter("CJORBOTD", value, Some((1e-12, "1e-12")), false, None, true, &[])?; self.params.p893 = value; self.mark_param_given(893); Ok(()) }
            "cjorstid" => { validate_parameter("CJORSTID", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p894 = value; self.mark_param_given(894); Ok(()) }
            "cjorgatd" => { validate_parameter("CJORGATD", value, Some((1e-18, "1e-18")), false, None, true, &[])?; self.params.p895 = value; self.mark_param_given(895); Ok(()) }
            "vbirbotd" => { validate_parameter("VBIRBOTD", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p896 = value; self.mark_param_given(896); Ok(()) }
            "vbirstid" => { validate_parameter("VBIRSTID", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p897 = value; self.mark_param_given(897); Ok(()) }
            "vbirgatd" => { validate_parameter("VBIRGATD", value, Some((0.05, "0.05")), false, None, true, &[])?; self.params.p898 = value; self.mark_param_given(898); Ok(()) }
            "pbotd" => { validate_parameter("PBOTD", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p899 = value; self.mark_param_given(899); Ok(()) }
            "pstid" => { validate_parameter("PSTID", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p900 = value; self.mark_param_given(900); Ok(()) }
            "pgatd" => { validate_parameter("PGATD", value, Some((0.05, "0.05")), false, Some((0.95, "0.95")), false, &[])?; self.params.p901 = value; self.mark_param_given(901); Ok(()) }
            "phigbotd" => { validate_finite_parameter("PHIGBOTD", value)?; self.params.p902 = value; self.mark_param_given(902); Ok(()) }
            "phigstid" => { validate_finite_parameter("PHIGSTID", value)?; self.params.p903 = value; self.mark_param_given(903); Ok(()) }
            "phiggatd" => { validate_finite_parameter("PHIGGATD", value)?; self.params.p904 = value; self.mark_param_given(904); Ok(()) }
            "idsatrbotd" => { validate_parameter("IDSATRBOTD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p905 = value; self.mark_param_given(905); Ok(()) }
            "idsatrstid" => { validate_parameter("IDSATRSTID", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p906 = value; self.mark_param_given(906); Ok(()) }
            "idsatrgatd" => { validate_parameter("IDSATRGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p907 = value; self.mark_param_given(907); Ok(()) }
            "csrhbotd" => { validate_parameter("CSRHBOTD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p908 = value; self.mark_param_given(908); Ok(()) }
            "csrhstid" => { validate_parameter("CSRHSTID", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p909 = value; self.mark_param_given(909); Ok(()) }
            "csrhgatd" => { validate_parameter("CSRHGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p910 = value; self.mark_param_given(910); Ok(()) }
            "xjunstid" => { validate_parameter("XJUNSTID", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p911 = value; self.mark_param_given(911); Ok(()) }
            "xjungatd" => { validate_parameter("XJUNGATD", value, Some((1e-9, "1e-9")), false, None, true, &[])?; self.params.p912 = value; self.mark_param_given(912); Ok(()) }
            "ctatbotd" => { validate_parameter("CTATBOTD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p913 = value; self.mark_param_given(913); Ok(()) }
            "ctatstid" => { validate_parameter("CTATSTID", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p914 = value; self.mark_param_given(914); Ok(()) }
            "ctatgatd" => { validate_parameter("CTATGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p915 = value; self.mark_param_given(915); Ok(()) }
            "mefftatbotd" => { validate_parameter("MEFFTATBOTD", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p916 = value; self.mark_param_given(916); Ok(()) }
            "mefftatstid" => { validate_parameter("MEFFTATSTID", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p917 = value; self.mark_param_given(917); Ok(()) }
            "mefftatgatd" => { validate_parameter("MEFFTATGATD", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p918 = value; self.mark_param_given(918); Ok(()) }
            "cbbtbotd" => { validate_parameter("CBBTBOTD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p919 = value; self.mark_param_given(919); Ok(()) }
            "cbbtstid" => { validate_parameter("CBBTSTID", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p920 = value; self.mark_param_given(920); Ok(()) }
            "cbbtgatd" => { validate_parameter("CBBTGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p921 = value; self.mark_param_given(921); Ok(()) }
            "fbbtrbotd" => { validate_finite_parameter("FBBTRBOTD", value)?; self.params.p922 = value; self.mark_param_given(922); Ok(()) }
            "fbbtrstid" => { validate_finite_parameter("FBBTRSTID", value)?; self.params.p923 = value; self.mark_param_given(923); Ok(()) }
            "fbbtrgatd" => { validate_finite_parameter("FBBTRGATD", value)?; self.params.p924 = value; self.mark_param_given(924); Ok(()) }
            "stfbbtbotd" => { validate_finite_parameter("STFBBTBOTD", value)?; self.params.p925 = value; self.mark_param_given(925); Ok(()) }
            "stfbbtstid" => { validate_finite_parameter("STFBBTSTID", value)?; self.params.p926 = value; self.mark_param_given(926); Ok(()) }
            "stfbbtgatd" => { validate_finite_parameter("STFBBTGATD", value)?; self.params.p927 = value; self.mark_param_given(927); Ok(()) }
            "vbrbotd" => { validate_parameter("VBRBOTD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p928 = value; self.mark_param_given(928); Ok(()) }
            "vbrstid" => { validate_parameter("VBRSTID", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p929 = value; self.mark_param_given(929); Ok(()) }
            "vbrgatd" => { validate_parameter("VBRGATD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p930 = value; self.mark_param_given(930); Ok(()) }
            "pbrbotd" => { validate_parameter("PBRBOTD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p931 = value; self.mark_param_given(931); Ok(()) }
            "pbrstid" => { validate_parameter("PBRSTID", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p932 = value; self.mark_param_given(932); Ok(()) }
            "pbrgatd" => { validate_parameter("PBRGATD", value, Some((0.1, "0.1")), false, None, true, &[])?; self.params.p933 = value; self.mark_param_given(933); Ok(()) }
            "fcjorgat2d" => { validate_parameter("FCJORGAT2D", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p934 = value; self.mark_param_given(934); Ok(()) }
            "fvbirgat2d" => { validate_parameter("FVBIRGAT2D", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p935 = value; self.mark_param_given(935); Ok(()) }
            "fpgat2d" => { validate_parameter("FPGAT2D", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p936 = value; self.mark_param_given(936); Ok(()) }
            "fphiggat2d" => { validate_parameter("FPHIGGAT2D", value, Some((0.01, "0.01")), false, None, true, &[])?; self.params.p937 = value; self.mark_param_given(937); Ok(()) }
            "vtrgatd" => { validate_parameter("VTRGATD", value, Some((-100.0, "-100.0")), false, Some((100.0, "100.0")), false, &[])?; self.params.p938 = value; self.mark_param_given(938); Ok(()) }
            "anugatd" => { validate_parameter("ANUGATD", value, Some((0.001, "0.001")), false, Some((10.0, "10.0")), false, &[])?; self.params.p939 = value; self.mark_param_given(939); Ok(()) }
            "advbrgatd" => { validate_parameter("ADVBRGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p940 = value; self.mark_param_given(940); Ok(()) }
            "bdvbrgatd" => { validate_parameter("BDVBRGATD", value, Some((0.2, "0.2")), false, None, true, &[])?; self.params.p941 = value; self.mark_param_given(941); Ok(()) }
            "adbbtgatd" => { validate_parameter("ADBBTGATD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p942 = value; self.mark_param_given(942); Ok(()) }
            "bdbbtgatd" => { validate_parameter("BDBBTGATD", value, Some((0.2, "0.2")), false, None, true, &[])?; self.params.p943 = value; self.mark_param_given(943); Ok(()) }
            "swjunexp" => { validate_parameter("SWJUNEXP", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p944 = value; self.mark_param_given(944); Ok(()) }
            "vjunref" => { validate_parameter("VJUNREF", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p945 = value; self.mark_param_given(945); Ok(()) }
            "fjunq" => { validate_parameter("FJUNQ", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p946 = value; self.mark_param_given(946); Ok(()) }
            "vjunrefd" => { validate_parameter("VJUNREFD", value, Some((0.5, "0.5")), false, None, true, &[])?; self.params.p947 = value; self.mark_param_given(947); Ok(()) }
            "fjunqd" => { validate_parameter("FJUNQD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p948 = value; self.mark_param_given(948); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'PSP104TVA'", name)),
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
