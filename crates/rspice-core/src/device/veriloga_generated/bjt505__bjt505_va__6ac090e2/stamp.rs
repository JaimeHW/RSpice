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

impl Instance {
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
        let v1: f64 = 1.0;
        let v4: f64 = 0.0;
        let v31: f64 = 0.001;
        let v32: f64 = 2.0;
        let v47: f64 = 0.1;
        let v155: f64 = 3.0;
        let v421: f64 = 1e-6;
        let v424: f64 = 0.5;
        let v436: f64 = 4.0;
        let v462: f64 = 6.0;
        let v727: f64 = ctx.node_voltage(nodes[6]);
        let v728: f64 = ctx.node_voltage(nodes[7]);
        let v730: f64 = (self.scalar_v0 * (v727 - v728));
        let v731: f64 = ctx.node_voltage(nodes[8]);
        let v733: f64 = (self.scalar_v0 * (v727 - v731));
        let v734: f64 = ctx.node_voltage(nodes[4]);
        let v736: f64 = (self.scalar_v0 * (v727 - v734));
        let v737: f64 = ctx.node_voltage(nodes[5]);
        let v739: f64 = (self.scalar_v0 * (v737 - v734));
        let v741: f64 = (self.scalar_v0 * (v737 - v727));
        let v744: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[3]) - v728));
        let v746: f64 = (self.scalar_v0 * (v728 - v731));
        let v747: f64 = ctx.node_voltage(nodes[2]);
        let v750: f64 = ctx.node_voltage(nodes[1]);
        let v752: f64 = (self.scalar_v0 * (v750 - v737));
        let v757: f64 = (self.scalar_v0 * (v750 - ctx.node_voltage(nodes[0])));
        let v758: f64 = ctx.node_voltage(nodes[10]);
        let v760: f64 = (self.scalar_v0 * (v758 - v728));
        let v763: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[9]) - v758));
        let v766: f64 = (((v733 + v741) - v746) - v760);
        let v770: f64 = ((v766 + (v752 + (-v757))) - v763);
        let v771: f64 = (v757 + v770);
        let v772: f64 = (v744 - v760);
        let v774: f64 = (self.scalar_v106 * v733);
        let v776: bool = (v774 < self.scalar_v775);
        let v777: f64 = ((v774) as f64).exp();
        let v779: bool = (!v776);
        let v781: f64 = (if v779 { self.scalar_v780 } else { v4 });
        let v785: f64 = (if v779 { (v781 * (v1 + (v774 - self.scalar_v775))) } else { (if v776 { v777 } else { v4 }) });
        let v786: f64 = (self.scalar_v106 * v736);
        let v787: f64 = (v786 / self.scalar_v384);
        let v788: bool = (v787 < self.scalar_v775);
        let v789: f64 = ((v787) as f64).exp();
        let v791: bool = (!v788);
        let v792: f64 = (if v791 { self.scalar_v780 } else { v781 });
        let v796: f64 = (if v791 { (v792 * (v1 + (v787 - self.scalar_v775))) } else { (if v788 { v789 } else { v4 }) });
        let v797: f64 = (self.scalar_v106 * v766);
        let v798: bool = (v797 < self.scalar_v775);
        let v799: f64 = ((v797) as f64).exp();
        let v801: bool = (!v798);
        let v802: f64 = (if v801 { self.scalar_v780 } else { v792 });
        let v806: f64 = (if v801 { (v802 * (v1 + (v797 - self.scalar_v775))) } else { (if v798 { v799 } else { v4 }) });
        let v807: f64 = (self.scalar_v106 * v741);
        let v808: bool = (v807 < self.scalar_v775);
        let v809: f64 = ((v807) as f64).exp();
        let v811: bool = (!v808);
        let v812: f64 = (if v811 { self.scalar_v780 } else { v802 });
        let v817: f64 = (self.scalar_v106 * v771);
        let v818: bool = (v817 < self.scalar_v775);
        let v819: f64 = ((v817) as f64).exp();
        let v821: bool = (!v818);
        let v822: f64 = (if v821 { self.scalar_v780 } else { v812 });
        let v826: f64 = (if v821 { (v822 * (v1 + (v817 - self.scalar_v775))) } else { (if v818 { v819 } else { v4 }) });
        let v827: f64 = (self.scalar_v106 * v744);
        let v828: bool = (v827 < self.scalar_v775);
        let v829: f64 = ((v827) as f64).exp();
        let v831: bool = (!v828);
        let v832: f64 = (if v831 { self.scalar_v780 } else { v822 });
        let v836: f64 = (if v831 { (v832 * (v1 + (v827 - self.scalar_v775))) } else { (if v828 { v829 } else { v4 }) });
        let v837: f64 = (self.scalar_v106 * (v772 - v763));
        let v838: bool = (v837 < self.scalar_v775);
        let v839: f64 = ((v837) as f64).exp();
        let v841: bool = (!v838);
        let v842: f64 = (if v841 { self.scalar_v780 } else { v832 });
        let v846: f64 = (if v841 { (v842 * (v1 + (v837 - self.scalar_v775))) } else { (if v838 { v839 } else { v4 }) });
        let v847: f64 = (self.scalar_v106 * v772);
        let v848: bool = (v847 < self.scalar_v775);
        let v849: f64 = ((v847) as f64).exp();
        let v851: bool = (!v848);
        let v852: f64 = (if v851 { self.scalar_v780 } else { v842 });
        let v856: f64 = (if v851 { (v852 * (v1 + (v847 - self.scalar_v775))) } else { (if v848 { v849 } else { v4 }) });
        let v858: f64 = (self.scalar_v106 * (v771 - self.scalar_v204));
        let v859: bool = (v858 < self.scalar_v775);
        let v860: f64 = ((v858) as f64).exp();
        let v862: bool = (!v859);
        let v863: f64 = (if v862 { self.scalar_v780 } else { v852 });
        let v869: f64 = (self.scalar_v106 * (v766 - self.scalar_v204));
        let v870: bool = (v869 < self.scalar_v775);
        let v871: f64 = ((v869) as f64).exp();
        let v873: bool = (!v870);
        let v874: f64 = (if v873 { self.scalar_v780 } else { v863 });
        let v880: f64 = (self.scalar_v106 * (v733 - self.scalar_v204));
        let v881: bool = (v880 < self.scalar_v775);
        let v882: f64 = ((v880) as f64).exp();
        let v884: bool = (!v881);
        let v885: f64 = (if v884 { self.scalar_v780 } else { v874 });
        let v889: f64 = (if v884 { (v885 * (v1 + (v880 - self.scalar_v775))) } else { (if v881 { v882 } else { v4 }) });
        let v891: f64 = (self.scalar_v106 * (v730 - self.scalar_v204));
        let v892: bool = (v891 < self.scalar_v775);
        let v893: f64 = ((v891) as f64).exp();
        let v895: bool = (!v892);
        let v896: f64 = (if v895 { self.scalar_v780 } else { v885 });
        let v900: f64 = (if v895 { (v896 * (v1 + (v891 - self.scalar_v775))) } else { (if v892 { v893 } else { v4 }) });
        let v903: f64 = (((v1 + (v436 * v889))) as f64).sqrt();
        let v906: f64 = (((v1 + (v436 * v900))) as f64).sqrt();
        let v907: f64 = (v32 * v900);
        let v908: f64 = (v1 + v906);
        let v909: f64 = (v907 / v908);
        let v911: bool = (v909 < self.scalar_v910);
        let v912: f64 = (if v911 { self.scalar_v910 } else { v909 });
        let v914: f64 = (v1 + v903);
        let v915: f64 = (v914 / v908);
        let v918: f64 = (self.scalar_v104 * ((v903 - v906) - ((v915) as f64).ln()));
        let v920: f64 = ((v746 + v918) / self.scalar_v352);
        let v921: bool = (v920 > v4);
        let v922: f64 = 100.0;
        let v923: bool = (v730 < v922);
        let v924: bool = (v921 && v923);
        let v927: bool = (v921 && (!v923));
        let v929: f64 = (v1 + (v730 - v922));
        let v935: f64 = (self.scalar_v352 * (v424 * v920));
        let v937: f64 = (v1 + (self.scalar_v106 * v935));
        let v942: f64 = (if v921 { ((self.scalar_v204 + (self.scalar_v933 * ((v937) as f64).ln())) - (if v927 { (v922 + ((v929) as f64).ln()) } else { (if v924 { v730 } else { v4 }) })) } else { v4 });
        let v945: f64 = (if v921 { self.scalar_v944 } else { v4 });
        let v947: f64 = (if v921 { (v945 * v945) } else { v421 });
        let v950: bool = (v942 < v4);
        let v951: bool = (v921 && v950);
        let v952: f64 = (v424 * v947);
        let v954: f64 = (((v947 + (if v921 { (v942 * v942) } else { self.scalar_v422 }))) as f64).sqrt();
        let v955: f64 = (v954 - v942);
        let v959: bool = (v921 && (!v950));
        let v962: f64 = (if v959 { (v424 * (v942 + v954)) } else { (if v951 { (v952 / v955) } else { v4 }) });
        let v966: f64 = (v962 + self.scalar_v965);
        let v967: f64 = (v962 * v966);
        let v970: f64 = (self.scalar_v964 * (v962 + self.scalar_v968));
        let v972: f64 = (if v921 { (v967 / v970) } else { v4 });
        let v974: f64 = (if v921 { (v920 / v972) } else { v4 });
        let v978: f64 = (if v921 { ((v974 - v1) / self.scalar_v976) } else { self.scalar_v394 });
        let v979: bool = (v974 < v1);
        let v980: bool = (v921 && v979);
        let v981: f64 = ((v978) as f64).exp();
        let v982: f64 = (v1 + v981);
        let v988: bool = (v921 && (!v979));
        let v990: f64 = (((-v978)) as f64).exp();
        let v991: f64 = (v1 + v990);
        let v1004: f64 = (if v921 { ((if v988 { (v974 + (self.scalar_v976 * ((v991) as f64).ln())) } else { (if v980 { (v1 + (self.scalar_v976 * ((v982) as f64).ln())) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v1006: f64 = (if v921 { (v962 / self.scalar_v965) } else { v4 });
        let v1007: f64 = (v436 * v1004);
        let v1008: f64 = (v1006 * v1007);
        let v1009: f64 = (v1 + v1006);
        let v1012: f64 = (((v1 + (v1008 * v1009))) as f64).sqrt();
        let v1013: f64 = (v1 + v1012);
        let v1014: f64 = (v32 * v1004);
        let v1015: f64 = (v1009 * v1014);
        let v1017: f64 = (if v921 { (v1013 / v1015) } else { v4 });
        let v1019: f64 = (v912 * v1017);
        let v1020: f64 = ((v1 - v1017) + v1019);
        let v1021: f64 = (v1 + v1019);
        let v1023: f64 = (if v921 { (v1020 / v1021) } else { v4 });
        let v1026: f64 = (if v921 { (self.scalar_v106 * (v935 * v1023)) } else { v4 });
        let v1029: f64 = (v1 + (v912 + v1026));
        let v1032: f64 = (if v921 { ((v32 * v1026) + (v912 * v1029)) } else { v4 });
        let v1035: f64 = (if v921 { (v424 * (v1026 - v1)) } else { v4 });
        let v1038: f64 = (if v921 { (v1032 + (v1035 * v1035)) } else { v4 });
        let v1039: bool = (v1026 >= v1);
        let v1040: bool = (v921 && v1039);
        let v1041: f64 = ((v1038) as f64).sqrt();
        let v1045: bool = (v921 && (!v1039));
        let v1046: f64 = (v1041 - v1035);
        let v1048: f64 = (if v1045 { (v1032 / v1046) } else { (if v1040 { (v1035 + v1041) } else { v4 }) });
        let v1051: bool = (v921 && (v1048 < self.scalar_v1049));
        let v1052: f64 = (if v1051 { self.scalar_v1049 } else { v1048 });
        let v1053: f64 = (v1 + v1052);
        let v1062: f64 = (if v921 { (self.scalar_v1059 * (v920 - self.scalar_v963)) } else { v4 });
        let v1069: f64 = ((((if v921 { (v920 * self.scalar_v1064) } else { v4 }) + (v1062 * v1062))) as f64).sqrt();
        let v1078: bool = (v921 && self.scalar_v1077);
        let v1079: f64 = (v32 * v920);
        let v1080: f64 = (v920 + v972);
        let v1085: f64 = (v920 * self.scalar_v963);
        let v1086: f64 = (v920 + self.scalar_v963);
        let v1091: bool = (!v921);
        let v1092: f64 = (v32 * v889);
        let v1095: f64 = (if v1091 { v785 } else { (if v921 { ((v1052 * v1053) * self.scalar_v1056) } else { v4 }) });
        let v1106: bool = ((((v746) as f64).abs() < self.scalar_v1098) || (((v918) as f64).abs() < (self.scalar_v1102 * (v903 + v906))));
        let v1107: bool = (v1091 && v1106);
        let v1108: f64 = (v912 + (if v1091 { (v1092 / v914) } else { v1052 }));
        let v1110: f64 = (if v1107 { (v424 * v1108) } else { v4 });
        let v1111: f64 = (v1 + v1110);
        let v1115: bool = (v1091 && (!v1106));
        let v1117: f64 = ((v733 + v918) - v730);
        let v1119: f64 = (if v1115 { (v918 / v1117) } else { (if v1107 { (v1110 / v1111) } else { v1023 }) });
        let v1121: f64 = (if v1091 { self.scalar_v1075 } else { (if v1078 { (self.scalar_v245 * (v47 + (v1079 / v1080))) } else { (if (v921 && self.scalar_v1073) { self.scalar_v1075 } else { v4 }) }) });
        let v1122: f64 = (if v1091 { v920 } else { (if v921 { (v1085 / v1086) } else { v4 }) });
        let v1125: f64 = (if v1091 { (v1 - (v1122 / self.scalar_v963)) } else { (if v921 { (self.scalar_v963 / v1086) } else { v4 }) });
        let v1132: f64 = ((v736 - self.scalar_v1129) / self.scalar_v1130);
        let v1133: bool = (v736 < self.scalar_v1129);
        let v1134: f64 = ((v1132) as f64).exp();
        let v1135: f64 = (v1 + v1134);
        let v1140: bool = (!v1133);
        let v1142: f64 = (((-v1132)) as f64).exp();
        let v1143: f64 = (v1 + v1142);
        let v1147: f64 = (if v1140 { (self.scalar_v1129 - (self.scalar_v1130 * ((v1143) as f64).ln())) } else { (if v1133 { (v736 - (self.scalar_v1130 * ((v1135) as f64).ln())) } else { v4 }) });
        let v1149: f64 = (v1 - (self.scalar_v292 * v1147));
        let v1151: f64 = f64::powf(v1149, self.scalar_v1150);
        let v1157: f64 = ((self.scalar_v1152 * (v1 - v1151)) + (v155 * (v736 - v1147)));
        let v1168: f64 = (if self.scalar_v1167 { v733 } else { (if self.scalar_v1163 { (v730 + (if v1091 { v746 } else { (if v921 { (v1062 + v1069) } else { v4 }) })) } else { (if self.scalar_v1159 { v730 } else { v4 }) }) });
        let v1176: f64 = (v1168 - self.scalar_v1175);
        let v1177: f64 = (v1176 / v1121);
        let v1178: bool = (v1168 < self.scalar_v1175);
        let v1179: f64 = ((v1177) as f64).exp();
        let v1180: f64 = (v1 + v1179);
        let v1181: f64 = ((v1180) as f64).ln();
        let v1185: bool = (!v1178);
        let v1187: f64 = (((-v1177)) as f64).exp();
        let v1188: f64 = (v1 + v1187);
        let v1189: f64 = ((v1188) as f64).ln();
        let v1192: f64 = (if v1185 { (self.scalar_v1175 - (v1121 * v1189)) } else { (if v1178 { (v1168 - (v1121 * v1181)) } else { v4 }) });
        let v1194: f64 = f64::powf(v1125, self.scalar_v1193);
        let v1198: f64 = (v1 - (v1192 / self.scalar_v245));
        let v1199: f64 = f64::powf(v1198, self.scalar_v1195);
        let v1203: f64 = (self.scalar_v1171 * v1194);
        let v1204: f64 = (v1168 - v1192);
        let v1209: f64 = ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - (v1194 * v1199))) + (v1203 * v1204))) + (self.scalar_v314 * v730));
        let v1212: f64 = (v796 * self.scalar_v1211);
        let v1214: f64 = (((v1 + v1212)) as f64).sqrt();
        let v1215: f64 = (v1 + v1214);
        let v1216: f64 = (v1212 / v1215);
        let v1218: f64 = f64::powf(v1095, self.scalar_v1217);
        let v1219: f64 = (self.scalar_v1211 * v1218);
        let v1221: f64 = (((v1 + v1219)) as f64).sqrt();
        let v1222: f64 = (v1 + v1221);
        let v1223: f64 = (v1219 / v1222);
        let v1226: f64 = (v1 + (v1157 / self.scalar_v623));
        let v1227: f64 = (v1209 / self.scalar_v620);
        let v1228: f64 = (v1226 + v1227);
        let v1239: f64 = (((if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v1226)) } else { v4 })) as f64).exp();
        let v1240: f64 = (((if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v1209) / self.scalar_v620))) } else { v4 })) as f64).exp();
        let v1246: f64 = (if self.scalar_v1230 { ((v1239 - v1240) / self.scalar_v1244) } else { (if self.scalar_v1224 { v1228 } else { v4 }) });
        let v1247: f64 = 0.010000000000000002;
        let v1248: f64 = (v1246 * v1246);
        let v1249: bool = (v1246 < v4);
        let v1250: f64 = 0.005000000000000001;
        let v1252: f64 = (((v1247 + v1248)) as f64).sqrt();
        let v1253: f64 = (v1252 - v1246);
        let v1256: bool = (!v1249);
        let v1259: f64 = (if v1256 { (v424 * (v1246 + v1252)) } else { (if v1249 { (v1250 / v1253) } else { v4 }) });
        let v1262: f64 = (v1 + (v424 * (v1216 + v1223)));
        let v1263: f64 = (v1259 * v1262);
        let v1266: f64 = (v1218 * self.scalar_v1265);
        let v1267: f64 = (self.scalar_v449 * v796);
        let v1268: f64 = (v1267 - v1266);
        let v1269: f64 = (v1268 / v1263);
        let v1270: f64 = 0.0001;
        let v1271: f64 = (v736 / v1270);
        let v1272: bool = (v736 < v4);
        let v1273: f64 = ((v1271) as f64).exp();
        let v1274: f64 = (v1 + v1273);
        let v1278: bool = (!v1272);
        let v1280: f64 = (((-v1271)) as f64).exp();
        let v1281: f64 = (v1 + v1280);
        let v1285: f64 = (if v1278 { (v736 + (v1270 * ((v1281) as f64).ln())) } else { (if v1272 { (v1270 * ((v1274) as f64).ln()) } else { v4 }) });
        let v1287: f64 = (v1285 / self.scalar_v1286);
        let v1288: bool = (v1287 < self.scalar_v775);
        let v1289: f64 = ((v1287) as f64).exp();
        let v1291: bool = (!v1288);
        let v1292: f64 = (if v1291 { self.scalar_v780 } else { v896 });
        let v1296: f64 = (if v1291 { (v1292 * (v1 + (v1287 - self.scalar_v775))) } else { (if v1288 { v1289 } else { v4 }) });
        let v1301: f64 = ((v736 - self.scalar_v1299) / v31);
        let v1302: bool = (v736 < self.scalar_v1299);
        let v1303: f64 = ((v1301) as f64).exp();
        let v1304: f64 = (v1 + v1303);
        let v1309: bool = (!v1302);
        let v1311: f64 = (((-v1301)) as f64).exp();
        let v1312: f64 = (v1 + v1311);
        let v1316: f64 = (if v1309 { (self.scalar_v1299 - (v31 * ((v1312) as f64).ln())) } else { (if v1302 { (v736 - (v31 * ((v1304) as f64).ln())) } else { v4 }) });
        let v1318: f64 = (v1316 * self.scalar_v1317);
        let v1319: f64 = (self.scalar_v1299 - v1316);
        let v1320: f64 = f64::powf(v1319, v32);
        let v1322: f64 = (v786 / self.scalar_v491);
        let v1323: bool = (v1322 < self.scalar_v775);
        let v1324: f64 = ((v1322) as f64).exp();
        let v1326: bool = (!v1323);
        let v1327: f64 = (if v1326 { self.scalar_v780 } else { v1292 });
        let v1331: f64 = (if v1326 { (v1327 * (v1 + (v1322 - self.scalar_v775))) } else { (if v1323 { v1324 } else { v1285 }) });
        let v1333: f64 = (self.scalar_v106 * (v736 - self.scalar_v268));
        let v1334: bool = (v1333 < self.scalar_v775);
        let v1335: bool = (self.scalar_v510 && v1334);
        let v1336: f64 = ((v1333) as f64).exp();
        let v1339: bool = (self.scalar_v510 && (!v1334));
        let v1340: f64 = (if v1339 { self.scalar_v780 } else { v1327 });
        let v1344: f64 = (if v1339 { (v1340 * (v1 + (v1333 - self.scalar_v775))) } else { (if v1335 { v1336 } else { v1287 }) });
        let v1347: f64 = ((v1269 / self.scalar_v449) - 1000.0);
        let v1348: f64 = 40.0;
        let v1349: bool = (v1347 < v1348);
        let v1350: bool = (self.scalar_v510 && v1349);
        let v1351: f64 = ((v1347) as f64).exp();
        let v1354: bool = (self.scalar_v510 && (!v1349));
        let v1356: f64 = (if v1354 { 2.3538526683702e17 } else { v1340 });
        let v1360: f64 = (if v1354 { (v1356 * (v1 + (v1347 - v1348))) } else { (if v1350 { v1351 } else { v1296 }) });
        let v1361: f64 = (v1331 - v1);
        let v1362: f64 = (self.scalar_v500 * v1361);
        let v1364: f64 = (v1361 * self.scalar_v1363);
        let v1367: f64 = (((v1 + (v436 * v1344))) as f64).sqrt();
        let v1368: f64 = (v1 + v1367);
        let v1369: f64 = (v1364 / v1368);
        let v1370: f64 = (v1 + v1227);
        let v1374: f64 = (self.scalar_v525 * (v1095 - v1));
        let v1375: f64 = (v1360 * v1374);
        let v1376: f64 = (v1 + v1360);
        let v1391: f64 = (self.scalar_v1380 * ((v1095 + v1331) - v32));
        let v1395: f64 = (if self.scalar_v1386 { (self.scalar_v500 * ((v1361 * self.scalar_v1387) + (v1370 * v1391))) } else { (if self.scalar_v1383 { v1362 } else { (if self.scalar_v510 { ((v1362 + (v1369 * v1370)) + (v1375 / v1376)) } else { v4 }) }) });
        let v1396: f64 = (self.scalar_v106 * v739);
        let v1397: f64 = (v1396 / self.scalar_v502);
        let v1398: bool = (v1397 < self.scalar_v775);
        let v1399: f64 = ((v1397) as f64).exp();
        let v1401: bool = (!v1398);
        let v1402: f64 = (if v1401 { self.scalar_v780 } else { v1356 });
        let v1406: f64 = (if v1401 { (v1402 * (v1 + (v1397 - self.scalar_v775))) } else { (if v1398 { v1399 } else { v1331 }) });
        let v1408: f64 = (self.scalar_v106 * (v739 - self.scalar_v268));
        let v1409: bool = (v1408 < self.scalar_v775);
        let v1410: bool = (self.scalar_v510 && v1409);
        let v1411: f64 = ((v1408) as f64).exp();
        let v1414: bool = (self.scalar_v510 && (!v1409));
        let v1415: f64 = (if v1414 { self.scalar_v780 } else { v1402 });
        let v1420: f64 = (v1406 - v1);
        let v1421: f64 = (self.scalar_v508 * v1420);
        let v1423: f64 = (v1420 * self.scalar_v1422);
        let v1426: f64 = (((v1 + (v436 * (if v1414 { (v1415 * (v1 + (v1408 - self.scalar_v775))) } else { (if v1410 { v1411 } else { v1344 }) })))) as f64).sqrt();
        let v1427: f64 = (v1 + v1426);
        let v1432: f64 = (v786 / self.scalar_v463);
        let v1433: bool = (v1432 < self.scalar_v775);
        let v1434: f64 = ((v1432) as f64).exp();
        let v1436: bool = (!v1433);
        let v1437: f64 = (if v1436 { self.scalar_v780 } else { v1415 });
        let v1441: f64 = (if v1436 { (v1437 * (v1 + (v1432 - self.scalar_v775))) } else { (if v1433 { v1434 } else { v1406 }) });
        let v1444: f64 = (v1396 / self.scalar_v546);
        let v1445: bool = (v1444 < self.scalar_v775);
        let v1446: f64 = ((v1444) as f64).exp();
        let v1448: bool = (!v1445);
        let v1449: f64 = (if v1448 { self.scalar_v780 } else { v1437 });
        let v1453: f64 = (if v1448 { (v1449 * (v1 + (v1444 - self.scalar_v775))) } else { (if v1445 { v1446 } else { v1441 }) });
        let v1456: f64 = (v797 / self.scalar_v476);
        let v1457: bool = (v1456 < self.scalar_v775);
        let v1458: f64 = ((v1456) as f64).exp();
        let v1460: bool = (!v1457);
        let v1461: f64 = (if v1460 { self.scalar_v780 } else { v1449 });
        let v1465: f64 = (if v1460 { (v1461 * (v1 + (v1456 - self.scalar_v775))) } else { (if v1457 { v1458 } else { v1453 }) });
        let v1467: f64 = (self.scalar_v486 * (v1465 - v1));
        let v1468: f64 = (v1396 / self.scalar_v556);
        let v1469: bool = (v1468 < self.scalar_v775);
        let v1470: f64 = ((v1468) as f64).exp();
        let v1472: bool = (!v1469);
        let v1473: f64 = (if v1472 { self.scalar_v780 } else { v1461 });
        let v1477: f64 = (if v1472 { (v1473 * (v1 + (v1468 - self.scalar_v775))) } else { (if v1469 { v1470 } else { v1465 }) });
        let v1483: bool = (v1272 && self.scalar_v1482);
        let v1484: f64 = (v32 * v1151);
        let v1487: f64 = (self.scalar_v583 * (v1 - (self.scalar_v35 / v1484)));
        let v1488: bool = (v1487 < self.scalar_v775);
        let v1489: bool = (v1483 && v1488);
        let v1490: f64 = ((v1487) as f64).exp();
        let v1493: bool = (v1483 && (!v1488));
        let v1494: f64 = (if v1493 { self.scalar_v780 } else { v1473 });
        let v1498: f64 = (if v1493 { (v1494 * (v1 + (v1487 - self.scalar_v775))) } else { (if v1489 { v1490 } else { v4 }) });
        let v1500: f64 = (if v1483 { (self.scalar_v292 * v736) } else { self.scalar_v617 });
        let v1502: f64 = 1e-30;
        let v1504: f64 = ((((v1500 * v1500) + v1502)) as f64).sqrt();
        let v1507: f64 = f64::powf(v1504, self.scalar_v1506);
        let v1515: f64 = (v462 * v1500);
        let v1516: f64 = (v1500 * v1515);
        let v1517: f64 = (v1500 + self.scalar_v1511);
        let v1519: f64 = ((self.scalar_v33 * (self.scalar_v1509 - ((v155 * v1500) * self.scalar_v1511))) - (v1516 * v1517));
        let v1521: f64 = 0.16666666666666666;
        let v1525: f64 = (self.scalar_v583 * (self.scalar_v35 * v736));
        let v1526: f64 = (self.scalar_v132 * (if v1483 { ((v1507 * v1519) * v1521) } else { v4 }));
        let v1528: f64 = (if v1483 { (v1525 / v1526) } else { v1500 });
        let v1529: f64 = -0.001;
        let v1530: bool = (v1528 < v1529);
        let v1531: bool = (v1528 < self.scalar_v775);
        let v1532: bool = (v1483 && v1530);
        let v1533: bool = (v1531 && v1532);
        let v1534: f64 = ((v1528) as f64).exp();
        let v1537: bool = (v1532 && (!v1531));
        let v1538: f64 = (if v1537 { self.scalar_v780 } else { v1494 });
        let v1543: f64 = (-v736);
        let v1544: f64 = (v1 - (if v1537 { (v1538 * (v1 + (v1528 - self.scalar_v775))) } else { (if v1533 { v1534 } else { v4 }) }));
        let v1546: f64 = (v1 + (v1544 / v1528));
        let v1550: bool = (v1483 && (!v1530));
        let v1551: f64 = (v424 * v736);
        let v1552: f64 = (v1528 * v1551);
        let v1553: f64 = 0.3333333333333333;
        let v1554: f64 = (v1528 * v1553);
        let v1555: f64 = 0.25;
        let v1557: f64 = (v1 + (v1528 * v1555));
        let v1559: f64 = (v1 + (v1554 * v1557));
        let v1563: f64 = ((if v1550 { (v1552 * v1559) } else { (if v1532 { (v1543 * v1546) } else { v4 }) }) * self.scalar_v1562);
        let v1564: f64 = (v1151 * v1563);
        let v1569: bool = (!v1483);
        let v1575: bool = (self.scalar_v1573 && (v730 < v4));
        let v1576: f64 = (self.scalar_v293 * v730);
        let v1577: f64 = (v1 - v1576);
        let v1579: f64 = (if v1575 { f64::powf(v1577, self.scalar_v1195) } else { v4 });
        let v1580: f64 = (v32 * v1579);
        let v1583: f64 = (self.scalar_v605 * (v1 - (self.scalar_v70 / v1580)));
        let v1584: bool = (v1583 < self.scalar_v775);
        let v1585: bool = (v1575 && v1584);
        let v1586: f64 = ((v1583) as f64).exp();
        let v1589: bool = (v1575 && (!v1584));
        let v1590: f64 = (if v1589 { self.scalar_v780 } else { v1538 });
        let v1594: f64 = (if v1589 { (v1590 * (v1 + (v1583 - self.scalar_v775))) } else { (if v1585 { v1586 } else { v4 }) });
        let v1595: f64 = (if v1575 { v1576 } else { self.scalar_v595 });
        let v1598: f64 = (((v1502 + (v1595 * v1595))) as f64).sqrt();
        let v1600: f64 = f64::powf(v1598, self.scalar_v1599);
        let v1608: f64 = (v462 * v1595);
        let v1609: f64 = (v1595 * v1608);
        let v1610: f64 = (v1595 + self.scalar_v1604);
        let v1612: f64 = ((self.scalar_v68 * (self.scalar_v1602 - ((v155 * v1595) * self.scalar_v1604))) - (v1609 * v1610));
        let v1617: f64 = (self.scalar_v605 * (self.scalar_v70 * v730));
        let v1618: f64 = (self.scalar_v154 * (if v1575 { (v1521 * (v1600 * v1612)) } else { v4 }));
        let v1620: f64 = (if v1575 { (v1617 / v1618) } else { v1595 });
        let v1621: bool = (v1620 < v1529);
        let v1622: bool = (v1620 < self.scalar_v775);
        let v1623: bool = (v1575 && v1621);
        let v1624: bool = (v1622 && v1623);
        let v1625: f64 = ((v1620) as f64).exp();
        let v1628: bool = (v1623 && (!v1622));
        let v1629: f64 = (if v1628 { self.scalar_v780 } else { v1590 });
        let v1634: f64 = (-v730);
        let v1635: f64 = (v1 - (if v1628 { (v1629 * (v1 + (v1620 - self.scalar_v775))) } else { (if v1624 { v1625 } else { v4 }) }));
        let v1637: f64 = (v1 + (v1635 / v1620));
        let v1641: bool = (v1575 && (!v1621));
        let v1642: f64 = (v424 * v730);
        let v1643: f64 = (v1620 * v1642);
        let v1644: f64 = (v1553 * v1620);
        let v1646: f64 = (v1 + (v1555 * v1620));
        let v1648: f64 = (v1 + (v1644 * v1646));
        let v1652: f64 = ((if v1641 { (v1643 * v1648) } else { (if v1623 { (v1634 * v1637) } else { v4 }) }) * self.scalar_v1651);
        let v1653: f64 = (v1579 * v1652);
        let v1658: bool = (!v1575);
        let v1659: f64 = (if v1658 { v4 } else { (if v1575 { (self.scalar_v71 * (self.scalar_v293 * (v1594 * v1653))) } else { v4 }) });
        let v1660: f64 = (v806 * self.scalar_v1211);
        let v1661: f64 = (v436 * (if v873 { (v874 * (v1 + (v869 - self.scalar_v775))) } else { (if v870 { v871 } else { v4 }) }));
        let v1662: f64 = (v1660 - self.scalar_v1211);
        let v1664: f64 = (((v1 + v1660)) as f64).sqrt();
        let v1665: f64 = (v1 + v1664);
        let v1668: f64 = (((v1 + v1661)) as f64).sqrt();
        let v1669: f64 = (v1 + v1668);
        let v1672: f64 = (v806 - v1);
        let v1673: f64 = (self.scalar_v1671 * v1672);
        let v1678: f64 = (((v1 + (v806 * self.scalar_v1675))) as f64).sqrt();
        let v1679: f64 = (v1 + v1678);
        let v1680: f64 = (v1673 / v1679);
        let v1687: f64 = (self.scalar_v1685 * (v785 - v836));
        let v1695: f64 = (((v1 + (self.scalar_v1689 * (v785 + (v836 * self.scalar_v1690))))) as f64).sqrt();
        let v1696: f64 = (v1 + v1695);
        let v1703: f64 = (self.scalar_v1701 * (v806 - v856));
        let v1708: f64 = (((v1 + (self.scalar_v1689 * (v806 + (v856 * self.scalar_v1690))))) as f64).sqrt();
        let v1709: f64 = (v1 + v1708);
        let v1714: f64 = (self.scalar_v1685 * (v785 - v1));
        let v1717: f64 = (((v1 + (v785 * self.scalar_v1689))) as f64).sqrt();
        let v1718: f64 = (v1 + v1717);
        let v1721: f64 = (v1672 * self.scalar_v1701);
        let v1724: f64 = (((v1 + (v806 * self.scalar_v1689))) as f64).sqrt();
        let v1725: f64 = (v1 + v1724);
        let v1727: f64 = (if self.scalar_v1712 { (v1721 / v1725) } else { (if self.scalar_v1682 { (v1703 / v1709) } else { v4 }) });
        let v1730: f64 = (self.scalar_v1728 * (v836 - v1));
        let v1736: f64 = (((v1 + (v836 * self.scalar_v1733))) as f64).sqrt();
        let v1737: f64 = (v1 + v1736);
        let v1746: f64 = (if self.scalar_v1744 { (self.scalar_v14 * v1680) } else { v1680 });
        let v1751: f64 = (v826 - v1);
        let v1752: f64 = (self.scalar_v1750 * v1751);
        let v1755: f64 = (((v1 + (v826 * self.scalar_v1675))) as f64).sqrt();
        let v1756: f64 = (v1 + v1755);
        let v1758: f64 = (if self.scalar_v1744 { (v1752 / v1756) } else { v4 });
        let v1764: f64 = (self.scalar_v1762 * (v826 - v846));
        let v1771: f64 = (((v1 + (self.scalar_v1766 * (v826 + (v846 * self.scalar_v1690))))) as f64).sqrt();
        let v1772: f64 = (v1 + v1771);
        let v1776: f64 = (v1751 * self.scalar_v1762);
        let v1779: f64 = (((v1 + (v826 * self.scalar_v1766))) as f64).sqrt();
        let v1780: f64 = (v1 + v1779);
        let v1782: f64 = (if self.scalar_v1775 { (v1776 / v1780) } else { (if self.scalar_v1759 { (v1764 / v1772) } else { v4 }) });
        let v1795: f64 = (if self.scalar_v1784 { (v771 - self.scalar_v1793) } else { v4 });
        let v1799: f64 = (if self.scalar_v1784 { (v1795 * v1795) } else { v1248 });
        let v1800: bool = (v1795 < v4);
        let v1801: bool = (self.scalar_v1784 && v1800);
        let v1804: f64 = (((self.scalar_v1797 + v1799)) as f64).sqrt();
        let v1805: f64 = (v1804 - v1795);
        let v1809: bool = (self.scalar_v1784 && (!v1800));
        let v1812: f64 = (if v1809 { (v424 * (v1795 + v1804)) } else { (if v1801 { (self.scalar_v1802 / v1805) } else { v4 }) });
        let v1816: f64 = (v1812 + (self.scalar_v1788 + (self.scalar_v340 * (v1758 + v1782))));
        let v1821: f64 = (if self.scalar_v1820 { v1 } else { (if self.scalar_v1784 { (v1812 / v1816) } else { v1 }) });
        let v1823: f64 = (if self.scalar_v1744 { (v1758 * v1821) } else { v4 });
        let v1829: f64 = (if self.scalar_v1827 { (v730 + v741) } else { v4 });
        let v1831: f64 = (-v1829);
        let v1834: bool = (v1831 < v4);
        let v1835: bool = (self.scalar_v1827 && v1834);
        let v1838: f64 = (((self.scalar_v1830 + (if self.scalar_v1827 { (v1829 * v1829) } else { v1799 }))) as f64).sqrt();
        let v1839: f64 = (v1838 - v1831);
        let v1843: bool = (self.scalar_v1827 && (!v1834));
        let v1846: f64 = (if v1843 { (v424 * (v1831 + v1838)) } else { (if v1835 { (self.scalar_v1836 / v1839) } else { v4 }) });
        let v1862: bool = (v1846 < self.scalar_v1854);
        let v1863: bool = (self.scalar_v1827 && v1862);
        let v1864: f64 = (v1846 / self.scalar_v1852);
        let v1866: f64 = (v1 - f64::powf(v1864, self.scalar_v1847));
        let v1870: bool = (self.scalar_v1827 && (!v1862));
        let v1876: f64 = (if self.scalar_v1875 { v1 } else { (if v1870 { (self.scalar_v1851 + (self.scalar_v1861 * (v1846 - self.scalar_v1854))) } else { (if v1863 { (v1 / v1866) } else { v4 }) }) });
        let v1882: bool = (v1228 < v4);
        let v1884: f64 = (((v1247 + (v1228 * v1228))) as f64).sqrt();
        let v1885: f64 = (v1884 - v1228);
        let v1888: bool = (!v1882);
        let v1891: f64 = (if v1888 { (v424 * (v1228 + v1884)) } else { (if v1882 { (v1250 / v1885) } else { v4 }) });
        let v1892: f64 = (v1262 * v1891);
        let v1893: f64 = (self.scalar_v328 / v1892);
        let v1894: bool = (v1893 < self.scalar_v28);
        let v1896: f64 = (v155 * (if v1894 { self.scalar_v28 } else { v1893 }));
        let v1899: f64 = (v741 + (self.scalar_v933 * ((if v811 { (v812 * (v1 + (v807 - self.scalar_v775))) } else { (if v808 { v809 } else { v4 }) }) - v1)));
        let v1901: bool = (v1269 > v4);
        let v1905: bool = (v730 < self.scalar_v1904);
        let v1908: f64 = ((-v1269) / self.scalar_v1907);
        let v1909: bool = (v1908 < self.scalar_v775);
        let v1911: bool = (v1905 && (v1901 && self.scalar_v1903));
        let v1912: bool = (v1909 && v1911);
        let v1913: f64 = ((v1908) as f64).exp();
        let v1916: bool = (v1911 && (!v1909));
        let v1917: f64 = (if v1916 { self.scalar_v780 } else { v1629 });
        let v1921: f64 = (if v1916 { (v1917 * (v1 + (v1908 - self.scalar_v775))) } else { (if v1912 { v1913 } else { v4 }) });
        let v1922: f64 = (self.scalar_v1904 - v730);
        let v1924: f64 = (if v1911 { (v1921 * v1922) } else { v4 });
        let v1928: f64 = (self.scalar_v1925 * f64::powf(v1924, self.scalar_v1926));
        let v1929: bool = (v1928 < self.scalar_v775);
        let v1930: bool = (v1911 && v1929);
        let v1931: f64 = ((v1928) as f64).exp();
        let v1934: bool = (v1911 && (!v1929));
        let v1935: f64 = (if v1934 { self.scalar_v780 } else { v1917 });
        let v1939: f64 = (if v1934 { (v1935 * (v1 + (v1928 - self.scalar_v775))) } else { (if v1930 { v1931 } else { v4 }) });
        let v1942: f64 = (v1924 * self.scalar_v1941);
        let v1948: bool = (v1901 && self.scalar_v1947);
        let v1950: bool = ((v730 < self.scalar_v204) && (self.scalar_v1945 && v1948));
        let v1956: f64 = (if v1950 { self.scalar_v1955 } else { v4 });
        let v1957: f64 = (self.scalar_v204 - v730);
        let v1959: f64 = (if v1950 { (v1957 / v1125) } else { v1038 });
        let v1962: f64 = ((((v32 * v1959) / v1956)) as f64).sqrt();
        let v1963: f64 = (if v1950 { v1962 } else { v4 });
        let v1966: bool = (v1950 && self.scalar_v1965);
        let v1969: bool = (v1950 && self.scalar_v1968);
        let v1972: f64 = (if v1969 { (v1 - (v424 * v1119)) } else { v4 });
        let v1973: f64 = (self.scalar_v1953 * v1972);
        let v1975: f64 = (if v1969 { (v1972 * v1973) } else { (if v1966 { self.scalar_v1953 } else { v4 }) });
        let v1976: f64 = (v1963 * v1975);
        let v1980: f64 = ((((v1963 * v1963) + (v1975 * v1975))) as f64).sqrt();
        let v1982: f64 = (if v1950 { (v1976 / v1980) } else { v4 });
        let v1984: f64 = (if v1950 { (v1957 / v1982) } else { v4 });
        let v1985: f64 = (v424 * v1982);
        let v1986: f64 = (v1956 * v1985);
        let v1989: f64 = (if v1950 { (v1984 + (v1125 * v1986)) } else { v4 });
        let v2002: f64 = (self.scalar_v963 * (if v1969 { (v1 + (self.scalar_v1992 * (v1 + (v32 * v1119)))) } else { v4 }));
        let v2004: f64 = ((if v1969 { self.scalar_v2000 } else { v4 }) - (v1269 / v2002));
        let v2007: f64 = (if v1969 { (v1984 - (v1986 * v2004)) } else { v4 });
        let v2008: f64 = (v2007 - v1989);
        let v2010: f64 = (v47 * v1984);
        let v2011: f64 = (v1984 * v2010);
        let v2017: f64 = (((if v1969 { ((v2008 * v2008) + ((v1122 * v2011) / self.scalar_v963)) } else { v1959 })) as f64).sqrt();
        let v2020: f64 = (if v1969 { (v424 * ((v1989 + v2007) + v2017)) } else { (if v1966 { v1989 } else { v4 }) });
        let v2021: f64 = (v2020 - v1984);
        let v2023: f64 = (if v1950 { (v2021 / v2020) } else { v4 });
        let v2026: bool = (((v2023) as f64).abs() > 1e-7);
        let v2027: bool = (v1950 && v2026);
        let v2029: f64 = (if v2027 { (v1985 / v2023) } else { v4 });
        let v2031: f64 = (v2020 * self.scalar_v2030);
        let v2032: f64 = (v2029 * v2031);
        let v2034: f64 = (self.scalar_v2033 / v2020);
        let v2035: f64 = ((v2034) as f64).exp();
        let v2037: f64 = (v1 + (v1975 / v2029));
        let v2039: f64 = (((v2034 * v2037)) as f64).exp();
        let v2040: f64 = (v2035 - v2039);
        let v2044: bool = (v1950 && (!v2026));
        let v2045: f64 = (self.scalar_v10 * v1975);
        let v2052: bool = (v1905 && (self.scalar_v2048 && (v1948 && self.scalar_v2049)));
        let v2053: f64 = f64::powf(v1922, self.scalar_v1926);
        let v2055: f64 = (v1269 + self.scalar_v2054);
        let v2057: f64 = (v1 - (v1269 / v2055));
        let v2059: f64 = f64::powf(v2057, self.scalar_v2058);
        let v2061: f64 = (if v2052 { (v2053 * v2059) } else { v4 });
        let v2062: bool = (self.scalar_v1965 && v2052);
        let v2064: bool = (self.scalar_v1968 && v2052);
        let v2068: f64 = (if v2064 { ((v1269 - self.scalar_v2065) / self.scalar_v2054) } else { v4 });
        let v2072: f64 = (if v2064 { ((v2068 - v1) / self.scalar_v2070) } else { v1301 });
        let v2073: bool = (v2068 < v1);
        let v2074: bool = (v2064 && v2073);
        let v2075: f64 = ((v2072) as f64).exp();
        let v2076: f64 = (v1 + v2075);
        let v2082: bool = (v2064 && (!v2073));
        let v2084: f64 = (((-v2072)) as f64).exp();
        let v2085: f64 = (v1 + v2084);
        let v2089: f64 = (if v2082 { (v2068 + (self.scalar_v2070 * ((v2085) as f64).ln())) } else { (if v2074 { (v1 + (self.scalar_v2070 * ((v2076) as f64).ln())) } else { v4 }) });
        let v2091: f64 = f64::powf(v2089, self.scalar_v2090);
        let v2094: f64 = (self.scalar_v1925 * (if v2064 { (v2061 * v2091) } else { (if v2062 { v2061 } else { v4 }) }));
        let v2095: bool = (v2094 < self.scalar_v775);
        let v2096: bool = (v2052 && v2095);
        let v2097: f64 = ((v2094) as f64).exp();
        let v2100: bool = (v2052 && (!v2095));
        let v2101: f64 = (if v2100 { self.scalar_v780 } else { v1935 });
        let v2105: f64 = (if v2100 { (v2101 * (v1 + (v2094 - self.scalar_v775))) } else { (if v2096 { v2097 } else { v1939 }) });
        let v2106: f64 = (v1922 * self.scalar_v1941);
        let v2108: f64 = (if v2052 { (v2105 * v2106) } else { (if v2044 { (v2035 * v2045) } else { (if v2027 { (v2032 * v2040) } else { (if v1911 { (v1939 * v1942) } else { v4 }) }) }) });
        let v2112: bool = (v1901 && (v2108 > v4));
        let v2113: bool = (self.scalar_v2111 && v2112);
        let v2114: f64 = (self.scalar_v335 + v1896);
        let v2115: f64 = (v1269 * v2114);
        let v2122: f64 = (if v2113 { (((self.scalar_v104 / v2115) + (self.scalar_v500 * (v1263 / self.scalar_v449))) + (self.scalar_v321 / v2114)) } else { v4 });
        let v2123: bool = (self.scalar_v2048 && v2113);
        let v2126: f64 = (if v2123 { ((v2108 - v2122) / v421) } else { v2072 });
        let v2127: bool = (v2108 < v2122);
        let v2128: bool = (v2123 && v2127);
        let v2129: f64 = ((v2126) as f64).exp();
        let v2130: f64 = (v1 + v2129);
        let v2136: bool = (v2123 && (!v2127));
        let v2138: f64 = (((-v2126)) as f64).exp();
        let v2139: f64 = (v1 + v2138);
        let v2143: f64 = (if v2136 { (v2122 - (v421 * ((v2139) as f64).ln())) } else { (if v2128 { (v2108 - (v421 * ((v2130) as f64).ln())) } else { v2108 }) });
        let v2144: f64 = (v1269 * v2143);
        let v2147: bool = (v2113 && self.scalar_v2146);
        let v2148: f64 = (v2122 * v2144);
        let v2149: f64 = (v2122 + v2143);
        let v2153: bool = (v2112 && self.scalar_v2152);
        let v2154: f64 = (if v2153 { v2144 } else { (if v2147 { (v2148 / v2149) } else { (if v2123 { v2144 } else { v4 }) }) });
        let v2160: f64 = ((v739 - self.scalar_v1129) / self.scalar_v1130);
        let v2161: bool = (v739 < self.scalar_v1129);
        let v2162: f64 = ((v2160) as f64).exp();
        let v2163: f64 = (v1 + v2162);
        let v2168: bool = (!v2161);
        let v2170: f64 = (((-v2160)) as f64).exp();
        let v2171: f64 = (v1 + v2170);
        let v2175: f64 = (if v2168 { (self.scalar_v1129 - (self.scalar_v1130 * ((v2171) as f64).ln())) } else { (if v2161 { (v739 - (self.scalar_v1130 * ((v2163) as f64).ln())) } else { v4 }) });
        let v2178: f64 = (v1 - (self.scalar_v292 * v2175));
        let v2191: f64 = (v1216 * self.scalar_v2190);
        let v2192: f64 = (v1891 * v2191);
        let v2193: f64 = (v1223 * self.scalar_v2190);
        let v2194: f64 = (v1891 * v2193);
        let v2196: f64 = ((v766 - self.scalar_v1175) / self.scalar_v1075);
        let v2197: bool = (v766 < self.scalar_v1175);
        let v2198: f64 = ((v2196) as f64).exp();
        let v2199: f64 = (v1 + v2198);
        let v2204: bool = (!v2197);
        let v2206: f64 = (((-v2196)) as f64).exp();
        let v2207: f64 = (v1 + v2206);
        let v2211: f64 = (if v2204 { (self.scalar_v1175 - (self.scalar_v1075 * ((v2207) as f64).ln())) } else { (if v2197 { (v766 - (self.scalar_v1075 * ((v2199) as f64).ln())) } else { v4 }) });
        let v2213: f64 = (v1 - (v2211 / self.scalar_v245));
        let v2226: f64 = (self.scalar_v14 * ((self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - f64::powf(v2213, self.scalar_v1195))) + (self.scalar_v1171 * (v766 - v2211)))) + (self.scalar_v314 * v766))) * self.scalar_v2224));
        let v2228: f64 = ((v771 - self.scalar_v1175) / self.scalar_v1075);
        let v2229: bool = (v771 < self.scalar_v1175);
        let v2230: f64 = ((v2228) as f64).exp();
        let v2231: f64 = (v1 + v2230);
        let v2236: bool = (!v2229);
        let v2238: f64 = (((-v2228)) as f64).exp();
        let v2239: f64 = (v1 + v2238);
        let v2243: f64 = (if v2236 { (self.scalar_v1175 - (self.scalar_v1075 * ((v2239) as f64).ln())) } else { (if v2229 { (v771 - (self.scalar_v1075 * ((v2231) as f64).ln())) } else { v4 }) });
        let v2245: f64 = (v1 - (v2243 / self.scalar_v245));
        let v2257: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - f64::powf(v2245, self.scalar_v1195))) + (self.scalar_v1171 * (v771 - v2243)))) + (self.scalar_v314 * v771)))));
        let v2264: f64 = ((v744 - self.scalar_v2262) / self.scalar_v2258);
        let v2265: bool = (v744 < self.scalar_v2262);
        let v2266: f64 = ((v2264) as f64).exp();
        let v2267: f64 = (v1 + v2266);
        let v2272: bool = (!v2265);
        let v2274: f64 = (((-v2264)) as f64).exp();
        let v2275: f64 = (v1 + v2274);
        let v2279: f64 = (if v2272 { (self.scalar_v2262 - (self.scalar_v2258 * ((v2275) as f64).ln())) } else { (if v2265 { (v744 - (self.scalar_v2258 * ((v2267) as f64).ln())) } else { v4 }) });
        let v2283: f64 = (v1 - (v2279 / self.scalar_v291));
        let v2298: f64 = (v736 / self.scalar_v2297);
        let v2299: bool = (v2298 < self.scalar_v775);
        let v2300: f64 = ((v2298) as f64).exp();
        let v2302: bool = (!v2299);
        let v2303: f64 = (if v2302 { self.scalar_v780 } else { v2101 });
        let v2308: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * (v1 + (v2298 - self.scalar_v775))) } else { (if v2299 { v2300 } else { v1477 }) }));
        let v2313: f64 = (v1119 * self.scalar_v2312);
        let v2314: f64 = (v32 + v1108);
        let v2328: f64 = (self.scalar_v106 * ((v766 - self.scalar_v225) / self.scalar_v2326));
        let v2329: bool = (v2328 < self.scalar_v775);
        let v2331: bool = (v2329 && self.scalar_v2330);
        let v2332: f64 = ((v2328) as f64).exp();
        let v2335: bool = (self.scalar_v2330 && (!v2329));
        let v2336: f64 = (if v2335 { self.scalar_v780 } else { v2303 });
        let v2342: f64 = (v806 * self.scalar_v2341);
        let v2345: f64 = (((v1 + (v436 * (if v2335 { (v2336 * (v1 + (v2328 - self.scalar_v775))) } else { (if v2331 { v2332 } else { v4 }) })))) as f64).sqrt();
        let v2346: f64 = (v1 + v2345);
        let v2348: f64 = (if self.scalar_v2330 { (v2342 / v2346) } else { (if self.scalar_v2317 { ((self.scalar_v2318 * (((v1662 / v1665) * self.scalar_v2189) + ((v1661 / v1669) * self.scalar_v2311))) / self.scalar_v674) } else { v4 }) });
        let v2356: f64 = (if self.scalar_v2354 { (v826 * self.scalar_v1211) } else { v4 });
        let v2357: f64 = (v2356 - self.scalar_v1211);
        let v2359: f64 = (((v1 + v2356)) as f64).sqrt();
        let v2360: f64 = (v1 + v2359);
        let v2364: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * (v1 + (v858 - self.scalar_v775))) } else { (if v859 { v860 } else { v4 }) })) } else { v4 });
        let v2366: f64 = (((v1 + v2364)) as f64).sqrt();
        let v2367: f64 = (v1 + v2366);
        let v2379: f64 = (self.scalar_v106 * (v771 - self.scalar_v225));
        let v2380: bool = (v2379 < self.scalar_v775);
        let v2382: bool = (v2380 && self.scalar_v2381);
        let v2383: f64 = ((v2379) as f64).exp();
        let v2386: bool = (self.scalar_v2381 && (!v2380));
        let v2387: f64 = (if v2386 { self.scalar_v780 } else { v2336 });
        let v2393: f64 = (v826 * self.scalar_v2392);
        let v2396: f64 = (((v1 + (v436 * (if v2386 { (v2387 * (v1 + (v2379 - self.scalar_v775))) } else { (if v2382 { v2383 } else { v4 }) })))) as f64).sqrt();
        let v2397: f64 = (v1 + v2396);
        let v2399: f64 = (if self.scalar_v2381 { (v2393 / v2397) } else { (if self.scalar_v2354 { ((self.scalar_v2371 * ((self.scalar_v2189 * (if self.scalar_v2354 { (v2357 / v2360) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (v2364 / v2367) } else { v4 })))) / self.scalar_v674) } else { v4 }) });
        let v2407: f64 = (if self.scalar_v2403 { (f64::powf(v1149, self.scalar_v2404) - v155) } else { v4 });
        let v2408: f64 = (if self.scalar_v2403 { v1132 } else { v4 });
        let v2409: bool = (v2408 < v4);
        let v2410: bool = (self.scalar_v2403 && v2409);
        let v2411: f64 = ((v2408) as f64).exp();
        let v2412: f64 = (v1 + v2411);
        let v2416: bool = (self.scalar_v2403 && (!v2409));
        let v2418: f64 = (((-v2408)) as f64).exp();
        let v2419: f64 = (v1 + v2418);
        let v2421: f64 = (if v2416 { (v2418 / v2419) } else { (if v2410 { (v1 / v2412) } else { v4 }) });
        let v2428: f64 = ((self.scalar_v106 * v1212) / self.scalar_v384);
        let v2429: f64 = (v424 / v1214);
        let v2431: f64 = (if self.scalar_v2403 { (v2428 * v2429) } else { v4 });
        let v2432: f64 = (v1891 * self.scalar_v2190);
        let v2437: f64 = (v741 * 0.2);
        let v2439: f64 = ((if self.scalar_v2403 { (v2308 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { (v155 + (v2407 * v2421)) } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { (v2431 * v2432) } else { v4 })));
        let v2448: f64 = (if self.scalar_v2403 { (v2192 + (v2308 * self.scalar_v2442)) } else { v4 });
        let v2457: f64 = (if self.scalar_v2456 { v2192 } else { (if self.scalar_v2403 { (v2448 * self.scalar_v2453) } else { v4 }) });
        let v2458: f64 = (if self.scalar_v2456 { v2194 } else { (if self.scalar_v2403 { (v2194 + (v2448 * self.scalar_v2449)) } else { v4 }) });
        let v2460: f64 = (v1266 + v1267);
        let v2461: f64 = (v2460 / v1263);
        let v2468: f64 = (if self.scalar_v2467 { v4 } else { (if self.scalar_v2463 { (((v2154 / v2461)) as f64).abs() } else { v4 }) });
        let v2469: bool = (v2461 > v4);
        let v2470: f64 = (v2457 + v2458);
        let v2473: bool = (!v2469);
        let v2474: f64 = (self.scalar_v667 * v1891);
        let v2476: f64 = (if v2473 { (v1263 * v2474) } else { (if v2469 { (v2470 / v2461) } else { v4 }) });
        let v2489: f64 = (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (v2476 * self.scalar_v2484) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v2476) } else { v4 }) }) });
        let v2506: f64 = (self.scalar_v27 * (self.scalar_v0 * v920));
        let v2508: f64 = (self.scalar_v27 * (self.scalar_v0 * v1269));
        let v2509: f64 = (self.scalar_v0 * ((self.scalar_v563 * (v1477 - v1)) + ((if self.scalar_v1382 { v1421 } else { (if self.scalar_v510 { (v1421 + (v1423 / v1427)) } else { v4 }) }) + (self.scalar_v554 * (v1453 - v1)))));
        let v2510: f64 = (self.scalar_v27 * v2509);
        let v2512: f64 = (((v1395 + (self.scalar_v474 * (v1441 - v1))) + (v4 * v736)) - (if v1569 { v4 } else { (if v1483 { (self.scalar_v36 * (self.scalar_v292 * (v1498 * v1564))) } else { v4 }) }));
        let v2516: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v570 * (v1296 - v1)) + ((v1318 * v1320) + v2512))));
        let v2519: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v1876))));
        let v2520: f64 = (if self.scalar_v510 { v2519 } else { v4 });
        let v2521: f64 = (if self.scalar_v1382 { v2519 } else { v4 });
        let v2523: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v1727) } else { v1727 })));
        let v2525: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1712 { (v1714 / v1718) } else { (if self.scalar_v1682 { (v1687 / v1696) } else { v4 }) })));
        let v2527: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v1782 * v1821) } else { v4 })));
        let v2529: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1730 / v1737) + (v4 * v744))));
        let v2531: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1899 / v1896)));
        let v2533: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v2154)));
        let v2536: f64 = (self.scalar_v27 * ((self.scalar_v0 * (self.scalar_v0 * (v747 - v734))) / self.scalar_v321));
        let v2539: f64 = (self.scalar_v27 * ((self.scalar_v0 * v752) / self.scalar_v335));
        let v2541: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_v0 * ((if self.scalar_v2456 { v2308 } else { (if self.scalar_v2403 { (v2308 * self.scalar_v2443) } else { v4 }) }) + ((v1157 * self.scalar_v2157) + v2457))));
        let v2542: f64 = (self.scalar_v27 * v2541);
        let v2544: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_v0 * (self.scalar_v2176 * ((self.scalar_v1152 * (v1 - f64::powf(v2178, self.scalar_v1150))) + (v155 * (v739 - v2175))))));
        let v2545: f64 = (self.scalar_v27 * v2544);
        let v2547: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (self.scalar_v0 * ((v2313 * v2314) + ((v1209 * self.scalar_v2187) + v2458))));
        let v2548: f64 = (self.scalar_v27 * v2547);
        let v2550: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (self.scalar_v0 * (self.scalar_v304 * ((self.scalar_v2281 * (v1 - f64::powf(v2283, self.scalar_v2280))) + (v32 * (v744 - v2279))))));
        let v2551: f64 = (self.scalar_v27 * v2550);
        let v2553: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * v2439) } else { v4 })));
        let v2554: f64 = (self.scalar_v27 * v2553);
        let v2557: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, ((self.scalar_v0 * (v750 - v747)) * self.scalar_v2555));
        let v2558: f64 = (self.scalar_v27 * v2557);
        let v2561: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (v757 * self.scalar_v2559));
        let v2562: f64 = (self.scalar_v27 * v2561);
        let v2564: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1823 * v1876)));
        let v2567: f64 = (self.scalar_v27 * (self.scalar_v710 * (self.scalar_v0 * v770)));
        let v2569: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (self.scalar_v0 * (v2257 + (if self.scalar_v2351 { (v1821 * v2399) } else { v4 }))));
        let v2570: f64 = (self.scalar_v27 * v2569);
        let v2573: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1746 * v1876) + ((v1467 * v1876) + (v4 * v766)))));
        let v2575: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, (self.scalar_v0 * (v2226 + (if self.scalar_v2351 { (self.scalar_v14 * v2348) } else { v2348 }))));
        let v2576: f64 = (self.scalar_v27 * v2575);
        let v2580: f64 = (if self.scalar_v711 { (self.scalar_v27 * (self.scalar_v718 * (self.scalar_v0 * v763))) } else { v4 });
        let v2584: f64 = (if self.scalar_v719 { (self.scalar_v27 * (self.scalar_v726 * (self.scalar_v0 * v760))) } else { v4 });
        let v2585: f64 = ctx.node_voltage(nodes[11]);
        let v2586: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, v2585);
        let v2587: f64 = (v2489 * v2586);
        let v2588: f64 = (v2468 * v2585);
        let v2601: f64 = (if v779 { (v781 * self.scalar_v2593) } else { (if v776 { (v777 * self.scalar_v2593) } else { v4 }) });
        let v2602: f64 = (if v779 { (v781 * self.scalar_v2594) } else { (if v776 { (v777 * self.scalar_v2594) } else { v4 }) });
        let v2611: f64 = (if v791 { (v792 * self.scalar_v2603) } else { (if v788 { (v789 * self.scalar_v2603) } else { v4 }) });
        let v2612: f64 = (if v791 { (v792 * self.scalar_v2604) } else { (if v788 { (v789 * self.scalar_v2604) } else { v4 }) });
        let v2627: f64 = (if v801 { (v802 * self.scalar_v2593) } else { (if v798 { (v799 * self.scalar_v2593) } else { v4 }) });
        let v2628: f64 = (if v801 { (v802 * self.scalar_v2613) } else { (if v798 { (v799 * self.scalar_v2613) } else { v4 }) });
        let v2629: f64 = (if v801 { (v802 * self.scalar_v2614) } else { (if v798 { (v799 * self.scalar_v2614) } else { v4 }) });
        let v2630: f64 = (if v801 { (v802 * self.scalar_v2594) } else { (if v798 { (v799 * self.scalar_v2594) } else { v4 }) });
        let v2652: f64 = (if v821 { (v822 * self.scalar_v2613) } else { (if v818 { (v819 * self.scalar_v2613) } else { v4 }) });
        let v2653: f64 = (if v821 { (v822 * self.scalar_v2639) } else { (if v818 { (v819 * self.scalar_v2639) } else { v4 }) });
        let v2654: f64 = (if v821 { (v822 * self.scalar_v2614) } else { (if v818 { (v819 * self.scalar_v2614) } else { v4 }) });
        let v2655: f64 = (if v821 { (v822 * self.scalar_v2594) } else { (if v818 { (v819 * self.scalar_v2594) } else { v4 }) });
        let v2662: f64 = (if v831 { (v832 * self.scalar_v2593) } else { (if v828 { (v829 * self.scalar_v2593) } else { v4 }) });
        let v2663: f64 = (if v831 { (v832 * self.scalar_v2594) } else { (if v828 { (v829 * self.scalar_v2594) } else { v4 }) });
        let v2673: f64 = (if v841 { (v842 * self.scalar_v2593) } else { (if v838 { (v839 * self.scalar_v2593) } else { v4 }) });
        let v2674: f64 = (if v841 { (v842 * self.scalar_v2614) } else { (if v838 { (v839 * self.scalar_v2614) } else { v4 }) });
        let v2675: f64 = (if v841 { (v842 * self.scalar_v2594) } else { (if v838 { (v839 * self.scalar_v2594) } else { v4 }) });
        let v2685: f64 = (if v851 { (v852 * self.scalar_v2593) } else { (if v848 { (v849 * self.scalar_v2593) } else { v4 }) });
        let v2686: f64 = (if v851 { (v852 * self.scalar_v2614) } else { (if v848 { (v849 * self.scalar_v2614) } else { v4 }) });
        let v2687: f64 = (if v851 { (v852 * self.scalar_v2594) } else { (if v848 { (v849 * self.scalar_v2594) } else { v4 }) });
        let v2726: f64 = (if v884 { (v885 * self.scalar_v2593) } else { (if v881 { (v882 * self.scalar_v2593) } else { v4 }) });
        let v2727: f64 = (if v884 { (v885 * self.scalar_v2594) } else { (if v881 { (v882 * self.scalar_v2594) } else { v4 }) });
        let v2734: f64 = (if v895 { (v896 * self.scalar_v2593) } else { (if v892 { (v893 * self.scalar_v2593) } else { v4 }) });
        let v2735: f64 = (if v895 { (v896 * self.scalar_v2594) } else { (if v892 { (v893 * self.scalar_v2594) } else { v4 }) });
        let v2738: f64 = (v32 * v903);
        let v2739: f64 = ((v436 * v2726) / v2738);
        let v2740: f64 = ((v436 * v2727) / v2738);
        let v2743: f64 = (v32 * v906);
        let v2744: f64 = ((v436 * v2734) / v2743);
        let v2745: f64 = ((v436 * v2735) / v2743);
        let v2751: f64 = (v908 * v908);
        let v2757: f64 = (if v911 { v4 } else { (((v908 * (v32 * v2734)) - (v907 * v2744)) / v2751) });
        let v2758: f64 = (if v911 { v4 } else { (((v908 * (v32 * v2735)) - (v907 * v2745)) / v2751) });
        let v2775: f64 = (self.scalar_v104 * ((v2739 - v2744) - ((((v908 * v2739) - (v914 * v2744)) / v2751) / v915)));
        let v2776: f64 = (self.scalar_v104 * ((-v2745) - (((-(v914 * v2745)) / v2751) / v915)));
        let v2777: f64 = (self.scalar_v104 * (v2740 - ((v2740 / v908) / v915)));
        let v2779: f64 = (self.scalar_v2589 + v2777);
        let v2780: f64 = (v2775 / self.scalar_v352);
        let v2781: f64 = ((self.scalar_v0 + v2776) / self.scalar_v352);
        let v2782: f64 = (v2779 / self.scalar_v352);
        let v2792: f64 = (self.scalar_v352 * (v424 * v2780));
        let v2793: f64 = (self.scalar_v352 * (v424 * v2781));
        let v2794: f64 = (self.scalar_v352 * (v424 * v2782));
        let v2806: f64 = (if v921 { ((self.scalar_v933 * ((self.scalar_v106 * v2792) / v937)) - (if v927 { (self.scalar_v0 / v929) } else { (if v924 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v2807: f64 = (if v921 { ((self.scalar_v933 * ((self.scalar_v106 * v2793) / v937)) - (if v927 { (self.scalar_v2589 / v929) } else { (if v924 { self.scalar_v2589 } else { v4 }) })) } else { v4 });
        let v2808: f64 = (if v921 { (self.scalar_v933 * ((self.scalar_v106 * v2794) / v937)) } else { v4 });
        let v2809: f64 = (v942 * v2806);
        let v2811: f64 = (v942 * v2807);
        let v2813: f64 = (v942 * v2808);
        let v2818: f64 = (v32 * v954);
        let v2819: f64 = ((if v921 { (v2809 + v2809) } else { v4 }) / v2818);
        let v2820: f64 = ((if v921 { (v2811 + v2811) } else { v4 }) / v2818);
        let v2821: f64 = ((if v921 { (v2813 + v2813) } else { v4 }) / v2818);
        let v2827: f64 = (v955 * v955);
        let v2844: f64 = (if v959 { (v424 * (v2806 + v2819)) } else { (if v951 { ((-(v952 * (v2819 - v2806))) / v2827) } else { v4 }) });
        let v2845: f64 = (if v959 { (v424 * (v2807 + v2820)) } else { (if v951 { ((-(v952 * (v2820 - v2807))) / v2827) } else { v4 }) });
        let v2846: f64 = (if v959 { (v424 * (v2808 + v2821)) } else { (if v951 { ((-(v952 * (v2821 - v2808))) / v2827) } else { v4 }) });
        let v2862: f64 = (v970 * v970);
        let v2872: f64 = (if v921 { (((v970 * ((v966 * v2844) + (v962 * v2844))) - (v967 * (self.scalar_v964 * v2844))) / v2862) } else { v4 });
        let v2873: f64 = (if v921 { (((v970 * ((v966 * v2845) + (v962 * v2845))) - (v967 * (self.scalar_v964 * v2845))) / v2862) } else { v4 });
        let v2874: f64 = (if v921 { (((v970 * ((v966 * v2846) + (v962 * v2846))) - (v967 * (self.scalar_v964 * v2846))) / v2862) } else { v4 });
        let v2878: f64 = (v972 * v972);
        let v2888: f64 = (if v921 { (((v972 * v2780) - (v920 * v2872)) / v2878) } else { v4 });
        let v2889: f64 = (if v921 { (((v972 * v2781) - (v920 * v2873)) / v2878) } else { v4 });
        let v2890: f64 = (if v921 { (((v972 * v2782) - (v920 * v2874)) / v2878) } else { v4 });
        let v2894: f64 = (if v921 { (v2888 / self.scalar_v976) } else { v4 });
        let v2895: f64 = (if v921 { (v2889 / self.scalar_v976) } else { v4 });
        let v2896: f64 = (if v921 { (v2890 / self.scalar_v976) } else { v4 });
        let v2930: f64 = (if v921 { ((if v988 { (v2888 + (self.scalar_v976 * ((v990 * (-v2894)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2894) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2931: f64 = (if v921 { ((if v988 { (v2889 + (self.scalar_v976 * ((v990 * (-v2895)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2895) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2932: f64 = (if v921 { ((if v988 { (v2890 + (self.scalar_v976 * ((v990 * (-v2896)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2896) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2936: f64 = (if v921 { (v2844 / self.scalar_v965) } else { v4 });
        let v2937: f64 = (if v921 { (v2845 / self.scalar_v965) } else { v4 });
        let v2938: f64 = (if v921 { (v2846 / self.scalar_v965) } else { v4 });
        let v2960: f64 = (v32 * v1012);
        let v2978: f64 = ((v1015 * (((v1009 * ((v1007 * v2936) + (v1006 * (v436 * v2930)))) + (v1008 * v2936)) / v2960)) - (v1013 * ((v1014 * v2936) + (v1009 * (v32 * v2930)))));
        let v2979: f64 = (v1015 * v1015);
        let v2983: f64 = ((v1015 * (((v1009 * ((v1007 * v2937) + (v1006 * (v436 * v2931)))) + (v1008 * v2937)) / v2960)) - (v1013 * ((v1014 * v2937) + (v1009 * (v32 * v2931)))));
        let v2987: f64 = ((v1015 * (((v1009 * ((v1007 * v2938) + (v1006 * (v436 * v2932)))) + (v1008 * v2938)) / v2960)) - (v1013 * ((v1014 * v2938) + (v1009 * (v32 * v2932)))));
        let v2989: f64 = (if v921 { (v2978 / v2979) } else { v4 });
        let v2990: f64 = (if v921 { (v2983 / v2979) } else { v4 });
        let v2991: f64 = (if v921 { (v2987 / v2979) } else { v4 });
        let v2997: f64 = ((v1017 * v2757) + (v912 * v2989));
        let v3000: f64 = ((v1017 * v2758) + (v912 * v2990));
        let v3001: f64 = (v912 * v2991);
        let v3008: f64 = (v1021 * v1021);
        let v3018: f64 = (if v921 { (((v1021 * ((-v2989) + v2997)) - (v1020 * v2997)) / v3008) } else { v4 });
        let v3019: f64 = (if v921 { (((v1021 * ((-v2990) + v3000)) - (v1020 * v3000)) / v3008) } else { v4 });
        let v3020: f64 = (if v921 { (((v1021 * ((-v2991) + v3001)) - (v1020 * v3001)) / v3008) } else { v4 });
        let v3033: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2792) + (v935 * v3018))) } else { v4 });
        let v3034: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2793) + (v935 * v3019))) } else { v4 });
        let v3035: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2794) + (v935 * v3020))) } else { v4 });
        let v3051: f64 = (if v921 { ((v32 * v3033) + ((v1029 * v2757) + (v912 * (v2757 + v3033)))) } else { v4 });
        let v3052: f64 = (if v921 { ((v32 * v3034) + ((v1029 * v2758) + (v912 * (v2758 + v3034)))) } else { v4 });
        let v3053: f64 = (if v921 { ((v32 * v3035) + (v912 * v3035)) } else { v4 });
        let v3057: f64 = (if v921 { (v424 * v3033) } else { v4 });
        let v3058: f64 = (if v921 { (v424 * v3034) } else { v4 });
        let v3059: f64 = (if v921 { (v424 * v3035) } else { v4 });
        let v3060: f64 = (v1035 * v3057);
        let v3062: f64 = (v1035 * v3058);
        let v3064: f64 = (v1035 * v3059);
        let v3069: f64 = (if v921 { (v3051 + (v3060 + v3060)) } else { v4 });
        let v3070: f64 = (if v921 { (v3052 + (v3062 + v3062)) } else { v4 });
        let v3071: f64 = (if v921 { (v3053 + (v3064 + v3064)) } else { v4 });
        let v3072: f64 = (v32 * v1041);
        let v3073: f64 = (v3069 / v3072);
        let v3074: f64 = (v3070 / v3072);
        let v3075: f64 = (v3071 / v3072);
        let v3088: f64 = (v1046 * v1046);
        let v3101: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3051) - (v1032 * (v3073 - v3057))) / v3088) } else { (if v1040 { (v3057 + v3073) } else { v4 }) }) });
        let v3102: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3052) - (v1032 * (v3074 - v3058))) / v3088) } else { (if v1040 { (v3058 + v3074) } else { v4 }) }) });
        let v3103: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3053) - (v1032 * (v3075 - v3059))) / v3088) } else { (if v1040 { (v3059 + v3075) } else { v4 }) }) });
        let v3122: f64 = (if v921 { (self.scalar_v1059 * v2780) } else { v4 });
        let v3123: f64 = (if v921 { (self.scalar_v1059 * v2781) } else { v4 });
        let v3124: f64 = (if v921 { (self.scalar_v1059 * v2782) } else { v4 });
        let v3131: f64 = (v1062 * v3122);
        let v3133: f64 = (v1062 * v3123);
        let v3135: f64 = (v1062 * v3124);
        let v3140: f64 = (v32 * v1069);
        let v3159: f64 = (v1080 * v1080);
        let v3175: f64 = (self.scalar_v963 * v2780);
        let v3176: f64 = (self.scalar_v963 * v2781);
        let v3177: f64 = (self.scalar_v963 * v2782);
        let v3181: f64 = (v1086 * v1086);
        let v3208: f64 = (v914 * v914);
        let v3216: f64 = (if v1091 { (((v914 * (v32 * v2727)) - (v1092 * v2740)) / v3208) } else { v3103 });
        let v3217: f64 = (if v1091 { v2601 } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3101) + (v1052 * v3101))) } else { v4 }) });
        let v3218: f64 = (if v1091 { v4 } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3102) + (v1052 * v3102))) } else { v4 }) });
        let v3219: f64 = (if v1091 { v2602 } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3103) + (v1052 * v3103))) } else { v4 }) });
        let v3220: f64 = (v2757 + (if v1091 { (((v914 * (v32 * v2726)) - (v1092 * v2739)) / v3208) } else { v3101 }));
        let v3221: f64 = (v2758 + (if v1091 { v4 } else { v3102 }));
        let v3225: f64 = (if v1107 { (v424 * v3220) } else { v4 });
        let v3226: f64 = (if v1107 { (v424 * v3221) } else { v4 });
        let v3227: f64 = (if v1107 { (v424 * v3216) } else { v4 });
        let v3231: f64 = (v1111 * v1111);
        let v3250: f64 = (v1117 * v1117);
        let v3260: f64 = (if v1115 { (((v1117 * v2775) - (v918 * ((self.scalar_v0 + v2775) - self.scalar_v0))) / v3250) } else { (if v1107 { (((v1111 * v3225) - (v1110 * v3225)) / v3231) } else { v3018 }) });
        let v3261: f64 = (if v1115 { (((v1117 * v2776) - (v918 * (v2776 - self.scalar_v2589))) / v3250) } else { (if v1107 { (((v1111 * v3226) - (v1110 * v3226)) / v3231) } else { v3019 }) });
        let v3262: f64 = (if v1115 { (((v1117 * v2777) - (v918 * v2779)) / v3250) } else { (if v1107 { (((v1111 * v3227) - (v1110 * v3227)) / v3231) } else { v3020 }) });
        let v3266: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2780)) - (v1079 * (v2780 + v2872))) / v3159)) } else { v4 }) });
        let v3267: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2781)) - (v1079 * (v2781 + v2873))) / v3159)) } else { v4 }) });
        let v3268: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2782)) - (v1079 * (v2782 + v2874))) / v3159)) } else { v4 }) });
        let v3269: f64 = (if v1091 { v2780 } else { (if v921 { (((v1086 * v3175) - (v1085 * v2780)) / v3181) } else { v4 }) });
        let v3270: f64 = (if v1091 { v2781 } else { (if v921 { (((v1086 * v3176) - (v1085 * v2781)) / v3181) } else { v4 }) });
        let v3271: f64 = (if v1091 { v2782 } else { (if v921 { (((v1086 * v3177) - (v1085 * v2782)) / v3181) } else { v4 }) });
        let v3278: f64 = (if v1091 { (-(v3269 / self.scalar_v963)) } else { (if v921 { ((-v3175) / v3181) } else { v4 }) });
        let v3279: f64 = (if v1091 { (-(v3270 / self.scalar_v963)) } else { (if v921 { ((-v3176) / v3181) } else { v4 }) });
        let v3280: f64 = (if v1091 { (-(v3271 / self.scalar_v963)) } else { (if v921 { ((-v3177) / v3181) } else { v4 }) });
        let v3303: f64 = (if v1140 { (-(self.scalar_v1130 * ((v1142 * self.scalar_v3293) / v1143))) } else { (if v1133 { (self.scalar_v2589 - (self.scalar_v1130 * ((v1134 * self.scalar_v3281) / v1135))) } else { v4 }) });
        let v3304: f64 = (if v1140 { (-(self.scalar_v1130 * ((v1142 * self.scalar_v3294) / v1143))) } else { (if v1133 { (self.scalar_v0 - (self.scalar_v1130 * ((v1134 * self.scalar_v3282) / v1135))) } else { v4 }) });
        let v3307: f64 = (-(self.scalar_v292 * v3303));
        let v3308: f64 = (-(self.scalar_v292 * v3304));
        let v3311: f64 = (self.scalar_v1150 * f64::powf(v1149, self.scalar_v3309));
        let v3312: f64 = (v3307 * v3311);
        let v3313: f64 = (v3308 * v3311);
        let v3322: f64 = ((self.scalar_v1152 * (-v3312)) + (v155 * (self.scalar_v2589 - v3303)));
        let v3323: f64 = ((self.scalar_v1152 * (-v3313)) + (v155 * (self.scalar_v0 - v3304)));
        let v3328: f64 = (if self.scalar_v1163 { (self.scalar_v0 + (if v1091 { v4 } else { (if v921 { (v3122 + (((if v921 { (self.scalar_v1064 * v2780) } else { v4 }) + (v3131 + v3131)) / v3140)) } else { v4 }) })) } else { self.scalar_v3324 });
        let v3329: f64 = (if self.scalar_v1163 { (self.scalar_v2589 + (if v1091 { self.scalar_v0 } else { (if v921 { (v3123 + (((if v921 { (self.scalar_v1064 * v2781) } else { v4 }) + (v3133 + v3133)) / v3140)) } else { v4 }) })) } else { self.scalar_v3325 });
        let v3331: f64 = (if self.scalar_v1167 { self.scalar_v0 } else { v3328 });
        let v3332: f64 = (if self.scalar_v1167 { v4 } else { v3329 });
        let v3333: f64 = (if self.scalar_v1167 { self.scalar_v2589 } else { (if self.scalar_v1163 { (if v1091 { self.scalar_v2589 } else { (if v921 { (v3124 + (((if v921 { (self.scalar_v1064 * v2782) } else { v4 }) + (v3135 + v3135)) / v3140)) } else { v4 }) }) } else { v4 }) });
        let v3337: f64 = (v1121 * v1121);
        let v3338: f64 = (((v1121 * v3331) - (v1176 * v3266)) / v3337);
        let v3342: f64 = (((v1121 * v3332) - (v1176 * v3267)) / v3337);
        let v3346: f64 = (((v1121 * v3333) - (v1176 * v3268)) / v3337);
        let v3389: f64 = (if v1185 { (-((v1189 * v3266) + (v1121 * ((v1187 * (-v3338)) / v1188)))) } else { (if v1178 { (v3331 - ((v1181 * v3266) + (v1121 * ((v1179 * v3338) / v1180)))) } else { v4 }) });
        let v3390: f64 = (if v1185 { (-((v1189 * v3267) + (v1121 * ((v1187 * (-v3342)) / v1188)))) } else { (if v1178 { (v3332 - ((v1181 * v3267) + (v1121 * ((v1179 * v3342) / v1180)))) } else { v4 }) });
        let v3391: f64 = (if v1185 { (-((v1189 * v3268) + (v1121 * ((v1187 * (-v3346)) / v1188)))) } else { (if v1178 { (v3333 - ((v1181 * v3268) + (v1121 * ((v1179 * v3346) / v1180)))) } else { v4 }) });
        let v3394: f64 = (self.scalar_v1193 * f64::powf(v1125, self.scalar_v3392));
        let v3395: f64 = (v3278 * v3394);
        let v3396: f64 = (v3279 * v3394);
        let v3397: f64 = (v3280 * v3394);
        let v3406: f64 = (self.scalar_v1195 * f64::powf(v1198, self.scalar_v3404));
        let v3440: f64 = ((self.scalar_v1196 * (-((v1199 * v3395) + (v1194 * ((-(v3389 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3395)) + (v1203 * (v3331 - v3389))));
        let v3441: f64 = ((self.scalar_v1196 * (-((v1199 * v3396) + (v1194 * ((-(v3390 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3396)) + (v1203 * (v3332 - v3390))));
        let v3442: f64 = ((self.scalar_v1196 * (-((v1199 * v3397) + (v1194 * ((-(v3391 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3397)) + (v1203 * (v3333 - v3391))));
        let v3445: f64 = (self.scalar_v1170 * v3442);
        let v3448: f64 = ((self.scalar_v1170 * v3440) + self.scalar_v3446);
        let v3449: f64 = ((self.scalar_v1170 * v3441) + self.scalar_v3447);
        let v3450: f64 = (self.scalar_v1211 * v2611);
        let v3451: f64 = (self.scalar_v1211 * v2612);
        let v3452: f64 = (v32 * v1214);
        let v3453: f64 = (v3450 / v3452);
        let v3454: f64 = (v3451 / v3452);
        let v3458: f64 = (v1215 * v1215);
        let v3459: f64 = (((v1215 * v3450) - (v1212 * v3453)) / v3458);
        let v3463: f64 = (((v1215 * v3451) - (v1212 * v3454)) / v3458);
        let v3466: f64 = (self.scalar_v1217 * f64::powf(v1095, self.scalar_v3464));
        let v3467: f64 = (v3217 * v3466);
        let v3468: f64 = (v3218 * v3466);
        let v3469: f64 = (v3219 * v3466);
        let v3470: f64 = (self.scalar_v1211 * v3467);
        let v3471: f64 = (self.scalar_v1211 * v3468);
        let v3472: f64 = (self.scalar_v1211 * v3469);
        let v3473: f64 = (v32 * v1221);
        let v3480: f64 = (v1222 * v1222);
        let v3481: f64 = (((v1222 * v3470) - (v1219 * (v3470 / v3473))) / v3480);
        let v3485: f64 = (((v1222 * v3471) - (v1219 * (v3471 / v3473))) / v3480);
        let v3489: f64 = (((v1222 * v3472) - (v1219 * (v3472 / v3473))) / v3480);
        let v3490: f64 = (v3322 / self.scalar_v623);
        let v3491: f64 = (v3323 / self.scalar_v623);
        let v3492: f64 = (v3448 / self.scalar_v620);
        let v3493: f64 = (v3449 / self.scalar_v620);
        let v3494: f64 = (v3445 / self.scalar_v620);
        let v3495: f64 = (v3491 + v3492);
        let v3530: f64 = (((v1239 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v3491)) } else { v4 })) - (v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3448) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244);
        let v3533: f64 = (if self.scalar_v1230 { ((v1239 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v3490)) } else { v4 })) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3490 } else { v4 }) });
        let v3534: f64 = (if self.scalar_v1230 { v3530 } else { (if self.scalar_v1224 { v3495 } else { v4 }) });
        let v3535: f64 = (if self.scalar_v1230 { ((-(v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3449) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3493 } else { v4 }) });
        let v3536: f64 = (if self.scalar_v1230 { ((-(v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3445) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3494 } else { v4 }) });
        let v3537: f64 = (v1246 * v3533);
        let v3538: f64 = (v3537 + v3537);
        let v3539: f64 = (v1246 * v3534);
        let v3540: f64 = (v3539 + v3539);
        let v3541: f64 = (v1246 * v3535);
        let v3542: f64 = (v3541 + v3541);
        let v3543: f64 = (v1246 * v3536);
        let v3544: f64 = (v3543 + v3543);
        let v3545: f64 = (v32 * v1252);
        let v3546: f64 = (v3538 / v3545);
        let v3547: f64 = (v3540 / v3545);
        let v3548: f64 = (v3542 / v3545);
        let v3549: f64 = (v3544 / v3545);
        let v3556: f64 = (v1253 * v1253);
        let v3584: f64 = (v424 * v3459);
        let v3585: f64 = (v424 * (v3463 + v3481));
        let v3586: f64 = (v424 * v3485);
        let v3587: f64 = (v424 * v3489);
        let v3590: f64 = ((v1262 * (if v1256 { (v424 * (v3533 + v3546)) } else { (if v1249 { ((-(v1250 * (v3546 - v3533))) / v3556) } else { v4 }) })) + (v1259 * v3584));
        let v3593: f64 = ((v1262 * (if v1256 { (v424 * (v3534 + v3547)) } else { (if v1249 { ((-(v1250 * (v3547 - v3534))) / v3556) } else { v4 }) })) + (v1259 * v3585));
        let v3596: f64 = ((v1262 * (if v1256 { (v424 * (v3535 + v3548)) } else { (if v1249 { ((-(v1250 * (v3548 - v3535))) / v3556) } else { v4 }) })) + (v1259 * v3586));
        let v3599: f64 = ((v1262 * (if v1256 { (v424 * (v3536 + v3549)) } else { (if v1249 { ((-(v1250 * (v3549 - v3536))) / v3556) } else { v4 }) })) + (v1259 * v3587));
        let v3600: f64 = (self.scalar_v1265 * v3467);
        let v3601: f64 = (self.scalar_v1265 * v3468);
        let v3602: f64 = (self.scalar_v1265 * v3469);
        let v3604: f64 = (self.scalar_v449 * v2612);
        let v3608: f64 = (v1263 * (self.scalar_v449 * v2611));
        let v3611: f64 = (v1263 * v1263);
        let v3612: f64 = ((v3608 - (v1268 * v3590)) / v3611);
        let v3616: f64 = (((v1263 * (v3604 - v3600)) - (v1268 * v3593)) / v3611);
        let v3620: f64 = (((v1263 * (-v3601)) - (v1268 * v3596)) / v3611);
        let v3624: f64 = (((v1263 * (-v3602)) - (v1268 * v3599)) / v3611);
        let v3645: f64 = (if v1278 { (self.scalar_v2589 + (v1270 * ((v1280 * self.scalar_v3635) / v1281))) } else { (if v1272 { (v1270 * ((v1273 * self.scalar_v3625) / v1274)) } else { v4 }) });
        let v3646: f64 = (if v1278 { (self.scalar_v0 + (v1270 * ((v1280 * self.scalar_v3636) / v1281))) } else { (if v1272 { (v1270 * ((v1273 * self.scalar_v3626) / v1274)) } else { v4 }) });
        let v3647: f64 = (v3645 / self.scalar_v1286);
        let v3648: f64 = (v3646 / self.scalar_v1286);
        let v3655: f64 = (if v1291 { (v1292 * v3647) } else { (if v1288 { (v1289 * v3647) } else { v4 }) });
        let v3656: f64 = (if v1291 { (v1292 * v3648) } else { (if v1288 { (v1289 * v3648) } else { v4 }) });
        let v3681: f64 = (if v1309 { (-(v31 * ((v1311 * self.scalar_v3671) / v1312))) } else { (if v1302 { (self.scalar_v2589 - (v31 * ((v1303 * self.scalar_v3659) / v1304))) } else { v4 }) });
        let v3682: f64 = (if v1309 { (-(v31 * ((v1311 * self.scalar_v3672) / v1312))) } else { (if v1302 { (self.scalar_v0 - (v31 * ((v1303 * self.scalar_v3660) / v1304))) } else { v4 }) });
        let v3688: f64 = (v32 * f64::powf(v1319, v1));
        let v3705: f64 = (if v1326 { (v1327 * self.scalar_v3697) } else { (if v1323 { (v1324 * self.scalar_v3697) } else { v3645 }) });
        let v3706: f64 = (if v1326 { (v1327 * self.scalar_v3698) } else { (if v1323 { (v1324 * self.scalar_v3698) } else { v3646 }) });
        let v3713: f64 = (if v1339 { (v1340 * self.scalar_v2594) } else { (if v1335 { (v1336 * self.scalar_v2594) } else { v3647 }) });
        let v3714: f64 = (if v1339 { (v1340 * self.scalar_v2593) } else { (if v1335 { (v1336 * self.scalar_v2593) } else { v3648 }) });
        let v3715: f64 = (v3612 / self.scalar_v449);
        let v3716: f64 = (v3616 / self.scalar_v449);
        let v3717: f64 = (v3620 / self.scalar_v449);
        let v3718: f64 = (v3624 / self.scalar_v449);
        let v3731: f64 = (if v1354 { (v1356 * v3715) } else { (if v1350 { (v1351 * v3715) } else { v3655 }) });
        let v3732: f64 = (if v1354 { (v1356 * v3716) } else { (if v1350 { (v1351 * v3716) } else { v3656 }) });
        let v3733: f64 = (if v1354 { (v1356 * v3717) } else { (if v1350 { (v1351 * v3717) } else { v4 }) });
        let v3734: f64 = (if v1354 { (v1356 * v3718) } else { (if v1350 { (v1351 * v3718) } else { v4 }) });
        let v3735: f64 = (self.scalar_v500 * v3705);
        let v3736: f64 = (self.scalar_v500 * v3706);
        let v3741: f64 = (v32 * v1367);
        let v3747: f64 = (v1368 * v1368);
        let v3777: f64 = (v1376 * v1376);
        let v3791: f64 = ((v3735 + (v1370 * (((v1368 * (self.scalar_v1363 * v3705)) - (v1364 * ((v436 * v3713) / v3741))) / v3747))) + (((v1376 * (v1374 * v3731)) - (v1375 * v3731)) / v3777));
        let v3792: f64 = ((v3736 + ((v1370 * (((v1368 * (self.scalar_v1363 * v3706)) - (v1364 * ((v436 * v3714) / v3741))) / v3747)) + (v1369 * v3492))) + (((v1376 * ((v1374 * v3732) + (v1360 * (self.scalar_v525 * v3217)))) - (v1375 * v3732)) / v3777));
        let v3801: f64 = (if self.scalar_v1383 { v4 } else { (if self.scalar_v510 { ((v1369 * v3493) + (((v1376 * ((v1374 * v3733) + (v1360 * (self.scalar_v525 * v3218)))) - (v1375 * v3733)) / v3777)) } else { v4 }) });
        let v3802: f64 = (if self.scalar_v1383 { v4 } else { (if self.scalar_v510 { ((v1369 * v3494) + (((v1376 * ((v1374 * v3734) + (v1360 * (self.scalar_v525 * v3219)))) - (v1375 * v3734)) / v3777)) } else { v4 }) });
        let v3827: f64 = (if self.scalar_v1386 { (self.scalar_v500 * ((self.scalar_v1387 * v3706) + ((v1391 * v3492) + (v1370 * (self.scalar_v1380 * (v3217 + v3706)))))) } else { (if self.scalar_v1383 { v3736 } else { (if self.scalar_v510 { v3792 } else { v4 }) }) });
        let v3839: f64 = (if v1401 { (v1402 * self.scalar_v3830) } else { (if v1398 { (v1399 * self.scalar_v3830) } else { v3705 }) });
        let v3840: f64 = (if v1401 { (v1402 * self.scalar_v3831) } else { (if v1398 { (v1399 * self.scalar_v3831) } else { v4 }) });
        let v3841: f64 = (if v1401 { v4 } else { (if v1398 { v4 } else { v3706 }) });
        let v3852: f64 = (self.scalar_v508 * v3839);
        let v3853: f64 = (self.scalar_v508 * v3840);
        let v3854: f64 = (self.scalar_v508 * v3841);
        let v3861: f64 = (v32 * v1426);
        let v3868: f64 = (v1427 * v1427);
        let v3869: f64 = (((v1427 * (self.scalar_v1422 * v3839)) - (v1423 * ((v436 * (if v1414 { (v1415 * self.scalar_v2594) } else { (if v1410 { (v1411 * self.scalar_v2594) } else { v3713 }) })) / v3861))) / v3868);
        let v3873: f64 = (((v1427 * (self.scalar_v1422 * v3840)) - (v1423 * ((v436 * (if v1414 { (v1415 * self.scalar_v2593) } else { (if v1410 { (v1411 * self.scalar_v2593) } else { v4 }) })) / v3861))) / v3868);
        let v3883: f64 = (if self.scalar_v510 { (v3854 + (((v1427 * (self.scalar_v1422 * v3841)) - (v1423 * ((v436 * (if v1414 { v4 } else { (if v1410 { v4 } else { v3714 }) })) / v3861))) / v3868)) } else { v4 });
        let v3896: f64 = (if v1436 { (v1437 * self.scalar_v3887) } else { (if v1433 { (v1434 * self.scalar_v3887) } else { v3839 }) });
        let v3897: f64 = (if v1436 { v4 } else { (if v1433 { v4 } else { v3840 }) });
        let v3898: f64 = (if v1436 { (v1437 * self.scalar_v3888) } else { (if v1433 { (v1434 * self.scalar_v3888) } else { v3841 }) });
        let v3911: f64 = (if v1448 { (v1449 * self.scalar_v3902) } else { (if v1445 { (v1446 * self.scalar_v3902) } else { v3896 }) });
        let v3912: f64 = (if v1448 { (v1449 * self.scalar_v3903) } else { (if v1445 { (v1446 * self.scalar_v3903) } else { v3897 }) });
        let v3913: f64 = (if v1448 { v4 } else { (if v1445 { v4 } else { v3898 }) });
        let v3934: f64 = (if v1460 { v4 } else { (if v1457 { v4 } else { v3911 }) });
        let v3935: f64 = (if v1460 { (v1461 * self.scalar_v3917) } else { (if v1457 { (v1458 * self.scalar_v3917) } else { v3912 }) });
        let v3936: f64 = (if v1460 { (v1461 * self.scalar_v3918) } else { (if v1457 { (v1458 * self.scalar_v3918) } else { v3913 }) });
        let v3937: f64 = (if v1460 { (v1461 * self.scalar_v3919) } else { (if v1457 { (v1458 * self.scalar_v3919) } else { v4 }) });
        let v3938: f64 = (if v1460 { (v1461 * self.scalar_v3920) } else { (if v1457 { (v1458 * self.scalar_v3920) } else { v4 }) });
        let v3955: f64 = (if v1472 { (v1473 * self.scalar_v3944) } else { (if v1469 { (v1470 * self.scalar_v3944) } else { v3934 }) });
        let v3956: f64 = (if v1472 { (v1473 * self.scalar_v3945) } else { (if v1469 { (v1470 * self.scalar_v3945) } else { v3935 }) });
        let v3957: f64 = (if v1472 { v4 } else { (if v1469 { v4 } else { v3936 }) });
        let v3958: f64 = (if v1472 { v4 } else { (if v1469 { v4 } else { v3937 }) });
        let v3959: f64 = (if v1472 { v4 } else { (if v1469 { v4 } else { v3938 }) });
        let v3969: f64 = (v1484 * v1484);
        let v3976: f64 = (self.scalar_v583 * (-((-(self.scalar_v35 * (v32 * v3312))) / v3969)));
        let v3977: f64 = (self.scalar_v583 * (-((-(self.scalar_v35 * (v32 * v3313))) / v3969)));
        let v3988: f64 = (if v1483 { self.scalar_v3986 } else { v4 });
        let v3989: f64 = (if v1483 { self.scalar_v3987 } else { v4 });
        let v3990: f64 = (v1500 * v3988);
        let v3992: f64 = (v1500 * v3989);
        let v3994: f64 = (v32 * v1504);
        let v3999: f64 = (self.scalar_v1506 * f64::powf(v1504, self.scalar_v3997));
        let v4027: f64 = (v1507 * ((self.scalar_v33 * (-(self.scalar_v1511 * (v155 * v3988)))) - ((v1517 * ((v1515 * v3988) + (v1500 * (v462 * v3988)))) + (v1516 * v3988))));
        let v4030: f64 = (v1507 * ((self.scalar_v33 * (-(self.scalar_v1511 * (v155 * v3989)))) - ((v1517 * ((v1515 * v3989) + (v1500 * (v462 * v3989)))) + (v1516 * v3989))));
        let v4045: f64 = (v1526 * v1526);
        let v4046: f64 = (((v1526 * self.scalar_v4038) - (v1525 * (self.scalar_v132 * (if v1483 { (v1521 * ((v1519 * (((v3990 + v3990) / v3994) * v3999)) + v4027)) } else { v4 })))) / v4045);
        let v4050: f64 = (((v1526 * self.scalar_v4039) - (v1525 * (self.scalar_v132 * (if v1483 { (v1521 * ((v1519 * (((v3992 + v3992) / v3994) * v3999)) + v4030)) } else { v4 })))) / v4045);
        let v4051: f64 = (if v1483 { v4046 } else { v3988 });
        let v4052: f64 = (if v1483 { v4050 } else { v3989 });
        let v4066: f64 = (v1528 * v1528);
        let v4074: f64 = ((self.scalar_v0 * v1546) + (v1543 * (((v1528 * (-(if v1537 { (v1538 * v4051) } else { (if v1533 { (v1534 * v4051) } else { v4 }) }))) - (v1544 * v4051)) / v4066)));
        let v4077: f64 = ((v1546 * self.scalar_v2589) + (v1543 * (((v1528 * (-(if v1537 { (v1538 * v4052) } else { (if v1533 { (v1534 * v4052) } else { v4 }) }))) - (v1544 * v4052)) / v4066)));
        let v4104: f64 = (if v1550 { ((v1559 * ((v1551 * v4051) + (v1528 * self.scalar_v4080))) + (v1552 * ((v1557 * (v1553 * v4051)) + (v1554 * (v1555 * v4051))))) } else { (if v1532 { v4074 } else { v4 }) });
        let v4105: f64 = (if v1550 { ((v1559 * ((v1551 * v4052) + (v1528 * self.scalar_v4081))) + (v1552 * ((v1557 * (v1553 * v4052)) + (v1554 * (v1555 * v4052))))) } else { (if v1532 { v4077 } else { v4 }) });
        let v4116: f64 = ((v1564 * (if v1493 { (v1494 * v3976) } else { (if v1489 { (v1490 * v3976) } else { v4 }) })) + (v1498 * ((v1563 * v3312) + (v1151 * (self.scalar_v1562 * v4104)))));
        let v4119: f64 = ((v1564 * (if v1493 { (v1494 * v3977) } else { (if v1489 { (v1490 * v3977) } else { v4 }) })) + (v1498 * ((v1563 * v3313) + (v1151 * (self.scalar_v1562 * v4105)))));
        let v4133: f64 = (self.scalar_v1195 * f64::powf(v1577, self.scalar_v3404));
        let v4136: f64 = (if v1575 { (self.scalar_v4130 * v4133) } else { v4 });
        let v4137: f64 = (if v1575 { (self.scalar_v4131 * v4133) } else { v4 });
        let v4142: f64 = (v1580 * v1580);
        let v4149: f64 = (self.scalar_v605 * (-((-(self.scalar_v70 * (v32 * v4136))) / v4142)));
        let v4150: f64 = (self.scalar_v605 * (-((-(self.scalar_v70 * (v32 * v4137))) / v4142)));
        let v4159: f64 = (if v1575 { self.scalar_v4128 } else { v4 });
        let v4160: f64 = (if v1575 { self.scalar_v4129 } else { v4 });
        let v4161: f64 = (v1595 * v4159);
        let v4163: f64 = (v1595 * v4160);
        let v4165: f64 = (v32 * v1598);
        let v4170: f64 = (self.scalar_v1599 * f64::powf(v1598, self.scalar_v4168));
        let v4198: f64 = (v1600 * ((self.scalar_v68 * (-(self.scalar_v1604 * (v155 * v4159)))) - ((v1610 * ((v1608 * v4159) + (v1595 * (v462 * v4159)))) + (v1609 * v4159))));
        let v4201: f64 = (v1600 * ((self.scalar_v68 * (-(self.scalar_v1604 * (v155 * v4160)))) - ((v1610 * ((v1608 * v4160) + (v1595 * (v462 * v4160)))) + (v1609 * v4160))));
        let v4216: f64 = (v1618 * v1618);
        let v4217: f64 = (((v1618 * self.scalar_v4209) - (v1617 * (self.scalar_v154 * (if v1575 { (v1521 * ((v1612 * (((v4161 + v4161) / v4165) * v4170)) + v4198)) } else { v4 })))) / v4216);
        let v4221: f64 = (((v1618 * self.scalar_v4210) - (v1617 * (self.scalar_v154 * (if v1575 { (v1521 * ((v1612 * (((v4163 + v4163) / v4165) * v4170)) + v4201)) } else { v4 })))) / v4216);
        let v4222: f64 = (if v1575 { v4217 } else { v4159 });
        let v4223: f64 = (if v1575 { v4221 } else { v4160 });
        let v4237: f64 = (v1620 * v1620);
        let v4245: f64 = ((v1637 * self.scalar_v2589) + (v1634 * (((v1620 * (-(if v1628 { (v1629 * v4222) } else { (if v1624 { (v1625 * v4222) } else { v4 }) }))) - (v1635 * v4222)) / v4237)));
        let v4248: f64 = ((self.scalar_v0 * v1637) + (v1634 * (((v1620 * (-(if v1628 { (v1629 * v4223) } else { (if v1624 { (v1625 * v4223) } else { v4 }) }))) - (v1635 * v4223)) / v4237)));
        let v4273: f64 = (if v1641 { ((v1648 * ((v1642 * v4222) + (v1620 * self.scalar_v4081))) + (v1643 * ((v1646 * (v1553 * v4222)) + (v1644 * (v1555 * v4222))))) } else { (if v1623 { v4245 } else { v4 }) });
        let v4274: f64 = (if v1641 { ((v1648 * ((v1642 * v4223) + (v1620 * self.scalar_v4080))) + (v1643 * ((v1646 * (v1553 * v4223)) + (v1644 * (v1555 * v4223))))) } else { (if v1623 { v4248 } else { v4 }) });
        let v4285: f64 = ((v1653 * (if v1589 { (v1590 * v4149) } else { (if v1585 { (v1586 * v4149) } else { v4 }) })) + (v1594 * ((v1652 * v4136) + (v1579 * (self.scalar_v1651 * v4273)))));
        let v4288: f64 = ((v1653 * (if v1589 { (v1590 * v4150) } else { (if v1585 { (v1586 * v4150) } else { v4 }) })) + (v1594 * ((v1652 * v4137) + (v1579 * (self.scalar_v1651 * v4274)))));
        let v4297: f64 = (self.scalar_v1211 * v2627);
        let v4298: f64 = (self.scalar_v1211 * v2628);
        let v4299: f64 = (self.scalar_v1211 * v2629);
        let v4300: f64 = (self.scalar_v1211 * v2630);
        let v4301: f64 = (v436 * (if v873 { (v874 * self.scalar_v2593) } else { (if v870 { (v871 * self.scalar_v2593) } else { v4 }) }));
        let v4302: f64 = (v436 * (if v873 { (v874 * self.scalar_v2613) } else { (if v870 { (v871 * self.scalar_v2613) } else { v4 }) }));
        let v4303: f64 = (v436 * (if v873 { (v874 * self.scalar_v2614) } else { (if v870 { (v871 * self.scalar_v2614) } else { v4 }) }));
        let v4304: f64 = (v436 * (if v873 { (v874 * self.scalar_v2594) } else { (if v870 { (v871 * self.scalar_v2594) } else { v4 }) }));
        let v4305: f64 = (v32 * v1664);
        let v4313: f64 = (v1665 * v1665);
        let v4327: f64 = (v32 * v1668);
        let v4335: f64 = (v1669 * v1669);
        let v4357: f64 = (v32 * v1678);
        let v4365: f64 = (v1679 * v1679);
        let v4366: f64 = (((v1679 * (self.scalar_v1671 * v2627)) - (v1673 * ((self.scalar_v1675 * v2627) / v4357))) / v4365);
        let v4370: f64 = (((v1679 * (self.scalar_v1671 * v2628)) - (v1673 * ((self.scalar_v1675 * v2628) / v4357))) / v4365);
        let v4374: f64 = (((v1679 * (self.scalar_v1671 * v2629)) - (v1673 * ((self.scalar_v1675 * v2629) / v4357))) / v4365);
        let v4378: f64 = (((v1679 * (self.scalar_v1671 * v2630)) - (v1673 * ((self.scalar_v1675 * v2630) / v4357))) / v4365);
        let v4382: f64 = (self.scalar_v1685 * v2601);
        let v4384: f64 = (self.scalar_v1685 * v2602);
        let v4388: f64 = (self.scalar_v1689 * v2601);
        let v4390: f64 = (self.scalar_v1689 * v2602);
        let v4391: f64 = (v32 * v1695);
        let v4399: f64 = (v1696 * v1696);
        let v4421: f64 = (self.scalar_v1701 * v2627);
        let v4422: f64 = (self.scalar_v1701 * v2628);
        let v4424: f64 = (self.scalar_v1701 * v2629);
        let v4432: f64 = (self.scalar_v1689 * v2627);
        let v4433: f64 = (self.scalar_v1689 * v2628);
        let v4435: f64 = (self.scalar_v1689 * v2629);
        let v4437: f64 = (v32 * v1708);
        let v4447: f64 = (v1709 * v1709);
        let v4475: f64 = (v32 * v1717);
        let v4481: f64 = (v1718 * v1718);
        let v4488: f64 = (if self.scalar_v1712 { (((v1718 * v4382) - (v1714 * (v4388 / v4475))) / v4481) } else { (if self.scalar_v1682 { (((v1696 * v4382) - (v1687 * (v4388 / v4391))) / v4399) } else { v4 }) });
        let v4490: f64 = (if self.scalar_v1712 { (((v1718 * v4384) - (v1714 * (v4390 / v4475))) / v4481) } else { (if self.scalar_v1682 { (((v1696 * v4384) - (v1687 * (v4390 / v4391))) / v4399) } else { v4 }) });
        let v4493: f64 = (v32 * v1724);
        let v4501: f64 = (v1725 * v1725);
        let v4510: f64 = (((v1725 * v4424) - (v1721 * (v4435 / v4493))) / v4501);
        let v4515: f64 = (if self.scalar_v1712 { v4 } else { (if self.scalar_v1682 { (((v1709 * (self.scalar_v1701 * (-v2685))) - (v1703 * ((self.scalar_v1689 * (self.scalar_v1690 * v2685)) / v4437))) / v4447) } else { v4 }) });
        let v4516: f64 = (if self.scalar_v1712 { (((v1725 * v4421) - (v1721 * (v4432 / v4493))) / v4501) } else { (if self.scalar_v1682 { (((v1709 * v4421) - (v1703 * (v4432 / v4437))) / v4447) } else { v4 }) });
        let v4517: f64 = (if self.scalar_v1712 { (((v1725 * v4422) - (v1721 * (v4433 / v4493))) / v4501) } else { (if self.scalar_v1682 { (((v1709 * v4422) - (v1703 * (v4433 / v4437))) / v4447) } else { v4 }) });
        let v4518: f64 = (if self.scalar_v1712 { v4510 } else { (if self.scalar_v1682 { (((v1709 * (self.scalar_v1701 * (v2629 - v2686))) - (v1703 * ((self.scalar_v1689 * (v2629 + (self.scalar_v1690 * v2686))) / v4437))) / v4447) } else { v4 }) });
        let v4519: f64 = (if self.scalar_v1712 { v4510 } else { (if self.scalar_v1682 { (((v1709 * v4424) - (v1703 * (v4435 / v4437))) / v4447) } else { v4 }) });
        let v4520: f64 = (if self.scalar_v1712 { (((v1725 * (self.scalar_v1701 * v2630)) - (v1721 * ((self.scalar_v1689 * v2630) / v4493))) / v4501) } else { (if self.scalar_v1682 { (((v1709 * (self.scalar_v1701 * (v2630 - v2687))) - (v1703 * ((self.scalar_v1689 * (v2630 + (self.scalar_v1690 * v2687))) / v4437))) / v4447) } else { v4 }) });
        let v4525: f64 = (v32 * v1736);
        let v4531: f64 = (v1737 * v1737);
        let v4569: f64 = (v32 * v1755);
        let v4577: f64 = (v1756 * v1756);
        let v4591: f64 = (if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2652)) - (v1752 * ((self.scalar_v1675 * v2652) / v4569))) / v4577) } else { v4 });
        let v4592: f64 = (if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2653)) - (v1752 * ((self.scalar_v1675 * v2653) / v4569))) / v4577) } else { v4 });
        let v4593: f64 = (if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2654)) - (v1752 * ((self.scalar_v1675 * v2654) / v4569))) / v4577) } else { v4 });
        let v4594: f64 = (if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2655)) - (v1752 * ((self.scalar_v1675 * v2655) / v4569))) / v4577) } else { v4 });
        let v4598: f64 = (self.scalar_v1762 * v2652);
        let v4599: f64 = (self.scalar_v1762 * v2653);
        let v4602: f64 = (self.scalar_v1762 * v2654);
        let v4609: f64 = (self.scalar_v1766 * v2652);
        let v4610: f64 = (self.scalar_v1766 * v2653);
        let v4613: f64 = (self.scalar_v1766 * v2654);
        let v4615: f64 = (v32 * v1771);
        let v4625: f64 = (v1772 * v1772);
        let v4655: f64 = (v32 * v1779);
        let v4663: f64 = (v1780 * v1780);
        let v4672: f64 = (((v1780 * v4602) - (v1776 * (v4613 / v4655))) / v4663);
        let v4677: f64 = (if self.scalar_v1775 { (((v1780 * v4598) - (v1776 * (v4609 / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * v4598) - (v1764 * (v4609 / v4615))) / v4625) } else { v4 }) });
        let v4678: f64 = (if self.scalar_v1775 { (((v1780 * v4599) - (v1776 * (v4610 / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * v4599) - (v1764 * (v4610 / v4615))) / v4625) } else { v4 }) });
        let v4679: f64 = (if self.scalar_v1775 { v4 } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (-v2673))) - (v1764 * ((self.scalar_v1766 * (self.scalar_v1690 * v2673)) / v4615))) / v4625) } else { v4 }) });
        let v4680: f64 = (if self.scalar_v1775 { v4672 } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (v2654 - v2674))) - (v1764 * ((self.scalar_v1766 * (v2654 + (self.scalar_v1690 * v2674))) / v4615))) / v4625) } else { v4 }) });
        let v4681: f64 = (if self.scalar_v1775 { v4672 } else { (if self.scalar_v1759 { (((v1772 * v4602) - (v1764 * (v4613 / v4615))) / v4625) } else { v4 }) });
        let v4682: f64 = (if self.scalar_v1775 { (((v1780 * (self.scalar_v1762 * v2655)) - (v1776 * ((self.scalar_v1766 * v2655) / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (v2655 - v2675))) - (v1764 * ((self.scalar_v1766 * (v2655 + (self.scalar_v1690 * v2675))) / v4615))) / v4625) } else { v4 }) });
        let v4687: f64 = (v1795 * self.scalar_v4683);
        let v4688: f64 = (v4687 + v4687);
        let v4689: f64 = (v1795 * self.scalar_v4684);
        let v4691: f64 = (v1795 * self.scalar_v4685);
        let v4692: f64 = (v4691 + v4691);
        let v4693: f64 = (v1795 * self.scalar_v4686);
        let v4695: f64 = (if self.scalar_v1784 { v4688 } else { v4 });
        let v4696: f64 = (if self.scalar_v1784 { (v4689 + v4689) } else { v4 });
        let v4697: f64 = (if self.scalar_v1784 { v4 } else { v3538 });
        let v4698: f64 = (if self.scalar_v1784 { v4688 } else { v3540 });
        let v4699: f64 = (if self.scalar_v1784 { v4692 } else { v3542 });
        let v4700: f64 = (if self.scalar_v1784 { v4692 } else { v3544 });
        let v4701: f64 = (if self.scalar_v1784 { (v4693 + v4693) } else { v4 });
        let v4702: f64 = (if self.scalar_v1784 { v4692 } else { v4 });
        let v4703: f64 = (v32 * v1804);
        let v4704: f64 = (v4695 / v4703);
        let v4705: f64 = (v4696 / v4703);
        let v4706: f64 = (v4697 / v4703);
        let v4707: f64 = (v4698 / v4703);
        let v4708: f64 = (v4699 / v4703);
        let v4709: f64 = (v4700 / v4703);
        let v4710: f64 = (v4701 / v4703);
        let v4711: f64 = (v4702 / v4703);
        let v4721: f64 = (v1805 * v1805);
        let v4767: f64 = (if v1809 { (v424 * (self.scalar_v4683 + v4704)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4704 - self.scalar_v4683))) / v4721) } else { v4 }) });
        let v4768: f64 = (if v1809 { (v424 * (self.scalar_v4684 + v4705)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4705 - self.scalar_v4684))) / v4721) } else { v4 }) });
        let v4769: f64 = (if v1809 { (v424 * v4706) } else { (if v1801 { ((-(self.scalar_v1802 * v4706)) / v4721) } else { v4 }) });
        let v4770: f64 = (if v1809 { (v424 * (self.scalar_v4683 + v4707)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4707 - self.scalar_v4683))) / v4721) } else { v4 }) });
        let v4771: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4708)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4708 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4772: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4709)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4709 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4773: f64 = (if v1809 { (v424 * (self.scalar_v4686 + v4710)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4710 - self.scalar_v4686))) / v4721) } else { v4 }) });
        let v4774: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4711)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4711 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4780: f64 = (self.scalar_v340 * (v4591 + v4677));
        let v4783: f64 = (self.scalar_v340 * (v4593 + v4680));
        let v4796: f64 = (v1816 * v1816);
        let v4838: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4767) - (v1812 * (v4767 + v4780))) / v4796) } else { v4 }) });
        let v4839: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4768) - (v1812 * (v4768 + (self.scalar_v340 * (v4592 + v4678))))) / v4796) } else { v4 }) });
        let v4840: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { ((-(v1812 * (self.scalar_v340 * v4679))) / v4796) } else { v4 }) });
        let v4841: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4769) - (v1812 * v4769)) / v4796) } else { v4 }) });
        let v4842: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4770) - (v1812 * (v4770 + v4780))) / v4796) } else { v4 }) });
        let v4843: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4771) - (v1812 * (v4771 + v4783))) / v4796) } else { v4 }) });
        let v4844: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4772) - (v1812 * (v4772 + (self.scalar_v340 * (v4593 + v4681))))) / v4796) } else { v4 }) });
        let v4845: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4773) - (v1812 * (v4773 + (self.scalar_v340 * (v4594 + v4682))))) / v4796) } else { v4 }) });
        let v4846: f64 = (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4774) - (v1812 * (v4774 + v4783))) / v4796) } else { v4 }) });
        let v4847: f64 = (v1821 * v4591);
        let v4857: f64 = (v1821 * v4593);
        let v4876: f64 = (v1821 * v4677);
        let v4888: f64 = (v1821 * v4680);
        let v4914: f64 = (v1829 * self.scalar_v4908);
        let v4916: f64 = (v1829 * self.scalar_v4909);
        let v4918: f64 = (v1829 * self.scalar_v4910);
        let v4929: f64 = (v32 * v1838);
        let v4930: f64 = ((if self.scalar_v1827 { v4 } else { v4695 }) / v4929);
        let v4931: f64 = ((if self.scalar_v1827 { v4 } else { v4696 }) / v4929);
        let v4932: f64 = ((if self.scalar_v1827 { v4 } else { v4697 }) / v4929);
        let v4933: f64 = ((if self.scalar_v1827 { (v4914 + v4914) } else { v4695 }) / v4929);
        let v4934: f64 = ((if self.scalar_v1827 { (v4916 + v4916) } else { v4698 }) / v4929);
        let v4935: f64 = ((if self.scalar_v1827 { (v4918 + v4918) } else { v4699 }) / v4929);
        let v4936: f64 = ((if self.scalar_v1827 { v4 } else { v4700 }) / v4929);
        let v4937: f64 = ((if self.scalar_v1827 { v4 } else { v4701 }) / v4929);
        let v4938: f64 = ((if self.scalar_v1827 { v4 } else { v4702 }) / v4929);
        let v4944: f64 = (v1839 * v1839);
        let v4991: f64 = (if v1843 { (v424 * v4930) } else { (if v1835 { ((-(self.scalar_v1836 * v4930)) / v4944) } else { v4 }) });
        let v4992: f64 = (if v1843 { (v424 * v4931) } else { (if v1835 { ((-(self.scalar_v1836 * v4931)) / v4944) } else { v4 }) });
        let v4993: f64 = (if v1843 { (v424 * v4932) } else { (if v1835 { ((-(self.scalar_v1836 * v4932)) / v4944) } else { v4 }) });
        let v4994: f64 = (if v1843 { (v424 * (self.scalar_v4911 + v4933)) } else { (if v1835 { ((-(self.scalar_v1836 * (v4933 - self.scalar_v4911))) / v4944) } else { v4 }) });
        let v4995: f64 = (if v1843 { (v424 * (self.scalar_v4912 + v4934)) } else { (if v1835 { ((-(self.scalar_v1836 * (v4934 - self.scalar_v4912))) / v4944) } else { v4 }) });
        let v4996: f64 = (if v1843 { (v424 * (self.scalar_v4913 + v4935)) } else { (if v1835 { ((-(self.scalar_v1836 * (v4935 - self.scalar_v4913))) / v4944) } else { v4 }) });
        let v4997: f64 = (if v1843 { (v424 * v4936) } else { (if v1835 { ((-(self.scalar_v1836 * v4936)) / v4944) } else { v4 }) });
        let v4998: f64 = (if v1843 { (v424 * v4937) } else { (if v1835 { ((-(self.scalar_v1836 * v4937)) / v4944) } else { v4 }) });
        let v4999: f64 = (if v1843 { (v424 * v4938) } else { (if v1835 { ((-(self.scalar_v1836 * v4938)) / v4944) } else { v4 }) });
        let v5010: f64 = (self.scalar_v1847 * f64::powf(v1864, self.scalar_v1856));
        let v5020: f64 = (v1866 * v1866);
        let v5057: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4991) } else { (if v1863 { (((v4991 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5058: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4992) } else { (if v1863 { (((v4992 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5059: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4993) } else { (if v1863 { (((v4993 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5060: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4994) } else { (if v1863 { (((v4994 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5061: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4995) } else { (if v1863 { (((v4995 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5062: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4996) } else { (if v1863 { (((v4996 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5063: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4997) } else { (if v1863 { (((v4997 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5064: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4998) } else { (if v1863 { (((v4998 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5065: f64 = (if self.scalar_v1875 { v4 } else { (if v1870 { (self.scalar_v1861 * v4999) } else { (if v1863 { (((v4999 / self.scalar_v1852) * v5010) / v5020) } else { v4 }) }) });
        let v5088: f64 = (v1876 * (if self.scalar_v1744 { (self.scalar_v14 * v4374) } else { v4374 }));
        let v5108: f64 = (v1876 * (self.scalar_v486 * v3937));
        let v5117: f64 = (v1876 * (if self.scalar_v1744 { (v4847 + (v1758 * v4838)) } else { v4 }));
        let v5144: f64 = (v1228 * v3490);
        let v5146: f64 = (v1228 * v3495);
        let v5148: f64 = (v1228 * v3493);
        let v5150: f64 = (v1228 * v3494);
        let v5152: f64 = (v32 * v1884);
        let v5153: f64 = ((v5144 + v5144) / v5152);
        let v5154: f64 = ((v5146 + v5146) / v5152);
        let v5155: f64 = ((v5148 + v5148) / v5152);
        let v5156: f64 = ((v5150 + v5150) / v5152);
        let v5163: f64 = (v1885 * v1885);
        let v5186: f64 = (if v1888 { (v424 * (v3490 + v5153)) } else { (if v1882 { ((-(v1250 * (v5153 - v3490))) / v5163) } else { v4 }) });
        let v5187: f64 = (if v1888 { (v424 * (v3495 + v5154)) } else { (if v1882 { ((-(v1250 * (v5154 - v3495))) / v5163) } else { v4 }) });
        let v5188: f64 = (if v1888 { (v424 * (v3493 + v5155)) } else { (if v1882 { ((-(v1250 * (v5155 - v3493))) / v5163) } else { v4 }) });
        let v5189: f64 = (if v1888 { (v424 * (v3494 + v5156)) } else { (if v1882 { ((-(v1250 * (v5156 - v3494))) / v5163) } else { v4 }) });
        let v5204: f64 = (v1892 * v1892);
        let v5219: f64 = (v155 * (if v1894 { v4 } else { ((-(self.scalar_v328 * ((v1891 * v3584) + (v1262 * v5186)))) / v5204) }));
        let v5220: f64 = (v155 * (if v1894 { v4 } else { ((-(self.scalar_v328 * ((v1891 * v3585) + (v1262 * v5187)))) / v5204) }));
        let v5221: f64 = (v155 * (if v1894 { v4 } else { ((-(self.scalar_v328 * ((v1891 * v3586) + (v1262 * v5188)))) / v5204) }));
        let v5222: f64 = (v155 * (if v1894 { v4 } else { ((-(self.scalar_v328 * ((v1891 * v3587) + (v1262 * v5189)))) / v5204) }));
        let v5229: f64 = (v1896 * v1896);
        let v5246: f64 = ((-v3612) / self.scalar_v1907);
        let v5247: f64 = ((-v3616) / self.scalar_v1907);
        let v5248: f64 = ((-v3620) / self.scalar_v1907);
        let v5249: f64 = ((-v3624) / self.scalar_v1907);
        let v5274: f64 = (if v1911 { (v1922 * (if v1916 { (v1917 * v5246) } else { (if v1912 { (v1913 * v5246) } else { v4 }) })) } else { v4 });
        let v5275: f64 = (if v1911 { ((v1922 * (if v1916 { (v1917 * v5247) } else { (if v1912 { (v1913 * v5247) } else { v4 }) })) + (v1921 * self.scalar_v2589)) } else { v4 });
        let v5276: f64 = (if v1911 { ((v1922 * (if v1916 { (v1917 * v5248) } else { (if v1912 { (v1913 * v5248) } else { v4 }) })) + (self.scalar_v0 * v1921)) } else { v4 });
        let v5277: f64 = (if v1911 { (v1922 * (if v1916 { (v1917 * v5249) } else { (if v1912 { (v1913 * v5249) } else { v4 }) })) } else { v4 });
        let v5280: f64 = (self.scalar_v1926 * f64::powf(v1924, self.scalar_v5278));
        let v5285: f64 = (self.scalar_v1925 * (v5274 * v5280));
        let v5286: f64 = (self.scalar_v1925 * (v5275 * v5280));
        let v5287: f64 = (self.scalar_v1925 * (v5276 * v5280));
        let v5288: f64 = (self.scalar_v1925 * (v5277 * v5280));
        let v5301: f64 = (if v1934 { (v1935 * v5285) } else { (if v1930 { (v1931 * v5285) } else { v4 }) });
        let v5302: f64 = (if v1934 { (v1935 * v5286) } else { (if v1930 { (v1931 * v5286) } else { v4 }) });
        let v5303: f64 = (if v1934 { (v1935 * v5287) } else { (if v1930 { (v1931 * v5287) } else { v4 }) });
        let v5304: f64 = (if v1934 { (v1935 * v5288) } else { (if v1930 { (v1931 * v5288) } else { v4 }) });
        let v5328: f64 = (v1125 * v1125);
        let v5337: f64 = (if v1950 { (((v1125 * self.scalar_v2589) - (v1957 * v3278)) / v5328) } else { v3069 });
        let v5338: f64 = (if v1950 { (((self.scalar_v0 * v1125) - (v1957 * v3279)) / v5328) } else { v3070 });
        let v5339: f64 = (if v1950 { ((-(v1957 * v3280)) / v5328) } else { v3071 });
        let v5346: f64 = (v32 * v1962);
        let v5350: f64 = (if v1950 { (((v32 * v5337) / v1956) / v5346) } else { v4 });
        let v5351: f64 = (if v1950 { (((v32 * v5338) / v1956) / v5346) } else { v4 });
        let v5352: f64 = (if v1950 { (((v32 * v5339) / v1956) / v5346) } else { v4 });
        let v5359: f64 = (if v1969 { (-(v424 * v3260)) } else { v4 });
        let v5360: f64 = (if v1969 { (-(v424 * v3261)) } else { v4 });
        let v5361: f64 = (if v1969 { (-(v424 * v3262)) } else { v4 });
        let v5374: f64 = (if v1969 { ((v1973 * v5359) + (v1972 * (self.scalar_v1953 * v5359))) } else { v4 });
        let v5375: f64 = (if v1969 { ((v1973 * v5360) + (v1972 * (self.scalar_v1953 * v5360))) } else { v4 });
        let v5376: f64 = (if v1969 { ((v1973 * v5361) + (v1972 * (self.scalar_v1953 * v5361))) } else { v4 });
        let v5386: f64 = (v1963 * v5350);
        let v5388: f64 = (v1963 * v5351);
        let v5390: f64 = (v1963 * v5352);
        let v5392: f64 = (v1975 * v5374);
        let v5394: f64 = (v1975 * v5375);
        let v5396: f64 = (v1975 * v5376);
        let v5401: f64 = (v32 * v1980);
        let v5408: f64 = (v1980 * v1980);
        let v5418: f64 = (if v1950 { (((v1980 * ((v1975 * v5350) + (v1963 * v5374))) - (v1976 * (((v5386 + v5386) + (v5392 + v5392)) / v5401))) / v5408) } else { v4 });
        let v5419: f64 = (if v1950 { (((v1980 * ((v1975 * v5351) + (v1963 * v5375))) - (v1976 * (((v5388 + v5388) + (v5394 + v5394)) / v5401))) / v5408) } else { v4 });
        let v5420: f64 = (if v1950 { (((v1980 * ((v1975 * v5352) + (v1963 * v5376))) - (v1976 * (((v5390 + v5390) + (v5396 + v5396)) / v5401))) / v5408) } else { v4 });
        let v5424: f64 = (v1982 * v1982);
        let v5433: f64 = (if v1950 { (((v1982 * self.scalar_v2589) - (v1957 * v5418)) / v5424) } else { v4 });
        let v5434: f64 = (if v1950 { (((self.scalar_v0 * v1982) - (v1957 * v5419)) / v5424) } else { v4 });
        let v5435: f64 = (if v1950 { ((-(v1957 * v5420)) / v5424) } else { v4 });
        let v5436: f64 = (v424 * v5418);
        let v5437: f64 = (v424 * v5419);
        let v5438: f64 = (v424 * v5420);
        let v5439: f64 = (v1956 * v5436);
        let v5440: f64 = (v1956 * v5437);
        let v5441: f64 = (v1956 * v5438);
        let v5454: f64 = (if v1950 { (v5433 + ((v1986 * v3278) + (v1125 * v5439))) } else { v4 });
        let v5455: f64 = (if v1950 { (v5434 + ((v1986 * v3279) + (v1125 * v5440))) } else { v4 });
        let v5456: f64 = (if v1950 { (v5435 + ((v1986 * v3280) + (v1125 * v5441))) } else { v4 });
        let v5476: f64 = (v2002 * v2002);
        let v5493: f64 = ((v2004 * v5439) + (v1986 * (-(((v2002 * v3616) - (v1269 * (self.scalar_v963 * (if v1969 { (self.scalar_v1992 * (v32 * v3260)) } else { v4 })))) / v5476))));
        let v5496: f64 = ((v2004 * v5440) + (v1986 * (-(((v2002 * v3620) - (v1269 * (self.scalar_v963 * (if v1969 { (self.scalar_v1992 * (v32 * v3261)) } else { v4 })))) / v5476))));
        let v5499: f64 = ((v2004 * v5441) + (v1986 * (-(((v2002 * v3624) - (v1269 * (self.scalar_v963 * (if v1969 { (self.scalar_v1992 * (v32 * v3262)) } else { v4 })))) / v5476))));
        let v5504: f64 = (if v1969 { (-(v1986 * (-(v3612 / v2002)))) } else { v4 });
        let v5505: f64 = (if v1969 { (v5433 - v5493) } else { v4 });
        let v5506: f64 = (if v1969 { (v5434 - v5496) } else { v4 });
        let v5507: f64 = (if v1969 { (v5435 - v5499) } else { v4 });
        let v5511: f64 = (v2008 * v5504);
        let v5513: f64 = (v2008 * (v5505 - v5454));
        let v5515: f64 = (v2008 * (v5506 - v5455));
        let v5517: f64 = (v2008 * (v5507 - v5456));
        let v5553: f64 = (v32 * v2017);
        let v5555: f64 = ((if v1969 { ((v5513 + v5513) + (((v2011 * v3269) + (v1122 * ((v2010 * v5433) + (v1984 * (v47 * v5433))))) / self.scalar_v963)) } else { v5337 }) / v5553);
        let v5556: f64 = ((if v1969 { ((v5515 + v5515) + (((v2011 * v3270) + (v1122 * ((v2010 * v5434) + (v1984 * (v47 * v5434))))) / self.scalar_v963)) } else { v5338 }) / v5553);
        let v5557: f64 = ((if v1969 { ((v5517 + v5517) + (((v2011 * v3271) + (v1122 * ((v2010 * v5435) + (v1984 * (v47 * v5435))))) / self.scalar_v963)) } else { v5339 }) / v5553);
        let v5566: f64 = (if v1969 { (v424 * (v5504 + ((if v1969 { (v5511 + v5511) } else { v4 }) / v5553))) } else { v4 });
        let v5567: f64 = (if v1969 { (v424 * ((v5454 + v5505) + v5555)) } else { (if v1966 { v5454 } else { v4 }) });
        let v5568: f64 = (if v1969 { (v424 * ((v5455 + v5506) + v5556)) } else { (if v1966 { v5455 } else { v4 }) });
        let v5569: f64 = (if v1969 { (v424 * ((v5456 + v5507) + v5557)) } else { (if v1966 { v5456 } else { v4 }) });
        let v5576: f64 = (v2020 * v2020);
        let v5596: f64 = (v2023 * v2023);
        let v5610: f64 = (if v2027 { ((-(v1985 * (if v1950 { (((v2020 * v5566) - (v2021 * v5566)) / v5576) } else { v4 }))) / v5596) } else { v4 });
        let v5611: f64 = (if v2027 { (((v2023 * v5436) - (v1985 * (if v1950 { (((v2020 * (v5567 - v5433)) - (v2021 * v5567)) / v5576) } else { v4 }))) / v5596) } else { v4 });
        let v5612: f64 = (if v2027 { (((v2023 * v5437) - (v1985 * (if v1950 { (((v2020 * (v5568 - v5434)) - (v2021 * v5568)) / v5576) } else { v4 }))) / v5596) } else { v4 });
        let v5613: f64 = (if v2027 { (((v2023 * v5438) - (v1985 * (if v1950 { (((v2020 * (v5569 - v5435)) - (v2021 * v5569)) / v5576) } else { v4 }))) / v5596) } else { v4 });
        let v5632: f64 = ((-(self.scalar_v2033 * v5566)) / v5576);
        let v5635: f64 = ((-(self.scalar_v2033 * v5567)) / v5576);
        let v5638: f64 = ((-(self.scalar_v2033 * v5568)) / v5576);
        let v5641: f64 = ((-(self.scalar_v2033 * v5569)) / v5576);
        let v5642: f64 = (v2035 * v5632);
        let v5643: f64 = (v2035 * v5635);
        let v5644: f64 = (v2035 * v5638);
        let v5645: f64 = (v2035 * v5641);
        let v5648: f64 = (v2029 * v2029);
        let v5684: f64 = ((v2040 * ((v2031 * v5610) + (v2029 * (self.scalar_v2030 * v5566)))) + (v2032 * (v5642 - (v2039 * ((v2037 * v5632) + (v2034 * ((-(v1975 * v5610)) / v5648)))))));
        let v5687: f64 = ((v2040 * ((v2031 * v5611) + (v2029 * (self.scalar_v2030 * v5567)))) + (v2032 * (v5643 - (v2039 * ((v2037 * v5635) + (v2034 * (((v2029 * v5374) - (v1975 * v5611)) / v5648)))))));
        let v5690: f64 = ((v2040 * ((v2031 * v5612) + (v2029 * (self.scalar_v2030 * v5568)))) + (v2032 * (v5644 - (v2039 * ((v2037 * v5638) + (v2034 * (((v2029 * v5375) - (v1975 * v5612)) / v5648)))))));
        let v5693: f64 = ((v2040 * ((v2031 * v5613) + (v2029 * (self.scalar_v2030 * v5569)))) + (v2032 * (v5645 - (v2039 * ((v2037 * v5641) + (v2034 * (((v2029 * v5376) - (v1975 * v5613)) / v5648)))))));
        let v5712: f64 = (if v2044 { ((v2045 * v5643) + (v2035 * (self.scalar_v10 * v5374))) } else { (if v2027 { v5687 } else { (if v1911 { ((v1942 * v5302) + (v1939 * (self.scalar_v1941 * v5275))) } else { v4 }) }) });
        let v5713: f64 = (if v2044 { ((v2045 * v5644) + (v2035 * (self.scalar_v10 * v5375))) } else { (if v2027 { v5690 } else { (if v1911 { ((v1942 * v5303) + (v1939 * (self.scalar_v1941 * v5276))) } else { v4 }) }) });
        let v5714: f64 = (if v2044 { ((v2045 * v5645) + (v2035 * (self.scalar_v10 * v5376))) } else { (if v2027 { v5693 } else { (if v1911 { ((v1942 * v5304) + (v1939 * (self.scalar_v1941 * v5277))) } else { v4 }) }) });
        let v5716: f64 = (self.scalar_v1926 * f64::powf(v1922, self.scalar_v5278));
        let v5722: f64 = (v2055 * v2055);
        let v5742: f64 = (self.scalar_v2058 * f64::powf(v2057, self.scalar_v5740));
        let v5755: f64 = (if v2052 { (v2053 * ((-(((v2055 * v3612) - (v1269 * v3612)) / v5722)) * v5742)) } else { v4 });
        let v5756: f64 = (if v2052 { ((v2059 * (self.scalar_v2589 * v5716)) + (v2053 * ((-(((v2055 * v3616) - (v1269 * v3616)) / v5722)) * v5742))) } else { v4 });
        let v5757: f64 = (if v2052 { ((v2059 * (self.scalar_v0 * v5716)) + (v2053 * ((-(((v2055 * v3620) - (v1269 * v3620)) / v5722)) * v5742))) } else { v4 });
        let v5758: f64 = (if v2052 { (v2053 * ((-(((v2055 * v3624) - (v1269 * v3624)) / v5722)) * v5742)) } else { v4 });
        let v5767: f64 = (if v2064 { (v3612 / self.scalar_v2054) } else { v4 });
        let v5768: f64 = (if v2064 { (v3616 / self.scalar_v2054) } else { v4 });
        let v5769: f64 = (if v2064 { (v3620 / self.scalar_v2054) } else { v4 });
        let v5770: f64 = (if v2064 { (v3624 / self.scalar_v2054) } else { v4 });
        let v5775: f64 = (if v2064 { (v5767 / self.scalar_v2070) } else { self.scalar_v3659 });
        let v5776: f64 = (if v2064 { (v5768 / self.scalar_v2070) } else { self.scalar_v3660 });
        let v5777: f64 = (if v2064 { (v5769 / self.scalar_v2070) } else { v4 });
        let v5778: f64 = (if v2064 { (v5770 / self.scalar_v2070) } else { v4 });
        let v5821: f64 = (self.scalar_v2090 * f64::powf(v2089, self.scalar_v5819));
        let v5827: f64 = (v2061 * ((if v2082 { (v5767 + (self.scalar_v2070 * ((v2084 * (-v5775)) / v2085))) } else { (if v2074 { (self.scalar_v2070 * ((v2075 * v5775) / v2076)) } else { v4 }) }) * v5821));
        let v5830: f64 = (v2061 * ((if v2082 { (v5768 + (self.scalar_v2070 * ((v2084 * (-v5776)) / v2085))) } else { (if v2074 { (self.scalar_v2070 * ((v2075 * v5776) / v2076)) } else { v4 }) }) * v5821));
        let v5833: f64 = (v2061 * ((if v2082 { (v5769 + (self.scalar_v2070 * ((v2084 * (-v5777)) / v2085))) } else { (if v2074 { (self.scalar_v2070 * ((v2075 * v5777) / v2076)) } else { v4 }) }) * v5821));
        let v5836: f64 = (v2061 * ((if v2082 { (v5770 + (self.scalar_v2070 * ((v2084 * (-v5778)) / v2085))) } else { (if v2074 { (self.scalar_v2070 * ((v2075 * v5778) / v2076)) } else { v4 }) }) * v5821));
        let v5842: f64 = (self.scalar_v1925 * (if v2064 { ((v2091 * v5755) + v5827) } else { (if v2062 { v5755 } else { v4 }) }));
        let v5843: f64 = (self.scalar_v1925 * (if v2064 { ((v2091 * v5756) + v5830) } else { (if v2062 { v5756 } else { v4 }) }));
        let v5844: f64 = (self.scalar_v1925 * (if v2064 { ((v2091 * v5757) + v5833) } else { (if v2062 { v5757 } else { v4 }) }));
        let v5845: f64 = (self.scalar_v1925 * (if v2064 { ((v2091 * v5758) + v5836) } else { (if v2062 { v5758 } else { v4 }) }));
        let v5872: f64 = (if v2052 { (v2106 * (if v2100 { (v2101 * v5842) } else { (if v2096 { (v2097 * v5842) } else { v5301 }) })) } else { (if v2044 { (v2045 * v5642) } else { (if v2027 { v5684 } else { (if v1911 { ((v1942 * v5301) + (v1939 * (self.scalar_v1941 * v5274))) } else { v4 }) }) }) });
        let v5873: f64 = (if v2052 { ((v2106 * (if v2100 { (v2101 * v5843) } else { (if v2096 { (v2097 * v5843) } else { v5302 }) })) + (v2105 * self.scalar_v5862)) } else { v5712 });
        let v5874: f64 = (if v2052 { ((v2106 * (if v2100 { (v2101 * v5844) } else { (if v2096 { (v2097 * v5844) } else { v5303 }) })) + (v2105 * self.scalar_v5863)) } else { v5713 });
        let v5875: f64 = (if v2052 { (v2106 * (if v2100 { (v2101 * v5845) } else { (if v2096 { (v2097 * v5845) } else { v5304 }) })) } else { v5714 });
        let v5890: f64 = (v2115 * v2115);
        let v5915: f64 = (v2114 * v2114);
        let v5926: f64 = ((((-(self.scalar_v104 * ((v2114 * v3612) + (v1269 * v5219)))) / v5890) + (self.scalar_v500 * (v3590 / self.scalar_v449))) + ((-(self.scalar_v321 * v5219)) / v5915));
        let v5927: f64 = ((((-(self.scalar_v104 * ((v2114 * v3616) + (v1269 * v5220)))) / v5890) + (self.scalar_v500 * (v3593 / self.scalar_v449))) + ((-(self.scalar_v321 * v5220)) / v5915));
        let v5928: f64 = ((((-(self.scalar_v104 * ((v2114 * v3620) + (v1269 * v5221)))) / v5890) + (self.scalar_v500 * (v3596 / self.scalar_v449))) + ((-(self.scalar_v321 * v5221)) / v5915));
        let v5929: f64 = ((((-(self.scalar_v104 * ((v2114 * v3624) + (v1269 * v5222)))) / v5890) + (self.scalar_v500 * (v3599 / self.scalar_v449))) + ((-(self.scalar_v321 * v5222)) / v5915));
        let v5930: f64 = (if v2113 { v5926 } else { v4 });
        let v5931: f64 = (if v2113 { v5927 } else { v4 });
        let v5932: f64 = (if v2113 { v5928 } else { v4 });
        let v5933: f64 = (if v2113 { v5929 } else { v4 });
        let v5942: f64 = (if v2123 { ((v5872 - v5930) / v421) } else { v5775 });
        let v5943: f64 = (if v2123 { ((v5873 - v5931) / v421) } else { v5776 });
        let v5944: f64 = (if v2123 { ((v5874 - v5932) / v421) } else { v5777 });
        let v5945: f64 = (if v2123 { ((v5875 - v5933) / v421) } else { v5778 });
        let v5986: f64 = (if v2136 { (v5930 - (v421 * ((v2138 * (-v5942)) / v2139))) } else { (if v2128 { (v5872 - (v421 * ((v2129 * v5942) / v2130))) } else { v5872 }) });
        let v5987: f64 = (if v2136 { (v5931 - (v421 * ((v2138 * (-v5943)) / v2139))) } else { (if v2128 { (v5873 - (v421 * ((v2129 * v5943) / v2130))) } else { v5873 }) });
        let v5988: f64 = (if v2136 { (v5932 - (v421 * ((v2138 * (-v5944)) / v2139))) } else { (if v2128 { (v5874 - (v421 * ((v2129 * v5944) / v2130))) } else { v5874 }) });
        let v5989: f64 = (if v2136 { (v5933 - (v421 * ((v2138 * (-v5945)) / v2139))) } else { (if v2128 { (v5875 - (v421 * ((v2129 * v5945) / v2130))) } else { v5875 }) });
        let v5992: f64 = ((v2143 * v3612) + (v1269 * v5986));
        let v5995: f64 = ((v2143 * v3616) + (v1269 * v5987));
        let v5998: f64 = ((v2143 * v3620) + (v1269 * v5988));
        let v6001: f64 = ((v2143 * v3624) + (v1269 * v5989));
        let v6025: f64 = (v2149 * v2149);
        let v6043: f64 = (if v2153 { v5992 } else { (if v2147 { (((v2149 * ((v2144 * v5930) + (v2122 * v5992))) - (v2148 * (v5930 + v5986))) / v6025) } else { (if v2123 { v5992 } else { v4 }) }) });
        let v6044: f64 = (if v2153 { v5995 } else { (if v2147 { (((v2149 * ((v2144 * v5931) + (v2122 * v5995))) - (v2148 * (v5931 + v5987))) / v6025) } else { (if v2123 { v5995 } else { v4 }) }) });
        let v6045: f64 = (if v2153 { v5998 } else { (if v2147 { (((v2149 * ((v2144 * v5932) + (v2122 * v5998))) - (v2148 * (v5932 + v5988))) / v6025) } else { (if v2123 { v5998 } else { v4 }) }) });
        let v6046: f64 = (if v2153 { v6001 } else { (if v2147 { (((v2149 * ((v2144 * v5933) + (v2122 * v6001))) - (v2148 * (v5933 + v5989))) / v6025) } else { (if v2123 { v6001 } else { v4 }) }) });
        let v6067: f64 = (if v2168 { (-(self.scalar_v1130 * ((v2170 * self.scalar_v3293) / v2171))) } else { (if v2161 { (self.scalar_v2589 - (self.scalar_v1130 * ((v2162 * self.scalar_v3281) / v2163))) } else { v4 }) });
        let v6068: f64 = (if v2168 { (-(self.scalar_v1130 * ((v2170 * self.scalar_v3294) / v2171))) } else { (if v2161 { (self.scalar_v0 - (self.scalar_v1130 * ((v2162 * self.scalar_v3282) / v2163))) } else { v4 }) });
        let v6074: f64 = (self.scalar_v1150 * f64::powf(v2178, self.scalar_v3309));
        let v6096: f64 = ((v2191 * v5186) + (v1891 * (self.scalar_v2190 * v3459)));
        let v6099: f64 = ((v2191 * v5187) + (v1891 * (self.scalar_v2190 * v3463)));
        let v6100: f64 = (v2191 * v5188);
        let v6101: f64 = (v2191 * v5189);
        let v6105: f64 = (v2193 * v5186);
        let v6108: f64 = ((v2193 * v5187) + (v1891 * (self.scalar_v2190 * v3481)));
        let v6111: f64 = ((v2193 * v5188) + (v1891 * (self.scalar_v2190 * v3485)));
        let v6114: f64 = ((v2193 * v5189) + (v1891 * (self.scalar_v2190 * v3489)));
        let v6159: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6139) / v2207))) } else { (if v2197 { (self.scalar_v0 - (self.scalar_v1075 * ((v2198 * self.scalar_v6115) / v2199))) } else { v4 }) });
        let v6160: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6140) / v2207))) } else { (if v2197 { (self.scalar_v2590 - (self.scalar_v1075 * ((v2198 * self.scalar_v6116) / v2199))) } else { v4 }) });
        let v6161: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6141) / v2207))) } else { (if v2197 { (self.scalar_v2591 - (self.scalar_v1075 * ((v2198 * self.scalar_v6117) / v2199))) } else { v4 }) });
        let v6162: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6142) / v2207))) } else { (if v2197 { (self.scalar_v2589 - (self.scalar_v1075 * ((v2198 * self.scalar_v6118) / v2199))) } else { v4 }) });
        let v6172: f64 = (self.scalar_v1195 * f64::powf(v2213, self.scalar_v3404));
        let v6215: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3446 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6159 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v0 - v6159))))))));
        let v6216: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6160 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2590 - v6160)))) + self.scalar_v6201))));
        let v6217: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6161 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2591 - v6161)))) + self.scalar_v6202))));
        let v6218: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3447 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6162 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2589 - v6162))))))));
        let v6257: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6140) / v2239))) } else { (if v2229 { (self.scalar_v2590 - (self.scalar_v1075 * ((v2230 * self.scalar_v6116) / v2231))) } else { v4 }) });
        let v6258: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6240) / v2239))) } else { (if v2229 { (self.scalar_v2592 - (self.scalar_v1075 * ((v2230 * self.scalar_v6219) / v2231))) } else { v4 }) });
        let v6259: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6141) / v2239))) } else { (if v2229 { (self.scalar_v2591 - (self.scalar_v1075 * ((v2230 * self.scalar_v6117) / v2231))) } else { v4 }) });
        let v6260: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6142) / v2239))) } else { (if v2229 { (self.scalar_v2589 - (self.scalar_v1075 * ((v2230 * self.scalar_v6118) / v2231))) } else { v4 }) });
        let v6270: f64 = (self.scalar_v1195 * f64::powf(v2245, self.scalar_v3404));
        let v6312: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v6201 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6257 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2590 - v6257))))))));
        let v6313: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6258 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2592 - v6258)))) + self.scalar_v6299))));
        let v6314: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v6202 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6259 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2591 - v6259))))))));
        let v6315: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3447 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6260 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2589 - v6260))))))));
        let v6338: f64 = (if v2272 { (-(self.scalar_v2258 * ((v2274 * self.scalar_v6328) / v2275))) } else { (if v2265 { (self.scalar_v0 - (self.scalar_v2258 * ((v2266 * self.scalar_v6316) / v2267))) } else { v4 }) });
        let v6339: f64 = (if v2272 { (-(self.scalar_v2258 * ((v2274 * self.scalar_v6329) / v2275))) } else { (if v2265 { (self.scalar_v2589 - (self.scalar_v2258 * ((v2266 * self.scalar_v6317) / v2267))) } else { v4 }) });
        let v6346: f64 = (self.scalar_v2280 * f64::powf(v2283, self.scalar_v6344));
        let v6377: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * self.scalar_v6361) } else { (if v2299 { (v2300 * self.scalar_v6361) } else { v3955 }) }));
        let v6378: f64 = (self.scalar_v2296 * (if v2302 { v4 } else { (if v2299 { v4 } else { v3956 }) }));
        let v6379: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * self.scalar_v6362) } else { (if v2299 { (v2300 * self.scalar_v6362) } else { v3957 }) }));
        let v6380: f64 = (self.scalar_v2296 * (if v2302 { v4 } else { (if v2299 { v4 } else { v3958 }) }));
        let v6381: f64 = (self.scalar_v2296 * (if v2302 { v4 } else { (if v2299 { v4 } else { v3959 }) }));
        let v6402: f64 = ((self.scalar_v2189 * (((v1665 * v4297) - (v1662 * (v4297 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4301) - (v1661 * (v4301 / v4327))) / v4335)));
        let v6403: f64 = ((self.scalar_v2189 * (((v1665 * v4298) - (v1662 * (v4298 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4302) - (v1661 * (v4302 / v4327))) / v4335)));
        let v6404: f64 = ((self.scalar_v2189 * (((v1665 * v4299) - (v1662 * (v4299 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4303) - (v1661 * (v4303 / v4327))) / v4335)));
        let v6405: f64 = ((self.scalar_v2189 * (((v1665 * v4300) - (v1662 * (v4300 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4304) - (v1661 * (v4304 / v4327))) / v4335)));
        let v6450: f64 = (v32 * v2345);
        let v6458: f64 = (v2346 * v2346);
        let v6459: f64 = (((v2346 * (self.scalar_v2341 * v2627)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6422) } else { (if v2331 { (v2332 * self.scalar_v6422) } else { v4 }) })) / v6450))) / v6458);
        let v6463: f64 = (((v2346 * (self.scalar_v2341 * v2628)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6423) } else { (if v2331 { (v2332 * self.scalar_v6423) } else { v4 }) })) / v6450))) / v6458);
        let v6467: f64 = (((v2346 * (self.scalar_v2341 * v2629)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6424) } else { (if v2331 { (v2332 * self.scalar_v6424) } else { v4 }) })) / v6450))) / v6458);
        let v6471: f64 = (((v2346 * (self.scalar_v2341 * v2630)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6425) } else { (if v2331 { (v2332 * self.scalar_v6425) } else { v4 }) })) / v6450))) / v6458);
        let v6472: f64 = (if self.scalar_v2330 { v6459 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6402) / self.scalar_v674) } else { v4 }) });
        let v6473: f64 = (if self.scalar_v2330 { v6463 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6403) / self.scalar_v674) } else { v4 }) });
        let v6474: f64 = (if self.scalar_v2330 { v6467 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6404) / self.scalar_v674) } else { v4 }) });
        let v6475: f64 = (if self.scalar_v2330 { v6471 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6405) / self.scalar_v674) } else { v4 }) });
        let v6488: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2652) } else { v4 });
        let v6489: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2653) } else { v4 });
        let v6490: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2654) } else { v4 });
        let v6491: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2655) } else { v4 });
        let v6492: f64 = (v32 * v2359);
        let v6500: f64 = (v2360 * v2360);
        let v6522: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2613) } else { (if v859 { (v860 * self.scalar_v2613) } else { v4 }) })) } else { v4 });
        let v6523: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2639) } else { (if v859 { (v860 * self.scalar_v2639) } else { v4 }) })) } else { v4 });
        let v6524: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2614) } else { (if v859 { (v860 * self.scalar_v2614) } else { v4 }) })) } else { v4 });
        let v6525: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2594) } else { (if v859 { (v860 * self.scalar_v2594) } else { v4 }) })) } else { v4 });
        let v6526: f64 = (v32 * v2366);
        let v6534: f64 = (v2367 * v2367);
        let v6560: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6488) - (v2357 * (v6488 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6522) - (v2364 * (v6522 / v6526))) / v6534) } else { v4 })));
        let v6561: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6489) - (v2357 * (v6489 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6523) - (v2364 * (v6523 / v6526))) / v6534) } else { v4 })));
        let v6562: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6490) - (v2357 * (v6490 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6524) - (v2364 * (v6524 / v6526))) / v6534) } else { v4 })));
        let v6563: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6491) - (v2357 * (v6491 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6525) - (v2364 * (v6525 / v6526))) / v6534) } else { v4 })));
        let v6600: f64 = (v32 * v2396);
        let v6608: f64 = (v2397 * v2397);
        let v6609: f64 = (((v2397 * (self.scalar_v2392 * v2652)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2613) } else { (if v2382 { (v2383 * self.scalar_v2613) } else { v4 }) })) / v6600))) / v6608);
        let v6613: f64 = (((v2397 * (self.scalar_v2392 * v2653)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2639) } else { (if v2382 { (v2383 * self.scalar_v2639) } else { v4 }) })) / v6600))) / v6608);
        let v6617: f64 = (((v2397 * (self.scalar_v2392 * v2654)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2614) } else { (if v2382 { (v2383 * self.scalar_v2614) } else { v4 }) })) / v6600))) / v6608);
        let v6621: f64 = (((v2397 * (self.scalar_v2392 * v2655)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2594) } else { (if v2382 { (v2383 * self.scalar_v2594) } else { v4 }) })) / v6600))) / v6608);
        let v6627: f64 = (v1821 * (if self.scalar_v2381 { v6609 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6560) / self.scalar_v674) } else { v4 }) }));
        let v6637: f64 = (v1821 * (if self.scalar_v2381 { v6617 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6562) / self.scalar_v674) } else { v4 }) }));
        let v6657: f64 = (self.scalar_v2404 * f64::powf(v1149, self.scalar_v6655));
        let v6667: f64 = (v2412 * v2412);
        let v6675: f64 = (v2418 * self.scalar_v6673);
        let v6676: f64 = (v2418 * self.scalar_v6674);
        let v6680: f64 = (v2419 * v2419);
        let v6690: f64 = ((v2421 * (if self.scalar_v2403 { (v3307 * v6657) } else { v4 })) + (v2407 * (if v2416 { (((v2419 * v6675) - (v2418 * v6675)) / v6680) } else { (if v2410 { ((-(v2411 * self.scalar_v6662)) / v6667) } else { v4 }) })));
        let v6693: f64 = ((v2421 * (if self.scalar_v2403 { (v3308 * v6657) } else { v4 })) + (v2407 * (if v2416 { (((v2419 * v6676) - (v2418 * v6676)) / v6680) } else { (if v2410 { ((-(v2411 * self.scalar_v6663)) / v6667) } else { v4 }) })));
        let v6706: f64 = (v1214 * v1214);
        let v6725: f64 = ((v2432 * (if self.scalar_v2403 { ((v2429 * ((self.scalar_v106 * v3450) / self.scalar_v384)) + (v2428 * ((-(v424 * v3453)) / v6706))) } else { v4 })) + (v2431 * (self.scalar_v2190 * v5186)));
        let v6728: f64 = ((v2432 * (if self.scalar_v2403 { ((v2429 * ((self.scalar_v106 * v3451) / self.scalar_v384)) + (v2428 * ((-(v424 * v3454)) / v6706))) } else { v4 })) + (v2431 * (self.scalar_v2190 * v5187)));
        let v6743: f64 = (if self.scalar_v2403 { (v6380 / self.scalar_v2297) } else { v4 });
        let v6759: f64 = ((v2439 * self.scalar_v6746) + (v2437 * ((if self.scalar_v2403 { (v6379 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { v6693 } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { v6728 } else { v4 })))));
        let v6763: f64 = (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v6377 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { v6690 } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { v6725 } else { v4 })))) } else { v4 });
        let v6782: f64 = (self.scalar_v2442 * v6380);
        let v6788: f64 = (if self.scalar_v2403 { (v6096 + (self.scalar_v2442 * v6377)) } else { v4 });
        let v6789: f64 = (if self.scalar_v2403 { (self.scalar_v2442 * v6378) } else { v4 });
        let v6790: f64 = (if self.scalar_v2403 { (v6099 + (self.scalar_v2442 * v6379)) } else { v4 });
        let v6791: f64 = (if self.scalar_v2403 { (v6100 + v6782) } else { v4 });
        let v6792: f64 = (if self.scalar_v2403 { (v6101 + v6782) } else { v4 });
        let v6793: f64 = (if self.scalar_v2403 { (self.scalar_v2442 * v6381) } else { v4 });
        let v6822: f64 = (if self.scalar_v2456 { v6096 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6788) } else { v4 }) });
        let v6823: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6789) } else { v4 }) });
        let v6824: f64 = (if self.scalar_v2456 { v6099 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6790) } else { v4 }) });
        let v6825: f64 = (if self.scalar_v2456 { v6100 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6791) } else { v4 }) });
        let v6826: f64 = (if self.scalar_v2456 { v6101 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6792) } else { v4 }) });
        let v6827: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6793) } else { v4 }) });
        let v6828: f64 = (if self.scalar_v2456 { v6105 } else { (if self.scalar_v2403 { (v6105 + (self.scalar_v2449 * v6788)) } else { v4 }) });
        let v6829: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2449 * v6789) } else { v4 }) });
        let v6830: f64 = (if self.scalar_v2456 { v6108 } else { (if self.scalar_v2403 { (v6108 + (self.scalar_v2449 * v6790)) } else { v4 }) });
        let v6831: f64 = (if self.scalar_v2456 { v6111 } else { (if self.scalar_v2403 { (v6111 + (self.scalar_v2449 * v6791)) } else { v4 }) });
        let v6832: f64 = (if self.scalar_v2456 { v6114 } else { (if self.scalar_v2403 { (v6114 + (self.scalar_v2449 * v6792)) } else { v4 }) });
        let v6833: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2449 * v6793) } else { v4 }) });
        let v6837: f64 = (if self.scalar_v2456 { v6380 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6380) } else { v4 }) });
        let v6855: f64 = (v2461 * v2461);
        let v6902: f64 = (if v2473 { ((v2474 * v3590) + (v1263 * (self.scalar_v667 * v5186))) } else { (if v2469 { (((v2461 * (v6822 + v6828)) - (v2470 * ((v3608 - (v2460 * v3590)) / v3611))) / v6855) } else { v4 }) });
        let v6903: f64 = (if v2473 { v4 } else { (if v2469 { ((v6823 + v6829) / v2461) } else { v4 }) });
        let v6904: f64 = (if v2473 { ((v2474 * v3593) + (v1263 * (self.scalar_v667 * v5187))) } else { (if v2469 { (((v2461 * (v6824 + v6830)) - (v2470 * (((v1263 * (v3600 + v3604)) - (v2460 * v3593)) / v3611))) / v6855) } else { v4 }) });
        let v6905: f64 = (if v2473 { ((v2474 * v3596) + (v1263 * (self.scalar_v667 * v5188))) } else { (if v2469 { (((v2461 * (v6825 + v6831)) - (v2470 * (((v1263 * v3601) - (v2460 * v3596)) / v3611))) / v6855) } else { v4 }) });
        let v6906: f64 = (if v2473 { ((v2474 * v3599) + (v1263 * (self.scalar_v667 * v5189))) } else { (if v2469 { (((v2461 * (v6826 + v6832)) - (v2470 * (((v1263 * v3602) - (v2460 * v3599)) / v3611))) / v6855) } else { v4 }) });
        let v6907: f64 = (if v2473 { v4 } else { (if v2469 { ((v6827 + v6833) / v2461) } else { v4 }) });
        let v6989: f64 = (self.scalar_v27 * (self.scalar_v0 * v2780));
        let v6990: f64 = (self.scalar_v27 * (self.scalar_v0 * v2781));
        let v6991: f64 = (self.scalar_v27 * (self.scalar_v0 * v2782));
        let v6996: f64 = (self.scalar_v27 * (self.scalar_v0 * v3612));
        let v6997: f64 = (self.scalar_v27 * (self.scalar_v0 * v3616));
        let v6998: f64 = (self.scalar_v27 * (self.scalar_v0 * v3620));
        let v6999: f64 = (self.scalar_v27 * (self.scalar_v0 * v3624));
        let v7005: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v563 * v3955) + ((if self.scalar_v1382 { v3852 } else { (if self.scalar_v510 { (v3852 + v3869) } else { v4 }) }) + (self.scalar_v554 * v3911)))));
        let v7006: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v563 * v3956) + ((if self.scalar_v1382 { v3853 } else { (if self.scalar_v510 { (v3853 + v3873) } else { v4 }) }) + (self.scalar_v554 * v3912)))));
        let v7007: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v563 * v3957) + ((if self.scalar_v1382 { v3854 } else { v3883 }) + (self.scalar_v554 * v3913)))));
        let v7008: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v563 * v3958)));
        let v7009: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v563 * v3959)));
        let v7010: f64 = (self.scalar_v4538 + ((if self.scalar_v1386 { (self.scalar_v500 * ((self.scalar_v1387 * v3705) + (v1370 * (self.scalar_v1380 * v3705)))) } else { (if self.scalar_v1383 { v3735 } else { (if self.scalar_v510 { v3791 } else { v4 }) }) }) + (self.scalar_v474 * v3896)));
        let v7014: f64 = (((v1320 * (self.scalar_v1317 * v3681)) + (v1318 * ((-v3681) * v3688))) + (v7010 - (if v1569 { v4 } else { (if v1483 { (self.scalar_v36 * (self.scalar_v292 * v4116)) } else { v4 }) })));
        let v7015: f64 = (((v1320 * (self.scalar_v1317 * v3682)) + (v1318 * ((-v3682) * v3688))) + ((self.scalar_v4537 + (v3827 + (self.scalar_v474 * v3898))) - (if v1569 { v4 } else { (if v1483 { (self.scalar_v36 * (self.scalar_v292 * v4119)) } else { v4 }) })));
        let v7023: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v570 * v3655) + v7014)));
        let v7024: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v474 * v3897)));
        let v7025: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v570 * v3656) + v7015)));
        let v7026: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1386 { (self.scalar_v500 * ((v1391 * v3493) + (v1370 * (self.scalar_v1380 * v3218)))) } else { v3801 })));
        let v7027: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1386 { (self.scalar_v500 * ((v1391 * v3494) + (v1370 * (self.scalar_v1380 * v3219)))) } else { v3802 })));
        let v7046: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5057))));
        let v7047: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5058))));
        let v7048: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5059))));
        let v7049: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5060))));
        let v7050: f64 = (self.scalar_v27 * (self.scalar_v0 * (-((v1876 * (if v1658 { v4 } else { (if v1575 { (self.scalar_v71 * (self.scalar_v293 * v4285)) } else { v4 }) })) + (v1659 * v5061)))));
        let v7051: f64 = (self.scalar_v27 * (self.scalar_v0 * (-((v1876 * (if v1658 { v4 } else { (if v1575 { (self.scalar_v71 * (self.scalar_v293 * v4288)) } else { v4 }) })) + (v1659 * v5062)))));
        let v7052: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5063))));
        let v7053: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5064))));
        let v7054: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1659 * v5065))));
        let v7055: f64 = (if self.scalar_v510 { v7046 } else { v4 });
        let v7056: f64 = (if self.scalar_v510 { v7047 } else { v4 });
        let v7057: f64 = (if self.scalar_v510 { v7048 } else { v4 });
        let v7058: f64 = (if self.scalar_v510 { v7049 } else { v4 });
        let v7059: f64 = (if self.scalar_v510 { v7050 } else { v4 });
        let v7060: f64 = (if self.scalar_v510 { v7051 } else { v4 });
        let v7061: f64 = (if self.scalar_v510 { v7052 } else { v4 });
        let v7062: f64 = (if self.scalar_v510 { v7053 } else { v4 });
        let v7063: f64 = (if self.scalar_v510 { v7054 } else { v4 });
        let v7064: f64 = (if self.scalar_v1382 { v7046 } else { v4 });
        let v7065: f64 = (if self.scalar_v1382 { v7047 } else { v4 });
        let v7066: f64 = (if self.scalar_v1382 { v7048 } else { v4 });
        let v7067: f64 = (if self.scalar_v1382 { v7049 } else { v4 });
        let v7068: f64 = (if self.scalar_v1382 { v7050 } else { v4 });
        let v7069: f64 = (if self.scalar_v1382 { v7051 } else { v4 });
        let v7070: f64 = (if self.scalar_v1382 { v7052 } else { v4 });
        let v7071: f64 = (if self.scalar_v1382 { v7053 } else { v4 });
        let v7072: f64 = (if self.scalar_v1382 { v7054 } else { v4 });
        let v7079: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4515) } else { v4515 })));
        let v7080: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4516) } else { v4516 })));
        let v7081: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4517) } else { v4517 })));
        let v7082: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4518) } else { v4518 })));
        let v7083: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4519) } else { v4519 })));
        let v7084: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (self.scalar_v14 * v4520) } else { v4520 })));
        let v7085: f64 = (self.scalar_v0 * (if self.scalar_v1712 { v4 } else { (if self.scalar_v1682 { (((v1696 * (self.scalar_v1685 * (-v2662))) - (v1687 * ((self.scalar_v1689 * (self.scalar_v1690 * v2662)) / v4391))) / v4399) } else { v4 }) }));
        let v7087: f64 = (self.scalar_v0 * (if self.scalar_v1712 { v4 } else { (if self.scalar_v1682 { (((v1696 * (self.scalar_v1685 * (-v2663))) - (v1687 * ((self.scalar_v1689 * (self.scalar_v1690 * v2663)) / v4391))) / v4399) } else { v4 }) }));
        let v7089: f64 = (self.scalar_v27 * v7085);
        let v7090: f64 = (self.scalar_v27 * (self.scalar_v0 * v4488));
        let v7091: f64 = (self.scalar_v27 * v7087);
        let v7092: f64 = (self.scalar_v27 * (self.scalar_v0 * v4490));
        let v7102: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v4876 + (v1782 * v4838)) } else { v4 })));
        let v7103: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { ((v1821 * v4678) + (v1782 * v4839)) } else { v4 })));
        let v7104: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { ((v1821 * v4679) + (v1782 * v4840)) } else { v4 })));
        let v7105: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v1782 * v4841) } else { v4 })));
        let v7106: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v4876 + (v1782 * v4842)) } else { v4 })));
        let v7107: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v4888 + (v1782 * v4843)) } else { v4 })));
        let v7108: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { ((v1821 * v4681) + (v1782 * v4844)) } else { v4 })));
        let v7109: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { ((v1821 * v4682) + (v1782 * v4845)) } else { v4 })));
        let v7110: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1744 { (v4888 + (v1782 * v4846)) } else { v4 })));
        let v7113: f64 = (self.scalar_v27 * (self.scalar_v0 * ((((v1737 * (self.scalar_v1728 * v2662)) - (v1730 * ((self.scalar_v1733 * v2662) / v4525))) / v4531) + self.scalar_v4537)));
        let v7114: f64 = (self.scalar_v27 * (self.scalar_v0 * ((((v1737 * (self.scalar_v1728 * v2663)) - (v1730 * ((self.scalar_v1733 * v2663) / v4525))) / v4531) + self.scalar_v4538)));
        let v7117: f64 = (self.scalar_v0 * (((v1896 * (self.scalar_v2589 + (self.scalar_v933 * (if v811 { (v812 * self.scalar_v2594) } else { (if v808 { (v809 * self.scalar_v2594) } else { v4 }) })))) - (v1899 * v5220)) / v5229));
        let v7120: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1899 * v5219)) / v5229)));
        let v7121: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v0 + (self.scalar_v933 * (if v811 { (v812 * self.scalar_v2593) } else { (if v808 { (v809 * self.scalar_v2593) } else { v4 }) }))) / v1896)));
        let v7122: f64 = (self.scalar_v27 * v7117);
        let v7123: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1899 * v5221)) / v5229)));
        let v7124: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1899 * v5222)) / v5229)));
        let v7129: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6043)));
        let v7130: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6044)));
        let v7131: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6045)));
        let v7132: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v6046)));
        let v7149: f64 = ddt_scale;
        let v7156: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2456 { v6377 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6377) } else { v4 }) }) + ((self.scalar_v2157 * v3322) + v6822))) * v7149));
        let v7157: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6823 + (if self.scalar_v2456 { v6378 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6378) } else { v4 }) }))) * v7149));
        let v7158: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2456 { v6379 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6379) } else { v4 }) }) + ((self.scalar_v2157 * v3323) + v6824))) * v7149));
        let v7159: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6825 + v6837)) * v7149));
        let v7160: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6826 + v6837)) * v7149));
        let v7161: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6827 + (if self.scalar_v2456 { v6381 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6381) } else { v4 }) }))) * v7149));
        let v7166: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v2176 * ((self.scalar_v1152 * (-((-(self.scalar_v292 * v6067)) * v6074))) + (v155 * (self.scalar_v2589 - v6067)))))));
        let v7167: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v2176 * ((self.scalar_v1152 * (-((-(self.scalar_v292 * v6068)) * v6074))) + (v155 * (self.scalar_v0 - v6068)))))));
        let v7180: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6828)));
        let v7181: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6829)));
        let v7182: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * (self.scalar_v2312 * v3260)) + (v2313 * v3220)) + ((self.scalar_v2187 * v3448) + v6830)))));
        let v7183: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * (self.scalar_v2312 * v3261)) + (v2313 * v3221)) + ((self.scalar_v2187 * v3449) + v6831)))));
        let v7184: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * (self.scalar_v2312 * v3262)) + (v2313 * v3216)) + ((self.scalar_v2187 * v3445) + v6832)))));
        let v7185: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6833)));
        let v7190: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v304 * ((self.scalar_v2281 * (-((-(v6338 / self.scalar_v291)) * v6346))) + (v32 * (self.scalar_v0 - v6338)))))));
        let v7191: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v304 * ((self.scalar_v2281 * (-((-(v6339 / self.scalar_v291)) * v6346))) + (v32 * (self.scalar_v2589 - v6339)))))));
        let v7204: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6763)));
        let v7205: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { ((v2439 * self.scalar_v6745) + (v2437 * (if self.scalar_v2403 { (v6378 / self.scalar_v2297) } else { v4 }))) } else { v4 }))));
        let v7206: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { v6759 } else { v4 }))));
        let v7207: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v2431 * (self.scalar_v2190 * v5188)) } else { v4 }) + v6743)) } else { v4 }))));
        let v7208: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v2431 * (self.scalar_v2190 * v5189)) } else { v4 }) + v6743)) } else { v4 }))));
        let v7209: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * (if self.scalar_v2403 { (v6381 / self.scalar_v2297) } else { v4 })) } else { v4 }))));
        let v7214: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7210));
        let v7215: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7211));
        let v7220: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7216));
        let v7221: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7217));
        let v7232: f64 = (self.scalar_v27 * (self.scalar_v0 * (v5117 + (v1823 * v5057))));
        let v7233: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { ((v1821 * v4592) + (v1758 * v4839)) } else { v4 })) + (v1823 * v5058))));
        let v7234: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1876 * (if self.scalar_v1744 { (v1758 * v4840) } else { v4 }))));
        let v7235: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { (v1758 * v4841) } else { v4 })) + (v1823 * v5059))));
        let v7236: f64 = (self.scalar_v27 * (self.scalar_v0 * (v5117 + (v1823 * v5060))));
        let v7237: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { (v4847 + (v1758 * v4842)) } else { v4 })) + (v1823 * v5061))));
        let v7238: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { (v4857 + (v1758 * v4843)) } else { v4 })) + (v1823 * v5062))));
        let v7239: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { (v4857 + (v1758 * v4844)) } else { v4 })) + (v1823 * v5063))));
        let v7240: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { ((v1821 * v4594) + (v1758 * v4845)) } else { v4 })) + (v1823 * v5064))));
        let v7241: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1876 * (if self.scalar_v1744 { (v4857 + (v1758 * v4846)) } else { v4 })) + (v1823 * v5065))));
        let v7262: f64 = (v7149 * (self.scalar_v0 * (v6313 + (if self.scalar_v2351 { ((v2399 * v4839) + (v1821 * (if self.scalar_v2381 { v6613 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6561) / self.scalar_v674) } else { v4 }) }))) } else { v4 }))));
        let v7268: f64 = (v7149 * (self.scalar_v0 * (v6315 + (if self.scalar_v2351 { ((v2399 * v4845) + (v1821 * (if self.scalar_v2381 { v6621 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6563) / self.scalar_v674) } else { v4 }) }))) } else { v4 }))));
        let v7270: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6312 + (if self.scalar_v2351 { ((v2399 * v4838) + v6627) } else { v4 })))));
        let v7271: f64 = (self.scalar_v27 * v7262);
        let v7272: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2351 { (v2399 * v4840) } else { v4 }))));
        let v7273: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2351 { (v2399 * v4841) } else { v4 }))));
        let v7274: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6312 + (if self.scalar_v2351 { (v6627 + (v2399 * v4842)) } else { v4 })))));
        let v7275: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6314 + (if self.scalar_v2351 { ((v2399 * v4843) + v6637) } else { v4 })))));
        let v7276: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6314 + (if self.scalar_v2351 { (v6637 + (v2399 * v4844)) } else { v4 })))));
        let v7277: f64 = (self.scalar_v27 * v7268);
        let v7278: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6314 + (if self.scalar_v2351 { (v6637 + (v2399 * v4846)) } else { v4 })))));
        let v7287: f64 = (self.scalar_v0 * (((v1876 * (if self.scalar_v1744 { (self.scalar_v14 * v4366) } else { v4366 })) + (v1746 * v5060)) + (self.scalar_v4537 + ((v1876 * (self.scalar_v486 * v3935)) + (v1467 * v5060)))));
        let v7288: f64 = (self.scalar_v0 * (((v1876 * (if self.scalar_v1744 { (self.scalar_v14 * v4370) } else { v4370 })) + (v1746 * v5061)) + (((v1876 * (self.scalar_v486 * v3936)) + (v1467 * v5061)) + self.scalar_v6946)));
        let v7292: f64 = (self.scalar_v0 * (((v1876 * (if self.scalar_v1744 { (self.scalar_v14 * v4378) } else { v4378 })) + (v1746 * v5065)) + (self.scalar_v4538 + ((v1876 * (self.scalar_v486 * v3938)) + (v1467 * v5065)))));
        let v7293: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1746 * v5057) + (v1467 * v5057))));
        let v7294: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1746 * v5058) + (v1467 * v5058))));
        let v7295: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1746 * v5059) + ((v1876 * (self.scalar_v486 * v3934)) + (v1467 * v5059)))));
        let v7296: f64 = (self.scalar_v27 * v7287);
        let v7297: f64 = (self.scalar_v27 * v7288);
        let v7298: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v5088 + (v1746 * v5062)) + ((v5108 + (v1467 * v5062)) + self.scalar_v6947))));
        let v7299: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v5088 + (v1746 * v5063)) + ((v5108 + (v1467 * v5063)) + self.scalar_v6947))));
        let v7300: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1746 * v5064) + (v1467 * v5064))));
        let v7301: f64 = (self.scalar_v27 * v7292);
        let v7310: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6215 + (if self.scalar_v2351 { (self.scalar_v14 * v6472) } else { v6472 })))));
        let v7311: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6216 + (if self.scalar_v2351 { (self.scalar_v14 * v6473) } else { v6473 })))));
        let v7312: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6217 + (if self.scalar_v2351 { (self.scalar_v14 * v6474) } else { v6474 })))));
        let v7313: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6218 + (if self.scalar_v2351 { (self.scalar_v14 * v6475) } else { v6475 })))));
        let v7326: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6902) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6902) } else { v4 }) }) }));
        let v7327: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6903) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6903) } else { v4 }) }) }));
        let v7328: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6904) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6904) } else { v4 }) }) }));
        let v7329: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6905) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6905) } else { v4 }) }) }));
        let v7330: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6906) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6906) } else { v4 }) }) }));
        let v7331: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6907) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6907) } else { v4 }) }) }));
        let v7332: f64 = (v2489 * v7149);

        let d2506_dn6: f64 = v6989;
        let d2506_dn7: f64 = v6990;
        let d2506_dn8: f64 = v6991;
        stamper.stamp_current_node3_local(
            Some(7),
            Some(8),
            multiplicity * (v2506),
            6,
            multiplicity * (d2506_dn6),
            7,
            multiplicity * (d2506_dn7),
            8,
            multiplicity * (d2506_dn8),
        );
        let d2508_dn4: f64 = v6996;
        let d2508_dn6: f64 = v6997;
        let d2508_dn7: f64 = v6998;
        let d2508_dn8: f64 = v6999;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(4),
            multiplicity * (v2508),
            [4, 6, 7, 8],
            [d2508_dn4, d2508_dn6, d2508_dn7, d2508_dn8],
            [],
            [],
            multiplicity,
        );
        let d2510_dn4: f64 = v7005;
        let d2510_dn5: f64 = v7006;
        let d2510_dn6: f64 = v7007;
        let d2510_dn7: f64 = v7008;
        let d2510_dn8: f64 = v7008;
        let d2510_dn10: f64 = v7009;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(4),
            multiplicity * (v2510),
            [4, 5, 6, 7, 8, 10],
            [d2510_dn4, d2510_dn5, d2510_dn6, d2510_dn7, d2510_dn8, d2510_dn10],
            [],
            [],
            multiplicity,
        );
        let d2516_dn4: f64 = v7023;
        let d2516_dn5: f64 = v7024;
        let d2516_dn6: f64 = v7025;
        let d2516_dn7: f64 = v7026;
        let d2516_dn8: f64 = v7027;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2516),
            [4, 5, 6, 7, 8],
            [d2516_dn4, d2516_dn5, d2516_dn6, d2516_dn7, d2516_dn8],
            [],
            [],
            multiplicity,
        );
        let d2520_dn0: f64 = v7055;
        let d2520_dn1: f64 = v7056;
        let d2520_dn4: f64 = v7057;
        let d2520_dn5: f64 = v7058;
        let d2520_dn6: f64 = v7059;
        let d2520_dn7: f64 = v7060;
        let d2520_dn8: f64 = v7061;
        let d2520_dn9: f64 = v7062;
        let d2520_dn10: f64 = v7063;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (v2520),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [d2520_dn0, d2520_dn1, d2520_dn4, d2520_dn5, d2520_dn6, d2520_dn7, d2520_dn8, d2520_dn9, d2520_dn10],
            [],
            [],
            multiplicity,
        );
        let d2521_dn0: f64 = v7064;
        let d2521_dn1: f64 = v7065;
        let d2521_dn4: f64 = v7066;
        let d2521_dn5: f64 = v7067;
        let d2521_dn6: f64 = v7068;
        let d2521_dn7: f64 = v7069;
        let d2521_dn8: f64 = v7070;
        let d2521_dn9: f64 = v7071;
        let d2521_dn10: f64 = v7072;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2521),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [d2521_dn0, d2521_dn1, d2521_dn4, d2521_dn5, d2521_dn6, d2521_dn7, d2521_dn8, d2521_dn9, d2521_dn10],
            [],
            [],
            multiplicity,
        );
        let d2523_dn3: f64 = v7079;
        let d2523_dn5: f64 = v7080;
        let d2523_dn6: f64 = v7081;
        let d2523_dn7: f64 = v7082;
        let d2523_dn8: f64 = v7083;
        let d2523_dn10: f64 = v7084;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2523),
            [3, 5, 6, 7, 8, 10],
            [d2523_dn3, d2523_dn5, d2523_dn6, d2523_dn7, d2523_dn8, d2523_dn10],
            [],
            [],
            multiplicity,
        );
        let d2525_dn3: f64 = v7089;
        let d2525_dn6: f64 = v7090;
        let d2525_dn7: f64 = v7091;
        let d2525_dn8: f64 = v7092;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(3),
            multiplicity * (v2525),
            [3, 6, 7, 8],
            [d2525_dn3, d2525_dn6, d2525_dn7, d2525_dn8],
            [],
            [],
            multiplicity,
        );
        let d2527_dn0: f64 = v7102;
        let d2527_dn1: f64 = v7103;
        let d2527_dn3: f64 = v7104;
        let d2527_dn4: f64 = v7105;
        let d2527_dn5: f64 = v7102;
        let d2527_dn6: f64 = v7106;
        let d2527_dn7: f64 = v7107;
        let d2527_dn8: f64 = v7108;
        let d2527_dn9: f64 = v7109;
        let d2527_dn10: f64 = v7110;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(3),
            multiplicity * (v2527),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2527_dn0, d2527_dn1, d2527_dn3, d2527_dn4, d2527_dn5, d2527_dn6, d2527_dn7, d2527_dn8, d2527_dn9, d2527_dn10],
            [],
            [],
            multiplicity,
        );
        let d2529_dn3: f64 = v7113;
        let d2529_dn7: f64 = v7114;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (v2529),
            3,
            multiplicity * (d2529_dn3),
            7,
            multiplicity * (d2529_dn7),
        );
        let d2531_dn4: f64 = v7120;
        let d2531_dn5: f64 = v7121;
        let d2531_dn6: f64 = v7122;
        let d2531_dn7: f64 = v7123;
        let d2531_dn8: f64 = v7124;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2531),
            [4, 5, 6, 7, 8],
            [d2531_dn4, d2531_dn5, d2531_dn6, d2531_dn7, d2531_dn8],
            [],
            [],
            multiplicity,
        );
        let d2533_dn4: f64 = v7129;
        let d2533_dn6: f64 = v7130;
        let d2533_dn7: f64 = v7131;
        let d2533_dn8: f64 = v7132;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2533),
            [4, 6, 7, 8],
            [d2533_dn4, d2533_dn6, d2533_dn7, d2533_dn8],
            [],
            [],
            multiplicity,
        );
        let d2536_dn2: f64 = self.scalar_v7137;
        let d2536_dn4: f64 = self.scalar_v7138;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(4),
            multiplicity * (v2536),
            2,
            multiplicity * (d2536_dn2),
            4,
            multiplicity * (d2536_dn4),
        );
        let d2539_dn1: f64 = self.scalar_v7141;
        let d2539_dn5: f64 = self.scalar_v7142;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (v2539),
            1,
            multiplicity * (d2539_dn1),
            5,
            multiplicity * (d2539_dn5),
        );
        let d2542_dn4: f64 = v7156;
        let d2542_dn5: f64 = v7157;
        let d2542_dn6: f64 = v7158;
        let d2542_dn7: f64 = v7159;
        let d2542_dn8: f64 = v7160;
        let d2542_dn10: f64 = v7161;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2542),
            [4, 5, 6, 7, 8, 10],
            [d2542_dn4, d2542_dn5, d2542_dn6, d2542_dn7, d2542_dn8, d2542_dn10],
            [],
            [],
            multiplicity,
        );
        let d2545_dn4: f64 = v7166;
        let d2545_dn5: f64 = v7167;
        stamper.stamp_current_node2_local(
            Some(5),
            Some(4),
            multiplicity * (v2545),
            4,
            multiplicity * (d2545_dn4),
            5,
            multiplicity * (d2545_dn5),
        );
        let d2548_dn4: f64 = v7180;
        let d2548_dn5: f64 = v7181;
        let d2548_dn6: f64 = v7182;
        let d2548_dn7: f64 = v7183;
        let d2548_dn8: f64 = v7184;
        let d2548_dn10: f64 = v7185;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (v2548),
            [4, 5, 6, 7, 8, 10],
            [d2548_dn4, d2548_dn5, d2548_dn6, d2548_dn7, d2548_dn8, d2548_dn10],
            [],
            [],
            multiplicity,
        );
        let d2551_dn3: f64 = v7190;
        let d2551_dn7: f64 = v7191;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(7),
            multiplicity * (v2551),
            3,
            multiplicity * (d2551_dn3),
            7,
            multiplicity * (d2551_dn7),
        );
        let d2554_dn4: f64 = v7204;
        let d2554_dn5: f64 = v7205;
        let d2554_dn6: f64 = v7206;
        let d2554_dn7: f64 = v7207;
        let d2554_dn8: f64 = v7208;
        let d2554_dn10: f64 = v7209;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2554),
            [4, 5, 6, 7, 8, 10],
            [d2554_dn4, d2554_dn5, d2554_dn6, d2554_dn7, d2554_dn8, d2554_dn10],
            [],
            [],
            multiplicity,
        );
        let d2558_dn1: f64 = v7214;
        let d2558_dn2: f64 = v7215;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2558),
            1,
            multiplicity * (d2558_dn1),
            2,
            multiplicity * (d2558_dn2),
        );
        let d2562_dn0: f64 = v7220;
        let d2562_dn1: f64 = v7221;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2562),
            0,
            multiplicity * (d2562_dn0),
            1,
            multiplicity * (d2562_dn1),
        );
        let d2564_dn0: f64 = v7232;
        let d2564_dn1: f64 = v7233;
        let d2564_dn3: f64 = v7234;
        let d2564_dn4: f64 = v7235;
        let d2564_dn5: f64 = v7236;
        let d2564_dn6: f64 = v7237;
        let d2564_dn7: f64 = v7238;
        let d2564_dn8: f64 = v7239;
        let d2564_dn9: f64 = v7240;
        let d2564_dn10: f64 = v7241;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2564),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2564_dn0, d2564_dn1, d2564_dn3, d2564_dn4, d2564_dn5, d2564_dn6, d2564_dn7, d2564_dn8, d2564_dn9, d2564_dn10],
            [],
            [],
            multiplicity,
        );
        let d2567_dn0: f64 = self.scalar_v7248;
        let d2567_dn1: f64 = self.scalar_v7249;
        let d2567_dn5: f64 = self.scalar_v7249;
        let d2567_dn6: f64 = self.scalar_v7249;
        let d2567_dn7: f64 = self.scalar_v7250;
        let d2567_dn8: f64 = self.scalar_v7250;
        let d2567_dn9: f64 = self.scalar_v7251;
        let d2567_dn10: f64 = self.scalar_v7250;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(9),
            multiplicity * (v2567),
            [0, 1, 5, 6, 7, 8, 9, 10],
            [d2567_dn0, d2567_dn1, d2567_dn5, d2567_dn6, d2567_dn7, d2567_dn8, d2567_dn9, d2567_dn10],
            [],
            [],
            multiplicity,
        );
        let d2570_dn0: f64 = v7270;
        let d2570_dn1: f64 = v7271;
        let d2570_dn3: f64 = v7272;
        let d2570_dn4: f64 = v7273;
        let d2570_dn5: f64 = v7270;
        let d2570_dn6: f64 = v7274;
        let d2570_dn7: f64 = v7275;
        let d2570_dn8: f64 = v7276;
        let d2570_dn9: f64 = v7277;
        let d2570_dn10: f64 = v7278;
        stamper.stamp_current_sparse_local::<10, 0>(
            Some(1),
            Some(9),
            multiplicity * (v2570),
            [0, 1, 3, 4, 5, 6, 7, 8, 9, 10],
            [d2570_dn0, d2570_dn1, d2570_dn3, d2570_dn4, d2570_dn5, d2570_dn6, d2570_dn7, d2570_dn8, d2570_dn9, d2570_dn10],
            [],
            [],
            multiplicity,
        );
        let d2573_dn0: f64 = v7293;
        let d2573_dn1: f64 = v7294;
        let d2573_dn4: f64 = v7295;
        let d2573_dn5: f64 = v7296;
        let d2573_dn6: f64 = v7297;
        let d2573_dn7: f64 = v7298;
        let d2573_dn8: f64 = v7299;
        let d2573_dn9: f64 = v7300;
        let d2573_dn10: f64 = v7301;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2573),
            [0, 1, 4, 5, 6, 7, 8, 9, 10],
            [d2573_dn0, d2573_dn1, d2573_dn4, d2573_dn5, d2573_dn6, d2573_dn7, d2573_dn8, d2573_dn9, d2573_dn10],
            [],
            [],
            multiplicity,
        );
        let d2576_dn5: f64 = v7310;
        let d2576_dn6: f64 = v7311;
        let d2576_dn7: f64 = v7312;
        let d2576_dn8: f64 = v7312;
        let d2576_dn10: f64 = v7313;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(10),
            multiplicity * (v2576),
            [5, 6, 7, 8, 10],
            [d2576_dn5, d2576_dn6, d2576_dn7, d2576_dn8, d2576_dn10],
            [],
            [],
            multiplicity,
        );
        let d2580_dn9: f64 = self.scalar_v7318;
        let d2580_dn10: f64 = self.scalar_v7319;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * (v2580),
            9,
            multiplicity * (d2580_dn9),
            10,
            multiplicity * (d2580_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(10),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v4,
        );
        let d2584_dn7: f64 = self.scalar_v7324;
        let d2584_dn10: f64 = self.scalar_v7325;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(7),
            multiplicity * (v2584),
            7,
            multiplicity * (d2584_dn7),
            10,
            multiplicity * (d2584_dn10),
        );
        stamper.stamp_potential_branch_local(
            Some(10),
            Some(7),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v4,
        );
        stamper.stamp_current_const_local(
            Some(11),
            None,
            multiplicity * (v4),
        );
        let d2585_dn11: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(11),
            None,
            multiplicity * (v2585),
            11,
            multiplicity * (d2585_dn11),
        );
        let d2587_dn4: f64 = v7326;
        let d2587_dn5: f64 = v7327;
        let d2587_dn6: f64 = v7328;
        let d2587_dn7: f64 = v7329;
        let d2587_dn8: f64 = v7330;
        let d2587_dn10: f64 = v7331;
        let d2587_dn11: f64 = v7332;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(6),
            Some(4),
            multiplicity * (v2587),
            [4, 5, 6, 7, 8, 10, 11],
            [d2587_dn4, d2587_dn5, d2587_dn6, d2587_dn7, d2587_dn8, d2587_dn10, d2587_dn11],
            [],
            [],
            multiplicity,
        );
        let d2588_dn11: f64 = v2468;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(6),
            multiplicity * (v2588),
            11,
            multiplicity * (d2588_dn11),
        );
        let d2585_dn11: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(8),
            Some(4),
            multiplicity * (v2585),
            11,
            multiplicity * (d2585_dn11),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(10),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(7),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (v4),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let v1: f64 = 1.0;
        let v4: f64 = 0.0;
        let v32: f64 = 2.0;
        let v155: f64 = 3.0;
        let v424: f64 = 0.5;
        let v436: f64 = 4.0;
        let v462: f64 = 6.0;
        let v727: f64 = ctx.node_voltage(nodes[6]);
        let v728: f64 = ctx.node_voltage(nodes[7]);
        let v730: f64 = (self.scalar_v0 * (v727 - v728));
        let v731: f64 = ctx.node_voltage(nodes[8]);
        let v733: f64 = (self.scalar_v0 * (v727 - v731));
        let v734: f64 = ctx.node_voltage(nodes[4]);
        let v736: f64 = (self.scalar_v0 * (v727 - v734));
        let v737: f64 = ctx.node_voltage(nodes[5]);
        let v739: f64 = (self.scalar_v0 * (v737 - v734));
        let v741: f64 = (self.scalar_v0 * (v737 - v727));
        let v744: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[3]) - v728));
        let v746: f64 = (self.scalar_v0 * (v728 - v731));
        let v750: f64 = ctx.node_voltage(nodes[1]);
        let v757: f64 = (self.scalar_v0 * (v750 - ctx.node_voltage(nodes[0])));
        let v758: f64 = ctx.node_voltage(nodes[10]);
        let v760: f64 = (self.scalar_v0 * (v758 - v728));
        let v763: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[9]) - v758));
        let v766: f64 = (((v733 + v741) - v746) - v760);
        let v771: f64 = (v757 + ((v766 + ((self.scalar_v0 * (v750 - v737)) + (-v757))) - v763));
        let v772: f64 = (v744 - v760);
        let v774: f64 = (self.scalar_v106 * v733);
        let v776: bool = (v774 < self.scalar_v775);
        let v777: f64 = ((v774) as f64).exp();
        let v779: bool = (!v776);
        let v781: f64 = (if v779 { self.scalar_v780 } else { v4 });
        let v786: f64 = (self.scalar_v106 * v736);
        let v787: f64 = (v786 / self.scalar_v384);
        let v788: bool = (v787 < self.scalar_v775);
        let v789: f64 = ((v787) as f64).exp();
        let v791: bool = (!v788);
        let v792: f64 = (if v791 { self.scalar_v780 } else { v781 });
        let v796: f64 = (if v791 { (v792 * (v1 + (v787 - self.scalar_v775))) } else { (if v788 { v789 } else { v4 }) });
        let v797: f64 = (self.scalar_v106 * v766);
        let v798: bool = (v797 < self.scalar_v775);
        let v799: f64 = ((v797) as f64).exp();
        let v801: bool = (!v798);
        let v802: f64 = (if v801 { self.scalar_v780 } else { v792 });
        let v806: f64 = (if v801 { (v802 * (v1 + (v797 - self.scalar_v775))) } else { (if v798 { v799 } else { v4 }) });
        let v817: f64 = (self.scalar_v106 * v771);
        let v818: bool = (v817 < self.scalar_v775);
        let v819: f64 = ((v817) as f64).exp();
        let v821: bool = (!v818);
        let v822: f64 = (if v821 { self.scalar_v780 } else { (if (!((self.scalar_v106 * v741) < self.scalar_v775)) { self.scalar_v780 } else { v802 }) });
        let v826: f64 = (if v821 { (v822 * (v1 + (v817 - self.scalar_v775))) } else { (if v818 { v819 } else { v4 }) });
        let v837: f64 = (self.scalar_v106 * (v772 - v763));
        let v838: bool = (v837 < self.scalar_v775);
        let v839: f64 = ((v837) as f64).exp();
        let v841: bool = (!v838);
        let v842: f64 = (if v841 { self.scalar_v780 } else { (if (!((self.scalar_v106 * v744) < self.scalar_v775)) { self.scalar_v780 } else { v822 }) });
        let v846: f64 = (if v841 { (v842 * (v1 + (v837 - self.scalar_v775))) } else { (if v838 { v839 } else { v4 }) });
        let v858: f64 = (self.scalar_v106 * (v771 - self.scalar_v204));
        let v859: bool = (v858 < self.scalar_v775);
        let v860: f64 = ((v858) as f64).exp();
        let v862: bool = (!v859);
        let v863: f64 = (if v862 { self.scalar_v780 } else { (if (!((self.scalar_v106 * v772) < self.scalar_v775)) { self.scalar_v780 } else { v842 }) });
        let v869: f64 = (self.scalar_v106 * (v766 - self.scalar_v204));
        let v870: bool = (v869 < self.scalar_v775);
        let v871: f64 = ((v869) as f64).exp();
        let v873: bool = (!v870);
        let v874: f64 = (if v873 { self.scalar_v780 } else { v863 });
        let v880: f64 = (self.scalar_v106 * (v733 - self.scalar_v204));
        let v881: bool = (v880 < self.scalar_v775);
        let v882: f64 = ((v880) as f64).exp();
        let v884: bool = (!v881);
        let v885: f64 = (if v884 { self.scalar_v780 } else { v874 });
        let v889: f64 = (if v884 { (v885 * (v1 + (v880 - self.scalar_v775))) } else { (if v881 { v882 } else { v4 }) });
        let v891: f64 = (self.scalar_v106 * (v730 - self.scalar_v204));
        let v892: bool = (v891 < self.scalar_v775);
        let v893: f64 = ((v891) as f64).exp();
        let v895: bool = (!v892);
        let v896: f64 = (if v895 { self.scalar_v780 } else { v885 });
        let v900: f64 = (if v895 { (v896 * (v1 + (v891 - self.scalar_v775))) } else { (if v892 { v893 } else { v4 }) });
        let v903: f64 = (((v1 + (v436 * v889))) as f64).sqrt();
        let v906: f64 = (((v1 + (v436 * v900))) as f64).sqrt();
        let v907: f64 = (v32 * v900);
        let v908: f64 = (v1 + v906);
        let v909: f64 = (v907 / v908);
        let v911: bool = (v909 < self.scalar_v910);
        let v912: f64 = (if v911 { self.scalar_v910 } else { v909 });
        let v914: f64 = (v1 + v903);
        let v915: f64 = (v914 / v908);
        let v918: f64 = (self.scalar_v104 * ((v903 - v906) - ((v915) as f64).ln()));
        let v920: f64 = ((v746 + v918) / self.scalar_v352);
        let v921: bool = (v920 > v4);
        let v922: f64 = 100.0;
        let v923: bool = (v730 < v922);
        let v924: bool = (v921 && v923);
        let v927: bool = (v921 && (!v923));
        let v929: f64 = (v1 + (v730 - v922));
        let v935: f64 = (self.scalar_v352 * (v424 * v920));
        let v937: f64 = (v1 + (self.scalar_v106 * v935));
        let v942: f64 = (if v921 { ((self.scalar_v204 + (self.scalar_v933 * ((v937) as f64).ln())) - (if v927 { (v922 + ((v929) as f64).ln()) } else { (if v924 { v730 } else { v4 }) })) } else { v4 });
        let v945: f64 = (if v921 { self.scalar_v944 } else { v4 });
        let v947: f64 = (if v921 { (v945 * v945) } else { 1e-6 });
        let v950: bool = (v942 < v4);
        let v951: bool = (v921 && v950);
        let v952: f64 = (v424 * v947);
        let v954: f64 = (((v947 + (if v921 { (v942 * v942) } else { self.scalar_v422 }))) as f64).sqrt();
        let v955: f64 = (v954 - v942);
        let v959: bool = (v921 && (!v950));
        let v962: f64 = (if v959 { (v424 * (v942 + v954)) } else { (if v951 { (v952 / v955) } else { v4 }) });
        let v966: f64 = (v962 + self.scalar_v965);
        let v967: f64 = (v962 * v966);
        let v970: f64 = (self.scalar_v964 * (v962 + self.scalar_v968));
        let v972: f64 = (if v921 { (v967 / v970) } else { v4 });
        let v974: f64 = (if v921 { (v920 / v972) } else { v4 });
        let v978: f64 = (if v921 { ((v974 - v1) / self.scalar_v976) } else { self.scalar_v394 });
        let v979: bool = (v974 < v1);
        let v980: bool = (v921 && v979);
        let v981: f64 = ((v978) as f64).exp();
        let v982: f64 = (v1 + v981);
        let v988: bool = (v921 && (!v979));
        let v990: f64 = (((-v978)) as f64).exp();
        let v991: f64 = (v1 + v990);
        let v1004: f64 = (if v921 { ((if v988 { (v974 + (self.scalar_v976 * ((v991) as f64).ln())) } else { (if v980 { (v1 + (self.scalar_v976 * ((v982) as f64).ln())) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v1006: f64 = (if v921 { (v962 / self.scalar_v965) } else { v4 });
        let v1007: f64 = (v436 * v1004);
        let v1008: f64 = (v1006 * v1007);
        let v1009: f64 = (v1 + v1006);
        let v1012: f64 = (((v1 + (v1008 * v1009))) as f64).sqrt();
        let v1013: f64 = (v1 + v1012);
        let v1014: f64 = (v32 * v1004);
        let v1015: f64 = (v1009 * v1014);
        let v1017: f64 = (if v921 { (v1013 / v1015) } else { v4 });
        let v1019: f64 = (v912 * v1017);
        let v1020: f64 = ((v1 - v1017) + v1019);
        let v1021: f64 = (v1 + v1019);
        let v1023: f64 = (if v921 { (v1020 / v1021) } else { v4 });
        let v1026: f64 = (if v921 { (self.scalar_v106 * (v935 * v1023)) } else { v4 });
        let v1029: f64 = (v1 + (v912 + v1026));
        let v1032: f64 = (if v921 { ((v32 * v1026) + (v912 * v1029)) } else { v4 });
        let v1035: f64 = (if v921 { (v424 * (v1026 - v1)) } else { v4 });
        let v1039: bool = (v1026 >= v1);
        let v1040: bool = (v921 && v1039);
        let v1041: f64 = (((if v921 { (v1032 + (v1035 * v1035)) } else { v4 })) as f64).sqrt();
        let v1045: bool = (v921 && (!v1039));
        let v1046: f64 = (v1041 - v1035);
        let v1048: f64 = (if v1045 { (v1032 / v1046) } else { (if v1040 { (v1035 + v1041) } else { v4 }) });
        let v1051: bool = (v921 && (v1048 < self.scalar_v1049));
        let v1052: f64 = (if v1051 { self.scalar_v1049 } else { v1048 });
        let v1053: f64 = (v1 + v1052);
        let v1062: f64 = (if v921 { (self.scalar_v1059 * (v920 - self.scalar_v963)) } else { v4 });
        let v1069: f64 = ((((if v921 { (v920 * self.scalar_v1064) } else { v4 }) + (v1062 * v1062))) as f64).sqrt();
        let v1078: bool = (v921 && self.scalar_v1077);
        let v1079: f64 = (v32 * v920);
        let v1080: f64 = (v920 + v972);
        let v1085: f64 = (v920 * self.scalar_v963);
        let v1086: f64 = (v920 + self.scalar_v963);
        let v1091: bool = (!v921);
        let v1092: f64 = (v32 * v889);
        let v1095: f64 = (if v1091 { (if v779 { (v781 * (v1 + (v774 - self.scalar_v775))) } else { (if v776 { v777 } else { v4 }) }) } else { (if v921 { ((v1052 * v1053) * self.scalar_v1056) } else { v4 }) });
        let v1106: bool = ((((v746) as f64).abs() < self.scalar_v1098) || (((v918) as f64).abs() < (self.scalar_v1102 * (v903 + v906))));
        let v1107: bool = (v1091 && v1106);
        let v1108: f64 = (v912 + (if v1091 { (v1092 / v914) } else { v1052 }));
        let v1110: f64 = (if v1107 { (v424 * v1108) } else { v4 });
        let v1111: f64 = (v1 + v1110);
        let v1115: bool = (v1091 && (!v1106));
        let v1117: f64 = ((v733 + v918) - v730);
        let v1121: f64 = (if v1091 { self.scalar_v1075 } else { (if v1078 { (self.scalar_v245 * (0.1 + (v1079 / v1080))) } else { (if (v921 && self.scalar_v1073) { self.scalar_v1075 } else { v4 }) }) });
        let v1125: f64 = (if v1091 { (v1 - ((if v1091 { v920 } else { (if v921 { (v1085 / v1086) } else { v4 }) }) / self.scalar_v963)) } else { (if v921 { (self.scalar_v963 / v1086) } else { v4 }) });
        let v1132: f64 = ((v736 - self.scalar_v1129) / self.scalar_v1130);
        let v1133: bool = (v736 < self.scalar_v1129);
        let v1134: f64 = ((v1132) as f64).exp();
        let v1135: f64 = (v1 + v1134);
        let v1140: bool = (!v1133);
        let v1142: f64 = (((-v1132)) as f64).exp();
        let v1143: f64 = (v1 + v1142);
        let v1147: f64 = (if v1140 { (self.scalar_v1129 - (self.scalar_v1130 * ((v1143) as f64).ln())) } else { (if v1133 { (v736 - (self.scalar_v1130 * ((v1135) as f64).ln())) } else { v4 }) });
        let v1149: f64 = (v1 - (self.scalar_v292 * v1147));
        let v1151: f64 = f64::powf(v1149, self.scalar_v1150);
        let v1157: f64 = ((self.scalar_v1152 * (v1 - v1151)) + (v155 * (v736 - v1147)));
        let v1168: f64 = (if self.scalar_v1167 { v733 } else { (if self.scalar_v1163 { (v730 + (if v1091 { v746 } else { (if v921 { (v1062 + v1069) } else { v4 }) })) } else { (if self.scalar_v1159 { v730 } else { v4 }) }) });
        let v1176: f64 = (v1168 - self.scalar_v1175);
        let v1177: f64 = (v1176 / v1121);
        let v1178: bool = (v1168 < self.scalar_v1175);
        let v1179: f64 = ((v1177) as f64).exp();
        let v1180: f64 = (v1 + v1179);
        let v1181: f64 = ((v1180) as f64).ln();
        let v1185: bool = (!v1178);
        let v1187: f64 = (((-v1177)) as f64).exp();
        let v1188: f64 = (v1 + v1187);
        let v1189: f64 = ((v1188) as f64).ln();
        let v1192: f64 = (if v1185 { (self.scalar_v1175 - (v1121 * v1189)) } else { (if v1178 { (v1168 - (v1121 * v1181)) } else { v4 }) });
        let v1194: f64 = f64::powf(v1125, self.scalar_v1193);
        let v1198: f64 = (v1 - (v1192 / self.scalar_v245));
        let v1199: f64 = f64::powf(v1198, self.scalar_v1195);
        let v1203: f64 = (self.scalar_v1171 * v1194);
        let v1204: f64 = (v1168 - v1192);
        let v1209: f64 = ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - (v1194 * v1199))) + (v1203 * v1204))) + (self.scalar_v314 * v730));
        let v1212: f64 = (v796 * self.scalar_v1211);
        let v1214: f64 = (((v1 + v1212)) as f64).sqrt();
        let v1215: f64 = (v1 + v1214);
        let v1216: f64 = (v1212 / v1215);
        let v1218: f64 = f64::powf(v1095, self.scalar_v1217);
        let v1219: f64 = (self.scalar_v1211 * v1218);
        let v1221: f64 = (((v1 + v1219)) as f64).sqrt();
        let v1222: f64 = (v1 + v1221);
        let v1223: f64 = (v1219 / v1222);
        let v1226: f64 = (v1 + (v1157 / self.scalar_v623));
        let v1228: f64 = (v1226 + (v1209 / self.scalar_v620));
        let v1239: f64 = (((if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v1226)) } else { v4 })) as f64).exp();
        let v1240: f64 = (((if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v1209) / self.scalar_v620))) } else { v4 })) as f64).exp();
        let v1246: f64 = (if self.scalar_v1230 { ((v1239 - v1240) / self.scalar_v1244) } else { (if self.scalar_v1224 { v1228 } else { v4 }) });
        let v1247: f64 = 0.010000000000000002;
        let v1248: f64 = (v1246 * v1246);
        let v1249: bool = (v1246 < v4);
        let v1250: f64 = 0.005000000000000001;
        let v1252: f64 = (((v1247 + v1248)) as f64).sqrt();
        let v1253: f64 = (v1252 - v1246);
        let v1256: bool = (!v1249);
        let v1259: f64 = (if v1256 { (v424 * (v1246 + v1252)) } else { (if v1249 { (v1250 / v1253) } else { v4 }) });
        let v1262: f64 = (v1 + (v424 * (v1216 + v1223)));
        let v1263: f64 = (v1259 * v1262);
        let v1266: f64 = (v1218 * self.scalar_v1265);
        let v1267: f64 = (self.scalar_v449 * v796);
        let v1269: f64 = ((v1267 - v1266) / v1263);
        let v1270: f64 = 0.0001;
        let v1271: f64 = (v736 / v1270);
        let v1272: bool = (v736 < v4);
        let v1273: f64 = ((v1271) as f64).exp();
        let v1274: f64 = (v1 + v1273);
        let v1278: bool = (!v1272);
        let v1280: f64 = (((-v1271)) as f64).exp();
        let v1281: f64 = (v1 + v1280);
        let v1285: f64 = (if v1278 { (v736 + (v1270 * ((v1281) as f64).ln())) } else { (if v1272 { (v1270 * ((v1274) as f64).ln()) } else { v4 }) });
        let v1322: f64 = (v786 / self.scalar_v491);
        let v1323: bool = (v1322 < self.scalar_v775);
        let v1324: f64 = ((v1322) as f64).exp();
        let v1326: bool = (!v1323);
        let v1327: f64 = (if v1326 { self.scalar_v780 } else { (if (!((v1285 / self.scalar_v1286) < self.scalar_v775)) { self.scalar_v780 } else { v896 }) });
        let v1356: f64 = (if (self.scalar_v510 && (!(((v1269 / self.scalar_v449) - 1000.0) < 40.0))) { 2.3538526683702e17 } else { (if (self.scalar_v510 && (!((self.scalar_v106 * (v736 - self.scalar_v268)) < self.scalar_v775))) { self.scalar_v780 } else { v1327 }) });
        let v1396: f64 = (self.scalar_v106 * v739);
        let v1397: f64 = (v1396 / self.scalar_v502);
        let v1398: bool = (v1397 < self.scalar_v775);
        let v1399: f64 = ((v1397) as f64).exp();
        let v1401: bool = (!v1398);
        let v1402: f64 = (if v1401 { self.scalar_v780 } else { v1356 });
        let v1406: f64 = (if v1401 { (v1402 * (v1 + (v1397 - self.scalar_v775))) } else { (if v1398 { v1399 } else { (if v1326 { (v1327 * (v1 + (v1322 - self.scalar_v775))) } else { (if v1323 { v1324 } else { v1285 }) }) }) });
        let v1432: f64 = (v786 / self.scalar_v463);
        let v1433: bool = (v1432 < self.scalar_v775);
        let v1434: f64 = ((v1432) as f64).exp();
        let v1436: bool = (!v1433);
        let v1437: f64 = (if v1436 { self.scalar_v780 } else { (if (self.scalar_v510 && (!((self.scalar_v106 * (v739 - self.scalar_v268)) < self.scalar_v775))) { self.scalar_v780 } else { v1402 }) });
        let v1444: f64 = (v1396 / self.scalar_v546);
        let v1445: bool = (v1444 < self.scalar_v775);
        let v1446: f64 = ((v1444) as f64).exp();
        let v1448: bool = (!v1445);
        let v1449: f64 = (if v1448 { self.scalar_v780 } else { v1437 });
        let v1453: f64 = (if v1448 { (v1449 * (v1 + (v1444 - self.scalar_v775))) } else { (if v1445 { v1446 } else { (if v1436 { (v1437 * (v1 + (v1432 - self.scalar_v775))) } else { (if v1433 { v1434 } else { v1406 }) }) }) });
        let v1456: f64 = (v797 / self.scalar_v476);
        let v1457: bool = (v1456 < self.scalar_v775);
        let v1458: f64 = ((v1456) as f64).exp();
        let v1460: bool = (!v1457);
        let v1461: f64 = (if v1460 { self.scalar_v780 } else { v1449 });
        let v1468: f64 = (v1396 / self.scalar_v556);
        let v1469: bool = (v1468 < self.scalar_v775);
        let v1470: f64 = ((v1468) as f64).exp();
        let v1472: bool = (!v1469);
        let v1473: f64 = (if v1472 { self.scalar_v780 } else { v1461 });
        let v1477: f64 = (if v1472 { (v1473 * (v1 + (v1468 - self.scalar_v775))) } else { (if v1469 { v1470 } else { (if v1460 { (v1461 * (v1 + (v1456 - self.scalar_v775))) } else { (if v1457 { v1458 } else { v1453 }) }) }) });
        let v1483: bool = (v1272 && self.scalar_v1482);
        let v1500: f64 = (if v1483 { (self.scalar_v292 * v736) } else { self.scalar_v617 });
        let v1502: f64 = 1e-30;
        let v1520: f64 = (f64::powf(((((v1500 * v1500) + v1502)) as f64).sqrt(), self.scalar_v1506) * ((self.scalar_v33 * (self.scalar_v1509 - ((v155 * v1500) * self.scalar_v1511))) - ((v1500 * (v462 * v1500)) * (v1500 + self.scalar_v1511))));
        let v1521: f64 = 0.16666666666666666;
        let v1528: f64 = (if v1483 { ((self.scalar_v583 * (self.scalar_v35 * v736)) / (self.scalar_v132 * (if v1483 { (v1520 * v1521) } else { v4 }))) } else { v1500 });
        let v1529: f64 = -0.001;
        let v1538: f64 = (if ((v1483 && (v1528 < v1529)) && (!(v1528 < self.scalar_v775))) { self.scalar_v780 } else { (if (v1483 && (!((self.scalar_v583 * (v1 - (self.scalar_v35 / (v32 * v1151)))) < self.scalar_v775))) { self.scalar_v780 } else { v1473 }) });
        let v1575: bool = (self.scalar_v1573 && (v730 < v4));
        let v1576: f64 = (self.scalar_v293 * v730);
        let v1595: f64 = (if v1575 { v1576 } else { self.scalar_v595 });
        let v1613: f64 = (f64::powf((((v1502 + (v1595 * v1595))) as f64).sqrt(), self.scalar_v1599) * ((self.scalar_v68 * (self.scalar_v1602 - ((v155 * v1595) * self.scalar_v1604))) - ((v1595 * (v462 * v1595)) * (v1595 + self.scalar_v1604))));
        let v1620: f64 = (if v1575 { ((self.scalar_v605 * (self.scalar_v70 * v730)) / (self.scalar_v154 * (if v1575 { (v1521 * v1613) } else { v4 }))) } else { v1595 });
        let v1629: f64 = (if ((v1575 && (v1620 < v1529)) && (!(v1620 < self.scalar_v775))) { self.scalar_v780 } else { (if (v1575 && (!((self.scalar_v605 * (v1 - (self.scalar_v70 / (v32 * (if v1575 { f64::powf((v1 - v1576), self.scalar_v1195) } else { v4 }))))) < self.scalar_v775))) { self.scalar_v780 } else { v1538 }) });
        let v1660: f64 = (v806 * self.scalar_v1211);
        let v1661: f64 = (v436 * (if v873 { (v874 * (v1 + (v869 - self.scalar_v775))) } else { (if v870 { v871 } else { v4 }) }));
        let v1662: f64 = (v1660 - self.scalar_v1211);
        let v1664: f64 = (((v1 + v1660)) as f64).sqrt();
        let v1665: f64 = (v1 + v1664);
        let v1668: f64 = (((v1 + v1661)) as f64).sqrt();
        let v1669: f64 = (v1 + v1668);
        let v1751: f64 = (v826 - v1);
        let v1752: f64 = (self.scalar_v1750 * v1751);
        let v1755: f64 = (((v1 + (v826 * self.scalar_v1675))) as f64).sqrt();
        let v1756: f64 = (v1 + v1755);
        let v1764: f64 = (self.scalar_v1762 * (v826 - v846));
        let v1771: f64 = (((v1 + (self.scalar_v1766 * (v826 + (v846 * self.scalar_v1690))))) as f64).sqrt();
        let v1772: f64 = (v1 + v1771);
        let v1776: f64 = (v1751 * self.scalar_v1762);
        let v1779: f64 = (((v1 + (v826 * self.scalar_v1766))) as f64).sqrt();
        let v1780: f64 = (v1 + v1779);
        let v1795: f64 = (if self.scalar_v1784 { (v771 - self.scalar_v1793) } else { v4 });
        let v1800: bool = (v1795 < v4);
        let v1801: bool = (self.scalar_v1784 && v1800);
        let v1804: f64 = (((self.scalar_v1797 + (if self.scalar_v1784 { (v1795 * v1795) } else { v1248 }))) as f64).sqrt();
        let v1805: f64 = (v1804 - v1795);
        let v1809: bool = (self.scalar_v1784 && (!v1800));
        let v1812: f64 = (if v1809 { (v424 * (v1795 + v1804)) } else { (if v1801 { (self.scalar_v1802 / v1805) } else { v4 }) });
        let v1816: f64 = (v1812 + (self.scalar_v1788 + (self.scalar_v340 * ((if self.scalar_v1744 { (v1752 / v1756) } else { v4 }) + (if self.scalar_v1775 { (v1776 / v1780) } else { (if self.scalar_v1759 { (v1764 / v1772) } else { v4 }) })))));
        let v1821: f64 = (if self.scalar_v1820 { v1 } else { (if self.scalar_v1784 { (v1812 / v1816) } else { v1 }) });
        let v1882: bool = (v1228 < v4);
        let v1884: f64 = (((v1247 + (v1228 * v1228))) as f64).sqrt();
        let v1885: f64 = (v1884 - v1228);
        let v1888: bool = (!v1882);
        let v1891: f64 = (if v1888 { (v424 * (v1228 + v1884)) } else { (if v1882 { (v1250 / v1885) } else { v4 }) });
        let v1901: bool = (v1269 > v4);
        let v1905: bool = (v730 < self.scalar_v1904);
        let v1908: f64 = ((-v1269) / self.scalar_v1907);
        let v1909: bool = (v1908 < self.scalar_v775);
        let v1911: bool = (v1905 && (v1901 && self.scalar_v1903));
        let v1916: bool = (v1911 && (!v1909));
        let v1917: f64 = (if v1916 { self.scalar_v780 } else { v1629 });
        let v1922: f64 = (self.scalar_v1904 - v730);
        let v1928: f64 = (self.scalar_v1925 * f64::powf((if v1911 { ((if v1916 { (v1917 * (v1 + (v1908 - self.scalar_v775))) } else { (if (v1909 && v1911) { ((v1908) as f64).exp() } else { v4 }) }) * v1922) } else { v4 }), self.scalar_v1926));
        let v2052: bool = (v1905 && (self.scalar_v2048 && ((v1901 && self.scalar_v1947) && self.scalar_v2049)));
        let v2061: f64 = (if v2052 { (f64::powf(v1922, self.scalar_v1926) * f64::powf((v1 - (v1269 / (v1269 + self.scalar_v2054))), self.scalar_v2058)) } else { v4 });
        let v2064: bool = (self.scalar_v1968 && v2052);
        let v2068: f64 = (if v2064 { ((v1269 - self.scalar_v2065) / self.scalar_v2054) } else { v4 });
        let v2072: f64 = (if v2064 { ((v2068 - v1) / self.scalar_v2070) } else { ((v736 - self.scalar_v1299) / 0.001) });
        let v2073: bool = (v2068 < v1);
        let v2089: f64 = (if (v2064 && (!v2073)) { (v2068 + (self.scalar_v2070 * (((v1 + (((-v2072)) as f64).exp())) as f64).ln())) } else { (if (v2064 && v2073) { (v1 + (self.scalar_v2070 * (((v1 + ((v2072) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v2101: f64 = (if (v2052 && (!((self.scalar_v1925 * (if v2064 { (v2061 * f64::powf(v2089, self.scalar_v2090)) } else { (if (self.scalar_v1965 && v2052) { v2061 } else { v4 }) })) < self.scalar_v775))) { self.scalar_v780 } else { (if (v1911 && (!(v1928 < self.scalar_v775))) { self.scalar_v780 } else { v1917 }) });
        let v2160: f64 = ((v739 - self.scalar_v1129) / self.scalar_v1130);
        let v2161: bool = (v739 < self.scalar_v1129);
        let v2162: f64 = ((v2160) as f64).exp();
        let v2163: f64 = (v1 + v2162);
        let v2168: bool = (!v2161);
        let v2170: f64 = (((-v2160)) as f64).exp();
        let v2171: f64 = (v1 + v2170);
        let v2175: f64 = (if v2168 { (self.scalar_v1129 - (self.scalar_v1130 * ((v2171) as f64).ln())) } else { (if v2161 { (v739 - (self.scalar_v1130 * ((v2163) as f64).ln())) } else { v4 }) });
        let v2178: f64 = (v1 - (self.scalar_v292 * v2175));
        let v2191: f64 = (v1216 * self.scalar_v2190);
        let v2192: f64 = (v1891 * v2191);
        let v2193: f64 = (v1223 * self.scalar_v2190);
        let v2194: f64 = (v1891 * v2193);
        let v2196: f64 = ((v766 - self.scalar_v1175) / self.scalar_v1075);
        let v2197: bool = (v766 < self.scalar_v1175);
        let v2198: f64 = ((v2196) as f64).exp();
        let v2199: f64 = (v1 + v2198);
        let v2204: bool = (!v2197);
        let v2206: f64 = (((-v2196)) as f64).exp();
        let v2207: f64 = (v1 + v2206);
        let v2211: f64 = (if v2204 { (self.scalar_v1175 - (self.scalar_v1075 * ((v2207) as f64).ln())) } else { (if v2197 { (v766 - (self.scalar_v1075 * ((v2199) as f64).ln())) } else { v4 }) });
        let v2213: f64 = (v1 - (v2211 / self.scalar_v245));
        let v2226: f64 = (self.scalar_v14 * ((self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - f64::powf(v2213, self.scalar_v1195))) + (self.scalar_v1171 * (v766 - v2211)))) + (self.scalar_v314 * v766))) * self.scalar_v2224));
        let v2228: f64 = ((v771 - self.scalar_v1175) / self.scalar_v1075);
        let v2229: bool = (v771 < self.scalar_v1175);
        let v2230: f64 = ((v2228) as f64).exp();
        let v2231: f64 = (v1 + v2230);
        let v2236: bool = (!v2229);
        let v2238: f64 = (((-v2228)) as f64).exp();
        let v2239: f64 = (v1 + v2238);
        let v2243: f64 = (if v2236 { (self.scalar_v1175 - (self.scalar_v1075 * ((v2239) as f64).ln())) } else { (if v2229 { (v771 - (self.scalar_v1075 * ((v2231) as f64).ln())) } else { v4 }) });
        let v2245: f64 = (v1 - (v2243 / self.scalar_v245));
        let v2257: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (v1 - f64::powf(v2245, self.scalar_v1195))) + (self.scalar_v1171 * (v771 - v2243)))) + (self.scalar_v314 * v771)))));
        let v2264: f64 = ((v744 - self.scalar_v2262) / self.scalar_v2258);
        let v2265: bool = (v744 < self.scalar_v2262);
        let v2266: f64 = ((v2264) as f64).exp();
        let v2267: f64 = (v1 + v2266);
        let v2272: bool = (!v2265);
        let v2274: f64 = (((-v2264)) as f64).exp();
        let v2275: f64 = (v1 + v2274);
        let v2279: f64 = (if v2272 { (self.scalar_v2262 - (self.scalar_v2258 * ((v2275) as f64).ln())) } else { (if v2265 { (v744 - (self.scalar_v2258 * ((v2267) as f64).ln())) } else { v4 }) });
        let v2283: f64 = (v1 - (v2279 / self.scalar_v291));
        let v2298: f64 = (v736 / self.scalar_v2297);
        let v2299: bool = (v2298 < self.scalar_v775);
        let v2300: f64 = ((v2298) as f64).exp();
        let v2302: bool = (!v2299);
        let v2303: f64 = (if v2302 { self.scalar_v780 } else { v2101 });
        let v2308: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * (v1 + (v2298 - self.scalar_v775))) } else { (if v2299 { v2300 } else { v1477 }) }));
        let v2313: f64 = ((if v1115 { (v918 / v1117) } else { (if v1107 { (v1110 / v1111) } else { v1023 }) }) * self.scalar_v2312);
        let v2314: f64 = (v32 + v1108);
        let v2328: f64 = (self.scalar_v106 * ((v766 - self.scalar_v225) / self.scalar_v2326));
        let v2329: bool = (v2328 < self.scalar_v775);
        let v2331: bool = (v2329 && self.scalar_v2330);
        let v2332: f64 = ((v2328) as f64).exp();
        let v2335: bool = (self.scalar_v2330 && (!v2329));
        let v2336: f64 = (if v2335 { self.scalar_v780 } else { v2303 });
        let v2342: f64 = (v806 * self.scalar_v2341);
        let v2345: f64 = (((v1 + (v436 * (if v2335 { (v2336 * (v1 + (v2328 - self.scalar_v775))) } else { (if v2331 { v2332 } else { v4 }) })))) as f64).sqrt();
        let v2346: f64 = (v1 + v2345);
        let v2348: f64 = (if self.scalar_v2330 { (v2342 / v2346) } else { (if self.scalar_v2317 { ((self.scalar_v2318 * (((v1662 / v1665) * self.scalar_v2189) + ((v1661 / v1669) * self.scalar_v2311))) / self.scalar_v674) } else { v4 }) });
        let v2356: f64 = (if self.scalar_v2354 { (v826 * self.scalar_v1211) } else { v4 });
        let v2357: f64 = (v2356 - self.scalar_v1211);
        let v2359: f64 = (((v1 + v2356)) as f64).sqrt();
        let v2360: f64 = (v1 + v2359);
        let v2364: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * (v1 + (v858 - self.scalar_v775))) } else { (if v859 { v860 } else { v4 }) })) } else { v4 });
        let v2366: f64 = (((v1 + v2364)) as f64).sqrt();
        let v2367: f64 = (v1 + v2366);
        let v2379: f64 = (self.scalar_v106 * (v771 - self.scalar_v225));
        let v2380: bool = (v2379 < self.scalar_v775);
        let v2382: bool = (v2380 && self.scalar_v2381);
        let v2383: f64 = ((v2379) as f64).exp();
        let v2386: bool = (self.scalar_v2381 && (!v2380));
        let v2387: f64 = (if v2386 { self.scalar_v780 } else { v2336 });
        let v2393: f64 = (v826 * self.scalar_v2392);
        let v2396: f64 = (((v1 + (v436 * (if v2386 { (v2387 * (v1 + (v2379 - self.scalar_v775))) } else { (if v2382 { v2383 } else { v4 }) })))) as f64).sqrt();
        let v2397: f64 = (v1 + v2396);
        let v2399: f64 = (if self.scalar_v2381 { (v2393 / v2397) } else { (if self.scalar_v2354 { ((self.scalar_v2371 * ((self.scalar_v2189 * (if self.scalar_v2354 { (v2357 / v2360) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (v2364 / v2367) } else { v4 })))) / self.scalar_v674) } else { v4 }) });
        let v2407: f64 = (if self.scalar_v2403 { (f64::powf(v1149, self.scalar_v2404) - v155) } else { v4 });
        let v2408: f64 = (if self.scalar_v2403 { v1132 } else { v4 });
        let v2409: bool = (v2408 < v4);
        let v2410: bool = (self.scalar_v2403 && v2409);
        let v2411: f64 = ((v2408) as f64).exp();
        let v2412: f64 = (v1 + v2411);
        let v2416: bool = (self.scalar_v2403 && (!v2409));
        let v2418: f64 = (((-v2408)) as f64).exp();
        let v2419: f64 = (v1 + v2418);
        let v2421: f64 = (if v2416 { (v2418 / v2419) } else { (if v2410 { (v1 / v2412) } else { v4 }) });
        let v2428: f64 = ((self.scalar_v106 * v1212) / self.scalar_v384);
        let v2429: f64 = (v424 / v1214);
        let v2431: f64 = (if self.scalar_v2403 { (v2428 * v2429) } else { v4 });
        let v2432: f64 = (v1891 * self.scalar_v2190);
        let v2437: f64 = (v741 * 0.2);
        let v2439: f64 = ((if self.scalar_v2403 { (v2308 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { (v155 + (v2407 * v2421)) } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { (v2431 * v2432) } else { v4 })));
        let v2448: f64 = (if self.scalar_v2403 { (v2192 + (v2308 * self.scalar_v2442)) } else { v4 });
        let v2457: f64 = (if self.scalar_v2456 { v2192 } else { (if self.scalar_v2403 { (v2448 * self.scalar_v2453) } else { v4 }) });
        let v2458: f64 = (if self.scalar_v2456 { v2194 } else { (if self.scalar_v2403 { (v2194 + (v2448 * self.scalar_v2449)) } else { v4 }) });
        let v2460: f64 = (v1266 + v1267);
        let v2461: f64 = (v2460 / v1263);
        let v2469: bool = (v2461 > v4);
        let v2470: f64 = (v2457 + v2458);
        let v2473: bool = (!v2469);
        let v2474: f64 = (self.scalar_v667 * v1891);
        let v2476: f64 = (if v2473 { (v1263 * v2474) } else { (if v2469 { (v2470 / v2461) } else { v4 }) });
        let v2489: f64 = (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (v2476 * self.scalar_v2484) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v2476) } else { v4 }) }) });
        let v2541: f64 = 0.0;
        let v2542: f64 = (self.scalar_v27 * v2541);
        let v2544: f64 = 0.0;
        let v2545: f64 = (self.scalar_v27 * v2544);
        let v2547: f64 = 0.0;
        let v2548: f64 = (self.scalar_v27 * v2547);
        let v2550: f64 = 0.0;
        let v2551: f64 = (self.scalar_v27 * v2550);
        let v2553: f64 = 0.0;
        let v2554: f64 = (self.scalar_v27 * v2553);
        let v2557: f64 = 0.0;
        let v2558: f64 = (self.scalar_v27 * v2557);
        let v2561: f64 = 0.0;
        let v2562: f64 = (self.scalar_v27 * v2561);
        let v2569: f64 = 0.0;
        let v2570: f64 = (self.scalar_v27 * v2569);
        let v2575: f64 = 0.0;
        let v2576: f64 = (self.scalar_v27 * v2575);
        let v2586: f64 = 0.0;
        let v2587: f64 = (v2489 * v2586);
        let v2611: f64 = (if v791 { (v792 * self.scalar_v2603) } else { (if v788 { (v789 * self.scalar_v2603) } else { v4 }) });
        let v2612: f64 = (if v791 { (v792 * self.scalar_v2604) } else { (if v788 { (v789 * self.scalar_v2604) } else { v4 }) });
        let v2627: f64 = (if v801 { (v802 * self.scalar_v2593) } else { (if v798 { (v799 * self.scalar_v2593) } else { v4 }) });
        let v2628: f64 = (if v801 { (v802 * self.scalar_v2613) } else { (if v798 { (v799 * self.scalar_v2613) } else { v4 }) });
        let v2629: f64 = (if v801 { (v802 * self.scalar_v2614) } else { (if v798 { (v799 * self.scalar_v2614) } else { v4 }) });
        let v2630: f64 = (if v801 { (v802 * self.scalar_v2594) } else { (if v798 { (v799 * self.scalar_v2594) } else { v4 }) });
        let v2652: f64 = (if v821 { (v822 * self.scalar_v2613) } else { (if v818 { (v819 * self.scalar_v2613) } else { v4 }) });
        let v2653: f64 = (if v821 { (v822 * self.scalar_v2639) } else { (if v818 { (v819 * self.scalar_v2639) } else { v4 }) });
        let v2654: f64 = (if v821 { (v822 * self.scalar_v2614) } else { (if v818 { (v819 * self.scalar_v2614) } else { v4 }) });
        let v2655: f64 = (if v821 { (v822 * self.scalar_v2594) } else { (if v818 { (v819 * self.scalar_v2594) } else { v4 }) });
        let v2673: f64 = (if v841 { (v842 * self.scalar_v2593) } else { (if v838 { (v839 * self.scalar_v2593) } else { v4 }) });
        let v2674: f64 = (if v841 { (v842 * self.scalar_v2614) } else { (if v838 { (v839 * self.scalar_v2614) } else { v4 }) });
        let v2675: f64 = (if v841 { (v842 * self.scalar_v2594) } else { (if v838 { (v839 * self.scalar_v2594) } else { v4 }) });
        let v2726: f64 = (if v884 { (v885 * self.scalar_v2593) } else { (if v881 { (v882 * self.scalar_v2593) } else { v4 }) });
        let v2727: f64 = (if v884 { (v885 * self.scalar_v2594) } else { (if v881 { (v882 * self.scalar_v2594) } else { v4 }) });
        let v2734: f64 = (if v895 { (v896 * self.scalar_v2593) } else { (if v892 { (v893 * self.scalar_v2593) } else { v4 }) });
        let v2735: f64 = (if v895 { (v896 * self.scalar_v2594) } else { (if v892 { (v893 * self.scalar_v2594) } else { v4 }) });
        let v2738: f64 = (v32 * v903);
        let v2739: f64 = ((v436 * v2726) / v2738);
        let v2740: f64 = ((v436 * v2727) / v2738);
        let v2743: f64 = (v32 * v906);
        let v2744: f64 = ((v436 * v2734) / v2743);
        let v2745: f64 = ((v436 * v2735) / v2743);
        let v2751: f64 = (v908 * v908);
        let v2757: f64 = (if v911 { v4 } else { (((v908 * (v32 * v2734)) - (v907 * v2744)) / v2751) });
        let v2758: f64 = (if v911 { v4 } else { (((v908 * (v32 * v2735)) - (v907 * v2745)) / v2751) });
        let v2775: f64 = (self.scalar_v104 * ((v2739 - v2744) - ((((v908 * v2739) - (v914 * v2744)) / v2751) / v915)));
        let v2776: f64 = (self.scalar_v104 * ((-v2745) - (((-(v914 * v2745)) / v2751) / v915)));
        let v2777: f64 = (self.scalar_v104 * (v2740 - ((v2740 / v908) / v915)));
        let v2779: f64 = (self.scalar_v2589 + v2777);
        let v2780: f64 = (v2775 / self.scalar_v352);
        let v2781: f64 = ((self.scalar_v0 + v2776) / self.scalar_v352);
        let v2782: f64 = (v2779 / self.scalar_v352);
        let v2792: f64 = (self.scalar_v352 * (v424 * v2780));
        let v2793: f64 = (self.scalar_v352 * (v424 * v2781));
        let v2794: f64 = (self.scalar_v352 * (v424 * v2782));
        let v2806: f64 = (if v921 { ((self.scalar_v933 * ((self.scalar_v106 * v2792) / v937)) - (if v927 { (self.scalar_v0 / v929) } else { (if v924 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v2807: f64 = (if v921 { ((self.scalar_v933 * ((self.scalar_v106 * v2793) / v937)) - (if v927 { (self.scalar_v2589 / v929) } else { (if v924 { self.scalar_v2589 } else { v4 }) })) } else { v4 });
        let v2808: f64 = (if v921 { (self.scalar_v933 * ((self.scalar_v106 * v2794) / v937)) } else { v4 });
        let v2809: f64 = (v942 * v2806);
        let v2811: f64 = (v942 * v2807);
        let v2813: f64 = (v942 * v2808);
        let v2818: f64 = (v32 * v954);
        let v2819: f64 = ((if v921 { (v2809 + v2809) } else { v4 }) / v2818);
        let v2820: f64 = ((if v921 { (v2811 + v2811) } else { v4 }) / v2818);
        let v2821: f64 = ((if v921 { (v2813 + v2813) } else { v4 }) / v2818);
        let v2827: f64 = (v955 * v955);
        let v2844: f64 = (if v959 { (v424 * (v2806 + v2819)) } else { (if v951 { ((-(v952 * (v2819 - v2806))) / v2827) } else { v4 }) });
        let v2845: f64 = (if v959 { (v424 * (v2807 + v2820)) } else { (if v951 { ((-(v952 * (v2820 - v2807))) / v2827) } else { v4 }) });
        let v2846: f64 = (if v959 { (v424 * (v2808 + v2821)) } else { (if v951 { ((-(v952 * (v2821 - v2808))) / v2827) } else { v4 }) });
        let v2862: f64 = (v970 * v970);
        let v2872: f64 = (if v921 { (((v970 * ((v966 * v2844) + (v962 * v2844))) - (v967 * (self.scalar_v964 * v2844))) / v2862) } else { v4 });
        let v2873: f64 = (if v921 { (((v970 * ((v966 * v2845) + (v962 * v2845))) - (v967 * (self.scalar_v964 * v2845))) / v2862) } else { v4 });
        let v2874: f64 = (if v921 { (((v970 * ((v966 * v2846) + (v962 * v2846))) - (v967 * (self.scalar_v964 * v2846))) / v2862) } else { v4 });
        let v2878: f64 = (v972 * v972);
        let v2888: f64 = (if v921 { (((v972 * v2780) - (v920 * v2872)) / v2878) } else { v4 });
        let v2889: f64 = (if v921 { (((v972 * v2781) - (v920 * v2873)) / v2878) } else { v4 });
        let v2890: f64 = (if v921 { (((v972 * v2782) - (v920 * v2874)) / v2878) } else { v4 });
        let v2894: f64 = (if v921 { (v2888 / self.scalar_v976) } else { v4 });
        let v2895: f64 = (if v921 { (v2889 / self.scalar_v976) } else { v4 });
        let v2896: f64 = (if v921 { (v2890 / self.scalar_v976) } else { v4 });
        let v2930: f64 = (if v921 { ((if v988 { (v2888 + (self.scalar_v976 * ((v990 * (-v2894)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2894) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2931: f64 = (if v921 { ((if v988 { (v2889 + (self.scalar_v976 * ((v990 * (-v2895)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2895) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2932: f64 = (if v921 { ((if v988 { (v2890 + (self.scalar_v976 * ((v990 * (-v2896)) / v991))) } else { (if v980 { (self.scalar_v976 * ((v981 * v2896) / v982)) } else { v4 }) }) / self.scalar_v1002) } else { v4 });
        let v2936: f64 = (if v921 { (v2844 / self.scalar_v965) } else { v4 });
        let v2937: f64 = (if v921 { (v2845 / self.scalar_v965) } else { v4 });
        let v2938: f64 = (if v921 { (v2846 / self.scalar_v965) } else { v4 });
        let v2960: f64 = (v32 * v1012);
        let v2978: f64 = ((v1015 * (((v1009 * ((v1007 * v2936) + (v1006 * (v436 * v2930)))) + (v1008 * v2936)) / v2960)) - (v1013 * ((v1014 * v2936) + (v1009 * (v32 * v2930)))));
        let v2979: f64 = (v1015 * v1015);
        let v2983: f64 = ((v1015 * (((v1009 * ((v1007 * v2937) + (v1006 * (v436 * v2931)))) + (v1008 * v2937)) / v2960)) - (v1013 * ((v1014 * v2937) + (v1009 * (v32 * v2931)))));
        let v2987: f64 = ((v1015 * (((v1009 * ((v1007 * v2938) + (v1006 * (v436 * v2932)))) + (v1008 * v2938)) / v2960)) - (v1013 * ((v1014 * v2938) + (v1009 * (v32 * v2932)))));
        let v2989: f64 = (if v921 { (v2978 / v2979) } else { v4 });
        let v2990: f64 = (if v921 { (v2983 / v2979) } else { v4 });
        let v2991: f64 = (if v921 { (v2987 / v2979) } else { v4 });
        let v2997: f64 = ((v1017 * v2757) + (v912 * v2989));
        let v3000: f64 = ((v1017 * v2758) + (v912 * v2990));
        let v3001: f64 = (v912 * v2991);
        let v3008: f64 = (v1021 * v1021);
        let v3018: f64 = (if v921 { (((v1021 * ((-v2989) + v2997)) - (v1020 * v2997)) / v3008) } else { v4 });
        let v3019: f64 = (if v921 { (((v1021 * ((-v2990) + v3000)) - (v1020 * v3000)) / v3008) } else { v4 });
        let v3020: f64 = (if v921 { (((v1021 * ((-v2991) + v3001)) - (v1020 * v3001)) / v3008) } else { v4 });
        let v3033: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2792) + (v935 * v3018))) } else { v4 });
        let v3034: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2793) + (v935 * v3019))) } else { v4 });
        let v3035: f64 = (if v921 { (self.scalar_v106 * ((v1023 * v2794) + (v935 * v3020))) } else { v4 });
        let v3051: f64 = (if v921 { ((v32 * v3033) + ((v1029 * v2757) + (v912 * (v2757 + v3033)))) } else { v4 });
        let v3052: f64 = (if v921 { ((v32 * v3034) + ((v1029 * v2758) + (v912 * (v2758 + v3034)))) } else { v4 });
        let v3053: f64 = (if v921 { ((v32 * v3035) + (v912 * v3035)) } else { v4 });
        let v3057: f64 = (if v921 { (v424 * v3033) } else { v4 });
        let v3058: f64 = (if v921 { (v424 * v3034) } else { v4 });
        let v3059: f64 = (if v921 { (v424 * v3035) } else { v4 });
        let v3060: f64 = (v1035 * v3057);
        let v3062: f64 = (v1035 * v3058);
        let v3064: f64 = (v1035 * v3059);
        let v3072: f64 = (v32 * v1041);
        let v3073: f64 = ((if v921 { (v3051 + (v3060 + v3060)) } else { v4 }) / v3072);
        let v3074: f64 = ((if v921 { (v3052 + (v3062 + v3062)) } else { v4 }) / v3072);
        let v3075: f64 = ((if v921 { (v3053 + (v3064 + v3064)) } else { v4 }) / v3072);
        let v3088: f64 = (v1046 * v1046);
        let v3101: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3051) - (v1032 * (v3073 - v3057))) / v3088) } else { (if v1040 { (v3057 + v3073) } else { v4 }) }) });
        let v3102: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3052) - (v1032 * (v3074 - v3058))) / v3088) } else { (if v1040 { (v3058 + v3074) } else { v4 }) }) });
        let v3103: f64 = (if v1051 { v4 } else { (if v1045 { (((v1046 * v3053) - (v1032 * (v3075 - v3059))) / v3088) } else { (if v1040 { (v3059 + v3075) } else { v4 }) }) });
        let v3122: f64 = (if v921 { (self.scalar_v1059 * v2780) } else { v4 });
        let v3123: f64 = (if v921 { (self.scalar_v1059 * v2781) } else { v4 });
        let v3124: f64 = (if v921 { (self.scalar_v1059 * v2782) } else { v4 });
        let v3131: f64 = (v1062 * v3122);
        let v3133: f64 = (v1062 * v3123);
        let v3135: f64 = (v1062 * v3124);
        let v3140: f64 = (v32 * v1069);
        let v3159: f64 = (v1080 * v1080);
        let v3175: f64 = (self.scalar_v963 * v2780);
        let v3176: f64 = (self.scalar_v963 * v2781);
        let v3177: f64 = (self.scalar_v963 * v2782);
        let v3181: f64 = (v1086 * v1086);
        let v3208: f64 = (v914 * v914);
        let v3216: f64 = (if v1091 { (((v914 * (v32 * v2727)) - (v1092 * v2740)) / v3208) } else { v3103 });
        let v3217: f64 = (if v1091 { (if v779 { (v781 * self.scalar_v2593) } else { (if v776 { (v777 * self.scalar_v2593) } else { v4 }) }) } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3101) + (v1052 * v3101))) } else { v4 }) });
        let v3219: f64 = (if v1091 { (if v779 { (v781 * self.scalar_v2594) } else { (if v776 { (v777 * self.scalar_v2594) } else { v4 }) }) } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3103) + (v1052 * v3103))) } else { v4 }) });
        let v3220: f64 = (v2757 + (if v1091 { (((v914 * (v32 * v2726)) - (v1092 * v2739)) / v3208) } else { v3101 }));
        let v3221: f64 = (v2758 + (if v1091 { v4 } else { v3102 }));
        let v3225: f64 = (if v1107 { (v424 * v3220) } else { v4 });
        let v3226: f64 = (if v1107 { (v424 * v3221) } else { v4 });
        let v3227: f64 = (if v1107 { (v424 * v3216) } else { v4 });
        let v3231: f64 = (v1111 * v1111);
        let v3250: f64 = (v1117 * v1117);
        let v3260: f64 = (if v1115 { (((v1117 * v2775) - (v918 * ((self.scalar_v0 + v2775) - self.scalar_v0))) / v3250) } else { (if v1107 { (((v1111 * v3225) - (v1110 * v3225)) / v3231) } else { v3018 }) });
        let v3261: f64 = (if v1115 { (((v1117 * v2776) - (v918 * (v2776 - self.scalar_v2589))) / v3250) } else { (if v1107 { (((v1111 * v3226) - (v1110 * v3226)) / v3231) } else { v3019 }) });
        let v3266: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2780)) - (v1079 * (v2780 + v2872))) / v3159)) } else { v4 }) });
        let v3267: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2781)) - (v1079 * (v2781 + v2873))) / v3159)) } else { v4 }) });
        let v3268: f64 = (if v1091 { v4 } else { (if v1078 { (self.scalar_v245 * (((v1080 * (v32 * v2782)) - (v1079 * (v2782 + v2874))) / v3159)) } else { v4 }) });
        let v3278: f64 = (if v1091 { (-((if v1091 { v2780 } else { (if v921 { (((v1086 * v3175) - (v1085 * v2780)) / v3181) } else { v4 }) }) / self.scalar_v963)) } else { (if v921 { ((-v3175) / v3181) } else { v4 }) });
        let v3279: f64 = (if v1091 { (-((if v1091 { v2781 } else { (if v921 { (((v1086 * v3176) - (v1085 * v2781)) / v3181) } else { v4 }) }) / self.scalar_v963)) } else { (if v921 { ((-v3176) / v3181) } else { v4 }) });
        let v3280: f64 = (if v1091 { (-((if v1091 { v2782 } else { (if v921 { (((v1086 * v3177) - (v1085 * v2782)) / v3181) } else { v4 }) }) / self.scalar_v963)) } else { (if v921 { ((-v3177) / v3181) } else { v4 }) });
        let v3303: f64 = (if v1140 { (-(self.scalar_v1130 * ((v1142 * self.scalar_v3293) / v1143))) } else { (if v1133 { (self.scalar_v2589 - (self.scalar_v1130 * ((v1134 * self.scalar_v3281) / v1135))) } else { v4 }) });
        let v3304: f64 = (if v1140 { (-(self.scalar_v1130 * ((v1142 * self.scalar_v3294) / v1143))) } else { (if v1133 { (self.scalar_v0 - (self.scalar_v1130 * ((v1134 * self.scalar_v3282) / v1135))) } else { v4 }) });
        let v3307: f64 = (-(self.scalar_v292 * v3303));
        let v3308: f64 = (-(self.scalar_v292 * v3304));
        let v3311: f64 = (self.scalar_v1150 * f64::powf(v1149, self.scalar_v3309));
        let v3322: f64 = ((self.scalar_v1152 * (-(v3307 * v3311))) + (v155 * (self.scalar_v2589 - v3303)));
        let v3323: f64 = ((self.scalar_v1152 * (-(v3308 * v3311))) + (v155 * (self.scalar_v0 - v3304)));
        let v3328: f64 = (if self.scalar_v1163 { (self.scalar_v0 + (if v1091 { v4 } else { (if v921 { (v3122 + (((if v921 { (self.scalar_v1064 * v2780) } else { v4 }) + (v3131 + v3131)) / v3140)) } else { v4 }) })) } else { self.scalar_v3324 });
        let v3329: f64 = (if self.scalar_v1163 { (self.scalar_v2589 + (if v1091 { self.scalar_v0 } else { (if v921 { (v3123 + (((if v921 { (self.scalar_v1064 * v2781) } else { v4 }) + (v3133 + v3133)) / v3140)) } else { v4 }) })) } else { self.scalar_v3325 });
        let v3331: f64 = (if self.scalar_v1167 { self.scalar_v0 } else { v3328 });
        let v3332: f64 = (if self.scalar_v1167 { v4 } else { v3329 });
        let v3333: f64 = (if self.scalar_v1167 { self.scalar_v2589 } else { (if self.scalar_v1163 { (if v1091 { self.scalar_v2589 } else { (if v921 { (v3124 + (((if v921 { (self.scalar_v1064 * v2782) } else { v4 }) + (v3135 + v3135)) / v3140)) } else { v4 }) }) } else { v4 }) });
        let v3337: f64 = (v1121 * v1121);
        let v3338: f64 = (((v1121 * v3331) - (v1176 * v3266)) / v3337);
        let v3342: f64 = (((v1121 * v3332) - (v1176 * v3267)) / v3337);
        let v3346: f64 = (((v1121 * v3333) - (v1176 * v3268)) / v3337);
        let v3389: f64 = (if v1185 { (-((v1189 * v3266) + (v1121 * ((v1187 * (-v3338)) / v1188)))) } else { (if v1178 { (v3331 - ((v1181 * v3266) + (v1121 * ((v1179 * v3338) / v1180)))) } else { v4 }) });
        let v3390: f64 = (if v1185 { (-((v1189 * v3267) + (v1121 * ((v1187 * (-v3342)) / v1188)))) } else { (if v1178 { (v3332 - ((v1181 * v3267) + (v1121 * ((v1179 * v3342) / v1180)))) } else { v4 }) });
        let v3391: f64 = (if v1185 { (-((v1189 * v3268) + (v1121 * ((v1187 * (-v3346)) / v1188)))) } else { (if v1178 { (v3333 - ((v1181 * v3268) + (v1121 * ((v1179 * v3346) / v1180)))) } else { v4 }) });
        let v3394: f64 = (self.scalar_v1193 * f64::powf(v1125, self.scalar_v3392));
        let v3395: f64 = (v3278 * v3394);
        let v3396: f64 = (v3279 * v3394);
        let v3397: f64 = (v3280 * v3394);
        let v3406: f64 = (self.scalar_v1195 * f64::powf(v1198, self.scalar_v3404));
        let v3440: f64 = ((self.scalar_v1196 * (-((v1199 * v3395) + (v1194 * ((-(v3389 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3395)) + (v1203 * (v3331 - v3389))));
        let v3441: f64 = ((self.scalar_v1196 * (-((v1199 * v3396) + (v1194 * ((-(v3390 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3396)) + (v1203 * (v3332 - v3390))));
        let v3442: f64 = ((self.scalar_v1196 * (-((v1199 * v3397) + (v1194 * ((-(v3391 / self.scalar_v245)) * v3406))))) + ((v1204 * (self.scalar_v1171 * v3397)) + (v1203 * (v3333 - v3391))));
        let v3445: f64 = (self.scalar_v1170 * v3442);
        let v3448: f64 = ((self.scalar_v1170 * v3440) + self.scalar_v3446);
        let v3449: f64 = ((self.scalar_v1170 * v3441) + self.scalar_v3447);
        let v3450: f64 = (self.scalar_v1211 * v2611);
        let v3451: f64 = (self.scalar_v1211 * v2612);
        let v3452: f64 = (v32 * v1214);
        let v3453: f64 = (v3450 / v3452);
        let v3454: f64 = (v3451 / v3452);
        let v3458: f64 = (v1215 * v1215);
        let v3459: f64 = (((v1215 * v3450) - (v1212 * v3453)) / v3458);
        let v3463: f64 = (((v1215 * v3451) - (v1212 * v3454)) / v3458);
        let v3466: f64 = (self.scalar_v1217 * f64::powf(v1095, self.scalar_v3464));
        let v3467: f64 = (v3217 * v3466);
        let v3468: f64 = ((if v1091 { v4 } else { (if v921 { (self.scalar_v1056 * ((v1053 * v3102) + (v1052 * v3102))) } else { v4 }) }) * v3466);
        let v3469: f64 = (v3219 * v3466);
        let v3470: f64 = (self.scalar_v1211 * v3467);
        let v3471: f64 = (self.scalar_v1211 * v3468);
        let v3472: f64 = (self.scalar_v1211 * v3469);
        let v3473: f64 = (v32 * v1221);
        let v3480: f64 = (v1222 * v1222);
        let v3481: f64 = (((v1222 * v3470) - (v1219 * (v3470 / v3473))) / v3480);
        let v3485: f64 = (((v1222 * v3471) - (v1219 * (v3471 / v3473))) / v3480);
        let v3489: f64 = (((v1222 * v3472) - (v1219 * (v3472 / v3473))) / v3480);
        let v3490: f64 = (v3322 / self.scalar_v623);
        let v3491: f64 = (v3323 / self.scalar_v623);
        let v3493: f64 = (v3449 / self.scalar_v620);
        let v3494: f64 = (v3445 / self.scalar_v620);
        let v3495: f64 = (v3491 + (v3448 / self.scalar_v620));
        let v3530: f64 = (((v1239 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v3491)) } else { v4 })) - (v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3448) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244);
        let v3533: f64 = (if self.scalar_v1230 { ((v1239 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * v3490)) } else { v4 })) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3490 } else { v4 }) });
        let v3534: f64 = (if self.scalar_v1230 { v3530 } else { (if self.scalar_v1224 { v3495 } else { v4 }) });
        let v3535: f64 = (if self.scalar_v1230 { ((-(v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3449) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3493 } else { v4 }) });
        let v3536: f64 = (if self.scalar_v1230 { ((-(v1240 * (if self.scalar_v1230 { (self.scalar_v106 * (self.scalar_v702 * ((-v3445) / self.scalar_v620))) } else { v4 }))) / self.scalar_v1244) } else { (if self.scalar_v1224 { v3494 } else { v4 }) });
        let v3537: f64 = (v1246 * v3533);
        let v3538: f64 = (v3537 + v3537);
        let v3539: f64 = (v1246 * v3534);
        let v3540: f64 = (v3539 + v3539);
        let v3541: f64 = (v1246 * v3535);
        let v3542: f64 = (v3541 + v3541);
        let v3543: f64 = (v1246 * v3536);
        let v3544: f64 = (v3543 + v3543);
        let v3545: f64 = (v32 * v1252);
        let v3546: f64 = (v3538 / v3545);
        let v3547: f64 = (v3540 / v3545);
        let v3548: f64 = (v3542 / v3545);
        let v3549: f64 = (v3544 / v3545);
        let v3556: f64 = (v1253 * v1253);
        let v3590: f64 = ((v1262 * (if v1256 { (v424 * (v3533 + v3546)) } else { (if v1249 { ((-(v1250 * (v3546 - v3533))) / v3556) } else { v4 }) })) + (v1259 * (v424 * v3459)));
        let v3593: f64 = ((v1262 * (if v1256 { (v424 * (v3534 + v3547)) } else { (if v1249 { ((-(v1250 * (v3547 - v3534))) / v3556) } else { v4 }) })) + (v1259 * (v424 * (v3463 + v3481))));
        let v3596: f64 = ((v1262 * (if v1256 { (v424 * (v3535 + v3548)) } else { (if v1249 { ((-(v1250 * (v3548 - v3535))) / v3556) } else { v4 }) })) + (v1259 * (v424 * v3485)));
        let v3599: f64 = ((v1262 * (if v1256 { (v424 * (v3536 + v3549)) } else { (if v1249 { ((-(v1250 * (v3549 - v3536))) / v3556) } else { v4 }) })) + (v1259 * (v424 * v3489)));
        let v3611: f64 = (v1263 * v1263);
        let v3701: f64 = (if v1323 { (v1324 * self.scalar_v3697) } else { (if v1278 { (self.scalar_v2589 + (v1270 * ((v1280 * self.scalar_v3635) / v1281))) } else { (if v1272 { (v1270 * ((v1273 * self.scalar_v3625) / v1274)) } else { v4 }) }) });
        let v3702: f64 = (if v1323 { (v1324 * self.scalar_v3698) } else { (if v1278 { (self.scalar_v0 + (v1270 * ((v1280 * self.scalar_v3636) / v1281))) } else { (if v1272 { (v1270 * ((v1273 * self.scalar_v3626) / v1274)) } else { v4 }) }) });
        let v3896: f64 = (if v1436 { (v1437 * self.scalar_v3887) } else { (if v1433 { (v1434 * self.scalar_v3887) } else { (if v1401 { (v1402 * self.scalar_v3830) } else { (if v1398 { (v1399 * self.scalar_v3830) } else { (if v1326 { (v1327 * self.scalar_v3697) } else { v3701 }) }) }) }) });
        let v3908: f64 = (if v1445 { v4 } else { (if v1436 { (v1437 * self.scalar_v3888) } else { (if v1433 { (v1434 * self.scalar_v3888) } else { (if v1401 { v4 } else { (if v1398 { v4 } else { (if v1326 { (v1327 * self.scalar_v3698) } else { v3702 }) }) }) }) }) });
        let v3912: f64 = (if v1448 { (v1449 * self.scalar_v3903) } else { (if v1445 { (v1446 * self.scalar_v3903) } else { (if v1436 { v4 } else { (if v1433 { v4 } else { (if v1401 { (v1402 * self.scalar_v3831) } else { (if v1398 { (v1399 * self.scalar_v3831) } else { v4 }) }) }) }) }) });
        let v3955: f64 = (if v1472 { (v1473 * self.scalar_v3944) } else { (if v1469 { (v1470 * self.scalar_v3944) } else { (if v1460 { v4 } else { (if v1457 { v4 } else { (if v1448 { (v1449 * self.scalar_v3902) } else { (if v1445 { (v1446 * self.scalar_v3902) } else { v3896 }) }) }) }) }) });
        let v4297: f64 = (self.scalar_v1211 * v2627);
        let v4298: f64 = (self.scalar_v1211 * v2628);
        let v4299: f64 = (self.scalar_v1211 * v2629);
        let v4300: f64 = (self.scalar_v1211 * v2630);
        let v4301: f64 = (v436 * (if v873 { (v874 * self.scalar_v2593) } else { (if v870 { (v871 * self.scalar_v2593) } else { v4 }) }));
        let v4302: f64 = (v436 * (if v873 { (v874 * self.scalar_v2613) } else { (if v870 { (v871 * self.scalar_v2613) } else { v4 }) }));
        let v4303: f64 = (v436 * (if v873 { (v874 * self.scalar_v2614) } else { (if v870 { (v871 * self.scalar_v2614) } else { v4 }) }));
        let v4304: f64 = (v436 * (if v873 { (v874 * self.scalar_v2594) } else { (if v870 { (v871 * self.scalar_v2594) } else { v4 }) }));
        let v4305: f64 = (v32 * v1664);
        let v4313: f64 = (v1665 * v1665);
        let v4327: f64 = (v32 * v1668);
        let v4335: f64 = (v1669 * v1669);
        let v4569: f64 = (v32 * v1755);
        let v4577: f64 = (v1756 * v1756);
        let v4593: f64 = (if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2654)) - (v1752 * ((self.scalar_v1675 * v2654) / v4569))) / v4577) } else { v4 });
        let v4598: f64 = (self.scalar_v1762 * v2652);
        let v4599: f64 = (self.scalar_v1762 * v2653);
        let v4602: f64 = (self.scalar_v1762 * v2654);
        let v4609: f64 = (self.scalar_v1766 * v2652);
        let v4610: f64 = (self.scalar_v1766 * v2653);
        let v4613: f64 = (self.scalar_v1766 * v2654);
        let v4615: f64 = (v32 * v1771);
        let v4625: f64 = (v1772 * v1772);
        let v4655: f64 = (v32 * v1779);
        let v4663: f64 = (v1780 * v1780);
        let v4672: f64 = (((v1780 * v4602) - (v1776 * (v4613 / v4655))) / v4663);
        let v4677: f64 = (if self.scalar_v1775 { (((v1780 * v4598) - (v1776 * (v4609 / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * v4598) - (v1764 * (v4609 / v4615))) / v4625) } else { v4 }) });
        let v4678: f64 = (if self.scalar_v1775 { (((v1780 * v4599) - (v1776 * (v4610 / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * v4599) - (v1764 * (v4610 / v4615))) / v4625) } else { v4 }) });
        let v4680: f64 = (if self.scalar_v1775 { v4672 } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (v2654 - v2674))) - (v1764 * ((self.scalar_v1766 * (v2654 + (self.scalar_v1690 * v2674))) / v4615))) / v4625) } else { v4 }) });
        let v4682: f64 = (if self.scalar_v1775 { (((v1780 * (self.scalar_v1762 * v2655)) - (v1776 * ((self.scalar_v1766 * v2655) / v4655))) / v4663) } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (v2655 - v2675))) - (v1764 * ((self.scalar_v1766 * (v2655 + (self.scalar_v1690 * v2675))) / v4615))) / v4625) } else { v4 }) });
        let v4687: f64 = (v1795 * self.scalar_v4683);
        let v4688: f64 = (v4687 + v4687);
        let v4689: f64 = (v1795 * self.scalar_v4684);
        let v4691: f64 = (v1795 * self.scalar_v4685);
        let v4692: f64 = (v4691 + v4691);
        let v4693: f64 = (v1795 * self.scalar_v4686);
        let v4703: f64 = (v32 * v1804);
        let v4704: f64 = ((if self.scalar_v1784 { v4688 } else { v4 }) / v4703);
        let v4705: f64 = ((if self.scalar_v1784 { (v4689 + v4689) } else { v4 }) / v4703);
        let v4706: f64 = ((if self.scalar_v1784 { v4 } else { v3538 }) / v4703);
        let v4707: f64 = ((if self.scalar_v1784 { v4688 } else { v3540 }) / v4703);
        let v4708: f64 = ((if self.scalar_v1784 { v4692 } else { v3542 }) / v4703);
        let v4709: f64 = ((if self.scalar_v1784 { v4692 } else { v3544 }) / v4703);
        let v4710: f64 = ((if self.scalar_v1784 { (v4693 + v4693) } else { v4 }) / v4703);
        let v4711: f64 = ((if self.scalar_v1784 { v4692 } else { v4 }) / v4703);
        let v4721: f64 = (v1805 * v1805);
        let v4767: f64 = (if v1809 { (v424 * (self.scalar_v4683 + v4704)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4704 - self.scalar_v4683))) / v4721) } else { v4 }) });
        let v4768: f64 = (if v1809 { (v424 * (self.scalar_v4684 + v4705)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4705 - self.scalar_v4684))) / v4721) } else { v4 }) });
        let v4769: f64 = (if v1809 { (v424 * v4706) } else { (if v1801 { ((-(self.scalar_v1802 * v4706)) / v4721) } else { v4 }) });
        let v4770: f64 = (if v1809 { (v424 * (self.scalar_v4683 + v4707)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4707 - self.scalar_v4683))) / v4721) } else { v4 }) });
        let v4771: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4708)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4708 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4772: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4709)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4709 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4773: f64 = (if v1809 { (v424 * (self.scalar_v4686 + v4710)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4710 - self.scalar_v4686))) / v4721) } else { v4 }) });
        let v4774: f64 = (if v1809 { (v424 * (self.scalar_v4685 + v4711)) } else { (if v1801 { ((-(self.scalar_v1802 * (v4711 - self.scalar_v4685))) / v4721) } else { v4 }) });
        let v4780: f64 = (self.scalar_v340 * ((if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2652)) - (v1752 * ((self.scalar_v1675 * v2652) / v4569))) / v4577) } else { v4 }) + v4677));
        let v4782: f64 = (self.scalar_v340 * (if self.scalar_v1775 { v4 } else { (if self.scalar_v1759 { (((v1772 * (self.scalar_v1762 * (-v2673))) - (v1764 * ((self.scalar_v1766 * (self.scalar_v1690 * v2673)) / v4615))) / v4625) } else { v4 }) }));
        let v4783: f64 = (self.scalar_v340 * (v4593 + v4680));
        let v4796: f64 = (v1816 * v1816);
        let v4799: f64 = (v1812 * (v4768 + (self.scalar_v340 * ((if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2653)) - (v1752 * ((self.scalar_v1675 * v2653) / v4569))) / v4577) } else { v4 }) + v4678))));
        let v4818: f64 = (v1812 * (v4772 + (self.scalar_v340 * (v4593 + (if self.scalar_v1775 { v4672 } else { (if self.scalar_v1759 { (((v1772 * v4602) - (v1764 * (v4613 / v4615))) / v4625) } else { v4 }) })))));
        let v4822: f64 = (v1812 * (v4773 + (self.scalar_v340 * ((if self.scalar_v1744 { (((v1756 * (self.scalar_v1750 * v2655)) - (v1752 * ((self.scalar_v1675 * v2655) / v4569))) / v4577) } else { v4 }) + v4682))));
        let v5144: f64 = (v1228 * v3490);
        let v5146: f64 = (v1228 * v3495);
        let v5148: f64 = (v1228 * v3493);
        let v5150: f64 = (v1228 * v3494);
        let v5152: f64 = (v32 * v1884);
        let v5153: f64 = ((v5144 + v5144) / v5152);
        let v5154: f64 = ((v5146 + v5146) / v5152);
        let v5155: f64 = ((v5148 + v5148) / v5152);
        let v5156: f64 = ((v5150 + v5150) / v5152);
        let v5163: f64 = (v1885 * v1885);
        let v5186: f64 = (if v1888 { (v424 * (v3490 + v5153)) } else { (if v1882 { ((-(v1250 * (v5153 - v3490))) / v5163) } else { v4 }) });
        let v5187: f64 = (if v1888 { (v424 * (v3495 + v5154)) } else { (if v1882 { ((-(v1250 * (v5154 - v3495))) / v5163) } else { v4 }) });
        let v5188: f64 = (if v1888 { (v424 * (v3493 + v5155)) } else { (if v1882 { ((-(v1250 * (v5155 - v3493))) / v5163) } else { v4 }) });
        let v5189: f64 = (if v1888 { (v424 * (v3494 + v5156)) } else { (if v1882 { ((-(v1250 * (v5156 - v3494))) / v5163) } else { v4 }) });
        let v6067: f64 = (if v2168 { (-(self.scalar_v1130 * ((v2170 * self.scalar_v3293) / v2171))) } else { (if v2161 { (self.scalar_v2589 - (self.scalar_v1130 * ((v2162 * self.scalar_v3281) / v2163))) } else { v4 }) });
        let v6068: f64 = (if v2168 { (-(self.scalar_v1130 * ((v2170 * self.scalar_v3294) / v2171))) } else { (if v2161 { (self.scalar_v0 - (self.scalar_v1130 * ((v2162 * self.scalar_v3282) / v2163))) } else { v4 }) });
        let v6074: f64 = (self.scalar_v1150 * f64::powf(v2178, self.scalar_v3309));
        let v6096: f64 = ((v2191 * v5186) + (v1891 * (self.scalar_v2190 * v3459)));
        let v6099: f64 = ((v2191 * v5187) + (v1891 * (self.scalar_v2190 * v3463)));
        let v6100: f64 = (v2191 * v5188);
        let v6101: f64 = (v2191 * v5189);
        let v6105: f64 = (v2193 * v5186);
        let v6108: f64 = ((v2193 * v5187) + (v1891 * (self.scalar_v2190 * v3481)));
        let v6111: f64 = ((v2193 * v5188) + (v1891 * (self.scalar_v2190 * v3485)));
        let v6114: f64 = ((v2193 * v5189) + (v1891 * (self.scalar_v2190 * v3489)));
        let v6159: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6139) / v2207))) } else { (if v2197 { (self.scalar_v0 - (self.scalar_v1075 * ((v2198 * self.scalar_v6115) / v2199))) } else { v4 }) });
        let v6160: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6140) / v2207))) } else { (if v2197 { (self.scalar_v2590 - (self.scalar_v1075 * ((v2198 * self.scalar_v6116) / v2199))) } else { v4 }) });
        let v6161: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6141) / v2207))) } else { (if v2197 { (self.scalar_v2591 - (self.scalar_v1075 * ((v2198 * self.scalar_v6117) / v2199))) } else { v4 }) });
        let v6162: f64 = (if v2204 { (-(self.scalar_v1075 * ((v2206 * self.scalar_v6142) / v2207))) } else { (if v2197 { (self.scalar_v2589 - (self.scalar_v1075 * ((v2198 * self.scalar_v6118) / v2199))) } else { v4 }) });
        let v6172: f64 = (self.scalar_v1195 * f64::powf(v2213, self.scalar_v3404));
        let v6215: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3446 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6159 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v0 - v6159))))))));
        let v6216: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6160 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2590 - v6160)))) + self.scalar_v6201))));
        let v6217: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6161 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2591 - v6161)))) + self.scalar_v6202))));
        let v6218: f64 = (self.scalar_v14 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3447 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6162 / self.scalar_v245)) * v6172))) + (self.scalar_v1171 * (self.scalar_v2589 - v6162))))))));
        let v6257: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6140) / v2239))) } else { (if v2229 { (self.scalar_v2590 - (self.scalar_v1075 * ((v2230 * self.scalar_v6116) / v2231))) } else { v4 }) });
        let v6258: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6240) / v2239))) } else { (if v2229 { (self.scalar_v2592 - (self.scalar_v1075 * ((v2230 * self.scalar_v6219) / v2231))) } else { v4 }) });
        let v6259: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6141) / v2239))) } else { (if v2229 { (self.scalar_v2591 - (self.scalar_v1075 * ((v2230 * self.scalar_v6117) / v2231))) } else { v4 }) });
        let v6260: f64 = (if v2236 { (-(self.scalar_v1075 * ((v2238 * self.scalar_v6142) / v2239))) } else { (if v2229 { (self.scalar_v2589 - (self.scalar_v1075 * ((v2230 * self.scalar_v6118) / v2231))) } else { v4 }) });
        let v6270: f64 = (self.scalar_v1195 * f64::powf(v2245, self.scalar_v3404));
        let v6312: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v6201 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6257 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2590 - v6257))))))));
        let v6313: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * ((self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6258 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2592 - v6258)))) + self.scalar_v6299))));
        let v6314: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v6202 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6259 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2591 - v6259))))))));
        let v6315: f64 = (self.scalar_v13 * (self.scalar_v2224 * (self.scalar_v313 * (self.scalar_v3447 + (self.scalar_v1170 * ((self.scalar_v1196 * (-((-(v6260 / self.scalar_v245)) * v6270))) + (self.scalar_v1171 * (self.scalar_v2589 - v6260))))))));
        let v6338: f64 = (if v2272 { (-(self.scalar_v2258 * ((v2274 * self.scalar_v6328) / v2275))) } else { (if v2265 { (self.scalar_v0 - (self.scalar_v2258 * ((v2266 * self.scalar_v6316) / v2267))) } else { v4 }) });
        let v6339: f64 = (if v2272 { (-(self.scalar_v2258 * ((v2274 * self.scalar_v6329) / v2275))) } else { (if v2265 { (self.scalar_v2589 - (self.scalar_v2258 * ((v2266 * self.scalar_v6317) / v2267))) } else { v4 }) });
        let v6346: f64 = (self.scalar_v2280 * f64::powf(v2283, self.scalar_v6344));
        let v6367: f64 = (if v2299 { (v2300 * self.scalar_v6362) } else { (if v1472 { v4 } else { (if v1469 { v4 } else { (if v1460 { (v1461 * self.scalar_v3918) } else { (if v1457 { (v1458 * self.scalar_v3918) } else { (if v1448 { v4 } else { v3908 }) }) }) }) }) });
        let v6373: f64 = (if v2302 { v4 } else { (if v2299 { v4 } else { (if v1472 { (v1473 * self.scalar_v3945) } else { (if v1469 { (v1470 * self.scalar_v3945) } else { (if v1460 { (v1461 * self.scalar_v3917) } else { (if v1457 { (v1458 * self.scalar_v3917) } else { v3912 }) }) }) }) }) });
        let v6377: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * self.scalar_v6361) } else { (if v2299 { (v2300 * self.scalar_v6361) } else { v3955 }) }));
        let v6378: f64 = (self.scalar_v2296 * v6373);
        let v6379: f64 = (self.scalar_v2296 * (if v2302 { (v2303 * self.scalar_v6362) } else { v6367 }));
        let v6380: f64 = (self.scalar_v2296 * (if v2302 { v4 } else { (if v2299 { v4 } else { (if v1472 { v4 } else { (if v1469 { v4 } else { (if v1460 { (v1461 * self.scalar_v3919) } else { (if v1457 { (v1458 * self.scalar_v3919) } else { v4 }) }) }) }) }) }));
        let v6381: f64 = (self.scalar_v2296 * (if v2302 { v4 } else { (if v2299 { v4 } else { (if v1472 { v4 } else { (if v1469 { v4 } else { (if v1460 { (v1461 * self.scalar_v3920) } else { (if v1457 { (v1458 * self.scalar_v3920) } else { v4 }) }) }) }) }) }));
        let v6384: f64 = (self.scalar_v2312 * (if v1115 { (((v1117 * v2777) - (v918 * v2779)) / v3250) } else { (if v1107 { (((v1111 * v3227) - (v1110 * v3227)) / v3231) } else { v3020 }) }));
        let v6402: f64 = ((self.scalar_v2189 * (((v1665 * v4297) - (v1662 * (v4297 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4301) - (v1661 * (v4301 / v4327))) / v4335)));
        let v6403: f64 = ((self.scalar_v2189 * (((v1665 * v4298) - (v1662 * (v4298 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4302) - (v1661 * (v4302 / v4327))) / v4335)));
        let v6404: f64 = ((self.scalar_v2189 * (((v1665 * v4299) - (v1662 * (v4299 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4303) - (v1661 * (v4303 / v4327))) / v4335)));
        let v6405: f64 = ((self.scalar_v2189 * (((v1665 * v4300) - (v1662 * (v4300 / v4305))) / v4313)) + (self.scalar_v2311 * (((v1669 * v4304) - (v1661 * (v4304 / v4327))) / v4335)));
        let v6450: f64 = (v32 * v2345);
        let v6458: f64 = (v2346 * v2346);
        let v6459: f64 = (((v2346 * (self.scalar_v2341 * v2627)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6422) } else { (if v2331 { (v2332 * self.scalar_v6422) } else { v4 }) })) / v6450))) / v6458);
        let v6463: f64 = (((v2346 * (self.scalar_v2341 * v2628)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6423) } else { (if v2331 { (v2332 * self.scalar_v6423) } else { v4 }) })) / v6450))) / v6458);
        let v6467: f64 = (((v2346 * (self.scalar_v2341 * v2629)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6424) } else { (if v2331 { (v2332 * self.scalar_v6424) } else { v4 }) })) / v6450))) / v6458);
        let v6471: f64 = (((v2346 * (self.scalar_v2341 * v2630)) - (v2342 * ((v436 * (if v2335 { (v2336 * self.scalar_v6425) } else { (if v2331 { (v2332 * self.scalar_v6425) } else { v4 }) })) / v6450))) / v6458);
        let v6472: f64 = (if self.scalar_v2330 { v6459 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6402) / self.scalar_v674) } else { v4 }) });
        let v6473: f64 = (if self.scalar_v2330 { v6463 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6403) / self.scalar_v674) } else { v4 }) });
        let v6474: f64 = (if self.scalar_v2330 { v6467 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6404) / self.scalar_v674) } else { v4 }) });
        let v6475: f64 = (if self.scalar_v2330 { v6471 } else { (if self.scalar_v2317 { ((self.scalar_v2318 * v6405) / self.scalar_v674) } else { v4 }) });
        let v6488: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2652) } else { v4 });
        let v6489: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2653) } else { v4 });
        let v6490: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2654) } else { v4 });
        let v6491: f64 = (if self.scalar_v2354 { (self.scalar_v1211 * v2655) } else { v4 });
        let v6492: f64 = (v32 * v2359);
        let v6500: f64 = (v2360 * v2360);
        let v6522: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2613) } else { (if v859 { (v860 * self.scalar_v2613) } else { v4 }) })) } else { v4 });
        let v6523: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2639) } else { (if v859 { (v860 * self.scalar_v2639) } else { v4 }) })) } else { v4 });
        let v6524: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2614) } else { (if v859 { (v860 * self.scalar_v2614) } else { v4 }) })) } else { v4 });
        let v6525: f64 = (if self.scalar_v2354 { (v436 * (if v862 { (v863 * self.scalar_v2594) } else { (if v859 { (v860 * self.scalar_v2594) } else { v4 }) })) } else { v4 });
        let v6526: f64 = (v32 * v2366);
        let v6534: f64 = (v2367 * v2367);
        let v6560: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6488) - (v2357 * (v6488 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6522) - (v2364 * (v6522 / v6526))) / v6534) } else { v4 })));
        let v6561: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6489) - (v2357 * (v6489 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6523) - (v2364 * (v6523 / v6526))) / v6534) } else { v4 })));
        let v6562: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6490) - (v2357 * (v6490 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6524) - (v2364 * (v6524 / v6526))) / v6534) } else { v4 })));
        let v6563: f64 = ((self.scalar_v2189 * (if self.scalar_v2354 { (((v2360 * v6491) - (v2357 * (v6491 / v6492))) / v6500) } else { v4 })) + (self.scalar_v2311 * (if self.scalar_v2354 { (((v2367 * v6525) - (v2364 * (v6525 / v6526))) / v6534) } else { v4 })));
        let v6600: f64 = (v32 * v2396);
        let v6608: f64 = (v2397 * v2397);
        let v6609: f64 = (((v2397 * (self.scalar_v2392 * v2652)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2613) } else { (if v2382 { (v2383 * self.scalar_v2613) } else { v4 }) })) / v6600))) / v6608);
        let v6613: f64 = (((v2397 * (self.scalar_v2392 * v2653)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2639) } else { (if v2382 { (v2383 * self.scalar_v2639) } else { v4 }) })) / v6600))) / v6608);
        let v6617: f64 = (((v2397 * (self.scalar_v2392 * v2654)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2614) } else { (if v2382 { (v2383 * self.scalar_v2614) } else { v4 }) })) / v6600))) / v6608);
        let v6621: f64 = (((v2397 * (self.scalar_v2392 * v2655)) - (v2393 * ((v436 * (if v2386 { (v2387 * self.scalar_v2594) } else { (if v2382 { (v2383 * self.scalar_v2594) } else { v4 }) })) / v6600))) / v6608);
        let v6627: f64 = (v1821 * (if self.scalar_v2381 { v6609 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6560) / self.scalar_v674) } else { v4 }) }));
        let v6631: f64 = ((v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4768) - v4799) / v4796) } else { v4 }) })) + (v1821 * (if self.scalar_v2381 { v6613 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6561) / self.scalar_v674) } else { v4 }) })));
        let v6637: f64 = (v1821 * (if self.scalar_v2381 { v6617 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6562) / self.scalar_v674) } else { v4 }) }));
        let v6643: f64 = ((v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4773) - v4822) / v4796) } else { v4 }) })) + (v1821 * (if self.scalar_v2381 { v6621 } else { (if self.scalar_v2354 { ((self.scalar_v2371 * v6563) / self.scalar_v674) } else { v4 }) })));
        let v6657: f64 = (self.scalar_v2404 * f64::powf(v1149, self.scalar_v6655));
        let v6667: f64 = (v2412 * v2412);
        let v6675: f64 = (v2418 * self.scalar_v6673);
        let v6676: f64 = (v2418 * self.scalar_v6674);
        let v6680: f64 = (v2419 * v2419);
        let v6690: f64 = ((v2421 * (if self.scalar_v2403 { (v3307 * v6657) } else { v4 })) + (v2407 * (if v2416 { (((v2419 * v6675) - (v2418 * v6675)) / v6680) } else { (if v2410 { ((-(v2411 * self.scalar_v6662)) / v6667) } else { v4 }) })));
        let v6693: f64 = ((v2421 * (if self.scalar_v2403 { (v3308 * v6657) } else { v4 })) + (v2407 * (if v2416 { (((v2419 * v6676) - (v2418 * v6676)) / v6680) } else { (if v2410 { ((-(v2411 * self.scalar_v6663)) / v6667) } else { v4 }) })));
        let v6706: f64 = (v1214 * v1214);
        let v6725: f64 = ((v2432 * (if self.scalar_v2403 { ((v2429 * ((self.scalar_v106 * v3450) / self.scalar_v384)) + (v2428 * ((-(v424 * v3453)) / v6706))) } else { v4 })) + (v2431 * (self.scalar_v2190 * v5186)));
        let v6728: f64 = ((v2432 * (if self.scalar_v2403 { ((v2429 * ((self.scalar_v106 * v3451) / self.scalar_v384)) + (v2428 * ((-(v424 * v3454)) / v6706))) } else { v4 })) + (v2431 * (self.scalar_v2190 * v5187)));
        let v6743: f64 = (if self.scalar_v2403 { (v6380 / self.scalar_v2297) } else { v4 });
        let v6759: f64 = ((v2439 * self.scalar_v6746) + (v2437 * ((if self.scalar_v2403 { (v6379 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { v6693 } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { v6728 } else { v4 })))));
        let v6763: f64 = (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v6377 / self.scalar_v2297) } else { v4 }) + ((if self.scalar_v2403 { (self.scalar_v2157 * (if self.scalar_v2403 { v6690 } else { v4 })) } else { v4 }) + (if self.scalar_v2403 { v6725 } else { v4 })))) } else { v4 });
        let v6782: f64 = (self.scalar_v2442 * v6380);
        let v6788: f64 = (if self.scalar_v2403 { (v6096 + (self.scalar_v2442 * v6377)) } else { v4 });
        let v6789: f64 = (if self.scalar_v2403 { (self.scalar_v2442 * v6378) } else { v4 });
        let v6790: f64 = (if self.scalar_v2403 { (v6099 + (self.scalar_v2442 * v6379)) } else { v4 });
        let v6791: f64 = (if self.scalar_v2403 { (v6100 + v6782) } else { v4 });
        let v6792: f64 = (if self.scalar_v2403 { (v6101 + v6782) } else { v4 });
        let v6793: f64 = (if self.scalar_v2403 { (self.scalar_v2442 * v6381) } else { v4 });
        let v6822: f64 = (if self.scalar_v2456 { v6096 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6788) } else { v4 }) });
        let v6823: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6789) } else { v4 }) });
        let v6824: f64 = (if self.scalar_v2456 { v6099 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6790) } else { v4 }) });
        let v6825: f64 = (if self.scalar_v2456 { v6100 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6791) } else { v4 }) });
        let v6826: f64 = (if self.scalar_v2456 { v6101 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6792) } else { v4 }) });
        let v6827: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2453 * v6793) } else { v4 }) });
        let v6828: f64 = (if self.scalar_v2456 { v6105 } else { (if self.scalar_v2403 { (v6105 + (self.scalar_v2449 * v6788)) } else { v4 }) });
        let v6829: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2449 * v6789) } else { v4 }) });
        let v6830: f64 = (if self.scalar_v2456 { v6108 } else { (if self.scalar_v2403 { (v6108 + (self.scalar_v2449 * v6790)) } else { v4 }) });
        let v6831: f64 = (if self.scalar_v2456 { v6111 } else { (if self.scalar_v2403 { (v6111 + (self.scalar_v2449 * v6791)) } else { v4 }) });
        let v6832: f64 = (if self.scalar_v2456 { v6114 } else { (if self.scalar_v2403 { (v6114 + (self.scalar_v2449 * v6792)) } else { v4 }) });
        let v6833: f64 = (if self.scalar_v2456 { v4 } else { (if self.scalar_v2403 { (self.scalar_v2449 * v6793) } else { v4 }) });
        let v6837: f64 = (if self.scalar_v2456 { v6380 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6380) } else { v4 }) });
        let v6855: f64 = (v2461 * v2461);
        let v6870: f64 = (((v2461 * (v6824 + v6830)) - (v2470 * (((v1263 * ((self.scalar_v1265 * v3467) + (self.scalar_v449 * v2612))) - (v2460 * v3593)) / v3611))) / v6855);
        let v6902: f64 = (if v2473 { ((v2474 * v3590) + (v1263 * (self.scalar_v667 * v5186))) } else { (if v2469 { (((v2461 * (v6822 + v6828)) - (v2470 * (((v1263 * (self.scalar_v449 * v2611)) - (v2460 * v3590)) / v3611))) / v6855) } else { v4 }) });
        let v6903: f64 = (if v2473 { v4 } else { (if v2469 { ((v6823 + v6829) / v2461) } else { v4 }) });
        let v6904: f64 = (if v2473 { ((v2474 * v3593) + (v1263 * (self.scalar_v667 * v5187))) } else { (if v2469 { v6870 } else { v4 }) });
        let v6905: f64 = (if v2473 { ((v2474 * v3596) + (v1263 * (self.scalar_v667 * v5188))) } else { (if v2469 { (((v2461 * (v6825 + v6831)) - (v2470 * (((v1263 * (self.scalar_v1265 * v3468)) - (v2460 * v3596)) / v3611))) / v6855) } else { v4 }) });
        let v6906: f64 = (if v2473 { ((v2474 * v3599) + (v1263 * (self.scalar_v667 * v5189))) } else { (if v2469 { (((v2461 * (v6826 + v6832)) - (v2470 * (((v1263 * (self.scalar_v1265 * v3469)) - (v2460 * v3599)) / v3611))) / v6855) } else { v4 }) });
        let v6907: f64 = (if v2473 { v4 } else { (if v2469 { ((v6827 + v6833) / v2461) } else { v4 }) });
        let v6979: f64 = (v6312 + (if self.scalar_v2351 { ((v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4767) - (v1812 * (v4767 + v4780))) / v4796) } else { v4 }) })) + v6627) } else { v4 }));
        let v6981: f64 = (v6312 + (if self.scalar_v2351 { (v6627 + (v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4770) - (v1812 * (v4770 + v4780))) / v4796) } else { v4 }) }))) } else { v4 }));
        let v6982: f64 = (v6314 + (if self.scalar_v2351 { ((v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4771) - (v1812 * (v4771 + v4783))) / v4796) } else { v4 }) })) + v6637) } else { v4 }));
        let v6985: f64 = (v6314 + (if self.scalar_v2351 { (v6637 + (v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4774) - (v1812 * (v4774 + v4783))) / v4796) } else { v4 }) }))) } else { v4 }));
        let v7149: f64 = 1.0;
        let v7156: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2456 { v6377 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6377) } else { v4 }) }) + ((self.scalar_v2157 * v3322) + v6822))) * v7149));
        let v7157: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6823 + (if self.scalar_v2456 { v6378 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6378) } else { v4 }) }))) * v7149));
        let v7158: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2456 { v6379 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6379) } else { v4 }) }) + ((self.scalar_v2157 * v3323) + v6824))) * v7149));
        let v7159: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6825 + v6837)) * v7149));
        let v7160: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6826 + v6837)) * v7149));
        let v7161: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6827 + (if self.scalar_v2456 { v6381 } else { (if self.scalar_v2403 { (self.scalar_v2443 * v6381) } else { v4 }) }))) * v7149));
        let v7166: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v2176 * ((self.scalar_v1152 * (-((-(self.scalar_v292 * v6067)) * v6074))) + (v155 * (self.scalar_v2589 - v6067)))))));
        let v7167: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v2176 * ((self.scalar_v1152 * (-((-(self.scalar_v292 * v6068)) * v6074))) + (v155 * (self.scalar_v0 - v6068)))))));
        let v7180: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6828)));
        let v7181: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6829)));
        let v7182: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * (self.scalar_v2312 * v3260)) + (v2313 * v3220)) + ((self.scalar_v2187 * v3448) + v6830)))));
        let v7183: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * (self.scalar_v2312 * v3261)) + (v2313 * v3221)) + ((self.scalar_v2187 * v3449) + v6831)))));
        let v7184: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (((v2314 * v6384) + (v2313 * v3216)) + ((self.scalar_v2187 * v3445) + v6832)))));
        let v7185: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6833)));
        let v7190: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v304 * ((self.scalar_v2281 * (-((-(v6338 / self.scalar_v291)) * v6346))) + (v32 * (self.scalar_v0 - v6338)))))));
        let v7191: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (self.scalar_v304 * ((self.scalar_v2281 * (-((-(v6339 / self.scalar_v291)) * v6346))) + (v32 * (self.scalar_v2589 - v6339)))))));
        let v7204: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6763)));
        let v7205: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { ((v2439 * self.scalar_v6745) + (v2437 * (if self.scalar_v2403 { (v6378 / self.scalar_v2297) } else { v4 }))) } else { v4 }))));
        let v7206: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { v6759 } else { v4 }))));
        let v7207: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v2431 * (self.scalar_v2190 * v5188)) } else { v4 }) + v6743)) } else { v4 }))));
        let v7208: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * ((if self.scalar_v2403 { (v2431 * (self.scalar_v2190 * v5189)) } else { v4 }) + v6743)) } else { v4 }))));
        let v7209: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2403 { (v2437 * (if self.scalar_v2403 { (v6381 / self.scalar_v2297) } else { v4 })) } else { v4 }))));
        let v7214: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7210));
        let v7215: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7211));
        let v7220: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7216));
        let v7221: f64 = (self.scalar_v27 * (v7149 * self.scalar_v7217));
        let v7267: f64 = (v7149 * (self.scalar_v0 * (v6314 + (if self.scalar_v2351 { (v6637 + (v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4772) - v4818) / v4796) } else { v4 }) }))) } else { v4 }))));
        let v7270: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6979)));
        let v7271: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6313 + (if self.scalar_v2351 { v6631 } else { v4 })))));
        let v7272: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2351 { (v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { ((-(v1812 * v4782)) / v4796) } else { v4 }) })) } else { v4 }))));
        let v7273: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (if self.scalar_v2351 { (v2399 * (if self.scalar_v1820 { v4 } else { (if self.scalar_v1784 { (((v1816 * v4769) - (v1812 * v4769)) / v4796) } else { v4 }) })) } else { v4 }))));
        let v7274: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6981)));
        let v7275: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6982)));
        let v7276: f64 = (self.scalar_v27 * v7267);
        let v7277: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6315 + (if self.scalar_v2351 { v6643 } else { v4 })))));
        let v7278: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * v6985)));
        let v7310: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6215 + (if self.scalar_v2351 { (self.scalar_v14 * v6472) } else { v6472 })))));
        let v7311: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6216 + (if self.scalar_v2351 { (self.scalar_v14 * v6473) } else { v6473 })))));
        let v7312: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6217 + (if self.scalar_v2351 { (self.scalar_v14 * v6474) } else { v6474 })))));
        let v7313: f64 = (self.scalar_v27 * (v7149 * (self.scalar_v0 * (v6218 + (if self.scalar_v2351 { (self.scalar_v14 * v6475) } else { v6475 })))));
        let v7326: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6902) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6902) } else { v4 }) }) }));
        let v7327: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6903) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6903) } else { v4 }) }) }));
        let v7328: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6904) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6904) } else { v4 }) }) }));
        let v7329: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6905) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6905) } else { v4 }) }) }));
        let v7330: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6906) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6906) } else { v4 }) }) }));
        let v7331: f64 = (v2586 * (if self.scalar_v2488 { v4 } else { (if self.scalar_v2483 { (self.scalar_v2484 * v6907) } else { (if self.scalar_v2478 { (self.scalar_v2449 * v6907) } else { v4 }) }) }));
        let v7332: f64 = (v2489 * v7149);

        let d2542_dn4: f64 = v7156;
        let d2542_dn5: f64 = v7157;
        let d2542_dn6: f64 = v7158;
        let d2542_dn7: f64 = v7159;
        let d2542_dn8: f64 = v7160;
        let d2542_dn10: f64 = v7161;
        let v2542_reactive_nodes: [usize; 6] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2542_reactive_node_derivatives: [f64; 6] = [d2542_dn4, d2542_dn5, d2542_dn6, d2542_dn7, d2542_dn8, d2542_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &v2542_reactive_nodes,
            &v2542_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2545_dn4: f64 = v7166;
        let d2545_dn5: f64 = v7167;
        stamper.stamp_current_reactive_node2(
            Some(nodes[5]),
            Some(nodes[4]),
            nodes[4],
            multiplicity * (d2545_dn4),
            nodes[5],
            multiplicity * (d2545_dn5),
        );
        let d2548_dn4: f64 = v7180;
        let d2548_dn5: f64 = v7181;
        let d2548_dn6: f64 = v7182;
        let d2548_dn7: f64 = v7183;
        let d2548_dn8: f64 = v7184;
        let d2548_dn10: f64 = v7185;
        let v2548_reactive_nodes: [usize; 6] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2548_reactive_node_derivatives: [f64; 6] = [d2548_dn4, d2548_dn5, d2548_dn6, d2548_dn7, d2548_dn8, d2548_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            &v2548_reactive_nodes,
            &v2548_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2551_dn3: f64 = v7190;
        let d2551_dn7: f64 = v7191;
        stamper.stamp_current_reactive_node2(
            Some(nodes[3]),
            Some(nodes[7]),
            nodes[3],
            multiplicity * (d2551_dn3),
            nodes[7],
            multiplicity * (d2551_dn7),
        );
        let d2554_dn4: f64 = v7204;
        let d2554_dn5: f64 = v7205;
        let d2554_dn6: f64 = v7206;
        let d2554_dn7: f64 = v7207;
        let d2554_dn8: f64 = v7208;
        let d2554_dn10: f64 = v7209;
        let v2554_reactive_nodes: [usize; 6] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2554_reactive_node_derivatives: [f64; 6] = [d2554_dn4, d2554_dn5, d2554_dn6, d2554_dn7, d2554_dn8, d2554_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            &v2554_reactive_nodes,
            &v2554_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2558_dn1: f64 = v7214;
        let d2558_dn2: f64 = v7215;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d2558_dn1),
            nodes[2],
            multiplicity * (d2558_dn2),
        );
        let d2562_dn0: f64 = v7220;
        let d2562_dn1: f64 = v7221;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d2562_dn0),
            nodes[1],
            multiplicity * (d2562_dn1),
        );
        let d2570_dn0: f64 = v7270;
        let d2570_dn1: f64 = v7271;
        let d2570_dn3: f64 = v7272;
        let d2570_dn4: f64 = v7273;
        let d2570_dn5: f64 = v7270;
        let d2570_dn6: f64 = v7274;
        let d2570_dn7: f64 = v7275;
        let d2570_dn8: f64 = v7276;
        let d2570_dn9: f64 = v7277;
        let d2570_dn10: f64 = v7278;
        let v2570_reactive_nodes: [usize; 10] = [nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]];
        let v2570_reactive_node_derivatives: [f64; 10] = [d2570_dn0, d2570_dn1, d2570_dn3, d2570_dn4, d2570_dn5, d2570_dn6, d2570_dn7, d2570_dn8, d2570_dn9, d2570_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[9]),
            &v2570_reactive_nodes,
            &v2570_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2576_dn5: f64 = v7310;
        let d2576_dn6: f64 = v7311;
        let d2576_dn7: f64 = v7312;
        let d2576_dn8: f64 = v7312;
        let d2576_dn10: f64 = v7313;
        let v2576_reactive_nodes: [usize; 5] = [nodes[5], nodes[6], nodes[7], nodes[8], nodes[10]];
        let v2576_reactive_node_derivatives: [f64; 5] = [d2576_dn5, d2576_dn6, d2576_dn7, d2576_dn8, d2576_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[10]),
            &v2576_reactive_nodes,
            &v2576_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2587_dn4: f64 = v7326;
        let d2587_dn5: f64 = v7327;
        let d2587_dn6: f64 = v7328;
        let d2587_dn7: f64 = v7329;
        let d2587_dn8: f64 = v7330;
        let d2587_dn10: f64 = v7331;
        let d2587_dn11: f64 = v7332;
        let v2587_reactive_nodes: [usize; 7] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[10], nodes[11]];
        let v2587_reactive_node_derivatives: [f64; 7] = [d2587_dn4, d2587_dn5, d2587_dn6, d2587_dn7, d2587_dn8, d2587_dn10, d2587_dn11];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[4]),
            &v2587_reactive_nodes,
            &v2587_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
    }
}
