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
    v1: f64, v3: f64, v33: f64, v34: f64, v49: f64, v161: f64, 
    v439: f64, v443: f64, v455: f64, v481: f64, v757: f64, v761: f64, 
    v763: f64, v768: f64, v771: f64, v774: f64, v779: f64, v787: f64, 
    v790: f64, v793: f64, v797: f64, v813: f64, v836: f64, v837: f64, 
    v839: f64, v842: bool, v843: f64, v859: f64, v861: f64, v864: bool, 
    v865: f64, v881: f64, v883: f64, v886: bool, v887: f64, v960: f64, 
    v1082: f64, v1142: f64, v1167: f64, v1170: f64, v1173: f64, v1200: f64, 
    v1280: f64, v1316: f64, v1317: f64, v1322: f64, v1323: f64, v1342: f64, 
    v1344: f64, v1347: bool, v1348: f64, v1357: f64, v1389: f64, v1391: f64, 
    v1393: f64, v1398: bool, v1399: f64, v1406: f64, v1407: f64, v1409: f64, 
    v1414: bool, v1416: f64, v1468: f64, v1470: f64, v1472: f64, v1477: bool, 
    v1478: f64, v1505: f64, v1518: f64, v1531: f64, v1544: f64, v1551: f64, 
    v1552: f64, v1555: f64, v1557: f64, v1562: bool, v1563: f64, v1569: f64, 
    v1573: f64, v1576: f64, v1584: f64, v1585: f64, v1586: f64, v1588: f64, 
    v1590: f64, v1594: f64, v1595: f64, v1597: f64, v1600: f64, v1602: f64, 
    v1603: bool, v1608: bool, v1609: f64, v1647: f64, v1649: f64, v1651: f64, 
    v1652: f64, v1655: f64, v1657: f64, v1662: bool, v1663: f64, v1668: f64, 
    v1671: f64, v1673: f64, v1681: f64, v1682: f64, v1683: f64, v1685: f64, 
    v1690: f64, v1691: f64, v1693: f64, v1695: f64, v1697: f64, v1698: bool, 
    v1703: bool, v1704: f64, v1835: f64, v1859: f64, v1877: f64, v1900: f64, 
    v1974: f64, v1986: f64, v1999: bool, v2000: bool, v2001: f64, v2004: bool, 
    v2005: f64, v2009: f64, v2010: f64, v2012: f64, v2016: f64, v2018: f64, 
    v2023: bool, v2024: f64, v2039: bool, v2146: bool, v2147: f64, v2149: f64, 
    v2151: f64, v2153: f64, v2155: f64, v2156: bool, v2158: bool, v2166: f64, 
    v2169: bool, v2170: f64, v2171: f64, v2177: bool, v2179: f64, v2180: f64, 
    v2184: f64, v2186: f64, v2189: f64, v2191: f64, v2196: bool, v2197: f64, 
    v2571: f64, v2603: f64, v2654: f64, v2657: f64, v2660: f64, v2663: f64, 
    v2666: f64, v2670: f64, v2674: f64, v2682: f64, v2688: f64, v2699: f64, 
    v2715: f64, v2716: f64, v2741: f64, v2742: f64, v2743: f64, v2744: f64, 
    v2894: f64, v2895: f64, v2896: f64, v3183: f64, v3184: f64, v3185: f64, 
    v3331: f64, v3332: f64, v3333: f64, v3374: f64, v3375: f64, v3376: f64, 
    v3383: f64, v3384: f64, v3385: f64, v3392: f64, v3393: f64, v3394: f64, 
    v3426: f64, v3427: f64, v3606: f64, v3607: f64, v3608: f64, v3698: f64, 
    v3699: f64, v3700: f64, v3701: f64, v3704: f64, v3707: f64, v3710: f64, 
    v3713: f64, v3714: f64, v3715: f64, v3716: f64, v3718: f64, v3722: f64, 
    v3725: f64, v3759: f64, v3760: f64, v3819: f64, v3820: f64, v3953: f64, 
    v3954: f64, v3955: f64, v4010: f64, v4011: f64, v4012: f64, v4025: f64, 
    v4026: f64, v4027: f64, v4048: f64, v4049: f64, v4050: f64, v4051: f64, 
    v4052: f64, v4069: f64, v4070: f64, v4071: f64, v4072: f64, v4073: f64, 
    v4705: f64, v4706: f64, v4707: f64, v4708: f64, v4791: f64, v4792: f64, 
    v4793: f64, v4794: f64, v4795: f64, v4796: f64, v4809: f64, v4810: f64, 
    v4811: f64, v4812: f64, v4813: f64, v4814: f64, v4815: f64, v4816: f64, 
    v4952: f64, v4953: f64, v4954: f64, v4955: f64, v4956: f64, v4957: f64, 
    v4958: f64, v4959: f64, v4960: f64, v5300: f64, v5301: f64, v5302: f64, 
    v5303: f64, v7046: f64, v7047: f64, v7048: f64, v7049: f64, v7050: f64, 
    v7051: f64, v7257: f64, v7258: f64, v7259: f64, v7260: f64, v7261: f64, 
    v7262: f64, v7276: f64, v7277: f64, v7282: f64, v7283: f64, v7284: f64, 
    v7285: f64, v7286: f64, v7287: f64, v7300: f64, v7301: f64, v7306: f64, 
    v7307: f64, v7308: f64, v7309: f64, v7310: f64, v7311: f64, v7366: f64, 
    v7367: f64, v7368: f64, v7369: f64, v7370: f64, v7371: f64, v7372: f64, 
    v7373: f64, v7374: f64, v7416: f64, v7417: f64, v7418: f64, v7419: f64, 
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=1.0;
        let v3=0.0;
        let v33=0.001;
        let v34=2.0;
        let v49=0.1;
        let v161=3.0;
        let v439=1e-6;
        let v443=0.5;
        let v455=4.0;
        let v481=6.0;
        let v754=ctx.node_voltage(nodes[6]);
        let v755=ctx.node_voltage(nodes[7]);
        let v757=(self.scalar_static_f64[0]*(v754-v755));
        let v758=ctx.node_voltage(nodes[8]);
        let v760=(self.scalar_static_f64[0]*(v754-v758));
        let v761=ctx.node_voltage(nodes[4]);
        let v763=(self.scalar_static_f64[0]*(v754-v761));
        let v764=ctx.node_voltage(nodes[5]);
        let v766=(self.scalar_static_f64[0]*(v764-v761));
        let v768=(self.scalar_static_f64[0]*(v764-v754));
        let v771=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[3])-v755));
        let v773=(self.scalar_static_f64[0]*(v755-v758));
        let v774=ctx.node_voltage(nodes[2]);
        let v777=ctx.node_voltage(nodes[1]);
        let v779=(self.scalar_static_f64[0]*(v777-v764));
        let v784=(self.scalar_static_f64[0]*(v777-ctx.node_voltage(nodes[0])));
        let v785=ctx.node_voltage(nodes[10]);
        let v787=(self.scalar_static_f64[0]*(v785-v755));
        let v790=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[9])-v785));
        let v793=(((v760+v768)-v773)-v787);
        let v797=((v793+(v779+(-v784)))-v790);
        let v798=(v784+v797);
        let v799=(v771-v787);
        let v801=(self.scalar_static_f64[412]*v760);
        let v804=(if (v801<self.scalar_static_f64[214]){v1}else{v3});
        let v805=(v801).exp();
        let v807=(!(v804!=0.0));
        let v809=(if v807{self.scalar_static_f64[215]}else{v3});
        let v813=(if v807{(v809*(v1+(v801-self.scalar_static_f64[214])))}else{(if (v804!=0.0){v805}else{v3})});
        let v814=(self.scalar_static_f64[412]*v763);
        let v815=(v814/self.scalar_static_f64[642]);
        let v817=(if (v815<self.scalar_static_f64[214]){v1}else{v3});
        let v818=(v815).exp();
        let v820=(!(v817!=0.0));
        let v821=(if v820{self.scalar_static_f64[215]}else{v809});
        let v825=(if v820{(v821*(v1+(v815-self.scalar_static_f64[214])))}else{(if (v817!=0.0){v818}else{v3})});
        let v826=(self.scalar_static_f64[412]*v793);
        let v828=(if (v826<self.scalar_static_f64[214]){v1}else{v3});
        let v829=(v826).exp();
        let v831=(!(v828!=0.0));
        let v832=(if v831{self.scalar_static_f64[215]}else{v821});
        let v836=(if v831{(v832*(v1+(v826-self.scalar_static_f64[214])))}else{(if (v828!=0.0){v829}else{v3})});
        let v837=(self.scalar_static_f64[412]*v768);
        let v839=(if (v837<self.scalar_static_f64[214]){v1}else{v3});
        let v842=(!(v839!=0.0));
        let v843=(if v842{self.scalar_static_f64[215]}else{v832});
        let v848=(self.scalar_static_f64[412]*v798);
        let v850=(if (v848<self.scalar_static_f64[214]){v1}else{v3});
        let v851=(v848).exp();
        let v853=(!(v850!=0.0));
        let v854=(if v853{self.scalar_static_f64[215]}else{v843});
        let v858=(if v853{(v854*(v1+(v848-self.scalar_static_f64[214])))}else{(if (v850!=0.0){v851}else{v3})});
        let v859=(self.scalar_static_f64[412]*v771);
        let v861=(if (v859<self.scalar_static_f64[214]){v1}else{v3});
        let v864=(!(v861!=0.0));
        let v865=(if v864{self.scalar_static_f64[215]}else{v854});
        let v870=(self.scalar_static_f64[412]*(v799-v790));
        let v872=(if (v870<self.scalar_static_f64[214]){v1}else{v3});
        let v873=(v870).exp();
        let v875=(!(v872!=0.0));
        let v876=(if v875{self.scalar_static_f64[215]}else{v865});
        let v880=(if v875{(v876*(v1+(v870-self.scalar_static_f64[214])))}else{(if (v872!=0.0){v873}else{v3})});
        let v881=(self.scalar_static_f64[412]*v799);
        let v883=(if (v881<self.scalar_static_f64[214]){v1}else{v3});
        let v886=(!(v883!=0.0));
        let v887=(if v886{self.scalar_static_f64[215]}else{v876});
        let v893=(self.scalar_static_f64[412]*(v798-self.scalar_static_f64[500]));
        let v895=(if (v893<self.scalar_static_f64[214]){v1}else{v3});
        let v896=(v893).exp();
        let v898=(!(v895!=0.0));
        let v899=(if v898{self.scalar_static_f64[215]}else{v887});
        let v905=(self.scalar_static_f64[412]*(v793-self.scalar_static_f64[500]));
        let v907=(if (v905<self.scalar_static_f64[214]){v1}else{v3});
        let v908=(v905).exp();
        let v910=(!(v907!=0.0));
        let v911=(if v910{self.scalar_static_f64[215]}else{v899});
        let v917=(self.scalar_static_f64[412]*(v760-self.scalar_static_f64[500]));
        let v919=(if (v917<self.scalar_static_f64[214]){v1}else{v3});
        let v920=(v917).exp();
        let v922=(!(v919!=0.0));
        let v923=(if v922{self.scalar_static_f64[215]}else{v911});
        let v927=(if v922{(v923*(v1+(v917-self.scalar_static_f64[214])))}else{(if (v919!=0.0){v920}else{v3})});
        let v929=(self.scalar_static_f64[412]*(v757-self.scalar_static_f64[500]));
        let v931=(if (v929<self.scalar_static_f64[214]){v1}else{v3});
        let v932=(v929).exp();
        let v934=(!(v931!=0.0));
        let v935=(if v934{self.scalar_static_f64[215]}else{v923});
        let v939=(if v934{(v935*(v1+(v929-self.scalar_static_f64[214])))}else{(if (v931!=0.0){v932}else{v3})});
        let v942=((v1+(v455*v927))).sqrt();
        let v945=((v1+(v455*v939))).sqrt();
        let v946=(v34*v939);
        let v947=(v1+v945);
        let v948=(v946/v947);
        let v951=(if (v948<self.scalar_static_f64[216]){v1}else{v3});
        let v952=(if (v951!=0.0){self.scalar_static_f64[216]}else{v948});
        let v954=(v1+v942);
        let v955=(v954/v947);
        let v958=(self.scalar_static_f64[411]*((v942-v945)-(v955).ln()));
        let v960=((v773+v958)/self.scalar_static_f64[618]);
        let v962=(if (v960>v3){v1}else{v3});
        let v963=100.0;
        let v965=(if (v757<v963){v1}else{v3});
        let v966=((v962!=0.0)&&(v965!=0.0));
        let v969=((v962!=0.0)&&(!(v965!=0.0)));
        let v971=(v1+(v757-v963));
        let v977=(self.scalar_static_f64[618]*(v443*v960));
        let v979=(v1+(self.scalar_static_f64[412]*v977));
        let v984=(if (v962!=0.0){((self.scalar_static_f64[500]+(self.scalar_static_f64[865]*(v979).ln()))-(if v969{(v963+(v971).ln())}else{(if v966{v757}else{v3})}))}else{v3});
        let v987=(if (v962!=0.0){self.scalar_static_f64[866]}else{v3});
        let v989=(if (v962!=0.0){(v987*v987)}else{v439});
        let v993=(if (v984<v3){v1}else{v3});
        let v994=((v962!=0.0)&&(v993!=0.0));
        let v995=(v443*v989);
        let v997=((v989+(if (v962!=0.0){(v984*v984)}else{self.scalar_static_f64[670]}))).sqrt();
        let v998=(v997-v984);
        let v1002=((v962!=0.0)&&(!(v993!=0.0)));
        let v1005=(if v1002{(v443*(v984+v997))}else{(if v994{(v995/v998)}else{v3})});
        let v1009=(v1005+self.scalar_static_f64[219]);
        let v1010=(v1005*v1009);
        let v1013=(self.scalar_static_f64[218]*(v1005+self.scalar_static_f64[867]));
        let v1015=(if (v962!=0.0){(v1010/v1013)}else{v3});
        let v1017=(if (v962!=0.0){(v960/v1015)}else{v3});
        let v1021=(if (v962!=0.0){((v1017-v1)/self.scalar_static_f64[220])}else{self.scalar_static_f64[649]});
        let v1023=(if (v1017<v1){v1}else{v3});
        let v1024=((v962!=0.0)&&(v1023!=0.0));
        let v1025=(v1021).exp();
        let v1026=(v1+v1025);
        let v1032=((v962!=0.0)&&(!(v1023!=0.0)));
        let v1034=((-v1021)).exp();
        let v1035=(v1+v1034);
        let v1048=(if (v962!=0.0){((if v1032{(v1017+(self.scalar_static_f64[220]*(v1035).ln()))}else{(if v1024{(v1+(self.scalar_static_f64[220]*(v1026).ln()))}else{v3})})/self.scalar_static_f64[226])}else{v3});
        let v1050=(if (v962!=0.0){(v1005/self.scalar_static_f64[219])}else{v3});
        let v1051=(v455*v1048);
        let v1052=(v1050*v1051);
        let v1053=(v1+v1050);
        let v1056=((v1+(v1052*v1053))).sqrt();
        let v1057=(v1+v1056);
        let v1058=(v34*v1048);
        let v1059=(v1053*v1058);
        let v1061=(if (v962!=0.0){(v1057/v1059)}else{v3});
        let v1063=(v952*v1061);
        let v1064=((v1-v1061)+v1063);
        let v1065=(v1+v1063);
        let v1067=(if (v962!=0.0){(v1064/v1065)}else{v3});
        let v1070=(if (v962!=0.0){(self.scalar_static_f64[412]*(v977*v1067))}else{v3});
        let v1073=(v1+(v952+v1070));
        let v1076=(if (v962!=0.0){((v34*v1070)+(v952*v1073))}else{v3});
        let v1079=(if (v962!=0.0){(v443*(v1070-v1))}else{v3});
        let v1082=(if (v962!=0.0){(v1076+(v1079*v1079))}else{v3});
        let v1084=(if (v1070>=v1){v1}else{v3});
        let v1085=((v962!=0.0)&&(v1084!=0.0));
        let v1086=(v1082).sqrt();
        let v1090=((v962!=0.0)&&(!(v1084!=0.0)));
        let v1091=(v1086-v1079);
        let v1093=(if v1090{(v1076/v1091)}else{(if v1085{(v1079+v1086)}else{v3})});
        let v1097=((v962!=0.0)&&((if (v1093<self.scalar_static_f64[227]){v1}else{v3})!=0.0));
        let v1098=(if v1097{self.scalar_static_f64[227]}else{v1093});
        let v1099=(v1+v1098);
        let v1108=(if (v962!=0.0){(self.scalar_static_f64[228]*(v960-self.scalar_static_f64[217]))}else{v3});
        let v1115=(((if (v962!=0.0){(v960*self.scalar_static_f64[871])}else{v3})+(v1108*v1108))).sqrt();
        let v1125=((v962!=0.0)&&self.scalar_static_bool[20]);
        let v1126=(v34*v960);
        let v1127=(v960+v1015);
        let v1132=(v960*self.scalar_static_f64[217]);
        let v1133=(v960+self.scalar_static_f64[217]);
        let v1138=(!(v962!=0.0));
        let v1139=(v34*v927);
        let v1142=(if v1138{v813}else{(if (v962!=0.0){((v1098*v1099)*self.scalar_static_f64[869])}else{v3})});
        let v1154=(if (((v773).abs()<self.scalar_static_f64[873])||((v958).abs()<(self.scalar_static_f64[874]*(v942+v945)))){v1}else{v3});
        let v1155=(v1138&&(v1154!=0.0));
        let v1156=(v952+(if v1138{(v1139/v954)}else{v1098}));
        let v1158=(if v1155{(v443*v1156)}else{v3});
        let v1159=(v1+v1158);
        let v1163=(v1138&&(!(v1154!=0.0)));
        let v1165=((v760+v958)-v757);
        let v1167=(if v1163{(v958/v1165)}else{(if v1155{(v1158/v1159)}else{v1067})});
        let v1169=(if v1138{self.scalar_static_f64[872]}else{(if v1125{(self.scalar_static_f64[538]*(v49+(v1126/v1127)))}else{(if ((v962!=0.0)&&(self.scalar_static_f64[230]!=0.0)){self.scalar_static_f64[872]}else{v3})})});
        let v1170=(if v1138{v960}else{(if (v962!=0.0){(v1132/v1133)}else{v3})});
        let v1173=(if v1138{(v1-(v1170/self.scalar_static_f64[217]))}else{(if (v962!=0.0){(self.scalar_static_f64[217]/v1133)}else{v3})});
        let v1180=((v763-self.scalar_static_f64[875])/self.scalar_static_f64[876]);
        let v1182=(if (v763<self.scalar_static_f64[875]){v1}else{v3});
        let v1183=(v1180).exp();
        let v1184=(v1+v1183);
        let v1189=(!(v1182!=0.0));
        let v1191=((-v1180)).exp();
        let v1192=(v1+v1191);
        let v1196=(if v1189{(self.scalar_static_f64[875]-(self.scalar_static_f64[876]*(v1192).ln()))}else{(if (v1182!=0.0){(v763-(self.scalar_static_f64[876]*(v1184).ln()))}else{v3})});
        let v1198=(v1-(self.scalar_static_f64[579]*v1196));
        let v1200=f64::powf(v1198,self.scalar_static_f64[234]);
        let v1206=((self.scalar_static_f64[877]*(v1-v1200))+(v161*(v763-v1196)));
        let v1219=(if self.scalar_static_bool[26]{v760}else{(if self.scalar_static_bool[24]{(v757+(if v1138{v773}else{(if (v962!=0.0){(v1108+v1115)}else{v3})}))}else{(if (self.scalar_static_f64[236]!=0.0){v757}else{v3})})});
        let v1227=(v1219-self.scalar_static_f64[883]);
        let v1228=(v1227/v1169);
        let v1230=(if (v1219<self.scalar_static_f64[883]){v1}else{v3});
        let v1231=(v1228).exp();
        let v1232=(v1+v1231);
        let v1233=(v1232).ln();
        let v1237=(!(v1230!=0.0));
        let v1239=((-v1228)).exp();
        let v1240=(v1+v1239);
        let v1241=(v1240).ln();
        let v1244=(if v1237{(self.scalar_static_f64[883]-(v1169*v1241))}else{(if (v1230!=0.0){(v1219-(v1169*v1233))}else{v3})});
        let v1246=f64::powf(v1173,self.scalar_static_f64[239]);
        let v1250=(v1-(v1244/self.scalar_static_f64[538]));
        let v1251=f64::powf(v1250,self.scalar_static_f64[240]);
        let v1255=(self.scalar_static_f64[880]*v1246);
        let v1256=(v1219-v1244);
        let v1261=((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(v1-(v1246*v1251)))+(v1255*v1256)))+(self.scalar_static_f64[595]*v757));
        let v1264=(v825*self.scalar_static_f64[886]);
        let v1266=((v1+v1264)).sqrt();
        let v1267=(v1+v1266);
        let v1268=(v1264/v1267);
        let v1270=f64::powf(v1142,self.scalar_static_f64[887]);
        let v1271=(self.scalar_static_f64[886]*v1270);
        let v1273=((v1+v1271)).sqrt();
        let v1274=(v1+v1273);
        let v1275=(v1271/v1274);
        let v1279=(v1+(v1206/self.scalar_static_f64[804]));
        let v1280=(v1261/self.scalar_static_f64[802]);
        let v1281=(v1279+v1280);
        let v1292=((if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*v1279))}else{v3})).exp();
        let v1293=((if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*((-v1261)/self.scalar_static_f64[802])))}else{v3})).exp();
        let v1299=(if self.scalar_static_bool[28]{((v1292-v1293)/self.scalar_static_f64[890])}else{(if (self.scalar_static_f64[241]!=0.0){v1281}else{v3})});
        let v1300=0.010000000000000002;
        let v1301=(v1299*v1299);
        let v1303=(if (v1299<v3){v1}else{v3});
        let v1304=0.005000000000000001;
        let v1306=((v1300+v1301)).sqrt();
        let v1307=(v1306-v1299);
        let v1310=(!(v1303!=0.0));
        let v1313=(if v1310{(v443*(v1299+v1306))}else{(if (v1303!=0.0){(v1304/v1307)}else{v3})});
        let v1316=(v1+(v443*(v1268+v1275)));
        let v1317=(v1313*v1316);
        let v1320=(v1270*self.scalar_static_f64[891]);
        let v1321=(self.scalar_static_f64[687]*v825);
        let v1322=(v1321-v1320);
        let v1323=(v1322/v1317);
        let v1324=0.0001;
        let v1325=(v763/v1324);
        let v1326=(v763<v3);
        let v1327=(if v1326{v1}else{v3});
        let v1328=(v1325).exp();
        let v1329=(v1+v1328);
        let v1333=(!(v1327!=0.0));
        let v1335=((-v1325)).exp();
        let v1336=(v1+v1335);
        let v1340=(if v1333{(v763+(v1324*(v1336).ln()))}else{(if (v1327!=0.0){(v1324*(v1329).ln())}else{v3})});
        let v1342=(v1340/self.scalar_static_f64[243]);
        let v1344=(if (v1342<self.scalar_static_f64[214]){v1}else{v3});
        let v1347=(!(v1344!=0.0));
        let v1348=(if v1347{self.scalar_static_f64[215]}else{v935});
        let v1357=((v763-self.scalar_static_f64[244])/v33);
        let v1379=(v814/self.scalar_static_f64[148]);
        let v1381=(if (v1379<self.scalar_static_f64[214]){v1}else{v3});
        let v1382=(v1379).exp();
        let v1384=(!(v1381!=0.0));
        let v1385=(if v1384{self.scalar_static_f64[215]}else{v1348});
        let v1389=(if v1384{(v1385*(v1+(v1379-self.scalar_static_f64[214])))}else{(if (v1381!=0.0){v1382}else{v1340})});
        let v1391=(self.scalar_static_f64[412]*(v763-self.scalar_static_f64[558]));
        let v1393=(if (v1391<self.scalar_static_f64[214]){v1}else{v3});
        let v1398=((self.scalar_static_f64[154]!=0.0)&&(!(v1393!=0.0)));
        let v1399=(if v1398{self.scalar_static_f64[215]}else{v1385});
        let v1406=((v1323/self.scalar_static_f64[687])-1000.0);
        let v1407=40.0;
        let v1409=(if (v1406<v1407){v1}else{v3});
        let v1414=((self.scalar_static_f64[154]!=0.0)&&(!(v1409!=0.0)));
        let v1416=(if v1414{2.3538526683702e17}else{v1399});
        let v1457=(self.scalar_static_f64[412]*v766);
        let v1458=(v1457/self.scalar_static_f64[152]);
        let v1460=(if (v1458<self.scalar_static_f64[214]){v1}else{v3});
        let v1461=(v1458).exp();
        let v1463=(!(v1460!=0.0));
        let v1464=(if v1463{self.scalar_static_f64[215]}else{v1416});
        let v1468=(if v1463{(v1464*(v1+(v1458-self.scalar_static_f64[214])))}else{(if (v1460!=0.0){v1461}else{v1389})});
        let v1470=(self.scalar_static_f64[412]*(v766-self.scalar_static_f64[558]));
        let v1472=(if (v1470<self.scalar_static_f64[214]){v1}else{v3});
        let v1477=((self.scalar_static_f64[154]!=0.0)&&(!(v1472!=0.0)));
        let v1478=(if v1477{self.scalar_static_f64[215]}else{v1464});
        let v1495=(v814/self.scalar_static_f64[135]);
        let v1497=(if (v1495<self.scalar_static_f64[214]){v1}else{v3});
        let v1498=(v1495).exp();
        let v1500=(!(v1497!=0.0));
        let v1501=(if v1500{self.scalar_static_f64[215]}else{v1478});
        let v1505=(if v1500{(v1501*(v1+(v1495-self.scalar_static_f64[214])))}else{(if (v1497!=0.0){v1498}else{v1468})});
        let v1508=(v1457/self.scalar_static_f64[170]);
        let v1510=(if (v1508<self.scalar_static_f64[214]){v1}else{v3});
        let v1511=(v1508).exp();
        let v1513=(!(v1510!=0.0));
        let v1514=(if v1513{self.scalar_static_f64[215]}else{v1501});
        let v1518=(if v1513{(v1514*(v1+(v1508-self.scalar_static_f64[214])))}else{(if (v1510!=0.0){v1511}else{v1505})});
        let v1521=(v826/self.scalar_static_f64[141]);
        let v1523=(if (v1521<self.scalar_static_f64[214]){v1}else{v3});
        let v1524=(v1521).exp();
        let v1526=(!(v1523!=0.0));
        let v1527=(if v1526{self.scalar_static_f64[215]}else{v1514});
        let v1531=(if v1526{(v1527*(v1+(v1521-self.scalar_static_f64[214])))}else{(if (v1523!=0.0){v1524}else{v1518})});
        let v1534=(v1457/self.scalar_static_f64[174]);
        let v1536=(if (v1534<self.scalar_static_f64[214]){v1}else{v3});
        let v1537=(v1534).exp();
        let v1539=(!(v1536!=0.0));
        let v1540=(if v1539{self.scalar_static_f64[215]}else{v1527});
        let v1544=(if v1539{(v1540*(v1+(v1534-self.scalar_static_f64[214])))}else{(if (v1536!=0.0){v1537}else{v1531})});
        let v1551=(if (v1326&&self.scalar_static_bool[36]){v1}else{v3});
        let v1552=(v34*v1200);
        let v1555=(self.scalar_static_f64[769]*(v1-(self.scalar_static_f64[21]/v1552)));
        let v1557=(if (v1555<self.scalar_static_f64[214]){v1}else{v3});
        let v1562=((v1551!=0.0)&&(!(v1557!=0.0)));
        let v1563=(if v1562{self.scalar_static_f64[215]}else{v1540});
        let v1569=(if (v1551!=0.0){(self.scalar_static_f64[579]*v763)}else{self.scalar_static_f64[800]});
        let v1571=1e-30;
        let v1573=(((v1569*v1569)+v1571)).sqrt();
        let v1576=f64::powf(v1573,self.scalar_static_f64[249]);
        let v1584=(v481*v1569);
        let v1585=(v1569*v1584);
        let v1586=(v1569+self.scalar_static_f64[252]);
        let v1588=((self.scalar_static_f64[19]*(self.scalar_static_f64[251]-((v161*v1569)*self.scalar_static_f64[252])))-(v1585*v1586));
        let v1590=0.16666666666666666;
        let v1594=(self.scalar_static_f64[769]*(self.scalar_static_f64[21]*v763));
        let v1595=(self.scalar_static_f64[436]*(if (v1551!=0.0){((v1576*v1588)*v1590)}else{v3}));
        let v1597=(if (v1551!=0.0){(v1594/v1595)}else{v1569});
        let v1598=-0.001;
        let v1600=(if (v1597<v1598){v1}else{v3});
        let v1602=(if (v1597<self.scalar_static_f64[214]){v1}else{v3});
        let v1603=((v1551!=0.0)&&(v1600!=0.0));
        let v1608=(v1603&&(!(v1602!=0.0)));
        let v1609=(if v1608{self.scalar_static_f64[215]}else{v1563});
        let v1647=(if (self.scalar_static_bool[39]&&(v757<v3)){v1}else{v3});
        let v1648=(self.scalar_static_f64[580]*v757);
        let v1649=(v1-v1648);
        let v1651=(if (v1647!=0.0){f64::powf(v1649,self.scalar_static_f64[240])}else{v3});
        let v1652=(v34*v1651);
        let v1655=(self.scalar_static_f64[789]*(v1-(self.scalar_static_f64[53]/v1652)));
        let v1657=(if (v1655<self.scalar_static_f64[214]){v1}else{v3});
        let v1662=((v1647!=0.0)&&(!(v1657!=0.0)));
        let v1663=(if v1662{self.scalar_static_f64[215]}else{v1609});
        let v1668=(if (v1647!=0.0){v1648}else{self.scalar_static_f64[780]});
        let v1671=((v1571+(v1668*v1668))).sqrt();
        let v1673=f64::powf(v1671,self.scalar_static_f64[253]);
        let v1681=(v481*v1668);
        let v1682=(v1668*v1681);
        let v1683=(v1668+self.scalar_static_f64[256]);
        let v1685=((self.scalar_static_f64[51]*(self.scalar_static_f64[255]-((v161*v1668)*self.scalar_static_f64[256])))-(v1682*v1683));
        let v1690=(self.scalar_static_f64[789]*(self.scalar_static_f64[53]*v757));
        let v1691=(self.scalar_static_f64[457]*(if (v1647!=0.0){(v1590*(v1673*v1685))}else{v3}));
        let v1693=(if (v1647!=0.0){(v1690/v1691)}else{v1668});
        let v1695=(if (v1693<v1598){v1}else{v3});
        let v1697=(if (v1693<self.scalar_static_f64[214]){v1}else{v3});
        let v1698=((v1647!=0.0)&&(v1695!=0.0));
        let v1703=(v1698&&(!(v1697!=0.0)));
        let v1704=(if v1703{self.scalar_static_f64[215]}else{v1663});
        let v1735=(v836*self.scalar_static_f64[886]);
        let v1736=(v455*(if v910{(v911*(v1+(v905-self.scalar_static_f64[214])))}else{(if (v907!=0.0){v908}else{v3})}));
        let v1737=(v1735-self.scalar_static_f64[886]);
        let v1739=((v1+v1735)).sqrt();
        let v1740=(v1+v1739);
        let v1743=((v1+v1736)).sqrt();
        let v1744=(v1+v1743);
        let v1828=(v858-v1);
        let v1829=(self.scalar_static_f64[906]*v1828);
        let v1832=((v1+(v858*self.scalar_static_f64[898]))).sqrt();
        let v1833=(v1+v1832);
        let v1835=(if (self.scalar_static_f64[266]!=0.0){(v1829/v1833)}else{v3});
        let v1841=(self.scalar_static_f64[907]*(v858-v880));
        let v1848=((v1+(self.scalar_static_f64[909]*(v858+(v880*self.scalar_static_f64[261]))))).sqrt();
        let v1849=(v1+v1848);
        let v1853=(v1828*self.scalar_static_f64[907]);
        let v1856=((v1+(v858*self.scalar_static_f64[909]))).sqrt();
        let v1857=(v1+v1856);
        let v1859=(if self.scalar_static_bool[46]{(v1853/v1857)}else{(if self.scalar_static_bool[45]{(v1841/v1849)}else{v3})});
        let v1873=(if self.scalar_static_bool[48]{(v798-self.scalar_static_f64[918])}else{v3});
        let v1877=(if self.scalar_static_bool[48]{(v1873*v1873)}else{v1301});
        let v1879=(if (v1873<v3){v1}else{v3});
        let v1880=(self.scalar_static_bool[48]&&(v1879!=0.0));
        let v1883=((self.scalar_static_f64[271]+v1877)).sqrt();
        let v1884=(v1883-v1873);
        let v1888=(self.scalar_static_bool[48]&&(!(v1879!=0.0)));
        let v1891=(if v1888{(v443*(v1873+v1883))}else{(if v1880{(self.scalar_static_f64[272]/v1884)}else{v3})});
        let v1895=(v1891+(self.scalar_static_f64[913]+(self.scalar_static_f64[611]*(v1835+v1859))));
        let v1900=(if self.scalar_static_bool[50]{v1}else{(if self.scalar_static_bool[48]{(v1891/v1895)}else{v1})});
        let v1965=(if (v1281<v3){v1}else{v3});
        let v1967=((v1300+(v1281*v1281))).sqrt();
        let v1968=(v1967-v1281);
        let v1971=(!(v1965!=0.0));
        let v1974=(if v1971{(v443*(v1281+v1967))}else{(if (v1965!=0.0){(v1304/v1968)}else{v3})});
        let v1986=(if (v1323>v3){v1}else{v3});
        let v1992=(if (v757<self.scalar_static_f64[294]){v1}else{v3});
        let v1995=((-v1323)/self.scalar_static_f64[295]);
        let v1997=(if (v1995<self.scalar_static_f64[214]){v1}else{v3});
        let v1999=((v1992!=0.0)&&((v1986!=0.0)&&(self.scalar_static_f64[293]!=0.0)));
        let v2000=((v1997!=0.0)&&v1999);
        let v2001=(v1995).exp();
        let v2004=(v1999&&(!(v1997!=0.0)));
        let v2005=(if v2004{self.scalar_static_f64[215]}else{v1704});
        let v2009=(if v2004{(v2005*(v1+(v1995-self.scalar_static_f64[214])))}else{(if v2000{v2001}else{v3})});
        let v2010=(self.scalar_static_f64[294]-v757);
        let v2012=(if v1999{(v2009*v2010)}else{v3});
        let v2016=(self.scalar_static_f64[919]*f64::powf(v2012,self.scalar_static_f64[296]));
        let v2018=(if (v2016<self.scalar_static_f64[214]){v1}else{v3});
        let v2023=(v1999&&(!(v2018!=0.0)));
        let v2024=(if v2023{self.scalar_static_f64[215]}else{v2005});
        let v2039=((v1986!=0.0)&&self.scalar_static_bool[55]);
        let v2146=((v1992!=0.0)&&((self.scalar_static_f64[311]!=0.0)&&(v2039&&self.scalar_static_bool[59])));
        let v2147=f64::powf(v2010,self.scalar_static_f64[296]);
        let v2149=(v1323+self.scalar_static_f64[312]);
        let v2151=(v1-(v1323/v2149));
        let v2153=f64::powf(v2151,self.scalar_static_f64[313]);
        let v2155=(if v2146{(v2147*v2153)}else{v3});
        let v2156=((self.scalar_static_f64[305]!=0.0)&&v2146);
        let v2158=(self.scalar_static_bool[57]&&v2146);
        let v2162=(if v2158{((v1323-self.scalar_static_f64[314])/self.scalar_static_f64[312])}else{v3});
        let v2166=(if v2158{((v2162-v1)/self.scalar_static_f64[315])}else{v1357});
        let v2168=(if (v2162<v1){v1}else{v3});
        let v2169=(v2158&&(v2168!=0.0));
        let v2170=(v2166).exp();
        let v2171=(v1+v2170);
        let v2177=(v2158&&(!(v2168!=0.0)));
        let v2179=((-v2166)).exp();
        let v2180=(v1+v2179);
        let v2184=(if v2177{(v2162+(self.scalar_static_f64[315]*(v2180).ln()))}else{(if v2169{(v1+(self.scalar_static_f64[315]*(v2171).ln()))}else{v3})});
        let v2186=f64::powf(v2184,self.scalar_static_f64[316]);
        let v2189=(self.scalar_static_f64[919]*(if v2158{(v2155*v2186)}else{(if v2156{v2155}else{v3})}));
        let v2191=(if (v2189<self.scalar_static_f64[214]){v1}else{v3});
        let v2196=(v2146&&(!(v2191!=0.0)));
        let v2197=(if v2196{self.scalar_static_f64[215]}else{v2024});
        let v2259=((v766-self.scalar_static_f64[875])/self.scalar_static_f64[876]);
        let v2261=(if (v766<self.scalar_static_f64[875]){v1}else{v3});
        let v2262=(v2259).exp();
        let v2263=(v1+v2262);
        let v2268=(!(v2261!=0.0));
        let v2270=((-v2259)).exp();
        let v2271=(v1+v2270);
        let v2275=(if v2268{(self.scalar_static_f64[875]-(self.scalar_static_f64[876]*(v2271).ln()))}else{(if (v2261!=0.0){(v766-(self.scalar_static_f64[876]*(v2263).ln()))}else{v3})});
        let v2278=(v1-(self.scalar_static_f64[579]*v2275));
        let v2291=(v1268*self.scalar_static_f64[927]);
        let v2292=(v1974*v2291);
        let v2293=(v1275*self.scalar_static_f64[927]);
        let v2294=(v1974*v2293);
        let v2296=((v793-self.scalar_static_f64[883])/self.scalar_static_f64[872]);
        let v2298=(if (v793<self.scalar_static_f64[883]){v1}else{v3});
        let v2299=(v2296).exp();
        let v2300=(v1+v2299);
        let v2305=(!(v2298!=0.0));
        let v2307=((-v2296)).exp();
        let v2308=(v1+v2307);
        let v2312=(if v2305{(self.scalar_static_f64[883]-(self.scalar_static_f64[872]*(v2308).ln()))}else{(if (v2298!=0.0){(v793-(self.scalar_static_f64[872]*(v2300).ln()))}else{v3})});
        let v2314=(v1-(v2312/self.scalar_static_f64[538]));
        let v2329=((v798-self.scalar_static_f64[883])/self.scalar_static_f64[872]);
        let v2331=(if (v798<self.scalar_static_f64[883]){v1}else{v3});
        let v2332=(v2329).exp();
        let v2333=(v1+v2332);
        let v2338=(!(v2331!=0.0));
        let v2340=((-v2329)).exp();
        let v2341=(v1+v2340);
        let v2345=(if v2338{(self.scalar_static_f64[883]-(self.scalar_static_f64[872]*(v2341).ln()))}else{(if (v2331!=0.0){(v798-(self.scalar_static_f64[872]*(v2333).ln()))}else{v3})});
        let v2347=(v1-(v2345/self.scalar_static_f64[538]));
        let v2366=((v771-self.scalar_static_f64[929])/self.scalar_static_f64[928]);
        let v2368=(if (v771<self.scalar_static_f64[929]){v1}else{v3});
        let v2369=(v2366).exp();
        let v2370=(v1+v2369);
        let v2375=(!(v2368!=0.0));
        let v2377=((-v2366)).exp();
        let v2378=(v1+v2377);
        let v2382=(if v2375{(self.scalar_static_f64[929]-(self.scalar_static_f64[928]*(v2378).ln()))}else{(if (v2368!=0.0){(v771-(self.scalar_static_f64[928]*(v2370).ln()))}else{v3})});
        let v2386=(v1-(v2382/self.scalar_static_f64[578]));
        let v2401=(v763/self.scalar_static_f64[935]);
        let v2403=(if (v2401<self.scalar_static_f64[214]){v1}else{v3});
        let v2404=(v2401).exp();
        let v2406=(!(v2403!=0.0));
        let v2407=(if v2406{self.scalar_static_f64[215]}else{v2197});
        let v2412=(self.scalar_static_f64[934]*(if v2406{(v2407*(v1+(v2401-self.scalar_static_f64[214])))}else{(if (v2403!=0.0){v2404}else{v1544})}));
        let v2417=(v1167*self.scalar_static_f64[939]);
        let v2418=(v34+v1156);
        let v2433=(self.scalar_static_f64[412]*((v793-self.scalar_static_f64[519])/self.scalar_static_f64[331]));
        let v2435=(if (v2433<self.scalar_static_f64[214]){v1}else{v3});
        let v2437=((v2435!=0.0)&&self.scalar_static_bool[64]);
        let v2438=(v2433).exp();
        let v2441=(self.scalar_static_bool[64]&&(!(v2435!=0.0)));
        let v2442=(if v2441{self.scalar_static_f64[215]}else{v2407});
        let v2448=(v836*self.scalar_static_f64[941]);
        let v2451=((v1+(v455*(if v2441{(v2442*(v1+(v2433-self.scalar_static_f64[214])))}else{(if v2437{v2438}else{v3})})))).sqrt();
        let v2452=(v1+v2451);
        let v2454=(if self.scalar_static_bool[64]{(v2448/v2452)}else{(if (self.scalar_static_f64[330]!=0.0){((self.scalar_static_f64[940]*(((v1737/v1740)*self.scalar_static_f64[926])+((v1736/v1744)*self.scalar_static_f64[938])))/self.scalar_static_f64[833])}else{v3})});
        let v2463=(if self.scalar_static_bool[68]{(v858*self.scalar_static_f64[886])}else{v3});
        let v2464=(v2463-self.scalar_static_f64[886]);
        let v2466=((v1+v2463)).sqrt();
        let v2467=(v1+v2466);
        let v2471=(if self.scalar_static_bool[68]{(v455*(if v898{(v899*(v1+(v893-self.scalar_static_f64[214])))}else{(if (v895!=0.0){v896}else{v3})}))}else{v3});
        let v2473=((v1+v2471)).sqrt();
        let v2474=(v1+v2473);
        let v2486=(self.scalar_static_f64[412]*(v798-self.scalar_static_f64[519]));
        let v2488=(if (v2486<self.scalar_static_f64[214]){v1}else{v3});
        let v2490=((v2488!=0.0)&&self.scalar_static_bool[69]);
        let v2491=(v2486).exp();
        let v2494=(self.scalar_static_bool[69]&&(!(v2488!=0.0)));
        let v2495=(if v2494{self.scalar_static_f64[215]}else{v2442});
        let v2501=(v858*self.scalar_static_f64[943]);
        let v2504=((v1+(v455*(if v2494{(v2495*(v1+(v2486-self.scalar_static_f64[214])))}else{(if v2490{v2491}else{v3})})))).sqrt();
        let v2505=(v1+v2504);
        let v2507=(if self.scalar_static_bool[69]{(v2501/v2505)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[942]*((self.scalar_static_f64[926]*(if self.scalar_static_bool[68]{(v2464/v2467)}else{v3}))+(self.scalar_static_f64[938]*(if self.scalar_static_bool[68]{(v2471/v2474)}else{v3}))))/self.scalar_static_f64[833])}else{v3})});
        let v2516=(if (self.scalar_static_f64[335]!=0.0){(f64::powf(v1198,self.scalar_static_f64[336])-v161)}else{v3});
        let v2517=(if (self.scalar_static_f64[335]!=0.0){v1180}else{v3});
        let v2519=(if (v2517<v3){v1}else{v3});
        let v2520=((self.scalar_static_f64[335]!=0.0)&&(v2519!=0.0));
        let v2521=(v2517).exp();
        let v2522=(v1+v2521);
        let v2526=((self.scalar_static_f64[335]!=0.0)&&(!(v2519!=0.0)));
        let v2528=((-v2517)).exp();
        let v2529=(v1+v2528);
        let v2531=(if v2526{(v2528/v2529)}else{(if v2520{(v1/v2522)}else{v3})});
        let v2538=((self.scalar_static_f64[412]*v1264)/self.scalar_static_f64[642]);
        let v2539=(v443/v1266);
        let v2541=(if (self.scalar_static_f64[335]!=0.0){(v2538*v2539)}else{v3});
        let v2542=(v1974*self.scalar_static_f64[927]);
        let v2547=(v768*0.2);
        let v2549=((if (self.scalar_static_f64[335]!=0.0){(v2412/self.scalar_static_f64[935])}else{v3})+((if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[923]*(if (self.scalar_static_f64[335]!=0.0){(v161+(v2516*v2531))}else{v3}))}else{v3})+(if (self.scalar_static_f64[335]!=0.0){(v2541*v2542)}else{v3})));
        let v2558=(if (self.scalar_static_f64[335]!=0.0){(v2292+(v2412*self.scalar_static_f64[337]))}else{v3});
        let v2567=(if self.scalar_static_bool[71]{v2292}else{(if (self.scalar_static_f64[335]!=0.0){(v2558*self.scalar_static_f64[340])}else{v3})});
        let v2568=(if self.scalar_static_bool[71]{v2294}else{(if (self.scalar_static_f64[335]!=0.0){(v2294+(v2558*self.scalar_static_f64[339]))}else{v3})});
        let v2570=(v1320+v1321);
        let v2571=(v2570/v1317);
        let v2581=(if (v2571>v3){v1}else{v3});
        let v2582=(v2567+v2568);
        let v2585=(!(v2581!=0.0));
        let v2586=(self.scalar_static_f64[829]*v1974);
        let v2588=(if v2585{(v1317*v2586)}else{(if (v2581!=0.0){(v2582/v2571)}else{v3})});
        let v2603=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(v2588*self.scalar_static_f64[346])}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v2588)}else{v3})})});
        let v2654=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v2412}else{(if (self.scalar_static_f64[335]!=0.0){(v2412*self.scalar_static_f64[338])}else{v3})})+((v1206*self.scalar_static_f64[923])+v2567)));
        let v2657=(self.scalar_static_f64[0]*(self.scalar_static_f64[924]*((self.scalar_static_f64[877]*(v1-f64::powf(v2278,self.scalar_static_f64[234])))+(v161*(v766-v2275)))));
        let v2660=(self.scalar_static_f64[0]*((v2417*v2418)+((v1261*self.scalar_static_f64[925])+v2568)));
        let v2663=(self.scalar_static_f64[0]*(self.scalar_static_f64[588]*((self.scalar_static_f64[930]*(v1-f64::powf(v2386,self.scalar_static_f64[326])))+(v34*(v771-v2382)))));
        let v2666=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){(v2547*v2549)}else{v3}));
        let v2670=((self.scalar_static_f64[0]*(v777-v774))*self.scalar_static_f64[349]);
        let v2674=(v784*self.scalar_static_f64[350]);
        let v2682=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(v1-f64::powf(v2347,self.scalar_static_f64[240])))+(self.scalar_static_f64[880]*(v798-v2345))))+(self.scalar_static_f64[595]*v798)))))+(if (self.scalar_static_f64[332]!=0.0){(v1900*v2507)}else{v3})));
        let v2688=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*((self.scalar_static_f64[594]*((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(v1-f64::powf(v2314,self.scalar_static_f64[240])))+(self.scalar_static_f64[880]*(v793-v2312))))+(self.scalar_static_f64[595]*v793)))*self.scalar_static_f64[322]))+(if (self.scalar_static_f64[332]!=0.0){(self.scalar_static_f64[7]*v2454)}else{v2454})));
        let v2699=ctx.node_voltage(nodes[11]);
        let v2715=(if v807{(v809*self.scalar_static_f64[944])}else{(if (v804!=0.0){(v805*self.scalar_static_f64[944])}else{v3})});
        let v2716=(if v807{(v809*self.scalar_static_f64[945])}else{(if (v804!=0.0){(v805*self.scalar_static_f64[945])}else{v3})});
        let v2725=(if v820{(v821*self.scalar_static_f64[946])}else{(if (v817!=0.0){(v818*self.scalar_static_f64[946])}else{v3})});
        let v2726=(if v820{(v821*self.scalar_static_f64[947])}else{(if (v817!=0.0){(v818*self.scalar_static_f64[947])}else{v3})});
        let v2741=(if v831{(v832*self.scalar_static_f64[944])}else{(if (v828!=0.0){(v829*self.scalar_static_f64[944])}else{v3})});
        let v2742=(if v831{(v832*self.scalar_static_f64[948])}else{(if (v828!=0.0){(v829*self.scalar_static_f64[948])}else{v3})});
        let v2743=(if v831{(v832*self.scalar_static_f64[949])}else{(if (v828!=0.0){(v829*self.scalar_static_f64[949])}else{v3})});
        let v2744=(if v831{(v832*self.scalar_static_f64[945])}else{(if (v828!=0.0){(v829*self.scalar_static_f64[945])}else{v3})});
        let v2766=(if v853{(v854*self.scalar_static_f64[948])}else{(if (v850!=0.0){(v851*self.scalar_static_f64[948])}else{v3})});
        let v2767=(if v853{(v854*self.scalar_static_f64[950])}else{(if (v850!=0.0){(v851*self.scalar_static_f64[950])}else{v3})});
        let v2768=(if v853{(v854*self.scalar_static_f64[949])}else{(if (v850!=0.0){(v851*self.scalar_static_f64[949])}else{v3})});
        let v2769=(if v853{(v854*self.scalar_static_f64[945])}else{(if (v850!=0.0){(v851*self.scalar_static_f64[945])}else{v3})});
        let v2787=(if v875{(v876*self.scalar_static_f64[944])}else{(if (v872!=0.0){(v873*self.scalar_static_f64[944])}else{v3})});
        let v2788=(if v875{(v876*self.scalar_static_f64[949])}else{(if (v872!=0.0){(v873*self.scalar_static_f64[949])}else{v3})});
        let v2789=(if v875{(v876*self.scalar_static_f64[945])}else{(if (v872!=0.0){(v873*self.scalar_static_f64[945])}else{v3})});
        let v2840=(if v922{(v923*self.scalar_static_f64[944])}else{(if (v919!=0.0){(v920*self.scalar_static_f64[944])}else{v3})});
        let v2841=(if v922{(v923*self.scalar_static_f64[945])}else{(if (v919!=0.0){(v920*self.scalar_static_f64[945])}else{v3})});
        let v2848=(if v934{(v935*self.scalar_static_f64[944])}else{(if (v931!=0.0){(v932*self.scalar_static_f64[944])}else{v3})});
        let v2849=(if v934{(v935*self.scalar_static_f64[945])}else{(if (v931!=0.0){(v932*self.scalar_static_f64[945])}else{v3})});
        let v2852=(v34*v942);
        let v2853=((v455*v2840)/v2852);
        let v2854=((v455*v2841)/v2852);
        let v2857=(v34*v945);
        let v2858=((v455*v2848)/v2857);
        let v2859=((v455*v2849)/v2857);
        let v2865=(v947*v947);
        let v2871=(if (v951!=0.0){v3}else{(((v947*(v34*v2848))-(v946*v2858))/v2865)});
        let v2872=(if (v951!=0.0){v3}else{(((v947*(v34*v2849))-(v946*v2859))/v2865)});
        let v2889=(self.scalar_static_f64[411]*((v2853-v2858)-((((v947*v2853)-(v954*v2858))/v2865)/v955)));
        let v2890=(self.scalar_static_f64[411]*((-v2859)-(((-(v954*v2859))/v2865)/v955)));
        let v2891=(self.scalar_static_f64[411]*(v2854-((v2854/v947)/v955)));
        let v2893=(self.scalar_static_f64[351]+v2891);
        let v2894=(v2889/self.scalar_static_f64[618]);
        let v2895=((self.scalar_static_f64[0]+v2890)/self.scalar_static_f64[618]);
        let v2896=(v2893/self.scalar_static_f64[618]);
        let v2906=(self.scalar_static_f64[618]*(v443*v2894));
        let v2907=(self.scalar_static_f64[618]*(v443*v2895));
        let v2908=(self.scalar_static_f64[618]*(v443*v2896));
        let v2920=(if (v962!=0.0){((self.scalar_static_f64[865]*((self.scalar_static_f64[412]*v2906)/v979))-(if v969{(self.scalar_static_f64[0]/v971)}else{(if v966{self.scalar_static_f64[0]}else{v3})}))}else{v3});
        let v2921=(if (v962!=0.0){((self.scalar_static_f64[865]*((self.scalar_static_f64[412]*v2907)/v979))-(if v969{(self.scalar_static_f64[351]/v971)}else{(if v966{self.scalar_static_f64[351]}else{v3})}))}else{v3});
        let v2922=(if (v962!=0.0){(self.scalar_static_f64[865]*((self.scalar_static_f64[412]*v2908)/v979))}else{v3});
        let v2923=(v984*v2920);
        let v2925=(v984*v2921);
        let v2927=(v984*v2922);
        let v2932=(v34*v997);
        let v2933=((if (v962!=0.0){(v2923+v2923)}else{v3})/v2932);
        let v2934=((if (v962!=0.0){(v2925+v2925)}else{v3})/v2932);
        let v2935=((if (v962!=0.0){(v2927+v2927)}else{v3})/v2932);
        let v2941=(v998*v998);
        let v2958=(if v1002{(v443*(v2920+v2933))}else{(if v994{((-(v995*(v2933-v2920)))/v2941)}else{v3})});
        let v2959=(if v1002{(v443*(v2921+v2934))}else{(if v994{((-(v995*(v2934-v2921)))/v2941)}else{v3})});
        let v2960=(if v1002{(v443*(v2922+v2935))}else{(if v994{((-(v995*(v2935-v2922)))/v2941)}else{v3})});
        let v2976=(v1013*v1013);
        let v2986=(if (v962!=0.0){(((v1013*((v1009*v2958)+(v1005*v2958)))-(v1010*(self.scalar_static_f64[218]*v2958)))/v2976)}else{v3});
        let v2987=(if (v962!=0.0){(((v1013*((v1009*v2959)+(v1005*v2959)))-(v1010*(self.scalar_static_f64[218]*v2959)))/v2976)}else{v3});
        let v2988=(if (v962!=0.0){(((v1013*((v1009*v2960)+(v1005*v2960)))-(v1010*(self.scalar_static_f64[218]*v2960)))/v2976)}else{v3});
        let v2992=(v1015*v1015);
        let v3002=(if (v962!=0.0){(((v1015*v2894)-(v960*v2986))/v2992)}else{v3});
        let v3003=(if (v962!=0.0){(((v1015*v2895)-(v960*v2987))/v2992)}else{v3});
        let v3004=(if (v962!=0.0){(((v1015*v2896)-(v960*v2988))/v2992)}else{v3});
        let v3008=(if (v962!=0.0){(v3002/self.scalar_static_f64[220])}else{v3});
        let v3009=(if (v962!=0.0){(v3003/self.scalar_static_f64[220])}else{v3});
        let v3010=(if (v962!=0.0){(v3004/self.scalar_static_f64[220])}else{v3});
        let v3044=(if (v962!=0.0){((if v1032{(v3002+(self.scalar_static_f64[220]*((v1034*(-v3008))/v1035)))}else{(if v1024{(self.scalar_static_f64[220]*((v1025*v3008)/v1026))}else{v3})})/self.scalar_static_f64[226])}else{v3});
        let v3045=(if (v962!=0.0){((if v1032{(v3003+(self.scalar_static_f64[220]*((v1034*(-v3009))/v1035)))}else{(if v1024{(self.scalar_static_f64[220]*((v1025*v3009)/v1026))}else{v3})})/self.scalar_static_f64[226])}else{v3});
        let v3046=(if (v962!=0.0){((if v1032{(v3004+(self.scalar_static_f64[220]*((v1034*(-v3010))/v1035)))}else{(if v1024{(self.scalar_static_f64[220]*((v1025*v3010)/v1026))}else{v3})})/self.scalar_static_f64[226])}else{v3});
        let v3050=(if (v962!=0.0){(v2958/self.scalar_static_f64[219])}else{v3});
        let v3051=(if (v962!=0.0){(v2959/self.scalar_static_f64[219])}else{v3});
        let v3052=(if (v962!=0.0){(v2960/self.scalar_static_f64[219])}else{v3});
        let v3074=(v34*v1056);
        let v3093=(v1059*v1059);
        let v3103=(if (v962!=0.0){(((v1059*(((v1053*((v1051*v3050)+(v1050*(v455*v3044))))+(v1052*v3050))/v3074))-(v1057*((v1058*v3050)+(v1053*(v34*v3044)))))/v3093)}else{v3});
        let v3104=(if (v962!=0.0){(((v1059*(((v1053*((v1051*v3051)+(v1050*(v455*v3045))))+(v1052*v3051))/v3074))-(v1057*((v1058*v3051)+(v1053*(v34*v3045)))))/v3093)}else{v3});
        let v3105=(if (v962!=0.0){(((v1059*(((v1053*((v1051*v3052)+(v1050*(v455*v3046))))+(v1052*v3052))/v3074))-(v1057*((v1058*v3052)+(v1053*(v34*v3046)))))/v3093)}else{v3});
        let v3111=((v1061*v2871)+(v952*v3103));
        let v3114=((v1061*v2872)+(v952*v3104));
        let v3115=(v952*v3105);
        let v3122=(v1065*v1065);
        let v3132=(if (v962!=0.0){(((v1065*((-v3103)+v3111))-(v1064*v3111))/v3122)}else{v3});
        let v3133=(if (v962!=0.0){(((v1065*((-v3104)+v3114))-(v1064*v3114))/v3122)}else{v3});
        let v3134=(if (v962!=0.0){(((v1065*((-v3105)+v3115))-(v1064*v3115))/v3122)}else{v3});
        let v3147=(if (v962!=0.0){(self.scalar_static_f64[412]*((v1067*v2906)+(v977*v3132)))}else{v3});
        let v3148=(if (v962!=0.0){(self.scalar_static_f64[412]*((v1067*v2907)+(v977*v3133)))}else{v3});
        let v3149=(if (v962!=0.0){(self.scalar_static_f64[412]*((v1067*v2908)+(v977*v3134)))}else{v3});
        let v3165=(if (v962!=0.0){((v34*v3147)+((v1073*v2871)+(v952*(v2871+v3147))))}else{v3});
        let v3166=(if (v962!=0.0){((v34*v3148)+((v1073*v2872)+(v952*(v2872+v3148))))}else{v3});
        let v3167=(if (v962!=0.0){((v34*v3149)+(v952*v3149))}else{v3});
        let v3171=(if (v962!=0.0){(v443*v3147)}else{v3});
        let v3172=(if (v962!=0.0){(v443*v3148)}else{v3});
        let v3173=(if (v962!=0.0){(v443*v3149)}else{v3});
        let v3174=(v1079*v3171);
        let v3176=(v1079*v3172);
        let v3178=(v1079*v3173);
        let v3183=(if (v962!=0.0){(v3165+(v3174+v3174))}else{v3});
        let v3184=(if (v962!=0.0){(v3166+(v3176+v3176))}else{v3});
        let v3185=(if (v962!=0.0){(v3167+(v3178+v3178))}else{v3});
        let v3186=(v34*v1086);
        let v3187=(v3183/v3186);
        let v3188=(v3184/v3186);
        let v3189=(v3185/v3186);
        let v3202=(v1091*v1091);
        let v3215=(if v1097{v3}else{(if v1090{(((v1091*v3165)-(v1076*(v3187-v3171)))/v3202)}else{(if v1085{(v3171+v3187)}else{v3})})});
        let v3216=(if v1097{v3}else{(if v1090{(((v1091*v3166)-(v1076*(v3188-v3172)))/v3202)}else{(if v1085{(v3172+v3188)}else{v3})})});
        let v3217=(if v1097{v3}else{(if v1090{(((v1091*v3167)-(v1076*(v3189-v3173)))/v3202)}else{(if v1085{(v3173+v3189)}else{v3})})});
        let v3236=(if (v962!=0.0){(self.scalar_static_f64[228]*v2894)}else{v3});
        let v3237=(if (v962!=0.0){(self.scalar_static_f64[228]*v2895)}else{v3});
        let v3238=(if (v962!=0.0){(self.scalar_static_f64[228]*v2896)}else{v3});
        let v3245=(v1108*v3236);
        let v3247=(v1108*v3237);
        let v3249=(v1108*v3238);
        let v3254=(v34*v1115);
        let v3273=(v1127*v1127);
        let v3289=(self.scalar_static_f64[217]*v2894);
        let v3290=(self.scalar_static_f64[217]*v2895);
        let v3291=(self.scalar_static_f64[217]*v2896);
        let v3295=(v1133*v1133);
        let v3322=(v954*v954);
        let v3330=(if v1138{(((v954*(v34*v2841))-(v1139*v2854))/v3322)}else{v3217});
        let v3331=(if v1138{v2715}else{(if (v962!=0.0){(self.scalar_static_f64[869]*((v1099*v3215)+(v1098*v3215)))}else{v3})});
        let v3332=(if v1138{v3}else{(if (v962!=0.0){(self.scalar_static_f64[869]*((v1099*v3216)+(v1098*v3216)))}else{v3})});
        let v3333=(if v1138{v2716}else{(if (v962!=0.0){(self.scalar_static_f64[869]*((v1099*v3217)+(v1098*v3217)))}else{v3})});
        let v3334=(v2871+(if v1138{(((v954*(v34*v2840))-(v1139*v2853))/v3322)}else{v3215}));
        let v3335=(v2872+(if v1138{v3}else{v3216}));
        let v3339=(if v1155{(v443*v3334)}else{v3});
        let v3340=(if v1155{(v443*v3335)}else{v3});
        let v3341=(if v1155{(v443*v3330)}else{v3});
        let v3345=(v1159*v1159);
        let v3364=(v1165*v1165);
        let v3374=(if v1163{(((v1165*v2889)-(v958*((self.scalar_static_f64[0]+v2889)-self.scalar_static_f64[0])))/v3364)}else{(if v1155{(((v1159*v3339)-(v1158*v3339))/v3345)}else{v3132})});
        let v3375=(if v1163{(((v1165*v2890)-(v958*(v2890-self.scalar_static_f64[351])))/v3364)}else{(if v1155{(((v1159*v3340)-(v1158*v3340))/v3345)}else{v3133})});
        let v3376=(if v1163{(((v1165*v2891)-(v958*v2893))/v3364)}else{(if v1155{(((v1159*v3341)-(v1158*v3341))/v3345)}else{v3134})});
        let v3380=(if v1138{v3}else{(if v1125{(self.scalar_static_f64[538]*(((v1127*(v34*v2894))-(v1126*(v2894+v2986)))/v3273))}else{v3})});
        let v3381=(if v1138{v3}else{(if v1125{(self.scalar_static_f64[538]*(((v1127*(v34*v2895))-(v1126*(v2895+v2987)))/v3273))}else{v3})});
        let v3382=(if v1138{v3}else{(if v1125{(self.scalar_static_f64[538]*(((v1127*(v34*v2896))-(v1126*(v2896+v2988)))/v3273))}else{v3})});
        let v3383=(if v1138{v2894}else{(if (v962!=0.0){(((v1133*v3289)-(v1132*v2894))/v3295)}else{v3})});
        let v3384=(if v1138{v2895}else{(if (v962!=0.0){(((v1133*v3290)-(v1132*v2895))/v3295)}else{v3})});
        let v3385=(if v1138{v2896}else{(if (v962!=0.0){(((v1133*v3291)-(v1132*v2896))/v3295)}else{v3})});
        let v3392=(if v1138{(-(v3383/self.scalar_static_f64[217]))}else{(if (v962!=0.0){((-v3289)/v3295)}else{v3})});
        let v3393=(if v1138{(-(v3384/self.scalar_static_f64[217]))}else{(if (v962!=0.0){((-v3290)/v3295)}else{v3})});
        let v3394=(if v1138{(-(v3385/self.scalar_static_f64[217]))}else{(if (v962!=0.0){((-v3291)/v3295)}else{v3})});
        let v3417=(if v1189{(-(self.scalar_static_f64[876]*((v1191*self.scalar_static_f64[953])/v1192)))}else{(if (v1182!=0.0){(self.scalar_static_f64[351]-(self.scalar_static_f64[876]*((v1183*self.scalar_static_f64[951])/v1184)))}else{v3})});
        let v3418=(if v1189{(-(self.scalar_static_f64[876]*((v1191*self.scalar_static_f64[954])/v1192)))}else{(if (v1182!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[876]*((v1183*self.scalar_static_f64[952])/v1184)))}else{v3})});
        let v3421=(-(self.scalar_static_f64[579]*v3417));
        let v3422=(-(self.scalar_static_f64[579]*v3418));
        let v3425=(self.scalar_static_f64[234]*f64::powf(v1198,self.scalar_static_f64[355]));
        let v3426=(v3421*v3425);
        let v3427=(v3422*v3425);
        let v3436=((self.scalar_static_f64[877]*(-v3426))+(v161*(self.scalar_static_f64[351]-v3417)));
        let v3437=((self.scalar_static_f64[877]*(-v3427))+(v161*(self.scalar_static_f64[0]-v3418)));
        let v3445=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v1138{v3}else{(if (v962!=0.0){(v3236+(((if (v962!=0.0){(self.scalar_static_f64[871]*v2894)}else{v3})+(v3245+v3245))/v3254))}else{v3})}))}else{self.scalar_static_f64[356]})});
        let v3446=(if self.scalar_static_bool[26]{v3}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[351]+(if v1138{self.scalar_static_f64[0]}else{(if (v962!=0.0){(v3237+(((if (v962!=0.0){(self.scalar_static_f64[871]*v2895)}else{v3})+(v3247+v3247))/v3254))}else{v3})}))}else{self.scalar_static_f64[357]})});
        let v3447=(if self.scalar_static_bool[26]{self.scalar_static_f64[351]}else{(if self.scalar_static_bool[24]{(if v1138{self.scalar_static_f64[351]}else{(if (v962!=0.0){(v3238+(((if (v962!=0.0){(self.scalar_static_f64[871]*v2896)}else{v3})+(v3249+v3249))/v3254))}else{v3})})}else{v3})});
        let v3451=(v1169*v1169);
        let v3452=(((v1169*v3445)-(v1227*v3380))/v3451);
        let v3456=(((v1169*v3446)-(v1227*v3381))/v3451);
        let v3460=(((v1169*v3447)-(v1227*v3382))/v3451);
        let v3503=(if v1237{(-((v1241*v3380)+(v1169*((v1239*(-v3452))/v1240))))}else{(if (v1230!=0.0){(v3445-((v1233*v3380)+(v1169*((v1231*v3452)/v1232))))}else{v3})});
        let v3504=(if v1237{(-((v1241*v3381)+(v1169*((v1239*(-v3456))/v1240))))}else{(if (v1230!=0.0){(v3446-((v1233*v3381)+(v1169*((v1231*v3456)/v1232))))}else{v3})});
        let v3505=(if v1237{(-((v1241*v3382)+(v1169*((v1239*(-v3460))/v1240))))}else{(if (v1230!=0.0){(v3447-((v1233*v3382)+(v1169*((v1231*v3460)/v1232))))}else{v3})});
        let v3508=(self.scalar_static_f64[239]*f64::powf(v1173,self.scalar_static_f64[358]));
        let v3509=(v3392*v3508);
        let v3510=(v3393*v3508);
        let v3511=(v3394*v3508);
        let v3520=(self.scalar_static_f64[240]*f64::powf(v1250,self.scalar_static_f64[359]));
        let v3559=(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((v1251*v3511)+(v1246*((-(v3505/self.scalar_static_f64[538]))*v3520)))))+((v1256*(self.scalar_static_f64[880]*v3511))+(v1255*(v3447-v3505)))));
        let v3562=((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((v1251*v3509)+(v1246*((-(v3503/self.scalar_static_f64[538]))*v3520)))))+((v1256*(self.scalar_static_f64[880]*v3509))+(v1255*(v3445-v3503)))))+self.scalar_static_f64[955]);
        let v3563=((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((v1251*v3510)+(v1246*((-(v3504/self.scalar_static_f64[538]))*v3520)))))+((v1256*(self.scalar_static_f64[880]*v3510))+(v1255*(v3446-v3504)))))+self.scalar_static_f64[956]);
        let v3564=(self.scalar_static_f64[886]*v2725);
        let v3565=(self.scalar_static_f64[886]*v2726);
        let v3566=(v34*v1266);
        let v3567=(v3564/v3566);
        let v3568=(v3565/v3566);
        let v3572=(v1267*v1267);
        let v3573=(((v1267*v3564)-(v1264*v3567))/v3572);
        let v3577=(((v1267*v3565)-(v1264*v3568))/v3572);
        let v3580=(self.scalar_static_f64[887]*f64::powf(v1142,self.scalar_static_f64[957]));
        let v3581=(v3331*v3580);
        let v3582=(v3332*v3580);
        let v3583=(v3333*v3580);
        let v3584=(self.scalar_static_f64[886]*v3581);
        let v3585=(self.scalar_static_f64[886]*v3582);
        let v3586=(self.scalar_static_f64[886]*v3583);
        let v3587=(v34*v1273);
        let v3594=(v1274*v1274);
        let v3595=(((v1274*v3584)-(v1271*(v3584/v3587)))/v3594);
        let v3599=(((v1274*v3585)-(v1271*(v3585/v3587)))/v3594);
        let v3603=(((v1274*v3586)-(v1271*(v3586/v3587)))/v3594);
        let v3604=(v3436/self.scalar_static_f64[804]);
        let v3605=(v3437/self.scalar_static_f64[804]);
        let v3606=(v3562/self.scalar_static_f64[802]);
        let v3607=(v3563/self.scalar_static_f64[802]);
        let v3608=(v3559/self.scalar_static_f64[802]);
        let v3609=(v3605+v3606);
        let v3647=(if self.scalar_static_bool[28]{((v1292*(if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*v3604))}else{v3}))/self.scalar_static_f64[890])}else{(if (self.scalar_static_f64[241]!=0.0){v3604}else{v3})});
        let v3648=(if self.scalar_static_bool[28]{(((v1292*(if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*v3605))}else{v3}))-(v1293*(if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*((-v3562)/self.scalar_static_f64[802])))}else{v3})))/self.scalar_static_f64[890])}else{(if (self.scalar_static_f64[241]!=0.0){v3609}else{v3})});
        let v3649=(if self.scalar_static_bool[28]{((-(v1293*(if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*((-v3563)/self.scalar_static_f64[802])))}else{v3})))/self.scalar_static_f64[890])}else{(if (self.scalar_static_f64[241]!=0.0){v3607}else{v3})});
        let v3650=(if self.scalar_static_bool[28]{((-(v1293*(if self.scalar_static_bool[28]{(self.scalar_static_f64[412]*(self.scalar_static_f64[849]*((-v3559)/self.scalar_static_f64[802])))}else{v3})))/self.scalar_static_f64[890])}else{(if (self.scalar_static_f64[241]!=0.0){v3608}else{v3})});
        let v3651=(v1299*v3647);
        let v3652=(v3651+v3651);
        let v3653=(v1299*v3648);
        let v3654=(v3653+v3653);
        let v3655=(v1299*v3649);
        let v3656=(v3655+v3655);
        let v3657=(v1299*v3650);
        let v3658=(v3657+v3657);
        let v3659=(v34*v1306);
        let v3660=(v3652/v3659);
        let v3661=(v3654/v3659);
        let v3662=(v3656/v3659);
        let v3663=(v3658/v3659);
        let v3670=(v1307*v1307);
        let v3698=(v443*v3573);
        let v3699=(v443*(v3577+v3595));
        let v3700=(v443*v3599);
        let v3701=(v443*v3603);
        let v3704=((v1316*(if v1310{(v443*(v3647+v3660))}else{(if (v1303!=0.0){((-(v1304*(v3660-v3647)))/v3670)}else{v3})}))+(v1313*v3698));
        let v3707=((v1316*(if v1310{(v443*(v3648+v3661))}else{(if (v1303!=0.0){((-(v1304*(v3661-v3648)))/v3670)}else{v3})}))+(v1313*v3699));
        let v3710=((v1316*(if v1310{(v443*(v3649+v3662))}else{(if (v1303!=0.0){((-(v1304*(v3662-v3649)))/v3670)}else{v3})}))+(v1313*v3700));
        let v3713=((v1316*(if v1310{(v443*(v3650+v3663))}else{(if (v1303!=0.0){((-(v1304*(v3663-v3650)))/v3670)}else{v3})}))+(v1313*v3701));
        let v3714=(self.scalar_static_f64[891]*v3581);
        let v3715=(self.scalar_static_f64[891]*v3582);
        let v3716=(self.scalar_static_f64[891]*v3583);
        let v3718=(self.scalar_static_f64[687]*v2726);
        let v3722=(v1317*(self.scalar_static_f64[687]*v2725));
        let v3725=(v1317*v1317);
        let v3759=(if v1333{(self.scalar_static_f64[351]+(v1324*((v1335*self.scalar_static_f64[362])/v1336)))}else{(if (v1327!=0.0){(v1324*((v1328*self.scalar_static_f64[360])/v1329))}else{v3})});
        let v3760=(if v1333{(self.scalar_static_f64[0]+(v1324*((v1335*self.scalar_static_f64[363])/v1336)))}else{(if (v1327!=0.0){(v1324*((v1328*self.scalar_static_f64[361])/v1329))}else{v3})});
        let v3819=(if v1384{(v1385*self.scalar_static_f64[958])}else{(if (v1381!=0.0){(v1382*self.scalar_static_f64[958])}else{v3759})});
        let v3820=(if v1384{(v1385*self.scalar_static_f64[959])}else{(if (v1381!=0.0){(v1382*self.scalar_static_f64[959])}else{v3760})});
        let v3953=(if v1463{(v1464*self.scalar_static_f64[960])}else{(if (v1460!=0.0){(v1461*self.scalar_static_f64[960])}else{v3819})});
        let v3954=(if v1463{(v1464*self.scalar_static_f64[961])}else{(if (v1460!=0.0){(v1461*self.scalar_static_f64[961])}else{v3})});
        let v3955=(if v1463{v3}else{(if (v1460!=0.0){v3}else{v3820})});
        let v4010=(if v1500{(v1501*self.scalar_static_f64[962])}else{(if (v1497!=0.0){(v1498*self.scalar_static_f64[962])}else{v3953})});
        let v4011=(if v1500{v3}else{(if (v1497!=0.0){v3}else{v3954})});
        let v4012=(if v1500{(v1501*self.scalar_static_f64[963])}else{(if (v1497!=0.0){(v1498*self.scalar_static_f64[963])}else{v3955})});
        let v4025=(if v1513{(v1514*self.scalar_static_f64[964])}else{(if (v1510!=0.0){(v1511*self.scalar_static_f64[964])}else{v4010})});
        let v4026=(if v1513{(v1514*self.scalar_static_f64[965])}else{(if (v1510!=0.0){(v1511*self.scalar_static_f64[965])}else{v4011})});
        let v4027=(if v1513{v3}else{(if (v1510!=0.0){v3}else{v4012})});
        let v4048=(if v1526{v3}else{(if (v1523!=0.0){v3}else{v4025})});
        let v4049=(if v1526{(v1527*self.scalar_static_f64[966])}else{(if (v1523!=0.0){(v1524*self.scalar_static_f64[966])}else{v4026})});
        let v4050=(if v1526{(v1527*self.scalar_static_f64[967])}else{(if (v1523!=0.0){(v1524*self.scalar_static_f64[967])}else{v4027})});
        let v4051=(if v1526{(v1527*self.scalar_static_f64[968])}else{(if (v1523!=0.0){(v1524*self.scalar_static_f64[968])}else{v3})});
        let v4052=(if v1526{(v1527*self.scalar_static_f64[969])}else{(if (v1523!=0.0){(v1524*self.scalar_static_f64[969])}else{v3})});
        let v4069=(if v1539{(v1540*self.scalar_static_f64[970])}else{(if (v1536!=0.0){(v1537*self.scalar_static_f64[970])}else{v4048})});
        let v4070=(if v1539{(v1540*self.scalar_static_f64[971])}else{(if (v1536!=0.0){(v1537*self.scalar_static_f64[971])}else{v4049})});
        let v4071=(if v1539{v3}else{(if (v1536!=0.0){v3}else{v4050})});
        let v4072=(if v1539{v3}else{(if (v1536!=0.0){v3}else{v4051})});
        let v4073=(if v1539{v3}else{(if (v1536!=0.0){v3}else{v4052})});
        let v4411=(self.scalar_static_f64[886]*v2741);
        let v4412=(self.scalar_static_f64[886]*v2742);
        let v4413=(self.scalar_static_f64[886]*v2743);
        let v4414=(self.scalar_static_f64[886]*v2744);
        let v4415=(v455*(if v910{(v911*self.scalar_static_f64[944])}else{(if (v907!=0.0){(v908*self.scalar_static_f64[944])}else{v3})}));
        let v4416=(v455*(if v910{(v911*self.scalar_static_f64[948])}else{(if (v907!=0.0){(v908*self.scalar_static_f64[948])}else{v3})}));
        let v4417=(v455*(if v910{(v911*self.scalar_static_f64[949])}else{(if (v907!=0.0){(v908*self.scalar_static_f64[949])}else{v3})}));
        let v4418=(v455*(if v910{(v911*self.scalar_static_f64[945])}else{(if (v907!=0.0){(v908*self.scalar_static_f64[945])}else{v3})}));
        let v4419=(v34*v1739);
        let v4427=(v1740*v1740);
        let v4441=(v34*v1743);
        let v4449=(v1744*v1744);
        let v4683=(v34*v1832);
        let v4691=(v1833*v1833);
        let v4705=(if (self.scalar_static_f64[266]!=0.0){(((v1833*(self.scalar_static_f64[906]*v2766))-(v1829*((self.scalar_static_f64[898]*v2766)/v4683)))/v4691)}else{v3});
        let v4706=(if (self.scalar_static_f64[266]!=0.0){(((v1833*(self.scalar_static_f64[906]*v2767))-(v1829*((self.scalar_static_f64[898]*v2767)/v4683)))/v4691)}else{v3});
        let v4707=(if (self.scalar_static_f64[266]!=0.0){(((v1833*(self.scalar_static_f64[906]*v2768))-(v1829*((self.scalar_static_f64[898]*v2768)/v4683)))/v4691)}else{v3});
        let v4708=(if (self.scalar_static_f64[266]!=0.0){(((v1833*(self.scalar_static_f64[906]*v2769))-(v1829*((self.scalar_static_f64[898]*v2769)/v4683)))/v4691)}else{v3});
        let v4712=(self.scalar_static_f64[907]*v2766);
        let v4713=(self.scalar_static_f64[907]*v2767);
        let v4716=(self.scalar_static_f64[907]*v2768);
        let v4723=(self.scalar_static_f64[909]*v2766);
        let v4724=(self.scalar_static_f64[909]*v2767);
        let v4727=(self.scalar_static_f64[909]*v2768);
        let v4729=(v34*v1848);
        let v4739=(v1849*v1849);
        let v4769=(v34*v1856);
        let v4777=(v1857*v1857);
        let v4786=(((v1857*v4716)-(v1853*(v4727/v4769)))/v4777);
        let v4791=(if self.scalar_static_bool[46]{(((v1857*v4712)-(v1853*(v4723/v4769)))/v4777)}else{(if self.scalar_static_bool[45]{(((v1849*v4712)-(v1841*(v4723/v4729)))/v4739)}else{v3})});
        let v4792=(if self.scalar_static_bool[46]{(((v1857*v4713)-(v1853*(v4724/v4769)))/v4777)}else{(if self.scalar_static_bool[45]{(((v1849*v4713)-(v1841*(v4724/v4729)))/v4739)}else{v3})});
        let v4793=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[45]{(((v1849*(self.scalar_static_f64[907]*(-v2787)))-(v1841*((self.scalar_static_f64[909]*(self.scalar_static_f64[261]*v2787))/v4729)))/v4739)}else{v3})});
        let v4794=(if self.scalar_static_bool[46]{v4786}else{(if self.scalar_static_bool[45]{(((v1849*(self.scalar_static_f64[907]*(v2768-v2788)))-(v1841*((self.scalar_static_f64[909]*(v2768+(self.scalar_static_f64[261]*v2788)))/v4729)))/v4739)}else{v3})});
        let v4795=(if self.scalar_static_bool[46]{v4786}else{(if self.scalar_static_bool[45]{(((v1849*v4716)-(v1841*(v4727/v4729)))/v4739)}else{v3})});
        let v4796=(if self.scalar_static_bool[46]{(((v1857*(self.scalar_static_f64[907]*v2769))-(v1853*((self.scalar_static_f64[909]*v2769)/v4769)))/v4777)}else{(if self.scalar_static_bool[45]{(((v1849*(self.scalar_static_f64[907]*(v2769-v2789)))-(v1841*((self.scalar_static_f64[909]*(v2769+(self.scalar_static_f64[261]*v2789)))/v4729)))/v4739)}else{v3})});
        let v4801=(v1873*self.scalar_static_f64[378]);
        let v4802=(v4801+v4801);
        let v4803=(v1873*self.scalar_static_f64[379]);
        let v4805=(v1873*self.scalar_static_f64[380]);
        let v4806=(v4805+v4805);
        let v4807=(v1873*self.scalar_static_f64[381]);
        let v4809=(if self.scalar_static_bool[48]{v4802}else{v3});
        let v4810=(if self.scalar_static_bool[48]{(v4803+v4803)}else{v3});
        let v4811=(if self.scalar_static_bool[48]{v3}else{v3652});
        let v4812=(if self.scalar_static_bool[48]{v4802}else{v3654});
        let v4813=(if self.scalar_static_bool[48]{v4806}else{v3656});
        let v4814=(if self.scalar_static_bool[48]{v4806}else{v3658});
        let v4815=(if self.scalar_static_bool[48]{(v4807+v4807)}else{v3});
        let v4816=(if self.scalar_static_bool[48]{v4806}else{v3});
        let v4817=(v34*v1883);
        let v4818=(v4809/v4817);
        let v4819=(v4810/v4817);
        let v4820=(v4811/v4817);
        let v4821=(v4812/v4817);
        let v4822=(v4813/v4817);
        let v4823=(v4814/v4817);
        let v4824=(v4815/v4817);
        let v4825=(v4816/v4817);
        let v4835=(v1884*v1884);
        let v4881=(if v1888{(v443*(self.scalar_static_f64[378]+v4818))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4818-self.scalar_static_f64[378])))/v4835)}else{v3})});
        let v4882=(if v1888{(v443*(self.scalar_static_f64[379]+v4819))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4819-self.scalar_static_f64[379])))/v4835)}else{v3})});
        let v4883=(if v1888{(v443*v4820)}else{(if v1880{((-(self.scalar_static_f64[272]*v4820))/v4835)}else{v3})});
        let v4884=(if v1888{(v443*(self.scalar_static_f64[378]+v4821))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4821-self.scalar_static_f64[378])))/v4835)}else{v3})});
        let v4885=(if v1888{(v443*(self.scalar_static_f64[380]+v4822))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4822-self.scalar_static_f64[380])))/v4835)}else{v3})});
        let v4886=(if v1888{(v443*(self.scalar_static_f64[380]+v4823))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4823-self.scalar_static_f64[380])))/v4835)}else{v3})});
        let v4887=(if v1888{(v443*(self.scalar_static_f64[381]+v4824))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4824-self.scalar_static_f64[381])))/v4835)}else{v3})});
        let v4888=(if v1888{(v443*(self.scalar_static_f64[380]+v4825))}else{(if v1880{((-(self.scalar_static_f64[272]*(v4825-self.scalar_static_f64[380])))/v4835)}else{v3})});
        let v4894=(self.scalar_static_f64[611]*(v4705+v4791));
        let v4897=(self.scalar_static_f64[611]*(v4707+v4794));
        let v4910=(v1895*v1895);
        let v4952=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4881)-(v1891*(v4881+v4894)))/v4910)}else{v3})});
        let v4953=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4882)-(v1891*(v4882+(self.scalar_static_f64[611]*(v4706+v4792)))))/v4910)}else{v3})});
        let v4954=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{((-(v1891*(self.scalar_static_f64[611]*v4793)))/v4910)}else{v3})});
        let v4955=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4883)-(v1891*v4883))/v4910)}else{v3})});
        let v4956=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4884)-(v1891*(v4884+v4894)))/v4910)}else{v3})});
        let v4957=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4885)-(v1891*(v4885+v4897)))/v4910)}else{v3})});
        let v4958=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4886)-(v1891*(v4886+(self.scalar_static_f64[611]*(v4707+v4795)))))/v4910)}else{v3})});
        let v4959=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4887)-(v1891*(v4887+(self.scalar_static_f64[611]*(v4708+v4796)))))/v4910)}else{v3})});
        let v4960=(if self.scalar_static_bool[50]{v3}else{(if self.scalar_static_bool[48]{(((v1895*v4888)-(v1891*(v4888+v4897)))/v4910)}else{v3})});
        let v5258=(v1281*v3604);
        let v5260=(v1281*v3609);
        let v5262=(v1281*v3607);
        let v5264=(v1281*v3608);
        let v5266=(v34*v1967);
        let v5267=((v5258+v5258)/v5266);
        let v5268=((v5260+v5260)/v5266);
        let v5269=((v5262+v5262)/v5266);
        let v5270=((v5264+v5264)/v5266);
        let v5277=(v1968*v1968);
        let v5300=(if v1971{(v443*(v3604+v5267))}else{(if (v1965!=0.0){((-(v1304*(v5267-v3604)))/v5277)}else{v3})});
        let v5301=(if v1971{(v443*(v3609+v5268))}else{(if (v1965!=0.0){((-(v1304*(v5268-v3609)))/v5277)}else{v3})});
        let v5302=(if v1971{(v443*(v3607+v5269))}else{(if (v1965!=0.0){((-(v1304*(v5269-v3607)))/v5277)}else{v3})});
        let v5303=(if v1971{(v443*(v3608+v5270))}else{(if (v1965!=0.0){((-(v1304*(v5270-v3608)))/v5277)}else{v3})});
        let v6181=(if v2268{(-(self.scalar_static_f64[876]*((v2270*self.scalar_static_f64[953])/v2271)))}else{(if (v2261!=0.0){(self.scalar_static_f64[351]-(self.scalar_static_f64[876]*((v2262*self.scalar_static_f64[951])/v2263)))}else{v3})});
        let v6182=(if v2268{(-(self.scalar_static_f64[876]*((v2270*self.scalar_static_f64[954])/v2271)))}else{(if (v2261!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[876]*((v2262*self.scalar_static_f64[952])/v2263)))}else{v3})});
        let v6188=(self.scalar_static_f64[234]*f64::powf(v2278,self.scalar_static_f64[355]));
        let v6210=((v2291*v5300)+(v1974*(self.scalar_static_f64[927]*v3573)));
        let v6213=((v2291*v5301)+(v1974*(self.scalar_static_f64[927]*v3577)));
        let v6214=(v2291*v5302);
        let v6215=(v2291*v5303);
        let v6219=(v2293*v5300);
        let v6222=((v2293*v5301)+(v1974*(self.scalar_static_f64[927]*v3595)));
        let v6225=((v2293*v5302)+(v1974*(self.scalar_static_f64[927]*v3599)));
        let v6228=((v2293*v5303)+(v1974*(self.scalar_static_f64[927]*v3603)));
        let v6273=(if v2305{(-(self.scalar_static_f64[872]*((v2307*self.scalar_static_f64[988])/v2308)))}else{(if (v2298!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[872]*((v2299*self.scalar_static_f64[984])/v2300)))}else{v3})});
        let v6274=(if v2305{(-(self.scalar_static_f64[872]*((v2307*self.scalar_static_f64[989])/v2308)))}else{(if (v2298!=0.0){(self.scalar_static_f64[352]-(self.scalar_static_f64[872]*((v2299*self.scalar_static_f64[985])/v2300)))}else{v3})});
        let v6275=(if v2305{(-(self.scalar_static_f64[872]*((v2307*self.scalar_static_f64[990])/v2308)))}else{(if (v2298!=0.0){(self.scalar_static_f64[353]-(self.scalar_static_f64[872]*((v2299*self.scalar_static_f64[986])/v2300)))}else{v3})});
        let v6276=(if v2305{(-(self.scalar_static_f64[872]*((v2307*self.scalar_static_f64[991])/v2308)))}else{(if (v2298!=0.0){(self.scalar_static_f64[351]-(self.scalar_static_f64[872]*((v2299*self.scalar_static_f64[987])/v2300)))}else{v3})});
        let v6286=(self.scalar_static_f64[240]*f64::powf(v2314,self.scalar_static_f64[359]));
        let v6371=(if v2338{(-(self.scalar_static_f64[872]*((v2340*self.scalar_static_f64[989])/v2341)))}else{(if (v2331!=0.0){(self.scalar_static_f64[352]-(self.scalar_static_f64[872]*((v2332*self.scalar_static_f64[985])/v2333)))}else{v3})});
        let v6372=(if v2338{(-(self.scalar_static_f64[872]*((v2340*self.scalar_static_f64[995])/v2341)))}else{(if (v2331!=0.0){(self.scalar_static_f64[354]-(self.scalar_static_f64[872]*((v2332*self.scalar_static_f64[994])/v2333)))}else{v3})});
        let v6373=(if v2338{(-(self.scalar_static_f64[872]*((v2340*self.scalar_static_f64[990])/v2341)))}else{(if (v2331!=0.0){(self.scalar_static_f64[353]-(self.scalar_static_f64[872]*((v2332*self.scalar_static_f64[986])/v2333)))}else{v3})});
        let v6374=(if v2338{(-(self.scalar_static_f64[872]*((v2340*self.scalar_static_f64[991])/v2341)))}else{(if (v2331!=0.0){(self.scalar_static_f64[351]-(self.scalar_static_f64[872]*((v2332*self.scalar_static_f64[987])/v2333)))}else{v3})});
        let v6384=(self.scalar_static_f64[240]*f64::powf(v2347,self.scalar_static_f64[359]));
        let v6426=(self.scalar_static_f64[6]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*(self.scalar_static_f64[992]+(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6371/self.scalar_static_f64[538]))*v6384)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[352]-v6371))))))));
        let v6428=(self.scalar_static_f64[6]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*(self.scalar_static_f64[993]+(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6373/self.scalar_static_f64[538]))*v6384)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[353]-v6373))))))));
        let v6452=(if v2375{(-(self.scalar_static_f64[928]*((v2377*self.scalar_static_f64[999])/v2378)))}else{(if (v2368!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[928]*((v2369*self.scalar_static_f64[997])/v2370)))}else{v3})});
        let v6453=(if v2375{(-(self.scalar_static_f64[928]*((v2377*self.scalar_static_f64[1000])/v2378)))}else{(if (v2368!=0.0){(self.scalar_static_f64[351]-(self.scalar_static_f64[928]*((v2369*self.scalar_static_f64[998])/v2370)))}else{v3})});
        let v6460=(self.scalar_static_f64[326]*f64::powf(v2386,self.scalar_static_f64[391]));
        let v6491=(self.scalar_static_f64[934]*(if v2406{(v2407*self.scalar_static_f64[1001])}else{(if (v2403!=0.0){(v2404*self.scalar_static_f64[1001])}else{v4069})}));
        let v6492=(self.scalar_static_f64[934]*(if v2406{v3}else{(if (v2403!=0.0){v3}else{v4070})}));
        let v6493=(self.scalar_static_f64[934]*(if v2406{(v2407*self.scalar_static_f64[1002])}else{(if (v2403!=0.0){(v2404*self.scalar_static_f64[1002])}else{v4071})}));
        let v6494=(self.scalar_static_f64[934]*(if v2406{v3}else{(if (v2403!=0.0){v3}else{v4072})}));
        let v6495=(self.scalar_static_f64[934]*(if v2406{v3}else{(if (v2403!=0.0){v3}else{v4073})}));
        let v6564=(v34*v2451);
        let v6572=(v2452*v2452);
        let v6586=(if self.scalar_static_bool[64]{(((v2452*(self.scalar_static_f64[941]*v2741))-(v2448*((v455*(if v2441{(v2442*self.scalar_static_f64[1003])}else{(if v2437{(v2438*self.scalar_static_f64[1003])}else{v3})}))/v6564)))/v6572)}else{(if (self.scalar_static_f64[330]!=0.0){((self.scalar_static_f64[940]*((self.scalar_static_f64[926]*(((v1740*v4411)-(v1737*(v4411/v4419)))/v4427))+(self.scalar_static_f64[938]*(((v1744*v4415)-(v1736*(v4415/v4441)))/v4449))))/self.scalar_static_f64[833])}else{v3})});
        let v6587=(if self.scalar_static_bool[64]{(((v2452*(self.scalar_static_f64[941]*v2742))-(v2448*((v455*(if v2441{(v2442*self.scalar_static_f64[1004])}else{(if v2437{(v2438*self.scalar_static_f64[1004])}else{v3})}))/v6564)))/v6572)}else{(if (self.scalar_static_f64[330]!=0.0){((self.scalar_static_f64[940]*((self.scalar_static_f64[926]*(((v1740*v4412)-(v1737*(v4412/v4419)))/v4427))+(self.scalar_static_f64[938]*(((v1744*v4416)-(v1736*(v4416/v4441)))/v4449))))/self.scalar_static_f64[833])}else{v3})});
        let v6588=(if self.scalar_static_bool[64]{(((v2452*(self.scalar_static_f64[941]*v2743))-(v2448*((v455*(if v2441{(v2442*self.scalar_static_f64[1005])}else{(if v2437{(v2438*self.scalar_static_f64[1005])}else{v3})}))/v6564)))/v6572)}else{(if (self.scalar_static_f64[330]!=0.0){((self.scalar_static_f64[940]*((self.scalar_static_f64[926]*(((v1740*v4413)-(v1737*(v4413/v4419)))/v4427))+(self.scalar_static_f64[938]*(((v1744*v4417)-(v1736*(v4417/v4441)))/v4449))))/self.scalar_static_f64[833])}else{v3})});
        let v6589=(if self.scalar_static_bool[64]{(((v2452*(self.scalar_static_f64[941]*v2744))-(v2448*((v455*(if v2441{(v2442*self.scalar_static_f64[1006])}else{(if v2437{(v2438*self.scalar_static_f64[1006])}else{v3})}))/v6564)))/v6572)}else{(if (self.scalar_static_f64[330]!=0.0){((self.scalar_static_f64[940]*((self.scalar_static_f64[926]*(((v1740*v4414)-(v1737*(v4414/v4419)))/v4427))+(self.scalar_static_f64[938]*(((v1744*v4418)-(v1736*(v4418/v4441)))/v4449))))/self.scalar_static_f64[833])}else{v3})});
        let v6602=(if self.scalar_static_bool[68]{(self.scalar_static_f64[886]*v2766)}else{v3});
        let v6603=(if self.scalar_static_bool[68]{(self.scalar_static_f64[886]*v2767)}else{v3});
        let v6604=(if self.scalar_static_bool[68]{(self.scalar_static_f64[886]*v2768)}else{v3});
        let v6605=(if self.scalar_static_bool[68]{(self.scalar_static_f64[886]*v2769)}else{v3});
        let v6606=(v34*v2466);
        let v6614=(v2467*v2467);
        let v6636=(if self.scalar_static_bool[68]{(v455*(if v898{(v899*self.scalar_static_f64[948])}else{(if (v895!=0.0){(v896*self.scalar_static_f64[948])}else{v3})}))}else{v3});
        let v6637=(if self.scalar_static_bool[68]{(v455*(if v898{(v899*self.scalar_static_f64[950])}else{(if (v895!=0.0){(v896*self.scalar_static_f64[950])}else{v3})}))}else{v3});
        let v6638=(if self.scalar_static_bool[68]{(v455*(if v898{(v899*self.scalar_static_f64[949])}else{(if (v895!=0.0){(v896*self.scalar_static_f64[949])}else{v3})}))}else{v3});
        let v6639=(if self.scalar_static_bool[68]{(v455*(if v898{(v899*self.scalar_static_f64[945])}else{(if (v895!=0.0){(v896*self.scalar_static_f64[945])}else{v3})}))}else{v3});
        let v6640=(v34*v2473);
        let v6648=(v2474*v2474);
        let v6714=(v34*v2504);
        let v6722=(v2505*v2505);
        let v6741=(v1900*(if self.scalar_static_bool[69]{(((v2505*(self.scalar_static_f64[943]*v2766))-(v2501*((v455*(if v2494{(v2495*self.scalar_static_f64[948])}else{(if v2490{(v2491*self.scalar_static_f64[948])}else{v3})}))/v6714)))/v6722)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[942]*((self.scalar_static_f64[926]*(if self.scalar_static_bool[68]{(((v2467*v6602)-(v2464*(v6602/v6606)))/v6614)}else{v3}))+(self.scalar_static_f64[938]*(if self.scalar_static_bool[68]{(((v2474*v6636)-(v2471*(v6636/v6640)))/v6648)}else{v3}))))/self.scalar_static_f64[833])}else{v3})}));
        let v6751=(v1900*(if self.scalar_static_bool[69]{(((v2505*(self.scalar_static_f64[943]*v2768))-(v2501*((v455*(if v2494{(v2495*self.scalar_static_f64[949])}else{(if v2490{(v2491*self.scalar_static_f64[949])}else{v3})}))/v6714)))/v6722)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[942]*((self.scalar_static_f64[926]*(if self.scalar_static_bool[68]{(((v2467*v6604)-(v2464*(v6604/v6606)))/v6614)}else{v3}))+(self.scalar_static_f64[938]*(if self.scalar_static_bool[68]{(((v2474*v6638)-(v2471*(v6638/v6640)))/v6648)}else{v3}))))/self.scalar_static_f64[833])}else{v3})}));
        let v6771=(self.scalar_static_f64[336]*f64::powf(v1198,self.scalar_static_f64[396]));
        let v6781=(v2522*v2522);
        let v6789=(v2528*self.scalar_static_f64[1009]);
        let v6790=(v2528*self.scalar_static_f64[1010]);
        let v6794=(v2529*v2529);
        let v6820=(v1266*v1266);
        let v6857=(if (self.scalar_static_f64[335]!=0.0){(v6494/self.scalar_static_f64[935])}else{v3});
        let v6896=(self.scalar_static_f64[337]*v6494);
        let v6902=(if (self.scalar_static_f64[335]!=0.0){(v6210+(self.scalar_static_f64[337]*v6491))}else{v3});
        let v6903=(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[337]*v6492)}else{v3});
        let v6904=(if (self.scalar_static_f64[335]!=0.0){(v6213+(self.scalar_static_f64[337]*v6493))}else{v3});
        let v6905=(if (self.scalar_static_f64[335]!=0.0){(v6214+v6896)}else{v3});
        let v6906=(if (self.scalar_static_f64[335]!=0.0){(v6215+v6896)}else{v3});
        let v6907=(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[337]*v6495)}else{v3});
        let v6936=(if self.scalar_static_bool[71]{v6210}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6902)}else{v3})});
        let v6937=(if self.scalar_static_bool[71]{v3}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6903)}else{v3})});
        let v6938=(if self.scalar_static_bool[71]{v6213}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6904)}else{v3})});
        let v6939=(if self.scalar_static_bool[71]{v6214}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6905)}else{v3})});
        let v6940=(if self.scalar_static_bool[71]{v6215}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6906)}else{v3})});
        let v6941=(if self.scalar_static_bool[71]{v3}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[340]*v6907)}else{v3})});
        let v6942=(if self.scalar_static_bool[71]{v6219}else{(if (self.scalar_static_f64[335]!=0.0){(v6219+(self.scalar_static_f64[339]*v6902))}else{v3})});
        let v6943=(if self.scalar_static_bool[71]{v3}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[339]*v6903)}else{v3})});
        let v6944=(if self.scalar_static_bool[71]{v6222}else{(if (self.scalar_static_f64[335]!=0.0){(v6222+(self.scalar_static_f64[339]*v6904))}else{v3})});
        let v6945=(if self.scalar_static_bool[71]{v6225}else{(if (self.scalar_static_f64[335]!=0.0){(v6225+(self.scalar_static_f64[339]*v6905))}else{v3})});
        let v6946=(if self.scalar_static_bool[71]{v6228}else{(if (self.scalar_static_f64[335]!=0.0){(v6228+(self.scalar_static_f64[339]*v6906))}else{v3})});
        let v6947=(if self.scalar_static_bool[71]{v3}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[339]*v6907)}else{v3})});
        let v6951=(if self.scalar_static_bool[71]{v6494}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[338]*v6494)}else{v3})});
        let v6969=(v2571*v2571);
        let v7016=(if v2585{((v2586*v3704)+(v1317*(self.scalar_static_f64[829]*v5300)))}else{(if (v2581!=0.0){(((v2571*(v6936+v6942))-(v2582*((v3722-(v2570*v3704))/v3725)))/v6969)}else{v3})});
        let v7017=(if v2585{v3}else{(if (v2581!=0.0){((v6937+v6943)/v2571)}else{v3})});
        let v7018=(if v2585{((v2586*v3707)+(v1317*(self.scalar_static_f64[829]*v5301)))}else{(if (v2581!=0.0){(((v2571*(v6938+v6944))-(v2582*(((v1317*(v3714+v3718))-(v2570*v3707))/v3725)))/v6969)}else{v3})});
        let v7019=(if v2585{((v2586*v3710)+(v1317*(self.scalar_static_f64[829]*v5302)))}else{(if (v2581!=0.0){(((v2571*(v6939+v6945))-(v2582*(((v1317*v3715)-(v2570*v3710))/v3725)))/v6969)}else{v3})});
        let v7020=(if v2585{((v2586*v3713)+(v1317*(self.scalar_static_f64[829]*v5303)))}else{(if (v2581!=0.0){(((v2571*(v6940+v6946))-(v2582*(((v1317*v3716)-(v2570*v3713))/v3725)))/v6969)}else{v3})});
        let v7021=(if v2585{v3}else{(if (v2581!=0.0){((v6941+v6947)/v2571)}else{v3})});
        let v7046=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7016)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7016)}else{v3})})});
        let v7047=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7017)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7017)}else{v3})})});
        let v7048=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7018)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7018)}else{v3})})});
        let v7049=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7019)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7019)}else{v3})})});
        let v7050=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7020)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7020)}else{v3})})});
        let v7051=(if self.scalar_static_bool[79]{v3}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[346]*v7021)}else{(if (self.scalar_static_f64[344]!=0.0){(self.scalar_static_f64[339]*v7021)}else{v3})})});
        let v7257=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v6491}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[338]*v6491)}else{v3})})+((self.scalar_static_f64[923]*v3436)+v6936)));
        let v7258=(self.scalar_static_f64[0]*(v6937+(if self.scalar_static_bool[71]{v6492}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[338]*v6492)}else{v3})})));
        let v7259=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v6493}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[338]*v6493)}else{v3})})+((self.scalar_static_f64[923]*v3437)+v6938)));
        let v7260=(self.scalar_static_f64[0]*(v6939+v6951));
        let v7261=(self.scalar_static_f64[0]*(v6940+v6951));
        let v7262=(self.scalar_static_f64[0]*(v6941+(if self.scalar_static_bool[71]{v6495}else{(if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[338]*v6495)}else{v3})})));
        let v7276=(self.scalar_static_f64[0]*(self.scalar_static_f64[924]*((self.scalar_static_f64[877]*(-((-(self.scalar_static_f64[579]*v6181))*v6188)))+(v161*(self.scalar_static_f64[351]-v6181)))));
        let v7277=(self.scalar_static_f64[0]*(self.scalar_static_f64[924]*((self.scalar_static_f64[877]*(-((-(self.scalar_static_f64[579]*v6182))*v6188)))+(v161*(self.scalar_static_f64[0]-v6182)))));
        let v7282=(self.scalar_static_f64[0]*v6942);
        let v7283=(self.scalar_static_f64[0]*v6943);
        let v7284=(self.scalar_static_f64[0]*(((v2418*(self.scalar_static_f64[939]*v3374))+(v2417*v3334))+((self.scalar_static_f64[925]*v3562)+v6944)));
        let v7285=(self.scalar_static_f64[0]*(((v2418*(self.scalar_static_f64[939]*v3375))+(v2417*v3335))+((self.scalar_static_f64[925]*v3563)+v6945)));
        let v7286=(self.scalar_static_f64[0]*(((v2418*(self.scalar_static_f64[939]*v3376))+(v2417*v3330))+((self.scalar_static_f64[925]*v3559)+v6946)));
        let v7287=(self.scalar_static_f64[0]*v6947);
        let v7300=(self.scalar_static_f64[0]*(self.scalar_static_f64[588]*((self.scalar_static_f64[930]*(-((-(v6452/self.scalar_static_f64[578]))*v6460)))+(v34*(self.scalar_static_f64[0]-v6452)))));
        let v7301=(self.scalar_static_f64[0]*(self.scalar_static_f64[588]*((self.scalar_static_f64[930]*(-((-(v6453/self.scalar_static_f64[578]))*v6460)))+(v34*(self.scalar_static_f64[351]-v6453)))));
        let v7306=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){(v2547*((if (self.scalar_static_f64[335]!=0.0){(v6491/self.scalar_static_f64[935])}else{v3})+((if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[923]*(if (self.scalar_static_f64[335]!=0.0){((v2531*(if (self.scalar_static_f64[335]!=0.0){(v3421*v6771)}else{v3}))+(v2516*(if v2526{(((v2529*v6789)-(v2528*v6789))/v6794)}else{(if v2520{((-(v2521*self.scalar_static_f64[1007]))/v6781)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[335]!=0.0){((v2542*(if (self.scalar_static_f64[335]!=0.0){((v2539*((self.scalar_static_f64[412]*v3564)/self.scalar_static_f64[642]))+(v2538*((-(v443*v3567))/v6820)))}else{v3}))+(v2541*(self.scalar_static_f64[927]*v5300)))}else{v3}))))}else{v3}));
        let v7307=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){((v2549*self.scalar_static_f64[397])+(v2547*(if (self.scalar_static_f64[335]!=0.0){(v6492/self.scalar_static_f64[935])}else{v3})))}else{v3}));
        let v7308=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){((v2549*self.scalar_static_f64[398])+(v2547*((if (self.scalar_static_f64[335]!=0.0){(v6493/self.scalar_static_f64[935])}else{v3})+((if (self.scalar_static_f64[335]!=0.0){(self.scalar_static_f64[923]*(if (self.scalar_static_f64[335]!=0.0){((v2531*(if (self.scalar_static_f64[335]!=0.0){(v3422*v6771)}else{v3}))+(v2516*(if v2526{(((v2529*v6790)-(v2528*v6790))/v6794)}else{(if v2520{((-(v2521*self.scalar_static_f64[1008]))/v6781)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[335]!=0.0){((v2542*(if (self.scalar_static_f64[335]!=0.0){((v2539*((self.scalar_static_f64[412]*v3565)/self.scalar_static_f64[642]))+(v2538*((-(v443*v3568))/v6820)))}else{v3}))+(v2541*(self.scalar_static_f64[927]*v5301)))}else{v3})))))}else{v3}));
        let v7309=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){(v2547*((if (self.scalar_static_f64[335]!=0.0){(v2541*(self.scalar_static_f64[927]*v5302))}else{v3})+v6857))}else{v3}));
        let v7310=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){(v2547*((if (self.scalar_static_f64[335]!=0.0){(v2541*(self.scalar_static_f64[927]*v5303))}else{v3})+v6857))}else{v3}));
        let v7311=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[335]!=0.0){(v2547*(if (self.scalar_static_f64[335]!=0.0){(v6495/self.scalar_static_f64[935])}else{v3}))}else{v3}));
        let v7366=(self.scalar_static_f64[0]*(v6426+(if (self.scalar_static_f64[332]!=0.0){((v2507*v4952)+v6741)}else{v3})));
        let v7367=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6372/self.scalar_static_f64[538]))*v6384)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[354]-v6372))))+self.scalar_static_f64[996]))))+(if (self.scalar_static_f64[332]!=0.0){((v2507*v4953)+(v1900*(if self.scalar_static_bool[69]{(((v2505*(self.scalar_static_f64[943]*v2767))-(v2501*((v455*(if v2494{(v2495*self.scalar_static_f64[950])}else{(if v2490{(v2491*self.scalar_static_f64[950])}else{v3})}))/v6714)))/v6722)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[942]*((self.scalar_static_f64[926]*(if self.scalar_static_bool[68]{(((v2467*v6603)-(v2464*(v6603/v6606)))/v6614)}else{v3}))+(self.scalar_static_f64[938]*(if self.scalar_static_bool[68]{(((v2474*v6637)-(v2471*(v6637/v6640)))/v6648)}else{v3}))))/self.scalar_static_f64[833])}else{v3})})))}else{v3})));
        let v7368=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[332]!=0.0){(v2507*v4954)}else{v3}));
        let v7369=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[332]!=0.0){(v2507*v4955)}else{v3}));
        let v7370=(self.scalar_static_f64[0]*(v6426+(if (self.scalar_static_f64[332]!=0.0){(v6741+(v2507*v4956))}else{v3})));
        let v7371=(self.scalar_static_f64[0]*(v6428+(if (self.scalar_static_f64[332]!=0.0){((v2507*v4957)+v6751)}else{v3})));
        let v7372=(self.scalar_static_f64[0]*(v6428+(if (self.scalar_static_f64[332]!=0.0){(v6751+(v2507*v4958))}else{v3})));
        let v7373=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*(self.scalar_static_f64[956]+(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6374/self.scalar_static_f64[538]))*v6384)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[351]-v6374))))))))+(if (self.scalar_static_f64[332]!=0.0){((v2507*v4959)+(v1900*(if self.scalar_static_bool[69]{(((v2505*(self.scalar_static_f64[943]*v2769))-(v2501*((v455*(if v2494{(v2495*self.scalar_static_f64[945])}else{(if v2490{(v2491*self.scalar_static_f64[945])}else{v3})}))/v6714)))/v6722)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[942]*((self.scalar_static_f64[926]*(if self.scalar_static_bool[68]{(((v2467*v6605)-(v2464*(v6605/v6606)))/v6614)}else{v3}))+(self.scalar_static_f64[938]*(if self.scalar_static_bool[68]{(((v2474*v6639)-(v2471*(v6639/v6640)))/v6648)}else{v3}))))/self.scalar_static_f64[833])}else{v3})})))}else{v3})));
        let v7374=(self.scalar_static_f64[0]*(v6428+(if (self.scalar_static_f64[332]!=0.0){(v6751+(v2507*v4960))}else{v3})));
        let v7416=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*(self.scalar_static_f64[955]+(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6273/self.scalar_static_f64[538]))*v6286)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[0]-v6273))))))))+(if (self.scalar_static_f64[332]!=0.0){(self.scalar_static_f64[7]*v6586)}else{v6586})));
        let v7417=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6274/self.scalar_static_f64[538]))*v6286)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[352]-v6274))))+self.scalar_static_f64[992]))))+(if (self.scalar_static_f64[332]!=0.0){(self.scalar_static_f64[7]*v6587)}else{v6587})));
        let v7418=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*((self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6275/self.scalar_static_f64[538]))*v6286)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[353]-v6275))))+self.scalar_static_f64[993]))))+(if (self.scalar_static_f64[332]!=0.0){(self.scalar_static_f64[7]*v6588)}else{v6588})));
        let v7419=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[322]*(self.scalar_static_f64[594]*(self.scalar_static_f64[956]+(self.scalar_static_f64[879]*((self.scalar_static_f64[884]*(-((-(v6276/self.scalar_static_f64[538]))*v6286)))+(self.scalar_static_f64[880]*(self.scalar_static_f64[351]-v6276))))))))+(if (self.scalar_static_f64[332]!=0.0){(self.scalar_static_f64[7]*v6589)}else{v6589})));

        CommonStampValues {
            v1, v3, v33, v34, v49, v161, v439, v443, 
            v455, v481, v757, v761, v763, v768, v771, v774, 
            v779, v787, v790, v793, v797, v813, v836, v837, 
            v839, v842, v843, v859, v861, v864, v865, v881, 
            v883, v886, v887, v960, v1082, v1142, v1167, v1170, 
            v1173, v1200, v1280, v1316, v1317, v1322, v1323, v1342, 
            v1344, v1347, v1348, v1357, v1389, v1391, v1393, v1398, 
            v1399, v1406, v1407, v1409, v1414, v1416, v1468, v1470, 
            v1472, v1477, v1478, v1505, v1518, v1531, v1544, v1551, 
            v1552, v1555, v1557, v1562, v1563, v1569, v1573, v1576, 
            v1584, v1585, v1586, v1588, v1590, v1594, v1595, v1597, 
            v1600, v1602, v1603, v1608, v1609, v1647, v1649, v1651, 
            v1652, v1655, v1657, v1662, v1663, v1668, v1671, v1673, 
            v1681, v1682, v1683, v1685, v1690, v1691, v1693, v1695, 
            v1697, v1698, v1703, v1704, v1835, v1859, v1877, v1900, 
            v1974, v1986, v1999, v2000, v2001, v2004, v2005, v2009, 
            v2010, v2012, v2016, v2018, v2023, v2024, v2039, v2146, 
            v2147, v2149, v2151, v2153, v2155, v2156, v2158, v2166, 
            v2169, v2170, v2171, v2177, v2179, v2180, v2184, v2186, 
            v2189, v2191, v2196, v2197, v2571, v2603, v2654, v2657, 
            v2660, v2663, v2666, v2670, v2674, v2682, v2688, v2699, 
            v2715, v2716, v2741, v2742, v2743, v2744, v2894, v2895, 
            v2896, v3183, v3184, v3185, v3331, v3332, v3333, v3374, 
            v3375, v3376, v3383, v3384, v3385, v3392, v3393, v3394, 
            v3426, v3427, v3606, v3607, v3608, v3698, v3699, v3700, 
            v3701, v3704, v3707, v3710, v3713, v3714, v3715, v3716, 
            v3718, v3722, v3725, v3759, v3760, v3819, v3820, v3953, 
            v3954, v3955, v4010, v4011, v4012, v4025, v4026, v4027, 
            v4048, v4049, v4050, v4051, v4052, v4069, v4070, v4071, 
            v4072, v4073, v4705, v4706, v4707, v4708, v4791, v4792, 
            v4793, v4794, v4795, v4796, v4809, v4810, v4811, v4812, 
            v4813, v4814, v4815, v4816, v4952, v4953, v4954, v4955, 
            v4956, v4957, v4958, v4959, v4960, v5300, v5301, v5302, 
            v5303, v7046, v7047, v7048, v7049, v7050, v7051, v7257, 
            v7258, v7259, v7260, v7261, v7262, v7276, v7277, v7282, 
            v7283, v7284, v7285, v7286, v7287, v7300, v7301, v7306, 
            v7307, v7308, v7309, v7310, v7311, v7366, v7367, v7368, 
            v7369, v7370, v7371, v7372, v7373, v7374, v7416, v7417, 
            v7418, v7419, 
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
        let v840=(common.v837).exp();
        let v862=(common.v859).exp();
        let v869=(if common.v864{(common.v865*(common.v1+(common.v859-self.scalar_static_f64[214])))}else{(if (common.v861!=0.0){v862}else{common.v3})});
        let v884=(common.v881).exp();
        let v891=(if common.v886{(common.v887*(common.v1+(common.v881-self.scalar_static_f64[214])))}else{(if (common.v883!=0.0){v884}else{common.v3})});
        let v1345=(common.v1342).exp();
        let v1352=(if common.v1347{(common.v1348*(common.v1+(common.v1342-self.scalar_static_f64[214])))}else{(if (common.v1344!=0.0){v1345}else{common.v3})});
        let v1359=(if (common.v763<self.scalar_static_f64[244]){common.v1}else{common.v3});
        let v1360=(common.v1357).exp();
        let v1361=(common.v1+v1360);
        let v1366=(!(v1359!=0.0));
        let v1368=((-common.v1357)).exp();
        let v1369=(common.v1+v1368);
        let v1373=(if v1366{(self.scalar_static_f64[244]-(common.v33*(v1369).ln()))}else{(if (v1359!=0.0){(common.v763-(common.v33*(v1361).ln()))}else{common.v3})});
        let v1375=(v1373*self.scalar_static_f64[245]);
        let v1376=(self.scalar_static_f64[244]-v1373);
        let v1377=f64::powf(v1376,common.v34);
        let v1394=((self.scalar_static_f64[154]!=0.0)&&(common.v1393!=0.0));
        let v1395=(common.v1391).exp();
        let v1403=(if common.v1398{(common.v1399*(common.v1+(common.v1391-self.scalar_static_f64[214])))}else{(if v1394{v1395}else{common.v1342})});
        let v1410=((self.scalar_static_f64[154]!=0.0)&&(common.v1409!=0.0));
        let v1411=(common.v1406).exp();
        let v1420=(if common.v1414{(common.v1416*(common.v1+(common.v1406-common.v1407)))}else{(if v1410{v1411}else{v1352})});
        let v1421=(common.v1389-common.v1);
        let v1422=(self.scalar_static_f64[715]*v1421);
        let v1424=(v1421*self.scalar_static_f64[892]);
        let v1427=((common.v1+(common.v455*v1403))).sqrt();
        let v1428=(common.v1+v1427);
        let v1429=(v1424/v1428);
        let v1430=(common.v1+common.v1280);
        let v1434=(self.scalar_static_f64[730]*(common.v1142-common.v1));
        let v1435=(v1420*v1434);
        let v1436=(common.v1+v1420);
        let v1452=(self.scalar_static_f64[246]*((common.v1142+common.v1389)-common.v34));
        let v1473=((self.scalar_static_f64[154]!=0.0)&&(common.v1472!=0.0));
        let v1474=(common.v1470).exp();
        let v1483=(common.v1468-common.v1);
        let v1484=(self.scalar_static_f64[721]*v1483);
        let v1486=(v1483*self.scalar_static_f64[893]);
        let v1489=((common.v1+(common.v455*(if common.v1477{(common.v1478*(common.v1+(common.v1470-self.scalar_static_f64[214])))}else{(if v1473{v1474}else{v1403})})))).sqrt();
        let v1490=(common.v1+v1489);
        let v1533=(self.scalar_static_f64[707]*(common.v1531-common.v1));
        let v1558=((common.v1551!=0.0)&&(common.v1557!=0.0));
        let v1559=(common.v1555).exp();
        let v1567=(if common.v1562{(common.v1563*(common.v1+(common.v1555-self.scalar_static_f64[214])))}else{(if v1558{v1559}else{common.v3})});
        let v1604=((common.v1602!=0.0)&&common.v1603);
        let v1605=(common.v1597).exp();
        let v1614=(-common.v763);
        let v1615=(common.v1-(if common.v1608{(common.v1609*(common.v1+(common.v1597-self.scalar_static_f64[214])))}else{(if v1604{v1605}else{common.v3})}));
        let v1617=(common.v1+(v1615/common.v1597));
        let v1621=((common.v1551!=0.0)&&(!(common.v1600!=0.0)));
        let v1622=(common.v443*common.v763);
        let v1623=(common.v1597*v1622);
        let v1624=0.3333333333333333;
        let v1625=(common.v1597*v1624);
        let v1626=0.25;
        let v1628=(common.v1+(common.v1597*v1626));
        let v1630=(common.v1+(v1625*v1628));
        let v1634=((if v1621{(v1623*v1630)}else{(if common.v1603{(v1614*v1617)}else{common.v3})})*self.scalar_static_f64[894]);
        let v1635=(common.v1200*v1634);
        let v1640=(!(common.v1551!=0.0));
        let v1658=((common.v1647!=0.0)&&(common.v1657!=0.0));
        let v1659=(common.v1655).exp();
        let v1667=(if common.v1662{(common.v1663*(common.v1+(common.v1655-self.scalar_static_f64[214])))}else{(if v1658{v1659}else{common.v3})});
        let v1699=((common.v1697!=0.0)&&common.v1698);
        let v1700=(common.v1693).exp();
        let v1709=(-common.v757);
        let v1710=(common.v1-(if common.v1703{(common.v1704*(common.v1+(common.v1693-self.scalar_static_f64[214])))}else{(if v1699{v1700}else{common.v3})}));
        let v1712=(common.v1+(v1710/common.v1693));
        let v1716=((common.v1647!=0.0)&&(!(common.v1695!=0.0)));
        let v1717=(common.v443*common.v757);
        let v1718=(common.v1693*v1717);
        let v1719=(v1624*common.v1693);
        let v1721=(common.v1+(v1626*common.v1693));
        let v1723=(common.v1+(v1719*v1721));
        let v1727=((if v1716{(v1718*v1723)}else{(if common.v1698{(v1709*v1712)}else{common.v3})})*self.scalar_static_f64[895]);
        let v1728=(common.v1651*v1727);
        let v1733=(!(common.v1647!=0.0));
        let v1734=(if v1733{common.v3}else{(if (common.v1647!=0.0){(self.scalar_static_f64[54]*(self.scalar_static_f64[580]*(v1667*v1728)))}else{common.v3})});
        let v1747=(common.v836-common.v1);
        let v1748=(self.scalar_static_f64[896]*v1747);
        let v1753=((common.v1+(common.v836*self.scalar_static_f64[898]))).sqrt();
        let v1754=(common.v1+v1753);
        let v1755=(v1748/v1754);
        let v1763=(self.scalar_static_f64[899]*(common.v813-v869));
        let v1771=((common.v1+(self.scalar_static_f64[901]*(common.v813+(v869*self.scalar_static_f64[261]))))).sqrt();
        let v1772=(common.v1+v1771);
        let v1779=(self.scalar_static_f64[902]*(common.v836-v891));
        let v1784=((common.v1+(self.scalar_static_f64[901]*(common.v836+(v891*self.scalar_static_f64[261]))))).sqrt();
        let v1785=(common.v1+v1784);
        let v1790=(self.scalar_static_f64[899]*(common.v813-common.v1));
        let v1793=((common.v1+(common.v813*self.scalar_static_f64[901]))).sqrt();
        let v1794=(common.v1+v1793);
        let v1797=(v1747*self.scalar_static_f64[902]);
        let v1800=((common.v1+(common.v836*self.scalar_static_f64[901]))).sqrt();
        let v1801=(common.v1+v1800);
        let v1803=(if self.scalar_static_bool[41]{(v1797/v1801)}else{(if (self.scalar_static_f64[258]!=0.0){(v1779/v1785)}else{common.v3})});
        let v1806=(self.scalar_static_f64[903]*(v869-common.v1));
        let v1812=((common.v1+(v869*self.scalar_static_f64[905]))).sqrt();
        let v1813=(common.v1+v1812);
        let v1823=(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v1755)}else{v1755});
        let v1902=(if (self.scalar_static_f64[266]!=0.0){(common.v1835*common.v1900)}else{common.v3});
        let v1909=(if (self.scalar_static_f64[274]!=0.0){(common.v757+common.v768)}else{common.v3});
        let v1911=(-v1909);
        let v1915=(if (v1911<common.v3){common.v1}else{common.v3});
        let v1916=((self.scalar_static_f64[274]!=0.0)&&(v1915!=0.0));
        let v1919=((self.scalar_static_f64[275]+(if (self.scalar_static_f64[274]!=0.0){(v1909*v1909)}else{common.v1877}))).sqrt();
        let v1920=(v1919-v1911);
        let v1924=((self.scalar_static_f64[274]!=0.0)&&(!(v1915!=0.0)));
        let v1927=(if v1924{(common.v443*(v1911+v1919))}else{(if v1916{(self.scalar_static_f64[276]/v1920)}else{common.v3})});
        let v1944=(if (v1927<self.scalar_static_f64[284]){common.v1}else{common.v3});
        let v1945=((self.scalar_static_f64[274]!=0.0)&&(v1944!=0.0));
        let v1946=(v1927/self.scalar_static_f64[282]);
        let v1948=(common.v1-f64::powf(v1946,self.scalar_static_f64[277]));
        let v1952=((self.scalar_static_f64[274]!=0.0)&&(!(v1944!=0.0)));
        let v1958=(if self.scalar_static_bool[52]{common.v1}else{(if v1952{(self.scalar_static_f64[281]+(self.scalar_static_f64[291]*(v1927-self.scalar_static_f64[284])))}else{(if v1945{(common.v1/v1948)}else{common.v3})})});
        let v1975=(common.v1316*common.v1974);
        let v1976=(self.scalar_static_f64[603]/v1975);
        let v1978=(if (v1976<self.scalar_static_f64[16]){common.v1}else{common.v3});
        let v1980=(common.v161*(if (v1978!=0.0){self.scalar_static_f64[16]}else{v1976}));
        let v1983=(common.v768+(self.scalar_static_f64[865]*((if common.v842{(common.v843*(common.v1+(common.v837-self.scalar_static_f64[214])))}else{(if (common.v839!=0.0){v840}else{common.v3})})-common.v1)));
        let v2019=(common.v1999&&(common.v2018!=0.0));
        let v2020=(common.v2016).exp();
        let v2028=(if common.v2023{(common.v2024*(common.v1+(common.v2016-self.scalar_static_f64[214])))}else{(if v2019{v2020}else{common.v3})});
        let v2031=(common.v2012*self.scalar_static_f64[920]);
        let v2041=(((if (common.v757<self.scalar_static_f64[500]){common.v1}else{common.v3})!=0.0)&&((self.scalar_static_f64[298]!=0.0)&&common.v2039));
        let v2047=(if v2041{self.scalar_static_f64[303]}else{common.v3});
        let v2048=(self.scalar_static_f64[500]-common.v757);
        let v2050=(if v2041{(v2048/common.v1173)}else{common.v1082});
        let v2053=(((common.v34*v2050)/v2047)).sqrt();
        let v2054=(if v2041{v2053}else{common.v3});
        let v2058=(v2041&&(self.scalar_static_f64[305]!=0.0));
        let v2061=(v2041&&self.scalar_static_bool[57]);
        let v2064=(if v2061{(common.v1-(common.v443*common.v1167))}else{common.v3});
        let v2065=(self.scalar_static_f64[301]*v2064);
        let v2067=(if v2061{(v2064*v2065)}else{(if v2058{self.scalar_static_f64[301]}else{common.v3})});
        let v2068=(v2054*v2067);
        let v2072=(((v2054*v2054)+(v2067*v2067))).sqrt();
        let v2074=(if v2041{(v2068/v2072)}else{common.v3});
        let v2076=(if v2041{(v2048/v2074)}else{common.v3});
        let v2077=(common.v443*v2074);
        let v2078=(v2047*v2077);
        let v2081=(if v2041{(v2076+(common.v1173*v2078))}else{common.v3});
        let v2094=(self.scalar_static_f64[217]*(if v2061{(common.v1+(self.scalar_static_f64[307]*(common.v1+(common.v34*common.v1167))))}else{common.v3}));
        let v2096=((if v2061{self.scalar_static_f64[310]}else{common.v3})-(common.v1323/v2094));
        let v2099=(if v2061{(v2076-(v2078*v2096))}else{common.v3});
        let v2100=(v2099-v2081);
        let v2102=(common.v49*v2076);
        let v2103=(v2076*v2102);
        let v2109=((if v2061{((v2100*v2100)+((common.v1170*v2103)/self.scalar_static_f64[217]))}else{v2050})).sqrt();
        let v2112=(if v2061{(common.v443*((v2081+v2099)+v2109))}else{(if v2058{v2081}else{common.v3})});
        let v2113=(v2112-v2076);
        let v2115=(if v2041{(v2113/v2112)}else{common.v3});
        let v2119=(if ((v2115).abs()>1e-7){common.v1}else{common.v3});
        let v2120=(v2041&&(v2119!=0.0));
        let v2122=(if v2120{(v2077/v2115)}else{common.v3});
        let v2124=(v2112*self.scalar_static_f64[921]);
        let v2125=(v2122*v2124);
        let v2127=(self.scalar_static_f64[922]/v2112);
        let v2128=(v2127).exp();
        let v2130=(common.v1+(v2067/v2122));
        let v2132=((v2127*v2130)).exp();
        let v2133=(v2128-v2132);
        let v2137=(v2041&&(!(v2119!=0.0)));
        let v2138=(self.scalar_static_f64[4]*v2067);
        let v2192=(common.v2146&&(common.v2191!=0.0));
        let v2193=(common.v2189).exp();
        let v2201=(if common.v2196{(common.v2197*(common.v1+(common.v2189-self.scalar_static_f64[214])))}else{(if v2192{v2193}else{v2028})});
        let v2202=(common.v2010*self.scalar_static_f64[920]);
        let v2204=(if common.v2146{(v2201*v2202)}else{(if v2137{(v2128*v2138)}else{(if v2120{(v2125*v2133)}else{(if common.v1999{(v2028*v2031)}else{common.v3})})})});
        let v2210=((common.v1986!=0.0)&&((if (v2204>common.v3){common.v1}else{common.v3})!=0.0));
        let v2211=((self.scalar_static_f64[318]!=0.0)&&v2210);
        let v2212=(self.scalar_static_f64[608]+v1980);
        let v2213=(common.v1323*v2212);
        let v2220=(if v2211{(((self.scalar_static_f64[411]/v2213)+(self.scalar_static_f64[715]*(common.v1317/self.scalar_static_f64[687])))+(self.scalar_static_f64[600]/v2212))}else{common.v3});
        let v2221=((self.scalar_static_f64[311]!=0.0)&&v2211);
        let v2224=(if v2221{((v2204-v2220)/common.v439)}else{common.v2166});
        let v2226=(if (v2204<v2220){common.v1}else{common.v3});
        let v2227=(v2221&&(v2226!=0.0));
        let v2228=(v2224).exp();
        let v2229=(common.v1+v2228);
        let v2235=(v2221&&(!(v2226!=0.0)));
        let v2237=((-v2224)).exp();
        let v2238=(common.v1+v2237);
        let v2242=(if v2235{(v2220-(common.v439*(v2238).ln()))}else{(if v2227{(v2204-(common.v439*(v2229).ln()))}else{v2204})});
        let v2243=(common.v1323*v2242);
        let v2246=(v2211&&self.scalar_static_bool[61]);
        let v2247=(v2220*v2243);
        let v2248=(v2220+v2242);
        let v2252=(v2210&&self.scalar_static_bool[62]);
        let v2253=(if v2252{v2243}else{(if v2246{(v2247/v2248)}else{(if v2221{v2243}else{common.v3})})});
        let v2579=(if self.scalar_static_bool[73]{common.v3}else{(if (self.scalar_static_f64[342]!=0.0){((v2253/common.v2571)).abs()}else{common.v3})});
        let v2633=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v1958))));
        let v2655=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2654);
        let v2658=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2657);
        let v2661=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2660);
        let v2664=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2663);
        let v2667=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2666);
        let v2671=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2670);
        let v2675=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2674);
        let v2683=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2682);
        let v2689=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2688);
        let v2700=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2699);
        let v2776=(if common.v864{(common.v865*self.scalar_static_f64[944])}else{(if (common.v861!=0.0){(v862*self.scalar_static_f64[944])}else{common.v3})});
        let v2777=(if common.v864{(common.v865*self.scalar_static_f64[945])}else{(if (common.v861!=0.0){(v862*self.scalar_static_f64[945])}else{common.v3})});
        let v2799=(if common.v886{(common.v887*self.scalar_static_f64[944])}else{(if (common.v883!=0.0){(v884*self.scalar_static_f64[944])}else{common.v3})});
        let v2800=(if common.v886{(common.v887*self.scalar_static_f64[949])}else{(if (common.v883!=0.0){(v884*self.scalar_static_f64[949])}else{common.v3})});
        let v2801=(if common.v886{(common.v887*self.scalar_static_f64[945])}else{(if (common.v883!=0.0){(v884*self.scalar_static_f64[945])}else{common.v3})});
        let v3726=((common.v3722-(common.v1322*common.v3704))/common.v3725);
        let v3730=(((common.v1317*(common.v3718-common.v3714))-(common.v1322*common.v3707))/common.v3725);
        let v3734=(((common.v1317*(-common.v3715))-(common.v1322*common.v3710))/common.v3725);
        let v3738=(((common.v1317*(-common.v3716))-(common.v1322*common.v3713))/common.v3725);
        let v3761=(common.v3759/self.scalar_static_f64[243]);
        let v3762=(common.v3760/self.scalar_static_f64[243]);
        let v3769=(if common.v1347{(common.v1348*v3761)}else{(if (common.v1344!=0.0){(v1345*v3761)}else{common.v3})});
        let v3770=(if common.v1347{(common.v1348*v3762)}else{(if (common.v1344!=0.0){(v1345*v3762)}else{common.v3})});
        let v3795=(if v1366{(-(common.v33*((v1368*self.scalar_static_f64[366])/v1369)))}else{(if (v1359!=0.0){(self.scalar_static_f64[351]-(common.v33*((v1360*self.scalar_static_f64[364])/v1361)))}else{common.v3})});
        let v3796=(if v1366{(-(common.v33*((v1368*self.scalar_static_f64[367])/v1369)))}else{(if (v1359!=0.0){(self.scalar_static_f64[0]-(common.v33*((v1360*self.scalar_static_f64[365])/v1361)))}else{common.v3})});
        let v3802=(common.v34*f64::powf(v1376,common.v1));
        let v3827=(if common.v1398{(common.v1399*self.scalar_static_f64[945])}else{(if v1394{(v1395*self.scalar_static_f64[945])}else{v3761})});
        let v3828=(if common.v1398{(common.v1399*self.scalar_static_f64[944])}else{(if v1394{(v1395*self.scalar_static_f64[944])}else{v3762})});
        let v3829=(v3726/self.scalar_static_f64[687]);
        let v3830=(v3730/self.scalar_static_f64[687]);
        let v3831=(v3734/self.scalar_static_f64[687]);
        let v3832=(v3738/self.scalar_static_f64[687]);
        let v3845=(if common.v1414{(common.v1416*v3829)}else{(if v1410{(v1411*v3829)}else{v3769})});
        let v3846=(if common.v1414{(common.v1416*v3830)}else{(if v1410{(v1411*v3830)}else{v3770})});
        let v3847=(if common.v1414{(common.v1416*v3831)}else{(if v1410{(v1411*v3831)}else{common.v3})});
        let v3848=(if common.v1414{(common.v1416*v3832)}else{(if v1410{(v1411*v3832)}else{common.v3})});
        let v3849=(self.scalar_static_f64[715]*common.v3819);
        let v3850=(self.scalar_static_f64[715]*common.v3820);
        let v3855=(common.v34*v1427);
        let v3861=(v1428*v1428);
        let v3891=(v1436*v1436);
        let v3966=(self.scalar_static_f64[721]*common.v3953);
        let v3967=(self.scalar_static_f64[721]*common.v3954);
        let v3968=(self.scalar_static_f64[721]*common.v3955);
        let v3975=(common.v34*v1489);
        let v3982=(v1490*v1490);
        let v4083=(common.v1552*common.v1552);
        let v4090=(self.scalar_static_f64[769]*(-((-(self.scalar_static_f64[21]*(common.v34*common.v3426)))/v4083)));
        let v4091=(self.scalar_static_f64[769]*(-((-(self.scalar_static_f64[21]*(common.v34*common.v3427)))/v4083)));
        let v4102=(if (common.v1551!=0.0){self.scalar_static_f64[972]}else{common.v3});
        let v4103=(if (common.v1551!=0.0){self.scalar_static_f64[973]}else{common.v3});
        let v4104=(common.v1569*v4102);
        let v4106=(common.v1569*v4103);
        let v4108=(common.v34*common.v1573);
        let v4113=(self.scalar_static_f64[249]*f64::powf(common.v1573,self.scalar_static_f64[368]));
        let v4159=(common.v1595*common.v1595);
        let v4165=(if (common.v1551!=0.0){(((common.v1595*self.scalar_static_f64[974])-(common.v1594*(self.scalar_static_f64[436]*(if (common.v1551!=0.0){(common.v1590*((common.v1588*(((v4104+v4104)/v4108)*v4113))+(common.v1576*((self.scalar_static_f64[19]*(-(self.scalar_static_f64[252]*(common.v161*v4102))))-((common.v1586*((common.v1584*v4102)+(common.v1569*(common.v481*v4102))))+(common.v1585*v4102))))))}else{common.v3}))))/v4159)}else{v4102});
        let v4166=(if (common.v1551!=0.0){(((common.v1595*self.scalar_static_f64[975])-(common.v1594*(self.scalar_static_f64[436]*(if (common.v1551!=0.0){(common.v1590*((common.v1588*(((v4106+v4106)/v4108)*v4113))+(common.v1576*((self.scalar_static_f64[19]*(-(self.scalar_static_f64[252]*(common.v161*v4103))))-((common.v1586*((common.v1584*v4103)+(common.v1569*(common.v481*v4103))))+(common.v1585*v4103))))))}else{common.v3}))))/v4159)}else{v4103});
        let v4180=(common.v1597*common.v1597);
        let v4247=(self.scalar_static_f64[240]*f64::powf(common.v1649,self.scalar_static_f64[359]));
        let v4250=(if (common.v1647!=0.0){(self.scalar_static_f64[978]*v4247)}else{common.v3});
        let v4251=(if (common.v1647!=0.0){(self.scalar_static_f64[979]*v4247)}else{common.v3});
        let v4256=(common.v1652*common.v1652);
        let v4263=(self.scalar_static_f64[789]*(-((-(self.scalar_static_f64[53]*(common.v34*v4250)))/v4256)));
        let v4264=(self.scalar_static_f64[789]*(-((-(self.scalar_static_f64[53]*(common.v34*v4251)))/v4256)));
        let v4273=(if (common.v1647!=0.0){self.scalar_static_f64[976]}else{common.v3});
        let v4274=(if (common.v1647!=0.0){self.scalar_static_f64[977]}else{common.v3});
        let v4275=(common.v1668*v4273);
        let v4277=(common.v1668*v4274);
        let v4279=(common.v34*common.v1671);
        let v4284=(self.scalar_static_f64[253]*f64::powf(common.v1671,self.scalar_static_f64[373]));
        let v4330=(common.v1691*common.v1691);
        let v4336=(if (common.v1647!=0.0){(((common.v1691*self.scalar_static_f64[980])-(common.v1690*(self.scalar_static_f64[457]*(if (common.v1647!=0.0){(common.v1590*((common.v1685*(((v4275+v4275)/v4279)*v4284))+(common.v1673*((self.scalar_static_f64[51]*(-(self.scalar_static_f64[256]*(common.v161*v4273))))-((common.v1683*((common.v1681*v4273)+(common.v1668*(common.v481*v4273))))+(common.v1682*v4273))))))}else{common.v3}))))/v4330)}else{v4273});
        let v4337=(if (common.v1647!=0.0){(((common.v1691*self.scalar_static_f64[981])-(common.v1690*(self.scalar_static_f64[457]*(if (common.v1647!=0.0){(common.v1590*((common.v1685*(((v4277+v4277)/v4279)*v4284))+(common.v1673*((self.scalar_static_f64[51]*(-(self.scalar_static_f64[256]*(common.v161*v4274))))-((common.v1683*((common.v1681*v4274)+(common.v1668*(common.v481*v4274))))+(common.v1682*v4274))))))}else{common.v3}))))/v4330)}else{v4274});
        let v4351=(common.v1693*common.v1693);
        let v4471=(common.v34*v1753);
        let v4479=(v1754*v1754);
        let v4480=(((v1754*(self.scalar_static_f64[896]*common.v2741))-(v1748*((self.scalar_static_f64[898]*common.v2741)/v4471)))/v4479);
        let v4484=(((v1754*(self.scalar_static_f64[896]*common.v2742))-(v1748*((self.scalar_static_f64[898]*common.v2742)/v4471)))/v4479);
        let v4488=(((v1754*(self.scalar_static_f64[896]*common.v2743))-(v1748*((self.scalar_static_f64[898]*common.v2743)/v4471)))/v4479);
        let v4492=(((v1754*(self.scalar_static_f64[896]*common.v2744))-(v1748*((self.scalar_static_f64[898]*common.v2744)/v4471)))/v4479);
        let v4496=(self.scalar_static_f64[899]*common.v2715);
        let v4498=(self.scalar_static_f64[899]*common.v2716);
        let v4502=(self.scalar_static_f64[901]*common.v2715);
        let v4504=(self.scalar_static_f64[901]*common.v2716);
        let v4505=(common.v34*v1771);
        let v4513=(v1772*v1772);
        let v4535=(self.scalar_static_f64[902]*common.v2741);
        let v4536=(self.scalar_static_f64[902]*common.v2742);
        let v4538=(self.scalar_static_f64[902]*common.v2743);
        let v4546=(self.scalar_static_f64[901]*common.v2741);
        let v4547=(self.scalar_static_f64[901]*common.v2742);
        let v4549=(self.scalar_static_f64[901]*common.v2743);
        let v4551=(common.v34*v1784);
        let v4561=(v1785*v1785);
        let v4589=(common.v34*v1793);
        let v4595=(v1794*v1794);
        let v4607=(common.v34*v1800);
        let v4615=(v1801*v1801);
        let v4624=(((v1801*v4538)-(v1797*(v4549/v4607)))/v4615);
        let v4629=(if self.scalar_static_bool[41]{common.v3}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*(self.scalar_static_f64[902]*(-v2799)))-(v1779*((self.scalar_static_f64[901]*(self.scalar_static_f64[261]*v2799))/v4551)))/v4561)}else{common.v3})});
        let v4630=(if self.scalar_static_bool[41]{(((v1801*v4535)-(v1797*(v4546/v4607)))/v4615)}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*v4535)-(v1779*(v4546/v4551)))/v4561)}else{common.v3})});
        let v4631=(if self.scalar_static_bool[41]{(((v1801*v4536)-(v1797*(v4547/v4607)))/v4615)}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*v4536)-(v1779*(v4547/v4551)))/v4561)}else{common.v3})});
        let v4632=(if self.scalar_static_bool[41]{v4624}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*(self.scalar_static_f64[902]*(common.v2743-v2800)))-(v1779*((self.scalar_static_f64[901]*(common.v2743+(self.scalar_static_f64[261]*v2800)))/v4551)))/v4561)}else{common.v3})});
        let v4633=(if self.scalar_static_bool[41]{v4624}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*v4538)-(v1779*(v4549/v4551)))/v4561)}else{common.v3})});
        let v4634=(if self.scalar_static_bool[41]{(((v1801*(self.scalar_static_f64[902]*common.v2744))-(v1797*((self.scalar_static_f64[901]*common.v2744)/v4607)))/v4615)}else{(if (self.scalar_static_f64[258]!=0.0){(((v1785*(self.scalar_static_f64[902]*(common.v2744-v2801)))-(v1779*((self.scalar_static_f64[901]*(common.v2744+(self.scalar_static_f64[261]*v2801)))/v4551)))/v4561)}else{common.v3})});
        let v4639=(common.v34*v1812);
        let v4645=(v1813*v1813);
        let v4961=(common.v1900*common.v4705);
        let v4971=(common.v1900*common.v4707);
        let v4990=(common.v1900*common.v4791);
        let v5002=(common.v1900*common.v4794);
        let v5028=(v1909*self.scalar_static_f64[382]);
        let v5030=(v1909*self.scalar_static_f64[383]);
        let v5032=(v1909*self.scalar_static_f64[384]);
        let v5043=(common.v34*v1919);
        let v5044=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4809})/v5043);
        let v5045=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4810})/v5043);
        let v5046=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4811})/v5043);
        let v5047=((if (self.scalar_static_f64[274]!=0.0){(v5028+v5028)}else{common.v4809})/v5043);
        let v5048=((if (self.scalar_static_f64[274]!=0.0){(v5030+v5030)}else{common.v4812})/v5043);
        let v5049=((if (self.scalar_static_f64[274]!=0.0){(v5032+v5032)}else{common.v4813})/v5043);
        let v5050=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4814})/v5043);
        let v5051=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4815})/v5043);
        let v5052=((if (self.scalar_static_f64[274]!=0.0){common.v3}else{common.v4816})/v5043);
        let v5058=(v1920*v1920);
        let v5105=(if v1924{(common.v443*v5044)}else{(if v1916{((-(self.scalar_static_f64[276]*v5044))/v5058)}else{common.v3})});
        let v5106=(if v1924{(common.v443*v5045)}else{(if v1916{((-(self.scalar_static_f64[276]*v5045))/v5058)}else{common.v3})});
        let v5107=(if v1924{(common.v443*v5046)}else{(if v1916{((-(self.scalar_static_f64[276]*v5046))/v5058)}else{common.v3})});
        let v5108=(if v1924{(common.v443*(self.scalar_static_f64[385]+v5047))}else{(if v1916{((-(self.scalar_static_f64[276]*(v5047-self.scalar_static_f64[385])))/v5058)}else{common.v3})});
        let v5109=(if v1924{(common.v443*(self.scalar_static_f64[386]+v5048))}else{(if v1916{((-(self.scalar_static_f64[276]*(v5048-self.scalar_static_f64[386])))/v5058)}else{common.v3})});
        let v5110=(if v1924{(common.v443*(self.scalar_static_f64[387]+v5049))}else{(if v1916{((-(self.scalar_static_f64[276]*(v5049-self.scalar_static_f64[387])))/v5058)}else{common.v3})});
        let v5111=(if v1924{(common.v443*v5050)}else{(if v1916{((-(self.scalar_static_f64[276]*v5050))/v5058)}else{common.v3})});
        let v5112=(if v1924{(common.v443*v5051)}else{(if v1916{((-(self.scalar_static_f64[276]*v5051))/v5058)}else{common.v3})});
        let v5113=(if v1924{(common.v443*v5052)}else{(if v1916{((-(self.scalar_static_f64[276]*v5052))/v5058)}else{common.v3})});
        let v5124=(self.scalar_static_f64[277]*f64::powf(v1946,self.scalar_static_f64[286]));
        let v5134=(v1948*v1948);
        let v5171=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5105)}else{(if v1945{(((v5105/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5172=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5106)}else{(if v1945{(((v5106/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5173=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5107)}else{(if v1945{(((v5107/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5174=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5108)}else{(if v1945{(((v5108/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5175=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5109)}else{(if v1945{(((v5109/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5176=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5110)}else{(if v1945{(((v5110/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5177=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5111)}else{(if v1945{(((v5111/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5178=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5112)}else{(if v1945{(((v5112/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5179=(if self.scalar_static_bool[52]{common.v3}else{(if v1952{(self.scalar_static_f64[291]*v5113)}else{(if v1945{(((v5113/self.scalar_static_f64[282])*v5124)/v5134)}else{common.v3})})});
        let v5202=(v1958*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4488)}else{v4488}));
        let v5222=(v1958*(self.scalar_static_f64[707]*common.v4051));
        let v5231=(v1958*(if (self.scalar_static_f64[266]!=0.0){(v4961+(common.v1835*common.v4952))}else{common.v3}));
        let v5318=(v1975*v1975);
        let v5333=(common.v161*(if (v1978!=0.0){common.v3}else{((-(self.scalar_static_f64[603]*((common.v1974*common.v3698)+(common.v1316*common.v5300))))/v5318)}));
        let v5334=(common.v161*(if (v1978!=0.0){common.v3}else{((-(self.scalar_static_f64[603]*((common.v1974*common.v3699)+(common.v1316*common.v5301))))/v5318)}));
        let v5335=(common.v161*(if (v1978!=0.0){common.v3}else{((-(self.scalar_static_f64[603]*((common.v1974*common.v3700)+(common.v1316*common.v5302))))/v5318)}));
        let v5336=(common.v161*(if (v1978!=0.0){common.v3}else{((-(self.scalar_static_f64[603]*((common.v1974*common.v3701)+(common.v1316*common.v5303))))/v5318)}));
        let v5343=(v1980*v1980);
        let v5360=((-v3726)/self.scalar_static_f64[295]);
        let v5361=((-v3730)/self.scalar_static_f64[295]);
        let v5362=((-v3734)/self.scalar_static_f64[295]);
        let v5363=((-v3738)/self.scalar_static_f64[295]);
        let v5388=(if common.v1999{(common.v2010*(if common.v2004{(common.v2005*v5360)}else{(if common.v2000{(common.v2001*v5360)}else{common.v3})}))}else{common.v3});
        let v5389=(if common.v1999{((common.v2010*(if common.v2004{(common.v2005*v5361)}else{(if common.v2000{(common.v2001*v5361)}else{common.v3})}))+(common.v2009*self.scalar_static_f64[351]))}else{common.v3});
        let v5390=(if common.v1999{((common.v2010*(if common.v2004{(common.v2005*v5362)}else{(if common.v2000{(common.v2001*v5362)}else{common.v3})}))+(self.scalar_static_f64[0]*common.v2009))}else{common.v3});
        let v5391=(if common.v1999{(common.v2010*(if common.v2004{(common.v2005*v5363)}else{(if common.v2000{(common.v2001*v5363)}else{common.v3})}))}else{common.v3});
        let v5394=(self.scalar_static_f64[296]*f64::powf(common.v2012,self.scalar_static_f64[388]));
        let v5399=(self.scalar_static_f64[919]*(v5388*v5394));
        let v5400=(self.scalar_static_f64[919]*(v5389*v5394));
        let v5401=(self.scalar_static_f64[919]*(v5390*v5394));
        let v5402=(self.scalar_static_f64[919]*(v5391*v5394));
        let v5415=(if common.v2023{(common.v2024*v5399)}else{(if v2019{(v2020*v5399)}else{common.v3})});
        let v5416=(if common.v2023{(common.v2024*v5400)}else{(if v2019{(v2020*v5400)}else{common.v3})});
        let v5417=(if common.v2023{(common.v2024*v5401)}else{(if v2019{(v2020*v5401)}else{common.v3})});
        let v5418=(if common.v2023{(common.v2024*v5402)}else{(if v2019{(v2020*v5402)}else{common.v3})});
        let v5442=(common.v1173*common.v1173);
        let v5451=(if v2041{(((common.v1173*self.scalar_static_f64[351])-(v2048*common.v3392))/v5442)}else{common.v3183});
        let v5452=(if v2041{(((self.scalar_static_f64[0]*common.v1173)-(v2048*common.v3393))/v5442)}else{common.v3184});
        let v5453=(if v2041{((-(v2048*common.v3394))/v5442)}else{common.v3185});
        let v5460=(common.v34*v2053);
        let v5464=(if v2041{(((common.v34*v5451)/v2047)/v5460)}else{common.v3});
        let v5465=(if v2041{(((common.v34*v5452)/v2047)/v5460)}else{common.v3});
        let v5466=(if v2041{(((common.v34*v5453)/v2047)/v5460)}else{common.v3});
        let v5473=(if v2061{(-(common.v443*common.v3374))}else{common.v3});
        let v5474=(if v2061{(-(common.v443*common.v3375))}else{common.v3});
        let v5475=(if v2061{(-(common.v443*common.v3376))}else{common.v3});
        let v5488=(if v2061{((v2065*v5473)+(v2064*(self.scalar_static_f64[301]*v5473)))}else{common.v3});
        let v5489=(if v2061{((v2065*v5474)+(v2064*(self.scalar_static_f64[301]*v5474)))}else{common.v3});
        let v5490=(if v2061{((v2065*v5475)+(v2064*(self.scalar_static_f64[301]*v5475)))}else{common.v3});
        let v5500=(v2054*v5464);
        let v5502=(v2054*v5465);
        let v5504=(v2054*v5466);
        let v5506=(v2067*v5488);
        let v5508=(v2067*v5489);
        let v5510=(v2067*v5490);
        let v5515=(common.v34*v2072);
        let v5522=(v2072*v2072);
        let v5532=(if v2041{(((v2072*((v2067*v5464)+(v2054*v5488)))-(v2068*(((v5500+v5500)+(v5506+v5506))/v5515)))/v5522)}else{common.v3});
        let v5533=(if v2041{(((v2072*((v2067*v5465)+(v2054*v5489)))-(v2068*(((v5502+v5502)+(v5508+v5508))/v5515)))/v5522)}else{common.v3});
        let v5534=(if v2041{(((v2072*((v2067*v5466)+(v2054*v5490)))-(v2068*(((v5504+v5504)+(v5510+v5510))/v5515)))/v5522)}else{common.v3});
        let v5538=(v2074*v2074);
        let v5547=(if v2041{(((v2074*self.scalar_static_f64[351])-(v2048*v5532))/v5538)}else{common.v3});
        let v5548=(if v2041{(((self.scalar_static_f64[0]*v2074)-(v2048*v5533))/v5538)}else{common.v3});
        let v5549=(if v2041{((-(v2048*v5534))/v5538)}else{common.v3});
        let v5550=(common.v443*v5532);
        let v5551=(common.v443*v5533);
        let v5552=(common.v443*v5534);
        let v5553=(v2047*v5550);
        let v5554=(v2047*v5551);
        let v5555=(v2047*v5552);
        let v5568=(if v2041{(v5547+((v2078*common.v3392)+(common.v1173*v5553)))}else{common.v3});
        let v5569=(if v2041{(v5548+((v2078*common.v3393)+(common.v1173*v5554)))}else{common.v3});
        let v5570=(if v2041{(v5549+((v2078*common.v3394)+(common.v1173*v5555)))}else{common.v3});
        let v5590=(v2094*v2094);
        let v5618=(if v2061{(-(v2078*(-(v3726/v2094))))}else{common.v3});
        let v5619=(if v2061{(v5547-((v2096*v5553)+(v2078*(-(((v2094*v3730)-(common.v1323*(self.scalar_static_f64[217]*(if v2061{(self.scalar_static_f64[307]*(common.v34*common.v3374))}else{common.v3}))))/v5590)))))}else{common.v3});
        let v5620=(if v2061{(v5548-((v2096*v5554)+(v2078*(-(((v2094*v3734)-(common.v1323*(self.scalar_static_f64[217]*(if v2061{(self.scalar_static_f64[307]*(common.v34*common.v3375))}else{common.v3}))))/v5590)))))}else{common.v3});
        let v5621=(if v2061{(v5549-((v2096*v5555)+(v2078*(-(((v2094*v3738)-(common.v1323*(self.scalar_static_f64[217]*(if v2061{(self.scalar_static_f64[307]*(common.v34*common.v3376))}else{common.v3}))))/v5590)))))}else{common.v3});
        let v5625=(v2100*v5618);
        let v5627=(v2100*(v5619-v5568));
        let v5629=(v2100*(v5620-v5569));
        let v5631=(v2100*(v5621-v5570));
        let v5667=(common.v34*v2109);
        let v5680=(if v2061{(common.v443*(v5618+((if v2061{(v5625+v5625)}else{common.v3})/v5667)))}else{common.v3});
        let v5681=(if v2061{(common.v443*((v5568+v5619)+((if v2061{((v5627+v5627)+(((v2103*common.v3383)+(common.v1170*((v2102*v5547)+(v2076*(common.v49*v5547)))))/self.scalar_static_f64[217]))}else{v5451})/v5667)))}else{(if v2058{v5568}else{common.v3})});
        let v5682=(if v2061{(common.v443*((v5569+v5620)+((if v2061{((v5629+v5629)+(((v2103*common.v3384)+(common.v1170*((v2102*v5548)+(v2076*(common.v49*v5548)))))/self.scalar_static_f64[217]))}else{v5452})/v5667)))}else{(if v2058{v5569}else{common.v3})});
        let v5683=(if v2061{(common.v443*((v5570+v5621)+((if v2061{((v5631+v5631)+(((v2103*common.v3385)+(common.v1170*((v2102*v5549)+(v2076*(common.v49*v5549)))))/self.scalar_static_f64[217]))}else{v5453})/v5667)))}else{(if v2058{v5570}else{common.v3})});
        let v5690=(v2112*v2112);
        let v5710=(v2115*v2115);
        let v5724=(if v2120{((-(v2077*(if v2041{(((v2112*v5680)-(v2113*v5680))/v5690)}else{common.v3})))/v5710)}else{common.v3});
        let v5725=(if v2120{(((v2115*v5550)-(v2077*(if v2041{(((v2112*(v5681-v5547))-(v2113*v5681))/v5690)}else{common.v3})))/v5710)}else{common.v3});
        let v5726=(if v2120{(((v2115*v5551)-(v2077*(if v2041{(((v2112*(v5682-v5548))-(v2113*v5682))/v5690)}else{common.v3})))/v5710)}else{common.v3});
        let v5727=(if v2120{(((v2115*v5552)-(v2077*(if v2041{(((v2112*(v5683-v5549))-(v2113*v5683))/v5690)}else{common.v3})))/v5710)}else{common.v3});
        let v5746=((-(self.scalar_static_f64[922]*v5680))/v5690);
        let v5749=((-(self.scalar_static_f64[922]*v5681))/v5690);
        let v5752=((-(self.scalar_static_f64[922]*v5682))/v5690);
        let v5755=((-(self.scalar_static_f64[922]*v5683))/v5690);
        let v5756=(v2128*v5746);
        let v5757=(v2128*v5749);
        let v5758=(v2128*v5752);
        let v5759=(v2128*v5755);
        let v5762=(v2122*v2122);
        let v5830=(self.scalar_static_f64[296]*f64::powf(common.v2010,self.scalar_static_f64[388]));
        let v5836=(common.v2149*common.v2149);
        let v5856=(self.scalar_static_f64[313]*f64::powf(common.v2151,self.scalar_static_f64[389]));
        let v5869=(if common.v2146{(common.v2147*((-(((common.v2149*v3726)-(common.v1323*v3726))/v5836))*v5856))}else{common.v3});
        let v5870=(if common.v2146{((common.v2153*(self.scalar_static_f64[351]*v5830))+(common.v2147*((-(((common.v2149*v3730)-(common.v1323*v3730))/v5836))*v5856)))}else{common.v3});
        let v5871=(if common.v2146{((common.v2153*(self.scalar_static_f64[0]*v5830))+(common.v2147*((-(((common.v2149*v3734)-(common.v1323*v3734))/v5836))*v5856)))}else{common.v3});
        let v5872=(if common.v2146{(common.v2147*((-(((common.v2149*v3738)-(common.v1323*v3738))/v5836))*v5856))}else{common.v3});
        let v5881=(if common.v2158{(v3726/self.scalar_static_f64[312])}else{common.v3});
        let v5882=(if common.v2158{(v3730/self.scalar_static_f64[312])}else{common.v3});
        let v5883=(if common.v2158{(v3734/self.scalar_static_f64[312])}else{common.v3});
        let v5884=(if common.v2158{(v3738/self.scalar_static_f64[312])}else{common.v3});
        let v5889=(if common.v2158{(v5881/self.scalar_static_f64[315])}else{self.scalar_static_f64[364]});
        let v5890=(if common.v2158{(v5882/self.scalar_static_f64[315])}else{self.scalar_static_f64[365]});
        let v5891=(if common.v2158{(v5883/self.scalar_static_f64[315])}else{common.v3});
        let v5892=(if common.v2158{(v5884/self.scalar_static_f64[315])}else{common.v3});
        let v5935=(self.scalar_static_f64[316]*f64::powf(common.v2184,self.scalar_static_f64[390]));
        let v5956=(self.scalar_static_f64[919]*(if common.v2158{((common.v2186*v5869)+(common.v2155*((if common.v2177{(v5881+(self.scalar_static_f64[315]*((common.v2179*(-v5889))/common.v2180)))}else{(if common.v2169{(self.scalar_static_f64[315]*((common.v2170*v5889)/common.v2171))}else{common.v3})})*v5935)))}else{(if common.v2156{v5869}else{common.v3})}));
        let v5957=(self.scalar_static_f64[919]*(if common.v2158{((common.v2186*v5870)+(common.v2155*((if common.v2177{(v5882+(self.scalar_static_f64[315]*((common.v2179*(-v5890))/common.v2180)))}else{(if common.v2169{(self.scalar_static_f64[315]*((common.v2170*v5890)/common.v2171))}else{common.v3})})*v5935)))}else{(if common.v2156{v5870}else{common.v3})}));
        let v5958=(self.scalar_static_f64[919]*(if common.v2158{((common.v2186*v5871)+(common.v2155*((if common.v2177{(v5883+(self.scalar_static_f64[315]*((common.v2179*(-v5891))/common.v2180)))}else{(if common.v2169{(self.scalar_static_f64[315]*((common.v2170*v5891)/common.v2171))}else{common.v3})})*v5935)))}else{(if common.v2156{v5871}else{common.v3})}));
        let v5959=(self.scalar_static_f64[919]*(if common.v2158{((common.v2186*v5872)+(common.v2155*((if common.v2177{(v5884+(self.scalar_static_f64[315]*((common.v2179*(-v5892))/common.v2180)))}else{(if common.v2169{(self.scalar_static_f64[315]*((common.v2170*v5892)/common.v2171))}else{common.v3})})*v5935)))}else{(if common.v2156{v5872}else{common.v3})}));
        let v5986=(if common.v2146{(v2202*(if common.v2196{(common.v2197*v5956)}else{(if v2192{(v2193*v5956)}else{v5415})}))}else{(if v2137{(v2138*v5756)}else{(if v2120{((v2133*((v2124*v5724)+(v2122*(self.scalar_static_f64[921]*v5680))))+(v2125*(v5756-(v2132*((v2130*v5746)+(v2127*((-(v2067*v5724))/v5762)))))))}else{(if common.v1999{((v2031*v5415)+(v2028*(self.scalar_static_f64[920]*v5388)))}else{common.v3})})})});
        let v5987=(if common.v2146{((v2202*(if common.v2196{(common.v2197*v5957)}else{(if v2192{(v2193*v5957)}else{v5416})}))+(v2201*self.scalar_static_f64[982]))}else{(if v2137{((v2138*v5757)+(v2128*(self.scalar_static_f64[4]*v5488)))}else{(if v2120{((v2133*((v2124*v5725)+(v2122*(self.scalar_static_f64[921]*v5681))))+(v2125*(v5757-(v2132*((v2130*v5749)+(v2127*(((v2122*v5488)-(v2067*v5725))/v5762)))))))}else{(if common.v1999{((v2031*v5416)+(v2028*(self.scalar_static_f64[920]*v5389)))}else{common.v3})})})});
        let v5988=(if common.v2146{((v2202*(if common.v2196{(common.v2197*v5958)}else{(if v2192{(v2193*v5958)}else{v5417})}))+(v2201*self.scalar_static_f64[983]))}else{(if v2137{((v2138*v5758)+(v2128*(self.scalar_static_f64[4]*v5489)))}else{(if v2120{((v2133*((v2124*v5726)+(v2122*(self.scalar_static_f64[921]*v5682))))+(v2125*(v5758-(v2132*((v2130*v5752)+(v2127*(((v2122*v5489)-(v2067*v5726))/v5762)))))))}else{(if common.v1999{((v2031*v5417)+(v2028*(self.scalar_static_f64[920]*v5390)))}else{common.v3})})})});
        let v5989=(if common.v2146{(v2202*(if common.v2196{(common.v2197*v5959)}else{(if v2192{(v2193*v5959)}else{v5418})}))}else{(if v2137{((v2138*v5759)+(v2128*(self.scalar_static_f64[4]*v5490)))}else{(if v2120{((v2133*((v2124*v5727)+(v2122*(self.scalar_static_f64[921]*v5683))))+(v2125*(v5759-(v2132*((v2130*v5755)+(v2127*(((v2122*v5490)-(v2067*v5727))/v5762)))))))}else{(if common.v1999{((v2031*v5418)+(v2028*(self.scalar_static_f64[920]*v5391)))}else{common.v3})})})});
        let v6004=(v2213*v2213);
        let v6029=(v2212*v2212);
        let v6044=(if v2211{((((-(self.scalar_static_f64[411]*((v2212*v3726)+(common.v1323*v5333))))/v6004)+(self.scalar_static_f64[715]*(common.v3704/self.scalar_static_f64[687])))+((-(self.scalar_static_f64[600]*v5333))/v6029))}else{common.v3});
        let v6045=(if v2211{((((-(self.scalar_static_f64[411]*((v2212*v3730)+(common.v1323*v5334))))/v6004)+(self.scalar_static_f64[715]*(common.v3707/self.scalar_static_f64[687])))+((-(self.scalar_static_f64[600]*v5334))/v6029))}else{common.v3});
        let v6046=(if v2211{((((-(self.scalar_static_f64[411]*((v2212*v3734)+(common.v1323*v5335))))/v6004)+(self.scalar_static_f64[715]*(common.v3710/self.scalar_static_f64[687])))+((-(self.scalar_static_f64[600]*v5335))/v6029))}else{common.v3});
        let v6047=(if v2211{((((-(self.scalar_static_f64[411]*((v2212*v3738)+(common.v1323*v5336))))/v6004)+(self.scalar_static_f64[715]*(common.v3713/self.scalar_static_f64[687])))+((-(self.scalar_static_f64[600]*v5336))/v6029))}else{common.v3});
        let v6056=(if v2221{((v5986-v6044)/common.v439)}else{v5889});
        let v6057=(if v2221{((v5987-v6045)/common.v439)}else{v5890});
        let v6058=(if v2221{((v5988-v6046)/common.v439)}else{v5891});
        let v6059=(if v2221{((v5989-v6047)/common.v439)}else{v5892});
        let v6100=(if v2235{(v6044-(common.v439*((v2237*(-v6056))/v2238)))}else{(if v2227{(v5986-(common.v439*((v2228*v6056)/v2229)))}else{v5986})});
        let v6101=(if v2235{(v6045-(common.v439*((v2237*(-v6057))/v2238)))}else{(if v2227{(v5987-(common.v439*((v2228*v6057)/v2229)))}else{v5987})});
        let v6102=(if v2235{(v6046-(common.v439*((v2237*(-v6058))/v2238)))}else{(if v2227{(v5988-(common.v439*((v2228*v6058)/v2229)))}else{v5988})});
        let v6103=(if v2235{(v6047-(common.v439*((v2237*(-v6059))/v2238)))}else{(if v2227{(v5989-(common.v439*((v2228*v6059)/v2229)))}else{v5989})});
        let v6106=((v2242*v3726)+(common.v1323*v6100));
        let v6109=((v2242*v3730)+(common.v1323*v6101));
        let v6112=((v2242*v3734)+(common.v1323*v6102));
        let v6115=((v2242*v3738)+(common.v1323*v6103));
        let v6139=(v2248*v2248);
        let v7122=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[753]*common.v4072)));
        let v7126=((self.scalar_static_f64[377]+((if self.scalar_static_bool[33]{(self.scalar_static_f64[715]*((self.scalar_static_f64[248]*common.v3819)+(v1430*(self.scalar_static_f64[246]*common.v3819))))}else{(if self.scalar_static_bool[31]{v3849}else{(if (self.scalar_static_f64[154]!=0.0){((v3849+(v1430*(((v1428*(self.scalar_static_f64[892]*common.v3819))-(v1424*((common.v455*v3827)/v3855)))/v3861)))+(((v1436*(v1434*v3845))-(v1435*v3845))/v3891))}else{common.v3})})})+(self.scalar_static_f64[700]*common.v4010)))-(if v1640{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[22]*(self.scalar_static_f64[579]*((v1635*(if common.v1562{(common.v1563*v4090)}else{(if v1558{(v1559*v4090)}else{common.v3})}))+(v1567*((v1634*common.v3426)+(common.v1200*(self.scalar_static_f64[894]*(if v1621{((v1630*((v1622*v4165)+(common.v1597*self.scalar_static_f64[371])))+(v1623*((v1628*(v1624*v4165))+(v1625*(v1626*v4165)))))}else{(if common.v1603{((self.scalar_static_f64[0]*v1617)+(v1614*(((common.v1597*(-(if common.v1608{(common.v1609*v4165)}else{(if v1604{(v1605*v4165)}else{common.v3})})))-(v1615*v4165))/v4180)))}else{common.v3})}))))))))}else{common.v3})}));
        let v7127=((self.scalar_static_f64[376]+((if self.scalar_static_bool[33]{(self.scalar_static_f64[715]*((self.scalar_static_f64[248]*common.v3820)+((v1452*common.v3606)+(v1430*(self.scalar_static_f64[246]*(common.v3331+common.v3820))))))}else{(if self.scalar_static_bool[31]{v3850}else{(if (self.scalar_static_f64[154]!=0.0){((v3850+((v1430*(((v1428*(self.scalar_static_f64[892]*common.v3820))-(v1424*((common.v455*v3828)/v3855)))/v3861))+(v1429*common.v3606)))+(((v1436*((v1434*v3846)+(v1420*(self.scalar_static_f64[730]*common.v3331))))-(v1435*v3846))/v3891))}else{common.v3})})})+(self.scalar_static_f64[700]*common.v4012)))-(if v1640{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[22]*(self.scalar_static_f64[579]*((v1635*(if common.v1562{(common.v1563*v4091)}else{(if v1558{(v1559*v4091)}else{common.v3})}))+(v1567*((v1634*common.v3427)+(common.v1200*(self.scalar_static_f64[894]*(if v1621{((v1630*((v1622*v4166)+(common.v1597*self.scalar_static_f64[372])))+(v1623*((v1628*(v1624*v4166))+(v1625*(v1626*v4166)))))}else{(if common.v1603{((v1617*self.scalar_static_f64[351])+(v1614*(((common.v1597*(-(if common.v1608{(common.v1609*v4166)}else{(if v1604{(v1605*v4166)}else{common.v3})})))-(v1615*v4166))/v4180)))}else{common.v3})}))))))))}else{common.v3})}));
        let v7160=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5171))));
        let v7161=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5172))));
        let v7162=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5173))));
        let v7163=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5174))));
        let v7164=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-((v1958*(if v1733{common.v3}else{(if (common.v1647!=0.0){(self.scalar_static_f64[54]*(self.scalar_static_f64[580]*((v1728*(if common.v1662{(common.v1663*v4263)}else{(if v1658{(v1659*v4263)}else{common.v3})}))+(v1667*((v1727*v4250)+(common.v1651*(self.scalar_static_f64[895]*(if v1716{((v1723*((v1717*v4336)+(common.v1693*self.scalar_static_f64[372])))+(v1718*((v1721*(v1624*v4336))+(v1719*(v1626*v4336)))))}else{(if common.v1698{((v1712*self.scalar_static_f64[351])+(v1709*(((common.v1693*(-(if common.v1703{(common.v1704*v4336)}else{(if v1699{(v1700*v4336)}else{common.v3})})))-(v1710*v4336))/v4351)))}else{common.v3})}))))))))}else{common.v3})}))+(v1734*v5175)))));
        let v7165=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-((v1958*(if v1733{common.v3}else{(if (common.v1647!=0.0){(self.scalar_static_f64[54]*(self.scalar_static_f64[580]*((v1728*(if common.v1662{(common.v1663*v4264)}else{(if v1658{(v1659*v4264)}else{common.v3})}))+(v1667*((v1727*v4251)+(common.v1651*(self.scalar_static_f64[895]*(if v1716{((v1723*((v1717*v4337)+(common.v1693*self.scalar_static_f64[371])))+(v1718*((v1721*(v1624*v4337))+(v1719*(v1626*v4337)))))}else{(if common.v1698{((self.scalar_static_f64[0]*v1712)+(v1709*(((common.v1693*(-(if common.v1703{(common.v1704*v4337)}else{(if v1699{(v1700*v4337)}else{common.v3})})))-(v1710*v4337))/v4351)))}else{common.v3})}))))))))}else{common.v3})}))+(v1734*v5176)))));
        let v7166=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5177))));
        let v7167=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5178))));
        let v7168=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1734*v5179))));
        let v7216=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(v4990+(common.v1859*common.v4952))}else{common.v3})));
        let v7263=ddt_scale;
        let v7384=(self.scalar_static_f64[15]*(v7263*common.v7366));
        let v7426=(self.scalar_static_f64[15]*(v7263*common.v7418));

        stamper.stamp_current_node3_local(
            Some(7),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v960))),
            6,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2894))),
            7,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2895))),
            8,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2896))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v1323))),
            [4, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3726)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3730)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3734)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3738))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[753]*(common.v1544-common.v1))+((if self.scalar_static_bool[30]{v1484}else{(if (self.scalar_static_f64[154]!=0.0){(v1484+(v1486/v1490))}else{common.v3})})+(self.scalar_static_f64[747]*(common.v1518-common.v1))))))),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[753]*common.v4069)+((if self.scalar_static_bool[30]{v3966}else{(if (self.scalar_static_f64[154]!=0.0){(v3966+(((v1490*(self.scalar_static_f64[893]*common.v3953))-(v1486*((common.v455*(if common.v1477{(common.v1478*self.scalar_static_f64[945])}else{(if v1473{(v1474*self.scalar_static_f64[945])}else{v3827})}))/v3975)))/v3982))}else{common.v3})})+(self.scalar_static_f64[747]*common.v4025))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[753]*common.v4070)+((if self.scalar_static_bool[30]{v3967}else{(if (self.scalar_static_f64[154]!=0.0){(v3967+(((v1490*(self.scalar_static_f64[893]*common.v3954))-(v1486*((common.v455*(if common.v1477{(common.v1478*self.scalar_static_f64[944])}else{(if v1473{(v1474*self.scalar_static_f64[944])}else{common.v3})}))/v3975)))/v3982))}else{common.v3})})+(self.scalar_static_f64[747]*common.v4026))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[753]*common.v4071)+((if self.scalar_static_bool[30]{v3968}else{(if (self.scalar_static_f64[154]!=0.0){(v3968+(((v1490*(self.scalar_static_f64[893]*common.v3955))-(v1486*((common.v455*(if common.v1477{common.v3}else{(if v1473{common.v3}else{v3828})}))/v3975)))/v3982))}else{common.v3})})+(self.scalar_static_f64[747]*common.v4027))))), v7122, v7122, (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[753]*common.v4073)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[758]*(v1352-common.v1))+((v1375*v1377)+((((if self.scalar_static_bool[33]{(self.scalar_static_f64[715]*((v1421*self.scalar_static_f64[248])+(v1430*v1452)))}else{(if self.scalar_static_bool[31]{v1422}else{(if (self.scalar_static_f64[154]!=0.0){((v1422+(v1429*v1430))+(v1435/v1436))}else{common.v3})})})+(self.scalar_static_f64[700]*(common.v1505-common.v1)))+(common.v3*common.v763))-(if v1640{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[22]*(self.scalar_static_f64[579]*(v1567*v1635)))}else{common.v3})}))))))),
            [4, 5, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[758]*v3769)+(((v1377*(self.scalar_static_f64[245]*v3795))+(v1375*((-v3795)*v3802)))+v7126)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[700]*common.v4011))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[758]*v3770)+(((v1377*(self.scalar_static_f64[245]*v3796))+(v1375*((-v3796)*v3802)))+v7127)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[715]*((v1452*common.v3607)+(v1430*(self.scalar_static_f64[246]*common.v3332))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[154]!=0.0){((v1429*common.v3607)+(((v1436*((v1434*v3847)+(v1420*(self.scalar_static_f64[730]*common.v3332))))-(v1435*v3847))/v3891))}else{common.v3})})}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[715]*((v1452*common.v3608)+(v1430*(self.scalar_static_f64[246]*common.v3333))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[154]!=0.0){((v1429*common.v3608)+(((v1436*((v1434*v3848)+(v1420*(self.scalar_static_f64[730]*common.v3333))))-(v1435*v3848))/v3891))}else{common.v3})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if (self.scalar_static_f64[154]!=0.0){v2633}else{common.v3})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if (self.scalar_static_f64[154]!=0.0){v7160}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7161}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7162}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7163}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7164}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7165}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7166}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7167}else{common.v3}), (if (self.scalar_static_f64[154]!=0.0){v7168}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[30]{v2633}else{common.v3})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[30]{v7160}else{common.v3}), (if self.scalar_static_bool[30]{v7161}else{common.v3}), (if self.scalar_static_bool[30]{v7162}else{common.v3}), (if self.scalar_static_bool[30]{v7163}else{common.v3}), (if self.scalar_static_bool[30]{v7164}else{common.v3}), (if self.scalar_static_bool[30]{v7165}else{common.v3}), (if self.scalar_static_bool[30]{v7166}else{common.v3}), (if self.scalar_static_bool[30]{v7167}else{common.v3}), (if self.scalar_static_bool[30]{v7168}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v1803)}else{v1803})))),
            [3, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4629)}else{v4629}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4630)}else{v4630}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4631)}else{v4631}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4632)}else{v4632}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4633)}else{v4633}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4634)}else{v4634})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(v1790/v1794)}else{(if (self.scalar_static_f64[258]!=0.0){(v1763/v1772)}else{common.v3})})))),
            [3, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{common.v3}else{(if (self.scalar_static_f64[258]!=0.0){(((v1772*(self.scalar_static_f64[899]*(-v2776)))-(v1763*((self.scalar_static_f64[901]*(self.scalar_static_f64[261]*v2776))/v4505)))/v4513)}else{common.v3})}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(((v1794*v4496)-(v1790*(v4502/v4589)))/v4595)}else{(if (self.scalar_static_f64[258]!=0.0){(((v1772*v4496)-(v1763*(v4502/v4505)))/v4513)}else{common.v3})}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{common.v3}else{(if (self.scalar_static_f64[258]!=0.0){(((v1772*(self.scalar_static_f64[899]*(-v2777)))-(v1763*((self.scalar_static_f64[901]*(self.scalar_static_f64[261]*v2777))/v4505)))/v4513)}else{common.v3})}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(((v1794*v4498)-(v1790*(v4504/v4589)))/v4595)}else{(if (self.scalar_static_f64[258]!=0.0){(((v1772*v4498)-(v1763*(v4504/v4505)))/v4513)}else{common.v3})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(common.v1859*common.v1900)}else{common.v3})))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v7216, (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4792)+(common.v1859*common.v4953))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4793)+(common.v1859*common.v4954))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(common.v1859*common.v4955)}else{common.v3}))), v7216, (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(v4990+(common.v1859*common.v4956))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(v5002+(common.v1859*common.v4957))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4795)+(common.v1859*common.v4958))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4796)+(common.v1859*common.v4959))}else{common.v3}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if (self.scalar_static_f64[266]!=0.0){(v5002+(common.v1859*common.v4960))}else{common.v3})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1806/v1813)+(common.v3*common.v771))))),
            3,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((((v1813*(self.scalar_static_f64[903]*v2776))-(v1806*((self.scalar_static_f64[905]*v2776)/v4639)))/v4645)+self.scalar_static_f64[376])))),
            7,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((((v1813*(self.scalar_static_f64[903]*v2777))-(v1806*((self.scalar_static_f64[905]*v2777)/v4639)))/v4645)+self.scalar_static_f64[377])))),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1983/v1980)))),
            [4, 5, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1983*v5333))/v5343))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[0]+(self.scalar_static_f64[865]*(if common.v842{(common.v843*self.scalar_static_f64[944])}else{(if (common.v839!=0.0){(v840*self.scalar_static_f64[944])}else{common.v3})})))/v1980))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1980*(self.scalar_static_f64[351]+(self.scalar_static_f64[865]*(if common.v842{(common.v843*self.scalar_static_f64[945])}else{(if (common.v839!=0.0){(v840*self.scalar_static_f64[945])}else{common.v3})}))))-(v1983*v5334))/v5343))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1983*v5335))/v5343))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1983*v5336))/v5343)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v2253)))),
            [4, 6, 7, 8],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2252{v6106}else{(if v2246{(((v2248*((v2243*v6044)+(v2220*v6106)))-(v2247*(v6044+v6100)))/v6139)}else{(if v2221{v6106}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2252{v6109}else{(if v2246{(((v2248*((v2243*v6045)+(v2220*v6109)))-(v2247*(v6045+v6101)))/v6139)}else{(if v2221{v6109}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2252{v6112}else{(if v2246{(((v2248*((v2243*v6046)+(v2220*v6112)))-(v2247*(v6046+v6102)))/v6139)}else{(if v2221{v6112}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2252{v6115}else{(if v2246{(((v2248*((v2243*v6047)+(v2220*v6115)))-(v2247*(v6047+v6103)))/v6139)}else{(if v2221{v6115}else{common.v3})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*((self.scalar_static_f64[0]*(self.scalar_static_f64[0]*(common.v774-common.v761)))/self.scalar_static_f64[600]))),
            2,
            multiplicity * (self.scalar_static_f64[1013]),
            4,
            multiplicity * (self.scalar_static_f64[1014]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * ((self.scalar_static_f64[15]*((self.scalar_static_f64[0]*common.v779)/self.scalar_static_f64[608]))),
            1,
            multiplicity * (self.scalar_static_f64[1017]),
            5,
            multiplicity * (self.scalar_static_f64[1018]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2655)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(common.v7257*v7263)), (self.scalar_static_f64[15]*(common.v7258*v7263)), (self.scalar_static_f64[15]*(common.v7259*v7263)), (self.scalar_static_f64[15]*(common.v7260*v7263)), (self.scalar_static_f64[15]*(common.v7261*v7263)), (self.scalar_static_f64[15]*(common.v7262*v7263))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*v2658)),
            4,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7276))),
            5,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7277))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*v2661)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v7263*common.v7282)), (self.scalar_static_f64[15]*(v7263*common.v7283)), (self.scalar_static_f64[15]*(v7263*common.v7284)), (self.scalar_static_f64[15]*(v7263*common.v7285)), (self.scalar_static_f64[15]*(v7263*common.v7286)), (self.scalar_static_f64[15]*(v7263*common.v7287))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((self.scalar_static_f64[15]*v2664)),
            3,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7300))),
            7,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7301))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[15]*v2667)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v7263*common.v7306)), (self.scalar_static_f64[15]*(v7263*common.v7307)), (self.scalar_static_f64[15]*(v7263*common.v7308)), (self.scalar_static_f64[15]*(v7263*common.v7309)), (self.scalar_static_f64[15]*(v7263*common.v7310)), (self.scalar_static_f64[15]*(v7263*common.v7311))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[15]*v2671)),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[403]))),
            2,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[404]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[15]*v2675)),
            0,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[405]))),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[406]))),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1902*v1958)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v5231+(v1902*v5171)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4706)+(common.v1835*common.v4953))}else{common.v3}))+(v1902*v5172)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1958*(if (self.scalar_static_f64[266]!=0.0){(common.v1835*common.v4954)}else{common.v3})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){(common.v1835*common.v4955)}else{common.v3}))+(v1902*v5173)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v5231+(v1902*v5174)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){(v4961+(common.v1835*common.v4956))}else{common.v3}))+(v1902*v5175)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){(v4971+(common.v1835*common.v4957))}else{common.v3}))+(v1902*v5176)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){(v4971+(common.v1835*common.v4958))}else{common.v3}))+(v1902*v5177)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){((common.v1900*common.v4708)+(common.v1835*common.v4959))}else{common.v3}))+(v1902*v5178)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1958*(if (self.scalar_static_f64[266]!=0.0){(v4971+(common.v1835*common.v4960))}else{common.v3}))+(v1902*v5179))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[854]*(self.scalar_static_f64[0]*common.v797)))),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [self.scalar_static_f64[1023], self.scalar_static_f64[1024], self.scalar_static_f64[1024], self.scalar_static_f64[1024], self.scalar_static_f64[1025], self.scalar_static_f64[1025], self.scalar_static_f64[1026], self.scalar_static_f64[1025]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*v2683)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v7384, (self.scalar_static_f64[15]*(v7263*common.v7367)), (self.scalar_static_f64[15]*(v7263*common.v7368)), (self.scalar_static_f64[15]*(v7263*common.v7369)), v7384, (self.scalar_static_f64[15]*(v7263*common.v7370)), (self.scalar_static_f64[15]*(v7263*common.v7371)), (self.scalar_static_f64[15]*(v7263*common.v7372)), (self.scalar_static_f64[15]*(v7263*common.v7373)), (self.scalar_static_f64[15]*(v7263*common.v7374))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1823*v1958)+((v1533*v1958)+(common.v3*common.v793)))))),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1823*v5171)+(v1533*v5171)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1823*v5172)+(v1533*v5172)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1823*v5173)+((v1958*(self.scalar_static_f64[707]*common.v4048))+(v1533*v5173))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1958*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4480)}else{v4480}))+(v1823*v5174))+(self.scalar_static_f64[376]+((v1958*(self.scalar_static_f64[707]*common.v4049))+(v1533*v5174)))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1958*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4484)}else{v4484}))+(v1823*v5175))+(((v1958*(self.scalar_static_f64[707]*common.v4050))+(v1533*v5175))+self.scalar_static_f64[399])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v5202+(v1823*v5176))+((v5222+(v1533*v5176))+self.scalar_static_f64[400])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v5202+(v1823*v5177))+((v5222+(v1533*v5177))+self.scalar_static_f64[400])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1823*v5178)+(v1533*v5178)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1958*(if (self.scalar_static_f64[266]!=0.0){(self.scalar_static_f64[7]*v4492)}else{v4492}))+(v1823*v5179))+(self.scalar_static_f64[377]+((v1958*(self.scalar_static_f64[707]*common.v4052))+(v1533*v5179))))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[15]*v2689)),
            [5, 6, 7, 8, 10],
            [(self.scalar_static_f64[15]*(v7263*common.v7416)), (self.scalar_static_f64[15]*(v7263*common.v7417)), v7426, v7426, (self.scalar_static_f64[15]*(v7263*common.v7419))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * ((if (self.scalar_static_f64[212]!=0.0){(self.scalar_static_f64[15]*(self.scalar_static_f64[859]*(self.scalar_static_f64[0]*common.v790)))}else{common.v3})),
            9,
            multiplicity * (self.scalar_static_f64[1031]),
            10,
            multiplicity * (self.scalar_static_f64[1032]),
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
        stamper.stamp_current_node2_local(
            Some(10),
            Some(7),
            multiplicity * ((if (self.scalar_static_f64[213]!=0.0){(self.scalar_static_f64[15]*(self.scalar_static_f64[864]*(self.scalar_static_f64[0]*common.v787)))}else{common.v3})),
            7,
            multiplicity * (self.scalar_static_f64[1037]),
            10,
            multiplicity * (self.scalar_static_f64[1038]),
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
            multiplicity * (common.v2699),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * ((common.v2603*v2700)),
            [4, 5, 6, 7, 8, 10, 11],
            [(v2700*common.v7046), (v2700*common.v7047), (v2700*common.v7048), (v2700*common.v7049), (v2700*common.v7050), (v2700*common.v7051), (common.v2603*v7263)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((v2579*common.v2699)),
            11,
            multiplicity * (v2579),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (common.v2699),
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
            Some(6),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
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
        let v2655=0.0;
        let v2658=0.0;
        let v2661=0.0;
        let v2664=0.0;
        let v2667=0.0;
        let v2671=0.0;
        let v2675=0.0;
        let v2683=0.0;
        let v2689=0.0;
        let v2700=0.0;
        let v7263=1.0;
        let v7384=(self.scalar_static_f64[15]*(v7263*common.v7366));
        let v7426=(self.scalar_static_f64[15]*(v7263*common.v7418));

        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(common.v7257*v7263)), (self.scalar_static_f64[15]*(common.v7258*v7263)), (self.scalar_static_f64[15]*(common.v7259*v7263)), (self.scalar_static_f64[15]*(common.v7260*v7263)), (self.scalar_static_f64[15]*(common.v7261*v7263)), (self.scalar_static_f64[15]*(common.v7262*v7263))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7276))),
            nodes[5],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7277))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v7263*common.v7282)), (self.scalar_static_f64[15]*(v7263*common.v7283)), (self.scalar_static_f64[15]*(v7263*common.v7284)), (self.scalar_static_f64[15]*(v7263*common.v7285)), (self.scalar_static_f64[15]*(v7263*common.v7286)), (self.scalar_static_f64[15]*(v7263*common.v7287))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7300))),
            nodes[7],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*common.v7301))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v7263*common.v7306)), (self.scalar_static_f64[15]*(v7263*common.v7307)), (self.scalar_static_f64[15]*(v7263*common.v7308)), (self.scalar_static_f64[15]*(v7263*common.v7309)), (self.scalar_static_f64[15]*(v7263*common.v7310)), (self.scalar_static_f64[15]*(v7263*common.v7311))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[403]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[404]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[405]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v7263*self.scalar_static_f64[406]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[v7384, (self.scalar_static_f64[15]*(v7263*common.v7367)), (self.scalar_static_f64[15]*(v7263*common.v7368)), (self.scalar_static_f64[15]*(v7263*common.v7369)), v7384, (self.scalar_static_f64[15]*(v7263*common.v7370)), (self.scalar_static_f64[15]*(v7263*common.v7371)), (self.scalar_static_f64[15]*(v7263*common.v7372)), (self.scalar_static_f64[15]*(v7263*common.v7373)), (self.scalar_static_f64[15]*(v7263*common.v7374))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &[nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[15]*(v7263*common.v7416)), (self.scalar_static_f64[15]*(v7263*common.v7417)), v7426, v7426, (self.scalar_static_f64[15]*(v7263*common.v7419))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]],
            &[(v2700*common.v7046), (v2700*common.v7047), (v2700*common.v7048), (v2700*common.v7049), (v2700*common.v7050), (v2700*common.v7051), (common.v2603*v7263)],
            &[],
            &[],
            multiplicity,
        );
    }
}
