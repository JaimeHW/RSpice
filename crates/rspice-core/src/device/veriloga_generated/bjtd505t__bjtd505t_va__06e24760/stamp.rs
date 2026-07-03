#![allow(dead_code, unused_imports, unused_parens, unused_variables)]

use super::state::Instance;
use crate::device::veriloga_generated::{GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper};

#[inline]
fn eval_ddt<const STATE_COUNT: usize>(
    current: &mut [f64; STATE_COUNT],
    previous: &mut [f64; STATE_COUNT],
    older: &mut [f64; STATE_COUNT],
    initialized: &mut [bool; STATE_COUNT],
    derivative_current: &mut [f64; STATE_COUNT],
    derivative_previous: &mut [f64; STATE_COUNT],
    ddt_active: bool,
    ddt_scale: f64,
    ddt_previous_value_scale: f64,
    ddt_older_value_scale: f64,
    ddt_previous_derivative_scale: f64,
    slot: usize,
    value: f64,
) -> f64 {
    debug_assert!(slot < STATE_COUNT, "generated ddt state slot out of range");
    let previous_value = if initialized[slot] { previous[slot] } else { value };
    let older_value = if initialized[slot] { older[slot] } else { value };
    current[slot] = value;
    if ddt_active {
        let result = value * ddt_scale
            - previous_value * ddt_previous_value_scale
            - older_value * ddt_older_value_scale
            - derivative_previous[slot] * ddt_previous_derivative_scale;
        derivative_current[slot] = result;
        result
    } else {
        current[slot] = value;
        previous[slot] = value;
        older[slot] = value;
        derivative_current[slot] = 0.0;
        derivative_previous[slot] = 0.0;
        initialized[slot] = true;
        0.0
    }
}

