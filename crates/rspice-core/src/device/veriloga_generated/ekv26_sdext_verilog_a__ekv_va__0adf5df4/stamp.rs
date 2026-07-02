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
    v25: f64,
    v50: f64,
    v188: f64,
    v191: f64,
    v200: f64,
    v206: f64,
    v215: f64,
    v218: f64,
    v222: f64,
    v225: f64,
    v251: f64,
    v253: f64,
    v257: f64,
    v261: f64,
    v317: f64,
    v319: f64,
    v320: f64,
    v324: f64,
    v326: f64,
    v327: f64,
    v328: f64,
    v334: f64,
    v349: f64,
    v352: f64,
    v353: f64,
    v356: f64,
    v359: f64,
    v411: f64,
    v478: f64,
    v481: f64,
    v482: f64,
    v483: f64,
    v484: f64,
    v485: f64,
    v486: f64,
    v487: f64,
    v488: f64,
    v490: f64,
    v506: f64,
    v555: f64,
    v662: f64,
    v665: f64,
    v798: f64,
    v811: f64,
    v823: f64,
    v826: bool,
    v1079: f64,
    v1122: f64,
    v1131: bool,
    v1186: f64,
    v1187: f64,
    v1188: f64,
    v1196: f64,
    v1197: f64,
    v1198: f64,
    v1206: f64,
    v1207: f64,
    v1208: f64,
    v1216: f64,
    v1217: f64,
    v1218: f64,
    v1250: f64,
    v1251: f64,
    v1252: f64,
    v1253: f64,
    v1258: f64,
    v1259: f64,
    v1260: f64,
    v1261: f64,
    v1281: f64,
    v1282: f64,
    v1283: f64,
    v1284: f64,
    v1305: f64,
    v1306: f64,
    v1307: f64,
    v1308: f64,
    v1486: f64,
    v1487: f64,
    v1488: f64,
    v1489: f64,
    v1492: f64,
    v1495: f64,
    v1498: f64,
    v1501: f64,
    v1502: f64,
    v1503: f64,
    v1504: f64,
    v1505: f64,
    v1511: f64,
    v1512: f64,
    v1513: f64,
    v1514: f64,
    v1515: f64,
    v1516: f64,
    v1517: f64,
    v1518: f64,
    v1519: f64,
    v1520: f64,
    v1521: f64,
    v1522: f64,
    v1523: f64,
    v1524: f64,
    v1537: f64,
    v1538: f64,
    v1539: f64,
    v1540: f64,
    v1600: f64,
    v1601: f64,
    v1602: f64,
    v1603: f64,
    v1604: f64,
    v1605: f64,
    v1606: f64,
    v1607: f64,
    v1608: f64,
    v1609: f64,
    v1610: f64,
    v1611: f64,
    v1625: f64,
    v1626: f64,
    v1627: f64,
    v1628: f64,
    v1642: f64,
    v1643: f64,
    v1644: f64,
    v1645: f64,
    v1834: f64,
    v1835: f64,
    v1836: f64,
    v1837: f64,
    v2077: f64,
    v2078: f64,
    v2079: f64,
    v2080: f64,
    v2083: f64,
    v2086: f64,
    v2089: f64,
    v2092: f64,
    v2093: f64,
    v2094: f64,
    v2095: f64,
    v2096: f64,
    v2097: f64,
    v2098: f64,
    v2099: f64,
    v2100: f64,
    v2101: f64,
    v2102: f64,
    v2103: f64,
    v2104: f64,
    v2106: f64,
    v2108: f64,
    v2110: f64,
    v2112: f64,
    v2117: f64,
    v2118: f64,
    v2119: f64,
    v2120: f64,
    v3173: f64,
    v4013: f64,
    v4016: f64,
    v4019: f64,
    v4022: f64,
    v4097: f64,
    v4100: f64,
    v4103: f64,
    v4106: f64,
    v4176: f64,
    v4177: f64,
    v4178: f64,
    v4179: f64,
    v4393: f64,
    v4396: f64,
    v4443: f64,
    v4446: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=0.0;
        let v9=3.0;
        let v25=0.5;
        let v50=1.0;
        let v183=ctx.node_voltage(nodes[3]);
        let v188=(self.scalar_static_f64[20]*(ctx.node_voltage(nodes[2])-v183));
        let v191=(self.scalar_static_f64[20]*(ctx.node_voltage(nodes[0])-v183));
        let v193=((v191-v188)<v1);
        let v197=(if v193{v191}else{v188});
        let v198=(if v193{(if v193{v188}else{v1})}else{v191});
        let v200=(if (!v193){v50}else{(if v193{-1.0}else{v1})});
        let v204=(self.scalar_static_f64[232]+(self.scalar_static_f64[210]+(((self.scalar_static_f64[20]*(ctx.node_voltage(nodes[1])-v183))-self.scalar_static_f64[228])-self.scalar_static_f64[85])));
        let v206=2.0;
        let v209=(((v204*v204)+self.scalar_static_f64[233])).sqrt();
        let v211=(v25*(v204+v209));
        let v212=(self.scalar_static_f64[210]+v197);
        let v215=((self.scalar_static_f64[179]+(v212*v212))).sqrt();
        let v218=((v25*(v212+v215))).sqrt();
        let v219=(self.scalar_static_f64[210]+v198);
        let v222=((self.scalar_static_f64[179]+(v219*v219))).sqrt();
        let v225=((v25*(v219+v222))).sqrt();
        let v231=0.25;
        let v235=((v211+self.scalar_static_f64[92])).sqrt();
        let v236=(v211-self.scalar_static_f64[210]);
        let v243=((self.scalar_static_f64[173]+(self.scalar_static_f64[210]+(v236-(self.scalar_static_f64[67]*(v235-self.scalar_static_f64[93])))))).sqrt();
        let v248=((self.scalar_static_f64[67]-(self.scalar_static_f64[90]*(v218+v225)))+(self.scalar_static_f64[88]*v243));
        let v251=((self.scalar_static_f64[173]+(v248*v248))).sqrt();
        let v253=(v25*(v248+v251));
        let v254=(v231*v253);
        let v257=((v211+(v253*v254))).sqrt();
        let v259=(v257-(v25*v253));
        let v261=(v236-(v253*v259));
        let v263=(self.scalar_static_f64[174]*(v261-v197));
        let v264=-0.35;
        let v265=(v263>v264);
        let v266=1.3;
        let v268=1.6;
        let v269=(v263+v268);
        let v271=((v263+v266)-(v269).ln());
        let v273=(if v265{(v206/v271)}else{v1});
        let v274=(v206+v273);
        let v275=(v50+v263);
        let v277=(v275+(v273).ln());
        let v279=(if v265{(v274/v277)}else{v1});
        let v281=(v275+(v279).ln());
        let v282=(v206+v279);
        let v285=-15.0;
        let v286=(v263>v285);
        let v287=(!v265);
        let v288=(v286&&v287);
        let v289=1.55;
        let v291=((-v263)).exp();
        let v293=(if v288{(v289+v291)}else{v273});
        let v294=(v206+v293);
        let v296=(v275+(v293).ln());
        let v298=(if v288{(v294/v296)}else{v279});
        let v300=(v275+(v298).ln());
        let v301=(v206+v298);
        let v304=-23.0;
        let v305=(v263>v304);
        let v307=(v287&&(!v286));
        let v308=(v305&&v307);
        let v309=(v206+v291);
        let v313=(v307&&(!v305));
        let v314=(v263).exp();
        let v315=1e-64;
        let v317=(if v313{(v314+v315)}else{(if v308{(v50/v309)}else{(if v288{(v300/v301)}else{(if v265{(v281/v282)}else{v1})})})});
        let v318=(v50+v317);
        let v319=(v317*v318);
        let v320=(v319).sqrt();
        let v324=((v231+(v320*self.scalar_static_f64[234]))).sqrt();
        let v326=(self.scalar_static_f64[216]*(v324-v25));
        let v327=(v198-v197);
        let v328=(v25*v327);
        let v334=(self.scalar_static_f64[179]*((self.scalar_static_f64[5]*(v320-(self.scalar_static_f64[174]*v326)))+0.015625));
        let v343=0.75;
        let v349=((v231+(self.scalar_static_f64[234]*(v320-(v343*(v319).ln()))))).sqrt();
        let v352=(self.scalar_static_f64[221]+(self.scalar_static_f64[216]*(v349-v25)));
        let v353=(v328-v352);
        let v356=((v334+(v352*v352))).sqrt();
        let v359=((v334+(v353*v353))).sqrt();
        let v364=(self.scalar_static_f64[174]*(v359+(((v261-v328)-v197)-v356)));
        let v365=(v364>v264);
        let v367=(v268+v364);
        let v369=((v266+v364)-(v367).ln());
        let v371=(if v365{(v206/v369)}else{v293});
        let v372=(v206+v371);
        let v373=(v50+v364);
        let v375=(v373+(v371).ln());
        let v377=(if v365{(v372/v375)}else{v298});
        let v379=(v373+(v377).ln());
        let v380=(v206+v377);
        let v383=(v364>v285);
        let v384=(!v365);
        let v385=(v383&&v384);
        let v387=((-v364)).exp();
        let v389=(if v385{(v289+v387)}else{v371});
        let v390=(v206+v389);
        let v392=(v373+(v389).ln());
        let v394=(if v385{(v390/v392)}else{v377});
        let v396=(v373+(v394).ln());
        let v397=(v206+v394);
        let v400=(v364>v304);
        let v402=(v384&&(!v383));
        let v403=(v400&&v402);
        let v404=(v206+v387);
        let v408=(v402&&(!v400));
        let v409=(v364).exp();
        let v411=(if v408{(v315+v409)}else{(if v403{(v50/v404)}else{(if v385{(v396/v397)}else{(if v365{(v379/v380)}else{v317})})})});
        let v431=(self.scalar_static_f64[174]*(v261-v198));
        let v432=(v431>v264);
        let v434=(v268+v431);
        let v436=((v266+v431)-(v434).ln());
        let v438=(if v432{(v206/v436)}else{v389});
        let v439=(v206+v438);
        let v440=(v50+v431);
        let v442=(v440+(v438).ln());
        let v444=(if v432{(v439/v442)}else{v394});
        let v446=(v440+(v444).ln());
        let v447=(v206+v444);
        let v450=(v431>v285);
        let v451=(!v432);
        let v452=(v450&&v451);
        let v454=((-v431)).exp();
        let v456=(if v452{(v289+v454)}else{v438});
        let v457=(v206+v456);
        let v459=(v440+(v456).ln());
        let v461=(if v452{(v457/v459)}else{v444});
        let v463=(v440+(v461).ln());
        let v464=(v206+v461);
        let v467=(v431>v304);
        let v469=(v451&&(!v450));
        let v470=(v467&&v469);
        let v471=(v206+v454);
        let v475=(v469&&(!v467));
        let v476=(v431).exp();
        let v478=(if v475{(v315+v476)}else{(if v470{(v50/v471)}else{(if v452{(v463/v464)}else{(if v432{(v446/v447)}else{v411})})})});
        let v479=(v50+v478);
        let v481=(v231+v319);
        let v482=(v231+(v478*v479));
        let v483=(v481).sqrt();
        let v484=(v482).sqrt();
        let v485=(v483+v484);
        let v486=(v485*v485);
        let v487=(self.scalar_static_f64[210]+v261);
        let v488=(1e-6+v487);
        let v490=(v206*(v488).sqrt());
        let v506=-0.5;
        let v554=(v206*v320);
        let v555=4.0;
        let v662=(v206*v484);
        let v665=(v206*v483);
        let v773=(v481*v483);
        let v774=(v482*v484);
        let v777=((self.scalar_static_f64[210]+(v25*v261))).sqrt();
        let v778=(v777+v777);
        let v783=(-(self.scalar_static_f64[105]*(self.scalar_static_f64[172]*(v50+(v253/v778)))));
        let v784=0.266666666;
        let v786=6.0;
        let v787=(v482*v786);
        let v790=(v484*v555);
        let v795=(v784*((((v9*v774)+(v483*v787))+(v481*v790))+(v206*v773)));
        let v797=((v795/v486)-v25);
        let v798=(v783*v797);
        let v800=(v481*v786);
        let v803=(v483*v555);
        let v808=(v784*((((v9*v773)+(v484*v800))+(v482*v803))+(v206*v774)));
        let v810=((v808/v486)-v25);
        let v811=(v783*v810);
        let v812=(v798+v811);
        let v813=(v253*v506);
        let v818=(v253*v812);
        let v819=(v253+v778);
        let v823=((-v812)-((self.scalar_static_f64[105]*((v211+(v490*v813))-v204))-(v818/v819)));
        let v826=(v50==v200);
        let v1030=(v191>v1);
        let v1035=(v50+(v191/self.scalar_static_f64[252]));
        let v1038=((self.scalar_static_f64[154]*(v1035).ln())).exp();
        let v1045=(v50+(v191/self.scalar_static_f64[254]));
        let v1048=((self.scalar_static_f64[156]*(v1045).ln())).exp();
        let v1055=(v50+(v191/self.scalar_static_f64[256]));
        let v1058=((self.scalar_static_f64[158]*(v1055).ln())).exp();
        let v1061=(!v1030);
        let v1078=((if v1061{(self.scalar_static_f64[292]*(v50-((v191*self.scalar_static_f64[157])/self.scalar_static_f64[256])))}else{(if v1030{(self.scalar_static_f64[292]*v1058)}else{v1})})+((if v1061{(self.scalar_static_f64[290]*(v50-((v191*self.scalar_static_f64[153])/self.scalar_static_f64[252])))}else{(if v1030{(self.scalar_static_f64[290]*v1038)}else{v1})})+(if v1061{(self.scalar_static_f64[291]*(v50-((v191*self.scalar_static_f64[155])/self.scalar_static_f64[254])))}else{(if v1030{(self.scalar_static_f64[291]*v1048)}else{v1})})));
        let v1079=(v191*v1078);
        let v1080=(v188>v1);
        let v1083=(v50+(v188/self.scalar_static_f64[252]));
        let v1086=((self.scalar_static_f64[154]*(v1083).ln())).exp();
        let v1091=(v50+(v188/self.scalar_static_f64[254]));
        let v1094=((self.scalar_static_f64[156]*(v1091).ln())).exp();
        let v1098=(v50+(v188/self.scalar_static_f64[256]));
        let v1101=((self.scalar_static_f64[158]*(v1098).ln())).exp();
        let v1104=(!v1080);
        let v1121=((if v1104{(self.scalar_static_f64[292]*(v50-((v188*self.scalar_static_f64[157])/self.scalar_static_f64[256])))}else{(if v1080{(self.scalar_static_f64[292]*v1101)}else{v1})})+((if v1104{(self.scalar_static_f64[293]*(v50-((v188*self.scalar_static_f64[153])/self.scalar_static_f64[252])))}else{(if v1080{(self.scalar_static_f64[293]*v1086)}else{v1})})+(if v1104{(self.scalar_static_f64[294]*(v50-((v188*self.scalar_static_f64[155])/self.scalar_static_f64[254])))}else{(if v1080{(self.scalar_static_f64[294]*v1094)}else{v1})})));
        let v1122=(v188*v1121);
        let v1131=(!v826);
        let v1163=(if v193{self.scalar_static_f64[20]}else{v1});
        let v1165=(if v193{v1}else{self.scalar_static_f64[20]});
        let v1166=(if v193{v1163}else{v1});
        let v1167=(if v193{(if v193{self.scalar_static_f64[160]}else{v1})}else{self.scalar_static_f64[160]});
        let v1168=(self.scalar_static_f64[20]*v204);
        let v1170=(v204*self.scalar_static_f64[160]);
        let v1172=(v206*v209);
        let v1177=(v25*(self.scalar_static_f64[20]+((v1168+v1168)/v1172)));
        let v1178=(v25*(self.scalar_static_f64[160]+((v1170+v1170)/v1172)));
        let v1179=(v212*v1163);
        let v1181=(v212*v1165);
        let v1183=(v212*self.scalar_static_f64[160]);
        let v1185=(v206*v215);
        let v1186=((v1179+v1179)/v1185);
        let v1187=((v1181+v1181)/v1185);
        let v1188=((v1183+v1183)/v1185);
        let v1195=(v206*v218);
        let v1196=((v25*(v1163+v1186))/v1195);
        let v1197=((v25*(v1165+v1187))/v1195);
        let v1198=((v25*(self.scalar_static_f64[160]+v1188))/v1195);
        let v1199=(v219*v1165);
        let v1201=(v219*v1166);
        let v1203=(v219*v1167);
        let v1205=(v206*v222);
        let v1206=((v1199+v1199)/v1205);
        let v1207=((v1201+v1201)/v1205);
        let v1208=((v1203+v1203)/v1205);
        let v1215=(v206*v225);
        let v1216=((v25*(v1165+v1206))/v1215);
        let v1217=((v25*(v1166+v1207))/v1215);
        let v1218=((v25*(v1167+v1208))/v1215);
        let v1219=(v206*v235);
        let v1226=(v206*v243);
        let v1235=(-(self.scalar_static_f64[90]*(v1196+v1216)));
        let v1236=(-(self.scalar_static_f64[90]*(v1197+v1217)));
        let v1238=(self.scalar_static_f64[88]*((v1177-(self.scalar_static_f64[67]*(v1177/v1219)))/v1226));
        let v1240=((-(self.scalar_static_f64[90]*(v1198+v1218)))+(self.scalar_static_f64[88]*((v1178-(self.scalar_static_f64[67]*(v1178/v1219)))/v1226)));
        let v1241=(v248*v1235);
        let v1243=(v248*v1238);
        let v1245=(v248*v1236);
        let v1247=(v248*v1240);
        let v1249=(v206*v251);
        let v1250=((v1241+v1241)/v1249);
        let v1251=((v1243+v1243)/v1249);
        let v1252=((v1245+v1245)/v1249);
        let v1253=((v1247+v1247)/v1249);
        let v1258=(v25*(v1235+v1250));
        let v1259=(v25*(v1238+v1251));
        let v1260=(v25*(v1236+v1252));
        let v1261=(v25*(v1240+v1253));
        let v1280=(v206*v257);
        let v1281=(((v254*v1258)+(v253*(v231*v1258)))/v1280);
        let v1282=((v1177+((v254*v1259)+(v253*(v231*v1259))))/v1280);
        let v1283=(((v254*v1260)+(v253*(v231*v1260)))/v1280);
        let v1284=((v1178+((v254*v1261)+(v253*(v231*v1261))))/v1280);
        let v1305=(-((v259*v1258)+(v253*(v1281-(v25*v1258)))));
        let v1306=(v1177-((v259*v1259)+(v253*(v1282-(v25*v1259)))));
        let v1307=(-((v259*v1260)+(v253*(v1283-(v25*v1260)))));
        let v1308=(v1178-((v259*v1261)+(v253*(v1284-(v25*v1261)))));
        let v1312=(self.scalar_static_f64[174]*(v1305-v1163));
        let v1313=(self.scalar_static_f64[174]*v1306);
        let v1314=(self.scalar_static_f64[174]*(v1307-v1165));
        let v1315=(self.scalar_static_f64[174]*(v1308-self.scalar_static_f64[160]));
        let v1326=(v271*v271);
        let v1337=(if v265{((-(v206*(v1312-(v1312/v269))))/v1326)}else{v1});
        let v1338=(if v265{((-(v206*(v1313-(v1313/v269))))/v1326)}else{v1});
        let v1339=(if v265{((-(v206*(v1314-(v1314/v269))))/v1326)}else{v1});
        let v1340=(if v265{((-(v206*(v1315-(v1315/v269))))/v1326)}else{v1});
        let v1352=(v277*v277);
        let v1366=(if v265{(((v277*v1337)-(v274*(v1312+(v1337/v273))))/v1352)}else{v1});
        let v1367=(if v265{(((v277*v1338)-(v274*(v1313+(v1338/v273))))/v1352)}else{v1});
        let v1368=(if v265{(((v277*v1339)-(v274*(v1314+(v1339/v273))))/v1352)}else{v1});
        let v1369=(if v265{(((v277*v1340)-(v274*(v1315+(v1340/v273))))/v1352)}else{v1});
        let v1381=(v282*v282);
        let v1400=(-v1313);
        let v1403=(v291*(-v1312));
        let v1404=(v291*v1400);
        let v1405=(v291*(-v1314));
        let v1406=(v291*(-v1315));
        let v1407=(if v288{v1403}else{v1337});
        let v1408=(if v288{v1404}else{v1338});
        let v1409=(if v288{v1405}else{v1339});
        let v1410=(if v288{v1406}else{v1340});
        let v1422=(v296*v296);
        let v1436=(if v288{(((v296*v1407)-(v294*(v1312+(v1407/v293))))/v1422)}else{v1366});
        let v1437=(if v288{(((v296*v1408)-(v294*(v1313+(v1408/v293))))/v1422)}else{v1367});
        let v1438=(if v288{(((v296*v1409)-(v294*(v1314+(v1409/v293))))/v1422)}else{v1368});
        let v1439=(if v288{(((v296*v1410)-(v294*(v1315+(v1410/v293))))/v1422)}else{v1369});
        let v1451=(v301*v301);
        let v1470=(v309*v309);
        let v1486=(if v313{(v314*v1312)}else{(if v308{((-v1403)/v1470)}else{(if v288{(((v301*(v1312+(v1436/v298)))-(v300*v1436))/v1451)}else{(if v265{(((v282*(v1312+(v1366/v279)))-(v281*v1366))/v1381)}else{v1})})})});
        let v1487=(if v313{(v314*v1313)}else{(if v308{((-v1404)/v1470)}else{(if v288{(((v301*(v1313+(v1437/v298)))-(v300*v1437))/v1451)}else{(if v265{(((v282*(v1313+(v1367/v279)))-(v281*v1367))/v1381)}else{v1})})})});
        let v1488=(if v313{(v314*v1314)}else{(if v308{((-v1405)/v1470)}else{(if v288{(((v301*(v1314+(v1438/v298)))-(v300*v1438))/v1451)}else{(if v265{(((v282*(v1314+(v1368/v279)))-(v281*v1368))/v1381)}else{v1})})})});
        let v1489=(if v313{(v314*v1315)}else{(if v308{((-v1406)/v1470)}else{(if v288{(((v301*(v1315+(v1439/v298)))-(v300*v1439))/v1451)}else{(if v265{(((v282*(v1315+(v1369/v279)))-(v281*v1369))/v1381)}else{v1})})})});
        let v1492=((v318*v1486)+(v317*v1486));
        let v1495=((v318*v1487)+(v317*v1487));
        let v1498=((v318*v1488)+(v317*v1488));
        let v1501=((v318*v1489)+(v317*v1489));
        let v1502=(v1492/v554);
        let v1503=(v1495/v554);
        let v1504=(v1498/v554);
        let v1505=(v1501/v554);
        let v1510=(v206*v324);
        let v1511=((self.scalar_static_f64[234]*v1502)/v1510);
        let v1512=((self.scalar_static_f64[234]*v1503)/v1510);
        let v1513=((self.scalar_static_f64[234]*v1504)/v1510);
        let v1514=((self.scalar_static_f64[234]*v1505)/v1510);
        let v1515=(self.scalar_static_f64[216]*v1511);
        let v1516=(self.scalar_static_f64[216]*v1512);
        let v1517=(self.scalar_static_f64[216]*v1513);
        let v1518=(self.scalar_static_f64[216]*v1514);
        let v1519=(v1165-v1163);
        let v1520=(v1166-v1165);
        let v1521=(v1167-self.scalar_static_f64[160]);
        let v1522=(v25*v1519);
        let v1523=(v25*v1520);
        let v1524=(v25*v1521);
        let v1537=(self.scalar_static_f64[179]*(self.scalar_static_f64[5]*(v1502-(self.scalar_static_f64[174]*v1515))));
        let v1538=(self.scalar_static_f64[179]*(self.scalar_static_f64[5]*(v1503-(self.scalar_static_f64[174]*v1516))));
        let v1539=(self.scalar_static_f64[179]*(self.scalar_static_f64[5]*(v1504-(self.scalar_static_f64[174]*v1517))));
        let v1540=(self.scalar_static_f64[179]*(self.scalar_static_f64[5]*(v1505-(self.scalar_static_f64[174]*v1518))));
        let v1599=(v206*v349);
        let v1600=((self.scalar_static_f64[234]*(v1502-(v343*(v1492/v319))))/v1599);
        let v1601=((self.scalar_static_f64[234]*(v1503-(v343*(v1495/v319))))/v1599);
        let v1602=((self.scalar_static_f64[234]*(v1504-(v343*(v1498/v319))))/v1599);
        let v1603=((self.scalar_static_f64[234]*(v1505-(v343*(v1501/v319))))/v1599);
        let v1604=(self.scalar_static_f64[216]*v1600);
        let v1605=(self.scalar_static_f64[216]*v1601);
        let v1606=(self.scalar_static_f64[216]*v1602);
        let v1607=(self.scalar_static_f64[216]*v1603);
        let v1608=(v1522-v1604);
        let v1609=(-v1605);
        let v1610=(v1523-v1606);
        let v1611=(v1524-v1607);
        let v1612=(v352*v1604);
        let v1614=(v352*v1605);
        let v1616=(v352*v1606);
        let v1618=(v352*v1607);
        let v1624=(v206*v356);
        let v1625=((v1537+(v1612+v1612))/v1624);
        let v1626=((v1538+(v1614+v1614))/v1624);
        let v1627=((v1539+(v1616+v1616))/v1624);
        let v1628=((v1540+(v1618+v1618))/v1624);
        let v1629=(v353*v1608);
        let v1631=(v353*v1609);
        let v1633=(v353*v1610);
        let v1635=(v353*v1611);
        let v1641=(v206*v359);
        let v1642=((v1537+(v1629+v1629))/v1641);
        let v1643=((v1538+(v1631+v1631))/v1641);
        let v1644=((v1539+(v1633+v1633))/v1641);
        let v1645=((v1540+(v1635+v1635))/v1641);
        let v1660=(self.scalar_static_f64[174]*(v1642+(((v1305-v1522)-v1163)-v1625)));
        let v1661=(self.scalar_static_f64[174]*(v1643+(v1306-v1626)));
        let v1662=(self.scalar_static_f64[174]*(v1644+(((v1307-v1523)-v1165)-v1627)));
        let v1663=(self.scalar_static_f64[174]*(v1645+(((v1308-v1524)-self.scalar_static_f64[160])-v1628)));
        let v1674=(v369*v369);
        let v1685=(if v365{((-(v206*(v1660-(v1660/v367))))/v1674)}else{v1407});
        let v1686=(if v365{((-(v206*(v1661-(v1661/v367))))/v1674)}else{v1408});
        let v1687=(if v365{((-(v206*(v1662-(v1662/v367))))/v1674)}else{v1409});
        let v1688=(if v365{((-(v206*(v1663-(v1663/v367))))/v1674)}else{v1410});
        let v1700=(v375*v375);
        let v1714=(if v365{(((v375*v1685)-(v372*(v1660+(v1685/v371))))/v1700)}else{v1436});
        let v1715=(if v365{(((v375*v1686)-(v372*(v1661+(v1686/v371))))/v1700)}else{v1437});
        let v1716=(if v365{(((v375*v1687)-(v372*(v1662+(v1687/v371))))/v1700)}else{v1438});
        let v1717=(if v365{(((v375*v1688)-(v372*(v1663+(v1688/v371))))/v1700)}else{v1439});
        let v1729=(v380*v380);
        let v1751=(v387*(-v1660));
        let v1752=(v387*(-v1661));
        let v1753=(v387*(-v1662));
        let v1754=(v387*(-v1663));
        let v1755=(if v385{v1751}else{v1685});
        let v1756=(if v385{v1752}else{v1686});
        let v1757=(if v385{v1753}else{v1687});
        let v1758=(if v385{v1754}else{v1688});
        let v1770=(v392*v392);
        let v1784=(if v385{(((v392*v1755)-(v390*(v1660+(v1755/v389))))/v1770)}else{v1714});
        let v1785=(if v385{(((v392*v1756)-(v390*(v1661+(v1756/v389))))/v1770)}else{v1715});
        let v1786=(if v385{(((v392*v1757)-(v390*(v1662+(v1757/v389))))/v1770)}else{v1716});
        let v1787=(if v385{(((v392*v1758)-(v390*(v1663+(v1758/v389))))/v1770)}else{v1717});
        let v1799=(v397*v397);
        let v1818=(v404*v404);
        let v1834=(if v408{(v409*v1660)}else{(if v403{((-v1751)/v1818)}else{(if v385{(((v397*(v1660+(v1784/v394)))-(v396*v1784))/v1799)}else{(if v365{(((v380*(v1660+(v1714/v377)))-(v379*v1714))/v1729)}else{v1486})})})});
        let v1835=(if v408{(v409*v1661)}else{(if v403{((-v1752)/v1818)}else{(if v385{(((v397*(v1661+(v1785/v394)))-(v396*v1785))/v1799)}else{(if v365{(((v380*(v1661+(v1715/v377)))-(v379*v1715))/v1729)}else{v1487})})})});
        let v1836=(if v408{(v409*v1662)}else{(if v403{((-v1753)/v1818)}else{(if v385{(((v397*(v1662+(v1786/v394)))-(v396*v1786))/v1799)}else{(if v365{(((v380*(v1662+(v1716/v377)))-(v379*v1716))/v1729)}else{v1488})})})});
        let v1837=(if v408{(v409*v1663)}else{(if v403{((-v1754)/v1818)}else{(if v385{(((v397*(v1663+(v1787/v394)))-(v396*v1787))/v1799)}else{(if v365{(((v380*(v1663+(v1717/v377)))-(v379*v1717))/v1729)}else{v1489})})})});
        let v1905=(self.scalar_static_f64[174]*(v1305-v1165));
        let v1906=(self.scalar_static_f64[174]*(v1307-v1166));
        let v1907=(self.scalar_static_f64[174]*(v1308-v1167));
        let v1918=(v436*v436);
        let v1929=(if v432{((-(v206*(v1905-(v1905/v434))))/v1918)}else{v1755});
        let v1930=(if v432{((-(v206*(v1313-(v1313/v434))))/v1918)}else{v1756});
        let v1931=(if v432{((-(v206*(v1906-(v1906/v434))))/v1918)}else{v1757});
        let v1932=(if v432{((-(v206*(v1907-(v1907/v434))))/v1918)}else{v1758});
        let v1944=(v442*v442);
        let v1958=(if v432{(((v442*v1929)-(v439*(v1905+(v1929/v438))))/v1944)}else{v1784});
        let v1959=(if v432{(((v442*v1930)-(v439*(v1313+(v1930/v438))))/v1944)}else{v1785});
        let v1960=(if v432{(((v442*v1931)-(v439*(v1906+(v1931/v438))))/v1944)}else{v1786});
        let v1961=(if v432{(((v442*v1932)-(v439*(v1907+(v1932/v438))))/v1944)}else{v1787});
        let v1973=(v447*v447);
        let v1994=(v454*(-v1905));
        let v1995=(v454*v1400);
        let v1996=(v454*(-v1906));
        let v1997=(v454*(-v1907));
        let v1998=(if v452{v1994}else{v1929});
        let v1999=(if v452{v1995}else{v1930});
        let v2000=(if v452{v1996}else{v1931});
        let v2001=(if v452{v1997}else{v1932});
        let v2013=(v459*v459);
        let v2027=(if v452{(((v459*v1998)-(v457*(v1905+(v1998/v456))))/v2013)}else{v1958});
        let v2028=(if v452{(((v459*v1999)-(v457*(v1313+(v1999/v456))))/v2013)}else{v1959});
        let v2029=(if v452{(((v459*v2000)-(v457*(v1906+(v2000/v456))))/v2013)}else{v1960});
        let v2030=(if v452{(((v459*v2001)-(v457*(v1907+(v2001/v456))))/v2013)}else{v1961});
        let v2042=(v464*v464);
        let v2061=(v471*v471);
        let v2077=(if v475{(v476*v1905)}else{(if v470{((-v1994)/v2061)}else{(if v452{(((v464*(v1905+(v2027/v461)))-(v463*v2027))/v2042)}else{(if v432{(((v447*(v1905+(v1958/v444)))-(v446*v1958))/v1973)}else{v1834})})})});
        let v2078=(if v475{(v476*v1313)}else{(if v470{((-v1995)/v2061)}else{(if v452{(((v464*(v1313+(v2028/v461)))-(v463*v2028))/v2042)}else{(if v432{(((v447*(v1313+(v1959/v444)))-(v446*v1959))/v1973)}else{v1835})})})});
        let v2079=(if v475{(v476*v1906)}else{(if v470{((-v1996)/v2061)}else{(if v452{(((v464*(v1906+(v2029/v461)))-(v463*v2029))/v2042)}else{(if v432{(((v447*(v1906+(v1960/v444)))-(v446*v1960))/v1973)}else{v1836})})})});
        let v2080=(if v475{(v476*v1907)}else{(if v470{((-v1997)/v2061)}else{(if v452{(((v464*(v1907+(v2030/v461)))-(v463*v2030))/v2042)}else{(if v432{(((v447*(v1907+(v1961/v444)))-(v446*v1961))/v1973)}else{v1837})})})});
        let v2083=((v479*v2077)+(v478*v2077));
        let v2086=((v479*v2078)+(v478*v2078));
        let v2089=((v479*v2079)+(v478*v2079));
        let v2092=((v479*v2080)+(v478*v2080));
        let v2093=(v1492/v665);
        let v2094=(v1495/v665);
        let v2095=(v1498/v665);
        let v2096=(v1501/v665);
        let v2097=(v2083/v662);
        let v2098=(v2086/v662);
        let v2099=(v2089/v662);
        let v2100=(v2092/v662);
        let v2101=(v2093+v2097);
        let v2102=(v2094+v2098);
        let v2103=(v2095+v2099);
        let v2104=(v2096+v2100);
        let v2105=(v485*v2101);
        let v2106=(v2105+v2105);
        let v2107=(v485*v2102);
        let v2108=(v2107+v2107);
        let v2109=(v485*v2103);
        let v2110=(v2109+v2109);
        let v2111=(v485*v2104);
        let v2112=(v2111+v2111);
        let v2117=(v206*(v1305/v490));
        let v2118=(v206*(v1306/v490));
        let v2119=(v206*(v1307/v490));
        let v2120=(v206*(v1308/v490));
        let v3173=(v486*v486);
        let v3875=((v483*v1492)+(v481*v2093));
        let v3878=((v483*v1495)+(v481*v2094));
        let v3881=((v483*v1498)+(v481*v2095));
        let v3884=((v483*v1501)+(v481*v2096));
        let v3887=((v484*v2083)+(v482*v2097));
        let v3890=((v484*v2086)+(v482*v2098));
        let v3893=((v484*v2089)+(v482*v2099));
        let v3896=((v484*v2092)+(v482*v2100));
        let v3901=(v206*v777);
        let v3902=((v25*v1305)/v3901);
        let v3903=((v25*v1306)/v3901);
        let v3904=((v25*v1307)/v3901);
        let v3905=((v25*v1308)/v3901);
        let v3906=(v3902+v3902);
        let v3907=(v3903+v3903);
        let v3908=(v3904+v3904);
        let v3909=(v3905+v3905);
        let v3913=(v778*v778);
        let v3935=(-(self.scalar_static_f64[105]*(self.scalar_static_f64[172]*(((v778*v1258)-(v253*v3906))/v3913))));
        let v3936=(-(self.scalar_static_f64[105]*(self.scalar_static_f64[172]*(((v778*v1259)-(v253*v3907))/v3913))));
        let v3937=(-(self.scalar_static_f64[105]*(self.scalar_static_f64[172]*(((v778*v1260)-(v253*v3908))/v3913))));
        let v3938=(-(self.scalar_static_f64[105]*(self.scalar_static_f64[172]*(((v778*v1261)-(v253*v3909))/v3913))));
        let v4013=((v797*v3935)+(v783*(((v486*(v784*((((v9*v3887)+((v787*v2093)+(v483*(v786*v2083))))+((v790*v1492)+(v481*(v555*v2097))))+(v206*v3875))))-(v795*v2106))/v3173)));
        let v4016=((v797*v3936)+(v783*(((v486*(v784*((((v9*v3890)+((v787*v2094)+(v483*(v786*v2086))))+((v790*v1495)+(v481*(v555*v2098))))+(v206*v3878))))-(v795*v2108))/v3173)));
        let v4019=((v797*v3937)+(v783*(((v486*(v784*((((v9*v3893)+((v787*v2095)+(v483*(v786*v2089))))+((v790*v1498)+(v481*(v555*v2099))))+(v206*v3881))))-(v795*v2110))/v3173)));
        let v4022=((v797*v3938)+(v783*(((v486*(v784*((((v9*v3896)+((v787*v2096)+(v483*(v786*v2092))))+((v790*v1501)+(v481*(v555*v2100))))+(v206*v3884))))-(v795*v2112))/v3173)));
        let v4097=((v810*v3935)+(v783*(((v486*(v784*((((v9*v3875)+((v800*v2097)+(v484*(v786*v1492))))+((v803*v2083)+(v482*(v555*v2093))))+(v206*v3887))))-(v808*v2106))/v3173)));
        let v4100=((v810*v3936)+(v783*(((v486*(v784*((((v9*v3878)+((v800*v2098)+(v484*(v786*v1495))))+((v803*v2086)+(v482*(v555*v2094))))+(v206*v3890))))-(v808*v2108))/v3173)));
        let v4103=((v810*v3937)+(v783*(((v486*(v784*((((v9*v3881)+((v800*v2099)+(v484*(v786*v1498))))+((v803*v2089)+(v482*(v555*v2095))))+(v206*v3893))))-(v808*v2110))/v3173)));
        let v4106=((v810*v3938)+(v783*(((v486*(v784*((((v9*v3884)+((v800*v2100)+(v484*(v786*v1501))))+((v803*v2092)+(v482*(v555*v2096))))+(v206*v3896))))-(v808*v2112))/v3173)));
        let v4107=(v4013+v4097);
        let v4108=(v4016+v4100);
        let v4109=(v4019+v4103);
        let v4110=(v4022+v4106);
        let v4154=(v819*v819);
        let v4176=((-v4107)-((self.scalar_static_f64[105]*((v813*v2117)+(v490*(v506*v1258))))-(((v819*((v812*v1258)+(v253*v4107)))-(v818*(v1258+v3906)))/v4154)));
        let v4177=((-v4108)-((self.scalar_static_f64[105]*((v1177+((v813*v2118)+(v490*(v506*v1259))))-self.scalar_static_f64[20]))-(((v819*((v812*v1259)+(v253*v4108)))-(v818*(v1259+v3907)))/v4154)));
        let v4178=((-v4109)-((self.scalar_static_f64[105]*((v813*v2119)+(v490*(v506*v1260))))-(((v819*((v812*v1260)+(v253*v4109)))-(v818*(v1260+v3908)))/v4154)));
        let v4179=((-v4110)-((self.scalar_static_f64[105]*((v1178+((v813*v2120)+(v490*(v506*v1261))))-self.scalar_static_f64[160]))-(((v819*((v812*v1261)+(v253*v4110)))-(v818*(v1261+v3909)))/v4154)));
        let v4393=((self.scalar_static_f64[20]*v1078)+(v191*((if v1061{self.scalar_static_f64[335]}else{(if v1030{(self.scalar_static_f64[292]*(v1058*(self.scalar_static_f64[158]*(self.scalar_static_f64[317]/v1055))))}else{v1})})+((if v1061{self.scalar_static_f64[323]}else{(if v1030{(self.scalar_static_f64[290]*(v1038*(self.scalar_static_f64[154]*(self.scalar_static_f64[313]/v1035))))}else{v1})})+(if v1061{self.scalar_static_f64[329]}else{(if v1030{(self.scalar_static_f64[291]*(v1048*(self.scalar_static_f64[156]*(self.scalar_static_f64[315]/v1045))))}else{v1})})))));
        let v4396=((v1078*self.scalar_static_f64[160])+(v191*((if v1061{self.scalar_static_f64[336]}else{(if v1030{(self.scalar_static_f64[292]*(v1058*(self.scalar_static_f64[158]*(self.scalar_static_f64[318]/v1055))))}else{v1})})+((if v1061{self.scalar_static_f64[324]}else{(if v1030{(self.scalar_static_f64[290]*(v1038*(self.scalar_static_f64[154]*(self.scalar_static_f64[314]/v1035))))}else{v1})})+(if v1061{self.scalar_static_f64[330]}else{(if v1030{(self.scalar_static_f64[291]*(v1048*(self.scalar_static_f64[156]*(self.scalar_static_f64[316]/v1045))))}else{v1})})))));
        let v4443=((self.scalar_static_f64[20]*v1121)+(v188*((if v1104{self.scalar_static_f64[335]}else{(if v1080{(self.scalar_static_f64[292]*(v1101*(self.scalar_static_f64[158]*(self.scalar_static_f64[317]/v1098))))}else{v1})})+((if v1104{self.scalar_static_f64[337]}else{(if v1080{(self.scalar_static_f64[293]*(v1086*(self.scalar_static_f64[154]*(self.scalar_static_f64[313]/v1083))))}else{v1})})+(if v1104{self.scalar_static_f64[339]}else{(if v1080{(self.scalar_static_f64[294]*(v1094*(self.scalar_static_f64[156]*(self.scalar_static_f64[315]/v1091))))}else{v1})})))));
        let v4446=((v1121*self.scalar_static_f64[160])+(v188*((if v1104{self.scalar_static_f64[336]}else{(if v1080{(self.scalar_static_f64[292]*(v1101*(self.scalar_static_f64[158]*(self.scalar_static_f64[318]/v1098))))}else{v1})})+((if v1104{self.scalar_static_f64[338]}else{(if v1080{(self.scalar_static_f64[293]*(v1086*(self.scalar_static_f64[154]*(self.scalar_static_f64[314]/v1083))))}else{v1})})+(if v1104{self.scalar_static_f64[340]}else{(if v1080{(self.scalar_static_f64[294]*(v1094*(self.scalar_static_f64[156]*(self.scalar_static_f64[316]/v1091))))}else{v1})})))));

        CommonStampValues {
            v1,
            v25,
            v50,
            v188,
            v191,
            v200,
            v206,
            v215,
            v218,
            v222,
            v225,
            v251,
            v253,
            v257,
            v261,
            v317,
            v319,
            v320,
            v324,
            v326,
            v327,
            v328,
            v334,
            v349,
            v352,
            v353,
            v356,
            v359,
            v411,
            v478,
            v481,
            v482,
            v483,
            v484,
            v485,
            v486,
            v487,
            v488,
            v490,
            v506,
            v555,
            v662,
            v665,
            v798,
            v811,
            v823,
            v826,
            v1079,
            v1122,
            v1131,
            v1186,
            v1187,
            v1188,
            v1196,
            v1197,
            v1198,
            v1206,
            v1207,
            v1208,
            v1216,
            v1217,
            v1218,
            v1250,
            v1251,
            v1252,
            v1253,
            v1258,
            v1259,
            v1260,
            v1261,
            v1281,
            v1282,
            v1283,
            v1284,
            v1305,
            v1306,
            v1307,
            v1308,
            v1486,
            v1487,
            v1488,
            v1489,
            v1492,
            v1495,
            v1498,
            v1501,
            v1502,
            v1503,
            v1504,
            v1505,
            v1511,
            v1512,
            v1513,
            v1514,
            v1515,
            v1516,
            v1517,
            v1518,
            v1519,
            v1520,
            v1521,
            v1522,
            v1523,
            v1524,
            v1537,
            v1538,
            v1539,
            v1540,
            v1600,
            v1601,
            v1602,
            v1603,
            v1604,
            v1605,
            v1606,
            v1607,
            v1608,
            v1609,
            v1610,
            v1611,
            v1625,
            v1626,
            v1627,
            v1628,
            v1642,
            v1643,
            v1644,
            v1645,
            v1834,
            v1835,
            v1836,
            v1837,
            v2077,
            v2078,
            v2079,
            v2080,
            v2083,
            v2086,
            v2089,
            v2092,
            v2093,
            v2094,
            v2095,
            v2096,
            v2097,
            v2098,
            v2099,
            v2100,
            v2101,
            v2102,
            v2103,
            v2104,
            v2106,
            v2108,
            v2110,
            v2112,
            v2117,
            v2118,
            v2119,
            v2120,
            v3173,
            v4013,
            v4016,
            v4019,
            v4022,
            v4097,
            v4100,
            v4103,
            v4106,
            v4176,
            v4177,
            v4178,
            v4179,
            v4393,
            v4396,
            v4443,
            v4446,
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
        let v337=((common.v334+(common.v326*common.v326))).sqrt();
        let v338=(common.v328-common.v326);
        let v341=((common.v334+(v338*v338))).sqrt();
        let v342=(v337-v341);
        let v412=(common.v50+common.v411);
        let v416=(common.v50+((common.v328-v342)/self.scalar_static_f64[213]));
        let v422=((self.scalar_static_f64[46]-(self.scalar_static_f64[6]*(v416).ln()))+(self.scalar_static_f64[212]*(common.v328+v342)));
        let v427=(((v422*v422)+self.scalar_static_f64[95])).sqrt();
        let v429=(common.v25*(v422+v427));
        let v491=(self.scalar_static_f64[67]/common.v490);
        let v492=(self.scalar_static_f64[67]+common.v490);
        let v493=(self.scalar_static_f64[67]/v492);
        let v494=(common.v50+v491);
        let v496=(self.scalar_static_f64[172]*(-v494));
        let v497=0.66666666;
        let v498=1.33333332;
        let v502=(v498*(common.v481+(common.v482+(common.v483*common.v484))));
        let v504=((v502/common.v485)-common.v50);
        let v505=(v496*v504);
        let v514=((self.scalar_static_f64[178]+(common.v261*common.v261))).sqrt();
        let v515=(if self.scalar_static_bool[11]{v514}else{common.v1});
        let v520=((if self.scalar_static_bool[11]{(common.v25*(common.v261+v515))}else{common.v1})*self.scalar_static_f64[97]);
        let v522=(if self.scalar_static_bool[11]{(common.v50+v520)}else{common.v1});
        let v523=(v429*v522);
        let v527=(((common.v490*self.scalar_static_f64[96])-(v493*v505))+(self.scalar_static_f64[21]*v505));
        let v528=(v527>common.v1);
        let v530=(v528&&self.scalar_static_bool[12]);
        let v531=(self.scalar_static_f64[16]*v527);
        let v535=(self.scalar_static_bool[12]&&(!v528));
        let v537=(if v535{(common.v50-v531)}else{(if v530{(common.v50+v531)}else{common.v1})});
        let v542=(v429*v537);
        let v544=(if self.scalar_static_bool[12]{(self.scalar_static_f64[238]/v542)}else{(if self.scalar_static_bool[11]{(self.scalar_static_f64[231]/v523)}else{common.v1})});
        let v545=(self.scalar_static_f64[176]+common.v487);
        let v546=(v545).sqrt();
        let v547=(common.v206*v546);
        let v549=(common.v50+(self.scalar_static_f64[67]/v547));
        let v550=(common.v319-(common.v411*v412));
        let v551=(self.scalar_static_f64[178]*v549);
        let v552=(v544*v551);
        let v553=(v550*v552);
        let v556=(common.v251+common.v251);
        let v559=((common.v253/v556)*self.scalar_static_f64[98]);
        let v560=(common.v225*v559);
        let v561=(v560/common.v222);
        let v562=(common.v218*v559);
        let v563=(v562/common.v215);
        let v565=(-(common.v487/common.v257));
        let v566=(v561*v565);
        let v567=(v563*v565);
        let v568=(self.scalar_static_f64[174]*common.v317);
        let v569=(v566*v568);
        let v570=(v567-common.v50);
        let v571=(v568*v570);
        let v572=(common.v324*common.v555);
        let v573=(common.v320*v572);
        let v574=(self.scalar_static_f64[172]/v573);
        let v575=(v569*v574);
        let v576=(v571*v574);
        let v579=(common.v320+common.v320);
        let v580=(self.scalar_static_f64[172]/v579);
        let v583=(self.scalar_static_f64[240]*((v569*v580)-v575));
        let v586=(self.scalar_static_f64[240]*((v571*v580)-v576));
        let v587=(common.v50/v337);
        let v588=(common.v50/v341);
        let v590=(v583+(common.v326*v575));
        let v592=(common.v25-v575);
        let v594=(v583+(v338*v592));
        let v596=((v587*v590)-(v588*v594));
        let v598=(v586+(common.v326*v576));
        let v600=(common.v506-v576);
        let v602=(v586+(v338*v600));
        let v604=((v587*v598)-(v588*v602));
        let v607=(self.scalar_static_f64[172]*(common.v320-1.5));
        let v608=(common.v349*common.v555);
        let v609=(common.v319*v608);
        let v610=(v607/v609);
        let v611=(v569*v610);
        let v612=(v571*v610);
        let v613=(self.scalar_static_f64[174]*common.v411);
        let v614=(common.v50/common.v356);
        let v615=(common.v50/common.v359);
        let v618=(v583+(common.v352*v611));
        let v621=(common.v25-v611);
        let v623=(v583+(common.v353*v621));
        let v625=(((v566-common.v25)-(v614*v618))+(v615*v623));
        let v629=(v586+(common.v352*v612));
        let v632=(common.v506-v612);
        let v634=(v586+(common.v353*v632));
        let v636=(((v567-common.v25)-(v614*v629))+(v615*v634));
        let v639=((self.scalar_static_f64[213]+common.v328)-v342);
        let v640=(self.scalar_static_f64[6]/v639);
        let v641=(common.v25-v596);
        let v643=(common.v506-v604);
        let v645=(common.v50/v427);
        let v649=((-(v640*v641))+(self.scalar_static_f64[212]*(common.v25+v596)));
        let v654=((-(v640*v643))+(self.scalar_static_f64[212]*(common.v506+v604)));
        let v656=(self.scalar_static_f64[174]*common.v478);
        let v657=(v566-common.v50);
        let v658=(v656*v657);
        let v659=(v567*v656);
        let v660=(v496*v497);
        let v661=(v660/common.v486);
        let v663=(common.v483+common.v662);
        let v664=(v661*v663);
        let v666=(common.v484+common.v665);
        let v667=(v661*v666);
        let v668=(-v491);
        let v669=(v505*v668);
        let v671=(v491+(common.v206+v491));
        let v672=(common.v488*v671);
        let v673=(v669/v672);
        let v678=(((v566*v673)+(v569*v664))+(v658*v667));
        let v683=(((v567*v673)+(v571*v664))+(v659*v667));
        let v684=(common.v206*v494);
        let v685=(common.v488*v684);
        let v687=(v494-(v505/v685));
        let v688=(-v493);
        let v690=(v678+(v566*v687));
        let v693=(v683+(v567*v687));
        let v695=(v515*v522);
        let v697=(if self.scalar_static_bool[11]{(v520/v695)}else{v687});
        let v702=(-(v645*v649));
        let v705=(-(v645*v654));
        let v709=(if self.scalar_static_bool[12]{(self.scalar_static_f64[16]/v537)}else{v697});
        let v711=((v688*v690)+(self.scalar_static_f64[21]*v678));
        let v716=((v688*v693)+(self.scalar_static_f64[21]*v683));
        let v721=(v549*common.v555);
        let v722=(v546*v721);
        let v723=(v545*v722);
        let v724=(self.scalar_static_f64[99]/v723);
        let v727=((if self.scalar_static_bool[12]{(v702+(v709*v711))}else{(if self.scalar_static_bool[11]{(v702-(if self.scalar_static_bool[11]{(v566*v697)}else{common.v1}))}else{common.v1})})+(v566*v724));
        let v730=((v569+(v550*v727))-(v613*v625));
        let v732=(-v552);
        let v733=((if self.scalar_static_bool[12]{(v705+(v709*v716))}else{(if self.scalar_static_bool[11]{(v705-(if self.scalar_static_bool[11]{(v567*v697)}else{common.v1}))}else{common.v1})})+(v567*v724));
        let v736=((v571+(v550*v733))-(v613*v636));
        let v746=((common.v50+((v732*v736)*self.scalar_static_f64[104]))+((v552*v730)*self.scalar_static_f64[104]));
        let v747=(common.v50/v746);
        let v748=(v553*v747);
        let v750=(common.v327-(self.scalar_static_f64[13]*common.v326));
        let v753=((v750>common.v1)&&self.scalar_static_bool[26]);
        let v758=(if v753{((if v753{(common.v50/v750)}else{common.v1})*self.scalar_static_f64[241])}else{common.v1});
        let v759=-35.0;
        let v761=(v753&&(v758<v759));
        let v763=((if v761{v759}else{v758})).exp();
        let v764=(if v753{v763}else{common.v1});
        let v765=(self.scalar_static_f64[215]*v750);
        let v767=(if v753{(v764*v765)}else{common.v1});
        let v770=(!v753);
        let v824=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v798);
        let v825=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v811);
        let v920=(-common.v191);
        let v923=((self.scalar_static_f64[186]*v920)/self.scalar_static_f64[281]);
        let v924=-40.0;
        let v925=(v923<v924);
        let v930=((self.scalar_static_f64[186]*(v920+self.scalar_static_f64[147]))/self.scalar_static_f64[281]);
        let v931=70.0;
        let v932=(v930>v931);
        let v934=(!v932);
        let v937=((-v930)).exp();
        let v940=(if v934{(common.v50+(self.scalar_static_f64[148]*v937))}else{(if v932{common.v50}else{common.v1})});
        let v943=(self.scalar_static_f64[186]*common.v191);
        let v947=((v943/self.scalar_static_f64[283])*self.scalar_static_f64[150]);
        let v948=(common.v191+self.scalar_static_f64[150]);
        let v949=0.001;
        let v950=(v948>v949);
        let v951=(if v950{v948}else{v949});
        let v953=((v947/v951)).exp();
        let v959=((v943/self.scalar_static_f64[284])*self.scalar_static_f64[151]);
        let v960=(common.v191+self.scalar_static_f64[151]);
        let v961=(v960>v949);
        let v962=(if v961{v960}else{v949});
        let v964=((v959/v962)).exp();
        let v971=((v943/self.scalar_static_f64[285])*self.scalar_static_f64[152]);
        let v972=(common.v191+self.scalar_static_f64[152]);
        let v973=(v972>v949);
        let v974=(if v973{v972}else{v949});
        let v976=((v971/v974)).exp();
        let v984=(-common.v188);
        let v986=((self.scalar_static_f64[186]*v984)/self.scalar_static_f64[281]);
        let v987=(v986<v924);
        let v991=((self.scalar_static_f64[186]*(self.scalar_static_f64[147]+v984))/self.scalar_static_f64[281]);
        let v992=(v991>v931);
        let v994=(!v992);
        let v996=((-v991)).exp();
        let v999=(if v994{(common.v50+(self.scalar_static_f64[148]*v996))}else{(if v992{common.v50}else{common.v1})});
        let v1000=(self.scalar_static_f64[186]*common.v188);
        let v1002=(self.scalar_static_f64[150]*(v1000/self.scalar_static_f64[283]));
        let v1003=(common.v188+self.scalar_static_f64[150]);
        let v1004=(v1003>v949);
        let v1005=(if v1004{v1003}else{v949});
        let v1007=((v1002/v1005)).exp();
        let v1011=(self.scalar_static_f64[151]*(v1000/self.scalar_static_f64[284]));
        let v1012=(common.v188+self.scalar_static_f64[151]);
        let v1013=(v1012>v949);
        let v1014=(if v1013{v1012}else{v949});
        let v1016=((v1011/v1014)).exp();
        let v1021=(self.scalar_static_f64[152]*(v1000/self.scalar_static_f64[285]));
        let v1022=(common.v188+self.scalar_static_f64[152]);
        let v1023=(v1022>v949);
        let v1024=(if v1023{v1022}else{v949});
        let v1026=((v1021/v1024)).exp();
        let v1123=(self.scalar_static_f64[20]*common.v200);
        let v1125=(self.scalar_static_f64[20]*v824);
        let v1127=(self.scalar_static_f64[20]*v825);
        let v1129=(self.scalar_static_f64[20]*(if v770{common.v1}else{(if v753{(v748*v767)}else{common.v1})}));
        let v1135=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v823);
        let v1137=((if v925{v924}else{v923})).exp();
        let v1139=(self.scalar_static_f64[280]*(common.v50-v1137));
        let v1147=((if v987{v924}else{v986})).exp();
        let v1149=(self.scalar_static_f64[289]*(common.v50-v1147));
        let v1156=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v1079);
        let v1159=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v1122);
        let v1541=(common.v326*common.v1515);
        let v1543=(common.v326*common.v1516);
        let v1545=(common.v326*common.v1517);
        let v1547=(common.v326*common.v1518);
        let v1553=(common.v206*v337);
        let v1554=((common.v1537+(v1541+v1541))/v1553);
        let v1555=((common.v1538+(v1543+v1543))/v1553);
        let v1556=((common.v1539+(v1545+v1545))/v1553);
        let v1557=((common.v1540+(v1547+v1547))/v1553);
        let v1558=(common.v1522-common.v1515);
        let v1559=(-common.v1516);
        let v1560=(common.v1523-common.v1517);
        let v1561=(common.v1524-common.v1518);
        let v1562=(v338*v1558);
        let v1564=(v338*v1559);
        let v1566=(v338*v1560);
        let v1568=(v338*v1561);
        let v1574=(common.v206*v341);
        let v1575=((common.v1537+(v1562+v1562))/v1574);
        let v1576=((common.v1538+(v1564+v1564))/v1574);
        let v1577=((common.v1539+(v1566+v1566))/v1574);
        let v1578=((common.v1540+(v1568+v1568))/v1574);
        let v1579=(v1554-v1575);
        let v1580=(v1555-v1576);
        let v1581=(v1556-v1577);
        let v1582=(v1557-v1578);
        let v1850=(common.v1522-v1579);
        let v1851=(-v1580);
        let v1852=(common.v1523-v1581);
        let v1853=(common.v1524-v1582);
        let v1877=((-(self.scalar_static_f64[6]*((v1850/self.scalar_static_f64[213])/v416)))+(self.scalar_static_f64[212]*(common.v1522+v1579)));
        let v1878=((-(self.scalar_static_f64[6]*((v1851/self.scalar_static_f64[213])/v416)))+(self.scalar_static_f64[212]*v1580));
        let v1879=((-(self.scalar_static_f64[6]*((v1852/self.scalar_static_f64[213])/v416)))+(self.scalar_static_f64[212]*(common.v1523+v1581)));
        let v1880=((-(self.scalar_static_f64[6]*((v1853/self.scalar_static_f64[213])/v416)))+(self.scalar_static_f64[212]*(common.v1524+v1582)));
        let v1881=(v422*v1877);
        let v1883=(v422*v1878);
        let v1885=(v422*v1879);
        let v1887=(v422*v1880);
        let v1889=(common.v206*v427);
        let v1890=((v1881+v1881)/v1889);
        let v1891=((v1883+v1883)/v1889);
        let v1892=((v1885+v1885)/v1889);
        let v1893=((v1887+v1887)/v1889);
        let v1898=(common.v25*(v1877+v1890));
        let v1899=(common.v25*(v1878+v1891));
        let v1900=(common.v25*(v1879+v1892));
        let v1901=(common.v25*(v1880+v1893));
        let v2122=(-(self.scalar_static_f64[67]*common.v2117));
        let v2123=(common.v490*common.v490);
        let v2124=(v2122/v2123);
        let v2126=(-(self.scalar_static_f64[67]*common.v2118));
        let v2127=(v2126/v2123);
        let v2129=(-(self.scalar_static_f64[67]*common.v2119));
        let v2130=(v2129/v2123);
        let v2132=(-(self.scalar_static_f64[67]*common.v2120));
        let v2133=(v2132/v2123);
        let v2134=(v492*v492);
        let v2135=(v2122/v2134);
        let v2136=(v2126/v2134);
        let v2137=(v2129/v2134);
        let v2138=(v2132/v2134);
        let v2139=(-v2124);
        let v2140=(-v2127);
        let v2141=(-v2130);
        let v2142=(-v2133);
        let v2143=(self.scalar_static_f64[172]*v2139);
        let v2144=(self.scalar_static_f64[172]*v2140);
        let v2145=(self.scalar_static_f64[172]*v2141);
        let v2146=(self.scalar_static_f64[172]*v2142);
        let v2189=((v504*v2143)+(v496*(((common.v485*(v498*(common.v1492+(common.v2083+((common.v484*common.v2093)+(common.v483*common.v2097))))))-(v502*common.v2101))/common.v486)));
        let v2192=((v504*v2144)+(v496*(((common.v485*(v498*(common.v1495+(common.v2086+((common.v484*common.v2094)+(common.v483*common.v2098))))))-(v502*common.v2102))/common.v486)));
        let v2195=((v504*v2145)+(v496*(((common.v485*(v498*(common.v1498+(common.v2089+((common.v484*common.v2095)+(common.v483*common.v2099))))))-(v502*common.v2103))/common.v486)));
        let v2198=((v504*v2146)+(v496*(((common.v485*(v498*(common.v1501+(common.v2092+((common.v484*common.v2096)+(common.v483*common.v2100))))))-(v502*common.v2104))/common.v486)));
        let v2219=(common.v261*common.v1305);
        let v2221=(common.v261*common.v1306);
        let v2223=(common.v261*common.v1307);
        let v2225=(common.v261*common.v1308);
        let v2227=(common.v206*v514);
        let v2232=(if self.scalar_static_bool[11]{((v2219+v2219)/v2227)}else{common.v1});
        let v2233=(if self.scalar_static_bool[11]{((v2221+v2221)/v2227)}else{common.v1});
        let v2234=(if self.scalar_static_bool[11]{((v2223+v2223)/v2227)}else{common.v1});
        let v2235=(if self.scalar_static_bool[11]{((v2225+v2225)/v2227)}else{common.v1});
        let v2248=(self.scalar_static_f64[97]*(if self.scalar_static_bool[11]{(common.v25*(common.v1305+v2232))}else{common.v1}));
        let v2249=(self.scalar_static_f64[97]*(if self.scalar_static_bool[11]{(common.v25*(common.v1306+v2233))}else{common.v1}));
        let v2250=(self.scalar_static_f64[97]*(if self.scalar_static_bool[11]{(common.v25*(common.v1307+v2234))}else{common.v1}));
        let v2251=(self.scalar_static_f64[97]*(if self.scalar_static_bool[11]{(common.v25*(common.v1308+v2235))}else{common.v1}));
        let v2252=(if self.scalar_static_bool[11]{v2248}else{common.v1});
        let v2253=(if self.scalar_static_bool[11]{v2249}else{common.v1});
        let v2254=(if self.scalar_static_bool[11]{v2250}else{common.v1});
        let v2255=(if self.scalar_static_bool[11]{v2251}else{common.v1});
        let v2270=(v523*v523);
        let v2293=(self.scalar_static_f64[16]*(((self.scalar_static_f64[96]*common.v2117)-((v505*v2135)+(v493*v2189)))+(self.scalar_static_f64[21]*v2189)));
        let v2294=(self.scalar_static_f64[16]*(((self.scalar_static_f64[96]*common.v2118)-((v505*v2136)+(v493*v2192)))+(self.scalar_static_f64[21]*v2192)));
        let v2295=(self.scalar_static_f64[16]*(((self.scalar_static_f64[96]*common.v2119)-((v505*v2137)+(v493*v2195)))+(self.scalar_static_f64[21]*v2195)));
        let v2296=(self.scalar_static_f64[16]*(((self.scalar_static_f64[96]*common.v2120)-((v505*v2138)+(v493*v2198)))+(self.scalar_static_f64[21]*v2198)));
        let v2305=(if v535{(-v2293)}else{(if v530{v2293}else{common.v1})});
        let v2306=(if v535{(-v2294)}else{(if v530{v2294}else{common.v1})});
        let v2307=(if v535{(-v2295)}else{(if v530{v2295}else{common.v1})});
        let v2308=(if v535{(-v2296)}else{(if v530{v2296}else{common.v1})});
        let v2323=(v542*v542);
        let v2338=(common.v1305/v547);
        let v2339=(common.v1306/v547);
        let v2340=(common.v1307/v547);
        let v2341=(common.v1308/v547);
        let v2348=(v547*v547);
        let v2349=((-(self.scalar_static_f64[67]*(common.v206*v2338)))/v2348);
        let v2352=((-(self.scalar_static_f64[67]*(common.v206*v2339)))/v2348);
        let v2355=((-(self.scalar_static_f64[67]*(common.v206*v2340)))/v2348);
        let v2358=((-(self.scalar_static_f64[67]*(common.v206*v2341)))/v2348);
        let v2359=(common.v1492-((v412*common.v1834)+(common.v411*common.v1834)));
        let v2360=(common.v1495-((v412*common.v1835)+(common.v411*common.v1835)));
        let v2361=(common.v1498-((v412*common.v1836)+(common.v411*common.v1836)));
        let v2362=(common.v1501-((v412*common.v1837)+(common.v411*common.v1837)));
        let v2369=((v551*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[238]*((v537*v1898)+(v429*v2305))))/v2323)}else{(if self.scalar_static_bool[11]{((-(self.scalar_static_f64[231]*((v522*v1898)+(v429*v2252))))/v2270)}else{common.v1})}))+(v544*(self.scalar_static_f64[178]*v2349)));
        let v2372=((v551*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[238]*((v537*v1899)+(v429*v2306))))/v2323)}else{(if self.scalar_static_bool[11]{((-(self.scalar_static_f64[231]*((v522*v1899)+(v429*v2253))))/v2270)}else{common.v1})}))+(v544*(self.scalar_static_f64[178]*v2352)));
        let v2375=((v551*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[238]*((v537*v1900)+(v429*v2307))))/v2323)}else{(if self.scalar_static_bool[11]{((-(self.scalar_static_f64[231]*((v522*v1900)+(v429*v2254))))/v2270)}else{common.v1})}))+(v544*(self.scalar_static_f64[178]*v2355)));
        let v2378=((v551*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[238]*((v537*v1901)+(v429*v2308))))/v2323)}else{(if self.scalar_static_bool[11]{((-(self.scalar_static_f64[231]*((v522*v1901)+(v429*v2255))))/v2270)}else{common.v1})}))+(v544*(self.scalar_static_f64[178]*v2358)));
        let v2398=(v556*v556);
        let v2412=(self.scalar_static_f64[98]*(((v556*common.v1258)-(common.v253*(common.v1250+common.v1250)))/v2398));
        let v2413=(self.scalar_static_f64[98]*(((v556*common.v1259)-(common.v253*(common.v1251+common.v1251)))/v2398));
        let v2414=(self.scalar_static_f64[98]*(((v556*common.v1260)-(common.v253*(common.v1252+common.v1252)))/v2398));
        let v2415=(self.scalar_static_f64[98]*(((v556*common.v1261)-(common.v253*(common.v1253+common.v1253)))/v2398));
        let v2429=(common.v222*common.v222);
        let v2453=(common.v215*common.v215);
        let v2467=(common.v257*common.v257);
        let v2481=(-(((common.v257*common.v1305)-(common.v487*common.v1281))/v2467));
        let v2482=(-(((common.v257*common.v1306)-(common.v487*common.v1282))/v2467));
        let v2483=(-(((common.v257*common.v1307)-(common.v487*common.v1283))/v2467));
        let v2484=(-(((common.v257*common.v1308)-(common.v487*common.v1284))/v2467));
        let v2487=((v565*(((common.v222*((v559*common.v1216)+(common.v225*v2412)))-(v560*common.v1206))/v2429))+(v561*v2481));
        let v2490=((v565*((common.v225*v2413)/common.v222))+(v561*v2482));
        let v2493=((v565*(((common.v222*((v559*common.v1217)+(common.v225*v2414)))-(v560*common.v1207))/v2429))+(v561*v2483));
        let v2496=((v565*(((common.v222*((v559*common.v1218)+(common.v225*v2415)))-(v560*common.v1208))/v2429))+(v561*v2484));
        let v2499=((v565*(((common.v215*((v559*common.v1196)+(common.v218*v2412)))-(v562*common.v1186))/v2453))+(v563*v2481));
        let v2502=((v565*((common.v218*v2413)/common.v215))+(v563*v2482));
        let v2505=((v565*(((common.v215*((v559*common.v1197)+(common.v218*v2414)))-(v562*common.v1187))/v2453))+(v563*v2483));
        let v2508=((v565*(((common.v215*((v559*common.v1198)+(common.v218*v2415)))-(v562*common.v1188))/v2453))+(v563*v2484));
        let v2509=(self.scalar_static_f64[174]*common.v1486);
        let v2510=(self.scalar_static_f64[174]*common.v1487);
        let v2511=(self.scalar_static_f64[174]*common.v1488);
        let v2512=(self.scalar_static_f64[174]*common.v1489);
        let v2515=((v568*v2487)+(v566*v2509));
        let v2518=((v568*v2490)+(v566*v2510));
        let v2521=((v568*v2493)+(v566*v2511));
        let v2524=((v568*v2496)+(v566*v2512));
        let v2527=((v570*v2509)+(v568*v2499));
        let v2530=((v570*v2510)+(v568*v2502));
        let v2533=((v570*v2511)+(v568*v2505));
        let v2536=((v570*v2512)+(v568*v2508));
        let v2555=(v573*v573);
        let v2556=((-(self.scalar_static_f64[172]*((v572*common.v1502)+(common.v320*(common.v555*common.v1511)))))/v2555);
        let v2559=((-(self.scalar_static_f64[172]*((v572*common.v1503)+(common.v320*(common.v555*common.v1512)))))/v2555);
        let v2562=((-(self.scalar_static_f64[172]*((v572*common.v1504)+(common.v320*(common.v555*common.v1513)))))/v2555);
        let v2565=((-(self.scalar_static_f64[172]*((v572*common.v1505)+(common.v320*(common.v555*common.v1514)))))/v2555);
        let v2568=((v574*v2515)+(v569*v2556));
        let v2571=((v574*v2518)+(v569*v2559));
        let v2574=((v574*v2521)+(v569*v2562));
        let v2577=((v574*v2524)+(v569*v2565));
        let v2580=((v574*v2527)+(v571*v2556));
        let v2583=((v574*v2530)+(v571*v2559));
        let v2586=((v574*v2533)+(v571*v2562));
        let v2589=((v574*v2536)+(v571*v2565));
        let v2596=(v579*v579);
        let v2597=((-(self.scalar_static_f64[172]*(common.v1502+common.v1502)))/v2596);
        let v2600=((-(self.scalar_static_f64[172]*(common.v1503+common.v1503)))/v2596);
        let v2603=((-(self.scalar_static_f64[172]*(common.v1504+common.v1504)))/v2596);
        let v2606=((-(self.scalar_static_f64[172]*(common.v1505+common.v1505)))/v2596);
        let v2623=(self.scalar_static_f64[240]*(((v580*v2515)+(v569*v2597))-v2568));
        let v2624=(self.scalar_static_f64[240]*(((v580*v2518)+(v569*v2600))-v2571));
        let v2625=(self.scalar_static_f64[240]*(((v580*v2521)+(v569*v2603))-v2574));
        let v2626=(self.scalar_static_f64[240]*(((v580*v2524)+(v569*v2606))-v2577));
        let v2643=(self.scalar_static_f64[240]*(((v580*v2527)+(v571*v2597))-v2580));
        let v2644=(self.scalar_static_f64[240]*(((v580*v2530)+(v571*v2600))-v2583));
        let v2645=(self.scalar_static_f64[240]*(((v580*v2533)+(v571*v2603))-v2586));
        let v2646=(self.scalar_static_f64[240]*(((v580*v2536)+(v571*v2606))-v2589));
        let v2648=(v337*v337);
        let v2649=((-v1554)/v2648);
        let v2651=((-v1555)/v2648);
        let v2653=((-v1556)/v2648);
        let v2655=((-v1557)/v2648);
        let v2657=(v341*v341);
        let v2658=((-v1575)/v2657);
        let v2660=((-v1576)/v2657);
        let v2662=((-v1577)/v2657);
        let v2664=((-v1578)/v2657);
        let v2725=(((v590*v2649)+(v587*(v2623+((v575*common.v1515)+(common.v326*v2568)))))-((v594*v2658)+(v588*(v2623+((v592*v1558)+(v338*(-v2568)))))));
        let v2726=(((v590*v2651)+(v587*(v2624+((v575*common.v1516)+(common.v326*v2571)))))-((v594*v2660)+(v588*(v2624+((v592*v1559)+(v338*(-v2571)))))));
        let v2727=(((v590*v2653)+(v587*(v2625+((v575*common.v1517)+(common.v326*v2574)))))-((v594*v2662)+(v588*(v2625+((v592*v1560)+(v338*(-v2574)))))));
        let v2728=(((v590*v2655)+(v587*(v2626+((v575*common.v1518)+(common.v326*v2577)))))-((v594*v2664)+(v588*(v2626+((v592*v1561)+(v338*(-v2577)))))));
        let v2789=(((v598*v2649)+(v587*(v2643+((v576*common.v1515)+(common.v326*v2580)))))-((v602*v2658)+(v588*(v2643+((v600*v1558)+(v338*(-v2580)))))));
        let v2790=(((v598*v2651)+(v587*(v2644+((v576*common.v1516)+(common.v326*v2583)))))-((v602*v2660)+(v588*(v2644+((v600*v1559)+(v338*(-v2583)))))));
        let v2791=(((v598*v2653)+(v587*(v2645+((v576*common.v1517)+(common.v326*v2586)))))-((v602*v2662)+(v588*(v2645+((v600*v1560)+(v338*(-v2586)))))));
        let v2792=(((v598*v2655)+(v587*(v2646+((v576*common.v1518)+(common.v326*v2589)))))-((v602*v2664)+(v588*(v2646+((v600*v1561)+(v338*(-v2589)))))));
        let v2816=(v609*v609);
        let v2817=(((v609*(self.scalar_static_f64[172]*common.v1502))-(v607*((v608*common.v1492)+(common.v319*(common.v555*common.v1600)))))/v2816);
        let v2821=(((v609*(self.scalar_static_f64[172]*common.v1503))-(v607*((v608*common.v1495)+(common.v319*(common.v555*common.v1601)))))/v2816);
        let v2825=(((v609*(self.scalar_static_f64[172]*common.v1504))-(v607*((v608*common.v1498)+(common.v319*(common.v555*common.v1602)))))/v2816);
        let v2829=(((v609*(self.scalar_static_f64[172]*common.v1505))-(v607*((v608*common.v1501)+(common.v319*(common.v555*common.v1603)))))/v2816);
        let v2832=((v610*v2515)+(v569*v2817));
        let v2835=((v610*v2518)+(v569*v2821));
        let v2838=((v610*v2521)+(v569*v2825));
        let v2841=((v610*v2524)+(v569*v2829));
        let v2844=((v610*v2527)+(v571*v2817));
        let v2847=((v610*v2530)+(v571*v2821));
        let v2850=((v610*v2533)+(v571*v2825));
        let v2853=((v610*v2536)+(v571*v2829));
        let v2854=(self.scalar_static_f64[174]*common.v1834);
        let v2855=(self.scalar_static_f64[174]*common.v1835);
        let v2856=(self.scalar_static_f64[174]*common.v1836);
        let v2857=(self.scalar_static_f64[174]*common.v1837);
        let v2859=(common.v356*common.v356);
        let v2860=((-common.v1625)/v2859);
        let v2862=((-common.v1626)/v2859);
        let v2864=((-common.v1627)/v2859);
        let v2866=((-common.v1628)/v2859);
        let v2868=(common.v359*common.v359);
        let v2869=((-common.v1642)/v2868);
        let v2871=((-common.v1643)/v2868);
        let v2873=((-common.v1644)/v2868);
        let v2875=((-common.v1645)/v2868);
        let v3038=(v639*v639);
        let v3039=((-(self.scalar_static_f64[6]*v1850))/v3038);
        let v3042=((-(self.scalar_static_f64[6]*v1851))/v3038);
        let v3045=((-(self.scalar_static_f64[6]*v1852))/v3038);
        let v3048=((-(self.scalar_static_f64[6]*v1853))/v3038);
        let v3082=(v427*v427);
        let v3083=((-v1890)/v3082);
        let v3085=((-v1891)/v3082);
        let v3087=((-v1892)/v3082);
        let v3089=((-v1893)/v3082);
        let v3138=(self.scalar_static_f64[174]*common.v2077);
        let v3139=(self.scalar_static_f64[174]*common.v2078);
        let v3140=(self.scalar_static_f64[174]*common.v2079);
        let v3141=(self.scalar_static_f64[174]*common.v2080);
        let v3174=(((common.v486*(v497*v2143))-(v660*common.v2106))/common.v3173);
        let v3178=(((common.v486*(v497*v2144))-(v660*common.v2108))/common.v3173);
        let v3182=(((common.v486*(v497*v2145))-(v660*common.v2110))/common.v3173);
        let v3186=(((common.v486*(v497*v2146))-(v660*common.v2112))/common.v3173);
        let v3197=((v663*v3174)+(v661*(common.v2093+(common.v206*common.v2097))));
        let v3200=((v663*v3178)+(v661*(common.v2094+(common.v206*common.v2098))));
        let v3203=((v663*v3182)+(v661*(common.v2095+(common.v206*common.v2099))));
        let v3206=((v663*v3186)+(v661*(common.v2096+(common.v206*common.v2100))));
        let v3217=((v666*v3174)+(v661*(common.v2097+(common.v206*common.v2093))));
        let v3220=((v666*v3178)+(v661*(common.v2098+(common.v206*common.v2094))));
        let v3223=((v666*v3182)+(v661*(common.v2099+(common.v206*common.v2095))));
        let v3226=((v666*v3186)+(v661*(common.v2100+(common.v206*common.v2096))));
        let v3258=(v672*v672);
        let v3259=(((v672*((v668*v2189)+(v505*v2139)))-(v669*((v671*common.v1305)+(common.v488*(v2124+v2124)))))/v3258);
        let v3263=(((v672*((v668*v2192)+(v505*v2140)))-(v669*((v671*common.v1306)+(common.v488*(v2127+v2127)))))/v3258);
        let v3267=(((v672*((v668*v2195)+(v505*v2141)))-(v669*((v671*common.v1307)+(common.v488*(v2130+v2130)))))/v3258);
        let v3271=(((v672*((v668*v2198)+(v505*v2142)))-(v669*((v671*common.v1308)+(common.v488*(v2133+v2133)))))/v3258);
        let v3312=((((v673*v2487)+(v566*v3259))+((v664*v2515)+(v569*v3197)))+((v667*((v657*v3138)+(v656*v2487)))+(v658*v3217)));
        let v3313=((((v673*v2490)+(v566*v3263))+((v664*v2518)+(v569*v3200)))+((v667*((v657*v3139)+(v656*v2490)))+(v658*v3220)));
        let v3314=((((v673*v2493)+(v566*v3267))+((v664*v2521)+(v569*v3203)))+((v667*((v657*v3140)+(v656*v2493)))+(v658*v3223)));
        let v3315=((((v673*v2496)+(v566*v3271))+((v664*v2524)+(v569*v3206)))+((v667*((v657*v3141)+(v656*v2496)))+(v658*v3226)));
        let v3356=((((v673*v2499)+(v567*v3259))+((v664*v2527)+(v571*v3197)))+((v667*((v656*v2499)+(v567*v3138)))+(v659*v3217)));
        let v3357=((((v673*v2502)+(v567*v3263))+((v664*v2530)+(v571*v3200)))+((v667*((v656*v2502)+(v567*v3139)))+(v659*v3220)));
        let v3358=((((v673*v2505)+(v567*v3267))+((v664*v2533)+(v571*v3203)))+((v667*((v656*v2505)+(v567*v3140)))+(v659*v3223)));
        let v3359=((((v673*v2508)+(v567*v3271))+((v664*v2536)+(v571*v3206)))+((v667*((v656*v2508)+(v567*v3141)))+(v659*v3226)));
        let v3379=(v685*v685);
        let v3393=(v2124-(((v685*v2189)-(v505*((v684*common.v1305)+(common.v488*(common.v206*v2124)))))/v3379));
        let v3394=(v2127-(((v685*v2192)-(v505*((v684*common.v1306)+(common.v488*(common.v206*v2127)))))/v3379));
        let v3395=(v2130-(((v685*v2195)-(v505*((v684*common.v1307)+(common.v488*(common.v206*v2130)))))/v3379));
        let v3396=(v2133-(((v685*v2198)-(v505*((v684*common.v1308)+(common.v488*(common.v206*v2133)))))/v3379));
        let v3397=(-v2135);
        let v3398=(-v2136);
        let v3399=(-v2137);
        let v3400=(-v2138);
        let v3472=(v695*v695);
        let v3486=(if self.scalar_static_bool[11]{(((v695*v2248)-(v520*((v522*v2232)+(v515*v2252))))/v3472)}else{v3393});
        let v3487=(if self.scalar_static_bool[11]{(((v695*v2249)-(v520*((v522*v2233)+(v515*v2253))))/v3472)}else{v3394});
        let v3488=(if self.scalar_static_bool[11]{(((v695*v2250)-(v520*((v522*v2234)+(v515*v2254))))/v3472)}else{v3395});
        let v3489=(if self.scalar_static_bool[11]{(((v695*v2251)-(v520*((v522*v2235)+(v515*v2255))))/v3472)}else{v3396});
        let v3522=(-((v649*v3083)+(v645*((-((v641*v3039)+(v640*(-v2725))))+(self.scalar_static_f64[212]*v2725)))));
        let v3523=(-((v649*v3085)+(v645*((-((v641*v3042)+(v640*(-v2726))))+(self.scalar_static_f64[212]*v2726)))));
        let v3524=(-((v649*v3087)+(v645*((-((v641*v3045)+(v640*(-v2727))))+(self.scalar_static_f64[212]*v2727)))));
        let v3525=(-((v649*v3089)+(v645*((-((v641*v3048)+(v640*(-v2728))))+(self.scalar_static_f64[212]*v2728)))));
        let v3534=(-((v654*v3083)+(v645*((-((v643*v3039)+(v640*(-v2789))))+(self.scalar_static_f64[212]*v2789)))));
        let v3535=(-((v654*v3085)+(v645*((-((v643*v3042)+(v640*(-v2790))))+(self.scalar_static_f64[212]*v2790)))));
        let v3536=(-((v654*v3087)+(v645*((-((v643*v3045)+(v640*(-v2791))))+(self.scalar_static_f64[212]*v2791)))));
        let v3537=(-((v654*v3089)+(v645*((-((v643*v3048)+(v640*(-v2792))))+(self.scalar_static_f64[212]*v2792)))));
        let v3548=(v537*v537);
        let v3559=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2305))/v3548)}else{v3486});
        let v3560=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2306))/v3548)}else{v3487});
        let v3561=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2307))/v3548)}else{v3488});
        let v3562=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2308))/v3548)}else{v3489});
        let v3649=(v723*v723);
        let v3650=((-(self.scalar_static_f64[99]*((v722*common.v1305)+(v545*((v721*v2338)+(v546*(common.v555*v2349)))))))/v3649);
        let v3653=((-(self.scalar_static_f64[99]*((v722*common.v1306)+(v545*((v721*v2339)+(v546*(common.v555*v2352)))))))/v3649);
        let v3656=((-(self.scalar_static_f64[99]*((v722*common.v1307)+(v545*((v721*v2340)+(v546*(common.v555*v2355)))))))/v3649);
        let v3659=((-(self.scalar_static_f64[99]*((v722*common.v1308)+(v545*((v721*v2341)+(v546*(common.v555*v2358)))))))/v3649);
        let v3768=((self.scalar_static_f64[104]*((v736*(-v2369))+(v732*((v2527+((v733*v2359)+(v550*((if self.scalar_static_bool[12]{(v3534+((v716*v3559)+(v709*(((v693*v3397)+(v688*(v3356+((v687*v2499)+(v567*v3393)))))+(self.scalar_static_f64[21]*v3356)))))}else{(if self.scalar_static_bool[11]{(v3534-(if self.scalar_static_bool[11]{((v697*v2499)+(v567*v3486))}else{common.v1}))}else{common.v1})})+((v724*v2499)+(v567*v3650))))))-((v636*v2854)+(v613*((v2499-((v629*v2860)+(v614*(v2643+((v612*common.v1604)+(common.v352*v2844))))))+((v634*v2869)+(v615*(v2643+((v632*common.v1608)+(common.v353*(-v2844)))))))))))))+(self.scalar_static_f64[104]*((v730*v2369)+(v552*((v2515+((v727*v2359)+(v550*((if self.scalar_static_bool[12]{(v3522+((v711*v3559)+(v709*(((v690*v3397)+(v688*(v3312+((v687*v2487)+(v566*v3393)))))+(self.scalar_static_f64[21]*v3312)))))}else{(if self.scalar_static_bool[11]{(v3522-(if self.scalar_static_bool[11]{((v697*v2487)+(v566*v3486))}else{common.v1}))}else{common.v1})})+((v724*v2487)+(v566*v3650))))))-((v625*v2854)+(v613*((v2487-((v618*v2860)+(v614*(v2623+((v611*common.v1604)+(common.v352*v2832))))))+((v623*v2869)+(v615*(v2623+((v621*common.v1608)+(common.v353*(-v2832))))))))))))));
        let v3769=((self.scalar_static_f64[104]*((v736*(-v2372))+(v732*((v2530+((v733*v2360)+(v550*((if self.scalar_static_bool[12]{(v3535+((v716*v3560)+(v709*(((v693*v3398)+(v688*(v3357+((v687*v2502)+(v567*v3394)))))+(self.scalar_static_f64[21]*v3357)))))}else{(if self.scalar_static_bool[11]{(v3535-(if self.scalar_static_bool[11]{((v697*v2502)+(v567*v3487))}else{common.v1}))}else{common.v1})})+((v724*v2502)+(v567*v3653))))))-((v636*v2855)+(v613*((v2502-((v629*v2862)+(v614*(v2644+((v612*common.v1605)+(common.v352*v2847))))))+((v634*v2871)+(v615*(v2644+((v632*common.v1609)+(common.v353*(-v2847)))))))))))))+(self.scalar_static_f64[104]*((v730*v2372)+(v552*((v2518+((v727*v2360)+(v550*((if self.scalar_static_bool[12]{(v3523+((v711*v3560)+(v709*(((v690*v3398)+(v688*(v3313+((v687*v2490)+(v566*v3394)))))+(self.scalar_static_f64[21]*v3313)))))}else{(if self.scalar_static_bool[11]{(v3523-(if self.scalar_static_bool[11]{((v697*v2490)+(v566*v3487))}else{common.v1}))}else{common.v1})})+((v724*v2490)+(v566*v3653))))))-((v625*v2855)+(v613*((v2490-((v618*v2862)+(v614*(v2624+((v611*common.v1605)+(common.v352*v2835))))))+((v623*v2871)+(v615*(v2624+((v621*common.v1609)+(common.v353*(-v2835))))))))))))));
        let v3770=((self.scalar_static_f64[104]*((v736*(-v2375))+(v732*((v2533+((v733*v2361)+(v550*((if self.scalar_static_bool[12]{(v3536+((v716*v3561)+(v709*(((v693*v3399)+(v688*(v3358+((v687*v2505)+(v567*v3395)))))+(self.scalar_static_f64[21]*v3358)))))}else{(if self.scalar_static_bool[11]{(v3536-(if self.scalar_static_bool[11]{((v697*v2505)+(v567*v3488))}else{common.v1}))}else{common.v1})})+((v724*v2505)+(v567*v3656))))))-((v636*v2856)+(v613*((v2505-((v629*v2864)+(v614*(v2645+((v612*common.v1606)+(common.v352*v2850))))))+((v634*v2873)+(v615*(v2645+((v632*common.v1610)+(common.v353*(-v2850)))))))))))))+(self.scalar_static_f64[104]*((v730*v2375)+(v552*((v2521+((v727*v2361)+(v550*((if self.scalar_static_bool[12]{(v3524+((v711*v3561)+(v709*(((v690*v3399)+(v688*(v3314+((v687*v2493)+(v566*v3395)))))+(self.scalar_static_f64[21]*v3314)))))}else{(if self.scalar_static_bool[11]{(v3524-(if self.scalar_static_bool[11]{((v697*v2493)+(v566*v3488))}else{common.v1}))}else{common.v1})})+((v724*v2493)+(v566*v3656))))))-((v625*v2856)+(v613*((v2493-((v618*v2864)+(v614*(v2625+((v611*common.v1606)+(common.v352*v2838))))))+((v623*v2873)+(v615*(v2625+((v621*common.v1610)+(common.v353*(-v2838))))))))))))));
        let v3771=((self.scalar_static_f64[104]*((v736*(-v2378))+(v732*((v2536+((v733*v2362)+(v550*((if self.scalar_static_bool[12]{(v3537+((v716*v3562)+(v709*(((v693*v3400)+(v688*(v3359+((v687*v2508)+(v567*v3396)))))+(self.scalar_static_f64[21]*v3359)))))}else{(if self.scalar_static_bool[11]{(v3537-(if self.scalar_static_bool[11]{((v697*v2508)+(v567*v3489))}else{common.v1}))}else{common.v1})})+((v724*v2508)+(v567*v3659))))))-((v636*v2857)+(v613*((v2508-((v629*v2866)+(v614*(v2646+((v612*common.v1607)+(common.v352*v2853))))))+((v634*v2875)+(v615*(v2646+((v632*common.v1611)+(common.v353*(-v2853)))))))))))))+(self.scalar_static_f64[104]*((v730*v2378)+(v552*((v2524+((v727*v2362)+(v550*((if self.scalar_static_bool[12]{(v3525+((v711*v3562)+(v709*(((v690*v3400)+(v688*(v3315+((v687*v2496)+(v566*v3396)))))+(self.scalar_static_f64[21]*v3315)))))}else{(if self.scalar_static_bool[11]{(v3525-(if self.scalar_static_bool[11]{((v697*v2496)+(v566*v3489))}else{common.v1}))}else{common.v1})})+((v724*v2496)+(v566*v3659))))))-((v625*v2857)+(v613*((v2496-((v618*v2866)+(v614*(v2626+((v611*common.v1607)+(common.v352*v2841))))))+((v623*v2875)+(v615*(v2626+((v621*common.v1611)+(common.v353*(-v2841))))))))))))));
        let v3773=(v746*v746);
        let v3783=((v747*((v552*v2359)+(v550*v2369)))+(v553*((-v3768)/v3773)));
        let v3786=((v747*((v552*v2360)+(v550*v2372)))+(v553*((-v3769)/v3773)));
        let v3789=((v747*((v552*v2361)+(v550*v2375)))+(v553*((-v3770)/v3773)));
        let v3792=((v747*((v552*v2362)+(v550*v2378)))+(v553*((-v3771)/v3773)));
        let v3794=(self.scalar_static_f64[13]*common.v1516);
        let v3797=(common.v1519-(self.scalar_static_f64[13]*common.v1515));
        let v3799=(common.v1520-(self.scalar_static_f64[13]*common.v1517));
        let v3800=(common.v1521-(self.scalar_static_f64[13]*common.v1518));
        let v3802=(v750*v750);
        let v4180=ddt_scale;
        let v4212=(v951*v951);
        let v4231=(v962*v962);
        let v4252=(v974*v974);
        let v4277=(v1005*v1005);
        let v4292=(v1014*v1014);
        let v4309=(v1024*v1024);
        let v4451=(self.scalar_static_f64[20]*(common.v4013*v4180));
        let v4452=(self.scalar_static_f64[20]*(common.v4016*v4180));
        let v4453=(self.scalar_static_f64[20]*(common.v4019*v4180));
        let v4454=(self.scalar_static_f64[20]*(common.v4022*v4180));
        let v4459=(self.scalar_static_f64[20]*(common.v4097*v4180));
        let v4460=(self.scalar_static_f64[20]*(common.v4100*v4180));
        let v4461=(self.scalar_static_f64[20]*(common.v4103*v4180));
        let v4462=(self.scalar_static_f64[20]*(common.v4106*v4180));
        let v4467=(self.scalar_static_f64[20]*(if v770{common.v1}else{(if v753{((v767*v3783)+(v748*(if v753{((v765*(if v753{(v763*(if v761{common.v1}else{(if v753{(self.scalar_static_f64[241]*(if v753{((-v3797)/v3802)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v764*(self.scalar_static_f64[215]*v3797)))}else{common.v1})))}else{common.v1})}));
        let v4468=(self.scalar_static_f64[20]*(if v770{common.v1}else{(if v753{((v767*v3786)+(v748*(if v753{((v765*(if v753{(v763*(if v761{common.v1}else{(if v753{(self.scalar_static_f64[241]*(if v753{(v3794/v3802)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v764*(self.scalar_static_f64[215]*(-v3794))))}else{common.v1})))}else{common.v1})}));
        let v4469=(self.scalar_static_f64[20]*(if v770{common.v1}else{(if v753{((v767*v3789)+(v748*(if v753{((v765*(if v753{(v763*(if v761{common.v1}else{(if v753{(self.scalar_static_f64[241]*(if v753{((-v3799)/v3802)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v764*(self.scalar_static_f64[215]*v3799)))}else{common.v1})))}else{common.v1})}));
        let v4470=(self.scalar_static_f64[20]*(if v770{common.v1}else{(if v753{((v767*v3792)+(v748*(if v753{((v765*(if v753{(v763*(if v761{common.v1}else{(if v753{(self.scalar_static_f64[241]*(if v753{((-v3800)/v3802)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v764*(self.scalar_static_f64[215]*v3800)))}else{common.v1})))}else{common.v1})}));

        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * ((v748*v1123)),
            &[(v1123*v3783),(v1123*v3786),(v1123*v3789),(v1123*v3792)],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if common.v826{v1125}else{common.v1})),
            &[(if common.v826{v4451}else{common.v1}),(if common.v826{v4452}else{common.v1}),(if common.v826{v4453}else{common.v1}),(if common.v826{v4454}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if common.v826{v1127}else{common.v1})),
            &[(if common.v826{v4459}else{common.v1}),(if common.v826{v4460}else{common.v1}),(if common.v826{v4461}else{common.v1}),(if common.v826{v4462}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if common.v826{v1129}else{common.v1})),
            &[(if common.v826{v4467}else{common.v1}),(if common.v826{v4468}else{common.v1}),(if common.v826{v4469}else{common.v1}),(if common.v826{v4470}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if common.v1131{v1125}else{common.v1})),
            &[(if common.v1131{v4451}else{common.v1}),(if common.v1131{v4452}else{common.v1}),(if common.v1131{v4453}else{common.v1}),(if common.v1131{v4454}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if common.v1131{v1127}else{common.v1})),
            &[(if common.v1131{v4459}else{common.v1}),(if common.v1131{v4460}else{common.v1}),(if common.v1131{v4461}else{common.v1}),(if common.v1131{v4462}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if common.v1131{v1129}else{common.v1})),
            &[(if common.v1131{v4467}else{common.v1}),(if common.v1131{v4468}else{common.v1}),(if common.v1131{v4469}else{common.v1}),(if common.v1131{v4470}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[20]*v1135)),
            &[(self.scalar_static_f64[20]*(common.v4176*v4180)),(self.scalar_static_f64[20]*(common.v4177*v4180)),(self.scalar_static_f64[20]*(common.v4178*v4180)),(self.scalar_static_f64[20]*(common.v4179*v4180))],
            &[],
            multiplicity,
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(2),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v953-common.v50))-(self.scalar_static_f64[277]*(v964-common.v50)))-(self.scalar_static_f64[276]*(v976-common.v50)))+((v940*v1139)+(common.v191*self.scalar_static_f64[159])))))),
            0,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v953*(((v951*self.scalar_static_f64[303])-(v947*(if v950{self.scalar_static_f64[20]}else{common.v1})))/v4212)))-(self.scalar_static_f64[277]*(v964*(((v962*self.scalar_static_f64[307])-(v959*(if v961{self.scalar_static_f64[20]}else{common.v1})))/v4231))))-(self.scalar_static_f64[276]*(v976*(((v974*self.scalar_static_f64[311])-(v971*(if v973{self.scalar_static_f64[20]}else{common.v1})))/v4252))))+(((v1139*(if v934{(self.scalar_static_f64[148]*(v937*self.scalar_static_f64[299]))}else{common.v1}))+(v940*(self.scalar_static_f64[280]*(-(v1137*(if v925{common.v1}else{self.scalar_static_f64[297]}))))))+self.scalar_static_f64[167]))))),
            3,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v953*(((v951*self.scalar_static_f64[304])-(v947*(if v950{self.scalar_static_f64[160]}else{common.v1})))/v4212)))-(self.scalar_static_f64[277]*(v964*(((v962*self.scalar_static_f64[308])-(v959*(if v961{self.scalar_static_f64[160]}else{common.v1})))/v4231))))-(self.scalar_static_f64[276]*(v976*(((v974*self.scalar_static_f64[312])-(v971*(if v973{self.scalar_static_f64[160]}else{common.v1})))/v4252))))+(((v1139*(if v934{(self.scalar_static_f64[148]*(v937*self.scalar_static_f64[300]))}else{common.v1}))+(v940*(self.scalar_static_f64[280]*(-(v1137*(if v925{common.v1}else{self.scalar_static_f64[298]}))))))+self.scalar_static_f64[168]))))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v1007-common.v50))-(self.scalar_static_f64[287]*(v1016-common.v50)))-(self.scalar_static_f64[286]*(v1026-common.v50)))+((v999*v1149)+(common.v188*self.scalar_static_f64[159])))))),
            2,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v1007*(((v1005*self.scalar_static_f64[303])-(v1002*(if v1004{self.scalar_static_f64[20]}else{common.v1})))/v4277)))-(self.scalar_static_f64[287]*(v1016*(((v1014*self.scalar_static_f64[307])-(v1011*(if v1013{self.scalar_static_f64[20]}else{common.v1})))/v4292))))-(self.scalar_static_f64[286]*(v1026*(((v1024*self.scalar_static_f64[311])-(v1021*(if v1023{self.scalar_static_f64[20]}else{common.v1})))/v4309))))+(self.scalar_static_f64[167]+((v1149*(if v994{(self.scalar_static_f64[148]*(v996*self.scalar_static_f64[299]))}else{common.v1}))+(v999*(self.scalar_static_f64[289]*(-(v1147*(if v987{common.v1}else{self.scalar_static_f64[297]}))))))))))),
            3,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[282]*(v1007*(((v1005*self.scalar_static_f64[304])-(v1002*(if v1004{self.scalar_static_f64[160]}else{common.v1})))/v4277)))-(self.scalar_static_f64[287]*(v1016*(((v1014*self.scalar_static_f64[308])-(v1011*(if v1013{self.scalar_static_f64[160]}else{common.v1})))/v4292))))-(self.scalar_static_f64[286]*(v1026*(((v1024*self.scalar_static_f64[312])-(v1021*(if v1023{self.scalar_static_f64[160]}else{common.v1})))/v4309))))+(self.scalar_static_f64[168]+((v1149*(if v994{(self.scalar_static_f64[148]*(v996*self.scalar_static_f64[300]))}else{common.v1}))+(v999*(self.scalar_static_f64[289]*(-(v1147*(if v987{common.v1}else{self.scalar_static_f64[298]}))))))))))),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*v1156))),
            0,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4393)))),
            3,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4396)))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*v1159))),
            2,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4443)))),
            3,
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4446)))),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v824=0.0;
        let v825=0.0;
        let v1125=(self.scalar_static_f64[20]*v824);
        let v1127=(self.scalar_static_f64[20]*v825);
        let v1135=0.0;
        let v1156=0.0;
        let v1159=0.0;
        let v4180=1.0;
        let v4451=(self.scalar_static_f64[20]*(common.v4013*v4180));
        let v4452=(self.scalar_static_f64[20]*(common.v4016*v4180));
        let v4453=(self.scalar_static_f64[20]*(common.v4019*v4180));
        let v4454=(self.scalar_static_f64[20]*(common.v4022*v4180));
        let v4459=(self.scalar_static_f64[20]*(common.v4097*v4180));
        let v4460=(self.scalar_static_f64[20]*(common.v4100*v4180));
        let v4461=(self.scalar_static_f64[20]*(common.v4103*v4180));
        let v4462=(self.scalar_static_f64[20]*(common.v4106*v4180));

        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v826{v4451}else{common.v1}),(if common.v826{v4452}else{common.v1}),(if common.v826{v4453}else{common.v1}),(if common.v826{v4454}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v826{v4459}else{common.v1}),(if common.v826{v4460}else{common.v1}),(if common.v826{v4461}else{common.v1}),(if common.v826{v4462}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v1131{v4451}else{common.v1}),(if common.v1131{v4452}else{common.v1}),(if common.v1131{v4453}else{common.v1}),(if common.v1131{v4454}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v1131{v4459}else{common.v1}),(if common.v1131{v4460}else{common.v1}),(if common.v1131{v4461}else{common.v1}),(if common.v1131{v4462}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[3]),
            &nodes,
            &[(self.scalar_static_f64[20]*(common.v4176*v4180)),(self.scalar_static_f64[20]*(common.v4177*v4180)),(self.scalar_static_f64[20]*(common.v4178*v4180)),(self.scalar_static_f64[20]*(common.v4179*v4180))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4393)))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4396)))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4443)))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[86]*(self.scalar_static_f64[20]*(v4180*common.v4446)))),
        );
    }
}
