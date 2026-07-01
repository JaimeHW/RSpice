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
        let v30: f64 = 0.001;
        let v31: f64 = 2.0;
        let v46: f64 = 0.1;
        let v154: f64 = 3.0;
        let v392: f64 = 1e-6;
        let v395: f64 = 0.5;
        let v407: f64 = 4.0;
        let v433: f64 = 6.0;
        let v670: f64 = ctx.node_voltage(nodes[5]);
        let v671: f64 = ctx.node_voltage(nodes[6]);
        let v673: f64 = (self.scalar_v0 * (v670 - v671));
        let v674: f64 = ctx.node_voltage(nodes[7]);
        let v676: f64 = (self.scalar_v0 * (v670 - v674));
        let v677: f64 = ctx.node_voltage(nodes[3]);
        let v679: f64 = (self.scalar_v0 * (v670 - v677));
        let v680: f64 = ctx.node_voltage(nodes[4]);
        let v682: f64 = (self.scalar_v0 * (v680 - v677));
        let v684: f64 = (self.scalar_v0 * (v680 - v670));
        let v686: f64 = (self.scalar_v0 * (v671 - v674));
        let v687: f64 = ctx.node_voltage(nodes[2]);
        let v690: f64 = ctx.node_voltage(nodes[1]);
        let v692: f64 = (self.scalar_v0 * (v690 - v680));
        let v697: f64 = (self.scalar_v0 * (v690 - ctx.node_voltage(nodes[0])));
        let v698: f64 = ctx.node_voltage(nodes[9]);
        let v700: f64 = (self.scalar_v0 * (v698 - v671));
        let v703: f64 = (self.scalar_v0 * (ctx.node_voltage(nodes[8]) - v698));
        let v706: f64 = (((v676 + v684) - v686) - v700);
        let v710: f64 = ((v706 + (v692 + (-v697))) - v703);
        let v711: f64 = (v697 + v710);
        let v712: f64 = (self.scalar_v105 * v676);
        let v714: bool = (v712 < self.scalar_v713);
        let v715: f64 = ((v712) as f64).exp();
        let v717: bool = (!v714);
        let v719: f64 = (if v717 { self.scalar_v718 } else { v4 });
        let v724: f64 = (self.scalar_v105 * v679);
        let v725: f64 = (v724 / self.scalar_v355);
        let v726: bool = (v725 < self.scalar_v713);
        let v727: f64 = ((v725) as f64).exp();
        let v729: bool = (!v726);
        let v730: f64 = (if v729 { self.scalar_v718 } else { v719 });
        let v734: f64 = (if v729 { (v730 * (v1 + (v725 - self.scalar_v713))) } else { (if v726 { v727 } else { v4 }) });
        let v735: f64 = (self.scalar_v105 * v706);
        let v736: bool = (v735 < self.scalar_v713);
        let v737: f64 = ((v735) as f64).exp();
        let v739: bool = (!v736);
        let v740: f64 = (if v739 { self.scalar_v718 } else { v730 });
        let v744: f64 = (if v739 { (v740 * (v1 + (v735 - self.scalar_v713))) } else { (if v736 { v737 } else { v4 }) });
        let v745: f64 = (self.scalar_v105 * v684);
        let v746: bool = (v745 < self.scalar_v713);
        let v747: f64 = ((v745) as f64).exp();
        let v749: bool = (!v746);
        let v750: f64 = (if v749 { self.scalar_v718 } else { v740 });
        let v755: f64 = (self.scalar_v105 * v711);
        let v756: bool = (v755 < self.scalar_v713);
        let v757: f64 = ((v755) as f64).exp();
        let v759: bool = (!v756);
        let v760: f64 = (if v759 { self.scalar_v718 } else { v750 });
        let v764: f64 = (if v759 { (v760 * (v1 + (v755 - self.scalar_v713))) } else { (if v756 { v757 } else { v4 }) });
        let v766: f64 = (self.scalar_v105 * (v711 - self.scalar_v203));
        let v767: bool = (v766 < self.scalar_v713);
        let v768: f64 = ((v766) as f64).exp();
        let v770: bool = (!v767);
        let v771: f64 = (if v770 { self.scalar_v718 } else { v760 });
        let v777: f64 = (self.scalar_v105 * (v706 - self.scalar_v203));
        let v778: bool = (v777 < self.scalar_v713);
        let v779: f64 = ((v777) as f64).exp();
        let v781: bool = (!v778);
        let v782: f64 = (if v781 { self.scalar_v718 } else { v771 });
        let v788: f64 = (self.scalar_v105 * (v676 - self.scalar_v203));
        let v789: bool = (v788 < self.scalar_v713);
        let v790: f64 = ((v788) as f64).exp();
        let v792: bool = (!v789);
        let v793: f64 = (if v792 { self.scalar_v718 } else { v782 });
        let v797: f64 = (if v792 { (v793 * (v1 + (v788 - self.scalar_v713))) } else { (if v789 { v790 } else { v4 }) });
        let v799: f64 = (self.scalar_v105 * (v673 - self.scalar_v203));
        let v800: bool = (v799 < self.scalar_v713);
        let v801: f64 = ((v799) as f64).exp();
        let v803: bool = (!v800);
        let v804: f64 = (if v803 { self.scalar_v718 } else { v793 });
        let v808: f64 = (if v803 { (v804 * (v1 + (v799 - self.scalar_v713))) } else { (if v800 { v801 } else { v4 }) });
        let v811: f64 = (((v1 + (v407 * v797))) as f64).sqrt();
        let v814: f64 = (((v1 + (v407 * v808))) as f64).sqrt();
        let v815: f64 = (v31 * v808);
        let v816: f64 = (v1 + v814);
        let v817: f64 = (v815 / v816);
        let v819: bool = (v817 < self.scalar_v818);
        let v820: f64 = (if v819 { self.scalar_v818 } else { v817 });
        let v822: f64 = (v1 + v811);
        let v823: f64 = (v822 / v816);
        let v826: f64 = (self.scalar_v103 * ((v811 - v814) - ((v823) as f64).ln()));
        let v828: f64 = ((v686 + v826) / self.scalar_v323);
        let v829: bool = (v828 > v4);
        let v830: f64 = 100.0;
        let v831: bool = (v673 < v830);
        let v832: bool = (v829 && v831);
        let v835: bool = (v829 && (!v831));
        let v837: f64 = (v1 + (v673 - v830));
        let v843: f64 = (self.scalar_v323 * (v395 * v828));
        let v845: f64 = (v1 + (self.scalar_v105 * v843));
        let v850: f64 = (if v829 { ((self.scalar_v203 + (self.scalar_v841 * ((v845) as f64).ln())) - (if v835 { (v830 + ((v837) as f64).ln()) } else { (if v832 { v673 } else { v4 }) })) } else { v4 });
        let v853: f64 = (if v829 { self.scalar_v852 } else { v4 });
        let v855: f64 = (if v829 { (v853 * v853) } else { v392 });
        let v858: bool = (v850 < v4);
        let v859: bool = (v829 && v858);
        let v860: f64 = (v395 * v855);
        let v862: f64 = (((v855 + (if v829 { (v850 * v850) } else { self.scalar_v393 }))) as f64).sqrt();
        let v863: f64 = (v862 - v850);
        let v867: bool = (v829 && (!v858));
        let v870: f64 = (if v867 { (v395 * (v850 + v862)) } else { (if v859 { (v860 / v863) } else { v4 }) });
        let v874: f64 = (v870 + self.scalar_v873);
        let v875: f64 = (v870 * v874);
        let v878: f64 = (self.scalar_v872 * (v870 + self.scalar_v876));
        let v880: f64 = (if v829 { (v875 / v878) } else { v4 });
        let v882: f64 = (if v829 { (v828 / v880) } else { v4 });
        let v886: f64 = (if v829 { ((v882 - v1) / self.scalar_v884) } else { self.scalar_v365 });
        let v887: bool = (v882 < v1);
        let v888: bool = (v829 && v887);
        let v889: f64 = ((v886) as f64).exp();
        let v890: f64 = (v1 + v889);
        let v896: bool = (v829 && (!v887));
        let v898: f64 = (((-v886)) as f64).exp();
        let v899: f64 = (v1 + v898);
        let v912: f64 = (if v829 { ((if v896 { (v882 + (self.scalar_v884 * ((v899) as f64).ln())) } else { (if v888 { (v1 + (self.scalar_v884 * ((v890) as f64).ln())) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v914: f64 = (if v829 { (v870 / self.scalar_v873) } else { v4 });
        let v915: f64 = (v407 * v912);
        let v916: f64 = (v914 * v915);
        let v917: f64 = (v1 + v914);
        let v920: f64 = (((v1 + (v916 * v917))) as f64).sqrt();
        let v921: f64 = (v1 + v920);
        let v922: f64 = (v31 * v912);
        let v923: f64 = (v917 * v922);
        let v925: f64 = (if v829 { (v921 / v923) } else { v4 });
        let v927: f64 = (v820 * v925);
        let v928: f64 = ((v1 - v925) + v927);
        let v929: f64 = (v1 + v927);
        let v931: f64 = (if v829 { (v928 / v929) } else { v4 });
        let v934: f64 = (if v829 { (self.scalar_v105 * (v843 * v931)) } else { v4 });
        let v937: f64 = (v1 + (v820 + v934));
        let v940: f64 = (if v829 { ((v31 * v934) + (v820 * v937)) } else { v4 });
        let v943: f64 = (if v829 { (v395 * (v934 - v1)) } else { v4 });
        let v946: f64 = (if v829 { (v940 + (v943 * v943)) } else { v4 });
        let v947: bool = (v934 >= v1);
        let v948: bool = (v829 && v947);
        let v949: f64 = ((v946) as f64).sqrt();
        let v953: bool = (v829 && (!v947));
        let v954: f64 = (v949 - v943);
        let v956: f64 = (if v953 { (v940 / v954) } else { (if v948 { (v943 + v949) } else { v4 }) });
        let v959: bool = (v829 && (v956 < self.scalar_v957));
        let v960: f64 = (if v959 { self.scalar_v957 } else { v956 });
        let v961: f64 = (v1 + v960);
        let v970: f64 = (if v829 { (self.scalar_v967 * (v828 - self.scalar_v871)) } else { v4 });
        let v977: f64 = ((((if v829 { (v828 * self.scalar_v972) } else { v4 }) + (v970 * v970))) as f64).sqrt();
        let v986: bool = (v829 && self.scalar_v985);
        let v987: f64 = (v31 * v828);
        let v988: f64 = (v828 + v880);
        let v993: f64 = (v828 * self.scalar_v871);
        let v994: f64 = (v828 + self.scalar_v871);
        let v999: bool = (!v829);
        let v1000: f64 = (v31 * v797);
        let v1003: f64 = (if v999 { (if v717 { (v719 * (v1 + (v712 - self.scalar_v713))) } else { (if v714 { v715 } else { v4 }) }) } else { (if v829 { ((v960 * v961) * self.scalar_v964) } else { v4 }) });
        let v1014: bool = ((((v686) as f64).abs() < self.scalar_v1006) || (((v826) as f64).abs() < (self.scalar_v1010 * (v811 + v814))));
        let v1015: bool = (v999 && v1014);
        let v1016: f64 = (v820 + (if v999 { (v1000 / v822) } else { v960 }));
        let v1018: f64 = (if v1015 { (v395 * v1016) } else { v4 });
        let v1019: f64 = (v1 + v1018);
        let v1023: bool = (v999 && (!v1014));
        let v1025: f64 = ((v676 + v826) - v673);
        let v1027: f64 = (if v1023 { (v826 / v1025) } else { (if v1015 { (v1018 / v1019) } else { v931 }) });
        let v1029: f64 = (if v999 { self.scalar_v983 } else { (if v986 { (self.scalar_v244 * (v46 + (v987 / v988))) } else { (if (v829 && self.scalar_v981) { self.scalar_v983 } else { v4 }) }) });
        let v1030: f64 = (if v999 { v828 } else { (if v829 { (v993 / v994) } else { v4 }) });
        let v1033: f64 = (if v999 { (v1 - (v1030 / self.scalar_v871)) } else { (if v829 { (self.scalar_v871 / v994) } else { v4 }) });
        let v1040: f64 = ((v679 - self.scalar_v1037) / self.scalar_v1038);
        let v1041: bool = (v679 < self.scalar_v1037);
        let v1042: f64 = ((v1040) as f64).exp();
        let v1043: f64 = (v1 + v1042);
        let v1048: bool = (!v1041);
        let v1050: f64 = (((-v1040)) as f64).exp();
        let v1051: f64 = (v1 + v1050);
        let v1055: f64 = (if v1048 { (self.scalar_v1037 - (self.scalar_v1038 * ((v1051) as f64).ln())) } else { (if v1041 { (v679 - (self.scalar_v1038 * ((v1043) as f64).ln())) } else { v4 }) });
        let v1057: f64 = (v1 - (self.scalar_v268 * v1055));
        let v1059: f64 = f64::powf(v1057, self.scalar_v1058);
        let v1065: f64 = ((self.scalar_v1060 * (v1 - v1059)) + (v154 * (v679 - v1055)));
        let v1076: f64 = (if self.scalar_v1075 { v676 } else { (if self.scalar_v1071 { (v673 + (if v999 { v686 } else { (if v829 { (v970 + v977) } else { v4 }) })) } else { (if self.scalar_v1067 { v673 } else { v4 }) }) });
        let v1084: f64 = (v1076 - self.scalar_v1083);
        let v1085: f64 = (v1084 / v1029);
        let v1086: bool = (v1076 < self.scalar_v1083);
        let v1087: f64 = ((v1085) as f64).exp();
        let v1088: f64 = (v1 + v1087);
        let v1089: f64 = ((v1088) as f64).ln();
        let v1093: bool = (!v1086);
        let v1095: f64 = (((-v1085)) as f64).exp();
        let v1096: f64 = (v1 + v1095);
        let v1097: f64 = ((v1096) as f64).ln();
        let v1100: f64 = (if v1093 { (self.scalar_v1083 - (v1029 * v1097)) } else { (if v1086 { (v1076 - (v1029 * v1089)) } else { v4 }) });
        let v1102: f64 = f64::powf(v1033, self.scalar_v1101);
        let v1106: f64 = (v1 - (v1100 / self.scalar_v244));
        let v1107: f64 = f64::powf(v1106, self.scalar_v1103);
        let v1111: f64 = (self.scalar_v1079 * v1102);
        let v1112: f64 = (v1076 - v1100);
        let v1117: f64 = ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - (v1102 * v1107))) + (v1111 * v1112))) + (self.scalar_v285 * v673));
        let v1120: f64 = (v734 * self.scalar_v1119);
        let v1122: f64 = (((v1 + v1120)) as f64).sqrt();
        let v1123: f64 = (v1 + v1122);
        let v1124: f64 = (v1120 / v1123);
        let v1126: f64 = f64::powf(v1003, self.scalar_v1125);
        let v1127: f64 = (self.scalar_v1119 * v1126);
        let v1129: f64 = (((v1 + v1127)) as f64).sqrt();
        let v1130: f64 = (v1 + v1129);
        let v1131: f64 = (v1127 / v1130);
        let v1134: f64 = (v1 + (v1065 / self.scalar_v594));
        let v1135: f64 = (v1117 / self.scalar_v591);
        let v1136: f64 = (v1134 + v1135);
        let v1147: f64 = (((if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v1134)) } else { v4 })) as f64).exp();
        let v1148: f64 = (((if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v1117) / self.scalar_v591))) } else { v4 })) as f64).exp();
        let v1154: f64 = (if self.scalar_v1138 { ((v1147 - v1148) / self.scalar_v1152) } else { (if self.scalar_v1132 { v1136 } else { v4 }) });
        let v1155: f64 = 0.010000000000000002;
        let v1156: f64 = (v1154 * v1154);
        let v1157: bool = (v1154 < v4);
        let v1158: f64 = 0.005000000000000001;
        let v1160: f64 = (((v1155 + v1156)) as f64).sqrt();
        let v1161: f64 = (v1160 - v1154);
        let v1164: bool = (!v1157);
        let v1167: f64 = (if v1164 { (v395 * (v1154 + v1160)) } else { (if v1157 { (v1158 / v1161) } else { v4 }) });
        let v1170: f64 = (v1 + (v395 * (v1124 + v1131)));
        let v1171: f64 = (v1167 * v1170);
        let v1174: f64 = (v1126 * self.scalar_v1173);
        let v1175: f64 = (self.scalar_v420 * v734);
        let v1176: f64 = (v1175 - v1174);
        let v1177: f64 = (v1176 / v1171);
        let v1178: f64 = 0.0001;
        let v1179: f64 = (v679 / v1178);
        let v1180: bool = (v679 < v4);
        let v1181: f64 = ((v1179) as f64).exp();
        let v1182: f64 = (v1 + v1181);
        let v1186: bool = (!v1180);
        let v1188: f64 = (((-v1179)) as f64).exp();
        let v1189: f64 = (v1 + v1188);
        let v1193: f64 = (if v1186 { (v679 + (v1178 * ((v1189) as f64).ln())) } else { (if v1180 { (v1178 * ((v1182) as f64).ln()) } else { v4 }) });
        let v1195: f64 = (v1193 / self.scalar_v1194);
        let v1196: bool = (v1195 < self.scalar_v713);
        let v1197: f64 = ((v1195) as f64).exp();
        let v1199: bool = (!v1196);
        let v1200: f64 = (if v1199 { self.scalar_v718 } else { v804 });
        let v1204: f64 = (if v1199 { (v1200 * (v1 + (v1195 - self.scalar_v713))) } else { (if v1196 { v1197 } else { v4 }) });
        let v1209: f64 = ((v679 - self.scalar_v1207) / v30);
        let v1210: bool = (v679 < self.scalar_v1207);
        let v1211: f64 = ((v1209) as f64).exp();
        let v1212: f64 = (v1 + v1211);
        let v1217: bool = (!v1210);
        let v1219: f64 = (((-v1209)) as f64).exp();
        let v1220: f64 = (v1 + v1219);
        let v1224: f64 = (if v1217 { (self.scalar_v1207 - (v30 * ((v1220) as f64).ln())) } else { (if v1210 { (v679 - (v30 * ((v1212) as f64).ln())) } else { v4 }) });
        let v1226: f64 = (v1224 * self.scalar_v1225);
        let v1227: f64 = (self.scalar_v1207 - v1224);
        let v1228: f64 = f64::powf(v1227, v31);
        let v1230: f64 = (v724 / self.scalar_v462);
        let v1231: bool = (v1230 < self.scalar_v713);
        let v1232: f64 = ((v1230) as f64).exp();
        let v1234: bool = (!v1231);
        let v1235: f64 = (if v1234 { self.scalar_v718 } else { v1200 });
        let v1239: f64 = (if v1234 { (v1235 * (v1 + (v1230 - self.scalar_v713))) } else { (if v1231 { v1232 } else { v1193 }) });
        let v1241: f64 = (self.scalar_v105 * (v679 - self.scalar_v267));
        let v1242: bool = (v1241 < self.scalar_v713);
        let v1243: bool = (self.scalar_v481 && v1242);
        let v1244: f64 = ((v1241) as f64).exp();
        let v1247: bool = (self.scalar_v481 && (!v1242));
        let v1248: f64 = (if v1247 { self.scalar_v718 } else { v1235 });
        let v1252: f64 = (if v1247 { (v1248 * (v1 + (v1241 - self.scalar_v713))) } else { (if v1243 { v1244 } else { v1195 }) });
        let v1255: f64 = ((v1177 / self.scalar_v420) - 1000.0);
        let v1256: f64 = 40.0;
        let v1257: bool = (v1255 < v1256);
        let v1258: bool = (self.scalar_v481 && v1257);
        let v1259: f64 = ((v1255) as f64).exp();
        let v1262: bool = (self.scalar_v481 && (!v1257));
        let v1264: f64 = (if v1262 { 2.3538526683702e17 } else { v1248 });
        let v1268: f64 = (if v1262 { (v1264 * (v1 + (v1255 - v1256))) } else { (if v1258 { v1259 } else { v1204 }) });
        let v1269: f64 = (v1239 - v1);
        let v1270: f64 = (self.scalar_v471 * v1269);
        let v1272: f64 = (v1269 * self.scalar_v1271);
        let v1275: f64 = (((v1 + (v407 * v1252))) as f64).sqrt();
        let v1276: f64 = (v1 + v1275);
        let v1277: f64 = (v1272 / v1276);
        let v1278: f64 = (v1 + v1135);
        let v1282: f64 = (self.scalar_v496 * (v1003 - v1));
        let v1283: f64 = (v1268 * v1282);
        let v1284: f64 = (v1 + v1268);
        let v1299: f64 = (self.scalar_v1288 * ((v1003 + v1239) - v31));
        let v1303: f64 = (if self.scalar_v1294 { (self.scalar_v471 * ((v1269 * self.scalar_v1295) + (v1278 * v1299))) } else { (if self.scalar_v1291 { v1270 } else { (if self.scalar_v481 { ((v1270 + (v1277 * v1278)) + (v1283 / v1284)) } else { v4 }) }) });
        let v1304: f64 = (self.scalar_v105 * v682);
        let v1305: f64 = (v1304 / self.scalar_v473);
        let v1306: bool = (v1305 < self.scalar_v713);
        let v1307: f64 = ((v1305) as f64).exp();
        let v1309: bool = (!v1306);
        let v1310: f64 = (if v1309 { self.scalar_v718 } else { v1264 });
        let v1314: f64 = (if v1309 { (v1310 * (v1 + (v1305 - self.scalar_v713))) } else { (if v1306 { v1307 } else { v1239 }) });
        let v1316: f64 = (self.scalar_v105 * (v682 - self.scalar_v267));
        let v1317: bool = (v1316 < self.scalar_v713);
        let v1318: bool = (self.scalar_v481 && v1317);
        let v1319: f64 = ((v1316) as f64).exp();
        let v1322: bool = (self.scalar_v481 && (!v1317));
        let v1323: f64 = (if v1322 { self.scalar_v718 } else { v1310 });
        let v1328: f64 = (v1314 - v1);
        let v1329: f64 = (self.scalar_v479 * v1328);
        let v1331: f64 = (v1328 * self.scalar_v1330);
        let v1334: f64 = (((v1 + (v407 * (if v1322 { (v1323 * (v1 + (v1316 - self.scalar_v713))) } else { (if v1318 { v1319 } else { v1252 }) })))) as f64).sqrt();
        let v1335: f64 = (v1 + v1334);
        let v1340: f64 = (v724 / self.scalar_v434);
        let v1341: bool = (v1340 < self.scalar_v713);
        let v1342: f64 = ((v1340) as f64).exp();
        let v1344: bool = (!v1341);
        let v1345: f64 = (if v1344 { self.scalar_v718 } else { v1323 });
        let v1349: f64 = (if v1344 { (v1345 * (v1 + (v1340 - self.scalar_v713))) } else { (if v1341 { v1342 } else { v1314 }) });
        let v1352: f64 = (v1304 / self.scalar_v517);
        let v1353: bool = (v1352 < self.scalar_v713);
        let v1354: f64 = ((v1352) as f64).exp();
        let v1356: bool = (!v1353);
        let v1357: f64 = (if v1356 { self.scalar_v718 } else { v1345 });
        let v1361: f64 = (if v1356 { (v1357 * (v1 + (v1352 - self.scalar_v713))) } else { (if v1353 { v1354 } else { v1349 }) });
        let v1364: f64 = (v735 / self.scalar_v447);
        let v1365: bool = (v1364 < self.scalar_v713);
        let v1366: f64 = ((v1364) as f64).exp();
        let v1368: bool = (!v1365);
        let v1369: f64 = (if v1368 { self.scalar_v718 } else { v1357 });
        let v1373: f64 = (if v1368 { (v1369 * (v1 + (v1364 - self.scalar_v713))) } else { (if v1365 { v1366 } else { v1361 }) });
        let v1375: f64 = (self.scalar_v457 * (v1373 - v1));
        let v1376: f64 = (v1304 / self.scalar_v527);
        let v1377: bool = (v1376 < self.scalar_v713);
        let v1378: f64 = ((v1376) as f64).exp();
        let v1380: bool = (!v1377);
        let v1381: f64 = (if v1380 { self.scalar_v718 } else { v1369 });
        let v1385: f64 = (if v1380 { (v1381 * (v1 + (v1376 - self.scalar_v713))) } else { (if v1377 { v1378 } else { v1373 }) });
        let v1391: bool = (v1180 && self.scalar_v1390);
        let v1392: f64 = (v31 * v1059);
        let v1395: f64 = (self.scalar_v554 * (v1 - (self.scalar_v34 / v1392)));
        let v1396: bool = (v1395 < self.scalar_v713);
        let v1397: bool = (v1391 && v1396);
        let v1398: f64 = ((v1395) as f64).exp();
        let v1401: bool = (v1391 && (!v1396));
        let v1402: f64 = (if v1401 { self.scalar_v718 } else { v1381 });
        let v1406: f64 = (if v1401 { (v1402 * (v1 + (v1395 - self.scalar_v713))) } else { (if v1397 { v1398 } else { v4 }) });
        let v1408: f64 = (if v1391 { (self.scalar_v268 * v679) } else { self.scalar_v588 });
        let v1410: f64 = 1e-30;
        let v1412: f64 = ((((v1408 * v1408) + v1410)) as f64).sqrt();
        let v1415: f64 = f64::powf(v1412, self.scalar_v1414);
        let v1423: f64 = (v433 * v1408);
        let v1424: f64 = (v1408 * v1423);
        let v1425: f64 = (v1408 + self.scalar_v1419);
        let v1427: f64 = ((self.scalar_v32 * (self.scalar_v1417 - ((v154 * v1408) * self.scalar_v1419))) - (v1424 * v1425));
        let v1429: f64 = 0.16666666666666666;
        let v1433: f64 = (self.scalar_v554 * (self.scalar_v34 * v679));
        let v1434: f64 = (self.scalar_v131 * (if v1391 { ((v1415 * v1427) * v1429) } else { v4 }));
        let v1436: f64 = (if v1391 { (v1433 / v1434) } else { v1408 });
        let v1437: f64 = -0.001;
        let v1438: bool = (v1436 < v1437);
        let v1439: bool = (v1436 < self.scalar_v713);
        let v1440: bool = (v1391 && v1438);
        let v1441: bool = (v1439 && v1440);
        let v1442: f64 = ((v1436) as f64).exp();
        let v1445: bool = (v1440 && (!v1439));
        let v1446: f64 = (if v1445 { self.scalar_v718 } else { v1402 });
        let v1451: f64 = (-v679);
        let v1452: f64 = (v1 - (if v1445 { (v1446 * (v1 + (v1436 - self.scalar_v713))) } else { (if v1441 { v1442 } else { v4 }) }));
        let v1454: f64 = (v1 + (v1452 / v1436));
        let v1458: bool = (v1391 && (!v1438));
        let v1459: f64 = (v395 * v679);
        let v1460: f64 = (v1436 * v1459);
        let v1461: f64 = 0.3333333333333333;
        let v1462: f64 = (v1436 * v1461);
        let v1463: f64 = 0.25;
        let v1465: f64 = (v1 + (v1436 * v1463));
        let v1467: f64 = (v1 + (v1462 * v1465));
        let v1471: f64 = ((if v1458 { (v1460 * v1467) } else { (if v1440 { (v1451 * v1454) } else { v4 }) }) * self.scalar_v1470);
        let v1472: f64 = (v1059 * v1471);
        let v1477: bool = (!v1391);
        let v1483: bool = (self.scalar_v1481 && (v673 < v4));
        let v1484: f64 = (self.scalar_v269 * v673);
        let v1485: f64 = (v1 - v1484);
        let v1487: f64 = (if v1483 { f64::powf(v1485, self.scalar_v1103) } else { v4 });
        let v1488: f64 = (v31 * v1487);
        let v1491: f64 = (self.scalar_v576 * (v1 - (self.scalar_v69 / v1488)));
        let v1492: bool = (v1491 < self.scalar_v713);
        let v1493: bool = (v1483 && v1492);
        let v1494: f64 = ((v1491) as f64).exp();
        let v1497: bool = (v1483 && (!v1492));
        let v1498: f64 = (if v1497 { self.scalar_v718 } else { v1446 });
        let v1502: f64 = (if v1497 { (v1498 * (v1 + (v1491 - self.scalar_v713))) } else { (if v1493 { v1494 } else { v4 }) });
        let v1503: f64 = (if v1483 { v1484 } else { self.scalar_v566 });
        let v1506: f64 = (((v1410 + (v1503 * v1503))) as f64).sqrt();
        let v1508: f64 = f64::powf(v1506, self.scalar_v1507);
        let v1516: f64 = (v433 * v1503);
        let v1517: f64 = (v1503 * v1516);
        let v1518: f64 = (v1503 + self.scalar_v1512);
        let v1520: f64 = ((self.scalar_v67 * (self.scalar_v1510 - ((v154 * v1503) * self.scalar_v1512))) - (v1517 * v1518));
        let v1525: f64 = (self.scalar_v576 * (self.scalar_v69 * v673));
        let v1526: f64 = (self.scalar_v153 * (if v1483 { (v1429 * (v1508 * v1520)) } else { v4 }));
        let v1528: f64 = (if v1483 { (v1525 / v1526) } else { v1503 });
        let v1529: bool = (v1528 < v1437);
        let v1530: bool = (v1528 < self.scalar_v713);
        let v1531: bool = (v1483 && v1529);
        let v1532: bool = (v1530 && v1531);
        let v1533: f64 = ((v1528) as f64).exp();
        let v1536: bool = (v1531 && (!v1530));
        let v1537: f64 = (if v1536 { self.scalar_v718 } else { v1498 });
        let v1542: f64 = (-v673);
        let v1543: f64 = (v1 - (if v1536 { (v1537 * (v1 + (v1528 - self.scalar_v713))) } else { (if v1532 { v1533 } else { v4 }) }));
        let v1545: f64 = (v1 + (v1543 / v1528));
        let v1549: bool = (v1483 && (!v1529));
        let v1550: f64 = (v395 * v673);
        let v1551: f64 = (v1528 * v1550);
        let v1552: f64 = (v1461 * v1528);
        let v1554: f64 = (v1 + (v1463 * v1528));
        let v1556: f64 = (v1 + (v1552 * v1554));
        let v1560: f64 = ((if v1549 { (v1551 * v1556) } else { (if v1531 { (v1542 * v1545) } else { v4 }) }) * self.scalar_v1559);
        let v1561: f64 = (v1487 * v1560);
        let v1566: bool = (!v1483);
        let v1567: f64 = (if v1566 { v4 } else { (if v1483 { (self.scalar_v70 * (self.scalar_v269 * (v1502 * v1561))) } else { v4 }) });
        let v1568: f64 = (v744 * self.scalar_v1119);
        let v1569: f64 = (v407 * (if v781 { (v782 * (v1 + (v777 - self.scalar_v713))) } else { (if v778 { v779 } else { v4 }) }));
        let v1570: f64 = (v1568 - self.scalar_v1119);
        let v1572: f64 = (((v1 + v1568)) as f64).sqrt();
        let v1573: f64 = (v1 + v1572);
        let v1576: f64 = (((v1 + v1569)) as f64).sqrt();
        let v1577: f64 = (v1 + v1576);
        let v1581: f64 = (self.scalar_v1579 * (v744 - v1));
        let v1586: f64 = (((v1 + (v744 * self.scalar_v1583))) as f64).sqrt();
        let v1587: f64 = (v1 + v1586);
        let v1588: f64 = (v1581 / v1587);
        let v1594: f64 = (if self.scalar_v1592 { (self.scalar_v14 * v1588) } else { v1588 });
        let v1598: f64 = (self.scalar_v1596 * (v764 - v1));
        let v1601: f64 = (((v1 + (v764 * self.scalar_v1583))) as f64).sqrt();
        let v1602: f64 = (v1 + v1601);
        let v1604: f64 = (if self.scalar_v1592 { (v1598 / v1602) } else { v4 });
        let v1616: f64 = (if self.scalar_v1606 { (v711 - self.scalar_v1614) } else { v4 });
        let v1620: f64 = (if self.scalar_v1606 { (v1616 * v1616) } else { v1156 });
        let v1621: bool = (v1616 < v4);
        let v1622: bool = (self.scalar_v1606 && v1621);
        let v1625: f64 = (((self.scalar_v1618 + v1620)) as f64).sqrt();
        let v1626: f64 = (v1625 - v1616);
        let v1630: bool = (self.scalar_v1606 && (!v1621));
        let v1633: f64 = (if v1630 { (v395 * (v1616 + v1625)) } else { (if v1622 { (self.scalar_v1623 / v1626) } else { v4 }) });
        let v1636: f64 = (v1633 + (self.scalar_v1609 + (self.scalar_v311 * v1604)));
        let v1641: f64 = (if self.scalar_v1640 { v1 } else { (if self.scalar_v1606 { (v1633 / v1636) } else { v1 }) });
        let v1643: f64 = (if self.scalar_v1592 { (v1604 * v1641) } else { v4 });
        let v1647: f64 = (if self.scalar_v1645 { (v673 + v684) } else { v4 });
        let v1649: f64 = (-v1647);
        let v1652: bool = (v1649 < v4);
        let v1653: bool = (self.scalar_v1645 && v1652);
        let v1656: f64 = (((self.scalar_v1648 + (if self.scalar_v1645 { (v1647 * v1647) } else { v1620 }))) as f64).sqrt();
        let v1657: f64 = (v1656 - v1649);
        let v1661: bool = (self.scalar_v1645 && (!v1652));
        let v1664: f64 = (if v1661 { (v395 * (v1649 + v1656)) } else { (if v1653 { (self.scalar_v1654 / v1657) } else { v4 }) });
        let v1680: bool = (v1664 < self.scalar_v1672);
        let v1681: bool = (self.scalar_v1645 && v1680);
        let v1682: f64 = (v1664 / self.scalar_v1670);
        let v1684: f64 = (v1 - f64::powf(v1682, self.scalar_v1665));
        let v1688: bool = (self.scalar_v1645 && (!v1680));
        let v1694: f64 = (if self.scalar_v1693 { v1 } else { (if v1688 { (self.scalar_v1669 + (self.scalar_v1679 * (v1664 - self.scalar_v1672))) } else { (if v1681 { (v1 / v1684) } else { v4 }) }) });
        let v1700: bool = (v1136 < v4);
        let v1702: f64 = (((v1155 + (v1136 * v1136))) as f64).sqrt();
        let v1703: f64 = (v1702 - v1136);
        let v1706: bool = (!v1700);
        let v1709: f64 = (if v1706 { (v395 * (v1136 + v1702)) } else { (if v1700 { (v1158 / v1703) } else { v4 }) });
        let v1710: f64 = (v1170 * v1709);
        let v1711: f64 = (self.scalar_v299 / v1710);
        let v1712: bool = (v1711 < self.scalar_v28);
        let v1714: f64 = (v154 * (if v1712 { self.scalar_v28 } else { v1711 }));
        let v1717: f64 = (v684 + (self.scalar_v841 * ((if v749 { (v750 * (v1 + (v745 - self.scalar_v713))) } else { (if v746 { v747 } else { v4 }) }) - v1)));
        let v1719: bool = (v1177 > v4);
        let v1723: bool = (v673 < self.scalar_v1722);
        let v1726: f64 = ((-v1177) / self.scalar_v1725);
        let v1727: bool = (v1726 < self.scalar_v713);
        let v1729: bool = (v1723 && (v1719 && self.scalar_v1721));
        let v1730: bool = (v1727 && v1729);
        let v1731: f64 = ((v1726) as f64).exp();
        let v1734: bool = (v1729 && (!v1727));
        let v1735: f64 = (if v1734 { self.scalar_v718 } else { v1537 });
        let v1739: f64 = (if v1734 { (v1735 * (v1 + (v1726 - self.scalar_v713))) } else { (if v1730 { v1731 } else { v4 }) });
        let v1740: f64 = (self.scalar_v1722 - v673);
        let v1742: f64 = (if v1729 { (v1739 * v1740) } else { v4 });
        let v1746: f64 = (self.scalar_v1743 * f64::powf(v1742, self.scalar_v1744));
        let v1747: bool = (v1746 < self.scalar_v713);
        let v1748: bool = (v1729 && v1747);
        let v1749: f64 = ((v1746) as f64).exp();
        let v1752: bool = (v1729 && (!v1747));
        let v1753: f64 = (if v1752 { self.scalar_v718 } else { v1735 });
        let v1757: f64 = (if v1752 { (v1753 * (v1 + (v1746 - self.scalar_v713))) } else { (if v1748 { v1749 } else { v4 }) });
        let v1760: f64 = (v1742 * self.scalar_v1759);
        let v1766: bool = (v1719 && self.scalar_v1765);
        let v1768: bool = ((v673 < self.scalar_v203) && (self.scalar_v1763 && v1766));
        let v1774: f64 = (if v1768 { self.scalar_v1773 } else { v4 });
        let v1775: f64 = (self.scalar_v203 - v673);
        let v1777: f64 = (if v1768 { (v1775 / v1033) } else { v946 });
        let v1780: f64 = ((((v31 * v1777) / v1774)) as f64).sqrt();
        let v1781: f64 = (if v1768 { v1780 } else { v4 });
        let v1784: bool = (v1768 && self.scalar_v1783);
        let v1787: bool = (v1768 && self.scalar_v1786);
        let v1790: f64 = (if v1787 { (v1 - (v395 * v1027)) } else { v4 });
        let v1791: f64 = (self.scalar_v1771 * v1790);
        let v1793: f64 = (if v1787 { (v1790 * v1791) } else { (if v1784 { self.scalar_v1771 } else { v4 }) });
        let v1794: f64 = (v1781 * v1793);
        let v1798: f64 = ((((v1781 * v1781) + (v1793 * v1793))) as f64).sqrt();
        let v1800: f64 = (if v1768 { (v1794 / v1798) } else { v4 });
        let v1802: f64 = (if v1768 { (v1775 / v1800) } else { v4 });
        let v1803: f64 = (v395 * v1800);
        let v1804: f64 = (v1774 * v1803);
        let v1807: f64 = (if v1768 { (v1802 + (v1033 * v1804)) } else { v4 });
        let v1820: f64 = (self.scalar_v871 * (if v1787 { (v1 + (self.scalar_v1810 * (v1 + (v31 * v1027)))) } else { v4 }));
        let v1822: f64 = ((if v1787 { self.scalar_v1818 } else { v4 }) - (v1177 / v1820));
        let v1825: f64 = (if v1787 { (v1802 - (v1804 * v1822)) } else { v4 });
        let v1826: f64 = (v1825 - v1807);
        let v1828: f64 = (v46 * v1802);
        let v1829: f64 = (v1802 * v1828);
        let v1835: f64 = (((if v1787 { ((v1826 * v1826) + ((v1030 * v1829) / self.scalar_v871)) } else { v1777 })) as f64).sqrt();
        let v1838: f64 = (if v1787 { (v395 * ((v1807 + v1825) + v1835)) } else { (if v1784 { v1807 } else { v4 }) });
        let v1839: f64 = (v1838 - v1802);
        let v1841: f64 = (if v1768 { (v1839 / v1838) } else { v4 });
        let v1844: bool = (((v1841) as f64).abs() > 1e-7);
        let v1845: bool = (v1768 && v1844);
        let v1847: f64 = (if v1845 { (v1803 / v1841) } else { v4 });
        let v1849: f64 = (v1838 * self.scalar_v1848);
        let v1850: f64 = (v1847 * v1849);
        let v1852: f64 = (self.scalar_v1851 / v1838);
        let v1853: f64 = ((v1852) as f64).exp();
        let v1855: f64 = (v1 + (v1793 / v1847));
        let v1857: f64 = (((v1852 * v1855)) as f64).exp();
        let v1858: f64 = (v1853 - v1857);
        let v1862: bool = (v1768 && (!v1844));
        let v1863: f64 = (self.scalar_v10 * v1793);
        let v1870: bool = (v1723 && (self.scalar_v1866 && (v1766 && self.scalar_v1867)));
        let v1871: f64 = f64::powf(v1740, self.scalar_v1744);
        let v1873: f64 = (v1177 + self.scalar_v1872);
        let v1875: f64 = (v1 - (v1177 / v1873));
        let v1877: f64 = f64::powf(v1875, self.scalar_v1876);
        let v1879: f64 = (if v1870 { (v1871 * v1877) } else { v4 });
        let v1880: bool = (self.scalar_v1783 && v1870);
        let v1882: bool = (self.scalar_v1786 && v1870);
        let v1886: f64 = (if v1882 { ((v1177 - self.scalar_v1883) / self.scalar_v1872) } else { v4 });
        let v1890: f64 = (if v1882 { ((v1886 - v1) / self.scalar_v1888) } else { v1209 });
        let v1891: bool = (v1886 < v1);
        let v1892: bool = (v1882 && v1891);
        let v1893: f64 = ((v1890) as f64).exp();
        let v1894: f64 = (v1 + v1893);
        let v1900: bool = (v1882 && (!v1891));
        let v1902: f64 = (((-v1890)) as f64).exp();
        let v1903: f64 = (v1 + v1902);
        let v1907: f64 = (if v1900 { (v1886 + (self.scalar_v1888 * ((v1903) as f64).ln())) } else { (if v1892 { (v1 + (self.scalar_v1888 * ((v1894) as f64).ln())) } else { v4 }) });
        let v1909: f64 = f64::powf(v1907, self.scalar_v1908);
        let v1912: f64 = (self.scalar_v1743 * (if v1882 { (v1879 * v1909) } else { (if v1880 { v1879 } else { v4 }) }));
        let v1913: bool = (v1912 < self.scalar_v713);
        let v1914: bool = (v1870 && v1913);
        let v1915: f64 = ((v1912) as f64).exp();
        let v1918: bool = (v1870 && (!v1913));
        let v1919: f64 = (if v1918 { self.scalar_v718 } else { v1753 });
        let v1923: f64 = (if v1918 { (v1919 * (v1 + (v1912 - self.scalar_v713))) } else { (if v1914 { v1915 } else { v1757 }) });
        let v1924: f64 = (v1740 * self.scalar_v1759);
        let v1926: f64 = (if v1870 { (v1923 * v1924) } else { (if v1862 { (v1853 * v1863) } else { (if v1845 { (v1850 * v1858) } else { (if v1729 { (v1757 * v1760) } else { v4 }) }) }) });
        let v1930: bool = (v1719 && (v1926 > v4));
        let v1931: bool = (self.scalar_v1929 && v1930);
        let v1932: f64 = (self.scalar_v306 + v1714);
        let v1933: f64 = (v1177 * v1932);
        let v1940: f64 = (if v1931 { (((self.scalar_v103 / v1933) + (self.scalar_v471 * (v1171 / self.scalar_v420))) + (self.scalar_v292 / v1932)) } else { v4 });
        let v1941: bool = (self.scalar_v1866 && v1931);
        let v1944: f64 = (if v1941 { ((v1926 - v1940) / v392) } else { v1890 });
        let v1945: bool = (v1926 < v1940);
        let v1946: bool = (v1941 && v1945);
        let v1947: f64 = ((v1944) as f64).exp();
        let v1948: f64 = (v1 + v1947);
        let v1954: bool = (v1941 && (!v1945));
        let v1956: f64 = (((-v1944)) as f64).exp();
        let v1957: f64 = (v1 + v1956);
        let v1961: f64 = (if v1954 { (v1940 - (v392 * ((v1957) as f64).ln())) } else { (if v1946 { (v1926 - (v392 * ((v1948) as f64).ln())) } else { v1926 }) });
        let v1962: f64 = (v1177 * v1961);
        let v1965: bool = (v1931 && self.scalar_v1964);
        let v1966: f64 = (v1940 * v1962);
        let v1967: f64 = (v1940 + v1961);
        let v1971: bool = (v1930 && self.scalar_v1970);
        let v1972: f64 = (if v1971 { v1962 } else { (if v1965 { (v1966 / v1967) } else { (if v1941 { v1962 } else { v4 }) }) });
        let v1978: f64 = ((v682 - self.scalar_v1037) / self.scalar_v1038);
        let v1979: bool = (v682 < self.scalar_v1037);
        let v1980: f64 = ((v1978) as f64).exp();
        let v1981: f64 = (v1 + v1980);
        let v1986: bool = (!v1979);
        let v1988: f64 = (((-v1978)) as f64).exp();
        let v1989: f64 = (v1 + v1988);
        let v1993: f64 = (if v1986 { (self.scalar_v1037 - (self.scalar_v1038 * ((v1989) as f64).ln())) } else { (if v1979 { (v682 - (self.scalar_v1038 * ((v1981) as f64).ln())) } else { v4 }) });
        let v1996: f64 = (v1 - (self.scalar_v268 * v1993));
        let v2009: f64 = (v1124 * self.scalar_v2008);
        let v2010: f64 = (v1709 * v2009);
        let v2011: f64 = (v1131 * self.scalar_v2008);
        let v2012: f64 = (v1709 * v2011);
        let v2014: f64 = ((v706 - self.scalar_v1083) / self.scalar_v983);
        let v2015: bool = (v706 < self.scalar_v1083);
        let v2016: f64 = ((v2014) as f64).exp();
        let v2017: f64 = (v1 + v2016);
        let v2022: bool = (!v2015);
        let v2024: f64 = (((-v2014)) as f64).exp();
        let v2025: f64 = (v1 + v2024);
        let v2029: f64 = (if v2022 { (self.scalar_v1083 - (self.scalar_v983 * ((v2025) as f64).ln())) } else { (if v2015 { (v706 - (self.scalar_v983 * ((v2017) as f64).ln())) } else { v4 }) });
        let v2031: f64 = (v1 - (v2029 / self.scalar_v244));
        let v2044: f64 = (self.scalar_v14 * ((self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - f64::powf(v2031, self.scalar_v1103))) + (self.scalar_v1079 * (v706 - v2029)))) + (self.scalar_v285 * v706))) * self.scalar_v2042));
        let v2046: f64 = ((v711 - self.scalar_v1083) / self.scalar_v983);
        let v2047: bool = (v711 < self.scalar_v1083);
        let v2048: f64 = ((v2046) as f64).exp();
        let v2049: f64 = (v1 + v2048);
        let v2054: bool = (!v2047);
        let v2056: f64 = (((-v2046)) as f64).exp();
        let v2057: f64 = (v1 + v2056);
        let v2061: f64 = (if v2054 { (self.scalar_v1083 - (self.scalar_v983 * ((v2057) as f64).ln())) } else { (if v2047 { (v711 - (self.scalar_v983 * ((v2049) as f64).ln())) } else { v4 }) });
        let v2063: f64 = (v1 - (v2061 / self.scalar_v244));
        let v2075: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - f64::powf(v2063, self.scalar_v1103))) + (self.scalar_v1079 * (v711 - v2061)))) + (self.scalar_v285 * v711)))));
        let v2083: f64 = (v679 / self.scalar_v2082);
        let v2084: bool = (v2083 < self.scalar_v713);
        let v2085: f64 = ((v2083) as f64).exp();
        let v2087: bool = (!v2084);
        let v2088: f64 = (if v2087 { self.scalar_v718 } else { v1919 });
        let v2093: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * (v1 + (v2083 - self.scalar_v713))) } else { (if v2084 { v2085 } else { v1385 }) }));
        let v2098: f64 = (v1027 * self.scalar_v2097);
        let v2099: f64 = (v31 + v1016);
        let v2113: f64 = (self.scalar_v105 * ((v706 - self.scalar_v224) / self.scalar_v2111));
        let v2114: bool = (v2113 < self.scalar_v713);
        let v2116: bool = (v2114 && self.scalar_v2115);
        let v2117: f64 = ((v2113) as f64).exp();
        let v2120: bool = (self.scalar_v2115 && (!v2114));
        let v2121: f64 = (if v2120 { self.scalar_v718 } else { v2088 });
        let v2127: f64 = (v744 * self.scalar_v2126);
        let v2130: f64 = (((v1 + (v407 * (if v2120 { (v2121 * (v1 + (v2113 - self.scalar_v713))) } else { (if v2116 { v2117 } else { v4 }) })))) as f64).sqrt();
        let v2131: f64 = (v1 + v2130);
        let v2133: f64 = (if self.scalar_v2115 { (v2127 / v2131) } else { (if self.scalar_v2102 { ((self.scalar_v2103 * (((v1570 / v1573) * self.scalar_v2007) + ((v1569 / v1577) * self.scalar_v2096))) / self.scalar_v617) } else { v4 }) });
        let v2141: f64 = (if self.scalar_v2139 { (v764 * self.scalar_v1119) } else { v4 });
        let v2142: f64 = (v2141 - self.scalar_v1119);
        let v2144: f64 = (((v1 + v2141)) as f64).sqrt();
        let v2145: f64 = (v1 + v2144);
        let v2149: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * (v1 + (v766 - self.scalar_v713))) } else { (if v767 { v768 } else { v4 }) })) } else { v4 });
        let v2151: f64 = (((v1 + v2149)) as f64).sqrt();
        let v2152: f64 = (v1 + v2151);
        let v2164: f64 = (self.scalar_v105 * (v711 - self.scalar_v224));
        let v2165: bool = (v2164 < self.scalar_v713);
        let v2167: bool = (v2165 && self.scalar_v2166);
        let v2168: f64 = ((v2164) as f64).exp();
        let v2171: bool = (self.scalar_v2166 && (!v2165));
        let v2172: f64 = (if v2171 { self.scalar_v718 } else { v2121 });
        let v2178: f64 = (v764 * self.scalar_v2177);
        let v2181: f64 = (((v1 + (v407 * (if v2171 { (v2172 * (v1 + (v2164 - self.scalar_v713))) } else { (if v2167 { v2168 } else { v4 }) })))) as f64).sqrt();
        let v2182: f64 = (v1 + v2181);
        let v2184: f64 = (if self.scalar_v2166 { (v2178 / v2182) } else { (if self.scalar_v2139 { ((self.scalar_v2156 * ((self.scalar_v2007 * (if self.scalar_v2139 { (v2142 / v2145) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (v2149 / v2152) } else { v4 })))) / self.scalar_v617) } else { v4 }) });
        let v2192: f64 = (if self.scalar_v2188 { (f64::powf(v1057, self.scalar_v2189) - v154) } else { v4 });
        let v2193: f64 = (if self.scalar_v2188 { v1040 } else { v4 });
        let v2194: bool = (v2193 < v4);
        let v2195: bool = (self.scalar_v2188 && v2194);
        let v2196: f64 = ((v2193) as f64).exp();
        let v2197: f64 = (v1 + v2196);
        let v2201: bool = (self.scalar_v2188 && (!v2194));
        let v2203: f64 = (((-v2193)) as f64).exp();
        let v2204: f64 = (v1 + v2203);
        let v2206: f64 = (if v2201 { (v2203 / v2204) } else { (if v2195 { (v1 / v2197) } else { v4 }) });
        let v2213: f64 = ((self.scalar_v105 * v1120) / self.scalar_v355);
        let v2214: f64 = (v395 / v1122);
        let v2216: f64 = (if self.scalar_v2188 { (v2213 * v2214) } else { v4 });
        let v2217: f64 = (v1709 * self.scalar_v2008);
        let v2222: f64 = (v684 * 0.2);
        let v2224: f64 = ((if self.scalar_v2188 { (v2093 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { (v154 + (v2192 * v2206)) } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { (v2216 * v2217) } else { v4 })));
        let v2233: f64 = (if self.scalar_v2188 { (v2010 + (v2093 * self.scalar_v2227)) } else { v4 });
        let v2242: f64 = (if self.scalar_v2241 { v2010 } else { (if self.scalar_v2188 { (v2233 * self.scalar_v2238) } else { v4 }) });
        let v2243: f64 = (if self.scalar_v2241 { v2012 } else { (if self.scalar_v2188 { (v2012 + (v2233 * self.scalar_v2234)) } else { v4 }) });
        let v2245: f64 = (v1174 + v1175);
        let v2246: f64 = (v2245 / v1171);
        let v2253: f64 = (if self.scalar_v2252 { v4 } else { (if self.scalar_v2248 { (((v1972 / v2246)) as f64).abs() } else { v4 }) });
        let v2254: bool = (v2246 > v4);
        let v2255: f64 = (v2242 + v2243);
        let v2258: bool = (!v2254);
        let v2259: f64 = (self.scalar_v610 * v1709);
        let v2261: f64 = (if v2258 { (v1171 * v2259) } else { (if v2254 { (v2255 / v2246) } else { v4 }) });
        let v2274: f64 = (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (v2261 * self.scalar_v2269) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v2261) } else { v4 }) }) });
        let v2291: f64 = (self.scalar_v27 * (self.scalar_v0 * v828));
        let v2293: f64 = (self.scalar_v27 * (self.scalar_v0 * v1177));
        let v2294: f64 = (self.scalar_v0 * ((self.scalar_v534 * (v1385 - v1)) + ((if self.scalar_v1290 { v1329 } else { (if self.scalar_v481 { (v1329 + (v1331 / v1335)) } else { v4 }) }) + (self.scalar_v525 * (v1361 - v1)))));
        let v2295: f64 = (self.scalar_v27 * v2294);
        let v2297: f64 = (((v1303 + (self.scalar_v445 * (v1349 - v1))) + (v4 * v679)) - (if v1477 { v4 } else { (if v1391 { (self.scalar_v35 * (self.scalar_v268 * (v1406 * v1472))) } else { v4 }) }));
        let v2301: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v541 * (v1204 - v1)) + ((v1226 * v1228) + v2297))));
        let v2304: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v1694))));
        let v2305: f64 = (if self.scalar_v481 { v2304 } else { v4 });
        let v2306: f64 = (if self.scalar_v1290 { v2304 } else { v4 });
        let v2308: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1717 / v1714)));
        let v2310: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v1972)));
        let v2313: f64 = (self.scalar_v27 * ((self.scalar_v0 * (self.scalar_v0 * (v687 - v677))) / self.scalar_v292));
        let v2316: f64 = (self.scalar_v27 * ((self.scalar_v0 * v692) / self.scalar_v306));
        let v2318: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, (self.scalar_v0 * ((if self.scalar_v2241 { v2093 } else { (if self.scalar_v2188 { (v2093 * self.scalar_v2228) } else { v4 }) }) + ((v1065 * self.scalar_v1975) + v2242))));
        let v2319: f64 = (self.scalar_v27 * v2318);
        let v2321: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, (self.scalar_v0 * (self.scalar_v1994 * ((self.scalar_v1060 * (v1 - f64::powf(v1996, self.scalar_v1058))) + (v154 * (v682 - v1993))))));
        let v2322: f64 = (self.scalar_v27 * v2321);
        let v2324: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, (self.scalar_v0 * ((v2098 * v2099) + ((v1117 * self.scalar_v2005) + v2243))));
        let v2325: f64 = (self.scalar_v27 * v2324);
        let v2327: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * v2224) } else { v4 })));
        let v2328: f64 = (self.scalar_v27 * v2327);
        let v2331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, ((self.scalar_v0 * (v690 - v687)) * self.scalar_v2329));
        let v2332: f64 = (self.scalar_v27 * v2331);
        let v2335: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, (v697 * self.scalar_v2333));
        let v2336: f64 = (self.scalar_v27 * v2335);
        let v2338: f64 = (self.scalar_v27 * (self.scalar_v0 * (v1643 * v1694)));
        let v2341: f64 = (self.scalar_v27 * (self.scalar_v653 * (self.scalar_v0 * v710)));
        let v2343: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, (self.scalar_v0 * (v2075 + (if self.scalar_v2136 { (v1641 * v2184) } else { v4 }))));
        let v2344: f64 = (self.scalar_v27 * v2343);
        let v2347: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1594 * v1694) + ((v1375 * v1694) + (v4 * v706)))));
        let v2349: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, (self.scalar_v0 * (v2044 + (if self.scalar_v2136 { (self.scalar_v14 * v2133) } else { v2133 }))));
        let v2350: f64 = (self.scalar_v27 * v2349);
        let v2354: f64 = (if self.scalar_v654 { (self.scalar_v27 * (self.scalar_v661 * (self.scalar_v0 * v703))) } else { v4 });
        let v2358: f64 = (if self.scalar_v662 { (self.scalar_v27 * (self.scalar_v669 * (self.scalar_v0 * v700))) } else { v4 });
        let v2359: f64 = ctx.node_voltage(nodes[10]);
        let v2360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, v2359);
        let v2361: f64 = (v2274 * v2360);
        let v2362: f64 = (v2253 * v2359);
        let v2385: f64 = (if v729 { (v730 * self.scalar_v2377) } else { (if v726 { (v727 * self.scalar_v2377) } else { v4 }) });
        let v2386: f64 = (if v729 { (v730 * self.scalar_v2378) } else { (if v726 { (v727 * self.scalar_v2378) } else { v4 }) });
        let v2401: f64 = (if v739 { (v740 * self.scalar_v2367) } else { (if v736 { (v737 * self.scalar_v2367) } else { v4 }) });
        let v2402: f64 = (if v739 { (v740 * self.scalar_v2387) } else { (if v736 { (v737 * self.scalar_v2387) } else { v4 }) });
        let v2403: f64 = (if v739 { (v740 * self.scalar_v2388) } else { (if v736 { (v737 * self.scalar_v2388) } else { v4 }) });
        let v2404: f64 = (if v739 { (v740 * self.scalar_v2368) } else { (if v736 { (v737 * self.scalar_v2368) } else { v4 }) });
        let v2426: f64 = (if v759 { (v760 * self.scalar_v2387) } else { (if v756 { (v757 * self.scalar_v2387) } else { v4 }) });
        let v2427: f64 = (if v759 { (v760 * self.scalar_v2413) } else { (if v756 { (v757 * self.scalar_v2413) } else { v4 }) });
        let v2428: f64 = (if v759 { (v760 * self.scalar_v2388) } else { (if v756 { (v757 * self.scalar_v2388) } else { v4 }) });
        let v2429: f64 = (if v759 { (v760 * self.scalar_v2368) } else { (if v756 { (v757 * self.scalar_v2368) } else { v4 }) });
        let v2468: f64 = (if v792 { (v793 * self.scalar_v2367) } else { (if v789 { (v790 * self.scalar_v2367) } else { v4 }) });
        let v2469: f64 = (if v792 { (v793 * self.scalar_v2368) } else { (if v789 { (v790 * self.scalar_v2368) } else { v4 }) });
        let v2476: f64 = (if v803 { (v804 * self.scalar_v2367) } else { (if v800 { (v801 * self.scalar_v2367) } else { v4 }) });
        let v2477: f64 = (if v803 { (v804 * self.scalar_v2368) } else { (if v800 { (v801 * self.scalar_v2368) } else { v4 }) });
        let v2480: f64 = (v31 * v811);
        let v2481: f64 = ((v407 * v2468) / v2480);
        let v2482: f64 = ((v407 * v2469) / v2480);
        let v2485: f64 = (v31 * v814);
        let v2486: f64 = ((v407 * v2476) / v2485);
        let v2487: f64 = ((v407 * v2477) / v2485);
        let v2493: f64 = (v816 * v816);
        let v2499: f64 = (if v819 { v4 } else { (((v816 * (v31 * v2476)) - (v815 * v2486)) / v2493) });
        let v2500: f64 = (if v819 { v4 } else { (((v816 * (v31 * v2477)) - (v815 * v2487)) / v2493) });
        let v2517: f64 = (self.scalar_v103 * ((v2481 - v2486) - ((((v816 * v2481) - (v822 * v2486)) / v2493) / v823)));
        let v2518: f64 = (self.scalar_v103 * ((-v2487) - (((-(v822 * v2487)) / v2493) / v823)));
        let v2519: f64 = (self.scalar_v103 * (v2482 - ((v2482 / v816) / v823)));
        let v2521: f64 = (self.scalar_v2363 + v2519);
        let v2522: f64 = (v2517 / self.scalar_v323);
        let v2523: f64 = ((self.scalar_v0 + v2518) / self.scalar_v323);
        let v2524: f64 = (v2521 / self.scalar_v323);
        let v2534: f64 = (self.scalar_v323 * (v395 * v2522));
        let v2535: f64 = (self.scalar_v323 * (v395 * v2523));
        let v2536: f64 = (self.scalar_v323 * (v395 * v2524));
        let v2548: f64 = (if v829 { ((self.scalar_v841 * ((self.scalar_v105 * v2534) / v845)) - (if v835 { (self.scalar_v0 / v837) } else { (if v832 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v2549: f64 = (if v829 { ((self.scalar_v841 * ((self.scalar_v105 * v2535) / v845)) - (if v835 { (self.scalar_v2363 / v837) } else { (if v832 { self.scalar_v2363 } else { v4 }) })) } else { v4 });
        let v2550: f64 = (if v829 { (self.scalar_v841 * ((self.scalar_v105 * v2536) / v845)) } else { v4 });
        let v2551: f64 = (v850 * v2548);
        let v2553: f64 = (v850 * v2549);
        let v2555: f64 = (v850 * v2550);
        let v2560: f64 = (v31 * v862);
        let v2561: f64 = ((if v829 { (v2551 + v2551) } else { v4 }) / v2560);
        let v2562: f64 = ((if v829 { (v2553 + v2553) } else { v4 }) / v2560);
        let v2563: f64 = ((if v829 { (v2555 + v2555) } else { v4 }) / v2560);
        let v2569: f64 = (v863 * v863);
        let v2586: f64 = (if v867 { (v395 * (v2548 + v2561)) } else { (if v859 { ((-(v860 * (v2561 - v2548))) / v2569) } else { v4 }) });
        let v2587: f64 = (if v867 { (v395 * (v2549 + v2562)) } else { (if v859 { ((-(v860 * (v2562 - v2549))) / v2569) } else { v4 }) });
        let v2588: f64 = (if v867 { (v395 * (v2550 + v2563)) } else { (if v859 { ((-(v860 * (v2563 - v2550))) / v2569) } else { v4 }) });
        let v2604: f64 = (v878 * v878);
        let v2614: f64 = (if v829 { (((v878 * ((v874 * v2586) + (v870 * v2586))) - (v875 * (self.scalar_v872 * v2586))) / v2604) } else { v4 });
        let v2615: f64 = (if v829 { (((v878 * ((v874 * v2587) + (v870 * v2587))) - (v875 * (self.scalar_v872 * v2587))) / v2604) } else { v4 });
        let v2616: f64 = (if v829 { (((v878 * ((v874 * v2588) + (v870 * v2588))) - (v875 * (self.scalar_v872 * v2588))) / v2604) } else { v4 });
        let v2620: f64 = (v880 * v880);
        let v2630: f64 = (if v829 { (((v880 * v2522) - (v828 * v2614)) / v2620) } else { v4 });
        let v2631: f64 = (if v829 { (((v880 * v2523) - (v828 * v2615)) / v2620) } else { v4 });
        let v2632: f64 = (if v829 { (((v880 * v2524) - (v828 * v2616)) / v2620) } else { v4 });
        let v2636: f64 = (if v829 { (v2630 / self.scalar_v884) } else { v4 });
        let v2637: f64 = (if v829 { (v2631 / self.scalar_v884) } else { v4 });
        let v2638: f64 = (if v829 { (v2632 / self.scalar_v884) } else { v4 });
        let v2672: f64 = (if v829 { ((if v896 { (v2630 + (self.scalar_v884 * ((v898 * (-v2636)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2636) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2673: f64 = (if v829 { ((if v896 { (v2631 + (self.scalar_v884 * ((v898 * (-v2637)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2637) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2674: f64 = (if v829 { ((if v896 { (v2632 + (self.scalar_v884 * ((v898 * (-v2638)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2638) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2678: f64 = (if v829 { (v2586 / self.scalar_v873) } else { v4 });
        let v2679: f64 = (if v829 { (v2587 / self.scalar_v873) } else { v4 });
        let v2680: f64 = (if v829 { (v2588 / self.scalar_v873) } else { v4 });
        let v2702: f64 = (v31 * v920);
        let v2720: f64 = ((v923 * (((v917 * ((v915 * v2678) + (v914 * (v407 * v2672)))) + (v916 * v2678)) / v2702)) - (v921 * ((v922 * v2678) + (v917 * (v31 * v2672)))));
        let v2721: f64 = (v923 * v923);
        let v2725: f64 = ((v923 * (((v917 * ((v915 * v2679) + (v914 * (v407 * v2673)))) + (v916 * v2679)) / v2702)) - (v921 * ((v922 * v2679) + (v917 * (v31 * v2673)))));
        let v2729: f64 = ((v923 * (((v917 * ((v915 * v2680) + (v914 * (v407 * v2674)))) + (v916 * v2680)) / v2702)) - (v921 * ((v922 * v2680) + (v917 * (v31 * v2674)))));
        let v2731: f64 = (if v829 { (v2720 / v2721) } else { v4 });
        let v2732: f64 = (if v829 { (v2725 / v2721) } else { v4 });
        let v2733: f64 = (if v829 { (v2729 / v2721) } else { v4 });
        let v2739: f64 = ((v925 * v2499) + (v820 * v2731));
        let v2742: f64 = ((v925 * v2500) + (v820 * v2732));
        let v2743: f64 = (v820 * v2733);
        let v2750: f64 = (v929 * v929);
        let v2760: f64 = (if v829 { (((v929 * ((-v2731) + v2739)) - (v928 * v2739)) / v2750) } else { v4 });
        let v2761: f64 = (if v829 { (((v929 * ((-v2732) + v2742)) - (v928 * v2742)) / v2750) } else { v4 });
        let v2762: f64 = (if v829 { (((v929 * ((-v2733) + v2743)) - (v928 * v2743)) / v2750) } else { v4 });
        let v2775: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2534) + (v843 * v2760))) } else { v4 });
        let v2776: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2535) + (v843 * v2761))) } else { v4 });
        let v2777: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2536) + (v843 * v2762))) } else { v4 });
        let v2793: f64 = (if v829 { ((v31 * v2775) + ((v937 * v2499) + (v820 * (v2499 + v2775)))) } else { v4 });
        let v2794: f64 = (if v829 { ((v31 * v2776) + ((v937 * v2500) + (v820 * (v2500 + v2776)))) } else { v4 });
        let v2795: f64 = (if v829 { ((v31 * v2777) + (v820 * v2777)) } else { v4 });
        let v2799: f64 = (if v829 { (v395 * v2775) } else { v4 });
        let v2800: f64 = (if v829 { (v395 * v2776) } else { v4 });
        let v2801: f64 = (if v829 { (v395 * v2777) } else { v4 });
        let v2802: f64 = (v943 * v2799);
        let v2804: f64 = (v943 * v2800);
        let v2806: f64 = (v943 * v2801);
        let v2811: f64 = (if v829 { (v2793 + (v2802 + v2802)) } else { v4 });
        let v2812: f64 = (if v829 { (v2794 + (v2804 + v2804)) } else { v4 });
        let v2813: f64 = (if v829 { (v2795 + (v2806 + v2806)) } else { v4 });
        let v2814: f64 = (v31 * v949);
        let v2815: f64 = (v2811 / v2814);
        let v2816: f64 = (v2812 / v2814);
        let v2817: f64 = (v2813 / v2814);
        let v2830: f64 = (v954 * v954);
        let v2843: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2793) - (v940 * (v2815 - v2799))) / v2830) } else { (if v948 { (v2799 + v2815) } else { v4 }) }) });
        let v2844: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2794) - (v940 * (v2816 - v2800))) / v2830) } else { (if v948 { (v2800 + v2816) } else { v4 }) }) });
        let v2845: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2795) - (v940 * (v2817 - v2801))) / v2830) } else { (if v948 { (v2801 + v2817) } else { v4 }) }) });
        let v2864: f64 = (if v829 { (self.scalar_v967 * v2522) } else { v4 });
        let v2865: f64 = (if v829 { (self.scalar_v967 * v2523) } else { v4 });
        let v2866: f64 = (if v829 { (self.scalar_v967 * v2524) } else { v4 });
        let v2873: f64 = (v970 * v2864);
        let v2875: f64 = (v970 * v2865);
        let v2877: f64 = (v970 * v2866);
        let v2882: f64 = (v31 * v977);
        let v2901: f64 = (v988 * v988);
        let v2917: f64 = (self.scalar_v871 * v2522);
        let v2918: f64 = (self.scalar_v871 * v2523);
        let v2919: f64 = (self.scalar_v871 * v2524);
        let v2923: f64 = (v994 * v994);
        let v2950: f64 = (v822 * v822);
        let v2958: f64 = (if v999 { (((v822 * (v31 * v2469)) - (v1000 * v2482)) / v2950) } else { v2845 });
        let v2959: f64 = (if v999 { (if v717 { (v719 * self.scalar_v2367) } else { (if v714 { (v715 * self.scalar_v2367) } else { v4 }) }) } else { (if v829 { (self.scalar_v964 * ((v961 * v2843) + (v960 * v2843))) } else { v4 }) });
        let v2960: f64 = (if v999 { v4 } else { (if v829 { (self.scalar_v964 * ((v961 * v2844) + (v960 * v2844))) } else { v4 }) });
        let v2961: f64 = (if v999 { (if v717 { (v719 * self.scalar_v2368) } else { (if v714 { (v715 * self.scalar_v2368) } else { v4 }) }) } else { (if v829 { (self.scalar_v964 * ((v961 * v2845) + (v960 * v2845))) } else { v4 }) });
        let v2962: f64 = (v2499 + (if v999 { (((v822 * (v31 * v2468)) - (v1000 * v2481)) / v2950) } else { v2843 }));
        let v2963: f64 = (v2500 + (if v999 { v4 } else { v2844 }));
        let v2967: f64 = (if v1015 { (v395 * v2962) } else { v4 });
        let v2968: f64 = (if v1015 { (v395 * v2963) } else { v4 });
        let v2969: f64 = (if v1015 { (v395 * v2958) } else { v4 });
        let v2973: f64 = (v1019 * v1019);
        let v2992: f64 = (v1025 * v1025);
        let v3002: f64 = (if v1023 { (((v1025 * v2517) - (v826 * ((self.scalar_v0 + v2517) - self.scalar_v0))) / v2992) } else { (if v1015 { (((v1019 * v2967) - (v1018 * v2967)) / v2973) } else { v2760 }) });
        let v3003: f64 = (if v1023 { (((v1025 * v2518) - (v826 * (v2518 - self.scalar_v2363))) / v2992) } else { (if v1015 { (((v1019 * v2968) - (v1018 * v2968)) / v2973) } else { v2761 }) });
        let v3004: f64 = (if v1023 { (((v1025 * v2519) - (v826 * v2521)) / v2992) } else { (if v1015 { (((v1019 * v2969) - (v1018 * v2969)) / v2973) } else { v2762 }) });
        let v3008: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2522)) - (v987 * (v2522 + v2614))) / v2901)) } else { v4 }) });
        let v3009: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2523)) - (v987 * (v2523 + v2615))) / v2901)) } else { v4 }) });
        let v3010: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2524)) - (v987 * (v2524 + v2616))) / v2901)) } else { v4 }) });
        let v3011: f64 = (if v999 { v2522 } else { (if v829 { (((v994 * v2917) - (v993 * v2522)) / v2923) } else { v4 }) });
        let v3012: f64 = (if v999 { v2523 } else { (if v829 { (((v994 * v2918) - (v993 * v2523)) / v2923) } else { v4 }) });
        let v3013: f64 = (if v999 { v2524 } else { (if v829 { (((v994 * v2919) - (v993 * v2524)) / v2923) } else { v4 }) });
        let v3020: f64 = (if v999 { (-(v3011 / self.scalar_v871)) } else { (if v829 { ((-v2917) / v2923) } else { v4 }) });
        let v3021: f64 = (if v999 { (-(v3012 / self.scalar_v871)) } else { (if v829 { ((-v2918) / v2923) } else { v4 }) });
        let v3022: f64 = (if v999 { (-(v3013 / self.scalar_v871)) } else { (if v829 { ((-v2919) / v2923) } else { v4 }) });
        let v3045: f64 = (if v1048 { (-(self.scalar_v1038 * ((v1050 * self.scalar_v3035) / v1051))) } else { (if v1041 { (self.scalar_v2363 - (self.scalar_v1038 * ((v1042 * self.scalar_v3023) / v1043))) } else { v4 }) });
        let v3046: f64 = (if v1048 { (-(self.scalar_v1038 * ((v1050 * self.scalar_v3036) / v1051))) } else { (if v1041 { (self.scalar_v0 - (self.scalar_v1038 * ((v1042 * self.scalar_v3024) / v1043))) } else { v4 }) });
        let v3049: f64 = (-(self.scalar_v268 * v3045));
        let v3050: f64 = (-(self.scalar_v268 * v3046));
        let v3053: f64 = (self.scalar_v1058 * f64::powf(v1057, self.scalar_v3051));
        let v3054: f64 = (v3049 * v3053);
        let v3055: f64 = (v3050 * v3053);
        let v3064: f64 = ((self.scalar_v1060 * (-v3054)) + (v154 * (self.scalar_v2363 - v3045)));
        let v3065: f64 = ((self.scalar_v1060 * (-v3055)) + (v154 * (self.scalar_v0 - v3046)));
        let v3070: f64 = (if self.scalar_v1071 { (self.scalar_v0 + (if v999 { v4 } else { (if v829 { (v2864 + (((if v829 { (self.scalar_v972 * v2522) } else { v4 }) + (v2873 + v2873)) / v2882)) } else { v4 }) })) } else { self.scalar_v3066 });
        let v3071: f64 = (if self.scalar_v1071 { (self.scalar_v2363 + (if v999 { self.scalar_v0 } else { (if v829 { (v2865 + (((if v829 { (self.scalar_v972 * v2523) } else { v4 }) + (v2875 + v2875)) / v2882)) } else { v4 }) })) } else { self.scalar_v3067 });
        let v3073: f64 = (if self.scalar_v1075 { self.scalar_v0 } else { v3070 });
        let v3074: f64 = (if self.scalar_v1075 { v4 } else { v3071 });
        let v3075: f64 = (if self.scalar_v1075 { self.scalar_v2363 } else { (if self.scalar_v1071 { (if v999 { self.scalar_v2363 } else { (if v829 { (v2866 + (((if v829 { (self.scalar_v972 * v2524) } else { v4 }) + (v2877 + v2877)) / v2882)) } else { v4 }) }) } else { v4 }) });
        let v3079: f64 = (v1029 * v1029);
        let v3080: f64 = (((v1029 * v3073) - (v1084 * v3008)) / v3079);
        let v3084: f64 = (((v1029 * v3074) - (v1084 * v3009)) / v3079);
        let v3088: f64 = (((v1029 * v3075) - (v1084 * v3010)) / v3079);
        let v3131: f64 = (if v1093 { (-((v1097 * v3008) + (v1029 * ((v1095 * (-v3080)) / v1096)))) } else { (if v1086 { (v3073 - ((v1089 * v3008) + (v1029 * ((v1087 * v3080) / v1088)))) } else { v4 }) });
        let v3132: f64 = (if v1093 { (-((v1097 * v3009) + (v1029 * ((v1095 * (-v3084)) / v1096)))) } else { (if v1086 { (v3074 - ((v1089 * v3009) + (v1029 * ((v1087 * v3084) / v1088)))) } else { v4 }) });
        let v3133: f64 = (if v1093 { (-((v1097 * v3010) + (v1029 * ((v1095 * (-v3088)) / v1096)))) } else { (if v1086 { (v3075 - ((v1089 * v3010) + (v1029 * ((v1087 * v3088) / v1088)))) } else { v4 }) });
        let v3136: f64 = (self.scalar_v1101 * f64::powf(v1033, self.scalar_v3134));
        let v3137: f64 = (v3020 * v3136);
        let v3138: f64 = (v3021 * v3136);
        let v3139: f64 = (v3022 * v3136);
        let v3148: f64 = (self.scalar_v1103 * f64::powf(v1106, self.scalar_v3146));
        let v3182: f64 = ((self.scalar_v1104 * (-((v1107 * v3137) + (v1102 * ((-(v3131 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3137)) + (v1111 * (v3073 - v3131))));
        let v3183: f64 = ((self.scalar_v1104 * (-((v1107 * v3138) + (v1102 * ((-(v3132 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3138)) + (v1111 * (v3074 - v3132))));
        let v3184: f64 = ((self.scalar_v1104 * (-((v1107 * v3139) + (v1102 * ((-(v3133 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3139)) + (v1111 * (v3075 - v3133))));
        let v3187: f64 = (self.scalar_v1078 * v3184);
        let v3190: f64 = ((self.scalar_v1078 * v3182) + self.scalar_v3188);
        let v3191: f64 = ((self.scalar_v1078 * v3183) + self.scalar_v3189);
        let v3192: f64 = (self.scalar_v1119 * v2385);
        let v3193: f64 = (self.scalar_v1119 * v2386);
        let v3194: f64 = (v31 * v1122);
        let v3195: f64 = (v3192 / v3194);
        let v3196: f64 = (v3193 / v3194);
        let v3200: f64 = (v1123 * v1123);
        let v3201: f64 = (((v1123 * v3192) - (v1120 * v3195)) / v3200);
        let v3205: f64 = (((v1123 * v3193) - (v1120 * v3196)) / v3200);
        let v3208: f64 = (self.scalar_v1125 * f64::powf(v1003, self.scalar_v3206));
        let v3209: f64 = (v2959 * v3208);
        let v3210: f64 = (v2960 * v3208);
        let v3211: f64 = (v2961 * v3208);
        let v3212: f64 = (self.scalar_v1119 * v3209);
        let v3213: f64 = (self.scalar_v1119 * v3210);
        let v3214: f64 = (self.scalar_v1119 * v3211);
        let v3215: f64 = (v31 * v1129);
        let v3222: f64 = (v1130 * v1130);
        let v3223: f64 = (((v1130 * v3212) - (v1127 * (v3212 / v3215))) / v3222);
        let v3227: f64 = (((v1130 * v3213) - (v1127 * (v3213 / v3215))) / v3222);
        let v3231: f64 = (((v1130 * v3214) - (v1127 * (v3214 / v3215))) / v3222);
        let v3232: f64 = (v3064 / self.scalar_v594);
        let v3233: f64 = (v3065 / self.scalar_v594);
        let v3234: f64 = (v3190 / self.scalar_v591);
        let v3235: f64 = (v3191 / self.scalar_v591);
        let v3236: f64 = (v3187 / self.scalar_v591);
        let v3237: f64 = (v3233 + v3234);
        let v3272: f64 = (((v1147 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v3233)) } else { v4 })) - (v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3190) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152);
        let v3275: f64 = (if self.scalar_v1138 { ((v1147 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v3232)) } else { v4 })) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3232 } else { v4 }) });
        let v3276: f64 = (if self.scalar_v1138 { v3272 } else { (if self.scalar_v1132 { v3237 } else { v4 }) });
        let v3277: f64 = (if self.scalar_v1138 { ((-(v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3191) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3235 } else { v4 }) });
        let v3278: f64 = (if self.scalar_v1138 { ((-(v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3187) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3236 } else { v4 }) });
        let v3279: f64 = (v1154 * v3275);
        let v3280: f64 = (v3279 + v3279);
        let v3281: f64 = (v1154 * v3276);
        let v3282: f64 = (v3281 + v3281);
        let v3283: f64 = (v1154 * v3277);
        let v3284: f64 = (v3283 + v3283);
        let v3285: f64 = (v1154 * v3278);
        let v3286: f64 = (v3285 + v3285);
        let v3287: f64 = (v31 * v1160);
        let v3288: f64 = (v3280 / v3287);
        let v3289: f64 = (v3282 / v3287);
        let v3290: f64 = (v3284 / v3287);
        let v3291: f64 = (v3286 / v3287);
        let v3298: f64 = (v1161 * v1161);
        let v3326: f64 = (v395 * v3201);
        let v3327: f64 = (v395 * (v3205 + v3223));
        let v3328: f64 = (v395 * v3227);
        let v3329: f64 = (v395 * v3231);
        let v3332: f64 = ((v1170 * (if v1164 { (v395 * (v3275 + v3288)) } else { (if v1157 { ((-(v1158 * (v3288 - v3275))) / v3298) } else { v4 }) })) + (v1167 * v3326));
        let v3335: f64 = ((v1170 * (if v1164 { (v395 * (v3276 + v3289)) } else { (if v1157 { ((-(v1158 * (v3289 - v3276))) / v3298) } else { v4 }) })) + (v1167 * v3327));
        let v3338: f64 = ((v1170 * (if v1164 { (v395 * (v3277 + v3290)) } else { (if v1157 { ((-(v1158 * (v3290 - v3277))) / v3298) } else { v4 }) })) + (v1167 * v3328));
        let v3341: f64 = ((v1170 * (if v1164 { (v395 * (v3278 + v3291)) } else { (if v1157 { ((-(v1158 * (v3291 - v3278))) / v3298) } else { v4 }) })) + (v1167 * v3329));
        let v3342: f64 = (self.scalar_v1173 * v3209);
        let v3343: f64 = (self.scalar_v1173 * v3210);
        let v3344: f64 = (self.scalar_v1173 * v3211);
        let v3346: f64 = (self.scalar_v420 * v2386);
        let v3350: f64 = (v1171 * (self.scalar_v420 * v2385));
        let v3353: f64 = (v1171 * v1171);
        let v3354: f64 = ((v3350 - (v1176 * v3332)) / v3353);
        let v3358: f64 = (((v1171 * (v3346 - v3342)) - (v1176 * v3335)) / v3353);
        let v3362: f64 = (((v1171 * (-v3343)) - (v1176 * v3338)) / v3353);
        let v3366: f64 = (((v1171 * (-v3344)) - (v1176 * v3341)) / v3353);
        let v3387: f64 = (if v1186 { (self.scalar_v2363 + (v1178 * ((v1188 * self.scalar_v3377) / v1189))) } else { (if v1180 { (v1178 * ((v1181 * self.scalar_v3367) / v1182)) } else { v4 }) });
        let v3388: f64 = (if v1186 { (self.scalar_v0 + (v1178 * ((v1188 * self.scalar_v3378) / v1189))) } else { (if v1180 { (v1178 * ((v1181 * self.scalar_v3368) / v1182)) } else { v4 }) });
        let v3389: f64 = (v3387 / self.scalar_v1194);
        let v3390: f64 = (v3388 / self.scalar_v1194);
        let v3397: f64 = (if v1199 { (v1200 * v3389) } else { (if v1196 { (v1197 * v3389) } else { v4 }) });
        let v3398: f64 = (if v1199 { (v1200 * v3390) } else { (if v1196 { (v1197 * v3390) } else { v4 }) });
        let v3423: f64 = (if v1217 { (-(v30 * ((v1219 * self.scalar_v3413) / v1220))) } else { (if v1210 { (self.scalar_v2363 - (v30 * ((v1211 * self.scalar_v3401) / v1212))) } else { v4 }) });
        let v3424: f64 = (if v1217 { (-(v30 * ((v1219 * self.scalar_v3414) / v1220))) } else { (if v1210 { (self.scalar_v0 - (v30 * ((v1211 * self.scalar_v3402) / v1212))) } else { v4 }) });
        let v3430: f64 = (v31 * f64::powf(v1227, v1));
        let v3447: f64 = (if v1234 { (v1235 * self.scalar_v3439) } else { (if v1231 { (v1232 * self.scalar_v3439) } else { v3387 }) });
        let v3448: f64 = (if v1234 { (v1235 * self.scalar_v3440) } else { (if v1231 { (v1232 * self.scalar_v3440) } else { v3388 }) });
        let v3455: f64 = (if v1247 { (v1248 * self.scalar_v2368) } else { (if v1243 { (v1244 * self.scalar_v2368) } else { v3389 }) });
        let v3456: f64 = (if v1247 { (v1248 * self.scalar_v2367) } else { (if v1243 { (v1244 * self.scalar_v2367) } else { v3390 }) });
        let v3457: f64 = (v3354 / self.scalar_v420);
        let v3458: f64 = (v3358 / self.scalar_v420);
        let v3459: f64 = (v3362 / self.scalar_v420);
        let v3460: f64 = (v3366 / self.scalar_v420);
        let v3473: f64 = (if v1262 { (v1264 * v3457) } else { (if v1258 { (v1259 * v3457) } else { v3397 }) });
        let v3474: f64 = (if v1262 { (v1264 * v3458) } else { (if v1258 { (v1259 * v3458) } else { v3398 }) });
        let v3475: f64 = (if v1262 { (v1264 * v3459) } else { (if v1258 { (v1259 * v3459) } else { v4 }) });
        let v3476: f64 = (if v1262 { (v1264 * v3460) } else { (if v1258 { (v1259 * v3460) } else { v4 }) });
        let v3477: f64 = (self.scalar_v471 * v3447);
        let v3478: f64 = (self.scalar_v471 * v3448);
        let v3483: f64 = (v31 * v1275);
        let v3489: f64 = (v1276 * v1276);
        let v3519: f64 = (v1284 * v1284);
        let v3533: f64 = ((v3477 + (v1278 * (((v1276 * (self.scalar_v1271 * v3447)) - (v1272 * ((v407 * v3455) / v3483))) / v3489))) + (((v1284 * (v1282 * v3473)) - (v1283 * v3473)) / v3519));
        let v3534: f64 = ((v3478 + ((v1278 * (((v1276 * (self.scalar_v1271 * v3448)) - (v1272 * ((v407 * v3456) / v3483))) / v3489)) + (v1277 * v3234))) + (((v1284 * ((v1282 * v3474) + (v1268 * (self.scalar_v496 * v2959)))) - (v1283 * v3474)) / v3519));
        let v3543: f64 = (if self.scalar_v1291 { v4 } else { (if self.scalar_v481 { ((v1277 * v3235) + (((v1284 * ((v1282 * v3475) + (v1268 * (self.scalar_v496 * v2960)))) - (v1283 * v3475)) / v3519)) } else { v4 }) });
        let v3544: f64 = (if self.scalar_v1291 { v4 } else { (if self.scalar_v481 { ((v1277 * v3236) + (((v1284 * ((v1282 * v3476) + (v1268 * (self.scalar_v496 * v2961)))) - (v1283 * v3476)) / v3519)) } else { v4 }) });
        let v3569: f64 = (if self.scalar_v1294 { (self.scalar_v471 * ((self.scalar_v1295 * v3448) + ((v1299 * v3234) + (v1278 * (self.scalar_v1288 * (v2959 + v3448)))))) } else { (if self.scalar_v1291 { v3478 } else { (if self.scalar_v481 { v3534 } else { v4 }) }) });
        let v3581: f64 = (if v1309 { (v1310 * self.scalar_v3572) } else { (if v1306 { (v1307 * self.scalar_v3572) } else { v3447 }) });
        let v3582: f64 = (if v1309 { (v1310 * self.scalar_v3573) } else { (if v1306 { (v1307 * self.scalar_v3573) } else { v4 }) });
        let v3583: f64 = (if v1309 { v4 } else { (if v1306 { v4 } else { v3448 }) });
        let v3594: f64 = (self.scalar_v479 * v3581);
        let v3595: f64 = (self.scalar_v479 * v3582);
        let v3596: f64 = (self.scalar_v479 * v3583);
        let v3603: f64 = (v31 * v1334);
        let v3610: f64 = (v1335 * v1335);
        let v3611: f64 = (((v1335 * (self.scalar_v1330 * v3581)) - (v1331 * ((v407 * (if v1322 { (v1323 * self.scalar_v2368) } else { (if v1318 { (v1319 * self.scalar_v2368) } else { v3455 }) })) / v3603))) / v3610);
        let v3615: f64 = (((v1335 * (self.scalar_v1330 * v3582)) - (v1331 * ((v407 * (if v1322 { (v1323 * self.scalar_v2367) } else { (if v1318 { (v1319 * self.scalar_v2367) } else { v4 }) })) / v3603))) / v3610);
        let v3625: f64 = (if self.scalar_v481 { (v3596 + (((v1335 * (self.scalar_v1330 * v3583)) - (v1331 * ((v407 * (if v1322 { v4 } else { (if v1318 { v4 } else { v3456 }) })) / v3603))) / v3610)) } else { v4 });
        let v3638: f64 = (if v1344 { (v1345 * self.scalar_v3629) } else { (if v1341 { (v1342 * self.scalar_v3629) } else { v3581 }) });
        let v3639: f64 = (if v1344 { v4 } else { (if v1341 { v4 } else { v3582 }) });
        let v3640: f64 = (if v1344 { (v1345 * self.scalar_v3630) } else { (if v1341 { (v1342 * self.scalar_v3630) } else { v3583 }) });
        let v3653: f64 = (if v1356 { (v1357 * self.scalar_v3644) } else { (if v1353 { (v1354 * self.scalar_v3644) } else { v3638 }) });
        let v3654: f64 = (if v1356 { (v1357 * self.scalar_v3645) } else { (if v1353 { (v1354 * self.scalar_v3645) } else { v3639 }) });
        let v3655: f64 = (if v1356 { v4 } else { (if v1353 { v4 } else { v3640 }) });
        let v3676: f64 = (if v1368 { v4 } else { (if v1365 { v4 } else { v3653 }) });
        let v3677: f64 = (if v1368 { (v1369 * self.scalar_v3659) } else { (if v1365 { (v1366 * self.scalar_v3659) } else { v3654 }) });
        let v3678: f64 = (if v1368 { (v1369 * self.scalar_v3660) } else { (if v1365 { (v1366 * self.scalar_v3660) } else { v3655 }) });
        let v3679: f64 = (if v1368 { (v1369 * self.scalar_v3661) } else { (if v1365 { (v1366 * self.scalar_v3661) } else { v4 }) });
        let v3680: f64 = (if v1368 { (v1369 * self.scalar_v3662) } else { (if v1365 { (v1366 * self.scalar_v3662) } else { v4 }) });
        let v3697: f64 = (if v1380 { (v1381 * self.scalar_v3686) } else { (if v1377 { (v1378 * self.scalar_v3686) } else { v3676 }) });
        let v3698: f64 = (if v1380 { (v1381 * self.scalar_v3687) } else { (if v1377 { (v1378 * self.scalar_v3687) } else { v3677 }) });
        let v3699: f64 = (if v1380 { v4 } else { (if v1377 { v4 } else { v3678 }) });
        let v3700: f64 = (if v1380 { v4 } else { (if v1377 { v4 } else { v3679 }) });
        let v3701: f64 = (if v1380 { v4 } else { (if v1377 { v4 } else { v3680 }) });
        let v3711: f64 = (v1392 * v1392);
        let v3718: f64 = (self.scalar_v554 * (-((-(self.scalar_v34 * (v31 * v3054))) / v3711)));
        let v3719: f64 = (self.scalar_v554 * (-((-(self.scalar_v34 * (v31 * v3055))) / v3711)));
        let v3730: f64 = (if v1391 { self.scalar_v3728 } else { v4 });
        let v3731: f64 = (if v1391 { self.scalar_v3729 } else { v4 });
        let v3732: f64 = (v1408 * v3730);
        let v3734: f64 = (v1408 * v3731);
        let v3736: f64 = (v31 * v1412);
        let v3741: f64 = (self.scalar_v1414 * f64::powf(v1412, self.scalar_v3739));
        let v3769: f64 = (v1415 * ((self.scalar_v32 * (-(self.scalar_v1419 * (v154 * v3730)))) - ((v1425 * ((v1423 * v3730) + (v1408 * (v433 * v3730)))) + (v1424 * v3730))));
        let v3772: f64 = (v1415 * ((self.scalar_v32 * (-(self.scalar_v1419 * (v154 * v3731)))) - ((v1425 * ((v1423 * v3731) + (v1408 * (v433 * v3731)))) + (v1424 * v3731))));
        let v3787: f64 = (v1434 * v1434);
        let v3788: f64 = (((v1434 * self.scalar_v3780) - (v1433 * (self.scalar_v131 * (if v1391 { (v1429 * ((v1427 * (((v3732 + v3732) / v3736) * v3741)) + v3769)) } else { v4 })))) / v3787);
        let v3792: f64 = (((v1434 * self.scalar_v3781) - (v1433 * (self.scalar_v131 * (if v1391 { (v1429 * ((v1427 * (((v3734 + v3734) / v3736) * v3741)) + v3772)) } else { v4 })))) / v3787);
        let v3793: f64 = (if v1391 { v3788 } else { v3730 });
        let v3794: f64 = (if v1391 { v3792 } else { v3731 });
        let v3808: f64 = (v1436 * v1436);
        let v3816: f64 = ((self.scalar_v0 * v1454) + (v1451 * (((v1436 * (-(if v1445 { (v1446 * v3793) } else { (if v1441 { (v1442 * v3793) } else { v4 }) }))) - (v1452 * v3793)) / v3808)));
        let v3819: f64 = ((v1454 * self.scalar_v2363) + (v1451 * (((v1436 * (-(if v1445 { (v1446 * v3794) } else { (if v1441 { (v1442 * v3794) } else { v4 }) }))) - (v1452 * v3794)) / v3808)));
        let v3846: f64 = (if v1458 { ((v1467 * ((v1459 * v3793) + (v1436 * self.scalar_v3822))) + (v1460 * ((v1465 * (v1461 * v3793)) + (v1462 * (v1463 * v3793))))) } else { (if v1440 { v3816 } else { v4 }) });
        let v3847: f64 = (if v1458 { ((v1467 * ((v1459 * v3794) + (v1436 * self.scalar_v3823))) + (v1460 * ((v1465 * (v1461 * v3794)) + (v1462 * (v1463 * v3794))))) } else { (if v1440 { v3819 } else { v4 }) });
        let v3858: f64 = ((v1472 * (if v1401 { (v1402 * v3718) } else { (if v1397 { (v1398 * v3718) } else { v4 }) })) + (v1406 * ((v1471 * v3054) + (v1059 * (self.scalar_v1470 * v3846)))));
        let v3861: f64 = ((v1472 * (if v1401 { (v1402 * v3719) } else { (if v1397 { (v1398 * v3719) } else { v4 }) })) + (v1406 * ((v1471 * v3055) + (v1059 * (self.scalar_v1470 * v3847)))));
        let v3875: f64 = (self.scalar_v1103 * f64::powf(v1485, self.scalar_v3146));
        let v3878: f64 = (if v1483 { (self.scalar_v3872 * v3875) } else { v4 });
        let v3879: f64 = (if v1483 { (self.scalar_v3873 * v3875) } else { v4 });
        let v3884: f64 = (v1488 * v1488);
        let v3891: f64 = (self.scalar_v576 * (-((-(self.scalar_v69 * (v31 * v3878))) / v3884)));
        let v3892: f64 = (self.scalar_v576 * (-((-(self.scalar_v69 * (v31 * v3879))) / v3884)));
        let v3901: f64 = (if v1483 { self.scalar_v3870 } else { v4 });
        let v3902: f64 = (if v1483 { self.scalar_v3871 } else { v4 });
        let v3903: f64 = (v1503 * v3901);
        let v3905: f64 = (v1503 * v3902);
        let v3907: f64 = (v31 * v1506);
        let v3912: f64 = (self.scalar_v1507 * f64::powf(v1506, self.scalar_v3910));
        let v3940: f64 = (v1508 * ((self.scalar_v67 * (-(self.scalar_v1512 * (v154 * v3901)))) - ((v1518 * ((v1516 * v3901) + (v1503 * (v433 * v3901)))) + (v1517 * v3901))));
        let v3943: f64 = (v1508 * ((self.scalar_v67 * (-(self.scalar_v1512 * (v154 * v3902)))) - ((v1518 * ((v1516 * v3902) + (v1503 * (v433 * v3902)))) + (v1517 * v3902))));
        let v3958: f64 = (v1526 * v1526);
        let v3959: f64 = (((v1526 * self.scalar_v3951) - (v1525 * (self.scalar_v153 * (if v1483 { (v1429 * ((v1520 * (((v3903 + v3903) / v3907) * v3912)) + v3940)) } else { v4 })))) / v3958);
        let v3963: f64 = (((v1526 * self.scalar_v3952) - (v1525 * (self.scalar_v153 * (if v1483 { (v1429 * ((v1520 * (((v3905 + v3905) / v3907) * v3912)) + v3943)) } else { v4 })))) / v3958);
        let v3964: f64 = (if v1483 { v3959 } else { v3901 });
        let v3965: f64 = (if v1483 { v3963 } else { v3902 });
        let v3979: f64 = (v1528 * v1528);
        let v3987: f64 = ((v1545 * self.scalar_v2363) + (v1542 * (((v1528 * (-(if v1536 { (v1537 * v3964) } else { (if v1532 { (v1533 * v3964) } else { v4 }) }))) - (v1543 * v3964)) / v3979)));
        let v3990: f64 = ((self.scalar_v0 * v1545) + (v1542 * (((v1528 * (-(if v1536 { (v1537 * v3965) } else { (if v1532 { (v1533 * v3965) } else { v4 }) }))) - (v1543 * v3965)) / v3979)));
        let v4015: f64 = (if v1549 { ((v1556 * ((v1550 * v3964) + (v1528 * self.scalar_v3823))) + (v1551 * ((v1554 * (v1461 * v3964)) + (v1552 * (v1463 * v3964))))) } else { (if v1531 { v3987 } else { v4 }) });
        let v4016: f64 = (if v1549 { ((v1556 * ((v1550 * v3965) + (v1528 * self.scalar_v3822))) + (v1551 * ((v1554 * (v1461 * v3965)) + (v1552 * (v1463 * v3965))))) } else { (if v1531 { v3990 } else { v4 }) });
        let v4027: f64 = ((v1561 * (if v1497 { (v1498 * v3891) } else { (if v1493 { (v1494 * v3891) } else { v4 }) })) + (v1502 * ((v1560 * v3878) + (v1487 * (self.scalar_v1559 * v4015)))));
        let v4030: f64 = ((v1561 * (if v1497 { (v1498 * v3892) } else { (if v1493 { (v1494 * v3892) } else { v4 }) })) + (v1502 * ((v1560 * v3879) + (v1487 * (self.scalar_v1559 * v4016)))));
        let v4039: f64 = (self.scalar_v1119 * v2401);
        let v4040: f64 = (self.scalar_v1119 * v2402);
        let v4041: f64 = (self.scalar_v1119 * v2403);
        let v4042: f64 = (self.scalar_v1119 * v2404);
        let v4043: f64 = (v407 * (if v781 { (v782 * self.scalar_v2367) } else { (if v778 { (v779 * self.scalar_v2367) } else { v4 }) }));
        let v4044: f64 = (v407 * (if v781 { (v782 * self.scalar_v2387) } else { (if v778 { (v779 * self.scalar_v2387) } else { v4 }) }));
        let v4045: f64 = (v407 * (if v781 { (v782 * self.scalar_v2388) } else { (if v778 { (v779 * self.scalar_v2388) } else { v4 }) }));
        let v4046: f64 = (v407 * (if v781 { (v782 * self.scalar_v2368) } else { (if v778 { (v779 * self.scalar_v2368) } else { v4 }) }));
        let v4047: f64 = (v31 * v1572);
        let v4055: f64 = (v1573 * v1573);
        let v4069: f64 = (v31 * v1576);
        let v4077: f64 = (v1577 * v1577);
        let v4099: f64 = (v31 * v1586);
        let v4107: f64 = (v1587 * v1587);
        let v4108: f64 = (((v1587 * (self.scalar_v1579 * v2401)) - (v1581 * ((self.scalar_v1583 * v2401) / v4099))) / v4107);
        let v4112: f64 = (((v1587 * (self.scalar_v1579 * v2402)) - (v1581 * ((self.scalar_v1583 * v2402) / v4099))) / v4107);
        let v4116: f64 = (((v1587 * (self.scalar_v1579 * v2403)) - (v1581 * ((self.scalar_v1583 * v2403) / v4099))) / v4107);
        let v4120: f64 = (((v1587 * (self.scalar_v1579 * v2404)) - (v1581 * ((self.scalar_v1583 * v2404) / v4099))) / v4107);
        let v4137: f64 = (v31 * v1601);
        let v4145: f64 = (v1602 * v1602);
        let v4159: f64 = (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2426)) - (v1598 * ((self.scalar_v1583 * v2426) / v4137))) / v4145) } else { v4 });
        let v4160: f64 = (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2427)) - (v1598 * ((self.scalar_v1583 * v2427) / v4137))) / v4145) } else { v4 });
        let v4161: f64 = (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2428)) - (v1598 * ((self.scalar_v1583 * v2428) / v4137))) / v4145) } else { v4 });
        let v4162: f64 = (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2429)) - (v1598 * ((self.scalar_v1583 * v2429) / v4137))) / v4145) } else { v4 });
        let v4167: f64 = (v1616 * self.scalar_v4163);
        let v4168: f64 = (v4167 + v4167);
        let v4169: f64 = (v1616 * self.scalar_v4164);
        let v4171: f64 = (v1616 * self.scalar_v4165);
        let v4172: f64 = (v4171 + v4171);
        let v4173: f64 = (v1616 * self.scalar_v4166);
        let v4175: f64 = (if self.scalar_v1606 { v4168 } else { v4 });
        let v4176: f64 = (if self.scalar_v1606 { (v4169 + v4169) } else { v4 });
        let v4177: f64 = (if self.scalar_v1606 { v4 } else { v3280 });
        let v4178: f64 = (if self.scalar_v1606 { v4168 } else { v3282 });
        let v4179: f64 = (if self.scalar_v1606 { v4172 } else { v3284 });
        let v4180: f64 = (if self.scalar_v1606 { v4172 } else { v3286 });
        let v4181: f64 = (if self.scalar_v1606 { (v4173 + v4173) } else { v4 });
        let v4182: f64 = (if self.scalar_v1606 { v4172 } else { v4 });
        let v4183: f64 = (v31 * v1625);
        let v4184: f64 = (v4175 / v4183);
        let v4185: f64 = (v4176 / v4183);
        let v4186: f64 = (v4177 / v4183);
        let v4187: f64 = (v4178 / v4183);
        let v4188: f64 = (v4179 / v4183);
        let v4189: f64 = (v4180 / v4183);
        let v4190: f64 = (v4181 / v4183);
        let v4191: f64 = (v4182 / v4183);
        let v4201: f64 = (v1626 * v1626);
        let v4247: f64 = (if v1630 { (v395 * (self.scalar_v4163 + v4184)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4184 - self.scalar_v4163))) / v4201) } else { v4 }) });
        let v4248: f64 = (if v1630 { (v395 * (self.scalar_v4164 + v4185)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4185 - self.scalar_v4164))) / v4201) } else { v4 }) });
        let v4249: f64 = (if v1630 { (v395 * v4186) } else { (if v1622 { ((-(self.scalar_v1623 * v4186)) / v4201) } else { v4 }) });
        let v4250: f64 = (if v1630 { (v395 * (self.scalar_v4163 + v4187)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4187 - self.scalar_v4163))) / v4201) } else { v4 }) });
        let v4251: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4188)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4188 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4252: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4189)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4189 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4253: f64 = (if v1630 { (v395 * (self.scalar_v4166 + v4190)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4190 - self.scalar_v4166))) / v4201) } else { v4 }) });
        let v4254: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4191)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4191 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4255: f64 = (self.scalar_v311 * v4159);
        let v4257: f64 = (self.scalar_v311 * v4161);
        let v4269: f64 = (v1636 * v1636);
        let v4307: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4247) - (v1633 * (v4247 + v4255))) / v4269) } else { v4 }) });
        let v4308: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4248) - (v1633 * (v4248 + (self.scalar_v311 * v4160)))) / v4269) } else { v4 }) });
        let v4309: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4249) - (v1633 * v4249)) / v4269) } else { v4 }) });
        let v4310: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4250) - (v1633 * (v4250 + v4255))) / v4269) } else { v4 }) });
        let v4311: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4251) - (v1633 * (v4251 + v4257))) / v4269) } else { v4 }) });
        let v4312: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4252) - (v1633 * (v4252 + v4257))) / v4269) } else { v4 }) });
        let v4313: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4253) - (v1633 * (v4253 + (self.scalar_v311 * v4162)))) / v4269) } else { v4 }) });
        let v4314: f64 = (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4254) - (v1633 * (v4254 + v4257))) / v4269) } else { v4 }) });
        let v4315: f64 = (v1641 * v4159);
        let v4324: f64 = (v1641 * v4161);
        let v4348: f64 = (v1647 * self.scalar_v4342);
        let v4350: f64 = (v1647 * self.scalar_v4343);
        let v4352: f64 = (v1647 * self.scalar_v4344);
        let v4363: f64 = (v31 * v1656);
        let v4364: f64 = ((if self.scalar_v1645 { v4 } else { v4175 }) / v4363);
        let v4365: f64 = ((if self.scalar_v1645 { v4 } else { v4176 }) / v4363);
        let v4366: f64 = ((if self.scalar_v1645 { v4 } else { v4177 }) / v4363);
        let v4367: f64 = ((if self.scalar_v1645 { (v4348 + v4348) } else { v4175 }) / v4363);
        let v4368: f64 = ((if self.scalar_v1645 { (v4350 + v4350) } else { v4178 }) / v4363);
        let v4369: f64 = ((if self.scalar_v1645 { (v4352 + v4352) } else { v4179 }) / v4363);
        let v4370: f64 = ((if self.scalar_v1645 { v4 } else { v4180 }) / v4363);
        let v4371: f64 = ((if self.scalar_v1645 { v4 } else { v4181 }) / v4363);
        let v4372: f64 = ((if self.scalar_v1645 { v4 } else { v4182 }) / v4363);
        let v4378: f64 = (v1657 * v1657);
        let v4425: f64 = (if v1661 { (v395 * v4364) } else { (if v1653 { ((-(self.scalar_v1654 * v4364)) / v4378) } else { v4 }) });
        let v4426: f64 = (if v1661 { (v395 * v4365) } else { (if v1653 { ((-(self.scalar_v1654 * v4365)) / v4378) } else { v4 }) });
        let v4427: f64 = (if v1661 { (v395 * v4366) } else { (if v1653 { ((-(self.scalar_v1654 * v4366)) / v4378) } else { v4 }) });
        let v4428: f64 = (if v1661 { (v395 * (self.scalar_v4345 + v4367)) } else { (if v1653 { ((-(self.scalar_v1654 * (v4367 - self.scalar_v4345))) / v4378) } else { v4 }) });
        let v4429: f64 = (if v1661 { (v395 * (self.scalar_v4346 + v4368)) } else { (if v1653 { ((-(self.scalar_v1654 * (v4368 - self.scalar_v4346))) / v4378) } else { v4 }) });
        let v4430: f64 = (if v1661 { (v395 * (self.scalar_v4347 + v4369)) } else { (if v1653 { ((-(self.scalar_v1654 * (v4369 - self.scalar_v4347))) / v4378) } else { v4 }) });
        let v4431: f64 = (if v1661 { (v395 * v4370) } else { (if v1653 { ((-(self.scalar_v1654 * v4370)) / v4378) } else { v4 }) });
        let v4432: f64 = (if v1661 { (v395 * v4371) } else { (if v1653 { ((-(self.scalar_v1654 * v4371)) / v4378) } else { v4 }) });
        let v4433: f64 = (if v1661 { (v395 * v4372) } else { (if v1653 { ((-(self.scalar_v1654 * v4372)) / v4378) } else { v4 }) });
        let v4444: f64 = (self.scalar_v1665 * f64::powf(v1682, self.scalar_v1674));
        let v4454: f64 = (v1684 * v1684);
        let v4491: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4425) } else { (if v1681 { (((v4425 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4492: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4426) } else { (if v1681 { (((v4426 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4493: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4427) } else { (if v1681 { (((v4427 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4494: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4428) } else { (if v1681 { (((v4428 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4495: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4429) } else { (if v1681 { (((v4429 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4496: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4430) } else { (if v1681 { (((v4430 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4497: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4431) } else { (if v1681 { (((v4431 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4498: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4432) } else { (if v1681 { (((v4432 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4499: f64 = (if self.scalar_v1693 { v4 } else { (if v1688 { (self.scalar_v1679 * v4433) } else { (if v1681 { (((v4433 / self.scalar_v1670) * v4444) / v4454) } else { v4 }) }) });
        let v4522: f64 = (v1694 * (if self.scalar_v1592 { (self.scalar_v14 * v4116) } else { v4116 }));
        let v4542: f64 = (v1694 * (self.scalar_v457 * v3679));
        let v4551: f64 = (v1694 * (if self.scalar_v1592 { (v4315 + (v1604 * v4307)) } else { v4 }));
        let v4577: f64 = (v1136 * v3232);
        let v4579: f64 = (v1136 * v3237);
        let v4581: f64 = (v1136 * v3235);
        let v4583: f64 = (v1136 * v3236);
        let v4585: f64 = (v31 * v1702);
        let v4586: f64 = ((v4577 + v4577) / v4585);
        let v4587: f64 = ((v4579 + v4579) / v4585);
        let v4588: f64 = ((v4581 + v4581) / v4585);
        let v4589: f64 = ((v4583 + v4583) / v4585);
        let v4596: f64 = (v1703 * v1703);
        let v4619: f64 = (if v1706 { (v395 * (v3232 + v4586)) } else { (if v1700 { ((-(v1158 * (v4586 - v3232))) / v4596) } else { v4 }) });
        let v4620: f64 = (if v1706 { (v395 * (v3237 + v4587)) } else { (if v1700 { ((-(v1158 * (v4587 - v3237))) / v4596) } else { v4 }) });
        let v4621: f64 = (if v1706 { (v395 * (v3235 + v4588)) } else { (if v1700 { ((-(v1158 * (v4588 - v3235))) / v4596) } else { v4 }) });
        let v4622: f64 = (if v1706 { (v395 * (v3236 + v4589)) } else { (if v1700 { ((-(v1158 * (v4589 - v3236))) / v4596) } else { v4 }) });
        let v4637: f64 = (v1710 * v1710);
        let v4652: f64 = (v154 * (if v1712 { v4 } else { ((-(self.scalar_v299 * ((v1709 * v3326) + (v1170 * v4619)))) / v4637) }));
        let v4653: f64 = (v154 * (if v1712 { v4 } else { ((-(self.scalar_v299 * ((v1709 * v3327) + (v1170 * v4620)))) / v4637) }));
        let v4654: f64 = (v154 * (if v1712 { v4 } else { ((-(self.scalar_v299 * ((v1709 * v3328) + (v1170 * v4621)))) / v4637) }));
        let v4655: f64 = (v154 * (if v1712 { v4 } else { ((-(self.scalar_v299 * ((v1709 * v3329) + (v1170 * v4622)))) / v4637) }));
        let v4662: f64 = (v1714 * v1714);
        let v4679: f64 = ((-v3354) / self.scalar_v1725);
        let v4680: f64 = ((-v3358) / self.scalar_v1725);
        let v4681: f64 = ((-v3362) / self.scalar_v1725);
        let v4682: f64 = ((-v3366) / self.scalar_v1725);
        let v4707: f64 = (if v1729 { (v1740 * (if v1734 { (v1735 * v4679) } else { (if v1730 { (v1731 * v4679) } else { v4 }) })) } else { v4 });
        let v4708: f64 = (if v1729 { ((v1740 * (if v1734 { (v1735 * v4680) } else { (if v1730 { (v1731 * v4680) } else { v4 }) })) + (v1739 * self.scalar_v2363)) } else { v4 });
        let v4709: f64 = (if v1729 { ((v1740 * (if v1734 { (v1735 * v4681) } else { (if v1730 { (v1731 * v4681) } else { v4 }) })) + (self.scalar_v0 * v1739)) } else { v4 });
        let v4710: f64 = (if v1729 { (v1740 * (if v1734 { (v1735 * v4682) } else { (if v1730 { (v1731 * v4682) } else { v4 }) })) } else { v4 });
        let v4713: f64 = (self.scalar_v1744 * f64::powf(v1742, self.scalar_v4711));
        let v4718: f64 = (self.scalar_v1743 * (v4707 * v4713));
        let v4719: f64 = (self.scalar_v1743 * (v4708 * v4713));
        let v4720: f64 = (self.scalar_v1743 * (v4709 * v4713));
        let v4721: f64 = (self.scalar_v1743 * (v4710 * v4713));
        let v4734: f64 = (if v1752 { (v1753 * v4718) } else { (if v1748 { (v1749 * v4718) } else { v4 }) });
        let v4735: f64 = (if v1752 { (v1753 * v4719) } else { (if v1748 { (v1749 * v4719) } else { v4 }) });
        let v4736: f64 = (if v1752 { (v1753 * v4720) } else { (if v1748 { (v1749 * v4720) } else { v4 }) });
        let v4737: f64 = (if v1752 { (v1753 * v4721) } else { (if v1748 { (v1749 * v4721) } else { v4 }) });
        let v4761: f64 = (v1033 * v1033);
        let v4770: f64 = (if v1768 { (((v1033 * self.scalar_v2363) - (v1775 * v3020)) / v4761) } else { v2811 });
        let v4771: f64 = (if v1768 { (((self.scalar_v0 * v1033) - (v1775 * v3021)) / v4761) } else { v2812 });
        let v4772: f64 = (if v1768 { ((-(v1775 * v3022)) / v4761) } else { v2813 });
        let v4779: f64 = (v31 * v1780);
        let v4783: f64 = (if v1768 { (((v31 * v4770) / v1774) / v4779) } else { v4 });
        let v4784: f64 = (if v1768 { (((v31 * v4771) / v1774) / v4779) } else { v4 });
        let v4785: f64 = (if v1768 { (((v31 * v4772) / v1774) / v4779) } else { v4 });
        let v4792: f64 = (if v1787 { (-(v395 * v3002)) } else { v4 });
        let v4793: f64 = (if v1787 { (-(v395 * v3003)) } else { v4 });
        let v4794: f64 = (if v1787 { (-(v395 * v3004)) } else { v4 });
        let v4807: f64 = (if v1787 { ((v1791 * v4792) + (v1790 * (self.scalar_v1771 * v4792))) } else { v4 });
        let v4808: f64 = (if v1787 { ((v1791 * v4793) + (v1790 * (self.scalar_v1771 * v4793))) } else { v4 });
        let v4809: f64 = (if v1787 { ((v1791 * v4794) + (v1790 * (self.scalar_v1771 * v4794))) } else { v4 });
        let v4819: f64 = (v1781 * v4783);
        let v4821: f64 = (v1781 * v4784);
        let v4823: f64 = (v1781 * v4785);
        let v4825: f64 = (v1793 * v4807);
        let v4827: f64 = (v1793 * v4808);
        let v4829: f64 = (v1793 * v4809);
        let v4834: f64 = (v31 * v1798);
        let v4841: f64 = (v1798 * v1798);
        let v4851: f64 = (if v1768 { (((v1798 * ((v1793 * v4783) + (v1781 * v4807))) - (v1794 * (((v4819 + v4819) + (v4825 + v4825)) / v4834))) / v4841) } else { v4 });
        let v4852: f64 = (if v1768 { (((v1798 * ((v1793 * v4784) + (v1781 * v4808))) - (v1794 * (((v4821 + v4821) + (v4827 + v4827)) / v4834))) / v4841) } else { v4 });
        let v4853: f64 = (if v1768 { (((v1798 * ((v1793 * v4785) + (v1781 * v4809))) - (v1794 * (((v4823 + v4823) + (v4829 + v4829)) / v4834))) / v4841) } else { v4 });
        let v4857: f64 = (v1800 * v1800);
        let v4866: f64 = (if v1768 { (((v1800 * self.scalar_v2363) - (v1775 * v4851)) / v4857) } else { v4 });
        let v4867: f64 = (if v1768 { (((self.scalar_v0 * v1800) - (v1775 * v4852)) / v4857) } else { v4 });
        let v4868: f64 = (if v1768 { ((-(v1775 * v4853)) / v4857) } else { v4 });
        let v4869: f64 = (v395 * v4851);
        let v4870: f64 = (v395 * v4852);
        let v4871: f64 = (v395 * v4853);
        let v4872: f64 = (v1774 * v4869);
        let v4873: f64 = (v1774 * v4870);
        let v4874: f64 = (v1774 * v4871);
        let v4887: f64 = (if v1768 { (v4866 + ((v1804 * v3020) + (v1033 * v4872))) } else { v4 });
        let v4888: f64 = (if v1768 { (v4867 + ((v1804 * v3021) + (v1033 * v4873))) } else { v4 });
        let v4889: f64 = (if v1768 { (v4868 + ((v1804 * v3022) + (v1033 * v4874))) } else { v4 });
        let v4909: f64 = (v1820 * v1820);
        let v4926: f64 = ((v1822 * v4872) + (v1804 * (-(((v1820 * v3358) - (v1177 * (self.scalar_v871 * (if v1787 { (self.scalar_v1810 * (v31 * v3002)) } else { v4 })))) / v4909))));
        let v4929: f64 = ((v1822 * v4873) + (v1804 * (-(((v1820 * v3362) - (v1177 * (self.scalar_v871 * (if v1787 { (self.scalar_v1810 * (v31 * v3003)) } else { v4 })))) / v4909))));
        let v4932: f64 = ((v1822 * v4874) + (v1804 * (-(((v1820 * v3366) - (v1177 * (self.scalar_v871 * (if v1787 { (self.scalar_v1810 * (v31 * v3004)) } else { v4 })))) / v4909))));
        let v4937: f64 = (if v1787 { (-(v1804 * (-(v3354 / v1820)))) } else { v4 });
        let v4938: f64 = (if v1787 { (v4866 - v4926) } else { v4 });
        let v4939: f64 = (if v1787 { (v4867 - v4929) } else { v4 });
        let v4940: f64 = (if v1787 { (v4868 - v4932) } else { v4 });
        let v4944: f64 = (v1826 * v4937);
        let v4946: f64 = (v1826 * (v4938 - v4887));
        let v4948: f64 = (v1826 * (v4939 - v4888));
        let v4950: f64 = (v1826 * (v4940 - v4889));
        let v4986: f64 = (v31 * v1835);
        let v4988: f64 = ((if v1787 { ((v4946 + v4946) + (((v1829 * v3011) + (v1030 * ((v1828 * v4866) + (v1802 * (v46 * v4866))))) / self.scalar_v871)) } else { v4770 }) / v4986);
        let v4989: f64 = ((if v1787 { ((v4948 + v4948) + (((v1829 * v3012) + (v1030 * ((v1828 * v4867) + (v1802 * (v46 * v4867))))) / self.scalar_v871)) } else { v4771 }) / v4986);
        let v4990: f64 = ((if v1787 { ((v4950 + v4950) + (((v1829 * v3013) + (v1030 * ((v1828 * v4868) + (v1802 * (v46 * v4868))))) / self.scalar_v871)) } else { v4772 }) / v4986);
        let v4999: f64 = (if v1787 { (v395 * (v4937 + ((if v1787 { (v4944 + v4944) } else { v4 }) / v4986))) } else { v4 });
        let v5000: f64 = (if v1787 { (v395 * ((v4887 + v4938) + v4988)) } else { (if v1784 { v4887 } else { v4 }) });
        let v5001: f64 = (if v1787 { (v395 * ((v4888 + v4939) + v4989)) } else { (if v1784 { v4888 } else { v4 }) });
        let v5002: f64 = (if v1787 { (v395 * ((v4889 + v4940) + v4990)) } else { (if v1784 { v4889 } else { v4 }) });
        let v5009: f64 = (v1838 * v1838);
        let v5029: f64 = (v1841 * v1841);
        let v5043: f64 = (if v1845 { ((-(v1803 * (if v1768 { (((v1838 * v4999) - (v1839 * v4999)) / v5009) } else { v4 }))) / v5029) } else { v4 });
        let v5044: f64 = (if v1845 { (((v1841 * v4869) - (v1803 * (if v1768 { (((v1838 * (v5000 - v4866)) - (v1839 * v5000)) / v5009) } else { v4 }))) / v5029) } else { v4 });
        let v5045: f64 = (if v1845 { (((v1841 * v4870) - (v1803 * (if v1768 { (((v1838 * (v5001 - v4867)) - (v1839 * v5001)) / v5009) } else { v4 }))) / v5029) } else { v4 });
        let v5046: f64 = (if v1845 { (((v1841 * v4871) - (v1803 * (if v1768 { (((v1838 * (v5002 - v4868)) - (v1839 * v5002)) / v5009) } else { v4 }))) / v5029) } else { v4 });
        let v5065: f64 = ((-(self.scalar_v1851 * v4999)) / v5009);
        let v5068: f64 = ((-(self.scalar_v1851 * v5000)) / v5009);
        let v5071: f64 = ((-(self.scalar_v1851 * v5001)) / v5009);
        let v5074: f64 = ((-(self.scalar_v1851 * v5002)) / v5009);
        let v5075: f64 = (v1853 * v5065);
        let v5076: f64 = (v1853 * v5068);
        let v5077: f64 = (v1853 * v5071);
        let v5078: f64 = (v1853 * v5074);
        let v5081: f64 = (v1847 * v1847);
        let v5117: f64 = ((v1858 * ((v1849 * v5043) + (v1847 * (self.scalar_v1848 * v4999)))) + (v1850 * (v5075 - (v1857 * ((v1855 * v5065) + (v1852 * ((-(v1793 * v5043)) / v5081)))))));
        let v5120: f64 = ((v1858 * ((v1849 * v5044) + (v1847 * (self.scalar_v1848 * v5000)))) + (v1850 * (v5076 - (v1857 * ((v1855 * v5068) + (v1852 * (((v1847 * v4807) - (v1793 * v5044)) / v5081)))))));
        let v5123: f64 = ((v1858 * ((v1849 * v5045) + (v1847 * (self.scalar_v1848 * v5001)))) + (v1850 * (v5077 - (v1857 * ((v1855 * v5071) + (v1852 * (((v1847 * v4808) - (v1793 * v5045)) / v5081)))))));
        let v5126: f64 = ((v1858 * ((v1849 * v5046) + (v1847 * (self.scalar_v1848 * v5002)))) + (v1850 * (v5078 - (v1857 * ((v1855 * v5074) + (v1852 * (((v1847 * v4809) - (v1793 * v5046)) / v5081)))))));
        let v5145: f64 = (if v1862 { ((v1863 * v5076) + (v1853 * (self.scalar_v10 * v4807))) } else { (if v1845 { v5120 } else { (if v1729 { ((v1760 * v4735) + (v1757 * (self.scalar_v1759 * v4708))) } else { v4 }) }) });
        let v5146: f64 = (if v1862 { ((v1863 * v5077) + (v1853 * (self.scalar_v10 * v4808))) } else { (if v1845 { v5123 } else { (if v1729 { ((v1760 * v4736) + (v1757 * (self.scalar_v1759 * v4709))) } else { v4 }) }) });
        let v5147: f64 = (if v1862 { ((v1863 * v5078) + (v1853 * (self.scalar_v10 * v4809))) } else { (if v1845 { v5126 } else { (if v1729 { ((v1760 * v4737) + (v1757 * (self.scalar_v1759 * v4710))) } else { v4 }) }) });
        let v5149: f64 = (self.scalar_v1744 * f64::powf(v1740, self.scalar_v4711));
        let v5155: f64 = (v1873 * v1873);
        let v5175: f64 = (self.scalar_v1876 * f64::powf(v1875, self.scalar_v5173));
        let v5188: f64 = (if v1870 { (v1871 * ((-(((v1873 * v3354) - (v1177 * v3354)) / v5155)) * v5175)) } else { v4 });
        let v5189: f64 = (if v1870 { ((v1877 * (self.scalar_v2363 * v5149)) + (v1871 * ((-(((v1873 * v3358) - (v1177 * v3358)) / v5155)) * v5175))) } else { v4 });
        let v5190: f64 = (if v1870 { ((v1877 * (self.scalar_v0 * v5149)) + (v1871 * ((-(((v1873 * v3362) - (v1177 * v3362)) / v5155)) * v5175))) } else { v4 });
        let v5191: f64 = (if v1870 { (v1871 * ((-(((v1873 * v3366) - (v1177 * v3366)) / v5155)) * v5175)) } else { v4 });
        let v5200: f64 = (if v1882 { (v3354 / self.scalar_v1872) } else { v4 });
        let v5201: f64 = (if v1882 { (v3358 / self.scalar_v1872) } else { v4 });
        let v5202: f64 = (if v1882 { (v3362 / self.scalar_v1872) } else { v4 });
        let v5203: f64 = (if v1882 { (v3366 / self.scalar_v1872) } else { v4 });
        let v5208: f64 = (if v1882 { (v5200 / self.scalar_v1888) } else { self.scalar_v3401 });
        let v5209: f64 = (if v1882 { (v5201 / self.scalar_v1888) } else { self.scalar_v3402 });
        let v5210: f64 = (if v1882 { (v5202 / self.scalar_v1888) } else { v4 });
        let v5211: f64 = (if v1882 { (v5203 / self.scalar_v1888) } else { v4 });
        let v5254: f64 = (self.scalar_v1908 * f64::powf(v1907, self.scalar_v5252));
        let v5260: f64 = (v1879 * ((if v1900 { (v5200 + (self.scalar_v1888 * ((v1902 * (-v5208)) / v1903))) } else { (if v1892 { (self.scalar_v1888 * ((v1893 * v5208) / v1894)) } else { v4 }) }) * v5254));
        let v5263: f64 = (v1879 * ((if v1900 { (v5201 + (self.scalar_v1888 * ((v1902 * (-v5209)) / v1903))) } else { (if v1892 { (self.scalar_v1888 * ((v1893 * v5209) / v1894)) } else { v4 }) }) * v5254));
        let v5266: f64 = (v1879 * ((if v1900 { (v5202 + (self.scalar_v1888 * ((v1902 * (-v5210)) / v1903))) } else { (if v1892 { (self.scalar_v1888 * ((v1893 * v5210) / v1894)) } else { v4 }) }) * v5254));
        let v5269: f64 = (v1879 * ((if v1900 { (v5203 + (self.scalar_v1888 * ((v1902 * (-v5211)) / v1903))) } else { (if v1892 { (self.scalar_v1888 * ((v1893 * v5211) / v1894)) } else { v4 }) }) * v5254));
        let v5275: f64 = (self.scalar_v1743 * (if v1882 { ((v1909 * v5188) + v5260) } else { (if v1880 { v5188 } else { v4 }) }));
        let v5276: f64 = (self.scalar_v1743 * (if v1882 { ((v1909 * v5189) + v5263) } else { (if v1880 { v5189 } else { v4 }) }));
        let v5277: f64 = (self.scalar_v1743 * (if v1882 { ((v1909 * v5190) + v5266) } else { (if v1880 { v5190 } else { v4 }) }));
        let v5278: f64 = (self.scalar_v1743 * (if v1882 { ((v1909 * v5191) + v5269) } else { (if v1880 { v5191 } else { v4 }) }));
        let v5305: f64 = (if v1870 { (v1924 * (if v1918 { (v1919 * v5275) } else { (if v1914 { (v1915 * v5275) } else { v4734 }) })) } else { (if v1862 { (v1863 * v5075) } else { (if v1845 { v5117 } else { (if v1729 { ((v1760 * v4734) + (v1757 * (self.scalar_v1759 * v4707))) } else { v4 }) }) }) });
        let v5306: f64 = (if v1870 { ((v1924 * (if v1918 { (v1919 * v5276) } else { (if v1914 { (v1915 * v5276) } else { v4735 }) })) + (v1923 * self.scalar_v5295)) } else { v5145 });
        let v5307: f64 = (if v1870 { ((v1924 * (if v1918 { (v1919 * v5277) } else { (if v1914 { (v1915 * v5277) } else { v4736 }) })) + (v1923 * self.scalar_v5296)) } else { v5146 });
        let v5308: f64 = (if v1870 { (v1924 * (if v1918 { (v1919 * v5278) } else { (if v1914 { (v1915 * v5278) } else { v4737 }) })) } else { v5147 });
        let v5323: f64 = (v1933 * v1933);
        let v5348: f64 = (v1932 * v1932);
        let v5359: f64 = ((((-(self.scalar_v103 * ((v1932 * v3354) + (v1177 * v4652)))) / v5323) + (self.scalar_v471 * (v3332 / self.scalar_v420))) + ((-(self.scalar_v292 * v4652)) / v5348));
        let v5360: f64 = ((((-(self.scalar_v103 * ((v1932 * v3358) + (v1177 * v4653)))) / v5323) + (self.scalar_v471 * (v3335 / self.scalar_v420))) + ((-(self.scalar_v292 * v4653)) / v5348));
        let v5361: f64 = ((((-(self.scalar_v103 * ((v1932 * v3362) + (v1177 * v4654)))) / v5323) + (self.scalar_v471 * (v3338 / self.scalar_v420))) + ((-(self.scalar_v292 * v4654)) / v5348));
        let v5362: f64 = ((((-(self.scalar_v103 * ((v1932 * v3366) + (v1177 * v4655)))) / v5323) + (self.scalar_v471 * (v3341 / self.scalar_v420))) + ((-(self.scalar_v292 * v4655)) / v5348));
        let v5363: f64 = (if v1931 { v5359 } else { v4 });
        let v5364: f64 = (if v1931 { v5360 } else { v4 });
        let v5365: f64 = (if v1931 { v5361 } else { v4 });
        let v5366: f64 = (if v1931 { v5362 } else { v4 });
        let v5375: f64 = (if v1941 { ((v5305 - v5363) / v392) } else { v5208 });
        let v5376: f64 = (if v1941 { ((v5306 - v5364) / v392) } else { v5209 });
        let v5377: f64 = (if v1941 { ((v5307 - v5365) / v392) } else { v5210 });
        let v5378: f64 = (if v1941 { ((v5308 - v5366) / v392) } else { v5211 });
        let v5419: f64 = (if v1954 { (v5363 - (v392 * ((v1956 * (-v5375)) / v1957))) } else { (if v1946 { (v5305 - (v392 * ((v1947 * v5375) / v1948))) } else { v5305 }) });
        let v5420: f64 = (if v1954 { (v5364 - (v392 * ((v1956 * (-v5376)) / v1957))) } else { (if v1946 { (v5306 - (v392 * ((v1947 * v5376) / v1948))) } else { v5306 }) });
        let v5421: f64 = (if v1954 { (v5365 - (v392 * ((v1956 * (-v5377)) / v1957))) } else { (if v1946 { (v5307 - (v392 * ((v1947 * v5377) / v1948))) } else { v5307 }) });
        let v5422: f64 = (if v1954 { (v5366 - (v392 * ((v1956 * (-v5378)) / v1957))) } else { (if v1946 { (v5308 - (v392 * ((v1947 * v5378) / v1948))) } else { v5308 }) });
        let v5425: f64 = ((v1961 * v3354) + (v1177 * v5419));
        let v5428: f64 = ((v1961 * v3358) + (v1177 * v5420));
        let v5431: f64 = ((v1961 * v3362) + (v1177 * v5421));
        let v5434: f64 = ((v1961 * v3366) + (v1177 * v5422));
        let v5458: f64 = (v1967 * v1967);
        let v5476: f64 = (if v1971 { v5425 } else { (if v1965 { (((v1967 * ((v1962 * v5363) + (v1940 * v5425))) - (v1966 * (v5363 + v5419))) / v5458) } else { (if v1941 { v5425 } else { v4 }) }) });
        let v5477: f64 = (if v1971 { v5428 } else { (if v1965 { (((v1967 * ((v1962 * v5364) + (v1940 * v5428))) - (v1966 * (v5364 + v5420))) / v5458) } else { (if v1941 { v5428 } else { v4 }) }) });
        let v5478: f64 = (if v1971 { v5431 } else { (if v1965 { (((v1967 * ((v1962 * v5365) + (v1940 * v5431))) - (v1966 * (v5365 + v5421))) / v5458) } else { (if v1941 { v5431 } else { v4 }) }) });
        let v5479: f64 = (if v1971 { v5434 } else { (if v1965 { (((v1967 * ((v1962 * v5366) + (v1940 * v5434))) - (v1966 * (v5366 + v5422))) / v5458) } else { (if v1941 { v5434 } else { v4 }) }) });
        let v5500: f64 = (if v1986 { (-(self.scalar_v1038 * ((v1988 * self.scalar_v3035) / v1989))) } else { (if v1979 { (self.scalar_v2363 - (self.scalar_v1038 * ((v1980 * self.scalar_v3023) / v1981))) } else { v4 }) });
        let v5501: f64 = (if v1986 { (-(self.scalar_v1038 * ((v1988 * self.scalar_v3036) / v1989))) } else { (if v1979 { (self.scalar_v0 - (self.scalar_v1038 * ((v1980 * self.scalar_v3024) / v1981))) } else { v4 }) });
        let v5507: f64 = (self.scalar_v1058 * f64::powf(v1996, self.scalar_v3051));
        let v5529: f64 = ((v2009 * v4619) + (v1709 * (self.scalar_v2008 * v3201)));
        let v5532: f64 = ((v2009 * v4620) + (v1709 * (self.scalar_v2008 * v3205)));
        let v5533: f64 = (v2009 * v4621);
        let v5534: f64 = (v2009 * v4622);
        let v5538: f64 = (v2011 * v4619);
        let v5541: f64 = ((v2011 * v4620) + (v1709 * (self.scalar_v2008 * v3223)));
        let v5544: f64 = ((v2011 * v4621) + (v1709 * (self.scalar_v2008 * v3227)));
        let v5547: f64 = ((v2011 * v4622) + (v1709 * (self.scalar_v2008 * v3231)));
        let v5592: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5572) / v2025))) } else { (if v2015 { (self.scalar_v0 - (self.scalar_v983 * ((v2016 * self.scalar_v5548) / v2017))) } else { v4 }) });
        let v5593: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5573) / v2025))) } else { (if v2015 { (self.scalar_v2364 - (self.scalar_v983 * ((v2016 * self.scalar_v5549) / v2017))) } else { v4 }) });
        let v5594: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5574) / v2025))) } else { (if v2015 { (self.scalar_v2365 - (self.scalar_v983 * ((v2016 * self.scalar_v5550) / v2017))) } else { v4 }) });
        let v5595: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5575) / v2025))) } else { (if v2015 { (self.scalar_v2363 - (self.scalar_v983 * ((v2016 * self.scalar_v5551) / v2017))) } else { v4 }) });
        let v5605: f64 = (self.scalar_v1103 * f64::powf(v2031, self.scalar_v3146));
        let v5648: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3188 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5592 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v0 - v5592))))))));
        let v5649: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5593 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2364 - v5593)))) + self.scalar_v5634))));
        let v5650: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5594 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2365 - v5594)))) + self.scalar_v5635))));
        let v5651: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3189 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5595 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2363 - v5595))))))));
        let v5690: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5573) / v2057))) } else { (if v2047 { (self.scalar_v2364 - (self.scalar_v983 * ((v2048 * self.scalar_v5549) / v2049))) } else { v4 }) });
        let v5691: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5673) / v2057))) } else { (if v2047 { (self.scalar_v2366 - (self.scalar_v983 * ((v2048 * self.scalar_v5652) / v2049))) } else { v4 }) });
        let v5692: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5574) / v2057))) } else { (if v2047 { (self.scalar_v2365 - (self.scalar_v983 * ((v2048 * self.scalar_v5550) / v2049))) } else { v4 }) });
        let v5693: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5575) / v2057))) } else { (if v2047 { (self.scalar_v2363 - (self.scalar_v983 * ((v2048 * self.scalar_v5551) / v2049))) } else { v4 }) });
        let v5703: f64 = (self.scalar_v1103 * f64::powf(v2063, self.scalar_v3146));
        let v5745: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v5634 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5690 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2364 - v5690))))))));
        let v5746: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5691 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2366 - v5691)))) + self.scalar_v5732))));
        let v5747: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v5635 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5692 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2365 - v5692))))))));
        let v5748: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3189 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5693 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2363 - v5693))))))));
        let v5765: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * self.scalar_v5749) } else { (if v2084 { (v2085 * self.scalar_v5749) } else { v3697 }) }));
        let v5766: f64 = (self.scalar_v2081 * (if v2087 { v4 } else { (if v2084 { v4 } else { v3698 }) }));
        let v5767: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * self.scalar_v5750) } else { (if v2084 { (v2085 * self.scalar_v5750) } else { v3699 }) }));
        let v5768: f64 = (self.scalar_v2081 * (if v2087 { v4 } else { (if v2084 { v4 } else { v3700 }) }));
        let v5769: f64 = (self.scalar_v2081 * (if v2087 { v4 } else { (if v2084 { v4 } else { v3701 }) }));
        let v5790: f64 = ((self.scalar_v2007 * (((v1573 * v4039) - (v1570 * (v4039 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4043) - (v1569 * (v4043 / v4069))) / v4077)));
        let v5791: f64 = ((self.scalar_v2007 * (((v1573 * v4040) - (v1570 * (v4040 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4044) - (v1569 * (v4044 / v4069))) / v4077)));
        let v5792: f64 = ((self.scalar_v2007 * (((v1573 * v4041) - (v1570 * (v4041 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4045) - (v1569 * (v4045 / v4069))) / v4077)));
        let v5793: f64 = ((self.scalar_v2007 * (((v1573 * v4042) - (v1570 * (v4042 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4046) - (v1569 * (v4046 / v4069))) / v4077)));
        let v5838: f64 = (v31 * v2130);
        let v5846: f64 = (v2131 * v2131);
        let v5847: f64 = (((v2131 * (self.scalar_v2126 * v2401)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5810) } else { (if v2116 { (v2117 * self.scalar_v5810) } else { v4 }) })) / v5838))) / v5846);
        let v5851: f64 = (((v2131 * (self.scalar_v2126 * v2402)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5811) } else { (if v2116 { (v2117 * self.scalar_v5811) } else { v4 }) })) / v5838))) / v5846);
        let v5855: f64 = (((v2131 * (self.scalar_v2126 * v2403)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5812) } else { (if v2116 { (v2117 * self.scalar_v5812) } else { v4 }) })) / v5838))) / v5846);
        let v5859: f64 = (((v2131 * (self.scalar_v2126 * v2404)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5813) } else { (if v2116 { (v2117 * self.scalar_v5813) } else { v4 }) })) / v5838))) / v5846);
        let v5860: f64 = (if self.scalar_v2115 { v5847 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5790) / self.scalar_v617) } else { v4 }) });
        let v5861: f64 = (if self.scalar_v2115 { v5851 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5791) / self.scalar_v617) } else { v4 }) });
        let v5862: f64 = (if self.scalar_v2115 { v5855 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5792) / self.scalar_v617) } else { v4 }) });
        let v5863: f64 = (if self.scalar_v2115 { v5859 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5793) / self.scalar_v617) } else { v4 }) });
        let v5876: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2426) } else { v4 });
        let v5877: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2427) } else { v4 });
        let v5878: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2428) } else { v4 });
        let v5879: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2429) } else { v4 });
        let v5880: f64 = (v31 * v2144);
        let v5888: f64 = (v2145 * v2145);
        let v5910: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2387) } else { (if v767 { (v768 * self.scalar_v2387) } else { v4 }) })) } else { v4 });
        let v5911: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2413) } else { (if v767 { (v768 * self.scalar_v2413) } else { v4 }) })) } else { v4 });
        let v5912: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2388) } else { (if v767 { (v768 * self.scalar_v2388) } else { v4 }) })) } else { v4 });
        let v5913: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2368) } else { (if v767 { (v768 * self.scalar_v2368) } else { v4 }) })) } else { v4 });
        let v5914: f64 = (v31 * v2151);
        let v5922: f64 = (v2152 * v2152);
        let v5948: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5876) - (v2142 * (v5876 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5910) - (v2149 * (v5910 / v5914))) / v5922) } else { v4 })));
        let v5949: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5877) - (v2142 * (v5877 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5911) - (v2149 * (v5911 / v5914))) / v5922) } else { v4 })));
        let v5950: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5878) - (v2142 * (v5878 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5912) - (v2149 * (v5912 / v5914))) / v5922) } else { v4 })));
        let v5951: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5879) - (v2142 * (v5879 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5913) - (v2149 * (v5913 / v5914))) / v5922) } else { v4 })));
        let v5988: f64 = (v31 * v2181);
        let v5996: f64 = (v2182 * v2182);
        let v5997: f64 = (((v2182 * (self.scalar_v2177 * v2426)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2387) } else { (if v2167 { (v2168 * self.scalar_v2387) } else { v4 }) })) / v5988))) / v5996);
        let v6001: f64 = (((v2182 * (self.scalar_v2177 * v2427)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2413) } else { (if v2167 { (v2168 * self.scalar_v2413) } else { v4 }) })) / v5988))) / v5996);
        let v6005: f64 = (((v2182 * (self.scalar_v2177 * v2428)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2388) } else { (if v2167 { (v2168 * self.scalar_v2388) } else { v4 }) })) / v5988))) / v5996);
        let v6009: f64 = (((v2182 * (self.scalar_v2177 * v2429)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2368) } else { (if v2167 { (v2168 * self.scalar_v2368) } else { v4 }) })) / v5988))) / v5996);
        let v6015: f64 = (v1641 * (if self.scalar_v2166 { v5997 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5948) / self.scalar_v617) } else { v4 }) }));
        let v6024: f64 = (v1641 * (if self.scalar_v2166 { v6005 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5950) / self.scalar_v617) } else { v4 }) }));
        let v6043: f64 = (self.scalar_v2189 * f64::powf(v1057, self.scalar_v6041));
        let v6053: f64 = (v2197 * v2197);
        let v6061: f64 = (v2203 * self.scalar_v6059);
        let v6062: f64 = (v2203 * self.scalar_v6060);
        let v6066: f64 = (v2204 * v2204);
        let v6076: f64 = ((v2206 * (if self.scalar_v2188 { (v3049 * v6043) } else { v4 })) + (v2192 * (if v2201 { (((v2204 * v6061) - (v2203 * v6061)) / v6066) } else { (if v2195 { ((-(v2196 * self.scalar_v6048)) / v6053) } else { v4 }) })));
        let v6079: f64 = ((v2206 * (if self.scalar_v2188 { (v3050 * v6043) } else { v4 })) + (v2192 * (if v2201 { (((v2204 * v6062) - (v2203 * v6062)) / v6066) } else { (if v2195 { ((-(v2196 * self.scalar_v6049)) / v6053) } else { v4 }) })));
        let v6092: f64 = (v1122 * v1122);
        let v6111: f64 = ((v2217 * (if self.scalar_v2188 { ((v2214 * ((self.scalar_v105 * v3192) / self.scalar_v355)) + (v2213 * ((-(v395 * v3195)) / v6092))) } else { v4 })) + (v2216 * (self.scalar_v2008 * v4619)));
        let v6114: f64 = ((v2217 * (if self.scalar_v2188 { ((v2214 * ((self.scalar_v105 * v3193) / self.scalar_v355)) + (v2213 * ((-(v395 * v3196)) / v6092))) } else { v4 })) + (v2216 * (self.scalar_v2008 * v4620)));
        let v6129: f64 = (if self.scalar_v2188 { (v5768 / self.scalar_v2082) } else { v4 });
        let v6145: f64 = ((v2224 * self.scalar_v6132) + (v2222 * ((if self.scalar_v2188 { (v5767 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { v6079 } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { v6114 } else { v4 })))));
        let v6149: f64 = (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v5765 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { v6076 } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { v6111 } else { v4 })))) } else { v4 });
        let v6168: f64 = (self.scalar_v2227 * v5768);
        let v6174: f64 = (if self.scalar_v2188 { (v5529 + (self.scalar_v2227 * v5765)) } else { v4 });
        let v6175: f64 = (if self.scalar_v2188 { (self.scalar_v2227 * v5766) } else { v4 });
        let v6176: f64 = (if self.scalar_v2188 { (v5532 + (self.scalar_v2227 * v5767)) } else { v4 });
        let v6177: f64 = (if self.scalar_v2188 { (v5533 + v6168) } else { v4 });
        let v6178: f64 = (if self.scalar_v2188 { (v5534 + v6168) } else { v4 });
        let v6179: f64 = (if self.scalar_v2188 { (self.scalar_v2227 * v5769) } else { v4 });
        let v6208: f64 = (if self.scalar_v2241 { v5529 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6174) } else { v4 }) });
        let v6209: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6175) } else { v4 }) });
        let v6210: f64 = (if self.scalar_v2241 { v5532 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6176) } else { v4 }) });
        let v6211: f64 = (if self.scalar_v2241 { v5533 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6177) } else { v4 }) });
        let v6212: f64 = (if self.scalar_v2241 { v5534 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6178) } else { v4 }) });
        let v6213: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6179) } else { v4 }) });
        let v6214: f64 = (if self.scalar_v2241 { v5538 } else { (if self.scalar_v2188 { (v5538 + (self.scalar_v2234 * v6174)) } else { v4 }) });
        let v6215: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2234 * v6175) } else { v4 }) });
        let v6216: f64 = (if self.scalar_v2241 { v5541 } else { (if self.scalar_v2188 { (v5541 + (self.scalar_v2234 * v6176)) } else { v4 }) });
        let v6217: f64 = (if self.scalar_v2241 { v5544 } else { (if self.scalar_v2188 { (v5544 + (self.scalar_v2234 * v6177)) } else { v4 }) });
        let v6218: f64 = (if self.scalar_v2241 { v5547 } else { (if self.scalar_v2188 { (v5547 + (self.scalar_v2234 * v6178)) } else { v4 }) });
        let v6219: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2234 * v6179) } else { v4 }) });
        let v6223: f64 = (if self.scalar_v2241 { v5768 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5768) } else { v4 }) });
        let v6241: f64 = (v2246 * v2246);
        let v6288: f64 = (if v2258 { ((v2259 * v3332) + (v1171 * (self.scalar_v610 * v4619))) } else { (if v2254 { (((v2246 * (v6208 + v6214)) - (v2255 * ((v3350 - (v2245 * v3332)) / v3353))) / v6241) } else { v4 }) });
        let v6289: f64 = (if v2258 { v4 } else { (if v2254 { ((v6209 + v6215) / v2246) } else { v4 }) });
        let v6290: f64 = (if v2258 { ((v2259 * v3335) + (v1171 * (self.scalar_v610 * v4620))) } else { (if v2254 { (((v2246 * (v6210 + v6216)) - (v2255 * (((v1171 * (v3342 + v3346)) - (v2245 * v3335)) / v3353))) / v6241) } else { v4 }) });
        let v6291: f64 = (if v2258 { ((v2259 * v3338) + (v1171 * (self.scalar_v610 * v4621))) } else { (if v2254 { (((v2246 * (v6211 + v6217)) - (v2255 * (((v1171 * v3343) - (v2245 * v3338)) / v3353))) / v6241) } else { v4 }) });
        let v6292: f64 = (if v2258 { ((v2259 * v3341) + (v1171 * (self.scalar_v610 * v4622))) } else { (if v2254 { (((v2246 * (v6212 + v6218)) - (v2255 * (((v1171 * v3344) - (v2245 * v3341)) / v3353))) / v6241) } else { v4 }) });
        let v6293: f64 = (if v2258 { v4 } else { (if v2254 { ((v6213 + v6219) / v2246) } else { v4 }) });
        let v6377: f64 = (self.scalar_v27 * (self.scalar_v0 * v2522));
        let v6378: f64 = (self.scalar_v27 * (self.scalar_v0 * v2523));
        let v6379: f64 = (self.scalar_v27 * (self.scalar_v0 * v2524));
        let v6384: f64 = (self.scalar_v27 * (self.scalar_v0 * v3354));
        let v6385: f64 = (self.scalar_v27 * (self.scalar_v0 * v3358));
        let v6386: f64 = (self.scalar_v27 * (self.scalar_v0 * v3362));
        let v6387: f64 = (self.scalar_v27 * (self.scalar_v0 * v3366));
        let v6393: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v534 * v3697) + ((if self.scalar_v1290 { v3594 } else { (if self.scalar_v481 { (v3594 + v3611) } else { v4 }) }) + (self.scalar_v525 * v3653)))));
        let v6394: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v534 * v3698) + ((if self.scalar_v1290 { v3595 } else { (if self.scalar_v481 { (v3595 + v3615) } else { v4 }) }) + (self.scalar_v525 * v3654)))));
        let v6395: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v534 * v3699) + ((if self.scalar_v1290 { v3596 } else { v3625 }) + (self.scalar_v525 * v3655)))));
        let v6396: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v534 * v3700)));
        let v6397: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v534 * v3701)));
        let v6398: f64 = (((if self.scalar_v1294 { (self.scalar_v471 * ((self.scalar_v1295 * v3447) + (v1278 * (self.scalar_v1288 * v3447)))) } else { (if self.scalar_v1291 { v3477 } else { (if self.scalar_v481 { v3533 } else { v4 }) }) }) + (self.scalar_v445 * v3638)) + self.scalar_v6332);
        let v6402: f64 = (((v1228 * (self.scalar_v1225 * v3423)) + (v1226 * ((-v3423) * v3430))) + (v6398 - (if v1477 { v4 } else { (if v1391 { (self.scalar_v35 * (self.scalar_v268 * v3858)) } else { v4 }) })));
        let v6403: f64 = (((v1228 * (self.scalar_v1225 * v3424)) + (v1226 * ((-v3424) * v3430))) + (((v3569 + (self.scalar_v445 * v3640)) + self.scalar_v6333) - (if v1477 { v4 } else { (if v1391 { (self.scalar_v35 * (self.scalar_v268 * v3861)) } else { v4 }) })));
        let v6411: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v541 * v3397) + v6402)));
        let v6412: f64 = (self.scalar_v27 * (self.scalar_v0 * (self.scalar_v445 * v3639)));
        let v6413: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v541 * v3398) + v6403)));
        let v6414: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1294 { (self.scalar_v471 * ((v1299 * v3235) + (v1278 * (self.scalar_v1288 * v2960)))) } else { v3543 })));
        let v6415: f64 = (self.scalar_v27 * (self.scalar_v0 * (if self.scalar_v1294 { (self.scalar_v471 * ((v1299 * v3236) + (v1278 * (self.scalar_v1288 * v2961)))) } else { v3544 })));
        let v6434: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4491))));
        let v6435: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4492))));
        let v6436: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4493))));
        let v6437: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4494))));
        let v6438: f64 = (self.scalar_v27 * (self.scalar_v0 * (-((v1694 * (if v1566 { v4 } else { (if v1483 { (self.scalar_v70 * (self.scalar_v269 * v4027)) } else { v4 }) })) + (v1567 * v4495)))));
        let v6439: f64 = (self.scalar_v27 * (self.scalar_v0 * (-((v1694 * (if v1566 { v4 } else { (if v1483 { (self.scalar_v70 * (self.scalar_v269 * v4030)) } else { v4 }) })) + (v1567 * v4496)))));
        let v6440: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4497))));
        let v6441: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4498))));
        let v6442: f64 = (self.scalar_v27 * (self.scalar_v0 * (-(v1567 * v4499))));
        let v6443: f64 = (if self.scalar_v481 { v6434 } else { v4 });
        let v6444: f64 = (if self.scalar_v481 { v6435 } else { v4 });
        let v6445: f64 = (if self.scalar_v481 { v6436 } else { v4 });
        let v6446: f64 = (if self.scalar_v481 { v6437 } else { v4 });
        let v6447: f64 = (if self.scalar_v481 { v6438 } else { v4 });
        let v6448: f64 = (if self.scalar_v481 { v6439 } else { v4 });
        let v6449: f64 = (if self.scalar_v481 { v6440 } else { v4 });
        let v6450: f64 = (if self.scalar_v481 { v6441 } else { v4 });
        let v6451: f64 = (if self.scalar_v481 { v6442 } else { v4 });
        let v6452: f64 = (if self.scalar_v1290 { v6434 } else { v4 });
        let v6453: f64 = (if self.scalar_v1290 { v6435 } else { v4 });
        let v6454: f64 = (if self.scalar_v1290 { v6436 } else { v4 });
        let v6455: f64 = (if self.scalar_v1290 { v6437 } else { v4 });
        let v6456: f64 = (if self.scalar_v1290 { v6438 } else { v4 });
        let v6457: f64 = (if self.scalar_v1290 { v6439 } else { v4 });
        let v6458: f64 = (if self.scalar_v1290 { v6440 } else { v4 });
        let v6459: f64 = (if self.scalar_v1290 { v6441 } else { v4 });
        let v6460: f64 = (if self.scalar_v1290 { v6442 } else { v4 });
        let v6463: f64 = (self.scalar_v0 * (((v1714 * (self.scalar_v2363 + (self.scalar_v841 * (if v749 { (v750 * self.scalar_v2368) } else { (if v746 { (v747 * self.scalar_v2368) } else { v4 }) })))) - (v1717 * v4653)) / v4662));
        let v6466: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1717 * v4652)) / v4662)));
        let v6467: f64 = (self.scalar_v27 * (self.scalar_v0 * ((self.scalar_v0 + (self.scalar_v841 * (if v749 { (v750 * self.scalar_v2367) } else { (if v746 { (v747 * self.scalar_v2367) } else { v4 }) }))) / v1714)));
        let v6468: f64 = (self.scalar_v27 * v6463);
        let v6469: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1717 * v4654)) / v4662)));
        let v6470: f64 = (self.scalar_v27 * (self.scalar_v0 * ((-(v1717 * v4655)) / v4662)));
        let v6475: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5476)));
        let v6476: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5477)));
        let v6477: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5478)));
        let v6478: f64 = (self.scalar_v27 * (self.scalar_v0 * (-v5479)));
        let v6495: f64 = ddt_scale;
        let v6502: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2241 { v5765 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5765) } else { v4 }) }) + ((self.scalar_v1975 * v3064) + v6208))) * v6495));
        let v6503: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6209 + (if self.scalar_v2241 { v5766 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5766) } else { v4 }) }))) * v6495));
        let v6504: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2241 { v5767 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5767) } else { v4 }) }) + ((self.scalar_v1975 * v3065) + v6210))) * v6495));
        let v6505: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6211 + v6223)) * v6495));
        let v6506: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6212 + v6223)) * v6495));
        let v6507: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6213 + (if self.scalar_v2241 { v5769 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5769) } else { v4 }) }))) * v6495));
        let v6512: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (self.scalar_v1994 * ((self.scalar_v1060 * (-((-(self.scalar_v268 * v5500)) * v5507))) + (v154 * (self.scalar_v2363 - v5500)))))));
        let v6513: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (self.scalar_v1994 * ((self.scalar_v1060 * (-((-(self.scalar_v268 * v5501)) * v5507))) + (v154 * (self.scalar_v0 - v5501)))))));
        let v6526: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6214)));
        let v6527: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6215)));
        let v6528: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * (self.scalar_v2097 * v3002)) + (v2098 * v2962)) + ((self.scalar_v2005 * v3190) + v6216)))));
        let v6529: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * (self.scalar_v2097 * v3003)) + (v2098 * v2963)) + ((self.scalar_v2005 * v3191) + v6217)))));
        let v6530: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * (self.scalar_v2097 * v3004)) + (v2098 * v2958)) + ((self.scalar_v2005 * v3187) + v6218)))));
        let v6531: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6219)));
        let v6544: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6149)));
        let v6545: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { ((v2224 * self.scalar_v6131) + (v2222 * (if self.scalar_v2188 { (v5766 / self.scalar_v2082) } else { v4 }))) } else { v4 }))));
        let v6546: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { v6145 } else { v4 }))));
        let v6547: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v2216 * (self.scalar_v2008 * v4621)) } else { v4 }) + v6129)) } else { v4 }))));
        let v6548: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v2216 * (self.scalar_v2008 * v4622)) } else { v4 }) + v6129)) } else { v4 }))));
        let v6549: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * (if self.scalar_v2188 { (v5769 / self.scalar_v2082) } else { v4 })) } else { v4 }))));
        let v6554: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6550));
        let v6555: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6551));
        let v6560: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6556));
        let v6561: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6557));
        let v6571: f64 = (self.scalar_v27 * (self.scalar_v0 * (v4551 + (v1643 * v4491))));
        let v6572: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { ((v1641 * v4160) + (v1604 * v4308)) } else { v4 })) + (v1643 * v4492))));
        let v6573: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { (v1604 * v4309) } else { v4 })) + (v1643 * v4493))));
        let v6574: f64 = (self.scalar_v27 * (self.scalar_v0 * (v4551 + (v1643 * v4494))));
        let v6575: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { (v4315 + (v1604 * v4310)) } else { v4 })) + (v1643 * v4495))));
        let v6576: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { (v4324 + (v1604 * v4311)) } else { v4 })) + (v1643 * v4496))));
        let v6577: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { (v4324 + (v1604 * v4312)) } else { v4 })) + (v1643 * v4497))));
        let v6578: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { ((v1641 * v4162) + (v1604 * v4313)) } else { v4 })) + (v1643 * v4498))));
        let v6579: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1694 * (if self.scalar_v1592 { (v4324 + (v1604 * v4314)) } else { v4 })) + (v1643 * v4499))));
        let v6599: f64 = (v6495 * (self.scalar_v0 * (v5746 + (if self.scalar_v2136 { ((v2184 * v4308) + (v1641 * (if self.scalar_v2166 { v6001 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5949) / self.scalar_v617) } else { v4 }) }))) } else { v4 }))));
        let v6604: f64 = (v6495 * (self.scalar_v0 * (v5748 + (if self.scalar_v2136 { ((v2184 * v4313) + (v1641 * (if self.scalar_v2166 { v6009 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5951) / self.scalar_v617) } else { v4 }) }))) } else { v4 }))));
        let v6606: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5745 + (if self.scalar_v2136 { ((v2184 * v4307) + v6015) } else { v4 })))));
        let v6607: f64 = (self.scalar_v27 * v6599);
        let v6608: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2136 { (v2184 * v4309) } else { v4 }))));
        let v6609: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5745 + (if self.scalar_v2136 { (v6015 + (v2184 * v4310)) } else { v4 })))));
        let v6610: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5747 + (if self.scalar_v2136 { ((v2184 * v4311) + v6024) } else { v4 })))));
        let v6611: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5747 + (if self.scalar_v2136 { (v6024 + (v2184 * v4312)) } else { v4 })))));
        let v6612: f64 = (self.scalar_v27 * v6604);
        let v6613: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5747 + (if self.scalar_v2136 { (v6024 + (v2184 * v4314)) } else { v4 })))));
        let v6622: f64 = (self.scalar_v0 * (((v1694 * (if self.scalar_v1592 { (self.scalar_v14 * v4108) } else { v4108 })) + (v1594 * v4494)) + (((v1694 * (self.scalar_v457 * v3677)) + (v1375 * v4494)) + self.scalar_v6333)));
        let v6623: f64 = (self.scalar_v0 * (((v1694 * (if self.scalar_v1592 { (self.scalar_v14 * v4112) } else { v4112 })) + (v1594 * v4495)) + (((v1694 * (self.scalar_v457 * v3678)) + (v1375 * v4495)) + self.scalar_v6334)));
        let v6627: f64 = (self.scalar_v0 * (((v1694 * (if self.scalar_v1592 { (self.scalar_v14 * v4120) } else { v4120 })) + (v1594 * v4499)) + (((v1694 * (self.scalar_v457 * v3680)) + (v1375 * v4499)) + self.scalar_v6332)));
        let v6628: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1594 * v4491) + (v1375 * v4491))));
        let v6629: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1594 * v4492) + (v1375 * v4492))));
        let v6630: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1594 * v4493) + ((v1694 * (self.scalar_v457 * v3676)) + (v1375 * v4493)))));
        let v6631: f64 = (self.scalar_v27 * v6622);
        let v6632: f64 = (self.scalar_v27 * v6623);
        let v6633: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v4522 + (v1594 * v4496)) + ((v4542 + (v1375 * v4496)) + self.scalar_v6335))));
        let v6634: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v4522 + (v1594 * v4497)) + ((v4542 + (v1375 * v4497)) + self.scalar_v6335))));
        let v6635: f64 = (self.scalar_v27 * (self.scalar_v0 * ((v1594 * v4498) + (v1375 * v4498))));
        let v6636: f64 = (self.scalar_v27 * v6627);
        let v6645: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5648 + (if self.scalar_v2136 { (self.scalar_v14 * v5860) } else { v5860 })))));
        let v6646: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5649 + (if self.scalar_v2136 { (self.scalar_v14 * v5861) } else { v5861 })))));
        let v6647: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5650 + (if self.scalar_v2136 { (self.scalar_v14 * v5862) } else { v5862 })))));
        let v6648: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5651 + (if self.scalar_v2136 { (self.scalar_v14 * v5863) } else { v5863 })))));
        let v6661: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6288) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6288) } else { v4 }) }) }));
        let v6662: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6289) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6289) } else { v4 }) }) }));
        let v6663: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6290) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6290) } else { v4 }) }) }));
        let v6664: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6291) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6291) } else { v4 }) }) }));
        let v6665: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6292) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6292) } else { v4 }) }) }));
        let v6666: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6293) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6293) } else { v4 }) }) }));
        let v6667: f64 = (v2274 * v6495);

        let d2291_dn5: f64 = v6377;
        let d2291_dn6: f64 = v6378;
        let d2291_dn7: f64 = v6379;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * (v2291),
            5,
            multiplicity * (d2291_dn5),
            6,
            multiplicity * (d2291_dn6),
            7,
            multiplicity * (d2291_dn7),
        );
        let d2293_dn3: f64 = v6384;
        let d2293_dn5: f64 = v6385;
        let d2293_dn6: f64 = v6386;
        let d2293_dn7: f64 = v6387;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(3),
            multiplicity * (v2293),
            [3, 5, 6, 7],
            [d2293_dn3, d2293_dn5, d2293_dn6, d2293_dn7],
            [],
            [],
            multiplicity,
        );
        let d2295_dn3: f64 = v6393;
        let d2295_dn4: f64 = v6394;
        let d2295_dn5: f64 = v6395;
        let d2295_dn6: f64 = v6396;
        let d2295_dn7: f64 = v6396;
        let d2295_dn9: f64 = v6397;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(3),
            multiplicity * (v2295),
            [3, 4, 5, 6, 7, 9],
            [d2295_dn3, d2295_dn4, d2295_dn5, d2295_dn6, d2295_dn7, d2295_dn9],
            [],
            [],
            multiplicity,
        );
        let d2301_dn3: f64 = v6411;
        let d2301_dn4: f64 = v6412;
        let d2301_dn5: f64 = v6413;
        let d2301_dn6: f64 = v6414;
        let d2301_dn7: f64 = v6415;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2301),
            [3, 4, 5, 6, 7],
            [d2301_dn3, d2301_dn4, d2301_dn5, d2301_dn6, d2301_dn7],
            [],
            [],
            multiplicity,
        );
        let d2305_dn0: f64 = v6443;
        let d2305_dn1: f64 = v6444;
        let d2305_dn3: f64 = v6445;
        let d2305_dn4: f64 = v6446;
        let d2305_dn5: f64 = v6447;
        let d2305_dn6: f64 = v6448;
        let d2305_dn7: f64 = v6449;
        let d2305_dn8: f64 = v6450;
        let d2305_dn9: f64 = v6451;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(6),
            multiplicity * (v2305),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [d2305_dn0, d2305_dn1, d2305_dn3, d2305_dn4, d2305_dn5, d2305_dn6, d2305_dn7, d2305_dn8, d2305_dn9],
            [],
            [],
            multiplicity,
        );
        let d2306_dn0: f64 = v6452;
        let d2306_dn1: f64 = v6453;
        let d2306_dn3: f64 = v6454;
        let d2306_dn4: f64 = v6455;
        let d2306_dn5: f64 = v6456;
        let d2306_dn6: f64 = v6457;
        let d2306_dn7: f64 = v6458;
        let d2306_dn8: f64 = v6459;
        let d2306_dn9: f64 = v6460;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2306),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [d2306_dn0, d2306_dn1, d2306_dn3, d2306_dn4, d2306_dn5, d2306_dn6, d2306_dn7, d2306_dn8, d2306_dn9],
            [],
            [],
            multiplicity,
        );
        let d2308_dn3: f64 = v6466;
        let d2308_dn4: f64 = v6467;
        let d2308_dn5: f64 = v6468;
        let d2308_dn6: f64 = v6469;
        let d2308_dn7: f64 = v6470;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(5),
            multiplicity * (v2308),
            [3, 4, 5, 6, 7],
            [d2308_dn3, d2308_dn4, d2308_dn5, d2308_dn6, d2308_dn7],
            [],
            [],
            multiplicity,
        );
        let d2310_dn3: f64 = v6475;
        let d2310_dn5: f64 = v6476;
        let d2310_dn6: f64 = v6477;
        let d2310_dn7: f64 = v6478;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2310),
            [3, 5, 6, 7],
            [d2310_dn3, d2310_dn5, d2310_dn6, d2310_dn7],
            [],
            [],
            multiplicity,
        );
        let d2313_dn2: f64 = self.scalar_v6483;
        let d2313_dn3: f64 = self.scalar_v6484;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(3),
            multiplicity * (v2313),
            2,
            multiplicity * (d2313_dn2),
            3,
            multiplicity * (d2313_dn3),
        );
        let d2316_dn1: f64 = self.scalar_v6487;
        let d2316_dn4: f64 = self.scalar_v6488;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(4),
            multiplicity * (v2316),
            1,
            multiplicity * (d2316_dn1),
            4,
            multiplicity * (d2316_dn4),
        );
        let d2319_dn3: f64 = v6502;
        let d2319_dn4: f64 = v6503;
        let d2319_dn5: f64 = v6504;
        let d2319_dn6: f64 = v6505;
        let d2319_dn7: f64 = v6506;
        let d2319_dn9: f64 = v6507;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2319),
            [3, 4, 5, 6, 7, 9],
            [d2319_dn3, d2319_dn4, d2319_dn5, d2319_dn6, d2319_dn7, d2319_dn9],
            [],
            [],
            multiplicity,
        );
        let d2322_dn3: f64 = v6512;
        let d2322_dn4: f64 = v6513;
        stamper.stamp_current_node2_local(
            Some(4),
            Some(3),
            multiplicity * (v2322),
            3,
            multiplicity * (d2322_dn3),
            4,
            multiplicity * (d2322_dn4),
        );
        let d2325_dn3: f64 = v6526;
        let d2325_dn4: f64 = v6527;
        let d2325_dn5: f64 = v6528;
        let d2325_dn6: f64 = v6529;
        let d2325_dn7: f64 = v6530;
        let d2325_dn9: f64 = v6531;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            Some(7),
            multiplicity * (v2325),
            [3, 4, 5, 6, 7, 9],
            [d2325_dn3, d2325_dn4, d2325_dn5, d2325_dn6, d2325_dn7, d2325_dn9],
            [],
            [],
            multiplicity,
        );
        let d2328_dn3: f64 = v6544;
        let d2328_dn4: f64 = v6545;
        let d2328_dn5: f64 = v6546;
        let d2328_dn6: f64 = v6547;
        let d2328_dn7: f64 = v6548;
        let d2328_dn9: f64 = v6549;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(4),
            Some(5),
            multiplicity * (v2328),
            [3, 4, 5, 6, 7, 9],
            [d2328_dn3, d2328_dn4, d2328_dn5, d2328_dn6, d2328_dn7, d2328_dn9],
            [],
            [],
            multiplicity,
        );
        let d2332_dn1: f64 = v6554;
        let d2332_dn2: f64 = v6555;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2332),
            1,
            multiplicity * (d2332_dn1),
            2,
            multiplicity * (d2332_dn2),
        );
        let d2336_dn0: f64 = v6560;
        let d2336_dn1: f64 = v6561;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2336),
            0,
            multiplicity * (d2336_dn0),
            1,
            multiplicity * (d2336_dn1),
        );
        let d2338_dn0: f64 = v6571;
        let d2338_dn1: f64 = v6572;
        let d2338_dn3: f64 = v6573;
        let d2338_dn4: f64 = v6574;
        let d2338_dn5: f64 = v6575;
        let d2338_dn6: f64 = v6576;
        let d2338_dn7: f64 = v6577;
        let d2338_dn8: f64 = v6578;
        let d2338_dn9: f64 = v6579;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * (v2338),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [d2338_dn0, d2338_dn1, d2338_dn3, d2338_dn4, d2338_dn5, d2338_dn6, d2338_dn7, d2338_dn8, d2338_dn9],
            [],
            [],
            multiplicity,
        );
        let d2341_dn0: f64 = self.scalar_v6586;
        let d2341_dn1: f64 = self.scalar_v6587;
        let d2341_dn4: f64 = self.scalar_v6587;
        let d2341_dn5: f64 = self.scalar_v6587;
        let d2341_dn6: f64 = self.scalar_v6588;
        let d2341_dn7: f64 = self.scalar_v6588;
        let d2341_dn8: f64 = self.scalar_v6589;
        let d2341_dn9: f64 = self.scalar_v6588;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(0),
            Some(8),
            multiplicity * (v2341),
            [0, 1, 4, 5, 6, 7, 8, 9],
            [d2341_dn0, d2341_dn1, d2341_dn4, d2341_dn5, d2341_dn6, d2341_dn7, d2341_dn8, d2341_dn9],
            [],
            [],
            multiplicity,
        );
        let d2344_dn0: f64 = v6606;
        let d2344_dn1: f64 = v6607;
        let d2344_dn3: f64 = v6608;
        let d2344_dn4: f64 = v6606;
        let d2344_dn5: f64 = v6609;
        let d2344_dn6: f64 = v6610;
        let d2344_dn7: f64 = v6611;
        let d2344_dn8: f64 = v6612;
        let d2344_dn9: f64 = v6613;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(1),
            Some(8),
            multiplicity * (v2344),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [d2344_dn0, d2344_dn1, d2344_dn3, d2344_dn4, d2344_dn5, d2344_dn6, d2344_dn7, d2344_dn8, d2344_dn9],
            [],
            [],
            multiplicity,
        );
        let d2347_dn0: f64 = v6628;
        let d2347_dn1: f64 = v6629;
        let d2347_dn3: f64 = v6630;
        let d2347_dn4: f64 = v6631;
        let d2347_dn5: f64 = v6632;
        let d2347_dn6: f64 = v6633;
        let d2347_dn7: f64 = v6634;
        let d2347_dn8: f64 = v6635;
        let d2347_dn9: f64 = v6636;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(9),
            multiplicity * (v2347),
            [0, 1, 3, 4, 5, 6, 7, 8, 9],
            [d2347_dn0, d2347_dn1, d2347_dn3, d2347_dn4, d2347_dn5, d2347_dn6, d2347_dn7, d2347_dn8, d2347_dn9],
            [],
            [],
            multiplicity,
        );
        let d2350_dn4: f64 = v6645;
        let d2350_dn5: f64 = v6646;
        let d2350_dn6: f64 = v6647;
        let d2350_dn7: f64 = v6647;
        let d2350_dn9: f64 = v6648;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            Some(9),
            multiplicity * (v2350),
            [4, 5, 6, 7, 9],
            [d2350_dn4, d2350_dn5, d2350_dn6, d2350_dn7, d2350_dn9],
            [],
            [],
            multiplicity,
        );
        let d2354_dn8: f64 = self.scalar_v6653;
        let d2354_dn9: f64 = self.scalar_v6654;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (v2354),
            8,
            multiplicity * (d2354_dn8),
            9,
            multiplicity * (d2354_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(8),
            Some(9),
            0,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            0,
            v4,
        );
        let d2358_dn6: f64 = self.scalar_v6659;
        let d2358_dn9: f64 = self.scalar_v6660;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(6),
            multiplicity * (v2358),
            6,
            multiplicity * (d2358_dn6),
            9,
            multiplicity * (d2358_dn9),
        );
        stamper.stamp_potential_branch_local(
            Some(9),
            Some(6),
            1,
            multiplicity,
        );
        stamper.stamp_potential_const_local(
            1,
            v4,
        );
        stamper.stamp_current_const_local(
            Some(10),
            None,
            multiplicity * (v4),
        );
        let d2359_dn10: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (v2359),
            10,
            multiplicity * (d2359_dn10),
        );
        let d2361_dn3: f64 = v6661;
        let d2361_dn4: f64 = v6662;
        let d2361_dn5: f64 = v6663;
        let d2361_dn6: f64 = v6664;
        let d2361_dn7: f64 = v6665;
        let d2361_dn9: f64 = v6666;
        let d2361_dn10: f64 = v6667;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(5),
            Some(3),
            multiplicity * (v2361),
            [3, 4, 5, 6, 7, 9, 10],
            [d2361_dn3, d2361_dn4, d2361_dn5, d2361_dn6, d2361_dn7, d2361_dn9, d2361_dn10],
            [],
            [],
            multiplicity,
        );
        let d2362_dn10: f64 = v2253;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(5),
            multiplicity * (v2362),
            10,
            multiplicity * (d2362_dn10),
        );
        let d2359_dn10: f64 = v1;
        stamper.stamp_current_node1_local(
            Some(7),
            Some(3),
            multiplicity * (v2359),
            10,
            multiplicity * (d2359_dn10),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(4),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(3),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(4),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(5),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(8),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(9),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(9),
            Some(6),
            multiplicity * (v4),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(6),
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
        let v31: f64 = 2.0;
        let v154: f64 = 3.0;
        let v395: f64 = 0.5;
        let v407: f64 = 4.0;
        let v433: f64 = 6.0;
        let v670: f64 = ctx.node_voltage(nodes[5]);
        let v671: f64 = ctx.node_voltage(nodes[6]);
        let v673: f64 = (self.scalar_v0 * (v670 - v671));
        let v674: f64 = ctx.node_voltage(nodes[7]);
        let v676: f64 = (self.scalar_v0 * (v670 - v674));
        let v677: f64 = ctx.node_voltage(nodes[3]);
        let v679: f64 = (self.scalar_v0 * (v670 - v677));
        let v680: f64 = ctx.node_voltage(nodes[4]);
        let v682: f64 = (self.scalar_v0 * (v680 - v677));
        let v684: f64 = (self.scalar_v0 * (v680 - v670));
        let v686: f64 = (self.scalar_v0 * (v671 - v674));
        let v690: f64 = ctx.node_voltage(nodes[1]);
        let v697: f64 = (self.scalar_v0 * (v690 - ctx.node_voltage(nodes[0])));
        let v698: f64 = ctx.node_voltage(nodes[9]);
        let v706: f64 = (((v676 + v684) - v686) - (self.scalar_v0 * (v698 - v671)));
        let v711: f64 = (v697 + ((v706 + ((self.scalar_v0 * (v690 - v680)) + (-v697))) - (self.scalar_v0 * (ctx.node_voltage(nodes[8]) - v698))));
        let v712: f64 = (self.scalar_v105 * v676);
        let v714: bool = (v712 < self.scalar_v713);
        let v715: f64 = ((v712) as f64).exp();
        let v717: bool = (!v714);
        let v719: f64 = (if v717 { self.scalar_v718 } else { v4 });
        let v724: f64 = (self.scalar_v105 * v679);
        let v725: f64 = (v724 / self.scalar_v355);
        let v726: bool = (v725 < self.scalar_v713);
        let v727: f64 = ((v725) as f64).exp();
        let v729: bool = (!v726);
        let v730: f64 = (if v729 { self.scalar_v718 } else { v719 });
        let v734: f64 = (if v729 { (v730 * (v1 + (v725 - self.scalar_v713))) } else { (if v726 { v727 } else { v4 }) });
        let v735: f64 = (self.scalar_v105 * v706);
        let v736: bool = (v735 < self.scalar_v713);
        let v737: f64 = ((v735) as f64).exp();
        let v739: bool = (!v736);
        let v740: f64 = (if v739 { self.scalar_v718 } else { v730 });
        let v744: f64 = (if v739 { (v740 * (v1 + (v735 - self.scalar_v713))) } else { (if v736 { v737 } else { v4 }) });
        let v755: f64 = (self.scalar_v105 * v711);
        let v756: bool = (v755 < self.scalar_v713);
        let v757: f64 = ((v755) as f64).exp();
        let v759: bool = (!v756);
        let v760: f64 = (if v759 { self.scalar_v718 } else { (if (!((self.scalar_v105 * v684) < self.scalar_v713)) { self.scalar_v718 } else { v740 }) });
        let v764: f64 = (if v759 { (v760 * (v1 + (v755 - self.scalar_v713))) } else { (if v756 { v757 } else { v4 }) });
        let v766: f64 = (self.scalar_v105 * (v711 - self.scalar_v203));
        let v767: bool = (v766 < self.scalar_v713);
        let v768: f64 = ((v766) as f64).exp();
        let v770: bool = (!v767);
        let v771: f64 = (if v770 { self.scalar_v718 } else { v760 });
        let v777: f64 = (self.scalar_v105 * (v706 - self.scalar_v203));
        let v778: bool = (v777 < self.scalar_v713);
        let v779: f64 = ((v777) as f64).exp();
        let v781: bool = (!v778);
        let v782: f64 = (if v781 { self.scalar_v718 } else { v771 });
        let v788: f64 = (self.scalar_v105 * (v676 - self.scalar_v203));
        let v789: bool = (v788 < self.scalar_v713);
        let v790: f64 = ((v788) as f64).exp();
        let v792: bool = (!v789);
        let v793: f64 = (if v792 { self.scalar_v718 } else { v782 });
        let v797: f64 = (if v792 { (v793 * (v1 + (v788 - self.scalar_v713))) } else { (if v789 { v790 } else { v4 }) });
        let v799: f64 = (self.scalar_v105 * (v673 - self.scalar_v203));
        let v800: bool = (v799 < self.scalar_v713);
        let v801: f64 = ((v799) as f64).exp();
        let v803: bool = (!v800);
        let v804: f64 = (if v803 { self.scalar_v718 } else { v793 });
        let v808: f64 = (if v803 { (v804 * (v1 + (v799 - self.scalar_v713))) } else { (if v800 { v801 } else { v4 }) });
        let v811: f64 = (((v1 + (v407 * v797))) as f64).sqrt();
        let v814: f64 = (((v1 + (v407 * v808))) as f64).sqrt();
        let v815: f64 = (v31 * v808);
        let v816: f64 = (v1 + v814);
        let v817: f64 = (v815 / v816);
        let v819: bool = (v817 < self.scalar_v818);
        let v820: f64 = (if v819 { self.scalar_v818 } else { v817 });
        let v822: f64 = (v1 + v811);
        let v823: f64 = (v822 / v816);
        let v826: f64 = (self.scalar_v103 * ((v811 - v814) - ((v823) as f64).ln()));
        let v828: f64 = ((v686 + v826) / self.scalar_v323);
        let v829: bool = (v828 > v4);
        let v830: f64 = 100.0;
        let v831: bool = (v673 < v830);
        let v832: bool = (v829 && v831);
        let v835: bool = (v829 && (!v831));
        let v837: f64 = (v1 + (v673 - v830));
        let v843: f64 = (self.scalar_v323 * (v395 * v828));
        let v845: f64 = (v1 + (self.scalar_v105 * v843));
        let v850: f64 = (if v829 { ((self.scalar_v203 + (self.scalar_v841 * ((v845) as f64).ln())) - (if v835 { (v830 + ((v837) as f64).ln()) } else { (if v832 { v673 } else { v4 }) })) } else { v4 });
        let v853: f64 = (if v829 { self.scalar_v852 } else { v4 });
        let v855: f64 = (if v829 { (v853 * v853) } else { 1e-6 });
        let v858: bool = (v850 < v4);
        let v859: bool = (v829 && v858);
        let v860: f64 = (v395 * v855);
        let v862: f64 = (((v855 + (if v829 { (v850 * v850) } else { self.scalar_v393 }))) as f64).sqrt();
        let v863: f64 = (v862 - v850);
        let v867: bool = (v829 && (!v858));
        let v870: f64 = (if v867 { (v395 * (v850 + v862)) } else { (if v859 { (v860 / v863) } else { v4 }) });
        let v874: f64 = (v870 + self.scalar_v873);
        let v875: f64 = (v870 * v874);
        let v878: f64 = (self.scalar_v872 * (v870 + self.scalar_v876));
        let v880: f64 = (if v829 { (v875 / v878) } else { v4 });
        let v882: f64 = (if v829 { (v828 / v880) } else { v4 });
        let v886: f64 = (if v829 { ((v882 - v1) / self.scalar_v884) } else { self.scalar_v365 });
        let v887: bool = (v882 < v1);
        let v888: bool = (v829 && v887);
        let v889: f64 = ((v886) as f64).exp();
        let v890: f64 = (v1 + v889);
        let v896: bool = (v829 && (!v887));
        let v898: f64 = (((-v886)) as f64).exp();
        let v899: f64 = (v1 + v898);
        let v912: f64 = (if v829 { ((if v896 { (v882 + (self.scalar_v884 * ((v899) as f64).ln())) } else { (if v888 { (v1 + (self.scalar_v884 * ((v890) as f64).ln())) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v914: f64 = (if v829 { (v870 / self.scalar_v873) } else { v4 });
        let v915: f64 = (v407 * v912);
        let v916: f64 = (v914 * v915);
        let v917: f64 = (v1 + v914);
        let v920: f64 = (((v1 + (v916 * v917))) as f64).sqrt();
        let v921: f64 = (v1 + v920);
        let v922: f64 = (v31 * v912);
        let v923: f64 = (v917 * v922);
        let v925: f64 = (if v829 { (v921 / v923) } else { v4 });
        let v927: f64 = (v820 * v925);
        let v928: f64 = ((v1 - v925) + v927);
        let v929: f64 = (v1 + v927);
        let v931: f64 = (if v829 { (v928 / v929) } else { v4 });
        let v934: f64 = (if v829 { (self.scalar_v105 * (v843 * v931)) } else { v4 });
        let v937: f64 = (v1 + (v820 + v934));
        let v940: f64 = (if v829 { ((v31 * v934) + (v820 * v937)) } else { v4 });
        let v943: f64 = (if v829 { (v395 * (v934 - v1)) } else { v4 });
        let v947: bool = (v934 >= v1);
        let v948: bool = (v829 && v947);
        let v949: f64 = (((if v829 { (v940 + (v943 * v943)) } else { v4 })) as f64).sqrt();
        let v953: bool = (v829 && (!v947));
        let v954: f64 = (v949 - v943);
        let v956: f64 = (if v953 { (v940 / v954) } else { (if v948 { (v943 + v949) } else { v4 }) });
        let v959: bool = (v829 && (v956 < self.scalar_v957));
        let v960: f64 = (if v959 { self.scalar_v957 } else { v956 });
        let v961: f64 = (v1 + v960);
        let v970: f64 = (if v829 { (self.scalar_v967 * (v828 - self.scalar_v871)) } else { v4 });
        let v977: f64 = ((((if v829 { (v828 * self.scalar_v972) } else { v4 }) + (v970 * v970))) as f64).sqrt();
        let v986: bool = (v829 && self.scalar_v985);
        let v987: f64 = (v31 * v828);
        let v988: f64 = (v828 + v880);
        let v993: f64 = (v828 * self.scalar_v871);
        let v994: f64 = (v828 + self.scalar_v871);
        let v999: bool = (!v829);
        let v1000: f64 = (v31 * v797);
        let v1003: f64 = (if v999 { (if v717 { (v719 * (v1 + (v712 - self.scalar_v713))) } else { (if v714 { v715 } else { v4 }) }) } else { (if v829 { ((v960 * v961) * self.scalar_v964) } else { v4 }) });
        let v1014: bool = ((((v686) as f64).abs() < self.scalar_v1006) || (((v826) as f64).abs() < (self.scalar_v1010 * (v811 + v814))));
        let v1015: bool = (v999 && v1014);
        let v1016: f64 = (v820 + (if v999 { (v1000 / v822) } else { v960 }));
        let v1018: f64 = (if v1015 { (v395 * v1016) } else { v4 });
        let v1019: f64 = (v1 + v1018);
        let v1023: bool = (v999 && (!v1014));
        let v1025: f64 = ((v676 + v826) - v673);
        let v1029: f64 = (if v999 { self.scalar_v983 } else { (if v986 { (self.scalar_v244 * (0.1 + (v987 / v988))) } else { (if (v829 && self.scalar_v981) { self.scalar_v983 } else { v4 }) }) });
        let v1033: f64 = (if v999 { (v1 - ((if v999 { v828 } else { (if v829 { (v993 / v994) } else { v4 }) }) / self.scalar_v871)) } else { (if v829 { (self.scalar_v871 / v994) } else { v4 }) });
        let v1040: f64 = ((v679 - self.scalar_v1037) / self.scalar_v1038);
        let v1041: bool = (v679 < self.scalar_v1037);
        let v1042: f64 = ((v1040) as f64).exp();
        let v1043: f64 = (v1 + v1042);
        let v1048: bool = (!v1041);
        let v1050: f64 = (((-v1040)) as f64).exp();
        let v1051: f64 = (v1 + v1050);
        let v1055: f64 = (if v1048 { (self.scalar_v1037 - (self.scalar_v1038 * ((v1051) as f64).ln())) } else { (if v1041 { (v679 - (self.scalar_v1038 * ((v1043) as f64).ln())) } else { v4 }) });
        let v1057: f64 = (v1 - (self.scalar_v268 * v1055));
        let v1059: f64 = f64::powf(v1057, self.scalar_v1058);
        let v1065: f64 = ((self.scalar_v1060 * (v1 - v1059)) + (v154 * (v679 - v1055)));
        let v1076: f64 = (if self.scalar_v1075 { v676 } else { (if self.scalar_v1071 { (v673 + (if v999 { v686 } else { (if v829 { (v970 + v977) } else { v4 }) })) } else { (if self.scalar_v1067 { v673 } else { v4 }) }) });
        let v1084: f64 = (v1076 - self.scalar_v1083);
        let v1085: f64 = (v1084 / v1029);
        let v1086: bool = (v1076 < self.scalar_v1083);
        let v1087: f64 = ((v1085) as f64).exp();
        let v1088: f64 = (v1 + v1087);
        let v1089: f64 = ((v1088) as f64).ln();
        let v1093: bool = (!v1086);
        let v1095: f64 = (((-v1085)) as f64).exp();
        let v1096: f64 = (v1 + v1095);
        let v1097: f64 = ((v1096) as f64).ln();
        let v1100: f64 = (if v1093 { (self.scalar_v1083 - (v1029 * v1097)) } else { (if v1086 { (v1076 - (v1029 * v1089)) } else { v4 }) });
        let v1102: f64 = f64::powf(v1033, self.scalar_v1101);
        let v1106: f64 = (v1 - (v1100 / self.scalar_v244));
        let v1107: f64 = f64::powf(v1106, self.scalar_v1103);
        let v1111: f64 = (self.scalar_v1079 * v1102);
        let v1112: f64 = (v1076 - v1100);
        let v1117: f64 = ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - (v1102 * v1107))) + (v1111 * v1112))) + (self.scalar_v285 * v673));
        let v1120: f64 = (v734 * self.scalar_v1119);
        let v1122: f64 = (((v1 + v1120)) as f64).sqrt();
        let v1123: f64 = (v1 + v1122);
        let v1124: f64 = (v1120 / v1123);
        let v1126: f64 = f64::powf(v1003, self.scalar_v1125);
        let v1127: f64 = (self.scalar_v1119 * v1126);
        let v1129: f64 = (((v1 + v1127)) as f64).sqrt();
        let v1130: f64 = (v1 + v1129);
        let v1131: f64 = (v1127 / v1130);
        let v1134: f64 = (v1 + (v1065 / self.scalar_v594));
        let v1136: f64 = (v1134 + (v1117 / self.scalar_v591));
        let v1147: f64 = (((if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v1134)) } else { v4 })) as f64).exp();
        let v1148: f64 = (((if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v1117) / self.scalar_v591))) } else { v4 })) as f64).exp();
        let v1154: f64 = (if self.scalar_v1138 { ((v1147 - v1148) / self.scalar_v1152) } else { (if self.scalar_v1132 { v1136 } else { v4 }) });
        let v1155: f64 = 0.010000000000000002;
        let v1156: f64 = (v1154 * v1154);
        let v1157: bool = (v1154 < v4);
        let v1158: f64 = 0.005000000000000001;
        let v1160: f64 = (((v1155 + v1156)) as f64).sqrt();
        let v1161: f64 = (v1160 - v1154);
        let v1164: bool = (!v1157);
        let v1167: f64 = (if v1164 { (v395 * (v1154 + v1160)) } else { (if v1157 { (v1158 / v1161) } else { v4 }) });
        let v1170: f64 = (v1 + (v395 * (v1124 + v1131)));
        let v1171: f64 = (v1167 * v1170);
        let v1174: f64 = (v1126 * self.scalar_v1173);
        let v1175: f64 = (self.scalar_v420 * v734);
        let v1177: f64 = ((v1175 - v1174) / v1171);
        let v1178: f64 = 0.0001;
        let v1179: f64 = (v679 / v1178);
        let v1180: bool = (v679 < v4);
        let v1181: f64 = ((v1179) as f64).exp();
        let v1182: f64 = (v1 + v1181);
        let v1186: bool = (!v1180);
        let v1188: f64 = (((-v1179)) as f64).exp();
        let v1189: f64 = (v1 + v1188);
        let v1193: f64 = (if v1186 { (v679 + (v1178 * ((v1189) as f64).ln())) } else { (if v1180 { (v1178 * ((v1182) as f64).ln()) } else { v4 }) });
        let v1230: f64 = (v724 / self.scalar_v462);
        let v1231: bool = (v1230 < self.scalar_v713);
        let v1232: f64 = ((v1230) as f64).exp();
        let v1234: bool = (!v1231);
        let v1235: f64 = (if v1234 { self.scalar_v718 } else { (if (!((v1193 / self.scalar_v1194) < self.scalar_v713)) { self.scalar_v718 } else { v804 }) });
        let v1264: f64 = (if (self.scalar_v481 && (!(((v1177 / self.scalar_v420) - 1000.0) < 40.0))) { 2.3538526683702e17 } else { (if (self.scalar_v481 && (!((self.scalar_v105 * (v679 - self.scalar_v267)) < self.scalar_v713))) { self.scalar_v718 } else { v1235 }) });
        let v1304: f64 = (self.scalar_v105 * v682);
        let v1305: f64 = (v1304 / self.scalar_v473);
        let v1306: bool = (v1305 < self.scalar_v713);
        let v1307: f64 = ((v1305) as f64).exp();
        let v1309: bool = (!v1306);
        let v1310: f64 = (if v1309 { self.scalar_v718 } else { v1264 });
        let v1314: f64 = (if v1309 { (v1310 * (v1 + (v1305 - self.scalar_v713))) } else { (if v1306 { v1307 } else { (if v1234 { (v1235 * (v1 + (v1230 - self.scalar_v713))) } else { (if v1231 { v1232 } else { v1193 }) }) }) });
        let v1340: f64 = (v724 / self.scalar_v434);
        let v1341: bool = (v1340 < self.scalar_v713);
        let v1342: f64 = ((v1340) as f64).exp();
        let v1344: bool = (!v1341);
        let v1345: f64 = (if v1344 { self.scalar_v718 } else { (if (self.scalar_v481 && (!((self.scalar_v105 * (v682 - self.scalar_v267)) < self.scalar_v713))) { self.scalar_v718 } else { v1310 }) });
        let v1352: f64 = (v1304 / self.scalar_v517);
        let v1353: bool = (v1352 < self.scalar_v713);
        let v1354: f64 = ((v1352) as f64).exp();
        let v1356: bool = (!v1353);
        let v1357: f64 = (if v1356 { self.scalar_v718 } else { v1345 });
        let v1361: f64 = (if v1356 { (v1357 * (v1 + (v1352 - self.scalar_v713))) } else { (if v1353 { v1354 } else { (if v1344 { (v1345 * (v1 + (v1340 - self.scalar_v713))) } else { (if v1341 { v1342 } else { v1314 }) }) }) });
        let v1364: f64 = (v735 / self.scalar_v447);
        let v1365: bool = (v1364 < self.scalar_v713);
        let v1366: f64 = ((v1364) as f64).exp();
        let v1368: bool = (!v1365);
        let v1369: f64 = (if v1368 { self.scalar_v718 } else { v1357 });
        let v1376: f64 = (v1304 / self.scalar_v527);
        let v1377: bool = (v1376 < self.scalar_v713);
        let v1378: f64 = ((v1376) as f64).exp();
        let v1380: bool = (!v1377);
        let v1381: f64 = (if v1380 { self.scalar_v718 } else { v1369 });
        let v1385: f64 = (if v1380 { (v1381 * (v1 + (v1376 - self.scalar_v713))) } else { (if v1377 { v1378 } else { (if v1368 { (v1369 * (v1 + (v1364 - self.scalar_v713))) } else { (if v1365 { v1366 } else { v1361 }) }) }) });
        let v1391: bool = (v1180 && self.scalar_v1390);
        let v1408: f64 = (if v1391 { (self.scalar_v268 * v679) } else { self.scalar_v588 });
        let v1410: f64 = 1e-30;
        let v1428: f64 = (f64::powf(((((v1408 * v1408) + v1410)) as f64).sqrt(), self.scalar_v1414) * ((self.scalar_v32 * (self.scalar_v1417 - ((v154 * v1408) * self.scalar_v1419))) - ((v1408 * (v433 * v1408)) * (v1408 + self.scalar_v1419))));
        let v1429: f64 = 0.16666666666666666;
        let v1436: f64 = (if v1391 { ((self.scalar_v554 * (self.scalar_v34 * v679)) / (self.scalar_v131 * (if v1391 { (v1428 * v1429) } else { v4 }))) } else { v1408 });
        let v1437: f64 = -0.001;
        let v1446: f64 = (if ((v1391 && (v1436 < v1437)) && (!(v1436 < self.scalar_v713))) { self.scalar_v718 } else { (if (v1391 && (!((self.scalar_v554 * (v1 - (self.scalar_v34 / (v31 * v1059)))) < self.scalar_v713))) { self.scalar_v718 } else { v1381 }) });
        let v1483: bool = (self.scalar_v1481 && (v673 < v4));
        let v1484: f64 = (self.scalar_v269 * v673);
        let v1503: f64 = (if v1483 { v1484 } else { self.scalar_v566 });
        let v1521: f64 = (f64::powf((((v1410 + (v1503 * v1503))) as f64).sqrt(), self.scalar_v1507) * ((self.scalar_v67 * (self.scalar_v1510 - ((v154 * v1503) * self.scalar_v1512))) - ((v1503 * (v433 * v1503)) * (v1503 + self.scalar_v1512))));
        let v1528: f64 = (if v1483 { ((self.scalar_v576 * (self.scalar_v69 * v673)) / (self.scalar_v153 * (if v1483 { (v1429 * v1521) } else { v4 }))) } else { v1503 });
        let v1537: f64 = (if ((v1483 && (v1528 < v1437)) && (!(v1528 < self.scalar_v713))) { self.scalar_v718 } else { (if (v1483 && (!((self.scalar_v576 * (v1 - (self.scalar_v69 / (v31 * (if v1483 { f64::powf((v1 - v1484), self.scalar_v1103) } else { v4 }))))) < self.scalar_v713))) { self.scalar_v718 } else { v1446 }) });
        let v1568: f64 = (v744 * self.scalar_v1119);
        let v1569: f64 = (v407 * (if v781 { (v782 * (v1 + (v777 - self.scalar_v713))) } else { (if v778 { v779 } else { v4 }) }));
        let v1570: f64 = (v1568 - self.scalar_v1119);
        let v1572: f64 = (((v1 + v1568)) as f64).sqrt();
        let v1573: f64 = (v1 + v1572);
        let v1576: f64 = (((v1 + v1569)) as f64).sqrt();
        let v1577: f64 = (v1 + v1576);
        let v1598: f64 = (self.scalar_v1596 * (v764 - v1));
        let v1601: f64 = (((v1 + (v764 * self.scalar_v1583))) as f64).sqrt();
        let v1602: f64 = (v1 + v1601);
        let v1616: f64 = (if self.scalar_v1606 { (v711 - self.scalar_v1614) } else { v4 });
        let v1621: bool = (v1616 < v4);
        let v1622: bool = (self.scalar_v1606 && v1621);
        let v1625: f64 = (((self.scalar_v1618 + (if self.scalar_v1606 { (v1616 * v1616) } else { v1156 }))) as f64).sqrt();
        let v1626: f64 = (v1625 - v1616);
        let v1630: bool = (self.scalar_v1606 && (!v1621));
        let v1633: f64 = (if v1630 { (v395 * (v1616 + v1625)) } else { (if v1622 { (self.scalar_v1623 / v1626) } else { v4 }) });
        let v1636: f64 = (v1633 + (self.scalar_v1609 + (self.scalar_v311 * (if self.scalar_v1592 { (v1598 / v1602) } else { v4 }))));
        let v1641: f64 = (if self.scalar_v1640 { v1 } else { (if self.scalar_v1606 { (v1633 / v1636) } else { v1 }) });
        let v1700: bool = (v1136 < v4);
        let v1702: f64 = (((v1155 + (v1136 * v1136))) as f64).sqrt();
        let v1703: f64 = (v1702 - v1136);
        let v1706: bool = (!v1700);
        let v1709: f64 = (if v1706 { (v395 * (v1136 + v1702)) } else { (if v1700 { (v1158 / v1703) } else { v4 }) });
        let v1719: bool = (v1177 > v4);
        let v1723: bool = (v673 < self.scalar_v1722);
        let v1726: f64 = ((-v1177) / self.scalar_v1725);
        let v1727: bool = (v1726 < self.scalar_v713);
        let v1729: bool = (v1723 && (v1719 && self.scalar_v1721));
        let v1734: bool = (v1729 && (!v1727));
        let v1735: f64 = (if v1734 { self.scalar_v718 } else { v1537 });
        let v1740: f64 = (self.scalar_v1722 - v673);
        let v1746: f64 = (self.scalar_v1743 * f64::powf((if v1729 { ((if v1734 { (v1735 * (v1 + (v1726 - self.scalar_v713))) } else { (if (v1727 && v1729) { ((v1726) as f64).exp() } else { v4 }) }) * v1740) } else { v4 }), self.scalar_v1744));
        let v1870: bool = (v1723 && (self.scalar_v1866 && ((v1719 && self.scalar_v1765) && self.scalar_v1867)));
        let v1879: f64 = (if v1870 { (f64::powf(v1740, self.scalar_v1744) * f64::powf((v1 - (v1177 / (v1177 + self.scalar_v1872))), self.scalar_v1876)) } else { v4 });
        let v1882: bool = (self.scalar_v1786 && v1870);
        let v1886: f64 = (if v1882 { ((v1177 - self.scalar_v1883) / self.scalar_v1872) } else { v4 });
        let v1890: f64 = (if v1882 { ((v1886 - v1) / self.scalar_v1888) } else { ((v679 - self.scalar_v1207) / 0.001) });
        let v1891: bool = (v1886 < v1);
        let v1907: f64 = (if (v1882 && (!v1891)) { (v1886 + (self.scalar_v1888 * (((v1 + (((-v1890)) as f64).exp())) as f64).ln())) } else { (if (v1882 && v1891) { (v1 + (self.scalar_v1888 * (((v1 + ((v1890) as f64).exp())) as f64).ln())) } else { v4 }) });
        let v1919: f64 = (if (v1870 && (!((self.scalar_v1743 * (if v1882 { (v1879 * f64::powf(v1907, self.scalar_v1908)) } else { (if (self.scalar_v1783 && v1870) { v1879 } else { v4 }) })) < self.scalar_v713))) { self.scalar_v718 } else { (if (v1729 && (!(v1746 < self.scalar_v713))) { self.scalar_v718 } else { v1735 }) });
        let v1978: f64 = ((v682 - self.scalar_v1037) / self.scalar_v1038);
        let v1979: bool = (v682 < self.scalar_v1037);
        let v1980: f64 = ((v1978) as f64).exp();
        let v1981: f64 = (v1 + v1980);
        let v1986: bool = (!v1979);
        let v1988: f64 = (((-v1978)) as f64).exp();
        let v1989: f64 = (v1 + v1988);
        let v1993: f64 = (if v1986 { (self.scalar_v1037 - (self.scalar_v1038 * ((v1989) as f64).ln())) } else { (if v1979 { (v682 - (self.scalar_v1038 * ((v1981) as f64).ln())) } else { v4 }) });
        let v1996: f64 = (v1 - (self.scalar_v268 * v1993));
        let v2009: f64 = (v1124 * self.scalar_v2008);
        let v2010: f64 = (v1709 * v2009);
        let v2011: f64 = (v1131 * self.scalar_v2008);
        let v2012: f64 = (v1709 * v2011);
        let v2014: f64 = ((v706 - self.scalar_v1083) / self.scalar_v983);
        let v2015: bool = (v706 < self.scalar_v1083);
        let v2016: f64 = ((v2014) as f64).exp();
        let v2017: f64 = (v1 + v2016);
        let v2022: bool = (!v2015);
        let v2024: f64 = (((-v2014)) as f64).exp();
        let v2025: f64 = (v1 + v2024);
        let v2029: f64 = (if v2022 { (self.scalar_v1083 - (self.scalar_v983 * ((v2025) as f64).ln())) } else { (if v2015 { (v706 - (self.scalar_v983 * ((v2017) as f64).ln())) } else { v4 }) });
        let v2031: f64 = (v1 - (v2029 / self.scalar_v244));
        let v2044: f64 = (self.scalar_v14 * ((self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - f64::powf(v2031, self.scalar_v1103))) + (self.scalar_v1079 * (v706 - v2029)))) + (self.scalar_v285 * v706))) * self.scalar_v2042));
        let v2046: f64 = ((v711 - self.scalar_v1083) / self.scalar_v983);
        let v2047: bool = (v711 < self.scalar_v1083);
        let v2048: f64 = ((v2046) as f64).exp();
        let v2049: f64 = (v1 + v2048);
        let v2054: bool = (!v2047);
        let v2056: f64 = (((-v2046)) as f64).exp();
        let v2057: f64 = (v1 + v2056);
        let v2061: f64 = (if v2054 { (self.scalar_v1083 - (self.scalar_v983 * ((v2057) as f64).ln())) } else { (if v2047 { (v711 - (self.scalar_v983 * ((v2049) as f64).ln())) } else { v4 }) });
        let v2063: f64 = (v1 - (v2061 / self.scalar_v244));
        let v2075: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (v1 - f64::powf(v2063, self.scalar_v1103))) + (self.scalar_v1079 * (v711 - v2061)))) + (self.scalar_v285 * v711)))));
        let v2083: f64 = (v679 / self.scalar_v2082);
        let v2084: bool = (v2083 < self.scalar_v713);
        let v2085: f64 = ((v2083) as f64).exp();
        let v2087: bool = (!v2084);
        let v2088: f64 = (if v2087 { self.scalar_v718 } else { v1919 });
        let v2093: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * (v1 + (v2083 - self.scalar_v713))) } else { (if v2084 { v2085 } else { v1385 }) }));
        let v2098: f64 = ((if v1023 { (v826 / v1025) } else { (if v1015 { (v1018 / v1019) } else { v931 }) }) * self.scalar_v2097);
        let v2099: f64 = (v31 + v1016);
        let v2113: f64 = (self.scalar_v105 * ((v706 - self.scalar_v224) / self.scalar_v2111));
        let v2114: bool = (v2113 < self.scalar_v713);
        let v2116: bool = (v2114 && self.scalar_v2115);
        let v2117: f64 = ((v2113) as f64).exp();
        let v2120: bool = (self.scalar_v2115 && (!v2114));
        let v2121: f64 = (if v2120 { self.scalar_v718 } else { v2088 });
        let v2127: f64 = (v744 * self.scalar_v2126);
        let v2130: f64 = (((v1 + (v407 * (if v2120 { (v2121 * (v1 + (v2113 - self.scalar_v713))) } else { (if v2116 { v2117 } else { v4 }) })))) as f64).sqrt();
        let v2131: f64 = (v1 + v2130);
        let v2133: f64 = (if self.scalar_v2115 { (v2127 / v2131) } else { (if self.scalar_v2102 { ((self.scalar_v2103 * (((v1570 / v1573) * self.scalar_v2007) + ((v1569 / v1577) * self.scalar_v2096))) / self.scalar_v617) } else { v4 }) });
        let v2141: f64 = (if self.scalar_v2139 { (v764 * self.scalar_v1119) } else { v4 });
        let v2142: f64 = (v2141 - self.scalar_v1119);
        let v2144: f64 = (((v1 + v2141)) as f64).sqrt();
        let v2145: f64 = (v1 + v2144);
        let v2149: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * (v1 + (v766 - self.scalar_v713))) } else { (if v767 { v768 } else { v4 }) })) } else { v4 });
        let v2151: f64 = (((v1 + v2149)) as f64).sqrt();
        let v2152: f64 = (v1 + v2151);
        let v2164: f64 = (self.scalar_v105 * (v711 - self.scalar_v224));
        let v2165: bool = (v2164 < self.scalar_v713);
        let v2167: bool = (v2165 && self.scalar_v2166);
        let v2168: f64 = ((v2164) as f64).exp();
        let v2171: bool = (self.scalar_v2166 && (!v2165));
        let v2172: f64 = (if v2171 { self.scalar_v718 } else { v2121 });
        let v2178: f64 = (v764 * self.scalar_v2177);
        let v2181: f64 = (((v1 + (v407 * (if v2171 { (v2172 * (v1 + (v2164 - self.scalar_v713))) } else { (if v2167 { v2168 } else { v4 }) })))) as f64).sqrt();
        let v2182: f64 = (v1 + v2181);
        let v2184: f64 = (if self.scalar_v2166 { (v2178 / v2182) } else { (if self.scalar_v2139 { ((self.scalar_v2156 * ((self.scalar_v2007 * (if self.scalar_v2139 { (v2142 / v2145) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (v2149 / v2152) } else { v4 })))) / self.scalar_v617) } else { v4 }) });
        let v2192: f64 = (if self.scalar_v2188 { (f64::powf(v1057, self.scalar_v2189) - v154) } else { v4 });
        let v2193: f64 = (if self.scalar_v2188 { v1040 } else { v4 });
        let v2194: bool = (v2193 < v4);
        let v2195: bool = (self.scalar_v2188 && v2194);
        let v2196: f64 = ((v2193) as f64).exp();
        let v2197: f64 = (v1 + v2196);
        let v2201: bool = (self.scalar_v2188 && (!v2194));
        let v2203: f64 = (((-v2193)) as f64).exp();
        let v2204: f64 = (v1 + v2203);
        let v2206: f64 = (if v2201 { (v2203 / v2204) } else { (if v2195 { (v1 / v2197) } else { v4 }) });
        let v2213: f64 = ((self.scalar_v105 * v1120) / self.scalar_v355);
        let v2214: f64 = (v395 / v1122);
        let v2216: f64 = (if self.scalar_v2188 { (v2213 * v2214) } else { v4 });
        let v2217: f64 = (v1709 * self.scalar_v2008);
        let v2222: f64 = (v684 * 0.2);
        let v2224: f64 = ((if self.scalar_v2188 { (v2093 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { (v154 + (v2192 * v2206)) } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { (v2216 * v2217) } else { v4 })));
        let v2233: f64 = (if self.scalar_v2188 { (v2010 + (v2093 * self.scalar_v2227)) } else { v4 });
        let v2242: f64 = (if self.scalar_v2241 { v2010 } else { (if self.scalar_v2188 { (v2233 * self.scalar_v2238) } else { v4 }) });
        let v2243: f64 = (if self.scalar_v2241 { v2012 } else { (if self.scalar_v2188 { (v2012 + (v2233 * self.scalar_v2234)) } else { v4 }) });
        let v2245: f64 = (v1174 + v1175);
        let v2246: f64 = (v2245 / v1171);
        let v2254: bool = (v2246 > v4);
        let v2255: f64 = (v2242 + v2243);
        let v2258: bool = (!v2254);
        let v2259: f64 = (self.scalar_v610 * v1709);
        let v2261: f64 = (if v2258 { (v1171 * v2259) } else { (if v2254 { (v2255 / v2246) } else { v4 }) });
        let v2274: f64 = (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (v2261 * self.scalar_v2269) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v2261) } else { v4 }) }) });
        let v2318: f64 = 0.0;
        let v2319: f64 = (self.scalar_v27 * v2318);
        let v2321: f64 = 0.0;
        let v2322: f64 = (self.scalar_v27 * v2321);
        let v2324: f64 = 0.0;
        let v2325: f64 = (self.scalar_v27 * v2324);
        let v2327: f64 = 0.0;
        let v2328: f64 = (self.scalar_v27 * v2327);
        let v2331: f64 = 0.0;
        let v2332: f64 = (self.scalar_v27 * v2331);
        let v2335: f64 = 0.0;
        let v2336: f64 = (self.scalar_v27 * v2335);
        let v2343: f64 = 0.0;
        let v2344: f64 = (self.scalar_v27 * v2343);
        let v2349: f64 = 0.0;
        let v2350: f64 = (self.scalar_v27 * v2349);
        let v2360: f64 = 0.0;
        let v2361: f64 = (v2274 * v2360);
        let v2385: f64 = (if v729 { (v730 * self.scalar_v2377) } else { (if v726 { (v727 * self.scalar_v2377) } else { v4 }) });
        let v2386: f64 = (if v729 { (v730 * self.scalar_v2378) } else { (if v726 { (v727 * self.scalar_v2378) } else { v4 }) });
        let v2401: f64 = (if v739 { (v740 * self.scalar_v2367) } else { (if v736 { (v737 * self.scalar_v2367) } else { v4 }) });
        let v2402: f64 = (if v739 { (v740 * self.scalar_v2387) } else { (if v736 { (v737 * self.scalar_v2387) } else { v4 }) });
        let v2403: f64 = (if v739 { (v740 * self.scalar_v2388) } else { (if v736 { (v737 * self.scalar_v2388) } else { v4 }) });
        let v2404: f64 = (if v739 { (v740 * self.scalar_v2368) } else { (if v736 { (v737 * self.scalar_v2368) } else { v4 }) });
        let v2426: f64 = (if v759 { (v760 * self.scalar_v2387) } else { (if v756 { (v757 * self.scalar_v2387) } else { v4 }) });
        let v2427: f64 = (if v759 { (v760 * self.scalar_v2413) } else { (if v756 { (v757 * self.scalar_v2413) } else { v4 }) });
        let v2428: f64 = (if v759 { (v760 * self.scalar_v2388) } else { (if v756 { (v757 * self.scalar_v2388) } else { v4 }) });
        let v2429: f64 = (if v759 { (v760 * self.scalar_v2368) } else { (if v756 { (v757 * self.scalar_v2368) } else { v4 }) });
        let v2468: f64 = (if v792 { (v793 * self.scalar_v2367) } else { (if v789 { (v790 * self.scalar_v2367) } else { v4 }) });
        let v2469: f64 = (if v792 { (v793 * self.scalar_v2368) } else { (if v789 { (v790 * self.scalar_v2368) } else { v4 }) });
        let v2476: f64 = (if v803 { (v804 * self.scalar_v2367) } else { (if v800 { (v801 * self.scalar_v2367) } else { v4 }) });
        let v2477: f64 = (if v803 { (v804 * self.scalar_v2368) } else { (if v800 { (v801 * self.scalar_v2368) } else { v4 }) });
        let v2480: f64 = (v31 * v811);
        let v2481: f64 = ((v407 * v2468) / v2480);
        let v2482: f64 = ((v407 * v2469) / v2480);
        let v2485: f64 = (v31 * v814);
        let v2486: f64 = ((v407 * v2476) / v2485);
        let v2487: f64 = ((v407 * v2477) / v2485);
        let v2493: f64 = (v816 * v816);
        let v2499: f64 = (if v819 { v4 } else { (((v816 * (v31 * v2476)) - (v815 * v2486)) / v2493) });
        let v2500: f64 = (if v819 { v4 } else { (((v816 * (v31 * v2477)) - (v815 * v2487)) / v2493) });
        let v2517: f64 = (self.scalar_v103 * ((v2481 - v2486) - ((((v816 * v2481) - (v822 * v2486)) / v2493) / v823)));
        let v2518: f64 = (self.scalar_v103 * ((-v2487) - (((-(v822 * v2487)) / v2493) / v823)));
        let v2519: f64 = (self.scalar_v103 * (v2482 - ((v2482 / v816) / v823)));
        let v2521: f64 = (self.scalar_v2363 + v2519);
        let v2522: f64 = (v2517 / self.scalar_v323);
        let v2523: f64 = ((self.scalar_v0 + v2518) / self.scalar_v323);
        let v2524: f64 = (v2521 / self.scalar_v323);
        let v2534: f64 = (self.scalar_v323 * (v395 * v2522));
        let v2535: f64 = (self.scalar_v323 * (v395 * v2523));
        let v2536: f64 = (self.scalar_v323 * (v395 * v2524));
        let v2548: f64 = (if v829 { ((self.scalar_v841 * ((self.scalar_v105 * v2534) / v845)) - (if v835 { (self.scalar_v0 / v837) } else { (if v832 { self.scalar_v0 } else { v4 }) })) } else { v4 });
        let v2549: f64 = (if v829 { ((self.scalar_v841 * ((self.scalar_v105 * v2535) / v845)) - (if v835 { (self.scalar_v2363 / v837) } else { (if v832 { self.scalar_v2363 } else { v4 }) })) } else { v4 });
        let v2550: f64 = (if v829 { (self.scalar_v841 * ((self.scalar_v105 * v2536) / v845)) } else { v4 });
        let v2551: f64 = (v850 * v2548);
        let v2553: f64 = (v850 * v2549);
        let v2555: f64 = (v850 * v2550);
        let v2560: f64 = (v31 * v862);
        let v2561: f64 = ((if v829 { (v2551 + v2551) } else { v4 }) / v2560);
        let v2562: f64 = ((if v829 { (v2553 + v2553) } else { v4 }) / v2560);
        let v2563: f64 = ((if v829 { (v2555 + v2555) } else { v4 }) / v2560);
        let v2569: f64 = (v863 * v863);
        let v2586: f64 = (if v867 { (v395 * (v2548 + v2561)) } else { (if v859 { ((-(v860 * (v2561 - v2548))) / v2569) } else { v4 }) });
        let v2587: f64 = (if v867 { (v395 * (v2549 + v2562)) } else { (if v859 { ((-(v860 * (v2562 - v2549))) / v2569) } else { v4 }) });
        let v2588: f64 = (if v867 { (v395 * (v2550 + v2563)) } else { (if v859 { ((-(v860 * (v2563 - v2550))) / v2569) } else { v4 }) });
        let v2604: f64 = (v878 * v878);
        let v2614: f64 = (if v829 { (((v878 * ((v874 * v2586) + (v870 * v2586))) - (v875 * (self.scalar_v872 * v2586))) / v2604) } else { v4 });
        let v2615: f64 = (if v829 { (((v878 * ((v874 * v2587) + (v870 * v2587))) - (v875 * (self.scalar_v872 * v2587))) / v2604) } else { v4 });
        let v2616: f64 = (if v829 { (((v878 * ((v874 * v2588) + (v870 * v2588))) - (v875 * (self.scalar_v872 * v2588))) / v2604) } else { v4 });
        let v2620: f64 = (v880 * v880);
        let v2630: f64 = (if v829 { (((v880 * v2522) - (v828 * v2614)) / v2620) } else { v4 });
        let v2631: f64 = (if v829 { (((v880 * v2523) - (v828 * v2615)) / v2620) } else { v4 });
        let v2632: f64 = (if v829 { (((v880 * v2524) - (v828 * v2616)) / v2620) } else { v4 });
        let v2636: f64 = (if v829 { (v2630 / self.scalar_v884) } else { v4 });
        let v2637: f64 = (if v829 { (v2631 / self.scalar_v884) } else { v4 });
        let v2638: f64 = (if v829 { (v2632 / self.scalar_v884) } else { v4 });
        let v2672: f64 = (if v829 { ((if v896 { (v2630 + (self.scalar_v884 * ((v898 * (-v2636)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2636) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2673: f64 = (if v829 { ((if v896 { (v2631 + (self.scalar_v884 * ((v898 * (-v2637)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2637) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2674: f64 = (if v829 { ((if v896 { (v2632 + (self.scalar_v884 * ((v898 * (-v2638)) / v899))) } else { (if v888 { (self.scalar_v884 * ((v889 * v2638) / v890)) } else { v4 }) }) / self.scalar_v910) } else { v4 });
        let v2678: f64 = (if v829 { (v2586 / self.scalar_v873) } else { v4 });
        let v2679: f64 = (if v829 { (v2587 / self.scalar_v873) } else { v4 });
        let v2680: f64 = (if v829 { (v2588 / self.scalar_v873) } else { v4 });
        let v2702: f64 = (v31 * v920);
        let v2720: f64 = ((v923 * (((v917 * ((v915 * v2678) + (v914 * (v407 * v2672)))) + (v916 * v2678)) / v2702)) - (v921 * ((v922 * v2678) + (v917 * (v31 * v2672)))));
        let v2721: f64 = (v923 * v923);
        let v2725: f64 = ((v923 * (((v917 * ((v915 * v2679) + (v914 * (v407 * v2673)))) + (v916 * v2679)) / v2702)) - (v921 * ((v922 * v2679) + (v917 * (v31 * v2673)))));
        let v2729: f64 = ((v923 * (((v917 * ((v915 * v2680) + (v914 * (v407 * v2674)))) + (v916 * v2680)) / v2702)) - (v921 * ((v922 * v2680) + (v917 * (v31 * v2674)))));
        let v2731: f64 = (if v829 { (v2720 / v2721) } else { v4 });
        let v2732: f64 = (if v829 { (v2725 / v2721) } else { v4 });
        let v2733: f64 = (if v829 { (v2729 / v2721) } else { v4 });
        let v2739: f64 = ((v925 * v2499) + (v820 * v2731));
        let v2742: f64 = ((v925 * v2500) + (v820 * v2732));
        let v2743: f64 = (v820 * v2733);
        let v2750: f64 = (v929 * v929);
        let v2760: f64 = (if v829 { (((v929 * ((-v2731) + v2739)) - (v928 * v2739)) / v2750) } else { v4 });
        let v2761: f64 = (if v829 { (((v929 * ((-v2732) + v2742)) - (v928 * v2742)) / v2750) } else { v4 });
        let v2762: f64 = (if v829 { (((v929 * ((-v2733) + v2743)) - (v928 * v2743)) / v2750) } else { v4 });
        let v2775: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2534) + (v843 * v2760))) } else { v4 });
        let v2776: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2535) + (v843 * v2761))) } else { v4 });
        let v2777: f64 = (if v829 { (self.scalar_v105 * ((v931 * v2536) + (v843 * v2762))) } else { v4 });
        let v2793: f64 = (if v829 { ((v31 * v2775) + ((v937 * v2499) + (v820 * (v2499 + v2775)))) } else { v4 });
        let v2794: f64 = (if v829 { ((v31 * v2776) + ((v937 * v2500) + (v820 * (v2500 + v2776)))) } else { v4 });
        let v2795: f64 = (if v829 { ((v31 * v2777) + (v820 * v2777)) } else { v4 });
        let v2799: f64 = (if v829 { (v395 * v2775) } else { v4 });
        let v2800: f64 = (if v829 { (v395 * v2776) } else { v4 });
        let v2801: f64 = (if v829 { (v395 * v2777) } else { v4 });
        let v2802: f64 = (v943 * v2799);
        let v2804: f64 = (v943 * v2800);
        let v2806: f64 = (v943 * v2801);
        let v2814: f64 = (v31 * v949);
        let v2815: f64 = ((if v829 { (v2793 + (v2802 + v2802)) } else { v4 }) / v2814);
        let v2816: f64 = ((if v829 { (v2794 + (v2804 + v2804)) } else { v4 }) / v2814);
        let v2817: f64 = ((if v829 { (v2795 + (v2806 + v2806)) } else { v4 }) / v2814);
        let v2830: f64 = (v954 * v954);
        let v2843: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2793) - (v940 * (v2815 - v2799))) / v2830) } else { (if v948 { (v2799 + v2815) } else { v4 }) }) });
        let v2844: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2794) - (v940 * (v2816 - v2800))) / v2830) } else { (if v948 { (v2800 + v2816) } else { v4 }) }) });
        let v2845: f64 = (if v959 { v4 } else { (if v953 { (((v954 * v2795) - (v940 * (v2817 - v2801))) / v2830) } else { (if v948 { (v2801 + v2817) } else { v4 }) }) });
        let v2864: f64 = (if v829 { (self.scalar_v967 * v2522) } else { v4 });
        let v2865: f64 = (if v829 { (self.scalar_v967 * v2523) } else { v4 });
        let v2866: f64 = (if v829 { (self.scalar_v967 * v2524) } else { v4 });
        let v2873: f64 = (v970 * v2864);
        let v2875: f64 = (v970 * v2865);
        let v2877: f64 = (v970 * v2866);
        let v2882: f64 = (v31 * v977);
        let v2901: f64 = (v988 * v988);
        let v2917: f64 = (self.scalar_v871 * v2522);
        let v2918: f64 = (self.scalar_v871 * v2523);
        let v2919: f64 = (self.scalar_v871 * v2524);
        let v2923: f64 = (v994 * v994);
        let v2950: f64 = (v822 * v822);
        let v2958: f64 = (if v999 { (((v822 * (v31 * v2469)) - (v1000 * v2482)) / v2950) } else { v2845 });
        let v2959: f64 = (if v999 { (if v717 { (v719 * self.scalar_v2367) } else { (if v714 { (v715 * self.scalar_v2367) } else { v4 }) }) } else { (if v829 { (self.scalar_v964 * ((v961 * v2843) + (v960 * v2843))) } else { v4 }) });
        let v2961: f64 = (if v999 { (if v717 { (v719 * self.scalar_v2368) } else { (if v714 { (v715 * self.scalar_v2368) } else { v4 }) }) } else { (if v829 { (self.scalar_v964 * ((v961 * v2845) + (v960 * v2845))) } else { v4 }) });
        let v2962: f64 = (v2499 + (if v999 { (((v822 * (v31 * v2468)) - (v1000 * v2481)) / v2950) } else { v2843 }));
        let v2963: f64 = (v2500 + (if v999 { v4 } else { v2844 }));
        let v2967: f64 = (if v1015 { (v395 * v2962) } else { v4 });
        let v2968: f64 = (if v1015 { (v395 * v2963) } else { v4 });
        let v2969: f64 = (if v1015 { (v395 * v2958) } else { v4 });
        let v2973: f64 = (v1019 * v1019);
        let v2992: f64 = (v1025 * v1025);
        let v3002: f64 = (if v1023 { (((v1025 * v2517) - (v826 * ((self.scalar_v0 + v2517) - self.scalar_v0))) / v2992) } else { (if v1015 { (((v1019 * v2967) - (v1018 * v2967)) / v2973) } else { v2760 }) });
        let v3003: f64 = (if v1023 { (((v1025 * v2518) - (v826 * (v2518 - self.scalar_v2363))) / v2992) } else { (if v1015 { (((v1019 * v2968) - (v1018 * v2968)) / v2973) } else { v2761 }) });
        let v3008: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2522)) - (v987 * (v2522 + v2614))) / v2901)) } else { v4 }) });
        let v3009: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2523)) - (v987 * (v2523 + v2615))) / v2901)) } else { v4 }) });
        let v3010: f64 = (if v999 { v4 } else { (if v986 { (self.scalar_v244 * (((v988 * (v31 * v2524)) - (v987 * (v2524 + v2616))) / v2901)) } else { v4 }) });
        let v3020: f64 = (if v999 { (-((if v999 { v2522 } else { (if v829 { (((v994 * v2917) - (v993 * v2522)) / v2923) } else { v4 }) }) / self.scalar_v871)) } else { (if v829 { ((-v2917) / v2923) } else { v4 }) });
        let v3021: f64 = (if v999 { (-((if v999 { v2523 } else { (if v829 { (((v994 * v2918) - (v993 * v2523)) / v2923) } else { v4 }) }) / self.scalar_v871)) } else { (if v829 { ((-v2918) / v2923) } else { v4 }) });
        let v3022: f64 = (if v999 { (-((if v999 { v2524 } else { (if v829 { (((v994 * v2919) - (v993 * v2524)) / v2923) } else { v4 }) }) / self.scalar_v871)) } else { (if v829 { ((-v2919) / v2923) } else { v4 }) });
        let v3045: f64 = (if v1048 { (-(self.scalar_v1038 * ((v1050 * self.scalar_v3035) / v1051))) } else { (if v1041 { (self.scalar_v2363 - (self.scalar_v1038 * ((v1042 * self.scalar_v3023) / v1043))) } else { v4 }) });
        let v3046: f64 = (if v1048 { (-(self.scalar_v1038 * ((v1050 * self.scalar_v3036) / v1051))) } else { (if v1041 { (self.scalar_v0 - (self.scalar_v1038 * ((v1042 * self.scalar_v3024) / v1043))) } else { v4 }) });
        let v3049: f64 = (-(self.scalar_v268 * v3045));
        let v3050: f64 = (-(self.scalar_v268 * v3046));
        let v3053: f64 = (self.scalar_v1058 * f64::powf(v1057, self.scalar_v3051));
        let v3064: f64 = ((self.scalar_v1060 * (-(v3049 * v3053))) + (v154 * (self.scalar_v2363 - v3045)));
        let v3065: f64 = ((self.scalar_v1060 * (-(v3050 * v3053))) + (v154 * (self.scalar_v0 - v3046)));
        let v3070: f64 = (if self.scalar_v1071 { (self.scalar_v0 + (if v999 { v4 } else { (if v829 { (v2864 + (((if v829 { (self.scalar_v972 * v2522) } else { v4 }) + (v2873 + v2873)) / v2882)) } else { v4 }) })) } else { self.scalar_v3066 });
        let v3071: f64 = (if self.scalar_v1071 { (self.scalar_v2363 + (if v999 { self.scalar_v0 } else { (if v829 { (v2865 + (((if v829 { (self.scalar_v972 * v2523) } else { v4 }) + (v2875 + v2875)) / v2882)) } else { v4 }) })) } else { self.scalar_v3067 });
        let v3073: f64 = (if self.scalar_v1075 { self.scalar_v0 } else { v3070 });
        let v3074: f64 = (if self.scalar_v1075 { v4 } else { v3071 });
        let v3075: f64 = (if self.scalar_v1075 { self.scalar_v2363 } else { (if self.scalar_v1071 { (if v999 { self.scalar_v2363 } else { (if v829 { (v2866 + (((if v829 { (self.scalar_v972 * v2524) } else { v4 }) + (v2877 + v2877)) / v2882)) } else { v4 }) }) } else { v4 }) });
        let v3079: f64 = (v1029 * v1029);
        let v3080: f64 = (((v1029 * v3073) - (v1084 * v3008)) / v3079);
        let v3084: f64 = (((v1029 * v3074) - (v1084 * v3009)) / v3079);
        let v3088: f64 = (((v1029 * v3075) - (v1084 * v3010)) / v3079);
        let v3131: f64 = (if v1093 { (-((v1097 * v3008) + (v1029 * ((v1095 * (-v3080)) / v1096)))) } else { (if v1086 { (v3073 - ((v1089 * v3008) + (v1029 * ((v1087 * v3080) / v1088)))) } else { v4 }) });
        let v3132: f64 = (if v1093 { (-((v1097 * v3009) + (v1029 * ((v1095 * (-v3084)) / v1096)))) } else { (if v1086 { (v3074 - ((v1089 * v3009) + (v1029 * ((v1087 * v3084) / v1088)))) } else { v4 }) });
        let v3133: f64 = (if v1093 { (-((v1097 * v3010) + (v1029 * ((v1095 * (-v3088)) / v1096)))) } else { (if v1086 { (v3075 - ((v1089 * v3010) + (v1029 * ((v1087 * v3088) / v1088)))) } else { v4 }) });
        let v3136: f64 = (self.scalar_v1101 * f64::powf(v1033, self.scalar_v3134));
        let v3137: f64 = (v3020 * v3136);
        let v3138: f64 = (v3021 * v3136);
        let v3139: f64 = (v3022 * v3136);
        let v3148: f64 = (self.scalar_v1103 * f64::powf(v1106, self.scalar_v3146));
        let v3182: f64 = ((self.scalar_v1104 * (-((v1107 * v3137) + (v1102 * ((-(v3131 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3137)) + (v1111 * (v3073 - v3131))));
        let v3183: f64 = ((self.scalar_v1104 * (-((v1107 * v3138) + (v1102 * ((-(v3132 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3138)) + (v1111 * (v3074 - v3132))));
        let v3184: f64 = ((self.scalar_v1104 * (-((v1107 * v3139) + (v1102 * ((-(v3133 / self.scalar_v244)) * v3148))))) + ((v1112 * (self.scalar_v1079 * v3139)) + (v1111 * (v3075 - v3133))));
        let v3187: f64 = (self.scalar_v1078 * v3184);
        let v3190: f64 = ((self.scalar_v1078 * v3182) + self.scalar_v3188);
        let v3191: f64 = ((self.scalar_v1078 * v3183) + self.scalar_v3189);
        let v3192: f64 = (self.scalar_v1119 * v2385);
        let v3193: f64 = (self.scalar_v1119 * v2386);
        let v3194: f64 = (v31 * v1122);
        let v3195: f64 = (v3192 / v3194);
        let v3196: f64 = (v3193 / v3194);
        let v3200: f64 = (v1123 * v1123);
        let v3201: f64 = (((v1123 * v3192) - (v1120 * v3195)) / v3200);
        let v3205: f64 = (((v1123 * v3193) - (v1120 * v3196)) / v3200);
        let v3208: f64 = (self.scalar_v1125 * f64::powf(v1003, self.scalar_v3206));
        let v3209: f64 = (v2959 * v3208);
        let v3210: f64 = ((if v999 { v4 } else { (if v829 { (self.scalar_v964 * ((v961 * v2844) + (v960 * v2844))) } else { v4 }) }) * v3208);
        let v3211: f64 = (v2961 * v3208);
        let v3212: f64 = (self.scalar_v1119 * v3209);
        let v3213: f64 = (self.scalar_v1119 * v3210);
        let v3214: f64 = (self.scalar_v1119 * v3211);
        let v3215: f64 = (v31 * v1129);
        let v3222: f64 = (v1130 * v1130);
        let v3223: f64 = (((v1130 * v3212) - (v1127 * (v3212 / v3215))) / v3222);
        let v3227: f64 = (((v1130 * v3213) - (v1127 * (v3213 / v3215))) / v3222);
        let v3231: f64 = (((v1130 * v3214) - (v1127 * (v3214 / v3215))) / v3222);
        let v3232: f64 = (v3064 / self.scalar_v594);
        let v3233: f64 = (v3065 / self.scalar_v594);
        let v3235: f64 = (v3191 / self.scalar_v591);
        let v3236: f64 = (v3187 / self.scalar_v591);
        let v3237: f64 = (v3233 + (v3190 / self.scalar_v591));
        let v3272: f64 = (((v1147 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v3233)) } else { v4 })) - (v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3190) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152);
        let v3275: f64 = (if self.scalar_v1138 { ((v1147 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * v3232)) } else { v4 })) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3232 } else { v4 }) });
        let v3276: f64 = (if self.scalar_v1138 { v3272 } else { (if self.scalar_v1132 { v3237 } else { v4 }) });
        let v3277: f64 = (if self.scalar_v1138 { ((-(v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3191) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3235 } else { v4 }) });
        let v3278: f64 = (if self.scalar_v1138 { ((-(v1148 * (if self.scalar_v1138 { (self.scalar_v105 * (self.scalar_v645 * ((-v3187) / self.scalar_v591))) } else { v4 }))) / self.scalar_v1152) } else { (if self.scalar_v1132 { v3236 } else { v4 }) });
        let v3279: f64 = (v1154 * v3275);
        let v3280: f64 = (v3279 + v3279);
        let v3281: f64 = (v1154 * v3276);
        let v3282: f64 = (v3281 + v3281);
        let v3283: f64 = (v1154 * v3277);
        let v3284: f64 = (v3283 + v3283);
        let v3285: f64 = (v1154 * v3278);
        let v3286: f64 = (v3285 + v3285);
        let v3287: f64 = (v31 * v1160);
        let v3288: f64 = (v3280 / v3287);
        let v3289: f64 = (v3282 / v3287);
        let v3290: f64 = (v3284 / v3287);
        let v3291: f64 = (v3286 / v3287);
        let v3298: f64 = (v1161 * v1161);
        let v3332: f64 = ((v1170 * (if v1164 { (v395 * (v3275 + v3288)) } else { (if v1157 { ((-(v1158 * (v3288 - v3275))) / v3298) } else { v4 }) })) + (v1167 * (v395 * v3201)));
        let v3335: f64 = ((v1170 * (if v1164 { (v395 * (v3276 + v3289)) } else { (if v1157 { ((-(v1158 * (v3289 - v3276))) / v3298) } else { v4 }) })) + (v1167 * (v395 * (v3205 + v3223))));
        let v3338: f64 = ((v1170 * (if v1164 { (v395 * (v3277 + v3290)) } else { (if v1157 { ((-(v1158 * (v3290 - v3277))) / v3298) } else { v4 }) })) + (v1167 * (v395 * v3227)));
        let v3341: f64 = ((v1170 * (if v1164 { (v395 * (v3278 + v3291)) } else { (if v1157 { ((-(v1158 * (v3291 - v3278))) / v3298) } else { v4 }) })) + (v1167 * (v395 * v3231)));
        let v3353: f64 = (v1171 * v1171);
        let v3443: f64 = (if v1231 { (v1232 * self.scalar_v3439) } else { (if v1186 { (self.scalar_v2363 + (v1178 * ((v1188 * self.scalar_v3377) / v1189))) } else { (if v1180 { (v1178 * ((v1181 * self.scalar_v3367) / v1182)) } else { v4 }) }) });
        let v3444: f64 = (if v1231 { (v1232 * self.scalar_v3440) } else { (if v1186 { (self.scalar_v0 + (v1178 * ((v1188 * self.scalar_v3378) / v1189))) } else { (if v1180 { (v1178 * ((v1181 * self.scalar_v3368) / v1182)) } else { v4 }) }) });
        let v3638: f64 = (if v1344 { (v1345 * self.scalar_v3629) } else { (if v1341 { (v1342 * self.scalar_v3629) } else { (if v1309 { (v1310 * self.scalar_v3572) } else { (if v1306 { (v1307 * self.scalar_v3572) } else { (if v1234 { (v1235 * self.scalar_v3439) } else { v3443 }) }) }) }) });
        let v3650: f64 = (if v1353 { v4 } else { (if v1344 { (v1345 * self.scalar_v3630) } else { (if v1341 { (v1342 * self.scalar_v3630) } else { (if v1309 { v4 } else { (if v1306 { v4 } else { (if v1234 { (v1235 * self.scalar_v3440) } else { v3444 }) }) }) }) }) });
        let v3654: f64 = (if v1356 { (v1357 * self.scalar_v3645) } else { (if v1353 { (v1354 * self.scalar_v3645) } else { (if v1344 { v4 } else { (if v1341 { v4 } else { (if v1309 { (v1310 * self.scalar_v3573) } else { (if v1306 { (v1307 * self.scalar_v3573) } else { v4 }) }) }) }) }) });
        let v3697: f64 = (if v1380 { (v1381 * self.scalar_v3686) } else { (if v1377 { (v1378 * self.scalar_v3686) } else { (if v1368 { v4 } else { (if v1365 { v4 } else { (if v1356 { (v1357 * self.scalar_v3644) } else { (if v1353 { (v1354 * self.scalar_v3644) } else { v3638 }) }) }) }) }) });
        let v4039: f64 = (self.scalar_v1119 * v2401);
        let v4040: f64 = (self.scalar_v1119 * v2402);
        let v4041: f64 = (self.scalar_v1119 * v2403);
        let v4042: f64 = (self.scalar_v1119 * v2404);
        let v4043: f64 = (v407 * (if v781 { (v782 * self.scalar_v2367) } else { (if v778 { (v779 * self.scalar_v2367) } else { v4 }) }));
        let v4044: f64 = (v407 * (if v781 { (v782 * self.scalar_v2387) } else { (if v778 { (v779 * self.scalar_v2387) } else { v4 }) }));
        let v4045: f64 = (v407 * (if v781 { (v782 * self.scalar_v2388) } else { (if v778 { (v779 * self.scalar_v2388) } else { v4 }) }));
        let v4046: f64 = (v407 * (if v781 { (v782 * self.scalar_v2368) } else { (if v778 { (v779 * self.scalar_v2368) } else { v4 }) }));
        let v4047: f64 = (v31 * v1572);
        let v4055: f64 = (v1573 * v1573);
        let v4069: f64 = (v31 * v1576);
        let v4077: f64 = (v1577 * v1577);
        let v4137: f64 = (v31 * v1601);
        let v4145: f64 = (v1602 * v1602);
        let v4167: f64 = (v1616 * self.scalar_v4163);
        let v4168: f64 = (v4167 + v4167);
        let v4169: f64 = (v1616 * self.scalar_v4164);
        let v4171: f64 = (v1616 * self.scalar_v4165);
        let v4172: f64 = (v4171 + v4171);
        let v4173: f64 = (v1616 * self.scalar_v4166);
        let v4183: f64 = (v31 * v1625);
        let v4184: f64 = ((if self.scalar_v1606 { v4168 } else { v4 }) / v4183);
        let v4185: f64 = ((if self.scalar_v1606 { (v4169 + v4169) } else { v4 }) / v4183);
        let v4186: f64 = ((if self.scalar_v1606 { v4 } else { v3280 }) / v4183);
        let v4187: f64 = ((if self.scalar_v1606 { v4168 } else { v3282 }) / v4183);
        let v4188: f64 = ((if self.scalar_v1606 { v4172 } else { v3284 }) / v4183);
        let v4189: f64 = ((if self.scalar_v1606 { v4172 } else { v3286 }) / v4183);
        let v4190: f64 = ((if self.scalar_v1606 { (v4173 + v4173) } else { v4 }) / v4183);
        let v4191: f64 = ((if self.scalar_v1606 { v4172 } else { v4 }) / v4183);
        let v4201: f64 = (v1626 * v1626);
        let v4247: f64 = (if v1630 { (v395 * (self.scalar_v4163 + v4184)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4184 - self.scalar_v4163))) / v4201) } else { v4 }) });
        let v4248: f64 = (if v1630 { (v395 * (self.scalar_v4164 + v4185)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4185 - self.scalar_v4164))) / v4201) } else { v4 }) });
        let v4249: f64 = (if v1630 { (v395 * v4186) } else { (if v1622 { ((-(self.scalar_v1623 * v4186)) / v4201) } else { v4 }) });
        let v4250: f64 = (if v1630 { (v395 * (self.scalar_v4163 + v4187)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4187 - self.scalar_v4163))) / v4201) } else { v4 }) });
        let v4251: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4188)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4188 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4252: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4189)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4189 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4253: f64 = (if v1630 { (v395 * (self.scalar_v4166 + v4190)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4190 - self.scalar_v4166))) / v4201) } else { v4 }) });
        let v4254: f64 = (if v1630 { (v395 * (self.scalar_v4165 + v4191)) } else { (if v1622 { ((-(self.scalar_v1623 * (v4191 - self.scalar_v4165))) / v4201) } else { v4 }) });
        let v4255: f64 = (self.scalar_v311 * (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2426)) - (v1598 * ((self.scalar_v1583 * v2426) / v4137))) / v4145) } else { v4 }));
        let v4257: f64 = (self.scalar_v311 * (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2428)) - (v1598 * ((self.scalar_v1583 * v2428) / v4137))) / v4145) } else { v4 }));
        let v4269: f64 = (v1636 * v1636);
        let v4273: f64 = ((v1636 * v4248) - (v1633 * (v4248 + (self.scalar_v311 * (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2427)) - (v1598 * ((self.scalar_v1583 * v2427) / v4137))) / v4145) } else { v4 })))));
        let v4293: f64 = ((v1636 * v4253) - (v1633 * (v4253 + (self.scalar_v311 * (if self.scalar_v1592 { (((v1602 * (self.scalar_v1596 * v2429)) - (v1598 * ((self.scalar_v1583 * v2429) / v4137))) / v4145) } else { v4 })))));
        let v4577: f64 = (v1136 * v3232);
        let v4579: f64 = (v1136 * v3237);
        let v4581: f64 = (v1136 * v3235);
        let v4583: f64 = (v1136 * v3236);
        let v4585: f64 = (v31 * v1702);
        let v4586: f64 = ((v4577 + v4577) / v4585);
        let v4587: f64 = ((v4579 + v4579) / v4585);
        let v4588: f64 = ((v4581 + v4581) / v4585);
        let v4589: f64 = ((v4583 + v4583) / v4585);
        let v4596: f64 = (v1703 * v1703);
        let v4619: f64 = (if v1706 { (v395 * (v3232 + v4586)) } else { (if v1700 { ((-(v1158 * (v4586 - v3232))) / v4596) } else { v4 }) });
        let v4620: f64 = (if v1706 { (v395 * (v3237 + v4587)) } else { (if v1700 { ((-(v1158 * (v4587 - v3237))) / v4596) } else { v4 }) });
        let v4621: f64 = (if v1706 { (v395 * (v3235 + v4588)) } else { (if v1700 { ((-(v1158 * (v4588 - v3235))) / v4596) } else { v4 }) });
        let v4622: f64 = (if v1706 { (v395 * (v3236 + v4589)) } else { (if v1700 { ((-(v1158 * (v4589 - v3236))) / v4596) } else { v4 }) });
        let v5500: f64 = (if v1986 { (-(self.scalar_v1038 * ((v1988 * self.scalar_v3035) / v1989))) } else { (if v1979 { (self.scalar_v2363 - (self.scalar_v1038 * ((v1980 * self.scalar_v3023) / v1981))) } else { v4 }) });
        let v5501: f64 = (if v1986 { (-(self.scalar_v1038 * ((v1988 * self.scalar_v3036) / v1989))) } else { (if v1979 { (self.scalar_v0 - (self.scalar_v1038 * ((v1980 * self.scalar_v3024) / v1981))) } else { v4 }) });
        let v5507: f64 = (self.scalar_v1058 * f64::powf(v1996, self.scalar_v3051));
        let v5529: f64 = ((v2009 * v4619) + (v1709 * (self.scalar_v2008 * v3201)));
        let v5532: f64 = ((v2009 * v4620) + (v1709 * (self.scalar_v2008 * v3205)));
        let v5533: f64 = (v2009 * v4621);
        let v5534: f64 = (v2009 * v4622);
        let v5538: f64 = (v2011 * v4619);
        let v5541: f64 = ((v2011 * v4620) + (v1709 * (self.scalar_v2008 * v3223)));
        let v5544: f64 = ((v2011 * v4621) + (v1709 * (self.scalar_v2008 * v3227)));
        let v5547: f64 = ((v2011 * v4622) + (v1709 * (self.scalar_v2008 * v3231)));
        let v5592: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5572) / v2025))) } else { (if v2015 { (self.scalar_v0 - (self.scalar_v983 * ((v2016 * self.scalar_v5548) / v2017))) } else { v4 }) });
        let v5593: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5573) / v2025))) } else { (if v2015 { (self.scalar_v2364 - (self.scalar_v983 * ((v2016 * self.scalar_v5549) / v2017))) } else { v4 }) });
        let v5594: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5574) / v2025))) } else { (if v2015 { (self.scalar_v2365 - (self.scalar_v983 * ((v2016 * self.scalar_v5550) / v2017))) } else { v4 }) });
        let v5595: f64 = (if v2022 { (-(self.scalar_v983 * ((v2024 * self.scalar_v5575) / v2025))) } else { (if v2015 { (self.scalar_v2363 - (self.scalar_v983 * ((v2016 * self.scalar_v5551) / v2017))) } else { v4 }) });
        let v5605: f64 = (self.scalar_v1103 * f64::powf(v2031, self.scalar_v3146));
        let v5648: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3188 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5592 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v0 - v5592))))))));
        let v5649: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5593 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2364 - v5593)))) + self.scalar_v5634))));
        let v5650: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5594 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2365 - v5594)))) + self.scalar_v5635))));
        let v5651: f64 = (self.scalar_v14 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3189 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5595 / self.scalar_v244)) * v5605))) + (self.scalar_v1079 * (self.scalar_v2363 - v5595))))))));
        let v5690: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5573) / v2057))) } else { (if v2047 { (self.scalar_v2364 - (self.scalar_v983 * ((v2048 * self.scalar_v5549) / v2049))) } else { v4 }) });
        let v5691: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5673) / v2057))) } else { (if v2047 { (self.scalar_v2366 - (self.scalar_v983 * ((v2048 * self.scalar_v5652) / v2049))) } else { v4 }) });
        let v5692: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5574) / v2057))) } else { (if v2047 { (self.scalar_v2365 - (self.scalar_v983 * ((v2048 * self.scalar_v5550) / v2049))) } else { v4 }) });
        let v5693: f64 = (if v2054 { (-(self.scalar_v983 * ((v2056 * self.scalar_v5575) / v2057))) } else { (if v2047 { (self.scalar_v2363 - (self.scalar_v983 * ((v2048 * self.scalar_v5551) / v2049))) } else { v4 }) });
        let v5703: f64 = (self.scalar_v1103 * f64::powf(v2063, self.scalar_v3146));
        let v5745: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v5634 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5690 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2364 - v5690))))))));
        let v5746: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * ((self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5691 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2366 - v5691)))) + self.scalar_v5732))));
        let v5747: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v5635 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5692 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2365 - v5692))))))));
        let v5748: f64 = (self.scalar_v13 * (self.scalar_v2042 * (self.scalar_v284 * (self.scalar_v3189 + (self.scalar_v1078 * ((self.scalar_v1104 * (-((-(v5693 / self.scalar_v244)) * v5703))) + (self.scalar_v1079 * (self.scalar_v2363 - v5693))))))));
        let v5755: f64 = (if v2084 { (v2085 * self.scalar_v5750) } else { (if v1380 { v4 } else { (if v1377 { v4 } else { (if v1368 { (v1369 * self.scalar_v3660) } else { (if v1365 { (v1366 * self.scalar_v3660) } else { (if v1356 { v4 } else { v3650 }) }) }) }) }) });
        let v5761: f64 = (if v2087 { v4 } else { (if v2084 { v4 } else { (if v1380 { (v1381 * self.scalar_v3687) } else { (if v1377 { (v1378 * self.scalar_v3687) } else { (if v1368 { (v1369 * self.scalar_v3659) } else { (if v1365 { (v1366 * self.scalar_v3659) } else { v3654 }) }) }) }) }) });
        let v5765: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * self.scalar_v5749) } else { (if v2084 { (v2085 * self.scalar_v5749) } else { v3697 }) }));
        let v5766: f64 = (self.scalar_v2081 * v5761);
        let v5767: f64 = (self.scalar_v2081 * (if v2087 { (v2088 * self.scalar_v5750) } else { v5755 }));
        let v5768: f64 = (self.scalar_v2081 * (if v2087 { v4 } else { (if v2084 { v4 } else { (if v1380 { v4 } else { (if v1377 { v4 } else { (if v1368 { (v1369 * self.scalar_v3661) } else { (if v1365 { (v1366 * self.scalar_v3661) } else { v4 }) }) }) }) }) }));
        let v5769: f64 = (self.scalar_v2081 * (if v2087 { v4 } else { (if v2084 { v4 } else { (if v1380 { v4 } else { (if v1377 { v4 } else { (if v1368 { (v1369 * self.scalar_v3662) } else { (if v1365 { (v1366 * self.scalar_v3662) } else { v4 }) }) }) }) }) }));
        let v5772: f64 = (self.scalar_v2097 * (if v1023 { (((v1025 * v2519) - (v826 * v2521)) / v2992) } else { (if v1015 { (((v1019 * v2969) - (v1018 * v2969)) / v2973) } else { v2762 }) }));
        let v5790: f64 = ((self.scalar_v2007 * (((v1573 * v4039) - (v1570 * (v4039 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4043) - (v1569 * (v4043 / v4069))) / v4077)));
        let v5791: f64 = ((self.scalar_v2007 * (((v1573 * v4040) - (v1570 * (v4040 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4044) - (v1569 * (v4044 / v4069))) / v4077)));
        let v5792: f64 = ((self.scalar_v2007 * (((v1573 * v4041) - (v1570 * (v4041 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4045) - (v1569 * (v4045 / v4069))) / v4077)));
        let v5793: f64 = ((self.scalar_v2007 * (((v1573 * v4042) - (v1570 * (v4042 / v4047))) / v4055)) + (self.scalar_v2096 * (((v1577 * v4046) - (v1569 * (v4046 / v4069))) / v4077)));
        let v5838: f64 = (v31 * v2130);
        let v5846: f64 = (v2131 * v2131);
        let v5847: f64 = (((v2131 * (self.scalar_v2126 * v2401)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5810) } else { (if v2116 { (v2117 * self.scalar_v5810) } else { v4 }) })) / v5838))) / v5846);
        let v5851: f64 = (((v2131 * (self.scalar_v2126 * v2402)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5811) } else { (if v2116 { (v2117 * self.scalar_v5811) } else { v4 }) })) / v5838))) / v5846);
        let v5855: f64 = (((v2131 * (self.scalar_v2126 * v2403)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5812) } else { (if v2116 { (v2117 * self.scalar_v5812) } else { v4 }) })) / v5838))) / v5846);
        let v5859: f64 = (((v2131 * (self.scalar_v2126 * v2404)) - (v2127 * ((v407 * (if v2120 { (v2121 * self.scalar_v5813) } else { (if v2116 { (v2117 * self.scalar_v5813) } else { v4 }) })) / v5838))) / v5846);
        let v5860: f64 = (if self.scalar_v2115 { v5847 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5790) / self.scalar_v617) } else { v4 }) });
        let v5861: f64 = (if self.scalar_v2115 { v5851 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5791) / self.scalar_v617) } else { v4 }) });
        let v5862: f64 = (if self.scalar_v2115 { v5855 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5792) / self.scalar_v617) } else { v4 }) });
        let v5863: f64 = (if self.scalar_v2115 { v5859 } else { (if self.scalar_v2102 { ((self.scalar_v2103 * v5793) / self.scalar_v617) } else { v4 }) });
        let v5876: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2426) } else { v4 });
        let v5877: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2427) } else { v4 });
        let v5878: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2428) } else { v4 });
        let v5879: f64 = (if self.scalar_v2139 { (self.scalar_v1119 * v2429) } else { v4 });
        let v5880: f64 = (v31 * v2144);
        let v5888: f64 = (v2145 * v2145);
        let v5910: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2387) } else { (if v767 { (v768 * self.scalar_v2387) } else { v4 }) })) } else { v4 });
        let v5911: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2413) } else { (if v767 { (v768 * self.scalar_v2413) } else { v4 }) })) } else { v4 });
        let v5912: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2388) } else { (if v767 { (v768 * self.scalar_v2388) } else { v4 }) })) } else { v4 });
        let v5913: f64 = (if self.scalar_v2139 { (v407 * (if v770 { (v771 * self.scalar_v2368) } else { (if v767 { (v768 * self.scalar_v2368) } else { v4 }) })) } else { v4 });
        let v5914: f64 = (v31 * v2151);
        let v5922: f64 = (v2152 * v2152);
        let v5948: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5876) - (v2142 * (v5876 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5910) - (v2149 * (v5910 / v5914))) / v5922) } else { v4 })));
        let v5949: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5877) - (v2142 * (v5877 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5911) - (v2149 * (v5911 / v5914))) / v5922) } else { v4 })));
        let v5950: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5878) - (v2142 * (v5878 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5912) - (v2149 * (v5912 / v5914))) / v5922) } else { v4 })));
        let v5951: f64 = ((self.scalar_v2007 * (if self.scalar_v2139 { (((v2145 * v5879) - (v2142 * (v5879 / v5880))) / v5888) } else { v4 })) + (self.scalar_v2096 * (if self.scalar_v2139 { (((v2152 * v5913) - (v2149 * (v5913 / v5914))) / v5922) } else { v4 })));
        let v5988: f64 = (v31 * v2181);
        let v5996: f64 = (v2182 * v2182);
        let v5997: f64 = (((v2182 * (self.scalar_v2177 * v2426)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2387) } else { (if v2167 { (v2168 * self.scalar_v2387) } else { v4 }) })) / v5988))) / v5996);
        let v6001: f64 = (((v2182 * (self.scalar_v2177 * v2427)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2413) } else { (if v2167 { (v2168 * self.scalar_v2413) } else { v4 }) })) / v5988))) / v5996);
        let v6005: f64 = (((v2182 * (self.scalar_v2177 * v2428)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2388) } else { (if v2167 { (v2168 * self.scalar_v2388) } else { v4 }) })) / v5988))) / v5996);
        let v6009: f64 = (((v2182 * (self.scalar_v2177 * v2429)) - (v2178 * ((v407 * (if v2171 { (v2172 * self.scalar_v2368) } else { (if v2167 { (v2168 * self.scalar_v2368) } else { v4 }) })) / v5988))) / v5996);
        let v6015: f64 = (v1641 * (if self.scalar_v2166 { v5997 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5948) / self.scalar_v617) } else { v4 }) }));
        let v6019: f64 = ((v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (v4273 / v4269) } else { v4 }) })) + (v1641 * (if self.scalar_v2166 { v6001 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5949) / self.scalar_v617) } else { v4 }) })));
        let v6024: f64 = (v1641 * (if self.scalar_v2166 { v6005 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5950) / self.scalar_v617) } else { v4 }) }));
        let v6030: f64 = ((v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (v4293 / v4269) } else { v4 }) })) + (v1641 * (if self.scalar_v2166 { v6009 } else { (if self.scalar_v2139 { ((self.scalar_v2156 * v5951) / self.scalar_v617) } else { v4 }) })));
        let v6043: f64 = (self.scalar_v2189 * f64::powf(v1057, self.scalar_v6041));
        let v6053: f64 = (v2197 * v2197);
        let v6061: f64 = (v2203 * self.scalar_v6059);
        let v6062: f64 = (v2203 * self.scalar_v6060);
        let v6066: f64 = (v2204 * v2204);
        let v6076: f64 = ((v2206 * (if self.scalar_v2188 { (v3049 * v6043) } else { v4 })) + (v2192 * (if v2201 { (((v2204 * v6061) - (v2203 * v6061)) / v6066) } else { (if v2195 { ((-(v2196 * self.scalar_v6048)) / v6053) } else { v4 }) })));
        let v6079: f64 = ((v2206 * (if self.scalar_v2188 { (v3050 * v6043) } else { v4 })) + (v2192 * (if v2201 { (((v2204 * v6062) - (v2203 * v6062)) / v6066) } else { (if v2195 { ((-(v2196 * self.scalar_v6049)) / v6053) } else { v4 }) })));
        let v6092: f64 = (v1122 * v1122);
        let v6111: f64 = ((v2217 * (if self.scalar_v2188 { ((v2214 * ((self.scalar_v105 * v3192) / self.scalar_v355)) + (v2213 * ((-(v395 * v3195)) / v6092))) } else { v4 })) + (v2216 * (self.scalar_v2008 * v4619)));
        let v6114: f64 = ((v2217 * (if self.scalar_v2188 { ((v2214 * ((self.scalar_v105 * v3193) / self.scalar_v355)) + (v2213 * ((-(v395 * v3196)) / v6092))) } else { v4 })) + (v2216 * (self.scalar_v2008 * v4620)));
        let v6129: f64 = (if self.scalar_v2188 { (v5768 / self.scalar_v2082) } else { v4 });
        let v6145: f64 = ((v2224 * self.scalar_v6132) + (v2222 * ((if self.scalar_v2188 { (v5767 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { v6079 } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { v6114 } else { v4 })))));
        let v6149: f64 = (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v5765 / self.scalar_v2082) } else { v4 }) + ((if self.scalar_v2188 { (self.scalar_v1975 * (if self.scalar_v2188 { v6076 } else { v4 })) } else { v4 }) + (if self.scalar_v2188 { v6111 } else { v4 })))) } else { v4 });
        let v6168: f64 = (self.scalar_v2227 * v5768);
        let v6174: f64 = (if self.scalar_v2188 { (v5529 + (self.scalar_v2227 * v5765)) } else { v4 });
        let v6175: f64 = (if self.scalar_v2188 { (self.scalar_v2227 * v5766) } else { v4 });
        let v6176: f64 = (if self.scalar_v2188 { (v5532 + (self.scalar_v2227 * v5767)) } else { v4 });
        let v6177: f64 = (if self.scalar_v2188 { (v5533 + v6168) } else { v4 });
        let v6178: f64 = (if self.scalar_v2188 { (v5534 + v6168) } else { v4 });
        let v6179: f64 = (if self.scalar_v2188 { (self.scalar_v2227 * v5769) } else { v4 });
        let v6208: f64 = (if self.scalar_v2241 { v5529 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6174) } else { v4 }) });
        let v6209: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6175) } else { v4 }) });
        let v6210: f64 = (if self.scalar_v2241 { v5532 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6176) } else { v4 }) });
        let v6211: f64 = (if self.scalar_v2241 { v5533 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6177) } else { v4 }) });
        let v6212: f64 = (if self.scalar_v2241 { v5534 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6178) } else { v4 }) });
        let v6213: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2238 * v6179) } else { v4 }) });
        let v6214: f64 = (if self.scalar_v2241 { v5538 } else { (if self.scalar_v2188 { (v5538 + (self.scalar_v2234 * v6174)) } else { v4 }) });
        let v6215: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2234 * v6175) } else { v4 }) });
        let v6216: f64 = (if self.scalar_v2241 { v5541 } else { (if self.scalar_v2188 { (v5541 + (self.scalar_v2234 * v6176)) } else { v4 }) });
        let v6217: f64 = (if self.scalar_v2241 { v5544 } else { (if self.scalar_v2188 { (v5544 + (self.scalar_v2234 * v6177)) } else { v4 }) });
        let v6218: f64 = (if self.scalar_v2241 { v5547 } else { (if self.scalar_v2188 { (v5547 + (self.scalar_v2234 * v6178)) } else { v4 }) });
        let v6219: f64 = (if self.scalar_v2241 { v4 } else { (if self.scalar_v2188 { (self.scalar_v2234 * v6179) } else { v4 }) });
        let v6223: f64 = (if self.scalar_v2241 { v5768 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5768) } else { v4 }) });
        let v6241: f64 = (v2246 * v2246);
        let v6256: f64 = (((v2246 * (v6210 + v6216)) - (v2255 * (((v1171 * ((self.scalar_v1173 * v3209) + (self.scalar_v420 * v2386))) - (v2245 * v3335)) / v3353))) / v6241);
        let v6288: f64 = (if v2258 { ((v2259 * v3332) + (v1171 * (self.scalar_v610 * v4619))) } else { (if v2254 { (((v2246 * (v6208 + v6214)) - (v2255 * (((v1171 * (self.scalar_v420 * v2385)) - (v2245 * v3332)) / v3353))) / v6241) } else { v4 }) });
        let v6289: f64 = (if v2258 { v4 } else { (if v2254 { ((v6209 + v6215) / v2246) } else { v4 }) });
        let v6290: f64 = (if v2258 { ((v2259 * v3335) + (v1171 * (self.scalar_v610 * v4620))) } else { (if v2254 { v6256 } else { v4 }) });
        let v6291: f64 = (if v2258 { ((v2259 * v3338) + (v1171 * (self.scalar_v610 * v4621))) } else { (if v2254 { (((v2246 * (v6211 + v6217)) - (v2255 * (((v1171 * (self.scalar_v1173 * v3210)) - (v2245 * v3338)) / v3353))) / v6241) } else { v4 }) });
        let v6292: f64 = (if v2258 { ((v2259 * v3341) + (v1171 * (self.scalar_v610 * v4622))) } else { (if v2254 { (((v2246 * (v6212 + v6218)) - (v2255 * (((v1171 * (self.scalar_v1173 * v3211)) - (v2245 * v3341)) / v3353))) / v6241) } else { v4 }) });
        let v6293: f64 = (if v2258 { v4 } else { (if v2254 { ((v6213 + v6219) / v2246) } else { v4 }) });
        let v6367: f64 = (v5745 + (if self.scalar_v2136 { ((v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4247) - (v1633 * (v4247 + v4255))) / v4269) } else { v4 }) })) + v6015) } else { v4 }));
        let v6369: f64 = (v5745 + (if self.scalar_v2136 { (v6015 + (v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4250) - (v1633 * (v4250 + v4255))) / v4269) } else { v4 }) }))) } else { v4 }));
        let v6370: f64 = (v5747 + (if self.scalar_v2136 { ((v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4251) - (v1633 * (v4251 + v4257))) / v4269) } else { v4 }) })) + v6024) } else { v4 }));
        let v6371: f64 = (v5747 + (if self.scalar_v2136 { (v6024 + (v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4252) - (v1633 * (v4252 + v4257))) / v4269) } else { v4 }) }))) } else { v4 }));
        let v6373: f64 = (v5747 + (if self.scalar_v2136 { (v6024 + (v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4254) - (v1633 * (v4254 + v4257))) / v4269) } else { v4 }) }))) } else { v4 }));
        let v6495: f64 = 1.0;
        let v6502: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2241 { v5765 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5765) } else { v4 }) }) + ((self.scalar_v1975 * v3064) + v6208))) * v6495));
        let v6503: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6209 + (if self.scalar_v2241 { v5766 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5766) } else { v4 }) }))) * v6495));
        let v6504: f64 = (self.scalar_v27 * ((self.scalar_v0 * ((if self.scalar_v2241 { v5767 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5767) } else { v4 }) }) + ((self.scalar_v1975 * v3065) + v6210))) * v6495));
        let v6505: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6211 + v6223)) * v6495));
        let v6506: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6212 + v6223)) * v6495));
        let v6507: f64 = (self.scalar_v27 * ((self.scalar_v0 * (v6213 + (if self.scalar_v2241 { v5769 } else { (if self.scalar_v2188 { (self.scalar_v2228 * v5769) } else { v4 }) }))) * v6495));
        let v6512: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (self.scalar_v1994 * ((self.scalar_v1060 * (-((-(self.scalar_v268 * v5500)) * v5507))) + (v154 * (self.scalar_v2363 - v5500)))))));
        let v6513: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (self.scalar_v1994 * ((self.scalar_v1060 * (-((-(self.scalar_v268 * v5501)) * v5507))) + (v154 * (self.scalar_v0 - v5501)))))));
        let v6526: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6214)));
        let v6527: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6215)));
        let v6528: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * (self.scalar_v2097 * v3002)) + (v2098 * v2962)) + ((self.scalar_v2005 * v3190) + v6216)))));
        let v6529: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * (self.scalar_v2097 * v3003)) + (v2098 * v2963)) + ((self.scalar_v2005 * v3191) + v6217)))));
        let v6530: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (((v2099 * v5772) + (v2098 * v2958)) + ((self.scalar_v2005 * v3187) + v6218)))));
        let v6531: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6219)));
        let v6544: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6149)));
        let v6545: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { ((v2224 * self.scalar_v6131) + (v2222 * (if self.scalar_v2188 { (v5766 / self.scalar_v2082) } else { v4 }))) } else { v4 }))));
        let v6546: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { v6145 } else { v4 }))));
        let v6547: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v2216 * (self.scalar_v2008 * v4621)) } else { v4 }) + v6129)) } else { v4 }))));
        let v6548: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * ((if self.scalar_v2188 { (v2216 * (self.scalar_v2008 * v4622)) } else { v4 }) + v6129)) } else { v4 }))));
        let v6549: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2188 { (v2222 * (if self.scalar_v2188 { (v5769 / self.scalar_v2082) } else { v4 })) } else { v4 }))));
        let v6554: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6550));
        let v6555: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6551));
        let v6560: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6556));
        let v6561: f64 = (self.scalar_v27 * (v6495 * self.scalar_v6557));
        let v6606: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6367)));
        let v6607: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5746 + (if self.scalar_v2136 { v6019 } else { v4 })))));
        let v6608: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (if self.scalar_v2136 { (v2184 * (if self.scalar_v1640 { v4 } else { (if self.scalar_v1606 { (((v1636 * v4249) - (v1633 * v4249)) / v4269) } else { v4 }) })) } else { v4 }))));
        let v6609: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6369)));
        let v6610: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6370)));
        let v6611: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6371)));
        let v6612: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5748 + (if self.scalar_v2136 { v6030 } else { v4 })))));
        let v6613: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * v6373)));
        let v6645: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5648 + (if self.scalar_v2136 { (self.scalar_v14 * v5860) } else { v5860 })))));
        let v6646: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5649 + (if self.scalar_v2136 { (self.scalar_v14 * v5861) } else { v5861 })))));
        let v6647: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5650 + (if self.scalar_v2136 { (self.scalar_v14 * v5862) } else { v5862 })))));
        let v6648: f64 = (self.scalar_v27 * (v6495 * (self.scalar_v0 * (v5651 + (if self.scalar_v2136 { (self.scalar_v14 * v5863) } else { v5863 })))));
        let v6661: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6288) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6288) } else { v4 }) }) }));
        let v6662: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6289) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6289) } else { v4 }) }) }));
        let v6663: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6290) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6290) } else { v4 }) }) }));
        let v6664: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6291) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6291) } else { v4 }) }) }));
        let v6665: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6292) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6292) } else { v4 }) }) }));
        let v6666: f64 = (v2360 * (if self.scalar_v2273 { v4 } else { (if self.scalar_v2268 { (self.scalar_v2269 * v6293) } else { (if self.scalar_v2263 { (self.scalar_v2234 * v6293) } else { v4 }) }) }));
        let v6667: f64 = (v2274 * v6495);

        let d2319_dn3: f64 = v6502;
        let d2319_dn4: f64 = v6503;
        let d2319_dn5: f64 = v6504;
        let d2319_dn6: f64 = v6505;
        let d2319_dn7: f64 = v6506;
        let d2319_dn9: f64 = v6507;
        let v2319_reactive_nodes: [usize; 6] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]];
        let v2319_reactive_node_derivatives: [f64; 6] = [d2319_dn3, d2319_dn4, d2319_dn5, d2319_dn6, d2319_dn7, d2319_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &v2319_reactive_nodes,
            &v2319_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2322_dn3: f64 = v6512;
        let d2322_dn4: f64 = v6513;
        stamper.stamp_current_reactive_node2(
            Some(nodes[4]),
            Some(nodes[3]),
            nodes[3],
            multiplicity * (d2322_dn3),
            nodes[4],
            multiplicity * (d2322_dn4),
        );
        let d2325_dn3: f64 = v6526;
        let d2325_dn4: f64 = v6527;
        let d2325_dn5: f64 = v6528;
        let d2325_dn6: f64 = v6529;
        let d2325_dn7: f64 = v6530;
        let d2325_dn9: f64 = v6531;
        let v2325_reactive_nodes: [usize; 6] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]];
        let v2325_reactive_node_derivatives: [f64; 6] = [d2325_dn3, d2325_dn4, d2325_dn5, d2325_dn6, d2325_dn7, d2325_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            &v2325_reactive_nodes,
            &v2325_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2328_dn3: f64 = v6544;
        let d2328_dn4: f64 = v6545;
        let d2328_dn5: f64 = v6546;
        let d2328_dn6: f64 = v6547;
        let d2328_dn7: f64 = v6548;
        let d2328_dn9: f64 = v6549;
        let v2328_reactive_nodes: [usize; 6] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]];
        let v2328_reactive_node_derivatives: [f64; 6] = [d2328_dn3, d2328_dn4, d2328_dn5, d2328_dn6, d2328_dn7, d2328_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[5]),
            &v2328_reactive_nodes,
            &v2328_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2332_dn1: f64 = v6554;
        let d2332_dn2: f64 = v6555;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (d2332_dn1),
            nodes[2],
            multiplicity * (d2332_dn2),
        );
        let d2336_dn0: f64 = v6560;
        let d2336_dn1: f64 = v6561;
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (d2336_dn0),
            nodes[1],
            multiplicity * (d2336_dn1),
        );
        let d2344_dn0: f64 = v6606;
        let d2344_dn1: f64 = v6607;
        let d2344_dn3: f64 = v6608;
        let d2344_dn4: f64 = v6606;
        let d2344_dn5: f64 = v6609;
        let d2344_dn6: f64 = v6610;
        let d2344_dn7: f64 = v6611;
        let d2344_dn8: f64 = v6612;
        let d2344_dn9: f64 = v6613;
        let v2344_reactive_nodes: [usize; 9] = [nodes[0], nodes[1], nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[8], nodes[9]];
        let v2344_reactive_node_derivatives: [f64; 9] = [d2344_dn0, d2344_dn1, d2344_dn3, d2344_dn4, d2344_dn5, d2344_dn6, d2344_dn7, d2344_dn8, d2344_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[1]),
            Some(nodes[8]),
            &v2344_reactive_nodes,
            &v2344_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2350_dn4: f64 = v6645;
        let d2350_dn5: f64 = v6646;
        let d2350_dn6: f64 = v6647;
        let d2350_dn7: f64 = v6647;
        let d2350_dn9: f64 = v6648;
        let v2350_reactive_nodes: [usize; 5] = [nodes[4], nodes[5], nodes[6], nodes[7], nodes[9]];
        let v2350_reactive_node_derivatives: [f64; 5] = [d2350_dn4, d2350_dn5, d2350_dn6, d2350_dn7, d2350_dn9];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            Some(nodes[9]),
            &v2350_reactive_nodes,
            &v2350_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
        let d2361_dn3: f64 = v6661;
        let d2361_dn4: f64 = v6662;
        let d2361_dn5: f64 = v6663;
        let d2361_dn6: f64 = v6664;
        let d2361_dn7: f64 = v6665;
        let d2361_dn9: f64 = v6666;
        let d2361_dn10: f64 = v6667;
        let v2361_reactive_nodes: [usize; 7] = [nodes[3], nodes[4], nodes[5], nodes[6], nodes[7], nodes[9], nodes[10]];
        let v2361_reactive_node_derivatives: [f64; 7] = [d2361_dn3, d2361_dn4, d2361_dn5, d2361_dn6, d2361_dn7, d2361_dn9, d2361_dn10];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[3]),
            &v2361_reactive_nodes,
            &v2361_reactive_node_derivatives,
            &[],
            &[],
            multiplicity,
        );
    }
}
