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
    v31: f64,
    v32: f64,
    v47: f64,
    v155: f64,
    v421: f64,
    v424: f64,
    v436: f64,
    v462: f64,
    v730: f64,
    v734: f64,
    v736: f64,
    v741: f64,
    v744: f64,
    v747: f64,
    v752: f64,
    v760: f64,
    v763: f64,
    v766: f64,
    v770: f64,
    v785: f64,
    v806: f64,
    v807: f64,
    v808: bool,
    v811: bool,
    v812: f64,
    v827: f64,
    v828: bool,
    v831: bool,
    v832: f64,
    v847: f64,
    v848: bool,
    v851: bool,
    v852: f64,
    v920: f64,
    v1038: f64,
    v1095: f64,
    v1119: f64,
    v1122: f64,
    v1125: f64,
    v1151: f64,
    v1227: f64,
    v1262: f64,
    v1263: f64,
    v1268: f64,
    v1269: f64,
    v1287: f64,
    v1288: bool,
    v1291: bool,
    v1292: f64,
    v1301: f64,
    v1331: f64,
    v1333: f64,
    v1334: bool,
    v1339: bool,
    v1340: f64,
    v1347: f64,
    v1348: f64,
    v1349: bool,
    v1354: bool,
    v1356: f64,
    v1406: f64,
    v1408: f64,
    v1409: bool,
    v1414: bool,
    v1415: f64,
    v1441: f64,
    v1453: f64,
    v1465: f64,
    v1477: f64,
    v1483: bool,
    v1484: f64,
    v1487: f64,
    v1488: bool,
    v1493: bool,
    v1494: f64,
    v1500: f64,
    v1504: f64,
    v1507: f64,
    v1515: f64,
    v1516: f64,
    v1517: f64,
    v1519: f64,
    v1521: f64,
    v1525: f64,
    v1526: f64,
    v1528: f64,
    v1530: bool,
    v1531: bool,
    v1532: bool,
    v1537: bool,
    v1538: f64,
    v1575: bool,
    v1577: f64,
    v1579: f64,
    v1580: f64,
    v1583: f64,
    v1584: bool,
    v1589: bool,
    v1590: f64,
    v1595: f64,
    v1598: f64,
    v1600: f64,
    v1608: f64,
    v1609: f64,
    v1610: f64,
    v1612: f64,
    v1617: f64,
    v1618: f64,
    v1620: f64,
    v1621: bool,
    v1622: bool,
    v1623: bool,
    v1628: bool,
    v1629: f64,
    v1758: f64,
    v1782: f64,
    v1799: f64,
    v1821: f64,
    v1891: f64,
    v1901: bool,
    v1911: bool,
    v1912: bool,
    v1913: f64,
    v1916: bool,
    v1917: f64,
    v1921: f64,
    v1922: f64,
    v1924: f64,
    v1928: f64,
    v1929: bool,
    v1934: bool,
    v1935: f64,
    v1948: bool,
    v2052: bool,
    v2053: f64,
    v2055: f64,
    v2057: f64,
    v2059: f64,
    v2061: f64,
    v2062: bool,
    v2064: bool,
    v2072: f64,
    v2074: bool,
    v2075: f64,
    v2076: f64,
    v2082: bool,
    v2084: f64,
    v2085: f64,
    v2089: f64,
    v2091: f64,
    v2094: f64,
    v2095: bool,
    v2100: bool,
    v2101: f64,
    v2461: f64,
    v2489: f64,
    v2540: f64,
    v2543: f64,
    v2546: f64,
    v2549: f64,
    v2552: f64,
    v2556: f64,
    v2560: f64,
    v2568: f64,
    v2574: f64,
    v2585: f64,
    v2601: f64,
    v2602: f64,
    v2627: f64,
    v2628: f64,
    v2629: f64,
    v2630: f64,
    v2780: f64,
    v2781: f64,
    v2782: f64,
    v3069: f64,
    v3070: f64,
    v3071: f64,
    v3217: f64,
    v3218: f64,
    v3219: f64,
    v3260: f64,
    v3261: f64,
    v3262: f64,
    v3269: f64,
    v3270: f64,
    v3271: f64,
    v3278: f64,
    v3279: f64,
    v3280: f64,
    v3312: f64,
    v3313: f64,
    v3492: f64,
    v3493: f64,
    v3494: f64,
    v3584: f64,
    v3585: f64,
    v3586: f64,
    v3587: f64,
    v3590: f64,
    v3593: f64,
    v3596: f64,
    v3599: f64,
    v3600: f64,
    v3601: f64,
    v3602: f64,
    v3604: f64,
    v3608: f64,
    v3611: f64,
    v3645: f64,
    v3646: f64,
    v3705: f64,
    v3706: f64,
    v3839: f64,
    v3840: f64,
    v3841: f64,
    v3896: f64,
    v3897: f64,
    v3898: f64,
    v3911: f64,
    v3912: f64,
    v3913: f64,
    v3934: f64,
    v3935: f64,
    v3936: f64,
    v3937: f64,
    v3938: f64,
    v3955: f64,
    v3956: f64,
    v3957: f64,
    v3958: f64,
    v3959: f64,
    v4591: f64,
    v4592: f64,
    v4593: f64,
    v4594: f64,
    v4677: f64,
    v4678: f64,
    v4679: f64,
    v4680: f64,
    v4681: f64,
    v4682: f64,
    v4695: f64,
    v4696: f64,
    v4697: f64,
    v4698: f64,
    v4699: f64,
    v4700: f64,
    v4701: f64,
    v4702: f64,
    v4838: f64,
    v4839: f64,
    v4840: f64,
    v4841: f64,
    v4842: f64,
    v4843: f64,
    v4844: f64,
    v4845: f64,
    v4846: f64,
    v5186: f64,
    v5187: f64,
    v5188: f64,
    v5189: f64,
    v6932: f64,
    v6933: f64,
    v6934: f64,
    v6935: f64,
    v6936: f64,
    v6937: f64,
    v7143: f64,
    v7144: f64,
    v7145: f64,
    v7146: f64,
    v7147: f64,
    v7148: f64,
    v7162: f64,
    v7163: f64,
    v7168: f64,
    v7169: f64,
    v7170: f64,
    v7171: f64,
    v7172: f64,
    v7173: f64,
    v7186: f64,
    v7187: f64,
    v7192: f64,
    v7193: f64,
    v7194: f64,
    v7195: f64,
    v7196: f64,
    v7197: f64,
    v7252: f64,
    v7253: f64,
    v7254: f64,
    v7255: f64,
    v7256: f64,
    v7257: f64,
    v7258: f64,
    v7259: f64,
    v7260: f64,
    v7302: f64,
    v7303: f64,
    v7304: f64,
    v7305: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v1=1.0;
        let v4=0.0;
        let v31=0.001;
        let v32=2.0;
        let v47=0.1;
        let v155=3.0;
        let v421=1e-6;
        let v424=0.5;
        let v436=4.0;
        let v462=6.0;
        let v727=ctx.node_voltage(nodes[6]);
        let v728=ctx.node_voltage(nodes[7]);
        let v730=(self.scalar_static_f64[0]*(v727-v728));
        let v731=ctx.node_voltage(nodes[8]);
        let v733=(self.scalar_static_f64[0]*(v727-v731));
        let v734=ctx.node_voltage(nodes[4]);
        let v736=(self.scalar_static_f64[0]*(v727-v734));
        let v737=ctx.node_voltage(nodes[5]);
        let v739=(self.scalar_static_f64[0]*(v737-v734));
        let v741=(self.scalar_static_f64[0]*(v737-v727));
        let v744=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[3])-v728));
        let v746=(self.scalar_static_f64[0]*(v728-v731));
        let v747=ctx.node_voltage(nodes[2]);
        let v750=ctx.node_voltage(nodes[1]);
        let v752=(self.scalar_static_f64[0]*(v750-v737));
        let v757=(self.scalar_static_f64[0]*(v750-ctx.node_voltage(nodes[0])));
        let v758=ctx.node_voltage(nodes[10]);
        let v760=(self.scalar_static_f64[0]*(v758-v728));
        let v763=(self.scalar_static_f64[0]*(ctx.node_voltage(nodes[9])-v758));
        let v766=(((v733+v741)-v746)-v760);
        let v770=((v766+(v752+(-v757)))-v763);
        let v771=(v757+v770);
        let v772=(v744-v760);
        let v774=(self.scalar_static_f64[382]*v733);
        let v776=(v774<self.scalar_static_f64[204]);
        let v777=(v774).exp();
        let v779=(!v776);
        let v781=(if v779{self.scalar_static_f64[205]}else{v4});
        let v785=(if v779{(v781*(v1+(v774-self.scalar_static_f64[204])))}else{(if v776{v777}else{v4})});
        let v786=(self.scalar_static_f64[382]*v736);
        let v787=(v786/self.scalar_static_f64[601]);
        let v788=(v787<self.scalar_static_f64[204]);
        let v789=(v787).exp();
        let v791=(!v788);
        let v792=(if v791{self.scalar_static_f64[205]}else{v781});
        let v796=(if v791{(v792*(v1+(v787-self.scalar_static_f64[204])))}else{(if v788{v789}else{v4})});
        let v797=(self.scalar_static_f64[382]*v766);
        let v798=(v797<self.scalar_static_f64[204]);
        let v799=(v797).exp();
        let v801=(!v798);
        let v802=(if v801{self.scalar_static_f64[205]}else{v792});
        let v806=(if v801{(v802*(v1+(v797-self.scalar_static_f64[204])))}else{(if v798{v799}else{v4})});
        let v807=(self.scalar_static_f64[382]*v741);
        let v808=(v807<self.scalar_static_f64[204]);
        let v811=(!v808);
        let v812=(if v811{self.scalar_static_f64[205]}else{v802});
        let v817=(self.scalar_static_f64[382]*v771);
        let v818=(v817<self.scalar_static_f64[204]);
        let v819=(v817).exp();
        let v821=(!v818);
        let v822=(if v821{self.scalar_static_f64[205]}else{v812});
        let v826=(if v821{(v822*(v1+(v817-self.scalar_static_f64[204])))}else{(if v818{v819}else{v4})});
        let v827=(self.scalar_static_f64[382]*v744);
        let v828=(v827<self.scalar_static_f64[204]);
        let v831=(!v828);
        let v832=(if v831{self.scalar_static_f64[205]}else{v822});
        let v837=(self.scalar_static_f64[382]*(v772-v763));
        let v838=(v837<self.scalar_static_f64[204]);
        let v839=(v837).exp();
        let v841=(!v838);
        let v842=(if v841{self.scalar_static_f64[205]}else{v832});
        let v846=(if v841{(v842*(v1+(v837-self.scalar_static_f64[204])))}else{(if v838{v839}else{v4})});
        let v847=(self.scalar_static_f64[382]*v772);
        let v848=(v847<self.scalar_static_f64[204]);
        let v851=(!v848);
        let v852=(if v851{self.scalar_static_f64[205]}else{v842});
        let v858=(self.scalar_static_f64[382]*(v771-self.scalar_static_f64[466]));
        let v859=(v858<self.scalar_static_f64[204]);
        let v860=(v858).exp();
        let v862=(!v859);
        let v863=(if v862{self.scalar_static_f64[205]}else{v852});
        let v869=(self.scalar_static_f64[382]*(v766-self.scalar_static_f64[466]));
        let v870=(v869<self.scalar_static_f64[204]);
        let v871=(v869).exp();
        let v873=(!v870);
        let v874=(if v873{self.scalar_static_f64[205]}else{v863});
        let v880=(self.scalar_static_f64[382]*(v733-self.scalar_static_f64[466]));
        let v881=(v880<self.scalar_static_f64[204]);
        let v882=(v880).exp();
        let v884=(!v881);
        let v885=(if v884{self.scalar_static_f64[205]}else{v874});
        let v889=(if v884{(v885*(v1+(v880-self.scalar_static_f64[204])))}else{(if v881{v882}else{v4})});
        let v891=(self.scalar_static_f64[382]*(v730-self.scalar_static_f64[466]));
        let v892=(v891<self.scalar_static_f64[204]);
        let v893=(v891).exp();
        let v895=(!v892);
        let v896=(if v895{self.scalar_static_f64[205]}else{v885});
        let v900=(if v895{(v896*(v1+(v891-self.scalar_static_f64[204])))}else{(if v892{v893}else{v4})});
        let v903=((v1+(v436*v889))).sqrt();
        let v906=((v1+(v436*v900))).sqrt();
        let v907=(v32*v900);
        let v908=(v1+v906);
        let v909=(v907/v908);
        let v911=(v909<self.scalar_static_f64[206]);
        let v912=(if v911{self.scalar_static_f64[206]}else{v909});
        let v914=(v1+v903);
        let v915=(v914/v908);
        let v918=(self.scalar_static_f64[381]*((v903-v906)-(v915).ln()));
        let v920=((v746+v918)/self.scalar_static_f64[578]);
        let v921=(v920>v4);
        let v922=100.0;
        let v923=(v730<v922);
        let v924=(v921&&v923);
        let v927=(v921&&(!v923));
        let v929=(v1+(v730-v922));
        let v935=(self.scalar_static_f64[578]*(v424*v920));
        let v937=(v1+(self.scalar_static_f64[382]*v935));
        let v942=(if v921{((self.scalar_static_f64[466]+(self.scalar_static_f64[818]*(v937).ln()))-(if v927{(v922+(v929).ln())}else{(if v924{v730}else{v4})}))}else{v4});
        let v945=(if v921{self.scalar_static_f64[819]}else{v4});
        let v947=(if v921{(v945*v945)}else{v421});
        let v950=(v942<v4);
        let v951=(v921&&v950);
        let v952=(v424*v947);
        let v954=((v947+(if v921{(v942*v942)}else{self.scalar_static_f64[628]}))).sqrt();
        let v955=(v954-v942);
        let v959=(v921&&(!v950));
        let v962=(if v959{(v424*(v942+v954))}else{(if v951{(v952/v955)}else{v4})});
        let v966=(v962+self.scalar_static_f64[209]);
        let v967=(v962*v966);
        let v970=(self.scalar_static_f64[208]*(v962+self.scalar_static_f64[820]));
        let v972=(if v921{(v967/v970)}else{v4});
        let v974=(if v921{(v920/v972)}else{v4});
        let v978=(if v921{((v974-v1)/self.scalar_static_f64[210])}else{self.scalar_static_f64[608]});
        let v979=(v974<v1);
        let v980=(v921&&v979);
        let v981=(v978).exp();
        let v982=(v1+v981);
        let v988=(v921&&(!v979));
        let v990=((-v978)).exp();
        let v991=(v1+v990);
        let v1004=(if v921{((if v988{(v974+(self.scalar_static_f64[210]*(v991).ln()))}else{(if v980{(v1+(self.scalar_static_f64[210]*(v982).ln()))}else{v4})})/self.scalar_static_f64[216])}else{v4});
        let v1006=(if v921{(v962/self.scalar_static_f64[209])}else{v4});
        let v1007=(v436*v1004);
        let v1008=(v1006*v1007);
        let v1009=(v1+v1006);
        let v1012=((v1+(v1008*v1009))).sqrt();
        let v1013=(v1+v1012);
        let v1014=(v32*v1004);
        let v1015=(v1009*v1014);
        let v1017=(if v921{(v1013/v1015)}else{v4});
        let v1019=(v912*v1017);
        let v1020=((v1-v1017)+v1019);
        let v1021=(v1+v1019);
        let v1023=(if v921{(v1020/v1021)}else{v4});
        let v1026=(if v921{(self.scalar_static_f64[382]*(v935*v1023))}else{v4});
        let v1029=(v1+(v912+v1026));
        let v1032=(if v921{((v32*v1026)+(v912*v1029))}else{v4});
        let v1035=(if v921{(v424*(v1026-v1))}else{v4});
        let v1038=(if v921{(v1032+(v1035*v1035))}else{v4});
        let v1039=(v1026>=v1);
        let v1040=(v921&&v1039);
        let v1041=(v1038).sqrt();
        let v1045=(v921&&(!v1039));
        let v1046=(v1041-v1035);
        let v1048=(if v1045{(v1032/v1046)}else{(if v1040{(v1035+v1041)}else{v4})});
        let v1051=(v921&&(v1048<self.scalar_static_f64[217]));
        let v1052=(if v1051{self.scalar_static_f64[217]}else{v1048});
        let v1053=(v1+v1052);
        let v1062=(if v921{(self.scalar_static_f64[218]*(v920-self.scalar_static_f64[207]))}else{v4});
        let v1069=(((if v921{(v920*self.scalar_static_f64[824])}else{v4})+(v1062*v1062))).sqrt();
        let v1078=(v921&&self.scalar_static_bool[20]);
        let v1079=(v32*v920);
        let v1080=(v920+v972);
        let v1085=(v920*self.scalar_static_f64[207]);
        let v1086=(v920+self.scalar_static_f64[207]);
        let v1091=(!v921);
        let v1092=(v32*v889);
        let v1095=(if v1091{v785}else{(if v921{((v1052*v1053)*self.scalar_static_f64[822])}else{v4})});
        let v1106=(((v746).abs()<self.scalar_static_f64[826])||((v918).abs()<(self.scalar_static_f64[827]*(v903+v906))));
        let v1107=(v1091&&v1106);
        let v1108=(v912+(if v1091{(v1092/v914)}else{v1052}));
        let v1110=(if v1107{(v424*v1108)}else{v4});
        let v1111=(v1+v1110);
        let v1115=(v1091&&(!v1106));
        let v1117=((v733+v918)-v730);
        let v1119=(if v1115{(v918/v1117)}else{(if v1107{(v1110/v1111)}else{v1023})});
        let v1121=(if v1091{self.scalar_static_f64[825]}else{(if v1078{(self.scalar_static_f64[502]*(v47+(v1079/v1080)))}else{(if (v921&&self.scalar_static_bool[19]){self.scalar_static_f64[825]}else{v4})})});
        let v1122=(if v1091{v920}else{(if v921{(v1085/v1086)}else{v4})});
        let v1125=(if v1091{(v1-(v1122/self.scalar_static_f64[207]))}else{(if v921{(self.scalar_static_f64[207]/v1086)}else{v4})});
        let v1132=((v736-self.scalar_static_f64[828])/self.scalar_static_f64[829]);
        let v1133=(v736<self.scalar_static_f64[828]);
        let v1134=(v1132).exp();
        let v1135=(v1+v1134);
        let v1140=(!v1133);
        let v1142=((-v1132)).exp();
        let v1143=(v1+v1142);
        let v1147=(if v1140{(self.scalar_static_f64[828]-(self.scalar_static_f64[829]*(v1143).ln()))}else{(if v1133{(v736-(self.scalar_static_f64[829]*(v1135).ln()))}else{v4})});
        let v1149=(v1-(self.scalar_static_f64[541]*v1147));
        let v1151=f64::powf(v1149,self.scalar_static_f64[223]);
        let v1157=((self.scalar_static_f64[830]*(v1-v1151))+(v155*(v736-v1147)));
        let v1168=(if self.scalar_static_bool[26]{v733}else{(if self.scalar_static_bool[24]{(v730+(if v1091{v746}else{(if v921{(v1062+v1069)}else{v4})}))}else{(if self.scalar_static_bool[21]{v730}else{v4})})});
        let v1176=(v1168-self.scalar_static_f64[836]);
        let v1177=(v1176/v1121);
        let v1178=(v1168<self.scalar_static_f64[836]);
        let v1179=(v1177).exp();
        let v1180=(v1+v1179);
        let v1181=(v1180).ln();
        let v1185=(!v1178);
        let v1187=((-v1177)).exp();
        let v1188=(v1+v1187);
        let v1189=(v1188).ln();
        let v1192=(if v1185{(self.scalar_static_f64[836]-(v1121*v1189))}else{(if v1178{(v1168-(v1121*v1181))}else{v4})});
        let v1194=f64::powf(v1125,self.scalar_static_f64[226]);
        let v1198=(v1-(v1192/self.scalar_static_f64[502]));
        let v1199=f64::powf(v1198,self.scalar_static_f64[227]);
        let v1203=(self.scalar_static_f64[833]*v1194);
        let v1204=(v1168-v1192);
        let v1209=((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(v1-(v1194*v1199)))+(v1203*v1204)))+(self.scalar_static_f64[557]*v730));
        let v1212=(v796*self.scalar_static_f64[839]);
        let v1214=((v1+v1212)).sqrt();
        let v1215=(v1+v1214);
        let v1216=(v1212/v1215);
        let v1218=f64::powf(v1095,self.scalar_static_f64[840]);
        let v1219=(self.scalar_static_f64[839]*v1218);
        let v1221=((v1+v1219)).sqrt();
        let v1222=(v1+v1221);
        let v1223=(v1219/v1222);
        let v1226=(v1+(v1157/self.scalar_static_f64[761]));
        let v1227=(v1209/self.scalar_static_f64[759]);
        let v1228=(v1226+v1227);
        let v1239=((if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*v1226))}else{v4})).exp();
        let v1240=((if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*((-v1209)/self.scalar_static_f64[759])))}else{v4})).exp();
        let v1246=(if self.scalar_static_bool[28]{((v1239-v1240)/self.scalar_static_f64[843])}else{(if self.scalar_static_bool[27]{v1228}else{v4})});
        let v1247=0.010000000000000002;
        let v1248=(v1246*v1246);
        let v1249=(v1246<v4);
        let v1250=0.005000000000000001;
        let v1252=((v1247+v1248)).sqrt();
        let v1253=(v1252-v1246);
        let v1256=(!v1249);
        let v1259=(if v1256{(v424*(v1246+v1252))}else{(if v1249{(v1250/v1253)}else{v4})});
        let v1262=(v1+(v424*(v1216+v1223)));
        let v1263=(v1259*v1262);
        let v1266=(v1218*self.scalar_static_f64[844]);
        let v1267=(self.scalar_static_f64[644]*v796);
        let v1268=(v1267-v1266);
        let v1269=(v1268/v1263);
        let v1270=0.0001;
        let v1271=(v736/v1270);
        let v1272=(v736<v4);
        let v1273=(v1271).exp();
        let v1274=(v1+v1273);
        let v1278=(!v1272);
        let v1280=((-v1271)).exp();
        let v1281=(v1+v1280);
        let v1285=(if v1278{(v736+(v1270*(v1281).ln()))}else{(if v1272{(v1270*(v1274).ln())}else{v4})});
        let v1287=(v1285/self.scalar_static_f64[229]);
        let v1288=(v1287<self.scalar_static_f64[204]);
        let v1291=(!v1288);
        let v1292=(if v1291{self.scalar_static_f64[205]}else{v896});
        let v1301=((v736-self.scalar_static_f64[230])/v31);
        let v1322=(v786/self.scalar_static_f64[142]);
        let v1323=(v1322<self.scalar_static_f64[204]);
        let v1324=(v1322).exp();
        let v1326=(!v1323);
        let v1327=(if v1326{self.scalar_static_f64[205]}else{v1292});
        let v1331=(if v1326{(v1327*(v1+(v1322-self.scalar_static_f64[204])))}else{(if v1323{v1324}else{v1285})});
        let v1333=(self.scalar_static_f64[382]*(v736-self.scalar_static_f64[521]));
        let v1334=(v1333<self.scalar_static_f64[204]);
        let v1339=(self.scalar_static_bool[12]&&(!v1334));
        let v1340=(if v1339{self.scalar_static_f64[205]}else{v1327});
        let v1347=((v1269/self.scalar_static_f64[644])-1000.0);
        let v1348=40.0;
        let v1349=(v1347<v1348);
        let v1354=(self.scalar_static_bool[12]&&(!v1349));
        let v1356=(if v1354{2.3538526683702e17}else{v1340});
        let v1396=(self.scalar_static_f64[382]*v739);
        let v1397=(v1396/self.scalar_static_f64[146]);
        let v1398=(v1397<self.scalar_static_f64[204]);
        let v1399=(v1397).exp();
        let v1401=(!v1398);
        let v1402=(if v1401{self.scalar_static_f64[205]}else{v1356});
        let v1406=(if v1401{(v1402*(v1+(v1397-self.scalar_static_f64[204])))}else{(if v1398{v1399}else{v1331})});
        let v1408=(self.scalar_static_f64[382]*(v739-self.scalar_static_f64[521]));
        let v1409=(v1408<self.scalar_static_f64[204]);
        let v1414=(self.scalar_static_bool[12]&&(!v1409));
        let v1415=(if v1414{self.scalar_static_f64[205]}else{v1402});
        let v1432=(v786/self.scalar_static_f64[129]);
        let v1433=(v1432<self.scalar_static_f64[204]);
        let v1434=(v1432).exp();
        let v1436=(!v1433);
        let v1437=(if v1436{self.scalar_static_f64[205]}else{v1415});
        let v1441=(if v1436{(v1437*(v1+(v1432-self.scalar_static_f64[204])))}else{(if v1433{v1434}else{v1406})});
        let v1444=(v1396/self.scalar_static_f64[163]);
        let v1445=(v1444<self.scalar_static_f64[204]);
        let v1446=(v1444).exp();
        let v1448=(!v1445);
        let v1449=(if v1448{self.scalar_static_f64[205]}else{v1437});
        let v1453=(if v1448{(v1449*(v1+(v1444-self.scalar_static_f64[204])))}else{(if v1445{v1446}else{v1441})});
        let v1456=(v797/self.scalar_static_f64[135]);
        let v1457=(v1456<self.scalar_static_f64[204]);
        let v1458=(v1456).exp();
        let v1460=(!v1457);
        let v1461=(if v1460{self.scalar_static_f64[205]}else{v1449});
        let v1465=(if v1460{(v1461*(v1+(v1456-self.scalar_static_f64[204])))}else{(if v1457{v1458}else{v1453})});
        let v1468=(v1396/self.scalar_static_f64[167]);
        let v1469=(v1468<self.scalar_static_f64[204]);
        let v1470=(v1468).exp();
        let v1472=(!v1469);
        let v1473=(if v1472{self.scalar_static_f64[205]}else{v1461});
        let v1477=(if v1472{(v1473*(v1+(v1468-self.scalar_static_f64[204])))}else{(if v1469{v1470}else{v1465})});
        let v1483=(v1272&&self.scalar_static_bool[36]);
        let v1484=(v32*v1151);
        let v1487=(self.scalar_static_f64[726]*(v1-(self.scalar_static_f64[19]/v1484)));
        let v1488=(v1487<self.scalar_static_f64[204]);
        let v1493=(v1483&&(!v1488));
        let v1494=(if v1493{self.scalar_static_f64[205]}else{v1473});
        let v1500=(if v1483{(self.scalar_static_f64[541]*v736)}else{self.scalar_static_f64[757]});
        let v1502=1e-30;
        let v1504=(((v1500*v1500)+v1502)).sqrt();
        let v1507=f64::powf(v1504,self.scalar_static_f64[234]);
        let v1515=(v462*v1500);
        let v1516=(v1500*v1515);
        let v1517=(v1500+self.scalar_static_f64[237]);
        let v1519=((self.scalar_static_f64[17]*(self.scalar_static_f64[236]-((v155*v1500)*self.scalar_static_f64[237])))-(v1516*v1517));
        let v1521=0.16666666666666666;
        let v1525=(self.scalar_static_f64[726]*(self.scalar_static_f64[19]*v736));
        let v1526=(self.scalar_static_f64[405]*(if v1483{((v1507*v1519)*v1521)}else{v4}));
        let v1528=(if v1483{(v1525/v1526)}else{v1500});
        let v1529=-0.001;
        let v1530=(v1528<v1529);
        let v1531=(v1528<self.scalar_static_f64[204]);
        let v1532=(v1483&&v1530);
        let v1537=(v1532&&(!v1531));
        let v1538=(if v1537{self.scalar_static_f64[205]}else{v1494});
        let v1575=(self.scalar_static_bool[39]&&(v730<v4));
        let v1576=(self.scalar_static_f64[542]*v730);
        let v1577=(v1-v1576);
        let v1579=(if v1575{f64::powf(v1577,self.scalar_static_f64[227])}else{v4});
        let v1580=(v32*v1579);
        let v1583=(self.scalar_static_f64[746]*(v1-(self.scalar_static_f64[50]/v1580)));
        let v1584=(v1583<self.scalar_static_f64[204]);
        let v1589=(v1575&&(!v1584));
        let v1590=(if v1589{self.scalar_static_f64[205]}else{v1538});
        let v1595=(if v1575{v1576}else{self.scalar_static_f64[737]});
        let v1598=((v1502+(v1595*v1595))).sqrt();
        let v1600=f64::powf(v1598,self.scalar_static_f64[238]);
        let v1608=(v462*v1595);
        let v1609=(v1595*v1608);
        let v1610=(v1595+self.scalar_static_f64[241]);
        let v1612=((self.scalar_static_f64[48]*(self.scalar_static_f64[240]-((v155*v1595)*self.scalar_static_f64[241])))-(v1609*v1610));
        let v1617=(self.scalar_static_f64[746]*(self.scalar_static_f64[50]*v730));
        let v1618=(self.scalar_static_f64[425]*(if v1575{(v1521*(v1600*v1612))}else{v4}));
        let v1620=(if v1575{(v1617/v1618)}else{v1595});
        let v1621=(v1620<v1529);
        let v1622=(v1620<self.scalar_static_f64[204]);
        let v1623=(v1575&&v1621);
        let v1628=(v1623&&(!v1622));
        let v1629=(if v1628{self.scalar_static_f64[205]}else{v1590});
        let v1660=(v806*self.scalar_static_f64[839]);
        let v1661=(v436*(if v873{(v874*(v1+(v869-self.scalar_static_f64[204])))}else{(if v870{v871}else{v4})}));
        let v1662=(v1660-self.scalar_static_f64[839]);
        let v1664=((v1+v1660)).sqrt();
        let v1665=(v1+v1664);
        let v1668=((v1+v1661)).sqrt();
        let v1669=(v1+v1668);
        let v1751=(v826-v1);
        let v1752=(self.scalar_static_f64[859]*v1751);
        let v1755=((v1+(v826*self.scalar_static_f64[851]))).sqrt();
        let v1756=(v1+v1755);
        let v1758=(if self.scalar_static_bool[44]{(v1752/v1756)}else{v4});
        let v1764=(self.scalar_static_f64[860]*(v826-v846));
        let v1771=((v1+(self.scalar_static_f64[862]*(v826+(v846*self.scalar_static_f64[245]))))).sqrt();
        let v1772=(v1+v1771);
        let v1776=(v1751*self.scalar_static_f64[860]);
        let v1779=((v1+(v826*self.scalar_static_f64[862]))).sqrt();
        let v1780=(v1+v1779);
        let v1782=(if self.scalar_static_bool[46]{(v1776/v1780)}else{(if self.scalar_static_bool[45]{(v1764/v1772)}else{v4})});
        let v1795=(if self.scalar_static_bool[48]{(v771-self.scalar_static_f64[871])}else{v4});
        let v1799=(if self.scalar_static_bool[48]{(v1795*v1795)}else{v1248});
        let v1800=(v1795<v4);
        let v1801=(self.scalar_static_bool[48]&&v1800);
        let v1804=((self.scalar_static_f64[253]+v1799)).sqrt();
        let v1805=(v1804-v1795);
        let v1809=(self.scalar_static_bool[48]&&(!v1800));
        let v1812=(if v1809{(v424*(v1795+v1804))}else{(if v1801{(self.scalar_static_f64[254]/v1805)}else{v4})});
        let v1816=(v1812+(self.scalar_static_f64[866]+(self.scalar_static_f64[571]*(v1758+v1782))));
        let v1821=(if self.scalar_static_bool[50]{v1}else{(if self.scalar_static_bool[48]{(v1812/v1816)}else{v1})});
        let v1882=(v1228<v4);
        let v1884=((v1247+(v1228*v1228))).sqrt();
        let v1885=(v1884-v1228);
        let v1888=(!v1882);
        let v1891=(if v1888{(v424*(v1228+v1884))}else{(if v1882{(v1250/v1885)}else{v4})});
        let v1901=(v1269>v4);
        let v1905=(v730<self.scalar_static_f64[274]);
        let v1908=((-v1269)/self.scalar_static_f64[275]);
        let v1909=(v1908<self.scalar_static_f64[204]);
        let v1911=(v1905&&(v1901&&self.scalar_static_bool[53]));
        let v1912=(v1909&&v1911);
        let v1913=(v1908).exp();
        let v1916=(v1911&&(!v1909));
        let v1917=(if v1916{self.scalar_static_f64[205]}else{v1629});
        let v1921=(if v1916{(v1917*(v1+(v1908-self.scalar_static_f64[204])))}else{(if v1912{v1913}else{v4})});
        let v1922=(self.scalar_static_f64[274]-v730);
        let v1924=(if v1911{(v1921*v1922)}else{v4});
        let v1928=(self.scalar_static_f64[872]*f64::powf(v1924,self.scalar_static_f64[276]));
        let v1929=(v1928<self.scalar_static_f64[204]);
        let v1934=(v1911&&(!v1929));
        let v1935=(if v1934{self.scalar_static_f64[205]}else{v1917});
        let v1948=(v1901&&self.scalar_static_bool[55]);
        let v2052=(v1905&&(self.scalar_static_bool[58]&&(v1948&&self.scalar_static_bool[59])));
        let v2053=f64::powf(v1922,self.scalar_static_f64[276]);
        let v2055=(v1269+self.scalar_static_f64[289]);
        let v2057=(v1-(v1269/v2055));
        let v2059=f64::powf(v2057,self.scalar_static_f64[290]);
        let v2061=(if v2052{(v2053*v2059)}else{v4});
        let v2062=(self.scalar_static_bool[56]&&v2052);
        let v2064=(self.scalar_static_bool[57]&&v2052);
        let v2068=(if v2064{((v1269-self.scalar_static_f64[291])/self.scalar_static_f64[289])}else{v4});
        let v2072=(if v2064{((v2068-v1)/self.scalar_static_f64[292])}else{v1301});
        let v2073=(v2068<v1);
        let v2074=(v2064&&v2073);
        let v2075=(v2072).exp();
        let v2076=(v1+v2075);
        let v2082=(v2064&&(!v2073));
        let v2084=((-v2072)).exp();
        let v2085=(v1+v2084);
        let v2089=(if v2082{(v2068+(self.scalar_static_f64[292]*(v2085).ln()))}else{(if v2074{(v1+(self.scalar_static_f64[292]*(v2076).ln()))}else{v4})});
        let v2091=f64::powf(v2089,self.scalar_static_f64[293]);
        let v2094=(self.scalar_static_f64[872]*(if v2064{(v2061*v2091)}else{(if v2062{v2061}else{v4})}));
        let v2095=(v2094<self.scalar_static_f64[204]);
        let v2100=(v2052&&(!v2095));
        let v2101=(if v2100{self.scalar_static_f64[205]}else{v1935});
        let v2160=((v739-self.scalar_static_f64[828])/self.scalar_static_f64[829]);
        let v2161=(v739<self.scalar_static_f64[828]);
        let v2162=(v2160).exp();
        let v2163=(v1+v2162);
        let v2168=(!v2161);
        let v2170=((-v2160)).exp();
        let v2171=(v1+v2170);
        let v2175=(if v2168{(self.scalar_static_f64[828]-(self.scalar_static_f64[829]*(v2171).ln()))}else{(if v2161{(v739-(self.scalar_static_f64[829]*(v2163).ln()))}else{v4})});
        let v2178=(v1-(self.scalar_static_f64[541]*v2175));
        let v2191=(v1216*self.scalar_static_f64[880]);
        let v2192=(v1891*v2191);
        let v2193=(v1223*self.scalar_static_f64[880]);
        let v2194=(v1891*v2193);
        let v2196=((v766-self.scalar_static_f64[836])/self.scalar_static_f64[825]);
        let v2197=(v766<self.scalar_static_f64[836]);
        let v2198=(v2196).exp();
        let v2199=(v1+v2198);
        let v2204=(!v2197);
        let v2206=((-v2196)).exp();
        let v2207=(v1+v2206);
        let v2211=(if v2204{(self.scalar_static_f64[836]-(self.scalar_static_f64[825]*(v2207).ln()))}else{(if v2197{(v766-(self.scalar_static_f64[825]*(v2199).ln()))}else{v4})});
        let v2213=(v1-(v2211/self.scalar_static_f64[502]));
        let v2228=((v771-self.scalar_static_f64[836])/self.scalar_static_f64[825]);
        let v2229=(v771<self.scalar_static_f64[836]);
        let v2230=(v2228).exp();
        let v2231=(v1+v2230);
        let v2236=(!v2229);
        let v2238=((-v2228)).exp();
        let v2239=(v1+v2238);
        let v2243=(if v2236{(self.scalar_static_f64[836]-(self.scalar_static_f64[825]*(v2239).ln()))}else{(if v2229{(v771-(self.scalar_static_f64[825]*(v2231).ln()))}else{v4})});
        let v2245=(v1-(v2243/self.scalar_static_f64[502]));
        let v2264=((v744-self.scalar_static_f64[882])/self.scalar_static_f64[881]);
        let v2265=(v744<self.scalar_static_f64[882]);
        let v2266=(v2264).exp();
        let v2267=(v1+v2266);
        let v2272=(!v2265);
        let v2274=((-v2264)).exp();
        let v2275=(v1+v2274);
        let v2279=(if v2272{(self.scalar_static_f64[882]-(self.scalar_static_f64[881]*(v2275).ln()))}else{(if v2265{(v744-(self.scalar_static_f64[881]*(v2267).ln()))}else{v4})});
        let v2283=(v1-(v2279/self.scalar_static_f64[540]));
        let v2298=(v736/self.scalar_static_f64[888]);
        let v2299=(v2298<self.scalar_static_f64[204]);
        let v2300=(v2298).exp();
        let v2302=(!v2299);
        let v2303=(if v2302{self.scalar_static_f64[205]}else{v2101});
        let v2308=(self.scalar_static_f64[887]*(if v2302{(v2303*(v1+(v2298-self.scalar_static_f64[204])))}else{(if v2299{v2300}else{v1477})}));
        let v2313=(v1119*self.scalar_static_f64[892]);
        let v2314=(v32+v1108);
        let v2328=(self.scalar_static_f64[382]*((v766-self.scalar_static_f64[484])/self.scalar_static_f64[306]));
        let v2329=(v2328<self.scalar_static_f64[204]);
        let v2331=(v2329&&self.scalar_static_bool[64]);
        let v2332=(v2328).exp();
        let v2335=(self.scalar_static_bool[64]&&(!v2329));
        let v2336=(if v2335{self.scalar_static_f64[205]}else{v2303});
        let v2342=(v806*self.scalar_static_f64[894]);
        let v2345=((v1+(v436*(if v2335{(v2336*(v1+(v2328-self.scalar_static_f64[204])))}else{(if v2331{v2332}else{v4})})))).sqrt();
        let v2346=(v1+v2345);
        let v2348=(if self.scalar_static_bool[64]{(v2342/v2346)}else{(if self.scalar_static_bool[63]{((self.scalar_static_f64[893]*(((v1662/v1665)*self.scalar_static_f64[879])+((v1661/v1669)*self.scalar_static_f64[891])))/self.scalar_static_f64[790])}else{v4})});
        let v2356=(if self.scalar_static_bool[68]{(v826*self.scalar_static_f64[839])}else{v4});
        let v2357=(v2356-self.scalar_static_f64[839]);
        let v2359=((v1+v2356)).sqrt();
        let v2360=(v1+v2359);
        let v2364=(if self.scalar_static_bool[68]{(v436*(if v862{(v863*(v1+(v858-self.scalar_static_f64[204])))}else{(if v859{v860}else{v4})}))}else{v4});
        let v2366=((v1+v2364)).sqrt();
        let v2367=(v1+v2366);
        let v2379=(self.scalar_static_f64[382]*(v771-self.scalar_static_f64[484]));
        let v2380=(v2379<self.scalar_static_f64[204]);
        let v2382=(v2380&&self.scalar_static_bool[69]);
        let v2383=(v2379).exp();
        let v2386=(self.scalar_static_bool[69]&&(!v2380));
        let v2387=(if v2386{self.scalar_static_f64[205]}else{v2336});
        let v2393=(v826*self.scalar_static_f64[896]);
        let v2396=((v1+(v436*(if v2386{(v2387*(v1+(v2379-self.scalar_static_f64[204])))}else{(if v2382{v2383}else{v4})})))).sqrt();
        let v2397=(v1+v2396);
        let v2399=(if self.scalar_static_bool[69]{(v2393/v2397)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[895]*((self.scalar_static_f64[879]*(if self.scalar_static_bool[68]{(v2357/v2360)}else{v4}))+(self.scalar_static_f64[891]*(if self.scalar_static_bool[68]{(v2364/v2367)}else{v4}))))/self.scalar_static_f64[790])}else{v4})});
        let v2407=(if self.scalar_static_bool[70]{(f64::powf(v1149,self.scalar_static_f64[309])-v155)}else{v4});
        let v2408=(if self.scalar_static_bool[70]{v1132}else{v4});
        let v2409=(v2408<v4);
        let v2410=(self.scalar_static_bool[70]&&v2409);
        let v2411=(v2408).exp();
        let v2412=(v1+v2411);
        let v2416=(self.scalar_static_bool[70]&&(!v2409));
        let v2418=((-v2408)).exp();
        let v2419=(v1+v2418);
        let v2421=(if v2416{(v2418/v2419)}else{(if v2410{(v1/v2412)}else{v4})});
        let v2428=((self.scalar_static_f64[382]*v1212)/self.scalar_static_f64[601]);
        let v2429=(v424/v1214);
        let v2431=(if self.scalar_static_bool[70]{(v2428*v2429)}else{v4});
        let v2432=(v1891*self.scalar_static_f64[880]);
        let v2437=(v741*0.2);
        let v2439=((if self.scalar_static_bool[70]{(v2308/self.scalar_static_f64[888])}else{v4})+((if self.scalar_static_bool[70]{(self.scalar_static_f64[876]*(if self.scalar_static_bool[70]{(v155+(v2407*v2421))}else{v4}))}else{v4})+(if self.scalar_static_bool[70]{(v2431*v2432)}else{v4})));
        let v2448=(if self.scalar_static_bool[70]{(v2192+(v2308*self.scalar_static_f64[310]))}else{v4});
        let v2457=(if self.scalar_static_bool[71]{v2192}else{(if self.scalar_static_bool[70]{(v2448*self.scalar_static_f64[313])}else{v4})});
        let v2458=(if self.scalar_static_bool[71]{v2194}else{(if self.scalar_static_bool[70]{(v2194+(v2448*self.scalar_static_f64[312]))}else{v4})});
        let v2460=(v1266+v1267);
        let v2461=(v2460/v1263);
        let v2469=(v2461>v4);
        let v2470=(v2457+v2458);
        let v2473=(!v2469);
        let v2474=(self.scalar_static_f64[786]*v1891);
        let v2476=(if v2473{(v1263*v2474)}else{(if v2469{(v2470/v2461)}else{v4})});
        let v2489=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(v2476*self.scalar_static_f64[316])}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v2476)}else{v4})})});
        let v2540=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v2308}else{(if self.scalar_static_bool[70]{(v2308*self.scalar_static_f64[311])}else{v4})})+((v1157*self.scalar_static_f64[876])+v2457)));
        let v2543=(self.scalar_static_f64[0]*(self.scalar_static_f64[877]*((self.scalar_static_f64[830]*(v1-f64::powf(v2178,self.scalar_static_f64[223])))+(v155*(v739-v2175)))));
        let v2546=(self.scalar_static_f64[0]*((v2313*v2314)+((v1209*self.scalar_static_f64[878])+v2458)));
        let v2549=(self.scalar_static_f64[0]*(self.scalar_static_f64[550]*((self.scalar_static_f64[883]*(v1-f64::powf(v2283,self.scalar_static_f64[302])))+(v32*(v744-v2279)))));
        let v2552=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{(v2437*v2439)}else{v4}));
        let v2556=((self.scalar_static_f64[0]*(v750-v747))*self.scalar_static_f64[319]);
        let v2560=(v757*self.scalar_static_f64[320]);
        let v2568=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(v1-f64::powf(v2245,self.scalar_static_f64[227])))+(self.scalar_static_f64[833]*(v771-v2243))))+(self.scalar_static_f64[557]*v771)))))+(if self.scalar_static_bool[67]{(v1821*v2399)}else{v4})));
        let v2574=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*((self.scalar_static_f64[556]*((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(v1-f64::powf(v2213,self.scalar_static_f64[227])))+(self.scalar_static_f64[833]*(v766-v2211))))+(self.scalar_static_f64[557]*v766)))*self.scalar_static_f64[298]))+(if self.scalar_static_bool[67]{(self.scalar_static_f64[6]*v2348)}else{v2348})));
        let v2585=ctx.node_voltage(nodes[11]);
        let v2601=(if v779{(v781*self.scalar_static_f64[897])}else{(if v776{(v777*self.scalar_static_f64[897])}else{v4})});
        let v2602=(if v779{(v781*self.scalar_static_f64[898])}else{(if v776{(v777*self.scalar_static_f64[898])}else{v4})});
        let v2611=(if v791{(v792*self.scalar_static_f64[899])}else{(if v788{(v789*self.scalar_static_f64[899])}else{v4})});
        let v2612=(if v791{(v792*self.scalar_static_f64[900])}else{(if v788{(v789*self.scalar_static_f64[900])}else{v4})});
        let v2627=(if v801{(v802*self.scalar_static_f64[897])}else{(if v798{(v799*self.scalar_static_f64[897])}else{v4})});
        let v2628=(if v801{(v802*self.scalar_static_f64[901])}else{(if v798{(v799*self.scalar_static_f64[901])}else{v4})});
        let v2629=(if v801{(v802*self.scalar_static_f64[902])}else{(if v798{(v799*self.scalar_static_f64[902])}else{v4})});
        let v2630=(if v801{(v802*self.scalar_static_f64[898])}else{(if v798{(v799*self.scalar_static_f64[898])}else{v4})});
        let v2652=(if v821{(v822*self.scalar_static_f64[901])}else{(if v818{(v819*self.scalar_static_f64[901])}else{v4})});
        let v2653=(if v821{(v822*self.scalar_static_f64[903])}else{(if v818{(v819*self.scalar_static_f64[903])}else{v4})});
        let v2654=(if v821{(v822*self.scalar_static_f64[902])}else{(if v818{(v819*self.scalar_static_f64[902])}else{v4})});
        let v2655=(if v821{(v822*self.scalar_static_f64[898])}else{(if v818{(v819*self.scalar_static_f64[898])}else{v4})});
        let v2673=(if v841{(v842*self.scalar_static_f64[897])}else{(if v838{(v839*self.scalar_static_f64[897])}else{v4})});
        let v2674=(if v841{(v842*self.scalar_static_f64[902])}else{(if v838{(v839*self.scalar_static_f64[902])}else{v4})});
        let v2675=(if v841{(v842*self.scalar_static_f64[898])}else{(if v838{(v839*self.scalar_static_f64[898])}else{v4})});
        let v2726=(if v884{(v885*self.scalar_static_f64[897])}else{(if v881{(v882*self.scalar_static_f64[897])}else{v4})});
        let v2727=(if v884{(v885*self.scalar_static_f64[898])}else{(if v881{(v882*self.scalar_static_f64[898])}else{v4})});
        let v2734=(if v895{(v896*self.scalar_static_f64[897])}else{(if v892{(v893*self.scalar_static_f64[897])}else{v4})});
        let v2735=(if v895{(v896*self.scalar_static_f64[898])}else{(if v892{(v893*self.scalar_static_f64[898])}else{v4})});
        let v2738=(v32*v903);
        let v2739=((v436*v2726)/v2738);
        let v2740=((v436*v2727)/v2738);
        let v2743=(v32*v906);
        let v2744=((v436*v2734)/v2743);
        let v2745=((v436*v2735)/v2743);
        let v2751=(v908*v908);
        let v2757=(if v911{v4}else{(((v908*(v32*v2734))-(v907*v2744))/v2751)});
        let v2758=(if v911{v4}else{(((v908*(v32*v2735))-(v907*v2745))/v2751)});
        let v2775=(self.scalar_static_f64[381]*((v2739-v2744)-((((v908*v2739)-(v914*v2744))/v2751)/v915)));
        let v2776=(self.scalar_static_f64[381]*((-v2745)-(((-(v914*v2745))/v2751)/v915)));
        let v2777=(self.scalar_static_f64[381]*(v2740-((v2740/v908)/v915)));
        let v2779=(self.scalar_static_f64[321]+v2777);
        let v2780=(v2775/self.scalar_static_f64[578]);
        let v2781=((self.scalar_static_f64[0]+v2776)/self.scalar_static_f64[578]);
        let v2782=(v2779/self.scalar_static_f64[578]);
        let v2792=(self.scalar_static_f64[578]*(v424*v2780));
        let v2793=(self.scalar_static_f64[578]*(v424*v2781));
        let v2794=(self.scalar_static_f64[578]*(v424*v2782));
        let v2806=(if v921{((self.scalar_static_f64[818]*((self.scalar_static_f64[382]*v2792)/v937))-(if v927{(self.scalar_static_f64[0]/v929)}else{(if v924{self.scalar_static_f64[0]}else{v4})}))}else{v4});
        let v2807=(if v921{((self.scalar_static_f64[818]*((self.scalar_static_f64[382]*v2793)/v937))-(if v927{(self.scalar_static_f64[321]/v929)}else{(if v924{self.scalar_static_f64[321]}else{v4})}))}else{v4});
        let v2808=(if v921{(self.scalar_static_f64[818]*((self.scalar_static_f64[382]*v2794)/v937))}else{v4});
        let v2809=(v942*v2806);
        let v2811=(v942*v2807);
        let v2813=(v942*v2808);
        let v2818=(v32*v954);
        let v2819=((if v921{(v2809+v2809)}else{v4})/v2818);
        let v2820=((if v921{(v2811+v2811)}else{v4})/v2818);
        let v2821=((if v921{(v2813+v2813)}else{v4})/v2818);
        let v2827=(v955*v955);
        let v2844=(if v959{(v424*(v2806+v2819))}else{(if v951{((-(v952*(v2819-v2806)))/v2827)}else{v4})});
        let v2845=(if v959{(v424*(v2807+v2820))}else{(if v951{((-(v952*(v2820-v2807)))/v2827)}else{v4})});
        let v2846=(if v959{(v424*(v2808+v2821))}else{(if v951{((-(v952*(v2821-v2808)))/v2827)}else{v4})});
        let v2862=(v970*v970);
        let v2872=(if v921{(((v970*((v966*v2844)+(v962*v2844)))-(v967*(self.scalar_static_f64[208]*v2844)))/v2862)}else{v4});
        let v2873=(if v921{(((v970*((v966*v2845)+(v962*v2845)))-(v967*(self.scalar_static_f64[208]*v2845)))/v2862)}else{v4});
        let v2874=(if v921{(((v970*((v966*v2846)+(v962*v2846)))-(v967*(self.scalar_static_f64[208]*v2846)))/v2862)}else{v4});
        let v2878=(v972*v972);
        let v2888=(if v921{(((v972*v2780)-(v920*v2872))/v2878)}else{v4});
        let v2889=(if v921{(((v972*v2781)-(v920*v2873))/v2878)}else{v4});
        let v2890=(if v921{(((v972*v2782)-(v920*v2874))/v2878)}else{v4});
        let v2894=(if v921{(v2888/self.scalar_static_f64[210])}else{v4});
        let v2895=(if v921{(v2889/self.scalar_static_f64[210])}else{v4});
        let v2896=(if v921{(v2890/self.scalar_static_f64[210])}else{v4});
        let v2930=(if v921{((if v988{(v2888+(self.scalar_static_f64[210]*((v990*(-v2894))/v991)))}else{(if v980{(self.scalar_static_f64[210]*((v981*v2894)/v982))}else{v4})})/self.scalar_static_f64[216])}else{v4});
        let v2931=(if v921{((if v988{(v2889+(self.scalar_static_f64[210]*((v990*(-v2895))/v991)))}else{(if v980{(self.scalar_static_f64[210]*((v981*v2895)/v982))}else{v4})})/self.scalar_static_f64[216])}else{v4});
        let v2932=(if v921{((if v988{(v2890+(self.scalar_static_f64[210]*((v990*(-v2896))/v991)))}else{(if v980{(self.scalar_static_f64[210]*((v981*v2896)/v982))}else{v4})})/self.scalar_static_f64[216])}else{v4});
        let v2936=(if v921{(v2844/self.scalar_static_f64[209])}else{v4});
        let v2937=(if v921{(v2845/self.scalar_static_f64[209])}else{v4});
        let v2938=(if v921{(v2846/self.scalar_static_f64[209])}else{v4});
        let v2960=(v32*v1012);
        let v2979=(v1015*v1015);
        let v2989=(if v921{(((v1015*(((v1009*((v1007*v2936)+(v1006*(v436*v2930))))+(v1008*v2936))/v2960))-(v1013*((v1014*v2936)+(v1009*(v32*v2930)))))/v2979)}else{v4});
        let v2990=(if v921{(((v1015*(((v1009*((v1007*v2937)+(v1006*(v436*v2931))))+(v1008*v2937))/v2960))-(v1013*((v1014*v2937)+(v1009*(v32*v2931)))))/v2979)}else{v4});
        let v2991=(if v921{(((v1015*(((v1009*((v1007*v2938)+(v1006*(v436*v2932))))+(v1008*v2938))/v2960))-(v1013*((v1014*v2938)+(v1009*(v32*v2932)))))/v2979)}else{v4});
        let v2997=((v1017*v2757)+(v912*v2989));
        let v3000=((v1017*v2758)+(v912*v2990));
        let v3001=(v912*v2991);
        let v3008=(v1021*v1021);
        let v3018=(if v921{(((v1021*((-v2989)+v2997))-(v1020*v2997))/v3008)}else{v4});
        let v3019=(if v921{(((v1021*((-v2990)+v3000))-(v1020*v3000))/v3008)}else{v4});
        let v3020=(if v921{(((v1021*((-v2991)+v3001))-(v1020*v3001))/v3008)}else{v4});
        let v3033=(if v921{(self.scalar_static_f64[382]*((v1023*v2792)+(v935*v3018)))}else{v4});
        let v3034=(if v921{(self.scalar_static_f64[382]*((v1023*v2793)+(v935*v3019)))}else{v4});
        let v3035=(if v921{(self.scalar_static_f64[382]*((v1023*v2794)+(v935*v3020)))}else{v4});
        let v3051=(if v921{((v32*v3033)+((v1029*v2757)+(v912*(v2757+v3033))))}else{v4});
        let v3052=(if v921{((v32*v3034)+((v1029*v2758)+(v912*(v2758+v3034))))}else{v4});
        let v3053=(if v921{((v32*v3035)+(v912*v3035))}else{v4});
        let v3057=(if v921{(v424*v3033)}else{v4});
        let v3058=(if v921{(v424*v3034)}else{v4});
        let v3059=(if v921{(v424*v3035)}else{v4});
        let v3060=(v1035*v3057);
        let v3062=(v1035*v3058);
        let v3064=(v1035*v3059);
        let v3069=(if v921{(v3051+(v3060+v3060))}else{v4});
        let v3070=(if v921{(v3052+(v3062+v3062))}else{v4});
        let v3071=(if v921{(v3053+(v3064+v3064))}else{v4});
        let v3072=(v32*v1041);
        let v3073=(v3069/v3072);
        let v3074=(v3070/v3072);
        let v3075=(v3071/v3072);
        let v3088=(v1046*v1046);
        let v3101=(if v1051{v4}else{(if v1045{(((v1046*v3051)-(v1032*(v3073-v3057)))/v3088)}else{(if v1040{(v3057+v3073)}else{v4})})});
        let v3102=(if v1051{v4}else{(if v1045{(((v1046*v3052)-(v1032*(v3074-v3058)))/v3088)}else{(if v1040{(v3058+v3074)}else{v4})})});
        let v3103=(if v1051{v4}else{(if v1045{(((v1046*v3053)-(v1032*(v3075-v3059)))/v3088)}else{(if v1040{(v3059+v3075)}else{v4})})});
        let v3122=(if v921{(self.scalar_static_f64[218]*v2780)}else{v4});
        let v3123=(if v921{(self.scalar_static_f64[218]*v2781)}else{v4});
        let v3124=(if v921{(self.scalar_static_f64[218]*v2782)}else{v4});
        let v3131=(v1062*v3122);
        let v3133=(v1062*v3123);
        let v3135=(v1062*v3124);
        let v3140=(v32*v1069);
        let v3159=(v1080*v1080);
        let v3175=(self.scalar_static_f64[207]*v2780);
        let v3176=(self.scalar_static_f64[207]*v2781);
        let v3177=(self.scalar_static_f64[207]*v2782);
        let v3181=(v1086*v1086);
        let v3208=(v914*v914);
        let v3216=(if v1091{(((v914*(v32*v2727))-(v1092*v2740))/v3208)}else{v3103});
        let v3217=(if v1091{v2601}else{(if v921{(self.scalar_static_f64[822]*((v1053*v3101)+(v1052*v3101)))}else{v4})});
        let v3218=(if v1091{v4}else{(if v921{(self.scalar_static_f64[822]*((v1053*v3102)+(v1052*v3102)))}else{v4})});
        let v3219=(if v1091{v2602}else{(if v921{(self.scalar_static_f64[822]*((v1053*v3103)+(v1052*v3103)))}else{v4})});
        let v3220=(v2757+(if v1091{(((v914*(v32*v2726))-(v1092*v2739))/v3208)}else{v3101}));
        let v3221=(v2758+(if v1091{v4}else{v3102}));
        let v3225=(if v1107{(v424*v3220)}else{v4});
        let v3226=(if v1107{(v424*v3221)}else{v4});
        let v3227=(if v1107{(v424*v3216)}else{v4});
        let v3231=(v1111*v1111);
        let v3250=(v1117*v1117);
        let v3260=(if v1115{(((v1117*v2775)-(v918*((self.scalar_static_f64[0]+v2775)-self.scalar_static_f64[0])))/v3250)}else{(if v1107{(((v1111*v3225)-(v1110*v3225))/v3231)}else{v3018})});
        let v3261=(if v1115{(((v1117*v2776)-(v918*(v2776-self.scalar_static_f64[321])))/v3250)}else{(if v1107{(((v1111*v3226)-(v1110*v3226))/v3231)}else{v3019})});
        let v3262=(if v1115{(((v1117*v2777)-(v918*v2779))/v3250)}else{(if v1107{(((v1111*v3227)-(v1110*v3227))/v3231)}else{v3020})});
        let v3266=(if v1091{v4}else{(if v1078{(self.scalar_static_f64[502]*(((v1080*(v32*v2780))-(v1079*(v2780+v2872)))/v3159))}else{v4})});
        let v3267=(if v1091{v4}else{(if v1078{(self.scalar_static_f64[502]*(((v1080*(v32*v2781))-(v1079*(v2781+v2873)))/v3159))}else{v4})});
        let v3268=(if v1091{v4}else{(if v1078{(self.scalar_static_f64[502]*(((v1080*(v32*v2782))-(v1079*(v2782+v2874)))/v3159))}else{v4})});
        let v3269=(if v1091{v2780}else{(if v921{(((v1086*v3175)-(v1085*v2780))/v3181)}else{v4})});
        let v3270=(if v1091{v2781}else{(if v921{(((v1086*v3176)-(v1085*v2781))/v3181)}else{v4})});
        let v3271=(if v1091{v2782}else{(if v921{(((v1086*v3177)-(v1085*v2782))/v3181)}else{v4})});
        let v3278=(if v1091{(-(v3269/self.scalar_static_f64[207]))}else{(if v921{((-v3175)/v3181)}else{v4})});
        let v3279=(if v1091{(-(v3270/self.scalar_static_f64[207]))}else{(if v921{((-v3176)/v3181)}else{v4})});
        let v3280=(if v1091{(-(v3271/self.scalar_static_f64[207]))}else{(if v921{((-v3177)/v3181)}else{v4})});
        let v3303=(if v1140{(-(self.scalar_static_f64[829]*((v1142*self.scalar_static_f64[906])/v1143)))}else{(if v1133{(self.scalar_static_f64[321]-(self.scalar_static_f64[829]*((v1134*self.scalar_static_f64[904])/v1135)))}else{v4})});
        let v3304=(if v1140{(-(self.scalar_static_f64[829]*((v1142*self.scalar_static_f64[907])/v1143)))}else{(if v1133{(self.scalar_static_f64[0]-(self.scalar_static_f64[829]*((v1134*self.scalar_static_f64[905])/v1135)))}else{v4})});
        let v3307=(-(self.scalar_static_f64[541]*v3303));
        let v3308=(-(self.scalar_static_f64[541]*v3304));
        let v3311=(self.scalar_static_f64[223]*f64::powf(v1149,self.scalar_static_f64[325]));
        let v3312=(v3307*v3311);
        let v3313=(v3308*v3311);
        let v3322=((self.scalar_static_f64[830]*(-v3312))+(v155*(self.scalar_static_f64[321]-v3303)));
        let v3323=((self.scalar_static_f64[830]*(-v3313))+(v155*(self.scalar_static_f64[0]-v3304)));
        let v3331=(if self.scalar_static_bool[26]{self.scalar_static_f64[0]}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[0]+(if v1091{v4}else{(if v921{(v3122+(((if v921{(self.scalar_static_f64[824]*v2780)}else{v4})+(v3131+v3131))/v3140))}else{v4})}))}else{self.scalar_static_f64[326]})});
        let v3332=(if self.scalar_static_bool[26]{v4}else{(if self.scalar_static_bool[24]{(self.scalar_static_f64[321]+(if v1091{self.scalar_static_f64[0]}else{(if v921{(v3123+(((if v921{(self.scalar_static_f64[824]*v2781)}else{v4})+(v3133+v3133))/v3140))}else{v4})}))}else{self.scalar_static_f64[327]})});
        let v3333=(if self.scalar_static_bool[26]{self.scalar_static_f64[321]}else{(if self.scalar_static_bool[24]{(if v1091{self.scalar_static_f64[321]}else{(if v921{(v3124+(((if v921{(self.scalar_static_f64[824]*v2782)}else{v4})+(v3135+v3135))/v3140))}else{v4})})}else{v4})});
        let v3337=(v1121*v1121);
        let v3338=(((v1121*v3331)-(v1176*v3266))/v3337);
        let v3342=(((v1121*v3332)-(v1176*v3267))/v3337);
        let v3346=(((v1121*v3333)-(v1176*v3268))/v3337);
        let v3389=(if v1185{(-((v1189*v3266)+(v1121*((v1187*(-v3338))/v1188))))}else{(if v1178{(v3331-((v1181*v3266)+(v1121*((v1179*v3338)/v1180))))}else{v4})});
        let v3390=(if v1185{(-((v1189*v3267)+(v1121*((v1187*(-v3342))/v1188))))}else{(if v1178{(v3332-((v1181*v3267)+(v1121*((v1179*v3342)/v1180))))}else{v4})});
        let v3391=(if v1185{(-((v1189*v3268)+(v1121*((v1187*(-v3346))/v1188))))}else{(if v1178{(v3333-((v1181*v3268)+(v1121*((v1179*v3346)/v1180))))}else{v4})});
        let v3394=(self.scalar_static_f64[226]*f64::powf(v1125,self.scalar_static_f64[328]));
        let v3395=(v3278*v3394);
        let v3396=(v3279*v3394);
        let v3397=(v3280*v3394);
        let v3406=(self.scalar_static_f64[227]*f64::powf(v1198,self.scalar_static_f64[329]));
        let v3445=(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((v1199*v3397)+(v1194*((-(v3391/self.scalar_static_f64[502]))*v3406)))))+((v1204*(self.scalar_static_f64[833]*v3397))+(v1203*(v3333-v3391)))));
        let v3448=((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((v1199*v3395)+(v1194*((-(v3389/self.scalar_static_f64[502]))*v3406)))))+((v1204*(self.scalar_static_f64[833]*v3395))+(v1203*(v3331-v3389)))))+self.scalar_static_f64[908]);
        let v3449=((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((v1199*v3396)+(v1194*((-(v3390/self.scalar_static_f64[502]))*v3406)))))+((v1204*(self.scalar_static_f64[833]*v3396))+(v1203*(v3332-v3390)))))+self.scalar_static_f64[909]);
        let v3450=(self.scalar_static_f64[839]*v2611);
        let v3451=(self.scalar_static_f64[839]*v2612);
        let v3452=(v32*v1214);
        let v3453=(v3450/v3452);
        let v3454=(v3451/v3452);
        let v3458=(v1215*v1215);
        let v3459=(((v1215*v3450)-(v1212*v3453))/v3458);
        let v3463=(((v1215*v3451)-(v1212*v3454))/v3458);
        let v3466=(self.scalar_static_f64[840]*f64::powf(v1095,self.scalar_static_f64[910]));
        let v3467=(v3217*v3466);
        let v3468=(v3218*v3466);
        let v3469=(v3219*v3466);
        let v3470=(self.scalar_static_f64[839]*v3467);
        let v3471=(self.scalar_static_f64[839]*v3468);
        let v3472=(self.scalar_static_f64[839]*v3469);
        let v3473=(v32*v1221);
        let v3480=(v1222*v1222);
        let v3481=(((v1222*v3470)-(v1219*(v3470/v3473)))/v3480);
        let v3485=(((v1222*v3471)-(v1219*(v3471/v3473)))/v3480);
        let v3489=(((v1222*v3472)-(v1219*(v3472/v3473)))/v3480);
        let v3490=(v3322/self.scalar_static_f64[761]);
        let v3491=(v3323/self.scalar_static_f64[761]);
        let v3492=(v3448/self.scalar_static_f64[759]);
        let v3493=(v3449/self.scalar_static_f64[759]);
        let v3494=(v3445/self.scalar_static_f64[759]);
        let v3495=(v3491+v3492);
        let v3533=(if self.scalar_static_bool[28]{((v1239*(if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*v3490))}else{v4}))/self.scalar_static_f64[843])}else{(if self.scalar_static_bool[27]{v3490}else{v4})});
        let v3534=(if self.scalar_static_bool[28]{(((v1239*(if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*v3491))}else{v4}))-(v1240*(if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*((-v3448)/self.scalar_static_f64[759])))}else{v4})))/self.scalar_static_f64[843])}else{(if self.scalar_static_bool[27]{v3495}else{v4})});
        let v3535=(if self.scalar_static_bool[28]{((-(v1240*(if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*((-v3449)/self.scalar_static_f64[759])))}else{v4})))/self.scalar_static_f64[843])}else{(if self.scalar_static_bool[27]{v3493}else{v4})});
        let v3536=(if self.scalar_static_bool[28]{((-(v1240*(if self.scalar_static_bool[28]{(self.scalar_static_f64[382]*(self.scalar_static_f64[805]*((-v3445)/self.scalar_static_f64[759])))}else{v4})))/self.scalar_static_f64[843])}else{(if self.scalar_static_bool[27]{v3494}else{v4})});
        let v3537=(v1246*v3533);
        let v3538=(v3537+v3537);
        let v3539=(v1246*v3534);
        let v3540=(v3539+v3539);
        let v3541=(v1246*v3535);
        let v3542=(v3541+v3541);
        let v3543=(v1246*v3536);
        let v3544=(v3543+v3543);
        let v3545=(v32*v1252);
        let v3546=(v3538/v3545);
        let v3547=(v3540/v3545);
        let v3548=(v3542/v3545);
        let v3549=(v3544/v3545);
        let v3556=(v1253*v1253);
        let v3584=(v424*v3459);
        let v3585=(v424*(v3463+v3481));
        let v3586=(v424*v3485);
        let v3587=(v424*v3489);
        let v3590=((v1262*(if v1256{(v424*(v3533+v3546))}else{(if v1249{((-(v1250*(v3546-v3533)))/v3556)}else{v4})}))+(v1259*v3584));
        let v3593=((v1262*(if v1256{(v424*(v3534+v3547))}else{(if v1249{((-(v1250*(v3547-v3534)))/v3556)}else{v4})}))+(v1259*v3585));
        let v3596=((v1262*(if v1256{(v424*(v3535+v3548))}else{(if v1249{((-(v1250*(v3548-v3535)))/v3556)}else{v4})}))+(v1259*v3586));
        let v3599=((v1262*(if v1256{(v424*(v3536+v3549))}else{(if v1249{((-(v1250*(v3549-v3536)))/v3556)}else{v4})}))+(v1259*v3587));
        let v3600=(self.scalar_static_f64[844]*v3467);
        let v3601=(self.scalar_static_f64[844]*v3468);
        let v3602=(self.scalar_static_f64[844]*v3469);
        let v3604=(self.scalar_static_f64[644]*v2612);
        let v3608=(v1263*(self.scalar_static_f64[644]*v2611));
        let v3611=(v1263*v1263);
        let v3645=(if v1278{(self.scalar_static_f64[321]+(v1270*((v1280*self.scalar_static_f64[332])/v1281)))}else{(if v1272{(v1270*((v1273*self.scalar_static_f64[330])/v1274))}else{v4})});
        let v3646=(if v1278{(self.scalar_static_f64[0]+(v1270*((v1280*self.scalar_static_f64[333])/v1281)))}else{(if v1272{(v1270*((v1273*self.scalar_static_f64[331])/v1274))}else{v4})});
        let v3705=(if v1326{(v1327*self.scalar_static_f64[911])}else{(if v1323{(v1324*self.scalar_static_f64[911])}else{v3645})});
        let v3706=(if v1326{(v1327*self.scalar_static_f64[912])}else{(if v1323{(v1324*self.scalar_static_f64[912])}else{v3646})});
        let v3839=(if v1401{(v1402*self.scalar_static_f64[913])}else{(if v1398{(v1399*self.scalar_static_f64[913])}else{v3705})});
        let v3840=(if v1401{(v1402*self.scalar_static_f64[914])}else{(if v1398{(v1399*self.scalar_static_f64[914])}else{v4})});
        let v3841=(if v1401{v4}else{(if v1398{v4}else{v3706})});
        let v3896=(if v1436{(v1437*self.scalar_static_f64[915])}else{(if v1433{(v1434*self.scalar_static_f64[915])}else{v3839})});
        let v3897=(if v1436{v4}else{(if v1433{v4}else{v3840})});
        let v3898=(if v1436{(v1437*self.scalar_static_f64[916])}else{(if v1433{(v1434*self.scalar_static_f64[916])}else{v3841})});
        let v3911=(if v1448{(v1449*self.scalar_static_f64[917])}else{(if v1445{(v1446*self.scalar_static_f64[917])}else{v3896})});
        let v3912=(if v1448{(v1449*self.scalar_static_f64[918])}else{(if v1445{(v1446*self.scalar_static_f64[918])}else{v3897})});
        let v3913=(if v1448{v4}else{(if v1445{v4}else{v3898})});
        let v3934=(if v1460{v4}else{(if v1457{v4}else{v3911})});
        let v3935=(if v1460{(v1461*self.scalar_static_f64[919])}else{(if v1457{(v1458*self.scalar_static_f64[919])}else{v3912})});
        let v3936=(if v1460{(v1461*self.scalar_static_f64[920])}else{(if v1457{(v1458*self.scalar_static_f64[920])}else{v3913})});
        let v3937=(if v1460{(v1461*self.scalar_static_f64[921])}else{(if v1457{(v1458*self.scalar_static_f64[921])}else{v4})});
        let v3938=(if v1460{(v1461*self.scalar_static_f64[922])}else{(if v1457{(v1458*self.scalar_static_f64[922])}else{v4})});
        let v3955=(if v1472{(v1473*self.scalar_static_f64[923])}else{(if v1469{(v1470*self.scalar_static_f64[923])}else{v3934})});
        let v3956=(if v1472{(v1473*self.scalar_static_f64[924])}else{(if v1469{(v1470*self.scalar_static_f64[924])}else{v3935})});
        let v3957=(if v1472{v4}else{(if v1469{v4}else{v3936})});
        let v3958=(if v1472{v4}else{(if v1469{v4}else{v3937})});
        let v3959=(if v1472{v4}else{(if v1469{v4}else{v3938})});
        let v4297=(self.scalar_static_f64[839]*v2627);
        let v4298=(self.scalar_static_f64[839]*v2628);
        let v4299=(self.scalar_static_f64[839]*v2629);
        let v4300=(self.scalar_static_f64[839]*v2630);
        let v4301=(v436*(if v873{(v874*self.scalar_static_f64[897])}else{(if v870{(v871*self.scalar_static_f64[897])}else{v4})}));
        let v4302=(v436*(if v873{(v874*self.scalar_static_f64[901])}else{(if v870{(v871*self.scalar_static_f64[901])}else{v4})}));
        let v4303=(v436*(if v873{(v874*self.scalar_static_f64[902])}else{(if v870{(v871*self.scalar_static_f64[902])}else{v4})}));
        let v4304=(v436*(if v873{(v874*self.scalar_static_f64[898])}else{(if v870{(v871*self.scalar_static_f64[898])}else{v4})}));
        let v4305=(v32*v1664);
        let v4313=(v1665*v1665);
        let v4327=(v32*v1668);
        let v4335=(v1669*v1669);
        let v4569=(v32*v1755);
        let v4577=(v1756*v1756);
        let v4591=(if self.scalar_static_bool[44]{(((v1756*(self.scalar_static_f64[859]*v2652))-(v1752*((self.scalar_static_f64[851]*v2652)/v4569)))/v4577)}else{v4});
        let v4592=(if self.scalar_static_bool[44]{(((v1756*(self.scalar_static_f64[859]*v2653))-(v1752*((self.scalar_static_f64[851]*v2653)/v4569)))/v4577)}else{v4});
        let v4593=(if self.scalar_static_bool[44]{(((v1756*(self.scalar_static_f64[859]*v2654))-(v1752*((self.scalar_static_f64[851]*v2654)/v4569)))/v4577)}else{v4});
        let v4594=(if self.scalar_static_bool[44]{(((v1756*(self.scalar_static_f64[859]*v2655))-(v1752*((self.scalar_static_f64[851]*v2655)/v4569)))/v4577)}else{v4});
        let v4598=(self.scalar_static_f64[860]*v2652);
        let v4599=(self.scalar_static_f64[860]*v2653);
        let v4602=(self.scalar_static_f64[860]*v2654);
        let v4609=(self.scalar_static_f64[862]*v2652);
        let v4610=(self.scalar_static_f64[862]*v2653);
        let v4613=(self.scalar_static_f64[862]*v2654);
        let v4615=(v32*v1771);
        let v4625=(v1772*v1772);
        let v4655=(v32*v1779);
        let v4663=(v1780*v1780);
        let v4672=(((v1780*v4602)-(v1776*(v4613/v4655)))/v4663);
        let v4677=(if self.scalar_static_bool[46]{(((v1780*v4598)-(v1776*(v4609/v4655)))/v4663)}else{(if self.scalar_static_bool[45]{(((v1772*v4598)-(v1764*(v4609/v4615)))/v4625)}else{v4})});
        let v4678=(if self.scalar_static_bool[46]{(((v1780*v4599)-(v1776*(v4610/v4655)))/v4663)}else{(if self.scalar_static_bool[45]{(((v1772*v4599)-(v1764*(v4610/v4615)))/v4625)}else{v4})});
        let v4679=(if self.scalar_static_bool[46]{v4}else{(if self.scalar_static_bool[45]{(((v1772*(self.scalar_static_f64[860]*(-v2673)))-(v1764*((self.scalar_static_f64[862]*(self.scalar_static_f64[245]*v2673))/v4615)))/v4625)}else{v4})});
        let v4680=(if self.scalar_static_bool[46]{v4672}else{(if self.scalar_static_bool[45]{(((v1772*(self.scalar_static_f64[860]*(v2654-v2674)))-(v1764*((self.scalar_static_f64[862]*(v2654+(self.scalar_static_f64[245]*v2674)))/v4615)))/v4625)}else{v4})});
        let v4681=(if self.scalar_static_bool[46]{v4672}else{(if self.scalar_static_bool[45]{(((v1772*v4602)-(v1764*(v4613/v4615)))/v4625)}else{v4})});
        let v4682=(if self.scalar_static_bool[46]{(((v1780*(self.scalar_static_f64[860]*v2655))-(v1776*((self.scalar_static_f64[862]*v2655)/v4655)))/v4663)}else{(if self.scalar_static_bool[45]{(((v1772*(self.scalar_static_f64[860]*(v2655-v2675)))-(v1764*((self.scalar_static_f64[862]*(v2655+(self.scalar_static_f64[245]*v2675)))/v4615)))/v4625)}else{v4})});
        let v4687=(v1795*self.scalar_static_f64[348]);
        let v4688=(v4687+v4687);
        let v4689=(v1795*self.scalar_static_f64[349]);
        let v4691=(v1795*self.scalar_static_f64[350]);
        let v4692=(v4691+v4691);
        let v4693=(v1795*self.scalar_static_f64[351]);
        let v4695=(if self.scalar_static_bool[48]{v4688}else{v4});
        let v4696=(if self.scalar_static_bool[48]{(v4689+v4689)}else{v4});
        let v4697=(if self.scalar_static_bool[48]{v4}else{v3538});
        let v4698=(if self.scalar_static_bool[48]{v4688}else{v3540});
        let v4699=(if self.scalar_static_bool[48]{v4692}else{v3542});
        let v4700=(if self.scalar_static_bool[48]{v4692}else{v3544});
        let v4701=(if self.scalar_static_bool[48]{(v4693+v4693)}else{v4});
        let v4702=(if self.scalar_static_bool[48]{v4692}else{v4});
        let v4703=(v32*v1804);
        let v4704=(v4695/v4703);
        let v4705=(v4696/v4703);
        let v4706=(v4697/v4703);
        let v4707=(v4698/v4703);
        let v4708=(v4699/v4703);
        let v4709=(v4700/v4703);
        let v4710=(v4701/v4703);
        let v4711=(v4702/v4703);
        let v4721=(v1805*v1805);
        let v4767=(if v1809{(v424*(self.scalar_static_f64[348]+v4704))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4704-self.scalar_static_f64[348])))/v4721)}else{v4})});
        let v4768=(if v1809{(v424*(self.scalar_static_f64[349]+v4705))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4705-self.scalar_static_f64[349])))/v4721)}else{v4})});
        let v4769=(if v1809{(v424*v4706)}else{(if v1801{((-(self.scalar_static_f64[254]*v4706))/v4721)}else{v4})});
        let v4770=(if v1809{(v424*(self.scalar_static_f64[348]+v4707))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4707-self.scalar_static_f64[348])))/v4721)}else{v4})});
        let v4771=(if v1809{(v424*(self.scalar_static_f64[350]+v4708))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4708-self.scalar_static_f64[350])))/v4721)}else{v4})});
        let v4772=(if v1809{(v424*(self.scalar_static_f64[350]+v4709))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4709-self.scalar_static_f64[350])))/v4721)}else{v4})});
        let v4773=(if v1809{(v424*(self.scalar_static_f64[351]+v4710))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4710-self.scalar_static_f64[351])))/v4721)}else{v4})});
        let v4774=(if v1809{(v424*(self.scalar_static_f64[350]+v4711))}else{(if v1801{((-(self.scalar_static_f64[254]*(v4711-self.scalar_static_f64[350])))/v4721)}else{v4})});
        let v4780=(self.scalar_static_f64[571]*(v4591+v4677));
        let v4783=(self.scalar_static_f64[571]*(v4593+v4680));
        let v4796=(v1816*v1816);
        let v4838=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4767)-(v1812*(v4767+v4780)))/v4796)}else{v4})});
        let v4839=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4768)-(v1812*(v4768+(self.scalar_static_f64[571]*(v4592+v4678)))))/v4796)}else{v4})});
        let v4840=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{((-(v1812*(self.scalar_static_f64[571]*v4679)))/v4796)}else{v4})});
        let v4841=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4769)-(v1812*v4769))/v4796)}else{v4})});
        let v4842=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4770)-(v1812*(v4770+v4780)))/v4796)}else{v4})});
        let v4843=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4771)-(v1812*(v4771+v4783)))/v4796)}else{v4})});
        let v4844=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4772)-(v1812*(v4772+(self.scalar_static_f64[571]*(v4593+v4681)))))/v4796)}else{v4})});
        let v4845=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4773)-(v1812*(v4773+(self.scalar_static_f64[571]*(v4594+v4682)))))/v4796)}else{v4})});
        let v4846=(if self.scalar_static_bool[50]{v4}else{(if self.scalar_static_bool[48]{(((v1816*v4774)-(v1812*(v4774+v4783)))/v4796)}else{v4})});
        let v5144=(v1228*v3490);
        let v5146=(v1228*v3495);
        let v5148=(v1228*v3493);
        let v5150=(v1228*v3494);
        let v5152=(v32*v1884);
        let v5153=((v5144+v5144)/v5152);
        let v5154=((v5146+v5146)/v5152);
        let v5155=((v5148+v5148)/v5152);
        let v5156=((v5150+v5150)/v5152);
        let v5163=(v1885*v1885);
        let v5186=(if v1888{(v424*(v3490+v5153))}else{(if v1882{((-(v1250*(v5153-v3490)))/v5163)}else{v4})});
        let v5187=(if v1888{(v424*(v3495+v5154))}else{(if v1882{((-(v1250*(v5154-v3495)))/v5163)}else{v4})});
        let v5188=(if v1888{(v424*(v3493+v5155))}else{(if v1882{((-(v1250*(v5155-v3493)))/v5163)}else{v4})});
        let v5189=(if v1888{(v424*(v3494+v5156))}else{(if v1882{((-(v1250*(v5156-v3494)))/v5163)}else{v4})});
        let v6067=(if v2168{(-(self.scalar_static_f64[829]*((v2170*self.scalar_static_f64[906])/v2171)))}else{(if v2161{(self.scalar_static_f64[321]-(self.scalar_static_f64[829]*((v2162*self.scalar_static_f64[904])/v2163)))}else{v4})});
        let v6068=(if v2168{(-(self.scalar_static_f64[829]*((v2170*self.scalar_static_f64[907])/v2171)))}else{(if v2161{(self.scalar_static_f64[0]-(self.scalar_static_f64[829]*((v2162*self.scalar_static_f64[905])/v2163)))}else{v4})});
        let v6074=(self.scalar_static_f64[223]*f64::powf(v2178,self.scalar_static_f64[325]));
        let v6096=((v2191*v5186)+(v1891*(self.scalar_static_f64[880]*v3459)));
        let v6099=((v2191*v5187)+(v1891*(self.scalar_static_f64[880]*v3463)));
        let v6100=(v2191*v5188);
        let v6101=(v2191*v5189);
        let v6105=(v2193*v5186);
        let v6108=((v2193*v5187)+(v1891*(self.scalar_static_f64[880]*v3481)));
        let v6111=((v2193*v5188)+(v1891*(self.scalar_static_f64[880]*v3485)));
        let v6114=((v2193*v5189)+(v1891*(self.scalar_static_f64[880]*v3489)));
        let v6159=(if v2204{(-(self.scalar_static_f64[825]*((v2206*self.scalar_static_f64[941])/v2207)))}else{(if v2197{(self.scalar_static_f64[0]-(self.scalar_static_f64[825]*((v2198*self.scalar_static_f64[937])/v2199)))}else{v4})});
        let v6160=(if v2204{(-(self.scalar_static_f64[825]*((v2206*self.scalar_static_f64[942])/v2207)))}else{(if v2197{(self.scalar_static_f64[322]-(self.scalar_static_f64[825]*((v2198*self.scalar_static_f64[938])/v2199)))}else{v4})});
        let v6161=(if v2204{(-(self.scalar_static_f64[825]*((v2206*self.scalar_static_f64[943])/v2207)))}else{(if v2197{(self.scalar_static_f64[323]-(self.scalar_static_f64[825]*((v2198*self.scalar_static_f64[939])/v2199)))}else{v4})});
        let v6162=(if v2204{(-(self.scalar_static_f64[825]*((v2206*self.scalar_static_f64[944])/v2207)))}else{(if v2197{(self.scalar_static_f64[321]-(self.scalar_static_f64[825]*((v2198*self.scalar_static_f64[940])/v2199)))}else{v4})});
        let v6172=(self.scalar_static_f64[227]*f64::powf(v2213,self.scalar_static_f64[329]));
        let v6257=(if v2236{(-(self.scalar_static_f64[825]*((v2238*self.scalar_static_f64[942])/v2239)))}else{(if v2229{(self.scalar_static_f64[322]-(self.scalar_static_f64[825]*((v2230*self.scalar_static_f64[938])/v2231)))}else{v4})});
        let v6258=(if v2236{(-(self.scalar_static_f64[825]*((v2238*self.scalar_static_f64[948])/v2239)))}else{(if v2229{(self.scalar_static_f64[324]-(self.scalar_static_f64[825]*((v2230*self.scalar_static_f64[947])/v2231)))}else{v4})});
        let v6259=(if v2236{(-(self.scalar_static_f64[825]*((v2238*self.scalar_static_f64[943])/v2239)))}else{(if v2229{(self.scalar_static_f64[323]-(self.scalar_static_f64[825]*((v2230*self.scalar_static_f64[939])/v2231)))}else{v4})});
        let v6260=(if v2236{(-(self.scalar_static_f64[825]*((v2238*self.scalar_static_f64[944])/v2239)))}else{(if v2229{(self.scalar_static_f64[321]-(self.scalar_static_f64[825]*((v2230*self.scalar_static_f64[940])/v2231)))}else{v4})});
        let v6270=(self.scalar_static_f64[227]*f64::powf(v2245,self.scalar_static_f64[329]));
        let v6312=(self.scalar_static_f64[5]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*(self.scalar_static_f64[945]+(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6257/self.scalar_static_f64[502]))*v6270)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[322]-v6257))))))));
        let v6314=(self.scalar_static_f64[5]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*(self.scalar_static_f64[946]+(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6259/self.scalar_static_f64[502]))*v6270)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[323]-v6259))))))));
        let v6338=(if v2272{(-(self.scalar_static_f64[881]*((v2274*self.scalar_static_f64[952])/v2275)))}else{(if v2265{(self.scalar_static_f64[0]-(self.scalar_static_f64[881]*((v2266*self.scalar_static_f64[950])/v2267)))}else{v4})});
        let v6339=(if v2272{(-(self.scalar_static_f64[881]*((v2274*self.scalar_static_f64[953])/v2275)))}else{(if v2265{(self.scalar_static_f64[321]-(self.scalar_static_f64[881]*((v2266*self.scalar_static_f64[951])/v2267)))}else{v4})});
        let v6346=(self.scalar_static_f64[302]*f64::powf(v2283,self.scalar_static_f64[361]));
        let v6377=(self.scalar_static_f64[887]*(if v2302{(v2303*self.scalar_static_f64[954])}else{(if v2299{(v2300*self.scalar_static_f64[954])}else{v3955})}));
        let v6378=(self.scalar_static_f64[887]*(if v2302{v4}else{(if v2299{v4}else{v3956})}));
        let v6379=(self.scalar_static_f64[887]*(if v2302{(v2303*self.scalar_static_f64[955])}else{(if v2299{(v2300*self.scalar_static_f64[955])}else{v3957})}));
        let v6380=(self.scalar_static_f64[887]*(if v2302{v4}else{(if v2299{v4}else{v3958})}));
        let v6381=(self.scalar_static_f64[887]*(if v2302{v4}else{(if v2299{v4}else{v3959})}));
        let v6450=(v32*v2345);
        let v6458=(v2346*v2346);
        let v6472=(if self.scalar_static_bool[64]{(((v2346*(self.scalar_static_f64[894]*v2627))-(v2342*((v436*(if v2335{(v2336*self.scalar_static_f64[956])}else{(if v2331{(v2332*self.scalar_static_f64[956])}else{v4})}))/v6450)))/v6458)}else{(if self.scalar_static_bool[63]{((self.scalar_static_f64[893]*((self.scalar_static_f64[879]*(((v1665*v4297)-(v1662*(v4297/v4305)))/v4313))+(self.scalar_static_f64[891]*(((v1669*v4301)-(v1661*(v4301/v4327)))/v4335))))/self.scalar_static_f64[790])}else{v4})});
        let v6473=(if self.scalar_static_bool[64]{(((v2346*(self.scalar_static_f64[894]*v2628))-(v2342*((v436*(if v2335{(v2336*self.scalar_static_f64[957])}else{(if v2331{(v2332*self.scalar_static_f64[957])}else{v4})}))/v6450)))/v6458)}else{(if self.scalar_static_bool[63]{((self.scalar_static_f64[893]*((self.scalar_static_f64[879]*(((v1665*v4298)-(v1662*(v4298/v4305)))/v4313))+(self.scalar_static_f64[891]*(((v1669*v4302)-(v1661*(v4302/v4327)))/v4335))))/self.scalar_static_f64[790])}else{v4})});
        let v6474=(if self.scalar_static_bool[64]{(((v2346*(self.scalar_static_f64[894]*v2629))-(v2342*((v436*(if v2335{(v2336*self.scalar_static_f64[958])}else{(if v2331{(v2332*self.scalar_static_f64[958])}else{v4})}))/v6450)))/v6458)}else{(if self.scalar_static_bool[63]{((self.scalar_static_f64[893]*((self.scalar_static_f64[879]*(((v1665*v4299)-(v1662*(v4299/v4305)))/v4313))+(self.scalar_static_f64[891]*(((v1669*v4303)-(v1661*(v4303/v4327)))/v4335))))/self.scalar_static_f64[790])}else{v4})});
        let v6475=(if self.scalar_static_bool[64]{(((v2346*(self.scalar_static_f64[894]*v2630))-(v2342*((v436*(if v2335{(v2336*self.scalar_static_f64[959])}else{(if v2331{(v2332*self.scalar_static_f64[959])}else{v4})}))/v6450)))/v6458)}else{(if self.scalar_static_bool[63]{((self.scalar_static_f64[893]*((self.scalar_static_f64[879]*(((v1665*v4300)-(v1662*(v4300/v4305)))/v4313))+(self.scalar_static_f64[891]*(((v1669*v4304)-(v1661*(v4304/v4327)))/v4335))))/self.scalar_static_f64[790])}else{v4})});
        let v6488=(if self.scalar_static_bool[68]{(self.scalar_static_f64[839]*v2652)}else{v4});
        let v6489=(if self.scalar_static_bool[68]{(self.scalar_static_f64[839]*v2653)}else{v4});
        let v6490=(if self.scalar_static_bool[68]{(self.scalar_static_f64[839]*v2654)}else{v4});
        let v6491=(if self.scalar_static_bool[68]{(self.scalar_static_f64[839]*v2655)}else{v4});
        let v6492=(v32*v2359);
        let v6500=(v2360*v2360);
        let v6522=(if self.scalar_static_bool[68]{(v436*(if v862{(v863*self.scalar_static_f64[901])}else{(if v859{(v860*self.scalar_static_f64[901])}else{v4})}))}else{v4});
        let v6523=(if self.scalar_static_bool[68]{(v436*(if v862{(v863*self.scalar_static_f64[903])}else{(if v859{(v860*self.scalar_static_f64[903])}else{v4})}))}else{v4});
        let v6524=(if self.scalar_static_bool[68]{(v436*(if v862{(v863*self.scalar_static_f64[902])}else{(if v859{(v860*self.scalar_static_f64[902])}else{v4})}))}else{v4});
        let v6525=(if self.scalar_static_bool[68]{(v436*(if v862{(v863*self.scalar_static_f64[898])}else{(if v859{(v860*self.scalar_static_f64[898])}else{v4})}))}else{v4});
        let v6526=(v32*v2366);
        let v6534=(v2367*v2367);
        let v6600=(v32*v2396);
        let v6608=(v2397*v2397);
        let v6627=(v1821*(if self.scalar_static_bool[69]{(((v2397*(self.scalar_static_f64[896]*v2652))-(v2393*((v436*(if v2386{(v2387*self.scalar_static_f64[901])}else{(if v2382{(v2383*self.scalar_static_f64[901])}else{v4})}))/v6600)))/v6608)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[895]*((self.scalar_static_f64[879]*(if self.scalar_static_bool[68]{(((v2360*v6488)-(v2357*(v6488/v6492)))/v6500)}else{v4}))+(self.scalar_static_f64[891]*(if self.scalar_static_bool[68]{(((v2367*v6522)-(v2364*(v6522/v6526)))/v6534)}else{v4}))))/self.scalar_static_f64[790])}else{v4})}));
        let v6637=(v1821*(if self.scalar_static_bool[69]{(((v2397*(self.scalar_static_f64[896]*v2654))-(v2393*((v436*(if v2386{(v2387*self.scalar_static_f64[902])}else{(if v2382{(v2383*self.scalar_static_f64[902])}else{v4})}))/v6600)))/v6608)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[895]*((self.scalar_static_f64[879]*(if self.scalar_static_bool[68]{(((v2360*v6490)-(v2357*(v6490/v6492)))/v6500)}else{v4}))+(self.scalar_static_f64[891]*(if self.scalar_static_bool[68]{(((v2367*v6524)-(v2364*(v6524/v6526)))/v6534)}else{v4}))))/self.scalar_static_f64[790])}else{v4})}));
        let v6657=(self.scalar_static_f64[309]*f64::powf(v1149,self.scalar_static_f64[366]));
        let v6667=(v2412*v2412);
        let v6675=(v2418*self.scalar_static_f64[962]);
        let v6676=(v2418*self.scalar_static_f64[963]);
        let v6680=(v2419*v2419);
        let v6706=(v1214*v1214);
        let v6743=(if self.scalar_static_bool[70]{(v6380/self.scalar_static_f64[888])}else{v4});
        let v6782=(self.scalar_static_f64[310]*v6380);
        let v6788=(if self.scalar_static_bool[70]{(v6096+(self.scalar_static_f64[310]*v6377))}else{v4});
        let v6789=(if self.scalar_static_bool[70]{(self.scalar_static_f64[310]*v6378)}else{v4});
        let v6790=(if self.scalar_static_bool[70]{(v6099+(self.scalar_static_f64[310]*v6379))}else{v4});
        let v6791=(if self.scalar_static_bool[70]{(v6100+v6782)}else{v4});
        let v6792=(if self.scalar_static_bool[70]{(v6101+v6782)}else{v4});
        let v6793=(if self.scalar_static_bool[70]{(self.scalar_static_f64[310]*v6381)}else{v4});
        let v6822=(if self.scalar_static_bool[71]{v6096}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6788)}else{v4})});
        let v6823=(if self.scalar_static_bool[71]{v4}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6789)}else{v4})});
        let v6824=(if self.scalar_static_bool[71]{v6099}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6790)}else{v4})});
        let v6825=(if self.scalar_static_bool[71]{v6100}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6791)}else{v4})});
        let v6826=(if self.scalar_static_bool[71]{v6101}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6792)}else{v4})});
        let v6827=(if self.scalar_static_bool[71]{v4}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[313]*v6793)}else{v4})});
        let v6828=(if self.scalar_static_bool[71]{v6105}else{(if self.scalar_static_bool[70]{(v6105+(self.scalar_static_f64[312]*v6788))}else{v4})});
        let v6829=(if self.scalar_static_bool[71]{v4}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[312]*v6789)}else{v4})});
        let v6830=(if self.scalar_static_bool[71]{v6108}else{(if self.scalar_static_bool[70]{(v6108+(self.scalar_static_f64[312]*v6790))}else{v4})});
        let v6831=(if self.scalar_static_bool[71]{v6111}else{(if self.scalar_static_bool[70]{(v6111+(self.scalar_static_f64[312]*v6791))}else{v4})});
        let v6832=(if self.scalar_static_bool[71]{v6114}else{(if self.scalar_static_bool[70]{(v6114+(self.scalar_static_f64[312]*v6792))}else{v4})});
        let v6833=(if self.scalar_static_bool[71]{v4}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[312]*v6793)}else{v4})});
        let v6837=(if self.scalar_static_bool[71]{v6380}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[311]*v6380)}else{v4})});
        let v6855=(v2461*v2461);
        let v6902=(if v2473{((v2474*v3590)+(v1263*(self.scalar_static_f64[786]*v5186)))}else{(if v2469{(((v2461*(v6822+v6828))-(v2470*((v3608-(v2460*v3590))/v3611)))/v6855)}else{v4})});
        let v6903=(if v2473{v4}else{(if v2469{((v6823+v6829)/v2461)}else{v4})});
        let v6904=(if v2473{((v2474*v3593)+(v1263*(self.scalar_static_f64[786]*v5187)))}else{(if v2469{(((v2461*(v6824+v6830))-(v2470*(((v1263*(v3600+v3604))-(v2460*v3593))/v3611)))/v6855)}else{v4})});
        let v6905=(if v2473{((v2474*v3596)+(v1263*(self.scalar_static_f64[786]*v5188)))}else{(if v2469{(((v2461*(v6825+v6831))-(v2470*(((v1263*v3601)-(v2460*v3596))/v3611)))/v6855)}else{v4})});
        let v6906=(if v2473{((v2474*v3599)+(v1263*(self.scalar_static_f64[786]*v5189)))}else{(if v2469{(((v2461*(v6826+v6832))-(v2470*(((v1263*v3602)-(v2460*v3599))/v3611)))/v6855)}else{v4})});
        let v6907=(if v2473{v4}else{(if v2469{((v6827+v6833)/v2461)}else{v4})});
        let v6932=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6902)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6902)}else{v4})})});
        let v6933=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6903)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6903)}else{v4})})});
        let v6934=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6904)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6904)}else{v4})})});
        let v6935=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6905)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6905)}else{v4})})});
        let v6936=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6906)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6906)}else{v4})})});
        let v6937=(if self.scalar_static_bool[79]{v4}else{(if self.scalar_static_bool[77]{(self.scalar_static_f64[316]*v6907)}else{(if self.scalar_static_bool[74]{(self.scalar_static_f64[312]*v6907)}else{v4})})});
        let v7143=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v6377}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[311]*v6377)}else{v4})})+((self.scalar_static_f64[876]*v3322)+v6822)));
        let v7144=(self.scalar_static_f64[0]*(v6823+(if self.scalar_static_bool[71]{v6378}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[311]*v6378)}else{v4})})));
        let v7145=(self.scalar_static_f64[0]*((if self.scalar_static_bool[71]{v6379}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[311]*v6379)}else{v4})})+((self.scalar_static_f64[876]*v3323)+v6824)));
        let v7146=(self.scalar_static_f64[0]*(v6825+v6837));
        let v7147=(self.scalar_static_f64[0]*(v6826+v6837));
        let v7148=(self.scalar_static_f64[0]*(v6827+(if self.scalar_static_bool[71]{v6381}else{(if self.scalar_static_bool[70]{(self.scalar_static_f64[311]*v6381)}else{v4})})));
        let v7162=(self.scalar_static_f64[0]*(self.scalar_static_f64[877]*((self.scalar_static_f64[830]*(-((-(self.scalar_static_f64[541]*v6067))*v6074)))+(v155*(self.scalar_static_f64[321]-v6067)))));
        let v7163=(self.scalar_static_f64[0]*(self.scalar_static_f64[877]*((self.scalar_static_f64[830]*(-((-(self.scalar_static_f64[541]*v6068))*v6074)))+(v155*(self.scalar_static_f64[0]-v6068)))));
        let v7168=(self.scalar_static_f64[0]*v6828);
        let v7169=(self.scalar_static_f64[0]*v6829);
        let v7170=(self.scalar_static_f64[0]*(((v2314*(self.scalar_static_f64[892]*v3260))+(v2313*v3220))+((self.scalar_static_f64[878]*v3448)+v6830)));
        let v7171=(self.scalar_static_f64[0]*(((v2314*(self.scalar_static_f64[892]*v3261))+(v2313*v3221))+((self.scalar_static_f64[878]*v3449)+v6831)));
        let v7172=(self.scalar_static_f64[0]*(((v2314*(self.scalar_static_f64[892]*v3262))+(v2313*v3216))+((self.scalar_static_f64[878]*v3445)+v6832)));
        let v7173=(self.scalar_static_f64[0]*v6833);
        let v7186=(self.scalar_static_f64[0]*(self.scalar_static_f64[550]*((self.scalar_static_f64[883]*(-((-(v6338/self.scalar_static_f64[540]))*v6346)))+(v32*(self.scalar_static_f64[0]-v6338)))));
        let v7187=(self.scalar_static_f64[0]*(self.scalar_static_f64[550]*((self.scalar_static_f64[883]*(-((-(v6339/self.scalar_static_f64[540]))*v6346)))+(v32*(self.scalar_static_f64[321]-v6339)))));
        let v7192=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{(v2437*((if self.scalar_static_bool[70]{(v6377/self.scalar_static_f64[888])}else{v4})+((if self.scalar_static_bool[70]{(self.scalar_static_f64[876]*(if self.scalar_static_bool[70]{((v2421*(if self.scalar_static_bool[70]{(v3307*v6657)}else{v4}))+(v2407*(if v2416{(((v2419*v6675)-(v2418*v6675))/v6680)}else{(if v2410{((-(v2411*self.scalar_static_f64[960]))/v6667)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[70]{((v2432*(if self.scalar_static_bool[70]{((v2429*((self.scalar_static_f64[382]*v3450)/self.scalar_static_f64[601]))+(v2428*((-(v424*v3453))/v6706)))}else{v4}))+(v2431*(self.scalar_static_f64[880]*v5186)))}else{v4}))))}else{v4}));
        let v7193=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{((v2439*self.scalar_static_f64[367])+(v2437*(if self.scalar_static_bool[70]{(v6378/self.scalar_static_f64[888])}else{v4})))}else{v4}));
        let v7194=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{((v2439*self.scalar_static_f64[368])+(v2437*((if self.scalar_static_bool[70]{(v6379/self.scalar_static_f64[888])}else{v4})+((if self.scalar_static_bool[70]{(self.scalar_static_f64[876]*(if self.scalar_static_bool[70]{((v2421*(if self.scalar_static_bool[70]{(v3308*v6657)}else{v4}))+(v2407*(if v2416{(((v2419*v6676)-(v2418*v6676))/v6680)}else{(if v2410{((-(v2411*self.scalar_static_f64[961]))/v6667)}else{v4})})))}else{v4}))}else{v4})+(if self.scalar_static_bool[70]{((v2432*(if self.scalar_static_bool[70]{((v2429*((self.scalar_static_f64[382]*v3451)/self.scalar_static_f64[601]))+(v2428*((-(v424*v3454))/v6706)))}else{v4}))+(v2431*(self.scalar_static_f64[880]*v5187)))}else{v4})))))}else{v4}));
        let v7195=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{(v2437*((if self.scalar_static_bool[70]{(v2431*(self.scalar_static_f64[880]*v5188))}else{v4})+v6743))}else{v4}));
        let v7196=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{(v2437*((if self.scalar_static_bool[70]{(v2431*(self.scalar_static_f64[880]*v5189))}else{v4})+v6743))}else{v4}));
        let v7197=(self.scalar_static_f64[0]*(if self.scalar_static_bool[70]{(v2437*(if self.scalar_static_bool[70]{(v6381/self.scalar_static_f64[888])}else{v4}))}else{v4}));
        let v7252=(self.scalar_static_f64[0]*(v6312+(if self.scalar_static_bool[67]{((v2399*v4838)+v6627)}else{v4})));
        let v7253=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6258/self.scalar_static_f64[502]))*v6270)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[324]-v6258))))+self.scalar_static_f64[949]))))+(if self.scalar_static_bool[67]{((v2399*v4839)+(v1821*(if self.scalar_static_bool[69]{(((v2397*(self.scalar_static_f64[896]*v2653))-(v2393*((v436*(if v2386{(v2387*self.scalar_static_f64[903])}else{(if v2382{(v2383*self.scalar_static_f64[903])}else{v4})}))/v6600)))/v6608)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[895]*((self.scalar_static_f64[879]*(if self.scalar_static_bool[68]{(((v2360*v6489)-(v2357*(v6489/v6492)))/v6500)}else{v4}))+(self.scalar_static_f64[891]*(if self.scalar_static_bool[68]{(((v2367*v6523)-(v2364*(v6523/v6526)))/v6534)}else{v4}))))/self.scalar_static_f64[790])}else{v4})})))}else{v4})));
        let v7254=(self.scalar_static_f64[0]*(if self.scalar_static_bool[67]{(v2399*v4840)}else{v4}));
        let v7255=(self.scalar_static_f64[0]*(if self.scalar_static_bool[67]{(v2399*v4841)}else{v4}));
        let v7256=(self.scalar_static_f64[0]*(v6312+(if self.scalar_static_bool[67]{(v6627+(v2399*v4842))}else{v4})));
        let v7257=(self.scalar_static_f64[0]*(v6314+(if self.scalar_static_bool[67]{((v2399*v4843)+v6637)}else{v4})));
        let v7258=(self.scalar_static_f64[0]*(v6314+(if self.scalar_static_bool[67]{(v6637+(v2399*v4844))}else{v4})));
        let v7259=(self.scalar_static_f64[0]*((self.scalar_static_f64[5]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*(self.scalar_static_f64[909]+(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6260/self.scalar_static_f64[502]))*v6270)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[321]-v6260))))))))+(if self.scalar_static_bool[67]{((v2399*v4845)+(v1821*(if self.scalar_static_bool[69]{(((v2397*(self.scalar_static_f64[896]*v2655))-(v2393*((v436*(if v2386{(v2387*self.scalar_static_f64[898])}else{(if v2382{(v2383*self.scalar_static_f64[898])}else{v4})}))/v6600)))/v6608)}else{(if self.scalar_static_bool[68]{((self.scalar_static_f64[895]*((self.scalar_static_f64[879]*(if self.scalar_static_bool[68]{(((v2360*v6491)-(v2357*(v6491/v6492)))/v6500)}else{v4}))+(self.scalar_static_f64[891]*(if self.scalar_static_bool[68]{(((v2367*v6525)-(v2364*(v6525/v6526)))/v6534)}else{v4}))))/self.scalar_static_f64[790])}else{v4})})))}else{v4})));
        let v7260=(self.scalar_static_f64[0]*(v6314+(if self.scalar_static_bool[67]{(v6637+(v2399*v4846))}else{v4})));
        let v7302=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*(self.scalar_static_f64[908]+(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6159/self.scalar_static_f64[502]))*v6172)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[0]-v6159))))))))+(if self.scalar_static_bool[67]{(self.scalar_static_f64[6]*v6472)}else{v6472})));
        let v7303=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6160/self.scalar_static_f64[502]))*v6172)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[322]-v6160))))+self.scalar_static_f64[945]))))+(if self.scalar_static_bool[67]{(self.scalar_static_f64[6]*v6473)}else{v6473})));
        let v7304=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*((self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6161/self.scalar_static_f64[502]))*v6172)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[323]-v6161))))+self.scalar_static_f64[946]))))+(if self.scalar_static_bool[67]{(self.scalar_static_f64[6]*v6474)}else{v6474})));
        let v7305=(self.scalar_static_f64[0]*((self.scalar_static_f64[6]*(self.scalar_static_f64[298]*(self.scalar_static_f64[556]*(self.scalar_static_f64[909]+(self.scalar_static_f64[832]*((self.scalar_static_f64[837]*(-((-(v6162/self.scalar_static_f64[502]))*v6172)))+(self.scalar_static_f64[833]*(self.scalar_static_f64[321]-v6162))))))))+(if self.scalar_static_bool[67]{(self.scalar_static_f64[6]*v6475)}else{v6475})));

        CommonStampValues {
            v1,
            v4,
            v31,
            v32,
            v47,
            v155,
            v421,
            v424,
            v436,
            v462,
            v730,
            v734,
            v736,
            v741,
            v744,
            v747,
            v752,
            v760,
            v763,
            v766,
            v770,
            v785,
            v806,
            v807,
            v808,
            v811,
            v812,
            v827,
            v828,
            v831,
            v832,
            v847,
            v848,
            v851,
            v852,
            v920,
            v1038,
            v1095,
            v1119,
            v1122,
            v1125,
            v1151,
            v1227,
            v1262,
            v1263,
            v1268,
            v1269,
            v1287,
            v1288,
            v1291,
            v1292,
            v1301,
            v1331,
            v1333,
            v1334,
            v1339,
            v1340,
            v1347,
            v1348,
            v1349,
            v1354,
            v1356,
            v1406,
            v1408,
            v1409,
            v1414,
            v1415,
            v1441,
            v1453,
            v1465,
            v1477,
            v1483,
            v1484,
            v1487,
            v1488,
            v1493,
            v1494,
            v1500,
            v1504,
            v1507,
            v1515,
            v1516,
            v1517,
            v1519,
            v1521,
            v1525,
            v1526,
            v1528,
            v1530,
            v1531,
            v1532,
            v1537,
            v1538,
            v1575,
            v1577,
            v1579,
            v1580,
            v1583,
            v1584,
            v1589,
            v1590,
            v1595,
            v1598,
            v1600,
            v1608,
            v1609,
            v1610,
            v1612,
            v1617,
            v1618,
            v1620,
            v1621,
            v1622,
            v1623,
            v1628,
            v1629,
            v1758,
            v1782,
            v1799,
            v1821,
            v1891,
            v1901,
            v1911,
            v1912,
            v1913,
            v1916,
            v1917,
            v1921,
            v1922,
            v1924,
            v1928,
            v1929,
            v1934,
            v1935,
            v1948,
            v2052,
            v2053,
            v2055,
            v2057,
            v2059,
            v2061,
            v2062,
            v2064,
            v2072,
            v2074,
            v2075,
            v2076,
            v2082,
            v2084,
            v2085,
            v2089,
            v2091,
            v2094,
            v2095,
            v2100,
            v2101,
            v2461,
            v2489,
            v2540,
            v2543,
            v2546,
            v2549,
            v2552,
            v2556,
            v2560,
            v2568,
            v2574,
            v2585,
            v2601,
            v2602,
            v2627,
            v2628,
            v2629,
            v2630,
            v2780,
            v2781,
            v2782,
            v3069,
            v3070,
            v3071,
            v3217,
            v3218,
            v3219,
            v3260,
            v3261,
            v3262,
            v3269,
            v3270,
            v3271,
            v3278,
            v3279,
            v3280,
            v3312,
            v3313,
            v3492,
            v3493,
            v3494,
            v3584,
            v3585,
            v3586,
            v3587,
            v3590,
            v3593,
            v3596,
            v3599,
            v3600,
            v3601,
            v3602,
            v3604,
            v3608,
            v3611,
            v3645,
            v3646,
            v3705,
            v3706,
            v3839,
            v3840,
            v3841,
            v3896,
            v3897,
            v3898,
            v3911,
            v3912,
            v3913,
            v3934,
            v3935,
            v3936,
            v3937,
            v3938,
            v3955,
            v3956,
            v3957,
            v3958,
            v3959,
            v4591,
            v4592,
            v4593,
            v4594,
            v4677,
            v4678,
            v4679,
            v4680,
            v4681,
            v4682,
            v4695,
            v4696,
            v4697,
            v4698,
            v4699,
            v4700,
            v4701,
            v4702,
            v4838,
            v4839,
            v4840,
            v4841,
            v4842,
            v4843,
            v4844,
            v4845,
            v4846,
            v5186,
            v5187,
            v5188,
            v5189,
            v6932,
            v6933,
            v6934,
            v6935,
            v6936,
            v6937,
            v7143,
            v7144,
            v7145,
            v7146,
            v7147,
            v7148,
            v7162,
            v7163,
            v7168,
            v7169,
            v7170,
            v7171,
            v7172,
            v7173,
            v7186,
            v7187,
            v7192,
            v7193,
            v7194,
            v7195,
            v7196,
            v7197,
            v7252,
            v7253,
            v7254,
            v7255,
            v7256,
            v7257,
            v7258,
            v7259,
            v7260,
            v7302,
            v7303,
            v7304,
            v7305,
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
        let v809=(common.v807).exp();
        let v829=(common.v827).exp();
        let v836=(if common.v831{(common.v832*(common.v1+(common.v827-self.scalar_static_f64[204])))}else{(if common.v828{v829}else{common.v4})});
        let v849=(common.v847).exp();
        let v856=(if common.v851{(common.v852*(common.v1+(common.v847-self.scalar_static_f64[204])))}else{(if common.v848{v849}else{common.v4})});
        let v1289=(common.v1287).exp();
        let v1296=(if common.v1291{(common.v1292*(common.v1+(common.v1287-self.scalar_static_f64[204])))}else{(if common.v1288{v1289}else{common.v4})});
        let v1302=(common.v736<self.scalar_static_f64[230]);
        let v1303=(common.v1301).exp();
        let v1304=(common.v1+v1303);
        let v1309=(!v1302);
        let v1311=((-common.v1301)).exp();
        let v1312=(common.v1+v1311);
        let v1316=(if v1309{(self.scalar_static_f64[230]-(common.v31*(v1312).ln()))}else{(if v1302{(common.v736-(common.v31*(v1304).ln()))}else{common.v4})});
        let v1318=(v1316*self.scalar_static_f64[231]);
        let v1319=(self.scalar_static_f64[230]-v1316);
        let v1320=f64::powf(v1319,common.v32);
        let v1335=(self.scalar_static_bool[12]&&common.v1334);
        let v1336=(common.v1333).exp();
        let v1344=(if common.v1339{(common.v1340*(common.v1+(common.v1333-self.scalar_static_f64[204])))}else{(if v1335{v1336}else{common.v1287})});
        let v1350=(self.scalar_static_bool[12]&&common.v1349);
        let v1351=(common.v1347).exp();
        let v1360=(if common.v1354{(common.v1356*(common.v1+(common.v1347-common.v1348)))}else{(if v1350{v1351}else{v1296})});
        let v1361=(common.v1331-common.v1);
        let v1362=(self.scalar_static_f64[672]*v1361);
        let v1364=(v1361*self.scalar_static_f64[845]);
        let v1367=((common.v1+(common.v436*v1344))).sqrt();
        let v1368=(common.v1+v1367);
        let v1369=(v1364/v1368);
        let v1370=(common.v1+common.v1227);
        let v1374=(self.scalar_static_f64[687]*(common.v1095-common.v1));
        let v1375=(v1360*v1374);
        let v1376=(common.v1+v1360);
        let v1391=(self.scalar_static_f64[232]*((common.v1095+common.v1331)-common.v32));
        let v1410=(self.scalar_static_bool[12]&&common.v1409);
        let v1411=(common.v1408).exp();
        let v1420=(common.v1406-common.v1);
        let v1421=(self.scalar_static_f64[678]*v1420);
        let v1423=(v1420*self.scalar_static_f64[846]);
        let v1426=((common.v1+(common.v436*(if common.v1414{(common.v1415*(common.v1+(common.v1408-self.scalar_static_f64[204])))}else{(if v1410{v1411}else{v1344})})))).sqrt();
        let v1427=(common.v1+v1426);
        let v1467=(self.scalar_static_f64[664]*(common.v1465-common.v1));
        let v1489=(common.v1483&&common.v1488);
        let v1490=(common.v1487).exp();
        let v1498=(if common.v1493{(common.v1494*(common.v1+(common.v1487-self.scalar_static_f64[204])))}else{(if v1489{v1490}else{common.v4})});
        let v1533=(common.v1531&&common.v1532);
        let v1534=(common.v1528).exp();
        let v1543=(-common.v736);
        let v1544=(common.v1-(if common.v1537{(common.v1538*(common.v1+(common.v1528-self.scalar_static_f64[204])))}else{(if v1533{v1534}else{common.v4})}));
        let v1546=(common.v1+(v1544/common.v1528));
        let v1550=(common.v1483&&(!common.v1530));
        let v1551=(common.v424*common.v736);
        let v1552=(common.v1528*v1551);
        let v1553=0.3333333333333333;
        let v1554=(common.v1528*v1553);
        let v1555=0.25;
        let v1557=(common.v1+(common.v1528*v1555));
        let v1559=(common.v1+(v1554*v1557));
        let v1563=((if v1550{(v1552*v1559)}else{(if common.v1532{(v1543*v1546)}else{common.v4})})*self.scalar_static_f64[847]);
        let v1564=(common.v1151*v1563);
        let v1569=(!common.v1483);
        let v1585=(common.v1575&&common.v1584);
        let v1586=(common.v1583).exp();
        let v1594=(if common.v1589{(common.v1590*(common.v1+(common.v1583-self.scalar_static_f64[204])))}else{(if v1585{v1586}else{common.v4})});
        let v1624=(common.v1622&&common.v1623);
        let v1625=(common.v1620).exp();
        let v1634=(-common.v730);
        let v1635=(common.v1-(if common.v1628{(common.v1629*(common.v1+(common.v1620-self.scalar_static_f64[204])))}else{(if v1624{v1625}else{common.v4})}));
        let v1637=(common.v1+(v1635/common.v1620));
        let v1641=(common.v1575&&(!common.v1621));
        let v1642=(common.v424*common.v730);
        let v1643=(common.v1620*v1642);
        let v1644=(v1553*common.v1620);
        let v1646=(common.v1+(v1555*common.v1620));
        let v1648=(common.v1+(v1644*v1646));
        let v1652=((if v1641{(v1643*v1648)}else{(if common.v1623{(v1634*v1637)}else{common.v4})})*self.scalar_static_f64[848]);
        let v1653=(common.v1579*v1652);
        let v1658=(!common.v1575);
        let v1659=(if v1658{common.v4}else{(if common.v1575{(self.scalar_static_f64[51]*(self.scalar_static_f64[542]*(v1594*v1653)))}else{common.v4})});
        let v1672=(common.v806-common.v1);
        let v1673=(self.scalar_static_f64[849]*v1672);
        let v1678=((common.v1+(common.v806*self.scalar_static_f64[851]))).sqrt();
        let v1679=(common.v1+v1678);
        let v1680=(v1673/v1679);
        let v1687=(self.scalar_static_f64[852]*(common.v785-v836));
        let v1695=((common.v1+(self.scalar_static_f64[854]*(common.v785+(v836*self.scalar_static_f64[245]))))).sqrt();
        let v1696=(common.v1+v1695);
        let v1703=(self.scalar_static_f64[855]*(common.v806-v856));
        let v1708=((common.v1+(self.scalar_static_f64[854]*(common.v806+(v856*self.scalar_static_f64[245]))))).sqrt();
        let v1709=(common.v1+v1708);
        let v1714=(self.scalar_static_f64[852]*(common.v785-common.v1));
        let v1717=((common.v1+(common.v785*self.scalar_static_f64[854]))).sqrt();
        let v1718=(common.v1+v1717);
        let v1721=(v1672*self.scalar_static_f64[855]);
        let v1724=((common.v1+(common.v806*self.scalar_static_f64[854]))).sqrt();
        let v1725=(common.v1+v1724);
        let v1727=(if self.scalar_static_bool[41]{(v1721/v1725)}else{(if self.scalar_static_bool[40]{(v1703/v1709)}else{common.v4})});
        let v1730=(self.scalar_static_f64[856]*(v836-common.v1));
        let v1736=((common.v1+(v836*self.scalar_static_f64[858]))).sqrt();
        let v1737=(common.v1+v1736);
        let v1746=(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v1680)}else{v1680});
        let v1823=(if self.scalar_static_bool[44]{(common.v1758*common.v1821)}else{common.v4});
        let v1829=(if self.scalar_static_bool[51]{(common.v730+common.v741)}else{common.v4});
        let v1831=(-v1829);
        let v1834=(v1831<common.v4);
        let v1835=(self.scalar_static_bool[51]&&v1834);
        let v1838=((self.scalar_static_f64[256]+(if self.scalar_static_bool[51]{(v1829*v1829)}else{common.v1799}))).sqrt();
        let v1839=(v1838-v1831);
        let v1843=(self.scalar_static_bool[51]&&(!v1834));
        let v1846=(if v1843{(common.v424*(v1831+v1838))}else{(if v1835{(self.scalar_static_f64[257]/v1839)}else{common.v4})});
        let v1862=(v1846<self.scalar_static_f64[265]);
        let v1863=(self.scalar_static_bool[51]&&v1862);
        let v1864=(v1846/self.scalar_static_f64[263]);
        let v1866=(common.v1-f64::powf(v1864,self.scalar_static_f64[258]));
        let v1870=(self.scalar_static_bool[51]&&(!v1862));
        let v1876=(if self.scalar_static_bool[52]{common.v1}else{(if v1870{(self.scalar_static_f64[262]+(self.scalar_static_f64[272]*(v1846-self.scalar_static_f64[265])))}else{(if v1863{(common.v1/v1866)}else{common.v4})})});
        let v1892=(common.v1262*common.v1891);
        let v1893=(self.scalar_static_f64[564]/v1892);
        let v1894=(v1893<self.scalar_static_f64[14]);
        let v1896=(common.v155*(if v1894{self.scalar_static_f64[14]}else{v1893}));
        let v1899=(common.v741+(self.scalar_static_f64[818]*((if common.v811{(common.v812*(common.v1+(common.v807-self.scalar_static_f64[204])))}else{(if common.v808{v809}else{common.v4})})-common.v1)));
        let v1930=(common.v1911&&common.v1929);
        let v1931=(common.v1928).exp();
        let v1939=(if common.v1934{(common.v1935*(common.v1+(common.v1928-self.scalar_static_f64[204])))}else{(if v1930{v1931}else{common.v4})});
        let v1942=(common.v1924*self.scalar_static_f64[873]);
        let v1950=((common.v730<self.scalar_static_f64[466])&&(self.scalar_static_bool[54]&&common.v1948));
        let v1956=(if v1950{self.scalar_static_f64[282]}else{common.v4});
        let v1957=(self.scalar_static_f64[466]-common.v730);
        let v1959=(if v1950{(v1957/common.v1125)}else{common.v1038});
        let v1962=(((common.v32*v1959)/v1956)).sqrt();
        let v1963=(if v1950{v1962}else{common.v4});
        let v1966=(v1950&&self.scalar_static_bool[56]);
        let v1969=(v1950&&self.scalar_static_bool[57]);
        let v1972=(if v1969{(common.v1-(common.v424*common.v1119))}else{common.v4});
        let v1973=(self.scalar_static_f64[280]*v1972);
        let v1975=(if v1969{(v1972*v1973)}else{(if v1966{self.scalar_static_f64[280]}else{common.v4})});
        let v1976=(v1963*v1975);
        let v1980=(((v1963*v1963)+(v1975*v1975))).sqrt();
        let v1982=(if v1950{(v1976/v1980)}else{common.v4});
        let v1984=(if v1950{(v1957/v1982)}else{common.v4});
        let v1985=(common.v424*v1982);
        let v1986=(v1956*v1985);
        let v1989=(if v1950{(v1984+(common.v1125*v1986))}else{common.v4});
        let v2002=(self.scalar_static_f64[207]*(if v1969{(common.v1+(self.scalar_static_f64[285]*(common.v1+(common.v32*common.v1119))))}else{common.v4}));
        let v2004=((if v1969{self.scalar_static_f64[288]}else{common.v4})-(common.v1269/v2002));
        let v2007=(if v1969{(v1984-(v1986*v2004))}else{common.v4});
        let v2008=(v2007-v1989);
        let v2010=(common.v47*v1984);
        let v2011=(v1984*v2010);
        let v2017=((if v1969{((v2008*v2008)+((common.v1122*v2011)/self.scalar_static_f64[207]))}else{v1959})).sqrt();
        let v2020=(if v1969{(common.v424*((v1989+v2007)+v2017))}else{(if v1966{v1989}else{common.v4})});
        let v2021=(v2020-v1984);
        let v2023=(if v1950{(v2021/v2020)}else{common.v4});
        let v2026=((v2023).abs()>1e-7);
        let v2027=(v1950&&v2026);
        let v2029=(if v2027{(v1985/v2023)}else{common.v4});
        let v2031=(v2020*self.scalar_static_f64[874]);
        let v2032=(v2029*v2031);
        let v2034=(self.scalar_static_f64[875]/v2020);
        let v2035=(v2034).exp();
        let v2037=(common.v1+(v1975/v2029));
        let v2039=((v2034*v2037)).exp();
        let v2040=(v2035-v2039);
        let v2044=(v1950&&(!v2026));
        let v2045=(self.scalar_static_f64[3]*v1975);
        let v2096=(common.v2052&&common.v2095);
        let v2097=(common.v2094).exp();
        let v2105=(if common.v2100{(common.v2101*(common.v1+(common.v2094-self.scalar_static_f64[204])))}else{(if v2096{v2097}else{v1939})});
        let v2106=(common.v1922*self.scalar_static_f64[873]);
        let v2108=(if common.v2052{(v2105*v2106)}else{(if v2044{(v2035*v2045)}else{(if v2027{(v2032*v2040)}else{(if common.v1911{(v1939*v1942)}else{common.v4})})})});
        let v2112=(common.v1901&&(v2108>common.v4));
        let v2113=(self.scalar_static_bool[60]&&v2112);
        let v2114=(self.scalar_static_f64[568]+v1896);
        let v2115=(common.v1269*v2114);
        let v2122=(if v2113{(((self.scalar_static_f64[381]/v2115)+(self.scalar_static_f64[672]*(common.v1263/self.scalar_static_f64[644])))+(self.scalar_static_f64[561]/v2114))}else{common.v4});
        let v2123=(self.scalar_static_bool[58]&&v2113);
        let v2126=(if v2123{((v2108-v2122)/common.v421)}else{common.v2072});
        let v2127=(v2108<v2122);
        let v2128=(v2123&&v2127);
        let v2129=(v2126).exp();
        let v2130=(common.v1+v2129);
        let v2136=(v2123&&(!v2127));
        let v2138=((-v2126)).exp();
        let v2139=(common.v1+v2138);
        let v2143=(if v2136{(v2122-(common.v421*(v2139).ln()))}else{(if v2128{(v2108-(common.v421*(v2130).ln()))}else{v2108})});
        let v2144=(common.v1269*v2143);
        let v2147=(v2113&&self.scalar_static_bool[61]);
        let v2148=(v2122*v2144);
        let v2149=(v2122+v2143);
        let v2153=(v2112&&self.scalar_static_bool[62]);
        let v2154=(if v2153{v2144}else{(if v2147{(v2148/v2149)}else{(if v2123{v2144}else{common.v4})})});
        let v2468=(if self.scalar_static_bool[73]{common.v4}else{(if self.scalar_static_bool[72]{((v2154/common.v2461)).abs()}else{common.v4})});
        let v2519=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v1876))));
        let v2541=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2540);
        let v2544=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2543);
        let v2547=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2546);
        let v2550=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2549);
        let v2553=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2552);
        let v2557=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2556);
        let v2561=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2560);
        let v2569=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2568);
        let v2575=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2574);
        let v2586=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2585);
        let v2662=(if common.v831{(common.v832*self.scalar_static_f64[897])}else{(if common.v828{(v829*self.scalar_static_f64[897])}else{common.v4})});
        let v2663=(if common.v831{(common.v832*self.scalar_static_f64[898])}else{(if common.v828{(v829*self.scalar_static_f64[898])}else{common.v4})});
        let v2685=(if common.v851{(common.v852*self.scalar_static_f64[897])}else{(if common.v848{(v849*self.scalar_static_f64[897])}else{common.v4})});
        let v2686=(if common.v851{(common.v852*self.scalar_static_f64[902])}else{(if common.v848{(v849*self.scalar_static_f64[902])}else{common.v4})});
        let v2687=(if common.v851{(common.v852*self.scalar_static_f64[898])}else{(if common.v848{(v849*self.scalar_static_f64[898])}else{common.v4})});
        let v3612=((common.v3608-(common.v1268*common.v3590))/common.v3611);
        let v3616=(((common.v1263*(common.v3604-common.v3600))-(common.v1268*common.v3593))/common.v3611);
        let v3620=(((common.v1263*(-common.v3601))-(common.v1268*common.v3596))/common.v3611);
        let v3624=(((common.v1263*(-common.v3602))-(common.v1268*common.v3599))/common.v3611);
        let v3647=(common.v3645/self.scalar_static_f64[229]);
        let v3648=(common.v3646/self.scalar_static_f64[229]);
        let v3655=(if common.v1291{(common.v1292*v3647)}else{(if common.v1288{(v1289*v3647)}else{common.v4})});
        let v3656=(if common.v1291{(common.v1292*v3648)}else{(if common.v1288{(v1289*v3648)}else{common.v4})});
        let v3681=(if v1309{(-(common.v31*((v1311*self.scalar_static_f64[336])/v1312)))}else{(if v1302{(self.scalar_static_f64[321]-(common.v31*((v1303*self.scalar_static_f64[334])/v1304)))}else{common.v4})});
        let v3682=(if v1309{(-(common.v31*((v1311*self.scalar_static_f64[337])/v1312)))}else{(if v1302{(self.scalar_static_f64[0]-(common.v31*((v1303*self.scalar_static_f64[335])/v1304)))}else{common.v4})});
        let v3688=(common.v32*f64::powf(v1319,common.v1));
        let v3713=(if common.v1339{(common.v1340*self.scalar_static_f64[898])}else{(if v1335{(v1336*self.scalar_static_f64[898])}else{v3647})});
        let v3714=(if common.v1339{(common.v1340*self.scalar_static_f64[897])}else{(if v1335{(v1336*self.scalar_static_f64[897])}else{v3648})});
        let v3715=(v3612/self.scalar_static_f64[644]);
        let v3716=(v3616/self.scalar_static_f64[644]);
        let v3717=(v3620/self.scalar_static_f64[644]);
        let v3718=(v3624/self.scalar_static_f64[644]);
        let v3731=(if common.v1354{(common.v1356*v3715)}else{(if v1350{(v1351*v3715)}else{v3655})});
        let v3732=(if common.v1354{(common.v1356*v3716)}else{(if v1350{(v1351*v3716)}else{v3656})});
        let v3733=(if common.v1354{(common.v1356*v3717)}else{(if v1350{(v1351*v3717)}else{common.v4})});
        let v3734=(if common.v1354{(common.v1356*v3718)}else{(if v1350{(v1351*v3718)}else{common.v4})});
        let v3735=(self.scalar_static_f64[672]*common.v3705);
        let v3736=(self.scalar_static_f64[672]*common.v3706);
        let v3741=(common.v32*v1367);
        let v3747=(v1368*v1368);
        let v3777=(v1376*v1376);
        let v3852=(self.scalar_static_f64[678]*common.v3839);
        let v3853=(self.scalar_static_f64[678]*common.v3840);
        let v3854=(self.scalar_static_f64[678]*common.v3841);
        let v3861=(common.v32*v1426);
        let v3868=(v1427*v1427);
        let v3969=(common.v1484*common.v1484);
        let v3976=(self.scalar_static_f64[726]*(-((-(self.scalar_static_f64[19]*(common.v32*common.v3312)))/v3969)));
        let v3977=(self.scalar_static_f64[726]*(-((-(self.scalar_static_f64[19]*(common.v32*common.v3313)))/v3969)));
        let v3988=(if common.v1483{self.scalar_static_f64[925]}else{common.v4});
        let v3989=(if common.v1483{self.scalar_static_f64[926]}else{common.v4});
        let v3990=(common.v1500*v3988);
        let v3992=(common.v1500*v3989);
        let v3994=(common.v32*common.v1504);
        let v3999=(self.scalar_static_f64[234]*f64::powf(common.v1504,self.scalar_static_f64[338]));
        let v4045=(common.v1526*common.v1526);
        let v4051=(if common.v1483{(((common.v1526*self.scalar_static_f64[927])-(common.v1525*(self.scalar_static_f64[405]*(if common.v1483{(common.v1521*((common.v1519*(((v3990+v3990)/v3994)*v3999))+(common.v1507*((self.scalar_static_f64[17]*(-(self.scalar_static_f64[237]*(common.v155*v3988))))-((common.v1517*((common.v1515*v3988)+(common.v1500*(common.v462*v3988))))+(common.v1516*v3988))))))}else{common.v4}))))/v4045)}else{v3988});
        let v4052=(if common.v1483{(((common.v1526*self.scalar_static_f64[928])-(common.v1525*(self.scalar_static_f64[405]*(if common.v1483{(common.v1521*((common.v1519*(((v3992+v3992)/v3994)*v3999))+(common.v1507*((self.scalar_static_f64[17]*(-(self.scalar_static_f64[237]*(common.v155*v3989))))-((common.v1517*((common.v1515*v3989)+(common.v1500*(common.v462*v3989))))+(common.v1516*v3989))))))}else{common.v4}))))/v4045)}else{v3989});
        let v4066=(common.v1528*common.v1528);
        let v4133=(self.scalar_static_f64[227]*f64::powf(common.v1577,self.scalar_static_f64[329]));
        let v4136=(if common.v1575{(self.scalar_static_f64[931]*v4133)}else{common.v4});
        let v4137=(if common.v1575{(self.scalar_static_f64[932]*v4133)}else{common.v4});
        let v4142=(common.v1580*common.v1580);
        let v4149=(self.scalar_static_f64[746]*(-((-(self.scalar_static_f64[50]*(common.v32*v4136)))/v4142)));
        let v4150=(self.scalar_static_f64[746]*(-((-(self.scalar_static_f64[50]*(common.v32*v4137)))/v4142)));
        let v4159=(if common.v1575{self.scalar_static_f64[929]}else{common.v4});
        let v4160=(if common.v1575{self.scalar_static_f64[930]}else{common.v4});
        let v4161=(common.v1595*v4159);
        let v4163=(common.v1595*v4160);
        let v4165=(common.v32*common.v1598);
        let v4170=(self.scalar_static_f64[238]*f64::powf(common.v1598,self.scalar_static_f64[343]));
        let v4216=(common.v1618*common.v1618);
        let v4222=(if common.v1575{(((common.v1618*self.scalar_static_f64[933])-(common.v1617*(self.scalar_static_f64[425]*(if common.v1575{(common.v1521*((common.v1612*(((v4161+v4161)/v4165)*v4170))+(common.v1600*((self.scalar_static_f64[48]*(-(self.scalar_static_f64[241]*(common.v155*v4159))))-((common.v1610*((common.v1608*v4159)+(common.v1595*(common.v462*v4159))))+(common.v1609*v4159))))))}else{common.v4}))))/v4216)}else{v4159});
        let v4223=(if common.v1575{(((common.v1618*self.scalar_static_f64[934])-(common.v1617*(self.scalar_static_f64[425]*(if common.v1575{(common.v1521*((common.v1612*(((v4163+v4163)/v4165)*v4170))+(common.v1600*((self.scalar_static_f64[48]*(-(self.scalar_static_f64[241]*(common.v155*v4160))))-((common.v1610*((common.v1608*v4160)+(common.v1595*(common.v462*v4160))))+(common.v1609*v4160))))))}else{common.v4}))))/v4216)}else{v4160});
        let v4237=(common.v1620*common.v1620);
        let v4357=(common.v32*v1678);
        let v4365=(v1679*v1679);
        let v4366=(((v1679*(self.scalar_static_f64[849]*common.v2627))-(v1673*((self.scalar_static_f64[851]*common.v2627)/v4357)))/v4365);
        let v4370=(((v1679*(self.scalar_static_f64[849]*common.v2628))-(v1673*((self.scalar_static_f64[851]*common.v2628)/v4357)))/v4365);
        let v4374=(((v1679*(self.scalar_static_f64[849]*common.v2629))-(v1673*((self.scalar_static_f64[851]*common.v2629)/v4357)))/v4365);
        let v4378=(((v1679*(self.scalar_static_f64[849]*common.v2630))-(v1673*((self.scalar_static_f64[851]*common.v2630)/v4357)))/v4365);
        let v4382=(self.scalar_static_f64[852]*common.v2601);
        let v4384=(self.scalar_static_f64[852]*common.v2602);
        let v4388=(self.scalar_static_f64[854]*common.v2601);
        let v4390=(self.scalar_static_f64[854]*common.v2602);
        let v4391=(common.v32*v1695);
        let v4399=(v1696*v1696);
        let v4421=(self.scalar_static_f64[855]*common.v2627);
        let v4422=(self.scalar_static_f64[855]*common.v2628);
        let v4424=(self.scalar_static_f64[855]*common.v2629);
        let v4432=(self.scalar_static_f64[854]*common.v2627);
        let v4433=(self.scalar_static_f64[854]*common.v2628);
        let v4435=(self.scalar_static_f64[854]*common.v2629);
        let v4437=(common.v32*v1708);
        let v4447=(v1709*v1709);
        let v4475=(common.v32*v1717);
        let v4481=(v1718*v1718);
        let v4493=(common.v32*v1724);
        let v4501=(v1725*v1725);
        let v4510=(((v1725*v4424)-(v1721*(v4435/v4493)))/v4501);
        let v4515=(if self.scalar_static_bool[41]{common.v4}else{(if self.scalar_static_bool[40]{(((v1709*(self.scalar_static_f64[855]*(-v2685)))-(v1703*((self.scalar_static_f64[854]*(self.scalar_static_f64[245]*v2685))/v4437)))/v4447)}else{common.v4})});
        let v4516=(if self.scalar_static_bool[41]{(((v1725*v4421)-(v1721*(v4432/v4493)))/v4501)}else{(if self.scalar_static_bool[40]{(((v1709*v4421)-(v1703*(v4432/v4437)))/v4447)}else{common.v4})});
        let v4517=(if self.scalar_static_bool[41]{(((v1725*v4422)-(v1721*(v4433/v4493)))/v4501)}else{(if self.scalar_static_bool[40]{(((v1709*v4422)-(v1703*(v4433/v4437)))/v4447)}else{common.v4})});
        let v4518=(if self.scalar_static_bool[41]{v4510}else{(if self.scalar_static_bool[40]{(((v1709*(self.scalar_static_f64[855]*(common.v2629-v2686)))-(v1703*((self.scalar_static_f64[854]*(common.v2629+(self.scalar_static_f64[245]*v2686)))/v4437)))/v4447)}else{common.v4})});
        let v4519=(if self.scalar_static_bool[41]{v4510}else{(if self.scalar_static_bool[40]{(((v1709*v4424)-(v1703*(v4435/v4437)))/v4447)}else{common.v4})});
        let v4520=(if self.scalar_static_bool[41]{(((v1725*(self.scalar_static_f64[855]*common.v2630))-(v1721*((self.scalar_static_f64[854]*common.v2630)/v4493)))/v4501)}else{(if self.scalar_static_bool[40]{(((v1709*(self.scalar_static_f64[855]*(common.v2630-v2687)))-(v1703*((self.scalar_static_f64[854]*(common.v2630+(self.scalar_static_f64[245]*v2687)))/v4437)))/v4447)}else{common.v4})});
        let v4525=(common.v32*v1736);
        let v4531=(v1737*v1737);
        let v4847=(common.v1821*common.v4591);
        let v4857=(common.v1821*common.v4593);
        let v4876=(common.v1821*common.v4677);
        let v4888=(common.v1821*common.v4680);
        let v4914=(v1829*self.scalar_static_f64[352]);
        let v4916=(v1829*self.scalar_static_f64[353]);
        let v4918=(v1829*self.scalar_static_f64[354]);
        let v4929=(common.v32*v1838);
        let v4930=((if self.scalar_static_bool[51]{common.v4}else{common.v4695})/v4929);
        let v4931=((if self.scalar_static_bool[51]{common.v4}else{common.v4696})/v4929);
        let v4932=((if self.scalar_static_bool[51]{common.v4}else{common.v4697})/v4929);
        let v4933=((if self.scalar_static_bool[51]{(v4914+v4914)}else{common.v4695})/v4929);
        let v4934=((if self.scalar_static_bool[51]{(v4916+v4916)}else{common.v4698})/v4929);
        let v4935=((if self.scalar_static_bool[51]{(v4918+v4918)}else{common.v4699})/v4929);
        let v4936=((if self.scalar_static_bool[51]{common.v4}else{common.v4700})/v4929);
        let v4937=((if self.scalar_static_bool[51]{common.v4}else{common.v4701})/v4929);
        let v4938=((if self.scalar_static_bool[51]{common.v4}else{common.v4702})/v4929);
        let v4944=(v1839*v1839);
        let v4991=(if v1843{(common.v424*v4930)}else{(if v1835{((-(self.scalar_static_f64[257]*v4930))/v4944)}else{common.v4})});
        let v4992=(if v1843{(common.v424*v4931)}else{(if v1835{((-(self.scalar_static_f64[257]*v4931))/v4944)}else{common.v4})});
        let v4993=(if v1843{(common.v424*v4932)}else{(if v1835{((-(self.scalar_static_f64[257]*v4932))/v4944)}else{common.v4})});
        let v4994=(if v1843{(common.v424*(self.scalar_static_f64[355]+v4933))}else{(if v1835{((-(self.scalar_static_f64[257]*(v4933-self.scalar_static_f64[355])))/v4944)}else{common.v4})});
        let v4995=(if v1843{(common.v424*(self.scalar_static_f64[356]+v4934))}else{(if v1835{((-(self.scalar_static_f64[257]*(v4934-self.scalar_static_f64[356])))/v4944)}else{common.v4})});
        let v4996=(if v1843{(common.v424*(self.scalar_static_f64[357]+v4935))}else{(if v1835{((-(self.scalar_static_f64[257]*(v4935-self.scalar_static_f64[357])))/v4944)}else{common.v4})});
        let v4997=(if v1843{(common.v424*v4936)}else{(if v1835{((-(self.scalar_static_f64[257]*v4936))/v4944)}else{common.v4})});
        let v4998=(if v1843{(common.v424*v4937)}else{(if v1835{((-(self.scalar_static_f64[257]*v4937))/v4944)}else{common.v4})});
        let v4999=(if v1843{(common.v424*v4938)}else{(if v1835{((-(self.scalar_static_f64[257]*v4938))/v4944)}else{common.v4})});
        let v5010=(self.scalar_static_f64[258]*f64::powf(v1864,self.scalar_static_f64[267]));
        let v5020=(v1866*v1866);
        let v5057=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4991)}else{(if v1863{(((v4991/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5058=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4992)}else{(if v1863{(((v4992/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5059=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4993)}else{(if v1863{(((v4993/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5060=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4994)}else{(if v1863{(((v4994/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5061=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4995)}else{(if v1863{(((v4995/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5062=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4996)}else{(if v1863{(((v4996/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5063=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4997)}else{(if v1863{(((v4997/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5064=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4998)}else{(if v1863{(((v4998/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5065=(if self.scalar_static_bool[52]{common.v4}else{(if v1870{(self.scalar_static_f64[272]*v4999)}else{(if v1863{(((v4999/self.scalar_static_f64[263])*v5010)/v5020)}else{common.v4})})});
        let v5088=(v1876*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4374)}else{v4374}));
        let v5108=(v1876*(self.scalar_static_f64[664]*common.v3937));
        let v5117=(v1876*(if self.scalar_static_bool[44]{(v4847+(common.v1758*common.v4838))}else{common.v4}));
        let v5204=(v1892*v1892);
        let v5219=(common.v155*(if v1894{common.v4}else{((-(self.scalar_static_f64[564]*((common.v1891*common.v3584)+(common.v1262*common.v5186))))/v5204)}));
        let v5220=(common.v155*(if v1894{common.v4}else{((-(self.scalar_static_f64[564]*((common.v1891*common.v3585)+(common.v1262*common.v5187))))/v5204)}));
        let v5221=(common.v155*(if v1894{common.v4}else{((-(self.scalar_static_f64[564]*((common.v1891*common.v3586)+(common.v1262*common.v5188))))/v5204)}));
        let v5222=(common.v155*(if v1894{common.v4}else{((-(self.scalar_static_f64[564]*((common.v1891*common.v3587)+(common.v1262*common.v5189))))/v5204)}));
        let v5229=(v1896*v1896);
        let v5246=((-v3612)/self.scalar_static_f64[275]);
        let v5247=((-v3616)/self.scalar_static_f64[275]);
        let v5248=((-v3620)/self.scalar_static_f64[275]);
        let v5249=((-v3624)/self.scalar_static_f64[275]);
        let v5274=(if common.v1911{(common.v1922*(if common.v1916{(common.v1917*v5246)}else{(if common.v1912{(common.v1913*v5246)}else{common.v4})}))}else{common.v4});
        let v5275=(if common.v1911{((common.v1922*(if common.v1916{(common.v1917*v5247)}else{(if common.v1912{(common.v1913*v5247)}else{common.v4})}))+(common.v1921*self.scalar_static_f64[321]))}else{common.v4});
        let v5276=(if common.v1911{((common.v1922*(if common.v1916{(common.v1917*v5248)}else{(if common.v1912{(common.v1913*v5248)}else{common.v4})}))+(self.scalar_static_f64[0]*common.v1921))}else{common.v4});
        let v5277=(if common.v1911{(common.v1922*(if common.v1916{(common.v1917*v5249)}else{(if common.v1912{(common.v1913*v5249)}else{common.v4})}))}else{common.v4});
        let v5280=(self.scalar_static_f64[276]*f64::powf(common.v1924,self.scalar_static_f64[358]));
        let v5285=(self.scalar_static_f64[872]*(v5274*v5280));
        let v5286=(self.scalar_static_f64[872]*(v5275*v5280));
        let v5287=(self.scalar_static_f64[872]*(v5276*v5280));
        let v5288=(self.scalar_static_f64[872]*(v5277*v5280));
        let v5301=(if common.v1934{(common.v1935*v5285)}else{(if v1930{(v1931*v5285)}else{common.v4})});
        let v5302=(if common.v1934{(common.v1935*v5286)}else{(if v1930{(v1931*v5286)}else{common.v4})});
        let v5303=(if common.v1934{(common.v1935*v5287)}else{(if v1930{(v1931*v5287)}else{common.v4})});
        let v5304=(if common.v1934{(common.v1935*v5288)}else{(if v1930{(v1931*v5288)}else{common.v4})});
        let v5328=(common.v1125*common.v1125);
        let v5337=(if v1950{(((common.v1125*self.scalar_static_f64[321])-(v1957*common.v3278))/v5328)}else{common.v3069});
        let v5338=(if v1950{(((self.scalar_static_f64[0]*common.v1125)-(v1957*common.v3279))/v5328)}else{common.v3070});
        let v5339=(if v1950{((-(v1957*common.v3280))/v5328)}else{common.v3071});
        let v5346=(common.v32*v1962);
        let v5350=(if v1950{(((common.v32*v5337)/v1956)/v5346)}else{common.v4});
        let v5351=(if v1950{(((common.v32*v5338)/v1956)/v5346)}else{common.v4});
        let v5352=(if v1950{(((common.v32*v5339)/v1956)/v5346)}else{common.v4});
        let v5359=(if v1969{(-(common.v424*common.v3260))}else{common.v4});
        let v5360=(if v1969{(-(common.v424*common.v3261))}else{common.v4});
        let v5361=(if v1969{(-(common.v424*common.v3262))}else{common.v4});
        let v5374=(if v1969{((v1973*v5359)+(v1972*(self.scalar_static_f64[280]*v5359)))}else{common.v4});
        let v5375=(if v1969{((v1973*v5360)+(v1972*(self.scalar_static_f64[280]*v5360)))}else{common.v4});
        let v5376=(if v1969{((v1973*v5361)+(v1972*(self.scalar_static_f64[280]*v5361)))}else{common.v4});
        let v5386=(v1963*v5350);
        let v5388=(v1963*v5351);
        let v5390=(v1963*v5352);
        let v5392=(v1975*v5374);
        let v5394=(v1975*v5375);
        let v5396=(v1975*v5376);
        let v5401=(common.v32*v1980);
        let v5408=(v1980*v1980);
        let v5418=(if v1950{(((v1980*((v1975*v5350)+(v1963*v5374)))-(v1976*(((v5386+v5386)+(v5392+v5392))/v5401)))/v5408)}else{common.v4});
        let v5419=(if v1950{(((v1980*((v1975*v5351)+(v1963*v5375)))-(v1976*(((v5388+v5388)+(v5394+v5394))/v5401)))/v5408)}else{common.v4});
        let v5420=(if v1950{(((v1980*((v1975*v5352)+(v1963*v5376)))-(v1976*(((v5390+v5390)+(v5396+v5396))/v5401)))/v5408)}else{common.v4});
        let v5424=(v1982*v1982);
        let v5433=(if v1950{(((v1982*self.scalar_static_f64[321])-(v1957*v5418))/v5424)}else{common.v4});
        let v5434=(if v1950{(((self.scalar_static_f64[0]*v1982)-(v1957*v5419))/v5424)}else{common.v4});
        let v5435=(if v1950{((-(v1957*v5420))/v5424)}else{common.v4});
        let v5436=(common.v424*v5418);
        let v5437=(common.v424*v5419);
        let v5438=(common.v424*v5420);
        let v5439=(v1956*v5436);
        let v5440=(v1956*v5437);
        let v5441=(v1956*v5438);
        let v5454=(if v1950{(v5433+((v1986*common.v3278)+(common.v1125*v5439)))}else{common.v4});
        let v5455=(if v1950{(v5434+((v1986*common.v3279)+(common.v1125*v5440)))}else{common.v4});
        let v5456=(if v1950{(v5435+((v1986*common.v3280)+(common.v1125*v5441)))}else{common.v4});
        let v5476=(v2002*v2002);
        let v5504=(if v1969{(-(v1986*(-(v3612/v2002))))}else{common.v4});
        let v5505=(if v1969{(v5433-((v2004*v5439)+(v1986*(-(((v2002*v3616)-(common.v1269*(self.scalar_static_f64[207]*(if v1969{(self.scalar_static_f64[285]*(common.v32*common.v3260))}else{common.v4}))))/v5476)))))}else{common.v4});
        let v5506=(if v1969{(v5434-((v2004*v5440)+(v1986*(-(((v2002*v3620)-(common.v1269*(self.scalar_static_f64[207]*(if v1969{(self.scalar_static_f64[285]*(common.v32*common.v3261))}else{common.v4}))))/v5476)))))}else{common.v4});
        let v5507=(if v1969{(v5435-((v2004*v5441)+(v1986*(-(((v2002*v3624)-(common.v1269*(self.scalar_static_f64[207]*(if v1969{(self.scalar_static_f64[285]*(common.v32*common.v3262))}else{common.v4}))))/v5476)))))}else{common.v4});
        let v5511=(v2008*v5504);
        let v5513=(v2008*(v5505-v5454));
        let v5515=(v2008*(v5506-v5455));
        let v5517=(v2008*(v5507-v5456));
        let v5553=(common.v32*v2017);
        let v5566=(if v1969{(common.v424*(v5504+((if v1969{(v5511+v5511)}else{common.v4})/v5553)))}else{common.v4});
        let v5567=(if v1969{(common.v424*((v5454+v5505)+((if v1969{((v5513+v5513)+(((v2011*common.v3269)+(common.v1122*((v2010*v5433)+(v1984*(common.v47*v5433)))))/self.scalar_static_f64[207]))}else{v5337})/v5553)))}else{(if v1966{v5454}else{common.v4})});
        let v5568=(if v1969{(common.v424*((v5455+v5506)+((if v1969{((v5515+v5515)+(((v2011*common.v3270)+(common.v1122*((v2010*v5434)+(v1984*(common.v47*v5434)))))/self.scalar_static_f64[207]))}else{v5338})/v5553)))}else{(if v1966{v5455}else{common.v4})});
        let v5569=(if v1969{(common.v424*((v5456+v5507)+((if v1969{((v5517+v5517)+(((v2011*common.v3271)+(common.v1122*((v2010*v5435)+(v1984*(common.v47*v5435)))))/self.scalar_static_f64[207]))}else{v5339})/v5553)))}else{(if v1966{v5456}else{common.v4})});
        let v5576=(v2020*v2020);
        let v5596=(v2023*v2023);
        let v5610=(if v2027{((-(v1985*(if v1950{(((v2020*v5566)-(v2021*v5566))/v5576)}else{common.v4})))/v5596)}else{common.v4});
        let v5611=(if v2027{(((v2023*v5436)-(v1985*(if v1950{(((v2020*(v5567-v5433))-(v2021*v5567))/v5576)}else{common.v4})))/v5596)}else{common.v4});
        let v5612=(if v2027{(((v2023*v5437)-(v1985*(if v1950{(((v2020*(v5568-v5434))-(v2021*v5568))/v5576)}else{common.v4})))/v5596)}else{common.v4});
        let v5613=(if v2027{(((v2023*v5438)-(v1985*(if v1950{(((v2020*(v5569-v5435))-(v2021*v5569))/v5576)}else{common.v4})))/v5596)}else{common.v4});
        let v5632=((-(self.scalar_static_f64[875]*v5566))/v5576);
        let v5635=((-(self.scalar_static_f64[875]*v5567))/v5576);
        let v5638=((-(self.scalar_static_f64[875]*v5568))/v5576);
        let v5641=((-(self.scalar_static_f64[875]*v5569))/v5576);
        let v5642=(v2035*v5632);
        let v5643=(v2035*v5635);
        let v5644=(v2035*v5638);
        let v5645=(v2035*v5641);
        let v5648=(v2029*v2029);
        let v5716=(self.scalar_static_f64[276]*f64::powf(common.v1922,self.scalar_static_f64[358]));
        let v5722=(common.v2055*common.v2055);
        let v5742=(self.scalar_static_f64[290]*f64::powf(common.v2057,self.scalar_static_f64[359]));
        let v5755=(if common.v2052{(common.v2053*((-(((common.v2055*v3612)-(common.v1269*v3612))/v5722))*v5742))}else{common.v4});
        let v5756=(if common.v2052{((common.v2059*(self.scalar_static_f64[321]*v5716))+(common.v2053*((-(((common.v2055*v3616)-(common.v1269*v3616))/v5722))*v5742)))}else{common.v4});
        let v5757=(if common.v2052{((common.v2059*(self.scalar_static_f64[0]*v5716))+(common.v2053*((-(((common.v2055*v3620)-(common.v1269*v3620))/v5722))*v5742)))}else{common.v4});
        let v5758=(if common.v2052{(common.v2053*((-(((common.v2055*v3624)-(common.v1269*v3624))/v5722))*v5742))}else{common.v4});
        let v5767=(if common.v2064{(v3612/self.scalar_static_f64[289])}else{common.v4});
        let v5768=(if common.v2064{(v3616/self.scalar_static_f64[289])}else{common.v4});
        let v5769=(if common.v2064{(v3620/self.scalar_static_f64[289])}else{common.v4});
        let v5770=(if common.v2064{(v3624/self.scalar_static_f64[289])}else{common.v4});
        let v5775=(if common.v2064{(v5767/self.scalar_static_f64[292])}else{self.scalar_static_f64[334]});
        let v5776=(if common.v2064{(v5768/self.scalar_static_f64[292])}else{self.scalar_static_f64[335]});
        let v5777=(if common.v2064{(v5769/self.scalar_static_f64[292])}else{common.v4});
        let v5778=(if common.v2064{(v5770/self.scalar_static_f64[292])}else{common.v4});
        let v5821=(self.scalar_static_f64[293]*f64::powf(common.v2089,self.scalar_static_f64[360]));
        let v5842=(self.scalar_static_f64[872]*(if common.v2064{((common.v2091*v5755)+(common.v2061*((if common.v2082{(v5767+(self.scalar_static_f64[292]*((common.v2084*(-v5775))/common.v2085)))}else{(if common.v2074{(self.scalar_static_f64[292]*((common.v2075*v5775)/common.v2076))}else{common.v4})})*v5821)))}else{(if common.v2062{v5755}else{common.v4})}));
        let v5843=(self.scalar_static_f64[872]*(if common.v2064{((common.v2091*v5756)+(common.v2061*((if common.v2082{(v5768+(self.scalar_static_f64[292]*((common.v2084*(-v5776))/common.v2085)))}else{(if common.v2074{(self.scalar_static_f64[292]*((common.v2075*v5776)/common.v2076))}else{common.v4})})*v5821)))}else{(if common.v2062{v5756}else{common.v4})}));
        let v5844=(self.scalar_static_f64[872]*(if common.v2064{((common.v2091*v5757)+(common.v2061*((if common.v2082{(v5769+(self.scalar_static_f64[292]*((common.v2084*(-v5777))/common.v2085)))}else{(if common.v2074{(self.scalar_static_f64[292]*((common.v2075*v5777)/common.v2076))}else{common.v4})})*v5821)))}else{(if common.v2062{v5757}else{common.v4})}));
        let v5845=(self.scalar_static_f64[872]*(if common.v2064{((common.v2091*v5758)+(common.v2061*((if common.v2082{(v5770+(self.scalar_static_f64[292]*((common.v2084*(-v5778))/common.v2085)))}else{(if common.v2074{(self.scalar_static_f64[292]*((common.v2075*v5778)/common.v2076))}else{common.v4})})*v5821)))}else{(if common.v2062{v5758}else{common.v4})}));
        let v5872=(if common.v2052{(v2106*(if common.v2100{(common.v2101*v5842)}else{(if v2096{(v2097*v5842)}else{v5301})}))}else{(if v2044{(v2045*v5642)}else{(if v2027{((v2040*((v2031*v5610)+(v2029*(self.scalar_static_f64[874]*v5566))))+(v2032*(v5642-(v2039*((v2037*v5632)+(v2034*((-(v1975*v5610))/v5648)))))))}else{(if common.v1911{((v1942*v5301)+(v1939*(self.scalar_static_f64[873]*v5274)))}else{common.v4})})})});
        let v5873=(if common.v2052{((v2106*(if common.v2100{(common.v2101*v5843)}else{(if v2096{(v2097*v5843)}else{v5302})}))+(v2105*self.scalar_static_f64[935]))}else{(if v2044{((v2045*v5643)+(v2035*(self.scalar_static_f64[3]*v5374)))}else{(if v2027{((v2040*((v2031*v5611)+(v2029*(self.scalar_static_f64[874]*v5567))))+(v2032*(v5643-(v2039*((v2037*v5635)+(v2034*(((v2029*v5374)-(v1975*v5611))/v5648)))))))}else{(if common.v1911{((v1942*v5302)+(v1939*(self.scalar_static_f64[873]*v5275)))}else{common.v4})})})});
        let v5874=(if common.v2052{((v2106*(if common.v2100{(common.v2101*v5844)}else{(if v2096{(v2097*v5844)}else{v5303})}))+(v2105*self.scalar_static_f64[936]))}else{(if v2044{((v2045*v5644)+(v2035*(self.scalar_static_f64[3]*v5375)))}else{(if v2027{((v2040*((v2031*v5612)+(v2029*(self.scalar_static_f64[874]*v5568))))+(v2032*(v5644-(v2039*((v2037*v5638)+(v2034*(((v2029*v5375)-(v1975*v5612))/v5648)))))))}else{(if common.v1911{((v1942*v5303)+(v1939*(self.scalar_static_f64[873]*v5276)))}else{common.v4})})})});
        let v5875=(if common.v2052{(v2106*(if common.v2100{(common.v2101*v5845)}else{(if v2096{(v2097*v5845)}else{v5304})}))}else{(if v2044{((v2045*v5645)+(v2035*(self.scalar_static_f64[3]*v5376)))}else{(if v2027{((v2040*((v2031*v5613)+(v2029*(self.scalar_static_f64[874]*v5569))))+(v2032*(v5645-(v2039*((v2037*v5641)+(v2034*(((v2029*v5376)-(v1975*v5613))/v5648)))))))}else{(if common.v1911{((v1942*v5304)+(v1939*(self.scalar_static_f64[873]*v5277)))}else{common.v4})})})});
        let v5890=(v2115*v2115);
        let v5915=(v2114*v2114);
        let v5930=(if v2113{((((-(self.scalar_static_f64[381]*((v2114*v3612)+(common.v1269*v5219))))/v5890)+(self.scalar_static_f64[672]*(common.v3590/self.scalar_static_f64[644])))+((-(self.scalar_static_f64[561]*v5219))/v5915))}else{common.v4});
        let v5931=(if v2113{((((-(self.scalar_static_f64[381]*((v2114*v3616)+(common.v1269*v5220))))/v5890)+(self.scalar_static_f64[672]*(common.v3593/self.scalar_static_f64[644])))+((-(self.scalar_static_f64[561]*v5220))/v5915))}else{common.v4});
        let v5932=(if v2113{((((-(self.scalar_static_f64[381]*((v2114*v3620)+(common.v1269*v5221))))/v5890)+(self.scalar_static_f64[672]*(common.v3596/self.scalar_static_f64[644])))+((-(self.scalar_static_f64[561]*v5221))/v5915))}else{common.v4});
        let v5933=(if v2113{((((-(self.scalar_static_f64[381]*((v2114*v3624)+(common.v1269*v5222))))/v5890)+(self.scalar_static_f64[672]*(common.v3599/self.scalar_static_f64[644])))+((-(self.scalar_static_f64[561]*v5222))/v5915))}else{common.v4});
        let v5942=(if v2123{((v5872-v5930)/common.v421)}else{v5775});
        let v5943=(if v2123{((v5873-v5931)/common.v421)}else{v5776});
        let v5944=(if v2123{((v5874-v5932)/common.v421)}else{v5777});
        let v5945=(if v2123{((v5875-v5933)/common.v421)}else{v5778});
        let v5986=(if v2136{(v5930-(common.v421*((v2138*(-v5942))/v2139)))}else{(if v2128{(v5872-(common.v421*((v2129*v5942)/v2130)))}else{v5872})});
        let v5987=(if v2136{(v5931-(common.v421*((v2138*(-v5943))/v2139)))}else{(if v2128{(v5873-(common.v421*((v2129*v5943)/v2130)))}else{v5873})});
        let v5988=(if v2136{(v5932-(common.v421*((v2138*(-v5944))/v2139)))}else{(if v2128{(v5874-(common.v421*((v2129*v5944)/v2130)))}else{v5874})});
        let v5989=(if v2136{(v5933-(common.v421*((v2138*(-v5945))/v2139)))}else{(if v2128{(v5875-(common.v421*((v2129*v5945)/v2130)))}else{v5875})});
        let v5992=((v2143*v3612)+(common.v1269*v5986));
        let v5995=((v2143*v3616)+(common.v1269*v5987));
        let v5998=((v2143*v3620)+(common.v1269*v5988));
        let v6001=((v2143*v3624)+(common.v1269*v5989));
        let v6025=(v2149*v2149);
        let v7008=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[710]*common.v3958)));
        let v7012=((self.scalar_static_f64[347]+((if self.scalar_static_bool[33]{(self.scalar_static_f64[672]*((self.scalar_static_f64[233]*common.v3705)+(v1370*(self.scalar_static_f64[232]*common.v3705))))}else{(if self.scalar_static_bool[31]{v3735}else{(if self.scalar_static_bool[12]{((v3735+(v1370*(((v1368*(self.scalar_static_f64[845]*common.v3705))-(v1364*((common.v436*v3713)/v3741)))/v3747)))+(((v1376*(v1374*v3731))-(v1375*v3731))/v3777))}else{common.v4})})})+(self.scalar_static_f64[657]*common.v3896)))-(if v1569{common.v4}else{(if common.v1483{(self.scalar_static_f64[20]*(self.scalar_static_f64[541]*((v1564*(if common.v1493{(common.v1494*v3976)}else{(if v1489{(v1490*v3976)}else{common.v4})}))+(v1498*((v1563*common.v3312)+(common.v1151*(self.scalar_static_f64[847]*(if v1550{((v1559*((v1551*v4051)+(common.v1528*self.scalar_static_f64[341])))+(v1552*((v1557*(v1553*v4051))+(v1554*(v1555*v4051)))))}else{(if common.v1532{((self.scalar_static_f64[0]*v1546)+(v1543*(((common.v1528*(-(if common.v1537{(common.v1538*v4051)}else{(if v1533{(v1534*v4051)}else{common.v4})})))-(v1544*v4051))/v4066)))}else{common.v4})}))))))))}else{common.v4})}));
        let v7013=((self.scalar_static_f64[346]+((if self.scalar_static_bool[33]{(self.scalar_static_f64[672]*((self.scalar_static_f64[233]*common.v3706)+((v1391*common.v3492)+(v1370*(self.scalar_static_f64[232]*(common.v3217+common.v3706))))))}else{(if self.scalar_static_bool[31]{v3736}else{(if self.scalar_static_bool[12]{((v3736+((v1370*(((v1368*(self.scalar_static_f64[845]*common.v3706))-(v1364*((common.v436*v3714)/v3741)))/v3747))+(v1369*common.v3492)))+(((v1376*((v1374*v3732)+(v1360*(self.scalar_static_f64[687]*common.v3217))))-(v1375*v3732))/v3777))}else{common.v4})})})+(self.scalar_static_f64[657]*common.v3898)))-(if v1569{common.v4}else{(if common.v1483{(self.scalar_static_f64[20]*(self.scalar_static_f64[541]*((v1564*(if common.v1493{(common.v1494*v3977)}else{(if v1489{(v1490*v3977)}else{common.v4})}))+(v1498*((v1563*common.v3313)+(common.v1151*(self.scalar_static_f64[847]*(if v1550{((v1559*((v1551*v4052)+(common.v1528*self.scalar_static_f64[342])))+(v1552*((v1557*(v1553*v4052))+(v1554*(v1555*v4052)))))}else{(if common.v1532{((v1546*self.scalar_static_f64[321])+(v1543*(((common.v1528*(-(if common.v1537{(common.v1538*v4052)}else{(if v1533{(v1534*v4052)}else{common.v4})})))-(v1544*v4052))/v4066)))}else{common.v4})}))))))))}else{common.v4})}));
        let v7046=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5057))));
        let v7047=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5058))));
        let v7048=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5059))));
        let v7049=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5060))));
        let v7050=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-((v1876*(if v1658{common.v4}else{(if common.v1575{(self.scalar_static_f64[51]*(self.scalar_static_f64[542]*((v1653*(if common.v1589{(common.v1590*v4149)}else{(if v1585{(v1586*v4149)}else{common.v4})}))+(v1594*((v1652*v4136)+(common.v1579*(self.scalar_static_f64[848]*(if v1641{((v1648*((v1642*v4222)+(common.v1620*self.scalar_static_f64[342])))+(v1643*((v1646*(v1553*v4222))+(v1644*(v1555*v4222)))))}else{(if common.v1623{((v1637*self.scalar_static_f64[321])+(v1634*(((common.v1620*(-(if common.v1628{(common.v1629*v4222)}else{(if v1624{(v1625*v4222)}else{common.v4})})))-(v1635*v4222))/v4237)))}else{common.v4})}))))))))}else{common.v4})}))+(v1659*v5061)))));
        let v7051=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-((v1876*(if v1658{common.v4}else{(if common.v1575{(self.scalar_static_f64[51]*(self.scalar_static_f64[542]*((v1653*(if common.v1589{(common.v1590*v4150)}else{(if v1585{(v1586*v4150)}else{common.v4})}))+(v1594*((v1652*v4137)+(common.v1579*(self.scalar_static_f64[848]*(if v1641{((v1648*((v1642*v4223)+(common.v1620*self.scalar_static_f64[341])))+(v1643*((v1646*(v1553*v4223))+(v1644*(v1555*v4223)))))}else{(if common.v1623{((self.scalar_static_f64[0]*v1637)+(v1634*(((common.v1620*(-(if common.v1628{(common.v1629*v4223)}else{(if v1624{(v1625*v4223)}else{common.v4})})))-(v1635*v4223))/v4237)))}else{common.v4})}))))))))}else{common.v4})}))+(v1659*v5062)))));
        let v7052=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5063))));
        let v7053=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5064))));
        let v7054=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(v1659*v5065))));
        let v7102=(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(v4876+(common.v1782*common.v4838))}else{common.v4})));
        let v7149=ddt_scale;
        let v7270=(self.scalar_static_f64[13]*(v7149*common.v7252));
        let v7312=(self.scalar_static_f64[13]*(v7149*common.v7304));

        stamper.stamp_current_node3_local(
            Some(7),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v920))),
            6,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2780))),
            7,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2781))),
            8,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v2782))),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*common.v1269))),
            [4, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3612)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3616)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3620)), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*v3624))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[710]*(common.v1477-common.v1))+((if self.scalar_static_bool[30]{v1421}else{(if self.scalar_static_bool[12]{(v1421+(v1423/v1427))}else{common.v4})})+(self.scalar_static_f64[704]*(common.v1453-common.v1))))))),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[710]*common.v3955)+((if self.scalar_static_bool[30]{v3852}else{(if self.scalar_static_bool[12]{(v3852+(((v1427*(self.scalar_static_f64[846]*common.v3839))-(v1423*((common.v436*(if common.v1414{(common.v1415*self.scalar_static_f64[898])}else{(if v1410{(v1411*self.scalar_static_f64[898])}else{v3713})}))/v3861)))/v3868))}else{common.v4})})+(self.scalar_static_f64[704]*common.v3911))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[710]*common.v3956)+((if self.scalar_static_bool[30]{v3853}else{(if self.scalar_static_bool[12]{(v3853+(((v1427*(self.scalar_static_f64[846]*common.v3840))-(v1423*((common.v436*(if common.v1414{(common.v1415*self.scalar_static_f64[897])}else{(if v1410{(v1411*self.scalar_static_f64[897])}else{common.v4})}))/v3861)))/v3868))}else{common.v4})})+(self.scalar_static_f64[704]*common.v3912))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[710]*common.v3957)+((if self.scalar_static_bool[30]{v3854}else{(if self.scalar_static_bool[12]{(v3854+(((v1427*(self.scalar_static_f64[846]*common.v3841))-(v1423*((common.v436*(if common.v1414{common.v4}else{(if v1410{common.v4}else{v3714})}))/v3861)))/v3868))}else{common.v4})})+(self.scalar_static_f64[704]*common.v3913))))), v7008, v7008, (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[710]*common.v3959)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[715]*(v1296-common.v1))+((v1318*v1320)+((((if self.scalar_static_bool[33]{(self.scalar_static_f64[672]*((v1361*self.scalar_static_f64[233])+(v1370*v1391)))}else{(if self.scalar_static_bool[31]{v1362}else{(if self.scalar_static_bool[12]{((v1362+(v1369*v1370))+(v1375/v1376))}else{common.v4})})})+(self.scalar_static_f64[657]*(common.v1441-common.v1)))+(common.v4*common.v736))-(if v1569{common.v4}else{(if common.v1483{(self.scalar_static_f64[20]*(self.scalar_static_f64[541]*(v1498*v1564)))}else{common.v4})}))))))),
            [4, 5, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[715]*v3655)+(((v1320*(self.scalar_static_f64[231]*v3681))+(v1318*((-v3681)*v3688)))+v7012)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(self.scalar_static_f64[657]*common.v3897))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[715]*v3656)+(((v1320*(self.scalar_static_f64[231]*v3682))+(v1318*((-v3682)*v3688)))+v7013)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[672]*((v1391*common.v3493)+(v1370*(self.scalar_static_f64[232]*common.v3218))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1369*common.v3493)+(((v1376*((v1374*v3733)+(v1360*(self.scalar_static_f64[687]*common.v3218))))-(v1375*v3733))/v3777))}else{common.v4})})}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[33]{(self.scalar_static_f64[672]*((v1391*common.v3494)+(v1370*(self.scalar_static_f64[232]*common.v3219))))}else{(if self.scalar_static_bool[31]{common.v4}else{(if self.scalar_static_bool[12]{((v1369*common.v3494)+(((v1376*((v1374*v3734)+(v1360*(self.scalar_static_f64[687]*common.v3219))))-(v1375*v3734))/v3777))}else{common.v4})})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[12]{v2519}else{common.v4})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[12]{v7046}else{common.v4}), (if self.scalar_static_bool[12]{v7047}else{common.v4}), (if self.scalar_static_bool[12]{v7048}else{common.v4}), (if self.scalar_static_bool[12]{v7049}else{common.v4}), (if self.scalar_static_bool[12]{v7050}else{common.v4}), (if self.scalar_static_bool[12]{v7051}else{common.v4}), (if self.scalar_static_bool[12]{v7052}else{common.v4}), (if self.scalar_static_bool[12]{v7053}else{common.v4}), (if self.scalar_static_bool[12]{v7054}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(8),
            multiplicity * ((if self.scalar_static_bool[30]{v2519}else{common.v4})),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(if self.scalar_static_bool[30]{v7046}else{common.v4}), (if self.scalar_static_bool[30]{v7047}else{common.v4}), (if self.scalar_static_bool[30]{v7048}else{common.v4}), (if self.scalar_static_bool[30]{v7049}else{common.v4}), (if self.scalar_static_bool[30]{v7050}else{common.v4}), (if self.scalar_static_bool[30]{v7051}else{common.v4}), (if self.scalar_static_bool[30]{v7052}else{common.v4}), (if self.scalar_static_bool[30]{v7053}else{common.v4}), (if self.scalar_static_bool[30]{v7054}else{common.v4})],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v1727)}else{v1727})))),
            [3, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4515)}else{v4515}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4516)}else{v4516}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4517)}else{v4517}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4518)}else{v4518}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4519)}else{v4519}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4520)}else{v4520})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(v1714/v1718)}else{(if self.scalar_static_bool[40]{(v1687/v1696)}else{common.v4})})))),
            [3, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{common.v4}else{(if self.scalar_static_bool[40]{(((v1696*(self.scalar_static_f64[852]*(-v2662)))-(v1687*((self.scalar_static_f64[854]*(self.scalar_static_f64[245]*v2662))/v4391)))/v4399)}else{common.v4})}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(((v1718*v4382)-(v1714*(v4388/v4475)))/v4481)}else{(if self.scalar_static_bool[40]{(((v1696*v4382)-(v1687*(v4388/v4391)))/v4399)}else{common.v4})}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{common.v4}else{(if self.scalar_static_bool[40]{(((v1696*(self.scalar_static_f64[852]*(-v2663)))-(v1687*((self.scalar_static_f64[854]*(self.scalar_static_f64[245]*v2663))/v4391)))/v4399)}else{common.v4})}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[41]{(((v1718*v4384)-(v1714*(v4390/v4475)))/v4481)}else{(if self.scalar_static_bool[40]{(((v1696*v4384)-(v1687*(v4390/v4391)))/v4399)}else{common.v4})})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(common.v1782*common.v1821)}else{common.v4})))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v7102, (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{((common.v1821*common.v4678)+(common.v1782*common.v4839))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{((common.v1821*common.v4679)+(common.v1782*common.v4840))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(common.v1782*common.v4841)}else{common.v4}))), v7102, (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(v4876+(common.v1782*common.v4842))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(v4888+(common.v1782*common.v4843))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{((common.v1821*common.v4681)+(common.v1782*common.v4844))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{((common.v1821*common.v4682)+(common.v1782*common.v4845))}else{common.v4}))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(if self.scalar_static_bool[44]{(v4888+(common.v1782*common.v4846))}else{common.v4})))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1730/v1737)+(common.v4*common.v744))))),
            3,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((((v1737*(self.scalar_static_f64[856]*v2662))-(v1730*((self.scalar_static_f64[858]*v2662)/v4525)))/v4531)+self.scalar_static_f64[346])))),
            7,
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((((v1737*(self.scalar_static_f64[856]*v2663))-(v1730*((self.scalar_static_f64[858]*v2663)/v4525)))/v4531)+self.scalar_static_f64[347])))),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1899/v1896)))),
            [4, 5, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1899*v5219))/v5229))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((self.scalar_static_f64[0]+(self.scalar_static_f64[818]*(if common.v811{(common.v812*self.scalar_static_f64[897])}else{(if common.v808{(v809*self.scalar_static_f64[897])}else{common.v4})})))/v1896))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1896*(self.scalar_static_f64[321]+(self.scalar_static_f64[818]*(if common.v811{(common.v812*self.scalar_static_f64[898])}else{(if common.v808{(v809*self.scalar_static_f64[898])}else{common.v4})}))))-(v1899*v5220))/v5229))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1899*v5221))/v5229))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((-(v1899*v5222))/v5229)))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-v2154)))),
            [4, 6, 7, 8],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v2153{v5992}else{(if v2147{(((v2149*((v2144*v5930)+(v2122*v5992)))-(v2148*(v5930+v5986)))/v6025)}else{(if v2123{v5992}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v2153{v5995}else{(if v2147{(((v2149*((v2144*v5931)+(v2122*v5995)))-(v2148*(v5931+v5987)))/v6025)}else{(if v2123{v5995}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v2153{v5998}else{(if v2147{(((v2149*((v2144*v5932)+(v2122*v5998)))-(v2148*(v5932+v5988)))/v6025)}else{(if v2123{v5998}else{common.v4})})})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(-(if v2153{v6001}else{(if v2147{(((v2149*((v2144*v5933)+(v2122*v6001)))-(v2148*(v5933+v5989)))/v6025)}else{(if v2123{v6001}else{common.v4})})}))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(2),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*((self.scalar_static_f64[0]*(self.scalar_static_f64[0]*(common.v747-common.v734)))/self.scalar_static_f64[561]))),
            2,
            multiplicity * (self.scalar_static_f64[966]),
            4,
            multiplicity * (self.scalar_static_f64[967]),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * ((self.scalar_static_f64[13]*((self.scalar_static_f64[0]*common.v752)/self.scalar_static_f64[568]))),
            1,
            multiplicity * (self.scalar_static_f64[970]),
            5,
            multiplicity * (self.scalar_static_f64[971]),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2541)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(common.v7143*v7149)), (self.scalar_static_f64[13]*(common.v7144*v7149)), (self.scalar_static_f64[13]*(common.v7145*v7149)), (self.scalar_static_f64[13]*(common.v7146*v7149)), (self.scalar_static_f64[13]*(common.v7147*v7149)), (self.scalar_static_f64[13]*(common.v7148*v7149))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * ((self.scalar_static_f64[13]*v2544)),
            4,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7162))),
            5,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7163))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * ((self.scalar_static_f64[13]*v2547)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v7149*common.v7168)), (self.scalar_static_f64[13]*(v7149*common.v7169)), (self.scalar_static_f64[13]*(v7149*common.v7170)), (self.scalar_static_f64[13]*(v7149*common.v7171)), (self.scalar_static_f64[13]*(v7149*common.v7172)), (self.scalar_static_f64[13]*(v7149*common.v7173))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * ((self.scalar_static_f64[13]*v2550)),
            3,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7186))),
            7,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7187))),
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * ((self.scalar_static_f64[13]*v2553)),
            [4, 5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v7149*common.v7192)), (self.scalar_static_f64[13]*(v7149*common.v7193)), (self.scalar_static_f64[13]*(v7149*common.v7194)), (self.scalar_static_f64[13]*(v7149*common.v7195)), (self.scalar_static_f64[13]*(v7149*common.v7196)), (self.scalar_static_f64[13]*(v7149*common.v7197))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * ((self.scalar_static_f64[13]*v2557)),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[373]))),
            2,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[374]))),
        );
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * ((self.scalar_static_f64[13]*v2561)),
            0,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[375]))),
            1,
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[376]))),
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1823*v1876)))),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v5117+(v1823*v5057)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{((common.v1821*common.v4592)+(common.v1758*common.v4839))}else{common.v4}))+(v1823*v5058)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v1876*(if self.scalar_static_bool[44]{(common.v1758*common.v4840)}else{common.v4})))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{(common.v1758*common.v4841)}else{common.v4}))+(v1823*v5059)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(v5117+(v1823*v5060)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{(v4847+(common.v1758*common.v4842))}else{common.v4}))+(v1823*v5061)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{(v4857+(common.v1758*common.v4843))}else{common.v4}))+(v1823*v5062)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{(v4857+(common.v1758*common.v4844))}else{common.v4}))+(v1823*v5063)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{((common.v1821*common.v4594)+(common.v1758*common.v4845))}else{common.v4}))+(v1823*v5064)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1876*(if self.scalar_static_bool[44]{(v4857+(common.v1758*common.v4846))}else{common.v4}))+(v1823*v5065))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[809]*(self.scalar_static_f64[0]*common.v770)))),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [self.scalar_static_f64[976], self.scalar_static_f64[977], self.scalar_static_f64[977], self.scalar_static_f64[977], self.scalar_static_f64[978], self.scalar_static_f64[978], self.scalar_static_f64[979], self.scalar_static_f64[978]],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * ((self.scalar_static_f64[13]*v2569)),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [v7270, (self.scalar_static_f64[13]*(v7149*common.v7253)), (self.scalar_static_f64[13]*(v7149*common.v7254)), (self.scalar_static_f64[13]*(v7149*common.v7255)), v7270, (self.scalar_static_f64[13]*(v7149*common.v7256)), (self.scalar_static_f64[13]*(v7149*common.v7257)), (self.scalar_static_f64[13]*(v7149*common.v7258)), (self.scalar_static_f64[13]*(v7149*common.v7259)), (self.scalar_static_f64[13]*(v7149*common.v7260))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1746*v1876)+((v1467*v1876)+(common.v4*common.v766)))))),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [(self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1746*v5057)+(v1467*v5057)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1746*v5058)+(v1467*v5058)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1746*v5059)+((v1876*(self.scalar_static_f64[664]*common.v3934))+(v1467*v5059))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1876*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4366)}else{v4366}))+(v1746*v5060))+(self.scalar_static_f64[346]+((v1876*(self.scalar_static_f64[664]*common.v3935))+(v1467*v5060)))))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1876*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4370)}else{v4370}))+(v1746*v5061))+(((v1876*(self.scalar_static_f64[664]*common.v3936))+(v1467*v5061))+self.scalar_static_f64[369])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v5088+(v1746*v5062))+((v5108+(v1467*v5062))+self.scalar_static_f64[370])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v5088+(v1746*v5063))+((v5108+(v1467*v5063))+self.scalar_static_f64[370])))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*((v1746*v5064)+(v1467*v5064)))), (self.scalar_static_f64[13]*(self.scalar_static_f64[0]*(((v1876*(if self.scalar_static_bool[44]{(self.scalar_static_f64[6]*v4378)}else{v4378}))+(v1746*v5065))+(self.scalar_static_f64[347]+((v1876*(self.scalar_static_f64[664]*common.v3938))+(v1467*v5065))))))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * ((self.scalar_static_f64[13]*v2575)),
            [5, 6, 7, 8, 10],
            [(self.scalar_static_f64[13]*(v7149*common.v7302)), (self.scalar_static_f64[13]*(v7149*common.v7303)), v7312, v7312, (self.scalar_static_f64[13]*(v7149*common.v7305))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * ((if self.scalar_static_bool[15]{(self.scalar_static_f64[13]*(self.scalar_static_f64[813]*(self.scalar_static_f64[0]*common.v763)))}else{common.v4})),
            9,
            multiplicity * (self.scalar_static_f64[984]),
            10,
            multiplicity * (self.scalar_static_f64[985]),
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
        stamper.stamp_current_node2_local(
            Some(10),
            Some(7),
            multiplicity * ((if self.scalar_static_bool[17]{(self.scalar_static_f64[13]*(self.scalar_static_f64[817]*(self.scalar_static_f64[0]*common.v760)))}else{common.v4})),
            7,
            multiplicity * (self.scalar_static_f64[990]),
            10,
            multiplicity * (self.scalar_static_f64[991]),
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
            multiplicity * (common.v2585),
            11,
            multiplicity * (common.v1),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * ((common.v2489*v2586)),
            [4, 5, 6, 7, 8, 10, 11],
            [(v2586*common.v6932), (v2586*common.v6933), (v2586*common.v6934), (v2586*common.v6935), (v2586*common.v6936), (v2586*common.v6937), (common.v2489*v7149)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * ((v2468*common.v2585)),
            11,
            multiplicity * (v2468),
        );
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (common.v2585),
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
            Some(6),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (common.v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
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
        let v2541=0.0;
        let v2544=0.0;
        let v2547=0.0;
        let v2550=0.0;
        let v2553=0.0;
        let v2557=0.0;
        let v2561=0.0;
        let v2569=0.0;
        let v2575=0.0;
        let v2586=0.0;
        let v7149=1.0;
        let v7270=(self.scalar_static_f64[13]*(v7149*common.v7252));
        let v7312=(self.scalar_static_f64[13]*(v7149*common.v7304));

        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(common.v7143*v7149)), (self.scalar_static_f64[13]*(common.v7144*v7149)), (self.scalar_static_f64[13]*(common.v7145*v7149)), (self.scalar_static_f64[13]*(common.v7146*v7149)), (self.scalar_static_f64[13]*(common.v7147*v7149)), (self.scalar_static_f64[13]*(common.v7148*v7149))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7162))),
            nodes[5],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7163))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v7149*common.v7168)), (self.scalar_static_f64[13]*(v7149*common.v7169)), (self.scalar_static_f64[13]*(v7149*common.v7170)), (self.scalar_static_f64[13]*(v7149*common.v7171)), (self.scalar_static_f64[13]*(v7149*common.v7172)), (self.scalar_static_f64[13]*(v7149*common.v7173))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes[3],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7186))),
            nodes[7],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*common.v7187))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v7149*common.v7192)), (self.scalar_static_f64[13]*(v7149*common.v7193)), (self.scalar_static_f64[13]*(v7149*common.v7194)), (self.scalar_static_f64[13]*(v7149*common.v7195)), (self.scalar_static_f64[13]*(v7149*common.v7196)), (self.scalar_static_f64[13]*(v7149*common.v7197))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[373]))),
            nodes[2],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[374]))),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[375]))),
            nodes[1],
            multiplicity * ((self.scalar_static_f64[13]*(v7149*self.scalar_static_f64[376]))),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &[nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[v7270, (self.scalar_static_f64[13]*(v7149*common.v7253)), (self.scalar_static_f64[13]*(v7149*common.v7254)), (self.scalar_static_f64[13]*(v7149*common.v7255)), v7270, (self.scalar_static_f64[13]*(v7149*common.v7256)), (self.scalar_static_f64[13]*(v7149*common.v7257)), (self.scalar_static_f64[13]*(v7149*common.v7258)), (self.scalar_static_f64[13]*(v7149*common.v7259)), (self.scalar_static_f64[13]*(v7149*common.v7260))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &[nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]],
            &[(self.scalar_static_f64[13]*(v7149*common.v7302)), (self.scalar_static_f64[13]*(v7149*common.v7303)), v7312, v7312, (self.scalar_static_f64[13]*(v7149*common.v7305))],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &[nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]],
            &[(v2586*common.v6932), (v2586*common.v6933), (v2586*common.v6934), (v2586*common.v6935), (v2586*common.v6936), (v2586*common.v6937), (common.v2489*v7149)],
            &[],
            &[],
            multiplicity,
        );
    }
}
