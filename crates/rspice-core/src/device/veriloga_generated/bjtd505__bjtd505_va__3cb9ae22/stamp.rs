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
    v154: f64,
    v392: f64,
    v395: f64,
    v407: f64,
    v433: f64,
    v673: f64,
    v677: f64,
    v679: f64,
    v684: f64,
    v687: f64,
    v692: f64,
    v700: f64,
    v703: f64,
    v706: f64,
    v710: f64,
    v744: f64,
    v745: f64,
    v746: bool,
    v749: bool,
    v750: f64,
    v828: f64,
    v946: f64,
    v1003: f64,
    v1027: f64,
    v1030: f64,
    v1033: f64,
    v1059: f64,
    v1135: f64,
    v1170: f64,
    v1171: f64,
    v1176: f64,
    v1177: f64,
    v1195: f64,
    v1196: bool,
    v1199: bool,
    v1200: f64,
    v1209: f64,
    v1239: f64,
    v1241: f64,
    v1242: bool,
    v1247: bool,
    v1248: f64,
    v1255: f64,
    v1256: f64,
    v1257: bool,
    v1262: bool,
    v1264: f64,
    v1314: f64,
    v1316: f64,
    v1317: bool,
    v1322: bool,
    v1323: f64,
    v1349: f64,
    v1361: f64,
    v1373: f64,
    v1385: f64,
    v1391: bool,
    v1392: f64,
    v1395: f64,
    v1396: bool,
    v1401: bool,
    v1402: f64,
    v1408: f64,
    v1412: f64,
    v1415: f64,
    v1423: f64,
    v1424: f64,
    v1425: f64,
    v1427: f64,
    v1429: f64,
    v1433: f64,
    v1434: f64,
    v1436: f64,
    v1438: bool,
    v1439: bool,
    v1440: bool,
    v1445: bool,
    v1446: f64,
    v1483: bool,
    v1485: f64,
    v1487: f64,
    v1488: f64,
    v1491: f64,
    v1492: bool,
    v1497: bool,
    v1498: f64,
    v1503: f64,
    v1506: f64,
    v1508: f64,
    v1516: f64,
    v1517: f64,
    v1518: f64,
    v1520: f64,
    v1525: f64,
    v1526: f64,
    v1528: f64,
    v1529: bool,
    v1530: bool,
    v1531: bool,
    v1536: bool,
    v1537: f64,
    v1604: f64,
    v1620: f64,
    v1641: f64,
    v1709: f64,
    v1719: bool,
    v1729: bool,
    v1730: bool,
    v1731: f64,
    v1734: bool,
    v1735: f64,
    v1739: f64,
    v1740: f64,
    v1742: f64,
    v1746: f64,
    v1747: bool,
    v1752: bool,
    v1753: f64,
    v1766: bool,
    v1870: bool,
    v1871: f64,
    v1873: f64,
    v1875: f64,
    v1877: f64,
    v1879: f64,
    v1880: bool,
    v1882: bool,
    v1890: f64,
    v1892: bool,
    v1893: f64,
    v1894: f64,
    v1900: bool,
    v1902: f64,
    v1903: f64,
    v1907: f64,
    v1909: f64,
    v1912: f64,
    v1913: bool,
    v1918: bool,
    v1919: f64,
    v2246: f64,
    v2274: f64,
    v2317: f64,
    v2320: f64,
    v2323: f64,
    v2326: f64,
    v2330: f64,
    v2334: f64,
    v2342: f64,
    v2348: f64,
    v2359: f64,
    v2401: f64,
    v2402: f64,
    v2403: f64,
    v2404: f64,
    v2522: f64,
    v2523: f64,
    v2524: f64,
    v2811: f64,
    v2812: f64,
    v2813: f64,
    v2959: f64,
    v2960: f64,
    v2961: f64,
    v3002: f64,
    v3003: f64,
    v3004: f64,
    v3011: f64,
    v3012: f64,
    v3013: f64,
    v3020: f64,
    v3021: f64,
    v3022: f64,
    v3054: f64,
    v3055: f64,
    v3234: f64,
    v3235: f64,
    v3236: f64,
    v3326: f64,
    v3327: f64,
    v3328: f64,
    v3329: f64,
    v3332: f64,
    v3335: f64,
    v3338: f64,
    v3341: f64,
    v3342: f64,
    v3343: f64,
    v3344: f64,
    v3346: f64,
    v3350: f64,
    v3353: f64,
    v3387: f64,
    v3388: f64,
    v3447: f64,
    v3448: f64,
    v3581: f64,
    v3582: f64,
    v3583: f64,
    v3638: f64,
    v3639: f64,
    v3640: f64,
    v3653: f64,
    v3654: f64,
    v3655: f64,
    v3676: f64,
    v3677: f64,
    v3678: f64,
    v3679: f64,
    v3680: f64,
    v3697: f64,
    v3698: f64,
    v3699: f64,
    v3700: f64,
    v3701: f64,
    v4159: f64,
    v4160: f64,
    v4161: f64,
    v4162: f64,
    v4175: f64,
    v4176: f64,
    v4177: f64,
    v4178: f64,
    v4179: f64,
    v4180: f64,
    v4181: f64,
    v4182: f64,
    v4307: f64,
    v4308: f64,
    v4309: f64,
    v4310: f64,
    v4311: f64,
    v4312: f64,
    v4313: f64,
    v4314: f64,
    v4619: f64,
    v4620: f64,
    v4621: f64,
    v4622: f64,
    v6318: f64,
    v6319: f64,
    v6320: f64,
    v6321: f64,
    v6322: f64,
    v6323: f64,
    v6489: f64,
    v6490: f64,
    v6491: f64,
    v6492: f64,
    v6493: f64,
    v6494: f64,
    v6508: f64,
    v6509: f64,
    v6514: f64,
    v6515: f64,
    v6516: f64,
    v6517: f64,
    v6518: f64,
    v6519: f64,
    v6532: f64,
    v6533: f64,
    v6534: f64,
    v6535: f64,
    v6536: f64,
    v6537: f64,
    v6590: f64,
    v6591: f64,
    v6592: f64,
    v6593: f64,
    v6594: f64,
    v6595: f64,
    v6596: f64,
    v6597: f64,
    v6637: f64,
    v6638: f64,
    v6639: f64,
    v6640: f64,
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
        let v46=0.1;
        let v154=3.0;
        let v392=1e-6;
        let v395=0.5;
        let v407=4.0;
        let v433=6.0;
        let v670=ctx.node_voltage(nodes[5]);
        let v671=ctx.node_voltage(nodes[6]);
        let v673=(self.scalar_static_f64[0]*(v670-v671));
        let v674=ctx.node_voltage(nodes[7]);
        let v676=(self.scalar_static_f64[0]*(v670-v674));
        let v677=ctx.node_voltage(nodes[3]);
        let v679=(self.scalar_static_f64[0]*(v670-v677));
        let v680=ctx.node_voltage(nodes[4]);
        let v682=(self.scalar_static_f64[0]*(v680-v677));
        let v684=(self.scalar_static_f64[0]*(v680-v670));
        let v686=(self.scalar_static_f64[0]*(v671-v674));
        let v687=ctx.node_voltage(nodes[2]);
        let v690=ctx.node_voltage(nodes[1]);
        let v692=(self.scalar_static_f64[0]*(v690-v680));
        let v697=(self.scalar_static_f64[0]*(v690-ctx.node_voltage(nodes[0])));
        let v698=ctx.node_voltage(nodes[9]);
        let v700=(self.scalar_static_f64[0]*(v698-v671));
        let v703=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[8])-v698));
        let v706=(((v676+v684)-v686)-v700);
        let v710=((v706+(v692+(-v697)))-v703);
        let v711=(v697+v710);
        let v712=(self.scalar_static_f64[352]*v676);
        let v714=(v712<self.scalar_static_f64[188]);
        let v715=(v712).exp();
        let v717=(!v714);
        let v719=(if v717{self.scalar_static_f64[189]}else{v4});
        let v724=(self.scalar_static_f64[352]*v679);
        let v725=(v724/self.scalar_static_f64[549]);
        let v726=(v725<self.scalar_static_f64[188]);
        let v727=(v725).exp();
        let v729=(!v726);
        let v730=(if v729{self.scalar_static_f64[189]}else{v719});
        let v734=(if v729{(v730*(v1+(v725-self.scalar_static_f64[188])))}else{(if v726{v727}else{v4})});
        let v735=(self.scalar_static_f64[352]*v706);
        let v736=(v735<self.scalar_static_f64[188]);
        let v737=(v735).exp();
        let v739=(!v736);
        let v740=(if v739{self.scalar_static_f64[189]}else{v730});
        let v744=(if v739{(v740*(v1+(v735-self.scalar_static_f64[188])))}else{(if v736{v737}else{v4})});
        let v745=(self.scalar_static_f64[352]*v684);
        let v746=(v745<self.scalar_static_f64[188]);
        let v749=(!v746);
        let v750=(if v749{self.scalar_static_f64[189]}else{v740});
        let v755=(self.scalar_static_f64[352]*v711);
        let v756=(v755<self.scalar_static_f64[188]);
        let v757=(v755).exp();
        let v759=(!v756);
        let v760=(if v759{self.scalar_static_f64[189]}else{v750});
        let v764=(if v759{(v760*(v1+(v755-self.scalar_static_f64[188])))}else{(if v756{v757}else{v4})});
        let v766=(self.scalar_static_f64[352]*(v711-self.scalar_static_f64[436]));
        let v767=(v766<self.scalar_static_f64[188]);
        let v768=(v766).exp();
        let v770=(!v767);
        let v771=(if v770{self.scalar_static_f64[189]}else{v760});
        let v777=(self.scalar_static_f64[352]*(v706-self.scalar_static_f64[436]));
        let v778=(v777<self.scalar_static_f64[188]);
        let v779=(v777).exp();
        let v781=(!v778);
        let v782=(if v781{self.scalar_static_f64[189]}else{v771});
        let v788=(self.scalar_static_f64[352]*(v676-self.scalar_static_f64[436]));
        let v789=(v788<self.scalar_static_f64[188]);
        let v790=(v788).exp();
        let v792=(!v789);
        let v793=(if v792{self.scalar_static_f64[189]}else{v782});
        let v797=(if v792{(v793*(v1+(v788-self.scalar_static_f64[188])))}else{(if v789{v790}else{v4})});
        let v799=(self.scalar_static_f64[352]*(v673-self.scalar_static_f64[436]));
        let v800=(v799<self.scalar_static_f64[188]);
        let v801=(v799).exp();
        let v803=(!v800);
        let v804=(if v803{self.scalar_static_f64[189]}else{v793});
        let v808=(if v803{(v804*(v1+(v799-self.scalar_static_f64[188])))}else{(if v800{v801}else{v4})});
        let v811=((v1+(v407*v797))).sqrt();
        let v814=((v1+(v407*v808))).sqrt();
        let v815=(v31*v808);
        let v816=(v1+v814);
        let v817=(v815/v816);
        let v819=(v817<self.scalar_static_f64[190]);
        let v820=(if v819{self.scalar_static_f64[190]}else{v817});
        let v822=(v1+v811);
        let v823=(v822/v816);
        let v826=(self.scalar_static_f64[351]*((v811-v814)-(v823).ln()));
        let v828=((v686+v826)/self.scalar_static_f64[526]);
        let v829=(v828>v4);
        let v830=100.0;
        let v831=(v673<v830);
        let v832=(v829&&v831);
        let v835=(v829&&(!v831));
        let v837=(v1+(v673-v830));
        let v843=(self.scalar_static_f64[526]*(v395*v828));
        let v845=(v1+(self.scalar_static_f64[352]*v843));
        let v850=(if v829{((self.scalar_static_f64[436]+(self.scalar_static_f64[750]*(v845).ln()))-(if v835{(v830+(v837).ln())}else{(if v832{v673}else{v4})}))}else{v4});
        let v853=(if v829{self.scalar_static_f64[751]}else{v4});
        let v855=(if v829{(v853*v853)}else{v392});
        let v858=(v850<v4);
        let v859=(v829&&v858);
        let v860=(v395*v855);
        let v862=((v855+(if v829{(v850*v850)}else{self.scalar_static_f64[576]}))).sqrt();
        let v863=(v862-v850);
        let v867=(v829&&(!v858));
        let v870=(if v867{(v395*(v850+v862))}else{(if v859{(v860/v863)}else{v4})});
        let v874=(v870+self.scalar_static_f64[193]);
        let v875=(v870*v874);
        let v878=(self.scalar_static_f64[192]*(v870+self.scalar_static_f64[752]));
        let v880=(if v829{(v875/v878)}else{v4});
        let v882=(if v829{(v828/v880)}else{v4});
        let v886=(if v829{((v882-v1)/self.scalar_static_f64[194])}else{self.scalar_static_f64[556]});
        let v887=(v882<v1);
        let v888=(v829&&v887);
        let v889=(v886).exp();
        let v890=(v1+v889);
        let v896=(v829&&(!v887));
        let v898=((-v886)).exp();
        let v899=(v1+v898);
        let v912=(if v829{((if v896{(v882+(self.scalar_static_f64[194]*(v899).ln()))}else{(if v888{(v1+(self.scalar_static_f64[194]*(v890).ln()))}else{v4})})/self.scalar_static_f64[200])}else{v4});
        let v914=(if v829{(v870/self.scalar_static_f64[193])}else{v4});
        let v915=(v407*v912);
        let v916=(v914*v915);
        let v917=(v1+v914);
        let v920=((v1+(v916*v917))).sqrt();
        let v921=(v1+v920);
        let v922=(v31*v912);
        let v923=(v917*v922);
        let v925=(if v829{(v921/v923)}else{v4});
        let v927=(v820*v925);
        let v928=((v1-v925)+v927);
        let v929=(v1+v927);
        let v931=(if v829{(v928/v929)}else{v4});
        let v934=(if v829{(self.scalar_static_f64[352]*(v843*v931))}else{v4});
        let v937=(v1+(v820+v934));
        let v940=(if v829{((v31*v934)+(v820*v937))}else{v4});
        let v943=(if v829{(v395*(v934-v1))}else{v4});
        let v946=(if v829{(v940+(v943*v943))}else{v4});
        let v947=(v934>=v1);
        let v948=(v829&&v947);
        let v949=(v946).sqrt();
        let v953=(v829&&(!v947));
        let v954=(v949-v943);
        let v956=(if v953{(v940/v954)}else{(if v948{(v943+v949)}else{v4})});
        let v959=(v829&&(v956<self.scalar_static_f64[201]));
        let v960=(if v959{self.scalar_static_f64[201]}else{v956});
        let v961=(v1+v960);
        let v970=(if v829{(self.scalar_static_f64[202]*(v828-self.scalar_static_f64[191]))}else{v4});
        let v977=(((if v829{(v828*self.scalar_static_f64[756])}else{v4})+(v970*v970))).sqrt();
        let v986=(v829&&self.scalar_static_bool[20]);
        let v987=(v31*v828);
        let v988=(v828+v880);
        let v993=(v828*self.scalar_static_f64[191]);
        let v994=(v828+self.scalar_static_f64[191]);
        let v999=(!v829);
        let v1000=(v31*v797);
        let v1003=(if v999{(if v717{(v719*(v1+(v712-self.scalar_static_f64[188])))}else{(if v714{v715}else{v4})})}else{(if v829{((v960*v961)*self.scalar_static_f64[754])}else{v4})});
        let v1014=(((v686).abs()<self.scalar_static_f64[758])||((v826).abs()<(self.scalar_static_f64[759]*(v811+v814))));
        let v1015=(v999&&v1014);
        let v1016=(v820+(if v999{(v1000/v822)}else{v960}));
        let v1018=(if v1015{(v395*v1016)}else{v4});
        let v1019=(v1+v1018);
        let v1023=(v999&&(!v1014));
        let v1025=((v676+v826)-v673);
        let v1027=(if v1023{(v826/v1025)}else{(if v1015{(v1018/v1019)}else{v931})});
        let v1029=(if v999{self.scalar_static_f64[757]}else{(if v986{(self.scalar_static_f64[472]*(v46+(v987/v988)))}else{(if (v829&&self.scalar_static_bool[19]){self.scalar_static_f64[757]}else{v4})})});
        let v1030=(if v999{v828}else{(if v829{(v993/v994)}else{v4})});
        let v1033=(if v999{(v1-(v1030/self.scalar_static_f64[191]))}else{(if v829{(self.scalar_static_f64[191]/v994)}else{v4})});
        let v1040=((v679-self.scalar_static_f64[760])/self.scalar_static_f64[761]);
        let v1041=(v679<self.scalar_static_f64[760]);
        let v1042=(v1040).exp();
        let v1043=(v1+v1042);
        let v1048=(!v1041);
        let v1050=((-v1040)).exp();
        let v1051=(v1+v1050);
        let v1055=(if v1048{(self.scalar_static_f64[760]-(self.scalar_static_f64[761]*(v1051).ln()))}else{(if v1041{(v679-(self.scalar_static_f64[761]*(v1043).ln()))}else{v4})});
        let v1057=(v1-(self.scalar_static_f64[492]*v1055));
        let v1059=f64::powf(v1057,self.scalar_static_f64[207]);
        let v1065=((self.scalar_static_f64[762]*(v1-v1059))+(v154*(v679-v1055)));
        let v1076=(if self.scalar_static_bool[26]{v676}else{(if self.scalar_static_bool[24]{(v673+(if v999{v686}else{(if v829{(v970+v977)}else{v4})}))}else{(if self.scalar_static_bool[21]{v673}else{v4})})});
        let v1084=(v1076-self.scalar_static_f64[768]);
        let v1085=(v1084/v1029);
        let v1086=(v1076<self.scalar_static_f64[768]);
        let v1087=(v1085).exp();
        let v1088=(v1+v1087);
        let v1089=(v1088).ln();
        let v1093=(!v1086);
        let v1095=((-v1085)).exp();
        let v1096=(v1+v1095);
        let v1097=(v1096).ln();
        let v1100=(if v1093{(self.scalar_static_f64[768]-(v1029*v1097))}else{(if v1086{(v1076-(v1029*v1089))}else{v4})});
        let v1102=f64::powf(v1033,self.scalar_static_f64[210]);
        let v1106=(v1-(v1100/self.scalar_static_f64[472]));
        let v1107=f64::powf(v1106,self.scalar_static_f64[211]);
        let v1111=(self.scalar_static_f64[765]*v1102);
        let v1112=(v1076-v1100);
        let v1117=((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(v1-(v1102*v1107)))+(v1111*v1112)))+(self.scalar_static_f64[505]*v673));
        let v1120=(v734*self.scalar_static_f64[771]);
        let v1122=((v1+v1120)).sqrt();
        let v1123=(v1+v1122);
        let v1124=(v1120/v1123);
        let v1126=f64::powf(v1003,self.scalar_static_f64[772]);
        let v1127=(self.scalar_static_f64[771]*v1126);
        let v1129=((v1+v1127)).sqrt();
        let v1130=(v1+v1129);
        let v1131=(v1127/v1130);
        let v1134=(v1+(v1065/self.scalar_static_f64[709]));
        let v1135=(v1117/self.scalar_static_f64[707]);
        let v1136=(v1134+v1135);
        let v1147=((if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*v1134))}else{v4})).exp();
        let v1148=((if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*((-v1117)/self.scalar_static_f64[707])))}else{v4})).exp();
        let v1154=(if self.scalar_static_bool[28]{((v1147-v1148)/self.scalar_static_f64[775])}else{(if self.scalar_static_bool[27]{v1136}else{v4})});
        let v1155=0.010000000000000002;
        let v1156=(v1154*v1154);
        let v1157=(v1154<v4);
        let v1158=0.005000000000000001;
        let v1160=((v1155+v1156)).sqrt();
        let v1161=(v1160-v1154);
        let v1164=(!v1157);
        let v1167=(if v1164{(v395*(v1154+v1160))}else{(if v1157{(v1158/v1161)}else{v4})});
        let v1170=(v1+(v395*(v1124+v1131)));
        let v1171=(v1167*v1170);
        let v1174=(v1126*self.scalar_static_f64[776]);
        let v1175=(self.scalar_static_f64[592]*v734);
        let v1176=(v1175-v1174);
        let v1177=(v1176/v1171);
        let v1178=0.0001;
        let v1179=(v679/v1178);
        let v1180=(v679<v4);
        let v1181=(v1179).exp();
        let v1182=(v1+v1181);
        let v1186=(!v1180);
        let v1188=((-v1179)).exp();
        let v1189=(v1+v1188);
        let v1193=(if v1186{(v679+(v1178*(v1189).ln()))}else{(if v1180{(v1178*(v1182).ln())}else{v4})});
        let v1195=(v1193/self.scalar_static_f64[213]);
        let v1196=(v1195<self.scalar_static_f64[188]);
        let v1199=(!v1196);
        let v1200=(if v1199{self.scalar_static_f64[189]}else{v804});
        let v1209=((v679-self.scalar_static_f64[214])/v30);
        let v1230=(v724/self.scalar_static_f64[137]);
        let v1231=(v1230<self.scalar_static_f64[188]);
        let v1232=(v1230).exp();
        let v1234=(!v1231);
        let v1235=(if v1234{self.scalar_static_f64[189]}else{v1200});
        let v1239=(if v1234{(v1235*(v1+(v1230-self.scalar_static_f64[188])))}else{(if v1231{v1232}else{v1193})});
        let v1241=(self.scalar_static_f64[352]*(v679-self.scalar_static_f64[491]));
        let v1242=(v1241<self.scalar_static_f64[188]);
        let v1247=(self.scalar_static_bool[12]&&(!v1242));
        let v1248=(if v1247{self.scalar_static_f64[189]}else{v1235});
        let v1255=((v1177/self.scalar_static_f64[592])-1000.0);
        let v1256=40.0;
        let v1257=(v1255<v1256);
        let v1262=(self.scalar_static_bool[12]&&(!v1257));
        let v1264=(if v1262{2.3538526683702e17}else{v1248});
        let v1304=(self.scalar_static_f64[352]*v682);
        let v1305=(v1304/self.scalar_static_f64[141]);
        let v1306=(v1305<self.scalar_static_f64[188]);
        let v1307=(v1305).exp();
        let v1309=(!v1306);
        let v1310=(if v1309{self.scalar_static_f64[189]}else{v1264});
        let v1314=(if v1309{(v1310*(v1+(v1305-self.scalar_static_f64[188])))}else{(if v1306{v1307}else{v1239})});
        let v1316=(self.scalar_static_f64[352]*(v682-self.scalar_static_f64[491]));
        let v1317=(v1316<self.scalar_static_f64[188]);
        let v1322=(self.scalar_static_bool[12]&&(!v1317));
        let v1323=(if v1322{self.scalar_static_f64[189]}else{v1310});
        let v1340=(v724/self.scalar_static_f64[124]);
        let v1341=(v1340<self.scalar_static_f64[188]);
        let v1342=(v1340).exp();
        let v1344=(!v1341);
        let v1345=(if v1344{self.scalar_static_f64[189]}else{v1323});
        let v1349=(if v1344{(v1345*(v1+(v1340-self.scalar_static_f64[188])))}else{(if v1341{v1342}else{v1314})});
        let v1352=(v1304/self.scalar_static_f64[158]);
        let v1353=(v1352<self.scalar_static_f64[188]);
        let v1354=(v1352).exp();
        let v1356=(!v1353);
        let v1357=(if v1356{self.scalar_static_f64[189]}else{v1345});
        let v1361=(if v1356{(v1357*(v1+(v1352-self.scalar_static_f64[188])))}else{(if v1353{v1354}else{v1349})});
        let v1364=(v735/self.scalar_static_f64[130]);
        let v1365=(v1364<self.scalar_static_f64[188]);
        let v1366=(v1364).exp();
        let v1368=(!v1365);
        let v1369=(if v1368{self.scalar_static_f64[189]}else{v1357});
        let v1373=(if v1368{(v1369*(v1+(v1364-self.scalar_static_f64[188])))}else{(if v1365{v1366}else{v1361})});
        let v1376=(v1304/self.scalar_static_f64[162]);
        let v1377=(v1376<self.scalar_static_f64[188]);
        let v1378=(v1376).exp();
        let v1380=(!v1377);
        let v1381=(if v1380{self.scalar_static_f64[189]}else{v1369});
        let v1385=(if v1380{(v1381*(v1+(v1376-self.scalar_static_f64[188])))}else{(if v1377{v1378}else{v1373})});
        let v1391=(v1180&&self.scalar_static_bool[36]);
        let v1392=(v31*v1059);
        let v1395=(self.scalar_static_f64[674]*(v1-(self.scalar_static_f64[18]/v1392)));
        let v1396=(v1395<self.scalar_static_f64[188]);
        let v1401=(v1391&&(!v1396));
        let v1402=(if v1401{self.scalar_static_f64[189]}else{v1381});
        let v1408=(if v1391{(self.scalar_static_f64[492]*v679)}else{self.scalar_static_f64[705]});
        let v1410=1e-30;
        let v1412=(((v1408*v1408)+v1410)).sqrt();
        let v1415=f64::powf(v1412,self.scalar_static_f64[218]);
        let v1423=(v433*v1408);
        let v1424=(v1408*v1423);
        let v1425=(v1408+self.scalar_static_f64[221]);
        let v1427=((self.scalar_static_f64[16]*(self.scalar_static_f64[220]-((v154*v1408)*self.scalar_static_f64[221])))-(v1424*v1425));
        let v1429=0.16666666666666666;
        let v1433=(self.scalar_static_f64[674]*(self.scalar_static_f64[18]*v679));
        let v1434=(self.scalar_static_f64[375]*(if v1391{((v1415*v1427)*v1429)}else{v4}));
        let v1436=(if v1391{(v1433/v1434)}else{v1408});
        let v1437=-0.001;
        let v1438=(v1436<v1437);
        let v1439=(v1436<self.scalar_static_f64[188]);
        let v1440=(v1391&&v1438);
        let v1445=(v1440&&(!v1439));
        let v1446=(if v1445{self.scalar_static_f64[189]}else{v1402});
        let v1483=(self.scalar_static_bool[39]&&(v673<v4));
        let v1484=(self.scalar_static_f64[493]*v673);
        let v1485=(v1-v1484);
        let v1487=(if v1483{f64::powf(v1485,self.scalar_static_f64[211])}else{v4});
        let v1488=(v31*v1487);
        let v1491=(self.scalar_static_f64[694]*(v1-(self.scalar_static_f64[49]/v1488)));
        let v1492=(v1491<self.scalar_static_f64[188]);
        let v1497=(v1483&&(!v1492));
        let v1498=(if v1497{self.scalar_static_f64[189]}else{v1446});
        let v1503=(if v1483{v1484}else{self.scalar_static_f64[685]});
        let v1506=((v1410+(v1503*v1503))).sqrt();
        let v1508=f64::powf(v1506,self.scalar_static_f64[222]);
        let v1516=(v433*v1503);
        let v1517=(v1503*v1516);
        let v1518=(v1503+self.scalar_static_f64[225]);
        let v1520=((self.scalar_static_f64[47]*(self.scalar_static_f64[224]-((v154*v1503)*self.scalar_static_f64[225])))-(v1517*v1518));
        let v1525=(self.scalar_static_f64[694]*(self.scalar_static_f64[49]*v673));
        let v1526=(self.scalar_static_f64[395]*(if v1483{(v1429*(v1508*v1520))}else{v4}));
        let v1528=(if v1483{(v1525/v1526)}else{v1503});
        let v1529=(v1528<v1437);
        let v1530=(v1528<self.scalar_static_f64[188]);
        let v1531=(v1483&&v1529);
        let v1536=(v1531&&(!v1530));
        let v1537=(if v1536{self.scalar_static_f64[189]}else{v1498});
        let v1568=(v744*self.scalar_static_f64[771]);
        let v1569=(v407*(if v781{(v782*(v1+(v777-self.scalar_static_f64[188])))}else{(if v778{v779}else{v4})}));
        let v1570=(v1568-self.scalar_static_f64[771]);
        let v1572=((v1+v1568)).sqrt();
        let v1573=(v1+v1572);
        let v1576=((v1+v1569)).sqrt();
        let v1577=(v1+v1576);
        let v1598=(self.scalar_static_f64[784]*(v764-v1));
        let v1601=((v1+(v764*self.scalar_static_f64[783]))).sqrt();
        let v1602=(v1+v1601);
        let v1604=(if self.scalar_static_bool[42]{(v1598/v1602)}else{v4});
        let v1616=(if self.scalar_static_bool[44]{(v711-self.scalar_static_f64[792])}else{v4});
        let v1620=(if self.scalar_static_bool[44]{(v1616*v1616)}else{v1156});
        let v1621=(v1616<v4);
        let v1622=(self.scalar_static_bool[44]&&v1621);
        let v1625=((self.scalar_static_f64[228]+v1620)).sqrt();
        let v1626=(v1625-v1616);
        let v1630=(self.scalar_static_bool[44]&&(!v1621));
        let v1633=(if v1630{(v395*(v1616+v1625))}else{(if v1622{(self.scalar_static_f64[229]/v1626)}else{v4})});
        let v1636=(v1633+(self.scalar_static_f64[787]+(self.scalar_static_f64[519]*v1604)));
        let v1641=(if self.scalar_static_bool[46]{v1}else{(if self.scalar_static_bool[44]{(v1633/v1636)}else{v1})});
        let v1700=(v1136<v4);
        let v1702=((v1155+(v1136*v1136))).sqrt();
        let v1703=(v1702-v1136);
        let v1706=(!v1700);
        let v1709=(if v1706{(v395*(v1136+v1702))}else{(if v1700{(v1158/v1703)}else{v4})});
        let v1719=(v1177>v4);
        let v1723=(v673<self.scalar_static_f64[249]);
        let v1726=((-v1177)/self.scalar_static_f64[250]);
        let v1727=(v1726<self.scalar_static_f64[188]);
        let v1729=(v1723&&(v1719&&self.scalar_static_bool[49]));
        let v1730=(v1727&&v1729);
        let v1731=(v1726).exp();
        let v1734=(v1729&&(!v1727));
        let v1735=(if v1734{self.scalar_static_f64[189]}else{v1537});
        let v1739=(if v1734{(v1735*(v1+(v1726-self.scalar_static_f64[188])))}else{(if v1730{v1731}else{v4})});
        let v1740=(self.scalar_static_f64[249]-v673);
        let v1742=(if v1729{(v1739*v1740)}else{v4});
        let v1746=(self.scalar_static_f64[793]*f64::powf(v1742,self.scalar_static_f64[251]));
        let v1747=(v1746<self.scalar_static_f64[188]);
        let v1752=(v1729&&(!v1747));
        let v1753=(if v1752{self.scalar_static_f64[189]}else{v1735});
        let v1766=(v1719&&self.scalar_static_bool[51]);
        let v1870=(v1723&&(self.scalar_static_bool[54]&&(v1766&&self.scalar_static_bool[55])));
        let v1871=f64::powf(v1740,self.scalar_static_f64[251]);
        let v1873=(v1177+self.scalar_static_f64[264]);
        let v1875=(v1-(v1177/v1873));
        let v1877=f64::powf(v1875,self.scalar_static_f64[265]);
        let v1879=(if v1870{(v1871*v1877)}else{v4});
        let v1880=(self.scalar_static_bool[52]&&v1870);
        let v1882=(self.scalar_static_bool[53]&&v1870);
        let v1886=(if v1882{((v1177-self.scalar_static_f64[266])/self.scalar_static_f64[264])}else{v4});
        let v1890=(if v1882{((v1886-v1)/self.scalar_static_f64[267])}else{v1209});
        let v1891=(v1886<v1);
        let v1892=(v1882&&v1891);
        let v1893=(v1890).exp();
        let v1894=(v1+v1893);
        let v1900=(v1882&&(!v1891));
        let v1902=((-v1890)).exp();
        let v1903=(v1+v1902);
        let v1907=(if v1900{(v1886+(self.scalar_static_f64[267]*(v1903).ln()))}else{(if v1892{(v1+(self.scalar_static_f64[267]*(v1894).ln()))}else{v4})});
        let v1909=f64::powf(v1907,self.scalar_static_f64[268]);
        let v1912=(self.scalar_static_f64[793]*(if v1882{(v1879*v1909)}else{(if v1880{v1879}else{v4})}));
        let v1913=(v1912<self.scalar_static_f64[188]);
        let v1918=(v1870&&(!v1913));
        let v1919=(if v1918{self.scalar_static_f64[189]}else{v1753});
        let v1978=((v682-self.scalar_static_f64[760])/self.scalar_static_f64[761]);
        let v1979=(v682<self.scalar_static_f64[760]);
        let v1980=(v1978).exp();
        let v1981=(v1+v1980);
        let v1986=(!v1979);
        let v1988=((-v1978)).exp();
        let v1989=(v1+v1988);
        let v1993=(if v1986{(self.scalar_static_f64[760]-(self.scalar_static_f64[761]*(v1989).ln()))}else{(if v1979{(v682-(self.scalar_static_f64[761]*(v1981).ln()))}else{v4})});
        let v1996=(v1-(self.scalar_static_f64[492]*v1993));
        let v2009=(v1124*self.scalar_static_f64[801]);
        let v2010=(v1709*v2009);
        let v2011=(v1131*self.scalar_static_f64[801]);
        let v2012=(v1709*v2011);
        let v2014=((v706-self.scalar_static_f64[768])/self.scalar_static_f64[757]);
        let v2015=(v706<self.scalar_static_f64[768]);
        let v2016=(v2014).exp();
        let v2017=(v1+v2016);
        let v2022=(!v2015);
        let v2024=((-v2014)).exp();
        let v2025=(v1+v2024);
        let v2029=(if v2022{(self.scalar_static_f64[768]-(self.scalar_static_f64[757]*(v2025).ln()))}else{(if v2015{(v706-(self.scalar_static_f64[757]*(v2017).ln()))}else{v4})});
        let v2031=(v1-(v2029/self.scalar_static_f64[472]));
        let v2046=((v711-self.scalar_static_f64[768])/self.scalar_static_f64[757]);
        let v2047=(v711<self.scalar_static_f64[768]);
        let v2048=(v2046).exp();
        let v2049=(v1+v2048);
        let v2054=(!v2047);
        let v2056=((-v2046)).exp();
        let v2057=(v1+v2056);
        let v2061=(if v2054{(self.scalar_static_f64[768]-(self.scalar_static_f64[757]*(v2057).ln()))}else{(if v2047{(v711-(self.scalar_static_f64[757]*(v2049).ln()))}else{v4})});
        let v2063=(v1-(v2061/self.scalar_static_f64[472]));
        let v2083=(v679/self.scalar_static_f64[806]);
        let v2084=(v2083<self.scalar_static_f64[188]);
        let v2085=(v2083).exp();
        let v2087=(!v2084);
        let v2088=(if v2087{self.scalar_static_f64[189]}else{v1919});
        let v2093=(self.scalar_static_f64[805]*(if v2087{(v2088*(v1+(v2083-self.scalar_static_f64[188])))}else{(if v2084{v2085}else{v1385})}));
        let v2098=(v1027*self.scalar_static_f64[810]);
        let v2099=(v31+v1016);
        let v2113=(self.scalar_static_f64[352]*((v706-self.scalar_static_f64[454])/self.scalar_static_f64[277]));
        let v2114=(v2113<self.scalar_static_f64[188]);
        let v2116=(v2114&&self.scalar_static_bool[60]);
        let v2117=(v2113).exp();
        let v2120=(self.scalar_static_bool[60]&&(!v2114));
        let v2121=(if v2120{self.scalar_static_f64[189]}else{v2088});
        let v2127=(v744*self.scalar_static_f64[812]);
        let v2130=((v1+(v407*(if v2120{(v2121*(v1+(v2113-self.scalar_static_f64[188])))}else{(if v2116{v2117}else{v4})})))).sqrt();
        let v2131=(v1+v2130);
        let v2133=(if self.scalar_static_bool[60]{(v2127/v2131)}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[811]*(((v1570/v1573)*self.scalar_static_f64[800])+((v1569/v1577)*self.scalar_static_f64[809])))/self.scalar_static_f64[722])}else{v4})});
        let v2141=(if self.scalar_static_bool[64]{(v764*self.scalar_static_f64[771])}else{v4});
        let v2142=(v2141-self.scalar_static_f64[771]);
        let v2144=((v1+v2141)).sqrt();
        let v2145=(v1+v2144);
        let v2149=(if self.scalar_static_bool[64]{(v407*(if v770{(v771*(v1+(v766-self.scalar_static_f64[188])))}else{(if v767{v768}else{v4})}))}else{v4});
        let v2151=((v1+v2149)).sqrt();
        let v2152=(v1+v2151);
        let v2164=(self.scalar_static_f64[352]*(v711-self.scalar_static_f64[454]));
        let v2165=(v2164<self.scalar_static_f64[188]);
        let v2167=(v2165&&self.scalar_static_bool[65]);
        let v2168=(v2164).exp();
        let v2171=(self.scalar_static_bool[65]&&(!v2165));
        let v2172=(if v2171{self.scalar_static_f64[189]}else{v2121});
        let v2178=(v764*self.scalar_static_f64[814]);
        let v2181=((v1+(v407*(if v2171{(v2172*(v1+(v2164-self.scalar_static_f64[188])))}else{(if v2167{v2168}else{v4})})))).sqrt();
        let v2182=(v1+v2181);
        let v2184=(if self.scalar_static_bool[65]{(v2178/v2182)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[813]*((self.scalar_static_f64[800]*(if self.scalar_static_bool[64]{(v2142/v2145)}else{v4}))+(self.scalar_static_f64[809]*(if self.scalar_static_bool[64]{(v2149/v2152)}else{v4}))))/self.scalar_static_f64[722])}else{v4})});
        let v2192=(if self.scalar_static_bool[66]{(f64::powf(v1057,self.scalar_static_f64[280])-v154)}else{v4});
        let v2193=(if self.scalar_static_bool[66]{v1040}else{v4});
        let v2194=(v2193<v4);
        let v2195=(self.scalar_static_bool[66]&&v2194);
        let v2196=(v2193).exp();
        let v2197=(v1+v2196);
        let v2201=(self.scalar_static_bool[66]&&(!v2194));
        let v2203=((-v2193)).exp();
        let v2204=(v1+v2203);
        let v2206=(if v2201{(v2203/v2204)}else{(if v2195{(v1/v2197)}else{v4})});
        let v2213=((self.scalar_static_f64[352]*v1120)/self.scalar_static_f64[549]);
        let v2214=(v395/v1122);
        let v2216=(if self.scalar_static_bool[66]{(v2213*v2214)}else{v4});
        let v2217=(v1709*self.scalar_static_f64[801]);
        let v2222=(v684*0.2);
        let v2224=((if self.scalar_static_bool[66]{(v2093/self.scalar_static_f64[806])}else{v4})+((if self.scalar_static_bool[66]{(self.scalar_static_f64[797]*(if self.scalar_static_bool[66]{(v154+(v2192*v2206))}else{v4}))}else{v4})+(if self.scalar_static_bool[66]{(v2216*v2217)}else{v4})));
        let v2233=(if self.scalar_static_bool[66]{(v2010+(v2093*self.scalar_static_f64[281]))}else{v4});
        let v2242=(if self.scalar_static_bool[67]{v2010}else{(if self.scalar_static_bool[66]{(v2233*self.scalar_static_f64[284])}else{v4})});
        let v2243=(if self.scalar_static_bool[67]{v2012}else{(if self.scalar_static_bool[66]{(v2012+(v2233*self.scalar_static_f64[283]))}else{v4})});
        let v2245=(v1174+v1175);
        let v2246=(v2245/v1171);
        let v2254=(v2246>v4);
        let v2255=(v2242+v2243);
        let v2258=(!v2254);
        let v2259=(self.scalar_static_f64[718]*v1709);
        let v2261=(if v2258{(v1171*v2259)}else{(if v2254{(v2255/v2246)}else{v4})});
        let v2274=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(v2261*self.scalar_static_f64[287])}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v2261)}else{v4})})});
        let v2317=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v2093}else{(if self.scalar_static_bool[66]{(v2093*self.scalar_static_f64[282])}else{v4})})+((v1065*self.scalar_static_f64[797])+v2242)));
        let v2320=(self.scalar_static_f64[0]*(self.scalar_static_f64[798]*((self.scalar_static_f64[762]*(v1-f64::powf(v1996,self.scalar_static_f64[207])))+(v154*(v682-v1993)))));
        let v2323=(self.scalar_static_f64[0]*((v2098*v2099)+((v1117*self.scalar_static_f64[799])+v2243)));
        let v2326=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2222*v2224)}else{v4}));
        let v2330=((self.scalar_static_f64[0]*(v690-v687))*self.scalar_static_f64[290]);
        let v2334=(v697*self.scalar_static_f64[291]);
        let v2342=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(v1-f64::powf(v2063,self.scalar_static_f64[211])))+(self.scalar_static_f64[765]*(v711-v2061))))+(self.scalar_static_f64[505]*v711)))))+(if self.scalar_static_bool[63]{(v1641*v2184)}else{v4})));
        let v2348=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*((self.scalar_static_f64[504]*((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(v1-f64::powf(v2031,self.scalar_static_f64[211])))+(self.scalar_static_f64[765]*(v706-v2029))))+(self.scalar_static_f64[505]*v706)))*self.scalar_static_f64[273]))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v2133)}else{v2133})));
        let v2359=ctx.node_voltage(nodes[10]);
        let v2385=(if v729{(v730*self.scalar_static_f64[817])}else{(if v726{(v727*self.scalar_static_f64[817])}else{v4})});
        let v2386=(if v729{(v730*self.scalar_static_f64[818])}else{(if v726{(v727*self.scalar_static_f64[818])}else{v4})});
        let v2401=(if v739{(v740*self.scalar_static_f64[815])}else{(if v736{(v737*self.scalar_static_f64[815])}else{v4})});
        let v2402=(if v739{(v740*self.scalar_static_f64[819])}else{(if v736{(v737*self.scalar_static_f64[819])}else{v4})});
        let v2403=(if v739{(v740*self.scalar_static_f64[820])}else{(if v736{(v737*self.scalar_static_f64[820])}else{v4})});
        let v2404=(if v739{(v740*self.scalar_static_f64[816])}else{(if v736{(v737*self.scalar_static_f64[816])}else{v4})});
        let v2426=(if v759{(v760*self.scalar_static_f64[819])}else{(if v756{(v757*self.scalar_static_f64[819])}else{v4})});
        let v2427=(if v759{(v760*self.scalar_static_f64[821])}else{(if v756{(v757*self.scalar_static_f64[821])}else{v4})});
        let v2428=(if v759{(v760*self.scalar_static_f64[820])}else{(if v756{(v757*self.scalar_static_f64[820])}else{v4})});
        let v2429=(if v759{(v760*self.scalar_static_f64[816])}else{(if v756{(v757*self.scalar_static_f64[816])}else{v4})});
        let v2468=(if v792{(v793*self.scalar_static_f64[815])}else{(if v789{(v790*self.scalar_static_f64[815])}else{v4})});
        let v2469=(if v792{(v793*self.scalar_static_f64[816])}else{(if v789{(v790*self.scalar_static_f64[816])}else{v4})});
        let v2476=(if v803{(v804*self.scalar_static_f64[815])}else{(if v800{(v801*self.scalar_static_f64[815])}else{v4})});
        let v2477=(if v803{(v804*self.scalar_static_f64[816])}else{(if v800{(v801*self.scalar_static_f64[816])}else{v4})});
        let v2480=(v31*v811);
        let v2481=((v407*v2468)/v2480);
        let v2482=((v407*v2469)/v2480);
        let v2485=(v31*v814);
        let v2486=((v407*v2476)/v2485);
        let v2487=((v407*v2477)/v2485);
        let v2493=(v816*v816);
        let v2499=(if v819{v4}else{(((v816*(v31*v2476))-(v815*v2486))/v2493)});
        let v2500=(if v819{v4}else{(((v816*(v31*v2477))-(v815*v2487))/v2493)});
        let v2517=(self.scalar_static_f64[351]*((v2481-v2486)-((((v816*v2481)-(v822*v2486))/v2493)/v823)));
        let v2518=(self.scalar_static_f64[351]*((-v2487)-(((-(v822*v2487))/v2493)/v823)));
        let v2519=(self.scalar_static_f64[351]*(v2482-((v2482/v816)/v823)));
        let v2521=(self.scalar_static_f64[292]+v2519);
        let v2522=(v2517/self.scalar_static_f64[526]);
        let v2523=((self.scalar_static_f64[0]+v2518)/self.scalar_static_f64[526]);
        let v2524=(v2521/self.scalar_static_f64[526]);
        let v2534=(self.scalar_static_f64[526]*(v395*v2522));
        let v2535=(self.scalar_static_f64[526]*(v395*v2523));
        let v2536=(self.scalar_static_f64[526]*(v395*v2524));
        let v2548=(if v829{((self.scalar_static_f64[750]*((self.scalar_static_f64[352]*v2534)/v845))-(if v835{(self.scalar_static_f64[0]/v837)}else{(if v832{self.scalar_static_f64[0]}else{v4})}))}else{v4});
        let v2549=(if v829{((self.scalar_static_f64[750]*((self.scalar_static_f64[352]*v2535)/v845))-(if v835{(self.scalar_static_f64[292]/v837)}else{(if v832{self.scalar_static_f64[292]}else{v4})}))}else{v4});
        let v2550=(if v829{(self.scalar_static_f64[750]*((self.scalar_static_f64[352]*v2536)/v845))}else{v4});
        let v2551=(v850*v2548);
        let v2553=(v850*v2549);
        let v2555=(v850*v2550);
        let v2560=(v31*v862);
        let v2561=((if v829{(v2551+v2551)}else{v4})/v2560);
        let v2562=((if v829{(v2553+v2553)}else{v4})/v2560);
        let v2563=((if v829{(v2555+v2555)}else{v4})/v2560);
        let v2569=(v863*v863);
        let v2586=(if v867{(v395*(v2548+v2561))}else{(if v859{((-(v860*(v2561-v2548)))/v2569)}else{v4})});
        let v2587=(if v867{(v395*(v2549+v2562))}else{(if v859{((-(v860*(v2562-v2549)))/v2569)}else{v4})});
        let v2588=(if v867{(v395*(v2550+v2563))}else{(if v859{((-(v860*(v2563-v2550)))/v2569)}else{v4})});
        let v2604=(v878*v878);
        let v2614=(if v829{(((v878*((v874*v2586)+(v870*v2586)))-(v875*(self.scalar_static_f64[192]*v2586)))/v2604)}else{v4});
        let v2615=(if v829{(((v878*((v874*v2587)+(v870*v2587)))-(v875*(self.scalar_static_f64[192]*v2587)))/v2604)}else{v4});
        let v2616=(if v829{(((v878*((v874*v2588)+(v870*v2588)))-(v875*(self.scalar_static_f64[192]*v2588)))/v2604)}else{v4});
        let v2620=(v880*v880);
        let v2630=(if v829{(((v880*v2522)-(v828*v2614))/v2620)}else{v4});
        let v2631=(if v829{(((v880*v2523)-(v828*v2615))/v2620)}else{v4});
        let v2632=(if v829{(((v880*v2524)-(v828*v2616))/v2620)}else{v4});
        let v2636=(if v829{(v2630/self.scalar_static_f64[194])}else{v4});
        let v2637=(if v829{(v2631/self.scalar_static_f64[194])}else{v4});
        let v2638=(if v829{(v2632/self.scalar_static_f64[194])}else{v4});
        let v2672=(if v829{((if v896{(v2630+(self.scalar_static_f64[194]*((v898*(-v2636))/v899)))}else{(if v888{(self.scalar_static_f64[194]*((v889*v2636)/v890))}else{v4})})/self.scalar_static_f64[200])}else{v4});
        let v2673=(if v829{((if v896{(v2631+(self.scalar_static_f64[194]*((v898*(-v2637))/v899)))}else{(if v888{(self.scalar_static_f64[194]*((v889*v2637)/v890))}else{v4})})/self.scalar_static_f64[200])}else{v4});
        let v2674=(if v829{((if v896{(v2632+(self.scalar_static_f64[194]*((v898*(-v2638))/v899)))}else{(if v888{(self.scalar_static_f64[194]*((v889*v2638)/v890))}else{v4})})/self.scalar_static_f64[200])}else{v4});
        let v2678=(if v829{(v2586/self.scalar_static_f64[193])}else{v4});
        let v2679=(if v829{(v2587/self.scalar_static_f64[193])}else{v4});
        let v2680=(if v829{(v2588/self.scalar_static_f64[193])}else{v4});
        let v2702=(v31*v920);
        let v2721=(v923*v923);
        let v2731=(if v829{(((v923*(((v917*((v915*v2678)+(v914*(v407*v2672))))+(v916*v2678))/v2702))-(v921*((v922*v2678)+(v917*(v31*v2672)))))/v2721)}else{v4});
        let v2732=(if v829{(((v923*(((v917*((v915*v2679)+(v914*(v407*v2673))))+(v916*v2679))/v2702))-(v921*((v922*v2679)+(v917*(v31*v2673)))))/v2721)}else{v4});
        let v2733=(if v829{(((v923*(((v917*((v915*v2680)+(v914*(v407*v2674))))+(v916*v2680))/v2702))-(v921*((v922*v2680)+(v917*(v31*v2674)))))/v2721)}else{v4});
        let v2739=((v925*v2499)+(v820*v2731));
        let v2742=((v925*v2500)+(v820*v2732));
        let v2743=(v820*v2733);
        let v2750=(v929*v929);
        let v2760=(if v829{(((v929*((-v2731)+v2739))-(v928*v2739))/v2750)}else{v4});
        let v2761=(if v829{(((v929*((-v2732)+v2742))-(v928*v2742))/v2750)}else{v4});
        let v2762=(if v829{(((v929*((-v2733)+v2743))-(v928*v2743))/v2750)}else{v4});
        let v2775=(if v829{(self.scalar_static_f64[352]*((v931*v2534)+(v843*v2760)))}else{v4});
        let v2776=(if v829{(self.scalar_static_f64[352]*((v931*v2535)+(v843*v2761)))}else{v4});
        let v2777=(if v829{(self.scalar_static_f64[352]*((v931*v2536)+(v843*v2762)))}else{v4});
        let v2793=(if v829{((v31*v2775)+((v937*v2499)+(v820*(v2499+v2775))))}else{v4});
        let v2794=(if v829{((v31*v2776)+((v937*v2500)+(v820*(v2500+v2776))))}else{v4});
        let v2795=(if v829{((v31*v2777)+(v820*v2777))}else{v4});
        let v2799=(if v829{(v395*v2775)}else{v4});
        let v2800=(if v829{(v395*v2776)}else{v4});
        let v2801=(if v829{(v395*v2777)}else{v4});
        let v2802=(v943*v2799);
        let v2804=(v943*v2800);
        let v2806=(v943*v2801);
        let v2811=(if v829{(v2793+(v2802+v2802))}else{v4});
        let v2812=(if v829{(v2794+(v2804+v2804))}else{v4});
        let v2813=(if v829{(v2795+(v2806+v2806))}else{v4});
        let v2814=(v31*v949);
        let v2815=(v2811/v2814);
        let v2816=(v2812/v2814);
        let v2817=(v2813/v2814);
        let v2830=(v954*v954);
        let v2843=(if v959{v4}else{(if v953{(((v954*v2793)-(v940*(v2815-v2799)))/v2830)}else{(if v948{(v2799+v2815)}else{v4})})});
        let v2844=(if v959{v4}else{(if v953{(((v954*v2794)-(v940*(v2816-v2800)))/v2830)}else{(if v948{(v2800+v2816)}else{v4})})});
        let v2845=(if v959{v4}else{(if v953{(((v954*v2795)-(v940*(v2817-v2801)))/v2830)}else{(if v948{(v2801+v2817)}else{v4})})});
        let v2864=(if v829{(self.scalar_static_f64[202]*v2522)}else{v4});
        let v2865=(if v829{(self.scalar_static_f64[202]*v2523)}else{v4});
        let v2866=(if v829{(self.scalar_static_f64[202]*v2524)}else{v4});
        let v2873=(v970*v2864);
        let v2875=(v970*v2865);
        let v2877=(v970*v2866);
        let v2882=(v31*v977);
        let v2901=(v988*v988);
        let v2917=(self.scalar_static_f64[191]*v2522);
        let v2918=(self.scalar_static_f64[191]*v2523);
        let v2919=(self.scalar_static_f64[191]*v2524);
        let v2923=(v994*v994);
        let v2950=(v822*v822);
        let v2958=(if v999{(((v822*(v31*v2469))-(v1000*v2482))/v2950)}else{v2845});
        let v2959=(if v999{(if v717{(v719*self.scalar_static_f64[815])}else{(if v714{(v715*self.scalar_static_f64[815])}else{v4})})}else{(if v829{(self.scalar_static_f64[754]*((v961*v2843)+(v960*v2843)))}else{v4})});
        let v2960=(if v999{v4}else{(if v829{(self.scalar_static_f64[754]*((v961*v2844)+(v960*v2844)))}else{v4})});
        let v2961=(if v999{(if v717{(v719*self.scalar_static_f64[816])}else{(if v714{(v715*self.scalar_static_f64[816])}else{v4})})}else{(if v829{(self.scalar_static_f64[754]*((v961*v2845)+(v960*v2845)))}else{v4})});
        let v2962=(v2499+(if v999{(((v822*(v31*v2468))-(v1000*v2481))/v2950)}else{v2843}));
        let v2963=(v2500+(if v999{v4}else{v2844}));
        let v2967=(if v1015{(v395*v2962)}else{v4});
        let v2968=(if v1015{(v395*v2963)}else{v4});
        let v2969=(if v1015{(v395*v2958)}else{v4});
        let v2973=(v1019*v1019);
        let v2992=(v1025*v1025);
        let v3002=(if v1023{(((v1025*v2517)-(v826*((self.scalar_static_f64[0]+v2517)-self.scalar_static_f64[0])))/v2992)}else{(if v1015{(((v1019*v2967)-(v1018*v2967))/v2973)}else{v2760})});
        let v3003=(if v1023{(((v1025*v2518)-(v826*(v2518-self.scalar_static_f64[292])))/v2992)}else{(if v1015{(((v1019*v2968)-(v1018*v2968))/v2973)}else{v2761})});
        let v3004=(if v1023{(((v1025*v2519)-(v826*v2521))/v2992)}else{(if v1015{(((v1019*v2969)-(v1018*v2969))/v2973)}else{v2762})});
        let v3008=(if v999{v4}else{(if v986{(self.scalar_static_f64[472]*(((v988*(v31*v2522))-(v987*(v2522+v2614)))/v2901))}else{v4})});
        let v3009=(if v999{v4}else{(if v986{(self.scalar_static_f64[472]*(((v988*(v31*v2523))-(v987*(v2523+v2615)))/v2901))}else{v4})});
        let v3010=(if v999{v4}else{(if v986{(self.scalar_static_f64[472]*(((v988*(v31*v2524))-(v987*(v2524+v2616)))/v2901))}else{v4})});
        let v3011=(if v999{v2522}else{(if v829{(((v994*v2917)-(v993*v2522))/v2923)}else{v4})});
        let v3012=(if v999{v2523}else{(if v829{(((v994*v2918)-(v993*v2523))/v2923)}else{v4})});
        let v3013=(if v999{v2524}else{(if v829{(((v994*v2919)-(v993*v2524))/v2923)}else{v4})});
        let v3020=(if v999{(-(v3011/self.scalar_static_f64[191]))}else{(if v829{((-v2917)/v2923)}else{v4})});
        let v3021=(if v999{(-(v3012/self.scalar_static_f64[191]))}else{(if v829{((-v2918)/v2923)}else{v4})});
        let v3022=(if v999{(-(v3013/self.scalar_static_f64[191]))}else{(if v829{((-v2919)/v2923)}else{v4})});
        let v3045=(if v1048{(-(self.scalar_static_f64[761]*((v1050*self.scalar_static_f64[824])/v1051)))}else{(if v1041{(self.scalar_static_f64[292]-(self.scalar_static_f64[761]*((v1042*self.scalar_static_f64[822])/v1043)))}else{v4})});
        let v3046=(if v1048{(-(self.scalar_static_f64[761]*((v1050*self.scalar_static_f64[825])/v1051)))}else{(if v1041{(self.scalar_static_f64[0]-(self.scalar_static_f64[761]*((v1042*self.scalar_static_f64[823])/v1043)))}else{v4})});
        let v3049=(-(self.scalar_static_f64[492]*v3045));
        let v3050=(-(self.scalar_static_f64[492]*v3046));
        let v3053=(self.scalar_static_f64[207]*f64::powf(v1057,self.scalar_static_f64[296]));
        let v3054=(v3049*v3053);
        let v3055=(v3050*v3053);
        let v3064=((self.scalar_static_f64[762]*(-v3054))+(v154*(self.scalar_static_f64[292]-v3045)));
        let v3065=((self.scalar_static_f64[762]*(-v3055))+(v154*(self.scalar_static_f64[0]-v3046)));
        let v3073=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v999{v4}else{(if v829{(v2864+(((if v829{(self.scalar_static_f64[756]*v2522)}else{v4})+(v2873+v2873))/v2882))}else{v4})}))}else{self.scalar_static_f64[297]})});
        let v3074=(if self.scalar_static_bool[26]{v4}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[292]+(if v999{self.scalar_static_f64[0]}else{(if v829{(v2865+(((if v829{(self.scalar_static_f64[756]*v2523)}else{v4})+(v2875+v2875))/v2882))}else{v4})}))}else{self.scalar_static_f64[298]})});
        let v3075=(if self.scalar_static_bool[26]{self.scalar_static_f64[292]}else{(if self.scalar_static_bool[24]{(if v999{self.scalar_static_f64[292]}else{(if v829{(v2866+(((if v829{(self.scalar_static_f64[756]*v2524)}else{v4})+(v2877+v2877))/v2882))}else{v4})})}else{v4})});
        let v3079=(v1029*v1029);
        let v3080=(((v1029*v3073)-(v1084*v3008))/v3079);
        let v3084=(((v1029*v3074)-(v1084*v3009))/v3079);
        let v3088=(((v1029*v3075)-(v1084*v3010))/v3079);
        let v3131=(if v1093{(-((v1097*v3008)+(v1029*((v1095*(-v3080))/v1096))))}else{(if v1086{(v3073-((v1089*v3008)+(v1029*((v1087*v3080)/v1088))))}else{v4})});
        let v3132=(if v1093{(-((v1097*v3009)+(v1029*((v1095*(-v3084))/v1096))))}else{(if v1086{(v3074-((v1089*v3009)+(v1029*((v1087*v3084)/v1088))))}else{v4})});
        let v3133=(if v1093{(-((v1097*v3010)+(v1029*((v1095*(-v3088))/v1096))))}else{(if v1086{(v3075-((v1089*v3010)+(v1029*((v1087*v3088)/v1088))))}else{v4})});
        let v3136=(self.scalar_static_f64[210]*f64::powf(v1033,self.scalar_static_f64[299]));
        let v3137=(v3020*v3136);
        let v3138=(v3021*v3136);
        let v3139=(v3022*v3136);
        let v3148=(self.scalar_static_f64[211]*f64::powf(v1106,self.scalar_static_f64[300]));
        let v3187=(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((v1107*v3139)+(v1102*((-(v3133/self.scalar_static_f64[472]))*v3148)))))+((v1112*(self.scalar_static_f64[765]*v3139))+(v1111*(v3075-v3133)))));
        let v3190=((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((v1107*v3137)+(v1102*((-(v3131/self.scalar_static_f64[472]))*v3148)))))+((v1112*(self.scalar_static_f64[765]*v3137))+(v1111*(v3073-v3131)))))+self.scalar_static_f64[826]);
        let v3191=((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((v1107*v3138)+(v1102*((-(v3132/self.scalar_static_f64[472]))*v3148)))))+((v1112*(self.scalar_static_f64[765]*v3138))+(v1111*(v3074-v3132)))))+self.scalar_static_f64[827]);
        let v3192=(self.scalar_static_f64[771]*v2385);
        let v3193=(self.scalar_static_f64[771]*v2386);
        let v3194=(v31*v1122);
        let v3195=(v3192/v3194);
        let v3196=(v3193/v3194);
        let v3200=(v1123*v1123);
        let v3201=(((v1123*v3192)-(v1120*v3195))/v3200);
        let v3205=(((v1123*v3193)-(v1120*v3196))/v3200);
        let v3208=(self.scalar_static_f64[772]*f64::powf(v1003,self.scalar_static_f64[828]));
        let v3209=(v2959*v3208);
        let v3210=(v2960*v3208);
        let v3211=(v2961*v3208);
        let v3212=(self.scalar_static_f64[771]*v3209);
        let v3213=(self.scalar_static_f64[771]*v3210);
        let v3214=(self.scalar_static_f64[771]*v3211);
        let v3215=(v31*v1129);
        let v3222=(v1130*v1130);
        let v3223=(((v1130*v3212)-(v1127*(v3212/v3215)))/v3222);
        let v3227=(((v1130*v3213)-(v1127*(v3213/v3215)))/v3222);
        let v3231=(((v1130*v3214)-(v1127*(v3214/v3215)))/v3222);
        let v3232=(v3064/self.scalar_static_f64[709]);
        let v3233=(v3065/self.scalar_static_f64[709]);
        let v3234=(v3190/self.scalar_static_f64[707]);
        let v3235=(v3191/self.scalar_static_f64[707]);
        let v3236=(v3187/self.scalar_static_f64[707]);
        let v3237=(v3233+v3234);
        let v3275=(if self.scalar_static_bool[28]{((v1147*(if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*v3232))}else{v4}))/self.scalar_static_f64[775])}else{(if self.scalar_static_bool[27]{v3232}else{v4})});
        let v3276=(if self.scalar_static_bool[28]{(((v1147*(if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*v3233))}else{v4}))-(v1148*(if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*((-v3190)/self.scalar_static_f64[707])))}else{v4})))/self.scalar_static_f64[775])}else{(if self.scalar_static_bool[27]{v3237}else{v4})});
        let v3277=(if self.scalar_static_bool[28]{((-(v1148*(if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*((-v3191)/self.scalar_static_f64[707])))}else{v4})))/self.scalar_static_f64[775])}else{(if self.scalar_static_bool[27]{v3235}else{v4})});
        let v3278=(if self.scalar_static_bool[28]{((-(v1148*(if self.scalar_static_bool[28]{(self.scalar_static_f64[352]*(self.scalar_static_f64[737]*((-v3187)/self.scalar_static_f64[707])))}else{v4})))/self.scalar_static_f64[775])}else{(if self.scalar_static_bool[27]{v3236}else{v4})});
        let v3279=(v1154*v3275);
        let v3280=(v3279+v3279);
        let v3281=(v1154*v3276);
        let v3282=(v3281+v3281);
        let v3283=(v1154*v3277);
        let v3284=(v3283+v3283);
        let v3285=(v1154*v3278);
        let v3286=(v3285+v3285);
        let v3287=(v31*v1160);
        let v3288=(v3280/v3287);
        let v3289=(v3282/v3287);
        let v3290=(v3284/v3287);
        let v3291=(v3286/v3287);
        let v3298=(v1161*v1161);
        let v3326=(v395*v3201);
        let v3327=(v395*(v3205+v3223));
        let v3328=(v395*v3227);
        let v3329=(v395*v3231);
        let v3332=((v1170*(if v1164{(v395*(v3275+v3288))}else{(if v1157{((-(v1158*(v3288-v3275)))/v3298)}else{v4})}))+(v1167*v3326));
        let v3335=((v1170*(if v1164{(v395*(v3276+v3289))}else{(if v1157{((-(v1158*(v3289-v3276)))/v3298)}else{v4})}))+(v1167*v3327));
        let v3338=((v1170*(if v1164{(v395*(v3277+v3290))}else{(if v1157{((-(v1158*(v3290-v3277)))/v3298)}else{v4})}))+(v1167*v3328));
        let v3341=((v1170*(if v1164{(v395*(v3278+v3291))}else{(if v1157{((-(v1158*(v3291-v3278)))/v3298)}else{v4})}))+(v1167*v3329));
        let v3342=(self.scalar_static_f64[776]*v3209);
        let v3343=(self.scalar_static_f64[776]*v3210);
        let v3344=(self.scalar_static_f64[776]*v3211);
        let v3346=(self.scalar_static_f64[592]*v2386);
        let v3350=(v1171*(self.scalar_static_f64[592]*v2385));
        let v3353=(v1171*v1171);
        let v3387=(if v1186{(self.scalar_static_f64[292]+(v1178*((v1188*self.scalar_static_f64[303])/v1189)))}else{(if v1180{(v1178*((v1181*self.scalar_static_f64[301])/v1182))}else{v4})});
        let v3388=(if v1186{(self.scalar_static_f64[0]+(v1178*((v1188*self.scalar_static_f64[304])/v1189)))}else{(if v1180{(v1178*((v1181*self.scalar_static_f64[302])/v1182))}else{v4})});
        let v3447=(if v1234{(v1235*self.scalar_static_f64[829])}else{(if v1231{(v1232*self.scalar_static_f64[829])}else{v3387})});
        let v3448=(if v1234{(v1235*self.scalar_static_f64[830])}else{(if v1231{(v1232*self.scalar_static_f64[830])}else{v3388})});
        let v3581=(if v1309{(v1310*self.scalar_static_f64[831])}else{(if v1306{(v1307*self.scalar_static_f64[831])}else{v3447})});
        let v3582=(if v1309{(v1310*self.scalar_static_f64[832])}else{(if v1306{(v1307*self.scalar_static_f64[832])}else{v4})});
        let v3583=(if v1309{v4}else{(if v1306{v4}else{v3448})});
        let v3638=(if v1344{(v1345*self.scalar_static_f64[833])}else{(if v1341{(v1342*self.scalar_static_f64[833])}else{v3581})});
        let v3639=(if v1344{v4}else{(if v1341{v4}else{v3582})});
        let v3640=(if v1344{(v1345*self.scalar_static_f64[834])}else{(if v1341{(v1342*self.scalar_static_f64[834])}else{v3583})});
        let v3653=(if v1356{(v1357*self.scalar_static_f64[835])}else{(if v1353{(v1354*self.scalar_static_f64[835])}else{v3638})});
        let v3654=(if v1356{(v1357*self.scalar_static_f64[836])}else{(if v1353{(v1354*self.scalar_static_f64[836])}else{v3639})});
        let v3655=(if v1356{v4}else{(if v1353{v4}else{v3640})});
        let v3676=(if v1368{v4}else{(if v1365{v4}else{v3653})});
        let v3677=(if v1368{(v1369*self.scalar_static_f64[837])}else{(if v1365{(v1366*self.scalar_static_f64[837])}else{v3654})});
        let v3678=(if v1368{(v1369*self.scalar_static_f64[838])}else{(if v1365{(v1366*self.scalar_static_f64[838])}else{v3655})});
        let v3679=(if v1368{(v1369*self.scalar_static_f64[839])}else{(if v1365{(v1366*self.scalar_static_f64[839])}else{v4})});
        let v3680=(if v1368{(v1369*self.scalar_static_f64[840])}else{(if v1365{(v1366*self.scalar_static_f64[840])}else{v4})});
        let v3697=(if v1380{(v1381*self.scalar_static_f64[841])}else{(if v1377{(v1378*self.scalar_static_f64[841])}else{v3676})});
        let v3698=(if v1380{(v1381*self.scalar_static_f64[842])}else{(if v1377{(v1378*self.scalar_static_f64[842])}else{v3677})});
        let v3699=(if v1380{v4}else{(if v1377{v4}else{v3678})});
        let v3700=(if v1380{v4}else{(if v1377{v4}else{v3679})});
        let v3701=(if v1380{v4}else{(if v1377{v4}else{v3680})});
        let v4039=(self.scalar_static_f64[771]*v2401);
        let v4040=(self.scalar_static_f64[771]*v2402);
        let v4041=(self.scalar_static_f64[771]*v2403);
        let v4042=(self.scalar_static_f64[771]*v2404);
        let v4043=(v407*(if v781{(v782*self.scalar_static_f64[815])}else{(if v778{(v779*self.scalar_static_f64[815])}else{v4})}));
        let v4044=(v407*(if v781{(v782*self.scalar_static_f64[819])}else{(if v778{(v779*self.scalar_static_f64[819])}else{v4})}));
        let v4045=(v407*(if v781{(v782*self.scalar_static_f64[820])}else{(if v778{(v779*self.scalar_static_f64[820])}else{v4})}));
        let v4046=(v407*(if v781{(v782*self.scalar_static_f64[816])}else{(if v778{(v779*self.scalar_static_f64[816])}else{v4})}));
        let v4047=(v31*v1572);
        let v4055=(v1573*v1573);
        let v4069=(v31*v1576);
        let v4077=(v1577*v1577);
        let v4137=(v31*v1601);
        let v4145=(v1602*v1602);
        let v4159=(if self.scalar_static_bool[42]{(((v1602*(self.scalar_static_f64[784]*v2426))-(v1598*((self.scalar_static_f64[783]*v2426)/v4137)))/v4145)}else{v4});
        let v4160=(if self.scalar_static_bool[42]{(((v1602*(self.scalar_static_f64[784]*v2427))-(v1598*((self.scalar_static_f64[783]*v2427)/v4137)))/v4145)}else{v4});
        let v4161=(if self.scalar_static_bool[42]{(((v1602*(self.scalar_static_f64[784]*v2428))-(v1598*((self.scalar_static_f64[783]*v2428)/v4137)))/v4145)}else{v4});
        let v4162=(if self.scalar_static_bool[42]{(((v1602*(self.scalar_static_f64[784]*v2429))-(v1598*((self.scalar_static_f64[783]*v2429)/v4137)))/v4145)}else{v4});
        let v4167=(v1616*self.scalar_static_f64[317]);
        let v4168=(v4167+v4167);
        let v4169=(v1616*self.scalar_static_f64[318]);
        let v4171=(v1616*self.scalar_static_f64[319]);
        let v4172=(v4171+v4171);
        let v4173=(v1616*self.scalar_static_f64[320]);
        let v4175=(if self.scalar_static_bool[44]{v4168}else{v4});
        let v4176=(if self.scalar_static_bool[44]{(v4169+v4169)}else{v4});
        let v4177=(if self.scalar_static_bool[44]{v4}else{v3280});
        let v4178=(if self.scalar_static_bool[44]{v4168}else{v3282});
        let v4179=(if self.scalar_static_bool[44]{v4172}else{v3284});
        let v4180=(if self.scalar_static_bool[44]{v4172}else{v3286});
        let v4181=(if self.scalar_static_bool[44]{(v4173+v4173)}else{v4});
        let v4182=(if self.scalar_static_bool[44]{v4172}else{v4});
        let v4183=(v31*v1625);
        let v4184=(v4175/v4183);
        let v4185=(v4176/v4183);
        let v4186=(v4177/v4183);
        let v4187=(v4178/v4183);
        let v4188=(v4179/v4183);
        let v4189=(v4180/v4183);
        let v4190=(v4181/v4183);
        let v4191=(v4182/v4183);
        let v4201=(v1626*v1626);
        let v4247=(if v1630{(v395*(self.scalar_static_f64[317]+v4184))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4184-self.scalar_static_f64[317])))/v4201)}else{v4})});
        let v4248=(if v1630{(v395*(self.scalar_static_f64[318]+v4185))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4185-self.scalar_static_f64[318])))/v4201)}else{v4})});
        let v4249=(if v1630{(v395*v4186)}else{(if v1622{((-(self.scalar_static_f64[229]*v4186))/v4201)}else{v4})});
        let v4250=(if v1630{(v395*(self.scalar_static_f64[317]+v4187))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4187-self.scalar_static_f64[317])))/v4201)}else{v4})});
        let v4251=(if v1630{(v395*(self.scalar_static_f64[319]+v4188))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4188-self.scalar_static_f64[319])))/v4201)}else{v4})});
        let v4252=(if v1630{(v395*(self.scalar_static_f64[319]+v4189))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4189-self.scalar_static_f64[319])))/v4201)}else{v4})});
        let v4253=(if v1630{(v395*(self.scalar_static_f64[320]+v4190))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4190-self.scalar_static_f64[320])))/v4201)}else{v4})});
        let v4254=(if v1630{(v395*(self.scalar_static_f64[319]+v4191))}else{(if v1622{((-(self.scalar_static_f64[229]*(v4191-self.scalar_static_f64[319])))/v4201)}else{v4})});
        let v4255=(self.scalar_static_f64[519]*v4159);
        let v4257=(self.scalar_static_f64[519]*v4161);
        let v4269=(v1636*v1636);
        let v4307=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4247)-(v1633*(v4247+v4255)))/v4269)}else{v4})});
        let v4308=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4248)-(v1633*(v4248+(self.scalar_static_f64[519]*v4160))))/v4269)}else{v4})});
        let v4309=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4249)-(v1633*v4249))/v4269)}else{v4})});
        let v4310=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4250)-(v1633*(v4250+v4255)))/v4269)}else{v4})});
        let v4311=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4251)-(v1633*(v4251+v4257)))/v4269)}else{v4})});
        let v4312=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4252)-(v1633*(v4252+v4257)))/v4269)}else{v4})});
        let v4313=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4253)-(v1633*(v4253+(self.scalar_static_f64[519]*v4162))))/v4269)}else{v4})});
        let v4314=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[44]{(((v1636*v4254)-(v1633*(v4254+v4257)))/v4269)}else{v4})});
        let v4577=(v1136*v3232);
        let v4579=(v1136*v3237);
        let v4581=(v1136*v3235);
        let v4583=(v1136*v3236);
        let v4585=(v31*v1702);
        let v4586=((v4577+v4577)/v4585);
        let v4587=((v4579+v4579)/v4585);
        let v4588=((v4581+v4581)/v4585);
        let v4589=((v4583+v4583)/v4585);
        let v4596=(v1703*v1703);
        let v4619=(if v1706{(v395*(v3232+v4586))}else{(if v1700{((-(v1158*(v4586-v3232)))/v4596)}else{v4})});
        let v4620=(if v1706{(v395*(v3237+v4587))}else{(if v1700{((-(v1158*(v4587-v3237)))/v4596)}else{v4})});
        let v4621=(if v1706{(v395*(v3235+v4588))}else{(if v1700{((-(v1158*(v4588-v3235)))/v4596)}else{v4})});
        let v4622=(if v1706{(v395*(v3236+v4589))}else{(if v1700{((-(v1158*(v4589-v3236)))/v4596)}else{v4})});
        let v5500=(if v1986{(-(self.scalar_static_f64[761]*((v1988*self.scalar_static_f64[824])/v1989)))}else{(if v1979{(self.scalar_static_f64[292]-(self.scalar_static_f64[761]*((v1980*self.scalar_static_f64[822])/v1981)))}else{v4})});
        let v5501=(if v1986{(-(self.scalar_static_f64[761]*((v1988*self.scalar_static_f64[825])/v1989)))}else{(if v1979{(self.scalar_static_f64[0]-(self.scalar_static_f64[761]*((v1980*self.scalar_static_f64[823])/v1981)))}else{v4})});
        let v5507=(self.scalar_static_f64[207]*f64::powf(v1996,self.scalar_static_f64[296]));
        let v5529=((v2009*v4619)+(v1709*(self.scalar_static_f64[801]*v3201)));
        let v5532=((v2009*v4620)+(v1709*(self.scalar_static_f64[801]*v3205)));
        let v5533=(v2009*v4621);
        let v5534=(v2009*v4622);
        let v5538=(v2011*v4619);
        let v5541=((v2011*v4620)+(v1709*(self.scalar_static_f64[801]*v3223)));
        let v5544=((v2011*v4621)+(v1709*(self.scalar_static_f64[801]*v3227)));
        let v5547=((v2011*v4622)+(v1709*(self.scalar_static_f64[801]*v3231)));
        let v5592=(if v2022{(-(self.scalar_static_f64[757]*((v2024*self.scalar_static_f64[859])/v2025)))}else{(if v2015{(self.scalar_static_f64[0]-(self.scalar_static_f64[757]*((v2016*self.scalar_static_f64[855])/v2017)))}else{v4})});
        let v5593=(if v2022{(-(self.scalar_static_f64[757]*((v2024*self.scalar_static_f64[860])/v2025)))}else{(if v2015{(self.scalar_static_f64[293]-(self.scalar_static_f64[757]*((v2016*self.scalar_static_f64[856])/v2017)))}else{v4})});
        let v5594=(if v2022{(-(self.scalar_static_f64[757]*((v2024*self.scalar_static_f64[861])/v2025)))}else{(if v2015{(self.scalar_static_f64[294]-(self.scalar_static_f64[757]*((v2016*self.scalar_static_f64[857])/v2017)))}else{v4})});
        let v5595=(if v2022{(-(self.scalar_static_f64[757]*((v2024*self.scalar_static_f64[862])/v2025)))}else{(if v2015{(self.scalar_static_f64[292]-(self.scalar_static_f64[757]*((v2016*self.scalar_static_f64[858])/v2017)))}else{v4})});
        let v5605=(self.scalar_static_f64[211]*f64::powf(v2031,self.scalar_static_f64[300]));
        let v5690=(if v2054{(-(self.scalar_static_f64[757]*((v2056*self.scalar_static_f64[860])/v2057)))}else{(if v2047{(self.scalar_static_f64[293]-(self.scalar_static_f64[757]*((v2048*self.scalar_static_f64[856])/v2049)))}else{v4})});
        let v5691=(if v2054{(-(self.scalar_static_f64[757]*((v2056*self.scalar_static_f64[866])/v2057)))}else{(if v2047{(self.scalar_static_f64[295]-(self.scalar_static_f64[757]*((v2048*self.scalar_static_f64[865])/v2049)))}else{v4})});
        let v5692=(if v2054{(-(self.scalar_static_f64[757]*((v2056*self.scalar_static_f64[861])/v2057)))}else{(if v2047{(self.scalar_static_f64[294]-(self.scalar_static_f64[757]*((v2048*self.scalar_static_f64[857])/v2049)))}else{v4})});
        let v5693=(if v2054{(-(self.scalar_static_f64[757]*((v2056*self.scalar_static_f64[862])/v2057)))}else{(if v2047{(self.scalar_static_f64[292]-(self.scalar_static_f64[757]*((v2048*self.scalar_static_f64[858])/v2049)))}else{v4})});
        let v5703=(self.scalar_static_f64[211]*f64::powf(v2063,self.scalar_static_f64[300]));
        let v5745=(self.scalar_static_f64[5]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*(self.scalar_static_f64[863]+(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5690/self.scalar_static_f64[472]))*v5703)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[293]-v5690))))))));
        let v5747=(self.scalar_static_f64[5]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*(self.scalar_static_f64[864]+(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5692/self.scalar_static_f64[472]))*v5703)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[294]-v5692))))))));
        let v5765=(self.scalar_static_f64[805]*(if v2087{(v2088*self.scalar_static_f64[868])}else{(if v2084{(v2085*self.scalar_static_f64[868])}else{v3697})}));
        let v5766=(self.scalar_static_f64[805]*(if v2087{v4}else{(if v2084{v4}else{v3698})}));
        let v5767=(self.scalar_static_f64[805]*(if v2087{(v2088*self.scalar_static_f64[869])}else{(if v2084{(v2085*self.scalar_static_f64[869])}else{v3699})}));
        let v5768=(self.scalar_static_f64[805]*(if v2087{v4}else{(if v2084{v4}else{v3700})}));
        let v5769=(self.scalar_static_f64[805]*(if v2087{v4}else{(if v2084{v4}else{v3701})}));
        let v5838=(v31*v2130);
        let v5846=(v2131*v2131);
        let v5860=(if self.scalar_static_bool[60]{(((v2131*(self.scalar_static_f64[812]*v2401))-(v2127*((v407*(if v2120{(v2121*self.scalar_static_f64[870])}else{(if v2116{(v2117*self.scalar_static_f64[870])}else{v4})}))/v5838)))/v5846)}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[811]*((self.scalar_static_f64[800]*(((v1573*v4039)-(v1570*(v4039/v4047)))/v4055))+(self.scalar_static_f64[809]*(((v1577*v4043)-(v1569*(v4043/v4069)))/v4077))))/self.scalar_static_f64[722])}else{v4})});
        let v5861=(if self.scalar_static_bool[60]{(((v2131*(self.scalar_static_f64[812]*v2402))-(v2127*((v407*(if v2120{(v2121*self.scalar_static_f64[871])}else{(if v2116{(v2117*self.scalar_static_f64[871])}else{v4})}))/v5838)))/v5846)}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[811]*((self.scalar_static_f64[800]*(((v1573*v4040)-(v1570*(v4040/v4047)))/v4055))+(self.scalar_static_f64[809]*(((v1577*v4044)-(v1569*(v4044/v4069)))/v4077))))/self.scalar_static_f64[722])}else{v4})});
        let v5862=(if self.scalar_static_bool[60]{(((v2131*(self.scalar_static_f64[812]*v2403))-(v2127*((v407*(if v2120{(v2121*self.scalar_static_f64[872])}else{(if v2116{(v2117*self.scalar_static_f64[872])}else{v4})}))/v5838)))/v5846)}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[811]*((self.scalar_static_f64[800]*(((v1573*v4041)-(v1570*(v4041/v4047)))/v4055))+(self.scalar_static_f64[809]*(((v1577*v4045)-(v1569*(v4045/v4069)))/v4077))))/self.scalar_static_f64[722])}else{v4})});
        let v5863=(if self.scalar_static_bool[60]{(((v2131*(self.scalar_static_f64[812]*v2404))-(v2127*((v407*(if v2120{(v2121*self.scalar_static_f64[873])}else{(if v2116{(v2117*self.scalar_static_f64[873])}else{v4})}))/v5838)))/v5846)}else{(if self.scalar_static_bool[59]{((self.scalar_static_f64[811]*((self.scalar_static_f64[800]*(((v1573*v4042)-(v1570*(v4042/v4047)))/v4055))+(self.scalar_static_f64[809]*(((v1577*v4046)-(v1569*(v4046/v4069)))/v4077))))/self.scalar_static_f64[722])}else{v4})});
        let v5876=(if self.scalar_static_bool[64]{(self.scalar_static_f64[771]*v2426)}else{v4});
        let v5877=(if self.scalar_static_bool[64]{(self.scalar_static_f64[771]*v2427)}else{v4});
        let v5878=(if self.scalar_static_bool[64]{(self.scalar_static_f64[771]*v2428)}else{v4});
        let v5879=(if self.scalar_static_bool[64]{(self.scalar_static_f64[771]*v2429)}else{v4});
        let v5880=(v31*v2144);
        let v5888=(v2145*v2145);
        let v5910=(if self.scalar_static_bool[64]{(v407*(if v770{(v771*self.scalar_static_f64[819])}else{(if v767{(v768*self.scalar_static_f64[819])}else{v4})}))}else{v4});
        let v5911=(if self.scalar_static_bool[64]{(v407*(if v770{(v771*self.scalar_static_f64[821])}else{(if v767{(v768*self.scalar_static_f64[821])}else{v4})}))}else{v4});
        let v5912=(if self.scalar_static_bool[64]{(v407*(if v770{(v771*self.scalar_static_f64[820])}else{(if v767{(v768*self.scalar_static_f64[820])}else{v4})}))}else{v4});
        let v5913=(if self.scalar_static_bool[64]{(v407*(if v770{(v771*self.scalar_static_f64[816])}else{(if v767{(v768*self.scalar_static_f64[816])}else{v4})}))}else{v4});
        let v5914=(v31*v2151);
        let v5922=(v2152*v2152);
        let v5988=(v31*v2181);
        let v5996=(v2182*v2182);
        let v6015=(v1641*(if self.scalar_static_bool[65]{(((v2182*(self.scalar_static_f64[814]*v2426))-(v2178*((v407*(if v2171{(v2172*self.scalar_static_f64[819])}else{(if v2167{(v2168*self.scalar_static_f64[819])}else{v4})}))/v5988)))/v5996)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[813]*((self.scalar_static_f64[800]*(if self.scalar_static_bool[64]{(((v2145*v5876)-(v2142*(v5876/v5880)))/v5888)}else{v4}))+(self.scalar_static_f64[809]*(if self.scalar_static_bool[64]{(((v2152*v5910)-(v2149*(v5910/v5914)))/v5922)}else{v4}))))/self.scalar_static_f64[722])}else{v4})}));
        let v6024=(v1641*(if self.scalar_static_bool[65]{(((v2182*(self.scalar_static_f64[814]*v2428))-(v2178*((v407*(if v2171{(v2172*self.scalar_static_f64[820])}else{(if v2167{(v2168*self.scalar_static_f64[820])}else{v4})}))/v5988)))/v5996)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[813]*((self.scalar_static_f64[800]*(if self.scalar_static_bool[64]{(((v2145*v5878)-(v2142*(v5878/v5880)))/v5888)}else{v4}))+(self.scalar_static_f64[809]*(if self.scalar_static_bool[64]{(((v2152*v5912)-(v2149*(v5912/v5914)))/v5922)}else{v4}))))/self.scalar_static_f64[722])}else{v4})}));
        let v6043=(self.scalar_static_f64[280]*f64::powf(v1057,self.scalar_static_f64[334]));
        let v6053=(v2197*v2197);
        let v6061=(v2203*self.scalar_static_f64[876]);
        let v6062=(v2203*self.scalar_static_f64[877]);
        let v6066=(v2204*v2204);
        let v6092=(v1122*v1122);
        let v6129=(if self.scalar_static_bool[66]{(v5768/self.scalar_static_f64[806])}else{v4});
        let v6168=(self.scalar_static_f64[281]*v5768);
        let v6174=(if self.scalar_static_bool[66]{(v5529+(self.scalar_static_f64[281]*v5765))}else{v4});
        let v6175=(if self.scalar_static_bool[66]{(self.scalar_static_f64[281]*v5766)}else{v4});
        let v6176=(if self.scalar_static_bool[66]{(v5532+(self.scalar_static_f64[281]*v5767))}else{v4});
        let v6177=(if self.scalar_static_bool[66]{(v5533+v6168)}else{v4});
        let v6178=(if self.scalar_static_bool[66]{(v5534+v6168)}else{v4});
        let v6179=(if self.scalar_static_bool[66]{(self.scalar_static_f64[281]*v5769)}else{v4});
        let v6208=(if self.scalar_static_bool[67]{v5529}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6174)}else{v4})});
        let v6209=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6175)}else{v4})});
        let v6210=(if self.scalar_static_bool[67]{v5532}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6176)}else{v4})});
        let v6211=(if self.scalar_static_bool[67]{v5533}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6177)}else{v4})});
        let v6212=(if self.scalar_static_bool[67]{v5534}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6178)}else{v4})});
        let v6213=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[284]*v6179)}else{v4})});
        let v6214=(if self.scalar_static_bool[67]{v5538}else{(if self.scalar_static_bool[66]{(v5538+(self.scalar_static_f64[283]*v6174))}else{v4})});
        let v6215=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[283]*v6175)}else{v4})});
        let v6216=(if self.scalar_static_bool[67]{v5541}else{(if self.scalar_static_bool[66]{(v5541+(self.scalar_static_f64[283]*v6176))}else{v4})});
        let v6217=(if self.scalar_static_bool[67]{v5544}else{(if self.scalar_static_bool[66]{(v5544+(self.scalar_static_f64[283]*v6177))}else{v4})});
        let v6218=(if self.scalar_static_bool[67]{v5547}else{(if self.scalar_static_bool[66]{(v5547+(self.scalar_static_f64[283]*v6178))}else{v4})});
        let v6219=(if self.scalar_static_bool[67]{v4}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[283]*v6179)}else{v4})});
        let v6223=(if self.scalar_static_bool[67]{v5768}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[282]*v5768)}else{v4})});
        let v6241=(v2246*v2246);
        let v6288=(if v2258{((v2259*v3332)+(v1171*(self.scalar_static_f64[718]*v4619)))}else{(if v2254{(((v2246*(v6208+v6214))-(v2255*((v3350-(v2245*v3332))/v3353)))/v6241)}else{v4})});
        let v6289=(if v2258{v4}else{(if v2254{((v6209+v6215)/v2246)}else{v4})});
        let v6290=(if v2258{((v2259*v3335)+(v1171*(self.scalar_static_f64[718]*v4620)))}else{(if v2254{(((v2246*(v6210+v6216))-(v2255*(((v1171*(v3342+v3346))-(v2245*v3335))/v3353)))/v6241)}else{v4})});
        let v6291=(if v2258{((v2259*v3338)+(v1171*(self.scalar_static_f64[718]*v4621)))}else{(if v2254{(((v2246*(v6211+v6217))-(v2255*(((v1171*v3343)-(v2245*v3338))/v3353)))/v6241)}else{v4})});
        let v6292=(if v2258{((v2259*v3341)+(v1171*(self.scalar_static_f64[718]*v4622)))}else{(if v2254{(((v2246*(v6212+v6218))-(v2255*(((v1171*v3344)-(v2245*v3341))/v3353)))/v6241)}else{v4})});
        let v6293=(if v2258{v4}else{(if v2254{((v6213+v6219)/v2246)}else{v4})});
        let v6318=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6288)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6288)}else{v4})})});
        let v6319=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6289)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6289)}else{v4})})});
        let v6320=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6290)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6290)}else{v4})})});
        let v6321=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6291)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6291)}else{v4})})});
        let v6322=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6292)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6292)}else{v4})})});
        let v6323=(if self.scalar_static_bool[75]{v4}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[287]*v6293)}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[283]*v6293)}else{v4})})});
        let v6489=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v5765}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[282]*v5765)}else{v4})})+((self.scalar_static_f64[797]*v3064)+v6208)));
        let v6490=(self.scalar_static_f64[0]*(v6209+(if self.scalar_static_bool[67]{v5766}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[282]*v5766)}else{v4})})));
        let v6491=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v5767}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[282]*v5767)}else{v4})})+((self.scalar_static_f64[797]*v3065)+v6210)));
        let v6492=(self.scalar_static_f64[0]*(v6211+v6223));
        let v6493=(self.scalar_static_f64[0]*(v6212+v6223));
        let v6494=(self.scalar_static_f64[0]*(v6213+(if self.scalar_static_bool[67]{v5769}else{(if self.scalar_static_bool[66]{(self.scalar_static_f64[282]*v5769)}else{v4})})));
        let v6508=(self.scalar_static_f64[0]*(self.scalar_static_f64[798]*((self.scalar_static_f64[762]*(-((-(self.scalar_static_f64[492]*v5500))*v5507)))+(v154*(self.scalar_static_f64[292]-v5500)))));
        let v6509=(self.scalar_static_f64[0]*(self.scalar_static_f64[798]*((self.scalar_static_f64[762]*(-((-(self.scalar_static_f64[492]*v5501))*v5507)))+(v154*(self.scalar_static_f64[0]-v5501)))));
        let v6514=(self.scalar_static_f64[0]*v6214);
        let v6515=(self.scalar_static_f64[0]*v6215);
        let v6516=(self.scalar_static_f64[0]*(((v2099*(self.scalar_static_f64[810]*v3002))+(v2098*v2962))+((self.scalar_static_f64[799]*v3190)+v6216)));
        let v6517=(self.scalar_static_f64[0]*(((v2099*(self.scalar_static_f64[810]*v3003))+(v2098*v2963))+((self.scalar_static_f64[799]*v3191)+v6217)));
        let v6518=(self.scalar_static_f64[0]*(((v2099*(self.scalar_static_f64[810]*v3004))+(v2098*v2958))+((self.scalar_static_f64[799]*v3187)+v6218)));
        let v6519=(self.scalar_static_f64[0]*v6219);
        let v6532=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2222*((if self.scalar_static_bool[66]{(v5765/self.scalar_static_f64[806])}else{v4})+((if self.scalar_static_bool[66]{(self.scalar_static_f64[797]*(if self.scalar_static_bool[66]{((v2206*(if self.scalar_static_bool[66]{(v3049*v6043)}else{v4}))+(v2192*(if v2201{(((v2204*v6061)-(v2203*v6061))/v6066)}else{(if v2195{((-(v2196*self.scalar_static_f64[874]))/v6053)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[66]{((v2217*(if self.scalar_static_bool[66]{((v2214*((self.scalar_static_f64[352]*v3192)/self.scalar_static_f64[549]))+(v2213*((-(v395*v3195))/v6092)))}else{v4}))+(v2216*(self.scalar_static_f64[801]*v4619)))}else{v4}))))}else{v4}));
        let v6533=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{((v2224*self.scalar_static_f64[335])+(v2222*(if self.scalar_static_bool[66]{(v5766/self.scalar_static_f64[806])}else{v4})))}else{v4}));
        let v6534=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{((v2224*self.scalar_static_f64[336])+(v2222*((if self.scalar_static_bool[66]{(v5767/self.scalar_static_f64[806])}else{v4})+((if self.scalar_static_bool[66]{(self.scalar_static_f64[797]*(if self.scalar_static_bool[66]{((v2206*(if self.scalar_static_bool[66]{(v3050*v6043)}else{v4}))+(v2192*(if v2201{(((v2204*v6062)-(v2203*v6062))/v6066)}else{(if v2195{((-(v2196*self.scalar_static_f64[875]))/v6053)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[66]{((v2217*(if self.scalar_static_bool[66]{((v2214*((self.scalar_static_f64[352]*v3193)/self.scalar_static_f64[549]))+(v2213*((-(v395*v3196))/v6092)))}else{v4}))+(v2216*(self.scalar_static_f64[801]*v4620)))}else{v4})))))}else{v4}));
        let v6535=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2222*((if self.scalar_static_bool[66]{(v2216*(self.scalar_static_f64[801]*v4621))}else{v4})+v6129))}else{v4}));
        let v6536=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2222*((if self.scalar_static_bool[66]{(v2216*(self.scalar_static_f64[801]*v4622))}else{v4})+v6129))}else{v4}));
        let v6537=(self.scalar_static_f64[0]*(if self.scalar_static_bool[66]{(v2222*(if self.scalar_static_bool[66]{(v5769/self.scalar_static_f64[806])}else{v4}))}else{v4}));
        let v6590=(self.scalar_static_f64[0]*(v5745+(if self.scalar_static_bool[63]{((v2184*v4307)+v6015)}else{v4})));
        let v6591=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5691/self.scalar_static_f64[472]))*v5703)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[295]-v5691))))+self.scalar_static_f64[867]))))+(if self.scalar_static_bool[63]{((v2184*v4308)+(v1641*(if self.scalar_static_bool[65]{(((v2182*(self.scalar_static_f64[814]*v2427))-(v2178*((v407*(if v2171{(v2172*self.scalar_static_f64[821])}else{(if v2167{(v2168*self.scalar_static_f64[821])}else{v4})}))/v5988)))/v5996)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[813]*((self.scalar_static_f64[800]*(if self.scalar_static_bool[64]{(((v2145*v5877)-(v2142*(v5877/v5880)))/v5888)}else{v4}))+(self.scalar_static_f64[809]*(if self.scalar_static_bool[64]{(((v2152*v5911)-(v2149*(v5911/v5914)))/v5922)}else{v4}))))/self.scalar_static_f64[722])}else{v4})})))}else{v4})));
        let v6592=(self.scalar_static_f64[0]*(if self.scalar_static_bool[63]{(v2184*v4309)}else{v4}));
        let v6593=(self.scalar_static_f64[0]*(v5745+(if self.scalar_static_bool[63]{(v6015+(v2184*v4310))}else{v4})));
        let v6594=(self.scalar_static_f64[0]*(v5747+(if self.scalar_static_bool[63]{((v2184*v4311)+v6024)}else{v4})));
        let v6595=(self.scalar_static_f64[0]*(v5747+(if self.scalar_static_bool[63]{(v6024+(v2184*v4312))}else{v4})));
        let v6596=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*(self.scalar_static_f64[827]+(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5693/self.scalar_static_f64[472]))*v5703)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[292]-v5693))))))))+(if self.scalar_static_bool[63]{((v2184*v4313)+(v1641*(if self.scalar_static_bool[65]{(((v2182*(self.scalar_static_f64[814]*v2429))-(v2178*((v407*(if v2171{(v2172*self.scalar_static_f64[816])}else{(if v2167{(v2168*self.scalar_static_f64[816])}else{v4})}))/v5988)))/v5996)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[813]*((self.scalar_static_f64[800]*(if self.scalar_static_bool[64]{(((v2145*v5879)-(v2142*(v5879/v5880)))/v5888)}else{v4}))+(self.scalar_static_f64[809]*(if self.scalar_static_bool[64]{(((v2152*v5913)-(v2149*(v5913/v5914)))/v5922)}else{v4}))))/self.scalar_static_f64[722])}else{v4})})))}else{v4})));
        let v6597=(self.scalar_static_f64[0]*(v5747+(if self.scalar_static_bool[63]{(v6024+(v2184*v4314))}else{v4})));
        let v6637=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*(self.scalar_static_f64[826]+(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5592/self.scalar_static_f64[472]))*v5605)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[0]-v5592))))))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v5860)}else{v5860})));
        let v6638=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5593/self.scalar_static_f64[472]))*v5605)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[293]-v5593))))+self.scalar_static_f64[863]))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v5861)}else{v5861})));
        let v6639=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*((self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5594/self.scalar_static_f64[472]))*v5605)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[294]-v5594))))+self.scalar_static_f64[864]))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v5862)}else{v5862})));
        let v6640=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[273]*(self.scalar_static_f64[504]*(self.scalar_static_f64[827]+(self.scalar_static_f64[764]*((self.scalar_static_f64[769]*(-((-(v5595/self.scalar_static_f64[472]))*v5605)))+(self.scalar_static_f64[765]*(self.scalar_static_f64[292]-v5595))))))))+(if self.scalar_static_bool[63]{(self.scalar_static_f64[6]*v5863)}else{v5863})));

        CommonStampValues {
            v1,
            v4,
            v30,
            v31,
            v46,
            v154,
            v392,
            v395,
            v407,
            v433,
            v673,
            v677,
            v679,
            v684,
            v687,
            v692,
            v700,
            v703,
            v706,
            v710,
            v744,
            v745,
            v746,
            v749,
            v750,
            v828,
            v946,
            v1003,
            v1027,
            v1030,
            v1033,
            v1059,
            v1135,
            v1170,
            v1171,
            v1176,
            v1177,
            v1195,
            v1196,
            v1199,
            v1200,
            v1209,
            v1239,
            v1241,
            v1242,
            v1247,
            v1248,
            v1255,
            v1256,
            v1257,
            v1262,
            v1264,
            v1314,
            v1316,
            v1317,
            v1322,
            v1323,
            v1349,
            v1361,
            v1373,
            v1385,
            v1391,
            v1392,
            v1395,
            v1396,
            v1401,
            v1402,
            v1408,
            v1412,
            v1415,
            v1423,
            v1424,
            v1425,
            v1427,
            v1429,
            v1433,
            v1434,
            v1436,
            v1438,
            v1439,
            v1440,
            v1445,
            v1446,
            v1483,
            v1485,
            v1487,
            v1488,
            v1491,
            v1492,
            v1497,
            v1498,
            v1503,
            v1506,
            v1508,
            v1516,
            v1517,
            v1518,
            v1520,
            v1525,
            v1526,
            v1528,
            v1529,
            v1530,
            v1531,
            v1536,
            v1537,
            v1604,
            v1620,
            v1641,
            v1709,
            v1719,
            v1729,
            v1730,
            v1731,
            v1734,
            v1735,
            v1739,
            v1740,
            v1742,
            v1746,
            v1747,
            v1752,
            v1753,
            v1766,
            v1870,
            v1871,
            v1873,
            v1875,
            v1877,
            v1879,
            v1880,
            v1882,
            v1890,
            v1892,
            v1893,
            v1894,
            v1900,
            v1902,
            v1903,
            v1907,
            v1909,
            v1912,
            v1913,
            v1918,
            v1919,
            v2246,
            v2274,
            v2317,
            v2320,
            v2323,
            v2326,
            v2330,
            v2334,
            v2342,
            v2348,
            v2359,
            v2401,
            v2402,
            v2403,
            v2404,
            v2522,
            v2523,
            v2524,
            v2811,
            v2812,
            v2813,
            v2959,
            v2960,
            v2961,
            v3002,
            v3003,
            v3004,
            v3011,
            v3012,
            v3013,
            v3020,
            v3021,
            v3022,
            v3054,
            v3055,
            v3234,
            v3235,
            v3236,
            v3326,
            v3327,
            v3328,
            v3329,
            v3332,
            v3335,
            v3338,
            v3341,
            v3342,
            v3343,
            v3344,
            v3346,
            v3350,
            v3353,
            v3387,
            v3388,
            v3447,
            v3448,
            v3581,
            v3582,
            v3583,
            v3638,
            v3639,
            v3640,
            v3653,
            v3654,
            v3655,
            v3676,
            v3677,
            v3678,
            v3679,
            v3680,
            v3697,
            v3698,
            v3699,
            v3700,
            v3701,
            v4159,
            v4160,
            v4161,
            v4162,
            v4175,
            v4176,
            v4177,
            v4178,
            v4179,
            v4180,
            v4181,
            v4182,
            v4307,
            v4308,
            v4309,
            v4310,
            v4311,
            v4312,
            v4313,
            v4314,
            v4619,
            v4620,
            v4621,
            v4622,
            v6318,
            v6319,
            v6320,
            v6321,
            v6322,
            v6323,
            v6489,
            v6490,
            v6491,
            v6492,
            v6493,
            v6494,
            v6508,
            v6509,
            v6514,
            v6515,
            v6516,
            v6517,
            v6518,
            v6519,
            v6532,
            v6533,
            v6534,
            v6535,
            v6536,
            v6537,
            v6590,
            v6591,
            v6592,
            v6593,
            v6594,
            v6595,
            v6596,
            v6597,
            v6637,
            v6638,
            v6639,
            v6640,
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
        let v747=(common.v745).exp();
        let v1197=(common.v1195).exp();
        let v1204=(if common.v1199{(common.v1200*(common.v1+(common.v1195-self.scalar_static_f64[188])))}else{(if common.v1196{v1197}else{common.v4})});
        let v1210=(common.v679<self.scalar_static_f64[214]);
        let v1211=(common.v1209).exp();
        let v1212=(common.v1+v1211);
        let v1217=(!v1210);
        let v1219=((-common.v1209)).exp();
        let v1220=(common.v1+v1219);
        let v1224=(if v1217{(self.scalar_static_f64[214]-(common.v30*(v1220).ln()))}else{(if v1210{(common.v679-(common.v30*(v1212).ln()))}else{common.v4})});
        let v1226=(v1224*self.scalar_static_f64[215]);
        let v1227=(self.scalar_static_f64[214]-v1224);
        let v1228=f64::powf(v1227,common.v31);
        let v1243=(self.scalar_static_bool[12]&&common.v1242);
        let v1244=(common.v1241).exp();
        let v1252=(if common.v1247{(common.v1248*(common.v1+(common.v1241-self.scalar_static_f64[188])))}else{(if v1243{v1244}else{common.v1195})});
        let v1258=(self.scalar_static_bool[12]&&common.v1257);
        let v1259=(common.v1255).exp();
        let v1268=(if common.v1262{(common.v1264*(common.v1+(common.v1255-common.v1256)))}else{(if v1258{v1259}else{v1204})});
        let v1269=(common.v1239-common.v1);
        let v1270=(self.scalar_static_f64[620]*v1269);
        let v1272=(v1269*self.scalar_static_f64[777]);
        let v1275=((common.v1+(common.v407*v1252))).sqrt();
        let v1276=(common.v1+v1275);
        let v1277=(v1272/v1276);
        let v1278=(common.v1+common.v1135);
        let v1282=(self.scalar_static_f64[635]*(common.v1003-common.v1));
        let v1283=(v1268*v1282);
        let v1284=(common.v1+v1268);
        let v1299=(self.scalar_static_f64[216]*((common.v1003+common.v1239)-common.v31));
        let v1318=(self.scalar_static_bool[12]&&common.v1317);
        let v1319=(common.v1316).exp();
        let v1328=(common.v1314-common.v1);
        let v1329=(self.scalar_static_f64[626]*v1328);
        let v1331=(v1328*self.scalar_static_f64[778]);
        let v1334=((common.v1+(common.v407*(if common.v1322{(common.v1323*(common.v1+(common.v1316-self.scalar_static_f64[188])))}else{(if v1318{v1319}else{v1252})})))).sqrt();
        let v1335=(common.v1+v1334);
        let v1375=(self.scalar_static_f64[612]*(common.v1373-common.v1));
        let v1397=(common.v1391&&common.v1396);
        let v1398=(common.v1395).exp();
        let v1406=(if common.v1401{(common.v1402*(common.v1+(common.v1395-self.scalar_static_f64[188])))}else{(if v1397{v1398}else{common.v4})});
        let v1441=(common.v1439&&common.v1440);
        let v1442=(common.v1436).exp();
        let v1451=(-common.v679);
        let v1452=(common.v1-(if common.v1445{(common.v1446*(common.v1+(common.v1436-self.scalar_static_f64[188])))}else{(if v1441{v1442}else{common.v4})}));
        let v1454=(common.v1+(v1452/common.v1436));
        let v1458=(common.v1391&&(!common.v1438));
        let v1459=(common.v395*common.v679);
        let v1460=(common.v1436*v1459);
        let v1461=0.3333333333333333;
        let v1462=(common.v1436*v1461);
        let v1463=0.25;
        let v1465=(common.v1+(common.v1436*v1463));
        let v1467=(common.v1+(v1462*v1465));
        let v1471=((if v1458{(v1460*v1467)}else{(if common.v1440{(v1451*v1454)}else{common.v4})})*self.scalar_static_f64[779]);
        let v1472=(common.v1059*v1471);
        let v1477=(!common.v1391);
        let v1493=(common.v1483&&common.v1492);
        let v1494=(common.v1491).exp();
        let v1502=(if common.v1497{(common.v1498*(common.v1+(common.v1491-self.scalar_static_f64[188])))}else{(if v1493{v1494}else{common.v4})});
        let v1532=(common.v1530&&common.v1531);
        let v1533=(common.v1528).exp();
        let v1542=(-common.v673);
        let v1543=(common.v1-(if common.v1536{(common.v1537*(common.v1+(common.v1528-self.scalar_static_f64[188])))}else{(if v1532{v1533}else{common.v4})}));
        let v1545=(common.v1+(v1543/common.v1528));
        let v1549=(common.v1483&&(!common.v1529));
        let v1550=(common.v395*common.v673);
        let v1551=(common.v1528*v1550);
        let v1552=(v1461*common.v1528);
        let v1554=(common.v1+(v1463*common.v1528));
        let v1556=(common.v1+(v1552*v1554));
        let v1560=((if v1549{(v1551*v1556)}else{(if common.v1531{(v1542*v1545)}else{common.v4})})*self.scalar_static_f64[780]);
        let v1561=(common.v1487*v1560);
        let v1566=(!common.v1483);
        let v1567=(if v1566{common.v4}else{(if common.v1483{(self.scalar_static_f64[50]*(self.scalar_static_f64[493]*(v1502*v1561)))}else{common.v4})});
        let v1581=(self.scalar_static_f64[781]*(common.v744-common.v1));
        let v1586=((common.v1+(common.v744*self.scalar_static_f64[783]))).sqrt();
        let v1587=(common.v1+v1586);
        let v1588=(v1581/v1587);
        let v1594=(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v1588)}else{v1588});
        let v1643=(if self.scalar_static_bool[42]{(common.v1604*common.v1641)}else{common.v4});
        let v1647=(if self.scalar_static_bool[47]{(common.v673+common.v684)}else{common.v4});
        let v1649=(-v1647);
        let v1652=(v1649<common.v4);
        let v1653=(self.scalar_static_bool[47]&&v1652);
        let v1656=((self.scalar_static_f64[231]+(if self.scalar_static_bool[47]{(v1647*v1647)}else{common.v1620}))).sqrt();
        let v1657=(v1656-v1649);
        let v1661=(self.scalar_static_bool[47]&&(!v1652));
        let v1664=(if v1661{(common.v395*(v1649+v1656))}else{(if v1653{(self.scalar_static_f64[232]/v1657)}else{common.v4})});
        let v1680=(v1664<self.scalar_static_f64[240]);
        let v1681=(self.scalar_static_bool[47]&&v1680);
        let v1682=(v1664/self.scalar_static_f64[238]);
        let v1684=(common.v1-f64::powf(v1682,self.scalar_static_f64[233]));
        let v1688=(self.scalar_static_bool[47]&&(!v1680));
        let v1694=(if self.scalar_static_bool[48]{common.v1}else{(if v1688{(self.scalar_static_f64[237]+(self.scalar_static_f64[247]*(v1664-self.scalar_static_f64[240])))}else{(if v1681{(common.v1/v1684)}else{common.v4})})});
        let v1710=(common.v1170*common.v1709);
        let v1711=(self.scalar_static_f64[512]/v1710);
        let v1712=(v1711<self.scalar_static_f64[14]);
        let v1714=(common.v154*(if v1712{self.scalar_static_f64[14]}else{v1711}));
        let v1717=(common.v684+(self.scalar_static_f64[750]*((if common.v749{(common.v750*(common.v1+(common.v745-self.scalar_static_f64[188])))}else{(if common.v746{v747}else{common.v4})})-common.v1)));
        let v1748=(common.v1729&&common.v1747);
        let v1749=(common.v1746).exp();
        let v1757=(if common.v1752{(common.v1753*(common.v1+(common.v1746-self.scalar_static_f64[188])))}else{(if v1748{v1749}else{common.v4})});
        let v1760=(common.v1742*self.scalar_static_f64[794]);
        let v1768=((common.v673<self.scalar_static_f64[436])&&(self.scalar_static_bool[50]&&common.v1766));
        let v1774=(if v1768{self.scalar_static_f64[257]}else{common.v4});
        let v1775=(self.scalar_static_f64[436]-common.v673);
        let v1777=(if v1768{(v1775/common.v1033)}else{common.v946});
        let v1780=(((common.v31*v1777)/v1774)).sqrt();
        let v1781=(if v1768{v1780}else{common.v4});
        let v1784=(v1768&&self.scalar_static_bool[52]);
        let v1787=(v1768&&self.scalar_static_bool[53]);
        let v1790=(if v1787{(common.v1-(common.v395*common.v1027))}else{common.v4});
        let v1791=(self.scalar_static_f64[255]*v1790);
        let v1793=(if v1787{(v1790*v1791)}else{(if v1784{self.scalar_static_f64[255]}else{common.v4})});
        let v1794=(v1781*v1793);
        let v1798=(((v1781*v1781)+(v1793*v1793))).sqrt();
        let v1800=(if v1768{(v1794/v1798)}else{common.v4});
        let v1802=(if v1768{(v1775/v1800)}else{common.v4});
        let v1803=(common.v395*v1800);
        let v1804=(v1774*v1803);
        let v1807=(if v1768{(v1802+(common.v1033*v1804))}else{common.v4});
        let v1820=(self.scalar_static_f64[191]*(if v1787{(common.v1+(self.scalar_static_f64[260]*(common.v1+(common.v31*common.v1027))))}else{common.v4}));
        let v1822=((if v1787{self.scalar_static_f64[263]}else{common.v4})-(common.v1177/v1820));
        let v1825=(if v1787{(v1802-(v1804*v1822))}else{common.v4});
        let v1826=(v1825-v1807);
        let v1828=(common.v46*v1802);
        let v1829=(v1802*v1828);
        let v1835=((if v1787{((v1826*v1826)+((common.v1030*v1829)/self.scalar_static_f64[191]))}else{v1777})).sqrt();
        let v1838=(if v1787{(common.v395*((v1807+v1825)+v1835))}else{(if v1784{v1807}else{common.v4})});
        let v1839=(v1838-v1802);
        let v1841=(if v1768{(v1839/v1838)}else{common.v4});
        let v1844=((v1841).abs()>1e-7);
        let v1845=(v1768&&v1844);
        let v1847=(if v1845{(v1803/v1841)}else{common.v4});
        let v1849=(v1838*self.scalar_static_f64[795]);
        let v1850=(v1847*v1849);
        let v1852=(self.scalar_static_f64[796]/v1838);
        let v1853=(v1852).exp();
        let v1855=(common.v1+(v1793/v1847));
        let v1857=((v1852*v1855)).exp();
        let v1858=(v1853-v1857);
        let v1862=(v1768&&(!v1844));
        let v1863=(self.scalar_static_f64[3]*v1793);
        let v1914=(common.v1870&&common.v1913);
        let v1915=(common.v1912).exp();
        let v1923=(if common.v1918{(common.v1919*(common.v1+(common.v1912-self.scalar_static_f64[188])))}else{(if v1914{v1915}else{v1757})});
        let v1924=(common.v1740*self.scalar_static_f64[794]);
        let v1926=(if common.v1870{(v1923*v1924)}else{(if v1862{(v1853*v1863)}else{(if v1845{(v1850*v1858)}else{(if common.v1729{(v1757*v1760)}else{common.v4})})})});
        let v1930=(common.v1719&&(v1926>common.v4));
        let v1931=(self.scalar_static_bool[56]&&v1930);
        let v1932=(self.scalar_static_f64[516]+v1714);
        let v1933=(common.v1177*v1932);
        let v1940=(if v1931{(((self.scalar_static_f64[351]/v1933)+(self.scalar_static_f64[620]*(common.v1171/self.scalar_static_f64[592])))+(self.scalar_static_f64[509]/v1932))}else{common.v4});
        let v1941=(self.scalar_static_bool[54]&&v1931);
        let v1944=(if v1941{((v1926-v1940)/common.v392)}else{common.v1890});
        let v1945=(v1926<v1940);
        let v1946=(v1941&&v1945);
        let v1947=(v1944).exp();
        let v1948=(common.v1+v1947);
        let v1954=(v1941&&(!v1945));
        let v1956=((-v1944)).exp();
        let v1957=(common.v1+v1956);
        let v1961=(if v1954{(v1940-(common.v392*(v1957).ln()))}else{(if v1946{(v1926-(common.v392*(v1948).ln()))}else{v1926})});
        let v1962=(common.v1177*v1961);
        let v1965=(v1931&&self.scalar_static_bool[57]);
        let v1966=(v1940*v1962);
        let v1967=(v1940+v1961);
        let v1971=(v1930&&self.scalar_static_bool[58]);
        let v1972=(if v1971{v1962}else{(if v1965{(v1966/v1967)}else{(if v1941{v1962}else{common.v4})})});
        let v2253=(if self.scalar_static_bool[69]{common.v4}else{(if self.scalar_static_bool[68]{((v1972/common.v2246)).abs()}else{common.v4})});
        let v2304=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v1694))));
        let v2318=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2317);
        let v2321=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2320);
        let v2324=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2323);
        let v2327=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2326);
        let v2331=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2330);
        let v2335=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2334);
        let v2343=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2342);
        let v2349=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2348);
        let v2360=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2359);
        let v3354=((common.v3350-(common.v1176*common.v3332))/common.v3353);
        let v3358=(((common.v1171*(common.v3346-common.v3342))-(common.v1176*common.v3335))/common.v3353);
        let v3362=(((common.v1171*(-common.v3343))-(common.v1176*common.v3338))/common.v3353);
        let v3366=(((common.v1171*(-common.v3344))-(common.v1176*common.v3341))/common.v3353);
        let v3389=(common.v3387/self.scalar_static_f64[213]);
        let v3390=(common.v3388/self.scalar_static_f64[213]);
        let v3397=(if common.v1199{(common.v1200*v3389)}else{(if common.v1196{(v1197*v3389)}else{common.v4})});
        let v3398=(if common.v1199{(common.v1200*v3390)}else{(if common.v1196{(v1197*v3390)}else{common.v4})});
        let v3423=(if v1217{(-(common.v30*((v1219*self.scalar_static_f64[307])/v1220)))}else{(if v1210{(self.scalar_static_f64[292]-(common.v30*((v1211*self.scalar_static_f64[305])/v1212)))}else{common.v4})});
        let v3424=(if v1217{(-(common.v30*((v1219*self.scalar_static_f64[308])/v1220)))}else{(if v1210{(self.scalar_static_f64[0]-(common.v30*((v1211*self.scalar_static_f64[306])/v1212)))}else{common.v4})});
        let v3430=(common.v31*f64::powf(v1227,common.v1));
        let v3455=(if common.v1247{(common.v1248*self.scalar_static_f64[816])}else{(if v1243{(v1244*self.scalar_static_f64[816])}else{v3389})});
        let v3456=(if common.v1247{(common.v1248*self.scalar_static_f64[815])}else{(if v1243{(v1244*self.scalar_static_f64[815])}else{v3390})});
        let v3457=(v3354/self.scalar_static_f64[592]);
        let v3458=(v3358/self.scalar_static_f64[592]);
        let v3459=(v3362/self.scalar_static_f64[592]);
        let v3460=(v3366/self.scalar_static_f64[592]);
        let v3473=(if common.v1262{(common.v1264*v3457)}else{(if v1258{(v1259*v3457)}else{v3397})});
        let v3474=(if common.v1262{(common.v1264*v3458)}else{(if v1258{(v1259*v3458)}else{v3398})});
        let v3475=(if common.v1262{(common.v1264*v3459)}else{(if v1258{(v1259*v3459)}else{common.v4})});
        let v3476=(if common.v1262{(common.v1264*v3460)}else{(if v1258{(v1259*v3460)}else{common.v4})});
        let v3477=(self.scalar_static_f64[620]*common.v3447);
        let v3478=(self.scalar_static_f64[620]*common.v3448);
        let v3483=(common.v31*v1275);
        let v3489=(v1276*v1276);
        let v3519=(v1284*v1284);
        let v3594=(self.scalar_static_f64[626]*common.v3581);
        let v3595=(self.scalar_static_f64[626]*common.v3582);
        let v3596=(self.scalar_static_f64[626]*common.v3583);
        let v3603=(common.v31*v1334);
        let v3610=(v1335*v1335);
        let v3711=(common.v1392*common.v1392);
        let v3718=(self.scalar_static_f64[674]*(-((-(self.scalar_static_f64[18]*(common.v31*common.v3054)))/v3711)));
        let v3719=(self.scalar_static_f64[674]*(-((-(self.scalar_static_f64[18]*(common.v31*common.v3055)))/v3711)));
        let v3730=(if common.v1391{self.scalar_static_f64[843]}else{common.v4});
        let v3731=(if common.v1391{self.scalar_static_f64[844]}else{common.v4});
        let v3732=(common.v1408*v3730);
        let v3734=(common.v1408*v3731);
        let v3736=(common.v31*common.v1412);
        let v3741=(self.scalar_static_f64[218]*f64::powf(common.v1412,self.scalar_static_f64[309]));
        let v3787=(common.v1434*common.v1434);
        let v3793=(if common.v1391{(((common.v1434*self.scalar_static_f64[845])-(common.v1433*(self.scalar_static_f64[375]*(if common.v1391{(common.v1429*((common.v1427*(((v3732+v3732)/v3736)*v3741))+(common.v1415*((self.scalar_static_f64[16]*(-(self.scalar_static_f64[221]*(common.v154*v3730))))-((common.v1425*((common.v1423*v3730)+(common.v1408*(common.v433*v3730))))+(common.v1424*v3730))))))}else{common.v4}))))/v3787)}else{v3730});
        let v3794=(if common.v1391{(((common.v1434*self.scalar_static_f64[846])-(common.v1433*(self.scalar_static_f64[375]*(if common.v1391{(common.v1429*((common.v1427*(((v3734+v3734)/v3736)*v3741))+(common.v1415*((self.scalar_static_f64[16]*(-(self.scalar_static_f64[221]*(common.v154*v3731))))-((common.v1425*((common.v1423*v3731)+(common.v1408*(common.v433*v3731))))+(common.v1424*v3731))))))}else{common.v4}))))/v3787)}else{v3731});
        let v3808=(common.v1436*common.v1436);
        let v3875=(self.scalar_static_f64[211]*f64::powf(common.v1485,self.scalar_static_f64[300]));
        let v3878=(if common.v1483{(self.scalar_static_f64[849]*v3875)}else{common.v4});
        let v3879=(if common.v1483{(self.scalar_static_f64[850]*v3875)}else{common.v4});
        let v3884=(common.v1488*common.v1488);
        let v3891=(self.scalar_static_f64[694]*(-((-(self.scalar_static_f64[49]*(common.v31*v3878)))/v3884)));
        let v3892=(self.scalar_static_f64[694]*(-((-(self.scalar_static_f64[49]*(common.v31*v3879)))/v3884)));
        let v3901=(if common.v1483{self.scalar_static_f64[847]}else{common.v4});
        let v3902=(if common.v1483{self.scalar_static_f64[848]}else{common.v4});
        let v3903=(common.v1503*v3901);
        let v3905=(common.v1503*v3902);
        let v3907=(common.v31*common.v1506);
        let v3912=(self.scalar_static_f64[222]*f64::powf(common.v1506,self.scalar_static_f64[314]));
        let v3958=(common.v1526*common.v1526);
        let v3964=(if common.v1483{(((common.v1526*self.scalar_static_f64[851])-(common.v1525*(self.scalar_static_f64[395]*(if common.v1483{(common.v1429*((common.v1520*(((v3903+v3903)/v3907)*v3912))+(common.v1508*((self.scalar_static_f64[47]*(-(self.scalar_static_f64[225]*(common.v154*v3901))))-((common.v1518*((common.v1516*v3901)+(common.v1503*(common.v433*v3901))))+(common.v1517*v3901))))))}else{common.v4}))))/v3958)}else{v3901});
        let v3965=(if common.v1483{(((common.v1526*self.scalar_static_f64[852])-(common.v1525*(self.scalar_static_f64[395]*(if common.v1483{(common.v1429*((common.v1520*(((v3905+v3905)/v3907)*v3912))+(common.v1508*((self.scalar_static_f64[47]*(-(self.scalar_static_f64[225]*(common.v154*v3902))))-((common.v1518*((common.v1516*v3902)+(common.v1503*(common.v433*v3902))))+(common.v1517*v3902))))))}else{common.v4}))))/v3958)}else{v3902});
        let v3979=(common.v1528*common.v1528);
        let v4099=(common.v31*v1586);
        let v4107=(v1587*v1587);
        let v4108=(((v1587*(self.scalar_static_f64[781]*common.v2401))-(v1581*((self.scalar_static_f64[783]*common.v2401)/v4099)))/v4107);
        let v4112=(((v1587*(self.scalar_static_f64[781]*common.v2402))-(v1581*((self.scalar_static_f64[783]*common.v2402)/v4099)))/v4107);
        let v4116=(((v1587*(self.scalar_static_f64[781]*common.v2403))-(v1581*((self.scalar_static_f64[783]*common.v2403)/v4099)))/v4107);
        let v4120=(((v1587*(self.scalar_static_f64[781]*common.v2404))-(v1581*((self.scalar_static_f64[783]*common.v2404)/v4099)))/v4107);
        let v4315=(common.v1641*common.v4159);
        let v4324=(common.v1641*common.v4161);
        let v4348=(v1647*self.scalar_static_f64[321]);
        let v4350=(v1647*self.scalar_static_f64[322]);
        let v4352=(v1647*self.scalar_static_f64[323]);
        let v4363=(common.v31*v1656);
        let v4364=((if self.scalar_static_bool[47]{common.v4}else{common.v4175})/v4363);
        let v4365=((if self.scalar_static_bool[47]{common.v4}else{common.v4176})/v4363);
        let v4366=((if self.scalar_static_bool[47]{common.v4}else{common.v4177})/v4363);
        let v4367=((if self.scalar_static_bool[47]{(v4348+v4348)}else{common.v4175})/v4363);
        let v4368=((if self.scalar_static_bool[47]{(v4350+v4350)}else{common.v4178})/v4363);
        let v4369=((if self.scalar_static_bool[47]{(v4352+v4352)}else{common.v4179})/v4363);
        let v4370=((if self.scalar_static_bool[47]{common.v4}else{common.v4180})/v4363);
        let v4371=((if self.scalar_static_bool[47]{common.v4}else{common.v4181})/v4363);
        let v4372=((if self.scalar_static_bool[47]{common.v4}else{common.v4182})/v4363);
        let v4378=(v1657*v1657);
        let v4425=(if v1661{(common.v395*v4364)}else{(if v1653{((-(self.scalar_static_f64[232]*v4364))/v4378)}else{common.v4})});
        let v4426=(if v1661{(common.v395*v4365)}else{(if v1653{((-(self.scalar_static_f64[232]*v4365))/v4378)}else{common.v4})});
        let v4427=(if v1661{(common.v395*v4366)}else{(if v1653{((-(self.scalar_static_f64[232]*v4366))/v4378)}else{common.v4})});
        let v4428=(if v1661{(common.v395*(self.scalar_static_f64[324]+v4367))}else{(if v1653{((-(self.scalar_static_f64[232]*(v4367-self.scalar_static_f64[324])))/v4378)}else{common.v4})});
        let v4429=(if v1661{(common.v395*(self.scalar_static_f64[325]+v4368))}else{(if v1653{((-(self.scalar_static_f64[232]*(v4368-self.scalar_static_f64[325])))/v4378)}else{common.v4})});
        let v4430=(if v1661{(common.v395*(self.scalar_static_f64[326]+v4369))}else{(if v1653{((-(self.scalar_static_f64[232]*(v4369-self.scalar_static_f64[326])))/v4378)}else{common.v4})});
        let v4431=(if v1661{(common.v395*v4370)}else{(if v1653{((-(self.scalar_static_f64[232]*v4370))/v4378)}else{common.v4})});
        let v4432=(if v1661{(common.v395*v4371)}else{(if v1653{((-(self.scalar_static_f64[232]*v4371))/v4378)}else{common.v4})});
        let v4433=(if v1661{(common.v395*v4372)}else{(if v1653{((-(self.scalar_static_f64[232]*v4372))/v4378)}else{common.v4})});
        let v4444=(self.scalar_static_f64[233]*f64::powf(v1682,self.scalar_static_f64[242]));
        let v4454=(v1684*v1684);
        let v4491=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4425)}else{(if v1681{(((v4425/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4492=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4426)}else{(if v1681{(((v4426/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4493=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4427)}else{(if v1681{(((v4427/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4494=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4428)}else{(if v1681{(((v4428/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4495=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4429)}else{(if v1681{(((v4429/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4496=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4430)}else{(if v1681{(((v4430/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4497=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4431)}else{(if v1681{(((v4431/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4498=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4432)}else{(if v1681{(((v4432/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4499=(if self.scalar_static_bool[48]{common.v4}else{(if v1688{(self.scalar_static_f64[247]*v4433)}else{(if v1681{(((v4433/self.scalar_static_f64[238])*v4444)/v4454)}else{common.v4})})});
        let v4522=(v1694*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v4116)}else{v4116}));
        let v4542=(v1694*(self.scalar_static_f64[612]*common.v3679));
        let v4551=(v1694*(if self.scalar_static_bool[42]{(v4315+(common.v1604*common.v4307))}else{common.v4}));
        let v4637=(v1710*v1710);
        let v4652=(common.v154*(if v1712{common.v4}else{((-(self.scalar_static_f64[512]*((common.v1709*common.v3326)+(common.v1170*common.v4619))))/v4637)}));
        let v4653=(common.v154*(if v1712{common.v4}else{((-(self.scalar_static_f64[512]*((common.v1709*common.v3327)+(common.v1170*common.v4620))))/v4637)}));
        let v4654=(common.v154*(if v1712{common.v4}else{((-(self.scalar_static_f64[512]*((common.v1709*common.v3328)+(common.v1170*common.v4621))))/v4637)}));
        let v4655=(common.v154*(if v1712{common.v4}else{((-(self.scalar_static_f64[512]*((common.v1709*common.v3329)+(common.v1170*common.v4622))))/v4637)}));
        let v4662=(v1714*v1714);
        let v4679=((-v3354)/self.scalar_static_f64[250]);
        let v4680=((-v3358)/self.scalar_static_f64[250]);
        let v4681=((-v3362)/self.scalar_static_f64[250]);
        let v4682=((-v3366)/self.scalar_static_f64[250]);
        let v4707=(if common.v1729{(common.v1740*(if common.v1734{(common.v1735*v4679)}else{(if common.v1730{(common.v1731*v4679)}else{common.v4})}))}else{common.v4});
        let v4708=(if common.v1729{((common.v1740*(if common.v1734{(common.v1735*v4680)}else{(if common.v1730{(common.v1731*v4680)}else{common.v4})}))+(common.v1739*self.scalar_static_f64[292]))}else{common.v4});
        let v4709=(if common.v1729{((common.v1740*(if common.v1734{(common.v1735*v4681)}else{(if common.v1730{(common.v1731*v4681)}else{common.v4})}))+(self.scalar_static_f64[0]*common.v1739))}else{common.v4});
        let v4710=(if common.v1729{(common.v1740*(if common.v1734{(common.v1735*v4682)}else{(if common.v1730{(common.v1731*v4682)}else{common.v4})}))}else{common.v4});
        let v4713=(self.scalar_static_f64[251]*f64::powf(common.v1742,self.scalar_static_f64[327]));
        let v4718=(self.scalar_static_f64[793]*(v4707*v4713));
        let v4719=(self.scalar_static_f64[793]*(v4708*v4713));
        let v4720=(self.scalar_static_f64[793]*(v4709*v4713));
        let v4721=(self.scalar_static_f64[793]*(v4710*v4713));
        let v4734=(if common.v1752{(common.v1753*v4718)}else{(if v1748{(v1749*v4718)}else{common.v4})});
        let v4735=(if common.v1752{(common.v1753*v4719)}else{(if v1748{(v1749*v4719)}else{common.v4})});
        let v4736=(if common.v1752{(common.v1753*v4720)}else{(if v1748{(v1749*v4720)}else{common.v4})});
        let v4737=(if common.v1752{(common.v1753*v4721)}else{(if v1748{(v1749*v4721)}else{common.v4})});
        let v4761=(common.v1033*common.v1033);
        let v4770=(if v1768{(((common.v1033*self.scalar_static_f64[292])-(v1775*common.v3020))/v4761)}else{common.v2811});
        let v4771=(if v1768{(((self.scalar_static_f64[0]*common.v1033)-(v1775*common.v3021))/v4761)}else{common.v2812});
        let v4772=(if v1768{((-(v1775*common.v3022))/v4761)}else{common.v2813});
        let v4779=(common.v31*v1780);
        let v4783=(if v1768{(((common.v31*v4770)/v1774)/v4779)}else{common.v4});
        let v4784=(if v1768{(((common.v31*v4771)/v1774)/v4779)}else{common.v4});
        let v4785=(if v1768{(((common.v31*v4772)/v1774)/v4779)}else{common.v4});
        let v4792=(if v1787{(-(common.v395*common.v3002))}else{common.v4});
        let v4793=(if v1787{(-(common.v395*common.v3003))}else{common.v4});
        let v4794=(if v1787{(-(common.v395*common.v3004))}else{common.v4});
        let v4807=(if v1787{((v1791*v4792)+(v1790*(self.scalar_static_f64[255]*v4792)))}else{common.v4});
        let v4808=(if v1787{((v1791*v4793)+(v1790*(self.scalar_static_f64[255]*v4793)))}else{common.v4});
        let v4809=(if v1787{((v1791*v4794)+(v1790*(self.scalar_static_f64[255]*v4794)))}else{common.v4});
        let v4819=(v1781*v4783);
        let v4821=(v1781*v4784);
        let v4823=(v1781*v4785);
        let v4825=(v1793*v4807);
        let v4827=(v1793*v4808);
        let v4829=(v1793*v4809);
        let v4834=(common.v31*v1798);
        let v4841=(v1798*v1798);
        let v4851=(if v1768{(((v1798*((v1793*v4783)+(v1781*v4807)))-(v1794*(((v4819+v4819)+(v4825+v4825))/v4834)))/v4841)}else{common.v4});
        let v4852=(if v1768{(((v1798*((v1793*v4784)+(v1781*v4808)))-(v1794*(((v4821+v4821)+(v4827+v4827))/v4834)))/v4841)}else{common.v4});
        let v4853=(if v1768{(((v1798*((v1793*v4785)+(v1781*v4809)))-(v1794*(((v4823+v4823)+(v4829+v4829))/v4834)))/v4841)}else{common.v4});
        let v4857=(v1800*v1800);
        let v4866=(if v1768{(((v1800*self.scalar_static_f64[292])-(v1775*v4851))/v4857)}else{common.v4});
        let v4867=(if v1768{(((self.scalar_static_f64[0]*v1800)-(v1775*v4852))/v4857)}else{common.v4});
        let v4868=(if v1768{((-(v1775*v4853))/v4857)}else{common.v4});
        let v4869=(common.v395*v4851);
        let v4870=(common.v395*v4852);
        let v4871=(common.v395*v4853);
        let v4872=(v1774*v4869);
        let v4873=(v1774*v4870);
        let v4874=(v1774*v4871);
        let v4887=(if v1768{(v4866+((v1804*common.v3020)+(common.v1033*v4872)))}else{common.v4});
        let v4888=(if v1768{(v4867+((v1804*common.v3021)+(common.v1033*v4873)))}else{common.v4});
        let v4889=(if v1768{(v4868+((v1804*common.v3022)+(common.v1033*v4874)))}else{common.v4});
        let v4909=(v1820*v1820);
        let v4937=(if v1787{(-(v1804*(-(v3354/v1820))))}else{common.v4});
        let v4938=(if v1787{(v4866-((v1822*v4872)+(v1804*(-(((v1820*v3358)-(common.v1177*(self.scalar_static_f64[191]*(if v1787{(self.scalar_static_f64[260]*(common.v31*common.v3002))}else{common.v4}))))/v4909)))))}else{common.v4});
        let v4939=(if v1787{(v4867-((v1822*v4873)+(v1804*(-(((v1820*v3362)-(common.v1177*(self.scalar_static_f64[191]*(if v1787{(self.scalar_static_f64[260]*(common.v31*common.v3003))}else{common.v4}))))/v4909)))))}else{common.v4});
        let v4940=(if v1787{(v4868-((v1822*v4874)+(v1804*(-(((v1820*v3366)-(common.v1177*(self.scalar_static_f64[191]*(if v1787{(self.scalar_static_f64[260]*(common.v31*common.v3004))}else{common.v4}))))/v4909)))))}else{common.v4});
        let v4944=(v1826*v4937);
        let v4946=(v1826*(v4938-v4887));
        let v4948=(v1826*(v4939-v4888));
        let v4950=(v1826*(v4940-v4889));
        let v4986=(common.v31*v1835);
        let v4999=(if v1787{(common.v395*(v4937+((if v1787{(v4944+v4944)}else{common.v4})/v4986)))}else{common.v4});
        let v5000=(if v1787{(common.v395*((v4887+v4938)+((if v1787{((v4946+v4946)+(((v1829*common.v3011)+(common.v1030*((v1828*v4866)+(v1802*(common.v46*v4866)))))/self.scalar_static_f64[191]))}else{v4770})/v4986)))}else{(if v1784{v4887}else{common.v4})});
        let v5001=(if v1787{(common.v395*((v4888+v4939)+((if v1787{((v4948+v4948)+(((v1829*common.v3012)+(common.v1030*((v1828*v4867)+(v1802*(common.v46*v4867)))))/self.scalar_static_f64[191]))}else{v4771})/v4986)))}else{(if v1784{v4888}else{common.v4})});
        let v5002=(if v1787{(common.v395*((v4889+v4940)+((if v1787{((v4950+v4950)+(((v1829*common.v3013)+(common.v1030*((v1828*v4868)+(v1802*(common.v46*v4868)))))/self.scalar_static_f64[191]))}else{v4772})/v4986)))}else{(if v1784{v4889}else{common.v4})});
        let v5009=(v1838*v1838);
        let v5029=(v1841*v1841);
        let v5043=(if v1845{((-(v1803*(if v1768{(((v1838*v4999)-(v1839*v4999))/v5009)}else{common.v4})))/v5029)}else{common.v4});
        let v5044=(if v1845{(((v1841*v4869)-(v1803*(if v1768{(((v1838*(v5000-v4866))-(v1839*v5000))/v5009)}else{common.v4})))/v5029)}else{common.v4});
        let v5045=(if v1845{(((v1841*v4870)-(v1803*(if v1768{(((v1838*(v5001-v4867))-(v1839*v5001))/v5009)}else{common.v4})))/v5029)}else{common.v4});
        let v5046=(if v1845{(((v1841*v4871)-(v1803*(if v1768{(((v1838*(v5002-v4868))-(v1839*v5002))/v5009)}else{common.v4})))/v5029)}else{common.v4});
        let v5065=((-(self.scalar_static_f64[796]*v4999))/v5009);
        let v5068=((-(self.scalar_static_f64[796]*v5000))/v5009);
        let v5071=((-(self.scalar_static_f64[796]*v5001))/v5009);
        let v5074=((-(self.scalar_static_f64[796]*v5002))/v5009);
        let v5075=(v1853*v5065);
        let v5076=(v1853*v5068);
        let v5077=(v1853*v5071);
        let v5078=(v1853*v5074);
        let v5081=(v1847*v1847);
        let v5149=(self.scalar_static_f64[251]*f64::powf(common.v1740,self.scalar_static_f64[327]));
        let v5155=(common.v1873*common.v1873);
        let v5175=(self.scalar_static_f64[265]*f64::powf(common.v1875,self.scalar_static_f64[328]));
        let v5188=(if common.v1870{(common.v1871*((-(((common.v1873*v3354)-(common.v1177*v3354))/v5155))*v5175))}else{common.v4});
        let v5189=(if common.v1870{((common.v1877*(self.scalar_static_f64[292]*v5149))+(common.v1871*((-(((common.v1873*v3358)-(common.v1177*v3358))/v5155))*v5175)))}else{common.v4});
        let v5190=(if common.v1870{((common.v1877*(self.scalar_static_f64[0]*v5149))+(common.v1871*((-(((common.v1873*v3362)-(common.v1177*v3362))/v5155))*v5175)))}else{common.v4});
        let v5191=(if common.v1870{(common.v1871*((-(((common.v1873*v3366)-(common.v1177*v3366))/v5155))*v5175))}else{common.v4});
        let v5200=(if common.v1882{(v3354/self.scalar_static_f64[264])}else{common.v4});
        let v5201=(if common.v1882{(v3358/self.scalar_static_f64[264])}else{common.v4});
        let v5202=(if common.v1882{(v3362/self.scalar_static_f64[264])}else{common.v4});
        let v5203=(if common.v1882{(v3366/self.scalar_static_f64[264])}else{common.v4});
        let v5208=(if common.v1882{(v5200/self.scalar_static_f64[267])}else{self.scalar_static_f64[305]});
        let v5209=(if common.v1882{(v5201/self.scalar_static_f64[267])}else{self.scalar_static_f64[306]});
        let v5210=(if common.v1882{(v5202/self.scalar_static_f64[267])}else{common.v4});
        let v5211=(if common.v1882{(v5203/self.scalar_static_f64[267])}else{common.v4});
        let v5254=(self.scalar_static_f64[268]*f64::powf(common.v1907,self.scalar_static_f64[329]));
        let v5275=(self.scalar_static_f64[793]*(if common.v1882{((common.v1909*v5188)+(common.v1879*((if common.v1900{(v5200+(self.scalar_static_f64[267]*((common.v1902*(-v5208))/common.v1903)))}else{(if common.v1892{(self.scalar_static_f64[267]*((common.v1893*v5208)/common.v1894))}else{common.v4})})*v5254)))}else{(if common.v1880{v5188}else{common.v4})}));
        let v5276=(self.scalar_static_f64[793]*(if common.v1882{((common.v1909*v5189)+(common.v1879*((if common.v1900{(v5201+(self.scalar_static_f64[267]*((common.v1902*(-v5209))/common.v1903)))}else{(if common.v1892{(self.scalar_static_f64[267]*((common.v1893*v5209)/common.v1894))}else{common.v4})})*v5254)))}else{(if common.v1880{v5189}else{common.v4})}));
        let v5277=(self.scalar_static_f64[793]*(if common.v1882{((common.v1909*v5190)+(common.v1879*((if common.v1900{(v5202+(self.scalar_static_f64[267]*((common.v1902*(-v5210))/common.v1903)))}else{(if common.v1892{(self.scalar_static_f64[267]*((common.v1893*v5210)/common.v1894))}else{common.v4})})*v5254)))}else{(if common.v1880{v5190}else{common.v4})}));
        let v5278=(self.scalar_static_f64[793]*(if common.v1882{((common.v1909*v5191)+(common.v1879*((if common.v1900{(v5203+(self.scalar_static_f64[267]*((common.v1902*(-v5211))/common.v1903)))}else{(if common.v1892{(self.scalar_static_f64[267]*((common.v1893*v5211)/common.v1894))}else{common.v4})})*v5254)))}else{(if common.v1880{v5191}else{common.v4})}));
        let v5305=(if common.v1870{(v1924*(if common.v1918{(common.v1919*v5275)}else{(if v1914{(v1915*v5275)}else{v4734})}))}else{(if v1862{(v1863*v5075)}else{(if v1845{((v1858*((v1849*v5043)+(v1847*(self.scalar_static_f64[795]*v4999))))+(v1850*(v5075-(v1857*((v1855*v5065)+(v1852*((-(v1793*v5043))/v5081)))))))}else{(if common.v1729{((v1760*v4734)+(v1757*(self.scalar_static_f64[794]*v4707)))}else{common.v4})})})});
        let v5306=(if common.v1870{((v1924*(if common.v1918{(common.v1919*v5276)}else{(if v1914{(v1915*v5276)}else{v4735})}))+(v1923*self.scalar_static_f64[853]))}else{(if v1862{((v1863*v5076)+(v1853*(self.scalar_static_f64[3]*v4807)))}else{(if v1845{((v1858*((v1849*v5044)+(v1847*(self.scalar_static_f64[795]*v5000))))+(v1850*(v5076-(v1857*((v1855*v5068)+(v1852*(((v1847*v4807)-(v1793*v5044))/v5081)))))))}else{(if common.v1729{((v1760*v4735)+(v1757*(self.scalar_static_f64[794]*v4708)))}else{common.v4})})})});
        let v5307=(if common.v1870{((v1924*(if common.v1918{(common.v1919*v5277)}else{(if v1914{(v1915*v5277)}else{v4736})}))+(v1923*self.scalar_static_f64[854]))}else{(if v1862{((v1863*v5077)+(v1853*(self.scalar_static_f64[3]*v4808)))}else{(if v1845{((v1858*((v1849*v5045)+(v1847*(self.scalar_static_f64[795]*v5001))))+(v1850*(v5077-(v1857*((v1855*v5071)+(v1852*(((v1847*v4808)-(v1793*v5045))/v5081)))))))}else{(if common.v1729{((v1760*v4736)+(v1757*(self.scalar_static_f64[794]*v4709)))}else{common.v4})})})});
        let v5308=(if common.v1870{(v1924*(if common.v1918{(common.v1919*v5278)}else{(if v1914{(v1915*v5278)}else{v4737})}))}else{(if v1862{((v1863*v5078)+(v1853*(self.scalar_static_f64[3]*v4809)))}else{(if v1845{((v1858*((v1849*v5046)+(v1847*(self.scalar_static_f64[795]*v5002))))+(v1850*(v5078-(v1857*((v1855*v5074)+(v1852*(((v1847*v4809)-(v1793*v5046))/v5081)))))))}else{(if common.v1729{((v1760*v4737)+(v1757*(self.scalar_static_f64[794]*v4710)))}else{common.v4})})})});
        let v5323=(v1933*v1933);
        let v5348=(v1932*v1932);
        let v5363=(if v1931{((((-(self.scalar_static_f64[351]*((v1932*v3354)+(common.v1177*v4652))))/v5323)+(self.scalar_static_f64[620]*(common.v3332/self.scalar_static_f64[592])))+((-(self.scalar_static_f64[509]*v4652))/v5348))}else{common.v4});
        let v5364=(if v1931{((((-(self.scalar_static_f64[351]*((v1932*v3358)+(common.v1177*v4653))))/v5323)+(self.scalar_static_f64[620]*(common.v3335/self.scalar_static_f64[592])))+((-(self.scalar_static_f64[509]*v4653))/v5348))}else{common.v4});
        let v5365=(if v1931{((((-(self.scalar_static_f64[351]*((v1932*v3362)+(common.v1177*v4654))))/v5323)+(self.scalar_static_f64[620]*(common.v3338/self.scalar_static_f64[592])))+((-(self.scalar_static_f64[509]*v4654))/v5348))}else{common.v4});
        let v5366=(if v1931{((((-(self.scalar_static_f64[351]*((v1932*v3366)+(common.v1177*v4655))))/v5323)+(self.scalar_static_f64[620]*(common.v3341/self.scalar_static_f64[592])))+((-(self.scalar_static_f64[509]*v4655))/v5348))}else{common.v4});
        let v5375=(if v1941{((v5305-v5363)/common.v392)}else{v5208});
        let v5376=(if v1941{((v5306-v5364)/common.v392)}else{v5209});
        let v5377=(if v1941{((v5307-v5365)/common.v392)}else{v5210});
        let v5378=(if v1941{((v5308-v5366)/common.v392)}else{v5211});
        let v5419=(if v1954{(v5363-(common.v392*((v1956*(-v5375))/v1957)))}else{(if v1946{(v5305-(common.v392*((v1947*v5375)/v1948)))}else{v5305})});
        let v5420=(if v1954{(v5364-(common.v392*((v1956*(-v5376))/v1957)))}else{(if v1946{(v5306-(common.v392*((v1947*v5376)/v1948)))}else{v5306})});
        let v5421=(if v1954{(v5365-(common.v392*((v1956*(-v5377))/v1957)))}else{(if v1946{(v5307-(common.v392*((v1947*v5377)/v1948)))}else{v5307})});
        let v5422=(if v1954{(v5366-(common.v392*((v1956*(-v5378))/v1957)))}else{(if v1946{(v5308-(common.v392*((v1947*v5378)/v1948)))}else{v5308})});
        let v5425=((v1961*v3354)+(common.v1177*v5419));
        let v5428=((v1961*v3358)+(common.v1177*v5420));
        let v5431=((v1961*v3362)+(common.v1177*v5421));
        let v5434=((v1961*v3366)+(common.v1177*v5422));
        let v5458=(v1967*v1967);
        let v6396=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[658]*common.v3700)));
        let v6400=((((if self.scalar_static_bool[33]{(self.scalar_static_f64[620]*((self.scalar_static_f64[217]*common.v3447)+(v1278*(self.scalar_static_f64[216]*common.v3447))))}else{(if self.scalar_static_bool[31]{v3477}else{(if self.scalar_static_bool[12]{((v3477+(v1278*(((v1276*(self.scalar_static_f64[777]*common.v3447))-(v1272*((common.v407*v3455)/v3483)))/v3489)))+(((v1284*(v1282*v3473))-(v1283*v3473))/v3519))}else{common.v4})})})+(self.scalar_static_f64[605]*common.v3638))+self.scalar_static_f64[337])-(if v1477{common.v4}else{(if common.v1391{(self.scalar_static_f64[19]*(self.scalar_static_f64[492]*((v1472*(if common.v1401{(common.v1402*v3718)}else{(if v1397{(v1398*v3718)}else{common.v4})}))+(v1406*((v1471*common.v3054)+(common.v1059*(self.scalar_static_f64[779]*(if v1458{((v1467*((v1459*v3793)+(common.v1436*self.scalar_static_f64[312])))+(v1460*((v1465*(v1461*v3793))+(v1462*(v1463*v3793)))))}else{(if common.v1440{((self.scalar_static_f64[0]*v1454)+(v1451*(((common.v1436*(-(if common.v1445{(common.v1446*v3793)}else{(if v1441{(v1442*v3793)}else{common.v4})})))-(v1452*v3793))/v3808)))}else{common.v4})}))))))))}else{common.v4})}));
        let v6401=((((if self.scalar_static_bool[33]{(self.scalar_static_f64[620]*((self.scalar_static_f64[217]*common.v3448)+((v1299*common.v3234)+(v1278*(self.scalar_static_f64[216]*(common.v2959+common.v3448))))))}else{(if self.scalar_static_bool[31]{v3478}else{(if self.scalar_static_bool[12]{((v3478+((v1278*(((v1276*(self.scalar_static_f64[777]*common.v3448))-(v1272*((common.v407*v3456)/v3483)))/v3489))+(v1277*common.v3234)))+(((v1284*((v1282*v3474)+(v1268*(self.scalar_static_f64[635]*common.v2959))))-(v1283*v3474))/v3519))}else{common.v4})})})+(self.scalar_static_f64[605]*common.v3640))+self.scalar_static_f64[338])-(if v1477{common.v4}else{(if common.v1391{(self.scalar_static_f64[19]*(self.scalar_static_f64[492]*((v1472*(if common.v1401{(common.v1402*v3719)}else{(if v1397{(v1398*v3719)}else{common.v4})}))+(v1406*((v1471*common.v3055)+(common.v1059*(self.scalar_static_f64[779]*(if v1458{((v1467*((v1459*v3794)+(common.v1436*self.scalar_static_f64[313])))+(v1460*((v1465*(v1461*v3794))+(v1462*(v1463*v3794)))))}else{(if common.v1440{((v1454*self.scalar_static_f64[292])+(v1451*(((common.v1436*(-(if common.v1445{(common.v1446*v3794)}else{(if v1441{(v1442*v3794)}else{common.v4})})))-(v1452*v3794))/v3808)))}else{common.v4})}))))))))}else{common.v4})}));
        let v6434=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4491))));
        let v6435=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4492))));
        let v6436=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4493))));
        let v6437=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4494))));
        let v6438=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-((v1694*(if v1566{common.v4}else{(if common.v1483{(self.scalar_static_f64[50]*(self.scalar_static_f64[493]*((v1561*(if common.v1497{(common.v1498*v3891)}else{(if v1493{(v1494*v3891)}else{common.v4})}))+(v1502*((v1560*v3878)+(common.v1487*(self.scalar_static_f64[780]*(if v1549{((v1556*((v1550*v3964)+(common.v1528*self.scalar_static_f64[313])))+(v1551*((v1554*(v1461*v3964))+(v1552*(v1463*v3964)))))}else{(if common.v1531{((v1545*self.scalar_static_f64[292])+(v1542*(((common.v1528*(-(if common.v1536{(common.v1537*v3964)}else{(if v1532{(v1533*v3964)}else{common.v4})})))-(v1543*v3964))/v3979)))}else{common.v4})}))))))))}else{common.v4})}))+(v1567*v4495)))));
        let v6439=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-((v1694*(if v1566{common.v4}else{(if common.v1483{(self.scalar_static_f64[50]*(self.scalar_static_f64[493]*((v1561*(if common.v1497{(common.v1498*v3892)}else{(if v1493{(v1494*v3892)}else{common.v4})}))+(v1502*((v1560*v3879)+(common.v1487*(self.scalar_static_f64[780]*(if v1549{((v1556*((v1550*v3965)+(common.v1528*self.scalar_static_f64[312])))+(v1551*((v1554*(v1461*v3965))+(v1552*(v1463*v3965)))))}else{(if common.v1531{((self.scalar_static_f64[0]*v1545)+(v1542*(((common.v1528*(-(if common.v1536{(common.v1537*v3965)}else{(if v1532{(v1533*v3965)}else{common.v4})})))-(v1543*v3965))/v3979)))}else{common.v4})}))))))))}else{common.v4})}))+(v1567*v4496)))));
        let v6440=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4497))));
        let v6441=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4498))));
        let v6442=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1567*v4499))));
        let v6495=ddt_scale;
        let v6606=(self.scalar_static_f64[13]*(v6495*common.v6590));
        let v6647=(self.scalar_static_f64[13]*(v6495*common.v6639));

        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v828))),
            5,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2522))),
            6,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2523))),
            7,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2524))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v1177))),
            [3, 5, 6, 7],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3354)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3358)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3362)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3366))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[658]*(common.v1385-common.v1))+((if self.scalar_static_bool[30]{v1329}else{(if self.scalar_static_bool[12]{(v1329+(v1331/v1335))}else{common.v4})})+(self.scalar_static_f64[652]*(common.v1361-common.v1))))))),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[658]*common.v3697)+((if self.scalar_static_bool[30]{v3594}else{(if self.scalar_static_bool[12]{(v3594+(((v1335*(self.scalar_static_f64[778]*common.v3581))-(v1331*((common.v407*(if common.v1322{(common.v1323*self.scalar_static_f64[816])}else{(if v1318{(v1319*self.scalar_static_f64[816])}else{v3455})}))/v3603)))/v3610))}else{common.v4})})+(self.scalar_static_f64[652]*common.v3653))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[658]*common.v3698)+((if self.scalar_static_bool[30]{v3595}else{(if self.scalar_static_bool[12]{(v3595+(((v1335*(self.scalar_static_f64[778]*common.v3582))-(v1331*((common.v407*(if common.v1322{(common.v1323*self.scalar_static_f64[815])}else{(if v1318{(v1319*self.scalar_static_f64[815])}else{common.v4})}))/v3603)))/v3610))}else{common.v4})})+(self.scalar_static_f64[652]*common.v3654))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[658]*common.v3699)+((if self.scalar_static_bool[30]{v3596}else{(if self.scalar_static_bool[12]{(v3596+(((v1335*(self.scalar_static_f64[778]*common.v3583))-(v1331*((common.v407*(if common.v1322{common.v4}else{(if v1318{common.v4}else{v3456})}))/v3603)))/v3610))}else{common.v4})})+(self.scalar_static_f64[652]*common.v3655))))), v6396, v6396, (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[658]*common.v3701)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[663]*(v1204-common.v1))+((v1226*v1228)+((((if self.scalar_static_bool[33]{(self.scalar_static_f64[620]*((v1269*self.scalar_static_f64[217])+(v1278*v1299)))}else{(if self.scalar_static_bool[31]{v1270}else{(if self.scalar_static_bool[12]{((v1270+(v1277*v1278))+(v1283/v1284))}else{common.v4})})})+(self.scalar_static_f64[605]*(common.v1349-common.v1)))+(common.v4*common.v679))-(if v1477{common.v4}else{(if common.v1391{(self.scalar_static_f64[19]*(self.scalar_static_f64[492]*(v1406*v1472)))}else{common.v4})}))))))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[663]*v3397)+(((v1228*(self.scalar_static_f64[215]*v3423))+(v1226*((-v3423)*v3430)))+v6400)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[605]*common.v3639))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[663]*v3398)+(((v1228*(self.scalar_static_f64[215]*v3424))+(v1226*((-v3424)*v3430)))+v6401)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[620]*((v1299*common.v3235)+(v1278*(self.scalar_static_f64[216]*common.v2960))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1277*common.v3235)+(((v1284*((v1282*v3475)+(v1268*(self.scalar_static_f64[635]*common.v2960))))-(v1283*v3475))/v3519))}else{common.v4})})}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[620]*((v1299*common.v3236)+(v1278*(self.scalar_static_f64[216]*common.v2961))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1277*common.v3236)+(((v1284*((v1282*v3476)+(v1268*(self.scalar_static_f64[635]*common.v2961))))-(v1283*v3476))/v3519))}else{common.v4})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * ((if self.scalar_static_bool[12]{v2304}else{common.v4})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if self.scalar_static_bool[12]{v6434}else{common.v4}), (if self.scalar_static_bool[12]{v6435}else{common.v4}), (if self.scalar_static_bool[12]{v6436}else{common.v4}), (if self.scalar_static_bool[12]{v6437}else{common.v4}), (if self.scalar_static_bool[12]{v6438}else{common.v4}), (if self.scalar_static_bool[12]{v6439}else{common.v4}), (if self.scalar_static_bool[12]{v6440}else{common.v4}), (if self.scalar_static_bool[12]{v6441}else{common.v4}), (if self.scalar_static_bool[12]{v6442}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[30]{v2304}else{common.v4})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if self.scalar_static_bool[30]{v6434}else{common.v4}), (if self.scalar_static_bool[30]{v6435}else{common.v4}), (if self.scalar_static_bool[30]{v6436}else{common.v4}), (if self.scalar_static_bool[30]{v6437}else{common.v4}), (if self.scalar_static_bool[30]{v6438}else{common.v4}), (if self.scalar_static_bool[30]{v6439}else{common.v4}), (if self.scalar_static_bool[30]{v6440}else{common.v4}), (if self.scalar_static_bool[30]{v6441}else{common.v4}), (if self.scalar_static_bool[30]{v6442}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1717/v1714)))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1717*v4652))/v4662))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[0]+(self.scalar_static_f64[750]*(if common.v749{(common.v750*self.scalar_static_f64[815])}else{(if common.v746{(v747*self.scalar_static_f64[815])}else{common.v4})})))/v1714))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1714*(self.scalar_static_f64[292]+(self.scalar_static_f64[750]*(if common.v749{(common.v750*self.scalar_static_f64[816])}else{(if common.v746{(v747*self.scalar_static_f64[816])}else{common.v4})}))))-(v1717*v4653))/v4662))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1717*v4654))/v4662))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1717*v4655))/v4662)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v1972)))),
            [3, 5, 6, 7],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v1971{v5425}else{(if v1965{(((v1967*((v1962*v5363)+(v1940*v5425)))-(v1966*(v5363+v5419)))/v5458)}else{(if v1941{v5425}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v1971{v5428}else{(if v1965{(((v1967*((v1962*v5364)+(v1940*v5428)))-(v1966*(v5364+v5420)))/v5458)}else{(if v1941{v5428}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v1971{v5431}else{(if v1965{(((v1967*((v1962*v5365)+(v1940*v5431)))-(v1966*(v5365+v5421)))/v5458)}else{(if v1941{v5431}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v1971{v5434}else{(if v1965{(((v1967*((v1962*v5366)+(v1940*v5434)))-(v1966*(v5366+v5422)))/v5458)}else{(if v1941{v5434}else{common.v4})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*((self.scalar_static_f64[0]*(self.scalar_static_f64[0]*(common.v687-common.v677)))/self.scalar_static_f64[509]))),
            2,
            multiplicity * (self.scalar_static_f64[880]),
            3,
            multiplicity * (self.scalar_static_f64[881]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*((self.scalar_static_f64[0]*common.v692)/self.scalar_static_f64[516]))),
            1,
            multiplicity * (self.scalar_static_f64[884]),
            4,
            multiplicity * (self.scalar_static_f64[885]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*v2318)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[13]*(common.v6489*v6495)), (self.scalar_static_f64[13]*(common.v6490*v6495)), (self.scalar_static_f64[13]*(common.v6491*v6495)), (self.scalar_static_f64[13]*(common.v6492*v6495)), (self.scalar_static_f64[13]*(common.v6493*v6495)), (self.scalar_static_f64[13]*(common.v6494*v6495))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*v2321)),
            3,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*common.v6508))),
            4,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*common.v6509))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[13]*v2324)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[13]*(v6495*common.v6514)), (self.scalar_static_f64[13]*(v6495*common.v6515)), (self.scalar_static_f64[13]*(v6495*common.v6516)), (self.scalar_static_f64[13]*(v6495*common.v6517)), (self.scalar_static_f64[13]*(v6495*common.v6518)), (self.scalar_static_f64[13]*(v6495*common.v6519))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[13]*v2327)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[13]*(v6495*common.v6532)), (self.scalar_static_f64[13]*(v6495*common.v6533)), (self.scalar_static_f64[13]*(v6495*common.v6534)), (self.scalar_static_f64[13]*(v6495*common.v6535)), (self.scalar_static_f64[13]*(v6495*common.v6536)), (self.scalar_static_f64[13]*(v6495*common.v6537))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[13]*v2331)),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[343]))),
            2,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[344]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[13]*v2335)),
            0,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[345]))),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[346]))),
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1643*v1694)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v4551+(v1643*v4491)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{((common.v1641*common.v4160)+(common.v1604*common.v4308))}else{common.v4}))+(v1643*v4492)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{(common.v1604*common.v4309)}else{common.v4}))+(v1643*v4493)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v4551+(v1643*v4494)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{(v4315+(common.v1604*common.v4310))}else{common.v4}))+(v1643*v4495)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{(v4324+(common.v1604*common.v4311))}else{common.v4}))+(v1643*v4496)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{(v4324+(common.v1604*common.v4312))}else{common.v4}))+(v1643*v4497)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{((common.v1641*common.v4162)+(common.v1604*common.v4313))}else{common.v4}))+(v1643*v4498)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1694*(if self.scalar_static_bool[42]{(v4324+(common.v1604*common.v4314))}else{common.v4}))+(v1643*v4499))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[741]*(self.scalar_static_f64[0]*common.v710)))),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [self.scalar_static_f64[890], self.scalar_static_f64[891], self.scalar_static_f64[891], self.scalar_static_f64[891], self.scalar_static_f64[892], self.scalar_static_f64[892], self.scalar_static_f64[893], self.scalar_static_f64[892]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*v2343)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [v6606, (self.scalar_static_f64[13]*(v6495*common.v6591)), (self.scalar_static_f64[13]*(v6495*common.v6592)), v6606, (self.scalar_static_f64[13]*(v6495*common.v6593)), (self.scalar_static_f64[13]*(v6495*common.v6594)), (self.scalar_static_f64[13]*(v6495*common.v6595)), (self.scalar_static_f64[13]*(v6495*common.v6596)), (self.scalar_static_f64[13]*(v6495*common.v6597))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1594*v1694)+((v1375*v1694)+(common.v4*common.v706)))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1594*v4491)+(v1375*v4491)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1594*v4492)+(v1375*v4492)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1594*v4493)+((v1694*(self.scalar_static_f64[612]*common.v3676))+(v1375*v4493))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1694*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v4108)}else{v4108}))+(v1594*v4494))+(((v1694*(self.scalar_static_f64[612]*common.v3677))+(v1375*v4494))+self.scalar_static_f64[338])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1694*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v4112)}else{v4112}))+(v1594*v4495))+(((v1694*(self.scalar_static_f64[612]*common.v3678))+(v1375*v4495))+self.scalar_static_f64[339])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v4522+(v1594*v4496))+((v4542+(v1375*v4496))+self.scalar_static_f64[340])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v4522+(v1594*v4497))+((v4542+(v1375*v4497))+self.scalar_static_f64[340])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1594*v4498)+(v1375*v4498)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1694*(if self.scalar_static_bool[42]{(self.scalar_static_f64[6]*v4120)}else{v4120}))+(v1594*v4499))+(((v1694*(self.scalar_static_f64[612]*common.v3680))+(v1375*v4499))+self.scalar_static_f64[337]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*v2349)),
            [4, 5, 6, 7, 9],
            [(self.scalar_static_f64[13]*(v6495*common.v6637)), (self.scalar_static_f64[13]*(v6495*common.v6638)), v6647, v6647, (self.scalar_static_f64[13]*(v6495*common.v6640))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(self.scalar_static_f64[745]*(self.scalar_static_f64[0]*common.v703)))}else{common.v4})),
            8,
            multiplicity * (self.scalar_static_f64[898]),
            9,
            multiplicity * (self.scalar_static_f64[899]),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v4,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(6),
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(self.scalar_static_f64[749]*(self.scalar_static_f64[0]*common.v700)))}else{common.v4})),
            6,
            multiplicity * (self.scalar_static_f64[904]),
            9,
            multiplicity * (self.scalar_static_f64[905]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v4,
        );
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (common.v4),
        );
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (common.v2359),
            10,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * ((common.v2274*v2360)),
            [3, 4, 5, 6, 7, 9, 10],
            [(v2360*common.v6318), (v2360*common.v6319), (v2360*common.v6320), (v2360*common.v6321), (v2360*common.v6322), (v2360*common.v6323), (common.v2274*v6495)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((v2253*common.v2359)),
            10,
            multiplicity * (v2253),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (common.v2359),
            10,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
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
            Some(6),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
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
        let v2318=0.0;
        let v2321=0.0;
        let v2324=0.0;
        let v2327=0.0;
        let v2331=0.0;
        let v2335=0.0;
        let v2343=0.0;
        let v2349=0.0;
        let v2360=0.0;
        let v6495=1.0;
        let v6606=(self.scalar_static_f64[13]*(v6495*common.v6590));
        let v6647=(self.scalar_static_f64[13]*(v6495*common.v6639));

        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[13]*(common.v6489*v6495)), (self.scalar_static_f64[13]*(common.v6490*v6495)), (self.scalar_static_f64[13]*(common.v6491*v6495)), (self.scalar_static_f64[13]*(common.v6492*v6495)), (self.scalar_static_f64[13]*(common.v6493*v6495)), (self.scalar_static_f64[13]*(common.v6494*v6495))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*common.v6508))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*common.v6509))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[13]*(v6495*common.v6514)), (self.scalar_static_f64[13]*(v6495*common.v6515)), (self.scalar_static_f64[13]*(v6495*common.v6516)), (self.scalar_static_f64[13]*(v6495*common.v6517)), (self.scalar_static_f64[13]*(v6495*common.v6518)), (self.scalar_static_f64[13]*(v6495*common.v6519))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[13]*(v6495*common.v6532)), (self.scalar_static_f64[13]*(v6495*common.v6533)), (self.scalar_static_f64[13]*(v6495*common.v6534)), (self.scalar_static_f64[13]*(v6495*common.v6535)), (self.scalar_static_f64[13]*(v6495*common.v6536)), (self.scalar_static_f64[13]*(v6495*common.v6537))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[343]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[344]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[345]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v6495*self.scalar_static_f64[346]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[v6606, (self.scalar_static_f64[13]*(v6495*common.v6591)), (self.scalar_static_f64[13]*(v6495*common.v6592)), v6606, (self.scalar_static_f64[13]*(v6495*common.v6593)), (self.scalar_static_f64[13]*(v6495*common.v6594)), (self.scalar_static_f64[13]*(v6495*common.v6595)), (self.scalar_static_f64[13]*(v6495*common.v6596)), (self.scalar_static_f64[13]*(v6495*common.v6597))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[13]*(v6495*common.v6637)), (self.scalar_static_f64[13]*(v6495*common.v6638)), v6647, v6647, (self.scalar_static_f64[13]*(v6495*common.v6640))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9], nodes[10]],
            &[(v2360*common.v6318), (v2360*common.v6319), (v2360*common.v6320), (v2360*common.v6321), (v2360*common.v6322), (v2360*common.v6323), (common.v2274*v6495)],
            &[],
            &[],
            multiplicity,
        );
    }
}