struct CommonStampValues {
    v1: f64, v3: f64, v32: f64, v33: f64, v48: f64, v105: f64, 
    v122: f64, v123: f64, v125: f64, v127: f64, v129: f64, v130: f64, 
    v131: f64, v132: f64, v133: f64, v134: f64, v140: f64, v141: f64, 
    v142: f64, v147: bool, v149: f64, v150: f64, v154: f64, v155: f64, 
    v156: f64, v157: f64, v163: f64, v164: f64, v165: f64, v170: bool, 
    v172: f64, v173: f64, v177: f64, v178: f64, v205: f64, v229: f64, 
    v272: f64, v282: f64, v283: f64, v284: f64, v285: f64, v289: bool, 
    v291: f64, v292: f64, v293: f64, v297: f64, v298: f64, v300: f64, 
    v301: f64, v302: f64, v342: f64, v427: f64, v430: f64, v431: f64, 
    v432: f64, v434: f64, v435: f64, v438: bool, v441: f64, v443: f64, 
    v456: f64, v469: f64, v579: f64, v580: f64, v581: f64, v582: f64, 
    v584: f64, v585: f64, v586: f64, v588: f64, v591: f64, v602: f64, 
    v603: f64, v604: f64, v606: f64, v607: f64, v608: f64, v610: f64, 
    v613: f64, v722: f64, v725: f64, v726: f64, v728: f64, v731: f64, 
    v733: f64, v736: f64, v741: f64, v749: f64, v752: f64, v755: f64, 
    v759: f64, v760: f64, v796: f64, v797: f64, v799: f64, v802: bool, 
    v803: f64, v887: f64, v902: f64, v1009: f64, v1069: f64, v1094: f64, 
    v1097: f64, v1100: f64, v1127: f64, v1207: f64, v1243: f64, v1244: f64, 
    v1249: f64, v1250: f64, v1269: f64, v1271: f64, v1274: bool, v1275: f64, 
    v1284: f64, v1316: f64, v1317: f64, v1318: f64, v1320: f64, v1325: bool, 
    v1326: f64, v1333: f64, v1334: f64, v1336: f64, v1341: bool, v1343: f64, 
    v1395: f64, v1396: f64, v1397: f64, v1399: f64, v1404: bool, v1405: f64, 
    v1432: f64, v1445: f64, v1458: f64, v1471: f64, v1478: f64, v1479: f64, 
    v1481: f64, v1482: f64, v1484: f64, v1489: bool, v1490: f64, v1496: f64, 
    v1500: f64, v1503: f64, v1511: f64, v1512: f64, v1513: f64, v1515: f64, 
    v1517: f64, v1519: f64, v1520: f64, v1521: f64, v1522: f64, v1524: f64, 
    v1527: f64, v1529: f64, v1530: bool, v1535: bool, v1536: f64, v1574: f64, 
    v1576: f64, v1578: f64, v1579: f64, v1581: f64, v1582: f64, v1584: f64, 
    v1589: bool, v1590: f64, v1595: f64, v1598: f64, v1600: f64, v1608: f64, 
    v1609: f64, v1610: f64, v1612: f64, v1615: f64, v1616: f64, v1617: f64, 
    v1618: f64, v1620: f64, v1622: f64, v1624: f64, v1625: bool, v1630: bool, 
    v1631: f64, v1673: f64, v1677: f64, v1699: f64, v1716: f64, v1738: f64, 
    v1810: f64, v1822: f64, v1835: bool, v1836: bool, v1837: f64, v1840: bool, 
    v1841: f64, v1845: f64, v1846: f64, v1848: f64, v1849: f64, v1851: f64, 
    v1852: f64, v1854: f64, v1859: bool, v1860: f64, v1875: bool, v1982: bool, 
    v1983: f64, v1985: f64, v1987: f64, v1989: f64, v1991: f64, v1992: bool, 
    v1994: bool, v2002: f64, v2005: bool, v2006: f64, v2007: f64, v2013: bool, 
    v2015: f64, v2016: f64, v2020: f64, v2022: f64, v2024: f64, v2025: f64, 
    v2027: f64, v2032: bool, v2033: f64, v2092: f64, v2427: f64, v2466: f64, 
    v2498: f64, v2534: f64, v2537: f64, v2540: f64, v2543: f64, v2547: f64, 
    v2551: f64, v2559: f64, v2565: f64, v2576: f64, v2585: f64, v2586: f64, 
    v2587: f64, v2590: f64, v2591: f64, v2661: f64, v2684: f64, v2728: f64, 
    v2732: f64, v2737: f64, v2754: f64, v2756: f64, v2761: f64, v2792: f64, 
    v2835: f64, v2837: f64, v2865: f64, v2961: f64, v3036: f64, v3098: f64, 
    v3099: f64, v3149: f64, v3150: f64, v3151: f64, v3152: f64, v3153: f64, 
    v3331: f64, v3332: f64, v3333: f64, v3334: f64, v3341: f64, v3733: f64, 
    v3734: f64, v3735: f64, v3736: f64, v3944: f64, v3945: f64, v3946: f64, 
    v3947: f64, v4000: f64, v4001: f64, v4002: f64, v4003: f64, v4012: f64, 
    v4013: f64, v4014: f64, v4015: f64, v4024: f64, v4025: f64, v4026: f64, 
    v4027: f64, v4086: f64, v4087: f64, v4088: f64, v4377: f64, v4378: f64, 
    v4379: f64, v4380: f64, v4516: f64, v4517: f64, v4518: f64, v4519: f64, 
    v4520: f64, v4523: f64, v4526: f64, v4529: f64, v4532: f64, v4535: f64, 
    v4539: f64, v4540: f64, v4541: f64, v4542: f64, v4545: f64, v4547: f64, 
    v4555: f64, v4557: f64, v4593: f64, v4594: f64, v4658: f64, v4659: f64, 
    v4660: f64, v4856: f64, v4857: f64, v4858: f64, v4859: f64, v4940: f64, 
    v4941: f64, v4942: f64, v4943: f64, v4963: f64, v4964: f64, v4965: f64, 
    v4966: f64, v4994: f64, v4995: f64, v4996: f64, v4997: f64, v4998: f64, 
    v4999: f64, v5023: f64, v5024: f64, v5025: f64, v5026: f64, v5027: f64, 
    v5028: f64, v5601: f64, v5614: f64, v5701: f64, v5702: f64, v5703: f64, 
    v5704: f64, v5705: f64, v5736: f64, v5737: f64, v5738: f64, v5739: f64, 
    v5740: f64, v5741: f64, v5742: f64, v5743: f64, v5744: f64, v5888: f64, 
    v5889: f64, v5890: f64, v5891: f64, v5892: f64, v5893: f64, v5894: f64, 
    v5895: f64, v5896: f64, v6242: f64, v6243: f64, v6244: f64, v6245: f64, 
    v6246: f64, v8898: f64, v8899: f64, v8900: f64, v8901: f64, v8902: f64, 
    v8903: f64, v8904: f64, v9100: f64, v9101: f64, v9102: f64, v9103: f64, 
    v9104: f64, v9105: f64, v9106: f64, v9121: f64, v9122: f64, v9123: f64, 
    v9130: f64, v9131: f64, v9132: f64, v9133: f64, v9134: f64, v9135: f64, 
    v9136: f64, v9151: f64, v9152: f64, v9153: f64, v9154: f64, v9155: f64, 
    v9156: f64, v9157: f64, v9216: f64, v9217: f64, v9218: f64, v9219: f64, 
    v9220: f64, v9221: f64, v9222: f64, v9223: f64, v9224: f64, v9268: f64, 
    v9269: f64, v9270: f64, v9271: f64, v9272: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=1.0;
        let v3=0.0;
        let v32=0.001;
        let v33=2.0;
        let v46=0.05;
        let v48=0.1;
        let v105=ctx.node_voltage(nodes[3]);
        let v107=(if (v105<v3){v1}else{v3});
        let v108=(v1-v105);
        let v111=(if (v107!=0.0){(-(v108).ln())}else{v105});
        let v114=(if (v111<self.scalar_static_f64[83]){v1}else{v3});
        let v116=(!(v114!=0.0));
        let v118=(v1+(v111-self.scalar_static_f64[83]));
        let v122=(self.scalar_static_f64[397]+(if v116{(self.scalar_static_f64[83]+(v118).ln())}else{(if (v114!=0.0){v111}else{v3})}));
        let v123=(v122/self.scalar_static_f64[9]);
        let v124=8.617086918058125e-5;
        let v125=(v122*v124);
        let v127=(v1/v125);
        let v129=(v127-self.scalar_static_f64[85]);
        let v130=(v122-self.scalar_static_f64[9]);
        let v131=(v123).ln();
        let v132=(self.scalar_static_f64[23]*v122);
        let v133=(v122*v132);
        let v134=(self.scalar_static_f64[26]+v122);
        let v136=(self.scalar_static_f64[45]-(v133/v134));
        let v138=((v136-v46)/v48);
        let v140=(if (v136<v46){v1}else{v3});
        let v141=(v138).exp();
        let v142=(v1+v141);
        let v147=(!(v140!=0.0));
        let v149=((-v138)).exp();
        let v150=(v1+v149);
        let v154=(if v147{(v136+(v48*(v150).ln()))}else{(if (v140!=0.0){(v46+(v48*(v142).ln()))}else{v3})});
        let v155=(self.scalar_static_f64[55]*v122);
        let v156=(v122*v155);
        let v157=(self.scalar_static_f64[58]+v122);
        let v159=(self.scalar_static_f64[77]-(v156/v157));
        let v161=((v159-v46)/v48);
        let v163=(if (v159<v46){v1}else{v3});
        let v164=(v161).exp();
        let v165=(v1+v164);
        let v170=(!(v163!=0.0));
        let v172=((-v161)).exp();
        let v173=(v1+v172);
        let v177=(if v170{(v159+(v48*(v173).ln()))}else{(if (v163!=0.0){(v46+(v48*(v165).ln()))}else{v3})});
        let v178=3.0;
        let v179=-3.0;
        let v180=(v125*v179);
        let v181=(v131*v180);
        let v184=(v1-v123);
        let v187=((v181+(self.scalar_static_f64[47]*v123))+(v184*self.scalar_static_f64[86]));
        let v188=(v46-v187);
        let v189=(v188/v125);
        let v191=(if (v46<v187){v1}else{v3});
        let v192=(v189).exp();
        let v193=(v1+v192);
        let v194=(v193).ln();
        let v198=(!(v191!=0.0));
        let v200=((-v189)).exp();
        let v201=(v1+v200);
        let v202=(v201).ln();
        let v205=(if v198{(v46+(v125*v202))}else{(if (v191!=0.0){(v187+(v125*v194))}else{v3})});
        let v210=(v184*self.scalar_static_f64[88]);
        let v211=((v181+(v123*self.scalar_static_f64[87]))+v210);
        let v212=(v46-v211);
        let v213=(v212/v125);
        let v215=(if (v46<v211){v1}else{v3});
        let v216=(v213).exp();
        let v217=(v1+v216);
        let v218=(v217).ln();
        let v222=(!(v215!=0.0));
        let v224=((-v213)).exp();
        let v225=(v1+v224);
        let v226=(v225).ln();
        let v229=(if v222{(v46+(v125*v226))}else{(if (v215!=0.0){(v211+(v125*v218))}else{v3})});
        let v233=(v210+(v181+(v123*self.scalar_static_f64[89])));
        let v234=(v46-v233);
        let v235=(v234/v125);
        let v237=(if (v46<v233){v1}else{v3});
        let v238=(v235).exp();
        let v239=(v1+v238);
        let v240=(v239).ln();
        let v244=(!(v237!=0.0));
        let v246=((-v235)).exp();
        let v247=(v1+v246);
        let v248=(v247).ln();
        let v251=(if v244{(v46+(v125*v248))}else{(if (v237!=0.0){(v233+(v125*v240))}else{v3})});
        let v254=(v210+(v181+(self.scalar_static_f64[49]*v123)));
        let v255=(v46-v254);
        let v256=(v255/v125);
        let v258=(if (v46<v254){v1}else{v3});
        let v259=(v256).exp();
        let v260=(v1+v259);
        let v261=(v260).ln();
        let v265=(!(v258!=0.0));
        let v267=((-v256)).exp();
        let v268=(v1+v267);
        let v269=(v268).ln();
        let v272=(if v265{(v46+(v125*v269))}else{(if (v258!=0.0){(v254+(v125*v261))}else{v3})});
        let v278=((v181+(v123*self.scalar_static_f64[90]))+(v184*self.scalar_static_f64[91]));
        let v279=(v46-v278);
        let v280=(v279/v125);
        let v282=(if (v46<v278){v1}else{v3});
        let v283=(v280).exp();
        let v284=(v1+v283);
        let v285=(v284).ln();
        let v289=(!(v282!=0.0));
        let v291=((-v280)).exp();
        let v292=(v1+v291);
        let v293=(v292).ln();
        let v296=(if v289{(v46+(v125*v293))}else{(if (v282!=0.0){(v278+(v125*v285))}else{v3})});
        let v297=(v1/v205);
        let v298=(v1/v272);
        let v299=(self.scalar_static_f64[47]*v297);
        let v300=f64::powf(v299,self.scalar_static_f64[18]);
        let v301=(self.scalar_static_f64[49]*v298);
        let v302=f64::powf(v301,self.scalar_static_f64[50]);
        let v304=(v300*self.scalar_static_f64[92]);
        let v307=(self.scalar_static_f64[49]/v272);
        let v310=(self.scalar_static_f64[93]+(self.scalar_static_f64[94]*f64::powf(v307,self.scalar_static_f64[50])));
        let v311=(v1/v310);
        let v313=(v310*self.scalar_static_f64[95]);
        let v314=(self.scalar_static_f64[93]*v311);
        let v341=((v131*self.scalar_static_f64[105])).exp();
        let v342=(self.scalar_static_f64[104]*v341);
        let v353=((v131*self.scalar_static_f64[110])).exp();
        let v354=(self.scalar_static_f64[109]*v353);
        let v362=(if (self.scalar_static_f64[112]!=0.0){(self.scalar_static_f64[113]*(v1+(v130*self.scalar_static_f64[111])))}else{v3});
        let v365=(if (self.scalar_static_f64[112]!=0.0){((v362-v1)/v32)}else{v280});
        let v367=(if (v362<v1){v1}else{v3});
        let v368=((self.scalar_static_f64[112]!=0.0)&&(v367!=0.0));
        let v369=(v365).exp();
        let v370=(v1+v369);
        let v374=(if v368{(v1+(v32*(v370).ln()))}else{v362});
        let v376=((self.scalar_static_f64[112]!=0.0)&&(!(v367!=0.0)));
        let v378=((-v365)).exp();
        let v379=(v1+v378);
        let v384=0.0006931471805599453;
        let v388=(if self.scalar_static_bool[9]{self.scalar_static_f64[113]}else{(if (self.scalar_static_f64[112]!=0.0){((if v376{(v374+(v32*(v379).ln()))}else{v374})-v384)}else{v3})});
        let v396=(if (self.scalar_static_f64[115]!=0.0){(self.scalar_static_f64[116]*(v1+(v130*self.scalar_static_f64[114])))}else{v3});
        let v399=(if (self.scalar_static_f64[115]!=0.0){((v396-v1)/v32)}else{v365});
        let v401=(if (v396<v1){v1}else{v3});
        let v402=((self.scalar_static_f64[115]!=0.0)&&(v401!=0.0));
        let v403=(v399).exp();
        let v404=(v1+v403);
        let v408=(if v402{(v1+(v32*(v404).ln()))}else{v396});
        let v410=((self.scalar_static_f64[115]!=0.0)&&(!(v401!=0.0)));
        let v412=((-v399)).exp();
        let v413=(v1+v412);
        let v421=(if self.scalar_static_bool[11]{self.scalar_static_f64[116]}else{(if (self.scalar_static_f64[115]!=0.0){((if v410{(v408+(v32*(v413).ln()))}else{v408})-v384)}else{v3})});
        let v426=(self.scalar_static_f64[117]*(v1+(v130*self.scalar_static_f64[118])));
        let v427=1e-6;
        let v428=(v426*v426);
        let v430=(if (v426<v3){v1}else{v3});
        let v431=0.5;
        let v432=5e-7;
        let v434=((v427+v428)).sqrt();
        let v435=(v434-v426);
        let v438=(!(v430!=0.0));
        let v441=(if v438{(v431*(v426+v434))}else{(if (v430!=0.0){(v432/v435)}else{v3})});
        let v443=4.0;
        let v448=(v131*self.scalar_static_f64[123]);
        let v450=((v448/v388)).exp();
        let v451=(self.scalar_static_f64[119]*v450);
        let v453=(v129*self.scalar_static_f64[124]);
        let v455=((v453/v388)).exp();
        let v456=(v451*v455);
        let v460=((v131*self.scalar_static_f64[126])).exp();
        let v461=(self.scalar_static_f64[125]*v460);
        let v466=((v131*self.scalar_static_f64[129])).exp();
        let v467=(self.scalar_static_f64[127]*v466);
        let v469=6.0;
        let v546=((v131*self.scalar_static_f64[162])).exp();
        let v547=(self.scalar_static_f64[160]*v546);
        let v551=((v129*self.scalar_static_f64[164])).exp();
        let v552=(v547*v551);
        let v579=(self.scalar_static_f64[46]*v154);
        let v580=-0.5;
        let v581=f64::powf(v579,v580);
        let v582=(v1/v300);
        let v584=(v154*self.scalar_static_f64[174]);
        let v585=(v154*v584);
        let v586=(v581*v585);
        let v588=(self.scalar_static_f64[47]*(v582*v586));
        let v591=(self.scalar_static_f64[46]*(self.scalar_static_f64[46]*(v297*v588)));
        let v602=(self.scalar_static_f64[78]*v177);
        let v603=f64::powf(v602,v580);
        let v604=(v1/v302);
        let v606=(v177*self.scalar_static_f64[176]);
        let v607=(v177*v606);
        let v608=(v603*v607);
        let v610=(self.scalar_static_f64[49]*(v604*v608));
        let v613=(self.scalar_static_f64[78]*(self.scalar_static_f64[78]*(v298*v610)));
        let v625=((v131*self.scalar_static_f64[100])).exp();
        let v627=(v625*self.scalar_static_f64[178]);
        let v628=(v311*v627);
        let v630=(v625*self.scalar_static_f64[179]);
        let v631=(v582*v630);
        let v635=((v131*self.scalar_static_f64[181])).exp();
        let v636=(self.scalar_static_f64[180]*v635);
        let v640=((v129*self.scalar_static_f64[183])).exp();
        let v641=(v636*v640);
        let v646=((v131*self.scalar_static_f64[186])).exp();
        let v647=(self.scalar_static_f64[184]*v646);
        let v651=((v131*self.scalar_static_f64[188])).exp();
        let v652=(self.scalar_static_f64[187]*v651);
        let v654=(v647+v652);
        let v657=((self.scalar_static_f64[189]*v654)/self.scalar_static_f64[190]);
        let v662=((v131*self.scalar_static_f64[193])).exp();
        let v663=(self.scalar_static_f64[191]*v662);
        let v683=(v625*self.scalar_static_f64[195]);
        let v719=ctx.node_voltage(nodes[6]);
        let v720=ctx.node_voltage(nodes[7]);
        let v722=(self.scalar_static_f64[0]*(v719-v720));
        let v723=ctx.node_voltage(nodes[8]);
        let v725=(self.scalar_static_f64[0]*(v719-v723));
        let v726=ctx.node_voltage(nodes[4]);
        let v728=(self.scalar_static_f64[0]*(v719-v726));
        let v729=ctx.node_voltage(nodes[5]);
        let v731=(self.scalar_static_f64[0]*(v729-v726));
        let v733=(self.scalar_static_f64[0]*(v729-v719));
        let v735=(self.scalar_static_f64[0]*(v720-v723));
        let v736=ctx.node_voltage(nodes[2]);
        let v739=ctx.node_voltage(nodes[1]);
        let v741=(self.scalar_static_f64[0]*(v739-v729));
        let v746=(self.scalar_static_f64[0]*(v739-ctx.node_voltage(nodes[0])));
        let v747=ctx.node_voltage(nodes[10]);
        let v749=(self.scalar_static_f64[0]*(v747-v720));
        let v752=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[9])-v747));
        let v755=(((v725+v733)-v735)-v749);
        let v759=((v755+(v741+(-v746)))-v752);
        let v760=(v746+v759);
        let v761=(v127*v725);
        let v764=(if (v761<self.scalar_static_f64[201]){v1}else{v3});
        let v765=(v761).exp();
        let v767=(!(v764!=0.0));
        let v769=(if v767{self.scalar_static_f64[202]}else{v3});
        let v774=(v127*v728);
        let v775=(v774/v388);
        let v777=(if (v775<self.scalar_static_f64[201]){v1}else{v3});
        let v778=(v775).exp();
        let v780=(!(v777!=0.0));
        let v781=(if v780{self.scalar_static_f64[202]}else{v769});
        let v785=(if v780{(v781*(v1+(v775-self.scalar_static_f64[201])))}else{(if (v777!=0.0){v778}else{v3})});
        let v786=(v127*v755);
        let v788=(if (v786<self.scalar_static_f64[201]){v1}else{v3});
        let v789=(v786).exp();
        let v791=(!(v788!=0.0));
        let v792=(if v791{self.scalar_static_f64[202]}else{v781});
        let v796=(if v791{(v792*(v1+(v786-self.scalar_static_f64[201])))}else{(if (v788!=0.0){v789}else{v3})});
        let v797=(v127*v733);
        let v799=(if (v797<self.scalar_static_f64[201]){v1}else{v3});
        let v802=(!(v799!=0.0));
        let v803=(if v802{self.scalar_static_f64[202]}else{v792});
        let v808=(v127*v760);
        let v810=(if (v808<self.scalar_static_f64[201]){v1}else{v3});
        let v811=(v808).exp();
        let v813=(!(v810!=0.0));
        let v814=(if v813{self.scalar_static_f64[202]}else{v803});
        let v818=(if v813{(v814*(v1+(v808-self.scalar_static_f64[201])))}else{(if (v810!=0.0){v811}else{v3})});
        let v819=(v760-v229);
        let v820=(v127*v819);
        let v822=(if (v820<self.scalar_static_f64[201]){v1}else{v3});
        let v823=(v820).exp();
        let v825=(!(v822!=0.0));
        let v826=(if v825{self.scalar_static_f64[202]}else{v814});
        let v831=(v755-v229);
        let v832=(v127*v831);
        let v834=(if (v832<self.scalar_static_f64[201]){v1}else{v3});
        let v835=(v832).exp();
        let v837=(!(v834!=0.0));
        let v838=(if v837{self.scalar_static_f64[202]}else{v826});
        let v843=(v725-v229);
        let v844=(v127*v843);
        let v846=(if (v844<self.scalar_static_f64[201]){v1}else{v3});
        let v847=(v844).exp();
        let v849=(!(v846!=0.0));
        let v850=(if v849{self.scalar_static_f64[202]}else{v838});
        let v854=(if v849{(v850*(v1+(v844-self.scalar_static_f64[201])))}else{(if (v846!=0.0){v847}else{v3})});
        let v855=(v722-v229);
        let v856=(v127*v855);
        let v858=(if (v856<self.scalar_static_f64[201]){v1}else{v3});
        let v859=(v856).exp();
        let v861=(!(v858!=0.0));
        let v862=(if v861{self.scalar_static_f64[202]}else{v850});
        let v866=(if v861{(v862*(v1+(v856-self.scalar_static_f64[201])))}else{(if (v858!=0.0){v859}else{v3})});
        let v869=((v1+(v443*v854))).sqrt();
        let v872=((v1+(v443*v866))).sqrt();
        let v873=(v33*v866);
        let v874=(v1+v872);
        let v875=(v873/v874);
        let v878=(if (v875<self.scalar_static_f64[203]){v1}else{v3});
        let v879=(if (v878!=0.0){self.scalar_static_f64[203]}else{v875});
        let v881=(v1+v869);
        let v882=(v881/v874);
        let v884=((v869-v872)-(v882).ln());
        let v885=(v125*v884);
        let v886=(v735+v885);
        let v887=(v886/v354);
        let v889=(if (v887>v3){v1}else{v3});
        let v890=100.0;
        let v892=(if (v722<v890){v1}else{v3});
        let v893=((v889!=0.0)&&(v892!=0.0));
        let v896=((v889!=0.0)&&(!(v892!=0.0)));
        let v898=(v1+(v722-v890));
        let v902=(v33*v125);
        let v903=(v431*v887);
        let v904=(v354*v903);
        let v906=(v1+(v127*v904));
        let v907=(v906).ln();
        let v911=(if (v889!=0.0){((v229+(v902*v907))-(if v896{(v890+(v898).ln())}else{(if v893{v722}else{v3})}))}else{v3});
        let v912=0.2;
        let v914=(if (v889!=0.0){(v229*v912)}else{v3});
        let v916=(if (v889!=0.0){(v914*v914)}else{v427});
        let v920=(if (v911<v3){v1}else{v3});
        let v921=((v889!=0.0)&&(v920!=0.0));
        let v922=(v431*v916);
        let v924=((v916+(if (v889!=0.0){(v911*v911)}else{v428}))).sqrt();
        let v925=(v924-v911);
        let v929=((v889!=0.0)&&(!(v920!=0.0)));
        let v932=(if v929{(v431*(v911+v924))}else{(if v921{(v922/v925)}else{v3})});
        let v936=(v932+self.scalar_static_f64[206]);
        let v937=(v932*v936);
        let v940=(self.scalar_static_f64[205]*(v932+(v354*self.scalar_static_f64[204])));
        let v942=(if (v889!=0.0){(v937/v940)}else{v3});
        let v944=(if (v889!=0.0){(v887/v942)}else{v3});
        let v948=(if (v889!=0.0){((v944-v1)/self.scalar_static_f64[207])}else{v399});
        let v950=(if (v944<v1){v1}else{v3});
        let v951=((v889!=0.0)&&(v950!=0.0));
        let v952=(v948).exp();
        let v953=(v1+v952);
        let v959=((v889!=0.0)&&(!(v950!=0.0)));
        let v961=((-v948)).exp();
        let v962=(v1+v961);
        let v975=(if (v889!=0.0){((if v959{(v944+(self.scalar_static_f64[207]*(v962).ln()))}else{(if v951{(v1+(self.scalar_static_f64[207]*(v953).ln()))}else{v3})})/self.scalar_static_f64[213])}else{v3});
        let v977=(if (v889!=0.0){(v932/self.scalar_static_f64[206])}else{v3});
        let v978=(v443*v975);
        let v979=(v977*v978);
        let v980=(v1+v977);
        let v983=((v1+(v979*v980))).sqrt();
        let v984=(v1+v983);
        let v985=(v33*v975);
        let v986=(v980*v985);
        let v988=(if (v889!=0.0){(v984/v986)}else{v3});
        let v990=(v879*v988);
        let v991=((v1-v988)+v990);
        let v992=(v1+v990);
        let v994=(if (v889!=0.0){(v991/v992)}else{v3});
        let v995=(v904*v994);
        let v997=(if (v889!=0.0){(v127*v995)}else{v3});
        let v1000=(v1+(v879+v997));
        let v1003=(if (v889!=0.0){((v33*v997)+(v879*v1000))}else{v3});
        let v1006=(if (v889!=0.0){(v431*(v997-v1))}else{v3});
        let v1009=(if (v889!=0.0){(v1003+(v1006*v1006))}else{v3});
        let v1011=(if (v997>=v1){v1}else{v3});
        let v1012=((v889!=0.0)&&(v1011!=0.0));
        let v1013=(v1009).sqrt();
        let v1017=((v889!=0.0)&&(!(v1011!=0.0)));
        let v1018=(v1013-v1006);
        let v1020=(if v1017{(v1003/v1018)}else{(if v1012{(v1006+v1013)}else{v3})});
        let v1024=((v889!=0.0)&&((if (v1020<self.scalar_static_f64[214]){v1}else{v3})!=0.0));
        let v1025=(if v1024{self.scalar_static_f64[214]}else{v1020});
        let v1026=(v1+v1025);
        let v1027=(v1025*v1026);
        let v1029=((v127*v229)).exp();
        let v1035=(if (v889!=0.0){(self.scalar_static_f64[215]*(v887-self.scalar_static_f64[204]))}else{v3});
        let v1037=(self.scalar_static_f64[204]*(v354*self.scalar_static_f64[205]));
        let v1042=(((if (v889!=0.0){(v887*v1037)}else{v3})+(v1035*v1035))).sqrt();
        let v1048=((v889!=0.0)&&(self.scalar_static_f64[217]!=0.0));
        let v1049=(v48*v272);
        let v1052=((v889!=0.0)&&self.scalar_static_bool[20]);
        let v1053=(v33*v887);
        let v1054=(v887+v942);
        let v1056=(v48+(v1053/v1054));
        let v1059=(v887*self.scalar_static_f64[204]);
        let v1060=(v887+self.scalar_static_f64[204]);
        let v1065=(!(v889!=0.0));
        let v1066=(v33*v854);
        let v1069=(if v1065{(if v767{(v769*(v1+(v761-self.scalar_static_f64[201])))}else{(if (v764!=0.0){v765}else{v3})})}else{(if (v889!=0.0){(v1027*v1029)}else{v3})});
        let v1081=(if (((v735).abs()<(v125*1e-5))||((v885).abs()<((v125*1e-40)*(v869+v872)))){v1}else{v3});
        let v1082=(v1065&&(v1081!=0.0));
        let v1083=(v879+(if v1065{(v1066/v881)}else{v1025}));
        let v1085=(if v1082{(v431*v1083)}else{v3});
        let v1086=(v1+v1085);
        let v1090=(v1065&&(!(v1081!=0.0)));
        let v1092=((v725+v885)-v722);
        let v1094=(if v1090{(v885/v1092)}else{(if v1082{(v1085/v1086)}else{v994})});
        let v1096=(if v1065{v1049}else{(if v1052{(v272*v1056)}else{(if v1048{v1049}else{v3})})});
        let v1097=(if v1065{v887}else{(if (v889!=0.0){(v1059/v1060)}else{v3})});
        let v1100=(if v1065{(v1-(v1097/self.scalar_static_f64[204]))}else{(if (v889!=0.0){(self.scalar_static_f64[204]/v1060)}else{v3})});
        let v1104=(v205*self.scalar_static_f64[220]);
        let v1105=(v48*v205);
        let v1106=(v728-v1104);
        let v1107=(v1106/v1105);
        let v1109=(if (v728<v1104){v1}else{v3});
        let v1110=(v1107).exp();
        let v1111=(v1+v1110);
        let v1112=(v1111).ln();
        let v1116=(!(v1109!=0.0));
        let v1118=((-v1107)).exp();
        let v1119=(v1+v1118);
        let v1120=(v1119).ln();
        let v1123=(if v1116{(v1104-(v1105*v1120))}else{(if (v1109!=0.0){(v728-(v1105*v1112))}else{v3})});
        let v1125=(v1-(v297*v1123));
        let v1127=f64::powf(v1125,self.scalar_static_f64[221]);
        let v1128=(v205/self.scalar_static_f64[221]);
        let v1129=(v1-v1127);
        let v1133=((v1128*v1129)+(v178*(v728-v1123)));
        let v1146=(if self.scalar_static_bool[26]{v725}else{(if self.scalar_static_bool[24]{(v722+(if v1065{v735}else{(if (v889!=0.0){(v1035+v1042)}else{v3})}))}else{(if (self.scalar_static_f64[223]!=0.0){v722}else{v3})})});
        let v1147=(v33-v314);
        let v1148=(v1-v314);
        let v1149=(v1147/v1148);
        let v1152=(v1-f64::powf(v1149,self.scalar_static_f64[225]));
        let v1153=(v272*v1152);
        let v1154=(v1146-v1153);
        let v1155=(v1154/v1096);
        let v1157=(if (v1146<v1153){v1}else{v3});
        let v1158=(v1155).exp();
        let v1159=(v1+v1158);
        let v1160=(v1159).ln();
        let v1164=(!(v1157!=0.0));
        let v1166=((-v1155)).exp();
        let v1167=(v1+v1166);
        let v1168=(v1167).ln();
        let v1171=(if v1164{(v1153-(v1096*v1168))}else{(if (v1157!=0.0){(v1146-(v1096*v1160))}else{v3})});
        let v1173=f64::powf(v1100,self.scalar_static_f64[226]);
        let v1175=(v272/self.scalar_static_f64[227]);
        let v1177=(v1-(v1171/v272));
        let v1178=f64::powf(v1177,self.scalar_static_f64[227]);
        let v1180=(v1-(v1173*v1178));
        let v1182=(v1149*v1173);
        let v1183=(v1146-v1171);
        let v1185=((v1175*v1180)+(v1182*v1183));
        let v1188=((v1148*v1185)+(v314*v722));
        let v1189=(v443*v456);
        let v1190=(v1189/v461);
        let v1191=(v785*v1190);
        let v1193=((v1+v1191)).sqrt();
        let v1194=(v1+v1193);
        let v1195=(v1191/v1194);
        let v1196=(v1/v421);
        let v1197=f64::powf(v1069,v1196);
        let v1198=(v1190*v1197);
        let v1200=((v1+v1198)).sqrt();
        let v1201=(v1+v1200);
        let v1202=(v1198/v1201);
        let v1206=(v1+(v1133/v631));
        let v1207=(v1188/v628);
        let v1208=(v1206+v1207);
        let v1211=(v683*v1206);
        let v1214=(-v1188);
        let v1215=(v1214/v628);
        let v1216=(v683*v1215);
        let v1219=((if self.scalar_static_bool[28]{(v127*v1211)}else{v3})).exp();
        let v1220=((if self.scalar_static_bool[28]{(v127*v1216)}else{v3})).exp();
        let v1221=(v1219-v1220);
        let v1223=((v127*v683)).exp();
        let v1224=(v1223-v1);
        let v1226=(if self.scalar_static_bool[28]{(v1221/v1224)}else{(if (self.scalar_static_f64[228]!=0.0){v1208}else{v3})});
        let v1227=0.010000000000000002;
        let v1228=(v1226*v1226);
        let v1230=(if (v1226<v3){v1}else{v3});
        let v1231=0.005000000000000001;
        let v1233=((v1227+v1228)).sqrt();
        let v1234=(v1233-v1226);
        let v1237=(!(v1230!=0.0));
        let v1240=(if v1237{(v431*(v1226+v1233))}else{(if (v1230!=0.0){(v1231/v1234)}else{v3})});
        let v1243=(v1+(v431*(v1195+v1202)));
        let v1244=(v1240*v1243);
        let v1246=(v456*self.scalar_static_f64[229]);
        let v1247=(v1197*v1246);
        let v1248=(v456*v785);
        let v1249=(v1248-v1247);
        let v1250=(v1249/v1244);
        let v1251=0.0001;
        let v1252=(v728/v1251);
        let v1253=(v728<v3);
        let v1254=(if v1253{v1}else{v3});
        let v1255=(v1252).exp();
        let v1256=(v1+v1255);
        let v1260=(!(v1254!=0.0));
        let v1262=((-v1252)).exp();
        let v1263=(v1+v1262);
        let v1267=(if v1260{(v728+(v1251*(v1263).ln()))}else{(if (v1254!=0.0){(v1251*(v1256).ln())}else{v3})});
        let v1269=(v1267/self.scalar_static_f64[230]);
        let v1271=(if (v1269<self.scalar_static_f64[201]){v1}else{v3});
        let v1274=(!(v1271!=0.0));
        let v1275=(if v1274{self.scalar_static_f64[202]}else{v862});
        let v1284=((v728-self.scalar_static_f64[231])/v32);
        let v1306=(v774/self.scalar_static_f64[144]);
        let v1308=(if (v1306<self.scalar_static_f64[201]){v1}else{v3});
        let v1309=(v1306).exp();
        let v1311=(!(v1308!=0.0));
        let v1312=(if v1311{self.scalar_static_f64[202]}else{v1275});
        let v1316=(if v1311{(v1312*(v1+(v1306-self.scalar_static_f64[201])))}else{(if (v1308!=0.0){v1309}else{v1267})});
        let v1317=(v728-v296);
        let v1318=(v127*v1317);
        let v1320=(if (v1318<self.scalar_static_f64[201]){v1}else{v3});
        let v1325=((self.scalar_static_f64[150]!=0.0)&&(!(v1320!=0.0)));
        let v1326=(if v1325{self.scalar_static_f64[202]}else{v1312});
        let v1333=((v1250/v456)-1000.0);
        let v1334=40.0;
        let v1336=(if (v1333<v1334){v1}else{v3});
        let v1341=((self.scalar_static_f64[150]!=0.0)&&(!(v1336!=0.0)));
        let v1343=(if v1341{2.3538526683702e17}else{v1326});
        let v1384=(v127*v731);
        let v1385=(v1384/self.scalar_static_f64[148]);
        let v1387=(if (v1385<self.scalar_static_f64[201]){v1}else{v3});
        let v1388=(v1385).exp();
        let v1390=(!(v1387!=0.0));
        let v1391=(if v1390{self.scalar_static_f64[202]}else{v1343});
        let v1395=(if v1390{(v1391*(v1+(v1385-self.scalar_static_f64[201])))}else{(if (v1387!=0.0){v1388}else{v1316})});
        let v1396=(v731-v296);
        let v1397=(v127*v1396);
        let v1399=(if (v1397<self.scalar_static_f64[201]){v1}else{v3});
        let v1404=((self.scalar_static_f64[150]!=0.0)&&(!(v1399!=0.0)));
        let v1405=(if v1404{self.scalar_static_f64[202]}else{v1391});
        let v1422=(v774/self.scalar_static_f64[131]);
        let v1424=(if (v1422<self.scalar_static_f64[201]){v1}else{v3});
        let v1425=(v1422).exp();
        let v1427=(!(v1424!=0.0));
        let v1428=(if v1427{self.scalar_static_f64[202]}else{v1405});
        let v1432=(if v1427{(v1428*(v1+(v1422-self.scalar_static_f64[201])))}else{(if (v1424!=0.0){v1425}else{v1395})});
        let v1435=(v1384/self.scalar_static_f64[166]);
        let v1437=(if (v1435<self.scalar_static_f64[201]){v1}else{v3});
        let v1438=(v1435).exp();
        let v1440=(!(v1437!=0.0));
        let v1441=(if v1440{self.scalar_static_f64[202]}else{v1428});
        let v1445=(if v1440{(v1441*(v1+(v1435-self.scalar_static_f64[201])))}else{(if (v1437!=0.0){v1438}else{v1432})});
        let v1448=(v786/self.scalar_static_f64[137]);
        let v1450=(if (v1448<self.scalar_static_f64[201]){v1}else{v3});
        let v1451=(v1448).exp();
        let v1453=(!(v1450!=0.0));
        let v1454=(if v1453{self.scalar_static_f64[202]}else{v1441});
        let v1458=(if v1453{(v1454*(v1+(v1448-self.scalar_static_f64[201])))}else{(if (v1450!=0.0){v1451}else{v1445})});
        let v1461=(v1384/self.scalar_static_f64[170]);
        let v1463=(if (v1461<self.scalar_static_f64[201]){v1}else{v3});
        let v1464=(v1461).exp();
        let v1466=(!(v1463!=0.0));
        let v1467=(if v1466{self.scalar_static_f64[202]}else{v1454});
        let v1471=(if v1466{(v1467*(v1+(v1461-self.scalar_static_f64[201])))}else{(if (v1463!=0.0){v1464}else{v1458})});
        let v1478=(if (v1253&&self.scalar_static_bool[36]){v1}else{v3});
        let v1479=(v33*v1127);
        let v1481=(v1-(self.scalar_static_f64[20]/v1479));
        let v1482=(v591*v1481);
        let v1484=(if (v1482<self.scalar_static_f64[201]){v1}else{v3});
        let v1489=((v1478!=0.0)&&(!(v1484!=0.0)));
        let v1490=(if v1489{self.scalar_static_f64[202]}else{v1467});
        let v1496=(if (v1478!=0.0){(v297*v728)}else{v625});
        let v1498=1e-30;
        let v1500=(((v1496*v1496)+v1498)).sqrt();
        let v1503=f64::powf(v1500,self.scalar_static_f64[236]);
        let v1511=(v469*v1496);
        let v1512=(v1496*v1511);
        let v1513=(v1496+self.scalar_static_f64[239]);
        let v1515=((self.scalar_static_f64[18]*(self.scalar_static_f64[238]-((v178*v1496)*self.scalar_static_f64[239])))-(v1512*v1513));
        let v1517=0.16666666666666666;
        let v1519=(if (v1478!=0.0){((v1503*v1515)*v1517)}else{v3});
        let v1520=(self.scalar_static_f64[20]*v728);
        let v1521=(v591*v1520);
        let v1522=(v154*v1519);
        let v1524=(if (v1478!=0.0){(v1521/v1522)}else{v1496});
        let v1525=-0.001;
        let v1527=(if (v1524<v1525){v1}else{v3});
        let v1529=(if (v1524<self.scalar_static_f64[201]){v1}else{v3});
        let v1530=((v1478!=0.0)&&(v1527!=0.0));
        let v1535=(v1530&&(!(v1529!=0.0)));
        let v1536=(if v1535{self.scalar_static_f64[202]}else{v1490});
        let v1574=(if (self.scalar_static_bool[39]&&(v722<v3)){v1}else{v3});
        let v1575=(v298*v722);
        let v1576=(v1-v1575);
        let v1578=(if (v1574!=0.0){f64::powf(v1576,self.scalar_static_f64[227])}else{v3});
        let v1579=(v33*v1578);
        let v1581=(v1-(self.scalar_static_f64[52]/v1579));
        let v1582=(v613*v1581);
        let v1584=(if (v1582<self.scalar_static_f64[201]){v1}else{v3});
        let v1589=((v1574!=0.0)&&(!(v1584!=0.0)));
        let v1590=(if v1589{self.scalar_static_f64[202]}else{v1536});
        let v1595=(if (v1574!=0.0){v1575}else{v603});
        let v1598=((v1498+(v1595*v1595))).sqrt();
        let v1600=f64::powf(v1598,self.scalar_static_f64[240]);
        let v1608=(v469*v1595);
        let v1609=(v1595*v1608);
        let v1610=(v1595+self.scalar_static_f64[243]);
        let v1612=((self.scalar_static_f64[50]*(self.scalar_static_f64[242]-((v178*v1595)*self.scalar_static_f64[243])))-(v1609*v1610));
        let v1615=(if (v1574!=0.0){(v1517*(v1600*v1612))}else{v3});
        let v1616=(self.scalar_static_f64[52]*v722);
        let v1617=(v613*v1616);
        let v1618=(v177*v1615);
        let v1620=(if (v1574!=0.0){(v1617/v1618)}else{v1595});
        let v1622=(if (v1620<v1525){v1}else{v3});
        let v1624=(if (v1620<self.scalar_static_f64[201]){v1}else{v3});
        let v1625=((v1574!=0.0)&&(v1622!=0.0));
        let v1630=(v1625&&(!(v1624!=0.0)));
        let v1631=(if v1630{self.scalar_static_f64[202]}else{v1590});
        let v1662=(v796*v1190);
        let v1663=(v443*(if v837{(v838*(v1+(v832-self.scalar_static_f64[201])))}else{(if (v834!=0.0){v835}else{v3})}));
        let v1664=(v1662-v1190);
        let v1666=((v1+v1662)).sqrt();
        let v1667=(v1+v1666);
        let v1668=(v1664/v1667);
        let v1670=((v1+v1663)).sqrt();
        let v1671=(v1+v1670);
        let v1672=(v1663/v1671);
        let v1673=(v33*v552);
        let v1676=(v443*v552);
        let v1677=(v1676/v467);
        let v1691=(v552*self.scalar_static_f64[246]);
        let v1692=(v818-v1);
        let v1693=(v1691*v1692);
        let v1696=((v1+(v818*v1677))).sqrt();
        let v1697=(v1+v1696);
        let v1699=(if (self.scalar_static_f64[245]!=0.0){(v1693/v1697)}else{v3});
        let v1703=(self.scalar_static_f64[6]*v552);
        let v1705=(if self.scalar_static_bool[44]{(v342*v1703)}else{v3});
        let v1706=(v127*v1705);
        let v1708=(v33-(v1706).ln());
        let v1712=(if self.scalar_static_bool[44]{(v760-(if self.scalar_static_bool[44]{(v125*v1708)}else{v3}))}else{v3});
        let v1716=(if self.scalar_static_bool[44]{(v1712*v1712)}else{v1228});
        let v1718=(if (v1712<v3){v1}else{v3});
        let v1719=(self.scalar_static_bool[44]&&(v1718!=0.0));
        let v1722=((self.scalar_static_f64[248]+v1716)).sqrt();
        let v1723=(v1722-v1712);
        let v1727=(self.scalar_static_bool[44]&&(!(v1718!=0.0)));
        let v1730=(if v1727{(v431*(v1712+v1722))}else{(if v1719{(self.scalar_static_f64[249]/v1723)}else{v3})});
        let v1733=(v1730+(v1705+(v342*v1699)));
        let v1738=(if self.scalar_static_bool[46]{v1}else{(if self.scalar_static_bool[44]{(v1730/v1733)}else{v1})});
        let v1801=(if (v1208<v3){v1}else{v3});
        let v1803=((v1227+(v1208*v1208))).sqrt();
        let v1804=(v1803-v1208);
        let v1807=(!(v1801!=0.0));
        let v1810=(if v1807{(v431*(v1208+v1803))}else{(if (v1801!=0.0){(v1231/v1804)}else{v3})});
        let v1822=(if (v1250>v3){v1}else{v3});
        let v1828=(if (v722<self.scalar_static_f64[271]){v1}else{v3});
        let v1831=((-v1250)/self.scalar_static_f64[272]);
        let v1833=(if (v1831<self.scalar_static_f64[201]){v1}else{v3});
        let v1835=((v1828!=0.0)&&((v1822!=0.0)&&(self.scalar_static_f64[270]!=0.0)));
        let v1836=((v1833!=0.0)&&v1835);
        let v1837=(v1831).exp();
        let v1840=(v1835&&(!(v1833!=0.0)));
        let v1841=(if v1840{self.scalar_static_f64[202]}else{v1631});
        let v1845=(if v1840{(v1841*(v1+(v1831-self.scalar_static_f64[201])))}else{(if v1836{v1837}else{v3})});
        let v1846=(self.scalar_static_f64[271]-v722);
        let v1848=(if v1835{(v1845*v1846)}else{v3});
        let v1849=(-v441);
        let v1851=f64::powf(v1848,self.scalar_static_f64[273]);
        let v1852=(v1849*v1851);
        let v1854=(if (v1852<self.scalar_static_f64[201]){v1}else{v3});
        let v1859=(v1835&&(!(v1854!=0.0)));
        let v1860=(if v1859{self.scalar_static_f64[202]}else{v1841});
        let v1875=((v1822!=0.0)&&self.scalar_static_bool[51]);
        let v1982=((v1828!=0.0)&&((self.scalar_static_f64[288]!=0.0)&&(v1875&&self.scalar_static_bool[55])));
        let v1983=f64::powf(v1846,self.scalar_static_f64[273]);
        let v1985=(v1250+self.scalar_static_f64[289]);
        let v1987=(v1-(v1250/v1985));
        let v1989=f64::powf(v1987,self.scalar_static_f64[290]);
        let v1991=(if v1982{(v1983*v1989)}else{v3});
        let v1992=((self.scalar_static_f64[282]!=0.0)&&v1982);
        let v1994=(self.scalar_static_bool[53]&&v1982);
        let v1998=(if v1994{((v1250-self.scalar_static_f64[291])/self.scalar_static_f64[289])}else{v3});
        let v2002=(if v1994{((v1998-v1)/self.scalar_static_f64[292])}else{v1284});
        let v2004=(if (v1998<v1){v1}else{v3});
        let v2005=(v1994&&(v2004!=0.0));
        let v2006=(v2002).exp();
        let v2007=(v1+v2006);
        let v2013=(v1994&&(!(v2004!=0.0)));
        let v2015=((-v2002)).exp();
        let v2016=(v1+v2015);
        let v2020=(if v2013{(v1998+(self.scalar_static_f64[292]*(v2016).ln()))}else{(if v2005{(v1+(self.scalar_static_f64[292]*(v2007).ln()))}else{v3})});
        let v2022=f64::powf(v2020,self.scalar_static_f64[293]);
        let v2024=(if v1994{(v1991*v2022)}else{(if v1992{v1991}else{v3})});
        let v2025=(v1849*v2024);
        let v2027=(if (v2025<self.scalar_static_f64[201]){v1}else{v3});
        let v2032=(v1982&&(!(v2027!=0.0)));
        let v2033=(if v2032{self.scalar_static_f64[202]}else{v1860});
        let v2092=(v1069).ln();
        let v2146=(v304*self.scalar_static_f64[297]);
        let v2148=(v731-v1104);
        let v2149=(v2148/v1105);
        let v2151=(if (v731<v1104){v1}else{v3});
        let v2152=(v2149).exp();
        let v2153=(v1+v2152);
        let v2154=(v2153).ln();
        let v2158=(!(v2151!=0.0));
        let v2160=((-v2149)).exp();
        let v2161=(v1+v2160);
        let v2162=(v2161).ln();
        let v2165=(if v2158{(v1104-(v1105*v2162))}else{(if (v2151!=0.0){(v731-(v1105*v2154))}else{v3})});
        let v2166=(v304*self.scalar_static_f64[296]);
        let v2168=(v1-(v297*v2165));
        let v2170=(v1-f64::powf(v2168,self.scalar_static_f64[221]));
        let v2174=((v1128*v2170)+(v178*(v731-v2165)));
        let v2177=(v313*self.scalar_static_f64[298]);
        let v2179=(v461*v647);
        let v2180=(v431*v2179);
        let v2181=(v1195*v2180);
        let v2182=(v1810*v2181);
        let v2183=(v1202*v2180);
        let v2184=(v1810*v2183);
        let v2185=(v755-v1153);
        let v2186=(v2185/v1049);
        let v2188=(if (v755<v1153){v1}else{v3});
        let v2189=(v2186).exp();
        let v2190=(v1+v2189);
        let v2191=(v2190).ln();
        let v2195=(!(v2188!=0.0));
        let v2197=((-v2186)).exp();
        let v2198=(v1+v2197);
        let v2199=(v2198).ln();
        let v2202=(if v2195{(v1153-(v1049*v2199))}else{(if (v2188!=0.0){(v755-(v1049*v2191))}else{v3})});
        let v2204=(v1-(v2202/v272));
        let v2206=(v1-f64::powf(v2204,self.scalar_static_f64[227]));
        let v2208=(v755-v2202);
        let v2210=((v1175*v2206)+(v1149*v2208));
        let v2213=((v1148*v2210)+(v314*v755));
        let v2218=(v760-v1153);
        let v2219=(v2218/v1049);
        let v2221=(if (v760<v1153){v1}else{v3});
        let v2222=(v2219).exp();
        let v2223=(v1+v2222);
        let v2224=(v2223).ln();
        let v2228=(!(v2221!=0.0));
        let v2230=((-v2219)).exp();
        let v2231=(v1+v2230);
        let v2232=(v2231).ln();
        let v2235=(if v2228{(v1153-(v1049*v2232))}else{(if (v2221!=0.0){(v760-(v1049*v2224))}else{v3})});
        let v2237=(v1-(v2235/v272));
        let v2239=(v1-f64::powf(v2237,self.scalar_static_f64[227]));
        let v2241=(v760-v2235);
        let v2243=((v1175*v2239)+(v1149*v2241));
        let v2246=((v1148*v2243)+(v314*v760));
        let v2250=(v461*v641);
        let v2251=(v456/v461);
        let v2254=f64::powf(v2251,self.scalar_static_f64[301]);
        let v2255=(v2250*v2254);
        let v2256=(v125*self.scalar_static_f64[300]);
        let v2257=(v728/v2256);
        let v2259=(if (v2257<self.scalar_static_f64[201]){v1}else{v3});
        let v2260=(v2257).exp();
        let v2262=(!(v2259!=0.0));
        let v2263=(if v2262{self.scalar_static_f64[202]}else{v2033});
        let v2267=(if v2262{(v2263*(v1+(v2257-self.scalar_static_f64[201])))}else{(if (v2259!=0.0){v2260}else{v1471})});
        let v2268=(v2255*v2267);
        let v2269=(v443*v652);
        let v2270=(v125*v2269);
        let v2271=(v2270/v354);
        let v2272=(v431*v2271);
        let v2273=(v1094*v2272);
        let v2274=(v33+v1083);
        let v2279=(v431*v657);
        let v2282=((v1668*v2179)+(v1672*v2271));
        let v2283=(v2279*v2282);
        let v2288=((v755-v251)/self.scalar_static_f64[304]);
        let v2289=(v127*v2288);
        let v2291=(if (v2289<self.scalar_static_f64[201]){v1}else{v3});
        let v2293=((v2291!=0.0)&&self.scalar_static_bool[60]);
        let v2294=(v2289).exp();
        let v2297=(self.scalar_static_bool[60]&&(!(v2291!=0.0)));
        let v2298=(if v2297{self.scalar_static_f64[202]}else{v2263});
        let v2303=(v663*v1673);
        let v2304=(v796*v2303);
        let v2307=((v1+(v443*(if v2297{(v2298*(v1+(v2289-self.scalar_static_f64[201])))}else{(if v2293{v2294}else{v3})})))).sqrt();
        let v2308=(v1+v2307);
        let v2310=(if self.scalar_static_bool[60]{(v2304/v2308)}else{(if (self.scalar_static_f64[303]!=0.0){(v2283/v654)}else{v3})});
        let v2319=(if self.scalar_static_bool[64]{(v818*v1190)}else{v3});
        let v2320=(v2319-v1190);
        let v2322=((v1+v2319)).sqrt();
        let v2323=(v1+v2322);
        let v2325=(if self.scalar_static_bool[64]{(v2320/v2323)}else{v3});
        let v2327=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*(v1+(v820-self.scalar_static_f64[201])))}else{(if (v822!=0.0){v823}else{v3})}))}else{v3});
        let v2329=((v1+v2327)).sqrt();
        let v2330=(v1+v2329);
        let v2332=(if self.scalar_static_bool[64]{(v2327/v2330)}else{v3});
        let v2334=(v657*self.scalar_static_f64[306]);
        let v2337=((v2179*v2325)+(v2271*v2332));
        let v2338=(v2334*v2337);
        let v2341=(v760-v251);
        let v2342=(v127*v2341);
        let v2344=(if (v2342<self.scalar_static_f64[201]){v1}else{v3});
        let v2346=((v2344!=0.0)&&self.scalar_static_bool[65]);
        let v2347=(v2342).exp();
        let v2350=(self.scalar_static_bool[65]&&(!(v2344!=0.0)));
        let v2351=(if v2350{self.scalar_static_f64[202]}else{v2298});
        let v2356=(v663*v1691);
        let v2357=(v818*v2356);
        let v2360=((v1+(v443*(if v2350{(v2351*(v1+(v2342-self.scalar_static_f64[201])))}else{(if v2346{v2347}else{v3})})))).sqrt();
        let v2361=(v1+v2360);
        let v2363=(if self.scalar_static_bool[65]{(v2357/v2361)}else{(if self.scalar_static_bool[64]{(v2338/v654)}else{v3})});
        let v2372=(if (self.scalar_static_f64[308]!=0.0){(f64::powf(v1125,self.scalar_static_f64[309])-v178)}else{v3});
        let v2373=(if (self.scalar_static_f64[308]!=0.0){v1107}else{v3});
        let v2375=(if (v2373<v3){v1}else{v3});
        let v2376=((self.scalar_static_f64[308]!=0.0)&&(v2375!=0.0));
        let v2377=(v2373).exp();
        let v2378=(v1+v2377);
        let v2382=((self.scalar_static_f64[308]!=0.0)&&(!(v2375!=0.0)));
        let v2384=((-v2373)).exp();
        let v2385=(v1+v2384);
        let v2387=(if v2382{(v2384/v2385)}else{(if v2376{(v1/v2378)}else{v3})});
        let v2390=(if (self.scalar_static_f64[308]!=0.0){(v178+(v2372*v2387))}else{v3});
        let v2393=(v127*v1191);
        let v2394=(v2393/v388);
        let v2395=(v431/v1193);
        let v2397=(if (self.scalar_static_f64[308]!=0.0){(v2394*v2395)}else{v3});
        let v2398=(v1810*v2180);
        let v2403=(v733*v912);
        let v2405=((if (self.scalar_static_f64[308]!=0.0){(v2268/v2256)}else{v3})+((if (self.scalar_static_f64[308]!=0.0){(v2146*v2390)}else{v3})+(if (self.scalar_static_f64[308]!=0.0){(v2397*v2398)}else{v3})));
        let v2414=(if (self.scalar_static_f64[308]!=0.0){(v2182+(v2268*self.scalar_static_f64[310]))}else{v3});
        let v2423=(if self.scalar_static_bool[67]{v2182}else{(if (self.scalar_static_f64[308]!=0.0){(v2414*self.scalar_static_f64[313])}else{v3})});
        let v2424=(if self.scalar_static_bool[67]{v2184}else{(if (self.scalar_static_f64[308]!=0.0){(v2184+(v2414*self.scalar_static_f64[312]))}else{v3})});
        let v2427=(v105*self.scalar_static_f64[314]);
        let v2465=(v1247+v1248);
        let v2466=(v2465/v1244);
        let v2476=(if (v2466>v3){v1}else{v3});
        let v2477=(v2423+v2424);
        let v2480=(!(v2476!=0.0));
        let v2481=(v647*v1810);
        let v2483=(if v2480{(v1244*v2481)}else{(if (v2476!=0.0){(v2477/v2466)}else{v3})});
        let v2498=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(v2483*self.scalar_static_f64[326])}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v2483)}else{v3})})});
        let v2534=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v2268}else{(if (self.scalar_static_f64[308]!=0.0){(v2268*self.scalar_static_f64[311])}else{v3})})+((v1133*v2146)+v2423)));
        let v2537=(self.scalar_static_f64[0]*(v2166*v2174));
        let v2540=(self.scalar_static_f64[0]*((v2273*v2274)+((v1188*v2177)+v2424)));
        let v2543=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*v2405)}else{v3}));
        let v2547=((self.scalar_static_f64[0]*(v739-v736))*self.scalar_static_f64[329]);
        let v2551=(v746*self.scalar_static_f64[330]);
        let v2559=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[299]*(v313*v2246)))+(if (self.scalar_static_f64[305]!=0.0){(v1738*v2363)}else{v3})));
        let v2565=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*((v313*v2213)*self.scalar_static_f64[299]))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v2310)}else{v2310})));
        let v2576=ctx.node_voltage(nodes[11]);
        let v2582=(if (v107!=0.0){(-(-1.0/v108))}else{v1});
        let v2585=(if v116{(v2582/v118)}else{(if (v114!=0.0){v2582}else{v3})});
        let v2586=(v2585/self.scalar_static_f64[9]);
        let v2587=(v124*v2585);
        let v2589=(v125*v125);
        let v2590=((-v2587)/v2589);
        let v2591=(v2586/v123);
        let v2637=((v180*v2591)+(v131*(v179*v2587)));
        let v2640=(-v2586);
        let v2642=((v2637+(self.scalar_static_f64[47]*v2586))+(self.scalar_static_f64[86]*v2640));
        let v2647=(((v125*(-v2642))-(v188*v2587))/v2589);
        let v2661=(if v198{((v202*v2587)+(v125*((v200*(-v2647))/v201)))}else{(if (v191!=0.0){(v2642+((v194*v2587)+(v125*((v192*v2647)/v193))))}else{v3})});
        let v2664=(self.scalar_static_f64[88]*v2640);
        let v2665=((v2637+(self.scalar_static_f64[87]*v2586))+v2664);
        let v2670=(((v125*(-v2665))-(v212*v2587))/v2589);
        let v2684=(if v222{((v226*v2587)+(v125*((v224*(-v2670))/v225)))}else{(if (v215!=0.0){(v2665+((v218*v2587)+(v125*((v216*v2670)/v217))))}else{v3})});
        let v2687=(v2664+(v2637+(self.scalar_static_f64[89]*v2586)));
        let v2692=(((v125*(-v2687))-(v234*v2587))/v2589);
        let v2709=(v2664+(v2637+(self.scalar_static_f64[49]*v2586)));
        let v2714=(((v125*(-v2709))-(v255*v2587))/v2589);
        let v2728=(if v265{((v269*v2587)+(v125*((v267*(-v2714))/v268)))}else{(if (v258!=0.0){(v2709+((v261*v2587)+(v125*((v259*v2714)/v260))))}else{v3})});
        let v2732=((v2637+(self.scalar_static_f64[90]*v2586))+(self.scalar_static_f64[91]*v2640));
        let v2737=(((v125*(-v2732))-(v279*v2587))/v2589);
        let v2754=((-v2661)/(v205*v205));
        let v2756=(v272*v272);
        let v2761=((self.scalar_static_f64[47]*v2754)*(self.scalar_static_f64[18]*f64::powf(v299,self.scalar_static_f64[239])));
        let v2766=(self.scalar_static_f64[92]*v2761);
        let v2773=(self.scalar_static_f64[94]*(((-(self.scalar_static_f64[49]*v2728))/v2756)*(self.scalar_static_f64[50]*f64::powf(v307,self.scalar_static_f64[243]))));
        let v2776=((-v2773)/(v310*v310));
        let v2777=(self.scalar_static_f64[95]*v2773);
        let v2778=(self.scalar_static_f64[93]*v2776);
        let v2792=(self.scalar_static_f64[104]*(v341*(self.scalar_static_f64[105]*v2591)));
        let v2799=(self.scalar_static_f64[109]*(v353*(self.scalar_static_f64[110]*v2591)));
        let v2802=(if (self.scalar_static_f64[112]!=0.0){(self.scalar_static_f64[113]*(self.scalar_static_f64[111]*v2585))}else{v3});
        let v2804=(if (self.scalar_static_f64[112]!=0.0){(v2802/v32)}else{v2737});
        let v2808=(if v368{(v32*((v369*v2804)/v370))}else{v2802});
        let v2816=(if self.scalar_static_bool[9]{v3}else{(if (self.scalar_static_f64[112]!=0.0){(if v376{(v2808+(v32*((v378*(-v2804))/v379)))}else{v2808})}else{v3})});
        let v2819=(if (self.scalar_static_f64[115]!=0.0){(self.scalar_static_f64[116]*(self.scalar_static_f64[114]*v2585))}else{v3});
        let v2821=(if (self.scalar_static_f64[115]!=0.0){(v2819/v32)}else{v2804});
        let v2825=(if v402{(v32*((v403*v2821)/v404))}else{v2819});
        let v2835=(self.scalar_static_f64[117]*(self.scalar_static_f64[118]*v2585));
        let v2836=(v426*v2835);
        let v2837=(v2836+v2836);
        let v2853=(v388*v388);
        let v2865=((v455*(self.scalar_static_f64[119]*(v450*(((v388*(self.scalar_static_f64[123]*v2591))-(v448*v2816))/v2853))))+(v451*(v455*(((v388*(self.scalar_static_f64[124]*v2590))-(v453*v2816))/v2853))));
        let v2868=(self.scalar_static_f64[125]*(v460*(self.scalar_static_f64[126]*v2591)));
        let v2929=((v551*(self.scalar_static_f64[160]*(v546*(self.scalar_static_f64[162]*v2591))))+(v547*(v551*(self.scalar_static_f64[164]*v2590))));
        let v2961=((-v2761)/(v300*v300));
        let v3036=(v625*(self.scalar_static_f64[100]*v2591));
        let v3040=((v627*v2776)+(v311*(self.scalar_static_f64[178]*v3036)));
        let v3055=(self.scalar_static_f64[184]*(v646*(self.scalar_static_f64[186]*v2591)));
        let v3058=(self.scalar_static_f64[187]*(v651*(self.scalar_static_f64[188]*v2591)));
        let v3059=(v3055+v3058);
        let v3061=((self.scalar_static_f64[189]*v3059)/self.scalar_static_f64[190]);
        let v3064=(self.scalar_static_f64[191]*(v662*(self.scalar_static_f64[193]*v2591)));
        let v3074=(self.scalar_static_f64[195]*v3036);
        let v3097=(v725*v2590);
        let v3098=(self.scalar_static_f64[0]*v127);
        let v3099=(v127*self.scalar_static_f64[331]);
        let v3112=(v728*v2590);
        let v3116=(((v388*v3112)-(v774*v2816))/v2853);
        let v3117=(v3099/v388);
        let v3118=(v3098/v388);
        let v3128=(if v780{(v781*v3116)}else{(if (v777!=0.0){(v778*v3116)}else{v3})});
        let v3129=(if v780{(v781*v3117)}else{(if (v777!=0.0){(v778*v3117)}else{v3})});
        let v3130=(if v780{(v781*v3118)}else{(if (v777!=0.0){(v778*v3118)}else{v3})});
        let v3131=(v755*v2590);
        let v3132=(v127*self.scalar_static_f64[332]);
        let v3133=(v127*self.scalar_static_f64[333]);
        let v3149=(if v791{(v792*v3131)}else{(if (v788!=0.0){(v789*v3131)}else{v3})});
        let v3150=(if v791{(v792*v3098)}else{(if (v788!=0.0){(v789*v3098)}else{v3})});
        let v3151=(if v791{(v792*v3132)}else{(if (v788!=0.0){(v789*v3132)}else{v3})});
        let v3152=(if v791{(v792*v3133)}else{(if (v788!=0.0){(v789*v3133)}else{v3})});
        let v3153=(if v791{(v792*v3099)}else{(if (v788!=0.0){(v789*v3099)}else{v3})});
        let v3167=(v127*self.scalar_static_f64[334]);
        let v3168=(v760*v2590);
        let v3184=(if v813{(v814*v3132)}else{(if (v810!=0.0){(v811*v3132)}else{v3})});
        let v3185=(if v813{(v814*v3167)}else{(if (v810!=0.0){(v811*v3167)}else{v3})});
        let v3186=(if v813{(v814*v3168)}else{(if (v810!=0.0){(v811*v3168)}else{v3})});
        let v3187=(if v813{(v814*v3133)}else{(if (v810!=0.0){(v811*v3133)}else{v3})});
        let v3188=(if v813{(v814*v3099)}else{(if (v810!=0.0){(v811*v3099)}else{v3})});
        let v3191=(v127*(-v2684));
        let v3192=((v819*v2590)+v3191);
        let v3214=(v3191+(v831*v2590));
        let v3236=(v3191+(v843*v2590));
        let v3246=(if v849{(v850*v3236)}else{(if (v846!=0.0){(v847*v3236)}else{v3})});
        let v3247=(if v849{(v850*v3098)}else{(if (v846!=0.0){(v847*v3098)}else{v3})});
        let v3248=(if v849{(v850*v3099)}else{(if (v846!=0.0){(v847*v3099)}else{v3})});
        let v3250=(v3191+(v855*v2590));
        let v3260=(if v861{(v862*v3250)}else{(if (v858!=0.0){(v859*v3250)}else{v3})});
        let v3261=(if v861{(v862*v3098)}else{(if (v858!=0.0){(v859*v3098)}else{v3})});
        let v3262=(if v861{(v862*v3099)}else{(if (v858!=0.0){(v859*v3099)}else{v3})});
        let v3266=(v33*v869);
        let v3267=((v443*v3246)/v3266);
        let v3268=((v443*v3247)/v3266);
        let v3269=((v443*v3248)/v3266);
        let v3273=(v33*v872);
        let v3274=((v443*v3260)/v3273);
        let v3275=((v443*v3261)/v3273);
        let v3276=((v443*v3262)/v3273);
        let v3283=(v874*v874);
        let v3293=(if (v878!=0.0){v3}else{(((v874*(v33*v3260))-(v873*v3274))/v3283)});
        let v3294=(if (v878!=0.0){v3}else{(((v874*(v33*v3261))-(v873*v3275))/v3283)});
        let v3295=(if (v878!=0.0){v3}else{(((v874*(v33*v3262))-(v873*v3276))/v3283)});
        let v3321=((v884*v2587)+(v125*((v3267-v3274)-((((v874*v3267)-(v881*v3274))/v3283)/v882))));
        let v3322=(v125*((v3268-v3275)-((((v874*v3268)-(v881*v3275))/v3283)/v882)));
        let v3323=(v125*((-v3276)-(((-(v881*v3276))/v3283)/v882)));
        let v3324=(v125*(v3269-((v3269/v874)/v882)));
        let v3326=(self.scalar_static_f64[331]+v3324);
        let v3330=(v354*v354);
        let v3331=(((v354*v3321)-(v886*v2799))/v3330);
        let v3332=(v3322/v354);
        let v3333=((self.scalar_static_f64[0]+v3323)/v354);
        let v3334=(v3326/v354);
        let v3341=(v33*v2587);
        let v3348=((v903*v2799)+(v354*(v431*v3331)));
        let v3349=(v354*(v431*v3332));
        let v3350=(v354*(v431*v3333));
        let v3351=(v354*(v431*v3334));
        let v3371=(if (v889!=0.0){(v2684+((v907*v3341)+(v902*(((v904*v2590)+(v127*v3348))/v906))))}else{v3});
        let v3372=(if (v889!=0.0){((v902*((v127*v3349)/v906))-(if v896{(self.scalar_static_f64[0]/v898)}else{(if v893{self.scalar_static_f64[0]}else{v3})}))}else{v3});
        let v3373=(if (v889!=0.0){((v902*((v127*v3350)/v906))-(if v896{(self.scalar_static_f64[331]/v898)}else{(if v893{self.scalar_static_f64[331]}else{v3})}))}else{v3});
        let v3374=(if (v889!=0.0){(v902*((v127*v3351)/v906))}else{v3});
        let v3377=(v914*(if (v889!=0.0){(v912*v2684)}else{v3}));
        let v3379=(if (v889!=0.0){(v3377+v3377)}else{v3});
        let v3380=(v911*v3371);
        let v3382=(v911*v3372);
        let v3384=(v911*v3373);
        let v3386=(v911*v3374);
        let v3394=(v33*v924);
        let v3395=((v3379+(if (v889!=0.0){(v3380+v3380)}else{v2837}))/v3394);
        let v3396=((if (v889!=0.0){(v3382+v3382)}else{v3})/v3394);
        let v3397=((if (v889!=0.0){(v3384+v3384)}else{v3})/v3394);
        let v3398=((if (v889!=0.0){(v3386+v3386)}else{v3})/v3394);
        let v3406=(v925*v925);
        let v3429=(if v929{(v431*(v3371+v3395))}else{(if v921{(((v925*(v431*v3379))-(v922*(v3395-v3371)))/v3406)}else{v3})});
        let v3430=(if v929{(v431*(v3372+v3396))}else{(if v921{((-(v922*(v3396-v3372)))/v3406)}else{v3})});
        let v3431=(if v929{(v431*(v3373+v3397))}else{(if v921{((-(v922*(v3397-v3373)))/v3406)}else{v3})});
        let v3432=(if v929{(v431*(v3374+v3398))}else{(if v921{((-(v922*(v3398-v3374)))/v3406)}else{v3})});
        let v3454=(v940*v940);
        let v3468=(if (v889!=0.0){(((v940*((v936*v3429)+(v932*v3429)))-(v937*(self.scalar_static_f64[205]*(v3429+(self.scalar_static_f64[204]*v2799)))))/v3454)}else{v3});
        let v3469=(if (v889!=0.0){(((v940*((v936*v3430)+(v932*v3430)))-(v937*(self.scalar_static_f64[205]*v3430)))/v3454)}else{v3});
        let v3470=(if (v889!=0.0){(((v940*((v936*v3431)+(v932*v3431)))-(v937*(self.scalar_static_f64[205]*v3431)))/v3454)}else{v3});
        let v3471=(if (v889!=0.0){(((v940*((v936*v3432)+(v932*v3432)))-(v937*(self.scalar_static_f64[205]*v3432)))/v3454)}else{v3});
        let v3475=(v942*v942);
        let v3489=(if (v889!=0.0){(((v942*v3331)-(v887*v3468))/v3475)}else{v3});
        let v3490=(if (v889!=0.0){(((v942*v3332)-(v887*v3469))/v3475)}else{v3});
        let v3491=(if (v889!=0.0){(((v942*v3333)-(v887*v3470))/v3475)}else{v3});
        let v3492=(if (v889!=0.0){(((v942*v3334)-(v887*v3471))/v3475)}else{v3});
        let v3497=(if (v889!=0.0){(v3489/self.scalar_static_f64[207])}else{v2821});
        let v3498=(if (v889!=0.0){(v3490/self.scalar_static_f64[207])}else{v3});
        let v3499=(if (v889!=0.0){(v3491/self.scalar_static_f64[207])}else{v3});
        let v3500=(if (v889!=0.0){(v3492/self.scalar_static_f64[207])}else{v3});
        let v3545=(if (v889!=0.0){((if v959{(v3489+(self.scalar_static_f64[207]*((v961*(-v3497))/v962)))}else{(if v951{(self.scalar_static_f64[207]*((v952*v3497)/v953))}else{v3})})/self.scalar_static_f64[213])}else{v3});
        let v3546=(if (v889!=0.0){((if v959{(v3490+(self.scalar_static_f64[207]*((v961*(-v3498))/v962)))}else{(if v951{(self.scalar_static_f64[207]*((v952*v3498)/v953))}else{v3})})/self.scalar_static_f64[213])}else{v3});
        let v3547=(if (v889!=0.0){((if v959{(v3491+(self.scalar_static_f64[207]*((v961*(-v3499))/v962)))}else{(if v951{(self.scalar_static_f64[207]*((v952*v3499)/v953))}else{v3})})/self.scalar_static_f64[213])}else{v3});
        let v3548=(if (v889!=0.0){((if v959{(v3492+(self.scalar_static_f64[207]*((v961*(-v3500))/v962)))}else{(if v951{(self.scalar_static_f64[207]*((v952*v3500)/v953))}else{v3})})/self.scalar_static_f64[213])}else{v3});
        let v3553=(if (v889!=0.0){(v3429/self.scalar_static_f64[206])}else{v3});
        let v3554=(if (v889!=0.0){(v3430/self.scalar_static_f64[206])}else{v3});
        let v3555=(if (v889!=0.0){(v3431/self.scalar_static_f64[206])}else{v3});
        let v3556=(if (v889!=0.0){(v3432/self.scalar_static_f64[206])}else{v3});
        let v3585=(v33*v983);
        let v3609=(v986*v986);
        let v3623=(if (v889!=0.0){(((v986*(((v980*((v978*v3553)+(v977*(v443*v3545))))+(v979*v3553))/v3585))-(v984*((v985*v3553)+(v980*(v33*v3545)))))/v3609)}else{v3});
        let v3624=(if (v889!=0.0){(((v986*(((v980*((v978*v3554)+(v977*(v443*v3546))))+(v979*v3554))/v3585))-(v984*((v985*v3554)+(v980*(v33*v3546)))))/v3609)}else{v3});
        let v3625=(if (v889!=0.0){(((v986*(((v980*((v978*v3555)+(v977*(v443*v3547))))+(v979*v3555))/v3585))-(v984*((v985*v3555)+(v980*(v33*v3547)))))/v3609)}else{v3});
        let v3626=(if (v889!=0.0){(((v986*(((v980*((v978*v3556)+(v977*(v443*v3548))))+(v979*v3556))/v3585))-(v984*((v985*v3556)+(v980*(v33*v3548)))))/v3609)}else{v3});
        let v3633=((v988*v3293)+(v879*v3623));
        let v3636=((v988*v3294)+(v879*v3624));
        let v3639=((v988*v3295)+(v879*v3625));
        let v3640=(v879*v3626);
        let v3648=(v992*v992);
        let v3662=(if (v889!=0.0){(((v992*((-v3623)+v3633))-(v991*v3633))/v3648)}else{v3});
        let v3663=(if (v889!=0.0){(((v992*((-v3624)+v3636))-(v991*v3636))/v3648)}else{v3});
        let v3664=(if (v889!=0.0){(((v992*((-v3625)+v3639))-(v991*v3639))/v3648)}else{v3});
        let v3665=(if (v889!=0.0){(((v992*((-v3626)+v3640))-(v991*v3640))/v3648)}else{v3});
        let v3684=(if (v889!=0.0){((v995*v2590)+(v127*((v994*v3348)+(v904*v3662))))}else{v3});
        let v3685=(if (v889!=0.0){(v127*((v994*v3349)+(v904*v3663)))}else{v3});
        let v3686=(if (v889!=0.0){(v127*((v994*v3350)+(v904*v3664)))}else{v3});
        let v3687=(if (v889!=0.0){(v127*((v994*v3351)+(v904*v3665)))}else{v3});
        let v3709=(if (v889!=0.0){((v33*v3684)+((v1000*v3293)+(v879*(v3293+v3684))))}else{v3});
        let v3710=(if (v889!=0.0){((v33*v3685)+((v1000*v3294)+(v879*(v3294+v3685))))}else{v3});
        let v3711=(if (v889!=0.0){((v33*v3686)+((v1000*v3295)+(v879*(v3295+v3686))))}else{v3});
        let v3712=(if (v889!=0.0){((v33*v3687)+(v879*v3687))}else{v3});
        let v3717=(if (v889!=0.0){(v431*v3684)}else{v3});
        let v3718=(if (v889!=0.0){(v431*v3685)}else{v3});
        let v3719=(if (v889!=0.0){(v431*v3686)}else{v3});
        let v3720=(if (v889!=0.0){(v431*v3687)}else{v3});
        let v3721=(v1006*v3717);
        let v3723=(v1006*v3718);
        let v3725=(v1006*v3719);
        let v3727=(v1006*v3720);
        let v3733=(if (v889!=0.0){(v3709+(v3721+v3721))}else{v3});
        let v3734=(if (v889!=0.0){(v3710+(v3723+v3723))}else{v3});
        let v3735=(if (v889!=0.0){(v3711+(v3725+v3725))}else{v3});
        let v3736=(if (v889!=0.0){(v3712+(v3727+v3727))}else{v3});
        let v3737=(v33*v1013);
        let v3738=(v3733/v3737);
        let v3739=(v3734/v3737);
        let v3740=(v3735/v3737);
        let v3741=(v3736/v3737);
        let v3757=(v1018*v1018);
        let v3775=(if v1024{v3}else{(if v1017{(((v1018*v3709)-(v1003*(v3738-v3717)))/v3757)}else{(if v1012{(v3717+v3738)}else{v3})})});
        let v3776=(if v1024{v3}else{(if v1017{(((v1018*v3710)-(v1003*(v3739-v3718)))/v3757)}else{(if v1012{(v3718+v3739)}else{v3})})});
        let v3777=(if v1024{v3}else{(if v1017{(((v1018*v3711)-(v1003*(v3740-v3719)))/v3757)}else{(if v1012{(v3719+v3740)}else{v3})})});
        let v3778=(if v1024{v3}else{(if v1017{(((v1018*v3712)-(v1003*(v3741-v3720)))/v3757)}else{(if v1012{(v3720+v3741)}else{v3})})});
        let v3809=(if (v889!=0.0){(self.scalar_static_f64[215]*v3331)}else{v3});
        let v3810=(if (v889!=0.0){(self.scalar_static_f64[215]*v3332)}else{v3});
        let v3811=(if (v889!=0.0){(self.scalar_static_f64[215]*v3333)}else{v3});
        let v3812=(if (v889!=0.0){(self.scalar_static_f64[215]*v3334)}else{v3});
        let v3825=(v1035*v3809);
        let v3827=(v1035*v3810);
        let v3829=(v1035*v3811);
        let v3831=(v1035*v3812);
        let v3837=(v33*v1042);
        let v3850=(v48*v2728);
        let v3863=(v1054*v1054);
        let v3887=(self.scalar_static_f64[204]*v3331);
        let v3888=(self.scalar_static_f64[204]*v3332);
        let v3889=(self.scalar_static_f64[204]*v3333);
        let v3890=(self.scalar_static_f64[204]*v3334);
        let v3894=(v1060*v1060);
        let v3930=(v881*v881);
        let v3943=(if v1065{(((v881*(v33*v3248))-(v1066*v3269))/v3930)}else{v3778});
        let v3944=(if v1065{(if v767{(v769*v3097)}else{(if (v764!=0.0){(v765*v3097)}else{v3})})}else{(if (v889!=0.0){((v1029*((v1026*v3775)+(v1025*v3775)))+(v1027*(v1029*((v229*v2590)+(v127*v2684)))))}else{v3})});
        let v3945=(if v1065{(if v767{(v769*v3098)}else{(if (v764!=0.0){(v765*v3098)}else{v3})})}else{(if (v889!=0.0){(v1029*((v1026*v3776)+(v1025*v3776)))}else{v3})});
        let v3946=(if v1065{v3}else{(if (v889!=0.0){(v1029*((v1026*v3777)+(v1025*v3777)))}else{v3})});
        let v3947=(if v1065{(if v767{(v769*v3099)}else{(if (v764!=0.0){(v765*v3099)}else{v3})})}else{(if (v889!=0.0){(v1029*((v1026*v3778)+(v1025*v3778)))}else{v3})});
        let v3948=(v3293+(if v1065{(((v881*(v33*v3246))-(v1066*v3267))/v3930)}else{v3775}));
        let v3949=(v3294+(if v1065{(((v881*(v33*v3247))-(v1066*v3268))/v3930)}else{v3776}));
        let v3950=(v3295+(if v1065{v3}else{v3777}));
        let v3955=(if v1082{(v431*v3948)}else{v3});
        let v3956=(if v1082{(v431*v3949)}else{v3});
        let v3957=(if v1082{(v431*v3950)}else{v3});
        let v3958=(if v1082{(v431*v3943)}else{v3});
        let v3962=(v1086*v1086);
        let v3986=(v1092*v1092);
        let v4000=(if v1090{(((v1092*v3321)-(v885*v3321))/v3986)}else{(if v1082{(((v1086*v3955)-(v1085*v3955))/v3962)}else{v3662})});
        let v4001=(if v1090{(((v1092*v3322)-(v885*((self.scalar_static_f64[0]+v3322)-self.scalar_static_f64[0])))/v3986)}else{(if v1082{(((v1086*v3956)-(v1085*v3956))/v3962)}else{v3663})});
        let v4002=(if v1090{(((v1092*v3323)-(v885*(v3323-self.scalar_static_f64[331])))/v3986)}else{(if v1082{(((v1086*v3957)-(v1085*v3957))/v3962)}else{v3664})});
        let v4003=(if v1090{(((v1092*v3324)-(v885*v3326))/v3986)}else{(if v1082{(((v1086*v3958)-(v1085*v3958))/v3962)}else{v3665})});
        let v4008=(if v1065{v3850}else{(if v1052{((v1056*v2728)+(v272*(((v1054*(v33*v3331))-(v1053*(v3331+v3468)))/v3863)))}else{(if v1048{v3850}else{v3})})});
        let v4009=(if v1065{v3}else{(if v1052{(v272*(((v1054*(v33*v3332))-(v1053*(v3332+v3469)))/v3863))}else{v3})});
        let v4010=(if v1065{v3}else{(if v1052{(v272*(((v1054*(v33*v3333))-(v1053*(v3333+v3470)))/v3863))}else{v3})});
        let v4011=(if v1065{v3}else{(if v1052{(v272*(((v1054*(v33*v3334))-(v1053*(v3334+v3471)))/v3863))}else{v3})});
        let v4012=(if v1065{v3331}else{(if (v889!=0.0){(((v1060*v3887)-(v1059*v3331))/v3894)}else{v3})});
        let v4013=(if v1065{v3332}else{(if (v889!=0.0){(((v1060*v3888)-(v1059*v3332))/v3894)}else{v3})});
        let v4014=(if v1065{v3333}else{(if (v889!=0.0){(((v1060*v3889)-(v1059*v3333))/v3894)}else{v3})});
        let v4015=(if v1065{v3334}else{(if (v889!=0.0){(((v1060*v3890)-(v1059*v3334))/v3894)}else{v3})});
        let v4024=(if v1065{(-(v4012/self.scalar_static_f64[204]))}else{(if (v889!=0.0){((-v3887)/v3894)}else{v3})});
        let v4025=(if v1065{(-(v4013/self.scalar_static_f64[204]))}else{(if (v889!=0.0){((-v3888)/v3894)}else{v3})});
        let v4026=(if v1065{(-(v4014/self.scalar_static_f64[204]))}else{(if (v889!=0.0){((-v3889)/v3894)}else{v3})});
        let v4027=(if v1065{(-(v4015/self.scalar_static_f64[204]))}else{(if (v889!=0.0){((-v3890)/v3894)}else{v3})});
        let v4028=(self.scalar_static_f64[220]*v2661);
        let v4029=(v48*v2661);
        let v4031=(v1105*(-v4028));
        let v4034=(v1105*v1105);
        let v4035=((v4031-(v1106*v4029))/v4034);
        let v4036=(self.scalar_static_f64[331]/v1105);
        let v4037=(self.scalar_static_f64[0]/v1105);
        let v4056=(-v4036);
        let v4057=(-v4037);
        let v4072=(if v1116{(v4028-((v1120*v4029)+(v1105*((v1118*(-v4035))/v1119))))}else{(if (v1109!=0.0){(-((v1112*v4029)+(v1105*((v1110*v4035)/v1111))))}else{v3})});
        let v4073=(if v1116{(-(v1105*((v1118*v4056)/v1119)))}else{(if (v1109!=0.0){(self.scalar_static_f64[331]-(v1105*((v1110*v4036)/v1111)))}else{v3})});
        let v4074=(if v1116{(-(v1105*((v1118*v4057)/v1119)))}else{(if (v1109!=0.0){(self.scalar_static_f64[0]-(v1105*((v1110*v4037)/v1111)))}else{v3})});
        let v4080=(-((v1123*v2754)+(v297*v4072)));
        let v4081=(-(v297*v4073));
        let v4082=(-(v297*v4074));
        let v4085=(self.scalar_static_f64[221]*f64::powf(v1125,self.scalar_static_f64[335]));
        let v4086=(v4080*v4085);
        let v4087=(v4081*v4085);
        let v4088=(v4082*v4085);
        let v4089=(v2661/self.scalar_static_f64[221]);
        let v4104=(((v1129*v4089)+(v1128*(-v4086)))+(v178*(-v4072)));
        let v4105=((v1128*(-v4087))+(v178*(self.scalar_static_f64[331]-v4073)));
        let v4106=((v1128*(-v4088))+(v178*(self.scalar_static_f64[0]-v4074)));
        let v4115=(if self.scalar_static_bool[26]{v3}else{(if self.scalar_static_bool[24]{(if v1065{v3}else{(if (v889!=0.0){(v3809+(((if (v889!=0.0){((v1037*v3331)+(v887*(self.scalar_static_f64[204]*(self.scalar_static_f64[205]*v2799))))}else{v3})+(v3825+v3825))/v3837))}else{v3})})}else{v3})});
        let v4116=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v1065{v3}else{(if (v889!=0.0){(v3810+(((if (v889!=0.0){(v1037*v3332)}else{v3})+(v3827+v3827))/v3837))}else{v3})}))}else{self.scalar_static_f64[336]})});
        let v4117=(if self.scalar_static_bool[26]{v3}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[331]+(if v1065{self.scalar_static_f64[0]}else{(if (v889!=0.0){(v3811+(((if (v889!=0.0){(v1037*v3333)}else{v3})+(v3829+v3829))/v3837))}else{v3})}))}else{self.scalar_static_f64[337]})});
        let v4118=(if self.scalar_static_bool[26]{self.scalar_static_f64[331]}else{(if self.scalar_static_bool[24]{(if v1065{self.scalar_static_f64[331]}else{(if (v889!=0.0){(v3812+(((if (v889!=0.0){(v1037*v3334)}else{v3})+(v3831+v3831))/v3837))}else{v3})})}else{v3})});
        let v4119=(-v2778);
        let v4124=(((v1148*v4119)-(v1147*v4119))/(v1148*v1148));
        let v4132=((v1152*v2728)+(v272*(-(v4124*(self.scalar_static_f64[225]*f64::powf(v1149,self.scalar_static_f64[338]))))));
        let v4137=(v1096*v1096);
        let v4138=(((v1096*(v4115-v4132))-(v1154*v4008))/v4137);
        let v4142=(((v1096*v4116)-(v1154*v4009))/v4137);
        let v4146=(((v1096*v4117)-(v1154*v4010))/v4137);
        let v4150=(((v1096*v4118)-(v1154*v4011))/v4137);
        let v4207=(if v1164{(v4132-((v1168*v4008)+(v1096*((v1166*(-v4138))/v1167))))}else{(if (v1157!=0.0){(v4115-((v1160*v4008)+(v1096*((v1158*v4138)/v1159))))}else{v3})});
        let v4208=(if v1164{(-((v1168*v4009)+(v1096*((v1166*(-v4142))/v1167))))}else{(if (v1157!=0.0){(v4116-((v1160*v4009)+(v1096*((v1158*v4142)/v1159))))}else{v3})});
        let v4209=(if v1164{(-((v1168*v4010)+(v1096*((v1166*(-v4146))/v1167))))}else{(if (v1157!=0.0){(v4117-((v1160*v4010)+(v1096*((v1158*v4146)/v1159))))}else{v3})});
        let v4210=(if v1164{(-((v1168*v4011)+(v1096*((v1166*(-v4150))/v1167))))}else{(if (v1157!=0.0){(v4118-((v1160*v4011)+(v1096*((v1158*v4150)/v1159))))}else{v3})});
        let v4213=(self.scalar_static_f64[226]*f64::powf(v1100,self.scalar_static_f64[339]));
        let v4214=(v4024*v4213);
        let v4215=(v4025*v4213);
        let v4216=(v4026*v4213);
        let v4217=(v4027*v4213);
        let v4218=(v2728/self.scalar_static_f64[227]);
        let v4232=(self.scalar_static_f64[227]*f64::powf(v1177,self.scalar_static_f64[340]));
        let v4290=(v1148*((v1175*(-((v1178*v4217)+(v1173*((-(v4210/v272))*v4232)))))+((v1183*(v1149*v4217))+(v1182*(v4118-v4210)))));
        let v4292=(self.scalar_static_f64[0]*v314);
        let v4293=(v314*self.scalar_static_f64[331]);
        let v4294=(((v1185*v4119)+(v1148*(((v1180*v4218)+(v1175*(-((v1178*v4214)+(v1173*((-(((v272*v4207)-(v1171*v2728))/v2756))*v4232))))))+((v1183*((v1173*v4124)+(v1149*v4214)))+(v1182*(v4115-v4207))))))+(v722*v2778));
        let v4295=((v1148*((v1175*(-((v1178*v4215)+(v1173*((-(v4208/v272))*v4232)))))+((v1183*(v1149*v4215))+(v1182*(v4116-v4208)))))+v4292);
        let v4296=((v1148*((v1175*(-((v1178*v4216)+(v1173*((-(v4209/v272))*v4232)))))+((v1183*(v1149*v4216))+(v1182*(v4117-v4209)))))+v4293);
        let v4301=(v461*v461);
        let v4302=(((v461*(v443*v2865))-(v1189*v2868))/v4301);
        let v4305=((v1190*v3128)+(v785*v4302));
        let v4306=(v1190*v3129);
        let v4307=(v1190*v3130);
        let v4308=(v33*v1193);
        let v4309=(v4305/v4308);
        let v4310=(v4306/v4308);
        let v4311=(v4307/v4308);
        let v4315=(v1194*v1194);
        let v4316=(((v1194*v4305)-(v1191*v4309))/v4315);
        let v4320=(((v1194*v4306)-(v1191*v4310))/v4315);
        let v4324=(((v1194*v4307)-(v1191*v4311))/v4315);
        let v4330=(v1196*f64::powf(v1069,(v1196-v1)));
        let v4334=((v3944*v4330)+(((-(if self.scalar_static_bool[11]{v3}else{(if (self.scalar_static_f64[115]!=0.0){(if v410{(v2825+(v32*((v412*(-v2821))/v413)))}else{v2825})}else{v3})}))/(v421*v421))*(v1197*v2092)));
        let v4335=(v3945*v4330);
        let v4336=(v3946*v4330);
        let v4337=(v3947*v4330);
        let v4340=((v1197*v4302)+(v1190*v4334));
        let v4341=(v1190*v4335);
        let v4342=(v1190*v4336);
        let v4343=(v1190*v4337);
        let v4344=(v33*v1200);
        let v4352=(v1201*v1201);
        let v4353=(((v1201*v4340)-(v1198*(v4340/v4344)))/v4352);
        let v4357=(((v1201*v4341)-(v1198*(v4341/v4344)))/v4352);
        let v4361=(((v1201*v4342)-(v1198*(v4342/v4344)))/v4352);
        let v4365=(((v1201*v4343)-(v1198*(v4343/v4344)))/v4352);
        let v4370=(((v631*v4104)-(v1133*((v630*v2961)+(v582*(self.scalar_static_f64[179]*v3036)))))/(v631*v631));
        let v4371=(v4105/v631);
        let v4372=(v4106/v631);
        let v4376=(v628*v628);
        let v4377=(((v628*v4294)-(v1188*v3040))/v4376);
        let v4378=(v4295/v628);
        let v4379=(v4296/v628);
        let v4380=(v4290/v628);
        let v4381=(v4370+v4377);
        let v4382=(v4372+v4378);
        let v4452=(if self.scalar_static_bool[28]{(((v1224*((v1219*(if self.scalar_static_bool[28]{((v1211*v2590)+(v127*((v1206*v3074)+(v683*v4370))))}else{v3}))-(v1220*(if self.scalar_static_bool[28]{((v1216*v2590)+(v127*((v1215*v3074)+(v683*(((v628*(-v4294))-(v1214*v3040))/v4376)))))}else{v3}))))-(v1221*(v1223*((v683*v2590)+(v127*v3074)))))/(v1224*v1224))}else{(if (self.scalar_static_f64[228]!=0.0){v4381}else{v3})});
        let v4453=(if self.scalar_static_bool[28]{((v1219*(if self.scalar_static_bool[28]{(v127*(v683*v4371))}else{v3}))/v1224)}else{(if (self.scalar_static_f64[228]!=0.0){v4371}else{v3})});
        let v4454=(if self.scalar_static_bool[28]{(((v1219*(if self.scalar_static_bool[28]{(v127*(v683*v4372))}else{v3}))-(v1220*(if self.scalar_static_bool[28]{(v127*(v683*((-v4295)/v628)))}else{v3})))/v1224)}else{(if (self.scalar_static_f64[228]!=0.0){v4382}else{v3})});
        let v4455=(if self.scalar_static_bool[28]{((-(v1220*(if self.scalar_static_bool[28]{(v127*(v683*((-v4296)/v628)))}else{v3})))/v1224)}else{(if (self.scalar_static_f64[228]!=0.0){v4379}else{v3})});
        let v4456=(if self.scalar_static_bool[28]{((-(v1220*(if self.scalar_static_bool[28]{(v127*(v683*((-v4290)/v628)))}else{v3})))/v1224)}else{(if (self.scalar_static_f64[228]!=0.0){v4380}else{v3})});
        let v4457=(v1226*v4452);
        let v4458=(v4457+v4457);
        let v4459=(v1226*v4453);
        let v4460=(v4459+v4459);
        let v4461=(v1226*v4454);
        let v4462=(v4461+v4461);
        let v4463=(v1226*v4455);
        let v4464=(v4463+v4463);
        let v4465=(v1226*v4456);
        let v4466=(v4465+v4465);
        let v4467=(v33*v1233);
        let v4468=(v4458/v4467);
        let v4469=(v4460/v4467);
        let v4470=(v4462/v4467);
        let v4471=(v4464/v4467);
        let v4472=(v4466/v4467);
        let v4480=(v1234*v1234);
        let v4516=(v431*(v4316+v4353));
        let v4517=(v431*v4320);
        let v4518=(v431*(v4324+v4357));
        let v4519=(v431*v4361);
        let v4520=(v431*v4365);
        let v4523=((v1243*(if v1237{(v431*(v4452+v4468))}else{(if (v1230!=0.0){((-(v1231*(v4468-v4452)))/v4480)}else{v3})}))+(v1240*v4516));
        let v4526=((v1243*(if v1237{(v431*(v4453+v4469))}else{(if (v1230!=0.0){((-(v1231*(v4469-v4453)))/v4480)}else{v3})}))+(v1240*v4517));
        let v4529=((v1243*(if v1237{(v431*(v4454+v4470))}else{(if (v1230!=0.0){((-(v1231*(v4470-v4454)))/v4480)}else{v3})}))+(v1240*v4518));
        let v4532=((v1243*(if v1237{(v431*(v4455+v4471))}else{(if (v1230!=0.0){((-(v1231*(v4471-v4455)))/v4480)}else{v3})}))+(v1240*v4519));
        let v4535=((v1243*(if v1237{(v431*(v4456+v4472))}else{(if (v1230!=0.0){((-(v1231*(v4472-v4456)))/v4480)}else{v3})}))+(v1240*v4520));
        let v4539=((v1246*v4334)+(v1197*(self.scalar_static_f64[229]*v2865)));
        let v4540=(v1246*v4335);
        let v4541=(v1246*v4336);
        let v4542=(v1246*v4337);
        let v4545=((v785*v2865)+(v456*v3128));
        let v4547=(v456*v3130);
        let v4555=(v1244*v1244);
        let v4557=(v1244*(v456*v3129));
        let v4593=(if v1260{(self.scalar_static_f64[331]+(v1251*((v1262*self.scalar_static_f64[343])/v1263)))}else{(if (v1254!=0.0){(v1251*((v1255*self.scalar_static_f64[341])/v1256))}else{v3})});
        let v4594=(if v1260{(self.scalar_static_f64[0]+(v1251*((v1262*self.scalar_static_f64[344])/v1263)))}else{(if (v1254!=0.0){(v1251*((v1255*self.scalar_static_f64[342])/v1256))}else{v3})});
        let v4646=(v3112/self.scalar_static_f64[144]);
        let v4647=(v3099/self.scalar_static_f64[144]);
        let v4648=(v3098/self.scalar_static_f64[144]);
        let v4658=(if v1311{(v1312*v4646)}else{(if (v1308!=0.0){(v1309*v4646)}else{v3})});
        let v4659=(if v1311{(v1312*v4647)}else{(if (v1308!=0.0){(v1309*v4647)}else{v4593})});
        let v4660=(if v1311{(v1312*v4648)}else{(if (v1308!=0.0){(v1309*v4648)}else{v4594})});
        let v4842=(v731*v2590);
        let v4843=(v4842/self.scalar_static_f64[148]);
        let v4844=(v3099/self.scalar_static_f64[148]);
        let v4845=(v3098/self.scalar_static_f64[148]);
        let v4856=(if v1390{(v1391*v4843)}else{(if (v1387!=0.0){(v1388*v4843)}else{v4658})});
        let v4857=(if v1390{(v1391*v4844)}else{(if (v1387!=0.0){(v1388*v4844)}else{v4659})});
        let v4858=(if v1390{(v1391*v4845)}else{(if (v1387!=0.0){(v1388*v4845)}else{v3})});
        let v4859=(if v1390{v3}else{(if (v1387!=0.0){v3}else{v4660})});
        let v4927=(v3112/self.scalar_static_f64[131]);
        let v4928=(v3099/self.scalar_static_f64[131]);
        let v4929=(v3098/self.scalar_static_f64[131]);
        let v4940=(if v1427{(v1428*v4927)}else{(if (v1424!=0.0){(v1425*v4927)}else{v4856})});
        let v4941=(if v1427{(v1428*v4928)}else{(if (v1424!=0.0){(v1425*v4928)}else{v4857})});
        let v4942=(if v1427{v3}else{(if (v1424!=0.0){v3}else{v4858})});
        let v4943=(if v1427{(v1428*v4929)}else{(if (v1424!=0.0){(v1425*v4929)}else{v4859})});
        let v4950=(v4842/self.scalar_static_f64[166]);
        let v4951=(v3099/self.scalar_static_f64[166]);
        let v4952=(v3098/self.scalar_static_f64[166]);
        let v4963=(if v1440{(v1441*v4950)}else{(if (v1437!=0.0){(v1438*v4950)}else{v4940})});
        let v4964=(if v1440{(v1441*v4951)}else{(if (v1437!=0.0){(v1438*v4951)}else{v4941})});
        let v4965=(if v1440{(v1441*v4952)}else{(if (v1437!=0.0){(v1438*v4952)}else{v4942})});
        let v4966=(if v1440{v3}else{(if (v1437!=0.0){v3}else{v4943})});
        let v4973=(v3131/self.scalar_static_f64[137]);
        let v4974=(v3098/self.scalar_static_f64[137]);
        let v4975=(v3132/self.scalar_static_f64[137]);
        let v4976=(v3133/self.scalar_static_f64[137]);
        let v4977=(v3099/self.scalar_static_f64[137]);
        let v4994=(if v1453{(v1454*v4973)}else{(if (v1450!=0.0){(v1451*v4973)}else{v4963})});
        let v4995=(if v1453{v3}else{(if (v1450!=0.0){v3}else{v4964})});
        let v4996=(if v1453{(v1454*v4974)}else{(if (v1450!=0.0){(v1451*v4974)}else{v4965})});
        let v4997=(if v1453{(v1454*v4975)}else{(if (v1450!=0.0){(v1451*v4975)}else{v4966})});
        let v4998=(if v1453{(v1454*v4976)}else{(if (v1450!=0.0){(v1451*v4976)}else{v3})});
        let v4999=(if v1453{(v1454*v4977)}else{(if (v1450!=0.0){(v1451*v4977)}else{v3})});
        let v5008=(v4842/self.scalar_static_f64[170]);
        let v5009=(v3099/self.scalar_static_f64[170]);
        let v5010=(v3098/self.scalar_static_f64[170]);
        let v5023=(if v1466{(v1467*v5008)}else{(if (v1463!=0.0){(v1464*v5008)}else{v4994})});
        let v5024=(if v1466{(v1467*v5009)}else{(if (v1463!=0.0){(v1464*v5009)}else{v4995})});
        let v5025=(if v1466{(v1467*v5010)}else{(if (v1463!=0.0){(v1464*v5010)}else{v4996})});
        let v5026=(if v1466{v3}else{(if (v1463!=0.0){v3}else{v4997})});
        let v5027=(if v1466{v3}else{(if (v1463!=0.0){v3}else{v4998})});
        let v5028=(if v1466{v3}else{(if (v1463!=0.0){v3}else{v4999})});
        let v5536=((v1190*v3149)+(v796*v4302));
        let v5537=(v1190*v3150);
        let v5538=(v1190*v3151);
        let v5539=(v1190*v3152);
        let v5540=(v1190*v3153);
        let v5541=(v443*(if v837{(v838*v3214)}else{(if (v834!=0.0){(v835*v3214)}else{v3})}));
        let v5542=(v443*(if v837{(v838*v3098)}else{(if (v834!=0.0){(v835*v3098)}else{v3})}));
        let v5543=(v443*(if v837{(v838*v3132)}else{(if (v834!=0.0){(v835*v3132)}else{v3})}));
        let v5544=(v443*(if v837{(v838*v3133)}else{(if (v834!=0.0){(v835*v3133)}else{v3})}));
        let v5545=(v443*(if v837{(v838*v3099)}else{(if (v834!=0.0){(v835*v3099)}else{v3})}));
        let v5547=(v33*v1666);
        let v5556=(v1667*v1667);
        let v5574=(v33*v1670);
        let v5583=(v1671*v1671);
        let v5601=(v33*v2929);
        let v5614=(((v467*(v443*v2929))-(v1676*(self.scalar_static_f64[127]*(v466*(self.scalar_static_f64[129]*v2591)))))/(v467*v467));
        let v5659=(self.scalar_static_f64[246]*v2929);
        let v5674=(v33*v1696);
        let v5683=(v1697*v1697);
        let v5701=(if (self.scalar_static_f64[245]!=0.0){(((v1697*(v1691*v3184))-(v1693*((v1677*v3184)/v5674)))/v5683)}else{v3});
        let v5702=(if (self.scalar_static_f64[245]!=0.0){(((v1697*(v1691*v3185))-(v1693*((v1677*v3185)/v5674)))/v5683)}else{v3});
        let v5703=(if (self.scalar_static_f64[245]!=0.0){(((v1697*((v1692*v5659)+(v1691*v3186)))-(v1693*(((v1677*v3186)+(v818*v5614))/v5674)))/v5683)}else{v3});
        let v5704=(if (self.scalar_static_f64[245]!=0.0){(((v1697*(v1691*v3187))-(v1693*((v1677*v3187)/v5674)))/v5683)}else{v3});
        let v5705=(if (self.scalar_static_f64[245]!=0.0){(((v1697*(v1691*v3188))-(v1693*((v1677*v3188)/v5674)))/v5683)}else{v3});
        let v5710=(if self.scalar_static_bool[44]{((v1703*v2792)+(v342*(self.scalar_static_f64[6]*v2929)))}else{v3});
        let v5723=(if self.scalar_static_bool[44]{(-(if self.scalar_static_bool[44]{((v1708*v2587)+(v125*(-(((v1705*v2590)+(v127*v5710))/v1706))))}else{v3}))}else{v3});
        let v5726=(v1712*self.scalar_static_f64[357]);
        let v5727=(v5726+v5726);
        let v5728=(v1712*self.scalar_static_f64[358]);
        let v5730=(v1712*v5723);
        let v5732=(v1712*self.scalar_static_f64[359]);
        let v5733=(v5732+v5732);
        let v5734=(v1712*self.scalar_static_f64[360]);
        let v5736=(if self.scalar_static_bool[44]{v5727}else{v3});
        let v5737=(if self.scalar_static_bool[44]{(v5728+v5728)}else{v3});
        let v5738=(if self.scalar_static_bool[44]{(v5730+v5730)}else{v4458});
        let v5739=(if self.scalar_static_bool[44]{v3}else{v4460});
        let v5740=(if self.scalar_static_bool[44]{v5727}else{v4462});
        let v5741=(if self.scalar_static_bool[44]{v5733}else{v4464});
        let v5742=(if self.scalar_static_bool[44]{v5733}else{v4466});
        let v5743=(if self.scalar_static_bool[44]{(v5734+v5734)}else{v3});
        let v5744=(if self.scalar_static_bool[44]{v5733}else{v3});
        let v5745=(v33*v1722);
        let v5746=(v5736/v5745);
        let v5747=(v5737/v5745);
        let v5748=(v5738/v5745);
        let v5749=(v5739/v5745);
        let v5750=(v5740/v5745);
        let v5751=(v5741/v5745);
        let v5752=(v5742/v5745);
        let v5753=(v5743/v5745);
        let v5754=(v5744/v5745);
        let v5765=(v1723*v1723);
        let v5817=(if v1727{(v431*(self.scalar_static_f64[357]+v5746))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5746-self.scalar_static_f64[357])))/v5765)}else{v3})});
        let v5818=(if v1727{(v431*(self.scalar_static_f64[358]+v5747))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5747-self.scalar_static_f64[358])))/v5765)}else{v3})});
        let v5819=(if v1727{(v431*(v5723+v5748))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5748-v5723)))/v5765)}else{v3})});
        let v5820=(if v1727{(v431*v5749)}else{(if v1719{((-(self.scalar_static_f64[249]*v5749))/v5765)}else{v3})});
        let v5821=(if v1727{(v431*(self.scalar_static_f64[357]+v5750))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5750-self.scalar_static_f64[357])))/v5765)}else{v3})});
        let v5822=(if v1727{(v431*(self.scalar_static_f64[359]+v5751))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5751-self.scalar_static_f64[359])))/v5765)}else{v3})});
        let v5823=(if v1727{(v431*(self.scalar_static_f64[359]+v5752))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5752-self.scalar_static_f64[359])))/v5765)}else{v3})});
        let v5824=(if v1727{(v431*(self.scalar_static_f64[360]+v5753))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5753-self.scalar_static_f64[360])))/v5765)}else{v3})});
        let v5825=(if v1727{(v431*(self.scalar_static_f64[359]+v5754))}else{(if v1719{((-(self.scalar_static_f64[249]*(v5754-self.scalar_static_f64[359])))/v5765)}else{v3})});
        let v5826=(v342*v5701);
        let v5831=(v342*v5704);
        let v5845=(v1733*v1733);
        let v5888=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5817)-(v1730*(v5817+v5826)))/v5845)}else{v3})});
        let v5889=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5818)-(v1730*(v5818+(v342*v5702))))/v5845)}else{v3})});
        let v5890=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5819)-(v1730*(v5819+(v5710+((v1699*v2792)+(v342*v5703))))))/v5845)}else{v3})});
        let v5891=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5820)-(v1730*v5820))/v5845)}else{v3})});
        let v5892=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5821)-(v1730*(v5821+v5826)))/v5845)}else{v3})});
        let v5893=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5822)-(v1730*(v5822+v5831)))/v5845)}else{v3})});
        let v5894=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5823)-(v1730*(v5823+v5831)))/v5845)}else{v3})});
        let v5895=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5824)-(v1730*(v5824+(v342*v5705))))/v5845)}else{v3})});
        let v5896=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1733*v5825)-(v1730*(v5825+v5831)))/v5845)}else{v3})});
        let v6190=(v1208*v4381);
        let v6192=(v1208*v4371);
        let v6194=(v1208*v4382);
        let v6196=(v1208*v4379);
        let v6198=(v1208*v4380);
        let v6200=(v33*v1803);
        let v6201=((v6190+v6190)/v6200);
        let v6202=((v6192+v6192)/v6200);
        let v6203=((v6194+v6194)/v6200);
        let v6204=((v6196+v6196)/v6200);
        let v6205=((v6198+v6198)/v6200);
        let v6213=(v1804*v1804);
        let v6242=(if v1807{(v431*(v4381+v6201))}else{(if (v1801!=0.0){((-(v1231*(v6201-v4381)))/v6213)}else{v3})});
        let v6243=(if v1807{(v431*(v4371+v6202))}else{(if (v1801!=0.0){((-(v1231*(v6202-v4371)))/v6213)}else{v3})});
        let v6244=(if v1807{(v431*(v4382+v6203))}else{(if (v1801!=0.0){((-(v1231*(v6203-v4382)))/v6213)}else{v3})});
        let v6245=(if v1807{(v431*(v4379+v6204))}else{(if (v1801!=0.0){((-(v1231*(v6204-v4379)))/v6213)}else{v3})});
        let v6246=(if v1807{(v431*(v4380+v6205))}else{(if (v1801!=0.0){((-(v1231*(v6205-v4380)))/v6213)}else{v3})});
        let v7667=(self.scalar_static_f64[297]*v2766);
        let v7675=((v4031-(v2148*v4029))/v4034);
        let v7708=(if v2158{(v4028-((v2162*v4029)+(v1105*((v2160*(-v7675))/v2161))))}else{(if (v2151!=0.0){(-((v2154*v4029)+(v1105*((v2152*v7675)/v2153))))}else{v3})});
        let v7709=(if v2158{(-(v1105*((v2160*v4056)/v2161)))}else{(if (v2151!=0.0){(self.scalar_static_f64[331]-(v1105*((v2152*v4036)/v2153)))}else{v3})});
        let v7710=(if v2158{(-(v1105*((v2160*v4057)/v2161)))}else{(if (v2151!=0.0){(self.scalar_static_f64[0]-(v1105*((v2152*v4037)/v2153)))}else{v3})});
        let v7721=(self.scalar_static_f64[221]*f64::powf(v2168,self.scalar_static_f64[335]));
        let v7756=((v647*v2868)+(v461*v3055));
        let v7757=(v431*v7756);
        let v7765=((v2181*v6242)+(v1810*((v2180*v4316)+(v1195*v7757))));
        let v7768=((v2181*v6243)+(v1810*(v2180*v4320)));
        let v7771=((v2181*v6244)+(v1810*(v2180*v4324)));
        let v7772=(v2181*v6245);
        let v7773=(v2181*v6246);
        let v7782=((v2183*v6242)+(v1810*((v2180*v4353)+(v1202*v7757))));
        let v7783=(v2183*v6243);
        let v7786=((v2183*v6244)+(v1810*(v2180*v4357)));
        let v7789=((v2183*v6245)+(v1810*(v2180*v4361)));
        let v7792=((v2183*v6246)+(v1810*(v2180*v4365)));
        let v7794=(v1049*(-v4132));
        let v7797=(v1049*v1049);
        let v7798=((v7794-(v2185*v3850))/v7797);
        let v7799=(self.scalar_static_f64[0]/v1049);
        let v7800=(self.scalar_static_f64[332]/v1049);
        let v7801=(self.scalar_static_f64[333]/v1049);
        let v7802=(self.scalar_static_f64[331]/v1049);
        let v7832=(-v7800);
        let v7833=(-v7801);
        let v7834=(-v7802);
        let v7857=(if v2195{(v4132-((v2199*v3850)+(v1049*((v2197*(-v7798))/v2198))))}else{(if (v2188!=0.0){(-((v2191*v3850)+(v1049*((v2189*v7798)/v2190))))}else{v3})});
        let v7858=(if v2195{(-(v1049*((v2197*(-v7799))/v2198)))}else{(if (v2188!=0.0){(self.scalar_static_f64[0]-(v1049*((v2189*v7799)/v2190)))}else{v3})});
        let v7859=(if v2195{(-(v1049*((v2197*v7832)/v2198)))}else{(if (v2188!=0.0){(self.scalar_static_f64[332]-(v1049*((v2189*v7800)/v2190)))}else{v3})});
        let v7860=(if v2195{(-(v1049*((v2197*v7833)/v2198)))}else{(if (v2188!=0.0){(self.scalar_static_f64[333]-(v1049*((v2189*v7801)/v2190)))}else{v3})});
        let v7861=(if v2195{(-(v1049*((v2197*v7834)/v2198)))}else{(if (v2188!=0.0){(self.scalar_static_f64[331]-(v1049*((v2189*v7802)/v2190)))}else{v3})});
        let v7876=(self.scalar_static_f64[227]*f64::powf(v2204,self.scalar_static_f64[340]));
        let v7919=(v314*self.scalar_static_f64[332]);
        let v7920=(v314*self.scalar_static_f64[333]);
        let v7943=(self.scalar_static_f64[334]/v1049);
        let v7946=((v7794-(v2218*v3850))/v7797);
        let v7998=(if v2228{(-(v1049*((v2230*v7832)/v2231)))}else{(if (v2221!=0.0){(self.scalar_static_f64[332]-(v1049*((v2222*v7800)/v2223)))}else{v3})});
        let v7999=(if v2228{(-(v1049*((v2230*(-v7943))/v2231)))}else{(if (v2221!=0.0){(self.scalar_static_f64[334]-(v1049*((v2222*v7943)/v2223)))}else{v3})});
        let v8000=(if v2228{(v4132-((v2232*v3850)+(v1049*((v2230*(-v7946))/v2231))))}else{(if (v2221!=0.0){(-((v2224*v3850)+(v1049*((v2222*v7946)/v2223))))}else{v3})});
        let v8001=(if v2228{(-(v1049*((v2230*v7833)/v2231)))}else{(if (v2221!=0.0){(self.scalar_static_f64[333]-(v1049*((v2222*v7801)/v2223)))}else{v3})});
        let v8002=(if v2228{(-(v1049*((v2230*v7834)/v2231)))}else{(if (v2221!=0.0){(self.scalar_static_f64[331]-(v1049*((v2222*v7802)/v2223)))}else{v3})});
        let v8017=(self.scalar_static_f64[227]*f64::powf(v2237,self.scalar_static_f64[340]));
        let v8078=(self.scalar_static_f64[6]*(self.scalar_static_f64[299]*(v313*(v7919+(v1148*((v1175*(-((-(v7998/v272))*v8017)))+(v1149*(self.scalar_static_f64[332]-v7998))))))));
        let v8081=(self.scalar_static_f64[6]*(self.scalar_static_f64[299]*(v313*(v7920+(v1148*((v1175*(-((-(v8001/v272))*v8017)))+(v1149*(self.scalar_static_f64[333]-v8001))))))));
        let v8097=(self.scalar_static_f64[300]*v2587);
        let v8100=(v2256*v2256);
        let v8101=((-(v728*v8097))/v8100);
        let v8102=(self.scalar_static_f64[331]/v2256);
        let v8103=(self.scalar_static_f64[0]/v2256);
        let v8124=((v2267*((v2254*((v641*v2868)+(v461*((v640*(self.scalar_static_f64[180]*(v635*(self.scalar_static_f64[181]*v2591))))+(v636*(v640*(self.scalar_static_f64[183]*v2590)))))))+(v2250*((((v461*v2865)-(v456*v2868))/v4301)*(self.scalar_static_f64[301]*f64::powf(v2251,self.scalar_static_f64[379]))))))+(v2255*(if v2262{(v2263*v8101)}else{(if (v2259!=0.0){(v2260*v8101)}else{v5023})})));
        let v8125=(v2255*(if v2262{(v2263*v8102)}else{(if (v2259!=0.0){(v2260*v8102)}else{v5024})}));
        let v8126=(v2255*(if v2262{v3}else{(if (v2259!=0.0){v3}else{v5025})}));
        let v8127=(v2255*(if v2262{(v2263*v8103)}else{(if (v2259!=0.0){(v2260*v8103)}else{v5026})}));
        let v8128=(v2255*(if v2262{v3}else{(if (v2259!=0.0){v3}else{v5027})}));
        let v8129=(v2255*(if v2262{v3}else{(if (v2259!=0.0){v3}else{v5028})}));
        let v8137=(((v354*((v2269*v2587)+(v125*(v443*v3058))))-(v2270*v2799))/v3330);
        let v8187=(v654*v654);
        let v8198=(-(if v244{((v248*v2587)+(v125*((v246*(-v2692))/v247)))}else{(if (v237!=0.0){(v2687+((v240*v2587)+(v125*((v238*v2692)/v239))))}else{v3})}));
        let v8206=((v2288*v2590)+(v127*(v8198/self.scalar_static_f64[304])));
        let v8207=(v127*self.scalar_static_f64[380]);
        let v8208=(v127*self.scalar_static_f64[381]);
        let v8209=(v127*self.scalar_static_f64[382]);
        let v8210=(v127*self.scalar_static_f64[383]);
        let v8246=(v33*v2307);
        let v8255=(v2308*v2308);
        let v8273=(if self.scalar_static_bool[60]{(((v2308*((v2303*v3149)+(v796*((v1673*v3064)+(v663*v5601)))))-(v2304*((v443*(if v2297{(v2298*v8206)}else{(if v2293{(v2294*v8206)}else{v3})}))/v8246)))/v8255)}else{(if (self.scalar_static_f64[303]!=0.0){(((v654*((v2282*(v431*v3061))+(v2279*(((v2179*(((v1667*(v5536-v4302))-(v1664*(v5536/v5547)))/v5556))+(v1668*v7756))+((v2271*(((v1671*v5541)-(v1663*(v5541/v5574)))/v5583))+(v1672*v8137))))))-(v2283*v3059))/v8187)}else{v3})});
        let v8274=(if self.scalar_static_bool[60]{(((v2308*(v2303*v3150))-(v2304*((v443*(if v2297{(v2298*v8207)}else{(if v2293{(v2294*v8207)}else{v3})}))/v8246)))/v8255)}else{(if (self.scalar_static_f64[303]!=0.0){((v2279*((v2179*(((v1667*v5537)-(v1664*(v5537/v5547)))/v5556))+(v2271*(((v1671*v5542)-(v1663*(v5542/v5574)))/v5583))))/v654)}else{v3})});
        let v8275=(if self.scalar_static_bool[60]{(((v2308*(v2303*v3151))-(v2304*((v443*(if v2297{(v2298*v8208)}else{(if v2293{(v2294*v8208)}else{v3})}))/v8246)))/v8255)}else{(if (self.scalar_static_f64[303]!=0.0){((v2279*((v2179*(((v1667*v5538)-(v1664*(v5538/v5547)))/v5556))+(v2271*(((v1671*v5543)-(v1663*(v5543/v5574)))/v5583))))/v654)}else{v3})});
        let v8276=(if self.scalar_static_bool[60]{(((v2308*(v2303*v3152))-(v2304*((v443*(if v2297{(v2298*v8209)}else{(if v2293{(v2294*v8209)}else{v3})}))/v8246)))/v8255)}else{(if (self.scalar_static_f64[303]!=0.0){((v2279*((v2179*(((v1667*v5539)-(v1664*(v5539/v5547)))/v5556))+(v2271*(((v1671*v5544)-(v1663*(v5544/v5574)))/v5583))))/v654)}else{v3})});
        let v8277=(if self.scalar_static_bool[60]{(((v2308*(v2303*v3153))-(v2304*((v443*(if v2297{(v2298*v8210)}else{(if v2293{(v2294*v8210)}else{v3})}))/v8246)))/v8255)}else{(if (self.scalar_static_f64[303]!=0.0){((v2279*((v2179*(((v1667*v5540)-(v1664*(v5540/v5547)))/v5556))+(v2271*(((v1671*v5545)-(v1663*(v5545/v5574)))/v5583))))/v654)}else{v3})});
        let v8295=(if self.scalar_static_bool[64]{(v1190*v3184)}else{v3});
        let v8296=(if self.scalar_static_bool[64]{(v1190*v3185)}else{v3});
        let v8297=(if self.scalar_static_bool[64]{((v1190*v3186)+(v818*v4302))}else{v3});
        let v8298=(if self.scalar_static_bool[64]{(v1190*v3187)}else{v3});
        let v8299=(if self.scalar_static_bool[64]{(v1190*v3188)}else{v3});
        let v8301=(v33*v2322);
        let v8310=(v2323*v2323);
        let v8338=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*v3132)}else{(if (v822!=0.0){(v823*v3132)}else{v3})}))}else{v3});
        let v8339=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*v3167)}else{(if (v822!=0.0){(v823*v3167)}else{v3})}))}else{v3});
        let v8340=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*v3192)}else{(if (v822!=0.0){(v823*v3192)}else{v3})}))}else{v3});
        let v8341=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*v3133)}else{(if (v822!=0.0){(v823*v3133)}else{v3})}))}else{v3});
        let v8342=(if self.scalar_static_bool[64]{(v443*(if v825{(v826*v3099)}else{(if (v822!=0.0){(v823*v3099)}else{v3})}))}else{v3});
        let v8343=(v33*v2329);
        let v8352=(v2330*v2330);
        let v8417=((v2341*v2590)+(v127*v8198));
        let v8453=(v33*v2360);
        let v8462=(v2361*v2361);
        let v8486=(v1738*(if self.scalar_static_bool[65]{(((v2361*(v2356*v3184))-(v2357*((v443*(if v2350{(v2351*v3132)}else{(if v2346{(v2347*v3132)}else{v3})}))/v8453)))/v8462)}else{(if self.scalar_static_bool[64]{((v2334*((v2179*(if self.scalar_static_bool[64]{(((v2323*v8295)-(v2320*(v8295/v8301)))/v8310)}else{v3}))+(v2271*(if self.scalar_static_bool[64]{(((v2330*v8338)-(v2327*(v8338/v8343)))/v8352)}else{v3}))))/v654)}else{v3})}));
        let v8498=(v1738*(if self.scalar_static_bool[65]{(((v2361*(v2356*v3187))-(v2357*((v443*(if v2350{(v2351*v3133)}else{(if v2346{(v2347*v3133)}else{v3})}))/v8453)))/v8462)}else{(if self.scalar_static_bool[64]{((v2334*((v2179*(if self.scalar_static_bool[64]{(((v2323*v8298)-(v2320*(v8298/v8301)))/v8310)}else{v3}))+(v2271*(if self.scalar_static_bool[64]{(((v2330*v8341)-(v2327*(v8341/v8343)))/v8352)}else{v3}))))/v654)}else{v3})}));
        let v8518=(self.scalar_static_f64[309]*f64::powf(v1125,self.scalar_static_f64[384]));
        let v8525=(if (self.scalar_static_f64[308]!=0.0){v4035}else{v3});
        let v8526=(if (self.scalar_static_f64[308]!=0.0){v4036}else{v3});
        let v8527=(if (self.scalar_static_f64[308]!=0.0){v4037}else{v3});
        let v8532=(v2378*v2378);
        let v8544=(v2384*(-v8525));
        let v8545=(v2384*(-v8526));
        let v8546=(v2384*(-v8527));
        let v8550=(v2385*v2385);
        let v8596=(v1193*v1193);
        let v8652=(if (self.scalar_static_f64[308]!=0.0){(v8128/v2256)}else{v3});
        let v8698=(self.scalar_static_f64[310]*v8128);
        let v8705=(if (self.scalar_static_f64[308]!=0.0){(v7765+(self.scalar_static_f64[310]*v8124))}else{v3});
        let v8706=(if (self.scalar_static_f64[308]!=0.0){(v7768+(self.scalar_static_f64[310]*v8125))}else{v3});
        let v8707=(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[310]*v8126)}else{v3});
        let v8708=(if (self.scalar_static_f64[308]!=0.0){(v7771+(self.scalar_static_f64[310]*v8127))}else{v3});
        let v8709=(if (self.scalar_static_f64[308]!=0.0){(v7772+v8698)}else{v3});
        let v8710=(if (self.scalar_static_f64[308]!=0.0){(v7773+v8698)}else{v3});
        let v8711=(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[310]*v8129)}else{v3});
        let v8745=(if self.scalar_static_bool[67]{v7765}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8705)}else{v3})});
        let v8746=(if self.scalar_static_bool[67]{v7768}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8706)}else{v3})});
        let v8747=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8707)}else{v3})});
        let v8748=(if self.scalar_static_bool[67]{v7771}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8708)}else{v3})});
        let v8749=(if self.scalar_static_bool[67]{v7772}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8709)}else{v3})});
        let v8750=(if self.scalar_static_bool[67]{v7773}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8710)}else{v3})});
        let v8751=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[313]*v8711)}else{v3})});
        let v8752=(if self.scalar_static_bool[67]{v7782}else{(if (self.scalar_static_f64[308]!=0.0){(v7782+(self.scalar_static_f64[312]*v8705))}else{v3})});
        let v8753=(if self.scalar_static_bool[67]{v7783}else{(if (self.scalar_static_f64[308]!=0.0){(v7783+(self.scalar_static_f64[312]*v8706))}else{v3})});
        let v8754=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[312]*v8707)}else{v3})});
        let v8755=(if self.scalar_static_bool[67]{v7786}else{(if (self.scalar_static_f64[308]!=0.0){(v7786+(self.scalar_static_f64[312]*v8708))}else{v3})});
        let v8756=(if self.scalar_static_bool[67]{v7789}else{(if (self.scalar_static_f64[308]!=0.0){(v7789+(self.scalar_static_f64[312]*v8709))}else{v3})});
        let v8757=(if self.scalar_static_bool[67]{v7792}else{(if (self.scalar_static_f64[308]!=0.0){(v7792+(self.scalar_static_f64[312]*v8710))}else{v3})});
        let v8758=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[312]*v8711)}else{v3})});
        let v8763=(if self.scalar_static_bool[67]{v8128}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8128)}else{v3})});
        let v8804=(v2466*v2466);
        let v8863=(if v2480{((v2481*v4523)+(v1244*((v1810*v3055)+(v647*v6242))))}else{(if (v2476!=0.0){(((v2466*(v8745+v8752))-(v2477*(((v1244*(v4539+v4545))-(v2465*v4523))/v4555)))/v8804)}else{v3})});
        let v8864=(if v2480{((v2481*v4526)+(v1244*(v647*v6243)))}else{(if (v2476!=0.0){(((v2466*(v8746+v8753))-(v2477*((v4557-(v2465*v4526))/v4555)))/v8804)}else{v3})});
        let v8865=(if v2480{v3}else{(if (v2476!=0.0){((v8747+v8754)/v2466)}else{v3})});
        let v8866=(if v2480{((v2481*v4529)+(v1244*(v647*v6244)))}else{(if (v2476!=0.0){(((v2466*(v8748+v8755))-(v2477*(((v1244*(v4540+v4547))-(v2465*v4529))/v4555)))/v8804)}else{v3})});
        let v8867=(if v2480{((v2481*v4532)+(v1244*(v647*v6245)))}else{(if (v2476!=0.0){(((v2466*(v8749+v8756))-(v2477*(((v1244*v4541)-(v2465*v4532))/v4555)))/v8804)}else{v3})});
        let v8868=(if v2480{((v2481*v4535)+(v1244*(v647*v6246)))}else{(if (v2476!=0.0){(((v2466*(v8750+v8757))-(v2477*(((v1244*v4542)-(v2465*v4535))/v4555)))/v8804)}else{v3})});
        let v8869=(if v2480{v3}else{(if (v2476!=0.0){((v8751+v8758)/v2466)}else{v3})});
        let v8898=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8863)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8863)}else{v3})})});
        let v8899=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8864)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8864)}else{v3})})});
        let v8900=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8865)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8865)}else{v3})})});
        let v8901=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8866)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8866)}else{v3})})});
        let v8902=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8867)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8867)}else{v3})})});
        let v8903=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8868)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8868)}else{v3})})});
        let v8904=(if self.scalar_static_bool[85]{v3}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[326]*v8869)}else{(if (self.scalar_static_f64[324]!=0.0){(self.scalar_static_f64[312]*v8869)}else{v3})})});
        let v8940=((self.scalar_static_f64[6]*(self.scalar_static_f64[299]*((v2246*v2777)+(v313*(((v2243*v4119)+(v1148*(((v2239*v4218)+(v1175*(-((-(((v272*v8000)-(v2235*v2728))/v2756))*v8017))))+((v2241*v4124)+(v1149*(-v8000))))))+(v760*v2778))))))+(if (self.scalar_static_f64[305]!=0.0){((v2363*v5890)+(v1738*(if self.scalar_static_bool[65]{(((v2361*((v2356*v3186)+(v818*((v1691*v3064)+(v663*v5659)))))-(v2357*((v443*(if v2350{(v2351*v8417)}else{(if v2346{(v2347*v8417)}else{v3})}))/v8453)))/v8462)}else{(if self.scalar_static_bool[64]{(((v654*((v2337*(self.scalar_static_f64[306]*v3061))+(v2334*(((v2325*v7756)+(v2179*(if self.scalar_static_bool[64]{(((v2323*(v8297-v4302))-(v2320*(v8297/v8301)))/v8310)}else{v3})))+((v2332*v8137)+(v2271*(if self.scalar_static_bool[64]{(((v2330*v8340)-(v2327*(v8340/v8343)))/v8352)}else{v3})))))))-(v2338*v3059))/v8187)}else{v3})})))}else{v3}));
        let v9100=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8124}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8124)}else{v3})})+(((v2146*v4104)+(v1133*v7667))+v8745)));
        let v9101=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8125}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8125)}else{v3})})+((v2146*v4105)+v8746)));
        let v9102=(self.scalar_static_f64[0]*(v8747+(if self.scalar_static_bool[67]{v8126}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8126)}else{v3})})));
        let v9103=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8127}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8127)}else{v3})})+((v2146*v4106)+v8748)));
        let v9104=(self.scalar_static_f64[0]*(v8749+v8763));
        let v9105=(self.scalar_static_f64[0]*(v8750+v8763));
        let v9106=(self.scalar_static_f64[0]*(v8751+(if self.scalar_static_bool[67]{v8129}else{(if (self.scalar_static_f64[308]!=0.0){(self.scalar_static_f64[311]*v8129)}else{v3})})));
        let v9121=(self.scalar_static_f64[0]*((v2174*(self.scalar_static_f64[296]*v2766))+(v2166*(((v2170*v4089)+(v1128*(-((-((v2165*v2754)+(v297*v7708)))*v7721))))+(v178*(-v7708))))));
        let v9122=(self.scalar_static_f64[0]*(v2166*((v1128*(-((-(v297*v7709))*v7721)))+(v178*(self.scalar_static_f64[331]-v7709)))));
        let v9123=(self.scalar_static_f64[0]*(v2166*((v1128*(-((-(v297*v7710))*v7721)))+(v178*(self.scalar_static_f64[0]-v7710)))));
        let v9130=(self.scalar_static_f64[0]*(((v2274*((v2272*v4000)+(v1094*(v431*v8137))))+(v2273*v3948))+(((v2177*v4294)+(v1188*(self.scalar_static_f64[298]*v2777)))+v8752)));
        let v9131=(self.scalar_static_f64[0]*v8753);
        let v9132=(self.scalar_static_f64[0]*v8754);
        let v9133=(self.scalar_static_f64[0]*(((v2274*(v2272*v4001))+(v2273*v3949))+((v2177*v4295)+v8755)));
        let v9134=(self.scalar_static_f64[0]*(((v2274*(v2272*v4002))+(v2273*v3950))+((v2177*v4296)+v8756)));
        let v9135=(self.scalar_static_f64[0]*(((v2274*(v2272*v4003))+(v2273*v3943))+((v2177*v4290)+v8757)));
        let v9136=(self.scalar_static_f64[0]*v8758);
        let v9151=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*((if (self.scalar_static_f64[308]!=0.0){(((v2256*v8124)-(v2268*v8097))/v8100)}else{v3})+((if (self.scalar_static_f64[308]!=0.0){((v2390*v7667)+(v2146*(if (self.scalar_static_f64[308]!=0.0){((v2387*(if (self.scalar_static_f64[308]!=0.0){(v4080*v8518)}else{v3}))+(v2372*(if v2382{(((v2385*v8544)-(v2384*v8544))/v8550)}else{(if v2376{((-(v2377*v8525))/v8532)}else{v3})})))}else{v3})))}else{v3})+(if (self.scalar_static_f64[308]!=0.0){((v2398*(if (self.scalar_static_f64[308]!=0.0){((v2395*(((v388*((v1191*v2590)+(v127*v4305)))-(v2393*v2816))/v2853))+(v2394*((-(v431*v4309))/v8596)))}else{v3}))+(v2397*((v2180*v6242)+(v1810*v7757))))}else{v3}))))}else{v3}));
        let v9152=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*((if (self.scalar_static_f64[308]!=0.0){(v8125/v2256)}else{v3})+((if (self.scalar_static_f64[308]!=0.0){(v2146*(if (self.scalar_static_f64[308]!=0.0){((v2387*(if (self.scalar_static_f64[308]!=0.0){(v4081*v8518)}else{v3}))+(v2372*(if v2382{(((v2385*v8545)-(v2384*v8545))/v8550)}else{(if v2376{((-(v2377*v8526))/v8532)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[308]!=0.0){((v2398*(if (self.scalar_static_f64[308]!=0.0){((v2395*((v127*v4306)/v388))+(v2394*((-(v431*v4310))/v8596)))}else{v3}))+(v2397*(v2180*v6243)))}else{v3}))))}else{v3}));
        let v9153=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){((v2405*self.scalar_static_f64[385])+(v2403*(if (self.scalar_static_f64[308]!=0.0){(v8126/v2256)}else{v3})))}else{v3}));
        let v9154=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){((v2405*self.scalar_static_f64[386])+(v2403*((if (self.scalar_static_f64[308]!=0.0){(v8127/v2256)}else{v3})+((if (self.scalar_static_f64[308]!=0.0){(v2146*(if (self.scalar_static_f64[308]!=0.0){((v2387*(if (self.scalar_static_f64[308]!=0.0){(v4082*v8518)}else{v3}))+(v2372*(if v2382{(((v2385*v8546)-(v2384*v8546))/v8550)}else{(if v2376{((-(v2377*v8527))/v8532)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[308]!=0.0){((v2398*(if (self.scalar_static_f64[308]!=0.0){((v2395*((v127*v4307)/v388))+(v2394*((-(v431*v4311))/v8596)))}else{v3}))+(v2397*(v2180*v6244)))}else{v3})))))}else{v3}));
        let v9155=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*((if (self.scalar_static_f64[308]!=0.0){(v2397*(v2180*v6245))}else{v3})+v8652))}else{v3}));
        let v9156=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*((if (self.scalar_static_f64[308]!=0.0){(v2397*(v2180*v6246))}else{v3})+v8652))}else{v3}));
        let v9157=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[308]!=0.0){(v2403*(if (self.scalar_static_f64[308]!=0.0){(v8129/v2256)}else{v3}))}else{v3}));
        let v9216=(self.scalar_static_f64[0]*(v8078+(if (self.scalar_static_f64[305]!=0.0){((v2363*v5888)+v8486)}else{v3})));
        let v9217=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[299]*(v313*((v1148*((v1175*(-((-(v7999/v272))*v8017)))+(v1149*(self.scalar_static_f64[334]-v7999))))+(v314*self.scalar_static_f64[334])))))+(if (self.scalar_static_f64[305]!=0.0){((v2363*v5889)+(v1738*(if self.scalar_static_bool[65]{(((v2361*(v2356*v3185))-(v2357*((v443*(if v2350{(v2351*v3167)}else{(if v2346{(v2347*v3167)}else{v3})}))/v8453)))/v8462)}else{(if self.scalar_static_bool[64]{((v2334*((v2179*(if self.scalar_static_bool[64]{(((v2323*v8296)-(v2320*(v8296/v8301)))/v8310)}else{v3}))+(v2271*(if self.scalar_static_bool[64]{(((v2330*v8339)-(v2327*(v8339/v8343)))/v8352)}else{v3}))))/v654)}else{v3})})))}else{v3})));
        let v9218=(self.scalar_static_f64[0]*v8940);
        let v9219=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2363*v5891)}else{v3}));
        let v9220=(self.scalar_static_f64[0]*(v8078+(if (self.scalar_static_f64[305]!=0.0){(v8486+(v2363*v5892))}else{v3})));
        let v9221=(self.scalar_static_f64[0]*(v8081+(if (self.scalar_static_f64[305]!=0.0){((v2363*v5893)+v8498)}else{v3})));
        let v9222=(self.scalar_static_f64[0]*(v8081+(if (self.scalar_static_f64[305]!=0.0){(v8498+(v2363*v5894))}else{v3})));
        let v9223=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[299]*(v313*(v4293+(v1148*((v1175*(-((-(v8002/v272))*v8017)))+(v1149*(self.scalar_static_f64[331]-v8002))))))))+(if (self.scalar_static_f64[305]!=0.0){((v2363*v5895)+(v1738*(if self.scalar_static_bool[65]{(((v2361*(v2356*v3188))-(v2357*((v443*(if v2350{(v2351*v3099)}else{(if v2346{(v2347*v3099)}else{v3})}))/v8453)))/v8462)}else{(if self.scalar_static_bool[64]{((v2334*((v2179*(if self.scalar_static_bool[64]{(((v2323*v8299)-(v2320*(v8299/v8301)))/v8310)}else{v3}))+(v2271*(if self.scalar_static_bool[64]{(((v2330*v8342)-(v2327*(v8342/v8343)))/v8352)}else{v3}))))/v654)}else{v3})})))}else{v3})));
        let v9224=(self.scalar_static_f64[0]*(v8081+(if (self.scalar_static_f64[305]!=0.0){(v8498+(v2363*v5896))}else{v3})));
        let v9268=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[299]*((v2213*v2777)+(v313*(((v2210*v4119)+(v1148*(((v2206*v4218)+(v1175*(-((-(((v272*v7857)-(v2202*v2728))/v2756))*v7876))))+((v2208*v4124)+(v1149*(-v7857))))))+(v755*v2778))))))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v8273)}else{v8273})));
        let v9269=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[299]*(v313*(v4292+(v1148*((v1175*(-((-(v7858/v272))*v7876)))+(v1149*(self.scalar_static_f64[0]-v7858))))))))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v8274)}else{v8274})));
        let v9270=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[299]*(v313*((v1148*((v1175*(-((-(v7859/v272))*v7876)))+(v1149*(self.scalar_static_f64[332]-v7859))))+v7919))))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v8275)}else{v8275})));
        let v9271=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[299]*(v313*((v1148*((v1175*(-((-(v7860/v272))*v7876)))+(v1149*(self.scalar_static_f64[333]-v7860))))+v7920))))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v8276)}else{v8276})));
        let v9272=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[299]*(v313*(v4293+(v1148*((v1175*(-((-(v7861/v272))*v7876)))+(v1149*(self.scalar_static_f64[331]-v7861))))))))+(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[7]*v8277)}else{v8277})));

        CommonStampValues {
            v1, v3, v32, v33, v48, v105, v122, v123, 
            v125, v127, v129, v130, v131, v132, v133, v134, 
            v140, v141, v142, v147, v149, v150, v154, v155, 
            v156, v157, v163, v164, v165, v170, v172, v173, 
            v177, v178, v205, v229, v272, v282, v283, v284, 
            v285, v289, v291, v292, v293, v297, v298, v300, 
            v301, v302, v342, v427, v430, v431, v432, v434, 
            v435, v438, v441, v443, v456, v469, v579, v580, 
            v581, v582, v584, v585, v586, v588, v591, v602, 
            v603, v604, v606, v607, v608, v610, v613, v722, 
            v725, v726, v728, v731, v733, v736, v741, v749, 
            v752, v755, v759, v760, v796, v797, v799, v802, 
            v803, v887, v902, v1009, v1069, v1094, v1097, v1100, 
            v1127, v1207, v1243, v1244, v1249, v1250, v1269, v1271, 
            v1274, v1275, v1284, v1316, v1317, v1318, v1320, v1325, 
            v1326, v1333, v1334, v1336, v1341, v1343, v1395, v1396, 
            v1397, v1399, v1404, v1405, v1432, v1445, v1458, v1471, 
            v1478, v1479, v1481, v1482, v1484, v1489, v1490, v1496, 
            v1500, v1503, v1511, v1512, v1513, v1515, v1517, v1519, 
            v1520, v1521, v1522, v1524, v1527, v1529, v1530, v1535, 
            v1536, v1574, v1576, v1578, v1579, v1581, v1582, v1584, 
            v1589, v1590, v1595, v1598, v1600, v1608, v1609, v1610, 
            v1612, v1615, v1616, v1617, v1618, v1620, v1622, v1624, 
            v1625, v1630, v1631, v1673, v1677, v1699, v1716, v1738, 
            v1810, v1822, v1835, v1836, v1837, v1840, v1841, v1845, 
            v1846, v1848, v1849, v1851, v1852, v1854, v1859, v1860, 
            v1875, v1982, v1983, v1985, v1987, v1989, v1991, v1992, 
            v1994, v2002, v2005, v2006, v2007, v2013, v2015, v2016, 
            v2020, v2022, v2024, v2025, v2027, v2032, v2033, v2092, 
            v2427, v2466, v2498, v2534, v2537, v2540, v2543, v2547, 
            v2551, v2559, v2565, v2576, v2585, v2586, v2587, v2590, 
            v2591, v2661, v2684, v2728, v2732, v2737, v2754, v2756, 
            v2761, v2792, v2835, v2837, v2865, v2961, v3036, v3098, 
            v3099, v3149, v3150, v3151, v3152, v3153, v3331, v3332, 
            v3333, v3334, v3341, v3733, v3734, v3735, v3736, v3944, 
            v3945, v3946, v3947, v4000, v4001, v4002, v4003, v4012, 
            v4013, v4014, v4015, v4024, v4025, v4026, v4027, v4086, 
            v4087, v4088, v4377, v4378, v4379, v4380, v4516, v4517, 
            v4518, v4519, v4520, v4523, v4526, v4529, v4532, v4535, 
            v4539, v4540, v4541, v4542, v4545, v4547, v4555, v4557, 
            v4593, v4594, v4658, v4659, v4660, v4856, v4857, v4858, 
            v4859, v4940, v4941, v4942, v4943, v4963, v4964, v4965, 
            v4966, v4994, v4995, v4996, v4997, v4998, v4999, v5023, 
            v5024, v5025, v5026, v5027, v5028, v5601, v5614, v5701, 
            v5702, v5703, v5704, v5705, v5736, v5737, v5738, v5739, 
            v5740, v5741, v5742, v5743, v5744, v5888, v5889, v5890, 
            v5891, v5892, v5893, v5894, v5895, v5896, v6242, v6243, 
            v6244, v6245, v6246, v8898, v8899, v8900, v8901, v8902, 
            v8903, v8904, v9100, v9101, v9102, v9103, v9104, v9105, 
            v9106, v9121, v9122, v9123, v9130, v9131, v9132, v9133, 
            v9134, v9135, v9136, v9151, v9152, v9153, v9154, v9155, 
            v9156, v9157, v9216, v9217, v9218, v9219, v9220, v9221, 
            v9222, v9223, v9224, v9268, v9269, v9270, v9271, v9272, 
        }
    }

    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let multiplicity = self.multiplicity;
        let timestep = self.timestep;
        let ddt_state_current = self.ddt_state_current.as_mut();
        let ddt_state_previous = self.ddt_state_previous.as_mut();
        let ddt_state_older = self.ddt_state_older.as_mut();
        let ddt_state_initialized = self.ddt_state_initialized.as_mut();
        let ddt_derivative_current = self.ddt_derivative_current.as_mut();
        let ddt_derivative_previous = self.ddt_derivative_previous.as_mut();
        let ddt_active = self.ddt_coefficients.active;
        let ddt_scale = self.ddt_coefficients.derivative_scale;
        let ddt_previous_value_scale = self.ddt_coefficients.previous_value_scale;
        let ddt_older_value_scale = self.ddt_coefficients.older_value_scale;
        let ddt_previous_derivative_scale = self.ddt_coefficients.previous_derivative_scale;
        let common=self.eval_common_stamp_values(ctx);
        let v318=((common.v131*self.scalar_static_f64[97])).exp();
        let v319=(self.scalar_static_f64[96]*v318);
        let v321=(if (v319<self.scalar_static_f64[16]){common.v1}else{common.v3});
        let v322=(if (v321!=0.0){self.scalar_static_f64[16]}else{v319});
        let v328=((common.v131*self.scalar_static_f64[101])).exp();
        let v329=(self.scalar_static_f64[98]*v328);
        let v333=((common.v131*self.scalar_static_f64[103])).exp();
        let v334=(self.scalar_static_f64[102]*v333);
        let v336=(if (v334<self.scalar_static_f64[16]){common.v1}else{common.v3});
        let v337=(if (v336!=0.0){self.scalar_static_f64[16]}else{v334});
        let v346=((common.v131*self.scalar_static_f64[107])).exp();
        let v347=(self.scalar_static_f64[106]*v346);
        let v349=(v346*self.scalar_static_f64[108]);
        let v474=((common.v131*self.scalar_static_f64[133])).exp();
        let v475=(self.scalar_static_f64[130]*v474);
        let v478=(common.v129*self.scalar_static_f64[135]);
        let v480=((v478/self.scalar_static_f64[131])).exp();
        let v481=(v475*v480);
        let v487=((common.v131*self.scalar_static_f64[139])).exp();
        let v488=(self.scalar_static_f64[136]*v487);
        let v492=(((common.v129*self.scalar_static_f64[140])/self.scalar_static_f64[137])).exp();
        let v493=(v488*v492);
        let v497=(common.v131*self.scalar_static_f64[143]);
        let v500=((v497/self.scalar_static_f64[144])).exp();
        let v501=(self.scalar_static_f64[141]*v500);
        let v504=(common.v129*self.scalar_static_f64[146]);
        let v506=((v504/self.scalar_static_f64[144])).exp();
        let v507=(v501*v506);
        let v511=((v497/self.scalar_static_f64[148])).exp();
        let v512=(self.scalar_static_f64[147]*v511);
        let v514=((v504/self.scalar_static_f64[148])).exp();
        let v515=(v512*v514);
        let v524=(((common.v129*self.scalar_static_f64[153])/self.scalar_static_f64[144])).exp();
        let v531=((common.v129*self.scalar_static_f64[156])).exp();
        let v533=(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[154]*v531)}else{common.v3});
        let v539=(((common.v129*self.scalar_static_f64[159])/self.scalar_static_f64[148])).exp();
        let v558=((common.v131*self.scalar_static_f64[168])).exp();
        let v559=(self.scalar_static_f64[165]*v558);
        let v561=((v478/self.scalar_static_f64[166])).exp();
        let v562=(v559*v561);
        let v567=((common.v131*self.scalar_static_f64[171])).exp();
        let v568=(self.scalar_static_f64[169]*v567);
        let v570=((v478/self.scalar_static_f64[170])).exp();
        let v571=(v568*v570);
        let v573=(common.v123).sqrt();
        let v574=(self.scalar_static_f64[172]*v573);
        let v577=((common.v130*self.scalar_static_f64[173])).exp();
        let v578=(v574*v577);
        let v593=(common.v581*self.scalar_static_f64[175]);
        let v594=(common.v205*v593);
        let v597=(self.scalar_static_f64[48]*(self.scalar_static_f64[48]*(common.v205*v594)));
        let v598=(common.v300*v597);
        let v600=((self.scalar_static_f64[174]-common.v591)).exp();
        let v615=(common.v603*self.scalar_static_f64[177]);
        let v616=(common.v272*v615);
        let v619=(self.scalar_static_f64[79]*(self.scalar_static_f64[79]*(common.v272*v616)));
        let v620=(common.v302*v619);
        let v622=((self.scalar_static_f64[176]-common.v613)).exp();
        let v665=(common.v122-300.0);
        let v668=(if (common.v122<525.0){common.v1}else{common.v3});
        let v669=0.00072;
        let v672=1.6e-6;
        let v673=(v665*v672);
        let v678=(!(v668!=0.0));
        let v681=(if v678{self.scalar_static_f64[194]}else{(if (v668!=0.0){(self.scalar_static_f64[5]*((common.v1+(v665*v669))-(v665*v673)))}else{common.v3})});
        let v692=(if (self.scalar_static_f64[198]!=0.0){(common.v1/common.v342)}else{common.v3});
        let v695=((self.scalar_static_f64[198]!=0.0)&&((if (v692>self.scalar_static_f64[17]){common.v1}else{common.v3})!=0.0));
        let v698=(if self.scalar_static_bool[14]{common.v3}else{(if v695{self.scalar_static_f64[17]}else{v692})});
        let v702=(if (self.scalar_static_f64[199]!=0.0){(common.v1/v347)}else{common.v3});
        let v705=((self.scalar_static_f64[199]!=0.0)&&((if (v702>self.scalar_static_f64[17]){common.v1}else{common.v3})!=0.0));
        let v708=(if self.scalar_static_bool[16]{common.v3}else{(if v705{self.scalar_static_f64[17]}else{v702})});
        let v712=(if (self.scalar_static_f64[200]!=0.0){(common.v1/v349)}else{common.v3});
        let v715=((self.scalar_static_f64[200]!=0.0)&&((if (v712>self.scalar_static_f64[17]){common.v1}else{common.v3})!=0.0));
        let v718=(if self.scalar_static_bool[18]{common.v3}else{(if v715{self.scalar_static_f64[17]}else{v712})});
        let v738=(self.scalar_static_f64[0]*(common.v736-common.v726));
        let v800=(common.v797).exp();
        let v1272=(common.v1269).exp();
        let v1279=(if common.v1274{(common.v1275*(common.v1+(common.v1269-self.scalar_static_f64[201])))}else{(if (common.v1271!=0.0){v1272}else{common.v3})});
        let v1280=(v1279-common.v1);
        let v1286=(if (common.v728<self.scalar_static_f64[231]){common.v1}else{common.v3});
        let v1287=(common.v1284).exp();
        let v1288=(common.v1+v1287);
        let v1293=(!(v1286!=0.0));
        let v1295=((-common.v1284)).exp();
        let v1296=(common.v1+v1295);
        let v1300=(if v1293{(self.scalar_static_f64[231]-(common.v32*(v1296).ln()))}else{(if (v1286!=0.0){(common.v728-(common.v32*(v1288).ln()))}else{common.v3})});
        let v1302=(v1300*self.scalar_static_f64[232]);
        let v1303=(self.scalar_static_f64[231]-v1300);
        let v1304=f64::powf(v1303,common.v33);
        let v1321=((self.scalar_static_f64[150]!=0.0)&&(common.v1320!=0.0));
        let v1322=(common.v1318).exp();
        let v1330=(if common.v1325{(common.v1326*(common.v1+(common.v1318-self.scalar_static_f64[201])))}else{(if v1321{v1322}else{common.v1269})});
        let v1337=((self.scalar_static_f64[150]!=0.0)&&(common.v1336!=0.0));
        let v1338=(common.v1333).exp();
        let v1347=(if common.v1341{(common.v1343*(common.v1+(common.v1333-common.v1334)))}else{(if v1337{v1338}else{v1279})});
        let v1348=(common.v1316-common.v1);
        let v1349=(v507*v1348);
        let v1350=(common.v33*(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[151]*v524)}else{common.v3}));
        let v1351=(v1348*v1350);
        let v1354=((common.v1+(common.v443*v1330))).sqrt();
        let v1355=(common.v1+v1354);
        let v1356=(v1351/v1355);
        let v1357=(common.v1+common.v1207);
        let v1360=(common.v1069-common.v1);
        let v1361=(v533*v1360);
        let v1362=(v1347*v1361);
        let v1363=(common.v1+v1347);
        let v1379=(self.scalar_static_f64[233]*((common.v1069+common.v1316)-common.v33));
        let v1381=((v1348*self.scalar_static_f64[235])+(v1357*v1379));
        let v1400=((self.scalar_static_f64[150]!=0.0)&&(common.v1399!=0.0));
        let v1401=(common.v1397).exp();
        let v1410=(common.v1395-common.v1);
        let v1411=(v515*v1410);
        let v1412=(common.v33*(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[157]*v539)}else{common.v3}));
        let v1413=(v1410*v1412);
        let v1416=((common.v1+(common.v443*(if common.v1404{(common.v1405*(common.v1+(common.v1397-self.scalar_static_f64[201])))}else{(if v1400{v1401}else{v1330})})))).sqrt();
        let v1417=(common.v1+v1416);
        let v1433=(common.v1432-common.v1);
        let v1446=(common.v1445-common.v1);
        let v1459=(common.v1458-common.v1);
        let v1460=(v493*v1459);
        let v1472=(common.v1471-common.v1);
        let v1485=((common.v1478!=0.0)&&(common.v1484!=0.0));
        let v1486=(common.v1482).exp();
        let v1494=(if common.v1489{(common.v1490*(common.v1+(common.v1482-self.scalar_static_f64[201])))}else{(if v1485{v1486}else{common.v3})});
        let v1531=((common.v1529!=0.0)&&common.v1530);
        let v1532=(common.v1524).exp();
        let v1541=(-common.v728);
        let v1542=(common.v1-(if common.v1535{(common.v1536*(common.v1+(common.v1524-self.scalar_static_f64[201])))}else{(if v1531{v1532}else{common.v3})}));
        let v1544=(common.v1+(v1542/common.v1524));
        let v1548=((common.v1478!=0.0)&&(!(common.v1527!=0.0)));
        let v1549=(common.v431*common.v728);
        let v1550=(common.v1524*v1549);
        let v1551=0.3333333333333333;
        let v1552=(common.v1524*v1551);
        let v1553=0.25;
        let v1555=(common.v1+(common.v1524*v1553));
        let v1557=(common.v1+(v1552*v1555));
        let v1559=(if v1548{(v1550*v1557)}else{(if common.v1530{(v1541*v1544)}else{common.v3})});
        let v1560=(common.v33*(v598*v600));
        let v1561=(v1559*v1560);
        let v1562=(common.v1127*v1561);
        let v1563=(v1494*v1562);
        let v1567=(!(common.v1478!=0.0));
        let v1585=((common.v1574!=0.0)&&(common.v1584!=0.0));
        let v1586=(common.v1582).exp();
        let v1594=(if common.v1589{(common.v1590*(common.v1+(common.v1582-self.scalar_static_f64[201])))}else{(if v1585{v1586}else{common.v3})});
        let v1626=((common.v1624!=0.0)&&common.v1625);
        let v1627=(common.v1620).exp();
        let v1636=(-common.v722);
        let v1637=(common.v1-(if common.v1630{(common.v1631*(common.v1+(common.v1620-self.scalar_static_f64[201])))}else{(if v1626{v1627}else{common.v3})}));
        let v1639=(common.v1+(v1637/common.v1620));
        let v1643=((common.v1574!=0.0)&&(!(common.v1622!=0.0)));
        let v1644=(common.v431*common.v722);
        let v1645=(common.v1620*v1644);
        let v1646=(v1551*common.v1620);
        let v1648=(common.v1+(v1553*common.v1620));
        let v1650=(common.v1+(v1646*v1648));
        let v1652=(if v1643{(v1645*v1650)}else{(if common.v1625{(v1636*v1639)}else{common.v3})});
        let v1653=(common.v33*(v620*v622));
        let v1654=(v1652*v1653);
        let v1655=(common.v1578*v1654);
        let v1656=(v1594*v1655);
        let v1660=(!(common.v1574!=0.0));
        let v1661=(if v1660{common.v3}else{(if (common.v1574!=0.0){(self.scalar_static_f64[53]*(common.v298*v1656))}else{common.v3})});
        let v1674=(common.v796-common.v1);
        let v1675=(common.v1673*v1674);
        let v1680=((common.v1+(common.v796*common.v1677))).sqrt();
        let v1681=(common.v1+v1680);
        let v1682=(v1675/v1681);
        let v1689=(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v1682)}else{v1682});
        let v1740=(if (self.scalar_static_f64[245]!=0.0){(common.v1699*common.v1738)}else{common.v3});
        let v1745=(if (self.scalar_static_f64[251]!=0.0){(common.v722+common.v733)}else{common.v3});
        let v1747=(-v1745);
        let v1751=(if (v1747<common.v3){common.v1}else{common.v3});
        let v1752=((self.scalar_static_f64[251]!=0.0)&&(v1751!=0.0));
        let v1755=((self.scalar_static_f64[252]+(if (self.scalar_static_f64[251]!=0.0){(v1745*v1745)}else{common.v1716}))).sqrt();
        let v1756=(v1755-v1747);
        let v1760=((self.scalar_static_f64[251]!=0.0)&&(!(v1751!=0.0)));
        let v1763=(if v1760{(common.v431*(v1747+v1755))}else{(if v1752{(self.scalar_static_f64[253]/v1756)}else{common.v3})});
        let v1780=(if (v1763<self.scalar_static_f64[261]){common.v1}else{common.v3});
        let v1781=((self.scalar_static_f64[251]!=0.0)&&(v1780!=0.0));
        let v1782=(v1763/self.scalar_static_f64[259]);
        let v1784=(common.v1-f64::powf(v1782,self.scalar_static_f64[254]));
        let v1788=((self.scalar_static_f64[251]!=0.0)&&(!(v1780!=0.0)));
        let v1794=(if self.scalar_static_bool[48]{common.v1}else{(if v1788{(self.scalar_static_f64[258]+(self.scalar_static_f64[268]*(v1763-self.scalar_static_f64[261])))}else{(if v1781{(common.v1/v1784)}else{common.v3})})});
        let v1795=(v1661*v1794);
        let v1796=(v1689*v1794);
        let v1797=(v1460*v1794);
        let v1798=(v1740*v1794);
        let v1811=(common.v1243*common.v1810);
        let v1812=(v329/v1811);
        let v1814=(if (v1812<self.scalar_static_f64[16]){common.v1}else{common.v3});
        let v1816=(common.v178*(if (v1814!=0.0){self.scalar_static_f64[16]}else{v1812}));
        let v1817=((if common.v802{(common.v803*(common.v1+(common.v797-self.scalar_static_f64[201])))}else{(if (common.v799!=0.0){v800}else{common.v3})})-common.v1);
        let v1819=(common.v733+(common.v902*v1817));
        let v1820=(v1819/v1816);
        let v1855=(common.v1835&&(common.v1854!=0.0));
        let v1856=(common.v1852).exp();
        let v1864=(if common.v1859{(common.v1860*(common.v1+(common.v1852-self.scalar_static_f64[201])))}else{(if v1855{v1856}else{common.v3})});
        let v1866=(self.scalar_static_f64[274]/common.v441);
        let v1867=(common.v1848*v1866);
        let v1877=(((if (common.v722<common.v229){common.v1}else{common.v3})!=0.0)&&((self.scalar_static_f64[275]!=0.0)&&common.v1875));
        let v1883=(if v1877{self.scalar_static_f64[280]}else{common.v3});
        let v1884=(common.v229-common.v722);
        let v1886=(if v1877{(v1884/common.v1100)}else{common.v1009});
        let v1889=(((common.v33*v1886)/v1883)).sqrt();
        let v1890=(if v1877{v1889}else{common.v3});
        let v1894=(v1877&&(self.scalar_static_f64[282]!=0.0));
        let v1897=(v1877&&self.scalar_static_bool[53]);
        let v1900=(if v1897{(common.v1-(common.v431*common.v1094))}else{common.v3});
        let v1901=(self.scalar_static_f64[278]*v1900);
        let v1903=(if v1897{(v1900*v1901)}else{(if v1894{self.scalar_static_f64[278]}else{common.v3})});
        let v1904=(v1890*v1903);
        let v1908=(((v1890*v1890)+(v1903*v1903))).sqrt();
        let v1910=(if v1877{(v1904/v1908)}else{common.v3});
        let v1912=(if v1877{(v1884/v1910)}else{common.v3});
        let v1913=(common.v431*v1910);
        let v1914=(v1883*v1913);
        let v1917=(if v1877{(v1912+(common.v1100*v1914))}else{common.v3});
        let v1930=(self.scalar_static_f64[204]*(if v1897{(common.v1+(self.scalar_static_f64[284]*(common.v1+(common.v33*common.v1094))))}else{common.v3}));
        let v1932=((if v1897{self.scalar_static_f64[287]}else{common.v3})-(common.v1250/v1930));
        let v1935=(if v1897{(v1912-(v1914*v1932))}else{common.v3});
        let v1936=(v1935-v1917);
        let v1938=(common.v48*v1912);
        let v1939=(v1912*v1938);
        let v1945=((if v1897{((v1936*v1936)+((common.v1097*v1939)/self.scalar_static_f64[204]))}else{v1886})).sqrt();
        let v1948=(if v1897{(common.v431*((v1917+v1935)+v1945))}else{(if v1894{v1917}else{common.v3})});
        let v1949=(v1948-v1912);
        let v1951=(if v1877{(v1949/v1948)}else{common.v3});
        let v1955=(if ((v1951).abs()>1e-7){common.v1}else{common.v3});
        let v1956=(v1877&&(v1955!=0.0));
        let v1958=(if v1956{(v1913/v1951)}else{common.v3});
        let v1959=(self.scalar_static_f64[4]/v681);
        let v1960=(v1948*v1959);
        let v1961=(v1958*v1960);
        let v1962=(-v681);
        let v1963=(v1962/v1948);
        let v1964=(v1963).exp();
        let v1966=(common.v1+(v1903/v1958));
        let v1968=((v1963*v1966)).exp();
        let v1969=(v1964-v1968);
        let v1973=(v1877&&(!(v1955!=0.0)));
        let v1974=(self.scalar_static_f64[4]*v1903);
        let v2028=(common.v1982&&(common.v2027!=0.0));
        let v2029=(common.v2025).exp();
        let v2037=(if common.v2032{(common.v2033*(common.v1+(common.v2025-self.scalar_static_f64[201])))}else{(if v2028{v2029}else{v1864})});
        let v2038=(common.v1846*v1866);
        let v2040=(if common.v1982{(v2037*v2038)}else{(if v1973{(v1964*v1974)}else{(if v1956{(v1961*v1969)}else{(if common.v1835{(v1864*v1867)}else{common.v3})})})});
        let v2046=((common.v1822!=0.0)&&((if (v2040>common.v3){common.v1}else{common.v3})!=0.0));
        let v2047=((self.scalar_static_f64[295]!=0.0)&&v2046);
        let v2048=(v337+v1816);
        let v2049=(common.v1250*v2048);
        let v2051=(common.v1244/common.v456);
        let v2056=(if v2047{(((common.v125/v2049)+(v507*v2051))+(v322/v2048))}else{common.v3});
        let v2057=((self.scalar_static_f64[288]!=0.0)&&v2047);
        let v2060=(if v2057{((v2040-v2056)/common.v427)}else{common.v2002});
        let v2062=(if (v2040<v2056){common.v1}else{common.v3});
        let v2063=(v2057&&(v2062!=0.0));
        let v2064=(v2060).exp();
        let v2065=(common.v1+v2064);
        let v2071=(v2057&&(!(v2062!=0.0)));
        let v2073=((-v2060)).exp();
        let v2074=(common.v1+v2073);
        let v2078=(if v2071{(v2056-(common.v427*(v2074).ln()))}else{(if v2063{(v2040-(common.v427*(v2065).ln()))}else{v2040})});
        let v2079=(common.v1250*v2078);
        let v2082=(v2047&&self.scalar_static_bool[57]);
        let v2083=(v2056*v2079);
        let v2084=(v2056+v2078);
        let v2088=(v2046&&self.scalar_static_bool[58]);
        let v2089=(if v2088{v2079}else{(if v2082{(v2083/v2084)}else{(if v2057{v2079}else{common.v3})})});
        let v2091=(if (common.v1069>common.v3){common.v1}else{common.v3});
        let v2095=(!(v2091!=0.0));
        let v2096=(if v2095{common.v725}else{(if (v2091!=0.0){(common.v125*common.v2092)}else{common.v3})});
        let v2098=(if self.scalar_static_bool[30]{common.v725}else{(if (self.scalar_static_f64[150]!=0.0){common.v722}else{common.v3})});
        let v2099=(common.v728-v2096);
        let v2101=(v2096-common.v722);
        let v2106=(v738*v738);
        let v2109=(common.v759*common.v759);
        let v2112=(common.v752*common.v752);
        let v2115=(common.v749*common.v749);
        let v2118=(common.v741*common.v741);
        let v2128=((v578*v1280)+((v1302*v1304)+((((if self.scalar_static_bool[33]{(v507*v1381)}else{(if self.scalar_static_bool[31]{v1349}else{(if (self.scalar_static_f64[150]!=0.0){((v1349+(v1356*v1357))+(v1362/v1363))}else{common.v3})})})+(v481*v1433))+(common.v3*common.v728))-(if v1567{common.v3}else{(if (common.v1478!=0.0){(self.scalar_static_f64[21]*(common.v297*v1563))}else{common.v3})}))));
        let v2134=((v571*v1472)+((if self.scalar_static_bool[30]{v1411}else{(if (self.scalar_static_f64[150]!=0.0){(v1411+(v1413/v1417))}else{common.v3})})+(v562*v1446)));
        let v2138=(common.v3*common.v755);
        let v2139=((v1796+v1797)+v2138);
        let v2428=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2427);
        let v2449=(common.v1+(common.v105/self.scalar_static_f64[397]));
        let v2474=(if self.scalar_static_bool[79]{common.v3}else{(if (self.scalar_static_f64[322]!=0.0){((v2089/common.v2466)).abs()}else{common.v3})});
        let v2513=(self.scalar_static_f64[0]*v2134);
        let v2515=(self.scalar_static_f64[0]*v2128);
        let v2519=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v1795)));
        let v2522=(self.scalar_static_f64[0]*v1820);
        let v2526=(self.scalar_static_f64[0]*v738);
        let v2529=(self.scalar_static_f64[0]*common.v741);
        let v2535=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2534);
        let v2538=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2537);
        let v2541=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2540);
        let v2544=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2543);
        let v2548=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2547);
        let v2552=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2551);
        let v2556=(self.scalar_static_f64[0]*common.v759);
        let v2560=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2559);
        let v2566=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2565);
        let v2568=(self.scalar_static_f64[0]*common.v752);
        let v2572=(self.scalar_static_f64[0]*common.v749);
        let v2577=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2576);
        let v2601=(-(((common.v134*((common.v132*common.v2585)+(common.v122*(self.scalar_static_f64[23]*common.v2585))))-(common.v133*common.v2585))/(common.v134*common.v134)));
        let v2602=(v2601/common.v48);
        let v2612=(if common.v147{(v2601+(common.v48*((common.v149*(-v2602))/common.v150)))}else{(if (common.v140!=0.0){(common.v48*((common.v141*v2602)/common.v142))}else{common.v3})});
        let v2622=(-(((common.v157*((common.v155*common.v2585)+(common.v122*(self.scalar_static_f64[55]*common.v2585))))-(common.v156*common.v2585))/(common.v157*common.v157)));
        let v2623=(v2622/common.v48);
        let v2633=(if common.v170{(v2622+(common.v48*((common.v172*(-v2623))/common.v173)))}else{(if (common.v163!=0.0){(common.v48*((common.v164*v2623)/common.v165))}else{common.v3})});
        let v2757=((-common.v2728)/common.v2756);
        let v2765=((self.scalar_static_f64[49]*v2757)*(self.scalar_static_f64[50]*f64::powf(common.v301,self.scalar_static_f64[243])));
        let v2782=(if (v321!=0.0){common.v3}else{(self.scalar_static_f64[96]*(v318*(self.scalar_static_f64[97]*common.v2591)))});
        let v2789=(if (v336!=0.0){common.v3}else{(self.scalar_static_f64[102]*(v333*(self.scalar_static_f64[103]*common.v2591)))});
        let v2794=(v346*(self.scalar_static_f64[107]*common.v2591));
        let v2839=(common.v2837/(common.v33*common.v434));
        let v2848=(if common.v438{(common.v431*(common.v2835+v2839))}else{(if (common.v430!=0.0){((-(common.v432*(v2839-common.v2835)))/(common.v435*common.v435))}else{common.v3})});
        let v2875=(self.scalar_static_f64[135]*common.v2590);
        let v2890=(self.scalar_static_f64[143]*common.v2591);
        let v2894=(self.scalar_static_f64[146]*common.v2590);
        let v2899=((v506*(self.scalar_static_f64[141]*(v500*(v2890/self.scalar_static_f64[144]))))+(v501*(v506*(v2894/self.scalar_static_f64[144]))));
        let v2955=-1.5;
        let v2958=((self.scalar_static_f64[46]*v2612)*(common.v580*f64::powf(common.v579,v2955)));
        let v2977=(self.scalar_static_f64[46]*(self.scalar_static_f64[46]*((common.v588*common.v2754)+(common.v297*(self.scalar_static_f64[47]*((common.v586*common.v2961)+(common.v582*((common.v585*v2958)+(common.v581*((common.v584*v2612)+(common.v154*(self.scalar_static_f64[174]*v2612))))))))))));
        let v2998=((self.scalar_static_f64[78]*v2633)*(common.v580*f64::powf(common.v602,v2955)));
        let v3017=(self.scalar_static_f64[78]*(self.scalar_static_f64[78]*((common.v610*v2757)+(common.v298*(self.scalar_static_f64[49]*((common.v608*((-v2765)/(common.v302*common.v302)))+(common.v604*((common.v607*v2998)+(common.v603*((common.v606*v2633)+(common.v177*(self.scalar_static_f64[176]*v2633))))))))))));
        let v3073=(if v678{common.v3}else{(if (v668!=0.0){(self.scalar_static_f64[5]*((v669*common.v2585)-((v673*common.v2585)+(v665*(v672*common.v2585)))))}else{common.v3})});
        let v3080=(if self.scalar_static_bool[14]{common.v3}else{(if v695{common.v3}else{(if (self.scalar_static_f64[198]!=0.0){((-common.v2792)/(common.v342*common.v342))}else{common.v3})})});
        let v3086=(if self.scalar_static_bool[16]{common.v3}else{(if v705{common.v3}else{(if (self.scalar_static_f64[199]!=0.0){((-(self.scalar_static_f64[106]*v2794))/(v347*v347))}else{common.v3})})});
        let v3092=(if self.scalar_static_bool[18]{common.v3}else{(if v715{common.v3}else{(if (self.scalar_static_f64[200]!=0.0){((-(self.scalar_static_f64[108]*v2794))/(v349*v349))}else{common.v3})})});
        let v3154=(common.v733*common.v2590);
        let v4556=(((common.v1244*(common.v4545-common.v4539))-(common.v1249*common.v4523))/common.v4555);
        let v4560=((common.v4557-(common.v1249*common.v4526))/common.v4555);
        let v4564=(((common.v1244*(common.v4547-common.v4540))-(common.v1249*common.v4529))/common.v4555);
        let v4568=(((common.v1244*(-common.v4541))-(common.v1249*common.v4532))/common.v4555);
        let v4572=(((common.v1244*(-common.v4542))-(common.v1249*common.v4535))/common.v4555);
        let v4595=(common.v4593/self.scalar_static_f64[230]);
        let v4596=(common.v4594/self.scalar_static_f64[230]);
        let v4603=(if common.v1274{(common.v1275*v4595)}else{(if (common.v1271!=0.0){(v1272*v4595)}else{common.v3})});
        let v4604=(if common.v1274{(common.v1275*v4596)}else{(if (common.v1271!=0.0){(v1272*v4596)}else{common.v3})});
        let v4630=(if v1293{(-(common.v32*((v1295*self.scalar_static_f64[347])/v1296)))}else{(if (v1286!=0.0){(self.scalar_static_f64[331]-(common.v32*((v1287*self.scalar_static_f64[345])/v1288)))}else{common.v3})});
        let v4631=(if v1293{(-(common.v32*((v1295*self.scalar_static_f64[348])/v1296)))}else{(if (v1286!=0.0){(self.scalar_static_f64[0]-(common.v32*((v1287*self.scalar_static_f64[346])/v1288)))}else{common.v3})});
        let v4637=(common.v33*f64::powf(v1303,common.v1));
        let v4663=(common.v127*(-(if common.v289{((common.v293*common.v2587)+(common.v125*((common.v291*(-common.v2737))/common.v292)))}else{(if (common.v282!=0.0){(common.v2732+((common.v285*common.v2587)+(common.v125*((common.v283*common.v2737)/common.v284))))}else{common.v3})})));
        let v4664=((common.v1317*common.v2590)+v4663);
        let v4674=(if common.v1325{(common.v1326*v4664)}else{(if v1321{(v1322*v4664)}else{common.v3})});
        let v4675=(if common.v1325{(common.v1326*common.v3099)}else{(if v1321{(v1322*common.v3099)}else{v4595})});
        let v4676=(if common.v1325{(common.v1326*common.v3098)}else{(if v1321{(v1322*common.v3098)}else{v4596})});
        let v4680=(common.v456*common.v456);
        let v4681=(((common.v456*v4556)-(common.v1250*common.v2865))/v4680);
        let v4682=(v4560/common.v456);
        let v4683=(v4564/common.v456);
        let v4684=(v4568/common.v456);
        let v4685=(v4572/common.v456);
        let v4701=(if common.v1341{(common.v1343*v4681)}else{(if v1337{(v1338*v4681)}else{common.v3})});
        let v4702=(if common.v1341{(common.v1343*v4682)}else{(if v1337{(v1338*v4682)}else{v4603})});
        let v4703=(if common.v1341{(common.v1343*v4683)}else{(if v1337{(v1338*v4683)}else{v4604})});
        let v4704=(if common.v1341{(common.v1343*v4684)}else{(if v1337{(v1338*v4684)}else{common.v3})});
        let v4705=(if common.v1341{(common.v1343*v4685)}else{(if v1337{(v1338*v4685)}else{common.v3})});
        let v4708=((v1348*v2899)+(v507*common.v4658));
        let v4709=(v507*common.v4659);
        let v4710=(v507*common.v4660);
        let v4720=(common.v33*v1354);
        let v4727=(v1355*v1355);
        let v4771=(v1363*v1363);
        let v4840=(if self.scalar_static_bool[33]{(v507*((v1379*common.v4379)+(v1357*(self.scalar_static_f64[233]*common.v3946))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[150]!=0.0){((v1356*common.v4379)+(((v1363*((v1361*v4704)+(v1347*(v533*common.v3946))))-(v1362*v4704))/v4771))}else{common.v3})})});
        let v4841=(if self.scalar_static_bool[33]{(v507*((v1379*common.v4380)+(v1357*(self.scalar_static_f64[233]*common.v3947))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[150]!=0.0){((v1356*common.v4380)+(((v1363*((v1361*v4705)+(v1347*(v533*common.v3947))))-(v1362*v4705))/v4771))}else{common.v3})})});
        let v4861=(v4663+(common.v1396*common.v2590));
        let v4878=((v1410*((v514*(self.scalar_static_f64[147]*(v511*(v2890/self.scalar_static_f64[148]))))+(v512*(v514*(v2894/self.scalar_static_f64[148])))))+(v515*common.v4856));
        let v4879=(v515*common.v4857);
        let v4880=(v515*common.v4858);
        let v4881=(v515*common.v4859);
        let v4893=(common.v33*v1416);
        let v4901=(v1417*v1417);
        let v4948=(v481*common.v4942);
        let v5035=(v571*common.v5027);
        let v5036=(v571*common.v5028);
        let v5042=(common.v1479*common.v1479);
        let v5055=((common.v1481*v2977)+(common.v591*(-((-(self.scalar_static_f64[20]*(common.v33*common.v4086)))/v5042))));
        let v5056=(common.v591*(-((-(self.scalar_static_f64[20]*(common.v33*common.v4087)))/v5042)));
        let v5057=(common.v591*(-((-(self.scalar_static_f64[20]*(common.v33*common.v4088)))/v5042)));
        let v5073=(if (common.v1478!=0.0){(common.v728*common.v2754)}else{common.v3036});
        let v5074=(if (common.v1478!=0.0){(common.v297*self.scalar_static_f64[331])}else{common.v3});
        let v5075=(if (common.v1478!=0.0){(self.scalar_static_f64[0]*common.v297)}else{common.v3});
        let v5076=(common.v1496*v5073);
        let v5078=(common.v1496*v5074);
        let v5080=(common.v1496*v5075);
        let v5082=(common.v33*common.v1500);
        let v5088=(self.scalar_static_f64[236]*f64::powf(common.v1500,self.scalar_static_f64[349]));
        let v5156=(common.v1522*common.v1522);
        let v5166=(if (common.v1478!=0.0){(((common.v1522*(common.v1520*v2977))-(common.v1521*((common.v1519*v2612)+(common.v154*(if (common.v1478!=0.0){(common.v1517*((common.v1515*(((v5076+v5076)/v5082)*v5088))+(common.v1503*((self.scalar_static_f64[18]*(-(self.scalar_static_f64[239]*(common.v178*v5073))))-((common.v1513*((common.v1511*v5073)+(common.v1496*(common.v469*v5073))))+(common.v1512*v5073))))))}else{common.v3})))))/v5156)}else{v5073});
        let v5167=(if (common.v1478!=0.0){(((common.v1522*(common.v591*self.scalar_static_f64[350]))-(common.v1521*(common.v154*(if (common.v1478!=0.0){(common.v1517*((common.v1515*(((v5078+v5078)/v5082)*v5088))+(common.v1503*((self.scalar_static_f64[18]*(-(self.scalar_static_f64[239]*(common.v178*v5074))))-((common.v1513*((common.v1511*v5074)+(common.v1496*(common.v469*v5074))))+(common.v1512*v5074))))))}else{common.v3}))))/v5156)}else{v5074});
        let v5168=(if (common.v1478!=0.0){(((common.v1522*(common.v591*self.scalar_static_f64[351]))-(common.v1521*(common.v154*(if (common.v1478!=0.0){(common.v1517*((common.v1515*(((v5080+v5080)/v5082)*v5088))+(common.v1503*((self.scalar_static_f64[18]*(-(self.scalar_static_f64[239]*(common.v178*v5075))))-((common.v1513*((common.v1511*v5075)+(common.v1496*(common.v469*v5075))))+(common.v1512*v5075))))))}else{common.v3}))))/v5156)}else{v5075});
        let v5187=(common.v1524*common.v1524);
        let v5281=(common.v722*v2757);
        let v5282=(self.scalar_static_f64[0]*common.v298);
        let v5283=(common.v298*self.scalar_static_f64[331]);
        let v5288=(self.scalar_static_f64[227]*f64::powf(common.v1576,self.scalar_static_f64[340]));
        let v5292=(if (common.v1574!=0.0){((-v5281)*v5288)}else{common.v3});
        let v5293=(if (common.v1574!=0.0){((-v5282)*v5288)}else{common.v3});
        let v5294=(if (common.v1574!=0.0){((-v5283)*v5288)}else{common.v3});
        let v5300=(common.v1579*common.v1579);
        let v5313=((common.v1581*v3017)+(common.v613*(-((-(self.scalar_static_f64[52]*(common.v33*v5292)))/v5300))));
        let v5314=(common.v613*(-((-(self.scalar_static_f64[52]*(common.v33*v5293)))/v5300)));
        let v5315=(common.v613*(-((-(self.scalar_static_f64[52]*(common.v33*v5294)))/v5300)));
        let v5328=(if (common.v1574!=0.0){v5281}else{v2998});
        let v5329=(if (common.v1574!=0.0){v5282}else{common.v3});
        let v5330=(if (common.v1574!=0.0){v5283}else{common.v3});
        let v5331=(common.v1595*v5328);
        let v5333=(common.v1595*v5329);
        let v5335=(common.v1595*v5330);
        let v5337=(common.v33*common.v1598);
        let v5343=(self.scalar_static_f64[240]*f64::powf(common.v1598,self.scalar_static_f64[354]));
        let v5411=(common.v1618*common.v1618);
        let v5421=(if (common.v1574!=0.0){(((common.v1618*(common.v1616*v3017))-(common.v1617*((common.v1615*v2633)+(common.v177*(if (common.v1574!=0.0){(common.v1517*((common.v1612*(((v5331+v5331)/v5337)*v5343))+(common.v1600*((self.scalar_static_f64[50]*(-(self.scalar_static_f64[243]*(common.v178*v5328))))-((common.v1610*((common.v1608*v5328)+(common.v1595*(common.v469*v5328))))+(common.v1609*v5328))))))}else{common.v3})))))/v5411)}else{v5328});
        let v5422=(if (common.v1574!=0.0){(((common.v1618*(common.v613*self.scalar_static_f64[355]))-(common.v1617*(common.v177*(if (common.v1574!=0.0){(common.v1517*((common.v1612*(((v5333+v5333)/v5337)*v5343))+(common.v1600*((self.scalar_static_f64[50]*(-(self.scalar_static_f64[243]*(common.v178*v5329))))-((common.v1610*((common.v1608*v5329)+(common.v1595*(common.v469*v5329))))+(common.v1609*v5329))))))}else{common.v3}))))/v5411)}else{v5329});
        let v5423=(if (common.v1574!=0.0){(((common.v1618*(common.v613*self.scalar_static_f64[356]))-(common.v1617*(common.v177*(if (common.v1574!=0.0){(common.v1517*((common.v1612*(((v5335+v5335)/v5337)*v5343))+(common.v1600*((self.scalar_static_f64[50]*(-(self.scalar_static_f64[243]*(common.v178*v5330))))-((common.v1610*((common.v1608*v5330)+(common.v1595*(common.v469*v5330))))+(common.v1609*v5330))))))}else{common.v3}))))/v5411)}else{v5330});
        let v5442=(common.v1620*common.v1620);
        let v5622=(common.v33*v1680);
        let v5631=(v1681*v1681);
        let v5632=(((v1681*((v1674*common.v5601)+(common.v1673*common.v3149)))-(v1675*(((common.v1677*common.v3149)+(common.v796*common.v5614))/v5622)))/v5631);
        let v5636=(((v1681*(common.v1673*common.v3150))-(v1675*((common.v1677*common.v3150)/v5622)))/v5631);
        let v5640=(((v1681*(common.v1673*common.v3151))-(v1675*((common.v1677*common.v3151)/v5622)))/v5631);
        let v5644=(((v1681*(common.v1673*common.v3152))-(v1675*((common.v1677*common.v3152)/v5622)))/v5631);
        let v5648=(((v1681*(common.v1673*common.v3153))-(v1675*((common.v1677*common.v3153)/v5622)))/v5631);
        let v5897=(common.v1738*common.v5701);
        let v5909=(common.v1738*common.v5704);
        let v5934=(v1745*self.scalar_static_f64[361]);
        let v5936=(v1745*self.scalar_static_f64[362]);
        let v5938=(v1745*self.scalar_static_f64[363]);
        let v5950=(common.v33*v1755);
        let v5951=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5736})/v5950);
        let v5952=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5737})/v5950);
        let v5953=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5738})/v5950);
        let v5954=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5739})/v5950);
        let v5955=((if (self.scalar_static_f64[251]!=0.0){(v5934+v5934)}else{common.v5736})/v5950);
        let v5956=((if (self.scalar_static_f64[251]!=0.0){(v5936+v5936)}else{common.v5740})/v5950);
        let v5957=((if (self.scalar_static_f64[251]!=0.0){(v5938+v5938)}else{common.v5741})/v5950);
        let v5958=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5742})/v5950);
        let v5959=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5743})/v5950);
        let v5960=((if (self.scalar_static_f64[251]!=0.0){common.v3}else{common.v5744})/v5950);
        let v5966=(v1756*v1756);
        let v6018=(if v1760{(common.v431*v5951)}else{(if v1752{((-(self.scalar_static_f64[253]*v5951))/v5966)}else{common.v3})});
        let v6019=(if v1760{(common.v431*v5952)}else{(if v1752{((-(self.scalar_static_f64[253]*v5952))/v5966)}else{common.v3})});
        let v6020=(if v1760{(common.v431*v5953)}else{(if v1752{((-(self.scalar_static_f64[253]*v5953))/v5966)}else{common.v3})});
        let v6021=(if v1760{(common.v431*v5954)}else{(if v1752{((-(self.scalar_static_f64[253]*v5954))/v5966)}else{common.v3})});
        let v6022=(if v1760{(common.v431*(self.scalar_static_f64[364]+v5955))}else{(if v1752{((-(self.scalar_static_f64[253]*(v5955-self.scalar_static_f64[364])))/v5966)}else{common.v3})});
        let v6023=(if v1760{(common.v431*(self.scalar_static_f64[365]+v5956))}else{(if v1752{((-(self.scalar_static_f64[253]*(v5956-self.scalar_static_f64[365])))/v5966)}else{common.v3})});
        let v6024=(if v1760{(common.v431*(self.scalar_static_f64[366]+v5957))}else{(if v1752{((-(self.scalar_static_f64[253]*(v5957-self.scalar_static_f64[366])))/v5966)}else{common.v3})});
        let v6025=(if v1760{(common.v431*v5958)}else{(if v1752{((-(self.scalar_static_f64[253]*v5958))/v5966)}else{common.v3})});
        let v6026=(if v1760{(common.v431*v5959)}else{(if v1752{((-(self.scalar_static_f64[253]*v5959))/v5966)}else{common.v3})});
        let v6027=(if v1760{(common.v431*v5960)}else{(if v1752{((-(self.scalar_static_f64[253]*v5960))/v5966)}else{common.v3})});
        let v6039=(self.scalar_static_f64[254]*f64::powf(v1782,self.scalar_static_f64[263]));
        let v6050=(v1784*v1784);
        let v6091=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6018)}else{(if v1781{(((v6018/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6092=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6019)}else{(if v1781{(((v6019/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6093=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6020)}else{(if v1781{(((v6020/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6094=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6021)}else{(if v1781{(((v6021/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6095=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6022)}else{(if v1781{(((v6022/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6096=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6023)}else{(if v1781{(((v6023/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6097=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6024)}else{(if v1781{(((v6024/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6098=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6025)}else{(if v1781{(((v6025/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6099=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6026)}else{(if v1781{(((v6026/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6100=(if self.scalar_static_bool[48]{common.v3}else{(if v1788{(self.scalar_static_f64[268]*v6027)}else{(if v1781{(((v6027/self.scalar_static_f64[259])*v6039)/v6050)}else{common.v3})})});
        let v6101=(v1661*v6091);
        let v6102=(v1661*v6092);
        let v6105=((v1794*(if v1660{common.v3}else{(if (common.v1574!=0.0){(self.scalar_static_f64[53]*((v1656*v2757)+(common.v298*((v1655*(if common.v1589{(common.v1590*v5313)}else{(if v1585{(v1586*v5313)}else{common.v3})}))+(v1594*((v1654*v5292)+(common.v1578*((v1653*(if v1643{((v1650*(v1644*v5421))+(v1645*((v1648*(v1551*v5421))+(v1646*(v1553*v5421)))))}else{(if common.v1625{(v1636*(((common.v1620*(-(if common.v1630{(common.v1631*v5421)}else{(if v1626{(v1627*v5421)}else{common.v3})})))-(v1637*v5421))/v5442))}else{common.v3})}))+(v1652*(common.v33*((v622*((v619*v2765)+(common.v302*(self.scalar_static_f64[79]*(self.scalar_static_f64[79]*((v616*common.v2728)+(common.v272*((v615*common.v2728)+(common.v272*(self.scalar_static_f64[177]*v2998))))))))))+(v620*(v622*(-v3017))))))))))))))}else{common.v3})}))+(v1661*v6093));
        let v6106=(v1661*v6094);
        let v6107=(v1661*v6095);
        let v6110=((v1794*(if v1660{common.v3}else{(if (common.v1574!=0.0){(self.scalar_static_f64[53]*(common.v298*((v1655*(if common.v1589{(common.v1590*v5314)}else{(if v1585{(v1586*v5314)}else{common.v3})}))+(v1594*((v1654*v5293)+(common.v1578*(v1653*(if v1643{((v1650*((v1644*v5422)+(common.v1620*self.scalar_static_f64[353])))+(v1645*((v1648*(v1551*v5422))+(v1646*(v1553*v5422)))))}else{(if common.v1625{((v1639*self.scalar_static_f64[331])+(v1636*(((common.v1620*(-(if common.v1630{(common.v1631*v5422)}else{(if v1626{(v1627*v5422)}else{common.v3})})))-(v1637*v5422))/v5442)))}else{common.v3})}))))))))}else{common.v3})}))+(v1661*v6096));
        let v6113=((v1794*(if v1660{common.v3}else{(if (common.v1574!=0.0){(self.scalar_static_f64[53]*(common.v298*((v1655*(if common.v1589{(common.v1590*v5315)}else{(if v1585{(v1586*v5315)}else{common.v3})}))+(v1594*((v1654*v5294)+(common.v1578*(v1653*(if v1643{((v1650*((v1644*v5423)+(common.v1620*self.scalar_static_f64[352])))+(v1645*((v1648*(v1551*v5423))+(v1646*(v1553*v5423)))))}else{(if common.v1625{((self.scalar_static_f64[0]*v1639)+(v1636*(((common.v1620*(-(if common.v1630{(common.v1631*v5423)}else{(if v1626{(v1627*v5423)}else{common.v3})})))-(v1637*v5423))/v5442)))}else{common.v3})}))))))))}else{common.v3})}))+(v1661*v6097));
        let v6114=(v1661*v6098);
        let v6115=(v1661*v6099);
        let v6116=(v1661*v6100);
        let v6125=((v1794*(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v5636)}else{v5636}))+(v1689*v6095));
        let v6128=((v1794*(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v5640)}else{v5640}))+(v1689*v6096));
        let v6129=(v1794*(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v5644)}else{v5644}));
        let v6131=(v6129+(v1689*v6097));
        let v6133=(v6129+(v1689*v6098));
        let v6137=((v1794*(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v5648)}else{v5648}))+(v1689*v6100));
        let v6148=((v1794*(v493*common.v4996))+(v1460*v6095));
        let v6151=((v1794*(v493*common.v4997))+(v1460*v6096));
        let v6152=(v1794*(v493*common.v4998));
        let v6154=(v6152+(v1460*v6097));
        let v6156=(v6152+(v1460*v6098));
        let v6160=((v1794*(v493*common.v4999))+(v1460*v6100));
        let v6161=(v1794*(if (self.scalar_static_f64[245]!=0.0){(v5897+(common.v1699*common.v5888))}else{common.v3}));
        let v6163=(v6161+(v1740*v6091));
        let v6166=((v1794*(if (self.scalar_static_f64[245]!=0.0){((common.v1738*common.v5702)+(common.v1699*common.v5889))}else{common.v3}))+(v1740*v6092));
        let v6169=((v1794*(if (self.scalar_static_f64[245]!=0.0){((common.v1738*common.v5703)+(common.v1699*common.v5890))}else{common.v3}))+(v1740*v6093));
        let v6172=((v1794*(if (self.scalar_static_f64[245]!=0.0){(common.v1699*common.v5891)}else{common.v3}))+(v1740*v6094));
        let v6174=(v6161+(v1740*v6095));
        let v6177=((v1794*(if (self.scalar_static_f64[245]!=0.0){(v5897+(common.v1699*common.v5892))}else{common.v3}))+(v1740*v6096));
        let v6180=((v1794*(if (self.scalar_static_f64[245]!=0.0){(v5909+(common.v1699*common.v5893))}else{common.v3}))+(v1740*v6097));
        let v6183=((v1794*(if (self.scalar_static_f64[245]!=0.0){(v5909+(common.v1699*common.v5894))}else{common.v3}))+(v1740*v6098));
        let v6186=((v1794*(if (self.scalar_static_f64[245]!=0.0){((common.v1738*common.v5705)+(common.v1699*common.v5895))}else{common.v3}))+(v1740*v6099));
        let v6189=((v1794*(if (self.scalar_static_f64[245]!=0.0){(v5909+(common.v1699*common.v5896))}else{common.v3}))+(v1740*v6100));
        let v6265=(v1811*v1811);
        let v6284=(common.v178*(if (v1814!=0.0){common.v3}else{(((v1811*(self.scalar_static_f64[98]*(v328*(self.scalar_static_f64[101]*common.v2591))))-(v329*((common.v1810*common.v4516)+(common.v1243*common.v6242))))/v6265)}));
        let v6285=(common.v178*(if (v1814!=0.0){common.v3}else{((-(v329*((common.v1810*common.v4517)+(common.v1243*common.v6243))))/v6265)}));
        let v6286=(common.v178*(if (v1814!=0.0){common.v3}else{((-(v329*((common.v1810*common.v4518)+(common.v1243*common.v6244))))/v6265)}));
        let v6287=(common.v178*(if (v1814!=0.0){common.v3}else{((-(v329*((common.v1810*common.v4519)+(common.v1243*common.v6245))))/v6265)}));
        let v6288=(common.v178*(if (v1814!=0.0){common.v3}else{((-(v329*((common.v1810*common.v4520)+(common.v1243*common.v6246))))/v6265)}));
        let v6299=(v1816*v1816);
        let v6300=(((v1816*((v1817*common.v3341)+(common.v902*(if common.v802{(common.v803*v3154)}else{(if (common.v799!=0.0){(v800*v3154)}else{common.v3})}))))-(v1819*v6284))/v6299);
        let v6303=((-(v1819*v6285))/v6299);
        let v6304=((self.scalar_static_f64[0]+(common.v902*(if common.v802{(common.v803*common.v3098)}else{(if (common.v799!=0.0){(v800*common.v3098)}else{common.v3})})))/v1816);
        let v6308=(((v1816*(self.scalar_static_f64[331]+(common.v902*(if common.v802{(common.v803*common.v3099)}else{(if (common.v799!=0.0){(v800*common.v3099)}else{common.v3})}))))-(v1819*v6286))/v6299);
        let v6311=((-(v1819*v6287))/v6299);
        let v6314=((-(v1819*v6288))/v6299);
        let v6320=((-v4556)/self.scalar_static_f64[272]);
        let v6321=((-v4560)/self.scalar_static_f64[272]);
        let v6322=((-v4564)/self.scalar_static_f64[272]);
        let v6323=((-v4568)/self.scalar_static_f64[272]);
        let v6324=((-v4572)/self.scalar_static_f64[272]);
        let v6354=(if common.v1835{(common.v1846*(if common.v1840{(common.v1841*v6320)}else{(if common.v1836{(common.v1837*v6320)}else{common.v3})}))}else{common.v3});
        let v6355=(if common.v1835{(common.v1846*(if common.v1840{(common.v1841*v6321)}else{(if common.v1836{(common.v1837*v6321)}else{common.v3})}))}else{common.v3});
        let v6356=(if common.v1835{((common.v1846*(if common.v1840{(common.v1841*v6322)}else{(if common.v1836{(common.v1837*v6322)}else{common.v3})}))+(common.v1845*self.scalar_static_f64[331]))}else{common.v3});
        let v6357=(if common.v1835{((common.v1846*(if common.v1840{(common.v1841*v6323)}else{(if common.v1836{(common.v1837*v6323)}else{common.v3})}))+(self.scalar_static_f64[0]*common.v1845))}else{common.v3});
        let v6358=(if common.v1835{(common.v1846*(if common.v1840{(common.v1841*v6324)}else{(if common.v1836{(common.v1837*v6324)}else{common.v3})}))}else{common.v3});
        let v6359=(-v2848);
        let v6362=(self.scalar_static_f64[273]*f64::powf(common.v1848,self.scalar_static_f64[367]));
        let v6370=((common.v1851*v6359)+(common.v1849*(v6354*v6362)));
        let v6371=(common.v1849*(v6355*v6362));
        let v6372=(common.v1849*(v6356*v6362));
        let v6373=(common.v1849*(v6357*v6362));
        let v6374=(common.v1849*(v6358*v6362));
        let v6390=(if common.v1859{(common.v1860*v6370)}else{(if v1855{(v1856*v6370)}else{common.v3})});
        let v6391=(if common.v1859{(common.v1860*v6371)}else{(if v1855{(v1856*v6371)}else{common.v3})});
        let v6392=(if common.v1859{(common.v1860*v6372)}else{(if v1855{(v1856*v6372)}else{common.v3})});
        let v6393=(if common.v1859{(common.v1860*v6373)}else{(if v1855{(v1856*v6373)}else{common.v3})});
        let v6394=(if common.v1859{(common.v1860*v6374)}else{(if v1855{(v1856*v6374)}else{common.v3})});
        let v6398=((-(self.scalar_static_f64[274]*v2848))/(common.v441*common.v441));
        let v6429=(common.v1100*common.v1100);
        let v6442=(if v1877{(((common.v1100*common.v2684)-(v1884*common.v4024))/v6429)}else{common.v3733});
        let v6443=(if v1877{(((common.v1100*self.scalar_static_f64[331])-(v1884*common.v4025))/v6429)}else{common.v3734});
        let v6444=(if v1877{(((self.scalar_static_f64[0]*common.v1100)-(v1884*common.v4026))/v6429)}else{common.v3735});
        let v6445=(if v1877{((-(v1884*common.v4027))/v6429)}else{common.v3736});
        let v6454=(common.v33*v1889);
        let v6459=(if v1877{(((common.v33*v6442)/v1883)/v6454)}else{common.v3});
        let v6460=(if v1877{(((common.v33*v6443)/v1883)/v6454)}else{common.v3});
        let v6461=(if v1877{(((common.v33*v6444)/v1883)/v6454)}else{common.v3});
        let v6462=(if v1877{(((common.v33*v6445)/v1883)/v6454)}else{common.v3});
        let v6471=(if v1897{(-(common.v431*common.v4000))}else{common.v3});
        let v6472=(if v1897{(-(common.v431*common.v4001))}else{common.v3});
        let v6473=(if v1897{(-(common.v431*common.v4002))}else{common.v3});
        let v6474=(if v1897{(-(common.v431*common.v4003))}else{common.v3});
        let v6491=(if v1897{((v1901*v6471)+(v1900*(self.scalar_static_f64[278]*v6471)))}else{common.v3});
        let v6492=(if v1897{((v1901*v6472)+(v1900*(self.scalar_static_f64[278]*v6472)))}else{common.v3});
        let v6493=(if v1897{((v1901*v6473)+(v1900*(self.scalar_static_f64[278]*v6473)))}else{common.v3});
        let v6494=(if v1897{((v1901*v6474)+(v1900*(self.scalar_static_f64[278]*v6474)))}else{common.v3});
        let v6507=(v1890*v6459);
        let v6509=(v1890*v6460);
        let v6511=(v1890*v6461);
        let v6513=(v1890*v6462);
        let v6515=(v1903*v6491);
        let v6517=(v1903*v6492);
        let v6519=(v1903*v6493);
        let v6521=(v1903*v6494);
        let v6527=(common.v33*v1908);
        let v6535=(v1908*v1908);
        let v6549=(if v1877{(((v1908*((v1903*v6459)+(v1890*v6491)))-(v1904*(((v6507+v6507)+(v6515+v6515))/v6527)))/v6535)}else{common.v3});
        let v6550=(if v1877{(((v1908*((v1903*v6460)+(v1890*v6492)))-(v1904*(((v6509+v6509)+(v6517+v6517))/v6527)))/v6535)}else{common.v3});
        let v6551=(if v1877{(((v1908*((v1903*v6461)+(v1890*v6493)))-(v1904*(((v6511+v6511)+(v6519+v6519))/v6527)))/v6535)}else{common.v3});
        let v6552=(if v1877{(((v1908*((v1903*v6462)+(v1890*v6494)))-(v1904*(((v6513+v6513)+(v6521+v6521))/v6527)))/v6535)}else{common.v3});
        let v6556=(v1910*v1910);
        let v6569=(if v1877{(((v1910*common.v2684)-(v1884*v6549))/v6556)}else{common.v3});
        let v6570=(if v1877{(((v1910*self.scalar_static_f64[331])-(v1884*v6550))/v6556)}else{common.v3});
        let v6571=(if v1877{(((self.scalar_static_f64[0]*v1910)-(v1884*v6551))/v6556)}else{common.v3});
        let v6572=(if v1877{((-(v1884*v6552))/v6556)}else{common.v3});
        let v6573=(common.v431*v6549);
        let v6574=(common.v431*v6550);
        let v6575=(common.v431*v6551);
        let v6576=(common.v431*v6552);
        let v6577=(v1883*v6573);
        let v6578=(v1883*v6574);
        let v6579=(v1883*v6575);
        let v6580=(v1883*v6576);
        let v6597=(if v1877{(v6569+((v1914*common.v4024)+(common.v1100*v6577)))}else{common.v3});
        let v6598=(if v1877{(v6570+((v1914*common.v4025)+(common.v1100*v6578)))}else{common.v3});
        let v6599=(if v1877{(v6571+((v1914*common.v4026)+(common.v1100*v6579)))}else{common.v3});
        let v6600=(if v1877{(v6572+((v1914*common.v4027)+(common.v1100*v6580)))}else{common.v3});
        let v6624=(v1930*v1930);
        let v6662=(if v1897{(v6569-((v1932*v6577)+(v1914*(-(((v1930*v4556)-(common.v1250*(self.scalar_static_f64[204]*(if v1897{(self.scalar_static_f64[284]*(common.v33*common.v4000))}else{common.v3}))))/v6624)))))}else{common.v3});
        let v6663=(if v1897{(-(v1914*(-(v4560/v1930))))}else{common.v3});
        let v6664=(if v1897{(v6570-((v1932*v6578)+(v1914*(-(((v1930*v4564)-(common.v1250*(self.scalar_static_f64[204]*(if v1897{(self.scalar_static_f64[284]*(common.v33*common.v4001))}else{common.v3}))))/v6624)))))}else{common.v3});
        let v6665=(if v1897{(v6571-((v1932*v6579)+(v1914*(-(((v1930*v4568)-(common.v1250*(self.scalar_static_f64[204]*(if v1897{(self.scalar_static_f64[284]*(common.v33*common.v4002))}else{common.v3}))))/v6624)))))}else{common.v3});
        let v6666=(if v1897{(v6572-((v1932*v6580)+(v1914*(-(((v1930*v4572)-(common.v1250*(self.scalar_static_f64[204]*(if v1897{(self.scalar_static_f64[284]*(common.v33*common.v4003))}else{common.v3}))))/v6624)))))}else{common.v3});
        let v6671=(v1936*(v6662-v6597));
        let v6673=(v1936*v6663);
        let v6675=(v1936*(v6664-v6598));
        let v6677=(v1936*(v6665-v6599));
        let v6679=(v1936*(v6666-v6600));
        let v6726=(common.v33*v1945);
        let v6742=(if v1897{(common.v431*((v6597+v6662)+((if v1897{((v6671+v6671)+(((v1939*common.v4012)+(common.v1097*((v1938*v6569)+(v1912*(common.v48*v6569)))))/self.scalar_static_f64[204]))}else{v6442})/v6726)))}else{(if v1894{v6597}else{common.v3})});
        let v6743=(if v1897{(common.v431*(v6663+((if v1897{(v6673+v6673)}else{common.v3})/v6726)))}else{common.v3});
        let v6744=(if v1897{(common.v431*((v6598+v6664)+((if v1897{((v6675+v6675)+(((v1939*common.v4013)+(common.v1097*((v1938*v6570)+(v1912*(common.v48*v6570)))))/self.scalar_static_f64[204]))}else{v6443})/v6726)))}else{(if v1894{v6598}else{common.v3})});
        let v6745=(if v1897{(common.v431*((v6599+v6665)+((if v1897{((v6677+v6677)+(((v1939*common.v4014)+(common.v1097*((v1938*v6571)+(v1912*(common.v48*v6571)))))/self.scalar_static_f64[204]))}else{v6444})/v6726)))}else{(if v1894{v6599}else{common.v3})});
        let v6746=(if v1897{(common.v431*((v6600+v6666)+((if v1897{((v6679+v6679)+(((v1939*common.v4015)+(common.v1097*((v1938*v6572)+(v1912*(common.v48*v6572)))))/self.scalar_static_f64[204]))}else{v6445})/v6726)))}else{(if v1894{v6600}else{common.v3})});
        let v6754=(v1948*v1948);
        let v6780=(v1951*v1951);
        let v6797=(if v1956{(((v1951*v6573)-(v1913*(if v1877{(((v1948*(v6742-v6569))-(v1949*v6742))/v6754)}else{common.v3})))/v6780)}else{common.v3});
        let v6798=(if v1956{((-(v1913*(if v1877{(((v1948*v6743)-(v1949*v6743))/v6754)}else{common.v3})))/v6780)}else{common.v3});
        let v6799=(if v1956{(((v1951*v6574)-(v1913*(if v1877{(((v1948*(v6744-v6570))-(v1949*v6744))/v6754)}else{common.v3})))/v6780)}else{common.v3});
        let v6800=(if v1956{(((v1951*v6575)-(v1913*(if v1877{(((v1948*(v6745-v6571))-(v1949*v6745))/v6754)}else{common.v3})))/v6780)}else{common.v3});
        let v6801=(if v1956{(((v1951*v6576)-(v1913*(if v1877{(((v1948*(v6746-v6572))-(v1949*v6746))/v6754)}else{common.v3})))/v6780)}else{common.v3});
        let v6832=(((v1948*(-v3073))-(v1962*v6742))/v6754);
        let v6835=((-(v1962*v6743))/v6754);
        let v6838=((-(v1962*v6744))/v6754);
        let v6841=((-(v1962*v6745))/v6754);
        let v6844=((-(v1962*v6746))/v6754);
        let v6845=(v1964*v6832);
        let v6846=(v1964*v6835);
        let v6847=(v1964*v6838);
        let v6848=(v1964*v6841);
        let v6849=(v1964*v6844);
        let v6853=(v1958*v1958);
        let v6938=(self.scalar_static_f64[273]*f64::powf(common.v1846,self.scalar_static_f64[367]));
        let v6944=(common.v1985*common.v1985);
        let v6969=(self.scalar_static_f64[290]*f64::powf(common.v1987,self.scalar_static_f64[368]));
        let v6984=(if common.v1982{(common.v1983*((-(((common.v1985*v4556)-(common.v1250*v4556))/v6944))*v6969))}else{common.v3});
        let v6985=(if common.v1982{(common.v1983*((-(((common.v1985*v4560)-(common.v1250*v4560))/v6944))*v6969))}else{common.v3});
        let v6986=(if common.v1982{((common.v1989*(self.scalar_static_f64[331]*v6938))+(common.v1983*((-(((common.v1985*v4564)-(common.v1250*v4564))/v6944))*v6969)))}else{common.v3});
        let v6987=(if common.v1982{((common.v1989*(self.scalar_static_f64[0]*v6938))+(common.v1983*((-(((common.v1985*v4568)-(common.v1250*v4568))/v6944))*v6969)))}else{common.v3});
        let v6988=(if common.v1982{(common.v1983*((-(((common.v1985*v4572)-(common.v1250*v4572))/v6944))*v6969))}else{common.v3});
        let v6999=(if common.v1994{(v4556/self.scalar_static_f64[289])}else{common.v3});
        let v7000=(if common.v1994{(v4560/self.scalar_static_f64[289])}else{common.v3});
        let v7001=(if common.v1994{(v4564/self.scalar_static_f64[289])}else{common.v3});
        let v7002=(if common.v1994{(v4568/self.scalar_static_f64[289])}else{common.v3});
        let v7003=(if common.v1994{(v4572/self.scalar_static_f64[289])}else{common.v3});
        let v7009=(if common.v1994{(v6999/self.scalar_static_f64[292])}else{common.v3});
        let v7010=(if common.v1994{(v7000/self.scalar_static_f64[292])}else{self.scalar_static_f64[345]});
        let v7011=(if common.v1994{(v7001/self.scalar_static_f64[292])}else{self.scalar_static_f64[346]});
        let v7012=(if common.v1994{(v7002/self.scalar_static_f64[292])}else{common.v3});
        let v7013=(if common.v1994{(v7003/self.scalar_static_f64[292])}else{common.v3});
        let v7066=(self.scalar_static_f64[293]*f64::powf(common.v2020,self.scalar_static_f64[369]));
        let v7094=((common.v2024*v6359)+(common.v1849*(if common.v1994{((common.v2022*v6984)+(common.v1991*((if common.v2013{(v6999+(self.scalar_static_f64[292]*((common.v2015*(-v7009))/common.v2016)))}else{(if common.v2005{(self.scalar_static_f64[292]*((common.v2006*v7009)/common.v2007))}else{common.v3})})*v7066)))}else{(if common.v1992{v6984}else{common.v3})})));
        let v7095=(common.v1849*(if common.v1994{((common.v2022*v6985)+(common.v1991*((if common.v2013{(v7000+(self.scalar_static_f64[292]*((common.v2015*(-v7010))/common.v2016)))}else{(if common.v2005{(self.scalar_static_f64[292]*((common.v2006*v7010)/common.v2007))}else{common.v3})})*v7066)))}else{(if common.v1992{v6985}else{common.v3})}));
        let v7096=(common.v1849*(if common.v1994{((common.v2022*v6986)+(common.v1991*((if common.v2013{(v7001+(self.scalar_static_f64[292]*((common.v2015*(-v7011))/common.v2016)))}else{(if common.v2005{(self.scalar_static_f64[292]*((common.v2006*v7011)/common.v2007))}else{common.v3})})*v7066)))}else{(if common.v1992{v6986}else{common.v3})}));
        let v7097=(common.v1849*(if common.v1994{((common.v2022*v6987)+(common.v1991*((if common.v2013{(v7002+(self.scalar_static_f64[292]*((common.v2015*(-v7012))/common.v2016)))}else{(if common.v2005{(self.scalar_static_f64[292]*((common.v2006*v7012)/common.v2007))}else{common.v3})})*v7066)))}else{(if common.v1992{v6987}else{common.v3})}));
        let v7098=(common.v1849*(if common.v1994{((common.v2022*v6988)+(common.v1991*((if common.v2013{(v7003+(self.scalar_static_f64[292]*((common.v2015*(-v7013))/common.v2016)))}else{(if common.v2005{(self.scalar_static_f64[292]*((common.v2006*v7013)/common.v2007))}else{common.v3})})*v7066)))}else{(if common.v1992{v6988}else{common.v3})}));
        let v7133=(if common.v1982{((v2038*(if common.v2032{(common.v2033*v7094)}else{(if v2028{(v2029*v7094)}else{v6390})}))+(v2037*(common.v1846*v6398)))}else{(if v1973{((v1974*v6845)+(v1964*(self.scalar_static_f64[4]*v6491)))}else{(if v1956{((v1969*((v1960*v6797)+(v1958*((v1959*v6742)+(v1948*((-(self.scalar_static_f64[4]*v3073))/(v681*v681)))))))+(v1961*(v6845-(v1968*((v1966*v6832)+(v1963*(((v1958*v6491)-(v1903*v6797))/v6853)))))))}else{(if common.v1835{((v1867*v6390)+(v1864*((v1866*v6354)+(common.v1848*v6398))))}else{common.v3})})})});
        let v7134=(if common.v1982{(v2038*(if common.v2032{(common.v2033*v7095)}else{(if v2028{(v2029*v7095)}else{v6391})}))}else{(if v1973{(v1974*v6846)}else{(if v1956{((v1969*((v1960*v6798)+(v1958*(v1959*v6743))))+(v1961*(v6846-(v1968*((v1966*v6835)+(v1963*((-(v1903*v6798))/v6853)))))))}else{(if common.v1835{((v1867*v6391)+(v1864*(v1866*v6355)))}else{common.v3})})})});
        let v7135=(if common.v1982{((v2038*(if common.v2032{(common.v2033*v7096)}else{(if v2028{(v2029*v7096)}else{v6392})}))+(v2037*(v1866*self.scalar_static_f64[331])))}else{(if v1973{((v1974*v6847)+(v1964*(self.scalar_static_f64[4]*v6492)))}else{(if v1956{((v1969*((v1960*v6799)+(v1958*(v1959*v6744))))+(v1961*(v6847-(v1968*((v1966*v6838)+(v1963*(((v1958*v6492)-(v1903*v6799))/v6853)))))))}else{(if common.v1835{((v1867*v6392)+(v1864*(v1866*v6356)))}else{common.v3})})})});
        let v7136=(if common.v1982{((v2038*(if common.v2032{(common.v2033*v7097)}else{(if v2028{(v2029*v7097)}else{v6393})}))+(v2037*(self.scalar_static_f64[0]*v1866)))}else{(if v1973{((v1974*v6848)+(v1964*(self.scalar_static_f64[4]*v6493)))}else{(if v1956{((v1969*((v1960*v6800)+(v1958*(v1959*v6745))))+(v1961*(v6848-(v1968*((v1966*v6841)+(v1963*(((v1958*v6493)-(v1903*v6800))/v6853)))))))}else{(if common.v1835{((v1867*v6393)+(v1864*(v1866*v6357)))}else{common.v3})})})});
        let v7137=(if common.v1982{(v2038*(if common.v2032{(common.v2033*v7098)}else{(if v2028{(v2029*v7098)}else{v6394})}))}else{(if v1973{((v1974*v6849)+(v1964*(self.scalar_static_f64[4]*v6494)))}else{(if v1956{((v1969*((v1960*v6801)+(v1958*(v1959*v6746))))+(v1961*(v6849-(v1968*((v1966*v6844)+(v1963*(((v1958*v6494)-(v1903*v6801))/v6853)))))))}else{(if common.v1835{((v1867*v6394)+(v1864*(v1866*v6358)))}else{common.v3})})})});
        let v7138=(v2789+v6284);
        let v7157=(v2049*v2049);
        let v7194=(v2048*v2048);
        let v7213=(if v2047{(((((v2049*common.v2587)-(common.v125*((v2048*v4556)+(common.v1250*v7138))))/v7157)+((v2051*v2899)+(v507*(((common.v456*common.v4523)-(common.v1244*common.v2865))/v4680))))+(((v2048*v2782)-(v322*v7138))/v7194))}else{common.v3});
        let v7214=(if v2047{((((-(common.v125*((v2048*v4560)+(common.v1250*v6285))))/v7157)+(v507*(common.v4526/common.v456)))+((-(v322*v6285))/v7194))}else{common.v3});
        let v7215=(if v2047{((((-(common.v125*((v2048*v4564)+(common.v1250*v6286))))/v7157)+(v507*(common.v4529/common.v456)))+((-(v322*v6286))/v7194))}else{common.v3});
        let v7216=(if v2047{((((-(common.v125*((v2048*v4568)+(common.v1250*v6287))))/v7157)+(v507*(common.v4532/common.v456)))+((-(v322*v6287))/v7194))}else{common.v3});
        let v7217=(if v2047{((((-(common.v125*((v2048*v4572)+(common.v1250*v6288))))/v7157)+(v507*(common.v4535/common.v456)))+((-(v322*v6288))/v7194))}else{common.v3});
        let v7228=(if v2057{((v7133-v7213)/common.v427)}else{v7009});
        let v7229=(if v2057{((v7134-v7214)/common.v427)}else{v7010});
        let v7230=(if v2057{((v7135-v7215)/common.v427)}else{v7011});
        let v7231=(if v2057{((v7136-v7216)/common.v427)}else{v7012});
        let v7232=(if v2057{((v7137-v7217)/common.v427)}else{v7013});
        let v7283=(if v2071{(v7213-(common.v427*((v2073*(-v7228))/v2074)))}else{(if v2063{(v7133-(common.v427*((v2064*v7228)/v2065)))}else{v7133})});
        let v7284=(if v2071{(v7214-(common.v427*((v2073*(-v7229))/v2074)))}else{(if v2063{(v7134-(common.v427*((v2064*v7229)/v2065)))}else{v7134})});
        let v7285=(if v2071{(v7215-(common.v427*((v2073*(-v7230))/v2074)))}else{(if v2063{(v7135-(common.v427*((v2064*v7230)/v2065)))}else{v7135})});
        let v7286=(if v2071{(v7216-(common.v427*((v2073*(-v7231))/v2074)))}else{(if v2063{(v7136-(common.v427*((v2064*v7231)/v2065)))}else{v7136})});
        let v7287=(if v2071{(v7217-(common.v427*((v2073*(-v7232))/v2074)))}else{(if v2063{(v7137-(common.v427*((v2064*v7232)/v2065)))}else{v7137})});
        let v7290=((v2078*v4556)+(common.v1250*v7283));
        let v7293=((v2078*v4560)+(common.v1250*v7284));
        let v7296=((v2078*v4564)+(common.v1250*v7285));
        let v7299=((v2078*v4568)+(common.v1250*v7286));
        let v7302=((v2078*v4572)+(common.v1250*v7287));
        let v7331=(v2084*v2084);
        let v7354=(if v2088{v7290}else{(if v2082{(((v2084*((v2079*v7213)+(v2056*v7290)))-(v2083*(v7213+v7283)))/v7331)}else{(if v2057{v7290}else{common.v3})})});
        let v7355=(if v2088{v7293}else{(if v2082{(((v2084*((v2079*v7214)+(v2056*v7293)))-(v2083*(v7214+v7284)))/v7331)}else{(if v2057{v7293}else{common.v3})})});
        let v7356=(if v2088{v7296}else{(if v2082{(((v2084*((v2079*v7215)+(v2056*v7296)))-(v2083*(v7215+v7285)))/v7331)}else{(if v2057{v7296}else{common.v3})})});
        let v7357=(if v2088{v7299}else{(if v2082{(((v2084*((v2079*v7216)+(v2056*v7299)))-(v2083*(v7216+v7286)))/v7331)}else{(if v2057{v7299}else{common.v3})})});
        let v7358=(if v2088{v7302}else{(if v2082{(((v2084*((v2079*v7217)+(v2056*v7302)))-(v2083*(v7217+v7287)))/v7331)}else{(if v2057{v7302}else{common.v3})})});
        let v7373=(if v2095{common.v3}else{(if (v2091!=0.0){((common.v2092*common.v2587)+(common.v125*(common.v3944/common.v1069)))}else{common.v3})});
        let v7374=(if v2095{self.scalar_static_f64[0]}else{(if (v2091!=0.0){(common.v125*(common.v3945/common.v1069))}else{common.v3})});
        let v7375=(if v2095{common.v3}else{(if (v2091!=0.0){(common.v125*(common.v3946/common.v1069))}else{common.v3})});
        let v7376=(if v2095{self.scalar_static_f64[331]}else{(if (v2091!=0.0){(common.v125*(common.v3947/common.v1069))}else{common.v3})});
        let v7438=(v738*self.scalar_static_f64[331]);
        let v7443=(v322*v322);
        let v7449=(common.v759*self.scalar_static_f64[332]);
        let v7451=(common.v759*self.scalar_static_f64[333]);
        let v7453=(common.v759*self.scalar_static_f64[331]);
        let v7456=(v698*(v7449+v7449));
        let v7458=(v698*(v7451+v7451));
        let v7465=(common.v752*self.scalar_static_f64[331]);
        let v7473=(common.v749*self.scalar_static_f64[331]);
        let v7483=(common.v741*self.scalar_static_f64[331]);
        let v7488=(v337*v337);
        let v7516=(((if self.scalar_static_bool[33]{((v1381*v2899)+(v507*((self.scalar_static_f64[235]*common.v4658)+((v1379*common.v4377)+(v1357*(self.scalar_static_f64[233]*(common.v3944+common.v4658)))))))}else{(if self.scalar_static_bool[31]{v4708}else{(if (self.scalar_static_f64[150]!=0.0){((v4708+((v1357*(((v1355*((v1350*common.v4658)+(v1348*(common.v33*(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[151]*(v524*((self.scalar_static_f64[153]*common.v2590)/self.scalar_static_f64[144])))}else{common.v3})))))-(v1351*((common.v443*v4674)/v4720)))/v4727))+(v1356*common.v4377)))+(((v1363*((v1361*v4701)+(v1347*((v1360*(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[154]*(v531*(self.scalar_static_f64[156]*common.v2590)))}else{common.v3}))+(v533*common.v3944)))))-(v1362*v4701))/v4771))}else{common.v3})})})+((v1433*((v480*(self.scalar_static_f64[130]*(v474*(self.scalar_static_f64[133]*common.v2591))))+(v475*(v480*(v2875/self.scalar_static_f64[131])))))+(v481*common.v4940)))-(if v1567{common.v3}else{(if (common.v1478!=0.0){(self.scalar_static_f64[21]*((v1563*common.v2754)+(common.v297*((v1562*(if common.v1489{(common.v1490*v5055)}else{(if v1485{(v1486*v5055)}else{common.v3})}))+(v1494*((v1561*common.v4086)+(common.v1127*((v1560*(if v1548{((v1557*(v1549*v5166))+(v1550*((v1555*(v1551*v5166))+(v1552*(v1553*v5166)))))}else{(if common.v1530{(v1541*(((common.v1524*(-(if common.v1535{(common.v1536*v5166)}else{(if v1531{(v1532*v5166)}else{common.v3})})))-(v1542*v5166))/v5187))}else{common.v3})}))+(v1559*(common.v33*((v600*((v597*common.v2761)+(common.v300*(self.scalar_static_f64[48]*(self.scalar_static_f64[48]*((v594*common.v2661)+(common.v205*((v593*common.v2661)+(common.v205*(self.scalar_static_f64[175]*v2958))))))))))+(v598*(v600*(-v2977))))))))))))))}else{common.v3})}));
        let v7517=((((if self.scalar_static_bool[33]{(v507*((self.scalar_static_f64[235]*common.v4659)+(v1357*(self.scalar_static_f64[233]*common.v4659))))}else{(if self.scalar_static_bool[31]{v4709}else{(if (self.scalar_static_f64[150]!=0.0){((v4709+(v1357*(((v1355*(v1350*common.v4659))-(v1351*((common.v443*v4675)/v4720)))/v4727)))+(((v1363*(v1361*v4702))-(v1362*v4702))/v4771))}else{common.v3})})})+(v481*common.v4941))+self.scalar_static_f64[375])-(if v1567{common.v3}else{(if (common.v1478!=0.0){(self.scalar_static_f64[21]*(common.v297*((v1562*(if common.v1489{(common.v1490*v5056)}else{(if v1485{(v1486*v5056)}else{common.v3})}))+(v1494*((v1561*common.v4087)+(common.v1127*(v1560*(if v1548{((v1557*((v1549*v5167)+(common.v1524*self.scalar_static_f64[352])))+(v1550*((v1555*(v1551*v5167))+(v1552*(v1553*v5167)))))}else{(if common.v1530{((self.scalar_static_f64[0]*v1544)+(v1541*(((common.v1524*(-(if common.v1535{(common.v1536*v5167)}else{(if v1531{(v1532*v5167)}else{common.v3})})))-(v1542*v5167))/v5187)))}else{common.v3})}))))))))}else{common.v3})}));
        let v7518=((((if self.scalar_static_bool[33]{(v507*((self.scalar_static_f64[235]*common.v4660)+((v1379*common.v4378)+(v1357*(self.scalar_static_f64[233]*(common.v3945+common.v4660))))))}else{(if self.scalar_static_bool[31]{v4710}else{(if (self.scalar_static_f64[150]!=0.0){((v4710+((v1357*(((v1355*(v1350*common.v4660))-(v1351*((common.v443*v4676)/v4720)))/v4727))+(v1356*common.v4378)))+(((v1363*((v1361*v4703)+(v1347*(v533*common.v3945))))-(v1362*v4703))/v4771))}else{common.v3})})})+(v481*common.v4943))+self.scalar_static_f64[376])-(if v1567{common.v3}else{(if (common.v1478!=0.0){(self.scalar_static_f64[21]*(common.v297*((v1562*(if common.v1489{(common.v1490*v5057)}else{(if v1485{(v1486*v5057)}else{common.v3})}))+(v1494*((v1561*common.v4088)+(common.v1127*(v1560*(if v1548{((v1557*((v1549*v5168)+(common.v1524*self.scalar_static_f64[353])))+(v1550*((v1555*(v1551*v5168))+(v1552*(v1553*v5168)))))}else{(if common.v1530{((v1544*self.scalar_static_f64[331])+(v1541*(((common.v1524*(-(if common.v1535{(common.v1536*v5168)}else{(if v1531{(v1532*v5168)}else{common.v3})})))-(v1542*v5168))/v5187)))}else{common.v3})}))))))))}else{common.v3})}));
        let v7521=((v1280*((v577*(self.scalar_static_f64[172]*(common.v2586/(common.v33*v573))))+(v574*(v577*(self.scalar_static_f64[173]*common.v2585)))))+v7516);
        let v7522=((v578*v4603)+(((v1304*(self.scalar_static_f64[232]*v4630))+(v1302*((-v4630)*v4637)))+v7517));
        let v7523=((v578*v4604)+(((v1304*(self.scalar_static_f64[232]*v4631))+(v1302*((-v4631)*v4637)))+v7518));
        let v7569=(((v1472*((v570*(self.scalar_static_f64[169]*(v567*(self.scalar_static_f64[171]*common.v2591))))+(v568*(v570*(v2875/self.scalar_static_f64[170])))))+(v571*common.v5023))+((if self.scalar_static_bool[30]{v4878}else{(if (self.scalar_static_f64[150]!=0.0){(v4878+(((v1417*((v1412*common.v4856)+(v1410*(common.v33*(if (self.scalar_static_f64[150]!=0.0){(self.scalar_static_f64[157]*(v539*((self.scalar_static_f64[159]*common.v2590)/self.scalar_static_f64[148])))}else{common.v3})))))-(v1413*((common.v443*(if common.v1404{(common.v1405*v4861)}else{(if v1400{(v1401*v4861)}else{v4674})}))/v4893)))/v4901))}else{common.v3})})+((v1446*((v561*(self.scalar_static_f64[165]*(v558*(self.scalar_static_f64[168]*common.v2591))))+(v559*(v561*(v2875/self.scalar_static_f64[166])))))+(v562*common.v4963))));
        let v7570=((v571*common.v5024)+((if self.scalar_static_bool[30]{v4879}else{(if (self.scalar_static_f64[150]!=0.0){(v4879+(((v1417*(v1412*common.v4857))-(v1413*((common.v443*(if common.v1404{(common.v1405*common.v3099)}else{(if v1400{(v1401*common.v3099)}else{v4675})}))/v4893)))/v4901))}else{common.v3})})+(v562*common.v4964)));
        let v7571=((v571*common.v5025)+((if self.scalar_static_bool[30]{v4880}else{(if (self.scalar_static_f64[150]!=0.0){(v4880+(((v1417*(v1412*common.v4858))-(v1413*((common.v443*(if common.v1404{(common.v1405*common.v3098)}else{(if v1400{(v1401*common.v3098)}else{common.v3})}))/v4893)))/v4901))}else{common.v3})})+(v562*common.v4965)));
        let v7572=((v571*common.v5026)+((if self.scalar_static_bool[30]{v4881}else{(if (self.scalar_static_f64[150]!=0.0){(v4881+(((v1417*(v1412*common.v4859))-(v1413*((common.v443*(if common.v1404{common.v3}else{(if v1400{common.v3}else{v4676})}))/v4893)))/v4901))}else{common.v3})})+(v562*common.v4966)));
        let v7580=(common.v731*v5035);
        let v7589=((v1689*v6091)+(v1460*v6091));
        let v7590=((v1689*v6092)+(v1460*v6092));
        let v7591=(((v1794*(if (self.scalar_static_f64[245]!=0.0){(self.scalar_static_f64[7]*v5632)}else{v5632}))+(v1689*v6093))+((v1794*((v1459*((v492*(self.scalar_static_f64[136]*(v487*(self.scalar_static_f64[139]*common.v2591))))+(v488*(v492*((self.scalar_static_f64[140]*common.v2590)/self.scalar_static_f64[137])))))+(v493*common.v4994)))+(v1460*v6093)));
        let v7592=((v1689*v6094)+((v1794*(v493*common.v4995))+(v1460*v6094)));
        let v7597=((v1689*v6099)+(v1460*v6099));
        let v7616=(v2139*self.scalar_static_f64[333]);
        let v7635=(v1798*self.scalar_static_f64[332]);
        let v7647=(v1798*self.scalar_static_f64[333]);
        let v8765=ddt_scale;
        let v8974=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v5035));
        let v9008=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6101)));
        let v9009=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6102)));
        let v9010=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6105)));
        let v9011=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6106)));
        let v9012=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6107)));
        let v9013=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6110)));
        let v9014=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6113)));
        let v9015=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6114)));
        let v9016=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6115)));
        let v9017=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v6116)));
        let v9212=(self.scalar_static_f64[15]*(v698*self.scalar_static_f64[395]));
        let v9214=(self.scalar_static_f64[15]*(v698*self.scalar_static_f64[396]));
        let v9234=(self.scalar_static_f64[15]*(v8765*common.v9216));
        let v9281=(self.scalar_static_f64[15]*(v8765*common.v9271));

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v887))),
            [3, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v3331)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v3332)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v3333)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v3334))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v1250))),
            [3, 4, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4556)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4560)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4564)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4568)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4572))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2513)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7569)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7570)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7571)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7572)), v8974, v8974, (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v5036))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2515)),
            [3, 4, 5, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7521)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7522)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4948)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7523)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4840)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v4841))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if (self.scalar_static_f64[150]!=0.0){v2519}else{common.v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(if (self.scalar_static_f64[150]!=0.0){v9008}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9009}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9010}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9011}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9012}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9013}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9014}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9015}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9016}else{common.v3}), (if (self.scalar_static_f64[150]!=0.0){v9017}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[30]{v2519}else{common.v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[30]{v9008}else{common.v3}), (if self.scalar_static_bool[30]{v9009}else{common.v3}), (if self.scalar_static_bool[30]{v9010}else{common.v3}), (if self.scalar_static_bool[30]{v9011}else{common.v3}), (if self.scalar_static_bool[30]{v9012}else{common.v3}), (if self.scalar_static_bool[30]{v9013}else{common.v3}), (if self.scalar_static_bool[30]{v9014}else{common.v3}), (if self.scalar_static_bool[30]{v9015}else{common.v3}), (if self.scalar_static_bool[30]{v9016}else{common.v3}), (if self.scalar_static_bool[30]{v9017}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[15]*v2522)),
            [3, 4, 5, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6300)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6303)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6304)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6308)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6311)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6314))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v2089)))),
            [3, 4, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v7354))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v7355))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v7356))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v7357))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v7358)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*(v2526/v322))),
            2,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[389]/v322))),
            3,
            multiplicity * ((self.scalar_static_f64[15]*((-(v2526*v2782))/v7443))),
            4,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[390]/v322))),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((self.scalar_static_f64[15]*(v2529/v337))),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[389]/v337))),
            3,
            multiplicity * ((self.scalar_static_f64[15]*((-(v2529*v2789))/v7488))),
            5,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[390]/v337))),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[77]{(common.v105/self.scalar_static_f64[14])}else{(if self.scalar_static_bool[76]{(self.scalar_static_f64[405]*(f64::powf(v2449,self.scalar_static_f64[315])-common.v1))}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[402]*(v2449).ln())}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[15]*(common.v105/self.scalar_static_f64[400]))}else{common.v3})})})})),
            3,
            multiplicity * ((if self.scalar_static_bool[77]{self.scalar_static_f64[388]}else{(if self.scalar_static_bool[76]{(self.scalar_static_f64[405]*(self.scalar_static_f64[409]*(self.scalar_static_f64[315]*f64::powf(v2449,self.scalar_static_f64[387]))))}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[402]*(self.scalar_static_f64[409]/v2449))}else{self.scalar_static_f64[408]})})})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((self.scalar_static_f64[15]*v2428)),
            3,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[314]*v8765))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * ((self.scalar_static_f64[15]*(-((((((((((((((common.v1250*v2099)+(common.v887*v2101))-(v2089*v2096))+(v2106/v322))+(v698*v2109))+(v708*v2112))+(v718*v2115))+(v2118/v337))+(common.v733*v1820))+(common.v728*v2128))-(v1795*v2098))+(common.v731*v2134))+(common.v755*v2139))+(common.v760*v1798))))),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[(self.scalar_static_f64[15]*(-((((v698*(v2556+v2556))-(v2098*v6101))+(common.v755*v7589))+(v7635+(common.v760*v6163))))), (self.scalar_static_f64[15]*(-((((v7456+((v2529+v2529)/v337))-(v2098*v6102))+(common.v755*v7590))+((v1798*self.scalar_static_f64[334])+(common.v760*v6166))))), (self.scalar_static_f64[15]*(-((v2526+v2526)/v322))), (self.scalar_static_f64[15]*(-(((((((((((((((v2099*v4556)+(common.v1250*(-v7373)))+((v2101*common.v3331)+(common.v887*v7373)))-((v2096*v7354)+(v2089*v7373)))+((-(v2106*v2782))/v7443))+(v2109*v3080))+(v2112*v3086))+(v2115*v3092))+((-(v2118*v2789))/v7488))+(common.v733*v6300))+(common.v728*v7521))-(v2098*v6105))+(common.v731*v7569))+(common.v755*v7591))+(common.v760*v6169)))), (self.scalar_static_f64[15]*(-((((((((((v2099*v4560)+(common.v1250*self.scalar_static_f64[331]))-(v2096*v7355))+((v7438+v7438)/v322))+(common.v733*v6303))+((v2128*self.scalar_static_f64[331])+(common.v728*v7522)))-(v2098*v6106))+((v2134*self.scalar_static_f64[331])+(common.v731*v7570)))+(common.v755*v7592))+(common.v760*v6172)))), (self.scalar_static_f64[15]*(-(((((((v7456+((v7483+v7483)/v337))+(v2522+(common.v733*v6304)))+(common.v728*v4948))-(v2098*v6107))+(v2513+(common.v731*v7571)))+((self.scalar_static_f64[0]*v2139)+(common.v755*(self.scalar_static_f64[376]+(v6125+v6148)))))+(v7635+(common.v760*v6174))))), (self.scalar_static_f64[15]*(-(((((((((((v2099*v4564)+(common.v1250*(self.scalar_static_f64[0]-v7374)))+((v2101*common.v3332)+(common.v887*(v7374-self.scalar_static_f64[0]))))-((v2096*v7356)+(v2089*v7374)))+v7456)+((v1820*self.scalar_static_f64[331])+(common.v733*v6308)))+(v2515+(common.v728*v7523)))-((v2098*v6110)+(v1795*self.scalar_static_f64[372])))+(common.v731*v7572))+((v2139*self.scalar_static_f64[332])+(common.v755*((v6128+v6151)+self.scalar_static_f64[377]))))+(v7635+(common.v760*v6177))))), (self.scalar_static_f64[15]*(-((((((((((((v2099*v4568)+(common.v1250*(-v7375)))+((v2101*common.v3333)+(common.v887*(v7375-self.scalar_static_f64[331]))))-((v2096*v7357)+(v2089*v7375)))+v7458)+(v718*(v7473+v7473)))+(common.v733*v6311))+(common.v728*v4840))-((v2098*v6113)+(v1795*self.scalar_static_f64[373])))+v7580)+(v7616+(common.v755*((v6131+v6154)+self.scalar_static_f64[378]))))+(v7647+(common.v760*v6180))))), (self.scalar_static_f64[15]*(-(((((((((((v2099*v4572)+(common.v1250*(-v7376)))+((v2101*common.v3334)+(common.v887*v7376)))-((v2096*v7358)+(v2089*v7376)))+v7458)+(common.v733*v6314))+(common.v728*v4841))-((v2098*v6114)+(v1795*self.scalar_static_f64[374])))+v7580)+(v7616+(common.v755*((v6133+v6156)+self.scalar_static_f64[378]))))+(v7647+(common.v760*v6183))))), (self.scalar_static_f64[15]*(-(((((v698*(v7453+v7453))+(v708*(v2568+v2568)))-(v2098*v6115))+(common.v755*v7597))+((v1798*self.scalar_static_f64[331])+(common.v760*v6186))))), (self.scalar_static_f64[15]*(-((((((v7458+(v708*(v7465+v7465)))+(v718*(v2572+v2572)))-(v2098*v6116))+(common.v731*v5036))+((v2139*self.scalar_static_f64[331])+(common.v755*(self.scalar_static_f64[375]+(v6137+v6160)))))+(v7647+(common.v760*v6189)))))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2535)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v8765*common.v9100)), (self.scalar_static_f64[15]*(v8765*common.v9101)), (self.scalar_static_f64[15]*(v8765*common.v9102)), (self.scalar_static_f64[15]*(v8765*common.v9103)), (self.scalar_static_f64[15]*(v8765*common.v9104)), (self.scalar_static_f64[15]*(v8765*common.v9105)), (self.scalar_static_f64[15]*(v8765*common.v9106))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2538)),
            3,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9121))),
            4,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9122))),
            5,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9123))),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*v2541)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v8765*common.v9130)), (self.scalar_static_f64[15]*(v8765*common.v9131)), (self.scalar_static_f64[15]*(v8765*common.v9132)), (self.scalar_static_f64[15]*(v8765*common.v9133)), (self.scalar_static_f64[15]*(v8765*common.v9134)), (self.scalar_static_f64[15]*(v8765*common.v9135)), (self.scalar_static_f64[15]*(v8765*common.v9136))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[15]*v2544)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v8765*common.v9151)), (self.scalar_static_f64[15]*(v8765*common.v9152)), (self.scalar_static_f64[15]*(v8765*common.v9153)), (self.scalar_static_f64[15]*(v8765*common.v9154)), (self.scalar_static_f64[15]*(v8765*common.v9155)), (self.scalar_static_f64[15]*(v8765*common.v9156)), (self.scalar_static_f64[15]*(v8765*common.v9157))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[15]*v2548)),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[391]))),
            2,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[392]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[15]*v2552)),
            0,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[393]))),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[394]))),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v1798))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6163)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6166)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6169)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6172)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6174)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6177)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6180)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6183)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6186)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v6189))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*(v698*v2556))),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[15]*(v698*self.scalar_static_f64[389])), v9212, (self.scalar_static_f64[15]*(v2556*v3080)), v9212, v9212, v9214, v9214, (self.scalar_static_f64[15]*(v698*self.scalar_static_f64[390])), v9214],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*v2560)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v9234, (self.scalar_static_f64[15]*(v8765*common.v9217)), (self.scalar_static_f64[15]*(v8765*common.v9218)), (self.scalar_static_f64[15]*(v8765*common.v9219)), v9234, (self.scalar_static_f64[15]*(v8765*common.v9220)), (self.scalar_static_f64[15]*(v8765*common.v9221)), (self.scalar_static_f64[15]*(v8765*common.v9222)), (self.scalar_static_f64[15]*(v8765*common.v9223)), (self.scalar_static_f64[15]*(v8765*common.v9224))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1796+(v1797+v2138))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7589)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7590)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7591)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7592)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v6125+(v6148+self.scalar_static_f64[376])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v6128+(v6151+self.scalar_static_f64[377])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v6131+(v6154+self.scalar_static_f64[378])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v6133+(v6156+self.scalar_static_f64[378])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v7597)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v6137+(v6160+self.scalar_static_f64[375]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[15]*v2566)),
            [3, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v8765*common.v9268)), (self.scalar_static_f64[15]*(v8765*common.v9269)), (self.scalar_static_f64[15]*(v8765*common.v9270)), v9281, v9281, (self.scalar_static_f64[15]*(v8765*common.v9272))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(9),
            Some(10),
            multiplicity * ((if (self.scalar_static_f64[199]!=0.0){(self.scalar_static_f64[15]*(v708*v2568))}else{common.v3})),
            3,
            multiplicity * ((if (self.scalar_static_f64[199]!=0.0){(self.scalar_static_f64[15]*(v2568*v3086))}else{common.v3})),
            9,
            multiplicity * ((if (self.scalar_static_f64[199]!=0.0){(self.scalar_static_f64[15]*(v708*self.scalar_static_f64[389]))}else{common.v3})),
            10,
            multiplicity * ((if (self.scalar_static_f64[199]!=0.0){(self.scalar_static_f64[15]*(v708*self.scalar_static_f64[390]))}else{common.v3})),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v3,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(7),
            multiplicity * ((if (self.scalar_static_f64[200]!=0.0){(self.scalar_static_f64[15]*(v718*v2572))}else{common.v3})),
            3,
            multiplicity * ((if (self.scalar_static_f64[200]!=0.0){(self.scalar_static_f64[15]*(v2572*v3092))}else{common.v3})),
            7,
            multiplicity * ((if (self.scalar_static_f64[200]!=0.0){(self.scalar_static_f64[15]*(v718*self.scalar_static_f64[390]))}else{common.v3})),
            10,
            multiplicity * ((if (self.scalar_static_f64[200]!=0.0){(self.scalar_static_f64[15]*(v718*self.scalar_static_f64[389]))}else{common.v3})),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v3,
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (common.v3),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (common.v2576),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(4),
            multiplicity * ((common.v2498*v2577)),
            [3, 4, 5, 6, 7, 8, 10, 11],
            [(v2577*common.v8898), (v2577*common.v8899), (v2577*common.v8900), (v2577*common.v8901), (v2577*common.v8902), (v2577*common.v8903), (v2577*common.v8904), (common.v2498*v8765)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((v2474*common.v2576)),
            11,
            multiplicity * (v2474),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (common.v2576),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (common.v3),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v2428=0.0;
        let v2535=0.0;
        let v2538=0.0;
        let v2541=0.0;
        let v2544=0.0;
        let v2548=0.0;
        let v2552=0.0;
        let v2560=0.0;
        let v2566=0.0;
        let v2577=0.0;
        let v8765=1.0;
        let v9234=(self.scalar_static_f64[15]*(v8765*common.v9216));
        let v9281=(self.scalar_static_f64[15]*(v8765*common.v9271));

        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[314]*v8765))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v8765*common.v9100)), (self.scalar_static_f64[15]*(v8765*common.v9101)), (self.scalar_static_f64[15]*(v8765*common.v9102)), (self.scalar_static_f64[15]*(v8765*common.v9103)), (self.scalar_static_f64[15]*(v8765*common.v9104)), (self.scalar_static_f64[15]*(v8765*common.v9105)), (self.scalar_static_f64[15]*(v8765*common.v9106))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9121))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9122))),
            nodes[5],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*common.v9123))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v8765*common.v9130)), (self.scalar_static_f64[15]*(v8765*common.v9131)), (self.scalar_static_f64[15]*(v8765*common.v9132)), (self.scalar_static_f64[15]*(v8765*common.v9133)), (self.scalar_static_f64[15]*(v8765*common.v9134)), (self.scalar_static_f64[15]*(v8765*common.v9135)), (self.scalar_static_f64[15]*(v8765*common.v9136))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v8765*common.v9151)), (self.scalar_static_f64[15]*(v8765*common.v9152)), (self.scalar_static_f64[15]*(v8765*common.v9153)), (self.scalar_static_f64[15]*(v8765*common.v9154)), (self.scalar_static_f64[15]*(v8765*common.v9155)), (self.scalar_static_f64[15]*(v8765*common.v9156)), (self.scalar_static_f64[15]*(v8765*common.v9157))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[391]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[392]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[393]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v8765*self.scalar_static_f64[394]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[v9234, (self.scalar_static_f64[15]*(v8765*common.v9217)), (self.scalar_static_f64[15]*(v8765*common.v9218)), (self.scalar_static_f64[15]*(v8765*common.v9219)), v9234, (self.scalar_static_f64[15]*(v8765*common.v9220)), (self.scalar_static_f64[15]*(v8765*common.v9221)), (self.scalar_static_f64[15]*(v8765*common.v9222)), (self.scalar_static_f64[15]*(v8765*common.v9223)), (self.scalar_static_f64[15]*(v8765*common.v9224))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &[nodes[3], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v8765*common.v9268)), (self.scalar_static_f64[15]*(v8765*common.v9269)), (self.scalar_static_f64[15]*(v8765*common.v9270)), v9281, v9281, (self.scalar_static_f64[15]*(v8765*common.v9272))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]],
            &[(v2577*common.v8898), (v2577*common.v8899), (v2577*common.v8900), (v2577*common.v8901), (v2577*common.v8902), (v2577*common.v8903), (v2577*common.v8904), (common.v2498*v8765)],
            &[],
            &[],
            multiplicity,
        );
    }
}
