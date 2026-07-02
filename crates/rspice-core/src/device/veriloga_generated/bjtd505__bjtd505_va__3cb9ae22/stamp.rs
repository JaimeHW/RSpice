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
    v3: f64,
    v32: f64,
    v33: f64,
    v48: f64,
    v160: f64,
    v409: f64,
    v413: f64,
    v425: f64,
    v451: f64,
    v699: f64,
    v703: f64,
    v705: f64,
    v710: f64,
    v713: f64,
    v718: f64,
    v726: f64,
    v729: f64,
    v732: f64,
    v736: f64,
    v773: f64,
    v774: f64,
    v776: f64,
    v779: bool,
    v780: f64,
    v864: f64,
    v986: f64,
    v1046: f64,
    v1071: f64,
    v1074: f64,
    v1077: f64,
    v1104: f64,
    v1184: f64,
    v1220: f64,
    v1221: f64,
    v1226: f64,
    v1227: f64,
    v1246: f64,
    v1248: f64,
    v1251: bool,
    v1252: f64,
    v1261: f64,
    v1293: f64,
    v1295: f64,
    v1297: f64,
    v1302: bool,
    v1303: f64,
    v1310: f64,
    v1311: f64,
    v1313: f64,
    v1318: bool,
    v1320: f64,
    v1372: f64,
    v1374: f64,
    v1376: f64,
    v1381: bool,
    v1382: f64,
    v1409: f64,
    v1422: f64,
    v1435: f64,
    v1448: f64,
    v1455: f64,
    v1456: f64,
    v1459: f64,
    v1461: f64,
    v1466: bool,
    v1467: f64,
    v1473: f64,
    v1477: f64,
    v1480: f64,
    v1488: f64,
    v1489: f64,
    v1490: f64,
    v1492: f64,
    v1494: f64,
    v1498: f64,
    v1499: f64,
    v1501: f64,
    v1504: f64,
    v1506: f64,
    v1507: bool,
    v1512: bool,
    v1513: f64,
    v1551: f64,
    v1553: f64,
    v1555: f64,
    v1556: f64,
    v1559: f64,
    v1561: f64,
    v1566: bool,
    v1567: f64,
    v1572: f64,
    v1575: f64,
    v1577: f64,
    v1585: f64,
    v1586: f64,
    v1587: f64,
    v1589: f64,
    v1594: f64,
    v1595: f64,
    v1597: f64,
    v1599: f64,
    v1601: f64,
    v1602: bool,
    v1607: bool,
    v1608: f64,
    v1676: f64,
    v1693: f64,
    v1715: f64,
    v1787: f64,
    v1799: f64,
    v1812: bool,
    v1813: bool,
    v1814: f64,
    v1817: bool,
    v1818: f64,
    v1822: f64,
    v1823: f64,
    v1825: f64,
    v1829: f64,
    v1831: f64,
    v1836: bool,
    v1837: f64,
    v1852: bool,
    v1959: bool,
    v1960: f64,
    v1962: f64,
    v1964: f64,
    v1966: f64,
    v1968: f64,
    v1969: bool,
    v1971: bool,
    v1979: f64,
    v1982: bool,
    v1983: f64,
    v1984: f64,
    v1990: bool,
    v1992: f64,
    v1993: f64,
    v1997: f64,
    v1999: f64,
    v2002: f64,
    v2004: f64,
    v2009: bool,
    v2010: f64,
    v2350: f64,
    v2382: f64,
    v2425: f64,
    v2428: f64,
    v2431: f64,
    v2434: f64,
    v2438: f64,
    v2442: f64,
    v2450: f64,
    v2456: f64,
    v2467: f64,
    v2509: f64,
    v2510: f64,
    v2511: f64,
    v2512: f64,
    v2630: f64,
    v2631: f64,
    v2632: f64,
    v2919: f64,
    v2920: f64,
    v2921: f64,
    v3067: f64,
    v3068: f64,
    v3069: f64,
    v3110: f64,
    v3111: f64,
    v3112: f64,
    v3119: f64,
    v3120: f64,
    v3121: f64,
    v3128: f64,
    v3129: f64,
    v3130: f64,
    v3162: f64,
    v3163: f64,
    v3342: f64,
    v3343: f64,
    v3344: f64,
    v3434: f64,
    v3435: f64,
    v3436: f64,
    v3437: f64,
    v3440: f64,
    v3443: f64,
    v3446: f64,
    v3449: f64,
    v3450: f64,
    v3451: f64,
    v3452: f64,
    v3454: f64,
    v3458: f64,
    v3461: f64,
    v3495: f64,
    v3496: f64,
    v3555: f64,
    v3556: f64,
    v3689: f64,
    v3690: f64,
    v3691: f64,
    v3746: f64,
    v3747: f64,
    v3748: f64,
    v3761: f64,
    v3762: f64,
    v3763: f64,
    v3784: f64,
    v3785: f64,
    v3786: f64,
    v3787: f64,
    v3788: f64,
    v3805: f64,
    v3806: f64,
    v3807: f64,
    v3808: f64,
    v3809: f64,
    v4267: f64,
    v4268: f64,
    v4269: f64,
    v4270: f64,
    v4283: f64,
    v4284: f64,
    v4285: f64,
    v4286: f64,
    v4287: f64,
    v4288: f64,
    v4289: f64,
    v4290: f64,
    v4415: f64,
    v4416: f64,
    v4417: f64,
    v4418: f64,
    v4419: f64,
    v4420: f64,
    v4421: f64,
    v4422: f64,
    v4727: f64,
    v4728: f64,
    v4729: f64,
    v4730: f64,
    v6426: f64,
    v6427: f64,
    v6428: f64,
    v6429: f64,
    v6430: f64,
    v6431: f64,
    v6597: f64,
    v6598: f64,
    v6599: f64,
    v6600: f64,
    v6601: f64,
    v6602: f64,
    v6616: f64,
    v6617: f64,
    v6622: f64,
    v6623: f64,
    v6624: f64,
    v6625: f64,
    v6626: f64,
    v6627: f64,
    v6640: f64,
    v6641: f64,
    v6642: f64,
    v6643: f64,
    v6644: f64,
    v6645: f64,
    v6698: f64,
    v6699: f64,
    v6700: f64,
    v6701: f64,
    v6702: f64,
    v6703: f64,
    v6704: f64,
    v6705: f64,
    v6745: f64,
    v6746: f64,
    v6747: f64,
    v6748: f64,
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
        let v48=0.1;
        let v160=3.0;
        let v409=1e-6;
        let v413=0.5;
        let v425=4.0;
        let v451=6.0;
        let v696=ctx.node_voltage(nodes[5]);
        let v697=ctx.node_voltage(nodes[6]);
        let v699=(self.scalar_static_f64[0]*(v696-v697));
        let v700=ctx.node_voltage(nodes[7]);
        let v702=(self.scalar_static_f64[0]*(v696-v700));
        let v703=ctx.node_voltage(nodes[3]);
        let v705=(self.scalar_static_f64[0]*(v696-v703));
        let v706=ctx.node_voltage(nodes[4]);
        let v708=(self.scalar_static_f64[0]*(v706-v703));
        let v710=(self.scalar_static_f64[0]*(v706-v696));
        let v712=(self.scalar_static_f64[0]*(v697-v700));
        let v713=ctx.node_voltage(nodes[2]);
        let v716=ctx.node_voltage(nodes[1]);
        let v718=(self.scalar_static_f64[0]*(v716-v706));
        let v723=(self.scalar_static_f64[0]*(v716-ctx.node_voltage(nodes[0])));
        let v724=ctx.node_voltage(nodes[9]);
        let v726=(self.scalar_static_f64[0]*(v724-v697));
        let v729=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[8])-v724));
        let v732=(((v702+v710)-v712)-v726);
        let v736=((v732+(v718+(-v723)))-v729);
        let v737=(v723+v736);
        let v738=(self.scalar_static_f64[381]*v702);
        let v741=(if (v738<self.scalar_static_f64[198]){v1}else{v3});
        let v742=(v738).exp();
        let v744=(!(v741!=0.0));
        let v746=(if v744{self.scalar_static_f64[199]}else{v3});
        let v751=(self.scalar_static_f64[381]*v705);
        let v752=(v751/self.scalar_static_f64[588]);
        let v754=(if (v752<self.scalar_static_f64[198]){v1}else{v3});
        let v755=(v752).exp();
        let v757=(!(v754!=0.0));
        let v758=(if v757{self.scalar_static_f64[199]}else{v746});
        let v762=(if v757{(v758*(v1+(v752-self.scalar_static_f64[198])))}else{(if (v754!=0.0){v755}else{v3})});
        let v763=(self.scalar_static_f64[381]*v732);
        let v765=(if (v763<self.scalar_static_f64[198]){v1}else{v3});
        let v766=(v763).exp();
        let v768=(!(v765!=0.0));
        let v769=(if v768{self.scalar_static_f64[199]}else{v758});
        let v773=(if v768{(v769*(v1+(v763-self.scalar_static_f64[198])))}else{(if (v765!=0.0){v766}else{v3})});
        let v774=(self.scalar_static_f64[381]*v710);
        let v776=(if (v774<self.scalar_static_f64[198]){v1}else{v3});
        let v779=(!(v776!=0.0));
        let v780=(if v779{self.scalar_static_f64[199]}else{v769});
        let v785=(self.scalar_static_f64[381]*v737);
        let v787=(if (v785<self.scalar_static_f64[198]){v1}else{v3});
        let v788=(v785).exp();
        let v790=(!(v787!=0.0));
        let v791=(if v790{self.scalar_static_f64[199]}else{v780});
        let v795=(if v790{(v791*(v1+(v785-self.scalar_static_f64[198])))}else{(if (v787!=0.0){v788}else{v3})});
        let v797=(self.scalar_static_f64[381]*(v737-self.scalar_static_f64[469]));
        let v799=(if (v797<self.scalar_static_f64[198]){v1}else{v3});
        let v800=(v797).exp();
        let v802=(!(v799!=0.0));
        let v803=(if v802{self.scalar_static_f64[199]}else{v791});
        let v809=(self.scalar_static_f64[381]*(v732-self.scalar_static_f64[469]));
        let v811=(if (v809<self.scalar_static_f64[198]){v1}else{v3});
        let v812=(v809).exp();
        let v814=(!(v811!=0.0));
        let v815=(if v814{self.scalar_static_f64[199]}else{v803});
        let v821=(self.scalar_static_f64[381]*(v702-self.scalar_static_f64[469]));
        let v823=(if (v821<self.scalar_static_f64[198]){v1}else{v3});
        let v824=(v821).exp();
        let v826=(!(v823!=0.0));
        let v827=(if v826{self.scalar_static_f64[199]}else{v815});
        let v831=(if v826{(v827*(v1+(v821-self.scalar_static_f64[198])))}else{(if (v823!=0.0){v824}else{v3})});
        let v833=(self.scalar_static_f64[381]*(v699-self.scalar_static_f64[469]));
        let v835=(if (v833<self.scalar_static_f64[198]){v1}else{v3});
        let v836=(v833).exp();
        let v838=(!(v835!=0.0));
        let v839=(if v838{self.scalar_static_f64[199]}else{v827});
        let v843=(if v838{(v839*(v1+(v833-self.scalar_static_f64[198])))}else{(if (v835!=0.0){v836}else{v3})});
        let v846=((v1+(v425*v831))).sqrt();
        let v849=((v1+(v425*v843))).sqrt();
        let v850=(v33*v843);
        let v851=(v1+v849);
        let v852=(v850/v851);
        let v855=(if (v852<self.scalar_static_f64[200]){v1}else{v3});
        let v856=(if (v855!=0.0){self.scalar_static_f64[200]}else{v852});
        let v858=(v1+v846);
        let v859=(v858/v851);
        let v862=(self.scalar_static_f64[380]*((v846-v849)-(v859).ln()));
        let v864=((v712+v862)/self.scalar_static_f64[564]);
        let v866=(if (v864>v3){v1}else{v3});
        let v867=100.0;
        let v869=(if (v699<v867){v1}else{v3});
        let v870=((v866!=0.0)&&(v869!=0.0));
        let v873=((v866!=0.0)&&(!(v869!=0.0)));
        let v875=(v1+(v699-v867));
        let v881=(self.scalar_static_f64[564]*(v413*v864));
        let v883=(v1+(self.scalar_static_f64[381]*v881));
        let v888=(if (v866!=0.0){((self.scalar_static_f64[469]+(self.scalar_static_f64[795]*(v883).ln()))-(if v873{(v867+(v875).ln())}else{(if v870{v699}else{v3})}))}else{v3});
        let v891=(if (v866!=0.0){self.scalar_static_f64[796]}else{v3});
        let v893=(if (v866!=0.0){(v891*v891)}else{v409});
        let v897=(if (v888<v3){v1}else{v3});
        let v898=((v866!=0.0)&&(v897!=0.0));
        let v899=(v413*v893);
        let v901=((v893+(if (v866!=0.0){(v888*v888)}else{self.scalar_static_f64[616]}))).sqrt();
        let v902=(v901-v888);
        let v906=((v866!=0.0)&&(!(v897!=0.0)));
        let v909=(if v906{(v413*(v888+v901))}else{(if v898{(v899/v902)}else{v3})});
        let v913=(v909+self.scalar_static_f64[203]);
        let v914=(v909*v913);
        let v917=(self.scalar_static_f64[202]*(v909+self.scalar_static_f64[797]));
        let v919=(if (v866!=0.0){(v914/v917)}else{v3});
        let v921=(if (v866!=0.0){(v864/v919)}else{v3});
        let v925=(if (v866!=0.0){((v921-v1)/self.scalar_static_f64[204])}else{self.scalar_static_f64[595]});
        let v927=(if (v921<v1){v1}else{v3});
        let v928=((v866!=0.0)&&(v927!=0.0));
        let v929=(v925).exp();
        let v930=(v1+v929);
        let v936=((v866!=0.0)&&(!(v927!=0.0)));
        let v938=((-v925)).exp();
        let v939=(v1+v938);
        let v952=(if (v866!=0.0){((if v936{(v921+(self.scalar_static_f64[204]*(v939).ln()))}else{(if v928{(v1+(self.scalar_static_f64[204]*(v930).ln()))}else{v3})})/self.scalar_static_f64[210])}else{v3});
        let v954=(if (v866!=0.0){(v909/self.scalar_static_f64[203])}else{v3});
        let v955=(v425*v952);
        let v956=(v954*v955);
        let v957=(v1+v954);
        let v960=((v1+(v956*v957))).sqrt();
        let v961=(v1+v960);
        let v962=(v33*v952);
        let v963=(v957*v962);
        let v965=(if (v866!=0.0){(v961/v963)}else{v3});
        let v967=(v856*v965);
        let v968=((v1-v965)+v967);
        let v969=(v1+v967);
        let v971=(if (v866!=0.0){(v968/v969)}else{v3});
        let v974=(if (v866!=0.0){(self.scalar_static_f64[381]*(v881*v971))}else{v3});
        let v977=(v1+(v856+v974));
        let v980=(if (v866!=0.0){((v33*v974)+(v856*v977))}else{v3});
        let v983=(if (v866!=0.0){(v413*(v974-v1))}else{v3});
        let v986=(if (v866!=0.0){(v980+(v983*v983))}else{v3});
        let v988=(if (v974>=v1){v1}else{v3});
        let v989=((v866!=0.0)&&(v988!=0.0));
        let v990=(v986).sqrt();
        let v994=((v866!=0.0)&&(!(v988!=0.0)));
        let v995=(v990-v983);
        let v997=(if v994{(v980/v995)}else{(if v989{(v983+v990)}else{v3})});
        let v1001=((v866!=0.0)&&((if (v997<self.scalar_static_f64[211]){v1}else{v3})!=0.0));
        let v1002=(if v1001{self.scalar_static_f64[211]}else{v997});
        let v1003=(v1+v1002);
        let v1012=(if (v866!=0.0){(self.scalar_static_f64[212]*(v864-self.scalar_static_f64[201]))}else{v3});
        let v1019=(((if (v866!=0.0){(v864*self.scalar_static_f64[801])}else{v3})+(v1012*v1012))).sqrt();
        let v1029=((v866!=0.0)&&self.scalar_static_bool[20]);
        let v1030=(v33*v864);
        let v1031=(v864+v919);
        let v1036=(v864*self.scalar_static_f64[201]);
        let v1037=(v864+self.scalar_static_f64[201]);
        let v1042=(!(v866!=0.0));
        let v1043=(v33*v831);
        let v1046=(if v1042{(if v744{(v746*(v1+(v738-self.scalar_static_f64[198])))}else{(if (v741!=0.0){v742}else{v3})})}else{(if (v866!=0.0){((v1002*v1003)*self.scalar_static_f64[799])}else{v3})});
        let v1058=(if (((v712).abs()<self.scalar_static_f64[803])||((v862).abs()<(self.scalar_static_f64[804]*(v846+v849)))){v1}else{v3});
        let v1059=(v1042&&(v1058!=0.0));
        let v1060=(v856+(if v1042{(v1043/v858)}else{v1002}));
        let v1062=(if v1059{(v413*v1060)}else{v3});
        let v1063=(v1+v1062);
        let v1067=(v1042&&(!(v1058!=0.0)));
        let v1069=((v702+v862)-v699);
        let v1071=(if v1067{(v862/v1069)}else{(if v1059{(v1062/v1063)}else{v971})});
        let v1073=(if v1042{self.scalar_static_f64[802]}else{(if v1029{(self.scalar_static_f64[507]*(v48+(v1030/v1031)))}else{(if ((v866!=0.0)&&(self.scalar_static_f64[214]!=0.0)){self.scalar_static_f64[802]}else{v3})})});
        let v1074=(if v1042{v864}else{(if (v866!=0.0){(v1036/v1037)}else{v3})});
        let v1077=(if v1042{(v1-(v1074/self.scalar_static_f64[201]))}else{(if (v866!=0.0){(self.scalar_static_f64[201]/v1037)}else{v3})});
        let v1084=((v705-self.scalar_static_f64[805])/self.scalar_static_f64[806]);
        let v1086=(if (v705<self.scalar_static_f64[805]){v1}else{v3});
        let v1087=(v1084).exp();
        let v1088=(v1+v1087);
        let v1093=(!(v1086!=0.0));
        let v1095=((-v1084)).exp();
        let v1096=(v1+v1095);
        let v1100=(if v1093{(self.scalar_static_f64[805]-(self.scalar_static_f64[806]*(v1096).ln()))}else{(if (v1086!=0.0){(v705-(self.scalar_static_f64[806]*(v1088).ln()))}else{v3})});
        let v1102=(v1-(self.scalar_static_f64[528]*v1100));
        let v1104=f64::powf(v1102,self.scalar_static_f64[218]);
        let v1110=((self.scalar_static_f64[807]*(v1-v1104))+(v160*(v705-v1100)));
        let v1123=(if self.scalar_static_bool[26]{v702}else{(if self.scalar_static_bool[24]{(v699+(if v1042{v712}else{(if (v866!=0.0){(v1012+v1019)}else{v3})}))}else{(if (self.scalar_static_f64[220]!=0.0){v699}else{v3})})});
        let v1131=(v1123-self.scalar_static_f64[813]);
        let v1132=(v1131/v1073);
        let v1134=(if (v1123<self.scalar_static_f64[813]){v1}else{v3});
        let v1135=(v1132).exp();
        let v1136=(v1+v1135);
        let v1137=(v1136).ln();
        let v1141=(!(v1134!=0.0));
        let v1143=((-v1132)).exp();
        let v1144=(v1+v1143);
        let v1145=(v1144).ln();
        let v1148=(if v1141{(self.scalar_static_f64[813]-(v1073*v1145))}else{(if (v1134!=0.0){(v1123-(v1073*v1137))}else{v3})});
        let v1150=f64::powf(v1077,self.scalar_static_f64[223]);
        let v1154=(v1-(v1148/self.scalar_static_f64[507]));
        let v1155=f64::powf(v1154,self.scalar_static_f64[224]);
        let v1159=(self.scalar_static_f64[810]*v1150);
        let v1160=(v1123-v1148);
        let v1165=((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(v1-(v1150*v1155)))+(v1159*v1160)))+(self.scalar_static_f64[541]*v699));
        let v1168=(v762*self.scalar_static_f64[816]);
        let v1170=((v1+v1168)).sqrt();
        let v1171=(v1+v1170);
        let v1172=(v1168/v1171);
        let v1174=f64::powf(v1046,self.scalar_static_f64[817]);
        let v1175=(self.scalar_static_f64[816]*v1174);
        let v1177=((v1+v1175)).sqrt();
        let v1178=(v1+v1177);
        let v1179=(v1175/v1178);
        let v1183=(v1+(v1110/self.scalar_static_f64[750]));
        let v1184=(v1165/self.scalar_static_f64[748]);
        let v1185=(v1183+v1184);
        let v1196=((if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*v1183))}else{v3})).exp();
        let v1197=((if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*((-v1165)/self.scalar_static_f64[748])))}else{v3})).exp();
        let v1203=(if self.scalar_static_bool[28]{((v1196-v1197)/self.scalar_static_f64[820])}else{(if (self.scalar_static_f64[225]!=0.0){v1185}else{v3})});
        let v1204=0.010000000000000002;
        let v1205=(v1203*v1203);
        let v1207=(if (v1203<v3){v1}else{v3});
        let v1208=0.005000000000000001;
        let v1210=((v1204+v1205)).sqrt();
        let v1211=(v1210-v1203);
        let v1214=(!(v1207!=0.0));
        let v1217=(if v1214{(v413*(v1203+v1210))}else{(if (v1207!=0.0){(v1208/v1211)}else{v3})});
        let v1220=(v1+(v413*(v1172+v1179)));
        let v1221=(v1217*v1220);
        let v1224=(v1174*self.scalar_static_f64[821]);
        let v1225=(self.scalar_static_f64[633]*v762);
        let v1226=(v1225-v1224);
        let v1227=(v1226/v1221);
        let v1228=0.0001;
        let v1229=(v705/v1228);
        let v1230=(v705<v3);
        let v1231=(if v1230{v1}else{v3});
        let v1232=(v1229).exp();
        let v1233=(v1+v1232);
        let v1237=(!(v1231!=0.0));
        let v1239=((-v1229)).exp();
        let v1240=(v1+v1239);
        let v1244=(if v1237{(v705+(v1228*(v1240).ln()))}else{(if (v1231!=0.0){(v1228*(v1233).ln())}else{v3})});
        let v1246=(v1244/self.scalar_static_f64[227]);
        let v1248=(if (v1246<self.scalar_static_f64[198]){v1}else{v3});
        let v1251=(!(v1248!=0.0));
        let v1252=(if v1251{self.scalar_static_f64[199]}else{v839});
        let v1261=((v705-self.scalar_static_f64[228])/v32);
        let v1283=(v751/self.scalar_static_f64[143]);
        let v1285=(if (v1283<self.scalar_static_f64[198]){v1}else{v3});
        let v1286=(v1283).exp();
        let v1288=(!(v1285!=0.0));
        let v1289=(if v1288{self.scalar_static_f64[199]}else{v1252});
        let v1293=(if v1288{(v1289*(v1+(v1283-self.scalar_static_f64[198])))}else{(if (v1285!=0.0){v1286}else{v1244})});
        let v1295=(self.scalar_static_f64[381]*(v705-self.scalar_static_f64[527]));
        let v1297=(if (v1295<self.scalar_static_f64[198]){v1}else{v3});
        let v1302=((self.scalar_static_f64[149]!=0.0)&&(!(v1297!=0.0)));
        let v1303=(if v1302{self.scalar_static_f64[199]}else{v1289});
        let v1310=((v1227/self.scalar_static_f64[633])-1000.0);
        let v1311=40.0;
        let v1313=(if (v1310<v1311){v1}else{v3});
        let v1318=((self.scalar_static_f64[149]!=0.0)&&(!(v1313!=0.0)));
        let v1320=(if v1318{2.3538526683702e17}else{v1303});
        let v1361=(self.scalar_static_f64[381]*v708);
        let v1362=(v1361/self.scalar_static_f64[147]);
        let v1364=(if (v1362<self.scalar_static_f64[198]){v1}else{v3});
        let v1365=(v1362).exp();
        let v1367=(!(v1364!=0.0));
        let v1368=(if v1367{self.scalar_static_f64[199]}else{v1320});
        let v1372=(if v1367{(v1368*(v1+(v1362-self.scalar_static_f64[198])))}else{(if (v1364!=0.0){v1365}else{v1293})});
        let v1374=(self.scalar_static_f64[381]*(v708-self.scalar_static_f64[527]));
        let v1376=(if (v1374<self.scalar_static_f64[198]){v1}else{v3});
        let v1381=((self.scalar_static_f64[149]!=0.0)&&(!(v1376!=0.0)));
        let v1382=(if v1381{self.scalar_static_f64[199]}else{v1368});
        let v1399=(v751/self.scalar_static_f64[130]);
        let v1401=(if (v1399<self.scalar_static_f64[198]){v1}else{v3});
        let v1402=(v1399).exp();
        let v1404=(!(v1401!=0.0));
        let v1405=(if v1404{self.scalar_static_f64[199]}else{v1382});
        let v1409=(if v1404{(v1405*(v1+(v1399-self.scalar_static_f64[198])))}else{(if (v1401!=0.0){v1402}else{v1372})});
        let v1412=(v1361/self.scalar_static_f64[165]);
        let v1414=(if (v1412<self.scalar_static_f64[198]){v1}else{v3});
        let v1415=(v1412).exp();
        let v1417=(!(v1414!=0.0));
        let v1418=(if v1417{self.scalar_static_f64[199]}else{v1405});
        let v1422=(if v1417{(v1418*(v1+(v1412-self.scalar_static_f64[198])))}else{(if (v1414!=0.0){v1415}else{v1409})});
        let v1425=(v763/self.scalar_static_f64[136]);
        let v1427=(if (v1425<self.scalar_static_f64[198]){v1}else{v3});
        let v1428=(v1425).exp();
        let v1430=(!(v1427!=0.0));
        let v1431=(if v1430{self.scalar_static_f64[199]}else{v1418});
        let v1435=(if v1430{(v1431*(v1+(v1425-self.scalar_static_f64[198])))}else{(if (v1427!=0.0){v1428}else{v1422})});
        let v1438=(v1361/self.scalar_static_f64[169]);
        let v1440=(if (v1438<self.scalar_static_f64[198]){v1}else{v3});
        let v1441=(v1438).exp();
        let v1443=(!(v1440!=0.0));
        let v1444=(if v1443{self.scalar_static_f64[199]}else{v1431});
        let v1448=(if v1443{(v1444*(v1+(v1438-self.scalar_static_f64[198])))}else{(if (v1440!=0.0){v1441}else{v1435})});
        let v1455=(if (v1230&&self.scalar_static_bool[36]){v1}else{v3});
        let v1456=(v33*v1104);
        let v1459=(self.scalar_static_f64[715]*(v1-(self.scalar_static_f64[20]/v1456)));
        let v1461=(if (v1459<self.scalar_static_f64[198]){v1}else{v3});
        let v1466=((v1455!=0.0)&&(!(v1461!=0.0)));
        let v1467=(if v1466{self.scalar_static_f64[199]}else{v1444});
        let v1473=(if (v1455!=0.0){(self.scalar_static_f64[528]*v705)}else{self.scalar_static_f64[746]});
        let v1475=1e-30;
        let v1477=(((v1473*v1473)+v1475)).sqrt();
        let v1480=f64::powf(v1477,self.scalar_static_f64[233]);
        let v1488=(v451*v1473);
        let v1489=(v1473*v1488);
        let v1490=(v1473+self.scalar_static_f64[236]);
        let v1492=((self.scalar_static_f64[18]*(self.scalar_static_f64[235]-((v160*v1473)*self.scalar_static_f64[236])))-(v1489*v1490));
        let v1494=0.16666666666666666;
        let v1498=(self.scalar_static_f64[715]*(self.scalar_static_f64[20]*v705));
        let v1499=(self.scalar_static_f64[405]*(if (v1455!=0.0){((v1480*v1492)*v1494)}else{v3}));
        let v1501=(if (v1455!=0.0){(v1498/v1499)}else{v1473});
        let v1502=-0.001;
        let v1504=(if (v1501<v1502){v1}else{v3});
        let v1506=(if (v1501<self.scalar_static_f64[198]){v1}else{v3});
        let v1507=((v1455!=0.0)&&(v1504!=0.0));
        let v1512=(v1507&&(!(v1506!=0.0)));
        let v1513=(if v1512{self.scalar_static_f64[199]}else{v1467});
        let v1551=(if (self.scalar_static_bool[39]&&(v699<v3)){v1}else{v3});
        let v1552=(self.scalar_static_f64[529]*v699);
        let v1553=(v1-v1552);
        let v1555=(if (v1551!=0.0){f64::powf(v1553,self.scalar_static_f64[224])}else{v3});
        let v1556=(v33*v1555);
        let v1559=(self.scalar_static_f64[735]*(v1-(self.scalar_static_f64[52]/v1556)));
        let v1561=(if (v1559<self.scalar_static_f64[198]){v1}else{v3});
        let v1566=((v1551!=0.0)&&(!(v1561!=0.0)));
        let v1567=(if v1566{self.scalar_static_f64[199]}else{v1513});
        let v1572=(if (v1551!=0.0){v1552}else{self.scalar_static_f64[726]});
        let v1575=((v1475+(v1572*v1572))).sqrt();
        let v1577=f64::powf(v1575,self.scalar_static_f64[237]);
        let v1585=(v451*v1572);
        let v1586=(v1572*v1585);
        let v1587=(v1572+self.scalar_static_f64[240]);
        let v1589=((self.scalar_static_f64[50]*(self.scalar_static_f64[239]-((v160*v1572)*self.scalar_static_f64[240])))-(v1586*v1587));
        let v1594=(self.scalar_static_f64[735]*(self.scalar_static_f64[52]*v699));
        let v1595=(self.scalar_static_f64[426]*(if (v1551!=0.0){(v1494*(v1577*v1589))}else{v3}));
        let v1597=(if (v1551!=0.0){(v1594/v1595)}else{v1572});
        let v1599=(if (v1597<v1502){v1}else{v3});
        let v1601=(if (v1597<self.scalar_static_f64[198]){v1}else{v3});
        let v1602=((v1551!=0.0)&&(v1599!=0.0));
        let v1607=(v1602&&(!(v1601!=0.0)));
        let v1608=(if v1607{self.scalar_static_f64[199]}else{v1567});
        let v1639=(v773*self.scalar_static_f64[816]);
        let v1640=(v425*(if v814{(v815*(v1+(v809-self.scalar_static_f64[198])))}else{(if (v811!=0.0){v812}else{v3})}));
        let v1641=(v1639-self.scalar_static_f64[816]);
        let v1643=((v1+v1639)).sqrt();
        let v1644=(v1+v1643);
        let v1647=((v1+v1640)).sqrt();
        let v1648=(v1+v1647);
        let v1670=(self.scalar_static_f64[829]*(v795-v1));
        let v1673=((v1+(v795*self.scalar_static_f64[828]))).sqrt();
        let v1674=(v1+v1673);
        let v1676=(if (self.scalar_static_f64[242]!=0.0){(v1670/v1674)}else{v3});
        let v1689=(if self.scalar_static_bool[44]{(v737-self.scalar_static_f64[837])}else{v3});
        let v1693=(if self.scalar_static_bool[44]{(v1689*v1689)}else{v1205});
        let v1695=(if (v1689<v3){v1}else{v3});
        let v1696=(self.scalar_static_bool[44]&&(v1695!=0.0));
        let v1699=((self.scalar_static_f64[245]+v1693)).sqrt();
        let v1700=(v1699-v1689);
        let v1704=(self.scalar_static_bool[44]&&(!(v1695!=0.0)));
        let v1707=(if v1704{(v413*(v1689+v1699))}else{(if v1696{(self.scalar_static_f64[246]/v1700)}else{v3})});
        let v1710=(v1707+(self.scalar_static_f64[832]+(self.scalar_static_f64[557]*v1676)));
        let v1715=(if self.scalar_static_bool[46]{v1}else{(if self.scalar_static_bool[44]{(v1707/v1710)}else{v1})});
        let v1778=(if (v1185<v3){v1}else{v3});
        let v1780=((v1204+(v1185*v1185))).sqrt();
        let v1781=(v1780-v1185);
        let v1784=(!(v1778!=0.0));
        let v1787=(if v1784{(v413*(v1185+v1780))}else{(if (v1778!=0.0){(v1208/v1781)}else{v3})});
        let v1799=(if (v1227>v3){v1}else{v3});
        let v1805=(if (v699<self.scalar_static_f64[268]){v1}else{v3});
        let v1808=((-v1227)/self.scalar_static_f64[269]);
        let v1810=(if (v1808<self.scalar_static_f64[198]){v1}else{v3});
        let v1812=((v1805!=0.0)&&((v1799!=0.0)&&(self.scalar_static_f64[267]!=0.0)));
        let v1813=((v1810!=0.0)&&v1812);
        let v1814=(v1808).exp();
        let v1817=(v1812&&(!(v1810!=0.0)));
        let v1818=(if v1817{self.scalar_static_f64[199]}else{v1608});
        let v1822=(if v1817{(v1818*(v1+(v1808-self.scalar_static_f64[198])))}else{(if v1813{v1814}else{v3})});
        let v1823=(self.scalar_static_f64[268]-v699);
        let v1825=(if v1812{(v1822*v1823)}else{v3});
        let v1829=(self.scalar_static_f64[838]*f64::powf(v1825,self.scalar_static_f64[270]));
        let v1831=(if (v1829<self.scalar_static_f64[198]){v1}else{v3});
        let v1836=(v1812&&(!(v1831!=0.0)));
        let v1837=(if v1836{self.scalar_static_f64[199]}else{v1818});
        let v1852=((v1799!=0.0)&&self.scalar_static_bool[51]);
        let v1959=((v1805!=0.0)&&((self.scalar_static_f64[285]!=0.0)&&(v1852&&self.scalar_static_bool[55])));
        let v1960=f64::powf(v1823,self.scalar_static_f64[270]);
        let v1962=(v1227+self.scalar_static_f64[286]);
        let v1964=(v1-(v1227/v1962));
        let v1966=f64::powf(v1964,self.scalar_static_f64[287]);
        let v1968=(if v1959{(v1960*v1966)}else{v3});
        let v1969=((self.scalar_static_f64[279]!=0.0)&&v1959);
        let v1971=(self.scalar_static_bool[53]&&v1959);
        let v1975=(if v1971{((v1227-self.scalar_static_f64[288])/self.scalar_static_f64[286])}else{v3});
        let v1979=(if v1971{((v1975-v1)/self.scalar_static_f64[289])}else{v1261});
        let v1981=(if (v1975<v1){v1}else{v3});
        let v1982=(v1971&&(v1981!=0.0));
        let v1983=(v1979).exp();
        let v1984=(v1+v1983);
        let v1990=(v1971&&(!(v1981!=0.0)));
        let v1992=((-v1979)).exp();
        let v1993=(v1+v1992);
        let v1997=(if v1990{(v1975+(self.scalar_static_f64[289]*(v1993).ln()))}else{(if v1982{(v1+(self.scalar_static_f64[289]*(v1984).ln()))}else{v3})});
        let v1999=f64::powf(v1997,self.scalar_static_f64[290]);
        let v2002=(self.scalar_static_f64[838]*(if v1971{(v1968*v1999)}else{(if v1969{v1968}else{v3})}));
        let v2004=(if (v2002<self.scalar_static_f64[198]){v1}else{v3});
        let v2009=(v1959&&(!(v2004!=0.0)));
        let v2010=(if v2009{self.scalar_static_f64[199]}else{v1837});
        let v2072=((v708-self.scalar_static_f64[805])/self.scalar_static_f64[806]);
        let v2074=(if (v708<self.scalar_static_f64[805]){v1}else{v3});
        let v2075=(v2072).exp();
        let v2076=(v1+v2075);
        let v2081=(!(v2074!=0.0));
        let v2083=((-v2072)).exp();
        let v2084=(v1+v2083);
        let v2088=(if v2081{(self.scalar_static_f64[805]-(self.scalar_static_f64[806]*(v2084).ln()))}else{(if (v2074!=0.0){(v708-(self.scalar_static_f64[806]*(v2076).ln()))}else{v3})});
        let v2091=(v1-(self.scalar_static_f64[528]*v2088));
        let v2104=(v1172*self.scalar_static_f64[846]);
        let v2105=(v1787*v2104);
        let v2106=(v1179*self.scalar_static_f64[846]);
        let v2107=(v1787*v2106);
        let v2109=((v732-self.scalar_static_f64[813])/self.scalar_static_f64[802]);
        let v2111=(if (v732<self.scalar_static_f64[813]){v1}else{v3});
        let v2112=(v2109).exp();
        let v2113=(v1+v2112);
        let v2118=(!(v2111!=0.0));
        let v2120=((-v2109)).exp();
        let v2121=(v1+v2120);
        let v2125=(if v2118{(self.scalar_static_f64[813]-(self.scalar_static_f64[802]*(v2121).ln()))}else{(if (v2111!=0.0){(v732-(self.scalar_static_f64[802]*(v2113).ln()))}else{v3})});
        let v2127=(v1-(v2125/self.scalar_static_f64[507]));
        let v2142=((v737-self.scalar_static_f64[813])/self.scalar_static_f64[802]);
        let v2144=(if (v737<self.scalar_static_f64[813]){v1}else{v3});
        let v2145=(v2142).exp();
        let v2146=(v1+v2145);
        let v2151=(!(v2144!=0.0));
        let v2153=((-v2142)).exp();
        let v2154=(v1+v2153);
        let v2158=(if v2151{(self.scalar_static_f64[813]-(self.scalar_static_f64[802]*(v2154).ln()))}else{(if (v2144!=0.0){(v737-(self.scalar_static_f64[802]*(v2146).ln()))}else{v3})});
        let v2160=(v1-(v2158/self.scalar_static_f64[507]));
        let v2180=(v705/self.scalar_static_f64[851]);
        let v2182=(if (v2180<self.scalar_static_f64[198]){v1}else{v3});
        let v2183=(v2180).exp();
        let v2185=(!(v2182!=0.0));
        let v2186=(if v2185{self.scalar_static_f64[199]}else{v2010});
        let v2191=(self.scalar_static_f64[850]*(if v2185{(v2186*(v1+(v2180-self.scalar_static_f64[198])))}else{(if (v2182!=0.0){v2183}else{v1448})}));
        let v2196=(v1071*self.scalar_static_f64[855]);
        let v2197=(v33+v1060);
        let v2212=(self.scalar_static_f64[381]*((v732-self.scalar_static_f64[488])/self.scalar_static_f64[301]));
        let v2214=(if (v2212<self.scalar_static_f64[198]){v1}else{v3});
        let v2216=((v2214!=0.0)&&self.scalar_static_bool[60]);
        let v2217=(v2212).exp();
        let v2220=(self.scalar_static_bool[60]&&(!(v2214!=0.0)));
        let v2221=(if v2220{self.scalar_static_f64[199]}else{v2186});
        let v2227=(v773*self.scalar_static_f64[857]);
        let v2230=((v1+(v425*(if v2220{(v2221*(v1+(v2212-self.scalar_static_f64[198])))}else{(if v2216{v2217}else{v3})})))).sqrt();
        let v2231=(v1+v2230);
        let v2233=(if self.scalar_static_bool[60]{(v2227/v2231)}else{(if (self.scalar_static_f64[300]!=0.0){((self.scalar_static_f64[856]*(((v1641/v1644)*self.scalar_static_f64[845])+((v1640/v1648)*self.scalar_static_f64[854])))/self.scalar_static_f64[763])}else{v3})});
        let v2242=(if self.scalar_static_bool[64]{(v795*self.scalar_static_f64[816])}else{v3});
        let v2243=(v2242-self.scalar_static_f64[816]);
        let v2245=((v1+v2242)).sqrt();
        let v2246=(v1+v2245);
        let v2250=(if self.scalar_static_bool[64]{(v425*(if v802{(v803*(v1+(v797-self.scalar_static_f64[198])))}else{(if (v799!=0.0){v800}else{v3})}))}else{v3});
        let v2252=((v1+v2250)).sqrt();
        let v2253=(v1+v2252);
        let v2265=(self.scalar_static_f64[381]*(v737-self.scalar_static_f64[488]));
        let v2267=(if (v2265<self.scalar_static_f64[198]){v1}else{v3});
        let v2269=((v2267!=0.0)&&self.scalar_static_bool[65]);
        let v2270=(v2265).exp();
        let v2273=(self.scalar_static_bool[65]&&(!(v2267!=0.0)));
        let v2274=(if v2273{self.scalar_static_f64[199]}else{v2221});
        let v2280=(v795*self.scalar_static_f64[859]);
        let v2283=((v1+(v425*(if v2273{(v2274*(v1+(v2265-self.scalar_static_f64[198])))}else{(if v2269{v2270}else{v3})})))).sqrt();
        let v2284=(v1+v2283);
        let v2286=(if self.scalar_static_bool[65]{(v2280/v2284)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[858]*((self.scalar_static_f64[845]*(if self.scalar_static_bool[64]{(v2243/v2246)}else{v3}))+(self.scalar_static_f64[854]*(if self.scalar_static_bool[64]{(v2250/v2253)}else{v3}))))/self.scalar_static_f64[763])}else{v3})});
        let v2295=(if (self.scalar_static_f64[305]!=0.0){(f64::powf(v1102,self.scalar_static_f64[306])-v160)}else{v3});
        let v2296=(if (self.scalar_static_f64[305]!=0.0){v1084}else{v3});
        let v2298=(if (v2296<v3){v1}else{v3});
        let v2299=((self.scalar_static_f64[305]!=0.0)&&(v2298!=0.0));
        let v2300=(v2296).exp();
        let v2301=(v1+v2300);
        let v2305=((self.scalar_static_f64[305]!=0.0)&&(!(v2298!=0.0)));
        let v2307=((-v2296)).exp();
        let v2308=(v1+v2307);
        let v2310=(if v2305{(v2307/v2308)}else{(if v2299{(v1/v2301)}else{v3})});
        let v2317=((self.scalar_static_f64[381]*v1168)/self.scalar_static_f64[588]);
        let v2318=(v413/v1170);
        let v2320=(if (self.scalar_static_f64[305]!=0.0){(v2317*v2318)}else{v3});
        let v2321=(v1787*self.scalar_static_f64[846]);
        let v2326=(v710*0.2);
        let v2328=((if (self.scalar_static_f64[305]!=0.0){(v2191/self.scalar_static_f64[851])}else{v3})+((if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[842]*(if (self.scalar_static_f64[305]!=0.0){(v160+(v2295*v2310))}else{v3}))}else{v3})+(if (self.scalar_static_f64[305]!=0.0){(v2320*v2321)}else{v3})));
        let v2337=(if (self.scalar_static_f64[305]!=0.0){(v2105+(v2191*self.scalar_static_f64[307]))}else{v3});
        let v2346=(if self.scalar_static_bool[67]{v2105}else{(if (self.scalar_static_f64[305]!=0.0){(v2337*self.scalar_static_f64[310])}else{v3})});
        let v2347=(if self.scalar_static_bool[67]{v2107}else{(if (self.scalar_static_f64[305]!=0.0){(v2107+(v2337*self.scalar_static_f64[309]))}else{v3})});
        let v2349=(v1224+v1225);
        let v2350=(v2349/v1221);
        let v2360=(if (v2350>v3){v1}else{v3});
        let v2361=(v2346+v2347);
        let v2364=(!(v2360!=0.0));
        let v2365=(self.scalar_static_f64[759]*v1787);
        let v2367=(if v2364{(v1221*v2365)}else{(if (v2360!=0.0){(v2361/v2350)}else{v3})});
        let v2382=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(v2367*self.scalar_static_f64[316])}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v2367)}else{v3})})});
        let v2425=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v2191}else{(if (self.scalar_static_f64[305]!=0.0){(v2191*self.scalar_static_f64[308])}else{v3})})+((v1110*self.scalar_static_f64[842])+v2346)));
        let v2428=(self.scalar_static_f64[0]*(self.scalar_static_f64[843]*((self.scalar_static_f64[807]*(v1-f64::powf(v2091,self.scalar_static_f64[218])))+(v160*(v708-v2088)))));
        let v2431=(self.scalar_static_f64[0]*((v2196*v2197)+((v1165*self.scalar_static_f64[844])+v2347)));
        let v2434=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2326*v2328)}else{v3}));
        let v2438=((self.scalar_static_f64[0]*(v716-v713))*self.scalar_static_f64[319]);
        let v2442=(v723*self.scalar_static_f64[320]);
        let v2450=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(v1-f64::powf(v2160,self.scalar_static_f64[224])))+(self.scalar_static_f64[810]*(v737-v2158))))+(self.scalar_static_f64[541]*v737)))))+(if (self.scalar_static_f64[302]!=0.0){(v1715*v2286)}else{v3})));
        let v2456=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*((self.scalar_static_f64[540]*((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(v1-f64::powf(v2127,self.scalar_static_f64[224])))+(self.scalar_static_f64[810]*(v732-v2125))))+(self.scalar_static_f64[541]*v732)))*self.scalar_static_f64[296]))+(if (self.scalar_static_f64[302]!=0.0){(self.scalar_static_f64[7]*v2233)}else{v2233})));
        let v2467=ctx.node_voltage(nodes[10]);
        let v2493=(if v757{(v758*self.scalar_static_f64[862])}else{(if (v754!=0.0){(v755*self.scalar_static_f64[862])}else{v3})});
        let v2494=(if v757{(v758*self.scalar_static_f64[863])}else{(if (v754!=0.0){(v755*self.scalar_static_f64[863])}else{v3})});
        let v2509=(if v768{(v769*self.scalar_static_f64[860])}else{(if (v765!=0.0){(v766*self.scalar_static_f64[860])}else{v3})});
        let v2510=(if v768{(v769*self.scalar_static_f64[864])}else{(if (v765!=0.0){(v766*self.scalar_static_f64[864])}else{v3})});
        let v2511=(if v768{(v769*self.scalar_static_f64[865])}else{(if (v765!=0.0){(v766*self.scalar_static_f64[865])}else{v3})});
        let v2512=(if v768{(v769*self.scalar_static_f64[861])}else{(if (v765!=0.0){(v766*self.scalar_static_f64[861])}else{v3})});
        let v2534=(if v790{(v791*self.scalar_static_f64[864])}else{(if (v787!=0.0){(v788*self.scalar_static_f64[864])}else{v3})});
        let v2535=(if v790{(v791*self.scalar_static_f64[866])}else{(if (v787!=0.0){(v788*self.scalar_static_f64[866])}else{v3})});
        let v2536=(if v790{(v791*self.scalar_static_f64[865])}else{(if (v787!=0.0){(v788*self.scalar_static_f64[865])}else{v3})});
        let v2537=(if v790{(v791*self.scalar_static_f64[861])}else{(if (v787!=0.0){(v788*self.scalar_static_f64[861])}else{v3})});
        let v2576=(if v826{(v827*self.scalar_static_f64[860])}else{(if (v823!=0.0){(v824*self.scalar_static_f64[860])}else{v3})});
        let v2577=(if v826{(v827*self.scalar_static_f64[861])}else{(if (v823!=0.0){(v824*self.scalar_static_f64[861])}else{v3})});
        let v2584=(if v838{(v839*self.scalar_static_f64[860])}else{(if (v835!=0.0){(v836*self.scalar_static_f64[860])}else{v3})});
        let v2585=(if v838{(v839*self.scalar_static_f64[861])}else{(if (v835!=0.0){(v836*self.scalar_static_f64[861])}else{v3})});
        let v2588=(v33*v846);
        let v2589=((v425*v2576)/v2588);
        let v2590=((v425*v2577)/v2588);
        let v2593=(v33*v849);
        let v2594=((v425*v2584)/v2593);
        let v2595=((v425*v2585)/v2593);
        let v2601=(v851*v851);
        let v2607=(if (v855!=0.0){v3}else{(((v851*(v33*v2584))-(v850*v2594))/v2601)});
        let v2608=(if (v855!=0.0){v3}else{(((v851*(v33*v2585))-(v850*v2595))/v2601)});
        let v2625=(self.scalar_static_f64[380]*((v2589-v2594)-((((v851*v2589)-(v858*v2594))/v2601)/v859)));
        let v2626=(self.scalar_static_f64[380]*((-v2595)-(((-(v858*v2595))/v2601)/v859)));
        let v2627=(self.scalar_static_f64[380]*(v2590-((v2590/v851)/v859)));
        let v2629=(self.scalar_static_f64[321]+v2627);
        let v2630=(v2625/self.scalar_static_f64[564]);
        let v2631=((self.scalar_static_f64[0]+v2626)/self.scalar_static_f64[564]);
        let v2632=(v2629/self.scalar_static_f64[564]);
        let v2642=(self.scalar_static_f64[564]*(v413*v2630));
        let v2643=(self.scalar_static_f64[564]*(v413*v2631));
        let v2644=(self.scalar_static_f64[564]*(v413*v2632));
        let v2656=(if (v866!=0.0){((self.scalar_static_f64[795]*((self.scalar_static_f64[381]*v2642)/v883))-(if v873{(self.scalar_static_f64[0]/v875)}else{(if v870{self.scalar_static_f64[0]}else{v3})}))}else{v3});
        let v2657=(if (v866!=0.0){((self.scalar_static_f64[795]*((self.scalar_static_f64[381]*v2643)/v883))-(if v873{(self.scalar_static_f64[321]/v875)}else{(if v870{self.scalar_static_f64[321]}else{v3})}))}else{v3});
        let v2658=(if (v866!=0.0){(self.scalar_static_f64[795]*((self.scalar_static_f64[381]*v2644)/v883))}else{v3});
        let v2659=(v888*v2656);
        let v2661=(v888*v2657);
        let v2663=(v888*v2658);
        let v2668=(v33*v901);
        let v2669=((if (v866!=0.0){(v2659+v2659)}else{v3})/v2668);
        let v2670=((if (v866!=0.0){(v2661+v2661)}else{v3})/v2668);
        let v2671=((if (v866!=0.0){(v2663+v2663)}else{v3})/v2668);
        let v2677=(v902*v902);
        let v2694=(if v906{(v413*(v2656+v2669))}else{(if v898{((-(v899*(v2669-v2656)))/v2677)}else{v3})});
        let v2695=(if v906{(v413*(v2657+v2670))}else{(if v898{((-(v899*(v2670-v2657)))/v2677)}else{v3})});
        let v2696=(if v906{(v413*(v2658+v2671))}else{(if v898{((-(v899*(v2671-v2658)))/v2677)}else{v3})});
        let v2712=(v917*v917);
        let v2722=(if (v866!=0.0){(((v917*((v913*v2694)+(v909*v2694)))-(v914*(self.scalar_static_f64[202]*v2694)))/v2712)}else{v3});
        let v2723=(if (v866!=0.0){(((v917*((v913*v2695)+(v909*v2695)))-(v914*(self.scalar_static_f64[202]*v2695)))/v2712)}else{v3});
        let v2724=(if (v866!=0.0){(((v917*((v913*v2696)+(v909*v2696)))-(v914*(self.scalar_static_f64[202]*v2696)))/v2712)}else{v3});
        let v2728=(v919*v919);
        let v2738=(if (v866!=0.0){(((v919*v2630)-(v864*v2722))/v2728)}else{v3});
        let v2739=(if (v866!=0.0){(((v919*v2631)-(v864*v2723))/v2728)}else{v3});
        let v2740=(if (v866!=0.0){(((v919*v2632)-(v864*v2724))/v2728)}else{v3});
        let v2744=(if (v866!=0.0){(v2738/self.scalar_static_f64[204])}else{v3});
        let v2745=(if (v866!=0.0){(v2739/self.scalar_static_f64[204])}else{v3});
        let v2746=(if (v866!=0.0){(v2740/self.scalar_static_f64[204])}else{v3});
        let v2780=(if (v866!=0.0){((if v936{(v2738+(self.scalar_static_f64[204]*((v938*(-v2744))/v939)))}else{(if v928{(self.scalar_static_f64[204]*((v929*v2744)/v930))}else{v3})})/self.scalar_static_f64[210])}else{v3});
        let v2781=(if (v866!=0.0){((if v936{(v2739+(self.scalar_static_f64[204]*((v938*(-v2745))/v939)))}else{(if v928{(self.scalar_static_f64[204]*((v929*v2745)/v930))}else{v3})})/self.scalar_static_f64[210])}else{v3});
        let v2782=(if (v866!=0.0){((if v936{(v2740+(self.scalar_static_f64[204]*((v938*(-v2746))/v939)))}else{(if v928{(self.scalar_static_f64[204]*((v929*v2746)/v930))}else{v3})})/self.scalar_static_f64[210])}else{v3});
        let v2786=(if (v866!=0.0){(v2694/self.scalar_static_f64[203])}else{v3});
        let v2787=(if (v866!=0.0){(v2695/self.scalar_static_f64[203])}else{v3});
        let v2788=(if (v866!=0.0){(v2696/self.scalar_static_f64[203])}else{v3});
        let v2810=(v33*v960);
        let v2829=(v963*v963);
        let v2839=(if (v866!=0.0){(((v963*(((v957*((v955*v2786)+(v954*(v425*v2780))))+(v956*v2786))/v2810))-(v961*((v962*v2786)+(v957*(v33*v2780)))))/v2829)}else{v3});
        let v2840=(if (v866!=0.0){(((v963*(((v957*((v955*v2787)+(v954*(v425*v2781))))+(v956*v2787))/v2810))-(v961*((v962*v2787)+(v957*(v33*v2781)))))/v2829)}else{v3});
        let v2841=(if (v866!=0.0){(((v963*(((v957*((v955*v2788)+(v954*(v425*v2782))))+(v956*v2788))/v2810))-(v961*((v962*v2788)+(v957*(v33*v2782)))))/v2829)}else{v3});
        let v2847=((v965*v2607)+(v856*v2839));
        let v2850=((v965*v2608)+(v856*v2840));
        let v2851=(v856*v2841);
        let v2858=(v969*v969);
        let v2868=(if (v866!=0.0){(((v969*((-v2839)+v2847))-(v968*v2847))/v2858)}else{v3});
        let v2869=(if (v866!=0.0){(((v969*((-v2840)+v2850))-(v968*v2850))/v2858)}else{v3});
        let v2870=(if (v866!=0.0){(((v969*((-v2841)+v2851))-(v968*v2851))/v2858)}else{v3});
        let v2883=(if (v866!=0.0){(self.scalar_static_f64[381]*((v971*v2642)+(v881*v2868)))}else{v3});
        let v2884=(if (v866!=0.0){(self.scalar_static_f64[381]*((v971*v2643)+(v881*v2869)))}else{v3});
        let v2885=(if (v866!=0.0){(self.scalar_static_f64[381]*((v971*v2644)+(v881*v2870)))}else{v3});
        let v2901=(if (v866!=0.0){((v33*v2883)+((v977*v2607)+(v856*(v2607+v2883))))}else{v3});
        let v2902=(if (v866!=0.0){((v33*v2884)+((v977*v2608)+(v856*(v2608+v2884))))}else{v3});
        let v2903=(if (v866!=0.0){((v33*v2885)+(v856*v2885))}else{v3});
        let v2907=(if (v866!=0.0){(v413*v2883)}else{v3});
        let v2908=(if (v866!=0.0){(v413*v2884)}else{v3});
        let v2909=(if (v866!=0.0){(v413*v2885)}else{v3});
        let v2910=(v983*v2907);
        let v2912=(v983*v2908);
        let v2914=(v983*v2909);
        let v2919=(if (v866!=0.0){(v2901+(v2910+v2910))}else{v3});
        let v2920=(if (v866!=0.0){(v2902+(v2912+v2912))}else{v3});
        let v2921=(if (v866!=0.0){(v2903+(v2914+v2914))}else{v3});
        let v2922=(v33*v990);
        let v2923=(v2919/v2922);
        let v2924=(v2920/v2922);
        let v2925=(v2921/v2922);
        let v2938=(v995*v995);
        let v2951=(if v1001{v3}else{(if v994{(((v995*v2901)-(v980*(v2923-v2907)))/v2938)}else{(if v989{(v2907+v2923)}else{v3})})});
        let v2952=(if v1001{v3}else{(if v994{(((v995*v2902)-(v980*(v2924-v2908)))/v2938)}else{(if v989{(v2908+v2924)}else{v3})})});
        let v2953=(if v1001{v3}else{(if v994{(((v995*v2903)-(v980*(v2925-v2909)))/v2938)}else{(if v989{(v2909+v2925)}else{v3})})});
        let v2972=(if (v866!=0.0){(self.scalar_static_f64[212]*v2630)}else{v3});
        let v2973=(if (v866!=0.0){(self.scalar_static_f64[212]*v2631)}else{v3});
        let v2974=(if (v866!=0.0){(self.scalar_static_f64[212]*v2632)}else{v3});
        let v2981=(v1012*v2972);
        let v2983=(v1012*v2973);
        let v2985=(v1012*v2974);
        let v2990=(v33*v1019);
        let v3009=(v1031*v1031);
        let v3025=(self.scalar_static_f64[201]*v2630);
        let v3026=(self.scalar_static_f64[201]*v2631);
        let v3027=(self.scalar_static_f64[201]*v2632);
        let v3031=(v1037*v1037);
        let v3058=(v858*v858);
        let v3066=(if v1042{(((v858*(v33*v2577))-(v1043*v2590))/v3058)}else{v2953});
        let v3067=(if v1042{(if v744{(v746*self.scalar_static_f64[860])}else{(if (v741!=0.0){(v742*self.scalar_static_f64[860])}else{v3})})}else{(if (v866!=0.0){(self.scalar_static_f64[799]*((v1003*v2951)+(v1002*v2951)))}else{v3})});
        let v3068=(if v1042{v3}else{(if (v866!=0.0){(self.scalar_static_f64[799]*((v1003*v2952)+(v1002*v2952)))}else{v3})});
        let v3069=(if v1042{(if v744{(v746*self.scalar_static_f64[861])}else{(if (v741!=0.0){(v742*self.scalar_static_f64[861])}else{v3})})}else{(if (v866!=0.0){(self.scalar_static_f64[799]*((v1003*v2953)+(v1002*v2953)))}else{v3})});
        let v3070=(v2607+(if v1042{(((v858*(v33*v2576))-(v1043*v2589))/v3058)}else{v2951}));
        let v3071=(v2608+(if v1042{v3}else{v2952}));
        let v3075=(if v1059{(v413*v3070)}else{v3});
        let v3076=(if v1059{(v413*v3071)}else{v3});
        let v3077=(if v1059{(v413*v3066)}else{v3});
        let v3081=(v1063*v1063);
        let v3100=(v1069*v1069);
        let v3110=(if v1067{(((v1069*v2625)-(v862*((self.scalar_static_f64[0]+v2625)-self.scalar_static_f64[0])))/v3100)}else{(if v1059{(((v1063*v3075)-(v1062*v3075))/v3081)}else{v2868})});
        let v3111=(if v1067{(((v1069*v2626)-(v862*(v2626-self.scalar_static_f64[321])))/v3100)}else{(if v1059{(((v1063*v3076)-(v1062*v3076))/v3081)}else{v2869})});
        let v3112=(if v1067{(((v1069*v2627)-(v862*v2629))/v3100)}else{(if v1059{(((v1063*v3077)-(v1062*v3077))/v3081)}else{v2870})});
        let v3116=(if v1042{v3}else{(if v1029{(self.scalar_static_f64[507]*(((v1031*(v33*v2630))-(v1030*(v2630+v2722)))/v3009))}else{v3})});
        let v3117=(if v1042{v3}else{(if v1029{(self.scalar_static_f64[507]*(((v1031*(v33*v2631))-(v1030*(v2631+v2723)))/v3009))}else{v3})});
        let v3118=(if v1042{v3}else{(if v1029{(self.scalar_static_f64[507]*(((v1031*(v33*v2632))-(v1030*(v2632+v2724)))/v3009))}else{v3})});
        let v3119=(if v1042{v2630}else{(if (v866!=0.0){(((v1037*v3025)-(v1036*v2630))/v3031)}else{v3})});
        let v3120=(if v1042{v2631}else{(if (v866!=0.0){(((v1037*v3026)-(v1036*v2631))/v3031)}else{v3})});
        let v3121=(if v1042{v2632}else{(if (v866!=0.0){(((v1037*v3027)-(v1036*v2632))/v3031)}else{v3})});
        let v3128=(if v1042{(-(v3119/self.scalar_static_f64[201]))}else{(if (v866!=0.0){((-v3025)/v3031)}else{v3})});
        let v3129=(if v1042{(-(v3120/self.scalar_static_f64[201]))}else{(if (v866!=0.0){((-v3026)/v3031)}else{v3})});
        let v3130=(if v1042{(-(v3121/self.scalar_static_f64[201]))}else{(if (v866!=0.0){((-v3027)/v3031)}else{v3})});
        let v3153=(if v1093{(-(self.scalar_static_f64[806]*((v1095*self.scalar_static_f64[869])/v1096)))}else{(if (v1086!=0.0){(self.scalar_static_f64[321]-(self.scalar_static_f64[806]*((v1087*self.scalar_static_f64[867])/v1088)))}else{v3})});
        let v3154=(if v1093{(-(self.scalar_static_f64[806]*((v1095*self.scalar_static_f64[870])/v1096)))}else{(if (v1086!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[806]*((v1087*self.scalar_static_f64[868])/v1088)))}else{v3})});
        let v3157=(-(self.scalar_static_f64[528]*v3153));
        let v3158=(-(self.scalar_static_f64[528]*v3154));
        let v3161=(self.scalar_static_f64[218]*f64::powf(v1102,self.scalar_static_f64[325]));
        let v3162=(v3157*v3161);
        let v3163=(v3158*v3161);
        let v3172=((self.scalar_static_f64[807]*(-v3162))+(v160*(self.scalar_static_f64[321]-v3153)));
        let v3173=((self.scalar_static_f64[807]*(-v3163))+(v160*(self.scalar_static_f64[0]-v3154)));
        let v3181=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v1042{v3}else{(if (v866!=0.0){(v2972+(((if (v866!=0.0){(self.scalar_static_f64[801]*v2630)}else{v3})+(v2981+v2981))/v2990))}else{v3})}))}else{self.scalar_static_f64[326]})});
        let v3182=(if self.scalar_static_bool[26]{v3}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[321]+(if v1042{self.scalar_static_f64[0]}else{(if (v866!=0.0){(v2973+(((if (v866!=0.0){(self.scalar_static_f64[801]*v2631)}else{v3})+(v2983+v2983))/v2990))}else{v3})}))}else{self.scalar_static_f64[327]})});
        let v3183=(if self.scalar_static_bool[26]{self.scalar_static_f64[321]}else{(if self.scalar_static_bool[24]{(if v1042{self.scalar_static_f64[321]}else{(if (v866!=0.0){(v2974+(((if (v866!=0.0){(self.scalar_static_f64[801]*v2632)}else{v3})+(v2985+v2985))/v2990))}else{v3})})}else{v3})});
        let v3187=(v1073*v1073);
        let v3188=(((v1073*v3181)-(v1131*v3116))/v3187);
        let v3192=(((v1073*v3182)-(v1131*v3117))/v3187);
        let v3196=(((v1073*v3183)-(v1131*v3118))/v3187);
        let v3239=(if v1141{(-((v1145*v3116)+(v1073*((v1143*(-v3188))/v1144))))}else{(if (v1134!=0.0){(v3181-((v1137*v3116)+(v1073*((v1135*v3188)/v1136))))}else{v3})});
        let v3240=(if v1141{(-((v1145*v3117)+(v1073*((v1143*(-v3192))/v1144))))}else{(if (v1134!=0.0){(v3182-((v1137*v3117)+(v1073*((v1135*v3192)/v1136))))}else{v3})});
        let v3241=(if v1141{(-((v1145*v3118)+(v1073*((v1143*(-v3196))/v1144))))}else{(if (v1134!=0.0){(v3183-((v1137*v3118)+(v1073*((v1135*v3196)/v1136))))}else{v3})});
        let v3244=(self.scalar_static_f64[223]*f64::powf(v1077,self.scalar_static_f64[328]));
        let v3245=(v3128*v3244);
        let v3246=(v3129*v3244);
        let v3247=(v3130*v3244);
        let v3256=(self.scalar_static_f64[224]*f64::powf(v1154,self.scalar_static_f64[329]));
        let v3295=(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((v1155*v3247)+(v1150*((-(v3241/self.scalar_static_f64[507]))*v3256)))))+((v1160*(self.scalar_static_f64[810]*v3247))+(v1159*(v3183-v3241)))));
        let v3298=((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((v1155*v3245)+(v1150*((-(v3239/self.scalar_static_f64[507]))*v3256)))))+((v1160*(self.scalar_static_f64[810]*v3245))+(v1159*(v3181-v3239)))))+self.scalar_static_f64[871]);
        let v3299=((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((v1155*v3246)+(v1150*((-(v3240/self.scalar_static_f64[507]))*v3256)))))+((v1160*(self.scalar_static_f64[810]*v3246))+(v1159*(v3182-v3240)))))+self.scalar_static_f64[872]);
        let v3300=(self.scalar_static_f64[816]*v2493);
        let v3301=(self.scalar_static_f64[816]*v2494);
        let v3302=(v33*v1170);
        let v3303=(v3300/v3302);
        let v3304=(v3301/v3302);
        let v3308=(v1171*v1171);
        let v3309=(((v1171*v3300)-(v1168*v3303))/v3308);
        let v3313=(((v1171*v3301)-(v1168*v3304))/v3308);
        let v3316=(self.scalar_static_f64[817]*f64::powf(v1046,self.scalar_static_f64[873]));
        let v3317=(v3067*v3316);
        let v3318=(v3068*v3316);
        let v3319=(v3069*v3316);
        let v3320=(self.scalar_static_f64[816]*v3317);
        let v3321=(self.scalar_static_f64[816]*v3318);
        let v3322=(self.scalar_static_f64[816]*v3319);
        let v3323=(v33*v1177);
        let v3330=(v1178*v1178);
        let v3331=(((v1178*v3320)-(v1175*(v3320/v3323)))/v3330);
        let v3335=(((v1178*v3321)-(v1175*(v3321/v3323)))/v3330);
        let v3339=(((v1178*v3322)-(v1175*(v3322/v3323)))/v3330);
        let v3340=(v3172/self.scalar_static_f64[750]);
        let v3341=(v3173/self.scalar_static_f64[750]);
        let v3342=(v3298/self.scalar_static_f64[748]);
        let v3343=(v3299/self.scalar_static_f64[748]);
        let v3344=(v3295/self.scalar_static_f64[748]);
        let v3345=(v3341+v3342);
        let v3383=(if self.scalar_static_bool[28]{((v1196*(if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*v3340))}else{v3}))/self.scalar_static_f64[820])}else{(if (self.scalar_static_f64[225]!=0.0){v3340}else{v3})});
        let v3384=(if self.scalar_static_bool[28]{(((v1196*(if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*v3341))}else{v3}))-(v1197*(if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*((-v3298)/self.scalar_static_f64[748])))}else{v3})))/self.scalar_static_f64[820])}else{(if (self.scalar_static_f64[225]!=0.0){v3345}else{v3})});
        let v3385=(if self.scalar_static_bool[28]{((-(v1197*(if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*((-v3299)/self.scalar_static_f64[748])))}else{v3})))/self.scalar_static_f64[820])}else{(if (self.scalar_static_f64[225]!=0.0){v3343}else{v3})});
        let v3386=(if self.scalar_static_bool[28]{((-(v1197*(if self.scalar_static_bool[28]{(self.scalar_static_f64[381]*(self.scalar_static_f64[779]*((-v3295)/self.scalar_static_f64[748])))}else{v3})))/self.scalar_static_f64[820])}else{(if (self.scalar_static_f64[225]!=0.0){v3344}else{v3})});
        let v3387=(v1203*v3383);
        let v3388=(v3387+v3387);
        let v3389=(v1203*v3384);
        let v3390=(v3389+v3389);
        let v3391=(v1203*v3385);
        let v3392=(v3391+v3391);
        let v3393=(v1203*v3386);
        let v3394=(v3393+v3393);
        let v3395=(v33*v1210);
        let v3396=(v3388/v3395);
        let v3397=(v3390/v3395);
        let v3398=(v3392/v3395);
        let v3399=(v3394/v3395);
        let v3406=(v1211*v1211);
        let v3434=(v413*v3309);
        let v3435=(v413*(v3313+v3331));
        let v3436=(v413*v3335);
        let v3437=(v413*v3339);
        let v3440=((v1220*(if v1214{(v413*(v3383+v3396))}else{(if (v1207!=0.0){((-(v1208*(v3396-v3383)))/v3406)}else{v3})}))+(v1217*v3434));
        let v3443=((v1220*(if v1214{(v413*(v3384+v3397))}else{(if (v1207!=0.0){((-(v1208*(v3397-v3384)))/v3406)}else{v3})}))+(v1217*v3435));
        let v3446=((v1220*(if v1214{(v413*(v3385+v3398))}else{(if (v1207!=0.0){((-(v1208*(v3398-v3385)))/v3406)}else{v3})}))+(v1217*v3436));
        let v3449=((v1220*(if v1214{(v413*(v3386+v3399))}else{(if (v1207!=0.0){((-(v1208*(v3399-v3386)))/v3406)}else{v3})}))+(v1217*v3437));
        let v3450=(self.scalar_static_f64[821]*v3317);
        let v3451=(self.scalar_static_f64[821]*v3318);
        let v3452=(self.scalar_static_f64[821]*v3319);
        let v3454=(self.scalar_static_f64[633]*v2494);
        let v3458=(v1221*(self.scalar_static_f64[633]*v2493));
        let v3461=(v1221*v1221);
        let v3495=(if v1237{(self.scalar_static_f64[321]+(v1228*((v1239*self.scalar_static_f64[332])/v1240)))}else{(if (v1231!=0.0){(v1228*((v1232*self.scalar_static_f64[330])/v1233))}else{v3})});
        let v3496=(if v1237{(self.scalar_static_f64[0]+(v1228*((v1239*self.scalar_static_f64[333])/v1240)))}else{(if (v1231!=0.0){(v1228*((v1232*self.scalar_static_f64[331])/v1233))}else{v3})});
        let v3555=(if v1288{(v1289*self.scalar_static_f64[874])}else{(if (v1285!=0.0){(v1286*self.scalar_static_f64[874])}else{v3495})});
        let v3556=(if v1288{(v1289*self.scalar_static_f64[875])}else{(if (v1285!=0.0){(v1286*self.scalar_static_f64[875])}else{v3496})});
        let v3689=(if v1367{(v1368*self.scalar_static_f64[876])}else{(if (v1364!=0.0){(v1365*self.scalar_static_f64[876])}else{v3555})});
        let v3690=(if v1367{(v1368*self.scalar_static_f64[877])}else{(if (v1364!=0.0){(v1365*self.scalar_static_f64[877])}else{v3})});
        let v3691=(if v1367{v3}else{(if (v1364!=0.0){v3}else{v3556})});
        let v3746=(if v1404{(v1405*self.scalar_static_f64[878])}else{(if (v1401!=0.0){(v1402*self.scalar_static_f64[878])}else{v3689})});
        let v3747=(if v1404{v3}else{(if (v1401!=0.0){v3}else{v3690})});
        let v3748=(if v1404{(v1405*self.scalar_static_f64[879])}else{(if (v1401!=0.0){(v1402*self.scalar_static_f64[879])}else{v3691})});
        let v3761=(if v1417{(v1418*self.scalar_static_f64[880])}else{(if (v1414!=0.0){(v1415*self.scalar_static_f64[880])}else{v3746})});
        let v3762=(if v1417{(v1418*self.scalar_static_f64[881])}else{(if (v1414!=0.0){(v1415*self.scalar_static_f64[881])}else{v3747})});
        let v3763=(if v1417{v3}else{(if (v1414!=0.0){v3}else{v3748})});
        let v3784=(if v1430{v3}else{(if (v1427!=0.0){v3}else{v3761})});
        let v3785=(if v1430{(v1431*self.scalar_static_f64[882])}else{(if (v1427!=0.0){(v1428*self.scalar_static_f64[882])}else{v3762})});
        let v3786=(if v1430{(v1431*self.scalar_static_f64[883])}else{(if (v1427!=0.0){(v1428*self.scalar_static_f64[883])}else{v3763})});
        let v3787=(if v1430{(v1431*self.scalar_static_f64[884])}else{(if (v1427!=0.0){(v1428*self.scalar_static_f64[884])}else{v3})});
        let v3788=(if v1430{(v1431*self.scalar_static_f64[885])}else{(if (v1427!=0.0){(v1428*self.scalar_static_f64[885])}else{v3})});
        let v3805=(if v1443{(v1444*self.scalar_static_f64[886])}else{(if (v1440!=0.0){(v1441*self.scalar_static_f64[886])}else{v3784})});
        let v3806=(if v1443{(v1444*self.scalar_static_f64[887])}else{(if (v1440!=0.0){(v1441*self.scalar_static_f64[887])}else{v3785})});
        let v3807=(if v1443{v3}else{(if (v1440!=0.0){v3}else{v3786})});
        let v3808=(if v1443{v3}else{(if (v1440!=0.0){v3}else{v3787})});
        let v3809=(if v1443{v3}else{(if (v1440!=0.0){v3}else{v3788})});
        let v4147=(self.scalar_static_f64[816]*v2509);
        let v4148=(self.scalar_static_f64[816]*v2510);
        let v4149=(self.scalar_static_f64[816]*v2511);
        let v4150=(self.scalar_static_f64[816]*v2512);
        let v4151=(v425*(if v814{(v815*self.scalar_static_f64[860])}else{(if (v811!=0.0){(v812*self.scalar_static_f64[860])}else{v3})}));
        let v4152=(v425*(if v814{(v815*self.scalar_static_f64[864])}else{(if (v811!=0.0){(v812*self.scalar_static_f64[864])}else{v3})}));
        let v4153=(v425*(if v814{(v815*self.scalar_static_f64[865])}else{(if (v811!=0.0){(v812*self.scalar_static_f64[865])}else{v3})}));
        let v4154=(v425*(if v814{(v815*self.scalar_static_f64[861])}else{(if (v811!=0.0){(v812*self.scalar_static_f64[861])}else{v3})}));
        let v4155=(v33*v1643);
        let v4163=(v1644*v1644);
        let v4177=(v33*v1647);
        let v4185=(v1648*v1648);
        let v4245=(v33*v1673);
        let v4253=(v1674*v1674);
        let v4267=(if (self.scalar_static_f64[242]!=0.0){(((v1674*(self.scalar_static_f64[829]*v2534))-(v1670*((self.scalar_static_f64[828]*v2534)/v4245)))/v4253)}else{v3});
        let v4268=(if (self.scalar_static_f64[242]!=0.0){(((v1674*(self.scalar_static_f64[829]*v2535))-(v1670*((self.scalar_static_f64[828]*v2535)/v4245)))/v4253)}else{v3});
        let v4269=(if (self.scalar_static_f64[242]!=0.0){(((v1674*(self.scalar_static_f64[829]*v2536))-(v1670*((self.scalar_static_f64[828]*v2536)/v4245)))/v4253)}else{v3});
        let v4270=(if (self.scalar_static_f64[242]!=0.0){(((v1674*(self.scalar_static_f64[829]*v2537))-(v1670*((self.scalar_static_f64[828]*v2537)/v4245)))/v4253)}else{v3});
        let v4275=(v1689*self.scalar_static_f64[346]);
        let v4276=(v4275+v4275);
        let v4277=(v1689*self.scalar_static_f64[347]);
        let v4279=(v1689*self.scalar_static_f64[348]);
        let v4280=(v4279+v4279);
        let v4281=(v1689*self.scalar_static_f64[349]);
        let v4283=(if self.scalar_static_bool[44]{v4276}else{v3});
        let v4284=(if self.scalar_static_bool[44]{(v4277+v4277)}else{v3});
        let v4285=(if self.scalar_static_bool[44]{v3}else{v3388});
        let v4286=(if self.scalar_static_bool[44]{v4276}else{v3390});
        let v4287=(if self.scalar_static_bool[44]{v4280}else{v3392});
        let v4288=(if self.scalar_static_bool[44]{v4280}else{v3394});
        let v4289=(if self.scalar_static_bool[44]{(v4281+v4281)}else{v3});
        let v4290=(if self.scalar_static_bool[44]{v4280}else{v3});
        let v4291=(v33*v1699);
        let v4292=(v4283/v4291);
        let v4293=(v4284/v4291);
        let v4294=(v4285/v4291);
        let v4295=(v4286/v4291);
        let v4296=(v4287/v4291);
        let v4297=(v4288/v4291);
        let v4298=(v4289/v4291);
        let v4299=(v4290/v4291);
        let v4309=(v1700*v1700);
        let v4355=(if v1704{(v413*(self.scalar_static_f64[346]+v4292))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4292-self.scalar_static_f64[346])))/v4309)}else{v3})});
        let v4356=(if v1704{(v413*(self.scalar_static_f64[347]+v4293))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4293-self.scalar_static_f64[347])))/v4309)}else{v3})});
        let v4357=(if v1704{(v413*v4294)}else{(if v1696{((-(self.scalar_static_f64[246]*v4294))/v4309)}else{v3})});
        let v4358=(if v1704{(v413*(self.scalar_static_f64[346]+v4295))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4295-self.scalar_static_f64[346])))/v4309)}else{v3})});
        let v4359=(if v1704{(v413*(self.scalar_static_f64[348]+v4296))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4296-self.scalar_static_f64[348])))/v4309)}else{v3})});
        let v4360=(if v1704{(v413*(self.scalar_static_f64[348]+v4297))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4297-self.scalar_static_f64[348])))/v4309)}else{v3})});
        let v4361=(if v1704{(v413*(self.scalar_static_f64[349]+v4298))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4298-self.scalar_static_f64[349])))/v4309)}else{v3})});
        let v4362=(if v1704{(v413*(self.scalar_static_f64[348]+v4299))}else{(if v1696{((-(self.scalar_static_f64[246]*(v4299-self.scalar_static_f64[348])))/v4309)}else{v3})});
        let v4363=(self.scalar_static_f64[557]*v4267);
        let v4365=(self.scalar_static_f64[557]*v4269);
        let v4377=(v1710*v1710);
        let v4415=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4355)-(v1707*(v4355+v4363)))/v4377)}else{v3})});
        let v4416=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4356)-(v1707*(v4356+(self.scalar_static_f64[557]*v4268))))/v4377)}else{v3})});
        let v4417=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4357)-(v1707*v4357))/v4377)}else{v3})});
        let v4418=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4358)-(v1707*(v4358+v4363)))/v4377)}else{v3})});
        let v4419=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4359)-(v1707*(v4359+v4365)))/v4377)}else{v3})});
        let v4420=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4360)-(v1707*(v4360+v4365)))/v4377)}else{v3})});
        let v4421=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4361)-(v1707*(v4361+(self.scalar_static_f64[557]*v4270))))/v4377)}else{v3})});
        let v4422=(if self.scalar_static_bool[46]{v3}else{(if self.scalar_static_bool[44]{(((v1710*v4362)-(v1707*(v4362+v4365)))/v4377)}else{v3})});
        let v4685=(v1185*v3340);
        let v4687=(v1185*v3345);
        let v4689=(v1185*v3343);
        let v4691=(v1185*v3344);
        let v4693=(v33*v1780);
        let v4694=((v4685+v4685)/v4693);
        let v4695=((v4687+v4687)/v4693);
        let v4696=((v4689+v4689)/v4693);
        let v4697=((v4691+v4691)/v4693);
        let v4704=(v1781*v1781);
        let v4727=(if v1784{(v413*(v3340+v4694))}else{(if (v1778!=0.0){((-(v1208*(v4694-v3340)))/v4704)}else{v3})});
        let v4728=(if v1784{(v413*(v3345+v4695))}else{(if (v1778!=0.0){((-(v1208*(v4695-v3345)))/v4704)}else{v3})});
        let v4729=(if v1784{(v413*(v3343+v4696))}else{(if (v1778!=0.0){((-(v1208*(v4696-v3343)))/v4704)}else{v3})});
        let v4730=(if v1784{(v413*(v3344+v4697))}else{(if (v1778!=0.0){((-(v1208*(v4697-v3344)))/v4704)}else{v3})});
        let v5608=(if v2081{(-(self.scalar_static_f64[806]*((v2083*self.scalar_static_f64[869])/v2084)))}else{(if (v2074!=0.0){(self.scalar_static_f64[321]-(self.scalar_static_f64[806]*((v2075*self.scalar_static_f64[867])/v2076)))}else{v3})});
        let v5609=(if v2081{(-(self.scalar_static_f64[806]*((v2083*self.scalar_static_f64[870])/v2084)))}else{(if (v2074!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[806]*((v2075*self.scalar_static_f64[868])/v2076)))}else{v3})});
        let v5615=(self.scalar_static_f64[218]*f64::powf(v2091,self.scalar_static_f64[325]));
        let v5637=((v2104*v4727)+(v1787*(self.scalar_static_f64[846]*v3309)));
        let v5640=((v2104*v4728)+(v1787*(self.scalar_static_f64[846]*v3313)));
        let v5641=(v2104*v4729);
        let v5642=(v2104*v4730);
        let v5646=(v2106*v4727);
        let v5649=((v2106*v4728)+(v1787*(self.scalar_static_f64[846]*v3331)));
        let v5652=((v2106*v4729)+(v1787*(self.scalar_static_f64[846]*v3335)));
        let v5655=((v2106*v4730)+(v1787*(self.scalar_static_f64[846]*v3339)));
        let v5700=(if v2118{(-(self.scalar_static_f64[802]*((v2120*self.scalar_static_f64[904])/v2121)))}else{(if (v2111!=0.0){(self.scalar_static_f64[0]-(self.scalar_static_f64[802]*((v2112*self.scalar_static_f64[900])/v2113)))}else{v3})});
        let v5701=(if v2118{(-(self.scalar_static_f64[802]*((v2120*self.scalar_static_f64[905])/v2121)))}else{(if (v2111!=0.0){(self.scalar_static_f64[322]-(self.scalar_static_f64[802]*((v2112*self.scalar_static_f64[901])/v2113)))}else{v3})});
        let v5702=(if v2118{(-(self.scalar_static_f64[802]*((v2120*self.scalar_static_f64[906])/v2121)))}else{(if (v2111!=0.0){(self.scalar_static_f64[323]-(self.scalar_static_f64[802]*((v2112*self.scalar_static_f64[902])/v2113)))}else{v3})});
        let v5703=(if v2118{(-(self.scalar_static_f64[802]*((v2120*self.scalar_static_f64[907])/v2121)))}else{(if (v2111!=0.0){(self.scalar_static_f64[321]-(self.scalar_static_f64[802]*((v2112*self.scalar_static_f64[903])/v2113)))}else{v3})});
        let v5713=(self.scalar_static_f64[224]*f64::powf(v2127,self.scalar_static_f64[329]));
        let v5798=(if v2151{(-(self.scalar_static_f64[802]*((v2153*self.scalar_static_f64[905])/v2154)))}else{(if (v2144!=0.0){(self.scalar_static_f64[322]-(self.scalar_static_f64[802]*((v2145*self.scalar_static_f64[901])/v2146)))}else{v3})});
        let v5799=(if v2151{(-(self.scalar_static_f64[802]*((v2153*self.scalar_static_f64[911])/v2154)))}else{(if (v2144!=0.0){(self.scalar_static_f64[324]-(self.scalar_static_f64[802]*((v2145*self.scalar_static_f64[910])/v2146)))}else{v3})});
        let v5800=(if v2151{(-(self.scalar_static_f64[802]*((v2153*self.scalar_static_f64[906])/v2154)))}else{(if (v2144!=0.0){(self.scalar_static_f64[323]-(self.scalar_static_f64[802]*((v2145*self.scalar_static_f64[902])/v2146)))}else{v3})});
        let v5801=(if v2151{(-(self.scalar_static_f64[802]*((v2153*self.scalar_static_f64[907])/v2154)))}else{(if (v2144!=0.0){(self.scalar_static_f64[321]-(self.scalar_static_f64[802]*((v2145*self.scalar_static_f64[903])/v2146)))}else{v3})});
        let v5811=(self.scalar_static_f64[224]*f64::powf(v2160,self.scalar_static_f64[329]));
        let v5853=(self.scalar_static_f64[6]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*(self.scalar_static_f64[908]+(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5798/self.scalar_static_f64[507]))*v5811)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[322]-v5798))))))));
        let v5855=(self.scalar_static_f64[6]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*(self.scalar_static_f64[909]+(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5800/self.scalar_static_f64[507]))*v5811)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[323]-v5800))))))));
        let v5873=(self.scalar_static_f64[850]*(if v2185{(v2186*self.scalar_static_f64[913])}else{(if (v2182!=0.0){(v2183*self.scalar_static_f64[913])}else{v3805})}));
        let v5874=(self.scalar_static_f64[850]*(if v2185{v3}else{(if (v2182!=0.0){v3}else{v3806})}));
        let v5875=(self.scalar_static_f64[850]*(if v2185{(v2186*self.scalar_static_f64[914])}else{(if (v2182!=0.0){(v2183*self.scalar_static_f64[914])}else{v3807})}));
        let v5876=(self.scalar_static_f64[850]*(if v2185{v3}else{(if (v2182!=0.0){v3}else{v3808})}));
        let v5877=(self.scalar_static_f64[850]*(if v2185{v3}else{(if (v2182!=0.0){v3}else{v3809})}));
        let v5946=(v33*v2230);
        let v5954=(v2231*v2231);
        let v5968=(if self.scalar_static_bool[60]{(((v2231*(self.scalar_static_f64[857]*v2509))-(v2227*((v425*(if v2220{(v2221*self.scalar_static_f64[915])}else{(if v2216{(v2217*self.scalar_static_f64[915])}else{v3})}))/v5946)))/v5954)}else{(if (self.scalar_static_f64[300]!=0.0){((self.scalar_static_f64[856]*((self.scalar_static_f64[845]*(((v1644*v4147)-(v1641*(v4147/v4155)))/v4163))+(self.scalar_static_f64[854]*(((v1648*v4151)-(v1640*(v4151/v4177)))/v4185))))/self.scalar_static_f64[763])}else{v3})});
        let v5969=(if self.scalar_static_bool[60]{(((v2231*(self.scalar_static_f64[857]*v2510))-(v2227*((v425*(if v2220{(v2221*self.scalar_static_f64[916])}else{(if v2216{(v2217*self.scalar_static_f64[916])}else{v3})}))/v5946)))/v5954)}else{(if (self.scalar_static_f64[300]!=0.0){((self.scalar_static_f64[856]*((self.scalar_static_f64[845]*(((v1644*v4148)-(v1641*(v4148/v4155)))/v4163))+(self.scalar_static_f64[854]*(((v1648*v4152)-(v1640*(v4152/v4177)))/v4185))))/self.scalar_static_f64[763])}else{v3})});
        let v5970=(if self.scalar_static_bool[60]{(((v2231*(self.scalar_static_f64[857]*v2511))-(v2227*((v425*(if v2220{(v2221*self.scalar_static_f64[917])}else{(if v2216{(v2217*self.scalar_static_f64[917])}else{v3})}))/v5946)))/v5954)}else{(if (self.scalar_static_f64[300]!=0.0){((self.scalar_static_f64[856]*((self.scalar_static_f64[845]*(((v1644*v4149)-(v1641*(v4149/v4155)))/v4163))+(self.scalar_static_f64[854]*(((v1648*v4153)-(v1640*(v4153/v4177)))/v4185))))/self.scalar_static_f64[763])}else{v3})});
        let v5971=(if self.scalar_static_bool[60]{(((v2231*(self.scalar_static_f64[857]*v2512))-(v2227*((v425*(if v2220{(v2221*self.scalar_static_f64[918])}else{(if v2216{(v2217*self.scalar_static_f64[918])}else{v3})}))/v5946)))/v5954)}else{(if (self.scalar_static_f64[300]!=0.0){((self.scalar_static_f64[856]*((self.scalar_static_f64[845]*(((v1644*v4150)-(v1641*(v4150/v4155)))/v4163))+(self.scalar_static_f64[854]*(((v1648*v4154)-(v1640*(v4154/v4177)))/v4185))))/self.scalar_static_f64[763])}else{v3})});
        let v5984=(if self.scalar_static_bool[64]{(self.scalar_static_f64[816]*v2534)}else{v3});
        let v5985=(if self.scalar_static_bool[64]{(self.scalar_static_f64[816]*v2535)}else{v3});
        let v5986=(if self.scalar_static_bool[64]{(self.scalar_static_f64[816]*v2536)}else{v3});
        let v5987=(if self.scalar_static_bool[64]{(self.scalar_static_f64[816]*v2537)}else{v3});
        let v5988=(v33*v2245);
        let v5996=(v2246*v2246);
        let v6018=(if self.scalar_static_bool[64]{(v425*(if v802{(v803*self.scalar_static_f64[864])}else{(if (v799!=0.0){(v800*self.scalar_static_f64[864])}else{v3})}))}else{v3});
        let v6019=(if self.scalar_static_bool[64]{(v425*(if v802{(v803*self.scalar_static_f64[866])}else{(if (v799!=0.0){(v800*self.scalar_static_f64[866])}else{v3})}))}else{v3});
        let v6020=(if self.scalar_static_bool[64]{(v425*(if v802{(v803*self.scalar_static_f64[865])}else{(if (v799!=0.0){(v800*self.scalar_static_f64[865])}else{v3})}))}else{v3});
        let v6021=(if self.scalar_static_bool[64]{(v425*(if v802{(v803*self.scalar_static_f64[861])}else{(if (v799!=0.0){(v800*self.scalar_static_f64[861])}else{v3})}))}else{v3});
        let v6022=(v33*v2252);
        let v6030=(v2253*v2253);
        let v6096=(v33*v2283);
        let v6104=(v2284*v2284);
        let v6123=(v1715*(if self.scalar_static_bool[65]{(((v2284*(self.scalar_static_f64[859]*v2534))-(v2280*((v425*(if v2273{(v2274*self.scalar_static_f64[864])}else{(if v2269{(v2270*self.scalar_static_f64[864])}else{v3})}))/v6096)))/v6104)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[858]*((self.scalar_static_f64[845]*(if self.scalar_static_bool[64]{(((v2246*v5984)-(v2243*(v5984/v5988)))/v5996)}else{v3}))+(self.scalar_static_f64[854]*(if self.scalar_static_bool[64]{(((v2253*v6018)-(v2250*(v6018/v6022)))/v6030)}else{v3}))))/self.scalar_static_f64[763])}else{v3})}));
        let v6132=(v1715*(if self.scalar_static_bool[65]{(((v2284*(self.scalar_static_f64[859]*v2536))-(v2280*((v425*(if v2273{(v2274*self.scalar_static_f64[865])}else{(if v2269{(v2270*self.scalar_static_f64[865])}else{v3})}))/v6096)))/v6104)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[858]*((self.scalar_static_f64[845]*(if self.scalar_static_bool[64]{(((v2246*v5986)-(v2243*(v5986/v5988)))/v5996)}else{v3}))+(self.scalar_static_f64[854]*(if self.scalar_static_bool[64]{(((v2253*v6020)-(v2250*(v6020/v6022)))/v6030)}else{v3}))))/self.scalar_static_f64[763])}else{v3})}));
        let v6151=(self.scalar_static_f64[306]*f64::powf(v1102,self.scalar_static_f64[363]));
        let v6161=(v2301*v2301);
        let v6169=(v2307*self.scalar_static_f64[921]);
        let v6170=(v2307*self.scalar_static_f64[922]);
        let v6174=(v2308*v2308);
        let v6200=(v1170*v1170);
        let v6237=(if (self.scalar_static_f64[305]!=0.0){(v5876/self.scalar_static_f64[851])}else{v3});
        let v6276=(self.scalar_static_f64[307]*v5876);
        let v6282=(if (self.scalar_static_f64[305]!=0.0){(v5637+(self.scalar_static_f64[307]*v5873))}else{v3});
        let v6283=(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[307]*v5874)}else{v3});
        let v6284=(if (self.scalar_static_f64[305]!=0.0){(v5640+(self.scalar_static_f64[307]*v5875))}else{v3});
        let v6285=(if (self.scalar_static_f64[305]!=0.0){(v5641+v6276)}else{v3});
        let v6286=(if (self.scalar_static_f64[305]!=0.0){(v5642+v6276)}else{v3});
        let v6287=(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[307]*v5877)}else{v3});
        let v6316=(if self.scalar_static_bool[67]{v5637}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6282)}else{v3})});
        let v6317=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6283)}else{v3})});
        let v6318=(if self.scalar_static_bool[67]{v5640}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6284)}else{v3})});
        let v6319=(if self.scalar_static_bool[67]{v5641}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6285)}else{v3})});
        let v6320=(if self.scalar_static_bool[67]{v5642}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6286)}else{v3})});
        let v6321=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[310]*v6287)}else{v3})});
        let v6322=(if self.scalar_static_bool[67]{v5646}else{(if (self.scalar_static_f64[305]!=0.0){(v5646+(self.scalar_static_f64[309]*v6282))}else{v3})});
        let v6323=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[309]*v6283)}else{v3})});
        let v6324=(if self.scalar_static_bool[67]{v5649}else{(if (self.scalar_static_f64[305]!=0.0){(v5649+(self.scalar_static_f64[309]*v6284))}else{v3})});
        let v6325=(if self.scalar_static_bool[67]{v5652}else{(if (self.scalar_static_f64[305]!=0.0){(v5652+(self.scalar_static_f64[309]*v6285))}else{v3})});
        let v6326=(if self.scalar_static_bool[67]{v5655}else{(if (self.scalar_static_f64[305]!=0.0){(v5655+(self.scalar_static_f64[309]*v6286))}else{v3})});
        let v6327=(if self.scalar_static_bool[67]{v3}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[309]*v6287)}else{v3})});
        let v6331=(if self.scalar_static_bool[67]{v5876}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[308]*v5876)}else{v3})});
        let v6349=(v2350*v2350);
        let v6396=(if v2364{((v2365*v3440)+(v1221*(self.scalar_static_f64[759]*v4727)))}else{(if (v2360!=0.0){(((v2350*(v6316+v6322))-(v2361*((v3458-(v2349*v3440))/v3461)))/v6349)}else{v3})});
        let v6397=(if v2364{v3}else{(if (v2360!=0.0){((v6317+v6323)/v2350)}else{v3})});
        let v6398=(if v2364{((v2365*v3443)+(v1221*(self.scalar_static_f64[759]*v4728)))}else{(if (v2360!=0.0){(((v2350*(v6318+v6324))-(v2361*(((v1221*(v3450+v3454))-(v2349*v3443))/v3461)))/v6349)}else{v3})});
        let v6399=(if v2364{((v2365*v3446)+(v1221*(self.scalar_static_f64[759]*v4729)))}else{(if (v2360!=0.0){(((v2350*(v6319+v6325))-(v2361*(((v1221*v3451)-(v2349*v3446))/v3461)))/v6349)}else{v3})});
        let v6400=(if v2364{((v2365*v3449)+(v1221*(self.scalar_static_f64[759]*v4730)))}else{(if (v2360!=0.0){(((v2350*(v6320+v6326))-(v2361*(((v1221*v3452)-(v2349*v3449))/v3461)))/v6349)}else{v3})});
        let v6401=(if v2364{v3}else{(if (v2360!=0.0){((v6321+v6327)/v2350)}else{v3})});
        let v6426=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6396)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6396)}else{v3})})});
        let v6427=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6397)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6397)}else{v3})})});
        let v6428=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6398)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6398)}else{v3})})});
        let v6429=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6399)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6399)}else{v3})})});
        let v6430=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6400)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6400)}else{v3})})});
        let v6431=(if self.scalar_static_bool[75]{v3}else{(if self.scalar_static_bool[73]{(self.scalar_static_f64[316]*v6401)}else{(if (self.scalar_static_f64[314]!=0.0){(self.scalar_static_f64[309]*v6401)}else{v3})})});
        let v6597=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v5873}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[308]*v5873)}else{v3})})+((self.scalar_static_f64[842]*v3172)+v6316)));
        let v6598=(self.scalar_static_f64[0]*(v6317+(if self.scalar_static_bool[67]{v5874}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[308]*v5874)}else{v3})})));
        let v6599=(self.scalar_static_f64[0]*((if self.scalar_static_bool[67]{v5875}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[308]*v5875)}else{v3})})+((self.scalar_static_f64[842]*v3173)+v6318)));
        let v6600=(self.scalar_static_f64[0]*(v6319+v6331));
        let v6601=(self.scalar_static_f64[0]*(v6320+v6331));
        let v6602=(self.scalar_static_f64[0]*(v6321+(if self.scalar_static_bool[67]{v5877}else{(if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[308]*v5877)}else{v3})})));
        let v6616=(self.scalar_static_f64[0]*(self.scalar_static_f64[843]*((self.scalar_static_f64[807]*(-((-(self.scalar_static_f64[528]*v5608))*v5615)))+(v160*(self.scalar_static_f64[321]-v5608)))));
        let v6617=(self.scalar_static_f64[0]*(self.scalar_static_f64[843]*((self.scalar_static_f64[807]*(-((-(self.scalar_static_f64[528]*v5609))*v5615)))+(v160*(self.scalar_static_f64[0]-v5609)))));
        let v6622=(self.scalar_static_f64[0]*v6322);
        let v6623=(self.scalar_static_f64[0]*v6323);
        let v6624=(self.scalar_static_f64[0]*(((v2197*(self.scalar_static_f64[855]*v3110))+(v2196*v3070))+((self.scalar_static_f64[844]*v3298)+v6324)));
        let v6625=(self.scalar_static_f64[0]*(((v2197*(self.scalar_static_f64[855]*v3111))+(v2196*v3071))+((self.scalar_static_f64[844]*v3299)+v6325)));
        let v6626=(self.scalar_static_f64[0]*(((v2197*(self.scalar_static_f64[855]*v3112))+(v2196*v3066))+((self.scalar_static_f64[844]*v3295)+v6326)));
        let v6627=(self.scalar_static_f64[0]*v6327);
        let v6640=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2326*((if (self.scalar_static_f64[305]!=0.0){(v5873/self.scalar_static_f64[851])}else{v3})+((if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[842]*(if (self.scalar_static_f64[305]!=0.0){((v2310*(if (self.scalar_static_f64[305]!=0.0){(v3157*v6151)}else{v3}))+(v2295*(if v2305{(((v2308*v6169)-(v2307*v6169))/v6174)}else{(if v2299{((-(v2300*self.scalar_static_f64[919]))/v6161)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[305]!=0.0){((v2321*(if (self.scalar_static_f64[305]!=0.0){((v2318*((self.scalar_static_f64[381]*v3300)/self.scalar_static_f64[588]))+(v2317*((-(v413*v3303))/v6200)))}else{v3}))+(v2320*(self.scalar_static_f64[846]*v4727)))}else{v3}))))}else{v3}));
        let v6641=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){((v2328*self.scalar_static_f64[364])+(v2326*(if (self.scalar_static_f64[305]!=0.0){(v5874/self.scalar_static_f64[851])}else{v3})))}else{v3}));
        let v6642=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){((v2328*self.scalar_static_f64[365])+(v2326*((if (self.scalar_static_f64[305]!=0.0){(v5875/self.scalar_static_f64[851])}else{v3})+((if (self.scalar_static_f64[305]!=0.0){(self.scalar_static_f64[842]*(if (self.scalar_static_f64[305]!=0.0){((v2310*(if (self.scalar_static_f64[305]!=0.0){(v3158*v6151)}else{v3}))+(v2295*(if v2305{(((v2308*v6170)-(v2307*v6170))/v6174)}else{(if v2299{((-(v2300*self.scalar_static_f64[920]))/v6161)}else{v3})})))}else{v3}))}else{v3})+(if (self.scalar_static_f64[305]!=0.0){((v2321*(if (self.scalar_static_f64[305]!=0.0){((v2318*((self.scalar_static_f64[381]*v3301)/self.scalar_static_f64[588]))+(v2317*((-(v413*v3304))/v6200)))}else{v3}))+(v2320*(self.scalar_static_f64[846]*v4728)))}else{v3})))))}else{v3}));
        let v6643=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2326*((if (self.scalar_static_f64[305]!=0.0){(v2320*(self.scalar_static_f64[846]*v4729))}else{v3})+v6237))}else{v3}));
        let v6644=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2326*((if (self.scalar_static_f64[305]!=0.0){(v2320*(self.scalar_static_f64[846]*v4730))}else{v3})+v6237))}else{v3}));
        let v6645=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[305]!=0.0){(v2326*(if (self.scalar_static_f64[305]!=0.0){(v5877/self.scalar_static_f64[851])}else{v3}))}else{v3}));
        let v6698=(self.scalar_static_f64[0]*(v5853+(if (self.scalar_static_f64[302]!=0.0){((v2286*v4415)+v6123)}else{v3})));
        let v6699=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5799/self.scalar_static_f64[507]))*v5811)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[324]-v5799))))+self.scalar_static_f64[912]))))+(if (self.scalar_static_f64[302]!=0.0){((v2286*v4416)+(v1715*(if self.scalar_static_bool[65]{(((v2284*(self.scalar_static_f64[859]*v2535))-(v2280*((v425*(if v2273{(v2274*self.scalar_static_f64[866])}else{(if v2269{(v2270*self.scalar_static_f64[866])}else{v3})}))/v6096)))/v6104)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[858]*((self.scalar_static_f64[845]*(if self.scalar_static_bool[64]{(((v2246*v5985)-(v2243*(v5985/v5988)))/v5996)}else{v3}))+(self.scalar_static_f64[854]*(if self.scalar_static_bool[64]{(((v2253*v6019)-(v2250*(v6019/v6022)))/v6030)}else{v3}))))/self.scalar_static_f64[763])}else{v3})})))}else{v3})));
        let v6700=(self.scalar_static_f64[0]*(if (self.scalar_static_f64[302]!=0.0){(v2286*v4417)}else{v3}));
        let v6701=(self.scalar_static_f64[0]*(v5853+(if (self.scalar_static_f64[302]!=0.0){(v6123+(v2286*v4418))}else{v3})));
        let v6702=(self.scalar_static_f64[0]*(v5855+(if (self.scalar_static_f64[302]!=0.0){((v2286*v4419)+v6132)}else{v3})));
        let v6703=(self.scalar_static_f64[0]*(v5855+(if (self.scalar_static_f64[302]!=0.0){(v6132+(v2286*v4420))}else{v3})));
        let v6704=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*(self.scalar_static_f64[872]+(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5801/self.scalar_static_f64[507]))*v5811)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[321]-v5801))))))))+(if (self.scalar_static_f64[302]!=0.0){((v2286*v4421)+(v1715*(if self.scalar_static_bool[65]{(((v2284*(self.scalar_static_f64[859]*v2537))-(v2280*((v425*(if v2273{(v2274*self.scalar_static_f64[861])}else{(if v2269{(v2270*self.scalar_static_f64[861])}else{v3})}))/v6096)))/v6104)}else{(if self.scalar_static_bool[64]{((self.scalar_static_f64[858]*((self.scalar_static_f64[845]*(if self.scalar_static_bool[64]{(((v2246*v5987)-(v2243*(v5987/v5988)))/v5996)}else{v3}))+(self.scalar_static_f64[854]*(if self.scalar_static_bool[64]{(((v2253*v6021)-(v2250*(v6021/v6022)))/v6030)}else{v3}))))/self.scalar_static_f64[763])}else{v3})})))}else{v3})));
        let v6705=(self.scalar_static_f64[0]*(v5855+(if (self.scalar_static_f64[302]!=0.0){(v6132+(v2286*v4422))}else{v3})));
        let v6745=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*(self.scalar_static_f64[871]+(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5700/self.scalar_static_f64[507]))*v5713)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[0]-v5700))))))))+(if (self.scalar_static_f64[302]!=0.0){(self.scalar_static_f64[7]*v5968)}else{v5968})));
        let v6746=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5701/self.scalar_static_f64[507]))*v5713)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[322]-v5701))))+self.scalar_static_f64[908]))))+(if (self.scalar_static_f64[302]!=0.0){(self.scalar_static_f64[7]*v5969)}else{v5969})));
        let v6747=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*((self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5702/self.scalar_static_f64[507]))*v5713)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[323]-v5702))))+self.scalar_static_f64[909]))))+(if (self.scalar_static_f64[302]!=0.0){(self.scalar_static_f64[7]*v5970)}else{v5970})));
        let v6748=(self.scalar_static_f64[0]*((self.scalar_static_f64[7]*(self.scalar_static_f64[296]*(self.scalar_static_f64[540]*(self.scalar_static_f64[872]+(self.scalar_static_f64[809]*((self.scalar_static_f64[814]*(-((-(v5703/self.scalar_static_f64[507]))*v5713)))+(self.scalar_static_f64[810]*(self.scalar_static_f64[321]-v5703))))))))+(if (self.scalar_static_f64[302]!=0.0){(self.scalar_static_f64[7]*v5971)}else{v5971})));

        CommonStampValues {
            v1,
            v3,
            v32,
            v33,
            v48,
            v160,
            v409,
            v413,
            v425,
            v451,
            v699,
            v703,
            v705,
            v710,
            v713,
            v718,
            v726,
            v729,
            v732,
            v736,
            v773,
            v774,
            v776,
            v779,
            v780,
            v864,
            v986,
            v1046,
            v1071,
            v1074,
            v1077,
            v1104,
            v1184,
            v1220,
            v1221,
            v1226,
            v1227,
            v1246,
            v1248,
            v1251,
            v1252,
            v1261,
            v1293,
            v1295,
            v1297,
            v1302,
            v1303,
            v1310,
            v1311,
            v1313,
            v1318,
            v1320,
            v1372,
            v1374,
            v1376,
            v1381,
            v1382,
            v1409,
            v1422,
            v1435,
            v1448,
            v1455,
            v1456,
            v1459,
            v1461,
            v1466,
            v1467,
            v1473,
            v1477,
            v1480,
            v1488,
            v1489,
            v1490,
            v1492,
            v1494,
            v1498,
            v1499,
            v1501,
            v1504,
            v1506,
            v1507,
            v1512,
            v1513,
            v1551,
            v1553,
            v1555,
            v1556,
            v1559,
            v1561,
            v1566,
            v1567,
            v1572,
            v1575,
            v1577,
            v1585,
            v1586,
            v1587,
            v1589,
            v1594,
            v1595,
            v1597,
            v1599,
            v1601,
            v1602,
            v1607,
            v1608,
            v1676,
            v1693,
            v1715,
            v1787,
            v1799,
            v1812,
            v1813,
            v1814,
            v1817,
            v1818,
            v1822,
            v1823,
            v1825,
            v1829,
            v1831,
            v1836,
            v1837,
            v1852,
            v1959,
            v1960,
            v1962,
            v1964,
            v1966,
            v1968,
            v1969,
            v1971,
            v1979,
            v1982,
            v1983,
            v1984,
            v1990,
            v1992,
            v1993,
            v1997,
            v1999,
            v2002,
            v2004,
            v2009,
            v2010,
            v2350,
            v2382,
            v2425,
            v2428,
            v2431,
            v2434,
            v2438,
            v2442,
            v2450,
            v2456,
            v2467,
            v2509,
            v2510,
            v2511,
            v2512,
            v2630,
            v2631,
            v2632,
            v2919,
            v2920,
            v2921,
            v3067,
            v3068,
            v3069,
            v3110,
            v3111,
            v3112,
            v3119,
            v3120,
            v3121,
            v3128,
            v3129,
            v3130,
            v3162,
            v3163,
            v3342,
            v3343,
            v3344,
            v3434,
            v3435,
            v3436,
            v3437,
            v3440,
            v3443,
            v3446,
            v3449,
            v3450,
            v3451,
            v3452,
            v3454,
            v3458,
            v3461,
            v3495,
            v3496,
            v3555,
            v3556,
            v3689,
            v3690,
            v3691,
            v3746,
            v3747,
            v3748,
            v3761,
            v3762,
            v3763,
            v3784,
            v3785,
            v3786,
            v3787,
            v3788,
            v3805,
            v3806,
            v3807,
            v3808,
            v3809,
            v4267,
            v4268,
            v4269,
            v4270,
            v4283,
            v4284,
            v4285,
            v4286,
            v4287,
            v4288,
            v4289,
            v4290,
            v4415,
            v4416,
            v4417,
            v4418,
            v4419,
            v4420,
            v4421,
            v4422,
            v4727,
            v4728,
            v4729,
            v4730,
            v6426,
            v6427,
            v6428,
            v6429,
            v6430,
            v6431,
            v6597,
            v6598,
            v6599,
            v6600,
            v6601,
            v6602,
            v6616,
            v6617,
            v6622,
            v6623,
            v6624,
            v6625,
            v6626,
            v6627,
            v6640,
            v6641,
            v6642,
            v6643,
            v6644,
            v6645,
            v6698,
            v6699,
            v6700,
            v6701,
            v6702,
            v6703,
            v6704,
            v6705,
            v6745,
            v6746,
            v6747,
            v6748,
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
        let v777=(common.v774).exp();
        let v1249=(common.v1246).exp();
        let v1256=(if common.v1251{(common.v1252*(common.v1+(common.v1246-self.scalar_static_f64[198])))}else{(if (common.v1248!=0.0){v1249}else{common.v3})});
        let v1263=(if (common.v705<self.scalar_static_f64[228]){common.v1}else{common.v3});
        let v1264=(common.v1261).exp();
        let v1265=(common.v1+v1264);
        let v1270=(!(v1263!=0.0));
        let v1272=((-common.v1261)).exp();
        let v1273=(common.v1+v1272);
        let v1277=(if v1270{(self.scalar_static_f64[228]-(common.v32*(v1273).ln()))}else{(if (v1263!=0.0){(common.v705-(common.v32*(v1265).ln()))}else{common.v3})});
        let v1279=(v1277*self.scalar_static_f64[229]);
        let v1280=(self.scalar_static_f64[228]-v1277);
        let v1281=f64::powf(v1280,common.v33);
        let v1298=((self.scalar_static_f64[149]!=0.0)&&(common.v1297!=0.0));
        let v1299=(common.v1295).exp();
        let v1307=(if common.v1302{(common.v1303*(common.v1+(common.v1295-self.scalar_static_f64[198])))}else{(if v1298{v1299}else{common.v1246})});
        let v1314=((self.scalar_static_f64[149]!=0.0)&&(common.v1313!=0.0));
        let v1315=(common.v1310).exp();
        let v1324=(if common.v1318{(common.v1320*(common.v1+(common.v1310-common.v1311)))}else{(if v1314{v1315}else{v1256})});
        let v1325=(common.v1293-common.v1);
        let v1326=(self.scalar_static_f64[661]*v1325);
        let v1328=(v1325*self.scalar_static_f64[822]);
        let v1331=((common.v1+(common.v425*v1307))).sqrt();
        let v1332=(common.v1+v1331);
        let v1333=(v1328/v1332);
        let v1334=(common.v1+common.v1184);
        let v1338=(self.scalar_static_f64[676]*(common.v1046-common.v1));
        let v1339=(v1324*v1338);
        let v1340=(common.v1+v1324);
        let v1356=(self.scalar_static_f64[230]*((common.v1046+common.v1293)-common.v33));
        let v1377=((self.scalar_static_f64[149]!=0.0)&&(common.v1376!=0.0));
        let v1378=(common.v1374).exp();
        let v1387=(common.v1372-common.v1);
        let v1388=(self.scalar_static_f64[667]*v1387);
        let v1390=(v1387*self.scalar_static_f64[823]);
        let v1393=((common.v1+(common.v425*(if common.v1381{(common.v1382*(common.v1+(common.v1374-self.scalar_static_f64[198])))}else{(if v1377{v1378}else{v1307})})))).sqrt();
        let v1394=(common.v1+v1393);
        let v1437=(self.scalar_static_f64[653]*(common.v1435-common.v1));
        let v1462=((common.v1455!=0.0)&&(common.v1461!=0.0));
        let v1463=(common.v1459).exp();
        let v1471=(if common.v1466{(common.v1467*(common.v1+(common.v1459-self.scalar_static_f64[198])))}else{(if v1462{v1463}else{common.v3})});
        let v1508=((common.v1506!=0.0)&&common.v1507);
        let v1509=(common.v1501).exp();
        let v1518=(-common.v705);
        let v1519=(common.v1-(if common.v1512{(common.v1513*(common.v1+(common.v1501-self.scalar_static_f64[198])))}else{(if v1508{v1509}else{common.v3})}));
        let v1521=(common.v1+(v1519/common.v1501));
        let v1525=((common.v1455!=0.0)&&(!(common.v1504!=0.0)));
        let v1526=(common.v413*common.v705);
        let v1527=(common.v1501*v1526);
        let v1528=0.3333333333333333;
        let v1529=(common.v1501*v1528);
        let v1530=0.25;
        let v1532=(common.v1+(common.v1501*v1530));
        let v1534=(common.v1+(v1529*v1532));
        let v1538=((if v1525{(v1527*v1534)}else{(if common.v1507{(v1518*v1521)}else{common.v3})})*self.scalar_static_f64[824]);
        let v1539=(common.v1104*v1538);
        let v1544=(!(common.v1455!=0.0));
        let v1562=((common.v1551!=0.0)&&(common.v1561!=0.0));
        let v1563=(common.v1559).exp();
        let v1571=(if common.v1566{(common.v1567*(common.v1+(common.v1559-self.scalar_static_f64[198])))}else{(if v1562{v1563}else{common.v3})});
        let v1603=((common.v1601!=0.0)&&common.v1602);
        let v1604=(common.v1597).exp();
        let v1613=(-common.v699);
        let v1614=(common.v1-(if common.v1607{(common.v1608*(common.v1+(common.v1597-self.scalar_static_f64[198])))}else{(if v1603{v1604}else{common.v3})}));
        let v1616=(common.v1+(v1614/common.v1597));
        let v1620=((common.v1551!=0.0)&&(!(common.v1599!=0.0)));
        let v1621=(common.v413*common.v699);
        let v1622=(common.v1597*v1621);
        let v1623=(v1528*common.v1597);
        let v1625=(common.v1+(v1530*common.v1597));
        let v1627=(common.v1+(v1623*v1625));
        let v1631=((if v1620{(v1622*v1627)}else{(if common.v1602{(v1613*v1616)}else{common.v3})})*self.scalar_static_f64[825]);
        let v1632=(common.v1555*v1631);
        let v1637=(!(common.v1551!=0.0));
        let v1638=(if v1637{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[53]*(self.scalar_static_f64[529]*(v1571*v1632)))}else{common.v3})});
        let v1652=(self.scalar_static_f64[826]*(common.v773-common.v1));
        let v1657=((common.v1+(common.v773*self.scalar_static_f64[828]))).sqrt();
        let v1658=(common.v1+v1657);
        let v1659=(v1652/v1658);
        let v1666=(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[7]*v1659)}else{v1659});
        let v1717=(if (self.scalar_static_f64[242]!=0.0){(common.v1676*common.v1715)}else{common.v3});
        let v1722=(if (self.scalar_static_f64[248]!=0.0){(common.v699+common.v710)}else{common.v3});
        let v1724=(-v1722);
        let v1728=(if (v1724<common.v3){common.v1}else{common.v3});
        let v1729=((self.scalar_static_f64[248]!=0.0)&&(v1728!=0.0));
        let v1732=((self.scalar_static_f64[249]+(if (self.scalar_static_f64[248]!=0.0){(v1722*v1722)}else{common.v1693}))).sqrt();
        let v1733=(v1732-v1724);
        let v1737=((self.scalar_static_f64[248]!=0.0)&&(!(v1728!=0.0)));
        let v1740=(if v1737{(common.v413*(v1724+v1732))}else{(if v1729{(self.scalar_static_f64[250]/v1733)}else{common.v3})});
        let v1757=(if (v1740<self.scalar_static_f64[258]){common.v1}else{common.v3});
        let v1758=((self.scalar_static_f64[248]!=0.0)&&(v1757!=0.0));
        let v1759=(v1740/self.scalar_static_f64[256]);
        let v1761=(common.v1-f64::powf(v1759,self.scalar_static_f64[251]));
        let v1765=((self.scalar_static_f64[248]!=0.0)&&(!(v1757!=0.0)));
        let v1771=(if self.scalar_static_bool[48]{common.v1}else{(if v1765{(self.scalar_static_f64[255]+(self.scalar_static_f64[265]*(v1740-self.scalar_static_f64[258])))}else{(if v1758{(common.v1/v1761)}else{common.v3})})});
        let v1788=(common.v1220*common.v1787);
        let v1789=(self.scalar_static_f64[549]/v1788);
        let v1791=(if (v1789<self.scalar_static_f64[16]){common.v1}else{common.v3});
        let v1793=(common.v160*(if (v1791!=0.0){self.scalar_static_f64[16]}else{v1789}));
        let v1796=(common.v710+(self.scalar_static_f64[795]*((if common.v779{(common.v780*(common.v1+(common.v774-self.scalar_static_f64[198])))}else{(if (common.v776!=0.0){v777}else{common.v3})})-common.v1)));
        let v1832=(common.v1812&&(common.v1831!=0.0));
        let v1833=(common.v1829).exp();
        let v1841=(if common.v1836{(common.v1837*(common.v1+(common.v1829-self.scalar_static_f64[198])))}else{(if v1832{v1833}else{common.v3})});
        let v1844=(common.v1825*self.scalar_static_f64[839]);
        let v1854=(((if (common.v699<self.scalar_static_f64[469]){common.v1}else{common.v3})!=0.0)&&((self.scalar_static_f64[272]!=0.0)&&common.v1852));
        let v1860=(if v1854{self.scalar_static_f64[277]}else{common.v3});
        let v1861=(self.scalar_static_f64[469]-common.v699);
        let v1863=(if v1854{(v1861/common.v1077)}else{common.v986});
        let v1866=(((common.v33*v1863)/v1860)).sqrt();
        let v1867=(if v1854{v1866}else{common.v3});
        let v1871=(v1854&&(self.scalar_static_f64[279]!=0.0));
        let v1874=(v1854&&self.scalar_static_bool[53]);
        let v1877=(if v1874{(common.v1-(common.v413*common.v1071))}else{common.v3});
        let v1878=(self.scalar_static_f64[275]*v1877);
        let v1880=(if v1874{(v1877*v1878)}else{(if v1871{self.scalar_static_f64[275]}else{common.v3})});
        let v1881=(v1867*v1880);
        let v1885=(((v1867*v1867)+(v1880*v1880))).sqrt();
        let v1887=(if v1854{(v1881/v1885)}else{common.v3});
        let v1889=(if v1854{(v1861/v1887)}else{common.v3});
        let v1890=(common.v413*v1887);
        let v1891=(v1860*v1890);
        let v1894=(if v1854{(v1889+(common.v1077*v1891))}else{common.v3});
        let v1907=(self.scalar_static_f64[201]*(if v1874{(common.v1+(self.scalar_static_f64[281]*(common.v1+(common.v33*common.v1071))))}else{common.v3}));
        let v1909=((if v1874{self.scalar_static_f64[284]}else{common.v3})-(common.v1227/v1907));
        let v1912=(if v1874{(v1889-(v1891*v1909))}else{common.v3});
        let v1913=(v1912-v1894);
        let v1915=(common.v48*v1889);
        let v1916=(v1889*v1915);
        let v1922=((if v1874{((v1913*v1913)+((common.v1074*v1916)/self.scalar_static_f64[201]))}else{v1863})).sqrt();
        let v1925=(if v1874{(common.v413*((v1894+v1912)+v1922))}else{(if v1871{v1894}else{common.v3})});
        let v1926=(v1925-v1889);
        let v1928=(if v1854{(v1926/v1925)}else{common.v3});
        let v1932=(if ((v1928).abs()>1e-7){common.v1}else{common.v3});
        let v1933=(v1854&&(v1932!=0.0));
        let v1935=(if v1933{(v1890/v1928)}else{common.v3});
        let v1937=(v1925*self.scalar_static_f64[840]);
        let v1938=(v1935*v1937);
        let v1940=(self.scalar_static_f64[841]/v1925);
        let v1941=(v1940).exp();
        let v1943=(common.v1+(v1880/v1935));
        let v1945=((v1940*v1943)).exp();
        let v1946=(v1941-v1945);
        let v1950=(v1854&&(!(v1932!=0.0)));
        let v1951=(self.scalar_static_f64[4]*v1880);
        let v2005=(common.v1959&&(common.v2004!=0.0));
        let v2006=(common.v2002).exp();
        let v2014=(if common.v2009{(common.v2010*(common.v1+(common.v2002-self.scalar_static_f64[198])))}else{(if v2005{v2006}else{v1841})});
        let v2015=(common.v1823*self.scalar_static_f64[839]);
        let v2017=(if common.v1959{(v2014*v2015)}else{(if v1950{(v1941*v1951)}else{(if v1933{(v1938*v1946)}else{(if common.v1812{(v1841*v1844)}else{common.v3})})})});
        let v2023=((common.v1799!=0.0)&&((if (v2017>common.v3){common.v1}else{common.v3})!=0.0));
        let v2024=((self.scalar_static_f64[292]!=0.0)&&v2023);
        let v2025=(self.scalar_static_f64[554]+v1793);
        let v2026=(common.v1227*v2025);
        let v2033=(if v2024{(((self.scalar_static_f64[380]/v2026)+(self.scalar_static_f64[661]*(common.v1221/self.scalar_static_f64[633])))+(self.scalar_static_f64[546]/v2025))}else{common.v3});
        let v2034=((self.scalar_static_f64[285]!=0.0)&&v2024);
        let v2037=(if v2034{((v2017-v2033)/common.v409)}else{common.v1979});
        let v2039=(if (v2017<v2033){common.v1}else{common.v3});
        let v2040=(v2034&&(v2039!=0.0));
        let v2041=(v2037).exp();
        let v2042=(common.v1+v2041);
        let v2048=(v2034&&(!(v2039!=0.0)));
        let v2050=((-v2037)).exp();
        let v2051=(common.v1+v2050);
        let v2055=(if v2048{(v2033-(common.v409*(v2051).ln()))}else{(if v2040{(v2017-(common.v409*(v2042).ln()))}else{v2017})});
        let v2056=(common.v1227*v2055);
        let v2059=(v2024&&self.scalar_static_bool[57]);
        let v2060=(v2033*v2056);
        let v2061=(v2033+v2055);
        let v2065=(v2023&&self.scalar_static_bool[58]);
        let v2066=(if v2065{v2056}else{(if v2059{(v2060/v2061)}else{(if v2034{v2056}else{common.v3})})});
        let v2358=(if self.scalar_static_bool[69]{common.v3}else{(if (self.scalar_static_f64[312]!=0.0){((v2066/common.v2350)).abs()}else{common.v3})});
        let v2412=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v1771))));
        let v2426=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2425);
        let v2429=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2428);
        let v2432=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2431);
        let v2435=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2434);
        let v2439=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2438);
        let v2443=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2442);
        let v2451=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2450);
        let v2457=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2456);
        let v2468=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2467);
        let v3462=((common.v3458-(common.v1226*common.v3440))/common.v3461);
        let v3466=(((common.v1221*(common.v3454-common.v3450))-(common.v1226*common.v3443))/common.v3461);
        let v3470=(((common.v1221*(-common.v3451))-(common.v1226*common.v3446))/common.v3461);
        let v3474=(((common.v1221*(-common.v3452))-(common.v1226*common.v3449))/common.v3461);
        let v3497=(common.v3495/self.scalar_static_f64[227]);
        let v3498=(common.v3496/self.scalar_static_f64[227]);
        let v3505=(if common.v1251{(common.v1252*v3497)}else{(if (common.v1248!=0.0){(v1249*v3497)}else{common.v3})});
        let v3506=(if common.v1251{(common.v1252*v3498)}else{(if (common.v1248!=0.0){(v1249*v3498)}else{common.v3})});
        let v3531=(if v1270{(-(common.v32*((v1272*self.scalar_static_f64[336])/v1273)))}else{(if (v1263!=0.0){(self.scalar_static_f64[321]-(common.v32*((v1264*self.scalar_static_f64[334])/v1265)))}else{common.v3})});
        let v3532=(if v1270{(-(common.v32*((v1272*self.scalar_static_f64[337])/v1273)))}else{(if (v1263!=0.0){(self.scalar_static_f64[0]-(common.v32*((v1264*self.scalar_static_f64[335])/v1265)))}else{common.v3})});
        let v3538=(common.v33*f64::powf(v1280,common.v1));
        let v3563=(if common.v1302{(common.v1303*self.scalar_static_f64[861])}else{(if v1298{(v1299*self.scalar_static_f64[861])}else{v3497})});
        let v3564=(if common.v1302{(common.v1303*self.scalar_static_f64[860])}else{(if v1298{(v1299*self.scalar_static_f64[860])}else{v3498})});
        let v3565=(v3462/self.scalar_static_f64[633]);
        let v3566=(v3466/self.scalar_static_f64[633]);
        let v3567=(v3470/self.scalar_static_f64[633]);
        let v3568=(v3474/self.scalar_static_f64[633]);
        let v3581=(if common.v1318{(common.v1320*v3565)}else{(if v1314{(v1315*v3565)}else{v3505})});
        let v3582=(if common.v1318{(common.v1320*v3566)}else{(if v1314{(v1315*v3566)}else{v3506})});
        let v3583=(if common.v1318{(common.v1320*v3567)}else{(if v1314{(v1315*v3567)}else{common.v3})});
        let v3584=(if common.v1318{(common.v1320*v3568)}else{(if v1314{(v1315*v3568)}else{common.v3})});
        let v3585=(self.scalar_static_f64[661]*common.v3555);
        let v3586=(self.scalar_static_f64[661]*common.v3556);
        let v3591=(common.v33*v1331);
        let v3597=(v1332*v1332);
        let v3627=(v1340*v1340);
        let v3702=(self.scalar_static_f64[667]*common.v3689);
        let v3703=(self.scalar_static_f64[667]*common.v3690);
        let v3704=(self.scalar_static_f64[667]*common.v3691);
        let v3711=(common.v33*v1393);
        let v3718=(v1394*v1394);
        let v3819=(common.v1456*common.v1456);
        let v3826=(self.scalar_static_f64[715]*(-((-(self.scalar_static_f64[20]*(common.v33*common.v3162)))/v3819)));
        let v3827=(self.scalar_static_f64[715]*(-((-(self.scalar_static_f64[20]*(common.v33*common.v3163)))/v3819)));
        let v3838=(if (common.v1455!=0.0){self.scalar_static_f64[888]}else{common.v3});
        let v3839=(if (common.v1455!=0.0){self.scalar_static_f64[889]}else{common.v3});
        let v3840=(common.v1473*v3838);
        let v3842=(common.v1473*v3839);
        let v3844=(common.v33*common.v1477);
        let v3849=(self.scalar_static_f64[233]*f64::powf(common.v1477,self.scalar_static_f64[338]));
        let v3895=(common.v1499*common.v1499);
        let v3901=(if (common.v1455!=0.0){(((common.v1499*self.scalar_static_f64[890])-(common.v1498*(self.scalar_static_f64[405]*(if (common.v1455!=0.0){(common.v1494*((common.v1492*(((v3840+v3840)/v3844)*v3849))+(common.v1480*((self.scalar_static_f64[18]*(-(self.scalar_static_f64[236]*(common.v160*v3838))))-((common.v1490*((common.v1488*v3838)+(common.v1473*(common.v451*v3838))))+(common.v1489*v3838))))))}else{common.v3}))))/v3895)}else{v3838});
        let v3902=(if (common.v1455!=0.0){(((common.v1499*self.scalar_static_f64[891])-(common.v1498*(self.scalar_static_f64[405]*(if (common.v1455!=0.0){(common.v1494*((common.v1492*(((v3842+v3842)/v3844)*v3849))+(common.v1480*((self.scalar_static_f64[18]*(-(self.scalar_static_f64[236]*(common.v160*v3839))))-((common.v1490*((common.v1488*v3839)+(common.v1473*(common.v451*v3839))))+(common.v1489*v3839))))))}else{common.v3}))))/v3895)}else{v3839});
        let v3916=(common.v1501*common.v1501);
        let v3983=(self.scalar_static_f64[224]*f64::powf(common.v1553,self.scalar_static_f64[329]));
        let v3986=(if (common.v1551!=0.0){(self.scalar_static_f64[894]*v3983)}else{common.v3});
        let v3987=(if (common.v1551!=0.0){(self.scalar_static_f64[895]*v3983)}else{common.v3});
        let v3992=(common.v1556*common.v1556);
        let v3999=(self.scalar_static_f64[735]*(-((-(self.scalar_static_f64[52]*(common.v33*v3986)))/v3992)));
        let v4000=(self.scalar_static_f64[735]*(-((-(self.scalar_static_f64[52]*(common.v33*v3987)))/v3992)));
        let v4009=(if (common.v1551!=0.0){self.scalar_static_f64[892]}else{common.v3});
        let v4010=(if (common.v1551!=0.0){self.scalar_static_f64[893]}else{common.v3});
        let v4011=(common.v1572*v4009);
        let v4013=(common.v1572*v4010);
        let v4015=(common.v33*common.v1575);
        let v4020=(self.scalar_static_f64[237]*f64::powf(common.v1575,self.scalar_static_f64[343]));
        let v4066=(common.v1595*common.v1595);
        let v4072=(if (common.v1551!=0.0){(((common.v1595*self.scalar_static_f64[896])-(common.v1594*(self.scalar_static_f64[426]*(if (common.v1551!=0.0){(common.v1494*((common.v1589*(((v4011+v4011)/v4015)*v4020))+(common.v1577*((self.scalar_static_f64[50]*(-(self.scalar_static_f64[240]*(common.v160*v4009))))-((common.v1587*((common.v1585*v4009)+(common.v1572*(common.v451*v4009))))+(common.v1586*v4009))))))}else{common.v3}))))/v4066)}else{v4009});
        let v4073=(if (common.v1551!=0.0){(((common.v1595*self.scalar_static_f64[897])-(common.v1594*(self.scalar_static_f64[426]*(if (common.v1551!=0.0){(common.v1494*((common.v1589*(((v4013+v4013)/v4015)*v4020))+(common.v1577*((self.scalar_static_f64[50]*(-(self.scalar_static_f64[240]*(common.v160*v4010))))-((common.v1587*((common.v1585*v4010)+(common.v1572*(common.v451*v4010))))+(common.v1586*v4010))))))}else{common.v3}))))/v4066)}else{v4010});
        let v4087=(common.v1597*common.v1597);
        let v4207=(common.v33*v1657);
        let v4215=(v1658*v1658);
        let v4216=(((v1658*(self.scalar_static_f64[826]*common.v2509))-(v1652*((self.scalar_static_f64[828]*common.v2509)/v4207)))/v4215);
        let v4220=(((v1658*(self.scalar_static_f64[826]*common.v2510))-(v1652*((self.scalar_static_f64[828]*common.v2510)/v4207)))/v4215);
        let v4224=(((v1658*(self.scalar_static_f64[826]*common.v2511))-(v1652*((self.scalar_static_f64[828]*common.v2511)/v4207)))/v4215);
        let v4228=(((v1658*(self.scalar_static_f64[826]*common.v2512))-(v1652*((self.scalar_static_f64[828]*common.v2512)/v4207)))/v4215);
        let v4423=(common.v1715*common.v4267);
        let v4432=(common.v1715*common.v4269);
        let v4456=(v1722*self.scalar_static_f64[350]);
        let v4458=(v1722*self.scalar_static_f64[351]);
        let v4460=(v1722*self.scalar_static_f64[352]);
        let v4471=(common.v33*v1732);
        let v4472=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4283})/v4471);
        let v4473=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4284})/v4471);
        let v4474=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4285})/v4471);
        let v4475=((if (self.scalar_static_f64[248]!=0.0){(v4456+v4456)}else{common.v4283})/v4471);
        let v4476=((if (self.scalar_static_f64[248]!=0.0){(v4458+v4458)}else{common.v4286})/v4471);
        let v4477=((if (self.scalar_static_f64[248]!=0.0){(v4460+v4460)}else{common.v4287})/v4471);
        let v4478=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4288})/v4471);
        let v4479=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4289})/v4471);
        let v4480=((if (self.scalar_static_f64[248]!=0.0){common.v3}else{common.v4290})/v4471);
        let v4486=(v1733*v1733);
        let v4533=(if v1737{(common.v413*v4472)}else{(if v1729{((-(self.scalar_static_f64[250]*v4472))/v4486)}else{common.v3})});
        let v4534=(if v1737{(common.v413*v4473)}else{(if v1729{((-(self.scalar_static_f64[250]*v4473))/v4486)}else{common.v3})});
        let v4535=(if v1737{(common.v413*v4474)}else{(if v1729{((-(self.scalar_static_f64[250]*v4474))/v4486)}else{common.v3})});
        let v4536=(if v1737{(common.v413*(self.scalar_static_f64[353]+v4475))}else{(if v1729{((-(self.scalar_static_f64[250]*(v4475-self.scalar_static_f64[353])))/v4486)}else{common.v3})});
        let v4537=(if v1737{(common.v413*(self.scalar_static_f64[354]+v4476))}else{(if v1729{((-(self.scalar_static_f64[250]*(v4476-self.scalar_static_f64[354])))/v4486)}else{common.v3})});
        let v4538=(if v1737{(common.v413*(self.scalar_static_f64[355]+v4477))}else{(if v1729{((-(self.scalar_static_f64[250]*(v4477-self.scalar_static_f64[355])))/v4486)}else{common.v3})});
        let v4539=(if v1737{(common.v413*v4478)}else{(if v1729{((-(self.scalar_static_f64[250]*v4478))/v4486)}else{common.v3})});
        let v4540=(if v1737{(common.v413*v4479)}else{(if v1729{((-(self.scalar_static_f64[250]*v4479))/v4486)}else{common.v3})});
        let v4541=(if v1737{(common.v413*v4480)}else{(if v1729{((-(self.scalar_static_f64[250]*v4480))/v4486)}else{common.v3})});
        let v4552=(self.scalar_static_f64[251]*f64::powf(v1759,self.scalar_static_f64[260]));
        let v4562=(v1761*v1761);
        let v4599=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4533)}else{(if v1758{(((v4533/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4600=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4534)}else{(if v1758{(((v4534/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4601=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4535)}else{(if v1758{(((v4535/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4602=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4536)}else{(if v1758{(((v4536/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4603=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4537)}else{(if v1758{(((v4537/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4604=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4538)}else{(if v1758{(((v4538/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4605=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4539)}else{(if v1758{(((v4539/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4606=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4540)}else{(if v1758{(((v4540/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4607=(if self.scalar_static_bool[48]{common.v3}else{(if v1765{(self.scalar_static_f64[265]*v4541)}else{(if v1758{(((v4541/self.scalar_static_f64[256])*v4552)/v4562)}else{common.v3})})});
        let v4630=(v1771*(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[7]*v4224)}else{v4224}));
        let v4650=(v1771*(self.scalar_static_f64[653]*common.v3787));
        let v4659=(v1771*(if (self.scalar_static_f64[242]!=0.0){(v4423+(common.v1676*common.v4415))}else{common.v3}));
        let v4745=(v1788*v1788);
        let v4760=(common.v160*(if (v1791!=0.0){common.v3}else{((-(self.scalar_static_f64[549]*((common.v1787*common.v3434)+(common.v1220*common.v4727))))/v4745)}));
        let v4761=(common.v160*(if (v1791!=0.0){common.v3}else{((-(self.scalar_static_f64[549]*((common.v1787*common.v3435)+(common.v1220*common.v4728))))/v4745)}));
        let v4762=(common.v160*(if (v1791!=0.0){common.v3}else{((-(self.scalar_static_f64[549]*((common.v1787*common.v3436)+(common.v1220*common.v4729))))/v4745)}));
        let v4763=(common.v160*(if (v1791!=0.0){common.v3}else{((-(self.scalar_static_f64[549]*((common.v1787*common.v3437)+(common.v1220*common.v4730))))/v4745)}));
        let v4770=(v1793*v1793);
        let v4787=((-v3462)/self.scalar_static_f64[269]);
        let v4788=((-v3466)/self.scalar_static_f64[269]);
        let v4789=((-v3470)/self.scalar_static_f64[269]);
        let v4790=((-v3474)/self.scalar_static_f64[269]);
        let v4815=(if common.v1812{(common.v1823*(if common.v1817{(common.v1818*v4787)}else{(if common.v1813{(common.v1814*v4787)}else{common.v3})}))}else{common.v3});
        let v4816=(if common.v1812{((common.v1823*(if common.v1817{(common.v1818*v4788)}else{(if common.v1813{(common.v1814*v4788)}else{common.v3})}))+(common.v1822*self.scalar_static_f64[321]))}else{common.v3});
        let v4817=(if common.v1812{((common.v1823*(if common.v1817{(common.v1818*v4789)}else{(if common.v1813{(common.v1814*v4789)}else{common.v3})}))+(self.scalar_static_f64[0]*common.v1822))}else{common.v3});
        let v4818=(if common.v1812{(common.v1823*(if common.v1817{(common.v1818*v4790)}else{(if common.v1813{(common.v1814*v4790)}else{common.v3})}))}else{common.v3});
        let v4821=(self.scalar_static_f64[270]*f64::powf(common.v1825,self.scalar_static_f64[356]));
        let v4826=(self.scalar_static_f64[838]*(v4815*v4821));
        let v4827=(self.scalar_static_f64[838]*(v4816*v4821));
        let v4828=(self.scalar_static_f64[838]*(v4817*v4821));
        let v4829=(self.scalar_static_f64[838]*(v4818*v4821));
        let v4842=(if common.v1836{(common.v1837*v4826)}else{(if v1832{(v1833*v4826)}else{common.v3})});
        let v4843=(if common.v1836{(common.v1837*v4827)}else{(if v1832{(v1833*v4827)}else{common.v3})});
        let v4844=(if common.v1836{(common.v1837*v4828)}else{(if v1832{(v1833*v4828)}else{common.v3})});
        let v4845=(if common.v1836{(common.v1837*v4829)}else{(if v1832{(v1833*v4829)}else{common.v3})});
        let v4869=(common.v1077*common.v1077);
        let v4878=(if v1854{(((common.v1077*self.scalar_static_f64[321])-(v1861*common.v3128))/v4869)}else{common.v2919});
        let v4879=(if v1854{(((self.scalar_static_f64[0]*common.v1077)-(v1861*common.v3129))/v4869)}else{common.v2920});
        let v4880=(if v1854{((-(v1861*common.v3130))/v4869)}else{common.v2921});
        let v4887=(common.v33*v1866);
        let v4891=(if v1854{(((common.v33*v4878)/v1860)/v4887)}else{common.v3});
        let v4892=(if v1854{(((common.v33*v4879)/v1860)/v4887)}else{common.v3});
        let v4893=(if v1854{(((common.v33*v4880)/v1860)/v4887)}else{common.v3});
        let v4900=(if v1874{(-(common.v413*common.v3110))}else{common.v3});
        let v4901=(if v1874{(-(common.v413*common.v3111))}else{common.v3});
        let v4902=(if v1874{(-(common.v413*common.v3112))}else{common.v3});
        let v4915=(if v1874{((v1878*v4900)+(v1877*(self.scalar_static_f64[275]*v4900)))}else{common.v3});
        let v4916=(if v1874{((v1878*v4901)+(v1877*(self.scalar_static_f64[275]*v4901)))}else{common.v3});
        let v4917=(if v1874{((v1878*v4902)+(v1877*(self.scalar_static_f64[275]*v4902)))}else{common.v3});
        let v4927=(v1867*v4891);
        let v4929=(v1867*v4892);
        let v4931=(v1867*v4893);
        let v4933=(v1880*v4915);
        let v4935=(v1880*v4916);
        let v4937=(v1880*v4917);
        let v4942=(common.v33*v1885);
        let v4949=(v1885*v1885);
        let v4959=(if v1854{(((v1885*((v1880*v4891)+(v1867*v4915)))-(v1881*(((v4927+v4927)+(v4933+v4933))/v4942)))/v4949)}else{common.v3});
        let v4960=(if v1854{(((v1885*((v1880*v4892)+(v1867*v4916)))-(v1881*(((v4929+v4929)+(v4935+v4935))/v4942)))/v4949)}else{common.v3});
        let v4961=(if v1854{(((v1885*((v1880*v4893)+(v1867*v4917)))-(v1881*(((v4931+v4931)+(v4937+v4937))/v4942)))/v4949)}else{common.v3});
        let v4965=(v1887*v1887);
        let v4974=(if v1854{(((v1887*self.scalar_static_f64[321])-(v1861*v4959))/v4965)}else{common.v3});
        let v4975=(if v1854{(((self.scalar_static_f64[0]*v1887)-(v1861*v4960))/v4965)}else{common.v3});
        let v4976=(if v1854{((-(v1861*v4961))/v4965)}else{common.v3});
        let v4977=(common.v413*v4959);
        let v4978=(common.v413*v4960);
        let v4979=(common.v413*v4961);
        let v4980=(v1860*v4977);
        let v4981=(v1860*v4978);
        let v4982=(v1860*v4979);
        let v4995=(if v1854{(v4974+((v1891*common.v3128)+(common.v1077*v4980)))}else{common.v3});
        let v4996=(if v1854{(v4975+((v1891*common.v3129)+(common.v1077*v4981)))}else{common.v3});
        let v4997=(if v1854{(v4976+((v1891*common.v3130)+(common.v1077*v4982)))}else{common.v3});
        let v5017=(v1907*v1907);
        let v5045=(if v1874{(-(v1891*(-(v3462/v1907))))}else{common.v3});
        let v5046=(if v1874{(v4974-((v1909*v4980)+(v1891*(-(((v1907*v3466)-(common.v1227*(self.scalar_static_f64[201]*(if v1874{(self.scalar_static_f64[281]*(common.v33*common.v3110))}else{common.v3}))))/v5017)))))}else{common.v3});
        let v5047=(if v1874{(v4975-((v1909*v4981)+(v1891*(-(((v1907*v3470)-(common.v1227*(self.scalar_static_f64[201]*(if v1874{(self.scalar_static_f64[281]*(common.v33*common.v3111))}else{common.v3}))))/v5017)))))}else{common.v3});
        let v5048=(if v1874{(v4976-((v1909*v4982)+(v1891*(-(((v1907*v3474)-(common.v1227*(self.scalar_static_f64[201]*(if v1874{(self.scalar_static_f64[281]*(common.v33*common.v3112))}else{common.v3}))))/v5017)))))}else{common.v3});
        let v5052=(v1913*v5045);
        let v5054=(v1913*(v5046-v4995));
        let v5056=(v1913*(v5047-v4996));
        let v5058=(v1913*(v5048-v4997));
        let v5094=(common.v33*v1922);
        let v5107=(if v1874{(common.v413*(v5045+((if v1874{(v5052+v5052)}else{common.v3})/v5094)))}else{common.v3});
        let v5108=(if v1874{(common.v413*((v4995+v5046)+((if v1874{((v5054+v5054)+(((v1916*common.v3119)+(common.v1074*((v1915*v4974)+(v1889*(common.v48*v4974)))))/self.scalar_static_f64[201]))}else{v4878})/v5094)))}else{(if v1871{v4995}else{common.v3})});
        let v5109=(if v1874{(common.v413*((v4996+v5047)+((if v1874{((v5056+v5056)+(((v1916*common.v3120)+(common.v1074*((v1915*v4975)+(v1889*(common.v48*v4975)))))/self.scalar_static_f64[201]))}else{v4879})/v5094)))}else{(if v1871{v4996}else{common.v3})});
        let v5110=(if v1874{(common.v413*((v4997+v5048)+((if v1874{((v5058+v5058)+(((v1916*common.v3121)+(common.v1074*((v1915*v4976)+(v1889*(common.v48*v4976)))))/self.scalar_static_f64[201]))}else{v4880})/v5094)))}else{(if v1871{v4997}else{common.v3})});
        let v5117=(v1925*v1925);
        let v5137=(v1928*v1928);
        let v5151=(if v1933{((-(v1890*(if v1854{(((v1925*v5107)-(v1926*v5107))/v5117)}else{common.v3})))/v5137)}else{common.v3});
        let v5152=(if v1933{(((v1928*v4977)-(v1890*(if v1854{(((v1925*(v5108-v4974))-(v1926*v5108))/v5117)}else{common.v3})))/v5137)}else{common.v3});
        let v5153=(if v1933{(((v1928*v4978)-(v1890*(if v1854{(((v1925*(v5109-v4975))-(v1926*v5109))/v5117)}else{common.v3})))/v5137)}else{common.v3});
        let v5154=(if v1933{(((v1928*v4979)-(v1890*(if v1854{(((v1925*(v5110-v4976))-(v1926*v5110))/v5117)}else{common.v3})))/v5137)}else{common.v3});
        let v5173=((-(self.scalar_static_f64[841]*v5107))/v5117);
        let v5176=((-(self.scalar_static_f64[841]*v5108))/v5117);
        let v5179=((-(self.scalar_static_f64[841]*v5109))/v5117);
        let v5182=((-(self.scalar_static_f64[841]*v5110))/v5117);
        let v5183=(v1941*v5173);
        let v5184=(v1941*v5176);
        let v5185=(v1941*v5179);
        let v5186=(v1941*v5182);
        let v5189=(v1935*v1935);
        let v5257=(self.scalar_static_f64[270]*f64::powf(common.v1823,self.scalar_static_f64[356]));
        let v5263=(common.v1962*common.v1962);
        let v5283=(self.scalar_static_f64[287]*f64::powf(common.v1964,self.scalar_static_f64[357]));
        let v5296=(if common.v1959{(common.v1960*((-(((common.v1962*v3462)-(common.v1227*v3462))/v5263))*v5283))}else{common.v3});
        let v5297=(if common.v1959{((common.v1966*(self.scalar_static_f64[321]*v5257))+(common.v1960*((-(((common.v1962*v3466)-(common.v1227*v3466))/v5263))*v5283)))}else{common.v3});
        let v5298=(if common.v1959{((common.v1966*(self.scalar_static_f64[0]*v5257))+(common.v1960*((-(((common.v1962*v3470)-(common.v1227*v3470))/v5263))*v5283)))}else{common.v3});
        let v5299=(if common.v1959{(common.v1960*((-(((common.v1962*v3474)-(common.v1227*v3474))/v5263))*v5283))}else{common.v3});
        let v5308=(if common.v1971{(v3462/self.scalar_static_f64[286])}else{common.v3});
        let v5309=(if common.v1971{(v3466/self.scalar_static_f64[286])}else{common.v3});
        let v5310=(if common.v1971{(v3470/self.scalar_static_f64[286])}else{common.v3});
        let v5311=(if common.v1971{(v3474/self.scalar_static_f64[286])}else{common.v3});
        let v5316=(if common.v1971{(v5308/self.scalar_static_f64[289])}else{self.scalar_static_f64[334]});
        let v5317=(if common.v1971{(v5309/self.scalar_static_f64[289])}else{self.scalar_static_f64[335]});
        let v5318=(if common.v1971{(v5310/self.scalar_static_f64[289])}else{common.v3});
        let v5319=(if common.v1971{(v5311/self.scalar_static_f64[289])}else{common.v3});
        let v5362=(self.scalar_static_f64[290]*f64::powf(common.v1997,self.scalar_static_f64[358]));
        let v5383=(self.scalar_static_f64[838]*(if common.v1971{((common.v1999*v5296)+(common.v1968*((if common.v1990{(v5308+(self.scalar_static_f64[289]*((common.v1992*(-v5316))/common.v1993)))}else{(if common.v1982{(self.scalar_static_f64[289]*((common.v1983*v5316)/common.v1984))}else{common.v3})})*v5362)))}else{(if common.v1969{v5296}else{common.v3})}));
        let v5384=(self.scalar_static_f64[838]*(if common.v1971{((common.v1999*v5297)+(common.v1968*((if common.v1990{(v5309+(self.scalar_static_f64[289]*((common.v1992*(-v5317))/common.v1993)))}else{(if common.v1982{(self.scalar_static_f64[289]*((common.v1983*v5317)/common.v1984))}else{common.v3})})*v5362)))}else{(if common.v1969{v5297}else{common.v3})}));
        let v5385=(self.scalar_static_f64[838]*(if common.v1971{((common.v1999*v5298)+(common.v1968*((if common.v1990{(v5310+(self.scalar_static_f64[289]*((common.v1992*(-v5318))/common.v1993)))}else{(if common.v1982{(self.scalar_static_f64[289]*((common.v1983*v5318)/common.v1984))}else{common.v3})})*v5362)))}else{(if common.v1969{v5298}else{common.v3})}));
        let v5386=(self.scalar_static_f64[838]*(if common.v1971{((common.v1999*v5299)+(common.v1968*((if common.v1990{(v5311+(self.scalar_static_f64[289]*((common.v1992*(-v5319))/common.v1993)))}else{(if common.v1982{(self.scalar_static_f64[289]*((common.v1983*v5319)/common.v1984))}else{common.v3})})*v5362)))}else{(if common.v1969{v5299}else{common.v3})}));
        let v5413=(if common.v1959{(v2015*(if common.v2009{(common.v2010*v5383)}else{(if v2005{(v2006*v5383)}else{v4842})}))}else{(if v1950{(v1951*v5183)}else{(if v1933{((v1946*((v1937*v5151)+(v1935*(self.scalar_static_f64[840]*v5107))))+(v1938*(v5183-(v1945*((v1943*v5173)+(v1940*((-(v1880*v5151))/v5189)))))))}else{(if common.v1812{((v1844*v4842)+(v1841*(self.scalar_static_f64[839]*v4815)))}else{common.v3})})})});
        let v5414=(if common.v1959{((v2015*(if common.v2009{(common.v2010*v5384)}else{(if v2005{(v2006*v5384)}else{v4843})}))+(v2014*self.scalar_static_f64[898]))}else{(if v1950{((v1951*v5184)+(v1941*(self.scalar_static_f64[4]*v4915)))}else{(if v1933{((v1946*((v1937*v5152)+(v1935*(self.scalar_static_f64[840]*v5108))))+(v1938*(v5184-(v1945*((v1943*v5176)+(v1940*(((v1935*v4915)-(v1880*v5152))/v5189)))))))}else{(if common.v1812{((v1844*v4843)+(v1841*(self.scalar_static_f64[839]*v4816)))}else{common.v3})})})});
        let v5415=(if common.v1959{((v2015*(if common.v2009{(common.v2010*v5385)}else{(if v2005{(v2006*v5385)}else{v4844})}))+(v2014*self.scalar_static_f64[899]))}else{(if v1950{((v1951*v5185)+(v1941*(self.scalar_static_f64[4]*v4916)))}else{(if v1933{((v1946*((v1937*v5153)+(v1935*(self.scalar_static_f64[840]*v5109))))+(v1938*(v5185-(v1945*((v1943*v5179)+(v1940*(((v1935*v4916)-(v1880*v5153))/v5189)))))))}else{(if common.v1812{((v1844*v4844)+(v1841*(self.scalar_static_f64[839]*v4817)))}else{common.v3})})})});
        let v5416=(if common.v1959{(v2015*(if common.v2009{(common.v2010*v5386)}else{(if v2005{(v2006*v5386)}else{v4845})}))}else{(if v1950{((v1951*v5186)+(v1941*(self.scalar_static_f64[4]*v4917)))}else{(if v1933{((v1946*((v1937*v5154)+(v1935*(self.scalar_static_f64[840]*v5110))))+(v1938*(v5186-(v1945*((v1943*v5182)+(v1940*(((v1935*v4917)-(v1880*v5154))/v5189)))))))}else{(if common.v1812{((v1844*v4845)+(v1841*(self.scalar_static_f64[839]*v4818)))}else{common.v3})})})});
        let v5431=(v2026*v2026);
        let v5456=(v2025*v2025);
        let v5471=(if v2024{((((-(self.scalar_static_f64[380]*((v2025*v3462)+(common.v1227*v4760))))/v5431)+(self.scalar_static_f64[661]*(common.v3440/self.scalar_static_f64[633])))+((-(self.scalar_static_f64[546]*v4760))/v5456))}else{common.v3});
        let v5472=(if v2024{((((-(self.scalar_static_f64[380]*((v2025*v3466)+(common.v1227*v4761))))/v5431)+(self.scalar_static_f64[661]*(common.v3443/self.scalar_static_f64[633])))+((-(self.scalar_static_f64[546]*v4761))/v5456))}else{common.v3});
        let v5473=(if v2024{((((-(self.scalar_static_f64[380]*((v2025*v3470)+(common.v1227*v4762))))/v5431)+(self.scalar_static_f64[661]*(common.v3446/self.scalar_static_f64[633])))+((-(self.scalar_static_f64[546]*v4762))/v5456))}else{common.v3});
        let v5474=(if v2024{((((-(self.scalar_static_f64[380]*((v2025*v3474)+(common.v1227*v4763))))/v5431)+(self.scalar_static_f64[661]*(common.v3449/self.scalar_static_f64[633])))+((-(self.scalar_static_f64[546]*v4763))/v5456))}else{common.v3});
        let v5483=(if v2034{((v5413-v5471)/common.v409)}else{v5316});
        let v5484=(if v2034{((v5414-v5472)/common.v409)}else{v5317});
        let v5485=(if v2034{((v5415-v5473)/common.v409)}else{v5318});
        let v5486=(if v2034{((v5416-v5474)/common.v409)}else{v5319});
        let v5527=(if v2048{(v5471-(common.v409*((v2050*(-v5483))/v2051)))}else{(if v2040{(v5413-(common.v409*((v2041*v5483)/v2042)))}else{v5413})});
        let v5528=(if v2048{(v5472-(common.v409*((v2050*(-v5484))/v2051)))}else{(if v2040{(v5414-(common.v409*((v2041*v5484)/v2042)))}else{v5414})});
        let v5529=(if v2048{(v5473-(common.v409*((v2050*(-v5485))/v2051)))}else{(if v2040{(v5415-(common.v409*((v2041*v5485)/v2042)))}else{v5415})});
        let v5530=(if v2048{(v5474-(common.v409*((v2050*(-v5486))/v2051)))}else{(if v2040{(v5416-(common.v409*((v2041*v5486)/v2042)))}else{v5416})});
        let v5533=((v2055*v3462)+(common.v1227*v5527));
        let v5536=((v2055*v3466)+(common.v1227*v5528));
        let v5539=((v2055*v3470)+(common.v1227*v5529));
        let v5542=((v2055*v3474)+(common.v1227*v5530));
        let v5566=(v2061*v2061);
        let v6504=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[699]*common.v3808)));
        let v6508=((((if self.scalar_static_bool[33]{(self.scalar_static_f64[661]*((self.scalar_static_f64[232]*common.v3555)+(v1334*(self.scalar_static_f64[230]*common.v3555))))}else{(if self.scalar_static_bool[31]{v3585}else{(if (self.scalar_static_f64[149]!=0.0){((v3585+(v1334*(((v1332*(self.scalar_static_f64[822]*common.v3555))-(v1328*((common.v425*v3563)/v3591)))/v3597)))+(((v1340*(v1338*v3581))-(v1339*v3581))/v3627))}else{common.v3})})})+(self.scalar_static_f64[646]*common.v3746))+self.scalar_static_f64[366])-(if v1544{common.v3}else{(if (common.v1455!=0.0){(self.scalar_static_f64[21]*(self.scalar_static_f64[528]*((v1539*(if common.v1466{(common.v1467*v3826)}else{(if v1462{(v1463*v3826)}else{common.v3})}))+(v1471*((v1538*common.v3162)+(common.v1104*(self.scalar_static_f64[824]*(if v1525{((v1534*((v1526*v3901)+(common.v1501*self.scalar_static_f64[341])))+(v1527*((v1532*(v1528*v3901))+(v1529*(v1530*v3901)))))}else{(if common.v1507{((self.scalar_static_f64[0]*v1521)+(v1518*(((common.v1501*(-(if common.v1512{(common.v1513*v3901)}else{(if v1508{(v1509*v3901)}else{common.v3})})))-(v1519*v3901))/v3916)))}else{common.v3})}))))))))}else{common.v3})}));
        let v6509=((((if self.scalar_static_bool[33]{(self.scalar_static_f64[661]*((self.scalar_static_f64[232]*common.v3556)+((v1356*common.v3342)+(v1334*(self.scalar_static_f64[230]*(common.v3067+common.v3556))))))}else{(if self.scalar_static_bool[31]{v3586}else{(if (self.scalar_static_f64[149]!=0.0){((v3586+((v1334*(((v1332*(self.scalar_static_f64[822]*common.v3556))-(v1328*((common.v425*v3564)/v3591)))/v3597))+(v1333*common.v3342)))+(((v1340*((v1338*v3582)+(v1324*(self.scalar_static_f64[676]*common.v3067))))-(v1339*v3582))/v3627))}else{common.v3})})})+(self.scalar_static_f64[646]*common.v3748))+self.scalar_static_f64[367])-(if v1544{common.v3}else{(if (common.v1455!=0.0){(self.scalar_static_f64[21]*(self.scalar_static_f64[528]*((v1539*(if common.v1466{(common.v1467*v3827)}else{(if v1462{(v1463*v3827)}else{common.v3})}))+(v1471*((v1538*common.v3163)+(common.v1104*(self.scalar_static_f64[824]*(if v1525{((v1534*((v1526*v3902)+(common.v1501*self.scalar_static_f64[342])))+(v1527*((v1532*(v1528*v3902))+(v1529*(v1530*v3902)))))}else{(if common.v1507{((v1521*self.scalar_static_f64[321])+(v1518*(((common.v1501*(-(if common.v1512{(common.v1513*v3902)}else{(if v1508{(v1509*v3902)}else{common.v3})})))-(v1519*v3902))/v3916)))}else{common.v3})}))))))))}else{common.v3})}));
        let v6542=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4599))));
        let v6543=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4600))));
        let v6544=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4601))));
        let v6545=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4602))));
        let v6546=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-((v1771*(if v1637{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[53]*(self.scalar_static_f64[529]*((v1632*(if common.v1566{(common.v1567*v3999)}else{(if v1562{(v1563*v3999)}else{common.v3})}))+(v1571*((v1631*v3986)+(common.v1555*(self.scalar_static_f64[825]*(if v1620{((v1627*((v1621*v4072)+(common.v1597*self.scalar_static_f64[342])))+(v1622*((v1625*(v1528*v4072))+(v1623*(v1530*v4072)))))}else{(if common.v1602{((v1616*self.scalar_static_f64[321])+(v1613*(((common.v1597*(-(if common.v1607{(common.v1608*v4072)}else{(if v1603{(v1604*v4072)}else{common.v3})})))-(v1614*v4072))/v4087)))}else{common.v3})}))))))))}else{common.v3})}))+(v1638*v4603)))));
        let v6547=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-((v1771*(if v1637{common.v3}else{(if (common.v1551!=0.0){(self.scalar_static_f64[53]*(self.scalar_static_f64[529]*((v1632*(if common.v1566{(common.v1567*v4000)}else{(if v1562{(v1563*v4000)}else{common.v3})}))+(v1571*((v1631*v3987)+(common.v1555*(self.scalar_static_f64[825]*(if v1620{((v1627*((v1621*v4073)+(common.v1597*self.scalar_static_f64[341])))+(v1622*((v1625*(v1528*v4073))+(v1623*(v1530*v4073)))))}else{(if common.v1602{((self.scalar_static_f64[0]*v1616)+(v1613*(((common.v1597*(-(if common.v1607{(common.v1608*v4073)}else{(if v1603{(v1604*v4073)}else{common.v3})})))-(v1614*v4073))/v4087)))}else{common.v3})}))))))))}else{common.v3})}))+(v1638*v4604)))));
        let v6548=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4605))));
        let v6549=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4606))));
        let v6550=(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(v1638*v4607))));
        let v6603=ddt_scale;
        let v6714=(self.scalar_static_f64[15]*(v6603*common.v6698));
        let v6755=(self.scalar_static_f64[15]*(v6603*common.v6747));

        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v864))),
            5,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2630))),
            6,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2631))),
            7,
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v2632))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*common.v1227))),
            [3, 5, 6, 7],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3462)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3466)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3470)), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*v3474))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[699]*(common.v1448-common.v1))+((if self.scalar_static_bool[30]{v1388}else{(if (self.scalar_static_f64[149]!=0.0){(v1388+(v1390/v1394))}else{common.v3})})+(self.scalar_static_f64[693]*(common.v1422-common.v1))))))),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[699]*common.v3805)+((if self.scalar_static_bool[30]{v3702}else{(if (self.scalar_static_f64[149]!=0.0){(v3702+(((v1394*(self.scalar_static_f64[823]*common.v3689))-(v1390*((common.v425*(if common.v1381{(common.v1382*self.scalar_static_f64[861])}else{(if v1377{(v1378*self.scalar_static_f64[861])}else{v3563})}))/v3711)))/v3718))}else{common.v3})})+(self.scalar_static_f64[693]*common.v3761))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[699]*common.v3806)+((if self.scalar_static_bool[30]{v3703}else{(if (self.scalar_static_f64[149]!=0.0){(v3703+(((v1394*(self.scalar_static_f64[823]*common.v3690))-(v1390*((common.v425*(if common.v1381{(common.v1382*self.scalar_static_f64[860])}else{(if v1377{(v1378*self.scalar_static_f64[860])}else{common.v3})}))/v3711)))/v3718))}else{common.v3})})+(self.scalar_static_f64[693]*common.v3762))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[699]*common.v3807)+((if self.scalar_static_bool[30]{v3704}else{(if (self.scalar_static_f64[149]!=0.0){(v3704+(((v1394*(self.scalar_static_f64[823]*common.v3691))-(v1390*((common.v425*(if common.v1381{common.v3}else{(if v1377{common.v3}else{v3564})}))/v3711)))/v3718))}else{common.v3})})+(self.scalar_static_f64[693]*common.v3763))))), v6504, v6504, (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[699]*common.v3809)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[704]*(v1256-common.v1))+((v1279*v1281)+((((if self.scalar_static_bool[33]{(self.scalar_static_f64[661]*((v1325*self.scalar_static_f64[232])+(v1334*v1356)))}else{(if self.scalar_static_bool[31]{v1326}else{(if (self.scalar_static_f64[149]!=0.0){((v1326+(v1333*v1334))+(v1339/v1340))}else{common.v3})})})+(self.scalar_static_f64[646]*(common.v1409-common.v1)))+(common.v3*common.v705))-(if v1544{common.v3}else{(if (common.v1455!=0.0){(self.scalar_static_f64[21]*(self.scalar_static_f64[528]*(v1471*v1539)))}else{common.v3})}))))))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[704]*v3505)+(((v1281*(self.scalar_static_f64[229]*v3531))+(v1279*((-v3531)*v3538)))+v6508)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(self.scalar_static_f64[646]*common.v3747))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[704]*v3506)+(((v1281*(self.scalar_static_f64[229]*v3532))+(v1279*((-v3532)*v3538)))+v6509)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[661]*((v1356*common.v3343)+(v1334*(self.scalar_static_f64[230]*common.v3068))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[149]!=0.0){((v1333*common.v3343)+(((v1340*((v1338*v3583)+(v1324*(self.scalar_static_f64[676]*common.v3068))))-(v1339*v3583))/v3627))}else{common.v3})})}))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[661]*((v1356*common.v3344)+(v1334*(self.scalar_static_f64[230]*common.v3069))))}else{(if self.scalar_static_bool[31]{common.v3}else{(if (self.scalar_static_f64[149]!=0.0){((v1333*common.v3344)+(((v1340*((v1338*v3584)+(v1324*(self.scalar_static_f64[676]*common.v3069))))-(v1339*v3584))/v3627))}else{common.v3})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * ((if (self.scalar_static_f64[149]!=0.0){v2412}else{common.v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if (self.scalar_static_f64[149]!=0.0){v6542}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6543}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6544}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6545}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6546}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6547}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6548}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6549}else{common.v3}), (if (self.scalar_static_f64[149]!=0.0){v6550}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[30]{v2412}else{common.v3})),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(if self.scalar_static_bool[30]{v6542}else{common.v3}), (if self.scalar_static_bool[30]{v6543}else{common.v3}), (if self.scalar_static_bool[30]{v6544}else{common.v3}), (if self.scalar_static_bool[30]{v6545}else{common.v3}), (if self.scalar_static_bool[30]{v6546}else{common.v3}), (if self.scalar_static_bool[30]{v6547}else{common.v3}), (if self.scalar_static_bool[30]{v6548}else{common.v3}), (if self.scalar_static_bool[30]{v6549}else{common.v3}), (if self.scalar_static_bool[30]{v6550}else{common.v3})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1796/v1793)))),
            [3, 4, 5, 6, 7],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1796*v4760))/v4770))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((self.scalar_static_f64[0]+(self.scalar_static_f64[795]*(if common.v779{(common.v780*self.scalar_static_f64[860])}else{(if (common.v776!=0.0){(v777*self.scalar_static_f64[860])}else{common.v3})})))/v1793))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1793*(self.scalar_static_f64[321]+(self.scalar_static_f64[795]*(if common.v779{(common.v780*self.scalar_static_f64[861])}else{(if (common.v776!=0.0){(v777*self.scalar_static_f64[861])}else{common.v3})}))))-(v1796*v4761))/v4770))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1796*v4762))/v4770))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((-(v1796*v4763))/v4770)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-v2066)))),
            [3, 5, 6, 7],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2065{v5533}else{(if v2059{(((v2061*((v2056*v5471)+(v2033*v5533)))-(v2060*(v5471+v5527)))/v5566)}else{(if v2034{v5533}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2065{v5536}else{(if v2059{(((v2061*((v2056*v5472)+(v2033*v5536)))-(v2060*(v5472+v5528)))/v5566)}else{(if v2034{v5536}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2065{v5539}else{(if v2059{(((v2061*((v2056*v5473)+(v2033*v5539)))-(v2060*(v5473+v5529)))/v5566)}else{(if v2034{v5539}else{common.v3})})})))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(-(if v2065{v5542}else{(if v2059{(((v2061*((v2056*v5474)+(v2033*v5542)))-(v2060*(v5474+v5530)))/v5566)}else{(if v2034{v5542}else{common.v3})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*((self.scalar_static_f64[0]*(self.scalar_static_f64[0]*(common.v713-common.v703)))/self.scalar_static_f64[546]))),
            2,
            multiplicity * (self.scalar_static_f64[925]),
            3,
            multiplicity * (self.scalar_static_f64[926]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * ((self.scalar_static_f64[15]*((self.scalar_static_f64[0]*common.v718)/self.scalar_static_f64[554]))),
            1,
            multiplicity * (self.scalar_static_f64[929]),
            4,
            multiplicity * (self.scalar_static_f64[930]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*v2426)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[15]*(common.v6597*v6603)), (self.scalar_static_f64[15]*(common.v6598*v6603)), (self.scalar_static_f64[15]*(common.v6599*v6603)), (self.scalar_static_f64[15]*(common.v6600*v6603)), (self.scalar_static_f64[15]*(common.v6601*v6603)), (self.scalar_static_f64[15]*(common.v6602*v6603))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * ((self.scalar_static_f64[15]*v2429)),
            3,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*common.v6616))),
            4,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*common.v6617))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * ((self.scalar_static_f64[15]*v2432)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[15]*(v6603*common.v6622)), (self.scalar_static_f64[15]*(v6603*common.v6623)), (self.scalar_static_f64[15]*(v6603*common.v6624)), (self.scalar_static_f64[15]*(v6603*common.v6625)), (self.scalar_static_f64[15]*(v6603*common.v6626)), (self.scalar_static_f64[15]*(v6603*common.v6627))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * ((self.scalar_static_f64[15]*v2435)),
            [3, 4, 5, 6, 7, 9],
            [(self.scalar_static_f64[15]*(v6603*common.v6640)), (self.scalar_static_f64[15]*(v6603*common.v6641)), (self.scalar_static_f64[15]*(v6603*common.v6642)), (self.scalar_static_f64[15]*(v6603*common.v6643)), (self.scalar_static_f64[15]*(v6603*common.v6644)), (self.scalar_static_f64[15]*(v6603*common.v6645))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[15]*v2439)),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[372]))),
            2,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[373]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[15]*v2443)),
            0,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[374]))),
            1,
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[375]))),
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v1717*v1771)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v4659+(v1717*v4599)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){((common.v1715*common.v4268)+(common.v1676*common.v4416))}else{common.v3}))+(v1717*v4600)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){(common.v1676*common.v4417)}else{common.v3}))+(v1717*v4601)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(v4659+(v1717*v4602)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){(v4423+(common.v1676*common.v4418))}else{common.v3}))+(v1717*v4603)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){(v4432+(common.v1676*common.v4419))}else{common.v3}))+(v1717*v4604)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){(v4432+(common.v1676*common.v4420))}else{common.v3}))+(v1717*v4605)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){((common.v1715*common.v4270)+(common.v1676*common.v4421))}else{common.v3}))+(v1717*v4606)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1771*(if (self.scalar_static_f64[242]!=0.0){(v4432+(common.v1676*common.v4422))}else{common.v3}))+(v1717*v4607))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[784]*(self.scalar_static_f64[0]*common.v736)))),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [self.scalar_static_f64[935], self.scalar_static_f64[936], self.scalar_static_f64[936], self.scalar_static_f64[936], self.scalar_static_f64[937], self.scalar_static_f64[937], self.scalar_static_f64[938], self.scalar_static_f64[937]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * ((self.scalar_static_f64[15]*v2451)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [v6714, (self.scalar_static_f64[15]*(v6603*common.v6699)), (self.scalar_static_f64[15]*(v6603*common.v6700)), v6714, (self.scalar_static_f64[15]*(v6603*common.v6701)), (self.scalar_static_f64[15]*(v6603*common.v6702)), (self.scalar_static_f64[15]*(v6603*common.v6703)), (self.scalar_static_f64[15]*(v6603*common.v6704)), (self.scalar_static_f64[15]*(v6603*common.v6705))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1666*v1771)+((v1437*v1771)+(common.v3*common.v732)))))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [(self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1666*v4599)+(v1437*v4599)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1666*v4600)+(v1437*v4600)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1666*v4601)+((v1771*(self.scalar_static_f64[653]*common.v3784))+(v1437*v4601))))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1771*(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[7]*v4216)}else{v4216}))+(v1666*v4602))+(((v1771*(self.scalar_static_f64[653]*common.v3785))+(v1437*v4602))+self.scalar_static_f64[367])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1771*(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[7]*v4220)}else{v4220}))+(v1666*v4603))+(((v1771*(self.scalar_static_f64[653]*common.v3786))+(v1437*v4603))+self.scalar_static_f64[368])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v4630+(v1666*v4604))+((v4650+(v1437*v4604))+self.scalar_static_f64[369])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v4630+(v1666*v4605))+((v4650+(v1437*v4605))+self.scalar_static_f64[369])))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*((v1666*v4606)+(v1437*v4606)))), (self.scalar_static_f64[15]*(self.scalar_static_f64[0]*(((v1771*(if (self.scalar_static_f64[242]!=0.0){(self.scalar_static_f64[7]*v4228)}else{v4228}))+(v1666*v4607))+(((v1771*(self.scalar_static_f64[653]*common.v3788))+(v1437*v4607))+self.scalar_static_f64[366]))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * ((self.scalar_static_f64[15]*v2457)),
            [4, 5, 6, 7, 9],
            [(self.scalar_static_f64[15]*(v6603*common.v6745)), (self.scalar_static_f64[15]*(v6603*common.v6746)), v6755, v6755, (self.scalar_static_f64[15]*(v6603*common.v6748))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * ((if (self.scalar_static_f64[196]!=0.0){(self.scalar_static_f64[15]*(self.scalar_static_f64[789]*(self.scalar_static_f64[0]*common.v729)))}else{common.v3})),
            8,
            multiplicity * (self.scalar_static_f64[943]),
            9,
            multiplicity * (self.scalar_static_f64[944]),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            common.v3,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(6),
            multiplicity * ((if (self.scalar_static_f64[197]!=0.0){(self.scalar_static_f64[15]*(self.scalar_static_f64[794]*(self.scalar_static_f64[0]*common.v726)))}else{common.v3})),
            6,
            multiplicity * (self.scalar_static_f64[949]),
            9,
            multiplicity * (self.scalar_static_f64[950]),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            common.v3,
        );
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (common.v3),
        );
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (common.v2467),
            10,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * ((common.v2382*v2468)),
            [3, 4, 5, 6, 7, 9, 10],
            [(v2468*common.v6426), (v2468*common.v6427), (v2468*common.v6428), (v2468*common.v6429), (v2468*common.v6430), (v2468*common.v6431), (common.v2382*v6603)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * ((v2358*common.v2467)),
            10,
            multiplicity * (v2358),
        );
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (common.v2467),
            10,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
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
            Some(6),
            multiplicity * (common.v3),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
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
        let v2426=0.0;
        let v2429=0.0;
        let v2432=0.0;
        let v2435=0.0;
        let v2439=0.0;
        let v2443=0.0;
        let v2451=0.0;
        let v2457=0.0;
        let v2468=0.0;
        let v6603=1.0;
        let v6714=(self.scalar_static_f64[15]*(v6603*common.v6698));
        let v6755=(self.scalar_static_f64[15]*(v6603*common.v6747));

        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[15]*(common.v6597*v6603)), (self.scalar_static_f64[15]*(common.v6598*v6603)), (self.scalar_static_f64[15]*(common.v6599*v6603)), (self.scalar_static_f64[15]*(common.v6600*v6603)), (self.scalar_static_f64[15]*(common.v6601*v6603)), (self.scalar_static_f64[15]*(common.v6602*v6603))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*common.v6616))),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*common.v6617))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[15]*(v6603*common.v6622)), (self.scalar_static_f64[15]*(v6603*common.v6623)), (self.scalar_static_f64[15]*(v6603*common.v6624)), (self.scalar_static_f64[15]*(v6603*common.v6625)), (self.scalar_static_f64[15]*(v6603*common.v6626)), (self.scalar_static_f64[15]*(v6603*common.v6627))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[15]*(v6603*common.v6640)), (self.scalar_static_f64[15]*(v6603*common.v6641)), (self.scalar_static_f64[15]*(v6603*common.v6642)), (self.scalar_static_f64[15]*(v6603*common.v6643)), (self.scalar_static_f64[15]*(v6603*common.v6644)), (self.scalar_static_f64[15]*(v6603*common.v6645))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[372]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[373]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[374]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[15]*(v6603*self.scalar_static_f64[375]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9]],
            &[v6714, (self.scalar_static_f64[15]*(v6603*common.v6699)), (self.scalar_static_f64[15]*(v6603*common.v6700)), v6714, (self.scalar_static_f64[15]*(v6603*common.v6701)), (self.scalar_static_f64[15]*(v6603*common.v6702)), (self.scalar_static_f64[15]*(v6603*common.v6703)), (self.scalar_static_f64[15]*(v6603*common.v6704)), (self.scalar_static_f64[15]*(v6603*common.v6705))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]],
            &[(self.scalar_static_f64[15]*(v6603*common.v6745)), (self.scalar_static_f64[15]*(v6603*common.v6746)), v6755, v6755, (self.scalar_static_f64[15]*(v6603*common.v6748))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &[nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9], nodes[10]],
            &[(v2468*common.v6426), (v2468*common.v6427), (v2468*common.v6428), (v2468*common.v6429), (v2468*common.v6430), (v2468*common.v6431), (common.v2382*v6603)],
            &[],
            &[],
            multiplicity,
        );
    }
}
