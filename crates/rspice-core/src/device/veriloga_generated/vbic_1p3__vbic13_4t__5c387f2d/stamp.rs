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
    v0: f64,
    v1: f64,
    v2: f64,
    v18: f64,
    v26: f64,
    v36: f64,
    v39: f64,
    v49: f64,
    v100: f64,
    v125: f64,
    v139: f64,
    v141: f64,
    v388: f64,
    v409: f64,
    v410: f64,
    v411: f64,
    v479: f64,
    v498: f64,
    v502: f64,
    v509: f64,
    v516: f64,
    v523: f64,
    v534: f64,
    v571: f64,
    v634: f64,
    v689: f64,
    v690: f64,
    v741: f64,
    v742: f64,
    v744: f64,
    v745: f64,
    v747: f64,
    v748: f64,
    v750: f64,
    v751: f64,
    v756: f64,
    v758: f64,
    v759: f64,
    v760: f64,
    v764: f64,
    v773: f64,
    v775: f64,
    v780: f64,
    v781: f64,
    v1067: f64,
    v1085: f64,
    v1090: f64,
    v1093: f64,
    v1098: f64,
    v1125: f64,
    v1138: f64,
    v1180: f64,
    v1209: f64,
    v1239: f64,
    v1241: f64,
    v1297: f64,
    v1322: f64,
    v1323: f64,
    v1362: f64,
    v1380: f64,
    v1381: f64,
    v1423: f64,
    v1440: f64,
    v1441: f64,
    v1477: f64,
    v1496: f64,
    v1497: f64,
    v1535: f64,
    v1536: f64,
    v1566: f64,
    v1583: f64,
    v1586: f64,
    v1750: f64,
    v1765: f64,
    v2151: f64,
    v2153: f64,
    v2155: f64,
    v2157: f64,
    v2160: f64,
    v2161: f64,
    v2162: f64,
    v2163: f64,
    v2164: f64,
    v2165: f64,
    v2166: f64,
    v2171: f64,
    v2173: f64,
    v2174: f64,
    v2245: f64,
    v2288: f64,
    v2295: f64,
    v2299: f64,
    v2311: f64,
    v2315: f64,
    v2327: f64,
    v2331: f64,
    v2343: f64,
    v2347: f64,
    v2367: f64,
    v2371: f64,
    v2495: f64,
    v2579: f64,
    v2582: f64,
    v2586: f64,
    v2641: f64,
    v3232: f64,
    v3233: f64,
    v3234: f64,
    v3268: f64,
    v3269: f64,
    v3270: f64,
    v3271: f64,
    v3301: f64,
    v3302: f64,
    v3303: f64,
    v3304: f64,
    v3363: f64,
    v3364: f64,
    v3365: f64,
    v3366: f64,
    v3393: f64,
    v3394: f64,
    v3395: f64,
    v3396: f64,
    v3400: f64,
    v3492: f64,
    v3493: f64,
    v3494: f64,
    v3495: f64,
    v3496: f64,
    v3497: f64,
    v3562: f64,
    v3563: f64,
    v3564: f64,
    v3565: f64,
    v3566: f64,
    v3567: f64,
    v3568: f64,
    v3668: f64,
    v3669: f64,
    v3670: f64,
    v3671: f64,
    v3672: f64,
    v3673: f64,
    v3674: f64,
    v3677: f64,
    v3784: f64,
    v3785: f64,
    v3786: f64,
    v3787: f64,
    v3830: f64,
    v3831: f64,
    v3832: f64,
    v3833: f64,
    v3834: f64,
    v3835: f64,
    v3836: f64,
    v3837: f64,
    v3911: f64,
    v3912: f64,
    v3913: f64,
    v3914: f64,
    v3950: f64,
    v3951: f64,
    v3952: f64,
    v3953: f64,
    v3954: f64,
    v3955: f64,
    v3956: f64,
    v3957: f64,
    v4071: f64,
    v4072: f64,
    v4073: f64,
    v4074: f64,
    v4110: f64,
    v4111: f64,
    v4112: f64,
    v4113: f64,
    v4114: f64,
    v4115: f64,
    v4116: f64,
    v4117: f64,
    v4198: f64,
    v4199: f64,
    v4200: f64,
    v4201: f64,
    v4238: f64,
    v4239: f64,
    v4240: f64,
    v4241: f64,
    v4242: f64,
    v4243: f64,
    v4244: f64,
    v4246: f64,
    v4314: f64,
    v4315: f64,
    v4316: f64,
    v4317: f64,
    v4318: f64,
    v4319: f64,
    v4320: f64,
    v4321: f64,
    v4437: f64,
    v4438: f64,
    v4439: f64,
    v4440: f64,
    v4441: f64,
    v4442: f64,
    v4443: f64,
    v4452: f64,
    v4453: f64,
    v4454: f64,
    v4455: f64,
    v4456: f64,
    v4936: f64,
    v4958: f64,
    v4959: f64,
    v4960: f64,
    v4961: f64,
    v4962: f64,
    v4963: f64,
    v4964: f64,
    v6094: f64,
    v6095: f64,
    v6096: f64,
    v6097: f64,
    v6098: f64,
    v6099: f64,
    v6100: f64,
    v6101: f64,
    v6102: f64,
    v6103: f64,
    v6104: f64,
    v6105: f64,
    v6106: f64,
    v6107: f64,
    v6108: f64,
    v6109: f64,
    v6110: f64,
    v6111: f64,
    v6112: f64,
    v6113: f64,
    v6114: f64,
    v6115: f64,
    v6116: f64,
    v6117: f64,
    v6118: f64,
    v6119: f64,
    v6120: f64,
    v6121: f64,
    v6122: f64,
    v6123: f64,
    v6124: f64,
}

