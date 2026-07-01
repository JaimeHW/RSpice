#![allow(dead_code, unused_parens, unused_variables)]

use crate::device::veriloga_generated::GeneratedDdtCoefficients;

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
            params.p0 = 0.0;
            params.p1 = 5e-6;
            params.p2 = 5e-6;
            params.p3 = 1.0;
            params.p4 = 0.0;
            params.p5 = 0.0;
            params.p6 = 0.0;
            params.p7 = 0.0;
            params.p8 = 0.0;
            params.p9 = 0.0;
            params.p10 = 0.0;
            params.p11 = 1.0;
            params.p12 = 1.0;
            params.p13 = 0.0;
            params.p14 = 0.0;
            params.p15 = 1e-5;
            params.p16 = 1.0;
            params.p17 = 1.0;
            params.p18 = 50.0;
            params.p19 = 50.0;
            params.p20 = 0.0;
            params.p21 = 0.0;
            params.p22 = 0.0;
            params.p23 = 1.0;
            params.p24 = 0.0;
            params.p25 = 0.0;
            params.p26 = 0.0;
            params.p27 = 0.0;
            params.p28 = params.p26;
            validate_parameter("AGBCPD", params.p28, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p29 = 0.0;
            params.p30 = 1.0;
            params.p31 = 1.0;
            params.p32 = 1.0;
            params.p33 = 1.0;
            params.p34 = params.p32;
            validate_parameter("MULT_FN", params.p34, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p35 = 0.0;
            params.p36 = 0.0;
            params.p37 = 1.0;
            params.p38 = 4.7;
            params.p39 = 0.0;
            params.p40 = 0.0;
            params.p41 = 0.0;
            params.p42 = if (params.p38 >= 4.2) { 1.0 } else { 0.0 };
            validate_parameter("VGSTCVMOD", params.p42, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p43 = 0.0;
            params.p44 = 0.0;
            params.p45 = 1e-8;
            params.p46 = 3.9;
            params.p47 = 11.7;
            params.p48 = 14500000000.0;
            params.p49 = 1.16;
            params.p50 = 0.000702;
            params.p51 = 1108.0;
            params.p52 = 4.05;
            params.p53 = 4.05;
            params.p54 = 1.0;
            params.p55 = 10.0;
            params.p56 = if (params.p37 == 1.0) { 1.5 } else { (-1.5) };
            validate_finite_parameter("VDDEOT", params.p56).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p57 = 300.15;
            params.p58 = 1.0;
            params.p59 = 1.0;
            params.p60 = 11.7;
            params.p61 = 2.0;
            params.p62 = 1.0;
            params.p63 = 0.0;
            params.p64 = 1.0;
            params.p65 = 1.0;
            params.p66 = 1e-8;
            params.p67 = params.p66;
            validate_parameter("TOXM", params.p67, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p68 = 0.0;
            params.p69 = 0.00024;
            params.p70 = 0.0;
            params.p71 = 0.0;
            params.p72 = 0.0;
            params.p73 = 1.0;
            params.p74 = 80000.0;
            params.p75 = 33000.0;
            params.p76 = 1.0;
            params.p77 = 0.0;
            params.p78 = 0.0;
            params.p79 = 1.0;
            params.p80 = -0.6;
            params.p81 = 6e16;
            params.p82 = 1.7e17;
            params.p83 = 0.0;
            params.p84 = 1e20;
            params.p85 = 0.0;
            params.p86 = 0.0;
            params.p87 = 0.0;
            params.p88 = -3.0;
            params.p89 = 1.55e-7;
            params.p90 = 0.53;
            params.p91 = -0.11;
            params.p92 = 0.0;
            params.p93 = 0.022;
            params.p94 = -0.0186;
            params.p95 = 0.0;
            params.p96 = 0.0;
            params.p97 = 2.5e-6;
            params.p98 = 0.0;
            params.p99 = 2.2;
            params.p100 = 0.53;
            params.p101 = -0.032;
            params.p102 = 0.0;
            params.p103 = 5300000.0;
            params.p104 = -0.032;
            params.p105 = 0.56;
            params.p106 = params.p105;
            validate_finite_parameter("DSUB", params.p106).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p107 = if (params.p37 == 1.0) { 0.7 } else { (-0.7) };
            validate_finite_parameter("VTHO", params.p107).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p108 = params.p107;
            validate_finite_parameter("VTH0", params.p108).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p109 = -1.0;
            params.p110 = 2.25e-9;
            params.p111 = 4.31e-9;
            params.p112 = 5.87e-19;
            params.p113 = -7.61e-18;
            params.p114 = if (params.p62 == 3.0) { (-0.0465) } else { (-4.65e-11) };
            validate_finite_parameter("UC", params.p114).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p115 = if (params.p62 == 3.0) { (-0.056) } else { (-5.6e-11) };
            validate_finite_parameter("UC1", params.p115).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p116 = if (params.p37 == 1.0) { 0.067 } else { 0.025 };
            validate_finite_parameter("U0", params.p116).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p117 = if (params.p37 == 1.0) { 1.67 } else { 1.0 };
            validate_finite_parameter("EU", params.p117).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p118 = -1.5;
            params.p119 = if (params.p37 == 1.0) { 1.67 } else { 1.0 };
            validate_finite_parameter("UCS", params.p119).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p120 = -0.004775;
            params.p121 = 0.0;
            params.p122 = 0.0;
            params.p123 = 0.0;
            params.p124 = 0.0;
            params.p125 = -0.08;
            params.p126 = 27.0;
            params.p127 = 0.0;
            params.p128 = 0.0;
            params.p129 = 0.0;
            params.p130 = 0.01;
            params.p131 = 0.0;
            params.p132 = 100.0;
            params.p133 = 50.0;
            params.p134 = 50.0;
            params.p135 = 0.0;
            params.p136 = 0.0;
            params.p137 = 0.0;
            params.p138 = 0.0;
            params.p139 = 0.0;
            params.p140 = 0.0;
            params.p141 = 0.0;
            params.p142 = 0.0;
            params.p143 = 0.0;
            params.p144 = 0.0;
            params.p145 = 0.08;
            params.p146 = -0.07;
            params.p147 = params.p145;
            validate_finite_parameter("ETA0CV", params.p147).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p148 = params.p146;
            validate_finite_parameter("ETABCV", params.p148).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p149 = 1.3;
            params.p150 = 0.39;
            params.p151 = 0.0086;
            params.p152 = 0.0;
            params.p153 = 0.0;
            params.p154 = 3e-7;
            params.p155 = 1e-7;
            params.p156 = 1e-7;
            params.p157 = params.p155;
            validate_parameter("XJ", params.p157, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p158 = 0.0;
            params.p159 = 2300000000.0;
            params.p160 = 0.0;
            params.p161 = 0.5;
            params.p162 = 1.0;
            params.p163 = 0.0;
            params.p164 = 0.0;
            params.p165 = params.p158;
            validate_finite_parameter("AGISL", params.p165).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p166 = params.p159;
            validate_finite_parameter("BGISL", params.p166).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p167 = params.p160;
            validate_finite_parameter("BGISL1", params.p167).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p168 = params.p161;
            validate_finite_parameter("CGISL", params.p168).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p169 = params.p162;
            validate_finite_parameter("RGISL", params.p169).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p170 = params.p163;
            validate_finite_parameter("KGISL", params.p170).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p171 = params.p164;
            validate_finite_parameter("FGISL", params.p171).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p172 = 1.0;
            params.p173 = params.p172;
            validate_parameter("NDIODED", params.p173, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p174 = 1.0;
            params.p175 = params.p174;
            validate_finite_parameter("XDIF", params.p175).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p176 = 1.0;
            params.p177 = 0.0;
            params.p178 = params.p175;
            validate_finite_parameter("XDIFD", params.p178).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p179 = params.p176;
            validate_finite_parameter("XRECD", params.p179).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p180 = params.p177;
            validate_finite_parameter("XTUND", params.p180).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p181 = 0.7;
            params.p182 = params.p181;
            validate_parameter("PBSWGD", params.p182, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p183 = 0.5;
            params.p184 = params.p183;
            validate_finite_parameter("MJSWGD", params.p184).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p185 = 1e-10;
            params.p186 = params.p185;
            validate_parameter("CJSWGD", params.p186, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p187 = 0.0;
            params.p188 = 0.0;
            params.p189 = 0.0;
            params.p190 = 1.0;
            params.p191 = 0.0;
            params.p192 = 0.0;
            params.p193 = 1.0;
            params.p194 = 0.0;
            params.p195 = 0.0;
            params.p196 = 1.0;
            params.p197 = 0.0;
            params.p198 = 0.0;
            params.p199 = 0.0;
            params.p200 = 0.0;
            params.p201 = 0.0;
            params.p202 = 1.0;
            params.p203 = 0.0;
            params.p204 = 0.0;
            params.p205 = 1.0;
            params.p206 = 0.0;
            params.p207 = 0.0;
            params.p208 = 0.0;
            params.p209 = 0.0;
            params.p210 = 0.0;
            params.p211 = 0.0;
            params.p212 = 0.6;
            params.p213 = 0.0;
            params.p214 = 1e-8;
            params.p215 = 0.0;
            params.p216 = params.p197;
            validate_finite_parameter("DWC", params.p216).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p217 = params.p187;
            validate_finite_parameter("DLC", params.p217).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p218 = 0.0;
            params.p219 = if (params.p37 == 1.0) { 6.25e41 } else { 6.188e40 };
            validate_finite_parameter("NOIA", params.p219).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p220 = if (params.p37 == 1.0) { 3.125e26 } else { 1.5e25 };
            validate_finite_parameter("NOIB", params.p220).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p221 = 8750000000.0;
            params.p222 = 1.0;
            params.p223 = 0.0;
            params.p224 = 3.5;
            params.p225 = 0.395;
            params.p226 = 100000.0;
            params.p227 = 1.5;
            params.p228 = 3.5;
            params.p229 = 0.577;
            params.p230 = 0.37;
            params.p231 = 1.0;
            params.p232 = 1e-6;
            params.p233 = 1e-6;
            params.p234 = 0.0;
            params.p235 = 0.0;
            params.p236 = 0.0;
            params.p237 = 0.0;
            params.p238 = 0.0;
            params.p239 = 0.0;
            params.p240 = 0.0;
            params.p241 = 0.0;
            params.p242 = 0.0;
            params.p243 = 0.0;
            params.p244 = 0.0;
            params.p245 = 0.0;
            params.p246 = 0.0;
            params.p247 = 0.0;
            params.p248 = 0.0;
            params.p249 = 0.0;
            params.p250 = 1.0;
            params.p251 = 0.0;
            params.p252 = 1.0;
            params.p253 = params.p251;
            validate_finite_parameter("STETA0CV", params.p253).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p254 = params.p252;
            validate_finite_parameter("LODETA0CV", params.p254).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p255 = 1e-12;
            params.p256 = 2.0;
            params.p257 = 1e-5;
            params.p258 = 0.0;
            params.p259 = 0.0;
            params.p260 = 0.0;
            params.p261 = 0.0;
            params.p262 = 0.0;
            params.p263 = 0.0;
            params.p264 = 0.0;
            params.p265 = 0.0;
            params.p266 = 0.0;
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
            params.p278 = 0.0;
            params.p279 = 0.0;
            params.p280 = 0.0;
            params.p281 = 0.0;
            params.p282 = 1e-20;
            params.p283 = 1.0;
            params.p284 = 0.0;
            params.p285 = 0.0;
            params.p286 = 0.0;
            params.p287 = 0.0;
            params.p288 = 0.0;
            params.p289 = 0.0;
            params.p290 = 0.0;
            params.p291 = 0.0;
            params.p292 = 0.0;
            params.p293 = 0.0;
            params.p294 = 0.0;
            params.p295 = 41000000.0;
            params.p296 = 1.0;
            params.p297 = 1.0;
            params.p298 = 0.0;
            params.p299 = 1.0;
            params.p300 = 0.0;
            params.p301 = 0.0;
            params.p302 = 0.0;
            params.p303 = 0.0;
            params.p304 = 0.0;
            params.p305 = 0.0;
            params.p306 = 0.1;
            params.p307 = 0.9;
            params.p308 = 0.0;
            params.p309 = 0.0;
            params.p310 = 0.5;
            params.p311 = 0.1;
            params.p312 = 0.0;
            params.p313 = 0.0;
            params.p314 = 0.0;
            params.p315 = 0.0;
            params.p316 = 0.0;
            params.p317 = 0.0;
            params.p318 = 0.0;
            params.p319 = 0.4;
            params.p320 = 0.0;
            params.p321 = 10000000.0;
            params.p322 = 10.0;
            params.p323 = params.p322;
            validate_parameter("NTUND", params.p323, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p324 = 2.0;
            params.p325 = params.p324;
            validate_parameter("NRECF0D", params.p325, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p326 = 10.0;
            params.p327 = params.p326;
            validate_parameter("NRECR0D", params.p327, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p328 = 1e-6;
            params.p329 = params.p328;
            validate_parameter("IDBJT", params.p329, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p330 = 0.0;
            params.p331 = params.p330;
            validate_parameter("IDDIF", params.p331, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p332 = 1e-5;
            params.p333 = params.p332;
            validate_parameter("IDREC", params.p333, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p334 = 0.0;
            params.p335 = params.p334;
            validate_parameter("IDTUN", params.p335, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p336 = 2e-6;
            params.p337 = 0.0;
            params.p338 = params.p337;
            validate_finite_parameter("VREC0D", params.p338).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p339 = 0.0;
            params.p340 = params.p339;
            validate_finite_parameter("VTUN0D", params.p340).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p341 = 1.0;
            params.p342 = 2e-7;
            params.p343 = 1.0;
            params.p344 = 10.0;
            params.p345 = 0.0;
            params.p346 = 0.0;
            params.p347 = params.p346;
            validate_finite_parameter("AHLID", params.p347).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p348 = 0.0;
            params.p349 = 0.0;
            params.p350 = 0.0;
            params.p351 = 1e-12;
            params.p352 = -1.0;
            params.p353 = 0.0;
            params.p354 = 0.0;
            params.p355 = 0.0;
            params.p356 = 0.3;
            params.p357 = 0.0;
            params.p358 = 0.0;
            params.p359 = 0.0;
            params.p360 = 0.0;
            params.p361 = 1.0;
            params.p362 = 0.0;
            params.p363 = 0.0;
            params.p364 = params.p362;
            validate_finite_parameter("TCJSWGD", params.p364).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p365 = params.p363;
            validate_finite_parameter("TPBSWGD", params.p365).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p366 = 1.0;
            params.p367 = 15.0;
            params.p368 = 1.0;
            params.p369 = params.p368;
            validate_parameter("NOFF2", params.p369, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p370 = 0.0;
            params.p371 = 1.0;
            params.p372 = 0.0;
            params.p373 = 1.0;
            params.p374 = 0.0;
            params.p375 = 0.0;
            params.p376 = params.p66;
            validate_parameter("TOXQM", params.p376, Some((0.0, "0.0")), true, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p377 = 0.0;
            params.p378 = 1000000000000000.0;
            params.p379 = 1.0;
            params.p380 = 2.5e-9;
            params.p381 = 1.2;
            params.p382 = 0.075;
            params.p383 = 0.35;
            params.p384 = 0.0;
            params.p385 = 0.03;
            params.p386 = 300.0;
            params.p387 = 0.026;
            params.p388 = 0.43;
            params.p389 = 0.0;
            params.p390 = 0.05;
            params.p391 = 17.0;
            params.p392 = 0.043;
            params.p393 = 0.0;
            params.p394 = 0.0054;
            params.p395 = 0.0075;
            params.p396 = 5.0;
            params.p397 = 0.005;
            params.p398 = if (params.p37 == 1.0) { 0.43 } else { 0.31 };
            validate_finite_parameter("AIGC", params.p398).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p399 = 0.0;
            params.p400 = if (params.p37 == 1.0) { 0.054 } else { 0.024 };
            validate_finite_parameter("BIGC", params.p400).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p401 = if (params.p37 == 1.0) { 0.075 } else { 0.03 };
            validate_finite_parameter("CIGC", params.p401).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p402 = if (params.p37 == 1.0) { 0.43 } else { 0.31 };
            validate_finite_parameter("AIGSD", params.p402).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p403 = 0.0;
            params.p404 = if (params.p37 == 1.0) { 0.054 } else { 0.024 };
            validate_finite_parameter("BIGSD", params.p404).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p405 = if (params.p37 == 1.0) { 0.075 } else { 0.03 };
            validate_finite_parameter("CIGSD", params.p405).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p406 = 1.0;
            params.p407 = 1.0;
            params.p408 = 1.0;
            params.p409 = 2.5;
            params.p410 = params.p187;
            validate_finite_parameter("DLCIG", params.p410).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p411 = 0.0;
            params.p412 = 0.5;
            params.p413 = 0.0;
            params.p414 = 1.0;
            params.p415 = 0.0;
            params.p416 = 1.0;
            params.p417 = 0.0;
            params.p418 = 0.0;
            params.p419 = 0.0;
            params.p420 = 0.0;
            params.p421 = 1000.0;
            params.p422 = 12.0;
            params.p423 = 1.0;
            params.p424 = 0.1;
            params.p425 = 1.0;
            params.p426 = 0.0;
            params.p427 = 0.0;
            params.p428 = 0.0;
            params.p429 = 0.0;
            params.p430 = 0.0;
            params.p431 = 0.001;
            validate_parameter("MINR", params.p431, Some((0.0, "0.0")), false, None, true, &[]).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p432 = 0.0;
            params.p433 = 0.0;
            params.p434 = 0.0;
            params.p435 = 0.0;
            params.p436 = 0.0;
            params.p437 = 0.0;
            params.p438 = 0.0;
            params.p439 = 0.0;
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
            params.p460 = 0.0;
            params.p461 = 0.0;
            params.p462 = 0.0;
            params.p463 = 0.0;
            params.p464 = 0.0;
            params.p465 = 0.0;
            params.p466 = 0.0;
            params.p467 = 0.0;
            params.p468 = 0.0;
            params.p469 = 0.0;
            params.p470 = 0.0;
            params.p471 = 0.0;
            params.p472 = 0.0;
            params.p473 = params.p470;
            validate_finite_parameter("LXDIFD", params.p473).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p474 = params.p471;
            validate_finite_parameter("LXRECD", params.p474).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p475 = params.p472;
            validate_finite_parameter("LXTUND", params.p475).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p476 = 0.0;
            params.p477 = 0.0;
            params.p478 = 0.0;
            params.p479 = 0.0;
            params.p480 = 0.0;
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
            params.p508 = 0.0;
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
            params.p536 = 0.0;
            params.p537 = 0.0;
            params.p538 = 0.0;
            params.p539 = 0.0;
            params.p540 = params.p538;
            validate_finite_parameter("LETA0CV", params.p540).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p541 = params.p539;
            validate_finite_parameter("LETABCV", params.p541).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p542 = 0.0;
            params.p543 = 0.0;
            params.p544 = 0.0;
            params.p545 = 0.0;
            params.p546 = 0.0;
            params.p547 = 0.0;
            params.p548 = 0.0;
            params.p549 = 0.0;
            params.p550 = 0.0;
            params.p551 = 0.0;
            params.p552 = 0.0;
            params.p553 = 0.0;
            params.p554 = 0.0;
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
            params.p578 = params.p571;
            validate_finite_parameter("LAGISL", params.p578).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p579 = params.p572;
            validate_finite_parameter("LBGISL", params.p579).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p580 = params.p573;
            validate_finite_parameter("LBGISL1", params.p580).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p581 = params.p574;
            validate_finite_parameter("LCGISL", params.p581).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p582 = params.p575;
            validate_finite_parameter("LRGISL", params.p582).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p583 = params.p576;
            validate_finite_parameter("LKGISL", params.p583).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p584 = params.p577;
            validate_finite_parameter("LFGISL", params.p584).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p585 = 0.0;
            params.p586 = params.p585;
            validate_finite_parameter("LNTUND", params.p586).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p587 = 0.0;
            params.p588 = params.p587;
            validate_finite_parameter("LNDIODED", params.p588).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p589 = 0.0;
            params.p590 = params.p589;
            validate_finite_parameter("LNRECF0D", params.p590).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p591 = 0.0;
            params.p592 = params.p591;
            validate_finite_parameter("LNRECR0D", params.p592).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p593 = 0.0;
            params.p594 = params.p593;
            validate_finite_parameter("LIDBJT", params.p594).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p595 = 0.0;
            params.p596 = params.p595;
            validate_finite_parameter("LIDDIF", params.p596).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p597 = 0.0;
            params.p598 = params.p597;
            validate_finite_parameter("LIDREC", params.p598).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p599 = 0.0;
            params.p600 = params.p599;
            validate_finite_parameter("LIDTUN", params.p600).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p601 = 0.0;
            params.p602 = params.p601;
            validate_finite_parameter("LVREC0D", params.p602).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p603 = 0.0;
            params.p604 = params.p603;
            validate_finite_parameter("LVTUN0D", params.p604).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p605 = 0.0;
            params.p606 = 0.0;
            params.p607 = 0.0;
            params.p608 = 0.0;
            params.p609 = 0.0;
            params.p610 = params.p609;
            validate_finite_parameter("LAHLID", params.p610).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p611 = 0.0;
            params.p612 = 0.0;
            params.p613 = 0.0;
            params.p614 = 0.0;
            params.p615 = 0.0;
            params.p616 = 0.0;
            params.p617 = params.p616;
            validate_finite_parameter("LNOFF2", params.p617).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p663 = params.p660;
            validate_finite_parameter("WXDIFD", params.p663).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p664 = params.p661;
            validate_finite_parameter("WXRECD", params.p664).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p665 = params.p662;
            validate_finite_parameter("WXTUND", params.p665).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p666 = 0.0;
            params.p667 = 0.0;
            params.p668 = 0.0;
            params.p669 = 0.0;
            params.p670 = 0.0;
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
            params.p730 = params.p728;
            validate_finite_parameter("WETA0CV", params.p730).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p731 = params.p729;
            validate_finite_parameter("WETABCV", params.p731).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p763 = 0.0;
            params.p764 = 0.0;
            params.p765 = 0.0;
            params.p766 = 0.0;
            params.p767 = 0.0;
            params.p768 = params.p761;
            validate_finite_parameter("WAGISL", params.p768).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p769 = params.p762;
            validate_finite_parameter("WBGISL", params.p769).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p770 = params.p763;
            validate_finite_parameter("WBGISL1", params.p770).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p771 = params.p764;
            validate_finite_parameter("WCGISL", params.p771).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p772 = params.p765;
            validate_finite_parameter("WRGISL", params.p772).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p773 = params.p766;
            validate_finite_parameter("WKGISL", params.p773).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p774 = params.p767;
            validate_finite_parameter("WFGISL", params.p774).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p775 = 0.0;
            params.p776 = params.p775;
            validate_finite_parameter("WNTUND", params.p776).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p777 = 0.0;
            params.p778 = params.p777;
            validate_finite_parameter("WNDIODED", params.p778).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p779 = 0.0;
            params.p780 = params.p779;
            validate_finite_parameter("WNRECF0D", params.p780).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p781 = 0.0;
            params.p782 = params.p781;
            validate_finite_parameter("WNRECR0D", params.p782).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p783 = 0.0;
            params.p784 = params.p783;
            validate_finite_parameter("WIDBJT", params.p784).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p785 = 0.0;
            params.p786 = params.p785;
            validate_finite_parameter("WIDDIF", params.p786).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p787 = 0.0;
            params.p788 = params.p787;
            validate_finite_parameter("WIDREC", params.p788).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p789 = 0.0;
            params.p790 = params.p789;
            validate_finite_parameter("WIDTUN", params.p790).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p791 = 0.0;
            params.p792 = params.p791;
            validate_finite_parameter("WVREC0D", params.p792).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p793 = 0.0;
            params.p794 = params.p793;
            validate_finite_parameter("WVTUN0D", params.p794).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p795 = 0.0;
            params.p796 = 0.0;
            params.p797 = 0.0;
            params.p798 = 0.0;
            params.p799 = 0.0;
            params.p800 = params.p799;
            validate_finite_parameter("WAHLID", params.p800).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p801 = 0.0;
            params.p802 = 0.0;
            params.p803 = 0.0;
            params.p804 = 0.0;
            params.p805 = 0.0;
            params.p806 = 0.0;
            params.p807 = params.p806;
            validate_finite_parameter("WNOFF2", params.p807).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p824 = 0.0;
            params.p825 = 0.0;
            params.p826 = 0.0;
            params.p827 = 0.0;
            params.p828 = 0.0;
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
            params.p839 = 0.0;
            params.p840 = 0.0;
            params.p841 = 0.0;
            params.p842 = 0.0;
            params.p843 = 0.0;
            params.p844 = 0.0;
            params.p845 = 0.0;
            params.p846 = 0.0;
            params.p847 = 0.0;
            params.p848 = 0.0;
            params.p849 = 0.0;
            params.p850 = 0.0;
            params.p851 = 0.0;
            params.p852 = 0.0;
            params.p853 = params.p850;
            validate_finite_parameter("PXDIFD", params.p853).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p854 = params.p851;
            validate_finite_parameter("PXRECD", params.p854).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p855 = params.p852;
            validate_finite_parameter("PXTUND", params.p855).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p856 = 0.0;
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
            params.p867 = 0.0;
            params.p868 = 0.0;
            params.p869 = 0.0;
            params.p870 = 0.0;
            params.p871 = 0.0;
            params.p872 = 0.0;
            params.p873 = 0.0;
            params.p874 = 0.0;
            params.p875 = 0.0;
            params.p876 = 0.0;
            params.p877 = 0.0;
            params.p878 = 0.0;
            params.p879 = 0.0;
            params.p880 = 0.0;
            params.p881 = 0.0;
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
            params.p895 = 0.0;
            params.p896 = 0.0;
            params.p897 = 0.0;
            params.p898 = 0.0;
            params.p899 = 0.0;
            params.p900 = 0.0;
            params.p901 = 0.0;
            params.p902 = 0.0;
            params.p903 = 0.0;
            params.p904 = 0.0;
            params.p905 = 0.0;
            params.p906 = 0.0;
            params.p907 = 0.0;
            params.p908 = 0.0;
            params.p909 = 0.0;
            params.p910 = 0.0;
            params.p911 = 0.0;
            params.p912 = 0.0;
            params.p913 = 0.0;
            params.p914 = 0.0;
            params.p915 = 0.0;
            params.p916 = 0.0;
            params.p917 = 0.0;
            params.p918 = 0.0;
            params.p919 = 0.0;
            params.p920 = params.p918;
            validate_finite_parameter("PETA0CV", params.p920).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p921 = params.p919;
            validate_finite_parameter("PETABCV", params.p921).expect("generated Verilog-A parameter default must satisfy declared range");
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
            params.p947 = 0.0;
            params.p948 = 0.0;
            params.p949 = 0.0;
            params.p950 = 0.0;
            params.p951 = 0.0;
            params.p952 = 0.0;
            params.p953 = 0.0;
            params.p954 = 0.0;
            params.p955 = 0.0;
            params.p956 = 0.0;
            params.p957 = 0.0;
            params.p958 = params.p951;
            validate_finite_parameter("PAGISL", params.p958).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p959 = params.p952;
            validate_finite_parameter("PBGISL", params.p959).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p960 = params.p953;
            validate_finite_parameter("PBGISL1", params.p960).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p961 = params.p954;
            validate_finite_parameter("PCGISL", params.p961).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p962 = params.p955;
            validate_finite_parameter("PRGISL", params.p962).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p963 = params.p956;
            validate_finite_parameter("PKGISL", params.p963).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p964 = params.p957;
            validate_finite_parameter("PFGISL", params.p964).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p965 = 0.0;
            params.p966 = params.p965;
            validate_finite_parameter("PNTUND", params.p966).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p967 = 0.0;
            params.p968 = params.p967;
            validate_finite_parameter("PNDIODED", params.p968).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p969 = 0.0;
            params.p970 = params.p969;
            validate_finite_parameter("PNRECF0D", params.p970).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p971 = 0.0;
            params.p972 = params.p971;
            validate_finite_parameter("PNRECR0D", params.p972).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p973 = 0.0;
            params.p974 = params.p973;
            validate_finite_parameter("PIDBJT", params.p974).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p975 = 0.0;
            params.p976 = params.p975;
            validate_finite_parameter("PIDDIF", params.p976).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p977 = 0.0;
            params.p978 = params.p977;
            validate_finite_parameter("PIDREC", params.p978).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p979 = 0.0;
            params.p980 = params.p979;
            validate_finite_parameter("PIDTUN", params.p980).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p981 = 0.0;
            params.p982 = params.p981;
            validate_finite_parameter("PVREC0D", params.p982).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p983 = 0.0;
            params.p984 = params.p983;
            validate_finite_parameter("PVTUN0D", params.p984).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p985 = 0.0;
            params.p986 = 0.0;
            params.p987 = 0.0;
            params.p988 = 0.0;
            params.p989 = 0.0;
            params.p990 = params.p989;
            validate_finite_parameter("PAHLID", params.p990).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p991 = 0.0;
            params.p992 = 0.0;
            params.p993 = 0.0;
            params.p994 = 0.0;
            params.p995 = 0.0;
            params.p996 = 0.0;
            params.p997 = params.p996;
            validate_finite_parameter("PNOFF2", params.p997).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p998 = 0.0;
            params.p999 = 0.0;
            params.p1000 = 0.0;
            params.p1001 = 0.0;
            params.p1002 = 0.0;
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
            params.p1013 = 1.74e-7;
            params.p1014 = 0.0;
            params.p1015 = 0.0;
            params.p1016 = 0.0;
            params.p1017 = 1.2;
            params.p1018 = 0.0;
            params.p1019 = 0.0;
            params.p1020 = 0.0;
            params.p1021 = params.p1013;
            validate_finite_parameter("LPE0", params.p1021).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1022 = params.p1017;
            validate_finite_parameter("EGIDL", params.p1022).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1023 = params.p1022;
            validate_finite_parameter("EGISL", params.p1023).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1024 = params.p1014;
            validate_finite_parameter("LLPE0", params.p1024).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1025 = params.p1018;
            validate_finite_parameter("LEGIDL", params.p1025).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1026 = params.p1025;
            validate_finite_parameter("LEGISL", params.p1026).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1027 = params.p1015;
            validate_finite_parameter("WLPE0", params.p1027).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1028 = params.p1019;
            validate_finite_parameter("WEGIDL", params.p1028).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1029 = params.p1028;
            validate_finite_parameter("WEGISL", params.p1029).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1030 = params.p1016;
            validate_finite_parameter("PLPE0", params.p1030).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1031 = params.p1020;
            validate_finite_parameter("PEGIDL", params.p1031).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1032 = params.p1031;
            validate_finite_parameter("PEGISL", params.p1032).expect("generated Verilog-A parameter default must satisfy declared range");
            params.p1033 = 1.12;
            params.p1034 = 1.12;
            params.p1035 = 3.7622e-7;
            params.p1036 = -31051000000.0;
            params.p1037 = 4.9758e-7;
            params.p1038 = -23570000000.0;
            params.p1039 = 3.42537e-7;
            params.p1040 = 4.97232e-7;
            params.p1041 = 1166450000000.0;
            params.p1042 = 745669000000.0;
            params.p1043 = 0.026;
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
    pub nodes: [usize; 14],
    pub branches: [usize; 19],
    pub(crate) params: Box<Parameters>,
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
    pub(crate) scalar_v0: f64,
    pub(crate) scalar_v1: f64,
    pub(crate) scalar_v2: f64,
    pub(crate) scalar_v5: f64,
    pub(crate) scalar_v6: f64,
    pub(crate) scalar_v7: f64,
    pub(crate) scalar_v9: f64,
    pub(crate) scalar_v10: f64,
    pub(crate) scalar_v11: f64,
    pub(crate) scalar_v13: f64,
    pub(crate) scalar_v14: f64,
    pub(crate) scalar_v15: f64,
    pub(crate) scalar_v16: bool,
    pub(crate) scalar_v17: f64,
    pub(crate) scalar_v19: f64,
    pub(crate) scalar_v21: f64,
    pub(crate) scalar_v22: f64,
    pub(crate) scalar_v24: bool,
    pub(crate) scalar_v25: f64,
    pub(crate) scalar_v26: bool,
    pub(crate) scalar_v27: f64,
    pub(crate) scalar_v28: bool,
    pub(crate) scalar_v32: f64,
    pub(crate) scalar_v33: bool,
    pub(crate) scalar_v34: bool,
    pub(crate) scalar_v35: f64,
    pub(crate) scalar_v36: bool,
    pub(crate) scalar_v37: f64,
    pub(crate) scalar_v38: bool,
    pub(crate) scalar_v39: bool,
    pub(crate) scalar_v40: bool,
    pub(crate) scalar_v41: bool,
    pub(crate) scalar_v42: bool,
    pub(crate) scalar_v43: f64,
    pub(crate) scalar_v44: bool,
    pub(crate) scalar_v45: bool,
    pub(crate) scalar_v46: f64,
    pub(crate) scalar_v47: bool,
    pub(crate) scalar_v48: bool,
    pub(crate) scalar_v49: f64,
    pub(crate) scalar_v50: bool,
    pub(crate) scalar_v51: f64,
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
    pub(crate) scalar_v101: bool,
    pub(crate) scalar_v103: f64,
    pub(crate) scalar_v104: f64,
    pub(crate) scalar_v105: f64,
    pub(crate) scalar_v106: f64,
    pub(crate) scalar_v108: f64,
    pub(crate) scalar_v109: f64,
    pub(crate) scalar_v110: f64,
    pub(crate) scalar_v111: bool,
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
    pub(crate) scalar_v149: f64,
    pub(crate) scalar_v150: f64,
    pub(crate) scalar_v151: f64,
    pub(crate) scalar_v152: f64,
    pub(crate) scalar_v153: f64,
    pub(crate) scalar_v154: f64,
    pub(crate) scalar_v155: f64,
    pub(crate) scalar_v156: f64,
    pub(crate) scalar_v157: f64,
    pub(crate) scalar_v158: f64,
    pub(crate) scalar_v161: f64,
    pub(crate) scalar_v162: f64,
    pub(crate) scalar_v163: bool,
    pub(crate) scalar_v164: f64,
    pub(crate) scalar_v165: bool,
    pub(crate) scalar_v166: f64,
    pub(crate) scalar_v167: bool,
    pub(crate) scalar_v168: f64,
    pub(crate) scalar_v169: f64,
    pub(crate) scalar_v171: f64,
    pub(crate) scalar_v172: f64,
    pub(crate) scalar_v173: f64,
    pub(crate) scalar_v174: bool,
    pub(crate) scalar_v175: f64,
    pub(crate) scalar_v176: f64,
    pub(crate) scalar_v178: f64,
    pub(crate) scalar_v179: f64,
    pub(crate) scalar_v180: f64,
    pub(crate) scalar_v181: f64,
    pub(crate) scalar_v182: f64,
    pub(crate) scalar_v183: f64,
    pub(crate) scalar_v184: bool,
    pub(crate) scalar_v185: bool,
    pub(crate) scalar_v186: f64,
    pub(crate) scalar_v187: bool,
    pub(crate) scalar_v189: f64,
    pub(crate) scalar_v190: f64,
    pub(crate) scalar_v191: f64,
    pub(crate) scalar_v192: f64,
    pub(crate) scalar_v193: f64,
    pub(crate) scalar_v194: bool,
    pub(crate) scalar_v195: bool,
    pub(crate) scalar_v196: f64,
    pub(crate) scalar_v197: f64,
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
    pub(crate) scalar_v211: f64,
    pub(crate) scalar_v212: f64,
    pub(crate) scalar_v213: f64,
    pub(crate) scalar_v214: f64,
    pub(crate) scalar_v216: bool,
    pub(crate) scalar_v217: bool,
    pub(crate) scalar_v218: bool,
    pub(crate) scalar_v219: f64,
    pub(crate) scalar_v220: bool,
    pub(crate) scalar_v221: bool,
    pub(crate) scalar_v222: bool,
    pub(crate) scalar_v223: bool,
    pub(crate) scalar_v224: f64,
    pub(crate) scalar_v225: bool,
    pub(crate) scalar_v226: bool,
    pub(crate) scalar_v227: f64,
    pub(crate) scalar_v228: f64,
    pub(crate) scalar_v229: f64,
    pub(crate) scalar_v230: f64,
    pub(crate) scalar_v231: f64,
    pub(crate) scalar_v232: bool,
    pub(crate) scalar_v233: bool,
    pub(crate) scalar_v234: f64,
    pub(crate) scalar_v235: f64,
    pub(crate) scalar_v236: f64,
    pub(crate) scalar_v237: bool,
    pub(crate) scalar_v238: bool,
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
    pub(crate) scalar_v249: f64,
    pub(crate) scalar_v250: f64,
    pub(crate) scalar_v251: f64,
    pub(crate) scalar_v252: f64,
    pub(crate) scalar_v253: f64,
    pub(crate) scalar_v254: f64,
    pub(crate) scalar_v255: f64,
    pub(crate) scalar_v256: f64,
    pub(crate) scalar_v257: bool,
    pub(crate) scalar_v258: f64,
    pub(crate) scalar_v259: f64,
    pub(crate) scalar_v260: bool,
    pub(crate) scalar_v262: f64,
    pub(crate) scalar_v263: f64,
    pub(crate) scalar_v264: f64,
    pub(crate) scalar_v266: bool,
    pub(crate) scalar_v267: f64,
    pub(crate) scalar_v268: bool,
    pub(crate) scalar_v269: f64,
    pub(crate) scalar_v270: bool,
    pub(crate) scalar_v271: bool,
    pub(crate) scalar_v272: f64,
    pub(crate) scalar_v273: f64,
    pub(crate) scalar_v274: f64,
    pub(crate) scalar_v275: f64,
    pub(crate) scalar_v276: f64,
    pub(crate) scalar_v277: bool,
    pub(crate) scalar_v278: bool,
    pub(crate) scalar_v279: f64,
    pub(crate) scalar_v280: bool,
    pub(crate) scalar_v281: bool,
    pub(crate) scalar_v282: f64,
    pub(crate) scalar_v283: f64,
    pub(crate) scalar_v284: f64,
    pub(crate) scalar_v285: bool,
    pub(crate) scalar_v286: f64,
    pub(crate) scalar_v287: f64,
    pub(crate) scalar_v288: bool,
    pub(crate) scalar_v289: bool,
    pub(crate) scalar_v290: bool,
    pub(crate) scalar_v291: bool,
    pub(crate) scalar_v293: bool,
    pub(crate) scalar_v294: bool,
    pub(crate) scalar_v298: bool,
    pub(crate) scalar_v299: f64,
    pub(crate) scalar_v300: f64,
    pub(crate) scalar_v301: bool,
    pub(crate) scalar_v302: bool,
    pub(crate) scalar_v303: bool,
    pub(crate) scalar_v304: bool,
    pub(crate) scalar_v305: bool,
    pub(crate) scalar_v306: f64,
    pub(crate) scalar_v307: f64,
    pub(crate) scalar_v308: f64,
    pub(crate) scalar_v309: f64,
    pub(crate) scalar_v310: bool,
    pub(crate) scalar_v311: bool,
    pub(crate) scalar_v312: bool,
    pub(crate) scalar_v313: bool,
    pub(crate) scalar_v314: bool,
    pub(crate) scalar_v315: bool,
    pub(crate) scalar_v316: bool,
    pub(crate) scalar_v317: bool,
    pub(crate) scalar_v318: bool,
    pub(crate) scalar_v319: bool,
    pub(crate) scalar_v320: bool,
    pub(crate) scalar_v321: bool,
    pub(crate) scalar_v322: bool,
    pub(crate) scalar_v323: bool,
    pub(crate) scalar_v324: bool,
    pub(crate) scalar_v325: bool,
    pub(crate) scalar_v326: bool,
    pub(crate) scalar_v327: bool,
    pub(crate) scalar_v328: bool,
    pub(crate) scalar_v329: bool,
    pub(crate) scalar_v330: bool,
    pub(crate) scalar_v331: bool,
    pub(crate) scalar_v332: bool,
    pub(crate) scalar_v333: bool,
    pub(crate) scalar_v334: f64,
    pub(crate) scalar_v335: bool,
    pub(crate) scalar_v336: bool,
    pub(crate) scalar_v337: bool,
    pub(crate) scalar_v338: f64,
    pub(crate) scalar_v339: bool,
    pub(crate) scalar_v340: bool,
    pub(crate) scalar_v341: f64,
    pub(crate) scalar_v342: bool,
    pub(crate) scalar_v343: bool,
    pub(crate) scalar_v344: bool,
    pub(crate) scalar_v345: f64,
    pub(crate) scalar_v346: bool,
    pub(crate) scalar_v347: bool,
    pub(crate) scalar_v348: bool,
    pub(crate) scalar_v349: bool,
    pub(crate) scalar_v350: f64,
    pub(crate) scalar_v351: bool,
    pub(crate) scalar_v352: bool,
    pub(crate) scalar_v353: f64,
    pub(crate) scalar_v354: bool,
    pub(crate) scalar_v355: bool,
    pub(crate) scalar_v356: f64,
    pub(crate) scalar_v357: bool,
    pub(crate) scalar_v358: bool,
    pub(crate) scalar_v359: f64,
    pub(crate) scalar_v360: f64,
    pub(crate) scalar_v361: f64,
    pub(crate) scalar_v362: f64,
    pub(crate) scalar_v363: f64,
    pub(crate) scalar_v366: f64,
    pub(crate) scalar_v367: bool,
    pub(crate) scalar_v368: f64,
    pub(crate) scalar_v369: f64,
    pub(crate) scalar_v370: bool,
    pub(crate) scalar_v371: f64,
    pub(crate) scalar_v372: f64,
    pub(crate) scalar_v373: f64,
    pub(crate) scalar_v374: f64,
    pub(crate) scalar_v375: bool,
    pub(crate) scalar_v381: f64,
    pub(crate) scalar_v382: f64,
    pub(crate) scalar_v383: f64,
    pub(crate) scalar_v392: f64,
    pub(crate) scalar_v393: f64,
    pub(crate) scalar_v394: bool,
    pub(crate) scalar_v395: bool,
    pub(crate) scalar_v396: f64,
    pub(crate) scalar_v397: bool,
    pub(crate) scalar_v398: bool,
    pub(crate) scalar_v399: f64,
    pub(crate) scalar_v400: bool,
    pub(crate) scalar_v401: f64,
    pub(crate) scalar_v402: bool,
    pub(crate) scalar_v403: f64,
    pub(crate) scalar_v404: f64,
    pub(crate) scalar_v405: f64,
    pub(crate) scalar_v406: f64,
    pub(crate) scalar_v407: f64,
    pub(crate) scalar_v408: f64,
    pub(crate) scalar_v409: f64,
    pub(crate) scalar_v410: f64,
    pub(crate) scalar_v411: f64,
    pub(crate) scalar_v412: f64,
    pub(crate) scalar_v413: f64,
    pub(crate) scalar_v414: f64,
    pub(crate) scalar_v415: f64,
    pub(crate) scalar_v416: f64,
    pub(crate) scalar_v417: f64,
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
            scalar_v0: self.scalar_v0,
            scalar_v1: self.scalar_v1,
            scalar_v2: self.scalar_v2,
            scalar_v5: self.scalar_v5,
            scalar_v6: self.scalar_v6,
            scalar_v7: self.scalar_v7,
            scalar_v9: self.scalar_v9,
            scalar_v10: self.scalar_v10,
            scalar_v11: self.scalar_v11,
            scalar_v13: self.scalar_v13,
            scalar_v14: self.scalar_v14,
            scalar_v15: self.scalar_v15,
            scalar_v16: self.scalar_v16,
            scalar_v17: self.scalar_v17,
            scalar_v19: self.scalar_v19,
            scalar_v21: self.scalar_v21,
            scalar_v22: self.scalar_v22,
            scalar_v24: self.scalar_v24,
            scalar_v25: self.scalar_v25,
            scalar_v26: self.scalar_v26,
            scalar_v27: self.scalar_v27,
            scalar_v28: self.scalar_v28,
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
            scalar_v103: self.scalar_v103,
            scalar_v104: self.scalar_v104,
            scalar_v105: self.scalar_v105,
            scalar_v106: self.scalar_v106,
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
            scalar_v161: self.scalar_v161,
            scalar_v162: self.scalar_v162,
            scalar_v163: self.scalar_v163,
            scalar_v164: self.scalar_v164,
            scalar_v165: self.scalar_v165,
            scalar_v166: self.scalar_v166,
            scalar_v167: self.scalar_v167,
            scalar_v168: self.scalar_v168,
            scalar_v169: self.scalar_v169,
            scalar_v171: self.scalar_v171,
            scalar_v172: self.scalar_v172,
            scalar_v173: self.scalar_v173,
            scalar_v174: self.scalar_v174,
            scalar_v175: self.scalar_v175,
            scalar_v176: self.scalar_v176,
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
            scalar_v211: self.scalar_v211,
            scalar_v212: self.scalar_v212,
            scalar_v213: self.scalar_v213,
            scalar_v214: self.scalar_v214,
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
            scalar_v246: self.scalar_v246,
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
            scalar_v262: self.scalar_v262,
            scalar_v263: self.scalar_v263,
            scalar_v264: self.scalar_v264,
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
            scalar_v293: self.scalar_v293,
            scalar_v294: self.scalar_v294,
            scalar_v298: self.scalar_v298,
            scalar_v299: self.scalar_v299,
            scalar_v300: self.scalar_v300,
            scalar_v301: self.scalar_v301,
            scalar_v302: self.scalar_v302,
            scalar_v303: self.scalar_v303,
            scalar_v304: self.scalar_v304,
            scalar_v305: self.scalar_v305,
            scalar_v306: self.scalar_v306,
            scalar_v307: self.scalar_v307,
            scalar_v308: self.scalar_v308,
            scalar_v309: self.scalar_v309,
            scalar_v310: self.scalar_v310,
            scalar_v311: self.scalar_v311,
            scalar_v312: self.scalar_v312,
            scalar_v313: self.scalar_v313,
            scalar_v314: self.scalar_v314,
            scalar_v315: self.scalar_v315,
            scalar_v316: self.scalar_v316,
            scalar_v317: self.scalar_v317,
            scalar_v318: self.scalar_v318,
            scalar_v319: self.scalar_v319,
            scalar_v320: self.scalar_v320,
            scalar_v321: self.scalar_v321,
            scalar_v322: self.scalar_v322,
            scalar_v323: self.scalar_v323,
            scalar_v324: self.scalar_v324,
            scalar_v325: self.scalar_v325,
            scalar_v326: self.scalar_v326,
            scalar_v327: self.scalar_v327,
            scalar_v328: self.scalar_v328,
            scalar_v329: self.scalar_v329,
            scalar_v330: self.scalar_v330,
            scalar_v331: self.scalar_v331,
            scalar_v332: self.scalar_v332,
            scalar_v333: self.scalar_v333,
            scalar_v334: self.scalar_v334,
            scalar_v335: self.scalar_v335,
            scalar_v336: self.scalar_v336,
            scalar_v337: self.scalar_v337,
            scalar_v338: self.scalar_v338,
            scalar_v339: self.scalar_v339,
            scalar_v340: self.scalar_v340,
            scalar_v341: self.scalar_v341,
            scalar_v342: self.scalar_v342,
            scalar_v343: self.scalar_v343,
            scalar_v344: self.scalar_v344,
            scalar_v345: self.scalar_v345,
            scalar_v346: self.scalar_v346,
            scalar_v347: self.scalar_v347,
            scalar_v348: self.scalar_v348,
            scalar_v349: self.scalar_v349,
            scalar_v350: self.scalar_v350,
            scalar_v351: self.scalar_v351,
            scalar_v352: self.scalar_v352,
            scalar_v353: self.scalar_v353,
            scalar_v354: self.scalar_v354,
            scalar_v355: self.scalar_v355,
            scalar_v356: self.scalar_v356,
            scalar_v357: self.scalar_v357,
            scalar_v358: self.scalar_v358,
            scalar_v359: self.scalar_v359,
            scalar_v360: self.scalar_v360,
            scalar_v361: self.scalar_v361,
            scalar_v362: self.scalar_v362,
            scalar_v363: self.scalar_v363,
            scalar_v366: self.scalar_v366,
            scalar_v367: self.scalar_v367,
            scalar_v368: self.scalar_v368,
            scalar_v369: self.scalar_v369,
            scalar_v370: self.scalar_v370,
            scalar_v371: self.scalar_v371,
            scalar_v372: self.scalar_v372,
            scalar_v373: self.scalar_v373,
            scalar_v374: self.scalar_v374,
            scalar_v375: self.scalar_v375,
            scalar_v381: self.scalar_v381,
            scalar_v382: self.scalar_v382,
            scalar_v383: self.scalar_v383,
            scalar_v392: self.scalar_v392,
            scalar_v393: self.scalar_v393,
            scalar_v394: self.scalar_v394,
            scalar_v395: self.scalar_v395,
            scalar_v396: self.scalar_v396,
            scalar_v397: self.scalar_v397,
            scalar_v398: self.scalar_v398,
            scalar_v399: self.scalar_v399,
            scalar_v400: self.scalar_v400,
            scalar_v401: self.scalar_v401,
            scalar_v402: self.scalar_v402,
            scalar_v403: self.scalar_v403,
            scalar_v404: self.scalar_v404,
            scalar_v405: self.scalar_v405,
            scalar_v406: self.scalar_v406,
            scalar_v407: self.scalar_v407,
            scalar_v408: self.scalar_v408,
            scalar_v409: self.scalar_v409,
            scalar_v410: self.scalar_v410,
            scalar_v411: self.scalar_v411,
            scalar_v412: self.scalar_v412,
            scalar_v413: self.scalar_v413,
            scalar_v414: self.scalar_v414,
            scalar_v415: self.scalar_v415,
            scalar_v416: self.scalar_v416,
            scalar_v417: self.scalar_v417,
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
            scalar_v0: 0.0,
            scalar_v1: 0.0,
            scalar_v2: 0.0,
            scalar_v5: 0.0,
            scalar_v6: 0.0,
            scalar_v7: 0.0,
            scalar_v9: 0.0,
            scalar_v10: 0.0,
            scalar_v11: 0.0,
            scalar_v13: 0.0,
            scalar_v14: 0.0,
            scalar_v15: 0.0,
            scalar_v16: false,
            scalar_v17: 0.0,
            scalar_v19: 0.0,
            scalar_v21: 0.0,
            scalar_v22: 0.0,
            scalar_v24: false,
            scalar_v25: 0.0,
            scalar_v26: false,
            scalar_v27: 0.0,
            scalar_v28: false,
            scalar_v32: 0.0,
            scalar_v33: false,
            scalar_v34: false,
            scalar_v35: 0.0,
            scalar_v36: false,
            scalar_v37: 0.0,
            scalar_v38: false,
            scalar_v39: false,
            scalar_v40: false,
            scalar_v41: false,
            scalar_v42: false,
            scalar_v43: 0.0,
            scalar_v44: false,
            scalar_v45: false,
            scalar_v46: 0.0,
            scalar_v47: false,
            scalar_v48: false,
            scalar_v49: 0.0,
            scalar_v50: false,
            scalar_v51: 0.0,
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
            scalar_v101: false,
            scalar_v103: 0.0,
            scalar_v104: 0.0,
            scalar_v105: 0.0,
            scalar_v106: 0.0,
            scalar_v108: 0.0,
            scalar_v109: 0.0,
            scalar_v110: 0.0,
            scalar_v111: false,
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
            scalar_v149: 0.0,
            scalar_v150: 0.0,
            scalar_v151: 0.0,
            scalar_v152: 0.0,
            scalar_v153: 0.0,
            scalar_v154: 0.0,
            scalar_v155: 0.0,
            scalar_v156: 0.0,
            scalar_v157: 0.0,
            scalar_v158: 0.0,
            scalar_v161: 0.0,
            scalar_v162: 0.0,
            scalar_v163: false,
            scalar_v164: 0.0,
            scalar_v165: false,
            scalar_v166: 0.0,
            scalar_v167: false,
            scalar_v168: 0.0,
            scalar_v169: 0.0,
            scalar_v171: 0.0,
            scalar_v172: 0.0,
            scalar_v173: 0.0,
            scalar_v174: false,
            scalar_v175: 0.0,
            scalar_v176: 0.0,
            scalar_v178: 0.0,
            scalar_v179: 0.0,
            scalar_v180: 0.0,
            scalar_v181: 0.0,
            scalar_v182: 0.0,
            scalar_v183: 0.0,
            scalar_v184: false,
            scalar_v185: false,
            scalar_v186: 0.0,
            scalar_v187: false,
            scalar_v189: 0.0,
            scalar_v190: 0.0,
            scalar_v191: 0.0,
            scalar_v192: 0.0,
            scalar_v193: 0.0,
            scalar_v194: false,
            scalar_v195: false,
            scalar_v196: 0.0,
            scalar_v197: 0.0,
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
            scalar_v211: 0.0,
            scalar_v212: 0.0,
            scalar_v213: 0.0,
            scalar_v214: 0.0,
            scalar_v216: false,
            scalar_v217: false,
            scalar_v218: false,
            scalar_v219: 0.0,
            scalar_v220: false,
            scalar_v221: false,
            scalar_v222: false,
            scalar_v223: false,
            scalar_v224: 0.0,
            scalar_v225: false,
            scalar_v226: false,
            scalar_v227: 0.0,
            scalar_v228: 0.0,
            scalar_v229: 0.0,
            scalar_v230: 0.0,
            scalar_v231: 0.0,
            scalar_v232: false,
            scalar_v233: false,
            scalar_v234: 0.0,
            scalar_v235: 0.0,
            scalar_v236: 0.0,
            scalar_v237: false,
            scalar_v238: false,
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
            scalar_v249: 0.0,
            scalar_v250: 0.0,
            scalar_v251: 0.0,
            scalar_v252: 0.0,
            scalar_v253: 0.0,
            scalar_v254: 0.0,
            scalar_v255: 0.0,
            scalar_v256: 0.0,
            scalar_v257: false,
            scalar_v258: 0.0,
            scalar_v259: 0.0,
            scalar_v260: false,
            scalar_v262: 0.0,
            scalar_v263: 0.0,
            scalar_v264: 0.0,
            scalar_v266: false,
            scalar_v267: 0.0,
            scalar_v268: false,
            scalar_v269: 0.0,
            scalar_v270: false,
            scalar_v271: false,
            scalar_v272: 0.0,
            scalar_v273: 0.0,
            scalar_v274: 0.0,
            scalar_v275: 0.0,
            scalar_v276: 0.0,
            scalar_v277: false,
            scalar_v278: false,
            scalar_v279: 0.0,
            scalar_v280: false,
            scalar_v281: false,
            scalar_v282: 0.0,
            scalar_v283: 0.0,
            scalar_v284: 0.0,
            scalar_v285: false,
            scalar_v286: 0.0,
            scalar_v287: 0.0,
            scalar_v288: false,
            scalar_v289: false,
            scalar_v290: false,
            scalar_v291: false,
            scalar_v293: false,
            scalar_v294: false,
            scalar_v298: false,
            scalar_v299: 0.0,
            scalar_v300: 0.0,
            scalar_v301: false,
            scalar_v302: false,
            scalar_v303: false,
            scalar_v304: false,
            scalar_v305: false,
            scalar_v306: 0.0,
            scalar_v307: 0.0,
            scalar_v308: 0.0,
            scalar_v309: 0.0,
            scalar_v310: false,
            scalar_v311: false,
            scalar_v312: false,
            scalar_v313: false,
            scalar_v314: false,
            scalar_v315: false,
            scalar_v316: false,
            scalar_v317: false,
            scalar_v318: false,
            scalar_v319: false,
            scalar_v320: false,
            scalar_v321: false,
            scalar_v322: false,
            scalar_v323: false,
            scalar_v324: false,
            scalar_v325: false,
            scalar_v326: false,
            scalar_v327: false,
            scalar_v328: false,
            scalar_v329: false,
            scalar_v330: false,
            scalar_v331: false,
            scalar_v332: false,
            scalar_v333: false,
            scalar_v334: 0.0,
            scalar_v335: false,
            scalar_v336: false,
            scalar_v337: false,
            scalar_v338: 0.0,
            scalar_v339: false,
            scalar_v340: false,
            scalar_v341: 0.0,
            scalar_v342: false,
            scalar_v343: false,
            scalar_v344: false,
            scalar_v345: 0.0,
            scalar_v346: false,
            scalar_v347: false,
            scalar_v348: false,
            scalar_v349: false,
            scalar_v350: 0.0,
            scalar_v351: false,
            scalar_v352: false,
            scalar_v353: 0.0,
            scalar_v354: false,
            scalar_v355: false,
            scalar_v356: 0.0,
            scalar_v357: false,
            scalar_v358: false,
            scalar_v359: 0.0,
            scalar_v360: 0.0,
            scalar_v361: 0.0,
            scalar_v362: 0.0,
            scalar_v363: 0.0,
            scalar_v366: 0.0,
            scalar_v367: false,
            scalar_v368: 0.0,
            scalar_v369: 0.0,
            scalar_v370: false,
            scalar_v371: 0.0,
            scalar_v372: 0.0,
            scalar_v373: 0.0,
            scalar_v374: 0.0,
            scalar_v375: false,
            scalar_v381: 0.0,
            scalar_v382: 0.0,
            scalar_v383: 0.0,
            scalar_v392: 0.0,
            scalar_v393: 0.0,
            scalar_v394: false,
            scalar_v395: false,
            scalar_v396: 0.0,
            scalar_v397: false,
            scalar_v398: false,
            scalar_v399: 0.0,
            scalar_v400: false,
            scalar_v401: 0.0,
            scalar_v402: false,
            scalar_v403: 0.0,
            scalar_v404: 0.0,
            scalar_v405: 0.0,
            scalar_v406: 0.0,
            scalar_v407: 0.0,
            scalar_v408: 0.0,
            scalar_v409: 0.0,
            scalar_v410: 0.0,
            scalar_v411: 0.0,
            scalar_v412: 0.0,
            scalar_v413: 0.0,
            scalar_v414: 0.0,
            scalar_v415: 0.0,
            scalar_v416: 0.0,
            scalar_v417: 0.0,
        };
        instance.recompute_instance_static();
        instance
    }

    #[inline]
    pub fn restore_from_snapshot(&mut self, snapshot: Self) {
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
            scalar_v0,
            scalar_v1,
            scalar_v2,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
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
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
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
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
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
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
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
            scalar_v246,
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
            scalar_v262,
            scalar_v263,
            scalar_v264,
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
            scalar_v293,
            scalar_v294,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
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
            scalar_v0,
            scalar_v1,
            scalar_v2,
            scalar_v5,
            scalar_v6,
            scalar_v7,
            scalar_v9,
            scalar_v10,
            scalar_v11,
            scalar_v13,
            scalar_v14,
            scalar_v15,
            scalar_v16,
            scalar_v17,
            scalar_v19,
            scalar_v21,
            scalar_v22,
            scalar_v24,
            scalar_v25,
            scalar_v26,
            scalar_v27,
            scalar_v28,
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
            scalar_v103,
            scalar_v104,
            scalar_v105,
            scalar_v106,
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
            scalar_v161,
            scalar_v162,
            scalar_v163,
            scalar_v164,
            scalar_v165,
            scalar_v166,
            scalar_v167,
            scalar_v168,
            scalar_v169,
            scalar_v171,
            scalar_v172,
            scalar_v173,
            scalar_v174,
            scalar_v175,
            scalar_v176,
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
            scalar_v211,
            scalar_v212,
            scalar_v213,
            scalar_v214,
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
            scalar_v246,
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
            scalar_v262,
            scalar_v263,
            scalar_v264,
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
            scalar_v293,
            scalar_v294,
            scalar_v298,
            scalar_v299,
            scalar_v300,
            scalar_v301,
            scalar_v302,
            scalar_v303,
            scalar_v304,
            scalar_v305,
            scalar_v306,
            scalar_v307,
            scalar_v308,
            scalar_v309,
            scalar_v310,
            scalar_v311,
            scalar_v312,
            scalar_v313,
            scalar_v314,
            scalar_v315,
            scalar_v316,
            scalar_v317,
            scalar_v318,
            scalar_v319,
            scalar_v320,
            scalar_v321,
            scalar_v322,
            scalar_v323,
            scalar_v324,
            scalar_v325,
            scalar_v326,
            scalar_v327,
            scalar_v328,
            scalar_v329,
            scalar_v330,
            scalar_v331,
            scalar_v332,
            scalar_v333,
            scalar_v334,
            scalar_v335,
            scalar_v336,
            scalar_v337,
            scalar_v338,
            scalar_v339,
            scalar_v340,
            scalar_v341,
            scalar_v342,
            scalar_v343,
            scalar_v344,
            scalar_v345,
            scalar_v346,
            scalar_v347,
            scalar_v348,
            scalar_v349,
            scalar_v350,
            scalar_v351,
            scalar_v352,
            scalar_v353,
            scalar_v354,
            scalar_v355,
            scalar_v356,
            scalar_v357,
            scalar_v358,
            scalar_v359,
            scalar_v360,
            scalar_v361,
            scalar_v362,
            scalar_v363,
            scalar_v366,
            scalar_v367,
            scalar_v368,
            scalar_v369,
            scalar_v370,
            scalar_v371,
            scalar_v372,
            scalar_v373,
            scalar_v374,
            scalar_v375,
            scalar_v381,
            scalar_v382,
            scalar_v383,
            scalar_v392,
            scalar_v393,
            scalar_v394,
            scalar_v395,
            scalar_v396,
            scalar_v397,
            scalar_v398,
            scalar_v399,
            scalar_v400,
            scalar_v401,
            scalar_v402,
            scalar_v403,
            scalar_v404,
            scalar_v405,
            scalar_v406,
            scalar_v407,
            scalar_v408,
            scalar_v409,
            scalar_v410,
            scalar_v411,
            scalar_v412,
            scalar_v413,
            scalar_v414,
            scalar_v415,
            scalar_v416,
            scalar_v417,
        };
    }

    #[inline]
    pub fn set_branch_indices(&mut self, branches: &[usize]) {
        assert_eq!(branches.len(), Self::BRANCH_COUNT, "generated Verilog-A branch count mismatch");
        self.branches.copy_from_slice(branches);
    }

    pub fn set_parameter(&mut self, name: &str, value: f64) -> Result<(), String> {
        match name.to_ascii_lowercase().as_str() {
            "dtemp" => { validate_finite_parameter("DTEMP", value)?; self.params.p0 = value; self.mark_param_given(0); self.recompute_instance_static(); Ok(()) }
            "l" => { validate_parameter("L", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1 = value; self.mark_param_given(1); self.recompute_instance_static(); Ok(()) }
            "w" => { validate_parameter("W", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p2 = value; self.mark_param_given(2); self.recompute_instance_static(); Ok(()) }
            "nf" => { validate_parameter("NF", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p3 = value; self.mark_param_given(3); self.recompute_instance_static(); Ok(()) }
            "sa" => { validate_parameter("SA", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p4 = value; self.mark_param_given(4); self.recompute_instance_static(); Ok(()) }
            "sb" => { validate_parameter("SB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p5 = value; self.mark_param_given(5); self.recompute_instance_static(); Ok(()) }
            "sd" => { validate_parameter("SD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p6 = value; self.mark_param_given(6); self.recompute_instance_static(); Ok(()) }
            "ad" => { validate_parameter("AD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p7 = value; self.mark_param_given(7); self.recompute_instance_static(); Ok(()) }
            "as" => { validate_parameter("AS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p8 = value; self.mark_param_given(8); self.recompute_instance_static(); Ok(()) }
            "pd" => { validate_parameter("PD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p9 = value; self.mark_param_given(9); self.recompute_instance_static(); Ok(()) }
            "ps" => { validate_parameter("PS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p10 = value; self.mark_param_given(10); self.recompute_instance_static(); Ok(()) }
            "nrd" => { validate_parameter("NRD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p11 = value; self.mark_param_given(11); self.recompute_instance_static(); Ok(()) }
            "nrs" => { validate_parameter("NRS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p12 = value; self.mark_param_given(12); self.recompute_instance_static(); Ok(()) }
            "bjtoff" => { validate_parameter("BJTOFF", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p13 = value; self.mark_param_given(13); self.recompute_instance_static(); Ok(()) }
            "rth0" => { validate_parameter("RTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p14 = value; self.mark_param_given(14); self.recompute_instance_static(); Ok(()) }
            "cth0" => { validate_parameter("CTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p15 = value; self.mark_param_given(15); self.recompute_instance_static(); Ok(()) }
            "nrb" => { validate_parameter("NRB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p16 = value; self.mark_param_given(16); self.recompute_instance_static(); Ok(()) }
            "frbody" => { validate_finite_parameter("FRBODY", value)?; self.params.p17 = value; self.mark_param_given(17); self.recompute_instance_static(); Ok(()) }
            "rbdb" => { validate_parameter("RBDB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p18 = value; self.mark_param_given(18); self.recompute_instance_static(); Ok(()) }
            "rbsb" => { validate_parameter("RBSB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p19 = value; self.mark_param_given(19); self.recompute_instance_static(); Ok(()) }
            "delvto" => { validate_finite_parameter("DELVTO", value)?; self.params.p20 = value; self.mark_param_given(20); self.recompute_instance_static(); Ok(()) }
            "soimod" => { validate_parameter("SOIMOD", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p21 = value; self.mark_param_given(21); self.recompute_instance_static(); Ok(()) }
            "nbc" => { validate_parameter("NBC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p22 = value; self.mark_param_given(22); self.recompute_instance_static(); Ok(()) }
            "nseg" => { validate_parameter("NSEG", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p23 = value; self.mark_param_given(23); self.recompute_instance_static(); Ok(()) }
            "pdbcp" => { validate_parameter("PDBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p24 = value; self.mark_param_given(24); self.recompute_instance_static(); Ok(()) }
            "psbcp" => { validate_parameter("PSBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p25 = value; self.mark_param_given(25); self.recompute_instance_static(); Ok(()) }
            "agbcp" => { validate_parameter("AGBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p26 = value; self.mark_param_given(26); self.recompute_instance_static(); Ok(()) }
            "agbcp2" => { validate_parameter("AGBCP2", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p27 = value; self.mark_param_given(27); self.recompute_instance_static(); Ok(()) }
            "agbcpd" => { validate_parameter("AGBCPD", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p28 = value; self.mark_param_given(28); self.recompute_instance_static(); Ok(()) }
            "aebcp" => { validate_parameter("AEBCP", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p29 = value; self.mark_param_given(29); self.recompute_instance_static(); Ok(()) }
            "ids0mult" => { validate_parameter("IDS0MULT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p30 = value; self.mark_param_given(30); self.recompute_instance_static(); Ok(()) }
            "u0mult" => { validate_parameter("U0MULT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p31 = value; self.mark_param_given(31); self.recompute_instance_static(); Ok(()) }
            "mult_i" => { validate_parameter("MULT_I", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p32 = value; self.mark_param_given(32); self.recompute_instance_static(); Ok(()) }
            "mult_q" => { validate_parameter("MULT_Q", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p33 = value; self.mark_param_given(33); self.recompute_instance_static(); Ok(()) }
            "mult_fn" => { validate_parameter("MULT_FN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p34 = value; self.mark_param_given(34); self.recompute_instance_static(); Ok(()) }
            "tnodeout" => { validate_parameter("TNODEOUT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p35 = value; self.mark_param_given(35); self.recompute_instance_static(); Ok(()) }
            "shmod" => { validate_parameter("SHMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p36 = value; self.mark_param_given(36); self.recompute_instance_static(); Ok(()) }
            "type" => { validate_parameter("TYPE", value, Some((-1.0, "-1.0")), false, Some((1.0, "1.0")), false, &[(0.0, "0.0")])?; self.params.p37 = value; self.mark_param_given(37); self.recompute_instance_static(); Ok(()) }
            "version" => { validate_parameter("VERSION", value, Some((4.0, "4.0")), false, Some((5.0, "5.0")), true, &[])?; self.params.p38 = value; self.mark_param_given(38); self.recompute_instance_static(); Ok(()) }
            "rgatemod" => { validate_parameter("RGATEMOD", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p39 = value; self.mark_param_given(39); self.recompute_instance_static(); Ok(()) }
            "rbodymod" => { validate_parameter("RBODYMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p40 = value; self.mark_param_given(40); self.recompute_instance_static(); Ok(()) }
            "mtrlmod" => { validate_parameter("MTRLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p41 = value; self.mark_param_given(41); self.recompute_instance_static(); Ok(()) }
            "vgstcvmod" => { validate_parameter("VGSTCVMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p42 = value; self.mark_param_given(42); self.recompute_instance_static(); Ok(()) }
            "gidlmod" => { validate_parameter("GIDLMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p43 = value; self.mark_param_given(43); self.recompute_instance_static(); Ok(()) }
            "iiimod" => { validate_parameter("IIIMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p44 = value; self.mark_param_given(44); self.recompute_instance_static(); Ok(()) }
            "eot" => { validate_parameter("EOT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p45 = value; self.mark_param_given(45); self.recompute_instance_static(); Ok(()) }
            "epsrox" => { validate_parameter("EPSROX", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p46 = value; self.mark_param_given(46); self.recompute_instance_static(); Ok(()) }
            "epsrsub" => { validate_parameter("EPSRSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p47 = value; self.mark_param_given(47); self.recompute_instance_static(); Ok(()) }
            "ni0sub" => { validate_parameter("NI0SUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p48 = value; self.mark_param_given(48); self.recompute_instance_static(); Ok(()) }
            "bg0sub" => { validate_parameter("BG0SUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p49 = value; self.mark_param_given(49); self.recompute_instance_static(); Ok(()) }
            "tbgasub" => { validate_parameter("TBGASUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p50 = value; self.mark_param_given(50); self.recompute_instance_static(); Ok(()) }
            "tbgbsub" => { validate_parameter("TBGBSUB", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p51 = value; self.mark_param_given(51); self.recompute_instance_static(); Ok(()) }
            "phig" => { validate_parameter("PHIG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p52 = value; self.mark_param_given(52); self.recompute_instance_static(); Ok(()) }
            "easub" => { validate_finite_parameter("EASUB", value)?; self.params.p53 = value; self.mark_param_given(53); self.recompute_instance_static(); Ok(()) }
            "leffeot" => { validate_parameter("LEFFEOT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p54 = value; self.mark_param_given(54); self.recompute_instance_static(); Ok(()) }
            "weffeot" => { validate_finite_parameter("WEFFEOT", value)?; self.params.p55 = value; self.mark_param_given(55); self.recompute_instance_static(); Ok(()) }
            "vddeot" => { validate_finite_parameter("VDDEOT", value)?; self.params.p56 = value; self.mark_param_given(56); self.recompute_instance_static(); Ok(()) }
            "tempeot" => { validate_parameter("TEMPEOT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p57 = value; self.mark_param_given(57); self.recompute_instance_static(); Ok(()) }
            "ados" => { validate_parameter("ADOS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p58 = value; self.mark_param_given(58); self.recompute_instance_static(); Ok(()) }
            "bdos" => { validate_parameter("BDOS", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p59 = value; self.mark_param_given(59); self.recompute_instance_static(); Ok(()) }
            "epsrgate" => { validate_parameter("EPSRGATE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p60 = value; self.mark_param_given(60); self.recompute_instance_static(); Ok(()) }
            "capmod" => { validate_parameter("CAPMOD", value, Some((2.0, "2.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p61 = value; self.mark_param_given(61); self.recompute_instance_static(); Ok(()) }
            "mobmod" => { validate_parameter("MOBMOD", value, Some((1.0, "1.0")), false, Some((4.0, "4.0")), false, &[])?; self.params.p62 = value; self.mark_param_given(62); self.recompute_instance_static(); Ok(()) }
            "paramchk" => { validate_parameter("PARAMCHK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p63 = value; self.mark_param_given(63); self.recompute_instance_static(); Ok(()) }
            "nodechk" => { validate_parameter("NODECHK", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p64 = value; self.mark_param_given(64); self.recompute_instance_static(); Ok(()) }
            "binunit" => { validate_parameter("BINUNIT", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p65 = value; self.mark_param_given(65); self.recompute_instance_static(); Ok(()) }
            "tox" => { validate_parameter("TOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p66 = value; self.mark_param_given(66); self.recompute_instance_static(); Ok(()) }
            "toxm" => { validate_parameter("TOXM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p67 = value; self.mark_param_given(67); self.recompute_instance_static(); Ok(()) }
            "dtoxcv" => { validate_finite_parameter("DTOXCV", value)?; self.params.p68 = value; self.mark_param_given(68); self.recompute_instance_static(); Ok(()) }
            "cdsc" => { validate_finite_parameter("CDSC", value)?; self.params.p69 = value; self.mark_param_given(69); self.recompute_instance_static(); Ok(()) }
            "cdscb" => { validate_finite_parameter("CDSCB", value)?; self.params.p70 = value; self.mark_param_given(70); self.recompute_instance_static(); Ok(()) }
            "cdscd" => { validate_finite_parameter("CDSCD", value)?; self.params.p71 = value; self.mark_param_given(71); self.recompute_instance_static(); Ok(()) }
            "cit" => { validate_finite_parameter("CIT", value)?; self.params.p72 = value; self.mark_param_given(72); self.recompute_instance_static(); Ok(()) }
            "nfactor" => { validate_finite_parameter("NFACTOR", value)?; self.params.p73 = value; self.mark_param_given(73); self.recompute_instance_static(); Ok(()) }
            "vsat" => { validate_parameter("VSAT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p74 = value; self.mark_param_given(74); self.recompute_instance_static(); Ok(()) }
            "at" => { validate_finite_parameter("AT", value)?; self.params.p75 = value; self.mark_param_given(75); self.recompute_instance_static(); Ok(()) }
            "a0" => { validate_finite_parameter("A0", value)?; self.params.p76 = value; self.mark_param_given(76); self.recompute_instance_static(); Ok(()) }
            "ags" => { validate_finite_parameter("AGS", value)?; self.params.p77 = value; self.mark_param_given(77); self.recompute_instance_static(); Ok(()) }
            "a1" => { validate_finite_parameter("A1", value)?; self.params.p78 = value; self.mark_param_given(78); self.recompute_instance_static(); Ok(()) }
            "a2" => { validate_finite_parameter("A2", value)?; self.params.p79 = value; self.mark_param_given(79); self.recompute_instance_static(); Ok(()) }
            "keta" => { validate_finite_parameter("KETA", value)?; self.params.p80 = value; self.mark_param_given(80); self.recompute_instance_static(); Ok(()) }
            "nsub" => { validate_parameter("NSUB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p81 = value; self.mark_param_given(81); self.recompute_instance_static(); Ok(()) }
            "nch" => { validate_parameter("NCH", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p82 = value; self.mark_param_given(82); self.recompute_instance_static(); Ok(()) }
            "ngate" => { validate_parameter("NGATE", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p83 = value; self.mark_param_given(83); self.recompute_instance_static(); Ok(()) }
            "nsd" => { validate_parameter("NSD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p84 = value; self.mark_param_given(84); self.recompute_instance_static(); Ok(()) }
            "gamma1" => { validate_finite_parameter("GAMMA1", value)?; self.params.p85 = value; self.mark_param_given(85); self.recompute_instance_static(); Ok(()) }
            "gamma2" => { validate_finite_parameter("GAMMA2", value)?; self.params.p86 = value; self.mark_param_given(86); self.recompute_instance_static(); Ok(()) }
            "vbx" => { validate_finite_parameter("VBX", value)?; self.params.p87 = value; self.mark_param_given(87); self.recompute_instance_static(); Ok(()) }
            "vbm" => { validate_finite_parameter("VBM", value)?; self.params.p88 = value; self.mark_param_given(88); self.recompute_instance_static(); Ok(()) }
            "xt" => { validate_parameter("XT", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p89 = value; self.mark_param_given(89); self.recompute_instance_static(); Ok(()) }
            "k1" => { validate_finite_parameter("K1", value)?; self.params.p90 = value; self.mark_param_given(90); self.recompute_instance_static(); Ok(()) }
            "kt1" => { validate_finite_parameter("KT1", value)?; self.params.p91 = value; self.mark_param_given(91); self.recompute_instance_static(); Ok(()) }
            "kt1l" => { validate_finite_parameter("KT1L", value)?; self.params.p92 = value; self.mark_param_given(92); self.recompute_instance_static(); Ok(()) }
            "kt2" => { validate_finite_parameter("KT2", value)?; self.params.p93 = value; self.mark_param_given(93); self.recompute_instance_static(); Ok(()) }
            "k2" => { validate_finite_parameter("K2", value)?; self.params.p94 = value; self.mark_param_given(94); self.recompute_instance_static(); Ok(()) }
            "k3" => { validate_finite_parameter("K3", value)?; self.params.p95 = value; self.mark_param_given(95); self.recompute_instance_static(); Ok(()) }
            "k3b" => { validate_finite_parameter("K3B", value)?; self.params.p96 = value; self.mark_param_given(96); self.recompute_instance_static(); Ok(()) }
            "w0" => { validate_finite_parameter("W0", value)?; self.params.p97 = value; self.mark_param_given(97); self.recompute_instance_static(); Ok(()) }
            "lpeb" => { validate_finite_parameter("LPEB", value)?; self.params.p98 = value; self.mark_param_given(98); self.recompute_instance_static(); Ok(()) }
            "dvt0" => { validate_finite_parameter("DVT0", value)?; self.params.p99 = value; self.mark_param_given(99); self.recompute_instance_static(); Ok(()) }
            "dvt1" => { validate_finite_parameter("DVT1", value)?; self.params.p100 = value; self.mark_param_given(100); self.recompute_instance_static(); Ok(()) }
            "dvt2" => { validate_finite_parameter("DVT2", value)?; self.params.p101 = value; self.mark_param_given(101); self.recompute_instance_static(); Ok(()) }
            "dvt0w" => { validate_finite_parameter("DVT0W", value)?; self.params.p102 = value; self.mark_param_given(102); self.recompute_instance_static(); Ok(()) }
            "dvt1w" => { validate_finite_parameter("DVT1W", value)?; self.params.p103 = value; self.mark_param_given(103); self.recompute_instance_static(); Ok(()) }
            "dvt2w" => { validate_finite_parameter("DVT2W", value)?; self.params.p104 = value; self.mark_param_given(104); self.recompute_instance_static(); Ok(()) }
            "drout" => { validate_finite_parameter("DROUT", value)?; self.params.p105 = value; self.mark_param_given(105); self.recompute_instance_static(); Ok(()) }
            "dsub" => { validate_finite_parameter("DSUB", value)?; self.params.p106 = value; self.mark_param_given(106); self.recompute_instance_static(); Ok(()) }
            "vtho" => { validate_finite_parameter("VTHO", value)?; self.params.p107 = value; self.mark_param_given(107); self.recompute_instance_static(); Ok(()) }
            "vth0" => { validate_finite_parameter("VTH0", value)?; self.params.p108 = value; self.mark_param_given(108); self.recompute_instance_static(); Ok(()) }
            "vfb" => { validate_finite_parameter("VFB", value)?; self.params.p109 = value; self.mark_param_given(109); self.recompute_instance_static(); Ok(()) }
            "ua" => { validate_finite_parameter("UA", value)?; self.params.p110 = value; self.mark_param_given(110); self.recompute_instance_static(); Ok(()) }
            "ua1" => { validate_finite_parameter("UA1", value)?; self.params.p111 = value; self.mark_param_given(111); self.recompute_instance_static(); Ok(()) }
            "ub" => { validate_finite_parameter("UB", value)?; self.params.p112 = value; self.mark_param_given(112); self.recompute_instance_static(); Ok(()) }
            "ub1" => { validate_finite_parameter("UB1", value)?; self.params.p113 = value; self.mark_param_given(113); self.recompute_instance_static(); Ok(()) }
            "uc" => { validate_finite_parameter("UC", value)?; self.params.p114 = value; self.mark_param_given(114); self.recompute_instance_static(); Ok(()) }
            "uc1" => { validate_finite_parameter("UC1", value)?; self.params.p115 = value; self.mark_param_given(115); self.recompute_instance_static(); Ok(()) }
            "u0" => { validate_finite_parameter("U0", value)?; self.params.p116 = value; self.mark_param_given(116); self.recompute_instance_static(); Ok(()) }
            "eu" => { validate_finite_parameter("EU", value)?; self.params.p117 = value; self.mark_param_given(117); self.recompute_instance_static(); Ok(()) }
            "ute" => { validate_finite_parameter("UTE", value)?; self.params.p118 = value; self.mark_param_given(118); self.recompute_instance_static(); Ok(()) }
            "ucs" => { validate_finite_parameter("UCS", value)?; self.params.p119 = value; self.mark_param_given(119); self.recompute_instance_static(); Ok(()) }
            "ucste" => { validate_finite_parameter("UCSTE", value)?; self.params.p120 = value; self.mark_param_given(120); self.recompute_instance_static(); Ok(()) }
            "ud" => { validate_finite_parameter("UD", value)?; self.params.p121 = value; self.mark_param_given(121); self.recompute_instance_static(); Ok(()) }
            "ud1" => { validate_finite_parameter("UD1", value)?; self.params.p122 = value; self.mark_param_given(122); self.recompute_instance_static(); Ok(()) }
            "ubg1" => { validate_finite_parameter("UBG1", value)?; self.params.p123 = value; self.mark_param_given(123); self.recompute_instance_static(); Ok(()) }
            "ubg2" => { validate_finite_parameter("UBG2", value)?; self.params.p124 = value; self.mark_param_given(124); self.recompute_instance_static(); Ok(()) }
            "voff" => { validate_finite_parameter("VOFF", value)?; self.params.p125 = value; self.mark_param_given(125); self.recompute_instance_static(); Ok(()) }
            "tnom" => { validate_finite_parameter("TNOM", value)?; self.params.p126 = value; self.mark_param_given(126); self.recompute_instance_static(); Ok(()) }
            "cgso" => { validate_parameter("CGSO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p127 = value; self.mark_param_given(127); self.recompute_instance_static(); Ok(()) }
            "cgdo" => { validate_parameter("CGDO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p128 = value; self.mark_param_given(128); self.recompute_instance_static(); Ok(()) }
            "xpart" => { validate_finite_parameter("XPART", value)?; self.params.p129 = value; self.mark_param_given(129); self.recompute_instance_static(); Ok(()) }
            "delta" => { validate_finite_parameter("DELTA", value)?; self.params.p130 = value; self.mark_param_given(130); self.recompute_instance_static(); Ok(()) }
            "rsh" => { validate_parameter("RSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p131 = value; self.mark_param_given(131); self.recompute_instance_static(); Ok(()) }
            "rdsw" => { validate_parameter("RDSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p132 = value; self.mark_param_given(132); self.recompute_instance_static(); Ok(()) }
            "rsw" => { validate_parameter("RSW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p133 = value; self.mark_param_given(133); self.recompute_instance_static(); Ok(()) }
            "rdw" => { validate_parameter("RDW", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p134 = value; self.mark_param_given(134); self.recompute_instance_static(); Ok(()) }
            "rsc" => { validate_parameter("RSC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p135 = value; self.mark_param_given(135); self.recompute_instance_static(); Ok(()) }
            "rdc" => { validate_parameter("RDC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p136 = value; self.mark_param_given(136); self.recompute_instance_static(); Ok(()) }
            "trs" => { validate_finite_parameter("TRS", value)?; self.params.p137 = value; self.mark_param_given(137); self.recompute_instance_static(); Ok(()) }
            "trd" => { validate_finite_parameter("TRD", value)?; self.params.p138 = value; self.mark_param_given(138); self.recompute_instance_static(); Ok(()) }
            "rswmin" => { validate_parameter("RSWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p139 = value; self.mark_param_given(139); self.recompute_instance_static(); Ok(()) }
            "rdwmin" => { validate_parameter("RDWMIN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p140 = value; self.mark_param_given(140); self.recompute_instance_static(); Ok(()) }
            "prwg" => { validate_finite_parameter("PRWG", value)?; self.params.p141 = value; self.mark_param_given(141); self.recompute_instance_static(); Ok(()) }
            "prwb" => { validate_finite_parameter("PRWB", value)?; self.params.p142 = value; self.mark_param_given(142); self.recompute_instance_static(); Ok(()) }
            "prwe" => { validate_finite_parameter("PRWE", value)?; self.params.p143 = value; self.mark_param_given(143); self.recompute_instance_static(); Ok(()) }
            "prt" => { validate_finite_parameter("PRT", value)?; self.params.p144 = value; self.mark_param_given(144); self.recompute_instance_static(); Ok(()) }
            "eta0" => { validate_finite_parameter("ETA0", value)?; self.params.p145 = value; self.mark_param_given(145); self.recompute_instance_static(); Ok(()) }
            "etab" => { validate_finite_parameter("ETAB", value)?; self.params.p146 = value; self.mark_param_given(146); self.recompute_instance_static(); Ok(()) }
            "eta0cv" => { validate_finite_parameter("ETA0CV", value)?; self.params.p147 = value; self.mark_param_given(147); self.recompute_instance_static(); Ok(()) }
            "etabcv" => { validate_finite_parameter("ETABCV", value)?; self.params.p148 = value; self.mark_param_given(148); self.recompute_instance_static(); Ok(()) }
            "pclm" => { validate_finite_parameter("PCLM", value)?; self.params.p149 = value; self.mark_param_given(149); self.recompute_instance_static(); Ok(()) }
            "pdiblc1" => { validate_finite_parameter("PDIBLC1", value)?; self.params.p150 = value; self.mark_param_given(150); self.recompute_instance_static(); Ok(()) }
            "pdiblc2" => { validate_finite_parameter("PDIBLC2", value)?; self.params.p151 = value; self.mark_param_given(151); self.recompute_instance_static(); Ok(()) }
            "pdiblcb" => { validate_finite_parameter("PDIBLCB", value)?; self.params.p152 = value; self.mark_param_given(152); self.recompute_instance_static(); Ok(()) }
            "pvag" => { validate_finite_parameter("PVAG", value)?; self.params.p153 = value; self.mark_param_given(153); self.recompute_instance_static(); Ok(()) }
            "tbox" => { validate_parameter("TBOX", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p154 = value; self.mark_param_given(154); self.recompute_instance_static(); Ok(()) }
            "tsi" => { validate_parameter("TSI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p155 = value; self.mark_param_given(155); self.recompute_instance_static(); Ok(()) }
            "etsi" => { validate_parameter("ETSI", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p156 = value; self.mark_param_given(156); self.recompute_instance_static(); Ok(()) }
            "xj" => { validate_parameter("XJ", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p157 = value; self.mark_param_given(157); self.recompute_instance_static(); Ok(()) }
            "agidl" => { validate_finite_parameter("AGIDL", value)?; self.params.p158 = value; self.mark_param_given(158); self.recompute_instance_static(); Ok(()) }
            "bgidl" => { validate_finite_parameter("BGIDL", value)?; self.params.p159 = value; self.mark_param_given(159); self.recompute_instance_static(); Ok(()) }
            "bgidl1" => { validate_finite_parameter("BGIDL1", value)?; self.params.p160 = value; self.mark_param_given(160); self.recompute_instance_static(); Ok(()) }
            "cgidl" => { validate_finite_parameter("CGIDL", value)?; self.params.p161 = value; self.mark_param_given(161); self.recompute_instance_static(); Ok(()) }
            "rgidl" => { validate_finite_parameter("RGIDL", value)?; self.params.p162 = value; self.mark_param_given(162); self.recompute_instance_static(); Ok(()) }
            "kgidl" => { validate_finite_parameter("KGIDL", value)?; self.params.p163 = value; self.mark_param_given(163); self.recompute_instance_static(); Ok(()) }
            "fgidl" => { validate_finite_parameter("FGIDL", value)?; self.params.p164 = value; self.mark_param_given(164); self.recompute_instance_static(); Ok(()) }
            "agisl" => { validate_finite_parameter("AGISL", value)?; self.params.p165 = value; self.mark_param_given(165); self.recompute_instance_static(); Ok(()) }
            "bgisl" => { validate_finite_parameter("BGISL", value)?; self.params.p166 = value; self.mark_param_given(166); self.recompute_instance_static(); Ok(()) }
            "bgisl1" => { validate_finite_parameter("BGISL1", value)?; self.params.p167 = value; self.mark_param_given(167); self.recompute_instance_static(); Ok(()) }
            "cgisl" => { validate_finite_parameter("CGISL", value)?; self.params.p168 = value; self.mark_param_given(168); self.recompute_instance_static(); Ok(()) }
            "rgisl" => { validate_finite_parameter("RGISL", value)?; self.params.p169 = value; self.mark_param_given(169); self.recompute_instance_static(); Ok(()) }
            "kgisl" => { validate_finite_parameter("KGISL", value)?; self.params.p170 = value; self.mark_param_given(170); self.recompute_instance_static(); Ok(()) }
            "fgisl" => { validate_finite_parameter("FGISL", value)?; self.params.p171 = value; self.mark_param_given(171); self.recompute_instance_static(); Ok(()) }
            "ndiode" => { validate_parameter("NDIODE", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p172 = value; self.mark_param_given(172); self.recompute_instance_static(); Ok(()) }
            "ndioded" => { validate_parameter("NDIODED", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p173 = value; self.mark_param_given(173); self.recompute_instance_static(); Ok(()) }
            "xbjt" => { validate_finite_parameter("XBJT", value)?; self.params.p174 = value; self.mark_param_given(174); self.recompute_instance_static(); Ok(()) }
            "xdif" => { validate_finite_parameter("XDIF", value)?; self.params.p175 = value; self.mark_param_given(175); self.recompute_instance_static(); Ok(()) }
            "xrec" => { validate_finite_parameter("XREC", value)?; self.params.p176 = value; self.mark_param_given(176); self.recompute_instance_static(); Ok(()) }
            "xtun" => { validate_finite_parameter("XTUN", value)?; self.params.p177 = value; self.mark_param_given(177); self.recompute_instance_static(); Ok(()) }
            "xdifd" => { validate_finite_parameter("XDIFD", value)?; self.params.p178 = value; self.mark_param_given(178); self.recompute_instance_static(); Ok(()) }
            "xrecd" => { validate_finite_parameter("XRECD", value)?; self.params.p179 = value; self.mark_param_given(179); self.recompute_instance_static(); Ok(()) }
            "xtund" => { validate_finite_parameter("XTUND", value)?; self.params.p180 = value; self.mark_param_given(180); self.recompute_instance_static(); Ok(()) }
            "pbswg" => { validate_parameter("PBSWG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p181 = value; self.mark_param_given(181); self.recompute_instance_static(); Ok(()) }
            "pbswgd" => { validate_parameter("PBSWGD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p182 = value; self.mark_param_given(182); self.recompute_instance_static(); Ok(()) }
            "mjswg" => { validate_finite_parameter("MJSWG", value)?; self.params.p183 = value; self.mark_param_given(183); self.recompute_instance_static(); Ok(()) }
            "mjswgd" => { validate_finite_parameter("MJSWGD", value)?; self.params.p184 = value; self.mark_param_given(184); self.recompute_instance_static(); Ok(()) }
            "cjswg" => { validate_parameter("CJSWG", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p185 = value; self.mark_param_given(185); self.recompute_instance_static(); Ok(()) }
            "cjswgd" => { validate_parameter("CJSWGD", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p186 = value; self.mark_param_given(186); self.recompute_instance_static(); Ok(()) }
            "lint" => { validate_finite_parameter("LINT", value)?; self.params.p187 = value; self.mark_param_given(187); self.recompute_instance_static(); Ok(()) }
            "ll" => { validate_finite_parameter("LL", value)?; self.params.p188 = value; self.mark_param_given(188); self.recompute_instance_static(); Ok(()) }
            "llc" => { validate_finite_parameter("LLC", value)?; self.params.p189 = value; self.mark_param_given(189); self.recompute_instance_static(); Ok(()) }
            "lln" => { validate_finite_parameter("LLN", value)?; self.params.p190 = value; self.mark_param_given(190); self.recompute_instance_static(); Ok(()) }
            "lw" => { validate_finite_parameter("LW", value)?; self.params.p191 = value; self.mark_param_given(191); self.recompute_instance_static(); Ok(()) }
            "lwc" => { validate_finite_parameter("LWC", value)?; self.params.p192 = value; self.mark_param_given(192); self.recompute_instance_static(); Ok(()) }
            "lwn" => { validate_finite_parameter("LWN", value)?; self.params.p193 = value; self.mark_param_given(193); self.recompute_instance_static(); Ok(()) }
            "lwl" => { validate_finite_parameter("LWL", value)?; self.params.p194 = value; self.mark_param_given(194); self.recompute_instance_static(); Ok(()) }
            "lwlc" => { validate_finite_parameter("LWLC", value)?; self.params.p195 = value; self.mark_param_given(195); self.recompute_instance_static(); Ok(()) }
            "wr" => { validate_finite_parameter("WR", value)?; self.params.p196 = value; self.mark_param_given(196); self.recompute_instance_static(); Ok(()) }
            "wint" => { validate_finite_parameter("WINT", value)?; self.params.p197 = value; self.mark_param_given(197); self.recompute_instance_static(); Ok(()) }
            "dwg" => { validate_finite_parameter("DWG", value)?; self.params.p198 = value; self.mark_param_given(198); self.recompute_instance_static(); Ok(()) }
            "dwb" => { validate_finite_parameter("DWB", value)?; self.params.p199 = value; self.mark_param_given(199); self.recompute_instance_static(); Ok(()) }
            "wl" => { validate_finite_parameter("WL", value)?; self.params.p200 = value; self.mark_param_given(200); self.recompute_instance_static(); Ok(()) }
            "wlc" => { validate_finite_parameter("WLC", value)?; self.params.p201 = value; self.mark_param_given(201); self.recompute_instance_static(); Ok(()) }
            "wln" => { validate_finite_parameter("WLN", value)?; self.params.p202 = value; self.mark_param_given(202); self.recompute_instance_static(); Ok(()) }
            "ww" => { validate_finite_parameter("WW", value)?; self.params.p203 = value; self.mark_param_given(203); self.recompute_instance_static(); Ok(()) }
            "wwc" => { validate_finite_parameter("WWC", value)?; self.params.p204 = value; self.mark_param_given(204); self.recompute_instance_static(); Ok(()) }
            "wwn" => { validate_finite_parameter("WWN", value)?; self.params.p205 = value; self.mark_param_given(205); self.recompute_instance_static(); Ok(()) }
            "wwl" => { validate_finite_parameter("WWL", value)?; self.params.p206 = value; self.mark_param_given(206); self.recompute_instance_static(); Ok(()) }
            "wwlc" => { validate_finite_parameter("WWLC", value)?; self.params.p207 = value; self.mark_param_given(207); self.recompute_instance_static(); Ok(()) }
            "b0" => { validate_finite_parameter("B0", value)?; self.params.p208 = value; self.mark_param_given(208); self.recompute_instance_static(); Ok(()) }
            "b1" => { validate_finite_parameter("B1", value)?; self.params.p209 = value; self.mark_param_given(209); self.recompute_instance_static(); Ok(()) }
            "cgsl" => { validate_finite_parameter("CGSL", value)?; self.params.p210 = value; self.mark_param_given(210); self.recompute_instance_static(); Ok(()) }
            "cgdl" => { validate_finite_parameter("CGDL", value)?; self.params.p211 = value; self.mark_param_given(211); self.recompute_instance_static(); Ok(()) }
            "ckappa" => { validate_parameter("CKAPPA", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p212 = value; self.mark_param_given(212); self.recompute_instance_static(); Ok(()) }
            "cf" => { validate_parameter("CF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p213 = value; self.mark_param_given(213); self.recompute_instance_static(); Ok(()) }
            "clc" => { validate_parameter("CLC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p214 = value; self.mark_param_given(214); self.recompute_instance_static(); Ok(()) }
            "cle" => { validate_finite_parameter("CLE", value)?; self.params.p215 = value; self.mark_param_given(215); self.recompute_instance_static(); Ok(()) }
            "dwc" => { validate_finite_parameter("DWC", value)?; self.params.p216 = value; self.mark_param_given(216); self.recompute_instance_static(); Ok(()) }
            "dlc" => { validate_finite_parameter("DLC", value)?; self.params.p217 = value; self.mark_param_given(217); self.recompute_instance_static(); Ok(()) }
            "alpha0" => { validate_finite_parameter("ALPHA0", value)?; self.params.p218 = value; self.mark_param_given(218); self.recompute_instance_static(); Ok(()) }
            "noia" => { validate_finite_parameter("NOIA", value)?; self.params.p219 = value; self.mark_param_given(219); self.recompute_instance_static(); Ok(()) }
            "noib" => { validate_finite_parameter("NOIB", value)?; self.params.p220 = value; self.mark_param_given(220); self.recompute_instance_static(); Ok(()) }
            "noic" => { validate_finite_parameter("NOIC", value)?; self.params.p221 = value; self.mark_param_given(221); self.recompute_instance_static(); Ok(()) }
            "fnoimod" => { validate_parameter("FNOIMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p222 = value; self.mark_param_given(222); self.recompute_instance_static(); Ok(()) }
            "tnoimod" => { validate_parameter("TNOIMOD", value, Some((0.0, "0.0")), false, Some((3.0, "3.0")), false, &[])?; self.params.p223 = value; self.mark_param_given(223); self.recompute_instance_static(); Ok(()) }
            "tnoic" => { validate_finite_parameter("TNOIC", value)?; self.params.p224 = value; self.mark_param_given(224); self.recompute_instance_static(); Ok(()) }
            "rnoic" => { validate_finite_parameter("RNOIC", value)?; self.params.p225 = value; self.mark_param_given(225); self.recompute_instance_static(); Ok(()) }
            "scalen" => { validate_parameter("SCALEN", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p226 = value; self.mark_param_given(226); self.recompute_instance_static(); Ok(()) }
            "tnoia" => { validate_finite_parameter("TNOIA", value)?; self.params.p227 = value; self.mark_param_given(227); self.recompute_instance_static(); Ok(()) }
            "tnoib" => { validate_finite_parameter("TNOIB", value)?; self.params.p228 = value; self.mark_param_given(228); self.recompute_instance_static(); Ok(()) }
            "rnoia" => { validate_finite_parameter("RNOIA", value)?; self.params.p229 = value; self.mark_param_given(229); self.recompute_instance_static(); Ok(()) }
            "rnoib" => { validate_finite_parameter("RNOIB", value)?; self.params.p230 = value; self.mark_param_given(230); self.recompute_instance_static(); Ok(()) }
            "ntnoi" => { validate_parameter("NTNOI", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p231 = value; self.mark_param_given(231); self.recompute_instance_static(); Ok(()) }
            "saref" => { validate_parameter("SAREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p232 = value; self.mark_param_given(232); self.recompute_instance_static(); Ok(()) }
            "sbref" => { validate_parameter("SBREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p233 = value; self.mark_param_given(233); self.recompute_instance_static(); Ok(()) }
            "wlod" => { validate_finite_parameter("WLOD", value)?; self.params.p234 = value; self.mark_param_given(234); self.recompute_instance_static(); Ok(()) }
            "ku0" => { validate_finite_parameter("KU0", value)?; self.params.p235 = value; self.mark_param_given(235); self.recompute_instance_static(); Ok(()) }
            "kvsat" => { validate_finite_parameter("KVSAT", value)?; self.params.p236 = value; self.mark_param_given(236); self.recompute_instance_static(); Ok(()) }
            "kvth0" => { validate_finite_parameter("KVTH0", value)?; self.params.p237 = value; self.mark_param_given(237); self.recompute_instance_static(); Ok(()) }
            "tku0" => { validate_finite_parameter("TKU0", value)?; self.params.p238 = value; self.mark_param_given(238); self.recompute_instance_static(); Ok(()) }
            "llodku0" => { validate_finite_parameter("LLODKU0", value)?; self.params.p239 = value; self.mark_param_given(239); self.recompute_instance_static(); Ok(()) }
            "wlodku0" => { validate_finite_parameter("WLODKU0", value)?; self.params.p240 = value; self.mark_param_given(240); self.recompute_instance_static(); Ok(()) }
            "llodvth" => { validate_finite_parameter("LLODVTH", value)?; self.params.p241 = value; self.mark_param_given(241); self.recompute_instance_static(); Ok(()) }
            "wlodvth" => { validate_finite_parameter("WLODVTH", value)?; self.params.p242 = value; self.mark_param_given(242); self.recompute_instance_static(); Ok(()) }
            "lku0" => { validate_finite_parameter("LKU0", value)?; self.params.p243 = value; self.mark_param_given(243); self.recompute_instance_static(); Ok(()) }
            "wku0" => { validate_finite_parameter("WKU0", value)?; self.params.p244 = value; self.mark_param_given(244); self.recompute_instance_static(); Ok(()) }
            "pku0" => { validate_finite_parameter("PKU0", value)?; self.params.p245 = value; self.mark_param_given(245); self.recompute_instance_static(); Ok(()) }
            "lkvth0" => { validate_finite_parameter("LKVTH0", value)?; self.params.p246 = value; self.mark_param_given(246); self.recompute_instance_static(); Ok(()) }
            "wkvth0" => { validate_finite_parameter("WKVTH0", value)?; self.params.p247 = value; self.mark_param_given(247); self.recompute_instance_static(); Ok(()) }
            "pkvth0" => { validate_finite_parameter("PKVTH0", value)?; self.params.p248 = value; self.mark_param_given(248); self.recompute_instance_static(); Ok(()) }
            "stk2" => { validate_finite_parameter("STK2", value)?; self.params.p249 = value; self.mark_param_given(249); self.recompute_instance_static(); Ok(()) }
            "lodk2" => { validate_finite_parameter("LODK2", value)?; self.params.p250 = value; self.mark_param_given(250); self.recompute_instance_static(); Ok(()) }
            "steta0" => { validate_finite_parameter("STETA0", value)?; self.params.p251 = value; self.mark_param_given(251); self.recompute_instance_static(); Ok(()) }
            "lodeta0" => { validate_finite_parameter("LODETA0", value)?; self.params.p252 = value; self.mark_param_given(252); self.recompute_instance_static(); Ok(()) }
            "steta0cv" => { validate_finite_parameter("STETA0CV", value)?; self.params.p253 = value; self.mark_param_given(253); self.recompute_instance_static(); Ok(()) }
            "lodeta0cv" => { validate_finite_parameter("LODETA0CV", value)?; self.params.p254 = value; self.mark_param_given(254); self.recompute_instance_static(); Ok(()) }
            "gbmin" => { validate_finite_parameter("GBMIN", value)?; self.params.p255 = value; self.mark_param_given(255); self.recompute_instance_static(); Ok(()) }
            "bf" => { validate_finite_parameter("BF", value)?; self.params.p256 = value; self.mark_param_given(256); self.recompute_instance_static(); Ok(()) }
            "w0flk" => { validate_finite_parameter("W0FLK", value)?; self.params.p257 = value; self.mark_param_given(257); self.recompute_instance_static(); Ok(()) }
            "dvtp0" => { validate_finite_parameter("DVTP0", value)?; self.params.p258 = value; self.mark_param_given(258); self.recompute_instance_static(); Ok(()) }
            "ldvtp0" => { validate_finite_parameter("LDVTP0", value)?; self.params.p259 = value; self.mark_param_given(259); self.recompute_instance_static(); Ok(()) }
            "wdvtp0" => { validate_finite_parameter("WDVTP0", value)?; self.params.p260 = value; self.mark_param_given(260); self.recompute_instance_static(); Ok(()) }
            "pdvtp0" => { validate_finite_parameter("PDVTP0", value)?; self.params.p261 = value; self.mark_param_given(261); self.recompute_instance_static(); Ok(()) }
            "dvtp1" => { validate_finite_parameter("DVTP1", value)?; self.params.p262 = value; self.mark_param_given(262); self.recompute_instance_static(); Ok(()) }
            "ldvtp1" => { validate_finite_parameter("LDVTP1", value)?; self.params.p263 = value; self.mark_param_given(263); self.recompute_instance_static(); Ok(()) }
            "wdvtp1" => { validate_finite_parameter("WDVTP1", value)?; self.params.p264 = value; self.mark_param_given(264); self.recompute_instance_static(); Ok(()) }
            "pdvtp1" => { validate_finite_parameter("PDVTP1", value)?; self.params.p265 = value; self.mark_param_given(265); self.recompute_instance_static(); Ok(()) }
            "dvtp2" => { validate_finite_parameter("DVTP2", value)?; self.params.p266 = value; self.mark_param_given(266); self.recompute_instance_static(); Ok(()) }
            "ldvtp2" => { validate_finite_parameter("LDVTP2", value)?; self.params.p267 = value; self.mark_param_given(267); self.recompute_instance_static(); Ok(()) }
            "wdvtp2" => { validate_finite_parameter("WDVTP2", value)?; self.params.p268 = value; self.mark_param_given(268); self.recompute_instance_static(); Ok(()) }
            "pdvtp2" => { validate_finite_parameter("PDVTP2", value)?; self.params.p269 = value; self.mark_param_given(269); self.recompute_instance_static(); Ok(()) }
            "dvtp3" => { validate_finite_parameter("DVTP3", value)?; self.params.p270 = value; self.mark_param_given(270); self.recompute_instance_static(); Ok(()) }
            "ldvtp3" => { validate_finite_parameter("LDVTP3", value)?; self.params.p271 = value; self.mark_param_given(271); self.recompute_instance_static(); Ok(()) }
            "wdvtp3" => { validate_finite_parameter("WDVTP3", value)?; self.params.p272 = value; self.mark_param_given(272); self.recompute_instance_static(); Ok(()) }
            "pdvtp3" => { validate_finite_parameter("PDVTP3", value)?; self.params.p273 = value; self.mark_param_given(273); self.recompute_instance_static(); Ok(()) }
            "dvtp4" => { validate_finite_parameter("DVTP4", value)?; self.params.p274 = value; self.mark_param_given(274); self.recompute_instance_static(); Ok(()) }
            "ldvtp4" => { validate_finite_parameter("LDVTP4", value)?; self.params.p275 = value; self.mark_param_given(275); self.recompute_instance_static(); Ok(()) }
            "wdvtp4" => { validate_finite_parameter("WDVTP4", value)?; self.params.p276 = value; self.mark_param_given(276); self.recompute_instance_static(); Ok(()) }
            "pdvtp4" => { validate_finite_parameter("PDVTP4", value)?; self.params.p277 = value; self.mark_param_given(277); self.recompute_instance_static(); Ok(()) }
            "minv" => { validate_finite_parameter("MINV", value)?; self.params.p278 = value; self.mark_param_given(278); self.recompute_instance_static(); Ok(()) }
            "lminv" => { validate_finite_parameter("LMINV", value)?; self.params.p279 = value; self.mark_param_given(279); self.recompute_instance_static(); Ok(()) }
            "wminv" => { validate_finite_parameter("WMINV", value)?; self.params.p280 = value; self.mark_param_given(280); self.recompute_instance_static(); Ok(()) }
            "pminv" => { validate_finite_parameter("PMINV", value)?; self.params.p281 = value; self.mark_param_given(281); self.recompute_instance_static(); Ok(()) }
            "pdits" => { validate_finite_parameter("PDITS", value)?; self.params.p282 = value; self.mark_param_given(282); self.recompute_instance_static(); Ok(()) }
            "pditsl" => { validate_finite_parameter("PDITSL", value)?; self.params.p283 = value; self.mark_param_given(283); self.recompute_instance_static(); Ok(()) }
            "pditsd" => { validate_finite_parameter("PDITSD", value)?; self.params.p284 = value; self.mark_param_given(284); self.recompute_instance_static(); Ok(()) }
            "fprout" => { validate_finite_parameter("FPROUT", value)?; self.params.p285 = value; self.mark_param_given(285); self.recompute_instance_static(); Ok(()) }
            "lfprout" => { validate_finite_parameter("LFPROUT", value)?; self.params.p286 = value; self.mark_param_given(286); self.recompute_instance_static(); Ok(()) }
            "lpdits" => { validate_finite_parameter("LPDITS", value)?; self.params.p287 = value; self.mark_param_given(287); self.recompute_instance_static(); Ok(()) }
            "lpditsd" => { validate_finite_parameter("LPDITSD", value)?; self.params.p288 = value; self.mark_param_given(288); self.recompute_instance_static(); Ok(()) }
            "wfprout" => { validate_finite_parameter("WFPROUT", value)?; self.params.p289 = value; self.mark_param_given(289); self.recompute_instance_static(); Ok(()) }
            "wpdits" => { validate_finite_parameter("WPDITS", value)?; self.params.p290 = value; self.mark_param_given(290); self.recompute_instance_static(); Ok(()) }
            "wpditsd" => { validate_finite_parameter("WPDITSD", value)?; self.params.p291 = value; self.mark_param_given(291); self.recompute_instance_static(); Ok(()) }
            "pfprout" => { validate_finite_parameter("PFPROUT", value)?; self.params.p292 = value; self.mark_param_given(292); self.recompute_instance_static(); Ok(()) }
            "ppdits" => { validate_finite_parameter("PPDITS", value)?; self.params.p293 = value; self.mark_param_given(293); self.recompute_instance_static(); Ok(()) }
            "ppditsd" => { validate_finite_parameter("PPDITSD", value)?; self.params.p294 = value; self.mark_param_given(294); self.recompute_instance_static(); Ok(()) }
            "em" => { validate_finite_parameter("EM", value)?; self.params.p295 = value; self.mark_param_given(295); self.recompute_instance_static(); Ok(()) }
            "ef" => { validate_parameter("EF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p296 = value; self.mark_param_given(296); self.recompute_instance_static(); Ok(()) }
            "af" => { validate_finite_parameter("AF", value)?; self.params.p297 = value; self.mark_param_given(297); self.recompute_instance_static(); Ok(()) }
            "kf" => { validate_parameter("KF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p298 = value; self.mark_param_given(298); self.recompute_instance_static(); Ok(()) }
            "noif" => { validate_parameter("NOIF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p299 = value; self.mark_param_given(299); self.recompute_instance_static(); Ok(()) }
            "k1w1" => { validate_finite_parameter("K1W1", value)?; self.params.p300 = value; self.mark_param_given(300); self.recompute_instance_static(); Ok(()) }
            "k1w2" => { validate_finite_parameter("K1W2", value)?; self.params.p301 = value; self.mark_param_given(301); self.recompute_instance_static(); Ok(()) }
            "ketas" => { validate_finite_parameter("KETAS", value)?; self.params.p302 = value; self.mark_param_given(302); self.recompute_instance_static(); Ok(()) }
            "dwbc" => { validate_finite_parameter("DWBC", value)?; self.params.p303 = value; self.mark_param_given(303); self.recompute_instance_static(); Ok(()) }
            "beta0" => { validate_finite_parameter("BETA0", value)?; self.params.p304 = value; self.mark_param_given(304); self.recompute_instance_static(); Ok(()) }
            "beta1" => { validate_finite_parameter("BETA1", value)?; self.params.p305 = value; self.mark_param_given(305); self.recompute_instance_static(); Ok(()) }
            "beta2" => { validate_finite_parameter("BETA2", value)?; self.params.p306 = value; self.mark_param_given(306); self.recompute_instance_static(); Ok(()) }
            "vdsatii0" => { validate_finite_parameter("VDSATII0", value)?; self.params.p307 = value; self.mark_param_given(307); self.recompute_instance_static(); Ok(()) }
            "tii" => { validate_finite_parameter("TII", value)?; self.params.p308 = value; self.mark_param_given(308); self.recompute_instance_static(); Ok(()) }
            "lii" => { validate_finite_parameter("LII", value)?; self.params.p309 = value; self.mark_param_given(309); self.recompute_instance_static(); Ok(()) }
            "sii0" => { validate_finite_parameter("SII0", value)?; self.params.p310 = value; self.mark_param_given(310); self.recompute_instance_static(); Ok(()) }
            "sii1" => { validate_finite_parameter("SII1", value)?; self.params.p311 = value; self.mark_param_given(311); self.recompute_instance_static(); Ok(()) }
            "sii2" => { validate_finite_parameter("SII2", value)?; self.params.p312 = value; self.mark_param_given(312); self.recompute_instance_static(); Ok(()) }
            "siid" => { validate_finite_parameter("SIID", value)?; self.params.p313 = value; self.mark_param_given(313); self.recompute_instance_static(); Ok(()) }
            "fbjtii" => { validate_finite_parameter("FBJTII", value)?; self.params.p314 = value; self.mark_param_given(314); self.recompute_instance_static(); Ok(()) }
            "ebjtii" => { validate_finite_parameter("EBJTII", value)?; self.params.p315 = value; self.mark_param_given(315); self.recompute_instance_static(); Ok(()) }
            "cbjtii" => { validate_finite_parameter("CBJTII", value)?; self.params.p316 = value; self.mark_param_given(316); self.recompute_instance_static(); Ok(()) }
            "vbci" => { validate_finite_parameter("VBCI", value)?; self.params.p317 = value; self.mark_param_given(317); self.recompute_instance_static(); Ok(()) }
            "abjtii" => { validate_finite_parameter("ABJTII", value)?; self.params.p318 = value; self.mark_param_given(318); self.recompute_instance_static(); Ok(()) }
            "mbjtii" => { validate_finite_parameter("MBJTII", value)?; self.params.p319 = value; self.mark_param_given(319); self.recompute_instance_static(); Ok(()) }
            "tvbci" => { validate_finite_parameter("TVBCI", value)?; self.params.p320 = value; self.mark_param_given(320); self.recompute_instance_static(); Ok(()) }
            "esatii" => { validate_finite_parameter("ESATII", value)?; self.params.p321 = value; self.mark_param_given(321); self.recompute_instance_static(); Ok(()) }
            "ntun" => { validate_parameter("NTUN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p322 = value; self.mark_param_given(322); self.recompute_instance_static(); Ok(()) }
            "ntund" => { validate_parameter("NTUND", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p323 = value; self.mark_param_given(323); self.recompute_instance_static(); Ok(()) }
            "nrecf0" => { validate_parameter("NRECF0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p324 = value; self.mark_param_given(324); self.recompute_instance_static(); Ok(()) }
            "nrecf0d" => { validate_parameter("NRECF0D", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p325 = value; self.mark_param_given(325); self.recompute_instance_static(); Ok(()) }
            "nrecr0" => { validate_parameter("NRECR0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p326 = value; self.mark_param_given(326); self.recompute_instance_static(); Ok(()) }
            "nrecr0d" => { validate_parameter("NRECR0D", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p327 = value; self.mark_param_given(327); self.recompute_instance_static(); Ok(()) }
            "isbjt" => { validate_parameter("ISBJT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p328 = value; self.mark_param_given(328); self.recompute_instance_static(); Ok(()) }
            "idbjt" => { validate_parameter("IDBJT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p329 = value; self.mark_param_given(329); self.recompute_instance_static(); Ok(()) }
            "isdif" => { validate_parameter("ISDIF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p330 = value; self.mark_param_given(330); self.recompute_instance_static(); Ok(()) }
            "iddif" => { validate_parameter("IDDIF", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p331 = value; self.mark_param_given(331); self.recompute_instance_static(); Ok(()) }
            "isrec" => { validate_parameter("ISREC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p332 = value; self.mark_param_given(332); self.recompute_instance_static(); Ok(()) }
            "idrec" => { validate_parameter("IDREC", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p333 = value; self.mark_param_given(333); self.recompute_instance_static(); Ok(()) }
            "istun" => { validate_parameter("ISTUN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p334 = value; self.mark_param_given(334); self.recompute_instance_static(); Ok(()) }
            "idtun" => { validate_parameter("IDTUN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p335 = value; self.mark_param_given(335); self.recompute_instance_static(); Ok(()) }
            "ln" => { validate_parameter("LN", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p336 = value; self.mark_param_given(336); self.recompute_instance_static(); Ok(()) }
            "vrec0" => { validate_finite_parameter("VREC0", value)?; self.params.p337 = value; self.mark_param_given(337); self.recompute_instance_static(); Ok(()) }
            "vrec0d" => { validate_finite_parameter("VREC0D", value)?; self.params.p338 = value; self.mark_param_given(338); self.recompute_instance_static(); Ok(()) }
            "vtun0" => { validate_finite_parameter("VTUN0", value)?; self.params.p339 = value; self.mark_param_given(339); self.recompute_instance_static(); Ok(()) }
            "vtun0d" => { validate_finite_parameter("VTUN0D", value)?; self.params.p340 = value; self.mark_param_given(340); self.recompute_instance_static(); Ok(()) }
            "nbjt" => { validate_finite_parameter("NBJT", value)?; self.params.p341 = value; self.mark_param_given(341); self.recompute_instance_static(); Ok(()) }
            "lbjt0" => { validate_finite_parameter("LBJT0", value)?; self.params.p342 = value; self.mark_param_given(342); self.recompute_instance_static(); Ok(()) }
            "ldif0" => { validate_finite_parameter("LDIF0", value)?; self.params.p343 = value; self.mark_param_given(343); self.recompute_instance_static(); Ok(()) }
            "vabjt" => { validate_finite_parameter("VABJT", value)?; self.params.p344 = value; self.mark_param_given(344); self.recompute_instance_static(); Ok(()) }
            "aely" => { validate_finite_parameter("AELY", value)?; self.params.p345 = value; self.mark_param_given(345); self.recompute_instance_static(); Ok(()) }
            "ahli" => { validate_finite_parameter("AHLI", value)?; self.params.p346 = value; self.mark_param_given(346); self.recompute_instance_static(); Ok(()) }
            "ahlid" => { validate_finite_parameter("AHLID", value)?; self.params.p347 = value; self.mark_param_given(347); self.recompute_instance_static(); Ok(()) }
            "rbody" => { validate_parameter("RBODY", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p348 = value; self.mark_param_given(348); self.recompute_instance_static(); Ok(()) }
            "rbsh" => { validate_parameter("RBSH", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p349 = value; self.mark_param_given(349); self.recompute_instance_static(); Ok(()) }
            "cgeo" => { validate_finite_parameter("CGEO", value)?; self.params.p350 = value; self.mark_param_given(350); self.recompute_instance_static(); Ok(()) }
            "tt" => { validate_parameter("TT", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p351 = value; self.mark_param_given(351); self.recompute_instance_static(); Ok(()) }
            "ndif" => { validate_finite_parameter("NDIF", value)?; self.params.p352 = value; self.mark_param_given(352); self.recompute_instance_static(); Ok(()) }
            "vsdfb" => { validate_finite_parameter("VSDFB", value)?; self.params.p353 = value; self.mark_param_given(353); self.recompute_instance_static(); Ok(()) }
            "vsdth" => { validate_finite_parameter("VSDTH", value)?; self.params.p354 = value; self.mark_param_given(354); self.recompute_instance_static(); Ok(()) }
            "csdmin" => { validate_finite_parameter("CSDMIN", value)?; self.params.p355 = value; self.mark_param_given(355); self.recompute_instance_static(); Ok(()) }
            "asd" => { validate_parameter("ASD", value, Some((0.0, "0.0")), true, Some((1.0, "1.0")), true, &[])?; self.params.p356 = value; self.mark_param_given(356); self.recompute_instance_static(); Ok(()) }
            "csdesw" => { validate_finite_parameter("CSDESW", value)?; self.params.p357 = value; self.mark_param_given(357); self.recompute_instance_static(); Ok(()) }
            "ntrecf" => { validate_finite_parameter("NTRECF", value)?; self.params.p358 = value; self.mark_param_given(358); self.recompute_instance_static(); Ok(()) }
            "ntrecr" => { validate_finite_parameter("NTRECR", value)?; self.params.p359 = value; self.mark_param_given(359); self.recompute_instance_static(); Ok(()) }
            "dlcb" => { validate_finite_parameter("DLCB", value)?; self.params.p360 = value; self.mark_param_given(360); self.recompute_instance_static(); Ok(()) }
            "fbody" => { validate_finite_parameter("FBODY", value)?; self.params.p361 = value; self.mark_param_given(361); self.recompute_instance_static(); Ok(()) }
            "tcjswg" => { validate_finite_parameter("TCJSWG", value)?; self.params.p362 = value; self.mark_param_given(362); self.recompute_instance_static(); Ok(()) }
            "tpbswg" => { validate_finite_parameter("TPBSWG", value)?; self.params.p363 = value; self.mark_param_given(363); self.recompute_instance_static(); Ok(()) }
            "tcjswgd" => { validate_finite_parameter("TCJSWGD", value)?; self.params.p364 = value; self.mark_param_given(364); self.recompute_instance_static(); Ok(()) }
            "tpbswgd" => { validate_finite_parameter("TPBSWGD", value)?; self.params.p365 = value; self.mark_param_given(365); self.recompute_instance_static(); Ok(()) }
            "acde" => { validate_finite_parameter("ACDE", value)?; self.params.p366 = value; self.mark_param_given(366); self.recompute_instance_static(); Ok(()) }
            "moin" => { validate_finite_parameter("MOIN", value)?; self.params.p367 = value; self.mark_param_given(367); self.recompute_instance_static(); Ok(()) }
            "noff" => { validate_parameter("NOFF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p368 = value; self.mark_param_given(368); self.recompute_instance_static(); Ok(()) }
            "noff2" => { validate_parameter("NOFF2", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p369 = value; self.mark_param_given(369); self.recompute_instance_static(); Ok(()) }
            "delvt" => { validate_finite_parameter("DELVT", value)?; self.params.p370 = value; self.mark_param_given(370); self.recompute_instance_static(); Ok(()) }
            "kb1" => { validate_finite_parameter("KB1", value)?; self.params.p371 = value; self.mark_param_given(371); self.recompute_instance_static(); Ok(()) }
            "dlbg" => { validate_finite_parameter("DLBG", value)?; self.params.p372 = value; self.mark_param_given(372); self.recompute_instance_static(); Ok(()) }
            "cfrcoeff" => { validate_finite_parameter("CFRCOEFF", value)?; self.params.p373 = value; self.mark_param_given(373); self.recompute_instance_static(); Ok(()) }
            "igbmod" => { validate_parameter("IGBMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p374 = value; self.mark_param_given(374); self.recompute_instance_static(); Ok(()) }
            "igcmod" => { validate_parameter("IGCMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p375 = value; self.mark_param_given(375); self.recompute_instance_static(); Ok(()) }
            "toxqm" => { validate_parameter("TOXQM", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p376 = value; self.mark_param_given(376); self.recompute_instance_static(); Ok(()) }
            "wth0" => { validate_parameter("WTH0", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p377 = value; self.mark_param_given(377); self.recompute_instance_static(); Ok(()) }
            "rhalo" => { validate_parameter("RHALO", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p378 = value; self.mark_param_given(378); self.recompute_instance_static(); Ok(()) }
            "ntox" => { validate_finite_parameter("NTOX", value)?; self.params.p379 = value; self.mark_param_given(379); self.recompute_instance_static(); Ok(()) }
            "toxref" => { validate_parameter("TOXREF", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p380 = value; self.mark_param_given(380); self.recompute_instance_static(); Ok(()) }
            "ebg" => { validate_finite_parameter("EBG", value)?; self.params.p381 = value; self.mark_param_given(381); self.recompute_instance_static(); Ok(()) }
            "vevb" => { validate_parameter("VEVB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p382 = value; self.mark_param_given(382); self.recompute_instance_static(); Ok(()) }
            "alphagb1" => { validate_finite_parameter("ALPHAGB1", value)?; self.params.p383 = value; self.mark_param_given(383); self.recompute_instance_static(); Ok(()) }
            "alphagb1_t" => { validate_finite_parameter("ALPHAGB1_T", value)?; self.params.p384 = value; self.mark_param_given(384); self.recompute_instance_static(); Ok(()) }
            "betagb1" => { validate_finite_parameter("BETAGB1", value)?; self.params.p385 = value; self.mark_param_given(385); self.recompute_instance_static(); Ok(()) }
            "vgb1" => { validate_finite_parameter("VGB1", value)?; self.params.p386 = value; self.mark_param_given(386); self.recompute_instance_static(); Ok(()) }
            "vecb" => { validate_parameter("VECB", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p387 = value; self.mark_param_given(387); self.recompute_instance_static(); Ok(()) }
            "alphagb2" => { validate_finite_parameter("ALPHAGB2", value)?; self.params.p388 = value; self.mark_param_given(388); self.recompute_instance_static(); Ok(()) }
            "alphagb2_t" => { validate_finite_parameter("ALPHAGB2_T", value)?; self.params.p389 = value; self.mark_param_given(389); self.recompute_instance_static(); Ok(()) }
            "betagb2" => { validate_finite_parameter("BETAGB2", value)?; self.params.p390 = value; self.mark_param_given(390); self.recompute_instance_static(); Ok(()) }
            "vgb2" => { validate_finite_parameter("VGB2", value)?; self.params.p391 = value; self.mark_param_given(391); self.recompute_instance_static(); Ok(()) }
            "aigbcp2" => { validate_finite_parameter("AIGBCP2", value)?; self.params.p392 = value; self.mark_param_given(392); self.recompute_instance_static(); Ok(()) }
            "aigbcp2_t" => { validate_finite_parameter("AIGBCP2_T", value)?; self.params.p393 = value; self.mark_param_given(393); self.recompute_instance_static(); Ok(()) }
            "bigbcp2" => { validate_finite_parameter("BIGBCP2", value)?; self.params.p394 = value; self.mark_param_given(394); self.recompute_instance_static(); Ok(()) }
            "cigbcp2" => { validate_finite_parameter("CIGBCP2", value)?; self.params.p395 = value; self.mark_param_given(395); self.recompute_instance_static(); Ok(()) }
            "voxh" => { validate_finite_parameter("VOXH", value)?; self.params.p396 = value; self.mark_param_given(396); self.recompute_instance_static(); Ok(()) }
            "deltavox" => { validate_finite_parameter("DELTAVOX", value)?; self.params.p397 = value; self.mark_param_given(397); self.recompute_instance_static(); Ok(()) }
            "aigc" => { validate_finite_parameter("AIGC", value)?; self.params.p398 = value; self.mark_param_given(398); self.recompute_instance_static(); Ok(()) }
            "aigc1" => { validate_finite_parameter("AIGC1", value)?; self.params.p399 = value; self.mark_param_given(399); self.recompute_instance_static(); Ok(()) }
            "bigc" => { validate_finite_parameter("BIGC", value)?; self.params.p400 = value; self.mark_param_given(400); self.recompute_instance_static(); Ok(()) }
            "cigc" => { validate_finite_parameter("CIGC", value)?; self.params.p401 = value; self.mark_param_given(401); self.recompute_instance_static(); Ok(()) }
            "aigsd" => { validate_finite_parameter("AIGSD", value)?; self.params.p402 = value; self.mark_param_given(402); self.recompute_instance_static(); Ok(()) }
            "aigsd1" => { validate_finite_parameter("AIGSD1", value)?; self.params.p403 = value; self.mark_param_given(403); self.recompute_instance_static(); Ok(()) }
            "bigsd" => { validate_finite_parameter("BIGSD", value)?; self.params.p404 = value; self.mark_param_given(404); self.recompute_instance_static(); Ok(()) }
            "cigsd" => { validate_finite_parameter("CIGSD", value)?; self.params.p405 = value; self.mark_param_given(405); self.recompute_instance_static(); Ok(()) }
            "nigc" => { validate_finite_parameter("NIGC", value)?; self.params.p406 = value; self.mark_param_given(406); self.recompute_instance_static(); Ok(()) }
            "pigcd" => { validate_finite_parameter("PIGCD", value)?; self.params.p407 = value; self.mark_param_given(407); self.recompute_instance_static(); Ok(()) }
            "poxedge" => { validate_finite_parameter("POXEDGE", value)?; self.params.p408 = value; self.mark_param_given(408); self.recompute_instance_static(); Ok(()) }
            "igt" => { validate_finite_parameter("IGT", value)?; self.params.p409 = value; self.mark_param_given(409); self.recompute_instance_static(); Ok(()) }
            "dlcig" => { validate_finite_parameter("DLCIG", value)?; self.params.p410 = value; self.mark_param_given(410); self.recompute_instance_static(); Ok(()) }
            "vbs0pd" => { validate_finite_parameter("VBS0PD", value)?; self.params.p411 = value; self.mark_param_given(411); self.recompute_instance_static(); Ok(()) }
            "vbs0fd" => { validate_finite_parameter("VBS0FD", value)?; self.params.p412 = value; self.mark_param_given(412); self.recompute_instance_static(); Ok(()) }
            "vbsa" => { validate_finite_parameter("VBSA", value)?; self.params.p413 = value; self.mark_param_given(413); self.recompute_instance_static(); Ok(()) }
            "nofffd" => { validate_finite_parameter("NOFFFD", value)?; self.params.p414 = value; self.mark_param_given(414); self.recompute_instance_static(); Ok(()) }
            "vofffd" => { validate_finite_parameter("VOFFFD", value)?; self.params.p415 = value; self.mark_param_given(415); self.recompute_instance_static(); Ok(()) }
            "k1b" => { validate_finite_parameter("K1B", value)?; self.params.p416 = value; self.mark_param_given(416); self.recompute_instance_static(); Ok(()) }
            "k2b" => { validate_finite_parameter("K2B", value)?; self.params.p417 = value; self.mark_param_given(417); self.recompute_instance_static(); Ok(()) }
            "dk2b" => { validate_finite_parameter("DK2B", value)?; self.params.p418 = value; self.mark_param_given(418); self.recompute_instance_static(); Ok(()) }
            "dvbd0" => { validate_finite_parameter("DVBD0", value)?; self.params.p419 = value; self.mark_param_given(419); self.recompute_instance_static(); Ok(()) }
            "dvbd1" => { validate_finite_parameter("DVBD1", value)?; self.params.p420 = value; self.mark_param_given(420); self.recompute_instance_static(); Ok(()) }
            "moinfd" => { validate_finite_parameter("MOINFD", value)?; self.params.p421 = value; self.mark_param_given(421); self.recompute_instance_static(); Ok(()) }
            "xrcrg1" => { validate_finite_parameter("XRCRG1", value)?; self.params.p422 = value; self.mark_param_given(422); self.recompute_instance_static(); Ok(()) }
            "xrcrg2" => { validate_finite_parameter("XRCRG2", value)?; self.params.p423 = value; self.mark_param_given(423); self.recompute_instance_static(); Ok(()) }
            "rshg" => { validate_parameter("RSHG", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p424 = value; self.mark_param_given(424); self.recompute_instance_static(); Ok(()) }
            "ngcon" => { validate_parameter("NGCON", value, Some((1.0, "1.0")), false, None, true, &[])?; self.params.p425 = value; self.mark_param_given(425); self.recompute_instance_static(); Ok(()) }
            "rver" => { validate_parameter("RVER", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p426 = value; self.mark_param_given(426); self.recompute_instance_static(); Ok(()) }
            "xgw" => { validate_finite_parameter("XGW", value)?; self.params.p427 = value; self.mark_param_given(427); self.recompute_instance_static(); Ok(()) }
            "xgl" => { validate_finite_parameter("XGL", value)?; self.params.p428 = value; self.mark_param_given(428); self.recompute_instance_static(); Ok(()) }
            "rdsmod" => { validate_parameter("RDSMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p429 = value; self.mark_param_given(429); self.recompute_instance_static(); Ok(()) }
            "ids0multmod" => { validate_parameter("IDS0MULTMOD", value, Some((0.0, "0.0")), false, Some((2.0, "2.0")), false, &[])?; self.params.p430 = value; self.mark_param_given(430); self.recompute_instance_static(); Ok(()) }
            "minr" => { validate_parameter("MINR", value, Some((0.0, "0.0")), false, None, true, &[])?; self.params.p431 = value; self.mark_param_given(431); self.recompute_instance_static(); Ok(()) }
            "fdmod" => { validate_parameter("FDMOD", value, Some((0.0, "0.0")), false, Some((1.0, "1.0")), false, &[])?; self.params.p432 = value; self.mark_param_given(432); self.recompute_instance_static(); Ok(()) }
            "vsce" => { validate_finite_parameter("VSCE", value)?; self.params.p433 = value; self.mark_param_given(433); self.recompute_instance_static(); Ok(()) }
            "cdsbs" => { validate_finite_parameter("CDSBS", value)?; self.params.p434 = value; self.mark_param_given(434); self.recompute_instance_static(); Ok(()) }
            "minvcv" => { validate_finite_parameter("MINVCV", value)?; self.params.p435 = value; self.mark_param_given(435); self.recompute_instance_static(); Ok(()) }
            "lminvcv" => { validate_finite_parameter("LMINVCV", value)?; self.params.p436 = value; self.mark_param_given(436); self.recompute_instance_static(); Ok(()) }
            "wminvcv" => { validate_finite_parameter("WMINVCV", value)?; self.params.p437 = value; self.mark_param_given(437); self.recompute_instance_static(); Ok(()) }
            "pminvcv" => { validate_finite_parameter("PMINVCV", value)?; self.params.p438 = value; self.mark_param_given(438); self.recompute_instance_static(); Ok(()) }
            "voffcv" => { validate_finite_parameter("VOFFCV", value)?; self.params.p439 = value; self.mark_param_given(439); self.recompute_instance_static(); Ok(()) }
            "lvoffcv" => { validate_finite_parameter("LVOFFCV", value)?; self.params.p440 = value; self.mark_param_given(440); self.recompute_instance_static(); Ok(()) }
            "wvoffcv" => { validate_finite_parameter("WVOFFCV", value)?; self.params.p441 = value; self.mark_param_given(441); self.recompute_instance_static(); Ok(()) }
            "pvoffcv" => { validate_finite_parameter("PVOFFCV", value)?; self.params.p442 = value; self.mark_param_given(442); self.recompute_instance_static(); Ok(()) }
            "lxj" => { validate_finite_parameter("LXJ", value)?; self.params.p443 = value; self.mark_param_given(443); self.recompute_instance_static(); Ok(()) }
            "lalphagb1" => { validate_finite_parameter("LALPHAGB1", value)?; self.params.p444 = value; self.mark_param_given(444); self.recompute_instance_static(); Ok(()) }
            "lalphagb1_t" => { validate_finite_parameter("LALPHAGB1_T", value)?; self.params.p445 = value; self.mark_param_given(445); self.recompute_instance_static(); Ok(()) }
            "lbetagb1" => { validate_finite_parameter("LBETAGB1", value)?; self.params.p446 = value; self.mark_param_given(446); self.recompute_instance_static(); Ok(()) }
            "lalphagb2" => { validate_finite_parameter("LALPHAGB2", value)?; self.params.p447 = value; self.mark_param_given(447); self.recompute_instance_static(); Ok(()) }
            "lalphagb2_t" => { validate_finite_parameter("LALPHAGB2_T", value)?; self.params.p448 = value; self.mark_param_given(448); self.recompute_instance_static(); Ok(()) }
            "lbetagb2" => { validate_finite_parameter("LBETAGB2", value)?; self.params.p449 = value; self.mark_param_given(449); self.recompute_instance_static(); Ok(()) }
            "laigbcp2" => { validate_finite_parameter("LAIGBCP2", value)?; self.params.p450 = value; self.mark_param_given(450); self.recompute_instance_static(); Ok(()) }
            "laigbcp2_t" => { validate_finite_parameter("LAIGBCP2_T", value)?; self.params.p451 = value; self.mark_param_given(451); self.recompute_instance_static(); Ok(()) }
            "lbigbcp2" => { validate_finite_parameter("LBIGBCP2", value)?; self.params.p452 = value; self.mark_param_given(452); self.recompute_instance_static(); Ok(()) }
            "lcigbcp2" => { validate_finite_parameter("LCIGBCP2", value)?; self.params.p453 = value; self.mark_param_given(453); self.recompute_instance_static(); Ok(()) }
            "lcgsl" => { validate_finite_parameter("LCGSL", value)?; self.params.p454 = value; self.mark_param_given(454); self.recompute_instance_static(); Ok(()) }
            "lcgdl" => { validate_finite_parameter("LCGDL", value)?; self.params.p455 = value; self.mark_param_given(455); self.recompute_instance_static(); Ok(()) }
            "lckappa" => { validate_finite_parameter("LCKAPPA", value)?; self.params.p456 = value; self.mark_param_given(456); self.recompute_instance_static(); Ok(()) }
            "lndif" => { validate_finite_parameter("LNDIF", value)?; self.params.p457 = value; self.mark_param_given(457); self.recompute_instance_static(); Ok(()) }
            "lute" => { validate_finite_parameter("LUTE", value)?; self.params.p458 = value; self.mark_param_given(458); self.recompute_instance_static(); Ok(()) }
            "lkt1" => { validate_finite_parameter("LKT1", value)?; self.params.p459 = value; self.mark_param_given(459); self.recompute_instance_static(); Ok(()) }
            "lkt1l" => { validate_finite_parameter("LKT1L", value)?; self.params.p460 = value; self.mark_param_given(460); self.recompute_instance_static(); Ok(()) }
            "lkt2" => { validate_finite_parameter("LKT2", value)?; self.params.p461 = value; self.mark_param_given(461); self.recompute_instance_static(); Ok(()) }
            "lua1" => { validate_finite_parameter("LUA1", value)?; self.params.p462 = value; self.mark_param_given(462); self.recompute_instance_static(); Ok(()) }
            "lub1" => { validate_finite_parameter("LUB1", value)?; self.params.p463 = value; self.mark_param_given(463); self.recompute_instance_static(); Ok(()) }
            "luc1" => { validate_finite_parameter("LUC1", value)?; self.params.p464 = value; self.mark_param_given(464); self.recompute_instance_static(); Ok(()) }
            "lat" => { validate_finite_parameter("LAT", value)?; self.params.p465 = value; self.mark_param_given(465); self.recompute_instance_static(); Ok(()) }
            "lprt" => { validate_finite_parameter("LPRT", value)?; self.params.p466 = value; self.mark_param_given(466); self.recompute_instance_static(); Ok(()) }
            "lntrecf" => { validate_finite_parameter("LNTRECF", value)?; self.params.p467 = value; self.mark_param_given(467); self.recompute_instance_static(); Ok(()) }
            "lntrecr" => { validate_finite_parameter("LNTRECR", value)?; self.params.p468 = value; self.mark_param_given(468); self.recompute_instance_static(); Ok(()) }
            "lxbjt" => { validate_finite_parameter("LXBJT", value)?; self.params.p469 = value; self.mark_param_given(469); self.recompute_instance_static(); Ok(()) }
            "lxdif" => { validate_finite_parameter("LXDIF", value)?; self.params.p470 = value; self.mark_param_given(470); self.recompute_instance_static(); Ok(()) }
            "lxrec" => { validate_finite_parameter("LXREC", value)?; self.params.p471 = value; self.mark_param_given(471); self.recompute_instance_static(); Ok(()) }
            "lxtun" => { validate_finite_parameter("LXTUN", value)?; self.params.p472 = value; self.mark_param_given(472); self.recompute_instance_static(); Ok(()) }
            "lxdifd" => { validate_finite_parameter("LXDIFD", value)?; self.params.p473 = value; self.mark_param_given(473); self.recompute_instance_static(); Ok(()) }
            "lxrecd" => { validate_finite_parameter("LXRECD", value)?; self.params.p474 = value; self.mark_param_given(474); self.recompute_instance_static(); Ok(()) }
            "lxtund" => { validate_finite_parameter("LXTUND", value)?; self.params.p475 = value; self.mark_param_given(475); self.recompute_instance_static(); Ok(()) }
            "laigc" => { validate_finite_parameter("LAIGC", value)?; self.params.p476 = value; self.mark_param_given(476); self.recompute_instance_static(); Ok(()) }
            "laigc1" => { validate_finite_parameter("LAIGC1", value)?; self.params.p477 = value; self.mark_param_given(477); self.recompute_instance_static(); Ok(()) }
            "lbigc" => { validate_finite_parameter("LBIGC", value)?; self.params.p478 = value; self.mark_param_given(478); self.recompute_instance_static(); Ok(()) }
            "lcigc" => { validate_finite_parameter("LCIGC", value)?; self.params.p479 = value; self.mark_param_given(479); self.recompute_instance_static(); Ok(()) }
            "laigsd" => { validate_finite_parameter("LAIGSD", value)?; self.params.p480 = value; self.mark_param_given(480); self.recompute_instance_static(); Ok(()) }
            "laigsd1" => { validate_finite_parameter("LAIGSD1", value)?; self.params.p481 = value; self.mark_param_given(481); self.recompute_instance_static(); Ok(()) }
            "lbigsd" => { validate_finite_parameter("LBIGSD", value)?; self.params.p482 = value; self.mark_param_given(482); self.recompute_instance_static(); Ok(()) }
            "lcigsd" => { validate_finite_parameter("LCIGSD", value)?; self.params.p483 = value; self.mark_param_given(483); self.recompute_instance_static(); Ok(()) }
            "lnigc" => { validate_finite_parameter("LNIGC", value)?; self.params.p484 = value; self.mark_param_given(484); self.recompute_instance_static(); Ok(()) }
            "lpigcd" => { validate_finite_parameter("LPIGCD", value)?; self.params.p485 = value; self.mark_param_given(485); self.recompute_instance_static(); Ok(()) }
            "lpoxedge" => { validate_finite_parameter("LPOXEDGE", value)?; self.params.p486 = value; self.mark_param_given(486); self.recompute_instance_static(); Ok(()) }
            "ligt" => { validate_finite_parameter("LIGT", value)?; self.params.p487 = value; self.mark_param_given(487); self.recompute_instance_static(); Ok(()) }
            "lnch" => { validate_finite_parameter("LNCH", value)?; self.params.p488 = value; self.mark_param_given(488); self.recompute_instance_static(); Ok(()) }
            "lnsub" => { validate_finite_parameter("LNSUB", value)?; self.params.p489 = value; self.mark_param_given(489); self.recompute_instance_static(); Ok(()) }
            "lngate" => { validate_finite_parameter("LNGATE", value)?; self.params.p490 = value; self.mark_param_given(490); self.recompute_instance_static(); Ok(()) }
            "lnsd" => { validate_finite_parameter("LNSD", value)?; self.params.p491 = value; self.mark_param_given(491); self.recompute_instance_static(); Ok(()) }
            "lvth0" => { validate_finite_parameter("LVTH0", value)?; self.params.p492 = value; self.mark_param_given(492); self.recompute_instance_static(); Ok(()) }
            "lvfb" => { validate_finite_parameter("LVFB", value)?; self.params.p493 = value; self.mark_param_given(493); self.recompute_instance_static(); Ok(()) }
            "lk1" => { validate_finite_parameter("LK1", value)?; self.params.p494 = value; self.mark_param_given(494); self.recompute_instance_static(); Ok(()) }
            "lk1w1" => { validate_finite_parameter("LK1W1", value)?; self.params.p495 = value; self.mark_param_given(495); self.recompute_instance_static(); Ok(()) }
            "lk1w2" => { validate_finite_parameter("LK1W2", value)?; self.params.p496 = value; self.mark_param_given(496); self.recompute_instance_static(); Ok(()) }
            "lk2" => { validate_finite_parameter("LK2", value)?; self.params.p497 = value; self.mark_param_given(497); self.recompute_instance_static(); Ok(()) }
            "lk3" => { validate_finite_parameter("LK3", value)?; self.params.p498 = value; self.mark_param_given(498); self.recompute_instance_static(); Ok(()) }
            "lk3b" => { validate_finite_parameter("LK3B", value)?; self.params.p499 = value; self.mark_param_given(499); self.recompute_instance_static(); Ok(()) }
            "lkb1" => { validate_finite_parameter("LKB1", value)?; self.params.p500 = value; self.mark_param_given(500); self.recompute_instance_static(); Ok(()) }
            "lw0" => { validate_finite_parameter("LW0", value)?; self.params.p501 = value; self.mark_param_given(501); self.recompute_instance_static(); Ok(()) }
            "llpeb" => { validate_finite_parameter("LLPEB", value)?; self.params.p502 = value; self.mark_param_given(502); self.recompute_instance_static(); Ok(()) }
            "ldvt0" => { validate_finite_parameter("LDVT0", value)?; self.params.p503 = value; self.mark_param_given(503); self.recompute_instance_static(); Ok(()) }
            "ldvt1" => { validate_finite_parameter("LDVT1", value)?; self.params.p504 = value; self.mark_param_given(504); self.recompute_instance_static(); Ok(()) }
            "ldvt2" => { validate_finite_parameter("LDVT2", value)?; self.params.p505 = value; self.mark_param_given(505); self.recompute_instance_static(); Ok(()) }
            "ldvt0w" => { validate_finite_parameter("LDVT0W", value)?; self.params.p506 = value; self.mark_param_given(506); self.recompute_instance_static(); Ok(()) }
            "ldvt1w" => { validate_finite_parameter("LDVT1W", value)?; self.params.p507 = value; self.mark_param_given(507); self.recompute_instance_static(); Ok(()) }
            "ldvt2w" => { validate_finite_parameter("LDVT2W", value)?; self.params.p508 = value; self.mark_param_given(508); self.recompute_instance_static(); Ok(()) }
            "lu0" => { validate_finite_parameter("LU0", value)?; self.params.p509 = value; self.mark_param_given(509); self.recompute_instance_static(); Ok(()) }
            "leu" => { validate_finite_parameter("LEU", value)?; self.params.p510 = value; self.mark_param_given(510); self.recompute_instance_static(); Ok(()) }
            "lua" => { validate_finite_parameter("LUA", value)?; self.params.p511 = value; self.mark_param_given(511); self.recompute_instance_static(); Ok(()) }
            "lub" => { validate_finite_parameter("LUB", value)?; self.params.p512 = value; self.mark_param_given(512); self.recompute_instance_static(); Ok(()) }
            "luc" => { validate_finite_parameter("LUC", value)?; self.params.p513 = value; self.mark_param_given(513); self.recompute_instance_static(); Ok(()) }
            "lud" => { validate_finite_parameter("LUD", value)?; self.params.p514 = value; self.mark_param_given(514); self.recompute_instance_static(); Ok(()) }
            "lud1" => { validate_finite_parameter("LUD1", value)?; self.params.p515 = value; self.mark_param_given(515); self.recompute_instance_static(); Ok(()) }
            "lucste" => { validate_finite_parameter("LUCSTE", value)?; self.params.p516 = value; self.mark_param_given(516); self.recompute_instance_static(); Ok(()) }
            "lucs" => { validate_finite_parameter("LUCS", value)?; self.params.p517 = value; self.mark_param_given(517); self.recompute_instance_static(); Ok(()) }
            "lvsat" => { validate_finite_parameter("LVSAT", value)?; self.params.p518 = value; self.mark_param_given(518); self.recompute_instance_static(); Ok(()) }
            "la0" => { validate_finite_parameter("LA0", value)?; self.params.p519 = value; self.mark_param_given(519); self.recompute_instance_static(); Ok(()) }
            "lags" => { validate_finite_parameter("LAGS", value)?; self.params.p520 = value; self.mark_param_given(520); self.recompute_instance_static(); Ok(()) }
            "lb0" => { validate_finite_parameter("LB0", value)?; self.params.p521 = value; self.mark_param_given(521); self.recompute_instance_static(); Ok(()) }
            "lb1" => { validate_finite_parameter("LB1", value)?; self.params.p522 = value; self.mark_param_given(522); self.recompute_instance_static(); Ok(()) }
            "lketa" => { validate_finite_parameter("LKETA", value)?; self.params.p523 = value; self.mark_param_given(523); self.recompute_instance_static(); Ok(()) }
            "lketas" => { validate_finite_parameter("LKETAS", value)?; self.params.p524 = value; self.mark_param_given(524); self.recompute_instance_static(); Ok(()) }
            "la1" => { validate_finite_parameter("LA1", value)?; self.params.p525 = value; self.mark_param_given(525); self.recompute_instance_static(); Ok(()) }
            "la2" => { validate_finite_parameter("LA2", value)?; self.params.p526 = value; self.mark_param_given(526); self.recompute_instance_static(); Ok(()) }
            "lrdsw" => { validate_finite_parameter("LRDSW", value)?; self.params.p527 = value; self.mark_param_given(527); self.recompute_instance_static(); Ok(()) }
            "lrsw" => { validate_finite_parameter("LRSW", value)?; self.params.p528 = value; self.mark_param_given(528); self.recompute_instance_static(); Ok(()) }
            "lrdw" => { validate_finite_parameter("LRDW", value)?; self.params.p529 = value; self.mark_param_given(529); self.recompute_instance_static(); Ok(()) }
            "lprwb" => { validate_finite_parameter("LPRWB", value)?; self.params.p530 = value; self.mark_param_given(530); self.recompute_instance_static(); Ok(()) }
            "lprwe" => { validate_finite_parameter("LPRWE", value)?; self.params.p531 = value; self.mark_param_given(531); self.recompute_instance_static(); Ok(()) }
            "lprwg" => { validate_finite_parameter("LPRWG", value)?; self.params.p532 = value; self.mark_param_given(532); self.recompute_instance_static(); Ok(()) }
            "lwr" => { validate_finite_parameter("LWR", value)?; self.params.p533 = value; self.mark_param_given(533); self.recompute_instance_static(); Ok(()) }
            "lnfactor" => { validate_finite_parameter("LNFACTOR", value)?; self.params.p534 = value; self.mark_param_given(534); self.recompute_instance_static(); Ok(()) }
            "ldwg" => { validate_finite_parameter("LDWG", value)?; self.params.p535 = value; self.mark_param_given(535); self.recompute_instance_static(); Ok(()) }
            "ldwb" => { validate_finite_parameter("LDWB", value)?; self.params.p536 = value; self.mark_param_given(536); self.recompute_instance_static(); Ok(()) }
            "lvoff" => { validate_finite_parameter("LVOFF", value)?; self.params.p537 = value; self.mark_param_given(537); self.recompute_instance_static(); Ok(()) }
            "leta0" => { validate_finite_parameter("LETA0", value)?; self.params.p538 = value; self.mark_param_given(538); self.recompute_instance_static(); Ok(()) }
            "letab" => { validate_finite_parameter("LETAB", value)?; self.params.p539 = value; self.mark_param_given(539); self.recompute_instance_static(); Ok(()) }
            "leta0cv" => { validate_finite_parameter("LETA0CV", value)?; self.params.p540 = value; self.mark_param_given(540); self.recompute_instance_static(); Ok(()) }
            "letabcv" => { validate_finite_parameter("LETABCV", value)?; self.params.p541 = value; self.mark_param_given(541); self.recompute_instance_static(); Ok(()) }
            "ldsub" => { validate_finite_parameter("LDSUB", value)?; self.params.p542 = value; self.mark_param_given(542); self.recompute_instance_static(); Ok(()) }
            "lcit" => { validate_finite_parameter("LCIT", value)?; self.params.p543 = value; self.mark_param_given(543); self.recompute_instance_static(); Ok(()) }
            "lcdsc" => { validate_finite_parameter("LCDSC", value)?; self.params.p544 = value; self.mark_param_given(544); self.recompute_instance_static(); Ok(()) }
            "lcdscb" => { validate_finite_parameter("LCDSCB", value)?; self.params.p545 = value; self.mark_param_given(545); self.recompute_instance_static(); Ok(()) }
            "lcdscd" => { validate_finite_parameter("LCDSCD", value)?; self.params.p546 = value; self.mark_param_given(546); self.recompute_instance_static(); Ok(()) }
            "lpclm" => { validate_finite_parameter("LPCLM", value)?; self.params.p547 = value; self.mark_param_given(547); self.recompute_instance_static(); Ok(()) }
            "lpdiblc1" => { validate_finite_parameter("LPDIBLC1", value)?; self.params.p548 = value; self.mark_param_given(548); self.recompute_instance_static(); Ok(()) }
            "lpdiblc2" => { validate_finite_parameter("LPDIBLC2", value)?; self.params.p549 = value; self.mark_param_given(549); self.recompute_instance_static(); Ok(()) }
            "lpdiblcb" => { validate_finite_parameter("LPDIBLCB", value)?; self.params.p550 = value; self.mark_param_given(550); self.recompute_instance_static(); Ok(()) }
            "ldrout" => { validate_finite_parameter("LDROUT", value)?; self.params.p551 = value; self.mark_param_given(551); self.recompute_instance_static(); Ok(()) }
            "lpvag" => { validate_finite_parameter("LPVAG", value)?; self.params.p552 = value; self.mark_param_given(552); self.recompute_instance_static(); Ok(()) }
            "ldelta" => { validate_finite_parameter("LDELTA", value)?; self.params.p553 = value; self.mark_param_given(553); self.recompute_instance_static(); Ok(()) }
            "lalpha0" => { validate_finite_parameter("LALPHA0", value)?; self.params.p554 = value; self.mark_param_given(554); self.recompute_instance_static(); Ok(()) }
            "lfbjtii" => { validate_finite_parameter("LFBJTII", value)?; self.params.p555 = value; self.mark_param_given(555); self.recompute_instance_static(); Ok(()) }
            "labjtii" => { validate_finite_parameter("LABJTII", value)?; self.params.p556 = value; self.mark_param_given(556); self.recompute_instance_static(); Ok(()) }
            "lcbjtii" => { validate_finite_parameter("LCBJTII", value)?; self.params.p557 = value; self.mark_param_given(557); self.recompute_instance_static(); Ok(()) }
            "lebjtii" => { validate_finite_parameter("LEBJTII", value)?; self.params.p558 = value; self.mark_param_given(558); self.recompute_instance_static(); Ok(()) }
            "lmbjtii" => { validate_finite_parameter("LMBJTII", value)?; self.params.p559 = value; self.mark_param_given(559); self.recompute_instance_static(); Ok(()) }
            "lvbci" => { validate_finite_parameter("LVBCI", value)?; self.params.p560 = value; self.mark_param_given(560); self.recompute_instance_static(); Ok(()) }
            "lbeta0" => { validate_finite_parameter("LBETA0", value)?; self.params.p561 = value; self.mark_param_given(561); self.recompute_instance_static(); Ok(()) }
            "lbeta1" => { validate_finite_parameter("LBETA1", value)?; self.params.p562 = value; self.mark_param_given(562); self.recompute_instance_static(); Ok(()) }
            "lbeta2" => { validate_finite_parameter("LBETA2", value)?; self.params.p563 = value; self.mark_param_given(563); self.recompute_instance_static(); Ok(()) }
            "lvdsatii0" => { validate_finite_parameter("LVDSATII0", value)?; self.params.p564 = value; self.mark_param_given(564); self.recompute_instance_static(); Ok(()) }
            "llii" => { validate_finite_parameter("LLII", value)?; self.params.p565 = value; self.mark_param_given(565); self.recompute_instance_static(); Ok(()) }
            "lesatii" => { validate_finite_parameter("LESATII", value)?; self.params.p566 = value; self.mark_param_given(566); self.recompute_instance_static(); Ok(()) }
            "lsii0" => { validate_finite_parameter("LSII0", value)?; self.params.p567 = value; self.mark_param_given(567); self.recompute_instance_static(); Ok(()) }
            "lsii1" => { validate_finite_parameter("LSII1", value)?; self.params.p568 = value; self.mark_param_given(568); self.recompute_instance_static(); Ok(()) }
            "lsii2" => { validate_finite_parameter("LSII2", value)?; self.params.p569 = value; self.mark_param_given(569); self.recompute_instance_static(); Ok(()) }
            "lsiid" => { validate_finite_parameter("LSIID", value)?; self.params.p570 = value; self.mark_param_given(570); self.recompute_instance_static(); Ok(()) }
            "lagidl" => { validate_finite_parameter("LAGIDL", value)?; self.params.p571 = value; self.mark_param_given(571); self.recompute_instance_static(); Ok(()) }
            "lbgidl" => { validate_finite_parameter("LBGIDL", value)?; self.params.p572 = value; self.mark_param_given(572); self.recompute_instance_static(); Ok(()) }
            "lbgidl1" => { validate_finite_parameter("LBGIDL1", value)?; self.params.p573 = value; self.mark_param_given(573); self.recompute_instance_static(); Ok(()) }
            "lcgidl" => { validate_finite_parameter("LCGIDL", value)?; self.params.p574 = value; self.mark_param_given(574); self.recompute_instance_static(); Ok(()) }
            "lrgidl" => { validate_finite_parameter("LRGIDL", value)?; self.params.p575 = value; self.mark_param_given(575); self.recompute_instance_static(); Ok(()) }
            "lkgidl" => { validate_finite_parameter("LKGIDL", value)?; self.params.p576 = value; self.mark_param_given(576); self.recompute_instance_static(); Ok(()) }
            "lfgidl" => { validate_finite_parameter("LFGIDL", value)?; self.params.p577 = value; self.mark_param_given(577); self.recompute_instance_static(); Ok(()) }
            "lagisl" => { validate_finite_parameter("LAGISL", value)?; self.params.p578 = value; self.mark_param_given(578); self.recompute_instance_static(); Ok(()) }
            "lbgisl" => { validate_finite_parameter("LBGISL", value)?; self.params.p579 = value; self.mark_param_given(579); self.recompute_instance_static(); Ok(()) }
            "lbgisl1" => { validate_finite_parameter("LBGISL1", value)?; self.params.p580 = value; self.mark_param_given(580); self.recompute_instance_static(); Ok(()) }
            "lcgisl" => { validate_finite_parameter("LCGISL", value)?; self.params.p581 = value; self.mark_param_given(581); self.recompute_instance_static(); Ok(()) }
            "lrgisl" => { validate_finite_parameter("LRGISL", value)?; self.params.p582 = value; self.mark_param_given(582); self.recompute_instance_static(); Ok(()) }
            "lkgisl" => { validate_finite_parameter("LKGISL", value)?; self.params.p583 = value; self.mark_param_given(583); self.recompute_instance_static(); Ok(()) }
            "lfgisl" => { validate_finite_parameter("LFGISL", value)?; self.params.p584 = value; self.mark_param_given(584); self.recompute_instance_static(); Ok(()) }
            "lntun" => { validate_finite_parameter("LNTUN", value)?; self.params.p585 = value; self.mark_param_given(585); self.recompute_instance_static(); Ok(()) }
            "lntund" => { validate_finite_parameter("LNTUND", value)?; self.params.p586 = value; self.mark_param_given(586); self.recompute_instance_static(); Ok(()) }
            "lndiode" => { validate_finite_parameter("LNDIODE", value)?; self.params.p587 = value; self.mark_param_given(587); self.recompute_instance_static(); Ok(()) }
            "lndioded" => { validate_finite_parameter("LNDIODED", value)?; self.params.p588 = value; self.mark_param_given(588); self.recompute_instance_static(); Ok(()) }
            "lnrecf0" => { validate_finite_parameter("LNRECF0", value)?; self.params.p589 = value; self.mark_param_given(589); self.recompute_instance_static(); Ok(()) }
            "lnrecf0d" => { validate_finite_parameter("LNRECF0D", value)?; self.params.p590 = value; self.mark_param_given(590); self.recompute_instance_static(); Ok(()) }
            "lnrecr0" => { validate_finite_parameter("LNRECR0", value)?; self.params.p591 = value; self.mark_param_given(591); self.recompute_instance_static(); Ok(()) }
            "lnrecr0d" => { validate_finite_parameter("LNRECR0D", value)?; self.params.p592 = value; self.mark_param_given(592); self.recompute_instance_static(); Ok(()) }
            "lisbjt" => { validate_finite_parameter("LISBJT", value)?; self.params.p593 = value; self.mark_param_given(593); self.recompute_instance_static(); Ok(()) }
            "lidbjt" => { validate_finite_parameter("LIDBJT", value)?; self.params.p594 = value; self.mark_param_given(594); self.recompute_instance_static(); Ok(()) }
            "lisdif" => { validate_finite_parameter("LISDIF", value)?; self.params.p595 = value; self.mark_param_given(595); self.recompute_instance_static(); Ok(()) }
            "liddif" => { validate_finite_parameter("LIDDIF", value)?; self.params.p596 = value; self.mark_param_given(596); self.recompute_instance_static(); Ok(()) }
            "lisrec" => { validate_finite_parameter("LISREC", value)?; self.params.p597 = value; self.mark_param_given(597); self.recompute_instance_static(); Ok(()) }
            "lidrec" => { validate_finite_parameter("LIDREC", value)?; self.params.p598 = value; self.mark_param_given(598); self.recompute_instance_static(); Ok(()) }
            "listun" => { validate_finite_parameter("LISTUN", value)?; self.params.p599 = value; self.mark_param_given(599); self.recompute_instance_static(); Ok(()) }
            "lidtun" => { validate_finite_parameter("LIDTUN", value)?; self.params.p600 = value; self.mark_param_given(600); self.recompute_instance_static(); Ok(()) }
            "lvrec0" => { validate_finite_parameter("LVREC0", value)?; self.params.p601 = value; self.mark_param_given(601); self.recompute_instance_static(); Ok(()) }
            "lvrec0d" => { validate_finite_parameter("LVREC0D", value)?; self.params.p602 = value; self.mark_param_given(602); self.recompute_instance_static(); Ok(()) }
            "lvtun0" => { validate_finite_parameter("LVTUN0", value)?; self.params.p603 = value; self.mark_param_given(603); self.recompute_instance_static(); Ok(()) }
            "lvtun0d" => { validate_finite_parameter("LVTUN0D", value)?; self.params.p604 = value; self.mark_param_given(604); self.recompute_instance_static(); Ok(()) }
            "lnbjt" => { validate_finite_parameter("LNBJT", value)?; self.params.p605 = value; self.mark_param_given(605); self.recompute_instance_static(); Ok(()) }
            "llbjt0" => { validate_finite_parameter("LLBJT0", value)?; self.params.p606 = value; self.mark_param_given(606); self.recompute_instance_static(); Ok(()) }
            "lvabjt" => { validate_finite_parameter("LVABJT", value)?; self.params.p607 = value; self.mark_param_given(607); self.recompute_instance_static(); Ok(()) }
            "laely" => { validate_finite_parameter("LAELY", value)?; self.params.p608 = value; self.mark_param_given(608); self.recompute_instance_static(); Ok(()) }
            "lahli" => { validate_finite_parameter("LAHLI", value)?; self.params.p609 = value; self.mark_param_given(609); self.recompute_instance_static(); Ok(()) }
            "lahlid" => { validate_finite_parameter("LAHLID", value)?; self.params.p610 = value; self.mark_param_given(610); self.recompute_instance_static(); Ok(()) }
            "lvsdfb" => { validate_finite_parameter("LVSDFB", value)?; self.params.p611 = value; self.mark_param_given(611); self.recompute_instance_static(); Ok(()) }
            "lvsdth" => { validate_finite_parameter("LVSDTH", value)?; self.params.p612 = value; self.mark_param_given(612); self.recompute_instance_static(); Ok(()) }
            "ldelvt" => { validate_finite_parameter("LDELVT", value)?; self.params.p613 = value; self.mark_param_given(613); self.recompute_instance_static(); Ok(()) }
            "lacde" => { validate_finite_parameter("LACDE", value)?; self.params.p614 = value; self.mark_param_given(614); self.recompute_instance_static(); Ok(()) }
            "lmoin" => { validate_finite_parameter("LMOIN", value)?; self.params.p615 = value; self.mark_param_given(615); self.recompute_instance_static(); Ok(()) }
            "lnoff" => { validate_finite_parameter("LNOFF", value)?; self.params.p616 = value; self.mark_param_given(616); self.recompute_instance_static(); Ok(()) }
            "lnoff2" => { validate_finite_parameter("LNOFF2", value)?; self.params.p617 = value; self.mark_param_given(617); self.recompute_instance_static(); Ok(()) }
            "lxrcrg1" => { validate_finite_parameter("LXRCRG1", value)?; self.params.p618 = value; self.mark_param_given(618); self.recompute_instance_static(); Ok(()) }
            "lxrcrg2" => { validate_finite_parameter("LXRCRG2", value)?; self.params.p619 = value; self.mark_param_given(619); self.recompute_instance_static(); Ok(()) }
            "lvbsa" => { validate_finite_parameter("LVBSA", value)?; self.params.p620 = value; self.mark_param_given(620); self.recompute_instance_static(); Ok(()) }
            "lvsce" => { validate_finite_parameter("LVSCE", value)?; self.params.p621 = value; self.mark_param_given(621); self.recompute_instance_static(); Ok(()) }
            "lcdsbs" => { validate_finite_parameter("LCDSBS", value)?; self.params.p622 = value; self.mark_param_given(622); self.recompute_instance_static(); Ok(()) }
            "lnofffd" => { validate_finite_parameter("LNOFFFD", value)?; self.params.p623 = value; self.mark_param_given(623); self.recompute_instance_static(); Ok(()) }
            "lvofffd" => { validate_finite_parameter("LVOFFFD", value)?; self.params.p624 = value; self.mark_param_given(624); self.recompute_instance_static(); Ok(()) }
            "lk1b" => { validate_finite_parameter("LK1B", value)?; self.params.p625 = value; self.mark_param_given(625); self.recompute_instance_static(); Ok(()) }
            "lk2b" => { validate_finite_parameter("LK2B", value)?; self.params.p626 = value; self.mark_param_given(626); self.recompute_instance_static(); Ok(()) }
            "ldk2b" => { validate_finite_parameter("LDK2B", value)?; self.params.p627 = value; self.mark_param_given(627); self.recompute_instance_static(); Ok(()) }
            "ldvbd0" => { validate_finite_parameter("LDVBD0", value)?; self.params.p628 = value; self.mark_param_given(628); self.recompute_instance_static(); Ok(()) }
            "ldvbd1" => { validate_finite_parameter("LDVBD1", value)?; self.params.p629 = value; self.mark_param_given(629); self.recompute_instance_static(); Ok(()) }
            "lmoinfd" => { validate_finite_parameter("LMOINFD", value)?; self.params.p630 = value; self.mark_param_given(630); self.recompute_instance_static(); Ok(()) }
            "lvbs0pd" => { validate_finite_parameter("LVBS0PD", value)?; self.params.p631 = value; self.mark_param_given(631); self.recompute_instance_static(); Ok(()) }
            "lvbs0fd" => { validate_finite_parameter("LVBS0FD", value)?; self.params.p632 = value; self.mark_param_given(632); self.recompute_instance_static(); Ok(()) }
            "wxj" => { validate_finite_parameter("WXJ", value)?; self.params.p633 = value; self.mark_param_given(633); self.recompute_instance_static(); Ok(()) }
            "walphagb1" => { validate_finite_parameter("WALPHAGB1", value)?; self.params.p634 = value; self.mark_param_given(634); self.recompute_instance_static(); Ok(()) }
            "walphagb1_t" => { validate_finite_parameter("WALPHAGB1_T", value)?; self.params.p635 = value; self.mark_param_given(635); self.recompute_instance_static(); Ok(()) }
            "wbetagb1" => { validate_finite_parameter("WBETAGB1", value)?; self.params.p636 = value; self.mark_param_given(636); self.recompute_instance_static(); Ok(()) }
            "walphagb2" => { validate_finite_parameter("WALPHAGB2", value)?; self.params.p637 = value; self.mark_param_given(637); self.recompute_instance_static(); Ok(()) }
            "walphagb2_t" => { validate_finite_parameter("WALPHAGB2_T", value)?; self.params.p638 = value; self.mark_param_given(638); self.recompute_instance_static(); Ok(()) }
            "wbetagb2" => { validate_finite_parameter("WBETAGB2", value)?; self.params.p639 = value; self.mark_param_given(639); self.recompute_instance_static(); Ok(()) }
            "waigbcp2" => { validate_finite_parameter("WAIGBCP2", value)?; self.params.p640 = value; self.mark_param_given(640); self.recompute_instance_static(); Ok(()) }
            "waigbcp2_t" => { validate_finite_parameter("WAIGBCP2_T", value)?; self.params.p641 = value; self.mark_param_given(641); self.recompute_instance_static(); Ok(()) }
            "wbigbcp2" => { validate_finite_parameter("WBIGBCP2", value)?; self.params.p642 = value; self.mark_param_given(642); self.recompute_instance_static(); Ok(()) }
            "wcigbcp2" => { validate_finite_parameter("WCIGBCP2", value)?; self.params.p643 = value; self.mark_param_given(643); self.recompute_instance_static(); Ok(()) }
            "wcgsl" => { validate_finite_parameter("WCGSL", value)?; self.params.p644 = value; self.mark_param_given(644); self.recompute_instance_static(); Ok(()) }
            "wcgdl" => { validate_finite_parameter("WCGDL", value)?; self.params.p645 = value; self.mark_param_given(645); self.recompute_instance_static(); Ok(()) }
            "wckappa" => { validate_finite_parameter("WCKAPPA", value)?; self.params.p646 = value; self.mark_param_given(646); self.recompute_instance_static(); Ok(()) }
            "wndif" => { validate_finite_parameter("WNDIF", value)?; self.params.p647 = value; self.mark_param_given(647); self.recompute_instance_static(); Ok(()) }
            "wute" => { validate_finite_parameter("WUTE", value)?; self.params.p648 = value; self.mark_param_given(648); self.recompute_instance_static(); Ok(()) }
            "wkt1" => { validate_finite_parameter("WKT1", value)?; self.params.p649 = value; self.mark_param_given(649); self.recompute_instance_static(); Ok(()) }
            "wkt1l" => { validate_finite_parameter("WKT1L", value)?; self.params.p650 = value; self.mark_param_given(650); self.recompute_instance_static(); Ok(()) }
            "wkt2" => { validate_finite_parameter("WKT2", value)?; self.params.p651 = value; self.mark_param_given(651); self.recompute_instance_static(); Ok(()) }
            "wua1" => { validate_finite_parameter("WUA1", value)?; self.params.p652 = value; self.mark_param_given(652); self.recompute_instance_static(); Ok(()) }
            "wub1" => { validate_finite_parameter("WUB1", value)?; self.params.p653 = value; self.mark_param_given(653); self.recompute_instance_static(); Ok(()) }
            "wuc1" => { validate_finite_parameter("WUC1", value)?; self.params.p654 = value; self.mark_param_given(654); self.recompute_instance_static(); Ok(()) }
            "wat" => { validate_finite_parameter("WAT", value)?; self.params.p655 = value; self.mark_param_given(655); self.recompute_instance_static(); Ok(()) }
            "wprt" => { validate_finite_parameter("WPRT", value)?; self.params.p656 = value; self.mark_param_given(656); self.recompute_instance_static(); Ok(()) }
            "wntrecf" => { validate_finite_parameter("WNTRECF", value)?; self.params.p657 = value; self.mark_param_given(657); self.recompute_instance_static(); Ok(()) }
            "wntrecr" => { validate_finite_parameter("WNTRECR", value)?; self.params.p658 = value; self.mark_param_given(658); self.recompute_instance_static(); Ok(()) }
            "wxbjt" => { validate_finite_parameter("WXBJT", value)?; self.params.p659 = value; self.mark_param_given(659); self.recompute_instance_static(); Ok(()) }
            "wxdif" => { validate_finite_parameter("WXDIF", value)?; self.params.p660 = value; self.mark_param_given(660); self.recompute_instance_static(); Ok(()) }
            "wxrec" => { validate_finite_parameter("WXREC", value)?; self.params.p661 = value; self.mark_param_given(661); self.recompute_instance_static(); Ok(()) }
            "wxtun" => { validate_finite_parameter("WXTUN", value)?; self.params.p662 = value; self.mark_param_given(662); self.recompute_instance_static(); Ok(()) }
            "wxdifd" => { validate_finite_parameter("WXDIFD", value)?; self.params.p663 = value; self.mark_param_given(663); self.recompute_instance_static(); Ok(()) }
            "wxrecd" => { validate_finite_parameter("WXRECD", value)?; self.params.p664 = value; self.mark_param_given(664); self.recompute_instance_static(); Ok(()) }
            "wxtund" => { validate_finite_parameter("WXTUND", value)?; self.params.p665 = value; self.mark_param_given(665); self.recompute_instance_static(); Ok(()) }
            "waigc" => { validate_finite_parameter("WAIGC", value)?; self.params.p666 = value; self.mark_param_given(666); self.recompute_instance_static(); Ok(()) }
            "waigc1" => { validate_finite_parameter("WAIGC1", value)?; self.params.p667 = value; self.mark_param_given(667); self.recompute_instance_static(); Ok(()) }
            "wbigc" => { validate_finite_parameter("WBIGC", value)?; self.params.p668 = value; self.mark_param_given(668); self.recompute_instance_static(); Ok(()) }
            "wcigc" => { validate_finite_parameter("WCIGC", value)?; self.params.p669 = value; self.mark_param_given(669); self.recompute_instance_static(); Ok(()) }
            "waigsd" => { validate_finite_parameter("WAIGSD", value)?; self.params.p670 = value; self.mark_param_given(670); self.recompute_instance_static(); Ok(()) }
            "waigsd1" => { validate_finite_parameter("WAIGSD1", value)?; self.params.p671 = value; self.mark_param_given(671); self.recompute_instance_static(); Ok(()) }
            "wbigsd" => { validate_finite_parameter("WBIGSD", value)?; self.params.p672 = value; self.mark_param_given(672); self.recompute_instance_static(); Ok(()) }
            "wcigsd" => { validate_finite_parameter("WCIGSD", value)?; self.params.p673 = value; self.mark_param_given(673); self.recompute_instance_static(); Ok(()) }
            "wnigc" => { validate_finite_parameter("WNIGC", value)?; self.params.p674 = value; self.mark_param_given(674); self.recompute_instance_static(); Ok(()) }
            "wpigcd" => { validate_finite_parameter("WPIGCD", value)?; self.params.p675 = value; self.mark_param_given(675); self.recompute_instance_static(); Ok(()) }
            "wpoxedge" => { validate_finite_parameter("WPOXEDGE", value)?; self.params.p676 = value; self.mark_param_given(676); self.recompute_instance_static(); Ok(()) }
            "wigt" => { validate_finite_parameter("WIGT", value)?; self.params.p677 = value; self.mark_param_given(677); self.recompute_instance_static(); Ok(()) }
            "wnch" => { validate_finite_parameter("WNCH", value)?; self.params.p678 = value; self.mark_param_given(678); self.recompute_instance_static(); Ok(()) }
            "wnsub" => { validate_finite_parameter("WNSUB", value)?; self.params.p679 = value; self.mark_param_given(679); self.recompute_instance_static(); Ok(()) }
            "wngate" => { validate_finite_parameter("WNGATE", value)?; self.params.p680 = value; self.mark_param_given(680); self.recompute_instance_static(); Ok(()) }
            "wnsd" => { validate_finite_parameter("WNSD", value)?; self.params.p681 = value; self.mark_param_given(681); self.recompute_instance_static(); Ok(()) }
            "wvth0" => { validate_finite_parameter("WVTH0", value)?; self.params.p682 = value; self.mark_param_given(682); self.recompute_instance_static(); Ok(()) }
            "wvfb" => { validate_finite_parameter("WVFB", value)?; self.params.p683 = value; self.mark_param_given(683); self.recompute_instance_static(); Ok(()) }
            "wk1" => { validate_finite_parameter("WK1", value)?; self.params.p684 = value; self.mark_param_given(684); self.recompute_instance_static(); Ok(()) }
            "wk1w1" => { validate_finite_parameter("WK1W1", value)?; self.params.p685 = value; self.mark_param_given(685); self.recompute_instance_static(); Ok(()) }
            "wk1w2" => { validate_finite_parameter("WK1W2", value)?; self.params.p686 = value; self.mark_param_given(686); self.recompute_instance_static(); Ok(()) }
            "wk2" => { validate_finite_parameter("WK2", value)?; self.params.p687 = value; self.mark_param_given(687); self.recompute_instance_static(); Ok(()) }
            "wk3" => { validate_finite_parameter("WK3", value)?; self.params.p688 = value; self.mark_param_given(688); self.recompute_instance_static(); Ok(()) }
            "wk3b" => { validate_finite_parameter("WK3B", value)?; self.params.p689 = value; self.mark_param_given(689); self.recompute_instance_static(); Ok(()) }
            "wkb1" => { validate_finite_parameter("WKB1", value)?; self.params.p690 = value; self.mark_param_given(690); self.recompute_instance_static(); Ok(()) }
            "ww0" => { validate_finite_parameter("WW0", value)?; self.params.p691 = value; self.mark_param_given(691); self.recompute_instance_static(); Ok(()) }
            "wlpeb" => { validate_finite_parameter("WLPEB", value)?; self.params.p692 = value; self.mark_param_given(692); self.recompute_instance_static(); Ok(()) }
            "wdvt0" => { validate_finite_parameter("WDVT0", value)?; self.params.p693 = value; self.mark_param_given(693); self.recompute_instance_static(); Ok(()) }
            "wdvt1" => { validate_finite_parameter("WDVT1", value)?; self.params.p694 = value; self.mark_param_given(694); self.recompute_instance_static(); Ok(()) }
            "wdvt2" => { validate_finite_parameter("WDVT2", value)?; self.params.p695 = value; self.mark_param_given(695); self.recompute_instance_static(); Ok(()) }
            "wdvt0w" => { validate_finite_parameter("WDVT0W", value)?; self.params.p696 = value; self.mark_param_given(696); self.recompute_instance_static(); Ok(()) }
            "wdvt1w" => { validate_finite_parameter("WDVT1W", value)?; self.params.p697 = value; self.mark_param_given(697); self.recompute_instance_static(); Ok(()) }
            "wdvt2w" => { validate_finite_parameter("WDVT2W", value)?; self.params.p698 = value; self.mark_param_given(698); self.recompute_instance_static(); Ok(()) }
            "wu0" => { validate_finite_parameter("WU0", value)?; self.params.p699 = value; self.mark_param_given(699); self.recompute_instance_static(); Ok(()) }
            "weu" => { validate_finite_parameter("WEU", value)?; self.params.p700 = value; self.mark_param_given(700); self.recompute_instance_static(); Ok(()) }
            "wua" => { validate_finite_parameter("WUA", value)?; self.params.p701 = value; self.mark_param_given(701); self.recompute_instance_static(); Ok(()) }
            "wub" => { validate_finite_parameter("WUB", value)?; self.params.p702 = value; self.mark_param_given(702); self.recompute_instance_static(); Ok(()) }
            "wuc" => { validate_finite_parameter("WUC", value)?; self.params.p703 = value; self.mark_param_given(703); self.recompute_instance_static(); Ok(()) }
            "wud" => { validate_finite_parameter("WUD", value)?; self.params.p704 = value; self.mark_param_given(704); self.recompute_instance_static(); Ok(()) }
            "wud1" => { validate_finite_parameter("WUD1", value)?; self.params.p705 = value; self.mark_param_given(705); self.recompute_instance_static(); Ok(()) }
            "wucste" => { validate_finite_parameter("WUCSTE", value)?; self.params.p706 = value; self.mark_param_given(706); self.recompute_instance_static(); Ok(()) }
            "wucs" => { validate_finite_parameter("WUCS", value)?; self.params.p707 = value; self.mark_param_given(707); self.recompute_instance_static(); Ok(()) }
            "wvsat" => { validate_finite_parameter("WVSAT", value)?; self.params.p708 = value; self.mark_param_given(708); self.recompute_instance_static(); Ok(()) }
            "wa0" => { validate_finite_parameter("WA0", value)?; self.params.p709 = value; self.mark_param_given(709); self.recompute_instance_static(); Ok(()) }
            "wags" => { validate_finite_parameter("WAGS", value)?; self.params.p710 = value; self.mark_param_given(710); self.recompute_instance_static(); Ok(()) }
            "wb0" => { validate_finite_parameter("WB0", value)?; self.params.p711 = value; self.mark_param_given(711); self.recompute_instance_static(); Ok(()) }
            "wb1" => { validate_finite_parameter("WB1", value)?; self.params.p712 = value; self.mark_param_given(712); self.recompute_instance_static(); Ok(()) }
            "wketa" => { validate_finite_parameter("WKETA", value)?; self.params.p713 = value; self.mark_param_given(713); self.recompute_instance_static(); Ok(()) }
            "wketas" => { validate_finite_parameter("WKETAS", value)?; self.params.p714 = value; self.mark_param_given(714); self.recompute_instance_static(); Ok(()) }
            "wa1" => { validate_finite_parameter("WA1", value)?; self.params.p715 = value; self.mark_param_given(715); self.recompute_instance_static(); Ok(()) }
            "wa2" => { validate_finite_parameter("WA2", value)?; self.params.p716 = value; self.mark_param_given(716); self.recompute_instance_static(); Ok(()) }
            "wrdsw" => { validate_finite_parameter("WRDSW", value)?; self.params.p717 = value; self.mark_param_given(717); self.recompute_instance_static(); Ok(()) }
            "wrsw" => { validate_finite_parameter("WRSW", value)?; self.params.p718 = value; self.mark_param_given(718); self.recompute_instance_static(); Ok(()) }
            "wrdw" => { validate_finite_parameter("WRDW", value)?; self.params.p719 = value; self.mark_param_given(719); self.recompute_instance_static(); Ok(()) }
            "wprwb" => { validate_finite_parameter("WPRWB", value)?; self.params.p720 = value; self.mark_param_given(720); self.recompute_instance_static(); Ok(()) }
            "wprwe" => { validate_finite_parameter("WPRWE", value)?; self.params.p721 = value; self.mark_param_given(721); self.recompute_instance_static(); Ok(()) }
            "wprwg" => { validate_finite_parameter("WPRWG", value)?; self.params.p722 = value; self.mark_param_given(722); self.recompute_instance_static(); Ok(()) }
            "wwr" => { validate_finite_parameter("WWR", value)?; self.params.p723 = value; self.mark_param_given(723); self.recompute_instance_static(); Ok(()) }
            "wnfactor" => { validate_finite_parameter("WNFACTOR", value)?; self.params.p724 = value; self.mark_param_given(724); self.recompute_instance_static(); Ok(()) }
            "wdwg" => { validate_finite_parameter("WDWG", value)?; self.params.p725 = value; self.mark_param_given(725); self.recompute_instance_static(); Ok(()) }
            "wdwb" => { validate_finite_parameter("WDWB", value)?; self.params.p726 = value; self.mark_param_given(726); self.recompute_instance_static(); Ok(()) }
            "wvoff" => { validate_finite_parameter("WVOFF", value)?; self.params.p727 = value; self.mark_param_given(727); self.recompute_instance_static(); Ok(()) }
            "weta0" => { validate_finite_parameter("WETA0", value)?; self.params.p728 = value; self.mark_param_given(728); self.recompute_instance_static(); Ok(()) }
            "wetab" => { validate_finite_parameter("WETAB", value)?; self.params.p729 = value; self.mark_param_given(729); self.recompute_instance_static(); Ok(()) }
            "weta0cv" => { validate_finite_parameter("WETA0CV", value)?; self.params.p730 = value; self.mark_param_given(730); self.recompute_instance_static(); Ok(()) }
            "wetabcv" => { validate_finite_parameter("WETABCV", value)?; self.params.p731 = value; self.mark_param_given(731); self.recompute_instance_static(); Ok(()) }
            "wdsub" => { validate_finite_parameter("WDSUB", value)?; self.params.p732 = value; self.mark_param_given(732); self.recompute_instance_static(); Ok(()) }
            "wcit" => { validate_finite_parameter("WCIT", value)?; self.params.p733 = value; self.mark_param_given(733); self.recompute_instance_static(); Ok(()) }
            "wcdsc" => { validate_finite_parameter("WCDSC", value)?; self.params.p734 = value; self.mark_param_given(734); self.recompute_instance_static(); Ok(()) }
            "wcdscb" => { validate_finite_parameter("WCDSCB", value)?; self.params.p735 = value; self.mark_param_given(735); self.recompute_instance_static(); Ok(()) }
            "wcdscd" => { validate_finite_parameter("WCDSCD", value)?; self.params.p736 = value; self.mark_param_given(736); self.recompute_instance_static(); Ok(()) }
            "wpclm" => { validate_finite_parameter("WPCLM", value)?; self.params.p737 = value; self.mark_param_given(737); self.recompute_instance_static(); Ok(()) }
            "wpdiblc1" => { validate_finite_parameter("WPDIBLC1", value)?; self.params.p738 = value; self.mark_param_given(738); self.recompute_instance_static(); Ok(()) }
            "wpdiblc2" => { validate_finite_parameter("WPDIBLC2", value)?; self.params.p739 = value; self.mark_param_given(739); self.recompute_instance_static(); Ok(()) }
            "wpdiblcb" => { validate_finite_parameter("WPDIBLCB", value)?; self.params.p740 = value; self.mark_param_given(740); self.recompute_instance_static(); Ok(()) }
            "wdrout" => { validate_finite_parameter("WDROUT", value)?; self.params.p741 = value; self.mark_param_given(741); self.recompute_instance_static(); Ok(()) }
            "wpvag" => { validate_finite_parameter("WPVAG", value)?; self.params.p742 = value; self.mark_param_given(742); self.recompute_instance_static(); Ok(()) }
            "wdelta" => { validate_finite_parameter("WDELTA", value)?; self.params.p743 = value; self.mark_param_given(743); self.recompute_instance_static(); Ok(()) }
            "walpha0" => { validate_finite_parameter("WALPHA0", value)?; self.params.p744 = value; self.mark_param_given(744); self.recompute_instance_static(); Ok(()) }
            "wfbjtii" => { validate_finite_parameter("WFBJTII", value)?; self.params.p745 = value; self.mark_param_given(745); self.recompute_instance_static(); Ok(()) }
            "wabjtii" => { validate_finite_parameter("WABJTII", value)?; self.params.p746 = value; self.mark_param_given(746); self.recompute_instance_static(); Ok(()) }
            "wcbjtii" => { validate_finite_parameter("WCBJTII", value)?; self.params.p747 = value; self.mark_param_given(747); self.recompute_instance_static(); Ok(()) }
            "webjtii" => { validate_finite_parameter("WEBJTII", value)?; self.params.p748 = value; self.mark_param_given(748); self.recompute_instance_static(); Ok(()) }
            "wmbjtii" => { validate_finite_parameter("WMBJTII", value)?; self.params.p749 = value; self.mark_param_given(749); self.recompute_instance_static(); Ok(()) }
            "wvbci" => { validate_finite_parameter("WVBCI", value)?; self.params.p750 = value; self.mark_param_given(750); self.recompute_instance_static(); Ok(()) }
            "wbeta0" => { validate_finite_parameter("WBETA0", value)?; self.params.p751 = value; self.mark_param_given(751); self.recompute_instance_static(); Ok(()) }
            "wbeta1" => { validate_finite_parameter("WBETA1", value)?; self.params.p752 = value; self.mark_param_given(752); self.recompute_instance_static(); Ok(()) }
            "wbeta2" => { validate_finite_parameter("WBETA2", value)?; self.params.p753 = value; self.mark_param_given(753); self.recompute_instance_static(); Ok(()) }
            "wvdsatii0" => { validate_finite_parameter("WVDSATII0", value)?; self.params.p754 = value; self.mark_param_given(754); self.recompute_instance_static(); Ok(()) }
            "wlii" => { validate_finite_parameter("WLII", value)?; self.params.p755 = value; self.mark_param_given(755); self.recompute_instance_static(); Ok(()) }
            "wesatii" => { validate_finite_parameter("WESATII", value)?; self.params.p756 = value; self.mark_param_given(756); self.recompute_instance_static(); Ok(()) }
            "wsii0" => { validate_finite_parameter("WSII0", value)?; self.params.p757 = value; self.mark_param_given(757); self.recompute_instance_static(); Ok(()) }
            "wsii1" => { validate_finite_parameter("WSII1", value)?; self.params.p758 = value; self.mark_param_given(758); self.recompute_instance_static(); Ok(()) }
            "wsii2" => { validate_finite_parameter("WSII2", value)?; self.params.p759 = value; self.mark_param_given(759); self.recompute_instance_static(); Ok(()) }
            "wsiid" => { validate_finite_parameter("WSIID", value)?; self.params.p760 = value; self.mark_param_given(760); self.recompute_instance_static(); Ok(()) }
            "wagidl" => { validate_finite_parameter("WAGIDL", value)?; self.params.p761 = value; self.mark_param_given(761); self.recompute_instance_static(); Ok(()) }
            "wbgidl" => { validate_finite_parameter("WBGIDL", value)?; self.params.p762 = value; self.mark_param_given(762); self.recompute_instance_static(); Ok(()) }
            "wbgidl1" => { validate_finite_parameter("WBGIDL1", value)?; self.params.p763 = value; self.mark_param_given(763); self.recompute_instance_static(); Ok(()) }
            "wcgidl" => { validate_finite_parameter("WCGIDL", value)?; self.params.p764 = value; self.mark_param_given(764); self.recompute_instance_static(); Ok(()) }
            "wrgidl" => { validate_finite_parameter("WRGIDL", value)?; self.params.p765 = value; self.mark_param_given(765); self.recompute_instance_static(); Ok(()) }
            "wkgidl" => { validate_finite_parameter("WKGIDL", value)?; self.params.p766 = value; self.mark_param_given(766); self.recompute_instance_static(); Ok(()) }
            "wfgidl" => { validate_finite_parameter("WFGIDL", value)?; self.params.p767 = value; self.mark_param_given(767); self.recompute_instance_static(); Ok(()) }
            "wagisl" => { validate_finite_parameter("WAGISL", value)?; self.params.p768 = value; self.mark_param_given(768); self.recompute_instance_static(); Ok(()) }
            "wbgisl" => { validate_finite_parameter("WBGISL", value)?; self.params.p769 = value; self.mark_param_given(769); self.recompute_instance_static(); Ok(()) }
            "wbgisl1" => { validate_finite_parameter("WBGISL1", value)?; self.params.p770 = value; self.mark_param_given(770); self.recompute_instance_static(); Ok(()) }
            "wcgisl" => { validate_finite_parameter("WCGISL", value)?; self.params.p771 = value; self.mark_param_given(771); self.recompute_instance_static(); Ok(()) }
            "wrgisl" => { validate_finite_parameter("WRGISL", value)?; self.params.p772 = value; self.mark_param_given(772); self.recompute_instance_static(); Ok(()) }
            "wkgisl" => { validate_finite_parameter("WKGISL", value)?; self.params.p773 = value; self.mark_param_given(773); self.recompute_instance_static(); Ok(()) }
            "wfgisl" => { validate_finite_parameter("WFGISL", value)?; self.params.p774 = value; self.mark_param_given(774); self.recompute_instance_static(); Ok(()) }
            "wntun" => { validate_finite_parameter("WNTUN", value)?; self.params.p775 = value; self.mark_param_given(775); self.recompute_instance_static(); Ok(()) }
            "wntund" => { validate_finite_parameter("WNTUND", value)?; self.params.p776 = value; self.mark_param_given(776); self.recompute_instance_static(); Ok(()) }
            "wndiode" => { validate_finite_parameter("WNDIODE", value)?; self.params.p777 = value; self.mark_param_given(777); self.recompute_instance_static(); Ok(()) }
            "wndioded" => { validate_finite_parameter("WNDIODED", value)?; self.params.p778 = value; self.mark_param_given(778); self.recompute_instance_static(); Ok(()) }
            "wnrecf0" => { validate_finite_parameter("WNRECF0", value)?; self.params.p779 = value; self.mark_param_given(779); self.recompute_instance_static(); Ok(()) }
            "wnrecf0d" => { validate_finite_parameter("WNRECF0D", value)?; self.params.p780 = value; self.mark_param_given(780); self.recompute_instance_static(); Ok(()) }
            "wnrecr0" => { validate_finite_parameter("WNRECR0", value)?; self.params.p781 = value; self.mark_param_given(781); self.recompute_instance_static(); Ok(()) }
            "wnrecr0d" => { validate_finite_parameter("WNRECR0D", value)?; self.params.p782 = value; self.mark_param_given(782); self.recompute_instance_static(); Ok(()) }
            "wisbjt" => { validate_finite_parameter("WISBJT", value)?; self.params.p783 = value; self.mark_param_given(783); self.recompute_instance_static(); Ok(()) }
            "widbjt" => { validate_finite_parameter("WIDBJT", value)?; self.params.p784 = value; self.mark_param_given(784); self.recompute_instance_static(); Ok(()) }
            "wisdif" => { validate_finite_parameter("WISDIF", value)?; self.params.p785 = value; self.mark_param_given(785); self.recompute_instance_static(); Ok(()) }
            "widdif" => { validate_finite_parameter("WIDDIF", value)?; self.params.p786 = value; self.mark_param_given(786); self.recompute_instance_static(); Ok(()) }
            "wisrec" => { validate_finite_parameter("WISREC", value)?; self.params.p787 = value; self.mark_param_given(787); self.recompute_instance_static(); Ok(()) }
            "widrec" => { validate_finite_parameter("WIDREC", value)?; self.params.p788 = value; self.mark_param_given(788); self.recompute_instance_static(); Ok(()) }
            "wistun" => { validate_finite_parameter("WISTUN", value)?; self.params.p789 = value; self.mark_param_given(789); self.recompute_instance_static(); Ok(()) }
            "widtun" => { validate_finite_parameter("WIDTUN", value)?; self.params.p790 = value; self.mark_param_given(790); self.recompute_instance_static(); Ok(()) }
            "wvrec0" => { validate_finite_parameter("WVREC0", value)?; self.params.p791 = value; self.mark_param_given(791); self.recompute_instance_static(); Ok(()) }
            "wvrec0d" => { validate_finite_parameter("WVREC0D", value)?; self.params.p792 = value; self.mark_param_given(792); self.recompute_instance_static(); Ok(()) }
            "wvtun0" => { validate_finite_parameter("WVTUN0", value)?; self.params.p793 = value; self.mark_param_given(793); self.recompute_instance_static(); Ok(()) }
            "wvtun0d" => { validate_finite_parameter("WVTUN0D", value)?; self.params.p794 = value; self.mark_param_given(794); self.recompute_instance_static(); Ok(()) }
            "wnbjt" => { validate_finite_parameter("WNBJT", value)?; self.params.p795 = value; self.mark_param_given(795); self.recompute_instance_static(); Ok(()) }
            "wlbjt0" => { validate_finite_parameter("WLBJT0", value)?; self.params.p796 = value; self.mark_param_given(796); self.recompute_instance_static(); Ok(()) }
            "wvabjt" => { validate_finite_parameter("WVABJT", value)?; self.params.p797 = value; self.mark_param_given(797); self.recompute_instance_static(); Ok(()) }
            "waely" => { validate_finite_parameter("WAELY", value)?; self.params.p798 = value; self.mark_param_given(798); self.recompute_instance_static(); Ok(()) }
            "wahli" => { validate_finite_parameter("WAHLI", value)?; self.params.p799 = value; self.mark_param_given(799); self.recompute_instance_static(); Ok(()) }
            "wahlid" => { validate_finite_parameter("WAHLID", value)?; self.params.p800 = value; self.mark_param_given(800); self.recompute_instance_static(); Ok(()) }
            "wvsdfb" => { validate_finite_parameter("WVSDFB", value)?; self.params.p801 = value; self.mark_param_given(801); self.recompute_instance_static(); Ok(()) }
            "wvsdth" => { validate_finite_parameter("WVSDTH", value)?; self.params.p802 = value; self.mark_param_given(802); self.recompute_instance_static(); Ok(()) }
            "wdelvt" => { validate_finite_parameter("WDELVT", value)?; self.params.p803 = value; self.mark_param_given(803); self.recompute_instance_static(); Ok(()) }
            "wacde" => { validate_finite_parameter("WACDE", value)?; self.params.p804 = value; self.mark_param_given(804); self.recompute_instance_static(); Ok(()) }
            "wmoin" => { validate_finite_parameter("WMOIN", value)?; self.params.p805 = value; self.mark_param_given(805); self.recompute_instance_static(); Ok(()) }
            "wnoff" => { validate_finite_parameter("WNOFF", value)?; self.params.p806 = value; self.mark_param_given(806); self.recompute_instance_static(); Ok(()) }
            "wnoff2" => { validate_finite_parameter("WNOFF2", value)?; self.params.p807 = value; self.mark_param_given(807); self.recompute_instance_static(); Ok(()) }
            "wxrcrg1" => { validate_finite_parameter("WXRCRG1", value)?; self.params.p808 = value; self.mark_param_given(808); self.recompute_instance_static(); Ok(()) }
            "wxrcrg2" => { validate_finite_parameter("WXRCRG2", value)?; self.params.p809 = value; self.mark_param_given(809); self.recompute_instance_static(); Ok(()) }
            "wvbsa" => { validate_finite_parameter("WVBSA", value)?; self.params.p810 = value; self.mark_param_given(810); self.recompute_instance_static(); Ok(()) }
            "wvsce" => { validate_finite_parameter("WVSCE", value)?; self.params.p811 = value; self.mark_param_given(811); self.recompute_instance_static(); Ok(()) }
            "wcdsbs" => { validate_finite_parameter("WCDSBS", value)?; self.params.p812 = value; self.mark_param_given(812); self.recompute_instance_static(); Ok(()) }
            "wnofffd" => { validate_finite_parameter("WNOFFFD", value)?; self.params.p813 = value; self.mark_param_given(813); self.recompute_instance_static(); Ok(()) }
            "wvofffd" => { validate_finite_parameter("WVOFFFD", value)?; self.params.p814 = value; self.mark_param_given(814); self.recompute_instance_static(); Ok(()) }
            "wk1b" => { validate_finite_parameter("WK1B", value)?; self.params.p815 = value; self.mark_param_given(815); self.recompute_instance_static(); Ok(()) }
            "wk2b" => { validate_finite_parameter("WK2B", value)?; self.params.p816 = value; self.mark_param_given(816); self.recompute_instance_static(); Ok(()) }
            "wdk2b" => { validate_finite_parameter("WDK2B", value)?; self.params.p817 = value; self.mark_param_given(817); self.recompute_instance_static(); Ok(()) }
            "wdvbd0" => { validate_finite_parameter("WDVBD0", value)?; self.params.p818 = value; self.mark_param_given(818); self.recompute_instance_static(); Ok(()) }
            "wdvbd1" => { validate_finite_parameter("WDVBD1", value)?; self.params.p819 = value; self.mark_param_given(819); self.recompute_instance_static(); Ok(()) }
            "wmoinfd" => { validate_finite_parameter("WMOINFD", value)?; self.params.p820 = value; self.mark_param_given(820); self.recompute_instance_static(); Ok(()) }
            "wvbs0pd" => { validate_finite_parameter("WVBS0PD", value)?; self.params.p821 = value; self.mark_param_given(821); self.recompute_instance_static(); Ok(()) }
            "wvbs0fd" => { validate_finite_parameter("WVBS0FD", value)?; self.params.p822 = value; self.mark_param_given(822); self.recompute_instance_static(); Ok(()) }
            "pxj" => { validate_finite_parameter("PXJ", value)?; self.params.p823 = value; self.mark_param_given(823); self.recompute_instance_static(); Ok(()) }
            "palphagb1" => { validate_finite_parameter("PALPHAGB1", value)?; self.params.p824 = value; self.mark_param_given(824); self.recompute_instance_static(); Ok(()) }
            "palphagb1_t" => { validate_finite_parameter("PALPHAGB1_T", value)?; self.params.p825 = value; self.mark_param_given(825); self.recompute_instance_static(); Ok(()) }
            "pbetagb1" => { validate_finite_parameter("PBETAGB1", value)?; self.params.p826 = value; self.mark_param_given(826); self.recompute_instance_static(); Ok(()) }
            "palphagb2" => { validate_finite_parameter("PALPHAGB2", value)?; self.params.p827 = value; self.mark_param_given(827); self.recompute_instance_static(); Ok(()) }
            "palphagb2_t" => { validate_finite_parameter("PALPHAGB2_T", value)?; self.params.p828 = value; self.mark_param_given(828); self.recompute_instance_static(); Ok(()) }
            "pbetagb2" => { validate_finite_parameter("PBETAGB2", value)?; self.params.p829 = value; self.mark_param_given(829); self.recompute_instance_static(); Ok(()) }
            "paigbcp2" => { validate_finite_parameter("PAIGBCP2", value)?; self.params.p830 = value; self.mark_param_given(830); self.recompute_instance_static(); Ok(()) }
            "paigbcp2_t" => { validate_finite_parameter("PAIGBCP2_T", value)?; self.params.p831 = value; self.mark_param_given(831); self.recompute_instance_static(); Ok(()) }
            "pbigbcp2" => { validate_finite_parameter("PBIGBCP2", value)?; self.params.p832 = value; self.mark_param_given(832); self.recompute_instance_static(); Ok(()) }
            "pcigbcp2" => { validate_finite_parameter("PCIGBCP2", value)?; self.params.p833 = value; self.mark_param_given(833); self.recompute_instance_static(); Ok(()) }
            "pcgsl" => { validate_finite_parameter("PCGSL", value)?; self.params.p834 = value; self.mark_param_given(834); self.recompute_instance_static(); Ok(()) }
            "pcgdl" => { validate_finite_parameter("PCGDL", value)?; self.params.p835 = value; self.mark_param_given(835); self.recompute_instance_static(); Ok(()) }
            "pckappa" => { validate_finite_parameter("PCKAPPA", value)?; self.params.p836 = value; self.mark_param_given(836); self.recompute_instance_static(); Ok(()) }
            "pndif" => { validate_finite_parameter("PNDIF", value)?; self.params.p837 = value; self.mark_param_given(837); self.recompute_instance_static(); Ok(()) }
            "pute" => { validate_finite_parameter("PUTE", value)?; self.params.p838 = value; self.mark_param_given(838); self.recompute_instance_static(); Ok(()) }
            "pkt1" => { validate_finite_parameter("PKT1", value)?; self.params.p839 = value; self.mark_param_given(839); self.recompute_instance_static(); Ok(()) }
            "pkt1l" => { validate_finite_parameter("PKT1L", value)?; self.params.p840 = value; self.mark_param_given(840); self.recompute_instance_static(); Ok(()) }
            "pkt2" => { validate_finite_parameter("PKT2", value)?; self.params.p841 = value; self.mark_param_given(841); self.recompute_instance_static(); Ok(()) }
            "pua1" => { validate_finite_parameter("PUA1", value)?; self.params.p842 = value; self.mark_param_given(842); self.recompute_instance_static(); Ok(()) }
            "pub1" => { validate_finite_parameter("PUB1", value)?; self.params.p843 = value; self.mark_param_given(843); self.recompute_instance_static(); Ok(()) }
            "puc1" => { validate_finite_parameter("PUC1", value)?; self.params.p844 = value; self.mark_param_given(844); self.recompute_instance_static(); Ok(()) }
            "pat" => { validate_finite_parameter("PAT", value)?; self.params.p845 = value; self.mark_param_given(845); self.recompute_instance_static(); Ok(()) }
            "pprt" => { validate_finite_parameter("PPRT", value)?; self.params.p846 = value; self.mark_param_given(846); self.recompute_instance_static(); Ok(()) }
            "pntrecf" => { validate_finite_parameter("PNTRECF", value)?; self.params.p847 = value; self.mark_param_given(847); self.recompute_instance_static(); Ok(()) }
            "pntrecr" => { validate_finite_parameter("PNTRECR", value)?; self.params.p848 = value; self.mark_param_given(848); self.recompute_instance_static(); Ok(()) }
            "pxbjt" => { validate_finite_parameter("PXBJT", value)?; self.params.p849 = value; self.mark_param_given(849); self.recompute_instance_static(); Ok(()) }
            "pxdif" => { validate_finite_parameter("PXDIF", value)?; self.params.p850 = value; self.mark_param_given(850); self.recompute_instance_static(); Ok(()) }
            "pxrec" => { validate_finite_parameter("PXREC", value)?; self.params.p851 = value; self.mark_param_given(851); self.recompute_instance_static(); Ok(()) }
            "pxtun" => { validate_finite_parameter("PXTUN", value)?; self.params.p852 = value; self.mark_param_given(852); self.recompute_instance_static(); Ok(()) }
            "pxdifd" => { validate_finite_parameter("PXDIFD", value)?; self.params.p853 = value; self.mark_param_given(853); self.recompute_instance_static(); Ok(()) }
            "pxrecd" => { validate_finite_parameter("PXRECD", value)?; self.params.p854 = value; self.mark_param_given(854); self.recompute_instance_static(); Ok(()) }
            "pxtund" => { validate_finite_parameter("PXTUND", value)?; self.params.p855 = value; self.mark_param_given(855); self.recompute_instance_static(); Ok(()) }
            "paigc" => { validate_finite_parameter("PAIGC", value)?; self.params.p856 = value; self.mark_param_given(856); self.recompute_instance_static(); Ok(()) }
            "paigc1" => { validate_finite_parameter("PAIGC1", value)?; self.params.p857 = value; self.mark_param_given(857); self.recompute_instance_static(); Ok(()) }
            "pbigc" => { validate_finite_parameter("PBIGC", value)?; self.params.p858 = value; self.mark_param_given(858); self.recompute_instance_static(); Ok(()) }
            "pcigc" => { validate_finite_parameter("PCIGC", value)?; self.params.p859 = value; self.mark_param_given(859); self.recompute_instance_static(); Ok(()) }
            "paigsd" => { validate_finite_parameter("PAIGSD", value)?; self.params.p860 = value; self.mark_param_given(860); self.recompute_instance_static(); Ok(()) }
            "paigsd1" => { validate_finite_parameter("PAIGSD1", value)?; self.params.p861 = value; self.mark_param_given(861); self.recompute_instance_static(); Ok(()) }
            "pbigsd" => { validate_finite_parameter("PBIGSD", value)?; self.params.p862 = value; self.mark_param_given(862); self.recompute_instance_static(); Ok(()) }
            "pcigsd" => { validate_finite_parameter("PCIGSD", value)?; self.params.p863 = value; self.mark_param_given(863); self.recompute_instance_static(); Ok(()) }
            "pnigc" => { validate_finite_parameter("PNIGC", value)?; self.params.p864 = value; self.mark_param_given(864); self.recompute_instance_static(); Ok(()) }
            "ppigcd" => { validate_finite_parameter("PPIGCD", value)?; self.params.p865 = value; self.mark_param_given(865); self.recompute_instance_static(); Ok(()) }
            "ppoxedge" => { validate_finite_parameter("PPOXEDGE", value)?; self.params.p866 = value; self.mark_param_given(866); self.recompute_instance_static(); Ok(()) }
            "pigt" => { validate_finite_parameter("PIGT", value)?; self.params.p867 = value; self.mark_param_given(867); self.recompute_instance_static(); Ok(()) }
            "pnch" => { validate_finite_parameter("PNCH", value)?; self.params.p868 = value; self.mark_param_given(868); self.recompute_instance_static(); Ok(()) }
            "pnsub" => { validate_finite_parameter("PNSUB", value)?; self.params.p869 = value; self.mark_param_given(869); self.recompute_instance_static(); Ok(()) }
            "pnsd" => { validate_finite_parameter("PNSD", value)?; self.params.p870 = value; self.mark_param_given(870); self.recompute_instance_static(); Ok(()) }
            "pngate" => { validate_finite_parameter("PNGATE", value)?; self.params.p871 = value; self.mark_param_given(871); self.recompute_instance_static(); Ok(()) }
            "pvth0" => { validate_finite_parameter("PVTH0", value)?; self.params.p872 = value; self.mark_param_given(872); self.recompute_instance_static(); Ok(()) }
            "pvfb" => { validate_finite_parameter("PVFB", value)?; self.params.p873 = value; self.mark_param_given(873); self.recompute_instance_static(); Ok(()) }
            "pk1" => { validate_finite_parameter("PK1", value)?; self.params.p874 = value; self.mark_param_given(874); self.recompute_instance_static(); Ok(()) }
            "pk1w1" => { validate_finite_parameter("PK1W1", value)?; self.params.p875 = value; self.mark_param_given(875); self.recompute_instance_static(); Ok(()) }
            "pk1w2" => { validate_finite_parameter("PK1W2", value)?; self.params.p876 = value; self.mark_param_given(876); self.recompute_instance_static(); Ok(()) }
            "pk2" => { validate_finite_parameter("PK2", value)?; self.params.p877 = value; self.mark_param_given(877); self.recompute_instance_static(); Ok(()) }
            "pk3" => { validate_finite_parameter("PK3", value)?; self.params.p878 = value; self.mark_param_given(878); self.recompute_instance_static(); Ok(()) }
            "pk3b" => { validate_finite_parameter("PK3B", value)?; self.params.p879 = value; self.mark_param_given(879); self.recompute_instance_static(); Ok(()) }
            "pkb1" => { validate_finite_parameter("PKB1", value)?; self.params.p880 = value; self.mark_param_given(880); self.recompute_instance_static(); Ok(()) }
            "pw0" => { validate_finite_parameter("PW0", value)?; self.params.p881 = value; self.mark_param_given(881); self.recompute_instance_static(); Ok(()) }
            "plpeb" => { validate_finite_parameter("PLPEB", value)?; self.params.p882 = value; self.mark_param_given(882); self.recompute_instance_static(); Ok(()) }
            "pdvt0" => { validate_finite_parameter("PDVT0", value)?; self.params.p883 = value; self.mark_param_given(883); self.recompute_instance_static(); Ok(()) }
            "pdvt1" => { validate_finite_parameter("PDVT1", value)?; self.params.p884 = value; self.mark_param_given(884); self.recompute_instance_static(); Ok(()) }
            "pdvt2" => { validate_finite_parameter("PDVT2", value)?; self.params.p885 = value; self.mark_param_given(885); self.recompute_instance_static(); Ok(()) }
            "pdvt0w" => { validate_finite_parameter("PDVT0W", value)?; self.params.p886 = value; self.mark_param_given(886); self.recompute_instance_static(); Ok(()) }
            "pdvt1w" => { validate_finite_parameter("PDVT1W", value)?; self.params.p887 = value; self.mark_param_given(887); self.recompute_instance_static(); Ok(()) }
            "pdvt2w" => { validate_finite_parameter("PDVT2W", value)?; self.params.p888 = value; self.mark_param_given(888); self.recompute_instance_static(); Ok(()) }
            "pu0" => { validate_finite_parameter("PU0", value)?; self.params.p889 = value; self.mark_param_given(889); self.recompute_instance_static(); Ok(()) }
            "peu" => { validate_finite_parameter("PEU", value)?; self.params.p890 = value; self.mark_param_given(890); self.recompute_instance_static(); Ok(()) }
            "pua" => { validate_finite_parameter("PUA", value)?; self.params.p891 = value; self.mark_param_given(891); self.recompute_instance_static(); Ok(()) }
            "pub" => { validate_finite_parameter("PUB", value)?; self.params.p892 = value; self.mark_param_given(892); self.recompute_instance_static(); Ok(()) }
            "puc" => { validate_finite_parameter("PUC", value)?; self.params.p893 = value; self.mark_param_given(893); self.recompute_instance_static(); Ok(()) }
            "pud" => { validate_finite_parameter("PUD", value)?; self.params.p894 = value; self.mark_param_given(894); self.recompute_instance_static(); Ok(()) }
            "pud1" => { validate_finite_parameter("PUD1", value)?; self.params.p895 = value; self.mark_param_given(895); self.recompute_instance_static(); Ok(()) }
            "pucste" => { validate_finite_parameter("PUCSTE", value)?; self.params.p896 = value; self.mark_param_given(896); self.recompute_instance_static(); Ok(()) }
            "pucs" => { validate_finite_parameter("PUCS", value)?; self.params.p897 = value; self.mark_param_given(897); self.recompute_instance_static(); Ok(()) }
            "pvsat" => { validate_finite_parameter("PVSAT", value)?; self.params.p898 = value; self.mark_param_given(898); self.recompute_instance_static(); Ok(()) }
            "pa0" => { validate_finite_parameter("PA0", value)?; self.params.p899 = value; self.mark_param_given(899); self.recompute_instance_static(); Ok(()) }
            "pags" => { validate_finite_parameter("PAGS", value)?; self.params.p900 = value; self.mark_param_given(900); self.recompute_instance_static(); Ok(()) }
            "pb0" => { validate_finite_parameter("PB0", value)?; self.params.p901 = value; self.mark_param_given(901); self.recompute_instance_static(); Ok(()) }
            "pb1" => { validate_finite_parameter("PB1", value)?; self.params.p902 = value; self.mark_param_given(902); self.recompute_instance_static(); Ok(()) }
            "pketa" => { validate_finite_parameter("PKETA", value)?; self.params.p903 = value; self.mark_param_given(903); self.recompute_instance_static(); Ok(()) }
            "pketas" => { validate_finite_parameter("PKETAS", value)?; self.params.p904 = value; self.mark_param_given(904); self.recompute_instance_static(); Ok(()) }
            "pa1" => { validate_finite_parameter("PA1", value)?; self.params.p905 = value; self.mark_param_given(905); self.recompute_instance_static(); Ok(()) }
            "pa2" => { validate_finite_parameter("PA2", value)?; self.params.p906 = value; self.mark_param_given(906); self.recompute_instance_static(); Ok(()) }
            "prdsw" => { validate_finite_parameter("PRDSW", value)?; self.params.p907 = value; self.mark_param_given(907); self.recompute_instance_static(); Ok(()) }
            "prsw" => { validate_finite_parameter("PRSW", value)?; self.params.p908 = value; self.mark_param_given(908); self.recompute_instance_static(); Ok(()) }
            "prdw" => { validate_finite_parameter("PRDW", value)?; self.params.p909 = value; self.mark_param_given(909); self.recompute_instance_static(); Ok(()) }
            "pprwb" => { validate_finite_parameter("PPRWB", value)?; self.params.p910 = value; self.mark_param_given(910); self.recompute_instance_static(); Ok(()) }
            "pprwe" => { validate_finite_parameter("PPRWE", value)?; self.params.p911 = value; self.mark_param_given(911); self.recompute_instance_static(); Ok(()) }
            "pprwg" => { validate_finite_parameter("PPRWG", value)?; self.params.p912 = value; self.mark_param_given(912); self.recompute_instance_static(); Ok(()) }
            "pwr" => { validate_finite_parameter("PWR", value)?; self.params.p913 = value; self.mark_param_given(913); self.recompute_instance_static(); Ok(()) }
            "pnfactor" => { validate_finite_parameter("PNFACTOR", value)?; self.params.p914 = value; self.mark_param_given(914); self.recompute_instance_static(); Ok(()) }
            "pdwg" => { validate_finite_parameter("PDWG", value)?; self.params.p915 = value; self.mark_param_given(915); self.recompute_instance_static(); Ok(()) }
            "pdwb" => { validate_finite_parameter("PDWB", value)?; self.params.p916 = value; self.mark_param_given(916); self.recompute_instance_static(); Ok(()) }
            "pvoff" => { validate_finite_parameter("PVOFF", value)?; self.params.p917 = value; self.mark_param_given(917); self.recompute_instance_static(); Ok(()) }
            "peta0" => { validate_finite_parameter("PETA0", value)?; self.params.p918 = value; self.mark_param_given(918); self.recompute_instance_static(); Ok(()) }
            "petab" => { validate_finite_parameter("PETAB", value)?; self.params.p919 = value; self.mark_param_given(919); self.recompute_instance_static(); Ok(()) }
            "peta0cv" => { validate_finite_parameter("PETA0CV", value)?; self.params.p920 = value; self.mark_param_given(920); self.recompute_instance_static(); Ok(()) }
            "petabcv" => { validate_finite_parameter("PETABCV", value)?; self.params.p921 = value; self.mark_param_given(921); self.recompute_instance_static(); Ok(()) }
            "pdsub" => { validate_finite_parameter("PDSUB", value)?; self.params.p922 = value; self.mark_param_given(922); self.recompute_instance_static(); Ok(()) }
            "pcit" => { validate_finite_parameter("PCIT", value)?; self.params.p923 = value; self.mark_param_given(923); self.recompute_instance_static(); Ok(()) }
            "pcdsc" => { validate_finite_parameter("PCDSC", value)?; self.params.p924 = value; self.mark_param_given(924); self.recompute_instance_static(); Ok(()) }
            "pcdscb" => { validate_finite_parameter("PCDSCB", value)?; self.params.p925 = value; self.mark_param_given(925); self.recompute_instance_static(); Ok(()) }
            "pcdscd" => { validate_finite_parameter("PCDSCD", value)?; self.params.p926 = value; self.mark_param_given(926); self.recompute_instance_static(); Ok(()) }
            "ppclm" => { validate_finite_parameter("PPCLM", value)?; self.params.p927 = value; self.mark_param_given(927); self.recompute_instance_static(); Ok(()) }
            "ppdiblc1" => { validate_finite_parameter("PPDIBLC1", value)?; self.params.p928 = value; self.mark_param_given(928); self.recompute_instance_static(); Ok(()) }
            "ppdiblc2" => { validate_finite_parameter("PPDIBLC2", value)?; self.params.p929 = value; self.mark_param_given(929); self.recompute_instance_static(); Ok(()) }
            "ppdiblcb" => { validate_finite_parameter("PPDIBLCB", value)?; self.params.p930 = value; self.mark_param_given(930); self.recompute_instance_static(); Ok(()) }
            "pdrout" => { validate_finite_parameter("PDROUT", value)?; self.params.p931 = value; self.mark_param_given(931); self.recompute_instance_static(); Ok(()) }
            "ppvag" => { validate_finite_parameter("PPVAG", value)?; self.params.p932 = value; self.mark_param_given(932); self.recompute_instance_static(); Ok(()) }
            "pdelta" => { validate_finite_parameter("PDELTA", value)?; self.params.p933 = value; self.mark_param_given(933); self.recompute_instance_static(); Ok(()) }
            "palpha0" => { validate_finite_parameter("PALPHA0", value)?; self.params.p934 = value; self.mark_param_given(934); self.recompute_instance_static(); Ok(()) }
            "pfbjtii" => { validate_finite_parameter("PFBJTII", value)?; self.params.p935 = value; self.mark_param_given(935); self.recompute_instance_static(); Ok(()) }
            "pabjtii" => { validate_finite_parameter("PABJTII", value)?; self.params.p936 = value; self.mark_param_given(936); self.recompute_instance_static(); Ok(()) }
            "pcbjtii" => { validate_finite_parameter("PCBJTII", value)?; self.params.p937 = value; self.mark_param_given(937); self.recompute_instance_static(); Ok(()) }
            "pebjtii" => { validate_finite_parameter("PEBJTII", value)?; self.params.p938 = value; self.mark_param_given(938); self.recompute_instance_static(); Ok(()) }
            "pmbjtii" => { validate_finite_parameter("PMBJTII", value)?; self.params.p939 = value; self.mark_param_given(939); self.recompute_instance_static(); Ok(()) }
            "pvbci" => { validate_finite_parameter("PVBCI", value)?; self.params.p940 = value; self.mark_param_given(940); self.recompute_instance_static(); Ok(()) }
            "pbeta0" => { validate_finite_parameter("PBETA0", value)?; self.params.p941 = value; self.mark_param_given(941); self.recompute_instance_static(); Ok(()) }
            "pbeta1" => { validate_finite_parameter("PBETA1", value)?; self.params.p942 = value; self.mark_param_given(942); self.recompute_instance_static(); Ok(()) }
            "pbeta2" => { validate_finite_parameter("PBETA2", value)?; self.params.p943 = value; self.mark_param_given(943); self.recompute_instance_static(); Ok(()) }
            "pvdsatii0" => { validate_finite_parameter("PVDSATII0", value)?; self.params.p944 = value; self.mark_param_given(944); self.recompute_instance_static(); Ok(()) }
            "plii" => { validate_finite_parameter("PLII", value)?; self.params.p945 = value; self.mark_param_given(945); self.recompute_instance_static(); Ok(()) }
            "pesatii" => { validate_finite_parameter("PESATII", value)?; self.params.p946 = value; self.mark_param_given(946); self.recompute_instance_static(); Ok(()) }
            "psii0" => { validate_finite_parameter("PSII0", value)?; self.params.p947 = value; self.mark_param_given(947); self.recompute_instance_static(); Ok(()) }
            "psii1" => { validate_finite_parameter("PSII1", value)?; self.params.p948 = value; self.mark_param_given(948); self.recompute_instance_static(); Ok(()) }
            "psii2" => { validate_finite_parameter("PSII2", value)?; self.params.p949 = value; self.mark_param_given(949); self.recompute_instance_static(); Ok(()) }
            "psiid" => { validate_finite_parameter("PSIID", value)?; self.params.p950 = value; self.mark_param_given(950); self.recompute_instance_static(); Ok(()) }
            "pagidl" => { validate_finite_parameter("PAGIDL", value)?; self.params.p951 = value; self.mark_param_given(951); self.recompute_instance_static(); Ok(()) }
            "pbgidl" => { validate_finite_parameter("PBGIDL", value)?; self.params.p952 = value; self.mark_param_given(952); self.recompute_instance_static(); Ok(()) }
            "pbgidl1" => { validate_finite_parameter("PBGIDL1", value)?; self.params.p953 = value; self.mark_param_given(953); self.recompute_instance_static(); Ok(()) }
            "pcgidl" => { validate_finite_parameter("PCGIDL", value)?; self.params.p954 = value; self.mark_param_given(954); self.recompute_instance_static(); Ok(()) }
            "prgidl" => { validate_finite_parameter("PRGIDL", value)?; self.params.p955 = value; self.mark_param_given(955); self.recompute_instance_static(); Ok(()) }
            "pkgidl" => { validate_finite_parameter("PKGIDL", value)?; self.params.p956 = value; self.mark_param_given(956); self.recompute_instance_static(); Ok(()) }
            "pfgidl" => { validate_finite_parameter("PFGIDL", value)?; self.params.p957 = value; self.mark_param_given(957); self.recompute_instance_static(); Ok(()) }
            "pagisl" => { validate_finite_parameter("PAGISL", value)?; self.params.p958 = value; self.mark_param_given(958); self.recompute_instance_static(); Ok(()) }
            "pbgisl" => { validate_finite_parameter("PBGISL", value)?; self.params.p959 = value; self.mark_param_given(959); self.recompute_instance_static(); Ok(()) }
            "pbgisl1" => { validate_finite_parameter("PBGISL1", value)?; self.params.p960 = value; self.mark_param_given(960); self.recompute_instance_static(); Ok(()) }
            "pcgisl" => { validate_finite_parameter("PCGISL", value)?; self.params.p961 = value; self.mark_param_given(961); self.recompute_instance_static(); Ok(()) }
            "prgisl" => { validate_finite_parameter("PRGISL", value)?; self.params.p962 = value; self.mark_param_given(962); self.recompute_instance_static(); Ok(()) }
            "pkgisl" => { validate_finite_parameter("PKGISL", value)?; self.params.p963 = value; self.mark_param_given(963); self.recompute_instance_static(); Ok(()) }
            "pfgisl" => { validate_finite_parameter("PFGISL", value)?; self.params.p964 = value; self.mark_param_given(964); self.recompute_instance_static(); Ok(()) }
            "pntun" => { validate_finite_parameter("PNTUN", value)?; self.params.p965 = value; self.mark_param_given(965); self.recompute_instance_static(); Ok(()) }
            "pntund" => { validate_finite_parameter("PNTUND", value)?; self.params.p966 = value; self.mark_param_given(966); self.recompute_instance_static(); Ok(()) }
            "pndiode" => { validate_finite_parameter("PNDIODE", value)?; self.params.p967 = value; self.mark_param_given(967); self.recompute_instance_static(); Ok(()) }
            "pndioded" => { validate_finite_parameter("PNDIODED", value)?; self.params.p968 = value; self.mark_param_given(968); self.recompute_instance_static(); Ok(()) }
            "pnrecf0" => { validate_finite_parameter("PNRECF0", value)?; self.params.p969 = value; self.mark_param_given(969); self.recompute_instance_static(); Ok(()) }
            "pnrecf0d" => { validate_finite_parameter("PNRECF0D", value)?; self.params.p970 = value; self.mark_param_given(970); self.recompute_instance_static(); Ok(()) }
            "pnrecr0" => { validate_finite_parameter("PNRECR0", value)?; self.params.p971 = value; self.mark_param_given(971); self.recompute_instance_static(); Ok(()) }
            "pnrecr0d" => { validate_finite_parameter("PNRECR0D", value)?; self.params.p972 = value; self.mark_param_given(972); self.recompute_instance_static(); Ok(()) }
            "pisbjt" => { validate_finite_parameter("PISBJT", value)?; self.params.p973 = value; self.mark_param_given(973); self.recompute_instance_static(); Ok(()) }
            "pidbjt" => { validate_finite_parameter("PIDBJT", value)?; self.params.p974 = value; self.mark_param_given(974); self.recompute_instance_static(); Ok(()) }
            "pisdif" => { validate_finite_parameter("PISDIF", value)?; self.params.p975 = value; self.mark_param_given(975); self.recompute_instance_static(); Ok(()) }
            "piddif" => { validate_finite_parameter("PIDDIF", value)?; self.params.p976 = value; self.mark_param_given(976); self.recompute_instance_static(); Ok(()) }
            "pisrec" => { validate_finite_parameter("PISREC", value)?; self.params.p977 = value; self.mark_param_given(977); self.recompute_instance_static(); Ok(()) }
            "pidrec" => { validate_finite_parameter("PIDREC", value)?; self.params.p978 = value; self.mark_param_given(978); self.recompute_instance_static(); Ok(()) }
            "pistun" => { validate_finite_parameter("PISTUN", value)?; self.params.p979 = value; self.mark_param_given(979); self.recompute_instance_static(); Ok(()) }
            "pidtun" => { validate_finite_parameter("PIDTUN", value)?; self.params.p980 = value; self.mark_param_given(980); self.recompute_instance_static(); Ok(()) }
            "pvrec0" => { validate_finite_parameter("PVREC0", value)?; self.params.p981 = value; self.mark_param_given(981); self.recompute_instance_static(); Ok(()) }
            "pvrec0d" => { validate_finite_parameter("PVREC0D", value)?; self.params.p982 = value; self.mark_param_given(982); self.recompute_instance_static(); Ok(()) }
            "pvtun0" => { validate_finite_parameter("PVTUN0", value)?; self.params.p983 = value; self.mark_param_given(983); self.recompute_instance_static(); Ok(()) }
            "pvtun0d" => { validate_finite_parameter("PVTUN0D", value)?; self.params.p984 = value; self.mark_param_given(984); self.recompute_instance_static(); Ok(()) }
            "pnbjt" => { validate_finite_parameter("PNBJT", value)?; self.params.p985 = value; self.mark_param_given(985); self.recompute_instance_static(); Ok(()) }
            "plbjt0" => { validate_finite_parameter("PLBJT0", value)?; self.params.p986 = value; self.mark_param_given(986); self.recompute_instance_static(); Ok(()) }
            "pvabjt" => { validate_finite_parameter("PVABJT", value)?; self.params.p987 = value; self.mark_param_given(987); self.recompute_instance_static(); Ok(()) }
            "paely" => { validate_finite_parameter("PAELY", value)?; self.params.p988 = value; self.mark_param_given(988); self.recompute_instance_static(); Ok(()) }
            "pahli" => { validate_finite_parameter("PAHLI", value)?; self.params.p989 = value; self.mark_param_given(989); self.recompute_instance_static(); Ok(()) }
            "pahlid" => { validate_finite_parameter("PAHLID", value)?; self.params.p990 = value; self.mark_param_given(990); self.recompute_instance_static(); Ok(()) }
            "pvsdfb" => { validate_finite_parameter("PVSDFB", value)?; self.params.p991 = value; self.mark_param_given(991); self.recompute_instance_static(); Ok(()) }
            "pvsdth" => { validate_finite_parameter("PVSDTH", value)?; self.params.p992 = value; self.mark_param_given(992); self.recompute_instance_static(); Ok(()) }
            "pdelvt" => { validate_finite_parameter("PDELVT", value)?; self.params.p993 = value; self.mark_param_given(993); self.recompute_instance_static(); Ok(()) }
            "pacde" => { validate_finite_parameter("PACDE", value)?; self.params.p994 = value; self.mark_param_given(994); self.recompute_instance_static(); Ok(()) }
            "pmoin" => { validate_finite_parameter("PMOIN", value)?; self.params.p995 = value; self.mark_param_given(995); self.recompute_instance_static(); Ok(()) }
            "pnoff" => { validate_finite_parameter("PNOFF", value)?; self.params.p996 = value; self.mark_param_given(996); self.recompute_instance_static(); Ok(()) }
            "pnoff2" => { validate_finite_parameter("PNOFF2", value)?; self.params.p997 = value; self.mark_param_given(997); self.recompute_instance_static(); Ok(()) }
            "pxrcrg1" => { validate_finite_parameter("PXRCRG1", value)?; self.params.p998 = value; self.mark_param_given(998); self.recompute_instance_static(); Ok(()) }
            "pxrcrg2" => { validate_finite_parameter("PXRCRG2", value)?; self.params.p999 = value; self.mark_param_given(999); self.recompute_instance_static(); Ok(()) }
            "pvbsa" => { validate_finite_parameter("PVBSA", value)?; self.params.p1000 = value; self.mark_param_given(1000); self.recompute_instance_static(); Ok(()) }
            "pvsce" => { validate_finite_parameter("PVSCE", value)?; self.params.p1001 = value; self.mark_param_given(1001); self.recompute_instance_static(); Ok(()) }
            "pcdsbs" => { validate_finite_parameter("PCDSBS", value)?; self.params.p1002 = value; self.mark_param_given(1002); self.recompute_instance_static(); Ok(()) }
            "pnofffd" => { validate_finite_parameter("PNOFFFD", value)?; self.params.p1003 = value; self.mark_param_given(1003); self.recompute_instance_static(); Ok(()) }
            "pvofffd" => { validate_finite_parameter("PVOFFFD", value)?; self.params.p1004 = value; self.mark_param_given(1004); self.recompute_instance_static(); Ok(()) }
            "pk1b" => { validate_finite_parameter("PK1B", value)?; self.params.p1005 = value; self.mark_param_given(1005); self.recompute_instance_static(); Ok(()) }
            "pk2b" => { validate_finite_parameter("PK2B", value)?; self.params.p1006 = value; self.mark_param_given(1006); self.recompute_instance_static(); Ok(()) }
            "pdk2b" => { validate_finite_parameter("PDK2B", value)?; self.params.p1007 = value; self.mark_param_given(1007); self.recompute_instance_static(); Ok(()) }
            "pdvbd0" => { validate_finite_parameter("PDVBD0", value)?; self.params.p1008 = value; self.mark_param_given(1008); self.recompute_instance_static(); Ok(()) }
            "pdvbd1" => { validate_finite_parameter("PDVBD1", value)?; self.params.p1009 = value; self.mark_param_given(1009); self.recompute_instance_static(); Ok(()) }
            "pmoinfd" => { validate_finite_parameter("PMOINFD", value)?; self.params.p1010 = value; self.mark_param_given(1010); self.recompute_instance_static(); Ok(()) }
            "pvbs0pd" => { validate_finite_parameter("PVBS0PD", value)?; self.params.p1011 = value; self.mark_param_given(1011); self.recompute_instance_static(); Ok(()) }
            "pvbs0fd" => { validate_finite_parameter("PVBS0FD", value)?; self.params.p1012 = value; self.mark_param_given(1012); self.recompute_instance_static(); Ok(()) }
            "nlx" => { validate_finite_parameter("NLX", value)?; self.params.p1013 = value; self.mark_param_given(1013); self.recompute_instance_static(); Ok(()) }
            "lnlx" => { validate_finite_parameter("LNLX", value)?; self.params.p1014 = value; self.mark_param_given(1014); self.recompute_instance_static(); Ok(()) }
            "wnlx" => { validate_finite_parameter("WNLX", value)?; self.params.p1015 = value; self.mark_param_given(1015); self.recompute_instance_static(); Ok(()) }
            "pnlx" => { validate_finite_parameter("PNLX", value)?; self.params.p1016 = value; self.mark_param_given(1016); self.recompute_instance_static(); Ok(()) }
            "ngidl" => { validate_finite_parameter("NGIDL", value)?; self.params.p1017 = value; self.mark_param_given(1017); self.recompute_instance_static(); Ok(()) }
            "lngidl" => { validate_finite_parameter("LNGIDL", value)?; self.params.p1018 = value; self.mark_param_given(1018); self.recompute_instance_static(); Ok(()) }
            "wngidl" => { validate_finite_parameter("WNGIDL", value)?; self.params.p1019 = value; self.mark_param_given(1019); self.recompute_instance_static(); Ok(()) }
            "pngidl" => { validate_finite_parameter("PNGIDL", value)?; self.params.p1020 = value; self.mark_param_given(1020); self.recompute_instance_static(); Ok(()) }
            "lpe0" => { validate_finite_parameter("LPE0", value)?; self.params.p1021 = value; self.mark_param_given(1021); self.recompute_instance_static(); Ok(()) }
            "egidl" => { validate_finite_parameter("EGIDL", value)?; self.params.p1022 = value; self.mark_param_given(1022); self.recompute_instance_static(); Ok(()) }
            "egisl" => { validate_finite_parameter("EGISL", value)?; self.params.p1023 = value; self.mark_param_given(1023); self.recompute_instance_static(); Ok(()) }
            "llpe0" => { validate_finite_parameter("LLPE0", value)?; self.params.p1024 = value; self.mark_param_given(1024); self.recompute_instance_static(); Ok(()) }
            "legidl" => { validate_finite_parameter("LEGIDL", value)?; self.params.p1025 = value; self.mark_param_given(1025); self.recompute_instance_static(); Ok(()) }
            "legisl" => { validate_finite_parameter("LEGISL", value)?; self.params.p1026 = value; self.mark_param_given(1026); self.recompute_instance_static(); Ok(()) }
            "wlpe0" => { validate_finite_parameter("WLPE0", value)?; self.params.p1027 = value; self.mark_param_given(1027); self.recompute_instance_static(); Ok(()) }
            "wegidl" => { validate_finite_parameter("WEGIDL", value)?; self.params.p1028 = value; self.mark_param_given(1028); self.recompute_instance_static(); Ok(()) }
            "wegisl" => { validate_finite_parameter("WEGISL", value)?; self.params.p1029 = value; self.mark_param_given(1029); self.recompute_instance_static(); Ok(()) }
            "plpe0" => { validate_finite_parameter("PLPE0", value)?; self.params.p1030 = value; self.mark_param_given(1030); self.recompute_instance_static(); Ok(()) }
            "pegidl" => { validate_finite_parameter("PEGIDL", value)?; self.params.p1031 = value; self.mark_param_given(1031); self.recompute_instance_static(); Ok(()) }
            "pegisl" => { validate_finite_parameter("PEGISL", value)?; self.params.p1032 = value; self.mark_param_given(1032); self.recompute_instance_static(); Ok(()) }
            "eggbcp2" => { validate_finite_parameter("EGGBCP2", value)?; self.params.p1033 = value; self.mark_param_given(1033); self.recompute_instance_static(); Ok(()) }
            "eggdep" => { validate_finite_parameter("EGGDEP", value)?; self.params.p1034 = value; self.mark_param_given(1034); self.recompute_instance_static(); Ok(()) }
            "agb1" => { validate_finite_parameter("AGB1", value)?; self.params.p1035 = value; self.mark_param_given(1035); self.recompute_instance_static(); Ok(()) }
            "bgb1" => { validate_finite_parameter("BGB1", value)?; self.params.p1036 = value; self.mark_param_given(1036); self.recompute_instance_static(); Ok(()) }
            "agb2" => { validate_finite_parameter("AGB2", value)?; self.params.p1037 = value; self.mark_param_given(1037); self.recompute_instance_static(); Ok(()) }
            "bgb2" => { validate_finite_parameter("BGB2", value)?; self.params.p1038 = value; self.mark_param_given(1038); self.recompute_instance_static(); Ok(()) }
            "agbc2n" => { validate_finite_parameter("AGBC2N", value)?; self.params.p1039 = value; self.mark_param_given(1039); self.recompute_instance_static(); Ok(()) }
            "agbc2p" => { validate_finite_parameter("AGBC2P", value)?; self.params.p1040 = value; self.mark_param_given(1040); self.recompute_instance_static(); Ok(()) }
            "bgbc2n" => { validate_finite_parameter("BGBC2N", value)?; self.params.p1041 = value; self.mark_param_given(1041); self.recompute_instance_static(); Ok(()) }
            "bgbc2p" => { validate_finite_parameter("BGBC2P", value)?; self.params.p1042 = value; self.mark_param_given(1042); self.recompute_instance_static(); Ok(()) }
            "vtm00" => { validate_parameter("VTM00", value, Some((0.0, "0.0")), true, None, true, &[])?; self.params.p1043 = value; self.mark_param_given(1043); self.recompute_instance_static(); Ok(()) }
            _ => Err(format!("unknown parameter '{}' for generated Verilog-A model 'bsimsoi'", name)),
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
        let param_given = self.param_given.as_ref();
        let v0: f64 = p.p21;
        self.scalar_v0 = v0;
        let v1: f64 = p.p348;
        self.scalar_v1 = v1;
        let v2: f64 = p.p41;
        self.scalar_v2 = v2;
        let v5: f64 = (if (p.p41 != 0.0) { 3.9 } else { 0.0 });
        self.scalar_v5 = v5;
        let v6: f64 = p.p45;
        self.scalar_v6 = v6;
        let v7: f64 = (if (p.p41 != 0.0) { p.p45 } else { 0.0 });
        self.scalar_v7 = v7;
        let v9: f64 = p.p47;
        self.scalar_v9 = v9;
        let v10: f64 = (8.85418e-12 * p.p47);
        self.scalar_v10 = v10;
        let v11: f64 = (if (p.p41 != 0.0) { v10 } else { 0.0 });
        self.scalar_v11 = v11;
        let v13: f64 = (v5 * 8.85418e-12);
        self.scalar_v13 = v13;
        let v14: f64 = (v13 / v7);
        self.scalar_v14 = v14;
        let v15: f64 = (if (p.p41 != 0.0) { v14 } else { 0.0 });
        self.scalar_v15 = v15;
        let v16: bool = (!(p.p41 != 0.0));
        self.scalar_v16 = v16;
        let v17: f64 = p.p66;
        self.scalar_v17 = v17;
        let v19: f64 = (if v16 { 1.03594e-10 } else { v11 });
        self.scalar_v19 = v19;
        let v21: f64 = (3.453133e-11 / p.p66);
        self.scalar_v21 = v21;
        let v22: f64 = (if v16 { v21 } else { v15 });
        self.scalar_v22 = v22;
        let v24: bool = (p.p21 == 2.0);
        self.scalar_v24 = v24;
        let v25: f64 = p.p36;
        self.scalar_v25 = v25;
        let v26: bool = (0.0 == p.p36);
        self.scalar_v26 = v26;
        let v27: f64 = p.p35;
        self.scalar_v27 = v27;
        let v28: bool = (0.0 == p.p35);
        self.scalar_v28 = v28;
        let v32: f64 = (if v24 { 0.0 } else { 0.0 });
        self.scalar_v32 = v32;
        let v33: bool = (!v24);
        self.scalar_v33 = v33;
        let v34: bool = (false && v33);
        self.scalar_v34 = v34;
        let v35: f64 = (if v34 { 0.0 } else { v32 });
        self.scalar_v35 = v35;
        let v36: bool = (p.p348 == 0.0);
        self.scalar_v36 = v36;
        let v37: f64 = p.p349;
        self.scalar_v37 = v37;
        let v38: bool = (0.0 == p.p349);
        self.scalar_v38 = v38;
        let v39: bool = (v36 && v38);
        self.scalar_v39 = v39;
        let v40: bool = (true && v33);
        self.scalar_v40 = v40;
        let v41: bool = (false && v40);
        self.scalar_v41 = v41;
        let v42: bool = (v39 && v41);
        self.scalar_v42 = v42;
        let v43: f64 = (if v42 { 2.0 } else { v35 });
        self.scalar_v43 = v43;
        let v44: bool = (!v39);
        self.scalar_v44 = v44;
        let v45: bool = (v41 && v44);
        self.scalar_v45 = v45;
        let v46: f64 = (if v45 { 1.0 } else { v43 });
        self.scalar_v46 = v46;
        let v47: bool = (true && v40);
        self.scalar_v47 = v47;
        let v48: bool = (v39 && v47);
        self.scalar_v48 = v48;
        let v49: f64 = (if v48 { 1.0 } else { v46 });
        self.scalar_v49 = v49;
        let v50: bool = (v44 && v47);
        self.scalar_v50 = v50;
        let v51: f64 = (if v50 { 1.0 } else { v49 });
        self.scalar_v51 = v51;
        let v53: f64 = p.p49;
        self.scalar_v53 = v53;
        let v54: f64 = p.p1;
        self.scalar_v54 = v54;
        let v55: f64 = p.p2;
        self.scalar_v55 = v55;
        let v56: f64 = p.p3;
        self.scalar_v56 = v56;
        let v57: f64 = (p.p2 / p.p3);
        self.scalar_v57 = v57;
        let v58: f64 = p.p190;
        self.scalar_v58 = v58;
        let v59: f64 = f64::powf(p.p1, p.p190);
        self.scalar_v59 = v59;
        let v60: f64 = p.p193;
        self.scalar_v60 = v60;
        let v61: f64 = f64::powf(v57, p.p193);
        self.scalar_v61 = v61;
        let v62: f64 = p.p188;
        self.scalar_v62 = v62;
        let v63: f64 = (p.p188 / v59);
        self.scalar_v63 = v63;
        let v64: f64 = p.p191;
        self.scalar_v64 = v64;
        let v65: f64 = (p.p191 / v61);
        self.scalar_v65 = v65;
        let v66: f64 = (v63 + v65);
        self.scalar_v66 = v66;
        let v67: f64 = p.p194;
        self.scalar_v67 = v67;
        let v68: f64 = (v59 * v61);
        self.scalar_v68 = v68;
        let v69: f64 = (p.p194 / v68);
        self.scalar_v69 = v69;
        let v70: f64 = (v66 + v69);
        self.scalar_v70 = v70;
        let v71: f64 = p.p187;
        self.scalar_v71 = v71;
        let v72: f64 = (v70 + p.p187);
        self.scalar_v72 = v72;
        let v73: f64 = p.p202;
        self.scalar_v73 = v73;
        let v74: f64 = f64::powf(p.p1, p.p202);
        self.scalar_v74 = v74;
        let v75: f64 = p.p205;
        self.scalar_v75 = v75;
        let v76: f64 = f64::powf(v57, p.p205);
        self.scalar_v76 = v76;
        let v77: f64 = p.p200;
        self.scalar_v77 = v77;
        let v78: f64 = (p.p200 / v74);
        self.scalar_v78 = v78;
        let v79: f64 = p.p203;
        self.scalar_v79 = v79;
        let v80: f64 = (p.p203 / v76);
        self.scalar_v80 = v80;
        let v81: f64 = (v78 + v80);
        self.scalar_v81 = v81;
        let v82: f64 = p.p206;
        self.scalar_v82 = v82;
        let v83: f64 = (v74 * v76);
        self.scalar_v83 = v83;
        let v84: f64 = (p.p206 / v83);
        self.scalar_v84 = v84;
        let v85: f64 = (v81 + v84);
        self.scalar_v85 = v85;
        let v86: f64 = p.p197;
        self.scalar_v86 = v86;
        let v87: f64 = (v85 + p.p197);
        self.scalar_v87 = v87;
        let v88: f64 = (2.0 * v72);
        self.scalar_v88 = v88;
        let v89: f64 = (p.p1 - v88);
        self.scalar_v89 = v89;
        let v90: f64 = p.p22;
        self.scalar_v90 = v90;
        let v91: f64 = p.p303;
        self.scalar_v91 = v91;
        let v92: f64 = (p.p22 * p.p303);
        self.scalar_v92 = v92;
        let v93: f64 = (v57 - v92);
        self.scalar_v93 = v93;
        let v94: f64 = (2.0 - p.p22);
        self.scalar_v94 = v94;
        let v95: f64 = (v87 * v94);
        self.scalar_v95 = v95;
        let v96: f64 = (v93 - v95);
        self.scalar_v96 = v96;
        let v97: f64 = p.p23;
        self.scalar_v97 = v97;
        let v98: f64 = (v96 / p.p23);
        self.scalar_v98 = v98;
        let v99: f64 = p.p85;
        self.scalar_v99 = v99;
        let v100: f64 = p.p65;
        self.scalar_v100 = v100;
        let v101: bool = (1.0 == p.p65);
        self.scalar_v101 = v101;
        let v103: f64 = (1e-6 / v89);
        self.scalar_v103 = v103;
        let v104: f64 = (if v101 { v103 } else { 0.0 });
        self.scalar_v104 = v104;
        let v105: f64 = (1e-6 / v96);
        self.scalar_v105 = v105;
        let v106: f64 = (if v101 { v105 } else { 0.0 });
        self.scalar_v106 = v106;
        let v108: f64 = (v89 * v96);
        self.scalar_v108 = v108;
        let v109: f64 = (1e-12 / v108);
        self.scalar_v109 = v109;
        let v110: f64 = (if v101 { v109 } else { 0.0 });
        self.scalar_v110 = v110;
        let v111: bool = (!v101);
        self.scalar_v111 = v111;
        let v112: f64 = (1.0 / v89);
        self.scalar_v112 = v112;
        let v113: f64 = (if v111 { v112 } else { v104 });
        self.scalar_v113 = v113;
        let v114: f64 = (1.0 / v96);
        self.scalar_v114 = v114;
        let v115: f64 = (if v111 { v114 } else { v106 });
        self.scalar_v115 = v115;
        let v116: f64 = (1.0 / v108);
        self.scalar_v116 = v116;
        let v117: f64 = (if v111 { v116 } else { v110 });
        self.scalar_v117 = v117;
        let v118: f64 = p.p82;
        self.scalar_v118 = v118;
        let v119: f64 = p.p488;
        self.scalar_v119 = v119;
        let v120: f64 = (v113 * p.p488);
        self.scalar_v120 = v120;
        let v121: f64 = (p.p82 + v120);
        self.scalar_v121 = v121;
        let v122: f64 = p.p678;
        self.scalar_v122 = v122;
        let v123: f64 = (v115 * p.p678);
        self.scalar_v123 = v123;
        let v124: f64 = (v121 + v123);
        self.scalar_v124 = v124;
        let v125: f64 = p.p868;
        self.scalar_v125 = v125;
        let v126: f64 = (v117 * p.p868);
        self.scalar_v126 = v126;
        let v127: f64 = (v124 + v126);
        self.scalar_v127 = v127;
        let v128: f64 = p.p1021;
        self.scalar_v128 = v128;
        let v129: f64 = p.p413;
        self.scalar_v129 = v129;
        let v130: f64 = p.p620;
        self.scalar_v130 = v130;
        let v131: f64 = (v113 * p.p620);
        self.scalar_v131 = v131;
        let v132: f64 = (p.p413 + v131);
        self.scalar_v132 = v132;
        let v133: f64 = p.p810;
        self.scalar_v133 = v133;
        let v134: f64 = (v115 * p.p810);
        self.scalar_v134 = v134;
        let v135: f64 = (v132 + v134);
        self.scalar_v135 = v135;
        let v136: f64 = p.p1000;
        self.scalar_v136 = v136;
        let v137: f64 = (v117 * p.p1000);
        self.scalar_v137 = v137;
        let v138: f64 = (v135 + v137);
        self.scalar_v138 = v138;
        let v139: f64 = p.p411;
        self.scalar_v139 = v139;
        let v140: f64 = p.p631;
        self.scalar_v140 = v140;
        let v141: f64 = (v113 * p.p631);
        self.scalar_v141 = v141;
        let v142: f64 = (p.p411 + v141);
        self.scalar_v142 = v142;
        let v143: f64 = p.p821;
        self.scalar_v143 = v143;
        let v144: f64 = (v115 * p.p821);
        self.scalar_v144 = v144;
        let v145: f64 = (v142 + v144);
        self.scalar_v145 = v145;
        let v146: f64 = p.p1011;
        self.scalar_v146 = v146;
        let v147: f64 = (v117 * p.p1011);
        self.scalar_v147 = v147;
        let v148: f64 = (v145 + v147);
        self.scalar_v148 = v148;
        let v149: f64 = p.p412;
        self.scalar_v149 = v149;
        let v150: f64 = p.p632;
        self.scalar_v150 = v150;
        let v151: f64 = (v113 * p.p632);
        self.scalar_v151 = v151;
        let v152: f64 = (p.p412 + v151);
        self.scalar_v152 = v152;
        let v153: f64 = p.p822;
        self.scalar_v153 = v153;
        let v154: f64 = (v115 * p.p822);
        self.scalar_v154 = v154;
        let v155: f64 = (v152 + v154);
        self.scalar_v155 = v155;
        let v156: f64 = p.p1012;
        self.scalar_v156 = v156;
        let v157: f64 = (v117 * p.p1012);
        self.scalar_v157 = v157;
        let v158: f64 = (v155 + v157);
        self.scalar_v158 = v158;
        let v161: f64 = p.p14;
        self.scalar_v161 = v161;
        let v162: f64 = p.p429;
        self.scalar_v162 = v162;
        let v163: bool = (1.0 == p.p429);
        self.scalar_v163 = v163;
        let v164: f64 = if param_given[82] { 1.0 } else { 0.0 };
        self.scalar_v164 = v164;
        let v165: bool = (!(if param_given[82] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v165 = v165;
        let v166: f64 = if param_given[85] { 1.0 } else { 0.0 };
        self.scalar_v166 = v166;
        let v167: bool = (v165 && (if param_given[85] { 1.0 } else { 0.0 } != 0.0));
        self.scalar_v167 = v167;
        let v168: f64 = (v22 * p.p85);
        self.scalar_v168 = v168;
        let v169: f64 = (if v167 { v168 } else { v59 });
        self.scalar_v169 = v169;
        let v171: f64 = (v169 * 3.021e22);
        self.scalar_v171 = v171;
        let v172: f64 = (v169 * v171);
        self.scalar_v172 = v172;
        let v173: f64 = (if v167 { v172 } else { v127 });
        self.scalar_v173 = v173;
        let v174: bool = ((p.p41 != 0.0) && v24);
        self.scalar_v174 = v174;
        let v175: f64 = (p.p49 - 0.1);
        self.scalar_v175 = v175;
        let v176: f64 = (v175 / 1.602176462e-19);
        self.scalar_v176 = v176;
        let v178: f64 = (v176 * 2e-6);
        self.scalar_v178 = v178;
        let v179: f64 = (v19 * v178);
        self.scalar_v179 = v179;
        let v180: f64 = p.p156;
        self.scalar_v180 = v180;
        let v181: f64 = (p.p156 * p.p156);
        self.scalar_v181 = v181;
        let v182: f64 = (v179 / v181);
        self.scalar_v182 = v182;
        let v183: f64 = (if v174 { v182 } else { 0.0 });
        self.scalar_v183 = v183;
        let v184: bool = (v173 > v183);
        self.scalar_v184 = v184;
        let v185: bool = (v174 && v184);
        self.scalar_v185 = v185;
        let v186: f64 = (if v185 { v183 } else { v173 });
        self.scalar_v186 = v186;
        let v187: bool = (v16 && v24);
        self.scalar_v187 = v187;
        let v189: f64 = (v19 * 12732679878803.51);
        self.scalar_v189 = v189;
        let v190: f64 = p.p155;
        self.scalar_v190 = v190;
        let v191: f64 = (p.p155 * p.p155);
        self.scalar_v191 = v191;
        let v192: f64 = (v189 / v191);
        self.scalar_v192 = v192;
        let v193: f64 = (if v187 { v192 } else { v183 });
        self.scalar_v193 = v193;
        let v194: bool = (v186 > v193);
        self.scalar_v194 = v194;
        let v195: bool = (v187 && v194);
        self.scalar_v195 = v195;
        let v196: f64 = (if v195 { v193 } else { v186 });
        self.scalar_v196 = v196;
        let v197: f64 = (1.03594e-10 / p.p156);
        self.scalar_v197 = v197;
        let v198: f64 = (if (p.p41 != 0.0) { v197 } else { 0.0 });
        self.scalar_v198 = v198;
        let v199: f64 = (1.03594e-10 / p.p155);
        self.scalar_v199 = v199;
        let v200: f64 = (if v16 { v199 } else { v198 });
        self.scalar_v200 = v200;
        let v201: f64 = (1.602176462e-19 * v196);
        self.scalar_v201 = v201;
        let v202: f64 = (p.p1021 / p.p1);
        self.scalar_v202 = v202;
        let v203: f64 = (1.0 + v202);
        self.scalar_v203 = v203;
        let v204: f64 = (v201 * v203);
        self.scalar_v204 = v204;
        let v205: f64 = (1000000.0 * v204);
        self.scalar_v205 = v205;
        let v206: f64 = (p.p156 * v205);
        self.scalar_v206 = v206;
        let v207: f64 = (if (p.p41 != 0.0) { v206 } else { 0.0 });
        self.scalar_v207 = v207;
        let v208: f64 = (p.p155 * v205);
        self.scalar_v208 = v208;
        let v209: f64 = (if v16 { v208 } else { v207 });
        self.scalar_v209 = v209;
        let v211: f64 = (0.5 * v209);
        self.scalar_v211 = v211;
        let v212: f64 = (v211 / v200);
        self.scalar_v212 = v212;
        let v213: f64 = (0.8 - v212);
        self.scalar_v213 = v213;
        let v214: f64 = (v138 + v213);
        self.scalar_v214 = v214;
        let v216: bool = (p.p21 == 3.0);
        self.scalar_v216 = v216;
        let v217: bool = (v214 > v158);
        self.scalar_v217 = v217;
        let v218: bool = (v216 && v217);
        self.scalar_v218 = v218;
        let v219: f64 = (if v218 { 2.0 } else { p.p21 });
        self.scalar_v219 = v219;
        let v220: bool = (v214 < v148);
        self.scalar_v220 = v220;
        let v221: bool = (!v217);
        self.scalar_v221 = v221;
        let v222: bool = (v216 && v221);
        self.scalar_v222 = v222;
        let v223: bool = (v220 && v222);
        self.scalar_v223 = v223;
        let v224: f64 = (if v223 { 0.0 } else { v219 });
        self.scalar_v224 = v224;
        let v225: bool = (!v220);
        self.scalar_v225 = v225;
        let v226: bool = (v222 && v225);
        self.scalar_v226 = v226;
        let v227: f64 = (if v226 { 1.0 } else { v224 });
        self.scalar_v227 = v227;
        let v228: f64 = p.p131;
        self.scalar_v228 = v228;
        let v229: f64 = p.p11;
        self.scalar_v229 = v229;
        let v230: f64 = (p.p131 * p.p11);
        self.scalar_v230 = v230;
        let v231: f64 = p.p431;
        self.scalar_v231 = v231;
        let v232: bool = (v230 < p.p431);
        self.scalar_v232 = v232;
        let v233: bool = (v163 && v232);
        self.scalar_v233 = v233;
        let v234: f64 = (if v233 { p.p431 } else { v230 });
        self.scalar_v234 = v234;
        let v235: f64 = p.p12;
        self.scalar_v235 = v235;
        let v236: f64 = (p.p131 * p.p12);
        self.scalar_v236 = v236;
        let v237: bool = (v236 < p.p431);
        self.scalar_v237 = v237;
        let v238: bool = (v163 && v237);
        self.scalar_v238 = v238;
        let v239: f64 = (if v238 { p.p431 } else { v236 });
        self.scalar_v239 = v239;
        let v240: f64 = p.p424;
        self.scalar_v240 = v240;
        let v241: f64 = p.p427;
        self.scalar_v241 = v241;
        let v242: f64 = (v98 / 3.0);
        self.scalar_v242 = v242;
        let v243: f64 = p.p425;
        self.scalar_v243 = v243;
        let v244: f64 = (v242 / p.p425);
        self.scalar_v244 = v244;
        let v245: f64 = (p.p427 + v244);
        self.scalar_v245 = v245;
        let v246: f64 = (p.p424 * v245);
        self.scalar_v246 = v246;
        let v247: f64 = (p.p3 * p.p425);
        self.scalar_v247 = v247;
        let v248: f64 = p.p428;
        self.scalar_v248 = v248;
        let v249: f64 = (p.p1 - p.p428);
        self.scalar_v249 = v249;
        let v250: f64 = (v247 * v249);
        self.scalar_v250 = v250;
        let v251: f64 = (v246 / v250);
        self.scalar_v251 = v251;
        let v252: f64 = p.p426;
        self.scalar_v252 = v252;
        let v253: f64 = (p.p1 * v96);
        self.scalar_v253 = v253;
        let v254: f64 = (p.p3 * v253);
        self.scalar_v254 = v254;
        let v255: f64 = (p.p426 / v254);
        self.scalar_v255 = v255;
        let v256: f64 = (v251 + v255);
        self.scalar_v256 = v256;
        let v257: bool = (v256 > 0.0);
        self.scalar_v257 = v257;
        let v258: f64 = (1.0 / v256);
        self.scalar_v258 = v258;
        let v259: f64 = (if v257 { v258 } else { v256 });
        self.scalar_v259 = v259;
        let v260: bool = (!v257);
        self.scalar_v260 = v260;
        let v262: f64 = (if v260 { 1000.0 } else { v259 });
        self.scalar_v262 = v262;
        let v263: f64 = p.p39;
        self.scalar_v263 = v263;
        let v264: f64 = p.p18;
        self.scalar_v264 = v264;
        let v266: bool = (p.p18 < 0.001);
        self.scalar_v266 = v266;
        let v267: f64 = p.p40;
        self.scalar_v267 = v267;
        let v268: bool = (v266 && (p.p40 != 0.0));
        self.scalar_v268 = v268;
        let v269: f64 = (if v268 { 1000.0 } else { 0.0 });
        self.scalar_v269 = v269;
        let v270: bool = (!v266);
        self.scalar_v270 = v270;
        let v271: bool = ((p.p40 != 0.0) && v270);
        self.scalar_v271 = v271;
        let v272: f64 = p.p255;
        self.scalar_v272 = v272;
        let v273: f64 = (1.0 / p.p18);
        self.scalar_v273 = v273;
        let v274: f64 = (p.p255 + v273);
        self.scalar_v274 = v274;
        let v275: f64 = (if v271 { v274 } else { v269 });
        self.scalar_v275 = v275;
        let v276: f64 = p.p19;
        self.scalar_v276 = v276;
        let v277: bool = (p.p19 < 0.001);
        self.scalar_v277 = v277;
        let v278: bool = ((p.p40 != 0.0) && v277);
        self.scalar_v278 = v278;
        let v279: f64 = (if v278 { 1000.0 } else { 0.0 });
        self.scalar_v279 = v279;
        let v280: bool = (!v277);
        self.scalar_v280 = v280;
        let v281: bool = ((p.p40 != 0.0) && v280);
        self.scalar_v281 = v281;
        let v282: f64 = (1.0 / p.p19);
        self.scalar_v282 = v282;
        let v283: f64 = (p.p255 + v282);
        self.scalar_v283 = v283;
        let v284: f64 = (if v281 { v283 } else { v279 });
        self.scalar_v284 = v284;
        let v285: bool = (!(p.p40 != 0.0));
        self.scalar_v285 = v285;
        let v286: f64 = (if v285 { 0.0 } else { v275 });
        self.scalar_v286 = v286;
        let v287: f64 = (if v285 { 0.0 } else { v284 });
        self.scalar_v287 = v287;
        let v288: bool = (p.p36 == 1.0);
        self.scalar_v288 = v288;
        let v289: bool = (0.0 != p.p14);
        self.scalar_v289 = v289;
        let v290: bool = (v288 && v289);
        self.scalar_v290 = v290;
        let v291: bool = ((p.p35 != 0.0) && false);
        self.scalar_v291 = v291;
        let v293: bool = (!v291);
        self.scalar_v293 = v293;
        let v294: bool = (!v290);
        self.scalar_v294 = v294;
        let v298: bool = (2.0 == v227);
        self.scalar_v298 = v298;
        let v299: f64 = p.p135;
        self.scalar_v299 = v299;
        let v300: f64 = p.p136;
        self.scalar_v300 = v300;
        let v301: bool = (0.0 == v51);
        self.scalar_v301 = v301;
        let v302: bool = (2.0 == v51);
        self.scalar_v302 = v302;
        let v303: bool = (v301 || v302);
        self.scalar_v303 = v303;
        let v304: bool = (!v303);
        self.scalar_v304 = v304;
        let v305: bool = (2.0 == p.p39);
        self.scalar_v305 = v305;
        let v306: f64 = (v239 + p.p135);
        self.scalar_v306 = v306;
        let v307: f64 = (v234 + p.p136);
        self.scalar_v307 = v307;
        let v308: f64 = (if v298 { 0.0 } else { 0.0 });
        self.scalar_v308 = v308;
        let v309: f64 = p.p223;
        self.scalar_v309 = v309;
        let v310: bool = (0.0 == p.p223);
        self.scalar_v310 = v310;
        let v311: bool = (1.0 == p.p223);
        self.scalar_v311 = v311;
        let v312: bool = (2.0 == p.p223);
        self.scalar_v312 = v312;
        let v313: bool = (3.0 == p.p223);
        self.scalar_v313 = v313;
        let v314: bool = (!v310);
        self.scalar_v314 = v314;
        let v315: bool = (v311 && v314);
        self.scalar_v315 = v315;
        let v316: bool = (v310 || v311);
        self.scalar_v316 = v316;
        let v317: bool = (!v316);
        self.scalar_v317 = v317;
        let v318: bool = (v312 && v317);
        self.scalar_v318 = v318;
        let v319: bool = (v312 || v316);
        self.scalar_v319 = v319;
        let v320: bool = (!v319);
        self.scalar_v320 = v320;
        let v321: bool = (v313 && v320);
        self.scalar_v321 = v321;
        let v322: bool = (3.0 != p.p223);
        self.scalar_v322 = v322;
        let v323: bool = (2.0 != p.p429);
        self.scalar_v323 = v323;
        let v324: bool = (v307 >= p.p431);
        self.scalar_v324 = v324;
        let v325: bool = (v323 && v324);
        self.scalar_v325 = v325;
        let v326: bool = (v306 >= p.p431);
        self.scalar_v326 = v326;
        let v327: bool = (v323 && v326);
        self.scalar_v327 = v327;
        let v328: bool = (0.0 == p.p39);
        self.scalar_v328 = v328;
        let v329: bool = (v305 || v328);
        self.scalar_v329 = v329;
        let v330: bool = (1.0 == p.p39);
        self.scalar_v330 = v330;
        let v331: bool = (v328 || v330);
        self.scalar_v331 = v331;
        let v332: bool = (!v331);
        self.scalar_v332 = v332;
        let v333: bool = (v305 && v332);
        self.scalar_v333 = v333;
        let v334: f64 = p.p32;
        self.scalar_v334 = v334;
        let v335: bool = (v24 && v26);
        self.scalar_v335 = v335;
        let v336: bool = (v28 && v335);
        self.scalar_v336 = v336;
        let v337: bool = (true && v336);
        self.scalar_v337 = v337;
        let v338: f64 = (if v337 { 0.0 } else { 0.0 });
        self.scalar_v338 = v338;
        let v339: bool = (false && v336);
        self.scalar_v339 = v339;
        let v340: bool = ((1.0 != 0.0) && v339);
        self.scalar_v340 = v340;
        let v341: f64 = (if v340 { 0.0 } else { 0.0 });
        self.scalar_v341 = v341;
        let v342: bool = (!v28);
        self.scalar_v342 = v342;
        let v343: bool = (v335 && v342);
        self.scalar_v343 = v343;
        let v344: bool = (true && v343);
        self.scalar_v344 = v344;
        let v345: f64 = (if v344 { 0.0 } else { 0.0 });
        self.scalar_v345 = v345;
        let v346: bool = (!v26);
        self.scalar_v346 = v346;
        let v347: bool = (v24 && v346);
        self.scalar_v347 = v347;
        let v348: bool = (v28 && v347);
        self.scalar_v348 = v348;
        let v349: bool = (true && v348);
        self.scalar_v349 = v349;
        let v350: f64 = (if v349 { 0.0 } else { 0.0 });
        self.scalar_v350 = v350;
        let v351: bool = (false && v348);
        self.scalar_v351 = v351;
        let v352: bool = (true && v351);
        self.scalar_v352 = v352;
        let v353: f64 = (if v352 { 0.0 } else { 0.0 });
        self.scalar_v353 = v353;
        let v354: bool = (false && v351);
        self.scalar_v354 = v354;
        let v355: bool = ((1.0 != 0.0) && v354);
        self.scalar_v355 = v355;
        let v356: f64 = (if v355 { 0.0 } else { 0.0 });
        self.scalar_v356 = v356;
        let v357: bool = (v342 && v347);
        self.scalar_v357 = v357;
        let v358: bool = (true && v357);
        self.scalar_v358 = v358;
        let v359: f64 = (if v358 { 0.0 } else { 0.0 });
        self.scalar_v359 = v359;
        let v360: f64 = (if v310 { 0.0 } else { 0.0 });
        self.scalar_v360 = v360;
        let v361: f64 = (if v315 { 0.0 } else { 0.0 });
        self.scalar_v361 = v361;
        let v362: f64 = (if v318 { 0.0 } else { 0.0 });
        self.scalar_v362 = v362;
        let v363: f64 = (if v321 { 0.0 } else { 0.0 });
        self.scalar_v363 = v363;
        let v366: f64 = (if v325 { 0.0 } else { 0.0 });
        self.scalar_v366 = v366;
        let v367: bool = (!v325);
        self.scalar_v367 = v367;
        let v368: f64 = (if v367 { 0.0 } else { 0.0 });
        self.scalar_v368 = v368;
        let v369: f64 = (if v327 { 0.0 } else { 0.0 });
        self.scalar_v369 = v369;
        let v370: bool = (!v327);
        self.scalar_v370 = v370;
        let v371: f64 = (if v370 { 0.0 } else { 0.0 });
        self.scalar_v371 = v371;
        let v372: f64 = (if v303 { 0.0 } else { 0.0 });
        self.scalar_v372 = v372;
        let v373: f64 = (if v304 { 0.0 } else { 0.0 });
        self.scalar_v373 = v373;
        let v374: f64 = (if v329 { 0.0 } else { 0.0 });
        self.scalar_v374 = v374;
        let v375: bool = (!v329);
        self.scalar_v375 = v375;
        let v381: f64 = (if v375 { 0.0 } else { 0.0 });
        self.scalar_v381 = v381;
        let v382: f64 = (if v331 { 0.0 } else { 0.0 });
        self.scalar_v382 = v382;
        let v383: f64 = (if v333 { 0.0 } else { 0.0 });
        self.scalar_v383 = v383;
        let v392: f64 = (if (p.p40 != 0.0) { 0.0 } else { 0.0 });
        self.scalar_v392 = v392;
        let v393: f64 = (if v285 { 0.0 } else { 0.0 });
        self.scalar_v393 = v393;
        let v394: bool = (v291 && v294);
        self.scalar_v394 = v394;
        let v395: bool = ((1.0 != 0.0) && v394);
        self.scalar_v395 = v395;
        let v396: f64 = (if v395 { 0.0 } else { 0.0 });
        self.scalar_v396 = v396;
        let v397: bool = (false && v394);
        self.scalar_v397 = v397;
        let v398: bool = ((1.0 != 0.0) && v397);
        self.scalar_v398 = v398;
        let v399: f64 = (if v398 { 0.0 } else { 0.0 });
        self.scalar_v399 = v399;
        let v400: bool = (false && v397);
        self.scalar_v400 = v400;
        let v401: f64 = (if v400 { 0.0 } else { 0.0 });
        self.scalar_v401 = v401;
        let v402: bool = (v293 && v294);
        self.scalar_v402 = v402;
        let v403: f64 = (if v402 { 0.0 } else { 0.0 });
        self.scalar_v403 = v403;
        let v404: f64 = (if v322 { 1.0 } else { 0.0 });
        self.scalar_v404 = v404;
        let v405: f64 = (-p.p32);
        self.scalar_v405 = v405;
        let v406: f64 = (v262 * p.p32);
        self.scalar_v406 = v406;
        let v407: f64 = (v262 * v405);
        self.scalar_v407 = v407;
        let v408: f64 = (if v375 { v406 } else { 0.0 });
        self.scalar_v408 = v408;
        let v409: f64 = (if v375 { v407 } else { 0.0 });
        self.scalar_v409 = v409;
        let v410: f64 = (v286 * p.p32);
        self.scalar_v410 = v410;
        let v411: f64 = (v286 * v405);
        self.scalar_v411 = v411;
        let v412: f64 = (if (p.p40 != 0.0) { v410 } else { 0.0 });
        self.scalar_v412 = v412;
        let v413: f64 = (if (p.p40 != 0.0) { v411 } else { 0.0 });
        self.scalar_v413 = v413;
        let v414: f64 = (v287 * p.p32);
        self.scalar_v414 = v414;
        let v415: f64 = (v287 * v405);
        self.scalar_v415 = v415;
        let v416: f64 = (if (p.p40 != 0.0) { v414 } else { 0.0 });
        self.scalar_v416 = v416;
        let v417: f64 = (if (p.p40 != 0.0) { v415 } else { 0.0 });
        self.scalar_v417 = v417;
    }
}
