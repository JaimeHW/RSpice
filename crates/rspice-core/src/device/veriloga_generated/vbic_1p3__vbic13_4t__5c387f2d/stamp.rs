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
    v98: f64,
    v122: f64,
    v134: f64,
    v136: f64,
    v371: f64,
    v390: f64,
    v391: f64,
    v392: f64,
    v460: f64,
    v479: f64,
    v483: f64,
    v490: f64,
    v497: f64,
    v504: f64,
    v515: f64,
    v552: f64,
    v615: f64,
    v670: f64,
    v671: f64,
    v722: f64,
    v723: f64,
    v725: f64,
    v726: f64,
    v728: f64,
    v729: f64,
    v731: f64,
    v732: f64,
    v737: f64,
    v739: f64,
    v740: f64,
    v741: f64,
    v745: f64,
    v754: f64,
    v756: f64,
    v761: f64,
    v762: f64,
    v1041: f64,
    v1058: f64,
    v1063: f64,
    v1066: f64,
    v1071: f64,
    v1096: f64,
    v1108: f64,
    v1147: f64,
    v1174: f64,
    v1202: f64,
    v1204: f64,
    v1256: f64,
    v1279: f64,
    v1280: f64,
    v1317: f64,
    v1335: f64,
    v1336: f64,
    v1377: f64,
    v1394: f64,
    v1395: f64,
    v1430: f64,
    v1448: f64,
    v1449: f64,
    v1484: f64,
    v1485: f64,
    v1513: f64,
    v1529: f64,
    v1532: f64,
    v1689: f64,
    v1703: f64,
    v2081: f64,
    v2083: f64,
    v2085: f64,
    v2087: f64,
    v2090: f64,
    v2091: f64,
    v2092: f64,
    v2093: f64,
    v2094: f64,
    v2095: f64,
    v2096: f64,
    v2101: f64,
    v2103: f64,
    v2104: f64,
    v2175: f64,
    v2218: f64,
    v2225: f64,
    v2229: f64,
    v2241: f64,
    v2245: f64,
    v2257: f64,
    v2261: f64,
    v2273: f64,
    v2277: f64,
    v2297: f64,
    v2301: f64,
    v2425: f64,
    v2509: f64,
    v2512: f64,
    v2516: f64,
    v2571: f64,
    v3162: f64,
    v3163: f64,
    v3164: f64,
    v3198: f64,
    v3199: f64,
    v3200: f64,
    v3201: f64,
    v3231: f64,
    v3232: f64,
    v3233: f64,
    v3234: f64,
    v3293: f64,
    v3294: f64,
    v3295: f64,
    v3296: f64,
    v3323: f64,
    v3324: f64,
    v3325: f64,
    v3326: f64,
    v3330: f64,
    v3422: f64,
    v3423: f64,
    v3424: f64,
    v3425: f64,
    v3426: f64,
    v3427: f64,
    v3492: f64,
    v3493: f64,
    v3494: f64,
    v3495: f64,
    v3496: f64,
    v3497: f64,
    v3498: f64,
    v3598: f64,
    v3599: f64,
    v3600: f64,
    v3601: f64,
    v3602: f64,
    v3603: f64,
    v3604: f64,
    v3607: f64,
    v3714: f64,
    v3715: f64,
    v3716: f64,
    v3717: f64,
    v3760: f64,
    v3761: f64,
    v3762: f64,
    v3763: f64,
    v3764: f64,
    v3765: f64,
    v3766: f64,
    v3767: f64,
    v3841: f64,
    v3842: f64,
    v3843: f64,
    v3844: f64,
    v3880: f64,
    v3881: f64,
    v3882: f64,
    v3883: f64,
    v3884: f64,
    v3885: f64,
    v3886: f64,
    v3887: f64,
    v4001: f64,
    v4002: f64,
    v4003: f64,
    v4004: f64,
    v4040: f64,
    v4041: f64,
    v4042: f64,
    v4043: f64,
    v4044: f64,
    v4045: f64,
    v4046: f64,
    v4047: f64,
    v4128: f64,
    v4129: f64,
    v4130: f64,
    v4131: f64,
    v4168: f64,
    v4169: f64,
    v4170: f64,
    v4171: f64,
    v4172: f64,
    v4173: f64,
    v4174: f64,
    v4176: f64,
    v4244: f64,
    v4245: f64,
    v4246: f64,
    v4247: f64,
    v4248: f64,
    v4249: f64,
    v4250: f64,
    v4251: f64,
    v4367: f64,
    v4368: f64,
    v4369: f64,
    v4370: f64,
    v4371: f64,
    v4372: f64,
    v4373: f64,
    v4382: f64,
    v4383: f64,
    v4384: f64,
    v4385: f64,
    v4386: f64,
    v4866: f64,
    v4888: f64,
    v4889: f64,
    v4890: f64,
    v4891: f64,
    v4892: f64,
    v4893: f64,
    v4894: f64,
    v6024: f64,
    v6025: f64,
    v6026: f64,
    v6027: f64,
    v6028: f64,
    v6029: f64,
    v6030: f64,
    v6031: f64,
    v6032: f64,
    v6033: f64,
    v6034: f64,
    v6035: f64,
    v6036: f64,
    v6037: f64,
    v6038: f64,
    v6039: f64,
    v6040: f64,
    v6041: f64,
    v6042: f64,
    v6043: f64,
    v6044: f64,
    v6045: f64,
    v6046: f64,
    v6047: f64,
    v6048: f64,
    v6049: f64,
    v6050: f64,
    v6051: f64,
    v6052: f64,
    v6053: f64,
    v6054: f64,
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
        let v94=1.380662e-23;
        let v96=1.602189e-19;
        let v98=(self.scalar_static_f64[266]/v70);
        let v114=(if self.scalar_static_bool[11]{v1}else{(if self.scalar_static_bool[10]{(self.scalar_static_f64[269]*((self.scalar_static_f64[271]+(v18/self.scalar_static_f64[34]))).ln())}else{v1})});
        let v122=(v2-v98);
        let v127=((self.scalar_static_f64[33]*f64::powf(v98,self.scalar_static_f64[40]))*(((self.scalar_static_f64[42]*v122)/self.scalar_static_f64[272])).exp());
        let v128=(v127>v1);
        let v132=(self.scalar_static_bool[12]&&(v18>self.scalar_static_f64[43]));
        let v134=0.5;
        let v135=(v18*v134);
        let v136=4.0;
        let v157=(if (!v128){v1}else{(if (v128&&(!v132)){(self.scalar_static_f64[272]*((v2+(v18/v127))).ln())}else{(if (v128&&v132){(self.scalar_static_f64[272]*((v2+(f64::powf((v135*self.scalar_static_f64[46]),self.scalar_static_f64[48])/v127))).ln())}else{v1})})});
        let v170=((self.scalar_static_f64[49]*f64::powf(v98,self.scalar_static_f64[52]))*(((v122*self.scalar_static_f64[54])/self.scalar_static_f64[273])).exp());
        let v172=(v128&&(v170>v1));
        let v174=(self.scalar_static_bool[5]&&(v18>self.scalar_static_f64[10]));
        let v180=(v127*v170);
        let v194=(if (!v172){v1}else{(if (v172&&(!v174)){(self.scalar_static_f64[273]*((v2+(v18/v180))).ln())}else{(if (v172&&v174){(self.scalar_static_f64[273]*((v2+(f64::powf((v135*self.scalar_static_f64[56]),self.scalar_static_f64[48])/v180))).ln())}else{v1})})});
        let v206=((self.scalar_static_f64[57]*f64::powf(v98,self.scalar_static_f64[59]))*(((v122*self.scalar_static_f64[61])/self.scalar_static_f64[274])).exp());
        let v207=(v206>v1);
        let v209=(self.scalar_static_bool[6]&&(v18>self.scalar_static_f64[13]));
        let v226=(if (!v207){v1}else{(if (v207&&(!v209)){(self.scalar_static_f64[274]*((v2+(v18/v206))).ln())}else{(if (v207&&v209){(self.scalar_static_f64[274]*((v2+((v49*(v18*v18))/v206))).ln())}else{v1})})});
        let v239=((self.scalar_static_f64[62]*f64::powf(v98,self.scalar_static_f64[65]))*(((v122*self.scalar_static_f64[67])/self.scalar_static_f64[275])).exp());
        let v240=(v239>v1);
        let v247=(if (!v240){v1}else{(if v240{(self.scalar_static_f64[275]*((v2+(v18/v239))).ln())}else{v1})});
        let v272=f64::powf(v98,self.scalar_static_f64[76]);
        let v279=(((v122*self.scalar_static_f64[78])/self.scalar_static_f64[277])).exp();
        let v280=((self.scalar_static_f64[74]*v272)*v279);
        let v281=(v280>v1);
        let v288=(if (!v281){v1}else{(if v281{(self.scalar_static_f64[277]*((v2+(v18/v280))).ln())}else{v1})});
        let v311=(v279*(v272*self.scalar_static_f64[84]));
        let v312=(v311>v1);
        let v319=(if (!v312){v1}else{(if v312{(self.scalar_static_f64[277]*((v2+(v18/v311))).ln())}else{v1})});
        let v342=((self.scalar_static_f64[86]*f64::powf(v98,self.scalar_static_f64[88]))*(((v122*self.scalar_static_f64[90])/self.scalar_static_f64[279])).exp());
        let v343=(v342>v1);
        let v350=(if (!v343){v1}else{(if v343{(self.scalar_static_f64[279]*((v2+(v18/v342))).ln())}else{v1})});
        let v371=ctx.node_voltage(nodes[4]);
        let v373=((self.scalar_static_f64[254]+v371)-v67);
        let v374=(v373<self.scalar_static_f64[30]);
        let v377=(((v373-self.scalar_static_f64[29])-v2)).exp();
        let v379=(if v374{(self.scalar_static_f64[29]+v377)}else{v373});
        let v382=((v379>self.scalar_static_f64[32])&&(!v374));
        let v385=(((self.scalar_static_f64[31]-v379)-v2)).exp();
        let v388=(v67+(if v382{(self.scalar_static_f64[31]-v385)}else{v379}));
        let v390=((v94*v388)/v96);
        let v391=(v388/v70);
        let v392=(v388-v70);
        let v395=(self.scalar_static_f64[43]*f64::powf(v391,self.scalar_static_f64[96]));
        let v459=(self.scalar_static_f64[33]*f64::powf(v391,self.scalar_static_f64[40]));
        let v460=(v2-v391);
        let v461=(self.scalar_static_f64[42]*v460);
        let v462=(self.scalar_static_f64[39]*v390);
        let v464=((v461/v462)).exp();
        let v465=(v459*v464);
        let v467=(self.scalar_static_f64[49]*f64::powf(v391,self.scalar_static_f64[52]));
        let v468=(self.scalar_static_f64[54]*v460);
        let v469=(self.scalar_static_f64[51]*v390);
        let v471=((v468/v469)).exp();
        let v472=(v467*v471);
        let v474=(self.scalar_static_f64[57]*f64::powf(v391,self.scalar_static_f64[59]));
        let v475=(self.scalar_static_f64[61]*v460);
        let v476=(self.scalar_static_f64[58]*v390);
        let v478=((v475/v476)).exp();
        let v479=(v474*v478);
        let v483=(self.scalar_static_f64[64]*v390);
        let v490=(self.scalar_static_f64[70]*v390);
        let v497=(self.scalar_static_f64[75]*v390);
        let v504=(self.scalar_static_f64[80]*v390);
        let v515=(self.scalar_static_f64[87]*v390);
        let v528=(v2+(v392*self.scalar_static_f64[120]));
        let v529=(self.scalar_static_f64[39]*v528);
        let v530=(self.scalar_static_f64[51]*v528);
        let v544=(self.scalar_static_f64[125]+(v392*self.scalar_static_f64[126]));
        let v551=(self.scalar_static_f64[35]*(v2+(v392*self.scalar_static_f64[127])));
        let v552=2.0;
        let v554=(v552*(v390/v391));
        let v557=(v391*self.scalar_static_f64[129]);
        let v559=((v557/v390)).exp();
        let v560=-0.5;
        let v562=(v391*self.scalar_static_f64[130]);
        let v564=((v562/v390)).exp();
        let v565=(v559-v564);
        let v566=(v565).ln();
        let v567=(v554*v566);
        let v569=3.0;
        let v570=(v390*v569);
        let v571=(v391).ln();
        let v572=(v570*v571);
        let v574=(v391-v2);
        let v576=(((v391*v567)-v572)-(self.scalar_static_f64[66]*v574));
        let v577=(v390*v552);
        let v578=(-v576);
        let v580=((v578/v390)).exp();
        let v583=((v2+(v136*v580))).sqrt();
        let v585=(v134*(v2+v583));
        let v586=(v585).ln();
        let v588=(v576+(v577*v586));
        let v591=(v391*self.scalar_static_f64[132]);
        let v593=((v591/v390)).exp();
        let v595=(v391*self.scalar_static_f64[133]);
        let v597=((v595/v390)).exp();
        let v598=(v593-v597);
        let v599=(v598).ln();
        let v600=(v554*v599);
        let v604=(((v391*v600)-v572)-(self.scalar_static_f64[77]*v574));
        let v605=(-v604);
        let v607=((v605/v390)).exp();
        let v610=((v2+(v136*v607))).sqrt();
        let v612=(v134*(v2+v610));
        let v613=(v612).ln();
        let v615=(v604+(v577*v613));
        let v618=(v391*self.scalar_static_f64[135]);
        let v620=((v618/v390)).exp();
        let v622=(v391*self.scalar_static_f64[136]);
        let v624=((v622/v390)).exp();
        let v625=(v620-v624);
        let v626=(v625).ln();
        let v627=(v554*v626);
        let v631=(((v391*v627)-v572)-(self.scalar_static_f64[89]*v574));
        let v632=(-v631);
        let v634=((v632/v390)).exp();
        let v637=((v2+(v136*v634))).sqrt();
        let v639=(v134*(v2+v637));
        let v640=(v639).ln();
        let v642=(v631+(v577*v640));
        let v644=(self.scalar_static_f64[128]/v588);
        let v647=(self.scalar_static_f64[137]*f64::powf(v644,self.scalar_static_f64[138]));
        let v649=(self.scalar_static_f64[131]/v615);
        let v651=f64::powf(v649,self.scalar_static_f64[140]);
        let v652=(self.scalar_static_f64[139]*v651);
        let v654=(v651*self.scalar_static_f64[141]);
        let v656=(self.scalar_static_f64[134]/v642);
        let v659=(self.scalar_static_f64[142]*f64::powf(v656,self.scalar_static_f64[143]));
        let v662=(self.scalar_static_f64[144]*f64::powf(v391,self.scalar_static_f64[38]));
        let v664=((v461/v390)).exp();
        let v665=(v662*v664);
        let v670=(-(self.scalar_static_f64[36]*(v2+(v392*v544))));
        let v671=(v390*v551);
        let v678=(self.scalar_static_f64[147]*(v2+(v392*self.scalar_static_f64[148])));
        let v683=(self.scalar_static_f64[149]*(v2+(v392*self.scalar_static_f64[150])));
        let v710=(v678>v1);
        let v712=(if v710{(v2/v678)}else{v1});
        let v713=(v683>v1);
        let v715=(if v713{(v2/v683)}else{v1});
        let v716=(v395>v1);
        let v718=(if v716{(v2/v395)}else{v1});
        let v722=ctx.node_voltage(nodes[8]);
        let v723=ctx.node_voltage(nodes[9]);
        let v725=(v36*(v722-v723));
        let v726=ctx.node_voltage(nodes[7]);
        let v728=(v36*(v726-v723));
        let v729=ctx.node_voltage(nodes[6]);
        let v731=(v36*(v722-v729));
        let v732=ctx.node_voltage(nodes[5]);
        let v734=(v36*(v722-v732));
        let v737=ctx.node_voltage(nodes[10]);
        let v739=(v36*(v726-v737));
        let v740=ctx.node_voltage(nodes[1]);
        let v741=ctx.node_voltage(nodes[2]);
        let v745=ctx.node_voltage(nodes[0]);
        let v754=ctx.node_voltage(nodes[11]);
        let v756=(v36*(v754-v737));
        let v761=ctx.node_voltage(nodes[12]);
        let v762=ctx.node_voltage(nodes[13]);
        let v763=(-v588);
        let v765=(v763*self.scalar_static_f64[151]);
        let v768=(v725+v765);
        let v769=(if self.scalar_static_bool[18]{v768}else{v1});
        let v770=(v769>v1);
        let v771=(self.scalar_static_bool[18]&&v770);
        let v775=(if v771{self.scalar_static_f64[155]}else{v1});
        let v777=(v2-(self.scalar_static_f64[153]*v775));
        let v783=(v769*self.scalar_static_f64[157]);
        let v784=(v588*self.scalar_static_f64[153]);
        let v786=(v2+(v783/v784));
        let v791=(self.scalar_static_bool[18]&&(!v770));
        let v793=(v2-(v725/v588));
        let v795=(v2-f64::powf(v793,self.scalar_static_f64[156]));
        let v798=(if v791{((v588*v795)/self.scalar_static_f64[156])}else{(if v771{((v588*v777)/self.scalar_static_f64[156])}else{v1})});
        let v807=(((v765*v765)+self.scalar_static_f64[159])).sqrt();
        let v811=(if self.scalar_static_bool[19]{(v560*(v765+(if self.scalar_static_bool[19]{v807}else{v1})))}else{v1});
        let v813=(v2-(v811/v588));
        let v814=f64::powf(v813,self.scalar_static_f64[156]);
        let v817=(if self.scalar_static_bool[19]{((v763*v814)/self.scalar_static_f64[156])}else{v1});
        let v818=(if self.scalar_static_bool[19]{v768}else{v1});
        let v821=((self.scalar_static_f64[159]+(v818*v818))).sqrt();
        let v826=(if self.scalar_static_bool[19]{((v134*(v818-(if self.scalar_static_bool[19]{v821}else{v1})))-v765)}else{v1});
        let v828=(v2-(v826/v588));
        let v829=f64::powf(v828,self.scalar_static_f64[156]);
        let v834=(v811+(v725-v826));
        let v835=(self.scalar_static_f64[155]*v834);
        let v836=(self.scalar_static_f64[157]*v834);
        let v838=(v2+(v836/v784));
        let v842=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{((v763*v829)/self.scalar_static_f64[156])}else{v798})+(v835*v838))-v817)}else{(if self.scalar_static_bool[18]{(v798+(if v791{v1}else{(if v771{(v775*(v769*v786))}else{v1})}))}else{v1})});
        let v843=(-v615);
        let v844=(self.scalar_static_f64[151]*v843);
        let v847=(v731+v844);
        let v848=(if self.scalar_static_bool[20]{v847}else{v1});
        let v849=(v848>v1);
        let v850=(self.scalar_static_bool[20]&&v849);
        let v853=(if v850{self.scalar_static_f64[162]}else{v1});
        let v856=(v2-(self.scalar_static_f64[153]*(self.scalar_static_f64[153]*v853)));
        let v862=(v848*self.scalar_static_f64[164]);
        let v864=(self.scalar_static_f64[153]+(v862/v615));
        let v872=(self.scalar_static_bool[21]&&(v731<self.scalar_static_f64[166]));
        let v874=(self.scalar_static_bool[20]&&(!v849));
        let v875=(v872&&v874);
        let v877=(v2+(self.scalar_static_f64[165]/v615));
        let v878=f64::powf(v877,self.scalar_static_f64[163]);
        let v880=(self.scalar_static_f64[163]*(v731+self.scalar_static_f64[165]));
        let v881=(v615+self.scalar_static_f64[165]);
        let v883=(v2-(v880/v881));
        let v885=(v2-(v878*v883));
        let v890=(v874&&(!v872));
        let v892=(v2-(v731/v615));
        let v894=(v2-f64::powf(v892,self.scalar_static_f64[163]));
        let v897=(if v890{((v615*v894)/self.scalar_static_f64[163])}else{(if v875{((v615*v885)/self.scalar_static_f64[163])}else{(if v850{((v615*v856)/self.scalar_static_f64[163])}else{v1})})});
        let v906=(v844+self.scalar_static_f64[165]);
        let v907=(self.scalar_static_f64[165]-v844);
        let v909=(if self.scalar_static_bool[25]{(v906/v907)}else{v1});
        let v910=(v552*v909);
        let v911=(v909-v2);
        let v916=(((v911*v911)+self.scalar_static_f64[169])).sqrt();
        let v917=(v2+v909);
        let v922=(((v917*v917)+self.scalar_static_f64[171])).sqrt();
        let v923=(v916+v922);
        let v925=(if self.scalar_static_bool[25]{(v910/v923)}else{v1});
        let v930=(if self.scalar_static_bool[25]{(v134*(((v907*v925)-self.scalar_static_f64[165])-v844))}else{v1});
        let v932=(v2-(v930/v615));
        let v934=(v2-f64::powf(v932,self.scalar_static_f64[163]));
        let v937=(if self.scalar_static_bool[25]{((v615*v934)/self.scalar_static_f64[163])}else{v1});
        let v940=(v844+(self.scalar_static_f64[165]+(v552*v731)));
        let v942=(if self.scalar_static_bool[25]{(v940/v907)}else{v1});
        let v943=(v552*v942);
        let v944=(v942-v2);
        let v947=((self.scalar_static_f64[169]+(v944*v944))).sqrt();
        let v948=(v2+v942);
        let v951=((self.scalar_static_f64[171]+(v948*v948))).sqrt();
        let v952=(v947+v951);
        let v954=(if self.scalar_static_bool[25]{(v943/v952)}else{v1});
        let v959=(if self.scalar_static_bool[25]{(v134*(((v907*v954)-self.scalar_static_f64[165])-v844))}else{v1});
        let v961=(v2-(v959/v615));
        let v963=(v2-f64::powf(v961,self.scalar_static_f64[163]));
        let v966=(if self.scalar_static_bool[25]{((v615*v963)/self.scalar_static_f64[163])}else{v897});
        let v969=(if self.scalar_static_bool[25]{(v134*(v2+v954))}else{v1});
        let v972=(if self.scalar_static_bool[25]{f64::powf(v877,self.scalar_static_f64[172])}else{v1});
        let v974=(v2+(v844/v615));
        let v976=(if self.scalar_static_bool[25]{f64::powf(v974,self.scalar_static_f64[172])}else{v1});
        let v977=(v2-v969);
        let v981=(if self.scalar_static_bool[25]{((v972*v977)+(v969*v976))}else{v1});
        let v983=(v930+(v731-v959));
        let v993=((self.scalar_static_f64[169]+(v844*v844))).sqrt();
        let v997=(if self.scalar_static_bool[27]{(v560*(v844+(if self.scalar_static_bool[27]{v993}else{v1})))}else{v930});
        let v999=(v2-(v997/v615));
        let v1000=f64::powf(v999,self.scalar_static_f64[163]);
        let v1003=(if self.scalar_static_bool[27]{((v843*v1000)/self.scalar_static_f64[163])}else{v1});
        let v1004=(if self.scalar_static_bool[27]{v847}else{v1});
        let v1007=((self.scalar_static_f64[169]+(v1004*v1004))).sqrt();
        let v1012=(if self.scalar_static_bool[27]{((v134*(v1004-(if self.scalar_static_bool[27]{v1007}else{v1})))-v844)}else{v959});
        let v1014=(v2-(v1012/v615));
        let v1015=f64::powf(v1014,self.scalar_static_f64[163]);
        let v1025=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{((v843*v1015)/self.scalar_static_f64[163])}else{v966})+(self.scalar_static_f64[173]*(v997+(v731-v1012))))-v1003)}else{(if self.scalar_static_bool[25]{((v966+(if self.scalar_static_bool[25]{(v981*v983)}else{v1}))-v937)}else{(if self.scalar_static_bool[20]{(v897+(if v874{v1}else{(if v850{(v853*(v848*v864))}else{v1})}))}else{v1})})});
        let v1026=(v390*v529);
        let v1027=(v2/v1026);
        let v1028=(v725<v157);
        let v1030=((v725*v1027)).exp();
        let v1032=(!v1028);
        let v1034=((v157*v1027)).exp();
        let v1035=(v725-v157);
        let v1037=(v2+(v1027*v1035));
        let v1039=(if v1032{(v1034*v1037)}else{(if v1028{v1030}else{v1})});
        let v1040=(v1039-v2);
        let v1041=(v465*v1040);
        let v1042=(v390*v530);
        let v1043=(v2/v1042);
        let v1044=(v731<v194);
        let v1046=((v731*v1043)).exp();
        let v1048=(!v1044);
        let v1050=((v194*v1043)).exp();
        let v1051=(v731-v194);
        let v1053=(v2+(v1043*v1051));
        let v1055=(if v1048{(v1050*v1053)}else{(if v1044{v1046}else{v1039})});
        let v1056=(v465*v472);
        let v1057=(v1055-v2);
        let v1058=(v1056*v1057);
        let v1063=0.0001;
        let v1064=(((v2+(v715*v842))+(v712*v1025))-v1063);
        let v1066=1e-8;
        let v1068=(((v1064*v1064)+v1066)).sqrt();
        let v1071=(v1063+(v134*(v1064+v1068)));
        let v1079=(v136*((v718*v1041)+(v44*v1058)));
        let v1081=(if self.scalar_static_bool[28]{(f64::powf(v1071,self.scalar_static_f64[175])+v1079)}else{v1});
        let v1082=(v1081>v1066);
        let v1083=(self.scalar_static_bool[28]&&v1082);
        let v1089=(self.scalar_static_bool[28]&&(!v1082));
        let v1096=(if self.scalar_static_bool[29]{(v2+v1079)}else{v1081});
        let v1097=(v1096>v1066);
        let v1098=(self.scalar_static_bool[29]&&v1097);
        let v1099=(v134*v1071);
        let v1101=(v2+f64::powf(v1096,self.scalar_static_f64[45]));
        let v1105=(self.scalar_static_bool[29]&&(!v1097));
        let v1108=(if v1105{(v1099*self.scalar_static_f64[177])}else{(if v1098{(v1099*v1101)}else{(if v1089{(v134*(v1071+self.scalar_static_f64[176]))}else{(if v1083{(v134*(v1071+f64::powf(v1081,self.scalar_static_f64[45])))}else{v1})})})});
        let v1113=(if self.scalar_static_bool[30]{(v2/v476)}else{v1043});
        let v1114=(v739<v226);
        let v1115=(self.scalar_static_bool[30]&&v1114);
        let v1117=((v739*v1113)).exp();
        let v1120=(self.scalar_static_bool[30]&&(!v1114));
        let v1122=((v226*v1113)).exp();
        let v1123=(v739-v226);
        let v1125=(v2+(v1113*v1123));
        let v1127=(if v1120{(v1122*v1125)}else{(if v1115{v1117}else{v1055})});
        let v1128=(v731<v226);
        let v1129=(self.scalar_static_bool[30]&&v1128);
        let v1131=((v731*v1113)).exp();
        let v1134=(self.scalar_static_bool[30]&&(!v1128));
        let v1135=(v731-v226);
        let v1137=(v2+(v1113*v1135));
        let v1139=(if v1134{(v1122*v1137)}else{(if v1129{v1131}else{v1})});
        let v1145=(((v1127*self.scalar_static_f64[178])+(v1139*self.scalar_static_f64[179]))-v2);
        let v1147=(if self.scalar_static_bool[30]{(v479*v1145)}else{v1});
        let v1163=(v756<v226);
        let v1164=(self.scalar_static_bool[30]&&v1163);
        let v1166=((v756*v1113)).exp();
        let v1169=(self.scalar_static_bool[30]&&(!v1163));
        let v1170=(v756-v226);
        let v1172=(v2+(v1113*v1170));
        let v1174=(if v1169{(v1122*v1172)}else{(if v1164{v1166}else{v1127})});
        let v1187=(v2/v483);
        let v1188=(if self.scalar_static_bool[32]{v1187}else{v1113});
        let v1189=(v725<v247);
        let v1190=(self.scalar_static_bool[32]&&v1189);
        let v1192=((v725*v1188)).exp();
        let v1194=(!v1189);
        let v1195=(self.scalar_static_bool[32]&&v1194);
        let v1197=((v247*v1188)).exp();
        let v1198=(v725-v247);
        let v1200=(v2+(v1188*v1198));
        let v1202=(if v1195{(v1197*v1200)}else{(if v1190{v1192}else{v1174})});
        let v1203=(v2/v490);
        let v1204=(if self.scalar_static_bool[32]{v1203}else{v1188});
        let v1239=(v670-v725);
        let v1240=(if self.scalar_static_bool[38]{v1239}else{v1});
        let v1241=(v2/v671);
        let v1242=(if self.scalar_static_bool[38]{v1241}else{v1204});
        let v1243=(v1240<v114);
        let v1244=(self.scalar_static_bool[38]&&v1243);
        let v1246=((v1240*v1242)).exp();
        let v1249=(self.scalar_static_bool[38]&&(!v1243));
        let v1251=((v114*v1242)).exp();
        let v1252=(v1240-v114);
        let v1254=(v2+(v1242*v1252));
        let v1256=(if v1249{(v1251*v1254)}else{(if v1244{v1246}else{v1139})});
        let v1265=(if self.scalar_static_bool[41]{v1187}else{v1242});
        let v1266=(v728<v247);
        let v1267=(self.scalar_static_bool[41]&&v1266);
        let v1269=((v728*v1265)).exp();
        let v1271=(!v1266);
        let v1272=(self.scalar_static_bool[41]&&v1271);
        let v1274=((v247*v1265)).exp();
        let v1275=(v728-v247);
        let v1277=(v2+(v1265*v1275));
        let v1279=(if v1272{(v1274*v1277)}else{(if v1267{v1269}else{v1202})});
        let v1280=(if self.scalar_static_bool[41]{v1203}else{v1265});
        let v1302=(if self.scalar_static_bool[42]{v1239}else{v1240});
        let v1303=(if self.scalar_static_bool[42]{v1241}else{v1280});
        let v1304=(v1302<v114);
        let v1305=(self.scalar_static_bool[42]&&v1304);
        let v1307=((v1302*v1303)).exp();
        let v1310=(self.scalar_static_bool[42]&&(!v1304));
        let v1312=((v114*v1303)).exp();
        let v1313=(v1302-v114);
        let v1315=(v2+(v1303*v1313));
        let v1317=(if v1310{(v1312*v1315)}else{(if v1305{v1307}else{v1256})});
        let v1324=(if self.scalar_static_bool[44]{v1187}else{v1303});
        let v1325=(v1189&&self.scalar_static_bool[44]);
        let v1327=((v725*v1324)).exp();
        let v1329=(v1194&&self.scalar_static_bool[44]);
        let v1331=((v247*v1324)).exp();
        let v1333=(v2+(v1198*v1324));
        let v1335=(if v1329{(v1331*v1333)}else{(if v1325{v1327}else{v1279})});
        let v1336=(if self.scalar_static_bool[44]{v1203}else{v1324});
        let v1362=(if self.scalar_static_bool[47]{v1239}else{v1302});
        let v1363=(if self.scalar_static_bool[47]{v1241}else{v1336});
        let v1364=(v1362<v114);
        let v1365=(self.scalar_static_bool[47]&&v1364);
        let v1367=((v1362*v1363)).exp();
        let v1370=(self.scalar_static_bool[47]&&(!v1364));
        let v1372=((v114*v1363)).exp();
        let v1373=(v1362-v114);
        let v1375=(v2+(v1363*v1373));
        let v1377=(if v1370{(v1372*v1375)}else{(if v1365{v1367}else{v1317})});
        let v1383=(if self.scalar_static_bool[44]{v1187}else{v1363});
        let v1384=(v1266&&self.scalar_static_bool[44]);
        let v1386=((v728*v1383)).exp();
        let v1388=(v1271&&self.scalar_static_bool[44]);
        let v1390=((v247*v1383)).exp();
        let v1392=(v2+(v1275*v1383));
        let v1394=(if v1388{(v1390*v1392)}else{(if v1384{v1386}else{v1335})});
        let v1395=(if self.scalar_static_bool[44]{v1203}else{v1383});
        let v1415=(if self.scalar_static_bool[47]{v1239}else{v1362});
        let v1416=(if self.scalar_static_bool[47]{v1241}else{v1395});
        let v1417=(v1415<v114);
        let v1418=(self.scalar_static_bool[47]&&v1417);
        let v1420=((v1415*v1416)).exp();
        let v1423=(self.scalar_static_bool[47]&&(!v1417));
        let v1425=((v114*v1416)).exp();
        let v1426=(v1415-v114);
        let v1428=(v2+(v1416*v1426));
        let v1430=(if v1423{(v1425*v1428)}else{(if v1418{v1420}else{v1377})});
        let v1436=(v2/v497);
        let v1437=(v731<v288);
        let v1439=((v731*v1436)).exp();
        let v1441=(!v1437);
        let v1443=((v288*v1436)).exp();
        let v1444=(v731-v288);
        let v1446=(v2+(v1436*v1444));
        let v1448=(if v1441{(v1443*v1446)}else{(if v1437{v1439}else{v1394})});
        let v1449=(v2/v504);
        let v1470=(if self.scalar_static_bool[50]{v1436}else{v1449});
        let v1471=(v739<v319);
        let v1472=(self.scalar_static_bool[50]&&v1471);
        let v1474=((v739*v1470)).exp();
        let v1477=(self.scalar_static_bool[50]&&(!v1471));
        let v1479=((v319*v1470)).exp();
        let v1480=(v739-v319);
        let v1482=(v2+(v1470*v1480));
        let v1484=(if v1477{(v1479*v1482)}else{(if v1472{v1474}else{v1448})});
        let v1485=(if self.scalar_static_bool[50]{v1449}else{v1470});
        let v1508=(v731/v390);
        let v1509=(v1508<v39);
        let v1510=(v1508).exp();
        let v1512=(!v1509);
        let v1513=(v39).exp();
        let v1517=(if v1512{(v1513*(v2+(v1508-v39)))}else{(if v1509{v1510}else{v1484})});
        let v1518=(v734/v390);
        let v1519=(v1518<v39);
        let v1520=(v1518).exp();
        let v1522=(!v1519);
        let v1526=(if v1522{(v1513*(v2+(v1518-v39)))}else{(if v1519{v1520}else{v1430})});
        let v1529=((v2+(v665*v1517))).sqrt();
        let v1532=((v2+(v665*v1526))).sqrt();
        let v1689=(if self.scalar_static_bool[66]{(v2/v515)}else{v1485});
        let v1690=(v756<v350);
        let v1691=(self.scalar_static_bool[66]&&v1690);
        let v1693=((v756*v1689)).exp();
        let v1696=(self.scalar_static_bool[66]&&(!v1690));
        let v1698=((v350*v1689)).exp();
        let v1699=(v756-v350);
        let v1701=(v2+(v1689*v1699));
        let v1703=(if v1696{(v1698*v1701)}else{(if v1691{v1693}else{v1517})});
        let v1784=(-v642);
        let v1786=(if self.scalar_static_bool[68]{(self.scalar_static_f64[151]*v1784)}else{v1});
        let v1790=(v756+v1786);
        let v1791=(if self.scalar_static_bool[70]{v1790}else{v1});
        let v1792=(v1791>v1);
        let v1793=(self.scalar_static_bool[70]&&v1792);
        let v1796=(if v1793{self.scalar_static_f64[202]}else{v1});
        let v1798=(v2-(self.scalar_static_f64[153]*v1796));
        let v1804=(v1791*self.scalar_static_f64[204]);
        let v1805=(v642*self.scalar_static_f64[153]);
        let v1807=(v2+(v1804/v1805));
        let v1812=(self.scalar_static_bool[70]&&(!v1792));
        let v1814=(v2-(v756/v642));
        let v1816=(v2-f64::powf(v1814,self.scalar_static_f64[203]));
        let v1819=(if v1812{((v642*v1816)/self.scalar_static_f64[203])}else{(if v1793{((v642*v1798)/self.scalar_static_f64[203])}else{v1})});
        let v1829=(((v1786*v1786)+self.scalar_static_f64[206])).sqrt();
        let v1833=(if self.scalar_static_bool[72]{(v560*(v1786+(if self.scalar_static_bool[72]{v1829}else{v1})))}else{v1});
        let v1835=(v2-(v1833/v642));
        let v1836=f64::powf(v1835,self.scalar_static_f64[203]);
        let v1840=(if self.scalar_static_bool[72]{v1790}else{v1});
        let v1843=((self.scalar_static_f64[206]+(v1840*v1840))).sqrt();
        let v1848=(if self.scalar_static_bool[72]{((v134*(v1840-(if self.scalar_static_bool[72]{v1843}else{v1})))-v1786)}else{v1});
        let v1850=(v2-(v1848/v642));
        let v1851=f64::powf(v1850,self.scalar_static_f64[203]);
        let v1856=(v1833+(v756-v1848));
        let v1857=(self.scalar_static_f64[202]*v1856);
        let v1858=(self.scalar_static_f64[204]*v1856);
        let v1860=(v2+(v1858/v1805));
        let v1866=(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{(((if self.scalar_static_bool[72]{((v1784*v1851)/self.scalar_static_f64[203])}else{v1819})+(v1857*v1860))-(if self.scalar_static_bool[72]{((v1784*v1836)/self.scalar_static_f64[203])}else{v1}))}else{(if self.scalar_static_bool[70]{(v1819+(if v1812{v1}else{(if v1793{(v1796*(v1791*v1807))}else{v1})}))}else{v1})})});
        let v1867=(v728+v765);
        let v1868=(if self.scalar_static_bool[18]{v1867}else{v1});
        let v1869=(v1868>v1);
        let v1870=(self.scalar_static_bool[18]&&v1869);
        let v1871=(if v1870{self.scalar_static_f64[155]}else{v1});
        let v1873=(v2-(self.scalar_static_f64[153]*v1871));
        let v1877=(self.scalar_static_f64[157]*v1868);
        let v1879=(v2+(v1877/v784));
        let v1884=(self.scalar_static_bool[18]&&(!v1869));
        let v1886=(v2-(v728/v588));
        let v1888=(v2-f64::powf(v1886,self.scalar_static_f64[156]));
        let v1891=(if v1884{((v588*v1888)/self.scalar_static_f64[156])}else{(if v1870{((v588*v1873)/self.scalar_static_f64[156])}else{v1})});
        let v1895=(if self.scalar_static_bool[19]{v1867}else{v1});
        let v1898=((self.scalar_static_f64[159]+(v1895*v1895))).sqrt();
        let v1903=(if self.scalar_static_bool[19]{((v134*(v1895-(if self.scalar_static_bool[19]{v1898}else{v1})))-v765)}else{v1});
        let v1905=(v2-(v1903/v588));
        let v1906=f64::powf(v1905,self.scalar_static_f64[156]);
        let v1911=(v811+(v728-v1903));
        let v1912=(self.scalar_static_f64[155]*v1911);
        let v1913=(self.scalar_static_f64[157]*v1911);
        let v1915=(v2+(v1913/v784));
        let v1919=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{((v763*v1906)/self.scalar_static_f64[156])}else{v1891})+(v1912*v1915))-v817)}else{(if self.scalar_static_bool[18]{(v1891+(if v1884{v1}else{(if v1870{(v1871*(v1868*v1879))}else{v1})}))}else{v1})});
        let v1920=(v739+v844);
        let v1921=(if self.scalar_static_bool[20]{v1920}else{v1});
        let v1922=(v1921>v1);
        let v1923=(self.scalar_static_bool[20]&&v1922);
        let v1924=(if v1923{self.scalar_static_f64[162]}else{v1});
        let v1927=(v2-(self.scalar_static_f64[153]*(self.scalar_static_f64[153]*v1924)));
        let v1931=(self.scalar_static_f64[164]*v1921);
        let v1933=(self.scalar_static_f64[153]+(v1931/v615));
        let v1938=(self.scalar_static_bool[21]&&(v739<self.scalar_static_f64[166]));
        let v1940=(self.scalar_static_bool[20]&&(!v1922));
        let v1941=(v1938&&v1940);
        let v1943=(self.scalar_static_f64[163]*(v739+self.scalar_static_f64[165]));
        let v1945=(v2-(v1943/v881));
        let v1947=(v2-(v878*v1945));
        let v1952=(v1940&&(!v1938));
        let v1954=(v2-(v739/v615));
        let v1956=(v2-f64::powf(v1954,self.scalar_static_f64[163]));
        let v1959=(if v1952{((v615*v1956)/self.scalar_static_f64[163])}else{(if v1941{((v615*v1947)/self.scalar_static_f64[163])}else{(if v1923{((v615*v1927)/self.scalar_static_f64[163])}else{v1})})});
        let v1965=(v844+(self.scalar_static_f64[165]+(v552*v739)));
        let v1967=(if self.scalar_static_bool[25]{(v1965/v907)}else{v1});
        let v1968=(v552*v1967);
        let v1969=(v1967-v2);
        let v1972=((self.scalar_static_f64[169]+(v1969*v1969))).sqrt();
        let v1973=(v2+v1967);
        let v1976=((self.scalar_static_f64[171]+(v1973*v1973))).sqrt();
        let v1977=(v1972+v1976);
        let v1979=(if self.scalar_static_bool[25]{(v1968/v1977)}else{v1});
        let v1984=(if self.scalar_static_bool[25]{(v134*(((v907*v1979)-self.scalar_static_f64[165])-v844))}else{v1});
        let v1986=(v2-(v1984/v615));
        let v1988=(v2-f64::powf(v1986,self.scalar_static_f64[163]));
        let v1991=(if self.scalar_static_bool[25]{((v615*v1988)/self.scalar_static_f64[163])}else{v1959});
        let v1994=(if self.scalar_static_bool[25]{(v134*(v2+v1979))}else{v1});
        let v1995=(v2-v1994);
        let v1999=(if self.scalar_static_bool[25]{((v972*v1995)+(v976*v1994))}else{v1});
        let v2001=(v930+(v739-v1984));
        let v2007=(if self.scalar_static_bool[27]{v1920}else{v1});
        let v2010=((self.scalar_static_f64[169]+(v2007*v2007))).sqrt();
        let v2015=(if self.scalar_static_bool[27]{((v134*(v2007-(if self.scalar_static_bool[27]{v2010}else{v1})))-v844)}else{v1984});
        let v2017=(v2-(v2015/v615));
        let v2018=f64::powf(v2017,self.scalar_static_f64[163]);
        let v2027=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{((v843*v2018)/self.scalar_static_f64[163])}else{v1991})+(self.scalar_static_f64[173]*(v997+(v739-v2015))))-v1003)}else{(if self.scalar_static_bool[25]{((v1991+(if self.scalar_static_bool[25]{(v1999*v2001)}else{v1}))-v937)}else{(if self.scalar_static_bool[20]{(v1959+(if v1940{v1}else{(if v1923{(v1924*(v1921*v1933))}else{v1})}))}else{v1})})});
        let v2029=(if (v1041>v1){v2}else{v1});
        let v2031=(v64*(v1041*v2029));
        let v2032=(v2+v2031);
        let v2033=(v2031/v2032);
        let v2035=1.44;
        let v2036=((v59*v731)/v2035);
        let v2037=(v2036<v39);
        let v2038=(v2036).exp();
        let v2040=(!v2037);
        let v2049=(self.scalar_static_f64[207]*(v2+(v1071*self.scalar_static_f64[208])));
        let v2051=((if v2040{(v1513*(v2+(v2036-v39)))}else{(if v2037{v2038}else{v1703})})*self.scalar_static_f64[209]);
        let v2053=((if (v0!=0.0){self.scalar_static_f64[25]}else{v1})+(v2033*v2033));
        let v2056=(v2+(v2029*(v2051*v2053)));
        let v2057=(v2049*v2056);
        let v2060=(v1041*v2057);
        let v2081=((v740-v741)*self.scalar_static_f64[213]);
        let v2083=((v740-v745)*self.scalar_static_f64[214]);
        let v2085=(v371*self.scalar_static_f64[215]);
        let v2087=(v761*self.scalar_static_f64[216]);
        let v2090=((v762*self.scalar_static_f64[216])*0.3333333333333333);
        let v2091=(v36*((self.scalar_static_f64[180]*(v647*v842))+(v2060/v1108)));
        let v2092=(v36*(self.scalar_static_f64[183]*(v647*v1919)));
        let v2093=(v36*(((v652*v1025)+(v1058*self.scalar_static_f64[210]))+(v1529*self.scalar_static_f64[211])));
        let v2094=(v36*(v1532*self.scalar_static_f64[211]));
        let v2095=(v36*((v654*v2027)+((if self.scalar_static_bool[31]{v1}else{v1147})*self.scalar_static_f64[210])));
        let v2096=(v36*((v659*v1866)+(v756*self.scalar_static_f64[212])));
        let v2097=(if v374{v377}else{v2});
        let v2101=(if v382{(-(v385*(-v2097)))}else{v2097});
        let v2103=((v94*v2101)/v96);
        let v2104=(v2101/v70);
        let v2175=(-v2104);
        let v2176=(self.scalar_static_f64[42]*v2175);
        let v2186=((v464*(self.scalar_static_f64[33]*(v2104*(self.scalar_static_f64[40]*f64::powf(v391,self.scalar_static_f64[227])))))+(v459*(v464*(((v462*v2176)-(v461*(self.scalar_static_f64[39]*v2103)))/(v462*v462)))));
        let v2209=(self.scalar_static_f64[58]*v2103);
        let v2213=(v476*v476);
        let v2218=((v478*(self.scalar_static_f64[57]*(v2104*(self.scalar_static_f64[59]*f64::powf(v391,self.scalar_static_f64[229])))))+(v474*(v478*(((v476*(self.scalar_static_f64[61]*v2175))-(v475*v2209))/v2213))));
        let v2225=(self.scalar_static_f64[64]*v2103);
        let v2229=(v483*v483);
        let v2241=(self.scalar_static_f64[70]*v2103);
        let v2245=(v490*v490);
        let v2257=(self.scalar_static_f64[75]*v2103);
        let v2261=(v497*v497);
        let v2273=(self.scalar_static_f64[80]*v2103);
        let v2277=(v504*v504);
        let v2297=(self.scalar_static_f64[87]*v2103);
        let v2301=(v515*v515);
        let v2323=(self.scalar_static_f64[120]*v2101);
        let v2342=(v552*(((v391*v2103)-(v390*v2104))/(v391*v391)));
        let v2347=(v390*v390);
        let v2368=((v571*(v569*v2103))+(v570*(v2104/v391)));
        let v2371=((((v567*v2104)+(v391*((v566*v2342)+(v554*(((v559*(((v390*(self.scalar_static_f64[129]*v2104))-(v557*v2103))/v2347))-(v564*(((v390*(self.scalar_static_f64[130]*v2104))-(v562*v2103))/v2347)))/v565)))))-v2368)-(self.scalar_static_f64[66]*v2104));
        let v2372=(v552*v2103);
        let v2387=(v2371+((v586*v2372)+(v577*((v134*((v136*(v580*(((v390*(-v2371))-(v578*v2103))/v2347)))/(v552*v583)))/v585))));
        let v2410=((((v600*v2104)+(v391*((v599*v2342)+(v554*(((v593*(((v390*(self.scalar_static_f64[132]*v2104))-(v591*v2103))/v2347))-(v597*(((v390*(self.scalar_static_f64[133]*v2104))-(v595*v2103))/v2347)))/v598)))))-v2368)-(self.scalar_static_f64[77]*v2104));
        let v2425=(v2410+((v613*v2372)+(v577*((v134*((v136*(v607*(((v390*(-v2410))-(v605*v2103))/v2347)))/(v552*v610)))/v612))));
        let v2448=((((v627*v2104)+(v391*((v626*v2342)+(v554*(((v620*(((v390*(self.scalar_static_f64[135]*v2104))-(v618*v2103))/v2347))-(v624*(((v390*(self.scalar_static_f64[136]*v2104))-(v622*v2103))/v2347)))/v625)))))-v2368)-(self.scalar_static_f64[89]*v2104));
        let v2463=(v2448+((v640*v2372)+(v577*((v134*((v136*(v634*(((v390*(-v2448))-(v632*v2103))/v2347)))/(v552*v637)))/v639))));
        let v2466=(v588*v588);
        let v2472=(self.scalar_static_f64[137]*(((-(self.scalar_static_f64[128]*v2387))/v2466)*(self.scalar_static_f64[138]*f64::powf(v644,self.scalar_static_f64[236]))));
        let v2475=(v615*v615);
        let v2479=(((-(self.scalar_static_f64[131]*v2425))/v2475)*(self.scalar_static_f64[140]*f64::powf(v649,self.scalar_static_f64[188])));
        let v2484=(v642*v642);
        let v2503=((v664*(self.scalar_static_f64[144]*(v2104*(self.scalar_static_f64[38]*f64::powf(v391,self.scalar_static_f64[238])))))+(v662*(v664*(((v390*v2176)-(v461*v2103))/v2347))));
        let v2509=(-(self.scalar_static_f64[36]*((v544*v2101)+(v392*(self.scalar_static_f64[126]*v2101)))));
        let v2512=((v551*v2103)+(v390*(self.scalar_static_f64[35]*(self.scalar_static_f64[127]*v2101))));
        let v2516=(v671*v671);
        let v2571=(-v36);
        let v2572=(-v2387);
        let v2573=(self.scalar_static_f64[151]*v2572);
        let v2574=(if self.scalar_static_bool[18]{v2573}else{v1});
        let v2575=(if self.scalar_static_bool[18]{v36}else{v1});
        let v2576=(if self.scalar_static_bool[18]{v2571}else{v1});
        let v2583=(self.scalar_static_f64[153]*v2387);
        let v2584=(v784*(self.scalar_static_f64[157]*v2574));
        let v2587=(v784*v784);
        let v2589=((self.scalar_static_f64[157]*v2575)/v784);
        let v2590=((self.scalar_static_f64[157]*v2576)/v784);
        let v2612=(-(v36/v588));
        let v2613=(-(v2571/v588));
        let v2616=(self.scalar_static_f64[156]*f64::powf(v793,self.scalar_static_f64[240]));
        let v2631=(if v791{(((v795*v2387)+(v588*(-((-((-(v725*v2387))/v2466))*v2616))))/self.scalar_static_f64[156])}else{(if v771{((v777*v2387)/self.scalar_static_f64[156])}else{v1})});
        let v2632=(if v791{((v588*(-(v2612*v2616)))/self.scalar_static_f64[156])}else{v1});
        let v2633=(if v791{((v588*(-(v2613*v2616)))/self.scalar_static_f64[156])}else{v1});
        let v2643=(v765*v2573);
        let v2650=(if self.scalar_static_bool[19]{(v560*(v2573+(if self.scalar_static_bool[19]{((v2643+v2643)/(v552*v807))}else{v1})))}else{v1});
        let v2663=(if self.scalar_static_bool[19]{(((v814*v2572)+(v763*((-(((v588*v2650)-(v811*v2387))/v2466))*(self.scalar_static_f64[156]*f64::powf(v813,self.scalar_static_f64[240])))))/self.scalar_static_f64[156])}else{v1});
        let v2664=(if self.scalar_static_bool[19]{v2573}else{v1});
        let v2665=(if self.scalar_static_bool[19]{v36}else{v1});
        let v2666=(if self.scalar_static_bool[19]{v2571}else{v1});
        let v2667=(v818*v2664);
        let v2669=(v818*v2665);
        let v2671=(v818*v2666);
        let v2673=(v552*v821);
        let v2687=(if self.scalar_static_bool[19]{((v134*(v2664-(if self.scalar_static_bool[19]{((v2667+v2667)/v2673)}else{v1})))-v2573)}else{v1});
        let v2688=(if self.scalar_static_bool[19]{(v134*(v2665-(if self.scalar_static_bool[19]{((v2669+v2669)/v2673)}else{v1})))}else{v1});
        let v2689=(if self.scalar_static_bool[19]{(v134*(v2666-(if self.scalar_static_bool[19]{((v2671+v2671)/v2673)}else{v1})))}else{v1});
        let v2700=(self.scalar_static_f64[156]*f64::powf(v828,self.scalar_static_f64[240]));
        let v2716=(v36-v2688);
        let v2717=(v2571-v2689);
        let v2718=(v2650+(-v2687));
        let v2744=(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{(((v829*v2572)+(v763*((-(((v588*v2687)-(v826*v2387))/v2466))*v2700)))/self.scalar_static_f64[156])}else{v2631})+((v838*(self.scalar_static_f64[155]*v2718))+(v835*(((v784*(self.scalar_static_f64[157]*v2718))-(v836*v2583))/v2587))))-v2663)}else{(if self.scalar_static_bool[18]{(v2631+(if v791{v1}else{(if v771{(v775*((v786*v2574)+(v769*((v2584-(v783*v2583))/v2587))))}else{v1})}))}else{v1})});
        let v2745=(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v763*((-(v2688/v588))*v2700))/self.scalar_static_f64[156])}else{v2632})+((v838*(self.scalar_static_f64[155]*v2716))+(v835*((self.scalar_static_f64[157]*v2716)/v784))))}else{(if self.scalar_static_bool[18]{(v2632+(if v791{v1}else{(if v771{(v775*((v786*v2575)+(v769*v2589)))}else{v1})}))}else{v1})});
        let v2746=(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v763*((-(v2689/v588))*v2700))/self.scalar_static_f64[156])}else{v2633})+((v838*(self.scalar_static_f64[155]*v2717))+(v835*((self.scalar_static_f64[157]*v2717)/v784))))}else{(if self.scalar_static_bool[18]{(v2633+(if v791{v1}else{(if v771{(v775*((v786*v2576)+(v769*v2590)))}else{v1})}))}else{v1})});
        let v2747=(-v2425);
        let v2748=(self.scalar_static_f64[151]*v2747);
        let v2749=(if self.scalar_static_bool[20]{v2748}else{v1});
        let v2750=(if self.scalar_static_bool[20]{v2571}else{v1});
        let v2751=(if self.scalar_static_bool[20]{v36}else{v1});
        let v2758=(v615*(self.scalar_static_f64[164]*v2749));
        let v2762=((self.scalar_static_f64[164]*v2750)/v615);
        let v2763=((self.scalar_static_f64[164]*v2751)/v615);
        let v2781=((-(self.scalar_static_f64[165]*v2425))/v2475);
        let v2785=(v2781*(self.scalar_static_f64[163]*f64::powf(v877,self.scalar_static_f64[241])));
        let v2790=(v881*v881);
        let v2811=((v615*(-(v878*(-((self.scalar_static_f64[163]*v2571)/v881)))))/self.scalar_static_f64[163]);
        let v2812=((v615*(-(v878*(-((v36*self.scalar_static_f64[163])/v881)))))/self.scalar_static_f64[163]);
        let v2822=(-(v2571/v615));
        let v2823=(-(v36/v615));
        let v2825=(self.scalar_static_f64[163]*f64::powf(v892,self.scalar_static_f64[241]));
        let v2840=(if v890{(((v894*v2425)+(v615*(-((-((-(v731*v2425))/v2475))*v2825))))/self.scalar_static_f64[163])}else{(if v875{(((v885*v2425)+(v615*(-((v883*v2785)+(v878*(-((-(v880*v2425))/v2790)))))))/self.scalar_static_f64[163])}else{(if v850{((v856*v2425)/self.scalar_static_f64[163])}else{v1})})});
        let v2841=(if v890{((v615*(-(v2822*v2825)))/self.scalar_static_f64[163])}else{(if v875{v2811}else{v1})});
        let v2842=(if v890{((v615*(-(v2823*v2825)))/self.scalar_static_f64[163])}else{(if v875{v2812}else{v1})});
        let v2852=(-v2748);
        let v2853=(v907*v2748);
        let v2856=(v907*v907);
        let v2858=(if self.scalar_static_bool[25]{((v2853-(v906*v2852))/v2856)}else{v1});
        let v2860=(v911*v2858);
        let v2864=(v917*v2858);
        let v2880=(if self.scalar_static_bool[25]{(v134*(((v925*v2852)+(v907*(if self.scalar_static_bool[25]{(((v923*(v552*v2858))-(v910*(((v2860+v2860)/(v552*v916))+((v2864+v2864)/(v552*v922)))))/(v923*v923))}else{v1})))-v2748))}else{v1});
        let v2894=(if self.scalar_static_bool[25]{(((v934*v2425)+(v615*(-((-(((v615*v2880)-(v930*v2425))/v2475))*(self.scalar_static_f64[163]*f64::powf(v932,self.scalar_static_f64[241]))))))/self.scalar_static_f64[163])}else{v1});
        let v2902=(if self.scalar_static_bool[25]{((v2853-(v940*v2852))/v2856)}else{v1});
        let v2903=(if self.scalar_static_bool[25]{((v552*v2571)/v907)}else{v1});
        let v2904=(if self.scalar_static_bool[25]{((v36*v552)/v907)}else{v1});
        let v2906=(v552*v2903);
        let v2907=(v552*v2904);
        let v2908=(v944*v2902);
        let v2910=(v944*v2903);
        let v2912=(v944*v2904);
        let v2914=(v552*v947);
        let v2918=(v948*v2902);
        let v2920=(v948*v2903);
        let v2922=(v948*v2904);
        let v2924=(v552*v951);
        let v2934=(v952*v952);
        let v2944=(if self.scalar_static_bool[25]{(((v952*(v552*v2902))-(v943*(((v2908+v2908)/v2914)+((v2918+v2918)/v2924))))/v2934)}else{v1});
        let v2945=(if self.scalar_static_bool[25]{(((v952*v2906)-(v943*(((v2910+v2910)/v2914)+((v2920+v2920)/v2924))))/v2934)}else{v1});
        let v2946=(if self.scalar_static_bool[25]{(((v952*v2907)-(v943*(((v2912+v2912)/v2914)+((v2922+v2922)/v2924))))/v2934)}else{v1});
        let v2956=(if self.scalar_static_bool[25]{(v134*(((v954*v2852)+(v907*v2944))-v2748))}else{v1});
        let v2957=(if self.scalar_static_bool[25]{(v134*(v907*v2945))}else{v1});
        let v2958=(if self.scalar_static_bool[25]{(v134*(v907*v2946))}else{v1});
        let v2969=(self.scalar_static_f64[163]*f64::powf(v961,self.scalar_static_f64[241]));
        let v2984=(if self.scalar_static_bool[25]{(((v963*v2425)+(v615*(-((-(((v615*v2956)-(v959*v2425))/v2475))*v2969))))/self.scalar_static_f64[163])}else{v2840});
        let v2985=(if self.scalar_static_bool[25]{((v615*(-((-(v2957/v615))*v2969)))/self.scalar_static_f64[163])}else{v2841});
        let v2986=(if self.scalar_static_bool[25]{((v615*(-((-(v2958/v615))*v2969)))/self.scalar_static_f64[163])}else{v2842});
        let v2990=(if self.scalar_static_bool[25]{(v134*v2944)}else{v1});
        let v2991=(if self.scalar_static_bool[25]{(v134*v2945)}else{v1});
        let v2992=(if self.scalar_static_bool[25]{(v134*v2946)}else{v1});
        let v2997=(if self.scalar_static_bool[25]{(v2781*(self.scalar_static_f64[172]*f64::powf(v877,self.scalar_static_f64[242])))}else{v1});
        let v3005=(if self.scalar_static_bool[25]{((((v615*v2748)-(v844*v2425))/v2475)*(self.scalar_static_f64[172]*f64::powf(v974,self.scalar_static_f64[242])))}else{v1});
        let v3048=(v844*v2748);
        let v3055=(if self.scalar_static_bool[27]{(v560*(v2748+(if self.scalar_static_bool[27]{((v3048+v3048)/(v552*v993))}else{v1})))}else{v2880});
        let v3068=(if self.scalar_static_bool[27]{(((v1000*v2747)+(v843*((-(((v615*v3055)-(v997*v2425))/v2475))*(self.scalar_static_f64[163]*f64::powf(v999,self.scalar_static_f64[241])))))/self.scalar_static_f64[163])}else{v1});
        let v3069=(if self.scalar_static_bool[27]{v2748}else{v1});
        let v3070=(if self.scalar_static_bool[27]{v2571}else{v1});
        let v3071=(if self.scalar_static_bool[27]{v36}else{v1});
        let v3072=(v1004*v3069);
        let v3074=(v1004*v3070);
        let v3076=(v1004*v3071);
        let v3078=(v552*v1007);
        let v3092=(if self.scalar_static_bool[27]{((v134*(v3069-(if self.scalar_static_bool[27]{((v3072+v3072)/v3078)}else{v1})))-v2748)}else{v2956});
        let v3093=(if self.scalar_static_bool[27]{(v134*(v3070-(if self.scalar_static_bool[27]{((v3074+v3074)/v3078)}else{v1})))}else{v2957});
        let v3094=(if self.scalar_static_bool[27]{(v134*(v3071-(if self.scalar_static_bool[27]{((v3076+v3076)/v3078)}else{v1})))}else{v2958});
        let v3105=(self.scalar_static_f64[163]*f64::powf(v1014,self.scalar_static_f64[241]));
        let v3131=(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{(((v1015*v2747)+(v843*((-(((v615*v3092)-(v1012*v2425))/v2475))*v3105)))/self.scalar_static_f64[163])}else{v2984})+(self.scalar_static_f64[173]*(v3055+(-v3092))))-v3068)}else{(if self.scalar_static_bool[25]{((v2984+(if self.scalar_static_bool[25]{((v983*(if self.scalar_static_bool[25]{(((v977*v2997)+(v972*(-v2990)))+((v976*v2990)+(v969*v3005)))}else{v1}))+(v981*(v2880+(-v2956))))}else{v1}))-v2894)}else{(if self.scalar_static_bool[20]{(v2840+(if v874{v1}else{(if v850{(v853*((v864*v2749)+(v848*((v2758-(v862*v2425))/v2475))))}else{v1})}))}else{v1})})});
        let v3132=(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v843*((-(v3093/v615))*v3105))/self.scalar_static_f64[163])}else{v2985})+(self.scalar_static_f64[173]*(v2571-v3093)))}else{(if self.scalar_static_bool[25]{(v2985+(if self.scalar_static_bool[25]{((v983*(if self.scalar_static_bool[25]{((v972*(-v2991))+(v976*v2991))}else{v1}))+(v981*(v2571-v2957)))}else{v1}))}else{(if self.scalar_static_bool[20]{(v2841+(if v874{v1}else{(if v850{(v853*((v864*v2750)+(v848*v2762)))}else{v1})}))}else{v1})})});
        let v3133=(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v843*((-(v3094/v615))*v3105))/self.scalar_static_f64[163])}else{v2986})+(self.scalar_static_f64[173]*(v36-v3094)))}else{(if self.scalar_static_bool[25]{(v2986+(if self.scalar_static_bool[25]{((v983*(if self.scalar_static_bool[25]{((v972*(-v2992))+(v976*v2992))}else{v1}))+(v981*(v36-v2958)))}else{v1}))}else{(if self.scalar_static_bool[20]{(v2842+(if v874{v1}else{(if v850{(v853*((v864*v2751)+(v848*v2763)))}else{v1})}))}else{v1})})});
        let v3139=((-((v529*v2103)+(v390*(self.scalar_static_f64[39]*v2323))))/(v1026*v1026));
        let v3141=(v36*v1027);
        let v3142=(v1027*v2571);
        let v3157=(if v1032{((v1037*(v1034*(v157*v3139)))+(v1034*(v1035*v3139)))}else{(if v1028{(v1030*(v725*v3139))}else{v1})});
        let v3158=(if v1032{(v1034*v3141)}else{(if v1028{(v1030*v3141)}else{v1})});
        let v3159=(if v1032{(v1034*v3142)}else{(if v1028{(v1030*v3142)}else{v1})});
        let v3162=((v1040*v2186)+(v465*v3157));
        let v3163=(v465*v3158);
        let v3164=(v465*v3159);
        let v3170=((-((v530*v2103)+(v390*(self.scalar_static_f64[51]*v2323))))/(v1042*v1042));
        let v3172=(v1043*v2571);
        let v3173=(v36*v1043);
        let v3189=(if v1048{((v1053*(v1050*(v194*v3170)))+(v1050*(v1051*v3170)))}else{(if v1044{(v1046*(v731*v3170))}else{v3157})});
        let v3190=(if v1048{(v1050*v3172)}else{(if v1044{(v1046*v3172)}else{v1})});
        let v3191=(if v1048{(v1050*v3173)}else{(if v1044{(v1046*v3173)}else{v3158})});
        let v3192=(if v1048{v1}else{(if v1044{v1}else{v3159})});
        let v3198=((v1057*((v472*v2186)+(v465*((v471*(self.scalar_static_f64[49]*(v2104*(self.scalar_static_f64[52]*f64::powf(v391,self.scalar_static_f64[228])))))+(v467*(v471*(((v469*(self.scalar_static_f64[54]*v2175))-(v468*(self.scalar_static_f64[51]*v2103)))/(v469*v469))))))))+(v1056*v3189));
        let v3199=(v1056*v3190);
        let v3200=(v1056*v3191);
        let v3201=(v1056*v3192);
        let v3206=(v715*v2746);
        let v3210=(v712*v3132);
        let v3212=(((v842*(if v713{((-(self.scalar_static_f64[149]*(self.scalar_static_f64[150]*v2101)))/(v683*v683))}else{v1}))+(v715*v2744))+((v1025*(if v710{((-(self.scalar_static_f64[147]*(self.scalar_static_f64[148]*v2101)))/(v678*v678))}else{v1}))+(v712*v3131)));
        let v3213=((v715*v2745)+(v712*v3133));
        let v3214=(v1064*v3212);
        let v3216=(v1064*v3210);
        let v3218=(v1064*v3213);
        let v3220=(v1064*v3206);
        let v3222=(v552*v1068);
        let v3231=(v134*(v3212+((v3214+v3214)/v3222)));
        let v3232=(v134*(v3210+((v3216+v3216)/v3222)));
        let v3233=(v134*(v3213+((v3218+v3218)/v3222)));
        let v3234=(v134*(v3206+((v3220+v3220)/v3222)));
        let v3249=(self.scalar_static_f64[175]*f64::powf(v1071,self.scalar_static_f64[243]));
        let v3254=(v136*(((v1041*(if v716{((-(self.scalar_static_f64[43]*(v2104*(self.scalar_static_f64[96]*f64::powf(v391,self.scalar_static_f64[217])))))/(v395*v395))}else{v1}))+(v718*v3162))+(v44*v3198)));
        let v3255=(v136*(v44*v3199));
        let v3256=(v136*((v718*v3163)+(v44*v3200)));
        let v3257=(v136*((v718*v3164)+(v44*v3201)));
        let v3262=(if self.scalar_static_bool[28]{((v3231*v3249)+v3254)}else{v1});
        let v3263=(if self.scalar_static_bool[28]{((v3232*v3249)+v3255)}else{v1});
        let v3264=(if self.scalar_static_bool[28]{((v3233*v3249)+v3256)}else{v1});
        let v3265=(if self.scalar_static_bool[28]{((v3234*v3249)+v3257)}else{v1});
        let v3268=(self.scalar_static_f64[45]*f64::powf(v1081,self.scalar_static_f64[244]));
        let v3285=(v134*v3231);
        let v3286=(v134*v3232);
        let v3287=(v134*v3233);
        let v3288=(v134*v3234);
        let v3293=(if self.scalar_static_bool[29]{v3254}else{v3262});
        let v3294=(if self.scalar_static_bool[29]{v3255}else{v3263});
        let v3295=(if self.scalar_static_bool[29]{v3256}else{v3264});
        let v3296=(if self.scalar_static_bool[29]{v3257}else{v3265});
        let v3298=(self.scalar_static_f64[45]*f64::powf(v1096,self.scalar_static_f64[244]));
        let v3323=(if v1105{(self.scalar_static_f64[177]*v3285)}else{(if v1098{((v1101*v3285)+(v1099*(v3293*v3298)))}else{(if v1089{v3285}else{(if v1083{(v134*(v3231+(v3262*v3268)))}else{v1})})})});
        let v3324=(if v1105{(self.scalar_static_f64[177]*v3286)}else{(if v1098{((v1101*v3286)+(v1099*(v3294*v3298)))}else{(if v1089{v3286}else{(if v1083{(v134*(v3232+(v3263*v3268)))}else{v1})})})});
        let v3325=(if v1105{(self.scalar_static_f64[177]*v3287)}else{(if v1098{((v1101*v3287)+(v1099*(v3295*v3298)))}else{(if v1089{v3287}else{(if v1083{(v134*(v3233+(v3264*v3268)))}else{v1})})})});
        let v3326=(if v1105{(self.scalar_static_f64[177]*v3288)}else{(if v1098{((v1101*v3288)+(v1099*(v3296*v3298)))}else{(if v1089{v3288}else{(if v1083{(v134*(v3234+(v3265*v3268)))}else{v1})})})});
        let v3330=(v1108*v1108);
        let v3361=(if self.scalar_static_bool[30]{((-v2209)/v2213)}else{v3170});
        let v3363=(v36*v1113);
        let v3364=(v1113*v2571);
        let v3375=(v1122*(v226*v3361));
        let v3380=(v1122*v3363);
        let v3381=(v1122*v3364);
        let v3382=(if v1120{((v1125*v3375)+(v1122*(v1123*v3361)))}else{(if v1115{(v1117*(v739*v3361))}else{v3189})});
        let v3383=(if v1120{v1}else{(if v1115{v1}else{v3190})});
        let v3384=(if v1120{v3380}else{(if v1115{(v1117*v3363)}else{v1})});
        let v3385=(if v1120{v1}else{(if v1115{v1}else{v3191})});
        let v3386=(if v1120{v1}else{(if v1115{v1}else{v3192})});
        let v3387=(if v1120{v3381}else{(if v1115{(v1117*v3364)}else{v1})});
        let v3399=(if v1134{((v1137*v3375)+(v1122*(v1135*v3361)))}else{(if v1129{(v1131*(v731*v3361))}else{v1})});
        let v3400=(if v1134{v3381}else{(if v1129{(v1131*v3364)}else{v1})});
        let v3401=(if v1134{v3380}else{(if v1129{(v1131*v3363)}else{v1})});
        let v3422=(if self.scalar_static_bool[30]{((v1145*v2218)+(v479*((self.scalar_static_f64[178]*v3382)+(self.scalar_static_f64[179]*v3399))))}else{v1});
        let v3423=(if self.scalar_static_bool[30]{(v479*((self.scalar_static_f64[178]*v3383)+(self.scalar_static_f64[179]*v3400)))}else{v1});
        let v3424=(if self.scalar_static_bool[30]{(v479*(self.scalar_static_f64[178]*v3384))}else{v1});
        let v3425=(if self.scalar_static_bool[30]{(v479*((self.scalar_static_f64[178]*v3385)+(self.scalar_static_f64[179]*v3401)))}else{v1});
        let v3426=(if self.scalar_static_bool[30]{(v479*(self.scalar_static_f64[178]*v3386))}else{v1});
        let v3427=(if self.scalar_static_bool[30]{(v479*(self.scalar_static_f64[178]*v3387))}else{v1});
        let v3492=(if v1169{((v1172*v3375)+(v1122*(v1170*v3361)))}else{(if v1164{(v1166*(v756*v3361))}else{v3382})});
        let v3493=(if v1169{v1}else{(if v1164{v1}else{v3383})});
        let v3494=(if v1169{v1}else{(if v1164{v1}else{v3384})});
        let v3495=(if v1169{v1}else{(if v1164{v1}else{v3385})});
        let v3496=(if v1169{v1}else{(if v1164{v1}else{v3386})});
        let v3497=(if v1169{v3381}else{(if v1164{(v1166*v3364)}else{v3387})});
        let v3498=(if v1169{v3380}else{(if v1164{(v1166*v3363)}else{v1})});
        let v3575=((-v2225)/v2229);
        let v3576=(if self.scalar_static_bool[32]{v3575}else{v3361});
        let v3578=(v36*v1188);
        let v3579=(v1188*v2571);
        let v3598=(if v1195{((v1200*(v1197*(v247*v3576)))+(v1197*(v1198*v3576)))}else{(if v1190{(v1192*(v725*v3576))}else{v3492})});
        let v3599=(if v1195{v1}else{(if v1190{v1}else{v3493})});
        let v3600=(if v1195{v1}else{(if v1190{v1}else{v3494})});
        let v3601=(if v1195{(v1197*v3578)}else{(if v1190{(v1192*v3578)}else{v3495})});
        let v3602=(if v1195{(v1197*v3579)}else{(if v1190{(v1192*v3579)}else{v3496})});
        let v3603=(if v1195{v1}else{(if v1190{v1}else{v3497})});
        let v3604=(if v1195{v1}else{(if v1190{v1}else{v3498})});
        let v3606=((-v2241)/v2245);
        let v3607=(if self.scalar_static_bool[32]{v3606}else{v3576});
        let v3687=(if self.scalar_static_bool[38]{v2509}else{v1});
        let v3688=(if self.scalar_static_bool[38]{v2571}else{v1});
        let v3689=(if self.scalar_static_bool[38]{v36}else{v1});
        let v3691=((-v2512)/v2516);
        let v3692=(if self.scalar_static_bool[38]{v3691}else{v3607});
        let v3693=(v1242*v3687);
        let v3696=(v1242*v3688);
        let v3697=(v1242*v3689);
        let v3714=(if v1249{((v1254*(v1251*(v114*v3692)))+(v1251*(v3693+(v1252*v3692))))}else{(if v1244{(v1246*(v3693+(v1240*v3692)))}else{v3399})});
        let v3715=(if v1249{v1}else{(if v1244{v1}else{v3400})});
        let v3716=(if v1249{(v1251*v3696)}else{(if v1244{(v1246*v3696)}else{v3401})});
        let v3717=(if v1249{(v1251*v3697)}else{(if v1244{(v1246*v3697)}else{v1})});
        let v3738=(if self.scalar_static_bool[41]{v3575}else{v3692});
        let v3740=(v36*v1265);
        let v3741=(v1265*v2571);
        let v3760=(if v1272{((v1277*(v1274*(v247*v3738)))+(v1274*(v1275*v3738)))}else{(if v1267{(v1269*(v728*v3738))}else{v3598})});
        let v3761=(if v1272{v1}else{(if v1267{v1}else{v3599})});
        let v3762=(if v1272{(v1274*v3740)}else{(if v1267{(v1269*v3740)}else{v3600})});
        let v3763=(if v1272{v1}else{(if v1267{v1}else{v3601})});
        let v3764=(if v1272{(v1274*v3741)}else{(if v1267{(v1269*v3741)}else{v3602})});
        let v3765=(if v1272{v1}else{(if v1267{v1}else{v3603})});
        let v3766=(if v1272{v1}else{(if v1267{v1}else{v3604})});
        let v3767=(if self.scalar_static_bool[41]{v3606}else{v3738});
        let v3816=(if self.scalar_static_bool[42]{v2509}else{v3687});
        let v3817=(if self.scalar_static_bool[42]{v2571}else{v3688});
        let v3818=(if self.scalar_static_bool[42]{v36}else{v3689});
        let v3819=(if self.scalar_static_bool[42]{v3691}else{v3767});
        let v3820=(v1303*v3816);
        let v3823=(v1303*v3817);
        let v3824=(v1303*v3818);
        let v3841=(if v1310{((v1315*(v1312*(v114*v3819)))+(v1312*(v3820+(v1313*v3819))))}else{(if v1305{(v1307*(v3820+(v1302*v3819)))}else{v3714})});
        let v3842=(if v1310{v1}else{(if v1305{v1}else{v3715})});
        let v3843=(if v1310{(v1312*v3823)}else{(if v1305{(v1307*v3823)}else{v3716})});
        let v3844=(if v1310{(v1312*v3824)}else{(if v1305{(v1307*v3824)}else{v3717})});
        let v3858=(if self.scalar_static_bool[44]{v3575}else{v3819});
        let v3860=(v36*v1324);
        let v3861=(v1324*v2571);
        let v3880=(if v1329{((v1333*(v1331*(v247*v3858)))+(v1331*(v1198*v3858)))}else{(if v1325{(v1327*(v725*v3858))}else{v3760})});
        let v3881=(if v1329{v1}else{(if v1325{v1}else{v3761})});
        let v3882=(if v1329{v1}else{(if v1325{v1}else{v3762})});
        let v3883=(if v1329{(v1331*v3860)}else{(if v1325{(v1327*v3860)}else{v3763})});
        let v3884=(if v1329{(v1331*v3861)}else{(if v1325{(v1327*v3861)}else{v3764})});
        let v3885=(if v1329{v1}else{(if v1325{v1}else{v3765})});
        let v3886=(if v1329{v1}else{(if v1325{v1}else{v3766})});
        let v3887=(if self.scalar_static_bool[44]{v3606}else{v3858});
        let v3976=(if self.scalar_static_bool[47]{v2509}else{v3816});
        let v3977=(if self.scalar_static_bool[47]{v2571}else{v3817});
        let v3978=(if self.scalar_static_bool[47]{v36}else{v3818});
        let v3979=(if self.scalar_static_bool[47]{v3691}else{v3887});
        let v3980=(v1363*v3976);
        let v3983=(v1363*v3977);
        let v3984=(v1363*v3978);
        let v4001=(if v1370{((v1375*(v1372*(v114*v3979)))+(v1372*(v3980+(v1373*v3979))))}else{(if v1365{(v1367*(v3980+(v1362*v3979)))}else{v3841})});
        let v4002=(if v1370{v1}else{(if v1365{v1}else{v3842})});
        let v4003=(if v1370{(v1372*v3983)}else{(if v1365{(v1367*v3983)}else{v3843})});
        let v4004=(if v1370{(v1372*v3984)}else{(if v1365{(v1367*v3984)}else{v3844})});
        let v4018=(if self.scalar_static_bool[44]{v3575}else{v3979});
        let v4020=(v36*v1383);
        let v4021=(v1383*v2571);
        let v4040=(if v1388{((v1392*(v1390*(v247*v4018)))+(v1390*(v1275*v4018)))}else{(if v1384{(v1386*(v728*v4018))}else{v3880})});
        let v4041=(if v1388{v1}else{(if v1384{v1}else{v3881})});
        let v4042=(if v1388{(v1390*v4020)}else{(if v1384{(v1386*v4020)}else{v3882})});
        let v4043=(if v1388{v1}else{(if v1384{v1}else{v3883})});
        let v4044=(if v1388{(v1390*v4021)}else{(if v1384{(v1386*v4021)}else{v3884})});
        let v4045=(if v1388{v1}else{(if v1384{v1}else{v3885})});
        let v4046=(if v1388{v1}else{(if v1384{v1}else{v3886})});
        let v4047=(if self.scalar_static_bool[44]{v3606}else{v4018});
        let v4106=(if self.scalar_static_bool[47]{v3691}else{v4047});
        let v4107=(v1416*(if self.scalar_static_bool[47]{v2509}else{v3976}));
        let v4110=(v1416*(if self.scalar_static_bool[47]{v2571}else{v3977}));
        let v4111=(v1416*(if self.scalar_static_bool[47]{v36}else{v3978}));
        let v4128=(if v1423{((v1428*(v1425*(v114*v4106)))+(v1425*(v4107+(v1426*v4106))))}else{(if v1418{(v1420*(v4107+(v1415*v4106)))}else{v4001})});
        let v4129=(if v1423{v1}else{(if v1418{v1}else{v4002})});
        let v4130=(if v1423{(v1425*v4110)}else{(if v1418{(v1420*v4110)}else{v4003})});
        let v4131=(if v1423{(v1425*v4111)}else{(if v1418{(v1420*v4111)}else{v4004})});
        let v4146=((-v2257)/v2261);
        let v4148=(v1436*v2571);
        let v4149=(v36*v1436);
        let v4168=(if v1441{((v1446*(v1443*(v288*v4146)))+(v1443*(v1444*v4146)))}else{(if v1437{(v1439*(v731*v4146))}else{v4040})});
        let v4169=(if v1441{(v1443*v4148)}else{(if v1437{(v1439*v4148)}else{v4041})});
        let v4170=(if v1441{v1}else{(if v1437{v1}else{v4042})});
        let v4171=(if v1441{(v1443*v4149)}else{(if v1437{(v1439*v4149)}else{v4043})});
        let v4172=(if v1441{v1}else{(if v1437{v1}else{v4044})});
        let v4173=(if v1441{v1}else{(if v1437{v1}else{v4045})});
        let v4174=(if v1441{v1}else{(if v1437{v1}else{v4046})});
        let v4176=((-v2273)/v2277);
        let v4222=(if self.scalar_static_bool[50]{v4146}else{v4176});
        let v4224=(v36*v1470);
        let v4225=(v1470*v2571);
        let v4244=(if v1477{((v1482*(v1479*(v319*v4222)))+(v1479*(v1480*v4222)))}else{(if v1472{(v1474*(v739*v4222))}else{v4168})});
        let v4245=(if v1477{v1}else{(if v1472{v1}else{v4169})});
        let v4246=(if v1477{(v1479*v4224)}else{(if v1472{(v1474*v4224)}else{v4170})});
        let v4247=(if v1477{v1}else{(if v1472{v1}else{v4171})});
        let v4248=(if v1477{v1}else{(if v1472{v1}else{v4172})});
        let v4249=(if v1477{(v1479*v4225)}else{(if v1472{(v1474*v4225)}else{v4173})});
        let v4250=(if v1477{v1}else{(if v1472{v1}else{v4174})});
        let v4251=(if self.scalar_static_bool[50]{v4176}else{v4222});
        let v4317=((-(v731*v2103))/v2347);
        let v4318=(v2571/v390);
        let v4319=(v36/v390);
        let v4331=(v1513*v4318);
        let v4332=(v1513*v4319);
        let v4333=(if v1512{(v1513*v4317)}else{(if v1509{(v1510*v4317)}else{v4244})});
        let v4334=(if v1512{v4331}else{(if v1509{(v1510*v4318)}else{v4245})});
        let v4335=(if v1512{v1}else{(if v1509{v1}else{v4246})});
        let v4336=(if v1512{v4332}else{(if v1509{(v1510*v4319)}else{v4247})});
        let v4337=(if v1512{v1}else{(if v1509{v1}else{v4248})});
        let v4338=(if v1512{v1}else{(if v1509{v1}else{v4249})});
        let v4339=(if v1512{v1}else{(if v1509{v1}else{v4250})});
        let v4342=((-(v734*v2103))/v2347);
        let v4366=(v552*v1529);
        let v4367=(((v1517*v2503)+(v665*v4333))/v4366);
        let v4368=((v665*v4334)/v4366);
        let v4369=((v665*v4335)/v4366);
        let v4370=((v665*v4336)/v4366);
        let v4371=((v665*v4337)/v4366);
        let v4372=((v665*v4338)/v4366);
        let v4373=((v665*v4339)/v4366);
        let v4381=(v552*v1532);
        let v4382=(((v1526*v2503)+(v665*(if v1522{(v1513*v4342)}else{(if v1519{(v1520*v4342)}else{v4128})})))/v4381);
        let v4383=((v665*(if v1522{v4331}else{(if v1519{(v1520*v4318)}else{v1})}))/v4381);
        let v4384=((v665*(if v1522{v1}else{(if v1519{v1}else{v4129})}))/v4381);
        let v4385=((v665*(if v1522{v4332}else{(if v1519{(v1520*v4319)}else{v4130})}))/v4381);
        let v4386=((v665*(if v1522{v1}else{(if v1519{v1}else{v4131})}))/v4381);
        let v4866=(if self.scalar_static_bool[66]{((-v2297)/v2301)}else{v4251});
        let v4868=(v1689*v2571);
        let v4869=(v36*v1689);
        let v4888=(if v1696{((v1701*(v1698*(v350*v4866)))+(v1698*(v1699*v4866)))}else{(if v1691{(v1693*(v756*v4866))}else{v4333})});
        let v4889=(if v1696{v1}else{(if v1691{v1}else{v4334})});
        let v4890=(if v1696{v1}else{(if v1691{v1}else{v4335})});
        let v4891=(if v1696{v1}else{(if v1691{v1}else{v4336})});
        let v4892=(if v1696{v1}else{(if v1691{v1}else{v4337})});
        let v4893=(if v1696{(v1698*v4868)}else{(if v1691{(v1693*v4868)}else{v4338})});
        let v4894=(if v1696{(v1698*v4869)}else{(if v1691{(v1693*v4869)}else{v4339})});
        let v5248=(-v2463);
        let v5250=(if self.scalar_static_bool[68]{(self.scalar_static_f64[151]*v5248)}else{v1});
        let v5251=(if self.scalar_static_bool[70]{v5250}else{v1});
        let v5252=(if self.scalar_static_bool[70]{v2571}else{v1});
        let v5253=(if self.scalar_static_bool[70]{v36}else{v1});
        let v5260=(self.scalar_static_f64[153]*v2463);
        let v5264=(v1805*v1805);
        let v5293=(self.scalar_static_f64[203]*f64::powf(v1814,self.scalar_static_f64[250]));
        let v5308=(if v1812{(((v1816*v2463)+(v642*(-((-((-(v756*v2463))/v2484))*v5293))))/self.scalar_static_f64[203])}else{(if v1793{((v1798*v2463)/self.scalar_static_f64[203])}else{v1})});
        let v5309=(if v1812{((v642*(-((-(v2571/v642))*v5293)))/self.scalar_static_f64[203])}else{v1});
        let v5310=(if v1812{((v642*(-((-(v36/v642))*v5293)))/self.scalar_static_f64[203])}else{v1});
        let v5320=(v1786*v5250);
        let v5327=(if self.scalar_static_bool[72]{(v560*(v5250+(if self.scalar_static_bool[72]{((v5320+v5320)/(v552*v1829))}else{v1})))}else{v1});
        let v5341=(if self.scalar_static_bool[72]{v5250}else{v1});
        let v5342=(if self.scalar_static_bool[72]{v2571}else{v1});
        let v5343=(if self.scalar_static_bool[72]{v36}else{v1});
        let v5344=(v1840*v5341);
        let v5346=(v1840*v5342);
        let v5348=(v1840*v5343);
        let v5350=(v552*v1843);
        let v5364=(if self.scalar_static_bool[72]{((v134*(v5341-(if self.scalar_static_bool[72]{((v5344+v5344)/v5350)}else{v1})))-v5250)}else{v1});
        let v5365=(if self.scalar_static_bool[72]{(v134*(v5342-(if self.scalar_static_bool[72]{((v5346+v5346)/v5350)}else{v1})))}else{v1});
        let v5366=(if self.scalar_static_bool[72]{(v134*(v5343-(if self.scalar_static_bool[72]{((v5348+v5348)/v5350)}else{v1})))}else{v1});
        let v5377=(self.scalar_static_f64[203]*f64::powf(v1850,self.scalar_static_f64[250]));
        let v5393=(v2571-v5365);
        let v5394=(v36-v5366);
        let v5395=(v5327+(-v5364));
        let v5453=(self.scalar_static_f64[156]*f64::powf(v1886,self.scalar_static_f64[240]));
        let v5468=(if v1884{(((v1888*v2387)+(v588*(-((-((-(v728*v2387))/v2466))*v5453))))/self.scalar_static_f64[156])}else{(if v1870{((v1873*v2387)/self.scalar_static_f64[156])}else{v1})});
        let v5469=(if v1884{((v588*(-(v2612*v5453)))/self.scalar_static_f64[156])}else{v1});
        let v5470=(if v1884{((v588*(-(v2613*v5453)))/self.scalar_static_f64[156])}else{v1});
        let v5480=(v1895*v2664);
        let v5482=(v1895*v2665);
        let v5484=(v1895*v2666);
        let v5486=(v552*v1898);
        let v5500=(if self.scalar_static_bool[19]{((v134*(v2664-(if self.scalar_static_bool[19]{((v5480+v5480)/v5486)}else{v1})))-v2573)}else{v1});
        let v5501=(if self.scalar_static_bool[19]{(v134*(v2665-(if self.scalar_static_bool[19]{((v5482+v5482)/v5486)}else{v1})))}else{v1});
        let v5502=(if self.scalar_static_bool[19]{(v134*(v2666-(if self.scalar_static_bool[19]{((v5484+v5484)/v5486)}else{v1})))}else{v1});
        let v5513=(self.scalar_static_f64[156]*f64::powf(v1905,self.scalar_static_f64[240]));
        let v5529=(v36-v5501);
        let v5530=(v2571-v5502);
        let v5531=(v2650+(-v5500));
        let v5601=(self.scalar_static_f64[163]*f64::powf(v1954,self.scalar_static_f64[241]));
        let v5616=(if v1952{(((v1956*v2425)+(v615*(-((-((-(v739*v2425))/v2475))*v5601))))/self.scalar_static_f64[163])}else{(if v1941{(((v1947*v2425)+(v615*(-((v1945*v2785)+(v878*(-((-(v1943*v2425))/v2790)))))))/self.scalar_static_f64[163])}else{(if v1923{((v1927*v2425)/self.scalar_static_f64[163])}else{v1})})});
        let v5617=(if v1952{((v615*(-(v2823*v5601)))/self.scalar_static_f64[163])}else{(if v1941{v2812}else{v1})});
        let v5618=(if v1952{((v615*(-(v2822*v5601)))/self.scalar_static_f64[163])}else{(if v1941{v2811}else{v1})});
        let v5631=(if self.scalar_static_bool[25]{((v2853-(v1965*v2852))/v2856)}else{v1});
        let v5633=(v1969*v5631);
        let v5635=(v1969*v2904);
        let v5637=(v1969*v2903);
        let v5639=(v552*v1972);
        let v5643=(v1973*v5631);
        let v5645=(v1973*v2904);
        let v5647=(v1973*v2903);
        let v5649=(v552*v1976);
        let v5659=(v1977*v1977);
        let v5669=(if self.scalar_static_bool[25]{(((v1977*(v552*v5631))-(v1968*(((v5633+v5633)/v5639)+((v5643+v5643)/v5649))))/v5659)}else{v1});
        let v5670=(if self.scalar_static_bool[25]{(((v1977*v2907)-(v1968*(((v5635+v5635)/v5639)+((v5645+v5645)/v5649))))/v5659)}else{v1});
        let v5671=(if self.scalar_static_bool[25]{(((v1977*v2906)-(v1968*(((v5637+v5637)/v5639)+((v5647+v5647)/v5649))))/v5659)}else{v1});
        let v5681=(if self.scalar_static_bool[25]{(v134*(((v1979*v2852)+(v907*v5669))-v2748))}else{v1});
        let v5682=(if self.scalar_static_bool[25]{(v134*(v907*v5670))}else{v1});
        let v5683=(if self.scalar_static_bool[25]{(v134*(v907*v5671))}else{v1});
        let v5694=(self.scalar_static_f64[163]*f64::powf(v1986,self.scalar_static_f64[241]));
        let v5709=(if self.scalar_static_bool[25]{(((v1988*v2425)+(v615*(-((-(((v615*v5681)-(v1984*v2425))/v2475))*v5694))))/self.scalar_static_f64[163])}else{v5616});
        let v5710=(if self.scalar_static_bool[25]{((v615*(-((-(v5682/v615))*v5694)))/self.scalar_static_f64[163])}else{v5617});
        let v5711=(if self.scalar_static_bool[25]{((v615*(-((-(v5683/v615))*v5694)))/self.scalar_static_f64[163])}else{v5618});
        let v5715=(if self.scalar_static_bool[25]{(v134*v5669)}else{v1});
        let v5716=(if self.scalar_static_bool[25]{(v134*v5670)}else{v1});
        let v5717=(if self.scalar_static_bool[25]{(v134*v5671)}else{v1});
        let v5760=(v2007*v3069);
        let v5762=(v2007*v3071);
        let v5764=(v2007*v3070);
        let v5766=(v552*v2010);
        let v5780=(if self.scalar_static_bool[27]{((v134*(v3069-(if self.scalar_static_bool[27]{((v5760+v5760)/v5766)}else{v1})))-v2748)}else{v5681});
        let v5781=(if self.scalar_static_bool[27]{(v134*(v3071-(if self.scalar_static_bool[27]{((v5762+v5762)/v5766)}else{v1})))}else{v5682});
        let v5782=(if self.scalar_static_bool[27]{(v134*(v3070-(if self.scalar_static_bool[27]{((v5764+v5764)/v5766)}else{v1})))}else{v5683});
        let v5793=(self.scalar_static_f64[163]*f64::powf(v2017,self.scalar_static_f64[241]));
        let v5825=(v64*(v2029*v3162));
        let v5826=(v64*(v2029*v3163));
        let v5827=(v64*(v2029*v3164));
        let v5831=(v2032*v2032);
        let v5843=((v59*v2571)/v2035);
        let v5844=((v36*v59)/v2035);
        let v5878=(v2033*(((v2032*v5825)-(v2031*v5825))/v5831));
        let v5880=(v2033*(((v2032*v5826)-(v2031*v5826))/v5831));
        let v5882=(v2033*(((v2032*v5827)-(v2031*v5827))/v5831));
        let v6024=(v36*((self.scalar_static_f64[180]*((v842*v2472)+(v647*v2744)))+(((v1108*((v2057*v3162)+(v1041*((v2056*(self.scalar_static_f64[207]*(self.scalar_static_f64[208]*v3231)))+(v2049*(v2029*((v2053*(self.scalar_static_f64[209]*(if v2040{v1}else{(if v2037{v1}else{v4888})})))+(v2051*(v5878+v5878)))))))))-(v2060*v3323))/v3330)));
        let v6025=(v36*(((v1108*(v1041*((v2056*(self.scalar_static_f64[207]*(self.scalar_static_f64[208]*v3232)))+(v2049*(v2029*(v2053*(self.scalar_static_f64[209]*(if v2040{(v1513*v5843)}else{(if v2037{(v2038*v5843)}else{v4889})}))))))))-(v2060*v3324))/v3330));
        let v6026=(v36*((v1041*(v2049*(v2029*(v2053*(self.scalar_static_f64[209]*(if v2040{v1}else{(if v2037{v1}else{v4890})}))))))/v1108));
        let v6027=(v36*((self.scalar_static_f64[180]*(v647*v2745))+(((v1108*((v2057*v3163)+(v1041*((v2056*(self.scalar_static_f64[207]*(self.scalar_static_f64[208]*v3233)))+(v2049*(v2029*((v2053*(self.scalar_static_f64[209]*(if v2040{(v1513*v5844)}else{(if v2037{(v2038*v5844)}else{v4891})})))+(v2051*(v5880+v5880)))))))))-(v2060*v3325))/v3330)));
        let v6028=(v36*((self.scalar_static_f64[180]*(v647*v2746))+(((v1108*((v2057*v3164)+(v1041*((v2056*(self.scalar_static_f64[207]*(self.scalar_static_f64[208]*v3234)))+(v2049*(v2029*((v2053*(self.scalar_static_f64[209]*(if v2040{v1}else{(if v2037{v1}else{v4892})})))+(v2051*(v5882+v5882)))))))))-(v2060*v3326))/v3330)));
        let v6029=(v36*((v1041*(v2049*(v2029*(v2053*(self.scalar_static_f64[209]*(if v2040{v1}else{(if v2037{v1}else{v4893})}))))))/v1108));
        let v6030=(v36*((v1041*(v2049*(v2029*(v2053*(self.scalar_static_f64[209]*(if v2040{v1}else{(if v2037{v1}else{v4894})}))))))/v1108));
        let v6031=(v36*(self.scalar_static_f64[183]*((v1919*v2472)+(v647*(if self.scalar_static_bool[19]{(((if self.scalar_static_bool[19]{(((v1906*v2572)+(v763*((-(((v588*v5500)-(v1903*v2387))/v2466))*v5513)))/self.scalar_static_f64[156])}else{v5468})+((v1915*(self.scalar_static_f64[155]*v5531))+(v1912*(((v784*(self.scalar_static_f64[157]*v5531))-(v1913*v2583))/v2587))))-v2663)}else{(if self.scalar_static_bool[18]{(v5468+(if v1884{v1}else{(if v1870{(v1871*((v1879*v2574)+(v1868*((v2584-(v1877*v2583))/v2587))))}else{v1})}))}else{v1})})))));
        let v6032=(v36*(self.scalar_static_f64[183]*(v647*(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v763*((-(v5501/v588))*v5513))/self.scalar_static_f64[156])}else{v5469})+((v1915*(self.scalar_static_f64[155]*v5529))+(v1912*((self.scalar_static_f64[157]*v5529)/v784))))}else{(if self.scalar_static_bool[18]{(v5469+(if v1884{v1}else{(if v1870{(v1871*((v1879*v2575)+(v1868*v2589)))}else{v1})}))}else{v1})}))));
        let v6033=(v36*(self.scalar_static_f64[183]*(v647*(if self.scalar_static_bool[19]{((if self.scalar_static_bool[19]{((v763*((-(v5502/v588))*v5513))/self.scalar_static_f64[156])}else{v5470})+((v1915*(self.scalar_static_f64[155]*v5530))+(v1912*((self.scalar_static_f64[157]*v5530)/v784))))}else{(if self.scalar_static_bool[18]{(v5470+(if v1884{v1}else{(if v1870{(v1871*((v1879*v2576)+(v1868*v2590)))}else{v1})}))}else{v1})}))));
        let v6034=(v36*((((v1025*(self.scalar_static_f64[139]*v2479))+(v652*v3131))+(self.scalar_static_f64[210]*v3198))+(self.scalar_static_f64[211]*v4367)));
        let v6035=(v36*(((v652*v3132)+(self.scalar_static_f64[210]*v3199))+(self.scalar_static_f64[211]*v4368)));
        let v6036=(v36*(self.scalar_static_f64[211]*v4369));
        let v6037=(v36*(((v652*v3133)+(self.scalar_static_f64[210]*v3200))+(self.scalar_static_f64[211]*v4370)));
        let v6038=(v36*((self.scalar_static_f64[210]*v3201)+(self.scalar_static_f64[211]*v4371)));
        let v6039=(v36*(self.scalar_static_f64[211]*v4372));
        let v6040=(v36*(self.scalar_static_f64[211]*v4373));
        let v6041=(v36*(self.scalar_static_f64[211]*v4382));
        let v6042=(v36*(self.scalar_static_f64[211]*v4383));
        let v6043=(v36*(self.scalar_static_f64[211]*v4384));
        let v6044=(v36*(self.scalar_static_f64[211]*v4385));
        let v6045=(v36*(self.scalar_static_f64[211]*v4386));
        let v6046=(v36*(((v2027*(self.scalar_static_f64[141]*v2479))+(v654*(if self.scalar_static_bool[27]{(((if self.scalar_static_bool[27]{(((v2018*v2747)+(v843*((-(((v615*v5780)-(v2015*v2425))/v2475))*v5793)))/self.scalar_static_f64[163])}else{v5709})+(self.scalar_static_f64[173]*(v3055+(-v5780))))-v3068)}else{(if self.scalar_static_bool[25]{((v5709+(if self.scalar_static_bool[25]{((v2001*(if self.scalar_static_bool[25]{(((v1995*v2997)+(v972*(-v5715)))+((v1994*v3005)+(v976*v5715)))}else{v1}))+(v1999*(v2880+(-v5681))))}else{v1}))-v2894)}else{(if self.scalar_static_bool[20]{(v5616+(if v1940{v1}else{(if v1923{(v1924*((v1933*v2749)+(v1921*((v2758-(v1931*v2425))/v2475))))}else{v1})}))}else{v1})})})))+(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3422}))));
        let v6047=(v36*(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3423})));
        let v6048=(v36*((v654*(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v843*((-(v5781/v615))*v5793))/self.scalar_static_f64[163])}else{v5710})+(self.scalar_static_f64[173]*(v36-v5781)))}else{(if self.scalar_static_bool[25]{(v5710+(if self.scalar_static_bool[25]{((v2001*(if self.scalar_static_bool[25]{((v972*(-v5716))+(v976*v5716))}else{v1}))+(v1999*(v36-v5682)))}else{v1}))}else{(if self.scalar_static_bool[20]{(v5617+(if v1940{v1}else{(if v1923{(v1924*((v1933*v2751)+(v1921*v2763)))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3424}))));
        let v6049=(v36*(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3425})));
        let v6050=(v36*(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3426})));
        let v6051=(v36*((v654*(if self.scalar_static_bool[27]{((if self.scalar_static_bool[27]{((v843*((-(v5782/v615))*v5793))/self.scalar_static_f64[163])}else{v5711})+(self.scalar_static_f64[173]*(v2571-v5782)))}else{(if self.scalar_static_bool[25]{(v5711+(if self.scalar_static_bool[25]{((v2001*(if self.scalar_static_bool[25]{((v972*(-v5717))+(v976*v5717))}else{v1}))+(v1999*(v2571-v5683)))}else{v1}))}else{(if self.scalar_static_bool[20]{(v5618+(if v1940{v1}else{(if v1923{(v1924*((v1933*v2750)+(v1921*v2762)))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[210]*(if self.scalar_static_bool[31]{v1}else{v3427}))));
        let v6052=(v36*((v1866*(self.scalar_static_f64[142]*(((-(self.scalar_static_f64[134]*v2463))/v2484)*(self.scalar_static_f64[143]*f64::powf(v656,self.scalar_static_f64[237])))))+(v659*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{(((if self.scalar_static_bool[72]{(((v1851*v5248)+(v1784*((-(((v642*v5364)-(v1848*v2463))/v2484))*v5377)))/self.scalar_static_f64[203])}else{v5308})+((v1860*(self.scalar_static_f64[202]*v5395))+(v1857*(((v1805*(self.scalar_static_f64[204]*v5395))-(v1858*v5260))/v5264))))-(if self.scalar_static_bool[72]{(((v1836*v5248)+(v1784*((-(((v642*v5327)-(v1833*v2463))/v2484))*(self.scalar_static_f64[203]*f64::powf(v1835,self.scalar_static_f64[250])))))/self.scalar_static_f64[203])}else{v1}))}else{(if self.scalar_static_bool[70]{(v5308+(if v1812{v1}else{(if v1793{(v1796*((v1807*v5251)+(v1791*(((v1805*(self.scalar_static_f64[204]*v5251))-(v1804*v5260))/v5264))))}else{v1})}))}else{v1})})}))));
        let v6053=(v36*((v659*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{((if self.scalar_static_bool[72]{((v1784*((-(v5365/v642))*v5377))/self.scalar_static_f64[203])}else{v5309})+((v1860*(self.scalar_static_f64[202]*v5393))+(v1857*((self.scalar_static_f64[204]*v5393)/v1805))))}else{(if self.scalar_static_bool[70]{(v5309+(if v1812{v1}else{(if v1793{(v1796*((v1807*v5252)+(v1791*((self.scalar_static_f64[204]*v5252)/v1805))))}else{v1})}))}else{v1})})}))+(self.scalar_static_f64[212]*v2571)));
        let v6054=(v36*((v659*(if self.scalar_static_bool[73]{v1}else{(if self.scalar_static_bool[72]{((if self.scalar_static_bool[72]{((v1784*((-(v5366/v642))*v5377))/self.scalar_static_f64[203])}else{v5310})+((v1860*(self.scalar_static_f64[202]*v5394))+(v1857*((self.scalar_static_f64[204]*v5394)/v1805))))}else{(if self.scalar_static_bool[70]{(v5310+(if v1812{v1}else{(if v1793{(v1796*((v1807*v5253)+(v1791*((self.scalar_static_f64[204]*v5253)/v1805))))}else{v1})}))}else{v1})})}))+(v36*self.scalar_static_f64[212])));

        CommonStampValues {
            v0,
            v1,
            v2,
            v18,
            v26,
            v36,
            v39,
            v49,
            v98,
            v122,
            v134,
            v136,
            v371,
            v390,
            v391,
            v392,
            v460,
            v479,
            v483,
            v490,
            v497,
            v504,
            v515,
            v552,
            v615,
            v670,
            v671,
            v722,
            v723,
            v725,
            v726,
            v728,
            v729,
            v731,
            v732,
            v737,
            v739,
            v740,
            v741,
            v745,
            v754,
            v756,
            v761,
            v762,
            v1041,
            v1058,
            v1063,
            v1066,
            v1071,
            v1096,
            v1108,
            v1147,
            v1174,
            v1202,
            v1204,
            v1256,
            v1279,
            v1280,
            v1317,
            v1335,
            v1336,
            v1377,
            v1394,
            v1395,
            v1430,
            v1448,
            v1449,
            v1484,
            v1485,
            v1513,
            v1529,
            v1532,
            v1689,
            v1703,
            v2081,
            v2083,
            v2085,
            v2087,
            v2090,
            v2091,
            v2092,
            v2093,
            v2094,
            v2095,
            v2096,
            v2101,
            v2103,
            v2104,
            v2175,
            v2218,
            v2225,
            v2229,
            v2241,
            v2245,
            v2257,
            v2261,
            v2273,
            v2277,
            v2297,
            v2301,
            v2425,
            v2509,
            v2512,
            v2516,
            v2571,
            v3162,
            v3163,
            v3164,
            v3198,
            v3199,
            v3200,
            v3201,
            v3231,
            v3232,
            v3233,
            v3234,
            v3293,
            v3294,
            v3295,
            v3296,
            v3323,
            v3324,
            v3325,
            v3326,
            v3330,
            v3422,
            v3423,
            v3424,
            v3425,
            v3426,
            v3427,
            v3492,
            v3493,
            v3494,
            v3495,
            v3496,
            v3497,
            v3498,
            v3598,
            v3599,
            v3600,
            v3601,
            v3602,
            v3603,
            v3604,
            v3607,
            v3714,
            v3715,
            v3716,
            v3717,
            v3760,
            v3761,
            v3762,
            v3763,
            v3764,
            v3765,
            v3766,
            v3767,
            v3841,
            v3842,
            v3843,
            v3844,
            v3880,
            v3881,
            v3882,
            v3883,
            v3884,
            v3885,
            v3886,
            v3887,
            v4001,
            v4002,
            v4003,
            v4004,
            v4040,
            v4041,
            v4042,
            v4043,
            v4044,
            v4045,
            v4046,
            v4047,
            v4128,
            v4129,
            v4130,
            v4131,
            v4168,
            v4169,
            v4170,
            v4171,
            v4172,
            v4173,
            v4174,
            v4176,
            v4244,
            v4245,
            v4246,
            v4247,
            v4248,
            v4249,
            v4250,
            v4251,
            v4367,
            v4368,
            v4369,
            v4370,
            v4371,
            v4372,
            v4373,
            v4382,
            v4383,
            v4384,
            v4385,
            v4386,
            v4866,
            v4888,
            v4889,
            v4890,
            v4891,
            v4892,
            v4893,
            v4894,
            v6024,
            v6025,
            v6026,
            v6027,
            v6028,
            v6029,
            v6030,
            v6031,
            v6032,
            v6033,
            v6034,
            v6035,
            v6036,
            v6037,
            v6038,
            v6039,
            v6040,
            v6041,
            v6042,
            v6043,
            v6044,
            v6045,
            v6046,
            v6047,
            v6048,
            v6049,
            v6050,
            v6051,
            v6052,
            v6053,
            v6054,
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
        let v260=((self.scalar_static_f64[68]*f64::powf(common.v98,self.scalar_static_f64[71]))*(((common.v122*self.scalar_static_f64[73])/self.scalar_static_f64[276])).exp());
        let v261=(v260>common.v1);
        let v268=(if (!v261){common.v1}else{(if v261{(self.scalar_static_f64[276]*((common.v2+(common.v18/v260))).ln())}else{common.v1})});
        let v292=f64::powf(common.v98,self.scalar_static_f64[81]);
        let v299=(((common.v122*self.scalar_static_f64[83])/self.scalar_static_f64[278])).exp();
        let v300=((self.scalar_static_f64[79]*v292)*v299);
        let v301=(v300>common.v1);
        let v308=(if (!v301){common.v1}else{(if v301{(self.scalar_static_f64[278]*((common.v2+(common.v18/v300))).ln())}else{common.v1})});
        let v322=(v299*(v292*self.scalar_static_f64[85]));
        let v323=(v322>common.v1);
        let v330=(if (!v323){common.v1}else{(if v323{(self.scalar_static_f64[278]*((common.v2+(common.v18/v322))).ln())}else{common.v1})});
        let v362=((self.scalar_static_f64[91]*f64::powf(common.v98,self.scalar_static_f64[93]))*(((common.v122*self.scalar_static_f64[95])/self.scalar_static_f64[280])).exp());
        let v363=(v362>common.v1);
        let v370=(if (!v363){common.v1}else{(if v363{(self.scalar_static_f64[280]*((common.v2+(common.v18/v362))).ln())}else{common.v1})});
        let v404=f64::powf(common.v391,self.scalar_static_f64[100]);
        let v406=(if self.scalar_static_bool[13]{(self.scalar_static_f64[98]*v404)}else{(if (self.scalar_static_f64[97]!=0.0){(self.scalar_static_f64[98]*f64::powf(common.v391,self.scalar_static_f64[99]))}else{common.v1})});
        let v415=(if self.scalar_static_bool[14]{(v404*self.scalar_static_f64[102])}else{(if (self.scalar_static_f64[101]!=0.0){(self.scalar_static_f64[102]*f64::powf(common.v391,self.scalar_static_f64[103]))}else{common.v1})});
        let v424=f64::powf(common.v391,self.scalar_static_f64[107]);
        let v426=(if self.scalar_static_bool[15]{(self.scalar_static_f64[105]*v424)}else{(if (self.scalar_static_f64[104]!=0.0){(self.scalar_static_f64[105]*f64::powf(common.v391,self.scalar_static_f64[106]))}else{common.v1})});
        let v435=(if self.scalar_static_bool[16]{(v424*self.scalar_static_f64[109])}else{(if (self.scalar_static_f64[108]!=0.0){(self.scalar_static_f64[109]*f64::powf(common.v391,self.scalar_static_f64[110]))}else{common.v1})});
        let v439=(self.scalar_static_f64[111]*f64::powf(common.v391,self.scalar_static_f64[112]));
        let v443=(self.scalar_static_f64[113]*f64::powf(common.v391,self.scalar_static_f64[114]));
        let v452=(if self.scalar_static_bool[17]{(v404*self.scalar_static_f64[116])}else{(if (self.scalar_static_f64[115]!=0.0){(self.scalar_static_f64[116]*f64::powf(common.v391,self.scalar_static_f64[117]))}else{common.v1})});
        let v457=(self.scalar_static_f64[118]*(common.v2+(common.v392*self.scalar_static_f64[119])));
        let v481=(self.scalar_static_f64[62]*f64::powf(common.v391,self.scalar_static_f64[65]));
        let v482=(self.scalar_static_f64[67]*common.v460);
        let v485=((v482/common.v483)).exp();
        let v486=(v481*v485);
        let v488=(self.scalar_static_f64[68]*f64::powf(common.v391,self.scalar_static_f64[71]));
        let v489=(self.scalar_static_f64[73]*common.v460);
        let v492=((v489/common.v490)).exp();
        let v493=(v488*v492);
        let v494=f64::powf(common.v391,self.scalar_static_f64[76]);
        let v495=(self.scalar_static_f64[74]*v494);
        let v496=(self.scalar_static_f64[78]*common.v460);
        let v499=((v496/common.v497)).exp();
        let v500=(v495*v499);
        let v501=f64::powf(common.v391,self.scalar_static_f64[81]);
        let v502=(self.scalar_static_f64[79]*v501);
        let v503=(self.scalar_static_f64[83]*common.v460);
        let v506=((v503/common.v504)).exp();
        let v507=(v502*v506);
        let v508=(self.scalar_static_f64[84]*v494);
        let v509=(v499*v508);
        let v510=(self.scalar_static_f64[85]*v501);
        let v511=(v506*v510);
        let v513=(self.scalar_static_f64[86]*f64::powf(common.v391,self.scalar_static_f64[88]));
        let v514=(self.scalar_static_f64[90]*common.v460);
        let v517=((v514/common.v515)).exp();
        let v518=(v513*v517);
        let v520=(self.scalar_static_f64[91]*f64::powf(common.v391,self.scalar_static_f64[93]));
        let v521=(self.scalar_static_f64[95]*common.v460);
        let v522=(self.scalar_static_f64[92]*common.v390);
        let v524=((v521/v522)).exp();
        let v525=(v520*v524);
        let v535=(self.scalar_static_f64[121]*(common.v2+(common.v392*self.scalar_static_f64[122])));
        let v540=(self.scalar_static_f64[123]*(common.v2+(common.v392*self.scalar_static_f64[124])));
        let v669=(self.scalar_static_f64[145]*f64::powf(common.v391,self.scalar_static_f64[146]));
        let v673=((common.v670/common.v671)).exp();
        let v684=0.001;
        let v685=(v406>v684);
        let v687=1000.0;
        let v688=(if v685{(common.v2/v406)}else{v687});
        let v689=(v415>v684);
        let v691=(if v689{(common.v2/v415)}else{v687});
        let v692=(v426>v684);
        let v694=(if v692{(common.v2/v426)}else{v687});
        let v695=(v435>v684);
        let v697=(if v695{(common.v2/v435)}else{v687});
        let v698=(v439>v684);
        let v700=(if v698{(common.v2/v439)}else{v687});
        let v701=(v452>v684);
        let v703=(if v701{(common.v2/v452)}else{v687});
        let v704=(v443>v684);
        let v706=(if v704{(common.v2/v443)}else{v687});
        let v707=(v457>v684);
        let v709=(if v707{(common.v2/v457)}else{v687});
        let v719=(v669>common.v1);
        let v721=(if v719{(common.v2/v669)}else{common.v1});
        let v736=(common.v36*(common.v726-common.v732));
        let v744=(common.v36*(common.v729-common.v723));
        let v747=(common.v745-common.v732);
        let v749=(common.v36*(common.v732-common.v729));
        let v750=(common.v740-common.v726);
        let v751=(common.v726-common.v722);
        let v752=(common.v741-common.v723);
        let v753=(common.v737-common.v732);
        let v758=(common.v36*(common.v726-common.v754));
        let v760=(ctx.node_voltage(nodes[3])-common.v754);
        let v1109=(common.v1058/common.v1108);
        let v1110=(common.v1041/common.v1108);
        let v1152=(if self.scalar_static_bool[30]{(common.v2+(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v1147)}else{common.v1})))}else{common.v1096});
        let v1153=(v1152>common.v1066);
        let v1154=(self.scalar_static_bool[30]&&v1153);
        let v1155=(v1152).sqrt();
        let v1160=(self.scalar_static_bool[30]&&(!v1153));
        let v1162=(if v1160{0.50005}else{(if v1154{(common.v134*(common.v2+v1155))}else{common.v1})});
        let v1175=(common.v1174-common.v2);
        let v1178=(common.v1147-(if self.scalar_static_bool[30]{(common.v479*v1175)}else{common.v1}));
        let v1183=(if self.scalar_static_bool[31]{common.v2}else{v1162});
        let v1184=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(v1178/v1162)}else{common.v1})});
        let v1205=(common.v725<v268);
        let v1206=(self.scalar_static_bool[32]&&v1205);
        let v1208=((common.v725*common.v1204)).exp();
        let v1210=(!v1205);
        let v1211=(self.scalar_static_bool[32]&&v1210);
        let v1213=((v268*common.v1204)).exp();
        let v1214=(common.v725-v268);
        let v1216=(common.v2+(common.v1204*v1214));
        let v1218=(if v1211{(v1213*v1216)}else{(if v1206{v1208}else{common.v1})});
        let v1224=(common.v2+(self.scalar_static_f64[181]*(common.v1071-common.v2)));
        let v1225=(v486*v1224);
        let v1226=(common.v1202-common.v2);
        let v1228=(v1218-common.v2);
        let v1229=(v493*v1228);
        let v1236=(if self.scalar_static_bool[36]{(v1229+(v486*v1226))}else{(if self.scalar_static_bool[34]{((v1225*v1226)+v1229)}else{common.v1})});
        let v1281=(common.v728<v268);
        let v1282=(self.scalar_static_bool[41]&&v1281);
        let v1284=((common.v728*common.v1280)).exp();
        let v1286=(!v1281);
        let v1287=(self.scalar_static_bool[41]&&v1286);
        let v1289=((v268*common.v1280)).exp();
        let v1290=(common.v728-v268);
        let v1292=(common.v2+(common.v1280*v1290));
        let v1294=(if v1287{(v1289*v1292)}else{(if v1282{v1284}else{v1218})});
        let v1295=(common.v1279-common.v2);
        let v1297=(v1294-common.v2);
        let v1300=(if self.scalar_static_bool[41]{((v486*v1295)+(v493*v1297))}else{common.v1});
        let v1337=(v1205&&self.scalar_static_bool[44]);
        let v1339=((common.v725*common.v1336)).exp();
        let v1341=(v1210&&self.scalar_static_bool[44]);
        let v1343=((v268*common.v1336)).exp();
        let v1345=(common.v2+(v1214*common.v1336));
        let v1347=(if v1341{(v1343*v1345)}else{(if v1337{v1339}else{v1294})});
        let v1349=(common.v1335-common.v2);
        let v1351=(v1347-common.v2);
        let v1352=(v493*v1351);
        let v1360=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v1352+(v486*v1349)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*((v1225*v1349)+v1352))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v1236-(self.scalar_static_f64[34]*(common.v1256-v673)))}else{v1236})})})});
        let v1382=(if self.scalar_static_bool[47]{(v1360-(self.scalar_static_f64[182]*(common.v1377-v673)))}else{v1360});
        let v1396=(v1281&&self.scalar_static_bool[44]);
        let v1398=((common.v728*common.v1395)).exp();
        let v1400=(v1286&&self.scalar_static_bool[44]);
        let v1402=((v268*common.v1395)).exp();
        let v1404=(common.v2+(v1290*common.v1395));
        let v1406=(if v1400{(v1402*v1404)}else{(if v1396{v1398}else{v1347})});
        let v1408=(common.v1394-common.v2);
        let v1410=(v1406-common.v2);
        let v1414=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*((v486*v1408)+(v493*v1410)))}else{(if self.scalar_static_bool[42]{(v1300-(self.scalar_static_f64[34]*(common.v1317-v673)))}else{v1300})});
        let v1435=(if self.scalar_static_bool[47]{(v1414-(self.scalar_static_f64[184]*(common.v1430-v673)))}else{v1414});
        let v1450=(common.v731<v308);
        let v1452=((common.v731*common.v1449)).exp();
        let v1454=(!v1450);
        let v1456=((v308*common.v1449)).exp();
        let v1457=(common.v731-v308);
        let v1459=(common.v2+(common.v1449*v1457));
        let v1461=(if v1454{(v1456*v1459)}else{(if v1450{v1452}else{v1406})});
        let v1462=(common.v1448-common.v2);
        let v1464=(v1461-common.v2);
        let v1466=((v500*v1462)+(v507*v1464));
        let v1486=(common.v739<v330);
        let v1487=(self.scalar_static_bool[50]&&v1486);
        let v1489=((common.v739*common.v1485)).exp();
        let v1492=(self.scalar_static_bool[50]&&(!v1486));
        let v1494=((v330*common.v1485)).exp();
        let v1495=(common.v739-v330);
        let v1497=(common.v2+(common.v1485*v1495));
        let v1499=(if v1492{(v1494*v1497)}else{(if v1487{v1489}else{v1461})});
        let v1500=(common.v1484-common.v2);
        let v1502=(v1499-common.v2);
        let v1507=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*v1500)+(v511*v1502))}else{common.v1})});
        let v1533=(v688*v747);
        let v1534=(common.v2+common.v1529);
        let v1535=(common.v2+common.v1532);
        let v1536=(v1534/v1535);
        let v1539=((common.v1529-common.v1532)-(v1536).ln());
        let v1541=(v749+(common.v390*v1539));
        let v1542=(v691*v1541);
        let v1543=(v721*v1542);
        let v1545=(v54*(common.v134*v721));
        let v1548=((v3+(v749*v749))).sqrt();
        let v1550=(common.v2+(v1545*v1548));
        let v1551=(v691*v1550);
        let v1552=(v1543/v1551);
        let v1555=((common.v2+(v1552*v1552))).sqrt();
        let v1556=(v1542/v1555);
        let v1557=(v694*v750);
        let v1558=(v751*common.v1108);
        let v1559=(v697*v1558);
        let v1560=(v700*v752);
        let v1561=(v753*v1183);
        let v1562=(v703*v1561);
        let v1563=(v706*v760);
        let v1566=0.02;
        let v1568=(v1566*(common.v2+v535));
        let v1573=(if self.scalar_static_bool[52]{f64::powf(v1568,self.scalar_static_f64[187])}else{common.v1});
        let v1575=((common.v615-common.v731)-v1573);
        let v1578=((v3+(v1575*v1575))).sqrt();
        let v1582=(if self.scalar_static_bool[52]{(v1573+(common.v134*(v1575+v1578)))}else{common.v1});
        let v1583=(-v535);
        let v1585=f64::powf(v1582,self.scalar_static_f64[188]);
        let v1587=(if self.scalar_static_bool[52]{(v1583*v1585)}else{common.v1});
        let v1588=(v1587<common.v39);
        let v1589=(self.scalar_static_bool[52]&&v1588);
        let v1590=(v1587).exp();
        let v1593=(self.scalar_static_bool[52]&&(!v1588));
        let v1594=(if v1593{common.v1513}else{common.v1});
        let v1598=(if v1593{(v1594*(common.v2+(v1587-common.v39)))}else{(if v1589{v1590}else{common.v1})});
        let v1599=(self.scalar_static_f64[185]*v1582);
        let v1601=(if self.scalar_static_bool[52]{(v1598*v1599)}else{common.v1});
        let v1602=(common.v762-v1109);
        let v1603=(v1602-v1466);
        let v1611=(v1566*(common.v2+v540));
        let v1616=(if self.scalar_static_bool[54]{f64::powf(v1611,self.scalar_static_f64[192])}else{common.v1});
        let v1618=((common.v1-v736)-v1616);
        let v1621=((v3+(v1618*v1618))).sqrt();
        let v1625=(if self.scalar_static_bool[54]{(v1616+(common.v134*(v1618+v1621)))}else{common.v1});
        let v1626=(-v540);
        let v1628=f64::powf(v1625,self.scalar_static_f64[193]);
        let v1630=(if self.scalar_static_bool[54]{(v1626*v1628)}else{common.v1});
        let v1631=(v1630<common.v39);
        let v1632=(self.scalar_static_bool[54]&&v1631);
        let v1633=(v1630).exp();
        let v1636=(self.scalar_static_bool[54]&&(!v1631));
        let v1637=(if v1636{common.v1513}else{common.v1});
        let v1641=(if v1636{(v1637*(common.v2+(v1630-common.v39)))}else{(if v1632{v1633}else{common.v1})});
        let v1642=(self.scalar_static_f64[189]*v1625);
        let v1644=(if self.scalar_static_bool[54]{(v1641*v1642)}else{v1601});
        let v1645=(-v1533);
        let v1660=0.1;
        let v1662=(if self.scalar_static_bool[60]{((common.v2-(common.v731/self.scalar_static_f64[196]))-v1660)}else{common.v1});
        let v1665=((common.v1063+(v1662*v1662))).sqrt();
        let v1674=(if self.scalar_static_bool[62]{self.scalar_static_f64[195]}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[195]*(if self.scalar_static_bool[60]{(v1660+(common.v134*(v1662+v1665)))}else{v1662}))}else{common.v1})});
        let v1676=((v1110/v1674)-common.v2);
        let v1684=((v1466-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{(v1601*v1603)}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[194]*f64::powf(v1676,self.scalar_static_f64[197]))}else{common.v1})}));
        let v1705=(if self.scalar_static_bool[66]{(common.v2/v522)}else{common.v1689});
        let v1706=(common.v756<v370);
        let v1707=(self.scalar_static_bool[66]&&v1706);
        let v1709=((common.v756*v1705)).exp();
        let v1712=(self.scalar_static_bool[66]&&(!v1706));
        let v1714=((v370*v1705)).exp();
        let v1715=(common.v756-v370);
        let v1717=(common.v2+(v1705*v1715));
        let v1720=(common.v1703-common.v2);
        let v1722=((if v1712{(v1714*v1717)}else{(if v1707{v1709}else{v1499})})-common.v2);
        let v1727=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*v1720)+(v525*v1722))}else{common.v1})});
        let v1780=(common.v36*v1556);
        let v1782=(common.v36*v1184);
        let v2119=(common.v2104*(self.scalar_static_f64[100]*f64::powf(common.v391,self.scalar_static_f64[219])));
        let v2139=(common.v2104*(self.scalar_static_f64[107]*f64::powf(common.v391,self.scalar_static_f64[222])));
        let v2234=((v485*(self.scalar_static_f64[62]*(common.v2104*(self.scalar_static_f64[65]*f64::powf(common.v391,self.scalar_static_f64[230])))))+(v481*(v485*(((common.v483*(self.scalar_static_f64[67]*common.v2175))-(v482*common.v2225))/common.v2229))));
        let v2250=((v492*(self.scalar_static_f64[68]*(common.v2104*(self.scalar_static_f64[71]*f64::powf(common.v391,self.scalar_static_f64[231])))))+(v488*(v492*(((common.v490*(self.scalar_static_f64[73]*common.v2175))-(v489*common.v2241))/common.v2245))));
        let v2254=(common.v2104*(self.scalar_static_f64[76]*f64::powf(common.v391,self.scalar_static_f64[232])));
        let v2263=(v499*(((common.v497*(self.scalar_static_f64[78]*common.v2175))-(v496*common.v2257))/common.v2261));
        let v2270=(common.v2104*(self.scalar_static_f64[81]*f64::powf(common.v391,self.scalar_static_f64[233])));
        let v2279=(v506*(((common.v504*(self.scalar_static_f64[83]*common.v2175))-(v503*common.v2273))/common.v2277));
        let v2313=(self.scalar_static_f64[92]*common.v2103);
        let v2317=(v522*v522);
        let v2327=(self.scalar_static_f64[121]*(self.scalar_static_f64[122]*common.v2101));
        let v2329=(self.scalar_static_f64[123]*(self.scalar_static_f64[124]*common.v2101));
        let v2518=(v673*(((common.v671*common.v2509)-(common.v670*common.v2512))/common.v2516));
        let v2530=(if v689{((-(if self.scalar_static_bool[14]{(self.scalar_static_f64[102]*v2119)}else{(if (self.scalar_static_f64[101]!=0.0){(self.scalar_static_f64[102]*(common.v2104*(self.scalar_static_f64[103]*f64::powf(common.v391,self.scalar_static_f64[220]))))}else{common.v1})}))/(v415*v415))}else{common.v1});
        let v2570=(if v719{((-(self.scalar_static_f64[145]*(common.v2104*(self.scalar_static_f64[146]*f64::powf(common.v391,self.scalar_static_f64[239])))))/(v669*v669))}else{common.v1});
        let v3331=(((common.v1108*common.v3198)-(common.v1058*common.v3323))/common.v3330);
        let v3335=(((common.v1108*common.v3199)-(common.v1058*common.v3324))/common.v3330);
        let v3339=(((common.v1108*common.v3200)-(common.v1058*common.v3325))/common.v3330);
        let v3343=(((common.v1108*common.v3201)-(common.v1058*common.v3326))/common.v3330);
        let v3347=(((common.v1108*common.v3162)-(common.v1041*common.v3323))/common.v3330);
        let v3350=((-(common.v1041*common.v3324))/common.v3330);
        let v3354=(((common.v1108*common.v3163)-(common.v1041*common.v3325))/common.v3330);
        let v3358=(((common.v1108*common.v3164)-(common.v1041*common.v3326))/common.v3330);
        let v3452=(common.v552*v1155);
        let v3471=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3422)}else{common.v1}))}else{common.v3293})/v3452))}else{common.v1})});
        let v3472=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3423)}else{common.v1}))}else{common.v3294})/v3452))}else{common.v1})});
        let v3473=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3424)}else{common.v1}))}else{common.v1})/v3452))}else{common.v1})});
        let v3474=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3425)}else{common.v1}))}else{common.v3295})/v3452))}else{common.v1})});
        let v3475=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3426)}else{common.v1}))}else{common.v3296})/v3452))}else{common.v1})});
        let v3476=(if v1160{common.v1}else{(if v1154{(common.v134*((if self.scalar_static_bool[30]{(common.v136*(if self.scalar_static_bool[30]{(common.v49*common.v3427)}else{common.v1}))}else{common.v1})/v3452))}else{common.v1})});
        let v3525=(v1162*v1162);
        let v3567=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3422-(if self.scalar_static_bool[30]{((v1175*common.v2218)+(common.v479*common.v3492))}else{common.v1})))-(v1178*v3471))/v3525)}else{common.v1})});
        let v3568=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3423-(if self.scalar_static_bool[30]{(common.v479*common.v3493)}else{common.v1})))-(v1178*v3472))/v3525)}else{common.v1})});
        let v3569=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3424-(if self.scalar_static_bool[30]{(common.v479*common.v3494)}else{common.v1})))-(v1178*v3473))/v3525)}else{common.v1})});
        let v3570=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3425-(if self.scalar_static_bool[30]{(common.v479*common.v3495)}else{common.v1})))-(v1178*v3474))/v3525)}else{common.v1})});
        let v3571=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3426-(if self.scalar_static_bool[30]{(common.v479*common.v3496)}else{common.v1})))-(v1178*v3475))/v3525)}else{common.v1})});
        let v3572=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{(((v1162*(common.v3427-(if self.scalar_static_bool[30]{(common.v479*common.v3497)}else{common.v1})))-(v1178*v3476))/v3525)}else{common.v1})});
        let v3573=(if self.scalar_static_bool[31]{common.v1}else{(if self.scalar_static_bool[30]{((-(if self.scalar_static_bool[30]{(common.v479*common.v3498)}else{common.v1}))/v1162)}else{common.v1})});
        let v3609=(common.v36*common.v1204);
        let v3610=(common.v1204*common.v2571);
        let v3625=(if v1211{((v1216*(v1213*(v268*common.v3607)))+(v1213*(v1214*common.v3607)))}else{(if v1206{(v1208*(common.v725*common.v3607))}else{common.v1})});
        let v3626=(if v1211{(v1213*v3609)}else{(if v1206{(v1208*v3609)}else{common.v1})});
        let v3627=(if v1211{(v1213*v3610)}else{(if v1206{(v1208*v3610)}else{common.v1})});
        let v3634=((v1224*v2234)+(v486*(self.scalar_static_f64[181]*common.v3231)));
        let v3635=(v486*(self.scalar_static_f64[181]*common.v3232));
        let v3636=(v486*(self.scalar_static_f64[181]*common.v3233));
        let v3637=(v486*(self.scalar_static_f64[181]*common.v3234));
        let v3655=((v1228*v2250)+(v493*v3625));
        let v3656=(v493*v3626);
        let v3657=(v493*v3627);
        let v3680=(if self.scalar_static_bool[36]{(v3655+((v1226*v2234)+(v486*common.v3598)))}else{(if self.scalar_static_bool[34]{(((v1226*v3634)+(v1225*common.v3598))+v3655)}else{common.v1})});
        let v3681=(if self.scalar_static_bool[36]{(v486*common.v3599)}else{(if self.scalar_static_bool[34]{((v1226*v3635)+(v1225*common.v3599))}else{common.v1})});
        let v3683=(if self.scalar_static_bool[36]{(v3656+(v486*common.v3601))}else{(if self.scalar_static_bool[34]{(((v1226*v3636)+(v1225*common.v3601))+v3656)}else{common.v1})});
        let v3684=(if self.scalar_static_bool[36]{(v3657+(v486*common.v3602))}else{(if self.scalar_static_bool[34]{(((v1226*v3637)+(v1225*common.v3602))+v3657)}else{common.v1})});
        let v3769=(common.v36*common.v1280);
        let v3770=(common.v1280*common.v2571);
        let v3786=(if v1287{((v1292*(v1289*(v268*common.v3767)))+(v1289*(v1290*common.v3767)))}else{(if v1282{(v1284*(common.v728*common.v3767))}else{v3625})});
        let v3787=(if v1287{(v1289*v3769)}else{(if v1282{(v1284*v3769)}else{common.v1})});
        let v3788=(if v1287{common.v1}else{(if v1282{common.v1}else{v3626})});
        let v3789=(if v1287{(v1289*v3770)}else{(if v1282{(v1284*v3770)}else{v3627})});
        let v3809=(if self.scalar_static_bool[41]{(((v1295*v2234)+(v486*common.v3760))+((v1297*v2250)+(v493*v3786)))}else{common.v1});
        let v3810=(if self.scalar_static_bool[41]{(v486*common.v3761)}else{common.v1});
        let v3812=(if self.scalar_static_bool[41]{((v486*common.v3763)+(v493*v3788))}else{common.v1});
        let v3813=(if self.scalar_static_bool[41]{((v486*common.v3764)+(v493*v3789))}else{common.v1});
        let v3889=(common.v36*common.v1336);
        let v3890=(common.v1336*common.v2571);
        let v3906=(if v1341{((v1345*(v1343*(v268*common.v3887)))+(v1343*(v1214*common.v3887)))}else{(if v1337{(v1339*(common.v725*common.v3887))}else{v3786})});
        let v3907=(if v1341{common.v1}else{(if v1337{common.v1}else{v3787})});
        let v3908=(if v1341{(v1343*v3889)}else{(if v1337{(v1339*v3889)}else{v3788})});
        let v3909=(if v1341{(v1343*v3890)}else{(if v1337{(v1339*v3890)}else{v3789})});
        let v3927=((v1351*v2250)+(v493*v3906));
        let v3928=(v493*v3907);
        let v3929=(v493*v3908);
        let v3930=(v493*v3909);
        let v3969=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v3927+((v1349*v2234)+(v486*common.v3880))))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*(((v1349*v3634)+(v1225*common.v3880))+v3927))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3680-(self.scalar_static_f64[34]*(common.v3714-v2518)))}else{v3680})})})});
        let v3970=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v486*common.v3881))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*((v1349*v3635)+(v1225*common.v3881)))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3681-(self.scalar_static_f64[34]*common.v3715))}else{v3681})})})});
        let v3971=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v3928+(v486*common.v3882)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*((v1225*common.v3882)+v3928))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v486*common.v3600)}else{(if self.scalar_static_bool[34]{(v1225*common.v3600)}else{common.v1})})})})});
        let v3972=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v3929+(v486*common.v3883)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*(((v1349*v3636)+(v1225*common.v3883))+v3929))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3683-(self.scalar_static_f64[34]*common.v3716))}else{v3683})})})});
        let v3973=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v3930+(v486*common.v3884)))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*(((v1349*v3637)+(v1225*common.v3884))+v3930))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[38]{(v3684-(self.scalar_static_f64[34]*common.v3717))}else{v3684})})})});
        let v3974=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v486*common.v3885))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*(v1225*common.v3885))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v486*common.v3603)}else{(if self.scalar_static_bool[34]{(v1225*common.v3603)}else{common.v1})})})})});
        let v3975=(if self.scalar_static_bool[46]{(self.scalar_static_f64[180]*(v486*common.v3886))}else{(if self.scalar_static_bool[45]{(self.scalar_static_f64[180]*(v1225*common.v3886))}else{(if self.scalar_static_bool[41]{common.v1}else{(if self.scalar_static_bool[36]{(v486*common.v3604)}else{(if self.scalar_static_bool[34]{(v1225*common.v3604)}else{common.v1})})})})});
        let v4014=(if self.scalar_static_bool[47]{(v3969-(self.scalar_static_f64[182]*(common.v4001-v2518)))}else{v3969});
        let v4015=(if self.scalar_static_bool[47]{(v3970-(self.scalar_static_f64[182]*common.v4002))}else{v3970});
        let v4016=(if self.scalar_static_bool[47]{(v3972-(self.scalar_static_f64[182]*common.v4003))}else{v3972});
        let v4017=(if self.scalar_static_bool[47]{(v3973-(self.scalar_static_f64[182]*common.v4004))}else{v3973});
        let v4049=(common.v36*common.v1395);
        let v4050=(common.v1395*common.v2571);
        let v4066=(if v1400{((v1404*(v1402*(v268*common.v4047)))+(v1402*(v1290*common.v4047)))}else{(if v1396{(v1398*(common.v728*common.v4047))}else{v3906})});
        let v4067=(if v1400{(v1402*v4049)}else{(if v1396{(v1398*v4049)}else{v3907})});
        let v4068=(if v1400{common.v1}else{(if v1396{common.v1}else{v3908})});
        let v4069=(if v1400{(v1402*v4050)}else{(if v1396{(v1398*v4050)}else{v3909})});
        let v4096=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*(((v1408*v2234)+(v486*common.v4040))+((v1410*v2250)+(v493*v4066))))}else{(if self.scalar_static_bool[42]{(v3809-(self.scalar_static_f64[34]*(common.v3841-v2518)))}else{v3809})});
        let v4097=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*(v486*common.v4041))}else{(if self.scalar_static_bool[42]{(v3810-(self.scalar_static_f64[34]*common.v3842))}else{v3810})});
        let v4098=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*((v486*common.v4042)+(v493*v4067)))}else{(if self.scalar_static_bool[41]{((v486*common.v3762)+(v493*v3787))}else{common.v1})});
        let v4099=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*((v486*common.v4043)+(v493*v4068)))}else{(if self.scalar_static_bool[42]{(v3812-(self.scalar_static_f64[34]*common.v3843))}else{v3812})});
        let v4100=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*((v486*common.v4044)+(v493*v4069)))}else{(if self.scalar_static_bool[42]{(v3813-(self.scalar_static_f64[34]*common.v3844))}else{v3813})});
        let v4101=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*(v486*common.v4045))}else{(if self.scalar_static_bool[41]{(v486*common.v3765)}else{common.v1})});
        let v4102=(if self.scalar_static_bool[44]{(self.scalar_static_f64[183]*(v486*common.v4046))}else{(if self.scalar_static_bool[41]{(v486*common.v3766)}else{common.v1})});
        let v4141=(if self.scalar_static_bool[47]{(v4096-(self.scalar_static_f64[184]*(common.v4128-v2518)))}else{v4096});
        let v4142=(if self.scalar_static_bool[47]{(v4097-(self.scalar_static_f64[184]*common.v4129))}else{v4097});
        let v4143=(if self.scalar_static_bool[47]{(v4099-(self.scalar_static_f64[184]*common.v4130))}else{v4099});
        let v4144=(if self.scalar_static_bool[47]{(v4100-(self.scalar_static_f64[184]*common.v4131))}else{v4100});
        let v4178=(common.v1449*common.v2571);
        let v4179=(common.v36*common.v1449);
        let v4196=(if v1454{((v1459*(v1456*(v308*common.v4176)))+(v1456*(v1457*common.v4176)))}else{(if v1450{(v1452*(common.v731*common.v4176))}else{v4066})});
        let v4197=(if v1454{(v1456*v4178)}else{(if v1450{(v1452*v4178)}else{common.v1})});
        let v4198=(if v1454{common.v1}else{(if v1450{common.v1}else{v4067})});
        let v4199=(if v1454{(v1456*v4179)}else{(if v1450{(v1452*v4179)}else{v4068})});
        let v4200=(if v1454{common.v1}else{(if v1450{common.v1}else{v4069})});
        let v4208=(v500*common.v4173);
        let v4209=(v500*common.v4174);
        let v4217=(((v1462*((v499*(self.scalar_static_f64[74]*v2254))+(v495*v2263)))+(v500*common.v4168))+((v1464*((v506*(self.scalar_static_f64[79]*v2270))+(v502*v2279)))+(v507*v4196)));
        let v4218=((v500*common.v4169)+(v507*v4197));
        let v4219=((v500*common.v4170)+(v507*v4198));
        let v4220=((v500*common.v4171)+(v507*v4199));
        let v4221=((v500*common.v4172)+(v507*v4200));
        let v4253=(common.v36*common.v1485);
        let v4254=(common.v1485*common.v2571);
        let v4272=(if v1492{((v1497*(v1494*(v330*common.v4251)))+(v1494*(v1495*common.v4251)))}else{(if v1487{(v1489*(common.v739*common.v4251))}else{v4196})});
        let v4273=(if v1492{common.v1}else{(if v1487{common.v1}else{v4197})});
        let v4274=(if v1492{(v1494*v4253)}else{(if v1487{(v1489*v4253)}else{v4198})});
        let v4275=(if v1492{common.v1}else{(if v1487{common.v1}else{v4199})});
        let v4276=(if v1492{common.v1}else{(if v1487{common.v1}else{v4200})});
        let v4277=(if v1492{(v1494*v4254)}else{(if v1487{(v1489*v4254)}else{common.v1})});
        let v4308=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{(((v1500*((v508*v2263)+(v499*(self.scalar_static_f64[84]*v2254))))+(v509*common.v4244))+((v1502*((v510*v2279)+(v506*(self.scalar_static_f64[85]*v2270))))+(v511*v4272)))}else{common.v1})});
        let v4309=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*common.v4245)+(v511*v4273))}else{common.v1})});
        let v4310=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*common.v4246)+(v511*v4274))}else{common.v1})});
        let v4311=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*common.v4247)+(v511*v4275))}else{common.v1})});
        let v4312=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*common.v4248)+(v511*v4276))}else{common.v1})});
        let v4313=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{((v509*common.v4249)+(v511*v4277))}else{common.v1})});
        let v4314=(if self.scalar_static_bool[51]{common.v1}else{(if self.scalar_static_bool[50]{(v509*common.v4250)}else{common.v1})});
        let v4387=(v747*(if v685{((-(if self.scalar_static_bool[13]{(self.scalar_static_f64[98]*v2119)}else{(if (self.scalar_static_f64[97]!=0.0){(self.scalar_static_f64[98]*(common.v2104*(self.scalar_static_f64[99]*f64::powf(common.v391,self.scalar_static_f64[218]))))}else{common.v1})}))/(v406*v406))}else{common.v1}));
        let v4388=(-v688);
        let v4392=(v1535*v1535);
        let v4447=((v1541*v2530)+(v691*((v1539*common.v2103)+(common.v390*((common.v4367-common.v4382)-((((v1535*common.v4367)-(v1534*common.v4382))/v4392)/v1536))))));
        let v4448=(v691*(common.v36+(common.v390*((-common.v4383)-(((-(v1534*common.v4383))/v4392)/v1536)))));
        let v4449=(v691*(common.v2571+(common.v390*((common.v4368-common.v4384)-((((v1535*common.v4368)-(v1534*common.v4384))/v4392)/v1536)))));
        let v4450=(v691*(common.v390*(common.v4369-((common.v4369/v1535)/v1536))));
        let v4451=(v691*(common.v390*((common.v4370-common.v4385)-((((v1535*common.v4370)-(v1534*common.v4385))/v4392)/v1536))));
        let v4452=(v691*(common.v390*((common.v4371-common.v4386)-((((v1535*common.v4371)-(v1534*common.v4386))/v4392)/v1536))));
        let v4453=(v691*(common.v390*(common.v4372-((common.v4372/v1535)/v1536))));
        let v4454=(v691*(common.v390*(common.v4373-((common.v4373/v1535)/v1536))));
        let v4467=(common.v36*v749);
        let v4469=(v749*common.v2571);
        let v4471=(common.v552*v1548);
        let v4485=(v1551*v1551);
        let v4500=(v1552*(((v1551*((v1542*v2570)+(v721*v4447)))-(v1543*((v1550*v2530)+(v691*(v1548*(v54*(common.v134*v2570)))))))/v4485));
        let v4502=(v1552*(((v1551*(v721*v4448))-(v1543*(v691*(v1545*((v4467+v4467)/v4471)))))/v4485));
        let v4504=(v1552*(((v1551*(v721*v4449))-(v1543*(v691*(v1545*((v4469+v4469)/v4471)))))/v4485));
        let v4506=(v1552*((v721*v4450)/v1551));
        let v4508=(v1552*((v721*v4451)/v1551));
        let v4510=(v1552*((v721*v4452)/v1551));
        let v4512=(v1552*((v721*v4453)/v1551));
        let v4514=(v1552*((v721*v4454)/v1551));
        let v4516=(common.v552*v1555);
        let v4528=(v1555*v1555);
        let v4529=(((v1555*v4447)-(v1542*((v4500+v4500)/v4516)))/v4528);
        let v4533=(((v1555*v4448)-(v1542*((v4502+v4502)/v4516)))/v4528);
        let v4537=(((v1555*v4449)-(v1542*((v4504+v4504)/v4516)))/v4528);
        let v4541=(((v1555*v4450)-(v1542*((v4506+v4506)/v4516)))/v4528);
        let v4545=(((v1555*v4451)-(v1542*((v4508+v4508)/v4516)))/v4528);
        let v4549=(((v1555*v4452)-(v1542*((v4510+v4510)/v4516)))/v4528);
        let v4553=(((v1555*v4453)-(v1542*((v4512+v4512)/v4516)))/v4528);
        let v4557=(((v1555*v4454)-(v1542*((v4514+v4514)/v4516)))/v4528);
        let v4558=(v750*(if v692{((-(if self.scalar_static_bool[15]{(self.scalar_static_f64[105]*v2139)}else{(if (self.scalar_static_f64[104]!=0.0){(self.scalar_static_f64[105]*(common.v2104*(self.scalar_static_f64[106]*f64::powf(common.v391,self.scalar_static_f64[221]))))}else{common.v1})}))/(v426*v426))}else{common.v1}));
        let v4559=(-v694);
        let v4568=((v1558*(if v695{((-(if self.scalar_static_bool[16]{(self.scalar_static_f64[109]*v2139)}else{(if (self.scalar_static_f64[108]!=0.0){(self.scalar_static_f64[109]*(common.v2104*(self.scalar_static_f64[110]*f64::powf(common.v391,self.scalar_static_f64[223]))))}else{common.v1})}))/(v435*v435))}else{common.v1}))+(v697*(v751*common.v3323)));
        let v4569=(v697*(v751*common.v3324));
        let v4570=(v697*common.v1108);
        let v4571=(v697*((-common.v1108)+(v751*common.v3325)));
        let v4572=(v697*(v751*common.v3326));
        let v4573=(v752*(if v698{((-(self.scalar_static_f64[111]*(common.v2104*(self.scalar_static_f64[112]*f64::powf(common.v391,self.scalar_static_f64[224])))))/(v439*v439))}else{common.v1}));
        let v4574=(-v700);
        let v4585=((v1561*(if v701{((-(if self.scalar_static_bool[17]{(self.scalar_static_f64[116]*v2119)}else{(if (self.scalar_static_f64[115]!=0.0){(self.scalar_static_f64[116]*(common.v2104*(self.scalar_static_f64[117]*f64::powf(common.v391,self.scalar_static_f64[226]))))}else{common.v1})}))/(v452*v452))}else{common.v1}))+(v703*(v753*(if self.scalar_static_bool[31]{common.v1}else{v3471}))));
        let v4586=(v703*(-v1183));
        let v4587=(v703*(v753*(if self.scalar_static_bool[31]{common.v1}else{v3472})));
        let v4588=(v703*(v753*(if self.scalar_static_bool[31]{common.v1}else{v3473})));
        let v4589=(v703*(v753*(if self.scalar_static_bool[31]{common.v1}else{v3474})));
        let v4590=(v703*(v753*(if self.scalar_static_bool[31]{common.v1}else{v3475})));
        let v4591=(v703*(v1183+(v753*(if self.scalar_static_bool[31]{common.v1}else{v3476}))));
        let v4592=(v760*(if v704{((-(self.scalar_static_f64[113]*(common.v2104*(self.scalar_static_f64[114]*f64::powf(common.v391,self.scalar_static_f64[225])))))/(v443*v443))}else{common.v1}));
        let v4593=(-v706);
        let v4599=(if self.scalar_static_bool[52]{((v1566*v2327)*(self.scalar_static_f64[187]*f64::powf(v1568,self.scalar_static_f64[245])))}else{common.v1});
        let v4600=(common.v2425-v4599);
        let v4601=(v1575*v4600);
        let v4603=(common.v36*v1575);
        let v4605=(v1575*common.v2571);
        let v4607=(common.v552*v1578);
        let v4618=(if self.scalar_static_bool[52]{(v4599+(common.v134*(v4600+((v4601+v4601)/v4607))))}else{common.v1});
        let v4619=(if self.scalar_static_bool[52]{(common.v134*(common.v36+((v4603+v4603)/v4607)))}else{common.v1});
        let v4620=(if self.scalar_static_bool[52]{(common.v134*(common.v2571+((v4605+v4605)/v4607)))}else{common.v1});
        let v4624=(self.scalar_static_f64[188]*f64::powf(v1582,self.scalar_static_f64[246]));
        let v4633=(if self.scalar_static_bool[52]{((v1585*(-v2327))+(v1583*(v4618*v4624)))}else{common.v1});
        let v4634=(if self.scalar_static_bool[52]{(v1583*(v4619*v4624))}else{common.v1});
        let v4635=(if self.scalar_static_bool[52]{(v1583*(v4620*v4624))}else{common.v1});
        let v4660=(if self.scalar_static_bool[52]{((v1599*(if v1593{(v1594*v4633)}else{(if v1589{(v1590*v4633)}else{common.v1})}))+(v1598*(self.scalar_static_f64[185]*v4618)))}else{common.v1});
        let v4661=(if self.scalar_static_bool[52]{((v1599*(if v1593{(v1594*v4634)}else{(if v1589{(v1590*v4634)}else{common.v1})}))+(v1598*(self.scalar_static_f64[185]*v4619)))}else{common.v1});
        let v4662=(if self.scalar_static_bool[52]{((v1599*(if v1593{(v1594*v4635)}else{(if v1589{(v1590*v4635)}else{common.v1})}))+(v1598*(self.scalar_static_f64[185]*v4620)))}else{common.v1});
        let v4663=(-v3331);
        let v4664=(-v3335);
        let v4665=(-v3339);
        let v4666=(-v3343);
        let v4708=(if self.scalar_static_bool[54]{((v1566*v2329)*(self.scalar_static_f64[192]*f64::powf(v1611,self.scalar_static_f64[247])))}else{common.v1});
        let v4709=(-v4708);
        let v4710=(v1618*v4709);
        let v4712=(common.v36*v1618);
        let v4714=(v1618*common.v2571);
        let v4716=(common.v552*v1621);
        let v4727=(if self.scalar_static_bool[54]{(v4708+(common.v134*(v4709+((v4710+v4710)/v4716))))}else{common.v1});
        let v4728=(if self.scalar_static_bool[54]{(common.v134*(common.v36+((v4712+v4712)/v4716)))}else{common.v1});
        let v4729=(if self.scalar_static_bool[54]{(common.v134*(common.v2571+((v4714+v4714)/v4716)))}else{common.v1});
        let v4733=(self.scalar_static_f64[193]*f64::powf(v1625,self.scalar_static_f64[248]));
        let v4742=(if self.scalar_static_bool[54]{((v1628*(-v2329))+(v1626*(v4727*v4733)))}else{common.v1});
        let v4743=(if self.scalar_static_bool[54]{(v1626*(v4728*v4733))}else{common.v1});
        let v4744=(if self.scalar_static_bool[54]{(v1626*(v4729*v4733))}else{common.v1});
        let v4801=(if self.scalar_static_bool[60]{(-(common.v2571/self.scalar_static_f64[196]))}else{common.v1});
        let v4802=(if self.scalar_static_bool[60]{(-(common.v36/self.scalar_static_f64[196]))}else{common.v1});
        let v4803=(v1662*v4801);
        let v4805=(v1662*v4802);
        let v4807=(common.v552*v1665);
        let v4826=(v1674*v1674);
        let v4835=(self.scalar_static_f64[197]*f64::powf(v1676,self.scalar_static_f64[249]));
        let v4854=(v4219-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{(v1601*(-v4219))}else{common.v1})}));
        let v4857=(v4208-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{(v1601*(-v4208))}else{common.v1})}));
        let v4858=(v4209-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{(v1601*(-v4209))}else{common.v1})}));
        let v4859=(-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{v1601}else{common.v1})}));
        let v4860=((v4217-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{((v1603*v4660)+(v1601*(v4663-v4217)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[194]*((v3347/v1674)*v4835))}else{common.v1})}));
        let v4861=((v4218-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{((v1603*v4661)+(v1601*(v4664-v4218)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[194]*((((v1674*v3350)-(v1110*(if self.scalar_static_bool[62]{common.v1}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[195]*(if self.scalar_static_bool[60]{(common.v134*(v4801+((v4803+v4803)/v4807)))}else{v4801}))}else{common.v1})})))/v4826)*v4835))}else{common.v1})}));
        let v4862=((v4220-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{((v1603*v4662)+(v1601*(v4665-v4220)))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[194]*((((v1674*v3354)-(v1110*(if self.scalar_static_bool[62]{common.v1}else{(if self.scalar_static_bool[60]{(self.scalar_static_f64[195]*(if self.scalar_static_bool[60]{(common.v134*(v4802+((v4805+v4805)/v4807)))}else{v4802}))}else{common.v1})})))/v4826)*v4835))}else{common.v1})}));
        let v4863=((v4221-(if self.scalar_static_bool[53]{common.v1}else{(if self.scalar_static_bool[52]{(v1601*(v4666-v4221))}else{common.v1})}))-(if self.scalar_static_bool[63]{common.v1}else{(if self.scalar_static_bool[58]{(self.scalar_static_f64[194]*((v3358/v1674)*v4835))}else{common.v1})}));
        let v4897=(if self.scalar_static_bool[66]{((-v2313)/v2317)}else{common.v4866});
        let v4899=(v1705*common.v2571);
        let v4900=(common.v36*v1705);
        let v4958=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{(((v1720*((v517*(self.scalar_static_f64[86]*(common.v2104*(self.scalar_static_f64[88]*f64::powf(common.v391,self.scalar_static_f64[234])))))+(v513*(v517*(((common.v515*(self.scalar_static_f64[90]*common.v2175))-(v514*common.v2297))/common.v2301)))))+(v518*common.v4888))+((v1722*((v524*(self.scalar_static_f64[91]*(common.v2104*(self.scalar_static_f64[93]*f64::powf(common.v391,self.scalar_static_f64[235])))))+(v520*(v524*(((v522*(self.scalar_static_f64[95]*common.v2175))-(v521*v2313))/v2317)))))+(v525*(if v1712{((v1717*(v1714*(v370*v4897)))+(v1714*(v1715*v4897)))}else{(if v1707{(v1709*(common.v756*v4897))}else{v4272})}))))}else{common.v1})});
        let v4959=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4889)+(v525*(if v1712{common.v1}else{(if v1707{common.v1}else{v4273})})))}else{common.v1})});
        let v4960=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4890)+(v525*(if v1712{common.v1}else{(if v1707{common.v1}else{v4274})})))}else{common.v1})});
        let v4961=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4891)+(v525*(if v1712{common.v1}else{(if v1707{common.v1}else{v4275})})))}else{common.v1})});
        let v4962=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4892)+(v525*(if v1712{common.v1}else{(if v1707{common.v1}else{v4276})})))}else{common.v1})});
        let v4963=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4893)+(v525*(if v1712{(v1714*v4899)}else{(if v1707{(v1709*v4899)}else{v4277})})))}else{common.v1})});
        let v4964=(if self.scalar_static_bool[67]{common.v1}else{(if self.scalar_static_bool[66]{((v518*common.v4894)+(v525*(if v1712{(v1714*v4900)}else{(if v1707{(v1709*v4900)}else{common.v1})})))}else{common.v1})});
        let v5173=(v11*common.v36);
        let v5174=(v11*common.v2571);

        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * ((common.v36*(v1382+(v11*common.v725)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4014), (common.v36*v4015), (common.v36*v3971), (common.v36*(v4016+v5173)), (common.v36*(v4017+v5174)), (common.v36*v3974), (common.v36*v3975)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(9),
            multiplicity * ((common.v36*(v1435+(v11*common.v728)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4141), (common.v36*v4142), (common.v36*(v4098+v5173)), (common.v36*v4143), (common.v36*(v4144+v5174)), (common.v36*v4101), (common.v36*v4102)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node1_local(
            Some(6),
            Some(9),
            multiplicity * ((common.v36*common.v762)),
            13,
            multiplicity * (common.v36),
        );
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(9),
            Some(6),
            multiplicity * ((common.v36*v1109)),
            [4, 6, 8, 9],
            [(common.v36*v3331), (common.v36*v3335), (common.v36*v3339), (common.v36*v3343)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * ((common.v36*(v1684+(v11*common.v731)))),
            [4, 6, 7, 8, 9, 10, 11, 13],
            [(common.v36*v4860), (common.v36*(v4861+v5174)), (common.v36*v4854), (common.v36*(v4862+v5173)), (common.v36*v4863), (common.v36*v4857), (common.v36*v4858), (common.v36*v4859)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(5),
            multiplicity * ((common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{(v1644*v1645)}else{common.v1})})+(v11*v736)))),
            [0, 4, 5, 6, 7, 8],
            [(common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{(v1644*v4388)}else{common.v1})})), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{((v1645*(if self.scalar_static_bool[54]{((v1642*(if v1636{(v1637*v4742)}else{(if v1632{(v1633*v4742)}else{common.v1})}))+(v1641*(self.scalar_static_f64[189]*v4727)))}else{v4660}))+(v1644*(-v4387)))}else{common.v1})})), (common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{((v1645*(if self.scalar_static_bool[54]{((v1642*(if v1636{(v1637*v4743)}else{(if v1632{(v1633*v4743)}else{common.v1})}))+(v1641*(self.scalar_static_f64[189]*v4728)))}else{common.v1}))+(v688*v1644))}else{common.v1})})+v5174)), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{(v1645*(if self.scalar_static_bool[54]{common.v1}else{v4661}))}else{common.v1})})), (common.v36*((if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{(v1645*(if self.scalar_static_bool[54]{((v1642*(if v1636{(v1637*v4744)}else{(if v1632{(v1633*v4744)}else{common.v1})}))+(v1641*(self.scalar_static_f64[189]*v4729)))}else{common.v1}))}else{common.v1})})+v5173)), (common.v36*(if self.scalar_static_bool[55]{common.v1}else{(if self.scalar_static_bool[54]{(v1645*(if self.scalar_static_bool[54]{common.v1}else{v4662}))}else{common.v1})}))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(10),
            multiplicity * ((common.v36*(v1507+(v11*common.v739)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4308), (common.v36*v4309), (common.v36*(v4310+v5173)), (common.v36*v4311), (common.v36*v4312), (common.v36*(v4313+v5174)), (common.v36*v4314)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(0),
            Some(5),
            multiplicity * (v1533),
            0,
            multiplicity * (v688),
            4,
            multiplicity * (v4387),
            5,
            multiplicity * (v4388),
        );
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(5),
            Some(6),
            multiplicity * (v1780),
            [4, 5, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4529), (common.v36*v4533), (common.v36*v4537), (common.v36*v4541), (common.v36*v4545), (common.v36*v4549), (common.v36*v4553), (common.v36*v4557)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(1),
            Some(7),
            multiplicity * (v1557),
            1,
            multiplicity * (v694),
            4,
            multiplicity * (v4558),
            7,
            multiplicity * (v4559),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (v1559),
            [4, 6, 7, 8, 9],
            [v4568, v4569, v4570, v4571, v4572],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(2),
            Some(9),
            multiplicity * (v1560),
            2,
            multiplicity * (v700),
            4,
            multiplicity * (v4573),
            9,
            multiplicity * (v4574),
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(10),
            Some(5),
            multiplicity * (v1562),
            [4, 5, 6, 7, 8, 9, 10],
            [v4585, v4586, v4587, v4588, v4589, v4590, v4591],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(11),
            Some(10),
            multiplicity * ((common.v36*(v1727+(v11*common.v756)))),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v4958), (common.v36*v4959), (common.v36*v4960), (common.v36*v4961), (common.v36*v4962), (common.v36*(v4963+v5174)), (common.v36*(v4964+v5173))],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(7),
            Some(11),
            multiplicity * (v1782),
            [4, 6, 7, 8, 9, 10, 11],
            [(common.v36*v3567), (common.v36*v3568), (common.v36*v3569), (common.v36*v3570), (common.v36*v3571), (common.v36*v3572), (common.v36*v3573)],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node3_local(
            Some(3),
            Some(11),
            multiplicity * (v1563),
            3,
            multiplicity * (v706),
            4,
            multiplicity * (v4592),
            11,
            multiplicity * (v4593),
        );
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(12),
            None,
            multiplicity * ((common.v762-v1110)),
            [4, 6, 8, 9, 13],
            [(-v3347), (-v3350), (-v3354), (-v3358), common.v2],
            [],
            [],
            multiplicity,
        );
        stamper.stamp_current_node2_local(
            Some(13),
            None,
            multiplicity * ((common.v762-common.v761)),
            12,
            multiplicity * (common.v26),
            13,
            multiplicity * (common.v2),
        );
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * ((common.v371*v709)),
            4,
            multiplicity * ((v709+(common.v371*(if v707{((-(self.scalar_static_f64[118]*(self.scalar_static_f64[119]*common.v2101)))/(v457*v457))}else{common.v1})))),
        );
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * ((((((((((((((((common.v725*v1382)+(common.v731*v1684))+(v744*v1602))+(common.v728*v1435))+(common.v739*v1507))+(v760*v1563))+(common.v756*v1727))+(v758*v1184))+(v747*v1533))+(v749*v1556))+(v750*v1557))+(v751*v1559))+(v752*v1560))+(v753*v1562))*self.scalar_static_f64[199])),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13],
            &[(self.scalar_static_f64[199]*(v1533+v1533)), (self.scalar_static_f64[199]*(v1557+v1557)), (self.scalar_static_f64[199]*(v1560+v1560)), (self.scalar_static_f64[199]*(v1563+v1563)), (self.scalar_static_f64[199]*((((((((((((((common.v725*v4014)+(common.v731*v4860))+(v744*v4663))+(common.v728*v4141))+(common.v739*v4308))+(v760*v4592))+(common.v756*v4958))+(v758*v3567))+(v747*v4387))+(v749*v4529))+(v750*v4558))+(v751*v4568))+(v752*v4573))+(v753*v4585))), (self.scalar_static_f64[199]*(((v1645+(v747*v4388))+(v1780+(v749*v4533)))+((-v1562)+(v753*v4586)))), (self.scalar_static_f64[199]*((((((((((common.v725*v4015)+((v1684*common.v2571)+(common.v731*v4861)))+((common.v36*v1602)+(v744*v4664)))+(common.v728*v4142))+(common.v739*v4309))+(common.v756*v4959))+(v758*v3568))+((v1556*common.v2571)+(v749*v4537)))+(v751*v4569))+(v753*v4587))), (self.scalar_static_f64[199]*((((((((((common.v725*v3971)+(common.v731*v4854))+((common.v36*v1435)+(common.v728*v4098)))+((common.v36*v1507)+(common.v739*v4310)))+(common.v756*v4960))+(v1782+(v758*v3569)))+(v749*v4541))+((-v1557)+(v750*v4559)))+(v1559+(v751*v4570)))+(v753*v4588))), (self.scalar_static_f64[199]*(((((((((((common.v36*v1382)+(common.v725*v4016))+((common.v36*v1684)+(common.v731*v4862)))+(v744*v4665))+(common.v728*v4143))+(common.v739*v4311))+(common.v756*v4961))+(v758*v3570))+(v749*v4545))+((-v1559)+(v751*v4571)))+(v753*v4589))), (self.scalar_static_f64[199]*((((((((((((v1382*common.v2571)+(common.v725*v4017))+(common.v731*v4863))+((v1602*common.v2571)+(v744*v4666)))+((v1435*common.v2571)+(common.v728*v4144)))+(common.v739*v4312))+(common.v756*v4962))+(v758*v3571))+(v749*v4549))+(v751*v4572))+((-v1560)+(v752*v4574)))+(v753*v4590))), (self.scalar_static_f64[199]*((((((((common.v725*v3974)+(common.v731*v4857))+(common.v728*v4101))+((v1507*common.v2571)+(common.v739*v4313)))+((v1727*common.v2571)+(common.v756*v4963)))+(v758*v3572))+(v749*v4553))+(v1562+(v753*v4591)))), (self.scalar_static_f64[199]*((((((((common.v725*v3975)+(common.v731*v4858))+(common.v728*v4102))+(common.v739*v4314))+((-v1563)+(v760*v4593)))+((common.v36*v1727)+(common.v756*v4964)))+((v1184*common.v2571)+(v758*v3573)))+(v749*v4557))), (self.scalar_static_f64[199]*(v744+(common.v731*v4859)))],
            &[],
            &[],
            multiplicity,
        );
        let v2091_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, common.v2091);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(9),
            multiplicity * (v2091_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v6024) * ddt_scale), ((common.v6025) * ddt_scale), ((common.v6026) * ddt_scale), ((common.v6027) * ddt_scale), ((common.v6028) * ddt_scale), ((common.v6029) * ddt_scale), ((common.v6030) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2092_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, common.v2092);
        stamper.stamp_current_node3_local(
            Some(7),
            Some(9),
            multiplicity * (v2092_ddt),
            4,
            multiplicity * (((common.v6031) * ddt_scale)),
            7,
            multiplicity * (((common.v6032) * ddt_scale)),
            9,
            multiplicity * (((common.v6033) * ddt_scale)),
        );
        let v2093_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, common.v2093);
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(8),
            Some(6),
            multiplicity * (v2093_ddt),
            [4, 6, 7, 8, 9, 10, 11],
            [((common.v6034) * ddt_scale), ((common.v6035) * ddt_scale), ((common.v6036) * ddt_scale), ((common.v6037) * ddt_scale), ((common.v6038) * ddt_scale), ((common.v6039) * ddt_scale), ((common.v6040) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2094_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, common.v2094);
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(5),
            multiplicity * (v2094_ddt),
            [4, 5, 6, 8, 9],
            [((common.v6041) * ddt_scale), ((common.v6042) * ddt_scale), ((common.v6043) * ddt_scale), ((common.v6044) * ddt_scale), ((common.v6045) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2095_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, common.v2095);
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(7),
            Some(10),
            multiplicity * (v2095_ddt),
            [4, 6, 7, 8, 9, 10],
            [((common.v6046) * ddt_scale), ((common.v6047) * ddt_scale), ((common.v6048) * ddt_scale), ((common.v6049) * ddt_scale), ((common.v6050) * ddt_scale), ((common.v6051) * ddt_scale)],
            [],
            [],
            multiplicity,
        );
        let v2081_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, common.v2081);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(2),
            multiplicity * (v2081_ddt),
            1,
            multiplicity * (((self.scalar_static_f64[213]) * ddt_scale)),
            2,
            multiplicity * (((self.scalar_static_f64[251]) * ddt_scale)),
        );
        let v2083_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, common.v2083);
        stamper.stamp_current_node2_local(
            Some(1),
            Some(0),
            multiplicity * (v2083_ddt),
            0,
            multiplicity * (((self.scalar_static_f64[252]) * ddt_scale)),
            1,
            multiplicity * (((self.scalar_static_f64[214]) * ddt_scale)),
        );
        let v2096_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, common.v2096);
        stamper.stamp_current_node3_local(
            Some(11),
            Some(10),
            multiplicity * (v2096_ddt),
            4,
            multiplicity * (((common.v6052) * ddt_scale)),
            10,
            multiplicity * (((common.v6053) * ddt_scale)),
            11,
            multiplicity * (((common.v6054) * ddt_scale)),
        );
        let v2087_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, common.v2087);
        stamper.stamp_current_node1_local(
            Some(12),
            None,
            multiplicity * (v2087_ddt),
            12,
            multiplicity * (((self.scalar_static_f64[216]) * ddt_scale)),
        );
        let v2090_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, common.v2090);
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (v2090_ddt),
            13,
            multiplicity * (((self.scalar_static_f64[253]) * ddt_scale)),
        );
        let v2085_ddt=eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, common.v2085);
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (v2085_ddt),
            4,
            multiplicity * (((self.scalar_static_f64[215]) * ddt_scale)),
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
            &[common.v6024, common.v6025, common.v6026, common.v6027, common.v6028, common.v6029, common.v6030],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[7]),
            Some(nodes[9]),
            nodes[4],
            multiplicity * (common.v6031),
            nodes[7],
            multiplicity * (common.v6032),
            nodes[9],
            multiplicity * (common.v6033),
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10], nodes[11]],
            &[common.v6034, common.v6035, common.v6036, common.v6037, common.v6038, common.v6039, common.v6040],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[5]),
            &[nodes[4], nodes[5], nodes[6], nodes[8], nodes[9]],
            &[common.v6041, common.v6042, common.v6043, common.v6044, common.v6045],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[10]),
            &[nodes[4], nodes[6], nodes[7], nodes[8], nodes[9], nodes[10]],
            &[common.v6046, common.v6047, common.v6048, common.v6049, common.v6050, common.v6051],
            &[],
            &[],
            multiplicity,
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[2]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[213]),
            nodes[2],
            multiplicity * (self.scalar_static_f64[251]),
        );
        stamper.stamp_current_reactive_node2(
            Some(nodes[1]),
            Some(nodes[0]),
            nodes[0],
            multiplicity * (self.scalar_static_f64[252]),
            nodes[1],
            multiplicity * (self.scalar_static_f64[214]),
        );
        stamper.stamp_current_reactive_node3(
            Some(nodes[11]),
            Some(nodes[10]),
            nodes[4],
            multiplicity * (common.v6052),
            nodes[10],
            multiplicity * (common.v6053),
            nodes[11],
            multiplicity * (common.v6054),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[12]),
            None,
            nodes[12],
            multiplicity * (self.scalar_static_f64[216]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (self.scalar_static_f64[253]),
        );
        stamper.stamp_current_reactive_node1(
            Some(nodes[4]),
            None,
            nodes[4],
            multiplicity * (self.scalar_static_f64[215]),
        );
    }
}
