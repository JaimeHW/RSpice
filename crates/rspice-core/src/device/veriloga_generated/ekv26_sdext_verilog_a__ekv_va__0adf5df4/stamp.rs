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
    v1: f64, v25: f64, v31: f64, v192: f64, v195: f64, v205: f64, 
    v211: f64, v220: f64, v223: f64, v227: f64, v230: f64, v256: f64, 
    v258: f64, v262: f64, v266: f64, v325: f64, v327: f64, v328: f64, 
    v332: f64, v334: f64, v335: f64, v336: f64, v342: f64, v357: f64, 
    v360: f64, v361: f64, v364: f64, v367: f64, v422: f64, v492: f64, 
    v495: f64, v496: f64, v497: f64, v498: f64, v499: f64, v500: f64, 
    v501: f64, v502: f64, v504: f64, v520: f64, v571: f64, v678: f64, 
    v681: f64, v816: f64, v829: f64, v841: f64, v845: f64, v1107: f64, 
    v1151: f64, v1160: bool, v1215: f64, v1216: f64, v1217: f64, v1225: f64, 
    v1226: f64, v1227: f64, v1235: f64, v1236: f64, v1237: f64, v1245: f64, 
    v1246: f64, v1247: f64, v1279: f64, v1280: f64, v1281: f64, v1282: f64, 
    v1287: f64, v1288: f64, v1289: f64, v1290: f64, v1310: f64, v1311: f64, 
    v1312: f64, v1313: f64, v1334: f64, v1335: f64, v1336: f64, v1337: f64, 
    v1515: f64, v1516: f64, v1517: f64, v1518: f64, v1521: f64, v1524: f64, 
    v1527: f64, v1530: f64, v1531: f64, v1532: f64, v1533: f64, v1534: f64, 
    v1540: f64, v1541: f64, v1542: f64, v1543: f64, v1544: f64, v1545: f64, 
    v1546: f64, v1547: f64, v1548: f64, v1549: f64, v1550: f64, v1551: f64, 
    v1552: f64, v1553: f64, v1566: f64, v1567: f64, v1568: f64, v1569: f64, 
    v1629: f64, v1630: f64, v1631: f64, v1632: f64, v1633: f64, v1634: f64, 
    v1635: f64, v1636: f64, v1637: f64, v1638: f64, v1639: f64, v1640: f64, 
    v1654: f64, v1655: f64, v1656: f64, v1657: f64, v1671: f64, v1672: f64, 
    v1673: f64, v1674: f64, v1863: f64, v1864: f64, v1865: f64, v1866: f64, 
    v2106: f64, v2107: f64, v2108: f64, v2109: f64, v2112: f64, v2115: f64, 
    v2118: f64, v2121: f64, v2122: f64, v2123: f64, v2124: f64, v2125: f64, 
    v2126: f64, v2127: f64, v2128: f64, v2129: f64, v2130: f64, v2131: f64, 
    v2132: f64, v2133: f64, v2135: f64, v2137: f64, v2139: f64, v2141: f64, 
    v2146: f64, v2147: f64, v2148: f64, v2149: f64, v3202: f64, v4042: f64, 
    v4045: f64, v4048: f64, v4051: f64, v4126: f64, v4129: f64, v4132: f64, 
    v4135: f64, v4205: f64, v4206: f64, v4207: f64, v4208: f64, v4422: f64, 
    v4425: f64, v4472: f64, v4475: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=0.0;
        let v9=3.0;
        let v25=0.5;
        let v31=1.0;
        let v187=ctx.node_voltage(nodes[3]);
        let v192=(self.scalar_static_f64[20]*(ctx.node_voltage(nodes[2])-v187));
        let v195=(self.scalar_static_f64[20]*(ctx.node_voltage(nodes[0])-v187));
        let v198=(if ((v195-v192)<v1){v31}else{v1});
        let v202=(if (v198!=0.0){v195}else{v192});
        let v203=(if (v198!=0.0){(if (v198!=0.0){v192}else{v1})}else{v195});
        let v205=(if (!(v198!=0.0)){v31}else{(if (v198!=0.0){-1.0}else{v1})});
        let v209=(self.scalar_static_f64[241]+(self.scalar_static_f64[219]+(((self.scalar_static_f64[20]*(ctx.node_voltage(nodes[1])-v187))-self.scalar_static_f64[237])-self.scalar_static_f64[89])));
        let v211=2.0;
        let v214=(((v209*v209)+self.scalar_static_f64[242])).sqrt();
        let v216=(v25*(v209+v214));
        let v217=(self.scalar_static_f64[219]+v202);
        let v220=((self.scalar_static_f64[188]+(v217*v217))).sqrt();
        let v223=((v25*(v217+v220))).sqrt();
        let v224=(self.scalar_static_f64[219]+v203);
        let v227=((self.scalar_static_f64[188]+(v224*v224))).sqrt();
        let v230=((v25*(v224+v227))).sqrt();
        let v236=0.25;
        let v240=((v216+self.scalar_static_f64[96])).sqrt();
        let v241=(v216-self.scalar_static_f64[219]);
        let v248=((self.scalar_static_f64[182]+(self.scalar_static_f64[219]+(v241-(self.scalar_static_f64[70]*(v240-self.scalar_static_f64[97])))))).sqrt();
        let v253=((self.scalar_static_f64[70]-(self.scalar_static_f64[94]*(v223+v230)))+(self.scalar_static_f64[92]*v248));
        let v256=((self.scalar_static_f64[182]+(v253*v253))).sqrt();
        let v258=(v25*(v253+v256));
        let v259=(v236*v258);
        let v262=((v216+(v258*v259))).sqrt();
        let v264=(v262-(v25*v258));
        let v266=(v241-(v258*v264));
        let v268=(self.scalar_static_f64[183]*(v266-v202));
        let v269=-0.35;
        let v271=(if (v268>v269){v31}else{v1});
        let v272=1.3;
        let v274=1.6;
        let v275=(v268+v274);
        let v277=((v268+v272)-(v275).ln());
        let v279=(if (v271!=0.0){(v211/v277)}else{v1});
        let v280=(v211+v279);
        let v281=(v31+v268);
        let v283=(v281+(v279).ln());
        let v285=(if (v271!=0.0){(v280/v283)}else{v1});
        let v287=(v281+(v285).ln());
        let v288=(v211+v285);
        let v291=-15.0;
        let v293=(if (v268>v291){v31}else{v1});
        let v294=(!(v271!=0.0));
        let v295=((v293!=0.0)&&v294);
        let v296=1.55;
        let v298=((-v268)).exp();
        let v300=(if v295{(v296+v298)}else{v279});
        let v301=(v211+v300);
        let v303=(v281+(v300).ln());
        let v305=(if v295{(v301/v303)}else{v285});
        let v307=(v281+(v305).ln());
        let v308=(v211+v305);
        let v311=-23.0;
        let v313=(if (v268>v311){v31}else{v1});
        let v315=(v294&&(!(v293!=0.0)));
        let v316=((v313!=0.0)&&v315);
        let v317=(v211+v298);
        let v321=(v315&&(!(v313!=0.0)));
        let v322=(v268).exp();
        let v323=1e-64;
        let v325=(if v321{(v322+v323)}else{(if v316{(v31/v317)}else{(if v295{(v307/v308)}else{(if (v271!=0.0){(v287/v288)}else{v1})})})});
        let v326=(v31+v325);
        let v327=(v325*v326);
        let v328=(v327).sqrt();
        let v332=((v236+(v328*self.scalar_static_f64[243]))).sqrt();
        let v334=(self.scalar_static_f64[225]*(v332-v25));
        let v335=(v203-v202);
        let v336=(v25*v335);
        let v342=(self.scalar_static_f64[188]*((self.scalar_static_f64[5]*(v328-(self.scalar_static_f64[183]*v334)))+0.015625));
        let v351=0.75;
        let v357=((v236+(self.scalar_static_f64[243]*(v328-(v351*(v327).ln()))))).sqrt();
        let v360=(self.scalar_static_f64[230]+(self.scalar_static_f64[225]*(v357-v25)));
        let v361=(v336-v360);
        let v364=((v342+(v360*v360))).sqrt();
        let v367=((v342+(v361*v361))).sqrt();
        let v372=(self.scalar_static_f64[183]*(v367+(((v266-v336)-v202)-v364)));
        let v374=(if (v372>v269){v31}else{v1});
        let v376=(v274+v372);
        let v378=((v272+v372)-(v376).ln());
        let v380=(if (v374!=0.0){(v211/v378)}else{v300});
        let v381=(v211+v380);
        let v382=(v31+v372);
        let v384=(v382+(v380).ln());
        let v386=(if (v374!=0.0){(v381/v384)}else{v305});
        let v388=(v382+(v386).ln());
        let v389=(v211+v386);
        let v393=(if (v372>v291){v31}else{v1});
        let v394=(!(v374!=0.0));
        let v395=((v393!=0.0)&&v394);
        let v397=((-v372)).exp();
        let v399=(if v395{(v296+v397)}else{v380});
        let v400=(v211+v399);
        let v402=(v382+(v399).ln());
        let v404=(if v395{(v400/v402)}else{v386});
        let v406=(v382+(v404).ln());
        let v407=(v211+v404);
        let v411=(if (v372>v311){v31}else{v1});
        let v413=(v394&&(!(v393!=0.0)));
        let v414=((v411!=0.0)&&v413);
        let v415=(v211+v397);
        let v419=(v413&&(!(v411!=0.0)));
        let v420=(v372).exp();
        let v422=(if v419{(v323+v420)}else{(if v414{(v31/v415)}else{(if v395{(v406/v407)}else{(if (v374!=0.0){(v388/v389)}else{v325})})})});
        let v442=(self.scalar_static_f64[183]*(v266-v203));
        let v444=(if (v442>v269){v31}else{v1});
        let v446=(v274+v442);
        let v448=((v272+v442)-(v446).ln());
        let v450=(if (v444!=0.0){(v211/v448)}else{v399});
        let v451=(v211+v450);
        let v452=(v31+v442);
        let v454=(v452+(v450).ln());
        let v456=(if (v444!=0.0){(v451/v454)}else{v404});
        let v458=(v452+(v456).ln());
        let v459=(v211+v456);
        let v463=(if (v442>v291){v31}else{v1});
        let v464=(!(v444!=0.0));
        let v465=((v463!=0.0)&&v464);
        let v467=((-v442)).exp();
        let v469=(if v465{(v296+v467)}else{v450});
        let v470=(v211+v469);
        let v472=(v452+(v469).ln());
        let v474=(if v465{(v470/v472)}else{v456});
        let v476=(v452+(v474).ln());
        let v477=(v211+v474);
        let v481=(if (v442>v311){v31}else{v1});
        let v483=(v464&&(!(v463!=0.0)));
        let v484=((v481!=0.0)&&v483);
        let v485=(v211+v467);
        let v489=(v483&&(!(v481!=0.0)));
        let v490=(v442).exp();
        let v492=(if v489{(v323+v490)}else{(if v484{(v31/v485)}else{(if v465{(v476/v477)}else{(if (v444!=0.0){(v458/v459)}else{v422})})})});
        let v493=(v31+v492);
        let v495=(v236+v327);
        let v496=(v236+(v492*v493));
        let v497=(v495).sqrt();
        let v498=(v496).sqrt();
        let v499=(v497+v498);
        let v500=(v499*v499);
        let v501=(self.scalar_static_f64[219]+v266);
        let v502=(1e-6+v501);
        let v504=(v211*(v502).sqrt());
        let v520=-0.5;
        let v570=(v211*v328);
        let v571=4.0;
        let v678=(v211*v498);
        let v681=(v211*v497);
        let v791=(v495*v497);
        let v792=(v496*v498);
        let v795=((self.scalar_static_f64[219]+(v25*v266))).sqrt();
        let v796=(v795+v795);
        let v801=(-(self.scalar_static_f64[110]*(self.scalar_static_f64[181]*(v31+(v258/v796)))));
        let v802=0.266666666;
        let v804=6.0;
        let v805=(v496*v804);
        let v808=(v498*v571);
        let v813=(v802*((((v9*v792)+(v497*v805))+(v495*v808))+(v211*v791)));
        let v815=((v813/v500)-v25);
        let v816=(v801*v815);
        let v818=(v495*v804);
        let v821=(v497*v571);
        let v826=(v802*((((v9*v791)+(v498*v818))+(v496*v821))+(v211*v792)));
        let v828=((v826/v500)-v25);
        let v829=(v801*v828);
        let v830=(v816+v829);
        let v831=(v258*v520);
        let v836=(v258*v830);
        let v837=(v258+v796);
        let v841=((-v830)-((self.scalar_static_f64[110]*((v216+(v504*v831))-v209))-(v836/v837)));
        let v845=(if (v31==v205){v31}else{v1});
        let v1058=(if (v195>v1){v31}else{v1});
        let v1063=(v31+(v195/self.scalar_static_f64[261]));
        let v1066=((self.scalar_static_f64[163]*(v1063).ln())).exp();
        let v1073=(v31+(v195/self.scalar_static_f64[263]));
        let v1076=((self.scalar_static_f64[165]*(v1073).ln())).exp();
        let v1083=(v31+(v195/self.scalar_static_f64[265]));
        let v1086=((self.scalar_static_f64[167]*(v1083).ln())).exp();
        let v1089=(!(v1058!=0.0));
        let v1106=((if v1089{(self.scalar_static_f64[301]*(v31-((v195*self.scalar_static_f64[166])/self.scalar_static_f64[265])))}else{(if (v1058!=0.0){(self.scalar_static_f64[301]*v1086)}else{v1})})+((if v1089{(self.scalar_static_f64[299]*(v31-((v195*self.scalar_static_f64[162])/self.scalar_static_f64[261])))}else{(if (v1058!=0.0){(self.scalar_static_f64[299]*v1066)}else{v1})})+(if v1089{(self.scalar_static_f64[300]*(v31-((v195*self.scalar_static_f64[164])/self.scalar_static_f64[263])))}else{(if (v1058!=0.0){(self.scalar_static_f64[300]*v1076)}else{v1})})));
        let v1107=(v195*v1106);
        let v1109=(if (v192>v1){v31}else{v1});
        let v1112=(v31+(v192/self.scalar_static_f64[261]));
        let v1115=((self.scalar_static_f64[163]*(v1112).ln())).exp();
        let v1120=(v31+(v192/self.scalar_static_f64[263]));
        let v1123=((self.scalar_static_f64[165]*(v1120).ln())).exp();
        let v1127=(v31+(v192/self.scalar_static_f64[265]));
        let v1130=((self.scalar_static_f64[167]*(v1127).ln())).exp();
        let v1133=(!(v1109!=0.0));
        let v1150=((if v1133{(self.scalar_static_f64[301]*(v31-((v192*self.scalar_static_f64[166])/self.scalar_static_f64[265])))}else{(if (v1109!=0.0){(self.scalar_static_f64[301]*v1130)}else{v1})})+((if v1133{(self.scalar_static_f64[302]*(v31-((v192*self.scalar_static_f64[162])/self.scalar_static_f64[261])))}else{(if (v1109!=0.0){(self.scalar_static_f64[302]*v1115)}else{v1})})+(if v1133{(self.scalar_static_f64[303]*(v31-((v192*self.scalar_static_f64[164])/self.scalar_static_f64[263])))}else{(if (v1109!=0.0){(self.scalar_static_f64[303]*v1123)}else{v1})})));
        let v1151=(v192*v1150);
        let v1160=(!(v845!=0.0));
        let v1192=(if (v198!=0.0){self.scalar_static_f64[20]}else{v1});
        let v1194=(if (v198!=0.0){v1}else{self.scalar_static_f64[20]});
        let v1195=(if (v198!=0.0){v1192}else{v1});
        let v1196=(if (v198!=0.0){(if (v198!=0.0){self.scalar_static_f64[169]}else{v1})}else{self.scalar_static_f64[169]});
        let v1197=(self.scalar_static_f64[20]*v209);
        let v1199=(v209*self.scalar_static_f64[169]);
        let v1201=(v211*v214);
        let v1206=(v25*(self.scalar_static_f64[20]+((v1197+v1197)/v1201)));
        let v1207=(v25*(self.scalar_static_f64[169]+((v1199+v1199)/v1201)));
        let v1208=(v217*v1192);
        let v1210=(v217*v1194);
        let v1212=(v217*self.scalar_static_f64[169]);
        let v1214=(v211*v220);
        let v1215=((v1208+v1208)/v1214);
        let v1216=((v1210+v1210)/v1214);
        let v1217=((v1212+v1212)/v1214);
        let v1224=(v211*v223);
        let v1225=((v25*(v1192+v1215))/v1224);
        let v1226=((v25*(v1194+v1216))/v1224);
        let v1227=((v25*(self.scalar_static_f64[169]+v1217))/v1224);
        let v1228=(v224*v1194);
        let v1230=(v224*v1195);
        let v1232=(v224*v1196);
        let v1234=(v211*v227);
        let v1235=((v1228+v1228)/v1234);
        let v1236=((v1230+v1230)/v1234);
        let v1237=((v1232+v1232)/v1234);
        let v1244=(v211*v230);
        let v1245=((v25*(v1194+v1235))/v1244);
        let v1246=((v25*(v1195+v1236))/v1244);
        let v1247=((v25*(v1196+v1237))/v1244);
        let v1248=(v211*v240);
        let v1255=(v211*v248);
        let v1264=(-(self.scalar_static_f64[94]*(v1225+v1245)));
        let v1265=(-(self.scalar_static_f64[94]*(v1226+v1246)));
        let v1267=(self.scalar_static_f64[92]*((v1206-(self.scalar_static_f64[70]*(v1206/v1248)))/v1255));
        let v1269=((-(self.scalar_static_f64[94]*(v1227+v1247)))+(self.scalar_static_f64[92]*((v1207-(self.scalar_static_f64[70]*(v1207/v1248)))/v1255)));
        let v1270=(v253*v1264);
        let v1272=(v253*v1267);
        let v1274=(v253*v1265);
        let v1276=(v253*v1269);
        let v1278=(v211*v256);
        let v1279=((v1270+v1270)/v1278);
        let v1280=((v1272+v1272)/v1278);
        let v1281=((v1274+v1274)/v1278);
        let v1282=((v1276+v1276)/v1278);
        let v1287=(v25*(v1264+v1279));
        let v1288=(v25*(v1267+v1280));
        let v1289=(v25*(v1265+v1281));
        let v1290=(v25*(v1269+v1282));
        let v1309=(v211*v262);
        let v1310=(((v259*v1287)+(v258*(v236*v1287)))/v1309);
        let v1311=((v1206+((v259*v1288)+(v258*(v236*v1288))))/v1309);
        let v1312=(((v259*v1289)+(v258*(v236*v1289)))/v1309);
        let v1313=((v1207+((v259*v1290)+(v258*(v236*v1290))))/v1309);
        let v1334=(-((v264*v1287)+(v258*(v1310-(v25*v1287)))));
        let v1335=(v1206-((v264*v1288)+(v258*(v1311-(v25*v1288)))));
        let v1336=(-((v264*v1289)+(v258*(v1312-(v25*v1289)))));
        let v1337=(v1207-((v264*v1290)+(v258*(v1313-(v25*v1290)))));
        let v1341=(self.scalar_static_f64[183]*(v1334-v1192));
        let v1342=(self.scalar_static_f64[183]*v1335);
        let v1343=(self.scalar_static_f64[183]*(v1336-v1194));
        let v1344=(self.scalar_static_f64[183]*(v1337-self.scalar_static_f64[169]));
        let v1355=(v277*v277);
        let v1366=(if (v271!=0.0){((-(v211*(v1341-(v1341/v275))))/v1355)}else{v1});
        let v1367=(if (v271!=0.0){((-(v211*(v1342-(v1342/v275))))/v1355)}else{v1});
        let v1368=(if (v271!=0.0){((-(v211*(v1343-(v1343/v275))))/v1355)}else{v1});
        let v1369=(if (v271!=0.0){((-(v211*(v1344-(v1344/v275))))/v1355)}else{v1});
        let v1381=(v283*v283);
        let v1395=(if (v271!=0.0){(((v283*v1366)-(v280*(v1341+(v1366/v279))))/v1381)}else{v1});
        let v1396=(if (v271!=0.0){(((v283*v1367)-(v280*(v1342+(v1367/v279))))/v1381)}else{v1});
        let v1397=(if (v271!=0.0){(((v283*v1368)-(v280*(v1343+(v1368/v279))))/v1381)}else{v1});
        let v1398=(if (v271!=0.0){(((v283*v1369)-(v280*(v1344+(v1369/v279))))/v1381)}else{v1});
        let v1410=(v288*v288);
        let v1429=(-v1342);
        let v1432=(v298*(-v1341));
        let v1433=(v298*v1429);
        let v1434=(v298*(-v1343));
        let v1435=(v298*(-v1344));
        let v1436=(if v295{v1432}else{v1366});
        let v1437=(if v295{v1433}else{v1367});
        let v1438=(if v295{v1434}else{v1368});
        let v1439=(if v295{v1435}else{v1369});
        let v1451=(v303*v303);
        let v1465=(if v295{(((v303*v1436)-(v301*(v1341+(v1436/v300))))/v1451)}else{v1395});
        let v1466=(if v295{(((v303*v1437)-(v301*(v1342+(v1437/v300))))/v1451)}else{v1396});
        let v1467=(if v295{(((v303*v1438)-(v301*(v1343+(v1438/v300))))/v1451)}else{v1397});
        let v1468=(if v295{(((v303*v1439)-(v301*(v1344+(v1439/v300))))/v1451)}else{v1398});
        let v1480=(v308*v308);
        let v1499=(v317*v317);
        let v1515=(if v321{(v322*v1341)}else{(if v316{((-v1432)/v1499)}else{(if v295{(((v308*(v1341+(v1465/v305)))-(v307*v1465))/v1480)}else{(if (v271!=0.0){(((v288*(v1341+(v1395/v285)))-(v287*v1395))/v1410)}else{v1})})})});
        let v1516=(if v321{(v322*v1342)}else{(if v316{((-v1433)/v1499)}else{(if v295{(((v308*(v1342+(v1466/v305)))-(v307*v1466))/v1480)}else{(if (v271!=0.0){(((v288*(v1342+(v1396/v285)))-(v287*v1396))/v1410)}else{v1})})})});
        let v1517=(if v321{(v322*v1343)}else{(if v316{((-v1434)/v1499)}else{(if v295{(((v308*(v1343+(v1467/v305)))-(v307*v1467))/v1480)}else{(if (v271!=0.0){(((v288*(v1343+(v1397/v285)))-(v287*v1397))/v1410)}else{v1})})})});
        let v1518=(if v321{(v322*v1344)}else{(if v316{((-v1435)/v1499)}else{(if v295{(((v308*(v1344+(v1468/v305)))-(v307*v1468))/v1480)}else{(if (v271!=0.0){(((v288*(v1344+(v1398/v285)))-(v287*v1398))/v1410)}else{v1})})})});
        let v1521=((v326*v1515)+(v325*v1515));
        let v1524=((v326*v1516)+(v325*v1516));
        let v1527=((v326*v1517)+(v325*v1517));
        let v1530=((v326*v1518)+(v325*v1518));
        let v1531=(v1521/v570);
        let v1532=(v1524/v570);
        let v1533=(v1527/v570);
        let v1534=(v1530/v570);
        let v1539=(v211*v332);
        let v1540=((self.scalar_static_f64[243]*v1531)/v1539);
        let v1541=((self.scalar_static_f64[243]*v1532)/v1539);
        let v1542=((self.scalar_static_f64[243]*v1533)/v1539);
        let v1543=((self.scalar_static_f64[243]*v1534)/v1539);
        let v1544=(self.scalar_static_f64[225]*v1540);
        let v1545=(self.scalar_static_f64[225]*v1541);
        let v1546=(self.scalar_static_f64[225]*v1542);
        let v1547=(self.scalar_static_f64[225]*v1543);
        let v1548=(v1194-v1192);
        let v1549=(v1195-v1194);
        let v1550=(v1196-self.scalar_static_f64[169]);
        let v1551=(v25*v1548);
        let v1552=(v25*v1549);
        let v1553=(v25*v1550);
        let v1566=(self.scalar_static_f64[188]*(self.scalar_static_f64[5]*(v1531-(self.scalar_static_f64[183]*v1544))));
        let v1567=(self.scalar_static_f64[188]*(self.scalar_static_f64[5]*(v1532-(self.scalar_static_f64[183]*v1545))));
        let v1568=(self.scalar_static_f64[188]*(self.scalar_static_f64[5]*(v1533-(self.scalar_static_f64[183]*v1546))));
        let v1569=(self.scalar_static_f64[188]*(self.scalar_static_f64[5]*(v1534-(self.scalar_static_f64[183]*v1547))));
        let v1628=(v211*v357);
        let v1629=((self.scalar_static_f64[243]*(v1531-(v351*(v1521/v327))))/v1628);
        let v1630=((self.scalar_static_f64[243]*(v1532-(v351*(v1524/v327))))/v1628);
        let v1631=((self.scalar_static_f64[243]*(v1533-(v351*(v1527/v327))))/v1628);
        let v1632=((self.scalar_static_f64[243]*(v1534-(v351*(v1530/v327))))/v1628);
        let v1633=(self.scalar_static_f64[225]*v1629);
        let v1634=(self.scalar_static_f64[225]*v1630);
        let v1635=(self.scalar_static_f64[225]*v1631);
        let v1636=(self.scalar_static_f64[225]*v1632);
        let v1637=(v1551-v1633);
        let v1638=(-v1634);
        let v1639=(v1552-v1635);
        let v1640=(v1553-v1636);
        let v1641=(v360*v1633);
        let v1643=(v360*v1634);
        let v1645=(v360*v1635);
        let v1647=(v360*v1636);
        let v1653=(v211*v364);
        let v1654=((v1566+(v1641+v1641))/v1653);
        let v1655=((v1567+(v1643+v1643))/v1653);
        let v1656=((v1568+(v1645+v1645))/v1653);
        let v1657=((v1569+(v1647+v1647))/v1653);
        let v1658=(v361*v1637);
        let v1660=(v361*v1638);
        let v1662=(v361*v1639);
        let v1664=(v361*v1640);
        let v1670=(v211*v367);
        let v1671=((v1566+(v1658+v1658))/v1670);
        let v1672=((v1567+(v1660+v1660))/v1670);
        let v1673=((v1568+(v1662+v1662))/v1670);
        let v1674=((v1569+(v1664+v1664))/v1670);
        let v1689=(self.scalar_static_f64[183]*(v1671+(((v1334-v1551)-v1192)-v1654)));
        let v1690=(self.scalar_static_f64[183]*(v1672+(v1335-v1655)));
        let v1691=(self.scalar_static_f64[183]*(v1673+(((v1336-v1552)-v1194)-v1656)));
        let v1692=(self.scalar_static_f64[183]*(v1674+(((v1337-v1553)-self.scalar_static_f64[169])-v1657)));
        let v1703=(v378*v378);
        let v1714=(if (v374!=0.0){((-(v211*(v1689-(v1689/v376))))/v1703)}else{v1436});
        let v1715=(if (v374!=0.0){((-(v211*(v1690-(v1690/v376))))/v1703)}else{v1437});
        let v1716=(if (v374!=0.0){((-(v211*(v1691-(v1691/v376))))/v1703)}else{v1438});
        let v1717=(if (v374!=0.0){((-(v211*(v1692-(v1692/v376))))/v1703)}else{v1439});
        let v1729=(v384*v384);
        let v1743=(if (v374!=0.0){(((v384*v1714)-(v381*(v1689+(v1714/v380))))/v1729)}else{v1465});
        let v1744=(if (v374!=0.0){(((v384*v1715)-(v381*(v1690+(v1715/v380))))/v1729)}else{v1466});
        let v1745=(if (v374!=0.0){(((v384*v1716)-(v381*(v1691+(v1716/v380))))/v1729)}else{v1467});
        let v1746=(if (v374!=0.0){(((v384*v1717)-(v381*(v1692+(v1717/v380))))/v1729)}else{v1468});
        let v1758=(v389*v389);
        let v1780=(v397*(-v1689));
        let v1781=(v397*(-v1690));
        let v1782=(v397*(-v1691));
        let v1783=(v397*(-v1692));
        let v1784=(if v395{v1780}else{v1714});
        let v1785=(if v395{v1781}else{v1715});
        let v1786=(if v395{v1782}else{v1716});
        let v1787=(if v395{v1783}else{v1717});
        let v1799=(v402*v402);
        let v1813=(if v395{(((v402*v1784)-(v400*(v1689+(v1784/v399))))/v1799)}else{v1743});
        let v1814=(if v395{(((v402*v1785)-(v400*(v1690+(v1785/v399))))/v1799)}else{v1744});
        let v1815=(if v395{(((v402*v1786)-(v400*(v1691+(v1786/v399))))/v1799)}else{v1745});
        let v1816=(if v395{(((v402*v1787)-(v400*(v1692+(v1787/v399))))/v1799)}else{v1746});
        let v1828=(v407*v407);
        let v1847=(v415*v415);
        let v1863=(if v419{(v420*v1689)}else{(if v414{((-v1780)/v1847)}else{(if v395{(((v407*(v1689+(v1813/v404)))-(v406*v1813))/v1828)}else{(if (v374!=0.0){(((v389*(v1689+(v1743/v386)))-(v388*v1743))/v1758)}else{v1515})})})});
        let v1864=(if v419{(v420*v1690)}else{(if v414{((-v1781)/v1847)}else{(if v395{(((v407*(v1690+(v1814/v404)))-(v406*v1814))/v1828)}else{(if (v374!=0.0){(((v389*(v1690+(v1744/v386)))-(v388*v1744))/v1758)}else{v1516})})})});
        let v1865=(if v419{(v420*v1691)}else{(if v414{((-v1782)/v1847)}else{(if v395{(((v407*(v1691+(v1815/v404)))-(v406*v1815))/v1828)}else{(if (v374!=0.0){(((v389*(v1691+(v1745/v386)))-(v388*v1745))/v1758)}else{v1517})})})});
        let v1866=(if v419{(v420*v1692)}else{(if v414{((-v1783)/v1847)}else{(if v395{(((v407*(v1692+(v1816/v404)))-(v406*v1816))/v1828)}else{(if (v374!=0.0){(((v389*(v1692+(v1746/v386)))-(v388*v1746))/v1758)}else{v1518})})})});
        let v1934=(self.scalar_static_f64[183]*(v1334-v1194));
        let v1935=(self.scalar_static_f64[183]*(v1336-v1195));
        let v1936=(self.scalar_static_f64[183]*(v1337-v1196));
        let v1947=(v448*v448);
        let v1958=(if (v444!=0.0){((-(v211*(v1934-(v1934/v446))))/v1947)}else{v1784});
        let v1959=(if (v444!=0.0){((-(v211*(v1342-(v1342/v446))))/v1947)}else{v1785});
        let v1960=(if (v444!=0.0){((-(v211*(v1935-(v1935/v446))))/v1947)}else{v1786});
        let v1961=(if (v444!=0.0){((-(v211*(v1936-(v1936/v446))))/v1947)}else{v1787});
        let v1973=(v454*v454);
        let v1987=(if (v444!=0.0){(((v454*v1958)-(v451*(v1934+(v1958/v450))))/v1973)}else{v1813});
        let v1988=(if (v444!=0.0){(((v454*v1959)-(v451*(v1342+(v1959/v450))))/v1973)}else{v1814});
        let v1989=(if (v444!=0.0){(((v454*v1960)-(v451*(v1935+(v1960/v450))))/v1973)}else{v1815});
        let v1990=(if (v444!=0.0){(((v454*v1961)-(v451*(v1936+(v1961/v450))))/v1973)}else{v1816});
        let v2002=(v459*v459);
        let v2023=(v467*(-v1934));
        let v2024=(v467*v1429);
        let v2025=(v467*(-v1935));
        let v2026=(v467*(-v1936));
        let v2027=(if v465{v2023}else{v1958});
        let v2028=(if v465{v2024}else{v1959});
        let v2029=(if v465{v2025}else{v1960});
        let v2030=(if v465{v2026}else{v1961});
        let v2042=(v472*v472);
        let v2056=(if v465{(((v472*v2027)-(v470*(v1934+(v2027/v469))))/v2042)}else{v1987});
        let v2057=(if v465{(((v472*v2028)-(v470*(v1342+(v2028/v469))))/v2042)}else{v1988});
        let v2058=(if v465{(((v472*v2029)-(v470*(v1935+(v2029/v469))))/v2042)}else{v1989});
        let v2059=(if v465{(((v472*v2030)-(v470*(v1936+(v2030/v469))))/v2042)}else{v1990});
        let v2071=(v477*v477);
        let v2090=(v485*v485);
        let v2106=(if v489{(v490*v1934)}else{(if v484{((-v2023)/v2090)}else{(if v465{(((v477*(v1934+(v2056/v474)))-(v476*v2056))/v2071)}else{(if (v444!=0.0){(((v459*(v1934+(v1987/v456)))-(v458*v1987))/v2002)}else{v1863})})})});
        let v2107=(if v489{(v490*v1342)}else{(if v484{((-v2024)/v2090)}else{(if v465{(((v477*(v1342+(v2057/v474)))-(v476*v2057))/v2071)}else{(if (v444!=0.0){(((v459*(v1342+(v1988/v456)))-(v458*v1988))/v2002)}else{v1864})})})});
        let v2108=(if v489{(v490*v1935)}else{(if v484{((-v2025)/v2090)}else{(if v465{(((v477*(v1935+(v2058/v474)))-(v476*v2058))/v2071)}else{(if (v444!=0.0){(((v459*(v1935+(v1989/v456)))-(v458*v1989))/v2002)}else{v1865})})})});
        let v2109=(if v489{(v490*v1936)}else{(if v484{((-v2026)/v2090)}else{(if v465{(((v477*(v1936+(v2059/v474)))-(v476*v2059))/v2071)}else{(if (v444!=0.0){(((v459*(v1936+(v1990/v456)))-(v458*v1990))/v2002)}else{v1866})})})});
        let v2112=((v493*v2106)+(v492*v2106));
        let v2115=((v493*v2107)+(v492*v2107));
        let v2118=((v493*v2108)+(v492*v2108));
        let v2121=((v493*v2109)+(v492*v2109));
        let v2122=(v1521/v681);
        let v2123=(v1524/v681);
        let v2124=(v1527/v681);
        let v2125=(v1530/v681);
        let v2126=(v2112/v678);
        let v2127=(v2115/v678);
        let v2128=(v2118/v678);
        let v2129=(v2121/v678);
        let v2130=(v2122+v2126);
        let v2131=(v2123+v2127);
        let v2132=(v2124+v2128);
        let v2133=(v2125+v2129);
        let v2134=(v499*v2130);
        let v2135=(v2134+v2134);
        let v2136=(v499*v2131);
        let v2137=(v2136+v2136);
        let v2138=(v499*v2132);
        let v2139=(v2138+v2138);
        let v2140=(v499*v2133);
        let v2141=(v2140+v2140);
        let v2146=(v211*(v1334/v504));
        let v2147=(v211*(v1335/v504));
        let v2148=(v211*(v1336/v504));
        let v2149=(v211*(v1337/v504));
        let v3202=(v500*v500);
        let v3904=((v497*v1521)+(v495*v2122));
        let v3907=((v497*v1524)+(v495*v2123));
        let v3910=((v497*v1527)+(v495*v2124));
        let v3913=((v497*v1530)+(v495*v2125));
        let v3916=((v498*v2112)+(v496*v2126));
        let v3919=((v498*v2115)+(v496*v2127));
        let v3922=((v498*v2118)+(v496*v2128));
        let v3925=((v498*v2121)+(v496*v2129));
        let v3930=(v211*v795);
        let v3931=((v25*v1334)/v3930);
        let v3932=((v25*v1335)/v3930);
        let v3933=((v25*v1336)/v3930);
        let v3934=((v25*v1337)/v3930);
        let v3935=(v3931+v3931);
        let v3936=(v3932+v3932);
        let v3937=(v3933+v3933);
        let v3938=(v3934+v3934);
        let v3942=(v796*v796);
        let v3964=(-(self.scalar_static_f64[110]*(self.scalar_static_f64[181]*(((v796*v1287)-(v258*v3935))/v3942))));
        let v3965=(-(self.scalar_static_f64[110]*(self.scalar_static_f64[181]*(((v796*v1288)-(v258*v3936))/v3942))));
        let v3966=(-(self.scalar_static_f64[110]*(self.scalar_static_f64[181]*(((v796*v1289)-(v258*v3937))/v3942))));
        let v3967=(-(self.scalar_static_f64[110]*(self.scalar_static_f64[181]*(((v796*v1290)-(v258*v3938))/v3942))));
        let v4042=((v815*v3964)+(v801*(((v500*(v802*((((v9*v3916)+((v805*v2122)+(v497*(v804*v2112))))+((v808*v1521)+(v495*(v571*v2126))))+(v211*v3904))))-(v813*v2135))/v3202)));
        let v4045=((v815*v3965)+(v801*(((v500*(v802*((((v9*v3919)+((v805*v2123)+(v497*(v804*v2115))))+((v808*v1524)+(v495*(v571*v2127))))+(v211*v3907))))-(v813*v2137))/v3202)));
        let v4048=((v815*v3966)+(v801*(((v500*(v802*((((v9*v3922)+((v805*v2124)+(v497*(v804*v2118))))+((v808*v1527)+(v495*(v571*v2128))))+(v211*v3910))))-(v813*v2139))/v3202)));
        let v4051=((v815*v3967)+(v801*(((v500*(v802*((((v9*v3925)+((v805*v2125)+(v497*(v804*v2121))))+((v808*v1530)+(v495*(v571*v2129))))+(v211*v3913))))-(v813*v2141))/v3202)));
        let v4126=((v828*v3964)+(v801*(((v500*(v802*((((v9*v3904)+((v818*v2126)+(v498*(v804*v1521))))+((v821*v2112)+(v496*(v571*v2122))))+(v211*v3916))))-(v826*v2135))/v3202)));
        let v4129=((v828*v3965)+(v801*(((v500*(v802*((((v9*v3907)+((v818*v2127)+(v498*(v804*v1524))))+((v821*v2115)+(v496*(v571*v2123))))+(v211*v3919))))-(v826*v2137))/v3202)));
        let v4132=((v828*v3966)+(v801*(((v500*(v802*((((v9*v3910)+((v818*v2128)+(v498*(v804*v1527))))+((v821*v2118)+(v496*(v571*v2124))))+(v211*v3922))))-(v826*v2139))/v3202)));
        let v4135=((v828*v3967)+(v801*(((v500*(v802*((((v9*v3913)+((v818*v2129)+(v498*(v804*v1530))))+((v821*v2121)+(v496*(v571*v2125))))+(v211*v3925))))-(v826*v2141))/v3202)));
        let v4136=(v4042+v4126);
        let v4137=(v4045+v4129);
        let v4138=(v4048+v4132);
        let v4139=(v4051+v4135);
        let v4183=(v837*v837);
        let v4205=((-v4136)-((self.scalar_static_f64[110]*((v831*v2146)+(v504*(v520*v1287))))-(((v837*((v830*v1287)+(v258*v4136)))-(v836*(v1287+v3935)))/v4183)));
        let v4206=((-v4137)-((self.scalar_static_f64[110]*((v1206+((v831*v2147)+(v504*(v520*v1288))))-self.scalar_static_f64[20]))-(((v837*((v830*v1288)+(v258*v4137)))-(v836*(v1288+v3936)))/v4183)));
        let v4207=((-v4138)-((self.scalar_static_f64[110]*((v831*v2148)+(v504*(v520*v1289))))-(((v837*((v830*v1289)+(v258*v4138)))-(v836*(v1289+v3937)))/v4183)));
        let v4208=((-v4139)-((self.scalar_static_f64[110]*((v1207+((v831*v2149)+(v504*(v520*v1290))))-self.scalar_static_f64[169]))-(((v837*((v830*v1290)+(v258*v4139)))-(v836*(v1290+v3938)))/v4183)));
        let v4422=((self.scalar_static_f64[20]*v1106)+(v195*((if v1089{self.scalar_static_f64[344]}else{(if (v1058!=0.0){(self.scalar_static_f64[301]*(v1086*(self.scalar_static_f64[167]*(self.scalar_static_f64[326]/v1083))))}else{v1})})+((if v1089{self.scalar_static_f64[332]}else{(if (v1058!=0.0){(self.scalar_static_f64[299]*(v1066*(self.scalar_static_f64[163]*(self.scalar_static_f64[322]/v1063))))}else{v1})})+(if v1089{self.scalar_static_f64[338]}else{(if (v1058!=0.0){(self.scalar_static_f64[300]*(v1076*(self.scalar_static_f64[165]*(self.scalar_static_f64[324]/v1073))))}else{v1})})))));
        let v4425=((v1106*self.scalar_static_f64[169])+(v195*((if v1089{self.scalar_static_f64[345]}else{(if (v1058!=0.0){(self.scalar_static_f64[301]*(v1086*(self.scalar_static_f64[167]*(self.scalar_static_f64[327]/v1083))))}else{v1})})+((if v1089{self.scalar_static_f64[333]}else{(if (v1058!=0.0){(self.scalar_static_f64[299]*(v1066*(self.scalar_static_f64[163]*(self.scalar_static_f64[323]/v1063))))}else{v1})})+(if v1089{self.scalar_static_f64[339]}else{(if (v1058!=0.0){(self.scalar_static_f64[300]*(v1076*(self.scalar_static_f64[165]*(self.scalar_static_f64[325]/v1073))))}else{v1})})))));
        let v4472=((self.scalar_static_f64[20]*v1150)+(v192*((if v1133{self.scalar_static_f64[344]}else{(if (v1109!=0.0){(self.scalar_static_f64[301]*(v1130*(self.scalar_static_f64[167]*(self.scalar_static_f64[326]/v1127))))}else{v1})})+((if v1133{self.scalar_static_f64[346]}else{(if (v1109!=0.0){(self.scalar_static_f64[302]*(v1115*(self.scalar_static_f64[163]*(self.scalar_static_f64[322]/v1112))))}else{v1})})+(if v1133{self.scalar_static_f64[348]}else{(if (v1109!=0.0){(self.scalar_static_f64[303]*(v1123*(self.scalar_static_f64[165]*(self.scalar_static_f64[324]/v1120))))}else{v1})})))));
        let v4475=((v1150*self.scalar_static_f64[169])+(v192*((if v1133{self.scalar_static_f64[345]}else{(if (v1109!=0.0){(self.scalar_static_f64[301]*(v1130*(self.scalar_static_f64[167]*(self.scalar_static_f64[327]/v1127))))}else{v1})})+((if v1133{self.scalar_static_f64[347]}else{(if (v1109!=0.0){(self.scalar_static_f64[302]*(v1115*(self.scalar_static_f64[163]*(self.scalar_static_f64[323]/v1112))))}else{v1})})+(if v1133{self.scalar_static_f64[349]}else{(if (v1109!=0.0){(self.scalar_static_f64[303]*(v1123*(self.scalar_static_f64[165]*(self.scalar_static_f64[325]/v1120))))}else{v1})})))));

        CommonStampValues {
            v1, v25, v31, v192, v195, v205, v211, v220, 
            v223, v227, v230, v256, v258, v262, v266, v325, 
            v327, v328, v332, v334, v335, v336, v342, v357, 
            v360, v361, v364, v367, v422, v492, v495, v496, 
            v497, v498, v499, v500, v501, v502, v504, v520, 
            v571, v678, v681, v816, v829, v841, v845, v1107, 
            v1151, v1160, v1215, v1216, v1217, v1225, v1226, v1227, 
            v1235, v1236, v1237, v1245, v1246, v1247, v1279, v1280, 
            v1281, v1282, v1287, v1288, v1289, v1290, v1310, v1311, 
            v1312, v1313, v1334, v1335, v1336, v1337, v1515, v1516, 
            v1517, v1518, v1521, v1524, v1527, v1530, v1531, v1532, 
            v1533, v1534, v1540, v1541, v1542, v1543, v1544, v1545, 
            v1546, v1547, v1548, v1549, v1550, v1551, v1552, v1553, 
            v1566, v1567, v1568, v1569, v1629, v1630, v1631, v1632, 
            v1633, v1634, v1635, v1636, v1637, v1638, v1639, v1640, 
            v1654, v1655, v1656, v1657, v1671, v1672, v1673, v1674, 
            v1863, v1864, v1865, v1866, v2106, v2107, v2108, v2109, 
            v2112, v2115, v2118, v2121, v2122, v2123, v2124, v2125, 
            v2126, v2127, v2128, v2129, v2130, v2131, v2132, v2133, 
            v2135, v2137, v2139, v2141, v2146, v2147, v2148, v2149, 
            v3202, v4042, v4045, v4048, v4051, v4126, v4129, v4132, 
            v4135, v4205, v4206, v4207, v4208, v4422, v4425, v4472, 
            v4475, 
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
        let v345=((common.v342+(common.v334*common.v334))).sqrt();
        let v346=(common.v336-common.v334);
        let v349=((common.v342+(v346*v346))).sqrt();
        let v350=(v345-v349);
        let v423=(common.v31+common.v422);
        let v427=(common.v31+((common.v336-v350)/self.scalar_static_f64[222]));
        let v433=((self.scalar_static_f64[48]-(self.scalar_static_f64[6]*(v427).ln()))+(self.scalar_static_f64[221]*(common.v336+v350)));
        let v438=(((v433*v433)+self.scalar_static_f64[99])).sqrt();
        let v440=(common.v25*(v433+v438));
        let v505=(self.scalar_static_f64[70]/common.v504);
        let v506=(self.scalar_static_f64[70]+common.v504);
        let v507=(self.scalar_static_f64[70]/v506);
        let v508=(common.v31+v505);
        let v510=(self.scalar_static_f64[181]*(-v508));
        let v511=0.66666666;
        let v512=1.33333332;
        let v516=(v512*(common.v495+(common.v496+(common.v497*common.v498))));
        let v518=((v516/common.v499)-common.v31);
        let v519=(v510*v518);
        let v529=((self.scalar_static_f64[187]+(common.v266*common.v266))).sqrt();
        let v530=(if (self.scalar_static_f64[101]!=0.0){v529}else{common.v1});
        let v535=((if (self.scalar_static_f64[101]!=0.0){(common.v25*(common.v266+v530))}else{common.v1})*self.scalar_static_f64[102]);
        let v537=(if (self.scalar_static_f64[101]!=0.0){(common.v31+v535)}else{common.v1});
        let v538=(v440*v537);
        let v542=(((common.v504*self.scalar_static_f64[100])-(v507*v519))+(self.scalar_static_f64[21]*v519));
        let v544=(if (v542>common.v1){common.v31}else{common.v1});
        let v546=((v544!=0.0)&&self.scalar_static_bool[12]);
        let v547=(self.scalar_static_f64[16]*v542);
        let v551=(self.scalar_static_bool[12]&&(!(v544!=0.0)));
        let v553=(if v551{(common.v31-v547)}else{(if v546{(common.v31+v547)}else{common.v1})});
        let v558=(v440*v553);
        let v560=(if self.scalar_static_bool[12]{(self.scalar_static_f64[247]/v558)}else{(if (self.scalar_static_f64[101]!=0.0){(self.scalar_static_f64[240]/v538)}else{common.v1})});
        let v561=(self.scalar_static_f64[185]+common.v501);
        let v562=(v561).sqrt();
        let v563=(common.v211*v562);
        let v565=(common.v31+(self.scalar_static_f64[70]/v563));
        let v566=(common.v327-(common.v422*v423));
        let v567=(self.scalar_static_f64[187]*v565);
        let v568=(v560*v567);
        let v569=(v566*v568);
        let v572=(common.v256+common.v256);
        let v575=((common.v258/v572)*self.scalar_static_f64[103]);
        let v576=(common.v230*v575);
        let v577=(v576/common.v227);
        let v578=(common.v223*v575);
        let v579=(v578/common.v220);
        let v581=(-(common.v501/common.v262));
        let v582=(v577*v581);
        let v583=(v579*v581);
        let v584=(self.scalar_static_f64[183]*common.v325);
        let v585=(v582*v584);
        let v586=(v583-common.v31);
        let v587=(v584*v586);
        let v588=(common.v332*common.v571);
        let v589=(common.v328*v588);
        let v590=(self.scalar_static_f64[181]/v589);
        let v591=(v585*v590);
        let v592=(v587*v590);
        let v595=(common.v328+common.v328);
        let v596=(self.scalar_static_f64[181]/v595);
        let v599=(self.scalar_static_f64[249]*((v585*v596)-v591));
        let v602=(self.scalar_static_f64[249]*((v587*v596)-v592));
        let v603=(common.v31/v345);
        let v604=(common.v31/v349);
        let v606=(v599+(common.v334*v591));
        let v608=(common.v25-v591);
        let v610=(v599+(v346*v608));
        let v612=((v603*v606)-(v604*v610));
        let v614=(v602+(common.v334*v592));
        let v616=(common.v520-v592);
        let v618=(v602+(v346*v616));
        let v620=((v603*v614)-(v604*v618));
        let v623=(self.scalar_static_f64[181]*(common.v328-1.5));
        let v624=(common.v357*common.v571);
        let v625=(common.v327*v624);
        let v626=(v623/v625);
        let v627=(v585*v626);
        let v628=(v587*v626);
        let v629=(self.scalar_static_f64[183]*common.v422);
        let v630=(common.v31/common.v364);
        let v631=(common.v31/common.v367);
        let v634=(v599+(common.v360*v627));
        let v637=(common.v25-v627);
        let v639=(v599+(common.v361*v637));
        let v641=(((v582-common.v25)-(v630*v634))+(v631*v639));
        let v645=(v602+(common.v360*v628));
        let v648=(common.v520-v628);
        let v650=(v602+(common.v361*v648));
        let v652=(((v583-common.v25)-(v630*v645))+(v631*v650));
        let v655=((self.scalar_static_f64[222]+common.v336)-v350);
        let v656=(self.scalar_static_f64[6]/v655);
        let v657=(common.v25-v612);
        let v659=(common.v520-v620);
        let v661=(common.v31/v438);
        let v665=((-(v656*v657))+(self.scalar_static_f64[221]*(common.v25+v612)));
        let v670=((-(v656*v659))+(self.scalar_static_f64[221]*(common.v520+v620)));
        let v672=(self.scalar_static_f64[183]*common.v492);
        let v673=(v582-common.v31);
        let v674=(v672*v673);
        let v675=(v583*v672);
        let v676=(v510*v511);
        let v677=(v676/common.v500);
        let v679=(common.v497+common.v678);
        let v680=(v677*v679);
        let v682=(common.v498+common.v681);
        let v683=(v677*v682);
        let v684=(-v505);
        let v685=(v519*v684);
        let v687=(v505+(common.v211+v505));
        let v688=(common.v502*v687);
        let v689=(v685/v688);
        let v694=(((v582*v689)+(v585*v680))+(v674*v683));
        let v699=(((v583*v689)+(v587*v680))+(v675*v683));
        let v700=(common.v211*v508);
        let v701=(common.v502*v700);
        let v703=(v508-(v519/v701));
        let v704=(-v507);
        let v706=(v694+(v582*v703));
        let v709=(v699+(v583*v703));
        let v711=(v530*v537);
        let v713=(if (self.scalar_static_f64[101]!=0.0){(v535/v711)}else{v703});
        let v718=(-(v661*v665));
        let v721=(-(v661*v670));
        let v725=(if self.scalar_static_bool[12]{(self.scalar_static_f64[16]/v553)}else{v713});
        let v727=((v704*v706)+(self.scalar_static_f64[21]*v694));
        let v732=((v704*v709)+(self.scalar_static_f64[21]*v699));
        let v737=(v565*common.v571);
        let v738=(v562*v737);
        let v739=(v561*v738);
        let v740=(self.scalar_static_f64[104]/v739);
        let v743=((if self.scalar_static_bool[12]{(v718+(v725*v727))}else{(if (self.scalar_static_f64[101]!=0.0){(v718-(if (self.scalar_static_f64[101]!=0.0){(v582*v713)}else{common.v1}))}else{common.v1})})+(v582*v740));
        let v746=((v585+(v566*v743))-(v629*v641));
        let v748=(-v568);
        let v749=((if self.scalar_static_bool[12]{(v721+(v725*v732))}else{(if (self.scalar_static_f64[101]!=0.0){(v721-(if (self.scalar_static_f64[101]!=0.0){(v583*v713)}else{common.v1}))}else{common.v1})})+(v583*v740));
        let v752=((v587+(v566*v749))-(v629*v652));
        let v762=((common.v31+((v748*v752)*self.scalar_static_f64[109]))+((v568*v746)*self.scalar_static_f64[109]));
        let v763=(common.v31/v762);
        let v764=(v569*v763);
        let v766=(common.v335-(self.scalar_static_f64[13]*common.v334));
        let v770=(if ((v766>common.v1)&&self.scalar_static_bool[26]){common.v31}else{common.v1});
        let v775=(if (v770!=0.0){((if (v770!=0.0){(common.v31/v766)}else{common.v1})*self.scalar_static_f64[250])}else{common.v1});
        let v776=-35.0;
        let v779=((v770!=0.0)&&((if (v775<v776){common.v31}else{common.v1})!=0.0));
        let v781=((if v779{v776}else{v775})).exp();
        let v782=(if (v770!=0.0){v781}else{common.v1});
        let v783=(self.scalar_static_f64[224]*v766);
        let v785=(if (v770!=0.0){(v782*v783)}else{common.v1});
        let v788=(!(v770!=0.0));
        let v842=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v816);
        let v843=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v829);
        let v943=(-common.v195);
        let v946=((self.scalar_static_f64[195]*v943)/self.scalar_static_f64[290]);
        let v947=-40.0;
        let v949=(if (v946<v947){common.v31}else{common.v1});
        let v954=((self.scalar_static_f64[195]*(v943+self.scalar_static_f64[156]))/self.scalar_static_f64[290]);
        let v955=70.0;
        let v957=(if (v954>v955){common.v31}else{common.v1});
        let v959=(!(v957!=0.0));
        let v962=((-v954)).exp();
        let v965=(if v959{(common.v31+(self.scalar_static_f64[157]*v962))}else{(if (v957!=0.0){common.v31}else{common.v1})});
        let v968=(self.scalar_static_f64[195]*common.v195);
        let v972=((v968/self.scalar_static_f64[292])*self.scalar_static_f64[159]);
        let v973=(common.v195+self.scalar_static_f64[159]);
        let v974=0.001;
        let v975=(v973>v974);
        let v976=(if v975{v973}else{v974});
        let v978=((v972/v976)).exp();
        let v984=((v968/self.scalar_static_f64[293])*self.scalar_static_f64[160]);
        let v985=(common.v195+self.scalar_static_f64[160]);
        let v986=(v985>v974);
        let v987=(if v986{v985}else{v974});
        let v989=((v984/v987)).exp();
        let v996=((v968/self.scalar_static_f64[294])*self.scalar_static_f64[161]);
        let v997=(common.v195+self.scalar_static_f64[161]);
        let v998=(v997>v974);
        let v999=(if v998{v997}else{v974});
        let v1001=((v996/v999)).exp();
        let v1009=(-common.v192);
        let v1011=((self.scalar_static_f64[195]*v1009)/self.scalar_static_f64[290]);
        let v1013=(if (v1011<v947){common.v31}else{common.v1});
        let v1017=((self.scalar_static_f64[195]*(self.scalar_static_f64[156]+v1009))/self.scalar_static_f64[290]);
        let v1019=(if (v1017>v955){common.v31}else{common.v1});
        let v1021=(!(v1019!=0.0));
        let v1023=((-v1017)).exp();
        let v1026=(if v1021{(common.v31+(self.scalar_static_f64[157]*v1023))}else{(if (v1019!=0.0){common.v31}else{common.v1})});
        let v1027=(self.scalar_static_f64[195]*common.v192);
        let v1029=(self.scalar_static_f64[159]*(v1027/self.scalar_static_f64[292]));
        let v1030=(common.v192+self.scalar_static_f64[159]);
        let v1031=(v1030>v974);
        let v1032=(if v1031{v1030}else{v974});
        let v1034=((v1029/v1032)).exp();
        let v1038=(self.scalar_static_f64[160]*(v1027/self.scalar_static_f64[293]));
        let v1039=(common.v192+self.scalar_static_f64[160]);
        let v1040=(v1039>v974);
        let v1041=(if v1040{v1039}else{v974});
        let v1043=((v1038/v1041)).exp();
        let v1048=(self.scalar_static_f64[161]*(v1027/self.scalar_static_f64[294]));
        let v1049=(common.v192+self.scalar_static_f64[161]);
        let v1050=(v1049>v974);
        let v1051=(if v1050{v1049}else{v974});
        let v1053=((v1048/v1051)).exp();
        let v1152=(self.scalar_static_f64[20]*common.v205);
        let v1154=(self.scalar_static_f64[20]*v842);
        let v1156=(self.scalar_static_f64[20]*v843);
        let v1158=(self.scalar_static_f64[20]*(if v788{common.v1}else{(if (v770!=0.0){(v764*v785)}else{common.v1})}));
        let v1164=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v841);
        let v1166=((if (v949!=0.0){v947}else{v946})).exp();
        let v1168=(self.scalar_static_f64[289]*(common.v31-v1166));
        let v1176=((if (v1013!=0.0){v947}else{v1011})).exp();
        let v1178=(self.scalar_static_f64[298]*(common.v31-v1176));
        let v1185=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v1107);
        let v1188=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v1151);
        let v1570=(common.v334*common.v1544);
        let v1572=(common.v334*common.v1545);
        let v1574=(common.v334*common.v1546);
        let v1576=(common.v334*common.v1547);
        let v1582=(common.v211*v345);
        let v1583=((common.v1566+(v1570+v1570))/v1582);
        let v1584=((common.v1567+(v1572+v1572))/v1582);
        let v1585=((common.v1568+(v1574+v1574))/v1582);
        let v1586=((common.v1569+(v1576+v1576))/v1582);
        let v1587=(common.v1551-common.v1544);
        let v1588=(-common.v1545);
        let v1589=(common.v1552-common.v1546);
        let v1590=(common.v1553-common.v1547);
        let v1591=(v346*v1587);
        let v1593=(v346*v1588);
        let v1595=(v346*v1589);
        let v1597=(v346*v1590);
        let v1603=(common.v211*v349);
        let v1604=((common.v1566+(v1591+v1591))/v1603);
        let v1605=((common.v1567+(v1593+v1593))/v1603);
        let v1606=((common.v1568+(v1595+v1595))/v1603);
        let v1607=((common.v1569+(v1597+v1597))/v1603);
        let v1608=(v1583-v1604);
        let v1609=(v1584-v1605);
        let v1610=(v1585-v1606);
        let v1611=(v1586-v1607);
        let v1879=(common.v1551-v1608);
        let v1880=(-v1609);
        let v1881=(common.v1552-v1610);
        let v1882=(common.v1553-v1611);
        let v1906=((-(self.scalar_static_f64[6]*((v1879/self.scalar_static_f64[222])/v427)))+(self.scalar_static_f64[221]*(common.v1551+v1608)));
        let v1907=((-(self.scalar_static_f64[6]*((v1880/self.scalar_static_f64[222])/v427)))+(self.scalar_static_f64[221]*v1609));
        let v1908=((-(self.scalar_static_f64[6]*((v1881/self.scalar_static_f64[222])/v427)))+(self.scalar_static_f64[221]*(common.v1552+v1610)));
        let v1909=((-(self.scalar_static_f64[6]*((v1882/self.scalar_static_f64[222])/v427)))+(self.scalar_static_f64[221]*(common.v1553+v1611)));
        let v1910=(v433*v1906);
        let v1912=(v433*v1907);
        let v1914=(v433*v1908);
        let v1916=(v433*v1909);
        let v1918=(common.v211*v438);
        let v1919=((v1910+v1910)/v1918);
        let v1920=((v1912+v1912)/v1918);
        let v1921=((v1914+v1914)/v1918);
        let v1922=((v1916+v1916)/v1918);
        let v1927=(common.v25*(v1906+v1919));
        let v1928=(common.v25*(v1907+v1920));
        let v1929=(common.v25*(v1908+v1921));
        let v1930=(common.v25*(v1909+v1922));
        let v2151=(-(self.scalar_static_f64[70]*common.v2146));
        let v2152=(common.v504*common.v504);
        let v2153=(v2151/v2152);
        let v2155=(-(self.scalar_static_f64[70]*common.v2147));
        let v2156=(v2155/v2152);
        let v2158=(-(self.scalar_static_f64[70]*common.v2148));
        let v2159=(v2158/v2152);
        let v2161=(-(self.scalar_static_f64[70]*common.v2149));
        let v2162=(v2161/v2152);
        let v2163=(v506*v506);
        let v2164=(v2151/v2163);
        let v2165=(v2155/v2163);
        let v2166=(v2158/v2163);
        let v2167=(v2161/v2163);
        let v2168=(-v2153);
        let v2169=(-v2156);
        let v2170=(-v2159);
        let v2171=(-v2162);
        let v2172=(self.scalar_static_f64[181]*v2168);
        let v2173=(self.scalar_static_f64[181]*v2169);
        let v2174=(self.scalar_static_f64[181]*v2170);
        let v2175=(self.scalar_static_f64[181]*v2171);
        let v2218=((v518*v2172)+(v510*(((common.v499*(v512*(common.v1521+(common.v2112+((common.v498*common.v2122)+(common.v497*common.v2126))))))-(v516*common.v2130))/common.v500)));
        let v2221=((v518*v2173)+(v510*(((common.v499*(v512*(common.v1524+(common.v2115+((common.v498*common.v2123)+(common.v497*common.v2127))))))-(v516*common.v2131))/common.v500)));
        let v2224=((v518*v2174)+(v510*(((common.v499*(v512*(common.v1527+(common.v2118+((common.v498*common.v2124)+(common.v497*common.v2128))))))-(v516*common.v2132))/common.v500)));
        let v2227=((v518*v2175)+(v510*(((common.v499*(v512*(common.v1530+(common.v2121+((common.v498*common.v2125)+(common.v497*common.v2129))))))-(v516*common.v2133))/common.v500)));
        let v2248=(common.v266*common.v1334);
        let v2250=(common.v266*common.v1335);
        let v2252=(common.v266*common.v1336);
        let v2254=(common.v266*common.v1337);
        let v2256=(common.v211*v529);
        let v2261=(if (self.scalar_static_f64[101]!=0.0){((v2248+v2248)/v2256)}else{common.v1});
        let v2262=(if (self.scalar_static_f64[101]!=0.0){((v2250+v2250)/v2256)}else{common.v1});
        let v2263=(if (self.scalar_static_f64[101]!=0.0){((v2252+v2252)/v2256)}else{common.v1});
        let v2264=(if (self.scalar_static_f64[101]!=0.0){((v2254+v2254)/v2256)}else{common.v1});
        let v2277=(self.scalar_static_f64[102]*(if (self.scalar_static_f64[101]!=0.0){(common.v25*(common.v1334+v2261))}else{common.v1}));
        let v2278=(self.scalar_static_f64[102]*(if (self.scalar_static_f64[101]!=0.0){(common.v25*(common.v1335+v2262))}else{common.v1}));
        let v2279=(self.scalar_static_f64[102]*(if (self.scalar_static_f64[101]!=0.0){(common.v25*(common.v1336+v2263))}else{common.v1}));
        let v2280=(self.scalar_static_f64[102]*(if (self.scalar_static_f64[101]!=0.0){(common.v25*(common.v1337+v2264))}else{common.v1}));
        let v2281=(if (self.scalar_static_f64[101]!=0.0){v2277}else{common.v1});
        let v2282=(if (self.scalar_static_f64[101]!=0.0){v2278}else{common.v1});
        let v2283=(if (self.scalar_static_f64[101]!=0.0){v2279}else{common.v1});
        let v2284=(if (self.scalar_static_f64[101]!=0.0){v2280}else{common.v1});
        let v2299=(v538*v538);
        let v2322=(self.scalar_static_f64[16]*(((self.scalar_static_f64[100]*common.v2146)-((v519*v2164)+(v507*v2218)))+(self.scalar_static_f64[21]*v2218)));
        let v2323=(self.scalar_static_f64[16]*(((self.scalar_static_f64[100]*common.v2147)-((v519*v2165)+(v507*v2221)))+(self.scalar_static_f64[21]*v2221)));
        let v2324=(self.scalar_static_f64[16]*(((self.scalar_static_f64[100]*common.v2148)-((v519*v2166)+(v507*v2224)))+(self.scalar_static_f64[21]*v2224)));
        let v2325=(self.scalar_static_f64[16]*(((self.scalar_static_f64[100]*common.v2149)-((v519*v2167)+(v507*v2227)))+(self.scalar_static_f64[21]*v2227)));
        let v2334=(if v551{(-v2322)}else{(if v546{v2322}else{common.v1})});
        let v2335=(if v551{(-v2323)}else{(if v546{v2323}else{common.v1})});
        let v2336=(if v551{(-v2324)}else{(if v546{v2324}else{common.v1})});
        let v2337=(if v551{(-v2325)}else{(if v546{v2325}else{common.v1})});
        let v2352=(v558*v558);
        let v2367=(common.v1334/v563);
        let v2368=(common.v1335/v563);
        let v2369=(common.v1336/v563);
        let v2370=(common.v1337/v563);
        let v2377=(v563*v563);
        let v2378=((-(self.scalar_static_f64[70]*(common.v211*v2367)))/v2377);
        let v2381=((-(self.scalar_static_f64[70]*(common.v211*v2368)))/v2377);
        let v2384=((-(self.scalar_static_f64[70]*(common.v211*v2369)))/v2377);
        let v2387=((-(self.scalar_static_f64[70]*(common.v211*v2370)))/v2377);
        let v2388=(common.v1521-((v423*common.v1863)+(common.v422*common.v1863)));
        let v2389=(common.v1524-((v423*common.v1864)+(common.v422*common.v1864)));
        let v2390=(common.v1527-((v423*common.v1865)+(common.v422*common.v1865)));
        let v2391=(common.v1530-((v423*common.v1866)+(common.v422*common.v1866)));
        let v2398=((v567*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[247]*((v553*v1927)+(v440*v2334))))/v2352)}else{(if (self.scalar_static_f64[101]!=0.0){((-(self.scalar_static_f64[240]*((v537*v1927)+(v440*v2281))))/v2299)}else{common.v1})}))+(v560*(self.scalar_static_f64[187]*v2378)));
        let v2401=((v567*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[247]*((v553*v1928)+(v440*v2335))))/v2352)}else{(if (self.scalar_static_f64[101]!=0.0){((-(self.scalar_static_f64[240]*((v537*v1928)+(v440*v2282))))/v2299)}else{common.v1})}))+(v560*(self.scalar_static_f64[187]*v2381)));
        let v2404=((v567*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[247]*((v553*v1929)+(v440*v2336))))/v2352)}else{(if (self.scalar_static_f64[101]!=0.0){((-(self.scalar_static_f64[240]*((v537*v1929)+(v440*v2283))))/v2299)}else{common.v1})}))+(v560*(self.scalar_static_f64[187]*v2384)));
        let v2407=((v567*(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[247]*((v553*v1930)+(v440*v2337))))/v2352)}else{(if (self.scalar_static_f64[101]!=0.0){((-(self.scalar_static_f64[240]*((v537*v1930)+(v440*v2284))))/v2299)}else{common.v1})}))+(v560*(self.scalar_static_f64[187]*v2387)));
        let v2427=(v572*v572);
        let v2441=(self.scalar_static_f64[103]*(((v572*common.v1287)-(common.v258*(common.v1279+common.v1279)))/v2427));
        let v2442=(self.scalar_static_f64[103]*(((v572*common.v1288)-(common.v258*(common.v1280+common.v1280)))/v2427));
        let v2443=(self.scalar_static_f64[103]*(((v572*common.v1289)-(common.v258*(common.v1281+common.v1281)))/v2427));
        let v2444=(self.scalar_static_f64[103]*(((v572*common.v1290)-(common.v258*(common.v1282+common.v1282)))/v2427));
        let v2458=(common.v227*common.v227);
        let v2482=(common.v220*common.v220);
        let v2496=(common.v262*common.v262);
        let v2510=(-(((common.v262*common.v1334)-(common.v501*common.v1310))/v2496));
        let v2511=(-(((common.v262*common.v1335)-(common.v501*common.v1311))/v2496));
        let v2512=(-(((common.v262*common.v1336)-(common.v501*common.v1312))/v2496));
        let v2513=(-(((common.v262*common.v1337)-(common.v501*common.v1313))/v2496));
        let v2516=((v581*(((common.v227*((v575*common.v1245)+(common.v230*v2441)))-(v576*common.v1235))/v2458))+(v577*v2510));
        let v2519=((v581*((common.v230*v2442)/common.v227))+(v577*v2511));
        let v2522=((v581*(((common.v227*((v575*common.v1246)+(common.v230*v2443)))-(v576*common.v1236))/v2458))+(v577*v2512));
        let v2525=((v581*(((common.v227*((v575*common.v1247)+(common.v230*v2444)))-(v576*common.v1237))/v2458))+(v577*v2513));
        let v2528=((v581*(((common.v220*((v575*common.v1225)+(common.v223*v2441)))-(v578*common.v1215))/v2482))+(v579*v2510));
        let v2531=((v581*((common.v223*v2442)/common.v220))+(v579*v2511));
        let v2534=((v581*(((common.v220*((v575*common.v1226)+(common.v223*v2443)))-(v578*common.v1216))/v2482))+(v579*v2512));
        let v2537=((v581*(((common.v220*((v575*common.v1227)+(common.v223*v2444)))-(v578*common.v1217))/v2482))+(v579*v2513));
        let v2538=(self.scalar_static_f64[183]*common.v1515);
        let v2539=(self.scalar_static_f64[183]*common.v1516);
        let v2540=(self.scalar_static_f64[183]*common.v1517);
        let v2541=(self.scalar_static_f64[183]*common.v1518);
        let v2544=((v584*v2516)+(v582*v2538));
        let v2547=((v584*v2519)+(v582*v2539));
        let v2550=((v584*v2522)+(v582*v2540));
        let v2553=((v584*v2525)+(v582*v2541));
        let v2556=((v586*v2538)+(v584*v2528));
        let v2559=((v586*v2539)+(v584*v2531));
        let v2562=((v586*v2540)+(v584*v2534));
        let v2565=((v586*v2541)+(v584*v2537));
        let v2584=(v589*v589);
        let v2585=((-(self.scalar_static_f64[181]*((v588*common.v1531)+(common.v328*(common.v571*common.v1540)))))/v2584);
        let v2588=((-(self.scalar_static_f64[181]*((v588*common.v1532)+(common.v328*(common.v571*common.v1541)))))/v2584);
        let v2591=((-(self.scalar_static_f64[181]*((v588*common.v1533)+(common.v328*(common.v571*common.v1542)))))/v2584);
        let v2594=((-(self.scalar_static_f64[181]*((v588*common.v1534)+(common.v328*(common.v571*common.v1543)))))/v2584);
        let v2597=((v590*v2544)+(v585*v2585));
        let v2600=((v590*v2547)+(v585*v2588));
        let v2603=((v590*v2550)+(v585*v2591));
        let v2606=((v590*v2553)+(v585*v2594));
        let v2609=((v590*v2556)+(v587*v2585));
        let v2612=((v590*v2559)+(v587*v2588));
        let v2615=((v590*v2562)+(v587*v2591));
        let v2618=((v590*v2565)+(v587*v2594));
        let v2625=(v595*v595);
        let v2626=((-(self.scalar_static_f64[181]*(common.v1531+common.v1531)))/v2625);
        let v2629=((-(self.scalar_static_f64[181]*(common.v1532+common.v1532)))/v2625);
        let v2632=((-(self.scalar_static_f64[181]*(common.v1533+common.v1533)))/v2625);
        let v2635=((-(self.scalar_static_f64[181]*(common.v1534+common.v1534)))/v2625);
        let v2652=(self.scalar_static_f64[249]*(((v596*v2544)+(v585*v2626))-v2597));
        let v2653=(self.scalar_static_f64[249]*(((v596*v2547)+(v585*v2629))-v2600));
        let v2654=(self.scalar_static_f64[249]*(((v596*v2550)+(v585*v2632))-v2603));
        let v2655=(self.scalar_static_f64[249]*(((v596*v2553)+(v585*v2635))-v2606));
        let v2672=(self.scalar_static_f64[249]*(((v596*v2556)+(v587*v2626))-v2609));
        let v2673=(self.scalar_static_f64[249]*(((v596*v2559)+(v587*v2629))-v2612));
        let v2674=(self.scalar_static_f64[249]*(((v596*v2562)+(v587*v2632))-v2615));
        let v2675=(self.scalar_static_f64[249]*(((v596*v2565)+(v587*v2635))-v2618));
        let v2677=(v345*v345);
        let v2678=((-v1583)/v2677);
        let v2680=((-v1584)/v2677);
        let v2682=((-v1585)/v2677);
        let v2684=((-v1586)/v2677);
        let v2686=(v349*v349);
        let v2687=((-v1604)/v2686);
        let v2689=((-v1605)/v2686);
        let v2691=((-v1606)/v2686);
        let v2693=((-v1607)/v2686);
        let v2754=(((v606*v2678)+(v603*(v2652+((v591*common.v1544)+(common.v334*v2597)))))-((v610*v2687)+(v604*(v2652+((v608*v1587)+(v346*(-v2597)))))));
        let v2755=(((v606*v2680)+(v603*(v2653+((v591*common.v1545)+(common.v334*v2600)))))-((v610*v2689)+(v604*(v2653+((v608*v1588)+(v346*(-v2600)))))));
        let v2756=(((v606*v2682)+(v603*(v2654+((v591*common.v1546)+(common.v334*v2603)))))-((v610*v2691)+(v604*(v2654+((v608*v1589)+(v346*(-v2603)))))));
        let v2757=(((v606*v2684)+(v603*(v2655+((v591*common.v1547)+(common.v334*v2606)))))-((v610*v2693)+(v604*(v2655+((v608*v1590)+(v346*(-v2606)))))));
        let v2818=(((v614*v2678)+(v603*(v2672+((v592*common.v1544)+(common.v334*v2609)))))-((v618*v2687)+(v604*(v2672+((v616*v1587)+(v346*(-v2609)))))));
        let v2819=(((v614*v2680)+(v603*(v2673+((v592*common.v1545)+(common.v334*v2612)))))-((v618*v2689)+(v604*(v2673+((v616*v1588)+(v346*(-v2612)))))));
        let v2820=(((v614*v2682)+(v603*(v2674+((v592*common.v1546)+(common.v334*v2615)))))-((v618*v2691)+(v604*(v2674+((v616*v1589)+(v346*(-v2615)))))));
        let v2821=(((v614*v2684)+(v603*(v2675+((v592*common.v1547)+(common.v334*v2618)))))-((v618*v2693)+(v604*(v2675+((v616*v1590)+(v346*(-v2618)))))));
        let v2845=(v625*v625);
        let v2846=(((v625*(self.scalar_static_f64[181]*common.v1531))-(v623*((v624*common.v1521)+(common.v327*(common.v571*common.v1629)))))/v2845);
        let v2850=(((v625*(self.scalar_static_f64[181]*common.v1532))-(v623*((v624*common.v1524)+(common.v327*(common.v571*common.v1630)))))/v2845);
        let v2854=(((v625*(self.scalar_static_f64[181]*common.v1533))-(v623*((v624*common.v1527)+(common.v327*(common.v571*common.v1631)))))/v2845);
        let v2858=(((v625*(self.scalar_static_f64[181]*common.v1534))-(v623*((v624*common.v1530)+(common.v327*(common.v571*common.v1632)))))/v2845);
        let v2861=((v626*v2544)+(v585*v2846));
        let v2864=((v626*v2547)+(v585*v2850));
        let v2867=((v626*v2550)+(v585*v2854));
        let v2870=((v626*v2553)+(v585*v2858));
        let v2873=((v626*v2556)+(v587*v2846));
        let v2876=((v626*v2559)+(v587*v2850));
        let v2879=((v626*v2562)+(v587*v2854));
        let v2882=((v626*v2565)+(v587*v2858));
        let v2883=(self.scalar_static_f64[183]*common.v1863);
        let v2884=(self.scalar_static_f64[183]*common.v1864);
        let v2885=(self.scalar_static_f64[183]*common.v1865);
        let v2886=(self.scalar_static_f64[183]*common.v1866);
        let v2888=(common.v364*common.v364);
        let v2889=((-common.v1654)/v2888);
        let v2891=((-common.v1655)/v2888);
        let v2893=((-common.v1656)/v2888);
        let v2895=((-common.v1657)/v2888);
        let v2897=(common.v367*common.v367);
        let v2898=((-common.v1671)/v2897);
        let v2900=((-common.v1672)/v2897);
        let v2902=((-common.v1673)/v2897);
        let v2904=((-common.v1674)/v2897);
        let v3067=(v655*v655);
        let v3068=((-(self.scalar_static_f64[6]*v1879))/v3067);
        let v3071=((-(self.scalar_static_f64[6]*v1880))/v3067);
        let v3074=((-(self.scalar_static_f64[6]*v1881))/v3067);
        let v3077=((-(self.scalar_static_f64[6]*v1882))/v3067);
        let v3111=(v438*v438);
        let v3112=((-v1919)/v3111);
        let v3114=((-v1920)/v3111);
        let v3116=((-v1921)/v3111);
        let v3118=((-v1922)/v3111);
        let v3167=(self.scalar_static_f64[183]*common.v2106);
        let v3168=(self.scalar_static_f64[183]*common.v2107);
        let v3169=(self.scalar_static_f64[183]*common.v2108);
        let v3170=(self.scalar_static_f64[183]*common.v2109);
        let v3203=(((common.v500*(v511*v2172))-(v676*common.v2135))/common.v3202);
        let v3207=(((common.v500*(v511*v2173))-(v676*common.v2137))/common.v3202);
        let v3211=(((common.v500*(v511*v2174))-(v676*common.v2139))/common.v3202);
        let v3215=(((common.v500*(v511*v2175))-(v676*common.v2141))/common.v3202);
        let v3226=((v679*v3203)+(v677*(common.v2122+(common.v211*common.v2126))));
        let v3229=((v679*v3207)+(v677*(common.v2123+(common.v211*common.v2127))));
        let v3232=((v679*v3211)+(v677*(common.v2124+(common.v211*common.v2128))));
        let v3235=((v679*v3215)+(v677*(common.v2125+(common.v211*common.v2129))));
        let v3246=((v682*v3203)+(v677*(common.v2126+(common.v211*common.v2122))));
        let v3249=((v682*v3207)+(v677*(common.v2127+(common.v211*common.v2123))));
        let v3252=((v682*v3211)+(v677*(common.v2128+(common.v211*common.v2124))));
        let v3255=((v682*v3215)+(v677*(common.v2129+(common.v211*common.v2125))));
        let v3287=(v688*v688);
        let v3288=(((v688*((v684*v2218)+(v519*v2168)))-(v685*((v687*common.v1334)+(common.v502*(v2153+v2153)))))/v3287);
        let v3292=(((v688*((v684*v2221)+(v519*v2169)))-(v685*((v687*common.v1335)+(common.v502*(v2156+v2156)))))/v3287);
        let v3296=(((v688*((v684*v2224)+(v519*v2170)))-(v685*((v687*common.v1336)+(common.v502*(v2159+v2159)))))/v3287);
        let v3300=(((v688*((v684*v2227)+(v519*v2171)))-(v685*((v687*common.v1337)+(common.v502*(v2162+v2162)))))/v3287);
        let v3341=((((v689*v2516)+(v582*v3288))+((v680*v2544)+(v585*v3226)))+((v683*((v673*v3167)+(v672*v2516)))+(v674*v3246)));
        let v3342=((((v689*v2519)+(v582*v3292))+((v680*v2547)+(v585*v3229)))+((v683*((v673*v3168)+(v672*v2519)))+(v674*v3249)));
        let v3343=((((v689*v2522)+(v582*v3296))+((v680*v2550)+(v585*v3232)))+((v683*((v673*v3169)+(v672*v2522)))+(v674*v3252)));
        let v3344=((((v689*v2525)+(v582*v3300))+((v680*v2553)+(v585*v3235)))+((v683*((v673*v3170)+(v672*v2525)))+(v674*v3255)));
        let v3385=((((v689*v2528)+(v583*v3288))+((v680*v2556)+(v587*v3226)))+((v683*((v672*v2528)+(v583*v3167)))+(v675*v3246)));
        let v3386=((((v689*v2531)+(v583*v3292))+((v680*v2559)+(v587*v3229)))+((v683*((v672*v2531)+(v583*v3168)))+(v675*v3249)));
        let v3387=((((v689*v2534)+(v583*v3296))+((v680*v2562)+(v587*v3232)))+((v683*((v672*v2534)+(v583*v3169)))+(v675*v3252)));
        let v3388=((((v689*v2537)+(v583*v3300))+((v680*v2565)+(v587*v3235)))+((v683*((v672*v2537)+(v583*v3170)))+(v675*v3255)));
        let v3408=(v701*v701);
        let v3422=(v2153-(((v701*v2218)-(v519*((v700*common.v1334)+(common.v502*(common.v211*v2153)))))/v3408));
        let v3423=(v2156-(((v701*v2221)-(v519*((v700*common.v1335)+(common.v502*(common.v211*v2156)))))/v3408));
        let v3424=(v2159-(((v701*v2224)-(v519*((v700*common.v1336)+(common.v502*(common.v211*v2159)))))/v3408));
        let v3425=(v2162-(((v701*v2227)-(v519*((v700*common.v1337)+(common.v502*(common.v211*v2162)))))/v3408));
        let v3426=(-v2164);
        let v3427=(-v2165);
        let v3428=(-v2166);
        let v3429=(-v2167);
        let v3501=(v711*v711);
        let v3515=(if (self.scalar_static_f64[101]!=0.0){(((v711*v2277)-(v535*((v537*v2261)+(v530*v2281))))/v3501)}else{v3422});
        let v3516=(if (self.scalar_static_f64[101]!=0.0){(((v711*v2278)-(v535*((v537*v2262)+(v530*v2282))))/v3501)}else{v3423});
        let v3517=(if (self.scalar_static_f64[101]!=0.0){(((v711*v2279)-(v535*((v537*v2263)+(v530*v2283))))/v3501)}else{v3424});
        let v3518=(if (self.scalar_static_f64[101]!=0.0){(((v711*v2280)-(v535*((v537*v2264)+(v530*v2284))))/v3501)}else{v3425});
        let v3551=(-((v665*v3112)+(v661*((-((v657*v3068)+(v656*(-v2754))))+(self.scalar_static_f64[221]*v2754)))));
        let v3552=(-((v665*v3114)+(v661*((-((v657*v3071)+(v656*(-v2755))))+(self.scalar_static_f64[221]*v2755)))));
        let v3553=(-((v665*v3116)+(v661*((-((v657*v3074)+(v656*(-v2756))))+(self.scalar_static_f64[221]*v2756)))));
        let v3554=(-((v665*v3118)+(v661*((-((v657*v3077)+(v656*(-v2757))))+(self.scalar_static_f64[221]*v2757)))));
        let v3563=(-((v670*v3112)+(v661*((-((v659*v3068)+(v656*(-v2818))))+(self.scalar_static_f64[221]*v2818)))));
        let v3564=(-((v670*v3114)+(v661*((-((v659*v3071)+(v656*(-v2819))))+(self.scalar_static_f64[221]*v2819)))));
        let v3565=(-((v670*v3116)+(v661*((-((v659*v3074)+(v656*(-v2820))))+(self.scalar_static_f64[221]*v2820)))));
        let v3566=(-((v670*v3118)+(v661*((-((v659*v3077)+(v656*(-v2821))))+(self.scalar_static_f64[221]*v2821)))));
        let v3577=(v553*v553);
        let v3588=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2334))/v3577)}else{v3515});
        let v3589=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2335))/v3577)}else{v3516});
        let v3590=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2336))/v3577)}else{v3517});
        let v3591=(if self.scalar_static_bool[12]{((-(self.scalar_static_f64[16]*v2337))/v3577)}else{v3518});
        let v3678=(v739*v739);
        let v3679=((-(self.scalar_static_f64[104]*((v738*common.v1334)+(v561*((v737*v2367)+(v562*(common.v571*v2378)))))))/v3678);
        let v3682=((-(self.scalar_static_f64[104]*((v738*common.v1335)+(v561*((v737*v2368)+(v562*(common.v571*v2381)))))))/v3678);
        let v3685=((-(self.scalar_static_f64[104]*((v738*common.v1336)+(v561*((v737*v2369)+(v562*(common.v571*v2384)))))))/v3678);
        let v3688=((-(self.scalar_static_f64[104]*((v738*common.v1337)+(v561*((v737*v2370)+(v562*(common.v571*v2387)))))))/v3678);
        let v3797=((self.scalar_static_f64[109]*((v752*(-v2398))+(v748*((v2556+((v749*v2388)+(v566*((if self.scalar_static_bool[12]{(v3563+((v732*v3588)+(v725*(((v709*v3426)+(v704*(v3385+((v703*v2528)+(v583*v3422)))))+(self.scalar_static_f64[21]*v3385)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3563-(if (self.scalar_static_f64[101]!=0.0){((v713*v2528)+(v583*v3515))}else{common.v1}))}else{common.v1})})+((v740*v2528)+(v583*v3679))))))-((v652*v2883)+(v629*((v2528-((v645*v2889)+(v630*(v2672+((v628*common.v1633)+(common.v360*v2873))))))+((v650*v2898)+(v631*(v2672+((v648*common.v1637)+(common.v361*(-v2873)))))))))))))+(self.scalar_static_f64[109]*((v746*v2398)+(v568*((v2544+((v743*v2388)+(v566*((if self.scalar_static_bool[12]{(v3551+((v727*v3588)+(v725*(((v706*v3426)+(v704*(v3341+((v703*v2516)+(v582*v3422)))))+(self.scalar_static_f64[21]*v3341)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3551-(if (self.scalar_static_f64[101]!=0.0){((v713*v2516)+(v582*v3515))}else{common.v1}))}else{common.v1})})+((v740*v2516)+(v582*v3679))))))-((v641*v2883)+(v629*((v2516-((v634*v2889)+(v630*(v2652+((v627*common.v1633)+(common.v360*v2861))))))+((v639*v2898)+(v631*(v2652+((v637*common.v1637)+(common.v361*(-v2861))))))))))))));
        let v3798=((self.scalar_static_f64[109]*((v752*(-v2401))+(v748*((v2559+((v749*v2389)+(v566*((if self.scalar_static_bool[12]{(v3564+((v732*v3589)+(v725*(((v709*v3427)+(v704*(v3386+((v703*v2531)+(v583*v3423)))))+(self.scalar_static_f64[21]*v3386)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3564-(if (self.scalar_static_f64[101]!=0.0){((v713*v2531)+(v583*v3516))}else{common.v1}))}else{common.v1})})+((v740*v2531)+(v583*v3682))))))-((v652*v2884)+(v629*((v2531-((v645*v2891)+(v630*(v2673+((v628*common.v1634)+(common.v360*v2876))))))+((v650*v2900)+(v631*(v2673+((v648*common.v1638)+(common.v361*(-v2876)))))))))))))+(self.scalar_static_f64[109]*((v746*v2401)+(v568*((v2547+((v743*v2389)+(v566*((if self.scalar_static_bool[12]{(v3552+((v727*v3589)+(v725*(((v706*v3427)+(v704*(v3342+((v703*v2519)+(v582*v3423)))))+(self.scalar_static_f64[21]*v3342)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3552-(if (self.scalar_static_f64[101]!=0.0){((v713*v2519)+(v582*v3516))}else{common.v1}))}else{common.v1})})+((v740*v2519)+(v582*v3682))))))-((v641*v2884)+(v629*((v2519-((v634*v2891)+(v630*(v2653+((v627*common.v1634)+(common.v360*v2864))))))+((v639*v2900)+(v631*(v2653+((v637*common.v1638)+(common.v361*(-v2864))))))))))))));
        let v3799=((self.scalar_static_f64[109]*((v752*(-v2404))+(v748*((v2562+((v749*v2390)+(v566*((if self.scalar_static_bool[12]{(v3565+((v732*v3590)+(v725*(((v709*v3428)+(v704*(v3387+((v703*v2534)+(v583*v3424)))))+(self.scalar_static_f64[21]*v3387)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3565-(if (self.scalar_static_f64[101]!=0.0){((v713*v2534)+(v583*v3517))}else{common.v1}))}else{common.v1})})+((v740*v2534)+(v583*v3685))))))-((v652*v2885)+(v629*((v2534-((v645*v2893)+(v630*(v2674+((v628*common.v1635)+(common.v360*v2879))))))+((v650*v2902)+(v631*(v2674+((v648*common.v1639)+(common.v361*(-v2879)))))))))))))+(self.scalar_static_f64[109]*((v746*v2404)+(v568*((v2550+((v743*v2390)+(v566*((if self.scalar_static_bool[12]{(v3553+((v727*v3590)+(v725*(((v706*v3428)+(v704*(v3343+((v703*v2522)+(v582*v3424)))))+(self.scalar_static_f64[21]*v3343)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3553-(if (self.scalar_static_f64[101]!=0.0){((v713*v2522)+(v582*v3517))}else{common.v1}))}else{common.v1})})+((v740*v2522)+(v582*v3685))))))-((v641*v2885)+(v629*((v2522-((v634*v2893)+(v630*(v2654+((v627*common.v1635)+(common.v360*v2867))))))+((v639*v2902)+(v631*(v2654+((v637*common.v1639)+(common.v361*(-v2867))))))))))))));
        let v3800=((self.scalar_static_f64[109]*((v752*(-v2407))+(v748*((v2565+((v749*v2391)+(v566*((if self.scalar_static_bool[12]{(v3566+((v732*v3591)+(v725*(((v709*v3429)+(v704*(v3388+((v703*v2537)+(v583*v3425)))))+(self.scalar_static_f64[21]*v3388)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3566-(if (self.scalar_static_f64[101]!=0.0){((v713*v2537)+(v583*v3518))}else{common.v1}))}else{common.v1})})+((v740*v2537)+(v583*v3688))))))-((v652*v2886)+(v629*((v2537-((v645*v2895)+(v630*(v2675+((v628*common.v1636)+(common.v360*v2882))))))+((v650*v2904)+(v631*(v2675+((v648*common.v1640)+(common.v361*(-v2882)))))))))))))+(self.scalar_static_f64[109]*((v746*v2407)+(v568*((v2553+((v743*v2391)+(v566*((if self.scalar_static_bool[12]{(v3554+((v727*v3591)+(v725*(((v706*v3429)+(v704*(v3344+((v703*v2525)+(v582*v3425)))))+(self.scalar_static_f64[21]*v3344)))))}else{(if (self.scalar_static_f64[101]!=0.0){(v3554-(if (self.scalar_static_f64[101]!=0.0){((v713*v2525)+(v582*v3518))}else{common.v1}))}else{common.v1})})+((v740*v2525)+(v582*v3688))))))-((v641*v2886)+(v629*((v2525-((v634*v2895)+(v630*(v2655+((v627*common.v1636)+(common.v360*v2870))))))+((v639*v2904)+(v631*(v2655+((v637*common.v1640)+(common.v361*(-v2870))))))))))))));
        let v3802=(v762*v762);
        let v3812=((v763*((v568*v2388)+(v566*v2398)))+(v569*((-v3797)/v3802)));
        let v3815=((v763*((v568*v2389)+(v566*v2401)))+(v569*((-v3798)/v3802)));
        let v3818=((v763*((v568*v2390)+(v566*v2404)))+(v569*((-v3799)/v3802)));
        let v3821=((v763*((v568*v2391)+(v566*v2407)))+(v569*((-v3800)/v3802)));
        let v3823=(self.scalar_static_f64[13]*common.v1545);
        let v3826=(common.v1548-(self.scalar_static_f64[13]*common.v1544));
        let v3828=(common.v1549-(self.scalar_static_f64[13]*common.v1546));
        let v3829=(common.v1550-(self.scalar_static_f64[13]*common.v1547));
        let v3831=(v766*v766);
        let v4209=ddt_scale;
        let v4241=(v976*v976);
        let v4260=(v987*v987);
        let v4281=(v999*v999);
        let v4306=(v1032*v1032);
        let v4321=(v1041*v1041);
        let v4338=(v1051*v1051);
        let v4480=(self.scalar_static_f64[20]*(common.v4042*v4209));
        let v4481=(self.scalar_static_f64[20]*(common.v4045*v4209));
        let v4482=(self.scalar_static_f64[20]*(common.v4048*v4209));
        let v4483=(self.scalar_static_f64[20]*(common.v4051*v4209));
        let v4488=(self.scalar_static_f64[20]*(common.v4126*v4209));
        let v4489=(self.scalar_static_f64[20]*(common.v4129*v4209));
        let v4490=(self.scalar_static_f64[20]*(common.v4132*v4209));
        let v4491=(self.scalar_static_f64[20]*(common.v4135*v4209));
        let v4496=(self.scalar_static_f64[20]*(if v788{common.v1}else{(if (v770!=0.0){((v785*v3812)+(v764*(if (v770!=0.0){((v783*(if (v770!=0.0){(v781*(if v779{common.v1}else{(if (v770!=0.0){(self.scalar_static_f64[250]*(if (v770!=0.0){((-v3826)/v3831)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v782*(self.scalar_static_f64[224]*v3826)))}else{common.v1})))}else{common.v1})}));
        let v4497=(self.scalar_static_f64[20]*(if v788{common.v1}else{(if (v770!=0.0){((v785*v3815)+(v764*(if (v770!=0.0){((v783*(if (v770!=0.0){(v781*(if v779{common.v1}else{(if (v770!=0.0){(self.scalar_static_f64[250]*(if (v770!=0.0){(v3823/v3831)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v782*(self.scalar_static_f64[224]*(-v3823))))}else{common.v1})))}else{common.v1})}));
        let v4498=(self.scalar_static_f64[20]*(if v788{common.v1}else{(if (v770!=0.0){((v785*v3818)+(v764*(if (v770!=0.0){((v783*(if (v770!=0.0){(v781*(if v779{common.v1}else{(if (v770!=0.0){(self.scalar_static_f64[250]*(if (v770!=0.0){((-v3828)/v3831)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v782*(self.scalar_static_f64[224]*v3828)))}else{common.v1})))}else{common.v1})}));
        let v4499=(self.scalar_static_f64[20]*(if v788{common.v1}else{(if (v770!=0.0){((v785*v3821)+(v764*(if (v770!=0.0){((v783*(if (v770!=0.0){(v781*(if v779{common.v1}else{(if (v770!=0.0){(self.scalar_static_f64[250]*(if (v770!=0.0){((-v3829)/v3831)}else{common.v1}))}else{common.v1})}))}else{common.v1}))+(v782*(self.scalar_static_f64[224]*v3829)))}else{common.v1})))}else{common.v1})}));

        stamper.stamp_current_dense_local(
            Some(0),
            Some(2),
            multiplicity * ((v764*v1152)),
            &[(v1152*v3812),(v1152*v3815),(v1152*v3818),(v1152*v3821)],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (common.v845!=0.0){v1154}else{common.v1})),
            &[(if (common.v845!=0.0){v4480}else{common.v1}),(if (common.v845!=0.0){v4481}else{common.v1}),(if (common.v845!=0.0){v4482}else{common.v1}),(if (common.v845!=0.0){v4483}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if (common.v845!=0.0){v1156}else{common.v1})),
            &[(if (common.v845!=0.0){v4488}else{common.v1}),(if (common.v845!=0.0){v4489}else{common.v1}),(if (common.v845!=0.0){v4490}else{common.v1}),(if (common.v845!=0.0){v4491}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if (common.v845!=0.0){v1158}else{common.v1})),
            &[(if (common.v845!=0.0){v4496}else{common.v1}),(if (common.v845!=0.0){v4497}else{common.v1}),(if (common.v845!=0.0){v4498}else{common.v1}),(if (common.v845!=0.0){v4499}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if common.v1160{v1154}else{common.v1})),
            &[(if common.v1160{v4480}else{common.v1}),(if common.v1160{v4481}else{common.v1}),(if common.v1160{v4482}else{common.v1}),(if common.v1160{v4483}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(0),
            Some(3),
            multiplicity * ((if common.v1160{v1156}else{common.v1})),
            &[(if common.v1160{v4488}else{common.v1}),(if common.v1160{v4489}else{common.v1}),(if common.v1160{v4490}else{common.v1}),(if common.v1160{v4491}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(2),
            Some(3),
            multiplicity * ((if common.v1160{v1158}else{common.v1})),
            &[(if common.v1160{v4496}else{common.v1}),(if common.v1160{v4497}else{common.v1}),(if common.v1160{v4498}else{common.v1}),(if common.v1160{v4499}else{common.v1})],
            &[],
            multiplicity,
        );
        stamper.stamp_current_dense_local(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[20]*v1164)),
            &[(self.scalar_static_f64[20]*(common.v4205*v4209)),(self.scalar_static_f64[20]*(common.v4206*v4209)),(self.scalar_static_f64[20]*(common.v4207*v4209)),(self.scalar_static_f64[20]*(common.v4208*v4209))],
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
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v978-common.v31))-(self.scalar_static_f64[286]*(v989-common.v31)))-(self.scalar_static_f64[285]*(v1001-common.v31)))+((v965*v1168)+(common.v195*self.scalar_static_f64[168])))))),
            0,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v978*(((v976*self.scalar_static_f64[312])-(v972*(if v975{self.scalar_static_f64[20]}else{common.v1})))/v4241)))-(self.scalar_static_f64[286]*(v989*(((v987*self.scalar_static_f64[316])-(v984*(if v986{self.scalar_static_f64[20]}else{common.v1})))/v4260))))-(self.scalar_static_f64[285]*(v1001*(((v999*self.scalar_static_f64[320])-(v996*(if v998{self.scalar_static_f64[20]}else{common.v1})))/v4281))))+(((v1168*(if v959{(self.scalar_static_f64[157]*(v962*self.scalar_static_f64[308]))}else{common.v1}))+(v965*(self.scalar_static_f64[289]*(-(v1166*(if (v949!=0.0){common.v1}else{self.scalar_static_f64[306]}))))))+self.scalar_static_f64[176]))))),
            3,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v978*(((v976*self.scalar_static_f64[313])-(v972*(if v975{self.scalar_static_f64[169]}else{common.v1})))/v4241)))-(self.scalar_static_f64[286]*(v989*(((v987*self.scalar_static_f64[317])-(v984*(if v986{self.scalar_static_f64[169]}else{common.v1})))/v4260))))-(self.scalar_static_f64[285]*(v1001*(((v999*self.scalar_static_f64[321])-(v996*(if v998{self.scalar_static_f64[169]}else{common.v1})))/v4281))))+(((v1168*(if v959{(self.scalar_static_f64[157]*(v962*self.scalar_static_f64[309]))}else{common.v1}))+(v965*(self.scalar_static_f64[289]*(-(v1166*(if (v949!=0.0){common.v1}else{self.scalar_static_f64[307]}))))))+self.scalar_static_f64[177]))))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v1034-common.v31))-(self.scalar_static_f64[296]*(v1043-common.v31)))-(self.scalar_static_f64[295]*(v1053-common.v31)))+((v1026*v1178)+(common.v192*self.scalar_static_f64[168])))))),
            2,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v1034*(((v1032*self.scalar_static_f64[312])-(v1029*(if v1031{self.scalar_static_f64[20]}else{common.v1})))/v4306)))-(self.scalar_static_f64[296]*(v1043*(((v1041*self.scalar_static_f64[316])-(v1038*(if v1040{self.scalar_static_f64[20]}else{common.v1})))/v4321))))-(self.scalar_static_f64[295]*(v1053*(((v1051*self.scalar_static_f64[320])-(v1048*(if v1050{self.scalar_static_f64[20]}else{common.v1})))/v4338))))+(self.scalar_static_f64[176]+((v1178*(if v1021{(self.scalar_static_f64[157]*(v1023*self.scalar_static_f64[308]))}else{common.v1}))+(v1026*(self.scalar_static_f64[298]*(-(v1176*(if (v1013!=0.0){common.v1}else{self.scalar_static_f64[306]}))))))))))),
            3,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*((((self.scalar_static_f64[291]*(v1034*(((v1032*self.scalar_static_f64[313])-(v1029*(if v1031{self.scalar_static_f64[169]}else{common.v1})))/v4306)))-(self.scalar_static_f64[296]*(v1043*(((v1041*self.scalar_static_f64[317])-(v1038*(if v1040{self.scalar_static_f64[169]}else{common.v1})))/v4321))))-(self.scalar_static_f64[295]*(v1053*(((v1051*self.scalar_static_f64[321])-(v1048*(if v1050{self.scalar_static_f64[169]}else{common.v1})))/v4338))))+(self.scalar_static_f64[177]+((v1178*(if v1021{(self.scalar_static_f64[157]*(v1023*self.scalar_static_f64[309]))}else{common.v1}))+(v1026*(self.scalar_static_f64[298]*(-(v1176*(if (v1013!=0.0){common.v1}else{self.scalar_static_f64[307]}))))))))))),
        );
        stamper.stamp_current_node2_local(
            Some(0),
            Some(3),
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*v1185))),
            0,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4422)))),
            3,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4425)))),
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*v1188))),
            2,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4472)))),
            3,
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4475)))),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        let v842=0.0;
        let v843=0.0;
        let v1154=(self.scalar_static_f64[20]*v842);
        let v1156=(self.scalar_static_f64[20]*v843);
        let v1164=0.0;
        let v1185=0.0;
        let v1188=0.0;
        let v4209=1.0;
        let v4480=(self.scalar_static_f64[20]*(common.v4042*v4209));
        let v4481=(self.scalar_static_f64[20]*(common.v4045*v4209));
        let v4482=(self.scalar_static_f64[20]*(common.v4048*v4209));
        let v4483=(self.scalar_static_f64[20]*(common.v4051*v4209));
        let v4488=(self.scalar_static_f64[20]*(common.v4126*v4209));
        let v4489=(self.scalar_static_f64[20]*(common.v4129*v4209));
        let v4490=(self.scalar_static_f64[20]*(common.v4132*v4209));
        let v4491=(self.scalar_static_f64[20]*(common.v4135*v4209));

        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if (common.v845!=0.0){v4480}else{common.v1}),(if (common.v845!=0.0){v4481}else{common.v1}),(if (common.v845!=0.0){v4482}else{common.v1}),(if (common.v845!=0.0){v4483}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if (common.v845!=0.0){v4488}else{common.v1}),(if (common.v845!=0.0){v4489}else{common.v1}),(if (common.v845!=0.0){v4490}else{common.v1}),(if (common.v845!=0.0){v4491}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[2]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v1160{v4480}else{common.v1}),(if common.v1160{v4481}else{common.v1}),(if common.v1160{v4482}else{common.v1}),(if common.v1160{v4483}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[0]),
            Some(nodes[3]),
            &nodes,
            &[(if common.v1160{v4488}else{common.v1}),(if common.v1160{v4489}else{common.v1}),(if common.v1160{v4490}else{common.v1}),(if common.v1160{v4491}else{common.v1})],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[3]),
            &nodes,
            &[(self.scalar_static_f64[20]*(common.v4205*v4209)),(self.scalar_static_f64[20]*(common.v4206*v4209)),(self.scalar_static_f64[20]*(common.v4207*v4209)),(self.scalar_static_f64[20]*(common.v4208*v4209))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[0]),
            Some(nodes[3]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4422)))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4425)))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[2]),
            Some(nodes[3]),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4472)))),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[90]*(self.scalar_static_f64[20]*(v4209*common.v4475)))),
        );
    }
}