impl Instance {
    #[inline(always)]
    fn eval_common_stamp_values(&mut self, ctx: &GeneratedEvalContext<'_>) -> CommonStampValues {
        let nodes = self.nodes;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let v0=if ctx.analysis_static() { 1.0 } else { 0.0 };
        let v1=0.0;
        let v2=1.0;
        let v18=(if ((v0!=0.0)&&self.scalar_static_bool[1]){v2}else{(if ((v0!=0.0)&&(self.scalar_static_f64[2]!=0.0)){self.scalar_static_f64[3]}else{v1})});
        let v24=((v0!=0.0)&&self.scalar_static_bool[2]);
        let v26=-1.0;
        let v30=(v24&&self.scalar_static_bool[3]);
        let v36=(if (v30&&self.scalar_static_bool[4]){v2}else{(if ((self.scalar_static_f64[6]!=0.0)&&v30){self.scalar_static_f64[7]}else{(if ((self.scalar_static_f64[5]!=0.0)&&v24){v26}else{(if ((v0!=0.0)&&(self.scalar_static_f64[4]!=0.0)){v2}else{v1})})})});
        let v39=(if (v0!=0.0){self.scalar_static_f64[9]}else{v1});
        let v44=(if (v0!=0.0){self.scalar_static_f64[12]}else{v1});
        let v49=(if (v0!=0.0){self.scalar_static_f64[15]}else{v1});
        let v59=(if (v0!=0.0){self.scalar_static_f64[21]}else{v1});
        let v64=(if (v0!=0.0){self.scalar_static_f64[24]}else{v1});
        let v67=273.15;
        let v70=(if (v0!=0.0){self.scalar_static_f64[27]}else{v1});
        let v96=1.380662e-23;
        let v98=1.602189e-19;
        let v100=(self.scalar_static_f64[286]/v70);
        let v117=(if self.scalar_static_bool[11]{v1}else{(if (self.scalar_static_f64[35]!=0.0){(self.scalar_static_f64[289]*((self.scalar_static_f64[291]+(v18/self.scalar_static_f64[34]))).ln())}else{v1})});
        let v125=(v2-v100);
        let v130=((self.scalar_static_f64[33]*f64::powf(v100,self.scalar_static_f64[41]))*(((self.scalar_static_f64[43]*v125)/self.scalar_static_f64[292])).exp());
        let v131=(v130>v1);
        let v132=(if v131{v2}else{v1});
        let v137=(if (self.scalar_static_bool[12]&&(v18>self.scalar_static_f64[44])){v2}else{v1});
        let v139=0.5;
        let v140=(v18*v139);
        let v141=4.0;
        let v162=(if (!(v132!=0.0)){v1}else{(if ((v132!=0.0)&&(!(v137!=0.0))){(self.scalar_static_f64[292]*((v2+(v18/v130))).ln())}else{(if ((v132!=0.0)&&(v137!=0.0)){(self.scalar_static_f64[292]*((v2+(f64::powf((v140*self.scalar_static_f64[47]),self.scalar_static_f64[49])/v130))).ln())}else{v1})})});
        let v175=((self.scalar_static_f64[50]*f64::powf(v100,self.scalar_static_f64[53]))*(((v125*self.scalar_static_f64[55])/self.scalar_static_f64[293])).exp());
        let v178=(if (v131&&(v175>v1)){v2}else{v1});
        let v181=(if (self.scalar_static_bool[5]&&(v18>self.scalar_static_f64[10])){v2}else{v1});
        let v187=(v130*v175);
        let v201=(if (!(v178!=0.0)){v1}else{(if ((v178!=0.0)&&(!(v181!=0.0))){(self.scalar_static_f64[293]*((v2+(v18/v187))).ln())}else{(if ((v178!=0.0)&&(v181!=0.0)){(self.scalar_static_f64[293]*((v2+(f64::powf((v140*self.scalar_static_f64[57]),self.scalar_static_f64[49])/v187))).ln())}else{v1})})});
        let v213=((self.scalar_static_f64[58]*f64::powf(v100,self.scalar_static_f64[60]))*(((v125*self.scalar_static_f64[62])/self.scalar_static_f64[294])).exp());
        let v215=(if (v213>v1){v2}else{v1});
        let v218=(if (self.scalar_static_bool[6]&&(v18>self.scalar_static_f64[13])){v2}else{v1});
        let v235=(if (!(v215!=0.0)){v1}else{(if ((v215!=0.0)&&(!(v218!=0.0))){(self.scalar_static_f64[294]*((v2+(v18/v213))).ln())}else{(if ((v215!=0.0)&&(v218!=0.0)){(self.scalar_static_f64[294]*((v2+((v49*(v18*v18))/v213))).ln())}else{v1})})});
        let v248=((self.scalar_static_f64[63]*f64::powf(v100,self.scalar_static_f64[66]))*(((v125*self.scalar_static_f64[68])/self.scalar_static_f64[295])).exp());
        let v250=(if (v248>v1){v2}else{v1});
        let v257=(if (!(v250!=0.0)){v1}else{(if (v250!=0.0){(self.scalar_static_f64[295]*((v2+(v18/v248))).ln())}else{v1})});
        let v283=f64::powf(v100,self.scalar_static_f64[77]);
        let v290=(((v125*self.scalar_static_f64[79])/self.scalar_static_f64[297])).exp();
        let v291=((self.scalar_static_f64[75]*v283)*v290);
        let v293=(if (v291>v1){v2}else{v1});
        let v300=(if (!(v293!=0.0)){v1}else{(if (v293!=0.0){(self.scalar_static_f64[297]*((v2+(v18/v291))).ln())}else{v1})});
        let v324=(v290*(v283*self.scalar_static_f64[85]));
        let v326=(if (v324>v1){v2}else{v1});
        let v333=(if (!(v326!=0.0)){v1}else{(if (v326!=0.0){(self.scalar_static_f64[297]*((v2+(v18/v324))).ln())}else{v1})});
        let v357=((self.scalar_static_f64[87]*f64::powf(v100,self.scalar_static_f64[89]))*(((v125*self.scalar_static_f64[91])/self.scalar_static_f64[299])).exp());
        let v359=(if (v357>v1){v2}else{v1});
        let v366=(if (!(v359!=0.0)){v1}else{(if (v359!=0.0){(self.scalar_static_f64[299]*((v2+(v18/v357))).ln())}else{v1})});
        let v388=ctx.node_voltage(nodes[4]);
        let v390=((self.scalar_static_f64[272]+v388)-v67);
        let v392=(if (v390<self.scalar_static_f64[30]){v2}else{v1});
        let v395=(((v390-self.scalar_static_f64[29])-v2)).exp();
        let v397=(if (v392!=0.0){(self.scalar_static_f64[29]+v395)}else{v390});
        let v401=(((if (v397>self.scalar_static_f64[32]){v2}else{v1})!=0.0)&&(!(v392!=0.0)));
        let v404=(((self.scalar_static_f64[31]-v397)-v2)).exp();
        let v407=(v67+(if v401{(self.scalar_static_f64[31]-v404)}else{v397}));
        let v409=((v96*v407)/v98);
        let v410=(v407/v70);
        let v411=(v407-v70);
        let v414=(self.scalar_static_f64[44]*f64::powf(v410,self.scalar_static_f64[97]));
        let v478=(self.scalar_static_f64[33]*f64::powf(v410,self.scalar_static_f64[41]));
        let v479=(v2-v410);
        let v480=(self.scalar_static_f64[43]*v479);
        let v481=(self.scalar_static_f64[40]*v409);
        let v483=((v480/v481)).exp();
        let v484=(v478*v483);
        let v486=(self.scalar_static_f64[50]*f64::powf(v410,self.scalar_static_f64[53]));
        let v487=(self.scalar_static_f64[55]*v479);
        let v488=(self.scalar_static_f64[52]*v409);
        let v490=((v487/v488)).exp();
        let v491=(v486*v490);
        let v493=(self.scalar_static_f64[58]*f64::powf(v410,self.scalar_static_f64[60]));
        let v494=(self.scalar_static_f64[62]*v479);
        let v495=(self.scalar_static_f64[59]*v409);
        let v497=((v494/v495)).exp();
        let v498=(v493*v497);
        let v502=(self.scalar_static_f64[65]*v409);
        let v509=(self.scalar_static_f64[71]*v409);
        let v516=(self.scalar_static_f64[76]*v409);
        let v523=(self.scalar_static_f64[81]*v409);
        let v534=(self.scalar_static_f64[88]*v409);
        let v547=(v2+(v411*self.scalar_static_f64[121]));
        let v548=(self.scalar_static_f64[40]*v547);
        let v549=(self.scalar_static_f64[52]*v547);
        let v563=(self.scalar_static_f64[126]+(v411*self.scalar_static_f64[127]));
        let v570=(self.scalar_static_f64[36]*(v2+(v411*self.scalar_static_f64[128])));
        let v571=2.0;
        let v573=(v571*(v409/v410));
        let v576=(v410*self.scalar_static_f64[130]);
        let v578=((v576/v409)).exp();
        let v579=-0.5;
        let v581=(v410*self.scalar_static_f64[131]);
        let v583=((v581/v409)).exp();
        let v584=(v578-v583);
        let v585=(v584).ln();
        let v586=(v573*v585);
        let v588=3.0;
        let v589=(v409*v588);
        let v590=(v410).ln();
        let v591=(v589*v590);
        let v593=(v410-v2);
        let v595=(((v410*v586)-v591)-(self.scalar_static_f64[67]*v593));
        let v596=(v409*v571);
        let v597=(-v595);
        let v599=((v597/v409)).exp();
        let v602=((v2+(v141*v599))).sqrt();
        let v604=(v139*(v2+v602));
        let v605=(v604).ln();
        let v607=(v595+(v596*v605));
        let v610=(v410*self.scalar_static_f64[133]);
        let v612=((v610/v409)).exp();
        let v614=(v410*self.scalar_static_f64[134]);
        let v616=((v614/v409)).exp();
        let v617=(v612-v616);
        let v618=(v617).ln();
        let v619=(v573*v618);
        let v623=(((v410*v619)-v591)-(self.scalar_static_f64[78]*v593));
        let v624=(-v623);
        let v626=((v624/v409)).exp();
        let v629=((v2+(v141*v626))).sqrt();
        let v631=(v139*(v2+v629));
        let v632=(v631).ln();
        let v634=(v623+(v596*v632));
        let v637=(v410*self.scalar_static_f64[136]);
        let v639=((v637/v409)).exp();
        let v641=(v410*self.scalar_static_f64[137]);
        let v643=((v641/v409)).exp();
        let v644=(v639-v643);
        let v645=(v644).ln();
        let v646=(v573*v645);
        let v650=(((v410*v646)-v591)-(self.scalar_static_f64[90]*v593));
        let v651=(-v650);
        let v653=((v651/v409)).exp();
        let v656=((v2+(v141*v653))).sqrt();
        let v658=(v139*(v2+v656));
        let v659=(v658).ln();
        let v661=(v650+(v596*v659));
        let v663=(self.scalar_static_f64[129]/v607);
        let v666=(self.scalar_static_f64[138]*f64::powf(v663,self.scalar_static_f64[139]));
        let v668=(self.scalar_static_f64[132]/v634);
        let v670=f64::powf(v668,self.scalar_static_f64[141]);
        let v671=(self.scalar_static_f64[140]*v670);
        let v673=(v670*self.scalar_static_f64[142]);
        let v675=(self.scalar_static_f64[135]/v661);
        let v678=(self.scalar_static_f64[143]*f64::powf(v675,self.scalar_static_f64[144]));
        let v681=(self.scalar_static_f64[145]*f64::powf(v410,self.scalar_static_f64[39]));
        let v683=((v480/v409)).exp();
        let v684=(v681*v683);
        let v689=(-(self.scalar_static_f64[37]*(v2+(v411*v563))));
        let v690=(v409*v570);
        let v697=(self.scalar_static_f64[148]*(v2+(v411*self.scalar_static_f64[149])));
        let v702=(self.scalar_static_f64[150]*(v2+(v411*self.scalar_static_f64[151])));
        let v729=(v697>v1);
        let v731=(if v729{(v2/v697)}else{v1});
        let v732=(v702>v1);
        let v734=(if v732{(v2/v702)}else{v1});
        let v735=(v414>v1);
        let v737=(if v735{(v2/v414)}else{v1});
        let v741=ctx.node_voltage(nodes[8]);
        let v742=ctx.node_voltage(nodes[9]);
        let v744=(v36*(v741-v742));
        let v745=ctx.node_voltage(nodes[7]);
        let v747=(v36*(v745-v742));
        let v748=ctx.node_voltage(nodes[6]);
        let v750=(v36*(v741-v748));
        let v751=ctx.node_voltage(nodes[5]);
        let v753=(v36*(v741-v751));
        let v756=ctx.node_voltage(nodes[10]);
        let v758=(v36*(v745-v756));
        let v759=ctx.node_voltage(nodes[1]);
        let v760=ctx.node_voltage(nodes[2]);
        let v764=ctx.node_voltage(nodes[0]);
        let v773=ctx.node_voltage(nodes[11]);
        let v775=(v36*(v773-v756));
        let v780=ctx.node_voltage(nodes[12]);
        let v781=ctx.node_voltage(nodes[13]);
        let v782=(-v607);
        let v784=(v782*self.scalar_static_f64[152]);
        let v788=(v744+v784);
        let v789=(if (self.scalar_static_f64[154]!=0.0){v788}else{v1});
        let v791=(if (v789>v1){v2}else{v1});
        let v792=((self.scalar_static_f64[154]!=0.0)&&(v791!=0.0));
        let v796=(if v792{self.scalar_static_f64[157]}else{v1});
        let v798=(v2-(self.scalar_static_f64[155]*v796));
        let v804=(v789*self.scalar_static_f64[159]);
        let v805=(v607*self.scalar_static_f64[155]);
        let v807=(v2+(v804/v805));
        let v812=((self.scalar_static_f64[154]!=0.0)&&(!(v791!=0.0)));
        let v814=(v2-(v744/v607));
        let v816=(v2-f64::powf(v814,self.scalar_static_f64[158]));
        let v819=(if v812{((v607*v816)/self.scalar_static_f64[158])}else{(if v792{((v607*v798)/self.scalar_static_f64[158])}else{v1})});
        let v828=(((v784*v784)+self.scalar_static_f64[161])).sqrt();
        let v832=(if self.scalar_static_bool[19]{(v579*(v784+(if self.scalar_static_bool[19]{v828}else{v1})))}else{v1});
        let v834=(v2-(v832/v607));
        let v835=f64::powf(v834,self.scalar_static_f64[158]);
        let v838=(if self.scalar_static_bool[19]{((v782*v835)/self.scalar_static_f64[158])}else{v1});
        let v839=(if self.scalar_static_bool[19]{v788}else{v1});
        let v842=((self.scalar_static_f64[161]+(v839*v839))).sqrt();
        let v847=(if self.scalar_static_bool[19]{((v139*(v839-(if self.scalar_static_bool[19]{v842}else{v1})))-v784)}else{v1});
        let v849=(v2-(v847/v607));
        let v850=f64::powf(v849,self.scalar_static_f64[158]);
        let v855=(v832+(v744-v847));
        let v856=(self.scalar_static_f64[157]*v855);
        let v857=(self.scalar_static_f64[159]*v855);
        let v859=(v2+(v857/v805));
        let v863=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{((v782*v850)/self.scalar_static_f64[158])}else{v819})+(v856*v859))-v838)}else{(if (self.scalar_static_f64[154]!=0.0){(v819+(if v812{v1}else{(if v792{(v796*(v789*v807))}else{v1})}))}else{v1})});
        let v864=(-v634);
        let v865=(self.scalar_static_f64[152]*v864);
        let v869=(v750+v865);
        let v870=(if (self.scalar_static_f64[163]!=0.0){v869}else{v1});
        let v872=(if (v870>v1){v2}else{v1});
        let v873=((self.scalar_static_f64[163]!=0.0)&&(v872!=0.0));
        let v876=(if v873{self.scalar_static_f64[165]}else{v1});
        let v879=(v2-(self.scalar_static_f64[155]*(self.scalar_static_f64[155]*v876)));
        let v885=(v870*self.scalar_static_f64[167]);
        let v887=(self.scalar_static_f64[155]+(v885/v634));
        let v896=(if (self.scalar_static_bool[21]&&(v750<self.scalar_static_f64[169])){v2}else{v1});
        let v898=((self.scalar_static_f64[163]!=0.0)&&(!(v872!=0.0)));
        let v899=((v896!=0.0)&&v898);
        let v901=(v2+(self.scalar_static_f64[168]/v634));
        let v902=f64::powf(v901,self.scalar_static_f64[166]);
        let v904=(self.scalar_static_f64[166]*(v750+self.scalar_static_f64[168]));
        let v905=(v634+self.scalar_static_f64[168]);
        let v907=(v2-(v904/v905));
        let v909=(v2-(v902*v907));
        let v914=(v898&&(!(v896!=0.0)));
        let v916=(v2-(v750/v634));
        let v918=(v2-f64::powf(v916,self.scalar_static_f64[166]));
        let v921=(if v914{((v634*v918)/self.scalar_static_f64[166])}else{(if v899{((v634*v909)/self.scalar_static_f64[166])}else{(if v873{((v634*v879)/self.scalar_static_f64[166])}else{v1})})});
        let v931=(v865+self.scalar_static_f64[168]);
        let v932=(self.scalar_static_f64[168]-v865);
        let v934=(if self.scalar_static_bool[25]{(v931/v932)}else{v1});
        let v935=(v571*v934);
        let v936=(v934-v2);
        let v941=(((v936*v936)+self.scalar_static_f64[173])).sqrt();
        let v942=(v2+v934);
        let v947=(((v942*v942)+self.scalar_static_f64[175])).sqrt();
        let v948=(v941+v947);
        let v950=(if self.scalar_static_bool[25]{(v935/v948)}else{v1});
        let v955=(if self.scalar_static_bool[25]{(v139*(((v932*v950)-self.scalar_static_f64[168])-v865))}else{v1});
        let v957=(v2-(v955/v634));
        let v959=(v2-f64::powf(v957,self.scalar_static_f64[166]));
        let v962=(if self.scalar_static_bool[25]{((v634*v959)/self.scalar_static_f64[166])}else{v1});
        let v965=(v865+(self.scalar_static_f64[168]+(v571*v750)));
        let v967=(if self.scalar_static_bool[25]{(v965/v932)}else{v1});
        let v968=(v571*v967);
        let v969=(v967-v2);
        let v972=((self.scalar_static_f64[173]+(v969*v969))).sqrt();
        let v973=(v2+v967);
        let v976=((self.scalar_static_f64[175]+(v973*v973))).sqrt();
        let v977=(v972+v976);
        let v979=(if self.scalar_static_bool[25]{(v968/v977)}else{v1});
        let v984=(if self.scalar_static_bool[25]{(v139*(((v932*v979)-self.scalar_static_f64[168])-v865))}else{v1});
        let v986=(v2-(v984/v634));
        let v988=(v2-f64::powf(v986,self.scalar_static_f64[166]));
        let v991=(if self.scalar_static_bool[25]{((v634*v988)/self.scalar_static_f64[166])}else{v921});
        let v994=(if self.scalar_static_bool[25]{(v139*(v2+v979))}else{v1});
        let v997=(if self.scalar_static_bool[25]{f64::powf(v901,self.scalar_static_f64[176])}else{v1});
        let v999=(v2+(v865/v634));
        let v1001=(if self.scalar_static_bool[25]{f64::powf(v999,self.scalar_static_f64[176])}else{v1});
        let v1002=(v2-v994);
        let v1006=(if self.scalar_static_bool[25]{((v997*v1002)+(v994*v1001))}else{v1});
        let v1008=(v955+(v750-v984));
        let v1018=((self.scalar_static_f64[173]+(v865*v865))).sqrt();
        let v1022=(if self.scalar_static_bool[27]{(v579*(v865+(if self.scalar_static_bool[27]{v1018}else{v1})))}else{v955});
        let v1024=(v2-(v1022/v634));
        let v1025=f64::powf(v1024,self.scalar_static_f64[166]);
        let v1028=(if self.scalar_static_bool[27]{((v864*v1025)/self.scalar_static_f64[166])}else{v1});
        let v1029=(if self.scalar_static_bool[27]{v869}else{v1});
        let v1032=((self.scalar_static_f64[173]+(v1029*v1029))).sqrt();
        let v1037=(if self.scalar_static_bool[27]{((v139*(v1029-(if self.scalar_static_bool[27]{v1032}else{v1})))-v865)}else{v984});
        let v1039=(v2-(v1037/v634));
        let v1040=f64::powf(v1039,self.scalar_static_f64[166]);
        let v1050=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{((v864*v1040)/self.scalar_static_f64[166])}else{v991})+(self.scalar_static_f64[177]*(v1022+(v750-v1037))))-v1028)}else{(if self.scalar_static_bool[25]{((v991+(if self.scalar_static_bool[25]{(v1006*v1008)}else{v1}))-v962)}else{(if (self.scalar_static_f64[163]!=0.0){(v921+(if v898{v1}else{(if v873{(v876*(v870*v887))}else{v1})}))}else{v1})})});
        let v1051=(v409*v548);
        let v1052=(v2/v1051);
        let v1054=(if (v744<v162){v2}else{v1});
        let v1056=((v744*v1052)).exp();
        let v1058=(!(v1054!=0.0));
        let v1060=((v162*v1052)).exp();
        let v1061=(v744-v162);
        let v1063=(v2+(v1052*v1061));
        let v1065=(if v1058{(v1060*v1063)}else{(if (v1054!=0.0){v1056}else{v1})});
        let v1066=(v1065-v2);
        let v1067=(v484*v1066);
        let v1068=(v409*v549);
        let v1069=(v2/v1068);
        let v1071=(if (v750<v201){v2}else{v1});
        let v1073=((v750*v1069)).exp();
        let v1075=(!(v1071!=0.0));
        let v1077=((v201*v1069)).exp();
        let v1078=(v750-v201);
        let v1080=(v2+(v1069*v1078));
        let v1082=(if v1075{(v1077*v1080)}else{(if (v1071!=0.0){v1073}else{v1065})});
        let v1083=(v484*v491);
        let v1084=(v1082-v2);
        let v1085=(v1083*v1084);
        let v1090=0.0001;
        let v1091=(((v2+(v734*v863))+(v731*v1050))-v1090);
        let v1093=1e-8;
        let v1095=(((v1091*v1091)+v1093)).sqrt();
        let v1098=(v1090+(v139*(v1091+v1095)));
        let v1107=(v141*((v737*v1067)+(v44*v1085)));
        let v1109=(if (self.scalar_static_f64[179]!=0.0){(f64::powf(v1098,self.scalar_static_f64[180])+v1107)}else{v1});
        let v1111=(if (v1109>v1093){v2}else{v1});
        let v1112=((self.scalar_static_f64[179]!=0.0)&&(v1111!=0.0));
        let v1118=((self.scalar_static_f64[179]!=0.0)&&(!(v1111!=0.0)));
        let v1125=(if self.scalar_static_bool[29]{(v2+v1107)}else{v1109});
        let v1127=(if (v1125>v1093){v2}else{v1});
        let v1128=(self.scalar_static_bool[29]&&(v1127!=0.0));
        let v1129=(v139*v1098);
        let v1131=(v2+f64::powf(v1125,self.scalar_static_f64[46]));
        let v1135=(self.scalar_static_bool[29]&&(!(v1127!=0.0)));
        let v1138=(if v1135{(v1129*self.scalar_static_f64[182])}else{(if v1128{(v1129*v1131)}else{(if v1118{(v139*(v1098+self.scalar_static_f64[181]))}else{(if v1112{(v139*(v1098+f64::powf(v1109,self.scalar_static_f64[46])))}else{v1})})})});
        let v1144=(if (self.scalar_static_f64[183]!=0.0){(v2/v495)}else{v1069});
        let v1146=(if (v758<v235){v2}else{v1});
        let v1147=((self.scalar_static_f64[183]!=0.0)&&(v1146!=0.0));
        let v1149=((v758*v1144)).exp();
        let v1152=((self.scalar_static_f64[183]!=0.0)&&(!(v1146!=0.0)));
        let v1154=((v235*v1144)).exp();
        let v1155=(v758-v235);
        let v1157=(v2+(v1144*v1155));
        let v1159=(if v1152{(v1154*v1157)}else{(if v1147{v1149}else{v1082})});
        let v1161=(if (v750<v235){v2}else{v1});
        let v1162=((self.scalar_static_f64[183]!=0.0)&&(v1161!=0.0));
        let v1164=((v750*v1144)).exp();
        let v1167=((self.scalar_static_f64[183]!=0.0)&&(!(v1161!=0.0)));
        let v1168=(v750-v235);
        let v1170=(v2+(v1144*v1168));
        let v1172=(if v1167{(v1154*v1170)}else{(if v1162{v1164}else{v1})});
        let v1178=(((v1159*self.scalar_static_f64[184])+(v1172*self.scalar_static_f64[185]))-v2);
        let v1180=(if (self.scalar_static_f64[183]!=0.0){(v498*v1178)}else{v1});
        let v1198=(if (v775<v235){v2}else{v1});
        let v1199=((self.scalar_static_f64[183]!=0.0)&&(v1198!=0.0));
        let v1201=((v775*v1144)).exp();
        let v1204=((self.scalar_static_f64[183]!=0.0)&&(!(v1198!=0.0)));
        let v1205=(v775-v235);
        let v1207=(v2+(v1144*v1205));
        let v1209=(if v1204{(v1154*v1207)}else{(if v1199{v1201}else{v1159})});
        let v1223=(v2/v502);
        let v1224=(if (self.scalar_static_f64[187]!=0.0){v1223}else{v1144});
        let v1226=(if (v744<v257){v2}else{v1});
        let v1227=((self.scalar_static_f64[187]!=0.0)&&(v1226!=0.0));
        let v1229=((v744*v1224)).exp();
        let v1231=(!(v1226!=0.0));
        let v1232=((self.scalar_static_f64[187]!=0.0)&&v1231);
        let v1234=((v257*v1224)).exp();
        let v1235=(v744-v257);
        let v1237=(v2+(v1224*v1235));
        let v1239=(if v1232{(v1234*v1237)}else{(if v1227{v1229}else{v1209})});
        let v1240=(v2/v509);
        let v1241=(if (self.scalar_static_f64[187]!=0.0){v1240}else{v1224});
        let v1279=(v689-v744);
        let v1280=(if self.scalar_static_bool[38]{v1279}else{v1});
        let v1281=(v2/v690);
        let v1282=(if self.scalar_static_bool[38]{v1281}else{v1241});
        let v1284=(if (v1280<v117){v2}else{v1});
        let v1285=(self.scalar_static_bool[38]&&(v1284!=0.0));
        let v1287=((v1280*v1282)).exp();
        let v1290=(self.scalar_static_bool[38]&&(!(v1284!=0.0)));
        let v1292=((v117*v1282)).exp();
        let v1293=(v1280-v117);
        let v1295=(v2+(v1282*v1293));
        let v1297=(if v1290{(v1292*v1295)}else{(if v1285{v1287}else{v1172})});
        let v1307=(if self.scalar_static_bool[41]{v1223}else{v1282});
        let v1309=(if (v747<v257){v2}else{v1});
        let v1310=(self.scalar_static_bool[41]&&(v1309!=0.0));
        let v1312=((v747*v1307)).exp();
        let v1314=(!(v1309!=0.0));
        let v1315=(self.scalar_static_bool[41]&&v1314);
        let v1317=((v257*v1307)).exp();
        let v1318=(v747-v257);
        let v1320=(v2+(v1307*v1318));
        let v1322=(if v1315{(v1317*v1320)}else{(if v1310{v1312}else{v1239})});
        let v1323=(if self.scalar_static_bool[41]{v1240}else{v1307});
        let v1346=(if self.scalar_static_bool[42]{v1279}else{v1280});
        let v1347=(if self.scalar_static_bool[42]{v1281}else{v1323});
        let v1349=(if (v1346<v117){v2}else{v1});
        let v1350=(self.scalar_static_bool[42]&&(v1349!=0.0));
        let v1352=((v1346*v1347)).exp();
        let v1355=(self.scalar_static_bool[42]&&(!(v1349!=0.0)));
        let v1357=((v117*v1347)).exp();
        let v1358=(v1346-v117);
        let v1360=(v2+(v1347*v1358));
        let v1362=(if v1355{(v1357*v1360)}else{(if v1350{v1352}else{v1297})});
        let v1369=(if self.scalar_static_bool[44]{v1223}else{v1347});
        let v1370=((v1226!=0.0)&&self.scalar_static_bool[44]);
        let v1372=((v744*v1369)).exp();
        let v1374=(v1231&&self.scalar_static_bool[44]);
        let v1376=((v257*v1369)).exp();
        let v1378=(v2+(v1235*v1369));
        let v1380=(if v1374{(v1376*v1378)}else{(if v1370{v1372}else{v1322})});
        let v1381=(if self.scalar_static_bool[44]{v1240}else{v1369});
        let v1407=(if self.scalar_static_bool[47]{v1279}else{v1346});
        let v1408=(if self.scalar_static_bool[47]{v1281}else{v1381});
        let v1410=(if (v1407<v117){v2}else{v1});
        let v1411=(self.scalar_static_bool[47]&&(v1410!=0.0));
        let v1413=((v1407*v1408)).exp();
        let v1416=(self.scalar_static_bool[47]&&(!(v1410!=0.0)));
        let v1418=((v117*v1408)).exp();
        let v1419=(v1407-v117);
        let v1421=(v2+(v1408*v1419));
        let v1423=(if v1416{(v1418*v1421)}else{(if v1411{v1413}else{v1362})});
        let v1429=(if self.scalar_static_bool[44]{v1223}else{v1408});
        let v1430=((v1309!=0.0)&&self.scalar_static_bool[44]);
        let v1432=((v747*v1429)).exp();
        let v1434=(v1314&&self.scalar_static_bool[44]);
        let v1436=((v257*v1429)).exp();
        let v1438=(v2+(v1318*v1429));
        let v1440=(if v1434{(v1436*v1438)}else{(if v1430{v1432}else{v1380})});
        let v1441=(if self.scalar_static_bool[44]{v1240}else{v1429});
        let v1461=(if self.scalar_static_bool[47]{v1279}else{v1407});
        let v1462=(if self.scalar_static_bool[47]{v1281}else{v1441});
        let v1464=(if (v1461<v117){v2}else{v1});
        let v1465=(self.scalar_static_bool[47]&&(v1464!=0.0));
        let v1467=((v1461*v1462)).exp();
        let v1470=(self.scalar_static_bool[47]&&(!(v1464!=0.0)));
        let v1472=((v117*v1462)).exp();
        let v1473=(v1461-v117);
        let v1475=(v2+(v1462*v1473));
        let v1477=(if v1470{(v1472*v1475)}else{(if v1465{v1467}else{v1423})});
        let v1483=(v2/v516);
        let v1485=(if (v750<v300){v2}else{v1});
        let v1487=((v750*v1483)).exp();
        let v1489=(!(v1485!=0.0));
        let v1491=((v300*v1483)).exp();
        let v1492=(v750-v300);
        let v1494=(v2+(v1483*v1492));
        let v1496=(if v1489{(v1491*v1494)}else{(if (v1485!=0.0){v1487}else{v1440})});
        let v1497=(v2/v523);
        let v1520=(if (self.scalar_static_f64[195]!=0.0){v1483}else{v1497});
        let v1522=(if (v758<v333){v2}else{v1});
        let v1523=((self.scalar_static_f64[195]!=0.0)&&(v1522!=0.0));
        let v1525=((v758*v1520)).exp();
        let v1528=((self.scalar_static_f64[195]!=0.0)&&(!(v1522!=0.0)));
        let v1530=((v333*v1520)).exp();
        let v1531=(v758-v333);
        let v1533=(v2+(v1520*v1531));
        let v1535=(if v1528{(v1530*v1533)}else{(if v1523{v1525}else{v1496})});
        let v1536=(if (self.scalar_static_f64[195]!=0.0){v1497}else{v1520});
        let v1560=(v750/v409);
        let v1562=(if (v1560<v39){v2}else{v1});
        let v1563=(v1560).exp();
        let v1565=(!(v1562!=0.0));
        let v1566=(v39).exp();
        let v1570=(if v1565{(v1566*(v2+(v1560-v39)))}else{(if (v1562!=0.0){v1563}else{v1535})});
        let v1571=(v753/v409);
        let v1573=(if (v1571<v39){v2}else{v1});
        let v1574=(v1571).exp();
        let v1576=(!(v1573!=0.0));
        let v1580=(if v1576{(v1566*(v2+(v1571-v39)))}else{(if (v1573!=0.0){v1574}else{v1477})});
        let v1583=((v2+(v684*v1570))).sqrt();
        let v1586=((v2+(v684*v1580))).sqrt();
        let v1750=(if (self.scalar_static_f64[213]!=0.0){(v2/v534)}else{v1536});
        let v1752=(if (v775<v366){v2}else{v1});
        let v1753=((self.scalar_static_f64[213]!=0.0)&&(v1752!=0.0));
        let v1755=((v775*v1750)).exp();
        let v1758=((self.scalar_static_f64[213]!=0.0)&&(!(v1752!=0.0)));
        let v1760=((v366*v1750)).exp();
        let v1761=(v775-v366);
        let v1763=(v2+(v1750*v1761));
        let v1765=(if v1758{(v1760*v1763)}else{(if v1753{v1755}else{v1570})});
        let v1848=(-v661);
        let v1850=(if (self.scalar_static_f64[216]!=0.0){(self.scalar_static_f64[152]*v1848)}else{v1});
        let v1855=(v775+v1850);
        let v1856=(if self.scalar_static_bool[70]{v1855}else{v1});
        let v1858=(if (v1856>v1){v2}else{v1});
        let v1859=(self.scalar_static_bool[70]&&(v1858!=0.0));
        let v1862=(if v1859{self.scalar_static_f64[220]}else{v1});
        let v1864=(v2-(self.scalar_static_f64[155]*v1862));
        let v1870=(v1856*self.scalar_static_f64[222]);
        let v1871=(v661*self.scalar_static_f64[155]);
        let v1873=(v2+(v1870/v1871));
        let v1878=(self.scalar_static_bool[70]&&(!(v1858!=0.0)));
        let v1880=(v2-(v775/v661));
        let v1882=(v2-f64::powf(v1880,self.scalar_static_f64[221]));
        let v1885=(if v1878{((v661*v1882)/self.scalar_static_f64[221])}else{(if v1859{((v661*v1864)/self.scalar_static_f64[221])}else{v1})});
        let v1895=(((v1850*v1850)+self.scalar_static_f64[224])).sqrt();
        let v1899=(if self.scalar_static_bool[72]{(v579*(v1850+(if self.scalar_static_bool[72]{v1895}else{v1})))}else{v1});
        let v1901=(v2-(v1899/v661));
        let v1902=f64::powf(v1901,self.scalar_static_f64[221]);
        let v1906=(if self.scalar_static_bool[72]{v1855}else{v1});
        let v1909=((self.scalar_static_f64[224]+(v1906*v1906))).sqrt();
        let v1914=(if self.scalar_static_bool[72]{((v139*(v1906-(if self.scalar_static_bool[72]{v1909}else{v1})))-v1850)}else{v1});
        let v1916=(v2-(v1914/v661));
        let v1917=f64::powf(v1916,self.scalar_static_f64[221]);
        let v1922=(v1899+(v775-v1914));
        let v1923=(self.scalar_static_f64[220]*v1922);
        let v1924=(self.scalar_static_f64[222]*v1922);
        let v1926=(v2+(v1924/v1871));
        let v1932=(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{(((if self.scalar_static_bool[72]{((v1848*v1917)/self.scalar_static_f64[221])}else{v1885})+(v1923*v1926))-(if self.scalar_static_bool[72]{((v1848*v1902)/self.scalar_static_f64[221])}else{v1}))}else{(if self.scalar_static_bool[70]{(v1885+(if v1878{v1}else{(if v1859{(v1862*(v1856*v1873))}else{v1})}))}else{v1})})});
        let v1933=(v747+v784);
        let v1934=(if (self.scalar_static_f64[154]!=0.0){v1933}else{v1});
        let v1936=(if (v1934>v1){v2}else{v1});
        let v1937=((self.scalar_static_f64[154]!=0.0)&&(v1936!=0.0));
        let v1938=(if v1937{self.scalar_static_f64[157]}else{v1});
        let v1940=(v2-(self.scalar_static_f64[155]*v1938));
        let v1944=(self.scalar_static_f64[159]*v1934);
        let v1946=(v2+(v1944/v805));
        let v1951=((self.scalar_static_f64[154]!=0.0)&&(!(v1936!=0.0)));
        let v1953=(v2-(v747/v607));
        let v1955=(v2-f64::powf(v1953,self.scalar_static_f64[158]));
        let v1958=(if v1951{((v607*v1955)/self.scalar_static_f64[158])}else{(if v1937{((v607*v1940)/self.scalar_static_f64[158])}else{v1})});
        let v1962=(if self.scalar_static_bool[19]{v1933}else{v1});
        let v1965=((self.scalar_static_f64[161]+(v1962*v1962))).sqrt();
        let v1970=(if self.scalar_static_bool[19]{((v139*(v1962-(if self.scalar_static_bool[19]{v1965}else{v1})))-v784)}else{v1});
        let v1972=(v2-(v1970/v607));
        let v1973=f64::powf(v1972,self.scalar_static_f64[158]);
        let v1978=(v832+(v747-v1970));
        let v1979=(self.scalar_static_f64[157]*v1978);
        let v1980=(self.scalar_static_f64[159]*v1978);
        let v1982=(v2+(v1980/v805));
        let v1986=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{((v782*v1973)/self.scalar_static_f64[158])}else{v1958})+(v1979*v1982))-v838)}else{(if (self.scalar_static_f64[154]!=0.0){(v1958+(if v1951{v1}else{(if v1937{(v1938*(v1934*v1946))}else{v1})}))}else{v1})});
        let v1987=(v758+v865);
        let v1988=(if (self.scalar_static_f64[163]!=0.0){v1987}else{v1});
        let v1990=(if (v1988>v1){v2}else{v1});
        let v1991=((self.scalar_static_f64[163]!=0.0)&&(v1990!=0.0));
        let v1992=(if v1991{self.scalar_static_f64[165]}else{v1});
        let v1995=(v2-(self.scalar_static_f64[155]*(self.scalar_static_f64[155]*v1992)));
        let v1999=(self.scalar_static_f64[167]*v1988);
        let v2001=(self.scalar_static_f64[155]+(v1999/v634));
        let v2007=(if (self.scalar_static_bool[21]&&(v758<self.scalar_static_f64[169])){v2}else{v1});
        let v2009=((self.scalar_static_f64[163]!=0.0)&&(!(v1990!=0.0)));
        let v2010=((v2007!=0.0)&&v2009);
        let v2012=(self.scalar_static_f64[166]*(v758+self.scalar_static_f64[168]));
        let v2014=(v2-(v2012/v905));
        let v2016=(v2-(v902*v2014));
        let v2021=(v2009&&(!(v2007!=0.0)));
        let v2023=(v2-(v758/v634));
        let v2025=(v2-f64::powf(v2023,self.scalar_static_f64[166]));
        let v2028=(if v2021{((v634*v2025)/self.scalar_static_f64[166])}else{(if v2010{((v634*v2016)/self.scalar_static_f64[166])}else{(if v1991{((v634*v1995)/self.scalar_static_f64[166])}else{v1})})});
        let v2034=(v865+(self.scalar_static_f64[168]+(v571*v758)));
        let v2036=(if self.scalar_static_bool[25]{(v2034/v932)}else{v1});
        let v2037=(v571*v2036);
        let v2038=(v2036-v2);
        let v2041=((self.scalar_static_f64[173]+(v2038*v2038))).sqrt();
        let v2042=(v2+v2036);
        let v2045=((self.scalar_static_f64[175]+(v2042*v2042))).sqrt();
        let v2046=(v2041+v2045);
        let v2048=(if self.scalar_static_bool[25]{(v2037/v2046)}else{v1});
        let v2053=(if self.scalar_static_bool[25]{(v139*(((v932*v2048)-self.scalar_static_f64[168])-v865))}else{v1});
        let v2055=(v2-(v2053/v634));
        let v2057=(v2-f64::powf(v2055,self.scalar_static_f64[166]));
        let v2060=(if self.scalar_static_bool[25]{((v634*v2057)/self.scalar_static_f64[166])}else{v2028});
        let v2063=(if self.scalar_static_bool[25]{(v139*(v2+v2048))}else{v1});
        let v2064=(v2-v2063);
        let v2068=(if self.scalar_static_bool[25]{((v997*v2064)+(v1001*v2063))}else{v1});
        let v2070=(v955+(v758-v2053));
        let v2076=(if self.scalar_static_bool[27]{v1987}else{v1});
        let v2079=((self.scalar_static_f64[173]+(v2076*v2076))).sqrt();
        let v2084=(if self.scalar_static_bool[27]{((v139*(v2076-(if self.scalar_static_bool[27]{v2079}else{v1})))-v865)}else{v2053});
        let v2086=(v2-(v2084/v634));
        let v2087=f64::powf(v2086,self.scalar_static_f64[166]);
        let v2096=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{((v864*v2087)/self.scalar_static_f64[166])}else{v2060})+(self.scalar_static_f64[177]*(v1022+(v758-v2084))))-v1028)}else{(if self.scalar_static_bool[25]{((v2060+(if self.scalar_static_bool[25]{(v2068*v2070)}else{v1}))-v962)}else{(if (self.scalar_static_f64[163]!=0.0){(v2028+(if v2009{v1}else{(if v1991{(v1992*(v1988*v2001))}else{v1})}))}else{v1})})});
        let v2098=(if (v1067>v1){v2}else{v1});
        let v2100=(v64*(v1067*v2098));
        let v2101=(v2+v2100);
        let v2102=(v2100/v2101);
        let v2104=1.44;
        let v2105=((v59*v750)/v2104);
        let v2107=(if (v2105<v39){v2}else{v1});
        let v2108=(v2105).exp();
        let v2110=(!(v2107!=0.0));
        let v2119=(self.scalar_static_f64[225]*(v2+(v1098*self.scalar_static_f64[226])));
        let v2121=((if v2110{(v1566*(v2+(v2105-v39)))}else{(if (v2107!=0.0){v2108}else{v1765})})*self.scalar_static_f64[227]);
        let v2123=((if (v0!=0.0){self.scalar_static_f64[25]}else{v1})+(v2102*v2102));
        let v2126=(v2+(v2098*(v2121*v2123)));
        let v2127=(v2119*v2126);
        let v2130=(v1067*v2127);
        let v2151=((v759-v760)*self.scalar_static_f64[231]);
        let v2153=((v759-v764)*self.scalar_static_f64[232]);
        let v2155=(v388*self.scalar_static_f64[233]);
        let v2157=(v780*self.scalar_static_f64[234]);
        let v2160=((v781*self.scalar_static_f64[234])*0.3333333333333333);
        let v2161=(v36*((self.scalar_static_f64[186]*(v666*v863))+(v2130/v1138)));
        let v2162=(v36*(self.scalar_static_f64[193]*(v666*v1986)));
        let v2163=(v36*(((v671*v1050)+(v1085*self.scalar_static_f64[228]))+(v1583*self.scalar_static_f64[229])));
        let v2164=(v36*(v1586*self.scalar_static_f64[229]));
        let v2165=(v36*((v673*v2096)+((if self.scalar_static_bool[31]{v1}else{v1180})*self.scalar_static_f64[228])));
        let v2166=(v36*((v678*v1932)+(v775*self.scalar_static_f64[230])));
        let v2167=(if (v392!=0.0){v395}else{v2});
        let v2171=(if v401{(-(v404*(-v2167)))}else{v2167});
        let v2173=((v96*v2171)/v98);
        let v2174=(v2171/v70);
        let v2245=(-v2174);
        let v2246=(self.scalar_static_f64[43]*v2245);
        let v2256=((v483*(self.scalar_static_f64[33]*(v2174*(self.scalar_static_f64[41]*f64::powf(v410,self.scalar_static_f64[245])))))+(v478*(v483*(((v481*v2246)-(v480*(self.scalar_static_f64[40]*v2173)))/(v481*v481)))));
        let v2279=(self.scalar_static_f64[59]*v2173);
        let v2283=(v495*v495);
        let v2288=((v497*(self.scalar_static_f64[58]*(v2174*(self.scalar_static_f64[60]*f64::powf(v410,self.scalar_static_f64[247])))))+(v493*(v497*(((v495*(self.scalar_static_f64[62]*v2245))-(v494*v2279))/v2283))));
        let v2295=(self.scalar_static_f64[65]*v2173);
        let v2299=(v502*v502);
        let v2311=(self.scalar_static_f64[71]*v2173);
        let v2315=(v509*v509);
        let v2327=(self.scalar_static_f64[76]*v2173);
        let v2331=(v516*v516);
        let v2343=(self.scalar_static_f64[81]*v2173);
        let v2347=(v523*v523);
        let v2367=(self.scalar_static_f64[88]*v2173);
        let v2371=(v534*v534);
        let v2393=(self.scalar_static_f64[121]*v2171);
        let v2412=(v571*(((v410*v2173)-(v409*v2174))/(v410*v410)));
        let v2417=(v409*v409);
        let v2438=((v590*(v588*v2173))+(v589*(v2174/v410)));
        let v2441=((((v586*v2174)+(v410*((v585*v2412)+(v573*(((v578*(((v409*(self.scalar_static_f64[130]*v2174))-(v576*v2173))/v2417))-(v583*(((v409*(self.scalar_static_f64[131]*v2174))-(v581*v2173))/v2417)))/v584)))))-v2438)-(self.scalar_static_f64[67]*v2174));
        let v2442=(v571*v2173);
        let v2457=(v2441+((v605*v2442)+(v596*((v139*((v141*(v599*(((v409*(-v2441))-(v597*v2173))/v2417)))/(v571*v602)))/v604))));
        let v2480=((((v619*v2174)+(v410*((v618*v2412)+(v573*(((v612*(((v409*(self.scalar_static_f64[133]*v2174))-(v610*v2173))/v2417))-(v616*(((v409*(self.scalar_static_f64[134]*v2174))-(v614*v2173))/v2417)))/v617)))))-v2438)-(self.scalar_static_f64[78]*v2174));
        let v2495=(v2480+((v632*v2442)+(v596*((v139*((v141*(v626*(((v409*(-v2480))-(v624*v2173))/v2417)))/(v571*v629)))/v631))));
        let v2518=((((v646*v2174)+(v410*((v645*v2412)+(v573*(((v639*(((v409*(self.scalar_static_f64[136]*v2174))-(v637*v2173))/v2417))-(v643*(((v409*(self.scalar_static_f64[137]*v2174))-(v641*v2173))/v2417)))/v644)))))-v2438)-(self.scalar_static_f64[90]*v2174));
        let v2533=(v2518+((v659*v2442)+(v596*((v139*((v141*(v653*(((v409*(-v2518))-(v651*v2173))/v2417)))/(v571*v656)))/v658))));
        let v2536=(v607*v607);
        let v2542=(self.scalar_static_f64[138]*(((-(self.scalar_static_f64[129]*v2457))/v2536)*(self.scalar_static_f64[139]*f64::powf(v663,self.scalar_static_f64[254]))));
        let v2545=(v634*v634);
        let v2549=(((-(self.scalar_static_f64[132]*v2495))/v2545)*(self.scalar_static_f64[141]*f64::powf(v668,self.scalar_static_f64[200])));
        let v2554=(v661*v661);
        let v2573=((v683*(self.scalar_static_f64[145]*(v2174*(self.scalar_static_f64[39]*f64::powf(v410,self.scalar_static_f64[256])))))+(v681*(v683*(((v409*v2246)-(v480*v2173))/v2417))));
        let v2579=(-(self.scalar_static_f64[37]*((v563*v2171)+(v411*(self.scalar_static_f64[127]*v2171)))));
        let v2582=((v570*v2173)+(v409*(self.scalar_static_f64[36]*(self.scalar_static_f64[128]*v2171))));
        let v2586=(v690*v690);
        let v2641=(-v36);
        let v2642=(-v2457);
        let v2643=(self.scalar_static_f64[152]*v2642);
        let v2644=(if (self.scalar_static_f64[154]!=0.0){v2643}else{v1});
        let v2645=(if (self.scalar_static_f64[154]!=0.0){v36}else{v1});
        let v2646=(if (self.scalar_static_f64[154]!=0.0){v2641}else{v1});
        let v2653=(self.scalar_static_f64[155]*v2457);
        let v2654=(v805*(self.scalar_static_f64[159]*v2644));
        let v2657=(v805*v805);
        let v2659=((self.scalar_static_f64[159]*v2645)/v805);
        let v2660=((self.scalar_static_f64[159]*v2646)/v805);
        let v2682=(-(v36/v607));
        let v2683=(-(v2641/v607));
        let v2686=(self.scalar_static_f64[158]*f64::powf(v814,self.scalar_static_f64[258]));
        let v2701=(if v812{(((v816*v2457)+(v607*(-((-((-(v744*v2457))/v2536))*v2686))))/self.scalar_static_f64[158])}else{(if v792{((v798*v2457)/self.scalar_static_f64[158])}else{v1})});
        let v2702=(if v812{((v607*(-(v2682*v2686)))/self.scalar_static_f64[158])}else{v1});
        let v2703=(if v812{((v607*(-(v2683*v2686)))/self.scalar_static_f64[158])}else{v1});
        let v2713=(v784*v2643);
        let v2720=(if self.scalar_static_bool[19]{(v579*(v2643+(if self.scalar_static_bool[19]{((v2713+v2713)/(v571*v828))}else{v1})))}else{v1});
        let v2733=(if self.scalar_static_bool[19]{(((v835*v2642)+(v782*((-(((v607*v2720)-(v832*v2457))/v2536))*(self.scalar_static_f64[158]*f64::powf(v834,self.scalar_static_f64[258])))))/self.scalar_static_f64[158])}else{v1});
        let v2734=(if self.scalar_static_bool[19]{v2643}else{v1});
        let v2735=(if self.scalar_static_bool[19]{v36}else{v1});
        let v2736=(if self.scalar_static_bool[19]{v2641}else{v1});
        let v2737=(v839*v2734);
        let v2739=(v839*v2735);
        let v2741=(v839*v2736);
        let v2743=(v571*v842);
        let v2757=(if self.scalar_static_bool[19]{((v139*(v2734-(if self.scalar_static_bool[19]{((v2737+v2737)/v2743)}else{v1})))-v2643)}else{v1});
        let v2758=(if self.scalar_static_bool[19]{(v139*(v2735-(if self.scalar_static_bool[19]{((v2739+v2739)/v2743)}else{v1})))}else{v1});
        let v2759=(if self.scalar_static_bool[19]{(v139*(v2736-(if self.scalar_static_bool[19]{((v2741+v2741)/v2743)}else{v1})))}else{v1});
        let v2770=(self.scalar_static_f64[158]*f64::powf(v849,self.scalar_static_f64[258]));
        let v2786=(v36-v2758);
        let v2787=(v2641-v2759);
        let v2788=(v2720+(-v2757));
        let v2814=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{(((v850*v2642)+(v782*((-(((v607*v2757)-(v847*v2457))/v2536))*v2770)))/self.scalar_static_f64[158])}else{v2701})+((v859*(self.scalar_static_f64[157]*v2788))+(v856*(((v805*(self.scalar_static_f64[159]*v2788))-(v857*v2653))/v2657))))-v2733)}else{(if (self.scalar_static_f64[154]!=0.0){(v2701+(if v812{v1}else{(if v792{(v796*((v807*v2644)+(v789*((v2654-(v804*v2653))/v2657))))}else{v1})}))}else{v1})});
        let v2815=(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v782*((-(v2758/v607))*v2770))/self.scalar_static_f64[158])}else{v2702})+((v859*(self.scalar_static_f64[157]*v2786))+(v856*((self.scalar_static_f64[159]*v2786)/v805))))}else{(if (self.scalar_static_f64[154]!=0.0){(v2702+(if v812{v1}else{(if v792{(v796*((v807*v2645)+(v789*v2659)))}else{v1})}))}else{v1})});
        let v2816=(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v782*((-(v2759/v607))*v2770))/self.scalar_static_f64[158])}else{v2703})+((v859*(self.scalar_static_f64[157]*v2787))+(v856*((self.scalar_static_f64[159]*v2787)/v805))))}else{(if (self.scalar_static_f64[154]!=0.0){(v2703+(if v812{v1}else{(if v792{(v796*((v807*v2646)+(v789*v2660)))}else{v1})}))}else{v1})});
        let v2817=(-v2495);
        let v2818=(self.scalar_static_f64[152]*v2817);
        let v2819=(if (self.scalar_static_f64[163]!=0.0){v2818}else{v1});
        let v2820=(if (self.scalar_static_f64[163]!=0.0){v2641}else{v1});
        let v2821=(if (self.scalar_static_f64[163]!=0.0){v36}else{v1});
        let v2828=(v634*(self.scalar_static_f64[167]*v2819));
        let v2832=((self.scalar_static_f64[167]*v2820)/v634);
        let v2833=((self.scalar_static_f64[167]*v2821)/v634);
        let v2851=((-(self.scalar_static_f64[168]*v2495))/v2545);
        let v2855=(v2851*(self.scalar_static_f64[166]*f64::powf(v901,self.scalar_static_f64[259])));
        let v2860=(v905*v905);
        let v2881=((v634*(-(v902*(-((self.scalar_static_f64[166]*v2641)/v905)))))/self.scalar_static_f64[166]);
        let v2882=((v634*(-(v902*(-((v36*self.scalar_static_f64[166])/v905)))))/self.scalar_static_f64[166]);
        let v2892=(-(v2641/v634));
        let v2893=(-(v36/v634));
        let v2895=(self.scalar_static_f64[166]*f64::powf(v916,self.scalar_static_f64[259]));
        let v2910=(if v914{(((v918*v2495)+(v634*(-((-((-(v750*v2495))/v2545))*v2895))))/self.scalar_static_f64[166])}else{(if v899{(((v909*v2495)+(v634*(-((v907*v2855)+(v902*(-((-(v904*v2495))/v2860)))))))/self.scalar_static_f64[166])}else{(if v873{((v879*v2495)/self.scalar_static_f64[166])}else{v1})})});
        let v2911=(if v914{((v634*(-(v2892*v2895)))/self.scalar_static_f64[166])}else{(if v899{v2881}else{v1})});
        let v2912=(if v914{((v634*(-(v2893*v2895)))/self.scalar_static_f64[166])}else{(if v899{v2882}else{v1})});
        let v2922=(-v2818);
        let v2923=(v932*v2818);
        let v2926=(v932*v932);
        let v2928=(if self.scalar_static_bool[25]{((v2923-(v931*v2922))/v2926)}else{v1});
        let v2930=(v936*v2928);
        let v2934=(v942*v2928);
        let v2950=(if self.scalar_static_bool[25]{(v139*(((v950*v2922)+(v932*(if self.scalar_static_bool[25]{(((v948*(v571*v2928))-(v935*(((v2930+v2930)/(v571*v941))+((v2934+v2934)/(v571*v947)))))/(v948*v948))}else{v1})))-v2818))}else{v1});
        let v2964=(if self.scalar_static_bool[25]{(((v959*v2495)+(v634*(-((-(((v634*v2950)-(v955*v2495))/v2545))*(self.scalar_static_f64[166]*f64::powf(v957,self.scalar_static_f64[259]))))))/self.scalar_static_f64[166])}else{v1});
        let v2972=(if self.scalar_static_bool[25]{((v2923-(v965*v2922))/v2926)}else{v1});
        let v2973=(if self.scalar_static_bool[25]{((v571*v2641)/v932)}else{v1});
        let v2974=(if self.scalar_static_bool[25]{((v36*v571)/v932)}else{v1});
        let v2976=(v571*v2973);
        let v2977=(v571*v2974);
        let v2978=(v969*v2972);
        let v2980=(v969*v2973);
        let v2982=(v969*v2974);
        let v2984=(v571*v972);
        let v2988=(v973*v2972);
        let v2990=(v973*v2973);
        let v2992=(v973*v2974);
        let v2994=(v571*v976);
        let v3004=(v977*v977);
        let v3014=(if self.scalar_static_bool[25]{(((v977*(v571*v2972))-(v968*(((v2978+v2978)/v2984)+((v2988+v2988)/v2994))))/v3004)}else{v1});
        let v3015=(if self.scalar_static_bool[25]{(((v977*v2976)-(v968*(((v2980+v2980)/v2984)+((v2990+v2990)/v2994))))/v3004)}else{v1});
        let v3016=(if self.scalar_static_bool[25]{(((v977*v2977)-(v968*(((v2982+v2982)/v2984)+((v2992+v2992)/v2994))))/v3004)}else{v1});
        let v3026=(if self.scalar_static_bool[25]{(v139*(((v979*v2922)+(v932*v3014))-v2818))}else{v1});
        let v3027=(if self.scalar_static_bool[25]{(v139*(v932*v3015))}else{v1});
        let v3028=(if self.scalar_static_bool[25]{(v139*(v932*v3016))}else{v1});
        let v3039=(self.scalar_static_f64[166]*f64::powf(v986,self.scalar_static_f64[259]));
        let v3054=(if self.scalar_static_bool[25]{(((v988*v2495)+(v634*(-((-(((v634*v3026)-(v984*v2495))/v2545))*v3039))))/self.scalar_static_f64[166])}else{v2910});
        let v3055=(if self.scalar_static_bool[25]{((v634*(-((-(v3027/v634))*v3039)))/self.scalar_static_f64[166])}else{v2911});
        let v3056=(if self.scalar_static_bool[25]{((v634*(-((-(v3028/v634))*v3039)))/self.scalar_static_f64[166])}else{v2912});
        let v3060=(if self.scalar_static_bool[25]{(v139*v3014)}else{v1});
        let v3061=(if self.scalar_static_bool[25]{(v139*v3015)}else{v1});
        let v3062=(if self.scalar_static_bool[25]{(v139*v3016)}else{v1});
        let v3067=(if self.scalar_static_bool[25]{(v2851*(self.scalar_static_f64[176]*f64::powf(v901,self.scalar_static_f64[260])))}else{v1});
        let v3075=(if self.scalar_static_bool[25]{((((v634*v2818)-(v865*v2495))/v2545)*(self.scalar_static_f64[176]*f64::powf(v999,self.scalar_static_f64[260])))}else{v1});
        let v3118=(v865*v2818);
        let v3125=(if self.scalar_static_bool[27]{(v579*(v2818+(if self.scalar_static_bool[27]{((v3118+v3118)/(v571*v1018))}else{v1})))}else{v2950});
        let v3138=(if self.scalar_static_bool[27]{(((v1025*v2817)+(v864*((-(((v634*v3125)-(v1022*v2495))/v2545))*(self.scalar_static_f64[166]*f64::powf(v1024,self.scalar_static_f64[259])))))/self.scalar_static_f64[166])}else{v1});
        let v3139=(if self.scalar_static_bool[27]{v2818}else{v1});
        let v3140=(if self.scalar_static_bool[27]{v2641}else{v1});
        let v3141=(if self.scalar_static_bool[27]{v36}else{v1});
        let v3142=(v1029*v3139);
        let v3144=(v1029*v3140);
        let v3146=(v1029*v3141);
        let v3148=(v571*v1032);
        let v3162=(if self.scalar_static_bool[27]{((v139*(v3139-(if self.scalar_static_bool[27]{((v3142+v3142)/v3148)}else{v1})))-v2818)}else{v3026});
        let v3163=(if self.scalar_static_bool[27]{(v139*(v3140-(if self.scalar_static_bool[27]{((v3144+v3144)/v3148)}else{v1})))}else{v3027});
        let v3164=(if self.scalar_static_bool[27]{(v139*(v3141-(if self.scalar_static_bool[27]{((v3146+v3146)/v3148)}else{v1})))}else{v3028});
        let v3175=(self.scalar_static_f64[166]*f64::powf(v1039,self.scalar_static_f64[259]));
        let v3201=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{(((v1040*v2817)+(v864*((-(((v634*v3162)-(v1037*v2495))/v2545))*v3175)))/self.scalar_static_f64[166])}else{v3054})+(self.scalar_static_f64[177]*(v3125+(-v3162))))-v3138)}else{(if self.scalar_static_bool[25]{((v3054+(if self.scalar_static_bool[25]{((v1008*(if self.scalar_static_bool[25]{(((v1002*v3067)+(v997*(-v3060)))+((v1001*v3060)+(v994*v3075)))}else{v1}))+(v1006*(v2950+(-v3026))))}else{v1}))-v2964)}else{(if (self.scalar_static_f64[163]!=0.0){(v2910+(if v898{v1}else{(if v873{(v876*((v887*v2819)+(v870*((v2828-(v885*v2495))/v2545))))}else{v1})}))}else{v1})})});
        let v3202=(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v864*((-(v3163/v634))*v3175))/self.scalar_static_f64[166])}else{v3055})+(self.scalar_static_f64[177]*(v2641-v3163)))}else{(if self.scalar_static_bool[25]{(v3055+(if self.scalar_static_bool[25]{((v1008*(if self.scalar_static_bool[25]{((v997*(-v3061))+(v1001*v3061))}else{v1}))+(v1006*(v2641-v3027)))}else{v1}))}else{(if (self.scalar_static_f64[163]!=0.0){(v2911+(if v898{v1}else{(if v873{(v876*((v887*v2820)+(v870*v2832)))}else{v1})}))}else{v1})})});
        let v3203=(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v864*((-(v3164/v634))*v3175))/self.scalar_static_f64[166])}else{v3056})+(self.scalar_static_f64[177]*(v36-v3164)))}else{(if self.scalar_static_bool[25]{(v3056+(if self.scalar_static_bool[25]{((v1008*(if self.scalar_static_bool[25]{((v997*(-v3062))+(v1001*v3062))}else{v1}))+(v1006*(v36-v3028)))}else{v1}))}else{(if (self.scalar_static_f64[163]!=0.0){(v2912+(if v898{v1}else{(if v873{(v876*((v887*v2821)+(v870*v2833)))}else{v1})}))}else{v1})})});
        let v3209=((-((v548*v2173)+(v409*(self.scalar_static_f64[40]*v2393))))/(v1051*v1051));
        let v3211=(v36*v1052);
        let v3212=(v1052*v2641);
        let v3227=(if v1058{((v1063*(v1060*(v162*v3209)))+(v1060*(v1061*v3209)))}else{(if (v1054!=0.0){(v1056*(v744*v3209))}else{v1})});
        let v3228=(if v1058{(v1060*v3211)}else{(if (v1054!=0.0){(v1056*v3211)}else{v1})});
        let v3229=(if v1058{(v1060*v3212)}else{(if (v1054!=0.0){(v1056*v3212)}else{v1})});
        let v3232=((v1066*v2256)+(v484*v3227));
        let v3233=(v484*v3228);
        let v3234=(v484*v3229);
        let v3240=((-((v549*v2173)+(v409*(self.scalar_static_f64[52]*v2393))))/(v1068*v1068));
        let v3242=(v1069*v2641);
        let v3243=(v36*v1069);
        let v3259=(if v1075{((v1080*(v1077*(v201*v3240)))+(v1077*(v1078*v3240)))}else{(if (v1071!=0.0){(v1073*(v750*v3240))}else{v3227})});
        let v3260=(if v1075{(v1077*v3242)}else{(if (v1071!=0.0){(v1073*v3242)}else{v1})});
        let v3261=(if v1075{(v1077*v3243)}else{(if (v1071!=0.0){(v1073*v3243)}else{v3228})});
        let v3262=(if v1075{v1}else{(if (v1071!=0.0){v1}else{v3229})});
        let v3268=((v1084*((v491*v2256)+(v484*((v490*(self.scalar_static_f64[50]*(v2174*(self.scalar_static_f64[53]*f64::powf(v410,self.scalar_static_f64[246])))))+(v486*(v490*(((v488*(self.scalar_static_f64[55]*v2245))-(v487*(self.scalar_static_f64[52]*v2173)))/(v488*v488))))))))+(v1083*v3259));
        let v3269=(v1083*v3260);
        let v3270=(v1083*v3261);
        let v3271=(v1083*v3262);
        let v3276=(v734*v2816);
        let v3280=(v731*v3202);
        let v3282=(((v863*(if v732{((-(self.scalar_static_f64[150]*(self.scalar_static_f64[151]*v2171)))/(v702*v702))}else{v1}))+(v734*v2814))+((v1050*(if v729{((-(self.scalar_static_f64[148]*(self.scalar_static_f64[149]*v2171)))/(v697*v697))}else{v1}))+(v731*v3201)));
        let v3283=((v734*v2815)+(v731*v3203));
        let v3284=(v1091*v3282);
        let v3286=(v1091*v3280);
        let v3288=(v1091*v3283);
        let v3290=(v1091*v3276);
        let v3292=(v571*v1095);
        let v3301=(v139*(v3282+((v3284+v3284)/v3292)));
        let v3302=(v139*(v3280+((v3286+v3286)/v3292)));
        let v3303=(v139*(v3283+((v3288+v3288)/v3292)));
        let v3304=(v139*(v3276+((v3290+v3290)/v3292)));
        let v3319=(self.scalar_static_f64[180]*f64::powf(v1098,self.scalar_static_f64[261]));
        let v3324=(v141*(((v1067*(if v735{((-(self.scalar_static_f64[44]*(v2174*(self.scalar_static_f64[97]*f64::powf(v410,self.scalar_static_f64[235])))))/(v414*v414))}else{v1}))+(v737*v3232))+(v44*v3268)));
        let v3325=(v141*(v44*v3269));
        let v3326=(v141*((v737*v3233)+(v44*v3270)));
        let v3327=(v141*((v737*v3234)+(v44*v3271)));
        let v3332=(if (self.scalar_static_f64[179]!=0.0){((v3301*v3319)+v3324)}else{v1});
        let v3333=(if (self.scalar_static_f64[179]!=0.0){((v3302*v3319)+v3325)}else{v1});
        let v3334=(if (self.scalar_static_f64[179]!=0.0){((v3303*v3319)+v3326)}else{v1});
        let v3335=(if (self.scalar_static_f64[179]!=0.0){((v3304*v3319)+v3327)}else{v1});
        let v3338=(self.scalar_static_f64[46]*f64::powf(v1109,self.scalar_static_f64[262]));
        let v3355=(v139*v3301);
        let v3356=(v139*v3302);
        let v3357=(v139*v3303);
        let v3358=(v139*v3304);
        let v3363=(if self.scalar_static_bool[29]{v3324}else{v3332});
        let v3364=(if self.scalar_static_bool[29]{v3325}else{v3333});
        let v3365=(if self.scalar_static_bool[29]{v3326}else{v3334});
        let v3366=(if self.scalar_static_bool[29]{v3327}else{v3335});
        let v3368=(self.scalar_static_f64[46]*f64::powf(v1125,self.scalar_static_f64[262]));
        let v3393=(if v1135{(self.scalar_static_f64[182]*v3355)}else{(if v1128{((v1131*v3355)+(v1129*(v3363*v3368)))}else{(if v1118{v3355}else{(if v1112{(v139*(v3301+(v3332*v3338)))}else{v1})})})});
        let v3394=(if v1135{(self.scalar_static_f64[182]*v3356)}else{(if v1128{((v1131*v3356)+(v1129*(v3364*v3368)))}else{(if v1118{v3356}else{(if v1112{(v139*(v3302+(v3333*v3338)))}else{v1})})})});
        let v3395=(if v1135{(self.scalar_static_f64[182]*v3357)}else{(if v1128{((v1131*v3357)+(v1129*(v3365*v3368)))}else{(if v1118{v3357}else{(if v1112{(v139*(v3303+(v3334*v3338)))}else{v1})})})});
        let v3396=(if v1135{(self.scalar_static_f64[182]*v3358)}else{(if v1128{((v1131*v3358)+(v1129*(v3366*v3368)))}else{(if v1118{v3358}else{(if v1112{(v139*(v3304+(v3335*v3338)))}else{v1})})})});
        let v3400=(v1138*v1138);
        let v3431=(if (self.scalar_static_f64[183]!=0.0){((-v2279)/v2283)}else{v3240});
        let v3433=(v36*v1144);
        let v3434=(v1144*v2641);
        let v3445=(v1154*(v235*v3431));
        let v3450=(v1154*v3433);
        let v3451=(v1154*v3434);
        let v3452=(if v1152{((v1157*v3445)+(v1154*(v1155*v3431)))}else{(if v1147{(v1149*(v758*v3431))}else{v3259})});
        let v3453=(if v1152{v1}else{(if v1147{v1}else{v3260})});
        let v3454=(if v1152{v3450}else{(if v1147{(v1149*v3433)}else{v1})});
        let v3455=(if v1152{v1}else{(if v1147{v1}else{v3261})});
        let v3456=(if v1152{v1}else{(if v1147{v1}else{v3262})});
        let v3457=(if v1152{v3451}else{(if v1147{(v1149*v3434)}else{v1})});
        let v3469=(if v1167{((v1170*v3445)+(v1154*(v1168*v3431)))}else{(if v1162{(v1164*(v750*v3431))}else{v1})});
        let v3470=(if v1167{v3451}else{(if v1162{(v1164*v3434)}else{v1})});
        let v3471=(if v1167{v3450}else{(if v1162{(v1164*v3433)}else{v1})});
        let v3492=(if (self.scalar_static_f64[183]!=0.0){((v1178*v2288)+(v498*((self.scalar_static_f64[184]*v3452)+(self.scalar_static_f64[185]*v3469))))}else{v1});
        let v3493=(if (self.scalar_static_f64[183]!=0.0){(v498*((self.scalar_static_f64[184]*v3453)+(self.scalar_static_f64[185]*v3470)))}else{v1});
        let v3494=(if (self.scalar_static_f64[183]!=0.0){(v498*(self.scalar_static_f64[184]*v3454))}else{v1});
        let v3495=(if (self.scalar_static_f64[183]!=0.0){(v498*((self.scalar_static_f64[184]*v3455)+(self.scalar_static_f64[185]*v3471)))}else{v1});
        let v3496=(if (self.scalar_static_f64[183]!=0.0){(v498*(self.scalar_static_f64[184]*v3456))}else{v1});
        let v3497=(if (self.scalar_static_f64[183]!=0.0){(v498*(self.scalar_static_f64[184]*v3457))}else{v1});
        let v3562=(if v1204{((v1207*v3445)+(v1154*(v1205*v3431)))}else{(if v1199{(v1201*(v775*v3431))}else{v3452})});
        let v3563=(if v1204{v1}else{(if v1199{v1}else{v3453})});
        let v3564=(if v1204{v1}else{(if v1199{v1}else{v3454})});
        let v3565=(if v1204{v1}else{(if v1199{v1}else{v3455})});
        let v3566=(if v1204{v1}else{(if v1199{v1}else{v3456})});
        let v3567=(if v1204{v3451}else{(if v1199{(v1201*v3434)}else{v3457})});
        let v3568=(if v1204{v3450}else{(if v1199{(v1201*v3433)}else{v1})});
        let v3645=((-v2295)/v2299);
        let v3646=(if (self.scalar_static_f64[187]!=0.0){v3645}else{v3431});
        let v3648=(v36*v1224);
        let v3649=(v1224*v2641);
        let v3668=(if v1232{((v1237*(v1234*(v257*v3646)))+(v1234*(v1235*v3646)))}else{(if v1227{(v1229*(v744*v3646))}else{v3562})});
        let v3669=(if v1232{v1}else{(if v1227{v1}else{v3563})});
        let v3670=(if v1232{v1}else{(if v1227{v1}else{v3564})});
        let v3671=(if v1232{(v1234*v3648)}else{(if v1227{(v1229*v3648)}else{v3565})});
        let v3672=(if v1232{(v1234*v3649)}else{(if v1227{(v1229*v3649)}else{v3566})});
        let v3673=(if v1232{v1}else{(if v1227{v1}else{v3567})});
        let v3674=(if v1232{v1}else{(if v1227{v1}else{v3568})});
        let v3676=((-v2311)/v2315);
        let v3677=(if (self.scalar_static_f64[187]!=0.0){v3676}else{v3646});
        let v3757=(if self.scalar_static_bool[38]{v2579}else{v1});
        let v3758=(if self.scalar_static_bool[38]{v2641}else{v1});
        let v3759=(if self.scalar_static_bool[38]{v36}else{v1});
        let v3761=((-v2582)/v2586);
        let v3762=(if self.scalar_static_bool[38]{v3761}else{v3677});
        let v3763=(v1282*v3757);
        let v3766=(v1282*v3758);
        let v3767=(v1282*v3759);
        let v3784=(if v1290{((v1295*(v1292*(v117*v3762)))+(v1292*(v3763+(v1293*v3762))))}else{(if v1285{(v1287*(v3763+(v1280*v3762)))}else{v3469})});
        let v3785=(if v1290{v1}else{(if v1285{v1}else{v3470})});
        let v3786=(if v1290{(v1292*v3766)}else{(if v1285{(v1287*v3766)}else{v3471})});
        let v3787=(if v1290{(v1292*v3767)}else{(if v1285{(v1287*v3767)}else{v1})});
        let v3808=(if self.scalar_static_bool[41]{v3645}else{v3762});
        let v3810=(v36*v1307);
        let v3811=(v1307*v2641);
        let v3830=(if v1315{((v1320*(v1317*(v257*v3808)))+(v1317*(v1318*v3808)))}else{(if v1310{(v1312*(v747*v3808))}else{v3668})});
        let v3831=(if v1315{v1}else{(if v1310{v1}else{v3669})});
        let v3832=(if v1315{(v1317*v3810)}else{(if v1310{(v1312*v3810)}else{v3670})});
        let v3833=(if v1315{v1}else{(if v1310{v1}else{v3671})});
        let v3834=(if v1315{(v1317*v3811)}else{(if v1310{(v1312*v3811)}else{v3672})});
        let v3835=(if v1315{v1}else{(if v1310{v1}else{v3673})});
        let v3836=(if v1315{v1}else{(if v1310{v1}else{v3674})});
        let v3837=(if self.scalar_static_bool[41]{v3676}else{v3808});
        let v3886=(if self.scalar_static_bool[42]{v2579}else{v3757});
        let v3887=(if self.scalar_static_bool[42]{v2641}else{v3758});
        let v3888=(if self.scalar_static_bool[42]{v36}else{v3759});
        let v3889=(if self.scalar_static_bool[42]{v3761}else{v3837});
        let v3890=(v1347*v3886);
        let v3893=(v1347*v3887);
        let v3894=(v1347*v3888);
        let v3911=(if v1355{((v1360*(v1357*(v117*v3889)))+(v1357*(v3890+(v1358*v3889))))}else{(if v1350{(v1352*(v3890+(v1346*v3889)))}else{v3784})});
        let v3912=(if v1355{v1}else{(if v1350{v1}else{v3785})});
        let v3913=(if v1355{(v1357*v3893)}else{(if v1350{(v1352*v3893)}else{v3786})});
        let v3914=(if v1355{(v1357*v3894)}else{(if v1350{(v1352*v3894)}else{v3787})});
        let v3928=(if self.scalar_static_bool[44]{v3645}else{v3889});
        let v3930=(v36*v1369);
        let v3931=(v1369*v2641);
        let v3950=(if v1374{((v1378*(v1376*(v257*v3928)))+(v1376*(v1235*v3928)))}else{(if v1370{(v1372*(v744*v3928))}else{v3830})});
        let v3951=(if v1374{v1}else{(if v1370{v1}else{v3831})});
        let v3952=(if v1374{v1}else{(if v1370{v1}else{v3832})});
        let v3953=(if v1374{(v1376*v3930)}else{(if v1370{(v1372*v3930)}else{v3833})});
        let v3954=(if v1374{(v1376*v3931)}else{(if v1370{(v1372*v3931)}else{v3834})});
        let v3955=(if v1374{v1}else{(if v1370{v1}else{v3835})});
        let v3956=(if v1374{v1}else{(if v1370{v1}else{v3836})});
        let v3957=(if self.scalar_static_bool[44]{v3676}else{v3928});
        let v4046=(if self.scalar_static_bool[47]{v2579}else{v3886});
        let v4047=(if self.scalar_static_bool[47]{v2641}else{v3887});
        let v4048=(if self.scalar_static_bool[47]{v36}else{v3888});
        let v4049=(if self.scalar_static_bool[47]{v3761}else{v3957});
        let v4050=(v1408*v4046);
        let v4053=(v1408*v4047);
        let v4054=(v1408*v4048);
        let v4071=(if v1416{((v1421*(v1418*(v117*v4049)))+(v1418*(v4050+(v1419*v4049))))}else{(if v1411{(v1413*(v4050+(v1407*v4049)))}else{v3911})});
        let v4072=(if v1416{v1}else{(if v1411{v1}else{v3912})});
        let v4073=(if v1416{(v1418*v4053)}else{(if v1411{(v1413*v4053)}else{v3913})});
        let v4074=(if v1416{(v1418*v4054)}else{(if v1411{(v1413*v4054)}else{v3914})});
        let v4088=(if self.scalar_static_bool[44]{v3645}else{v4049});
        let v4090=(v36*v1429);
        let v4091=(v1429*v2641);
        let v4110=(if v1434{((v1438*(v1436*(v257*v4088)))+(v1436*(v1318*v4088)))}else{(if v1430{(v1432*(v747*v4088))}else{v3950})});
        let v4111=(if v1434{v1}else{(if v1430{v1}else{v3951})});
        let v4112=(if v1434{(v1436*v4090)}else{(if v1430{(v1432*v4090)}else{v3952})});
        let v4113=(if v1434{v1}else{(if v1430{v1}else{v3953})});
        let v4114=(if v1434{(v1436*v4091)}else{(if v1430{(v1432*v4091)}else{v3954})});
        let v4115=(if v1434{v1}else{(if v1430{v1}else{v3955})});
        let v4116=(if v1434{v1}else{(if v1430{v1}else{v3956})});
        let v4117=(if self.scalar_static_bool[44]{v3676}else{v4088});
        let v4176=(if self.scalar_static_bool[47]{v3761}else{v4117});
        let v4177=(v1462*(if self.scalar_static_bool[47]{v2579}else{v4046}));
        let v4180=(v1462*(if self.scalar_static_bool[47]{v2641}else{v4047}));
        let v4181=(v1462*(if self.scalar_static_bool[47]{v36}else{v4048}));
        let v4198=(if v1470{((v1475*(v1472*(v117*v4176)))+(v1472*(v4177+(v1473*v4176))))}else{(if v1465{(v1467*(v4177+(v1461*v4176)))}else{v4071})});
        let v4199=(if v1470{v1}else{(if v1465{v1}else{v4072})});
        let v4200=(if v1470{(v1472*v4180)}else{(if v1465{(v1467*v4180)}else{v4073})});
        let v4201=(if v1470{(v1472*v4181)}else{(if v1465{(v1467*v4181)}else{v4074})});
        let v4216=((-v2327)/v2331);
        let v4218=(v1483*v2641);
        let v4219=(v36*v1483);
        let v4238=(if v1489{((v1494*(v1491*(v300*v4216)))+(v1491*(v1492*v4216)))}else{(if (v1485!=0.0){(v1487*(v750*v4216))}else{v4110})});
        let v4239=(if v1489{(v1491*v4218)}else{(if (v1485!=0.0){(v1487*v4218)}else{v4111})});
        let v4240=(if v1489{v1}else{(if (v1485!=0.0){v1}else{v4112})});
        let v4241=(if v1489{(v1491*v4219)}else{(if (v1485!=0.0){(v1487*v4219)}else{v4113})});
        let v4242=(if v1489{v1}else{(if (v1485!=0.0){v1}else{v4114})});
        let v4243=(if v1489{v1}else{(if (v1485!=0.0){v1}else{v4115})});
        let v4244=(if v1489{v1}else{(if (v1485!=0.0){v1}else{v4116})});
        let v4246=((-v2343)/v2347);
        let v4292=(if (self.scalar_static_f64[195]!=0.0){v4216}else{v4246});
        let v4294=(v36*v1520);
        let v4295=(v1520*v2641);
        let v4314=(if v1528{((v1533*(v1530*(v333*v4292)))+(v1530*(v1531*v4292)))}else{(if v1523{(v1525*(v758*v4292))}else{v4238})});
        let v4315=(if v1528{v1}else{(if v1523{v1}else{v4239})});
        let v4316=(if v1528{(v1530*v4294)}else{(if v1523{(v1525*v4294)}else{v4240})});
        let v4317=(if v1528{v1}else{(if v1523{v1}else{v4241})});
        let v4318=(if v1528{v1}else{(if v1523{v1}else{v4242})});
        let v4319=(if v1528{(v1530*v4295)}else{(if v1523{(v1525*v4295)}else{v4243})});
        let v4320=(if v1528{v1}else{(if v1523{v1}else{v4244})});
        let v4321=(if (self.scalar_static_f64[195]!=0.0){v4246}else{v4292});
        let v4387=((-(v750*v2173))/v2417);
        let v4388=(v2641/v409);
        let v4389=(v36/v409);
        let v4401=(v1566*v4388);
        let v4402=(v1566*v4389);
        let v4403=(if v1565{(v1566*v4387)}else{(if (v1562!=0.0){(v1563*v4387)}else{v4314})});
        let v4404=(if v1565{v4401}else{(if (v1562!=0.0){(v1563*v4388)}else{v4315})});
        let v4405=(if v1565{v1}else{(if (v1562!=0.0){v1}else{v4316})});
        let v4406=(if v1565{v4402}else{(if (v1562!=0.0){(v1563*v4389)}else{v4317})});
        let v4407=(if v1565{v1}else{(if (v1562!=0.0){v1}else{v4318})});
        let v4408=(if v1565{v1}else{(if (v1562!=0.0){v1}else{v4319})});
        let v4409=(if v1565{v1}else{(if (v1562!=0.0){v1}else{v4320})});
        let v4412=((-(v753*v2173))/v2417);
        let v4436=(v571*v1583);
        let v4437=(((v1570*v2573)+(v684*v4403))/v4436);
        let v4438=((v684*v4404)/v4436);
        let v4439=((v684*v4405)/v4436);
        let v4440=((v684*v4406)/v4436);
        let v4441=((v684*v4407)/v4436);
        let v4442=((v684*v4408)/v4436);
        let v4443=((v684*v4409)/v4436);
        let v4451=(v571*v1586);
        let v4452=(((v1580*v2573)+(v684*(if v1576{(v1566*v4412)}else{(if (v1573!=0.0){(v1574*v4412)}else{v4198})})))/v4451);
        let v4453=((v684*(if v1576{v4401}else{(if (v1573!=0.0){(v1574*v4388)}else{v1})}))/v4451);
        let v4454=((v684*(if v1576{v1}else{(if (v1573!=0.0){v1}else{v4199})}))/v4451);
        let v4455=((v684*(if v1576{v4402}else{(if (v1573!=0.0){(v1574*v4389)}else{v4200})}))/v4451);
        let v4456=((v684*(if v1576{v1}else{(if (v1573!=0.0){v1}else{v4201})}))/v4451);
        let v4936=(if (self.scalar_static_f64[213]!=0.0){((-v2367)/v2371)}else{v4321});
        let v4938=(v1750*v2641);
        let v4939=(v36*v1750);
        let v4958=(if v1758{((v1763*(v1760*(v366*v4936)))+(v1760*(v1761*v4936)))}else{(if v1753{(v1755*(v775*v4936))}else{v4403})});
        let v4959=(if v1758{v1}else{(if v1753{v1}else{v4404})});
        let v4960=(if v1758{v1}else{(if v1753{v1}else{v4405})});
        let v4961=(if v1758{v1}else{(if v1753{v1}else{v4406})});
        let v4962=(if v1758{v1}else{(if v1753{v1}else{v4407})});
        let v4963=(if v1758{(v1760*v4938)}else{(if v1753{(v1755*v4938)}else{v4408})});
        let v4964=(if v1758{(v1760*v4939)}else{(if v1753{(v1755*v4939)}else{v4409})});
        let v5318=(-v2533);
        let v5320=(if (self.scalar_static_f64[216]!=0.0){(self.scalar_static_f64[152]*v5318)}else{v1});
        let v5321=(if self.scalar_static_bool[70]{v5320}else{v1});
        let v5322=(if self.scalar_static_bool[70]{v2641}else{v1});
        let v5323=(if self.scalar_static_bool[70]{v36}else{v1});
        let v5330=(self.scalar_static_f64[155]*v2533);
        let v5334=(v1871*v1871);
        let v5363=(self.scalar_static_f64[221]*f64::powf(v1880,self.scalar_static_f64[268]));
        let v5378=(if v1878{(((v1882*v2533)+(v661*(-((-((-(v775*v2533))/v2554))*v5363))))/self.scalar_static_f64[221])}else{(if v1859{((v1864*v2533)/self.scalar_static_f64[221])}else{v1})});
        let v5379=(if v1878{((v661*(-((-(v2641/v661))*v5363)))/self.scalar_static_f64[221])}else{v1});
        let v5380=(if v1878{((v661*(-((-(v36/v661))*v5363)))/self.scalar_static_f64[221])}else{v1});
        let v5390=(v1850*v5320);
        let v5397=(if self.scalar_static_bool[72]{(v579*(v5320+(if self.scalar_static_bool[72]{((v5390+v5390)/(v571*v1895))}else{v1})))}else{v1});
        let v5411=(if self.scalar_static_bool[72]{v5320}else{v1});
        let v5412=(if self.scalar_static_bool[72]{v2641}else{v1});
        let v5413=(if self.scalar_static_bool[72]{v36}else{v1});
        let v5414=(v1906*v5411);
        let v5416=(v1906*v5412);
        let v5418=(v1906*v5413);
        let v5420=(v571*v1909);
        let v5434=(if self.scalar_static_bool[72]{((v139*(v5411-(if self.scalar_static_bool[72]{((v5414+v5414)/v5420)}else{v1})))-v5320)}else{v1});
        let v5435=(if self.scalar_static_bool[72]{(v139*(v5412-(if self.scalar_static_bool[72]{((v5416+v5416)/v5420)}else{v1})))}else{v1});
        let v5436=(if self.scalar_static_bool[72]{(v139*(v5413-(if self.scalar_static_bool[72]{((v5418+v5418)/v5420)}else{v1})))}else{v1});
        let v5447=(self.scalar_static_f64[221]*f64::powf(v1916,self.scalar_static_f64[268]));
        let v5463=(v2641-v5435);
        let v5464=(v36-v5436);
        let v5465=(v5397+(-v5434));
        let v5523=(self.scalar_static_f64[158]*f64::powf(v1953,self.scalar_static_f64[258]));
        let v5538=(if v1951{(((v1955*v2457)+(v607*(-((-((-(v747*v2457))/v2536))*v5523))))/self.scalar_static_f64[158])}else{(if v1937{((v1940*v2457)/self.scalar_static_f64[158])}else{v1})});
        let v5539=(if v1951{((v607*(-(v2682*v5523)))/self.scalar_static_f64[158])}else{v1});
        let v5540=(if v1951{((v607*(-(v2683*v5523)))/self.scalar_static_f64[158])}else{v1});
        let v5550=(v1962*v2734);
        let v5552=(v1962*v2735);
        let v5554=(v1962*v2736);
        let v5556=(v571*v1965);
        let v5570=(if self.scalar_static_bool[19]{((v139*(v2734-(if self.scalar_static_bool[19]{((v5550+v5550)/v5556)}else{v1})))-v2643)}else{v1});
        let v5571=(if self.scalar_static_bool[19]{(v139*(v2735-(if self.scalar_static_bool[19]{((v5552+v5552)/v5556)}else{v1})))}else{v1});
        let v5572=(if self.scalar_static_bool[19]{(v139*(v2736-(if self.scalar_static_bool[19]{((v5554+v5554)/v5556)}else{v1})))}else{v1});
        let v5583=(self.scalar_static_f64[158]*f64::powf(v1972,self.scalar_static_f64[258]));
        let v5599=(v36-v5571);
        let v5600=(v2641-v5572);
        let v5601=(v2720+(-v5570));
        let v5671=(self.scalar_static_f64[166]*f64::powf(v2023,self.scalar_static_f64[259]));
        let v5686=(if v2021{(((v2025*v2495)+(v634*(-((-((-(v758*v2495))/v2545))*v5671))))/self.scalar_static_f64[166])}else{(if v2010{(((v2016*v2495)+(v634*(-((v2014*v2855)+(v902*(-((-(v2012*v2495))/v2860)))))))/self.scalar_static_f64[166])}else{(if v1991{((v1995*v2495)/self.scalar_static_f64[166])}else{v1})})});
        let v5687=(if v2021{((v634*(-(v2893*v5671)))/self.scalar_static_f64[166])}else{(if v2010{v2882}else{v1})});
        let v5688=(if v2021{((v634*(-(v2892*v5671)))/self.scalar_static_f64[166])}else{(if v2010{v2881}else{v1})});
        let v5701=(if self.scalar_static_bool[25]{((v2923-(v2034*v2922))/v2926)}else{v1});
        let v5703=(v2038*v5701);
        let v5705=(v2038*v2974);
        let v5707=(v2038*v2973);
        let v5709=(v571*v2041);
        let v5713=(v2042*v5701);
        let v5715=(v2042*v2974);
        let v5717=(v2042*v2973);
        let v5719=(v571*v2045);
        let v5729=(v2046*v2046);
        let v5739=(if self.scalar_static_bool[25]{(((v2046*(v571*v5701))-(v2037*(((v5703+v5703)/v5709)+((v5713+v5713)/v5719))))/v5729)}else{v1});
        let v5740=(if self.scalar_static_bool[25]{(((v2046*v2977)-(v2037*(((v5705+v5705)/v5709)+((v5715+v5715)/v5719))))/v5729)}else{v1});
        let v5741=(if self.scalar_static_bool[25]{(((v2046*v2976)-(v2037*(((v5707+v5707)/v5709)+((v5717+v5717)/v5719))))/v5729)}else{v1});
        let v5751=(if self.scalar_static_bool[25]{(v139*(((v2048*v2922)+(v932*v5739))-v2818))}else{v1});
        let v5752=(if self.scalar_static_bool[25]{(v139*(v932*v5740))}else{v1});
        let v5753=(if self.scalar_static_bool[25]{(v139*(v932*v5741))}else{v1});
        let v5764=(self.scalar_static_f64[166]*f64::powf(v2055,self.scalar_static_f64[259]));
        let v5779=(if self.scalar_static_bool[25]{(((v2057*v2495)+(v634*(-((-(((v634*v5751)-(v2053*v2495))/v2545))*v5764))))/self.scalar_static_f64[166])}else{v5686});
        let v5780=(if self.scalar_static_bool[25]{((v634*(-((-(v5752/v634))*v5764)))/self.scalar_static_f64[166])}else{v5687});
        let v5781=(if self.scalar_static_bool[25]{((v634*(-((-(v5753/v634))*v5764)))/self.scalar_static_f64[166])}else{v5688});
        let v5785=(if self.scalar_static_bool[25]{(v139*v5739)}else{v1});
        let v5786=(if self.scalar_static_bool[25]{(v139*v5740)}else{v1});
        let v5787=(if self.scalar_static_bool[25]{(v139*v5741)}else{v1});
        let v5830=(v2076*v3139);
        let v5832=(v2076*v3141);
        let v5834=(v2076*v3140);
        let v5836=(v571*v2079);
        let v5850=(if self.scalar_static_bool[27]{((v139*(v3139-(if self.scalar_static_bool[27]{((v5830+v5830)/v5836)}else{v1})))-v2818)}else{v5751});
        let v5851=(if self.scalar_static_bool[27]{(v139*(v3141-(if self.scalar_static_bool[27]{((v5832+v5832)/v5836)}else{v1})))}else{v5752});
        let v5852=(if self.scalar_static_bool[27]{(v139*(v3140-(if self.scalar_static_bool[27]{((v5834+v5834)/v5836)}else{v1})))}else{v5753});
        let v5863=(self.scalar_static_f64[166]*f64::powf(v2086,self.scalar_static_f64[259]));
        let v5895=(v64*(v2098*v3232));
        let v5896=(v64*(v2098*v3233));
        let v5897=(v64*(v2098*v3234));
        let v5901=(v2101*v2101);
        let v5913=((v59*v2641)/v2104);
        let v5914=((v36*v59)/v2104);
        let v5948=(v2102*(((v2101*v5895)-(v2100*v5895))/v5901));
        let v5950=(v2102*(((v2101*v5896)-(v2100*v5896))/v5901));
        let v5952=(v2102*(((v2101*v5897)-(v2100*v5897))/v5901));
        let v6094=(v36*((self.scalar_static_f64[186]*((v863*v2542)+(v666*v2814)))+(((v1138*((v2127*v3232)+(v1067*((v2126*(self.scalar_static_f64[225]*(self.scalar_static_f64[226]*v3301)))+(v2119*(v2098*((v2123*(self.scalar_static_f64[227]*(if v2110{v1}else{(if (v2107!=0.0){v1}else{v4958})})))+(v2121*(v5948+v5948)))))))))-(v2130*v3393))/v3400)));
        let v6095=(v36*(((v1138*(v1067*((v2126*(self.scalar_static_f64[225]*(self.scalar_static_f64[226]*v3302)))+(v2119*(v2098*(v2123*(self.scalar_static_f64[227]*(if v2110{(v1566*v5913)}else{(if (v2107!=0.0){(v2108*v5913)}else{v4959})}))))))))-(v2130*v3394))/v3400));
        let v6096=(v36*((v1067*(v2119*(v2098*(v2123*(self.scalar_static_f64[227]*(if v2110{v1}else{(if (v2107!=0.0){v1}else{v4960})}))))))/v1138));
        let v6097=(v36*((self.scalar_static_f64[186]*(v666*v2815))+(((v1138*((v2127*v3233)+(v1067*((v2126*(self.scalar_static_f64[225]*(self.scalar_static_f64[226]*v3303)))+(v2119*(v2098*((v2123*(self.scalar_static_f64[227]*(if v2110{(v1566*v5914)}else{(if (v2107!=0.0){(v2108*v5914)}else{v4961})})))+(v2121*(v5950+v5950)))))))))-(v2130*v3395))/v3400)));
        let v6098=(v36*((self.scalar_static_f64[186]*(v666*v2816))+(((v1138*((v2127*v3234)+(v1067*((v2126*(self.scalar_static_f64[225]*(self.scalar_static_f64[226]*v3304)))+(v2119*(v2098*((v2123*(self.scalar_static_f64[227]*(if v2110{v1}else{(if (v2107!=0.0){v1}else{v4962})})))+(v2121*(v5952+v5952)))))))))-(v2130*v3396))/v3400)));
        let v6099=(v36*((v1067*(v2119*(v2098*(v2123*(self.scalar_static_f64[227]*(if v2110{v1}else{(if (v2107!=0.0){v1}else{v4963})}))))))/v1138));
        let v6100=(v36*((v1067*(v2119*(v2098*(v2123*(self.scalar_static_f64[227]*(if v2110{v1}else{(if (v2107!=0.0){v1}else{v4964})}))))))/v1138));
        let v6101=(v36*(self.scalar_static_f64[193]*((v1986*v2542)+(v666*(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{(((v1973*v2642)+(v782*((-(((v607*v5570)-(v1970*v2457))/v2536))*v5583)))/self.scalar_static_f64[158])}else{v5538})+((v1982*(self.scalar_static_f64[157]*v5601))+(v1979*(((v805*(self.scalar_static_f64[159]*v5601))-(v1980*v2653))/v2657))))-v2733)}else{(if (self.scalar_static_f64[154]!=0.0){(v5538+(if v1951{v1}else{(if v1937{(v1938*((v1946*v2644)+(v1934*((v2654-(v1944*v2653))/v2657))))}else{v1})}))}else{v1})})))));
        let v6102=(v36*(self.scalar_static_f64[193]*(v666*(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v782*((-(v5571/v607))*v5583))/self.scalar_static_f64[158])}else{v5539})+((v1982*(self.scalar_static_f64[157]*v5599))+(v1979*((self.scalar_static_f64[159]*v5599)/v805))))}else{(if (self.scalar_static_f64[154]!=0.0){(v5539+(if v1951{v1}else{(if v1937{(v1938*((v1946*v2645)+(v1934*v2659)))}else{v1})}))}else{v1})}))));
        let v6103=(v36*(self.scalar_static_f64[193]*(v666*(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v782*((-(v5572/v607))*v5583))/self.scalar_static_f64[158])}else{v5540})+((v1982*(self.scalar_static_f64[157]*v5600))+(v1979*((self.scalar_static_f64[159]*v5600)/v805))))}else{(if (self.scalar_static_f64[154]!=0.0){(v5540+(if v1951{v1}else{(if v1937{(v1938*((v1946*v2646)+(v1934*v2660)))}else{v1})}))}else{v1})}))));
        let v6104=(v36*((((v1050*(self.scalar_static_f64[140]*v2549))+(v671*v3201))+(self.scalar_static_f64[228]*v3268))+(self.scalar_static_f64[229]*v4437)));
        let v6105=(v36*(((v671*v3202)+(self.scalar_static_f64[228]*v3269))+(self.scalar_static_f64[229]*v4438)));
        let v6106=(v36*(self.scalar_static_f64[229]*v4439));
        let v6107=(v36*(((v671*v3203)+(self.scalar_static_f64[228]*v3270))+(self.scalar_static_f64[229]*v4440)));
        let v6108=(v36*((self.scalar_static_f64[228]*v3271)+(self.scalar_static_f64[229]*v4441)));
        let v6109=(v36*(self.scalar_static_f64[229]*v4442));
        let v6110=(v36*(self.scalar_static_f64[229]*v4443));
        let v6111=(v36*(self.scalar_static_f64[229]*v4452));
        let v6112=(v36*(self.scalar_static_f64[229]*v4453));
        let v6113=(v36*(self.scalar_static_f64[229]*v4454));
        let v6114=(v36*(self.scalar_static_f64[229]*v4455));
        let v6115=(v36*(self.scalar_static_f64[229]*v4456));
        let v6116=(v36*(((v2096*(self.scalar_static_f64[142]*v2549))+(v673*(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{(((v2087*v2817)+(v864*((-(((v634*v5850)-(v2084*v2495))/v2545))*v5863)))/self.scalar_static_f64[166])}else{v5779})+(self.scalar_static_f64[177]*(v3125+(-v5850))))-v3138)}else{(if self.scalar_static_bool[25]{((v5779+(if self.scalar_static_bool[25]{((v2070*(if self.scalar_static_bool[25]{(((v2064*v3067)+(v997*(-v5785)))+((v2063*v3075)+(v1001*v5785)))}else{v1}))+(v2068*(v2950+(-v5751))))}else{v1}))-v2964)}else{(if (self.scalar_static_f64[163]!=0.0){(v5686+(if v2009{v1}else{(if v1991{(v1992*((v2001*v2819)+(v1988*((v2828-(v1999*v2495))/v2545))))}else{v1})}))}else{v1})})})))+(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3492}))));
        let v6117=(v36*(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3493})));
        let v6118=(v36*((v673*(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v864*((-(v5851/v634))*v5863))/self.scalar_static_f64[166])}else{v5780})+(self.scalar_static_f64[177]*(v36-v5851)))}else{(if self.scalar_static_bool[25]{(v5780+(if self.scalar_static_bool[25]{((v2070*(if self.scalar_static_bool[25]{((v997*(-v5786))+(v1001*v5786))}else{v1}))+(v2068*(v36-v5752)))}else{v1}))}else{(if (self.scalar_static_f64[163]!=0.0){(v5687+(if v2009{v1}else{(if v1991{(v1992*((v2001*v2821)+(v1988*v2833)))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3494}))));
        let v6119=(v36*(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3495})));
        let v6120=(v36*(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3496})));
        let v6121=(v36*((v673*(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v864*((-(v5852/v634))*v5863))/self.scalar_static_f64[166])}else{v5781})+(self.scalar_static_f64[177]*(v2641-v5852)))}else{(if self.scalar_static_bool[25]{(v5781+(if self.scalar_static_bool[25]{((v2070*(if self.scalar_static_bool[25]{((v997*(-v5787))+(v1001*v5787))}else{v1}))+(v2068*(v2641-v5753)))}else{v1}))}else{(if (self.scalar_static_f64[163]!=0.0){(v5688+(if v2009{v1}else{(if v1991{(v1992*((v2001*v2820)+(v1988*v2832)))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[228]*(if self.scalar_static_bool[31]{v1}else{v3497}))));
        let v6122=(v36*((v1932*(self.scalar_static_f64[143]*(((-(self.scalar_static_f64[135]*v2533))/v2554)*(self.scalar_static_f64[144]*f64::powf(v675,self.scalar_static_f64[255])))))+(v678*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{(((if self.scalar_static_bool[72]{(((v1917*v5318)+(v1848*((-(((v661*v5434)-(v1914*v2533))/v2554))*v5447)))/self.scalar_static_f64[221])}else{v5378})+((v1926*(self.scalar_static_f64[220]*v5465))+(v1923*(((v1871*(self.scalar_static_f64[222]*v5465))-(v1924*v5330))/v5334))))-(if self.scalar_static_bool[72]{(((v1902*v5318)+(v1848*((-(((v661*v5397)-(v1899*v2533))/v2554))*(self.scalar_static_f64[221]*f64::powf(v1901,self.scalar_static_f64[268])))))/self.scalar_static_f64[221])}else{v1}))}else{(if self.scalar_static_bool[70]{(v5378+(if v1878{v1}else{(if v1859{(v1862*((v1873*v5321)+(v1856*(((v1871*(self.scalar_static_f64[222]*v5321))-(v1870*v5330))/v5334))))}else{v1})}))}else{v1})})}))));
        let v6123=(v36*((v678*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{((if self.scalar_static_bool[72]{((v1848*((-(v5435/v661))*v5447))/self.scalar_static_f64[221])}else{v5379})+((v1926*(self.scalar_static_f64[220]*v5463))+(v1923*((self.scalar_static_f64[222]*v5463)/v1871))))}else{(if self.scalar_static_bool[70]{(v5379+(if v1878{v1}else{(if v1859{(v1862*((v1873*v5322)+(v1856*((self.scalar_static_f64[222]*v5322)/v1871))))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[230]*v2641)));
        let v6124=(v36*((v678*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{((if self.scalar_static_bool[72]{((v1848*((-(v5436/v661))*v5447))/self.scalar_static_f64[221])}else{v5380})+((v1926*(self.scalar_static_f64[220]*v5464))+(v1923*((self.scalar_static_f64[222]*v5464)/v1871))))}else{(if self.scalar_static_bool[70]{(v5380+(if v1878{v1}else{(if v1859{(v1862*((v1873*v5323)+(v1856*((self.scalar_static_f64[222]*v5323)/v1871))))}else{v1})}))}else{v1})})}))+(v36*self.scalar_static_f64[230])));

        CommonStampValues {
            v0,
            v1,
            v2,
            v18,
            v26,
            v36,
            v39,
            v49,
            v100,
            v125,
            v139,
            v141,
            v388,
            v409,
            v410,
            v411,
            v479,
            v498,
            v502,
            v509,
            v516,
            v523,
            v534,
            v571,
            v634,
            v689,
            v690,
            v741,
            v742,
            v744,
            v745,
            v747,
            v748,
            v750,
            v751,
            v756,
            v758,
            v759,
            v760,
            v764,
            v773,
            v775,
            v780,
            v781,
            v1067,
            v1085,
            v1090,
            v1093,
            v1098,
            v1125,
            v1138,
            v1180,
            v1209,
            v1239,
            v1241,
            v1297,
            v1322,
            v1323,
            v1362,
            v1380,
            v1381,
            v1423,
            v1440,
            v1441,
            v1477,
            v1496,
            v1497,
            v1535,
            v1536,
            v1566,
            v1583,
            v1586,
            v1750,
            v1765,
            v2151,
            v2153,
            v2155,
            v2157,
            v2160,
            v2161,
            v2162,
            v2163,
            v2164,
            v2165,
            v2166,
            v2171,
            v2173,
            v2174,
            v2245,
            v2288,
            v2295,
            v2299,
            v2311,
            v2315,
            v2327,
            v2331,
            v2343,
            v2347,
            v2367,
            v2371,
            v2495,
            v2579,
            v2582,
            v2586,
            v2641,
            v3232,
            v3233,
            v3234,
            v3268,
            v3269,
            v3270,
            v3271,
            v3301,
            v3302,
            v3303,
            v3304,
            v3363,
            v3364,
            v3365,
            v3366,
            v3393,
            v3394,
            v3395,
            v3396,
            v3400,
            v3492,
            v3493,
            v3494,
            v3495,
            v3496,
            v3497,
            v3562,
            v3563,
            v3564,
            v3565,
            v3566,
            v3567,
            v3568,
            v3668,
            v3669,
            v3670,
            v3671,
            v3672,
            v3673,
            v3674,
            v3677,
            v3784,
            v3785,
            v3786,
            v3787,
            v3830,
            v3831,
            v3832,
            v3833,
            v3834,
            v3835,
            v3836,
            v3837,
            v3911,
            v3912,
            v3913,
            v3914,
            v3950,
            v3951,
            v3952,
            v3953,
            v3954,
            v3955,
            v3956,
            v3957,
            v4071,
            v4072,
            v4073,
            v4074,
            v4110,
            v4111,
            v4112,
            v4113,
            v4114,
            v4115,
            v4116,
            v4117,
            v4198,
            v4199,
            v4200,
            v4201,
            v4238,
            v4239,
            v4240,
            v4241,
            v4242,
            v4243,
            v4244,
            v4246,
            v4314,
            v4315,
            v4316,
            v4317,
            v4318,
            v4319,
            v4320,
            v4321,
            v4437,
            v4438,
            v4439,
            v4440,
            v4441,
            v4442,
            v4443,
            v4452,
            v4453,
            v4454,
            v4455,
            v4456,
            v4936,
            v4958,
            v4959,
            v4960,
            v4961,
            v4962,
            v4963,
            v4964,
            v6094,
            v6095,
            v6096,
            v6097,
            v6098,
            v6099,
            v6100,
            v6101,
            v6102,
            v6103,
            v6104,
            v6105,
            v6106,
            v6107,
            v6108,
            v6109,
            v6110,
            v6111,
            v6112,
            v6113,
            v6114,
            v6115,
            v6116,
            v6117,
            v6118,
            v6119,
            v6120,
            v6121,
            v6122,
            v6123,
            v6124,
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
        let v3=0.01;
        let v11=(if ((common.v0!=0.0)&&self.scalar_static_bool[0]){1e-12}else{(if ((common.v0!=0.0)&&(self.scalar_static_f64[0]!=0.0)){self.scalar_static_f64[1]}else{common.v1})});
        let v54=(if (common.v0!=0.0){self.scalar_static_f64[18]}else{common.v1});
        let v270=((self.scalar_static_f64[69]*f64::powf(common.v100,self.scalar_static_f64[72]))*(((common.v125*self.scalar_static_f64[74])/self.scalar_static_f64[296])).exp());
        let v272=(if (v270>common.v1){common.v2}else{common.v1});
        let v279=(if (!(v272!=0.0)){common.v1}else{(if (v272!=0.0){(self.scalar_static_f64[296]*((common.v2+(common.v18/v270))).ln())}else{common.v1})});
        let v304=f64::powf(common.v100,self.scalar_static_f64[82]);
        let v311=(((common.v125*self.scalar_static_f64[84])/self.scalar_static_f64[298])).exp();
        let v312=((self.scalar_static_f64[80]*v304)*v311);
        let v314=(if (v312>common.v1){common.v2}else{common.v1});
        let v321=(if (!(v314!=0.0)){common.v1}else{(if (v314!=0.0){(self.scalar_static_f64[298]*((common.v2+(common.v18/v312))).ln())}else{common.v1})});
        let v336=(v311*(v304*self.scalar_static_f64[86]));
        let v338=(if (v336>common.v1){common.v2}else{common.v1});
        let v345=(if (!(v338!=0.0)){common.v1}else{(if (v338!=0.0){(self.scalar_static_f64[298]*((common.v2+(common.v18/v336))).ln())}else{common.v1})});
        let v378=((self.scalar_static_f64[92]*f64::powf(common.v100,self.scalar_static_f64[94]))*(((common.v125*self.scalar_static_f64[96])/self.scalar_static_f64[300])).exp());
        let v380=(if (v378>common.v1){common.v2}else{common.v1});
        let v387=(if (!(v380!=0.0)){common.v1}else{(if (v380!=0.0){(self.scalar_static_f64[300]*((common.v2+(common.v18/v378))).ln())}else{common.v1})});
        let v423=f64::powf(common.v410,self.scalar_static_f64[101]);
        let v425=(if self.scalar_static_bool[13]{(self.scalar_static_f64[99]*v423)}else{(if (self.scalar_static_f64[98]!=0.0){(self.scalar_static_f64[99]*f64::powf(common.v410,self.scalar_static_f64[100]))}else{common.v1})});
        let v434=(if self.scalar_static_bool[14]{(v423*self.scalar_static_f64[103])}else{(if (self.scalar_static_f64[102]!=0.0){(self.scalar_static_f64[103]*f64::powf(common.v410,self.scalar_static_f64[104]))}else{common.v1})});
        let v443=f64::powf(common.v410,self.scalar_static_f64[108]);
        let v445=(if self.scalar_static_bool[15]{(self.scalar_static_f64[106]*v443)}else{(if (self.scalar_static_f64[105]!=0.0){(self.scalar_static_f64[106]*f64::powf(common.v410,self.scalar_static_f64[107]))}else{common.v1})});
        let v454=(if self.scalar_static_bool[16]{(v443*self.scalar_static_f64[110])}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[110]*f64::powf(common.v410,self.scalar_static_f64[111]))}else{common.v1})});
        let v458=(self.scalar_static_f64[112]*f64::powf(common.v410,self.scalar_static_f64[113]));
        let v462=(self.scalar_static_f64[114]*f64::powf(common.v410,self.scalar_static_f64[115]));
        let v471=(if self.scalar_static_bool[17]{(v423*self.scalar_static_f64[117])}else{(if (self.scalar_static_f64[116]!=0.0){(self.scalar_static_f64[117]*f64::powf(common.v410,self.scalar_static_f64[118]))}else{common.v1})});
        let v476=(self.scalar_static_f64[119]*(common.v2+(common.v411*self.scalar_static_f64[120])));
        let v500=(self.scalar_static_f64[63]*f64::powf(common.v410,self.scalar_static_f64[66]));
        let v501=(self.scalar_static_f64[68]*common.v479);
        let v504=((v501/common.v502)).exp();
        let v505=(v500*v504);
        let v507=(self.scalar_static_f64[69]*f64::powf(common.v410,self.scalar_static_f64[72]));
        let v508=(self.scalar_static_f64[74]*common.v479);
        let v511=((v508/common.v509)).exp();
        let v512=(v507*v511);
        let v513=f64::powf(common.v410,self.scalar_static_f64[77]);
        let v514=(self.scalar_static_f64[75]*v513);
        let v515=(self.scalar_static_f64[79]*common.v479);
        let v518=((v515/common.v516)).exp();
        let v519=(v514*v518);
        let v520=f64::powf(common.v410,self.scalar_static_f64[82]);
        let v521=(self.scalar_static_f64[80]*v520);
        let v522=(self.scalar_static_f64[84]*common.v479);
        let v525=((v522/common.v523)).exp();
        let v526=(v521*v525);
        let v527=(self.scalar_static_f64[85]*v513);
        let v528=(v518*v527);
        let v529=(self.scalar_static_f64[86]*v520);
        let v530=(v525*v529);
        let v532=(self.scalar_static_f64[87]*f64::powf(common.v410,self.scalar_static_f64[89]));
        let v533=(self.scalar_static_f64[91]*common.v479);
        let v536=((v533/common.v534)).exp();
        let v537=(v532*v536);
        let v539=(self.scalar_static_f64[92]*f64::powf(common.v410,self.scalar_static_f64[94]));
        let v540=(self.scalar_static_f64[96]*common.v479);
        let v541=(self.scalar_static_f64[93]*common.v409);
        let v543=((v540/v541)).exp();
        let v544=(v539*v543);
        let v554=(self.scalar_static_f64[122]*(common.v2+(common.v411*self.scalar_static_f64[123])));
        let v559=(self.scalar_static_f64[124]*(common.v2+(common.v411*self.scalar_static_f64[125])));
        let v688=(self.scalar_static_f64[146]*f64::powf(common.v410,self.scalar_static_f64[147]));
        let v692=((common.v689/common.v690)).exp();
        let v703=0.001;
        let v704=(v425>v703);
        let v706=1000.0;
        let v707=(if v704{(common.v2/v425)}else{v706});
        let v708=(v434>v703);
        let v710=(if v708{(common.v2/v434)}else{v706});
        let v711=(v445>v703);
        let v713=(if v711{(common.v2/v445)}else{v706});
        let v714=(v454>v703);
        let v716=(if v714{(common.v2/v454)}else{v706});
        let v717=(v458>v703);
        let v719=(if v717{(common.v2/v458)}else{v706});
        let v720=(v471>v703);
        let v722=(if v720{(common.v2/v471)}else{v706});
        let v723=(v462>v703);
        let v725=(if v723{(common.v2/v462)}else{v706});
        let v726=(v476>v703);
        let v728=(if v726{(common.v2/v476)}else{v706});
        let v738=(v688>common.v1);
        let v740=(if v738{(common.v2/v688)}else{common.v1});
        let v755=(common.v36*(common.v745-common.v751));
        let v763=(common.v36*(common.v748-common.v742));
        let v766=(common.v764-common.v751);
        let v768=(common.v36*(common.v751-common.v748));
        let v769=(common.v759-common.v745);
        let v770=(common.v745-common.v741);
        let v771=(common.v760-common.v742);
        let v772=(common.v756-common.v751);
        let v777=(common.v36*(common.v745-common.v773));
        let v779=(ctx.node_voltage(nodes[3])-common.v773);
        let v1139=(common.v1085/common.v1138);
        let v1140=(common.v1067/common.v1138);
        let v1185=(if (self.scalar_static_f64[183]!=0.0){(common.v2+(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v1180)}else{common.v1})))}else{common.v1125});
        let v1187=(if (v1185>common.v1093){common.v2}else{common.v1});
        let v1188=((self.scalar_static_f64[183]!=0.0)&&(v1187!=0.0));
        let v1189=(v1185).sqrt();
        let v1194=((self.scalar_static_f64[183]!=0.0)&&(!(v1187!=0.0)));
        let v1196=(if v1194{0.50005}else{(if v1188{(common.v139*(common.v2+v1189))}else{common.v1})});
        let v1210=(common.v1209-common.v2);
        let v1213=(common.v1180-(if (self.scalar_static_f64[183]!=0.0){(common.v498*v1210)}else{common.v1}));
        let v1218=(if self.scalar_static_bool[31]{common.v2}else{v1196});
        let v1219=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(v1213/v1196)}else{common.v1})});
        let v1243=(if (common.v744<v279){common.v2}else{common.v1});
        let v1244=((self.scalar_static_f64[187]!=0.0)&&(v1243!=0.0));
        let v1246=((common.v744*common.v1241)).exp();
        let v1248=(!(v1243!=0.0));
        let v1249=((self.scalar_static_f64[187]!=0.0)&&v1248);
        let v1251=((v279*common.v1241)).exp();
        let v1252=(common.v744-v279);
        let v1254=(common.v2+(common.v1241*v1252));
        let v1256=(if v1249{(v1251*v1254)}else{(if v1244{v1246}else{common.v1})});
        let v1263=(common.v2+(self.scalar_static_f64[188]*(common.v1098-common.v2)));
        let v1264=(v505*v1263);
        let v1265=(common.v1239-common.v2);
        let v1267=(v1256-common.v2);
        let v1268=(v512*v1267);
        let v1275=(if self.scalar_static_bool[36]{(v1268+(v505*v1265))}else{(if self.scalar_static_bool[34]{((v1264*v1265)+v1268)}else{common.v1})});
        let v1325=(if (common.v747<v279){common.v2}else{common.v1});
        let v1326=(self.scalar_static_bool[41]&&(v1325!=0.0));
        let v1328=((common.v747*common.v1323)).exp();
        let v1330=(!(v1325!=0.0));
        let v1331=(self.scalar_static_bool[41]&&v1330);
        let v1333=((v279*common.v1323)).exp();
        let v1334=(common.v747-v279);
        let v1336=(common.v2+(common.v1323*v1334));
        let v1338=(if v1331{(v1333*v1336)}else{(if v1326{v1328}else{v1256})});
        let v1339=(common.v1322-common.v2);
        let v1341=(v1338-common.v2);
        let v1344=(if self.scalar_static_bool[41]{((v505*v1339)+(v512*v1341))}else{common.v1});
        let v1382=((v1243!=0.0)&&self.scalar_static_bool[44]);
        let v1384=((common.v744*common.v1381)).exp();
        let v1386=(v1248&&self.scalar_static_bool[44]);
        let v1388=((v279*common.v1381)).exp();
        let v1390=(common.v2+(v1252*common.v1381));
        let v1392=(if v1386{(v1388*v1390)}else{(if v1382{v1384}else{v1338})});
        let v1394=(common.v1380-common.v2);
        let v1396=(v1392-common.v2);
        let v1397=(v512*v1396);
        let v1405=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v1397+(v505*v1394)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*((v1264*v1394)+v1397))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v1275-(self.scalar_static_f64[34]*(common.v1297-v692)))}else{v1275})})})});
        let v1428=(if self.scalar_static_bool[47]{(v1405-(self.scalar_static_f64[192]*(common.v1423-v692)))}else{v1405});
        let v1442=((v1325!=0.0)&&self.scalar_static_bool[44]);
        let v1444=((common.v747*common.v1441)).exp();
        let v1446=(v1330&&self.scalar_static_bool[44]);
        let v1448=((v279*common.v1441)).exp();
        let v1450=(common.v2+(v1334*common.v1441));
        let v1452=(if v1446{(v1448*v1450)}else{(if v1442{v1444}else{v1392})});
        let v1454=(common.v1440-common.v2);
        let v1456=(v1452-common.v2);
        let v1460=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*((v505*v1454)+(v512*v1456)))}else{(if self.scalar_static_bool[42]{(v1344-(self.scalar_static_f64[34]*(common.v1362-v692)))}else{v1344})});
        let v1482=(if self.scalar_static_bool[47]{(v1460-(self.scalar_static_f64[194]*(common.v1477-v692)))}else{v1460});
        let v1499=(if (common.v750<v321){common.v2}else{common.v1});
        let v1501=((common.v750*common.v1497)).exp();
        let v1503=(!(v1499!=0.0));
        let v1505=((v321*common.v1497)).exp();
        let v1506=(common.v750-v321);
        let v1508=(common.v2+(common.v1497*v1506));
        let v1510=(if v1503{(v1505*v1508)}else{(if (v1499!=0.0){v1501}else{v1452})});
        let v1511=(common.v1496-common.v2);
        let v1513=(v1510-common.v2);
        let v1515=((v519*v1511)+(v526*v1513));
        let v1538=(if (common.v758<v345){common.v2}else{common.v1});
        let v1539=((self.scalar_static_f64[195]!=0.0)&&(v1538!=0.0));
        let v1541=((common.v758*common.v1536)).exp();
        let v1544=((self.scalar_static_f64[195]!=0.0)&&(!(v1538!=0.0)));
        let v1546=((v345*common.v1536)).exp();
        let v1547=(common.v758-v345);
        let v1549=(common.v2+(common.v1536*v1547));
        let v1551=(if v1544{(v1546*v1549)}else{(if v1539{v1541}else{v1510})});
        let v1552=(common.v1535-common.v2);
        let v1554=(v1551-common.v2);
        let v1559=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*v1552)+(v530*v1554))}else{common.v1})});
        let v1587=(v707*v766);
        let v1588=(common.v2+common.v1583);
        let v1589=(common.v2+common.v1586);
        let v1590=(v1588/v1589);
        let v1593=((common.v1583-common.v1586)-(v1590).ln());
        let v1595=(v768+(common.v409*v1593));
        let v1596=(v710*v1595);
        let v1597=(v740*v1596);
        let v1599=(v54*(common.v139*v740));
        let v1602=((v3+(v768*v768))).sqrt();
        let v1604=(common.v2+(v1599*v1602));
        let v1605=(v710*v1604);
        let v1606=(v1597/v1605);
        let v1609=((common.v2+(v1606*v1606))).sqrt();
        let v1610=(v1596/v1609);
        let v1611=(v713*v769);
        let v1612=(v770*common.v1138);
        let v1613=(v716*v1612);
        let v1614=(v719*v771);
        let v1615=(v772*v1218);
        let v1616=(v722*v1615);
        let v1617=(v725*v779);
        let v1621=0.02;
        let v1623=(v1621*(common.v2+v554));
        let v1628=(if (self.scalar_static_f64[197]!=0.0){f64::powf(v1623,self.scalar_static_f64[199])}else{common.v1});
        let v1630=((common.v634-common.v750)-v1628);
        let v1633=((v3+(v1630*v1630))).sqrt();
        let v1637=(if (self.scalar_static_f64[197]!=0.0){(v1628+(common.v139*(v1630+v1633)))}else{common.v1});
        let v1638=(-v554);
        let v1640=f64::powf(v1637,self.scalar_static_f64[200]);
        let v1642=(if (self.scalar_static_f64[197]!=0.0){(v1638*v1640)}else{common.v1});
        let v1644=(if (v1642<common.v39){common.v2}else{common.v1});
        let v1645=((self.scalar_static_f64[197]!=0.0)&&(v1644!=0.0));
        let v1646=(v1642).exp();
        let v1649=((self.scalar_static_f64[197]!=0.0)&&(!(v1644!=0.0)));
        let v1650=(if v1649{common.v1566}else{common.v1});
        let v1654=(if v1649{(v1650*(common.v2+(v1642-common.v39)))}else{(if v1645{v1646}else{common.v1})});
        let v1655=(self.scalar_static_f64[196]*v1637);
        let v1657=(if (self.scalar_static_f64[197]!=0.0){(v1654*v1655)}else{common.v1});
        let v1658=(common.v781-v1139);
        let v1659=(v1658-v1515);
        let v1668=(v1621*(common.v2+v559));
        let v1673=(if (self.scalar_static_f64[202]!=0.0){f64::powf(v1668,self.scalar_static_f64[205])}else{common.v1});
        let v1675=((common.v1-v755)-v1673);
        let v1678=((v3+(v1675*v1675))).sqrt();
        let v1682=(if (self.scalar_static_f64[202]!=0.0){(v1673+(common.v139*(v1675+v1678)))}else{common.v1});
        let v1683=(-v559);
        let v1685=f64::powf(v1682,self.scalar_static_f64[206]);
        let v1687=(if (self.scalar_static_f64[202]!=0.0){(v1683*v1685)}else{common.v1});
        let v1689=(if (v1687<common.v39){common.v2}else{common.v1});
        let v1690=((self.scalar_static_f64[202]!=0.0)&&(v1689!=0.0));
        let v1691=(v1687).exp();
        let v1694=((self.scalar_static_f64[202]!=0.0)&&(!(v1689!=0.0)));
        let v1695=(if v1694{common.v1566}else{common.v1});
        let v1699=(if v1694{(v1695*(common.v2+(v1687-common.v39)))}else{(if v1690{v1691}else{common.v1})});
        let v1700=(self.scalar_static_f64[201]*v1682);
        let v1702=(if (self.scalar_static_f64[202]!=0.0){(v1699*v1700)}else{v1657});
        let v1703=(-v1587);
        let v1720=0.1;
        let v1722=(if self.scalar_static_bool[60]{((common.v2-(common.v750/self.scalar_static_f64[210]))-v1720)}else{common.v1});
        let v1725=((common.v1090+(v1722*v1722))).sqrt();
        let v1734=(if self.scalar_static_bool[62]{self.scalar_static_f64[208]}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[208]*(if self.scalar_static_bool[60]{(v1720+(common.v139*(v1722+v1725)))}else{v1722}))}else{common.v1})});
        let v1736=((v1140/v1734)-common.v2);
        let v1744=((v1515-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){(v1657*v1659)}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if (self.scalar_static_f64[209]!=0.0){(self.scalar_static_f64[207]*f64::powf(v1736,self.scalar_static_f64[212]))}else{common.v1})}));
        let v1767=(if (self.scalar_static_f64[213]!=0.0){(common.v2/v541)}else{common.v1750});
        let v1769=(if (common.v775<v387){common.v2}else{common.v1});
        let v1770=((self.scalar_static_f64[213]!=0.0)&&(v1769!=0.0));
        let v1772=((common.v775*v1767)).exp();
        let v1775=((self.scalar_static_f64[213]!=0.0)&&(!(v1769!=0.0)));
        let v1777=((v387*v1767)).exp();
        let v1778=(common.v775-v387);
        let v1780=(common.v2+(v1767*v1778));
        let v1783=(common.v1765-common.v2);
        let v1785=((if v1775{(v1777*v1780)}else{(if v1770{v1772}else{v1551})})-common.v2);
        let v1790=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*v1783)+(v544*v1785))}else{common.v1})});
        let v1843=(common.v36*v1610);
        let v1845=(common.v36*v1219);
        let v2189=(common.v2174*(self.scalar_static_f64[101]*f64::powf(common.v410,self.scalar_static_f64[237])));
        let v2209=(common.v2174*(self.scalar_static_f64[108]*f64::powf(common.v410,self.scalar_static_f64[240])));
        let v2304=((v504*(self.scalar_static_f64[63]*(common.v2174*(self.scalar_static_f64[66]*f64::powf(common.v410,self.scalar_static_f64[248])))))+(v500*(v504*(((common.v502*(self.scalar_static_f64[68]*common.v2245))-(v501*common.v2295))/common.v2299))));
        let v2320=((v511*(self.scalar_static_f64[69]*(common.v2174*(self.scalar_static_f64[72]*f64::powf(common.v410,self.scalar_static_f64[249])))))+(v507*(v511*(((common.v509*(self.scalar_static_f64[74]*common.v2245))-(v508*common.v2311))/common.v2315))));
        let v2324=(common.v2174*(self.scalar_static_f64[77]*f64::powf(common.v410,self.scalar_static_f64[250])));
        let v2333=(v518*(((common.v516*(self.scalar_static_f64[79]*common.v2245))-(v515*common.v2327))/common.v2331));
        let v2340=(common.v2174*(self.scalar_static_f64[82]*f64::powf(common.v410,self.scalar_static_f64[251])));
        let v2349=(v525*(((common.v523*(self.scalar_static_f64[84]*common.v2245))-(v522*common.v2343))/common.v2347));
        let v2383=(self.scalar_static_f64[93]*common.v2173);
        let v2387=(v541*v541);
        let v2397=(self.scalar_static_f64[122]*(self.scalar_static_f64[123]*common.v2171));
        let v2399=(self.scalar_static_f64[124]*(self.scalar_static_f64[125]*common.v2171));
        let v2588=(v692*(((common.v690*common.v2579)-(common.v689*common.v2582))/common.v2586));
        let v2600=(if v708{((-(if self.scalar_static_bool[14]{(self.scalar_static_f64[103]*v2189)}else{(if (self.scalar_static_f64[102]!=0.0){(self.scalar_static_f64[103]*(common.v2174*(self.scalar_static_f64[104]*f64::powf(common.v410,self.scalar_static_f64[238]))))}else{common.v1})}))/(v434*v434))}else{common.v1});
        let v2640=(if v738{((-(self.scalar_static_f64[146]*(common.v2174*(self.scalar_static_f64[147]*f64::powf(common.v410,self.scalar_static_f64[257])))))/(v688*v688))}else{common.v1});
        let v3401=(((common.v1138*common.v3268)-(common.v1085*common.v3393))/common.v3400);
        let v3405=(((common.v1138*common.v3269)-(common.v1085*common.v3394))/common.v3400);
        let v3409=(((common.v1138*common.v3270)-(common.v1085*common.v3395))/common.v3400);
        let v3413=(((common.v1138*common.v3271)-(common.v1085*common.v3396))/common.v3400);
        let v3417=(((common.v1138*common.v3232)-(common.v1067*common.v3393))/common.v3400);
        let v3420=((-(common.v1067*common.v3394))/common.v3400);
        let v3424=(((common.v1138*common.v3233)-(common.v1067*common.v3395))/common.v3400);
        let v3428=(((common.v1138*common.v3234)-(common.v1067*common.v3396))/common.v3400);
        let v3522=(common.v571*v1189);
        let v3541=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3492)}else{common.v1}))}else{common.v3363})/v3522))}else{common.v1})});
        let v3542=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3493)}else{common.v1}))}else{common.v3364})/v3522))}else{common.v1})});
        let v3543=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3494)}else{common.v1}))}else{common.v1})/v3522))}else{common.v1})});
        let v3544=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3495)}else{common.v1}))}else{common.v3365})/v3522))}else{common.v1})});
        let v3545=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3496)}else{common.v1}))}else{common.v3366})/v3522))}else{common.v1})});
        let v3546=(if v1194{common.v1}else{(if v1188{(common.v139*((if (self.scalar_static_f64[183]!=0.0){(common.v141*(if (self.scalar_static_f64[183]!=0.0){(common.v49*common.v3497)}else{common.v1}))}else{common.v1})/v3522))}else{common.v1})});
        let v3595=(v1196*v1196);
        let v3637=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3492-(if (self.scalar_static_f64[183]!=0.0){((v1210*common.v2288)+(common.v498*common.v3562))}else{common.v1})))-(v1213*v3541))/v3595)}else{common.v1})});
        let v3638=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3493-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3563)}else{common.v1})))-(v1213*v3542))/v3595)}else{common.v1})});
        let v3639=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3494-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3564)}else{common.v1})))-(v1213*v3543))/v3595)}else{common.v1})});
        let v3640=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3495-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3565)}else{common.v1})))-(v1213*v3544))/v3595)}else{common.v1})});
        let v3641=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3496-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3566)}else{common.v1})))-(v1213*v3545))/v3595)}else{common.v1})});
        let v3642=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){(((v1196*(common.v3497-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3567)}else{common.v1})))-(v1213*v3546))/v3595)}else{common.v1})});
        let v3643=(if self.scalar_static_bool[31]{common.v1}else{(if (self.scalar_static_f64[183]!=0.0){((-(if (self.scalar_static_f64[183]!=0.0){(common.v498*common.v3568)}else{common.v1}))/v1196)}else{common.v1})});
        let v3679=(common.v36*common.v1241);
        let v3680=(common.v1241*common.v2641);
        let v3695=(if v1249{((v1254*(v1251*(v279*common.v3677)))+(v1251*(v1252*common.v3677)))}else{(if v1244{(v1246*(common.v744*common.v3677))}else{common.v1})});
        let v3696=(if v1249{(v1251*v3679)}else{(if v1244{(v1246*v3679)}else{common.v1})});
        let v3697=(if v1249{(v1251*v3680)}else{(if v1244{(v1246*v3680)}else{common.v1})});
        let v3704=((v1263*v2304)+(v505*(self.scalar_static_f64[188]*common.v3301)));
        let v3705=(v505*(self.scalar_static_f64[188]*common.v3302));
        let v3706=(v505*(self.scalar_static_f64[188]*common.v3303));
        let v3707=(v505*(self.scalar_static_f64[188]*common.v3304));
        let v3725=((v1267*v2320)+(v512*v3695));
        let v3726=(v512*v3696);
        let v3727=(v512*v3697);
        let v3750=(if self.scalar_static_bool[36]{(v3725+((v1265*v2304)+(v505*common.v3668)))}else{(if self.scalar_static_bool[34]{(((v1265*v3704)+(v1264*common.v3668))+v3725)}else{common.v1})});
        let v3751=(if self.scalar_static_bool[36]{(v505*common.v3669)}else{(if self.scalar_static_bool[34]{((v1265*v3705)+(v1264*common.v3669))}else{common.v1})});
        let v3753=(if self.scalar_static_bool[36]{(v3726+(v505*common.v3671))}else{(if self.scalar_static_bool[34]{(((v1265*v3706)+(v1264*common.v3671))+v3726)}else{common.v1})});
        let v3754=(if self.scalar_static_bool[36]{(v3727+(v505*common.v3672))}else{(if self.scalar_static_bool[34]{(((v1265*v3707)+(v1264*common.v3672))+v3727)}else{common.v1})});
        let v3839=(common.v36*common.v1323);
        let v3840=(common.v1323*common.v2641);
        let v3856=(if v1331{((v1336*(v1333*(v279*common.v3837)))+(v1333*(v1334*common.v3837)))}else{(if v1326{(v1328*(common.v747*common.v3837))}else{v3695})});
        let v3857=(if v1331{(v1333*v3839)}else{(if v1326{(v1328*v3839)}else{common.v1})});
        let v3858=(if v1331{common.v1}else{(if v1326{common.v1}else{v3696})});
        let v3859=(if v1331{(v1333*v3840)}else{(if v1326{(v1328*v3840)}else{v3697})});
        let v3879=(if self.scalar_static_bool[41]{(((v1339*v2304)+(v505*common.v3830))+((v1341*v2320)+(v512*v3856)))}else{common.v1});
        let v3880=(if self.scalar_static_bool[41]{(v505*common.v3831)}else{common.v1});
        let v3882=(if self.scalar_static_bool[41]{((v505*common.v3833)+(v512*v3858))}else{common.v1});
        let v3883=(if self.scalar_static_bool[41]{((v505*common.v3834)+(v512*v3859))}else{common.v1});
        let v3959=(common.v36*common.v1381);
        let v3960=(common.v1381*common.v2641);
        let v3976=(if v1386{((v1390*(v1388*(v279*common.v3957)))+(v1388*(v1252*common.v3957)))}else{(if v1382{(v1384*(common.v744*common.v3957))}else{v3856})});
        let v3977=(if v1386{common.v1}else{(if v1382{common.v1}else{v3857})});
        let v3978=(if v1386{(v1388*v3959)}else{(if v1382{(v1384*v3959)}else{v3858})});
        let v3979=(if v1386{(v1388*v3960)}else{(if v1382{(v1384*v3960)}else{v3859})});
        let v3997=((v1396*v2320)+(v512*v3976));
        let v3998=(v512*v3977);
        let v3999=(v512*v3978);
        let v4000=(v512*v3979);
        let v4039=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v3997+((v1394*v2304)+(v505*common.v3950))))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*(((v1394*v3704)+(v1264*common.v3950))+v3997))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3750-(self.scalar_static_f64[34]*(common.v3784-v2588)))}else{v3750})})})});
        let v4040=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v505*common.v3951))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*((v1394*v3705)+(v1264*common.v3951)))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3751-(self.scalar_static_f64[34]*common.v3785))}else{v3751})})})});
        let v4041=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v3998+(v505*common.v3952)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*((v1264*common.v3952)+v3998))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v505*common.v3670)}else{(if self.scalar_static_bool[34]{(v1264*common.v3670)}else{common.v1})})})})});
        let v4042=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v3999+(v505*common.v3953)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*(((v1394*v3706)+(v1264*common.v3953))+v3999))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3753-(self.scalar_static_f64[34]*common.v3786))}else{v3753})})})});
        let v4043=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v4000+(v505*common.v3954)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*(((v1394*v3707)+(v1264*common.v3954))+v4000))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3754-(self.scalar_static_f64[34]*common.v3787))}else{v3754})})})});
        let v4044=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v505*common.v3955))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*(v1264*common.v3955))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v505*common.v3673)}else{(if self.scalar_static_bool[34]{(v1264*common.v3673)}else{common.v1})})})})});
        let v4045=(if self.scalar_static_bool[46]{(self.scalar_static_f64[186]*(v505*common.v3956))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[186]*(v1264*common.v3956))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v505*common.v3674)}else{(if self.scalar_static_bool[34]{(v1264*common.v3674)}else{common.v1})})})})});
        let v4084=(if self.scalar_static_bool[47]{(v4039-(self.scalar_static_f64[192]*(common.v4071-v2588)))}else{v4039});
        let v4085=(if self.scalar_static_bool[47]{(v4040-(self.scalar_static_f64[192]*common.v4072))}else{v4040});
        let v4086=(if self.scalar_static_bool[47]{(v4042-(self.scalar_static_f64[192]*common.v4073))}else{v4042});
        let v4087=(if self.scalar_static_bool[47]{(v4043-(self.scalar_static_f64[192]*common.v4074))}else{v4043});
        let v4119=(common.v36*common.v1441);
        let v4120=(common.v1441*common.v2641);
        let v4136=(if v1446{((v1450*(v1448*(v279*common.v4117)))+(v1448*(v1334*common.v4117)))}else{(if v1442{(v1444*(common.v747*common.v4117))}else{v3976})});
        let v4137=(if v1446{(v1448*v4119)}else{(if v1442{(v1444*v4119)}else{v3977})});
        let v4138=(if v1446{common.v1}else{(if v1442{common.v1}else{v3978})});
        let v4139=(if v1446{(v1448*v4120)}else{(if v1442{(v1444*v4120)}else{v3979})});
        let v4166=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*(((v1454*v2304)+(v505*common.v4110))+((v1456*v2320)+(v512*v4136))))}else{(if self.scalar_static_bool[42]{(v3879-(self.scalar_static_f64[34]*(common.v3911-v2588)))}else{v3879})});
        let v4167=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*(v505*common.v4111))}else{(if self.scalar_static_bool[42]{(v3880-(self.scalar_static_f64[34]*common.v3912))}else{v3880})});
        let v4168=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*((v505*common.v4112)+(v512*v4137)))}else{(if self.scalar_static_bool[41]{((v505*common.v3832)+(v512*v3857))}else{common.v1})});
        let v4169=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*((v505*common.v4113)+(v512*v4138)))}else{(if self.scalar_static_bool[42]{(v3882-(self.scalar_static_f64[34]*common.v3913))}else{v3882})});
        let v4170=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*((v505*common.v4114)+(v512*v4139)))}else{(if self.scalar_static_bool[42]{(v3883-(self.scalar_static_f64[34]*common.v3914))}else{v3883})});
        let v4171=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*(v505*common.v4115))}else{(if self.scalar_static_bool[41]{(v505*common.v3835)}else{common.v1})});
        let v4172=(if self.scalar_static_bool[44]{(self.scalar_static_f64[193]*(v505*common.v4116))}else{(if self.scalar_static_bool[41]{(v505*common.v3836)}else{common.v1})});
        let v4211=(if self.scalar_static_bool[47]{(v4166-(self.scalar_static_f64[194]*(common.v4198-v2588)))}else{v4166});
        let v4212=(if self.scalar_static_bool[47]{(v4167-(self.scalar_static_f64[194]*common.v4199))}else{v4167});
        let v4213=(if self.scalar_static_bool[47]{(v4169-(self.scalar_static_f64[194]*common.v4200))}else{v4169});
        let v4214=(if self.scalar_static_bool[47]{(v4170-(self.scalar_static_f64[194]*common.v4201))}else{v4170});
        let v4248=(common.v1497*common.v2641);
        let v4249=(common.v36*common.v1497);
        let v4266=(if v1503{((v1508*(v1505*(v321*common.v4246)))+(v1505*(v1506*common.v4246)))}else{(if (v1499!=0.0){(v1501*(common.v750*common.v4246))}else{v4136})});
        let v4267=(if v1503{(v1505*v4248)}else{(if (v1499!=0.0){(v1501*v4248)}else{common.v1})});
        let v4268=(if v1503{common.v1}else{(if (v1499!=0.0){common.v1}else{v4137})});
        let v4269=(if v1503{(v1505*v4249)}else{(if (v1499!=0.0){(v1501*v4249)}else{v4138})});
        let v4270=(if v1503{common.v1}else{(if (v1499!=0.0){common.v1}else{v4139})});
        let v4278=(v519*common.v4243);
        let v4279=(v519*common.v4244);
        let v4287=(((v1511*((v518*(self.scalar_static_f64[75]*v2324))+(v514*v2333)))+(v519*common.v4238))+((v1513*((v525*(self.scalar_static_f64[80]*v2340))+(v521*v2349)))+(v526*v4266)));
        let v4288=((v519*common.v4239)+(v526*v4267));
        let v4289=((v519*common.v4240)+(v526*v4268));
        let v4290=((v519*common.v4241)+(v526*v4269));
        let v4291=((v519*common.v4242)+(v526*v4270));
        let v4323=(common.v36*common.v1536);
        let v4324=(common.v1536*common.v2641);
        let v4342=(if v1544{((v1549*(v1546*(v345*common.v4321)))+(v1546*(v1547*common.v4321)))}else{(if v1539{(v1541*(common.v758*common.v4321))}else{v4266})});
        let v4343=(if v1544{common.v1}else{(if v1539{common.v1}else{v4267})});
        let v4344=(if v1544{(v1546*v4323)}else{(if v1539{(v1541*v4323)}else{v4268})});
        let v4345=(if v1544{common.v1}else{(if v1539{common.v1}else{v4269})});
        let v4346=(if v1544{common.v1}else{(if v1539{common.v1}else{v4270})});
        let v4347=(if v1544{(v1546*v4324)}else{(if v1539{(v1541*v4324)}else{common.v1})});
        let v4378=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){(((v1552*((v527*v2333)+(v518*(self.scalar_static_f64[85]*v2324))))+(v528*common.v4314))+((v1554*((v529*v2349)+(v525*(self.scalar_static_f64[86]*v2340))))+(v530*v4342)))}else{common.v1})});
        let v4379=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*common.v4315)+(v530*v4343))}else{common.v1})});
        let v4380=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*common.v4316)+(v530*v4344))}else{common.v1})});
        let v4381=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*common.v4317)+(v530*v4345))}else{common.v1})});
        let v4382=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*common.v4318)+(v530*v4346))}else{common.v1})});
        let v4383=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){((v528*common.v4319)+(v530*v4347))}else{common.v1})});
        let v4384=(if self.scalar_static_bool[51]{common.v1}else{(if (self.scalar_static_f64[195]!=0.0){(v528*common.v4320)}else{common.v1})});
        let v4457=(v766*(if v704{((-(if self.scalar_static_bool[13]{(self.scalar_static_f64[99]*v2189)}else{(if (self.scalar_static_f64[98]!=0.0){(self.scalar_static_f64[99]*(common.v2174*(self.scalar_static_f64[100]*f64::powf(common.v410,self.scalar_static_f64[236]))))}else{common.v1})}))/(v425*v425))}else{common.v1}));
        let v4458=(-v707);
        let v4462=(v1589*v1589);
        let v4517=((v1595*v2600)+(v710*((v1593*common.v2173)+(common.v409*((common.v4437-common.v4452)-((((v1589*common.v4437)-(v1588*common.v4452))/v4462)/v1590))))));
        let v4518=(v710*(common.v36+(common.v409*((-common.v4453)-(((-(v1588*common.v4453))/v4462)/v1590)))));
        let v4519=(v710*(common.v2641+(common.v409*((common.v4438-common.v4454)-((((v1589*common.v4438)-(v1588*common.v4454))/v4462)/v1590)))));
        let v4520=(v710*(common.v409*(common.v4439-((common.v4439/v1589)/v1590))));
        let v4521=(v710*(common.v409*((common.v4440-common.v4455)-((((v1589*common.v4440)-(v1588*common.v4455))/v4462)/v1590))));
        let v4522=(v710*(common.v409*((common.v4441-common.v4456)-((((v1589*common.v4441)-(v1588*common.v4456))/v4462)/v1590))));
        let v4523=(v710*(common.v409*(common.v4442-((common.v4442/v1589)/v1590))));
        let v4524=(v710*(common.v409*(common.v4443-((common.v4443/v1589)/v1590))));
        let v4537=(common.v36*v768);
        let v4539=(v768*common.v2641);
        let v4541=(common.v571*v1602);
        let v4555=(v1605*v1605);
        let v4570=(v1606*(((v1605*((v1596*v2640)+(v740*v4517)))-(v1597*((v1604*v2600)+(v710*(v1602*(v54*(common.v139*v2640)))))))/v4555));
        let v4572=(v1606*(((v1605*(v740*v4518))-(v1597*(v710*(v1599*((v4537+v4537)/v4541)))))/v4555));
        let v4574=(v1606*(((v1605*(v740*v4519))-(v1597*(v710*(v1599*((v4539+v4539)/v4541)))))/v4555));
        let v4576=(v1606*((v740*v4520)/v1605));
        let v4578=(v1606*((v740*v4521)/v1605));
        let v4580=(v1606*((v740*v4522)/v1605));
        let v4582=(v1606*((v740*v4523)/v1605));
        let v4584=(v1606*((v740*v4524)/v1605));
        let v4586=(common.v571*v1609);
        let v4598=(v1609*v1609);
        let v4599=(((v1609*v4517)-(v1596*((v4570+v4570)/v4586)))/v4598);
        let v4603=(((v1609*v4518)-(v1596*((v4572+v4572)/v4586)))/v4598);
        let v4607=(((v1609*v4519)-(v1596*((v4574+v4574)/v4586)))/v4598);
        let v4611=(((v1609*v4520)-(v1596*((v4576+v4576)/v4586)))/v4598);
        let v4615=(((v1609*v4521)-(v1596*((v4578+v4578)/v4586)))/v4598);
        let v4619=(((v1609*v4522)-(v1596*((v4580+v4580)/v4586)))/v4598);
        let v4623=(((v1609*v4523)-(v1596*((v4582+v4582)/v4586)))/v4598);
        let v4627=(((v1609*v4524)-(v1596*((v4584+v4584)/v4586)))/v4598);
        let v4628=(v769*(if v711{((-(if self.scalar_static_bool[15]{(self.scalar_static_f64[106]*v2209)}else{(if (self.scalar_static_f64[105]!=0.0){(self.scalar_static_f64[106]*(common.v2174*(self.scalar_static_f64[107]*f64::powf(common.v410,self.scalar_static_f64[239]))))}else{common.v1})}))/(v445*v445))}else{common.v1}));
        let v4629=(-v713);
        let v4638=((v1612*(if v714{((-(if self.scalar_static_bool[16]{(self.scalar_static_f64[110]*v2209)}else{(if (self.scalar_static_f64[109]!=0.0){(self.scalar_static_f64[110]*(common.v2174*(self.scalar_static_f64[111]*f64::powf(common.v410,self.scalar_static_f64[241]))))}else{common.v1})}))/(v454*v454))}else{common.v1}))+(v716*(v770*common.v3393)));
        let v4639=(v716*(v770*common.v3394));
        let v4640=(v716*common.v1138);
        let v4641=(v716*((-common.v1138)+(v770*common.v3395)));
        let v4642=(v716*(v770*common.v3396));
        let v4643=(v771*(if v717{((-(self.scalar_static_f64[112]*(common.v2174*(self.scalar_static_f64[113]*f64::powf(common.v410,self.scalar_static_f64[242])))))/(v458*v458))}else{common.v1}));
        let v4644=(-v719);
        let v4655=((v1615*(if v720{((-(if self.scalar_static_bool[17]{(self.scalar_static_f64[117]*v2189)}else{(if (self.scalar_static_f64[116]!=0.0){(self.scalar_static_f64[117]*(common.v2174*(self.scalar_static_f64[118]*f64::powf(common.v410,self.scalar_static_f64[244]))))}else{common.v1})}))/(v471*v471))}else{common.v1}))+(v722*(v772*(if self.scalar_static_bool[31]{common.v1}else{v3541}))));
        let v4656=(v722*(-v1218));
        let v4657=(v722*(v772*(if self.scalar_static_bool[31]{common.v1}else{v3542})));
        let v4658=(v722*(v772*(if self.scalar_static_bool[31]{common.v1}else{v3543})));
        let v4659=(v722*(v772*(if self.scalar_static_bool[31]{common.v1}else{v3544})));
        let v4660=(v722*(v772*(if self.scalar_static_bool[31]{common.v1}else{v3545})));
        let v4661=(v722*(v1218+(v772*(if self.scalar_static_bool[31]{common.v1}else{v3546}))));
        let v4662=(v779*(if v723{((-(self.scalar_static_f64[114]*(common.v2174*(self.scalar_static_f64[115]*f64::powf(common.v410,self.scalar_static_f64[243])))))/(v462*v462))}else{common.v1}));
        let v4663=(-v725);
        let v4669=(if (self.scalar_static_f64[197]!=0.0){((v1621*v2397)*(self.scalar_static_f64[199]*f64::powf(v1623,self.scalar_static_f64[263])))}else{common.v1});
        let v4670=(common.v2495-v4669);
        let v4671=(v1630*v4670);
        let v4673=(common.v36*v1630);
        let v4675=(v1630*common.v2641);
        let v4677=(common.v571*v1633);
        let v4688=(if (self.scalar_static_f64[197]!=0.0){(v4669+(common.v139*(v4670+((v4671+v4671)/v4677))))}else{common.v1});
        let v4689=(if (self.scalar_static_f64[197]!=0.0){(common.v139*(common.v36+((v4673+v4673)/v4677)))}else{common.v1});
        let v4690=(if (self.scalar_static_f64[197]!=0.0){(common.v139*(common.v2641+((v4675+v4675)/v4677)))}else{common.v1});
        let v4694=(self.scalar_static_f64[200]*f64::powf(v1637,self.scalar_static_f64[264]));
        let v4703=(if (self.scalar_static_f64[197]!=0.0){((v1640*(-v2397))+(v1638*(v4688*v4694)))}else{common.v1});
        let v4704=(if (self.scalar_static_f64[197]!=0.0){(v1638*(v4689*v4694))}else{common.v1});
        let v4705=(if (self.scalar_static_f64[197]!=0.0){(v1638*(v4690*v4694))}else{common.v1});
        let v4730=(if (self.scalar_static_f64[197]!=0.0){((v1655*(if v1649{(v1650*v4703)}else{(if v1645{(v1646*v4703)}else{common.v1})}))+(v1654*(self.scalar_static_f64[196]*v4688)))}else{common.v1});
        let v4731=(if (self.scalar_static_f64[197]!=0.0){((v1655*(if v1649{(v1650*v4704)}else{(if v1645{(v1646*v4704)}else{common.v1})}))+(v1654*(self.scalar_static_f64[196]*v4689)))}else{common.v1});
        let v4732=(if (self.scalar_static_f64[197]!=0.0){((v1655*(if v1649{(v1650*v4705)}else{(if v1645{(v1646*v4705)}else{common.v1})}))+(v1654*(self.scalar_static_f64[196]*v4690)))}else{common.v1});
        let v4733=(-v3401);
        let v4734=(-v3405);
        let v4735=(-v3409);
        let v4736=(-v3413);
        let v4778=(if (self.scalar_static_f64[202]!=0.0){((v1621*v2399)*(self.scalar_static_f64[205]*f64::powf(v1668,self.scalar_static_f64[265])))}else{common.v1});
        let v4779=(-v4778);
        let v4780=(v1675*v4779);
        let v4782=(common.v36*v1675);
        let v4784=(v1675*common.v2641);
        let v4786=(common.v571*v1678);
        let v4797=(if (self.scalar_static_f64[202]!=0.0){(v4778+(common.v139*(v4779+((v4780+v4780)/v4786))))}else{common.v1});
        let v4798=(if (self.scalar_static_f64[202]!=0.0){(common.v139*(common.v36+((v4782+v4782)/v4786)))}else{common.v1});
        let v4799=(if (self.scalar_static_f64[202]!=0.0){(common.v139*(common.v2641+((v4784+v4784)/v4786)))}else{common.v1});
        let v4803=(self.scalar_static_f64[206]*f64::powf(v1682,self.scalar_static_f64[266]));
        let v4812=(if (self.scalar_static_f64[202]!=0.0){((v1685*(-v2399))+(v1683*(v4797*v4803)))}else{common.v1});
        let v4813=(if (self.scalar_static_f64[202]!=0.0){(v1683*(v4798*v4803))}else{common.v1});
        let v4814=(if (self.scalar_static_f64[202]!=0.0){(v1683*(v4799*v4803))}else{common.v1});
        let v4871=(if self.scalar_static_bool[60]{(-(common.v2641/self.scalar_static_f64[210]))}else{common.v1});
        let v4872=(if self.scalar_static_bool[60]{(-(common.v36/self.scalar_static_f64[210]))}else{common.v1});
        let v4873=(v1722*v4871);
        let v4875=(v1722*v4872);
        let v4877=(common.v571*v1725);
        let v4896=(v1734*v1734);
        let v4905=(self.scalar_static_f64[212]*f64::powf(v1736,self.scalar_static_f64[267]));
        let v4924=(v4289-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){(v1657*(-v4289))}else{common.v1})}));
        let v4927=(v4278-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){(v1657*(-v4278))}else{common.v1})}));
        let v4928=(v4279-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){(v1657*(-v4279))}else{common.v1})}));
        let v4929=(-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){v1657}else{common.v1})}));
        let v4930=((v4287-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){((v1659*v4730)+(v1657*(v4733-v4287)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if (self.scalar_static_f64[209]!=0.0){(self.scalar_static_f64[207]*((v3417/v1734)*v4905))}else{common.v1})}));
        let v4931=((v4288-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){((v1659*v4731)+(v1657*(v4734-v4288)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if (self.scalar_static_f64[209]!=0.0){(self.scalar_static_f64[207]*((((v1734*v3420)-(v1140*(if self.scalar_static_bool[62]{common.v1}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[208]*(if self.scalar_static_bool[60]{(common.v139*(v4871+((v4873+v4873)/v4877)))}else{v4871}))}else{common.v1})})))/v4896)*v4905))}else{common.v1})}));
        let v4932=((v4290-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){((v1659*v4732)+(v1657*(v4735-v4290)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if (self.scalar_static_f64[209]!=0.0){(self.scalar_static_f64[207]*((((v1734*v3424)-(v1140*(if self.scalar_static_bool[62]{common.v1}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[208]*(if self.scalar_static_bool[60]{(common.v139*(v4872+((v4875+v4875)/v4877)))}else{v4872}))}else{common.v1})})))/v4896)*v4905))}else{common.v1})}));
        let v4933=((v4291-(if self.scalar_static_bool[53]{common.v1}else{(if (self.scalar_static_f64[197]!=0.0){(v1657*(v4736-v4291))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if (self.scalar_static_f64[209]!=0.0){(self.scalar_static_f64[207]*((v3428/v1734)*v4905))}else{common.v1})}));
        let v4967=(if (self.scalar_static_f64[213]!=0.0){((-v2383)/v2387)}else{common.v4936});
        let v4969=(v1767*common.v2641);
        let v4970=(common.v36*v1767);
        let v5028=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){(((v1783*((v536*(self.scalar_static_f64[87]*(common.v2174*(self.scalar_static_f64[89]*f64::powf(common.v410,self.scalar_static_f64[252])))))+(v532*(v536*(((common.v534*(self.scalar_static_f64[91]*common.v2245))-(v533*common.v2367))/common.v2371)))))+(v537*common.v4958))+((v1785*((v543*(self.scalar_static_f64[92]*(common.v2174*(self.scalar_static_f64[94]*f64::powf(common.v410,self.scalar_static_f64[253])))))+(v539*(v543*(((v541*(self.scalar_static_f64[96]*common.v2245))-(v540*v2383))/v2387)))))+(v544*(if v1775{((v1780*(v1777*(v387*v4967)))+(v1777*(v1778*v4967)))}else{(if v1770{(v1772*(common.v775*v4967))}else{v4342})}))))}else{common.v1})});
        let v5029=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4959)+(v544*(if v1775{common.v1}else{(if v1770{common.v1}else{v4343})})))}else{common.v1})});
        let v5030=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4960)+(v544*(if v1775{common.v1}else{(if v1770{common.v1}else{v4344})})))}else{common.v1})});
        let v5031=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4961)+(v544*(if v1775{common.v1}else{(if v1770{common.v1}else{v4345})})))}else{common.v1})});
        let v5032=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4962)+(v544*(if v1775{common.v1}else{(if v1770{common.v1}else{v4346})})))}else{common.v1})});
        let v5033=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4963)+(v544*(if v1775{(v1777*v4969)}else{(if v1770{(v1772*v4969)}else{v4347})})))}else{common.v1})});
        let v5034=(if self.scalar_static_bool[67]{common.v1}else{(if (self.scalar_static_f64[213]!=0.0){((v537*common.v4964)+(v544*(if v1775{(v1777*v4970)}else{(if v1770{(v1772*v4970)}else{common.v1})})))}else{common.v1})});
        let v5243=(v11*common.v36);
        let v5244=(v11*common.v2641);

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * ((common.v36*(v1428+(v11*common.v744)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4084), (common.v36*v4085), (common.v36*v4041), (common.v36*(v4086+v5243)), (common.v36*(v4087+v5244)), (common.v36*v4044), (common.v36*v4045)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * ((common.v36*(v1482+(v11*common.v747)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4211), (common.v36*v4212), (common.v36*(v4168+v5243)), (common.v36*v4213), (common.v36*(v4214+v5244)), (common.v36*v4171), (common.v36*v4172)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(9),
            multiplicity * ((common.v36*common.v781)),
            13,
            multiplicity * (common.v36),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * ((common.v36*v1139)),
            [4, 6, 8, 9],
            [(common.v36*v3401), (common.v36*v3405), (common.v36*v3409), (common.v36*v3413)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * ((common.v36*(v1744+(v11*common.v750)))),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [(common.v36*v4930), (common.v36*(v4931+v5244)), (common.v36*v4924), (common.v36*(v4932+v5243)), (common.v36*v4933), (common.v36*v4927), (common.v36*v4928), (common.v36*v4929)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * ((common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){(v1702*v1703)}else{common.v1})})+(v11*v755)))),
            [0, 4, 5, 6, 7, 8],
            [(common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){(v1702*v4458)}else{common.v1})})), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){((v1703*(if (self.scalar_static_f64[202]!=0.0){((v1700*(if v1694{(v1695*v4812)}else{(if v1690{(v1691*v4812)}else{common.v1})}))+(v1699*(self.scalar_static_f64[201]*v4797)))}else{v4730}))+(v1702*(-v4457)))}else{common.v1})})), (common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){((v1703*(if (self.scalar_static_f64[202]!=0.0){((v1700*(if v1694{(v1695*v4813)}else{(if v1690{(v1691*v4813)}else{common.v1})}))+(v1699*(self.scalar_static_f64[201]*v4798)))}else{common.v1}))+(v707*v1702))}else{common.v1})})+v5244)), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){(v1703*(if (self.scalar_static_f64[202]!=0.0){common.v1}else{v4731}))}else{common.v1})})), (common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){(v1703*(if (self.scalar_static_f64[202]!=0.0){((v1700*(if v1694{(v1695*v4814)}else{(if v1690{(v1691*v4814)}else{common.v1})}))+(v1699*(self.scalar_static_f64[201]*v4799)))}else{common.v1}))}else{common.v1})})+v5243)), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if (self.scalar_static_f64[202]!=0.0){(v1703*(if (self.scalar_static_f64[202]!=0.0){common.v1}else{v4732}))}else{common.v1})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(10),
            multiplicity * ((common.v36*(v1559+(v11*common.v758)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4378), (common.v36*v4379), (common.v36*(v4380+v5243)), (common.v36*v4381), (common.v36*v4382), (common.v36*(v4383+v5244)), (common.v36*v4384)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1587),
            0,
            multiplicity * (v707),
            4,
            multiplicity * (v4457),
            5,
            multiplicity * (v4458),
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1843),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4599), (common.v36*v4603), (common.v36*v4607), (common.v36*v4611), (common.v36*v4615), (common.v36*v4619), (common.v36*v4623), (common.v36*v4627)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1611),
            1,
            multiplicity * (v713),
            4,
            multiplicity * (v4628),
            7,
            multiplicity * (v4629),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1613),
            [4, 6, 7, 8, 9],
            [v4638, v4639, v4640, v4641, v4642],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1614),
            2,
            multiplicity * (v719),
            4,
            multiplicity * (v4643),
            9,
            multiplicity * (v4644),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1616),
            [4, 5, 6, 7, 8, 9, 10],
            [v4655, v4656, v4657, v4658, v4659, v4660, v4661],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * ((common.v36*(v1790+(v11*common.v775)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v5028), (common.v36*v5029), (common.v36*v5030), (common.v36*v5031), (common.v36*v5032), (common.v36*(v5033+v5244)), (common.v36*(v5034+v5243))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1845),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v3637), (common.v36*v3638), (common.v36*v3639), (common.v36*v3640), (common.v36*v3641), (common.v36*v3642), (common.v36*v3643)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1617),
            3,
            multiplicity * (v725),
            4,
            multiplicity * (v4662),
            11,
            multiplicity * (v4663),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * ((common.v781-v1140)),
            [4, 6, 8, 9, 13],
            [(-v3417), (-v3420), (-v3424), (-v3428), common.v2],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            None,
            multiplicity * ((common.v781-common.v780)),
            12,
            multiplicity * (common.v26),
            13,
            multiplicity * (common.v2),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((common.v388*v728)),
            4,
            multiplicity * ((v728+(common.v388*(if v726{((-(self.scalar_static_f64[119]*(self.scalar_static_f64[120]*common.v2171)))/(v476*v476))}else{common.v1})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * ((((((((((((((((common.v744*v1428)+(common.v750*v1744))+(v763*v1658))+(common.v747*v1482))+(common.v758*v1559))+(v779*v1617))+(common.v775*v1790))+(v777*v1219))+(v766*v1587))+(v768*v1610))+(v769*v1611))+(v770*v1613))+(v771*v1614))+(v772*v1616))*self.scalar_static_f64[215])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            &[(self.scalar_static_f64[215]*(v1587+v1587)), (self.scalar_static_f64[215]*(v1611+v1611)), (self.scalar_static_f64[215]*(v1614+v1614)), (self.scalar_static_f64[215]*(v1617+v1617)), (self.scalar_static_f64[215]*((((((((((((((common.v744*v4084)+(common.v750*v4930))+(v763*v4733))+(common.v747*v4211))+(common.v758*v4378))+(v779*v4662))+(common.v775*v5028))+(v777*v3637))+(v766*v4457))+(v768*v4599))+(v769*v4628))+(v770*v4638))+(v771*v4643))+(v772*v4655))), (self.scalar_static_f64[215]*(((v1703+(v766*v4458))+(v1843+(v768*v4603)))+((-v1616)+(v772*v4656)))), (self.scalar_static_f64[215]*((((((((((common.v744*v4085)+((v1744*common.v2641)+(common.v750*v4931)))+((common.v36*v1658)+(v763*v4734)))+(common.v747*v4212))+(common.v758*v4379))+(common.v775*v5029))+(v777*v3638))+((v1610*common.v2641)+(v768*v4607)))+(v770*v4639))+(v772*v4657))), (self.scalar_static_f64[215]*((((((((((common.v744*v4041)+(common.v750*v4924))+((common.v36*v1482)+(common.v747*v4168)))+((common.v36*v1559)+(common.v758*v4380)))+(common.v775*v5030))+(v1845+(v777*v3639)))+(v768*v4611))+((-v1611)+(v769*v4629)))+(v1613+(v770*v4640)))+(v772*v4658))), (self.scalar_static_f64[215]*(((((((((((common.v36*v1428)+(common.v744*v4086))+((common.v36*v1744)+(common.v750*v4932)))+(v763*v4735))+(common.v747*v4213))+(common.v758*v4381))+(common.v775*v5031))+(v777*v3640))+(v768*v4615))+((-v1613)+(v770*v4641)))+(v772*v4659))), (self.scalar_static_f64[215]*((((((((((((v1428*common.v2641)+(common.v744*v4087))+(common.v750*v4933))+((v1658*common.v2641)+(v763*v4736)))+((v1482*common.v2641)+(common.v747*v4214)))+(common.v758*v4382))+(common.v775*v5032))+(v777*v3641))+(v768*v4619))+(v770*v4642))+((-v1614)+(v771*v4644)))+(v772*v4660))), (self.scalar_static_f64[215]*((((((((common.v744*v4044)+(common.v750*v4927))+(common.v747*v4171))+((v1559*common.v2641)+(common.v758*v4383)))+((v1790*common.v2641)+(common.v775*v5033)))+(v777*v3642))+(v768*v4623))+(v1616+(v772*v4661)))), (self.scalar_static_f64[215]*((((((((common.v744*v4045)+(common.v750*v4928))+(common.v747*v4172))+(common.v758*v4384))+((-v1617)+(v779*v4663)))+((common.v36*v1790)+(common.v775*v5034)))+((v1219*common.v2641)+(v777*v3643)))+(v768*v4627))), (self.scalar_static_f64[215]*(v763+(common.v750*v4929)))],
            &[],
            &[],
            multiplicity,
        );
        let v2161_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2161);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2161_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v6094) * ddt_scale), ((common.v6095) * ddt_scale), ((common.v6096) * ddt_scale), ((common.v6097) * ddt_scale), ((common.v6098) * ddt_scale), ((common.v6099) * ddt_scale), ((common.v6100) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2162_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2162);
        stamper.stamp_current_node3_local(
            Some(7),
            Some(9),
            multiplicity * (v2162_ddt),
            4,
            multiplicity * (((common.v6101) * ddt_scale)),
            7,
            multiplicity * (((common.v6102) * ddt_scale)),
            9,
            multiplicity * (((common.v6103) * ddt_scale)),
        );
        let v2163_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2163);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2163_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v6104) * ddt_scale), ((common.v6105) * ddt_scale), ((common.v6106) * ddt_scale), ((common.v6107) * ddt_scale), ((common.v6108) * ddt_scale), ((common.v6109) * ddt_scale), ((common.v6110) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2164_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2164);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v2164_ddt),
            [4, 5, 6, 8, 9],
            [((common.v6111) * ddt_scale), ((common.v6112) * ddt_scale), ((common.v6113) * ddt_scale), ((common.v6114) * ddt_scale), ((common.v6115) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2165_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2165);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v2165_ddt),
            [4, 6, 7, 8, 9, 10],
            [((common.v6116) * ddt_scale), ((common.v6117) * ddt_scale), ((common.v6118) * ddt_scale), ((common.v6119) * ddt_scale), ((common.v6120) * ddt_scale), ((common.v6121) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2151_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2151);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2151_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[231]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[269]) * ddt_scale)),
        );
        let v2153_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2153);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2153_ddt),
            0,
            multiplicity * (((self.scalar_static_f64[270]) * ddt_scale)),
            1,
            multiplicity * (((self.scalar_static_f64[232]) * ddt_scale)),
        );
        let v2166_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2166);
        stamper.stamp_current_node3_local(
            Some(11),
            Some(10),
            multiplicity * (v2166_ddt),
            4,
            multiplicity * (((common.v6122) * ddt_scale)),
            10,
            multiplicity * (((common.v6123) * ddt_scale)),
            11,
            multiplicity * (((common.v6124) * ddt_scale)),
        );
        let v2157_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2157);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v2157_ddt),
            12,
            multiplicity * (((self.scalar_static_f64[234]) * ddt_scale)),
        );
        let v2160_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2160);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v2160_ddt),
            13,
            multiplicity * (((self.scalar_static_f64[271]) * ddt_scale)),
        );
        let v2155_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v2155);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v2155_ddt),
            4,
            multiplicity * (((self.scalar_static_f64[233]) * ddt_scale)),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(6),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(10),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(0),
            Some(5),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(1),
            Some(7),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(8),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(2),
            Some(9),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(10),
            Some(5),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(7),
            Some(11),
            multiplicity * (common.v1),
        );
        stamper.stamp_current_const_local(
            Some(3),
            Some(11),
            multiplicity * (common.v1),
        );
    }

    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {
        let nodes = self.nodes;
        let branches = self.branches;
        self.ensure_temperature_static(ctx.temperature(), ctx.thermal_voltage());
        let p = &(*self.params);
        let multiplicity = self.multiplicity;
        let common=self.eval_common_stamp_values(ctx);
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[9]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[common.v6094, common.v6095, common.v6096, common.v6097, common.v6098, common.v6099, common.v6100],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes[4],
            multiplicity * (common.v6101),
            nodes[7],
            multiplicity * (common.v6102),
            nodes[9],
            multiplicity * (common.v6103),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[common.v6104, common.v6105, common.v6106, common.v6107, common.v6108, common.v6109, common.v6110],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &[nodes[4], nodes[5], nodes[6], nodes[8], nodes[9]],
            &[common.v6111, common.v6112, common.v6113, common.v6114, common.v6115],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[common.v6116, common.v6117, common.v6118, common.v6119, common.v6120, common.v6121],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[231]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[269]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (self.scalar_static_f64[270]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[232]),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes[4],
            multiplicity * (common.v6122),
            nodes[10],
            multiplicity * (common.v6123),
            nodes[11],
            multiplicity * (common.v6124),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (self.scalar_static_f64[234]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (self.scalar_static_f64[271]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (self.scalar_static_f64[233]),
        );
    }
}
