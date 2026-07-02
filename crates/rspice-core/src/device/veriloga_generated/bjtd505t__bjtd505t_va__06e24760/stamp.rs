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
    v1: f64,
    v4: f64,
    v30: f64,
    v31: f64,
    v46: f64,
    v101: f64,
    v116: f64,
    v117: f64,
    v119: f64,
    v121: f64,
    v123: f64,
    v124: f64,
    v125: f64,
    v126: f64,
    v127: f64,
    v128: f64,
    v133: bool,
    v134: f64,
    v135: f64,
    v140: bool,
    v142: f64,
    v143: f64,
    v147: f64,
    v148: f64,
    v149: f64,
    v150: f64,
    v155: bool,
    v156: f64,
    v157: f64,
    v162: bool,
    v164: f64,
    v165: f64,
    v169: f64,
    v170: f64,
    v196: f64,
    v219: f64,
    v260: f64,
    v269: bool,
    v270: f64,
    v271: f64,
    v272: f64,
    v276: bool,
    v278: f64,
    v279: f64,
    v280: f64,
    v284: f64,
    v285: f64,
    v287: f64,
    v288: f64,
    v289: f64,
    v327: f64,
    v408: f64,
    v410: bool,
    v411: f64,
    v412: f64,
    v414: f64,
    v415: f64,
    v418: bool,
    v421: f64,
    v423: f64,
    v436: f64,
    v449: f64,
    v558: f64,
    v559: f64,
    v560: f64,
    v561: f64,
    v563: f64,
    v564: f64,
    v565: f64,
    v567: f64,
    v570: f64,
    v581: f64,
    v582: f64,
    v583: f64,
    v585: f64,
    v586: f64,
    v587: f64,
    v589: f64,
    v592: f64,
    v694: f64,
    v697: f64,
    v698: f64,
    v700: f64,
    v703: f64,
    v705: f64,
    v708: f64,
    v713: f64,
    v721: f64,
    v724: f64,
    v727: f64,
    v731: f64,
    v732: f64,
    v765: f64,
    v766: f64,
    v767: bool,
    v770: bool,
    v771: f64,
    v849: f64,
    v862: f64,
    v967: f64,
    v1024: f64,
    v1048: f64,
    v1051: f64,
    v1054: f64,
    v1080: f64,
    v1156: f64,
    v1191: f64,
    v1192: f64,
    v1197: f64,
    v1198: f64,
    v1216: f64,
    v1217: bool,
    v1220: bool,
    v1221: f64,
    v1230: f64,
    v1260: f64,
    v1261: f64,
    v1262: f64,
    v1263: bool,
    v1268: bool,
    v1269: f64,
    v1276: f64,
    v1277: f64,
    v1278: bool,
    v1283: bool,
    v1285: f64,
    v1335: f64,
    v1336: f64,
    v1337: f64,
    v1338: bool,
    v1343: bool,
    v1344: f64,
    v1370: f64,
    v1382: f64,
    v1394: f64,
    v1406: f64,
    v1412: bool,
    v1413: f64,
    v1415: f64,
    v1416: f64,
    v1417: bool,
    v1422: bool,
    v1423: f64,
    v1429: f64,
    v1433: f64,
    v1436: f64,
    v1444: f64,
    v1445: f64,
    v1446: f64,
    v1448: f64,
    v1450: f64,
    v1452: f64,
    v1453: f64,
    v1454: f64,
    v1455: f64,
    v1457: f64,
    v1459: bool,
    v1460: bool,
    v1461: bool,
    v1466: bool,
    v1467: f64,
    v1504: bool,
    v1506: f64,
    v1508: f64,
    v1509: f64,
    v1511: f64,
    v1512: f64,
    v1513: bool,
    v1518: bool,
    v1519: f64,
    v1524: f64,
    v1527: f64,
    v1529: f64,
    v1537: f64,
    v1538: f64,
    v1539: f64,
    v1541: f64,
    v1544: f64,
    v1545: f64,
    v1546: f64,
    v1547: f64,
    v1549: f64,
    v1550: bool,
    v1551: bool,
    v1552: bool,
    v1557: bool,
    v1558: f64,
    v1600: f64,
    v1604: f64,
    v1625: f64,
    v1641: f64,
    v1662: f64,
    v1730: f64,
    v1740: bool,
    v1750: bool,
    v1751: bool,
    v1752: f64,
    v1755: bool,
    v1756: f64,
    v1760: f64,
    v1761: f64,
    v1763: f64,
    v1764: f64,
    v1766: f64,
    v1767: f64,
    v1768: bool,
    v1773: bool,
    v1774: f64,
    v1787: bool,
    v1891: bool,
    v1892: f64,
    v1894: f64,
    v1896: f64,
    v1898: f64,
    v1900: f64,
    v1901: bool,
    v1903: bool,
    v1911: f64,
    v1913: bool,
    v1914: f64,
    v1915: f64,
    v1921: bool,
    v1923: f64,
    v1924: f64,
    v1928: f64,
    v1930: f64,
    v1932: f64,
    v1933: f64,
    v1934: bool,
    v1939: bool,
    v1940: f64,
    v1995: f64,
    v2320: f64,
    v2356: f64,
    v2384: f64,
    v2420: f64,
    v2423: f64,
    v2426: f64,
    v2429: f64,
    v2433: f64,
    v2437: f64,
    v2445: f64,
    v2451: f64,
    v2462: f64,
    v2471: f64,
    v2472: f64,
    v2473: f64,
    v2476: f64,
    v2477: f64,
    v2547: f64,
    v2570: f64,
    v2614: f64,
    v2618: f64,
    v2623: f64,
    v2640: f64,
    v2642: f64,
    v2647: f64,
    v2678: f64,
    v2721: f64,
    v2723: f64,
    v2751: f64,
    v2847: f64,
    v2922: f64,
    v2984: f64,
    v2985: f64,
    v3035: f64,
    v3036: f64,
    v3037: f64,
    v3038: f64,
    v3039: f64,
    v3217: f64,
    v3218: f64,
    v3219: f64,
    v3220: f64,
    v3227: f64,
    v3619: f64,
    v3620: f64,
    v3621: f64,
    v3622: f64,
    v3830: f64,
    v3831: f64,
    v3832: f64,
    v3833: f64,
    v3886: f64,
    v3887: f64,
    v3888: f64,
    v3889: f64,
    v3898: f64,
    v3899: f64,
    v3900: f64,
    v3901: f64,
    v3910: f64,
    v3911: f64,
    v3912: f64,
    v3913: f64,
    v3972: f64,
    v3973: f64,
    v3974: f64,
    v4263: f64,
    v4264: f64,
    v4265: f64,
    v4266: f64,
    v4402: f64,
    v4403: f64,
    v4404: f64,
    v4405: f64,
    v4406: f64,
    v4409: f64,
    v4412: f64,
    v4415: f64,
    v4418: f64,
    v4421: f64,
    v4425: f64,
    v4426: f64,
    v4427: f64,
    v4428: f64,
    v4431: f64,
    v4433: f64,
    v4441: f64,
    v4443: f64,
    v4479: f64,
    v4480: f64,
    v4544: f64,
    v4545: f64,
    v4546: f64,
    v4742: f64,
    v4743: f64,
    v4744: f64,
    v4745: f64,
    v4826: f64,
    v4827: f64,
    v4828: f64,
    v4829: f64,
    v4849: f64,
    v4850: f64,
    v4851: f64,
    v4852: f64,
    v4880: f64,
    v4881: f64,
    v4882: f64,
    v4883: f64,
    v4884: f64,
    v4885: f64,
    v4909: f64,
    v4910: f64,
    v4911: f64,
    v4912: f64,
    v4913: f64,
    v4914: f64,
    v5487: f64,
    v5500: f64,
    v5587: f64,
    v5588: f64,
    v5589: f64,
    v5590: f64,
    v5591: f64,
    v5622: f64,
    v5623: f64,
    v5624: f64,
    v5625: f64,
    v5626: f64,
    v5627: f64,
    v5628: f64,
    v5629: f64,
    v5630: f64,
    v5774: f64,
    v5775: f64,
    v5776: f64,
    v5777: f64,
    v5778: f64,
    v5779: f64,
    v5780: f64,
    v5781: f64,
    v5782: f64,
    v6128: f64,
    v6129: f64,
    v6130: f64,
    v6131: f64,
    v6132: f64,
    v8784: f64,
    v8785: f64,
    v8786: f64,
    v8787: f64,
    v8788: f64,
    v8789: f64,
    v8790: f64,
    v8986: f64,
    v8987: f64,
    v8988: f64,
    v8989: f64,
    v8990: f64,
    v8991: f64,
    v8992: f64,
    v9007: f64,
    v9008: f64,
    v9009: f64,
    v9016: f64,
    v9017: f64,
    v9018: f64,
    v9019: f64,
    v9020: f64,
    v9021: f64,
    v9022: f64,
    v9037: f64,
    v9038: f64,
    v9039: f64,
    v9040: f64,
    v9041: f64,
    v9042: f64,
    v9043: f64,
    v9102: f64,
    v9103: f64,
    v9104: f64,
    v9105: f64,
    v9106: f64,
    v9107: f64,
    v9108: f64,
    v9109: f64,
    v9110: f64,
    v9154: f64,
    v9155: f64,
    v9156: f64,
    v9157: f64,
    v9158: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=1.0;
        let v4=0.0;
        let v30=0.001;
        let v31=2.0;
        let v44=0.05;
        let v46=0.1;
        let v101=ctx.node_voltage(nodes[3]);
        let v102=(v101<v4);
        let v103=(v1-v101);
        let v106=(if v102{(-(v103).ln())}else{v101});
        let v108=(v106<self.scalar_static_f64[79]);
        let v110=(!v108);
        let v112=(v1+(v106-self.scalar_static_f64[79]));
        let v116=(self.scalar_static_f64[365]+(if v110{(self.scalar_static_f64[79]+(v112).ln())}else{(if v108{v106}else{v4})}));
        let v117=(v116/self.scalar_static_f64[8]);
        let v118=8.617086918058125e-5;
        let v119=(v116*v118);
        let v121=(v1/v119);
        let v123=(v121-self.scalar_static_f64[81]);
        let v124=(v116-self.scalar_static_f64[8]);
        let v125=(v117).ln();
        let v126=(self.scalar_static_f64[21]*v116);
        let v127=(v116*v126);
        let v128=(self.scalar_static_f64[24]+v116);
        let v130=(self.scalar_static_f64[42]-(v127/v128));
        let v132=((v130-v44)/v46);
        let v133=(v130<v44);
        let v134=(v132).exp();
        let v135=(v1+v134);
        let v140=(!v133);
        let v142=((-v132)).exp();
        let v143=(v1+v142);
        let v147=(if v140{(v130+(v46*(v143).ln()))}else{(if v133{(v44+(v46*(v135).ln()))}else{v4})});
        let v148=(self.scalar_static_f64[52]*v116);
        let v149=(v116*v148);
        let v150=(self.scalar_static_f64[55]+v116);
        let v152=(self.scalar_static_f64[73]-(v149/v150));
        let v154=((v152-v44)/v46);
        let v155=(v152<v44);
        let v156=(v154).exp();
        let v157=(v1+v156);
        let v162=(!v155);
        let v164=((-v154)).exp();
        let v165=(v1+v164);
        let v169=(if v162{(v152+(v46*(v165).ln()))}else{(if v155{(v44+(v46*(v157).ln()))}else{v4})});
        let v170=3.0;
        let v171=-3.0;
        let v172=(v119*v171);
        let v173=(v125*v172);
        let v176=(v1-v117);
        let v179=((v173+(self.scalar_static_f64[44]*v117))+(v176*self.scalar_static_f64[82]));
        let v180=(v44-v179);
        let v181=(v180/v119);
        let v182=(v44<v179);
        let v183=(v181).exp();
        let v184=(v1+v183);
        let v185=(v184).ln();
        let v189=(!v182);
        let v191=((-v181)).exp();
        let v192=(v1+v191);
        let v193=(v192).ln();
        let v196=(if v189{(v44+(v119*v193))}else{(if v182{(v179+(v119*v185))}else{v4})});
        let v201=(v176*self.scalar_static_f64[84]);
        let v202=((v173+(v117*self.scalar_static_f64[83]))+v201);
        let v203=(v44-v202);
        let v204=(v203/v119);
        let v205=(v44<v202);
        let v206=(v204).exp();
        let v207=(v1+v206);
        let v208=(v207).ln();
        let v212=(!v205);
        let v214=((-v204)).exp();
        let v215=(v1+v214);
        let v216=(v215).ln();
        let v219=(if v212{(v44+(v119*v216))}else{(if v205{(v202+(v119*v208))}else{v4})});
        let v223=(v201+(v173+(v117*self.scalar_static_f64[85])));
        let v224=(v44-v223);
        let v225=(v224/v119);
        let v226=(v44<v223);
        let v227=(v225).exp();
        let v228=(v1+v227);
        let v229=(v228).ln();
        let v233=(!v226);
        let v235=((-v225)).exp();
        let v236=(v1+v235);
        let v237=(v236).ln();
        let v240=(if v233{(v44+(v119*v237))}else{(if v226{(v223+(v119*v229))}else{v4})});
        let v243=(v201+(v173+(self.scalar_static_f64[46]*v117)));
        let v244=(v44-v243);
        let v245=(v244/v119);
        let v246=(v44<v243);
        let v247=(v245).exp();
        let v248=(v1+v247);
        let v249=(v248).ln();
        let v253=(!v246);
        let v255=((-v245)).exp();
        let v256=(v1+v255);
        let v257=(v256).ln();
        let v260=(if v253{(v44+(v119*v257))}else{(if v246{(v243+(v119*v249))}else{v4})});
        let v266=((v173+(v117*self.scalar_static_f64[86]))+(v176*self.scalar_static_f64[87]));
        let v267=(v44-v266);
        let v268=(v267/v119);
        let v269=(v44<v266);
        let v270=(v268).exp();
        let v271=(v1+v270);
        let v272=(v271).ln();
        let v276=(!v269);
        let v278=((-v268)).exp();
        let v279=(v1+v278);
        let v280=(v279).ln();
        let v283=(if v276{(v44+(v119*v280))}else{(if v269{(v266+(v119*v272))}else{v4})});
        let v284=(v1/v196);
        let v285=(v1/v260);
        let v286=(self.scalar_static_f64[44]*v284);
        let v287=f64::powf(v286,self.scalar_static_f64[16]);
        let v288=(self.scalar_static_f64[46]*v285);
        let v289=f64::powf(v288,self.scalar_static_f64[47]);
        let v291=(v287*self.scalar_static_f64[88]);
        let v294=(self.scalar_static_f64[46]/v260);
        let v297=(self.scalar_static_f64[89]+(self.scalar_static_f64[90]*f64::powf(v294,self.scalar_static_f64[47])));
        let v298=(v1/v297);
        let v300=(v297*self.scalar_static_f64[91]);
        let v301=(self.scalar_static_f64[89]*v298);
        let v326=((v125*self.scalar_static_f64[101])).exp();
        let v327=(self.scalar_static_f64[100]*v326);
        let v338=((v125*self.scalar_static_f64[106])).exp();
        let v339=(self.scalar_static_f64[105]*v338);
        let v346=(if self.scalar_static_bool[8]{(self.scalar_static_f64[108]*(v1+(v124*self.scalar_static_f64[107])))}else{v4});
        let v349=(if self.scalar_static_bool[8]{((v346-v1)/v30)}else{v268});
        let v350=(v346<v1);
        let v351=(self.scalar_static_bool[8]&&v350);
        let v352=(v349).exp();
        let v353=(v1+v352);
        let v357=(if v351{(v1+(v30*(v353).ln()))}else{v346});
        let v359=(self.scalar_static_bool[8]&&(!v350));
        let v361=((-v349)).exp();
        let v362=(v1+v361);
        let v367=0.0006931471805599453;
        let v371=(if self.scalar_static_bool[9]{self.scalar_static_f64[108]}else{(if self.scalar_static_bool[8]{((if v359{(v357+(v30*(v362).ln()))}else{v357})-v367)}else{v4})});
        let v378=(if self.scalar_static_bool[10]{(self.scalar_static_f64[110]*(v1+(v124*self.scalar_static_f64[109])))}else{v4});
        let v381=(if self.scalar_static_bool[10]{((v378-v1)/v30)}else{v349});
        let v382=(v378<v1);
        let v383=(self.scalar_static_bool[10]&&v382);
        let v384=(v381).exp();
        let v385=(v1+v384);
        let v389=(if v383{(v1+(v30*(v385).ln()))}else{v378});
        let v391=(self.scalar_static_bool[10]&&(!v382));
        let v393=((-v381)).exp();
        let v394=(v1+v393);
        let v402=(if self.scalar_static_bool[11]{self.scalar_static_f64[110]}else{(if self.scalar_static_bool[10]{((if v391{(v389+(v30*(v394).ln()))}else{v389})-v367)}else{v4})});
        let v407=(self.scalar_static_f64[111]*(v1+(v124*self.scalar_static_f64[112])));
        let v408=1e-6;
        let v409=(v407*v407);
        let v410=(v407<v4);
        let v411=0.5;
        let v412=5e-7;
        let v414=((v408+v409)).sqrt();
        let v415=(v414-v407);
        let v418=(!v410);
        let v421=(if v418{(v411*(v407+v414))}else{(if v410{(v412/v415)}else{v4})});
        let v423=4.0;
        let v428=(v125*self.scalar_static_f64[117]);
        let v430=((v428/v371)).exp();
        let v431=(self.scalar_static_f64[113]*v430);
        let v433=(v123*self.scalar_static_f64[118]);
        let v435=((v433/v371)).exp();
        let v436=(v431*v435);
        let v440=((v125*self.scalar_static_f64[120])).exp();
        let v441=(self.scalar_static_f64[119]*v440);
        let v446=((v125*self.scalar_static_f64[123])).exp();
        let v447=(self.scalar_static_f64[121]*v446);
        let v449=6.0;
        let v525=((v125*self.scalar_static_f64[155])).exp();
        let v526=(self.scalar_static_f64[153]*v525);
        let v530=((v123*self.scalar_static_f64[157])).exp();
        let v531=(v526*v530);
        let v558=(self.scalar_static_f64[43]*v147);
        let v559=-0.5;
        let v560=f64::powf(v558,v559);
        let v561=(v1/v287);
        let v563=(v147*self.scalar_static_f64[167]);
        let v564=(v147*v563);
        let v565=(v560*v564);
        let v567=(self.scalar_static_f64[44]*(v561*v565));
        let v570=(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*(v284*v567)));
        let v581=(self.scalar_static_f64[74]*v169);
        let v582=f64::powf(v581,v559);
        let v583=(v1/v289);
        let v585=(v169*self.scalar_static_f64[169]);
        let v586=(v169*v585);
        let v587=(v582*v586);
        let v589=(self.scalar_static_f64[46]*(v583*v587));
        let v592=(self.scalar_static_f64[74]*(self.scalar_static_f64[74]*(v285*v589)));
        let v604=((v125*self.scalar_static_f64[96])).exp();
        let v606=(v604*self.scalar_static_f64[171]);
        let v607=(v298*v606);
        let v609=(v604*self.scalar_static_f64[172]);
        let v610=(v561*v609);
        let v614=((v125*self.scalar_static_f64[174])).exp();
        let v615=(self.scalar_static_f64[173]*v614);
        let v619=((v123*self.scalar_static_f64[176])).exp();
        let v620=(v615*v619);
        let v625=((v125*self.scalar_static_f64[179])).exp();
        let v626=(self.scalar_static_f64[177]*v625);
        let v630=((v125*self.scalar_static_f64[181])).exp();
        let v631=(self.scalar_static_f64[180]*v630);
        let v633=(v626+v631);
        let v636=((self.scalar_static_f64[182]*v633)/self.scalar_static_f64[183]);
        let v641=((v125*self.scalar_static_f64[186])).exp();
        let v642=(self.scalar_static_f64[184]*v641);
        let v661=(v604*self.scalar_static_f64[188]);
        let v691=ctx.node_voltage(nodes[6]);
        let v692=ctx.node_voltage(nodes[7]);
        let v694=(self.scalar_static_f64[0]*(v691-v692));
        let v695=ctx.node_voltage(nodes[8]);
        let v697=(self.scalar_static_f64[0]*(v691-v695));
        let v698=ctx.node_voltage(nodes[4]);
        let v700=(self.scalar_static_f64[0]*(v691-v698));
        let v701=ctx.node_voltage(nodes[5]);
        let v703=(self.scalar_static_f64[0]*(v701-v698));
        let v705=(self.scalar_static_f64[0]*(v701-v691));
        let v707=(self.scalar_static_f64[0]*(v692-v695));
        let v708=ctx.node_voltage(nodes[2]);
        let v711=ctx.node_voltage(nodes[1]);
        let v713=(self.scalar_static_f64[0]*(v711-v701));
        let v718=(self.scalar_static_f64[0]*(v711-ctx.node_voltage(nodes[0])));
        let v719=ctx.node_voltage(nodes[10]);
        let v721=(self.scalar_static_f64[0]*(v719-v692));
        let v724=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[9])-v719));
        let v727=(((v697+v705)-v707)-v721);
        let v731=((v727+(v713+(-v718)))-v724);
        let v732=(v718+v731);
        let v733=(v121*v697);
        let v735=(v733<self.scalar_static_f64[191]);
        let v736=(v733).exp();
        let v738=(!v735);
        let v740=(if v738{self.scalar_static_f64[192]}else{v4});
        let v745=(v121*v700);
        let v746=(v745/v371);
        let v747=(v746<self.scalar_static_f64[191]);
        let v748=(v746).exp();
        let v750=(!v747);
        let v751=(if v750{self.scalar_static_f64[192]}else{v740});
        let v755=(if v750{(v751*(v1+(v746-self.scalar_static_f64[191])))}else{(if v747{v748}else{v4})});
        let v756=(v121*v727);
        let v757=(v756<self.scalar_static_f64[191]);
        let v758=(v756).exp();
        let v760=(!v757);
        let v761=(if v760{self.scalar_static_f64[192]}else{v751});
        let v765=(if v760{(v761*(v1+(v756-self.scalar_static_f64[191])))}else{(if v757{v758}else{v4})});
        let v766=(v121*v705);
        let v767=(v766<self.scalar_static_f64[191]);
        let v770=(!v767);
        let v771=(if v770{self.scalar_static_f64[192]}else{v761});
        let v776=(v121*v732);
        let v777=(v776<self.scalar_static_f64[191]);
        let v778=(v776).exp();
        let v780=(!v777);
        let v781=(if v780{self.scalar_static_f64[192]}else{v771});
        let v785=(if v780{(v781*(v1+(v776-self.scalar_static_f64[191])))}else{(if v777{v778}else{v4})});
        let v786=(v732-v219);
        let v787=(v121*v786);
        let v788=(v787<self.scalar_static_f64[191]);
        let v789=(v787).exp();
        let v791=(!v788);
        let v792=(if v791{self.scalar_static_f64[192]}else{v781});
        let v797=(v727-v219);
        let v798=(v121*v797);
        let v799=(v798<self.scalar_static_f64[191]);
        let v800=(v798).exp();
        let v802=(!v799);
        let v803=(if v802{self.scalar_static_f64[192]}else{v792});
        let v808=(v697-v219);
        let v809=(v121*v808);
        let v810=(v809<self.scalar_static_f64[191]);
        let v811=(v809).exp();
        let v813=(!v810);
        let v814=(if v813{self.scalar_static_f64[192]}else{v803});
        let v818=(if v813{(v814*(v1+(v809-self.scalar_static_f64[191])))}else{(if v810{v811}else{v4})});
        let v819=(v694-v219);
        let v820=(v121*v819);
        let v821=(v820<self.scalar_static_f64[191]);
        let v822=(v820).exp();
        let v824=(!v821);
        let v825=(if v824{self.scalar_static_f64[192]}else{v814});
        let v829=(if v824{(v825*(v1+(v820-self.scalar_static_f64[191])))}else{(if v821{v822}else{v4})});
        let v832=((v1+(v423*v818))).sqrt();
        let v835=((v1+(v423*v829))).sqrt();
        let v836=(v31*v829);
        let v837=(v1+v835);
        let v838=(v836/v837);
        let v840=(v838<self.scalar_static_f64[193]);
        let v841=(if v840{self.scalar_static_f64[193]}else{v838});
        let v843=(v1+v832);
        let v844=(v843/v837);
        let v846=((v832-v835)-(v844).ln());
        let v847=(v119*v846);
        let v848=(v707+v847);
        let v849=(v848/v339);
        let v850=(v849>v4);
        let v851=100.0;
        let v852=(v694<v851);
        let v853=(v850&&v852);
        let v856=(v850&&(!v852));
        let v858=(v1+(v694-v851));
        let v862=(v31*v119);
        let v863=(v411*v849);
        let v864=(v339*v863);
        let v866=(v1+(v121*v864));
        let v867=(v866).ln();
        let v871=(if v850{((v219+(v862*v867))-(if v856{(v851+(v858).ln())}else{(if v853{v694}else{v4})}))}else{v4});
        let v872=0.2;
        let v874=(if v850{(v219*v872)}else{v4});
        let v876=(if v850{(v874*v874)}else{v408});
        let v879=(v871<v4);
        let v880=(v850&&v879);
        let v881=(v411*v876);
        let v883=((v876+(if v850{(v871*v871)}else{v409}))).sqrt();
        let v884=(v883-v871);
        let v888=(v850&&(!v879));
        let v891=(if v888{(v411*(v871+v883))}else{(if v880{(v881/v884)}else{v4})});
        let v895=(v891+self.scalar_static_f64[196]);
        let v896=(v891*v895);
        let v899=(self.scalar_static_f64[195]*(v891+(v339*self.scalar_static_f64[194])));
        let v901=(if v850{(v896/v899)}else{v4});
        let v903=(if v850{(v849/v901)}else{v4});
        let v907=(if v850{((v903-v1)/self.scalar_static_f64[197])}else{v381});
        let v908=(v903<v1);
        let v909=(v850&&v908);
        let v910=(v907).exp();
        let v911=(v1+v910);
        let v917=(v850&&(!v908));
        let v919=((-v907)).exp();
        let v920=(v1+v919);
        let v933=(if v850{((if v917{(v903+(self.scalar_static_f64[197]*(v920).ln()))}else{(if v909{(v1+(self.scalar_static_f64[197]*(v911).ln()))}else{v4})})/self.scalar_static_f64[203])}else{v4});
        let v935=(if v850{(v891/self.scalar_static_f64[196])}else{v4});
        let v936=(v423*v933);
        let v937=(v935*v936);
        let v938=(v1+v935);
        let v941=((v1+(v937*v938))).sqrt();
        let v942=(v1+v941);
        let v943=(v31*v933);
        let v944=(v938*v943);
        let v946=(if v850{(v942/v944)}else{v4});
        let v948=(v841*v946);
        let v949=((v1-v946)+v948);
        let v950=(v1+v948);
        let v952=(if v850{(v949/v950)}else{v4});
        let v953=(v864*v952);
        let v955=(if v850{(v121*v953)}else{v4});
        let v958=(v1+(v841+v955));
        let v961=(if v850{((v31*v955)+(v841*v958))}else{v4});
        let v964=(if v850{(v411*(v955-v1))}else{v4});
        let v967=(if v850{(v961+(v964*v964))}else{v4});
        let v968=(v955>=v1);
        let v969=(v850&&v968);
        let v970=(v967).sqrt();
        let v974=(v850&&(!v968));
        let v975=(v970-v964);
        let v977=(if v974{(v961/v975)}else{(if v969{(v964+v970)}else{v4})});
        let v980=(v850&&(v977<self.scalar_static_f64[204]));
        let v981=(if v980{self.scalar_static_f64[204]}else{v977});
        let v982=(v1+v981);
        let v983=(v981*v982);
        let v985=((v121*v219)).exp();
        let v991=(if v850{(self.scalar_static_f64[205]*(v849-self.scalar_static_f64[194]))}else{v4});
        let v993=(self.scalar_static_f64[194]*(v339*self.scalar_static_f64[195]));
        let v998=(((if v850{(v849*v993)}else{v4})+(v991*v991))).sqrt();
        let v1003=(v850&&self.scalar_static_bool[19]);
        let v1004=(v46*v260);
        let v1007=(v850&&self.scalar_static_bool[20]);
        let v1008=(v31*v849);
        let v1009=(v849+v901);
        let v1011=(v46+(v1008/v1009));
        let v1014=(v849*self.scalar_static_f64[194]);
        let v1015=(v849+self.scalar_static_f64[194]);
        let v1020=(!v850);
        let v1021=(v31*v818);
        let v1024=(if v1020{(if v738{(v740*(v1+(v733-self.scalar_static_f64[191])))}else{(if v735{v736}else{v4})})}else{(if v850{(v983*v985)}else{v4})});
        let v1035=(((v707).abs()<(v119*1e-5))||((v847).abs()<((v119*1e-40)*(v832+v835))));
        let v1036=(v1020&&v1035);
        let v1037=(v841+(if v1020{(v1021/v843)}else{v981}));
        let v1039=(if v1036{(v411*v1037)}else{v4});
        let v1040=(v1+v1039);
        let v1044=(v1020&&(!v1035));
        let v1046=((v697+v847)-v694);
        let v1048=(if v1044{(v847/v1046)}else{(if v1036{(v1039/v1040)}else{v952})});
        let v1050=(if v1020{v1004}else{(if v1007{(v260*v1011)}else{(if v1003{v1004}else{v4})})});
        let v1051=(if v1020{v849}else{(if v850{(v1014/v1015)}else{v4})});
        let v1054=(if v1020{(v1-(v1051/self.scalar_static_f64[194]))}else{(if v850{(self.scalar_static_f64[194]/v1015)}else{v4})});
        let v1058=(v196*self.scalar_static_f64[209]);
        let v1059=(v46*v196);
        let v1060=(v700-v1058);
        let v1061=(v1060/v1059);
        let v1062=(v700<v1058);
        let v1063=(v1061).exp();
        let v1064=(v1+v1063);
        let v1065=(v1064).ln();
        let v1069=(!v1062);
        let v1071=((-v1061)).exp();
        let v1072=(v1+v1071);
        let v1073=(v1072).ln();
        let v1076=(if v1069{(v1058-(v1059*v1073))}else{(if v1062{(v700-(v1059*v1065))}else{v4})});
        let v1078=(v1-(v284*v1076));
        let v1080=f64::powf(v1078,self.scalar_static_f64[210]);
        let v1081=(v196/self.scalar_static_f64[210]);
        let v1082=(v1-v1080);
        let v1086=((v1081*v1082)+(v170*(v700-v1076)));
        let v1097=(if self.scalar_static_bool[26]{v697}else{(if self.scalar_static_bool[24]{(v694+(if v1020{v707}else{(if v850{(v991+v998)}else{v4})}))}else{(if self.scalar_static_bool[21]{v694}else{v4})})});
        let v1098=(v31-v301);
        let v1099=(v1-v301);
        let v1100=(v1098/v1099);
        let v1103=(v1-f64::powf(v1100,self.scalar_static_f64[212]));
        let v1104=(v260*v1103);
        let v1105=(v1097-v1104);
        let v1106=(v1105/v1050);
        let v1107=(v1097<v1104);
        let v1108=(v1106).exp();
        let v1109=(v1+v1108);
        let v1110=(v1109).ln();
        let v1114=(!v1107);
        let v1116=((-v1106)).exp();
        let v1117=(v1+v1116);
        let v1118=(v1117).ln();
        let v1121=(if v1114{(v1104-(v1050*v1118))}else{(if v1107{(v1097-(v1050*v1110))}else{v4})});
        let v1123=f64::powf(v1054,self.scalar_static_f64[213]);
        let v1125=(v260/self.scalar_static_f64[214]);
        let v1127=(v1-(v1121/v260));
        let v1128=f64::powf(v1127,self.scalar_static_f64[214]);
        let v1130=(v1-(v1123*v1128));
        let v1132=(v1100*v1123);
        let v1133=(v1097-v1121);
        let v1135=((v1125*v1130)+(v1132*v1133));
        let v1138=((v1099*v1135)+(v301*v694));
        let v1139=(v423*v436);
        let v1140=(v1139/v441);
        let v1141=(v755*v1140);
        let v1143=((v1+v1141)).sqrt();
        let v1144=(v1+v1143);
        let v1145=(v1141/v1144);
        let v1146=(v1/v402);
        let v1147=f64::powf(v1024,v1146);
        let v1148=(v1140*v1147);
        let v1150=((v1+v1148)).sqrt();
        let v1151=(v1+v1150);
        let v1152=(v1148/v1151);
        let v1155=(v1+(v1086/v610));
        let v1156=(v1138/v607);
        let v1157=(v1155+v1156);
        let v1160=(v661*v1155);
        let v1163=(-v1138);
        let v1164=(v1163/v607);
        let v1165=(v661*v1164);
        let v1168=((if self.scalar_static_bool[28]{(v121*v1160)}else{v4})).exp();
        let v1169=((if self.scalar_static_bool[28]{(v121*v1165)}else{v4})).exp();
        let v1170=(v1168-v1169);
        let v1172=((v121*v661)).exp();
        let v1173=(v1172-v1);
        let v1175=(if self.scalar_static_bool[28]{(v1170/v1173)}else{(if self.scalar_static_bool[27]{v1157}else{v4})});
        let v1176=0.010000000000000002;
        let v1177=(v1175*v1175);
        let v1178=(v1175<v4);
        let v1179=0.005000000000000001;
        let v1181=((v1176+v1177)).sqrt();
        let v1182=(v1181-v1175);
        let v1185=(!v1178);
        let v1188=(if v1185{(v411*(v1175+v1181))}else{(if v1178{(v1179/v1182)}else{v4})});
        let v1191=(v1+(v411*(v1145+v1152)));
        let v1192=(v1188*v1191);
        let v1194=(v436*self.scalar_static_f64[215]);
        let v1195=(v1147*v1194);
        let v1196=(v436*v755);
        let v1197=(v1196-v1195);
        let v1198=(v1197/v1192);
        let v1199=0.0001;
        let v1200=(v700/v1199);
        let v1201=(v700<v4);
        let v1202=(v1200).exp();
        let v1203=(v1+v1202);
        let v1207=(!v1201);
        let v1209=((-v1200)).exp();
        let v1210=(v1+v1209);
        let v1214=(if v1207{(v700+(v1199*(v1210).ln()))}else{(if v1201{(v1199*(v1203).ln())}else{v4})});
        let v1216=(v1214/self.scalar_static_f64[216]);
        let v1217=(v1216<self.scalar_static_f64[191]);
        let v1220=(!v1217);
        let v1221=(if v1220{self.scalar_static_f64[192]}else{v825});
        let v1230=((v700-self.scalar_static_f64[217])/v30);
        let v1251=(v745/self.scalar_static_f64[138]);
        let v1252=(v1251<self.scalar_static_f64[191]);
        let v1253=(v1251).exp();
        let v1255=(!v1252);
        let v1256=(if v1255{self.scalar_static_f64[192]}else{v1221});
        let v1260=(if v1255{(v1256*(v1+(v1251-self.scalar_static_f64[191])))}else{(if v1252{v1253}else{v1214})});
        let v1261=(v700-v283);
        let v1262=(v121*v1261);
        let v1263=(v1262<self.scalar_static_f64[191]);
        let v1268=(self.scalar_static_bool[12]&&(!v1263));
        let v1269=(if v1268{self.scalar_static_f64[192]}else{v1256});
        let v1276=((v1198/v436)-1000.0);
        let v1277=40.0;
        let v1278=(v1276<v1277);
        let v1283=(self.scalar_static_bool[12]&&(!v1278));
        let v1285=(if v1283{2.3538526683702e17}else{v1269});
        let v1325=(v121*v703);
        let v1326=(v1325/self.scalar_static_f64[142]);
        let v1327=(v1326<self.scalar_static_f64[191]);
        let v1328=(v1326).exp();
        let v1330=(!v1327);
        let v1331=(if v1330{self.scalar_static_f64[192]}else{v1285});
        let v1335=(if v1330{(v1331*(v1+(v1326-self.scalar_static_f64[191])))}else{(if v1327{v1328}else{v1260})});
        let v1336=(v703-v283);
        let v1337=(v121*v1336);
        let v1338=(v1337<self.scalar_static_f64[191]);
        let v1343=(self.scalar_static_bool[12]&&(!v1338));
        let v1344=(if v1343{self.scalar_static_f64[192]}else{v1331});
        let v1361=(v745/self.scalar_static_f64[125]);
        let v1362=(v1361<self.scalar_static_f64[191]);
        let v1363=(v1361).exp();
        let v1365=(!v1362);
        let v1366=(if v1365{self.scalar_static_f64[192]}else{v1344});
        let v1370=(if v1365{(v1366*(v1+(v1361-self.scalar_static_f64[191])))}else{(if v1362{v1363}else{v1335})});
        let v1373=(v1325/self.scalar_static_f64[159]);
        let v1374=(v1373<self.scalar_static_f64[191]);
        let v1375=(v1373).exp();
        let v1377=(!v1374);
        let v1378=(if v1377{self.scalar_static_f64[192]}else{v1366});
        let v1382=(if v1377{(v1378*(v1+(v1373-self.scalar_static_f64[191])))}else{(if v1374{v1375}else{v1370})});
        let v1385=(v756/self.scalar_static_f64[131]);
        let v1386=(v1385<self.scalar_static_f64[191]);
        let v1387=(v1385).exp();
        let v1389=(!v1386);
        let v1390=(if v1389{self.scalar_static_f64[192]}else{v1378});
        let v1394=(if v1389{(v1390*(v1+(v1385-self.scalar_static_f64[191])))}else{(if v1386{v1387}else{v1382})});
        let v1397=(v1325/self.scalar_static_f64[163]);
        let v1398=(v1397<self.scalar_static_f64[191]);
        let v1399=(v1397).exp();
        let v1401=(!v1398);
        let v1402=(if v1401{self.scalar_static_f64[192]}else{v1390});
        let v1406=(if v1401{(v1402*(v1+(v1397-self.scalar_static_f64[191])))}else{(if v1398{v1399}else{v1394})});
        let v1412=(v1201&&self.scalar_static_bool[36]);
        let v1413=(v31*v1080);
        let v1415=(v1-(self.scalar_static_f64[18]/v1413));
        let v1416=(v570*v1415);
        let v1417=(v1416<self.scalar_static_f64[191]);
        let v1422=(v1412&&(!v1417));
        let v1423=(if v1422{self.scalar_static_f64[192]}else{v1402});
        let v1429=(if v1412{(v284*v700)}else{v604});
        let v1431=1e-30;
        let v1433=(((v1429*v1429)+v1431)).sqrt();
        let v1436=f64::powf(v1433,self.scalar_static_f64[221]);
        let v1444=(v449*v1429);
        let v1445=(v1429*v1444);
        let v1446=(v1429+self.scalar_static_f64[224]);
        let v1448=((self.scalar_static_f64[16]*(self.scalar_static_f64[223]-((v170*v1429)*self.scalar_static_f64[224])))-(v1445*v1446));
        let v1450=0.16666666666666666;
        let v1452=(if v1412{((v1436*v1448)*v1450)}else{v4});
        let v1453=(self.scalar_static_f64[18]*v700);
        let v1454=(v570*v1453);
        let v1455=(v147*v1452);
        let v1457=(if v1412{(v1454/v1455)}else{v1429});
        let v1458=-0.001;
        let v1459=(v1457<v1458);
        let v1460=(v1457<self.scalar_static_f64[191]);
        let v1461=(v1412&&v1459);
        let v1466=(v1461&&(!v1460));
        let v1467=(if v1466{self.scalar_static_f64[192]}else{v1423});
        let v1504=(self.scalar_static_bool[39]&&(v694<v4));
        let v1505=(v285*v694);
        let v1506=(v1-v1505);
        let v1508=(if v1504{f64::powf(v1506,self.scalar_static_f64[214])}else{v4});
        let v1509=(v31*v1508);
        let v1511=(v1-(self.scalar_static_f64[49]/v1509));
        let v1512=(v592*v1511);
        let v1513=(v1512<self.scalar_static_f64[191]);
        let v1518=(v1504&&(!v1513));
        let v1519=(if v1518{self.scalar_static_f64[192]}else{v1467});
        let v1524=(if v1504{v1505}else{v582});
        let v1527=((v1431+(v1524*v1524))).sqrt();
        let v1529=f64::powf(v1527,self.scalar_static_f64[225]);
        let v1537=(v449*v1524);
        let v1538=(v1524*v1537);
        let v1539=(v1524+self.scalar_static_f64[228]);
        let v1541=((self.scalar_static_f64[47]*(self.scalar_static_f64[227]-((v170*v1524)*self.scalar_static_f64[228])))-(v1538*v1539));
        let v1544=(if v1504{(v1450*(v1529*v1541))}else{v4});
        let v1545=(self.scalar_static_f64[49]*v694);
        let v1546=(v592*v1545);
        let v1547=(v169*v1544);
        let v1549=(if v1504{(v1546/v1547)}else{v1524});
        let v1550=(v1549<v1458);
        let v1551=(v1549<self.scalar_static_f64[191]);
        let v1552=(v1504&&v1550);
        let v1557=(v1552&&(!v1551));
        let v1558=(if v1557{self.scalar_static_f64[192]}else{v1519});
        let v1589=(v765*v1140);
        let v1590=(v423*(if v802{(v803*(v1+(v798-self.scalar_static_f64[191])))}else{(if v799{v800}else{v4})}));
        let v1591=(v1589-v1140);
        let v1593=((v1+v1589)).sqrt();
        let v1594=(v1+v1593);
        let v1595=(v1591/v1594);
        let v1597=((v1+v1590)).sqrt();
        let v1598=(v1+v1597);
        let v1599=(v1590/v1598);
        let v1600=(v31*v531);
        let v1603=(v423*v531);
        let v1604=(v1603/v447);
        let v1617=(v531*self.scalar_static_f64[230]);
        let v1618=(v785-v1);
        let v1619=(v1617*v1618);
        let v1622=((v1+(v785*v1604))).sqrt();
        let v1623=(v1+v1622);
        let v1625=(if self.scalar_static_bool[42]{(v1619/v1623)}else{v4});
        let v1628=(self.scalar_static_f64[5]*v531);
        let v1630=(if self.scalar_static_bool[44]{(v327*v1628)}else{v4});
        let v1631=(v121*v1630);
        let v1633=(v31-(v1631).ln());
        let v1637=(if self.scalar_static_bool[44]{(v732-(if self.scalar_static_bool[44]{(v119*v1633)}else{v4}))}else{v4});
        let v1641=(if self.scalar_static_bool[44]{(v1637*v1637)}else{v1177});
        let v1642=(v1637<v4);
        let v1643=(self.scalar_static_bool[44]&&v1642);
        let v1646=((self.scalar_static_f64[231]+v1641)).sqrt();
        let v1647=(v1646-v1637);
        let v1651=(self.scalar_static_bool[44]&&(!v1642));
        let v1654=(if v1651{(v411*(v1637+v1646))}else{(if v1643{(self.scalar_static_f64[232]/v1647)}else{v4})});
        let v1657=(v1654+(v1630+(v327*v1625)));
        let v1662=(if self.scalar_static_bool[46]{v1}else{(if self.scalar_static_bool[44]{(v1654/v1657)}else{v1})});
        let v1721=(v1157<v4);
        let v1723=((v1176+(v1157*v1157))).sqrt();
        let v1724=(v1723-v1157);
        let v1727=(!v1721);
        let v1730=(if v1727{(v411*(v1157+v1723))}else{(if v1721{(v1179/v1724)}else{v4})});
        let v1740=(v1198>v4);
        let v1744=(v694<self.scalar_static_f64[252]);
        let v1747=((-v1198)/self.scalar_static_f64[253]);
        let v1748=(v1747<self.scalar_static_f64[191]);
        let v1750=(v1744&&(v1740&&self.scalar_static_bool[49]));
        let v1751=(v1748&&v1750);
        let v1752=(v1747).exp();
        let v1755=(v1750&&(!v1748));
        let v1756=(if v1755{self.scalar_static_f64[192]}else{v1558});
        let v1760=(if v1755{(v1756*(v1+(v1747-self.scalar_static_f64[191])))}else{(if v1751{v1752}else{v4})});
        let v1761=(self.scalar_static_f64[252]-v694);
        let v1763=(if v1750{(v1760*v1761)}else{v4});
        let v1764=(-v421);
        let v1766=f64::powf(v1763,self.scalar_static_f64[254]);
        let v1767=(v1764*v1766);
        let v1768=(v1767<self.scalar_static_f64[191]);
        let v1773=(v1750&&(!v1768));
        let v1774=(if v1773{self.scalar_static_f64[192]}else{v1756});
        let v1787=(v1740&&self.scalar_static_bool[51]);
        let v1891=(v1744&&(self.scalar_static_bool[54]&&(v1787&&self.scalar_static_bool[55])));
        let v1892=f64::powf(v1761,self.scalar_static_f64[254]);
        let v1894=(v1198+self.scalar_static_f64[267]);
        let v1896=(v1-(v1198/v1894));
        let v1898=f64::powf(v1896,self.scalar_static_f64[268]);
        let v1900=(if v1891{(v1892*v1898)}else{v4});
        let v1901=(self.scalar_static_bool[52]&&v1891);
        let v1903=(self.scalar_static_bool[53]&&v1891);
        let v1907=(if v1903{((v1198-self.scalar_static_f64[269])/self.scalar_static_f64[267])}else{v4});
        let v1911=(if v1903{((v1907-v1)/self.scalar_static_f64[270])}else{v1230});
        let v1912=(v1907<v1);
        let v1913=(v1903&&v1912);
        let v1914=(v1911).exp();
        let v1915=(v1+v1914);
        let v1921=(v1903&&(!v1912));
        let v1923=((-v1911)).exp();
        let v1924=(v1+v1923);
        let v1928=(if v1921{(v1907+(self.scalar_static_f64[270]*(v1924).ln()))}else{(if v1913{(v1+(self.scalar_static_f64[270]*(v1915).ln()))}else{v4})});
        let v1930=f64::powf(v1928,self.scalar_static_f64[271]);
        let v1932=(if v1903{(v1900*v1930)}else{(if v1901{v1900}else{v4})});
        let v1933=(v1764*v1932);
        let v1934=(v1933<self.scalar_static_f64[191]);
        let v1939=(v1891&&(!v1934));
        let v1940=(if v1939{self.scalar_static_f64[192]}else{v1774});
        let v1995=(v1024).ln();
        let v2049=(v291*self.scalar_static_f64[274]);
        let v2051=(v703-v1058);
        let v2052=(v2051/v1059);
        let v2053=(v703<v1058);
        let v2054=(v2052).exp();
        let v2055=(v1+v2054);
        let v2056=(v2055).ln();
        let v2060=(!v2053);
        let v2062=((-v2052)).exp();
        let v2063=(v1+v2062);
        let v2064=(v2063).ln();
        let v2067=(if v2060{(v1058-(v1059*v2064))}else{(if v2053{(v703-(v1059*v2056))}else{v4})});
        let v2068=(v291*self.scalar_static_f64[273]);
        let v2070=(v1-(v284*v2067));
        let v2072=(v1-f64::powf(v2070,self.scalar_static_f64[210]));
        let v2076=((v1081*v2072)+(v170*(v703-v2067)));
        let v2079=(v300*self.scalar_static_f64[275]);
        let v2081=(v441*v626);
        let v2082=(v411*v2081);
        let v2083=(v1145*v2082);
        let v2084=(v1730*v2083);
        let v2085=(v1152*v2082);
        let v2086=(v1730*v2085);
        let v2087=(v727-v1104);
        let v2088=(v2087/v1004);
        let v2089=(v727<v1104);
        let v2090=(v2088).exp();
        let v2091=(v1+v2090);
        let v2092=(v2091).ln();
        let v2096=(!v2089);
        let v2098=((-v2088)).exp();
        let v2099=(v1+v2098);
        let v2100=(v2099).ln();
        let v2103=(if v2096{(v1104-(v1004*v2100))}else{(if v2089{(v727-(v1004*v2092))}else{v4})});
        let v2105=(v1-(v2103/v260));
        let v2107=(v1-f64::powf(v2105,self.scalar_static_f64[214]));
        let v2109=(v727-v2103);
        let v2111=((v1125*v2107)+(v1100*v2109));
        let v2114=((v1099*v2111)+(v301*v727));
        let v2119=(v732-v1104);
        let v2120=(v2119/v1004);
        let v2121=(v732<v1104);
        let v2122=(v2120).exp();
        let v2123=(v1+v2122);
        let v2124=(v2123).ln();
        let v2128=(!v2121);
        let v2130=((-v2120)).exp();
        let v2131=(v1+v2130);
        let v2132=(v2131).ln();
        let v2135=(if v2128{(v1104-(v1004*v2132))}else{(if v2121{(v732-(v1004*v2124))}else{v4})});
        let v2137=(v1-(v2135/v260));
        let v2139=(v1-f64::powf(v2137,self.scalar_static_f64[214]));
        let v2141=(v732-v2135);
        let v2143=((v1125*v2139)+(v1100*v2141));
        let v2146=((v1099*v2143)+(v301*v732));
        let v2150=(v441*v620);
        let v2151=(v436/v441);
        let v2154=f64::powf(v2151,self.scalar_static_f64[278]);
        let v2155=(v2150*v2154);
        let v2156=(v119*self.scalar_static_f64[277]);
        let v2157=(v700/v2156);
        let v2158=(v2157<self.scalar_static_f64[191]);
        let v2159=(v2157).exp();
        let v2161=(!v2158);
        let v2162=(if v2161{self.scalar_static_f64[192]}else{v1940});
        let v2166=(if v2161{(v2162*(v1+(v2157-self.scalar_static_f64[191])))}else{(if v2158{v2159}else{v1406})});
        let v2167=(v2155*v2166);
        let v2168=(v423*v631);
        let v2169=(v119*v2168);
        let v2170=(v2169/v339);
        let v2171=(v411*v2170);
        let v2172=(v1048*v2171);
        let v2173=(v31+v1037);
        let v2177=(v411*v636);
        let v2180=((v1595*v2081)+(v1599*v2170));
        let v2181=(v2177*v2180);
        let v2186=((v727-v240)/self.scalar_static_f64[280]);
        let v2187=(v121*v2186);
        let v2188=(v2187<self.scalar_static_f64[191]);
        let v2190=(v2188&&self.scalar_static_bool[60]);
        let v2191=(v2187).exp();
        let v2194=(self.scalar_static_bool[60]&&(!v2188));
        let v2195=(if v2194{self.scalar_static_f64[192]}else{v2162});
        let v2200=(v642*v1600);
        let v2201=(v765*v2200);
        let v2204=((v1+(v423*(if v2194{(v2195*(v1+(v2187-self.scalar_static_f64[191])))}else{(if v2190{v2191}else{v4})})))).sqrt();
        let v2205=(v1+v2204);
        let v2207=(if self.scalar_static_bool[60]{(v2201/v2205)}else{(if self.scalar_static_bool[59]{(v2181/v633)}else{v4})});
        let v2215=(if self.scalar_static_bool[64]{(v785*v1140)}else{v4});
        let v2216=(v2215-v1140);
        let v2218=((v1+v2215)).sqrt();
        let v2219=(v1+v2218);
        let v2221=(if self.scalar_static_bool[64]{(v2216/v2219)}else{v4});
        let v2223=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*(v1+(v787-self.scalar_static_f64[191])))}else{(if v788{v789}else{v4})}))}else{v4});
        let v2225=((v1+v2223)).sqrt();
        let v2226=(v1+v2225);
        let v2228=(if self.scalar_static_bool[64]{(v2223/v2226)}else{v4});
        let v2230=(v636*self.scalar_static_f64[281]);
        let v2233=((v2081*v2221)+(v2170*v2228));
        let v2234=(v2230*v2233);
        let v2237=(v732-v240);
        let v2238=(v121*v2237);
        let v2239=(v2238<self.scalar_static_f64[191]);
        let v2241=(v2239&&self.scalar_static_bool[65]);
        let v2242=(v2238).exp();
        let v2245=(self.scalar_static_bool[65]&&(!v2239));
        let v2246=(if v2245{self.scalar_static_f64[192]}else{v2195});
        let v2251=(v642*v1617);
        let v2252=(v785*v2251);
        let v2255=((v1+(v423*(if v2245{(v2246*(v1+(v2238-self.scalar_static_f64[191])))}else{(if v2241{v2242}else{v4})})))).sqrt();
        let v2256=(v1+v2255);
        let v2258=(if self.scalar_static_bool[65]{(v2252/v2256)}else{(if self.scalar_static_bool[64]{(v2234/v633)}else{v4})});
        let v2266=(if self.scalar_static_bool[66]{(f64::powf(v1078,self.scalar_static_f64[283])-v170)}else{v4});
        let v2267=(if self.scalar_static_bool[66]{v1061}else{v4});
        let v2268=(v2267<v4);
        let v2269=(self.scalar_static_bool[66]&&v2268);
        let v2270=(v2267).exp();
        let v2271=(v1+v2270);
        let v2275=(self.scalar_static_bool[66]&&(!v2268));
        let v2277=((-v2267)).exp();
        let v2278=(v1+v2277);
        let v2280=(if v2275{(v2277/v2278)}else{(if v2269{(v1/v2271)}else{v4})});
        let v2283=(if self.scalar_static_bool[66]{(v170+(v2266*v2280))}else{v4});
        let v2286=(v121*v1141);
        let v2287=(v2286/v371);
        let v2288=(v411/v1143);
        let v2290=(if self.scalar_static_bool[66]{(v2287*v2288)}else{v4});
        let v2291=(v1730*v2082);
        let v2296=(v705*v872);
        let v2298=((if self.scalar_static_bool[66]{(v2167/v2156)}else{v4})+((if self.scalar_static_bool[66]{(v2049*v2283)}else{v4})+(if self.scalar_static_bool[66]{(v2290*v2291)}else{v4})));
        let v2307=(if self.scalar_static_bool[66]{(v2084+(v2167*self.scalar_static_f64[284]))}else{v4});
        let v2316=(if self.scalar_static_bool[67]{v2084}else{(if self.scalar_static_bool[66]{(v2307*self.scalar_static_f64[287])}else{v4})});
        let v2317=(if self.scalar_static_bool[67]{v2086}else{(if self.scalar_static_bool[66]{(v2086+(v2307*self.scalar_static_f64[286]))}else{v4})});
        let v2320=(v101*self.scalar_static_f64[288]);
        let v2355=(v1195+v1196);
        let v2356=(v2355/v1192);
        let v2364=(v2356>v4);
        let v2365=(v2316+v2317);
        let v2368=(!v2364);
        let v2369=(v626*v1730);
        let v2371=(if v2368{(v1192*v2369)}else{(if v2364{(v2365/v2356)}else{v4})});
        let v2384=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(v2371*self.scalar_static_f64[294])}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v2371)}else{v4})})});
        let v2420=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v2167}else{(if self.scalar_static_bool[66]{(v2167*self.scalar_static_f64[285])}else{v4})})+((v1086*v2049)+v2316)));
        let v2423=(self.scalar_static_f64[0]*(v2068*v2076));
        let v2426=(self.scalar_static_f64[0]*((v2172*v2173)+((v1138*v2079)+v2317)));
        let v2429=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*v2298)}else{v4}));
        let v2433=((self.scalar_static_f64[0]*(v711-v708))*self.scalar_static_f64[297]);
        let v2437=(v718*self.scalar_static_f64[298]);
        let v2445=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[276]*(v300*v2146)))+(if self.scalar_static_bool[63]{(v1662*v2258)}else{v4})));
        let v2451=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*((v300*v2114)*self.scalar_static_f64[276]))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v2207)}else{v2207})));
        let v2462=ctx.node_voltage(nodes[11]);
        let v2468=(if v102{(-(-1.0/v103))}else{v1});
        let v2471=(if v110{(v2468/v112)}else{(if v108{v2468}else{v4})});
        let v2472=(v2471/self.scalar_static_f64[8]);
        let v2473=(v118*v2471);
        let v2475=(v119*v119);
        let v2476=((-v2473)/v2475);
        let v2477=(v2472/v117);
        let v2523=((v172*v2477)+(v125*(v171*v2473)));
        let v2526=(-v2472);
        let v2528=((v2523+(self.scalar_static_f64[44]*v2472))+(self.scalar_static_f64[82]*v2526));
        let v2533=(((v119*(-v2528))-(v180*v2473))/v2475);
        let v2547=(if v189{((v193*v2473)+(v119*((v191*(-v2533))/v192)))}else{(if v182{(v2528+((v185*v2473)+(v119*((v183*v2533)/v184))))}else{v4})});
        let v2550=(self.scalar_static_f64[84]*v2526);
        let v2551=((v2523+(self.scalar_static_f64[83]*v2472))+v2550);
        let v2556=(((v119*(-v2551))-(v203*v2473))/v2475);
        let v2570=(if v212{((v216*v2473)+(v119*((v214*(-v2556))/v215)))}else{(if v205{(v2551+((v208*v2473)+(v119*((v206*v2556)/v207))))}else{v4})});
        let v2573=(v2550+(v2523+(self.scalar_static_f64[85]*v2472)));
        let v2578=(((v119*(-v2573))-(v224*v2473))/v2475);
        let v2595=(v2550+(v2523+(self.scalar_static_f64[46]*v2472)));
        let v2600=(((v119*(-v2595))-(v244*v2473))/v2475);
        let v2614=(if v253{((v257*v2473)+(v119*((v255*(-v2600))/v256)))}else{(if v246{(v2595+((v249*v2473)+(v119*((v247*v2600)/v248))))}else{v4})});
        let v2618=((v2523+(self.scalar_static_f64[86]*v2472))+(self.scalar_static_f64[87]*v2526));
        let v2623=(((v119*(-v2618))-(v267*v2473))/v2475);
        let v2640=((-v2547)/(v196*v196));
        let v2642=(v260*v260);
        let v2647=((self.scalar_static_f64[44]*v2640)*(self.scalar_static_f64[16]*f64::powf(v286,self.scalar_static_f64[224])));
        let v2652=(self.scalar_static_f64[88]*v2647);
        let v2659=(self.scalar_static_f64[90]*(((-(self.scalar_static_f64[46]*v2614))/v2642)*(self.scalar_static_f64[47]*f64::powf(v294,self.scalar_static_f64[228]))));
        let v2662=((-v2659)/(v297*v297));
        let v2663=(self.scalar_static_f64[91]*v2659);
        let v2664=(self.scalar_static_f64[89]*v2662);
        let v2678=(self.scalar_static_f64[100]*(v326*(self.scalar_static_f64[101]*v2477)));
        let v2685=(self.scalar_static_f64[105]*(v338*(self.scalar_static_f64[106]*v2477)));
        let v2688=(if self.scalar_static_bool[8]{(self.scalar_static_f64[108]*(self.scalar_static_f64[107]*v2471))}else{v4});
        let v2690=(if self.scalar_static_bool[8]{(v2688/v30)}else{v2623});
        let v2694=(if v351{(v30*((v352*v2690)/v353))}else{v2688});
        let v2702=(if self.scalar_static_bool[9]{v4}else{(if self.scalar_static_bool[8]{(if v359{(v2694+(v30*((v361*(-v2690))/v362)))}else{v2694})}else{v4})});
        let v2705=(if self.scalar_static_bool[10]{(self.scalar_static_f64[110]*(self.scalar_static_f64[109]*v2471))}else{v4});
        let v2707=(if self.scalar_static_bool[10]{(v2705/v30)}else{v2690});
        let v2711=(if v383{(v30*((v384*v2707)/v385))}else{v2705});
        let v2721=(self.scalar_static_f64[111]*(self.scalar_static_f64[112]*v2471));
        let v2722=(v407*v2721);
        let v2723=(v2722+v2722);
        let v2739=(v371*v371);
        let v2751=((v435*(self.scalar_static_f64[113]*(v430*(((v371*(self.scalar_static_f64[117]*v2477))-(v428*v2702))/v2739))))+(v431*(v435*(((v371*(self.scalar_static_f64[118]*v2476))-(v433*v2702))/v2739))));
        let v2754=(self.scalar_static_f64[119]*(v440*(self.scalar_static_f64[120]*v2477)));
        let v2815=((v530*(self.scalar_static_f64[153]*(v525*(self.scalar_static_f64[155]*v2477))))+(v526*(v530*(self.scalar_static_f64[157]*v2476))));
        let v2847=((-v2647)/(v287*v287));
        let v2922=(v604*(self.scalar_static_f64[96]*v2477));
        let v2926=((v606*v2662)+(v298*(self.scalar_static_f64[171]*v2922)));
        let v2941=(self.scalar_static_f64[177]*(v625*(self.scalar_static_f64[179]*v2477)));
        let v2944=(self.scalar_static_f64[180]*(v630*(self.scalar_static_f64[181]*v2477)));
        let v2945=(v2941+v2944);
        let v2947=((self.scalar_static_f64[182]*v2945)/self.scalar_static_f64[183]);
        let v2950=(self.scalar_static_f64[184]*(v641*(self.scalar_static_f64[186]*v2477)));
        let v2960=(self.scalar_static_f64[188]*v2922);
        let v2983=(v697*v2476);
        let v2984=(self.scalar_static_f64[0]*v121);
        let v2985=(v121*self.scalar_static_f64[299]);
        let v2998=(v700*v2476);
        let v3002=(((v371*v2998)-(v745*v2702))/v2739);
        let v3003=(v2985/v371);
        let v3004=(v2984/v371);
        let v3014=(if v750{(v751*v3002)}else{(if v747{(v748*v3002)}else{v4})});
        let v3015=(if v750{(v751*v3003)}else{(if v747{(v748*v3003)}else{v4})});
        let v3016=(if v750{(v751*v3004)}else{(if v747{(v748*v3004)}else{v4})});
        let v3017=(v727*v2476);
        let v3018=(v121*self.scalar_static_f64[300]);
        let v3019=(v121*self.scalar_static_f64[301]);
        let v3035=(if v760{(v761*v3017)}else{(if v757{(v758*v3017)}else{v4})});
        let v3036=(if v760{(v761*v2984)}else{(if v757{(v758*v2984)}else{v4})});
        let v3037=(if v760{(v761*v3018)}else{(if v757{(v758*v3018)}else{v4})});
        let v3038=(if v760{(v761*v3019)}else{(if v757{(v758*v3019)}else{v4})});
        let v3039=(if v760{(v761*v2985)}else{(if v757{(v758*v2985)}else{v4})});
        let v3053=(v121*self.scalar_static_f64[302]);
        let v3054=(v732*v2476);
        let v3070=(if v780{(v781*v3018)}else{(if v777{(v778*v3018)}else{v4})});
        let v3071=(if v780{(v781*v3053)}else{(if v777{(v778*v3053)}else{v4})});
        let v3072=(if v780{(v781*v3054)}else{(if v777{(v778*v3054)}else{v4})});
        let v3073=(if v780{(v781*v3019)}else{(if v777{(v778*v3019)}else{v4})});
        let v3074=(if v780{(v781*v2985)}else{(if v777{(v778*v2985)}else{v4})});
        let v3077=(v121*(-v2570));
        let v3078=((v786*v2476)+v3077);
        let v3100=(v3077+(v797*v2476));
        let v3122=(v3077+(v808*v2476));
        let v3132=(if v813{(v814*v3122)}else{(if v810{(v811*v3122)}else{v4})});
        let v3133=(if v813{(v814*v2984)}else{(if v810{(v811*v2984)}else{v4})});
        let v3134=(if v813{(v814*v2985)}else{(if v810{(v811*v2985)}else{v4})});
        let v3136=(v3077+(v819*v2476));
        let v3146=(if v824{(v825*v3136)}else{(if v821{(v822*v3136)}else{v4})});
        let v3147=(if v824{(v825*v2984)}else{(if v821{(v822*v2984)}else{v4})});
        let v3148=(if v824{(v825*v2985)}else{(if v821{(v822*v2985)}else{v4})});
        let v3152=(v31*v832);
        let v3153=((v423*v3132)/v3152);
        let v3154=((v423*v3133)/v3152);
        let v3155=((v423*v3134)/v3152);
        let v3159=(v31*v835);
        let v3160=((v423*v3146)/v3159);
        let v3161=((v423*v3147)/v3159);
        let v3162=((v423*v3148)/v3159);
        let v3169=(v837*v837);
        let v3179=(if v840{v4}else{(((v837*(v31*v3146))-(v836*v3160))/v3169)});
        let v3180=(if v840{v4}else{(((v837*(v31*v3147))-(v836*v3161))/v3169)});
        let v3181=(if v840{v4}else{(((v837*(v31*v3148))-(v836*v3162))/v3169)});
        let v3207=((v846*v2473)+(v119*((v3153-v3160)-((((v837*v3153)-(v843*v3160))/v3169)/v844))));
        let v3208=(v119*((v3154-v3161)-((((v837*v3154)-(v843*v3161))/v3169)/v844)));
        let v3209=(v119*((-v3162)-(((-(v843*v3162))/v3169)/v844)));
        let v3210=(v119*(v3155-((v3155/v837)/v844)));
        let v3212=(self.scalar_static_f64[299]+v3210);
        let v3216=(v339*v339);
        let v3217=(((v339*v3207)-(v848*v2685))/v3216);
        let v3218=(v3208/v339);
        let v3219=((self.scalar_static_f64[0]+v3209)/v339);
        let v3220=(v3212/v339);
        let v3227=(v31*v2473);
        let v3234=((v863*v2685)+(v339*(v411*v3217)));
        let v3235=(v339*(v411*v3218));
        let v3236=(v339*(v411*v3219));
        let v3237=(v339*(v411*v3220));
        let v3257=(if v850{(v2570+((v867*v3227)+(v862*(((v864*v2476)+(v121*v3234))/v866))))}else{v4});
        let v3258=(if v850{((v862*((v121*v3235)/v866))-(if v856{(self.scalar_static_f64[0]/v858)}else{(if v853{self.scalar_static_f64[0]}else{v4})}))}else{v4});
        let v3259=(if v850{((v862*((v121*v3236)/v866))-(if v856{(self.scalar_static_f64[299]/v858)}else{(if v853{self.scalar_static_f64[299]}else{v4})}))}else{v4});
        let v3260=(if v850{(v862*((v121*v3237)/v866))}else{v4});
        let v3263=(v874*(if v850{(v872*v2570)}else{v4}));
        let v3265=(if v850{(v3263+v3263)}else{v4});
        let v3266=(v871*v3257);
        let v3268=(v871*v3258);
        let v3270=(v871*v3259);
        let v3272=(v871*v3260);
        let v3280=(v31*v883);
        let v3281=((v3265+(if v850{(v3266+v3266)}else{v2723}))/v3280);
        let v3282=((if v850{(v3268+v3268)}else{v4})/v3280);
        let v3283=((if v850{(v3270+v3270)}else{v4})/v3280);
        let v3284=((if v850{(v3272+v3272)}else{v4})/v3280);
        let v3292=(v884*v884);
        let v3315=(if v888{(v411*(v3257+v3281))}else{(if v880{(((v884*(v411*v3265))-(v881*(v3281-v3257)))/v3292)}else{v4})});
        let v3316=(if v888{(v411*(v3258+v3282))}else{(if v880{((-(v881*(v3282-v3258)))/v3292)}else{v4})});
        let v3317=(if v888{(v411*(v3259+v3283))}else{(if v880{((-(v881*(v3283-v3259)))/v3292)}else{v4})});
        let v3318=(if v888{(v411*(v3260+v3284))}else{(if v880{((-(v881*(v3284-v3260)))/v3292)}else{v4})});
        let v3340=(v899*v899);
        let v3354=(if v850{(((v899*((v895*v3315)+(v891*v3315)))-(v896*(self.scalar_static_f64[195]*(v3315+(self.scalar_static_f64[194]*v2685)))))/v3340)}else{v4});
        let v3355=(if v850{(((v899*((v895*v3316)+(v891*v3316)))-(v896*(self.scalar_static_f64[195]*v3316)))/v3340)}else{v4});
        let v3356=(if v850{(((v899*((v895*v3317)+(v891*v3317)))-(v896*(self.scalar_static_f64[195]*v3317)))/v3340)}else{v4});
        let v3357=(if v850{(((v899*((v895*v3318)+(v891*v3318)))-(v896*(self.scalar_static_f64[195]*v3318)))/v3340)}else{v4});
        let v3361=(v901*v901);
        let v3375=(if v850{(((v901*v3217)-(v849*v3354))/v3361)}else{v4});
        let v3376=(if v850{(((v901*v3218)-(v849*v3355))/v3361)}else{v4});
        let v3377=(if v850{(((v901*v3219)-(v849*v3356))/v3361)}else{v4});
        let v3378=(if v850{(((v901*v3220)-(v849*v3357))/v3361)}else{v4});
        let v3383=(if v850{(v3375/self.scalar_static_f64[197])}else{v2707});
        let v3384=(if v850{(v3376/self.scalar_static_f64[197])}else{v4});
        let v3385=(if v850{(v3377/self.scalar_static_f64[197])}else{v4});
        let v3386=(if v850{(v3378/self.scalar_static_f64[197])}else{v4});
        let v3431=(if v850{((if v917{(v3375+(self.scalar_static_f64[197]*((v919*(-v3383))/v920)))}else{(if v909{(self.scalar_static_f64[197]*((v910*v3383)/v911))}else{v4})})/self.scalar_static_f64[203])}else{v4});
        let v3432=(if v850{((if v917{(v3376+(self.scalar_static_f64[197]*((v919*(-v3384))/v920)))}else{(if v909{(self.scalar_static_f64[197]*((v910*v3384)/v911))}else{v4})})/self.scalar_static_f64[203])}else{v4});
        let v3433=(if v850{((if v917{(v3377+(self.scalar_static_f64[197]*((v919*(-v3385))/v920)))}else{(if v909{(self.scalar_static_f64[197]*((v910*v3385)/v911))}else{v4})})/self.scalar_static_f64[203])}else{v4});
        let v3434=(if v850{((if v917{(v3378+(self.scalar_static_f64[197]*((v919*(-v3386))/v920)))}else{(if v909{(self.scalar_static_f64[197]*((v910*v3386)/v911))}else{v4})})/self.scalar_static_f64[203])}else{v4});
        let v3439=(if v850{(v3315/self.scalar_static_f64[196])}else{v4});
        let v3440=(if v850{(v3316/self.scalar_static_f64[196])}else{v4});
        let v3441=(if v850{(v3317/self.scalar_static_f64[196])}else{v4});
        let v3442=(if v850{(v3318/self.scalar_static_f64[196])}else{v4});
        let v3471=(v31*v941);
        let v3495=(v944*v944);
        let v3509=(if v850{(((v944*(((v938*((v936*v3439)+(v935*(v423*v3431))))+(v937*v3439))/v3471))-(v942*((v943*v3439)+(v938*(v31*v3431)))))/v3495)}else{v4});
        let v3510=(if v850{(((v944*(((v938*((v936*v3440)+(v935*(v423*v3432))))+(v937*v3440))/v3471))-(v942*((v943*v3440)+(v938*(v31*v3432)))))/v3495)}else{v4});
        let v3511=(if v850{(((v944*(((v938*((v936*v3441)+(v935*(v423*v3433))))+(v937*v3441))/v3471))-(v942*((v943*v3441)+(v938*(v31*v3433)))))/v3495)}else{v4});
        let v3512=(if v850{(((v944*(((v938*((v936*v3442)+(v935*(v423*v3434))))+(v937*v3442))/v3471))-(v942*((v943*v3442)+(v938*(v31*v3434)))))/v3495)}else{v4});
        let v3519=((v946*v3179)+(v841*v3509));
        let v3522=((v946*v3180)+(v841*v3510));
        let v3525=((v946*v3181)+(v841*v3511));
        let v3526=(v841*v3512);
        let v3534=(v950*v950);
        let v3548=(if v850{(((v950*((-v3509)+v3519))-(v949*v3519))/v3534)}else{v4});
        let v3549=(if v850{(((v950*((-v3510)+v3522))-(v949*v3522))/v3534)}else{v4});
        let v3550=(if v850{(((v950*((-v3511)+v3525))-(v949*v3525))/v3534)}else{v4});
        let v3551=(if v850{(((v950*((-v3512)+v3526))-(v949*v3526))/v3534)}else{v4});
        let v3570=(if v850{((v953*v2476)+(v121*((v952*v3234)+(v864*v3548))))}else{v4});
        let v3571=(if v850{(v121*((v952*v3235)+(v864*v3549)))}else{v4});
        let v3572=(if v850{(v121*((v952*v3236)+(v864*v3550)))}else{v4});
        let v3573=(if v850{(v121*((v952*v3237)+(v864*v3551)))}else{v4});
        let v3595=(if v850{((v31*v3570)+((v958*v3179)+(v841*(v3179+v3570))))}else{v4});
        let v3596=(if v850{((v31*v3571)+((v958*v3180)+(v841*(v3180+v3571))))}else{v4});
        let v3597=(if v850{((v31*v3572)+((v958*v3181)+(v841*(v3181+v3572))))}else{v4});
        let v3598=(if v850{((v31*v3573)+(v841*v3573))}else{v4});
        let v3603=(if v850{(v411*v3570)}else{v4});
        let v3604=(if v850{(v411*v3571)}else{v4});
        let v3605=(if v850{(v411*v3572)}else{v4});
        let v3606=(if v850{(v411*v3573)}else{v4});
        let v3607=(v964*v3603);
        let v3609=(v964*v3604);
        let v3611=(v964*v3605);
        let v3613=(v964*v3606);
        let v3619=(if v850{(v3595+(v3607+v3607))}else{v4});
        let v3620=(if v850{(v3596+(v3609+v3609))}else{v4});
        let v3621=(if v850{(v3597+(v3611+v3611))}else{v4});
        let v3622=(if v850{(v3598+(v3613+v3613))}else{v4});
        let v3623=(v31*v970);
        let v3624=(v3619/v3623);
        let v3625=(v3620/v3623);
        let v3626=(v3621/v3623);
        let v3627=(v3622/v3623);
        let v3643=(v975*v975);
        let v3661=(if v980{v4}else{(if v974{(((v975*v3595)-(v961*(v3624-v3603)))/v3643)}else{(if v969{(v3603+v3624)}else{v4})})});
        let v3662=(if v980{v4}else{(if v974{(((v975*v3596)-(v961*(v3625-v3604)))/v3643)}else{(if v969{(v3604+v3625)}else{v4})})});
        let v3663=(if v980{v4}else{(if v974{(((v975*v3597)-(v961*(v3626-v3605)))/v3643)}else{(if v969{(v3605+v3626)}else{v4})})});
        let v3664=(if v980{v4}else{(if v974{(((v975*v3598)-(v961*(v3627-v3606)))/v3643)}else{(if v969{(v3606+v3627)}else{v4})})});
        let v3695=(if v850{(self.scalar_static_f64[205]*v3217)}else{v4});
        let v3696=(if v850{(self.scalar_static_f64[205]*v3218)}else{v4});
        let v3697=(if v850{(self.scalar_static_f64[205]*v3219)}else{v4});
        let v3698=(if v850{(self.scalar_static_f64[205]*v3220)}else{v4});
        let v3711=(v991*v3695);
        let v3713=(v991*v3696);
        let v3715=(v991*v3697);
        let v3717=(v991*v3698);
        let v3723=(v31*v998);
        let v3736=(v46*v2614);
        let v3749=(v1009*v1009);
        let v3773=(self.scalar_static_f64[194]*v3217);
        let v3774=(self.scalar_static_f64[194]*v3218);
        let v3775=(self.scalar_static_f64[194]*v3219);
        let v3776=(self.scalar_static_f64[194]*v3220);
        let v3780=(v1015*v1015);
        let v3816=(v843*v843);
        let v3829=(if v1020{(((v843*(v31*v3134))-(v1021*v3155))/v3816)}else{v3664});
        let v3830=(if v1020{(if v738{(v740*v2983)}else{(if v735{(v736*v2983)}else{v4})})}else{(if v850{((v985*((v982*v3661)+(v981*v3661)))+(v983*(v985*((v219*v2476)+(v121*v2570)))))}else{v4})});
        let v3831=(if v1020{(if v738{(v740*v2984)}else{(if v735{(v736*v2984)}else{v4})})}else{(if v850{(v985*((v982*v3662)+(v981*v3662)))}else{v4})});
        let v3832=(if v1020{v4}else{(if v850{(v985*((v982*v3663)+(v981*v3663)))}else{v4})});
        let v3833=(if v1020{(if v738{(v740*v2985)}else{(if v735{(v736*v2985)}else{v4})})}else{(if v850{(v985*((v982*v3664)+(v981*v3664)))}else{v4})});
        let v3834=(v3179+(if v1020{(((v843*(v31*v3132))-(v1021*v3153))/v3816)}else{v3661}));
        let v3835=(v3180+(if v1020{(((v843*(v31*v3133))-(v1021*v3154))/v3816)}else{v3662}));
        let v3836=(v3181+(if v1020{v4}else{v3663}));
        let v3841=(if v1036{(v411*v3834)}else{v4});
        let v3842=(if v1036{(v411*v3835)}else{v4});
        let v3843=(if v1036{(v411*v3836)}else{v4});
        let v3844=(if v1036{(v411*v3829)}else{v4});
        let v3848=(v1040*v1040);
        let v3872=(v1046*v1046);
        let v3886=(if v1044{(((v1046*v3207)-(v847*v3207))/v3872)}else{(if v1036{(((v1040*v3841)-(v1039*v3841))/v3848)}else{v3548})});
        let v3887=(if v1044{(((v1046*v3208)-(v847*((self.scalar_static_f64[0]+v3208)-self.scalar_static_f64[0])))/v3872)}else{(if v1036{(((v1040*v3842)-(v1039*v3842))/v3848)}else{v3549})});
        let v3888=(if v1044{(((v1046*v3209)-(v847*(v3209-self.scalar_static_f64[299])))/v3872)}else{(if v1036{(((v1040*v3843)-(v1039*v3843))/v3848)}else{v3550})});
        let v3889=(if v1044{(((v1046*v3210)-(v847*v3212))/v3872)}else{(if v1036{(((v1040*v3844)-(v1039*v3844))/v3848)}else{v3551})});
        let v3894=(if v1020{v3736}else{(if v1007{((v1011*v2614)+(v260*(((v1009*(v31*v3217))-(v1008*(v3217+v3354)))/v3749)))}else{(if v1003{v3736}else{v4})})});
        let v3895=(if v1020{v4}else{(if v1007{(v260*(((v1009*(v31*v3218))-(v1008*(v3218+v3355)))/v3749))}else{v4})});
        let v3896=(if v1020{v4}else{(if v1007{(v260*(((v1009*(v31*v3219))-(v1008*(v3219+v3356)))/v3749))}else{v4})});
        let v3897=(if v1020{v4}else{(if v1007{(v260*(((v1009*(v31*v3220))-(v1008*(v3220+v3357)))/v3749))}else{v4})});
        let v3898=(if v1020{v3217}else{(if v850{(((v1015*v3773)-(v1014*v3217))/v3780)}else{v4})});
        let v3899=(if v1020{v3218}else{(if v850{(((v1015*v3774)-(v1014*v3218))/v3780)}else{v4})});
        let v3900=(if v1020{v3219}else{(if v850{(((v1015*v3775)-(v1014*v3219))/v3780)}else{v4})});
        let v3901=(if v1020{v3220}else{(if v850{(((v1015*v3776)-(v1014*v3220))/v3780)}else{v4})});
        let v3910=(if v1020{(-(v3898/self.scalar_static_f64[194]))}else{(if v850{((-v3773)/v3780)}else{v4})});
        let v3911=(if v1020{(-(v3899/self.scalar_static_f64[194]))}else{(if v850{((-v3774)/v3780)}else{v4})});
        let v3912=(if v1020{(-(v3900/self.scalar_static_f64[194]))}else{(if v850{((-v3775)/v3780)}else{v4})});
        let v3913=(if v1020{(-(v3901/self.scalar_static_f64[194]))}else{(if v850{((-v3776)/v3780)}else{v4})});
        let v3914=(self.scalar_static_f64[209]*v2547);
        let v3915=(v46*v2547);
        let v3917=(v1059*(-v3914));
        let v3920=(v1059*v1059);
        let v3921=((v3917-(v1060*v3915))/v3920);
        let v3922=(self.scalar_static_f64[299]/v1059);
        let v3923=(self.scalar_static_f64[0]/v1059);
        let v3942=(-v3922);
        let v3943=(-v3923);
        let v3958=(if v1069{(v3914-((v1073*v3915)+(v1059*((v1071*(-v3921))/v1072))))}else{(if v1062{(-((v1065*v3915)+(v1059*((v1063*v3921)/v1064))))}else{v4})});
        let v3959=(if v1069{(-(v1059*((v1071*v3942)/v1072)))}else{(if v1062{(self.scalar_static_f64[299]-(v1059*((v1063*v3922)/v1064)))}else{v4})});
        let v3960=(if v1069{(-(v1059*((v1071*v3943)/v1072)))}else{(if v1062{(self.scalar_static_f64[0]-(v1059*((v1063*v3923)/v1064)))}else{v4})});
        let v3966=(-((v1076*v2640)+(v284*v3958)));
        let v3967=(-(v284*v3959));
        let v3968=(-(v284*v3960));
        let v3971=(self.scalar_static_f64[210]*f64::powf(v1078,self.scalar_static_f64[303]));
        let v3972=(v3966*v3971);
        let v3973=(v3967*v3971);
        let v3974=(v3968*v3971);
        let v3975=(v2547/self.scalar_static_f64[210]);
        let v3990=(((v1082*v3975)+(v1081*(-v3972)))+(v170*(-v3958)));
        let v3991=((v1081*(-v3973))+(v170*(self.scalar_static_f64[299]-v3959)));
        let v3992=((v1081*(-v3974))+(v170*(self.scalar_static_f64[0]-v3960)));
        let v4001=(if self.scalar_static_bool[26]{v4}else{(if self.scalar_static_bool[24]{(if v1020{v4}else{(if v850{(v3695+(((if v850{((v993*v3217)+(v849*(self.scalar_static_f64[194]*(self.scalar_static_f64[195]*v2685))))}else{v4})+(v3711+v3711))/v3723))}else{v4})})}else{v4})});
        let v4002=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v1020{v4}else{(if v850{(v3696+(((if v850{(v993*v3218)}else{v4})+(v3713+v3713))/v3723))}else{v4})}))}else{self.scalar_static_f64[304]})});
        let v4003=(if self.scalar_static_bool[26]{v4}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[299]+(if v1020{self.scalar_static_f64[0]}else{(if v850{(v3697+(((if v850{(v993*v3219)}else{v4})+(v3715+v3715))/v3723))}else{v4})}))}else{self.scalar_static_f64[305]})});
        let v4004=(if self.scalar_static_bool[26]{self.scalar_static_f64[299]}else{(if self.scalar_static_bool[24]{(if v1020{self.scalar_static_f64[299]}else{(if v850{(v3698+(((if v850{(v993*v3220)}else{v4})+(v3717+v3717))/v3723))}else{v4})})}else{v4})});
        let v4005=(-v2664);
        let v4010=(((v1099*v4005)-(v1098*v4005))/(v1099*v1099));
        let v4018=((v1103*v2614)+(v260*(-(v4010*(self.scalar_static_f64[212]*f64::powf(v1100,self.scalar_static_f64[306]))))));
        let v4023=(v1050*v1050);
        let v4024=(((v1050*(v4001-v4018))-(v1105*v3894))/v4023);
        let v4028=(((v1050*v4002)-(v1105*v3895))/v4023);
        let v4032=(((v1050*v4003)-(v1105*v3896))/v4023);
        let v4036=(((v1050*v4004)-(v1105*v3897))/v4023);
        let v4093=(if v1114{(v4018-((v1118*v3894)+(v1050*((v1116*(-v4024))/v1117))))}else{(if v1107{(v4001-((v1110*v3894)+(v1050*((v1108*v4024)/v1109))))}else{v4})});
        let v4094=(if v1114{(-((v1118*v3895)+(v1050*((v1116*(-v4028))/v1117))))}else{(if v1107{(v4002-((v1110*v3895)+(v1050*((v1108*v4028)/v1109))))}else{v4})});
        let v4095=(if v1114{(-((v1118*v3896)+(v1050*((v1116*(-v4032))/v1117))))}else{(if v1107{(v4003-((v1110*v3896)+(v1050*((v1108*v4032)/v1109))))}else{v4})});
        let v4096=(if v1114{(-((v1118*v3897)+(v1050*((v1116*(-v4036))/v1117))))}else{(if v1107{(v4004-((v1110*v3897)+(v1050*((v1108*v4036)/v1109))))}else{v4})});
        let v4099=(self.scalar_static_f64[213]*f64::powf(v1054,self.scalar_static_f64[307]));
        let v4100=(v3910*v4099);
        let v4101=(v3911*v4099);
        let v4102=(v3912*v4099);
        let v4103=(v3913*v4099);
        let v4104=(v2614/self.scalar_static_f64[214]);
        let v4118=(self.scalar_static_f64[214]*f64::powf(v1127,self.scalar_static_f64[308]));
        let v4176=(v1099*((v1125*(-((v1128*v4103)+(v1123*((-(v4096/v260))*v4118)))))+((v1133*(v1100*v4103))+(v1132*(v4004-v4096)))));
        let v4178=(self.scalar_static_f64[0]*v301);
        let v4179=(v301*self.scalar_static_f64[299]);
        let v4180=(((v1135*v4005)+(v1099*(((v1130*v4104)+(v1125*(-((v1128*v4100)+(v1123*((-(((v260*v4093)-(v1121*v2614))/v2642))*v4118))))))+((v1133*((v1123*v4010)+(v1100*v4100)))+(v1132*(v4001-v4093))))))+(v694*v2664));
        let v4181=((v1099*((v1125*(-((v1128*v4101)+(v1123*((-(v4094/v260))*v4118)))))+((v1133*(v1100*v4101))+(v1132*(v4002-v4094)))))+v4178);
        let v4182=((v1099*((v1125*(-((v1128*v4102)+(v1123*((-(v4095/v260))*v4118)))))+((v1133*(v1100*v4102))+(v1132*(v4003-v4095)))))+v4179);
        let v4187=(v441*v441);
        let v4188=(((v441*(v423*v2751))-(v1139*v2754))/v4187);
        let v4191=((v1140*v3014)+(v755*v4188));
        let v4192=(v1140*v3015);
        let v4193=(v1140*v3016);
        let v4194=(v31*v1143);
        let v4195=(v4191/v4194);
        let v4196=(v4192/v4194);
        let v4197=(v4193/v4194);
        let v4201=(v1144*v1144);
        let v4202=(((v1144*v4191)-(v1141*v4195))/v4201);
        let v4206=(((v1144*v4192)-(v1141*v4196))/v4201);
        let v4210=(((v1144*v4193)-(v1141*v4197))/v4201);
        let v4216=(v1146*f64::powf(v1024,(v1146-v1)));
        let v4220=((v3830*v4216)+(((-(if self.scalar_static_bool[11]{v4}else{(if self.scalar_static_bool[10]{(if v391{(v2711+(v30*((v393*(-v2707))/v394)))}else{v2711})}else{v4})}))/(v402*v402))*(v1147*v1995)));
        let v4221=(v3831*v4216);
        let v4222=(v3832*v4216);
        let v4223=(v3833*v4216);
        let v4226=((v1147*v4188)+(v1140*v4220));
        let v4227=(v1140*v4221);
        let v4228=(v1140*v4222);
        let v4229=(v1140*v4223);
        let v4230=(v31*v1150);
        let v4238=(v1151*v1151);
        let v4239=(((v1151*v4226)-(v1148*(v4226/v4230)))/v4238);
        let v4243=(((v1151*v4227)-(v1148*(v4227/v4230)))/v4238);
        let v4247=(((v1151*v4228)-(v1148*(v4228/v4230)))/v4238);
        let v4251=(((v1151*v4229)-(v1148*(v4229/v4230)))/v4238);
        let v4256=(((v610*v3990)-(v1086*((v609*v2847)+(v561*(self.scalar_static_f64[172]*v2922)))))/(v610*v610));
        let v4257=(v3991/v610);
        let v4258=(v3992/v610);
        let v4262=(v607*v607);
        let v4263=(((v607*v4180)-(v1138*v2926))/v4262);
        let v4264=(v4181/v607);
        let v4265=(v4182/v607);
        let v4266=(v4176/v607);
        let v4267=(v4256+v4263);
        let v4268=(v4258+v4264);
        let v4338=(if self.scalar_static_bool[28]{(((v1173*((v1168*(if self.scalar_static_bool[28]{((v1160*v2476)+(v121*((v1155*v2960)+(v661*v4256))))}else{v4}))-(v1169*(if self.scalar_static_bool[28]{((v1165*v2476)+(v121*((v1164*v2960)+(v661*(((v607*(-v4180))-(v1163*v2926))/v4262)))))}else{v4}))))-(v1170*(v1172*((v661*v2476)+(v121*v2960)))))/(v1173*v1173))}else{(if self.scalar_static_bool[27]{v4267}else{v4})});
        let v4339=(if self.scalar_static_bool[28]{((v1168*(if self.scalar_static_bool[28]{(v121*(v661*v4257))}else{v4}))/v1173)}else{(if self.scalar_static_bool[27]{v4257}else{v4})});
        let v4340=(if self.scalar_static_bool[28]{(((v1168*(if self.scalar_static_bool[28]{(v121*(v661*v4258))}else{v4}))-(v1169*(if self.scalar_static_bool[28]{(v121*(v661*((-v4181)/v607)))}else{v4})))/v1173)}else{(if self.scalar_static_bool[27]{v4268}else{v4})});
        let v4341=(if self.scalar_static_bool[28]{((-(v1169*(if self.scalar_static_bool[28]{(v121*(v661*((-v4182)/v607)))}else{v4})))/v1173)}else{(if self.scalar_static_bool[27]{v4265}else{v4})});
        let v4342=(if self.scalar_static_bool[28]{((-(v1169*(if self.scalar_static_bool[28]{(v121*(v661*((-v4176)/v607)))}else{v4})))/v1173)}else{(if self.scalar_static_bool[27]{v4266}else{v4})});
        let v4343=(v1175*v4338);
        let v4344=(v4343+v4343);
        let v4345=(v1175*v4339);
        let v4346=(v4345+v4345);
        let v4347=(v1175*v4340);
        let v4348=(v4347+v4347);
        let v4349=(v1175*v4341);
        let v4350=(v4349+v4349);
        let v4351=(v1175*v4342);
        let v4352=(v4351+v4351);
        let v4353=(v31*v1181);
        let v4354=(v4344/v4353);
        let v4355=(v4346/v4353);
        let v4356=(v4348/v4353);
        let v4357=(v4350/v4353);
        let v4358=(v4352/v4353);
        let v4366=(v1182*v1182);
        let v4402=(v411*(v4202+v4239));
        let v4403=(v411*v4206);
        let v4404=(v411*(v4210+v4243));
        let v4405=(v411*v4247);
        let v4406=(v411*v4251);
        let v4409=((v1191*(if v1185{(v411*(v4338+v4354))}else{(if v1178{((-(v1179*(v4354-v4338)))/v4366)}else{v4})}))+(v1188*v4402));
        let v4412=((v1191*(if v1185{(v411*(v4339+v4355))}else{(if v1178{((-(v1179*(v4355-v4339)))/v4366)}else{v4})}))+(v1188*v4403));
        let v4415=((v1191*(if v1185{(v411*(v4340+v4356))}else{(if v1178{((-(v1179*(v4356-v4340)))/v4366)}else{v4})}))+(v1188*v4404));
        let v4418=((v1191*(if v1185{(v411*(v4341+v4357))}else{(if v1178{((-(v1179*(v4357-v4341)))/v4366)}else{v4})}))+(v1188*v4405));
        let v4421=((v1191*(if v1185{(v411*(v4342+v4358))}else{(if v1178{((-(v1179*(v4358-v4342)))/v4366)}else{v4})}))+(v1188*v4406));
        let v4425=((v1194*v4220)+(v1147*(self.scalar_static_f64[215]*v2751)));
        let v4426=(v1194*v4221);
        let v4427=(v1194*v4222);
        let v4428=(v1194*v4223);
        let v4431=((v755*v2751)+(v436*v3014));
        let v4433=(v436*v3016);
        let v4441=(v1192*v1192);
        let v4443=(v1192*(v436*v3015));
        let v4479=(if v1207{(self.scalar_static_f64[299]+(v1199*((v1209*self.scalar_static_f64[311])/v1210)))}else{(if v1201{(v1199*((v1202*self.scalar_static_f64[309])/v1203))}else{v4})});
        let v4480=(if v1207{(self.scalar_static_f64[0]+(v1199*((v1209*self.scalar_static_f64[312])/v1210)))}else{(if v1201{(v1199*((v1202*self.scalar_static_f64[310])/v1203))}else{v4})});
        let v4532=(v2998/self.scalar_static_f64[138]);
        let v4533=(v2985/self.scalar_static_f64[138]);
        let v4534=(v2984/self.scalar_static_f64[138]);
        let v4544=(if v1255{(v1256*v4532)}else{(if v1252{(v1253*v4532)}else{v4})});
        let v4545=(if v1255{(v1256*v4533)}else{(if v1252{(v1253*v4533)}else{v4479})});
        let v4546=(if v1255{(v1256*v4534)}else{(if v1252{(v1253*v4534)}else{v4480})});
        let v4728=(v703*v2476);
        let v4729=(v4728/self.scalar_static_f64[142]);
        let v4730=(v2985/self.scalar_static_f64[142]);
        let v4731=(v2984/self.scalar_static_f64[142]);
        let v4742=(if v1330{(v1331*v4729)}else{(if v1327{(v1328*v4729)}else{v4544})});
        let v4743=(if v1330{(v1331*v4730)}else{(if v1327{(v1328*v4730)}else{v4545})});
        let v4744=(if v1330{(v1331*v4731)}else{(if v1327{(v1328*v4731)}else{v4})});
        let v4745=(if v1330{v4}else{(if v1327{v4}else{v4546})});
        let v4813=(v2998/self.scalar_static_f64[125]);
        let v4814=(v2985/self.scalar_static_f64[125]);
        let v4815=(v2984/self.scalar_static_f64[125]);
        let v4826=(if v1365{(v1366*v4813)}else{(if v1362{(v1363*v4813)}else{v4742})});
        let v4827=(if v1365{(v1366*v4814)}else{(if v1362{(v1363*v4814)}else{v4743})});
        let v4828=(if v1365{v4}else{(if v1362{v4}else{v4744})});
        let v4829=(if v1365{(v1366*v4815)}else{(if v1362{(v1363*v4815)}else{v4745})});
        let v4836=(v4728/self.scalar_static_f64[159]);
        let v4837=(v2985/self.scalar_static_f64[159]);
        let v4838=(v2984/self.scalar_static_f64[159]);
        let v4849=(if v1377{(v1378*v4836)}else{(if v1374{(v1375*v4836)}else{v4826})});
        let v4850=(if v1377{(v1378*v4837)}else{(if v1374{(v1375*v4837)}else{v4827})});
        let v4851=(if v1377{(v1378*v4838)}else{(if v1374{(v1375*v4838)}else{v4828})});
        let v4852=(if v1377{v4}else{(if v1374{v4}else{v4829})});
        let v4859=(v3017/self.scalar_static_f64[131]);
        let v4860=(v2984/self.scalar_static_f64[131]);
        let v4861=(v3018/self.scalar_static_f64[131]);
        let v4862=(v3019/self.scalar_static_f64[131]);
        let v4863=(v2985/self.scalar_static_f64[131]);
        let v4880=(if v1389{(v1390*v4859)}else{(if v1386{(v1387*v4859)}else{v4849})});
        let v4881=(if v1389{v4}else{(if v1386{v4}else{v4850})});
        let v4882=(if v1389{(v1390*v4860)}else{(if v1386{(v1387*v4860)}else{v4851})});
        let v4883=(if v1389{(v1390*v4861)}else{(if v1386{(v1387*v4861)}else{v4852})});
        let v4884=(if v1389{(v1390*v4862)}else{(if v1386{(v1387*v4862)}else{v4})});
        let v4885=(if v1389{(v1390*v4863)}else{(if v1386{(v1387*v4863)}else{v4})});
        let v4894=(v4728/self.scalar_static_f64[163]);
        let v4895=(v2985/self.scalar_static_f64[163]);
        let v4896=(v2984/self.scalar_static_f64[163]);
        let v4909=(if v1401{(v1402*v4894)}else{(if v1398{(v1399*v4894)}else{v4880})});
        let v4910=(if v1401{(v1402*v4895)}else{(if v1398{(v1399*v4895)}else{v4881})});
        let v4911=(if v1401{(v1402*v4896)}else{(if v1398{(v1399*v4896)}else{v4882})});
        let v4912=(if v1401{v4}else{(if v1398{v4}else{v4883})});
        let v4913=(if v1401{v4}else{(if v1398{v4}else{v4884})});
        let v4914=(if v1401{v4}else{(if v1398{v4}else{v4885})});
        let v5422=((v1140*v3035)+(v765*v4188));
        let v5423=(v1140*v3036);
        let v5424=(v1140*v3037);
        let v5425=(v1140*v3038);
        let v5426=(v1140*v3039);
        let v5427=(v423*(if v802{(v803*v3100)}else{(if v799{(v800*v3100)}else{v4})}));
        let v5428=(v423*(if v802{(v803*v2984)}else{(if v799{(v800*v2984)}else{v4})}));
        let v5429=(v423*(if v802{(v803*v3018)}else{(if v799{(v800*v3018)}else{v4})}));
        let v5430=(v423*(if v802{(v803*v3019)}else{(if v799{(v800*v3019)}else{v4})}));
        let v5431=(v423*(if v802{(v803*v2985)}else{(if v799{(v800*v2985)}else{v4})}));
        let v5433=(v31*v1593);
        let v5442=(v1594*v1594);
        let v5460=(v31*v1597);
        let v5469=(v1598*v1598);
        let v5487=(v31*v2815);
        let v5500=(((v447*(v423*v2815))-(v1603*(self.scalar_static_f64[121]*(v446*(self.scalar_static_f64[123]*v2477)))))/(v447*v447));
        let v5545=(self.scalar_static_f64[230]*v2815);
        let v5560=(v31*v1622);
        let v5569=(v1623*v1623);
        let v5587=(if self.scalar_static_bool[42]{(((v1623*(v1617*v3070))-(v1619*((v1604*v3070)/v5560)))/v5569)}else{v4});
        let v5588=(if self.scalar_static_bool[42]{(((v1623*(v1617*v3071))-(v1619*((v1604*v3071)/v5560)))/v5569)}else{v4});
        let v5589=(if self.scalar_static_bool[42]{(((v1623*((v1618*v5545)+(v1617*v3072)))-(v1619*(((v1604*v3072)+(v785*v5500))/v5560)))/v5569)}else{v4});
        let v5590=(if self.scalar_static_bool[42]{(((v1623*(v1617*v3073))-(v1619*((v1604*v3073)/v5560)))/v5569)}else{v4});
        let v5591=(if self.scalar_static_bool[42]{(((v1623*(v1617*v3074))-(v1619*((v1604*v3074)/v5560)))/v5569)}else{v4});
        let v5596=(if self.scalar_static_bool[44]{((v1628*v2678)+(v327*(self.scalar_static_f64[5]*v2815)))}else{v4});
        let v5609=(if self.scalar_static_bool[44]{(-(if self.scalar_static_bool[44]{((v1633*v2473)+(v119*(-(((v1630*v2476)+(v121*v5596))/v1631))))}else{v4}))}else{v4});
        let v5612=(v1637*self.scalar_static_f64[325]);
        let v5613=(v5612+v5612);
        let v5614=(v1637*self.scalar_static_f64[326]);
        let v5616=(v1637*v5609);
        let v5618=(v1637*self.scalar_static_f64[327]);
        let v5619=(v5618+v5618);
        let v5620=(v1637*self.scalar_static_f64[328]);
        let v5622=(if self.scalar_static_bool[44]{v5613}else{v4});
        let v5623=(if self.scalar_static_bool[44]{(v5614+v5614)}else{v4});
        let v5624=(if self.scalar_static_bool[44]{(v5616+v5616)}else{v4344});
        let v5625=(if self.scalar_static_bool[44]{v4}else{v4346});
        let v5626=(if self.scalar_static_bool[44]{v5613}else{v4348});
        let v5627=(if self.scalar_static_bool[44]{v5619}else{v4350});
        let v5628=(if self.scalar_static_bool[44]{v5619}else{v4352});
        let v5629=(if self.scalar_static_bool[44]{(v5620+v5620)}else{v4});
        let v5630=(if self.scalar_static_bool[44]{v5619}else{v4});
        let v5631=(v31*v1646);
        let v5632=(v5622/v5631);
        let v5633=(v5623/v5631);
        let v5634=(v5624/v5631);
        let v5635=(v5625/v5631);
        let v5636=(v5626/v5631);
        let v5637=(v5627/v5631);
        let v5638=(v5628/v5631);
        let v5639=(v5629/v5631);
        let v5640=(v5630/v5631);
        let v5651=(v1647*v1647);
        let v5703=(if v1651{(v411*(self.scalar_static_f64[325]+v5632))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5632-self.scalar_static_f64[325])))/v5651)}else{v4})});
        let v5704=(if v1651{(v411*(self.scalar_static_f64[326]+v5633))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5633-self.scalar_static_f64[326])))/v5651)}else{v4})});
        let v5705=(if v1651{(v411*(v5609+v5634))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5634-v5609)))/v5651)}else{v4})});
        let v5706=(if v1651{(v411*v5635)}else{(if v1643{((-(self.scalar_static_f64[232]*v5635))/v5651)}else{v4})});
        let v5707=(if v1651{(v411*(self.scalar_static_f64[325]+v5636))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5636-self.scalar_static_f64[325])))/v5651)}else{v4})});
        let v5708=(if v1651{(v411*(self.scalar_static_f64[327]+v5637))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5637-self.scalar_static_f64[327])))/v5651)}else{v4})});
        let v5709=(if v1651{(v411*(self.scalar_static_f64[327]+v5638))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5638-self.scalar_static_f64[327])))/v5651)}else{v4})});
        let v5710=(if v1651{(v411*(self.scalar_static_f64[328]+v5639))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5639-self.scalar_static_f64[328])))/v5651)}else{v4})});
        let v5711=(if v1651{(v411*(self.scalar_static_f64[327]+v5640))}else{(if v1643{((-(self.scalar_static_f64[232]*(v5640-self.scalar_static_f64[327])))/v5651)}else{v4})});
        let v5712=(v327*v5587);
        let v5717=(v327*v5590);
        let v5731=(v1657*v1657);
        let v5774=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5703)-(v1654*(v5703+v5712)))/v5731)}else{v4})});
        let v5775=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5704)-(v1654*(v5704+(v327*v5588))))/v5731)}else{v4})});
        let v5776=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5705)-(v1654*(v5705+(v5596+((v1625*v2678)+(v327*v5589))))))/v5731)}else{v4})});
        let v5777=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5706)-(v1654*v5706))/v5731)}else{v4})});
        let v5778=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5707)-(v1654*(v5707+v5712)))/v5731)}else{v4})});
        let v5779=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5708)-(v1654*(v5708+v5717)))/v5731)}else{v4})});
        let v5780=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5709)-(v1654*(v5709+v5717)))/v5731)}else{v4})});
        let v5781=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5710)-(v1654*(v5710+(v327*v5591))))/v5731)}else{v4})});
        let v5782=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1657*v5711)-(v1654*(v5711+v5717)))/v5731)}else{v4})});
        let v6076=(v1157*v4267);
        let v6078=(v1157*v4257);
        let v6080=(v1157*v4268);
        let v6082=(v1157*v4265);
        let v6084=(v1157*v4266);
        let v6086=(v31*v1723);
        let v6087=((v6076+v6076)/v6086);
        let v6088=((v6078+v6078)/v6086);
        let v6089=((v6080+v6080)/v6086);
        let v6090=((v6082+v6082)/v6086);
        let v6091=((v6084+v6084)/v6086);
        let v6099=(v1724*v1724);
        let v6128=(if v1727{(v411*(v4267+v6087))}else{(if v1721{((-(v1179*(v6087-v4267)))/v6099)}else{v4})});
        let v6129=(if v1727{(v411*(v4257+v6088))}else{(if v1721{((-(v1179*(v6088-v4257)))/v6099)}else{v4})});
        let v6130=(if v1727{(v411*(v4268+v6089))}else{(if v1721{((-(v1179*(v6089-v4268)))/v6099)}else{v4})});
        let v6131=(if v1727{(v411*(v4265+v6090))}else{(if v1721{((-(v1179*(v6090-v4265)))/v6099)}else{v4})});
        let v6132=(if v1727{(v411*(v4266+v6091))}else{(if v1721{((-(v1179*(v6091-v4266)))/v6099)}else{v4})});
        let v7553=(self.scalar_static_f64[274]*v2652);
        let v7561=((v3917-(v2051*v3915))/v3920);
        let v7594=(if v2060{(v3914-((v2064*v3915)+(v1059*((v2062*(-v7561))/v2063))))}else{(if v2053{(-((v2056*v3915)+(v1059*((v2054*v7561)/v2055))))}else{v4})});
        let v7595=(if v2060{(-(v1059*((v2062*v3942)/v2063)))}else{(if v2053{(self.scalar_static_f64[299]-(v1059*((v2054*v3922)/v2055)))}else{v4})});
        let v7596=(if v2060{(-(v1059*((v2062*v3943)/v2063)))}else{(if v2053{(self.scalar_static_f64[0]-(v1059*((v2054*v3923)/v2055)))}else{v4})});
        let v7607=(self.scalar_static_f64[210]*f64::powf(v2070,self.scalar_static_f64[303]));
        let v7642=((v626*v2754)+(v441*v2941));
        let v7643=(v411*v7642);
        let v7651=((v2083*v6128)+(v1730*((v2082*v4202)+(v1145*v7643))));
        let v7654=((v2083*v6129)+(v1730*(v2082*v4206)));
        let v7657=((v2083*v6130)+(v1730*(v2082*v4210)));
        let v7658=(v2083*v6131);
        let v7659=(v2083*v6132);
        let v7668=((v2085*v6128)+(v1730*((v2082*v4239)+(v1152*v7643))));
        let v7669=(v2085*v6129);
        let v7672=((v2085*v6130)+(v1730*(v2082*v4243)));
        let v7675=((v2085*v6131)+(v1730*(v2082*v4247)));
        let v7678=((v2085*v6132)+(v1730*(v2082*v4251)));
        let v7680=(v1004*(-v4018));
        let v7683=(v1004*v1004);
        let v7684=((v7680-(v2087*v3736))/v7683);
        let v7685=(self.scalar_static_f64[0]/v1004);
        let v7686=(self.scalar_static_f64[300]/v1004);
        let v7687=(self.scalar_static_f64[301]/v1004);
        let v7688=(self.scalar_static_f64[299]/v1004);
        let v7718=(-v7686);
        let v7719=(-v7687);
        let v7720=(-v7688);
        let v7743=(if v2096{(v4018-((v2100*v3736)+(v1004*((v2098*(-v7684))/v2099))))}else{(if v2089{(-((v2092*v3736)+(v1004*((v2090*v7684)/v2091))))}else{v4})});
        let v7744=(if v2096{(-(v1004*((v2098*(-v7685))/v2099)))}else{(if v2089{(self.scalar_static_f64[0]-(v1004*((v2090*v7685)/v2091)))}else{v4})});
        let v7745=(if v2096{(-(v1004*((v2098*v7718)/v2099)))}else{(if v2089{(self.scalar_static_f64[300]-(v1004*((v2090*v7686)/v2091)))}else{v4})});
        let v7746=(if v2096{(-(v1004*((v2098*v7719)/v2099)))}else{(if v2089{(self.scalar_static_f64[301]-(v1004*((v2090*v7687)/v2091)))}else{v4})});
        let v7747=(if v2096{(-(v1004*((v2098*v7720)/v2099)))}else{(if v2089{(self.scalar_static_f64[299]-(v1004*((v2090*v7688)/v2091)))}else{v4})});
        let v7762=(self.scalar_static_f64[214]*f64::powf(v2105,self.scalar_static_f64[308]));
        let v7805=(v301*self.scalar_static_f64[300]);
        let v7806=(v301*self.scalar_static_f64[301]);
        let v7829=(self.scalar_static_f64[302]/v1004);
        let v7832=((v7680-(v2119*v3736))/v7683);
        let v7884=(if v2128{(-(v1004*((v2130*v7718)/v2131)))}else{(if v2121{(self.scalar_static_f64[300]-(v1004*((v2122*v7686)/v2123)))}else{v4})});
        let v7885=(if v2128{(-(v1004*((v2130*(-v7829))/v2131)))}else{(if v2121{(self.scalar_static_f64[302]-(v1004*((v2122*v7829)/v2123)))}else{v4})});
        let v7886=(if v2128{(v4018-((v2132*v3736)+(v1004*((v2130*(-v7832))/v2131))))}else{(if v2121{(-((v2124*v3736)+(v1004*((v2122*v7832)/v2123))))}else{v4})});
        let v7887=(if v2128{(-(v1004*((v2130*v7719)/v2131)))}else{(if v2121{(self.scalar_static_f64[301]-(v1004*((v2122*v7687)/v2123)))}else{v4})});
        let v7888=(if v2128{(-(v1004*((v2130*v7720)/v2131)))}else{(if v2121{(self.scalar_static_f64[299]-(v1004*((v2122*v7688)/v2123)))}else{v4})});
        let v7903=(self.scalar_static_f64[214]*f64::powf(v2137,self.scalar_static_f64[308]));
        let v7964=(self.scalar_static_f64[5]*(self.scalar_static_f64[276]*(v300*(v7805+(v1099*((v1125*(-((-(v7884/v260))*v7903)))+(v1100*(self.scalar_static_f64[300]-v7884))))))));
        let v7967=(self.scalar_static_f64[5]*(self.scalar_static_f64[276]*(v300*(v7806+(v1099*((v1125*(-((-(v7887/v260))*v7903)))+(v1100*(self.scalar_static_f64[301]-v7887))))))));
        let v7983=(self.scalar_static_f64[277]*v2473);
        let v7986=(v2156*v2156);
        let v7987=((-(v700*v7983))/v7986);
        let v7988=(self.scalar_static_f64[299]/v2156);
        let v7989=(self.scalar_static_f64[0]/v2156);
        let v8010=((v2166*((v2154*((v620*v2754)+(v441*((v619*(self.scalar_static_f64[173]*(v614*(self.scalar_static_f64[174]*v2477))))+(v615*(v619*(self.scalar_static_f64[176]*v2476)))))))+(v2150*((((v441*v2751)-(v436*v2754))/v4187)*(self.scalar_static_f64[278]*f64::powf(v2151,self.scalar_static_f64[347]))))))+(v2155*(if v2161{(v2162*v7987)}else{(if v2158{(v2159*v7987)}else{v4909})})));
        let v8011=(v2155*(if v2161{(v2162*v7988)}else{(if v2158{(v2159*v7988)}else{v4910})}));
        let v8012=(v2155*(if v2161{v4}else{(if v2158{v4}else{v4911})}));
        let v8013=(v2155*(if v2161{(v2162*v7989)}else{(if v2158{(v2159*v7989)}else{v4912})}));
        let v8014=(v2155*(if v2161{v4}else{(if v2158{v4}else{v4913})}));
        let v8015=(v2155*(if v2161{v4}else{(if v2158{v4}else{v4914})}));
        let v8023=(((v339*((v2168*v2473)+(v119*(v423*v2944))))-(v2169*v2685))/v3216);
        let v8073=(v633*v633);
        let v8084=(-(if v233{((v237*v2473)+(v119*((v235*(-v2578))/v236)))}else{(if v226{(v2573+((v229*v2473)+(v119*((v227*v2578)/v228))))}else{v4})}));
        let v8092=((v2186*v2476)+(v121*(v8084/self.scalar_static_f64[280])));
        let v8093=(v121*self.scalar_static_f64[348]);
        let v8094=(v121*self.scalar_static_f64[349]);
        let v8095=(v121*self.scalar_static_f64[350]);
        let v8096=(v121*self.scalar_static_f64[351]);
        let v8132=(v31*v2204);
        let v8141=(v2205*v2205);
        let v8159=(if self.scalar_static_bool[60]{(((v2205*((v2200*v3035)+(v765*((v1600*v2950)+(v642*v5487)))))-(v2201*((v423*(if v2194{(v2195*v8092)}else{(if v2190{(v2191*v8092)}else{v4})}))/v8132)))/v8141)}else{(if self.scalar_static_bool[59]{(((v633*((v2180*(v411*v2947))+(v2177*(((v2081*(((v1594*(v5422-v4188))-(v1591*(v5422/v5433)))/v5442))+(v1595*v7642))+((v2170*(((v1598*v5427)-(v1590*(v5427/v5460)))/v5469))+(v1599*v8023))))))-(v2181*v2945))/v8073)}else{v4})});
        let v8160=(if self.scalar_static_bool[60]{(((v2205*(v2200*v3036))-(v2201*((v423*(if v2194{(v2195*v8093)}else{(if v2190{(v2191*v8093)}else{v4})}))/v8132)))/v8141)}else{(if self.scalar_static_bool[59]{((v2177*((v2081*(((v1594*v5423)-(v1591*(v5423/v5433)))/v5442))+(v2170*(((v1598*v5428)-(v1590*(v5428/v5460)))/v5469))))/v633)}else{v4})});
        let v8161=(if self.scalar_static_bool[60]{(((v2205*(v2200*v3037))-(v2201*((v423*(if v2194{(v2195*v8094)}else{(if v2190{(v2191*v8094)}else{v4})}))/v8132)))/v8141)}else{(if self.scalar_static_bool[59]{((v2177*((v2081*(((v1594*v5424)-(v1591*(v5424/v5433)))/v5442))+(v2170*(((v1598*v5429)-(v1590*(v5429/v5460)))/v5469))))/v633)}else{v4})});
        let v8162=(if self.scalar_static_bool[60]{(((v2205*(v2200*v3038))-(v2201*((v423*(if v2194{(v2195*v8095)}else{(if v2190{(v2191*v8095)}else{v4})}))/v8132)))/v8141)}else{(if self.scalar_static_bool[59]{((v2177*((v2081*(((v1594*v5425)-(v1591*(v5425/v5433)))/v5442))+(v2170*(((v1598*v5430)-(v1590*(v5430/v5460)))/v5469))))/v633)}else{v4})});
        let v8163=(if self.scalar_static_bool[60]{(((v2205*(v2200*v3039))-(v2201*((v423*(if v2194{(v2195*v8096)}else{(if v2190{(v2191*v8096)}else{v4})}))/v8132)))/v8141)}else{(if self.scalar_static_bool[59]{((v2177*((v2081*(((v1594*v5426)-(v1591*(v5426/v5433)))/v5442))+(v2170*(((v1598*v5431)-(v1590*(v5431/v5460)))/v5469))))/v633)}else{v4})});
        let v8181=(if self.scalar_static_bool[64]{(v1140*v3070)}else{v4});
        let v8182=(if self.scalar_static_bool[64]{(v1140*v3071)}else{v4});
        let v8183=(if self.scalar_static_bool[64]{((v1140*v3072)+(v785*v4188))}else{v4});
        let v8184=(if self.scalar_static_bool[64]{(v1140*v3073)}else{v4});
        let v8185=(if self.scalar_static_bool[64]{(v1140*v3074)}else{v4});
        let v8187=(v31*v2218);
        let v8196=(v2219*v2219);
        let v8224=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*v3018)}else{(if v788{(v789*v3018)}else{v4})}))}else{v4});
        let v8225=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*v3053)}else{(if v788{(v789*v3053)}else{v4})}))}else{v4});
        let v8226=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*v3078)}else{(if v788{(v789*v3078)}else{v4})}))}else{v4});
        let v8227=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*v3019)}else{(if v788{(v789*v3019)}else{v4})}))}else{v4});
        let v8228=(if self.scalar_static_bool[64]{(v423*(if v791{(v792*v2985)}else{(if v788{(v789*v2985)}else{v4})}))}else{v4});
        let v8229=(v31*v2225);
        let v8238=(v2226*v2226);
        let v8303=((v2237*v2476)+(v121*v8084));
        let v8339=(v31*v2255);
        let v8348=(v2256*v2256);
        let v8372=(v1662*(if self.scalar_static_bool[65]{(((v2256*(v2251*v3070))-(v2252*((v423*(if v2245{(v2246*v3018)}else{(if v2241{(v2242*v3018)}else{v4})}))/v8339)))/v8348)}else{(if self.scalar_static_bool[64]{((v2230*((v2081*(if self.scalar_static_bool[64]{(((v2219*v8181)-(v2216*(v8181/v8187)))/v8196)}else{v4}))+(v2170*(if self.scalar_static_bool[64]{(((v2226*v8224)-(v2223*(v8224/v8229)))/v8238)}else{v4}))))/v633)}else{v4})}));
        let v8384=(v1662*(if self.scalar_static_bool[65]{(((v2256*(v2251*v3073))-(v2252*((v423*(if v2245{(v2246*v3019)}else{(if v2241{(v2242*v3019)}else{v4})}))/v8339)))/v8348)}else{(if self.scalar_static_bool[64]{((v2230*((v2081*(if self.scalar_static_bool[64]{(((v2219*v8184)-(v2216*(v8184/v8187)))/v8196)}else{v4}))+(v2170*(if self.scalar_static_bool[64]{(((v2226*v8227)-(v2223*(v8227/v8229)))/v8238)}else{v4}))))/v633)}else{v4})}));
        let v8404=(self.scalar_static_f64[283]*f64::powf(v1078,self.scalar_static_f64[352]));
        let v8411=(if self.scalar_static_bool[66]{v3921}else{v4});
        let v8412=(if self.scalar_static_bool[66]{v3922}else{v4});
        let v8413=(if self.scalar_static_bool[66]{v3923}else{v4});
        let v8418=(v2271*v2271);
        let v8430=(v2277*(-v8411));
        let v8431=(v2277*(-v8412));
        let v8432=(v2277*(-v8413));
        let v8436=(v2278*v2278);
        let v8482=(v1143*v1143);
        let v8538=(if self.scalar_static_bool[66]{(v8014/v2156)}else{v4});
        let v8584=(self.scalar_static_f64[284]*v8014);
        let v8591=(if self.scalar_static_bool[66]{(v7651+(self.scalar_static_f64[284]*v8010))}else{v4});
        let v8592=(if self.scalar_static_bool[66]{(v7654+(self.scalar_static_f64[284]*v8011))}else{v4});
        let v8593=(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v8012)}else{v4});
        let v8594=(if self.scalar_static_bool[66]{(v7657+(self.scalar_static_f64[284]*v8013))}else{v4});
        let v8595=(if self.scalar_static_bool[66]{(v7658+v8584)}else{v4});
        let v8596=(if self.scalar_static_bool[66]{(v7659+v8584)}else{v4});
        let v8597=(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v8015)}else{v4});
        let v8631=(if self.scalar_static_bool[67]{v7651}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8591)}else{v4})});
        let v8632=(if self.scalar_static_bool[67]{v7654}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8592)}else{v4})});
        let v8633=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8593)}else{v4})});
        let v8634=(if self.scalar_static_bool[67]{v7657}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8594)}else{v4})});
        let v8635=(if self.scalar_static_bool[67]{v7658}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8595)}else{v4})});
        let v8636=(if self.scalar_static_bool[67]{v7659}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8596)}else{v4})});
        let v8637=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[287]*v8597)}else{v4})});
        let v8638=(if self.scalar_static_bool[67]{v7668}else{(if self.scalar_static_bool[66]{(v7668+(self.scalar_static_f64[286]*v8591))}else{v4})});
        let v8639=(if self.scalar_static_bool[67]{v7669}else{(if self.scalar_static_bool[66]{(v7669+(self.scalar_static_f64[286]*v8592))}else{v4})});
        let v8640=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[286]*v8593)}else{v4})});
        let v8641=(if self.scalar_static_bool[67]{v7672}else{(if self.scalar_static_bool[66]{(v7672+(self.scalar_static_f64[286]*v8594))}else{v4})});
        let v8642=(if self.scalar_static_bool[67]{v7675}else{(if self.scalar_static_bool[66]{(v7675+(self.scalar_static_f64[286]*v8595))}else{v4})});
        let v8643=(if self.scalar_static_bool[67]{v7678}else{(if self.scalar_static_bool[66]{(v7678+(self.scalar_static_f64[286]*v8596))}else{v4})});
        let v8644=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[286]*v8597)}else{v4})});
        let v8649=(if self.scalar_static_bool[67]{v8014}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8014)}else{v4})});
        let v8690=(v2356*v2356);
        let v8749=(if v2368{((v2369*v4409)+(v1192*((v1730*v2941)+(v626*v6128))))}else{(if v2364{(((v2356*(v8631+v8638))-(v2365*(((v1192*(v4425+v4431))-(v2355*v4409))/v4441)))/v8690)}else{v4})});
        let v8750=(if v2368{((v2369*v4412)+(v1192*(v626*v6129)))}else{(if v2364{(((v2356*(v8632+v8639))-(v2365*((v4443-(v2355*v4412))/v4441)))/v8690)}else{v4})});
        let v8751=(if v2368{v4}else{(if v2364{((v8633+v8640)/v2356)}else{v4})});
        let v8752=(if v2368{((v2369*v4415)+(v1192*(v626*v6130)))}else{(if v2364{(((v2356*(v8634+v8641))-(v2365*(((v1192*(v4426+v4433))-(v2355*v4415))/v4441)))/v8690)}else{v4})});
        let v8753=(if v2368{((v2369*v4418)+(v1192*(v626*v6131)))}else{(if v2364{(((v2356*(v8635+v8642))-(v2365*(((v1192*v4427)-(v2355*v4418))/v4441)))/v8690)}else{v4})});
        let v8754=(if v2368{((v2369*v4421)+(v1192*(v626*v6132)))}else{(if v2364{(((v2356*(v8636+v8643))-(v2365*(((v1192*v4428)-(v2355*v4421))/v4441)))/v8690)}else{v4})});
        let v8755=(if v2368{v4}else{(if v2364{((v8637+v8644)/v2356)}else{v4})});
        let v8784=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8749)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8749)}else{v4})})});
        let v8785=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8750)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8750)}else{v4})})});
        let v8786=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8751)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8751)}else{v4})})});
        let v8787=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8752)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8752)}else{v4})})});
        let v8788=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8753)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8753)}else{v4})})});
        let v8789=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8754)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8754)}else{v4})})});
        let v8790=(if self.scalar_static_bool[85]{v4}else{(if self.scalar_static_bool[83]{(self.scalar_static_f64[294]*v8755)}else{(if self.scalar_static_bool[80]{(self.scalar_static_f64[286]*v8755)}else{v4})})});
        let v8826=((self.scalar_static_f64[5]*(self.scalar_static_f64[276]*((v2146*v2663)+(v300*(((v2143*v4005)+(v1099*(((v2139*v4104)+(v1125*(-((-(((v260*v7886)-(v2135*v2614))/v2642))*v7903))))+((v2141*v4010)+(v1100*(-v7886))))))+(v732*v2664))))))+(if self.scalar_static_bool[63]{((v2258*v5776)+(v1662*(if self.scalar_static_bool[65]{(((v2256*((v2251*v3072)+(v785*((v1617*v2950)+(v642*v5545)))))-(v2252*((v423*(if v2245{(v2246*v8303)}else{(if v2241{(v2242*v8303)}else{v4})}))/v8339)))/v8348)}else{(if self.scalar_static_bool[64]{(((v633*((v2233*(self.scalar_static_f64[281]*v2947))+(v2230*(((v2221*v7642)+(v2081*(if self.scalar_static_bool[64]{(((v2219*(v8183-v4188))-(v2216*(v8183/v8187)))/v8196)}else{v4})))+((v2228*v8023)+(v2170*(if self.scalar_static_bool[64]{(((v2226*v8226)-(v2223*(v8226/v8229)))/v8238)}else{v4})))))))-(v2234*v2945))/v8073)}else{v4})})))}else{v4}));
        let v8986=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8010}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8010)}else{v4})})+(((v2049*v3990)+(v1086*v7553))+v8631)));
        let v8987=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8011}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8011)}else{v4})})+((v2049*v3991)+v8632)));
        let v8988=(self.scalar_static_f64[0]*(v8633+(if self.scalar_static_bool[67]{v8012}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8012)}else{v4})})));
        let v8989=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v8013}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8013)}else{v4})})+((v2049*v3992)+v8634)));
        let v8990=(self.scalar_static_f64[0]*(v8635+v8649));
        let v8991=(self.scalar_static_f64[0]*(v8636+v8649));
        let v8992=(self.scalar_static_f64[0]*(v8637+(if self.scalar_static_bool[67]{v8015}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[285]*v8015)}else{v4})})));
        let v9007=(self.scalar_static_f64[0]*((v2076*(self.scalar_static_f64[273]*v2652))+(v2068*(((v2072*v3975)+(v1081*(-((-((v2067*v2640)+(v284*v7594)))*v7607))))+(v170*(-v7594))))));
        let v9008=(self.scalar_static_f64[0]*(v2068*((v1081*(-((-(v284*v7595))*v7607)))+(v170*(self.scalar_static_f64[299]-v7595)))));
        let v9009=(self.scalar_static_f64[0]*(v2068*((v1081*(-((-(v284*v7596))*v7607)))+(v170*(self.scalar_static_f64[0]-v7596)))));
        let v9016=(self.scalar_static_f64[0]*(((v2173*((v2171*v3886)+(v1048*(v411*v8023))))+(v2172*v3834))+(((v2079*v4180)+(v1138*(self.scalar_static_f64[275]*v2663)))+v8638)));
        let v9017=(self.scalar_static_f64[0]*v8639);
        let v9018=(self.scalar_static_f64[0]*v8640);
        let v9019=(self.scalar_static_f64[0]*(((v2173*(v2171*v3887))+(v2172*v3835))+((v2079*v4181)+v8641)));
        let v9020=(self.scalar_static_f64[0]*(((v2173*(v2171*v3888))+(v2172*v3836))+((v2079*v4182)+v8642)));
        let v9021=(self.scalar_static_f64[0]*(((v2173*(v2171*v3889))+(v2172*v3829))+((v2079*v4176)+v8643)));
        let v9022=(self.scalar_static_f64[0]*v8644);
        let v9037=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*((if self.scalar_static_bool[66]{(((v2156*v8010)-(v2167*v7983))/v7986)}else{v4})+((if self.scalar_static_bool[66]{((v2283*v7553)+(v2049*(if self.scalar_static_bool[66]{((v2280*(if self.scalar_static_bool[66]{(v3966*v8404)}else{v4}))+(v2266*(if v2275{(((v2278*v8430)-(v2277*v8430))/v8436)}else{(if v2269{((-(v2270*v8411))/v8418)}else{v4})})))}else{v4})))}else{v4})+(if self.scalar_static_bool[66]{((v2291*(if self.scalar_static_bool[66]{((v2288*(((v371*((v1141*v2476)+(v121*v4191)))-(v2286*v2702))/v2739))+(v2287*((-(v411*v4195))/v8482)))}else{v4}))+(v2290*((v2082*v6128)+(v1730*v7643))))}else{v4}))))}else{v4}));
        let v9038=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*((if self.scalar_static_bool[66]{(v8011/v2156)}else{v4})+((if self.scalar_static_bool[66]{(v2049*(if self.scalar_static_bool[66]{((v2280*(if self.scalar_static_bool[66]{(v3967*v8404)}else{v4}))+(v2266*(if v2275{(((v2278*v8431)-(v2277*v8431))/v8436)}else{(if v2269{((-(v2270*v8412))/v8418)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[66]{((v2291*(if self.scalar_static_bool[66]{((v2288*((v121*v4192)/v371))+(v2287*((-(v411*v4196))/v8482)))}else{v4}))+(v2290*(v2082*v6129)))}else{v4}))))}else{v4}));
        let v9039=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{((v2298*self.scalar_static_f64[353])+(v2296*(if self.scalar_static_bool[66]{(v8012/v2156)}else{v4})))}else{v4}));
        let v9040=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{((v2298*self.scalar_static_f64[354])+(v2296*((if self.scalar_static_bool[66]{(v8013/v2156)}else{v4})+((if self.scalar_static_bool[66]{(v2049*(if self.scalar_static_bool[66]{((v2280*(if self.scalar_static_bool[66]{(v3968*v8404)}else{v4}))+(v2266*(if v2275{(((v2278*v8432)-(v2277*v8432))/v8436)}else{(if v2269{((-(v2270*v8413))/v8418)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[66]{((v2291*(if self.scalar_static_bool[66]{((v2288*((v121*v4193)/v371))+(v2287*((-(v411*v4197))/v8482)))}else{v4}))+(v2290*(v2082*v6130)))}else{v4})))))}else{v4}));
        let v9041=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*((if self.scalar_static_bool[66]{(v2290*(v2082*v6131))}else{v4})+v8538))}else{v4}));
        let v9042=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*((if self.scalar_static_bool[66]{(v2290*(v2082*v6132))}else{v4})+v8538))}else{v4}));
        let v9043=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2296*(if self.scalar_static_bool[66]{(v8015/v2156)}else{v4}))}else{v4}));
        let v9102=(self.scalar_static_f64[0]*(v7964+(if self.scalar_static_bool[63]{((v2258*v5774)+v8372)}else{v4})));
        let v9103=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[276]*(v300*((v1099*((v1125*(-((-(v7885/v260))*v7903)))+(v1100*(self.scalar_static_f64[302]-v7885))))+(v301*self.scalar_static_f64[302])))))+(if self.scalar_static_bool[63]{((v2258*v5775)+(v1662*(if self.scalar_static_bool[65]{(((v2256*(v2251*v3071))-(v2252*((v423*(if v2245{(v2246*v3053)}else{(if v2241{(v2242*v3053)}else{v4})}))/v8339)))/v8348)}else{(if self.scalar_static_bool[64]{((v2230*((v2081*(if self.scalar_static_bool[64]{(((v2219*v8182)-(v2216*(v8182/v8187)))/v8196)}else{v4}))+(v2170*(if self.scalar_static_bool[64]{(((v2226*v8225)-(v2223*(v8225/v8229)))/v8238)}else{v4}))))/v633)}else{v4})})))}else{v4})));
        let v9104=(self.scalar_static_f64[0]*v8826);
        let v9105=(self.scalar_static_f64[0]*(if self.scalar_static_bool[63]{(v2258*v5777)}else{v4}));
        let v9106=(self.scalar_static_f64[0]*(v7964+(if self.scalar_static_bool[63]{(v8372+(v2258*v5778))}else{v4})));
        let v9107=(self.scalar_static_f64[0]*(v7967+(if self.scalar_static_bool[63]{((v2258*v5779)+v8384)}else{v4})));
        let v9108=(self.scalar_static_f64[0]*(v7967+(if self.scalar_static_bool[63]{(v8384+(v2258*v5780))}else{v4})));
        let v9109=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[276]*(v300*(v4179+(v1099*((v1125*(-((-(v7888/v260))*v7903)))+(v1100*(self.scalar_static_f64[299]-v7888))))))))+(if self.scalar_static_bool[63]{((v2258*v5781)+(v1662*(if self.scalar_static_bool[65]{(((v2256*(v2251*v3074))-(v2252*((v423*(if v2245{(v2246*v2985)}else{(if v2241{(v2242*v2985)}else{v4})}))/v8339)))/v8348)}else{(if self.scalar_static_bool[64]{((v2230*((v2081*(if self.scalar_static_bool[64]{(((v2219*v8185)-(v2216*(v8185/v8187)))/v8196)}else{v4}))+(v2170*(if self.scalar_static_bool[64]{(((v2226*v8228)-(v2223*(v8228/v8229)))/v8238)}else{v4}))))/v633)}else{v4})})))}else{v4})));
        let v9110=(self.scalar_static_f64[0]*(v7967+(if self.scalar_static_bool[63]{(v8384+(v2258*v5782))}else{v4})));
        let v9154=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[276]*((v2114*v2663)+(v300*(((v2111*v4005)+(v1099*(((v2107*v4104)+(v1125*(-((-(((v260*v7743)-(v2103*v2614))/v2642))*v7762))))+((v2109*v4010)+(v1100*(-v7743))))))+(v727*v2664))))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v8159)}else{v8159})));
        let v9155=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[276]*(v300*(v4178+(v1099*((v1125*(-((-(v7744/v260))*v7762)))+(v1100*(self.scalar_static_f64[0]-v7744))))))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v8160)}else{v8160})));
        let v9156=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[276]*(v300*((v1099*((v1125*(-((-(v7745/v260))*v7762)))+(v1100*(self.scalar_static_f64[300]-v7745))))+v7805))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v8161)}else{v8161})));
        let v9157=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[276]*(v300*((v1099*((v1125*(-((-(v7746/v260))*v7762)))+(v1100*(self.scalar_static_f64[301]-v7746))))+v7806))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v8162)}else{v8162})));
        let v9158=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[276]*(v300*(v4179+(v1099*((v1125*(-((-(v7747/v260))*v7762)))+(v1100*(self.scalar_static_f64[299]-v7747))))))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v8163)}else{v8163})));

        CommonStampValues {
            v1,
            v4,
            v30,
            v31,
            v46,
            v101,
            v116,
            v117,
            v119,
            v121,
            v123,
            v124,
            v125,
            v126,
            v127,
            v128,
            v133,
            v134,
            v135,
            v140,
            v142,
            v143,
            v147,
            v148,
            v149,
            v150,
            v155,
            v156,
            v157,
            v162,
            v164,
            v165,
            v169,
            v170,
            v196,
            v219,
            v260,
            v269,
            v270,
            v271,
            v272,
            v276,
            v278,
            v279,
            v280,
            v284,
            v285,
            v287,
            v288,
            v289,
            v327,
            v408,
            v410,
            v411,
            v412,
            v414,
            v415,
            v418,
            v421,
            v423,
            v436,
            v449,
            v558,
            v559,
            v560,
            v561,
            v563,
            v564,
            v565,
            v567,
            v570,
            v581,
            v582,
            v583,
            v585,
            v586,
            v587,
            v589,
            v592,
            v694,
            v697,
            v698,
            v700,
            v703,
            v705,
            v708,
            v713,
            v721,
            v724,
            v727,
            v731,
            v732,
            v765,
            v766,
            v767,
            v770,
            v771,
            v849,
            v862,
            v967,
            v1024,
            v1048,
            v1051,
            v1054,
            v1080,
            v1156,
            v1191,
            v1192,
            v1197,
            v1198,
            v1216,
            v1217,
            v1220,
            v1221,
            v1230,
            v1260,
            v1261,
            v1262,
            v1263,
            v1268,
            v1269,
            v1276,
            v1277,
            v1278,
            v1283,
            v1285,
            v1335,
            v1336,
            v1337,
            v1338,
            v1343,
            v1344,
            v1370,
            v1382,
            v1394,
            v1406,
            v1412,
            v1413,
            v1415,
            v1416,
            v1417,
            v1422,
            v1423,
            v1429,
            v1433,
            v1436,
            v1444,
            v1445,
            v1446,
            v1448,
            v1450,
            v1452,
            v1453,
            v1454,
            v1455,
            v1457,
            v1459,
            v1460,
            v1461,
            v1466,
            v1467,
            v1504,
            v1506,
            v1508,
            v1509,
            v1511,
            v1512,
            v1513,
            v1518,
            v1519,
            v1524,
            v1527,
            v1529,
            v1537,
            v1538,
            v1539,
            v1541,
            v1544,
            v1545,
            v1546,
            v1547,
            v1549,
            v1550,
            v1551,
            v1552,
            v1557,
            v1558,
            v1600,
            v1604,
            v1625,
            v1641,
            v1662,
            v1730,
            v1740,
            v1750,
            v1751,
            v1752,
            v1755,
            v1756,
            v1760,
            v1761,
            v1763,
            v1764,
            v1766,
            v1767,
            v1768,
            v1773,
            v1774,
            v1787,
            v1891,
            v1892,
            v1894,
            v1896,
            v1898,
            v1900,
            v1901,
            v1903,
            v1911,
            v1913,
            v1914,
            v1915,
            v1921,
            v1923,
            v1924,
            v1928,
            v1930,
            v1932,
            v1933,
            v1934,
            v1939,
            v1940,
            v1995,
            v2320,
            v2356,
            v2384,
            v2420,
            v2423,
            v2426,
            v2429,
            v2433,
            v2437,
            v2445,
            v2451,
            v2462,
            v2471,
            v2472,
            v2473,
            v2476,
            v2477,
            v2547,
            v2570,
            v2614,
            v2618,
            v2623,
            v2640,
            v2642,
            v2647,
            v2678,
            v2721,
            v2723,
            v2751,
            v2847,
            v2922,
            v2984,
            v2985,
            v3035,
            v3036,
            v3037,
            v3038,
            v3039,
            v3217,
            v3218,
            v3219,
            v3220,
            v3227,
            v3619,
            v3620,
            v3621,
            v3622,
            v3830,
            v3831,
            v3832,
            v3833,
            v3886,
            v3887,
            v3888,
            v3889,
            v3898,
            v3899,
            v3900,
            v3901,
            v3910,
            v3911,
            v3912,
            v3913,
            v3972,
            v3973,
            v3974,
            v4263,
            v4264,
            v4265,
            v4266,
            v4402,
            v4403,
            v4404,
            v4405,
            v4406,
            v4409,
            v4412,
            v4415,
            v4418,
            v4421,
            v4425,
            v4426,
            v4427,
            v4428,
            v4431,
            v4433,
            v4441,
            v4443,
            v4479,
            v4480,
            v4544,
            v4545,
            v4546,
            v4742,
            v4743,
            v4744,
            v4745,
            v4826,
            v4827,
            v4828,
            v4829,
            v4849,
            v4850,
            v4851,
            v4852,
            v4880,
            v4881,
            v4882,
            v4883,
            v4884,
            v4885,
            v4909,
            v4910,
            v4911,
            v4912,
            v4913,
            v4914,
            v5487,
            v5500,
            v5587,
            v5588,
            v5589,
            v5590,
            v5591,
            v5622,
            v5623,
            v5624,
            v5625,
            v5626,
            v5627,
            v5628,
            v5629,
            v5630,
            v5774,
            v5775,
            v5776,
            v5777,
            v5778,
            v5779,
            v5780,
            v5781,
            v5782,
            v6128,
            v6129,
            v6130,
            v6131,
            v6132,
            v8784,
            v8785,
            v8786,
            v8787,
            v8788,
            v8789,
            v8790,
            v8986,
            v8987,
            v8988,
            v8989,
            v8990,
            v8991,
            v8992,
            v9007,
            v9008,
            v9009,
            v9016,
            v9017,
            v9018,
            v9019,
            v9020,
            v9021,
            v9022,
            v9037,
            v9038,
            v9039,
            v9040,
            v9041,
            v9042,
            v9043,
            v9102,
            v9103,
            v9104,
            v9105,
            v9106,
            v9107,
            v9108,
            v9109,
            v9110,
            v9154,
            v9155,
            v9156,
            v9157,
            v9158,
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
        let v305=((common.v125*self.scalar_static_f64[93])).exp();
        let v306=(self.scalar_static_f64[92]*v305);
        let v307=(v306<self.scalar_static_f64[14]);
        let v308=(if v307{self.scalar_static_f64[14]}else{v306});
        let v314=((common.v125*self.scalar_static_f64[97])).exp();
        let v315=(self.scalar_static_f64[94]*v314);
        let v319=((common.v125*self.scalar_static_f64[99])).exp();
        let v320=(self.scalar_static_f64[98]*v319);
        let v321=(v320<self.scalar_static_f64[14]);
        let v322=(if v321{self.scalar_static_f64[14]}else{v320});
        let v331=((common.v125*self.scalar_static_f64[103])).exp();
        let v332=(self.scalar_static_f64[102]*v331);
        let v334=(v331*self.scalar_static_f64[104]);
        let v454=((common.v125*self.scalar_static_f64[127])).exp();
        let v455=(self.scalar_static_f64[124]*v454);
        let v458=(common.v123*self.scalar_static_f64[129]);
        let v460=((v458/self.scalar_static_f64[125])).exp();
        let v461=(v455*v460);
        let v467=((common.v125*self.scalar_static_f64[133])).exp();
        let v468=(self.scalar_static_f64[130]*v467);
        let v472=(((common.v123*self.scalar_static_f64[134])/self.scalar_static_f64[131])).exp();
        let v473=(v468*v472);
        let v477=(common.v125*self.scalar_static_f64[137]);
        let v480=((v477/self.scalar_static_f64[138])).exp();
        let v481=(self.scalar_static_f64[135]*v480);
        let v484=(common.v123*self.scalar_static_f64[140]);
        let v486=((v484/self.scalar_static_f64[138])).exp();
        let v487=(v481*v486);
        let v491=((v477/self.scalar_static_f64[142])).exp();
        let v492=(self.scalar_static_f64[141]*v491);
        let v494=((v484/self.scalar_static_f64[142])).exp();
        let v495=(v492*v494);
        let v503=(((common.v123*self.scalar_static_f64[146])/self.scalar_static_f64[138])).exp();
        let v510=((common.v123*self.scalar_static_f64[149])).exp();
        let v512=(if self.scalar_static_bool[12]{(self.scalar_static_f64[147]*v510)}else{common.v4});
        let v518=(((common.v123*self.scalar_static_f64[152])/self.scalar_static_f64[142])).exp();
        let v537=((common.v125*self.scalar_static_f64[161])).exp();
        let v538=(self.scalar_static_f64[158]*v537);
        let v540=((v458/self.scalar_static_f64[159])).exp();
        let v541=(v538*v540);
        let v546=((common.v125*self.scalar_static_f64[164])).exp();
        let v547=(self.scalar_static_f64[162]*v546);
        let v549=((v458/self.scalar_static_f64[163])).exp();
        let v550=(v547*v549);
        let v552=(common.v117).sqrt();
        let v553=(self.scalar_static_f64[165]*v552);
        let v556=((common.v124*self.scalar_static_f64[166])).exp();
        let v557=(v553*v556);
        let v572=(common.v560*self.scalar_static_f64[168]);
        let v573=(common.v196*v572);
        let v576=(self.scalar_static_f64[45]*(self.scalar_static_f64[45]*(common.v196*v573)));
        let v577=(common.v287*v576);
        let v579=((self.scalar_static_f64[167]-common.v570)).exp();
        let v594=(common.v582*self.scalar_static_f64[170]);
        let v595=(common.v260*v594);
        let v598=(self.scalar_static_f64[75]*(self.scalar_static_f64[75]*(common.v260*v595)));
        let v599=(common.v289*v598);
        let v601=((self.scalar_static_f64[169]-common.v592)).exp();
        let v644=(common.v116-300.0);
        let v646=(common.v116<525.0);
        let v647=0.00072;
        let v650=1.6e-6;
        let v651=(v644*v650);
        let v656=(!v646);
        let v659=(if v656{self.scalar_static_f64[187]}else{(if v646{(self.scalar_static_f64[4]*((common.v1+(v644*v647))-(v644*v651)))}else{common.v4})});
        let v669=(if self.scalar_static_bool[13]{(common.v1/common.v327)}else{common.v4});
        let v671=(self.scalar_static_bool[13]&&(v669>self.scalar_static_f64[15]));
        let v674=(if self.scalar_static_bool[14]{common.v4}else{(if v671{self.scalar_static_f64[15]}else{v669})});
        let v677=(if self.scalar_static_bool[15]{(common.v1/v332)}else{common.v4});
        let v679=(self.scalar_static_bool[15]&&(v677>self.scalar_static_f64[15]));
        let v682=(if self.scalar_static_bool[16]{common.v4}else{(if v679{self.scalar_static_f64[15]}else{v677})});
        let v685=(if self.scalar_static_bool[17]{(common.v1/v334)}else{common.v4});
        let v687=(self.scalar_static_bool[17]&&(v685>self.scalar_static_f64[15]));
        let v690=(if self.scalar_static_bool[18]{common.v4}else{(if v687{self.scalar_static_f64[15]}else{v685})});
        let v710=(self.scalar_static_f64[0]*(common.v708-common.v698));
        let v768=(common.v766).exp();
        let v1218=(common.v1216).exp();
        let v1225=(if common.v1220{(common.v1221*(common.v1+(common.v1216-self.scalar_static_f64[191])))}else{(if common.v1217{v1218}else{common.v4})});
        let v1226=(v1225-common.v1);
        let v1231=(common.v700<self.scalar_static_f64[217]);
        let v1232=(common.v1230).exp();
        let v1233=(common.v1+v1232);
        let v1238=(!v1231);
        let v1240=((-common.v1230)).exp();
        let v1241=(common.v1+v1240);
        let v1245=(if v1238{(self.scalar_static_f64[217]-(common.v30*(v1241).ln()))}else{(if v1231{(common.v700-(common.v30*(v1233).ln()))}else{common.v4})});
        let v1247=(v1245*self.scalar_static_f64[218]);
        let v1248=(self.scalar_static_f64[217]-v1245);
        let v1249=f64::powf(v1248,common.v31);
        let v1264=(self.scalar_static_bool[12]&&common.v1263);
        let v1265=(common.v1262).exp();
        let v1273=(if common.v1268{(common.v1269*(common.v1+(common.v1262-self.scalar_static_f64[191])))}else{(if v1264{v1265}else{common.v1216})});
        let v1279=(self.scalar_static_bool[12]&&common.v1278);
        let v1280=(common.v1276).exp();
        let v1289=(if common.v1283{(common.v1285*(common.v1+(common.v1276-common.v1277)))}else{(if v1279{v1280}else{v1225})});
        let v1290=(common.v1260-common.v1);
        let v1291=(v487*v1290);
        let v1292=(common.v31*(if self.scalar_static_bool[12]{(self.scalar_static_f64[144]*v503)}else{common.v4}));
        let v1293=(v1290*v1292);
        let v1296=((common.v1+(common.v423*v1273))).sqrt();
        let v1297=(common.v1+v1296);
        let v1298=(v1293/v1297);
        let v1299=(common.v1+common.v1156);
        let v1302=(common.v1024-common.v1);
        let v1303=(v512*v1302);
        let v1304=(v1289*v1303);
        let v1305=(common.v1+v1289);
        let v1320=(self.scalar_static_f64[219]*((common.v1024+common.v1260)-common.v31));
        let v1322=((v1290*self.scalar_static_f64[220])+(v1299*v1320));
        let v1339=(self.scalar_static_bool[12]&&common.v1338);
        let v1340=(common.v1337).exp();
        let v1349=(common.v1335-common.v1);
        let v1350=(v495*v1349);
        let v1351=(common.v31*(if self.scalar_static_bool[12]{(self.scalar_static_f64[150]*v518)}else{common.v4}));
        let v1352=(v1349*v1351);
        let v1355=((common.v1+(common.v423*(if common.v1343{(common.v1344*(common.v1+(common.v1337-self.scalar_static_f64[191])))}else{(if v1339{v1340}else{v1273})})))).sqrt();
        let v1356=(common.v1+v1355);
        let v1371=(common.v1370-common.v1);
        let v1383=(common.v1382-common.v1);
        let v1395=(common.v1394-common.v1);
        let v1396=(v473*v1395);
        let v1407=(common.v1406-common.v1);
        let v1418=(common.v1412&&common.v1417);
        let v1419=(common.v1416).exp();
        let v1427=(if common.v1422{(common.v1423*(common.v1+(common.v1416-self.scalar_static_f64[191])))}else{(if v1418{v1419}else{common.v4})});
        let v1462=(common.v1460&&common.v1461);
        let v1463=(common.v1457).exp();
        let v1472=(-common.v700);
        let v1473=(common.v1-(if common.v1466{(common.v1467*(common.v1+(common.v1457-self.scalar_static_f64[191])))}else{(if v1462{v1463}else{common.v4})}));
        let v1475=(common.v1+(v1473/common.v1457));
        let v1479=(common.v1412&&(!common.v1459));
        let v1480=(common.v411*common.v700);
        let v1481=(common.v1457*v1480);
        let v1482=0.3333333333333333;
        let v1483=(common.v1457*v1482);
        let v1484=0.25;
        let v1486=(common.v1+(common.v1457*v1484));
        let v1488=(common.v1+(v1483*v1486));
        let v1490=(if v1479{(v1481*v1488)}else{(if common.v1461{(v1472*v1475)}else{common.v4})});
        let v1491=(common.v31*(v577*v579));
        let v1492=(v1490*v1491);
        let v1493=(common.v1080*v1492);
        let v1494=(v1427*v1493);
        let v1498=(!common.v1412);
        let v1514=(common.v1504&&common.v1513);
        let v1515=(common.v1512).exp();
        let v1523=(if common.v1518{(common.v1519*(common.v1+(common.v1512-self.scalar_static_f64[191])))}else{(if v1514{v1515}else{common.v4})});
        let v1553=(common.v1551&&common.v1552);
        let v1554=(common.v1549).exp();
        let v1563=(-common.v694);
        let v1564=(common.v1-(if common.v1557{(common.v1558*(common.v1+(common.v1549-self.scalar_static_f64[191])))}else{(if v1553{v1554}else{common.v4})}));
        let v1566=(common.v1+(v1564/common.v1549));
        let v1570=(common.v1504&&(!common.v1550));
        let v1571=(common.v411*common.v694);
        let v1572=(common.v1549*v1571);
        let v1573=(v1482*common.v1549);
        let v1575=(common.v1+(v1484*common.v1549));
        let v1577=(common.v1+(v1573*v1575));
        let v1579=(if v1570{(v1572*v1577)}else{(if common.v1552{(v1563*v1566)}else{common.v4})});
        let v1580=(common.v31*(v599*v601));
        let v1581=(v1579*v1580);
        let v1582=(common.v1508*v1581);
        let v1583=(v1523*v1582);
        let v1587=(!common.v1504);
        let v1588=(if v1587{common.v4}else{(if common.v1504{(self.scalar_static_f64[50]*(common.v285*v1583))}else{common.v4})});
        let v1601=(common.v765-common.v1);
        let v1602=(common.v1600*v1601);
        let v1607=((common.v1+(common.v765*common.v1604))).sqrt();
        let v1608=(common.v1+v1607);
        let v1609=(v1602/v1608);
        let v1615=(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v1609)}else{v1609});
        let v1664=(if self.scalar_static_bool[42]{(common.v1625*common.v1662)}else{common.v4});
        let v1668=(if self.scalar_static_bool[47]{(common.v694+common.v705)}else{common.v4});
        let v1670=(-v1668);
        let v1673=(v1670<common.v4);
        let v1674=(self.scalar_static_bool[47]&&v1673);
        let v1677=((self.scalar_static_f64[234]+(if self.scalar_static_bool[47]{(v1668*v1668)}else{common.v1641}))).sqrt();
        let v1678=(v1677-v1670);
        let v1682=(self.scalar_static_bool[47]&&(!v1673));
        let v1685=(if v1682{(common.v411*(v1670+v1677))}else{(if v1674{(self.scalar_static_f64[235]/v1678)}else{common.v4})});
        let v1701=(v1685<self.scalar_static_f64[243]);
        let v1702=(self.scalar_static_bool[47]&&v1701);
        let v1703=(v1685/self.scalar_static_f64[241]);
        let v1705=(common.v1-f64::powf(v1703,self.scalar_static_f64[236]));
        let v1709=(self.scalar_static_bool[47]&&(!v1701));
        let v1715=(if self.scalar_static_bool[48]{common.v1}else{(if v1709{(self.scalar_static_f64[240]+(self.scalar_static_f64[250]*(v1685-self.scalar_static_f64[243])))}else{(if v1702{(common.v1/v1705)}else{common.v4})})});
        let v1716=(v1588*v1715);
        let v1717=(v1615*v1715);
        let v1718=(v1396*v1715);
        let v1719=(v1664*v1715);
        let v1731=(common.v1191*common.v1730);
        let v1732=(v315/v1731);
        let v1733=(v1732<self.scalar_static_f64[14]);
        let v1735=(common.v170*(if v1733{self.scalar_static_f64[14]}else{v1732}));
        let v1736=((if common.v770{(common.v771*(common.v1+(common.v766-self.scalar_static_f64[191])))}else{(if common.v767{v768}else{common.v4})})-common.v1);
        let v1738=(common.v705+(common.v862*v1736));
        let v1739=(v1738/v1735);
        let v1769=(common.v1750&&common.v1768);
        let v1770=(common.v1767).exp();
        let v1778=(if common.v1773{(common.v1774*(common.v1+(common.v1767-self.scalar_static_f64[191])))}else{(if v1769{v1770}else{common.v4})});
        let v1780=(self.scalar_static_f64[255]/common.v421);
        let v1781=(common.v1763*v1780);
        let v1789=((common.v694<common.v219)&&(self.scalar_static_bool[50]&&common.v1787));
        let v1795=(if v1789{self.scalar_static_f64[260]}else{common.v4});
        let v1796=(common.v219-common.v694);
        let v1798=(if v1789{(v1796/common.v1054)}else{common.v967});
        let v1801=(((common.v31*v1798)/v1795)).sqrt();
        let v1802=(if v1789{v1801}else{common.v4});
        let v1805=(v1789&&self.scalar_static_bool[52]);
        let v1808=(v1789&&self.scalar_static_bool[53]);
        let v1811=(if v1808{(common.v1-(common.v411*common.v1048))}else{common.v4});
        let v1812=(self.scalar_static_f64[258]*v1811);
        let v1814=(if v1808{(v1811*v1812)}else{(if v1805{self.scalar_static_f64[258]}else{common.v4})});
        let v1815=(v1802*v1814);
        let v1819=(((v1802*v1802)+(v1814*v1814))).sqrt();
        let v1821=(if v1789{(v1815/v1819)}else{common.v4});
        let v1823=(if v1789{(v1796/v1821)}else{common.v4});
        let v1824=(common.v411*v1821);
        let v1825=(v1795*v1824);
        let v1828=(if v1789{(v1823+(common.v1054*v1825))}else{common.v4});
        let v1841=(self.scalar_static_f64[194]*(if v1808{(common.v1+(self.scalar_static_f64[263]*(common.v1+(common.v31*common.v1048))))}else{common.v4}));
        let v1843=((if v1808{self.scalar_static_f64[266]}else{common.v4})-(common.v1198/v1841));
        let v1846=(if v1808{(v1823-(v1825*v1843))}else{common.v4});
        let v1847=(v1846-v1828);
        let v1849=(common.v46*v1823);
        let v1850=(v1823*v1849);
        let v1856=((if v1808{((v1847*v1847)+((common.v1051*v1850)/self.scalar_static_f64[194]))}else{v1798})).sqrt();
        let v1859=(if v1808{(common.v411*((v1828+v1846)+v1856))}else{(if v1805{v1828}else{common.v4})});
        let v1860=(v1859-v1823);
        let v1862=(if v1789{(v1860/v1859)}else{common.v4});
        let v1865=((v1862).abs()>1e-7);
        let v1866=(v1789&&v1865);
        let v1868=(if v1866{(v1824/v1862)}else{common.v4});
        let v1869=(self.scalar_static_f64[3]/v659);
        let v1870=(v1859*v1869);
        let v1871=(v1868*v1870);
        let v1872=(-v659);
        let v1873=(v1872/v1859);
        let v1874=(v1873).exp();
        let v1876=(common.v1+(v1814/v1868));
        let v1878=((v1873*v1876)).exp();
        let v1879=(v1874-v1878);
        let v1883=(v1789&&(!v1865));
        let v1884=(self.scalar_static_f64[3]*v1814);
        let v1935=(common.v1891&&common.v1934);
        let v1936=(common.v1933).exp();
        let v1944=(if common.v1939{(common.v1940*(common.v1+(common.v1933-self.scalar_static_f64[191])))}else{(if v1935{v1936}else{v1778})});
        let v1945=(common.v1761*v1780);
        let v1947=(if common.v1891{(v1944*v1945)}else{(if v1883{(v1874*v1884)}else{(if v1866{(v1871*v1879)}else{(if common.v1750{(v1778*v1781)}else{common.v4})})})});
        let v1951=(common.v1740&&(v1947>common.v4));
        let v1952=(self.scalar_static_bool[56]&&v1951);
        let v1953=(v322+v1735);
        let v1954=(common.v1198*v1953);
        let v1956=(common.v1192/common.v436);
        let v1961=(if v1952{(((common.v119/v1954)+(v487*v1956))+(v308/v1953))}else{common.v4});
        let v1962=(self.scalar_static_bool[54]&&v1952);
        let v1965=(if v1962{((v1947-v1961)/common.v408)}else{common.v1911});
        let v1966=(v1947<v1961);
        let v1967=(v1962&&v1966);
        let v1968=(v1965).exp();
        let v1969=(common.v1+v1968);
        let v1975=(v1962&&(!v1966));
        let v1977=((-v1965)).exp();
        let v1978=(common.v1+v1977);
        let v1982=(if v1975{(v1961-(common.v408*(v1978).ln()))}else{(if v1967{(v1947-(common.v408*(v1969).ln()))}else{v1947})});
        let v1983=(common.v1198*v1982);
        let v1986=(v1952&&self.scalar_static_bool[57]);
        let v1987=(v1961*v1983);
        let v1988=(v1961+v1982);
        let v1992=(v1951&&self.scalar_static_bool[58]);
        let v1993=(if v1992{v1983}else{(if v1986{(v1987/v1988)}else{(if v1962{v1983}else{common.v4})})});
        let v1994=(common.v1024>common.v4);
        let v1998=(!v1994);
        let v1999=(if v1998{common.v697}else{(if v1994{(common.v119*common.v1995)}else{common.v4})});
        let v2001=(if self.scalar_static_bool[30]{common.v697}else{(if self.scalar_static_bool[12]{common.v694}else{common.v4})});
        let v2002=(common.v700-v1999);
        let v2004=(v1999-common.v694);
        let v2009=(v710*v710);
        let v2012=(common.v731*common.v731);
        let v2015=(common.v724*common.v724);
        let v2018=(common.v721*common.v721);
        let v2021=(common.v713*common.v713);
        let v2031=((v557*v1226)+((v1247*v1249)+((((if self.scalar_static_bool[33]{(v487*v1322)}else{(if self.scalar_static_bool[31]{v1291}else{(if self.scalar_static_bool[12]{((v1291+(v1298*v1299))+(v1304/v1305))}else{common.v4})})})+(v461*v1371))+(common.v4*common.v700))-(if v1498{common.v4}else{(if common.v1412{(self.scalar_static_f64[19]*(common.v284*v1494))}else{common.v4})}))));
        let v2037=((v550*v1407)+((if self.scalar_static_bool[30]{v1350}else{(if self.scalar_static_bool[12]{(v1350+(v1352/v1356))}else{common.v4})})+(v541*v1383)));
        let v2041=(common.v4*common.v727);
        let v2042=((v1717+v1718)+v2041);
        let v2321=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2320);
        let v2339=(common.v1+(common.v101/self.scalar_static_f64[365]));
        let v2363=(if self.scalar_static_bool[79]{common.v4}else{(if self.scalar_static_bool[78]{((v1993/common.v2356)).abs()}else{common.v4})});
        let v2399=(self.scalar_static_f64[0]*v2037);
        let v2401=(self.scalar_static_f64[0]*v2031);
        let v2405=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v1716)));
        let v2408=(self.scalar_static_f64[0]*v1739);
        let v2412=(self.scalar_static_f64[0]*v710);
        let v2415=(self.scalar_static_f64[0]*common.v713);
        let v2421=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2420);
        let v2424=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2423);
        let v2427=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2426);
        let v2430=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2429);
        let v2434=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2433);
        let v2438=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2437);
        let v2442=(self.scalar_static_f64[0]*common.v731);
        let v2446=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2445);
        let v2452=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2451);
        let v2454=(self.scalar_static_f64[0]*common.v724);
        let v2458=(self.scalar_static_f64[0]*common.v721);
        let v2463=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2462);
        let v2487=(-(((common.v128*((common.v126*common.v2471)+(common.v116*(self.scalar_static_f64[21]*common.v2471))))-(common.v127*common.v2471))/(common.v128*common.v128)));
        let v2488=(v2487/common.v46);
        let v2498=(if common.v140{(v2487+(common.v46*((common.v142*(-v2488))/common.v143)))}else{(if common.v133{(common.v46*((common.v134*v2488)/common.v135))}else{common.v4})});
        let v2508=(-(((common.v150*((common.v148*common.v2471)+(common.v116*(self.scalar_static_f64[52]*common.v2471))))-(common.v149*common.v2471))/(common.v150*common.v150)));
        let v2509=(v2508/common.v46);
        let v2519=(if common.v162{(v2508+(common.v46*((common.v164*(-v2509))/common.v165)))}else{(if common.v155{(common.v46*((common.v156*v2509)/common.v157))}else{common.v4})});
        let v2643=((-common.v2614)/common.v2642);
        let v2651=((self.scalar_static_f64[46]*v2643)*(self.scalar_static_f64[47]*f64::powf(common.v288,self.scalar_static_f64[228])));
        let v2668=(if v307{common.v4}else{(self.scalar_static_f64[92]*(v305*(self.scalar_static_f64[93]*common.v2477)))});
        let v2675=(if v321{common.v4}else{(self.scalar_static_f64[98]*(v319*(self.scalar_static_f64[99]*common.v2477)))});
        let v2680=(v331*(self.scalar_static_f64[103]*common.v2477));
        let v2725=(common.v2723/(common.v31*common.v414));
        let v2734=(if common.v418{(common.v411*(common.v2721+v2725))}else{(if common.v410{((-(common.v412*(v2725-common.v2721)))/(common.v415*common.v415))}else{common.v4})});
        let v2761=(self.scalar_static_f64[129]*common.v2476);
        let v2776=(self.scalar_static_f64[137]*common.v2477);
        let v2780=(self.scalar_static_f64[140]*common.v2476);
        let v2785=((v486*(self.scalar_static_f64[135]*(v480*(v2776/self.scalar_static_f64[138]))))+(v481*(v486*(v2780/self.scalar_static_f64[138]))));
        let v2841=-1.5;
        let v2844=((self.scalar_static_f64[43]*v2498)*(common.v559*f64::powf(common.v558,v2841)));
        let v2863=(self.scalar_static_f64[43]*(self.scalar_static_f64[43]*((common.v567*common.v2640)+(common.v284*(self.scalar_static_f64[44]*((common.v565*common.v2847)+(common.v561*((common.v564*v2844)+(common.v560*((common.v563*v2498)+(common.v147*(self.scalar_static_f64[167]*v2498))))))))))));
        let v2884=((self.scalar_static_f64[74]*v2519)*(common.v559*f64::powf(common.v581,v2841)));
        let v2903=(self.scalar_static_f64[74]*(self.scalar_static_f64[74]*((common.v589*v2643)+(common.v285*(self.scalar_static_f64[46]*((common.v587*((-v2651)/(common.v289*common.v289)))+(common.v583*((common.v586*v2884)+(common.v582*((common.v585*v2519)+(common.v169*(self.scalar_static_f64[169]*v2519))))))))))));
        let v2959=(if v656{common.v4}else{(if v646{(self.scalar_static_f64[4]*((v647*common.v2471)-((v651*common.v2471)+(v644*(v650*common.v2471)))))}else{common.v4})});
        let v2966=(if self.scalar_static_bool[14]{common.v4}else{(if v671{common.v4}else{(if self.scalar_static_bool[13]{((-common.v2678)/(common.v327*common.v327))}else{common.v4})})});
        let v2972=(if self.scalar_static_bool[16]{common.v4}else{(if v679{common.v4}else{(if self.scalar_static_bool[15]{((-(self.scalar_static_f64[102]*v2680))/(v332*v332))}else{common.v4})})});
        let v2978=(if self.scalar_static_bool[18]{common.v4}else{(if v687{common.v4}else{(if self.scalar_static_bool[17]{((-(self.scalar_static_f64[104]*v2680))/(v334*v334))}else{common.v4})})});
        let v3040=(common.v705*common.v2476);
        let v4442=(((common.v1192*(common.v4431-common.v4425))-(common.v1197*common.v4409))/common.v4441);
        let v4446=((common.v4443-(common.v1197*common.v4412))/common.v4441);
        let v4450=(((common.v1192*(common.v4433-common.v4426))-(common.v1197*common.v4415))/common.v4441);
        let v4454=(((common.v1192*(-common.v4427))-(common.v1197*common.v4418))/common.v4441);
        let v4458=(((common.v1192*(-common.v4428))-(common.v1197*common.v4421))/common.v4441);
        let v4481=(common.v4479/self.scalar_static_f64[216]);
        let v4482=(common.v4480/self.scalar_static_f64[216]);
        let v4489=(if common.v1220{(common.v1221*v4481)}else{(if common.v1217{(v1218*v4481)}else{common.v4})});
        let v4490=(if common.v1220{(common.v1221*v4482)}else{(if common.v1217{(v1218*v4482)}else{common.v4})});
        let v4516=(if v1238{(-(common.v30*((v1240*self.scalar_static_f64[315])/v1241)))}else{(if v1231{(self.scalar_static_f64[299]-(common.v30*((v1232*self.scalar_static_f64[313])/v1233)))}else{common.v4})});
        let v4517=(if v1238{(-(common.v30*((v1240*self.scalar_static_f64[316])/v1241)))}else{(if v1231{(self.scalar_static_f64[0]-(common.v30*((v1232*self.scalar_static_f64[314])/v1233)))}else{common.v4})});
        let v4523=(common.v31*f64::powf(v1248,common.v1));
        let v4549=(common.v121*(-(if common.v276{((common.v280*common.v2473)+(common.v119*((common.v278*(-common.v2623))/common.v279)))}else{(if common.v269{(common.v2618+((common.v272*common.v2473)+(common.v119*((common.v270*common.v2623)/common.v271))))}else{common.v4})})));
        let v4550=((common.v1261*common.v2476)+v4549);
        let v4560=(if common.v1268{(common.v1269*v4550)}else{(if v1264{(v1265*v4550)}else{common.v4})});
        let v4561=(if common.v1268{(common.v1269*common.v2985)}else{(if v1264{(v1265*common.v2985)}else{v4481})});
        let v4562=(if common.v1268{(common.v1269*common.v2984)}else{(if v1264{(v1265*common.v2984)}else{v4482})});
        let v4566=(common.v436*common.v436);
        let v4567=(((common.v436*v4442)-(common.v1198*common.v2751))/v4566);
        let v4568=(v4446/common.v436);
        let v4569=(v4450/common.v436);
        let v4570=(v4454/common.v436);
        let v4571=(v4458/common.v436);
        let v4587=(if common.v1283{(common.v1285*v4567)}else{(if v1279{(v1280*v4567)}else{common.v4})});
        let v4588=(if common.v1283{(common.v1285*v4568)}else{(if v1279{(v1280*v4568)}else{v4489})});
        let v4589=(if common.v1283{(common.v1285*v4569)}else{(if v1279{(v1280*v4569)}else{v4490})});
        let v4590=(if common.v1283{(common.v1285*v4570)}else{(if v1279{(v1280*v4570)}else{common.v4})});
        let v4591=(if common.v1283{(common.v1285*v4571)}else{(if v1279{(v1280*v4571)}else{common.v4})});
        let v4594=((v1290*v2785)+(v487*common.v4544));
        let v4595=(v487*common.v4545);
        let v4596=(v487*common.v4546);
        let v4606=(common.v31*v1296);
        let v4613=(v1297*v1297);
        let v4657=(v1305*v1305);
        let v4726=(if self.scalar_static_bool[33]{(v487*((v1320*common.v4265)+(v1299*(self.scalar_static_f64[219]*common.v3832))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1298*common.v4265)+(((v1305*((v1303*v4590)+(v1289*(v512*common.v3832))))-(v1304*v4590))/v4657))}else{common.v4})})});
        let v4727=(if self.scalar_static_bool[33]{(v487*((v1320*common.v4266)+(v1299*(self.scalar_static_f64[219]*common.v3833))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1298*common.v4266)+(((v1305*((v1303*v4591)+(v1289*(v512*common.v3833))))-(v1304*v4591))/v4657))}else{common.v4})})});
        let v4747=(v4549+(common.v1336*common.v2476));
        let v4764=((v1349*((v494*(self.scalar_static_f64[141]*(v491*(v2776/self.scalar_static_f64[142]))))+(v492*(v494*(v2780/self.scalar_static_f64[142])))))+(v495*common.v4742));
        let v4765=(v495*common.v4743);
        let v4766=(v495*common.v4744);
        let v4767=(v495*common.v4745);
        let v4779=(common.v31*v1355);
        let v4787=(v1356*v1356);
        let v4834=(v461*common.v4828);
        let v4921=(v550*common.v4913);
        let v4922=(v550*common.v4914);
        let v4928=(common.v1413*common.v1413);
        let v4941=((common.v1415*v2863)+(common.v570*(-((-(self.scalar_static_f64[18]*(common.v31*common.v3972)))/v4928))));
        let v4942=(common.v570*(-((-(self.scalar_static_f64[18]*(common.v31*common.v3973)))/v4928)));
        let v4943=(common.v570*(-((-(self.scalar_static_f64[18]*(common.v31*common.v3974)))/v4928)));
        let v4959=(if common.v1412{(common.v700*common.v2640)}else{common.v2922});
        let v4960=(if common.v1412{(common.v284*self.scalar_static_f64[299])}else{common.v4});
        let v4961=(if common.v1412{(self.scalar_static_f64[0]*common.v284)}else{common.v4});
        let v4962=(common.v1429*v4959);
        let v4964=(common.v1429*v4960);
        let v4966=(common.v1429*v4961);
        let v4968=(common.v31*common.v1433);
        let v4974=(self.scalar_static_f64[221]*f64::powf(common.v1433,self.scalar_static_f64[317]));
        let v5042=(common.v1455*common.v1455);
        let v5052=(if common.v1412{(((common.v1455*(common.v1453*v2863))-(common.v1454*((common.v1452*v2498)+(common.v147*(if common.v1412{(common.v1450*((common.v1448*(((v4962+v4962)/v4968)*v4974))+(common.v1436*((self.scalar_static_f64[16]*(-(self.scalar_static_f64[224]*(common.v170*v4959))))-((common.v1446*((common.v1444*v4959)+(common.v1429*(common.v449*v4959))))+(common.v1445*v4959))))))}else{common.v4})))))/v5042)}else{v4959});
        let v5053=(if common.v1412{(((common.v1455*(common.v570*self.scalar_static_f64[318]))-(common.v1454*(common.v147*(if common.v1412{(common.v1450*((common.v1448*(((v4964+v4964)/v4968)*v4974))+(common.v1436*((self.scalar_static_f64[16]*(-(self.scalar_static_f64[224]*(common.v170*v4960))))-((common.v1446*((common.v1444*v4960)+(common.v1429*(common.v449*v4960))))+(common.v1445*v4960))))))}else{common.v4}))))/v5042)}else{v4960});
        let v5054=(if common.v1412{(((common.v1455*(common.v570*self.scalar_static_f64[319]))-(common.v1454*(common.v147*(if common.v1412{(common.v1450*((common.v1448*(((v4966+v4966)/v4968)*v4974))+(common.v1436*((self.scalar_static_f64[16]*(-(self.scalar_static_f64[224]*(common.v170*v4961))))-((common.v1446*((common.v1444*v4961)+(common.v1429*(common.v449*v4961))))+(common.v1445*v4961))))))}else{common.v4}))))/v5042)}else{v4961});
        let v5073=(common.v1457*common.v1457);
        let v5167=(common.v694*v2643);
        let v5168=(self.scalar_static_f64[0]*common.v285);
        let v5169=(common.v285*self.scalar_static_f64[299]);
        let v5174=(self.scalar_static_f64[214]*f64::powf(common.v1506,self.scalar_static_f64[308]));
        let v5178=(if common.v1504{((-v5167)*v5174)}else{common.v4});
        let v5179=(if common.v1504{((-v5168)*v5174)}else{common.v4});
        let v5180=(if common.v1504{((-v5169)*v5174)}else{common.v4});
        let v5186=(common.v1509*common.v1509);
        let v5199=((common.v1511*v2903)+(common.v592*(-((-(self.scalar_static_f64[49]*(common.v31*v5178)))/v5186))));
        let v5200=(common.v592*(-((-(self.scalar_static_f64[49]*(common.v31*v5179)))/v5186)));
        let v5201=(common.v592*(-((-(self.scalar_static_f64[49]*(common.v31*v5180)))/v5186)));
        let v5214=(if common.v1504{v5167}else{v2884});
        let v5215=(if common.v1504{v5168}else{common.v4});
        let v5216=(if common.v1504{v5169}else{common.v4});
        let v5217=(common.v1524*v5214);
        let v5219=(common.v1524*v5215);
        let v5221=(common.v1524*v5216);
        let v5223=(common.v31*common.v1527);
        let v5229=(self.scalar_static_f64[225]*f64::powf(common.v1527,self.scalar_static_f64[322]));
        let v5297=(common.v1547*common.v1547);
        let v5307=(if common.v1504{(((common.v1547*(common.v1545*v2903))-(common.v1546*((common.v1544*v2519)+(common.v169*(if common.v1504{(common.v1450*((common.v1541*(((v5217+v5217)/v5223)*v5229))+(common.v1529*((self.scalar_static_f64[47]*(-(self.scalar_static_f64[228]*(common.v170*v5214))))-((common.v1539*((common.v1537*v5214)+(common.v1524*(common.v449*v5214))))+(common.v1538*v5214))))))}else{common.v4})))))/v5297)}else{v5214});
        let v5308=(if common.v1504{(((common.v1547*(common.v592*self.scalar_static_f64[323]))-(common.v1546*(common.v169*(if common.v1504{(common.v1450*((common.v1541*(((v5219+v5219)/v5223)*v5229))+(common.v1529*((self.scalar_static_f64[47]*(-(self.scalar_static_f64[228]*(common.v170*v5215))))-((common.v1539*((common.v1537*v5215)+(common.v1524*(common.v449*v5215))))+(common.v1538*v5215))))))}else{common.v4}))))/v5297)}else{v5215});
        let v5309=(if common.v1504{(((common.v1547*(common.v592*self.scalar_static_f64[324]))-(common.v1546*(common.v169*(if common.v1504{(common.v1450*((common.v1541*(((v5221+v5221)/v5223)*v5229))+(common.v1529*((self.scalar_static_f64[47]*(-(self.scalar_static_f64[228]*(common.v170*v5216))))-((common.v1539*((common.v1537*v5216)+(common.v1524*(common.v449*v5216))))+(common.v1538*v5216))))))}else{common.v4}))))/v5297)}else{v5216});
        let v5328=(common.v1549*common.v1549);
        let v5508=(common.v31*v1607);
        let v5517=(v1608*v1608);
        let v5518=(((v1608*((v1601*common.v5487)+(common.v1600*common.v3035)))-(v1602*(((common.v1604*common.v3035)+(common.v765*common.v5500))/v5508)))/v5517);
        let v5522=(((v1608*(common.v1600*common.v3036))-(v1602*((common.v1604*common.v3036)/v5508)))/v5517);
        let v5526=(((v1608*(common.v1600*common.v3037))-(v1602*((common.v1604*common.v3037)/v5508)))/v5517);
        let v5530=(((v1608*(common.v1600*common.v3038))-(v1602*((common.v1604*common.v3038)/v5508)))/v5517);
        let v5534=(((v1608*(common.v1600*common.v3039))-(v1602*((common.v1604*common.v3039)/v5508)))/v5517);
        let v5783=(common.v1662*common.v5587);
        let v5795=(common.v1662*common.v5590);
        let v5820=(v1668*self.scalar_static_f64[329]);
        let v5822=(v1668*self.scalar_static_f64[330]);
        let v5824=(v1668*self.scalar_static_f64[331]);
        let v5836=(common.v31*v1677);
        let v5837=((if self.scalar_static_bool[47]{common.v4}else{common.v5622})/v5836);
        let v5838=((if self.scalar_static_bool[47]{common.v4}else{common.v5623})/v5836);
        let v5839=((if self.scalar_static_bool[47]{common.v4}else{common.v5624})/v5836);
        let v5840=((if self.scalar_static_bool[47]{common.v4}else{common.v5625})/v5836);
        let v5841=((if self.scalar_static_bool[47]{(v5820+v5820)}else{common.v5622})/v5836);
        let v5842=((if self.scalar_static_bool[47]{(v5822+v5822)}else{common.v5626})/v5836);
        let v5843=((if self.scalar_static_bool[47]{(v5824+v5824)}else{common.v5627})/v5836);
        let v5844=((if self.scalar_static_bool[47]{common.v4}else{common.v5628})/v5836);
        let v5845=((if self.scalar_static_bool[47]{common.v4}else{common.v5629})/v5836);
        let v5846=((if self.scalar_static_bool[47]{common.v4}else{common.v5630})/v5836);
        let v5852=(v1678*v1678);
        let v5904=(if v1682{(common.v411*v5837)}else{(if v1674{((-(self.scalar_static_f64[235]*v5837))/v5852)}else{common.v4})});
        let v5905=(if v1682{(common.v411*v5838)}else{(if v1674{((-(self.scalar_static_f64[235]*v5838))/v5852)}else{common.v4})});
        let v5906=(if v1682{(common.v411*v5839)}else{(if v1674{((-(self.scalar_static_f64[235]*v5839))/v5852)}else{common.v4})});
        let v5907=(if v1682{(common.v411*v5840)}else{(if v1674{((-(self.scalar_static_f64[235]*v5840))/v5852)}else{common.v4})});
        let v5908=(if v1682{(common.v411*(self.scalar_static_f64[332]+v5841))}else{(if v1674{((-(self.scalar_static_f64[235]*(v5841-self.scalar_static_f64[332])))/v5852)}else{common.v4})});
        let v5909=(if v1682{(common.v411*(self.scalar_static_f64[333]+v5842))}else{(if v1674{((-(self.scalar_static_f64[235]*(v5842-self.scalar_static_f64[333])))/v5852)}else{common.v4})});
        let v5910=(if v1682{(common.v411*(self.scalar_static_f64[334]+v5843))}else{(if v1674{((-(self.scalar_static_f64[235]*(v5843-self.scalar_static_f64[334])))/v5852)}else{common.v4})});
        let v5911=(if v1682{(common.v411*v5844)}else{(if v1674{((-(self.scalar_static_f64[235]*v5844))/v5852)}else{common.v4})});
        let v5912=(if v1682{(common.v411*v5845)}else{(if v1674{((-(self.scalar_static_f64[235]*v5845))/v5852)}else{common.v4})});
        let v5913=(if v1682{(common.v411*v5846)}else{(if v1674{((-(self.scalar_static_f64[235]*v5846))/v5852)}else{common.v4})});
        let v5925=(self.scalar_static_f64[236]*f64::powf(v1703,self.scalar_static_f64[245]));
        let v5936=(v1705*v1705);
        let v5977=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5904)}else{(if v1702{(((v5904/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5978=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5905)}else{(if v1702{(((v5905/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5979=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5906)}else{(if v1702{(((v5906/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5980=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5907)}else{(if v1702{(((v5907/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5981=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5908)}else{(if v1702{(((v5908/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5982=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5909)}else{(if v1702{(((v5909/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5983=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5910)}else{(if v1702{(((v5910/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5984=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5911)}else{(if v1702{(((v5911/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5985=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5912)}else{(if v1702{(((v5912/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5986=(if self.scalar_static_bool[48]{common.v4}else{(if v1709{(self.scalar_static_f64[250]*v5913)}else{(if v1702{(((v5913/self.scalar_static_f64[241])*v5925)/v5936)}else{common.v4})})});
        let v5987=(v1588*v5977);
        let v5988=(v1588*v5978);
        let v5991=((v1715*(if v1587{common.v4}else{(if common.v1504{(self.scalar_static_f64[50]*((v1583*v2643)+(common.v285*((v1582*(if common.v1518{(common.v1519*v5199)}else{(if v1514{(v1515*v5199)}else{common.v4})}))+(v1523*((v1581*v5178)+(common.v1508*((v1580*(if v1570{((v1577*(v1571*v5307))+(v1572*((v1575*(v1482*v5307))+(v1573*(v1484*v5307)))))}else{(if common.v1552{(v1563*(((common.v1549*(-(if common.v1557{(common.v1558*v5307)}else{(if v1553{(v1554*v5307)}else{common.v4})})))-(v1564*v5307))/v5328))}else{common.v4})}))+(v1579*(common.v31*((v601*((v598*v2651)+(common.v289*(self.scalar_static_f64[75]*(self.scalar_static_f64[75]*((v595*common.v2614)+(common.v260*((v594*common.v2614)+(common.v260*(self.scalar_static_f64[170]*v2884))))))))))+(v599*(v601*(-v2903))))))))))))))}else{common.v4})}))+(v1588*v5979));
        let v5992=(v1588*v5980);
        let v5993=(v1588*v5981);
        let v5996=((v1715*(if v1587{common.v4}else{(if common.v1504{(self.scalar_static_f64[50]*(common.v285*((v1582*(if common.v1518{(common.v1519*v5200)}else{(if v1514{(v1515*v5200)}else{common.v4})}))+(v1523*((v1581*v5179)+(common.v1508*(v1580*(if v1570{((v1577*((v1571*v5308)+(common.v1549*self.scalar_static_f64[321])))+(v1572*((v1575*(v1482*v5308))+(v1573*(v1484*v5308)))))}else{(if common.v1552{((v1566*self.scalar_static_f64[299])+(v1563*(((common.v1549*(-(if common.v1557{(common.v1558*v5308)}else{(if v1553{(v1554*v5308)}else{common.v4})})))-(v1564*v5308))/v5328)))}else{common.v4})}))))))))}else{common.v4})}))+(v1588*v5982));
        let v5999=((v1715*(if v1587{common.v4}else{(if common.v1504{(self.scalar_static_f64[50]*(common.v285*((v1582*(if common.v1518{(common.v1519*v5201)}else{(if v1514{(v1515*v5201)}else{common.v4})}))+(v1523*((v1581*v5180)+(common.v1508*(v1580*(if v1570{((v1577*((v1571*v5309)+(common.v1549*self.scalar_static_f64[320])))+(v1572*((v1575*(v1482*v5309))+(v1573*(v1484*v5309)))))}else{(if common.v1552{((self.scalar_static_f64[0]*v1566)+(v1563*(((common.v1549*(-(if common.v1557{(common.v1558*v5309)}else{(if v1553{(v1554*v5309)}else{common.v4})})))-(v1564*v5309))/v5328)))}else{common.v4})}))))))))}else{common.v4})}))+(v1588*v5983));
        let v6000=(v1588*v5984);
        let v6001=(v1588*v5985);
        let v6002=(v1588*v5986);
        let v6011=((v1715*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v5522)}else{v5522}))+(v1615*v5981));
        let v6014=((v1715*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v5526)}else{v5526}))+(v1615*v5982));
        let v6015=(v1715*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v5530)}else{v5530}));
        let v6017=(v6015+(v1615*v5983));
        let v6019=(v6015+(v1615*v5984));
        let v6023=((v1715*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v5534)}else{v5534}))+(v1615*v5986));
        let v6034=((v1715*(v473*common.v4882))+(v1396*v5981));
        let v6037=((v1715*(v473*common.v4883))+(v1396*v5982));
        let v6038=(v1715*(v473*common.v4884));
        let v6040=(v6038+(v1396*v5983));
        let v6042=(v6038+(v1396*v5984));
        let v6046=((v1715*(v473*common.v4885))+(v1396*v5986));
        let v6047=(v1715*(if self.scalar_static_bool[42]{(v5783+(common.v1625*common.v5774))}else{common.v4}));
        let v6049=(v6047+(v1664*v5977));
        let v6052=((v1715*(if self.scalar_static_bool[42]{((common.v1662*common.v5588)+(common.v1625*common.v5775))}else{common.v4}))+(v1664*v5978));
        let v6055=((v1715*(if self.scalar_static_bool[42]{((common.v1662*common.v5589)+(common.v1625*common.v5776))}else{common.v4}))+(v1664*v5979));
        let v6058=((v1715*(if self.scalar_static_bool[42]{(common.v1625*common.v5777)}else{common.v4}))+(v1664*v5980));
        let v6060=(v6047+(v1664*v5981));
        let v6063=((v1715*(if self.scalar_static_bool[42]{(v5783+(common.v1625*common.v5778))}else{common.v4}))+(v1664*v5982));
        let v6066=((v1715*(if self.scalar_static_bool[42]{(v5795+(common.v1625*common.v5779))}else{common.v4}))+(v1664*v5983));
        let v6069=((v1715*(if self.scalar_static_bool[42]{(v5795+(common.v1625*common.v5780))}else{common.v4}))+(v1664*v5984));
        let v6072=((v1715*(if self.scalar_static_bool[42]{((common.v1662*common.v5591)+(common.v1625*common.v5781))}else{common.v4}))+(v1664*v5985));
        let v6075=((v1715*(if self.scalar_static_bool[42]{(v5795+(common.v1625*common.v5782))}else{common.v4}))+(v1664*v5986));
        let v6151=(v1731*v1731);
        let v6170=(common.v170*(if v1733{common.v4}else{(((v1731*(self.scalar_static_f64[94]*(v314*(self.scalar_static_f64[97]*common.v2477))))-(v315*((common.v1730*common.v4402)+(common.v1191*common.v6128))))/v6151)}));
        let v6171=(common.v170*(if v1733{common.v4}else{((-(v315*((common.v1730*common.v4403)+(common.v1191*common.v6129))))/v6151)}));
        let v6172=(common.v170*(if v1733{common.v4}else{((-(v315*((common.v1730*common.v4404)+(common.v1191*common.v6130))))/v6151)}));
        let v6173=(common.v170*(if v1733{common.v4}else{((-(v315*((common.v1730*common.v4405)+(common.v1191*common.v6131))))/v6151)}));
        let v6174=(common.v170*(if v1733{common.v4}else{((-(v315*((common.v1730*common.v4406)+(common.v1191*common.v6132))))/v6151)}));
        let v6185=(v1735*v1735);
        let v6186=(((v1735*((v1736*common.v3227)+(common.v862*(if common.v770{(common.v771*v3040)}else{(if common.v767{(v768*v3040)}else{common.v4})}))))-(v1738*v6170))/v6185);
        let v6189=((-(v1738*v6171))/v6185);
        let v6190=((self.scalar_static_f64[0]+(common.v862*(if common.v770{(common.v771*common.v2984)}else{(if common.v767{(v768*common.v2984)}else{common.v4})})))/v1735);
        let v6194=(((v1735*(self.scalar_static_f64[299]+(common.v862*(if common.v770{(common.v771*common.v2985)}else{(if common.v767{(v768*common.v2985)}else{common.v4})}))))-(v1738*v6172))/v6185);
        let v6197=((-(v1738*v6173))/v6185);
        let v6200=((-(v1738*v6174))/v6185);
        let v6206=((-v4442)/self.scalar_static_f64[253]);
        let v6207=((-v4446)/self.scalar_static_f64[253]);
        let v6208=((-v4450)/self.scalar_static_f64[253]);
        let v6209=((-v4454)/self.scalar_static_f64[253]);
        let v6210=((-v4458)/self.scalar_static_f64[253]);
        let v6240=(if common.v1750{(common.v1761*(if common.v1755{(common.v1756*v6206)}else{(if common.v1751{(common.v1752*v6206)}else{common.v4})}))}else{common.v4});
        let v6241=(if common.v1750{(common.v1761*(if common.v1755{(common.v1756*v6207)}else{(if common.v1751{(common.v1752*v6207)}else{common.v4})}))}else{common.v4});
        let v6242=(if common.v1750{((common.v1761*(if common.v1755{(common.v1756*v6208)}else{(if common.v1751{(common.v1752*v6208)}else{common.v4})}))+(common.v1760*self.scalar_static_f64[299]))}else{common.v4});
        let v6243=(if common.v1750{((common.v1761*(if common.v1755{(common.v1756*v6209)}else{(if common.v1751{(common.v1752*v6209)}else{common.v4})}))+(self.scalar_static_f64[0]*common.v1760))}else{common.v4});
        let v6244=(if common.v1750{(common.v1761*(if common.v1755{(common.v1756*v6210)}else{(if common.v1751{(common.v1752*v6210)}else{common.v4})}))}else{common.v4});
        let v6245=(-v2734);
        let v6248=(self.scalar_static_f64[254]*f64::powf(common.v1763,self.scalar_static_f64[335]));
        let v6256=((common.v1766*v6245)+(common.v1764*(v6240*v6248)));
        let v6257=(common.v1764*(v6241*v6248));
        let v6258=(common.v1764*(v6242*v6248));
        let v6259=(common.v1764*(v6243*v6248));
        let v6260=(common.v1764*(v6244*v6248));
        let v6276=(if common.v1773{(common.v1774*v6256)}else{(if v1769{(v1770*v6256)}else{common.v4})});
        let v6277=(if common.v1773{(common.v1774*v6257)}else{(if v1769{(v1770*v6257)}else{common.v4})});
        let v6278=(if common.v1773{(common.v1774*v6258)}else{(if v1769{(v1770*v6258)}else{common.v4})});
        let v6279=(if common.v1773{(common.v1774*v6259)}else{(if v1769{(v1770*v6259)}else{common.v4})});
        let v6280=(if common.v1773{(common.v1774*v6260)}else{(if v1769{(v1770*v6260)}else{common.v4})});
        let v6284=((-(self.scalar_static_f64[255]*v2734))/(common.v421*common.v421));
        let v6315=(common.v1054*common.v1054);
        let v6328=(if v1789{(((common.v1054*common.v2570)-(v1796*common.v3910))/v6315)}else{common.v3619});
        let v6329=(if v1789{(((common.v1054*self.scalar_static_f64[299])-(v1796*common.v3911))/v6315)}else{common.v3620});
        let v6330=(if v1789{(((self.scalar_static_f64[0]*common.v1054)-(v1796*common.v3912))/v6315)}else{common.v3621});
        let v6331=(if v1789{((-(v1796*common.v3913))/v6315)}else{common.v3622});
        let v6340=(common.v31*v1801);
        let v6345=(if v1789{(((common.v31*v6328)/v1795)/v6340)}else{common.v4});
        let v6346=(if v1789{(((common.v31*v6329)/v1795)/v6340)}else{common.v4});
        let v6347=(if v1789{(((common.v31*v6330)/v1795)/v6340)}else{common.v4});
        let v6348=(if v1789{(((common.v31*v6331)/v1795)/v6340)}else{common.v4});
        let v6357=(if v1808{(-(common.v411*common.v3886))}else{common.v4});
        let v6358=(if v1808{(-(common.v411*common.v3887))}else{common.v4});
        let v6359=(if v1808{(-(common.v411*common.v3888))}else{common.v4});
        let v6360=(if v1808{(-(common.v411*common.v3889))}else{common.v4});
        let v6377=(if v1808{((v1812*v6357)+(v1811*(self.scalar_static_f64[258]*v6357)))}else{common.v4});
        let v6378=(if v1808{((v1812*v6358)+(v1811*(self.scalar_static_f64[258]*v6358)))}else{common.v4});
        let v6379=(if v1808{((v1812*v6359)+(v1811*(self.scalar_static_f64[258]*v6359)))}else{common.v4});
        let v6380=(if v1808{((v1812*v6360)+(v1811*(self.scalar_static_f64[258]*v6360)))}else{common.v4});
        let v6393=(v1802*v6345);
        let v6395=(v1802*v6346);
        let v6397=(v1802*v6347);
        let v6399=(v1802*v6348);
        let v6401=(v1814*v6377);
        let v6403=(v1814*v6378);
        let v6405=(v1814*v6379);
        let v6407=(v1814*v6380);
        let v6413=(common.v31*v1819);
        let v6421=(v1819*v1819);
        let v6435=(if v1789{(((v1819*((v1814*v6345)+(v1802*v6377)))-(v1815*(((v6393+v6393)+(v6401+v6401))/v6413)))/v6421)}else{common.v4});
        let v6436=(if v1789{(((v1819*((v1814*v6346)+(v1802*v6378)))-(v1815*(((v6395+v6395)+(v6403+v6403))/v6413)))/v6421)}else{common.v4});
        let v6437=(if v1789{(((v1819*((v1814*v6347)+(v1802*v6379)))-(v1815*(((v6397+v6397)+(v6405+v6405))/v6413)))/v6421)}else{common.v4});
        let v6438=(if v1789{(((v1819*((v1814*v6348)+(v1802*v6380)))-(v1815*(((v6399+v6399)+(v6407+v6407))/v6413)))/v6421)}else{common.v4});
        let v6442=(v1821*v1821);
        let v6455=(if v1789{(((v1821*common.v2570)-(v1796*v6435))/v6442)}else{common.v4});
        let v6456=(if v1789{(((v1821*self.scalar_static_f64[299])-(v1796*v6436))/v6442)}else{common.v4});
        let v6457=(if v1789{(((self.scalar_static_f64[0]*v1821)-(v1796*v6437))/v6442)}else{common.v4});
        let v6458=(if v1789{((-(v1796*v6438))/v6442)}else{common.v4});
        let v6459=(common.v411*v6435);
        let v6460=(common.v411*v6436);
        let v6461=(common.v411*v6437);
        let v6462=(common.v411*v6438);
        let v6463=(v1795*v6459);
        let v6464=(v1795*v6460);
        let v6465=(v1795*v6461);
        let v6466=(v1795*v6462);
        let v6483=(if v1789{(v6455+((v1825*common.v3910)+(common.v1054*v6463)))}else{common.v4});
        let v6484=(if v1789{(v6456+((v1825*common.v3911)+(common.v1054*v6464)))}else{common.v4});
        let v6485=(if v1789{(v6457+((v1825*common.v3912)+(common.v1054*v6465)))}else{common.v4});
        let v6486=(if v1789{(v6458+((v1825*common.v3913)+(common.v1054*v6466)))}else{common.v4});
        let v6510=(v1841*v1841);
        let v6548=(if v1808{(v6455-((v1843*v6463)+(v1825*(-(((v1841*v4442)-(common.v1198*(self.scalar_static_f64[194]*(if v1808{(self.scalar_static_f64[263]*(common.v31*common.v3886))}else{common.v4}))))/v6510)))))}else{common.v4});
        let v6549=(if v1808{(-(v1825*(-(v4446/v1841))))}else{common.v4});
        let v6550=(if v1808{(v6456-((v1843*v6464)+(v1825*(-(((v1841*v4450)-(common.v1198*(self.scalar_static_f64[194]*(if v1808{(self.scalar_static_f64[263]*(common.v31*common.v3887))}else{common.v4}))))/v6510)))))}else{common.v4});
        let v6551=(if v1808{(v6457-((v1843*v6465)+(v1825*(-(((v1841*v4454)-(common.v1198*(self.scalar_static_f64[194]*(if v1808{(self.scalar_static_f64[263]*(common.v31*common.v3888))}else{common.v4}))))/v6510)))))}else{common.v4});
        let v6552=(if v1808{(v6458-((v1843*v6466)+(v1825*(-(((v1841*v4458)-(common.v1198*(self.scalar_static_f64[194]*(if v1808{(self.scalar_static_f64[263]*(common.v31*common.v3889))}else{common.v4}))))/v6510)))))}else{common.v4});
        let v6557=(v1847*(v6548-v6483));
        let v6559=(v1847*v6549);
        let v6561=(v1847*(v6550-v6484));
        let v6563=(v1847*(v6551-v6485));
        let v6565=(v1847*(v6552-v6486));
        let v6612=(common.v31*v1856);
        let v6628=(if v1808{(common.v411*((v6483+v6548)+((if v1808{((v6557+v6557)+(((v1850*common.v3898)+(common.v1051*((v1849*v6455)+(v1823*(common.v46*v6455)))))/self.scalar_static_f64[194]))}else{v6328})/v6612)))}else{(if v1805{v6483}else{common.v4})});
        let v6629=(if v1808{(common.v411*(v6549+((if v1808{(v6559+v6559)}else{common.v4})/v6612)))}else{common.v4});
        let v6630=(if v1808{(common.v411*((v6484+v6550)+((if v1808{((v6561+v6561)+(((v1850*common.v3899)+(common.v1051*((v1849*v6456)+(v1823*(common.v46*v6456)))))/self.scalar_static_f64[194]))}else{v6329})/v6612)))}else{(if v1805{v6484}else{common.v4})});
        let v6631=(if v1808{(common.v411*((v6485+v6551)+((if v1808{((v6563+v6563)+(((v1850*common.v3900)+(common.v1051*((v1849*v6457)+(v1823*(common.v46*v6457)))))/self.scalar_static_f64[194]))}else{v6330})/v6612)))}else{(if v1805{v6485}else{common.v4})});
        let v6632=(if v1808{(common.v411*((v6486+v6552)+((if v1808{((v6565+v6565)+(((v1850*common.v3901)+(common.v1051*((v1849*v6458)+(v1823*(common.v46*v6458)))))/self.scalar_static_f64[194]))}else{v6331})/v6612)))}else{(if v1805{v6486}else{common.v4})});
        let v6640=(v1859*v1859);
        let v6666=(v1862*v1862);
        let v6683=(if v1866{(((v1862*v6459)-(v1824*(if v1789{(((v1859*(v6628-v6455))-(v1860*v6628))/v6640)}else{common.v4})))/v6666)}else{common.v4});
        let v6684=(if v1866{((-(v1824*(if v1789{(((v1859*v6629)-(v1860*v6629))/v6640)}else{common.v4})))/v6666)}else{common.v4});
        let v6685=(if v1866{(((v1862*v6460)-(v1824*(if v1789{(((v1859*(v6630-v6456))-(v1860*v6630))/v6640)}else{common.v4})))/v6666)}else{common.v4});
        let v6686=(if v1866{(((v1862*v6461)-(v1824*(if v1789{(((v1859*(v6631-v6457))-(v1860*v6631))/v6640)}else{common.v4})))/v6666)}else{common.v4});
        let v6687=(if v1866{(((v1862*v6462)-(v1824*(if v1789{(((v1859*(v6632-v6458))-(v1860*v6632))/v6640)}else{common.v4})))/v6666)}else{common.v4});
        let v6718=(((v1859*(-v2959))-(v1872*v6628))/v6640);
        let v6721=((-(v1872*v6629))/v6640);
        let v6724=((-(v1872*v6630))/v6640);
        let v6727=((-(v1872*v6631))/v6640);
        let v6730=((-(v1872*v6632))/v6640);
        let v6731=(v1874*v6718);
        let v6732=(v1874*v6721);
        let v6733=(v1874*v6724);
        let v6734=(v1874*v6727);
        let v6735=(v1874*v6730);
        let v6739=(v1868*v1868);
        let v6824=(self.scalar_static_f64[254]*f64::powf(common.v1761,self.scalar_static_f64[335]));
        let v6830=(common.v1894*common.v1894);
        let v6855=(self.scalar_static_f64[268]*f64::powf(common.v1896,self.scalar_static_f64[336]));
        let v6870=(if common.v1891{(common.v1892*((-(((common.v1894*v4442)-(common.v1198*v4442))/v6830))*v6855))}else{common.v4});
        let v6871=(if common.v1891{(common.v1892*((-(((common.v1894*v4446)-(common.v1198*v4446))/v6830))*v6855))}else{common.v4});
        let v6872=(if common.v1891{((common.v1898*(self.scalar_static_f64[299]*v6824))+(common.v1892*((-(((common.v1894*v4450)-(common.v1198*v4450))/v6830))*v6855)))}else{common.v4});
        let v6873=(if common.v1891{((common.v1898*(self.scalar_static_f64[0]*v6824))+(common.v1892*((-(((common.v1894*v4454)-(common.v1198*v4454))/v6830))*v6855)))}else{common.v4});
        let v6874=(if common.v1891{(common.v1892*((-(((common.v1894*v4458)-(common.v1198*v4458))/v6830))*v6855))}else{common.v4});
        let v6885=(if common.v1903{(v4442/self.scalar_static_f64[267])}else{common.v4});
        let v6886=(if common.v1903{(v4446/self.scalar_static_f64[267])}else{common.v4});
        let v6887=(if common.v1903{(v4450/self.scalar_static_f64[267])}else{common.v4});
        let v6888=(if common.v1903{(v4454/self.scalar_static_f64[267])}else{common.v4});
        let v6889=(if common.v1903{(v4458/self.scalar_static_f64[267])}else{common.v4});
        let v6895=(if common.v1903{(v6885/self.scalar_static_f64[270])}else{common.v4});
        let v6896=(if common.v1903{(v6886/self.scalar_static_f64[270])}else{self.scalar_static_f64[313]});
        let v6897=(if common.v1903{(v6887/self.scalar_static_f64[270])}else{self.scalar_static_f64[314]});
        let v6898=(if common.v1903{(v6888/self.scalar_static_f64[270])}else{common.v4});
        let v6899=(if common.v1903{(v6889/self.scalar_static_f64[270])}else{common.v4});
        let v6952=(self.scalar_static_f64[271]*f64::powf(common.v1928,self.scalar_static_f64[337]));
        let v6980=((common.v1932*v6245)+(common.v1764*(if common.v1903{((common.v1930*v6870)+(common.v1900*((if common.v1921{(v6885+(self.scalar_static_f64[270]*((common.v1923*(-v6895))/common.v1924)))}else{(if common.v1913{(self.scalar_static_f64[270]*((common.v1914*v6895)/common.v1915))}else{common.v4})})*v6952)))}else{(if common.v1901{v6870}else{common.v4})})));
        let v6981=(common.v1764*(if common.v1903{((common.v1930*v6871)+(common.v1900*((if common.v1921{(v6886+(self.scalar_static_f64[270]*((common.v1923*(-v6896))/common.v1924)))}else{(if common.v1913{(self.scalar_static_f64[270]*((common.v1914*v6896)/common.v1915))}else{common.v4})})*v6952)))}else{(if common.v1901{v6871}else{common.v4})}));
        let v6982=(common.v1764*(if common.v1903{((common.v1930*v6872)+(common.v1900*((if common.v1921{(v6887+(self.scalar_static_f64[270]*((common.v1923*(-v6897))/common.v1924)))}else{(if common.v1913{(self.scalar_static_f64[270]*((common.v1914*v6897)/common.v1915))}else{common.v4})})*v6952)))}else{(if common.v1901{v6872}else{common.v4})}));
        let v6983=(common.v1764*(if common.v1903{((common.v1930*v6873)+(common.v1900*((if common.v1921{(v6888+(self.scalar_static_f64[270]*((common.v1923*(-v6898))/common.v1924)))}else{(if common.v1913{(self.scalar_static_f64[270]*((common.v1914*v6898)/common.v1915))}else{common.v4})})*v6952)))}else{(if common.v1901{v6873}else{common.v4})}));
        let v6984=(common.v1764*(if common.v1903{((common.v1930*v6874)+(common.v1900*((if common.v1921{(v6889+(self.scalar_static_f64[270]*((common.v1923*(-v6899))/common.v1924)))}else{(if common.v1913{(self.scalar_static_f64[270]*((common.v1914*v6899)/common.v1915))}else{common.v4})})*v6952)))}else{(if common.v1901{v6874}else{common.v4})}));
        let v7019=(if common.v1891{((v1945*(if common.v1939{(common.v1940*v6980)}else{(if v1935{(v1936*v6980)}else{v6276})}))+(v1944*(common.v1761*v6284)))}else{(if v1883{((v1884*v6731)+(v1874*(self.scalar_static_f64[3]*v6377)))}else{(if v1866{((v1879*((v1870*v6683)+(v1868*((v1869*v6628)+(v1859*((-(self.scalar_static_f64[3]*v2959))/(v659*v659)))))))+(v1871*(v6731-(v1878*((v1876*v6718)+(v1873*(((v1868*v6377)-(v1814*v6683))/v6739)))))))}else{(if common.v1750{((v1781*v6276)+(v1778*((v1780*v6240)+(common.v1763*v6284))))}else{common.v4})})})});
        let v7020=(if common.v1891{(v1945*(if common.v1939{(common.v1940*v6981)}else{(if v1935{(v1936*v6981)}else{v6277})}))}else{(if v1883{(v1884*v6732)}else{(if v1866{((v1879*((v1870*v6684)+(v1868*(v1869*v6629))))+(v1871*(v6732-(v1878*((v1876*v6721)+(v1873*((-(v1814*v6684))/v6739)))))))}else{(if common.v1750{((v1781*v6277)+(v1778*(v1780*v6241)))}else{common.v4})})})});
        let v7021=(if common.v1891{((v1945*(if common.v1939{(common.v1940*v6982)}else{(if v1935{(v1936*v6982)}else{v6278})}))+(v1944*(v1780*self.scalar_static_f64[299])))}else{(if v1883{((v1884*v6733)+(v1874*(self.scalar_static_f64[3]*v6378)))}else{(if v1866{((v1879*((v1870*v6685)+(v1868*(v1869*v6630))))+(v1871*(v6733-(v1878*((v1876*v6724)+(v1873*(((v1868*v6378)-(v1814*v6685))/v6739)))))))}else{(if common.v1750{((v1781*v6278)+(v1778*(v1780*v6242)))}else{common.v4})})})});
        let v7022=(if common.v1891{((v1945*(if common.v1939{(common.v1940*v6983)}else{(if v1935{(v1936*v6983)}else{v6279})}))+(v1944*(self.scalar_static_f64[0]*v1780)))}else{(if v1883{((v1884*v6734)+(v1874*(self.scalar_static_f64[3]*v6379)))}else{(if v1866{((v1879*((v1870*v6686)+(v1868*(v1869*v6631))))+(v1871*(v6734-(v1878*((v1876*v6727)+(v1873*(((v1868*v6379)-(v1814*v6686))/v6739)))))))}else{(if common.v1750{((v1781*v6279)+(v1778*(v1780*v6243)))}else{common.v4})})})});
        let v7023=(if common.v1891{(v1945*(if common.v1939{(common.v1940*v6984)}else{(if v1935{(v1936*v6984)}else{v6280})}))}else{(if v1883{((v1884*v6735)+(v1874*(self.scalar_static_f64[3]*v6380)))}else{(if v1866{((v1879*((v1870*v6687)+(v1868*(v1869*v6632))))+(v1871*(v6735-(v1878*((v1876*v6730)+(v1873*(((v1868*v6380)-(v1814*v6687))/v6739)))))))}else{(if common.v1750{((v1781*v6280)+(v1778*(v1780*v6244)))}else{common.v4})})})});
        let v7024=(v2675+v6170);
        let v7043=(v1954*v1954);
        let v7080=(v1953*v1953);
        let v7099=(if v1952{(((((v1954*common.v2473)-(common.v119*((v1953*v4442)+(common.v1198*v7024))))/v7043)+((v1956*v2785)+(v487*(((common.v436*common.v4409)-(common.v1192*common.v2751))/v4566))))+(((v1953*v2668)-(v308*v7024))/v7080))}else{common.v4});
        let v7100=(if v1952{((((-(common.v119*((v1953*v4446)+(common.v1198*v6171))))/v7043)+(v487*(common.v4412/common.v436)))+((-(v308*v6171))/v7080))}else{common.v4});
        let v7101=(if v1952{((((-(common.v119*((v1953*v4450)+(common.v1198*v6172))))/v7043)+(v487*(common.v4415/common.v436)))+((-(v308*v6172))/v7080))}else{common.v4});
        let v7102=(if v1952{((((-(common.v119*((v1953*v4454)+(common.v1198*v6173))))/v7043)+(v487*(common.v4418/common.v436)))+((-(v308*v6173))/v7080))}else{common.v4});
        let v7103=(if v1952{((((-(common.v119*((v1953*v4458)+(common.v1198*v6174))))/v7043)+(v487*(common.v4421/common.v436)))+((-(v308*v6174))/v7080))}else{common.v4});
        let v7114=(if v1962{((v7019-v7099)/common.v408)}else{v6895});
        let v7115=(if v1962{((v7020-v7100)/common.v408)}else{v6896});
        let v7116=(if v1962{((v7021-v7101)/common.v408)}else{v6897});
        let v7117=(if v1962{((v7022-v7102)/common.v408)}else{v6898});
        let v7118=(if v1962{((v7023-v7103)/common.v408)}else{v6899});
        let v7169=(if v1975{(v7099-(common.v408*((v1977*(-v7114))/v1978)))}else{(if v1967{(v7019-(common.v408*((v1968*v7114)/v1969)))}else{v7019})});
        let v7170=(if v1975{(v7100-(common.v408*((v1977*(-v7115))/v1978)))}else{(if v1967{(v7020-(common.v408*((v1968*v7115)/v1969)))}else{v7020})});
        let v7171=(if v1975{(v7101-(common.v408*((v1977*(-v7116))/v1978)))}else{(if v1967{(v7021-(common.v408*((v1968*v7116)/v1969)))}else{v7021})});
        let v7172=(if v1975{(v7102-(common.v408*((v1977*(-v7117))/v1978)))}else{(if v1967{(v7022-(common.v408*((v1968*v7117)/v1969)))}else{v7022})});
        let v7173=(if v1975{(v7103-(common.v408*((v1977*(-v7118))/v1978)))}else{(if v1967{(v7023-(common.v408*((v1968*v7118)/v1969)))}else{v7023})});
        let v7176=((v1982*v4442)+(common.v1198*v7169));
        let v7179=((v1982*v4446)+(common.v1198*v7170));
        let v7182=((v1982*v4450)+(common.v1198*v7171));
        let v7185=((v1982*v4454)+(common.v1198*v7172));
        let v7188=((v1982*v4458)+(common.v1198*v7173));
        let v7217=(v1988*v1988);
        let v7240=(if v1992{v7176}else{(if v1986{(((v1988*((v1983*v7099)+(v1961*v7176)))-(v1987*(v7099+v7169)))/v7217)}else{(if v1962{v7176}else{common.v4})})});
        let v7241=(if v1992{v7179}else{(if v1986{(((v1988*((v1983*v7100)+(v1961*v7179)))-(v1987*(v7100+v7170)))/v7217)}else{(if v1962{v7179}else{common.v4})})});
        let v7242=(if v1992{v7182}else{(if v1986{(((v1988*((v1983*v7101)+(v1961*v7182)))-(v1987*(v7101+v7171)))/v7217)}else{(if v1962{v7182}else{common.v4})})});
        let v7243=(if v1992{v7185}else{(if v1986{(((v1988*((v1983*v7102)+(v1961*v7185)))-(v1987*(v7102+v7172)))/v7217)}else{(if v1962{v7185}else{common.v4})})});
        let v7244=(if v1992{v7188}else{(if v1986{(((v1988*((v1983*v7103)+(v1961*v7188)))-(v1987*(v7103+v7173)))/v7217)}else{(if v1962{v7188}else{common.v4})})});
        let v7259=(if v1998{common.v4}else{(if v1994{((common.v1995*common.v2473)+(common.v119*(common.v3830/common.v1024)))}else{common.v4})});
        let v7260=(if v1998{self.scalar_static_f64[0]}else{(if v1994{(common.v119*(common.v3831/common.v1024))}else{common.v4})});
        let v7261=(if v1998{common.v4}else{(if v1994{(common.v119*(common.v3832/common.v1024))}else{common.v4})});
        let v7262=(if v1998{self.scalar_static_f64[299]}else{(if v1994{(common.v119*(common.v3833/common.v1024))}else{common.v4})});
        let v7324=(v710*self.scalar_static_f64[299]);
        let v7329=(v308*v308);
        let v7335=(common.v731*self.scalar_static_f64[300]);
        let v7337=(common.v731*self.scalar_static_f64[301]);
        let v7339=(common.v731*self.scalar_static_f64[299]);
        let v7342=(v674*(v7335+v7335));
        let v7344=(v674*(v7337+v7337));
        let v7351=(common.v724*self.scalar_static_f64[299]);
        let v7359=(common.v721*self.scalar_static_f64[299]);
        let v7369=(common.v713*self.scalar_static_f64[299]);
        let v7374=(v322*v322);
        let v7402=(((if self.scalar_static_bool[33]{((v1322*v2785)+(v487*((self.scalar_static_f64[220]*common.v4544)+((v1320*common.v4263)+(v1299*(self.scalar_static_f64[219]*(common.v3830+common.v4544)))))))}else{(if self.scalar_static_bool[31]{v4594}else{(if self.scalar_static_bool[12]{((v4594+((v1299*(((v1297*((v1292*common.v4544)+(v1290*(common.v31*(if self.scalar_static_bool[12]{(self.scalar_static_f64[144]*(v503*((self.scalar_static_f64[146]*common.v2476)/self.scalar_static_f64[138])))}else{common.v4})))))-(v1293*((common.v423*v4560)/v4606)))/v4613))+(v1298*common.v4263)))+(((v1305*((v1303*v4587)+(v1289*((v1302*(if self.scalar_static_bool[12]{(self.scalar_static_f64[147]*(v510*(self.scalar_static_f64[149]*common.v2476)))}else{common.v4}))+(v512*common.v3830)))))-(v1304*v4587))/v4657))}else{common.v4})})})+((v1371*((v460*(self.scalar_static_f64[124]*(v454*(self.scalar_static_f64[127]*common.v2477))))+(v455*(v460*(v2761/self.scalar_static_f64[125])))))+(v461*common.v4826)))-(if v1498{common.v4}else{(if common.v1412{(self.scalar_static_f64[19]*((v1494*common.v2640)+(common.v284*((v1493*(if common.v1422{(common.v1423*v4941)}else{(if v1418{(v1419*v4941)}else{common.v4})}))+(v1427*((v1492*common.v3972)+(common.v1080*((v1491*(if v1479{((v1488*(v1480*v5052))+(v1481*((v1486*(v1482*v5052))+(v1483*(v1484*v5052)))))}else{(if common.v1461{(v1472*(((common.v1457*(-(if common.v1466{(common.v1467*v5052)}else{(if v1462{(v1463*v5052)}else{common.v4})})))-(v1473*v5052))/v5073))}else{common.v4})}))+(v1490*(common.v31*((v579*((v576*common.v2647)+(common.v287*(self.scalar_static_f64[45]*(self.scalar_static_f64[45]*((v573*common.v2547)+(common.v196*((v572*common.v2547)+(common.v196*(self.scalar_static_f64[168]*v2844))))))))))+(v577*(v579*(-v2863))))))))))))))}else{common.v4})}));
        let v7403=((((if self.scalar_static_bool[33]{(v487*((self.scalar_static_f64[220]*common.v4545)+(v1299*(self.scalar_static_f64[219]*common.v4545))))}else{(if self.scalar_static_bool[31]{v4595}else{(if self.scalar_static_bool[12]{((v4595+(v1299*(((v1297*(v1292*common.v4545))-(v1293*((common.v423*v4561)/v4606)))/v4613)))+(((v1305*(v1303*v4588))-(v1304*v4588))/v4657))}else{common.v4})})})+(v461*common.v4827))+self.scalar_static_f64[343])-(if v1498{common.v4}else{(if common.v1412{(self.scalar_static_f64[19]*(common.v284*((v1493*(if common.v1422{(common.v1423*v4942)}else{(if v1418{(v1419*v4942)}else{common.v4})}))+(v1427*((v1492*common.v3973)+(common.v1080*(v1491*(if v1479{((v1488*((v1480*v5053)+(common.v1457*self.scalar_static_f64[320])))+(v1481*((v1486*(v1482*v5053))+(v1483*(v1484*v5053)))))}else{(if common.v1461{((self.scalar_static_f64[0]*v1475)+(v1472*(((common.v1457*(-(if common.v1466{(common.v1467*v5053)}else{(if v1462{(v1463*v5053)}else{common.v4})})))-(v1473*v5053))/v5073)))}else{common.v4})}))))))))}else{common.v4})}));
        let v7404=((((if self.scalar_static_bool[33]{(v487*((self.scalar_static_f64[220]*common.v4546)+((v1320*common.v4264)+(v1299*(self.scalar_static_f64[219]*(common.v3831+common.v4546))))))}else{(if self.scalar_static_bool[31]{v4596}else{(if self.scalar_static_bool[12]{((v4596+((v1299*(((v1297*(v1292*common.v4546))-(v1293*((common.v423*v4562)/v4606)))/v4613))+(v1298*common.v4264)))+(((v1305*((v1303*v4589)+(v1289*(v512*common.v3831))))-(v1304*v4589))/v4657))}else{common.v4})})})+(v461*common.v4829))+self.scalar_static_f64[344])-(if v1498{common.v4}else{(if common.v1412{(self.scalar_static_f64[19]*(common.v284*((v1493*(if common.v1422{(common.v1423*v4943)}else{(if v1418{(v1419*v4943)}else{common.v4})}))+(v1427*((v1492*common.v3974)+(common.v1080*(v1491*(if v1479{((v1488*((v1480*v5054)+(common.v1457*self.scalar_static_f64[321])))+(v1481*((v1486*(v1482*v5054))+(v1483*(v1484*v5054)))))}else{(if common.v1461{((v1475*self.scalar_static_f64[299])+(v1472*(((common.v1457*(-(if common.v1466{(common.v1467*v5054)}else{(if v1462{(v1463*v5054)}else{common.v4})})))-(v1473*v5054))/v5073)))}else{common.v4})}))))))))}else{common.v4})}));
        let v7407=((v1226*((v556*(self.scalar_static_f64[165]*(common.v2472/(common.v31*v552))))+(v553*(v556*(self.scalar_static_f64[166]*common.v2471)))))+v7402);
        let v7408=((v557*v4489)+(((v1249*(self.scalar_static_f64[218]*v4516))+(v1247*((-v4516)*v4523)))+v7403));
        let v7409=((v557*v4490)+(((v1249*(self.scalar_static_f64[218]*v4517))+(v1247*((-v4517)*v4523)))+v7404));
        let v7455=(((v1407*((v549*(self.scalar_static_f64[162]*(v546*(self.scalar_static_f64[164]*common.v2477))))+(v547*(v549*(v2761/self.scalar_static_f64[163])))))+(v550*common.v4909))+((if self.scalar_static_bool[30]{v4764}else{(if self.scalar_static_bool[12]{(v4764+(((v1356*((v1351*common.v4742)+(v1349*(common.v31*(if self.scalar_static_bool[12]{(self.scalar_static_f64[150]*(v518*((self.scalar_static_f64[152]*common.v2476)/self.scalar_static_f64[142])))}else{common.v4})))))-(v1352*((common.v423*(if common.v1343{(common.v1344*v4747)}else{(if v1339{(v1340*v4747)}else{v4560})}))/v4779)))/v4787))}else{common.v4})})+((v1383*((v540*(self.scalar_static_f64[158]*(v537*(self.scalar_static_f64[161]*common.v2477))))+(v538*(v540*(v2761/self.scalar_static_f64[159])))))+(v541*common.v4849))));
        let v7456=((v550*common.v4910)+((if self.scalar_static_bool[30]{v4765}else{(if self.scalar_static_bool[12]{(v4765+(((v1356*(v1351*common.v4743))-(v1352*((common.v423*(if common.v1343{(common.v1344*common.v2985)}else{(if v1339{(v1340*common.v2985)}else{v4561})}))/v4779)))/v4787))}else{common.v4})})+(v541*common.v4850)));
        let v7457=((v550*common.v4911)+((if self.scalar_static_bool[30]{v4766}else{(if self.scalar_static_bool[12]{(v4766+(((v1356*(v1351*common.v4744))-(v1352*((common.v423*(if common.v1343{(common.v1344*common.v2984)}else{(if v1339{(v1340*common.v2984)}else{common.v4})}))/v4779)))/v4787))}else{common.v4})})+(v541*common.v4851)));
        let v7458=((v550*common.v4912)+((if self.scalar_static_bool[30]{v4767}else{(if self.scalar_static_bool[12]{(v4767+(((v1356*(v1351*common.v4745))-(v1352*((common.v423*(if common.v1343{common.v4}else{(if v1339{common.v4}else{v4562})}))/v4779)))/v4787))}else{common.v4})})+(v541*common.v4852)));
        let v7466=(common.v703*v4921);
        let v7475=((v1615*v5977)+(v1396*v5977));
        let v7476=((v1615*v5978)+(v1396*v5978));
        let v7477=(((v1715*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v5518)}else{v5518}))+(v1615*v5979))+((v1715*((v1395*((v472*(self.scalar_static_f64[130]*(v467*(self.scalar_static_f64[133]*common.v2477))))+(v468*(v472*((self.scalar_static_f64[134]*common.v2476)/self.scalar_static_f64[131])))))+(v473*common.v4880)))+(v1396*v5979)));
        let v7478=((v1615*v5980)+((v1715*(v473*common.v4881))+(v1396*v5980)));
        let v7483=((v1615*v5985)+(v1396*v5985));
        let v7502=(v2042*self.scalar_static_f64[301]);
        let v7521=(v1719*self.scalar_static_f64[300]);
        let v7533=(v1719*self.scalar_static_f64[301]);
        let v8651=ddt_scale;
        let v8860=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4921));
        let v8894=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5987)));
        let v8895=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5988)));
        let v8896=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5991)));
        let v8897=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5992)));
        let v8898=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5993)));
        let v8899=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5996)));
        let v8900=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v5999)));
        let v8901=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v6000)));
        let v8902=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v6001)));
        let v8903=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v6002)));
        let v9098=(self.scalar_static_f64[13]*(v674*self.scalar_static_f64[363]));
        let v9100=(self.scalar_static_f64[13]*(v674*self.scalar_static_f64[364]));
        let v9120=(self.scalar_static_f64[13]*(v8651*common.v9102));
        let v9167=(self.scalar_static_f64[13]*(v8651*common.v9157));

        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v849))),
            [3, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v3217)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v3218)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v3219)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v3220))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v1198))),
            [3, 4, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4442)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4446)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4450)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4454)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4458))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2399)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7455)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7456)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7457)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7458)), v8860, v8860, (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4922))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2401)),
            [3, 4, 5, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7407)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7408)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4834)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7409)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4726)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v4727))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[12]{v2405}else{common.v4})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[12]{v8894}else{common.v4}), (if self.scalar_static_bool[12]{v8895}else{common.v4}), (if self.scalar_static_bool[12]{v8896}else{common.v4}), (if self.scalar_static_bool[12]{v8897}else{common.v4}), (if self.scalar_static_bool[12]{v8898}else{common.v4}), (if self.scalar_static_bool[12]{v8899}else{common.v4}), (if self.scalar_static_bool[12]{v8900}else{common.v4}), (if self.scalar_static_bool[12]{v8901}else{common.v4}), (if self.scalar_static_bool[12]{v8902}else{common.v4}), (if self.scalar_static_bool[12]{v8903}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[30]{v2405}else{common.v4})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[30]{v8894}else{common.v4}), (if self.scalar_static_bool[30]{v8895}else{common.v4}), (if self.scalar_static_bool[30]{v8896}else{common.v4}), (if self.scalar_static_bool[30]{v8897}else{common.v4}), (if self.scalar_static_bool[30]{v8898}else{common.v4}), (if self.scalar_static_bool[30]{v8899}else{common.v4}), (if self.scalar_static_bool[30]{v8900}else{common.v4}), (if self.scalar_static_bool[30]{v8901}else{common.v4}), (if self.scalar_static_bool[30]{v8902}else{common.v4}), (if self.scalar_static_bool[30]{v8903}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[13]*v2408)),
            [3, 4, 5, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6186)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6189)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6190)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6194)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6197)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6200))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v1993)))),
            [3, 4, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v7240))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v7241))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v7242))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v7243))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v7244)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*(v2412/v308))),
            2,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[357]/v308))),
            3,
            multiplicity * ((self.scalar_static_f64[13]*((-(v2412*v2668))/v7329))),
            4,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[358]/v308))),
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(5),
            multiplicity * ((self.scalar_static_f64[13]*(v2415/v322))),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[357]/v322))),
            3,
            multiplicity * ((self.scalar_static_f64[13]*((-(v2415*v2675))/v7374))),
            5,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[358]/v322))),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((if self.scalar_static_bool[77]{(common.v101/self.scalar_static_f64[12])}else{(if self.scalar_static_bool[76]{(self.scalar_static_f64[373]*(f64::powf(v2339,self.scalar_static_f64[289])-common.v1))}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[370]*(v2339).ln())}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[13]*(common.v101/self.scalar_static_f64[368]))}else{common.v4})})})})),
            3,
            multiplicity * ((if self.scalar_static_bool[77]{self.scalar_static_f64[356]}else{(if self.scalar_static_bool[76]{(self.scalar_static_f64[373]*(self.scalar_static_f64[377]*(self.scalar_static_f64[289]*f64::powf(v2339,self.scalar_static_f64[355]))))}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[370]*(self.scalar_static_f64[377]/v2339))}else{self.scalar_static_f64[376]})})})),
        );
        stamper.stamp_current_node1_local(
            Some(3),
            None,
            multiplicity * ((self.scalar_static_f64[13]*v2321)),
            3,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[288]*v8651))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(3),
            None,
            multiplicity * ((self.scalar_static_f64[13]*(-((((((((((((((common.v1198*v2002)+(common.v849*v2004))-(v1993*v1999))+(v2009/v308))+(v674*v2012))+(v682*v2015))+(v690*v2018))+(v2021/v322))+(common.v705*v1739))+(common.v700*v2031))-(v1716*v2001))+(common.v703*v2037))+(common.v727*v2042))+(common.v732*v1719))))),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            &[(self.scalar_static_f64[13]*(-((((v674*(v2442+v2442))-(v2001*v5987))+(common.v727*v7475))+(v7521+(common.v732*v6049))))), (self.scalar_static_f64[13]*(-((((v7342+((v2415+v2415)/v322))-(v2001*v5988))+(common.v727*v7476))+((v1719*self.scalar_static_f64[302])+(common.v732*v6052))))), (self.scalar_static_f64[13]*(-((v2412+v2412)/v308))), (self.scalar_static_f64[13]*(-(((((((((((((((v2002*v4442)+(common.v1198*(-v7259)))+((v2004*common.v3217)+(common.v849*v7259)))-((v1999*v7240)+(v1993*v7259)))+((-(v2009*v2668))/v7329))+(v2012*v2966))+(v2015*v2972))+(v2018*v2978))+((-(v2021*v2675))/v7374))+(common.v705*v6186))+(common.v700*v7407))-(v2001*v5991))+(common.v703*v7455))+(common.v727*v7477))+(common.v732*v6055)))), (self.scalar_static_f64[13]*(-((((((((((v2002*v4446)+(common.v1198*self.scalar_static_f64[299]))-(v1999*v7241))+((v7324+v7324)/v308))+(common.v705*v6189))+((v2031*self.scalar_static_f64[299])+(common.v700*v7408)))-(v2001*v5992))+((v2037*self.scalar_static_f64[299])+(common.v703*v7456)))+(common.v727*v7478))+(common.v732*v6058)))), (self.scalar_static_f64[13]*(-(((((((v7342+((v7369+v7369)/v322))+(v2408+(common.v705*v6190)))+(common.v700*v4834))-(v2001*v5993))+(v2399+(common.v703*v7457)))+((self.scalar_static_f64[0]*v2042)+(common.v727*(self.scalar_static_f64[344]+(v6011+v6034)))))+(v7521+(common.v732*v6060))))), (self.scalar_static_f64[13]*(-(((((((((((v2002*v4450)+(common.v1198*(self.scalar_static_f64[0]-v7260)))+((v2004*common.v3218)+(common.v849*(v7260-self.scalar_static_f64[0]))))-((v1999*v7242)+(v1993*v7260)))+v7342)+((v1739*self.scalar_static_f64[299])+(common.v705*v6194)))+(v2401+(common.v700*v7409)))-((v2001*v5996)+(v1716*self.scalar_static_f64[340])))+(common.v703*v7458))+((v2042*self.scalar_static_f64[300])+(common.v727*((v6014+v6037)+self.scalar_static_f64[345]))))+(v7521+(common.v732*v6063))))), (self.scalar_static_f64[13]*(-((((((((((((v2002*v4454)+(common.v1198*(-v7261)))+((v2004*common.v3219)+(common.v849*(v7261-self.scalar_static_f64[299]))))-((v1999*v7243)+(v1993*v7261)))+v7344)+(v690*(v7359+v7359)))+(common.v705*v6197))+(common.v700*v4726))-((v2001*v5999)+(v1716*self.scalar_static_f64[341])))+v7466)+(v7502+(common.v727*((v6017+v6040)+self.scalar_static_f64[346]))))+(v7533+(common.v732*v6066))))), (self.scalar_static_f64[13]*(-(((((((((((v2002*v4458)+(common.v1198*(-v7262)))+((v2004*common.v3220)+(common.v849*v7262)))-((v1999*v7244)+(v1993*v7262)))+v7344)+(common.v705*v6200))+(common.v700*v4727))-((v2001*v6000)+(v1716*self.scalar_static_f64[342])))+v7466)+(v7502+(common.v727*((v6019+v6042)+self.scalar_static_f64[346]))))+(v7533+(common.v732*v6069))))), (self.scalar_static_f64[13]*(-(((((v674*(v7339+v7339))+(v682*(v2454+v2454)))-(v2001*v6001))+(common.v727*v7483))+((v1719*self.scalar_static_f64[299])+(common.v732*v6072))))), (self.scalar_static_f64[13]*(-((((((v7344+(v682*(v7351+v7351)))+(v690*(v2458+v2458)))-(v2001*v6002))+(common.v703*v4922))+((v2042*self.scalar_static_f64[299])+(common.v727*(self.scalar_static_f64[343]+(v6023+v6046)))))+(v7533+(common.v732*v6075)))))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2421)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v8651*common.v8986)), (self.scalar_static_f64[13]*(v8651*common.v8987)), (self.scalar_static_f64[13]*(v8651*common.v8988)), (self.scalar_static_f64[13]*(v8651*common.v8989)), (self.scalar_static_f64[13]*(v8651*common.v8990)), (self.scalar_static_f64[13]*(v8651*common.v8991)), (self.scalar_static_f64[13]*(v8651*common.v8992))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2424)),
            3,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9007))),
            4,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9008))),
            5,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9009))),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*v2427)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v8651*common.v9016)), (self.scalar_static_f64[13]*(v8651*common.v9017)), (self.scalar_static_f64[13]*(v8651*common.v9018)), (self.scalar_static_f64[13]*(v8651*common.v9019)), (self.scalar_static_f64[13]*(v8651*common.v9020)), (self.scalar_static_f64[13]*(v8651*common.v9021)), (self.scalar_static_f64[13]*(v8651*common.v9022))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[13]*v2430)),
            [3, 4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v8651*common.v9037)), (self.scalar_static_f64[13]*(v8651*common.v9038)), (self.scalar_static_f64[13]*(v8651*common.v9039)), (self.scalar_static_f64[13]*(v8651*common.v9040)), (self.scalar_static_f64[13]*(v8651*common.v9041)), (self.scalar_static_f64[13]*(v8651*common.v9042)), (self.scalar_static_f64[13]*(v8651*common.v9043))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[13]*v2434)),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[359]))),
            2,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[360]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[13]*v2438)),
            0,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[361]))),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[362]))),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v1719))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6049)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6052)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6055)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6058)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6060)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6063)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6066)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6069)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6072)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v6075))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(0),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*(v674*v2442))),
            [0, 1, 3, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[13]*(v674*self.scalar_static_f64[357])), v9098, (self.scalar_static_f64[13]*(v2442*v2966)), v9098, v9098, v9100, v9100, (self.scalar_static_f64[13]*(v674*self.scalar_static_f64[358])), v9100],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*v2446)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v9120, (self.scalar_static_f64[13]*(v8651*common.v9103)), (self.scalar_static_f64[13]*(v8651*common.v9104)), (self.scalar_static_f64[13]*(v8651*common.v9105)), v9120, (self.scalar_static_f64[13]*(v8651*common.v9106)), (self.scalar_static_f64[13]*(v8651*common.v9107)), (self.scalar_static_f64[13]*(v8651*common.v9108)), (self.scalar_static_f64[13]*(v8651*common.v9109)), (self.scalar_static_f64[13]*(v8651*common.v9110))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1717+(v1718+v2041))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7475)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7476)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7477)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7478)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v6011+(v6034+self.scalar_static_f64[344])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v6014+(v6037+self.scalar_static_f64[345])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v6017+(v6040+self.scalar_static_f64[346])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v6019+(v6042+self.scalar_static_f64[346])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v7483)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v6023+(v6046+self.scalar_static_f64[343]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[13]*v2452)),
            [3, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v8651*common.v9154)), (self.scalar_static_f64[13]*(v8651*common.v9155)), (self.scalar_static_f64[13]*(v8651*common.v9156)), v9167, v9167, (self.scalar_static_f64[13]*(v8651*common.v9158))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(9),
            Some(10),
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(v682*v2454))}else{common.v4})),
            3,
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(v2454*v2972))}else{common.v4})),
            9,
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(v682*self.scalar_static_f64[357]))}else{common.v4})),
            10,
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(v682*self.scalar_static_f64[358]))}else{common.v4})),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v4,
        );
        stamper.stamp_current_node3_local(
            Some(10),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(v690*v2458))}else{common.v4})),
            3,
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(v2458*v2978))}else{common.v4})),
            7,
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(v690*self.scalar_static_f64[358]))}else{common.v4})),
            10,
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(v690*self.scalar_static_f64[357]))}else{common.v4})),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v4,
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (common.v4),
        );
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (common.v2462),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(4),
            multiplicity * ((common.v2384*v2463)),
            [3, 4, 5, 6, 7, 8, 10, 11],
            [(v2463*common.v8784), (v2463*common.v8785), (v2463*common.v8786), (v2463*common.v8787), (v2463*common.v8788), (v2463*common.v8789), (v2463*common.v8790), (common.v2384*v8651)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((v2363*common.v2462)),
            11,
            multiplicity * (v2363),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (common.v2462),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (common.v4),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v2321=0.0;
        let v2421=0.0;
        let v2424=0.0;
        let v2427=0.0;
        let v2430=0.0;
        let v2434=0.0;
        let v2438=0.0;
        let v2446=0.0;
        let v2452=0.0;
        let v2463=0.0;
        let v8651=1.0;
        let v9120=(self.scalar_static_f64[13]*(v8651*common.v9102));
        let v9167=(self.scalar_static_f64[13]*(v8651*common.v9157));

        stamper.stamp_current_reactive_node1(
            Some(nodes[3]),
            None,
            nodes[3],
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[288]*v8651))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v8651*common.v8986)), (self.scalar_static_f64[13]*(v8651*common.v8987)), (self.scalar_static_f64[13]*(v8651*common.v8988)), (self.scalar_static_f64[13]*(v8651*common.v8989)), (self.scalar_static_f64[13]*(v8651*common.v8990)), (self.scalar_static_f64[13]*(v8651*common.v8991)), (self.scalar_static_f64[13]*(v8651*common.v8992))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9007))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9008))),
            nodes[5],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*common.v9009))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v8651*common.v9016)), (self.scalar_static_f64[13]*(v8651*common.v9017)), (self.scalar_static_f64[13]*(v8651*common.v9018)), (self.scalar_static_f64[13]*(v8651*common.v9019)), (self.scalar_static_f64[13]*(v8651*common.v9020)), (self.scalar_static_f64[13]*(v8651*common.v9021)), (self.scalar_static_f64[13]*(v8651*common.v9022))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v8651*common.v9037)), (self.scalar_static_f64[13]*(v8651*common.v9038)), (self.scalar_static_f64[13]*(v8651*common.v9039)), (self.scalar_static_f64[13]*(v8651*common.v9040)), (self.scalar_static_f64[13]*(v8651*common.v9041)), (self.scalar_static_f64[13]*(v8651*common.v9042)), (self.scalar_static_f64[13]*(v8651*common.v9043))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[359]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[360]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[361]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v8651*self.scalar_static_f64[362]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[v9120, (self.scalar_static_f64[13]*(v8651*common.v9103)), (self.scalar_static_f64[13]*(v8651*common.v9104)), (self.scalar_static_f64[13]*(v8651*common.v9105)), v9120, (self.scalar_static_f64[13]*(v8651*common.v9106)), (self.scalar_static_f64[13]*(v8651*common.v9107)), (self.scalar_static_f64[13]*(v8651*common.v9108)), (self.scalar_static_f64[13]*(v8651*common.v9109)), (self.scalar_static_f64[13]*(v8651*common.v9110))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &[nodes[3], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v8651*common.v9154)), (self.scalar_static_f64[13]*(v8651*common.v9155)), (self.scalar_static_f64[13]*(v8651*common.v9156)), v9167, v9167, (self.scalar_static_f64[13]*(v8651*common.v9158))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]],
            &[(v2463*common.v8784), (v2463*common.v8785), (v2463*common.v8786), (v2463*common.v8787), (v2463*common.v8788), (v2463*common.v8789), (v2463*common.v8790), (common.v2384*v8651)],
            &[],
            &[],
            multiplicity,
        );
    }
}
